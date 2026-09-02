#!/usr/bin/env python3
"""Hermetic mutation tests for issue #328's classifier and workflow contract."""
from __future__ import annotations

import pathlib
import shutil
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(__file__).resolve().parent.parent
ROUTER = ROOT / "scripts/ci-path-router.py"
CHECKER = ROOT / "scripts/check-ci-path-routing.py"


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


def route_with(router: pathlib.Path, *paths: str) -> str:
    args = [sys.executable, str(router), "--event", "pull_request"]
    for path in paths:
        args += ["--path", path]
    return run(*args)


def route(*paths: str) -> str:
    return route_with(ROUTER, *paths)


def checker_fails(root: pathlib.Path) -> None:
    result = subprocess.run([sys.executable, str(CHECKER), "--root", str(root)],
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, check=False)
    if result.returncode == 0:
        raise AssertionError("workflow mutation was accepted")


def workspace() -> pathlib.Path:
    root = pathlib.Path(tempfile.mkdtemp(prefix="ci-path-routing-"))
    (root / ".github/workflows").mkdir(parents=True)
    (root / "scripts").mkdir()
    for name in ("ci.yml", "browser-qualification.yml", "release-build.yml", "sdk.yml"):
        shutil.copy2(ROOT / ".github/workflows" / name, root / ".github/workflows" / name)
    shutil.copy2(ROUTER, root / "scripts/ci-path-router.py")
    return root


def mutate(path: pathlib.Path, old: str, new: str) -> None:
    text = path.read_text(encoding="utf-8")
    if old not in text:
        raise AssertionError(f"mutation anchor absent: {old!r}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def git(root: pathlib.Path, *args: str) -> str:
    return run("git", "-C", str(root), *args)


def commit_file(root: pathlib.Path, path: str, contents: str, message: str) -> str:
    destination = root / path
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(contents, encoding="utf-8")
    git(root, "add", path)
    git(root, "commit", "-m", message)
    return git(root, "rev-parse", "HEAD")


def main() -> int:
    assert route("sdk/src/index.ts") == "sdk"
    for owned in (
        "scripts/check-sdk-deletions.py",
        "scripts/check-sdk-generated.sh",
        "scripts/check-sdk-headless.sh",
        "scripts/check-sdk-types.sh",
        "scripts/sdk-package.sh",
        "LICENSE",
    ):
        assert route(owned) == "sdk"
    assert route("docs/routing.md", "README.md") == "evidence"
    assert route("sdk/src/index.ts", "docs/routing.md") == "sdk"
    assert route("crates/engine/src/lib.rs") == "full"  # engine path cannot become SDK-only
    assert route("sdk/src/index.ts", "crates/engine/src/lib.rs") == "full"
    for full_input in (
        "Cargo.toml",
        "rust-toolchain.toml",
        "hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256",
        ".github/workflows/ci.yml",
        "scripts/ci-path-router.py",
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

    root = workspace()
    try:
        mutate(root / "scripts/ci-path-router.py", "    return None\n\n\ndef classify_paths",
               "    return \"sdk\"\n\n\ndef classify_paths")
        checker_fails(root)  # a dangerous engine/unknown-as-SDK fallback is rejected
    finally:
        shutil.rmtree(root)
    with tempfile.NamedTemporaryFile() as status:
        status.write(b"R100\0sdk/src/index.ts\0")  # a missing rename side is malformed/full
        status.flush()
        assert run(sys.executable, str(ROUTER), "--event", "pull_request",
                   "--name-status-file", status.name) == "full"

    root = workspace()
    try:
        run(sys.executable, str(CHECKER), "--root", str(root))
        mutate(root / ".github/workflows/sdk.yml",
               "          python3 -B scripts/test-ci-path-routing.py\n", "")
        checker_fails(root)  # an SDK-only route cannot consume unchecked workflow/router code
    finally:
        shutil.rmtree(root)
    root = workspace()
    try:
        mutate(root / ".github/workflows/ci.yml",
               "      - name: Validate path-routing policy and mutations\n        run: |\n"
               "          python3 -B scripts/check-ci-path-routing.py\n"
               "          python3 -B scripts/test-ci-path-routing.py\n"
               "      - id: classify",
               "      - id: classify")
        checker_fails(root)  # every independent route owns its cheap pre-classification policy
    finally:
        shutil.rmtree(root)
    root = workspace()
    try:
        mutate(root / ".github/workflows/ci.yml", "needs: [route, host, x86-probes, wasm, wasm-gates]",
               "needs: [route, host, x86-probes, wasm]")
        checker_fails(root)  # selected heavy job escaped aggregate dependencies
    finally:
        shutil.rmtree(root)
    root = workspace()
    try:
        mutate(root / ".github/workflows/sdk.yml", "    if: always()", "    if: success()")
        checker_fails(root)  # a required aggregate became conditional
    finally:
        shutil.rmtree(root)
    root = workspace()
    try:
        mutate(root / ".github/workflows/sdk.yml", "    if: always()",
               "    if: always()\n    continue-on-error: true")
        checker_fails(root)  # aggregate job failure suppression is forbidden
    finally:
        shutil.rmtree(root)
    root = workspace()
    try:
        mutate(root / ".github/workflows/sdk.yml", "      - name: Enforce SDK qualification route",
               "      - name: Enforce SDK qualification route\n        continue-on-error: true")
        checker_fails(root)  # enforcement-step failure suppression is forbidden
    finally:
        shutil.rmtree(root)
    root = workspace()
    try:
        mutate(root / ".github/workflows/browser-qualification.yml",
               '[[ "$ROUTE_RESULT" == success ]]', '[[ "$ROUTE_RESULT" == skipped ]]')
        checker_fails(root)  # a failed router must make its always-scheduled aggregate fail
    finally:
        shutil.rmtree(root)
    root = workspace()
    try:
        mutate(root / ".github/workflows/ci.yml",
               'for result in "$HOST_RESULT" "$X86_PROBES_RESULT" "$WASM_RESULT" "$WASM_GATES_RESULT"; do',
               'for result in "$X86_PROBES_RESULT" "$WASM_RESULT" "$WASM_GATES_RESULT"; do')
        checker_fails(root)  # a selected heavy failure cannot escape the aggregate evaluator
    finally:
        shutil.rmtree(root)
    root = workspace()
    try:
        mutate(root / ".github/workflows/browser-qualification.yml", "    branches:\n      - main\n  workflow_dispatch:",
               "    branches:\n      - main\n    paths:\n      - 'sdk/**'\n  workflow_dispatch:")
        checker_fails(root)  # path-filtered required PR workflow would leave its context pending
    finally:
        shutil.rmtree(root)

    print("ci path-routing classifier and mutation tests passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
