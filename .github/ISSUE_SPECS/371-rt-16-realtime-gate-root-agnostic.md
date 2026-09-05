Split out of the #349 efficiency-audit tracker, **Wave 0** (correctness and gate integrity). Rows: **RT-16,IO-14**.
This issue is self-contained: implement from this body alone. Do not re-audit #349.

## Assignment

| | |
|---|---|
| Implementer | Qwen 27B |
| Verifier | Fable 5.1 (reads the diff, reruns every gate below, posts the tracker note on #349) |
| Branch | `qwen/371-rt-16-realtime-gate-root-agnostic` off `main` |
| Land after | none (TOOL-11 on #306 lands after this) |
| PR | one PR, squash-merged, title prefixed with the row id(s) |

## Operating procedure (complete; no other instructions exist)

Repo: `/Users/bl/Documents/GitHub/misofm/engine` (GitHub `misofm/engine`). `N` is this issue's number.

**0. Claim.** `gh issue view N --json labels` must not contain `claimed`; if it does, stop, another agent owns it. Every issue in the "Land after" row must be `CLOSED` (`gh issue view <dep> --json state`); if not, stop and say which one is open. Then `gh issue edit N --add-label claimed`.

**1. Worktree.** Never work in the main checkout.
```sh
git -C /Users/bl/Documents/GitHub/misofm/engine fetch origin
git -C /Users/bl/Documents/GitHub/misofm/engine worktree add ../engine-N -b <branch from the Assignment table> origin/main
cd /Users/bl/Documents/GitHub/misofm/engine-N && export CARGO_TARGET_DIR=$PWD/target
```
Read `AGENTS.md` and `docs/REALTIME_DEPENDENCY_POLICY.md` once before editing.

**2. Scope.** Implement only the finding(s) in this issue, following any "Reconciled scope" / "Notes for the implementer" section first and the finding's "Proposed change" second. Locate code by the named functions; line numbers are from `main` `4797a544`. Anything else you notice goes in the PR under "Seen, not done" and stays untouched. No reformatting, renames, dependency bumps or new tooling. Where the finding lists ordered options, try them in that order; an option marked class B or owner ruling is never implemented: if you reach it, post the evidence on this issue and stop.

**3. Null result.** If the premise no longer holds on current `main` (code already matches the proposal, or the named code is gone), do not invent a change. Comment on this issue with a one-paragraph null note, remove the `claimed` label, and stop.

**4. Evidence, all required.**
- The exact "Verification gate" commands from the finding, output saved.
- `cargo test --workspace` in the worktree and on `origin/main`; pass counts must match unless this issue adds tests (state the delta).
- Any script you edited: also run its `scripts/test-<name>.sh` mutation harness.
- Render-path rows only: `scripts/check-realtime-policy.sh`, the bit-identity fixture suite named in the finding, and one run of `scripts/run-console-benchmark.sh` on the named row. Report before/after once; never tune.
- Codegen claims (an instruction that should disappear): attach before/after `llvm-objdump -d` (or `cargo asm`) for the named function.

**5. Deliver.**
- Commit: subject `<row-id>: <what changed>`, body listing the commands run, last line `Refs #N`.
- `gh pr create --base main` with sections: Summary; Rows closed; Gate output (fenced); Before/after; Seen, not done; Skipped (any step not done and why). Add `Closes #N` only when every row in this issue is done.
- Do not post on #349; the verifier does that. Do not `git push --force`, do not touch `main`, do not edit outside the worktree.
- Reply with the PR URL and the Skipped list.

## Reconciled scope (RT-16 and IO-14 describe the same gap; implement once)

1. In `scripts/check-realtime-policy.sh`, replace the single `find "$realtime_root"` walk with a discovered file set: `rg -l 'REALTIME_POLICY_BEGIN' crates hosts tools sidecars --glob '*.rs' | sort`. Keep the `crates/engine/src/realtime` directory-existence check as its own assertion. Add a floor on the **file count** (set to the count after step 2) alongside the existing marked-region floor (raise it to the current count), so deleting a marker to silence the gate fails it.
2. Add `REALTIME_POLICY_BEGIN`/`END` markers around: `crates/rack/src/lib.rs` `BankChain::run`, `gather*`, `scatter*`, `accumulate_aux`, the three `BankStage::process` bodies; `crates/builtins/src/lib.rs` `InputStage::process`/`process_mono`, `FaderRampStage::process`, `MatrixStage::process`, `MeterAccumulator::observe`; `hosts/host-web/src/lib.rs` `render_next`. **Also** `crates/effect-contract/src/live.rs`: verified 2026-09-04 that the existing markers (lines 458-544) wrap only `impl ObservationLane`; `EffectControlLane::stage` (line 184) is unmarked, contrary to IO-14's text. Wrap the `impl EffectControlLane` block (it is allocation-free by construction, `live.rs:27-33`) and use that region for the IO-14 red mutation in step 4. Re-read each region against the gate's regex before marking; fix only trivial hits (e.g. a `map_err` shape), and if a region needs a real code change to pass, leave it unmarked and say so in the PR.
3. Do **not** mark `crates/source/src/lib.rs`'s render pull: it carries 16 `expect` edges (IO-13, Wave 2) and would turn the gate red. IO-13 owns those markers.
4. Red mutations: `scripts/test-realtime-policy.sh` must fail the gate on a `let _ = vec![0u8; 1];` inside `execute_op`'s marked region in `crates/graph/src/runtime.rs` **and** inside `EffectControlLane::stage`'s region in `crates/effect-contract/src/live.rs`, and pass once removed. Record both in `crates/graph/tests/MUTATIONS.md` (or the sibling file the harness already uses).
5. Keep the change local to this script's walk. The shared `scripts/lib/gate.sh` extraction is #306 / TOOL-11 and lands after this issue.

## Finding (verbatim from #349)

#### RT-16: `check-realtime-policy.sh` scans only `crates/engine/src/realtime` — every marked region outside it, and all of `rack`/`builtins`, is ungated
- **Category:** architecture
- **Class:** N/A
- **Severity:** medium
- **Location:** `scripts/check-realtime-policy.sh:14-15` (`realtime_root="crates/engine/src/realtime"`),
  `scripts/check-realtime-policy.sh:38-46` (the `find "$realtime_root"` walk feeding the forbidden-
  surface regex); the unscanned marked regions are in `crates/graph/src/lib.rs`,
  `crates/graph/src/runtime.rs` and `crates/effect-contract/src/live.rs`.
- **Current code:**
```bash
realtime_root="crates/engine/src/realtime"
[[ -d "$realtime_root" ]] || fail "missing realtime module"
# …
done < <(find "$realtime_root" -name '*.rs' -type f | sort)
```
- **Problem:** the forbidden-surface regex (`Vec::|vec!|Box::|…|\.unwrap\(|panic!\(`) is applied
  only to files under `crates/engine/src/realtime`. `crates/graph/src/runtime.rs` — which contains
  `execute_op`, `reduce_plane`, `CompensationDelay::process`, `TrackDelayLine::process` and
  `publish_observations`, i.e. the actual per-block render body — writes
  `REALTIME_POLICY_BEGIN`/`END` markers around exactly those functions and **nothing checks them**.
  `crates/rack/src/lib.rs` and `crates/builtins/src/lib.rs` carry no markers at all, so
  `BankChain::run`, `InputStage::process` and every bank stage are outside the gate entirely. I ran
  the script's own regex over the three unscanned marked files by hand and they are clean today
  (625 marked lines, zero hits) — so this is a gate gap, not a live violation. That is exactly the
  moment to close it, before a `.unwrap()` lands in `execute_op`.
- **Proposed change:** make the marker walk root-agnostic. Replace the `find "$realtime_root"` walk
  with a walk over every file in `crates hosts tools sidecars` that contains
  `REALTIME_POLICY_BEGIN`, keep the existing `marker_count >= 4` floor (raise it to the current
  count), and keep the `crates/engine/src/realtime` directory-existence check as its own assertion.
  Then add markers to `crates/rack/src/lib.rs` around `BankChain::run`, `gather*`, `scatter*`,
  `accumulate_aux` and the three `BankStage::process` bodies, and to `crates/builtins/src/lib.rs`
  around `InputStage::process`/`process_mono`, `FaderRampStage::process`, `MatrixStage::process`
  and `MeterAccumulator::observe`. Two of those will need small edits to pass: the
  `.expect("validated bank shape")` at `crates/graph/src/runtime.rs:1298` is bind-time and outside
  a marker, but `EffectBankProcessBlock::new(...).map_err(...)` patterns should be re-read against
  the regex before markers go in.
- **Expected effect:** the render-path no-alloc/no-panic rule becomes enforced where the render
  path actually is, rather than where issue 003 left it. No runtime change.
- **Rulings / constraints checked:** `docs/REALTIME_DEPENDENCY_POLICY.md` and
  `AGENTS.md`'s "Render must perform zero allocations… with no exception" already assert the
  property for the whole render plane; this only makes the gate match the assertion.
- **Verification gate:** run the amended `scripts/check-realtime-policy.sh` and confirm it fails on
  a deliberate `let _ = vec![0u8; 1];` inserted inside `execute_op`'s marked region, then passes
  once removed (record it as a red mutation in `crates/graph/tests/MUTATIONS.md`).



#### IO-14: three files carry `REALTIME_POLICY` markers that the gate never scans
- **Category:** realtime-safety
- **Class:** N/A (gate coverage)
- **Severity:** medium
- **Location:** `scripts/check-realtime-policy.sh:34-48` (the scan root); the unscanned marked files `crates/graph/src/runtime.rs`, `crates/graph/src/lib.rs`, `crates/effect-contract/src/live.rs`.
- **Current code:**
```bash
# scripts/check-realtime-policy.sh:12-13
realtime_root="crates/engine/src/realtime"
[[ -d "$realtime_root" ]] || fail "missing realtime module"
...
# scripts/check-realtime-policy.sh:48
done < <(find "$realtime_root" -name '*.rs' -type f | sort)
```
- **Problem:** `grep -rl REALTIME_POLICY_BEGIN crates/ hosts/` returns nine files, six under `crates/engine/src/realtime` and **three outside it**: `crates/graph/src/runtime.rs` (the per-node render dispatch), `crates/graph/src/lib.rs`, and `crates/effect-contract/src/live.rs` (`EffectControlLane::stage` — the one function that applies live parameter changes on the render thread). The gate's `find` root is a single directory, so those three files' markers are decorative: the banned-token regex at `:53-57` never sees them, and the "unmatched markers" check at `:39-41` never runs on them either. I checked the current contents of all three marked regions against the gate's own regex and they are clean today, so this is a coverage gap rather than a live violation — but nothing prevents the next edit to `EffectControlLane::stage` from adding a `Vec::` or an `.expect(` under a marker that claims to forbid it.
- **Proposed change:** In `scripts/check-realtime-policy.sh`, replace the single root with the set of files that actually carry markers, discovered rather than listed:
```bash
mapfile -t marked < <(rg -l 'REALTIME_POLICY_BEGIN' crates hosts --glob '*.rs' | sort)
(( ${#marked[@]} >= 9 )) || fail "expected at least nine marked realtime files"
for source in "${marked[@]}"; do ... done
```
  Keep the `>= 4` marked-region floor and add a floor on the file count so deleting a marker to silence the gate fails it. Then add markers to `crates/source/src/lib.rs`'s render pull (see IO-13) and `hosts/host-web/src/lib.rs::render_next`.
- **Expected effect:** the gate covers the whole marked render path instead of one third of it; three files' worth of markers stop being decoration.
- **Rulings / constraints checked:** No ruling. `docs/REALTIME_DEPENDENCY_POLICY.md` is the normative document the gate enforces; nothing in it limits enforcement to `crates/engine/src/realtime`.
- **Verification gate:** `scripts/test-realtime-policy.sh` (the gate's own test — add a red mutation that puts `vec![]` inside `crates/effect-contract/src/live.rs`'s marked region and assert the gate fails), then `scripts/check-realtime-policy.sh`.





## Constraints (from #349, binding)

- Unfused `(a*b)+c` everywhere; bit identity across scalar/W4/W8/wasm. A class A row moves **no rendered bit**. If the row turns out to need one, stop, re-class it B on #349, and do not implement.
- No new crate names, no renames for their own sake, no runtime SIMD dispatch, no `unsafe` outside the existing allowlist without a ruling.
- Render path is zero alloc/free/lock/syscall/I/O/log/panic (`scripts/check-realtime-policy.sh`, `docs/REALTIME_DEPENDENCY_POLICY.md`).
- One measurement per row where the finding names one, recorded, not chased. No tuning loops.
- Line numbers above are from `main` `4797a544` and may have drifted; locate by the named functions. If the premise no longer holds on current `main`, close this issue with a one-paragraph null note and append the same note to #349.

## Done when

1. The finding's **Verification gate** above is green, with the command output attached to the PR.
2. `cargo test --workspace` passes with the same pass count as `main`, plus the named policy scripts.
3. A one-paragraph note on #349: row id, commit hash, what changed (or the null result).

Spec mirror: `.github/ISSUE_SPECS/371-rt-16-realtime-gate-root-agnostic.md`.


---

# Astra brief: #371 / PR #389 — finish marked realtime-gate coverage and qualify its artifact

**Scope APPROVED for one Luna recovery pass after #369 lands.** Current remote PR head is `20ea904dba11cab949b57395afba8d9897847639`. The existing local `engine-371` worktree is at rewritten/equivalent `8dedfbf3` with an uncommitted worklet pin; root must preserve it and its history. Do not use that dirty hash as the merged candidate's artifact authority. User explicitly authorizes resolving blocked work. Current roles: Astra brief/review, Luna first recovery attempt, Sol retries on failure (at most two, then stop/rescope); root owns checkpoints, pushes, PR updates and merge.

## Independent semantic assessment

No substantive blocker found in the remote patch against #371's explicit reconciled scope. The script discovers marked Rust files across crates/hosts/tools/sidecars, retains the foundation-directory assertion, and enforces the requested file/region floors. Read-only extraction from the exact remote tree finds **12 marked files, 42 regions, no nested/orphan/unclosed marker sequence**. The named rack, builtin, host render and EffectControlLane bodies are marked; the Rust changes are marker comments, not runtime behavior. Unmarked source pulls with the existing expect edges remain excluded exactly as the issue requires.

The gate/test scripts are byte-identical between remote20ea904d and local8dedfbf3. Independently ran both on the local tree: `realtime policy: ok (42 marked regions in 12 files)` and `realtime policy mutation tests: ok`. Existing mutations cover allocation in execute_op, EffectControlLane::stage and the host render body; discovery of an additional marked tools file; file/region removal floors; unmatched markers outside the old root; and exact unsafe ownership. The harness checks the intended diagnostic class after explicit unexpected-success refusal.

Known scan-error handling defects (`rg` status swallowing, process-substitution errors), shared helper extraction and broader non-vacuity hardening remain **#306**, expressly dependent on this issue. Do not add `scripts/lib/gate.sh`, rework all scans, introduce a policy table, or claim this marker gate is transitive call-graph proof. Do not expand malformed-marker grammar, marker roster or source-pull scope in this recovery absent a concrete new failure requiring a brief amendment. The generic failure-handling debt must be preserved for #306, not silently declared fixed by #371.

## Current blocker and smallest closable outcome

Required qualification at remote20ea904d fails at the shipped AudioWorklet build; other listed leaves passed. The current pin was not refreshed after marker edits. The local dirty candidate pin is unverified evidence for that older tree, not an acceptance result for the upcoming merge. Smallest outcome: preserve the existing semantics, integrate current main after #369, regenerate and fully qualify one actual merged-tree artifact, then land the existing PR. No fresh runtime feature or gate-framework project.

## Exact recovery procedure

1. After #369 is upstream/closed and its final artifact lineage is synchronized, root captures current main SHA and creates a NEW isolated recovery branch/worktree from remote20ea904d. Merge current main normally; no rebase/force-push and no editing of the original dirty worktree. Resolve only generated lineage conflicts using main's values as temporary placeholders pending regeneration. Preserve both sides' semantic changes and current target support. A conflict in policy/runtime semantics beyond the equivalent known marker patch stops for a brief amendment. Root commits a compiling merge checkpoint with any pending artifact qualification explicitly documented before layering more work.

2. Restore the missing local numbered #371 spec mirror from the existing GitHub issue, including its reconciled scope and this recovery brief. Preserve original issue number/title identity; synchronize remote scope/model roles. Allowed paths: existing #371 gate and mutation files, originally touched Rust marker/comment locations and MUTATIONS.md only if integration requires them; #371 local spec/evidence; actual artifact pin, qualification/results.json, generated BROWSER_DEPLOYMENT_MATRIX.md; `.github/workflows/npm-publish.yml` EXPECTED_WORKLET_SHA256 only; current artifact-identity prose in docs/C_ABI_V1_QUALIFICATION.md inherited from #369, clearly superseding its old artifact hash without rewriting old measurements. Other required generated lineage outputs need explicit identification before accepting them. No arbitrary scripts, SDK features, dependencies, fixture contents or release configuration changes.

3. Before artifact timing/build expense, run `bash scripts/check-realtime-policy.sh`, `bash scripts/test-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`, `cargo fmt --all -- --check`, and `git diff --check`. Confirm current marker count from actual merged source; retain the 12/42 floors unless a separately landed marker addition demands a documented upward update. Never lower them to get green. Inspect Rust diff relative to merged main: marker/comment changes only. Keep both required execute_op/stage red cases discriminating. Existing harness is sufficient; no new generic test framework.

4. Build the merged source through the CURRENT official script into a fresh existing empty directory: `MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1 bash scripts/build-web-audioworklet.sh <dir>`. Synchronize every CURRENT artifact assertion with the actual resulting digest, especially publisher EXPECTED_WORKLET_SHA256 and C-ABI qualification prose: #375 showed that the three generated files alone are insufficient. Root checkpoints the candidate/pin update so the browser qualification names an immutable source candidate. Rebuild without repin into a second fresh directory and require reproducibility. Never carry forward 7b89… or any old pin merely because it was produced in the preserved original worktree.

5. Run existing current AudioWorklet static/resource gates and the browser qualification pipeline in hosts/host-web/qualification: `npm run qualify -- --artifacts <dir> --browser all --self-test-mutations --record-matrix --candidate-commit <immutable candidate>`, then each Chromium/Firefox/WebKit checked-matrix self-test invocation. Use actual results to regenerate the matrix; do not manually invent successful browser records. Run the npm workflow's exact read-only source-pin equality assertion locally. Do not publish npm or invoke a publication workflow.

6. Complete proportional merged-base verification: all #371 named policy/harness gates, applicable existing docs/artifact validators, and full `cargo test --workspace` compared against the captured post-#369 main baseline. Expected cargo count delta is zero for marker-only #371; explain any independent main movement rather than rewriting counts. No benchmark or rendered-PCM change is claimed, and no optimization timing is required. Artifact build identity may move despite comment-only Rust changes; qualify the actual build rather than asserting it cannot change.

7. At each coherent green tranche Luna pauses and sends root exact paths/logs; root commits before more implementation and normal-pushes the recovery to the existing PR branch. Preserve the original worktree/rebased local history without force-push. Refresh PR #389 around FINAL cumulative behavior and actual new evidence, not the old failed artifact head. Astra reviews the pushed PR before merge; required current qualification must pass. Only then merge, verify #371 CLOSED and synchronize #349's RT-16/IO-14 note. #306 becomes eligible after that point.

## Failure/attempt boundary

Luna has one coherent recovery pass plus one Astra verdict; a failed attempt goes to Sol, with at most two Sol revisions and a hard stop after three failures total. Ordinary gate completion and checkpoint handoffs do not authorize hidden revision loops. An unrelated artifact/qualification runner defect gets one bounded correction with candid evidence, then a separate tooling issue/rescope instead of broadening #371. Do not weaken gates or turn stale historical results into the new candidate's evidence.

This briefing performed only source/issue inspection and the two non-timed policy checks, which create temporary fixture files. No repository or GitHub mutations, claims, artifact builds, timed workloads or agents were started.

## Recovery checkpoint — 2026-09-05

The remote PR head 20ea904d was normally merged with post-#369 main `1ef2375c0a9fa4b1481e844a1f68f48d22f8dc6f` without conflicts in a separate recovery worktree. Existing dirty/revised worktrees remain preserved. Artifact qualification is pending; inherited hashes are not claimed as this candidate’s evidence. Current user-assigned Astra/Luna/Sol roles supersede earlier implementation assignments.
