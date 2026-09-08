# Issue 594 attempt 2 — Astra LOW review

Reviewed exact clean pushed evidence checkpoint
`edb73387599a36f6ab5594653096eee0f04e66d2` against delivered main and the approved scope.

Verdict: **FAIL**. One implementation attempt remains.

Both attempt-1 production blockers are fixed. Astra accepted pending-intent rendering, ordinary
completion/overtake gating, matched shutdown completion, combined Applied/claimed/queued accounting,
paired native/scalar quiescence, sticky-fault ownership, zero shutdown render allocation/free, and
mutations one through four.

Three bounded evidence corrections remain:

1. Mutation five disables the terminal early return but supplies a discontinuous sample, so it fails
   validation before graph execution. Repeat at the unchanged valid sample/shape and require a direct
   PCM, plan-time, addressed-state, raw-drain, or pair-process discriminator. The attempt-2 record is
   corrected to remove its inaccurate graph-execution claim.
2. The Arc allocation test allows `actual >= reported`, so undercharging passes. Require exact
   independently observed allocation count and requested bytes. The aggregate teardown assertion can
   pass on unrelated plan frees; use the authorized private unit-test module to prove the lifecycle
   allocation survives control drop and `stop()`, then is reclaimed only when the stopped owner drops
   off render.
3. The pre-publication rendezvous retains its release sender outside `thread::scope`; an early panic
   can hang scope join. Move release ownership into the scoped closure or add failure-safe drop/release,
   then show a deliberate failing assertion exits rather than hanging.

Independent debug and release runs each passed 20 integration and nine endpoint unit tests. Strict
Clippy, warning-free rustdoc, formatting, and diff checks passed. Exact source ownership remains in
scope. The generated Cargo.lock ordering drift was restored.
