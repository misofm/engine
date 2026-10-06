# Replay an effect ramp whose endpoint clamp acts in the wasm corpus

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-06 by the decision-15 root coordinator's ruling (2) on *Keep every effect parameter
ramp inside its endpoints* (#1409, attempt 1 verdict NIT 7). Code anchors verified on
`codex/d15-stream-g` at `c7f7bbdfb`.

## Product outcome

The browser build is proven to clamp an effect's parameter ramp at its target bit for bit as the
native build does. Today the wasm `simd128` leg reaches #1409's clamp only through `lane`'s own
cases (the `ramp_toward` `pmin`/`pmax` lowering); no G5 corpus case renders an effect move in which
the clamp acts. After this slice at least one effect corpus case does, and its digest is replayed
under wasm.

## Context

- **The clamp.** #1409 routes every effect ramp site through
  `lane::kernels::ramp_toward(current, step, target)` (strict `min`/`max`, keeps an in-range word's
  bits, passes NaN). Gate 2 (`tests/ramp_endpoint.rs` per effect) runs native only; `aarch64-debug`
  runs it at NEON width.
- **The G5 corpus.** `tools/wasm-gate-corpus/src/lib.rs` delegates whole rendered effect blocks to
  each effect's `corpus` module (`transient_shaper`, `delay`, `multiband_compressor`, `soft_clip`,
  `parametric_eq`, `gate_expander`, `builtins`), pins their digests in the effect crate (for example
  `crates/gate-expander/src/gate_digests.in`), and replays them under wasm
  (`scripts/run-wasm-gates.sh`); `wasm-gates`' `g5_native_digests_match_pins` is the native owner of
  every pin (#1048). Some cases already have a D11 ramp in flight (gate-expander, parametric EQ),
  but #1409 moved no pin, so no case's ramp is one the clamp changes.
- **The moves.** #1409's scan found, per ramped word, an in-domain move whose unclamped word passed
  its target (for example the gate threshold at frame 33). Such a move is the case to add.

## Decisions frozen for this slice

- **D0. Root decision (2026-10-06).** The decision-15 root coordinator filed this slice as the wasm
  reach of #1409's clamp.
- **D1. One effect, one case.** Add one case to the gate-expander corpus (site 5, the gate
  prologue, a vector clamp site at every lane width): a move from #1409's scan, with lanes ending
  their ramps on different frames, in which the clamp acts on at least one frame per lane.
  Another effect may be chosen only with a reason recorded (for example a site the gate case cannot
  reach).
- **D2. New pin, no moved pin.** The new case adds one digest row; every existing pin stays.

## Deliverables

1. The new case in the chosen effect's `corpus` module, its pin in that crate's `.in` file, the
   case count in `tools/wasm-gate-corpus` updated.
2. The attempt record: the clamp acts in the case (the frames where the unclamped word passes its
   target), and the mutation evidence of gate 2.

## Authorized paths

- `crates/gate-expander/src/corpus.rs`, `crates/gate-expander/src/gate_digests.in` (one new row)
- `tools/wasm-gate-corpus/src/lib.rs` (case count and doc only)
- This spec

## Non-goals

- Cases for the other effects; changing the clamp or any render code; native gate-2 tests.

## Hazards

- Hot files: `crates/gate-expander/src/corpus.rs` and `tools/wasm-gate-corpus/src/lib.rs`
  (other slices add G5 cases to the same list); the later slice rebases.
- A move that is clamped only after the window, or on no lane at the native width, is vacuous:
  gate 1 checks reach.

## Objective gates

1. **Reach.** A check in the case's test (native) shows the unclamped law passes the target on at
   least one frame of the case per lane, so the clamp acts.
2. **Pins.** `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`
   passes with the new row, and no existing pin moved; `bash scripts/run-wasm-gates.sh` passes
   (simd128 leg replays the new case).
3. **Red.** The gate prologue clamp reverted to `add` (#1409's site-5 mutant) moves the new digest
   (red natively); recorded.

## Test value

A wasm `simd128` lowering of an effect's ramp clamp that differs from the native one (operand order
of `pmin`/`pmax`, a `-0.0` or NaN difference inside a real effect render path rather than inside a
lane kernel) turns the new case red under wasm; no existing case reaches it.

## Dependencies

- #1409 (merged with the stream G batch).
