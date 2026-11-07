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
- `hosts/host-web/tests/stem-pump-v1.mjs` is run by `node scripts/check-stem-store-v1.mjs`, the qualification `stem-store` step. Its #2266 tests:
  - `everyAbortSourceInEveryPhase`: the claim matrix below, one cell per abort source and phase. Each cell asserts four things:
    - the aborted open rejects with `AbortError` within 500 ms;
    - the gate ends in the state the source implies: `interactive` for a newer open, which must also see `loading` at its own `ready`; `idle` for `close()`; `refused` for the caller's signal;
    - the index holds exactly the newer open's pin, or none;
    - a direct `store.openSession({sessionId: "same"})` gets its lease within 500 ms, unless the newer open itself holds "same".
  - Its last leg is a caller signal that was already aborted.
  - `abortAsResumeSettlesNeverCommits`: the matrix's last column. Each source lands at microtask offsets 0-8 after the first open's resume settles, with no current lease. It applies the same assertions before the commit, and it must span the commit for every source.
  - `throwingResumeLeavesATerminalState`: a synchronous `resume()` throw has three legs:
    - with an open in flight, it leaves `refused`;
    - with only a current lease, it leaves the lease `interactive`;
    - with an open in flight behind a current lease, it leaves `refused`.
  - `closeNeverOverwritesALaterOpen`: for `close(); open()`, the state is `idle` at once, `loading` once the close settles, then `interactive`.
  - `closeReachesTheNewestOpen`: after a superseded open has settled, `close()` still aborts the newest open, and the gate ends `idle`.
  - Every lock-queue and readiness wait is bounded (`until`, 1,000 ticks) and throws.
- Ten #2266 entries in the nightly mutation ledger (`scripts/check-stem-store-v1.mjs --self-test`), each red at its target assertion:
  - the post-close check;
  - the post-resume check;
  - waiting for the resume without the abort;
  - `abortable`'s early return;
  - the throwing-resume refusal;
  - `close()` writing `idle` after its await;
  - removing the caller-signal forwarding once the lease is granted (M14);
  - checking ownership by identity, not by the signal (M5);
  - refusing a throwing resume only when no lease is current (M10);
  - the pin lock's `signal`.
- The claim matrix below has every cell filled. The attempt-4 guard table lists every guard line in `session-gate.js`: deleting it turns a test red, or the table gives the reason the mutant is equivalent.
- `node scripts/check-stem-store-v1.mjs` and `--self-test --budgets` pass; `bash scripts/test-web-audioworklet.sh` and `bash scripts/check-workspace-policy.sh` pass.

### Claim matrix
The claim: an open aborted by any source, in any phase, rejects with `AbortError`, never commits and never keeps its session pin. The gate ends `interactive` when a newer open aborted it (and that open saw `loading` while it loaded), `idle` after `close()`, and `refused` after the caller's signal. `E` is `everyAbortSourceInEveryPhase`; `S` is `abortAsResumeSettlesNeverCommits`. All 20 cells are reachable, and each runs.

| Abort source | Queued on a lock (the session pin, held by another lease) | From the store's `ready` callback (after `openSession`'s last signal check; no abort event reaches the resume wait) | During the resume wait | During the predecessor close | Between the last await and the commit |
|---|---|---|---|---|---|
| Newer open, same session (changes `#opening`) | E `(same-session open, queued on the pin)` | E `(same-session open, from ready)` | E `(same-session open, resume wait)` | E `(same-session open, predecessor close)` | S, same-session open, offsets 0-8 |
| Newer open, another session (changes `#opening`) | E `(other-session open, queued on the pin)` | E `(other-session open, from ready)` | E `(other-session open, resume wait)` | E `(other-session open, predecessor close)` | S, other-session open, offsets 0-8 |
| `close()` (clears `#opening`) | E `(close(), queued on the pin)`; also `closeReachesTheNewestOpen` | E `(close(), from ready)` | E `(close(), resume wait)` | E `(close(), predecessor close)` | S, `close()`, offsets 0-8 |
| The caller's signal (leaves `#opening` alone) | E `(caller signal, queued on the pin)`; already aborted: E's last leg | E `(caller signal, from ready)` | E `(caller signal, resume wait)` | E `(caller signal, predecessor close)` | S, caller signal, offsets 0-8 |

The first column holds the session pin, the lock this issue gave a signal. The ingest and index locks already took the opening's signal in `#withLock`. To the gate, every lock wait ends the same way: the store rejects with the abort reason before granting a lease. The predecessor column needs a current lease of another session ID. A same-ID predecessor is closed before the store open (the `sameSessionReplacementReleasesPredecessorPin` path), so that window is the first column's. A fifth source, a newer open whose `resume()` throws synchronously, aborts in the same way; `throwingResumeLeavesATerminalState` holds its state.

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

**Guard table.** Attempt 3's table is superseded by attempt 4's, which uses the folded test names and adds M5, M14, M10 and the `removeParentAbort()` rows.

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

**Verdict: FAIL** (Sol, one MAJOR). The reviewer re-verified the 23-row guard table, the six equivalent mutants and the dead catch code; all held. The MAJOR: no test aborted an open through the caller's signal after its lease was granted. Supersession and `close()` both change `#opening`, so no test could tell the signal checks from base's identity check. Two mutations stayed green on every suite, and under either one the caller-aborted open committed and held the pin, so a direct same-session `openSession` timed out (the headline deadlock):
- M14: `removeParentAbort()` right after the store open.
- M5: both `opening.signal.throwIfAborted()` replaced by `this.#opening !== opening` checks.

MINORs:
- An open in flight behind a current lease, then a throwing resume: M10 (`#state === "loading"` → `#lease === undefined`) stayed green.
- `removeParentAbort()` at lines 60 and 122 was missing from the equivalent mutants.
- The catch comment should read "…or never started it".

The reviewer asked for a claim matrix (abort source × phase) to replace the guard-line table.

### Attempt 4 (Terra, 2026-11-07)
**Merge.** `origin/main` (merge `9fe8a979f`, with #2263) was merged in with a normal merge commit. Both issues' tests and ledger entries survive, and every entry from both is `RED` at its target under `--self-test --budgets`.

**Change.** No product change beyond the comment fix: attempt 3's code already passes the full matrix. The tests are folded into the claim matrix:
- `everyAbortSourceInEveryPhase` and `abortAsResumeSettlesNeverCommits` (the matrix above) replace `supersededOpenNeverCommits`, `supersededAsResumeSettlesNeverCommits`, `neverSettlingResumeReleasesItsLease` and `pinLockWaitObeysAbort`. Each of those was one row or one cell of the matrix, so they were deleted, and their ledger entries were retargeted to the matrix cell that now holds them.
- `throwingResumeLeavesATerminalState` has its third leg, the open in flight behind a current lease.

**Red.**
- Base: `the aborted open never settled (same-session open, from ready)`. With the matrix removed, the sweep is red: `the aborted open committed (same-session open, after the resume settles, offset 2)`.
- M14: `the aborted open never settled (caller signal, resume wait)`.
- M5: `the aborted open committed (caller signal, predecessor close)`.
- M10: `a refused open left the gate loading behind a current lease with no open in flight`.
- Attempt-3 code: green, as expected (the MAJOR was a test gap).

**Guard table** (replaces attempt 3's). Each guard line in `session-gate.js` at the attempt-4 commit was deleted or weakened, and both gate test files were run. The first red assertion is listed; `E` and `S` are the matrix tests.

| Line | Guard | Mutation | Result |
|---|---|---|---|
| 47 | a new open aborts the open in flight | delete | red: E `never settled (same-session open, queued on the pin)` |
| 52 | the caller's signal is forwarded | no forwarding | red: E `never settled (caller signal, queued on the pin)` |
| 95 | ... until the open settles | M14: stop forwarding once the lease is granted | red: E `never settled (caller signal, resume wait)` |
| 153 | `forwardAbort` honours an already-aborted caller signal | listener only | red: E's last leg `an open whose caller's signal was already aborted went ahead` |
| 63 | a throwing resume refuses when it displaced an open in flight | delete | red: `a refused open left the gate loading with no open in flight` |
| 63 | ... whenever an open was in flight | M10: only with no lease | red: `... loading behind a current lease with no open in flight` |
| 63 | ... and only then | unconditional | red: `a refused resume displaced a session it never replaced` |
| 76 | `"loading"` on open | delete | red (gesture-gate test) |
| 84 | same-session predecessor released first | `if (false)` | red: `same-session replacement waited on its own predecessor pin lock` |
| 93 | the store open takes the opening's signal | delete | red: E `never settled (same-session open, queued on the pin)` |
| 98 | the resume wait ends at the abort | plain await | red: E `never settled (same-session open, resume wait)` |
| 141 | `abortable` rejects an already-aborted signal | delete | red: E `never settled (same-session open, from ready)` |
| 99 | a rejected resume refuses | delete | red: `Missing expected rejection` (`rejectedResumeDoesNotLeakPins`) |
| 101 | post-resume check, by the signal | delete | red: S `committed (same-session open, after the resume settles, offset 1)` |
| 101, 106 | ... by the signal, not identity | M5: `this.#opening !== opening` | red: E `committed (caller signal, predecessor close)` |
| 106 | post-close check | delete | red: E `committed (same-session open, predecessor close)` |
| 118 | the catch closes the open's lease | delete | red: E `the newer open never settled (same-session open, resume wait)` |
| 119 | `"refused"` only while the owner | unconditional | red: E `refusal overwrote the newer open's loading state (same-session open, queued on the pin)` |
| 119 | `"refused"` on failure | delete | red: E `the gate's state after the abort (caller signal, queued on the pin)` |
| 123 | `finally` clears `#opening` only while the owner | unconditional | red: `close() no longer reached the newest open` |
| 130 | `close()` aborts the open in flight | delete | red: E `never settled (close(), queued on the pin)` |
| 131 | `close()` clears `#opening` | delete | red: E `the gate's state after the abort (close(), queued on the pin)` |
| 134 | `close()` sets `"idle"` before its await | move after it (base) | red: `closeNeverOverwritesALaterOpen`, not `idle` right after `close()` |
| 134 | ... and never after it | also write after it | red: `close() reported idle while a later open was loading` |
| 134 | `close()` sets `"idle"` | delete | red: E `the gate's state after the abort (close(), queued on the pin)` |
| `opfs-store.js` 1184 | the pin lock wait takes the signal | drop `signal` | red: E `never settled (same-session open, queued on the pin)` |

Equivalent mutants. They are kept for the representation, and each stays green for the reason given:
- **Lines 59 and 123** (the throwing-resume path clears `#opening`; `finally` clears it at all). `#opening` is only ever aborted, which does nothing to a finished open's controller, or compared with the reader's own controller, which a finished open's controller never is.
- **Lines 86, 104 and 133** (take `#lease` out before awaiting its close). `#lease` is private, and its only use is `close()`, which returns at once on a lease already closing or closed.
- **Lines 60, 122 and 147** (`removeParentAbort()` on the throwing-resume path and in `finally`; `abortable` removing its listener). Listener hygiene. After an open settles, an abort reaching its controller or its finished race changes nothing.

**Green.**
- `node scripts/check-stem-store-v1.mjs`: PASS. `stem-pump-v1.mjs` passed 10 serial runs and 32 parallel runs.
- `node scripts/check-stem-store-v1.mjs --self-test --budgets`: PASS. All ten #2266 entries and all #2263 entries are `RED` at their targets.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.

**Test value.**
- `everyAbortSourceInEveryPhase`: red if any abort source, in any open phase, leaves the open committed, waiting, holding its pin, or the gate in the wrong state. The caller-signal row is the only one that tells a signal-based ownership check from an identity check (M5), or a forwarding that stops early (M14).
- `abortAsResumeSettlesNeverCommits`: red if an abort from any source between the resume settling and the commit still commits.
- `throwingResumeLeavesATerminalState` third leg: red if the throwing-resume refusal keys on the lease rather than on an open being in flight (M10).
