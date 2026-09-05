# Inspect a complete current scalar Wasm object population

**This bounded child owns Wasm inspection from parent #404. Keep #404 OPEN for its separate workspace-policy populations/queries and this child's delivery.** The actual remote #404 remains OPEN and assigns both outcomes; no exemption or closure is justified. The Wasm consumer/discovery fix is independently useful and bounded. Root is numbering this scope before implementation; Astra must approve the numbered synchronized spec before assignment.

## Existing defect and actual scope

check-wasm-realtime-atomics.sh builds scalar engine/source/target-smoke, extracts matching rlibs, discovers .o members, checks observation-symbol OR source-ObservationSlot presence and bans the existing `atomic\.` opcode pattern. The conditional objdump|rg pipeline accepts decoder failure as no atomic match. #399 recorded four bad-magic errors among six counted objects with exit0. #419 independently demonstrated a successful three-object NON-LTO inspection, then corrected a second mistake: mapfile/process-substitution status did not establish find/sort completion. Preserve both historical records; neither repaired the production gate.

The current required scalar job first builds an 18-package release scalar closure in target/ci/wasm-scalar, then calls this gate with that same directory. Cargo release fat LTO may produce LLVM bitcode members, which wasm-objdump cannot decode. Merely adding strict decoder status would honestly fail CI but would not deliver a working inspection. Inspectable object production and complete checked consumption are one coherent minimum outcome; no new browser artifact, linker harness or target matrix is needed.

## Necessary explicit child amendment before coding

Approve the following narrow execution ruling in the numbered child rather than treating it as implied by the current “no artifact redesign” text:

- Keep the existing optional target-directory argument and default. Treat it as the inspection build location ROOT; allocate one fresh, gate-owned temporary child target beneath it for this invocation. Set CARGO_PROFILE_RELEASE_LTO=false explicitly on the same scalar `cargo build --locked --release --target wasm32-unknown-unknown -p engine -p source -p target-smoke`, retaining `RUSTFLAGS=-C target-feature=-simd128`. This is a named inspection profile, not replacement or qualification of the shipped fat-LTO/SIMD artifact.
- Fresh ownership is essential: do not enumerate every historical hash in the CI scalar directory and call that the current population, and do not delete its shared cache. Remove only the gate-created child/scratch if cleanup is needed, preserving failure diagnostics. This modest three-package closure was already demonstrated in #399/#419; no second generic build framework.
- Update the required CI comment which presently promises reuse of incremental scalar rlibs. It becomes inaccurate when inspecting a dedicated non-LTO child. Add the focused hermetic suite immediately before/after this existing checker call in the SAME step/job; preserve triggers, routes and verdict expectations. Keep the existing 18-package scalar build and all supported artifact gates unchanged.

This amendment changes the gate's inspection profile/internal target layout deliberately while preserving CLI and actual named scope. Root/Astra must approve/synchronize the numbered child before assignment. If root prefers another population-selection strategy, freeze it before code; do not silently scan stale mixed-profile objects.

## Exact allowed paths and dependencies

Child owns scripts/check-wasm-realtime-atomics.sh, new scripts/test-wasm-realtime-atomics.sh, its numbered evidence and parent #404 accounting; qualification.yml only for the one existing-step fixture call and corrected profile/cache comment. No check-workspace-policy/test-workspace-policy edits, no Rust/Cargo manifest, SDK/browser/artifact pin, new opcode policy or framework. Existing shared helpers may be reused but no helper edit is necessary: bounded local explicit captures suffice. Source the actual physical library path if used; preserve current repository-root/CLI assumptions.

Required prerequisite is merged shared foundation #400 (already satisfied). There is no semantic dependency on #410/#411/#412 and no overlap in their checkers/tests. #412 alone may edit qualification.yml for its research suite; serialize those exact workflow edits/checkpoints, or schedule this child after #412 if root prefers one tooling stream. It need not wait for all workspace fixes in #404. Never compete with a feature benchmark quiet window. WIP remains one feature plus bounded independent tooling.

## Frozen complete producer/consumer contract

1. Check tool availability and rustc cfg production before parsing. Preserve pointer-width-atomic-support presence and target_feature=atomics absence policy; use explicit statuses (0 match/1 absence/>1 error), not quiet/pipeline ambiguity. Use the same explicit scalar flags for the cfg evidence whose source profile is being described; this adds no new supported-target claim. Cargo failure stops before archive acceptance, even if partial files exist.
2. Enumerate engine/source/target_smoke archives COMPLETELY with separately checked find/sort (or equivalent checked glob population), exactly one archive per family in the fresh child. Missing a family is failure, not an ignored unmatched glob. Any partial producer error fails regardless of discovered count. Capture ar member listing/extraction statuses before scanning; require each family's nonempty object membership. Reject duplicate object-member names rather than silently overwriting them during extraction. Decode the exact member population once; reconcile complete discovered/extracted identities, avoiding hidden omissions. No mapfile/process-substitution producer-status claim.
3. Decode EVERY object into a captured file/output with separate stderr; require decoder status0 before scanning. Empty or plausible-clean partial stdout with nonzero decoder status fails. Scan the complete decoded text with the unchanged `atomic\.` regex: status0 is policy violation,1 clean,>=2 execution failure. Do not impose new instruction/disassembler-output grammar; a successful valid empty decoded body is not newly banned. Require nonempty total object set before any success report.
4. Preserve observation-object OR source-fallback logic precisely. Search the checked complete object set using existing binary-symbol pattern/options; all actually invoked searches must finish successfully, even after an earlier match. If at least one object match exists and no errors occurred, observation passes without requiring source fallback. Only complete clean object no-match permits the exact source ObservationSlot search. Its match passes, no-match fails observation policy, execution error fails distinctly. Neither object search errors nor decoder errors can be masked by a successful fallback.
5. Success names scalar NON-LTO engine/source/target_smoke inspection and actual object count. Do not claim shipped AudioWorklet/fat-LTO qualification or removal of atomics from every workspace dependency. Preserve command errors/status/identities needed for the audit, without a new schema or byte-pinned prose ledger.

## Smallest hermetic suite

Stub the existing rustc/cargo/ar/wasm-objdump/search tools around one tiny disposable fake target/archive/object fixture. Keep the checker itself real. No Cargo or real timing in mutation tests. Saved real tools handle unrelated operations; targeted failures must reach their intended producer and assertions must name the operation/status rather than accept arbitrary failure.

Positive base: three archive families, complete objects, clean decoded text, object observation match. Also positive clean object no-match plus source ObservationSlot; no-source-needed when object match exists. Include a stale unrelated/fat-LTO archive outside the owned child to demonstrate current-population isolation without deleting it.

Directed reds: cfg execution error with plausible required content; cfg required absence/forbidden feature; failed Cargo after partial target output; missing each archive family; archive-discovery/sort error-only and full-looking partial output/error; ar list/extract failure including partial members; empty required object set and missing/duplicate member reconciliation; object-discovery partial/error; disassembler empty/error and valid-looking clean partial/error; actual atomic opcode; opcode scan execution error; observation object scan error despite present source fallback; source fallback read/search error; complete observation no-match in both alternatives. Preserve source fallback as optional when not consulted. Representative parameterized cases are enough, not a new target matrix.

Run at least two actual same-assertion counter-mutants: restore the old conditional decoder/scan false-pass and swallow a partial complete-looking population producer error. The original focused assertion must reject each at its named unexpected-success/semantic assertion, not syntax/tool failure; record the executed result. Additional shared-helper mutants are not required when unchanged helpers are reused.

## Delivery and retained parent accounting

Real gate once with the actual available toolchain at final qualification, after focused PASS, must produce inspectable non-LTO objects and pass ALL checked stages. Preserve original failure evidence and the new real result. No shipped browser artifact rebuild or benchmark is required by this shell-only change. Bash syntax/diff, focused suite, existing relevant policy and unchanged-count workspace delivery gate remain; actual PR Astra review plus required CI before merge/closure. If local tools are unavailable, required CI must provide the real result and local limitation is recorded.

Luna one coherent implementation attempt, Sol only after Astra FAIL up to two revisions, third failure hard stop/rescope. Root owns numbered sync, checkpoints, CI and closure. Child closes the ninth original Wasm object find loop plus this assigned consumer/status defect. Parent #404 remains open for all its original five workspace find populations, tracked-path pipelines, predicates/parsers/filters/optional-result semantics and final accounting. #306/#349 remain open through every other assigned child. No automatic broader framework, release matrix or new qualification issue is authorized.

Read-only local/remote #404, checker/CI, prior #399/#419 ruling/evidence inspection. No tests, Cargo, source/Git/GitHub mutation or timing performed.

## Root execution ruling and queue

Root approves the explicit fresh owned scalar non-LTO inspection profile and target-child strategy as the minimum correction, retaining the original CLI and named three-family scope. This is authorized repair of the audit defect, not a shipped artifact redesign. Queue this independent child after #412 to serialize the tooling stream and workflow edits; #410 → #411 → #412 retain their order. No implementation has started. Freeze the actual merged base before assigning Luna.

## Numbered identity

This is issue #427, created and synchronized before implementation. Parent #404 retains its workspace populations, tracked paths, predicates, parsers and final accounting. #427 owns the ninth original object-discovery loop and its inspection consumer contract.
