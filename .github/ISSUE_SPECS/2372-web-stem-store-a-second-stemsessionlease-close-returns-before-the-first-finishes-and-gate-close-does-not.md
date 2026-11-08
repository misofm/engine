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
