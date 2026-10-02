# Ramp a send's coefficients with the indexed ramp kernel

Slice 21 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K3. It touches only `crates/lane`, so it may be built beside slices 18a-20; it
merges in K3's order.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

The engine gains the one new render kernel live sends need: a 2x2 route mix whose four coefficients
ramp from their current values to a new target over a declicked window, vectorised over frames, with
bits that do not depend on lane width. *Ramp live send coefficients on the render plane* (#1220) drives it
from each live route's control queue.

The law is the **indexed ramp** (DESIGN P5, 5.7): a coefficient is a pure function of the frame's
index within the ramp, so the ramp vectorises over frames, any later fused or folded traversal can
compute it in any order, and a settled ramp lands exactly on its target. It is not master-plan D11,
which stays the law for faders, matrices and effects.

## Context (verified on `fe8ac679`)

Nothing in batches K1 or K2 touches these anchors.

- **The route mix today** is `mix2x2_block<L: Lane>` (`crates/lane/src/kernels.rs:1009-1050`):
  coefficients `c = [ll, lr, rl, rr]` already carry the route's gain, and per frame
  `l' = fma(lr, r, ll * l)`, `r' = fma(rr, r, rl * l)`, both from the frame's original `l` and `r`.
  It is `#[inline(always)]`, vectorised over frames at `L`, and finished by the same body at
  `L = f32`, so it is width-independent. `Lane::fma` is two IEEE roundings on every target (never
  fused; `scripts/check-unfused-seal.sh` holds that seal).
- **D11, not this law.** `ramp_block` (`kernels.rs:838-878`) and the builtins'
  `gain_mute_ramp_block` (`crates/lane/src/kernels/builtins.rs:185-202`) carry `g = g + step` from
  frame to frame (`current = select(done, target, current + step)`); their bits depend on the frame's
  history, not only its index. D11 is documented at `docs/BUILTINS_AND_METERING_V1.md:69-74`.
- **Lane types.** `lane::Simd4` is `wide::f32x4` and is always compiled; `lane::Simd8` is
  `wide::f32x8` under `target_feature = "avx2"` (`crates/lane/src/lib.rs:142`, `:161`); `f32`
  implements `Lane` (`crates/lane/src/scalar.rs`). The `Lane` trait has `splat`, `load`, `store`,
  `add`, `sub`, `mul`, `div` and `fma` (trait at `crates/lane/src/lib.rs:204`; `fma` at `:301`). Lane tests live in
  `crates/lane/tests/` and instantiate each width explicitly (for example `fader_matrix.rs`).
- **The browser kernel-shape rule.** `check-web-audioworklet.sh` runs
  `scripts/check-web-audioworklet-callgraph.py --kernel-shape --kernel-pattern '4wide6f32x4' --kernel-min 11`
  (`check-web-audioworklet.sh:470-471`). Its rule 3 requires every `4wide6f32x4` function with
  `f32x4` arithmetic to have more vector than scalar instructions. An `f32` tail written inside an
  `L`-generic body is unrolled into that function and fails the rule, so tails are outlined into a
  non-generic `#[inline(never)]` function: the #926 lesson (`67649092`;
  `crates/graph/tests/MUTATIONS.md:339`, row 926-7). This kernel has no caller until the next slice,
  so the artifact gate that sees it belongs to *Ramp live send coefficients on the render plane*.

## Decisions frozen for this slice

- **D1. The ramp state and the law**, in `crates/lane/src/kernels.rs`, beside `mix2x2_block`:

  ```rust
  pub const INDEXED_RAMP_LENGTH_MAXIMUM: u32 = 1 << 22;
  #[derive(Clone, Copy, Debug, PartialEq)]
  pub struct IndexedRamp { pub start: [f32; 4], pub step: [f32; 4], pub target: [f32; 4], pub length: u32 }
  impl IndexedRamp {
      /// A settled ramp: every index yields `target`.
      pub const fn settled(target: [f32; 4]) -> Self;
      /// A ramp from `start` to `target` over `length` frames.
      pub fn new(start: [f32; 4], target: [f32; 4], length: u32) -> Self;
      /// `c(k)`, exactly as the kernel computes it for frame index `k`.
      pub fn coefficients_at(&self, k: u32) -> [f32; 4];
  }
  ```

  - `c(0) = start`; `c(k) = round(round(k * step) + start)` for `1 <= k < length`, evaluated as
    `Lane::fma(k, step, start)` with `k` converted exactly to `f32` (`k <= 2^22`); `c(k) = target`
    for `k >= length`, assigned, never computed.
  - `new` computes `step = round(round(target - start) / length)` once, per coefficient, when
    `length > 0`. `length == 0` is a step: every `k` yields `target`.
  - **Non-finite difference** (VERIFY-2 MINOR 3). If `round(target - start)` is not finite for any of
    the four coefficients, `new` returns a step (`length = 0`) to `target`. The branch lives in
    `IndexedRamp::new`, once per record, never in the per-frame body, so no step is ever NaN or
    infinite.
  - `new` `debug_assert!`s `length <= INDEXED_RAMP_LENGTH_MAXIMUM`; the caller refuses longer
    lengths before a record exists (*Ramp live send coefficients on the render plane* re-exports the
    bound as `graph::ROUTE_RAMP_LENGTH_MAXIMUM`).
- **D2. The kernel.**

  ```rust
  #[inline(never)]
  pub fn route_mix_ramp_block<L: Lane>(left: &mut [f32], right: &mut [f32], ramp: &IndexedRamp, position: u32);
  ```

  - Frame `f` (0-based) of the block uses `k = position + f + 1`.
  - The frames with `k < length` run the ramp body: the D3 mix with per-frame coefficients
    `c(k)`, in `mix2x2_block`'s order (`l' = fma(c_lr, r, c_ll * l)`, `r' = fma(c_rr, r, c_rl * l)`).
  - The ramp body is vectorised over frames: a frame-index vector `k0 + [1, ..., L::WIDTH]` (loaded
    from a constant iota array and advanced by `L::WIDTH` per chunk; every value is an integer below
    `2^24`, so the adds are exact) yields each coefficient per frame lane with one `fma`.
  - The ramp frames that do not fill a whole vector run a separate **non-generic
    `#[inline(never)]`** `f32` function with the same arithmetic.
  - The remaining frames (`k >= length`) are `mix2x2_block::<L>` on `ramp.target`, unchanged.
  - `#[inline(never)]` on the generic kernel keeps each instantiation a named function, so the
    browser's kernel-shape gate can see the `f32x4` one in the next slice.
- **D3. Width independence** is by construction: `c(k)` is a pure function of `k`, and the snap
  frame is decided by `k`, not by a running value.
- **D4. Exactness after the ramp.** The settled coefficients are `target`, assigned. A route that has
  ramped and settled mixes exactly what `mix2x2_block` mixes with `target`.

## Deliverables

1. `INDEXED_RAMP_LENGTH_MAXIMUM`, `IndexedRamp` and `route_mix_ramp_block` with its outlined `f32`
   tail, in `crates/lane/src/kernels.rs` (D1, D2), with doc comments that state the law and why it is
   not D11.
2. A new lane test file, `crates/lane/tests/route_ramp.rs` (gates 1-3).
3. `crates/lane/tests/MUTATIONS.md` rows for the red mutations named in the gates.

## Authorized paths

- `crates/lane/src/kernels.rs`
- `crates/lane/tests/route_ramp.rs` (new), `crates/lane/tests/MUTATIONS.md`
- this spec

## Non-goals

- No caller, no queue, no record type, no activity rule (*Ramp live send coefficients on the render
  plane*).
- No change to `mix2x2_block`, to D11 or to any fader, matrix or effect ramp.
- No fused reduction (DESIGN O1).

## Hazards

- **Bits after the ramp.** The settled coefficients must be `target`, assigned, never
  `start + length * step`.
- **The snap frame inside a vector.** The frame where `k` reaches `length` can fall inside a chunk.
  The ramp/settled split is by frame count, decided from `position` and `length` before the loop.
- **Kernel shape.** An `f32` tail inside the generic body unrolls into the `f32x4` instantiation and
  fails rule 3 once the next slice links the kernel into the browser module. Keep it outlined and
  non-generic.
- **Index precision.** `k` must convert to `f32` exactly; that holds because `length <= 2^22`.

## Objective gates

1. **The law, at every width.** In `crates/lane/tests/route_ramp.rs`, `route_mix_ramp_block` at
   `lane::Simd8` (under `#[cfg(target_feature = "avx2")]`), at `lane::Simd4` and at `f32` equals,
   bit for bit, an independent scalar oracle that computes `c(k)` per frame with two explicit
   roundings and then the D3 mix. Cases:
   - `length` in {0, 1, 37, 4800, 2^22}, with ramps that end mid-block and mid-vector;
   - block lengths 128, 125 and 1 (whole vectors, a tail, a tail only);
   - a retarget mid-ramp, `IndexedRamp::new(current, target, length)` with
     `current = coefficients_at(position)` of the previous ramp;
   - planes with signed zeros, subnormals and non-finite samples (NaNs folded in the comparison).

   *Test value: it turns red if the frame indexing, the outlined tail, the snap frame or the rounding
   order drifts, or if the result depends on lane width. No existing kernel ramps coefficients as a
   function of the frame index.*
2. **No NaN step.** `IndexedRamp::new` with a start of `-3e38` and a target of `3e38` in one
   coefficient (their difference overflows) yields a ramp whose every `coefficients_at(k)` is
   `target`, and the kernel renders exactly `mix2x2_block` with `target`.
   *Test value: it turns red if an overflowing difference produces an infinite step and a NaN
   coefficient (VERIFY-2 MINOR 3).*
3. **A settled ramp is the static mix.** For random targets, a ramp whose `length` has elapsed
   renders bit-identically to `mix2x2_block` with the same target, at every width, including
   further blocks at `position == length` (position saturation; this is the one place it is
   tested).
   *Test value: it turns red if the settled remainder is computed (`start + k * step`) instead of
   assigned, which would leave a live route's bits off a freshly prepared plan's.*
4. **Policy and the DSP command.**
   - the test-debug-b command (DESIGN section 7), which runs `lane`'s tests
   - `bash scripts/check-lane-policy.sh` and `bash scripts/test-lane-policy.sh`
   - `bash scripts/check-unfused-seal.sh` and `bash scripts/check-unfused-seal.sh --self-test`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` job at the K3
     push (`lane` is in its list)

No allocation gate: the kernel takes borrowed planes and allocates nothing by construction.

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name, with its one-sentence test-value answer.
- The `MUTATIONS.md` rows with their observed red results (at least: tail inlined into the generic
  body; settled coefficients computed instead of assigned; `k` off by one).

## Dependencies

- *Build submix strips and bus taps in the SDK and teach agents to author them* (#1205, batch K1 closed and
  pushed). No code dependency within K3.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- "Bit-identical" gates are hard stops. NaNs are folded (decision 10).
- Only `crates/lane` names `wide` or intrinsics.
- One kernel shape for every target, generic over `Lane`. Lane width is keyed on target features,
  never on `target_arch`.
- No scalar loop where the vector form exists: the ramp is vectorised over frames.
- A test that greps source or prose is refused.
- Commit on the K3 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each.
