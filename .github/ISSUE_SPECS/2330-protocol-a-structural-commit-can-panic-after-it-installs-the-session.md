# Protocol: a structural commit can panic after it installs the session and queues SESSION_COMMITTED

Tracker #2258, item PR-1. Pattern: checks that run after the mutation they guard.

## Mission
`ProtocolController::commit_prepared_structural` must run every check before its first write, so a
panic in it, in a build that unwinds, leaves the controller exactly as it was. Two things depend
on that. The control plane's #2010/#2044 republish rule assumes a panic in the protocol commit
committed nothing. The C ABI catches panics. Each invariant the commit relied on should be a
proof taken at preparation or attachment, not an `.expect` after the write.

## Verified defect (base `10ce797ca`)
`commit_prepared_structural` (`crates/protocol/src/controller.rs` 3325-3442) runs in this order:
1. `session.commit_prepared_displacing` installs the session.
2. `commit_reserved_reliable_event` queues `SESSION_COMMITTED`.
3. `next_reliable_event_sequence` advances.
4. For an attached `OBSERVATION_ENDED`, the step does three things: an `.expect` on the paired
   `ended_reservations` `Option`; a search for a free cell,
   `.position(Option::is_none).expect("a reserved reliable slot has a free observation-ended
   cell")` (3380-3385); and `u32::try_from(cell).expect`.
5. It re-encodes the final frame with an `.expect` (3417-3421), then checks
   `assert_eq!(written, bytes.len())`.
6. It writes the replay entry with `.complete(...).expect` (3430-3434).

Every check after step 1 runs after the session is installed. Each one was re-verified:

| Check after the write | Can it fire? |
|---|---|
| Free observation-ended cell (step 4) | **Yes, through the public API.** There are `reliable_event_slots + 1` cells. Only the controller's egress (`dequeue_reliable_event_frame_into`) frees one. An `OBSERVATION_ENDED` slot popped through the public `queues_mut().try_dequeue_event()` leaks its cell. Once every cell is taken, `attach_observation_ended` still returns `Ok`, because it checks only queue room. The commit then panics at 3385 with revision `n+1` installed and `SESSION_COMMITTED` queued. No product host pops through `queues_mut()`. Through the controller's egress, the cells cannot run out: at least 2 slots are held, so occupied cells ≤ queued ends + 1 pending ≤ capacity − 1. |
| `ended_reservations` `.expect` | No. It is set and cleared together with `observation_ended`. It is still two `Option`s for one fact. |
| `commit_reserved_reliable_event`'s `remaining != 0` assert | No. The plan computes the reservation count from the edits (`usize::from(!monitor_only)`, 3005-3010). The commit decides from the form (`session.is_monitor_only()`, #2286). The store computes the form the same way (`model.rs` 1602). The two always agree, but by duplication: if the plan's count were 0 and the form `Compiled`, the assert would fire after step 1. |
| Re-encode `.expect` and `assert_eq!` | No. `EditPath` is a one-byte code (`message_wire.rs` 2850), and the frame is otherwise the one measured at preparation. |
| `replay.complete().expect` | No. The replay cache is written only by entry points that refuse while the generation is odd, so no write lands between the plan and the commit. `ReplayCache::complete` runs all its checks, then writes (#2254). |
| `u32::try_from(cell).expect` | No. The cell count is the queue length + 1. |
| `debug_assert_eq!(commit.applied_operations, …)` | No. Both are `edits.len()`. It is still a dev-build panic after the write. |

`process_structural_plan_into` (the one-call path, 3119-3167) already runs its encode and both
`.expect`s before its write. The one check left after the write is the `remaining != 0` assert of
the duplicated count. That path is always `Preparation::Whole`, so it never sees a monitor-only
transaction.

Release builds use `panic = "abort"` (`Cargo.toml` 140), so the half-committed state exists only
in builds that unwind: dev/test builds, and the C ABI's `catch_unwind`. There the control plane's
`rebuild_withdrawn` (`crates/control-plane/src/control.rs` 5670-5702) republishes the candidate it
withdrew, for a commit that happened.

## Decision / model
- **Order.** The commit runs every check first: the token's parts, the frame re-encode, its length
  and the cell's emptiness (`debug_assert`). Its first write is the replay entry. `complete`
  either changes nothing or completes, so its `.expect` also fires with nothing written. Only then
  do the session, `SESSION_COMMITTED` and the attached event follow, and none of them can fail.
  The `applied_operations` `debug_assert` moves ahead of the writes, through a crate-private
  `PreparedSessionTransaction::applied_operations()`.
- **The `SESSION_COMMITTED` slot comes from the form.** The token holds
  `session_committed: Option<ReliableEventReservation>` (a single hold).
  `reserve_session_committed(&PreparedSessionTransaction)` fills it from `is_monitor_only()` in
  both paths: the token's preparation, and the one-call path. The commit reads the same form and
  matches `(monitor_only, slot)`. The impossible pairs are an `unreachable!` placed before any
  write. `commit_reliable_event` consumes the hold, so the multi-hold `remaining` count is gone
  from both paths. The plan's predicted count stays only for the cheap room check before
  preparation, which keeps the refusal order.
- **The cell is proven at attachment.** `observation_ended` and `ended_reservations` become one
  `Option<EndedAttachment { event, room: ReliableEventReservation, cell: u32 }>`.
  `attach_observation_ended` finds the free cell before it reserves the room. If there is none, it
  refuses with the new `AttachObservationEndedError::NoFreeCell` (a defect; nothing held). The
  cell stays free until the commit: only a structural commit fills a cell, no other commit runs
  while the token's generation is odd, and a delivery only empties a cell. The control plane maps
  the new variant through its existing `Err(_) => CommandError::Internal`.
- The control plane's #2044 doc (`rebuild_superseding`) now cites the protocol commit's guarantee.

## Non-goals
- PR-2's token lease (the odd/even generation, `armed`) and the `.take().expect` pattern on the
  token's `Option` fields. Those `.expect`s run before any write and stay.
- Removing the now-unused `ReliableEventReservations` multi-hold API from `queue.rs`. It is a
  follow-up.
- `materialize_prepared` leaving the token without a session on a store error. It is a follow-up.
- No render path is touched, and there is no new cross-thread state.

## Gates
- A regression test red on base, green at head, in `protocol`, which owns the commit.
- Mutation tests of the fix.
- `cargo test --locked -p protocol`; `cargo test --locked -p control-plane --features test-support`;
  `-p capi` (it commits tokens and catches panics).
- `cargo clippy --locked -p protocol -p control-plane --all-targets -- -D warnings`,
  `cargo fmt --check`, `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p protocol -p control-plane`,
  `bash scripts/check-workspace-policy.sh`.

## Evidence (attempt 1)
**Red on base.** The regression test is
`controller::tests::a_commit_without_a_free_end_cell_is_refused_before_it_writes`
(`crates/protocol/src/controller/tests.rs`). The setup:
- A two-slot queue, so three cells.
- Three commits, each with one attached key. The test pops both events of each commit through
  `queues_mut().try_dequeue_event()`, which leaks the end event's cell.
- A fourth token, with one key attached.

If `attach_observation_ended` refuses, the test drops the token. If it succeeds, the test commits
under `catch_unwind`. Either way it then asserts:
- the revision is still 10;
- the reliable-event admission occupancy is 0;
- the replay cache still treats request 4 as new;
- the refusal is `NoFreeCell`.

On base `10ce797ca` the test ran with the last assertion written as a `Debug`-string check, because
the variant did not exist yet:
```
panicked at crates/protocol/src/controller.rs:3385:22:
a reserved reliable slot has a free observation-ended cell

panicked at crates/protocol/src/controller/tests.rs:1530:5:
assertion `left == right` failed: a commit that cannot finish installs nothing
  left: SessionRevision(11)
 right: SessionRevision(10)
```
So the commit panicked with revision 11 installed and `SESSION_COMMITTED` queued.

**Mutations (each reverted).**
- `attach_observation_ended` takes cell 0 instead of refusing (`.ok_or(NoFreeCell)?` changed to
  `.unwrap_or(0)`). The new test is red: `the attachment is refused: None`. The commit's
  pre-write `debug_assert` on the cell fired, and the test's revision, occupancy and replay
  assertions still passed. That shows the check now runs before the first write.
- `reserve_session_committed` reserves a slot for a monitor-only form too. The existing
  `a_transaction_of_monitor_writes_commits_no_model_revision_or_event` is red: the commit's
  pre-write `unreachable!` fires.
- The base order is the first mutation of the cell check: the cell is looked for after the session
  commit. That is the red-on-base run above.
- No test covers moving the re-encode or `replay.complete` back after the session write, because
  neither can fail (see the table above). This is a proof, not a test.

**Test value.** The test is red if the commit finds its observation-ended cell, or makes any
other check, only after it has installed the session and queued `SESSION_COMMITTED`. In a build
that unwinds, the result is a committed revision behind a panic, and the control plane republishes
a withdrawn candidate for it. No existing test reaches the cell-exhausted state.
`the_end_needs_room_with_the_commit_and_a_dropped_token_releases_it` covers queue room only, which
a leaked cell does not consume.

**No-bit-moved / render.** No render path is touched. This is control-side commit ordering only.

**Gates run (head of this branch).**
- `cargo test --locked --all-targets -p protocol -p control-plane -p capi -p host-web -p conformance --features protocol/test-support,control-plane/test-support,host-web/test-support,host-core/test-support`:
  769 passed, 0 failed (64 test binaries).
- `cargo test --locked --doc -p protocol -p control-plane --features …`: ok.
- `cargo clippy --locked -p protocol -p control-plane --all-targets --all-features -- -D warnings`:
  clean. Without `--all-features`, `control-plane` lib-test reports `compile_children_warm` as dead
  code on base too, so that is not this change; CI runs `--all-features`.
- `cargo fmt --check`, `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p protocol -p control-plane`,
  and `bash scripts/check-workspace-policy.sh` (`workspace policy: ok`).
- The realtime policy tool was not run, because no render code was touched.

**Follow-ups (not filed).**
- `ReliableEventReservations` (the multi-hold), `reserve_reliable_events`,
  `commit_reserved_reliable_event` and `release_reliable_events` in `crates/protocol/src/queue.rs`
  have no caller left outside their own #2254 test. They could be removed, and with them the
  `remaining` count that #2254 had to guard.
- `materialize_prepared` (`controller.rs`) takes the token's session and, on a store error, returns
  without putting it back. The token is then live with no session, and its
  `prospective_session()` accessor panics. The doc says that error cannot happen; the token should
  keep its session either way.

## Verdict (attempt 1)
**PASS** (Sol's adversarial review). The reviewer confirmed:
- The commit is atomic for every input: each check, the replay step's own included, runs before
  the first write.
- Every write after the replay entry is infallible.
- Reliable-event accounting and the refusal order are unchanged.
- No exhaustive match is affected by `AttachObservationEndedError::NoFreeCell`.

The follow-ups are tracked in #2258, not here:
- The dead multi-hold API in `queue.rs`, and `materialize_prepared` leaving a token without its
  session.
- Narrowing `queues_mut()`, under PR-2.
