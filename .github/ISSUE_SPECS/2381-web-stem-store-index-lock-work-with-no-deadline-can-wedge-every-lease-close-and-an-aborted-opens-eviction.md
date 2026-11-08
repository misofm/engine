# Web stem store: index-lock work with no deadline can wedge every lease close, and an aborted open's eviction runs on

Found during #2372's reviews. All three items are pre-existing, and none is made worse by #2372.

## Problem
In `hosts/host-web/web/stem-store/opfs-store.js`:
1. `#evictUnpinned` calls `removeEntry` inside its `#mutateIndex` with no deadline, while it holds the index lock. A wedged OPFS `removeEntry` therefore blocks every later index mutation. That includes every lease close's pin release, so `gate.close()` and `lease.close()` hang. This happens on base too.
2. `#readIndex`'s `getFileHandle(INDEX_FILE)` has no deadline under the index lock, unlike the deadlined reads and writes around it. A wedge has the same effect as item 1.
3. `#evictUnpinned` takes no signal, so an aborted open's eviction still runs once its stem lock frees. It holds no pin, so this costs work and is not a correctness issue.

## Contract
- Every OPFS operation made while holding the index lock has a deadline, matching the existing `withDeadline` convention. On expiry the operation fails typed, and the lock is released.
- An aborted open performs no eviction after its abort is observed.

## Gates
- Red on base: one row each for (1) and (2), using the fakes' wedge hooks, asserting that a later `lease.close()` settles within the deadline. One row for (3), asserting that no stem is removed after the abort.
- `node scripts/check-stem-store-v1.mjs` and `--self-test --budgets` green, with a ledger entry per row.

## Evidence / attempt record
(empty)
