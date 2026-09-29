# Target matrix

Which targets the engine builds for, the SIMD policy of each, and the CI rows that qualify it.
The session model and its semantics do not vary by Cargo feature or target capability.

| Artifact | Architecture and target | SIMD policy | CI evidence |
| --- | --- | --- | --- |
| Native x86-64 (tooling and tests) | `x86_64-unknown-linux-gnu` | Not a shipped product target. Pinned x86-64-v3 (#83 D4): `.cargo/config.toml` gives every `x86_64` build AVX2 and FMA, so `lane::Backend::current()` is `Simd8`, a compile-time constant. No runtime detection or dispatch. `lane` refuses to compile without both features, and `lane::attest_host()` refuses at boot on a CPU that lacks them. | `lint`: the `-avx2,-fma` and `+avx2,-fma` probes must fail with `requires x86-64-v3`; the `+avx2,+fma` probe compiles, with a cfg assertion. `lint`, `test-debug-a`, `test-debug-b`, `test-release` and `audit-native` run on this target. |
| ARM64 Android | `aarch64-linux-android` (arm64-v8a) | Product target (#1017). NEON `Simd4`, a compile-time constant; FPCR pinned per render block. | `cross-target`: the product crates checked with `--all-targets --all-features` and linted with clippy `-D warnings`. `aarch64-debug`/`aarch64-release`: tests on `ubuntu-24.04-arm`. |
| ARM64 iOS | `aarch64-apple-ios` | Product target (#1017). As Android. | `cross-target`: the same check and clippy rows, and the release-assembly scan `ios-asm-memset-pattern16` over every product crate (expected failures per crate until #1018). Tests run on the Linux arm64 legs; see "Native AArch64" below. |
| Refused | `armv7-linux-androideabi` (armeabi-v7a), `i686-*`, ILP32 ABIs and every other unlisted target | Refused at compile time by `lane` (#1041). | `scripts/check-cross-targets.sh` refusal row on `armv7-linux-androideabi`. |
| Browser Wasm | `wasm32-unknown-unknown` with `+simd128` | One shipped AudioWorklet artifact, `miso-engine-v1-audio-worklet.simd128.wasm` (W4-D1), built by `scripts/build-web-audioworklet.sh`. `Simd4`, a compile-time constant. Four-lane processing uses multiply plus add; relaxed SIMD and FMA assumptions are forbidden. The host refuses a browser without `simd128` with a typed `miso.unsupported.v1` error. A scalar build is not an artifact (see the scalar-wasm CI exception below). | `artifact` builds the module once; `artifact-gates` (`check-web-audioworklet.sh`) and `browser` (Chromium, Firefox, WebKit) qualify that module. |

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
- `qualification.yml` `cross-target`: the scalar rows of `scripts/check-cross-targets.sh`. (Its
  `check-effect-package-v1.sh` and `check-effect-descriptor-v1.sh` legs went with the
  `effect-package` crate in #1037.)

When the legs go, delete the one marked arm of the guard. The scalar arm of
`lane::Backend::current()` will then be unreachable.

The exception is also why `lane::Backend::Scalar` exists without `lane/test-support` on those
builds (#1059). A scalar-wasm build names the variant but compiles none of the whole-plan scalar
lowering, which is test-only: nothing on those legs compiles a plan, and one that tried would stop
at a named panic in `builtins-compiler`.

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

**The whole-plan scalar backend is a test-only oracle** (owner ruling 2026-09-28, decision 3;
#1059). `lane::Backend::Scalar` exists only with `lane/test-support` (and on the scalar-wasm
exception), and the per-node strip lowering, its scalar pair owners and the graph's scalar
pairing passes compile only for tests and the `test-support` features. Tests compile a plan at
`Scalar` to prove that banking never moves a rendered bit. `scripts/check-scalar-oracle-absent.py`
fails if the shipped AudioWorklet module or a release `capi` library contains any of it. The
one-lane `f32` `Lane`, each effect's per-node leg and every frame loop's tail, is not gated.

No Cargo feature is named `simd128`, `neon`, `avx2`, or `fma`. CPU ISA flags must never be made
global in `.cargo/config.toml`, package manifests, or release defaults beyond the workspace's
`x86-64-v3` pin. CI's deliberately scoped probe flags prove that sub-v3 builds are refused and
that the pinned set compiles; they are not deployment defaults.

## Reproducible checks

The pinned `rust-toolchain.toml` installs Rust 1.97.1 with `clippy`, `rustfmt`, and the browser
Wasm standard library. The relevant commands are:

```bash
cargo check --locked --workspace --all-targets

# The shipped browser artifact (W4-D1), into an existing empty directory.
bash scripts/build-web-audioworklet.sh <output-dir>

# Compile checks for the browser crates: the scalar-wasm CI exception, then simd128.
CARGO_TARGET_DIR=target/ci/wasm-scalar RUSTFLAGS="-C target-feature=-simd128" \
  cargo build --locked --release --target wasm32-unknown-unknown \
  -p engine -p session -p protocol \
  -p target-smoke -p host-web
CARGO_TARGET_DIR=target/ci/wasm-simd RUSTFLAGS="-C target-feature=+simd128" \
  cargo build --locked --release --target wasm32-unknown-unknown \
  -p engine -p session -p protocol \
  -p target-smoke -p host-web
```

`cargo check` verifies Rust compilation only. The browser runtime is qualified on the shipped
module by the `artifact-gates` and `browser` jobs (`hosts/host-web/qualification`), and native
AArch64 as below.

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
  path runs. `aarch64-release` runs, in the shipping release profile, the G5 class-A corpus
  (`wasm-gates`, #1048's native owner, as its own unfiltered step) and `lane` and `math` (G1-G4, G6
  under `FPCR.FZ`, P1, M1-M3). It also runs `console-workload`'s console digests and the realtime
  audits (`audit capi` and the delay, compressor, EQ and gate audits).
* **No silent skips.** Both legs refuse a test that returns early on a SIMD backend width. A test
  that only one width can run says so with `#[ignore = "…"]` on the other width.
* **Known defects fail by name and by reason.** The rows live in
  `scripts/lib/aarch64-known-defects.py`. Each test row names exactly one test in its leg, which
  is skipped by exact name in the main run and then run alone. There it must fail as that one test,
  with the row's reason in its panic: the AArch64 digest or assertion text the defect produces. A
  pass, another panic or another reason fails the leg. The iOS memset rows are per-crate ceilings:
  a crate at zero, a crate above its ceiling, and a crate with calls but no row each fail. See the
  register below.

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
- **AArch64 NaN encodings (#1065).** Found by #1017. Not LANE-3: no code shape changes a CPU's NaN
  rule, and the wasm spec leaves the same bits unspecified in the browser. An arithmetic
  NaN on AArch64 is `0x7FC00000`, where x86 answers `0xFFC00000`, and a signalling operand wins NaN
  propagation. So a pin that folds raw NaN words from hostile input moves on AArch64 in every
  profile. With every NaN folded as one word, these pins are identical on both architectures.
  #1065 asks the owner to rule whether class-A identity treats every NaN as one value. Expected
  failures in `aarch64-debug`: `compressor`
  `kernel::settled_body_tests::scenario_1006_ramping_prefix_is_pinned` and `parametric-eq` `bank`
  `admitted_blocks_render_the_base_bits_without_selects`. (#1049 deleted the compressor's
  `scenario_{981,983,985,995}` pins, and their rows, as dominated.) Every other test in the two
  legs passes on AArch64, the console digests and the G5 corpus included.
- **Darwin `memset_pattern16` in render (#1018).** On Apple targets LLVM lowers a stored `f32x4`
  splat constant to `bl _memset_pattern16`, a libc call. The constants are `lane::FLUSH_EPS` (the
  SVF flush), `1.0`, `0.5`, `2.0`, `1e-8`, `f32::MIN_POSITIVE` and others. So this is not the SVF
  flush alone. There are 3,494 calls across ten product crates, counted in each crate's
  `aarch64-apple-ios` release assembly on Rust 1.97.1:

  | crate | calls | where |
  |---|---|---|
  | `multiband-compressor` | 1,132 | `PreparedMultibandCompressorBank::process_bank` |
  | `compressor` | 970 | `kernel::process_block`, `ramping_main_scalar`, `process_block_mono`, `settled_sidechain` |
  | `transient-shaper` | 534 | `Shaper::process_block` |
  | `builtins` | 376 | `BuiltinInputBank::process`/`process_mono`, `InputStage::process` |
  | `gate-expander` | 181 | `PreparedGate::process_bank` |
  | `parametric-eq` | 151 | `PreparedParametricEq::process_bank`/`process_bank_mono`, `Channel::snap_ended` |
  | `true-peak-limiter` | 104 | `LimiterCore::process_block`, `process_bank_inner` |
  | `soft-clip` | 22 | `Channel::process`, `PreparedSoftClipBank::process_bank` |
  | `graph` | 20 | `runtime::bank_meter_pass`, `runtime::bank_sample_peak` |
  | `host-core` | 4 | `spectrum::SpectrumAnalyzer::analyze`/`analyze_continuous` |

  Nearly every call sits in a render function, which is a realtime-policy violation on every
  iPhone. There is no effect on `x86_64`, Linux AArch64 or `wasm32`. The expected failures are
  `ios-asm-memset-pattern16`, one row per crate with its count as a ceiling, in
  `scripts/lib/aarch64-known-defects.py`, scanned by `scripts/check-cross-targets.sh`. `capi` is
  scanned as an rlib and has none.
- **Resolved by #1017: tests that assumed eight lanes.** The seven test-only warnings
  (`host-core/tests/fp_environment.rs`, `lane/tests/fp_env.rs`) and the tests that asserted or
  returned on `Backend::current() == Simd8` (listed in
  `docs/audits/test-usefulness-2026-09-04/03-compilers-hosts-tools.md:227,240,297`, plus what the
  legs found in `console-workload`, `host-core`, `graph-compiler`, `true-peak-limiter`,
  `parametric-eq`, `effect-compiler`, `gate-expander` and `compressor`) are width-agnostic, or
  ignore the other width by name. Two stay eight-lane-only with their reason: the gate-expander W8
  bank tests and the per-node console-effect fixture in `host-core`'s `symmetry_witness`. #1060
  replaced the exact eight-lane byte totals in `capi`'s `resource_lifecycle` with per-width budgets,
  live-report requirements and an allocator oracle, which run at both widths. The W4 gate
  binding test runs only on AArch64. The gate-expander tests' W8-only claims (payload interchange,
  restore isolation, malformed-word rejection) are not yet checked at W4; making them width-generic
  is a successor issue.
