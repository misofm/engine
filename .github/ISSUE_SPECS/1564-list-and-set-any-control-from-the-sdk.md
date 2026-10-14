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
