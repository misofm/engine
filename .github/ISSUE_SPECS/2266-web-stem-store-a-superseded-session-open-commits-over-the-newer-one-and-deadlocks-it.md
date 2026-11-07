# Web stem store: a superseded session open commits over the newer one and deadlocks it

Child of the implicit-state-machine tracker #2258 (item WEB-4). Found by its read-only audit of 2026-11-06.

## Mission
A `StemSessionGate` open that a newer open (or `close()`, or the caller's signal) has superseded must never become the gate's session, and no aborted open may keep waiting on a session pin lock or on its `resume()`.

## Verified defect (base `de0878e23`)
`hosts/host-web/web/stem-store/session-gate.js` `open()`:
- Line 91 checks `this.#opening !== opening` once, then line 95 `await this.#lease?.close()` yields while closing the predecessor lease (a `releaseSessionPin` index rewrite under the index lock, `opfs-store.js` 293). Lines 96-97 then set `#lease` and `#state = "interactive"` unconditionally and the open resolves with its lease. A newer open started inside that window is overwritten, and so is a `close()`. With no current lease the window is still there: `await undefined` is one microtask, and a newer open started from a callback on the resume promise lands in it (attempt 2's offset sweep is red on base at offset 2).
- Lines 85-88 wait for the store open and the `resume()` promise together with `Promise.all`, which ignores the abort. A superseded open whose resume has not settled keeps its granted lease, and so its pin, until the resume settles; if it never settles, it keeps it forever and the caller's abort cannot reach it.
- With the same session ID the newer open deadlocks. The superseded open holds the store lease it was granted, and so the exclusive session pin lock `session:<tab>:<id>`. The newer open sees the gate's `#lease` still on the predecessor (a different ID), so it releases nothing, and its `openSession` queues on that pin in `#acquireSessionPinLock` (`opfs-store.js` 1167-1185). When the superseded open commits, nothing closes the lease except a later `close()` or a third open of the same ID. The newer open never settles, and the gate reports `interactive` for the stale open's stem list.
- Lines 48-50: a `resume()` that throws synchronously refuses its open after that open has already aborted the one in flight and cleared `#opening`, so the aborted open's catch no longer writes the state; the gate stays `"loading"` with no open in flight.
- Lines 119-121: `close()` writes `#lease = undefined` and `#state = "idle"` after its await, so `close(); open()` reports `idle` while the open is loading.
- `#acquireSessionPinLock` requests the lock `{ mode: "exclusive" }` with no `signal` (`opfs-store.js` 1179-1181). It is the only wait in `openSession` that ignores the open's signal (`#withLock` passes it), so an aborted open stays queued for as long as another holder of the same pin (a second gate or a direct `openSession` lease on the same store) stays open.

Reachability: the window is the predecessor close, an index lock round trip with an OPFS read and write, and it needs a predecessor of a different session ID. A different-ID overwrite (the newer open finishing inside that window) is not reachable with one `OpfsStemStore`: the newer open's index requests queue FIFO behind the predecessor close's request, so the stale open resumes first. The same-ID path needs only that the newer open *starts* inside the window, and it is reachable.

## Decision / model
- The gate's ownership token is the open's own `AbortController`. Supersession, `close()` and the caller's signal already abort it, so "this open still owns the gate" is exactly `!opening.signal.aborted`. After its last await, and before writing `#lease` or `#state`, an open calls `opening.signal.throwIfAborted()`; the predecessor close is followed by the same check, and there is no await between the check and the commit. A superseded open therefore rejects with the abort reason and its existing catch path closes the lease it was granted, which releases the pin the newer open waits on.
- The wait for `resume()` ends at the abort (`abortable(resumeOutcome, opening.signal)`), after the store open has granted its lease, so a superseded open never keeps its lease for a resume that does not settle. The `throwIfAborted()` right after it stays: an abort can land after the resume settles and before the open's continuation runs, and with no current lease that check is the only one before the commit. So "aborted, therefore rejects and releases its lease" holds at every await; only the two bounded lease closes are awaited to completion.
- A synchronous `resume()` throw sets `"refused"` when the state is `"loading"`. `"loading"` means an open was in flight, and this open has just aborted it, so nothing else will write a terminal state. With nothing in flight the current session's state is left alone.
- `close()` makes every write before its await (abort the open, clear `#opening`, take `#lease` out, `"idle"`), so an open started while the old lease closes owns the state from then on.
- The catch path closes only `lease`. Every path into it has seen the store open settle, so the attempt-1 orphan-lease handling and its `opening.abort(error)` were dead and are gone.
- The displaced predecessor is taken out of `#lease` before its close is awaited, as the same-ID branch already does, so the gate never holds a lease that is closing.
- A change from base: an open whose caller's signal aborts after its lease was granted now rejects (before, it committed). This follows from the token being the controller, and is what an abort means.
- `#acquireSessionPinLock(pin, signal)` passes the open's signal to the Web Lock: an aborted open gives up its place in the queue (Web Locks semantics); a granted lock is unaffected.
- The test `FakeLockManager` now honours `signal` the way the Web Locks API does (an aborted signal rejects without queueing; an abort while queued rejects and leaves the queue; an abort after the grant changes nothing). Before, it ignored it, so no store test could see a lock wait's abort.

## Non-goals
- Follow-up, to be filed by the coordinator. A second `StemSessionLease.close()` returns at once while the first is still in flight (`opfs-store.js` 1282). `gate.close()` still does not wait for an aborted open's own lease close, so it can resolve before that pin is released. Attempt 3 fixed only the state half of `close()`. Base behaves the same and nothing deadlocks.
- No change to the render path, Rust code or the `.d.ts` (the `state` union and `open()` signature are unchanged).

## Gates
- `hosts/host-web/tests/stem-pump-v1.mjs` (run by `node scripts/check-stem-store-v1.mjs`, the qualification `stem-store` step):
  - `supersededOpenNeverCommits`: red on base (the superseded open resolves with its lease and the newer open times out), green after.
  - `supersededAsResumeSettlesNeverCommits` (attempt 2): a gate with no current lease, the first lease granted, and a newer open of the same session started at each microtask offset 0-8 after the first resume settles. Before the commit the first open must reject; after it, the newer open is a legitimate reopen. Every offset's newer open resolves with one pin, and the sweep must span the commit. Red on base and when the post-resume check is deleted.
  - `neverSettlingResumeReleasesItsLease` (attempt 2): the first resume never settles; a newer open of the same session supersedes it. Red on base and on attempt 1 (the first open never settles and the newer one deadlocks on its pin).
  - `neverSettlingResumeReleasesItsLease` has a second leg (attempt 3): the newer open starts from the first open's synchronous `"ready"` progress callback. That abort lands before the resume wait begins and fires no event, so only `abortable`'s early return ends the wait.
  - `throwingResumeLeavesATerminalState` (attempt 3): a throwing resume with an open in flight leaves `"refused"`; with nothing in flight it leaves the session `"interactive"`. Red on base.
  - `closeNeverOverwritesALaterOpen` (attempt 3): `close(); open()`. `idle` at once, `loading` after the close settles, then `interactive` with one pin. Red on base.
  - `closeReachesTheNewestOpen` (attempt 3): after a superseded open has settled, `close()` still aborts the newest open, which is queued on a pin another lease holds, and the gate ends `idle`. Red on base.
  - `supersededOpenNeverCommits` also asserts the newer open sees `"loading"` while it loads (attempt 3), and `pinLockWaitObeysAbort` also refuses an already-aborted caller signal.
  - `pinLockWaitObeysAbort`: red on base (the aborted open times out), green after.
  - The tests' lock-queue and readiness waits are bounded (`until`, 1,000 ticks) and throw, so a regression goes red instead of hanging the gate or the ledger.
- Seven entries in the nightly mutation ledger (`scripts/check-stem-store-v1.mjs --self-test`), each red at its target assertion: removing the post-close check; removing the post-resume check; waiting for the resume without the abort; removing `abortable`'s early return; dropping the throwing-resume refusal; letting `close()` write `idle` after its await; dropping the pin lock's `signal`.
- Every guard line in `session-gate.js` is in the attempt-3 guard table: deleting it turns a test red, or the table gives the reason it is equivalent.
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

**Verdict: FAIL** (Sol, one MAJOR, same kind as attempt 1). `abortable`'s early return (`if (signal.aborted) return Promise.reject(signal.reason)`, attempt 2 `session-gate.js` 138) was untested. An open can be aborted after `openSession`'s last `throwIfAborted`: during the signal-less `#mutateIndex`, or from the synchronous `"ready"` `onProgress` callback. No abort event fires after that, so the early return is the only way out. With it deleted, a newer same-session open started from the first open's `"ready"` callback left both opens timed out and the state `loading`, with every suite green. MINORs, folded into attempt 3: a synchronous `resume()` throw that aborts an open in flight leaves the gate `"loading"` for good (base too); `close()` writes `#lease`/`"idle"` after its await with no ownership check, so `close(); open()` reads `loading -> idle -> interactive`. The reviewer judged the offset sweep robust (the commit falls at offset 5; 32 parallel runs passed) and found no other commit or pin-retention path.

### Attempt 3 (Terra, 2026-11-07)
**Fix.** No new mechanism; the same ownership model, applied to the two state writes it missed:
- The synchronous-throw refusal leaves `"refused"` when it displaced an open in flight.
- `close()` makes all its writes before its await.
- The catch drops the dead orphan-lease code. The early return in `abortable` is unchanged and now tested.

**Red.**
- On base:
  - `neverSettlingResumeReleasesItsLease`: `a superseded open waiting on its resume kept its lease (after ready)`.
  - `throwingResumeLeavesATerminalState`: `a refused open left the gate loading with no open in flight` (actual `'loading'`).
  - `closeNeverOverwritesALaterOpen`: still `'interactive'` right after `close()`.
  - `closeReachesTheNewestOpen`: the first open never rejects.
- On attempt 2: `throwingResumeLeavesATerminalState` and `closeNeverOverwritesALaterOpen` are red with the same messages. The `"from ready"` leg passes on attempt 2 (which had the early return) and is red when the early return is deleted (table below).

**Guard table.** Each guard line in `session-gate.js` at the attempt-3 commit was deleted or weakened, and both gate test files were run (`stem-pump-v1.mjs`, then `stem-store-core-v1.mjs`). The first red assertion is listed.

| Line | Guard | Mutation | Result |
|---|---|---|---|
| 47 | a new open aborts the open in flight | delete | red: `a superseded open committed its lease over the newer open` |
| 52 | the caller's signal is forwarded | no forwarding | red: `an aborted open kept waiting on a held session pin` |
| 153 | `forwardAbort` honours an already-aborted caller signal | listener only | red: `an open whose caller's signal was already aborted went ahead` |
| 63 | a throwing resume refuses when it displaced an open in flight | delete | red: `a refused open left the gate loading with no open in flight` |
| 63 | ... only then | unconditional | red: `a refused resume displaced a session it never replaced` |
| 76 | `"loading"` on open | delete | red (existing gesture-gate test) |
| 84 | same-session predecessor released first | `if (false)` | red: `same-session replacement waited on its own predecessor pin lock` |
| 93 | the store open takes the opening's signal | delete | red: `an aborted open kept waiting on a held session pin` |
| 98 | the resume wait ends at the abort | plain await | red: `... kept its lease (after ready)` |
| 141 | `abortable` rejects an already-aborted signal | delete | red: `... kept its lease (from ready)` |
| 99 | a rejected resume refuses | delete | red: `Missing expected rejection` (`rejectedResumeDoesNotLeakPins`) |
| 101 | post-resume ownership check | delete | red: `an open superseded as its resume settled committed its lease (offset 1)` |
| 106 | post-close ownership check | delete | red: `a superseded open committed its lease over the newer open` |
| 118 | the catch closes the open's lease | delete | red: `the newer open deadlocked on the superseded open's session pin` |
| 119 | `"refused"` only while the owner | unconditional | red: `the superseded open's refusal overwrote the newer open's loading state` |
| 119 | `"refused"` on failure | delete | red (quota-failure test) |
| 123 | `finally` clears `#opening` only while the owner | unconditional | red: `close() no longer reached the newest open` |
| 130 | `close()` aborts the open in flight | delete | red: `close() no longer reached the newest open` |
| 131 | `close()` clears `#opening` | delete | red: `a closed open's refusal overwrote the closed gate's idle` |
| 134 | `close()` sets `"idle"` before its await | move after it (base) | red: `gate.state` not `idle` right after `close()` |
| 134 | ... and never after it | also write after it | red: `close() reported idle while a later open was loading` |
| 134 | `close()` sets `"idle"` | delete | red |
| `opfs-store.js` 1184 | the pin lock wait takes the signal | drop `signal` | red: `an aborted open kept waiting on a held session pin` |

Equivalent mutants, kept for the representation; green, with the reason none can be observed:
- Line 59 (the throwing-resume path clears `#opening`) and line 123 (`finally` clears `#opening` at all). `#opening` is only aborted, which is a no-op on a finished open's controller, or compared with the reader's own controller, which a finished open's controller never is.
- Lines 86, 104 and 133 (take `#lease` out before awaiting its close). `#lease` is private, and its only use is `close()`, which returns at once on a lease already closing or closed. Leaving the closing lease in place changes no outcome.
- Line 147 (`abortable` removes its listener). Listener hygiene on a per-open signal; no behaviour.

**Green.**
- `node scripts/check-stem-store-v1.mjs`: PASS. `stem-pump-v1.mjs` passed 10 serial runs and 24 parallel runs.
- `node scripts/check-stem-store-v1.mjs --self-test --budgets`: PASS, all seven #2266 ledger entries `RED` at their targets. `pinLockWaitObeysAbort` now runs before `closeReachesTheNewestOpen`, so the pin-signal entry keeps its own target.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.

**Test value.**
- `neverSettlingResumeReleasesItsLease` `"from ready"` leg: red if an open aborted before its resume wait begins still waits, since no abort event will fire.
- `throwingResumeLeavesATerminalState`: red if a throwing resume that displaced an open in flight leaves `"loading"`, or if one with nothing in flight touches the current session's state. No other test throws from `resume()` with an open in flight.
- `closeNeverOverwritesALaterOpen`: red if `close()` writes the state after its await. No other test opens during a close.
- `closeReachesTheNewestOpen`: red if a settled superseded open, or `close()` itself, loses track of the newest open. No other test closes the gate while an open is queued.
