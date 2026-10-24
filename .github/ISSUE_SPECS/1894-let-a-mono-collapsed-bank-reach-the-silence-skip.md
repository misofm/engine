# Let a mono-collapsed bank reach the silence skip

Filed by root from the #1871 verifier's finding (backlog sprint, 2026-10-24).

## Product outcome

While a bank lane is mono-collapsed, its right side is frozen. The bank's `lanes_at_rest` takes `sides[0].rest_lanes() & sides[1].rest_lanes()` (`crates/multiband-compressor/src/lib.rs` near line 1941; the gate-expander has the same shape), and `advance_clocks` asserts the same AND. So during a collapsed run the rack-level silence skip fires only if the right side was already at rest when the collapse began. The answer is a subset of the true one, so it is sound, but a collapsed lane that falls silent keeps processing (memory: skip work on silence).

## Smallest closable slice

1. In every mono-collapse effect (gate-expander, multiband, soft clip, transient shaper and any other on the mono-collapse contract), answer `lanes_at_rest` and advance the clocks from side 0 alone for a collapsed lane, and keep the disengage copy as the one place side 1 is made equal again.
2. State the rule once in the shared mono-collapse contract doc in `crates/effect-contract`.
3. One test in the owning crate of the shared contract (or one per effect if the logic is per effect): a collapsed lane that falls silent is reported at rest and skipped, and on disengage it renders the never-collapsed bits.

## Order

After #1871, #1872 and #1873 merge (same files).

## Paths

The mono-collapse effects' `src/lib.rs` and tests, `crates/effect-contract/src/lib.rs` (doc), this spec.
