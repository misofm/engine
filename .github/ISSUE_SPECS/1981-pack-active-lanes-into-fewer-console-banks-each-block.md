# Pack active lanes into fewer console banks each block

Filed by root as #1782's second slice (backlog sprint, 2026-10-24). #1782a builds the lane map, the move and the fold on a displaced group; this issue is the per-block packing decision that uses them.

## Smallest closable slice

- After the drains, per group: a lane is packable when its input is exact `+0.0` on both planes
  (the per-lane `block_is_positive_zero` head) and `lanes_at_rest` covers it and it has idled `R`
  blocks. With `A` active lanes run the first `K = ceil(A/W)` units; an active lane in a unit
  `>= K` is swapped with a packable lane in a unit `< K`, at most `B = W` moves per group per
  block; a unit that keeps an active lane simply runs (always exact). Automated strips move only
  into units with the automation-slot bit. `R` and `B` are preallocated constants with reasons;
  recommend `R = 16` blocks (43 ms at 48 k: above any smoother ramp, below every effect rest
  bound, so it only debounces chains whose rest is immediate) and `B = W` (one bank's state per
  block is the worst swap cost; D5).
- Gates: (1) class A differential on compiled plans, packing on and off, random per-lane silence
  runs including whole-bank windows, a fader ride, stored automation on some strips, a collapsed
  mono-armed bank (R7), two route-sign layouts (D10): every strip and the master equal by
  `to_bits`, NaNs folded; Simd8 here, Simd4 on the arm64 CI legs at the batch push (D6). Test
  value: red when a pack moves a strip's or the master's bits, or packs a lane not at rest. Fold
  it into `silent_bank_skip.rs` (one claim, one owner crate). (2) allocation-free with packing
  active. (3) one native and one V8 console run (#1783's rows): the sparse row and
  `one_per_bank` against S4/opt6-1107-fix; all-active within 2 % of opt6-1107-fix (the re-based
  reference, D10 F1); the toggle row's cost recorded.

## Order

After #1782a. Reads #1783's toggle and one-active-lane-per-bank console rows.

## Owner condition

If all-active cannot hold within 2 % of opt6-1107-fix under #1782a's re-materialised home-tile fold, stop and ask the owner whether to accept the cost or shelve packing. A data-dependent fold order is never an option (D6).

## Paths

`crates/graph/src/runtime.rs`, `crates/graph/src/lib.rs`, `crates/graph-compiler/tests/silent_bank_skip.rs`, `artifacts/steps/b14-1782/`, this spec.
