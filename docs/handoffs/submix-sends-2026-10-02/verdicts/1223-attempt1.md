# #1223 attempt 1 verdict (Sol): `932f348a8` (base `466f0ab63`), branch `codex/batch-submix-k3`

**PASS.** No BLOCKER and no MAJOR. Two MINORs and three NITs follow; none of them blocks the slice.

- All three browser qualification legs pass in CI's exact mode: SDK source bundle, a fresh
  `--named-twin` build, and `--check-matrix --self-test-mutations`. This batch has no repeat of
  K2's BLOCKER-1.
- I worked only on an export of `932f348a8` under `/tmp/claude-1002/v1223/` (now deleted). The
  worktree was not touched. Nothing was committed or pushed, and nothing was edited on GitHub.

## Browser legs (the K2 failure class)

The command is the one in `.github/workflows/qualification.yml`:
`npm run qualify -- --artifacts <A> --sdk-root <export>/sdk --browser <b> --check-matrix
--self-test-mutations`.

- Each leg had its own private pulseaudio null sink, as in CI.
- `<A>` came from a fresh `build-web-audioworklet.sh --named-twin`.
- The export has no `sdk/dist`, so the harness reported "sdk bundle: the source at ... (CI's mode)".

| browser | rc | result |
|---|---|---|
| chromium 151.0.7922.34 | 0 | all qualification gates passed |
| firefox 153.0 | 0 | all qualification gates passed |
| webkit 26.5 | 0 | all qualification gates passed |

Why this slice cannot repeat K2:

- `SessionShape`, the type the seven `scratchBoot` stubs in
  `hosts/host-web/qualification/sdk-response-entry.ts` return, is unchanged here.
- `routes` was added only to `SessionMap` and `MisoSessionMap`.
- The only code that reads a host's `routes` is `createBrowserLiveControls`, and the harness
  reaches it through the real worklet, whose reply now carries `routes`.
- `qualification.js` reads `map.tracks` and `map.metersAttached` only.

## Gates reproduced on `932f348a8` (x86-64-v3)

Every gate below returned rc 0.

**Artifacts.** The build reproduced the record's digests:

- shipped module `97a758d8fa5d274a2c2e1603d768101632f55b5a24ddc99fec22067d88be7d23` (2,763,281 B);
- named twin `9e75ffa1…` (3,153,509 B).

**SDK:**

- `check-sdk-types.sh`;
- `check-sdk-generated.sh A`;
- `check-sdk-headless.sh A`: 355 pass, 0 fail;
- `sdk-package.sh check A`: the publishable-tarball gate passed.

**Web:**

- `check-web-audioworklet.sh A B/…named.wasm`, with metadata regeneration. The export list grows
  by exactly the two names.
- `test-web-audioworklet.sh`.
- `check-abi-layout-v1.py --self-test` (22 caught) and on the layout JSON.
- `check-session-map-shape.py --self-test` (20 caught) and plain.
- `check-browser-expected-resources.py --artifacts A` (32 red mutations; `idStagingBytes` did not
  move).

**Workspace:**

- test-debug-a, the exact CI command with `--no-fail-fast`: 108 binaries, 1,222 passed, 0 failed,
  9 ignored. This matches the record.
- `cargo fmt --all -- --check`.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`.
- `check-host-core-policy.sh`, `test-host-core-policy.sh` and `check-workspace-policy.sh`.

## Adversarial checks

**Export order equals send-index order.**

- `copy_live_control_route_id` and `live_control_route_count` read `ReadyOwnership::route_controls`.
- That is the same vector admission indexes: `ready.route_slot(track)`,
  `ready.route_controls.get(track)` and `ready.routes.get(track)` in `admit_commands`, and
  `ReadyOwnership::push`.
- Preparation already refuses a plan whose producer order differs, by ID, from
  `LiveRouteState::live_routes(model)` (`web.live_controls.routes`).
- Only sends are listed, never output routes. A host without live controls reports 0.
- On the shipped module, a probe session with a bus-to-bus send enumerated
  `["aaa-to-verb","kick-verb","vox-verb"]`, and every edit was admitted at its listed index.

**SDK encoding.**

- `RouteEdits` emits kinds 13, 14 and 15 with rack and channel 255 (`NONE`), effectIndex and
  parameterId 0, and `values` `[db,0,0,0]`, `[on?1:0,0,0,0]` and `[ll,lr,rl,rr]`.
- Gate 1 pins the browser request words literally, with `trackIndex: 1` written as a literal
  rather than derived.

**Refusals.**

- An unknown ID, and a track ID, throw `MisoUsageError` with diagnostic code `unknownRoute`, and
  the message lists the live routes.
- With a session, an output route gets the "output routes are not live" message.
- Without a session, an output route gets the listing message, as D4 requires.

**`kindNames`.**

- The `kindsAwaitingSdk` exception is gone. The hand list must equal
  `ABI_LAYOUT.constants.wireCommandKinds` exactly, so a future wire kind without an SDK entry turns
  the test red.
- The new title is accurate: the send kinds are admitted by live Wasm in the #1223 evals, not in
  this one-track session.
- No stale reference to the exception is left in a live file.

**`routes` is consistent across every spelling.**

- These all carry `routes`, in matching form:
  - the worklet reply;
  - the host's exact field list and its validator (array of non-empty strings);
  - `MisoSessionMap` in both `.d.ts` copies (`cmp`-identical);
  - the SDK `SessionMap` (`boundary.ts` reads the exports in engine order; the browser map copies
    the reply);
  - the shape checker's three new mutations.
- The SDK's `SessionMap` is held by `tsc`, because `routes` is required.

**Paths and records.**

- Every touched file is in Authorized paths.
- The three deviations are justified:
  - The host `.d.ts` has no export list.
  - `qualification.js` spells no field list.
  - The `sed` mutation is still a substring of the grown list.

## Mutations

These are my own. Each was applied to a scratch copy, run, and restored.

| # | mutation | result |
|---|---|---|
| V1 | the browser map reverses `remoteMap.routes` | gate 1 red |
| V2 | `RouteEdits.matrix` writes `ll, lr, rr, rl` | gate 1 red |
| V3 | the output-route filter matches `submix_input` | gate 2 red |
| V4 | `RouteEdits.mute` writes `on ? 0 : 1` (that method only) | gate 1 red |
| W1 | the worklet reads route `routeCount-1-index` | `test-web-audioworklet.mjs` red ("the enumerated live-route order") |
| R1 | the host enumerates `route_controls` reversed | gate 4 native test red (live route 0: 63 vs 6) |
| S1 | `longest_route_id_bytes` skips IDs of 32 bytes or more | `the_session_shape_measures_every_route_id` red (10 vs 39) |
| E1 | the module is rebuilt so that `push` targets `route ^ 1` (the record's unique-catch claim) | only gate 3 red; 11 others pass. Confirmed |
| T1 | `RouteEdits.mute` takes `LaneOptions` | **green** (MINOR-1) |
| W2 | the worklet drops the ASCII-byte check in the route reader | **green** (NIT-1) |

## Test value (one sentence each)

- **`live_route_ids_enumerate_in_send_index_order_through_staging_sized_for_them`:** red if ID
  staging ignores route IDs, or if the route-ID export enumerates any order or subset other than
  the live send producers (R1).
- **`the_session_shape_measures_every_route_id`:** red if `longest_route_id_bytes` skips some route
  IDs (S1).
- **Gate 1 eval:** red if the browser map reorders the engine's list, or if a send record's matrix
  words or mute polarity are written otherwise (V1, V2, V4).
- **Gate 2 eval:** red if the output-route reason is lost or an unplaceable ID gets an index (V3).
- **Gate 3 eval:** red if admission and enumeration disagree on which send an index names, on the
  shipped module. This is its unique catch (E1).
- **`live-controls-types.ts`:** red if `RouteEdits` gains a strip method or `gainDb` takes a lane.
  Not red for `mute` or `matrix`: see MINOR-1.
- **`test-web-audioworklet.mjs`:** red if the worklet reorders the engine's routes (W1), or if the
  host accepts a malformed route list.

## MINOR-1: the type probe's "a send gains a lane option" claim covers `gainDb` only

- **Problem.** T1 shows that `mute(on, options: LaneOptions)` stays green, and `matrix` is
  unprobed too.
  - At runtime a `channel` key is silently ignored, so a JavaScript caller expecting a one-lane
    send edit gets a both-lane edit.
  - The record (and `hosts/host-web/MUTATIONS.md`) says "Red if a send gains a lane option".
- **Fix.** Add these to `sdk/test/live-controls-types.ts`:
  ```ts
  // @ts-expect-error a send has no lane
  send.mute(true, { channel: "left" });
  // @ts-expect-error a send has no lane
  send.matrix({ ll: 1, lr: 0, rl: 0, rr: 1 }, { channel: "left" });
  ```
  Alternatively, narrow the record's claim to `gainDb`.

## MINOR-2: `APP-LIVE.md` does not say that `sessionMap()` now requires `routes`

- **Problem.**
  - `createBrowserLiveControls` spreads `remoteMap.routes`. A fake host whose `sessionMap()`
    omits it throws `TypeError: … not iterable` at live-control construction, even in an app that
    never uses sends.
  - The app's `src/lib/mixer/engine/fake-host.ts` builds its map `as MisoSessionMap`, so
    TypeScript will not flag the omission at the SDK bump.
  - This is the downstream twin of K2's BLOCKER-1 and NIT-4.
- **Fix.** Add one sentence to "Live sends": `MisoSessionMap`/`SessionMap` gained a required
  `routes` (`[]` with no send), and a test fake of `sessionMap()` must return it. The same
  sentence can cover `submixes`.

## NITs

1. **The ASCII-byte check is untested (W2).** Removing it from the worklet's route reader stays
   green. The track and submix readers have the same pre-existing gap, so a corrupt non-ASCII ID
   byte is never tested.
   - Optional: one fake-export row that returns a byte above `0x7f`, covering all three readers.
2. **No real-browser leg enumerates a non-empty `routes`.** The qualification sessions have no
   submix, so the real worklet reads route count 0.
   - The worklet reader is covered only by fake exports, and the real export only through the
     headless `WasmBoundary`.
   - K2's `submixes` reader is in the same position.
   - Candidate successor: give one qualification session a send, and drive it from the SDK in the
     browser leg.
3. **Two added lines are 103 columns.**
   - Worklet comment line 615: "…track, submix or route ID, so the capacity check below is a".
   - Host JS line 992: the field list.
   - Rewrap at the next touch. Both files ship, so this is an honest artifact change.

## INFO

- **The live-route domain is the open fold of #1215.** The probe on the shipped module showed:
  - `gainDb(1000)` and `gainDb(1e30)` are refused as `domain`;
  - `matrix ll 1e30` and `matrix ll 5` are admitted;
  - `gainDb(-1e30)` is admitted.
- This matches the session grammar (`validate_finite`). The handoff's "refuses a value outside it
  with `domain`" is accurate.
- The owner's Q2 bounds ([-144, 24] dB, [-1, 1]) are a later slice.
- Strip edits pre-check catalog domains in the SDK (`builtinNumber`), but route edits only check
  `finite`. That is expected until the route parameter-metadata family exists (non-goal, O11).
