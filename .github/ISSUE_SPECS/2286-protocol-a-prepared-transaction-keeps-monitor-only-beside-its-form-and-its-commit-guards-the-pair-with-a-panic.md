# Protocol: a prepared transaction keeps monitor_only beside its form, and its commit guards the pair with a panic

Tracker: #2258, item PR-8. Found by the implicit-state-machine audit of 2026-11-06.

## Problem
`PreparedSessionTransaction` (`crates/protocol/src/model.rs`) stored `monitor_only: bool` next to
`form: TransactionForm` (`Compiled`, `InPlace`, `Monitor`). The bool and the form described one
fact twice:
- `!monitor_only` with `Monitor` is a state the type admits and no constructor builds.
  `commit_prepared_displacing` met it with `unreachable!`.
- `monitor_only` with `Compiled` is a *legal* state, not a mismatch: the whole path's solo
  (`prepare_transaction_with`, which clones the committed compilation, so `compiled()` lends it).
  The tracker's wording is corrected here. `commit_prepared_displacing` refused this state only
  with a `debug_assert`; in release it reinstalled the clone and returned it as "displaced".
- Whether committing a solo is safe therefore rested on every caller choosing the right entry
  point: `commit_or_keep` for `apply_transaction_with_monitor`/`apply_transaction_in_place`, and
  the controller's `if !monitor_only` in `commit_prepared_structural`.

## Contract
- Find out, and record, whether any public path builds a mismatch or commits a monitor-only token
  through `commit_prepared`. If one does, a regression test reaches it, red on main.
- Delete the bool. The form alone says whether a transaction is monitor-only: `Monitor` splits
  into `MonitorWhole(CompiledSession)` (the whole path) and `MonitorInPlace(Vec<SessionEdit>)`
  (the in-place path).
- The commit is total: a monitor-only token commits nothing, returns the committed revision and
  displaces nothing. No `unreachable!`, no `debug_assert` on the pair. `commit_or_keep` goes.
- No public accessor's answer changes for any token a public path builds.

## Non-goals
`controller.rs` and `control.rs` are not edited (other branches own them): their readers
(`is_monitor_only`, `compiled`, `is_in_place`, `edited_owners`, `in_place_estimate`) keep their
meaning, and the controller's `if !monitor_only` guard stays, because it also decides the event
reservation and the edit path. The whole path's clone of the committed compilation for a solo is
not removed.

## Gates
- A red-on-main regression test, or a recorded proof that no public path reaches the pair.
- `cargo test --locked` for `protocol` and its in-workspace consumers (`control-plane`,
  `host-core`, `capi`), clippy `-D warnings`, `cargo fmt --check`, `cargo doc -D warnings`,
  `scripts/check-workspace-policy.sh`, the realtime policy tool.

## Evidence (attempt 1)

Base: `main` @ `24ea03525`.

**Reachability: no public path reaches either guard today.**
- Constructors. The fields are private, and `PreparedSessionTransaction` is built in exactly
  three places, all in `model.rs`: the end of `prepare_transaction_with` (`Compiled`, with
  `monitor_only` from `monitor && is_monitor_transaction(edits)`), and `prepare_in_place`'s two
  returns (`Monitor` with `true`; `InPlace` with `false`). So `!monitor_only` with `Monitor` is
  never built. `monitor_only` with `Compiled` is built by every whole-path solo, by design.
- Commits. `commit_prepared_displacing` is `pub(crate)`, and it has three callers:
  - `commit_prepared`. This serves `apply_transaction`, whose `prepare_transaction` passes
    `monitor = false`, so the token is never monitor-only. It also serves the controller's one-call
    path, `process_structural_plan_into` (`controller.rs`, `self.session.commit_prepared`). That
    path plans with `Preparation::Whole`, so `monitor = false` (`controller.rs`,
    `let monitor = preparation != Preparation::Whole`).
  - `commit_or_keep`. It returned early for a monitor-only token.
  - `commit_prepared_structural`. It calls the commit only under `if !monitor_only`.
- So the `unreachable!` and the `debug_assert` are unreachable from the public API. They held
  only because each caller chose correctly. The defect is the representation, not a live failure.
  It is hardened anyway, as the tracker asks.

**Model chosen.** `TransactionForm` has four variants: `Compiled`, `InPlace`, `MonitorWhole`,
`MonitorInPlace`.
- A nested `Monitor(MonitorForm { Whole, InPlace })` was tried first. Clippy refused it
  (`large_enum_variant`: 568 bytes against 24). Boxing it would add an allocation for no gain, so
  the variants are flat. The four variants exclude each other just as well.
- `MonitorWhole(CompiledSession)` is the whole path's clone of the committed compilation.
  It is what `compiled()` lends, which `control.rs`'s non-in-place branch (`expect("a compiled
  transaction")`) and `controller.rs`'s `removed_arm_tags` read.
- `MonitorInPlace(Vec<SessionEdit>)` holds the edits, for `materialize`.
- The accessors read the form:
  - `is_monitor_only`: `MonitorWhole | MonitorInPlace`.
  - `is_in_place`: `InPlace | MonitorInPlace`. Base answered `!Compiled`, and a whole-path solo
    was `Compiled` on base, so the answers match.
  - `compiled`: `Compiled | MonitorWhole`.
  - `materialize` returns a `Compiled | MonitorWhole` token as it is.
  - `revision`: both monitor variants give the base revision, which equals the clone's revision.
- Every accessor answers as on base for every token a public path builds.
- `commit_prepared_displacing`'s two monitor arms return `None` and writes nothing. The
  `unreachable!`, the `debug_assert` and `commit_or_keep` are deleted, and both `apply_*` entry
  points call `commit_prepared`. The doc of `in_place_estimate` claimed it returned "the committed
  estimate for solo writes only", but it returns `None` (`control.rs` substitutes the committed
  estimate). The doc now says so. The code is unchanged.

**Test.** `model::tests::a_compiled_commit_returns_the_compilation_it_displaced` is extended, not
duplicated: the claim is still "what a commit displaces".
- It prepares one solo both ways (`prepare_transaction_with_monitor`, then
  `prepare_transaction_in_place`) and commits each through `commit_prepared_displacing`.
- Each must keep the revision and the model, and displace nothing.
- On base `24ea03525` (test extension added, model unchanged):
  - debug: `panicked at crates/protocol/src/model.rs:1782:9: a monitor-only transaction commits no
    model` (the `debug_assert`; the in-place form reaches the same assertion first).
  - release: `panicked at crates/protocol/src/model.rs:2246:13: whole: a solo displaces no
    compilation`. The whole form reinstalled its clone and reported it as displaced. The in-place
    form would then hit `unreachable!`.
- At head: passes.
- Test value: this is the only test that commits a whole-path solo token through the commit
  itself. A commit arm that reinstalls the clone, bumps the revision or panics turns it red. The
  `apply_*` tests only see the revision, and the controller never commits a solo token.

**Mutations (head).** Each run is `cargo test --locked --no-fail-fast -p protocol`.
- M1: the commit's `MonitorWhole` arm reinstalls its compilation. Red:
  `a_compiled_commit_returns_the_compilation_it_displaced`.
- M2: `is_monitor_only` matches `MonitorInPlace` only. Red:
  `a_transaction_of_monitor_writes_commits_no_model_revision_or_event` and the test above.
- M3: the whole path never builds `MonitorWhole` (a solo becomes `Compiled`). Red: the same two.
- M4: `is_in_place` back to `!Compiled`. Red:
  `a_solo_and_a_master_of_one_strip_are_two_monitor_writes_on_both_paths`.
- M5: `compiled` returns `None` for `MonitorWhole`. Red:
  `a_transaction_of_monitor_writes_commits_no_model_revision_or_event`.

**No bit moved.** No render-path code is touched: `PreparedSessionTransaction` is control-plane
preparation state, and no rendered value reads the deleted bool.

**Gates (head).**
- `cargo test --locked --all-targets -p protocol -p control-plane -p host-core -p capi --features
  protocol/test-support,control-plane/test-support,host-core/test-support`: 1,159 passed, 0 failed
  (89 binaries). This includes `host-core/tests/in_place_commit.rs`, which applies solos through
  both `apply_*` entry points, now via `commit_prepared`.
- Doctests for `protocol`, `control-plane` and `host-core`, with the same features: 9 passed.
- `cargo clippy --locked` on the same packages and features, `--all-targets -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps -p protocol`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (292 marked regions in 46 files)`.

**Follow-ups (not in this issue).**
- The whole path clones the committed compilation for a solo, only so that `compiled()` can lend
  it. A reader could take the committed compilation from the store instead. That would move the
  readers in `control.rs` and `controller.rs`, which other branches own.
