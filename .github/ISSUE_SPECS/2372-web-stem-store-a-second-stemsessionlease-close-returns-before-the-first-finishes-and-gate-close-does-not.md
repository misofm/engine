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
(empty)
