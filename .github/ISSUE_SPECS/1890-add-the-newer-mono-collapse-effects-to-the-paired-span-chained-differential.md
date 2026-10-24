# Add the newer mono-collapse effects to the paired-span chained differential

Filed by root from #1872's scope note (backlog sprint, 2026-10-24).

## Product outcome

`tools/console-workload/tests/paired_spans.rs` Part 1 runs a chained differential over its `Fx` effects (the collapsed chain against the dual chain, span by span). #973 (gate-expander), #1871 (multiband compressor), #1872 (soft clip) and #1873 (transient shaper) each added only their Part 2 A3 per-effect obligation. None of them is an `Fx` of Part 1, so no chained run proves their mono body inside a chain beside other collapsed effects.

## Smallest closable slice

Add the gate-expander, multiband compressor, soft clip and transient shaper to Part 1's `Fx` (with their effect-specific `scenarios()`), once #1871 and #1873 have landed. One run per effect in each link mode the effect supports. No new test function: extend the existing table.

## Gates

`cargo test --locked --release -p console-workload --test paired_spans`; each new row is red if its effect's collapsed body diverges from the dual body inside a chain (prove with one mutant per effect).

## Paths

`tools/console-workload/tests/paired_spans.rs`, this spec. Blocked by #1871 and #1873.
