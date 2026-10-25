# Delete the browser's own track-response capture

Slice S21b of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13), second half of the planner's S21. Code anchors verified on `main` `df585e503`. Feature
sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review; no per-attempt
verifier or mutation runs.

## Product outcome

The browser has no track-response code of its own. Its twelve `track_response` exports, its
capture record, the worklet's capture handler and the 1 MiB capture staging that every boot
prewarmed are deleted. The browser keeps only transport, as decision 17 C1 requires; the track
response runs through the core on every host (#1577, #1578, #1579).

## Context

- **Exports.** `hosts/host-web/src/ffi.rs:3426-3574` (request, sizes, snapshot, `set_bytes`, track
  ID, capture, analysis, result and close).
- **Logic.** `ResponseStaging` (`ffi.rs:109-121`; after #1574 only its live fields remain), its
  constructor and reset (`:460-504`), `live_response_staging_retained_bytes` and
  `live_response_staging_largest_allocation_bytes` (`:380-394`), `live_response_rack_raw` and
  `live_response_rack` (`:755-779`), `copy_live_record` (`:881-902`),
  `live_response_error_code` and `live_response_capture_error_code` (`:904-925`), and
  `live_response_grid` through `run_live_response_analysis` (`:1526-2110`). The test-only fault
  hooks (`:630-631`), the native staging hook (`track_response_request`, `:4700`) and
  `live_response_ffi_tests` (`:4881-5348`).
- **Records and constants.** `hosts/host-web/src/lib.rs`: `RESPONSE_GRID_*` and
  `RESPONSE_CHANNEL_*` (`:185-196`, left by #1574), `LIVE_RESPONSE_*` (`:210-224`),
  `WebLiveResponseRequest`, `WebLiveResponseOwner`, `WebLiveResponseSection`,
  `WebLiveResponseResult` and their sizes (`:420-511`), `LIVE_RESPONSE_RACK_*` (`:968-975`),
  `AudioWorkletEngineHost::copy_response_snapshot` (`:2499-2515`), and the boot budget's two terms
  (`:6523`, `:6563`).
- **Scripts.** The worklet: constants (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:44-58`),
  the boot-time views (`:336-367`), the `miso.trackresponse.v1` dispatch (`:1054-1056`) and
  `receiveTrackResponse` (`:1182-1263`). The worklet host: constants (`:124-135`),
  `#inFlightTrackResponses` (`:723`), the `trackResponse` pending kind (`:883-884`),
  `captureTrackResponse` (`:1453`); its `.d.ts` (`miso-engine-v1-audio-worklet-host.d.ts:546-573` and `:973`)
  and the shipped copy (`sdk/src/browser/shipped-host.d.ts:973`).
- **Mirrors.** `tools/realtime-policy/render-thread-exports.json:18-20`;
  `tools/parameter-metadata/src/abi_layout.rs` (imports `:66-80`, `:96`; exports `:247-258`;
  `live_response_request_fields` `:908`, `live_response_owner_fields` `:1287`,
  `live_response_section_fields` `:1360`, `live_response_result_fields` `:1378`; structure rows
  `"liveResponseRequest"` `:1911` and `"liveResponseOwner"`, `"liveResponseSection"`,
  `"liveResponseResult"` `:1960-1974`; constant tables `"liveResponseRacks"` `:2064`,
  `"responseGrids"` `:2099`, `"responseChannels"` `:2107`, `"liveResponseModes"` `:2124`,
  `"liveResponseMeanings"` `:2129`; `liveResponseCaptureBytes` `:2214`);
  `scripts/check-abi-layout-v1.py` (`:98`, `:229-240`); `scripts/check-web-audioworklet.sh`
  (the `track_response` lines of `expected_exports`, from `:203`); `sdk/src/generated/abi.ts` and
  `sdk/assets/miso-engine-v1-abi-layout.json` (generated);
  `hosts/host-web/tests/render_locked_staging.rs` (`:28`, `:56-63`, `:163`); the host-level
  qualification row (`hosts/host-web/qualification/qualification.js:716-726`,
  `hosts/host-web/qualification/run.mjs:275`).

## Decisions frozen for this slice

- **D1. Everything in Context is deleted.** No replacement export is added: the command entry
  (#1562) and the event lane (#1570) carry the track response.
- **D2. The host-level qualification row** sends a `RESPONSE_GET` with `strip_id` through the
  `control` port message and checks one `RESPONSE_READY` with outcome 0 from #1570's lane,
  replacing the `captureTrackResponse` row.
- **D3. The boot budget** loses the two staging terms. The worklet instance retains 1 MiB less
  after boot; the boot budget checks keep their ceilings.
- **D4. No behaviour change.** The SDK already uses the frames (#1579); the core capture and its
  render step are unchanged.

## Deliverables

1. D1 in `hosts/host-web`, the worklet scripts, the declarations and every mirror.
2. D2 in the qualification; D3 in `hosts/host-web/src/lib.rs`.

## Authorized paths

- `hosts/host-web/src/{ffi.rs,lib.rs,tests.rs}`, `hosts/host-web/tests/render_locked_staging.rs`.
- `hosts/host-web/web/{miso-engine-v1-audio-worklet.js,miso-engine-v1-audio-worklet-host.js,
  miso-engine-v1-audio-worklet-host.d.ts}`, `sdk/src/browser/shipped-host.d.ts`.
- `tools/realtime-policy/render-thread-exports.json`, `tools/parameter-metadata/src/abi_layout.rs`,
  `scripts/check-abi-layout-v1.py` and its self-test fixture, `scripts/check-web-audioworklet.sh`,
  `sdk/src/generated/abi.ts`, `sdk/assets/miso-engine-v1-abi-layout.json` (regenerated).
- `hosts/host-web/qualification/{qualification.js,run.mjs}`.

## Non-goals

- No change to the core, the protocol or the SDK's public API.

## Objective gates

Commands: `cargo test --locked -p host-web`, `bash scripts/check-web-audioworklet.sh`,
`bash scripts/test-web-audioworklet.sh`, `python3 -B scripts/check-abi-layout-v1.py`,
`cargo build --locked --release -p realtime-policy && ./target/release/realtime_policy`,
`bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
`bash scripts/check-sdk-types.sh` (`check-web-audioworklet.sh` runs the boot budget check,
`scripts/check-web-audioworklet.sh:601`), and the browser qualification (`npm run qualify -- --artifacts ...
--sdk-root ... --browser <b> --check-matrix --self-test-mutations` in
`hosts/host-web/qualification`) for chromium, firefox and webkit.

1. **The surface is gone.** `check-web-audioworklet.sh`, `check-abi-layout-v1.py` and the
   realtime policy's export table list no `track_response` export; both worklet harnesses fail if
   a worklet calls an export the table does not list (their existing rule).
2. **Still served.** D2's qualification row passes in three browsers on both legs, and
   `sdk-track-response-subscription` (#1579) still passes.
3. **Render stays clean.** `miso_engine_web_v1_render_allocation_count` reads 0 on both legs over
   the qualification workload, which includes D2's request.
4. **Smaller boot.** The boot report's retained bytes for a fixture session are lower than on the
   merge base by at least `1 << 20` (PR evidence); the boot budget check passes.

## Test value

- Gate 1 turns red if a deleted export survives in one list or is still called by a worklet.
- Gate 2 turns red if the browser loses the track response with its own path.
- Gate 3 turns red if the core's render step allocates in the worklet.

## Dependencies

- #1579 (the SDK no longer calls the deleted code), #1574 (which deletes the preview half of
  `ResponseStaging` first).
- Stream H owns `hosts/host-web`, the worklet scripts and `sdk/`; `abi_layout.rs` follows
  `docs/handoffs/decision-15-2026-10-05/STREAMS.md` row 105 and the worklet host row 108: rebase
  after those.

## Supersedes and shrinks

- None.

## Evidence and decision record (attempt 1, 2026-10-25)

Base `f4d2b7862` (main with #1579a, #1579b and #1597). The spec's line anchors were stale; the
symbol names held.

### Deleted public surface

- **Exports (12):** `miso_engine_web_v1_track_response_{request_ptr, request_bytes, snapshot_ptr,
  snapshot_capacity, snapshot_set_bytes, track_id_ptr, track_id_capacity, capture, analysis,
  result_ptr, result_bytes, close}`. `abi_layout::EXPORTS` goes from 77 to 65.
- **host-web (Rust):** `WebLiveResponseRequest`, `WebLiveResponseOwner`, `WebLiveResponseSection`,
  `WebLiveResponseResult`; `LIVE_RESPONSE_{MAXIMUM_OWNERS, MAXIMUM_SECTIONS, MAXIMUM_ID_BYTES,
  MAXIMUM_POINTS, CAPTURE_BYTES, MODE_TARGET, MEANING_EQ_FILTER_SUBTOTAL, REQUEST_BYTES,
  OWNER_BYTES, SECTION_BYTES, RESULT_BYTES, RACK_INPUT_FILTERS, RACK_INSERTS, RACK_CONSOLE}`;
  `RESPONSE_GRID_{LINEAR, LOGARITHMIC}`, `RESPONSE_CHANNEL_{LEFT, RIGHT, BOTH}` (no reader was
  left after #1574); `AudioWorkletEngineHost::copy_response_snapshot`;
  `native_staging::track_response_request`. Internal: `ResponseStaging` and its accessors, the
  two boot-budget terms, the capture sink, the rack maps, the snapshot parser, the analysis and
  the record copier. The test-only `live_response_ffi_tests` module keeps only its counting
  allocator, renamed `allocation_counter`.
- **Worklet host (JS and both `.d.ts` twins):** `captureTrackResponse`, `MisoTrackResponseGrid`,
  `MisoTrackResponseChannels`, `MisoTrackResponseCaptureRequest`,
  `MisoTrackResponseCaptureReply`, the `trackResponse` pending kind. Worklet: the
  `miso.trackresponse.v1` message, `receiveTrackResponse` and its boot views.
- **ABI layout (regenerated):** structures `liveResponseRequest`, `liveResponseOwner`,
  `liveResponseSection`, `liveResponseResult`; constants `liveResponseRacks`, `responseGrids`,
  `responseChannels`, `liveResponseModes`, `liveResponseMeanings`, `maximumLiveResponseOwners`,
  `maximumLiveResponseSections`, `maximumLiveResponseIdBytes`, `maximumLiveResponsePoints`,
  `liveResponseCaptureBytes`. `sdk/src/generated/abi.ts` loses the same rows; the SDK's
  `constantValue` group union loses `"liveResponseRacks"`. `maximumResponsePoints` stays (the
  SDK's response bounds read it). No wire tag, schema hash, C ABI or `ABI_VERSION` moves.

### D2: the replacement row

`runStagingReadRun` sends one strip `RESPONSE_GET` for `track` through `host.control()` before the
offline context renders (`requestTrackResponseThroughSdk` in `sdk-response-entry.ts`, the SDK's
encoder, so `qualification.js` holds no encoder), and reads the reliable lane's `RESPONSE_READY`
events after the render. New gate `track-response-host`: control result 0, reply `ok`, exactly
one `RESPONSE_READY`, carrying the reply's tag, outcome 0 (`ok`). Mutations
`track-response-host-outcome` and `track-response-host-tag` name it in run.mjs's #2040 table. The
`staging-reads` gate keeps its observation half; its mutation is unchanged. Red first: with the
request's strip set to `absent`, chromium fails `track-response-host` (`replyStatus:
"not_found"`, no event). The render that answers the request is inside the run's
`render-allocations` row (`staging-reads=0` in all three browsers).

### D3: boot (PR evidence, not a committed count)

The observation fixture session booted on the shipped module: `bridgeRetainedBytes` 1,662,971 on
the base, 614,220 here (1,048,751 = `1 << 20` + 175 less); memory after boot 3,080,192 to
2,031,616 bytes; `largestBridgeAllocationBytes` 1,048,576 to 29,274. The boot budget check
passes.

### Deleted tests and their holders

| Deleted | Its claim now held by |
|---|---|
| `ffi::live_response_ffi_tests::ffi_live_capture_analysis_is_prewarmed_and_token_exhaustion_is_typed` | Feature gone (the staging, its token and the browser analysis). The core capture's allocation-free render step: `control-plane/tests/response_capture.rs::render_step_allocates_nothing`; in the browser, `track-response-host` with `render-allocations`. |
| `ffi::live_response_ffi_tests::live_response_owners_name_console_slots_inserts_and_input_filters` | Feature gone (the owner record and its rack codes). The core names members by section: `host-core/tests/track_response.rs::the_composer_names_every_member_in_signal_order_and_keeps_the_curves`. |
| `tests.rs` `gap_free_acceptance`: its last check (a capture after the swap is stamped with the continued clock) | `engine` `realtime::tests::synchronous_hand_over_continues_state_and_clock` (`captured_sample` after a carry). |
| `tests/render_locked_staging.rs`: the six response accessor touches and the staged capture | Feature gone; the observation half stays. |
| `check-abi-layout-v1.py --self-test` mutation "a retired owner-record rack code returns" | Feature gone (the table is deleted). 37 mutations remain. |
| qualification `staging-reads` predicate `trackResponseResult === 0 && trackResponseBytes > 0` | `track-response-host` (D2). |

Rewritten: `sdk/test/browser-evals.mjs` "generated u64 offsets preserve BigInt" reads
`planWatermark.revision` and `firstSample` instead of the two deleted `liveResponseResult` words.

### Paths outside the authorized list

Mirrors that named the deleted code (as #1597's slice did): `hosts/host-web/src/render_lock.rs`
(header export list), `docs/REALTIME_DEPENDENCY_POLICY.md` (unsafe-site and export lists),
`hosts/host-web/MUTATIONS.md` (the new mutations), `scripts/test-web-audioworklet.mjs` (the fake
module's deleted exports), `sdk/src/core/abi.ts` (the union member, needed to type-check),
`sdk/test/browser-evals.mjs` (above), and `hosts/host-web/qualification/sdk-response-entry.ts`
(the D2 helper).
