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

SDK_FILES = [
    "scripts/check-sdk-deletions.py",
    "scripts/check-sdk-generated.sh",
    "scripts/check-sdk-headless.sh",
    "scripts/check-sdk-types.sh",
    "scripts/sdk-package.sh",
    "scripts/test-sdk-artifact-builder-output-contract.sh",
]
GIT_DIFF_OPTIONS = ("--name-status", "-z", "--find-renames", "--find-copies-harder")
# Mirrors of ci-path-router.py's own math_closure/release_inputs step-condition constants (design
# #359 WP-1 §2/§5). AST-pinned by `check_classifier_contract` below the same way SDK_FILES and
# GIT_DIFF_OPTIONS are, so router/checker drift on these is caught by the policy checker rather
# than only by test-ci-path-routing.py's direct assertions.
DSP_RESEARCH_PREFIX = "dsp-research/"
MATH_CLOSURE_PREFIXES = ("crates/math/", "crates/lane/")
MATH_CLOSURE_FILES = {"Cargo.lock", "Cargo.toml", "rust-toolchain.toml", ".cargo/config.toml"}
RELEASE_INPUT_SUFFIX = "/Cargo.toml"
RELEASE_INPUT_FILES = {
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain.toml",
    "scripts/run-release-workspace-tests.sh",
    ".github/workflows/release-build.yml",
    ".cargo/config.toml",
    "scripts/check-release-shape.py",
}


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


def result_variable(job_name: str) -> str:
    return job_name.replace("-", "_").upper() + "_RESULT"


def router_assign(tree: ast.Module, name: str) -> ast.expr | None:
    """The literal value assigned to a single top-level `NAME = ...` in ci-path-router.py."""
    return next((
        node.value for node in tree.body
        if isinstance(node, ast.Assign)
        and any(isinstance(target, ast.Name) and target.id == name for target in node.targets)
    ), None)


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
    sdk_files = router_assign(tree, "SDK_FILES")
    require(isinstance(sdk_files, ast.Set), "ci-path-router.py: SDK_FILES must be a literal set")
    values = {
        element.value for element in sdk_files.elts
        if isinstance(element, ast.Constant) and isinstance(element.value, str)
    }
    expected = set(SDK_FILES)
    require(len(values) == len(sdk_files.elts) and values == expected,
            "ci-path-router.py: exact SDK file taxonomy drifted (LICENSE is full-route owned)")
    git_options = router_assign(tree, "GIT_DIFF_OPTIONS")
    require(isinstance(git_options, ast.Tuple),
            "ci-path-router.py: GIT_DIFF_OPTIONS must be a literal tuple")
    options = tuple(
        element.value for element in git_options.elts
        if isinstance(element, ast.Constant) and isinstance(element.value, str)
    )
    require(len(options) == len(git_options.elts) and options == GIT_DIFF_OPTIONS,
            "ci-path-router.py: Git diff must discover copies from unchanged full-route sources")
    dsp_research_prefix = router_assign(tree, "DSP_RESEARCH_PREFIX")
    require(isinstance(dsp_research_prefix, ast.Constant)
            and dsp_research_prefix.value == DSP_RESEARCH_PREFIX,
            "ci-path-router.py: DSP_RESEARCH_PREFIX drifted from the evidence-route taxonomy")
    math_closure_prefixes = router_assign(tree, "MATH_CLOSURE_PREFIXES")
    require(isinstance(math_closure_prefixes, ast.Tuple),
            "ci-path-router.py: MATH_CLOSURE_PREFIXES must be a literal tuple")
    prefixes = tuple(
        element.value for element in math_closure_prefixes.elts
        if isinstance(element, ast.Constant) and isinstance(element.value, str)
    )
    require(len(prefixes) == len(math_closure_prefixes.elts) and prefixes == MATH_CLOSURE_PREFIXES,
            "ci-path-router.py: math's reverse workspace closure (MATH_CLOSURE_PREFIXES) drifted")
    math_closure_files = router_assign(tree, "MATH_CLOSURE_FILES")
    require(isinstance(math_closure_files, ast.Set),
            "ci-path-router.py: MATH_CLOSURE_FILES must be a literal set")
    closure_files = {
        element.value for element in math_closure_files.elts
        if isinstance(element, ast.Constant) and isinstance(element.value, str)
    }
    require(len(closure_files) == len(math_closure_files.elts) and closure_files == MATH_CLOSURE_FILES,
            "ci-path-router.py: shared math_closure config-file set (MATH_CLOSURE_FILES) drifted")
    release_input_suffix = router_assign(tree, "RELEASE_INPUT_SUFFIX")
    require(isinstance(release_input_suffix, ast.Constant)
            and release_input_suffix.value == RELEASE_INPUT_SUFFIX,
            "ci-path-router.py: RELEASE_INPUT_SUFFIX drifted from the release-build.yml PR filter")
    release_input_files = router_assign(tree, "RELEASE_INPUT_FILES")
    require(isinstance(release_input_files, ast.Set),
            "ci-path-router.py: RELEASE_INPUT_FILES must be a literal set")
    input_files = {
        element.value for element in release_input_files.elts
        if isinstance(element, ast.Constant) and isinstance(element.value, str)
    }
    require(len(input_files) == len(release_input_files.elts) and input_files == RELEASE_INPUT_FILES,
            "ci-path-router.py: exact release_inputs file taxonomy (RELEASE_INPUT_FILES) drifted")
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


def check_qualification_no_path_filter(text: str) -> None:
    """qualification.yml is the always-reporting required workflow (design #359 §4/§7): a
    `paths:`/`paths-ignore:` filter on any of its triggers could leave a PR's required context
    pending forever, so every leaf job is gated by the router's `if:` instead."""
    on_block = section(text, "on:", ("concurrency:",))
    require("paths:" not in on_block and "paths-ignore:" not in on_block,
            "qualification.yml: on: must carry no paths:/paths-ignore: filter on any trigger")
    require("pull_request:" in on_block and "push:" in on_block and "workflow_dispatch:" in on_block,
            "qualification.yml: must trigger on pull_request, push, and workflow_dispatch")


def check_qualification_permissions(text: str) -> None:
    """The workflow-wide default must itself be read-only; a job-level override (verdict's
    `actions: read` addition) does not relax this, because check_mapping_structure has already
    pinned the top-level key order to name/on/concurrency/permissions/env/jobs."""
    head = text.split("\njobs:\n", 1)[0]
    match = re.search(r"^permissions:\n((?:  .+\n)+)", head, re.MULTILINE)
    require(match is not None, "qualification.yml: missing top-level permissions:")
    require(match.group(1) == "  contents: read\n",
            "qualification.yml: top-level permissions must be exactly contents: read")


def qualification_job_names(text: str) -> list[str]:
    jobs_block = section(text, "jobs:", ())
    names = [
        line.strip()[:-1] for line in jobs_block.splitlines()
        if line.startswith("  ") and not line.startswith("    ") and line.strip()
        and not line.lstrip().startswith("#") and line.rstrip().endswith(":")
    ]
    require(len(names) >= 2, "qualification.yml: could not enumerate any jobs")
    return names


def check_qualification_timeouts(text: str, names: list[str]) -> None:
    """Every leaf has a timeout (design §7): a hung runner must fail the verdict, not pend for
    six hours behind a required context."""
    for name in names:
        block = job(text, name)
        require(re.search(r"^    timeout-minutes: \d+$", block, re.MULTILINE) is not None,
                f"qualification.yml: job {name!r} is missing timeout-minutes")


def check_qualification_verdict_needs(text: str, names: list[str]) -> None:
    verdict = job(text, "verdict")
    others = [name for name in names if name != "verdict"]
    match = re.search(r"^    needs: \[(.+)\]$", verdict, re.MULTILINE)
    require(match is not None, "qualification.yml: verdict must declare needs: [...]")
    needs = [item.strip() for item in match.group(1).split(",")]
    require(needs == others,
            "qualification.yml: verdict's needs must equal the set of every other job, in job order")


def check_qualification_expectation_table(text: str, names: list[str]) -> None:
    """The static expectation table -- not a leaf `if:` echoed back at itself -- must mention
    every job so a leaf's `if:` drifting from it fails the verdict in both directions (design §7).
    `route` drives the table rather than being checked by it, so it is exempt. S4: a name grep
    alone is not a semantics check -- `check sdk "$LINT_RESULT"` would still mention 'sdk' by
    name, so the exact `check <job> "$<JOB>_RESULT"` pairing (uppercase, hyphens to underscores)
    is required, not merely the job name's presence somewhere in the table."""
    verdict = job(text, "verdict")
    for name in names:
        if name in ("route", "verdict"):
            continue
        variable = result_variable(name)
        require(re.search(rf'check {re.escape(name)} "\${re.escape(variable)}"', verdict) is not None,
                f"qualification.yml: expectation table does not check {name!r} against "
                f"\"${variable}\"")


def check_qualification_verdict_always(text: str) -> None:
    """S3: `if: always()` is the single property that makes the verdict -- the workflow's one
    required context -- always report, rather than resolving to `skipped` and leaving a required
    check pending forever (design §4/§7)."""
    verdict = job(text, "verdict")
    require(re.search(r"^    if: always\(\)$", verdict, re.MULTILINE) is not None,
            "qualification.yml: verdict must run unconditionally (if: always())")


def check_qualification_verdict_permissions(text: str) -> None:
    """The verdict's job-level permissions override the read-only top-level default (already
    pinned by check_qualification_permissions) to add exactly the `actions: read` its telemetry
    step needs to call the Actions API -- never more."""
    verdict = job(text, "verdict")
    require("    permissions:\n      actions: read\n      contents: read\n" in verdict,
            "qualification.yml: verdict permissions must be exactly actions: read and "
            "contents: read")


def check_qualification_concurrency(text: str) -> None:
    """S9: a superseded PR run must still be cancellable, but an unconditional
    `cancel-in-progress: true` also cancels a main push's own successor before its
    `save-if: github.ref == main` rust-cache post steps run, so under frequent merges the caches
    every PR depends on may never be written. Exactly this expression -- cancel only on
    pull_request, never on a main push -- is required, not merely the key's presence."""
    concurrency_block = section(text, "concurrency:", ("permissions:",))
    require("cancel-in-progress: ${{ github.event_name == 'pull_request' }}" in concurrency_block,
            "qualification.yml: concurrency must cancel pull_request runs only, "
            "never a main push")


def check_qualification_release_shape_guard(text: str) -> None:
    """S4: release-shape must be selected only when release_inputs is actually true -- dropping
    this guard would run the metadata/panic-clobber policy unconditionally on every full route,
    contradicting its own `needs.route.outputs.release_inputs == 'true'` job-level `if:`."""
    verdict = job(text, "verdict")
    require('[[ "$RELEASE_INPUTS" == "true" ]] && release_shape_expected=success' in verdict,
            "qualification.yml: release-shape expectation must be conditioned on "
            '"$RELEASE_INPUTS" == "true"')


def check_qualification_workflow(root: pathlib.Path) -> None:
    text = (root / ".github/workflows/qualification.yml").read_text(encoding="utf-8")
    check_qualification_no_path_filter(text)
    check_qualification_permissions(text)
    check_qualification_concurrency(text)
    names = qualification_job_names(text)
    check_qualification_timeouts(text, names)
    check_qualification_verdict_needs(text, names)
    check_qualification_verdict_always(text)
    check_qualification_verdict_permissions(text)
    check_qualification_expectation_table(text, names)
    check_qualification_release_shape_guard(text)


RETIRED_WORKFLOWS = ("ci.yml", "sdk.yml", "browser-qualification.yml", "release-build.yml")


def check_retired_workflows_absent(root: pathlib.Path) -> None:
    """Design #359 §12 stage 3: qualification.yml is the sole required PR workflow now that it
    has reported on >= 10 PRs alongside the four workflows it replaces (stage 1/2). If any of
    ci.yml, sdk.yml, browser-qualification.yml or release-build.yml reappears, every PR would
    silently need multiple required contexts again, reverting the migration without anyone
    updating branch protection to notice."""
    workflows = root / ".github/workflows"
    for name in RETIRED_WORKFLOWS:
        require(not (workflows / name).exists(),
                f"{name}: retired workflow must not exist (design #359 §12 stage 3)")


def check(root: pathlib.Path) -> None:
    check_retired_workflows_absent(root)
    check_classifier_contract(root)
    check_qualification_workflow(root)


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
