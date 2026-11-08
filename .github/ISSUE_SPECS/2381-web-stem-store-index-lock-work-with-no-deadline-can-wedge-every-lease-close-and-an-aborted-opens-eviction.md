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

**Follow-ups (not filed).**
- A timed-out removal that lands late, after its stem lock is released, can remove a final that a concurrent verify of the same identity has just passed. The pinned session then fails typed at read. The same is true of `#demote`'s and the promote cleanup's unawaited removals. A fence would hold the stem lock until the removal settles, as `#stalledIndexWrite` does for the index.
- A rebuild still treats a stalled hash of a final as damage and removes that final, as base did. The final is good and gets re-downloaded; nothing is lost.
- `#liveLockNames`'s `locks.query()` runs under the index lock with no deadline. It is Web Locks, not OPFS, so it is outside this contract.
