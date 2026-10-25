# Encode the session, submix, output, route, automation and VCA edits in the SDK

Stream H(c) of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-11, D15-13 E1).
Code anchors verified on `main` at `6fb211594`.

Second half of *Build and encode session transactions in the SDK* (#1383).

## Product outcome

The SDK transaction builder covers every session edit the protocol defines. A user can rename the
session, change the profiles or the console, and add, change or remove submixes, outputs, routes,
stored automation and VCAs in one `engine.apply` transaction. The bytes equal the engine codec's.

## Context

- The remaining opcodes (`SessionEditOpcode`, `crates/protocol/src/model.rs:22`): `SetSessionId`
  `0x0001`, `SetSampleRateHz` `0x0002`, `SetQuantumFrames` `0x0003`, `SetRenderProfile` `0x0004`,
  `SetOutputProfile` `0x0005`, `SetConsole` `0x0007`, `SetControlSmoothing` `0x0008`
  (`control_smoothing`, #1365; `0x0006` is retired); `UpsertSubmix`,
  `RemoveSubmix` (`0x0300-0x0301`); `UpsertOutput`, `RemoveOutput` (`0x0400-0x0401`);
  `UpsertRoute` to `SetRouteFollowsMute` (`0x0500-0x0507`); `UpsertAutomation` to
  `SetAutomationSegments` (`0x0600-0x0603`); `UpsertVca`, `RemoveVca`, `SetVcaFader`
  (`0x0700-0x0702`). Payload field order is `tx_edit_payload` (`crates/protocol/src/session_wire.rs:338`).
  `SetAutomationSegments` carries a repeated segment field (`:345-347`). `SetControlSmoothing`
  (Amendment 1) has field 1, an optional nested message with `f32` fields `mute_ms` 1, `fader_ms` 2
  and `pan_ms` 3; the field is absent for the clear.
- #1383 delivers the writer, the frame, the builder, the source/track/strip-effect methods, and
  the native oracle `hosts/host-web/examples/sdk_transaction_oracle.rs`.
- The `SessionBuilder` specs these methods reuse: `.console` (`sdk/src/core/session.ts:723`),
  `.submix` (`:773`), `.output` (`:791`), `.vca` (`:809`), `.route` (`:846`), `.automation`
  (`:886`). Automation targets resolve through `resolveAutomationTarget`
  (`sdk/src/core/session.ts:1405-1477`), which *Refuse automation on effect parameters that are
  not block-rate* (#1335) makes refuse a non-automatable or non-`Block` effect parameter.
- A rate or quantum change is legal on the wire, and the engine refuses it at apply with its typed
  result (#1290 D1). The builder encodes it so the C ABI and the browser see the same transaction.

## Decisions frozen for this slice

- **D1. Methods.** `setSessionId`, `setSampleRateHz`, `setQuantumFrames`, `setRenderProfile`,
  `setOutputProfile`, `setConsole`, `setControlSmoothing`; `upsertSubmix`, `removeSubmix`; `upsertOutput`,
  `removeOutput`; `upsertRoute`, `removeRoute`, `setRouteSource`, `setRouteDestination`,
  `setRouteChannelMatrix`, `setRouteGainDb`, `setRouteMute`, `setRouteFollowsMute`;
  `upsertAutomation`, `removeAutomation`, `setAutomationTarget`, `setAutomationSegments`;
  `upsertVca`, `removeVca`, `setVcaFader`. (Amendment 1) `setControlSmoothing({ muteMs, faderMs,
  panMs } | null)`: an object writes field 1; `null` writes no field, which clears the key so the
  session uses the default table.
- **D2. Validation.** Each method validates with the validator its `SessionBuilder` counterpart
  uses. Every automation target goes through `resolveAutomationTarget`, so #1335's refusal applies
  to transactions exactly as to built sessions. (Amendment 1) `setControlSmoothing` reuses
  `normalizeControlSmoothing(value, path)`, the f32 bounds helper that `SessionOptions.controlSmoothing`
  also uses and that lives in `sdk/src/internal/control-smoothing.ts` (Root ruling (2026-10-12): moved
  out of the module-private scope of `sdk/src/core/session.ts`, with no public export). It refuses
  with the builder's `MisoUsageError` text after the path (the transaction passes its own method path)
  and produces no bytes.
- **D3. Encoding.** The same writer and payload discipline as #1383 D3; no new writer.
- **D4. Per-edit ramp.** `setConsole`, `setRouteChannelMatrix`, `setRouteGainDb`, `setRouteMute`
  and `setVcaFader` take the optional `{ rampSamples?: number }` of #1383 D2 and write the
  `ramp_samples` field of *Carry an optional per-edit ramp length on live session edits* (#1394
  D1) when it is given, never when it is omitted.

## Deliverables

1. D1-D4 in `sdk/src/core/transaction.ts`.
2. Cases in `sdk/test/transaction-evals.mjs`.

## Authorized paths

- `sdk/src/core/transaction.ts`, `sdk/test/`
- Root ruling (2026-10-12): `sdk/src/internal/control-smoothing.ts` (import only; the stream E2 batch
  follow-up creates it, see *Batch verdict follow-ups*)

## Non-goals

- No oracle change: #1383's oracle decodes every opcode already. No SDK API change beyond the
  builder.

## Objective gates

1. **Bytes equal the codec's.** For each method in D1 at least once (`setAutomationSegments` with 0,
   1 and 3 segments; `setConsole` with an empty and a two-slot console; each D4 method with
   `rampSamples` omitted, 0 and 960), #1383's oracle
   re-encoding equals the SDK's bytes exactly.
2. **Semantics equal the builder's.** A transaction that adds a submix, routes a track to it with
   `followsMute`, adds a VCA over two tracks and adds automation on a block-rate EQ gain: the
   oracle snapshot equals the `toJson()` of a `SessionBuilder` that declares the result directly.
3. **Automation refusal.** `upsertAutomation` aimed at a parametric EQ band's `enabled` is refused
   by the builder with the same `MisoUsageError` text `SessionBuilder.automation` gives, and
   produces no bytes.
4. Commands, as #1383: `bash scripts/check-sdk-types.sh`, `bash scripts/check-sdk-headless.sh
   <artifacts>`, `bash scripts/check-sdk-generated.sh <artifacts>`,
   `python3 -B scripts/check-sdk-deletions.py`, `bash scripts/sdk-package.sh check <artifacts>`.

## Test value

- Gate 1: a field order, count or width error in any of the 26 payloads makes the codec decode a
  different edit or refuse the frame; it turns red. So does a D4 method that writes 0 for an
  omitted `rampSamples`.
- Gate 2: a byte-valid encoder that maps a spec field to the wrong session field (a route's tap,
  a VCA member list) changes the snapshot; it turns red.
- Gate 3: a transaction path that skips `resolveAutomationTarget` would let an app store automation
  the engine keeps prepared, acked with no effect; it turns red.

## Amendment 1 (root, 2026-10-12)

*Edit control_smoothing by a session transaction, model-only* (#1365) adds opcode
`SetControlSmoothing` `0x0008`. This slice encodes it (#1383 leaves `0x0001`-`0x0008` to it), or the SDK
would cover all but one protocol edit. The Context list, D1, D2, the Test value count and the Dependencies carry the
change above. Three gates are added (each with its own test-value sentence; the numbers A1.1 to
A1.3 are this amendment's own, so they do not clash with gates 1 to 4 above), and gates 1 and 2
also include the new method where they name "each method in D1":

A1.1. **Bytes** (the new method in gate 1's oracle comparison). The oracle's re-encoding equals the
   SDK's bytes for `{ muteMs: 5, faderMs: 15, panMs: 25 }`, `{ muteMs: 0, faderMs: -0, panMs: 1000 }`
   and `null`. Test value: red if the SDK writes bytes for one of those three values that the
   engine's codec decodes to a record it would then write differently, a non-canonical encoding. It
   does not catch
   field 1 written for `null`, or `-0` written as `+0`: decode then re-encode makes the oracle write
   the same bytes, so those stay green. It does not catch two swapped length field IDs either: the
   oracle re-encodes a byte-valid swap identically. A1.2's distinct values 5, 15 and 25 catch the
   swap, and its `null` case catches field 1 written for `null`.
A1.2. **Semantics** (the new method in gate 2's snapshot comparison). The oracle snapshot after
   `{ muteMs: 5, faderMs: 15, panMs: 25 }` equals the `toJson()` of a `SessionBuilder` that
   declares that `controlSmoothing`, and a following `null` gives `{ "kind": "default" }`. Test
   value: red if `null` encodes an explicit zero table instead of the clear.
A1.3. **Refusal** (the new method in gate 3's style). `1000.001`, `-1`, `NaN` and a missing length are
   refused with the builder's text after the path and no bytes. Test value: red if the transaction path skips the
   builder's bounds.

## Dependencies

- *Build and encode session transactions in the SDK* (#1383).
- *Refuse automation on effect parameters that are not block-rate* (#1335).
- *Carry an optional per-edit ramp length on live session edits* (#1394), for D4's field.
- *Edit control_smoothing by a session transaction, model-only* (#1365), for the `0x0008` opcode and
  its field 1 (Amendment 1).

## Batch verdict follow-ups (stream E2 batch, 2026-10-12)

- *n6:* A1.1's test-value sentence claimed a swap of two length field IDs turns it red. The oracle
  re-encodes a byte-valid swap identically, so A1.1 does not catch it; A1.2's distinct values do. The
  sentence now says only what A1.1 catches.
- *m3, Root ruling (2026-10-12):* D2 named a helper this slice could not reach (module-private, a
  hard-coded error path, and `core/session.ts` is not an authorized path). The stream E2 batch moved it to
  `sdk/src/internal/control-smoothing.ts` as `normalizeControlSmoothing(value, path)`, unchanged for the
  builder, and this slice imports it from there (Authorized paths and D2 above). Record: #1364's
  "Batch verdict follow-ups".

### Batch verdict 2 follow-ups (stream E2 batch, rebased tree)

Source: `stream-e2-batch-verdict-2.md` (batch head `fcc5c890e`).

- *n6.* A1.1's test-value sentence now states only what A1.1 catches: a non-canonical encoding of the three values. It
  names field 1 written for `null` and `-0` written as `+0` as cases it does not catch (decode then re-encode writes the
  same bytes); A1.2's `null` case catches the first.

**Decision 17 amendment (2026-10-13).** #1561 retires `SetControlSmoothing` (`0x0008`), `SetRouteChannelMatrix`, `SetRouteGainDb`, `SetRouteMute`, `SetRouteFollowsMute` (`0x0504`-`0x0507`) and `SetVcaFader` (`0x0702`); their values are `SET_CONTROL` edits. This issue encodes only the remaining structural ops. The stored automation target is a control address (#1566).

## Decision record (implementation, 2026-10-25)

Spec corrections (scope note of 2026-10-25, recorded here; they supersede the text above where it conflicts):

- **Deliverables 1** is `sdk/src/core/session-transaction.ts` (#1383's file); `transaction.ts` does not exist.
- **D1** is twenty methods. Decision 17 (#1561) retired `setControlSmoothing`, `setRouteChannelMatrix`,
  `setRouteGainDb`, `setRouteMute`, `setRouteFollowsMute` and `setVcaFader`; they are not added. **Amendment 1**
  (A1.1-A1.3) and the `sdk/src/internal/control-smoothing.ts` path are struck with `setControlSmoothing`.
- **D4** keeps `setConsole` only: `schema::session::ramp_samples_field` gives `ramp_samples` to `SetConsole`,
  `SetTrackConsole` and `SetControl` and to no other opcode.
- **D2** reuses `session.ts`'s validators, exported `@internal` (R1): `normalizeConsoleSlots` (the body of
  `SessionBuilder.console`), `normalizeSubmix` (a `verb` parameter, as `normalizeTrack`), `normalizeRoute`,
  `normalizeRouteSourceSpec`, `normalizeRouteDestinationSpec` (shape, endpoint existence and defaults, now also what
  `SessionBuilder.route` and its model run), `launchRate` and `quantum` (`session()`'s rate and quantum checks).
  `BuilderContext.otherGraphIds` is removed: the transaction keeps its own working track, submix, VCA and output
  ID sets, read from the base model, so an ID freed by a removal can be reused in the same transaction.
- A rate or quantum change (R2): the session store the oracle runs applies it; the control plane refuses it at apply
  with `control.apply.reprepare_required` (#1290a). So gate 1's row is an ordinary applied frame.
- The render and output profiles have no `SessionBuilder` counterpart (it writes them fixed). `setRenderProfile({ id,
  mode? })` and `setOutputProfile({ id, channels?, sampleFormat? })` refuse what the session validator refuses: a
  mode other than `single_thread`, channels other than 2 (`numeric.out_of_schema_range`), a format other than
  `f32_planar`.
- `consoleChanged` is true when the working console differs from the base session's, as the store's
  `SetTrackConsole` rule compares the candidate console with the committed one; a `setConsole` that restates the
  base console opens nothing.
- **Gate 1 test value, corrected:** the oracle re-encodes what it decoded, so an omitted `rampSamples` written as 0
  re-encodes to the same bytes and stays green on the byte comparison. The eval therefore also asserts that the
  omitted, 0 and 960 variants of the same console transaction are three different frames.
- **Gate 1 also owns the `setTrackConsole` byte rows** (root comment of 2026-10-24): `rampSamples` omitted, 0 and
  960, after a `setConsole` in the same transaction.
- **Slice b (automation, VCA).** `resolveAutomationTarget` takes an `AutomationScope` (console slots, each strip's
  `[insert ID, effect ID]` list, route and VCA IDs) instead of `session.ts`'s private `BuilderState`; the session
  builder and the transaction each build one from their own state (R1's intent, without a fake builder state).
  `normalizeAutomation` (a `verb` parameter), `normalizeAutomationSegments`, `automationTargetRecord`,
  `requireSegments` and `normalizeVca` are exported `@internal`. VCA members are written in the model's sorted order.
- **R4 corrected:** the engine refuses an empty span list (`SessionEditError::EmptyAutomationSegments` in
  `apply_session_edit`, and the validator's "automation must declare at least one segment"), so a 0-segment
  `setAutomationSegments` is not a clear. The builder refuses it with `SessionBuilder.automation`'s text; gate 1's span
  counts are 1 and 3, and the 0 case is a refusal row.
- **Gate 3 corrected:** since #1566 (decision 17 C2) `SessionBuilder.automation` no longer refuses a row the engine
  will not automate, and the generated control tables mark the EQ band's `enabled` row automatable. Gate 3 is now: each
  target or span `SessionBuilder.automation` refuses (an insert not on the strip, an unknown row, a missing owner, a
  VCA owner that is a track, a session owner; an empty, reversed or non-positive exponential span) is refused by the
  transaction with the same message text after the method's path, and produces no bytes.
- `setAutomationTarget` keeps the spans, so it refuses a row in another unit (`control.domain`, as the engine refuses
  a span outside its row's unit, `controls::automation`).
