//! `Lane`: the workspace SIMD foundation and its pinned per-operation numeric contract.
//!
//! This crate is the single home of every SIMD vocabulary type and of the per-operation numeric
//! contract they all obey. Everything else in the workspace is written once,
//! generic over [`Lane`], and instantiated per width; lane identity is therefore a property of the
//! code rather than of a fixture corpus (master plan for issue #83, §1 and §3).
//!
//! # What is pinned
//!
//! * The semantics of every operation (master plan §3.2). The scalar [`Lane`] implementation for
//!   [`prim@f32`] is the oracle and is written operation by operation to that table, never to a
//!   `std` convenience: `f32::max`, for example, has a different NaN rule than decision D8 and is
//!   forbidden on any render path.
//! * `max`/`min` are `select(a > b, a, b)` / `select(a < b, a, b)` (D8). The trait default is that
//!   expression, so a backend that overrides it must reproduce it bit for bit and never substitute
//!   an IEEE `maximum`: `x86` and wasm `simd128` each have one instruction with exactly the D8
//!   rule and use it, AArch64 has none and keeps the default
//!   (`wide_impl.rs`, "Lowerings chosen per backend").
//! * Fusion exists **nowhere** (issue #163 phase 2, amending D3). [`Lane::fma`] keeps its name but
//!   is `(a * b) + c` with two roundings on every backend. Rust never contracts `a * b + c`, so
//!   the absence of fusion is mechanically checkable and is sealed:
//!   `scripts/check-unfused-seal.sh` fails if `mul_add` or a fused intrinsic appears anywhere in
//!   workspace source, and `scripts/check-lane-policy.sh` keeps raw intrinsics inside this crate.
//! * Denormal handling is one mechanism on every target: [`flush`] with [`FLUSH_EPS`] (D7).
//!
//! # Backends
//!
//! Exactly three (D4): `Scalar` ([`prim@f32`], `WIDTH = 1`, the oracle), `Simd4`
//! ([`wide::f32x4`]) and `Simd8` ([`wide::f32x8`]). The operation bodies come from the `wide`
//! crate, which selects its own backend at **compile time**, so there is no runtime SIMD dispatch:
//! native `x86_64` builds are pinned to `x86-64-v3` by `.cargo/config.toml`, this crate refuses to
//! compile on `x86` without `avx2` and `fma`, and a host attests the CPU once at boot with
//! [`attest_host`]. `wide` is a vocabulary, not a semantics authority: its `max`, `min` and
//! `mul_add` differ per target and are never forwarded — `mul_add` least of all, since a fused
//! lowering on one target and an unfused one on another is precisely the split issue #163 phase 2
//! removed. Its `fast_max`/`fast_min`, which are the bare instruction with no fix-up, *are* used
//! on the two targets where that instruction is D8 exactly; the difference between borrowing a
//! semantics and borrowing an instruction is argued case by case in `wide_impl.rs`.
//!
//! The `f32` lane is live code at every width: it is each effect's one-lane leg and every frame
//! loop's tail. Selecting it for a *whole plan* is not: `Backend::Scalar` exists only with this
//! crate's `test-support` feature, as the oracle vector banking is tested against (issue #1059).
//! `Simd8` exists only where `avx2` is enabled: the 4-lane (NEON/simd128) builds never select it,
//! so they do not compile it (issues #1110 and #1112; `Simd8`'s own documentation says how).
//!
//! # `f64` lanes
//!
//! The owner ruling of 2026-09-26 -- "We shouldn't leave scalar arithmetic where vector arithmetic
//! is possible" -- gave this crate `f64` lanes (issue #949). They are two separate traits beside
//! [`Lane`]: [`LaneF64`], `W` lanes of IEEE binary64 with `load`, `store`, `add` and `mul` and
//! nothing else, and [`Widen`], which converts each backend's `f32` lanes to its `f64` companion
//! (`f64`, [`wide::f64x4`] and [`wide::f64x8`]). Their contract is pinned the same way: exact
//! binary64, never fused, with scalar `f64` as the oracle at every width. The `f32` [`Lane`]
//! contract is unchanged by them.
//!
//! # Realtime rules
//!
//! Every operation and every kernel body is `#[inline(always)]`, allocation-free, branch-free per
//! sample, and validated with `debug_assert!` only: block shapes are validated once at plan
//! preparation, never per block on the render thread.

#![no_std]
// `std` is used for exactly two things, neither of them on a render path: the `f32::floor` and
// `f32::sqrt` inherent methods (not available in `core` on the pinned toolchain), and
// `is_x86_feature_detected!` inside `attest_host`. The crate stays `#![no_std]` so
// that `Vec`, `String` and the rest of the allocating prelude are not reachable by accident.
extern crate std;

// Master plan #83 D4: native x86 is x86-64-v3, pinned at compile time by `.cargo/config.toml` and
// attested at boot by `attest_host`. Without the pin, `wide` silently lowers `f32x8` to two SSE2
// `m128` values, which halves the production width and would break D5 without any test noticing.
// (The `mul_add` half of this hazard is gone since issue #163 phase 2: the contract is unfused
// everywhere, so no lowering choice can change a rendered bit.) Refusing to compile is the guard
// (master plan §11).
// `cfg(doc)` is excluded because `rustdoc` does not receive `[target.*] rustflags` from
// `.cargo/config.toml`, and `RUSTDOCFLAGS` on the command line replaces any `rustdocflags` set
// there. Documentation is not a build artefact, so the guard has nothing to protect in that pass.
#[cfg(all(
    target_arch = "x86_64",
    not(all(target_feature = "avx2", target_feature = "fma")),
    not(doc)
))]
compile_error!(
    "lane requires x86-64-v3: build with -C target-feature=+avx2,+fma (the workspace \
     .cargo/config.toml sets this). Master plan #83 D4: there is no runtime SIMD dispatch and no \
     silent scalar fallback."
);

// Issue #1041: the engine builds for 64-bit targets only and refuses every other target here.
// Owner ruling 2026-09-28 (`docs/rulings/engine-footprint-2026-09-28.md`), "Yes let's go 64bit
// only": iOS arm64 and Android arm64-v8a ship, 32-bit ARM (armeabi-v7a) does not. Before this
// guard `Backend::current()` gave `Scalar` to every architecture it did not name, so a 32-bit ARM
// build compiled and ran the whole-plan scalar path without a word. The supported set is x86-64
// (its AVX2+FMA pin is the guard above), AArch64, and wasm32 with `simd128`. The two native arms
// also require 64-bit pointers, which keeps out the ILP32 ABIs that share an architecture name
// (`x86_64-unknown-linux-gnux32`, `arm64_32-apple-watchos`).
//
// wasm32 *without* `simd128` is refused too (owner ruling 2026-09-28, decision 7; issue #1062):
// the one shipped wasm artifact is `simd128` (owner decision W4-D1), and the scalar-wasm CI legs
// that once kept a scalar build compiling are retired. `docs/TARGET_MATRIX.md` names the checks
// that took over each of their claims.
//
// Unlike the guard above this one needs no `not(doc)` escape. Its outcome depends on
// `target_arch`, `target_pointer_width` and, on wasm32, `simd128`; the first two rustdoc sees
// correctly, and a wasm32 documentation pass passes `-C target-feature=+simd128` in `RUSTDOCFLAGS`
// exactly as a wasm32 build passes it in `RUSTFLAGS`, because neither comes from
// `.cargo/config.toml`.
#[cfg(not(any(
    all(target_arch = "x86_64", target_pointer_width = "64"),
    all(target_arch = "aarch64", target_pointer_width = "64"),
    all(target_arch = "wasm32", target_feature = "simd128")
)))]
compile_error!(
    "lane supports 64-bit targets only: x86-64 (with AVX2 and FMA), AArch64 (iOS arm64, Android \
     arm64-v8a) and wasm32 with simd128. 32-bit ARM (armeabi-v7a), 32-bit x86 and every other \
     target are refused, with no silent scalar fallback (owner ruling 2026-09-28, \
     docs/rulings/engine-footprint-2026-09-28.md; issue #1041)."
);

mod backend;
mod bits;
mod f64_lane;
pub mod fpenv;
pub mod kernels;
mod scalar;
mod simd4;
// Issues #1110 and #1112: eight lanes exist only where they can run. See `Simd8` below.
#[cfg(target_feature = "avx2")]
mod simd8;
pub mod softfma;
mod wide_impl;

pub use backend::{Backend, HostAttestation, attest_host};
pub use f64_lane::{LaneF64, Widen};
pub use fpenv::{CanonicalFpEnv, FpEnvironmentRejection, attest_fp_environment};

/// The 4-lane (NEON/simd128) production width: a NEON `float32x4_t` or a wasm `v128`.
///
/// Re-exported under a neutral name so that no crate outside this one has to name `wide`
/// (master plan §4.3: effects are generic over [`Lane`] and never name a vector library or an
/// intrinsic). `scripts/check-lane-policy.sh` enforces that.
pub use wide::f32x4 as Simd4;

/// The 8-lane (AVX2) production width: one `__m256`.
///
/// Re-exported under a neutral name, like [`Simd4`].
///
/// # Only where `avx2` is enabled (issues #1110 and #1112)
///
/// A 4-lane (NEON/simd128) build runs four lanes and nothing else ([`Backend::current`] is `Simd4`
/// there), so in it this type, its [`Lane`] and [`Widen`] implementations and `Backend::Simd8` do
/// not exist: not in the browser module, and not in the iOS or Android library. One predicate,
/// `#[cfg(target_feature = "avx2")]`, removes them, on the items themselves, and it is the one
/// [`Backend::current`] selects this width by; a crate that names any of them in code a 4-lane
/// build compiles fails to compile, so an eight-lane instantiation cannot reach one by accident.
/// The lane width is the only thing that differs: every kernel is still one generic body.
/// Downstream, the width is chosen through `effect_contract::BankWidth`, whose `Eight` variant
/// carries the same predicate, and through the `effect_contract::match_bank_width!` dispatch.
/// The width follows the target features, not the architecture's name (owner, issue #1112).
#[cfg(target_feature = "avx2")]
pub use wide::f32x8 as Simd8;

/// Magnitude below which a recursive state word is flushed to `+0.0` by [`flush`].
///
/// `1.0e-20` is about `2^-66`; `f32` subnormals start at `2^-126`, so the flush band strictly
/// contains the band hardware FTZ/DAZ acts on. Every recursive state word this law is applied to is
/// therefore FTZ-inert, which gate G6 (`tests/g6_ftz_inert.rs`) proves against a deliberately
/// unflushed control arm.
///
/// It is the *state* law, and issue #144's full-corpus reproducer showed it is not the whole story:
/// 69-70 of the 331 cross-target corpus rows still moved under hardware FTZ+DAZ, because a
/// transient intra-block denormal in a feed-forward lane, a scalar math kernel or an effect chain
/// is never a flushed state word. Denormal correctness for the whole render is owned by
/// [`fpenv::CanonicalFpEnv`], which pins the floating-point environment at every native render
/// entry (issue #146); the D7 flush remains the law that keeps recursive state from *reaching* the
/// subnormal range in the first place.
pub const FLUSH_EPS: f32 = 1.0e-20;

/// Magnitude below which *both* words of a two-word recursive state are flushed together by
/// [`flush_pair`], on a frame whose effect input has been exactly zero for at least
/// [`silence_frames`] frames (issue #1328, decision 15 D15-4(a), amendment A9).
///
/// The per-word law alone perturbs each word by at most [`FLUSH_EPS`] per step, and that is enough
/// to stall a decaying second-order state short of rest: a period-2 limit cycle where the input
/// filter's `c1` rounds to `1.0`, or a fixed point where `2 * d2` is below half an ulp of `ic2`
/// (an EQ low shelf). Such a trajectory can only stall inside a ball of radius
/// `κ·√2·FLUSH_EPS / (1 - ρ - 6·2^-24·κ)` around the origin (κ the eigenvector condition number
/// of the step matrix, ρ its spectral radius). Over the designable sections the largest radius is
/// the EQ's (about `3.4e-15`), so `1.0e-14` clears every domain by about 3x; the derivation and the
/// recomputed per-domain radii are in `dsp-research/filters.md`.
pub const REST_EPS: f32 = 1.0e-14;

/// How long an effect input must have been exactly zero, on a lane, before [`flush_pair`]'s joint
/// rule may fire on that lane (issue #1328, amendment A9): `N_SILENCE`, stated in time as the exact
/// ratio [`SILENCE_TIME_FRAMES`]` / `[`SILENCE_TIME_RATE`] seconds, `4096 / 48000` s (85 1/3 ms).
///
/// Silence is a time property. No rule that looks at one sample can tell "a tiny sample, then a
/// zero, then a tiny sample" from "a tiny sample, then silence", and a joint flush that fires on
/// the first kind erases the state a sparse signal is building (amendment A8's sparse-input dead
/// zone). So each effect input channel keeps one counter word per lane, [`silence_step`], and the
/// joint rule is armed only on a lane whose input has been `+0.0` or `-0.0` for this long: a signal
/// whose non-zero samples are closer together than this never reaches the rule at all, at any
/// level, and gets the effect's whole response.
///
/// A time, not one frame count, because the measurement said so: at the ruling's first choice,
/// 1,024 frames at every rate, a sparse signal whose gaps just exceed the window loses more than
/// one tail's worth (four +24 dB 10 Hz shelves: 1.1e-9 to 1.3e-9 at the four rates, against the
/// 3.48e-10 one tail can lose), and the window needed to bring it back to one tail is a time
/// (about 85 ms at every rate), not a count. [`silence_frames`] derives each rate's frames; the
/// measurement and the residual at each launch rate are in `dsp-research/filters.md` (numerical
/// limits).
pub const SILENCE_TIME_FRAMES: u32 = 4_096;

/// The rate [`SILENCE_TIME_FRAMES`] is counted at: the silence time is `4096 / 48000` seconds.
pub const SILENCE_TIME_RATE: u32 = 48_000;

/// `N_SILENCE` at `sample_rate_hz`: the silence time ([`SILENCE_TIME_FRAMES`]` / `
/// [`SILENCE_TIME_RATE`] seconds) in frames, rounded up -- 3,764 at 44.1 kHz, 4,096 at 48 kHz,
/// 7,527 at 88.2 kHz and 8,192 at 96 kHz.
///
/// Exact integer arithmetic. Every result up to the largest `u32` rate is far below the counter's
/// `2^24` saturation, so the comparison in [`silence_step`] is exact, and a value of `1` or more
/// for every rate above zero.
#[must_use]
pub const fn silence_frames(sample_rate_hz: u32) -> u32 {
    let numerator = sample_rate_hz as u64 * SILENCE_TIME_FRAMES as u64;
    numerator.div_ceil(SILENCE_TIME_RATE as u64) as u32
}

/// Flushes lanes whose magnitude is below [`FLUSH_EPS`] to exactly `+0.0`.
///
/// `flush(x) = andnot(abs(x) < FLUSH_EPS, x)` (D7): three operations, applied to each recursive
/// state word once per sample inside the kernel. NaN passes through unchanged (`abs(NaN) < eps` is
/// false under the ordered compare) and is caught by the once-per-block boundary check; `-0.0`
/// becomes `+0.0`.
#[inline(always)]
pub fn flush<L: Lane>(x: L) -> L {
    flush_with(x, L::splat(FLUSH_EPS))
}

/// [`flush`] with [`FLUSH_EPS`] supplied by the caller as a word, `flush_eps`, which must be
/// [`FLUSH_EPS`] on every lane: for a kernel that loads its constants from memory instead of
/// splatting them in the frame loop (known defect #1018; [`kernels::svf_step_when`]).
#[inline(always)]
pub fn flush_with<L: Lane>(x: L, flush_eps: L) -> L {
    x.andnot(x.abs().lt(flush_eps))
}

/// One frame of an effect input's silence counter (issue #1328, amendment A9): advances the
/// counter on the frame's input `x` and returns the frame's **rest threshold** for
/// [`flush_pair`].
///
/// ```text
/// run  = (x == 0) ? run + 1 : 0
/// rest = (run < armed_after) ? +0.0 : REST_EPS
/// ```
///
/// `armed_after` is `N_SILENCE` for the effect's rate, [`silence_frames`], splatted once per block
/// by the caller.
///
/// One counter word per lane per effect input channel: the builtin input stage's input, the
/// parametric EQ's input and the multiband compressor's input each own one per channel, and every
/// SVF section of that effect channel reads the threshold of the effect's own input for the same
/// frame. `x == 0` is the IEEE ordered equality, so `+0.0` and `-0.0` count and a NaN, an infinity
/// or a subnormal resets the run. `run + 1` is an `f32` addition: it is exact up to `2^24` and
/// rounds `2^24 + 1` to `2^24` (ties to even), so the counter saturates by itself and never wraps.
/// Five operations (`eq`, `add`, `select`, `lt`, `andnot`), once per frame per channel whatever
/// the number of sections.
///
/// The threshold is a lane word, not a mask, so that [`flush_pair`] spends one compare on it: a
/// lane whose input is live carries `+0.0`, below which no magnitude lies.
#[inline(always)]
pub fn silence_step<L: Lane>(x: L, run: &mut L, armed_after: L) -> L {
    silence_step_hoisted(x, run, armed_after, L::splat(1.0), L::splat(REST_EPS))
}

/// [`silence_step`] with its two constants, `one = splat(1.0)` and `rest_on = splat(REST_EPS)`,
/// supplied by the caller: a kernel splats them once per block, outside its frame loop, because a
/// splat inside the loop is a `memset_pattern16` call per frame on Apple targets (#1018).
#[inline(always)]
pub fn silence_step_hoisted<L: Lane>(x: L, run: &mut L, armed_after: L, one: L, rest_on: L) -> L {
    let counted = L::select(x.eq(L::zero()), run.add(one), L::zero());
    *run = counted;
    rest_on.andnot(counted.lt(armed_after))
}

/// `true` when some lane's silence counter `run` can reach `armed_after` ([`silence_frames`])
/// within a block of `frames` frames, so that some frame of the block may carry an armed rest
/// threshold; `false` guarantees every threshold of the block is `+0.0` whatever the input
/// (issue #1328, amendment A9).
///
/// A frame `f` (from `0`) counts at most `run + f + 1`, so the block can arm only if
/// `run + frames >= armed_after`, tested as `run >= armed_after - frames` (both sides exact
/// integers). One compare and one `mask_any`, once per block per channel: the kernels then run the
/// joint rule's arithmetic only on a block it can act on (the builtin input chain through
/// [`kernels::svf_step_when`]; the parametric EQ writes a rest plane only for such a block and
/// loads the shared all-`+0.0` plane for every other).
#[inline(always)]
pub fn silence_armable<L: Lane>(run: L, frames: usize, armed_after: L) -> bool {
    L::mask_any(run.ge(armed_after.sub(L::splat(frames as f32))))
}

/// The joint flush of a two-word recursive state `(n1, n2)` (issue #1328), armed by the rest
/// threshold `rest` that [`silence_step`] gave the effect input's frame (amendment A9).
///
/// Each word follows [`flush`]'s per-word law and, in addition, when both magnitudes are below the
/// lane's threshold the pair is zeroed together:
///
/// ```text
/// a1, a2 = |n1|, |n2|
/// joint  = max_u32(a1, a2) < rest
/// ic1    = andnot(n1, (a1 < FLUSH_EPS) | joint)
/// ic2    = andnot(n2, (a2 < FLUSH_EPS) | joint)
/// ```
///
/// `rest` is [`REST_EPS`] on a lane whose effect input has been exactly zero for at least
/// `N_SILENCE` frames ([`silence_frames`]), and `+0.0` on every other lane, where `joint` is false
/// for every pair: no magnitude, and no NaN, compares below `+0.0`. So while the effect's input is
/// live -- any non-zero sample within the last `N_SILENCE` frames, however small -- each word
/// follows the per-word law bit for bit, a section applies its whole response to whatever it is
/// given, and a chain of boosting sections applies every boost. Only silence that has lasted
/// `N_SILENCE` frames lets the pair rule end a decay.
///
/// `max_u32` of the two magnitudes is their larger one, bit for bit: the magnitude bits of a
/// non-negative `f32` order like the values, and every NaN's magnitude bits sort above `+inf`'s.
/// So `joint` is "both below `rest`" with one compare, and a NaN in either word makes `max_u32` a
/// NaN, which fails the ordered compare: a NaN passes through both rules and reaches the
/// once-per-block boundary check, and one word at or above the threshold keeps the pair on the
/// per-word law, so an audible partner word is never zeroed. `-0.0` becomes `+0.0`. Never
/// [`Lane::max`], which is `select(gt)` and would drop a NaN.
///
/// Ten operations for the two words (two `abs`, one `max_u32`, three compares, two mask `or`, two
/// `andnot`), against six for two [`flush`] calls; the threshold's five are paid once per frame
/// per channel by [`silence_step`]. Branch-free, one generic body at every width.
#[inline(always)]
pub fn flush_pair<L: Lane>(n1: L, n2: L, rest: L) -> (L, L) {
    flush_pair_with(n1, n2, rest, L::splat(FLUSH_EPS))
}

/// [`flush_pair`] with [`FLUSH_EPS`] supplied by the caller as a word, `flush_eps`, which must be
/// [`FLUSH_EPS`] on every lane ([`flush_with`]'s reason).
#[inline(always)]
pub fn flush_pair_with<L: Lane>(n1: L, n2: L, rest: L, flush_eps: L) -> (L, L) {
    let a1 = n1.abs();
    let a2 = n2.abs();
    let joint = a1.max_u32(a2).lt(rest);
    (
        n1.andnot(L::mask_or(a1.lt(flush_eps), joint)),
        n2.andnot(L::mask_or(a2.lt(flush_eps), joint)),
    )
}

/// One width of `f32` lanes with pinned IEEE-754 semantics.
///
/// Implemented by [`prim@f32`] (`WIDTH = 1`, the oracle), [`wide::f32x4`] and [`wide::f32x8`].
/// Every method is `#[inline(always)]` in every implementation. The surface is deliberately
/// minimal (master plan §3.1): no horizontal operations, no gather, no reciprocal or reciprocal
/// square-root approximations, and no runtime dispatch.
///
/// # Numeric contract
///
/// `add`, `sub`, `mul`, `div` and `sqrt` are IEEE-754 round-to-nearest-even; `fma` is
/// `(a * b) + c` and rounds twice (issue #163 phase 2);
/// `neg` and `abs` are sign-bit operations, never `0.0 - x`; comparisons are ordered (NaN compares
/// false); `select` is bitwise per lane. A [`Lane::Mask`] lane is either all zero bits or all one
/// bits — masks are produced only by the comparison and mask operations of this trait.
pub trait Lane: Copy + Send + Sync + 'static {
    /// Number of `f32` lanes in one value.
    const WIDTH: usize;

    /// How many cascade sections [`kernels::svf_cascade_interleaved`] fuses into one frame loop on
    /// this backend (issue #163 phase 3).
    ///
    /// This is the **only** tuned constant in the crate, and it is a *schedule* choice, not a
    /// numeric one: every value produces the same bits, because every value runs the same frozen
    /// operation order on the same values (gate G2,
    /// `tests/g2_kernel_identity.rs::interleaved_cascade_equals_a_chain_of_blocks`). It exists
    /// because a TPT recurrence is latency-bound, not width-bound -- `Simd4` and `Simd8` take the
    /// same wall time per chain-frame when one chain runs alone -- so the only way to fill the
    /// vector units is to keep several independent recurrences in flight, and the useful number is
    /// bounded above by the architectural register file.
    ///
    /// Chosen by the measured sweep in `tests/b2_interleave.rs`, over two interleaved streams (a
    /// bank's two channels) at every depth that divides a four-section cascade, on the #163 bench
    /// host (Zen 5, `x86-64-v3`), against the four-serial-blocks-per-channel shape the EQ ran
    /// before:
    ///
    /// | backend  | depth 1 | depth 2    | depth 4    | chosen |
    /// |----------|---------|------------|------------|--------|
    /// | `Scalar` | 1.622x  | 1.774x     | **2.092x** | 4      |
    /// | `Simd4`  | 1.800x  | **2.653x** | 2.085x     | 2      |
    /// | `Simd8`  | 1.721x  | **2.453x** | 1.889x     | 2      |
    ///
    /// The vector backends turn over at depth 4 because two streams times four sections is eight
    /// live integrator pairs -- sixteen vector registers -- plus the coefficient words, which
    /// spills a sixteen-register file. `Scalar` keeps its state in single `f32` slots and does not.
    ///
    /// The same sweep measured what *fusing independent banks* would add on top, by running four
    /// and eight streams instead of two: at `Simd8` the best cross-bank cell is 2.694x against
    /// 2.453x here, a 1.10x margin for a fusion that would have to break the effect contract's
    /// `dyn` boundary. `artifacts/issue163-phase3/` records that as a bounded, measured null.
    const SVF_CASCADE_DEPTH: usize;

    /// Result of a comparison: per lane either all zero bits or all one bits.
    ///
    /// `Send`, like the lane itself, so a derived mask can be held in an effect's state (the EQ's
    /// per-section dry masks, issue #1328).
    type Mask: Copy + Send;

    /// Broadcasts one value to every lane.
    fn splat(x: f32) -> Self;

    /// All lanes `+0.0`.
    fn zero() -> Self;

    /// Reads exactly [`Lane::WIDTH`] values from the front of `src`.
    ///
    /// # Panics
    ///
    /// Panics if `src` is shorter than [`Lane::WIDTH`]. Block shapes are validated once at plan
    /// preparation, so this bounds check is a debugging aid, not a render-path branch.
    fn load(src: &[f32]) -> Self;

    /// Writes exactly [`Lane::WIDTH`] values to the front of `dst`.
    ///
    /// # Panics
    ///
    /// Panics if `dst` is shorter than [`Lane::WIDTH`].
    fn store(self, dst: &mut [f32]);

    /// `self + b`, IEEE round-to-nearest-even.
    fn add(self, b: Self) -> Self;

    /// `self - b`, IEEE round-to-nearest-even.
    fn sub(self, b: Self) -> Self;

    /// `self * b`, IEEE round-to-nearest-even.
    fn mul(self, b: Self) -> Self;

    /// `self / b`, IEEE-exact. Audit every render-path use: division is not cheap.
    fn div(self, b: Self) -> Self;

    /// `sqrt(self)`, IEEE-exact on every target.
    fn sqrt(self) -> Self;

    /// `(self * b) + c` with **two** roundings, on every backend (issue #163 phase 2).
    ///
    /// The name is historical: this operation is not fused and no longer may be. It is retained as
    /// a named operation because the kernels' frozen operation orders are written in terms of it,
    /// and because naming it keeps the multiply and the add adjacent so no backend can reassociate
    /// them.
    ///
    /// # Why the contract is unfused
    ///
    /// A hardware fused multiply-add exists on `x86` (pinned `+fma`) and on AArch64 NEON, and does
    /// not exist in base wasm `simd128`. Emulating it exactly on wasm cost about 54 instructions
    /// per operation -- measured 5.5x on the SVF kernel -- which the product pays on the platform
    /// it is leaning hardest into. The alternative of using hardware fusion where it exists and
    /// emulation where it does not is what the engine did until phase 2; the alternative of using
    /// hardware fusion where it exists and `(a * b) + c` where it does not is a per-backend
    /// numeric split, which this crate exists to prevent.
    ///
    /// Unfusing is a numeric-contract change and was made under an owner ruling (issue #163,
    /// 2026-08-26). It is not a free one: it moves every pinned bit in the workspace. The audit
    /// behind it -- per-site verdicts, domain error bounds, and the proof that the change cannot
    /// move a filter pole -- is `docs/rulings/unfused-multiply-add-audit.md`.
    fn fma(self, b: Self, c: Self) -> Self;

    /// `-self` as a sign-bit flip. Never `0.0 - self`, which is wrong for `+0.0`.
    fn neg(self) -> Self;

    /// `|self|` as a sign-bit clear.
    fn abs(self) -> Self;

    /// IEEE `floor(self)`.
    fn floor(self) -> Self;

    /// Ordered `self < b`: NaN in either operand yields an all-zero lane.
    fn lt(self, b: Self) -> Self::Mask;

    /// Ordered `self <= b`.
    fn le(self, b: Self) -> Self::Mask;

    /// Ordered `self > b`.
    fn gt(self, b: Self) -> Self::Mask;

    /// Ordered `self >= b`.
    fn ge(self, b: Self) -> Self::Mask;

    /// Ordered `self == b`. `+0.0` and `-0.0` compare equal.
    fn eq(self, b: Self) -> Self::Mask;

    /// Bitwise `a & b` on two masks.
    fn mask_and(a: Self::Mask, b: Self::Mask) -> Self::Mask;

    /// Bitwise `a | b` on two masks.
    fn mask_or(a: Self::Mask, b: Self::Mask) -> Self::Mask;

    /// Bitwise `!a` on a mask.
    fn mask_not(a: Self::Mask) -> Self::Mask;

    /// `true` if any lane of the mask is set.
    ///
    /// This is the only operation that leaves the vector domain. It is a control-plane and
    /// once-per-block operation; it must never appear in a per-sample loop.
    fn mask_any(m: Self::Mask) -> bool;

    /// Per-lane bitwise `m ? a : b`.
    fn select(m: Self::Mask, a: Self, b: Self) -> Self;

    /// Clears every lane of `self` whose mask lane is set, making it exactly `+0.0`.
    fn andnot(self, m: Self::Mask) -> Self;

    /// Per-lane maximum of the two lanes' raw bit patterns read as **unsigned** 32-bit integers.
    ///
    /// Each result lane is one input lane's bits, unchanged: no float arithmetic, no NaN
    /// quieting, no signed-zero rule. One instruction on every target: `vpmaxud` (AVX2),
    /// `i32x4.max_u` (wasm `simd128`) and `umax` (NEON). On two non-negative floats -- magnitudes,
    /// the one use (`flush_pair`, issue #1328 amendment A9) -- it is the larger value, and a NaN
    /// magnitude wins over every number, `+inf` included, because NaN magnitude bits sort above
    /// `0x7f80_0000`. Gate G1 holds every width to the scalar oracle over the directed edge pool.
    fn max_u32(self, b: Self) -> Self;

    /// `select(self > b, self, b)`: returns `b` on equal lanes and on unordered lanes (D8).
    ///
    /// Consequences that are deliberate and gated: `max(-0.0, +0.0)` is `+0.0`,
    /// `max(+0.0, -0.0)` is `-0.0`, `max(NaN, x)` is `x` and `max(x, NaN)` is `NaN`.
    ///
    /// This body is the specification. An implementation may override it only with a lowering
    /// that reproduces it bit for bit on every input, which `x86`'s `maxps` and wasm's
    /// operand-swapped `f32x4.pmax` do and NEON's `vmaxq`/`vmaxnmq` do not; gate G1 is what makes
    /// that a checked claim rather than a comment.
    #[inline(always)]
    fn max(self, b: Self) -> Self {
        Self::select(self.gt(b), self, b)
    }

    /// `select(self < b, self, b)`: the mirror of [`Lane::max`] (D8), overridable on the same
    /// terms.
    #[inline(always)]
    fn min(self, b: Self) -> Self {
        Self::select(self.lt(b), self, b)
    }

    /// `2^n` for integer-valued `n`, by exponent-field construction (no rounding).
    ///
    /// `n` is clamped to `[-126, 127]` with the D8 form first, so every target sees the same
    /// in-range input and NaN maps to `-126` (master plan §11). For an integer-valued `n` the
    /// result is exact; for a non-integer `n` the result is unspecified but identical on every
    /// backend, because the clamp, the add and the shift are the same operations everywhere.
    #[inline(always)]
    fn exp2_int(n: Self) -> Self {
        let n = Self::min(
            Self::max(n, Self::splat(bits::EXP2_INT_MIN)),
            Self::splat(bits::EXP2_INT_MAX),
        );
        Self::exp2_int_in_range(n)
    }

    /// `2^n` for an integer-valued `n` already in `[-126, 127]`.
    ///
    /// The caller owns the clamp. Implementations check the range with a debug assertion only;
    /// release render paths contain no range-check branch.
    fn exp2_int_in_range(n: Self) -> Self;

    /// Splits a positive normal `self` into `(m, e)` with `self = m * 2^e` and `m` in `[1, 2)`.
    ///
    /// Used by `log2`. For inputs that are not positive normals the result is unspecified but,
    /// again, identical on every backend.
    fn frexp(self) -> (Self, Self);

    /// Writes the raw bits of each lane to the front of `dst`. Tests and digests only.
    ///
    /// # Panics
    ///
    /// Panics if `dst` is shorter than [`Lane::WIDTH`].
    fn store_bits(self, dst: &mut [u32]);
}

/// The lane type this build runs its banks and frame loops at: `Simd8`, the 8-lane (AVX2) width,
/// where `avx2` is enabled, and [`Simd4`], the 4-lane (NEON/simd128) width, where `neon` or
/// `simd128` is (issue #1112).
///
/// Its two definitions carry the width predicates [`Backend::current`] selects by, and the
/// assertion after them holds the two to one width. Code that runs at the build's own width names
/// this alias rather than restating the predicates, and so does a test whose claim holds at any one
/// vector width: every build then runs it at the width it ships.
#[cfg(target_feature = "avx2")]
pub type Native = Simd8;
/// The 4-lane (NEON/simd128) `Native`; see the 8-lane definition.
#[cfg(any(target_feature = "neon", target_feature = "simd128"))]
pub type Native = Simd4;

// `Native` and `Backend::current()` are chosen by the same two predicates; they must agree. (Not
// under `cfg(doc)`: rustdoc gets no `avx2` on x86-64, so neither `Native` exists there.)
#[cfg(not(doc))]
const _: () = assert!(Backend::current().width() == <Native as Lane>::WIDTH);

/// Evaluates `$body` once for each lane type this build has, narrowest first: `f32`, [`Simd4`],
/// and `Simd8` where `avx2` is enabled (issue #1112).
///
/// `each_lane!(|L| body)` binds `L` to each lane type in turn; `each_lane!(|L, N| body)` also binds
/// `N`, its lane count, as a `usize` constant. The one source tests and gates take their widths
/// from: an 8-lane (AVX2) build runs `body` three times, a 4-lane (NEON/simd128) build twice, and no
/// caller restates which widths a build has. Each run is a block in the caller, so `?` and
/// `return` act on the caller. The `cfg` is a target feature, which one build sets for every crate
/// it compiles, so it means the same thing wherever the macro expands, as `match_bank_width!`'s
/// does. [`each_vector_lane!`] is the same without `f32`.
#[macro_export]
macro_rules! each_lane {
    (|$lane:ident $(, $lanes:ident)?| $body:expr) => {{
        $crate::each_lane!(@at f32, 1, |$lane $(, $lanes)?| $body);
        $crate::each_vector_lane!(|$lane $(, $lanes)?| $body);
    }};
    (@at $type:ty, $count:literal, |$lane:ident $(, $lanes:ident)?| $body:expr) => {{
        type $lane = $type;
        $(const $lanes: usize = $count;)?
        $body;
    }};
}

/// [`each_lane!`] over the vector lane types only: [`Simd4`], and `Simd8` where `avx2` is enabled.
#[macro_export]
macro_rules! each_vector_lane {
    (|$lane:ident $(, $lanes:ident)?| $body:expr) => {{
        $crate::each_lane!(@at $crate::Simd4, 4, |$lane $(, $lanes)?| $body);
        #[cfg(target_feature = "avx2")]
        $crate::each_lane!(@at $crate::Simd8, 8, |$lane $(, $lanes)?| $body);
    }};
}
