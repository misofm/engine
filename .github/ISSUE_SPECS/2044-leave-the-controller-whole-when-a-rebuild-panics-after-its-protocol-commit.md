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
- `cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi --features test-support`.
- clippy `-D warnings` on the touched crates with no features and with `test-support`; AArch64 check.

## Paths
`crates/control-plane/src/control.rs`, its tests, `crates/capi/src/runtime/supersede_tests.rs`, this spec.
