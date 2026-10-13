# Send every SDK value edit as a SET_CONTROL transaction

Slice S11a of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
"the SDK is a thin client" and C5). Code anchors verified on `main` `df585e503`. Feature sprint
mode (owner, 2026-10-13): one implementer, normal tests, a batch review; no per-attempt verifier or
mutation runs.

Decision 17's SDK slice is split in two: this slice moves every value edit onto the core's op;
#1564 adds the control list and one generic setter. #1565 deletes host-web's own command path
after this slice.

## Product outcome

A browser or headless app that moves a fader, a send, a VCA, an effect knob, an input filter or a
solo through the SDK's live-control builders sends a `SET_CONTROL` transaction to the core, not a
48-byte command record. The SDK checks no domain of its own: an out-of-range value comes back as
the core's `control.domain`, with the same code on the browser and on the headless engine. The
builders keep their names and arguments.

## Context

- **The builders.** `EngineLiveControls` (`sdk/src/core/live-controls.ts:1005`) and
  `LiveControlEdits` (`:455`) with `TrackEdits` (`:757`), `SubmixEdits` (`:774`), `RouteEdits`
  (`:787`), `VcaEdits` (`:831`) and `EffectEdits` (`:877`). They resolve stable IDs to indices
  (`:463-471`) and build `LaneEdit` records (`sdk/src/core/writer.ts:45`).
- **The writer.** `encodeLaneEdits` (`writer.ts:166`) writes 48-byte records;
  `LiveControlWriter` (`:186`) stages and coalesces them and flushes batches. The ramp word rules
  live in `sdk/src/internal/ramp.ts` (`RAMPED_KINDS`, `requireU32`, the `SESSION_DEFAULT_RAMP`
  sentinel).
- **The transports.** Browser: `createBrowserLiveControls` (`sdk/src/browser/live-controls.ts:44`)
  posts `host.command({ commands })` to the worklet. Headless: `OfflineEngine.liveControls`
  (`sdk/src/headless/engine.ts:271`) calls `submitCommands` (`:328`) on the boundary.
- **The SDK's own checks.** The live-control builders check values themselves: a finite-and-range
  helper (`sdk/src/core/live-controls.ts:386-395`), the builtin lookup (`:397-401`), the lane and
  `u32` checks (`:306-340`) and the #1542 field checks; and `PREPARED_BYPASS_EFFECTS`
  (`live-controls.ts`), a copy of the engine's prepared-bypass list that `console-evals.mjs` holds
  to the engine. (The session builder's authoring checks in `sdk/src/core/session.ts` are not live
  controls and are out of scope.)
- **EQ and input filters** go through `prepared-control.js` (`hosts/host-web/web/prepared-control.js`,
  shipped to the SDK as `sdk/src/assets/prepared-control.js`): the page designs prepared targets
  and sends them with `preparedCommand`. After #1556 the core designs input-filter targets, and after
  #1554 the EQ's, in the classifier.
- **The core path exists.** #1562 adds the `control` message and the
  `miso_engine_web_v1_control_*` exports: any command frame goes to
  `SessionState::command`, and the reply is the response frame.
- **No SDK transaction encoder is on `main`.** *Build and encode session transactions in the SDK*
  (#1383) is open; the SDK has the protocol's BTLV primitives nowhere yet.

## Decisions frozen for this slice

- **D1. One encoder.** `sdk/src/core/control-frame.ts` (new) encodes a
  `SESSION_TRANSACTION_APPLY` frame of `SET_CONTROL` edits (opcode `0x0800`, #1553's payload:
  `address`, `value`, optional `ramp_samples`) and decodes the response frame's status, revision,
  `TransactionApplied` (`edit_path`) and refusal diagnostics. The bytes equal the protocol codec's
  for the same edits (gate 1). If #1383 is on `main` first, this slice uses its encoder and adds
  only the `SET_CONTROL` payload.
- **D2. The edit.** A builder returns `ControlEdit { address: ControlAddress, value: number,
  rampSamples?: number }`. `ControlAddress` is #1550's generated type. The builders map:
  `faderDb`, `mute`, `pan`, `matrix`, `trimDb`, `polarityInvert`, `inputFilters` (HPF and LPF rows),
  effect `parameter`, `bypass` (control `0`), route `gainDb`, `mute`, `matrix`, `followsMute`, VCA
  `faderDb` (offset) and `mute`, `solo`. A builder that sets several values (pan's two lanes, a
  matrix's four coefficients, input filters) returns several edits, in one transaction. Absent
  `rampSamples` omits the field; the SDK resolves no ramp.
- **D3. No SDK value checks.** Delete every domain, lane and index check of a value in the
  builders and the writer (the range helper, the #1542 checks, `PREPARED_BYPASS_EFFECTS`).
  Keep only what makes the frame encodable: `value` is a JavaScript number, `rampSamples` is an
  integer in `[0, 2^32 - 1]`, a lane is `left`, `right` or `both`, and an ID is a string (else
  `MisoUsageError`, before any byte). The
  core refuses everything else with its own code.
- **D4. One transaction queue per engine.** The SDK keeps one queue per engine instance. It holds
  the last committed revision the queue saw (from boot, then from every reply frame) and sends each
  transaction with that exact expected revision. One transaction is in flight at a time. While it
  is in flight, newly staged edits coalesce by address (the latest value and ramp per address
  win) and go in the next transaction. A refused transaction rejects its submit with the core's
  code and is not retried. Every later SDK transaction API (#1296 `engine.apply`) uses this queue.
- **D5. The writer.** `LiveControlWriter` keeps its role (stage, coalesce, flush) over D4's queue,
  keyed by address. `encodeLaneEdits` and `LaneEdit` keep only the observation kinds 7 and 8,
  which stay browser monitor operations until the later browser-feature phase (ruling C9).
- **D6. The report.** `submit` and `flush` resolve with `ControlReport { ok: boolean, revision:
  bigint, path: "live" | "model_only" | "rebuild" | "monitor", code?: string, editIndex?: number }`.
  `CommandReport` stays for the observation kinds only. This is a breaking type change of the
  submit result; releases own the version (`docs/RELEASE.md`).
- **D7. Transports.** Browser: the queue posts #1562's `control` message. Headless: the queue calls
  the `miso_engine_web_v1_control_*` exports in-process. Both decode the same reply bytes with D1.
- **D8. Prepared targets leave the SDK.** The SDK no longer calls `prepared-control.js` for EQ or
  input-filter edits; `sdk/src/assets/prepared-control.js` and its `.d.ts` are deleted, and
  `preparedCommand` is no longer sent.
- **D9. Index maps.** The builders address by stable ID, so the route and VCA index maps
  (`live-controls.ts:468-471`) are deleted. Strip order is still read for meters and observation;
  #1564 moves it to the control list.

## Deliverables

1. D1 and D4-D7 in `sdk/src/core/` and the two transports.
2. D2, D3, D8 and D9 in the builders.
3. The evals migrated: every eval that sent a value kind now sends `SET_CONTROL` and keeps its
   claim; an eval whose only claim was a removed SDK check is replaced by the core-refusal eval of
   gate 2.

## Authorized paths

- `sdk/src/core/{control-frame.ts,live-controls.ts,writer.ts,boundary.ts,errors.ts,types.ts}`,
  `sdk/src/internal/{ramp.ts,validation.ts}`, `sdk/src/browser/{live-controls.ts,engine.ts}`,
  `sdk/src/headless/engine.ts`, `sdk/src/assets/prepared-control.{js,d.ts}` (deleted),
  `sdk/src/index.ts` and the package barrels (the new and removed exports).
- `sdk/test/**`.
- `hosts/host-web/qualification/` (the live-control rows: they now send `SET_CONTROL` through the
  SDK; their pass criteria do not change).
- `scripts/check-sdk-deletions.py` and `scripts/sdk-package.sh` (the asset deletion only).

## Non-goals

- No host-web change (#1562 added the path; #1565 deletes the old one).
- No `engine.controls()`, no generic `edit.set`, no generated builder bodies (#1564).
- No structural SDK transactions (#1296, #1383, #1385).
- No observation or meter change.

## Objective gates

Commands: `bash scripts/check-sdk-types.sh`,
`bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`,
`bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
`python3 -B scripts/check-sdk-deletions.py`,
`bash scripts/sdk-package.sh check target/ci/qualification-artifacts`, and the browser
qualification (`npm run qualify -- --artifacts ... --sdk-root ... --browser <b> --check-matrix
--self-test-mutations` in `hosts/host-web/qualification`) for chromium, firefox and webkit.

1. **Bytes equal the core's.** A Rust test in `crates/protocol` (or `tools/parameter-metadata`)
   writes the codec's frame for a fixed list of `SET_CONTROL` edits (every owner kind and
   section, every lane, ramp absent, `0` and `480`) to a fixture; an SDK eval encodes the same
   edits with D1 and compares the bytes. This is a wire format, so a byte comparison is the right
   gate.
2. **One refusal on both engines.** On the headless engine and in the browser, through the
   builders: a fader of 30 dB, a pan of 2.0, a trim of `NaN` and a fader on an unknown strip ID
   each reject with the core's code (`control.domain` three times, then
   `control.address.unknown`) and change nothing (the next block equals an untouched engine's).
3. **Live, end to end.** On the headless engine, every builder of D2 renders the block it is sent
   before as an engine booted from the committed document would, and its report's path is `live`
   (`monitor` for solo).
4. **Coalescing and order.** With a transaction in flight, staging three values on one address and
   one on another sends one next transaction with two edits holding the latest values; the reports
   resolve in submit order.
5. **Browser.** Every live-control row of the browser qualification passes in three browsers on
   both legs.

## Test value

- Gate 1 turns red if the SDK's frame differs from the core's codec by one byte.
- Gate 2 turns red if the SDK still checks a domain itself, or if the two SDK engines refuse
  differently (the class of #1542).
- Gate 3 turns red if a builder maps to the wrong address, lane or row.
- Gate 4 turns red if the queue sends a stale value, sends two transactions at once (a stale
  revision), or reorders reports.
- Gate 5 turns red if the browser transport breaks in a real browser.

## Dependencies

- #1562 (the `control` message and exports), #1549 and #1550 (the generated `ControlAddress` and row IDs),
  #1553 (`SET_CONTROL`).
- *Build and encode session transactions in the SDK* (#1383) if it is on `main` first (D1).

## Supersedes and shrinks

- Supersedes *Refuse an out-of-range live-control field in the headless SDK writer, as the
  browser host does* (#1542): the core refuses on both engines.
- Supersedes *Test strip edits sent through the live-control writer on the headless engine*
  (#1539): the 48-byte writer for values is gone; gate 4 tests the new writer.
- Supersedes *Crossfade the browser's live bypass command over the session ramp* (#1393): the
  SDK's `bypass` is a `SET_CONTROL` on control `0`, which the core crossfades on both hosts
  (#1341).
- Shrinks *Build and encode session transactions in the SDK* (#1383): its value edits are D1's
  `SET_CONTROL` encoder.

## Amendment (decision 17, root 2026-10-13)

**Single instance first.** The slice is split. Both pieces stay in this issue.

- **#1563A**: D1 (the codec and the reply decoder), the Rust byte fixture and D4 (the per-engine
  queue) on the headless engine. Gates 1 and 4. Dependencies: #1549, #1550, #1553; it lands after
  #1532 (`sdk/src/browser/engine.ts` and `errors.ts` edit order).
- **#1563B**: D2, D3 and D5-D9: the builders on both transports, the deletions and the eval
  migration, and the qualification rows. Gates 2, 3 and 5. Gate 5 runs on the non-isolated leg.
  Dependencies: #1563A, #1562a, #1559 and #1341 (root, 2026-10-13). #1563B lands in one batch with
  #1381a, #1562a, #1570b and #1565A.

- Authorized paths gain `sdk/README.md` (the reference text of the changed submit result).
- *Build and encode session transactions in the SDK* (#1383) lands after this slice (root Q7), so
  D1's "if #1383 is on `main` first" clause does not apply.

**Flip batch 6W (root ruling A1).** #1381a, #1562a, #1570b, #1563B and #1565A land together, in one batch of lane W. Each slice keeps its own commits and tests and is green at its own boundary; the batch runs the full CI-equivalent gate set once. If one slice fails, the batch stops: `main` never gets a partial flip, so it never has two writers of one plan's revision gate and cells. No transitional API lends `SessionState`'s producers or revision to the old command path.
