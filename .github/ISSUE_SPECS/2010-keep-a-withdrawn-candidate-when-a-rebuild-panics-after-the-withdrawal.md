# Keep a withdrawn candidate when a rebuild panics after the withdrawal

Filed by root from slice 1324a (backlog sprint, 2026-10-25).

## Defect
The control plane withdraws the pending candidate before it prepares a rebuild's successor (#1310). If the
preparation then panics and the host catches the panic (the C ABI does), the withdrawn candidate is lost:
its revisions were acked, the publisher's "published" mark is used up, the next `withdraw()` returns
`Nothing`, `withdraw_pending_candidate` promotes the lost candidate as if render had taken it, its epoch is
never retired, the source producers bind to an epoch render never runs, a later seek never reaches the
running plan and the fed ring fills (`source.backpressure`). This is an ack before a drop.

1324a removed the one known trigger (a `debug_assert` in the input join for a high-pass above its low-pass);
any other panic in the same window still reaches the defect.

## Product outcome
A panic during a rebuild's preparation leaves the session exactly as before the call: the withdrawn candidate
is restored (or was never withdrawn), the call is refused with a typed error, and render, the source
producers and the retirement queue stay consistent.

## Smallest closable slice
Either restore the candidate on unwind (a guard that republishes the withdrawn candidate when the rebuild
does not complete) or withdraw only after the successor is prepared. Choose by the #1310 ordering constraints
and record why. One red-first reproducer: a test-only injected panic inside preparation after the withdrawal,
then the next edit and a seek behave as on a session that never saw the call.

## Paths
`crates/control-plane/src/control.rs`, its tests, this spec.

## Decision record (2026-10-25, root R1/R2 from the scope note)
- D1 (choice): restore the candidate on unwind, not withdraw after preparation. #1310 D1/D2 make the
  superseding preparation read the withdrawn candidate (its carried base in phase one, the source
  donation check, the "a withdrawn candidate is pending" check at the top of `rebuild`), so the
  withdrawal must precede preparation.
- D2 (mechanism, R1): the three supersession sites (transaction, declared discontinuity, spectrum
  monitor change) call one `rebuild_superseding` in `crates/control-plane/src/control.rs`. It
  withdraws, runs `rebuild` under `catch_unwind(AssertUnwindSafe)`, republishes a candidate that is
  still `Some` (refusal or panic), then `resume_unwind`s a panic. The C ABI still answers
  `RESULT_INTERNAL` through `catch_result`. A guard object cannot hold `&mut self.publisher` beside
  `rebuild(&mut self)`.
- D3 (scope, R2): the guard covers a panic up to the commit (`withdrawn.take()` in `rebuild`). The
  writes after the last fallible step and before that take (`fold_send_move`, `self.monitor`,
  `self.observation`) would leave a half-committed state on a panic; that is a successor defect
  (a panic after the commit), not this issue. Starting fact for that successor: a panic between the
  protocol commit (`prepared.commit`) and `withdrawn.take()` still republishes the candidate A, but
  over a half-committed controller, which then holds B's model, revision, monitor overlay,
  observation and send moves while A renders.
- D6 (verifier fold): `docs/REALTIME_DEPENDENCY_POLICY.md` lists the new `catch_unwind` boundary
  (control thread only; nothing unwinds under `panic = "abort"`).
- D4 (reach caveat): the release profile is `panic = "abort"` (`Cargo.toml`,
  `docs/REALTIME_DEPENDENCY_POLICY.md`), so a shipped host aborts before any half state exists and the
  `catch_unwind` changes nothing there; no rendered bit moves. The window is reachable where panics
  unwind: every `cargo test` harness, dev/debug C ABI builds (mobile development), and any host built
  without the abort profile. It is still an ack before a drop, so it is fixed.
- D5 (test): `TestStructuralFaultPhase::PanicAfterWithdrawal` (test-support only) panics inside
  `rebuild` after the successor is prepared and before admission. One test,
  `crates/capi/src/runtime/supersede_tests.rs::a_rebuild_that_panics_after_the_withdrawal_restores_the_candidate`.

## Evidence
- Red on main b0bc367af with only the fault phase added: `candidate_republished` is 0, not 1; with
  that assertion relaxed, block 8's feed is refused with `source.backpressure` (the lost candidate's
  rings fill). Green with `rebuild_superseding`.
