#!/usr/bin/env python3
"""Hermetic mutation tests for issue #328's classifier and workflow contract."""
from __future__ import annotations

import pathlib
import importlib.util
import json
import os
import shutil
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(__file__).resolve().parent.parent
ROUTER = ROOT / "scripts/ci-path-router.py"
CHECKER = ROOT / "scripts/check-ci-path-routing.py"
TEST = ROOT / "scripts/test-ci-path-routing.py"


def run(*args: str) -> str:
    result = subprocess.run(args, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
    if result.returncode:
        raise AssertionError(f"command failed: {' '.join(args)}\n{result.stderr}")
    return result.stdout.strip()


def run_at(root: pathlib.Path, *args: str) -> str:
    result = subprocess.run(args, cwd=root, text=True, stdout=subprocess.PIPE,
                            stderr=subprocess.PIPE, check=False)
    if result.returncode:
        raise AssertionError(f"command failed: {' '.join(args)}\n{result.stderr}")
    return result.stdout.strip()


def run_bytes_at(root: pathlib.Path, *args: str) -> bytes:
    result = subprocess.run(args, cwd=root, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                            check=False)
    if result.returncode:
        raise AssertionError(
            f"command failed: {' '.join(args)}\n{result.stderr.decode(errors='replace')}"
        )
    return result.stdout


def route_with(router: pathlib.Path, *paths: str) -> str:
    args = [sys.executable, str(router), "--event", "pull_request"]
    for path in paths:
        args += ["--path", path]
    return run(*args)


def route(*paths: str) -> str:
    return route_with(ROUTER, *paths)


def route_flags_with(router: pathlib.Path, *args: str) -> tuple[str, str, str]:
    """Run the router in `--flags` mode and return (route, math_closure, release_inputs).

    `--flags` mode prints only `key=value` lines -- no bare route line first -- so its whole
    stdout can be appended straight to `$GITHUB_OUTPUT`.
    """
    result = subprocess.run([sys.executable, str(router), "--flags", *args],
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, check=False)
    if result.returncode:
        raise AssertionError(f"command failed: {args}\n{result.stderr}")
    lines = result.stdout.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if len(lines) != 3:
        raise AssertionError(f"--flags must print exactly 3 lines, got {lines!r}")
    route_line, math_line, release_line = lines
    if not (route_line.startswith("route=") and math_line.startswith("math_closure=")
            and release_line.startswith("release_inputs=")):
        raise AssertionError(f"--flags lines have the wrong shape: {lines!r}")
    return (route_line[len("route="):], math_line[len("math_closure="):],
            release_line[len("release_inputs="):])


def route_flags(*paths: str) -> tuple[str, str, str]:
    args = ["--event", "pull_request"]
    for path in paths:
        args += ["--path", path]
    return route_flags_with(ROUTER, *args)


def test_new_router_behaviours() -> None:
    """Mutation coverage for issue #359 WP-1: dsp-research evidence, the new SDK script, and the
    `--flags` math-closure/release-inputs outputs. Pure router-argument assertions only -- these
    do not depend on any workflow file, so they run and report independently of whether
    .github/workflows/*.yml has been synchronized with the extended router/checker taxonomy yet.
    """
    # dsp-research/**/*.md joins the evidence family; any other dsp-research/ path is unknown
    # (fail-safe full), exactly like every other unclassified path.
    assert route("dsp-research/notes/topic.md") == "evidence"
    assert route("dsp-research/notes/topic.md", "docs/x.md", "README.md") == "evidence"
    assert route("dsp-research/notes/topic.md", "sdk/src/index.ts") == "sdk"
    assert route("dsp-research/data.wav") == "full"
    assert route("dsp-research/nested/dir/README") == "full"
    assert route("dsp-research/notes/topic.md", "dsp-research/data.wav") == "full"

    # scripts/test-sdk-artifact-builder-output-contract.sh joins the SDK ownership set.
    assert route("scripts/test-sdk-artifact-builder-output-contract.sh") == "sdk"
    assert route("scripts/test-sdk-artifact-builder-output-contract.sh", "sdk/x.ts") == "sdk"
    assert route("scripts/test-sdk-artifact-builder-output-contract.sh", "docs/x.md") == "sdk"

    # --flags: an ordinary path that is neither closure gets both flags false.
    assert route_flags("crates/engine/src/lib.rs") == ("full", "false", "false")
    assert route_flags("crates/session/src/lib.rs") == ("full", "false", "false")

    # math_closure: crates/math/**, crates/lane/** are math_closure-only (math's reverse
    # workspace closure is exactly {lane}); the four root config files are shared with
    # release_inputs.
    for math_only in ("crates/math/src/lib.rs", "crates/lane/src/lib.rs",
                       "crates/math/tests/m1.rs", "crates/lane/tests/g1.rs"):
        assert route_flags(math_only) == ("full", "true", "false"), math_only
    for shared in ("Cargo.lock", "Cargo.toml", "rust-toolchain.toml", ".cargo/config.toml"):
        assert route_flags(shared) == ("full", "true", "true"), shared

    # release_inputs: any Cargo.toml, Cargo.lock, rust-toolchain.toml, .cargo/config.toml, the
    # release test runner, the shape-policy script, and qualification.yml itself (it hosts the
    # release-shape job, so editing that job must select it).
    for release_only in (
        "scripts/run-release-workspace-tests.sh",
        ".github/workflows/qualification.yml",
        "scripts/check-release-shape.py",
        "crates/session/Cargo.toml",
        "hosts/host-web/Cargo.toml",
        "Cargo.toml",
    ):
        route_value, math_closure, release_inputs = route_flags(release_only)
        assert release_inputs == "true", release_only
        # Only the bare root Cargo.toml is also in the math_closure set; every other release
        # input here is release_inputs-only.
        assert math_closure == ("true" if release_only == "Cargo.toml" else "false"), release_only

    # Mixed: a math path with an evidence path still routes full (union, fail-safe) and stays
    # math_closure true; release_inputs is computed from the same path set and is false here.
    assert route_flags("crates/math/src/lib.rs", "docs/x.md") == ("full", "true", "false")

    # Mixed: an SDK path with a release-input manifest routes full (Cargo.lock is not SDK-owned)
    # and both closure flags are still computed from the whole path set, independent of route.
    assert route_flags("sdk/src/index.ts", "Cargo.lock") == ("full", "true", "true")

    # Fail-safe reasons force both flags true, never because of what a real path happened to be.
    dispatch = subprocess.run([sys.executable, str(ROUTER), "--event", "workflow_dispatch", "--flags"],
                              stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, check=False)
    assert dispatch.returncode == 0
    assert dispatch.stdout == "route=full\nmath_closure=true\nrelease_inputs=true\n"

    assert route_flags_with(ROUTER, "--event", "pull_request") == ("full", "true", "true")  # missing base
    assert route_flags_with(
        ROUTER, "--event", "pull_request", "--base", "definitely-not-a-revision", "--head", "HEAD",
    ) == ("full", "true", "true")  # malformed diff

    with tempfile.NamedTemporaryFile() as status:
        status.write(b"")  # empty diff
        status.flush()
        assert route_flags_with(
            ROUTER, "--event", "pull_request", "--name-status-file", status.name,
        ) == ("full", "true", "true")

    with tempfile.NamedTemporaryFile() as status:
        status.write(b"U\0sdk/src/index.ts\0")  # unrecognised status code
        status.flush()
        assert route_flags_with(
            ROUTER, "--event", "pull_request", "--name-status-file", status.name,
        ) == ("full", "true", "true")

    # R1 (WP-1 review S4): a path `path_kind` refuses as untrusted (leading `/`, a literal
    # backslash, or a `/../` traversal segment) must force both closure flags fail-safe true, the
    # same way it forces `route` to `full` -- never silently compute "definitely not math /
    # definitely not a release input" for a name the router does not trust the shape of.
    for untrusted in (
        "/crates/math/src/lib.rs",
        "docs/../crates/math/src/lib.rs",
        "crates\\math\\src\\lib.rs",
        "hosts/../Cargo.lock",
        "crates/lane/../math/src/lib.rs",
    ):
        assert route_flags(untrusted) == ("full", "true", "true"), untrusted

    # A well-formed but unrecognised path is not untrusted, so it must keep computing the flags
    # normally instead of being swept into the same fail-safe: this is a deliberate, documented
    # divergence from design #359 WP-1 §2's literal sentence (an unknown well-formed path forces
    # `route` to `full` but cannot affect `math_closure`/`release_inputs`, which are exact,
    # named prefix/suffix sets -- see `compute_flags`'s docstring).
    assert route_flags("LICENSE") == ("full", "false", "false")

    # Bare mode (no --flags) is unchanged: exactly one line, the route, for existing callers.
    bare_result = subprocess.run(
        [sys.executable, str(ROUTER), "--event", "pull_request", "--path", "sdk/x.ts"],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, check=False,
    )
    assert bare_result.returncode == 0
    assert bare_result.stdout == "sdk\n"


def checker_fails(root: pathlib.Path) -> None:
    result = subprocess.run([sys.executable, str(CHECKER), "--root", str(root)],
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, check=False)
    if result.returncode == 0:
        raise AssertionError("workflow mutation was accepted")


def workspace() -> pathlib.Path:
    """A scratch root carrying only the surviving qualification.yml -- design #359 §12 stage 3
    retired ci.yml, sdk.yml, browser-qualification.yml and release-build.yml, and
    check_retired_workflows_absent fails if any of the four exists here, so this workspace must
    never copy them in."""
    root = pathlib.Path(tempfile.mkdtemp(prefix="ci-path-routing-"))
    (root / ".github/workflows").mkdir(parents=True)
    (root / "scripts").mkdir()
    shutil.copy2(ROOT / ".github/workflows/qualification.yml",
                 root / ".github/workflows/qualification.yml")
    shutil.copy2(ROOT / ".github/workflows/nightly.yml",
                 root / ".github/workflows/nightly.yml")
    shutil.copy2(ROUTER, root / "scripts/ci-path-router.py")
    shutil.copy2(CHECKER, root / "scripts/check-ci-path-routing.py")
    shutil.copy2(TEST, root / "scripts/test-ci-path-routing.py")
    shutil.copy2(ROOT / "scripts/check-cross-targets.sh", root / "scripts/check-cross-targets.sh")
    return root


def mutate(path: pathlib.Path, old: str, new: str) -> None:
    text = path.read_text(encoding="utf-8")
    if old not in text:
        raise AssertionError(f"mutation anchor absent: {old!r}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def workflow_mutation_fails(workflow: str, old: str, new: str) -> None:
    root = workspace()
    try:
        mutate(root / ".github/workflows" / workflow, old, new)
        checker_fails(root)
    finally:
        shutil.rmtree(root)


def script_mutation_fails(script: str, old: str, new: str) -> None:
    root = workspace()
    try:
        mutate(root / "scripts" / script, old, new)
        checker_fails(root)
    finally:
        shutil.rmtree(root)


def router_mutation_fails(old: str, new: str) -> None:
    root = workspace()
    try:
        mutate(root / "scripts/ci-path-router.py", old, new)
        result = subprocess.run([sys.executable, str(root / "scripts/test-ci-path-routing.py")],
                                stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
                                check=False)
        if result.returncode == 0:
            raise AssertionError("router mutation survived the classifier regression suite")
    finally:
        shutil.rmtree(root)


def git(root: pathlib.Path, *args: str) -> str:
    return run("git", "-C", str(root), *args)


def commit_file(root: pathlib.Path, path: str, contents: str, message: str) -> str:
    destination = root / path
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(contents, encoding="utf-8")
    git(root, "add", path)
    git(root, "commit", "-m", message)
    return git(root, "rev-parse", "HEAD")


def test_nightly_budget_selection() -> None:
    spec = importlib.util.spec_from_file_location("ci_checker", CHECKER)
    checker = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(checker)
    body = checker.nightly_budget_script(ROOT)
    expected = [command.split()[1:] for command in checker.NIGHTLY_BUDGET_COMMANDS]
    with tempfile.TemporaryDirectory(prefix="nightly-budget-selection-") as directory:
        scratch = pathlib.Path(directory)
        cargo = scratch / "cargo"
        cargo.write_text(
            f"#!{sys.executable}\n"
            "import json, os, pathlib, sys\n"
            "p = pathlib.Path(os.environ['BUDGET_CALL_LOG'])\n"
            "rows = p.read_text().splitlines() if p.exists() else []\n"
            "with p.open('a') as output: output.write(json.dumps(sys.argv[1:]) + '\\n')\n"
            "sys.exit(37 if len(rows) + 1 == int(os.environ['FAIL_CALL']) else 0)\n"
        )
        cargo.chmod(0o755)
        for fail_call in range(4):
            log = scratch / f"calls-{fail_call}.jsonl"
            env = dict(os.environ, PATH=f"{scratch}:{os.environ['PATH']}",
                       BUDGET_CALL_LOG=str(log), FAIL_CALL=str(fail_call))
            result = subprocess.run(["bash", "-c", body], env=env, check=False)
            calls = [json.loads(line) for line in log.read_text().splitlines()]
            assert result.returncode == (37 if fail_call else 0)
            assert calls == expected[:fail_call or 3]
    for command in checker.NIGHTLY_BUDGET_COMMANDS:
        workflow_mutation_fails("nightly.yml", command, "true")
    workflow_mutation_fails("nightly.yml", "--ignored --exact", "--ignored")
    workflow_mutation_fails("nightly.yml", checker.NIGHTLY_BUDGET_COMMANDS[0],
                            "cargo test --locked --release -p host-web -p host-core -- --ignored")
    workflow_mutation_fails("nightly.yml", "          set -euo pipefail\n" +
                            "          " + checker.NIGHTLY_BUDGET_COMMANDS[0],
                            "          " + checker.NIGHTLY_BUDGET_COMMANDS[0])


def main() -> int:
    test_nightly_budget_selection()
    test_new_router_behaviours()
    print("new dsp-research/SDK-script/--flags mutation cases passed")

    assert route("sdk/src/index.ts") == "sdk"
    for owned in (
        "scripts/check-sdk-deletions.py",
        "scripts/check-sdk-generated.sh",
        "scripts/check-sdk-headless.sh",
        "scripts/check-sdk-types.sh",
        "scripts/sdk-package.sh",
    ):
        assert route(owned) == "sdk"
    assert route("docs/routing.md", "README.md") == "evidence"
    assert route("sdk/src/index.ts", "docs/routing.md") == "sdk"
    assert route("LICENSE") == "full"
    assert route("LICENSE", "docs/routing.md") == "full"
    assert route("LICENSE", "sdk/src/index.ts") == "full"
    assert route("LICENSE", "docs/routing.md", "sdk/src/index.ts") == "full"
    assert route("crates/engine/src/lib.rs") == "full"  # engine path cannot become SDK-only
    assert route("sdk/src/index.ts", "crates/engine/src/lib.rs") == "full"
    for full_input in (
        "Cargo.toml",
        "rust-toolchain.toml",
        "hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256",
        ".github/workflows/qualification.yml",
        "scripts/ci-path-router.py",
        "LICENSE",
        "future/unclassified-input",
    ):
        assert route(full_input) == "full"
    assert route() == "full"
    assert run(sys.executable, str(ROUTER), "--event", "workflow_dispatch") == "full"
    assert run(sys.executable, str(ROUTER), "--event", "pull_request",
               "--base", "definitely-not-a-revision", "--head", "HEAD") == "full"

    with tempfile.NamedTemporaryFile() as status:
        status.write(b"R100\0sdk/src/index.ts\0crates/engine/src/lib.rs\0")
        status.flush()
        assert run(sys.executable, str(ROUTER), "--event", "pull_request",
                   "--name-status-file", status.name) == "full"
    with tempfile.NamedTemporaryFile() as status:
        status.write(b"C100\0sdk/src/index.ts\0crates/engine/src/lib.rs\0")
        status.flush()
        assert run(sys.executable, str(ROUTER), "--event", "pull_request",
                   "--name-status-file", status.name) == "full"
    for ordinary in b"ADMT":
        with tempfile.NamedTemporaryFile() as status:
            status.write(bytes([ordinary]) + b"\0LICENSE\0")
            status.flush()
            assert run(sys.executable, str(ROUTER), "--event", "pull_request",
                       "--name-status-file", status.name) == "full"
    for operation in (b"R100", b"C075"):
        for old, new in ((b"LICENSE", b"sdk/new-license"),
                         (b"sdk/old-license", b"LICENSE")):
            with tempfile.NamedTemporaryFile() as status:
                status.write(operation + b"\0" + old + b"\0" + new + b"\0")
                status.flush()
                assert run(sys.executable, str(ROUTER), "--event", "pull_request",
                           "--name-status-file", status.name) == "full"
    for record in (
        b"U\0sdk/src/index.ts\0",
        b"X\0sdk/src/index.ts\0",
        b"B\0sdk/src/index.ts\0",
        b"M100\0sdk/src/index.ts\0",
        b"R\0sdk/old.ts\0sdk/new.ts\0",
        b"R10\0sdk/old.ts\0sdk/new.ts\0",
        b"R101\0sdk/old.ts\0sdk/new.ts\0",
    ):
        with tempfile.NamedTemporaryFile() as status:
            status.write(record)
            status.flush()
            assert run(sys.executable, str(ROUTER), "--event", "pull_request",
                       "--name-status-file", status.name) == "full"
    for record in (
        b"A\0sdk/src/index.ts\0",
        b"D\0sdk/src/index.ts\0",
        b"M\0sdk/src/index.ts\0",
        b"T\0sdk/src/index.ts\0",
        b"R100\0sdk/old.ts\0sdk/new.ts\0",
        b"C075\0sdk/old.ts\0sdk/new.ts\0",
    ):
        with tempfile.NamedTemporaryFile() as status:
            status.write(record)
            status.flush()
            assert run(sys.executable, str(ROUTER), "--event", "pull_request",
                       "--name-status-file", status.name) == "sdk"

    # A feature branch made from the common ancestor is SDK-only even when main later diverges
    # with an engine commit. GitHub's PR file set is a three-dot diff; push routing remains the
    # two-dot before/after transition and therefore sees both sides of this artificial divergence.
    repository = pathlib.Path(tempfile.mkdtemp(prefix="ci-path-routing-git-"))
    try:
        git(repository, "init", "-q", "-b", "main")
        git(repository, "config", "user.name", "CI path routing test")
        git(repository, "config", "user.email", "ci-path-routing@example.invalid")
        ancestor = commit_file(repository, "README.md", "base\n", "base")
        git(repository, "checkout", "-q", "-b", "feature", ancestor)
        head = commit_file(repository, "sdk/src/index.ts", "export {};\n", "sdk")
        git(repository, "checkout", "-q", "main")
        base = commit_file(repository, "crates/engine/src/lib.rs", "// diverged\n", "engine")
        assert run_at(repository, sys.executable, str(ROUTER), "--event", "pull_request",
                      "--base", base, "--head", head) == "sdk"
        assert run_at(repository, sys.executable, str(ROUTER), "--event", "push",
                      "--base", base, "--head", head) == "full"
    finally:
        shutil.rmtree(repository)

    # Copy discovery must include an unchanged source. Root LICENSE exists at the common ancestor;
    # the feature adds identical bytes only beneath sdk/, while main diverges independently. Plain
    # --find-copies reports A and would narrow both the real PR and linear push to SDK. Production's
    # --find-copies-harder reports both LICENSE and the SDK destination, preserving the full route.
    repository = pathlib.Path(tempfile.mkdtemp(prefix="ci-license-copy-git-"))
    try:
        git(repository, "init", "-q", "-b", "main")
        git(repository, "config", "user.name", "CI path routing test")
        git(repository, "config", "user.email", "ci-path-routing@example.invalid")
        ancestor = commit_file(repository, "LICENSE", "shared license fixture\n", "license")
        git(repository, "checkout", "-q", "-b", "feature", ancestor)
        destination = repository / "sdk/LICENSE-copy"
        destination.parent.mkdir(parents=True)
        shutil.copy2(repository / "LICENSE", destination)
        git(repository, "add", "sdk/LICENSE-copy")
        git(repository, "commit", "-m", "copy license into sdk")
        head = git(repository, "rev-parse", "HEAD")
        hard_copy = run_bytes_at(
            repository, "git", "diff", "--name-status", "-z", "--find-renames",
            "--find-copies-harder", f"{ancestor}...{head}",
        )
        assert hard_copy == b"C100\0LICENSE\0sdk/LICENSE-copy\0"
        ordinary_copy = run_bytes_at(
            repository, "git", "diff", "--name-status", "-z", "--find-renames",
            "--find-copies", f"{ancestor}...{head}",
        )
        assert ordinary_copy == b"A\0sdk/LICENSE-copy\0"
        assert run_at(repository, sys.executable, str(ROUTER), "--event", "push",
                      "--base", ancestor, "--head", head) == "full"
        git(repository, "checkout", "-q", "main")
        base = commit_file(repository, "README.md", "main diverged\n", "diverge main")
        assert run_at(repository, sys.executable, str(ROUTER), "--event", "pull_request",
                      "--base", base, "--head", head) == "full"
    finally:
        shutil.rmtree(repository)

    root = workspace()
    try:
        mutate(root / "scripts/ci-path-router.py", "    return None\n\n\ndef classify_paths",
               "    return \"sdk\"\n\n\ndef classify_paths")
        checker_fails(root)  # a dangerous engine/unknown-as-SDK fallback is rejected
    finally:
        shutil.rmtree(root)
    router_mutation_fails(
        '    "scripts/sdk-package.sh",\n',
        '    "scripts/sdk-package.sh",\n    "LICENSE",\n',
    )
    root = workspace()
    try:
        mutate(root / "scripts/ci-path-router.py", '    "scripts/sdk-package.sh",\n',
               '    "scripts/sdk-package.sh",\n    "LICENSE",\n')
        checker_fails(root)  # the static taxonomy contract independently rejects reintroduction
    finally:
        shutil.rmtree(root)
    router_mutation_fails(
        "        for value in fields[index:index + count]:\n",
        "        for value in fields[index + count - 1:index + count]:\n",
    )  # dropping a LICENSE rename/copy source would narrow to SDK
    router_mutation_fails(
        "        for value in fields[index:index + count]:\n",
        "        for value in fields[index:index + 1]:\n",
    )  # dropping a LICENSE rename/copy destination would narrow to SDK
    router_mutation_fails('"--find-copies-harder"', '"--find-copies"')
    root = workspace()
    try:
        mutate(root / "scripts/ci-path-router.py", '"--find-copies-harder"', '"--find-copies"')
        checker_fails(root)  # static policy also rejects unchanged-source copy discovery downgrade
    finally:
        shutil.rmtree(root)
    root = workspace()
    try:
        mutate(root / "scripts/ci-path-router.py",
               '["git", "diff", *GIT_DIFF_OPTIONS, *revisions]',
               '["git", "diff", "--name-status", "-z", "--find-renames", "--find-copies", *revisions]')
        checker_fails(root)  # leaving a correct unused constant cannot disguise production downgrade
    finally:
        shutil.rmtree(root)

    # R3 (WP-1 review B3/N21): DSP_RESEARCH_PREFIX, MATH_CLOSURE_*, and RELEASE_INPUT_FILES are
    # AST-pinned by check-ci-path-routing.py the same way SDK_FILES and GIT_DIFF_OPTIONS are, so
    # router/checker drift on any of them is caught by the policy checker rather than only by
    # this file's own direct assertions above.
    root = workspace()
    try:
        mutate(root / "scripts/ci-path-router.py",
               'DSP_RESEARCH_PREFIX = "dsp-research/"', 'DSP_RESEARCH_PREFIX = "dsp-notes/"')
        checker_fails(root)
    finally:
        shutil.rmtree(root)
    root = workspace()
    try:
        mutate(root / "scripts/ci-path-router.py",
               'MATH_CLOSURE_PREFIXES = ("crates/math/", "crates/lane/")',
               'MATH_CLOSURE_PREFIXES = ("crates/math/", "crates/lane/", "crates/engine/")')
        checker_fails(root)
    finally:
        shutil.rmtree(root)
    root = workspace()
    try:
        mutate(root / "scripts/ci-path-router.py",
               'MATH_CLOSURE_FILES = {"Cargo.lock", "Cargo.toml", "rust-toolchain.toml", ".cargo/config.toml"}',
               'MATH_CLOSURE_FILES = {"Cargo.lock", "Cargo.toml", "rust-toolchain.toml"}')
        checker_fails(root)
    finally:
        shutil.rmtree(root)
    root = workspace()
    try:
        mutate(root / "scripts/ci-path-router.py",
               'RELEASE_INPUT_SUFFIX = "/Cargo.toml"', 'RELEASE_INPUT_SUFFIX = "/Cargo.lock"')
        checker_fails(root)
    finally:
        shutil.rmtree(root)
    root = workspace()
    try:
        mutate(root / "scripts/ci-path-router.py",
               '    "scripts/check-release-shape.py",\n}',
               '    "scripts/check-release-shape.py",\n    "scripts/check-sdk-types.sh",\n}')
        checker_fails(root)
    finally:
        shutil.rmtree(root)

    with tempfile.NamedTemporaryFile() as status:
        status.write(b"R100\0sdk/src/index.ts\0")  # a missing rename side is malformed/full
        status.flush()
        assert run(sys.executable, str(ROUTER), "--event", "pull_request",
                   "--name-status-file", status.name) == "full"

    # Design #359 §12 stage 3 / check_retired_workflows_absent: qualification.yml is the sole
    # required PR workflow now. If ci.yml, sdk.yml, browser-qualification.yml or
    # release-build.yml reappears in .github/workflows/ -- even as an unrelated placeholder --
    # the checker must refuse, or a revert/merge accident could silently restore the old
    # multi-context topology without anyone noticing.
    for retired in ("ci.yml", "sdk.yml", "browser-qualification.yml", "release-build.yml"):
        root = workspace()
        try:
            (root / ".github/workflows" / retired).write_text(
                "name: placeholder\non:\n  push: {}\njobs: {}\n", encoding="utf-8",
            )
            checker_fails(root)  # a retired workflow's reappearance must fail the checker
        finally:
            shutil.rmtree(root)

    # qualification.yml (issue #359 WP-4 deliverable B): a path filter on any trigger, a relaxed
    # top-level permissions default, a verdict `needs:` that drifts from the job set, a missing
    # timeout, or an expectation table silently dropping a job must all fail the checker.
    root = workspace()
    try:
        mutate(root / ".github/workflows/qualification.yml",
               "  pull_request:\n    branches:\n      - main\n",
               "  pull_request:\n    branches:\n      - main\n    paths:\n      - 'sdk/**'\n")
        checker_fails(root)  # a path filter on the always-required workflow leaves it pending
    finally:
        shutil.rmtree(root)
    root = workspace()
    try:
        mutate(root / ".github/workflows/qualification.yml",
               "permissions:\n  contents: read\n",
               "permissions:\n  contents: read\n  actions: read\n")
        checker_fails(root)  # top-level permissions must stay exactly contents: read
    finally:
        shutil.rmtree(root)
    root = workspace()
    try:
        mutate(root / ".github/workflows/qualification.yml",
               "    needs: [route, docs-gates, artifact, artifact-identity, artifact-record, sdk, "
               "artifact-gates, browser, lint, test-debug-a, test-debug-b, test-release, "
               "audit-native, wasm-guests, cross-target, aarch64-debug, aarch64-release, "
               "release-shape]",
               "    needs: [route, docs-gates, artifact, artifact-identity, artifact-record, sdk, "
               "artifact-gates, browser, lint, test-debug-a, test-debug-b, test-release, "
               "audit-native, wasm-guests, cross-target, aarch64-debug, aarch64-release]")
        checker_fails(root)  # a job dropped from verdict's needs: escapes the aggregate entirely
    finally:
        shutil.rmtree(root)
    root = workspace()
    try:
        mutate(root / ".github/workflows/qualification.yml",
               "    timeout-minutes: 10\n    strategy:", "    strategy:")
        checker_fails(root)  # a leaf job without timeout-minutes can pend forever behind the gate
    finally:
        shutil.rmtree(root)
    root = workspace()
    try:
        mutate(root / ".github/workflows/qualification.yml",
               'check cross-target "$CROSS_TARGET_RESULT" "$full_expected"\n', "")
        checker_fails(root)  # a job dropped from the expectation table can drift silently
    finally:
        shutil.rmtree(root)

    # S3/S4/N2: the verdict's `if: always()`, its exact `check <job> "$<JOB>_RESULT"` pairing per
    # expectation-table row, the release_shape guard, and its own permissions/cancellation must
    # all be caught by the checker rather than only by the verdict's own runtime bash (design §7).
    workflow_mutation_fails(
        "qualification.yml", "    name: qualification\n    if: always()\n",
        "    name: qualification\n",
    )  # deleting if: always() would let the required context resolve to skipped forever
    workflow_mutation_fails(
        "qualification.yml",
        'check sdk "$SDK_RESULT" "$artifact_expected"\n',
        'check sdk "$LINT_RESULT" "$artifact_expected"\n',
    )  # a name grep alone would miss the expectation table checking the wrong job's result
    workflow_mutation_fails(
        "qualification.yml",
        '[[ "$RELEASE_INPUTS" == "true" ]] && release_shape_expected=success\n',
        "release_shape_expected=success\n",
    )  # release-shape must not be expected to run on a full route without release inputs
    workflow_mutation_fails(
        "qualification.yml",
        "    permissions:\n      actions: read\n      contents: read\n",
        "    permissions:\n      contents: read\n",
    )  # the verdict's permissions must stay exactly actions: read plus contents: read
    workflow_mutation_fails(
        "qualification.yml",
        "  cancel-in-progress: ${{ github.event_name == 'pull_request' }}\n",
        "",
    )  # dropping the pull_request-only cancellation lets a main push starve its own cache saves
    workflow_mutation_fails(
        "qualification.yml",
        "  cancel-in-progress: ${{ github.event_name == 'pull_request' }}\n",
        "  cancel-in-progress: true\n",
    )  # reverting to unconditional cancellation is exactly the regression S9 forbids

    # Stage-3 review S1/S2: the route job's self-validation step, its ordering, fetch-depth, the
    # SDK closure and the canonical workspace-policy step are pinned on the surviving workflow.
    workflow_mutation_fails(
        "qualification.yml",
        "      - name: Validate path-routing policy and mutations\n"
        "        run: |\n"
        "          python3 -B scripts/check-ci-path-routing.py\n"
        "          python3 -B scripts/test-ci-path-routing.py\n",
        "",
    )
    workflow_mutation_fails("qualification.yml", "          fetch-depth: 0\n", "          fetch-depth: 1\n")
    workflow_mutation_fails(
        "qualification.yml",
        "          bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts\n",
        "",
    )
    workflow_mutation_fails(
        "qualification.yml",
        "          bash scripts/sdk-package.sh check target/ci/qualification-artifacts\n",
        "",
    )
    workflow_mutation_fails(
        "qualification.yml",
        "        run: bash scripts/check-workspace-policy.sh\n",
        "        run: true\n",
    )
    # Issue #1021: the test-support coverage guard and its mutation tests stay on the lint job.
    spec = importlib.util.spec_from_file_location("ci_checker", CHECKER)
    checker = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(checker)
    for line in checker.TEST_SUPPORT_CI_LINES:
        workflow_mutation_fails("qualification.yml", "          " + line, "")
    # Issue #1009: wasm-guests leaves the V8 spill leg to artifact-gates, so artifact-gates must run
    # it on the downloaded artifact, after the digest check.
    workflow_mutation_fails(
        "qualification.yml",
        "          python3 -B scripts/check-web-audioworklet-v8-spill.py "
        "target/ci/qualification-artifacts/miso-engine-v1-audio-worklet.simd128.wasm\n",
        "",
    )
    workflow_mutation_fails(
        "qualification.yml",
        "          shared-key: artifact-gates\n",
        "          shared-key: artifact-gates\n"
        "      - name: V8 spill gate before the digest check\n"
        "        run: python3 -B scripts/check-web-audioworklet-v8-spill.py "
        "target/ci/qualification-artifacts/miso-engine-v1-audio-worklet.simd128.wasm\n",
    )

    # Issue #1061: every reader of the shipped module checks its download against the digest the
    # artifact job published, and the identity job reports ARTIFACT CHANGED or UNCHANGED, proves
    # the build reproducible and holds a release change to its pin, on the artifact job's routes.
    digest_step = checker.ARTIFACT_DIGEST_STEP
    workflow_mutation_fails("qualification.yml", checker.ARTIFACT_DIGEST_OUTPUT, "")
    for reader in checker.ARTIFACT_READERS:
        root = workspace()
        try:
            workflow = root / ".github/workflows/qualification.yml"
            text = workflow.read_text(encoding="utf-8")
            start = text.index(f"\n  {reader}:\n")
            step = text.index(digest_step, start)
            end = text.index("      - ", step + len(digest_step))
            workflow.write_text(text[:step] + text[end:], encoding="utf-8")
            checker_fails(root)  # a reader that stops verifying its download
        finally:
            shutil.rmtree(root)
    root = workspace()
    try:
        # The pre-#1061 step: the download held to the committed pin instead of the built digest.
        workflow = root / ".github/workflows/qualification.yml"
        text = workflow.read_text(encoding="utf-8")
        start = text.index("\n  browser:\n")
        step = text.index(digest_step, start)
        workflow.write_text(text[:step] + text[step:].replace(
            digest_step,
            "      - name: Verify the downloaded artifact against its source pin\n"
            "        env:\n"
            "          BUILT: hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256\n",
            1), encoding="utf-8")
        checker_fails(root)
    finally:
        shutil.rmtree(root)
    for step in checker.IDENTITY_STEPS:
        workflow_mutation_fails("qualification.yml", step, "")  # a pinned step deleted
    # Issue #1061 attempt 3: the record on main and the fetch-and-compare on every change. Each
    # mutant would leave the comparison without a record, skipped, or masked, with CI green.
    record_step = checker.ARTIFACT_RECORD_STEP
    workflow_mutation_fails("qualification.yml", record_step, "")  # main records nothing
    workflow_mutation_fails(
        "qualification.yml", checker.RECORD_HEAD,
        checker.RECORD_HEAD.replace("github.event_name == 'push' && ", ""),
    )  # the record job reachable by pull requests
    workflow_mutation_fails(
        "qualification.yml", checker.RECORD_HEAD,
        checker.RECORD_HEAD.replace(" && github.ref == 'refs/heads/main'", ""),
    )  # the record job run on any push
    workflow_mutation_fails(
        "qualification.yml", checker.RECORD_PERMISSIONS, "    permissions:\n      contents: read\n",
    )  # the post is refused on main
    workflow_mutation_fails(
        "qualification.yml", checker.RECORD_PERMISSIONS + "    steps:\n",
        checker.RECORD_PERMISSIONS + "    steps:\n      - uses: actions/checkout@"
        "11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\n",
    )  # the write-token job checks out, and so runs, repository code
    # #1061 attempt 3 verdict, finding 1: a write token on any job a pull request reaches lets a
    # same-repository change forge its base's record. Each grant is red.
    for anchor, grant in (
        ("    timeout-minutes: 15\n    outputs:\n      sha256:",
         "    timeout-minutes: 15\n    permissions:\n      contents: read\n      statuses: write\n"
         "    outputs:\n      sha256:"),  # artifact, which runs on every pull request
        (checker.IDENTITY_PERMISSIONS,
         "    permissions:\n      contents: read\n      statuses: write\n"),  # artifact-identity
        ("    timeout-minutes: 20\n    permissions:\n      contents: read\n      statuses: read\n",
         "    timeout-minutes: 20\n    permissions: write-all\n"),
    ):
        workflow_mutation_fails("qualification.yml", anchor, grant)
    for line in checker.ARTIFACT_BUILD_LINES:
        workflow_mutation_fails("qualification.yml", line, "")
    workflow_mutation_fails("qualification.yml", "      rustc: ${{ steps.build.outputs.rustc }}\n", "")
    workflow_mutation_fails(
        "qualification.yml", checker.IDENTITY_PERMISSIONS,
        "    permissions:\n      contents: read\n",
    )  # the lookup is refused on every change
    identity_head = ("    runs-on: ubuntu-24.04\n    timeout-minutes: 20\n"
                     + checker.IDENTITY_PERMISSIONS)
    for label, extra in (
        ("continue-on-error", "    continue-on-error: true\n"),
        ("defaults", "    defaults:\n      run:\n        shell: bash {0}\n"),
        ("env", "    env:\n      GH_TOKEN: ''\n"),
    ):
        workflow_mutation_fails("qualification.yml", identity_head, identity_head + extra)
    workflow_mutation_fails(
        "qualification.yml", checker.RECORD_PERMISSIONS,
        checker.RECORD_PERMISSIONS + "    env:\n      PATH: /tmp/forged-gh:/usr/bin:/bin\n",
    )  # a job PATH that puts another gh first could forge main's record
    report_step = checker.IDENTITY_REPORT_STEP
    workflow_mutation_fails(
        "qualification.yml", report_step,
        report_step.replace("        env:\n", "        if: github.event_name == 'push'\n        env:\n", 1),
    )  # the comparison skipped on pull requests
    workflow_mutation_fails("qualification.yml", 'report --event "$EVENT"', "report --event none")
    workflow_mutation_fails("qualification.yml", "          BEFORE: ${{ github.event.before }}\n",
                            '          BEFORE: ""\n')
    workflow_mutation_fails(
        "qualification.yml", "          GH_TOKEN: ${{ github.token }}\n          EVENT:",
        "          EVENT:",
    )  # gh unauthenticated
    workflow_mutation_fails(
        "qualification.yml", 'python3 -B scripts/web-audioworklet-identity.py --self-test\n', "",
    )
    workflow_mutation_fails(
        "qualification.yml", report_step,
        "      - name: Remove the twin module\n        run: rm -rf \"$RUNNER_TEMP/twin-module\"\n"
        + report_step,
    )  # a step slipped in before the report
    workflow_mutation_fails(
        "qualification.yml",
        "        with:\n          fetch-depth: 0\n"
        "      - name: Install pinned Rust toolchain and Wasm standard library\n",
        "      - name: Install pinned Rust toolchain and Wasm standard library\n",
    )  # the identity job's full history dropped: no base is reachable
    workflow_mutation_fails(
        "qualification.yml", 'git worktree add --detach "$RUNNER_TEMP/twin" HEAD\n',
        'ln -s "$GITHUB_WORKSPACE" "$RUNNER_TEMP/twin"\n',
    )  # a twin symlinked to the workspace rebuilds from the artifact job's own path
    workflow_mutation_fails(
        "qualification.yml",
        'CARGO_HOME="$RUNNER_TEMP/twin-cargo-home" bash "$RUNNER_TEMP/twin/',
        'bash "$RUNNER_TEMP/twin/',
    )  # a twin sharing the artifact job's CARGO_HOME proves nothing about it
    workflow_mutation_fails(
        "qualification.yml",
        "    name: shipped AudioWorklet artifact identity (changed or unchanged, reproducible, "
        "release pin)\n    needs: [route, artifact]\n"
        "    if: needs.route.outputs.route == 'sdk' || needs.route.outputs.route == 'full'\n",
        "    name: shipped AudioWorklet artifact identity (changed or unchanged, reproducible, "
        "release pin)\n    needs: [route, artifact]\n"
        "    if: needs.route.outputs.route == 'full'\n",
    )  # an sdk-route PR would build the module and say nothing about it
    workflow_mutation_fails(
        "qualification.yml",
        "    name: shipped AudioWorklet artifact identity (changed or unchanged, reproducible, "
        "release pin)\n    needs: [route, artifact]\n",
        "    name: shipped AudioWorklet artifact identity (changed or unchanged, reproducible, "
        "release pin)\n    needs: [route]\n",
    )
    workflow_mutation_fails(
        "qualification.yml",
        '          check artifact-identity "$ARTIFACT_IDENTITY_RESULT" "$artifact_expected"\n', "",
    )  # the identity job dropped from the expectation table
    workflow_mutation_fails(
        "qualification.yml",
        '          check artifact-record "$ARTIFACT_RECORD_RESULT" "$record_expected"\n', "",
    )  # the record job dropped from the expectation table

    # Issue #1017: the AArch64 compile rows and test jobs. Deleting a row, either half of the
    # row, a job's arm64 runner, its script, its full-route gating or its success expectation, or a
    # mobile target from cross-target's install, must each fail the checker.
    installed = "armv7-linux-androideabi aarch64-apple-ios aarch64-linux-android\n"
    for target in ("aarch64-apple-ios", "aarch64-linux-android"):
        script_mutation_fails("check-cross-targets.sh", f"aarch64_row {target}\n", "")
        script_mutation_fails("check-cross-targets.sh", f"aarch64_row {target}\n",
                              f"# aarch64_row {target}\n")
        workflow_mutation_fails("qualification.yml", installed,
                                installed.replace(f" {target}", ""))
    script_mutation_fails(
        "check-cross-targets.sh",
        '        "${product_packages[@]}" -- -D warnings\n',
        '        "${product_packages[@]}"\n',
    )
    script_mutation_fails(
        "check-cross-targets.sh",
        "        cargo clippy --quiet --locked --all-targets --all-features",
        "        cargo check --quiet --locked --all-targets --all-features",
    )
    script_mutation_fails(
        "check-cross-targets.sh",
        "        cargo check --quiet --locked --all-targets --all-features",
        "        cargo check --quiet --locked --all-features",
    )
    script_mutation_fails(
        "check-cross-targets.sh", "printf '%s\\n' \"$product_list\" >\"$asm_out/products\"\n",
        "printf 'parametric-eq\\nbuiltins\\n' >\"$asm_out/products\"\n",
    )  # the iOS memset judge handed a hand-picked subset
    script_mutation_fails(
        "check-cross-targets.sh",
        '"${known_defects[@]}" judge-memset "$asm_out/counts" "$asm_out/products" ||\n',
        'true ||\n',
    )  # the per-crate judge dropped
    for name, mode in (("aarch64-debug", "debug"), ("aarch64-release", "release")):
        variable = name.replace("-", "_").upper() + "_RESULT"
        workflow_mutation_fails(
            "qualification.yml", f"        run: bash scripts/run-aarch64-tests.sh {mode}\n",
            "        run: true\n",
        )
        workflow_mutation_fails(
            "qualification.yml",
            f'check {name} "${variable}" "$full_expected"',
            f'check {name} "${variable}" "$artifact_expected"',
        )
    workflow_mutation_fails(
        "qualification.yml",
        "    runs-on: ubuntu-24.04-arm\n    timeout-minutes: 30\n    env:\n"
        "      RUSTFLAGS: -D warnings\n    steps:\n      - uses: actions/checkout@",
        "    runs-on: ubuntu-24.04\n    timeout-minutes: 30\n    env:\n"
        "      RUSTFLAGS: -D warnings\n    steps:\n      - uses: actions/checkout@",
    )
    workflow_mutation_fails(
        "qualification.yml",
        "  aarch64-release:\n    name: AArch64 release digest gates and realtime audits "
        "(NEON Simd4, FPCR)\n    needs: route\n    if: needs.route.outputs.route == 'full'\n",
        "  aarch64-release:\n    name: AArch64 release digest gates and realtime audits "
        "(NEON Simd4, FPCR)\n    needs: route\n    if: needs.route.outputs.route == 'sdk'\n",
    )

    # #1048 review finding 2: the V8 pairing is found by token, so reordering the flags cannot
    # slip the spill leg out of CI once artifact-gates stops running it.
    root = workspace()
    try:
        workflow = root / ".github/workflows/qualification.yml"
        mutate(workflow, "bash scripts/run-wasm-gates.sh --without-v8-spill --without-native\n",
               "bash scripts/run-wasm-gates.sh --without-native --without-v8-spill\n")
        mutate(workflow,
               "          python3 -B scripts/check-web-audioworklet-v8-spill.py "
               "target/ci/qualification-artifacts/miso-engine-v1-audio-worklet.simd128.wasm\n",
               "")
        checker_fails(root)
    finally:
        shutil.rmtree(root)

    # Issue #1048: wasm-guests leaves run-wasm-gates.sh's native leg to test-release's G5 Rust
    # test, so test-release must run wasm-gates' tests in release, unfiltered, unconditionally and
    # on the same route.
    g5_step = ("        run: cargo test --locked --release -p lane -p math -p wasm-gates "
               "--features math/lane\n")
    workflow_mutation_fails(
        "qualification.yml", g5_step,
        "        run: cargo test --locked --release -p lane -p math --features math/lane\n",
    )  # the owner dropped from test-release
    workflow_mutation_fails(
        "qualification.yml", g5_step,
        "        run: cargo test --locked -p lane -p math -p wasm-gates --features math/lane\n",
    )  # the owner demoted to the debug profile
    workflow_mutation_fails(
        "qualification.yml", g5_step,
        g5_step.replace("--features math/lane\n",
                        "--features math/lane -- --skip g5_native_digests_match_pins\n"),
    )  # the owner filtered out by name
    workflow_mutation_fails(
        "qualification.yml", g5_step,
        g5_step.replace("-p wasm-gates", "-p wasm-gates --lib"),
    )  # the owner's integration tests deselected
    workflow_mutation_fails(
        "qualification.yml", g5_step,
        "        if: needs.route.outputs.math_closure == 'true'\n" + g5_step,
    )  # the owner made conditional on a narrower step condition
    workflow_mutation_fails(
        "qualification.yml",
        "    name: release-mode lane, math, and wasm-gates digest gates\n"
        "    needs: route\n"
        "    if: needs.route.outputs.route == 'full'\n",
        "    name: release-mode lane, math, and wasm-gates digest gates\n"
        "    needs: route\n"
        "    if: needs.route.outputs.math_closure == 'true'\n",
    )  # the owner's job on a narrower route than wasm-guests

    # Issue #1048 amendment 2: an AArch64 job (#1017) must carry the G5 owner in release.
    # `aarch64-release` does, in its own step; dropping, filtering, demoting or conditioning that
    # step fails, and the owner's own job moved onto an arm64 runner passes.
    aarch64_g5 = ("      - name: G5 class-A corpus on arm64 (the native owner, #1048)\n"
                  "        run: cargo test --locked --release -p wasm-gates --features math/lane\n")
    workflow_mutation_fails("qualification.yml", aarch64_g5, "")
    workflow_mutation_fails(
        "qualification.yml", aarch64_g5,
        aarch64_g5.replace("--features math/lane\n",
                           "--features math/lane -- --skip g5_native_digests_match_pins\n"),
    )
    workflow_mutation_fails("qualification.yml", aarch64_g5,
                            aarch64_g5.replace(" --release", ""))
    workflow_mutation_fails(
        "qualification.yml", aarch64_g5,
        aarch64_g5.replace("        run:",
                           "        if: needs.route.outputs.math_closure == 'true'\n        run:"),
    )
    root = workspace()
    try:
        mutate(root / ".github/workflows/qualification.yml",
               "    name: release-mode lane, math, and wasm-gates digest gates\n    needs: route\n"
               "    if: needs.route.outputs.route == 'full'\n    runs-on: ubuntu-24.04\n",
               "    name: release-mode lane, math, and wasm-gates digest gates\n    needs: route\n"
               "    if: needs.route.outputs.route == 'full'\n    runs-on: macos-15\n")
        run(sys.executable, str(CHECKER), "--root", str(root))
    finally:
        shutil.rmtree(root)

    # Baseline: the unmutated workspace() -- qualification.yml plus the router/checker/test
    # scripts, with none of the four retired workflows present -- must pass the checker outright.
    # Every workflow_mutation_fails/checker_fails call above depends on this holding; if it ever
    # regresses, every mutation test upstream would report false positives instead of the real
    # regression they intend to catch.
    root = workspace()
    try:
        run(sys.executable, str(CHECKER), "--root", str(root))
    finally:
        shutil.rmtree(root)

    print("ci path-routing classifier and mutation tests passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
