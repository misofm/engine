# Protocol: a solo token clones the committed compilation to lend it, and the commit decides "mutation or monitor" a second time

Successor of #2378 row C7, which came from #2286. Filed when #2378's bundle reached its bound.

## Problem
Two leftovers from #2286, which made `TransactionForm` the one statement of whether a transaction is monitor-only:

- **The clone.** A transaction of solo writes prepared on the whole path is `TransactionForm::MonitorWhole(CompiledSession)` (`crates/protocol/src/model.rs` ~1769). That compilation is a clone of the committed one, made only so that `PreparedSessionTransaction::compiled()` can lend a compilation. Its readers are:
  - `control-plane`'s non-in-place branch (`crates/control-plane/src/control.rs` ~7132, `expect("a compiled transaction")`);
  - the other `prospective_session().compiled()` readers in `control-plane` (around `control.rs` 4760, 4795, 4818, 5997, 7728 and 11546). The implementer lists each one and says whether it can meet a whole-path solo.

  The controller's `removed_arm_tags` (`crates/protocol/src/controller.rs` ~5359) already filters out monitor-only transactions, so it never reads a solo's compilation.

  Both could read the store's committed compilation, which is the same model at the same revision. Every whole-path solo pays a full `CompiledSession` clone and holds it for the token's life.
- **The second commit decision.** `commit_prepared_structural` re-derives `monitor_only` from the session, then matches `(monitor_only, prepared.session_committed)` (~3369). The two impossible pairs are an `unreachable!`. The slot was reserved from the same form (`reserve_session_committed`, ~3158), so the token carries one fact twice: the form, and whether it holds a `SESSION_COMMITTED` slot.

## Contract
- **No clone for a solo.** `MonitorWhole` holds no `CompiledSession`. Each reader of `compiled()` that can meet a solo reads the store's committed compilation instead: for example, an accessor that takes the store, or a borrow the controller lends. A solo token then allocates no compilation.
- **One commit decision.** The token's commit kind is one typed value, decided once when the token is made, so the commit matches a single enum. An example is `enum TokenCommit { Monitor, Mutation { committed: ReliableEventReservation, ended: Option<EndedAttachment> } }`. The `unreachable!` pairs are not representable. The `attach_observation_ended` refusal of a monitor token (`MonitorOnly`) reads the same value.
- **Every observable answer is the base's:**
  - a solo's response bytes, `EditPath::Monitor`, the revision it reports, its replay entry, and the event it does not emit;
  - every mutation's commit, its `SESSION_COMMITTED`, and its attached `OBSERVATION_ENDED`.
- The commit's no-failure-after-first-write rule (#2330) is kept.

## Non-goals
- The in-place solo form (`MonitorInPlace`), which already holds no compilation.
- `materialize` / `materialize_prepared` (#2378 C9 settled them).
- Any wire or C ABI change.

## Gates
- A solo's preparation clones no `CompiledSession`. Measure this with `bench_support::alloc` thread-scoped counters around a whole-path solo `prepare_transaction_with_monitor`: the head's allocated bytes are below base's by at least the byte size of the `CompiledSession` clone (an exact difference against base), since a solo prepare still allocates its monitor-edit vector. This test is red on base. It defends against a reintroduced clone, which no existing test catches.
- The existing monitor-write tests are green and unchanged in what they assert: `a_transaction_of_monitor_writes_commits_no_model_revision_or_event` and the model's solo commit tests.
- `cargo test --locked` for `protocol`, `control-plane`, `host-core` and `capi`, plus clippy `-D warnings`, `cargo fmt --check`, `cargo doc -D warnings`, `scripts/check-workspace-policy.sh` and the realtime policy tool.

## Evidence / attempt record

### Attempt 1 (Terra), branch `codex/issue-2394-solo-token-no-clone`, base `9806fdce0`

**Claims re-verified on the base.** Both hold.
- The clone: `crates/protocol/src/model.rs:1733` (`self.compiled.clone()` for `monitor_only`), held as
  `TransactionForm::MonitorWhole(compiled)` (`:1769`) and lent by `compiled()` (`:1156`).
- The second decision: `crates/protocol/src/controller.rs:3369` matches `(monitor_only,
  prepared.session_committed.take())`, with the `unreachable!` at `:3373`. The slot comes from
  `reserve_session_committed` (`:3158`), and `attach_observation_ended` re-reads the form (`:3283`).

**Change.**
- `MonitorWhole` holds nothing. `PreparedSessionTransaction::compiled(&self, store: &SessionStore)`
  lends `store.compiled()` for it (attempt 1 only debug-asserted the token's base revision; the
  fold below makes the check hold in release) and the token's own
  compilation for `Compiled`. A new `own_compiled()` returns only a mutation's own compilation (and
  `None` for a solo or in-place token). The preparation reads `compiled.as_ref().unwrap_or(&self.compiled)`
  for the ramp bound and the observe tagging, which is the same model as the old clone.
- `PreparedStructuralCommand` carries one `TokenCommit { Monitor | Mutation { committed, ended } }`
  in place of `session_committed` + `observation_ended`. `reserve_session_committed` returns it,
  decided once from the form. `commit_prepared_structural` chooses the path from it and commits the
  session, `SESSION_COMMITTED` and `OBSERVATION_ENDED` only in `Mutation`. The `unreachable!` pairs
  are gone. `attach_observation_ended` refuses `Monitor` with `MonitorOnly` (an empty list still
  gives `Ok(0)`). `is_monitor_only()` on the token reads it. The one-call path reads it too. The
  no-failure-after-first-write order (#2330) is unchanged: the replay entry is still the first write.

**Every reader of `PreparedSessionTransaction::compiled()`** (the signature change made the
compiler list them):
- `protocol` `removed_arm_tags` (`controller.rs:5359` on base): it filtered solos out, so it never
  met one. It now reads `own_compiled()`, which is `None` for a solo by construction.
- `control-plane` rebuild arm, `control.rs:4760` (rate/quantum), `:4795` (monitor overlay) and `:4818`
  (observation change): **cannot meet a solo.** The arm is reached only through
  `LiveCommit::Rebuild`, whose single return (`:7184`) follows `if monitor_only { return
  LiveCommit::Done(Err(Internal)) }` (`:7181`). These now read `own_compiled()`, keeping their
  existing `.ok_or(Internal)`.
- `control-plane` `Rebuild::Transaction` in the rebuild preparation, `control.rs:5997`: **cannot meet
  a solo**, for the same reason. It reads `own_compiled()`. Under `test-support`,
  `take_test_fault(&mut self)` runs while `prospective` is live, so a store borrow is not possible
  there.
- `control-plane` `commit_live_once` view, `control.rs:7131`: **meets a whole-path solo**: an
  `Observe`-only transaction is prepared whole, and so is a materialized in-place solo. It reads
  `compiled(self.controller.session())`. `owners_change` still reads `(edited_owners, is_in_place)`,
  which is unchanged for `MonitorWhole`, so the ramp-row source is the base's.
- `control-plane` `live_admission`, `control.rs:7728`: **meets a whole-path solo** (same route). It
  reads `compiled(self.controller.session())`, so the resource estimate is the committed one, as before.
- `control.rs:11546` is `SessionStore::compiled` (test code). It is not a reader.
- Tests: `protocol` `model.rs` (materialize test), `controller/tests.rs` (two asserts, unchanged in
  what they assert), `host-core/tests/live_edit_classifier.rs` (3) and `in_place_commit.rs` (1) now
  pass their store. capi and host-web have no reader.

**Red on base** (`cargo test -p protocol --test solo_allocation`, the new test on `9806fdce0`):
```
a whole-path solo requested 5314 bytes: at least its candidate model (857) and a clone of the
committed compilation (4292)
test a_whole_path_solo_preparation_clones_no_compilation ... FAILED
```
**Exact difference against base.** One whole-path solo `prepare_transaction_with_monitor` on the
canonical fixture requests 5314 B on base and 1022 B on head. The difference, 4292 B, is exactly
`store.compiled().clone()`. The remaining 1022 B is the candidate model clone (857 B) plus the monitor
write and the edit bookkeeping. The committed test is a ceiling: `prepare < model clone + compilation clone`.

**New tests and their test value.**
- `protocol/tests/solo_allocation.rs::a_whole_path_solo_preparation_clones_no_compilation`: red if a
  whole-path solo clones the committed `CompiledSession` again, even transiently. No existing test
  measures a solo preparation's allocations. `protocol` gains `bench-support` as a dev-dependency for
  the audited allocator's thread-scoped counters.
- `protocol` `controller::tests::a_monitor_token_refuses_an_observation_ended_attachment`: red if a
  monitor token accepts an `OBSERVATION_ENDED` attachment or holds a slot for it. Its commit emits no
  `SESSION_COMMITTED`, so the event would be acked and then dropped. The refusal (`MonitorOnly`) had
  no test on base, and the spec makes it read the new value.

**Mutations** (each applied to the committed tree, run, observed, and reverted with `git checkout`):

| # | mutation | observed |
|---|---|---|
| M1 | solo branch of `prepare_transaction_with` does `drop(self.compiled.clone()); None` | RED: `solo_allocation` 5314 B |
| M2 | `TokenCommit::Monitor => path` (the host's path) in `commit_prepared_structural` | RED: `a_transaction_of_monitor_writes_commits_no_model_revision_or_event` |
| M3 | `attach_observation_ended` returns `Ok(0)` for a monitor token's non-empty list | RED: `a_monitor_token_refuses_an_observation_ended_attachment` |
| M4 | `reserve_session_committed` never returns `Monitor` | RED: both monitor tests (the commit's debug assert and the token's `is_monitor_only`) |
| M5 | `compiled(store)` returns `None` for `MonitorWhole` | RED: the protocol monitor test, and 10+ `control-plane` observation tests (the live arm's `expect`), which confirms that the live arm meets whole-path solos |

**No behaviour moved (one-time comparison, not committed).** A temporary probe in
`controller/tests.rs` ran on base and on head. It printed the bytes of a solo commit (host path `Live`),
its events, revision and exact-replay bytes; a mutation token with a two-key `OBSERVATION_ENDED`
attachment (attach length, response bytes, both event frames, revision); and a one-call mutation
(response bytes, event frame, revision). The output was byte-identical on base and head (11 lines, sha256
`95cac8c2...d9e575` for both). No render code changed, so a render no-bit-moved comparison does not apply.

**Gates (head).**
- `cargo test --locked`: `protocol` (`--all-targets`, `test-support`), `host-core` (`test-support,control-provider`),
  `control-plane` and `capi` (`--all-targets` with `control-plane,protocol,host-core/test-support`): all green.
- `cargo clippy --locked -p protocol -p control-plane -p host-core -p capi --all-targets --all-features -- -D warnings`:
  clean. The same for the other dependents (`host-web`, `audit`, `bench`, `console-workload`,
  `parameter-metadata`, `conformance`).
- `cargo fmt --check`: clean. `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` for the four crates: clean.
- `scripts/check-workspace-policy.sh`: ok. `realtime_policy`: ok (298 marked regions in 48 files).

**Dropped claims.** None.

**Follow-ups.**
- #SUCCESSOR: a whole-path solo still clones the committed *model* as its candidate (857 B on the
  fixture; it grows with the session). It is freed when preparation ends; the token does not hold it.
  No edit of a monitor-only transaction writes the candidate, so the edits could borrow the committed
  model instead (low priority, for the performance pass).

### Sol verdict on attempt 1 (`3a4846b87`): PASS, minor findings only

Folded on the same branch:
1. **The lend's precondition now holds in release builds.** In attempt 1,
   `PreparedSessionTransaction::compiled` checked only with a `debug_assert` that the store was at
   the token's base revision. With the public `prepare_transaction_with_monitor` followed by
   `apply_transaction` on the same store, a release build would lend the newer compilation. It now
   returns `(store.revision() == self.base_revision).then(|| store.compiled())`, so it lends nothing
   once the store has moved.
   - New test: `model::tests::a_solo_lends_no_compilation_once_the_store_has_moved`. It turns red if
     the lend skips the revision check, because the token would then hand out a compilation it was
     never prepared against.
   - Mutation: replacing the check with plain `Some(store.compiled())` makes it RED at "a moved
     store's compilation is not the solo's". On attempt 1's head, the `debug_assert` panics in a
     debug build.
   - No production caller can meet a moved store. The controller refuses a stale token before any
     reader runs, and control-plane reads under the same `&mut` borrow.
2. **The candidate-model clone gets a successor.** A stateless successor spec was written, and the
   follow-up above names it as `#SUCCESSOR`.
