# Issue #579 attempt 1 adversarial review

Reviewed exact pushed head: `2744cc7ba0d344553878fdd5ed590527f491ae57`

Reviewer: Astra LOW

Verdict: **FAIL**

The true post-claim rendezvous and partial-injection PCM mutation are accepted
improvements. Full host-core all-target tests with `control-provider`, including 12
endpoint integration tests and private source tests, strict Clippy, formatting/diff,
workspace and host policy, and Wasm scalar/simd128 compilation passed independently.
The worktree was clean at the upstream reviewed head and remained disjoint from lane B.

Attempt 2 must correct these findings without widening ownership:

1. Cached cancellation polling must validate the supplied token before every cached
   response. The attempt-1 branch can return another generation's completion or `None`
   to a stale token.
2. Keep the endpoint cancellation gate closed until its retained completion has been
   reported exactly once. Publication and another cancellation must refuse after the
   last captured ticket is collected but before that report; unrelated new work must
   not delay or overwrite the retained completion.
3. Use the existing builtins-compiler test-support fader/matrix witness on the render
   thread and same-module stopped-plan access. Do not substitute generic engine
   dispatch defaults. Add an addressed nonbanked scalar owner plus target/ramp-state
   and PostFader comparison to the exact PCM reference.
4. Observe actual retained endpoint allocations rather than repeating production
   layout arithmetic through private mirror types. Repeat healthy and nonempty
   cancellation lifecycle reuse; one healthy render plus an empty cancellation is
   insufficient.
5. Mutate the faulted cancellation path to permit a Canceled terminal or cancellation
   success. Returning a healthy graph report targets fault reporting rather than the
   required possibly-applied cancellation claim.

Attempt 2 is limited to these corrections in the already approved endpoint module,
focused test, spec, and audit paths. No render-session, protocol, manifest, artifact,
pairing, lifecycle-publication, or lane-B edit is authorized.
