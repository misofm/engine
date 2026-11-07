# Web stem store: a failed pump Worker session keeps answering seek and spins

Tracker: #2258, item WEB-3. Found by the implicit-state-machine audit of 2026-11-06.

## Problem
The PCM pump Worker (`hosts/host-web/web/stem-store/pcm-pump-worker.js`) kept its phase in loose module variables (`pump`, `lease`, `selfDrivingIdleMs`, `driveToken`, `idleWake`; base lines 15-24). `reportSessionError` (base ~208) only called `pump?.stop()`: the stopped pump stayed installed, the self-driving cadence stayed configured, and nothing recorded that the session had failed.

- After a failure, `seek` (base ~69) still called `pump.seek()`, which on a stopped `CanonicalPcmPump` (`pcm-pump.js` `seek`, base ~260) re-tagged and re-seeked a ring whose writer was already released, and resolved. The Worker answered `sought` and re-armed the cadence: every pass of a stopped pump returned zero chunks and `finished: false` (base `pumpUntilFull`, ~242), so the loop slept `idleMs` forever, silently, after a success reply.
- A failure on the message path while the cadence ran (for example a `seek` the pump refused) left the drive token live: the loop kept ticking against the stopped pump with no seek at all.
- A `pump` message after a failure was answered `pumped` with zero chunks.

## Scope
- The Worker's session is one explicit phase, `idle | running{pump, lease, idleMs, drive, idleWake} | failed{error, lease}`. A running session is left only through `leaveRunning` (ends the loop, cancels the idle timer, stops the pump), on `stop`, `initialize` or failure. `failed` holds no pump and no cadence, so nothing can drive it.
- In `failed`, `seek` and `pump` are refused with a `session-error` answering their `requestId`, code `stem.pump.session_failed`, the first failure in `details.failure`; `stop` closes the lease and returns to `idle`; `initialize` starts a new session.
- A cadence pass's failure is recorded inside the message tail (`tick`), so no message lands between the failed pass and the phase that records it.
- `CanonicalPcmPump.seek` and `pumpUntilFull` on a stopped pump reject with `CanonicalPcmPumpError` `stem.pump.stopped` and touch no ring. `BrowserEngine.seek` then reports `SessionSeekProducerError` for a dead producer instead of succeeding.
- `initialize` validates `selfDriving` before any ring is engaged.
- No render-path or Rust change.

## Non-goals
- `session-gate.js` and `opfs-store.js` (WEB-4 owns those).
- Whether an argument error in a `seek` message should fail the session at all (today every Worker error is a session error; unchanged).

## Gates
- A Worker test, red on today's code: after a pump failure under `selfDriving`, `seek` and `pump` are answered `session-error` (`stem.pump.session_failed`) with their `requestId`, the ring's seek epoch does not move, and no idle timer is left; after a message-path failure the cadence's timer is cancelled.
- A pump test, red on today's code: `seek` and `pumpUntilFull` on a stopped pump reject `stem.pump.stopped` and leave the ring's seek epoch and generation tag unchanged.
- `node scripts/check-stem-store-v1.mjs` and its `--self-test` mutation ledger (with new mutations for the fix) pass; `bash scripts/test-web-audioworklet.sh` and `bash scripts/check-workspace-policy.sh` pass.

## Evidence
### Attempt 1 (Terra, 2026-11-07)
Base `de0878e23`.

**Verified on base by reading and by the tests below.** The tracker's claim holds as stated, plus the message-path spin and the `pumped` zero answer above.

**Red on base** (tests added in `hosts/host-web/tests/stem-pump-v1.mjs` before the fix; each Worker test run with the earlier new tests skipped so it reached its own assertion):
- `stoppedPumpRefusesSeekAndPump`: `AssertionError [ERR_ASSERTION]: Missing expected rejection (stopped): a stopped pump accepted a seek`.
- `failedSessionRefusesSeekAndPump`: `AssertionError [ERR_ASSERTION]: a failed session answered \`seek\` with sought` (`actual: 'sought', expected: 'session-error'`).
- `messagePathFailureStopsTheCadence`: `AssertionError [ERR_ASSERTION]: a message-path failure left the cadence sleeping` (`actual: 1, expected: 0`: the 30 s idle timer outlived the failure).

**Mutations** (added to `scripts/check-stem-store-v1.mjs`'s ledger, run by `--self-test`; each went red at its named assertion):
- `let a stopped pump seek` (drop the refusal in `seek`): `a stopped pump accepted a seek`.
- `let a stopped pump report a pass` (drop the refusal in `pumpUntilFull`): `a stopped pump reported a pass`.
- `leave a failed Worker session running` (drop the `running -> failed` transition): `the refusal must come from the failed session, not from a pump it still held` (the seek then reaches the stopped pump and is refused with the pump's `stem.pump.stopped`).
- `fail a Worker session without ending its cadence` (drop `leaveRunning` on failure): `a message-path failure left the cadence sleeping`.
- The existing #278 mutation `let stop leave the idle sleep pending` was re-pointed at `leaveRunning`'s wake and still goes red at `stop cancelled the idle timer rather than waiting it out`.

**Gates run:**
- `node scripts/check-stem-store-v1.mjs`: PASS. `node scripts/check-stem-store-v1.mjs --self-test`: PASS, every mutation RED at its target.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cd sdk && node --test test/browser-pcm-evals.mjs`: 47 pass, 3 fail; the three failures are the artifact-backed `actual Wasm` evals (`MISO_ENGINE_SDK_ARTIFACTS_HEX must encode a directory built by scripts/build-web-audioworklet.sh`), no artifact being built here. The package `CanonicalPcmPump` eval and the pump seek eval pass.
- No Rust and no render path changed, so no cargo, clippy, fmt, realtime-policy or no-bit-moved comparison applies.

**Test value.**
- `stoppedPumpRefusesSeekAndPump`: red if a stopped `CanonicalPcmPump` accepts a seek (re-tagging a released ring) or answers a pass with a zero-chunk success; no existing test calls the pump after `stop`.
- `failedSessionRefusesSeekAndPump`: red if a Worker session that failed on the cadence keeps answering `seek`/`pump` with success, re-arms the loop, or refuses only through a pump it should no longer hold; the #278 tests never fail a session.
- `messagePathFailureStopsTheCadence`: red if a failure that arrives on a message leaves the self-driving loop and its idle timer alive, or leaves the ring writer engaged; the drive-path test cannot see this because a cadence-pass failure ends its own loop.
