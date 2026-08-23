# Sol implementation brief — issue 117 two-phase C ABI transactions

## Decision

**STATELESS SOL XHIGH BRIEF / READY FOR SOL HIGH PASS 1 AFTER REMOTE SYNC.** Stopped Issue 113
proved that its one-call accepted protocol controller and unreserved plan exchange cannot implement
an atomic C structural transaction in either order. Add exactly the two missing shared capabilities,
then close their CAPI vertical. Sol High implements; Sol XHigh verifies. One pass plus one bounded
HOLD correction; a second HOLD stops. Benchmark, timing and real-workload counters remain zero.

Direct accepted dependencies are **Transport-neutral binary control protocol** (005), **Real-time
memory, buffers, queues, and plan lifetime** (003), and **Stable C ABI and host-fed planar PCM
render** (022). Stopped **Close C ABI control/event transport and transactional plan replacement**
(113) is readiness evidence only. PASS gates **Optional binary WebSocket sidecar** (025); accepted
116 + 117 gate **Qualify native C ABI and reference runner target matrix** (114), then 026.

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

Checkpoint 1 covers protocol token parity/ownership/cancel and core reservation/credit/render
safety. Checkpoint 2 covers all C commands/six event families, exact buffers/diagnostics/resources,
source-preserving/changing and serial replacements, old/new boundary output, provider epochs,
retirement/destroy, exhaustive per-phase and dual-fault atomicity, disposal/allocation accounting,
C11 ABI smoke and a static no-copied-protocol check. One clean proportional nonbenchmark seal closes
the issue; Issue-114 target qualification is separate.

Allowed: the narrow protocol controller/model/export/tests, core plan exchange/export/tests,
`miso-engine-capi/**`, minimal manifest/lock, additive event header/docs and exact policy/mutation/
evidence files. Frozen: protocol bytes, session schema, source decode, graph/DSP/effects, render
contract, existing ABI behavior, runner/fixtures and hosts. No benchmark, timing, real workload,
playback, listening, browser/device run or Issue-114 matrix.
