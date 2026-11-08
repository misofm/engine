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
(empty)
