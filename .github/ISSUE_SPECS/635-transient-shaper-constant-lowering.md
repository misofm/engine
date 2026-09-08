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

Attempt 1 stage-1 capture is complete at frozen HEAD
`186e6b3080b040d4a6e7c25b1516762224297381`. The exact required base object
`62045f40048ec230298fe0fd3935da3333f90b83`, direct merge-base check, locked
metadata, toolchain, targets, inspection tools, source hashes, and empty
tracked diff all passed before compilation. The local symbolic `base/main` ref
was absent in this worktree; the required commit object and exact merge-base
were used without substituting another ref.

The three planned commands ran once and returned status 0: one native release
capture containing scalar and AVX2 W8 monomorphizations, one Wasm scalar
capture with `-C target-feature=-simd128`, and one Wasm simd128 W4 capture with
`-C target-feature=+simd128`. They emitted assembly and LLVM IR into isolated
`/tmp/issue635-transient-*` directories. No audio, timing, benchmark, retry,
installation, source/test/manifest/lock/generated-resource edit, or payload
promotion occurred. Raw stdout, stderr, and status are retained under
`artifacts/issue635-transient-shaper-constant-lowering/`; only selected
excerpts, hashes, and the manifest are retained in the repository.

The caller map is attributable in the emitted LLVM:

- native scalar `PreparedNativeEffect::process` calls `Shaper<f32, 1>::process_block`;
- native AVX2 W8 `PreparedTransientShaperBank::process_bank` calls
  `Shaper<wide::f32x8, 8>::process_block`, and its mono entry is present;
- Wasm scalar `PreparedNativeEffect::process` calls `Shaper<f32, 1>::process_block`;
- Wasm simd128 W4 `PreparedTransientShaperBank::process_bank` calls
  `Shaper<wide::f32x4, 4>::process_block`, and its mono entry is present.

Each selected body contains both the ramping prefix and stationary suffix
loop. Native W8 uses `.LBB5_90`/`.LBB5_56` for the ramping frame path and
`.LBB5_93` for the stationary frame path, with explicit backedges. Native
scalar uses `.LBB6_38` and `.LBB6_51` in the ramping path and `.LBB6_148` in
the stationary path. Wasm scalar uses `.LBB6_5` and `.LBB6_32`; Wasm simd128
W4 uses `.LBB4_5` and `.LBB4_128`. The link-mode specializations are inside
these `process_block` bodies; no caller was inferred from source spelling
alone.

### Candidate disposition

| Candidate | Native scalar | Native AVX2 W8 | Wasm scalar | Wasm simd128 W4 | Residual interpretation |
| --- | --- | --- | --- | --- | --- |
| `FLOOR` | folded scalar memory operands in frame | repeated `vbroadcastss` in both frame loops | repeated `f32.const` in frame loop | repeated `v128.const` in frame loop | Actionable residual candidate in W8 and both Wasm shapes; scalar is folded-load materialization. |
| `DB_PER_OCTAVE` | folded scalar memory operand | repeated `vbroadcastss` | repeated scalar constant operand | repeated vector constant operand | Same residual pattern; no arithmetic change is implied. |
| `+/-CONTRAST_LIMIT_DB` | folded `vminss`/`vmaxss` operands | repeated broadcasts | repeated scalar constants | repeated vector constants | Actionable loop materialization in vector/Wasm bodies. |
| `+/-SHAPE_LIMIT_DB` | folded `vminss`/`vmaxss` operands | repeated broadcasts | repeated scalar constants | repeated vector constants | Actionable loop materialization in vector/Wasm bodies. |
| `OCTAVES_PER_DB` | folded scalar multiply operand | repeated `vbroadcastss` | repeated scalar constant operand | repeated vector constant operand | Actionable loop materialization in vector/Wasm bodies. |
| `0.5` average-link factor | folded scalar multiply operand | repeated broadcast in average-link body | repeated `f32.const` | repeated vector constant | Actionable only in the average-link specialization; dual-mono/maximum do not use it. |
| `zero` identity/clamp value | zeroing idiom or folded scalar zero | repeated zeroing/materialization in frame paths | repeated `f32.const 0` | repeated zero vector/zeroing idiom | Candidate is present in loop bodies, but any change must preserve signed-zero selection. |
| bypass mask | loop-entry materialization before prefix/tail | loop-entry materialization before prefix/tail | loop-entry local | loop-entry vector local | No per-frame residual observed. |
| prepared coefficient lanes | loop-entry loads, then scalar state use | loop-entry vector packing plus stack reloads in frame | loop-entry locals/loads | loop-entry vector locals/loads | State traffic/spill-reload classification, not a constant-splat residual. |

The native scalar backend therefore folds several source constants as scalar
memory operands, while AVX2 W8 and both Wasm shapes retain repeated constant
materialization in actual ramping and stationary frame loops. The evidence
does not establish a projected cycle saving or justify a source rewrite. A
real residual exists for a narrowly bounded constant-hoisting review, subject
to preserving the link specializations, signed-zero identity, state traffic,
and scalar/W4/W8 bit behavior.

### Stage-1 disposition proposed for Astra

Suggested Astra LOW verdict: **ACTIONABLE RESIDUAL, stage-2 amendment
required**. The evidence is sufficient to authorize only a bounded follow-up
scope for loop-invariant constant materialization in
`crates/transient-shaper/src/lib.rs`; it does not authorize implementation in
this stage. Any stage-2 brief must name the exact candidate set and require
post-change lowering for the same four target shapes, with explicit spill/
reload accounting and no timing or projected savings claim. The current
stage-1 tree is evidence/spec-only and has no product change.

### Attempt 1 Astra verdict

Astra LOW returned **FAIL** at exact clean pushed evidence head
`b8cce3c455f11ee79012548f94d86260db398d55`. All six temporary compiler
payload sizes and hashes, the single-run provenance, issue synchronization and
evidence-only path ownership verify. The retained attribution does not yet
support its candidate-wide conclusion:

- `capture-plan.md` records a malformed 63-character `.cargo/config.toml` hash,
  while `preflight.md` incorrectly says all 11 hashes matched;
- physical native excerpt locations are inaccurate, including `.LBB5_90` and
  the cited `.LCPI5_4` broadcast;
- Wasm `0x1.815182p2` is `DB_PER_OCTAVE`, not a math-kernel constant;
- average-link, zero and spill conclusions exceed their precise retained maps.

Attempt 2 may only correct the minimal excerpts, mappings, hash and preflight
wording from the existing verified payloads. It must preserve the original
commands, raw streams, statuses, payload identities and attempt-1 FAIL; narrow
every conclusion to individually proven caller/loop cases; and distinguish FX4
constants from math-kernel constants. No compilation, retry, source/test edit,
audio, timing, artifact work, PR or merge is authorized. Astra LOW must review
the corrected evidence before any stage-2 amendment.
