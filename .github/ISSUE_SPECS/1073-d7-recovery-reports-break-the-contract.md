# D7 recovery reports break the contract (frames vs blocks; missing counts)

Found by #1051's randomized differential (`crates/conformance/src/randomized.rs`, "#1051 defect 5", fixed input).

## Problem

The D7 recovery's reports are inconsistent across effects: gate, multiband and soft-clip count frames where the contract counts blocks, and transient and limiter report no count at all. Hosts reading the report cannot compare effects or trust the number.

## Smallest closable slice

Make every effect's D7 report follow the contract's unit (blocks) and report its count; update the contract doc if the contract itself is ambiguous. Un-ignore the reproducer.

## Gates

1. The reproducer passes for every effect.
2. A planted frames-instead-of-blocks report turns it red.
3. Console digests unchanged.

## Evidence and decision record (attempt 1, 2026-10-23)

- Re-run on main 87b818f17 (`--ignored`): gate counted `[64, 0]` (frames); multiband and soft-clip
  `[64, 64]` (frames, both channels); transient shaper and limiter counted nothing. Compressor,
  parametric EQ and delay pass the reproducer (checked with a temporary call, not committed).
- Decision: a counter is per lane and per channel, one per failing block, on the channel whose own
  output failed. `ProcessReport` already says "one vector compare per channel per block" and the
  compressor, EQ and delay already count so; the contract text needed no change. A channel zeroed
  only beside its failed partner (multiband, soft-clip, shaper, limiter recover both) is not counted.
  A collapsed mono block counts both channels, as `process_bank_mono`'s contract requires.
- The reproducer was latency-blind: the limiter's output on the poisoned block is its prepared zero
  delay line, so it reported "zeroed and counts nothing" for the wrong reason. It now runs to two
  blocks after the poison reaches the output (block 7 for the limiter at its defaults) and checks
  there. Two checks were added: at most one block per channel over the run, and no right block in
  `DualMono` when only the left plane is poisoned.
- Pre-fix (new reproducer, fixes reverted): all five red; the limiter red at block 7. Post-fix: all
  five green. Rendered output is unchanged: only report counters moved.
