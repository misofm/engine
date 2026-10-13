# Keep only handles and callbacks in the SDK and delete the browser's own observation exports

Slice S27b of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13). Second part of the planner's S27, after #1589. Code anchors verified on `main`
`df585e503`. Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch
review; no per-attempt verifier or mutation runs.

## Product outcome

The SDK keeps only what a client must own: its subscription handles, their timers and callbacks,
and each handle's delivery cursor. The core shares arms between subscribers (#1584 D3), so the SDK
sends one `Observe` edit per selection of each handle and no longer counts references or detects
window conflicts itself. The SDK learns which taps exist from the core's control list and the
generated effect catalog. host-web has no observation export, record, staging or boot word of its
own.

## Context

- **The SDK's own sharing.** `ObservationSubscriptionOwner` (`sdk/src/core/observation-subscriptions.ts:471`)
  keeps `BindingState` (`:199-211`, `refs` `:206`), the window-conflict check
  (`subscribe` `:547-565`, `update` `:605-630`), the re-arm and release rules (`update`
  `:631-660`, `close` `:708-745`) and the per-binding latest row. #1589 D5 keeps it so that each
  SDK binding holds one core reference.
- **The SDK's map.** `observationMap()` reads the browser's map exports
  (`sdk/src/browser/engine.ts:607`; `sdk/src/core/boundary.ts:612`), decoded by
  `enrichObservationMap` and `resolveObservationAddressesWithTracks`
  (`sdk/src/core/observation.ts:201`, `:239`). The generated catalog already carries each effect's
  declared taps (`ObservationDescriptor = EffectDescriptor["observations"][number]`,
  `observation.ts:69`). #1551's control list has one section header row per console slot and insert,
  with the effect's identity (#1551 D5, field 6 `effect`).
- **The browser's observation exports and records.** `miso_engine_web_v1_observation_count`
  (`hosts/host-web/src/ffi.rs:4227`) through `miso_engine_web_v1_observation_result_bytes`
  (`:4489`), with `ObservationStaging` (`:279`); `WebObservationSelection` and
  `WebObservationResult` (`hosts/host-web/src/lib.rs:726`, `:748`), `ObservationSelection`,
  `ObservationAddress`, `ObservationBinding`, `ObservationReadResult`, `ObservationReadValues`
  (`:678-850`) and the host methods `read_observations` ... `read_observation_addresses_into`
  (`:2785-2935`). `MAXIMUM_OBSERVATION_TAPS` (`:618`) bounds the boot word
  `live_control_observation_taps`, validated at `:6638`; `MAXIMUM_OBSERVATION_READS` (`:625`).
- Since #1589 no SDK path calls any of them.

## Decisions frozen for this slice

- **D1. The SDK sends per-handle edits.** `subscribe` sends one arm per selection; `update` sends,
  in one transaction, the disarms of the selections it drops and the arms of the selections it
  adds (disarms first, so a window change on a tap this handle alone holds is one re-arm, #1584 D3);
  `close` sends one disarm per selection. The core's `observation.window_conflict` and
  `observation.not_subscribed` reach the caller as `MisoUsageError` with today's messages
  (`"observation window conflicts with a managed subscriber"` for the first).
- **D2. What the SDK keeps.** Handles, the cadence timer, callbacks, per-handle delivery cursors
  and the missed-window counts (`nativeMissedWindows`, `skippedPublications`), `armed()` (#1589
  D4) and the bounds of `ObservationSubscriptionLimits` that limit the client's own work
  (`maximumHandles`, `maximumSelections`, `maximumCadenceMs`). `maximumBindings` and
  `maximumWindowBlocks` go: the core bounds both (the window by #1584 D2, the bindings by the
  plan's declared taps, one binding per tap at most, #1584 D3).
- **D3. The map from the core.** `observationMap()` is built from `CONTROL_LIST_GET` (the strip
  owners and their console and insert section headers, with the effect identity) and the
  generated catalog's taps. No index crosses the API (ruling C2).
- **D4. Browser deletions.** Every export from `miso_engine_web_v1_observation_count` to
  `miso_engine_web_v1_observation_result_bytes`, `ObservationStaging`, the two `Web*` records and
  their byte constants, the host-side observation types and methods above,
  `MAXIMUM_OBSERVATION_TAPS` and `MAXIMUM_OBSERVATION_READS`. Their rows go from
  `tools/parameter-metadata/src/abi_layout.rs`, `tools/realtime-policy/render-thread-exports.json`,
  `scripts/check-abi-layout-v1.py` and its fixture, the shipped host's `.d.ts` twins,
  `scripts/test-web-audioworklet.mjs` and the generated SDK mirrors.
- **D5. The boot word.** `live_control_observation_taps` becomes `reserved2: u64`, which must be
  zero (refused `web.options.reserved2` otherwise). `BOOT_OPTIONS_BYTES`, every other offset and
  `ABI_VERSION` stay. The SDK's `liveControls.observationTaps` option is deleted; the core attaches
  every declared tap on every plan (#1584 D7). The rule that a master designation needs observation
  taps goes from the boot check (`hosts/host-web/src/lib.rs:6642-6643`) and from the worklet host's
  option check (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:617`); the designation
  itself stays, as a projection of the meter frame.

## Deliverables

1. D1-D3 in `sdk/src/core/{observation-subscriptions.ts,observation.ts}`,
   `sdk/src/browser/engine.ts`, `sdk/src/headless/engine.ts`, `sdk/src/core/boundary.ts`,
   `sdk/src/core/abi.ts`.
2. D4 and D5 in host-web, the worklet scripts, the checkers and the regenerated mirrors.
3. The evals and tests below; deleted host-web tests are listed in the PR with the core or SDK
   test that now holds each claim.

## Authorized paths

- `sdk/src/**`, `sdk/test/**`, `sdk/assets/miso-engine-v1-abi-layout.json` (regenerated).
- `hosts/host-web/src/{lib.rs,ffi.rs,tests.rs}`, `hosts/host-web/web/*`,
  `hosts/host-web/qualification/{qualification.js,run.mjs}`.
- `tools/parameter-metadata/src/abi_layout.rs`, `tools/parameter-metadata/tests/abi_layout.rs`,
  `tools/realtime-policy/render-thread-exports.json`, `scripts/check-abi-layout-v1.py` and its
  self-test fixture, `scripts/check-web-audioworklet.sh`, `scripts/test-web-audioworklet.mjs`.

## Non-goals

- No change to the core's sharing rule, the edit or the read command.
- No meter, response or spectrum change (the spectrum and response subscriptions in the same SDK
  owner belong to their own slices).

## Objective gates

Commands: `bash scripts/check-sdk-types.sh`, `bash scripts/check-sdk-headless.sh <artifacts>`,
`bash scripts/check-sdk-generated.sh <artifacts>`, `python3 -B scripts/check-sdk-deletions.py`,
`bash scripts/sdk-package.sh check <artifacts>`, `cargo test --locked -p host-web --all-features`,
`cargo test --locked -p parameter-metadata`, #1534 gate 3's worklet commands, and the browser
qualification run of #1530 and #1535 on Chromium, Firefox and WebKit.

1. **Shared arms through the core.** On the headless engine, two handles subscribe the same tap
   and window: the engine holds one arm (one `OBSERVATION_READ` tag), closing one handle keeps the
   other's rows flowing, closing both disarms. A second window on the held tap rejects with the
   message of D1.
2. **Window change.** `update` of a handle that alone holds a tap from window 4 to 8 re-arms once:
   the reply has one new tag and the next ready row has `blocks == 8`.
3. **Map from the core.** `observationMap()` equals today's map for the observation fixture
   (`hosts/host-web/qualification/observation-session.json`) in strip, section and tap content,
   with stable IDs only.
4. **Deletions.** `python3 -B scripts/check-sdk-deletions.py` and the worklet export check list
   none of the deleted names; a boot with a nonzero `reserved2` is refused `web.options.reserved2`.
5. **Real browsers.** #1530's and #1535's gates pass in three browsers.

## Test value

- Gate 1 turns red if the SDK still counts references itself or the core does not share an arm.
- Gate 2 turns red if a window change sends two arms or a disarm that silences the tap.
- Gate 3 turns red if the core's list and catalog give a different set of taps than the browser
  map did.
- Gate 4 turns red if a deleted export, record or option survives.
- Gate 5 turns red if the slimmer SDK regresses the real-context observation flow.

## Dependencies

- #1589, #1584, #1586, #1551 (the section header rows), #1563.
- `hosts/host-web/src/lib.rs` is stream H's file; `sdk/src/browser/engine.ts` follows
  `STREAMS.md` row 104. This slice rebases after them.

## Supersedes and shrinks

- None.
