# Target matrix

Issue 001 establishes build policy, not a platform audio callback, device link, or browser runtime.
The session model and its semantics do not vary by Cargo feature or target capability.

| Artifact | Architecture and target | SIMD policy | CI evidence in issue 001 |
| --- | --- | --- | --- |
| Native baseline | `x86_64-unknown-linux-gnu` | Scalar baseline. It must not receive a global AVX2 or FMA flag. | Host check/test plus a `-avx2,-fma` compile probe. |
| Native AVX2 | `x86_64-unknown-linux-gnu` | A future internal AVX2 kernel is entered only after runtime AVX2 detection. | Separate `+avx2,-fma` compile directory and injected capability-assembly test. |
| Native AVX2/FMA | `x86_64-unknown-linux-gnu` | A future FMA kernel requires both independently detected AVX2 and FMA. | Separate `+avx2,+fma` compile directory and cfg assertion. |
| ARM64 Android | `aarch64-linux-android` (arm64-v8a) | Product target (#1017). NEON `Simd4`, a compile-time constant; FPCR pinned per render block. | `cross-target`: the product crates checked with `--all-targets --all-features` and linted with clippy `-D warnings`. `aarch64-debug`/`aarch64-release`: tests on `ubuntu-24.04-arm`. |
| ARM64 iOS | `aarch64-apple-ios` | Product target (#1017). As Android. | `cross-target`: the same check and clippy rows, and the release-assembly scan `ios-asm-memset-pattern16` (an expected failure until #1018). Tests run on the Linux arm64 legs; see "Native AArch64" below. |
| Refused | `armv7-linux-androideabi` (armeabi-v7a), `i686-*`, ILP32 ABIs and every other unlisted target | Refused at compile time by `lane` (#1041). | `scripts/check-cross-targets.sh` refusal row on `armv7-linux-androideabi`. |
| Browser Wasm | `wasm32-unknown-unknown` | Baseline and `+simd128` are distinct artifacts. Four-lane processing uses multiply plus add; relaxed SIMD and FMA assumptions are forbidden. | Two distinct release artifact directories. |

## 64-bit only (issue #1041)

Owner ruling 2026-09-28 (`docs/rulings/engine-footprint-2026-09-28.md`): "Yes let's go 64bit
only." iOS arm64 and Android arm64-v8a ship; 32-bit ARM (armeabi-v7a) does not. `lane` refuses
to compile for any target outside this set (`crates/lane/src/lib.rs`, beside the x86-64-v3 guard):

- `x86_64` with 64-bit pointers and the pinned AVX2 and FMA (the x86-64-v3 guard);
- `aarch64` with 64-bit pointers;
- `wasm32` with `simd128`.

`capi`, `host-core`, `host-web`, `graph` and every effect depend on `lane`, so the refusal covers
everything a mobile app or a browser embeds. The error reads
"lane supports 64-bit targets only". The pointer-width condition also refuses the ILP32 ABIs that
share an architecture name (`x86_64-unknown-linux-gnux32`, `arm64_32-apple-watchos`).
`scripts/check-cross-targets.sh` proves the refusal on `armv7-linux-androideabi` in CI's
`cross-target` job, and fails if that target ever compiles.

**The scalar-wasm CI exception.** `wasm32` *without* `simd128` still compiles. It is not a product
target: the one shipped artifact is `simd128` (W4-D1). It stays allowed only because CI builds
`lane` that way today, and removing those legs is a separate, pending decision. The legs are:

- `qualification.yml` `wasm-guests`: the scalar Wasm build, the scalar evidence-crate check,
  `scripts/check-wasm-realtime-atomics.sh`, the scalar variant of
  `scripts/check-protocol-wasm-parity.sh`, and the scalar guest of `scripts/run-wasm-gates.sh`
  (gate G5's scalar-width leg);
- `qualification.yml` `cross-target`: the scalar rows of `scripts/check-cross-targets.sh`, plus
  `scripts/check-effect-package-v1.sh` and `scripts/check-effect-descriptor-v1.sh`.

When the legs go, delete the one marked arm of the guard. The scalar arm of
`lane::Backend::current()` will then be unreachable.

## Dispatch contract

**Superseded by #83 D4 (revision 4) via #84 phase A.** There is no runtime SIMD dispatch and no
capability struct: `engine::target_capabilities()`, `TargetCapabilities` and
`KernelBackendV1` were deleted together with `crates/engine/src/arch`.
`lane::Backend::current()` is a compile-time constant (`Simd8` on `x86-64-v3`, `Simd4`
on AArch64 and on a wasm artifact built with `simd128`, and `Scalar` only under the scalar-wasm
CI exception above, since every other target fails to compile), and
`lane::attest_host()` refuses at boot on an x86 CPU that lacks the pinned AVX2/FMA
rather than degrading silently. `effect_contract::BankWidth::for_backend` is the
workspace's single backend-to-width law.

No Cargo feature is named `simd128`, `neon`, `avx2`, or `fma`. CPU ISA flags must never be made
global in `.cargo/config.toml`, package manifests, or release defaults beyond the workspace's
`x86-64-v3` pin. CI's deliberately scoped probe flags are evidence that separate artifacts
compile, not deployment defaults.

## Reproducible checks

The pinned `rust-toolchain.toml` installs Rust 1.97.1 with `clippy`, `rustfmt`, and the browser
Wasm standard library. After the workspace exists, the relevant commands are:

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

### Native AArch64 (#1017)

Owner ruling 2026-09-28 (`docs/rulings/engine-footprint-2026-09-28.md`): "Yes ARM64 will be an
official target." Fans open and play sessions in iOS and Android apps, which embed the engine
through the C ABI. This reverses #378. What qualifies the target:

* **Compile and lint rows** (`scripts/check-cross-targets.sh`, `qualification.yml` `cross-target`).
  For `aarch64-apple-ios` and `aarch64-linux-android`, the product crates are checked with
  `--all-targets --all-features` and linted with clippy `-D warnings`. The product crates are every
  workspace crate in `capi`'s normal-dependency closure (`scripts/lib/product-crates.sh`), derived
  rather than listed. `cargo check` links nothing, so neither Xcode nor the NDK is needed; linking
  and device or simulator execution remain unexercised.
* **Tests on arm64 hardware** (`scripts/run-aarch64-tests.sh`, `qualification.yml` `aarch64-debug`
  and `aarch64-release`, both on `ubuntu-24.04-arm`, both in the verdict's full-route table).
  `aarch64-debug` runs the product crates' tests, plus `dsp-reference`, `conformance` and
  `target-smoke`, in the debug profile: the four-lane banks bind natively and `lane::fpenv`'s FPCR
  path runs. `aarch64-release` runs `lane`, `math` and `wasm-gates` (G1-G6, M1-M3, the G5 class-A
  corpus, G6 under `FPCR.FZ`) in the shipping release profile, `console-workload`'s console digests,
  and the realtime audits (`audit capi` and the delay, compressor, EQ and gate audits).
* **No silent skips.** Both legs refuse a test that returns early on a SIMD backend width. A test
  that only one width can run says so with `#[ignore = "…"]` on the other width.
* **Known defects fail by name.** Each is an expected failure on its row, skipped by exact name in
  the main run and then run alone, where it must fail. If it passes, the leg fails and asks for the
  row to be deleted. See the register below.

**Why Linux arm64 and not also macOS arm64.** The Linux runner exercises the AArch64 code that
differs from x86 (NEON, FPCR, the fold behind LANE-3), and a public repository pays nothing for
it. The Darwin-only defect is a codegen fact of the Apple targets, so it is checked where it lives,
in the iOS release assembly, on the x86 `cross-target` job. A macOS arm64
runner would run `aarch64-apple-darwin` (an Apple M1 baseline) rather than the iOS target, and
none of the realtime audits sees a libc call that is not an allocation, lock or syscall. So it
would add runner minutes without covering the iOS defect.

Not covered: x86-64 Android emulators and iOS simulators, which inherit the workspace's
AVX2/FMA pin (`.cargo/config.toml`), and NEON performance.

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

## Known AArch64 defects (#1017)

#378 kept this register while native AArch64 was unsupported. #1017 reopened each entry on the
AArch64 legs. Each open entry is an expected failure, by name:

- **LANE-3 (#1019).** On AArch64 release builds the D8 `max`/`min` fold into `fmaxnm`/`fminnm`,
  which answer differently on NaN and signed-zero inputs. Gate G1's op-level pool passes on AArch64
  (1.97.1). The fold shows in `math`'s M2 inside `exp2_lane` and `log2_lane`, scalar and vector
  alike. Expected failures in `aarch64-release`: `math` `m2_lane_identity`
  `m2_exp2_lane_identity` and `m2_log2_lane_identity`. Both pass in the debug leg.
- **AArch64 NaN encodings (attributed to #1019; root to confirm).** Found by #1017. An arithmetic
  NaN on AArch64 is `0x7FC00000`, where x86 answers `0xFFC00000`, and a signalling operand wins NaN
  propagation. So a pin that folds raw NaN words from hostile input moves on AArch64 in every
  profile. With every NaN folded as one word, these pins are identical on both architectures.
  #1019's gate 1 already requires NaN-payload identity for the compressor on this leg. Expected
  failures in `aarch64-debug`: `compressor`
  `kernel::settled_body_tests::scenario_{981,983,985,995}_*_is_pinned` and
  `scenario_1006_ramping_prefix_is_pinned`, and `parametric-eq` `bank`
  `admitted_blocks_render_the_base_bits_without_selects`. Every other test in the two legs passes
  on AArch64, the console digests and the G5 corpus included.
- **Darwin `memset_pattern16` in render (#1018).** On Apple targets a stored splat, the SVF
  flush's `L::splat(FLUSH_EPS)` among others, lowers to `bl _memset_pattern16` inside the EQ and
  builtin render kernels: 151 calls in `parametric-eq`'s iOS release assembly and 376 in
  `builtins`'. This is a realtime-policy violation on Darwin, with no effect on `x86_64`, Linux
  AArch64 or `wasm32`. It is expected failure `ios-asm-memset-pattern16` in
  `scripts/check-cross-targets.sh`.
- **Resolved by #1017: tests that assumed eight lanes.** The seven test-only warnings
  (`host-core/tests/fp_environment.rs`, `lane/tests/fp_env.rs`) and the tests that asserted or
  returned on `Backend::current() == Simd8` (listed in
  `docs/audits/test-usefulness-2026-09-04/03-compilers-hosts-tools.md:227,240,297`, plus what the
  legs found in `console-workload`, `host-core`, `graph-compiler`, `true-peak-limiter`,
  `parametric-eq`, `effect-compiler`, `gate-expander` and `compressor`) are width-agnostic, or
  ignore the other width by name. Four stay eight-lane-only with their reason: the gate-expander W8
  bank tests, the exact eight-lane byte totals in `capi`'s `resource_lifecycle` (#1060 replaces
  them), the per-node console-effect fixture in `host-core`'s `symmetry_witness`, and compressor's
  MQ-2 preflight (#1027 ports it). The W4 gate binding test runs only on AArch64.
