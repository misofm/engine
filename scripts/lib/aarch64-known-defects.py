#!/usr/bin/env python3
"""The known AArch64 defects of issue #1017, as expected failures by name, and their judges.

Each row names its owning issue. Two scripts consume this file, so the rows live in one place:

* `scripts/run-aarch64-tests.sh`, the `aarch64-debug` and `aarch64-release` legs, reads the test
  rows. A row names one test and the reason it fails: a string its panic message must contain,
  normally the AArch64 digest or the assertion text that the defect produces. The leg skips each
  row's test by exact name in its main run and then runs it alone. The run must fail as that one
  test and for that reason. A pass, a different panic, or the same test failing for another reason
  fails the leg. So a fixed defect cannot leave a stale row, and a new regression cannot hide
  behind an old one.
* `scripts/check-cross-targets.sh`, the `cross-target` job, reads the iOS memset ceilings. The row
  for each product crate carries its count of `bl _memset_pattern16` in that crate's
  `aarch64-apple-ios` release assembly (#1018). A crate at zero with a row fails ("fixed: delete
  the row"). A crate above its ceiling fails ("rose"). A crate with calls and no row fails, and so
  does a row for a crate that is not a product crate. A crate below its ceiling passes and is
  reported, so a partial fix shows as progress and the defect reads fixed only when every row is
  gone.

Subcommands:

    rows <debug|release>                       issue|package|selector|name, one per line
    judge-skips <debug|release> <list-output>  each row names exactly one test in the leg
    judge-test <debug|release> <name> <log> <exit-status>
    judge-memset <counts-file> <product-crates-file>
    --self-test                                hermetic mutations of every judge
"""
from __future__ import annotations

import pathlib
import re
import sys

# issue, package, selector (`lib` or `test:<integration test>`), exact test name, reason.
TEST_ROWS: dict[str, list[tuple[str, str, str, str, str]]] = {
    # #1065 (AArch64 NaN encodings) left this leg: class-A identity reads every NaN as one value
    # (owner decision 10), so its two pins fold NaN words through `dsp_reference::class_a` and
    # pass on both CPUs.
    "debug": [],
    "release": [
        # LANE-3 (#1019): in release the D8 `select(a > b, a, b)` folds into `fmaxnm`/`fminnm`
        # inside `exp2_lane` and `log2_lane`, scalar and vector alike. Those instructions answer
        # differently on NaN and signed-zero inputs. The same tests pass in the debug leg.
        ("1019", "math", "test:m2_lane_identity", "m2_exp2_lane_identity",
         "exp2_lane: scalar digest 69c5e8f5e4639e1438b6f38bce75a7267904f84501a3d89c2704146272c4942a"
         " does not match the pin"),
        ("1019", "math", "test:m2_lane_identity", "m2_log2_lane_identity",
         "log2_lane: width 4 differs from the scalar oracle at input 870 (NaN): 0x43c06e2d vs "
         "0xc2fc0000"),
    ],
}

# Product crate -> (owning issue, ceiling): `bl _memset_pattern16` calls in the crate's
# `aarch64-apple-ios` release assembly, as `scripts/check-cross-targets.sh` emits it. Recorded by
# #1017 attempt 2 on Rust 1.97.1. A stored `f32x4` splat constant lowers to a libc call on Apple
# targets: `lane::FLUSH_EPS` and also `1.0`, `0.5`, `2.0`, `1e-8`, `f32::MIN_POSITIVE` and others.
# Nearly every call sits inside a render function; the register in `docs/TARGET_MATRIX.md` names
# them. #1112 lowered six rows (builtins 376, gate-expander 181, graph 20, multiband-compressor 1132,
# parametric-eq 146, transient-shaper 534): it removed the eight-lane instantiations from the
# AArch64 builds, and their two-half `f32x8` splats made the same calls. #1328 lowered two more
# (builtins 194 -> 186, parametric-eq 132 -> 122), as `check-cross-targets.sh` asked after the
# joint SVF flush and the EQ's dry masks held in channel state, and its amendment A9 two more
# (builtins 186 -> 71, parametric-eq 122 -> 88), as the check asked: the builtin input chain now
# loads its bodies' vector constants from its prepared coefficients (`InputChainConstants`) and the
# EQ reads its rest thresholds from memory, in place of splatted compares. Its follow-up lowered
# one more (parametric-eq 88 -> 48), as the check asked: the EQ's cascades run in two forms per
# block (armed and unarmed) and carry `lane::FLUSH_EPS` as a word from the prepared EQ
# (`ArmedRest`, `UnarmedRest`) instead of splatting it in each kernel instantiation.
IOS_MEMSET_CEILINGS: dict[str, tuple[str, int]] = {
    "builtins": ("1018", 71),
    "compressor": ("1018", 970),
    "gate-expander": ("1018", 91),
    "graph": ("1018", 10),
    "host-core": ("1018", 4),
    "multiband-compressor": ("1018", 566),
    "parametric-eq": ("1018", 48),
    "soft-clip": ("1018", 22),
    "transient-shaper": ("1018", 268),
    "true-peak-limiter": ("1018", 104),
}


class Refused(RuntimeError):
    pass


Row = tuple[str, str, str, str, str]


def row_for(rows: list[Row], name: str) -> Row:
    rows = [row for row in rows if row[3] == name]
    if len(rows) != 1:
        raise Refused(f"{name}: not exactly one row")
    return rows[0]


def judge_skips(rows: list[Row], listing: str) -> None:
    """`listing` is the leg's `cargo test ... -- --list` output. `--exact --skip <name>` applies
    to every test binary in the run, so each row's name must name exactly one test there."""
    for issue, package, _, name, _ in rows:
        count = sum(1 for line in listing.splitlines() if line == f"{name}: test")
        if count != 1:
            raise Refused(f"expected failure {package} {name} (#{issue}) names {count} tests in "
                          "the leg; a row must name exactly one")


def panic_block(log: str, name: str) -> list[str]:
    """The lines of `name`'s panic message: from its `thread '<name>' ... panicked at` line up to
    the next blank or `note:` line."""
    lines = log.splitlines()
    starts = [index for index, line in enumerate(lines)
              if line.startswith(f"thread '{name}'") and " panicked at " in line]
    if len(starts) != 1:
        raise Refused(f"{name}: expected one panic from the test, found {len(starts)}")
    block = []
    for line in lines[starts[0]:]:
        if block and (not line.strip() or line.startswith("note:")):
            break
        block.append(line)
    return block


def judge_test(rows: list[Row], name: str, log: str, status: int) -> str:
    issue, package, _, _, reason = row_for(rows, name)
    label = f"expected failure {package} {name} (#{issue})"
    if status == 0:
        raise Refused(f"{label} now passes: delete its row from scripts/lib/aarch64-known-defects.py")
    if (f"test {name} ... FAILED" not in log.splitlines()
            or len(re.findall(r"^test result: FAILED\. 0 passed; 1 failed;", log, re.MULTILINE)) != 1):
        raise Refused(f"{label} did not fail as that one test")
    block = panic_block(log, name)
    if not any(reason in line for line in block):
        raise Refused(f"{label} failed for another reason: its panic does not contain "
                      f"{reason!r}:\n" + "\n".join(block))
    return f"expected failure (#{issue}): {package} {name}"


def judge_memset(counts: dict[str, int], products: list[str],
                 ceilings: dict[str, tuple[str, int]]) -> list[str]:
    report: list[str] = []
    errors: list[str] = []
    for crate in sorted(set(ceilings) - set(products)):
        errors.append(f"{crate}: has a row but is not a product crate; delete its row")
    for crate in sorted(set(products) - set(counts)):
        errors.append(f"{crate}: no count was measured")
    for crate in sorted(counts):
        count = counts[crate]
        row = ceilings.get(crate)
        if row is None:
            if count:
                errors.append(f"{crate}: {count} memset_pattern16 calls and no row: a new libc "
                              "call in the iOS build (#1018)")
            continue
        issue, ceiling = row
        if count == 0:
            errors.append(f"{crate}: expected failure ios-asm-memset-pattern16 (#{issue}) now "
                          "passes: delete its row from scripts/lib/aarch64-known-defects.py")
        elif count > ceiling:
            errors.append(f"{crate}: memset_pattern16 calls rose from {ceiling} to {count} "
                          f"(#{issue})")
        else:
            note = "" if count == ceiling else f", down from {ceiling}: lower its row"
            report.append(f"ios-asm-memset-pattern16: expected failure (#{issue}): {crate} "
                          f"{count} calls{note}")
    if errors:
        raise Refused("ios-asm-memset-pattern16:\n  " + "\n  ".join(errors))
    return report


def parse_counts(text: str) -> dict[str, int]:
    counts: dict[str, int] = {}
    for line in text.splitlines():
        crate, count = line.split()
        if crate in counts:
            raise Refused(f"{crate}: counted twice")
        counts[crate] = int(count)
    return counts


def refuses(action) -> bool:
    try:
        action()
    except Refused:
        return True
    return False


def self_test() -> None:
    """Hermetic mutations of every judge, over synthetic rows: the real tables may shrink to
    nothing as defects are fixed, and the judges must still be proved."""
    reason = 'left: "' + "ab" * 32 + '"'
    name = "kernel::tests::scenario_is_pinned"
    rows: list[Row] = [("9999", "crate-a", "lib", name, reason),
                       ("9999", "crate-b", "test:t", "other_test", "assertion text")]
    ok_log = (f"running 1 test\ntest {name} ... FAILED\n\nfailures:\n\n---- {name} stdout ----\n\n"
              f"thread '{name}' (7) panicked at crates/x.rs:1:9:\nassertion `left == right` failed\n"
              f"  {reason}\n right: \"pin\"\nnote: run with `RUST_BACKTRACE=1`\n\n"
              f"test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 12 filtered out\n")
    assert judge_test(rows, name, ok_log, 101).startswith("expected failure (#9999)")
    mutations = {
        "passes": (ok_log, 0),
        "planted panic": (ok_log.replace(reason, "PLANTED: an unrelated regression"), 101),
        "another digest": (ok_log.replace(reason, reason[:-3] + '00"'), 101),
        "reason outside the panic": (ok_log.replace(f"  {reason}\n", "").replace(
            "running 1 test\n", f"running 1 test\n{reason}\n"), 101),
        "two failures": (ok_log.replace("0 passed; 1 failed", "0 passed; 2 failed"), 101),
        "not the test": (ok_log.replace(f"test {name} ... FAILED", f"test {name}_x ... FAILED"),
                         101),
    }
    for label, (log, status) in mutations.items():
        assert refuses(lambda: judge_test(rows, name, log, status)), label
    assert refuses(lambda: judge_test(rows, "not_a_row", ok_log, 101))

    listing = f"{name}: test\nother_test: test\nunrelated: test"
    judge_skips(rows, listing)
    assert refuses(lambda: judge_skips(rows, listing.replace(f"{name}: test\n", "")))
    assert refuses(lambda: judge_skips(rows, listing + f"\n{name}: test"))

    ceilings = {"crate-a": ("9999", 10), "crate-b": ("9999", 3)}
    products = ["crate-a", "crate-b", "crate-c"]
    counts = {"crate-a": 10, "crate-b": 3, "crate-c": 0}
    assert len(judge_memset(counts, products, ceilings)) == 2
    assert any("down from 10" in line
               for line in judge_memset(dict(counts, **{"crate-a": 4}), products, ceilings))
    for label, mutated, crates in (
        ("a row reaches zero", dict(counts, **{"crate-b": 0}), products),
        ("a count rises", dict(counts, **{"crate-a": 11}), products),
        ("a new crate has calls", dict(counts, **{"crate-c": 1}), products),
        ("a row for a crate that left the closure", counts, ["crate-a", "crate-c"]),
        ("a product crate was not measured", {k: v for k, v in counts.items() if k != "crate-c"},
         products),
    ):
        assert refuses(lambda: judge_memset(mutated, crates, ceilings)), label
    assert refuses(lambda: parse_counts("lane 0\nlane 0\n"))
    print("aarch64 known-defect judges: self-test passed")


def main(argv: list[str]) -> int:
    try:
        if argv == ["--self-test"]:
            self_test()
        elif len(argv) == 2 and argv[0] == "rows" and argv[1] in TEST_ROWS:
            for issue, package, selector, name, _ in TEST_ROWS[argv[1]]:
                print(f"{issue}|{package}|{selector}|{name}")
        elif len(argv) == 3 and argv[0] == "judge-skips" and argv[1] in TEST_ROWS:
            judge_skips(TEST_ROWS[argv[1]],
                        pathlib.Path(argv[2]).read_text(encoding="utf-8", errors="replace"))
        elif len(argv) == 5 and argv[0] == "judge-test" and argv[1] in TEST_ROWS:
            log = pathlib.Path(argv[3]).read_text(encoding="utf-8", errors="replace")
            print(judge_test(TEST_ROWS[argv[1]], argv[2], log, int(argv[4])))
        elif len(argv) == 3 and argv[0] == "judge-memset":
            counts = parse_counts(pathlib.Path(argv[1]).read_text(encoding="utf-8"))
            products = pathlib.Path(argv[2]).read_text(encoding="utf-8").split()
            print("\n".join(judge_memset(counts, products, IOS_MEMSET_CEILINGS)))
        else:
            print(__doc__, file=sys.stderr)
            return 2
    except Refused as error:
        print(f"aarch64 known defects: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
