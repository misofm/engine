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
