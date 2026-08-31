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

**Superseded by #83 D4 (revision 4) via #84 phase A.** There is no runtime SIMD dispatch and no
capability struct: `engine::target_capabilities()`, `TargetCapabilities` and
`KernelBackendV1` were deleted together with `crates/engine/src/arch`.
`lane::Backend::current()` is a compile-time constant (`Simd8` on `x86-64-v3`, `Simd4`
on AArch64 and on a wasm artifact built with `simd128`, `Scalar` otherwise), and
`lane::attest_host()` refuses at boot on an x86 CPU that lacks the pinned AVX2/FMA
rather than degrading silently. `effect_contract::BankWidth::for_backend` is the
workspace's single backend-to-width law.

No Cargo feature is named `simd128`, `neon`, `avx2`, or `fma`. CPU ISA flags must never be made
global in `.cargo/config.toml`, package manifests, or release defaults beyond the workspace's
`x86-64-v3` pin. CI's deliberately scoped probe flags are evidence that separate artifacts
compile, not deployment defaults.

## Reproducible checks

The pinned `rust-toolchain.toml` installs Rust 1.97.1 with `clippy`, `rustfmt`, and browser Wasm,
Android ARM64, and iOS ARM64 standard libraries. After the workspace exists, the relevant commands
are:

```bash
cargo check --locked --workspace --all-targets
cargo check --locked --target aarch64-linux-android \
  -p engine -p session -p protocol \
  -p capi -p target-smoke -p host-mobile
cargo check --locked --target aarch64-apple-ios \
  -p engine -p session -p protocol \
  -p capi -p target-smoke -p host-mobile

CARGO_TARGET_DIR=target/ci/wasm-scalar RUSTFLAGS="-C target-feature=-simd128" \
  cargo build --locked --release --target wasm32-unknown-unknown \
  -p engine -p session -p protocol \
  -p target-smoke -p host-web
CARGO_TARGET_DIR=target/ci/wasm-simd RUSTFLAGS="-C target-feature=+simd128" \
  cargo build --locked --release --target wasm32-unknown-unknown \
  -p engine -p session -p protocol \
  -p target-smoke -p host-web
```

`cargo check` verifies Rust compilation only. iOS linking/device execution needs Xcode and an iOS
SDK; Android linking/device execution needs a suitable NDK; browser execution needs a browser
test harness. Those are explicitly deferred to the platform adapter issues.

## Render threading (issue 100, removed)

There is no parallel render path and therefore no target gate for one. The native dependency-wave
scheduler was built and qualified under issue 100, then removed as production-unreachable: every
graph-side use was `cfg(not(target_arch = "wasm32"))`, host-core always bound sequentially, and no
wasm artifact ever contained a scheduler node. Render is single-threaded on every target, native
and browser alike.

The evidence rows this section used to carry -- coordinator/worker syscall counts under strace,
worker idle CPU from `/proc/<tid>/stat`, and the determinism/pool-lifetime/wake-protocol suites --
went with the machinery they measured. `x86_64-unknown-linux-gnu` no longer carries any
render-threading evidence that other targets lack.

Platform thread priority, affinity and workgroup adoption remain with the host issues, unchanged:
they were never part of the scheduler and are not affected by its removal.
