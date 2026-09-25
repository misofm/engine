// IEEE 754 correctly rounded square root operations.
//
// The standard library exposes the hardware operation as an inherent method because stable
// `core` does not yet expose `f32::sqrt` or `f64::sqrt`. IEEE 754 specifies the rounded numeric
// result for nonnegative inputs exactly. Negative inputs return NaN; its sign and payload are
// target-dependent (measured x86-64: negative quiet NaN; ARM: positive default NaN; wasm: NaN
// bits are nondeterministic). No NaN guard is added, since LLVM removes it at optimisation and
// NaN bit identity is outside the engine's determinism contract.

/// Correctly rounded square root (f64).
pub(crate) fn sqrt(x: f64) -> f64 {
    f64::sqrt(x)
}

/// Correctly rounded square root (f32).
pub(crate) fn sqrtf(x: f32) -> f32 {
    f32::sqrt(x)
}
