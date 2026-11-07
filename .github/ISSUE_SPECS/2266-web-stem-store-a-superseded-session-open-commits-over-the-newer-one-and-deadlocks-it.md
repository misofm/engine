# Web stem store: a superseded session open commits over the newer one and deadlocks it

Child of the implicit-state-machine tracker #2258 (item WEB-4). Found by its read-only audit of 2026-11-06.

## Mission
A `StemSessionGate` open that a newer open (or `close()`, or the caller's signal) has superseded must never become the gate's session, and no aborted open may keep waiting on a session pin lock or on its `resume()`.

## Verified defect (base `de0878e23`)
`hosts/host-web/web/stem-store/session-gate.js` `open()`:
- Line 91 checks `this.#opening !== opening` once, then line 95 `await this.#lease?.close()` yields while closing the predecessor lease (a `releaseSessionPin` index rewrite under the index lock, `opfs-store.js` 293). Lines 96-97 then set `#lease` and `#state = "interactive"` unconditionally and the open resolves with its lease. A newer open started inside that window is overwritten, and so is a `close()`. With no current lease the window is still there: `await undefined` is one microtask, and a newer open started from a callback on the resume promise lands in it (attempt 2's offset sweep is red on base at offset 2).
- Lines 85-88 wait for the store open and the `resume()` promise together with `Promise.all`, which ignores the abort. A superseded open whose resume has not settled keeps its granted lease, and so its pin, until the resume settles; if it never settles, it keeps it forever and the caller's abort cannot reach it.
- With the same session ID the newer open deadlocks. The superseded open holds the store lease it was granted, and so the exclusive session pin lock `session:<tab>:<id>`. The newer open sees the gate's `#lease` still on the predecessor (a different ID), so it releases nothing, and its `openSession` queues on that pin in `#acquireSessionPinLock` (`opfs-store.js` 1167-1185). When the superseded open commits, nothing closes the lease except a later `close()` or a third open of the same ID. The newer open never settles, and the gate reports `interactive` for the stale open's stem list.
- `#acquireSessionPinLock` requests the lock `{ mode: "exclusive" }` with no `signal` (`opfs-store.js` 1179-1181). It is the only wait in `openSession` that ignores the open's signal (`#withLock` passes it), so an aborted open stays queued for as long as another holder of the same pin (a second gate or a direct `openSession` lease on the same store) stays open.

Reachability: the window is the predecessor close, an index lock round trip with an OPFS read and write, and it needs a predecessor of a different session ID. A different-ID overwrite (the newer open finishing inside that window) is not reachable with one `OpfsStemStore`: the newer open's index requests queue FIFO behind the predecessor close's request, so the stale open resumes first. The same-ID path needs only that the newer open *starts* inside the window, and it is reachable.

## Decision / model
- The gate's ownership token is the open's own `AbortController`. Supersession, `close()` and the caller's signal already abort it, so "this open still owns the gate" is exactly `!opening.signal.aborted`. After its last await, and before writing `#lease` or `#state`, an open calls `opening.signal.throwIfAborted()`; the predecessor close is followed by the same check, and there is no await between the check and the commit. A superseded open therefore rejects with the abort reason and its existing catch path closes the lease it was granted, which releases the pin the newer open waits on.
- The wait for `resume()` ends at the abort (`abortable(resumeOutcome, opening.signal)`), after the store open has granted its lease, so a superseded open never keeps its lease for a resume that does not settle. The `throwIfAborted()` right after it stays: an abort can land after the resume settles and before the open's continuation runs, and with no current lease that check is the only one before the commit. So "aborted, therefore rejects and releases its lease" holds at every await; only the two bounded lease closes are awaited to completion.
- The displaced predecessor is taken out of `#lease` before its close is awaited, as the same-ID branch already does, so the gate never holds a lease that is closing.
- A change from base: an open whose caller's signal aborts after its lease was granted now rejects (before, it committed). This follows from the token being the controller, and is what an abort means.
- `#acquireSessionPinLock(pin, signal)` passes the open's signal to the Web Lock: an aborted open gives up its place in the queue (Web Locks semantics); a granted lock is unaffected.
- The test `FakeLockManager` now honours `signal` the way the Web Locks API does (an aborted signal rejects without queueing; an abort while queued rejects and leaves the queue; an abort after the grant changes nothing). Before, it ignored it, so no store test could see a lock wait's abort.

## Non-goals
- Follow-up, to be filed by the coordinator: a second `StemSessionLease.close()` returns at once while the first is still in flight (`opfs-store.js` 1282), and `gate.close()` does not wait for an in-flight open's cleanup, so `gate.close()` can resolve before a pin is released. Base behaves the same and nothing deadlocks.
- No change to the render path, Rust code or the `.d.ts` (the `state` union and `open()` signature are unchanged).

## Gates
- `hosts/host-web/tests/stem-pump-v1.mjs` (run by `node scripts/check-stem-store-v1.mjs`, the qualification `stem-store` step):
  - `supersededOpenNeverCommits`: red on base (the superseded open resolves with its lease and the newer open times out), green after.
  - `supersededAsResumeSettlesNeverCommits` (attempt 2): a gate with no current lease, the first lease granted, and a newer open of the same session started at each microtask offset 0-8 after the first resume settles. Before the commit the first open must reject; after it, the newer open is a legitimate reopen. Every offset's newer open resolves with one pin, and the sweep must span the commit. Red on base and when the post-resume check is deleted.
  - `neverSettlingResumeReleasesItsLease` (attempt 2): the first resume never settles; a newer open of the same session supersedes it. Red on base and on attempt 1 (the first open never settles and the newer one deadlocks on its pin).
  - `pinLockWaitObeysAbort`: red on base (the aborted open times out), green after.
  - The tests' lock-queue and readiness waits are bounded (`until`, 1,000 ticks) and throw, so a regression goes red instead of hanging the gate or the ledger.
- Four entries in the nightly mutation ledger (`scripts/check-stem-store-v1.mjs --self-test`): removing the post-close check, removing the post-resume check, waiting for the resume without the abort, and dropping the pin lock's `signal`. Each turns its target assertion red.
- `node scripts/check-stem-store-v1.mjs` and `--self-test` pass; `bash scripts/test-web-audioworklet.sh` and `bash scripts/check-workspace-policy.sh` pass.

## Evidence
### Attempt 1 (Terra, 2026-11-07)
Base `de0878e23`; branch `codex/issue-2266-session-gate-supersession`.

**Red on base** (the two tests and the signal-honouring `FakeLockManager` run against the base `session-gate.js` and `opfs-store.js`):
- `supersededOpenNeverCommits`: `AssertionError: a superseded open committed its lease over the newer open` (actual `undefined`, expected `'AbortError'`). A diagnostic run of the same scenario printed `{"first":"lease same","second":"timed-out","state":"interactive"}`: the stale open resolved with its lease, the newer open of the same session never settled, and the gate reported interactive.
- `pinLockWaitObeysAbort` (run alone): `AssertionError: an aborted open kept waiting on a held session pin` (actual `'timed-out'`).

**Each half is held by its own test.** Fixed gate with the base store: only `pinLockWaitObeysAbort` is red. Base gate with the fixed store: `supersededOpenNeverCommits` is red, and `pinLockWaitObeysAbort` alone passes.

**Mutations of the fix** (by hand, and as two new entries in the nightly mutation ledger):
- Remove the `opening.signal.throwIfAborted()` after `await displaced.close()`: red, `a superseded open committed its lease over the newer open`.
- Drop `signal` from the pin lock request: red, `an aborted open kept waiting on a held session pin`.
- Remove the check before the predecessor close (the base's own line-91 check, kept): green. **This note was wrong; see the attempt-1 verdict.** With no current lease it is the only check before the commit, and deleting it reproduces the headline defect.
- Keep `displaced` in `#lease` while it closes: green (equivalent: `#lease` is private and a closing lease's `close()` is a no-op). Kept for the representation, as the same-ID branch already does.
- A check after the same-ID `replaced.close()` was tried and dropped: equivalent, because `openSession` refuses an aborted signal before any work.

**Green** at the attempt-1 commit:
- `node scripts/check-stem-store-v1.mjs`: `stem-store-v1 gate: PASS`.
- `node scripts/check-stem-store-v1.mjs --self-test --budgets` (the nightly leg): PASS, both new ledger entries `RED` at their target assertions, every existing entry still `RED` at its target with the signal-honouring fake.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- No Rust and no render path changed, so cargo test/clippy/fmt, the realtime policy tool and a no-bit-moved comparison do not apply. The state is single-threaded JavaScript, so there is no loom model.

**Test value.**
- `supersededOpenNeverCommits`: red if an open superseded during its predecessor's close still commits its lease, which leaves a newer open of the same session queued forever on the pin; no existing test supersedes a gate open at all (the same-session test reopens only after the first open settled).
- `pinLockWaitObeysAbort`: red if the session pin lock wait ignores the open's signal; no existing test aborts an open while it waits on a lock, and the fake ignored `signal` until now.

**Verdict: FAIL** (Sol, one MAJOR). The pre-close `opening.signal.throwIfAborted()` (attempt 1 `session-gate.js` 98, the old line-91 check) was untested, and the note above understated it. With no current lease it is the only check before the commit; the reviewer deleted it and ran a first open of "same" whose `resume()` was still pending after its lease was granted, superseded by a second open of "same", then resolved the first resume: `{"ra":"lease","rb":"timed-out","state":"interactive"}`, the headline defect, with every suite green. MINORs: the `Promise.all` wait for `resume()` ignored the abort, so a superseded open whose resume never settles kept its lease and the doc sentence was false; `untilPending` polled with no bound. The reviewer accepted the different-session-unreachable claim and the behaviour change (a caller abort after the lease is granted now rejects).

### Attempt 2 (Terra, 2026-11-07)
**Fix.** The wait for the resume ends at the abort (`abortable`), after the store lease is granted. The post-resume `throwIfAborted()` stays, as the one check that covers an abort landing after the resume settles. `docs/STEM_STORE_V1.md` now says what holds. `until` bounds every wait in the tests.

**Red.**
- Base gate: `supersededAsResumeSettlesNeverCommits` red at offset 2 (`an open superseded as its resume settled committed its lease (offset 2)`); `neverSettlingResumeReleasesItsLease` alone red (`a superseded open waiting on its resume kept its lease`).
- Attempt-1 gate: `neverSettlingResumeReleasesItsLease` red with the same message; the sweep passes on attempt 1, whose post-resume check covered the window.

**Mutations, each held by exactly one test** (by hand, and as ledger entries):
- Delete the post-resume `throwIfAborted()`: the sweep is red at offset 1, and the suite without the sweep is green.
- Wait for the resume without `abortable`: `neverSettlingResumeReleasesItsLease` is red, and the suite without it is green, because the post-resume check still covers every settled offset.
- Delete both: the sweep is red at offset 0.
- Delete the post-close check: `supersededOpenNeverCommits` is red, as in attempt 1.

**Green.**
- `node scripts/check-stem-store-v1.mjs`: PASS; `stem-pump-v1.mjs` passed six consecutive runs.
- `node scripts/check-stem-store-v1.mjs --self-test --budgets`: PASS, all four #2266 entries `RED` at their targets.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.

**Test value.**
- `supersededAsResumeSettlesNeverCommits`: red if an open superseded between its resume settling and its commit still commits. With no current lease nothing else stops it, and no other test supersedes an open after its resume settles.
- `neverSettlingResumeReleasesItsLease`: red if the resume wait ignores the abort, which keeps a superseded open's lease and pin for as long as its resume is pending. No other test has a resume that does not settle.
