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

## Attempt 1 evidence

Terra, 2026-09-28, commit `4ef7c986` on `codex/1041-refuse-32-bit-targets` (base `4a0d60bd`). All
runs used the pinned `rustc 1.97.1 (8bab26f4f 2026-07-14)`. Every target's standard library was
available for 1.97.1 through `rustup target add`, so no other toolchain was used.

### What changed

- **The guard** (`crates/lane/src/lib.rs`, directly after the x86-64-v3 guard). It raises
  `compile_error!` unless one of these holds:
  - `x86_64` with 64-bit pointers;
  - `aarch64` with 64-bit pointers;
  - `wasm32` with `simd128`;
  - the marked **scalar-wasm CI exception**, `wasm32` without `simd128`.

  The message begins "lane supports 64-bit targets only", then names the three targets and cites
  the ruling and this issue. The x86-64-v3 guard is unchanged: CI's two sub-v3 probes still fail
  with 'requires x86-64-v3' and not with the new message. The guard needs no `not(doc)` escape
  because its result depends only on `target_arch` and `target_pointer_width`, which rustdoc sees.
  The pointer-width condition goes slightly beyond the brief. It refuses the ILP32 ABIs that share
  an architecture name (`x86_64-unknown-linux-gnux32`, `arm64_32-apple-watchos`), which are 32-bit
  builds under the ruling.
- **The exception.** It exists because CI builds `lane` for scalar wasm today. The workspace
  search (workflows, `scripts/`, `tools/`, `hosts/`, `sdk/`) found these legs:
  - in `wasm-guests`: the 18-package scalar build, the scalar evidence-crate check,
    `check-wasm-realtime-atomics.sh`, the scalar variant of `check-protocol-wasm-parity.sh`, and
    `run-wasm-gates.sh`'s scalar G5 guest;
  - in `cross-target`: the scalar rows of `check-cross-targets.sh`, `check-effect-package-v1.sh`
    and `check-effect-descriptor-v1.sh`.

  No other target the guard refuses is built anywhere in the workspace. Nightly, fuzz and
  npm-publish build only x86-64 or `simd128` wasm. `docs/TARGET_MATRIX.md` lists the legs.
  Deleting the one marked arm ends the exception.
- **Gate 3.** `scripts/check-cross-targets.sh` has a refusal row. It runs `cargo check --target
  armv7-linux-androideabi -p lane`, fails if that compiles, and fails if the error lacks the
  guard's message. The row also requires the target to be installed. The `cross-target` job in
  `qualification.yml` now installs `armv7-linux-androideabi` for it.
- **Comments and docs.** `Backend::current()`'s scalar arm is commented as reachable only through
  the exception. `docs/TARGET_MATRIX.md` gains a "64-bit only (issue #1041)" section and a
  "Refused" row, and its dispatch-contract sentence no longer says "`Scalar` otherwise". The stale
  x86 and wasm rows and the AArch64 rows are left for #1029 and #1017.

### Gate 1: refused and supported targets

`cargo check --locked --target <t> -p lane` gave these results before and after the change:

| target | base `4a0d60bd` | `4ef7c986` |
|---|---|---|
| `armv7-linux-androideabi` | builds (silent `Scalar`) | **refused**, only the new message, "1 previous error" |
| `thumbv7neon-linux-androideabi` | builds (silent `Scalar`) | **refused**, new message |
| `x86_64-unknown-linux-gnux32` | builds | **refused**, new message |
| `i686-linux-android` | refused by the x86-64-v3 guard | refused by both guards |

These still build at `4ef7c986`:

- `aarch64-apple-ios` and `aarch64-linux-android`: `cargo check` of `lane`, `capi`, `host-core`,
  `host-mobile`, `target-smoke`, `engine`, `session` and `protocol` passes, as does
  `cargo build --release -p lane`.
- `x86_64-unknown-linux-gnu`: workspace clippy, rustdoc and the release tests below pass.
- `wasm32-unknown-unknown` with `+simd128`: the artifact build below passes.
- `wasm32-unknown-unknown` with `-simd128`, under the exception: CI's exact 18-package
  `wasm-guests` scalar release build passes, and so do the scalar rows of `check-cross-targets.sh`.

`bash scripts/check-cross-targets.sh` prints PASS, ending "armv7 refused (#1041)". As a mutation
check, restoring the base `lib.rs` makes the script fail with "lane compiled for
armv7-linux-androideabi: the 64-bit-only guard (#1041) is gone".

### Gate 2: no rendered bit and no artifact byte changes

- `scripts/build-web-audioworklet.sh --module-only` was run at base and at `4ef7c986`. Both
  modules are `476e58ad74e8ddf92d12e6bb119c889447787cf3c25a7d916f9b142bae22fbf0`, `cmp`-identical
  and equal to `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`.
- `bash scripts/run-wasm-gates.sh --without-v8-spill` passes. It covers 142 cases and 358
  comparisons on each of three legs: native `Simd8`, wasm scalar (backend 0, the exception) and
  wasm `simd128`. All three have 0 mismatches against the pins.
- `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane` passes: G1-G4,
  G6, P1, M1-M3 and the native G5.

### Lint and policy gates

These all pass:

- `cargo fmt --all -- --check`;
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`;
- the check and test scripts for workspace, lane and artifact-evidence-leak policy;
- `check-unfused-seal.sh` and its `--self-test`;
- `check-effect-interchange-qualification.sh` and `test-effect-interchange-policy.sh`, which scan
  and mutate `check-cross-targets.sh`;
- `check-ci-path-routing.py` and `test-ci-path-routing.py`;
- CI's two sub-v3 x86 refusal probes, run locally.

### Open for review

- The scalar-wasm exception keeps `Backend::Scalar` reachable on wasm32. A `wasm32` build that
  forgets `+simd128` still compiles, but it is not the shipped artifact. The pin, `host.js`'s
  `simd128` probe and `check-browser-expected-resources.py`'s `+simd128` assertion still guard the
  artifact.
- The refusal proof is in the cross-target script, not in #1017's AArch64 leg, because that leg
  does not exist yet.

## Sol attempt 1 verdict: PASS

Sol, 2026-09-28, reviewing `4ef7c986` and `c56c94f8` against base `4a0d60bd`. Everything ran on
rustc 1.97.1 with `CARGO_INCREMENTAL=0` and scratch target directories.

### Gate 1: reproduced

- **Refused.** `cargo check -p lane` fails for `armv7-linux-androideabi`,
  `thumbv7neon-linux-androideabi` and `x86_64-unknown-linux-gnux32` with only the new message.
  `i686-linux-android` fails under both guards.
- **Still builds.** `lane` checks on `aarch64-apple-darwin` (the macOS host), `aarch64-apple-ios`
  and `aarch64-linux-android`. So does `--workspace --all-targets` on all three, minus four
  tools. `native-pcm-runner` and `stem-hasher` pull in blake3, and `wasm-console` and
  `wasm-gates` pull in wasmtime. Their C build scripts need a target sysroot this box lacks, and
  that failure has nothing to do with `lane`.
- **wasm32 and x86-64.** CI's 18-package scalar-wasm release build passes. The simd128 and
  scalar evidence-crate checks pass. CI's two sub-v3 x86 probes still fail with
  "requires x86-64-v3" only, and the AVX2+FMA probe builds.

### Gate 2: reproduced

- The AudioWorklet module is `476e58ad…` at both head and base. It is `cmp`-identical, and the
  head build passed the script's own pin check.
- `run-wasm-gates.sh --without-v8-spill` passes on all three legs (native backend 2, wasm 0 and
  wasm 1): 142 cases, 358 comparisons and 0 mismatches each.
- **New evidence.** A remapped, debuginfo-stripped x86-64 release `libcapi.so` is byte-identical
  at head and base (`d1bffffe…`). AArch64 binary identity was not measured, because this box has
  no NDK linker. On AArch64 the change is only a false `cfg` plus comments.

### Gate 3: reproduced

`check-cross-targets.sh` prints PASS. Two mutations, each run on the whole script, turn it red
with the matching diagnostic:

- base `lib.rs` gives "guard ... is gone";
- a reworded message gives "failed without lane's 64-bit-only message".

### Bypass analysis

No crate outside `lane` names `core::arch`, `std::arch` or `wide`, and `check-lane-policy.sh`
enforces that. Five workspace crates do not depend on `lane`: `engine`, `session`, `protocol`,
`dsp-reference` and `bench-support`. They carry no SIMD and no render kernels. `capi`, every
host, `graph` and every effect reach `lane` unconditionally.

`lane` has no Cargo features. `--cfg miso_wasm_simd8` and `+neon`/`+simd128` flags on armv7 still
hit the guard. Forging `--cfg target_arch="aarch64"` breaks the build rather than bypassing it.

### Scalar-wasm exception

The exception is bounded to `target_arch = "wasm32"` and cannot admit a native target. It is named
in the code and documented.

One gap in the documented inventory: `scripts/operator/preflight-wasm-console-benchmark.sh:201`
builds `wasm-console-guest` without `simd128`. It is an operator script, not a CI leg. Deleting the
marked arm must handle it too (issue #1039 may retire it first).

### CI, lint and policy

- **CI.** The `qualification.yml` change adds one target to `rustup target add` and a comment. No
  `if:`, job or verdict row changes. `check-ci-path-routing.py` and `test-ci-path-routing.py`
  pass, and the router classifies this diff `full`, so `cross-target` runs.
- **Lint.** fmt, workspace clippy with `-D warnings`, and rustdoc with `-D warnings` pass.
- **Policy.** These pass: the workspace, lane, realtime, host-core, artifact-evidence-leak,
  realtime-audit-leak and test-support checks and their tests; the unfused seal and
  `--self-test`; and effect-interchange qualification and its policy test. The worktree was
  clean afterwards.

### Non-blocking notes

- The `target_arch = "x86"` arms are now dead. They are in `Backend::current()` and in the copies
  in `target-smoke` and `soft-clip`.
- `aarch64_be` passes the guard, and there is no endianness guard anywhere. This is outside the
  brief.
- **Pre-existing on AArch64.** Two test-only warnings appear: `lane` `fp_env` has an unused
  import, and `host-core` `fp_environment` has dead code. These belong to #1017's leg.
- **Batch-level, not #1041.** `scripts/check-env-vocabulary.sh` already fails at base `4a0d60bd`.
  The cause is `docs/handoffs/test-value-2026-09-28/`, merged in `73e50f7d`. It will turn the
  batch's lint job red.
