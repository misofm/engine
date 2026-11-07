# Web stem store: a superseded session open commits over the newer one and deadlocks it

Child of the implicit-state-machine tracker #2258 (item WEB-4). Found by its read-only audit of 2026-11-06.

## Mission
A `StemSessionGate` open that a newer open (or `close()`, or the caller's signal) has superseded must never become the gate's session, and no open may wait unabortably on a session pin lock.

## Verified defect (base `de0878e23`)
`hosts/host-web/web/stem-store/session-gate.js` `open()`:
- Line 91 checks `this.#opening !== opening` once, then line 95 `await this.#lease?.close()` yields while closing the predecessor lease (a `releaseSessionPin` index rewrite under the index lock, `opfs-store.js` 293). Lines 96-97 then set `#lease` and `#state = "interactive"` unconditionally and the open resolves with its lease. A newer open started inside that window is overwritten, and so is a `close()`.
- With the same session ID the newer open deadlocks. The superseded open holds the store lease it was granted, and so the exclusive session pin lock `session:<tab>:<id>`. The newer open sees the gate's `#lease` still on the predecessor (a different ID), so it releases nothing, and its `openSession` queues on that pin in `#acquireSessionPinLock` (`opfs-store.js` 1167-1185). When the superseded open commits, nothing closes the lease except a later `close()` or a third open of the same ID. The newer open never settles, and the gate reports `interactive` for the stale open's stem list.
- `#acquireSessionPinLock` requests the lock `{ mode: "exclusive" }` with no `signal` (`opfs-store.js` 1179-1181). It is the only wait in `openSession` that ignores the open's signal (`#withLock` passes it), so an aborted open stays queued for as long as another holder of the same pin (a second gate or a direct `openSession` lease on the same store) stays open.

Reachability: the window is the predecessor close, an index lock round trip with an OPFS read and write, and it needs a predecessor of a different session ID. A different-ID overwrite (the newer open finishing inside that window) is not reachable with one `OpfsStemStore`: the newer open's index requests queue FIFO behind the predecessor close's request, so the stale open resumes first. The same-ID path needs only that the newer open *starts* inside the window, and it is reachable.

## Decision / model
- The gate's ownership token is the open's own `AbortController`. Supersession, `close()` and the caller's signal already abort it, so "this open still owns the gate" is exactly `!opening.signal.aborted`. After its last await, and before writing `#lease` or `#state`, an open calls `opening.signal.throwIfAborted()`; the predecessor close is followed by the same check, and there is no await between the check and the commit. A superseded open therefore rejects with the abort reason and its existing catch path closes the lease it was granted, which releases the pin the newer open waits on.
- The displaced predecessor is taken out of `#lease` before its close is awaited, as the same-ID branch already does, so the gate never holds a lease that is closing.
- A change from base: an open whose caller's signal aborts after its lease was granted now rejects (before, it committed). This follows from the token being the controller, and is what an abort means.
- `#acquireSessionPinLock(pin, signal)` passes the open's signal to the Web Lock: an aborted open gives up its place in the queue (Web Locks semantics); a granted lock is unaffected.
- The test `FakeLockManager` now honours `signal` the way the Web Locks API does (an aborted signal rejects without queueing; an abort while queued rejects and leaves the queue; an abort after the grant changes nothing). Before, it ignored it, so no store test could see a lock wait's abort.

## Non-goals
- `StemSessionLease.close()` returns at once on a second call while the first is still in flight, so `gate.close()` during a predecessor close can resolve before that pin is released. Separate issue if wanted.
- An open waiting on a `resume()` promise that never settles keeps its granted lease until it settles (`Promise.all` on line 85 does not observe the abort). Not reported by the audit; separate issue if wanted.
- No change to the render path, Rust code or the `.d.ts` (the `state` union and `open()` signature are unchanged).

## Gates
- `hosts/host-web/tests/stem-pump-v1.mjs` (run by `node scripts/check-stem-store-v1.mjs`, the qualification `stem-store` step):
  - `supersededOpenNeverCommits`: red on base (the superseded open resolves with its lease and the newer open times out), green after.
  - `pinLockWaitObeysAbort`: red on base (the aborted open times out), green after.
- Two entries in the nightly mutation ledger (`scripts/check-stem-store-v1.mjs --self-test`): removing the post-close ownership check, and dropping the pin lock's `signal`, each turn their target assertion red.
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
- Remove the check before the predecessor close (the base's own line-91 check, kept): green. No existing test holds it; without it a superseded open closes a predecessor of another session ID before the newer open is green. Pre-existing coverage gap, not this issue's claim.
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
