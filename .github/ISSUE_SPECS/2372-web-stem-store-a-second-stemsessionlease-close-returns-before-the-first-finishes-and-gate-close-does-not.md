# Web stem store: a second `StemSessionLease.close()` returns before the first finishes, and `gate.close()` does not wait for an aborted open's lease

Successor of #2266 (WEB-4), filed when the implicit-state-machine tracker #2258 closed. #2266 fixed the state half of `close()` and left this as a non-goal.

## Problem
In `hosts/host-web/web/stem-store/opfs-store.js`:
- A second `StemSessionLease.close()` (around line 1282) returns at once while the first close is still in flight. A caller that awaits the second close can therefore proceed while the pin is still held.
- `gate.close()` does not wait for an aborted open's own lease close. It can resolve before that pin is released.

Nothing deadlocks on base. The defect is that a resolved close promises a released pin it has not yet released.

## Contract
- Every `close()` on one lease resolves only after the pin is released, and every call shares the first call's promise.
- `gate.close()` resolves only after every lease it opened, including an aborted open's lease, has released its pin.

## Non-goals
- No change to the `.d.ts` signatures, the `state` union, the render path or Rust.

## Gates
- Red on base: (1) a second close that resolves while the first close's release is held open by a test hook; (2) a `gate.close()` that resolves while an aborted open's lease is still pinned.
- The stem-store JS suite is green on both browsers.

## Evidence / attempt record
### Attempt 1 (Terra, 2026-11-08)
Base `9c391cb46`; branch `codex/issue-2372-close-waits-for-pin-release`.

**Claims re-verified on base.** Both hold.
- `opfs-store.js` 1282-1290: `close()` returns at once when `#closed` is set, so a second call resolves while the first call's `releaseSessionPin` (an index rewrite under the index lock) is still pending.
- `session-gate.js` 127-136 (`close()`): it aborts `#opening` and awaits only the current `#lease`. An aborted open closes its own lease in its catch path (and a displaced or replaced predecessor in its body), and nothing in `close()` waits for that. A superseded open's lease is never waited for either.

**Fix.**
- `StemSessionLease.close()` returns one stored promise, `#closing ??= this.#release()`. Every call shares the first call's release, and `read()` refuses once `#closing` is set, which replaces `#closed`.
- `StemSessionGate` keeps `#settling`, the settle promises of its opens in flight. `open()` is now a sync wrapper over the old body (`#open`). The wrapper still calls `resume()` before its first await. `close()` awaits `Promise.allSettled([currentLeaseClose, ...#settling])` and then the current lease close, so a lease close failure still rejects `close()` as before. Every open in flight is aborted by the time `close()` awaits: the newest one by `close()` itself, the others by the open that superseded them. An open settles only after it has closed every lease it held (granted, displaced or replaced), so `close()` resolves only once all their pins are released. An open started after `close()` is not in the snapshot.
- No change to the `.d.ts`, the `state` union, the render path or Rust. `docs/STEM_STORE_V1.md` states the contract.

**Red on base** (`hosts/host-web/tests/stem-pump-v1.mjs`). Each test holds the index lock open with `holdTheIndex`, which is the test hook, so the pin release stalls:
- `everyLeaseCloseWaitsForThePinRelease`: `AssertionError: a second close() resolved while the first still held the pin` (actual `['session:matrix:same']`, expected `[]`).
- `gateCloseWaitsForEveryAbortedOpensLease`, run with the first test removed: `AssertionError: gate.close() resolved while an aborted open's lease still held its pin (aborted by close())` (actual `['session:matrix:same']`).

**Each half is held by its own test.**
- Fixed gate with the base store: only `everyLeaseCloseWaitsForThePinRelease` is red. The gate test passes because the aborted open awaits the first close.
- Base gate with the fixed store: `gateCloseWaitsForEveryAbortedOpensLease` is red.

**Mutations of the fix.** Each was done by hand, and the first three are also new entries in the nightly mutation ledger:
- A second lease close returns `Promise.resolve()`: red, `a second close() resolved while the first still held the pin`.
- `close()` waits only for the current lease (`Promise.allSettled([closing])`): red, `... still held its pin (aborted by close())`.
- `close()` waits only for the newest open (`#settling.clear()` before each add): red, `... still held its pin (superseded, then close())`. The superseded open's lease release is queued behind the held index. The newest open, queued on the index through `verifyIndexed`, which takes its signal, rejects at once. Only the second leg can see this mutant.
- Drop the `#settling.delete` on settle: green. Equivalent for behaviour: it only retains settled promises, so it is a memory leak with no observable effect.
- Drop the trailing `await closing`: green. With it gone, `close()` swallows a failure of the current lease's close. Base propagated that failure and no test covered it on base either; the line keeps base's behaviour.
- The existing ledger entry "let close() write idle after its await" searched the old `close()` text. It is retargeted to the new last line and is still `RED` at `close() reported idle while a later open was loading`.

**Green** at this commit:
- `node scripts/check-stem-store-v1.mjs`: `stem-store-v1 gate: PASS`.
- `node scripts/check-stem-store-v1.mjs --self-test --budgets`: PASS. Every ledger entry is `RED` at its target, including the three new #2372 entries.
- `stem-pump-v1.mjs`: 5 serial runs and 8 parallel runs, all PASS.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- No Rust and no render path changed, so cargo test/clippy/fmt/doc, the realtime policy tool and a no-bit-moved comparison do not apply. The state is single-threaded JavaScript, so there is no loom model.
- Browsers: the stem-store suite runs in Node against the fakes, which is how the qualification `stem-store` step runs it, and the repo has no browser runner for it. A scratch Playwright harness (not committed) ran both scenarios in Chromium and WebKit, against the `FakeLockManager` and against `navigator.locks`. Neither browser launches on this host: the system libraries are missing (`libatk-1.0.so.0`, `libgbm.so.1` and others), and the cached Chromium build does not match Playwright 1.62.1. **The two-browser gate is not met by this attempt.** It needs a host with Playwright's browser dependencies installed.

**Test value.**
- `everyLeaseCloseWaitsForThePinRelease`: red if a repeated `close()` on a lease resolves before the first call has released the pin. No existing test closes a lease twice while its release is in flight.
- `gateCloseWaitsForEveryAbortedOpensLease`: red if `gate.close()` resolves before an aborted open has closed its granted lease. Its second leg is also red if `close()` waits only for the newest open and not for an earlier superseded one. The #2266 matrix awaits `env.closing` but checks pins only after the aborted open itself has settled, so it cannot see this order.

**Follow-up (not filed).** An aborted `openSession` that joined another caller's in-flight ingest of the same stem waits for that ingest without its own signal (`opfs-store.js` `#ensureIngested`, `if (existing !== undefined) return existing`). With this fix, `gate.close()` waits for that too. That case needs a second caller on the same `OpfsStemStore` instance. With one gate per store, every ingest the gate's opens joined belongs to an open that is itself aborted, so the wait stays bounded by the abort.

**Verdict: FAIL** (Sol, one MAJOR). `gate.close()` now waits for every in-flight open, but `openSession` has two waits that ignore the open's signal, and neither of them holds a lease. A wedged one makes `close()` hang, where base resolved:
- (a) `await this.open()` (`opfs-store.js` 153): the store open, and `getDirectory`/`getDirectoryHandle` have no deadline;
- (b) the joined-ingest `return existing` (`opfs-store.js` 306).

The class comment at `session-gate.js` 14-15 overclaimed. MINORs:
- The sync `open()` wrapper's `opened.catch(() => {})` handled the caller's own promise, so a dropped failing `gate.open()` no longer raised `unhandledrejection`.
- Dropping the trailing `await closing` left the suite green, so it was not an equivalent mutant.
- `docs/STEM_STORE_V1.md` did not say that `close()` waits for aborted opens, nor why that wait is bounded.

The two-browser gate is not applicable: the stem-store suite runs only in Node against fakes, as the qualification `stem-store` step runs it.

### Attempt 2 (Terra, 2026-11-08)
**Fix.**
- `openSession` waits for the store open with `abortable(this.open(), options.signal)`. The memoized open keeps running for other callers, and `abortable` attaches to it, so its rejection is never unhandled.
- A joiner returns `abortable(existing, options.signal)`.
- The audit for other signal-less, deadline-free waits found one more of the same kind: `#preflight`'s `storage.estimate()`. It now takes the open's signal the same way. An abort there falls into the existing catch, and the ingest's own signal check refuses the open next.
- What an aborted open still waits through is the store's index work: index reads, pin writes, eviction and lease closes. That is the work base `close()` already awaited for the current lease, and its index reads and writes are deadlined. The class comment and `docs/STEM_STORE_V1.md` now say exactly this, and why the wait in `close()` is bounded.
- `open()` is `async` again and tracks its settlement on a private deferred, which its outermost `finally` resolves. The caller's promise is left alone.
- `docs/STEM_STORE_V1.md` states the memoized rejection: a failed release rejects that `close()` and every later one, and `gate.close()` rejects with it.

**Red on the attempt-1 code** (`e4b1f3b4b` store and gate, each test or leg run alone):
- `gateCloseNeverWaitsOnWorkThatHoldsNoLease`, three legs:
  - `gate.close() waited on an aborted open's wedged getDirectory`
  - `... wedged estimate`
  - `... joined ingest`. That leg first checks that the gate's open is not queued on the ingest's stem lock, so it really joined.
- `aDroppedFailedOpenStaysUnhandled`: `timed out waiting for the dropped open's unhandled rejection`.
- `aFailedPinReleaseRejectsEveryClose` is green on attempt 1, because attempt 1 already had both behaviours. It is the guard for the trailing `await closing` and for the memoized rejection; see the mutations.

**Mutations of the fix** (by hand, and four of them as new ledger entries):
- `await this.open()` without `abortable`: red, `wedged getDirectory`.
- The estimate without `abortable`: red, `wedged estimate`.
- A plain `return existing`: red, `joined ingest`.
- Drop the trailing `await closing`: red, `gate.close() hid a failed pin release`.
- Track the settlement on the caller's promise (attempt 1's wrapper): red, `timed out waiting for the dropped open's unhandled rejection`. This one is by hand only. No local text edit of the `async open()` form reproduces it without also dropping the tracking, which would turn an earlier test red first.
- Equivalent, by reason: dropping `#settling.delete` only retains settled promises, so it is a memory leak with no observable effect. Dropping `abortable`'s already-aborted branch only delays the refusal to the next signal check, since every later wait in `openSession` checks the same signal.
- The existing ledger entry "content-key -> per-session store directory" anchored on `await this.open()`, so it now anchors on the new line, and it is still `RED` at its target.

**Test value.**
- `gateCloseNeverWaitsOnWorkThatHoldsNoLease`: red if an aborted session open keeps waiting on work that holds no lease and has no deadline: the store open, the quota estimate, or another caller's ingest it joined. That makes `gate.close()` hang. No other test wedges any of the three.
- `aFailedPinReleaseRejectsEveryClose`: red if `gate.close()` swallows a failed release of the current lease, or if a repeat `lease.close()` resolves after a failed release. No other test fails a release.
- `aDroppedFailedOpenStaysUnhandled`: red if the gate's settlement tracking marks the caller's promise handled, which hides a dropped open's failure. No other test watches `unhandledRejection`.

**On base `9c391cb46`.** `aFailedPinReleaseRejectsEveryClose` is red: `Missing expected rejection: a repeat close() reported a release that failed`. A repeat close resolves at once after a failed release.

**Green** at the attempt-2 commit:
- `node scripts/check-stem-store-v1.mjs`: PASS.
- `node scripts/check-stem-store-v1.mjs --self-test --budgets`: PASS. Every ledger entry is `RED` at its target, including the seven #2372 entries.
- `stem-pump-v1.mjs`: 5 serial runs and 8 parallel runs, all PASS.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- No Rust and no render path changed. The two-browser gate is not applicable, per the attempt-1 verdict.

**Verdict: FAIL** (Sol, one MAJOR, the same class as attempt 1's). `gate.close()` still hung, where base resolved, on two waits of an aborted open that hold no lease. Probe P1: the open's own ingest-cleanup `removeEntry` of staging or final has no deadline. Probe P2: under quota pressure, `#evictUnpinned` waits on a stem lock another holder keeps, through `#withStemLock(identity, undefined, ...)`, which has no signal. `#quotaError`'s `estimate?.()` is unsignalled too. The reviewer asked which model is right before patching wait by wait: should `close()` wait only for leases that are granted or still grantable? MINORs:
- `session-gate.js` keeps its own `abortable`.
- `#preflight`'s catch-all swallows an abort.

### Attempt 3 (Terra, 2026-11-08)
**Model: `close()` waits for pins, not for opens.** A session open can take the session pin only after it passes `openSession`'s last signal check (`throwIfAborted(options.signal)`, right before the pin lock request, with no await between them). An open aborted before that check never takes a pin. So `close()` waits for exactly what holds a pin or can still take one:
- the current lease's close;
- a same-session predecessor an open took out of the gate and is closing (`replaced`, closed before that open's pin phase);
- every open that reached the pin phase.

The store reports the pin phase through an internal option, `onPinning`. It is called once, synchronously, right after that signal check. It is documented in `openSession`'s JSDoc as StemSessionGate's internal hook, outside the typed contract, so the `.d.ts` is unchanged. The gate keeps one `#releasing` set:
- `#track(promise)` adds a lease close, or an open's private settle deferred once that open reports the pin phase.
- `close()` waits for `Promise.allSettled([currentLeaseClose, ...#releasing])` and then rethrows the current lease's failure.

This removes the whole class, not one wait at a time. Whatever lease-free work an aborted open is still in, `close()` never waits on it: the store open, the estimate, a joined ingest, its own staging cleanup, or an eviction and its stem lock. Those waits behave as they do on base, and the aborted open rejects when the work ends.

**Why each wait `close()` still has is bounded.**
- An open in the pin phase:
  - pin lock acquisition: signalled; an abort while queued rejects it;
  - pin write: `#mutateIndex`, index work;
  - the `resume()` wait: `abortable`;
  - the displaced predecessor's close and its own lease's close: index work.
- A replaced predecessor's close: index work.
- The current lease's close: index work, which base `close()` already waited for.
- Index work means an unsignalled index-lock acquisition plus deadlined index reads and writes. Per the verdict this is pre-existing and bounded.

**Attempt 2's store changes are reverted.** `abortable` around the store open, the joined ingest and the estimate, and the `#preflight` signal are all gone. The model makes them unnecessary for `close()`, and each would be an unguarded line, because no test can tell it apart under the model. Both MINORs go away with them. Only `session-gate.js` uses an `abortable`, so there is no second copy to share, and `#preflight` again takes no signal, so its catch cannot swallow an abort.

**Tests** (`stem-pump-v1.mjs`)
- `gateCloseNeverWaitsOnWorkThatHoldsNoLease` now has five legs:
  - wedged `getDirectory`;
  - wedged estimate;
  - joined ingest;
  - P1: the open's own staging-cleanup `removeEntry` wedged, the second staging removal;
  - P2: an eviction under quota pressure queued on a held stem lock.

  Each leg asserts that `close()` resolves within 500 ms. It then releases the wedge and asserts that the aborted open rejects and that the index holds no pin for it.
- New `gateCloseWaitsForAnOpenInItsPinWrite`: the open holds the session pin lock, and its pin write is queued behind a held index. `close()` must not resolve while that lock is held.
- New `gateCloseWaitsForAReplacedPredecessor`: a same-session open is closing the lease it took out of the gate. `close()` must not resolve while that lease's pin is in the index.

**Red.**
- On attempt 2 (`acf3287dd`), each leg run alone:
  - P1: `gate.close() waited on an aborted open's wedged staging cleanup`.
  - P2: `gate.close() waited on an aborted open's eviction behind a held stem lock`.
  - The other three legs pass on attempt 2, which fixed them wait by wait. Attempt 1 fails them, as recorded above.
- The two new tests pass on attempts 1 and 2, whose settle model waited for everything. They guard the pin-phase boundary of the new model and are red under its mutations, below.

**Mutation ledger** (`--self-test --budgets`; all entries `RED` at their targets):
- `let gate.close() wait only for the current lease`: `... still held its pin (aborted by close())`.
- `never report a session open's pin phase`: same target.
- `report a session open's pin phase only once the pin is written`: `gate.close() resolved while an open in its pin write held the pin lock`.
- `close a replaced predecessor without gate.close() waiting for it`: `gate.close() resolved while a replaced predecessor still held its pin`.
- `let gate.close() wait for every open from its start`, the attempt-2 model: `gate.close() waited on an aborted open's wedged getDirectory`.
- `let gate.close() hide a failed release of the current lease` and `let a second lease close() resolve ...` are kept and retargeted to the new code.
- Two existing anchors that matched the old text were updated: `content-key -> per-session store directory` and `stop forwarding the caller's signal ...`. Both are still `RED` at their targets.
- The five no-lease legs share one ledger entry, not one each. Attempt 2's per-leg entries mutated per-wait store fixes that this model removes; here one mechanism holds all five legs. Each leg's red on the old code is recorded above, by hand.
- Equivalent: dropping the `#releasing.delete` only keeps settled promises around, a leak with no visible effect.

**Green.**
- `node scripts/check-stem-store-v1.mjs`: PASS.
- `--self-test --budgets`: PASS.
- `stem-pump-v1.mjs`: 5 serial runs and 8 parallel runs, all PASS.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: ok.
- No Rust and no render path changed. The two-browser gate is not applicable.

**Docs.** `docs/STEM_STORE_V1.md` and the `session-gate.js` class comment state the model, list what `close()` waits for, and say why it is bounded.

**Follow-ups (not filed).**
- The undeadlined `removeEntry` inside `#evictUnpinned`'s `#mutateIndex` holds the index lock, so a wedge there blocks every lease close, base's included.
- `#readIndex`'s `getFileHandle(INDEX_FILE)` has no deadline.
- An aborted open's eviction still runs once its stem lock frees, because `#evictUnpinned` takes no signal. That evicts an unpinned stem for an open that no longer needs the space; it holds no pin.
