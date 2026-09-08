# Determine transient-shaper loop-constant materialization

## Status

Final evidence PASS; pending exact-head PR review. This is the evidence-first lane-A child for audit #559 finding FX4, based on delivered main `62045f40048ec230298fe0fd3935da3333f90b83`. It occupies one active issue slot beside disjoint lane-B qualification successor #636; #633 is closed as superseded.

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
| `DB_PER_OCTAVE` | folded `vmulss` operand at `.s:17754` | repeated `vbroadcastss` at `.s:10211` and `.s:11493` | repeated `f32.const` at `.s:149497` | repeated `v128.const` at `.s:3076` and `.s:4286` | Individually mapped in all four cases; no projected saving is claimed. |
| `OCTAVES_PER_DB` | folded `vmulss` operand at `.s:17800` | repeated `vbroadcastss` at `.s:10250` and `.s:11626` | repeated `f32.const` at `.s:149577` | repeated `v128.const` at `.s:3124` and `.s:4334` | Individually mapped in all four cases. |

Only `DB_PER_OCTAVE` and `OCTAVES_PER_DB` are retained: each is independently
mapped in caller-reachable ramping and stationary frame-loop cases for native
scalar, native AVX2 W8, Wasm scalar, and Wasm simd128 W4. Every other candidate
is omitted because the retained payload maps do not prove its identity or
caller/loop behavior. No projected cycle saving or source rewrite is claimed.

### Stage-1 disposition proposed for Astra

Suggested Astra LOW verdict: **FINAL CORRECTED EVIDENCE READY FOR REVIEW; no
source implementation authorized**. The retained tranche contains only the
independently mapped `DB_PER_OCTAVE` and `OCTAVES_PER_DB` caller/loop cases
listed above. Astra must decide applicability from those exact maps; this
record authorizes no stage-2 or source work. The current tree remains
evidence/spec-only.

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

### Attempt 2 correction record

Attempt 2 was authorized by tracker commit `768b9ffe` at clean pushed HEAD
`f83be00394d2d2786498cfa2c42b779af0b8a3fa`. It performed no compilation,
retry, source/test edit, audio execution, timing, installation, artifact
promotion, PR/GitHub action, commit, or push. It changed only this issue record
and the existing evidence files.

The correction fixes the `.cargo/config.toml` SHA-256 to
`03b0fbd88c069abb0a8fbdca5921ba6a9899298291fe087977b509a29ebb7d0e`, makes
preflight wording candid about the superseded malformed spelling, and replaces
the prior physical excerpt references with checked locations from the same
payload identities. `DB_PER_OCTAVE` is explicitly mapped as the
`0x1.815182p2`/`0x40c0a8c1` constant; nearby math-kernel constants are
excluded. Average-link, zero, bypass, and coefficient spill/reload conclusions
are removed or qualified where the retained maps do not prove
candidate-specific behavior.

The corrected evidence narrows the possible residual to individually mapped
`DB_PER_OCTAVE`, `OCTAVES_PER_DB`, clamp, and selected `FLOOR` occurrences in
the identified caller/loop bodies. Suggested Astra verdict: **CORRECTED
EVIDENCE READY FOR ADVERSARIAL REVIEW; do not authorize stage 2 yet**.

### Attempt 2 Astra verdict and final correction

Astra LOW returned **FAIL** at exact clean pushed correction head
`764ba327ceba0a5070c7aadb55bbf980216339e4`. Integrity, unchanged raw
captures and payload identities, the corrected configuration hash, issue
synchronization and narrow path ownership pass. Three pool-to-source mappings
remain wrong:

- native W8 `.LCPI5_3` is `0x1e3ce508`; `FLOOR` (`0x322bcc77`) is
  `.LCPI5_4`, so the 10030/11256 FLOOR annotations are invalid;
- native scalar `.LCPI6_23/.24` are `+/-18`, not `+/-24`;
- native scalar `.LCPI6_26/.27` are `-126/+127` math-lowering range
  constants, not FX4 clamps.

Attempt 3 is the final #635 correction. It may only repair or remove those
mappings from the existing verified payloads, exclude math constants, narrow
the residual to the independently established `DB_PER_OCTAVE` and
`OCTAVES_PER_DB` cases, refresh retained-evidence hashes, and preserve both
prior FAIL verdicts plus all original capture bytes and identities. It may not
compile, retry, edit product/tests/dependencies, run audio or timing, perform
artifact work, open a PR, or merge.

Regardless of the attempt-3 verdict, #635 owns no source implementation. On
PASS it closes as the evidence/applicability slice and a separately numbered
successor may brief the two octave-conversion constants. On FAIL it reaches
the hard stop and must be respecified without weakening the evidence gates.

### Attempt 3 final correction record

Attempt 3 was authorized by tracker commit `7bd282f1` at exact clean pushed
HEAD `202f75b321a66609a5f4360e9f0db9083b58954d`. It performed no compilation,
retry, source/test/dependency edit, audio execution, timing, installation,
artifact work, GitHub action, commit, or push. It changed only this issue
record and the existing issue-635 evidence files, preserving both prior FAIL
records, all original raw streams/statuses, and all payload identities.

The retained native W8 excerpts now identify `.LCPI5_3` as `0x1e3ce508` and
`FLOOR` as `.LCPI5_4` (`0x322bcc77`), while omitting both from the candidate
residual. The native scalar excerpts identify `.LCPI6_23/.24` as `+/-18` and
`.LCPI6_26/.27` as `-126/+127` math-lowering range constants; those uncertain
or non-candidate mappings are omitted. The candidate table, selected excerpts,
and conclusion retain only independently proven `DB_PER_OCTAVE` and
`OCTAVES_PER_DB` cases across the four requested targets. Evidence-integrity
hashes were refreshed after these documentation-only changes.

Validation completed without a compiler invocation: physical excerpt locations
were checked against the existing payload bytes, `sha256sum -c` passed for the
retained evidence manifest, `git diff --check` passed, and no product/source,
test, dependency, generated payload, or full compiler-output path was changed.
Suggested final Astra verdict: **FINAL CORRECTED EVIDENCE READY FOR REVIEW; no
source implementation authorized**.

### Attempt 3 Astra verdict

Astra LOW returned **PASS** at exact clean pushed head
`b2a5e9577c34e41ed29c24002a8914d88e78fef1`. The independent review verified
the retained native and Wasm instruction locations, both constant identities,
caller/loop attribution, evidence integrity, unchanged raw captures, preserved
attempt-1/2 FAIL records, exact GitHub synchronization, and absence of source
change or recapture.

The definitive disposition is **ACTIONABLE RESIDUAL**, limited to
`DB_PER_OCTAVE` and `OCTAVES_PER_DB`. #635 may close as the delivered evidence/
applicability slice after exact-head review, required CI and post-main delivery.
It establishes no speedup and authorizes no source implementation. A separately
numbered successor must own any transient-shaper source work under fresh Luna
XHIGH implementation and Astra LOW scope/source reviews.

### Merge and post-main qualification disposition

PR #637 merged the reviewed evidence head `72b3908cc9b3bfbe7fc7410abf5c22adc63c573f`
to main as `d47b62ba0dfcaf5c1525510aaa7789fa8e0acb94`, with the expected previous-main
and reviewed-head parents. Required PR qualification run `34249662644` passed.

Post-main qualification run `34250520726` did not pass. Attempt 1 failed because
the lint/policy job reached `cargo tree --locked --offline --target all` without
the locked `wasi v0.11.1+wasi-snapshot-preview1` package in its runner cache, and
because an AudioWorklet consumer received an intermediary artifact-service HTTP
403. Astra LOW classified both as infrastructure failures and authorized exactly
one failed-job retry. The CLI reported that the run could not be rerun while the
Actions API created attempt 2 at the unchanged merge; both observations are
preserved. Attempt 2 cleared the artifact download and reproduced the same
offline `wasi` cache miss, so the aggregate verdict failed again. No third retry
or dispatch is authorized.

#635 is reopened and remains pending delivery. Issue #638 owns the bounded locked-
dependency preparation needed before the offline audit. Its successful
descendant-main qualification must verify unchanged #635 evidence ancestry; it
does not retroactively make failed run `34250520726` successful. #636 is queued
and has no execution authority while #635/#638 occupy the two active slots.
