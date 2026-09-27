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

## Attempt 1 evidence

**Verdict: HARD STOP on gate 3 (wasm timing). Nothing committed; `bank.rs` restored to `HEAD`.**
`chunks(128)` is slower than today on a live block in the wasm build (5.39 ns to 11.05 ns, about
2x), and gate 3 makes any slower wasm timing a hard stop. This is structural, not noise: a live
block exits after its first chunk, and the first chunk is now 128 words instead of 32. The
verification's own table (`docs/handoffs/plumbing-floor-2026-09-26/SILENCE-MASKS-VERIFY.md` §5,
"wasm live, first chunk": 4-6 ns today, 10 ns for `chunks(128)`) already showed this; the brief's
table carried only the two silent rows.

### Change measured

`crates/effect-runtime/src/bank.rs`, `block_is_positive_zero`: `io.chunks(32)` to `io.chunks(128)`,
fold and early exit untouched, plus the chunk-size sentence of the doc comment. No other line.

### Harness (throwaway, outside the repository, deleted afterwards)

A scratch crate with a path dependency on this worktree's `effect-runtime`, built with the shipped
release profile (`lto = "fat"`, `codegen-units = 1`, `panic = "abort"`), `CARGO_INCREMENTAL=0`:
native with `-C target-feature=+avx2,+fma` (the workspace pin), wasm32 with
`-C target-feature=+simd128` (the AudioWorklet flag). `pz_today` is a verbatim replica of today's
body; `pz_crate` calls the real `effect_runtime::bank::block_is_positive_zero`; both are
`#[inline(never)]` and called in a loop through `core::hint::black_box`. A/A check before the change:
the replica and the crate arm agree to 0.1 ns on every input on both targets.

### Gate 3 (a): x86-64 disassembly of the changed function (`objdump -d -M intel`)

The 32-word step is four packed `vpor ymm`, no reduction inside it; the horizontal reduction runs
once per 128-word chunk:

```
1e7f0: vpor ymm0,ymm0,YMMWORD PTR [rdi+r11*4]
1e7f6: vpor ymm1,ymm1,YMMWORD PTR [rdi+r11*4+0x20]
1e7fd: vpor ymm2,ymm2,YMMWORD PTR [rdi+r11*4+0x40]
1e804: vpor ymm3,ymm3,YMMWORD PTR [rdi+r11*4+0x60]
1e80b: add  r11,0x20
1e80f: cmp  r10,r11
1e812: jne  1e7f0
1e814: vpor ymm0,ymm1,ymm0          ; once per chunk from here
1e820: vextracti128 xmm1,ymm0,0x1
1e82a: vpshufd xmm1,xmm0,0xee
1e83c: vmovd r11d,xmm0
```

Result: pass.

### Gate 3 (b): wasm32 simd128 disassembly (`wasm-objdump -d`, wabt 1.0.34)

The chunk fold is one `v128.load` plus `v128.or` per 4 words; the only scalar `i32.or` in the
function is the sub-4-word remainder loop (one occurrence). Today's body is the same shape with
`i32.const 32` in place of `i32.const 128`.

```
loop
  local.get 0
  v128.load 2 0
  local.get 8
  v128.or
  local.set 8
  ...
  br_if 0
end
i8x16.shuffle 0x0b0a0908 0x0f0e0d0c 0x03020100 0x03020100   ; once per chunk
v128.or
i8x16.shuffle 0x07060504 0x03020100 0x03020100 0x03020100
v128.or
i32x4.extract_lane 0
```

Result: codegen pass.

### Gate 3 timing (descriptive; FAIL on wasm live)

AMD EPYC 7313P (Zen 3), TSC 3.0 GHz, pinned to one core with `taskset`; the host carried another
agent's build. x86: per call, minimum of 40 rounds of 2,000 calls, TSC ticks and ns. wasm: Node
v22.23.2, per call, minimum of 30 rounds of 20,000 calls. Arms alternate within each round. Ranges
span three A/B runs and two later runs of the same code (x86 code layout differs between those
builds; wasm did not move). The live block is 1,024 words of nonzero noise in +-0.25, nonzero from
word 0, so both versions exit on their first chunk.

| input | today, x86 | `chunks(128)`, x86 | today, wasm | `chunks(128)`, wasm |
|---|---:|---:|---:|---:|
| 128 silent words | 26.6-29.0 tsc (8.9-9.7 ns) | 12.9-15.3 tsc (4.3-5.1 ns) | 16.43 ns | 11.32 ns |
| 1,024 silent words | 201-202 tsc (67.1-67.5 ns) | 77.3-85.5 tsc (25.9-28.6 ns) | 119.5-119.7 ns | 73.7 ns |
| live 1,024-word block | 10.5-12.1 tsc (3.5-4.1 ns) | 12.9-15.3 tsc (4.3-5.1 ns) | **5.39 ns** | **11.05 ns** |

The live wasm row is slower than today: hard stop. (x86 live is also slower, though gate 3 only
makes the wasm row a stop.)

### Gate 1 (checked in the harness only, not committed as a test)

Over lengths 0..=2,100, a single nonzero pattern (`-0.0`, smallest subnormal, quiet NaN, `1.0`) at
the first word, the last word, the start of the last 128-word chunk, the last word of every 32-word
step and two seeded random positions, plus the all-zero block: 315,801 blocks, and today,
`chunks(128)` and the probe below all agree with `io.iter().all(|v| v.to_bits() == 0)`. Class A holds;
only the timing gate fails.

### Gates 2, 4, 5, 6

Not run: the hard stop came first and nothing is committed.

### Rescope probe (outside this brief's authorized change, descriptive only)

A 32-word head with today's early exit, then `chunks(128)` over the rest: x86 19.3 / 92.7 / 10.5 tsc
(128 silent / 1,024 silent / live; live equals today), wasm 12.66 / 71.3 / 5.93 ns. On wasm the live
block is still 0.54 ns slower than today's 5.39 ns, so it would also fail a strict "no slower on any
input" gate. Any shape that reads more than one 32-word chunk, or adds a split, before a live
block's first exit pays for it on wasm. A rescope has to either accept a bounded live-block cost
explicitly or leave the predicate as it is.
