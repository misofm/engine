# Builtin rest claims, and drain every builtin bank at `begin_block`

Draft, slice S3 of the silence architecture issue (A0). Class A. Evidence:
`docs/handoffs/silence-2026-09-27/DESIGN.md` sections 2 and 4.3. The builtin input section is 56 % of
the idle block (16.8 of 30.1 µs) and has no rest check; this slice gives the builtin banks the
contract's claim, and the seam it lives on. S4 is the consumer.

## Product outcome

* The seam of A0 D3 exists: `rack::BankStage::{silent_skippable, advance_silent}`
  (`crates/rack/src/lib.rs:559`) and the same two methods on
  `graph::GraphPreparedBuiltinBankProcessor` (`crates/graph/src/lib.rs:1048`), all defaulting to
  decline / no-op, with `graph`'s `BuiltinStage` (`crates/graph/src/runtime.rs:3881`) forwarding.
* The builtin banks answer it exactly:
  * **input section** (`BuiltinInputBank`, `crates/builtins/src/lib.rs:3409`; `InputStage`, `:952`):
    true iff no trim ramp and no filter ramp is in flight (`ramping`, `filter_ramping`, `:1004`,
    `:967`), no record was admitted by this block's `begin_block`, and every integrator word of every
    lane is `+0.0` (`state`, `:969`: eight lane vectors, `store_bits` and an OR);
  * **fader** (`BuiltinFaderBank`, `:3703`): true iff every lane's ramp countdown is 0 and no record
    was admitted this block;
  * **matrix** (`BuiltinMatrixBank`, `:3938`) and the fused fader/matrix processor: likewise settled.
* The fader, matrix and fused fader/matrix bank processors drain their queues in `begin_block`
  instead of at the top of `process` (`crates/builtins-compiler/src/lib.rs:634`, `:693`, and the
  fused processor's two drains at `:1070-1071`). Records still take effect on the first sample of the block that
  drains them; the only change is that the drain now precedes the chain's gather. This is what lets
  a skipped block never drop or delay a record.
* **What the move changes besides ordering** (state in the evidence; neither is a rendered bit on a
  successful block): (1) the error path: today the fused processor still renders the fader when the
  matrix drain fails and then returns `Err`; after the move, a drain `Err` from `begin_block` aborts
  the chain before slot 0 (`crates/rack/src/lib.rs` drain loop), and the executor fills the host
  planes with `+0.0` as on any unit failure; (2) test-support witness counters that read
  `active_lanes()` around the drain (`crates/builtins-compiler/src/lib.rs:1058-1066`) may move and
  must be re-pinned with the reason recorded.

**Why the input section's claim is exact.** With every integrator `+0.0` and a `+0.0` input,
`svf_step`'s update `flush(ic + 2d)` returns `+0.0` for every zero `d` (`flush` maps both zeros to
`+0.0`, `crates/lane/src/lib.rs:133`; `crates/lane/src/kernels.rs:350-359`), so the state is a fixed
point. The block's output is then a pure function of the constant state and coefficients, the same
every block; S4 verifies it is `+0.0` when it seals. The derivation in DESIGN.md 4.3 shows it is
`+0.0` in every shape (the last section always adds a `+0.0` term); gate 1 proves it.

## Authorized paths

`crates/rack/src/lib.rs` (the two trait methods and their docs only), `crates/graph/src/lib.rs` (the
two trait methods only), `crates/graph/src/runtime.rs` (`BuiltinStage` forwarding only),
`crates/builtins/src/lib.rs`, `crates/builtins-compiler/src/lib.rs`, tests under `crates/builtins/`,
`crates/builtins-compiler/` and `crates/rack/`, their `MUTATIONS.md`, this spec.

## Non-goals

No caller of `silent_skippable` (S4). No effect claims (S4 exposes them). No change to any kernel.

## Objective gates

1. **Fixed point, exhaustively.** For each width (4, 8, and the scalar tail), on the dual and the
   mono paths, and for every per-lane shape: HPF on/off, LPF on/off, each at a low, a mid and a high
   cutoff; polarity inverted or not; trim at -24, 0 and +24 dB; mixed shapes across the lanes of one
   bank; the elided and the unelided bodies. Drive 64 blocks of seeded noise, then silence until the
   claim holds (it must within 4,096 blocks); then for 64 further silent blocks assert: the claim
   stays true, the output block is `+0.0` by bits, and every state word
   (`builtins::test_support::bank_lane_state_words`, plus the fader and matrix `*_bank_lane_words`) is bit-identical before and after each block.
2. **Never early.** Over the same corpus, the claim is false on every block where any state word is
   nonzero or any ramp is in flight, and on the block that admits a trim, polarity or filter record
   (including an instant, zero-smoothing one).
3. **Fader and matrix.** Settled with no admission ⇒ claim true; the block after an admitted gain,
   mute or matrix record ⇒ false until the ramp ends.
4. **Drain move.** Every builtins-compiler control test passes unchanged; a new test pushes a fader
   record and a matrix record before block N and asserts both take effect on block N's first sample,
   bit-identical to the pre-change render (a pinned digest over a scripted command sequence, both
   control deliveries). The records' acknowledgements are unchanged.
5. **Red mutations** (`MUTATIONS.md`): drop the `ramping` test; drop the `filter_ramping` test; OR only
   `ic1`; leave the claim true on the admitting block; drain the fader in `process` again (gate 4
   still passes, so pair it with S4's release gate when S4 lands; record it as S4-dependent).
6. **Unchanged.** Every console row's digest; allocation-free render tests;
   `scripts/check-realtime-policy.sh`; the AudioWorklet callgraph gate (the claim's OR fold is integer
   code, not `f32x4` arithmetic).
7. fmt, clippy with `-D warnings`, `cargo test -p builtins -p builtins-compiler -p rack -p graph`.

## Console benchmark rows

None move. Every digest is unchanged.

## Dependencies

A0 ruled. Independent of S1 and S2.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A: every gate that says "unchanged" or "bit-identical" is a hard stop.
- Commit on `codex/<issue>-<slug>`. Do not run the timed runner; do not quote a projected saving.
