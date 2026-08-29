# 267 EffeTune Graph v1 architecture comparison: investigate compile snapshots and permutation gates

One-line summary: EffeTune Graph v1 independently validates V2-like deterministic topo, integer PDC,
liveness reuse and fixed render workspace; adopt its diagnostic/permutation evidence patterns, while
rejecting fixed limits, mutating compilation, reset-on-install and app-path realtime extrapolation.

**This is a completed research and decision record, not implementation authority.**

**Authority: GitHub issue #267.** This local file mirrors its source-backed decision record.

## Authority, pins and method

- Engine V2: [`90c3b9a598f1244938d9cdcce04c4a4641c6b758`](https://github.com/misofm/engine-v2/tree/90c3b9a598f1244938d9cdcce04c4a4641c6b758).
- EffeTune: [`bedc6c662a6edc88c9644b7e00cec9122a250cfb`](https://github.com/Frieve-A/effetune/tree/bedc6c662a6edc88c9644b7e00cec9122a250cfb).
- Static source/CI audit only: no execution, benchmark or legacy Miso access.

## Findings

Graph v1 is explicitly a static one-input/one-output, one-port DAG
([reference](https://github.com/Frieve-A/effetune/blob/bedc6c662a6edc88c9644b7e00cec9122a250cfb/docs/dsp/reference/graph-v1/index.md#L10-L19)).
It selects lexicographically ready nodes, sorts reductions by edge ID, computes per-channel integer
longest-path compensation, reuses buffers by liveness and allocates workspace/delay lines before
process
([graph.cpp](https://github.com/Frieve-A/effetune/blob/bedc6c662a6edc88c9644b7e00cec9122a250cfb/dsp/core/graph.cpp#L314-L704)).
Process follows that fixed schedule under an allocation guard. This independently supports V2's
existing planner decisions; it does not justify a transplant.

Graph v1 exposes a useful read-only compile snapshot: effective/dormant/bypass state, schedule,
buffer slots, channel groups, latency/PDC and capacity use
([reference](https://github.com/Frieve-A/effetune/blob/bedc6c662a6edc88c9644b7e00cec9122a250cfb/docs/dsp/reference/graph-v1/index.md#L253-L297)).
V2 already emits a graph digest and resource reports, but a compact protocol-visible per-node/edge
snapshot could improve agent diagnosis if generated off-render from the completed plan.

Candidate construction is not pure: it applies pending parameters to shared effect instances, and
successful installation resets graph instances and delays
([engine.cpp](https://github.com/Frieve-A/effetune/blob/bedc6c662a6edc88c9644b7e00cec9122a250cfb/dsp/core/engine.cpp#L1004-L1038)).
The docs disclaim state-preserving/clickless replacement. V2's transactional model preparation and
ownership-moving exchange remain stronger.

The C++ library's allocation evidence cannot be extrapolated to the shipping AudioWorklet, which can
reconfigure pipelines, poll deferred assets, check memory replacement, log/post messages and allocate
nonstandard buffers from processing branches. Graph v1 is explicitly not the app executor. Its
allocation guard is Debug-only and narrower than V2's lock/I/O/log/syscall audit.

Graph v1 also hard-codes 128 nodes, 96 effect instances, 512 edges, 129 live buffers and 64 MiB
workspace
([capacity](https://github.com/Frieve-A/effetune/blob/bedc6c662a6edc88c9644b7e00cec9122a250cfb/dsp/bindings/generated/graph-v1-capacity.h#L6-L11));
lacks sidechains/multi-port/multi-bus and scheduled graph events; and its master bypass resets delay
history instead of preserving declared latency. These conflict with V2.

## Decision

- **Investigate:** a compact versioned V2 compile snapshot keyed by stable node/edge IDs with
  effective state, schedule index, buffer slot, latency/PDC and exact resource rows. Generate it
  off-render from the completed immutable plan; cap its bytes.
- **Adopt as evidence:** graph-declaration permutation gates; same-frame automation merge differential
  cases; artifact rebuild/provenance checks; strict asset length/hash failures.
- **Preserve:** resource-derived capacities, pure prospective compilation, state-preserving exchange,
  latency-preserving bypass, broad realtime audit, streaming, AoSoA and one C render path.
- **Reject:** fixed ceilings, candidate mutation of shared instances, reset-on-install, missing
  ports/sidechains/buses/events, latency-removing bypass, app-path allocation/logging and whole-file
  offline decode.

## Objective gates

1. Random declaration permutations preserve canonical session, graph digest, compile snapshot,
   schedule, PDC, resources and PCM bits.
2. Failure injection after every compile phase leaves session, plan, parameters/state, delay history
   and revision unchanged.
3. Compatible live replacement preserves state/clock/latency without click, allocation/free or drop;
   bypass keeps exact declared latency.
4. Browser render call graphs pass allocation/free/log/postMessage/memory-grow hooks.
5. Snapshot generation is capped, deterministic, off-render and unable to mutate the plan/DSP.
6. SIMD artifact growth requires a named measured win under existing scalar/Simd4/Simd8/Wasm
   semantic gates; long assets remain ring-streamed.

## Limitations

The strongest evidence is limited to an experimental library boundary which its own docs exclude
from the shipping app. Its source/CI does not establish whole-application realtime correctness or
universal bit identity.

## Closure

Closed as completed research and superseded by final synthesis #268. A protocol-stable physical
compile snapshot was rejected; existing canonical graph evidence remains authoritative.
