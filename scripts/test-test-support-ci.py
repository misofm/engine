#!/usr/bin/env python3
"""Hermetic mutation tests for scripts/check-test-support-ci.py (issue #1021).

Each case copies the workspace manifests and qualification.yml into a scratch root, applies one
mutation and runs the checker there. The unmutated scratch must pass, and must report every
declaring package, or each "fails" below could be failing for an unrelated reason.
"""
from __future__ import annotations

import pathlib
import shutil
import subprocess
import sys
import tempfile
import tomllib

ROOT = pathlib.Path(__file__).resolve().parent.parent
CHECKER = ROOT / "scripts/check-test-support-ci.py"
WORKFLOW = ".github/workflows/qualification.yml"
DEBUG_A_FEATURES = (
    "--features builtins-compiler/test-support,graph/test-support,"
    "host-web/test-support,host-core/test-support,effect-compiler/test-support,"
    "protocol/test-support,engine/realtime-audit\n"
)
DEBUG_B_FEATURES = (
    "--features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support\n"
)
# Each debug job runs its doctests in a second step with the same packages and features (#1422),
# so a feature-list anchor occurs twice. Mutations of the whole-package step are scoped to the
# command that starts it, through the end of its `--features` line.
DEBUG_A_COMMAND = "cargo test --locked --workspace --all-targets \\\n"
DEBUG_B_COMMAND = "cargo test --locked --all-targets \\\n"


def workspace() -> pathlib.Path:
    root = pathlib.Path(tempfile.mkdtemp(prefix="test-support-ci-"))
    shutil.copy2(ROOT / "Cargo.toml", root / "Cargo.toml")
    manifest = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    for member in manifest["workspace"]["members"]:
        (root / member).mkdir(parents=True)
        shutil.copy2(ROOT / member / "Cargo.toml", root / member / "Cargo.toml")
    (root / ".github/workflows").mkdir(parents=True)
    shutil.copy2(ROOT / WORKFLOW, root / WORKFLOW)
    return root


def check(root: pathlib.Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run([sys.executable, "-B", str(CHECKER), "--root", str(root)],
                          stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, check=False)


def mutate(path: pathlib.Path, old: str, new: str) -> None:
    text = path.read_text(encoding="utf-8")
    if text.count(old) != 1:
        raise AssertionError(f"mutation anchor must occur once in {path.name}: {old!r}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def outcome(mutations: list[tuple[str, str, str]]) -> subprocess.CompletedProcess[str]:
    root = workspace()
    try:
        for relative, old, new in mutations:
            mutate(root / relative, old, new)
        return check(root)
    finally:
        shutil.rmtree(root)


def fails(reason: str, uncovered: set[str], *mutations: tuple[str, str, str]) -> None:
    """The checker must refuse, naming exactly `uncovered` as the packages CI no longer covers."""
    result = outcome(list(mutations))
    if result.returncode == 0:
        raise AssertionError(f"mutation accepted: {reason}")
    marker = "gated tests never run in CI: "
    named = set()
    if marker in result.stderr:
        named = set(result.stderr.split(marker, 1)[1].strip().split(", "))
    if named != uncovered:
        raise AssertionError(f"mutation refused with {sorted(named)}, expected {sorted(uncovered)} "
                             f"({reason}): {result.stderr}")


def refused(reason: str, text: str, *mutations: tuple[str, str, str]) -> None:
    result = outcome(list(mutations))
    if result.returncode == 0 or text not in result.stderr:
        raise AssertionError(f"mutation not refused with {text!r} ({reason}): {result.stderr}")


def passes(reason: str, *mutations: tuple[str, str, str]) -> None:
    result = outcome(list(mutations))
    if result.returncode != 0:
        raise AssertionError(f"mutation rejected ({reason}): {result.stderr}")


def in_step(command: str, old: str, new: str) -> tuple[str, str, str]:
    """A workflow mutation of `old` inside the step whose command starts with `command`."""
    text = (ROOT / WORKFLOW).read_text(encoding="utf-8")
    if text.count(command) != 1:
        raise AssertionError(f"step command must occur once in {WORKFLOW}: {command!r}")
    start = text.index(command)
    end = text.index("\n", text.index("--features", start)) + 1
    block = text[start:end]
    if block.count(old) != 1:
        raise AssertionError(f"mutation anchor must occur once in the step: {old!r}")
    return (WORKFLOW, block, block.replace(old, new, 1))


def in_a(old: str, new: str) -> tuple[str, str, str]:
    return in_step(DEBUG_A_COMMAND, old, new)


def in_b(old: str, new: str) -> tuple[str, str, str]:
    return in_step(DEBUG_B_COMMAND, old, new)


def feature_a(removed: str) -> tuple[str, str, str]:
    return in_a(DEBUG_A_FEATURES, DEBUG_A_FEATURES.replace(removed + ",", "", 1))


def feature_b(removed: str) -> tuple[str, str, str]:
    return in_b(DEBUG_B_FEATURES, DEBUG_B_FEATURES.replace("," + removed, "", 1))


def main() -> int:
    baseline = outcome([])
    if baseline.returncode != 0:
        raise AssertionError(f"unmutated workspace rejected: {baseline.stderr}")
    manifests = [
        tomllib.loads((ROOT / member / "Cargo.toml").read_text(encoding="utf-8"))
        for member in tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
        ["workspace"]["members"]
    ]
    declaring = sorted(manifest["package"]["name"] for manifest in manifests
                       if "test-support" in manifest.get("features", {}))
    reported = sorted(line.split("/", 1)[0] for line in baseline.stdout.splitlines()
                      if "/test-support: " in line)
    if reported != declaring or len(declaring) < 9:
        raise AssertionError(
            f"baseline must report every declaring package: {reported} vs {declaring}")

    # Removing a feature that nothing else in the step forwards leaves its package uncovered.
    for package in ("host-web", "protocol"):
        fails(f"{package}/test-support removed from test-debug-a", {package},
              feature_a(f"{package}/test-support"))
    for package in ("parametric-eq", "builtins", "lane"):
        fails(f"{package}/test-support removed from test-debug-b", {package},
              feature_b(f"{package}/test-support"))
    fails("every test-support feature removed from test-debug-a",
          {"builtins-compiler", "effect-compiler", "effect-contract", "graph", "host-core", "host-web",
           "protocol", "rack"},
          in_a(DEBUG_A_FEATURES, "--features engine/realtime-audit\n"))
    fails("every test-support feature removed from test-debug-b", {"builtins", "lane", "parametric-eq"},
          in_b(DEBUG_B_FEATURES, "--features math/lane\n"))

    # Forwarding is modelled from the manifests, not from the feature list's spelling: host-web
    # forwards host-core, which forwards effect-compiler, so both explicit entries are redundant
    # until the manifest stops forwarding.
    redundant = feature_a("host-core/test-support")
    redundant = (WORKFLOW, redundant[1],
                 redundant[2].replace("effect-compiler/test-support,", "", 1))
    passes("host-core and effect-compiler still forwarded from host-web/test-support", redundant)
    fails("host-web stops forwarding host-core/test-support",
          {"effect-compiler", "effect-contract", "host-core"},
          redundant,
          ("hosts/host-web/Cargo.toml",
           'test-support = ["builtins-compiler/test-support", "host-core/test-support"]',
           'test-support = ["builtins-compiler/test-support"]'))
    passes("graph/test-support still forwarded from builtins-compiler/test-support",
           feature_a("graph/test-support"))
    passes("builtins-compiler/test-support still forwarded from host-web/test-support",
           feature_a("builtins-compiler/test-support"))

    # The step must test the package, whole and unconditionally.
    fails("host-web excluded from test-debug-a", {"host-web"},
          in_a("--exclude wasm-gate-corpus \\\n",
                 "--exclude wasm-gate-corpus --exclude host-web \\\n"))
    step_b = "      - name: DSP crates debug tests (lane feature unification pinned explicitly)\n"
    fails("test-debug-b behind a step-level if:", {"builtins", "lane", "parametric-eq"},
          (WORKFLOW, step_b, step_b + "        if: needs.route.outputs.math_closure == 'true'\n"))
    fails("test-debug-b with --no-run", {"builtins", "lane", "parametric-eq"},
          (WORKFLOW, "cargo test --locked --all-targets \\\n            -p lane",
           "cargo test --locked --no-run --all-targets \\\n            -p lane"))
    fails("test-debug-b narrowed to --lib", {"builtins", "lane", "parametric-eq"},
          (WORKFLOW, "cargo test --locked --all-targets \\\n            -p lane",
           "cargo test --locked --lib \\\n            -p lane"))
    fails("test-debug-b with a harness name filter", {"builtins", "lane", "parametric-eq"},
          in_b(DEBUG_B_FEATURES, DEBUG_B_FEATURES[:-1] + " -- --exact bank\n"))
    fails("test-debug-b with a positional name filter", {"builtins", "lane", "parametric-eq"},
          (WORKFLOW, "cargo test --locked --all-targets \\\n            -p lane",
           "cargo test --locked --all-targets bank \\\n            -p lane"))
    fails("the feature only in a shell comment", {"parametric-eq"},
          feature_b("parametric-eq/test-support"),
          (WORKFLOW, "          cargo test --locked --all-targets \\\n            -p lane",
           "          # cargo test -p parametric-eq --features parametric-eq/test-support\n"
           "          cargo test --locked --all-targets \\\n            -p lane"))

    # A new test-support feature nobody enables in CI.
    fails("a new package declares test-support", {"session"},
          ("crates/session/Cargo.toml", "[features]\n", "[features]\ntest-support = []\n"))

    # A misspelt feature must not read as coverage or be silently ignored.
    refused("a misspelt test-support feature", "'host-web' declares no feature 'test-suport'",
            in_a("host-web/test-support,", "host-web/test-suport,"))

    print("test-support CI coverage mutation tests passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
