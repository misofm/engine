# 268 Engine architecture synthesis: preserve the runtime, close fan delivery, and reject speculative compiler surface

One-line summary: Thirteen source-backed comparisons found no public system that combines V2's
built-in native/Wasm bit identity with fan-runnable session closure; preserve the runtime, correct one
stale numeric-policy document, finish the existing content-addressed delivery train, and defer all
new compiler/adapter surfaces until real demand exists.

**This is a research/tracker decision record only.** It authorizes no cross-cutting implementation
branch, runtime rewrite, package format, compiler, adapter or issue mutation. Each implementation
requires its own smallest-closable Sol-approved brief and objective gates.

**Authority: GitHub issue #268.** This local file mirrors its final synthesis and closure record.

## Authority and method

- Engine V2 implementation baseline: [`90c3b9a`](https://github.com/misofm/engine-v2/tree/90c3b9a598f1244938d9cdcce04c4a4641c6b758), including completed #240.
- Source/official-document comparison records: #255–#267.
- Existing ownership checked against #239, #241, #243–#246, #252 and the current open-issue
  inventory.
- Four Sol xhigh research tracks inspected pinned public source and current V2. A fresh fifth Sol
  xhigh agent independently challenged and condensed their conclusions.
- No legacy/V1 Miso source was inspected. No cross-project benchmark ran.

## Executive verdict

Within the audited public projects, **none publicly establishes both**:

1. Engine V2's bit-exact built-in arithmetic across supported native and browser-Wasm legs; and
2. fan-runnable session closure through a small semantic document, engine-owned effects,
   content-addressed shared stems, verified lossless decode and no recipient DAW/plugin/license
   dependency.

Onda comes closest to one semantic program feeding native and Wasm, but its published parity policy
does not establish V2's transcendental bit contract and its project buffers are whole assets
([validator](https://github.com/onda-lang/onda/blob/60958ab177a3cf37407fe28c54778c4776b45fc8/crates/onda_mir/src/validate.rs#L30-L167),
[parity](https://github.com/onda-lang/onda/blob/60958ab177a3cf37407fe28c54778c4776b45fc8/packages/onda_binaryen_web/scripts/verify-backend-parity.mjs#L95-L208)).
Cmajor, RNBO and Faust provide cross-target toolchains but no public equivalent exact numeric/state
contract; Cmajor and Faust expose fast-math-capable paths
([Cmajor](https://github.com/cmajor-lang/cmajor/blob/024a208515f15e43271d9b2ea85ee22a2233384b/include/cmajor/API/cmaj_BuildSettings.h#L32-L45),
[Faust](https://github.com/grame-cncm/faust/blob/ee0013becc4ed9717517c45ce821e0f0459f1350/compiler/generator/llvm/llvm_code_container.cpp#L72-L91)).

DAWproject, Ableton and Logic provide interchange or consolidation, not portable renderer closure:
compatible applications, plugins, assets, versions or flattening remain necessary
([DAWproject](https://github.com/bitwig/dawproject/blob/ee4dcdde75940f30e14e55401a26955a58b8322b/README.md#L27-L71),
[Ableton](https://help.ableton.com/hc/en-us/articles/209775645-Collect-All-and-Save),
[Logic](https://support.apple.com/guide/logicpro/lgcpce09b9d8/mac)).

This verdict is limited to the audited public evidence. It is not proof about every private or
unexamined system.

## Preserve

1. One exclusively owned, preallocated, structurally immutable plan; one render thread; no callback
   allocation, free, lock, syscall, I/O, logging or structural reconciliation.
2. Target-pinned scalar/W4/W8 execution and unfused `(a * b) + c` with two roundings everywhere. The
   [lane contract](https://github.com/misofm/engine-v2/blob/90c3b9a598f1244938d9cdcce04c4a4641c6b758/crates/miso-engine-lane/src/lib.rs#L19-L23)
   remains authoritative.
3. Stable-ID topology/reductions, exact integer PDC, fixed effect latency, latency-preserving bypass,
   AoSoA cohorts and scalar tails.
4. Strict canonical TOML, transactional compilation, absolute-sample events, validate-then-admit
   queues and the rule that an acknowledgement can never precede a later drop.
5. Duration-independent source rings and a storage-blind engine.
6. Existing on-demand canonical graph evidence rather than physical plan details in a stable ABI.
   V2 already reports nodes, edges, schedule, reductions, PDC, timing, buffers and resources and
   hashes the result
   ([compile entry](https://github.com/misofm/engine-v2/blob/90c3b9a598f1244938d9cdcce04c4a4641c6b758/crates/miso-engine-graph-compiler/src/compile.rs#L20-L60),
   [canonical rows](https://github.com/misofm/engine-v2/blob/90c3b9a598f1244938d9cdcce04c4a4641c6b758/crates/miso-engine-graph-compiler/src/canonical.rs#L136-L309)).
7. Measure-first optimization under existing workload/budget owners. No comparison supplied a
   comparable benchmark that changes performance priorities.

## Dependency-ordered immediate plan

Keep only one launch-critical implementation issue active. The documentation-only correction may
proceed independently because it changes no runtime or fixture identity.

### 0. New evidence-only issue: correct stale FMA policy

**Proposed title:** `Correct retired fused-FMA wording in REALTIME_DEPENDENCY_POLICY after #163 phase 2`

**Ownership:** a new bounded documentation/policy issue touching only
`docs/REALTIME_DEPENDENCY_POLICY.md` and a focused text-consistency gate.

The current policy still says `softfma.rs` contains Wasm software-FMA intrinsics and that
`Lane::fma` uses native hardware fusion plus exact Wasm emulation
([unsafe paragraph](https://github.com/misofm/engine-v2/blob/90c3b9a598f1244938d9cdcce04c4a4641c6b758/docs/REALTIME_DEPENDENCY_POLICY.md#L46-L54),
[numeric paragraph](https://github.com/misofm/engine-v2/blob/90c3b9a598f1244938d9cdcce04c4a4641c6b758/docs/REALTIME_DEPENDENCY_POLICY.md#L146-L154),
[relaxed-SIMD paragraph](https://github.com/misofm/engine-v2/blob/90c3b9a598f1244938d9cdcce04c4a4641c6b758/docs/REALTIME_DEPENDENCY_POLICY.md#L287-L293)).
That contradicts current source and executable gates.

Smallest slice:

- state that fusion is retired and `Lane::fma` is unfused on every backend;
- explain that `softfma.rs` retains a historical filename but now owns only the independent unfused
  oracle and MXCSR helpers;
- correct the relaxed-SIMD example without rewriting clearly historical evidence.

Gates:

- no present-tense policy paragraph claims hardware/software fused execution;
- existing unfused seal, lane policy, realtime policy and native/Wasm bit gates remain green;
- a mutation restoring the stale fused claim fails a focused consistency check.

Non-goals: arithmetic or unsafe-allowlist changes, names, artifact re-pins or implementation of #172.

### 1. Preserve #252's P0 runtime program

The comparisons found no better runtime architecture. Create #252's smallest successor issues before
implementation, in this order:

1. **Rejected render is not a plan boundary.** Pre-mutation refusal leaves ownership, epoch,
   retirement, clock, handover and candidate unchanged; corrected retry applies once; render remains
   clean. Do not attempt rollback of arbitrarily mutated DSP or redesign the graph.
2. **Render-owner teardown receipt and reserved exceptional handoff.** Normal and early-drop paths
   hand every object to a control thread exactly once with zero render-side free/drop/join/lock/
   syscall. Do not add RCU, general refcounting or worker-pool redesign.
3. **Entry-snapshot effect-control staging.** Consume exactly the producer cursor captured at entry;
   adversarial refill cannot extend one invocation; FIFO and canonical last-wins remain exact for
   scalar/W4/W8. Do not introduce global compiled track/event caps or a second queue architecture.

Compatible effect-state continuity remains #252 item 5 and starts only after the first two items.
openDAW does not create new ownership for it.

### 2. Complete fan-runnable delivery through existing issues

The binding train is:

`#240 complete -> #241 -> {#242, #245} -> #243 -> #244 -> #246`

- **#241** owns canonical-PCM SHA-256 identity, source-schema simplification and canonical/wire
  updates. It does not own the store, decoder, SDK or app.
- **#245** owns FLAC as lossless transport, the pinned client decoder, publisher decode-back proof
  and catalog identity migration. It forbids lossy delivery and playback-time decode.
- **#243** owns SDK boot consumption and the resolver seam; it must not create a second TOML parser or
  storage policy.
- **#244** owns the content-keyed OPFS store, verify-before-play hard ingest, missing-only fetch,
  Worker pump, quota/pins/concurrency and typed storage refusal. Its primary proof remains 8 cold
  fetches for mix A, 2 for mix B sharing six stems, then 0 for pinned A/B switches.
- **#246** owns the user-visible prepare/open-for-sharing experience and real-app/browser evidence,
  including progress, cold/warm timing, throttled underrun proof, attestation and legacy cleanup.

Do not revive superseded #239 container-byte identity, raw-plus-decoded two-tier storage or progressive
trailing verification. Do not invent a portable ZIP/package. The fan-runnable unit is:

`canonical TOML + pinned engine/backend + verified shared canonical-PCM stems + available executable effects`

## Evidence decisions versus runtime decisions

Evidence-only decisions:

- make the FMA wording consistent;
- retain claim-specific provenance, exact native/Wasm digests, chunk/corpus non-vacuity and real
  browser gates without a new generic evidence framework;
- fold any later graph-permutation proof into a concrete nondeterminism defect or #252's independent
  semantic-oracle work;
- do not create a duplicate offline/realtime loop test while the native runner already invokes the
  same C render entry once per quantum
  ([runner loop](https://github.com/misofm/engine-v2/blob/90c3b9a598f1244938d9cdcce04c4a4641c6b758/tools/miso-engine-native-pcm-runner/src/lib.rs#L240-L282)).

Runtime decisions:

- no comparison-derived runtime change is approved;
- runtime correctness stays under #252;
- fan delivery stays under #241/#243/#244/#245/#246;
- measured optimization stays under its existing issue owners.

## Now, later, never

| Candidate | Ruling |
|---|---|
| Compile snapshots | **Never** stabilize physical slots/lowered details. **Later only on demonstrated diagnostic demand:** adapt existing canonical evidence through a capped off-render tool/SDK. |
| Offline/realtime loop parity | **Never as a standalone issue** while both paths call the same C render entry. Reconsider only after a genuinely independent offline loop appears. |
| Semantic effect IR | **Later only after approved programmable-effect scope.** Begin with schema, validator, lineage and one offline native/CoreWasm prototype; never retrofit built-ins or compile whole sessions. |
| Faust authoring | **Later only with measured authoring demand.** Pinned offline generation into audited V2 contracts; never libfaust/JIT, fast-math defaults, generic state or scheduler code in render. |
| DAWproject adapter | **Later**, after #246 and real import/export demand. Hostile-input adapter with deterministic capability/loss and explicit flatten-required records; never runtime authority. |
| Strong-time front end | **Never in render/ABI. Later only as an external compiler** producing capped absolute-sample commands and prepared replacements with canonical same-time order. |
| Sharing preflight | **Now, already owned by #244/#246.** No new package or core issue. |

## Deliberate non-adoptions

1. No whole-session JIT/AOT, mutable compiler AST as interchange, generated-C++ identity, callback VM,
   shred scheduler or render-time compilation/evaluation.
2. No fast math, target-dependent contraction, tolerated transcendental drift, raw memory snapshots or
   build-selected numeric semantics.
3. No dynamic graph reconciliation, reserve-then-grow/truncate queues, acknowledgement-before-drop,
   universal PPQN, fractional PDC, latency-blind effects or reset-on-install.
4. No runtime SIMD selection, shared-memory Wasm side modules as isolation, eager all-plugin
   compilation, speculative multicore render or fixed compiled track/bus ceilings.
5. No whole-stem residency, replay seeking, large PCM cache, path-based runtime identity, per-session
   media copies, temp-file assets or implicit filesystem access.
6. No DAW-shaped runtime package, generic-device bit-equivalence claim, installed plugin/license
   closure or silent bypass. Flattening is explicit, artist-authorized and non-editable.
7. No new optimization work from cross-project code shape alone; frozen V2 workloads/budgets remain
   the only performance authority.

## Comparison closure map

| Issue | Disposition |
|---|---|
| #255 Onda | Narrow semantic-artifact lineage behind real programmable-effect scope; no current work. |
| #256 Cmajor | No action; cache-key inputs support #255's conditional future slice. |
| #257 RNBO | Strict export profile belongs inside #255 if ever needed; no runtime/SDK work now. |
| #258 Csound | Conditional importer only after demand; compile to canonical V2 state off render. |
| #259 ChucK/WebChucK | Reject engine language/VM; possible external authoring compiler only. |
| #260 Faust | Only FMA documentation correction now; Faust authoring remains demand-gated/offline. |
| #261 libsidplayfp-wasm | Retain evidence lesson; no new framework or emulator/cache adoption. |
| #262 DAWproject | Adapter waits for #246 plus demand; #241/#244/#245 stay authoritative. |
| #263 Ableton | Closure UX narrows to #244/#246, not a package/collection issue. |
| #264 Logic | Hard closure and explicit flatten/refusal narrow to #244/#246; no platform package. |
| #265 libsonare | Reject immediate loop-parity issue; preserve ack-before-drop rejection. |
| #266 openDAW | Continuity stays #252; specialization stays existing performance ownership. |
| #267 EffeTune | Reject stable physical compile snapshot; diagnostics/permutation need demand/defect. |

Closing #255–#267 as completed research does not claim implementation PASS for a conditional option
or any existing owner issue.

## Adversarial review

The first synthesis draft did not pass. It was corrected:

1. A duplicate sharing-preflight slice was deleted because #244/#246 already own it.
2. #265's new loop-parity test was rejected after verifying the native runner already calls the same
   C render entry per quantum and pins whole output.
3. #267's compile-snapshot proposal was narrowed after finding richer existing canonical graph
   evidence; stable physical-plan exposure would lock optimizer internals.
4. Superseded #239 container-hash/progressive-feed text was replaced by binding #241/#244/#245 PCM
   identity, verify-before-play, shared store and FLAC-as-transport decisions.
5. Semantic IR, Faust and strong-time syntax moved behind demand because they repair no current defect.
6. The FMA contradiction was expanded from one paragraph to all three present-tense stale sections.
7. Competitor evidence gaps are reported as “not publicly established,” never proof of failure.
8. Evidence changes and runtime changes were separated; only one new immediate issue remains.
9. Cross-project code shape no longer authorizes performance work.

**Verdict: PASS as a dependency-ordered, non-duplicative research tracker.** Runtime architecture is
unchanged by the comparisons. Public numeric-policy consistency remains open until the bounded FMA
documentation successor closes.

