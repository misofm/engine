# Target matrix

Issue 001 establishes build policy, not a platform audio callback, device link, or browser runtime.
The session model and its semantics do not vary by Cargo feature or target capability.

| Artifact | Architecture and target | SIMD policy | CI evidence in issue 001 |
| --- | --- | --- | --- |
| Native baseline | `x86_64-unknown-linux-gnu` | Scalar baseline. It must not receive a global AVX2 or FMA flag. | Host check/test plus a `-avx2,-fma` compile probe. |
| Native AVX2 | `x86_64-unknown-linux-gnu` | A future internal AVX2 kernel is entered only after runtime AVX2 detection. | Separate `+avx2,-fma` compile directory and injected capability-assembly test. |
| Native AVX2/FMA | `x86_64-unknown-linux-gnu` | A future FMA kernel requires both independently detected AVX2 and FMA. | Separate `+avx2,+fma` compile directory and cfg assertion. |
| ARM64 Android | `aarch64-linux-android` | Unsupported; no claim. | Deferred, see #366 and #378. |
| ARM64 iOS | `aarch64-apple-ios` | Unsupported; no claim. | Deferred, see #366 and #378. |
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

CARGO_TARGET_DIR=target/ci/wasm-scalar RUSTFLAGS="-C target-feature=-simd128" \
  cargo build --locked --release --target wasm32-unknown-unknown \
  -p engine -p session -p protocol \
  -p target-smoke -p host-web
CARGO_TARGET_DIR=target/ci/wasm-simd RUSTFLAGS="-C target-feature=+simd128" \
  cargo build --locked --release --target wasm32-unknown-unknown \
  -p engine -p session -p protocol \
  -p target-smoke -p host-web
```

`cargo check` verifies Rust compilation only. Browser execution needs a browser test harness,
explicitly deferred to the platform adapter issues.

### When native AArch64 is revived

Native AArch64 (`aarch64-linux-android`, `aarch64-apple-ios`) is unsupported per the owner ruling
recorded in the deferred-defect register below (#378). A future revival must reopen the register
entries first, then restore this compile-checked evidence:

```bash
cargo check --locked --target aarch64-linux-android \
  -p engine -p session -p protocol \
  -p capi -p target-smoke -p host-mobile
cargo check --locked --target aarch64-apple-ios \
  -p engine -p session -p protocol \
  -p capi -p target-smoke -p host-mobile
```

iOS linking/device execution needs Xcode and an iOS SDK; Android linking/device execution needs a
suitable NDK. Neither was ever exercised even when these rows were compile-checked.

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

## Deferred-defect register (native AArch64, #378)

Owner ruling, 2026-09-04: native AArch64 (`aarch64-linux-android`, `aarch64-apple-ios`, and by
extension `aarch64-apple-darwin`) is downgraded from "compile-checked" to "unsupported, no claim"
until a native iOS/Android effort is scheduled (#378). This is a support-level downgrade, not a
code removal: the workspace's `cfg(target_arch = "aarch64")` lines stay as they are. Browsers on
Apple Silicon and iPhones run the `wasm32` build, so this does not affect any launch user. A future
revival of native AArch64 must reopen each entry below before claiming the target again.

- **LANE-3 (#366)**: on AArch64 release builds the D8 `max`/`min` fold into `fmaxnm`/`fminnm`,
  moving bits away from the pinned oracle; gate G1 is red there today. High severity when native
  AArch64 is a target.
- **SVF `flush()` emits `bl _memset_pattern16` on Darwin**: `svf_step`'s `L::splat(FLUSH_EPS)`
  (`crates/lane/src/kernels.rs:280-281`) compiles on Apple targets to two libc calls per frame
  inside the kernel loop (found while verifying #373; present on `main`). Realtime-policy violation
  on Darwin; no effect on `x86_64` or `wasm32`.
- The September 2026 test-usefulness audit
  (`docs/audits/test-usefulness-2026-09-04/03-compilers-hosts-tools.md:227,240,297`) lists tests
  that assert `Backend::current() == Simd8` and therefore fail on aarch64.
