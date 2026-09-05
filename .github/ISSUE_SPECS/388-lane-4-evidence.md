# LANE-4 evidence fix-forward

## Objective

Fix forward the evidence omissions in merged PR #384 without changing production code or rendered
bits. The post-merge Fable audit found the LANE-4 implementation correct but the original
implementer-authored verification and benchmark/codegen claims insufficiently reproducible.

## Scope

- Add `Lane::exp2_int_in_range` to G1 with only contract-valid integer inputs in `[-126, 127]`.
- Add a named scalar/Simd4/Simd8 differential test for `fast_gain_from_db` and `exp2_lane` over
  NaNs, infinities, signed zero, subnormals, and the neighbourhoods of `-127`, `-126`, `126`, and
  `127`.
- Register one unconsumed evidence arm for the console benchmark, run it once without deleting or
  overwriting tracked artifacts, and retain its raw, accepted, and disposition records.
- Retain reproducible probe source and full `llvm-objdump --demangle` output for both callers at
  pre-LANE-4 `9c062318` and merged LANE-4 `2b38ba7f`.
- Correct PR #384 and the implementer-authored #349 LANE-4 note: withdraw the unreproducible
  benchmark statement, remove any claim that the F1 bounds sweep proves bit identity, and identify
  Fable as the verifier for this fix-forward.

Production lane/math implementation, AArch64-specific code, floor accounting, dependencies, and
generated product artifacts are outside scope. If a production change is required, stop.

## Allowed paths

- `.github/ISSUE_SPECS/388-lane-4-evidence.md`
- `crates/lane/tests/g1_op_identity.rs`
- `crates/lane/tests/support/mod.rs`
- `crates/math/tests/m2_lane_identity.rs`
- `scripts/run-console-benchmark.sh`
- `artifacts/issue388-lane4-evidence/**`

## Objective gates

1. `cargo test --locked -p lane --release --test g1_op_identity` passes with
   `exp2_int_in_range` in `ALL_OPS`.
2. `cargo test --locked -p math --features lane --release --test m2_lane_identity` passes and the
   directed test names both callers and compares scalar, Simd4, and Simd8 bits.
3. `cargo test --locked --workspace` has the current `main` pass count plus the new directed test.
4. `scripts/test-console-benchmark.sh` and `scripts/check-realtime-policy.sh` pass.
5. Exactly one admitted invocation of
   `scripts/run-console-benchmark.sh --issue388-lane4-evidence` produces a disposition JSON and
   does not delete or overwrite any tracked artifact.
6. Full before/after disassembly for both non-inlined caller wrappers records commits `9c062318`
   and `2b38ba7f` and visibly removes one `vmaxps` plus one `vminps` per caller.
7. The branch changes no production Rust source. Fable 5.1 verifies and merges the open PR; Codex
   does neither.

## Seen, not done

- The browser qualification `candidateCommit` squash-merge convention and #384's immutable merge
  trailer are recorded by #388 but are not part of its done conditions or this evidence-only
  change.

## Evidence record

- G1 now routes `exp2_int_in_range` through the scalar/Simd4/Simd8 table over every legal integer
  in `[-126, 127]`; its two existing identity tests pass, and the original `exp2_int` operation
  and exactness test remain.
- M2's directed caller test passes over NaN payloads, infinities, signed zero, subnormals, minimum
  normals, and the one-ULP neighbourhoods of `-127`, `-126`, `126`, and `127`.
- Exact workspace tests pass at detached `origin/main`
  (`879269886102664f1c2194ee15b44fab528075c2`) and at committed candidate
  `a55234beafca0222266af4308cabc8a1759c8a63`; the candidate adds exactly the one named M2 test.
- The benchmark runner's new unconsumed arm completed exactly once with disposition
  `PASS/complete`, one warmup and two measured rounds. The uncontrolled-host override is recorded,
  so this is reproducibility evidence and not a performance acceptance claim.
- Full retained `llvm-objdump` output shows both requested callers going from two `vmaxps` plus two
  `vminps` at `9c062318` to one of each at `2b38ba7f`.
- Formatting, full workspace clippy, runner validator/mutation tests, realtime policy, artifact
  validation, and diff hygiene pass. No production source is changed.
- PR #384 and the implementer-authored #349 note are corrected after this evidence checkpoint is
  pushed. The #388 PR remains open for Fable verification and merge; Codex does not review or
  merge it.


## Astra-approved recovery brief — 2026-09-05

# Astra review: PR #396 / #388 at b29135f3

**VERDICT: FAIL for final delivery at exact head `b29135f32ff18d29e08828520407918cceafaad9`; bounded integration/scope-record recovery required.** The implemented identity tests, one-shot benchmark and attached codegen evidence pass this review. Do not rerun the benchmark or alter the production implementation.

## Evidence independently verified

- Release G1: 9 passed, 0 failed. Exp2IntInRange is in ALL_OPS; its direct/random inputs respect the documented integer domain [-126,127], and both Simd4 and Simd8 compare bits against scalar. Existing general Exp2Int coverage remains intact. The operation expands existing tests.
- Release M2: 4 passed, 0 failed. The new named directed test compares both fast_gain_from_db and exp2_lane across scalar/W4/W8 on the promised NaN/infinity/zero/subnormal/normal and boundary-neighbour pool. Refactoring comparison into compare_widths preserves the existing digest assertions. Production Rust is untouched.
- All 46 accepted benchmark records independently pass full record validation and aggregate validation at this head. Artifact hashes/byte counts, candidate a55234be, uncontrolled metadata and disposition agree. Disposition records one invocation, one warmup, two measured rounds and three launches. Historical reported compressor output digests agree; no controlled-speedup claim is made. The consumed run is sufficient and must not be repeated during integration.
- Full before/after objdump files and retained identical probe hash match the evidence README. There are four vmaxps and four vminps total before, two each after, distributed as one removed max/min pair per named Simd8 caller. Both full wrappers, exact source revisions and build/tool identities are attached, not merely hashed excerpts.
- Read current PR #384 and the corrected #349 comment: the prior unverifiable compressor-round1 completion/timings and verifier attribution have been withdrawn, F1/M1 are correctly described as accuracy/bounds rather than identity proof, and the actual new artifacts are linked.

Logs for independent focused reruns: `/tmp/astra-388-g1.log`, `/tmp/astra-388-m2.log`. No timing, source edits or GitHub writes were performed.

## Remaining delivery blockers

1. **Current main integration:** GitHub reports CONFLICTING/DIRTY. Read-only merge-tree against current origin/main shows exactly one conflict, `scripts/run-console-benchmark.sh`; #368 and #388 both extended usage/arm definitions. Resolve by retaining BOTH dedicated arms and both usage entries. Do not choose one side wholesale, remove existing arms, overwrite artifacts, or rerun consumed namespaces. A final current-base workspace comparison and fresh required CI are needed after the normal merge.

2. **Unresolved scope record:** Remote #388's “What was missing and must be fixed forward” item 4 explicitly requests a `candidateCommit` squash-merge convention documented where the matrix generator is described. The local spec and PR unilaterally place it out of scope merely because it is absent from the shorter Done-when checklist; no owner ruling is recorded in the fetched issue comments. Item 5's immutable trailer cannot be repaired in history and has been appropriately noted, but the convention is a remaining requested doc outcome. Root must explicitly resolve this before reporting all #388 work complete: either add the bounded convention paragraph under this issue, or synchronize a stateless successor/deferral in the local and remote scope. Do not invent historical provenance or silently discard the request.

## Minimal Luna recovery brief

Root captures synchronized main AFTER currently ordered prerequisite work and creates a fresh isolated branch/worktree from remote b29135f3. Preserve the existing clean engine-388 worktree/history. Normal-merge main, resolve only the known runner conflict as above, and have root checkpoint the compiling merge before further work. No rebase/force-push or runtime changes.

Before implementation, synchronize the existing #388 spec and issue/PR model roles to the current user workflow (Astra review, Luna one recovery attempt, Sol retry on failure), and settle item 4 explicitly. Suggested bounded convention: `candidateCommit` identifies the immutable final source candidate actually used to build and qualify the artifact; generated qualification evidence may follow it in a separate commit; a squash merge does not retroactively change that identity. Record the original source candidate and final merge/evidence mapping in the issue/PR so the qualified source remains identifiable. This follows the existing successful #369/#371 recovery discipline. Root must approve that exact convention or choose a stateless successor; do not derive it solely from this reviewer suggestion. No old results.json rewrite is needed to document the convention.

Allowed recovery paths: runner conflict resolution; local #388 spec/evidence README; one narrowly identified existing matrix-generator documentation location if root adopts the convention. Preserve all checked-in benchmark/disassembly bytes. Optional retention of the already-produced 88-byte stderr companion is evidence persistence, not permission to recreate timing.

Run release G1 and release M2, current console-validator mutation suite, full current record+aggregate validation of the existing 46-record capture, realtime policy, fmt/diff and applicable clippy. Fresh capture has no floor-group fields, so #368's current floor repricing must not cause a historical-schema rewrite. Complete workspace comparison against captured main: expected +1 named M2 test, G1 count unchanged. No benchmark, probe recapture, browser rebuild or new resource framework is warranted for the runner/test/doc-only integration unless a concrete required gate demonstrates an unrelated artifact issue; report that for bounded scope handling.

Pause at each coherent checkpoint for root commits. Normal-push to the existing PR, refresh its final scope/evidence/roles, then request Astra review at the exact pushed head. Merge only after PASS and required CI, then verify #388 CLOSED and synchronize #349. One Luna recovery pass, Sol only after failure, at most three total attempts; no hidden retries or weakening identity gates.


### Root scope and integration decision

Root adopts the immutable-final-source-candidate convention requested by remote issue item 4. Document it beside the deployment matrix reference in `hosts/host-web/DEPLOYMENT.md`, identifying `qualification/generate-matrix.mjs` / `npm run matrix` and explaining source-candidate versus later evidence/squash commits. This explicitly supersedes the earlier local exclusion. No existing matrix record is rewritten for this documentation. Current roles are Astra scope/review, Luna one recovery pass, Sol retries on failure; earlier verifier/model assignments are historical.

The original remote head b29135f3 is normally merged with post-#371 main `2a18b315067898a94fdc02e8f8b80f07b788ff89` in a separate recovery worktree. The sole conflict is the console runner usage/arm region; preserve both #368 and #388 arms and both usage entries. Original clean worktree/history remain untouched. The single #388 benchmark invocation is already consumed and MUST NOT run again. Final current-base qualification is pending.
