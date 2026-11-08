# Protocol: a monitor-only preparation clones the committed model as a candidate that none of its edits write

Successor of #2394, which removed the `CompiledSession` clone from a solo token and recorded this
remaining clone as a follow-up. Low priority: for the weekly performance pass.

## Problem
`SessionStore::prepare_transaction_with` (`crates/protocol/src/model.rs`, the `let mut candidate =
self.compiled.normalized_model().clone();` line, ~1643 at #2394's head) clones the committed model as
its candidate for every transaction prepared whole. That includes a transaction of monitor edits
only (`monitor_only`: solo writes and `Observe` edits).

For such a transaction, the candidate is only read:
- an `Observe` edit checks its target against it (`controls::observe_target`) and writes only to the
  copy of the binding table;
- a monitor-row `SetControl` is checked against it (`controls::check_expected_effect`,
  `controls::check_monitor`) and is written to the monitor-edit vector, then the loop takes `continue`;
- the `SetTrackConsole` guard compares its console with the committed one;
- the compile is skipped, and the ramp bound and the observe tagging read the committed compilation
  (#2394).

So the candidate is the committed model, unchanged, for the whole preparation. It is freed when the
preparation returns (the token does not hold it). On the canonical fixture it is 857 B of the 1022 B
that a whole-path solo preparation requests (#2394's evidence), and it grows with the session: a
session-sized allocation per solo or subscription transaction.

## Contract
- **No candidate clone for a monitor-only transaction.** When `monitor_only`, the edits are checked
  against the committed model by borrow (for example a `Cow<SessionModel>`, or a candidate made at
  the first edit that writes the model, which a monitor-only transaction never reaches).
- **First, prove the precondition.** Show from the code that no edit of a `monitor_only` transaction
  reaches `apply_session_edit` or a write to `candidate`. Pay particular attention to a monitor-row
  `SetControl` for which `check_monitor` returns `Ok(None)`. If one can reach it, the borrow must
  fall back to a clone at that point, and the spec records why.
- **Every observable answer is the base's:** each acceptance, each refusal with its operation index
  and error, `monitor_edits`, `set_control_rows`, `observe_bindings`, `observe_writes`, the arm tags,
  and the token's form and revision.
- Every transaction that is not monitor-only keeps its candidate clone exactly as it is now.

## Non-goals
- The in-place path (`prepare_in_place`), which already reads the committed model by borrow.
- Any change to the store's committed state, the controller, the wire or the C ABI.

## Gates
- **Allocation, red on base.** Measure a whole-path solo `prepare_transaction_with_monitor` with
  `bench_support::alloc` thread-scoped counters (as `crates/protocol/tests/solo_allocation.rs` does).
  The requested bytes fall by at least the size of one committed-model clone, an exact difference
  against base recorded as evidence. Tighten the existing `solo_allocation` ceiling, or replace it,
  rather than adding a second test of the same claim.
- **Mutation evidence.** Reintroduce the clone for the monitor-only path and observe red.
- **Nothing else changes.** The existing monitor, observe and solo tests in `protocol`, `host-core`
  and `control-plane` stay green and unchanged in what they assert.
- `cargo test --locked` for `protocol`, `control-plane` and `host-core`, plus clippy
  `-D warnings`, `cargo fmt --check`, `cargo doc -D warnings` and
  `scripts/check-workspace-policy.sh`.

## Evidence / attempt record
(empty)
