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
- A stalled lookup is a stalled read under #2381's one stall rule. It goes to `read()`'s existing failure path (`#demoteUnreadable` with the reading session's own pin, which `StemSessionLease.read()` passes; the Worker facade passes none, see the attempt 1 fold):
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
### Attempt 1 (Terra, 2026-11-08)
Base `f44d848fd`; branch `codex/issue-2405-lease-read-lookup-deadline`.

**Claims re-verified on base.** Line numbers are base `opfs-store.js`.
- `read()` awaits `this.#directory.getFileHandle(stemFileName(identity))` with no deadline (306). The `getFile()` after it is deadlined `storage.read_stalled` (307-312).
- The index read before it (296) runs under the index lock and has settled by then, so the lookup runs under no lock. `read()` takes no stem lock.
- Both steps sit in one `try`, whose `catch` (313-322) demotes a `NotFoundError` unconditionally and routes every other failure through `#demoteUnreadable(identity, undefined, options.ownPin, error)`, #2381's one stall rule. So a deadline on the lookup is the only change needed: its stall then takes the existing path.
- No claim is dropped.

**Fix** (`hosts/host-web/web/stem-store/opfs-store.js`). The lookup is wrapped in `withDeadline(…, this.#readDeadlineMs, "storage.read_stalled", "Looking up … made no progress")`, inside the existing `try`. There is no signal, as `getFile()` has none in `read()`. It is not fenced, and it takes no lock: a lookup lands nothing. A `NotFoundError` still demotes unconditionally. No new mechanism, no `.d.ts` change, no change to the lock rules, #2372's pin-phase model, the render path or Rust.

**Tests** (`hosts/host-web/tests/stem-store-core-v1.mjs`). The existing row `aStalledReadKeepsAStemAnotherSessionPins` gains legs; its `getFile` leg is the row as it was, with a one-shot wedge.
- `lookup`: sessions `a` and `b` pin the stem, and the fakes' `getFileHandleBlocker` wedges the lookup (`create: false`) of the final's name. `a.read()` must settle `storage.read_stalled` within 500 ms (deadline 20 ms), the row must keep `b`'s pin and the file must stay. Once the wedge is lifted, `b` reads the whole stem.
- `lookup, unpinned`: only `a` pins the stem. The stalled lookup must fail `storage.read_stalled` and demote the stem.
- Test value: `lookup` is red if a lease read waits on a wedged lookup with no deadline, so the Worker pump hangs on that stem; `lookup, unpinned` is red if the lookup's stall escapes the stall rule (for example, the lookup moved before the `try`), so a stem nobody else plays from stays indexed and unreadable. No existing test wedges `read()`'s lookup.
- No existing test becomes useless.

**Red on base** (base `opfs-store.js`, new test):
```
AssertionError [ERR_ASSERTION]: a lease read waited on a wedged lookup past its deadline
    at aStalledReadKeepsAStemAnotherSessionPins (…/stem-store-core-v1.mjs:827:12)
  actual: 'timed-out'
```

**Mutation ledger** (`scripts/check-stem-store-v1.mjs`, a new #2405 block of 2 entries, each `RED` at its target):
- "look up a lease read's final with no deadline" (the raw await of base), red at `a lease read waited on a wedged lookup past its deadline`;
- "keep a lease read's lookup stall out of the stall rule" (the deadlined lookup moved before the `try`), red at `a stalled read kept a stem no other session pins (lookup, unpinned)`.
- The existing #2381 entry "demote on any read() failure" is still red at its target, `a stalled read demoted a stem another session pins`, now from the `getFile` leg.

**Green** at this commit:
- `node scripts/check-stem-store-v1.mjs`: PASS.
- `node scripts/check-stem-store-v1.mjs --self-test --budgets`: PASS, 135 `RED` lines (133 before, plus 2).
- `stem-store-core-v1.mjs --budgets`: PASS.
- `stem-pump-v1.mjs`: PASS.
- `bash scripts/test-web-audioworklet.sh`: exit 0.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- Not applicable: no Rust and no render path changed, so cargo, the realtime policy tool, no-bit-moved and loom do not apply; the suite runs in Node against the fakes, so the two-browser gate does not apply.

**Docs.** `docs/STEM_STORE_V1.md` states that a session's `read()` holds no lock, that its lookup has the local-store deadline and fails `storage.read_stalled`, that the stall follows the stall rule, and that a missing final still demotes unconditionally.

**Follow-ups.** None.

**Verdict: PASS** (Sol, on `c9df307ca`). Two doc minors were folded, and nothing else changed.
1. The docs said a session's `read()` "holds no lock". That overstates it: `read()` takes the index lock for its index read, and its failure demotion takes locks. The docs now say it takes no stem lock and looks up its final under no lock.
2. The shipped Worker facade (`pcm-pump-worker.js:57`) calls `store.read(identity)` with no `ownPin`. From the Worker pump, the loading gate's own pin therefore always counts as another session's, so a read there that stalls or fails never demotes the stem. This is conservative and predates the branch.
   - The docs now record it.
   - The Contract's "with the lease's own pin" now applies only to `StemSessionLease.read()`.
   - The pump protocol is unchanged.

**Fold** (docs and spec only; no code or tests changed). `origin/main` (`f44d848fd`) is already merged, since it is the base; I reran `node scripts/check-stem-store-v1.mjs`: PASS.
