# Determine transient-shaper loop-constant materialization

## Status

Open. This is the evidence-first lane-A child for audit #559 finding FX4, based on delivered main `62045f40048ec230298fe0fd3935da3333f90b83`. It occupies the second active issue slot beside disjoint lane-B #633.

## Problem

`crates/transient-shaper/src/lib.rs` constructs block-invariant lane values at `link` near line 292 and inside `frame` near lines 317–331; the actual frame loops begin in `run` near line 488. Source-level `L::splat` calls do not establish that the compiler emits repeated per-sample broadcasts. No retained current transient-shaper lowering proves either an actionable residual or complete hoisting.

FX4 also names soft-clip and limiter sites. They remain separate future slices and are outside this issue.

## Smallest closable outcome

Capture and disposition the current lowering once, without timing. Inspect supported native scalar/W8 and Wasm scalar/SIMD shapes and trace the actual stationary/ramping and link-mode callers into the loop. Classify each candidate constant as a repeated broadcast, folded memory operand, loop-entry materialization, or spill/reload.

If current lowering has no actionable repeated materialization, close this child as a no-change applicability decision. If it does, Astra LOW must first pass the residual evidence before this issue may add bounded transient-shaper source ownership.

## Stage 1 exact ownership

Stage 1 owns only:

- `.github/ISSUE_SPECS/635-transient-shaper-constant-lowering.md`;
- `artifacts/issue635-transient-shaper-constant-lowering/` containing commands, toolchain/source identities, statuses, hashes, a manifest, and only minimal selected lowering excerpts;
- the #559/#560 tracker record.

Full compiler assembly/LLVM/Wasm payloads must remain outside the repository. Stage 1 owns no product source, tests, manifests, locks, benchmark code, generated resources, SDK/browser files, artifact pins, or qualification outputs. It must not inspect any legacy engine.

## One-shot capture contract

After exact-brief Astra LOW PASS, one Luna HIGH executor may perform one untimed current-release lowering inspection. It must freeze and record the source head, tool versions, target flags and commands before capture; use temporary target/output directories; cover native scalar and AVX2 W8 plus Wasm scalar and simd128; locate actual stationary/ramping and link-mode loop bodies; and preserve minimal excerpts sufficient to count or classify materializations and spills.

This is compilation and static inspection only. Do not execute an audio workload, benchmark, tune flags, retry a successful target shape, install tools, regenerate/pin an AudioWorklet, or quote projected savings. A missing prerequisite or failed target stops the attempt with candid evidence.

## Conditional stage 2

No source implementation is authorized by this brief. If Astra LOW confirms a real residual after stage 1, amend and synchronize this issue before implementation. The bounded source scope may then include only `crates/transient-shaper/src/lib.rs` and narrowly necessary existing oracle tests. Use Luna XHIGH because log/exp lowering and register pressure complicate the apparent hoist.

Any implementation must preserve bit-identical PCM and state, operation order, signed-zero behavior, stationary/ramping partitioning, detector link modes, scalar/W4/W8 equivalence, zero render allocations, supported target builds, and boundary behavior. Comparable post-change lowering must prove the targeted reduction without offsetting spills. No arithmetic substitution, math-kernel change, public API change, fixture expansion, timing, performance claim, or artifact promotion is authorized.

## Objective gates

1. Astra LOW exact-scope PASS on the clean pushed brief before capture.
2. One attributable Luna HIGH capture satisfying the one-shot contract and manifest integrity.
3. Astra LOW adversarial residual verdict: actionable and precisely bounded, or no-change disposition.
4. If no-change, one-file/evidence-only diff hygiene and exact-head/current-main review before PR.
5. If actionable, synchronized stage-2 amendment and a new Astra LOW scope PASS before Luna XHIGH source work.
6. Required PR qualification, guarded exact-head/live-base merge, successful post-main qualification, GitHub closure/body synchronization, tracker update, and clean delivered-worktree removal.

## Attempt record

Attempt 1 is pending. The pre-issue Astra LOW review selected this single-effect evidence-first shape at base `62045f40048ec230298fe0fd3935da3333f90b83`, found no applicable retained current lowering, and confirmed it is disjoint from #633 and lane-B artifact authority.
