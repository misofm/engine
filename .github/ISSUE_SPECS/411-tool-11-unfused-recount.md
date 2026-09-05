# Complete the unfused seal's searches before accepting its recount

Queued child of #402; grandparents #306/#349 TOOL-11. This is issue #411; the matching numbered local spec and GitHub body are synchronized before implementation. Depends on merged #400; depends on merged #410 and queues after that realtime/lane child to freeze any shared helper changes. The seal's numerical contract does not depend on a new runtime feature.

## Closable outcome and allowed paths

The existing unfused seal can certify its exact exemption registry only after complete successful discovery and reads. Only `scripts/check-unfused-seal.sh` including its existing embedded --self-test, minimal `scripts/lib/gate.sh` / `scripts/test-gate-lib.sh` extensions, and numbered spec/evidence. No new standalone seal framework, registry, Rust/codegen, artifacts or workflow. qualification.yml already runs this real checker and its --self-test; preserve both. Shared helper tests already have the required workspace-suite entry point.

## Frozen semantics

Preserve both dispatch files and exact positive `(self * b) + c` spellings after the existing comment-stripping logic; retain the fma-body cfg/target prohibition. Capture source/comment-strip/body-extraction/search statuses before interpreting output. A missing required dispatch file fails; successfully absent forbidden configuration passes.

Preserve the embedded two-file exemption registry, exact per-file counts, total expected_fused_call_count=8, registered-file existence, and six-line exemption-marker requirement. These are audit exemptions, not production FMA permission. Do not lower counts or broaden paths to get green. `count_calls` counts occurrences after comment stripping, not lines: zero is legitimate only for an otherwise applicable zero-call population; registered files must still match their existing positive counts. Distinguish successful no-match from read/rg/awk/count failure.

There are two whole-tree candidate producers: initial registration validation and later total recount. Each must complete successfully over required crates/hosts/tools/sidecars even if partial results already contain all eight allowed calls. It is permissible to reuse one checked complete candidate list while preserving its current comment/recount semantics. Prose-only candidates can legitimately disappear after comment stripping. An empty candidate list must still fail through the frozen nonzero registry/total requirements; do not turn traversal errors into zero counts. Registered file/name and registry aggregate parsers must be checked before consumer loops. Preserve the retired softfma definition absence check, including failures to read the required source.

## Directed verification

Keep every existing embedded positive/red mutation and real-tree check. Add missing required root with valid registry/dispatch fixtures; producer failure after valid registered filenames (including a partial set whose total would otherwise equal eight); failure of comment stripping/counting after valid output; discovered-file read failure; and explicit scan error on the retired-definition check. Preserve clean zero-call/prose-only positives and exact dispatch/registry/exemption negatives. Tests assert intended diagnostic, explicit unexpected-success refusal and counter-mutation liveness. The original unchecked find/rg shape must fail a new partial-producer test.

No extra fixture corpus: extend the seal's embedded fixture machinery. Real `bash scripts/check-unfused-seal.sh` and `--self-test`, helper tests if changed, bash syntax/diff, retained workspace unchanged-count comparison and required CI are the gates. Do not run disassembly, benchmarks, target matrices or DSP qualification because no arithmetic is changing.

## Shared delivery contract

All command-producing operations must distinguish successful output, successful allowed emptiness and execution failure before any filter/count/consumer can certify a result. Never depend on caller pipefail/errexit, conditional-function behavior, standalone ! assertions or lost process-substitution status. Preserve stderr/status evidence, exact regex/glob/allowlist semantics, physical-script helper sourcing before fixture-root cd, caller options/cwd and existing CLI. Do not filter required roots down to those present or mkdir around a missing input. Minimal common helpers only; no command-runner/parser framework.

Astra scopes and reviews; Luna gets one coherent implementation attempt, Sol at most two retries only after FAIL, then hard stop/rescope. Root owns isolated worktrees, exact-path checkpoints/pushes, status/issue synchronization and merge; pause at coherent green tranche before layering more edits. No active shared-helper edits overlap #406 or another tooling child. Full qualification is immutable and shared-target Cargo is serialized. Astra reviews the actual pushed PR head and required CI must succeed before merge.

Each child closes its named concrete outcome only after upstream evidence and PASS. #402 remains OPEN until all three children account for all six original gates, with no silent producer or policy-coverage omission. #306 and #349 TOOL-11 remain OPEN through their other children. The serialized children are #410 (realtime/lane), #411 (unfused), and #412 (environment/migration/research); #410 follows merged #406. No implementation is authorized merely by publishing this queued brief.
