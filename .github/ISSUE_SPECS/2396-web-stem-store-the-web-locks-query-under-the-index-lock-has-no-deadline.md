# Web stem store: the Web Locks query under the index lock has no deadline

Successor of #2381, which deadlined every OPFS step made while holding the stem store's index lock. This item was left out because it is a Web Locks call, not OPFS.

## Problem
In `hosts/host-web/web/stem-store/opfs-store.js`, `#liveLockNames()` awaits `navigator.locks.query()` with no deadline. Two callers run it while holding the index lock:
- the store's first open (`#openOnce`), before its debris sweeps;
- `#rebuildIndex`, before it scans the store's finals.

If that query never settles, the index lock is held forever. Every later index mutation then blocks, including every lease close's pin release, so `StemSessionGate.close()` and `StemSessionLease.close()` hang. #2372's pin-phase model and #2381's contract both rest on the index work being bounded.

`#liveLockNames()` already returns `null` when the query throws, and both callers treat `null` as "lock state unknown". The sweeps then fall back to the staging age check, and the rebuild hashes every final.

## Contract
- The index-lock wait on `locks.query()` has the store's local-store deadline (`readDeadlineMs`). On expiry `#liveLockNames()` behaves as it does when the query throws: it returns `null`, and its callers take their existing unknown-lock-state paths.
- A query that settles late changes nothing. Its result is dropped.

## Non-goals
- No change to the `.d.ts`, the lock names, the render path or Rust.
- No deadline on lock *acquisition*. A lock wait is bounded by its holder and, where the caller has one, by its signal.

## Gates
- Red on base: a `FakeLockManager` whose `query()` never settles makes a lease close that triggers a rebuild (a damaged index) settle within the deadline. A store `open()` against that lock manager settles too.
- `node scripts/check-stem-store-v1.mjs` and `node scripts/check-stem-store-v1.mjs --self-test --budgets` are green, with a ledger entry for the deadline.
- The suite runs in Node against the fakes, so the two-browser gate does not apply.

## Evidence / attempt record
### Attempt 1 (Terra, 2026-11-08)
Base `343cbfd36`; branch `codex/issue-2396-locks-query-deadline`.

**Claims re-verified on base.** Line numbers are base `opfs-store.js`.
- `#liveLockNames` (1216-1229) awaits `this.#locks.query()` (1219) with no deadline. A throw returns `null`.
- `#openOnce` calls it inside `#withIndexLock` (123), and `#rebuildIndex` calls it (1014). The rebuild runs under the index lock through `#readIndex(true)`, which every `#mutateIndex` uses, so a lease close's pin release reaches it on a damaged index.
- The `null` paths: `#purgeDeadSessionPins` purges nothing (1262). `#removeUnindexedFinals` removes every unindexed final (1207), each through `#removeFinal`'s stem lock taken without waiting. The rebuild hashes every final, and removes through the same primitive. `#sweepStaging` falls back to the 30-second age check (1239-1250) and removes through a raw `removeEntry`, with no lock.

**Decision: what the callers do on expiry.** A query that throws or makes no progress within `readDeadlineMs` leaves the lock state unknown, and `#liveLockNames` returns `null`, as the contract says. A late answer is dropped by the `withDeadline` race. Each caller's `null` path was checked against the rule that it must lose no pin and remove only what it can prove dead:
- The pin purge purges nothing. Kept.
- The finals sweep and the rebuild remove a final only after taking its stem lock without waiting. A live holder in any tab keeps the lock, so the removal is proven without the snapshot. Kept. The rebuild still runs, so a damaged index heals.
- The staging sweep is the one exception to the contract's "existing unknown-lock-state paths". Its age check is not proof. A live download writes its staged file through a writable, and that writable need not update the file's `lastModified` before it closes, so a download longer than 30 seconds can look dead. In a spec-compliant browser, an open writable's File System lock also makes the sweep's raw `removeEntry` fail. The guard still stands, because the store must not rely on that lock, and the Node fakes do not model it. Neither behaviour was verified in a browser for this issue. Under Web Locks, the only proof that a staged file is dead is that its stem lock is free. So when the store has Web Locks requests and no answered query, the staging sweep removes nothing. It reports `storage.maintenance_stalled` to the maintenance hook, and the open succeeds. The check is `typeof this.#locks?.request === "function"`, the same test `#acquireStemLock` uses to decide that stem locks are cross-tab, so a lock manager with `request` but no `query` is covered too (fold, below). The age check stays only for a store with no Web Locks at all.
- This applies to a query that throws as well as one that stalls, so there is one rule for "the query gave no answer". On base a throwing query age-swept staging. That is the same hazard, and the `query throws` leg below is red on base for it.
- Failing the open typed was considered and rejected. The sweeps are best-effort by design (`openSurvivesFailingSweeps`), and failing typed would make a stalled lock manager block every mix for nothing the open needs.

**Fix** (`hosts/host-web/web/stem-store/opfs-store.js`).
- `#liveLockNames` races `locks.query()` with `withDeadline(…, readDeadlineMs, "storage.maintenance_stalled")`. The existing catch turns the expiry into `null`.
- `#sweepStaging` throws `storage.maintenance_stalled` before listing when `liveLocks === null` and the store has a lock query. `#bestEffortSweep` reports it.
- Fake: `FakeLockManager.queryBlocker`, awaited in `query()` only when set.

**Tests** (`hosts/host-web/tests/stem-store-core-v1.mjs`).
- `aStalledLockQueryNeverHoldsTheIndex`: the index is damaged and the query never settles. A lease close, whose pin release rebuilds the index, must settle with no error, and the rebuild must adopt the good final. Test value: red if the query under the index lock has no deadline, which stalls every later index mutation, every lease close among them. No other test stalls the lock query.
- `anUnknownLockStateSweepsNothingStaged`, legs `query throws` and `query stalls`. Another tab is mid-download, and the opening store's clock makes the staged file look a minute old. The open must settle with no error, keep the other session's live pin, keep the staged file and report the staging sweep. The download then completes and reads. Test value: red if an open without an answered query removes a live download's staged file, if it reads an unanswered query as "no live locks" and purges a live pin, or (the `query stalls` leg) if the first open's query has no deadline. No other test fails the lock query.
- The contract's separate "store open settles" gate is the `query stalls` leg's first assertion, so it is not a second row.

**Red on base** (each test or leg run alone against base `opfs-store.js`):
- `aStalledLockQueryNeverHoldsTheIndex`: `a lease close waited on a lock query that never settles` (actual `'timed-out'`).
- `query throws`: `an open with unknown lock state removed a live download's staged file (query throws)` (actual `[]`).
- `query stalls`: `a store open waited on its lock query (query stalls)` (actual `'timed-out'`).

**Mutation ledger** (`scripts/check-stem-store-v1.mjs`, a new #2396 block). Each entry is `RED` at its target:
- the query with no deadline: `a lease close waited on a lock query that never settles`;
- an unanswered query read as an empty lock set: `an open with unknown lock state dropped a live pin (query throws)`;
- the staging guard dropped, which is the contract's literal `null` path: `an open with unknown lock state removed a live download's staged file (query throws)`. Run alone, the `query stalls` leg is red at the same assertion, so a deadline without the guard fails both legs.

**Green** at this commit:
- `node scripts/check-stem-store-v1.mjs`: PASS.
- `node scripts/check-stem-store-v1.mjs --self-test --budgets`: PASS, 97 `RED` lines (94 before, plus the 3 new entries).
- `stem-store-core-v1.mjs --budgets`: 4 parallel runs, all PASS.
- `stem-pump-v1.mjs`: PASS.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- Not applicable: no Rust and no render path changed, so cargo, the realtime policy tool, no-bit-moved and loom do not apply. The suite runs in Node against the fakes, so the two-browser gate does not apply.

**Docs.** `docs/STEM_STORE_V1.md` states the query deadline, that a late answer is dropped, that an unanswered query purges no pin and sweeps no staged file, that the age check is only for a store with no lock query, and why the finals sweep and a rebuild still run.

**#2381's binding model is unchanged.** The only new deadline is on an index-lock step. No removal of a final changes, and none gains a deadline. No new stall rule is added: an unanswered query is unknown state, not damage. `session-gate.js` and #2372's pin-phase model are untouched.

**Verdict: PASS** (Sol, minors only). Folded on the same branch:
1. The staging guard is keyed on `typeof this.#locks?.request === "function"`, as `#acquireStemLock` is, not on `query`. This removes attempt 1's follow-up, in which a lock manager with `request` but no `query` still age-swept the staged files of live cross-tab writers. The code comment, `docs/STEM_STORE_V1.md` and the ledger's search string are updated to match.
2. The age-check path had no test: both `if (liveLocks === null)` → `if (false)` and `oldEnough = true` kept the suite green. There is a new no-Web-Locks row, `theAgeCheckSweepsOnlyOldStagedFilesWithoutWebLocks`. A staged file a minute old is removed, and a fresh one is kept.
3. Decision bullet 3 and `docs/STEM_STORE_V1.md` now say that in a spec-compliant browser an open writable's File System lock makes the raw `removeEntry` fail. They also say that the guard stands anyway, because the store must not rely on that lock and the fakes do not model it. The hazard is not presented as browser-verified.

**Fold tests.**
- `anUnknownLockStateSweepsNothingStaged` gains a `no lock query` leg. The `FakeLockManager` keeps `request` and loses `query`. Red on the attempt-1 store `11f4a52ce`: `an open with unknown lock state removed a live download's staged file (no lock query)`. Test value: red if the guard is keyed on the lock query rather than on Web Locks requests.
- `theAgeCheckSweepsOnlyOldStagedFilesWithoutWebLocks`. Test value: red if the no-Web-Locks sweep skips the age check and removes a fresh, possibly live, staged file, or never removes an old dead one. No other test reaches the age check.

**Fold ledger.** Four new entries, each `RED` at its target:
- the guard keyed on `query`: `... staged file (no lock query)`;
- `if (liveLocks === null) {` → `if (false) {`: `the age check removed a fresh staged file`;
- `oldEnough = true`: `the age check removed a fresh staged file`;
- `oldEnough = false` (an old file never counts as dead): `the age check kept a dead staged file`. `if (oldEnough)` → `if (false)` was tried first and dropped: it also disables the Web Locks path removal, so `tabCloseMidStaging` already goes red at it, outside this row.

**Green after merging `origin/main` (`9806fdce0`):**
- `node scripts/check-stem-store-v1.mjs`: PASS.
- `node scripts/check-stem-store-v1.mjs --self-test --budgets`: PASS, 101 `RED` lines (97 at attempt 1, plus the 4 fold entries).
- `stem-store-core-v1.mjs --budgets`: 4 parallel runs, all PASS.
- `stem-pump-v1.mjs`: PASS.
- `bash scripts/check-workspace-policy.sh`: ok.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
