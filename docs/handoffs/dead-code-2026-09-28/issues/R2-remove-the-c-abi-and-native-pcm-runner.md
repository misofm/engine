# Remove the C ABI and the native PCM runner

**Blocked on an owner ruling.** Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`,
section 7, R2. The ruling to record: "No native or cloud embedding is planned. The C ABI
(`miso_engine_v1_*`) and the native PCM reference runner are removed. Reopening them needs a new
issue that re-earns them." If the ruling keeps native embedding, close this draft and still do
`02-…`.

## Context

- **`crates/capi`: 9,812 Rust lines.** That is 3,645 production, 3,346 in-source tests and 2,821
  in `tests/resource_lifecycle.rs`. It also has `include/miso_engine_v1.h` (255 lines) and C/C++
  smoke tests in `tests/c/` (145 lines).
  - Its dependencies are `host-core[control-provider]`, `protocol`, `source`, `session`, `engine`
    and `lane`.
  - It is linked by `tools/audit` (the `capi` subject, `src/capi.rs`, 342 lines) and
    `tools/native-pcm-runner`.
- **`tools/native-pcm-runner`: 2,692 lines.** It decodes WAV/RF64 and renders through the C ABI
  (`docs/NATIVE_PCM_REFERENCE_RUNNER_V1.md`). `fixtures/native-pcm-runner/v1` holds 12 files and
  127,545 bytes, including 5 WAVs.
- **Code in product crates that exists only for capi** (SCIP cross-reference; re-prove by
  compile):
  - `crates/host-core/src/control_provider.rs` (874 lines), the `control-provider` feature, and
    its 11 `cfg` sites in `lib.rs`, `prepare.rs` and `render_session.rs`;
  - the engine's transactional plan replacement in `crates/engine/src/realtime/plan_exchange.rs`:
    `reserve_replacement` (`:262`), `epoch` (`:295`), `commit` (`:301`), `next_absolute_sample`
    (`:430`) and `render_contiguous` (`:453`);
  - `lane::fpenv::in_canonical_fp_environment` (`fpenv.rs:239`), and `lane::softfma::MXCSR_FTZ`
    and `MXCSR_DAZ` (`softfma.rs:74`, `:78`);
  - `host-core/src/source.rs:71` `diagnostic` and `:214` `region`;
  - `source/src/native_wave.rs:298` `region`, used by native-pcm-runner.
- **Browser dependency: none.**
  - `cargo tree -p host-web --target wasm32-unknown-unknown` has no `capi`.
  - `scripts/check-host-core-policy.sh:77-89` forbids host-web from enabling `control-provider`.
  - The shipped module exports `miso_engine_web_v1_*`, not the 14 frozen C symbols.
  - `sdk/assets/miso-engine-v1-abi-layout.json` describes the **wasm** ABI and stays.
  - native-pcm-runner is the oracle for no browser gate. `scripts/check-browser-expected-resources.py:14`
    only mentions `crates/capi/tests/resource_lifecycle.rs` as a "native mirror", and nothing runs
    it for the browser.
- **CI** (run 36382785722):
  - audit-native "C ABI linkage, frozen symbol set, native consumer smoke test, and self-test"
    (`scripts/check-capi-abi.sh`, 213 lines): 45 s;
  - "Issue-544 C ABI runtime caller audit": 1 s, plus the capi half of the inline Python validator
    (`qualification.yml:594-654`);
  - capi in the 99 s release build step (`:572-573`);
  - lint "Native PCM runner static seal and mutation tests" (`:423-427`): 9 s;
  - capi tests inside test-debug-a.
- **Policies that name capi:**
  - `check-release-shape.py:15-38`, `:218` (the expected cdylib/staticlib set);
  - `check-host-core-policy.sh` and its test;
  - `check-realtime-policy.sh`'s unsafe exclusions (`capi/src/ffi.rs`,
    `capi/tests/resource_lifecycle.rs`, `audit/src/capi.rs`, `native-pcm-runner/src/lib.rs`) and
    its test cases;
  - `check-conformance-boundaries.sh:111` and its test;
  - `check-artifact-evidence-leak.sh:52`;
  - `check-bench-policy.sh:215`.
- **Dead whatever the ruling:** `fixtures/capi-qualification/v1`, which `06-…` deletes.
- **What removal loses:**
  - The only native public-entry render audit (100,000 calls; 0 allocations, locks and
    syscalls). The browser has its own: the wasm call-graph and allocation checks in
    `scripts/check-web-audioworklet.sh`.
  - The only user of block-boundary plan replacement. AGENTS.md describes that architecture;
    amend it.
- **Issues:** open #895 (native runner I/O) closes as descoped.

## Smallest closable slice

1. Delete `crates/capi`, `tools/native-pcm-runner`, `fixtures/native-pcm-runner/`,
   `tools/audit/src/capi.rs` and its subject, `scripts/check-capi-abi.sh`,
   `scripts/check-native-pcm-runner.sh`, `scripts/test-native-pcm-runner-v1-policy.sh`,
   `scripts/test-native-pcm-runner-portability-v1-policy.sh`, `docs/C_ABI_V1_QUALIFICATION.md` and
   `docs/NATIVE_PCM_REFERENCE_RUNNER_V1.md`.
2. **CI.**
   - Remove `-p capi` from the release build step.
   - Delete the capi audit step and the capi half of the Issue-544 validator. Keep the
     source-duration half unless `R4-…` removes it.
   - Delete the C ABI linkage step and the lint native-pcm-runner step.
3. **Policies.** Update `check-release-shape.py`'s expected set, and the other policy scripts
   listed above, with their mutation tests.
4. **Then, in the same issue or a follow-up** (`S4b` in the audit): delete host-core's
   `control_provider.rs`, the `control-provider` feature and its `cfg` sites, and the engine, lane,
   source and host-core items listed above that are left with no user. Prove each deletion by
   compile.
5. **Docs.** Update AGENTS.md: "Expose a broad semantic control model and a narrow C ABI", "Build
   for native/cloud embedding" and "Deliver … PCM runner, host adapters".

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web -p host-core`
     passes.
   - `CARGO_PROFILE_RELEASE_PANIC=unwind cargo check --locked --release --workspace --all-targets`
     passes.
2. **Console digests:** the `gain_pan_profile digests` output is byte-identical on base and
   change. `bash scripts/run-wasm-gates.sh` passes.
3. **Shipped artifact.** Build base and change on one machine.
   - Step 4 removes `cfg(feature = "control-provider")` lines from `host-core` files that are in
     the module's closure (`prepare.rs:407`, `:1035`, `:1653`; `render_session.rs:235`, `:245`).
   - Those lines are not compiled for the browser, but deleting them shifts panic line numbers.
     Prove with `wasm-objdump -d` that only those shift, and re-pin with that reason.
4. **CI routing.**
   - `python3 -B scripts/check-release-shape.py --self-test` and `check-release-shape.py` pass.
   - `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
   - The `verdict` table is unchanged: steps are removed, no job is.
   - Every policy script and mutation test listed above passes.
5. **No live claim lost.**
   - The removed tests are capi's own, native-pcm-runner's own, and the capi audit's.
   - For each browser-relevant claim they held (no allocation or syscall in render, resource
     lifecycle), name the surviving browser or host-core gate:
     - `check-web-audioworklet.sh`'s allocation-free render closure;
     - `audit realtime` and `audit builtins-graph`;
     - `hosts/host-web/tests/boot_transient_budget.rs`.

## Dependencies

The owner ruling, then `00-…` and `02-…`. `R3-…` (protocol) must come after this: capi is the
protocol's only shipped consumer. `R4-…`'s WAV-parser slice is tied to this.

## Standing rules for the implementer

- Commit on `codex/<issue>-remove-c-abi`. Do not run timed benchmarks.
