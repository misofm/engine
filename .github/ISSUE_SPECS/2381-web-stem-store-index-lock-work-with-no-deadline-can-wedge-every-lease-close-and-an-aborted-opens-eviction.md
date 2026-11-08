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
- `#liveLockNames`'s `locks.query()` runs under the index lock with no deadline. It is Web Locks, not OPFS, so it is outside this contract. Successor #2396.

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

**Successor.** #2396 covers the `locks.query()` deadline, filed by the coordinator.

**Verdict: FAIL** (Sol, one BLOCKER and one MAJOR). Both earlier reproducers are fixed. The two new hazards predate this branch, but both fall within the claim that a final is removed only under its stem lock.
- BLOCKER: `#removeFinal` asked `#stemFences.has(identity)`. That means "some hold in this store is live", not "the caller is inside a hold". So `read()`'s failure demotion, the rebuild and the sweep removed at once during another same-store hold, and fenced into that hold. A late landing then deleted the final the other hold had just promoted (reproducer `same-store-bypass`).
- MAJOR: the "not hardened" recheck case is reachable without corruption. A stalled read in `#verifyIndexed` demotes, the demotion's removal stalls, and the same hold then promotes a fresh final that the late removal deletes (reproducer `recheck-stall`).
- MINORs:
  - `FakeLockManager`'s `ifAvailable` was not faithful;
  - deleting the local fallback's `ifAvailable` line left every suite green;
  - a same-store waiter on a draining lock waited silently;
  - a test comment still said the eviction removal runs under the index lock;
  - keep the successor placeholder (now #2396).

### Attempt 3 (Terra, 2026-11-08)
**Explicit holds.**
- `#withStemLock` passes `work` a hold token, `{ identity, fences }`. The token is threaded through `#verifyIndexed`, `#demote(identity, hold)`, the eviction, `#ingestAttempt` and `#promote`, which uses it for both fallback-cleanup fences.
- `#removeFinal(identity, hold, deadline)` fences into `hold` only when the caller passes one. Otherwise it always takes the stem lock with `ifAvailable`. A hold of the same store is a held lock like any other, so a holdless removal leaves that hold's final alone.
- The ambient `#stemFences` map is gone. In its place, `#stemDrains` records only the holds whose work has ended and that are waiting on fences.

**The recheck.** Two changes, one for each way into it:
- `#promote` first waits up to `readDeadlineMs` for `hold.fences`. On expiry it fails typed `storage.remove_stalled`, and the lock drains. This covers a recheck that demotes a corrupt row.
- `#verifyIndexed` rethrows `storage.read_stalled` instead of demoting. That is the stall-is-not-damage rule `#readIndex` and the rebuild already follow. In the reviewer's `recheck-stall`, the demotion it removes would also have dropped the other tab's pinned row, and once the removal landed, deleted the file that tab plays from. The open now fails `storage.read_stalled` and the stem is untouched.

**Same-store waiters (MINOR 5).** `#waitForStemLock` gives every same-store stem-lock wait its own abort controller, linked to the caller's signal. When a hold of that lock starts draining, every registered waiter is notified, including a waiter that registers while the drain is already running. A notified waiter aborts its own wait one deadline later with `storage.remove_stalled`. This covers `verify()` with no signal, an open's verify loop and its ingest. An `#inFlight` joiner awaits the ingest's promise, which settles when its work ends, so it is never held by a drain itself. A waiter in another tab sees only a held lock. `docs/STEM_STORE_V1.md` states that its wait ends at its own signal or at the release.

**No-Web-Locks fallback (MINOR 4).**
- The local stem-lock queue now honours the signal. An abandoned place opens its gate, so the queue behind it waits only for the place before it.
- Each place deletes its own tail entry once its turn has passed. Before, a stale tail left by an aborted last waiter would have made every later `ifAvailable` answer "held".
- `ifAvailable` keeps its no-wait answer, and the new holdless-demotion row guards it.

**Fakes (MINOR 3).** `FakeLockManager.request`:
- with `ifAvailable`, tests `#held` and `#pending`, not `#tails`;
- rejects `signal` together with `ifAvailable` with `NotSupportedError`;
- runs the null callback from a queued task.

**Tests** (`stem-store-core-v1.mjs`).
- New `aHoldlessDemoteLeavesAFinalAnotherHoldOwns`, the `same-store-bypass` shape, with a Web Locks leg and a no-Web-Locks leg.
  - A read failure's demotion runs while another hold of this store re-ingests the stem. It must neither remove the final nor wait for that hold.
  - Once every hold is gone, the next read failure's demotion must remove the file.
- New `aHoldNeverWritesAFinalItIsStillRemoving`, the recheck through corruption. Another tab indexes a corrupt row after the open's verify loop, and the recheck's demotion removal wedges. The open must fail `storage.remove_stalled`.
- New `aStalledVerifyNeverDemotesAStem`, the `recheck-stall` shape reduced to one store. A stalled verify-on-open read fails the open `storage.read_stalled`, keeps the other session's pin, re-downloads nothing, and the pinned session still reads.
- `aTimedOutEvictionHoldsTheStemLockUntilItsRemovalSettles` now has three legs:
  - another tab, which waits and then re-ingests;
  - the same tab, which refuses typed within the deadline;
  - the same tab with no Web Locks, which does the same.

  After the removal lands, each leg must get the lock: the waiting open in the other tab, a fresh open in the same tab. That proves the local queue passes an abandoned place.
- `wedgedDemoteDeleteIsBounded`: the second verify of the same stem must now refuse typed (`storage.remove_stalled`), not wait silently.
- The comment on `aWedgedEvictionNeverHoldsTheIndex` is reworded (MINOR 6).

**Red** (each test run alone; "a2" is the attempt-2 store `3315395f8`, "base" is `dfc3ac325`):
- `aHoldlessDemoteLeavesAFinalAnotherHoldOwns`: a2 and base `a demotion with no hold removed a final another hold of this store had (Web Locks)`.
- `aHoldNeverWritesAFinalItIsStillRemoving`: a2 and base `a hold wrote a final while its own removal of that final was unsettled`.
- `aStalledVerifyNeverDemotesAStem`: a2 and base `a stalled verify demoted a pinned stem`.
- `wedgedDemoteDeleteIsBounded`:
  - a2 `a verify waited silently on a demote's unsettled delete`;
  - base `a verify ran while a demote's delete of its final was unsettled`.
- `aTimedOutEvictionHoldsTheStemLockUntilItsRemovalSettles`:
  - a2 `a same-store open waited silently on an unsettled removal (same tab)`;
  - base hangs, exit 13.
- The reviewer's reproducers on this attempt:
  - `same-store-bypass`: the demotion's removal never starts while B holds the lock, the final survives, and `B read OK bytes 4096`;
  - `recheck-stall`: `A open failed: storage.read_stalled`;
  - `rebuild-stall`: `storage.read_stalled after 21 ms`, final kept;
  - `late-evict`: B is in the same store, so it now refuses typed `storage.remove_stalled` instead of waiting. The script does not catch that rejection.

**Mutation ledger** (`--self-test --budgets`, 83 lines `RED` at their targets). The #2381 block has 29 entries, one per guard.
- New in attempt 3:
  - a holdless removal fenced into whichever same-store hold is live, which restores the ambient check;
  - the promote fence guard;
  - the stalled verify-on-open treated as damage;
  - a new same-store waiter not told of a running drain;
  - queued same-store waiters not notified when a drain starts;
  - the local wait ignoring its signal;
  - an abandoned local place left blocking the queue;
  - a finished local place never dropped;
  - a local `ifAvailable` request that waits.
- Attempt-2 entries retargeted to the new signatures.
- Two older anchors retargeted for the new signatures, both still `RED`:
  - `insert index before promote verification`;
  - the verify-on-open length arm, which now reads `#demote(stem.identity, hold)`.
- Equivalent, by reason: `#waitForStemLock`'s `finally` cleanup (`clearTimeout`, the forward removal, the waiter deletion). A timer that fires after the grant aborts a controller whose request has already been granted, which neither Web Locks nor the local queue observes. A stale notifier only starts such a timer. These are leaks with no visible effect.

**Green** at the attempt-3 commit:
- `node scripts/check-stem-store-v1.mjs`: PASS.
- `node scripts/check-stem-store-v1.mjs --self-test --budgets`: PASS.
- `stem-store-core-v1.mjs`: 8 parallel runs and 3 serial `--budgets` runs, all PASS.
- `stem-pump-v1.mjs`: 4 parallel runs, PASS.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: ok.
- Not applicable: no Rust and no render path changed. The suite runs in Node against the fakes.

**Docs.** `docs/STEM_STORE_V1.md` states:
- that a stalled verify-on-open read never demotes;
- the in-hold write rule;
- that a removal belongs to a hold only when its caller is inside it;
- the same-store waiter's typed refusal, and the cross-tab wait;
- the local fallback's queue.

**Verdict: FAIL** (Sol, four MAJORs; holds are now right, so there is no BLOCKER). Reproducers are in `repro3/`.
- (1) A holdless `ifAvailable` removal holds the lock without registering a drain, so `verify()` and an unsignalled open hang silently.
- (2) The drain timer is never cancelled when the drain ends, so a waiter refuses spuriously behind a later, legitimate hold.
- (3) Stall-is-not-damage in `#verifyIndexed`, the rebuild and `verify()` means a final whose reads always stall is never healed, and every open fails forever. Base re-downloaded it.
- (4) `read()`'s failure path still demotes on a stall, which contradicts the new docs.
- Minors:
  - (5) a waiter timer of 0 survives every test;
  - (6) `#verifyIndexed`'s try also covers the `lastUsedAt` mutation;
  - (7) OPFS awaits run under the stem lock with no deadline, which predates this branch (successor);
  - (8) the successor placeholder (now #2396).
- The coordinator rescoped attempt 4 to a simpler model that binds it. Its points are:
  - (A) the index-lock contract;
  - (B) one removal primitive with no deadline and no drains;
  - (C) one stall-versus-damage rule outside the index lock;
  - (D) tests and ledger for exactly that;
  - (E) a successor for (7).

### Attempt 4 (Terra, 2026-11-08)
**A. Index lock.** Unchanged from attempt 3:
- the index open, read and write deadlines;
- the `#readIndex` and rebuild-hash stall rethrows;
- the rebuild listing deadline;
- the eviction deleting its row first, with no OPFS step under the index lock.

The rebuild's and the sweep's removals run under the index lock, so the caller's *wait* for them keeps its deadline: `withDeadline`, and `#sweepStep` for the sweep. The removal itself does not, and its stem lock outlives it.

**B. One removal primitive.**
- Deleted: `#stemDrains`, `#stemWaiters`, `#waitForStemLock`, the drain notification, the waiter timers and the `remove_stalled` refusals.
- `#withStemLock(identity, signal, work, ifAvailable)` acquires the lock, runs `work(hold)`, and releases only after `settled(hold.fences)`. With `ifAvailable` it returns `false` when the lock is held or queued for.
- `#removeFinal(identity, hold, wait)`:
  - fences the removal into `hold`;
  - with no `hold`, wraps itself in `#withStemLock(…, ifAvailable)`;
  - waits for the removal through `wait` only.
- The eviction waits through `abortable(removal, signal)`.
- `#promote` waits plainly for `settled(hold.fences)`.
- Every stem-lock wait, same-store or cross-tab, ends at the lock's release or at the waiter's own abort. The no-Web-Locks queue honours the signal, and its abandoned and finished places clean up.

**One deviation from B, with evidence.** A demotion's own wait keeps base's best-effort deadline (`withDeadline(…).catch(() => {})`). The removal itself has no deadline and keeps the lock until it settles. With the wait abortable-only, base's own row `wedgedDemoteDeleteIsBounded` goes red: `a demote awaited a wedged delete without a deadline`. In that run, `verify()` called with no signal hangs on the wedged delete. `read()`'s failure path would hang the same way. This is not new machinery. It is base's existing wait, and the ledger now holds it with an entry.

**C. Stall versus damage outside the index lock.**
- `#demoteUnreadable(identity, hold, ownPin, error)` re-checks the row under the index lock. It demotes, and removes the file through `#removeDemoted`, only when no pin other than `ownPin` holds the row. Otherwise:
  - a `storage.read_stalled` error is rethrown as is;
  - any other error is wrapped in `storage.read_stalled`;
  - the row and the file stay.
- The rule is used for:
  - verify-on-open's read and hash;
  - `verify()`'s hash;
  - `read()`'s failure path. Here `StemSessionLease` passes its own pin as `read(identity, { ownPin })`, an internal option outside the `.d.ts`, like `onPinning`. A missing file is still damage and demotes unconditionally.
- Verify-on-open has no own pin: same-session reopen plumbing was written, and dropped as unguarded.
- Corruption and a wrong length still demote unconditionally.
- `#verifyIndexed`'s try now covers only the `getFile` and the hash (finding 6). The hash line keeps its `const observed = await this.#hashOpenFile(...)` shape, which an existing static check pins: it now ends in `.catch(unreadable)`.

**D. Tests** (`stem-store-core-v1.mjs`).
- Kept unchanged:
  - the index rows;
  - the rebuild legs: listing, removal with the lock held, and hash (`rebuild-stall`);
  - the sweep skip;
  - both fallback-fence legs;
  - `aHoldlessDemoteLeavesAFinalAnotherHoldOwns` (`same-store-bypass`);
  - `anAbortedOpenEvictsNothing`.
- Adapted:
  - `wedgedDemoteDeleteIsBounded`: the second verify waits while the delete is unsettled.
  - `aWedgedEvictionNeverHoldsTheIndex`: the evicting open's wait ends at its abort.
  - `anEvictionHoldsTheStemLockUntilItsRemovalSettles` (`late-evict`), three legs: another tab, the same tab, and the same tab with no Web Locks. The evicting open is aborted, and the B open must stay pending until the removal lands, then read. The other store opens first, so its own debris sweep does not mask the mutation.
  - `aHoldNeverWritesAFinalItIsStillRemoving`: the recheck waits for its own removal, then promotes and reads.
  - `aStalledVerifyKeepsAPinnedStem` (`recheck-stall`).
- New:
  - `aStallingUnpinnedFinalHeals` (r2), with open and `verify()` legs;
  - `aStalledReadKeepsAStemAnotherSessionPins` (r3);
  - `aHoldlessRemovalKeepsTheStemLockUntilItSettles` (r1, now a wait that ends at abort), with Web Locks and no-Web-Locks legs;
  - `aFailedLastUsedWriteNeverDemotes` (finding 6).
- Removed: every drain-refusal leg and assertion.

**Red** (each test run alone; "a3" is the attempt-3 store `55de84185`, "a2" is `3315395f8`, "base" is `dfc3ac325`):
- `aStallingUnpinnedFinalHeals`:
  - a3 `never healed (open)`;
  - a2 and base `a stalled verify() kept an unpinned final it cannot read`.
- `aStalledReadKeepsAStemAnotherSessionPins`: a3, a2 and base `a stalled read demoted a stem another session pins`.
- `aStalledVerifyKeepsAPinnedStem`: a2 and base `a stalled verify demoted a pinned stem`. Green on a3, which never demoted on a stall.
- `aHoldlessRemovalKeepsTheStemLockUntilItSettles`:
  - base `a verify ran while a holdless removal of its final was unsettled (Web Locks)`;
  - a2 `a wait for a removing stem lock ignored its abort (no Web Locks)`;
  - green on a3, whose waits also ended at abort.
- `aFailedLastUsedWriteNeverDemotes`: red on a3, a2 and base. They demote and re-download on the failed write.
- `aHoldNeverWritesAFinalItIsStillRemoving`: a3, a2 and base `a hold wrote a final while its own removal of that final was unsettled`.
- `wedgedDemoteDeleteIsBounded`: a3 and base `a verify ran while a demote's delete of its final was unsettled`.
- `anEvictionHoldsTheStemLockUntilItsRemovalSettles`:
  - a3 and a2 reject `remove_stalled` instead of `AbortError`;
  - base hangs, exit 13.
- `aWedgedEvictionNeverHoldsTheIndex`:
  - base `a lease close waited on an eviction's wedged removal`;
  - a2 and a3 reject `remove_stalled`, not `AbortError`.
- The reviewer's reproducers on this attempt:
  - r1: `verify()` and an unsignalled open now wait, by the model;
  - r2: every open succeeds, and `verify()` returns true after the heal;
  - r3: `a.read` fails `storage.read_stalled`, the final is kept, and `b.read` is OK;
  - r4: no spurious refusal; L and W both return `false`;
  - `same-store-bypass`: the final is kept, and B reads OK;
  - `recheck-stall`: A fails `storage.read_stalled`;
  - `rebuild-stall`: `read_stalled` after 21 ms, final kept;
  - `late-evict`: B stays pending until the removal lands, A completes, and B reads OK.

**Mutation ledger** (`--self-test --budgets`: PASS, 87 `RED` lines). The #2381 block has 33 entries, one per guard of this model. Each is `RED` at its target:
- A (6 entries):
  - the raw eviction removal under the index lock;
  - the index open deadline;
  - the stalled-read rebuild;
  - the listing deadline;
  - the rebuild removal deadline;
  - the rebuild hash stall.
- B (17 entries):
  - the eviction wait ignoring its abort;
  - an evicted final removed outside the hold;
  - the lock released as soon as its work settles;
  - a demoted final removed outside the hold;
  - the demotion wait with no deadline;
  - the two fallback fences;
  - the ambient-hold check restored;
  - the holdless lock released early;
  - a taken lock removed anyway;
  - the sweep's raw removal;
  - the rebuild's raw removal;
  - the promote fence;
  - the local wait ignoring its signal;
  - the abandoned local place;
  - the finished local place never dropped;
  - the local `ifAvailable` wait.
- C (6 entries):
  - a pinned unreadable stem demoted;
  - stalled verify-on-open never demoting;
  - the reader's own pin protecting its stem;
  - `read()` demoting on any failure;
  - `verify()` never demoting on a stall;
  - the `lastUsedAt` failure demoting.
- Aborted evictions (4 entries):
  - the preflight signal, now red at its earlier gate `an eviction's wait for its removal ignored the open's abort`;
  - the stem-lock signal;
  - the index-lock signal;
  - the in-mutation check.
- Removed with the machinery: the drain, notification, waiter-timer and refusal entries.
- Retargeted older anchors, all still `RED`:
  - `trust warm filename without verify-on-open`;
  - the two verify-on-open length arms, whose indentation changed.

**Green** at the attempt-4 commit:
- `node scripts/check-stem-store-v1.mjs`: PASS.
- `node scripts/check-stem-store-v1.mjs --self-test --budgets`: PASS.
- `stem-store-core-v1.mjs`: 8 parallel runs and 3 serial `--budgets` runs, all PASS.
- `stem-pump-v1.mjs`: 4 parallel runs, PASS.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: ok.
- Not applicable: no Rust and no render path changed. The suite runs in Node against the fakes.

**Docs.** `docs/STEM_STORE_V1.md` describes exactly this model:
- the index-lock deadlines;
- the stall rule;
- the single removal rule, with each caller's wait;
- that stem-lock waits end at release or abort;
- the local queue.

**Successors.**
- #2396: the `locks.query()` deadline.
- #2397: OPFS awaits with no deadline inside a stem-lock hold (finding 7).

Both are filed, and their specs are on this branch.

**Verdict: FAIL** (Sol, one MAJOR, no BLOCKER). There is no data loss, no deadlock and no index wedge, and #2372's bound holds. The demotion-wait deviation is judged sound.
- MAJOR, a regression against base: `#demoteUnreadable` refused whenever any pin other than `ownPin` existed. That included `offline:` keep-offline pins, and dead session pins without Web Locks, so a final whose reads always fail never healed (r7). Base healed it. The refusal also labelled a hard read error `storage.read_stalled`, "another session pins it".
- MINORs:
  - `#evictUnpinned`'s `if (evicted)` is an unguarded data-loss guard (r5);
  - `#promote`'s fence wait ignored the abort (r6);
  - three new lines survived their mutants: `verify()`'s abort check, `read()`'s `NotFoundError` arm and the `read_stalled` wrap;
  - the successor placeholders.

### Attempt 5 (Terra, 2026-11-08)
Exactly the verdict's changes, with no new machinery.

**Fixes** (`opfs-store.js`).
- `#demoteUnreadable` protects only live session pins: `row?.pins.some((pin) => pin.startsWith("session:") && pin !== ownPin)`.
  - A keep-offline pin no longer blocks the heal. It is lost with the row, as on base. Carrying it into the re-ingested row would need state that outlives the deleted row across tabs, which is not cheap, so the docs say it is lost.
  - A dead session pin still blocks healing until a store open purges it. Without Web Locks nothing purges a crashed tab's pin. The docs say so, and r7's `nolocks-crashed` mode shows it (every open fails `stem.read.failed`).
- A refused demotion now throws the read's own typed error through `classifyStorageError`: `storage.read_stalled` stays as it is, and a hard read error becomes `stem.read.failed`. The `read_stalled` "another session pins it" wrap is gone.
- `#promote` waits with `await abortable(settled(hold.fences), options.signal)`.

**Rows** (`stem-store-core-v1.mjs`).
- `aStallingUnpinnedFinalHeals` gains a `keep-offline pin` leg (r7).
- `aStalledVerifyKeepsAPinnedStem` gains a `read error` leg. A hard read error of a stem another session pins fails with `stem.read.failed` and keeps the row, the pin and the file. This guards the error-class change.
- `aHoldNeverWritesAFinalItIsStillRemoving` gains an `aborted` leg (r6).
- New `anAbortedReadNeverDemotes`, with `verify()` and open legs. It guards `verify()`'s new abort check, and the pre-existing one in verify-on-open.
- New `aMissingFinalIsDemotedEvenWhenPinned`. It guards `read()`'s `NotFoundError` arm.
- New `anEvictionSparesAVictimPinnedAfterItsPreflight`, the r5 shape. It guards `if (evicted)`.

**Red.**
- On the attempt-4 store `8cd7bc2f1`:
  - the `keep-offline pin` leg: `a stalling unpinned final never healed (keep-offline pin)`;
  - the `read error` leg: `a refused demotion misreported its read failure (read error)`;
  - the `aborted` leg: `a hold's wait for its own removal ignored the open's abort`.
- On base `dfc3ac325`:
  - the `read error` leg: `a stalled verify demoted a pinned stem (read error)`;
  - the `aborted` leg: `a hold wrote a final while its own removal of that final was unsettled`.
  - The `keep-offline pin` leg is green on base, which healed. This is the regression row against attempt 4.
- `anAbortedReadNeverDemotes`, `aMissingFinalIsDemotedEvenWhenPinned` and `anEvictionSparesAVictimPinnedAfterItsPreflight` guard behaviour that attempt 4 and base already had, so they are green on both. Their ledger mutations are red, below.

**Mutation ledger** (`--self-test --budgets`: PASS, 94 `RED` lines). The #2381 block has 40 entries, all `RED` at their targets. New:
- the promote wait ignoring the abort;
- `if (evicted)` → `if (true)`, red at `an eviction removed a final that was pinned after its preflight`;
- a keep-offline pin blocking the heal;
- a refused hard read error reported as a stall;
- `verify()`'s abort check dropped;
- verify-on-open's abort check dropped;
- `read()`'s `NotFoundError` arm dropped.

Retargeted, all still `RED`:
- the pin-rule entries, which now search the `session:` form;
- "let a stalled verify-on-open keep an unpinned final", which now goes red first at the new `read error` leg.

The two surviving mutants the verdict called pre-existing:
- the verify-on-open abort check: now guarded by its own entry, above;
- `#withStemLock`'s `throwIfAborted(signal)` after the grant: kept from base's `#withLock`. A signal that aborts between the grant and the work is checked again by every signalled wait inside the work, so it is recorded as pre-existing and not hardened here.

**Reproducers on this attempt:**
- r5: B reads OK, and the victim's final is kept.
- r6: the aborted open settles at once.
- r7 `offline`: every open succeeds, and `verify()` returns true.
- r7 `nolocks-crashed`: `stem.read.failed` throughout. This is the documented limitation.
- r1 to r4: unchanged from attempt 4.
- `late-evict`, `rebuild-stall`, `same-store-bypass` and `recheck-stall`: unchanged from attempt 4.

**Green:**
- `node scripts/check-stem-store-v1.mjs`: PASS.
- `node scripts/check-stem-store-v1.mjs --self-test --budgets`: PASS.
- `stem-store-core-v1.mjs`: 8 parallel runs and 3 serial `--budgets` runs, all PASS.
- `stem-pump-v1.mjs`: 4 parallel runs, PASS.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: ok.

**Docs.** `docs/STEM_STORE_V1.md` now states:
- that only another session's pin blocks a demotion;
- that keep-offline pins are lost, as on base;
- the dead-pin limitation without Web Locks;
- that an abort never demotes;
- the error each refused call reports;
- that the promote wait ends at the open's abort.

**Verdict: PASS** (Sol). Both documented limitations are accepted:
- keep-offline pins are lost with a demoted row, as on base;
- without Web Locks, a crashed tab's dead session pin blocks a heal. This is an accepted limitation of a configuration that is nearly unreachable: OPFS without Web Locks.

Two cosmetic minors are folded into the same branch:
- The ledger row "let a stalled verify-on-open keep an unpinned final" rethrows with a typed `classifyStorageError(error, "x", { code: "stem.read.failed" })`, which keeps the error class. It goes red at `a stalling unpinned final never healed (open)` again.
- The bodies of the new leg loops in `aHoldNeverWritesAFinalItIsStillRemoving` and `aStalledVerifyKeepsAPinnedStem` are re-indented, with whitespace-only changes.

After merging `origin/main`, `node scripts/check-stem-store-v1.mjs` and `node scripts/check-stem-store-v1.mjs --self-test --budgets` both pass.
