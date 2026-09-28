# Remove the native WAV/RF64 decode workers

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

**Owner ruling (2026-09-28):** Approved, including Part B (desktop and cloud are not within 6 months). The capi source-ring re-pin in the amendment is mandatory.

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
   `source/src/lib.rs` shifts panic line numbers. Build base and change on one machine (audit section 11), prove with
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

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, findings F3 and F5. With mobile playback in scope and the C ABI kept
(R2 amendment), the recommendation still stands for mobile, with one correction.

1. **Mobile does not use the native decoder.** capi takes host-decoded planar chunks
   (`miso_engine_v1_source_submit_planar_f32`) and never names `native_source` or `native_wave`
   in `src/`. A mobile app decodes with the platform (AVAudioFile/ExtAudioFile on iOS,
   MediaExtractor/MediaCodec on Android) and submits planar `f32`, which is what AGENTS.md
   already says for "browser/mobile hosts". The stem-identity contract hashes decoded PCM, so no
   WAV parser is needed on the device either.
2. **Correction: the native-only hooks are compiled into mobile builds.** They are
   `cfg(not(target_arch = "wasm32"))`, not "native desktop only", so every AArch64 build of capi
   carries `SourceGraphSource::retirement_worker` (`crates/source/src/lib.rs:1636`) and
   `SourceGraphSourceSetDriver::_retirement_workers` (`:1707`), unused. Removing them changes the
   capi source-ring layout on mobile as on x86. So Part A step 3 (re-pin
   `crates/capi/tests/resource_lifecycle.rs`'s `NativeSourceWorkerMirror` and
   `SourceGraphSourceSetDriverMirror`, `:803-826`, and the resource numbers they feed) is
   **mandatory**, not conditional on R2.
3. **Part B no longer depends on R2 landing.** R2 keeps capi, so Part B depends only on the
   separate native-pcm-runner decision (R2 amendment, item 5): the WAV parser's only non-audit
   users are native-pcm-runner and stem-hasher.
4. **Owner question the draft should state:** the ruling text removes the decoder for every native
   host. If a native desktop or cloud renderer (for example a server-side export) is still wanted,
   it would need a decoder; the owner's correction names web and mobile only.
5. **Gates:** add `cargo test -p capi` (the layout mirror) and an aarch64 `cargo check -p source
   -p capi` on base and change.

## Attempt 1 evidence

Terra, 2026-09-28. Base `c867ec2a` (the `codex/batch-slim-1` branch point); change `fecafc00` on
`codex/1035-remove-native-decode-workers`. 23 files, **+140 / -7,939 lines**. No timed benchmark
was run.

### What was delivered

**Part A, in full.**

1. `crates/source/src/native_source.rs` is deleted (5,007 lines: the decode worker threads, the
   resolver, `prepare_native_source`, `prepare_native_session_sources`,
   `NativeSessionPreparedSources`, `NativeSourceController`, the audit gate and layout helpers).
   In `lib.rs` the native-only hooks go: the module declaration, the `NativeSourceWorker` import,
   both re-export blocks, the producer's `reserve_block`/`commit_block`/`commit_deferred`,
   `ReservedBlock`, the provider's three `pub(crate)` wrappers,
   `SourceGraphSource::retirement_worker` and `with_native_worker`,
   `SourceGraphSourceSetDriver::_retirement_workers`, and the retirement-worker class of
   `SourceSetRetainedResourceReport` and its plumbing in `prepare_graph_source_set`. `lib.rs`
   production code is 1,940 -> 1,728 lines, its tests 2,162 -> 1,656.
   - `source/test-support` gated nothing but the worker hold/release, so the feature is deleted:
     `crates/source/Cargo.toml`, audit's dependency line, the test-debug-a `--features` list, and
     `scripts/test-test-support-ci.py` (its fixture string, the `source` mutation cases, and the
     declaring-package floor 10 -> 9, which is now the exact count). Gate 5's
     "with `source/test-support`" therefore runs without that feature.
   - The source-scrape allow-list in `scripts/check-workspace-policy.sh` follows the deleted
     scrapes exactly: `crates/source/src/lib.rs` 4 -> 2, and the `native_source.rs` and
     `native_wave.rs` rows are deleted.
   - `native_wave.rs` stays (Part B step 4, below), but the seek and strided-sink API only the
     worker called became dead code and is deleted: `next_source_frame`,
     `seek_to_source_frame`, `decode_planar`, `Strided`, `validate_seek_frame` (production
     925 -> 837 lines). `prepare`, `decode_into`, `parse_native_wave`, `metadata` and `region`
     stay for native-pcm-runner and `audit fixture-source`.
2. `audit source` and `audit source-duration` (`tools/audit/src/source.rs`, `source_duration.rs`,
   853 lines) and `scripts/trace-source-audit.sh` are deleted. CI: the "Issue-544 source-duration
   runtime caller audit" step and the source half of the inline Issue-544 validator are deleted;
   the capi half stays, because the C ABI stays (R2 amendment). "Source and graph realtime audits"
   keeps its graph line as "Graph realtime audit". No job is added or removed, so the `verdict`
   table is untouched. The `MISO_ENGINE_SOURCE_RT_BEGIN`/`_END` markers leave
   `docs/ENGINE_ENV_VOCABULARY.md` (two rows; the only edit to that shared file).
3. capi's layout mirror is re-pinned (`crates/capi/tests/resource_lifecycle.rs`):
   `NativeSourceWorkerMirror` and the driver mirror's `retirement_workers` slice are deleted. The
   driver shrinks by the 16-byte `Box<[NativeSourceWorker]>`, so per plan source overhead
   3,950 -> 3,934 and source total 12,142 -> 12,126 (the frozen scratch report and the primitive
   oracle), and the double-live rows 24,284 -> 24,252 and 7,900 -> 7,868. Nothing else moved.

**Part B, steps 5 and 6.** `tools/stem-hasher` is deleted (crate, 3 unit tests, 9 conformance
tests; workspace member, workspace dependency and `Cargo.lock` entry). Nothing outside it used
it: no script, workflow or sibling repository names it, and the release CLI (`misofm/cli`,
`src/stem-identity.ts`) is an independent implementation pinned to a copy of
`fixtures/stem-identity/v1/VECTORS.tsv` (the copy is byte-identical today). Docs: AGENTS.md's
sentence now reads "hosts decode and submit planar chunks into bounded SPSC PCM rings ... the
engine runs no decode worker threads"; `docs/STEM_IDENTITY_V1.md`'s "Reference oracle" section is
replaced by "Implementations and the corpus gate" (the browser stem store and the release CLI;
`generate.py --check`); the stem-identity README follows.

**Part B step 4 is not done: it is blocked by #1033.** `tools/native-pcm-runner` imports seven
`native_wave` items (`lib.rs:31-32`) and its tests read
`fixtures/stem-identity/v1/pcm16-stereo-boundaries.wav`. The owner ruled the runner's removal (R2),
but #1033 has not landed, and deleting the parser here would mean deleting or rewriting the
runner, which is #1033's scope. When #1033 lands, `native_wave.rs`, `audit fixture-source`,
`fixtures/sources/v1` and the three stem-identity `.wav` wrappers (with their `generate.py` rows)
have no user left; #1033 can absorb them, or a small Part B2 follows it. Until then the parser is
compiled into native builds of `source`, mobile included, where nothing calls it.

### Removed tests and the claim each held (62; `cargo test --workspace --all-targets --all-features -- --list`, base 2,456 -> change 2,398)

Every removed test's claim is about the native producer, its decoder API, or the deleted tools.

- **`source::native_source::tests` (37)**, the native decode worker and native preparation:
  `native_worker_idle_paths_do_not_use_active_spin_primitives`,
  `render_wait_is_half_the_prepared_ring_and_bounded`,
  `prepared_source_job_is_inert_until_the_single_start_boundary`,
  `resolver_preparation_validates_identity_rate_channels_region_and_fixed_caps`,
  `native_worker_and_host_provider_produce_identical_prepared_ring_pcm`,
  `controller_snapshot_and_terminal_watermarks_are_exact_and_monotonic`,
  `reserved_event_slot_preserves_terminal_after_ready_and_snapshot`,
  `multiblock_native_watermark_does_not_readd_the_cumulative_decoder_report`,
  `native_queue_layout_and_per_source_caps_use_exact_requests`,
  `controller_first_drop_does_not_detach_the_retirement_stop_owner`,
  `worker_seek_stop_wake_and_join_are_bounded_and_typed`,
  `single_worker_seek_resumes_contiguously_at_the_exact_frame`,
  `worker_coalesces_provider_backpressure_to_latest_exact_frame_without_intermediate_pcm`,
  `seek_continuation_pcm_reads_are_preceded_by_audit_acknowledgements`,
  `provider_seek_admission_precedes_decoder_reposition`,
  `pending_seek_stops_without_render_drain_and_controller_backpressure_does_not_advance`,
  `decoder_failure_after_accepted_seek_keeps_typed_terminal`,
  `terminal_event_carries_decode_failure_and_other_sources_continue`,
  `compiled_session_sources_prepare_once_and_publish_one_graph_source_set`,
  `compiled_multi_source_session_uses_one_exact_shared_worker`,
  `idle_decode_thread_cpu_is_bounded_and_one_thread_serves_a_set`,
  `native_sanitation_reaches_after_disarm_source_set_telemetry_and_drop_retires_worker`,
  `graph_set_driver_retains_all_native_workers_until_set_drop`,
  `invalid_graph_mapping_stops_carried_worker_before_consumer_cleanup`,
  `retired_graph_plan_is_the_native_worker_join_owner`,
  `compiled_session_source_mismatch_and_cap_fail_without_publication`,
  `combined_retained_cap_accepts_exactly_and_rejects_one_byte_short`,
  `launch_rates_prepare_and_extended_rate_mismatch_is_rejected`,
  `engine_boundary_refuses_declared_versus_decoded_depth_in_both_directions`,
  `retained_layout_grid_is_checked_without_source_duration_storage`,
  `compiled_sourceless_session_returns_collection_diagnostic_without_resolving`,
  `compiled_session_source_failure_collects_sorted_diagnostics_before_worker_start`,
  `native_worker_publishes_the_recorded_block_sequence_through_stall_and_seeks`,
  `native_worker_matches_the_pre_change_worker_block_for_block`,
  `a_stalled_seek_no_longer_counts_the_quantum_only_the_old_worker_decoded_ahead`,
  `worker_retries_a_full_commit_through_the_deferred_block_without_loss_or_duplication`, and the
  `#[ignore]`d `print_published_sequence_constants_from_the_pre_change_worker`. The host path's
  analogues of the session-level ones (caps admit exactly and reject one byte below, typed source
  errors, the launch rate set, sources fed independently) are host-core's `prepare.rs` tests.
- **`source` `lib.rs` tests (8)**, the native reserve/commit producer:
  `set_driver_declares_worker_tokens_before_source_consumers` (worker tokens drop before
  consumers; a source scrape), `native_commit_publishes_the_decoded_block_without_a_copy`
  (commit order; a source scrape), `host_and_native_submission_share_exact_short_eof_metadata_and_validation_order`
  (native/host parity), `native_commit_rejects_a_stale_generation_and_keeps_the_block_unpublished`
  (the rejected block stays with the worker), `native_full_commit_defers_the_block_and_its_retry_acks_it_exactly_once`
  and `native_deferred_retry_never_acks_a_block_a_seek_made_stale` (the native `commit_deferred`
  retry), `stamped_native_watermark_survives_seek_stale_discard_and_saturates` (the native
  sanitation watermark), and `prepared_contiguous_native_submission_matches_planar_ring_shape`
  (ported, below).
- **`source` `native_wave` tests (3)**: `buffered_quanta_seek_and_straddled_refill_preserve_exact_pcm`
  and `prepared_quantum_decode_uses_contiguous_planar_worker_storage` (the deleted seek and
  worker-block API), `plane_and_strided_sinks_match_every_encoding_across_chunks_refills_and_seek`
  (the two sinks agree; plus a source scrape). `failed_io_forgets_reader_position_and_retry_reseeks`
  keeps its name and is ported onto `decode_into`.
- **`stem-hasher` (12)**: `cli_is_closed_and_raw_shape_is_exact`,
  `raw_length_is_total_and_output_is_canonical`,
  `wave_depth_set_rejects_integer_pcm32_but_accepts_float32`, and the 9 `tests/conformance.rs`
  vector, CLI and WAVE-wrapper tests: the Rust oracle's own claims.
- **`audit` (2)**: `source::tests::asserted_worker_lifecycle_counter_and_pcm_transcript_is_canonical`,
  `source_duration::tests::exact_duration_independent_accounting_serialization_is_canonical`: the
  deleted subjects' record formats.

**Ported or new (4).** `one_quantum_ring_shape_matches_its_report_and_reads_back_planar` (the
shared shape and report claims of the native shape test, fed by `HostChunkProvider`);
`rejected_host_submission_publishes_nothing_and_leaves_the_producer_untouched` (the shared
validation claims of the two native commit-rejection tests: a stale-generation and a short
non-EOF submission are refused before any publication, telemetry does not move, and the render
discards nothing); in `native_wave`,
`buffered_decodes_share_one_fill_and_a_straddled_refill_keeps_the_reader_position` and
`every_encoding_decodes_the_same_bits_however_the_region_is_partitioned` (the buffering and
partition-invariance halves of the two deleted decoder tests, on `decode_into`).

**The live claims keep their guards.**

- Bounded ring: `report_separates_session_pcm_from_source_overhead`,
  `host_submission_is_fifo_wraparound_and_never_accepts_a_prefix`,
  `played_planes_stay_intact_while_the_producer_fills_its_configured_depth`, the new shape test.
- Generation-tagged seeks: `seek_switches_at_boundary_and_discards_older_queued_audio`,
  `paused_seek_prepares_full_queues_without_consuming_target`,
  `host_region_preparation_preserves_resources_and_absolute_ownership`,
  `graph_driver_forwards_underrun_and_seek_generation_facts`, the new rejection test.
- Underrun emits zero plus a counter: `underrun_is_positive_zero_and_eof_is_not_an_underrun`,
  `graph_driver_forwards_underrun_and_seek_generation_facts`.
- Host-core: `source_diagnostics.rs`, `source_in_place.rs` and the `prepare.rs` source tests.
- Realtime render over a ring: `audit capi` (100,000 C-ABI render calls over a host-fed source
  ring whose one quantum is followed by underruns; 0 allocations, locks and syscalls; unchanged in
  CI), the wasm render-closure checks of `check-web-audioworklet.sh`, and
  `check-realtime-policy.sh` over `source/src/lib.rs`'s realtime regions. `audit source`'s own
  subject was a native worker feeding the ring.
- Memory independent of stem duration: the host ring takes no duration input
  (`PcmSourceRingConfig` is channels, quantum, frame capacity and generation), and
  `host_region_preparation_preserves_resources_and_absolute_ownership` pins the report against the
  region origin. `audit source-duration` proved it for the file-backed native worker only.

### Gates

| gate | result |
|---|---|
| `cargo check --locked --workspace --all-targets --all-features` | pass, no warnings |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and without `--all-features` | pass |
| `cargo fmt --check` | pass |
| `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web -p source -p host-core` | pass |
| `cargo check --locked --all-targets --target aarch64-apple-ios -p source -p capi`, and `aarch64-linux-android` | pass on base and change |
| `cargo check --locked --workspace --lib --all-features --target <aarch64-apple-ios, aarch64-linux-android> --exclude native-pcm-runner --exclude wasm-console --exclude wasm-gates` | pass (the three excluded tools need a cross C toolchain for `blake3`/`wasmtime` build scripts, not installed here) |
| test-debug-a's exact command (new feature list) | 1,389 passed, 0 failed, 9 ignored |
| `cargo test --locked --release -p audit -p bench -p console-workload` | 154 passed, 0 failed |
| `cargo test --locked --release -p source -p capi -p host-core --features host-core/test-support` (gate 5 and amendment 5 in release; debug runs inside test-debug-a) | 260 passed, 0 failed |
| console digests, `gain_pan_profile digests` on base and change | 17 rows, byte-identical |
| `-- --list` diff | 62 removed (listed above), 4 added |
| every `scripts/check-*.sh` and `check-*.py` that takes no arguments (51; seven more require a path or mode), plus `test-ci-path-routing.py`, `test-test-support-ci.py`, `test-realtime-policy.sh` and `test-bench-policy.sh` | 48 checks and the 4 tests pass; the other three checks are the next two rows. Among the passes: `check-ci-path-routing.py`, `check-realtime-policy.sh`, `check-session-policy.sh` (which scans `fixtures/native-pcm-runner`), `check-env-vocabulary.sh`, `check-workspace-policy.sh` (the exact source-scrape allow-list), `check-test-support-ci.py`, `check-release-shape.py`, `check-native-pcm-runner.sh`, `check-capi-abi.sh`, `check-browser-expected-resources.py`, `check-cross-targets.sh` |
| `check-web-audioworklet.sh`, `check-sdk-headless.sh` | fail on the AudioWorklet pin only, as base does (base builds `f7bd75ca…`, the pin is `476e58ad…`; the batch boundary re-pins). The script's four call-graph gates, run directly on both modules (render closure, kernel shape, `meter_poll`, `command_submit`), pass on base and change |
| `check-sdk-types.sh` | not run: needs `npm ci` in `sdk/` (network) |
| `bash scripts/run-wasm-gates.sh` | pass (native, wasm scalar, wasm simd128, V8 spill) |
| `audit capi` through the edited Issue-544 validator (extracted from the workflow) | pass, 0 violations; the validator refuses a second record argument |
| `trace-graph-audit.sh` (the renamed step), `audit fixture-source` | pass (1,000,000 blocks); pass |
| `python3 fixtures/stem-identity/v1/generate.py --check` | pass |

### Shipped artifact

`bash scripts/build-web-audioworklet.sh --module-only` on base and change, same machine and
toolchain: base `f7bd75ca…`, change `fbc7c2ab…`, both 3,486,194 bytes, 17 bytes differ.

- 10 data bytes: seven `core::panic::Location` line fields, all in `crates/source/src/lib.rs`
  (lines 544 -> 518, 860 -> 834, 861 -> 835 twice, 1323 -> 1164, 1324 -> 1165 twice), columns
  unchanged: the deleted native lines above them.
- 7 code bytes, one function: `source::prepare_graph_source_set` (`func[597]`, preparation, not
  render) initializes three independent locals before its source loop in a different order
  (`local10 = 0; local12 = local7; local9 = 0` becomes `local9 = 0; local10 = 0;
  local12 = local7`), same length, same values. On `wasm32` the only compiled code change in the
  module's closure is the deleted always-zero retirement-worker class of the retained-resource
  report (its `wasm32` branch was a literal zero), which drops one term from two folds in that
  function.

`wasm-objdump -d` shows no other difference. The pin is not moved here; the batch boundary
re-pins.

### Follow-ups found (not done here)

1. Part B step 4 after #1033 (above).
2. Native-era residue that stays in the shared ring and so in the shipped module: the
   `native_decoder_sanitized_samples` field (transfer block, both telemetry structs, the fifth
   after-disarm telemetry word; always zero now), `SourceGraphSource`'s
   `additional_overhead_bytes`/`additional_largest_allocation_bytes` (host-core passes `0, 0`), and
   the producer's `deferred_block` arm, whose only retry was the native `commit_deferred` and
   which a prepared ring cannot reach. Removing them changes the transfer-block layout and the
   browser's resource rows, so it is its own issue.
3. The stem-identity corpus now has no CI consumer: stem-hasher's tests were the only one, and no
   workflow runs `generate.py --check`. One line in the lint job would restore it.
4. Two drafts target stem-hasher: `docs/handoffs/dual-mono-2026-09-27/issues/03` (report dual-mono
   stems) and the silence draft S2's "`tools/stem-hasher` subcommand" option. They need another
   home (the release CLI already detects dual-mono stems).
5. Open #124 (decode pool) closes as descoped.

## Sol verdict, attempt 1

**PASS.** Sol, 2026-09-28. Verified on a scratch merge of `002f6c3d` into the batch head
`92ef396f`. The merge is clean and the merge commit was not kept. `c867ec2a..92ef396f` touches only
the AudioWorklet pin and the qualification records, and #1021, #1024, #1026 and #1052 are already
inside the base, so the attempt was built on all four. Trial merges of the result with the in-flight
#1017, #1027, #1037, #1048, #1063 and #1064 branches are textually clean. None of those branches
names a deleted item or removes a `test-support` feature, so the declaring-package floor of 9 holds.
#1017 ignores one of the two exact-total tests re-pinned here on AArch64, which does not conflict
with the re-pin.

### What was checked

- **The live chunk path is unchanged.**
  - In `crates/source/src/lib.rs` the diff deletes only `cfg(not(wasm32))` items and the
    always-zero retirement-worker term of the retained report.
  - The host path is unchanged: `submit`, `submit_planes`, `validate_submission_metadata`,
    `take_recycled_block`, `publish_block`, `try_seek`, `read_block`, and all of capi's `src/` and
    `include/`, `host-core` and `host-web`.
  - The acked-batch question still holds: `publish_block` acks only after a successful push.
- **The guards that survive, all green:**
  - **C ABI submission and seeks:**
    - capi tests: `compile_publishes_both_children_and_source_control_is_region_checked`,
      `source_rejections_reach_the_c_host_as_their_own_diagnostic`, and `resource_lifecycle.rs`
      (6 tests).
    - `audit capi`: 100,000 calls over a host-fed ring followed by underruns, with 0 allocations,
      locks and syscalls. It passes the edited inline validator, which refuses a second record
      argument.
    - `check-capi-abi.sh` and its `--self-test`.
  - **Ring and `HostChunkProvider`:** `host_submission_is_fifo_wraparound_and_never_accepts_a_prefix`,
    `report_separates_session_pcm_from_source_overhead`,
    `played_planes_stay_intact_while_the_producer_fills_its_configured_depth`, and the two ports.
  - **Generation-tagged seeks:**
    - source: `seek_switches_at_boundary_and_discards_older_queued_audio`,
      `paused_seek_prepares_full_queues_without_consuming_target`,
      `graph_driver_forwards_underrun_and_seek_generation_facts`;
    - host-core: `source_diagnostics.rs` and `prepare.rs::source_control_errors_are_typed` (seek
      backpressure, generation zero, a generation that does not increase);
    - the browser adapter (host-web): `source_backpressure_seek_render_and_stable_output_are_bounded`,
      `paused_seek_recycles_full_internal_queue_before_first_target_quantum` and
      `source_seek_keeps_the_meter_epoch_and_absolute_peak_clock`.
  - **Underrun emits zero plus a counter:**
    - `underrun_is_positive_zero_and_eof_is_not_an_underrun` covers per-block frames and the
      event count.
    - `played_block_retention_keeps_the_pre_change_admission_sequence` covers cumulative frames.
    - host-core: `spectrum.rs::graph_wide_source_underrun_is_retained_in_the_completed_window`.
    - host-web: `ring_prefill_survives_stall` and `default_ring_covers_stall_tolerance`.
  - **Planted mutations in the ring:**
    - Deleting the cumulative-underrun increment turns `played_block_retention_keeps_the_pre_change_admission_sequence`
      red.
    - Deleting the stale-discard increment turns three tests red:
      `seek_switches_at_boundary_and_discards_older_queued_audio`,
      `paused_seek_prepares_full_queues_without_consuming_target` and
      `played_block_retention_keeps_the_pre_change_admission_sequence`.
- **capi re-pin.** The 16 bytes per plan are the `Box<[NativeSourceWorker]>` fat pointer deleted
  from `SourceGraphSourceSetDriver`.
  - The saving is the same on x86-64 and on both AArch64 mobile targets.
  - On `wasm32` the field was already compiled out, so no browser resource row moves
    (`check-browser-expected-resources.py` passes).
  - For a host-fed source, the deleted retirement-worker class was always zero.
  - The C header, the exported symbols and the ABI layouts are unchanged. A mobile app sees only a
    16-byte-smaller `source_overhead_bytes` value per plan.
  - The test oracle and the production report agree at 12,126 total and 3,934 overhead per plan.
- **Nothing live was deleted.**
  - Outside specs, handoffs, `artifacts/` and `docs/audits/`, I searched the merged tree for every
    deleted module, item, feature, script, CI step, audit record kind and tool. No code, workflow,
    script, Cargo feature, SDK file or `hosts/` file still names one. Docs residue is listed under
    finding 4.
  - `stem-hasher` had no operator workflow and no browser coupling: `stem-store-hash-v1.mjs` uses
    its own known-answer vectors.
  - Stem identity still has owners. The release CLI (`misofm/cli`, `src/stem-identity.ts`) owns
    serialization and WAVE stripping. Its copies of all ten corpus files are byte-identical to
    `fixtures/stem-identity/v1`, and `bun test tests/stem-identity.test.ts` passes 4/4. The browser
    stem store owns the BLAKE3-over-canonical-bytes check and the identity grammar.
- **Test lists.** I diffed the binary-qualified output of
  `cargo test --locked --workspace --all-targets --all-features -- --list`.
  - Base has 2,456 tests and the merge 2,398: exactly the 62 removals and 4 additions listed above.
  - Each removed test covers one of: the native worker or producer, the deleted decoder API, a
    deleted audit subject, or stem-hasher's own contract.
  - Where a claim is shared, host-path analogues exist:
    - host-core `prepare.rs`: caps admit exactly and reject one byte below, typed source errors,
      launch rates, sources fed independently;
    - host-core `source_diagnostics.rs`: `PlaneLength`, seek backpressure.
- **AGENTS.md.** The new sources paragraph matches the code.
  - No thread is spawned in `crates/source`.
  - capi and host-web feed `HostChunkProvider` through host-core's `SourceControlSet`.
  - It keeps bounded rings, generation-tagged seeks, underrun zero-plus-counter, no whole-stem
    loading, the rate set and no implicit SRC.
  - It removes nothing except the decode-worker clause.
- **Test value.** Each new or rewritten test was mutated; each went red, then green after revert.
  The unique catches below were checked against the whole `source` suite. For the two ring tests
  they were also checked against the host-core, capi, host-web and console-workload tests. For the
  decoder tests they were also checked against `native-pcm-runner`'s tests and `audit fixture-source`.
  - `one_quantum_ring_shape_matches_its_report_and_reads_back_planar`: a consumer
    `transfer_block_capacity()` that counts the retained block. No other test catches it.
  - `rejected_host_submission_publishes_nothing_and_leaves_the_producer_untouched`: a validator
    that admits a short chunk not marked end-of-region. No other test catches it.
  - `buffered_decodes_share_one_fill_and_a_straddled_refill_keeps_the_reader_position`: a decoder
    that refills and reads on every call instead of serving from its buffer. Only it and the
    rewritten failed-I/O test catch it.
  - `every_encoding_decodes_the_same_bits_however_the_region_is_partitioned`: a buffer-cursor byte
    offset computed with the `f32` stride instead of `block_align`, which corrupts every non-float
    encoding after a partial decode. No other test catches it.
  - `failed_io_forgets_reader_position_and_retry_reseeks` (rewritten): a failed read that keeps
    the stale reader position, so the retry reads without re-seeking. No other test catches it.
  - The three decoder tests go when `native_wave.rs` goes (finding 1).

### Gates on the merge

- `cargo check --locked --workspace --all-targets --all-features` passes with 0 warnings.
- `cargo clippy` with the same flags and `-D warnings` passes on all 44 packages.
- `cargo fmt --check` passes.
- The simd128 `wasm32` check of `host-web`, `source` and `host-core` passes.
- On `aarch64-apple-ios` and `aarch64-linux-android`, two checks pass:
  - `--all-targets -p source -p capi -p host-core`;
  - `--workspace --lib --all-features`, excluding `native-pcm-runner`, `wasm-console` and
    `wasm-gates`.
- test-debug-a's exact command, which runs the source, host-core, capi and host-web tests: 1,389
  passed, 0 failed, 9 ignored.
- `cargo test --release -p audit -p bench -p console-workload`: 154 passed.
- The `gain_pan_profile digests` output: all 17 rows are byte-identical to base.
- `scripts/run-wasm-gates.sh` passes.
- Also passing: `trace-graph-audit.sh`, `audit fixture-source` and `generate.py --check`.
- Of the 62 scripts (every `check-*.sh` and `check-*.py`, plus `test-ci-path-routing.py`,
  `test-test-support-ci.py`, `test-realtime-policy.sh` and `test-bench-policy.sh`):
  - 52 pass, including:
    - `check-ci-path-routing.py` and `test-ci-path-routing.py`;
    - `check-test-support-ci.py` and `test-test-support-ci.py`;
    - `check-workspace-policy.sh`, with the exact source-scrape allow-list;
    - `check-realtime-policy.sh` and `test-realtime-policy.sh`;
    - `check-session-policy.sh`, `check-env-vocabulary.sh` and `check-cross-targets.sh`;
    - `check-native-pcm-runner.sh`, `check-release-shape.py` and `check-host-core-policy.sh`.
  - The other ten are the next three rows.
- `check-web-audioworklet.sh` and `check-sdk-headless.sh` fail on the pin only. The merge builds
  `fbc7c2ab…`, while `92ef396f` pins `f7bd75ca…`. Under a trial re-pin, since reverted, both
  scripts pass in full. See finding 2.
- `check-sdk-types.sh` was not run: it needs `npm ci`. Seven `check-*.py` scripts require a path
  or a mode.
- **Shipped artifact.** Base `f7bd75ca…` and merge `fbc7c2ab…` are both 3,486,194 bytes, and 17
  bytes differ.
  - The data bytes are seven `Location` line fields for `crates/source/src/lib.rs`: 544 -> 518,
    860 -> 834, 861 -> 835 (twice), 1323 -> 1164 and 1324 -> 1165 (twice), with columns unchanged.
  - The code bytes are one same-length reorder of three independent local initializations in
    `func[597]` `source::prepare_graph_source_set`, which runs at preparation, not render.
  - `wasm-objdump -d` shows nothing else.
- **Pre-existing or environmental:**
  - The AArch64 `--all-targets -p host-core` check emits six dead-code warnings in
    `crates/host-core/tests/fp_environment.rs`, from x86-only helpers this change does not touch.
  - `check-env-vocabulary.sh` failed only while my scratch `target` symlink was present, and
    passes without it.

### Findings, by severity

1. **Medium: Part B step 4 has no tracked home.**
   - #1033's spec does not mention `native_wave.rs`, `audit fixture-source`, `fixtures/sources/v1`
     or the `NativeWave*` re-exports. Only this spec's evidence hands them on.
   - R4 approved Part B, so before #1035 closes, amend #1033's spec and its GitHub issue to absorb
     them, or open a stateless successor. Otherwise closing this issue drops approved scope.
2. **Medium, batch action: the AudioWorklet pin is stale.** Adding #1035 to the batch changes the
   shipped module, so the two pin gates are red on the batch until its boundary re-pin, which
   #1009's batch rule defers there. That re-pin must record the reason given here, or the
   combination of this reason with any other batch member that moves the module.
3. **Low: `docs/STEM_IDENTITY_V1.md` overstates the browser's part.**
   - The "Implementations" section names the browser stem store (`identity.js`) as implementing
     the contract. In fact `identity.js` only parses the `blake3:` spelling, and the store hashes
     canonical bytes it is given. It neither serializes decoded PCM nor strips WAVE; only the
     release CLI does.
   - "The reference WAVE path MUST strip…" now names no path in this repository.
   - The corpus has no CI consumer here (follow-up 3). Name the CLI as the serializer and WAVE
     owner, and add `generate.py --check` to the lint job.
4. **Low: stale references remain.**
   - `docs/derivations/241-browser-source-identities.md:186-195` still gives `stem-hasher`
     reproduction commands and says "the repository's native reference oracle agrees".
   - `docs/DELIVERY_CODEC_BOUNDARY.md:12` still names "the native WAVE/RF64 control-worker path".
   - `crates/source/src/lib.rs:1407` still says "native worker/decoder bytes".
   - `qualification.yml`'s test-debug-a comment still lists `source` among the packages whose
     `test-support` nothing turns on.
5. **Low: the producer's `deferred_block` arm is now reached by no test.** It lives in
   `take_recycled_block` and `publish_block`'s `Full` branch, is unreachable in a prepared ring, and
   ships in the browser module. Its only exercisers were the two deleted native deferred tests. It
   acks nothing, so no ack can precede a drop. Follow-up 2 should delete it.
6. **Info: duration-independent memory rests on structure alone on the host path.**
   `PcmSourceRingConfig` takes no duration, and `host_region_preparation_preserves_resources_and_absolute_ownership`
   varies only the origin. `audit source-duration` never measured the host path, so no claim is
   lost. A host-core test that prepares two sessions differing only in `frames` would re-earn the
   AGENTS.md gate for the shipping path.
