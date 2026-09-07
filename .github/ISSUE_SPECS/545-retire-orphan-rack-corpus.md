# Retire the unconsumed rack fixture validator and corpus

Parent #349 TOOL14. Base e5b86cf315487fcc602db420dc1a6121f1ac4837. Sol high source census is preserved with this issue. TOOL14 combines independently useful outcomes and must be split before implementation: this child retires the orphan rack corpus; separate queued children cover the native runner executable boundary and live operator/path instructions. Completing this child alone does not close TOOL14.

## Scope and decision

Delete only crates/graph-compiler/src/bin/rack_fixture.rs and fixtures/rack/v1/**. Current source and the existing test-usefulness audit establish that this binary/corpus validate only each other. Preserve graph_fixture, all production graph/rack code/tests, fixtures/rack/issue038-v1/** and scripts/check-rack-benchmark-fixture.sh byte-for-byte. No other orphan recommendations are authority here.

CP20/#543 found a hex encoder in the file being retired. It must not refactor the retired owner; after this child is delivered, root integrates its deletion into #543 and records a retired-consumer disposition. No stale-count or speedup claim.

## Gates

Use Cargo metadata before and after to verify the sole removed target is rack_fixture and no workspace member changes. Record tracked deleted paths and the absence of live consumers with checked rg scans; preserve historical mentions rather than erase them. Existing graph-compiler tests, workspace policy and diff/format checks must pass. Verify protected issue038 fixture/checker hashes remain identical. No fixture regeneration, timing, browser launch, audio/DSP edit or pin change. Do not add a new scanner framework or a test whose only purpose is to mirror file deletion.

## Execution and delivery

Sol high coordinates Luna high implementation; Sol xhigh verifies the exact candidate/current base. One coherent deletion pass then focused gates and root checkpoint/push. Root owns Git/GitHub writes and required qualification, merge-parent verification, issue/body synchronization and completed-worktree cleanup. Maximum three counted implementation attempts. Preserve actual command/source/exit evidence. If any current product consumer is found, stop deletion and report for scope review; do not break or remove that consumer.

Remaining TOOL14 obligations stay explicitly open under queued successor specs: actual native-pcm-runner process witness; current stale operator root/page/invocation/sweep references and stem-hasher command. Already retired FLAC tools remain retired; historical records remain untouched.

## Numbered TOOL14 disposition

This issue is #545. The complete residual is tracked by #545 orphan rack retirement, #546 current operator/path corrections and #547 native-runner process coverage. Only #545 is currently active; #546/#547 await a slot. FLAC retirement was delivered by #356. Closing one child does not close TOOL14.

## Attempt 1 coherent deletion checkpoint

Luna high removed exactly five tracked files: rack_fixture.rs and the four files under fixtures/rack/v1. Sol high coordinator reports the worker paused at green: metadata removes only the rack_fixture target, 83 graph-compiler tests pass, and final policy/fmt/diff checks pass. Root independently compared before/after metadata and verified every protected issue038 fixture/checker byte against e5b86cf3; hashes and raw available outputs are retained in artifacts/issue545-attempt1.

The initial direct workspace-policy invocation failed with permission denied; its stderr is retained, and the explicit bash invocation passed. The coordinator is collecting the exact command/status ledger before adversarial review; this checkpoint does not substitute its summary for that pending provenance. No further deletion or source implementation is authorized. Sol xhigh verification, required qualification and remote delivery remain pending. #546/#547 remain queued and TOOL14 remains partial.

## Completed command ledger

Sol high's final ledger is now preserved in artifacts/issue545-attempt1/sol-high-coordinate.md. It records metadata before/after status0 and45unchanged workspace members, the sole removed target, focused token-boundary live scan status1/no matches, protected hashes, graph tests83/status0, direct policy status126 followed by bash policy status0, finalfmt0 anddiff0. Unsupported jq --argfile was corrected to --slurpfile; an initial substring scan falsely matched track_fixture and was corrected to a token-boundary scan. These are retained inspection-command corrections, not product test failures or extra implementation attempts.

The ledger's f1d56f63 before/after identity describes the worker's frozen pre-root-commit state; root subsequently committed/pushed the exact deletion at7ed35661. No source changed after that checkpoint. Ready for Sol xhigh adversarial review, not yet delivery.
