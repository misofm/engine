# Remove the native PCM runner (keep the C ABI)

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

**Owner ruling (2026-09-28):** Keep the C ABI, which is the mobile interface. Remove the native PCM runner: desktop and cloud are not within 6 months, and removals are approved where nothing actively uses the code.

**Blocked on an owner ruling.** Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`,
section 7, R2. The ruling to record: "No native or cloud embedding is planned. The C ABI
(`miso_engine_v1_*`) and the native PCM reference runner are removed. Reopening them needs a new
issue that re-earns them." If the ruling keeps native embedding, close this draft and still do
`02-…`.

## Context

- **`crates/capi`: 9,802 Rust lines.** That is 3,645 production, 3,346 in-source tests and 2,820
  in `tests/resource_lifecycle.rs`. It also has `include/miso_engine_v1.h` (255 lines) and C/C++
  smoke tests in `tests/c/` (145 lines).
  - Its dependencies are `host-core[control-provider]`, `protocol`, `source`, `session`, `engine`
    and `lane`.
  - It is linked by `tools/audit` (the `capi` subject, `src/capi.rs`, 342 lines) and
    `tools/native-pcm-runner`.
- **`tools/native-pcm-runner`: 2,689 lines.** It decodes WAV/RF64 and renders through the C ABI
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
3. **Shipped artifact.** Build base and change on one machine, as audit section 11 describes.
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

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, finding F3. **The owner corrected the product scope after the audit:
fans open and play sessions in native iOS and Android apps.** The recommendation "remove unless a
native or cloud embedder is on the roadmap" now resolves to **keep the C ABI**. The ruling text
above ("No native or cloud embedding is planned") contradicts the owner and must not be recorded.

1. **`crates/capi` is the mobile playback surface, and it already covers playback.**
   - It builds as `rlib`, `staticlib` and `cdylib` (`crates/capi/Cargo.toml:11`): a static library
     for an iOS framework, a shared library behind a JNI shim for Android.
   - `include/miso_engine_v1.h:213-249` exposes session compile from JSON, planar PCM submission
     and seek, render at an absolute sample, command submission and event readout (meters and
     diagnostics).
   - It compiles for `aarch64-apple-ios` and `aarch64-linux-android` (`cargo check --all-targets
     --all-features`, Rust 1.98.1; not linked, not run: there is no NDK, Xcode or device here, and
     no CI leg).
2. **Its dependencies are live with it:** `host-core`'s `control-provider` and
   `control_provider.rs`, the engine's block-boundary plan replacement (`reserve_replacement`,
   `commit`, `render_contiguous`; used from `capi/src/runtime/control.rs` and
   `capi/src/runtime/plan.rs:201`), `lane::fpenv` (which has an AArch64 branch), and `protocol`
   (R3). The audit is right that the browser uses none of these and that only capi exercises plan
   replacement.
3. **Keep `audit capi`.** Its 100,000-call render audit (zero allocations, locks, syscalls) is the
   only realtime audit of the entry point a mobile app will call. It runs on x86 only; an aarch64
   equivalent is part of R1's recommended CI leg.
4. **Native benchmarking does not depend on capi.** `tools/console-workload` and `tools/bench` do
   not link it; `tools/audit` links it only for its `capi` subject.
5. **What is still a separate, optional call:** `tools/native-pcm-runner` (a desktop WAV-file
   reference runner through the C ABI) is not a mobile deliverable, and its CI step is a static
   seal. It can go on its own ruling, together with R4 Part B; `fixtures/capi-qualification/v1`
   (draft 06) is dead either way.
6. **Known mobile gaps this draft should hand to new issues rather than hide:**
   - live control: per `#140`'s spec (line 13), admitted protocol automation has no production
     consumer except cancellation, so a fan's live parameter change through the C ABI reaches PCM
     only by a structural plan replacement, which resets source rings at the boundary
     (`capi/src/runtime/control.rs:46-53`). See the amendment to `02-…`;
   - no aarch64 build, lint or test in CI (R1 amendment).
7. **Revised recommendation:** retire this draft as written. If the owner wants the runner gone,
   file a narrower "remove native-pcm-runner" draft (runner, its fixtures, its lint step and the
   `docs/NATIVE_PCM_REFERENCE_RUNNER_V1.md` doc), gated by the capi tests and `audit capi` staying
   green.

## Amendment (root, 2026-09-28): absorb #1035's Part B step 4 and its doc follow-ups

#1035 (merged into the batch) removed the native decode workers but had to keep the pieces the
native PCM runner still imports. Once this issue removes the runner, remove them here too (owner
ruling R4 approved them):

- `crates/source/src/native_wave.rs` and the `NativeWave*` re-exports;
- `audit fixture-source` and `fixtures/sources/v1`, unless a surviving test or tool still reads
  them (prove non-use by compile and grep first).

Also close #1035's verifier's Low findings (see `1035-*.md`, "Sol verdict, attempt 1"):

- `docs/STEM_IDENTITY_V1.md`: name the release CLI as the serializer and WAVE-stripping owner;
  the browser stem store only parses the `blake3:` spelling and hashes the canonical bytes it is
  given. Give the stem-identity corpus a CI consumer (`generate.py --check` in the lint job) or
  say why it has none.
- Stale references: `docs/derivations/241-browser-source-identities.md:186-195` (`stem-hasher`
  commands), `docs/DELIVERY_CODEC_BOUNDARY.md:12` (native WAVE/RF64 control-worker path),
  `crates/source/src/lib.rs:1407` ("native worker/decoder bytes"), and the `qualification.yml`
  test-debug-a comment that still lists `source`.

## Attempt 1 evidence

Terra, 2026-09-28. Base `c69736c1` (`codex/batch-slim-2`); change `5e15bc96` and `fbda196d` on
`codex/1033-remove-native-pcm-runner`. 49 files, **+100 / -9,505 lines** as git counts them (it
pairs the moved seek model with its old file; counted as a delete plus an add, +669 / -10,074),
plus five binary WAVs. No timed benchmark was run.

The owner ruling and the Amendments govern: the C ABI, `audit capi`, `check-capi-abi.sh`,
`control-provider`, plan replacement and `lane::fpenv` all stay. Removed: the native PCM runner
(Amendment 7's narrower draft), #1035's Part B step 4 and #1035's Low findings 3 and 4 (root's
amendment).

### Deleted, and what each covered

| item | size | what it covered |
|---|---:|---|
| `tools/native-pcm-runner` (`lib.rs`, `main.rs`, `tests/process_boundary.rs`, `Cargo.toml`, `MUTATIONS.md`) | 2,724 lines | the desktop reference tool: resolve WAV/RF64 files for a session, decode them with `native_wave`, compile, submit and render through the C ABI, publish planar `f32` files no-clobber; 20 tests |
| `fixtures/native-pcm-runner/v1` (5 WAV, 5 session JSON, `MANIFEST.tsv`, `generate.py`) | 12 files, 127,545 bytes | its corpus: the four RIFF launch rates and one RF64, with output digests |
| `scripts/check-native-pcm-runner.sh`, `test-native-pcm-runner-v1-policy.sh`, `test-native-pcm-runner-portability-v1-policy.sh`; the lint step "Native PCM runner static seal and mutation tests" | 495 lines | the runner's static seal (fixture identity, no graph bypass, no reverse dependency, portability contract) and its mutations |
| `docs/NATIVE_PCM_REFERENCE_RUNNER_V1.md` | 107 lines | the runner's contract |
| `crates/source/src/native_wave.rs` and the nine-line `NativeWave*`/`parse_native_wave` re-export block | 1,415 lines | the native RIFF/WAVE and RF64 parser and decoder. It was compiled into every native `source` build, mobile included, and called only by the runner and `audit fixture-source`; 8 tests |
| `audit fixture-source`: its dispatcher entries and the decoder half of `tools/audit/src/source_fixture.rs` (generated WAV fixtures, independent decode oracle, frozen diagnostic matrix); `fixtures/sources/v1` (README, manifest of 10 checksums) | 626 + 21 lines (the other 558 moved, below) | the native decoder's corpus; 2 tests |
| `blake3` workspace dependency; lockfile entries `blake3`, `arrayref`, `arrayvec`, `constant_time_eq`, `native-pcm-runner` | 47 lines (lockfile 46, manifest 1) | the runner was `blake3`'s only user. An AArch64 `--workspace --lib` check now needs only `wasm-console` and `wasm-gates` excluded |
| `audit`'s `source` dependency | 1 line | only `source_fixture.rs` used it |
| `MISO_ENGINE_REPIN_NATIVE_PCM_RUNNER` vocabulary row | 1 line | the runner's re-pin hook |
| allow-list rows naming the runner: `check-realtime-policy.sh` and `check-bench-policy.sh` unsafe owners, `check-session-policy.sh`'s `fixtures/native-pcm-runner` scan root | 3 rows | the runner's `unsafe` C calls and its TOML scan |

Proof of non-use before deletion: `git grep` for `native_wave`, `NativeWave`, `parse_native_wave`,
`source_fixture`, `fixtures/sources` and `native-pcm-runner` found users only in the runner,
`audit`'s `source_fixture.rs` and the policies listed above; after deletion
`cargo check --workspace --all-targets --all-features` passes with no warning.

### Kept although the runner called it

- Every `capi` item the runner used (8 exported calls, 20 ABI types and constants): the frozen C
  ABI, used by capi's own tests, `audit capi`, `check-capi-abi.sh`'s C11/C++17 consumer and mobile
  apps.
- `session::{parse_session_json, SessionModel, Source, SourceBitDepth}` and `source::SourceFrame`:
  used by host-core and capi.
- `fixtures/stem-identity/v1`, whose `pcm16-stereo-boundaries.wav` the runner's tests read: it is
  the contract corpus the release CLI copies byte for byte, now checked by the lint job.
- **The seek-schedule half of `source_fixture.rs`**, which never read `fixtures/sources/v1` and is
  a live browser and C ABI claim: 256 frozen schedules (queues of 1, 2, 3 and 8 quanta) drive the
  production `PcmSourceRing` through `HostChunkProvider`, the host path both adapters feed, and
  every submit result, sample, read-report field and the stale-discard counter must match an
  independent model; the schedule transcript is pinned. It moved unchanged (model, generator,
  pinned transcript `ec3b7fef…`, production exercise) to
  `crates/source/tests/seek_schedule_model.rs` as
  `frozen_seek_schedules_match_the_independent_ring_model`, with `sha2` as a `source`
  dev-dependency in place of `bench_support::digest`. Mutation: deleting the ring's
  stale-discard increment (`lib.rs:1311`) turns it red at schedule 0 step 3; reverted. It now runs
  in test-debug-a (debug) instead of audit-native (release), in 0.01 s.

### Policies, CI and docs

- CI: the runner lint step is deleted; the lint job gains "Stem identity corpus drift check"
  (`generate.py --check`), the corpus's only consumer here; test-debug-a's comment no longer lists
  `source`. No job is added or removed, so the `verdict` table is unchanged.
- `test-realtime-policy.sh`: `unsafe-outside-native-pcm-runner-lib` becomes
  `unsafe-in-deleted-native-pcm-runner-lib` (unsafe code at the old path is now rejected).
  `check-bench-policy.sh`: five unsafe owners, and `test-bench-policy.sh`'s count diagnostics pin 5.
  `test-session-policy.sh`: the scan root leaves its four loops.
- `docs/ENGINE_ENV_VOCABULARY.md`: the runner row goes, and `MISO_ENGINE_PRINT_HELPER_MANIFEST`'s
  row now names its real user, `check-effect-runtime-policy.sh` (it said "native PCM runner
  portability gate"). `test-env-vocabulary.sh`'s count: 68 -> 67.
- AGENTS.md: "Build for native/cloud embedding, iOS, Android, and browser WebAssembly" becomes the
  browser plus iOS and Android apps through the C ABI, with desktop and cloud not live scope
  (ruling R2); "PCM runner" leaves the deliverables and joins "Do not deliver". "a narrow C ABI"
  stays.
- #1035 Low finding 3: `docs/STEM_IDENTITY_V1.md` names the release CLI (`misofm/cli`,
  `src/stem-identity.ts`) as the only serializer and WAVE (RIFF and RF64) stripper, and the browser
  stem store as the owner of the `blake3:` grammar and of hashing the canonical bytes a resolver
  hands it (no container decoder). "The reference WAVE path" becomes "every WAVE path (today only
  the release CLI's)". Verified against a local `misofm/cli` checkout (`3c89436`): all ten corpus
  copies in `tests/fixtures/stem-identity` are byte-identical to this repository's.
- #1035 Low finding 4: `docs/derivations/241-browser-source-identities.md` records the
  `stem-hasher` agreement as history; `docs/DELIVERY_CODEC_BOUNDARY.md` says there is no
  in-repository file reader; `SourceGraphSource`'s two field docs (`source/src/lib.rs`, was
  `:1407`) no longer say "native worker/decoder"; the test-debug-a comment is fixed. Also fixed:
  `generate.py`'s comment naming "the streaming Rust implementation".
- Historical mentions: `docs/C_ABI_V1_QUALIFICATION.md` and `docs/IMPLEMENTATION_PLAN.md` get a
  one-line removal note; `BUILTINS_AND_METERING_V1.md` ruling D1 and a `parametric-eq` test comment
  no longer lean on the runner. The rulings inventories and `241-schema-repins.md` are left as
  history.

### Removed tests and the claim each held (`cargo test --locked --workspace --all-targets --all-features -- --list`, binary-qualified)

Base 2,260 -> change 2,231: **30 removed, 1 added**, nothing else moved.

- **`native_pcm_runner` (19) and `process_boundary` (1)**, the runner's own claims:
  `cli_is_closed_and_exact`, `identity_grammar_is_closed`,
  `encoder_preserves_signed_zero_and_publication_is_no_clobber`,
  `final_collisions_inserted_immediately_before_publication_are_preserved`,
  `injected_create_write_and_publish_failures_are_terminal`,
  `missing_mismatched_and_truncated_riff_sources_are_exact_precompile_failures`,
  `portable_publication_state_machine_freezes_every_race_and_failure`,
  `post_create_partial_replacements_are_preserved_and_never_published`,
  `preflight_and_resolution_fail_before_compile_or_output`,
  `publication_refuses_every_preexisting_final_and_partial_kind`,
  `real_c_abi_riff_and_rf64_render_exact_block_planar_outputs`,
  `real_output_faults_remove_only_the_owned_partial`,
  `resolver_rejects_identity_shape_file_shape_and_declaration_mismatches_precompile`,
  `resolver_rejects_integer_and_float_depth_mismatches_both_directions_precompile`,
  `reversed_source_declarations_submit_in_canonical_id_order`,
  `scalar_caps_precede_resolution_and_cover_overflow_and_unsupported_rate`,
  `shared_runner_orders_short_final_submission_and_terminal_failures`,
  `source_symlink_is_rejected_and_sentinel_preserved`,
  `unsupported_platform_stops_before_output_source_engine_or_publication`, and
  `executable_boundary_accepts_fixture_and_rejects_zero_frames`. CLI, file resolution, WAV
  decoding and file publication are desktop-tool claims with no live user. The C ABI half of the
  render test (compile, host-fed submission, render at the launch rates) stays guarded by capi's
  `direct_and_c_render_match_one_and_ten_tracks_across_launch_rates`,
  `compile_publishes_both_children_and_source_control_is_region_checked`,
  `source_rejections_reach_the_c_host_as_their_own_diagnostic`, `resource_lifecycle.rs`, and
  `check-capi-abi.sh`'s C11/C++17 consumer (generation 1 submit, generation 2 seek and submit, two
  renders); a short end-of-region submission by the seek model and host-core's `prepare.rs`
  source tests.
- **`source` `native_wave::tests` (8)**: `buffered_decodes_share_one_fill_and_a_straddled_refill_keeps_the_reader_position`,
  `classic_formats_decode_with_exact_pcm_scaling_and_float_sanitation`,
  `every_encoding_decodes_the_same_bits_however_the_region_is_partitioned`,
  `extensible_and_rf64_metadata_are_accepted_with_checked_sizes`,
  `failed_io_forgets_reader_position_and_retry_reseeks`,
  `invalid_native_containers_and_regions_have_frozen_diagnostics`,
  `malformed_ds64_byte_rate_duplicate_data_and_metadata_cap_reject_without_payload_retention`,
  `mask_sanitizers_freeze_float_boundary_bits_and_counts`: the deleted parser and decoder.
- **`audit` (2)**: `source_fixture::tests::generated_fixtures_match_manifest_oracles_and_mutation_policy`
  (its decoder half is deleted with the decoder; its ring half is the added test above) and
  `source_fixture::tests::shared_sha256_alias_matches_published_literals` (the shared
  `sha256_hex` known answers, still held by `bench_support` `digest::tests::matches_the_published_vectors`
  and four identical `audit` alias tests).
- **Added (1)**: `seek_schedule_model::frozen_seek_schedules_match_the_independent_ring_model`.

**Gate 5, the live claims.** No allocation, lock or syscall at the native public render entry:
`audit capi` (100,000 calls, all counters 0, through the Issue-544 validator). Browser render
closure: `check-web-audioworklet.sh`'s call-graph gates on this change's module. Realtime traces:
`trace-graph-audit.sh` and `trace-builtins-audit.sh` (1,000,000 blocks). Resource lifecycle:
`resource_lifecycle.rs` and `boot_transient_budget.rs` (test-debug-a). Ring seeks, admission and
underrun: the moved model plus the shared-ring tests #1035's verdict lists.

### Gates

| gate | result |
|---|---|
| `cargo check --locked --workspace --all-targets --all-features` | pass, 0 warnings |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and without `--all-features` | pass |
| `cargo fmt --all --check` | pass |
| `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web -p host-core -p source` | pass |
| `CARGO_PROFILE_RELEASE_PANIC=unwind cargo check --locked --release --workspace --all-targets` | pass |
| `cargo check --locked --all-targets --all-features --target <aarch64-apple-ios, aarch64-linux-android> -p source -p capi -p host-core` | pass, 0 warnings |
| `cargo check --locked --workspace --lib --all-features --target <aarch64-apple-ios, aarch64-linux-android> --exclude wasm-console --exclude wasm-gates` | pass (no `native-pcm-runner` exclusion needed now) |
| `scripts/check-cross-targets.sh` (product crates checked and clippy-linted on both AArch64 targets, #1018 memset ceilings, armv7 refusal, wasm rows) | PASS; 11 expected memset rows unchanged, `source` has no calls before or after |
| test-debug-a's exact command | 1,263 passed, 0 failed, 7 ignored |
| `cargo test --locked --release -p source -p capi -p host-core --features host-core/test-support` | 255 passed, 0 failed, 2 ignored |
| `cargo test --locked --release -p audit -p bench -p console-workload` (audit-native) | 128 passed, 0 failed, 2 ignored |
| `cargo test --locked -p audit` (dev) and `-p parametric-eq --test analytic` (dev and release) | 40 passed; 7 and 7 passed |
| console digests (`gain_pan_profile digests`) on base and change | 17 rows, byte-identical |
| `bash scripts/run-wasm-gates.sh` | pass (native, wasm scalar, wasm simd128, V8 spill) |
| `audit capi` through the workflow's inline Issue-544 validator | pass, `total_violations` 0 |
| `check-capi-abi.sh` and `--self-test` | pass (shared and static linkage; mutations ok) |
| `check-ci-path-routing.py`, `test-ci-path-routing.py`, `check-script-reachability.py`, `test-script-reachability.py` | pass (133 reached, 8 operator) |
| `check-env-vocabulary.sh`, `test-env-vocabulary.sh` | pass, 67 names |
| every `scripts/check-*`/`test-*` the workflow runs, with its CI arguments where it takes one (85 invocations: every lint policy pair, `check-release-shape.py` and `--self-test`, `check-test-support-ci.py`, `check-workspace-policy.sh`, `check-host-core-policy.sh`, `check-conformance-boundaries.sh`, `check-artifact-evidence-leak.sh`, `check-session-policy.sh`, `check-realtime-policy.sh`, the audit-native trace, fixture, console and effect-contract steps, `check-protocol-wasm-parity.sh`, `check-sdk-generated.sh`, `check-stem-store-v1.mjs`, `generate.py --check`) | all pass after `fbda196d` (the first run caught `test-bench-policy.sh`'s count pin) |
| `check-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts`, `check-web-audioworklet-v8-spill.py`, `check-sdk-headless.sh` over an artifact directory assembled from this change's module (the build script's copy steps, no pin) | pass (SDK 285/285) |
| not run | `check-sdk-types.sh` (needs `npm ci`), `run-aarch64-tests.sh` (needs the arm64 runner), `check-graph-determinism.sh` (untouched crates) |

### Shipped artifact

`bash scripts/build-web-audioworklet.sh --module-only` on base and change, same machine and
toolchain: base `6c952a2c…`, change `01dd58be…`, **both 3,485,631 bytes: size change 0**. 8 bytes
differ, all data: seven `core::panic::Location` line fields for `crates/source/src/lib.rs`, each
moved by -9 (the deleted `native_wave` block above them): 835 -> 826 twice, 834 -> 825,
518 -> 509, 1165 -> 1156 twice, 1164 -> 1155; columns unchanged. `wasm-objdump -d` of the two
modules is identical. The pin is not moved here; the batch's pin (`f7bd75ca…`) is already stale on
base, and the boundary re-pin must carry this reason.

### Follow-ups (not done here)

1. `SourceDiagnosticCode` (shipped in the module) now has 8 of 11 variants that nothing
   constructs: `ChannelsMismatch`, `RegionOutOfBounds`, `ContainerInvalid` and `FormatUnsupported`
   lost their only constructor with `native_wave`; `AssetUnresolved`, `ContentIdentityMismatch`,
   `GenerationNonMonotonic` and `GraphBindingMismatch` had none on base. Pruning the registry moves
   the artifact, so it is its own issue.
2. `fixtures/capi-qualification/v1` still names the runner's files; #1029 deletes it.
3. Open #895 (native runner I/O) closes as descoped.
