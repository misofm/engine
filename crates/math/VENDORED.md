# Vendored scalar math

## Source

* Crate: [`libm`](https://crates.io/crates/libm) (rust-lang/libm), the pure-Rust port of musl's
  libm.
* Version: **0.2.16** (`~/.cargo/registry/src/index.crates.io-*/libm-0.2.16/src/math/`).
* Licence: MIT. `LICENSE-libm.txt` in this directory is libm 0.2.16's `LICENSE.txt` verbatim,
  including the FreeBSD/Sun notices the individual files carry. Each vendored file keeps its own
  upstream header comment.

The engine does not depend on the `libm` crate: master plan D4 pins the reachable external
dependency set, and D6 requires that the transcendental implementation be *this* source, unchanged
by target features or optimisation level. Vendoring is what makes that inspectable.

## Files taken verbatim from `libm-0.2.16/src/math/<name>.rs`

`atan.rs` `atan2.rs` `cos.rs` `cosf.rs` `exp.rs` `exp2.rs` `exp2f.rs` `expf.rs` `expm1.rs`
`expm1f.rs` `k_cos.rs` `k_cosf.rs` `k_sin.rs` `k_sinf.rs` `k_tan.rs` `k_tanf.rs` `log.rs`
`log10.rs` `log10f.rs` `log2.rs` `log2f.rs` `logf.rs` `pow.rs` `powf.rs` `rem_pio2.rs`
`rem_pio2_large.rs` `rem_pio2f.rs` `sin.rs` `sinf.rs` `tan.rs` `tanf.rs` `tanh.rs` `tanhf.rs`

`exp`, `exp2`, `expf`, `exp2f`, `log`, `log2`, `logf`, `log2f`, `log10`, `log10f`, `pow`, `powf`,
`sin`, `cos`, `tan`, `sinf`, `cosf`, `tanf`, `tanh`, `tanhf`, `atan` and `atan2` are the functions
master plan §5.1 lists. The rest are what those implementations transitively need: `expm1`/`expm1f`
(`tanh`/`tanhf`), the `k_*` kernels and `rem_pio2*` (trig argument reduction).

## Files written here rather than vendored

* `mod.rs` — `fabs`/`fabsf` (`f64::abs` is available in `core`), `get_high_word`,
  `with_set_high_word`, `with_set_low_word`, and `scalbn`/`scalbnf` in the musl `src/math/scalbn.c`
  form. libm 0.2.16 has `scalbn` only as `generic/scalbn.rs`, generic over its private
  `support::Float` trait and carrying `f16`/`f128` configuration; the musl form is three exact
  prescaling multiplications and is what the vendored callers expect.
  `mod.rs` also redefines libm's `i!` and `div!` macros in safe form: upstream uses
  `get_unchecked`/`unchecked_div` in release builds, and the workspace denies `unsafe_code`. The
  values computed are identical.
* `floor.rs` — libm 0.2.16 has `floor` only in `generic/floor.rs`. This is that algorithm
  specialised to `f64`/`f32`, with the FP-status plumbing (inexact-flag bookkeeping only) removed.
* `sqrt.rs` — libm 0.2.16 has `sqrt` only in `generic/sqrt.rs`, which additionally short-circuits
  to a hardware instruction on most targets. Neither is usable here (`core` has no `f64::sqrt`, and
  the target-conditional path is exactly what D6 forbids), so `sqrt` is re-derived from `u128::isqrt`
  with a proof of correct rounding in the file's header comment. `sqrt` and `sqrtf` are needed by
  `pow`/`powf` for the `y == ±0.5` special case.

## Edits applied to every vendored file

1. A four-line provenance header naming the upstream path.
2. `#[cfg_attr(assert_no_panic, no_panic::no_panic)]` attributes deleted.
3. `select_implementation! { .. }` blocks deleted (they route to x87/intrinsic implementations in
   `exp`, `exp2`, `expf`, `exp2f`).
4. `force_eval!(..)` statements deleted. Every one of them exists only to raise an FP exception
   flag (`underflow`, `inexact`); none produces a value used later. The `unsafe { read_volatile }`
   macro itself is not vendored. Any local binding that existed only to feed a `force_eval!` was
   deleted with it.
5. `#[cfg(all(target_arch = "x86", not(target_feature = "sse2")))]` hunks deleted
   (`rem_pio2.rs`, `rem_pio2f.rs`, `pow.rs`); the plain path is what every supported target runs.
6. `#[cfg(test)] mod tests` blocks deleted; this crate's own tests replace them.
7. `pub fn` narrowed to `pub(crate) fn` — the public API is the documented wrapper layer in
   `lib.rs`, which is what `missing_docs` covers.
8. `rem_pio2_large.rs`: the 32/16-bit-pointer `IPIO2` table is deleted and the 64-bit table made
   unconditional, so wasm32 reduces huge trig arguments through the same table as x86_64; the
   `if cfg!(target_pointer_width = "64")` guard around the `e0` debug assertion is dropped with it;
   the inner `floor` shim loses its `x86_no_sse` attribute and `extern "C"`; and the
   `#[cfg(debug_assertions)] _ => unreachable!()` match arm is reduced to the release form `_ => {}`
   so the function cannot panic on a render path.

9. `cargo fmt` with the workspace `rustfmt.toml` (`max_width = 100`), which only closes the gaps
   the deletions above left behind.

After the edits, `src/vendored/` contains no occurrence of `target_feature`, `core::arch`,
`std::arch`, `mul_add`, `cfg(`, `unsafe`, `force_eval!` or `select_implementation!`.
`tests/m3_determinism.rs::m3_no_target_conditional_source` is the gate that keeps it that way.

## Re-vendoring

Bump the version above, re-copy the file list, re-apply the eleven edits, then run
`cargo test --release -p math`. `M3_DIGESTS` changing is a *deliberate* consequence of
a libm bump and must be re-pinned in the same commit with the upstream changelog cited; a digest
changing for any other reason is a determinism failure.
