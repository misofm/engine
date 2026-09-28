# Delete the items the compiler proves unused

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 2 ("Truly dead
code"). No owner ruling is needed: nothing in any build, test, tool or target uses these items.

## Context

The audit demoted every workspace item that rust-analyzer's cross-reference index (SCIP) showed with
zero references from `pub` to private in a scratch copy, then let rustc's `dead_code` lint judge
them. It ran `cargo check --workspace --lib --bins --examples --all-features` (non-test build) and
`cargo check --workspace --tests --all-features` (test build) on x86-64-v3, and
`cargo check --target wasm32-unknown-unknown` (simd128) over the browser closure. An item is listed
below only when rustc reported it unused in **both** the non-test and the test build, and no
integration test, tool or other crate failed to compile after the demotion (which would have meant
a real user). Items reachable only from other dead items (the "cascade") are marked.

The audit found 129 such items (about 910 lines including doc comments).

- The table below holds the 104 that this draft deletes: about 720 lines in 18 crates.
- The other 25 belong to other drafts and are listed under "Out of scope":
  - `protocol` has 20;
  - `host-core`'s endpoint files have 4 (`record_count`, the `report` field and method, and
    `fault`);
  - `host-mobile` has 1.

It is small because the Rust tree is otherwise very clean at the item level. `unreachable_pub`,
`missing_docs` and clippy `-D warnings` already keep private dead code out.

| crate | items | items (file:line at `a9414c0c`) |
|---|---|---|
| `builtins` | 8 | `lib.rs:67` `ChannelLinkMode` (cascade); `:394` `DISABLED_LATTICE_INDEX`; `:3186` `link_mode`; `:3379` `reset_lifetime_recovered_state`; `:3650` `trim_target`; `:3817` `target_gain`; `:3826` `is_muted`; `:4326` `balance_matrix` |
| `builtins-compiler` | 2 | `lib.rs:4071` `classes` (cascade); `:4079` `mono_track_count` |
| `conformance` | 1 | `block.rs:69` `channel` |
| `dsp-reference` | 23 | `block.rs:34` `from_samples`; `compressor.rs:104` `latency_samples`; `delay.rs:38` `delay_target_ms`, `:63` `set_delay`, `:117` `sample_rate_hz`, `:150` `set_delay`; `lr4.rs:86` `reset`; `parametric_eq.rs:198` `is_identity`, `:285` `coefficients`; `svf.rs:393` `reset` (cascade), `:400` `state`; `tpt.rs:70` `output` (field), `:144` `output`, `:284` `magnitude_db`; `transient_shaper.rs:90` `fast_envelope`, `:96` `slow_envelope`; `true_peak_limiter.rs:191-193` fields `sample_rate_hz`, `parameters`, `n`, `:283` `required_delay`, `:295` `parameters`, `:389` `gain`, `:394` `reset` |
| `effect-compiler` | 4 | `migration.rs:170` `new`, `:189` `step`, `:389` `replay`; `prepare.rs:1245` `empty` |
| `effect-contract` | 7 | `lib.rs:211` `new`, `:481` `as_str`, `:1459` `snap`, `:2147` `is_empty`; `live.rs:207` `capacity`, `:820` `retained_bytes`; `step.rs:23` `as_str` |
| `effect-package` | 4 | `cid.rs:60` `verify_raw_bytes`; `state.rs:847` `as_bytes`, `:884` `latency_samples`, `:887` `tail` |
| `engine` | 23 | `realtime/buffer.rs:73` `count`, `:78` `total_samples`, `:125` `clear`, `:133`/`:136` fields `storage`/`stride`, `:148` `try_new`, `:173` `plane`, `:271` `plane_range`; `realtime/disjoint.rs:104` `planes`, `:116` `frames`, `:519` `reserved`, `:535` `total_bytes`; `realtime/plan_exchange.rs:436` `copy_response_snapshot`, `:497` `copy_worker_audit_snapshots`, `:504` `dispatch_counters`; `realtime/plan.rs:466`/`:864` `copy_worker_audit_snapshots` and `:490`/`:656` `dispatch_counters` (cascade: auxiliary-worker remnants of the removed dependency-wave renderer); `realtime/spsc.rs:323` `overflow_count`, `:416` `generation`, `:431` `underrun_count`, `:456` `is_empty` |
| `graph` | 1 | `lib.rs:2905` `quantum_samples` |
| `host-core` | 6 | `control_preparation.rs:457` `factory`; `render_session.rs:104` `render_contiguous`, `:121` `render`, `:181` `copy_response_snapshot`; `source.rs:228` `is_empty`; `spectrum.rs:2953` `DEFAULT_SMOOTHING_MS` |
| `host-web` | 4 | `lib.rs:224` `SPECTRUM_WINDOW_BYTES` (cascade), `:226` `SPECTRUM_RESULT_BYTES`, `:232` `SPECTRUM_MAXIMUM_RESULT_BYTES`, `:495` `OBSERVATION_STATUS_BYTES` |
| `lane` | 2 | `kernels/builtins.rs:35` `all_lanes`, `:130` `nonfinite_lanes_block` |
| `parametric-eq` | 4 | `lib.rs:239` `reset_ramping_elided_blocks`, `:244` `ramping_elided_block_count`, `:250` `test_only_reset_ramping_elided_blocks`, `:258` `test_only_ramping_elided_blocks` (a `test-support` hook no test calls) |
| `rack` | 5 | `lib.rs:600`/`:3233` `disarm_observations` (one is cascade), `:906` `designed_lane_witness`, `:1575` `left_mut`, `:1579` `right_mut` |
| `rack-compiler` | 1 | `lib.rs:93` `name` |
| `session` | 3 | `compile.rs:57` field `graph_entity_indexes`, `:98` `graph_entity_index`; `diagnostic.rs:153` `id` |
| `source` | 5 | `lib.rs:398` and `:429` `diagnostic_code`, `:813` `channel_count`, `:819` `quantum_frames`, `:1268` `read_block_contiguous` |
| `stem-hasher` | 1 | `lib.rs:44` `token` |

Also in scope:

- **`crates/rack-compiler/Cargo.toml`: the `engine` dependency is unused.** `cargo machete` flags it,
  `crates/rack-compiler/src/lib.rs` never names `engine`, and the audit built
  `cargo check -p rack-compiler --all-targets` with the line removed.

- **47 `#[allow(dead_code)]` attributes that suppress nothing.** The audit stripped all 59
  non-test ones and ran `cargo check --lib`, natively and on `wasm32`. Only 12 warnings appeared:
  - Layout mirrors in `graph` (`lib.rs:2459`, `:2471`; `runtime.rs:988`, `:1035`, `:2109`,
    `:2262`, `:2287`), which keep theirs.
  - `capi/src/runtime/compile.rs:72`, which keeps its own.
  - Members used only by tests: `graph-compiler/src/canonical.rs:106` `edge_text_len`;
    `host-core/src/spectrum.rs:1643` `cadence`, `:1650` `observer_handle`, `:1780` `stage`;
    `host-web/src/lib.rs:2550` `boot_with_observation_demand`. These move behind `#[cfg(test)]`
    (or a `cfg_attr(not(test), expect(dead_code))`) instead of a blanket `allow`.

  The other 47 attributes go, including 17 in `host-web/src/observation_ingress.rs` and 16 in
  `host-web/src/lib.rs`.

Out of scope (each belongs to another draft, so it is not deleted twice):

- `protocol` (20 items) and `host-core`'s `builtin_batch_endpoint.rs`/`scalar_point_endpoint.rs`
  items (`record_count`, `report`, `fault`): see `02-...` and the protocol ruling (`R3-...`). If the
  owner keeps `protocol`, delete its 20 items here in a follow-up.
- `host-mobile::mobile_target_smoke`: the whole crate is `R1-...`.
- `session::CompileCaps`'s four fields documented as inert since #241
  (`crates/session/src/compile.rs:17-43`): public fields set by callers, so removing them edits every
  caller. File separately if wanted.

## Smallest closable slice

1. Delete every item in the table and the `engine` dependency of `rack-compiler`. Do not add
   `#[allow(dead_code)]` anywhere. If deleting an item makes another item unused, delete that too
   and list it in the evidence.
2. Remove the 47 no-op `#[allow(dead_code)]` attributes. Move the five test-only members listed
   above behind `#[cfg(test)]`.
3. If rustc reports that an item **is** used on some target or feature set the audit did not
   build, keep it and record where it is used.
4. Update doc comments that describe a deleted item. Do not touch `docs/handoffs/`.

## Objective gates

1. **Native build:** `cargo check --locked --workspace --all-targets --all-features`,
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` and
   `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` pass.
2. **Wasm build:** `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target
   wasm32-unknown-unknown -p host-web -p host-core` passes, and so does the wasm-guests job's
   scalar build line in `.github/workflows/qualification.yml` while that job exists.
3. **Console digests unchanged:** the output of
   `cargo test --locked --release -p console-workload --test gain_pan_profile -- --ignored --exact digests --nocapture`
   is byte-identical on the base commit and on the change (all standing 64-block console digests),
   and `bash scripts/run-wasm-gates.sh` passes.
4. **Shipped artifact:** build it with `bash scripts/build-web-audioworklet.sh --module-only EMPTY_DIR`
   on base and change, **on the same machine**. The digest depends on whether the toolchain's
   `rust-src` component is installed (audit section 9), so do not compare a local build against
   the committed pin.
   - The two modules must be byte-identical, or differ only in panic-location line numbers in the
     data section. Prove the second case by showing that `wasm-objdump -d` is identical function
     by function and the export list is identical.
   - Re-pin `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` only in that second
     case, and state the reason in the evidence. The CI `artifact` job is the authority.
   - Run `python3 -B scripts/check-browser-expected-resources.py`, and re-pin only what moves,
     with the reason.
5. **Tests (no live claim lost):** `cargo test --locked --workspace --all-targets` passes with the CI feature sets of
   the `test-debug-a` and `test-debug-b` jobs, plus the `test-support` features `00-...` adds. The
   `-- --list` output is identical on base and change: no test is deleted, because no test
   references these items.
6. **CI routing:** no workflow, script or router change is needed; `python3 -B scripts/check-ci-path-routing.py`
   and `python3 -B scripts/test-ci-path-routing.py` pass.

## Dependencies

`00-...` first, so gate 5 covers the `test-support` tests as well. Land this before `02-...` and
the ruling-dependent removals so their diffs stay small.

## Standing rules for the implementer

- No product behaviour change; a moved console digest is a hard stop.
- Commit on `codex/<issue>-delete-unused-items`. Do not run timed benchmarks.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, findings F8 and F3. A scratch copy with **47 of the 129 items deleted
outright** (18 crates, every host-web constant, the engine plan/exchange wrappers, lane, source,
rack, dsp-reference, builtins, session, effect-contract, effect-package, graph, rack-compiler with
its `engine` dependency, conformance, effect-compiler, builtins-compiler, host-core and stem-hasher)
was built natively (`--workspace --all-targets --all-features`), for `wasm32` `simd128`
(`-p host-web -p host-core`), for the scalar-wasm package list, for the fuzz bins, and with Rust
1.98.1 for `aarch64-apple-ios` and `aarch64-linux-android` (`--workspace --lib --all-features`,
excluding the two tools whose `blake3` build script needs a cross C compiler) and
`aarch64-unknown-linux-gnu` `--all-targets` for lane, soft-clip, graph, engine, capi and host-core.
45 of the 47 are dead on every one of those builds. Two are not:

1. **`parametric-eq` `lib.rs:239` `reset_ramping_elided_blocks` and `:244`
   `ramping_elided_block_count` are used.** The `#[cfg(test)] mod ramping_elision` (`lib.rs:6379`)
   calls them at `:7601`, `:7604` and `:7677`; deleting them gives E0425 in the `parametric-eq` lib
   test build. Delete only the two `test_only_*` hooks (`:250`, `:258`) and narrow the two helpers
   from `cfg(any(test, feature = "test-support"))` to `#[cfg(test)]`, or clippy `--all-features`
   warns on them. So the table's parametric-eq row is 2 deletions plus 2 narrowings, and the
   "unused in both builds" statement is wrong for these two.
2. **`Cargo.lock` changes.** Removing `rack-compiler`'s `engine` dependency rewrites the lockfile.
   Commit it in the same change, or every `--locked` gate fails.
3. **Expected knock-on warnings** (clippy `-D warnings` fails until removed): unused imports
   `ResponseSnapshot*` (`engine/src/realtime/plan_exchange.rs:15`), `QuantumFrames`
   (`graph/src/lib.rs:230`), `RenderTime` (`host-core/src/render_session.rs:35`). Step 1 already
   covers these ("delete that too and list it").
4. **The no-op `allow(dead_code)` count is at most 45, not 47.** Stripping all 56 non-test
   attributes (55 outer, plus the inner `#![allow(dead_code)]` at `engine/src/realtime/plan.rs:3`,
   which suppresses nothing) leaves 13 warnings natively, 11 on `wasm32` and 12 on iOS. The draft's
   keep-list misses `tools/native-pcm-runner/src/lib.rs:733` (`fault`, read only by test fault
   injection at `:1004-1159`). No extra warning appears on aarch64.
5. **Mobile scope (the owner's correction): no item here is used by AArch64-only code**, and all
   of them compile out of the capi closure cleanly. But the draft's "Out of scope" list changes:
   - the 20 `protocol` items: R3 is now retired (the C ABI needs the protocol), so delete them in a
     protocol-only follow-up of this draft, with the same gates;
   - `host-mobile::mobile_target_smoke`: goes only if R1's reduced stub removal is ruled.
6. **Non-Rust consumers checked:** the four host-web constants have none. The SDK reads those sizes
   through `miso_engine_web_v1_spectrum_result_bytes` (`hosts/host-web/src/ffi.rs:3127`) and
   `…_observation_status_bytes` (`:4832`), which compute their own values;
   `tools/parameter-metadata/src/abi_layout.rs` imports none of them; `node sdk/codegen/assets.mjs
   --check` passes on the scratch tree.
7. **Add gates:** an aarch64 `cargo check --lib` for `-p lane -p engine -p host-core -p capi -p
   source` (a toolchain with the aarch64 std; CI has none yet), and `cargo check --manifest-path
   fuzz/Cargo.toml --bins`.

## Attempt 1 evidence

Implementer: Terra (Claude Opus 5.5), branch `codex/1023-delete-proven-dead-items` from
`codex/batch-slim-1` at `4a0d60bd`. Commits `7594307d` (the item deletions), `7829d9fd` (the
`allow(dead_code)` pass), `528fee3f` (render input kept for #1024, the rack policy pin and the capi
re-pins) and `fa708d0b` (protocol and browser re-pins), plus this record. Nothing is pushed.

**Size:** 52 files, 845 lines deleted and 81 added, net **-764** (Rust -763). The item count is 95
of the table's 104 deleted, 2 narrowed to `cfg(test)`, and 7 kept for the reasons below. There
are 51 `allow(dead_code)` attributes removed and none added.

### Deleted beyond the table (step 1's cascade rule)

- **`ConsoleEffectBankStage`'s override of `BankStage::disarm_observations`.** The trait method is
  in the table. The inherent `ConsoleEffectBankStage::disarm_observations` stays, because
  `rack/tests/console_bank.rs:628` calls it.
- **`compile_session`'s `graph_entity_indexes` build.** It goes with the field (the `indexed(..)`
  call over tracks, submixes and outputs). Its only failure mode was an index that does not fit in
  a `u64`.
- **`ControlledSpectrumCandidate::cadence` (host-core `spectrum.rs`).** No build reads this field,
  tests included: `cargo check -p host-core --profile test --lib` warns. So it is deleted, not
  moved behind `cfg(test)` as the spec proposed. `stage_prepared_entry` still passes `cadence` to
  `reset_for_controlled_stage`.
- **parametric-eq's `RAMPING_ELIDED_BLOCKS` and `count_ramping_plan`.** These, and the counter's
  two call sites, narrow to `#[cfg(test)]` along with Amendment 1's two helpers. The deleted
  `test_only_*` hooks were the counter's only readers in a `test-support` build.
- **Leftovers:**
  - five impl blocks left empty (`RealtimePlanOwner`'s second block, `HostChunkError`,
    `SourceSeekError`, `ObservationTapId`, `DescriptorDiagnosticCode`);
  - Amendment 3's imports. `QuantumFrames` moves into graph's test module, whose tests use it.
- **A misplaced doc.** At the base, `ConsoleEffectBankStage`'s #140 doc block sat on
  `designed_lane_witness`. It now sits on the struct.
- **Docs that named deleted items now name what remains:**
  - `StartedRenderSession::render_planar` is host-core's guarded entry. It inherits the removed
    `render_contiguous`'s environment and error paragraph.
  - The D7 check is described in place of `nonfinite_lanes_block`.
  - `wide_impl.rs`'s list of mask producers drops `all_lanes`.
  - `BuiltinLatticePoints::disabled` no longer links `DISABLED_LATTICE_INDEX`.

### Kept (step 3), with the reason

- **`NativeEffectRegistry::is_empty` (effect-contract `lib.rs:2147`) and
  `SourceControlSet::is_empty` (host-core `source.rs:228`).** rustc proves both unused. But each
  sits beside a public `len`, and without it `clippy::len_without_is_empty` fails the lint job's
  `-D warnings` (natively and on both aarch64 targets). The audit's proof was rustc-only.
  `NativeEffectRegistry::len` also feeds the generated metadata
  (`tools/parameter-metadata/src/lib.rs:272`).
- **engine `PlanarBufferRef`'s `try_new`, `plane`, `plane_range` and fields `storage`/`stride`
  (`buffer.rs:133-288`): moved to #1024.** Deleting the fields removes the struct's null-pointer
  niche, so `Option<PlanarBufferRef>` in `RenderIo` grows a tag. In the shipped module,
  `PreparedRenderPlan::render_inner` then changed instruction by instruction: a tag compare
  replaced the null check, and the blocks were reordered (298 to 323 disassembly lines). That was
  the only render-path function to change. #1024 removes the whole render input, and its spec
  already expects the pin to move.
- **parametric-eq `reset_ramping_elided_blocks` and `ramping_elided_block_count`:** narrowed to
  `#[cfg(test)]`, per Amendment 1.

### `allow(dead_code)` (step 2)

I stripped all 60 plain `#[allow(dead_code)]`/`#![allow(dead_code)]` attributes outside `tests/`
directories, in 12 files. Then I built:

- native `--lib --bins --examples --all-features`;
- native `--all-targets --all-features`;
- native default features;
- wasm32 `simd128` (`-p host-web -p host-core`);
- the wasm-guests scalar package list;
- iOS and Android `--workspace --lib --all-features`.

That gave 15 warnings, all native. wasm32 and aarch64 added none.

- **Kept (10):**
  - graph's seven layout mirrors (`lib.rs` 2, `runtime.rs` 5);
  - `capi/src/runtime/compile.rs:70`;
  - native-pcm-runner's `fault` field (Amendment 4);
  - **rack `lib.rs:1450` `has_active_lanes_scan`, a finding.** It is a `#[cfg(test)]` helper that
    no test calls, so its attribute hides a *test-build* warning. The audit's `--lib`-only strip
    could not see it. I kept it rather than deleting it, because this is a test-value question:
    without a caller, `prepared_slot_dispatch_uses_constant_activity_checks`'s
    `lane_inspections == 0` assertion cannot fail.
- **Moved behind `#[cfg(test)]` (5):**
  - spectrum's `observer_handle` and `stage`;
  - host-web's `boot_with_observation_demand`;
  - graph-compiler's `edge_text_len` (its `cfg_attr(not(test), allow(dead_code))` became
    `#[cfg(test)]`);
  - bench's `digest`, which moved into its test module together with the `sha2` import it alone
    used.
- **Deleted: spectrum's `cadence`** (above).

That makes 51 attributes removed: `observation_ingress.rs` 17, host-web `lib.rs` 17,
`spectrum.rs` 11, and one each in `plan.rs` (the inner one), `ffi.rs`,
`builtin_batch_endpoint.rs`, native-pcm-runner's enum, bench and `canonical.rs`.

### Changes the spec said would not be needed

- **Script (gate 6 said "no script change").** `scripts/check-rack-policy.sh` pinned
  rack-compiler's dependencies to exactly `effect-contract`, `engine` and `rack`. It now pins
  `effect-contract` and `rack`, and `test-rack-policy.sh`'s fixture follows. The
  `test-workspace-policy.sh` failure that surfaced this is gone.
- **Size re-pins (no test deleted, no claim changed).** Each follows the 24-byte (64-bit)
  `BTreeMap` header that every retained `CompiledSession` no longer carries:
  - `capi/tests/resource_lifecycle.rs`: active CAPI 160,981 to 160,957; prepared protocol 24,736 to
    24,712; double-live CAPI and `oracle.capi` 204,423 to 204,375; the two
    `frozen_scratch_report` calls; and the tiny-frame base 178,514 to 178,490.
  - `protocol/src/controller/tests.rs`: `ProtocolController` 6,064 to 6,040 and
    `PreparedStructuralCommand` 752 to 728.
  - `hosts/host-web/tests/browser-v1/expected.json` (wasm32):
    - `bridgeMetadataBytes` 1,149,263 to 1,149,255;
    - `bridgeRetainedBytes` 1,169,772 to 1,169,764;
    - derived by `check-browser-expected-resources.py`; no other row moved.

### Gates

All runs were on one machine, with toolchain 1.97.1 and no `rust-src` installed. A cleanup of the
shared scratch directory by another agent deleted my first base worktree mid-run. The base
(`4a0d60bd`) was rebuilt in a fresh detached worktree with its own target directories (F21), and
every comparison below uses that rebuild.

1. **Native.** All pass:
   - `cargo check --locked --workspace --all-targets --all-features` (no warnings);
   - `cargo clippy … --all-targets --all-features -- -D warnings`;
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`;
   - `cargo fmt --all -- --check`.
2. **wasm32.** All pass with no warnings:
   - `simd128` check of `-p host-web -p host-core`;
   - the wasm-guests scalar `cargo build --release` line (18 packages);
   - the `dsp-reference`/`conformance` scalar and `simd128` checks;
   - `cargo check --manifest-path fuzz/Cargo.toml --bins`.
3. **aarch64 (Amendment 7).** The pinned 1.97.1 has the `aarch64-apple-ios` and
   `aarch64-linux-android` standard libraries, so no other toolchain was needed. On both targets:
   - **Passes:**
     - `cargo check` and `cargo clippy -- -D warnings` of `--workspace --lib --all-features`;
     - `cargo check --all-targets --all-features` of lane, engine, host-core, capi, source, graph
       and soft-clip.
   - **Excluded from the workspace check:** `native-pcm-runner` and `stem-hasher` (the `blake3`
     build script needs a C cross-compiler), and `wasm-console` and `wasm-gates` (`wasmtime`'s
     build needs one too).
   - **Warnings that predate this change:** `--all-targets` warns in `lane/tests/fp_env.rs:14` and
     `host-core/tests/fp_environment.rs`, test bodies gated to x86. The base shows the same
     warnings.
4. **Console digests.** All 17 digest rows are byte-identical between base and change. Only the
   `finished in` elapsed-time line differs. `bash scripts/run-wasm-gates.sh` passes: native, wasm
   scalar, wasm `simd128` and the V8 spill gate.
5. **Shipped artifact.**
   - **Hashes:** the base is `476e58ad…`, equal to the pin. The change is `3f744b03e22e…25da`.
   - **Size:** 3,498,409 to 3,486,775 bytes (-11,634). The export list is identical (135 exports).
   - **Not re-pinned:** this is not the panic-line-only case, and the batch re-pins at its
     boundary. The CI `artifact` job is the authority.
   - **Function-by-function `wasm-objdump -d`.** Symbol names were normalized for the crate-hash
     change (below). The function count went from 2,749 to 2,736:
     - 2,181 functions are identical.
     - 544 differ only in integer constants and load/store offsets:
       - static addresses shift, mostly by -48 (the data section is 48 bytes shorter; see the vtable
         slots below);
       - field offsets shift in the hosts that embed a smaller `CompiledSession`.
     - 24 base and 11 change bodies differ in instructions, all control-plane:
       - **Removed: four bodies reachable only through vtables.** These are
         `BankStage::disarm_observations` for `EffectBankStage` and `ConsoleEffectBankStage`, and
         `GraphExecutor`'s `PreparedPlanExecutor::dispatch_counters` and
         `copy_worker_audit_snapshots` defaults. Their vtable slots are the data section's lost 48
         bytes, and the element segment is 8 bytes shorter.
       - **Removed: the `graph_entity_indexes` build.** That is `session::compile::indexed` over
         the chained iterator, its `GenericShunt`, the `(StableId, u64)` stable-sort trio,
         `BTreeMap::bulk_build_from_sorted_iter`, and four per-crate `BTreeMap<StableId, u64>`
         drop glues.
       - **Changed: code that drops or builds a `CompiledSession`.** `compile_session`,
         `compile_host_model`, `compile_ready`, `prepare_native_session_effects`, and the drop
         glue of `ReadyOwnership`, `CompiledSession` and `EffectPreparedSession` now handle one map
         fewer, and their inlining shifts.
       - **Changed: the two functions that stored `cadence`.**
         `ControlledSpectrumCaptureCollection::stage_prepared_entry` and
         `HostObservationController::publish_candidate`.
     - **Crate hashes.** Removing rack-compiler's dependency changed the crate disambiguators of
       rack-compiler, builtins-compiler, graph-compiler, host-core and host-web. That moves the
       `name` section, and six `v128.const` `TypeId`s in builtins-compiler.
     - **Render path:** `render_inner`, `render_next` and every kernel are identical, or differ in
       constants and offsets only.
6. **Tests (gate 5).** The `-- --list` output is identical between base and change (and so is the
   list of test binaries run) for:
   - the `test-debug-a` set (1,534 tests);
   - the `test-debug-b` set (834);
   - `--workspace --all-targets` (2,552).

   Runs with `--no-fail-fast`:
   - `test-debug-a`: 1,526 passed, 0 failed.
   - `test-debug-b`: 807 passed, 0 failed.
   - `cargo test --workspace`: 2,533 passed, 0 failed. This run includes the doc tests.
   - The first `test-debug-a` run failed only the protocol size pin, which is now re-pinned.
7. **CI routing.** `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
8. **Policy and generated surfaces.**
   - **These pass:**
     - `check-`/`test-` workspace, realtime, lane, graph, rack, builtins, host-core,
       effect-runtime, bench and protocol-control policy;
     - `test-session-policy`;
     - `test-effect-interchange-policy`;
     - both native-pcm-runner policy tests;
     - realtime-audit-leak check and test;
     - `check-unfused-seal`, `check-wasm-realtime-atomics`, `check-effect-contract`,
       `check-capi-abi`, `check-conformance-boundaries` and `check-artifact-evidence-leak`;
     - `check-test-support-ci.py`, `test-test-support-ci.py`, `check-release-shape.py` and
       `check-sdk-deletions.py`;
     - `test-gate-lib.sh`;
     - `check-sdk-generated.sh`, which regenerates the parameter metadata, ABI layout JSON and SDK
       modules and compares them.
   - **These fail identically on the base:** `check-session-policy.sh`, `check-env-vocabulary.sh`,
     `test-env-vocabulary.sh` and `check-step-vocabulary.py`. Each flags text under
     `docs/handoffs/test-value-2026-09-28/`, merged docs-only in `73e50f7d`, so they are not this
     change's.
