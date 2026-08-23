# Sol implementation brief — issue 113 C ABI control and plan replacement

## Decision

**STATELESS SOL XHIGH BRIEF / READY FOR SOL HIGH PASS 1 AFTER REMOTE SYNC.** Add the smallest ABI V1
completion that makes the accepted Issue-005 controller usable from C and applies structural edits
through the accepted bounded plan-exchange lifecycle. Sol High implements; Sol XHigh verifies. One
pass plus one bounded HOLD correction; second HOLD stops. No benchmark, timing or workload.

Direct accepted dependencies are **Transport-neutral binary control protocol** (005), **Real-time
memory, buffers, queues, and plan lifetime** (003), and **Stable C ABI and host-fed planar PCM
render** (022). This issue gates **Optional binary WebSocket sidecar** (025). Issue 073 is independent.

## Frozen API and ordering

Keep ABI V1 and existing layouts/symbols. Add one hand-written event dequeue symbol using session,
the existing `bytes_out`, and a fixed reliable/lossy selector. Buffer-too-small and empty dequeue
must not consume. Use the accepted `ProtocolController`; do not copy its wire, replay, revision,
diagnostic or event policies.

For structural transactions: decode/validate -> prospective typed session/canonical TOML -> prepare
source endpoints -> compile complete plan -> reserve response/event/replay/publication/retirement ->
publish for next boundary -> atomically commit revision/model/provider epoch. Any failure leaves all
live state unchanged. Old source producers live with their matching old plan epoch until the
control-side retirer reclaims it. Full retirement defers. Render performs no destruction or other
forbidden work.

Nonstructural commands keep their accepted bounded queue behavior. Host submissions route to the
committed provider epoch only. Resource reports and limits include every new fixed allocation; no
lazy growth or implicit track ceiling.

## Evidence and fence

Prove byte-exact C/Rust parity for all commands/events, replay/revision and reliable/lossy behavior;
source-preserving/source-changing replacements; boundary output; serial order; every reservation/
compile/publication failure; disposal; allocation/drop/syscall safety; C11 layout/symbols; and clean
limits. Run proportional nonbenchmark gates only.

Edit `miso-engine-capi/**`, at most one sealed core realtime epoch-retirement seam, minimal
manifest/lock and exact checker/policy/docs rows. The accepted protocol/session/source/graph/DSP
contracts, runner and hosts are frozen. Hand off coherent controller/API and replacement checkpoints
within the same implementation pass.
