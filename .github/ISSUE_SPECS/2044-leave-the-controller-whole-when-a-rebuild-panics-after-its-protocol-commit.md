# Leave the controller whole when a rebuild panics after its protocol commit

Filed by root housekeeping (backlog sprint, 2026-10-25). Source: #2010 (closed), its decision record D3 (R2).

## Problem
#2010 made `rebuild_superseding` republish the withdrawn candidate when a rebuild panics. Its guard does
not cover a panic between the protocol commit (`prepared.commit`) and `withdrawn.take()`. Such a panic
still republishes candidate A, but over a half-committed controller: the controller then holds the
successor B's model, revision, monitor overlay, observation bindings and send moves while A renders. The
next edit is prepared against B's state that render never ran. This is reachable only where panics unwind
(every `cargo test` harness, dev/debug C ABI builds, a host built without `panic = "abort"`); the release
profile aborts, so no shipped rendered bit is affected.

## Evidence (main `c1a218022`)
- `crates/control-plane/src/control.rs:5090-5097`: `rebuild_superseding`'s doc names this window as not covered.
- `control.rs:5127`: the `catch_unwind`; `:5136`, `:5188`: the republish of a still-`Some` candidate.
- `control.rs:6043-6044`: `prepared.commit(&mut self.controller, protocol::EditPath::Rebuild)`.
- `control.rs:6088`: `fold_send_move(&mut self.send_moves, ...)`; `:6091`: `self.monitor` replaced;
  `:6093`: `self.observation` written; `:6096`: `withdrawn.take()`.
- Test hook from #2010: `TestStructuralFaultPhase::PanicAfterWithdrawal` and
  `crates/capi/src/runtime/supersede_tests.rs::a_rebuild_that_panics_after_the_withdrawal_restores_the_candidate`.

## Smallest slice
Make the window after the last fallible step infallible or restorable: either move every write that can
panic before `prepared.commit`, so nothing between the commit and `withdrawn.take()` can panic (state why
each remaining step cannot), or snapshot the controller fields the window writes and restore them on unwind
before the candidate is republished. Record which, by the #1310 and #2010 ordering constraints.

## Gates
- One red-first reproducer: a test-support fault phase that panics after `prepared.commit` and before
  `withdrawn.take()`; after the caught panic, the next edit and a seek behave as on a session that never saw
  the call (model, revision and observation equal the pre-call state; no `source.backpressure`). Record it
  red on main (PR evidence).
- `cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi` (capi has no `test-support` feature; its dev-dependency enables control-plane's).
- clippy `-D warnings` on the touched crates with no features and with `test-support`; AArch64 check.

## Paths
`crates/control-plane/src/control.rs`, its tests, `crates/capi/src/runtime/supersede_tests.rs`, this spec.

## Decision record (attempt 1, 2026-10-25)
- **D1 (root, Option 1: move).** `publish_primed` and its `expect` now run before `prepared.commit`,
  after every refusal (the commit's own predicate is checked just before, with no controller write
  between, so the commit cannot refuse after the prime flags are set). The candidate is taken right
  after the commit; `fold_send_move`, the monitor replace and the observation write follow the take.
  No protocol rollback (Option 2) was built.
- **D2 (the window).** Between the commit and the take only `take_displaced` runs: `Option::take`, which
  cannot panic. Its `debug_assert!(displaced.is_some())` is removed: it cannot fire, because the
  rebuild refuses a token without a compilation (`compiled().ok_or`) and a compiled token always
  displaces the committed model. `withdrawn` is `Some` on return exactly when the protocol commit did
  not happen. A panic after the take stays uncovered, as #2010 said.
- **D3 (found: the report lock; root ruling, attempt 2).** The rebuild held `shared.reports` from its
  capacity check to the publication, so a panic before the commit poisoned it and every later `service`
  refused (`source.epoch`), with or without a withdrawn candidate. Attempt 1's `clear_poison` on the
  republish is deleted. The lock is now taken for the capacity check only, in its own block, and again
  after the take for the `retain`/`push`, held to the publication as before (the row is in the table
  before `reservation.commit`). The second lock takes a poisoned lock as it is
  (`PoisonError::into_inner`): only the control plane writes the table, through `&mut self`, and every
  other holder only reads it. This also covers a rebuild that withdrew nothing.
- **D3b (prime flags, root ruling).** `publish_primed` writes the prime flags of the rings render reads.
  A panic after it and before the take (the commit's own steps) left a republished `Primed` W judged by
  the successor's flags. `rebuild_withdrawn` now sets them again (`restore_prime_flags`) before it
  republishes a candidate `self.primed` names: W's publication moved every producer it held vacant in
  from the epoch it succeeds, so W carries exactly the rings whose producers it holds and whose IDs that
  epoch holds vacant. Gate: `a_republished_warm_candidate_keeps_its_own_prime_flags` (control-plane):
  without the restore W is adopted with an unprimed ring (its prime underruns).
- **D4 (gate, re-expressed).** The spec's literal gate (a panic after `prepared.commit` leaves the session
  untouched) needs Option 2. Under Option 1 the gate is: a fault phase `PanicAtPrimedPublication` just
  after `publish_primed`, before the commit (attempt 2 moved it after the call, so the flags are written). On main (the phase after the commit) it is red: revision 44 (T2's) where T1's
  43 is expected, so the candidate was republished over T2's model. After the move it is green, and the
  run after it is bit-identical to the reference. Its last case, a rebuild that withdrew nothing
  panicking there, is red on attempt 1's code (`the session services: Internal`). It extends the #2010
  test; it is not a new test.
- **D5.** Class A: no rendered bit moves (only the order of control-thread steps before the publication
  changes, and the outputs of each step are the same). Under `panic = "abort"` nothing unwinds and this
  changes nothing. Acked-batch question: can an ack precede a drop? No: the ack is written after the
  commit, which now has no panic-capable step before the take; a panic before the commit acks nothing.
