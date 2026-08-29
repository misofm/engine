# 266 openDAW architecture comparison: reject render reconciliation and retain measured specialization lessons

One-line summary: openDAW demonstrates useful same-render differential tests and profile-driven Wasm
specialization, but its lazy render graph rebuild, reserve-then-grow/truncate paths, latency-blind
effects, coarse automation and whole-source residency conflict with V2's core contract.

**This is a completed research and decision record, not implementation authority.**

**Authority: GitHub issue #266.** This local file mirrors its source-backed decision record.

## Authority, pins and method

- Engine V2: [`90c3b9a598f1244938d9cdcce04c4a4641c6b758`](https://github.com/misofm/engine-v2/tree/90c3b9a598f1244938d9cdcce04c4a4641c6b758).
- openDAW: [`4a9f183f63dfc7ad049b5f24eca6081205a7c61b`](https://github.com/andremichelle/openDAW/tree/4a9f183f63dfc7ad049b5f24eca6081205a7c61b).
- Static audit only: no execution, benchmark or legacy Miso access.

## Findings

openDAW stores processors behind `Rc<RefCell<dyn Processor>>`; when topology changes, render lazily
recomputes topological order and rebuilds a queue inside `process`
([engine_context.rs](https://github.com/andremichelle/openDAW/blob/4a9f183f63dfc7ad049b5f24eca6081205a7c61b/crates/engine-env/src/engine_context.rs#L198-L228)).
Its supposedly pre-reserved sort may grow an exception report in render
([topological_sort.rs](https://github.com/andremichelle/openDAW/blob/4a9f183f63dfc7ad049b5f24eca6081205a7c61b/crates/engine-env/src/topological_sort.rs#L104-L109)).
Transport reserves 16 blocks but can push every split of a short looping transport, and the event path
collects into a retained vector before truncating to 256. “Usually enough reserve” is neither a bound
nor typed admission.

Automation is ten PPQN pulses rather than V2 absolute-sample segments
([ppqn.rs](https://github.com/andremichelle/openDAW/blob/4a9f183f63dfc7ad049b5f24eca6081205a7c61b/crates/dsp/src/ppqn.rs#L9-L21)).
The inspected effect/linker contracts expose no fixed effect latency, so they cannot provide V2's
exact send/sidechain PDC. Decoded samples reside whole in engine memory; project bundles ZIP samples
and soundfonts into OPFS, which is convenient but violates duration-independent engine memory.

The distinctive browser architecture is one host Wasm plus 29 trusted PIC side modules sharing
memory/table, each reserving a 256 KiB stack and all compiled at startup
([linker](https://github.com/andremichelle/openDAW/blob/4a9f183f63dfc7ad049b5f24eca6081205a7c61b/packages/studio/core-wasm/src/device-linker.ts#L120-L165)).
It is modular packaging, not isolation, and implies about 7.25 MiB of device stacks before other
state. It should not weaken V2's opaque third-party worker boundary.

Two positive evidence patterns survive scrutiny. The offline worker invokes the same engine render
and modules, and differential tests require same-state bit identity/restoration. Separately, the
engineering diary reports that simply enabling `simd128` produced no gain; profiling identified
specific vocoder lanes before specialization
([diary](https://github.com/andremichelle/openDAW/blob/4a9f183f63dfc7ad049b5f24eca6081205a7c61b/plans/wasm-audio/diary.md#L208-L219)).
That measure-first method reinforces V2's existing performance-pass rules; it does not replace AoSoA.

## Decision

- **Adopt as evidence:** same-render differential fixtures with real asset PCM; retain measure-first,
  workload-pinned SIMD specialization.
- **Investigate through existing continuity ownership:** reuse compatible device/effect state across
  off-render replacement, without shared mutable nodes. This overlaps #252's state-continuity plan
  and must not create a duplicate implementation issue.
- **Preserve:** immutable prepared ownership, exact PDC, absolute-sample control, streamed sources,
  strict sessions, AoSoA cohorts and narrow ABI.
- **Reject:** lazy render reconciliation, reserve-then-grow/truncate, coarse universal PPQN,
  latency-blind effects, whole-source residency, eager all-plugin compilation and shared-memory side
  modules as third-party isolation.

## Objective gates

1. Worst legal transport loops/events remain bounded with zero allocation, truncation or unbounded
   iteration; overload rejects before ack.
2. Every effect declares fixed latency/tail and passes route/send/sidechain/bypass/replacement impulse
   PDC gates.
3. Real-source differential fixtures run in required CI; long-source memory stays under fixed rings.
4. Any modular-Wasm experiment measures cold load, memory/stacks/table and per-block overhead on
   constrained targets; opaque modules stay isolated and load on demand.
5. No specialization lands without a frozen workload, semantic-identity gate and named measured win.

## Limitation

Useful local tests do not appear as required public CI at the inspected pin. The modular-Wasm design
fits one integrated app and does not establish portable plugin isolation or lower cost than V2.

