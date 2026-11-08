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

### Attempt 1 (Terra), branch `codex/issue-2403-monitor-candidate-borrow`, base `3375b619b`

**Claim re-verified on the base.** It holds: `crates/protocol/src/model.rs:1643` on base clones the
committed model for every whole-path preparation, monitor-only included.

**The precondition, proved from the code** (line numbers at the head). A transaction is monitor-only
when `monitor && is_monitor_transaction(edits)` (`model.rs:1632`): every edit is an `Observe` or a
`SetControl` whose row scope is `Monitor` (`model.rs:1976`, `:1983`, through
`controls::row_scope`, `crates/controls/src/registry.rs:322`).
- `Observe`: it reads the candidate (`observe_target`, `model.rs:1709`), writes only the copy of the
  binding table and either returns a refusal or takes `continue` (`model.rs:1723`).
- A monitor-row `SetControl` (only `monitor` is true here, so its branch is taken): `check_expected_effect`
  and `check_monitor` read the candidate. **`check_monitor` cannot return `Ok(None)` for it.** The only
  `Monitor`-scope rows are a strip's own rows `SOLO` (2) and `MASTER` (4)
  (`crates/controls/src/strip.rs:107`, `:152`; every other descriptor takes `BASE`, scope `Model`,
  `crates/controls/src/descriptor.rs:251`, and builtin rows are `Model`, `registry.rs:325`).
  `check_monitor` resolves the address with `strip::resolve`, which looks the row up in the same
  `strip::ROWS` table by the same ID (`strip.rs:416`; the IDs 1, 2, 3, 4, 64 and 65 are distinct). So it
  either finds that very row, whose scope is `Monitor` (so the `Ok(None)` branch at `registry.rs:564`
  is not taken), or refuses: an unknown strip, and a submix's solo (`on_submix: false`), are
  `AddressUnknown`. Its source is `StripValue::Monitor`, so `registry.rs:571` passes. The edit is
  then pushed to the monitor-edit vector and takes `continue` (`model.rs:1749`), or it is refused.
- So no edit of a monitor-only transaction reaches `apply_session_edit` (`model.rs:1752`). The
  `SetTrackConsole` guard (`:1677`) only reads, and a monitor-only transaction holds no
  `SetTrackConsole`. The replace's smoothing write (`:1763`) is unreachable: `prepare_replacement`
  passes `monitor: false`, so it is never monitor-only. The compile and its revision write (`:1775`)
  are skipped for a monitor-only transaction.

No fallback clone is needed.

**Change** (`crates/protocol/src/model.rs` only). The candidate is a `Cow<SessionModel>`:
`Borrowed(committed)` for a monitor-only transaction, `Owned(committed.clone())`, made at the same
place as before, for every other. The three writes go through `to_mut()`; for an owned candidate that
is the plain write it was. A `debug_assert!` before the compile states the precondition: a monitor-only
candidate is still borrowed. If the proof ever stopped holding, a release build would clone at the
write (`to_mut`), and its result would be the base's, which also cloned, wrote and then dropped the
candidate for a monitor-only transaction. The allocation gate turns red in that case.

**Allocation gate, red on base.** `crates/protocol/tests/solo_allocation.rs` is tightened, not
duplicated. The test is renamed `a_whole_path_solo_preparation_clones_no_model_or_compilation`, and
its ceiling is now `prepare < one model clone`. The old ceiling was `model clone + compilation clone`,
and the new one implies it, so #2394's claim is still defended. With the head's test on the base's
`model.rs`:
```
a whole-path solo requested 1022 bytes: at least a clone of the committed model (857)
test a_whole_path_solo_preparation_clones_no_model_or_compilation ... FAILED
```
**Exact difference against base.** On the canonical fixture, a whole-path solo
`prepare_transaction_with_monitor` requests 1022 B on base and 165 B on head. The difference, 857 B,
is exactly one `store.compiled().normalized_model().clone()`. The 165 B left are the monitor-edit
vector and the edit bookkeeping.

Test value: the test turns red if a monitor-only preparation clones the committed model (or the
compilation) again, even if only for a moment. No other test measures it. No test was added.

**Mutations** (each applied, run and reverted):

| # | mutation | observed |
|---|---|---|
| M1 | `Cow::Borrowed(committed)` becomes `Cow::Owned(committed.clone())` (the clone reintroduced) | RED: the `debug_assert` panics in `solo_allocation` |
| M1b | M1 with the `debug_assert` disabled | RED: `solo_allocation`, "requested 1022 bytes: at least a clone of the committed model (857)" |
| M2 | an `Observe` edit does `candidate.to_mut()` before its `continue` (a monitor edit that writes) | RED: 15 `control-plane` observation tests (`--features test-support`), for example `an_observe_transaction_arms_shares_and_disarms_the_tap`, at the `debug_assert` |

**No behaviour moved (a one-time comparison, not committed).** Two temporary probes ran on base and on head.
- Store level, in `model.rs` tests, on `fixtures/session/v1/irregular-graph.json` (24 tracks, 5
  submixes, console and insert taps). It ran 125 transactions: solo, master and solo-off; an unknown
  strip; a submix's solo; a non-toggle value; NaN; lane `left`; an expected effect; ramps of 48000 and
  48001; repeated rows; and solo mixed with model edits in either order. It also ran `Observe` arms and
  disarms on insert and console taps, tap IDs 0 to 3, windows 0, 4 and 100000, an ended-arm disarm,
  an unknown section, and `Observe` mixed with solo and with a model edit. Each ran with 4 observe
  contexts (none, empty, a held table, disabled), whole and in place. It printed every
  acceptance and refusal (with the operation index and the error), `is_monitor_only`, `is_in_place`,
  the revision, the applied operations, the lent compilation's revision, `monitor_edits`,
  `set_control_rows`, `observe_bindings`, `observe_writes` and `next_arm_tag_after`, then the same
  after `materialize`, and one commit. The output was 1000 lines, byte-identical on base and head
  (sha256 `9fc2ea9f8688f0136f7e12086e14c55158548af614090602020af3ee54f3688e`), with 118 accepted monitor-only preparations and 852 refusals.
- Controller level, in `controller/tests.rs`, on the canonical fixture: 11 transactions, each committed
  with `EditPath::Live` and with `ModelOnly`. It printed the prepare-time refusal bytes, or, for a
  token, its form, revision, monitor edits and rows, the response bytes, the exact-replay bytes, the
  reliable-event queue report, the revision after the commit and the snapshot length. The 21 lines were
  byte-identical (sha256 `a3f3df5f89cd7c4a8cfe36dc42d93c6efa49668188d085ef97cac3e0cd84ba32`).
- The two probes' bodies are in the issue's evidence comment (#2403), so the comparison can be
  reproduced. Paste them at the end of the `model.rs` tests module and of `controller/tests.rs`, and run
  each with `PROBE_OUT=<file> cargo test -p protocol --lib probe_2403`.

No render code changed.

**Gates (head).**
- `cargo test --locked`: `protocol` (`--all-targets --features test-support`), `control-plane`
  (`--features test-support,protocol/test-support`) and `host-core` (`--features
  test-support,control-provider`): all green.
- `cargo clippy --locked -p protocol -p control-plane -p host-core --all-targets --all-features -- -D warnings`: clean.
- `cargo fmt --check`: clean. `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps` for the three crates: clean.
- `scripts/check-workspace-policy.sh`: ok. No render code changed, so the realtime policy tool does not apply.

**Dropped claims.** None.

**Follow-ups.** None.

### Sol verdict on attempt 1 (`9a60edb37`): PASS, minor findings only

Folded on the same branch, after merging origin/main:
1. **The precondition check holds in release builds.** The `debug_assert!` before the compile is
   now a release-mode check. When a monitor-only transaction's candidate is `Cow::Owned`, the
   preparation returns the new typed internal error `SessionStoreError::MonitorCandidateWritten`. Such
   an edit is never accepted and then silently dropped. The controller answers it `Internal` with no
   diagnostics, as it does for `ArmTagsExhausted`. The check costs one discriminant test.
   - Mutation: an injected `let _ = candidate.to_mut();` in the monitor `SetControl` branch, before
     its `continue`, makes every solo preparation return the error. Six `protocol` tests go RED,
     including `a_solo_and_a_master_of_one_strip_are_two_monitor_writes_on_both_paths` ("prepares:
     MonitorCandidateWritten") and `a_transaction_of_monitor_writes_commits_no_model_revision_or_event`
     (no token).
2. **The `Observe` case is named.** The `solo_allocation` doc comment now says that an
   `Observe`-only transaction takes the same borrowed-candidate decision. The test measures the solo.
3. **The probe evidence can be reproduced.** The full sha256 values are above, and the probe bodies are
   in the issue comment. Both probes were rerun on the folded tree and were byte-identical to the base's
   (the same two digests).

Gates on the merged, folded tree (`082522049`), all green or clean:
- `cargo test --locked -j 4` for `protocol` (`--all-targets --features test-support`) and
  `control-plane` (`--features test-support,protocol/test-support`).
- `cargo clippy --locked -p protocol -p control-plane -p host-core -p capi -p host-web --all-targets
  --all-features -- -D warnings`.
- `cargo fmt --check`, `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p protocol`, and
  `scripts/check-workspace-policy.sh`.
