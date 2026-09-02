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

EVIDENCE = [".github/ISSUE_SPECS/**", "docs/**", "README.md"]
SDK = [
    "sdk/**",
    "scripts/check-sdk-deletions.py",
    "scripts/check-sdk-generated.sh",
    "scripts/check-sdk-headless.sh",
    "scripts/check-sdk-types.sh",
    "scripts/sdk-package.sh",
    "LICENSE",
]
RELEASE_PR_INPUTS = [
    "**/Cargo.toml",
    "Cargo.lock",
    "rust-toolchain.toml",
    "scripts/run-release-workspace-tests.sh",
    ".github/workflows/release-build.yml",
]


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


def path_values(block: str) -> list[str]:
    return re.findall(r"^      - '([^']+)'$", block, re.MULTILINE)


def check_trigger(text: str, workflow: str, push_ignores: list[str]) -> None:
    push = section(text, "  push:", ("  pull_request:", "  workflow_dispatch:"))
    require("branches:\n      - main" in push, f"{workflow}: push must target main")
    require(path_values(push) == push_ignores, f"{workflow}: push paths-ignore set drifted")
    pull_request = section(text, "  pull_request:", ("  workflow_dispatch:",))
    require("branches:\n      - main" in pull_request, f"{workflow}: pull requests must target main")
    if workflow != "release-build.yml":
        require("paths:" not in pull_request and "paths-ignore:" not in pull_request,
                f"{workflow}: required pull-request workflow cannot be path-filtered")
    else:
        require(path_values(pull_request) == RELEASE_PR_INPUTS,
                "release-build.yml: pull-request release-input filter drifted")
    require("  workflow_dispatch:\n" in text, f"{workflow}: missing manual dispatch")


def check_common(text: str, workflow: str, domain: str) -> None:
    require(f"group: {domain}-${{{{ github.workflow }}}}-${{{{ github.ref }}}}" in text,
            f"{workflow}: concurrency must be ref-scoped in its own domain")
    require("cancel-in-progress: true" in text, f"{workflow}: missing cancellation")


def check_router(text: str, workflow: str) -> None:
    route = job(text, "route")
    require("route: ${{ steps.classify.outputs.route }}" in route,
            f"{workflow}: router must expose route output")
    require("fetch-depth: 0" in route, f"{workflow}: router needs complete history")
    require("scripts/ci-path-router.py" in route and "--event \"${{ github.event_name }}\"" in route,
            f"{workflow}: router must route dispatch through the fail-safe classifier")


def result_variable(job_name: str) -> str:
    return job_name.replace("-", "_").upper() + "_RESULT"


def check_aggregate(
    text: str,
    workflow: str,
    title: str,
    heavy: list[str],
    selected: str,
    selected_routes: tuple[str, ...],
    skipped_routes: tuple[str, ...],
) -> None:
    aggregate = job(text, "qualification")
    job_name_count = len(re.findall(rf"^    name: {re.escape(title)}$", text, re.MULTILINE))
    require(job_name_count == 1 and f"name: {title}" in aggregate,
            f"{workflow}: aggregate name must be exactly once as {title!r}")
    require(re.search(r"^    if: always\(\)$", aggregate, re.MULTILINE) is not None,
            f"{workflow}: aggregate job itself must always run")
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


def check_classifier_fallback(root: pathlib.Path) -> None:
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


def check(root: pathlib.Path) -> None:
    workflows = root / ".github/workflows"
    ci = (workflows / "ci.yml").read_text(encoding="utf-8")
    browser = (workflows / "browser-qualification.yml").read_text(encoding="utf-8")
    sdk = (workflows / "sdk.yml").read_text(encoding="utf-8")
    release = (workflows / "release-build.yml").read_text(encoding="utf-8")

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
                    "needs.route.outputs.route == 'full'", ("full",), ("sdk|evidence",))
    check_aggregate(browser, "browser-qualification.yml", "browser qualification",
                    ["artifact", "browser"], "needs.route.outputs.route == 'full'",
                    ("full",), ("sdk|evidence",))
    check_aggregate(sdk, "sdk.yml", "SDK qualification", ["sdk"],
                    "needs.route.outputs.route == 'sdk' || needs.route.outputs.route == 'full'",
                    ("full|sdk",), ("evidence",))
    check_sdk_closure(sdk)
    check_classifier_fallback(root)
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
