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
(empty)
