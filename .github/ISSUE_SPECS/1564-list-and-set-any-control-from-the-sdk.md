# List and set any control from the SDK

Slice S11b of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1,
C4 and C5; "How to add a control"). Code anchors verified on `main` `df585e503`. Feature sprint
mode (owner, 2026-10-13): one implementer, normal tests, a batch review; no per-attempt verifier or
mutation runs.

## Product outcome

A browser or headless app lists every control of the running session with `engine.controls()`,
reads their values with `engine.controlState()`, and sets any of them with one call,
`edit.set(address, value, { rampSamples })`. The typed builders (`faderDb`, `gainDb`, an effect's
`parameter`, `bypass`, `solo` and the rest) are generated from the core's tables and are sugar over
`set`, so a new row in the core reaches the SDK by regenerating, with no hand-written code. The SDK
reads the strip order from the control list, not from host-web's index exports.

## Context

- **The list.** #1551 serves `CONTROL_LIST_GET` (`0x000c`) and `CONTROL_STATE_GET` (`0x000d`) in the
  TLV form of ruling C4: pages with `revision`, `structure_epoch`, `last_handle`, `eof`; header
  rows per owner and section; control rows with the address, the descriptor fields and the value.
  A stale `structure_epoch` is refused `control.list.stale`.
- **The path to the core.** #1562's `control` message (browser) and `miso_engine_web_v1_control_*`
  exports (headless); #1563's per-engine transaction queue and frame codec
  (`sdk/src/core/control-frame.ts`).
- **The catalog.** #1550 generates `ControlAddress`, the row-name unions and typed row metadata into
  `sdk/src/generated/` with `sdk/codegen/generate.mjs`, from the metadata JSON that
  `tools/parameter-metadata` emits.
- **The builders** are hand-written in `sdk/src/core/live-controls.ts` (`TrackEdits` `:757`,
  `SubmixEdits` `:774`, `RouteEdits` `:787`, `VcaEdits` `:831`, `EffectEdits` `:877`); after #1563
  each returns `ControlEdit`s.
- **Strip order.** `WebBoundary.shape()` (`sdk/src/core/boundary.ts:474-510`) reads tracks and
  submixes through `miso_engine_web_v1_live_control_{track,submix}_{count,id}`; the browser's
  `host.sessionMap()` reads the same exports in the worklet
  (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:597-660`). Meters and observation still
  address strips by index. The list's owner header rows come in `model.strips()` order (ruling C4),
  which is that index.
- **The cross-target digest.** `hosts/host-web/tests/browser-v1/direct-oracle.mjs` drives the raw
  `simd128` module and digests its PCM; `scripts/check-browser-expected-resources.py` checks it
  against `hosts/host-web/tests/browser-v1/expected.json`. It has a command-record timeline today
  and no `SET_CONTROL`.

## Decisions frozen for this slice

- **D1. `engine.controls(filter?)`.** On the browser engine and the headless engine. It sends
  `CONTROL_LIST_GET` pages (limit 256) through #1563's queue path until `eof`, and resolves with
  `ControlList { revision: bigint, structureEpoch: bigint, headers: ControlHeader[], controls:
  ControlRow[] }`. The field names and types mirror ruling C4's wire fields one to one (camel case).
  `filter` is `{ owner?: { kind: "strip" | "route" | "vca" | "session", id?: string } }`. A
  `control.list.stale` page restarts from the first page; after 3 restarts the call rejects with
  `MisoEngineError` code `control.list.stale`.
- **D2. `engine.controlState(list)`.** Sends `CONTROL_STATE_GET` with `list.structureEpoch` and
  resolves with `{ revision, values: Map<handle, number> }`. A stale epoch rejects with
  `control.list.stale`; the caller lists again.
- **D3. `edit.set`.** `engine.liveControls().edit.set(address: ControlAddress, value: number,
  options?: { rampSamples?: number })` stages one `ControlEdit` in the same transaction as the
  typed builders. It checks only encodability (#1563 D3).
- **D4. Generated builders.** `generate.mjs` emits `sdk/src/generated/control-builders.ts`: one
  method per row name per section kind (strip own and builtins, route, VCA, session, each effect's
  parameters and bypass), with the names and argument shapes #1563 left, each calling `set` with
  the row's address. The hand-written method bodies in `live-controls.ts` are deleted; the classes
  keep only owner selection (`track(id)`, `submix(id)`, `route(id)`, `vca(id)`, `console(slot)`,
  `insert(id)`). `scripts/check-sdk-generated.sh` already holds generated files to the generator.
- **D5. Strip order from the list.** `shape().tracks` and `.submixes` and the browser's session map
  come from the owner header rows of `engine.controls()` (kind track, then kind submix, in list
  order), read at boot and after every reply whose `structure_epoch` differs from the last one
  read. The SDK calls no `miso_engine_web_v1_live_control_*` export and reads no route or VCA
  order.
- **D6. One control script in the cross-target digest.** `direct-oracle.mjs` gains one timeline
  that sends three `SESSION_TRANSACTION_APPLY` frames of `SET_CONTROL` edits (a fader, a pan and an
  effect parameter of the fixture session, the frames committed as test fixtures) through the
  `miso_engine_web_v1_control_*` exports and digests the PCM. `expected.json` pins the new digest;
  the native leg of the #106 F4 session parity gate (`hosts/host-web/src/tests.rs:1625`) renders
  the same timeline natively and must
  match it.

## Deliverables

1. D1-D3 in `sdk/src/core/` (a list codec beside `control-frame.ts`) and both engines.
2. D4 in `sdk/codegen/generate.mjs` and `sdk/src/generated/`; the hand-written bodies deleted.
3. D5 in `sdk/src/core/boundary.ts`, `sdk/src/browser/{live-controls.ts,engine.ts}`.
4. D6 in `hosts/host-web/tests/browser-v1/` and the native parity leg.
5. The SDK reference docs for the three new calls (`sdk/README.md` or its docs page).

## Authorized paths

- `sdk/src/**`, `sdk/codegen/generate.mjs`, `sdk/test/**`, `sdk/README.md`.
- `hosts/host-web/tests/browser-v1/{direct-oracle.mjs,expected.json}` and new fixture frames there;
  the native F4 parity test in `hosts/host-web/src/tests.rs` (its timeline only).
- `scripts/check-browser-expected-resources.py` (the new timeline's schema row only).

## Non-goals

- No change to host-web's Rust or the worklet (#1565 deletes the exports this slice stops using).
- No structural SDK transactions (#1296, #1385).
- No list caching beyond D5's strip order; an app lists when it wants to.

## Objective gates

Commands: `bash scripts/check-sdk-types.sh`,
`bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`,
`bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
`bash scripts/sdk-package.sh check target/ci/qualification-artifacts`,
`python3 -B scripts/check-browser-expected-resources.py --artifacts target/ci/qualification-artifacts`,
`cargo test --locked -p host-web --features host-web/test-support`, and the browser qualification
for chromium, firefox and webkit.

1. **The list is the core's.** On the headless engine, for the #1560 parity session,
   `engine.controls()` returns exactly the rows the native `controls::list` yields (address, class,
   value, order), checked by an eval against a fixture the Rust side writes. Paging with limit 1 and
   256 gives the same list.
2. **Stale and restart.** A structural transaction between two pages makes the SDK restart and
   return the new list; a test transport that always answers stale rejects after 3 restarts.
3. **Sugar is `set`.** For every generated builder, the staged `ControlEdit` equals the one `set`
   stages for the same address and value (generated eval over every row of the catalog).
4. **Set any row.** `edit.set` on one row of each owner kind and section renders as the typed
   builder does, and `edit.set` on a derived row rejects `control.read_only`.
5. **Strip order.** With the `live_control_*` exports removed from a test copy of the export
   object, boot, meters and observation work on the headless engine.
6. **Cross-target digest.** `check-browser-expected-resources.py --artifacts` passes with D6's
   timeline on `simd128`, and the native leg matches it.

## Test value

- Gate 1 turns red if the SDK decodes a wire field wrong or drops rows.
- Gate 2 turns red if the SDK mixes handles of two structures or loops forever.
- Gate 3 turns red if a hand-written or generated builder drifts from the address its row names.
- Gate 4 turns red if a row has no SDK path or the SDK refuses on its own.
- Gate 5 turns red if the SDK still depends on host-web's index exports.
- Gate 6 turns red if `SET_CONTROL` renders differently on the shipped `simd128` module than
  natively.

## Dependencies

- #1563, #1562, #1551, #1549, #1550.

## Supersedes and shrinks

- Shrinks *Declare a strip's console link mode in the session, the wire and the SDK* (#1369): its
  SDK half becomes a generated builder of the link-mode row, once that row exists.
- Shrinks *Let a strip override a console slot's link mode* (#1236): its wire and SDK halves become
  one row; the session grammar and render work stay.

## Amendment (decision 17, root 2026-10-13)

**Name.** D1's list-row type is `ControlListEntry`, not `ControlRow`: `ControlRow` is already the
generated catalog type (`sdk/src/generated/controls.ts:4313`). `ControlList.controls` is
`ControlListEntry[]`.

**Split.** All three pieces stay in this issue.

- **(A)** D1-D3 and the list codec: `engine.controls()`, `engine.controlState()`, `edit.set`.
  Gates 1, 2 and 4. Dependencies: #1563B, #1562a, #1551, #1549, #1550.
- **(B)** D4, the generated builders. Gate 3. Dependency: (A).
- **(C)** D5 and D6: strip order from the list, the `direct-oracle.mjs` timeline and the native
  parity leg. Gates 5 and 6. Dependency: (A). #1565B needs (C).

## Amendment (controls coordinator, batch 8, 2026-10-14)

Checked against `main` at `32cee58d0`.

- **Dependencies.** #1549, #1550, #1551 and #1563 are closed. #1563A landed in batch 5
  (`d3b364446`); #1563B and #1562a landed in the flip, batch 7 (`0c6fdff1a`, `365482eb8`). #1562b is
  in the blocked carry chain. No piece of this issue needs it (gates 2 and 4 below).
- **Anchors.** The builders in `sdk/src/core/live-controls.ts` are now `TrackEdits` `:693`,
  `SubmixEdits` `:715`, `RouteEdits` `:726`, `VcaEdits` `:775` and `EffectEdits` `:805`; the
  engine's entry is `liveControls().edit` (`:944`). `WebBoundary.shape()` is
  `sdk/src/core/boundary.ts:457`; it reads the index exports at `:473-482`. The worklet reads the
  strip order at `hosts/host-web/web/miso-engine-v1-audio-worklet.js:611-645` and answers
  `miso.sessionmap.v1` at `:1024-1038`. The browser SDK reads that map at
  `sdk/src/browser/live-controls.ts:98`. The #106 F4 native leg is `hosts/host-web/src/tests.rs:1504`.
- **Name.** #1563B left no `ControlEdit` type. One edit is `ControlFrameEdit`
  (`sdk/src/core/control-frame.ts:63`: wire address, value, `rampSamples`). A builder returns a
  `ControlEditSet` (`live-controls.ts:134`). D3 and gate 3 read `ControlFrameEdit`. `edit.set`
  converts the generated `ControlAddress` to the `ControlWireAddress` of that type.
- **D1 path.** The list and state reads are not transactions. They use the endpoint's transport
  and its one request-ID allocator (`ControlRequestIds`, `sdk/src/core/control-queue.ts:83`). They
  do not go through the batching of `ControlTransactionQueue`, which merges edits.
- **D6 is done; #1564C shrinks.** #1565A (`5576cdc56`) replaced the command-record timeline of
  `direct-oracle.mjs` with `runSetControlTimeline`. It sends four committed frames
  (`hosts/host-web/tests/browser-v1/set-control-frames/frame-{1,2,3,4}.bin`: a fader, a pan, the EQ
  band 1 gain and one refused out-of-domain fader value). The native twin is
  `tests::native_set_control_timeline_digest_pins_the_wasm_parity`, and
  `tests::set_control_frames_are_the_committed_fixtures` holds the frames to their encoder.
  `expected.json` has `setControlTimeline`, and `check-browser-expected-resources.py` checks it
  (`:123`, `:585`). Gate 6 is met on `main`: PR #1614's chromium, firefox and webkit legs passed.
  #1564C keeps D5 and gate 5 only. Its authorized paths lose `direct-oracle.mjs`, `expected.json`,
  the F4 test and `check-browser-expected-resources.py`.
- **Gate 2.** Both SDK engines run the host-web module, which refuses a rebuild-path frame with
  `web.apply.rebuild_pending` until #1562b. So "a structural transaction" reads "a transaction that
  advances `structure_epoch`". Use a live `matrix_mode` switch (it changes descriptors) or a session
  ramp-length change. Both advance the epoch without a rebuild
  (`replace_session_catalog`, `crates/host-core/src/control_provider.rs:452-458`). The claim of the
  gate does not change.
- **Gate 4.** Use a live row for each owner kind and section. A route into the output is
  rebuild-only, and the browser refuses it until #1562b
  (`hosts/host-web/tests/control_parity.rs:685-688`): use a send into a submix. Add one case:
  `edit.set` on a rebuild-only row (for example `delay_samples`) rejects with diagnostic code
  `web.apply.rebuild_pending`, so the SDK passes the core's refusal through and does not refuse on
  its own.
- **#1565B** still needs (C): it deletes the `live_control_{track,submix}_{count,id}` exports after
  D5.

## Decision record: piece A, list and set (batch 8)

**Shipped.** `engine.controls(filter?)` and `engine.controlState(list, options?)` on the headless
engine and on `BrowserEngine`, and `liveControls().edit.set(address, value, { rampSamples? })`.
The codec is `sdk/src/core/control-list.ts` (request encoder, page decoders, `readControlList`,
`readControlState`); it reuses the frame readers of `control-frame.ts`, which now exports them
(`@internal`) together with a `decodeResponseEnvelope` / `decodeRefusalBody` pair that
`decodeControlReply` is built on and one `commandFrame` builder that both existing encoders use.
`src/index.ts` exports `control-frame.ts` by name now (it was `export *`), so the internal readers
do not reach the barrel. The list and state reads use the endpoint's transport and its one
`ControlRequestIds`, not the batching queue. The browser transport is
`createBrowserControlListTransport(host)`, which shares the queue's allocator.

**Departures from the text.**

- `ControlList` also carries `owner`, the filter the list was read with, so `controlState(list)`
  reads the same rows (a state request takes the same owner filter). `ControlListFilter` carries
  `pageLimit` (1 to 256, default 256): gate 1 needs limit 1 and 256 from the public call.
- `revision` of a list and of a state is the revision of the first page (revisions only grow, so it
  is the oldest value read).
- Other refusals of a page (not `control.list.stale`) reject with `MisoControlRefusal`; stale
  rejects with `MisoEngineError`, `code: "control.list.stale"`, as the spec says. The error-code
  union gained `SdkControlCode = "control.list.stale"`.
- `edit.set` returns a `ControlEditSet` (what the builders return), per the batch-8 amendment. It
  accepts a generated `ControlAddress` or a `ControlWireAddress` (a list entry's `address`). A row
  of an insert or console slot named by machine name needs the effect it runs, which only a session
  the SDK built tells the SDK (`withSession()`); otherwise it throws `MisoUsageError` that points to
  the numeric address. This is encodability, not a core decision: the name has no numeric ID
  without the effect.
- The Rust fixture writer is a new test outside the spec's authorized paths:
  `hosts/host-web/tests/control_list_fixture.rs` writes `hosts/host-web/tests/fixtures/control-list.json`
  (canonical parity session at 48 kHz, 397 native rows with address, class, descriptor facts and
  value bits). It lists the parse of the canonical document, not the builder's model: the booted
  model has its routes in ID order, and a first draft that listed the builder's model was red on
  route order. Rewrite with `MISO_ENGINE_BLESS_FIXTURES=1`.

**Finding for later slices.** The core takes request IDs in ascending order per endpoint
(`replay_expired`, diagnostic `protocol.failure`, for a lower one) and refuses a reused one. Two SDK
allocators on one core therefore collide (a second `ControlRequestIds` starts at 1 again): one
endpoint, one allocator. Tests that change the structure mid-read do it inside `requestIds.next()`.

**Gates and tests** (`sdk/test/control-list-evals.mjs`, 31 tests):

- Gate 1: `headless: engine.controls() yields the rows of the native controls::list, in order`
  (397 rows against the fixture), `paging with limit 1 and with 256 gives the same list` (also 7 and
  the default), `the headers open every owner...`, `an owner filter...`, `a row's descriptor is
  decoded from the wire...`, `the browser transport lists the same rows as the headless one, through
  a real BrowserEngine`. Rust: `control_list_fixture::the_control_list_fixture_is_the_native_list_of_the_parity_session`.
- Gate 2: `a transaction that advances the structure epoch makes the read restart and return the new
  list` (a live `matrix_mode` switch of `t3` between pages 1 and 2), `a transport that is stale on
  every continuation rejects after three restarts` (4 first pages, then `control.list.stale`),
  `a state read of a list from another structure rejects control.list.stale`, `a reply that makes no
  progress ends the read instead of looping`.
- Gate 4: `edit.set reaches any row`: a strip builtin (fader by name, one-lane mute, fixed-ramp
  high-pass), a strip own row (solo, `matrix_mode`), a route own row (`t1-b` gain), a VCA own row,
  the session's own `fader_ms`, an insert parameter, a console parameter and an insert bypass.
  Each builder-backed case renders bit-identically to the typed builder on a second core and
  leaves an identical list. `headless` and `browser`: `set on a derived row rejects control.read_only`;
  `set on a rebuild-only row passes the core's web.apply.rebuild_pending through` (delay_samples,
  nothing committed). Also: every live row of the list accepts a write of its own value; an
  out-of-domain value reaches the core (`control.domain`) and an unknown strip is
  `control.address.unknown`; `set` refuses only unencodable input.
- Run: `bash scripts/check-sdk-types.sh`, `check-sdk-headless.sh` (493 tests, 492 pass, 1 todo from
  before), `check-sdk-generated.sh`, `sdk-package.sh check`, `check-sdk-deletions.py`,
  `check-workspace-policy.sh`, `cargo clippy -p host-web --all-targets --all-features -D warnings`,
  the Rust fixture test; all with the artifacts of this tree.

**Left for B and C.** D4 (generated builders; `set` is the call they use, and the effect parameter
names of the control tables, such as `threshold_db`, differ from the catalog's builder names, such
as `threshold`), D5 (strip order from `controls().headers`: owner headers of kind `track`, then
`submix`, in list order, and a re-read when a reply's `structureEpoch` differs).

## Decision record: piece B, generated builders (batch 8)

**Shipped.** `sdk/codegen/generate.mjs` emits `sdk/src/generated/control-builders.ts`: `StripEdits`,
`TrackEdits`, `SubmixEdits`, `RouteEdits`, `VcaEdits`, `SessionEdits` (one method per settable row of
the strip, route, VCA and session tables) and `EffectControlBuilders<E>` (`parameter`, `bypass`,
`linkMode`), plus `CONTROL_BUILDERS` (a manifest of every builder and the rows it writes) and the
effect row tables. Every method builds the row's address and calls `set` (the engine's
`LiveControlEdits.set`, reached through a `ControlSetter`), so a builder stages what `edit.set`
stages. The hand-written bodies in `live-controls.ts` are gone (1174 to about 650 lines): it keeps
`LiveControlEdits` (owner selection, `set`, the row-name resolver), `EffectEdits` (observation and
the instance's section; it extends the generated class), the layout checks and `EngineLiveControls`.
The strip's `effect()`, `console()` and `insert()` addressing moved to `StripAddressing` in the new
`sdk/src/core/control-builder-support.ts` (also the option and edit types and the value helpers), a
base the generated `StripEdits` extends; `LiveControlEdits` hands each strip a `StripContext`
(`set` and an effect factory), which breaks the runtime import cycle.

**One source of truth.** Addresses come from the core's control tables (`controls`); a row is named
by its machine name there (`threshold_db`). The catalog's parameter names (`threshold`) stay the
keys of `parameter()`, so the public names are stable. The generator joins the two documents by the
numeric ID they share and throws if a catalog parameter has no control row, a different display name,
a different value domain or different enumeration choices. An effect instance knows its effect, so
`EffectEdits` addresses a row by its number (`effectControlId`) and needs no session; the named form
of `set` still needs `withSession()`, as in piece A.

**What the generator decides** (the public names #1563 left; each is checked against the tables and
throws on a missing row): `METHOD_NAMES` (the VCA's `offset_db` is `faderDb`), `TRACK_ONLY` (`solo`:
a bus is solo-safe), and `COMPOSITES` (`pan`, `matrix`, `inputFilters`, each several rows in one
transaction). Every other method is the camel case of the row name. The existing surface is
unchanged (`RouteEdits` and `VcaEdits` keep exactly their `keyof` the type probes pin).

**Departures from the text.**

- New methods, because the tables have the rows: strip `matrixMode`, `panWindowSamples`,
  `delaySamples` (rebuild-only; the core refuses it), `edit.session()` with `faderMs`, `muteMs`,
  `panMs`, and `linkMode` on an effect instance (an insert only: a console slot has no link-mode row
  and the core refuses it; an effect with no choice of it has `never` as the label type).
- A derived row has no builder (`set` refuses it with `control.read_only`).
- A per-row option shape follows the row: per-lane with a ramp takes `channel` and
  `smoothingSamples`, per-lane with a fixed ramp takes `channel`, shared with a ramp takes
  `smoothingSamples`, shared with a fixed ramp takes none. `InputFilterOptions` is kept as an alias
  of the new `ChannelOptions`.
- The public constructors of `TrackEdits`, `SubmixEdits`, `RouteEdits`, `VcaEdits` and `EffectEdits`
  take the `set` function (and the strip's context); only `LiveControlEdits` constructs them.
- Gate 3 is a hand-written eval over the generated manifest and the tables, not a generated test
  file: a row without a builder, or a builder that writes another row, lane or ramp, fails it.

**Gates and tests** (`sdk/test/control-builders-evals.mjs`, 28 tests): `every settable row of the
control tables has a builder, and no builder names a missing row`; one test per builder (`<owner>
<method> (<rows>) is set over its rows`, every option bag and every value); `a track offers solo and
a submix does not`; `every parameter, bypass and link mode row of every effect has a generated
builder row`; `inserts:` and `console: parameter, bypass and link mode stage what set stages`
(every parameter of every effect, positional and object form, four option bags). Red check: changing
`trimDb`'s lane in the generated file fails `strip trimDb`. Type probes added to
`sdk/test/live-controls-types.ts`. Run, with the artifacts of piece A: `check-sdk-types.sh`,
`check-sdk-generated.sh`, `check-sdk-headless.sh` (521 tests, 520 pass, 1 todo from before),
`sdk-package.sh check`, `check-sdk-deletions.py`, `check-workspace-policy.sh`.

## Decision record: piece C, strip order from the list (batch 8)

**Shipped (D5, gate 5).** The SDK calls no `miso_engine_web_v1_live_control_*` export and reads no
route or VCA order. The strip order is the owner headers of kind `track`, then `submix`, of the core's
control list (`ControlStripOrder { structureEpoch, tracks, submixes }`, read by `stripOrderOf` in
`sdk/src/core/control-list.ts`). The paging loop of `readControlList` is now a generator (`listSteps`)
that an async driver (`readControlList`, `readControlStripOrder`) and a sync driver
(`readControlStripOrderSync`) both run, so the in-process engine and the browser read the list with
one piece of code.

- **Headless.** `WasmBoundary.stripOrder()` feeds `shape()`, `sessionMap()`, `observationMap()` and
  `readObservations()`. `shape()` does not run on the render path any more (`OfflineEngine.render()`
  takes the quantum from `status()`).
- **Re-read on a new epoch.** Each `stripOrder()` call first asks for one row of the session owner (the
  smallest page the core serves) and reads the whole list only when that page's `structure_epoch` is
  not the one of the cached order. This follows the spec's "after every reply whose epoch differs"
  without the boundary decoding every reply, and it is never stale. `pollMeters()` is a per-block call:
  it uses the cached order and re-reads only when a frame's header `trackCount` disagrees with it.
  `reboot()` and `dispose()` clear the cache.
- **One allocator per endpoint.** The strip-order reads send frames, and the core takes request IDs in
  ascending order. `WasmBoundary.controlRequestIds` is therefore the one allocator of the instance's
  control endpoint, and `OfflineEngine.controlTransactions()` and `controls()` use it (a second
  `ControlRequestIds` on the same core collides: piece A's finding).
- **Browser.** `createBrowserLiveControls` takes `tracks` and `submixes` of its map from the list
  through the host's `control` message (sources and the meter lease still come from the worklet's
  `miso.sessionmap.v1`); `createBrowserStripOrder(host)` serves observation (`observationMap`,
  `readObservations`) with the same probe-then-read rule. The scratch boot's `shape` (the strips the
  meter projection uses) comes from `WasmBoundary.shape()` and so from the list.
- **Exports not required.** `exportsOf` skips the four index exports (`UNUSED_EXPORTS` in
  `boundary.ts`), so a module without them boots. #1565B deletes the exports, drops their names from
  the generated layout and deletes this set.

**Departures from the text.** (1) The browser meter feed keeps the strip ids of the boot-time shape
(`createMeasurementFeeds(host, shape.tracks, ...)`); a frame for another shape is refused
`abiMismatch` by `meterProjection` as before. No structure change that moves strips can commit before
#1562b, so there is nothing to follow yet. A successor that re-reads on a refused frame belongs with
#1562b. (2) The test fixtures that serve a browser host from a headless engine (`engineHost`) renumber
the browser's frames onto the engine's one allocator (`onEndpointIds`, `control-support.mjs`), and the
scripted fake hosts answer `CONTROL_LIST_GET` with strip headers (`stripListReply`).

**Gates and tests** (`sdk/test/strip-order-evals.mjs`, 4 tests): gate 5, `a module without the
live_control_* exports boots, meters and observes` (shape, session map, a completed meter frame, the
observation map and a read on the headless engine, on a copy of the export object with the four
exports deleted; red on the old boundary with `abiMismatch ... does not export
miso_engine_web_v1_live_control_submix_count`) and `a module that has the exports never sees them
called` (counts calls across boot, shape, meters, observation and `loadSession`); D5, `the list is
read once per epoch and again after the epoch moves` (an unchanged epoch returns the same order
object; a live `matrix_mode` switch makes a new one) and `the browser reader follows a new epoch and
keeps the order of an unchanged one`. `browser-defaults-evals.mjs` now makes the worklet's own
`tracks`/`submixes` wrong on purpose in the submix test, so reading them fails it. Frame counts in
`browser-defaults-evals.mjs` and the request-ID assertion in `live-controls-evals.mjs` count the
epoch probe and the list page. Run, with the artifacts of this tree: `check-sdk-types.sh`,
`check-sdk-headless.sh` (525 tests, 524 pass, 1 todo from before), `check-sdk-generated.sh`,
`sdk-package.sh check`, `check-sdk-deletions.py`, `check-workspace-policy.sh`.

**For #1565B** (readers of `live_control_{track,submix}_{count,id}` left): host-web
`ffi.rs:4112-4144` (four exports) and `lib.rs:2895`, `:2906` (`copy_live_control_{track,submix}_id`);
`render_lock.rs:26-27` (a comment); the worklet `miso-engine-v1-audio-worklet.js:611-645` (strip order
for `miso.sessionmap.v1`, answered at `:1024-1038`) and the host-side validation of that map in
`miso-engine-v1-audio-worklet-host.js` (`sessionMap`, `:1049`); the Rust tests `tests.rs:3502-3529`,
`:4702-4737`, `:4870`; `hosts/host-web/tests/browser-v1/direct-oracle.mjs:316`, `:383`;
`scripts/test-web-audioworklet.mjs:2733-2741`, `:4300-4307`; the export lists in
`scripts/check-web-audioworklet.sh:290-293`, `scripts/check-abi-layout-v1.py:138-141`,
`scripts/fixtures/abi-layout-v1-self-test.json:26-29`, `tools/parameter-metadata/src/abi_layout.rs:148-151`
and the mirrors they generate (`sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts`);
and the SDK's `UNUSED_EXPORTS` (`sdk/src/core/boundary.ts`). The SDK's `MisoSessionMap` type
(`shipped-host.d.ts`, a pinned mirror of the host's `.d.ts`) still carries `tracks`/`submixes`, which
the SDK now ignores.
