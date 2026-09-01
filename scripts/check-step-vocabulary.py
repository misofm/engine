#!/usr/bin/env python3
"""Prove one step vocabulary across the repository (issue #242, #239 Amendment 2 A7).

`nudge` is retired. It was #127's working name for value stepping, and in Logic and Pro Tools
"nudge" is established *timeline* vocabulary -- moving regions and the playhead in time -- so
borrowing it for value stepping would collide with the convention the moment arrangement concepts
ship. `step` unifies the noun (the unit), the ladder ("step sizes") and the engine-resolved verb.

This gate refuses the retired spelling wherever it would be a NAME: a JSON schema key, a Rust or
Python identifier, a TypeScript member. Ordinary English prose that happens to use the verb
"nudge" about something else is not a name and is not the subject of the ruling; the allow-list
below records each surviving prose use so a new one has to be admitted deliberately.

Usage:
    check-step-vocabulary.py [--self-test]
"""

from __future__ import annotations

import argparse
import pathlib
import re
import sys

REPOSITORY = pathlib.Path(__file__).resolve().parent.parent

# Extensions that carry names rather than only prose.
SCANNED_SUFFIXES = {".rs", ".py", ".json", ".js", ".mjs", ".ts", ".d.ts", ".sh", ".toml", ".md"}
SKIPPED_DIRECTORIES = {".git", "target", "node_modules"}

# A retired spelling used as a NAME: a quoted schema key, or an identifier-shaped occurrence such
# as `nudge_ladder`, `NudgeLadderV1`, `.nudge`, `nudge:` or `nudge(`.
NAME_PATTERN = re.compile(
    r'"nudge"'
    r"|'nudge'"
    r"|\bnudge_[a-z]"
    r"|[a-z]_nudge"
    r"|\bNudge[A-Z]"
    r"|[a-z]Nudge"
    r"|\.nudge\b"
    r"|\bnudge\s*[:=(]"
)

# This gate necessarily spells the retired name in order to refuse it.
SELF = "scripts/check-step-vocabulary.py"

# Surviving occurrences, each with the reason it is not a name. Every entry is `(path, needle)`;
# a line must contain the needle to be admitted, so an unrelated new line on the same path still
# fails.
ALLOWED = {
    # The ruling's own record, and the red mutation that proves the retired key is refused.
    ("scripts/check-parameter-metadata-v1.py", "replaced #127's"),
    ("scripts/check-parameter-metadata-v1.py", "retired nudge spelling"),
    ("scripts/check-parameter-metadata-v1.py", "nudge=d[\"builtins\"]"),
    ("tools/parameter-metadata/src/lib.rs", "before #242 renamed the vocabulary"),
    ("tools/parameter-metadata/src/lib.rs", "`nudge` is retired"),
    # Ordinary English about moving a test value, unrelated to the parameter ladder.
    ("crates/gate-expander/tests/contract.rs", "threshold is nudged"),
    ("crates/gate-expander/tests/MUTATIONS.md", "the test nudges the threshold"),
    ("crates/parametric-eq/tests/stationary_hoist.rs", "let nudged ="),
    ("crates/parametric-eq/tests/stationary_hoist.rs", "quiet, nudged,"),
    ("tools/bench/src/console.rs", "restate or nudge"),
    # The derivation record for the rename itself has to name what it renamed.
    ("docs/derivations/242-parameter-lattice.md", "slot became a populated"),
}


def scan(root: pathlib.Path) -> tuple[list[tuple[str, int, str]], set[tuple[str, str]]]:
    """Return unadmitted retired-spelling occurrences, and which allow-list rows were used."""
    findings: list[tuple[str, int, str]] = []
    used: set[tuple[str, str]] = set()
    for path in sorted(root.rglob("*")):
        if not path.is_file() or path.suffix not in SCANNED_SUFFIXES:
            continue
        relative = path.relative_to(root).as_posix()
        if relative == SELF:
            continue
        if any(part in SKIPPED_DIRECTORIES for part in path.relative_to(root).parts):
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError):
            continue
        if "nudge" not in text.lower():
            continue
        for number, line in enumerate(text.splitlines(), start=1):
            if "nudge" not in line.lower():
                continue
            admitted = {
                (allowed_path, needle)
                for allowed_path, needle in ALLOWED
                if relative == allowed_path and needle in line
            }
            if admitted:
                used |= admitted
                continue
            if NAME_PATTERN.search(line):
                findings.append((relative, number, line.strip()))
    return findings, used


def self_test() -> int:
    """Prove the gate discriminates: the real tree is clean and each mutation is caught."""
    failures = 0
    live, used = scan(REPOSITORY)
    # A stale allow-list row would quietly re-admit the retired spelling somewhere else later.
    for stale in sorted(ALLOWED - used):
        print(f"self-test FAILED: allow-list row matches nothing -- {stale}", file=sys.stderr)
        failures += 1
    if live:
        for relative, number, line in live:
            print(f"unadmitted retired spelling: {relative}:{number}: {line}", file=sys.stderr)
        failures += 1

    mutations = [
        ('a schema key returns to the retired spelling', '  "nudge": null'),
        ("a Rust identifier returns to the retired spelling", "pub struct NudgeLadder {"),
        ("a field returns to the retired spelling", "    nudge: StepLadder,"),
        ("an accessor returns to the retired spelling", "descriptor.nudge()"),
        ("a snake-case identifier returns", "fn resolve_nudge_size() {}"),
    ]
    for name, line in mutations:
        if not NAME_PATTERN.search(line):
            print(f"self-test FAILED: mutation escaped -- {name}", file=sys.stderr)
            failures += 1

    # A prose sentence is not a name, and must not be reported as one.
    if NAME_PATTERN.search("// the test nudges the threshold until it matches"):
        print("self-test FAILED: prose reported as a name", file=sys.stderr)
        failures += 1

    if failures == 0:
        print("step vocabulary self-test passed")
    return failures


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true")
    arguments = parser.parse_args()
    if arguments.self_test:
        return self_test()
    findings, _ = scan(REPOSITORY)
    for relative, number, line in findings:
        print(f"retired `nudge` spelling: {relative}:{number}: {line}", file=sys.stderr)
    if findings:
        return 1
    print("step vocabulary is uniform")
    return 0


if __name__ == "__main__":
    sys.exit(main())
