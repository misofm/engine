# Issue #600 attempt 2 review

Reviewer: Astra LOW

Verdict: **FAIL**

Exact pushed head: `000de31e30bacaabb68576be958a26b50fcaad21`

One implementation attempt remains. Final preflight and timing are not authorized.

Attempt 2 correctly separated untimed render tests from the timing helper, moved buffer validation
outside the timed closure, used checked counters and exact elapsed values, added a useful
capacity-one queue-drain witness, corrected the encoded Rust flags and inherited-profile handling,
changed to the repository cwd, preserved explicitly handled raw failures, added source/binary
identity cases and reached five synthetic launches. Safe subject, validator/lifecycle, strict
Clippy/rustdoc/format/diff, policy, and unchanged #238/#496 tests pass.

Six blockers remain:

1. The external trim oracle does not render or compare an owner, so it cannot detect a broken runtime
   ramp; the positive digest and added witness hashes remain self-derived.
2. No durable restored source-mutation diff, command, intended failure and restoration proof exists.
3. The subject emits two per-round timing-start markers while the runner and banner claim one timed
   benchmark invocation; the synthetic child emits one and masks the mismatch.
4. Full strict seal validation happens only after launch; permissive partial parsing can consume the
   sole workload before rejecting duplicate/wrong/incomplete seal fields.
5. The validator accepts a Scalar backend and permissive numeric seal-header types.
6. Some failure dispositions invent a validator status, advertise a scratch path later deleted by
   cleanup, and do not cover failure after timing begins.

No real subject, final preflight, capture, timing helper or source mutation ran in review. The runner
defect remains after its one bounded correction, so the issue's circuit breaker moves runner repair
and capture promotion to a separate tooling successor. #600 must be rebriefed before attempt 3 to
retain only a closable subject/untimed-runtime proof and remove the known-defective new runner paths.
