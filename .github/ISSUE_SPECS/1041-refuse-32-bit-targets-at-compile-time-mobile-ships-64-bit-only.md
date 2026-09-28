# Refuse 32-bit targets at compile time (mobile ships 64-bit only)

Owner ruling (2026-09-28): "Yes let's go 64bit only." iOS arm64 and Android arm64-v8a ship; 32-bit ARM (`armeabi-v7a`, `target_arch = "arm"`) does not (`docs/rulings/engine-footprint-2026-09-28.md`).

## Problem

`lane::Backend::current()` (`crates/lane/src/backend.rs:60-68`) gives `Backend::Scalar` for every architecture other than x86, x86_64, aarch64 and `wasm32`+`simd128`, silently. Only x86 has a `compile_error!` guard (`crates/lane/src/lib.rs:72-80`). A 32-bit ARM Android build would therefore compile and run the whole-plan scalar path, several times slower, with nothing warning anyone. Evidence: `docs/handoffs/dead-code-2026-09-28/VERIFY-DEAD-CODE.md`, finding F6 and the R8 amendment.

## Smallest closable slice

Add a `compile_error!` in `lane`, beside the x86 guard, that refuses every target the engine does not support: anything other than x86-64 (with the pinned AVX2+FMA features), `aarch64`, and `wasm32` with `simd128`. The message names the supported targets and cites this ruling. Update `docs/TARGET_MATRIX.md` to state 64-bit only.

## Objective gates

1. `cargo check -p lane --target armv7-linux-androideabi` (and `thumbv7neon-linux-androideabi` if installed) fails with the new message; `x86_64`, `aarch64-apple-ios`, `aarch64-linux-android` and `wasm32-unknown-unknown` (+simd128) still build.
2. No change to any rendered bit or to the shipped AudioWorklet artifact bytes.
3. The check that proves gate 1 runs in the AArch64 CI leg (#1017) or in the cross-target script.
