The #246 integration did the row-by-row comparison of the app's deleted `installed-msf.ts` (1,190 lines of hard-won OPFS lessons) against the vendored stem store. Most lessons are carried, several are stronger. Five are not — all in engine-owned store code:

1. **Undeadlined awaits in the recovery path** (`opfs-store.js:815-820` `#demote`, `894-907` `#writeIndex`): `#demote` is the store's entire self-heal path and awaits `removeEntry` on the very handle that just failed, with no deadline; `#writeIndex` runs undeadlined inside the index lock, so one wedged write blocks every stem in the realm. The old code's rule: never await a delete on a handle that already hung. Everything else in the file is deadline-wrapped — these are the gap.
2. **`detectSharedReadOnlyMode` is called from nowhere**: an unwritable device downloads every stem in full (resolver reads the whole container before `createWritable`) and then fails with a raw TypeError. Detection must run before the network opens.
3. **Open-time maintenance is unbounded and fatal** (`opfs-store.js:110-117`): the three sweeps are awaited inside `open()`, iterate undeadlined, and any throw makes every mix unopenable. Old rule: bounded, best-effort, never fatal.
4. **Per-open full re-hash** (verify-on-open streams every stem on every `openSession`): correct but unmemoized — 8×100 MB = 800 MB hashed per open. A generation-keyed memo that still checks existence + size would keep the gate's intent.
5. **Minor**: no cheap `file.size` comparison before hashing, so truncation costs a full read.

App-side halves (persist(), Clear-Data lock registration, cross-tab progress, decode-stream cancel) are being fixed in the #246 branch. Full comparison table in that branch's verification record.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

## Decision and evidence record (attempt 1, 2026-10-24)

Scope (root, from the scope note): items 1, 2, 3 and 5 in `hosts/host-web/web/stem-store/opfs-store.js`; item 4 goes to a successor.

- Item 1: `#demote` drops the index row first, then removes the final under `withDeadline(readDeadlineMs, "storage.remove_stalled")`, best-effort (a left final is unindexed debris that the next open sweeps). `#writeIndex` deadlines `getFileHandle`, `createWritable`, `write`, `close` and `abort` (`storage.write_stalled`).
- Item 2: `#ingestAttempt` opens the staging writable (deadlined) before `resolver.resolve`; a non-quota, non-abort refusal becomes a typed `storage.*` error and no resolver call happens. `detectSharedReadOnlyMode` stays exported unchanged: removing a public export is an owner API decision, and it is a read-mode probe, not write refusal.
- Item 3: the staging and unindexed-final sweeps iterate with every step deadlined against one overall budget (`maintenanceBudgetMs`, default `readDeadlineMs`, optional in `index.d.ts`). A failure is counted and reported through `hooks.maintenanceFailure({sweep, error, failures})` and never thrown; a failing entry is skipped, a failing listing ends that sweep. `#readIndex(true)` and the pin purge stay authoritative and fatal.
- Item 5: `#verifyIndexed` takes one `getFile()` snapshot; `file.size !== stem.bytes` demotes and reports `corrupt` with `observedBytes: file.size` without hashing. The hash then reads that same snapshot.
- Mutation ledger: the verify-on-open length row now drops both length arms (the pre-check and the post-hash comparison); the warm-filename row's search text follows the new `#hashOpenFile(file, ...)` call.
- Tests (`hosts/host-web/tests/stem-store-core-v1.mjs`), each red on the pre-fix store: `wedgedDemoteDeleteIsBounded`, `unwritableStoreRefusesBeforeNetwork`, `openSurvivesFailingSweeps`, and one assertion in `corruptionSelfHeals` (no `verified-open` chunk for a truncated stem).

Item 4 successor question for the owner (root records it): the store hashes every referenced stem on every session open (sealed by the 'trust warm filename without verify-on-open' mutation). Keep that, or allow a memo (`{bytes, lastModified, verifiedAt}`; re-hash only when size or `lastModified` changed) that gives up same-length silent-rot detection for open time (8 x 100 MB = 800 MB per open today)? Recommendation: keep full verification until an app measures open time as a product problem. Full verification stays meanwhile.

Verifier fold (attempt 1 PASS, root decision M2):
- A timed-out index write step may land later. Until it settles, this store refuses every new index write with `storage.write_stalled`, so a late `close()` cannot overwrite a newer index from this store (test `lateIndexWriteCannotOverwriteANewerOne`). Deviation from root's "rewrite once on settle": no rewrite follows. A late success already committed the newest index of this store (refused writes never mutate disk), a late failure matches the error its caller got, and a rewrite could overwrite an index that another tab wrote after the settle. Stated limit: another tab can still write while this store's write is stalled, and that tab's write can then be overwritten by the late close; preventing that would need the index lock held until the settle, which brings back the realm-wide wedge.
- A demote's delete that lands after its deadline can remove a final that was promoted again. The index row then names a missing file: verify-on-open finds no handle, demotes and re-ingests, and `read()` fails typed (`stem.read.failed`) and demotes. Content is re-resolvable by identity, so no user state is lost.
