#!/usr/bin/env python3
"""Classify a GitHub Actions diff into the least broad safe qualification route.

This program deliberately has no YAML or GitHub API dependency.  A missing revision, a failed
diff, an empty change set, or an unrecognised name is *full*, so a future input cannot silently
weaken qualification.  `--name-status-file` exists for hermetic tests; Actions uses the Git diff
path through `--base` and `--head`.
"""
from __future__ import annotations

import argparse
import pathlib
import re
import subprocess
import sys

EVIDENCE_PREFIXES = (".github/ISSUE_SPECS/", "docs/")
EVIDENCE_FILES = {"README.md"}
DSP_RESEARCH_PREFIX = "dsp-research/"
SDK_PREFIXES = ("sdk/",)
SDK_FILES = {
    "scripts/check-sdk-deletions.py",
    "scripts/check-sdk-generated.sh",
    "scripts/check-sdk-headless.sh",
    "scripts/check-sdk-types.sh",
    "scripts/sdk-package.sh",
    "scripts/test-sdk-artifact-builder-output-contract.sh",
}
GIT_DIFF_OPTIONS = ("--name-status", "-z", "--find-renames", "--find-copies-harder")

# Two closure-derived step conditions (design #359 WP-1, §2/§5): a tiny closure gating an
# expensive step *inside* an already-selected shard. They never widen or narrow `route` itself.
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


def path_kind(path: str) -> str | None:
    """Return the narrowest class for one repository-relative, nonempty path."""
    if not path or path.startswith("/") or "\\" in path or "/../" in f"/{path}/":
        return None
    if path in EVIDENCE_FILES or path.startswith(EVIDENCE_PREFIXES):
        return "evidence"
    if path in SDK_FILES or path.startswith(SDK_PREFIXES):
        return "sdk"
    if path.startswith(DSP_RESEARCH_PREFIX):
        return "evidence" if path.endswith(".md") else None
    return None


def classify_paths(paths: list[str]) -> str:
    """Classify paths, preserving a full route for empty, malformed, mixed, or unknown input."""
    if not paths:
        return "full"
    kinds = [path_kind(path) for path in paths]
    if any(kind is None for kind in kinds):
        return "full"
    if all(kind == "evidence" for kind in kinds):
        return "evidence"
    if all(kind in {"evidence", "sdk"} for kind in kinds):
        return "sdk"
    return "full"


def is_math_closure_path(path: str) -> bool:
    """A tiny, exact closure: math's reverse workspace closure is exactly {lane}."""
    return path.startswith(MATH_CLOSURE_PREFIXES) or path in MATH_CLOSURE_FILES


def is_release_input_path(path: str) -> bool:
    """The existing release-build.yml PR filter, plus its shape-policy inputs."""
    return path in RELEASE_INPUT_FILES or path.endswith(RELEASE_INPUT_SUFFIX)


def compute_flags(paths: list[str] | None) -> tuple[bool, bool]:
    """Both flags are fail-safe true whenever the path list itself is unavailable/untrusted.

    This mirrors `classify_paths`'s own `if not paths: return "full"`: a `None` (malformed diff,
    missing base, unknown status, workflow_dispatch) and an empty list (empty diff) are the same
    fail-safe case, and both must force the expensive gated step to run.
    """
    if not paths:
        return True, True
    math_closure = any(is_math_closure_path(path) for path in paths)
    release_inputs = any(is_release_input_path(path) for path in paths)
    return math_closure, release_inputs


def parse_name_status(raw: bytes) -> list[str] | None:
    """Parse `git diff --name-status -z`, retaining both rename/copy names."""
    fields = raw.split(b"\0")
    if not fields or fields[-1] != b"":
        return None
    fields.pop()
    paths: list[str] = []
    index = 0
    while index < len(fields):
        try:
            status = fields[index].decode("ascii")
        except UnicodeDecodeError:
            return None
        index += 1
        if status in {"A", "D", "M", "T"}:
            count = 1
        elif re.fullmatch(r"[RC][0-9]{3}", status) and int(status[1:]) <= 100:
            count = 2
        else:
            return None
        if index + count > len(fields):
            return None
        for value in fields[index:index + count]:
            try:
                path = value.decode("utf-8")
            except UnicodeDecodeError:
                return None
            if not path:
                return None
            paths.append(path)
        index += count
    return paths


def diff_paths(event: str, base: str, head: str) -> list[str] | None:
    if not base or not head or set(base) == {"0"} or set(head) == {"0"}:
        return None
    if event == "pull_request":
        revisions = [f"{base}...{head}"]
    elif event == "push":
        revisions = [base, head]
    else:
        return None
    result = subprocess.run(
        ["git", "diff", *GIT_DIFF_OPTIONS, *revisions],
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        check=False,
    )
    if result.returncode:
        return None
    return parse_name_status(result.stdout)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--event", required=True)
    parser.add_argument("--base")
    parser.add_argument("--head")
    parser.add_argument("--name-status-file", type=pathlib.Path)
    parser.add_argument("--path", action="append", default=[])
    parser.add_argument("--flags", action="store_true",
                         help="also print GITHUB_OUTPUT-style route/math_closure/release_inputs lines")
    args = parser.parse_args()

    if args.event == "workflow_dispatch":
        paths = None
    elif args.name_status_file is not None:
        try:
            paths = parse_name_status(args.name_status_file.read_bytes())
        except OSError:
            paths = None
    elif args.path:
        paths = args.path
    else:
        paths = diff_paths(args.event, args.base or "", args.head or "")

    route = classify_paths(paths) if paths is not None else "full"
    math_closure, release_inputs = compute_flags(paths)

    # Bare mode is unchanged: exactly the route, one line, for existing callers.
    print(route)
    if args.flags:
        print(f"route={route}")
        print(f"math_closure={'true' if math_closure else 'false'}")
        print(f"release_inputs={'true' if release_inputs else 'false'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
