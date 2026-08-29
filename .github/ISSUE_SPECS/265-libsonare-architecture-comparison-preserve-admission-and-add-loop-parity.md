# 265 libsonare architecture comparison: preserve atomic admission and add explicit loop parity

One-line summary: libsonare contributes one useful regression shape—exact offline-helper versus
realtime-quantum-loop parity—but its fixed capacities, insertion-sensitive reductions, fractional
PDC, per-sample epoch traffic and acknowledged-then-dropped commands are regressions from V2.

**This is a completed research and decision record, not implementation authority.**

**Authority: GitHub issue #265.** This local file mirrors its source-backed decision record.

## Authority, pins and method

- Engine V2: [`90c3b9a598f1244938d9cdcce04c4a4641c6b758`](https://github.com/misofm/engine-v2/tree/90c3b9a598f1244938d9cdcce04c4a4641c6b758).
- libsonare: [`8c5c637b06551a647d6f920525d146aecaa0831a`](https://github.com/libraz/libsonare/tree/8c5c637b06551a647d6f920525d146aecaa0831a).
- Static source/CI audit only: no execution, benchmark or legacy Miso access.

## Findings

libsonare compiles a graph and longest-path Q8 latency, but independent ready nodes follow insertion
order and same-port edges retain connection order after a stable port-only sort
([graph.cpp](https://github.com/libraz/libsonare/blob/8c5c637b06551a647d6f920525d146aecaa0831a/src/graph/graph.cpp#L109-L230)).
Its serial renderer mixes per sample and can use fractional delays. V2 instead freezes topo/reduction
order from stable IDs and exact integer-sample PDC, with symbolic lowering equivalence gates.

Its graph publisher is directionally good: a complete binding is prepared off-render and selected at
a block boundary without callback locks/refcount churn
([publisher](https://github.com/libraz/libsonare/blob/8c5c637b06551a647d6f920525d146aecaa0831a/src/rt/rt_publisher.h#L1-L38)).
V2's bounded publication plus retirement reservation and control-side reclamation is already the
stronger ownership-preserving form.

The critical rejection is command admission. `push_command` succeeds when an ingress SPSC accepts a
record, but render later moves it into a fixed pending bank which may evict a previously accepted
future command or drop the new one
([ingress](https://github.com/libraz/libsonare/blob/8c5c637b06551a647d6f920525d146aecaa0831a/src/engine/realtime_engine_commands.cpp#L15-L28),
[downstream drop](https://github.com/libraz/libsonare/blob/8c5c637b06551a647d6f920525d146aecaa0831a/src/engine/realtime_engine_commands.cpp#L111-L170)).
This is precisely V2's prohibited acknowledgement-before-drop defect. V2 validates the complete
batch, admits one whole slot, returns caller ownership on full and advances its ledger only after
success.

libsonare also hard-codes 32 tracks, eight buses and channel limits
([track_mixer.h](https://github.com/libraz/libsonare/blob/8c5c637b06551a647d6f920525d146aecaa0831a/src/engine/track_mixer.h#L72-L80));
per-sample paged-source reads enter/exit an epoch around atomic pointer loads; project JSON ignores
unknown fields and omits allocation counters, enabling persistent-ID reuse. Its README says native
and browser results are identical, while CI explicitly omits golden hashes because float/libm output
is not reproducible across architectures
([CI](https://github.com/libraz/libsonare/blob/8c5c637b06551a647d6f920525d146aecaa0831a/.github/workflows/ci.yml#L80-L84)).

The useful positive is that `render_offline` calls the same block `process` loop and an exact test
compares its two paths
([lifecycle](https://github.com/libraz/libsonare/blob/8c5c637b06551a647d6f920525d146aecaa0831a/src/engine/realtime_engine_lifecycle.cpp#L189-L239)).
V2 already routes offline output through the C render entry; an explicit fixed-target equivalence test
would make this property visible.

## Decision

- **Adopt:** one exact regression rendering the same session/state through callback-style quanta and
  the native offline runner, comparing PCM, clock, PDC/tail, automation and source counters.
- **Preserve:** stable-ID reductions, integer PDC, bounded plan exchange, validate-then-admit atomic
  queues, caller-derived capacities, block SPSC sources, strict TOML, AoSoA banks and the narrow ABI.
- **Reject:** fixed compiled track/bus/event caps, Q8/fractional PDC, per-sample epoch atomics,
  unknown-field tolerance, reusable IDs, broad per-feature C ABI and every two-stage queue where
  success can precede eviction/drop.

## Objective gates

1. Saturate every command staging layer: accepted batches apply exactly once at their sample or are
   explicitly cancelled; they are never counted as later drops.
2. Permuting equivalent graph declarations preserves PCM, PDC, graph digest and reduction order.
3. Source render has zero allocation/free/lock/syscall and per-sample atomic RMW; memory is independent
   of duration.
4. Callback-loop and offline-runner output is bit-equal on one pinned target across effects, sends,
   sidechains, seeks, underruns, bypass and tails.

## Limitation

The parity test is a clarity/evidence improvement, not evidence of a different DSP implementation or
a performance gain. Cross-target identity remains governed by V2's stronger numeric corpus.

