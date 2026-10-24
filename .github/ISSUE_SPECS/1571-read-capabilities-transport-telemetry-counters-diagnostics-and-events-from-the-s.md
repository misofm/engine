# Read capabilities, transport, telemetry, counters, diagnostics and events from the SDK

Slice S14b of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
"the SDK is a thin client" and C13). Code anchors verified on `main` `df585e503`. Feature sprint
mode (owner, 2026-10-13): one implementer, normal tests, a batch review; no per-attempt verifier or
mutation runs.

This is the SDK half of #1570. #1570 makes the browser host serve every protocol command and event
with the C ABI's bytes. This slice gives browser and headless apps typed calls for those messages.

## Product outcome

A browser or headless app reads the engine's capabilities, the committed session document, the
transport, the counters and the retained diagnostics. It sets the transport and the telemetry
configuration. It receives every protocol event as a typed value. Each call sends the core's frame
and decodes the core's reply, with the same names, types and results on both engines. A mobile app
reaches the same messages through `miso_engine_v1_submit_command` and
`miso_engine_v1_dequeue_event`, so no platform has a feature that another lacks.

## Context

- **The path to the core.** #1562's `control` message (browser) and its
  `miso_engine_web_v1_control_*` exports (headless) carry any command frame to
  `SessionState::command`. #1570's `control-event { lane, frame }` message and the wrapper's
  `onControlEvent` (browser), and its `miso_engine_web_v1_control_event` export (headless), carry
  every event frame.
- **The codec and the queue.** #1563 adds `sdk/src/core/control-frame.ts`: the BTLV frame
  primitives, the `SET_CONTROL` transaction encoder and the response decoder. #1563 D4 keeps one
  transaction queue per engine: one request in flight, the last committed revision, and the exact
  expected revision on each transaction.
- **The engines.** `BrowserEngine` (`sdk/src/browser/engine.ts:192`), `OfflineEngine`
  (`sdk/src/headless/engine.ts:114`) over `WasmBoundary` (`sdk/src/core/boundary.ts:332`), and the
  shipped wrapper's declaration `MisoAudioWorkletHost` (`sdk/src/browser/shipped-host.d.ts:946`).
  Engine refusals are `MisoEngineError` (`sdk/src/core/errors.ts:88`).
- **The messages** (`crates/protocol/src/message_wire.rs`): `Capabilities` (`:66`, 27 fields),
  `SessionSnapshotRequest` and `SessionSnapshot` (`:145`, `:153`), `TransportState` (`:480`,
  stopped 1 and playing 2), `TransportSetRequest` (`:490`), `TransportSnapshot` (`:499`),
  `CounterId` (`:626`), `TelemetryConfiguration` (`:648`), `CountersRequest` (`:660`),
  `CounterSnapshot` (`:677`), `DiagnosticsRequest` (`:696`), `DiagnosticsPage` (`:705`),
  `DiagnosticSeverity` (`:725`), `Diagnostic` (`:747`). Field numbers and wire types are in
  `crates/protocol/src/schema.rs`. Events are `MessageId` `0x8001`-`0x8030`
  (`crates/protocol/src/wire.rs:122-132`). `TRANSPORT_SET` and `TELEMETRY_CONFIGURE` need an exact
  expected revision (`crates/protocol/src/controller.rs:920-928`).
- **Planned browser-only paths.** *Export transaction apply and anchored seek from the browser
  engine module* (#1293, D4) plans `committed_document_ptr` and `_bytes` exports. *Send a session
  transaction to the browser control plane* (#1294, D1 and D6) plans a `miso.document.v1` message
  and a wrapper `committedDocument()` on them. `SESSION_SNAPSHOT_GET` already gives the same
  document on both hosts.

## Decisions frozen for this slice

- **D1. One message module.** New `sdk/src/core/protocol-messages.ts`, built on #1563's frame
  primitives (exported from `control-frame.ts` for SDK-internal use only). It holds:
  - request encoders for `CAPABILITIES_GET`, `SESSION_SNAPSHOT_GET`, `TRANSPORT_GET`,
    `TRANSPORT_SET`, `TELEMETRY_CONFIGURE`, `COUNTERS_GET` and `DIAGNOSTICS_GET`;
  - decoders for their success payloads, and for the non-OK response (status and diagnostics);
  - decoders for the six event payloads.

  Field numbers, types and presence rules follow `schema.rs` exactly. A frame the module cannot
  decode throws `MisoEngineError` with phase `lifecycle` and code `internal`.
- **D2. One transport for every frame.** Every request goes through #1563 D4's per-engine queue,
  so one request-ID counter serves the engine. `TRANSPORT_SET` and `TELEMETRY_CONFIGURE` carry the
  queue's exact expected revision. The queries carry revision-any. A non-OK reply rejects the call
  exactly as #1563's queue rejects a refused transaction, with the core's status and diagnostic
  codes.
- **D3. The calls.** `BrowserEngine` and `OfflineEngine` gain the same methods, with the same types,
  all returning a `Promise`:
  - `capabilities()`: the 27 fields, `u64` values as `bigint`, command and event IDs as arrays.
  - `committedDocument()`: `{ revision: bigint, document: string }`. It reads pages of at most
    65,536 bytes. The first page is revision-any. Each continuation names the first page's
    revision. On `REVISION_CONFLICT` it starts again at offset 0, at most 3 times in all, and then
    rejects with the core's code. It decodes the reassembled bytes as UTF-8 only at the end (a
    page may split a code point).
  - `transport()` and `setTransport({ state, position? })`: `state` is `"stopped"` or
    `"playing"`, `position` a `bigint`. Both resolve to `{ state, position, effectiveSample }`.
  - `configureTelemetry(config)`: `{ meterHandles, meterPeriodBlocks, counterIds,
    counterPeriodBlocks, diagnosticsEnabled, minimumSeverity }`. It resolves to the configuration
    the engine applied.
  - `counters(ids?)`: absent means all. Counter names are the camel-case `CounterId` names
    (`liveValuesSuperseded`, ...), generated from the Rust enum by `sdk/codegen/generate.mjs`. It
    resolves to `{ observedSample, values: { id, value: bigint }[] }`.
  - `diagnostics({ afterSequence?, limit?, minimumSeverity? })`: the defaults are `0n`, 64 and
    `"info"`. It resolves to `{ lastSequence, eof, diagnostics }`; each diagnostic has `code`,
    `severity`, `path`, `detail?`, `operationIndex?`, `sampleTime?`, `providerSequence?`.
  - `onEvent(listener)`: it returns an unsubscribe function. A listener receives
    `{ kind, revision, ... }` with `kind` one of `"sessionCommitted"`, `"automationCanceled"`,
    `"transportState"`, `"meterBatch"`, `"counterSnapshot"`, `"diagnostic"`, and that event's
    decoded fields.
- **D4. Events on both engines.** Browser: the engine subscribes once to the wrapper's
  `onControlEvent` and decodes each frame. Headless: after each queue request and after each
  `render()`, the engine runs #1381's service export (if no earlier slice runs it there) and then
  drains both lanes in #1570 D4's order and bound. Both engines deliver events to the listeners in
  engine order. With no listener, the engine drops the event and adds 1 to an internal
  `discardedEvents` count. An event is not the ack of a call, so no ack precedes a drop.
- **D5. No automation enqueue call.** `AUTOMATION_ENQUEUE` reaches no PCM on any host
  (`docs/CONTROL_PROTOCOL_SEMANTICS.md:15`), and #1315 refuses it. A typed automation call waits
  for the issue that renders automation (#1567 addresses it).
- **D6. No SDK checks.** The SDK checks only what makes a frame encodable, as #1563 D3: types,
  integer ranges of the wire fields, and known names. The core answers everything else with its
  own codes.

## Deliverables

1. D1 in `sdk/src/core/protocol-messages.ts`.
2. D2-D4 in both engines and `sdk/src/core/boundary.ts` (the headless `control_event` call).
3. The fixture writer and the evals of the gates; one browser qualification row.

## Authorized paths

- `sdk/src/core/{protocol-messages.ts,control-frame.ts,boundary.ts,errors.ts,types.ts}`
  (`control-frame.ts`: the internal export of its primitives only), `sdk/src/browser/engine.ts`,
  `sdk/src/headless/engine.ts`, `sdk/src/index.ts` and the package barrels (the new exports),
  `sdk/codegen/generate.mjs` and its regenerated output (the counter names only).
- `sdk/test/**`, including the new fixture `sdk/test/fixtures/protocol-messages.json`.
- `crates/protocol/tests/sdk_message_fixture.rs` (new: writes or checks the fixture).
- `hosts/host-web/qualification/` (gate 3's browser row).

## Non-goals

- No engine or host-web change (#1570 and #1562).
- No automation call (D5), no meter-handle meaning (#1583 to #1588), no control list call (#1564).
- No structural transaction API (#1296, #1389).

## Objective gates

Commands: `cargo test --locked -p protocol --test sdk_message_fixture`,
`bash scripts/check-sdk-types.sh`,
`bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`,
`bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
`python3 -B scripts/check-sdk-deletions.py`,
`bash scripts/sdk-package.sh check target/ci/qualification-artifacts`, and the browser
qualification (`npm run qualify -- --artifacts ... --sdk-root ... --browser <b> --check-matrix
--self-test-mutations` in `hosts/host-web/qualification`) for chromium, firefox and webkit.

1. **The SDK's bytes are the core's.** The Rust test encodes, with `ProtocolCodec`, one request
   frame per D1 command with fixed fields, and one response or event frame per D1 decoder with
   fixed payloads (every optional field present once and absent once). It checks them against
   `sdk/test/fixtures/protocol-messages.json` (the test rewrites the file under
   `UPDATE_FIXTURES=1`). An SDK eval encodes the same requests and compares the bytes. It decodes
   every response and event frame and compares the values. This is a wire format, so a byte
   comparison is the right gate.
2. **The same answers on both engines** (headless eval, and the browser row of gate 3). On one
   session with the same limits: `capabilities()`, `transport()` after
   `setTransport({ state: "playing", position: 0n })`, `counters(["liveValuesSuperseded"])` after
   one live fader edit, and `diagnostics()` give equal values.
3. **Events end to end.** On the headless engine, and in a qualification row in three browsers on
   both page modes: `configureTelemetry` with meter handle 1 (period 1), counter
   `liveValuesSuperseded` (period 1) and diagnostics on at `"info"`; then one `SET_CONTROL` fader
   edit, `setTransport` to playing, and 4 rendered blocks. `onEvent` receives
   `sessionCommitted` (its revision is the edit's), `transportState`, `counterSnapshot`,
   `meterBatch` and `diagnostic`, in engine order.
4. **The committed document.** On the headless engine, after one fader edit,
   `committedDocument()` equals the canonical document of the committed model, byte for byte. With
   a session whose document is larger than 65,536 bytes, the pages reassemble exactly.
5. **One refusal on both engines.** `diagnostics({ limit: 0 })` rejects with the core's status and
   code, the same on the headless engine and in the browser.

## Test value

- Gate 1 turns red if any SDK encoder or decoder differs from the core's codec by one field.
- Gate 2 turns red if the two SDK engines reach different code, or if one sends a stale revision.
- Gate 3 turns red if an event is lost, reordered or misdecoded between the core and a listener.
- Gate 4 turns red if paging drops, repeats or splits a byte of the committed document.
- Gate 5 turns red if the SDK checks a field the core owns, or answers differently per engine.

## Dependencies

- #1570 (the event export, the `control-event` message and the browser's render activity).
- #1563 (`control-frame.ts` and the per-engine queue) and #1562 (the `control` message).
- *Swap and retire browser plans through the Worker's service loop* (#1381): the service export
  that the headless engine runs (D4).

## Supersedes and shrinks

- Shrinks *Export transaction apply and anchored seek from the browser engine module* (#1293): its
  D4 (the committed-document exports) is withdrawn.
- Shrinks *Send a session transaction to the browser control plane* (#1294): its D1
  `miso.document.v1` message and its D6 `committedDocument()` wrapper method are withdrawn.

## Decision record (root, 2026-10-24) and tranche a evidence

**Split.** Tranche a: the codec (`sdk/src/core/protocol-messages.ts`), the Rust fixture, the
`OfflineEngine` calls and the headless evals. Tranche b: the same calls on `BrowserEngine` over
`host.control`/`host.onControlEvent`, and the browser qualification rows of gates 2, 3 and 5 (three
browsers, both page modes). The issue closes only after b (decision 17, C13).

**Spec corrections (facts on `main` `abbaee34d`).**
- Events are `SESSION_COMMITTED 0x8001`, `TRANSPORT_STATE 0x8010`, `OBSERVATION_ENDED 0x8011`,
  `METER_BATCH 0x8020`, `COUNTER_SNAPSHOT 0x8021`, `DIAGNOSTIC 0x8030`; `automationCanceled` is
  retired with `AUTOMATION_ENQUEUE` (#1627, #1628). `onEvent` kinds: `sessionCommitted`,
  `transportState`, `observationEnded` (through the one decoder in `observation-ended.ts`),
  `meterBatch`, `counterSnapshot`, `diagnostic`.
- `Capabilities` has 23 wire fields (9, 12, 19, 20 retired), 21 SDK fields (two versions).
- The transport position is retired (#1635): `setTransport({ state })` and
  `{ state, effectiveSample }`; a `position` key is a `MisoUsageError`, never dropped. No wire
  change was needed.
- `TRANSPORT_SET` and `TELEMETRY_CONFIGURE` expect the core's committed revision
  (`controlRevision()`), which equals the queue's and also sees a commit by another sender.
- Counter names: the Rust fixture lists every `CounterId` the codec accepts (found by decoding, with
  an exhaustive name match), and the eval holds the SDK's `COUNTER_IDS` equal to it. No codegen
  asset exists for the protocol registry, and adding one needs a Rust tool outside this issue's
  paths, so `generate.mjs` is not changed.
- `committedDocument()` pages at `min(65,536, maximumFrameBytes - 104)` bytes, rounded down to 8:
  the browser host's frame limit is 16 KiB, so 65,536-byte pages cannot fit in one reply.
- The headless engine reads events after every `render()` too (D4): the lossy lane holds one event,
  so without that read a block's telemetry was replaced before the next control frame. The read
  runs the core's service step first, so no separate service call is needed.
- The bless variable is `MISO_ENGINE_BLESS_FIXTURES=1`, as for the `SET_CONTROL` fixture.
- The headless evals are in a new `sdk/test/protocol-messages-evals.mjs`;
  `headless-path-evals.mjs` tests the gate script's paths, not the engine.

**Facts measured (headless).** One fader edit, `setTransport` playing and four blocks after
`configureTelemetry` (meter 1, `liveValuesSuperseded`, diagnostics at info) give, in order:
`sessionCommitted` (the edit's revision), `transportState`, then per block `diagnostic`
(`capi.render.activity`), `meterBatch` and `counterSnapshot`. `diagnostics({ limit: 0 })` is
refused by the core with status `malformed_frame` and code `protocol.failure`. `counters()` (all)
returns the eight counters the core serves, not all sixteen IDs.
