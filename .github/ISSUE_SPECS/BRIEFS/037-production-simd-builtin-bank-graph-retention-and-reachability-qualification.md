# Sol implementation brief — issue 037 production SIMD builtin bank graph retention and reachability qualification

## Decision and attempt budget

**READY FOR TERRA ATTEMPT 1.** Start from the preserved Issue-008 checkpoint `87783c5`. This
rescope permits exactly one Terra implementation/review attempt and at most one bounded Sol
correction/review. A second failure stops. Do not run, repair or authorize any benchmark;
`timed_benchmark_invocations=0` must remain unchanged.

## Accepted input and frozen boundary

Retain without redesign the safe `PreparedTptBankKernelV1`, exact non-FMA recurrence, three FMA
sites, transposed `BuiltinInputBankV1`, `KernelDispatch`, `AoSoaScratch`, graph topology/PDC/
observer rules and Issue-036 coefficient/cutoff contract. The defect is ownership/reachability:
production post-input builtin bindings are scalar and the old audit's bank is a scalar fixture
effect. Fix only that vertical plus its missing seeded/audit proof.

## Required production shape

1. During graph preparation, partition eligible `PostInputBuiltins` nodes in stable track-ID order
   by the already prepared dependency wave and selected backend width. Retain every full bank and
   then stable scalar tails; never pad.
2. A retained bank owns its `BuiltinInputBankV1`, exact member IDs/active mask and fixed AoSoA
   scratch. Gather from the original graph inputs, invoke the bank once per block, scatter to the
   original output buffers, then run observers in unchanged stable order.
3. Preserve graph nodes, wave membership, reduction order, PDC, sample time, fader/mute, matrix and
   routes. Any incompatible wave/backend/cap/factory shape stays scalar or fails transactionally.
4. Include bank, synthetic identity and scratch bytes in the existing exact resource report and
   reject arithmetic/cap failure before a publishable partial plan exists.
5. Retain bounded per-bank process/TPT-call evidence counters in owned state so the audit proves
   the real architecture token ran. Do not use global atomics, feature probes, logs or I/O.

## Frozen deterministic suite

Run counts `1,2,3,4,5,7,8,9,17` and exactly 100 additional layouts from
`0x000000008a050a08`. Layouts cover width four/eight selection, exact/incompatible waves,
identity positions, scalar tails/fallback, asymmetric L/R parameters/state, enabled/disabled
filters, cap/overflow and repeat determinism. Freeze one transcript hash after independent review.

Base non-FMA output/state is same-target bit-identical for finite-normal/no-sanitation data. FMA
and cross-target use only the already frozen `1e-6 + 2e-5 * abs(scalar)` samplewise bound. Check
signed-zero identity, left/right and cross-track perturbation isolation, lane-local recovery and
per-call counters. Do not change DSP or tolerance to fit an observation.

## Corrected 100,000-render audit

Prepare the 48-kHz/128-frame 12-track mixed production graph with nonidentity asymmetric TPT
builtins, at least one identity position and a real scalar tail/fallback. Before arming, assert
semantic backend, width, exact member IDs, bank/tail counts, fixed resource report and stable
addresses. Between markers render exactly 100,000 callbacks through `PreparedRenderPlan` and prove
the expected nonzero retained builtin-bank process count and exact architecture TPT-call count.
Freeze the PCM/state/counter hash.

While armed, allocation, deallocation, lock, feature detection, log, file/network I/O, syscall,
panic/unwind and structural-mutation counts are each zero; no drop occurs. Detector mutations must
prove each category where the existing audit contract requires it. Disarm before inspection and
destruction. The old scalar fixture bank cannot satisfy this gate.

## Ordered gates

1. Format and focused core/builtin/builtin-compiler/rack/rack-compiler/graph/graph-compiler tests.
2. Exact count set and 100-seed transcript/differential/isolation suite.
3. Corrected release 100,000-render production reachability audit and detector mutations.
4. Rack fixture check plus changed/missing/unlisted/coverage mutations.
5. Named scalar/AVX2/FMA/NEON/Wasm instruction inspection.
6. Locked workspace check/test, warning-denied all-target Clippy and rustdoc.
7. Workspace/realtime/builtin/graph/rack policies and their exact unsafe/feature/ceiling/allocation
   mutations.
8. Native baseline; Android/iOS ARM64; Wasm scalar and simd128 target matrix.
9. Candidate/evidence hash audit and explicit absence of Issue-037 benchmark artifacts.

## Stop conditions

FAIL immediately for changed TPT bits/operations/tolerance, graph/PDC/reduction reorder, shared
lane state, feature detection in render, padded tracks, unaccounted storage, mock-bank evidence in
place of production reachability, fewer/more than 100 seeded layouts, any benchmark launch, or an
attempt beyond the two-attempt budget. A PASS unblocks production-SIMD consumers and Issue 038;
it is not timing, device/browser runtime or release qualification.
