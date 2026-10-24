# Fuse the seam copy into the first seam-side slot


Filed from the dual-mono research (`docs/handoffs/dual-mono-2026-09-27/DUAL-MONO.md`). Class A copy removal. Base `6ca203f8`. Low measured value; filed
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


## Decision record (2026-10-24, attempt 1)

- Scope (root, from the scope note): the seam is in `BankChain::run` (`crates/rack/src/lib.rs`,
  `if collapse` after the prefix loop); the seam-side slots are the graph's `BuiltinStage` around
  `FaderBankProcessor` and `MatrixBankProcessor` (`crates/builtins-compiler/src/lib.rs`). Authorized
  paths add `crates/lane/src/kernels/builtins.rs` and `crates/graph` (trait default and the
  `BuiltinStage` forward). The "fused fader-matrix stage" gate is void: #1767 removed that stage and
  `make_fader_matrix`.
- Design: `BankStage::process_from_left` (and the same default on
  `GraphPreparedBuiltinBankProcessor`) copies the left plane into the right one, calls `process`
  and returns `Ok(false)`; an override that writes the right plane itself returns `Ok(true)`. The
  chain gives the seam to the first seam-side slot with an active lane and copies itself when no
  seam-side slot runs. `BankChain::seam_copies` counts collapsed blocks that still copied.
- The fader takes it when its right channel is settled and, on an automated bank, when no stored
  event lands inside the block (`process_with` then renders `process`); the matrix when every lane
  is settled. Otherwise copy, then the existing body. New lane kernels `gain_mute_block_from`,
  `matrix2x2_block_from_left` and `matrix2x2_block_without_identity_from_left` share one
  `#[inline(always)]` frame body with their in-place kernels, so the arithmetic and order are the
  same by construction. Class A.
- Evidence (PR, not committed): 64-block release digests of every `WORKLOADS` row and the ring row
  at Simd8 and Simd4, plus a fader ride on `sixty_four_track_console` and `_mono`, identical before
  and after. With temporary counters: `sixty_four_track_console_mono` at Simd8 renders 512
  collapsed blocks with 0 seam copies (`_half_mono` 256 / 0). At Simd4 dispatch on this AVX2 host
  the console folds into per-node inserts, so the collapsed chain holds no seam-side slot and every
  collapsed block takes the chain's own copy (1024 / 1024). A native four-lane build banks its
  effects, so its chain is expected to have the Simd8 shape; not measured here (no AArch64 host).
