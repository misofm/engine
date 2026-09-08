# Remove full LLVM IR captures from the default branch

GitHub: https://github.com/misofm/engine/issues/625

Parent: #560. Coordination: #559 and active #621.

At delivered main `7af655071f528f5cfcfbec3fa6de3a306d79a30a`, seven historical evidence directories track 39 full or selected LLVM IR (`.ll`) files totaling 13,345,253 bytes. GitHub Linguist classifies those payloads as LLVM, which makes the repository appear to be roughly 41% LLVM even though the product remains Rust. The captures are tied to exact source, compiler, flags and target identities and can be regenerated when a future investigation actually needs them. Their durable value is in the recorded identities, hashes, conclusions and small claim-specific excerpts, not permanent copies of whole compiler modules.

The owner ruled during #621 that full LLVM IR captures do not belong on the default branch. This issue applies that ruling to the historical files after #621 compacts its pending current/candidate evidence and explicitly releases its read dependency on #539. #621 and #625 are the two active issue slots. Lane B owns this cleanup; lane A retains its limiter source and evidence worktree until delivery.

## Smallest closable outcome

Delete every tracked `artifacts/**/*.ll` file from the current branch. The current census is:

| Evidence directory | Tracked `.ll` files |
| --- | ---: |
| `artifacts/issue475-luna-attempt1/` | 2 |
| `artifacts/issue534-baseline/` | 5 |
| `artifacts/issue534-luna-attempt2/` | 5 |
| `artifacts/issue537-baseline/` | 3 |
| `artifacts/issue537-candidate-lowering/` | 3 |
| `artifacts/issue539-baseline/` | 5 |
| `artifacts/issue539-candidate-lowering/` | 16 |

Repair only metadata in those seven directories and their numbered issue decision records where deletion would otherwise falsely claim that a payload remains present or checksum-verifiable. Preserve source commit, source-file hash, rustc/tool versions, target/configuration, command, original byte size/hash and the already accepted technical conclusion when they exist. A small text excerpt is allowed only when an existing conclusion would otherwise lose its claim-specific basis; it must not reconstruct a full function or disguise bulk IR under another extension. Historical Git objects remain available through old commits.

Add an `artifacts/**/*.ll` ignore rule and extend the existing workspace policy and its mutation suite so a force-added tracked LLVM IR capture under `artifacts/` fails qualification. The policy must inspect the Git index when in a worktree and retain the existing synthetic-tree behavior used by the mutation suite. It must not reject source fixtures outside `artifacts/`, temporary compiler output outside the repository, commands or metadata that mention `.ll`, or the historical assembly files left in scope.

Implementation ownership is limited to:

- deletion of the 39 tracked `.ll` files in the seven directories above;
- necessary candid metadata/decision-record repairs within those seven directories and issues #475, #534, #537 and #539;
- `.gitignore`;
- `scripts/check-workspace-policy.sh` and `scripts/test-workspace-policy.sh`;
- this issue spec and bounded review evidence.

Do not remove historical `.s` files, change Rust or JavaScript product source, rerun a compiler, benchmark or browser, edit Cargo manifests or `Cargo.lock`, rewrite history, change an AudioWorklet output/pin/lineage record, or revise an accepted DSP/performance conclusion. If a metadata repair cannot stay inside this boundary, record the stale historical path as unavailable on current main rather than expanding scope.

## Objective gates

Before implementation, Astra LOW must verify the census, the #621 dependency hold, the exact ownership above, and that the proposed policy test discriminates a tracked/force-added `artifacts/**/*.ll` file without banning ordinary compiler commands or allowed files.

After #621 releases the hold, Luna HIGH makes one coherent implementation tranche. The checkpoint must prove:

1. `git ls-files 'artifacts/**/*.ll'` is empty and the deleted-file census is exactly 39 files / 13,345,253 bytes against `7af65507`;
2. the seven affected evidence records do not claim the removed payloads are present or locally checksum-verifiable;
3. preserved identity and conclusion records remain internally consistent;
4. the workspace policy passes on the real tree, rejects a force-added artifact `.ll` mutation, continues accepting an allowed `.s` artifact and a non-artifact `.ll` fixture, and its complete existing mutation suite passes;
5. `git diff --check`, shell syntax checks for affected scripts, and the proportional workspace policy gates pass;
6. no file outside the ownership list changed and no compiler, benchmark, browser or artifact builder ran.

Astra LOW then adversarially reviews the checkpoint against the exact base and owner ruling. Root performs the exact-path status/commit/upstream audit, opens the PR only at a clean reviewed head, waits for required qualification, obtains a fresh Astra LOW merge review, verifies merge parents and post-main qualification, synchronizes #625/#559/#560, and removes the clean worktree.

## Decision record

### 2026-09-08 — Owner storage ruling and bounded brief

The owner confirmed that the `.ll` captures do not need permanent retention and asked when they would ever be reused. The only observed reuse was #621's one-time read of #539 lowering for a bounds-check investigation. That use established the replacement rule: retain the exact generation identities, hashes, technical conclusion and minimal claim-specific excerpts; regenerate a full module only inside a future bounded investigation when necessary.

The required issue-boundary audit found 306 numbered local specs and no local numbered spec lacking a matching GitHub issue before #625 was created. Existing historical remote issues without local specs and inherited title-prefix drift are outside this bounded cleanup. Main was clean and synchronized at `7af65507`. The baseline census is 39 tracked `.ll` files / 13,345,253 bytes in the seven directories above; 37 historical `.s` files / 4,189,861 bytes remain explicitly out of scope. No history rewrite is authorized.

### 2026-09-08 — Astra LOW scope PASS; dependency hold remains

Astra LOW passed the exact pushed brief `4ba68dca4fa7919954927d7de124bb1a58b5719c` against live main `7af655071f528f5cfcfbec3fa6de3a306d79a30a` and tracker `22d9e61f5a76297449a9b9ead0072474afeca619`. Its independent Git-tree census reproduced exactly 39 files / 13,345,253 bytes in the seven named roots. The owner ruling, historical-assembly exclusion, bounded metadata repairs and no-history-rewrite boundary are accepted.

The workspace policy and its existing mutation suite are the accepted prevention surface. The implementation must fail closed when Git-index inspection fails; an index-listing error cannot be treated as an empty census or select a permissive fallback. Mutations must distinguish a force-added artifact `.ll` from allowed artifact assembly and a non-artifact `.ll` fixture while preserving the checker's synthetic-tree behavior. Luna HIGH is authorized for one exact-scope tranche only after Astra LOW passes #621's compact evidence and #621 explicitly releases the #539 read dependency. No implementation begins before both conditions hold.

### 2026-09-08 — #621 compaction PASS; dependency released

Astra LOW passed #621's compact evidence at exact pushed checkpoint `3d1e92678f4999dff6834d800dc006ef785b6287`. It verified that no added full/selected `.ll` or redundant full `.s` survives, both small remaining archives contain only eight documented non-IR diagnostic/test streams, 27 bounded excerpts reproduce from the recorded original hashes and line ranges, and the frozen limiter source remains unchanged. #621 records the verdict at synchronized head `d727f7e51794d031c6e9ef5e0fd5716938fa46ac` and explicitly releases its historical #539 read dependency. The last #625 implementation blocker is clear; Luna HIGH may begin the single authorized tranche.
