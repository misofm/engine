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

## Attempt 2 evidence

**Verdict: STOP on the bounded gate 3. The predicate change is not committed; `bank.rs` is back to
`HEAD`.** The rescoped shape meets the live-block bound on both targets and the 1,024-word bound on
both, but a 128-word silent block is only 22.9 % faster than today on wasm (the bound is 25 %), and
on x86 the result swings between 21.4 % and 33.4 % with code layout between builds. Gates 1, 2, 4, 5
and 6 were not run in the repository, because the coordinator's instruction was to stop before
committing the code if the bounded gate failed.

### Change measured

`block_is_positive_zero`: `let (head, rest) = io.split_at(io.len().min(32));` then return `false` if
`or_bits(head) != 0`, then `for chunk in rest.chunks(128)` with the same early exit. `or_bits` is a
private `#[inline]` helper holding today's fold unchanged (`bits |= value.to_bits()`). The doc
comment's chunk-size sentence was the only prose change. No `unsafe`, no intrinsics, no
`chunks_exact`.

### Harness

The same throwaway harness as attempt 1, in a scratch directory and deleted afterwards. It uses the
same profile and flags and is pinned to one core. There are four arms: `today` (a verbatim
`chunks(32)` replica), `chunks(128)` (the attempt-1 replica), `head+128` (the real crate function
through a path dependency) and `floor` (the same call and loop around a function that only tests
`is_empty`). Arms rotate within each round. The x86 values are the minimum of 40 x 2,000 calls. The
wasm values are the minimum of 30 x 20,000 calls in Node v22.23.2. There were five runs across two
builds.

### Timing (per call)

| input | today | `chunks(128)` | head+128 | harness floor |
|---|---:|---:|---:|---:|
| x86 128 silent | 26.6-29.0 tsc | 13.7-14.5 | 19.3-20.9 | 4.8 |
| x86 1,024 silent | 201-205 tsc | 81.4-87.9 | 90.3-92.7 | 4.8 |
| x86 live 1,024 | 10.5-12.1 tsc | 13.7-14.5 | 9.7-10.5 | 4.8 |
| wasm 128 silent | 16.43 ns | 11.32 ns | **12.66 ns** | 3.14 ns |
| wasm 1,024 silent | 119.4-120.0 ns | 73.7-73.8 ns | 71.3-71.5 ns | 3.14 ns |
| wasm live 1,024 | 5.39 ns | 11.05 ns | 5.93 ns | 3.14 ns |

x86 TSC runs at 3.0 GHz, so 1 ns is 3 tsc. The two x86 builds were paired as follows: build A had
today at 26.6 and head+128 at 20.9 (21.4 % faster); build B had today at 29.0 and head+128 at 19.3
(33.4 % faster). The wasm numbers did not move between runs or builds.

Bounded gate 3, per call:

| bound | x86 | wasm |
|---|---|---|
| live at most today + 1 ns | pass (0.3 ns faster) | pass (+0.54 ns) |
| 1,024 silent at least 25 % faster | pass (54-55 %) | pass (40 %) |
| 128 silent at least 25 % faster | **layout-dependent (21.4-33.4 %)** | **FAIL (22.9 %)** |

The 128-word miss comes from the shape itself. A 128-word block under head+128 still pays two
reductions (the head and one 96-word chunk), against today's four. Only `chunks(128)` alone pays
one, and it fails the live bound. Moving the head's size does not change the count of two. With the
harness floor subtracted (a descriptive figure only, not the gate as written), the wasm 128-word
saving is 28.4 % (13.29 to 9.52 ns). Whether that is what the bound means is a coordinator ruling.

### Disassembly (requirements met)

- x86 (`objdump -d -M intel`). The head loop is 2 x `vpor ymm` per 16-word step, then one
  reduction. The wide loop is 4 x `vpor ymm` per 32-word step with no reduction inside it, and one
  `vextracti128`/`vpshufd`/`vmovd` reduction per 128-word chunk. Scalar `or r32, DWORD` appears only
  in the two remainder loops.
- wasm (`wasm-objdump -d`, wabt 1.0.34). Each of the two loops is `v128.load` + `v128.or` per
  4 words, followed by one `i8x16.shuffle`/`v128.or`/`i32x4.extract_lane` reduction.
  `i32.const 32` bounds the head and `i32.const 128` bounds the chunks. `i32.or` appears exactly
  twice, once in each remainder loop, with no chain.

### Gate 1 shape (harness only)

The harness compared all three shapes with `io.iter().all(|v| v.to_bits() == 0)` on 578,841 blocks.
Lengths ran over 0..=2,100. The patterns were `-0.0`, the smallest subnormal, a quiet NaN and
`1.0`. Each was placed at the first word, the last word, every 32-word boundary (the last word of the
head, the first word of each wide chunk and the last word of each wide chunk), and two seeded
random positions. Every block agreed.

## Attempt 3 evidence

**Verdict (implementer): every corrected gate passes, with one deviation for the verifier.** The
predicate carries `#[inline(always)]` where it had `#[inline]`. Without it, the head+128 shape
fails the live-block bound in the shipped wasm artifact; the reason is set out below.

### Change

`crates/effect-runtime/src/bank.rs` only:

- `block_is_positive_zero` folds the first `min(32, len)` words and returns `false` on any set bit.
  It then folds the rest in `chunks(128)` with the same early exit. The fold is today's
  `bits |= value.to_bits()`, moved unchanged into a private `#[inline] fn or_bits`. There is no
  `unsafe`, no intrinsic and no `chunks_exact`.
- **Deviation:** the predicate's attribute changes from `#[inline]` to `#[inline(always)]`.
- The doc comment updates its chunk-size sentence and adds three lines explaining the attribute.
- A `#[cfg(test)] mod tests` adds gate 1's property test.

### Why `#[inline(always)]` (found under gate 5)

With plain `#[inline]`, the release wasm AudioWorklet artifact (the build script's own cargo line)
stops inlining the two-loop predicate. It emits three outlined copies of
`effect_runtime::bank::block_is_positive_zero`. The limiter's copy is called from 7 sites, the
compressor's from 4 and the parametric EQ's from 4. Today's one-loop form is inlined at every site;
the base artifact has no such symbol. The attempt-2 harness had put both arms behind
`#[inline(never)]`, so it could not see this. Measuring each form as it ships (Node 22, per call):

| live 512-word block, wasm | today inlined | head+128 inlined | head+128 outlined |
|---|---:|---:|---:|
| ns per call | 4.04 | 4.04 | 5.93 |

The shipped plain-`#[inline]` form is therefore +1.89 ns on a live block, against a 1 ns bound.
`#[inline(always)]` removes all three outlined copies; the rebuilt artifact contains no
`block_is_positive_zero` symbol. The live cost then equals today's.

### Gate 3 timing (descriptive)

The throwaway harness sat in a scratch directory and was deleted afterwards. It used the release
profile (fat LTO, `codegen-units = 1`, `panic = "abort"`). Native was built with
`+avx2,+fma` and wasm with `+simd128`. It ran pinned with `taskset` on an AMD EPYC 7313P
(Zen 3, TSC at 3.0 GHz). x86 values are the minimum of 40 x 2,000 calls; wasm values are the
minimum of 30 x 20,000 calls in Node v22.23.2. Arms rotated within each round.

The arms were:

- `today`: a verbatim `chunks(32)` replica.
- `chunks(128)`: attempt 1's shape.
- `head+128`: the real crate function through a path dependency.
- *outlined*: the arm sits behind an `#[inline(never)]` wrapper.
- *inlined*: the arm is folded into the timing loop, which is how the shipped code calls it.

**Production form (inlined), the gate:**

| input | x86 today | x86 head+128 | wasm today | wasm head+128 |
|---|---:|---:|---:|---:|
| 128 silent | 24.2 tsc (8.09 ns) | 16.1 tsc (5.39 ns) | 15.08 ns | 10.78 ns |
| 512 silent | 91.9 tsc (30.71 ns) | 42.7 tsc (14.28 ns) | 57.25 ns | 35.05 ns |
| 1,024 silent | 195.9 tsc (65.45 ns) | 78.2 tsc (26.14 ns) | 118.3 ns | 68.1 ns |
| live (512 or 1,024) | 7.3 tsc (2.44 ns) | 7.2-7.3 tsc (2.44 ns) | 4.04 ns | 4.04 ns |

**Outlined, all three shapes (the attempt 1 and 2 method):**

| input | x86 today | x86 `chunks(128)` | x86 head+128 | wasm today | wasm `chunks(128)` | wasm head+128 |
|---|---:|---:|---:|---:|---:|---:|
| 128 silent | 26.6-29.0 | 14.5 | 19.3-20.9 | 16.43 | 11.32 | 12.66 |
| 512 silent | 96.8-99.2 | 45.9-46.7 | 49.1-50.8 | 58.6 | 37.98 | 37.45-37.48 |
| 1,024 silent | 200.9-204.8 | 87.9 | 90.3-92.7 | 119.4-119.9 | 73.7-73.8 | 71.2-71.5 |
| live | 10.5-12.1 | 14.5 | 9.7-10.5 | 5.39 | 11.05 | 5.93 |

In the outlined table, x86 values are in tsc and wasm values in ns.

**Corrected gate 3, production form:**

| bound | result |
|---|---|
| wasm 512 silent at least 25 % faster | pass: 38.8 % |
| wasm 1,024 silent at least 25 % faster | pass: 42.4 % |
| x86 1,024 silent at least 25 % faster | pass: 60.1 % |
| live at most 1 ns slower | pass: x86 +0.0 ns, wasm +0.00 ns |
| 128 silent no slower | pass: x86 33 % faster, wasm 28.5 % faster |

The outlined-both method also passes every bound: wasm 512 at 36.1 %, 1,024 at 40.2 % and live at
+0.54 ns; x86 1,024 at 54.7 %.

The attempt-2 finding stands: at 128 words the head+128 shape is 22.9 % faster on wasm in the
outlined method and 28.5 % inlined.

### Gate 3 disassembly

- **x86-64, harness `pz_crate`, final source.** The head is 2 x `vpor ymm` per 16-word step
  followed by one reduction. The wide loop is 4 x `vpor ymm` per 32-word step, with one
  `vextracti128`/`vpshufd`/`vmovd` reduction per 128-word chunk and none inside the step:
  ```
  vpor ymm0,ymm0,YMMWORD PTR [rax+r10*4]
  vpor ymm1,ymm1,YMMWORD PTR [rax+r10*4+0x20]
  vpor ymm2,ymm2,YMMWORD PTR [rax+r10*4+0x40]
  vpor ymm3,ymm3,YMMWORD PTR [rax+r10*4+0x60]
  add  r10,0x20
  ...
  vextracti128 xmm1,ymm0,0x1        ; once per chunk
  ```
  Scalar `or r32, DWORD` appears twice, once in each remainder loop.
- **wasm32 simd128 (`wasm-objdump -d`, wabt 1.0.34).**
  - In the harness, each loop is `v128.load 2 0` + `v128.or` per 4 words, followed by one
    `i8x16.shuffle`/`v128.or`/`i32x4.extract_lane` reduction. `i32.or` appears twice, once in each
    remainder loop, with no chain.
  - In the shipped artifact (`inline(always)`), the compressor bank's `render` carries the inlined
    wide loop (`i32.const 128` bound, `v128.load 2 0`, `v128.or`).
  - The `min(len, 128)` chunk-bound sequences rise from 13 in the base artifact to 46.
  - `v128.or` rises from 636 to 735 and `i32.or` from 2,031 to 2,064, the added remainder loops.
  - The longest run of consecutive `i32.or` is 39 in both base and change, and that run is not
    the predicate.

### Gate 1

`bank::tests::the_widened_chunk_agrees_with_the_naive_reference` runs 221,581 blocks.

- **Lengths:** every length 0..=2,100.
- **Blocks:** the all-zero block at each length, plus one nonzero word from five patterns: `-0.0`,
  the smallest subnormal, a quiet NaN, an all-ones NaN and `1.0`.
- **Positions:** the first word, the last word, the last word of the head, the first and last
  word of every 128-word chunk (the last chunk is the remainder), and one xorshift-seeded word per
  length.

The test compares every block against `io.iter().all(|v| v.to_bits() == 0)`. It passed.

Each red mutation was checked with `cargo test -p effect-runtime --lib`:

| mutation | first failure |
|---|---|
| sign bit masked out of the fold | len 1, word 0, `-0.0` |
| `chunks_exact(128)` in the wide loop | len 33, word 32 |
| return after the head | len 33, word 32 |
| head's early exit dropped | len 1, word 0 |

The throwaway harness separately agreed on 578,841 blocks across all three shapes.

### Gate 2

`cargo test --locked -p effect-runtime -p parametric-eq -p compressor -p true-peak-limiter`: 312
passed, 0 failed, 5 ignored, across 54 suites. The passing tests include:

- `a_negative_zero_input_block_is_not_treated_as_silence` in parametric-eq, compressor and the
  limiter.
- `a_settled_silent_bank_renders_exactly_the_never_fast_path` (parametric-eq) and
  `a_settled_silent_bank_renders_exactly_the_bank_that_never_fast_paths` (compressor).
- The limiter's silent-rest tests: `a_settled_silent_limiter_renders_exactly_the_never_fast_path`
  and `a_limiter_still_releasing_through_the_silence_is_never_frozen`.
- `a_detector_still_releasing_through_the_silence_is_never_frozen`.

### Gate 4: digests

A throwaway test, deleted and not committed, rendered all 16 `WORKLOADS` plus
`DRIVER_FED_WORKLOADS` (`sixty_four_track_plumbing_ring`). Each ran 64 blocks through
`SessionRuntime::render` + `hash_output`. The base commit and the final change were compared in
both the dev and the release profile. All 17 digests are identical in every comparison, and the
release digests equal the dev digests. `sixty_four_track_plumbing_only` equals the digest pinned
in `chain_shape.rs`, `57535244...f800`.

These digests are a non-regression check, not a discriminator. The "return after the head"
mutation also left all 17 unchanged, because no workload has a block that is silent for its first
32 words and then live. Gate 1 is the discriminating test.

### Gate 5

The build script's cargo line was used:
`RUSTFLAGS="-C target-feature=+simd128 -C strip=debuginfo <remap>" cargo build --locked --release --target wasm32-unknown-unknown -p host-web`.
It built into a scratch target directory with no repin, and `wasm-objdump -d` was piped into
`scripts/check-web-audioworklet-callgraph.py` with the four invocations from
`check-web-audioworklet.sh`. Every check passes on the final artifact. The base artifact,
plain `#[inline]` and `inline(always)` all give identical numbers:

| check | result |
|---|---|
| `--callgraph miso_engine_web_v1_render` | closure 8, traps 5 |
| `--kernel-shape` | f32x4 arithmetic 11,719, 15 kernels, every roster entry ok |
| `meter_poll` | closure 9, traps 2 |
| `command_submit --allocation-only` | closure 39 |

The artifact hash is not the pin, and neither is the base's (`0b6b0632...`): the branch carries
other issues, and the pin is repinned at the batch boundary.

### Gate 6

`cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
(178 crates checked) and `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` are
all clean. The four suites are as in gate 2. `cargo test --locked -p console-workload` gives 37
passed, 0 failed and 2 ignored. `scripts/check-realtime-policy.sh` reports ok (55 marked regions
in 16 files). All ran with `CARGO_INCREMENTAL=0`. The timed console runner was not run.
