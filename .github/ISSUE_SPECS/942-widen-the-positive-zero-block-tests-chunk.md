# Widen the positive-zero block test's chunk

**Ruled** (coordinator, 2026-09-26): successor S3 of "Skip unplayed source claims in the fused Output
reduction" (silence masks). Class A: a predicate with the same truth value on every input.

## Product outcome

`block_is_positive_zero` (`crates/effect-runtime/src/bank.rs:130`) is the admission test for every
effect's silent fast path: parametric EQ, compressor and true-peak limiter call it per bank per block.
Its `chunks(32)` loop is already vectorised on x86 (`vpor ymm`), but it pays a horizontal
reduction and a branch every 32 words, which is most of its cost. The adversarial verification
measured the candidates on both targets (release, fat LTO; x86 Zen 3 cycles, wasm simd128 in Node 22):

| input | today, x86 | `chunks(128)`, x86 | today, wasm | `chunks(128)`, wasm |
|---|---:|---:|---:|---:|
| 128 silent words | 66 cycles | 38 | 16 ns | 10 |
| 1,024 silent words | 508 | 217 | 119 ns | 68 |

`chunks_exact(64)`, the research's first proposal, compiles on wasm simd128 to 64 scalar `i32.or`
per chunk and ran about 3.4 times *slower* than today in the browser build (16 to 53 ns on 128
words, 119 to 405 ns on 1,024, and 6 to 27 ns on a live block). It must not be used.

On an all-silent session every silent bank pays this test several times per block. Widen the chunk
to 128 words, keeping the early exit on a live block.

## Smallest closable slice

Authorized paths: `crates/effect-runtime/src/bank.rs` (`block_is_positive_zero` and its tests only),
and this spec.

1. Change the loop's chunk from `chunks(32)` to `chunks(128)` (a constant, not a track cap). The fold
   and the early exit stay exactly as they are. No `unsafe`, no `wide`, no intrinsics. Do not switch to
   `chunks_exact`: see the wasm measurement above.
2. Keep the doc comment's argument (bits, not `== 0.0`; `-0.0` excluded) and update only the
   sentence about chunk size.

## Non-goals

No change to any caller, to the predicate's meaning, or to `lane_is_positive_zero` below it. No new
use of the test anywhere.

## Objective gates

1. **Same truth value.** A property test over at least 10,000 seeded blocks of lengths 0..=2,100
   (covering 0, 1, 63, 64, 65, 128, 256, 1,024, 2,048 and odd lengths) with a single nonzero bit
   pattern placed at every position class (first word, last word, last word of a chunk, the
   remainder), including `-0.0` (`0x8000_0000`), the smallest subnormal and a quiet NaN, agrees with
   a naive reference `io.iter().all(|v| v.to_bits() == 0)`.
2. **Standing tests.** `a_negative_zero_input_block_is_not_treated_as_silence` (parametric-eq), the
   `silent_fixed_point` tests of parametric-eq and compressor, and the limiter's silent-rest tests
   pass unchanged.
3. **Both targets.** Record in the evidence (a) the release x86-64 disassembly showing the chunk fold
   as packed `vpor` with no horizontal reduction inside each 32-word step, and (b) the release
   wasm32 simd128 disassembly (`wasm-tools print` or `wasm-objdump -d`) showing the chunk fold as
   `v128.or` with no scalar `i32.or` chain. Record a throwaway timing on both targets for 128 and
   1,024 silent words and one live block, today against the change (descriptive only). A wasm
   timing slower than today's on any input is a hard stop.
4. **Digests.** Every console workload's digest is unchanged.
5. **Wasm.** The batch's AudioWorklet artifact passes `scripts/check-web-audioworklet-callgraph.py`
   (rule 3 does not apply to an integer fold, but the gate must stay green).
6. fmt, clippy with `-D warnings`, `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace
   --no-deps`, `cargo test -p effect-runtime -p parametric-eq -p compressor -p true-peak-limiter`.

## Console benchmark rows

Can move: `sixty_four_track_idle` and any row with a silent bank. No digest may move.

## Dependencies

None.

## Standing rules for the implementer

- Work only from this body. Do not survey the workspace.
- Class A: every gate that says "unchanged" or "agrees" is a hard stop.
- Commit on `codex/<issue>-<slug>` from synchronized `main`. Do not run the timed runner; do not quote
  a projected saving.
- The AudioWorklet artifact pin and browser qualification are repinned once at the batch boundary.
