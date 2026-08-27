#!/usr/bin/env python3
"""The browser-correctness resource-row staleness tripwire (issue #217).

`hosts/miso-engine-host-web/tests/browser-v1/expected.json` pins the *resource report* of the
shipped `wasm32-unknown-unknown` simd128 worklet module alongside its PCM digests. The digests are
gated from three directions; the resource rows had exactly one consumer --
`scripts/web-audioworklet-browser-correctness.py --check` -- and that harness is not a sweep row,
because its sibling modes need a browser and the sweep's charter excludes browsers.

So the rows went stale, twice, with every gate green:

* d6674ce (#212, Strip Job 1) moved `builtinRetainedBytes` 706 -> 722 and
  `graphSessionPlusPlanBytes` 20524 -> 20492. #212 re-pinned its *native* mirror
  (`crates/miso-engine-capi/tests/resource_lifecycle.rs`) in the same commit; the browser pin has
  no such mirror and was missed.
* 0fb9325 (#216, Strip Job 2) moved them again, 722 -> 802 and 20492 -> 30374.

Between them, thirty-three merges moved nothing: this is not accumulated drift, it is two
un-re-pinned commits, and one gate red at either would have ended it.

**What this gate is.** Two witnesses of the same fixture compile, and no browser in either:

1. *The wasm32 leg, which is the one that catches the drift.* Build the simd128 module, run the
   fixture's own `direct-oracle.mjs` under Node in `MISO_ENGINE_WEB_ORACLE_PRINT=1` mode -- the
   house's derivation instrument, the same one a re-pin is read off -- and require the printed
   document to equal `expected.json`'s `directOracle` exactly. Node and a Wasm build, no browser,
   no WebDriver, no audio device.
2. *The native leg, which keeps the first one honest about what it is proving.* Compile the same
   `session.toml` through the same `AudioWorkletEngineHost` facade with the same
   `WebPrepareConfigV1` the oracle writes -- `examples/browser_fixture_resources.rs` -- and check
   each row against the wasm32 answer according to the class it is declared in.

**Why the native leg cannot simply be the gate.** The obvious cheap tripwire is "compile the
fixture natively, compare to `expected.json`", and it does not work: every one of the four rows
that drifted is target-dependent, so a native comparison would be red on a perfectly current pin.
Measured at 270f072, native x86_64 against wasm32 simd128:

    builtinRetainedBytes         802 -> 954      graphSessionPlusPlanBytes  30374 -> 45266
    graphIncrementalPlanBytes  28838 -> 43730    graphMetadataBytes          3455 ->  5591
    bridgeMetadataBytes         3753 ->  4417    bridgeRetainedBytes      1075129 -> 1075793
    sourceTotalBytes            2206 ->  2370    sourceOverheadBytes         1182 ->  1346

Two independent reasons, both structural. Pointer width: these rows are `size_of` sums over
structures holding references, `Vec` headers and `usize` -- `graph_metadata_bytes` is literally
`size_of::<GraphNode>()` times the node count plus four more such terms -- and a 64-bit host lays
every one of them out larger. Lane width: the retained bank payloads are AoSoA, so they scale with
`Lane::LANES`, which is 4 for the browser's simd128 module and 8 for the workspace's pinned
`+avx2` native toolchain. The browser report even labels this: the module reports
`BACKEND_SIMD128`, the native build reports `BACKEND_SCALAR`, because `selected_backend()` asks
`target_arch = "wasm32"`.

So the two legs divide the rows, and the division itself is checked in both directions. A row
declared target-independent must agree between the two builds; a row declared pointer/lane
sensitive must *disagree*. That is what stops the partition from rotting: a row that quietly
becomes target-independent, or one that quietly stops being, goes red here and has to be
re-justified rather than silently dropping out of, or into, the native leg's coverage.

`--self-test` proves the comparator discriminates before it is trusted: it fabricates a consistent
triple, checks it green, and then requires every one of the red mutations below -- including the
two real historical staleness values, 706 and 722 -- to be caught. It builds nothing and is
instant, so the gate runs it first on every invocation.
"""

from __future__ import annotations

import argparse
import json
import os
import pathlib
import shutil
import subprocess
import sys
import tempfile

REPO = pathlib.Path(__file__).resolve().parent.parent
FIXTURE = REPO / "hosts/miso-engine-host-web/tests/browser-v1"
EXPECTED_JSON = FIXTURE / "expected.json"
DIRECT_ORACLE = FIXTURE / "direct-oracle.mjs"
DELIVERY_SCRIPT = REPO / "scripts/build-web-audioworklet.sh"

MODULE_NAME = "miso-engine-v2-audio-worklet.simd128.wasm"
# Shared with every other `target/ci/*` gate: a persistent directory, so a sweep does not pay a
# cold Wasm build of the whole workspace on every run.
WASM_TARGET_DIR = REPO / "target/ci/browser-expected-resources"
WASM_TARGET = "wasm32-unknown-unknown"
# The one flag that changes an answer in this report: it selects `Lane::LANES == 4` and
# `BACKEND_SIMD128`. `-C strip=debuginfo` is a delivery-size decision and moves no row, so this
# gate does not carry it.
SIMD128_FLAG = "-C target-feature=+simd128"

DIRECT_ORACLE_SCHEMA = "miso.web.browser.direct-oracle.v2"
BACKEND_SIMD128 = 1
BACKEND_SCALAR = 0

# Rows whose value is a byte count both builds must agree on. Config-derived staging capacities
# (the caller writes them and the report echoes them back), the two `max()` rows this fixture's
# 1 MiB session-TOML staging dominates, and the four rows this identity fixture drives to zero --
# it has no delay, no scalar effect and no observation capacity. None of them reads a host
# pointer or a lane count.
TARGET_INDEPENDENT = frozenset(
    {
        "configBytes",
        "statusBytes",
        "sessionTomlBytes",
        "diagnosticBytes",
        "sourceIdBytes",
        "sourcePcmStagingBytes",
        "outputPcmBytes",
        "largestBridgeAllocationBytes",
        "largestNamedAllocationBytes",
        "effectScalarStateBytes",
        "effectScalarScratchBytes",
        "graphDelayBytes",
        "observationRetainedBytes",
    }
)

# Rows that are `size_of` sums over pointer-bearing structures, AoSoA payloads scaled by
# `Lane::LANES`, or both. A 64-bit host with eight lanes cannot produce the browser's numbers, and
# the gate requires it not to: an equality here would mean the row stopped measuring what its name
# says, and the native leg would start silently vouching for a number it cannot see.
POINTER_OR_LANE_SENSITIVE = frozenset(
    {
        "bridgeMetadataBytes",
        "bridgeRetainedBytes",
        "sourceTotalBytes",
        "sourceOverheadBytes",
        "builtinRetainedBytes",
        "graphSessionPlusPlanBytes",
        "graphIncrementalPlanBytes",
        "graphMetadataBytes",
    }
)

# Not byte counts: the compile's shape, which both builds must reproduce, and the backend label,
# which is a statement about the target and must therefore differ.
SHAPE_ROWS = frozenset({"sampleRateHz", "quantumFrames"})
BACKEND_ROW = "backend"

# The three digests `expected.json` pins. Named here so a report that stops carrying one is a
# failure rather than a silently skipped comparison.
DIGEST_PATHS = (
    ("simd128", "pcmF32leSha256"),
    ("commandTimeline", "pcmF32leSha256"),
    ("observationTimeline", "pcmF32leSha256"),
)


class Invalid(Exception):
    """One rule this gate enforces was broken."""


def require(condition: object, message: str) -> None:
    if not condition:
        raise Invalid(message)


def resource_rows(document: dict) -> dict:
    """The `simd128.resources` block of a `directOracle`-shaped document."""
    simd128 = document.get("simd128")
    require(isinstance(simd128, dict), "the oracle document has no simd128 leg")
    rows = simd128.get("resources")
    require(isinstance(rows, dict), "the oracle document's simd128 leg has no resources block")
    return rows


def check_partition(rows: set, independent: frozenset, sensitive: frozenset) -> None:
    """Every row is classified exactly once, and the classes name nothing that is not a row."""
    classified = independent | sensitive | SHAPE_ROWS | {BACKEND_ROW}
    both = independent & sensitive
    require(not both, f"rows declared both target-independent and target-sensitive: {sorted(both)}")
    unclassified = rows - classified
    require(
        not unclassified,
        "the resource report carries rows this gate does not classify, so the native leg would "
        f"silently ignore them: {sorted(unclassified)}",
    )
    absent = classified - rows
    require(
        not absent,
        f"this gate classifies rows the resource report does not carry: {sorted(absent)}",
    )


def check_pins(actual: dict, expected: dict) -> None:
    """The whole printed oracle document equals the pin, row by row, with named failures."""
    require(
        actual.get("schema") == DIRECT_ORACLE_SCHEMA,
        f"the oracle printed schema {actual.get('schema')!r}, not {DIRECT_ORACLE_SCHEMA!r}",
    )
    actual_rows = resource_rows(actual)
    expected_rows = resource_rows(expected)
    missing = set(expected_rows) - set(actual_rows)
    extra = set(actual_rows) - set(expected_rows)
    require(not missing, f"expected.json pins rows the report does not carry: {sorted(missing)}")
    require(not extra, f"the report carries rows expected.json does not pin: {sorted(extra)}")
    stale = {
        name: (expected_rows[name], actual_rows[name])
        for name in sorted(expected_rows)
        if expected_rows[name] != actual_rows[name]
    }
    require(
        not stale,
        "expected.json's resource rows are stale against the built module "
        "(pinned -> actual): "
        + ", ".join(f"{name} {pin} -> {live}" for name, (pin, live) in stale.items())
        + ". Re-derive with MISO_ENGINE_WEB_ORACLE_PRINT=1 and re-pin with the byte accounting "
        "for each moved row in the commit message.",
    )
    for leg, key in DIGEST_PATHS:
        actual_leg = actual.get(leg)
        expected_leg = expected.get(leg)
        require(isinstance(actual_leg, dict), f"the report has no {leg} leg")
        require(isinstance(expected_leg, dict), f"expected.json has no {leg} leg")
        require(key in actual_leg and key in expected_leg, f"{leg}.{key} is not carried")
        require(
            actual_leg[key] == expected_leg[key],
            f"{leg}.{key} moved: pinned {expected_leg[key]}, rendered {actual_leg[key]}",
        )
    require(actual == expected, "the printed oracle document differs from expected.json")


def check_native_witness(
    wasm_rows: dict,
    native_rows: dict,
    independent: frozenset,
    sensitive: frozenset,
) -> None:
    """The second witness, row class by row class."""
    for name in sorted(SHAPE_ROWS):
        require(name in native_rows, f"the native report has no {name}")
        require(
            str(native_rows[name]) == str(wasm_rows[name]),
            f"{name} differs between the two builds ({wasm_rows[name]} vs {native_rows[name]}): "
            "both compile the same fixture at the same rate and quantum",
        )
    require(
        int(wasm_rows[BACKEND_ROW]) == BACKEND_SIMD128,
        f"the module reports backend {wasm_rows[BACKEND_ROW]}, not BACKEND_SIMD128; it was not "
        "built with +simd128, so its lane width is not the browser's",
    )
    require(
        int(native_rows[BACKEND_ROW]) == BACKEND_SCALAR,
        f"the native build reports backend {native_rows[BACKEND_ROW]}, not BACKEND_SCALAR; "
        "selected_backend() asks target_arch = \"wasm32\", so a native build cannot report simd128",
    )
    disagreed = {
        name: (wasm_rows[name], native_rows[name])
        for name in sorted(independent)
        if str(wasm_rows[name]) != str(native_rows[name])
    }
    require(
        not disagreed,
        "rows declared target-independent disagree between the wasm32 and native builds "
        "(wasm32 -> native): "
        + ", ".join(f"{name} {left} -> {right}" for name, (left, right) in disagreed.items()),
    )
    agreed = sorted(name for name in sensitive if str(wasm_rows[name]) == str(native_rows[name]))
    require(
        not agreed,
        "rows declared pointer/lane sensitive now agree between the wasm32 and native builds: "
        + ", ".join(f"{name} = {wasm_rows[name]}" for name in agreed)
        + ". Either the row stopped measuring host layout, in which case move it to "
        "TARGET_INDEPENDENT with the reason, or the native build is not the pinned "
        "+avx2 toolchain.",
    )


def validate(
    actual: dict,
    expected: dict,
    native_rows: dict,
    independent: frozenset = TARGET_INDEPENDENT,
    sensitive: frozenset = POINTER_OR_LANE_SENSITIVE,
) -> None:
    check_pins(actual, expected)
    wasm_rows = resource_rows(actual)
    check_partition(set(wasm_rows), independent, sensitive)
    check_native_witness(wasm_rows, native_rows, independent, sensitive)


# --- collection -----------------------------------------------------------------------------


def build_module(destination: pathlib.Path) -> None:
    """Build the shipped simd128 module and place it under the name the oracle expects."""
    text = DELIVERY_SCRIPT.read_text()
    require(
        "target-feature=+simd128" in text,
        f"{DELIVERY_SCRIPT.name} no longer builds the shipped module with +simd128, so this "
        "gate's module would not be the shipped one",
    )
    require(
        "-p miso-engine-host-web" in text and WASM_TARGET in text,
        f"{DELIVERY_SCRIPT.name} no longer builds miso-engine-host-web for {WASM_TARGET}",
    )
    environment = dict(os.environ)
    environment["CARGO_TARGET_DIR"] = str(WASM_TARGET_DIR)
    environment["RUSTFLAGS"] = SIMD128_FLAG
    subprocess.run(
        [
            "cargo", "build", "--locked", "--release",
            "--target", WASM_TARGET, "-p", "miso-engine-host-web",
        ],
        cwd=REPO,
        env=environment,
        check=True,
    )
    shutil.copyfile(
        WASM_TARGET_DIR / WASM_TARGET / "release" / "miso_engine_host_web.wasm",
        destination / MODULE_NAME,
    )


def print_oracle(artifacts: pathlib.Path) -> dict:
    """Run the fixture's own oracle in its print mode and return the document it derives."""
    runtime = shutil.which("node") or shutil.which("bun")
    require(runtime is not None, "a Node.js-compatible runtime is required for the raw-Wasm oracle")
    environment = dict(os.environ)
    environment["MISO_ENGINE_WEB_ORACLE_PRINT"] = "1"
    completed = subprocess.run(
        [runtime, str(DIRECT_ORACLE), str(artifacts), str(EXPECTED_JSON)],
        cwd=REPO,
        env=environment,
        check=True,
        capture_output=True,
        text=True,
    )
    return json.loads(completed.stdout)


def native_report() -> dict:
    completed = subprocess.run(
        [
            "cargo", "run", "--locked", "--release", "-q",
            "-p", "miso-engine-host-web", "--example", "browser_fixture_resources",
        ],
        cwd=REPO,
        check=True,
        capture_output=True,
        text=True,
    )
    return json.loads(completed.stdout)


def collect(artifacts: pathlib.Path | None) -> tuple[dict, dict, dict]:
    expected = json.loads(EXPECTED_JSON.read_text())["directOracle"]
    if artifacts is not None:
        actual = print_oracle(artifacts.resolve())
    else:
        with tempfile.TemporaryDirectory() as staging:
            module_directory = pathlib.Path(staging)
            build_module(module_directory)
            actual = print_oracle(module_directory)
    return actual, expected, native_report()


# --- the self-test --------------------------------------------------------------------------


def consistent_triple() -> tuple[dict, dict, dict]:
    """A pin, a report that matches it, and a native witness the declared classes accept.

    Built from the committed `expected.json`, so the self-test's green case is the real document
    and its mutations are the real rows. The native witness is fabricated rather than compiled:
    the classes are what is under test here, not the compiler.
    """
    expected = json.loads(EXPECTED_JSON.read_text())["directOracle"]
    actual = json.loads(json.dumps(expected))
    rows = resource_rows(expected)
    native = {name: value for name, value in rows.items() if name in TARGET_INDEPENDENT}
    # A 64-bit host is larger in every sensitive row; `+ 8` stands for "larger", which is all the
    # class asserts.
    native.update(
        {name: str(int(rows[name]) + 8) for name in POINTER_OR_LANE_SENSITIVE}
    )
    native.update({name: rows[name] for name in SHAPE_ROWS})
    native[BACKEND_ROW] = BACKEND_SCALAR
    return actual, expected, native


def self_test() -> int:
    base = consistent_triple()
    validate(*base)

    def with_expected_row(name: str, value: str):
        def apply(triple: tuple[dict, dict, dict]) -> tuple:
            actual, expected, native = triple
            resource_rows(expected)[name] = value
            return actual, expected, native

        return apply

    def with_actual_row(name: str, value: str):
        def apply(triple: tuple[dict, dict, dict]) -> tuple:
            actual, expected, native = triple
            resource_rows(actual)[name] = value
            return actual, expected, native

        return apply

    def with_native_row(name: str, value: object):
        def apply(triple: tuple[dict, dict, dict]) -> tuple:
            actual, expected, native = triple
            native[name] = value
            return actual, expected, native

        return apply

    def native_equals_module(name: str):
        """Derived, never a literal: whatever the module reports, the native witness copies."""

        def apply(triple: tuple[dict, dict, dict]) -> tuple:
            actual, expected, native = triple
            native[name] = resource_rows(actual)[name]
            return actual, expected, native

        return apply

    def drop_expected_row(name: str):
        def apply(triple: tuple[dict, dict, dict]) -> tuple:
            actual, expected, native = triple
            del resource_rows(expected)[name]
            return actual, expected, native

        return apply

    def add_actual_row(name: str, value: str):
        def apply(triple: tuple[dict, dict, dict]) -> tuple:
            actual, expected, native = triple
            resource_rows(actual)[name] = value
            return actual, expected, native

        return apply

    def move_digest(leg: str):
        def apply(triple: tuple[dict, dict, dict]) -> tuple:
            actual, expected, native = triple
            actual[leg]["pcmF32leSha256"] = "0" * 64
            return actual, expected, native

        return apply

    def with_schema(value: str):
        def apply(triple: tuple[dict, dict, dict]) -> tuple:
            actual, expected, native = triple
            actual["schema"] = value
            return actual, expected, native

        return apply

    # `(name, mutation, classes)`; `classes` is `None` for the declared partition.
    mutations: list[tuple[str, object, object]] = [
        # The two historical staleness values this issue exists to end, exactly as they stood.
        (
            "expected.json still carries #216's builtinRetainedBytes",
            with_expected_row("builtinRetainedBytes", "722"),
            None,
        ),
        (
            "expected.json still carries #212's builtinRetainedBytes",
            with_expected_row("builtinRetainedBytes", "706"),
            None,
        ),
        (
            "expected.json still carries the pre-#216 graphSessionPlusPlanBytes",
            with_expected_row("graphSessionPlusPlanBytes", "20492"),
            None,
        ),
        (
            "expected.json still carries the pre-#212 graphSessionPlusPlanBytes",
            with_expected_row("graphSessionPlusPlanBytes", "20524"),
            None,
        ),
        (
            "expected.json still carries the pre-#216 graphMetadataBytes",
            with_expected_row("graphMetadataBytes", "3367"),
            None,
        ),
        (
            "expected.json still carries the pre-#216 graphIncrementalPlanBytes",
            with_expected_row("graphIncrementalPlanBytes", "18956"),
            None,
        ),
        # A row moving by one byte is the same defect class as a row moving by ten kilobytes.
        (
            "a target-independent row moves by one byte in the module",
            with_actual_row("outputPcmBytes", "1025"),
            None,
        ),
        (
            "the module grows a resource row expected.json does not pin",
            add_actual_row("stripStagingBytes", "4096"),
            None,
        ),
        ("expected.json drops a pinned row", drop_expected_row("graphDelayBytes"), None),
        # The native leg's own rules.
        (
            "a target-independent row disagrees between the two builds",
            with_native_row("sessionTomlBytes", "2097152"),
            None,
        ),
        (
            "a pointer-sensitive row now agrees between the two builds",
            native_equals_module("builtinRetainedBytes"),
            None,
        ),
        (
            "the native build claims to be the simd128 backend",
            with_native_row("backend", BACKEND_SIMD128),
            None,
        ),
        (
            "the fixture compiles at a different quantum in the two builds",
            with_native_row("quantumFrames", 256),
            None,
        ),
        # The digests, which this issue asserts did not move and which the gate must still watch.
        ("the identity-session digest moves", move_digest("simd128"), None),
        ("the command-timeline digest moves", move_digest("commandTimeline"), None),
        ("the observation-timeline digest moves", move_digest("observationTimeline"), None),
        ("the oracle prints a different schema", with_schema("miso.web.browser.v1"), None),
        # The partition itself.
        (
            "a row is classified in neither class",
            lambda triple: triple,
            (TARGET_INDEPENDENT - {"graphDelayBytes"}, POINTER_OR_LANE_SENSITIVE),
        ),
        (
            "a row is classified in both classes",
            lambda triple: triple,
            (TARGET_INDEPENDENT | {"graphMetadataBytes"}, POINTER_OR_LANE_SENSITIVE),
        ),
        (
            "a class names a row the report does not carry",
            lambda triple: triple,
            (TARGET_INDEPENDENT | {"stripStagingBytes"}, POINTER_OR_LANE_SENSITIVE),
        ),
        (
            "a drifting row is quietly reclassified as target-independent",
            lambda triple: triple,
            (
                TARGET_INDEPENDENT | {"builtinRetainedBytes"},
                POINTER_OR_LANE_SENSITIVE - {"builtinRetainedBytes"},
            ),
        ),
    ]

    failures = 0
    for name, apply, classes in mutations:
        triple = apply(consistent_triple())
        independent, sensitive = classes if classes is not None else (
            TARGET_INDEPENDENT,
            POINTER_OR_LANE_SENSITIVE,
        )
        try:
            validate(*triple, independent=independent, sensitive=sensitive)
        except Invalid:
            continue
        except Exception:  # noqa: BLE001 - a mutation that crashes the comparison still discriminates
            continue
        print(f"self-test FAILED: mutation escaped -- {name}", file=sys.stderr)
        failures += 1
    if failures == 0:
        print(
            "browser expected-resources self-test passed "
            f"({len(mutations)} red mutations)"
        )
    return failures


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--artifacts",
        type=pathlib.Path,
        help="a built worklet artifact directory to reuse instead of building the module",
    )
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    failed = 0
    try:
        validate(*collect(args.artifacts))
    except Invalid as error:
        print(f"FAIL browser expected resources: {error}", file=sys.stderr)
        failed = 1
    else:
        print(
            "browser-correctness expected.json resource rows and digests agree with the built "
            "simd128 module, and the native witness agrees where the rows are target-independent"
        )
    # Fail closed, and run second so a stale tree gets the drift diagnosis above before this.
    # Several red mutations restate a historical staleness value, so a tree that is *at* one of
    # those values makes that mutation a no-op and this reports the comparator cannot be trusted
    # -- which is the honest answer, and is red either way.
    return 1 if self_test() != 0 else failed


if __name__ == "__main__":
    raise SystemExit(main())
