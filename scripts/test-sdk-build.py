#!/usr/bin/env python3
"""Hermetic contract test for the Phase-0 SDK build wrapper."""

from __future__ import annotations

import argparse
import os
from pathlib import Path
import shutil
import stat
import subprocess
import tempfile


REPOSITORY_ROOT = Path(__file__).resolve().parent.parent
SCRIPTS = (
    "build-sdk.sh",
    "write-web-provenance-v1.py",
    "check-web-provenance-v1.py",
)
ARTIFACT_NAMES = (
    "miso-engine-v2-abi-layout.json",
    "miso-engine-v2-audio-worklet-host.d.ts",
    "miso-engine-v2-audio-worklet-host.js",
    "miso-engine-v2-audio-worklet.js",
    "miso-engine-v2-audio-worklet.simd128.wasm",
    "miso-engine-v2-parameter-metadata.json",
)
PROVENANCE_NAME = "miso-engine-v2.provenance.json"


def run(
    *arguments: str | os.PathLike[str],
    cwd: Path,
    environment: dict[str, str] | None = None,
    expected_returncode: int = 0,
) -> subprocess.CompletedProcess[str]:
    completed = subprocess.run(
        [str(argument) for argument in arguments],
        cwd=cwd,
        env=environment,
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if completed.returncode != expected_returncode:
        raise AssertionError(
            f"expected exit {expected_returncode}, got {completed.returncode}: "
            f"{' '.join(str(argument) for argument in arguments)}\n"
            f"stdout:\n{completed.stdout}\nstderr:\n{completed.stderr}"
        )
    return completed


def fake_builder() -> str:
    writes = "\n".join(
        f"printf %s {index!r} > \"$output_dir/{name}\""
        for index, name in enumerate(ARTIFACT_NAMES, start=1)
    )
    return f"""#!/usr/bin/env bash
set -euo pipefail
output_dir=$1
printf '%s\\n' invoked >> "$FAKE_BUILD_MARKER"
{writes}
"""


def invocation_count(marker: Path) -> int:
    if not marker.exists():
        return 0
    return len(marker.read_text(encoding="utf-8").splitlines())


def make_repository(root: Path) -> tuple[Path, Path, dict[str, str]]:
    repository = root / "repository"
    scripts = repository / "scripts"
    scripts.mkdir(parents=True)
    for name in SCRIPTS:
        shutil.copy2(REPOSITORY_ROOT / "scripts" / name, scripts / name)
    builder = scripts / "build-web-audioworklet.sh"
    builder.write_text(fake_builder(), encoding="utf-8")
    builder.chmod(0o755)

    run("git", "init", "-q", repository, cwd=root)
    run("git", "-C", repository, "config", "user.email", "sdk-test@example.invalid", cwd=root)
    run("git", "-C", repository, "config", "user.name", "SDK build test", cwd=root)
    run("git", "-C", repository, "add", "scripts", cwd=root)
    run("git", "-C", repository, "commit", "-qm", "SDK wrapper fixture", cwd=root)

    marker = root / "fake-build-count"
    environment = os.environ.copy()
    environment["FAKE_BUILD_MARKER"] = str(marker)
    return repository, marker, environment


def expect_direct_exec_refusal(wrapper: Path, repository: Path, output: Path) -> None:
    original_mode = stat.S_IMODE(wrapper.stat().st_mode)
    if not original_mode & stat.S_IXUSR:
        raise AssertionError("build-sdk.sh must be owner-executable for direct execution")
    wrapper.chmod(original_mode & ~(stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH))
    try:
        try:
            run(wrapper, output, cwd=repository)
        except PermissionError:
            pass
        else:
            raise AssertionError("direct execution unexpectedly accepted a non-executable wrapper")
    finally:
        wrapper.chmod(original_mode)


def self_test() -> None:
    with tempfile.TemporaryDirectory(prefix="miso-sdk-build-") as temporary:
        root = Path(temporary)
        repository, marker, environment = make_repository(root)
        wrapper = repository / "scripts" / "build-sdk.sh"
        checker = repository / "scripts" / "check-web-provenance-v1.py"

        expect_direct_exec_refusal(wrapper, repository, root / "not-created")

        nonempty = root / "nonempty"
        nonempty.mkdir()
        (nonempty / "already-there").write_text("x", encoding="utf-8")
        run(wrapper, nonempty, cwd=repository, environment=environment, expected_returncode=2)
        if invocation_count(marker) != 0:
            raise AssertionError("nonempty output refusal reached the web builder")

        dirty_output = root / "dirty-output"
        dirty_output.mkdir()
        (repository / "untracked").write_text("x", encoding="utf-8")
        run(wrapper, dirty_output, cwd=repository, environment=environment, expected_returncode=2)
        if invocation_count(marker) != 0:
            raise AssertionError("dirty-tree refusal reached the web builder")
        (repository / "untracked").unlink()

        output = root / "output"
        output.mkdir()
        run(wrapper, output, cwd=repository, environment=environment)
        expected_outputs = set(ARTIFACT_NAMES) | {PROVENANCE_NAME}
        actual_outputs = {path.name for path in output.iterdir()}
        if actual_outputs != expected_outputs:
            raise AssertionError(f"SDK output membership differs: {actual_outputs!r}")
        run("python3", "-I", "-B", checker, output, cwd=repository, environment=environment)
        if invocation_count(marker) != 1:
            raise AssertionError("happy path did not invoke the web builder exactly once")

        run(wrapper, output, cwd=repository, environment=environment, expected_returncode=2)
        if invocation_count(marker) != 1:
            raise AssertionError("overwrite refusal reached the web builder")

        checker.write_text("#!/usr/bin/env python3\nraise SystemExit(71)\n", encoding="utf-8")
        checker.chmod(0o755)
        run("git", "-C", repository, "add", "scripts/check-web-provenance-v1.py", cwd=root)
        run("git", "-C", repository, "commit", "-qm", "Fail checker fixture", cwd=root)
        checker_failure_output = root / "checker-failure-output"
        checker_failure_output.mkdir()
        run(
            wrapper,
            checker_failure_output,
            cwd=repository,
            environment=environment,
            expected_returncode=71,
        )
        if not (checker_failure_output / PROVENANCE_NAME).is_file():
            raise AssertionError("the provenance writer did not run before the checker")
        if invocation_count(marker) != 2:
            raise AssertionError("checker failure did not occur after the web builder")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true")
    arguments = parser.parse_args()
    if not arguments.self_test:
        parser.error("only --self-test is supported")
    self_test()
    print("sdk build wrapper self-test: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
