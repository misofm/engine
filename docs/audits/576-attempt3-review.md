# Issue 576 attempt 3 review

Reviewed head: `bb839fcc57651695b35276d578d8f6215a313817`

Reviewer: Astra LOW

Verdict: **FAIL — three-attempt hard stop reached**

The final attempt repaired premature ordinary collection through acknowledged-cancel
gating, installed allocator liveness, moved rendezvous ownership inside the scope,
added compile-fail thread-affinity checks, enforced allocation caps and compared
ordinary endpoint PCM with a separately prepared console render. Full host-core
all-target tests, eleven endpoint tests, doctests, strict Clippy, formatting/diff,
workspace/host policy and Wasm scalar/simd128 compilation passed.

Five frozen gates remain blocking:

1. `poll_cancel_boundary` still returns generic completion without joining outstanding
   Applied endpoint metadata and terminals.
2. The claimed post-claim test publishes only after the whole render returns, so it
   cannot detect an implementation that drains newly published work later in the same
   block.
3. The endpoint-specific audio proof lacks an addressed nonbanked scalar owner,
   state/post-fader equivalence, direct paired-dispatch refusal, and actual bitwise PCM
   comparison.
4. `host_retained_bytes` still names a host-wide quantity while containing only
   builtin retention; render inline reporting excludes started plan/fault state; and
   the test repeats report arithmetic instead of independently observing retained
   allocations. Repeated lifecycle reuse is also not established.
5. The mutation record mislabels prefix corruption as partial injection and does not
   preserve concrete mutations for a second within-block claim, post-claim same-block
   application, or Canceled-after-uncertain-application. The fault test never attempts
   cancellation after the sticky fault.

No fourth #576 revision is authorized. Preserve source head `bb839fcc` and all three
reviews. A numbered bounded successor may own only endpoint cancellation
reconciliation, truthful resource reporting, and the missing discriminating endpoint
tests/mutations within the same host-core isolation. It must not enable pairing,
expand protocol/graph APIs, or relabel this attempt.
