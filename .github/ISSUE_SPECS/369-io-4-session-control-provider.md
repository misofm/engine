# Issue 369: replace `MockProvider` in the production C ABI

## Scope

Close efficiency-audit row IO-4 only. The shipped `capi` controller currently instantiates
`protocol::MockProvider` with an empty enumerable catalog and one synthetic automation descriptor
for nonexistent track `capi`, effect `control`, handle `u32::MAX`. Replace it with a
`host-core::SessionControlProvider` derived from the compiled session and the live plan sample
projection. Apply IO-1 item 2 by gating `MockProvider` and `MockProviderConfig` behind
`cfg(any(test, feature = "test-support"))`.

All work is control-plane. No render arithmetic or rendered bit may move. Do not implement IO-5's
automation drain, other IO audit rows, crate renames, dependency updates, or new tooling.

## Product contract

- Parameter metadata is snapshotted from the exact accepted `EffectPreparedEntry` descriptors.
  Handles are nonzero, strictly increasing and revision-scoped.
- Parameter state reflects the matching accepted `bank_preparation.initial_values` in handle order.
- Automation domain admission uses the corresponding real descriptor.
- Current/effective sample reads the live plan's published next absolute sample.
- Transport is endpoint-local absolute state/position with that effective sample.
- Counters expose existing protocol telemetry counters and canceled automation.
- Diagnostics expose the existing bounded C ABI render-diagnostic slots without allocating on the
  render path.
- A structural replacement fully allocates its candidate catalog before protocol commit and
  publishes the catalog only after commit succeeds.
- Retained-resource admission includes active and candidate provider catalog allocations.
- Provider counters retain and account for all three owned slots independently of frame-derived
  telemetry configuration capacity.
- Host-core's adapter is optional and non-default; only capi enables it, preserving the default
  browser dependency boundary.

## Verification gates

- `cargo test -p capi`
- `cargo test -p capi --test resource_lifecycle`
- `scripts/check-capi-abi.sh`
- `scripts/check-abi-layout-v1.py`
- refresh `docs/C_ABI_V1_QUALIFICATION.md`
- `cargo test --workspace` on this branch and `origin/main`, with equal pass counts except tests
  introduced here
- compare the worktree with `origin/main`

This is a non-render row, so no benchmark, bit-identity suite, code-generation dump, or render-row
policy gate is required by the finding.

## Decision and evidence record

The provider belongs in `host-core` behind its non-default `control-provider` feature: descriptor
preparation is already shared there and the C ABI
continues to own protocol queues, replay, transport dispatch, render diagnostic reservations and
plan exchange. `PlanSampleSource` is a read-only shared projection; it adds no render operation and
uses the C ABI's existing release/acquire sample publication.

The retained lifecycle fixture's active CAPI row is re-derived from 149,851 bytes after the fixture
provider type replacement and subtraction of the removed mock strings (`4 + 7` bytes), plus 10,800
bytes of soft-clip catalog storage and 282 bytes of bounded diagnostic projection storage, for
160,933 bytes. The double-live CAPI admission is 204,375 bytes. The existing 58,804-byte canonical
writer remains the largest named allocation for that fixture.

Revision attempt 2 snapshots the catalog before graph lowering from the accepted prepared entries,
reserves at least three provider counter records, and narrows the #103 policy exception to an exact
optional edge enabled only by capi. The default host-web graph remains protocol-free. Both the
manifest change and the `#[cfg]`-guarded host-core source edits affect the reproducible linked crate
identity: applying only the host-core `Cargo.toml`/`Cargo.lock` change to main builds
`bf403ee6…`, not the final `d02f6fbb…`. The full shipped AudioWorklet lineage for the complete
source change is refreshed from `6dcd9ced…61e5` to `d02f6fbb…f238`.

Final command outputs and commit/PR identity are recorded in the pull request.


## Current-head Astra review and recovery brief (2026-09-04)

# Astra current-head review: #375 / #369 (IO-4)

**VERDICT: FAIL at `da1e4cc0d3ca3c2f206caf86dc91a70f0baf73ed`.** This is a current-head review, not inheritance of the older pre-rebase approvals. One bounded lineage/integration correction is needed; the reviewed provider implementation itself has no new blocking finding from this review.

## Concrete blocker

The rebase refreshed the shipped source pin, browser qualification results and generated matrix to `635b3e08247b6161d0c24ca178afeeb5895a236c548a930ffa4f8f2a43fcb72f`, but `.github/workflows/npm-publish.yml:33` still declares `EXPECTED_WORKLET_SHA256` as `d02f6fbbdf00036479c31933647bb394854244bdd12428264f9392334164f238`. That workflow's own `Assert the worklet sha256 pin matches its source-of-truth file` step (lines 69–78) deterministically fails. Independently compared the exact values: unequal. Required qualification was green at this head, but it does not prove this publication assertion.

`docs/C_ABI_V1_QUALIFICATION.md:114–116` likewise describes d02f6fbb as the refreshed reproducible current identity, and the local issue decision record leaves that older final identity unqualified. Complete the lineage update, identifying earlier hashes as historical where retained. Do not merely update the three generated merge-conflict files and repeat this partial refresh.

Separately, GitHub reports `mergeable: CONFLICTING`, `mergeStateStatus: DIRTY`. Root's merge-tree proof identifies exactly three generated conflicts against current main: BROWSER_DEPLOYMENT_MATRIX.md, qualification/results.json, and the worklet artifact.sha256. Old-head CI cannot substitute for current-base integration qualification.

## Independently reviewed evidence

Read current issue/PR bodies and previous adversarial reviews, the cumulative implementation, provider mapping/resource code, catalog publication in CAPI transactions, diagnostic/sample bridge, feature edges and mutation gates. Catalog construction uses the accepted EffectPreparedEntry descriptor and initial_values authority before graph consumption; handle/channel/rack mappings, initial-state snapshots, candidate replacement, minimum three counter slots and active/candidate accounting match the local reconciled contract. The transport snapshot is endpoint-local as explicitly recorded; this is not a render automation-drain implementation. Existing initial-state and effect-only catalog limitations are disclosed, not hidden by this verdict.

Current-head independent reruns:

- `cargo test --locked -p capi`: 35 passed (31 unit + 4 lifecycle), 0 failed.
- `cargo test --locked -p host-core --features control-provider`: 72 passed, 0 failed, 2 ignored.
- host-core policy and its mutation suite: PASS.
- realtime policy: PASS (7 marked regions at this old head).
- ABI layout self-test: PASS (17 mutations).
- `git diff --check origin/main...HEAD`: PASS.
- Inspected host-web dependency tree and optional/non-default protocol edge; no production host-web protocol edge introduced.

Logs: `/tmp/astra-369-capi.log`, `/tmp/astra-369-host-core.log`, `/tmp/astra-369-abi.log`, `/tmp/astra-369-host-web-tree.log`. Existing exact-head qualification run 33892033679 is green, including browser checks, but this does not cover the mismatch above. No source edits, publication, GitHub mutation, benchmark or subagents were used by this reviewer.

## Executable minimal recovery brief — Luna

Authority: current user explicitly authorizes resolving blocked work; existing claimed labels do not prevent this coordinated recovery. Root preserves original worktree/history and owns Git/GitHub mutations. This is a bounded integration/lineage repair, not a new provider implementation attempt or a restart of the prior design.

1. Root creates an isolated recovery worktree/branch from exact PR head da1e4cc0. Merge the synchronized default branch normally; no rebase or force-push. Use the latest explicit main commit (root's current proof used 0e248bb0). Preserve all source changes from both histories. Resolve only the three generated conflicts initially with main's versions, documenting that they are placeholders pending regeneration. Root commits the coherent compiling merge checkpoint before further implementation is layered on it. Any additional semantic conflict stops this brief for a bounded amendment rather than speculative conflict resolution.

2. Synchronize the local #369 spec and GitHub issue with this brief, current model roles and actual recovery baseline. Preserve prior evidence as historical. No changes to accepted provider behavior, protocol identities, realtime rules, DSP, dependencies, target support or test gates. Allowed recovery outputs: the three generated conflict paths; `.github/workflows/npm-publish.yml` expected worklet digest only; `docs/C_ABI_V1_QUALIFICATION.md`; the #369 evidence/spec and necessary generated lineage strictly required by the existing current-main build/qualification scripts. Report any unexpectedly broader regeneration before accepting it.

3. On the merged candidate, use the existing current build script with a fresh EMPTY output directory: `MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1 bash scripts/build-web-audioworklet.sh <empty-dir>`. Use the produced actual SHA-256 as the one current identity, synchronize the npm workflow expectation and current C-ABI qualification prose. Root commits the coherent candidate/pin checkpoint so browser qualification can name an immutable candidate. Rebuild without repin into a second fresh output directory and require exact digest reproducibility; no artificial pin edits or historical hash substitution.

4. Run the existing qualification commands in `hosts/host-web/qualification` using that artifact directory and immutable candidate: `npm run qualify -- --artifacts <dir> --browser all --self-test-mutations --record-matrix --candidate-commit <candidate>`; then separate `--browser chromium|firefox|webkit --check-matrix --self-test-mutations` invocations as the established pipeline requires. Regenerate the matrix from the actual results. The candidate source identity must match the artifact; final evidence commit may differ because generated qualification results refer to the already-built candidate. No publishing npm workflow invocation is required for this repair.

5. Required proportional gates on the merged candidate: capi tests and explicit resource_lifecycle suite; host-core control-provider tests; host-core/protocol/realtime policies and changed-script mutation suites; C ABI native linkage; ABI-layout self-test and generated-layout validation using the existing emitter; fmt, focused all-feature clippy; current AudioWorklet static/resource and all-three-browser checks; complete workspace tests compared against the chosen current-main baseline, with only the established five IO-4 tests as expected delta unless current main independently changed counts. Run the npm workflow's EXACT read-only digest equality assertion locally so the current blocker is directly proved closed. No benchmark or optimization pass.

6. Pause at a green coherent checkpoint and send root exact paths and logs. Root commits/pushes normally to the existing PR branch, refreshes PR #375's complete lineage/Before-after/Gates/Skipped sections, and requests Astra review at the final pushed head. Required qualification must pass again against current integration; the old green run is historical evidence only. Merge/close #369/post #349 only after exact-head Astra PASS and required CI. If artifact tooling reveals a new unrelated defect after one bounded correction, preserve its evidence and separate the tooling issue; do not let this become another broad qualification framework project.

Recovery worktree: `/home/bl/misofm/engine-369-recovery`, branch `codex/369-recovery`, starting PR head `da1e4cc0d3ca3c2f206caf86dc91a70f0baf73ed`. Integration baseline: `0e248bb07cfbf7dd136ec48649ec61ee9171d15b`. Root owns commits/pushes, Luna receives the bounded recovery implementation, Astra reviews the resulting PR; Sol is the retry if Luna fails. This section supersedes older conflicting model/checkout instructions.

## Recovery execution evidence (2026-09-05)

Normal merge `a3b987b1` integrates main `0e248bb0` while preserving original PR history. Luna's immutable artifact candidate `ee14a35990881eb4139cd0830129ad9821782560` produces reproducible worklet SHA-256 `e48504a17e00334e7719ac9defd11c3f52e11802f66a68cc01115641fd8c91b0`. Source pin, publication workflow expectation and current qualification prose agree. Browser evidence checkpoint `73751d63` records all-browser qualification and separate Chromium/Firefox/WebKit checked-matrix mutation passes against that candidate.

- Integrated capi tests: 35 passed; explicit resource_lifecycle: 4 passed.
- host-core with control-provider: 72 passed, 0 failed, 2 ignored.
- host-core and protocol-control policies plus mutation harnesses, realtime policy, fmt and focused all-feature clippy: PASS.
- C ABI shared/static native linkage and ABI-layout 17-mutation self-test: PASS; generated layout and AudioWorklet static/object gates: PASS.
- Built browser/native resource agreement plus 26 red mutations: PASS.
- Exact read-only publication/source-pin guard: PASS. No npm publication.
- Fresh main baseline `0e248bb0`: 1546 passed, 0 failed, 24 ignored. Serial candidate workspace at `73751d63`: 1551 passed, 0 failed, 24 ignored; exactly +5 IO-4 tests.

The first candidate workspace invocation passed executable suites but failed capi rustdoc dependency loading after other Cargo feature builds used the same target directory concurrently. The isolated capi doctest passed; a serialized complete workspace rerun passed. The failed log `/tmp/engine-369-recovery-workspace.log` is retained and is not counted as PASS; successful serial evidence is `/tmp/engine-369-recovery-workspace-serial.log`. Future same-target Cargo qualification is serialized. No source change or weakened test was used to resolve this orchestration error.

Other logs are `/tmp/engine-369-{integration-capi,final-host-core,final-lifecycle,final-protocol-policy,final-realtime,final-clippy,final-browser-resources,final-publisher-guard,check-web-2,check-capi,check-abi,qualify-all,check-matrix-chromium,check-matrix-firefox,check-matrix-webkit}.log`. Runtime provider behavior is unchanged by recovery. Astra final PR review and fresh required CI remain pending before merge. Older digest/count records above are historical, not the current qualification identity.
