# #1219 *Ramp a send's coefficients with the indexed ramp kernel*: Sol verdict, attempt 1

- **Reviewed:** `git diff 1b5034c35 cdde008f2` (4 files, +680). Branch `codex/batch-submix-k3`,
  worktree `/home/bl/misofm/wt-submix-k3`.
- **Binding documents:**
  - `AGENTS.md`;
  - the spec `.github/ISSUE_SPECS/1219-...md` with its Attempt 1 record;
  - DESIGN P5 and 5.7;
  - VERIFY-1 section 2 (the ramp law) and VERIFY-2 (the no-overshoot bound, MINOR 3);
  - #1220's spec (its authorized paths);
  - the owner preferences on scalar code, target-specific code and lane shape.
- **How I ran it:**
  - I did not modify the worktree, the branch or GitHub. `gh issue view 1219` is OPEN, and its
    title matches the spec's H1.
  - I exported `cdde008f2` with `git archive` to `/tmp/claude-1002/v1219/src`, with its own target
    directory, and ran every gate there. Mutations ran in a second copy.
  - I built a standalone probe: a `#[no_mangle]` export that calls
    `route_mix_ramp_block::<Simd4>`, using the workspace's release profile (fat LTO, one codegen
    unit, `panic = "abort"`). I built it three ways:
    - for `wasm32` with `+simd128`, read by `check-web-audioworklet-callgraph.py`;
    - as assembly for `aarch64-linux-android` (NEON);
    - as assembly for x86-64 with `+avx2,+fma`, at `Simd8`.
  - My scratch tests, the mutation driver and the probe source are saved beside this file as
    `1219-attempt1-verifier-scratch.rs`. Everything under `/tmp/claude-1002/v1219/` was deleted
    afterwards.

## Verdict: FAIL

There is **one MAJOR finding**, with a two-line fix that I verified.

- **The math and the bits are right.** Every spec gate is green, and every value-changing mutation
  I tried is red.
- **The defect:** the new kernel carries bounds-check traps. They will fail the browser's
  render-closure trap gate as soon as #1220 links the kernel, and #1220 has no authorized path to
  fix it (MAJOR-1).
- **Also:** one MINOR (a documentation claim that is false for subnormal steps) and three NITs.

Attempt 2 needs MAJOR-1's fix and its probe evidence. I recommend the MINOR comment fix in the same
pass.

## Findings

### MAJOR-1. The kernel has two trap owners that the browser render-closure gate refuses, and #1220 cannot repair them

**What fails.** I ran the browser's trap gate on the probe:

```
wasm-objdump -d probe.wasm | python3 -B scripts/check-web-audioworklet-callgraph.py --callgraph probe_ramp
FAIL probe_ramp: unexpected trap owner in the render closure:
  ..._4lane7kernels20route_mix_ramp_block..._4wide6f32x4_5f32x4E...
  ..._4lane7kernels22route_mix_settled_tail
probe_ramp: closure=5 traps=4 trap_owners=[route_mix_ramp_block<f32x4>, route_mix_settled_tail]
```

**Cause.** Two checked slice indexes, each compiled to `call slice_index_fail; unreachable`:

- `let right = &mut right[..count];` in `route_mix_ramp_block`;
- the identical `&mut right[..count]` in `mix2x2_block`, which runs inside the outlined
  `route_mix_settled_tail`. Nothing inside that outlined function proves the two planes have equal
  length.

The probe passes equal lengths, and fat LTO still keeps both traps.

**Why this differs from today's kernels.**

- In the shipped artifact, every lane kernel is inlined into `PreparedRenderPlan::render_inner`.
  My run of `check-web-audioworklet.sh` on this head shows `miso_engine_web_v1_render:
  closure=8 traps=5 trap_owners=[render_inner]`.
- `render_inner` is the one allow-listed trap owner (`TRAP_ALLOW_LIST`,
  `scripts/check-web-audioworklet-callgraph.py:111`).
- This kernel is `#[inline(never)]` by D2, and its tails are outlined, so its checks cannot land
  in `render_inner`.

**Why #1220 cannot fix it.**

- `check-web-audioworklet.sh:470` runs `--callgraph miso_engine_web_v1_render` with no
  `--trap-owner`.
- #1220's authorized paths do not include `crates/lane/src/kernels.rs`.
- In that script, #1220 may change only the `--kernel-min` value.

So #1220 would turn the required `qualification` check red, with no repair inside its scope. The
Attempt 1 record ran the probe through `--kernel-shape` only. The settled-tail deviation added the
second trap owner.

**Fix (verified).** Make the kernel trap-free by construction:

```rust
// route_mix_ramp_block
let count = core::cmp::min(left.len(), right.len());
let (left, right) = (&mut left[..count], &mut right[..count]);

// route_mix_settled_tail
let count = core::cmp::min(left.len(), right.len());
mix2x2_block::<f32>(&mut left[..count], &mut right[..count], c);
```

With the fix applied:

- the probe reads `closure=4 traps=0 trap_owners=[]`;
- the kernel shape is unchanged, at 22 vector and 0 scalar instructions;
- `route_ramp` passes 4/4;
- `cargo clippy -p lane --lib --tests --all-features -- -D warnings` is clean.

The `debug_assert_eq!` stays, so debug builds still reject mismatched planes. A release build
processes the common prefix instead of trapping. `mix2x2_block` already does that when `left` is
the shorter plane.

**Evidence for attempt 2:**

- the probe's `--callgraph` line beside its `--kernel-shape` line;
- a `MUTATIONS.md` row: restore `&mut right[..count]`, and the probe's `--callgraph` turns red.

**The alternative, which I do not recommend:** amend #1220 so it passes
`--trap-owner route_mix_ramp_block`. That weakens "nothing the render export calls may trap" for a
trap that can simply be removed.

### MINOR-1. "Cannot pass its target before the snap" is false when the step is subnormal

**The claim.** The doc comment on `INDEXED_RAMP_LENGTH_MAXIMUM` makes it, and DESIGN P5 and
VERIFY-2 share it.

**Why it fails.** The `(length - 1) * 3u < 1` bound assumes all three roundings are relative. When
`|target - start| < length * 2^-126`, the step is subnormal and its rounding error is absolute, up
to `2^-150`.

**Reproduced:**

- `IndexedRamp::new([0; 4], [5 * 2^-149, ..], 7)` gives `step = 2^-149`. Then
  `c(1..=6) = 1, 2, 3, 4, 5, 6` times `2^-149`, against a target of `5 * 2^-149`, so `c(6)`
  passes the target.
- 200 random ramps from a start of at most `1e-33` to `+0.0`, at random lengths up to `2^22`, had
  977 frames go negative before the snap.

**Magnitude:**

- The overshoot is at most about `(length - 1) * 2^-150`, which is below `2^-128` (about
  `2.9e-39`) in absolute terms. It is inaudible.
- Monotonicity still holds.
- It is reachable only for coefficient pairs within `2^-104` (about `5e-32`, or -626 dB) of each
  other at `length = 2^22`.

**Fix.** A comment change only. Qualify the claim like this:

> "while the step is normal; when `|target - start| < length * 2^-126` the step is subnormal and
> `c(k)` can pass the target by at most `(length - 1) * 2^-150`, under `2^-128` absolute"

#1220's "Live routes" section in `docs/BUILTINS_AND_METERING_V1.md` should carry the same
qualifier.

### NIT-1. Nothing guards the vectorisation of the ramp body

Two of my mutations are green on every gate here:

- V9: `vectored = 0`, so the whole ramp runs through the scalar tail;
- V10: every settled frame runs scalar.

V9 would also pass #1220's kernel-shape rule 3, because the scalar code is outlined and the `f32x4`
function keeps the settled mix's vector arithmetic.

**Suggestion:** record the `route_mix_ramp_block` `f32x4` vector count (22 at this head) in #1220's
evidence, so that a drop is visible.

### NIT-2. The snap block runs up to `2 * (WIDTH - 1)` frames scalar

These are the ramp tail plus the settled tail. One vector `select(index.lt(length), c, target)`
could cover the snap vector, and both operations exist on `Lane`.

D2 froze the outlined tail, and these are sub-vector remainders, which is the scalar tail the
"no scalar where vector possible" rule allows. This is not a defect. Revisit it only under a
measured optimisation issue.

### NIT-3. Gate 3 has an assertion that can never fail

Gate 3's `assert_eq!(position, ramp.length, "the position saturates")` checks the test's own `min`.
Saturation is the caller's job (#1220).

**Suggestion:** delete it, and drop the claim "this is the one place it is tested". Comparing with
`mix2x2_block::<f32>` rather than `::<L>` is fine, because `g2_kernel_identity` proves that
`mix2x2_block` does not depend on the width.

## What I verified independently

- **The law is exactly as defined:**
  - `c(0) = start`;
  - `c(k) = round(round(k * step) + start)` for `1 <= k < length`;
  - `c(k) = target`, assigned, from `k = length`.

  Over a full ramp of `2^22 - 1` frames, at blocks of 128, 125 and 3 frames, the kernel at `f32`,
  `Simd4` and `Simd8` matches `coefficients_at(k)` frame by frame: **0 mismatches**. That covers
  every `k` up to `2^22 - 1`, where `k * step` needs the most precision.
- **`k` is exact.** The index vector is `position + iota`, advanced by `WIDTH`. Every value is an
  integer below `2^22 + 16`, so every add is exact. The probe's `f32x4` function converts
  `position` once and does no other scalar work.
- **Monotone, with no overshoot in the normal range.**
  - 48 ramps scanned at every `k`, including lengths `2^22`, `2^22 - 1`, `2^22 - 3` and
    `3,000,001`: 0 overshoots, 0 non-monotone steps.
  - The worst error against an `f64` linear ramp is **2.196 ulp** of the ramp's scale. VERIFY-1
    measured 2.26.
  - A random endpoint search at five lengths near `2^22` checked 2,000,000 ramps times 4
    coefficients at `c(length - 1)`: 0 overshoots. In 65,864 coefficients, `c(length - 1)` equals
    the target exactly, which reaches the target but does not pass it.
- **Settled frames match the static mix bit for bit.**
  - At `position` equal to `length`, `length + 1`, `2^24` and `u32::MAX`, with blocks of 128, 125,
    7, 1 and 0 frames, the kernel equals `mix2x2_block::<L>` with the target at every width.
  - Bits after the ramp are the assigned target, never `start + length * step` (M2 and my V1 are
    red).
  - The deviation, settled whole vectors through `mix2x2_block::<L>` plus an outlined `f32` tail,
    is bit-identical by construction: the per-frame arithmetic is the same at every width. V10
    (all settled frames scalar) is green, which proves the split cannot move a bit.
- **Retarget.** `coefficients_at(p)` equals the kernel's `c(p)`, so
  `IndexedRamp::new(coefficients_at(position), ..)` continues exactly from the last rendered
  frame's coefficients.
- **Non-finite inputs.** Each of these yields `length == 0` and a finite step:
  - `NaN` in `start` or in `target`;
  - `inf - inf`;
  - `0 -> inf`;
  - `MAX -> -MAX`;
  - `-MAX -> MAX / 2`, which overflows.

  `MAX -> 0` ramps with a finite step. No finite pair produced a non-finite coefficient. Because
  `(length - 1) / length * (1 + 2u) < 1` for `length <= 2^22`, `k * step` cannot overflow.
- **No fused operations, and no dependence on them.** `Lane::fma` is `(a * b) + c` on every
  backend, and rustc never contracts. Emitted code:
  - wasm `f32x4` instance: `f32x4.mul` 12 and `f32x4.add` 10, with no `relaxed_madd`;
  - NEON `Simd4` instance: `fmul` 12 and `fadd` 10, with no `fmla` or `fmadd`; the NEON tails
    contain no fused instructions either;
  - AVX2 `Simd8` instance: `vmulps` 12 and `vaddps` 10, with no `vfmadd`.

  `check-unfused-seal.sh` is ok.
- **No target-specific code; lane shape follows target features.** There is one generic body. The
  only `cfg` is the existing `avx2` predicate on `Simd8`. `FRAME_INDEX_OFFSETS` has 16 entries, so
  it already fits a future 16-lane width.
- **Realtime safety.** Planes are borrowed and the arrays live on the stack. There is no
  allocation, lock or syscall. The only non-arithmetic path is the bounds check in MAJOR-1.

## Gates (head `cdde008f2`, x86-64-v3 with AVX2, AMD EPYC 7313P, rustc 1.97.1)

| Gate | Result |
|---|---|
| test-debug-b (DESIGN 7) | exit 0; 146 `test result: ok`; `route_ramp` 4/4 |
| `check-lane-policy.sh` / `test-lane-policy.sh` | ok / exit 0 |
| `check-unfused-seal.sh` / `--self-test` | ok (8 registered audit calls) / 57 passed, 0 failed |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| `build-web-audioworklet.sh --named-twin` | exit 0; module `1900ef1b...0e6`, as in the record |
| `check-web-audioworklet.sh` | exit 0; `f32x4_arith=9350 kernels=12`; the kernel is not linked yet |
| `check-browser-expected-resources.py --artifacts` | exit 0 |
| `test-web-audioworklet.sh` | exit 0 |
| `run-aarch64-tests.sh debug` | **not run**: no arm64 host or qemu here. CI `aarch64-debug` at batch push. The NEON codegen was inspected statically, as above |

## My mutations

Each mutation was applied alone in a scratch copy, and each ran `cargo test -p lane --test route_ramp`
in debug.

| # | Mutation | Result | First failure |
|---|---|---|---|
| V1 | the snap vector rounded up, so frames past `length` in that vector compute `c(k)` instead of taking `target` | RED: gate 1, retarget | `width 4, length 37, from position 30 ... left frame 7` |
| V2 | **FMA:** the body's coefficients fused (`f32::mul_add` per lane) | RED: gate 1, retarget | `width 1, length 37 ... left frame 8`, one ulp |
| V3 | **recursive drift:** coefficients carried `c += WIDTH * step` per chunk | RED: gate 1, retarget | `width 1 ... left frame 3`, one ulp |
| V4 | **`k` off by one at lane 3 of a vector** (iota `[1, 2, 3, 5, ..]`) | RED: gate 1, retarget, width 4 | `width 4 ... left frame 3` |
| V5 | **`k` off by one at the vector boundary** (advance `WIDTH + 1`) | RED: gate 1, retarget | `width 4 ... left frame 4` |
| V6 | snap one frame early | RED: gate 1, retarget | `width 1, length 37, from position 24 ... left frame 11` |
| V7 | non-finite branch checks `is_nan` only, so an infinite step gets through | RED: gate 2 alone | `coefficients_at(0)` |
| V8 | a lerp law, `start + (k / length) * (target - start)` | RED: gate 1, retarget | `width 1 ... left frame 2` |
| V9 | the ramp body never vectorised (`vectored = 0`) | GREEN (performance only; bits cannot move) | NIT-1 |
| V10 | the settled whole vectors run scalar | GREEN (performance only) | NIT-1 |
| V11 | the tail freezes `k` at its first frame | RED: gate 1, retarget | `width 4 ... from position 30 ... left frame 5` |

## Test value (one sentence each)

- **`the_kernel_is_the_indexed_ramp_law_at_every_width`:** it is red if the frame index, the snap
  frame, the outlined tail, the rounding or the fusing, or the width-independence drifts. V1-V6, V8
  and V11 are all red here. **PASS.**
- **`a_retarget_starts_from_the_exact_current_coefficients`:** it is red if `coefficients_at` stops
  being the kernel's `c(k)`, for example if `c(0)` is computed or `coefficients_at` is fused.
  M6 and M7 are red here alone. **PASS.**
- **`an_overflowing_difference_is_a_step_to_the_target`:** it is red if an overflowing difference
  yields an infinite step. M5 and my V7 (`is_nan`-only) are red here alone. **PASS.**
- **`a_settled_ramp_is_the_static_mix`:** it is red if the settled frames stop being
  `mix2x2_block`'s own bits, for example a private settled loop that matches the oracle while the
  static mix differs (M11). **PASS.** NIT-3 asks for one cleanup in it.
