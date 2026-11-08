# Web stem store: OPFS awaits under a stem lock have no deadline

Successor of #2381. #2381 deadlined every OPFS step made while holding the stem store's index lock. It decided that a removal of a final has no deadline, and that the stem lock stays held until the removal settles. The other OPFS awaits made while holding a stem lock were out of its scope.

## Problem
In `hosts/host-web/web/stem-store/opfs-store.js`, these awaits run inside a hold of a stem lock (`#withStemLock`) with no deadline. They are not removals of a final:
- `#fileHandle(stemFileName(...))` in `verify()` and in `#verifyIndexed`.
- In `#ingestAttempt`:
  - `removeEntry(this.#staging, stagingName)` before staging;
  - `this.#staging.getFileHandle(stagingName, { create: true })`;
  - the staging cleanup `removeEntry` in its `finally`.
- In `#promote`:
  - `stagingHandle.move(this.#directory, finalName)`;
  - the fallback's `this.#directory.getFileHandle(finalName, { create: true })`.
- The quota path's awaited `removeEntry` of the final in `#ingestAttempt`.

If any of these never settles, the hold never ends, and the stem lock stays held. An open that waits for it waits until its own abort. A `verify()` called with no signal, or a joiner of that ingest, waits forever. The open holding the lock never settles either. It holds no session pin at that point, so `StemSessionGate.close()` is not affected (#2372).

## Contract
- Every OPFS await made in a stem-lock hold that is not a removal of a final has the local-store deadline (`readDeadlineMs`), or the ingest deadline where it is part of an ingest. On expiry it fails typed with the code its neighbours use (`storage.read_stalled`, `storage.write_stalled` or `storage.remove_stalled`).
- An operation that times out but may still land keeps the stem lock until it settles: it is fenced into the hold (`#fence`), as removals of a final already are.
- A removal of a final keeps #2381's rule: it has no deadline, and its lock outlives it.

## Non-goals
- No change to the index-lock deadlines, the `.d.ts`, the render path or Rust.

## Gates
- Red on base: one row per await above, each using the fakes' wedge hooks. Each row asserts that the operation holding the lock settles typed within the deadline, and that the stem lock stays held while the wedged operation is unsettled. Add a `move` hook to the fakes if needed.
- `node scripts/check-stem-store-v1.mjs` and `node scripts/check-stem-store-v1.mjs --self-test --budgets` are green, with a ledger entry per deadline.
- The suite runs in Node against the fakes, so the two-browser gate does not apply.

## Evidence / attempt record
(empty)
