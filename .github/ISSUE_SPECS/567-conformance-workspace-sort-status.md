# Restore workspace-sort failure coverage in the conformance boundary fixture

Successor to #542 under audit tracker #560 (IO-1). Root owns Git, GitHub, checkpoints and delivery. Sol HIGH coordinates, Luna HIGH implements, and Astra LOW performs adversarial verification under the user's 2026-09-07 routing. This is non-audio work. Historical #542 verdict attribution remains unchanged.

## Problem and inherited checkpoint

#542's accepted product source moves protocol fixture ownership and tests into `conformance`. Its third attempt repaired the hermetic boundary fixture so required qualification could construct the three exact guarded protocol test children. Astra LOW then proved one retained status-control regression: both synthetic `sort` failures now stop in `gate_toml_dependencies`, so no test reaches the independent workspace manifest/name `gate_sort_lines` failure path. A scratch mutant that makes `gate_sort_lines` swallow a failing sort status still passes the complete fixture suite.

The accepted product/failure checkpoint is `976963578f88beadfb0c7265857f09ad89e5e61c`; the successor branch starts at #542's spec-link-only head `546e003d8bddd42c291dd6617f324ef42782c95b`. It includes accepted product source, qualification evidence, the failed third attempt and its exact counter-mutant. Current main at rescope is `b95c9b7b028ed07cfea2f7669467689c05320c37`. Do not reopen #542 as a fourth implementation attempt; this numbered successor restarts the workflow for the single missing status control.

## Smallest closable slice

Change only `scripts/test-conformance-boundaries.sh` so one deterministic injected `sort` failure reaches workspace manifest or library-name `gate_sort_lines` after all earlier TOML dependency extraction succeeds. Keep separate controls for TOML dependency sorting. The complete clean hermetic fixture, all existing find/awk/paste/rg/status mutations, the exact guarded-child controls, and the module fail-open counter-mutant must remain live.

Use a discriminator tied to the actual workspace-sort invocation or its input/arguments. Do not weaken the production checker, accept an earlier failure as equivalent, delete a retained mutation, or rely on invocation count that changes when manifests are added. No product, protocol, corpus, dependency, Wasm, benchmark, workflow, shared gate helper or unrelated policy change is authorized.

Allowed paths are:

- `scripts/test-conformance-boundaries.sh`
- `.github/ISSUE_SPECS/567-conformance-workspace-sort-status.md`
- focused evidence under `artifacts/issue567-*`

## Objective gates

1. `bash scripts/test-conformance-boundaries.sh` and `bash scripts/check-conformance-boundaries.sh` pass, with shell syntax and `git diff --check` clean.
2. The candidate suite rejects a scratch-only counter-mutant that changes `gate_sort_lines` to swallow a real failing `sort` status. Preserve its exact mutation, command, numeric nonzero exit and causal diagnostic.
3. Evidence distinguishes the workspace `gate_sort_lines` injection from the existing TOML dependency-sort injections and demonstrates each control independently reaches its named consumer.
4. The clean hermetic fixture still models all three exact cfg(test)-guarded protocol children and the approved `conformance` dependency union. Missing-child, missing/changed-guard and outside-path conformance-use controls remain red.
5. Existing find/awk/paste/rg/status mutations and the directed module fail-open mutant remain live; no expected diagnostic is changed merely to accept an earlier failure.
6. The diff from inherited #542 checkpoint contains only this spec, the one fixture script and focused evidence. Product source, production checker, prior evidence and lockfile are byte-identical.
7. Luna HIGH pauses with a coherent focused-green tranche for root checkpoint. Astra LOW reviews the exact committed head and evidence. Required PR `qualification`, merge, synchronized #567/#542/#560 GitHub state, post-main qualification and clean worktree removal complete delivery.

No performance, audio, wire, corpus, dependency or runtime behavior claim is made.
