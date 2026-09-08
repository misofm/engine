# Issue #579 attempt 3 adversarial review and hard stop

Reviewed exact pushed head: `d67becc3d4dd7100faf3b172c6d18e87d963bfaa`

Reviewer: Astra LOW

Verdict: **FAIL — three-attempt hard stop reached**

Accepted corrections include cancellation exact-once/token/gating behavior, the
test-only backend preparation and dev-dependency witness support, restored production
meter behavior, actual queue-retention measurement, forced-scalar state/PostFader
evidence, and direct zero-pair witnesses. The amended path boundary is respected and
production backend selection remains pinned.

Two blockers remain:

1. `prepare_builtin_batch_endpoint_with_backend` invokes the allocating
   `prepare_endpoint_queues` helper before endpoint retained/largest cap checks. This
   regresses the original allocation-before-cap refusal. A bounded successor must
   restore report-only projection and cap validation before actual queue allocation,
   with allocator evidence that endpoint-cap refusal allocates no endpoint queues.
2. The new native-bank and forced-scalar endpoint/reference PCM arrays use direct
   `f32` equality. The frozen gate requires mapped `to_bits` comparison so signed-zero
   drift cannot pass.

Independent complete debug/release host-core all-target suites, 14 endpoint integration
tests, doctests, strict Clippy/rustdoc, formatting/diff, workspace/host policies, CI
routing/classifier checks, and Wasm scalar/simd128 compilation passed.

No fourth #579 revision is authorized. Preserve exact source head `d67becc3` and all
three reviews. A newly numbered successor may own only the cap-order correction and
allocator-backed refusal discriminator plus the two PCM bit comparisons; it must not
reopen cancellation, backend, meter, resource-layout, pairing, protocol, graph,
artifact, or lane-B scope.
