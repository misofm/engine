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
