# Target matrix

Issue 001 establishes build policy, not a platform audio callback, device link, or browser runtime.
The session model and its semantics do not vary by Cargo feature or target capability.

| Artifact | Architecture and target | SIMD policy | CI evidence in issue 001 |
| --- | --- | --- | --- |
| Native baseline | `x86_64-unknown-linux-gnu` | Scalar baseline. It must not receive a global AVX2 or FMA flag. | Host check/test plus a `-avx2,-fma` compile probe. |
| Native AVX2 | `x86_64-unknown-linux-gnu` | A future internal AVX2 kernel is entered only after runtime AVX2 detection. | Separate `+avx2,-fma` compile directory and injected capability-assembly test. |
| Native AVX2/FMA | `x86_64-unknown-linux-gnu` | A future FMA kernel requires both independently detected AVX2 and FMA. | Separate `+avx2,+fma` compile directory and cfg assertion. |
| ARM64 Android | `aarch64-linux-android` | AArch64 NEON is selected statically as four-lane processing. | Linux pure-Rust `cargo check`; no NDK link or device claim. |
| ARM64 iOS | `aarch64-apple-ios` | AArch64 NEON is selected statically as four-lane processing. | macOS pure-Rust `cargo check`; no SDK link or device claim. |
| Browser Wasm | `wasm32-unknown-unknown` | Baseline and `+simd128` are distinct artifacts. Four-lane processing uses multiply plus add; relaxed SIMD and FMA assumptions are forbidden. | Two distinct release artifact directories. |

## Dispatch contract

`miso_engine_core::target_capabilities()` runs on the control plane while a prepared plan is
selected. It is not a realtime render operation. Scalar is always true. Wasm `simd128` reports
the module compilation setting. AArch64 NEON reports the target setting. x86 AVX2 and FMA are
separately runtime detected; AVX2 with FMA unavailable remains a supported selection.

No Cargo feature is named `simd128`, `neon`, `avx2`, or `fma`. CPU ISA flags must never be made
global in `.cargo/config.toml`, package manifests, or release defaults. CI's deliberately scoped
probe flags are evidence that separate artifacts compile, not deployment defaults. Future
per-function `target_feature` kernels live behind runtime dispatch in core and preserve scalar
fallbacks for every track tail.

## Reproducible checks

The pinned `rust-toolchain.toml` installs Rust 1.97.1 with `clippy`, `rustfmt`, and browser Wasm,
Android ARM64, and iOS ARM64 standard libraries. After the workspace exists, the relevant commands
are:

```bash
cargo check --locked --workspace --all-targets
cargo check --locked --target aarch64-linux-android \
  -p miso-engine-core -p miso-engine-session -p miso-engine-protocol \
  -p miso-engine-capi -p miso-engine-target-smoke -p miso-engine-host-mobile
cargo check --locked --target aarch64-apple-ios \
  -p miso-engine-core -p miso-engine-session -p miso-engine-protocol \
  -p miso-engine-capi -p miso-engine-target-smoke -p miso-engine-host-mobile

CARGO_TARGET_DIR=target/ci/wasm-scalar RUSTFLAGS="-C target-feature=-simd128" \
  cargo build --locked --release --target wasm32-unknown-unknown \
  -p miso-engine-core -p miso-engine-session -p miso-engine-protocol \
  -p miso-engine-target-smoke -p miso-engine-host-web
CARGO_TARGET_DIR=target/ci/wasm-simd RUSTFLAGS="-C target-feature=+simd128" \
  cargo build --locked --release --target wasm32-unknown-unknown \
  -p miso-engine-core -p miso-engine-session -p miso-engine-protocol \
  -p miso-engine-target-smoke -p miso-engine-host-web
```

`cargo check` verifies Rust compilation only. iOS linking/device execution needs Xcode and an iOS
SDK; Android linking/device execution needs a suitable NDK; browser execution needs a browser
test harness. Those are explicitly deferred to the platform adapter issues.
