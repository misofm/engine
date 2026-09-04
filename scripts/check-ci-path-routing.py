#!/usr/bin/env python3
"""Static contract checks for issue #328's path-aware Actions workflows.

The repository intentionally does not install PyYAML for policy checks.  These checks therefore
inspect the small, fixed workflow contract directly and leave YAML syntax validation to `yq` (or
the GitHub parser) in the caller.  They are deliberately exact about required contexts and their
dependencies, because a workflow that merely looks similar can leave a pull request pending or
let skipped heavy work pass as selected work.
"""
from __future__ import annotations

import argparse
import ast
import pathlib
import re
import sys

EVIDENCE = [".github/ISSUE_SPECS/**", "docs/**", "README.md", "dsp-research/**/*.md"]
SDK_FILES = [
    "scripts/check-sdk-deletions.py",
    "scripts/check-sdk-generated.sh",
    "scripts/check-sdk-headless.sh",
    "scripts/check-sdk-types.sh",
    "scripts/sdk-package.sh",
    "scripts/test-sdk-artifact-builder-output-contract.sh",
]
SDK = ["sdk/**", *SDK_FILES]
GIT_DIFF_OPTIONS = ("--name-status", "-z", "--find-renames", "--find-copies-harder")
RELEASE_PR_INPUTS = [
    "**/Cargo.toml",
    "Cargo.lock",
    "rust-toolchain.toml",
    "scripts/run-release-workspace-tests.sh",
    ".github/workflows/release-build.yml",
    ".cargo/config.toml",
    "scripts/check-release-shape.py",
]

CANONICAL_ROUTE_JOB = """    name: classify qualification paths
    runs-on: ubuntu-24.04
    outputs:
      route: ${{ steps.classify.outputs.route }}
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - name: Validate path-routing policy and mutations
        run: |
          python3 -B scripts/check-ci-path-routing.py
          python3 -B scripts/test-ci-path-routing.py
      - id: classify
        name: Select fail-safe qualification route
        run: |
          route=$(python3 -B scripts/ci-path-router.py \\
            --event "${{ github.event_name }}" \\
            --base "${{ github.event.pull_request.base.sha || github.event.before }}" \\
            --head "${{ github.event.pull_request.head.sha || github.sha }}")
          printf 'route=%s\\n' "$route" >> "$GITHUB_OUTPUT"

"""


class Invalid(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise Invalid(message)


def section(text: str, header: str, next_headers: tuple[str, ...]) -> str:
    match = re.search(rf"^{re.escape(header)}\n", text, re.MULTILINE)
    require(match is not None, f"missing {header.strip()}")
    tail = text[match.end():]
    end = len(tail)
    for next_header in next_headers:
        found = re.search(rf"^{re.escape(next_header)}\n", tail, re.MULTILINE)
        if found is not None:
            end = min(end, found.start())
    return tail[:end]


def job(text: str, name: str) -> str:
    jobs = section(text, "jobs:", ())
    match = re.search(rf"^  {re.escape(name)}:\n", jobs, re.MULTILINE)
    require(match is not None, f"missing job {name}")
    tail = jobs[match.end():]
    next_job = re.search(r"^  [A-Za-z0-9_-]+:\n", tail, re.MULTILINE)
    return tail[:next_job.start() if next_job else len(tail)]


def named_step(job_block: str, name: str) -> str:
    marker = f"      - name: {name}\n"
    require(job_block.count(marker) == 1, f"step {name!r} must exist exactly once")
    start = job_block.index(marker)
    tail = job_block[start + len(marker):]
    next_step = re.search(r"^      - ", tail, re.MULTILINE)
    return marker + tail[:next_step.start() if next_step else len(tail)]


def require_job_header(job_block: str, expected: list[str], message: str) -> None:
    header = [
        line for line in job_block.splitlines()
        if line.startswith("    ") and not line.startswith("      ") and line.strip()
    ]
    require(header == expected, message)


def canonical_trigger(push_ignores: list[str], pull_paths: list[str] | None) -> str:
    """Render the only accepted spelling of a security-critical Actions trigger."""
    ignored = "".join(f"      - '{path}'\n" for path in push_ignores)
    pull = "  pull_request:\n    branches:\n      - main\n"
    if pull_paths is not None:
        pull += "    paths:\n" + "".join(f"      - '{path}'\n" for path in pull_paths)
    return (
        "  push:\n"
        "    branches:\n"
        "      - main\n"
        "    paths-ignore:\n"
        f"{ignored}"
        f"{pull}"
        "  workflow_dispatch:\n\n"
    )


def check_trigger(text: str, workflow: str, push_ignores: list[str]) -> None:
    actual = section(text, "on:", ("concurrency:",))
    pull_paths = RELEASE_PR_INPUTS if workflow == "release-build.yml" else None
    require(actual == canonical_trigger(push_ignores, pull_paths),
            f"{workflow}: trigger block must match the exact canonical path contract")


def check_mapping_structure(text: str, workflow: str, title: str, jobs: list[str]) -> None:
    """Exclude duplicate or quoted override keys around the canonically pinned regions."""
    top_level = [
        line for line in text.splitlines()
        if line and not line[0].isspace() and not line.startswith("#")
    ]
    require(top_level == [f"name: {title}", "on:", "concurrency:", "env:", "jobs:"],
            f"{workflow}: top-level mapping structure must be exact and unique")
    jobs_block = section(text, "jobs:", ())
    job_headers = [
        line for line in jobs_block.splitlines()
        if line.startswith("  ") and not line.startswith("    ") and line.strip()
        and not line.lstrip().startswith("#")
    ]
    require(job_headers == [f"  {name}:" for name in jobs],
            f"{workflow}: job mapping structure must be exact and unique")


def check_common(text: str, workflow: str, domain: str) -> None:
    require(f"group: {domain}-${{{{ github.workflow }}}}-${{{{ github.ref }}}}" in text,
            f"{workflow}: concurrency must be ref-scoped in its own domain")
    require("cancel-in-progress: true" in text, f"{workflow}: missing cancellation")


def check_router(text: str, workflow: str) -> None:
    route = job(text, "route")
    require(route == CANONICAL_ROUTE_JOB,
            f"{workflow}: route job must match the exact unconditional, failure-propagating contract")
    require("route: ${{ steps.classify.outputs.route }}" in route,
            f"{workflow}: router must expose route output")
    require("fetch-depth: 0" in route, f"{workflow}: router needs complete history")
    policy = "python3 -B scripts/check-ci-path-routing.py\n          python3 -B scripts/test-ci-path-routing.py"
    require(policy in route,
            f"{workflow}: route must validate path policy and mutations before classification")
    require(route.index(policy) < route.index("scripts/ci-path-router.py"),
            f"{workflow}: route must validate checked-out workflow code before classification")
    require("scripts/ci-path-router.py" in route and "--event \"${{ github.event_name }}\"" in route,
            f"{workflow}: router must route dispatch through the fail-safe classifier")


def result_variable(job_name: str) -> str:
    return job_name.replace("-", "_").upper() + "_RESULT"


def canonical_aggregate(title: str, heavy: list[str], evaluator: tuple[str, ...]) -> str:
    """Render the only accepted aggregate job, including its failure semantics."""
    results = "".join(
        f"      {result_variable(name)}: ${{{{ needs.{name}.result }}}}\n" for name in heavy
    )
    script = "".join(f"          {line}\n" for line in evaluator)
    return (
        f"    name: {title}\n"
        "    if: always()\n"
        f"    needs: [route, {', '.join(heavy)}]\n"
        "    runs-on: ubuntu-24.04\n"
        "    env:\n"
        "      ROUTE_RESULT: ${{ needs.route.result }}\n"
        "      ROUTE: ${{ needs.route.outputs.route }}\n"
        f"{results}"
        "    steps:\n"
        f"      - name: Enforce {title} route\n"
        "        run: |\n"
        f"{script}"
    )


def check_aggregate(
    text: str,
    workflow: str,
    title: str,
    heavy: list[str],
    selected: str,
    selected_routes: tuple[str, ...],
    skipped_routes: tuple[str, ...],
    evaluator: tuple[str, ...],
) -> None:
    aggregate = job(text, "qualification")
    require(aggregate == canonical_aggregate(title, heavy, evaluator),
            f"{workflow}: aggregate must match the exact always-failing-on-error contract")
    job_name_count = len(re.findall(rf"^    name: {re.escape(title)}$", text, re.MULTILINE))
    require(job_name_count == 1 and f"name: {title}" in aggregate,
            f"{workflow}: aggregate name must be exactly once as {title!r}")
    require(re.search(r"^    if: always\(\)$", aggregate, re.MULTILINE) is not None,
            f"{workflow}: aggregate job itself must always run")
    require("continue-on-error:" not in aggregate,
            f"{workflow}: aggregate job and enforcement step must not suppress failures")
    require(f"needs: [route, {', '.join(heavy)}]" in aggregate,
            f"{workflow}: aggregate dependencies must include every heavy job exactly once")
    require("ROUTE_RESULT: ${{ needs.route.result }}" in aggregate
            and "ROUTE: ${{ needs.route.outputs.route }}" in aggregate,
            f"{workflow}: aggregate must pass bounded router values through env")
    require('[[ "$ROUTE_RESULT" == success ]]' in aggregate,
            f"{workflow}: aggregate evaluator must fail a failed router")
    for route in selected_routes:
        require(f"{route}) expected=success" in aggregate,
                f"{workflow}: selected {route} route must require heavy success")
    for route in skipped_routes:
        require(f"{route}) expected=skipped" in aggregate,
                f"{workflow}: unselected {route} route must require skipped heavy jobs")
    require('*) echo "unknown route: $ROUTE" >&2; exit 1 ;;' in aggregate,
            f"{workflow}: unknown route must fail")
    variables: list[str] = []
    for name in heavy:
        heavy_job = job(text, name)
        require("needs: route" in heavy_job or "needs: [route," in heavy_job,
                f"{workflow}: {name} must depend on router")
        require(selected in heavy_job, f"{workflow}: {name} must be selected only by the route")
        variable = result_variable(name)
        variables.append(variable)
        require(f"{variable}: ${{{{ needs.{name}.result }}}}" in aggregate,
                f"{workflow}: aggregate does not pass {name} result to its evaluator")
    if len(variables) == 1:
        require(f'[[ "${variables[0]}" == "$expected" ]] ||' in aggregate,
                f"{workflow}: aggregate evaluator does not fail {heavy[0]} result mismatch")
    else:
        loop = re.search(
            r'for result in (?P<inputs>[^;]+); do\n\s+\[\[ "\$result" == "\$expected" \]\] \|\|',
            aggregate,
        )
        require(loop is not None, f"{workflow}: aggregate must fail every heavy-result mismatch")
        for variable in variables:
            require(f'"${variable}"' in loop.group("inputs"),
                    f"{workflow}: aggregate evaluator does not fail {variable} mismatch")


def check_sdk_closure(text: str) -> None:
    heavy = job(text, "sdk")
    for required in (
        "npm ci",
        "wasm32-unknown-unknown",
        "bash scripts/build-web-audioworklet.sh target/ci/sdk-artifacts",
        "bash scripts/check-sdk-generated.sh",
        "python3 -B scripts/check-sdk-deletions.py",
        "bash scripts/check-sdk-types.sh",
        "bash scripts/check-sdk-headless.sh target/ci/sdk-artifacts",
        "bash scripts/sdk-package.sh check target/ci/sdk-artifacts",
    ):
        require(required in heavy, f"sdk.yml: SDK closure is missing {required!r}")


def check_classifier_contract(root: pathlib.Path) -> None:
    """Make the unknown-path fail-safe a checked policy, not an implied convention."""
    source = (root / "scripts/ci-path-router.py").read_text(encoding="utf-8")
    try:
        tree = ast.parse(source)
    except SyntaxError as error:
        raise Invalid(f"ci-path-router.py is not valid Python: {error}") from error
    function = next((node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == "path_kind"), None)
    require(function is not None, "ci-path-router.py: missing path_kind classifier")
    require(function.body and isinstance(function.body[-1], ast.Return)
            and isinstance(function.body[-1].value, ast.Constant)
            and function.body[-1].value.value is None,
            "ci-path-router.py: unknown paths must fall through to full qualification")
    sdk_files = next((
        node.value for node in tree.body
        if isinstance(node, ast.Assign)
        and any(isinstance(target, ast.Name) and target.id == "SDK_FILES" for target in node.targets)
    ), None)
    require(isinstance(sdk_files, ast.Set), "ci-path-router.py: SDK_FILES must be a literal set")
    values = {
        element.value for element in sdk_files.elts
        if isinstance(element, ast.Constant) and isinstance(element.value, str)
    }
    expected = set(SDK_FILES)
    require(len(values) == len(sdk_files.elts) and values == expected,
            "ci-path-router.py: exact SDK file taxonomy drifted (LICENSE is full-route owned)")
    git_options = next((
        node.value for node in tree.body
        if isinstance(node, ast.Assign)
        and any(isinstance(target, ast.Name) and target.id == "GIT_DIFF_OPTIONS"
                for target in node.targets)
    ), None)
    require(isinstance(git_options, ast.Tuple),
            "ci-path-router.py: GIT_DIFF_OPTIONS must be a literal tuple")
    options = tuple(
        element.value for element in git_options.elts
        if isinstance(element, ast.Constant) and isinstance(element.value, str)
    )
    require(len(options) == len(git_options.elts) and options == GIT_DIFF_OPTIONS,
            "ci-path-router.py: Git diff must discover copies from unchanged full-route sources")
    diff_function = next((
        node for node in tree.body
        if isinstance(node, ast.FunctionDef) and node.name == "diff_paths"
    ), None)
    run_calls = [
        node for node in ast.walk(diff_function) if isinstance(node, ast.Call)
        and isinstance(node.func, ast.Attribute) and node.func.attr == "run"
        and isinstance(node.func.value, ast.Name) and node.func.value.id == "subprocess"
    ] if diff_function is not None else []
    command = run_calls[0].args[0] if len(run_calls) == 1 and run_calls[0].args else None
    require(isinstance(command, ast.List) and len(command.elts) == 4
            and all(isinstance(command.elts[index], ast.Constant)
                    and command.elts[index].value == value
                    for index, value in enumerate(("git", "diff")))
            and isinstance(command.elts[2], ast.Starred)
            and isinstance(command.elts[2].value, ast.Name)
            and command.elts[2].value.id == "GIT_DIFF_OPTIONS"
            and isinstance(command.elts[3], ast.Starred)
            and isinstance(command.elts[3].value, ast.Name)
            and command.elts[3].value.id == "revisions",
            "ci-path-router.py: production Git diff must consume the pinned option tuple exactly")


def check_shared_license_ownership(ci: str, sdk: str) -> None:
    host = job(ci, "host")
    require_job_header(host, [
        "    name: host quality and native shell",
        "    needs: route",
        "    if: needs.route.outputs.route == 'full'",
        "    runs-on: ubuntu-24.04",
        "    steps:",
    ], "ci.yml: full-route host job ownership or failure semantics drifted")
    require(named_step(host, "Workspace policy") == (
        "      - name: Workspace policy\n"
        "        run: bash scripts/check-workspace-policy.sh\n"
    ), "ci.yml: host must run the unsuppressed canonical workspace-policy step")

    sdk_job = job(sdk, "sdk")
    require_job_header(sdk_job, [
        "    name: SDK package, generated surface, and headless qualification",
        "    needs: route",
        "    if: needs.route.outputs.route == 'sdk' || needs.route.outputs.route == 'full'",
        "    runs-on: ubuntu-24.04",
        "    steps:",
    ], "sdk.yml: SDK/full package job ownership or failure semantics drifted")
    require(re.search(
        r"^    if: needs\.route\.outputs\.route == 'sdk' \|\| needs\.route\.outputs\.route == 'full'$",
        sdk_job, re.MULTILINE,
    ) is not None, "sdk.yml: package qualification must be selected on SDK and full routes")
    require(named_step(sdk_job, "Build one pinned AudioWorklet closure and qualify the SDK package") == (
        "      - name: Build one pinned AudioWorklet closure and qualify the SDK package\n"
        "        run: |\n"
        "          mkdir -p target/ci/sdk-artifacts\n"
        "          bash scripts/build-web-audioworklet.sh target/ci/sdk-artifacts\n"
        "          bash scripts/check-sdk-generated.sh\n"
        "          python3 -B scripts/check-sdk-deletions.py\n"
        "          bash scripts/check-sdk-types.sh\n"
        "          bash scripts/check-sdk-headless.sh target/ci/sdk-artifacts\n"
        "          bash scripts/sdk-package.sh check target/ci/sdk-artifacts\n"
        "\n"
    ), "sdk.yml: full-route SDK package qualification step drifted or was suppressed")


def check(root: pathlib.Path) -> None:
    workflows = root / ".github/workflows"
    ci = (workflows / "ci.yml").read_text(encoding="utf-8")
    browser = (workflows / "browser-qualification.yml").read_text(encoding="utf-8")
    sdk = (workflows / "sdk.yml").read_text(encoding="utf-8")
    release = (workflows / "release-build.yml").read_text(encoding="utf-8")

    check_mapping_structure(ci, "ci.yml", "ci",
                            ["route", "host", "x86-probes", "wasm", "wasm-gates", "qualification"])
    check_mapping_structure(browser, "browser-qualification.yml", "browser qualification",
                            ["route", "artifact", "browser", "qualification"])
    check_mapping_structure(sdk, "sdk.yml", "SDK qualification",
                            ["route", "sdk", "qualification"])
    check_mapping_structure(release, "release-build.yml", "release build", ["release-workspace"])

    for text, name, ignored in (
        (ci, "ci.yml", EVIDENCE + SDK),
        (browser, "browser-qualification.yml", EVIDENCE + SDK),
        (release, "release-build.yml", EVIDENCE + SDK),
        (sdk, "sdk.yml", EVIDENCE),
    ):
        check_trigger(text, name, ignored)

    check_common(ci, "ci.yml", "engine-qualification")
    check_common(browser, "browser-qualification.yml", "browser-qualification")
    check_common(sdk, "sdk.yml", "sdk-qualification")
    check_common(release, "release-build.yml", "release-build")
    check_router(ci, "ci.yml")
    check_router(browser, "browser-qualification.yml")
    check_router(sdk, "sdk.yml")
    check_aggregate(ci, "ci.yml", "engine qualification",
                    ["host", "x86-probes", "wasm", "wasm-gates"],
                    "needs.route.outputs.route == 'full'", ("full",), ("sdk|evidence",), (
                        "set -euo pipefail",
                        '[[ "$ROUTE_RESULT" == success ]] || { echo "router result: $ROUTE_RESULT" >&2; exit 1; }',
                        'case "$ROUTE" in',
                        "  full) expected=success ;;",
                        "  sdk|evidence) expected=skipped ;;",
                        '  *) echo "unknown route: $ROUTE" >&2; exit 1 ;;',
                        "esac",
                        'for result in "$HOST_RESULT" "$X86_PROBES_RESULT" "$WASM_RESULT" "$WASM_GATES_RESULT"; do',
                        '  [[ "$result" == "$expected" ]] || { echo "expected $expected heavy result, got $result" >&2; exit 1; }',
                        "done",
                    ))
    check_aggregate(browser, "browser-qualification.yml", "browser qualification",
                    ["artifact", "browser"], "needs.route.outputs.route == 'full'",
                    ("full",), ("sdk|evidence",), (
                        "set -euo pipefail",
                        '[[ "$ROUTE_RESULT" == success ]] || { echo "router result: $ROUTE_RESULT" >&2; exit 1; }',
                        'case "$ROUTE" in',
                        "  full) expected=success ;;",
                        "  sdk|evidence) expected=skipped ;;",
                        '  *) echo "unknown route: $ROUTE" >&2; exit 1 ;;',
                        "esac",
                        'for result in "$ARTIFACT_RESULT" "$BROWSER_RESULT"; do',
                        '  [[ "$result" == "$expected" ]] || { echo "expected $expected heavy result, got $result" >&2; exit 1; }',
                        "done",
                    ))
    check_aggregate(sdk, "sdk.yml", "SDK qualification", ["sdk"],
                    "needs.route.outputs.route == 'sdk' || needs.route.outputs.route == 'full'",
                    ("full|sdk",), ("evidence",), (
                        "set -euo pipefail",
                        '[[ "$ROUTE_RESULT" == success ]] || { echo "router result: $ROUTE_RESULT" >&2; exit 1; }',
                        'case "$ROUTE" in',
                        "  full|sdk) expected=success ;;",
                        "  evidence) expected=skipped ;;",
                        '  *) echo "unknown route: $ROUTE" >&2; exit 1 ;;',
                        "esac",
                        '[[ "$SDK_RESULT" == "$expected" ]] || { echo "expected $expected SDK result, got $SDK_RESULT" >&2; exit 1; }',
                    ))
    check_sdk_closure(sdk)
    check_classifier_contract(root)
    check_shared_license_ownership(ci, sdk)
    require("check-sdk-generated" not in job(ci, "host"),
            "ci.yml: generated SDK ownership must be in sdk.yml")
    require("check-sdk-headless" not in job(ci, "wasm") and "sdk-package.sh" not in job(ci, "wasm"),
            "ci.yml: SDK package ownership must be in sdk.yml")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=pathlib.Path, default=pathlib.Path(__file__).resolve().parent.parent)
    args = parser.parse_args()
    try:
        check(args.root)
    except (Invalid, OSError) as error:
        print(f"ci path-routing check failed: {error}", file=sys.stderr)
        return 1
    print("ci path-routing workflow contract passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
