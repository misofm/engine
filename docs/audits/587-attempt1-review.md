# Issue 587 attempt 1 adversarial review

Reviewer: Astra LOW

Reviewed exact upstream source checkpoint:
`0b84ecbce4a4bd31aefa9bfb2e12810027ca1cfc`.

Verdict: **FAIL**. Two implementation attempts remain.

## Accepted behavior

The endpoint-only `BetweenRenderCalls` routing is sound. Public raw preparation
remains `Concurrent`, production stays pinned to `Backend::current()`, and no public
backend selector was added. Existing PostFader tests directly prove zero pair
selection for native bank and forced scalar execution while comparing PCM, state,
and meter bits, so they are valid decline evidence. The policy-off and temporary
paired-arithmetic mutations discriminate their intended claims and the excluded
compiler source is restored. Six endpoint unit tests, fourteen integration tests,
and strict Clippy pass. No production cancellation, resource, or realtime regression
was found.

## Blocking evidence gaps

1. The selected-pair test renders only one 128-frame block with 16/11-sample ramps.
   It must drive immediate, ongoing-ramp, settled, mid-ramp retarget, mute, and unmute
   blocks for both native and scalar backends, comparing endpoint/reference PCM bits
   after every block.
2. Positive pair counts and nonzero scalar words do not compare paired/reference
   target/ramp state or prove exact record drains and member accounting. The shared
   witness currently includes reference rendering before endpoint rendering. Capture
   independent snapshots or deltas for each owner and assert exact expected drains,
   members, and applicable state at each transition.
3. The native selection test uses a private preparation helper. Exercise the public
   `prepare_builtin_batch_endpoint` constructor for production native selection.
   Release, rustdoc, policy/router, and target gates remain for the corrected
   checkpoint.

## Attempt 2 boundary

Preserve the accepted policy change. Extend only the existing selected-pair/reference
fixture and its assertions in approved host-core paths. The already accepted decline
test needs no duplicate fixture. Cargo lock ordering drift remains unowned and must
be restored after commands.
