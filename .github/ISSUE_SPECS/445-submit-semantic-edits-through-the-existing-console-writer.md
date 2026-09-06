# Submit semantic edits through the existing ConsoleWriter

Status: root-approved implementation brief under the user’s Astra medium workflow. This closes the existing app #101 control-boundary mismatch; it adds no queue or playback behavior. Implementation starts only after the matching numbered spec is committed and synchronized.

## Concrete problem and smallest product slice

Inspected reviewed SDK #434 at `/private/tmp/miso-dx-sdk-observer`, HEAD
`8a19a84813230713e8f1604db04be4dccf653283`, specifically
`sdk/src/core/writer.ts`, its existing writer/type tests and ordinary SDK gates.
Inspected the authorized current app binding
`/private/tmp/miso-dx-app/src/lib/mixer/engine/console-writer.ts`.

The SDK writer already stages `LaneEdit` objects and selects the next batch in
`#flushOnce`; it encodes only when invoking its encoded `submit(records,count)`
callback. The app consequently decodes those generated-ABI records into host
commands for another encoding step. The reviewed adapter console accepts
`submit(...edits: readonly LaneEdit[]): Promise<CommandReport>`, and its SDK
console returns actual refusals as reports. One semantic callback on the SAME
writer removes the app's decode/re-encode bridge without replacing queue policy.

## Proposed contract

Support either encoded `submit(records,count)` or semantic
`submitEdits(edits: readonly LaneEdit[])`, each returning
`CommandReport | Promise<CommandReport>`. Preserve `maximumBatch` and its current
default/validation. Reject both or neither at the constructor boundary, including
untyped JavaScript callers. No new queue, writer instance, scheduler, host
protocol, receipt type, Rust change, or runtime dependency.

For strict source compatibility, preserve the currently exported `WriterOptions`
interface as the existing encoded contract. Add one exported semantic option type:

```ts
export interface SemanticWriterOptions extends Omit<WriterOptions, "submit"> {
  readonly submit?: never;
  readonly submitEdits:
    (edits: readonly LaneEdit[]) => CommandReport | Promise<CommandReport>;
}

constructor(options:
  | (WriterOptions & { readonly submitEdits?: never })
  | SemanticWriterOptions)
```

This keeps existing `WriterOptions["submit"]` non-optional and avoids breaking
consumers that extend the existing interface. Replacing `WriterOptions` itself
with a union would preserve ordinary constructor call sites but lose that source
compatibility; it is unnecessary for this slice. The root barrel already exports
all writer types, so the new type needs no hand-maintained export list.

Normalize the selected callback once to an internal semantic submit function:
encoded mode calls its existing callback with `encodeLaneEdits(edits)` and
`edits.length`; semantic mode calls `submitEdits(edits)` directly. Keep both under
the one existing `#tail` and `#flushOnce`. Do not encode, decode, fabricate a
report, or introduce an extra queue in semantic mode. The selected edit array is
readonly at the public boundary; no new freezing/copying policy is necessary.

All existing pending-map identity checks, insertion order, latest-wins staging,
coalescing stats, adaptive halve/grow behavior, escalation behavior, drain bounds,
and recovery of the flush chain remain unchanged. The callback receives the
selected addressed edits including kind names, optional values and smoothing;
it owns transport-specific encoding/validation. Successful accounting uses the
actual report's admitted count. Backpressure admits nothing and retains pending
edits; non-flow refusal still raises the existing MisoUsageError. Preserve the
existing FlushOutcome API (it is not a replacement CommandReport).

App #101 can then use this binding, retaining its existing scheduling/report
association:

```ts
new ConsoleWriter({
  submitEdits: async edits => {
    const report = await console.submit(...edits);
    this.#lastReport = report;
    return report;
  },
});
```

App #101 owns removing decodeRecords/generated ABI imports and the old host
acknowledgment bridge after integrating the reviewed SDK. This SDK issue does not
edit the app or block its ongoing public-host migration. No report fields or
appliedAtSample values are synthesized by this change.

## Exact implementation paths

- `sdk/src/core/writer.ts`: the single option type and constructor dispatch;
  route the already-selected edits through that callback in #flushOnce.
- `sdk/test/writer-evals.mjs`: extend existing real-engine/async fixtures for
  semantic dispatch rather than adding a second harness.
- `sdk/test/barrel-surface.ts`: constructor and public-type checks alongside
  existing writer exports, including the new semantic options type.
- `sdk/README.md`: one concise example of the two mutually exclusive modes.
- New matching numbered `.github/ISSUE_SPECS/...` assigned and synchronized by
  root before implementation; record decisions/checkpoints/evidence there.

No generated declarations, ABI/catalog artifacts, package metadata, build
scripts, browser host, adapter, application, or Rust edits are required.

## Minimum meaningful acceptance

1. Keep every existing encoded writer eval passing. Reuse the real paused-engine
   episode through `engine.console().submit(...edits)` to prove semantic async
   admission, real backpressure, adaptive split and latest-wins final landing.
   Existing helpers already provide lawful edits, queue fill/drain and reports.
2. Extend the existing deferred async race fixture in semantic mode: concurrent
   flushes remain serialized; a newer same-address edit staged while the earlier
   semantic batch is awaiting its actual report survives that report and is
   submitted next. Observe addressed values/counts, outcomes and stats.
3. Exercise a real non-backpressure refusal in semantic mode and confirm the
   existing escalation plus usable subsequent flush chain. No fake success or
   weakened refusal assertions. A small constructor check rejects both/neither.
4. Typecheck both legacy annotated WriterOptions and SemanticWriterOptions,
   reject mixed/neither constructor options and non-CommandReport callbacks,
   prove the callback array and LaneEdit values readonly, and retain all existing
   root-barrel type identities. No separate type harness.
5. Focused writer evals and types green -> pause for root's exact-path source
   checkpoint. Then ordinary headless and package gates once; preserve existing
   compiled Wasm artifacts and do not rebuild/repin on Darwin.

Commands from the SDK repository root (existing artifact directory):

```sh
bash scripts/check-sdk-types.sh
MISO_ENGINE_SDK_ARTIFACTS_HEX=$(node -e 'process.stdout.write(Buffer.from("/private/tmp/dx-393-current-artifacts").toString("hex"))') node --test sdk/test/writer-evals.mjs
bash scripts/check-sdk-headless.sh /private/tmp/dx-393-current-artifacts
bash scripts/sdk-package.sh check /private/tmp/dx-393-current-artifacts
```

The ordinary package gate includes generated-policy and fresh packed-consumer
checks. No new browser matrix, benchmark, harness, or allocator/protocol work.
A fresh independent Astra medium review follows the completed evidence, with
root handling checkpoint pushes and GitHub synchronization.

Matching issue: misofm/engine#445. Root approval: preserve existing WriterOptions interface, add the separate semantic option and one callback dispatch over the same queue.


## Attempt 1 implementation and evidence

Astra medium implemented the approved slice. Root source checkpoint `edfe7431`
contains only `sdk/src/core/writer.ts`, `sdk/test/writer-evals.mjs` and
`sdk/test/barrel-surface.ts`. The existing exported `WriterOptions` interface
remains intact. `SemanticWriterOptions` extends its non-submit options and
provides `submitEdits(readonly LaneEdit[])`; the constructor excludes mixed
modes at both the type and runtime boundaries.

Constructor dispatch normalizes the selected callback once. Only encoded mode
calls `encodeLaneEdits`; both modes continue through the same pending map,
serialized flush chain, batch selection, identity-based removal, adaptive
backpressure split and actual CommandReport handling. No additional writer,
queue, report synthesis, host protocol or app implementation was introduced.

The existing real-engine paused episode now compares semantic async admission,
backpressure, drain outcomes and stats with encoded synchronous/asynchronous
submission. Existing in-flight latest-wins and non-flow refusal/recovery
fixtures run in both modes. The semantic in-flight case queues another flush
before the first report, confirms no early second submission, and observes the
unencoded addressed value on its later callback. Type proofs preserve legacy
annotated/extended WriterOptions while rejecting mixed/neither modes, mutable
semantic batches and callbacks without CommandReport results.

Validation in `/private/tmp/miso-dx-sdk-writer`:

- `bash scripts/check-sdk-types.sh`: PASS, including the existing host mirror
  and root-barrel identities plus semantic/legacy constructor proofs.
- Artifact-backed `node --test sdk/test/writer-evals.mjs`: **16/16 PASS**;
  log `/private/tmp/dx445-writer-focused.log`.
- `bash scripts/check-sdk-headless.sh /private/tmp/dx-393-current-artifacts`:
  **163 PASS, 1 existing skip, 0 failures** (164 tests total);
  log `/private/tmp/dx445-headless.log`.
- `bash scripts/sdk-package.sh check /private/tmp/dx-393-current-artifacts`:
  PASS, including generated-policy checks and the existing fresh packed-consumer
  gate (77 package files); log `/private/tmp/dx445-package.log`.

All gates used the existing reviewed artifact directory. No Rust/Wasm rebuild,
generated artifact edit, dependency/package metadata change, new harness or
browser matrix occurred. README now documents both mutually exclusive callback
modes and their common queue/receipt behavior. App #101 can adopt submitEdits
and remove its already-listed encoded-record bridge independently.

Final README/spec evidence awaits root checkpoint. A dedicated independent
Astra medium review is in progress; no independent verdict is claimed here.

## Independent review and delivery

Dedicated independent Astra medium review records **PASS** at `48c026b8` (source `edfe7431`). Independent type checks, all **16** real-engine writer tests, and ordinary package/generated/fresh-consumer checks pass (77 package files). Review confirms one callback dispatch over the existing queue, unchanged admission/backpressure behavior, semantic edits without encoding, and legacy WriterOptions compatibility. Record: `/private/tmp/dx-445-astra-medium-review.md`. Author headless evidence is 163 PASS with one existing skip.

This completes the SDK submission seam required to remove the app’s encoded-record bridge. App adoption and browser acceptance remain in misofm/app#101. No Rust, ABI, artifact or dependency changes.
