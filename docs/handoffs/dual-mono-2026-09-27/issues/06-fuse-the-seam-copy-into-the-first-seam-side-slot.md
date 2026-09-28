# Fuse the seam copy into the first seam-side slot

Draft, not yet a GitHub issue. Class A copy removal. Base `6ca203f8`. Low measured value; filed
because of the standing rule that a render-path copy that is not strictly needed must not exist.

## Problem

A collapsed chain duplicates its resident left plane into the right one with a whole-block copy
(`crates/rack/src/lib.rs:2400-2401`) and then hands both planes to the fader (the first seam-side
slot), which reads them. The copy is not needed: the fader can read the left plane as its right
input and write the right plane directly.

Upper bound measured by skipping the copy (timing only, wrong bits;
`docs/handoffs/dual-mono-2026-09-27/DUAL-MONO.md` §4.2): 0.65 µs of 73.5 µs per block on
`sixty_four_track_console_mono` at 8 lanes (0.9%), 0.7 µs of 132 µs at 4 lanes (0.5%).

## Outcome

On a collapsed block, the first seam-side slot takes a "right input is the left plane" form of its
body: right output `= g_R * left` computed from the left plane before the left plane is scaled in
place, and the chain skips the copy. Bits are unchanged by construction: the right input words are
the left words either way. Dual blocks and non-collapsing chains are byte-for-byte unchanged.

## Read first

`crates/rack/src/lib.rs:2393-2416` (the seam), `crates/builtins/src/lib.rs:3703-3760`
(`BuiltinFaderBank::process`), `crates/builtins-compiler/src/lib.rs:600-720`
(`FaderBankProcessor`, `MatrixBankProcessor`) and `:1008` (`make_fader_matrix`, the fused pair
used by between-render-calls delivery).

## Authorized paths

`crates/rack/src/lib.rs` (a `BankStage` method with a default that performs today's copy then
`process`), `crates/builtins/src/lib.rs`, `crates/builtins-compiler/src/lib.rs`, their tests.

## Gates

* `sixty_four_track_console_mono` and its `_mono_dual` arm keep one digest; the chain-shape mono
  gates (`the_collapse_fires_on_every_mono_cohort_and_no_other`, the disengage and re-engage tests,
  `tools/console-workload/tests/chain_shape.rs:1028-1800`) pass unchanged.
* A counter or allocation-free test that a collapsed block performs no plane copy at the seam.
* Both fader delivery forms (separate fader and matrix slots; the fused fader-matrix stage) covered.
* fmt, clippy `-D warnings`, `cargo test -p rack -p builtins -p builtins-compiler`.

## Non-goals

Changing the matrix arithmetic (`ll*l + lr*r` stays two products even when `l == r`).
