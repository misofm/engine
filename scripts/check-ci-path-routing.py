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
import functools
import importlib.util
import os
import pathlib
import re
import shlex
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
    ".github/workflows/qualification.yml",
    ".cargo/config.toml",
    "scripts/check-release-shape.py",
}
# Issue #1043: the script-gate self-test suites. On a pull request each runs in qualification.yml's
# `gate-self-tests` job only when a changed path hits its key (ci-path-router.py SELF_TEST_INPUTS,
# in this order), and every night in nightly.yml's `moved-mutation-suites`; its gate runs on every
# change. Per suite: the job that runs its gate, the gate command, and the self-test commands.
SELF_TEST_SUITES = {
    "env-vocabulary": ("lint", "bash scripts/check-env-vocabulary.sh",
                       ("bash scripts/test-env-vocabulary.sh",)),
    "conformance-boundaries": ("lint", "bash scripts/check-conformance-boundaries.sh",
                               ("bash scripts/test-conformance-boundaries.sh",)),
    "console-benchmark": ("lint", "bash scripts/check-bench-preconditions.sh",
                          ("bash scripts/test-console-benchmark.sh",)),
    "sdk-deletions": ("sdk", "python3 -B scripts/check-sdk-deletions.py",
                      ("python3 -B scripts/check-sdk-deletions.py --self-test",)),
    "dsp-research": ("docs-gates", "bash scripts/check-dsp-research.sh",
                     ("bash scripts/test-dsp-research.sh",)),
}
SELF_TEST_SHARED_INPUTS = {".github/workflows/qualification.yml"}
# The mention rule scripts/check-script-reachability.py (#1027) applies to every workflow-reached
# script: a basename on a line that is not a comment, and a jq `include`/`import` module.
REACHABILITY_CHECKER = "scripts/check-script-reachability.py"


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


@functools.lru_cache(maxsize=None)
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
            "ci-path-router.py: RELEASE_INPUT_SUFFIX drifted from the release-shape input taxonomy")
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
    check_self_test_inputs(root, tree)


def literal_strings(node: ast.expr | None) -> list[str] | None:
    """The elements of a literal set, tuple or list of string constants, or None."""
    if not isinstance(node, (ast.Set, ast.Tuple, ast.List)):
        return None
    values = [element.value for element in node.elts
              if isinstance(element, ast.Constant) and isinstance(element.value, str)]
    return values if len(values) == len(node.elts) else None


def suite_scripts(commands: tuple[str, ...]) -> list[str]:
    """The repository scripts a suite's commands run."""
    return sorted({word for command in commands for word in shlex.split(command)
                   if word.startswith("scripts/")})


def key_covers(key: set[str], path: str) -> bool:
    return any(path == entry or (entry.endswith("/") and path.startswith(entry)) for entry in key)


def reachability_rule(root: pathlib.Path):
    source = root / REACHABILITY_CHECKER
    require(source.is_file(), f"{REACHABILITY_CHECKER} is missing: the self-test keys are checked "
            "with its mention rule")
    spec = importlib.util.spec_from_file_location("script_reachability", source)
    require(spec is not None and spec.loader is not None, f"cannot load {REACHABILITY_CHECKER}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


# The mentions of each (script, text) already scanned: a pure function of the text, kept so the
# mutation tests' in-process runs do not rescan unchanged scripts.
MENTIONS: dict[tuple[str, str], set[str]] = {}


def mention_closure(root: pathlib.Path, reachability):
    """A function from seed scripts to every file under scripts/ they mention, transitively through
    the scripts they mention, by the reachability rule's own definition of a mention (#1027)."""
    by_name: dict[str, list[str]] = {}
    for directory, subdirectories, files in os.walk(root / "scripts"):
        subdirectories[:] = sorted(name for name in subdirectories if name != "__pycache__")
        relative = pathlib.Path(directory).relative_to(root).as_posix()
        for name in sorted(files):
            by_name.setdefault(name, []).append(f"{relative}/{name}")

    def mentioned(current: str) -> set[str]:
        path = root / current
        if not current.endswith(reachability.CARRIER_SUFFIXES) or not path.is_file():
            return set()
        text = path.read_text(encoding="utf-8", errors="replace")
        if (current, text) not in MENTIONS:
            MENTIONS[(current, text)] = reachability.mentioned(current, text)
        return MENTIONS[(current, text)]

    def closure(seeds: list[str]) -> set[str]:
        reached = set(seeds)
        frontier = sorted(reached)
        while frontier:
            current = frontier.pop()
            for name in mentioned(current):
                for target in by_name.get(name, ()):
                    if target not in reached:
                        reached.add(target)
                        frontier.append(target)
        return reached

    return closure


def check_self_test_inputs(root: pathlib.Path, tree: ast.Module) -> None:
    """Issue #1043: each self-test suite's router key is every file the suite reads, not only its
    gate (amendment 4). Every script the suite runs, and every script under scripts/ those mention
    -- the gate, `lib/gate.sh`, jq libraries, validators, runners -- must be in its key, so an edit
    to any of them runs the suite. A tree read wholesale (the SDK self-test reads `sdk/`) is a
    prefix entry the rule cannot derive; the evidence in the spec records it."""
    table = router_assign(tree, "SELF_TEST_INPUTS")
    require(isinstance(table, ast.Dict),
            "ci-path-router.py: SELF_TEST_INPUTS must be a literal dict")
    suites = [key.value if isinstance(key, ast.Constant) else None for key in table.keys]
    require(suites == list(SELF_TEST_SUITES),
            "ci-path-router.py: SELF_TEST_INPUTS must name exactly the self-test suites "
            f"{list(SELF_TEST_SUITES)}, in that order")
    shared = literal_strings(router_assign(tree, "SELF_TEST_SHARED_INPUTS"))
    require(isinstance(router_assign(tree, "SELF_TEST_SHARED_INPUTS"), ast.Set)
            and shared is not None and set(shared) == SELF_TEST_SHARED_INPUTS,
            "ci-path-router.py: SELF_TEST_SHARED_INPUTS must be exactly the workflow hosting the "
            "self-test job")
    closure = mention_closure(root, reachability_rule(root))
    for suite, value in zip(suites, table.values):
        entries = literal_strings(value)
        require(isinstance(value, ast.Set) and entries is not None and entries
                and len(set(entries)) == len(entries),
                f"ci-path-router.py: SELF_TEST_INPUTS[{suite!r}] must be a literal set of paths")
        key = set(entries)
        missing = sorted(path for path in closure(suite_scripts(SELF_TEST_SUITES[suite][2]))
                         if not key_covers(key, path))
        require(not missing,
                f"ci-path-router.py: SELF_TEST_INPUTS[{suite!r}] misses files its suite reads: "
                + ", ".join(missing))


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


ROUTE_VALIDATION_STEP = """      - name: Validate path-routing policy and mutations
        run: |
          python3 -B scripts/check-ci-path-routing.py
          python3 -B scripts/test-ci-path-routing.py
"""

TEST_SUPPORT_CI_LINES = (
    "python3 -B scripts/check-test-support-ci.py\n",
    "python3 -B scripts/test-test-support-ci.py\n",
)

# check-sdk-deletions.py's --self-test moved to `gate-self-tests` (#1043); SELF_TEST_SUITES pins it.
# #1044: the SDK drift gate compares sdk/assets with the artifact job's generated documents, so it
# names the downloaded, closure-verified directory; without it the gate would run the generator.
SDK_CLOSURE_LINES = (
    "bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts",
    "python3 -B scripts/check-sdk-deletions.py",
    "bash scripts/check-sdk-types.sh",
    "bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts",
    "bash scripts/sdk-package.sh check target/ci/qualification-artifacts",
)


def check_qualification_route_job(text: str) -> None:
    """Stage-3 review S1: the route job is the only place this checker (and so the retired-workflow
    absence rule) executes in CI. It must validate policy before classifying, and it must check out
    full history so the router can diff against the real base."""
    route = job(text, "route")
    require(ROUTE_VALIDATION_STEP in route,
            "qualification.yml: route job must run the routing policy checker and its mutations "
            "before classifying")
    require("fetch-depth: 0" in route,
            "qualification.yml: route job must check out full history (fetch-depth: 0)")
    require(route.index(ROUTE_VALIDATION_STEP) < route.index("scripts/ci-path-router.py"),
            "qualification.yml: routing policy must be validated before the router runs")


def check_qualification_closures(text: str) -> None:
    """Stage-3 review S2: the SDK closure and the canonical workspace-policy gate were pinned by
    the retired sdk.yml/ci.yml rules; pin them on the surviving workflow instead."""
    sdk = job(text, "sdk")
    for line in SDK_CLOSURE_LINES:
        require(line in sdk, f"qualification.yml: sdk job is missing {line!r}")
    lint = job(text, "lint")
    require("run: bash scripts/check-workspace-policy.sh\n" in lint or
            "bash scripts/check-workspace-policy.sh\n" in lint,
            "qualification.yml: lint job must run the canonical check-workspace-policy.sh step")
    # Issue #1021: the guard that every workspace test-support feature reaches a CI test step.
    for line in TEST_SUPPORT_CI_LINES:
        require(line in lint, f"qualification.yml: lint job is missing {line!r}")


V8_SPILL_ARTIFACT_LINE = (
    "python3 -B scripts/check-web-audioworklet-v8-spill.py "
    "target/ci/qualification-artifacts/miso-engine-v1-audio-worklet.simd128.wasm"
)
# Issue #1061: every job that reads the shipped module checks its download against the digest of
# the bytes the `artifact` job built -- not against the committed pin, which is the release
# fingerprint -- so every artifact gate in a run reads exactly those bytes.
ARTIFACT_DIGEST_STEP = (
    "      - name: Verify the downloaded artifact against the artifact job's digest\n"
    "        env:\n"
    "          BUILT: ${{ needs.artifact.outputs.sha256 }}\n"
)
ARTIFACT_DIGEST_OUTPUT = (
    "    outputs:\n"
    "      sha256: ${{ steps.build.outputs.sha256 }}\n"
    "      rustc: ${{ steps.build.outputs.rustc }}\n"
    "      closure_sha256: ${{ steps.build.outputs.closure_sha256 }}\n"
)
ARTIFACT_READERS = ("sdk", "artifact-gates", "browser")
# Issue #1044: the parameter metadata and ABI layout are generated once per run, by the `artifact`
# job's delivery build (`parameter-metadata --write`), and every reader checks the whole downloaded
# directory against the digest of what that job built -- one sha256 over each file's sha256, in
# byte order of name, computed by exactly this line on both sides -- so the documents a reader
# consumes are the generator's output at this commit without the reader running it again.
ARTIFACT_CLOSURE_DIGEST = (
    "closure=\"$(cd target/ci/qualification-artifacts && find . -type f -printf '%P\\n' | "
    "LC_ALL=C sort | xargs sha256sum -- | sha256sum | awk '{print $1}')\"\n"
)
ARTIFACT_CLOSURE_VERIFY = (
    "          CLOSURE: ${{ needs.artifact.outputs.closure_sha256 }}\n",
    "          " + ARTIFACT_CLOSURE_DIGEST,
    '          [[ "$CLOSURE" =~ ^[0-9a-f]{64}$ && "$closure" == "$CLOSURE" ]] || { echo "downloaded '
    'artifact closure mismatch: the artifact job built $CLOSURE, got $closure" >&2; exit 1; }\n',
)
# The one flag that lets check-web-audioworklet.sh leave the generator out (#1044), and the only
# directory it may name: a reader's download, verified against ARTIFACT_CLOSURE_VERIFY first.
METADATA_SKIP_FLAG = "--without-metadata-regeneration"
ARTIFACT_DIRECTORY = "target/ci/qualification-artifacts"
# Issue #1061, attempt 3: a change's report compares its module with the digest its base's own CI
# run recorded, never with a base rebuilt inside the change's workflow. Attempts 1 and 2 rebuilt the
# base there, and a toolchain bump, one workflow-level cargo variable or one `GITHUB_PATH` line
# steered that rebuild and read UNCHANGED. What is pinned is what the comparison rests on:
# - on `main`, `artifact-record` posts the record: a job run only on a push to `main`, checking out
#   nothing, and the only job in the workflow holding a write permission, so no job a pull request
#   reaches can forge its base's record (#1061 attempt 3 verdict, finding 1);
# - the build step writes the digest and the rustc release the record and the report carry;
# - `artifact-identity` fetches and compares in one step with no `if:`, keeps its twin build, and
#   has the one read permission;
# - neither job carries a job-level `continue-on-error` (a failed record or comparison would still
#   pass), `defaults` (`shell: bash {0}` drops `-e` and masks a failed self-test) or `env` (a job
#   `PATH` could put another `gh` first and forge the record or the lookup).
# The build environment itself is deliberately not pinned: the record is what the base's own run
# built in its own environment, so a change to that environment reads CHANGED, as it should.
ARTIFACT_BUILD_LINES = (
    "          bash scripts/build-web-audioworklet.sh target/ci/qualification-artifacts\n",
    '          echo "sha256=$sha256" >> "$GITHUB_OUTPUT"\n',
    '          echo "rustc=$(rustc -vV | sed -n \'s/^release: //p\')" >> "$GITHUB_OUTPUT"\n',
    "          " + ARTIFACT_CLOSURE_DIGEST,
    '          echo "closure_sha256=$closure" >> "$GITHUB_OUTPUT"\n',
)
RECORD_JOB = "artifact-record"
RECORD_HEAD = (
    "    needs: [route, artifact]\n"
    "    if: github.event_name == 'push' && github.ref == 'refs/heads/main' && "
    "(needs.route.outputs.route == 'sdk' || needs.route.outputs.route == 'full')\n"
)
RECORD_PERMISSIONS = "    permissions:\n      statuses: write\n"
ARTIFACT_RECORD_STEP = """      - name: Record this commit's module for later changes to compare against
        env:
          GH_TOKEN: ${{ github.token }}
          SHA256: ${{ needs.artifact.outputs.sha256 }}
          RUSTC: ${{ needs.artifact.outputs.rustc }}
        run: |
          [[ "$SHA256" =~ ^[0-9a-f]{64}$ && "$RUSTC" =~ ^[0-9]+\\.[0-9]+\\.[0-9]+(-[A-Za-z0-9.]+)?$ ]] || { echo "malformed record: '$SHA256' rustc '$RUSTC'" >&2; exit 1; }
          gh api -X POST "repos/$GITHUB_REPOSITORY/statuses/$GITHUB_SHA" -f state=success -f context=audioworklet-sha256 -f "description=$SHA256 rustc $RUSTC" -f "target_url=$GITHUB_SERVER_URL/$GITHUB_REPOSITORY/actions/runs/$GITHUB_RUN_ID"
"""
WRITE_PERMISSION = re.compile(r"^\s+[a-z-]+: write\s*$|^\s*permissions: write-all\s*$",
                              re.MULTILINE)
IDENTITY_PERMISSIONS = "    permissions:\n      contents: read\n      statuses: read\n"
IDENTITY_CHECKOUT = re.compile(
    r"      - uses: actions/checkout@[0-9a-f]{40} # v\S+\n        with:\n          fetch-depth: 0\n\Z")
IDENTITY_INSTALL = "      - name: Install pinned Rust toolchain and Wasm standard library\n"
IDENTITY_TWIN_STEP = """      - name: Rebuild the module from another checkout path and CARGO_HOME
        run: |
          git worktree add --detach "$RUNNER_TEMP/twin" HEAD
          mkdir "$RUNNER_TEMP/twin-module"
          CARGO_HOME="$RUNNER_TEMP/twin-cargo-home" bash "$RUNNER_TEMP/twin/scripts/build-web-audioworklet.sh" --module-only "$RUNNER_TEMP/twin-module"
"""
IDENTITY_REPORT_STEP = """      - name: Report ARTIFACT CHANGED or UNCHANGED against the base's recorded digest, and hold a release change to its pin
        env:
          GH_TOKEN: ${{ github.token }}
          EVENT: ${{ github.event_name }}
          BEFORE: ${{ github.event.before }}
          BUILT: ${{ needs.artifact.outputs.sha256 }}
          RUSTC: ${{ needs.artifact.outputs.rustc }}
        run: |
          set -o pipefail
          python3 -B scripts/web-audioworklet-identity.py --self-test
          python3 -B scripts/web-audioworklet-identity.py report --event "$EVENT" --before "$BEFORE" --repository "$GITHUB_REPOSITORY" --built "$BUILT" --rustc "$RUSTC" --twin "$RUNNER_TEMP/twin-module/miso-engine-v1-audio-worklet.simd128.wasm" | tee -a "$GITHUB_STEP_SUMMARY"
"""
IDENTITY_STEPS = (IDENTITY_TWIN_STEP, IDENTITY_REPORT_STEP)
UNMASKABLE = re.compile(r"^    (continue-on-error|defaults|env):", re.MULTILINE)


def job_steps(job_text: str) -> list[str]:
    """A job's steps, each from its `      - ` line to the next, with comment-only lines and blank
    lines removed (comments document a step; they never change what it runs)."""
    body = job_text.split("    steps:\n", 1)
    require(len(body) == 2, "qualification.yml: job has no steps")
    lines = [line for line in body[1].splitlines(keepends=True)
             if line.strip() and not line.lstrip().startswith("#")]
    steps: list[str] = []
    for line in lines:
        if line.startswith("      - "):
            steps.append(line)
        else:
            require(bool(steps), "qualification.yml: text before the first step")
            steps[-1] += line
    return steps

def check_qualification_artifact_digest(text: str) -> None:
    """Issue #1061: the `artifact` job publishes its module's digest and rustc release, posts
    main's record on pushes, and every job that reads the module verifies its download against
    that digest before anything reads it. Issue #1044: the same holds for the whole directory,
    generated documents included, against the job's closure digest."""
    artifact = job(text, "artifact")
    require(ARTIFACT_DIGEST_OUTPUT in artifact,
            "qualification.yml: the artifact job must publish its module's sha256, rustc "
            "release and closure digest (#1044) as outputs")
    require(UNMASKABLE.search(artifact) is None,
            "qualification.yml: the artifact job must carry no job-level continue-on-error, "
            "defaults or env")
    steps = job_steps(artifact)
    build = [index for index, step in enumerate(steps)
             if step.startswith("      - name: Build the exact shipped artifact\n")]
    require(len(build) == 1, "qualification.yml: the artifact job must have one build step")
    for line in ARTIFACT_BUILD_LINES:
        require(line in steps[build[0]],
                f"qualification.yml: the artifact build step is missing {line.strip()!r}")
    for name in ARTIFACT_READERS:
        reader = job(text, name)
        require(re.search(r"^    needs: \[route, artifact\]$", reader, re.MULTILINE) is not None,
                f"qualification.yml: {name} must need exactly [route, artifact]")
        require(ARTIFACT_DIGEST_STEP in reader,
                f"qualification.yml: {name} must verify its download against the artifact job's "
                "digest")
        download = reader.index("path: target/ci/qualification-artifacts")
        verify = reader.index(ARTIFACT_DIGEST_STEP)
        require(download < verify, f"qualification.yml: {name} verifies before it downloads")
        verify_step = next((step for step in job_steps(reader)
                            if step.startswith(ARTIFACT_DIGEST_STEP.splitlines(keepends=True)[0])),
                           "")
        for line in ARTIFACT_CLOSURE_VERIFY:
            require(line in verify_step,
                    f"qualification.yml: {name} must verify its whole download against the artifact "
                    f"job's closure digest (#1044): missing {line.strip()!r}")
        later = [reader.index(line) for line in ("bash scripts/", "python3 -B scripts/",
                                                 "npm run qualify") if line in reader]
        require(all(verify < index for index in later),
                f"qualification.yml: {name} reads the artifact before verifying its digest")


def check_qualification_artifact_record(text: str, names: list[str]) -> None:
    """Issue #1061: main's record is posted by one job, on a push to `main` only, which checks out
    nothing and runs no repository code; it holds the workflow's only write permission, so no job a
    pull request reaches holds a write token (the top level is already exactly contents: read)."""
    record = job(text, RECORD_JOB)
    require(RECORD_HEAD in record,
            f"qualification.yml: {RECORD_JOB} must need [route, artifact] and run only on a push to "
            "main that built the module")
    require(RECORD_PERMISSIONS in record,
            f"qualification.yml: {RECORD_JOB}'s permissions must be exactly statuses: write")
    require(UNMASKABLE.search(record) is None,
            f"qualification.yml: {RECORD_JOB} must carry no job-level continue-on-error, defaults "
            "or env")
    require(job_steps(record) == [ARTIFACT_RECORD_STEP],
            f"qualification.yml: {RECORD_JOB} must be exactly its one pinned step, checking out "
            "nothing (scripts/check-ci-path-routing.py ARTIFACT_RECORD_STEP)")
    for name in names:
        if name == RECORD_JOB:
            continue
        require(WRITE_PERMISSION.search(job(text, name)) is None,
                f"qualification.yml: job {name!r} holds a write permission; only {RECORD_JOB}, "
                "which no pull request reaches, may")


def check_qualification_artifact_identity(text: str) -> None:
    """Issue #1061 (owner decision 5): every PR that builds the module reports whether it changed
    against its base's recorded digest, proves it reproducible, and holds a release change to the
    pin. The identity job must run exactly where `artifact` runs, after it, with full history, the
    one read permission, nothing that could skip or mask a step, and its twin and report steps
    exactly as pinned."""
    identity = job(text, "artifact-identity")
    require(job_if(identity) == job_if(job(text, "artifact")),
            "qualification.yml: artifact-identity must run on exactly the artifact job's routes")
    require(re.search(r"^    needs: \[route, artifact\]$", identity, re.MULTILINE) is not None,
            "qualification.yml: artifact-identity must need exactly [route, artifact]")
    require(IDENTITY_PERMISSIONS in identity,
            "qualification.yml: artifact-identity's permissions must be exactly contents: read and "
            "statuses: read")
    require(UNMASKABLE.search(identity) is None,
            "qualification.yml: artifact-identity must carry no job-level continue-on-error, "
            "defaults or env")
    steps = job_steps(identity)
    require(len(steps) == 4,
            "qualification.yml: artifact-identity must be exactly checkout, toolchain, twin and "
            "report steps")
    require(IDENTITY_CHECKOUT.fullmatch(steps[0]) is not None,
            "qualification.yml: artifact-identity must check out full history (fetch-depth: 0), "
            "or no run finds its base")
    require(steps[1].startswith(IDENTITY_INSTALL),
            "qualification.yml: artifact-identity's second step must install the toolchain")
    for step, pinned in zip(steps[2:], IDENTITY_STEPS):
        require(step == pinned,
                "qualification.yml: artifact-identity step differs from its pin: "
                f"{pinned.splitlines()[0].strip()!r} (scripts/check-ci-path-routing.py "
                "IDENTITY_STEPS)")


def check_qualification_metadata_generated_once(text: str, names: list[str]) -> None:
    """Issue #1044: `check-web-audioworklet.sh` may leave its `parameter-metadata --check` out only
    in a job that reads the `artifact` job's download and verifies the whole directory against that
    job's closure digest first (check_qualification_artifact_digest), and only for that directory:
    there the check compared the generator with itself. Anywhere else the flag would drop the only
    regeneration of a directory nothing proves the generator wrote."""
    for name in names:
        for command in step_commands(job(text, name)):
            try:
                words = shlex.split(command)
            except ValueError:
                words = command.split()
            for index, word in enumerate(words):
                if not word.endswith("check-web-audioworklet.sh"):
                    continue
                arguments = words[index + 1:]
                if METADATA_SKIP_FLAG not in arguments:
                    continue
                require(name in ARTIFACT_READERS,
                        f"qualification.yml: {name} passes {METADATA_SKIP_FLAG} but does not read "
                        "the artifact job's closure-verified download")
                require(arguments == [METADATA_SKIP_FLAG, ARTIFACT_DIRECTORY],
                        f"qualification.yml: {name} may pass {METADATA_SKIP_FLAG} only for "
                        f"{ARTIFACT_DIRECTORY}")


def check_metadata_skip_elsewhere(root: pathlib.Path) -> None:
    """Issue #1044: no other workflow may pass the flag; none of them verifies a closure digest."""
    for workflow in sorted((root / ".github/workflows").glob("*.y*ml")):
        if workflow.name == "qualification.yml":
            continue
        require(METADATA_SKIP_FLAG not in workflow.read_text(encoding="utf-8"),
                f"{workflow.name}: only qualification.yml's closure-verified artifact readers may "
                f"pass {METADATA_SKIP_FLAG} (#1044)")


# Issue #1044: each duplicate this issue removed names the one job that still runs the claim, on
# the full route, in a step no condition can skip. Dropping or conditioning any of these lines
# would take the claim out of CI with every job green.
# - artifact-gates: scripts/test-web-audioworklet.sh runs scripts/test-web-audioworklet.mjs, the
#   hermetic host/worklet/boot suite `lint` also ran;
# - lint: the effect-runtime policy and fixture scripts check-effect-contract.sh also ran;
# - test-release: a release `cargo test` runs `m3_determinism` on the x86-64-v3 build
#   (`.cargo/config.toml`, no RUSTFLAGS override on the command; see runs_m3_on_config_flags), and
#   the cfg line shows cargo builds `math` with FMA there, the claim of the retired
#   "Math M3 digests on an FMA-enabled build" step.
DEDUPLICATED_OWNERS = {
    "artifact-gates": ("bash scripts/test-web-audioworklet.sh",),
    "lint": (
        "bash scripts/check-effect-runtime-policy.sh",
        "bash scripts/test-effect-runtime-policy.sh",
        "bash scripts/check-effect-runtime-fixtures.sh",
        "bash scripts/test-effect-runtime-fixtures.sh",
    ),
    "test-release": (
        "cargo rustc --locked --release -p math --lib -- --print cfg | "
        "grep -x 'target_feature=\"fma\"'",
    ),
}
FULL_ROUTE_IF = "needs.route.outputs.route == 'full'"
M3_TEST = "m3_determinism"


def runs_m3_on_config_flags(command: str) -> bool:
    """`command` is a release-profile `cargo test` of `math` that runs `m3_determinism`, with no
    variable assignment in front of it that could replace `.cargo/config.toml`'s rustflags."""
    try:
        words = shlex.split(command)
    except ValueError:
        return False
    if words[:2] != ["cargo", "test"] or "--release" not in words or "--" in words:
        return False
    packages = {words[i + 1] for i, word in enumerate(words[:-1]) if word in ("-p", "--package")}
    tests = {words[i + 1] for i, word in enumerate(words[:-1]) if word == "--test"}
    return ("math" in packages
            and (not tests or M3_TEST in tests)
            and not any(word in NON_INTEGRATION_TARGET_SELECTORS for word in words))


def check_qualification_deduplicated_owners(text: str) -> None:
    for name, commands in DEDUPLICATED_OWNERS.items():
        block = job(text, name)
        require(job_if(block) == FULL_ROUTE_IF,
                f"qualification.yml: {name} must run on exactly the full route (#1044 owner)")
        unconditional = unconditional_step_commands(block)
        for command in commands:
            require(command in unconditional,
                    f"qualification.yml: {name} must run `{command}` unconditionally; it is the "
                    "one run left of a claim #1044 deduplicated")
    require(any(runs_m3_on_config_flags(command)
                for command in unconditional_step_commands(job(text, "test-release"))),
            "qualification.yml: test-release must run M3 (`cargo test --release -p math`, "
            f"`{M3_TEST}` not deselected) unconditionally with no RUSTFLAGS in front of it, on "
            "the x86-64-v3 flags .cargo/config.toml pins (#1044)")


def check_qualification_v8_spill(text: str) -> None:
    """Issue #1009: `wasm-guests` may leave `run-wasm-gates.sh`'s V8 spill leg out only because
    `artifact-gates` runs the same gate on the downloaded artifact, after verifying it against the
    artifact job's digest. Without this rule, deleting that step would take the gate out of CI with
    every job green."""
    wasm = job(text, "wasm-guests")
    require("bash scripts/run-wasm-gates.sh" in wasm,
            "qualification.yml: wasm-guests must run scripts/run-wasm-gates.sh")
    if "--without-v8-spill" not in run_wasm_gates_flags(wasm):
        return
    gates = job(text, "artifact-gates")
    require(V8_SPILL_ARTIFACT_LINE in gates and ARTIFACT_DIGEST_STEP in gates,
            "qualification.yml: wasm-guests runs run-wasm-gates.sh --without-v8-spill, so "
            "artifact-gates must run the V8 spill gate on the digest-verified artifact")
    require(gates.index(ARTIFACT_DIGEST_STEP) < gates.index(V8_SPILL_ARTIFACT_LINE),
            "qualification.yml: artifact-gates must verify the artifact's digest before the V8 "
            "spill gate reads it")


AARCH64_JOBS = {
    "aarch64-debug": "bash scripts/run-aarch64-tests.sh debug",
    "aarch64-release": "bash scripts/run-aarch64-tests.sh release",
}
AARCH64_TARGETS = ("aarch64-apple-ios", "aarch64-linux-android")
CROSS_TARGET_SCRIPT = "scripts/check-cross-targets.sh"


def check_qualification_aarch64(text: str) -> None:
    """Issue #1017: native AArch64 (iOS and Android arm64) is a product target. Its two test jobs
    run on arm64 hardware on every full route and the verdict expects them to succeed there, and
    cross-target installs both mobile targets for the compile rows. Any of these drifting would
    drop the mobile target out of the required workflow with every remaining job green."""
    for name, command in AARCH64_JOBS.items():
        block = job(text, name)
        require("    runs-on: ubuntu-24.04-arm\n" in block,
                f"qualification.yml: {name} must run on the ubuntu-24.04-arm runner")
        require("    if: needs.route.outputs.route == 'full'\n" in block,
                f"qualification.yml: {name} must run on every full route")
        require(f"        run: {command}\n" in block,
                f"qualification.yml: {name} must run `{command}`")
        variable = result_variable(name)
        require(f'check {name} "${variable}" "$full_expected"' in job(text, "verdict"),
                f"qualification.yml: the verdict must expect {name} to succeed on the full route")
    cross = job(text, "cross-target")
    require(f"        run: bash {CROSS_TARGET_SCRIPT}\n" in cross,
            f"qualification.yml: cross-target must run {CROSS_TARGET_SCRIPT}")
    install = next((line for line in cross.splitlines() if "rustup target add" in line), "")
    for target in AARCH64_TARGETS:
        require(target in install.split(),
                f"qualification.yml: cross-target must install the {target} standard library")


def check_cross_target_aarch64_rows(root: pathlib.Path) -> None:
    """Issue #1017: the compile rows are one `aarch64_row <target>` line per mobile target, and the
    row checks the product crates with --all-targets --all-features and lints them with clippy
    -D warnings. Deleting a row, or either half of the row, must fail here rather than silently
    shrink the matrix."""
    text = (root / CROSS_TARGET_SCRIPT).read_text(encoding="utf-8")
    lines = text.splitlines()
    for target in AARCH64_TARGETS:
        require(f"aarch64_row {target}" in lines,
                f"{CROSS_TARGET_SCRIPT}: missing the `aarch64_row {target}` compile row")
    match = re.search(r"^aarch64_row\(\) \{\n(.*?)^\}$", text, re.MULTILINE | re.DOTALL)
    require(match is not None, f"{CROSS_TARGET_SCRIPT}: missing the aarch64_row function")
    # One logical line per command: backslash continuations joined, so each flag is checked on
    # the command it belongs to and never on its neighbour.
    commands = re.sub(r"\s*\\\n\s*", " ", match.group(1)).splitlines()
    tail = ' --all-targets --all-features --target "$target" "${product_packages[@]}"'
    for tool, suffix in (("cargo check", ""), ("cargo clippy", " -- -D warnings")):
        require(any(tool in command and command.rstrip().endswith(tail + suffix)
                    for command in commands),
                f"{CROSS_TARGET_SCRIPT}: aarch64_row must run `{tool} ...{tail}{suffix}` over the "
                "product crates")
    # The iOS memset scan (#1018's expected failures): the judge refuses a product crate it has no
    # count for, so it must be handed the whole product list and must run. Scanning a hand-picked
    # subset, or dropping the judge, would read a partly fixed defect as fixed.
    for line, why in (
        ("printf '%s\\n' \"$product_list\" >\"$asm_out/products\"",
         "hand the judge every product crate"),
        ('"${known_defects[@]}" judge-memset "$asm_out/counts" "$asm_out/products" ||',
         "judge the per-crate counts"),
    ):
        require(line in lines,
                f"{CROSS_TARGET_SCRIPT}: the ios-asm-memset-pattern16 scan must {why} (`{line}`)")


# Cargo target selectors that would leave `wasm-gates`' integration tests, and with them
# `g5_native_digests_match_pins`, out of a `cargo test` invocation.
NON_INTEGRATION_TARGET_SELECTORS = (
    "--lib", "--bin", "--bins", "--example", "--examples", "--bench", "--benches", "--doc",
    "--no-run",
)


def job_if(job_text: str) -> str | None:
    match = re.search(r"^    if: (.*)$", job_text, re.MULTILINE)
    return match.group(1).strip() if match else None


def step_commands(job_text: str, conditional: bool = True) -> list[str]:
    """Every shell command line of the job's steps, with `\\` continuations joined; with
    `conditional=False`, only the steps that carry no step-level `if:`. Enough YAML for this
    workflow's fixed shape, as the rest of the checker is."""
    commands: list[str] = []
    for step in re.split(r"^      - ", job_text, flags=re.MULTILINE)[1:]:
        lines = step.splitlines()
        if not conditional and any(line.strip().startswith("if:") for line in lines):
            continue
        for index, line in enumerate(lines):
            stripped = line.strip()
            if stripped in ("run: |", "run: >"):
                body: list[str] = []
                for following in lines[index + 1:]:
                    if following.startswith("          "):
                        body.append(following.strip())
                    elif following.strip():
                        break
                commands.extend("\n".join(body).replace("\\\n", " ").splitlines())
            elif stripped.startswith("run: "):
                commands.append(stripped[len("run: "):])
    return commands


def unconditional_step_commands(job_text: str) -> list[str]:
    return step_commands(job_text, conditional=False)


def run_wasm_gates_flags(job_text: str) -> set[str]:
    """The options any invocation of `run-wasm-gates.sh` in the job passes, as tokens, so a leg the
    job leaves out is found whatever the flag order (#1009's and #1048's pairings)."""
    flags: set[str] = set()
    for command in step_commands(job_text):
        try:
            words = shlex.split(command)
        except ValueError:
            words = command.split()
        for index, word in enumerate(words):
            if word.endswith("run-wasm-gates.sh"):
                flags.update(following for following in words[index + 1:]
                             if following.startswith("--"))
    return flags


def runs_g5_native_test(command: str) -> bool:
    """`command` is a release-profile `cargo test` that runs `wasm-gates`' integration tests
    unfiltered, so it runs `g5_native_digests_match_pins`."""
    try:
        words = shlex.split(command)
    except ValueError:
        return False
    while words and re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*=.*", words[0]):
        words.pop(0)
    if words[:2] != ["cargo", "test"] or "--release" not in words or "--" in words:
        return False
    packages = {words[i + 1] for i, word in enumerate(words[:-1]) if word in ("-p", "--package")}
    tests = {words[i + 1] for i, word in enumerate(words[:-1]) if word == "--test"}
    return ("wasm-gates" in packages
            and (not tests or "g5_native_corpus" in tests)
            and not any(word in NON_INTEGRATION_TARGET_SELECTORS for word in words))


def check_qualification_native_g5(text: str) -> None:
    """Issue #1048: `wasm-guests` may leave `run-wasm-gates.sh`'s native leg out only because
    `test-release` runs the same comparison as the Rust test `g5_native_digests_match_pins`, in the
    shipping profile, on the same route. Without this rule, dropping `-p wasm-gates` from
    `test-release`, filtering its tests or making the step conditional would take the native digest
    comparison of every cross-target corpus out of CI with every job green."""
    wasm = job(text, "wasm-guests")
    if "--without-native" not in run_wasm_gates_flags(wasm):
        return
    release = job(text, "test-release")
    require(any(runs_g5_native_test(command) for command in unconditional_step_commands(release)),
            "qualification.yml: wasm-guests runs run-wasm-gates.sh --without-native, so "
            "test-release must run an unconditional, unfiltered `cargo test --release -p "
            "wasm-gates` (g5_native_digests_match_pins)")
    require(job_if(release) == job_if(wasm),
            "qualification.yml: test-release must run on exactly the route wasm-guests runs on, "
            "because it owns the native leg wasm-guests leaves out")


# GitHub-hosted AArch64 runner labels: the `-arm` Ubuntu images and the Apple-silicon macOS images
# (macOS 14 onward, except the Intel `-large` sizes).
AARCH64_RUNNER = re.compile(r"\b(?:ubuntu-[0-9.]+-arm|macos-(?:1[4-9]|[2-9][0-9]|latest)(?!-large))\b")


def check_qualification_aarch64_g5(text: str, names: list[str]) -> None:
    """Issue #1048, amendment 2: once a required job runs on AArch64 hardware (#1017), one of those
    jobs must run the single native owner of the cross-target digest corpora,
    `g5_native_digests_match_pins`, in the shipping profile. It is the only native test left that
    compares the effect families' pins, so an AArch64 leg without it would prove nothing about
    phones rendering the same bits as x86 and the browser."""
    aarch64 = [name for name in names if AARCH64_RUNNER.search(job(text, name))]
    if not aarch64:
        return
    require(any(runs_g5_native_test(command)
                for name in aarch64 for command in unconditional_step_commands(job(text, name))),
            "qualification.yml: an AArch64 job (" + ", ".join(aarch64) + ") must run an "
            "unconditional, unfiltered `cargo test --release -p wasm-gates` "
            "(g5_native_digests_match_pins)")


SELF_TEST_JOB = "gate-self-tests"
SELF_TEST_JOB_HEAD = "    needs: route\n    if: needs.route.outputs.self_tests != '[]'\n"
SELF_TEST_STEP_IF = "contains(fromJSON(needs.route.outputs.self_tests), '{suite}')"
SELF_TEST_ROUTE_LINES = (
    "      self_tests: ${{ steps.classify.outputs.self_tests }}\n",
    '            | tail -n 4 >> "$GITHUB_OUTPUT"\n',
)
SELF_TEST_VERDICT_LINES = (
    "      SELF_TESTS: ${{ needs.route.outputs.self_tests }}\n",
    '          [[ "$SELF_TESTS" =~ ^\\[(\\"[a-z-]+\\"(,\\"[a-z-]+\\")*)?\\]$ ]] || '
    '{ echo "malformed self_tests: $SELF_TESTS" >&2; exit 1; }\n',
    "          self_tests_expected=skipped\n",
    "          [[ \"$SELF_TESTS\" != '[]' ]] && self_tests_expected=success\n",
    '          check gate-self-tests "$GATE_SELF_TESTS_RESULT" "$self_tests_expected"\n',
)
NIGHTLY_SELF_TEST_JOB = "moved-mutation-suites"
NIGHTLY_STEP_IF = "${{ !cancelled() }}"
STEP_IF = re.compile(r"^        if: (.*)$", re.MULTILINE)


def step_if(step: str) -> str | None:
    match = STEP_IF.search(step)
    return match.group(1).strip() if match else None


def check_qualification_self_tests(text: str) -> None:
    """Issue #1043: each self-test suite runs in `gate-self-tests` exactly when the router selects
    it, the job runs exactly when the router selects any, the verdict expects success then and
    `skipped` otherwise, and every suite's gate still runs in an unconditional step of its per-PR
    job. Dropping a suite's step, widening or narrowing its condition, masking it, or dropping a
    gate from its per-PR job would each take a discrimination out of CI with every job green."""
    route = job(text, "route")
    for line in SELF_TEST_ROUTE_LINES:
        require(line in route, f"qualification.yml: route job is missing {line.strip()!r}")
    block = job(text, SELF_TEST_JOB)
    require(SELF_TEST_JOB_HEAD in block,
            f"qualification.yml: {SELF_TEST_JOB} must need route and run exactly when the router "
            "selects a suite")
    require(UNMASKABLE.search(block) is None,
            f"qualification.yml: {SELF_TEST_JOB} must carry no job-level continue-on-error, "
            "defaults or env")
    steps = job_steps(block)
    conditions = {SELF_TEST_STEP_IF.format(suite=suite): suite for suite in SELF_TEST_SUITES}
    for step in steps:
        condition = step_if(step)
        require(condition is None or condition in conditions,
                f"qualification.yml: {SELF_TEST_JOB} step has an unexpected condition "
                f"{condition!r}")
        require("continue-on-error:" not in step,
                f"qualification.yml: {SELF_TEST_JOB} steps must not continue on error")
    for suite, (host, gate, commands) in SELF_TEST_SUITES.items():
        selected = [step for step in steps
                    if step_if(step) == SELF_TEST_STEP_IF.format(suite=suite)]
        require(len(selected) == 1,
                f"qualification.yml: {SELF_TEST_JOB} must run suite {suite!r} in exactly one step "
                "conditioned on the router selecting it")
        require(step_commands(selected[0]) == list(commands),
                f"qualification.yml: {SELF_TEST_JOB}'s {suite!r} step must run exactly {commands}")
        require(gate in unconditional_step_commands(job(text, host)),
                f"qualification.yml: {host} must still run the {suite!r} gate `{gate}` "
                "unconditionally")
    verdict = job(text, "verdict")
    for line in SELF_TEST_VERDICT_LINES:
        require(line in verdict, f"qualification.yml: verdict is missing {line.strip()!r}")


def check_nightly_self_tests(root: pathlib.Path) -> None:
    """Issue #1043: every self-test suite runs every night, in a job the failure notice reports,
    each step running even after another failed, none masked."""
    text = (root / ".github/workflows/nightly.yml").read_text(encoding="utf-8")
    block = job(text, NIGHTLY_SELF_TEST_JOB)
    require(job_if(block) is None and UNMASKABLE.search(block) is None,
            f"nightly: {NIGHTLY_SELF_TEST_JOB} must run every night, unmasked")
    steps = job_steps(block)
    for suite, (_, _, commands) in SELF_TEST_SUITES.items():
        for command in commands:
            require(any(command in step_commands(step)
                        and step_if(step) in (None, NIGHTLY_STEP_IF)
                        and "continue-on-error:" not in step for step in steps),
                    f"nightly: {NIGHTLY_SELF_TEST_JOB} must run `{command}` ({suite}) every night, "
                    f"unmasked, unconditionally or under {NIGHTLY_STEP_IF}")
    notice = job(text, "failure-notice")
    needs = re.search(r"^    needs: \[(.*?)\]", notice, re.MULTILINE | re.DOTALL)
    require(needs is not None and NIGHTLY_SELF_TEST_JOB in
            [name.strip() for name in needs.group(1).replace("\n", " ").split(",")],
            f"nightly: failure-notice must report {NIGHTLY_SELF_TEST_JOB}")


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
    check_qualification_route_job(text)
    check_qualification_closures(text)
    check_qualification_artifact_digest(text)
    check_qualification_artifact_record(text, names)
    check_qualification_artifact_identity(text)
    check_qualification_metadata_generated_once(text, names)
    check_qualification_deduplicated_owners(text)
    check_qualification_v8_spill(text)
    check_qualification_aarch64(text)
    check_qualification_native_g5(text)
    check_qualification_aarch64_g5(text, names)
    check_qualification_self_tests(text)


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


NIGHTLY_BUDGET_COMMANDS = [
    'cargo test --locked --release -p host-web --lib -- --ignored --exact tests::maximum_document_dense_invalid_boot_finishes_under_one_second_in_release',
    'cargo test --locked --release -p host-core --test effect_observation -- --ignored --exact observation_cost_classes_are_separated_from_a_computed_scan_in_release',
    'cargo test --locked --release -p host-core --test prepare -- --ignored --exact dense_refusal_diagnostics_finish_under_one_second_in_release',
]


def nightly_budget_script(root: pathlib.Path) -> str:
    text = (root / ".github/workflows/nightly.yml").read_text(encoding="utf-8")
    block = job(text, "release-budgets")
    marker = "      - name: Release-mode wall-clock budget tests\n        run: |\n"
    require(block.count(marker) == 1, "nightly: missing exact budget step")
    body = block.split(marker, 1)[1]
    require(all(not line.strip() or line.startswith("          ")
                for line in body.splitlines()), "nightly: unexpected budget step content")
    return "\n".join(line[10:] for line in body.splitlines() if line.strip()) + "\n"


def check_nightly_budgets(root: pathlib.Path) -> None:
    expected = "set -euo pipefail\n" + "\n".join(NIGHTLY_BUDGET_COMMANDS) + "\n"
    require(nightly_budget_script(root) == expected,
            "nightly: run each of the three exact release budgets once with failure propagation")


def check(root: pathlib.Path) -> None:
    check_retired_workflows_absent(root)
    check_classifier_contract(root)
    check_qualification_workflow(root)
    check_metadata_skip_elsewhere(root)
    check_cross_target_aarch64_rows(root)
    check_nightly_budgets(root)
    check_nightly_self_tests(root)


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
