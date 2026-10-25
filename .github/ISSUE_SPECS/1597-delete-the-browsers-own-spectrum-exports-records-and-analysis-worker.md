# Delete the browser's own spectrum exports, records and analysis Worker

Slice S31c of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13, shape rule 3). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner,
2026-10-13): one implementer, normal tests, a batch review; no per-attempt verifier or mutation
runs.

## Product outcome

The browser has no spectrum code of its own. After #1596 no SDK path calls the browser's spectrum
exports; this slice deletes them, their `repr(C)` records, their ABI layout and render-thread rows,
the worklet's spectrum handler, the SDK's spectrum decoder and its analysis Worker. Spectrum in
the browser is the core's `SPECTRUM_*` frames through the one command entry, as on the C ABI.

## Context

- **host-web exports.** Every `miso_engine_web_v1_spectrum_*` export
  (`hosts/host-web/src/ffi.rs:2512-3424`), `miso_engine_web_v1_spectrum_hop_capability` (`:3845`)
  and `miso_engine_web_v1_boot_with_spectrum_hop` (`:3857`) with `reject_explicit_spectrum_hop`
  and `boot_staged_with_spectrum_hop` (`:3824-3843`); the spectrum part of `boot_staged`
  (`:3739-3753`) and of `reserve_stagings` (`:622`); `SpectrumStaging` (`:123-277`) and the helpers
  that only it uses (`:926-1524`, spectrum functions); the spectrum tests (`:5350-6438`).
- **host-web records and boot.** The spectrum constants and `WebSpectrum*` records
  (`hosts/host-web/src/lib.rs:226-418`), `boot_with_spectrum` and `boot_with_spectrum_config`
  (`:2170`, `:2178`), the spectrum staging bytes of the boot memory projection (`:6449-6470` and
  the spectrum terms near `:6525-6570`), and any spectrum branch that #1596 left in the browser's
  preparation.
- **Scripts.** The worklet's constants (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:59-71`,
  `:81`, `:95-100`), boot staging (`:277-336` spectrum parts, `stageSpectrumRequest` `:434`,
  `stageSpectrumCollectionRequest` `:476`), the `miso.spectrum.v1` dispatch (`:1057-1064`),
  `receiveSpectrum` (`:1267-1457`) and `readSpectrumStreamMetadata` (`:1459`); the worklet host's
  spectrum methods (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1482-1581`) and their
  `.d.ts` entries.
- **SDK.** `sdk/src/core/spectrum.ts` (`SpectrumModule` `:563`, `stageSpectrumRequest` `:416`,
  `stageSpectrumCollectionRequest` `:453`, `decodeSpectrumResult` `:788`), `BrowserSpectrum`
  (`sdk/src/browser/response.ts:399-676`), the spectrum messages of
  `sdk/src/browser/response-worker.ts` (`:18-31`, `:103-110`), the spectrum entries of
  `sdk/src/browser/host-mirror.ts` and `sdk/src/browser/shipped-host.d.ts`.
- **Tables and gates.** `tools/parameter-metadata/src/abi_layout.rs` (spectrum constants
  `:84-101`, exports `:141` and `:212` on), its generated outputs
  `sdk/assets/miso-engine-v1-abi-layout.json` and `sdk/src/generated/abi.ts`,
  `scripts/check-abi-layout-v1.py` and its self-test fixture,
  `tools/realtime-policy/render-thread-exports.json` (15 spectrum rows), the export lists of
  `scripts/check-web-audioworklet.sh` (`:210`, `:282-315`), the fakes of
  `scripts/test-web-audioworklet.mjs`, `scripts/check-sdk-deletions.py`.
- **Docs.** `docs/REALTIME_DEPENDENCY_POLICY.md:263-266`, `:331-337`, `:344-348`;
  `hosts/host-web/MUTATIONS.md` spectrum rows; qualification entries that call the exports.

## Decisions frozen for this slice

- **D1. Delete, do not wrap.** Every item in Context goes. No forwarding export, alias or
  deprecated record stays.
- **D2. Kept names.** The core constants (#1591 D2) stay in host-core. host-web re-exports none of
  them after this slice, because no browser export uses them.
- **D3. The deletion list is a gate.** `scripts/check-sdk-deletions.py` gains every deleted SDK
  export, type and asset name; the package smoke refuses them on every subpath.
- **D4. Realtime policy.** The render-thread table loses its spectrum rows; the realtime policy
  doc no longer names a spectrum export. No new render-thread export appears.

## Deliverables

1. D1-D4 across the paths below, with the generated files regenerated, never edited by hand.

## Authorized paths

- `hosts/host-web/src/`, `hosts/host-web/web/`, `hosts/host-web/tests/`,
  `hosts/host-web/qualification/`, `hosts/host-web/MUTATIONS.md`.
- `sdk/src/**`, `sdk/test/**`, `sdk/assets/miso-engine-v1-abi-layout.json` (regenerated).
- `tools/parameter-metadata/src/abi_layout.rs`, `tools/parameter-metadata/tests/abi_layout.rs`,
  `tools/realtime-policy/render-thread-exports.json`.
- `scripts/check-abi-layout-v1.py`, `scripts/fixtures/abi-layout-v1-self-test.json`,
  `scripts/check-web-audioworklet.sh`, `scripts/test-web-audioworklet.mjs`,
  `scripts/check-sdk-deletions.py`.
- `docs/REALTIME_DEPENDENCY_POLICY.md`.

## Non-goals

- No behaviour change: every #1596 gate passes unchanged.
- No core, protocol or C ABI change.

## Objective gates

Commands: as #1596's, plus `python3 -B scripts/check-abi-layout-v1.py`,
`python3 -B scripts/check-sdk-deletions.py`, `cargo run --locked --release -q -p realtime-policy`,
`bash scripts/check-workspace-policy.sh`.

1. **Nothing spectrum-specific is exported.** The built browser module's export list holds no name
   that contains `spectrum`; the ABI layout document holds no spectrum record or constant.
2. **The deletion gate.** `check-sdk-deletions.py` and the package smoke refuse every deleted
   name.
3. **Behaviour is unchanged.** Every #1596 gate passes, including the three-browser qualification
   on both legs, with `miso_engine_web_v1_render_allocation_count` at 0.

## Test value

- Gate 1 turns red if a browser-only spectrum path survives beside the core's frames.
- Gate 2 turns red if a deleted SDK name is still shipped.
- Gate 3 turns red if a deletion removes something the frame path still needed.

## Dependencies

- #1596.

## Supersedes and shrinks

- Shrinks *Engine-owned analysis: EQ response, spectrum, selective SDK streams, and sample-time
  correlation* (#763): its spectrum query and continuous spectrum now live in the core with one
  implementation and a C ABI form. Its other open scope stays (see the amendment entry).

## Decision record

- 2026-10-25, root ruling at #1596b: requirement. The headless boot still stages its spectrum taps
  through the staging exports so the first plan carries them (a headless engine adopts a plan only
  when its caller renders). When this issue deletes those exports, it owns the boot route and must
  give a headless boot its taps in the first plan: a capture is armable at sample 0.
- 2026-10-25, #1597 attempt 1 (implementer). **Boot route (root requirement 1).** A
  `SPECTRUM_CONFIGURE` that prepares a successor before the plan's first block adopts it in place:
  the host lends its plan to the command (`SessionState::command_with_plan`, which replaces
  `command_with_plan_observations`), `PlanState::adopt_before_first_block` runs the owner's
  `RealtimePlanOwner::adopt_before_first_block` (the first render call's adoption decision, made
  now; refused once a render call reached the owner), and the session promotes the candidate in
  the same command. The reply reports `rebuild` with `adopted_epoch == configured_epoch`, and a
  capture armed then covers sample 0. Why this design: it needs no new export, no boot input, no
  wire or C ABI change and no schema change, and it reuses the swap's own carry and promotion, so
  the in-place plan is exactly the one the first block would have adopted. Only a host that holds
  its plan between blocks lends it (the browser's single instance, headless and worklet); the C ABI
  is unchanged. The browser context is suspended until `createEngine` resolves, so its boot
  configure also meets an unrendered plan; if a render came first, the SDK's existing check refuses
  the boot (`abiMismatch`), never silently. Evidence (one-time, not committed): at `453ea7d26` the
  old staged boot and the new route render 63 blocks of the bus fixture with an output tap and a
  submix tap bit for bit alike, PCM and 31 meter frames. Paths beyond the scope note, forced by the
  route: `crates/engine/src/realtime/plan_exchange.rs`, `crates/control-plane/src/{plan.rs,
  control.rs,spectrum.rs}`, the control-plane tests that call `compile_children`, and
  `crates/capi/src/runtime/mod.rs` (the deleted `compile_children` argument).
- #1597 deletions. Exports (35): `boot_with_spectrum_hop`, `spectrum_analysis`, `spectrum_arm`,
  `spectrum_cancel`, `spectrum_capture_bytes`, `spectrum_capture_capacity`, `spectrum_capture_ptr`,
  `spectrum_capture_set_bytes`, `spectrum_close`, `spectrum_collection_entry_bytes`,
  `spectrum_collection_entry_capacity`, `spectrum_collection_entry_ptr`,
  `spectrum_collection_request_bytes`, `spectrum_collection_request_ptr`,
  `spectrum_collection_target_ids_capacity`, `spectrum_collection_target_ids_ptr`,
  `spectrum_hop_capability`, `spectrum_read`, `spectrum_request_bytes`, `spectrum_request_ptr`,
  `spectrum_result_bytes`, `spectrum_result_ptr`, `spectrum_select`, `spectrum_selection_epoch`,
  `spectrum_stream_analysis`, `spectrum_stream_analysis_configure`,
  `spectrum_stream_metadata_bytes`, `spectrum_stream_metadata_ptr`, `spectrum_stream_read`,
  `spectrum_stream_reset`, `spectrum_stream_select`, `spectrum_stream_start`, `spectrum_stream_stop`,
  `spectrum_target_id_capacity`, `spectrum_target_id_ptr` (all `miso_engine_web_v1_`). Records: the
  six `WebSpectrum*` records and their ABI layout structures, the spectrum constants of host-web
  and of the layout document, `SpectrumStaging`, `SpectrumPreparationRequest`, the host's spectrum
  methods, `compile_children`'s spectrum argument and the boot refusal it carried (the configure
  refuses the same tap, `spectrum.frame`), `preparation_fits`; `SessionState::spectrum_monitor{,_mut}`
  are test-support again. Worklet: the boot staging, the hop boot, `miso.spectrum.v1` and its
  handlers; worklet host: the spectrum boot options and the eight spectrum methods; both `.d.ts`
  twins. SDK: `SpectrumModule`, `decodeSpectrumResult`, `stageSpectrumRequest`,
  `stageSpectrumCollectionRequest`, `SpectrumEncodedStreamRead`, `OPTIONAL_ADDITIVE_EXPORTS`, the
  `spectrumCloseTimeout` code, `workletBootOptions`'s spectrum arguments and the boot options'
  spectrum fields of `toWebBootOptions`. The SDK keeps its spectrum bounds as its own constants
  (`SPECTRUM_MAXIMUM_ID_BYTES` 127, `SPECTRUM_MAXIMUM_CAPTURE_BYTES` 1 MiB, the 88- and 64-byte
  result and window headers of `resultBytes`), so no public result changes. The schema hash does
  not move; `ABI_VERSION` is unchanged (R2).
- #1597 gates, as the scope note restates them (R1): D3 adds no name list to
  `check-sdk-deletions.py` (the #1050 ruling stands; it runs as the offset gate). That nothing
  deleted ships is `check-web-audioworklet.sh`'s exact export list, `check-abi-layout-v1.py` on the
  regenerated layout and the package smoke. The staging-read qualification row keeps its observation
  and track-response reads; its spectrum reads and its stream instance went with the exports, so the
  worker instances are 15, not 16.
- #1597 attempt 2 (verifier folds, root rulings). The engine test
  `adopt_before_first_block_adopts_only_before_a_render_call` holds the owner's window: red on the
  mutant that deletes `self.entered = true;` from `render_contiguous` (the second call returns
  `Applied`), which no other test caught. What the one-time 63-block comparison covers: that no
  output or meter bit moves between the staged boot and the in-place route, the first block's
  meter restart included. That the taps are in the first plan (a capture armed before any block
  reads sample 0) is the control-plane gate-1 test's claim, not the comparison's.
- Public changes of the shipped worklet host declaration (`miso-engine-v1-audio-worklet-host.d.ts`,
  in the SDK as `browser/shipped-host.d.ts`): the types `MisoSpectrumTarget`,
  `MisoSpectrumChannels`, `MisoSpectrumBootOptions`, `MisoSpectrumCollectionEntry`,
  `MisoSpectrumCollectionBootOptions`, `MisoSpectrumOperation`, `MisoSpectrumReply`,
  `MisoSpectrumStreamMetadata`, `MisoSpectrumStreamStartReply`, `MisoSpectrumSelectionReply` and
  `MisoSpectrumStreamReadReply` are gone; `MisoWebBootOptions` lost `spectrumHopFrames`, `spectrum`
  and `spectrumCollection`; `MisoAudioWorkletHost` lost `armSpectrum`, `readSpectrum`,
  `cancelSpectrum`, `startSpectrumStream`, `selectSpectrum`, `selectSpectrumStream`,
  `readSpectrumStream` and `stopSpectrumStream`. The SDK barrels export none of them.
- Deleted tests and the live test that holds each claim (control-plane is
  `crates/control-plane/tests/spectrum_monitor.rs`; "feature gone" where the subject no longer
  exists):

  | deleted test | claim now held by |
  |---|---|
  | `the_first_plan_reads_the_reference_window` | `a_configure_before_the_first_block_reaches_sample_zero` (folded) |
  | `a_spectrum_boot_meters_every_submix_like_the_plain_boot` | `a_spectrum_tap_meters_every_submix_like_the_plain_boot` (renamed, configure route) |
  | `a_running_spectrum_stream_continues_across_a_rebuild` | `a_structural_edit_carries_the_stream`; `web_runtime_preparer_attaches_the_companions_on_every_branch` |
  | `removing_the_spectrum_target_strip_is_accepted_and_drops_the_service` | `a_removed_target_drops_its_entry_and_its_arm` |
  | `ordinary_spectrum_hops_reach_the_browser_stream_cadence` | `an_equal_or_hop_only_configure_publishes_no_plan`; SDK `boot-evals.mjs` copied-hop eval |
  | `explicit_ordinary_boot_publishes_h256_and_h1024_stream_metadata` | `an_equal_or_hop_only_configure_publishes_no_plan` |
  | `legacy_boot_wrappers_keep_the_derived_default_hop` | feature gone (hop boot exports) |
  | `invalid_explicit_hop_borrow_conflict_returns_zero_without_publication` | feature gone (hop boot export) |
  | `invalid_explicit_hops_refuse_before_publication_and_valid_retries_work` | feature gone; the configure refuses a hop (`spectrum.hop`), SDK `boot-evals.mjs` |
  | `spectrum_hop_capability_is_the_pinned_value` | feature gone; capability bit 16, SDK `boot-evals.mjs` |
  | `cold_spectrum_capture_configuration_is_idempotent` | feature gone (staging capture buffers) |
  | `spectrum_capture_capacity_validator_rejects_explicit_surplus` | feature gone (staging capture buffers) |
  | `collection_staging_uses_caller_count_and_packed_identity_bytes` | feature gone (collection staging) |
  | `staged_request_words_are_answered_as_the_core_answers_them` | feature gone; the configure's refusals, `a_refused_configure_changes_nothing_and_a_pending_edit_is_superseded` |
  | `the_read_export_answers_the_core_refusals` | `each_capture_refusal_answers_its_status_and_changes_nothing` |
  | `invalid_spectrum_cancel_refuses_before_touching_managed_stream` | `each_capture_refusal_answers_its_status_and_changes_nothing` |
  | `invalid_spectrum_channel_selection_preserves_window_for_valid_retry` | `each_capture_refusal_answers_its_status_and_changes_nothing` |
  | `select_after_the_callers_arm_is_backpressure_until_the_window_is_read` | `each_capture_refusal_answers_its_status_and_changes_nothing` (`spectrum.busy`) |
  | `collection_selection_is_atomic_and_keeps_one_active_capture` | `a_stream_read_follows_the_core_machine`; host-core `SpectrumService` select tests |
  | `stream_start_refuses_a_callers_arm_on_a_collection` | `a_stream_read_follows_the_core_machine` |
  | `stream_analysis_answers_by_the_stream_state` | `a_stream_read_follows_the_core_machine` |
  | `continuous_spectrum_ffi_reports_profile_and_analyzes_one_window` | `a_stream_read_follows_the_core_machine`; `a_capture_read_returns_the_reference_analysis` |
  | `a_promotion_that_drops_the_stream_ends_the_analysis` | feature gone (staging analysis); `a_removed_target_drops_its_entry_and_its_arm` |
  | `continuous_spectrum_ffi_imports_metadata_without_a_host_handle` | feature gone (analysis Worker) |
  | `imported_stream_analysis_accepts_supported_hops` | feature gone (analysis Worker) |
  | `imported_stream_analysis_rejects_unsupported_hops_transactionally` | feature gone (analysis Worker) |
  | `imported_stream_analysis_does_not_poison_one_shot_or_stream_history` | feature gone (analysis Worker) |
  | `stream_guard_refusals_preserve_markers_for_both_host_borrow_kinds` | feature gone (spectrum exports); the observation alias guards keep theirs |
