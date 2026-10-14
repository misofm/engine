# Delete the browser's own live-command path

Slice S10b of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design point C1:
"a behaviour that exists on one host only is a defect"; "What this supersedes"). Code anchors
verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one implementer, normal
tests, a batch review; no per-attempt verifier or mutation runs.

This slice lands after #1562 (the Worker commits `SET_CONTROL`) and after #1563 and #1564 (the SDK
no longer sends a value command, calls no prepared-target export and reads no index export).

## Product outcome

host-web keeps no value admission of its own. Its value command kinds, its admission and
composition, its prepared-target exports, its index exports, its own ramp table and its live-lane
boot word are gone, and so are host-core's browser-only mirrors. Every value in the browser goes
through the core, as on the C ABI. The browser keeps only transport and its monitor operations:
observation subscriptions (kinds 7 and 8) and the meter lease, which the later browser-feature
phase moves into the core (ruling C9, C13).

## Context

- **The value kinds and their admission.** Kinds 1-6 and 9-17 (`hosts/host-web/src/lib.rs:837-949`),
  `submit_commands` (`:3477`), `admit_commands` (`:4798`) and `admit_commands_staged` (`:5051`),
  the record builders (`into_track_record` `:4443`, `into_effect_records` `:4685`), the passes for
  solo, VCA and follow, and #1548's temporary `WebEffectiveView`. The reasons that only these kinds
  use (`COMMAND_REASON_NOT_SOLOABLE` `:1076`, `_UNKNOWN_ROUTE` `:1081`, `_UNKNOWN_VCA` `:1087`).
- **The prepared-target exports.** `miso_engine_web_v1_eq_target_*` (`hosts/host-web/src/ffi.rs:3578-3632`,
  `:4058-4073`), `miso_engine_web_v1_input_filters_prepare` (`:3638`) and `_config_copy` (`:4084`),
  `miso_engine_web_v1_prepared_command_submit` (`:4025`), their codec and workspace
  `hosts/host-web/src/control_targets.rs`, and the page module `hosts/host-web/web/prepared-control.js`.
  The core designs these targets in the classifier since #1554 (EQ) and #1556 (input filters).
- **The index exports.** `miso_engine_web_v1_live_control_{track,submix,route,vca}_{count,id}`
  (`ffi.rs:4148-4221`), read by the worklet's session map
  (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:597-660`). #1564 moves the SDK's strip order
  to the control list.
- **The ramp table.** `SessionRamps` (`lib.rs:1549`), built in `compile_ready` (`:6960`); #1562 D6
  leaves it serving only the old path.
- **The boot word.** `live_control_command_queue_records` (offset 32 of the boot options,
  `lib.rs:1251`): nonzero allocates the command staging (`:6498-6502`) and is required by
  observation taps (`:6638-6648`). Since #1562 the core chooses the live lanes. *Make the browser
  live-command boot word an on/off flag* (#1524) planned to rename it; this slice deletes it.
- **host-core's browser-only mirrors.** `LiveVcaState` (`crates/host-core/src/vca.rs:84`),
  `LiveControlSoloState` and `StripMuteSeed` (`crates/host-core/src/solo.rs:112-129`),
  `LiveRouteState` and `LiveRouteMuteFollow` (`crates/host-core/src/live_route_state.rs:67`,
  `:232`), re-exported at `crates/host-core/src/lib.rs:136`, `:164`, `:183`. Only host-web and their
  own tests use them (`crates/host-core/tests/vca_live.rs`); the C ABI diffs committed models (#1558,
  #1559).
- **The static gate.** `scripts/check-web-audioworklet.sh:494-511` holds `command_submit` to the
  allocation-only call graph on the worklet thread; `:258` lists `prepared_command_submit`.
- **Tests.** About 68 of the 168 tests in `hosts/host-web/src/tests.rs` use a value kind. The
  raw-module oracle `hosts/host-web/tests/browser-v1/direct-oracle.mjs` has a command-record
  timeline (kinds 1-6); #1564 added the `SET_CONTROL` timeline beside it.

## Decisions frozen for this slice

- **D1. Deleted kinds.** Kinds 1-6 and 9-17 are removed. A record with one of those kinds is
  refused `COMMAND_REASON_MALFORMED` at its index, before anything is admitted, as an unknown kind
  is today (`hosts/host-web/src/lib.rs:1031`, `:4564`). Their numbers are never reallocated. `COMMAND_RECORD_BYTES`, the record
  layout and `ABI_VERSION` do not change: kinds 7 and 8 keep the record.
- **D2. Deleted admission.** Every value arm, pass and helper of `admit_commands_staged`, the
  record builders, `WebEffectiveView`, `SessionRamps`, and the reasons that only deleted kinds use
  are removed. What stays in `admit_commands_staged` is the observation path only.
- **D3. Deleted exports.** `miso_engine_web_v1_prepared_command_submit`, every
  `miso_engine_web_v1_eq_target_*`, `miso_engine_web_v1_input_filters_prepare`,
  `miso_engine_web_v1_input_filters_config_copy` and every
  `miso_engine_web_v1_live_control_{track,submix,route,vca}_{count,id}`, with
  `hosts/host-web/src/control_targets.rs` and `hosts/host-web/web/prepared-control.js`. The
  worklet's `preparedCommand` message and its session-map fields for routes and VCAs go; its strip
  order, if the worklet still needs one for meters, comes from the core's list (#1564 D5).
- **D4. The boot word.** Offset 32 is renamed `reserved1: u64` and must be zero (refused
  `web.options.reserved1` otherwise). The command staging is allocated when
  `live_control_observation_taps != 0`, and kinds 7 and 8 are accepted only then (otherwise
  `RESULT_UNSUPPORTED` with `COMMAND_REASON_UNSUPPORTED_KIND`, as a command on a host with no live
  controls attached is today, `lib.rs:3489-3491`).
  `BOOT_OPTIONS_BYTES`, every other offset and `ABI_VERSION` do not change. The SDK's
  `policy.liveControls.commandQueueRecords` option is deleted; `host-mirror.ts` writes 0.
- **D5. Deleted mirrors.** `LiveVcaState`, `LiveVcaFaderDelta`, `LiveControlSoloState`,
  `LiveControlMuteDelta`, `StripMuteSeed`, `LiveRoute`, `LiveRouteState`, `LiveRouteStateError`,
  `LiveRouteMuteFollow` and (batch 4 review fold, #1560's browser fix) `LiveStripValues` with its
  module `strip_values.rs` and `StripValues` are removed from host-core with their modules' code and
  `crates/host-core/tests/vca_live.rs`. Doc comments elsewhere that point at them point at
  `controls::effective` instead.
- **D6. Tests.** Each host-web test that used a deleted kind is either rewritten to send
  `SET_CONTROL` through #1562's entry and keeps its claim, or deleted because a core test or the
  #1560 parity test holds the same claim; the PR lists each deleted test and the test that now holds
  its claim. The command-record timeline of `direct-oracle.mjs` is removed and `expected.json`
  loses its digest; the `SET_CONTROL` timeline of #1564 stays the cross-target owner.
- **D7. Gates and mirrors follow.** `scripts/check-web-audioworklet.sh` drops
  `prepared_command_submit` and keeps the `command_submit` allocation leg (kinds 7 and 8 still run
  on the worklet thread). The ABI layout document and its generator, `sdk/src/generated/abi.ts`,
  `sdk/assets/miso-engine-v1-abi-layout.json` (regenerated), `scripts/check-abi-layout-v1.py` and
  its self-test fixture, `tools/realtime-policy/render-thread-exports.json`, the shipped host's
  `.d.ts` files and `scripts/test-web-audioworklet.mjs` drop every deleted export, kind and field.
- **D8. Benchmarks.** `scripts/web-mixing-automation-benchmark.mjs`,
  `scripts/web-mixing-automation-lib.jq` and `tools/console-workload/src/mixing_automation.rs`
  send their edits as `SET_CONTROL` through the `control` message. Their record keys follow D4.

## Deliverables

1. D1-D4 and D6-D8 in host-web, the worklet scripts, the checkers and the generated mirrors.
2. D5 in host-core.
3. The SDK option deletion of D4 (`sdk/src/browser/{policy.ts,host-mirror.ts,engine.ts}`,
  `sdk/src/core/boundary.ts`).

## Authorized paths

- `hosts/host-web/src/`, `hosts/host-web/web/`, `hosts/host-web/tests/`,
  `hosts/host-web/qualification/`, `hosts/host-web/MUTATIONS.md`, `hosts/host-web/DEPLOYMENT.md`.
- `crates/host-core/src/{vca.rs,solo.rs,live_route_state.rs,strip_values.rs,lib.rs}`,
  `crates/host-core/tests/{vca_live.rs,MUTATIONS.md}`.
- `scripts/check-web-audioworklet.sh`, `scripts/check-web-audioworklet-callgraph.py`,
  `scripts/test-web-audioworklet.mjs`, `scripts/check-abi-layout-v1.py`,
  `scripts/fixtures/abi-layout-v1-self-test.json`, `scripts/check-browser-expected-resources.py`,
  `scripts/web-mixing-automation-benchmark.mjs`, `scripts/web-mixing-automation-lib.jq`.
- `tools/parameter-metadata/src/abi_layout.rs`, `tools/parameter-metadata/tests/{abi_layout.rs,round_trip.rs}`,
  `tools/realtime-policy/render-thread-exports.json`, `tools/console-workload/src/mixing_automation.rs`.
- Regenerated only: `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts`.
- `sdk/src/browser/{policy.ts,host-mirror.ts,engine.ts,shipped-host.d.ts}`,
  `sdk/src/core/boundary.ts` (D4 only), `sdk/test/**` (D4 only).

## Non-goals

- No change to observation subscriptions, the meter lease or spectrum (the later phase).
- No change to the core, the classifier or any record type. (Whether builtins-compiler's record
  types get private fields stays with #1423.)
- No new export.

## Objective gates

Commands: `cargo test --locked -p host-web --features host-web/test-support`,
`cargo test --locked -p host-core --all-features`, `cargo test --locked -p capi`,
`bash scripts/check-web-audioworklet.sh`, `bash scripts/test-web-audioworklet.sh`,
`python3 -B scripts/check-abi-layout-v1.py`,
`python3 -B scripts/check-browser-expected-resources.py --artifacts target/ci/qualification-artifacts`,
`bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
`bash scripts/check-sdk-types.sh`, `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`,
`bash scripts/check-workspace-policy.sh`, and the browser qualification for chromium, firefox and
webkit on both legs.

1. **Deleted kinds refuse.** For each of kinds 1-6 and 9-17, a one-record command is refused
   `COMMAND_REASON_MALFORMED` at index 0 and changes no cell.
2. **The boot word.** A nonzero offset 32 is refused `web.options.reserved1`; a boot with
   observation taps and offset 32 zero accepts kinds 7 and 8; a boot without taps refuses them
   with `COMMAND_REASON_UNSUPPORTED_KIND`.
3. **Every value still works.** The #1560 parity test and #1562's gates 1-6 pass unchanged.
4. **Observation still works.** Every observation test and every observation row of the browser
   qualification passes unchanged.
5. **Realtime.** `hosts/host-web/tests/worker_admission_realtime.rs` and the browser's
   `render_allocation_count` gate pass unchanged.

## Test value

- Gate 1 turns red if a value kind still reaches the cells without the core.
- Gate 2 turns red if the deleted word still changes behaviour, or if observation loses its
  staging.
- Gate 3 turns red if a deletion removed something the core path still needed.
- Gate 4 turns red if the deletion broke the monitor operations that stay.
- Gate 5 turns red if the remaining command path allocates on the render thread.

## Dependencies

- #1562, #1563, #1564.

## Supersedes and shrinks

- Supersedes *Make the browser live-command boot word an on/off flag* (#1524): the word is deleted.
- Supersedes *Test that a refused batch does not undo a committed browser VCA batch* (#1525): the
  browser VCA mirror and its commit are deleted; #1559 gate 3 holds the shared-path half.
- Shrinks *Make live strip records valid by construction* (#1423): no host builds a strip record
  any more; the private-field change stays as defence in depth for host-core.

## Amendment (decision 17, root 2026-10-13)

**Single instance first.** "Both legs" reads "the non-isolated leg" until #1332. The slice is
split. All three pieces stay in this issue.

- **#1565A**: D1-D3, D6 and D8 (kinds 1-6 and 9-17, their admission, `WebEffectiveView`,
  `SessionRamps`, the tests and benchmarks). Gates 1, 3 and 5. Dependencies: #1562a and #1563B
  (not #1564). It lands in one batch with #1381a, #1562a, #1570b and #1563B.
- **#1565B**: the exports and mirrors (D3's export deletions, D5, D7): `control_targets.rs`,
  `prepared-control.js`. Gate 4. Dependencies: #1565A, #1564C.
- **#1565C**: D4 (the boot word `reserved1`). Gate 2. Dependency: #1565A.

When #1565 merges, *Make the browser live-command boot word an on/off flag* (#1524) and *Test that
a refused batch does not undo a committed browser VCA batch* (#1525) close as superseded; neither
is implemented (root Q6).

**Flip batch 6W (root ruling A1).** #1381a, #1562a, #1570b, #1563B and #1565A land together, in one batch of lane W. Each slice keeps its own commits and tests and is green at its own boundary; the batch runs the full CI-equivalent gate set once. If one slice fails, the batch stops: `main` never gets a partial flip, so it never has two writers of one plan's revision gate and cells. No transitional API lends `SessionState`'s producers or revision to the old command path.

## Decision record: #1565A (flip batch, written 2026-10-14)

Root's slice order put D1-D4, D5, D6, D7 and D8 into one pass, because the SDK no longer sends a
value command (#1563B) and the Worker-side path serves every value (#1562a). What shipped and where
it departs from the filed text:

### What was deleted

- **Host-web.** Command kinds 1-6 and 9-17 and their constants; `admit_commands_staged`, the record
  builders, the solo, VCA and follow passes, `WebEffectiveView`, `WebControlRows`, `SessionRamps`,
  `SESSION_DEFAULT_RAMP`; the retained `ReadyOwnership` mirrors (`solo`, `vcas`, `routes`,
  `strip_values`, `ramps`, `rows`, `controls`, `plan_gate`, `effect_controls`, `route_controls`,
  `input_filter_shadows`) and the host generation; `prepared_command_submit`, every
  `eq_target_*` and `input_filters_*` export, `prepared_companion_ptr` and `_capacity`, the two
  `live_control_route_*` and `live_control_vca_*` pairs; `control_targets.rs` and
  `web/prepared-control.js`; the browser VCA bounds (`MAXIMUM_BROWSER_VCAS`,
  `MAXIMUM_BROWSER_VCA_REACH_PAIRS`, `web.vca.*`), which bounded only the deleted VCA command's
  per-command work. Reasons `notSoloable`, `unknownRoute` and `unknownVca` are gone.
- **Host-core (D5).** `LiveVcaState`, `LiveVcaFaderDelta`, `LiveControlSoloState`,
  `LiveControlMuteDelta`, `StripMuteSeed`, `LiveRoute`, `LiveRouteState`, `LiveRouteStateError`,
  `LiveRouteMuteFollow`, `LiveStripValues`, `StripValues` with their four modules and
  `tests/vca_live.rs`; the rows of `host-core/tests/MUTATIONS.md` and `host-web/MUTATIONS.md` that
  named them.
- **D4, taken in the same pass (the filed #1565C).** The word at offset 32 is `reserved1`, refused
  `web.options.reserved1` when nonzero. The command staging and its decode array exist exactly when
  `live_control_observation_taps != 0`; a boot without taps refuses every submission `UNSUPPORTED`
  with `COMMAND_REASON_UNSUPPORTED_KIND`, so reason `observationUnbound` (11) is declared and
  reachable by nothing (its number stays). `DEFAULT_COMMAND_QUEUE_RECORDS` is gone;
  `live_control_defaults()` is the default meter window alone. The decode array shrank from
  `2 * 256 + 2 * strips + routes + 2 * vca-reached strips` entries of `StagedCommand` to 256
  `StagedObserve` entries (none without taps), and the ID staging no longer reserves room for route
  and VCA IDs.
- **Kept on purpose, differing from D3.** `live_control_track_{count,id}` and
  `live_control_submix_{count,id}` stay. The SDK's `shape()` and the meter frame's strip order read
  them, and D3 itself moves that order to the core's list in #1564 D5 (not in this batch;
  #1565B needs #1564C). Reasons `unknownParameter` (5) and `domain` (6), which nothing raises any
  more, stay declared too: the filed text removes only the three reasons named above.

### Tests

- 111 of the 176 unit tests in `hosts/host-web/src/tests.rs` left (seven others were added). 98 of
  the 113 `#[ignore]`d for this slice were deleted outright: each tested a deleted kind, mirror or export, and the core path
  holds the claim in `tests/control_admission.rs` (gates 1-6, 11), `tests/control_parity.rs`
  (gate 8: path and revision, every block's bits at four rates, the superseded count, the
  `CONTROL_LIST_GET`, `CONTROL_STATE_GET` and snapshot bytes), `crates/host-core/tests/live_edit_classifier.rs`,
  `live_delta.rs` (including the delay's and the multiband's prepared bypass, #1266) and
  `crates/controls`. The 15 others were rewritten or deleted for a stated reason:
  - rewritten: `decoded_command_resource_is_exact_...` (now `observation_staging_resource_is_exact_for_the_taps_off_and_on_modes`),
    `exact_retained_total_is_checked_as_one_budget_not_independent_caps`,
    `idle_render_skips_admission_counter_clear_without_losing_queue_credit`,
    `observation_misuse_is_typed_and_all_or_nothing`, `a_submission_names_each_observation_tap_once`,
    `bus_edits_and_a_bus_observation_admit_and_render_without_allocating` (now the observation alone),
    `unknown_targets_are_typed_and_leave_the_engine_untouched` (now gate 1,
    `every_deleted_kind_is_refused_malformed_at_its_index_and_changes_no_cell`) and
    `native_command_timeline_digest_pins_the_wasm_parity` (D6, below);
  - deleted, the claim held elsewhere: `the_status_reports_the_sessions_superseded_count`
    (`control_parity.rs` compares the browser status word with the C ABI's count),
    `effect_control_browser_table_and_payload_reach_exact_budget_gate` and
    `late_mixed_effect_refusal_...` (the control plane's retained row, charged once, and the core's
    atomic transactions), `a_bus_session_admits_and_renders_without_allocating` (admission is control
    work now), `retired_rack_codes_are_refused_on_every_browser_record` (the observe records' retired
    racks are in `observation_misuse_...`), `a_live_bypass_cannot_lift_the_delay_or_multiband_session_bypass`
    (`live_delta.rs` #1266 gate 1) and `every_browser_plan_carries_the_strip_lanes_...` (the boot
    word no longer chooses lanes since #1562a; host-core's lane-bit tests hold the claim).
- New: gate 1 above; gate 2, `the_boot_word_is_reserved_and_observation_staging_follows_the_taps`;
  `a_subscription_without_capacity_is_unsupported`; and the D6 pair below.
- `tests/control_command_timing.rs` (#1612) did not compile on the batch head (it called the
  `native_staging::drain_reliable_events` that #1570b deleted); it drains through `control_event`
  now. Every `tests/*.rs` boot dropped the queue word.
- `tools/parameter-metadata/tests/round_trip.rs::every_metadata_id_resolves_through_a_command_acknowledgement`
  sent kind 4. It is now `every_metadata_effect_parameter_is_a_control_row_with_the_same_liveness`
  (every metadata parameter ID is a row of the control catalog, with the same `automatable` and
  `liveUpdatable` flags): driving the host there would add `protocol` and `test-support` to a
  crate whose manifest is outside this slice, and `tests/control_admission.rs` already commits a
  `SET_CONTROL` on an insert row and renders it as a fresh plan. The ABI-layout test's command
  record offsets now use an observe record.
- Gates `check-web-audioworklet-callgraph.py`: the `meter_poll` pin follows the renumbered `impl`
  (`Ms5_` to `Ms0_`) and `command_submit` has an empty indirect-site table (its two prepared-owner
  rows are gone); the session-map shape gate drops the route and VCA mutations.

### D6: the cross-target timeline

`direct-oracle.mjs` lost the command-record timeline (kinds 1-6, `createPreparedControl`) and gained
`runSetControlTimeline`: four `SESSION_TRANSACTION_APPLY` frames (a fader move, a pan move, the EQ
band 1 gain, and one refused out-of-domain fader value) committed as
`tests/browser-v1/set-control-frames/frame-N.bin`, sent through `control_request_ptr` and
`control_command`, with the events drained the way the worklet drains them. The native twin is
`tests::native_set_control_timeline_digest_pins_the_wasm_parity`; the frames are pinned to their
encoder by `tests::set_control_frames_are_the_committed_fixtures` (set
`MISO_ENGINE_WRITE_SET_CONTROL_FRAMES=1` to rewrite them). `expected.json` lost `commandTimeline` and
`nativeCommandTimelinePcmF32leSha256` and gained `setControlTimeline` and
`nativeSetControlTimelinePcmF32leSha256`. The observation timeline's no-taps leg now reports
`unsupportedKind` (7), not `observationUnbound` (11).

Re-pins and their history lines are recorded with the batch's gate run below.

### Re-pins and evidence (flip batch, 2026-10-14)

- `expected.json` (history line: #1565A deleted the command-record timeline; the cross-target
  owner is the `SET_CONTROL` timeline): `setControlTimeline` digest
  `c3659578f50d02030fcef19df98be88d70e89f2a0c9bade3e23e9570864d1cfd`, native and simd128 equal on
  the first run; the frames answer 80, 80, 80 and 256 bytes (the fourth is the refused
  `control.domain` frame at the unchanged revision 4). The two status `memoryBytes` rows moved
  (2,424,832 to 2,883,584 at boot; the old value predates the control plane's allocations since
  #1381a); the observation timeline's no-taps leg reports reason 7 (was 11); the native/simd128
  identity and observation digests did not move.
- Boot growth of the 1 MiB worst document: 25,427,968 bytes, unchanged from #1381a (the peak is the
  parse transient; the multiplier-25 gate passes).
- Deleted-kind refusal and the boot word: gates 1 and 2, as tests above. Gates 3 and 5 run unchanged
  (`control_parity.rs`, `control_admission.rs`, `worker_admission_realtime.rs`). Gate 4 and the
  browser qualification rows were not run here (no local browser runs).
- `RUSTDOCFLAGS='-D warnings' cargo doc` fails on `crates/control-plane/src/control.rs:236`
  (`SessionControlSnapshot::effect_cells` unresolved link) on the batch head, also without this
  slice's changes; that crate belongs to #1613's lane.
