# Web stem store: index-lock work with no deadline can wedge every lease close, and an aborted open's eviction runs on

Found during #2372's reviews. All three items are pre-existing, and none is made worse by #2372.

## Problem
In `hosts/host-web/web/stem-store/opfs-store.js`:
1. `#evictUnpinned` calls `removeEntry` inside its `#mutateIndex` with no deadline, while it holds the index lock. A wedged OPFS `removeEntry` therefore blocks every later index mutation. That includes every lease close's pin release, so `gate.close()` and `lease.close()` hang. This happens on base too.
2. `#readIndex`'s `getFileHandle(INDEX_FILE)` has no deadline under the index lock, unlike the deadlined reads and writes around it. A wedge has the same effect as item 1.
3. `#evictUnpinned` takes no signal, so an aborted open's eviction still runs once its stem lock frees. It holds no pin, so this costs work and is not a correctness issue.

## Contract
- Every OPFS operation made while holding the index lock has a deadline, matching the existing `withDeadline` convention. On expiry the operation fails typed, and the lock is released.
- An aborted open performs no eviction after its abort is observed.

## Gates
- Red on base: one row each for (1) and (2), using the fakes' wedge hooks, asserting that a later `lease.close()` settles within the deadline. One row for (3), asserting that no stem is removed after the abort.
- `node scripts/check-stem-store-v1.mjs` and `--self-test --budgets` green, with a ledger entry per row.

## Evidence / attempt record
### Attempt 1 (Terra, 2026-11-08)
Base `dfc3ac325`; branch `codex/issue-2381-index-lock-deadlines`.

**Claims re-verified on base.** All three hold. Line numbers are base `opfs-store.js`.
- (1) `#evictUnpinned` (821-832) awaits `removeEntry` (827) inside its `#mutateIndex` callback with no deadline.
- (2) `#readIndex` (901-930) awaits `this.#directory.getFileHandle(INDEX_FILE)` (904) with no deadline. Its `getFile()` and `text()` are deadlined.
- (3) `#evictUnpinned` passes `undefined` as the signal to `#withStemLock` (822), and its `#mutateIndex` takes no signal.

**What the audit of the contract added.** The contract covers every OPFS step under the index lock, so the audit went through every one:
- `#rebuildIndex`, which `#readIndex` calls on a missing or damaged index, runs under the lock. Two of its steps had no deadline: the `for await` listing (935) and `removeEntry` of a final that fails to self-verify (955, 958). Its hashing and its index write were already deadlined.
- A deadline expiry in `#readIndex` fell into its catch-all and **rebuilt the index**. That was already true for a stalled `getFile()`. With (2) deadlined but nothing else changed, a wedged index open would have turned into a rebuild. The rebuild resets every pin to `[]`, so a live session's stems become evictable. A stall is not damage, so a stalled read must fail typed and must not rebuild.
- Every other step was already deadlined: the index write, the open-time sweeps through `#sweepStep`, and `#hashFile`. Every other `#mutateIndex` callback is pure JavaScript. `#liveLockNames` is a Web Locks query, not OPFS.

**Fix** (`hosts/host-web/web/stem-store/opfs-store.js`).
- (1) The eviction's removal is wrapped in `withDeadline(..., readDeadlineMs, "storage.remove_stalled")`. On expiry the mutation throws, the index is not rewritten, the row stays, and the index lock is released. The open fails typed `storage.remove_stalled`. If the removal lands later, the row has no file. Verify-on-open treats that as a miss: it demotes the row and re-ingests. The removal stays under the lock, before the row deletion, so a removal failure never leaves an unindexed final behind.
- (2) The index `getFileHandle` is deadlined `storage.read_stalled`. `#readIndex` rethrows a `storage.read_stalled` `StemStoreError` before its repair path, so a stall never rebuilds.
- Rebuild: the listing steps through `iterator.next()` with a deadline (`storage.read_stalled`). Each removal is deadlined (`storage.remove_stalled`). The `finally` releases the iterator without waiting on it, like `#sweepEntries`. A hash failure and a hash mismatch both remove once. Base retried a failed removal of a mismatched final through the catch, and that retry would have doubled the deadline.
- (3) `openSession` passes its signal to `#preflight`, which passes it to `#evictUnpinned`. The eviction takes the signal on the victim's stem lock and on the index lock (`#mutateIndex(mutation, signal)`). Its mutation calls `throwIfAborted(signal)` again after the index read, so an abort that lands during that read still evicts nothing. The estimate and the preflight's index read stay unsignalled, because #2372 attempt 3 removed the estimate's signal on purpose. The first eviction's lock wait refuses an abort from either.
- Fakes: `FakeOpfsBackend` gains `getFileHandleBlocker({ name, create })`. It is awaited only when set, so runs without it keep their microtask order.
- #2372's pin-phase model is unchanged. This fix makes the index work it relies on bounded for every step. `gate.close()` and `session-gate.js` are untouched.

**Red on base** (`hosts/host-web/tests/stem-store-core-v1.mjs`, each test or leg run alone against the base store):
- `aWedgedEvictionNeverHoldsTheIndex`: `a lease close waited on an eviction's wedged removal` (actual `'timed-out'`).
- `aWedgedIndexReadFailsTypedAndKeepsEveryPin`: `a lease close waited on a wedged index open` (actual `'timed-out'`).
- `aWedgedIndexRebuildNeverHoldsTheIndex`: `a lease close waited on a rebuild's wedged listing` and `... wedged removal`.
- `anAbortedOpenEvictsNothing`: `an aborted open evicted a stem (stem lock)`, `(index lock)` and `(index read)`.

**Mutations of the fix.** Each is a new entry in the `--self-test` ledger, and each is `RED` at its target:
- Eviction removal without its deadline: `a lease close waited on an eviction's wedged removal`.
- Index open without its deadline: `a lease close waited on a wedged index open`.
- Drop the `storage.read_stalled` rethrow, so a stall rebuilds: `a stalled index read rebuilt the index and dropped a live pin`.
- Rebuild listing without its deadline: `a lease close waited on a rebuild's wedged listing`.
- Rebuild removal without its deadline: `a lease close waited on a rebuild's wedged removal`.
- `#preflight` gets no signal: `an aborted open evicted a stem (stem lock)`.
- Stem lock without the signal: `the aborted open waited for the stem lock holder`. The mutation's signal check still prevents the eviction, so only the promptness assertion sees this one.
- Index lock without the signal: `the aborted open waited for the index lock holder`.
- Drop the in-mutation `throwIfAborted`: `an aborted open evicted a stem (index read)`.

**Test value.**
- `aWedgedEvictionNeverHoldsTheIndex`: red if an eviction's removal under the index lock has no deadline, which stalls every later lease close. No other test wedges an eviction.
- `aWedgedIndexReadFailsTypedAndKeepsEveryPin`: red if the index open has no deadline, or if a stalled index read rebuilds and drops a live pin. No other test stalls an index read.
- `aWedgedIndexRebuildNeverHoldsTheIndex`: red if a rebuild's listing or removal has no deadline. `indexRecoveryAdoptsSelfVerifyingFinals` rebuilds the index but never wedges it.
- `anAbortedOpenEvictsNothing`: red if an aborted open still evicts once a wait frees (the stem lock, the index lock or the index read), or if it waits on a lock holder after its abort. No other test aborts an eviction.

**Deleted as made useless.** The `eviction behind a held stem lock` leg of #2372's `gateCloseNeverWaitsOnWorkThatHoldsNoLease` (`stem-pump-v1.mjs`). The eviction now takes the open's signal, so the store itself refuses at the abort. The leg can no longer tell a gate that waits for every open (that mutation is held by the `wedged getDirectory` leg) from a correct one. A signal-less eviction is now held by `anAbortedOpenEvictsNothing`.

**Green** at this commit:
- `node scripts/check-stem-store-v1.mjs`: `stem-store-v1 gate: PASS`.
- `node scripts/check-stem-store-v1.mjs --self-test --budgets`: PASS. Every ledger entry is `RED` at its target, including the nine new #2381 entries and the existing ones that anchor on `openSession` text.
- `stem-store-core-v1.mjs`: 8 parallel runs and 3 serial `--budgets` runs, all PASS.
- `stem-pump-v1.mjs`: PASS.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- No Rust and no render path changed, so cargo test, clippy, fmt, doc, the realtime policy tool and a no-bit-moved comparison do not apply. The state is single-threaded JavaScript, so there is no loom model. The suite runs in Node against the fakes, so the two-browser gate does not apply.

**Docs.** `docs/STEM_STORE_V1.md` lists the deadlined index-lock steps and their typed codes, says that a stalled index read never rebuilds, and states the abort rule for evictions.

**Follow-ups.**
- A timed-out removal that lands late, after its stem lock is released, can remove a final that a concurrent verify of the same identity has just passed. The pinned session then fails typed at read. The same is true of `#demote`'s and the promote cleanup's unawaited removals. A fence would hold the stem lock until the removal settles, as `#stalledIndexWrite` does for the index. Fixed in attempt 2.
- A rebuild still treats a stalled hash of a final as damage and removes that final, as base did. The final is good and gets re-downloaded; nothing is lost. Fixed in attempt 2.
- `#liveLockNames`'s `locks.query()` runs under the index lock with no deadline. It is Web Locks, not OPFS, so it is outside this contract. Successor #SUCCESSOR.

**Verdict: FAIL** (Sol, one BLOCKER and one MAJOR).
- BLOCKER: this attempt introduces a new hazard. A timed-out eviction removal releases both locks while the raw `removeEntry` is still in flight. A concurrent open of the victim's own stem can then verify and pin the file before the removal lands, and its `lease.read` fails `stem.read.failed`. Base fenced this by holding both locks for the whole removal. The reviewer's reproducer shows it. Required:
  - never release a stem lock while a removal of its final is unsettled: the row first, then the removal under the stem lock alone, held until the removal settles;
  - the rebuild's removal takes the stem lock with `ifAvailable`;
  - the same fence for `#demote` and the promote cleanup;
  - correct the code comment and the docs claim that verify-on-open re-ingests a late removal.
- MAJOR: a rebuild still treats a stalled hash of a good final as damage. It removes the final, and it holds the index lock for about twice the deadline per stalled final, which contradicts the new docs paragraph.
- MINOR: write a successor spec for the `locks.query()` deadline.

### Attempt 2 (Terra, 2026-11-08)
**Rule.** A final is removed only under its stem lock, and that lock is never released before the removal settles. A timed-out removal may still land, so the caller's deadline bounds only the caller's wait, never the lock. This is the stem-lock counterpart of `#stalledIndexWrite`.

**Mechanism** (`opfs-store.js`).
- `#withStemLock` returns the work's outcome as soon as the work settles. It keeps the lock until every removal fenced during the hold has settled (`#stemFences`, `#releaseOnceSettled`). The lock is built on `#acquireStemLock`, and the local no-Web-Locks fallback keeps its FIFO tails.
- `#holdLock` is the shared Web Locks acquire-and-hold. `#acquireSessionPinLock` now delegates to it with unchanged options. `#acquireStemLock` takes either the signal or `ifAvailable`, never both, because the Web Locks API refuses that combination.
- `#fence(identity, operation)` keeps this store's current hold of that stem lock until the operation settles.
- `#removeFinal(identity, deadline)` is the one way a final is removed. Inside this store's own hold, it fences that hold. Otherwise it takes the stem lock with `ifAvailable`, which never waits, so it is safe under the index lock. If another holder has the lock, it skips the final and returns `false`. If it takes the lock, it holds it until the removal settles.

**Sites.**
- Eviction:
  - The row is deleted first, by a pure-JavaScript mutation under `#mutateIndex(…, signal)` that keeps the signal check.
  - The deadlined removal then runs under the stem lock alone.
  - Past the deadline, the open fails typed `storage.remove_stalled` at once while the stem lock waits for the removal.
  - A removal that fails leaves unindexed debris, which the open-time sweep removes.
  - No OPFS step of an eviction runs under the index lock now.
- `#demote` goes through `#removeFinal`. From verify-on-open and `verify()`, it fences the caller's hold. From `read()`'s failure path, which holds no stem lock, it takes the lock with `ifAvailable`, and it leaves the file to the holder if another holder has the lock. The row deletion is unchanged.
- Promote fallback cleanup: the immediate `removeEntry` and the `abortFallbackWritable` chain are each fenced instead of `void`-ed. The caller still gets its typed error at once. If a fallback writable never settles, its stem lock stays held for the tab's lifetime. That is the rule: nothing else may touch that final while a write to it is unsettled. Every wait on a stem lock is signalled, and #2372's `gate.close()` never waits on a wait that holds no pin.
- The rebuild and the open-time sweep (`#removeUnindexedFinals`) remove through `#removeFinal`. Both run under the index lock with a `liveLocks` snapshot that can be stale, so the `ifAvailable` lock is what protects a final another holder took after the snapshot.
- MAJOR: the rebuild's hash catch rethrows `storage.read_stalled`, as `#readIndex` does. A stalled hash fails the rebuild typed within one deadline and keeps the final. A mismatch and any other hash error still remove the final.
- Unchanged, because neither has the gap:
  - `#ingestAttempt`'s quota-path removal of the final is awaited inside the stem lock;
  - `#promote`'s integrity-failure removal is awaited.
- Fakes:
  - `FakeLockManager.request` honours `ifAvailable`: a lock that is held or queued runs `work(null)`.
  - `FakeOpfsBackend` gains `getFileBlocker({ name })`, awaited only when set.

**Not covered, by reason.** One hold can start a demotion (in `#ensureIngested`'s recheck) and then promote a fresh final while the timed-out removal of the old one is still in flight. That needs an indexed row that appeared after the open's own verify loop, under that loop's own stem lock, and is corrupt. The verify loop demotes a corrupt indexed stem under its own hold, and the ingest waits for that hold's fence. So only another tab's newly indexed corrupt row reaches the case. It is not hardened here.

**Tests** (`stem-store-core-v1.mjs`).
- New `aTimedOutEvictionHoldsTheStemLockUntilItsRemovalSettles`, the reviewer's reproducer as a row. A same-stem open started during the eviction's wedged removal must not settle until the removal lands. It then reads the re-ingested stem.
- New `aFallbackCleanupHoldsTheStemLockUntilItsRemovalsSettle`, two legs. Each leg leaves exactly one of the cleanup's two removals unsettled (the late writable abort, or the immediate removal) and asserts that the stem lock is still held.
- New `aSweepNeverRemovesAFinalWhoseStemLockIsTaken`. A holder takes the debris final's stem lock after the open's `liveLocks` snapshot, during the listing, and the final must survive. A rebuild leg was written and dropped. It removes through the same `#removeFinal`, so the sweep row holds the skip, and the rebuild's own use of `#removeFinal` is held by the lock assertion below.
- `aWedgedIndexRebuildNeverHoldsTheIndex`:
  - its removal leg also asserts that the stem lock is still held after the typed failure;
  - a new `hash` leg stalls the good pinned final's first read and asserts that the final stays and the close fails `storage.read_stalled`.
- `wedgedDemoteDeleteIsBounded`: its second verify of the same stem now must wait while the first verify's timed-out delete is unsettled. The delete then fails and leaves the corrupt final, so the index-write half runs as before. Its old premise, that a second verify could run while the delete was wedged forever, is exactly the hazard.
- `aWedgedEvictionNeverHoldsTheIndex`: the open's typed failure now has its own message. That assertion is the guard of the eviction's deadline.

**Red** (each test or leg run alone; "a1" is the attempt-1 store `767fd0d6b`, "base" is `dfc3ac325`):
- The reproducer row:
  - a1: `an open verified a final that a timed-out eviction was still removing`;
  - base: hangs, exit 13, because base holds both locks for the whole removal with no deadline.
- Demote fence: a1 and base `a verify ran while a demote's delete of its final was unsettled`.
- Fallback legs: a1 and base `... before its writable abort settled` and `... before its immediate removal settled`.
- Sweep skip: a1 and base `a final was removed while another holder had its stem lock`.
- Rebuild removal lock:
  - a1 `a rebuild released a final's stem lock before its removal settled`;
  - base fails earlier at `a lease close waited on a rebuild's wedged removal`.
- Rebuild hash stall: a1 and base `a rebuild removed a good final whose hash stalled`.
- The reviewer's two scratch reproducers on this attempt:
  - `late-evict.mjs`: B stays unsettled until the removal lands, then `B read OK bytes 4096`;
  - `rebuild-stall.mjs`: `lease close: storage.read_stalled after 21 ms`, `good final still on disk: true`.

**Mutation ledger** (`--self-test --budgets`, all `RED` at their targets). The #2381 block now has 20 entries.
- Retargeted from attempt 1:
  - the index open;
  - the stall-rebuild;
  - the rebuild listing;
  - the rebuild removal deadline;
  - the four signal entries.
- New, one per guard:
  - the eviction's raw removal back under the index lock (the original defect form);
  - the eviction removal's deadline;
  - the eviction without its fence;
  - the stem lock released as soon as its work settles;
  - the demotion without its fence;
  - the fallback writable abort unfenced;
  - the fallback immediate removal unfenced;
  - the `ifAvailable` lock released before its removal settles;
  - a final removed although another holder has its lock;
  - the sweep's raw removal;
  - the rebuild's raw removal;
  - the rebuild's stalled hash treated as damage.
- Two older entries anchored on code this attempt reshaped. Both are retargeted and still `RED`:
  - `omit AbortSignal from the session pin lock wait` now mutates the `#holdLock` call;
  - `remove Web Lock single-flight` now also drops the stem lock's Web Lock path.
- `#releaseOnceSettled` deletes its map entry with no ownership check. A new hold of the same stem lock in this store can start only after `release()`, which runs after the delete, so such a check could never fail and would be an unguarded line.

**Green** at the attempt-2 commit:
- `node scripts/check-stem-store-v1.mjs --self-test --budgets`: PASS, 74 entries `RED` at their targets.
- `stem-store-core-v1.mjs`: 8 parallel runs and 3 serial `--budgets` runs, all PASS.
- `stem-pump-v1.mjs`: 4 parallel runs, PASS.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: ok.
- Not applicable: no Rust and no render path changed, so cargo, the realtime policy tool, no-bit-moved and loom do not apply. The suite runs in Node against the fakes, so the two-browser gate does not apply.

**Docs.** The attempt-1 claim that verify-on-open re-ingests a late removal is gone. `docs/STEM_STORE_V1.md` now states:
- the stem-lock removal rule and its fence;
- the `ifAvailable` skip;
- that a stalled hash is not damage;
- that every wait on a fenced lock ends at the waiting open's abort.

**Successor.** #SUCCESSOR covers the `locks.query()` deadline, which the coordinator files.
