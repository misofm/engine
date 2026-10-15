# Remove the redundant span kind and end from prepared automation spans

Successor of #1636 (retire the span kinds and sample rate no producer uses). Code anchors verified
on the batch 13b fold branch `codex/controls-b13b-fold`. Issue number: assigned at filing (written
here as TBD33). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch
review.

## Product outcome

An effect's automation input says only what the engine delivers: one value for one parameter
channel at one sample. No field can describe a span shape that no producer makes, so no effect and
no test has to check one.

## Context

- **The struct.** `PreparedAutomationSpan` (`crates/effect-contract/src/lib.rs`, near
  `:1650-1658`) has `kind: AutomationSpanKind`, `channel`, `parameter_index`, `start_sample`,
  `end_sample`, `start_value` and `end_value`. It is a plain Rust struct (no `repr(C)`), not on a
  wire or the C ABI.
- **Only `Point` is left.** After #1636, `AutomationSpanKind` is `{Point=1}` (`lib.rs` near `:426`;
  `from_raw` refuses 2 to 4). `valid_runtime_span` (near `:1806-1834`) checks that a `Point` has
  `start_sample == end_sample` and equal value bits, so `kind`, `end_sample` and `end_value` carry no
  information: a valid span always repeats its start in its end.
- **Who builds and reads spans.** Live records stage only `Point` spans
  (`crates/effect-contract/src/live.rs`, near `:1315-1325`), and stored automation stages only
  `Point` spans (#1058 A1.6). Construction and read sites are in `crates/effect-contract`,
  `effect-compiler`, `rack`, `graph`, `graph-compiler`, `host-core`, `conformance`, each native
  effect crate (`compressor`, `delay`, `gate-expander`, `multiband-compressor`, `parametric-eq`,
  `soft-clip`, `transient-shaper`, `true-peak-limiter`), and `tools/{audit,console-workload,parameter-metadata}`.
  A test that exists only to check a mismatched end (the delay's malformed-span test as #1636 left
  it: a `Point` whose end sample differs from its start) has nothing left to check.

## Decisions frozen for this slice

- **D1. Remove the fields.** `PreparedAutomationSpan` loses `kind`, `end_sample` and `end_value`.
  `valid_runtime_span` loses the checks that read them. `validate_automation_block`'s ordering
  rule (near `:1869-1872`, which compares a span's start with the previous span's `end_sample` and
  `end_value`) reads the previous span's sample and value instead, with the same verdict for every
  valid `Point`.
- **D2. Rename what is left.** `start_sample` becomes `sample` and `start_value` becomes `value`
  (a start names nothing without an end). The rename is mechanical.
- **D3. The kind enum.** `AutomationSpanKind` (and its alias `AutomationKind`) is deleted when no
  other type reads it; its retired raw values stay documented as retired where #1636 put that note.
- **D4. Tests.** A test that exists only for a removed field is deleted in the same PR; a fixture
  that sets the removed fields drops them.

## Deliverables

1. D1 to D4 across the construction and read sites named in Context.
2. `docs/EFFECT_CONTRACT_V1.md`: the span description names one sample and one value.

## Authorized paths

- `crates/effect-contract/src/`, `crates/effect-contract/tests/`
- The construction and read sites only, in `crates/{effect-compiler,rack,graph,graph-compiler,host-core,conformance}`
  and the eight native effect crates (sources and tests)
- `tools/{audit,console-workload,parameter-metadata}` (span construction only)
- `docs/EFFECT_CONTRACT_V1.md`

## Non-goals

- Any change to how an effect applies a `Point` (smoothing, ramps, counters).
- New span kinds.

## Hazards

- **Retained bytes.** The struct shrinks, so a staging window's bytes shrink. Ceilings stay; a test
  that pins an exact byte count of a span window is re-pinned with the reason
  "TBD33 removes the span kind and end".

## Objective gates

1. **No rendered bit moves.** `cargo build --locked --release -p audit -p capi &&
   ./target/release/audit capi` reports the same `pcm_digest` at base and head (PR evidence), and
   the browser legs pass with unchanged digests.
2. **Contract.** The effect-contract truth table for `valid_runtime_span` keeps every case that
   still has a field to read (non-finite value, sample outside the block, value outside the
   domain, channel against the parameter's channel policy), and `validate_automation_block`
   gives the same verdicts on its ordering cases (spans out of order; two spans at one sample with
   different values).
3. Commands: `cargo test --locked --workspace` (or the `test-debug-a` and `test-debug-b` legs),
   the randomized conformance run, `bash scripts/check-effect-runtime-policy.sh`,
   `bash scripts/check-workspace-policy.sh`, clippy `-D warnings`, `fmt --check`.

## Test value

- Gate 1: red if removing the fields changed what an effect applies.
- Gate 2: red if a check on a remaining field was dropped with the removed ones, or if the ordering
  rule changed its verdict when it lost the end fields.

## Dependencies

- #1636 (batch 13b).
