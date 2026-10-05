# Retarget the matrix stage inside a block

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1.5 and A1.7, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R2.

## Product outcome

The strip's matrix and pan stage can apply a retarget or an exact set at any sample offset inside a
block, per bank lane, with the same bits as the block split at that offset. The fused fader and
matrix path declines any block with such an operation. This is the in-block model of draft 08 for
the matrix stage; draft 14b *Render stored pan and matrix automation* is its first user, in the
same batch, so `main` never holds it with no caller. No rendered bit moves.

## Context

- **The matrix stage.** `MatrixStage` holds the ramp words and a per-lane countdown
  (`crates/builtins/src/lib.rs:2884-2894`). `set_target_over` retargets one bank lane, one division
  per coefficient (`:3008-3049`). The kernel `matrix2x2_ramp_block` advances each lane by its own
  additions, so a lane's ramp is partition-invariant (`crates/lane/src/kernels/builtins.rs:425-470`).
- **Bank processors.** `MatrixBankProcessor` (`crates/builtins-compiler/src/lib.rs:778`, impl
  `:787`) drains its lanes at block entry (`drain_matrix_controls`, `:1108`). The fused
  `FaderMatrixBankProcessor` (`:831`, impl `:1172-1261`) runs fader and matrix in one traversal only
  through `try_process_settled_with_matrix`, which returns false while any ramp runs
  (`crates/builtins/src/lib.rs:3926-3935`); draft 08 D5 makes it decline a block with any fader
  operation. Test-support counters record which path ran.
- **Draft 08** gives the fader stage its timed operations and the split model this slice follows.
- *Keep every trim, fader and matrix ramp inside its endpoints* (#1408) changes the ramp kernels'
  last-step law; it is a per-sample rule inside the kernel, so it does not change where operations
  go.

## Decisions frozen for this slice

- **D1. In-block operations on the matrix stage**, on draft 08's model: per bank lane, at an offset,
  "retarget to a 2x2 over `n` samples" (`set_target_over`) and "set a 2x2 exactly". Between
  operations the existing kernels run; operations at one offset apply in lane order. The stage is
  class A: a block with an operation at offset `o` equals the same block split at `o` with the
  operation at the second part's entry, bit for bit, at `Scalar`, `Simd4` and `Simd8`.
- **D2. The fused path** runs only on a block in which neither stage has an operation or a ramp in
  flight; any operation sends the block to the split fader and matrix path.
- **D3. The acked-batch question** has no subject here: the operations come from render's own
  caller; nothing is queued.

## Deliverables

1. D1 in `crates/builtins` and the matrix bank processors.
2. D2 in the fused processor.

## Authorized paths

- `crates/builtins/src/lib.rs` (the matrix stage), `crates/builtins/tests/matrix.rs`
- `crates/builtins-compiler/src/lib.rs` (the matrix and fused processors)
- `crates/lane/src/kernels/builtins.rs` (a range entry only, if the split needs one)

## Non-goals

- Stored pan and matrix automation, its events, group cell and edits (draft 14b).

## Hazards

- **`set_target_over` adopts the window as the lane's own** (`:3008-3012`); no production path reads
  that window after this slice, but a test that calls `set_target` after an operation sees it.

## Objective gates

1. **Partition identity** (`crates/builtins/tests/matrix.rs`, new). D1's identity at `o` in
   `{1, 63, 64, 127}` (`q = 128`) and in a block of 100, at `Scalar`, `Simd4` and `Simd8`, for both
   operations.
2. **Fused path** (`crates/builtins-compiler` test, new). On the fused form, the fused counter
   advances on a block with no operation and no ramp, and the fallback counter on every block with
   one.
3. **No rendered bit moves.** Every existing render test passes with unchanged digests; the G5
   corpus (`cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`)
   passes unchanged.
4. **Commands:**
   - `cargo test --locked -p builtins --features test-support,lane/test-support`,
     `cargo test --locked -p builtins-compiler --features test-support`
   - the DSP leg (`test-debug-b`) in `.github/workflows/qualification.yml`
   - `bash scripts/check-builtins-policy.sh`, `bash scripts/check-realtime-policy.sh`,
     `bash scripts/check-unfused-seal.sh`, `bash scripts/check-workspace-policy.sh`,
     `bash scripts/check-cross-targets.sh`, `bash scripts/run-aarch64-tests.sh debug`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1: red if the split re-derives a step or restarts a lane's ramp.
- Gate 2: red if the fused path skips an operation, or if the split path runs on silent, settled
  blocks.

## Dependencies

Batch R2. Direct dependency:

- Draft 08 *Apply timed operations inside a block on the strip fader stage* (the model and the
  fused path's fader half).

Draft 14b depends on this draft.
