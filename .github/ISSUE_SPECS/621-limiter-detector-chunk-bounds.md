# Prove bounded limiter detector chunk access

GitHub: https://github.com/misofm/engine/issues/621

Parent: #559 FX2. Coordination: #560. Predecessor: delivered #539/#619/PR #620.

At delivered main `cf9e079cd5ef80d1c7284e9edd0ffcc90b0db335`, `detector_chunk` still walks each detector frame with `L::load(&io[base..])` and stores with `store(&mut peaks[frame * width..])`. The audit's historical “79%” and “about eight bounds comparisons” are not current evidence. This issue first determines from retained production lowering whether a reachable repeated bounds-check residual remains, and only then makes the smallest source change that proves the active input/output windows once before frame iteration. Compiler-eliminated or unmappable checks close this issue as a no-change applicability decision.

The global eight-partial barrier is clear. #539 and #619 delivered through PR #620; required PR run `34214845635` and post-main run `34215512812` succeeded, their five worktrees are removed, and both issues are closed. At this boundary main is clean at `cf9e079c`; 304 numbered local specs map to remote issues with zero missing identities. This issue is the sole active slot. Sol HIGH coordinates documentation/checkpoints/GitHub. Luna HIGH/XHIGH owns conditional implementation. Astra LOW performs every scope, residual, source, artifact and delivery verification. Lane B alone owns any AudioWorklet artifact qualification and pin/lineage changes.

## Smallest closable outcome

Prove whether supported scalar, x86-64-v3 W8 and Wasm SIMD production lowering retains repeated slice-bound work inside the detector frame loop. Reuse the checksum-verified #539 native and Wasm lowering already on main when it contains the needed loop bodies and exact current limiter source identity. Do not recapture merely to obtain different symbol names or prettier excerpts. If retained evidence is insufficient, Astra LOW may authorize one bounded untimed compile/disassembly capture with the pinned toolchain and existing release configuration; no benchmark or timing is allowed.

If a reachable residual survives, change only the detector chunk access shape so each call establishes exact active windows before iteration and the frame loop consumes width-sized chunks. Preserve detector history residency and its single write-back, chunk size/span, all four Annex-2 phase accumulators, tap and phase order, separately rounded multiply/add steps, sample-center term, ordered maxima, and every caller's dual/mono and uniform/per-lane behavior. The change may remove the helper's absolute `chunk` index if callers instead pass an exact chunk window. It must not alter `Lane::load/store`, unsafe code policy, scratch sizes, allocation, public API, DSP/state format, coefficients, gain law, latency/tail, parameter behavior, or recovery.

Exact implementation ownership is limited to:

- `crates/true-peak-limiter/src/lib.rs`, including narrowly focused private tests;
- existing `crates/true-peak-limiter/tests/` files only if an inherited public-path assertion must be extended;
- `.github/ISSUE_SPECS/621-limiter-detector-chunk-bounds.md` and bounded evidence.

Lane B retains exclusive ownership of `hosts/host-web/**` artifact output, pin and qualification surfaces. No dependency, manifest, lockfile, lane/math kernel, workflow, generic harness, benchmark, timing capture, or listening campaign change belongs here.

## Objective gates

Before implementation, Astra LOW must PASS the scope and independently map any residual from retained evidence. The residual proof must distinguish loop-internal bounds work from entry validation, chunk/tail control, detector arithmetic, stack probing, and caller checks. It must identify actual supported scalar and W8 paths; Wasm evidence is a portability constraint and may not substitute for the native premise. No source edit is authorized when the residual is absent or unprovable.

A conditional Luna attempt must provide a compact old-shape test oracle under `cfg(test)` or an equally direct existing oracle that compares detector peak bits and complete twelve-word history after nonzero chunk offsets, full and short tail spans, and every supported lane width. It must show the active peak prefix is populated and no caller reads outside that prefix. Avoid a new fixture framework and do not test an implementation detail without a wrong-result control. Existing full PCM/state, partition, mono-collapse, restore, nonfinite, allocation/free and corpus gates remain authoritative.

After source change, capture one comparable untimed lowering record and require the repeated loop-internal bounds mechanism identified at baseline to be absent or reduced to the proven call-entry/window checks. Do not claim a cycle or percentage improvement. Run focused detector/oracle tests in debug and release, the complete limiter suite, strict affected Clippy/fmt, diff hygiene, realtime/lane/workspace/effect-runtime policies, native x86-64-v3 lowering, supported Wasm scalar/SIMD builds and the immutable corpus. Freeze the accepted source before any artifact decision.

Astra LOW adversarially reviews each implementation attempt. Maximum three implementation attempts; do not weaken the gate or disguise a fourth pass. If accepted source changes the ordinary six-file AudioWorklet output, lane B creates or uses a separate bounded qualification issue, proves ABI/resources/SDK/three-browser identity, and alone promotes the pin/lineage after Astra LOW PASS. Exact-head/current-main Astra LOW review, required PR checks, guarded merge-parent verification, post-main qualification, GitHub synchronization and clean delivered-worktree removal remain mandatory.

## DSP and research authority

This is access-only. Preserve the delivered limiter equations and update rules recorded in `.github/ISSUE_SPECS/539-stationary-limiter-dispatch.md`, the issue #90 wave-2 amendment in `.github/ISSUE_SPECS/BRIEFS/016-true-peak-limiter.md`, `.github/ISSUE_SPECS/090-audit-miso-engine-true-peak-limiter.md`, `docs/EFFECT_CONTRACT_V1.md`, and the current source. ITU-R BS.1770-5 remains authority only for the frozen Annex-2 detector coefficients/estimator; the existing repository research record covers gain separation, state, numerical behavior and listening evidence. This issue changes no sound-quality claim and schedules no new listening.

## Decision record

### 2026-09-08 — Astra LOW scope PASS; one current-source capture authorized

Astra LOW passed the scope at clean pushed head `8f2b03ca1c14334facc6c584955715659f8037c3`. The retained #539 lowering establishes only a historical residual: scalar `process-core-scalar.ll:752-758`, W8 dual `process-core-w8.ll:761-767`, and W8 mono `outlined-07.ll:611-617` each contain slice-start and remaining-length checks revisited by the detector-loop backedge. Those checks are distinct from loop termination and detector arithmetic, but they do not prove the audit's historical “eight comparisons” wording.

The retained captures identify source commit `615787e92a209d7146110f7920bb9c09d7f93a3d`, limiter source hash beginning `08ea2bfa`, and older dependency provenance. They therefore cannot prove a residual in delivered main `cf9e079cd5ef80d1c7284e9edd0ffcc90b0db335` and cannot support a no-change closure. Astra LOW authorizes exactly one bounded, untimed current-source compile/disassembly capture using the pinned toolchain and existing release configuration. It must preserve complete output and provenance, cover supported scalar and W8 detector callers plus the required Wasm portability evidence, and separately map loop-internal bounds branches/backedges from entry, chunk and tail checks. No source edit, benchmark or timing is authorized until Astra LOW reviews that capture and records a current residual PASS.
