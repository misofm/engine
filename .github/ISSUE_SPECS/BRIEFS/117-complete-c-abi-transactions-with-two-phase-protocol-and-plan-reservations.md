# Sol implementation brief — issue 117 two-phase C ABI transactions

## Decision

**TERMINAL STOP / TECHNICAL INPUT FOR ISSUE 118 ONLY.** Checkpoint 1 passed at
`c9bd936673bfe167d783ca6f2a62c495c0928f37`, preserving the accepted two-phase protocol token and
plan publication/retirement reservation. Checkpoint 2 retained the correct private protocol-commit,
provider-install, non-fallible-publish, response-last order and additive event ABI, but Sol XHigh
found double-live CAPI resource under-accounting and a missing cross-component evidence matrix. The
sole HOLD budget is exhausted. Technical checkpoint
`e1115750fba8a54e16ec2a0e333b40ce4f187f1c` is preserved, but Issue 117 has no overall PASS.

Direct accepted dependencies are **Transport-neutral binary control protocol** (005), **Real-time
memory, buffers, queues, and plan lifetime** (003), and **Stable C ABI and host-fed planar PCM
render** (022). Stopped **Close C ABI control/event transport and transactional plan replacement**
(113) is readiness evidence only. Successor **Close C ABI replacement resource accounting and
cross-component evidence** (118) consumes the stopped Issue-117 checkpoint plus accepted 005, 003
and 022. Accepted 118 gates **Optional binary WebSocket sidecar** (025); accepted 116 + 118 gate
**Qualify native C ABI and reference runner target matrix** (114), then 026.

## Frozen seams

Protocol: introduce an affine controller-bound prepared structural-command token. Preparation must
run the accepted decode/semantic/revision/replay/resource ordering and own the prospective compiled
session, canonical bytes, exact response, reliable event, cancellation effect and replay completion
without mutating live state. Immediate/cached/conflict/backpressure decisions remain byte-exact and
token-free. Commit is non-fallible/exactly once; cancel/drop is mutation-free. The existing one-call
API shares this machinery. No wire, opcode, status, diagnostic, event, replay or canonical byte may
change.

Core: introduce an affine exchange-bound replacement reservation for the exact envelope/epoch, one
publication slot and one eventual displaced-plan retirement credit. Commit with the bound plan is
non-fallible; cancel/drop returns both credits. The render owner consumes the pre-admitted credit at
the boundary and never performs a new fallible retirement admission for the Issue-117 path. Old
plan/provider destruction remains bounded and off render. Preserve existing one-shot callers and
render-report meanings.

CAPI: preserve ABI V1 and add only the frozen reliable/lossy event dequeue symbol. Structural order
is protocol prepare -> source/complete-plan prepare -> all admissions -> non-fallible protocol
commit kept private to the serial control call -> install the matched provider epoch and
non-fallibly publish its plan -> return the committed response. Any earlier failure cancels
everything and preserves live session/revision/provider/plan/event/replay state and caller buffers.
Publish last so a concurrent render boundary cannot observe the new plan before protocol commit.
Host submission follows the committed provider epoch; old plan/provider remain paired until
off-render reclaim.

## Evidence and fence

Checkpoint 1 closed protocol token parity/ownership/cancel and core reservation/credit/render
safety. Checkpoint 2 did not close exact double-live resources, all-command/six-event byte parity,
source-preserving/changing PCM boundaries, provider epochs, retirement/destroy, exhaustive per-phase
and dual-fault atomicity, or disposal/allocation accounting. Those omissions move intact to Issue
118; they are not waived or qualification work for Issue 114.

Allowed: the narrow protocol controller/model/export/tests, core plan exchange/export/tests,
`miso-engine-capi/**`, minimal manifest/lock, additive event header/docs and exact policy/mutation/
evidence files. Frozen: protocol bytes, session schema, source decode, graph/DSP/effects, render
contract, existing ABI behavior, runner/fixtures and hosts. No benchmark, timing, real workload,
playback, listening, browser/device run or Issue-114 matrix.

Final verdict: **STOPPED**, checkpoint 1 technical PASS only, no overall acceptance. Reported
benchmark/timing/real-workload/playback/listening/browser/device counters remain zero.
