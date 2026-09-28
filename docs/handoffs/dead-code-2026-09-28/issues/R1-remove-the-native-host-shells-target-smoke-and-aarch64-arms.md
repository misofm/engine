# Remove the native host shells, `target-smoke` and the AArch64 arms

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
