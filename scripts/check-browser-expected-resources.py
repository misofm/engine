#!/usr/bin/env python3
"""The browser identity fixture's resource budgets and PCM digests (issues #217 and #1060).

`hosts/host-web/tests/browser-v1/expected.json` records the shipped `wasm32-unknown-unknown`
simd128 worklet module's resource report beside its PCM digests. This gate builds the module with
the shipped builder, `scripts/build-web-audioworklet.sh --module-only` (or takes a built artifact
directory, as CI does), runs the fixture's own `direct-oracle.mjs` under Node in
`MISO_ENGINE_WEB_ORACLE_PRINT=1` mode -- no browser, no WebDriver, no audio device -- and checks
the printed document against `expected.json`.

**History.** #217 made this a staleness tripwire: every resource row was pinned to the byte,
and a native build of the same fixture (`examples/browser_fixture_resources.rs`, deleted by
#1060) was a second witness for the rows declared target-independent. The pins went red whenever
a layout moved -- the rows were re-pinned 24 times in September 2026 and none of the reds exposed
an unintended change (#1060) -- and the native witness cost about 60 s of every full PR.

**What this gate is now (#1060, owner decision 4: budgets, not exact counts).** Each printed
resource row belongs to exactly one class:

* `EXACT_ROWS` are contract facts, not layout: the boot options and status structures, the staged
  document, the configured diagnostic, ID and PCM staging capacities, the two largest-allocation
  rows (262,144 bytes, the default control frame since #2191; it was 16,384 after #1590 deleted the 24,576-byte observation staging), the four rows this identity fixture drives to zero (no delay, no scalar effect, no
  tap-declaring effect), and the compile's shape and backend. They must equal `expected.json`.
* `CEILING_ROWS` are retained production memory: `size_of` sums over pointer-bearing structures
  and AoSoA bank payloads. Each must be positive and at most its ceiling in `expected.json`'s
  `directOracle.simd128.resourceCeilings`. A ceiling is the row's value plus 10 %, rounded up to
  64 bytes; growth past
  it is red and raises the budget with its reason. The gate prints each row's headroom as a live
  report. A zero row is red too, because a row that stops being charged is a decrease a ceiling
  cannot see; that the rows charge every allocation is capi's allocator oracle
  (`crates/capi/tests/resource_lifecycle.rs`).

The status snapshots' `memoryBytes` is the one status field that is a size (#1691, which carries
#1720's D1): the oracle reads it from `exports.memory.buffer.byteLength`, not from the status
structure, whose seven words are contract facts and stay exact. Linear memory grows by whole 64 KiB
pages and never shrinks, so the four snapshots that carry it (`STATUS_MEMORY_PATHS`) share one
ceiling, `directOracle.memoryBytesCeiling`: each printed value must be positive, a whole page and
at most the ceiling, and the gate prints its headroom in pages. The ceiling is the measured peak
plus two pages, raised only with its reason. `expected.json` carries no `memoryBytes` (the oracle
prints them; the file carries the budget), and a `memoryBytes` anywhere else in the printed
document is red until it is classified here.

Everything else in the printed document -- the rest of the status snapshots, the result
transcripts, the SET_CONTROL, observation and automation timelines and the four PCM digests, which
`direct-oracle.mjs` has already asserted equal to the native digests before printing -- must equal
`expected.json` exactly.

`--self-test` proves the comparator discriminates before it is trusted: it fabricates a green
document at the ceilings, and then requires every red mutation below -- one byte over a ceiling,
a zeroed row, an unclassified row, a moved exact row or digest -- to be caught. It builds nothing
and is instant, so the gate runs it on every invocation.
"""

from __future__ import annotations

import argparse
import json
import os
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile

REPO = pathlib.Path(__file__).resolve().parent.parent
FIXTURE = REPO / "hosts/host-web/tests/browser-v1"
EXPECTED_JSON = FIXTURE / "expected.json"
DIRECT_ORACLE = FIXTURE / "direct-oracle.mjs"
DELIVERY_SCRIPT = REPO / "scripts/build-web-audioworklet.sh"
WORKLET_JS = REPO / "hosts/host-web/web/miso-engine-v1-audio-worklet.js"
FIXTURE_RUNNER_JS = FIXTURE / "browser-correctness.js"

MODULE_NAME = "miso-engine-v1-audio-worklet.simd128.wasm"

DIRECT_ORACLE_SCHEMA = "miso.web.browser.direct-oracle.v1"
BACKEND_SIMD128 = 1

# Contract facts, not layout: the boot options and status structure sizes, the staged document,
# the configured diagnostic, ID and PCM staging capacities, the two largest-allocation rows, the
# four rows this identity fixture drives to zero, and the
# compile's shape and backend. Each must equal `expected.json` exactly.
EXACT_ROWS = frozenset(
    {
        "sampleRateHz",
        "quantumFrames",
        "backend",
        "optionsBytes",
        "statusBytes",
        "sessionDocumentBytes",
        "diagnosticBytes",
        "idStagingBytes",
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

# Retained production memory: `size_of` sums over pointer-bearing structures and AoSoA bank
# payloads, which move with every layout change. Each is budgeted, not pinned (#1060): positive,
# and at most its ceiling in `directOracle.simd128.resourceCeilings`.
CEILING_ROWS = frozenset(
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

CEILINGS_KEY = "resourceCeilings"

# The status snapshots that carry the module's linear memory size,
# `exports.memory.buffer.byteLength` (#1691). One module, and a Wasm memory never shrinks, so all
# four share one ceiling.
STATUS_MEMORY_PATHS = (
    ("simd128", "initialStatus"),
    ("simd128", "beforeDisposeStatus"),
    ("setControlTimeline", "beforeDisposeStatus"),
    ("automationTimeline", "beforeDisposeStatus"),
)
MEMORY_KEY = "memoryBytes"
MEMORY_CEILING_KEY = "memoryBytesCeiling"
WASM_PAGE_BYTES = 65536

# The four digests `expected.json` pins. Named here so a report that stops carrying one is a
# failure rather than a silently skipped comparison.
DIGEST_PATHS = (
    ("simd128", "pcmF32leSha256"),
    ("setControlTimeline", "pcmF32leSha256"),
    ("observationTimeline", "pcmF32leSha256"),
    ("automationTimeline", "pcmF32leSha256"),
)


class Invalid(Exception):
    """One rule this gate enforces was broken."""


def require(condition: object, message: str) -> None:
    if not condition:
        raise Invalid(message)


def balanced(text: str, start: int, opening: str, closing: str) -> str:
    """Return `text[start:]` up to and including the bracket that closes the one at `start`."""
    require(text[start] == opening, f"expected {opening!r} at offset {start}")
    depth = 0
    for index in range(start, len(text)):
        if text[index] == opening:
            depth += 1
        elif text[index] == closing:
            depth -= 1
            if depth == 0:
                return text[start : index + 1]
    raise Invalid(f"unbalanced {opening!r} from offset {start}")


def block_after(text: str, anchor: str, opening: str, closing: str) -> str:
    """The bracketed block that follows `anchor`."""
    at = text.find(anchor)
    require(at >= 0, f"anchor not found: {anchor!r}")
    start = text.index(opening, at + len(anchor) - 1)
    return balanced(text, start, opening, closing)


def simd128_leg(document: dict, owner: str) -> dict:
    leg = document.get("simd128")
    require(isinstance(leg, dict), f"{owner} has no simd128 leg")
    return leg


def resource_rows(document: dict, owner: str) -> dict:
    """The `simd128.resources` block of a `directOracle`-shaped document."""
    rows = simd128_leg(document, owner).get("resources")
    require(isinstance(rows, dict), f"{owner}'s simd128 leg has no resources block")
    return rows


def check_partition(rows: set, exact: frozenset, ceiling: frozenset) -> None:
    """Every printed row is classified exactly once, and the classes name nothing else."""
    both = exact & ceiling
    require(not both, f"rows declared both exact and budgeted: {sorted(both)}")
    unclassified = rows - (exact | ceiling)
    require(
        not unclassified,
        "the resource report carries rows this gate does not classify, so nothing would check "
        f"them: {sorted(unclassified)}",
    )
    absent = (exact | ceiling) - rows
    require(
        not absent,
        f"this gate classifies rows the resource report does not carry: {sorted(absent)}",
    )


def as_bytes(value: object, name: str) -> int:
    require(
        isinstance(value, str) and value.isdigit(),
        f"{name} is not a decimal byte count: {value!r}",
    )
    return int(value)


def check_ceilings(actual_rows: dict, ceilings: object, ceiling_rows: frozenset) -> list[str]:
    """Each budgeted row is positive and within its ceiling; returns the live headroom report."""
    require(isinstance(ceilings, dict), f"expected.json's simd128 leg has no {CEILINGS_KEY}")
    missing = sorted(set(ceiling_rows) - set(ceilings))
    extra = sorted(set(ceilings) - set(ceiling_rows))
    require(
        not missing and not extra,
        f"{CEILINGS_KEY} must budget exactly the retained rows: missing {missing}, extra {extra}",
    )
    report = []
    over = []
    for name in sorted(ceiling_rows):
        value = as_bytes(actual_rows[name], name)
        ceiling = as_bytes(ceilings[name], f"{CEILINGS_KEY}.{name}")
        require(
            value > 0,
            f"{name} is zero: a retained row that stops being charged is a decrease no ceiling "
            "can see",
        )
        report.append(f"{name} {value} of {ceiling} ({ceiling - value} bytes free)")
        if value > ceiling:
            over.append(f"{name} {value} > {ceiling}")
    require(
        not over,
        "retained rows over their budget in the built module: "
        + ", ".join(over)
        + ". Raise the budget in expected.json's resourceCeilings with the reason in the commit "
        "message, or find the regression.",
    )
    return report


def memory_keys(value: object, at: str = "") -> list[str]:
    """Every path in `value` that carries a `memoryBytes` key."""
    found = []
    if isinstance(value, dict):
        for key, child in value.items():
            path = f"{at}.{key}" if at else key
            if key == MEMORY_KEY:
                found.append(path)
            found.extend(memory_keys(child, path))
    elif isinstance(value, list):
        for index, child in enumerate(value):
            found.extend(memory_keys(child, f"{at}[{index}]"))
    return found


def page_count(value: object, name: str) -> int:
    """A positive whole number of Wasm pages, in bytes."""
    require(
        isinstance(value, int) and not isinstance(value, bool) and value > 0,
        f"{name} is not a positive byte count: {value!r}",
    )
    require(
        value % WASM_PAGE_BYTES == 0,
        f"{name} {value} is not a whole number of {WASM_PAGE_BYTES}-byte Wasm pages",
    )
    return value


def check_status_memory(actual: dict, expected: dict) -> list[str]:
    """Each status snapshot's memory size is a positive page multiple within the one ceiling, and
    neither document carries a `memoryBytes` this gate does not budget. Strips the budgeted keys
    from `actual` and the ceiling from `expected`, both comparable copies; returns the report."""
    require(
        MEMORY_CEILING_KEY in expected,
        f"expected.json's directOracle carries no {MEMORY_CEILING_KEY}, so memory has no budget",
    )
    ceiling = page_count(expected.pop(MEMORY_CEILING_KEY), MEMORY_CEILING_KEY)
    # This check and the unclassified-key check below diagnose; the final exact comparison is
    # the backstop that would also turn either case red.
    pinned = memory_keys(expected)
    require(
        not pinned,
        f"expected.json pins {MEMORY_KEY} exactly at {pinned}; it is a size, budgeted by "
        f"{MEMORY_CEILING_KEY}, not a pin",
    )
    report = []
    over = []
    for leg, snapshot in STATUS_MEMORY_PATHS:
        name = f"{leg}.{snapshot}"
        status = actual.get(leg, {}).get(snapshot)
        require(
            isinstance(status, dict) and MEMORY_KEY in status,
            f"the report carries no {MEMORY_KEY} at {name}",
        )
        value = page_count(status.pop(MEMORY_KEY), f"{MEMORY_KEY} {name}")
        report.append(
            f"{MEMORY_KEY} {name} {value} of {ceiling} "
            f"({(ceiling - value) // WASM_PAGE_BYTES} pages free)"
        )
        if value > ceiling:
            over.append(f"{name} {value} > {ceiling}")
    require(
        not over,
        f"the module's linear memory is over its budget: {', '.join(over)}. Raise "
        f"{MEMORY_CEILING_KEY} in expected.json with the reason in the commit message, or find "
        "the regression.",
    )
    unclassified = memory_keys(actual)
    require(
        not unclassified,
        f"the report carries {MEMORY_KEY} at {unclassified}, which this gate does not budget",
    )
    return report


def check_document(
    actual: dict,
    expected: dict,
    exact: frozenset = EXACT_ROWS,
    ceiling: frozenset = CEILING_ROWS,
) -> list[str]:
    """The printed oracle document against `expected.json`: exact where it is a fact, budgeted
    where it is retained memory. Returns the budget report."""
    require(
        actual.get("schema") == DIRECT_ORACLE_SCHEMA,
        f"the oracle printed schema {actual.get('schema')!r}, not {DIRECT_ORACLE_SCHEMA!r}",
    )
    actual_rows = resource_rows(actual, "the printed oracle document")
    expected_rows = resource_rows(expected, "expected.json")
    check_partition(set(actual_rows), exact, ceiling)
    missing = sorted(set(exact) - set(expected_rows))
    extra = sorted(set(expected_rows) - set(exact))
    require(
        not missing and not extra,
        f"expected.json's resources must pin exactly the exact rows: missing {missing}, "
        f"extra {extra}",
    )
    moved = {
        name: (expected_rows[name], actual_rows[name])
        for name in sorted(exact)
        if expected_rows[name] != actual_rows[name]
    }
    require(
        not moved,
        "exact resource rows moved in the built module (expected -> actual): "
        + ", ".join(f"{name} {pin} -> {live}" for name, (pin, live) in moved.items()),
    )
    report = check_ceilings(
        actual_rows, simd128_leg(expected, "expected.json").get(CEILINGS_KEY), ceiling
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
    # Everything that is neither a budgeted row nor the budgets themselves is exact.
    comparable_actual = json.loads(json.dumps(actual))
    comparable_expected = json.loads(json.dumps(expected))
    for name in ceiling:
        del resource_rows(comparable_actual, "the printed oracle document")[name]
    del simd128_leg(comparable_expected, "expected.json")[CEILINGS_KEY]
    report += check_status_memory(comparable_actual, comparable_expected)
    require(
        comparable_actual == comparable_expected,
        "the printed oracle document differs from expected.json outside the budgeted rows",
    )
    return report


def worklet_limit_fields(text: str) -> list[str]:
    """The `exactFields` vocabulary the worklet refuses an `options` object against."""
    block = block_after(text, "const OPTION_FIELDS = ", "[", "]")
    names = re.findall(r'"([A-Za-z][A-Za-z0-9]*)"', block)
    require(names, "the worklet declares no OPTION_FIELDS")
    return sorted(names)


def fixture_limit_fields(text: str) -> list[str]:
    """The keys the browser fixture's `bootOptions()` actually builds."""
    block = block_after(text, "function bootOptions() {\n  return ", "{", "}")
    names = re.findall(r"^\s{4}([A-Za-z][A-Za-z0-9]*):", block, re.MULTILINE)
    require(names, "the browser fixture's bootOptions() builds no keys")
    return sorted(names)


def check_limits_vocabulary(worklet: str, fixture: str) -> None:
    """The fixture's boot `options` object and the worklet's guard are one vocabulary.

    The same staleness class as the resource rows, on the other half of the fixture, and it had
    already bitten: issue #143 carved the observation-taps word (retired by issue #1590; offset 48 is
    `maximumDocumentBytes` since issue #2191)
    and the master word (now `reserved2`, issue #1689) out of the configuration's last two reserved words and added
    both to the old guard, and the fixture's policy object was never extended. The field list is
    checked with `exactFields`, so the browser leg refused the fixture at boot with
    `RESULT_INVALID_ARGUMENT` from #143 until #217.
    Nothing was red: the `--check` leg drives the module through `direct-oracle.mjs`, which writes
    the configuration words itself and never crosses `miso-engine-v1-audio-worklet.js`, and the
    browser leg is not a sweep row because its sibling modes need a browser.
    """
    declared = worklet_limit_fields(worklet)
    supplied = fixture_limit_fields(fixture)
    missing = [name for name in declared if name not in supplied]
    extra = [name for name in supplied if name not in declared]
    require(
        not missing,
        "the browser fixture's bootOptions() is missing policy words the worklet's "
        f"exactFields guard requires, so the browser leg cannot boot: {missing}",
    )
    require(
        not extra,
        "the browser fixture's bootOptions() supplies words the worklet's exactFields guard does not "
        f"declare, so the browser leg cannot boot: {extra}",
    )


def validate(
    actual: dict,
    expected: dict,
    worklet: str,
    fixture: str,
    exact: frozenset = EXACT_ROWS,
    ceiling: frozenset = CEILING_ROWS,
) -> list[str]:
    check_limits_vocabulary(worklet, fixture)
    report = check_document(actual, expected, exact, ceiling)
    require(
        int(resource_rows(actual, "the printed oracle document")["backend"]) == BACKEND_SIMD128,
        "the module does not report BACKEND_SIMD128; it was not built with +simd128, so its lane "
        "width is not the browser's",
    )
    return report


# --- collection -----------------------------------------------------------------------------


def build_module(destination: pathlib.Path) -> None:
    """Build the shipped simd128 module into the empty `destination` with the shipped builder.

    `build-web-audioworklet.sh --module-only` is the module's one cargo line (the pinned nightly
    with `-Zbuild-std`, #1334) and owns its target directory, so this gate reads the bytes the
    delivery build ships, under the name the oracle expects (#2046).
    """
    subprocess.run(
        ["bash", str(DELIVERY_SCRIPT), "--module-only", str(destination)],
        cwd=REPO,
        check=True,
    )
    require(
        (destination / MODULE_NAME).is_file(),
        f"{DELIVERY_SCRIPT.name} --module-only did not write {MODULE_NAME}",
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


def collect(artifacts: pathlib.Path | None) -> tuple[dict, dict, str, str]:
    expected = json.loads(EXPECTED_JSON.read_text())["directOracle"]
    worklet = WORKLET_JS.read_text()
    fixture = FIXTURE_RUNNER_JS.read_text()
    if artifacts is not None:
        # The shipped worklet JS is the one the browser leg would actually load.
        worklet = (artifacts / WORKLET_JS.name).read_text()
        actual = print_oracle(artifacts.resolve())
    else:
        with tempfile.TemporaryDirectory() as staging:
            module_directory = pathlib.Path(staging)
            build_module(module_directory)
            actual = print_oracle(module_directory)
    return actual, expected, worklet, fixture


# --- the self-test --------------------------------------------------------------------------


def green_pair() -> tuple[dict, dict, str, str]:
    """A printed document and the committed `expected.json` it satisfies, at the boundary.

    Built from the committed `expected.json`, so the self-test's green case is the real document
    and its mutations are the real rows. Every budgeted row is printed at exactly its ceiling: the
    green case is the edge of the budget, so "one byte over" below is the other side of it.
    Each status snapshot prints `memoryBytes` at the memory ceiling, for the same reason.
    """
    expected = json.loads(EXPECTED_JSON.read_text())["directOracle"]
    actual = json.loads(json.dumps(expected))
    leg = simd128_leg(actual, "the fabricated document")
    ceilings = leg.pop(CEILINGS_KEY)
    leg["resources"].update(ceilings)
    memory_ceiling = actual.pop(MEMORY_CEILING_KEY)
    for status_leg, snapshot in STATUS_MEMORY_PATHS:
        actual[status_leg][snapshot][MEMORY_KEY] = memory_ceiling
    return actual, expected, WORKLET_JS.read_text(), FIXTURE_RUNNER_JS.read_text()


def self_test() -> int:
    base = green_pair()
    try:
        validate(*base)
    except Invalid as error:
        # The mutations are differences from a green tree, so a red tree cannot be a baseline.
        # The real diagnosis is printed by the comparison run; this says only that the self-test
        # could not be performed, which is itself red.
        print(
            f"self-test cannot run: the tree is already red -- {error}",
            file=sys.stderr,
        )
        return 1

    def printed_row(name: str, value: str):
        def apply(pair: tuple) -> tuple:
            actual, expected, worklet, fixture = pair
            resource_rows(actual, "the fabricated document")[name] = value
            return actual, expected, worklet, fixture

        return apply

    def one_byte_over(name: str):
        """Derived, never a literal: the row's own ceiling plus one."""

        def apply(pair: tuple) -> tuple:
            actual, expected, worklet, fixture = pair
            ceiling = int(simd128_leg(expected, "expected.json")[CEILINGS_KEY][name])
            resource_rows(actual, "the fabricated document")[name] = str(ceiling + 1)
            return actual, expected, worklet, fixture

        return apply

    def drop_ceiling(name: str):
        def apply(pair: tuple) -> tuple:
            actual, expected, worklet, fixture = pair
            del simd128_leg(expected, "expected.json")[CEILINGS_KEY][name]
            return actual, expected, worklet, fixture

        return apply

    def add_ceiling(name: str):
        def apply(pair: tuple) -> tuple:
            actual, expected, worklet, fixture = pair
            simd128_leg(expected, "expected.json")[CEILINGS_KEY][name] = "4096"
            return actual, expected, worklet, fixture

        return apply

    def drop_expected_row(name: str):
        def apply(pair: tuple) -> tuple:
            actual, expected, worklet, fixture = pair
            del resource_rows(expected, "expected.json")[name]
            return actual, expected, worklet, fixture

        return apply

    def drop_fixture_limit(name: str):
        """Derived, never a literal: the whole `name: <whatever>,` line the fixture writes."""

        def apply(pair: tuple) -> tuple:
            actual, expected, worklet, fixture = pair
            pattern = rf"\n    {name}: [^\n]*,"
            require(
                re.search(pattern, fixture) is not None,
                f"self-test mutation matched nothing: {name}",
            )
            return actual, expected, worklet, re.sub(pattern, "", fixture, count=1)

        return apply

    def add_fixture_limit(name: str):
        def apply(pair: tuple) -> tuple:
            actual, expected, worklet, fixture = pair
            anchor = "\n    reserved2: 0n,"
            require(anchor in fixture, "self-test mutation matched nothing")
            return actual, expected, worklet, fixture.replace(
                anchor, f"{anchor}\n    {name}: 0n,", 1
            )

        return apply

    def add_worklet_limit(name: str):
        def apply(pair: tuple) -> tuple:
            actual, expected, worklet, fixture = pair
            anchor = '"maximumDocumentBytes", "reserved2",'
            require(anchor in worklet, "self-test mutation matched nothing")
            return actual, expected, worklet.replace(
                anchor, f'{anchor} "{name}",', 1
            ), fixture

        return apply

    def move_digest(leg: str):
        def apply(pair: tuple) -> tuple:
            actual, expected, worklet, fixture = pair
            actual[leg]["pcmF32leSha256"] = "0" * 64
            return actual, expected, worklet, fixture

        return apply

    def move_status(field: str):
        def apply(pair: tuple) -> tuple:
            actual, expected, worklet, fixture = pair
            status = simd128_leg(actual, "the fabricated document")["beforeDisposeStatus"]
            status[field] = str(int(status[field]) + 128)
            return actual, expected, worklet, fixture

        return apply

    def printed_memory(leg: str, snapshot: str, offset: int):
        """Derived, never a literal: the memory ceiling plus `offset` bytes, at one snapshot."""

        def apply(pair: tuple) -> tuple:
            actual, expected, worklet, fixture = pair
            actual[leg][snapshot][MEMORY_KEY] = expected[MEMORY_CEILING_KEY] + offset
            return actual, expected, worklet, fixture

        return apply

    def zero_memory(pair: tuple) -> tuple:
        actual, expected, worklet, fixture = pair
        actual["simd128"]["beforeDisposeStatus"][MEMORY_KEY] = 0
        return actual, expected, worklet, fixture

    def repin_memory(pair: tuple) -> tuple:
        actual, expected, worklet, fixture = pair
        status = expected["automationTimeline"]["beforeDisposeStatus"]
        status[MEMORY_KEY] = actual["automationTimeline"]["beforeDisposeStatus"][MEMORY_KEY]
        return actual, expected, worklet, fixture

    def drop_memory_ceiling(pair: tuple) -> tuple:
        actual, expected, worklet, fixture = pair
        del expected[MEMORY_CEILING_KEY]
        return actual, expected, worklet, fixture

    def leg_memory_ceiling(pair: tuple) -> tuple:
        actual, expected, worklet, fixture = pair
        expected["automationTimeline"][MEMORY_CEILING_KEY] = expected[MEMORY_CEILING_KEY]
        return actual, expected, worklet, fixture

    def unbudgeted_memory(pair: tuple) -> tuple:
        actual, expected, worklet, fixture = pair
        actual["observationTimeline"][MEMORY_KEY] = expected[MEMORY_CEILING_KEY]
        return actual, expected, worklet, fixture

    def with_schema(value: str):
        def apply(pair: tuple) -> tuple:
            actual, expected, worklet, fixture = pair
            actual["schema"] = value
            return actual, expected, worklet, fixture

        return apply

    # `(name, mutation, classes)`; `classes` is `None` for the declared partition.
    mutations: list[tuple[str, object, object]] = [
        # The budgets: one byte over each ceiling is red, whichever row it is.
        *(
            (f"{name} is one byte over its ceiling", one_byte_over(name), None)
            for name in sorted(CEILING_ROWS)
        ),
        # A decrease no ceiling can see: a retained row that stops being charged.
        ("a retained row drops to zero", printed_row("builtinRetainedBytes", "0"), None),
        ("a retained row is not a byte count", printed_row("graphMetadataBytes", "-1"), None),
        ("expected.json stops budgeting a retained row", drop_ceiling("graphMetadataBytes"), None),
        ("expected.json budgets a row that is not retained", add_ceiling("outputPcmBytes"), None),
        # The linear memory budget: one page over the shared ceiling is red at every snapshot.
        *(
            (
                f"memoryBytes {leg}.{snapshot} is one page over its ceiling",
                printed_memory(leg, snapshot, WASM_PAGE_BYTES),
                None,
            )
            for leg, snapshot in STATUS_MEMORY_PATHS
        ),
        ("a status snapshot reports zero memory", zero_memory, None),
        (
            "a status snapshot reports a memory size that is not a whole page",
            printed_memory("simd128", "initialStatus", -1),
            None,
        ),
        ("expected.json pins memoryBytes exactly again", repin_memory, None),
        ("expected.json carries no memory ceiling", drop_memory_ceiling, None),
        ("expected.json carries a second memory ceiling", leg_memory_ceiling, None),
        ("a memoryBytes outside the budgeted snapshots", unbudgeted_memory, None),
        # The exact rows: a fact moving by one byte is red.
        (
            "a staging capacity moves by one byte in the module",
            printed_row("outputPcmBytes", "1025"),
            None,
        ),
        (
            "the staged document size moves by one byte",
            printed_row("sessionDocumentBytes", "2022"),
            None,
        ),
        ("a zero-by-fixture row becomes nonzero", printed_row("graphDelayBytes", "8"), None),
        ("expected.json drops an exact row", drop_expected_row("graphDelayBytes"), None),
        (
            "the module grows a resource row nothing classifies",
            printed_row("stripStagingBytes", "4096"),
            None,
        ),
        ("the module is not the simd128 backend", printed_row("backend", "0"), None),
        # The digests and the transcript, which this gate asserts did not move.
        ("the identity-session digest moves", move_digest("simd128"), None),
        ("the SET_CONTROL-timeline digest moves", move_digest("setControlTimeline"), None),
        ("the observation-timeline digest moves", move_digest("observationTimeline"), None),
        ("the automation-timeline digest moves", move_digest("automationTimeline"), None),
        ("the render transcript moves", move_status("nextAbsoluteSample"), None),
        ("the oracle prints a different schema", with_schema("miso.web.browser.v1"), None),
        # The boot-options vocabulary, and the exact #143 regression this fixture shipped with.
        (
            "the fixture's bootOptions() drops the #2191 document word (the #143 regression class)",
            drop_fixture_limit("maximumDocumentBytes"),
            None,
        ),
        (
            "the fixture's bootOptions() drops the #1689 reserved2 word",
            drop_fixture_limit("reserved2"),
            None,
        ),
        (
            "the fixture's bootOptions() drops a word that predates #143",
            drop_fixture_limit("sourceRingFrames"),
            None,
        ),
        (
            "the worklet declares a word the fixture does not supply",
            add_worklet_limit("liveControlAuxPlaneCount"),
            None,
        ),
        (
            "the fixture supplies a word the worklet does not declare",
            add_fixture_limit("liveControlAuxPlaneCount"),
            None,
        ),
        # The partition itself.
        (
            "a row is classified in neither class",
            lambda pair: pair,
            (EXACT_ROWS - {"graphDelayBytes"}, CEILING_ROWS),
        ),
        (
            "a row is classified in both classes",
            lambda pair: pair,
            (EXACT_ROWS | {"graphMetadataBytes"}, CEILING_ROWS),
        ),
        (
            "a class names a row the report does not carry",
            lambda pair: pair,
            (EXACT_ROWS | {"stripStagingBytes"}, CEILING_ROWS),
        ),
        (
            "a budgeted row is quietly reclassified as exact",
            lambda pair: pair,
            (EXACT_ROWS | {"builtinRetainedBytes"}, CEILING_ROWS - {"builtinRetainedBytes"}),
        ),
    ]

    failures = 0
    for name, apply, classes in mutations:
        pair = apply(green_pair())
        exact, ceiling = classes if classes is not None else (EXACT_ROWS, CEILING_ROWS)
        try:
            validate(*pair, exact=exact, ceiling=ceiling)  # noqa: B026
        except Invalid:
            continue
        except Exception:  # noqa: BLE001 - a mutation that crashes the comparison still discriminates
            continue
        print(f"self-test FAILED: mutation escaped -- {name}", file=sys.stderr)
        failures += 1
    if failures == 0:
        print(
            "browser expected-resources self-test passed "
            f"(green at every ceiling; {len(mutations)} red mutations)"
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
        report = validate(*collect(args.artifacts))
    except Invalid as error:
        print(f"FAIL browser expected resources: {error}", file=sys.stderr)
        failed = 1
    else:
        for line in report:
            print(f"budget: {line}")
        print(
            "browser-correctness expected.json digests and exact rows agree with the built simd128 "
            "module, and every retained row is within its budget"
        )
    # Fail closed, and run second so a red tree gets its own diagnosis above before this.
    return 1 if self_test() != 0 else failed


if __name__ == "__main__":
    raise SystemExit(main())
