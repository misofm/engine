# Web stem store: OPFS awaits under a stem lock have no deadline

Successor of #2381. #2381 deadlined every OPFS step made while holding the stem store's index lock. It decided that a removal of a final has no deadline, and that the stem lock stays held until the removal settles. The other OPFS awaits made while holding a stem lock were out of its scope.

## Problem
In `hosts/host-web/web/stem-store/opfs-store.js`, these awaits run inside a hold of a stem lock (`#withStemLock`) with no deadline. They are not removals of a final:
- `#fileHandle(stemFileName(...))` in `verify()` and in `#verifyIndexed`.
- In `#ingestAttempt`:
  - `removeEntry(this.#staging, stagingName)` before staging;
  - `this.#staging.getFileHandle(stagingName, { create: true })`;
  - the staging cleanup `removeEntry` in its `finally`.
- In `#promote`:
  - `stagingHandle.move(this.#directory, finalName)`;
  - the fallback's `this.#directory.getFileHandle(finalName, { create: true })`.
- The quota path's awaited `removeEntry` of the final in `#ingestAttempt`.

If any of these never settles, the hold never ends, and the stem lock stays held. An open that waits for it waits until its own abort. A `verify()` called with no signal, or a joiner of that ingest, waits forever. The open holding the lock never settles either. It holds no session pin at that point, so `StemSessionGate.close()` is not affected (#2372).

## Contract
- Every OPFS await made in a stem-lock hold that is not a removal of a final has the local-store deadline (`readDeadlineMs`), or the ingest deadline where it is part of an ingest. On expiry it fails typed with the code its neighbours use (`storage.read_stalled`, `storage.write_stalled` or `storage.remove_stalled`).
- An operation that times out but may still land keeps the stem lock until it settles: it is fenced into the hold (`#fence`), as removals of a final already are.
- A removal of a final keeps #2381's rule: it has no deadline, and its lock outlives it.

## Non-goals
- No change to the index-lock deadlines, the `.d.ts`, the render path or Rust.

## Gates
- Red on base: one row per await above, each using the fakes' wedge hooks. Each row asserts that the operation holding the lock settles typed within the deadline, and that the stem lock stays held while the wedged operation is unsettled. Add a `move` hook to the fakes if needed.
- `node scripts/check-stem-store-v1.mjs` and `node scripts/check-stem-store-v1.mjs --self-test --budgets` are green, with a ledger entry per deadline.
- The suite runs in Node against the fakes, so the two-browser gate does not apply.

## Evidence / attempt record
### Attempt 1 (Terra, 2026-11-08)
Base `d060df5cd`; branch `codex/issue-2397-stem-lock-deadlines`.

**Claims re-verified on base.** Every listed await holds. Line numbers are base `opfs-store.js`.
- `verify()` awaits `#fileHandle(stemFileName(identity), false)` (235), and `#verifyIndexed` awaits the same lookup (683). Neither has a deadline.
- `#ingestAttempt`: the staging clear (361), the staging create (362) and the `finally` cleanup (545) are raw awaits. The quota path's removal of the final (533-536) is a raw await with `.catch(() => {})`.
- `#promote`: `stagingHandle.move(...)` (582) and the fallback's `getFileHandle(finalName, { create: true })` (597-599) are raw awaits.

**What the audit added.** One await the spec does not list has the same gap: the removal of a fallback final that fails verification (671), a raw await inside the hold. Every other await inside a stem-lock hold was already deadlined (reads, hashes, the staging writable's steps, the fallback copy) or is index-lock work, which #2381 covers.

**Hazard check, one step at a time.** The question for each step is whether it can still land after its deadline, and so act on that identity once the stem lock is released (#2381).
- The two lookups use `create: false`. They change nothing, so a late one cannot land anything. They are deadlined and **not** fenced. Fencing them would keep the lock held for as long as a wedged lookup lasts, so an unsignalled `verify()` behind it would hang, and that buys nothing.
- The staging clear and the staging cleanup are removals of the staged file `${tabId}-${digest}`. A late one could delete the staged file that a later hold of this tab writes. The staging create could leave a staged file behind that hold. The `move()` writes the final, and the fallback create makes an empty final. Each of these is deadlined and **fenced** into the hold, so the stem lock stays held until the step settles.
- A stall in the staging clear or the staging create fails the attempt before anything is staged. A stall in the cleanup replaces the attempt's error with `storage.remove_stalled`. That is not `stem.ingest.integrity`, so `#ensureIngested` does not retry over the same staged file while the removal is in flight. On the move path the cleanup normally finds nothing to remove. If it stalls even so, an open whose stem was already promoted and indexed fails typed, and the next open verifies the stem as present. On base that open hung.
- The two removals of a final follow #2381's rule. Each goes through `#removeFinal` with no deadline on the removal, fenced into the hold. Only the caller's wait is bounded:
  - The quota path uses `#removeDemoted`. Its wait is deadlined and best-effort, as base's `.catch(() => {})` was, and the quota error stays authoritative. This is the same wait as the demotion's, the deviation #2381 attempt 4 judged sound.
  - After a fallback verification failure, the wait is deadlined with the open's signal, and the open fails `storage.remove_stalled`. Swallowing the stall would turn it into a `stem.ingest.integrity` retry, and that retry's `#promote` would then wait, with no deadline, on the fence of this same removal. The ledger entry "retry a fallback whose removal of its final stalled" holds this.
- Waits use the codes of their neighbours: `storage.read_stalled` for the lookups, `storage.write_stalled` for the creates and the move, and `storage.remove_stalled` for the removals. Every wait except the cleanup's takes the open's or `verify()`'s signal, as its neighbours do. The cleanup takes no signal, like the staging writable's abort, so an abort cannot skip its wait.
- A stalled lookup is a stalled read under #2381's one stall rule. It goes to the same `unreadable` / `#demoteUnreadable` path as a stalled `getFile()`: it demotes only when no other live session pins the stem, and otherwise fails typed. A hard, non-`NotFoundError` lookup error now takes that path too. On base it escaped `openSession` and `verify()` unclassified.

**Fix** (`hosts/host-web/web/stem-store/opfs-store.js`).
- `#fencedStep(hold, operation, code, message, signal)` is `#fence(hold, operation)` followed by `withDeadline(operation, readDeadlineMs, …)`. It adds no new mechanism: it reuses `hold.fences`, which `#withStemLock` already drains before it releases the lock. The five store-changing steps use it.
- The two lookups are wrapped in `withDeadline`, and each sits in the try that leads to the stall rule.
- The quota path calls `#removeDemoted(stem.identity, hold)`. The fallback integrity failure calls `#removeFinal(stem.identity, hold, withDeadline(…, options.signal))`.
- Fakes: `FakeOpfsBackend` gains `moveBlocker({ name })`, which is awaited only when set.
- Not changed: the index-lock deadlines, the `.d.ts`, `session-gate.js`, #2372's pin-phase model, the render path and Rust.

**Tests** (`hosts/host-web/tests/stem-store-core-v1.mjs`). There is one new row, `aStemLockStepNeverOutlastsItsDeadline`, with one leg per step. Each leg wedges exactly that step through the fakes' hooks, and asserts that the wedge was reached and that the call holding the stem lock (an open, or `verify()` with no signal) settles within 500 ms with the step's typed code.
- The seven fenced legs also assert that the stem lock is still held after the failure, and that it is released once the step settles. The legs are: staging clear, staging create, staging cleanup, move, fallback final create, quota removal and integrity removal.
- The two lookup legs use a stem that another session pins, so the stall fails typed (`storage.read_stalled`). They assert that the lock is released while the lookup is still wedged, and that the pinning session still reads.
- Test value: the row is red if any stem-lock step's caller waits on a wedged step with no deadline, or if a timed-out store-changing step releases the stem lock while it can still land. It is also red if a lookup is fenced, so that a wedged lookup holds the lock. No existing test wedges any of these steps: `fallbackWritesObeyAbortAndDeadline` wedges the fallback writable, not the create, and `aFallbackCleanupHoldsTheStemLockUntilItsRemovalsSettle` wedges only the cleanup removals after a failed copy.
- No existing test becomes useless.

**Red on base** (each leg run alone against base `opfs-store.js`, with the new fakes):
- every leg fails `a stem-lock hold waited on a wedged <leg> past its deadline` (actual `'timed-out'`);
- the legs are `verify() lookup`, `verify-on-open lookup`, `staging clear`, `staging create`, `staging cleanup`, `move`, `fallback final create`, `quota removal` and `integrity removal`.

**Mutation ledger** (`scripts/check-stem-store-v1.mjs`, a new #2397 block of 19 entries, each `RED` at its target):
- Per lookup (2 entries each):
  - no deadline, red at `… waited on a wedged <leg> past its deadline`;
  - fenced into the hold, red at `the stem lock's release after the <leg>`.
- Per store-changing step (2 entries each, 5 steps):
  - the raw await of base, red at the deadline assertion;
  - fenced into a throwaway hold, red at `a timed-out <leg> released the stem lock before it settled`.
- The quota removal (2 entries):
  - the raw removal of base, red at the deadline;
  - a deadlined removal outside the hold, red at the lock assertion.
- The integrity removal (3 entries):
  - the raw removal of base;
  - a deadlined removal outside the hold;
  - the stall swallowed, so that the ingest retries, red at the deadline assertion.
- Two existing anchors changed shape, and both are still `RED`:
  - the static check `await stagingHandle.move(this.#directory, finalName)` now matches `stagingHandle.move(this.#directory, finalName)`;
  - `fallbackFinalArm` anchors on the new comment after the fallback-final length check.

**Green** at this commit:
- `node scripts/check-stem-store-v1.mjs`: PASS.
- `node scripts/check-stem-store-v1.mjs --self-test --budgets`: PASS, 120 `RED` lines (101 before, plus 19).
- `stem-store-core-v1.mjs --budgets`: 4 parallel runs, all PASS.
- `stem-pump-v1.mjs`: PASS.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- Not applicable:
  - no Rust and no render path changed, so cargo, the realtime policy tool, no-bit-moved and loom do not apply;
  - the suite runs in Node against the fakes, so the two-browser gate does not apply.

**Docs.** `docs/STEM_STORE_V1.md` now states:
- the deadlined stem-lock steps and their codes;
- which steps are fenced, and why a lookup is not;
- that a stalled staging clear or cleanup ends the ingest rather than retrying it;
- each of the ingest's two removals of a final, with the wait it has.

**Follow-ups (not filed).**
- The staging writable's own steps (`createWritable` in `#openStagingWritable`, `write`, `close`, `abort`) were already deadlined, but they are not fenced. A `close()` that times out and lands after the lock is released could commit stale bytes to this tab's staged file. A later ingest of the same stem in this tab re-hashes the staged file before it promotes, so only a landing between that re-hash and the move gets through. Verify-on-open would then demote the result as corrupt. The fallback writable has the same shape: its fence awaits the writable's creation and its abort, not an in-flight `close()`.
- `read()`'s `getFileHandle` (line 297 on base) has no deadline. It runs under no stem lock, so it is outside this contract, but a wedged lookup there hangs a lease read.
