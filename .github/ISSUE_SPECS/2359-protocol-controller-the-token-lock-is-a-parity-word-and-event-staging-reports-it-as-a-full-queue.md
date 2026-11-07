# Protocol controller: the token lock is a parity word, and event staging reports it as a full queue

Tracker: #2258, item PR-2 ("Token lock is odd/even parity on `structural_generation`"). Found by
the implicit-state-machine audit of 2026-11-06. This issue also absorbs the earlier follow-up to
narrow `queues_mut()`.

## Problem
`ProtocolController` serializes its affine tokens (`PreparedStructuralCommand`,
`PreparedSessionCommand`, `PreparedObservationRead`) with one shared word,
`structural_generation: Arc<AtomicU64>` (base `8e76a1ae3`, `crates/protocol/src/controller.rs:1800`):
an odd value means a token is outstanding. Nothing names that phase.

- **The parity is hand-written at every site.** Seven `& 1 != 0` checks (`process` `:1918`,
  `process_command_frame_into` `:2095`, the four `prepare_command_frame*` entries `:2141`, `:2176`,
  `:2210`, `:2243`, `structural_outstanding` `:5295`) and three copies of the acquire
  (`checked_add(1).filter(odd)` then a CAS, `:2412`, `:2481`, `:2888`).
- **The release is copied three times.** Each token repeats `owner`, `generation`, `armed: bool`
  and a `Drop` CAS (`:1354`/`:1365`, `:1432`/`:1449`, `:1529`/`:1578`); completion clears `armed`
  by hand after a separate `store` (`:2758`, `:2792`, `:2816`, `:3487`). Because the tokens
  implement `Drop`, their fields cannot be moved out, so the structural token keeps its
  `response` as an `Option` reached through `.expect` (`:3419`).
- **`queues_mut()` and `set_provider_features()` guard the phase with a runtime `assert!`**
  (`:3915`, `:3947`): a host that calls either while a token is outstanding panics.
- **Misreport (the observable defect).** The four event-staging entry points refuse while a
  token is outstanding, but report it as `EventEgressError::ReliableQueueFull(report)`:
  `stage_meter_batch_event` (`:3976`), `stage_counter_snapshot_event` (`:4026`),
  `enqueue_diagnostic_event` (`:4070`), `enqueue_response_ready_event` (`:4132`). The meter and
  counter paths are lossy telemetry that never touch the reliable queue, and a session-command or
  observation-read token holds no reliable slot at all, so the attached report shows free room. The
  command entry points report the same phase as
  `CommandFrameProcessError::PreparedCommandOutstanding` (or `BACKPRESSURE` on `process`). A host
  reading the egress error is told to drain a queue that is not full, rather than to finish or drop
  its token.

No entry point was found that skips the check: the `dequeue_*` egress calls only empty queues and
are deliberately allowed while a token is outstanding; `process_session_transaction_btlv`
delegates to `process`.

## Decision
- `controller::lease::TokenLock` owns the word and the parity, privately: `is_idle()`,
  `acquire() -> Result<TokenLease, LeaseRefusal>`, `issued(&TokenLease)`, `is_current(&TokenLease)`.
  No other code reads or writes the word.
- `TokenLease` is the one RAII release: not `Clone`, constructible only by `acquire`, released by a
  compare-and-swap from its own odd generation to the next even one on drop. Completion is the
  same release (the commit drops the lease after its last write), so the separate `store` and
  every `armed` flag are gone. `acquire` refuses a generation whose release could overflow
  (unreachable: `2^63` tokens), where the base saturated and left the lock odd forever.
- The three tokens hold a `TokenLease` and no longer implement `Drop`, so completion destructures
  them; the structural token's `response` is no longer an `Option`.
- `ProtocolController::idle() -> Option<IdleController<'_, P>>`: the borrowed view of a controller
  with no token outstanding. `queues_mut()` and `set_provider_features()` move onto it and lose
  their runtime asserts; while the view lives, no token can be acquired (it holds `&mut`).
- `EventEgressError::PreparedCommandOutstanding`, returned by the four staging entry points, the
  same name the command path uses. `control-plane` maps it to `EventError::Backpressure`, as it
  mapped the old report, so no host-visible status changes.
- A loom model, `token_lease_loom`, over the real `TokenLock`/`TokenLease` (loom's atomic under
  `cfg(all(loom, test))`, so a dependent's loom build still uses `std`'s), with its CI leg.

Allocation: the lease is an `Arc` clone and a `u64`, as the base token fields were; nothing is
added on any path, and no render code is touched.

## Non-goals
Routing every controller entry point through `IdleController` (the tracker's full model): it
changes the public surface every host calls, and is a follow-up. `PreparedStructuralCommand`'s
`prospective_session: Option` stays: `materialize_prepared` takes it by `&mut` and replaces it.
PR-1's commit ordering.

## Gates
- Regression test red on base: a session-command token outstanding, all four staging entry
  points must refuse with `PreparedCommandOutstanding`, and accept after the token is dropped.
- Mutation: reverting any one staging site to `ReliableQueueFull` turns it red; removing the
  lease's drop release turns the existing token tests red.
- `token_lease_loom`: a lease dropped on another thread while the controller thread acquires;
  never two holders, and the lock is idle after both finish.
- `cargo test --locked -p protocol`, `-p control-plane`, `-p audit`; clippy `-D warnings`;
  `cargo fmt --check`; rustdoc `-D warnings`; `scripts/check-workspace-policy.sh`.

## Evidence / attempt record
### Attempt 1 (Terra), on base `8e76a1ae3`
**Red on base.** `a_held_token_refuses_event_staging_as_outstanding_not_as_a_full_queue`
(`crates/protocol/src/controller/tests.rs`), run on the base with only the new variant added so
the test compiles:

```
assertion `left == right` failed
  left: Err(ReliableQueueFull(QueueReport { kind: ReliableEvent, capacity: 4, occupancy: 0, requested_slots: 1, generation: QueueGeneration(4) }))
 right: Err(PreparedCommandOutstanding)
```

The base's report says the queue is empty (`occupancy: 0` of 4) while calling it full.

**What the tests defend.**
- `a_held_token_refuses_event_staging_as_outstanding_not_as_a_full_queue`: red if any of the four
  staging entry points reports an outstanding token as anything but `PreparedCommandOutstanding`,
  or the lock outlives the dropped token. It replaces the two rows that pinned the old report
  (the diagnostic row in `structural_token_cancel_owner_generation_and_serial_rules_are_affine`,
  and the faked odd-generation block in
  `a_response_ready_event_is_held_until_delivered_and_holds_off_a_strip_request`), both deleted.
  The `catch_unwind` row around `queues_mut()` is now `assert!(owner.idle().is_none())`.
- `an_exhausted_lock_refuses_rather_than_wedging` (`controller/lease.rs`): red if `acquire` takes
  a generation whose release overflows. The base saturated that release and left the lock odd
  forever (unreachable: `2^63` tokens).
- `token_lease_loom::a_lease_dropped_on_another_thread_releases_exactly_once`: red if two leases
  are held at once, a drop on another thread fails to release, or a foreign lease's drop releases
  this lock.

**Mutations** (each reverted after the run):

| Mutation | Result |
|---|---|
| meter staging site back to `ReliableQueueFull(report)` | staging test red (`tests.rs:5430`) |
| counter staging site back | red (`:5434`) |
| diagnostic site back | red (`:5438`) |
| response-ready site back | red (`:5445`) |
| `acquire` skips its outstanding check | loom red (two holders, `lease.rs:163`) |
| `TokenLease::drop` releases nothing | loom red (`lease.rs:172`), and 11 protocol lib tests red, the staging test and the exhaustion test among them |

**No bit moved / allocation.** No render code is touched. The lease is the base token's `Arc`
clone and `u64`; the completion path's `store(Release)` became the lease's
`compare_exchange(AcqRel)`, which succeeds exactly where the store ran (the generation was just
checked current under `&mut self`).

**Gates run** (`CARGO_TARGET_DIR` private to this worktree):
- `cargo test --locked -p protocol`: all pass (lib 192).
- `cargo test --locked -p control-plane --features test-support`: all pass.
- `cargo test --locked -p audit`: 27 pass.
- loom leg (as CI runs it): `RUSTFLAGS='--cfg loom --check-cfg=cfg(loom) -C target-feature=+avx2,+fma' cargo test --locked -p protocol --lib token_lease_loom`: pass. New step in `qualification.yml`'s `test-release` job; the job timeout goes from 34 to 37 minutes.
- `cargo check --locked --workspace --all-targets --all-features`: clean.
- `cargo clippy --locked -p protocol -p control-plane -p audit --all-targets --all-features -- -D warnings`: clean. (Without `--all-features`, `control-plane`'s lib test reports the existing unused `compile_children_warm`, `compile.rs:2729`, untouched here; CI runs with `--all-features`.)
- `cargo fmt --all --check`, `RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps -p protocol -p control-plane -p audit`, `bash scripts/check-workspace-policy.sh`: clean.

**Verdict on attempt 1 (Sol): PASS**, with four MINOR findings. The memory ordering, the control-plane retry, the error mapping and the CI step were all judged correct. The findings were folded in below, after a merge of `origin/main` at `5c71b3e8d`.

### Attempt 1 folds
1. **Lease order on drop.** `lease` was the first field of each token, so a dropped structural token released the lock before its `session_committed` and `observation_ended.room` reservations were returned. Since tokens are `Send`, a drop on another thread could let `idle()` succeed while a reservation was still counted, which contradicted the `IdleController::queues_mut` doc. `lease` is now the last field in all three tokens, with a comment saying the order is load-bearing: fields drop in declaration order.
2. **The loom model now guards data.** The holder thread writes a `Relaxed` word, the stand-in for the reservation counts, before it drops its lease. The next acquirer reads that word before any other load of the lock word. Loom lets a `Relaxed` load return any value the happens-before order allows, so only the lease's orderings make the write visible. The crate denies `unsafe`, so there is no `UnsafeCell`.

   | Mutation of the lease's orderings | Model |
   |---|---|
   | drop CAS weakened to `Relaxed` | red (`lease.rs:171`, read 0 not 1) |
   | `acquire`'s load and CAS weakened to `Relaxed` | red (same row) |
   | every ordering in `lease.rs` set to `Relaxed` | red |

   Under the attempt-1 model, all three of these stayed green.
3. **The `is_current` row.** In `a_session_command_completes_once_and_replays_its_exact_bytes`, completing controller A's session token on controller B now asserts:
   - B refuses with `Internal`, and B's replay cache stays empty.
   - A is locked while the token is outstanding, and the refused completion releases it, as a drop does.

   Mutation: with `complete_planned`'s `is_current` check replaced by `if false`, this test goes red. 191 others stay green, as all 192 did before this row.
4. **Restored assertion.** `assert!(!endpoint.response_ready_pending())` is back after the refused response-ready call in the staging test.

**Gates re-run after the folds:**
- `cargo test --locked -p protocol` and `-p control-plane --features test-support`: pass.
- The loom leg with CI's flags: pass.
- Clippy `--all-targets --all-features -D warnings` on protocol, control-plane and audit: clean.
- `cargo fmt --all --check`, rustdoc `-D warnings` and `check-workspace-policy.sh`: clean.

**Follow-ups (not in this issue).**
- Route the remaining controller entry points (`process`, `process_command_frame_into`, the
  `prepare_command_frame*` family, the four staging calls) through `IdleController`, so that their
  refusal is the type rather than an `is_idle()` check at each entry. It changes the public surface
  every host calls.
- `PreparedStructuralCommand::prospective_session` stays an `Option` because
  `materialize_prepared` replaces it through `&mut`; on a materialize error the token is left
  without one, and any later `prospective_session()` call panics. A by-value
  `materialize_prepared` would remove the `Option`.
