# Web stem store: a lease read's lookup of its final has no deadline

Successor of #2397. #2397 deadlined every OPFS step awaited inside a hold of a stem lock. It recorded this lookup as a follow-up, because `read()` holds no stem lock.

## Problem
In `hosts/host-web/web/stem-store/opfs-store.js`, `OpfsStemStore.read()` (the method behind `StemSessionLease.read()`) awaits `this.#directory.getFileHandle(stemFileName(identity))` with no deadline. That is the line before its deadlined `handle.getFile()`.

If that lookup never settles, the lease read never settles either. The Worker pump that called it then waits forever on that stem. No error reaches the session, so it cannot recover or report the stem. Every neighbouring step has the local-store deadline (`readDeadlineMs`):
- the index read under the index lock (#2381);
- the `getFile()` that follows;
- the same lookup inside verify-on-open and `verify()` (#2397).

`read()` holds no stem lock and no index lock while it does the lookup. A wedge there therefore blocks no other caller, and nothing else is affected.

## Contract
- `read()`'s lookup has the local-store deadline. On expiry it fails with `storage.read_stalled`, the code its `getFile()` uses.
- A stalled lookup is a stalled read under #2381's one stall rule. It goes to `read()`'s existing failure path (`#demoteUnreadable` with the lease's own pin):
  - the stem is demoted only when no other live session pins it;
  - the read fails typed `storage.read_stalled`, as a stalled `getFile()` does today.
- A `NotFoundError` from the lookup is still damage, and still demotes unconditionally.
- A lookup changes nothing on disk, so it is not fenced and takes no lock.

## Non-goals
- No change to the `.d.ts`, the stem-lock or index-lock rules, the render path or Rust.
- No lock for `read()`. It stays lock-free, as the module documents.

## Gates
- Red on base: one row using the fakes' `getFileHandleBlocker` (lookup with `create: false`, on the final's name). It asserts that `lease.read()` settles typed within the deadline. It also asserts that another session's pin keeps the row and the file, and that the other session still reads once the wedge is lifted.
- `node scripts/check-stem-store-v1.mjs` and `node scripts/check-stem-store-v1.mjs --self-test --budgets` are green, with a ledger entry for the deadline.
- The suite runs in Node against the fakes, so the two-browser gate does not apply.

## Evidence / attempt record
(empty)
