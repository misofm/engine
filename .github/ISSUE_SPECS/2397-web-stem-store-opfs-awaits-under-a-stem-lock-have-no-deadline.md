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

**Verdict: FAIL** (Sol, one BLOCKER and three MINORs). Every step this attempt fenced was judged correct.
- BLOCKER, in scope by Contract bullet 2 and the #2381 hazard. The staging writable's `write()`, `close()` and `abort()`, and `#openStagingWritable`'s `createWritable()`, are awaited in the hold but not fenced. With a wedged `closeBlocker`, the open fails `storage.write_stalled` and the stem lock is released. A second same-tab `openSession` then takes the lock and completes while the first `close()` is still in flight. The reviewer's reproducer is `close-repro.mjs`, in modes `staging` and `fallback`. Required:
  - fence `write`, `close` and `abort` with `#fencedStep`, with `abort` keeping its no-signal form;
  - fence `createWritable` with a cleanup that aborts a writable that arrives late, as `abortFallbackWritable` does;
  - add a `closeBlocker` leg, and "unfenced close" and "unfenced abort" ledger entries.
- MINOR: fence the fallback writable's `write` and `close` directly in `#promote`. Do not rely on Streams' abort waiting for a pending close, which the fake does not model; the `fallback` mode of the reproducer shows the gap. The follow-up note was wrong to say real browsers are exposed: they are covered, implicitly.
- MINOR: a stalled staging cleanup in `#ingestAttempt`'s `finally` throws `storage.remove_stalled` and replaces the attempt's own error. That loses `storage.insufficient`, and it also fails an open that already succeeded. Rethrow only when the pending error is `stem.ingest.integrity`, which stops the retry. Keep the original error otherwise, and swallow the stall on success. The removal stays fenced either way. Add a row and a ledger entry.
- MINOR: write the `read()` lookup follow-up as a successor spec, and fix the docs nit "a stalled or failed lookup".

### Attempt 2 (Terra, 2026-11-08)
Folded on the same branch, over `2dd630040`.

**Fixes** (`opfs-store.js`).
- Staging writable (BLOCKER):
  - `write()`, `close()` and `abort()` go through `#fencedStep`. `abort()` keeps its no-signal, best-effort wait.
  - `#openStagingWritable(handle, stem, options, hold)` keeps the `createWritable()` promise. If its wait fails (a deadline, an abort or an error), the hold fences `Promise.resolve(opening).then((late) => late?.abort(error))`, so a writable that opens late is aborted before the stem lock is released.
- Fallback writable (MINOR 2): `write()` and `close()` go through `#fencedStep`. The existing fences on the late abort and the removals are unchanged.
- Staging cleanup (MINOR 3): `#ingestAttempt` records its caught error as `failure`, and the `finally` removal is still fenced. A failed or stalled cleanup is rethrown only when `failure` is `stem.ingest.integrity`, whose retry would stage the same file under it. Otherwise the attempt's own outcome stands: its error, or on success the result, with the stall swallowed. A later ingest of the stem clears the staged file first.
  - The verdict's "keep the original error" is read for hard cleanup errors as well as stalls. That makes one rule, and leaves no line no test can see. On base, a hard cleanup error replaced the attempt's outcome.

**Tests** (`stem-store-core-v1.mjs`).
- `aStemLockStepNeverOutlastsItsDeadline` now has 17 legs. New:
  - `staging writable open`, `staging write`, `staging close` (the `closeBlocker` leg), `staging abort` (a failing resolver, then a wedged abort), `fallback write` and `fallback close`, each fenced;
  - `staging cleanup (integrity)`: corrupted bytes, then `storage.remove_stalled` with no retry;
  - `staging cleanup (quota)`: `storage.insufficient` is kept;
  - `staging cleanup (success)`: the open succeeds, and the lock is still held until the removal settles.
  - The attempt-1 `staging cleanup` leg is replaced by these three.
- Test value of the new legs: red if a writable step of either writable, or a late-opening staging writable, releases the stem lock while it can still land; or if a stalled cleanup replaces a quota error or fails a successful open; or if it stops failing after an integrity failure, which lets the retry run over it. No other test wedges these steps with the lock observed.
- `aFallbackCleanupHoldsTheStemLockUntilItsRemovalsSettle`, `immediate removal` leg: the fallback write is now fenced, so a write wedged forever kept that leg's lock held and broke its "exactly one step unsettled" premise. Its write now stalls past the deadline and fails 40 ms later, and the lock check runs at 60 ms, so the immediate removal is again the only step left unsettled.

**Red.**
- On the attempt-1 store `2dd630040`, each leg run alone:
  - `staging writable open`, `staging write`, `staging close`, `staging abort`, `fallback write` and `fallback close` fail `a timed-out <leg> released the stem lock before it settled`;
  - `staging cleanup (quota)` fails `failed with the wrong error`, with actual `storage.remove_stalled`, expected `storage.insufficient`;
  - `staging cleanup (success)` fails the same way, with actual `storage.remove_stalled`, expected none;
  - `staging cleanup (integrity)` is green on attempt 1, which rethrew every stall. It guards the integrity clause, and its ledger entries are red.
- On base `d060df5cd`, every one of the 17 legs is red:
  - the writable legs at the lock assertion;
  - every other leg at `waited on a wedged <leg> past its deadline`.
- The reviewer's reproducer, both modes: `stem lock held while the <mode> close() is still in flight: true`, and `second open waited`. Before this attempt it printed `false` and `second open completed`.

**Mutation ledger.** The #2397 block now has 28 entries, each `RED` at its target. `--self-test --budgets` passes with 129 `RED` lines, 101 plus 28. New in this attempt:
- unfenced: the late staging writable, staging write, staging close, staging abort, fallback write and fallback close;
- the cleanup policy:
  - always rethrow, red at the `quota` leg;
  - never rethrow, red at the `integrity` leg, through the retry;
  - drop `failure = error`, red at the `integrity` leg;
- the cleanup's deadline and fence entries, now retargeted to the `integrity` leg.

One existing anchor is retargeted and still `RED`: `restore the pre-successor direct fallback final write`, which now searches the `#fencedStep` form.

**Green:**
- `node scripts/check-stem-store-v1.mjs`: PASS.
- `node scripts/check-stem-store-v1.mjs --self-test --budgets`: PASS.
- `stem-store-core-v1.mjs --budgets`: 4 parallel runs, all PASS.
- `stem-pump-v1.mjs`: PASS.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: ok.

**Docs.** `docs/STEM_STORE_V1.md`:
- lists the writable steps and that they are fenced;
- states the late-writable abort;
- states the cleanup policy;
- has the "stalled or failed lookup" wording.

**Follow-ups.**
- Attempt 1's first follow-up is fixed by this attempt. Its fallback-writable half was overstated: in real browsers a Streams `abort()` waits for a pending `close()`, so only the fake had that gap.
- `read()`'s lookup with no deadline: successor #2405, spec drafted.

**Verdict: FAIL** (Sol, one BLOCKER, which is a regression from base, and three MINORs). The attempt-1 BLOCKER is fixed, and every step that changes the store is fenced.
- BLOCKER: a resolver overrun throws `stem.ingest.integrity` inside the inner try. The now-fenced staging `abort()` stalls, and its stall is swallowed. `#ensureIngested` then retries, and the retry's `#promote` waits on `settled(hold.fences)`, which still holds the wedged abort. An open with no signal never settles; base and attempt 1 opened in about 58 ms. The reviewer's probe is `abort-retry.mjs`. Required:
  - keep the abort fenced, but rethrow its failure or stall when the error is `stem.ingest.integrity`;
  - add a leg `staging abort (integrity)` and a ledger entry;
  - audit for any other swallowed stall that precedes a retry.
- MINOR: the late-writable abort is untested. Mutating it to `this.#fence(hold, opening)` keeps the suite green.
- MINOR: the successor is #2405. Add its spec and replace the placeholder.
- MINOR, optional: on the last attempt, a stalled cleanup replaces `stem.ingest.integrity` even though no retry follows.

### Attempt 3 (Terra, 2026-11-08)
Folded on the same branch, over `448688677`.

**Rule.** An integrity failure is retried once in the same hold, and the retry's `#promote` waits on every fence of that hold. So when a retry would follow, a staging abort or cleanup that failed or stalled fails the attempt with its own error, and that stops the retry. Otherwise the attempt's own outcome stands. `#ingestAttempt(stem, resolver, options, hold, lastAttempt)` decides this with one predicate, `retried(error) = !lastAttempt && error?.code === "stem.ingest.integrity"`. `#ensureIngested` passes `attempt === 1` as `lastAttempt`.
- Abort (BLOCKER): the staging abort stays fenced and keeps its no-signal wait. Its failure is recorded, and it is thrown after `reader.cancel` and the `leaveStaging` handling, when `retried(error)` holds. Otherwise the original error is thrown, as before.
- Cleanup: `if (retried(failure)) throw cleanupError`. A last attempt keeps `stem.ingest.integrity` (MINOR 4, done because it costs only the parameter). Every other outcome is unchanged from attempt 2.

**Audit: swallowed stalls that precede a retry.** A retry follows only a `stem.ingest.integrity` failure of the attempt. Each source of one:
- The overrun, inside the inner try: its abort was swallowed. Fixed above.
- The streamed-hash mismatch and the pre-promote re-hash mismatch: the staging writable has already closed, so only the cleanup follows. The cleanup is covered by the rule.
- The fallback final's verification failure: its removal's wait throws on a stall (attempt 1).
- No other fenced step precedes a retry:
  - the fallback copy's catch fences run only on write, read or abort errors, never on integrity;
  - the late-writable fence runs only when the writable's open failed, and that failure is never `stem.ingest.integrity`;
  - the quota path never retries.
- `#ensureIngested`'s recheck demotion (`#removeDemoted`, best-effort) is not followed by a retry. It is followed by an ingest whose promote waits for that removal, and that wait ends at the open's abort. This is #2381's accepted model (`aHoldNeverWritesAFinalItIsStillRemoving`), and it is unchanged here.

**Tests** (`aStemLockStepNeverOutlastsItsDeadline`, now 19 legs).
- New `staging abort (integrity)`: the first delivery overruns the declaration, and the abort is wedged. The open must settle `storage.write_stalled` within the deadline, and the stem lock must stay held until the abort settles.
- New `staging cleanup (last integrity)`: every delivery is corrupt, and the fourth removal of the staged file, the last attempt's cleanup, is wedged. The open must keep `stem.ingest.integrity`, and the lock must stay held until the removal settles.
- `staging writable open` now also asserts that the writable that opened late was aborted by the time the lock was released (MINOR 2). The fake's `abortBlocker` records the abort.
- Test value:
  - the abort leg is red if a failed or stalled abort is swallowed before an integrity retry, which then waits on it forever;
  - the last-integrity leg is red if a last attempt's cleanup stall replaces its integrity error;
  - the late-abort assertion is red if a late staging writable is fenced but never aborted.

**Red.**
- On the attempt-2 store `448688677`, each leg run alone:
  - `staging abort (integrity)`: `a stem-lock hold waited on a wedged staging abort (integrity) past its deadline` (actual `'timed-out'`);
  - `staging cleanup (last integrity)`: `a wedged staging cleanup (last integrity) failed with the wrong error` (actual `storage.remove_stalled`);
  - `staging writable open`: green on attempt 2, which already aborted the late writable. Its new assertion is held by a ledger entry.
- On base `d060df5cd`, each leg run alone:
  - `staging abort (integrity)`: `... failed with the wrong error` (actual `undefined`). Base retried and opened. Failing typed is the intended change: the retry would otherwise run while the abort is in flight.
  - `staging cleanup (last integrity)`: `... waited on a wedged staging cleanup (last integrity) past its deadline`;
  - `staging writable open`: `a timed-out staging writable open released the stem lock before it settled`.
- The reviewer's probe `abort-retry.mjs`:
  - attempt 2: `still pending after 1500 ms (resolves=2, abort wedged=true)`;
  - attempt 3, without and with a signal: `failed storage.write_stalled`, in 57 ms and 54 ms.
- `close-repro.mjs`, both modes, on attempt 3: the lock is held and the second open waits.

**Mutation ledger.** The #2397 block has 31 entries, each `RED` at its target. New:
- "fail the last attempt on its stalled staging cleanup", with `!lastAttempt` dropped: red at `... (last integrity) failed with the wrong error`;
- "swallow a stalled staging abort before an integrity retry": red at `... waited on a wedged staging abort (integrity) past its deadline`;
- "fence a late staging writable without aborting it", that is `this.#fence(hold, opening)`: red at `a staging writable that opened late was never aborted`.

The cleanup-policy entries are retargeted to the `retried(failure)` line.

The row now runs after `lyingDeclarationIsRefused`. Its `staging abort (integrity)` leg needs the over-length check. Run before that test, the leg went red first under the older entry "fill staging to the stream end before refusing an over-length delivery", which is red outside its target. That entry's own gate must stay the first to see it. Within the row, the `last integrity` leg runs after the `quota` and `success` legs, so "let a stalled staging cleanup replace every attempt outcome" is still red at the `quota` leg.

**Green:**
- `node scripts/check-stem-store-v1.mjs`: PASS.
- `node scripts/check-stem-store-v1.mjs --self-test --budgets`: PASS, 132 `RED` lines (101 plus 31).
- `stem-store-core-v1.mjs --budgets`: 4 parallel runs, all PASS.
- `stem-pump-v1.mjs`: PASS.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: ok.

**Successor.** `.github/ISSUE_SPECS/2405-web-stem-store-a-lease-read-s-lookup-of-its-final-has-no-deadline.md` is added, and the placeholder above now names #2405.

**Docs.** `docs/STEM_STORE_V1.md` now states the retry rule: a staging abort or cleanup fails a retried attempt, and a last attempt keeps `stem.ingest.integrity`.

**Verdict: FAIL** (Sol, one MAJOR). No hang path is left, and the BLOCKER is fixed.
- MAJOR: a staging abort that rejects outright now stops a retry that opened on base, and the error the caller gets is an unclassified `DOMException`. The reviewer's probe is `abort-hard.mjs`. A rejected abort has already settled, so nothing is in flight, and the stated reason does not apply. Narrowing the catch to stalls only also keeps the suite green, so that half of the rule is untested.
- The coordinator chose Sol's structural fix for attempt 4:
  - one wait, before the retry, for the hold's fences, with a deadline;
  - remove the per-step `retried()` / `lastAttempt` plumbing;
  - one attempt constant and one integrity predicate;
  - keep the two legs and add `staging abort rejected (integrity)`;
  - three ledger entries for the wait;
  - a comment at the leg whose order matters, and a docs rewrap.

### Attempt 4 (Terra, 2026-11-08)
Folded on the same branch, over `363b63a56`.

**One rule.** A retry runs only once every step the hold has fenced has settled. The hazard is a step that is still in flight: it could land on the staged file or the final that the retry writes, and the retry's promote would wait on it with no deadline. A step that rejected has already settled, so it does not stop the retry.

**Fix** (`opfs-store.js`).
- `#ensureIngested` loops `INGEST_ATTEMPTS` (2) times. Before every attempt after the first, it awaits `withDeadline(settled(hold.fences), readDeadlineMs, "storage.write_stalled", …, options.signal)`. On expiry the open fails typed and no retry runs.
- The loop's test is the shared predicate `isIntegrityFailure(error)`. Nothing else needs it now.
- Removed: `lastAttempt`, `retried()`, `abortFailure`, `failure`.
- The staging abort and the staging cleanup are both best-effort again (`.catch(() => {})`), and both stay fenced. Their failure never replaces the attempt's own outcome, which is attempt 2's "keep the original error" policy:
  - on success, a stalled cleanup is swallowed;
  - a last attempt keeps `stem.ingest.integrity`;
  - a rejected abort lets the retry run, as on base (`abort-hard.mjs`).
- The fallback final's verification-failure removal followed the same per-step rule: since attempt 1 it threw on a stall so that no retry ran. It is now best-effort through `#removeDemoted`, like the quota path's removal, and the retry waits for it. The ledger proved the throwing wait redundant: the attempt-1 entry "retry a fallback whose removal of its final stalled" went red outside its target under this rule, because the retry's wait now catches it. That entry is deleted.
  - Change from base: a hard (non-stall) removal error here used to fail the open; now the retry runs, by the same rule as a rejected abort.
- **Deliberate, bounded improvement.** A #2381 recheck demotion whose removal is still in flight at retry time now fails the open typed within the deadline. Before, it waited until the open's abort. The recheck path's first attempt is unchanged: its promote still waits for the demotion, until the open's abort, as #2381 accepted.

**Tests** (`aStemLockStepNeverOutlastsItsDeadline`, now 20 legs).
- `staging abort (integrity)`: as before. The open now fails `storage.write_stalled` from the retry's wait, and the lock is held until the abort settles.
- New `staging abort rejected (integrity)`: the first delivery overruns, and the abort rejects at once. The retry must open, as on base.
- `staging cleanup (integrity)`: the stalled cleanup is swallowed and the retry's wait expires, so the open fails `storage.write_stalled`. It was `storage.remove_stalled`.
- `staging cleanup (last integrity)`: kept, `stem.ingest.integrity`.
- `integrity removal`: the open now fails `storage.write_stalled` from the retry's wait.
- Finding 4: a one-line comment marks `staging cleanup (integrity)` as the first cleanup leg, which the ledger's staging-cleanup entries name as their target. The `last integrity` leg's position no longer matters, because the entry that needed it is deleted.
- Test value of the new leg: red if a step that has already failed, and so is no longer in flight, stops an integrity retry; base retried it.

**Red.**
- On the attempt-3 store `363b63a56`, each leg run alone:
  - `staging abort rejected (integrity)`: `a wedged staging abort rejected (integrity) failed the open` (actual `InvalidStateError: abort refused`);
  - `staging cleanup (integrity)` and `integrity removal`: wrong error (actual `storage.remove_stalled`).
- On base `d060df5cd`:
  - `staging abort rejected (integrity)` is green, by design: it holds base's behaviour.
  - `staging abort (integrity)` is red: wrong error, because base retried while the abort was in flight.
  - `staging cleanup (integrity)` and `integrity removal` are red: they waited past the deadline.
- Probes on attempt 4:
  - `abort-retry.mjs`: `failed storage.write_stalled`, in 74 ms, and 77 ms with a signal;
  - `abort-hard.mjs`: `opened (resolves=2)`, in 41 ms with and without a signal;
  - `close-repro.mjs`, both modes: the lock is held, and the second open waits.

**Mutation ledger.** The #2397 block has 30 entries, each `RED` at its target.
- Deleted, with the plumbing:
  - "let a stalled staging cleanup replace every attempt outcome";
  - "swallow a stalled staging cleanup after an integrity failure";
  - "forget the attempt's failure before its staging cleanup";
  - "fail the last attempt on its stalled staging cleanup";
  - "swallow a stalled staging abort before an integrity retry";
  - "retry a fallback whose removal of its final stalled".
- New:
  - "retry an ingest without waiting for its fenced steps";
  - "wait for a retry's fenced steps with no deadline";
  - "retry an ingest after its wait for fenced steps expired";
  - the first three are red at `… waited on a wedged staging abort (integrity) past its deadline`;
  - "let a staging cleanup failure replace the attempt's outcome", red at `staging cleanup (integrity)`;
  - "let a staging abort failure replace the attempt's error", red at `staging abort`.
- Retargeted:
  - the integrity removal's two entries now search the `#removeDemoted` call;
  - its outside-the-hold entry, and the cleanup's unfenced entry, are now red at `failed with the wrong error`, because with no fence the retry runs and fails integrity again;
  - `fallbackFinalArm` anchors on the new comment.

**Green:**
- `node scripts/check-stem-store-v1.mjs`: PASS.
- `node scripts/check-stem-store-v1.mjs --self-test --budgets`: PASS, 131 `RED` lines (101 plus 30).
- `stem-store-core-v1.mjs --budgets`: 4 parallel runs, all PASS.
- `stem-pump-v1.mjs`: PASS.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: ok.

**Docs.** `docs/STEM_STORE_V1.md` states the one rule:
- the staging abort and cleanup are best-effort;
- an integrity retry waits, with the deadline, for every fenced step;
- a rejected step does not stop the retry;
- the recheck demotion is now bounded at retry time.

The ingest's two removals of a final are now described as best-effort. The paragraph is rewrapped (finding 5).

**Verdict: PASS** (Sol). Two minors are folded on the same branch, and nothing else changed.
1. Nothing tested the signal on the retry's wait: the suite stayed green with `options.signal` dropped from it.
2. Nothing tested that a hard error from the fallback-final removal now lets the retry run: the suite stayed green with non-`StemStoreError` errors rethrown.

**Fold** (tests and ledger only; `opfs-store.js` is unchanged).
- Two new legs of `aStemLockStepNeverOutlastsItsDeadline`, which now has 22:
  - `staging abort (integrity, aborted)`: the open is aborted while its staging abort is wedged. Once that abort's best-effort wait expires, the retry's wait must refuse at once with `AbortError`. Without the signal it would wait out its deadline and fail `storage.write_stalled`. The lock stays held until the wedged abort settles.
  - `integrity removal rejected`: only the first fallback copy is corrupt, and the removal of that final rejects at once. It has settled, so the retry runs and the open succeeds.
- The leg loop also takes an expected error `name`, which the aborted leg uses.
- Two new ledger entries, 32 in the #2397 block:
  - "give a retry's wait for fenced steps no signal", red at `a wedged staging abort (integrity, aborted) failed with the wrong error`;
  - "fail an open on a rejected fallback-final removal", which rethrows non-`StemStoreError` errors, red at `a wedged integrity removal rejected failed the open`.
- Test value:
  - the aborted leg is red if the retry's wait ignores the open's abort;
  - the rejected leg is red if a removal that has already failed stops the retry.
  - Both legs hold behaviour attempt 4 already had, so they are green on attempt 4, and their ledger entries are the red evidence.

**Merged `origin/main`** (`3375b619b`). There is no overlap with this branch's files.

**Green** after the merge:
- `node scripts/check-stem-store-v1.mjs`: PASS.
- `node scripts/check-stem-store-v1.mjs --self-test --budgets`: PASS, 133 `RED` lines (101 plus 32).
- `stem-store-core-v1.mjs`: PASS.
- `stem-pump-v1.mjs`: PASS.
