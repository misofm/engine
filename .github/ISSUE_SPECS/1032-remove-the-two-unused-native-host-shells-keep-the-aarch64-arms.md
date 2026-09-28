# Remove the two unused native host shells (keep the AArch64 arms)

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

**Owner ruling (2026-09-28):** Keep the AArch64 arms: native AArch64 is an official target. Remove only the two unused native host shells (the amendment's optional step); target smoke and the AArch64 CI leg belong to the new AArch64 qualification issue.

**Blocked on an owner ruling.** Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`,
section 7, R1. The ruling to record: "The engine ships only as the browser AudioWorklet. Native
x86-64-v3 remains a build target for tests, audits and benchmarks, not a product. AArch64 and the
mobile and native host shells are removed."

## Context

- **`hosts/host-native` (30 lines).** A binary that attests the CPU and the FP environment, then
  prints `target_smoke()`. Its audio callback is "deferred to issue 023".
- **`hosts/host-mobile` (26 lines).** A library whose only function, `mobile_target_smoke`, has
  zero references.
- **`crates/target-smoke` (87 lines).** Used only by the two shells, and as a compile subject in
  CI:
  - `qualification.yml:450`, `:456`, `:462`: the three x86 lint probes, `-p engine -p target-smoke -p math`;
  - `:768`: the scalar wasm build;
  - `:780`: the SIMD128 compile probe;
  - `scripts/check-wasm-realtime-atomics.sh:36`, `:41`, and its test.
- **The "Native host smoke" step** (`qualification.yml:496-497`, `cargo run -p host-native`) takes
  0 s.
- **AArch64 has no toolchain target and no CI job.** `rust-toolchain.toml` lists only `wasm32`.
  `docs/TARGET_MATRIX.md` records the owner ruling of 2026-09-04 (#378): native AArch64 is
  "unsupported, no claim". It has known red defects: LANE-3 (#366) and a Darwin `memset` in the
  SVF kernel.
- **The `cfg(target_arch = "aarch64")` arms:**
  - `crates/lane/src/fpenv.rs` (27 mentions, FPCR handling) and `crates/lane/src/backend.rs` (2);
  - `crates/soft-clip/src/lib.rs:906`;
  - `crates/graph/src/runtime.rs:323`, `:331`;
  - `tools/audit/src/vectorization.rs` (10);
  - `crates/lane/tests/fp_env.rs`, `crates/soft-clip/tests/support/mod.rs`;
  - the `target-smoke` test.

  The mentions in `effect-package` (target triples in package metadata), `compressor`,
  `transient-shaper`, `math` and the wasm-gate corpus are data or comments. They belong to `R6-…`
  or stay.
- **Policy scripts that name these crates:**
  - `scripts/check-artifact-evidence-leak.sh:52` (`shipped=(… host-mobile …)`);
  - `scripts/check-conformance-boundaries.sh:111` and `scripts/test-conformance-boundaries.sh`
    (`production_crates` includes `target-smoke`);
  - `scripts/test-host-core-policy.sh` and `scripts/test-bench-policy.sh` (use `hosts/host-native`
    as a mutation fixture path).
- **Browser dependency:** none. `host-web` links none of these, and the AArch64 arms are not
  compiled for `wasm32`.

## Smallest closable slice

1. Delete `hosts/host-native`, `hosts/host-mobile` and `crates/target-smoke`, and their workspace
   entries.
2. **CI.** Delete the "Native host smoke" step. Replace `-p target-smoke` with `-p lane` in the
   three x86 probes, the scalar wasm build (if `R8-…` has not removed it) and the SIMD128 probe
   (rename the step). Update `check-wasm-realtime-atomics.sh` and its test the same way.
3. **Policy scripts.** Drop `host-mobile` from `check-artifact-evidence-leak.sh`'s `shipped` list,
   and `target-smoke` from `production_crates`. Re-point the mutation fixtures in
   `test-host-core-policy.sh` and `test-bench-policy.sh` to `hosts/host-web`, keeping each case's
   intent.
4. **AArch64.**
   - Remove the `cfg(target_arch = "aarch64")` arms listed above.
   - Make `lane`'s backend selection refuse to compile on any target other than x86-64-v3 and
     `wasm32` with `simd128`. Use the same style as the existing sub-v3 guard (a
     `compile_error!` naming the supported targets), so an unsupported target can never silently
     fall back to scalar.
   - Keep `Backend::Scalar` and the tail code: those are `R8-…`'s subject.
5. **Docs.** Update `docs/TARGET_MATRIX.md` (if `06-…` has not already), and AGENTS.md's mission
   ("Build for native/cloud embedding, iOS, Android…"), SIMD line ("AArch64 NEON four-lane") and
   sources line ("Browser/mobile hosts provide decoded chunks") to match the ruling.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web -p lane`
     passes.
   - The three x86 probes behave as before: the two sub-v3 builds are still refused with
     `requires x86-64-v3`, and the AVX2+FMA check passes.
   - Optional, because it needs the target's standard library installed:
     `cargo check -p lane --target aarch64-unknown-linux-gnu` fails with the new
     `compile_error!`. Record the message.
2. **Console digests:** the `gain_pan_profile digests` output is byte-identical on base and
   change. `bash scripts/run-wasm-gates.sh` passes.
3. **Shipped artifact: byte-identical.** Build base and change with
   `scripts/build-web-audioworklet.sh --module-only EMPTY_DIR` on one machine. No file in the module's
   closure changes, except `lane` and `graph`, whose AArch64 arms are not compiled for `wasm32`. If
   the hash moves, prove with `wasm-objdump -d` that only panic line numbers changed, and re-pin
   with that reason.
4. **CI routing.**
   - `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
   - `check-workspace-policy.sh`, `test-workspace-policy.sh`, `check-host-core-policy.sh`,
     `test-host-core-policy.sh`, `check-bench-policy.sh`, `test-bench-policy.sh`,
     `check-conformance-boundaries.sh`, `test-conformance-boundaries.sh`,
     `check-artifact-evidence-leak.sh` and `test-artifact-evidence-leak.sh` pass.
   - The `verdict` table is unchanged: no job is removed, only one step.
5. **No live claim lost.** The only tests removed are `target-smoke`'s one test and the AArch64
   `cfg` assertions. No shipped target is affected. List them from the `-- --list` diff (audit
   section 11).

## Dependencies

The owner ruling. It is independent of `R2-…` to `R10-…`. Land it after `00-…`.

## Standing rules for the implementer

- Commit on `codex/<issue>-remove-native-shells`. Do not run timed benchmarks.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, findings F3 and F4. **The owner corrected the product scope after the
audit: fans open and play sessions in native iOS and Android apps as well as the web app.** Mobile
embedding, and therefore native AArch64, is live scope. This draft's premise ("the engine ships
only as the browser AudioWorklet") no longer holds.

1. **Do not remove the AArch64 arms.** They are the mobile product path:
   - `lane::Backend::current()` gives `Simd4` (NEON) on `aarch64` (`crates/lane/src/backend.rs:36-39`);
   - `crates/lane/src/fpenv.rs`'s FPCR flush-to-zero handling is what makes AArch64 renders
     denormal-safe, the counterpart of the x86 MXCSR code;
   - `graph/src/runtime.rs:323`, `:331` and `soft-clip/src/lib.rs:906` are their AArch64 lowering.

   Step 4 (delete the arms, and `compile_error!` on every target except x86-64-v3 and
   `wasm32`+`simd128`) would make the mobile build impossible. Drop step 4 entirely.
2. **What is true today for mobile** (checked with Rust 1.98.1's `aarch64-apple-ios` and
   `aarch64-linux-android` standard libraries; no NDK, Xcode or device, so compile only):
   - every product crate, `capi` and `protocol` included, passes `cargo check --all-targets
     --all-features` for both targets. A workspace-wide iOS check fails only on tool crates whose
     build scripts need `xcrun` (`blake3` via native-pcm-runner and stem-hasher; `wasmtime` via
     wasm-gates and wasm-console);
   - none of the audit's 129 dead items is referenced from any AArch64-only code path;
   - seven test-only warnings appear on aarch64 (`host-core/tests/fp_environment.rs:25,26,52,63,81,102`,
     `lane/tests/fp_env.rs:14`), which CI's `-D warnings` would reject if an aarch64 leg existed;
   - **no CI job builds, lints or tests any AArch64 target.** `scripts/check-cross-targets.sh:4-5`
     records that the Android and iOS rows were removed under #378;
   - both register defects in `docs/TARGET_MATRIX.md` are still present: 151 `memset_pattern16`
     calls in `parametric-eq`'s iOS release assembly (reproduced) and `fmaxnm`/`fminnm` folds in
     the compressor's Android release assembly (LANE-3, #366, closed as deferred, not fixed). The
     first is a realtime-rule breach inside render on Apple targets; the second makes mobile output
     differ from the x86 and browser oracles bit for bit;
   - 32-bit `armeabi-v7a` Android falls silently to `Backend::Scalar` (`backend.rs:60-68`; only x86
     has a `compile_error!` guard, `lane/src/lib.rs:72-80`), and x86-64 Android emulators and iOS
     simulators inherit the AVX2+FMA pin from `.cargo/config.toml`, so the engine refuses to start
     on a CPU without them.
3. **Rulings that must be revisited before mobile work:** #378 ("native AArch64 unsupported, no
   claim") and #023 ("iOS and Android embedding examples", closed 2026-08-22), with spec 001's "Mobile support is browser-based; native iOS and Android embedding targets are deferred" (`.github/ISSUE_SPECS/001-…md:43`).
4. **What is still genuinely unneeded:** `hosts/host-mobile` (26 lines, one function nothing calls)
   and `hosts/host-native` (30 lines) are stubs that no mobile app would link; the real mobile
   surface is `crates/capi`. `target-smoke` is worth keeping only as the width assertion of a future
   aarch64 CI leg.
5. **Replacement recommendation (owner ruling needed):** "Native AArch64 (iOS, Android arm64-v8a)
   is a product target." Then, as separate issues rather than this draft:
   - (a) an aarch64 CI leg: `cargo check`/`clippy -D warnings` for `aarch64-apple-ios` and
     `aarch64-linux-android` on the product crates, plus `cargo test` of `lane`, `math` and the
     console digests under an aarch64 runner or emulator;
   - (b) fix LANE-3 (#366) and the Darwin `memset_pattern16` in the SVF flush;
   - (c) rule which Android ABIs ship. If `armeabi-v7a` ships, the whole-plan Scalar path is its
     production path (see R8); if not, add a `compile_error!` for 32-bit ARM like the x86 guard;
   - (d) decide whether emulator/simulator builds need a non-v3 x86 profile.

   Removing the two stub hosts may stay in this draft, reduced to steps 1-3 without
   `target-smoke`, if the owner wants it.
6. **Gates for whatever remains:** add `cargo +<toolchain with aarch64 std> check --target
   aarch64-apple-ios` and `--target aarch64-linux-android` for `-p capi -p host-core -p lane -p graph
   -p soft-clip` on base and change.
