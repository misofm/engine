# Remove the native WAV/RF64 decode workers

**Blocked on an owner ruling.** Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`,
section 7, R4. The ruling to record: "Sources are decoded by the host (the browser stem store and
PCM pump) and submitted as planar chunks. The engine carries no native file decoder or decode
worker threads." Part B depends on the C-ABI ruling (`R2-…`).

## Context

`crates/source` (10,787 lines) has three parts:

| part | production | tests | in the shipped module? |
|---|---:|---:|---|
| shared ring in `lib.rs` (`PcmSourceRing`, `TransferBlock`, `HostChunkProvider`, `prepare_graph_source_set`, telemetry) | about 1,819 | 2,164 | **yes, the browser path; stays** |
| native-only hooks in `lib.rs` (`cfg(not(target_arch = "wasm32"))`: module declarations and re-exports `:25-55`; `reserve_block`, `commit_block`, `commit_deferred` `:946-1100`; `retirement_worker` fields `:1635-2013`) | about 192 | — | no |
| `native_source.rs`: decode worker threads, resolver, session-level native prepare | 1,790 | 3,225 | no (cfg-excluded from `wasm32`) |
| `native_wave.rs`: RIFF/WAVE and RF64 parser and decoder | 925 | 672 | no |

**No shipping host uses native decode**, the C ABI included:

- capi takes host-decoded planar chunks (`miso_engine_v1_source_submit_planar_f32`), like the
  browser.
- `prepare_native_session_sources` (206 lines), `NativeSessionPreparedSources` and
  `NativeSourceController` have no caller outside `crates/source`.
- **`native_source` is used by** `tools/audit`'s `source` subject
  (`prepare_native_source_with_audit_gate`) and `source-duration` subject (`prepare_native_source`,
  `native_source_allocation_layout`), 855 lines. `crates/capi/tests/resource_lifecycle.rs:803-826`
  also mirrors its layout.
- **`native_wave` is used by** `tools/native-pcm-runner` (`R2-…`), `tools/stem-hasher` (883 lines;
  `wave` mode), and `tools/audit`'s `fixture-source` (1,181 lines; reads `fixtures/sources/v1`).
- **stem-hasher is not coupled to the browser.** `hosts/host-web/tests/stem-store-hash-v1.mjs` uses
  its own vectors and reads neither `fixtures/stem-identity/v1/` nor stem-hasher.
- **No console row uses native decode.** console-workload and bench submit PCM directly.

**CI.**

- audit-native: "Issue-544 source-duration runtime caller audit", the source half of the inline
  Issue-544 validator (`qualification.yml:585-687`), and `scripts/trace-source-audit.sh` (strace
  over `audit source`).
- `cargo test --release -p audit`: the `fixture-source` checker.
- test-debug-a: the `source` tests with `source/test-support`, and the stem-hasher tests.
- Each step is under 30 s.

**Issues:** open #124 (decode pool) closes as descoped.

## Smallest closable slice

**Part A (independent of `R2-…`):**

1. Delete `crates/source/src/native_source.rs`, the native-only hooks and re-exports in `lib.rs`,
   and the ring tests that exist only for the native producer. Keep every shared-ring test,
   porting any that used a native-only helper such as `commit_native` onto `HostChunkProvider`.
2. Delete `audit`'s `source` and `source-duration` subjects and `scripts/trace-source-audit.sh`.
   Delete the source-duration CI step and its validator half; if `R2-…` has landed, delete the
   whole Issue-544 validator.
3. Re-pin capi's layout mirror in `resource_lifecycle.rs` if `R2-…` has not removed capi.

**Part B (after `R2-…`):**

4. Delete `native_wave.rs`, `audit fixture-source` and `fixtures/sources/v1`.
5. Delete `tools/stem-hasher` and the WAV files of `fixtures/stem-identity/v1`, or keep stem-hasher
   in `raw` mode only. It is the reference oracle of `docs/STEM_IDENTITY_V1.md`; the owner decides
   whether a Rust oracle is still wanted when the browser implements the contract itself.
6. **Docs.**
   - AGENTS.md: "native WAV/RF64 decode workers fill bounded SPSC PCM rings" becomes "hosts decode
     and submit planar chunks into bounded rings".
   - Update `docs/STEM_IDENTITY_V1.md` and `docs/ENGINE_ENV_VOCABULARY.md`.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web -p source`
     passes.
2. **Console digests:** the `gain_pan_profile digests` output is byte-identical on base and
   change. No console row uses native decode.
3. **Shipped artifact.** The removed code is `cfg`'d out of `wasm32`, but deleting lines in
   `source/src/lib.rs` shifts panic line numbers. Build base and change on one machine, prove with
   `wasm-objdump -d` that only panic line numbers changed, and re-pin with that reason.
4. **CI routing.**
   - `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
   - The `verdict` table is unchanged.
   - `check-realtime-policy.sh`, `test-realtime-policy.sh`, `check-session-policy.sh` (which scans
     `fixtures/native-pcm-runner`) and `check-env-vocabulary.sh` pass.
5. **No live claim lost.**
   - The browser's source guarantees (bounded ring, generation-tagged seeks, underrun emits zero
     plus a counter) stay tested by the shared-ring tests in `source/src/lib.rs` and host-core's
     source tests. List each removed test with the native-only claim it held.
   - `cargo test -p source -p host-core` with `source/test-support` passes.

## Dependencies

The owner ruling. Part A after `00-…`; Part B after `R2-…`.

## Standing rules for the implementer

- Commit on `codex/<issue>-remove-native-decode`. Do not run timed benchmarks.
