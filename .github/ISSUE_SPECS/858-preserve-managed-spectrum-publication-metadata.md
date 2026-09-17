# Preserve managed spectrum publication metadata across nonpublishing reads

## Trigger

The H256 browser mixer receives complete spectrum results every roughly 18 ms, but the SDK can deliver only one public `ready` notification followed by sustained `pending`/`gap` notifications. A nonpublishing pending read overwrites a spectrum job's current metadata after a ready result has advanced its publication sequence but before a slower subscriber's 16 ms delivery deadline. The subscriber therefore receives the waiting ready publication labeled pending and unavailable, while `readLatest()` already contains that completed result. The app rejects it, its bounded retention expires, and the EQ graph blanks or resets.

## Smallest closable product slice

Keep latest native stream metadata separate from an immutable snapshot of the metadata belonging to the latest public spectrum publication. Update that snapshot atomically whenever `publicationSequence` advances for ready, gap, or failed. Build subscriber notification status, metadata, and availability from the publication snapshot so warming and pending reads cannot relabel an undispatched publication.

Initialize the snapshot on fresh job creation and explicitly replace it during collection updates. Keep loss accounting on latest native metadata. Do not change polling cadence, native capture, H256, the bounded gap-recovery read, delivery cursors, close/invalidation behavior, smoothing, result storage, or public types.

## Exact path boundary

- `sdk/src/core/observation-subscriptions.ts`
- `sdk/test/spectrum-evals.mjs`
- this issue spec

## Objective gates

- H256 capture with 16 ms delivery preserves a completed ready publication when subsequent reads are pending.
- `gap -> ready -> pending` delivers ready/available with metadata matching `readLatest()`, accrues native loss once, and preserves skipped-publication accounting.
- `ready -> gap -> pending` delivers gap/unavailable.
- `failed` with a new capture epoch followed by pending/warming stays failed/unavailable; an old ready result never resurfaces, and a new-epoch ready recovery has matching identity and reset loss accounting.
- Initial warming/pending creates no fictitious publication.
- Collection target or smoothing replacement cannot retain the previous job's result or publication metadata.
- Existing same-job update, duplicate-publication, one-in-flight, automatic recovery, and close-during-recovery behavior stays green.
- Focused SDK tests, SDK type/generated gates, and proportional repository gates pass.
- A packaged H256 browser probe after release reports continuing fresh ready display publications, no pending-caused blank/reset, and no downward-rate violation outside a genuine lifecycle change.

## Evidence and decision record

Fresh Astra xhigh review confirmed the defect at engine `0b3fe87d` and evaluated the proposed publication-metadata separation in memory. The existing automatic gap-recovery fixture already contains `gap -> ready -> pending`; adding a ready-status assertion fails before the change and passes with the snapshot separation while its loss assertions and collection-update coverage remain green. A no-stall packaged browser trace produced 178 distinct complete results at median 18.03 ms spacing, but only 1 ready public notification among 230 because later pending metadata relabeled undispatched publications.

The release dependency chain is explicit: publish an Engine SDK patch, publish a dependency/provenance-only web-adapter patch because adapter 0.5.11 pins Engine 0.4.1, then update the app to both matching packages and rerun the packaged browser trace. App issue #262 remains a bounded defensive retention fix and is not sufficient without this correction. Native capture loss and any later presentation-envelope redesign remain separate issues.
