Owner decision (2026-08-27): the move to the new dev machine (Ryzen 9 9950X) in the next day or two is the re-anchoring event — switch the measurement environment to the `performance` governor there and redo the benchmark baselines, rather than re-anchoring on the current 9700X.

## Why bundling is right
Absolute numbers in every sealed record are host-specific, so the machine change re-baselines the record families regardless; doing governor + host in one event avoids two discontinuities. Relative results (round-1/round-2 percentage wins, additivity checks) remain valid as expectations.

## Migration checklist
1. **Core reservation from day one** (replicate #195): pick the bench core + its SMT sibling on the 9950X — note it is dual-CCD (2×8 cores, per-CCD 32 MB L3): choose a core, pin its CCD, and document the choice in the record README. Apply the slice restrictions + the bench.slice scope wrapper; put tier 2 (`isolcpus=<pair> nohz_full rcu_nocbs`) straight into the kernel cmdline since the machine starts fresh.
2. **Governor**: `performance` on all cores including the bench pair (the new records' `governor_or_power_mode` field will say so).
3. **Re-derive the machine model** for floor accounting: the 3.7 vec-ops/cycle constant in `docs/rulings/effect-floor-accounting.md` is measured-per-host by design (probe source is in the ruling's appendix); re-run it on the 9950X and update the floor table + `tools/miso-engine-bench/src/floor.rs` in one commit. Fold in the pending max/min 2→1 lane-op re-pricing (lane-lowerings debt from #193) in the same recount so the new floors start correct.
4. **Re-baseline captures**: the standing intended-placement fixture, both legs (console + wasm-console), sealed as the new authority records; old records retire to history with a dated note per convention. Digest pins (output_sha256) must reproduce EXACTLY — outputs are machine-independent by the class-A/fma-contract design, and a digest mismatch on the new host is a stop-everything finding, not a re-pin.
5. **Verify the environment**: clock-drift guard behavior under performance (boost is stickier — spread should tighten), SMT-sibling idle check against the new numbering, cooldown timing.
6. Update agent conventions (CPU mutex path unchanged; core number in preconditions) and the project memory.

## Interim on the 9700X
Nothing re-anchors here: round-2's composed capture completes under the existing environment (powersave pair, comparable with all history) and closes the round; the diff chart is delivered from those records.
