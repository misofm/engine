# Complete workspace and Wasm discovery before certifying their policy

Stateless successor of #306; depends on merged #400. This is issue #404; root synchronizes its local/remote spec before implementation. Current roles: Astra brief/review, Luna one coherent attempt, Sol at most two retries following FAIL, hard stop/rescope after three failures. Root owns commits, pushes, review/CI/merge. No edits to another issue's active tranche.

## Smallest outcome and scope

Complete or fail the remaining workspace-policy discovery/filtered scans and the ninth original find loop in Wasm realtime atomics. Retain current policies, diagnostic classes, input roots/CLI and optional-result semantics. No new ban, target support, opcode policy, artifact schema or browser rebuild. This does not expand #400 or migrate another member of #401–#403.

Allowed production paths: `scripts/check-workspace-policy.sh`, `scripts/check-wasm-realtime-atomics.sh`, minimal extension of `scripts/lib/gate.sh` only if its merged API lacks the necessary checked producer operation. Tests: existing `scripts/test-workspace-policy.sh`, focused `scripts/test-wasm-realtime-atomics.sh` if still absent, shared helper tests if changed; numbered issue/evidence only. No generic test framework or command-runner library.

Freeze actual post-#400 sites before coding:

- Workspace five find-backed populations: package.json, package-lock.json, Cargo manifests under crates/hosts/tools/sidecars, forbidden retired directory names, shallow .fingerprint spill directories. Capture complete producer status before consuming results; check sort/filter status too.
- Workspace explicitly deferred positive queries (license/workspace inheritance, third-party reference, ISA table/pin), comment-stripped per-manifest retired-codec scan, tracked_paths manifest pipeline (git listing or non-Git fallback), optional .cargo filtered ISA query and global [build] ban. Check producer/read/parser statuses before applying no-match/allowlist logic. Keep existing TOML/comment interpretation unchanged; don't replace it with a parser project. An optional empty [[bin]] list remains valid, while a failed extractor cannot masquerade as that empty list.
- Wasm object's find/sort list must be captured once successfully and required nonempty before scanning. Reuse the checked population for observation-object presence if practical. Preserve the existing observation-symbol OR source-ObservationSlot semantics for successful searches; neither producer errors nor failed wasm-objdump/rg calls are legitimate absence. Keep atomic opcode bans exactly unchanged. This is error propagation through the existing object inspection, not a target/artifact redesign.

## Frozen empty-set rules

Successful empty npm package/lock discovery passes, so existing Rust-only fixture roots remain valid. All four Cargo discovery roots are required; at least one discovered package manifest across them is required, not one per root. Successful empty retired-directory/fingerprint results pass. A missing optional .cargo directory is valid; scan errors when it exists fail. Wasm objects are required nonempty. Every producer error fails regardless of output length, whether zero, clean partial rows or rows containing an actual violation. Preserve stderr/status evidence and never certify a partial population.

Use explicit conditionals/status capture; don't rely on set-e, a conditional caller's errexit, process substitution, pipefail alone, or `! command` as an assertion. A list cannot be trusted merely because nonempty. Safe fixture-root selection sources the real shared library before cd. Never filter required roots to those that exist or mkdir a missing root to obtain green.

## Proportional gates

- Existing workspace mutation suite and helper suite pass; preserve all current policy red cases. Add clean empty npm/lock and clean empty forbidden-directory/fingerprint positives; required-empty manifest red with a complete valid root fixture, plus missing required root. Ensure new cases reach the intended discovery site rather than an unrelated early license failure.
- For each materially different producer pipeline, inject failure with zero output AND failure after a valid nonviolating partial row. Cover find, tracked git listing/fallback, parser/filter and rg status >=2 where applicable. Distinguish rg 1 from failure. Root privileges must not make unreadability mutations vacuous; controlled executable failure is acceptable. Positive searches must distinguish missing required content from failed search in diagnostics.
- A hermetic Wasm-script harness stubs the existing external tools/build command and creates representative object/archive fixtures without compiling/timing. Prove valid object positive, atomic opcode red, empty object set red, one valid object followed by find failure red, failed disassembler red, and failed observation-object search cannot be hidden by a successful source fallback. Stub only the intended operation, not the policy under test; assert intended diagnostic and explicit unexpected-success refusal. A counter-mutation must prove at least the partial-producer failure assertion would reject the original unsafe control flow.
- Run `bash -n` on changed scripts, actual workspace policy, affected hermetic suites and diff hygiene. Run the existing real Wasm atomics gate once at final qualification with required toolchain available (non-timed build/inspection); do not invent a second matrix or rebuild the shipped AudioWorklet. If unavailable, required CI must provide the real result; record local limitation honestly. Existing full-workspace unchanged-count comparison remains the parent program gate at coherent delivery boundary; no new Rust tests for shell mirrors.

No benchmark invocation, mutation of sealed evidence or publication. Luna pauses after one coherent green tranche for root checkpoint. Astra reviews the actual pushed PR and root waits required CI. This successor can close independently; #306/#349 TOOL-11 remain open until #400–#403 and this successor meet the full original accounting.

## Concrete disassembler failure reproduction

During #399 qualification at source e46bc0d1a7917de8c65204cdee931877aea671d8, the existing scalar fat-LTO atomics gate exited 0 after four wasm-objdump bad-magic failures among six counted objects (`/tmp/engine-399-wasm-atomics.log`). This confirms the already assigned consumer-status defect; counting objects does not prove decoding. The later scalar non-LTO supplemental check explicitly required all three archive families and three objects, successful extraction/decoding, and opcode-search status 1 (`/tmp/engine-399-scalar-object-report.json`); it did not repair or qualify the failed LTO inspection. #404 must retain both empty and plausible partial-clean disassembly failure cases and reject the old conditional pipeline. No scope expansion or completion is claimed.

## Bounded Wasm child #427

#427 now owns the independently useful Wasm object population and decoder/scan correctness outcome. Its explicit amended inspection contract uses a fresh gate-owned child target and scalar non-LTO build for the same engine/source/target_smoke families; this resolves the demonstrated fat-LTO decoder incompatibility without changing the shipped artifact or old opcode/observation policy. Complete checked producers/consumers and real-toolchain qualification remain mandatory. Root queues #427 after #412 to serialize workflow edits. This parent remains OPEN for every five workspace find populations, tracked-path pipeline, predicate/parser/filter and optional-result obligation above, plus #427 delivery. No obligation is waived and no implementation has begun.

## Workspace remainder executable scope

Root adopts the following bounded workspace-only implementation scope. #427 is separately source-approved and qualifying; its Wasm implementation and CI placement are not reopened here. This branch is planning only until #427 is delivered and Astra approves the actual merged base.

# #404 workspace-policy remainder — bounded implementation brief

Recommendation: implement the remaining workspace checker directly under amended #404 after #427 delivery. No further product split is necessary: all remaining operations feed one workspace-policy success verdict and one existing hermetic suite. #427 exclusively owns Wasm population/decoder/observation work and its CI step; do not edit or duplicate it. #404 remains OPEN until this remainder and #427 are delivered, while #306/#349 retain their other accounting.

Read the full parent #404 and approved/assigned #427 specs, current `scripts/check-workspace-policy.sh`, `scripts/test-workspace-policy.sh`, shared `scripts/lib/gate.sh`, and existing workflow suite invocation in `/home/bl/misofm/engine-404-plan` on supplied delivered main `39da065507beb822ef70a1552ff5dcc363938dd4` (use root's exact frozen identity before assignment). No implementation, builds, tests, timing, legacy source or Git/GitHub mutation. This document is a proposed stateless parent amendment, not an implementation assignment; #442 is the sole feature and #427 is active independent tooling.

## Product and exact scope

A workspace-policy success must mean every applicable original population was completely enumerated and every invoked original reader/parser/filter completed before its semantic predicate was accepted. Preserve the existing CLI/root selection, policies, patterns, diagnostics for genuine violations, manifest/comment interpretation and optional-result semantics. Add operation/status-specific execution-error diagnostics with original stderr/partial output, rather than reporting failed positive searches as missing content.

Edit only check-workspace-policy.sh, existing test-workspace-policy.sh and #404's decision/evidence record. The already-required CI runs this suite; no workflow change is needed. Existing shared helpers are sufficient for ordinary searches; explicit local checked captures are sufficient for compound/NUL producers. No helper change is justified by the inspected sites, and no generic harness/parser framework or new corpus is needed. Preserve existing executable modes. Keep the existing canonical LICENSE digest: it protects actual legal text, not new prose byte-pinning.

Source the real physical helper path before entering a fixture root, as today. Preserve all four required Cargo roots; never discard absent roots or create them to make a scan succeed. Temporary scratch belongs outside the inspected fixture tree so it cannot contaminate whole-tree file discovery. Do not rely on set-e, process substitution, pipeline conditional status, pipefail alone, `|| true`, or quiet-search early success as proof of producer completion.

## Five original find populations — retain each scope

| Population / current site | Exact membership | Successful empty | Required checked stages |
| --- | --- | --- | --- |
| npm manifests, checker :78–82 | `find . -name package.json -type f -not -path '*/node_modules/*'` | Valid | find, sort, each jq license predicate |
| npm locks, :84–88 | same search for package-lock.json | Valid | find, sort, each jq root-package license predicate |
| package manifests, :90–166 | `find crates hosts tools sidecars -name Cargo.toml -type f` | Invalid collectively; each individual root may contain none | four roots exist, find, sort, every manifest reader/extractor |
| retired directory stubs, :189–199 | whole-tree directories with the existing five retired names; prune only .git and target as today | Valid | find before interpreting rows; do not suppress its stderr |
| shallow fingerprint spills, final loop | existing depth-2 `.fingerprint` directories excluding ./target/*, existing `%P` output | Valid | find before interpreting rows |

Capture each population to completion once before its consumer loop. A nonempty list never overrides find/sort failure. Retain paths with spaces via the existing line consumer; do not introduce a new filename grammar. A valid empty list must not become a synthetic blank item that calls jq or a manifest reader. The Cargo minimum is one package manifest across all four required roots, not one manifest per root.

Retired-directory and fingerprint nonempty rows are intrinsically policy violations. They have no nonempty nonviolating payload. Their partial-output/error cases must verify the traversal-error status/diagnostic wins over early violation handling; do not fabricate a supposedly clean row or require an impossible unexpected-success mutant for these classes.

## Remaining producers, predicates and empty-result contracts

1. **License chain and positive required searches.** Keep required nonempty LICENSE/NOTICE/THIRD_PARTY_LICENSES.md/math-license artifacts. Capture/check sha256sum before its existing awk digest-field extraction, and check awk separately; require the existing digest equality. Check the workspace and fuzz Apache license searches, per-package license.workspace inheritance, third-party libm-reference search and conditional ISA target-table search with completed nonquiet rg invocations. Status 0 satisfies the exact existing predicate, 1 is its existing missing-content policy failure, >=2 is a distinct execution failure. Partial matching output followed by error must fail; do not credit `-q` success after a matching prefix.
2. **npm JSON predicates.** Keep the exact two `jq -e` expressions. Status 0 passes, false/null status 1 is the existing license policy failure; parsing/read/usage/system errors are execution failures, not another clean or optional-empty result. The files themselves are not optional after successful discovery. Do not redesign JSON validation or introduce extra fields.
3. **TOML name extraction.** Retain current line-oriented awk bodies for [package], [lib] and [[bin]] names. Capture/check each invocation before using its stdout; failed optional extraction cannot masquerade as no section. Package name remains required through existing directory/name semantics. A successful empty lib result and successful empty bin-name list remain allowed. Consume all bin rows only after successful extraction. Preserve the currently accepted primary/suffixed tool-bin naming rule, even if another repository statement phrases the broader naming goal differently. Preserve package-name/directory equality, sysroot/prefix/retired-codec bans. Derive dirname/basename using shell expansion or individually checked commands; neither is grounds for a new path policy.
4. **Tracked paths, including non-Git fixtures.** Preserve `git ls-files -z --cached --others --exclude-standard` as the Git population and the existing non-Git `find . -type f` fallback excluding .git/target. Git classification must distinguish actual non-repository absence from execution errors; never use arbitrary nonzero or an error substring to enable fallback. The delivered env-vocabulary check supplies the existing bounded model: exact ordinary not-a-repository classification, exit128, no explicit GIT_DIR/GIT_WORK_TREE override, with deterministic locale for that classification. A configured invalid repository selector or fatal/error with plausible output must fail. Check the actual Git listing/fallback find independently, preserving NUL bytes in a file through that stage, then individually check the existing NUL-to-newline tr, leading-./ sed normalization, basename-Cargo.toml awk filter and LC_ALL=C sort. Keep existing tracked/untracked-not-ignored scope and its any-depth nested-manifest coverage. Successful empty filtered list is permitted by this stage; the separate required Cargo population still prevents certifying an empty package workspace. Preserve today's explicit skip of non-file listed paths (e.g. tracked deletions); do not silently expand this into a filesystem-race policy.
5. **Per-tracked-manifest retired-codec scan.** Keep exact existing quote-aware line-comment stripping and the retired identity regex. Capture/check the file redirection/read and strip_toml_comments awk before invoking rg over the captured result. rg0 means the existing violation with its matching rows; rg1 means clean; >=2 means scan failure. Successful empty/comment-only stripped text is allowed. Failed awk or rg cannot be hidden by a successful clean downstream search. Preserve current comment behavior rather than introducing a TOML parser or escaping-rule repair.
6. **Optional .cargo ISA chain.** Missing .cargo remains valid. If present, check the original directives search, comment-row exclusion, approved-pin exclusion, conditional exact target-table positive search and global `[build]` negative search independently. Each collection/exclusion rg accepts 0/1 as predicate results and rejects >=2; complete clean absence/all-filtered output is valid. Any remaining non-allowlisted directive is the existing ISA violation. The required scoped target table is consulted only when non-comment ISA directives exist. The `[build]` scan is still required even with no directives: match is forbidden, absence clean, errors fatal. Do not change regexes or introduce a new requirement that every repository contain .cargo/config.toml.
7. **Already checked forbidden scans.** Preserve shared scan_forbidden consumers for Cargo ISA features, lockfile codec identity, compiled track caps, prelaunch generations and versioned worklet implementation names. Their source roots and existing 0/1/error contract remain. No migration of neighboring checker families or changed bans is needed.

Every producer/reader error fails regardless of empty, clean-looking or violating output. Binary/NUL streams must not pass through shell command substitution before decoding. Complete stdout and stderr should remain distinguishable for parser input; success stderr is not another data row.

## Finite directed evidence

Extend the existing valid fixture and suite; the baseline already supplies real required license artifacts, all four Cargo roots, correctly named library/bin packages and a lockfile. Keep all old policy mutations. Add optional valid npm/lock and ISA/nested-manifest files only when a case needs those later sites. No Cargo build is necessary for any hermetic case.

Maintain one small case table keyed to the operations above, with saved real tools executing every unselected operation. For each invoked failure-prone stage in the five populations and compound parser chains, exercise empty output/error and otherwise-valid output/error where that result class exists. Reuse the same tiny fixture and wrapper mechanism; this is a finite operation table, not a combinatorial corpus. Select later package/manifest invocations to prove previous successful rows do not make later failures invisible.

Required directed distinctions:

- Positive Rust-only root with empty npm/locks, no retired stubs/fingerprints and no .cargo; positive roots with approved ISA/comment-only optional results, no lib/[[bin]], and one empty Cargo root but a nonempty collective package population.
- Missing each required Cargo root (parameterized) and collective successful-empty package discovery must fail at discovery, after the valid license prelude. Never create a deliberately invalid license fixture as that proof.
- Git and non-Git positives; explicit invalid GIT_DIR and classification execution failure; listing/fallback failures; NUL conversion, normalization, manifest filter/sort failures. Preserve and test nested Cargo.toml coverage with a real non-ignored nested manifest. NUL producers must emit actual NUL bytes, not an invented newline listing.
- Required search clean-missing versus execution-error diagnostics, including complete matching payload/error; optional lib/bin empty success versus extractor error; per-manifest comment-strip failure and retired scan failure with clean content; existing quoted-name/comment semantics.
- .cargo empty/comment-only/approved-pin positive, forbidden directive and `[build]` negatives, and each source/filter/table-search failure, including a late global-build search error after earlier clean results. Distinguish rg1 from >=2 explicitly.
- SHA producer/extractor and jq predicate/read failures remain distinct from content mismatch, using valid canonical digest/valid license result payloads for the positive-looking error variants.

For faults whose real successful result is empty (including retired/fingerprint discovery and exclusion-to-empty), empty/error is the otherwise-valid failed result. A nonempty violation/error case may additionally prove error precedence; label it honestly. For clean manifest/npm/path rows, prefer delegated real valid output, including a complete-looking list before an injected nonzero exit. A fake path or wrong package name that already fails policy cannot prove status-loss resistance.

New assertions must require the intended operation, injected exit status and sentinel stderr. Give unexpected checker success a distinct assertion exit (e.g. 97), separate from setup/missing-diagnostic failure. Execute two bounded actual production counter-controls: (a) swallow a selected clean-looking manifest/npm population producer failure, (b) swallow a selected late parser/filter/scan failure whose ordinary output permits the whole valid fixture to pass. Verify the exact intended production mutation and require the SAME targeted unexpected-success assertion to reject each mutant. Do not count missing fields, syntax errors, failed fixture setup or an arbitrary nonzero harness result. No extra shared-helper mutant campaign is required when that helper is unchanged.

## Delivery and accounting

After focused source PASS, run syntax/diff hygiene, actual workspace policy, the expanded existing suite and unchanged helper suite. At the coherent delivery boundary retain the inherited unchanged-count full-workspace comparison; shell-only work does not warrant new Rust tests or browser/artifact rebuilds. #427 owns its one real scalar non-LTO Wasm qualification and its own hermetic suite. Do not rerun or redesign that work for this remainder.

Root should amend/synchronize #404 with this exact workspace-only execution scope and #427 ownership, then obtain Astra numbered/frozen-base approval after #427 delivery before assigning the next tooling attempt. Keep Luna1/Sol2–3 only after FAIL and hard stop after three failures. Actual PR Astra review and required CI precede merge/closure. A completed #404 closes its five workspace populations, tracked-path/parser/predicate obligations and delivered #427 accounting; it cannot close unrelated #306/#349 children.

Only this `/tmp` brief was written. No source/spec changes, Git/GitHub mutations, tests/builds or timing were performed.

## Root scope and sequencing decision

Keep this cohesive remaining checker repair directly in #404; no new successor or framework is needed. The exact five populations, producer/predicate/parser and optional-empty contracts and finite directed cases above are binding. The task repairs completion/error handling without expanding or weakening existing semantic predicates; any separate naming-policy discrepancy with the standing agent guide must be reported for a distinct ruling rather than silently changing the scanner contract. Preserve canonical license text validation and existing legal artifacts.

Do not modify the current qualifying #427 worktree, checker, suite or CI leg. After #427 delivery, integrate the actual default branch into this dedicated workspace-discovery branch, confirm the stateless numbered body and source base with Astra, then assign Luna attempt 1. Root owns checkpoints/pushes/remote synchronization, Astra each consolidated verdict, Sol attempts 2/3 only after FAIL, then hard stop/rescope. #404 closure requires both this full workspace remainder and delivered #427; #306/#349 retain their other obligations. No source implementation, build, artifact or benchmark is authorized by this planning checkpoint.

## Delivered Wasm child and current implementation boundary

PR #451 delivered #427 as main `5a4a7d2071194cf6118241e24d073824668e3387` after actual-head Astra PASS and required CI SUCCESS. #427 is verified CLOSED. Root integrated that delivered base, preserving this parent amendment and the complete delivered child evidence in the document merge conflicts. This branch differs from main only in the #404 scope record. The workspace remainder is ready for Astra numbered/current-base approval; source implementation is still unassigned. #430 remains the sole runtime feature in its isolated worktree.

## Numbered current-base approval and Luna attempt 1

# Astra numbered/current-base review — #404

PASS for Luna attempt 1 at `2d57d4efd2881d28af805271a3d21fd53159fccc`, based on delivered main `5a4a7d2071194cf6118241e24d073824668e3387`. This is scope/base approval, not implementation or qualification acceptance.

I checked the complete amended parent, the adopted `/tmp/astra-404-workspace-remainder-brief.md`, current workspace checker, existing fixture and helper seams, and required CI invocation. The worktree is clean and the cumulative delta from delivered main is only the #404 spec (81 added lines). The adopted brief is present verbatim. Live GitHub #404 is OPEN with matching title and exact body; #427 is CLOSED.

The current source still has the five identified populations and the specified unchecked license, manifest extraction, tracked-path/comment-strip and ISA chains. Membership and existing predicates in the brief match those sites. Empty npm/lock discovery, lib/bin extraction and optional ISA results remain valid; Cargo discovery requires a collectively nonempty population across four existing roots, not a manifest per root. Retired-directory/fingerprint output is intrinsically violating, so its error precedence proof correctly does not demand a fictitious nonviolating row. The complete Git/NUL conversion/filter/sort chain and late parser/search sites remain explicitly assigned.

The root adoption is faithful and resolves the historical sequencing text through the final delivered-child/current-boundary section. For the forthcoming assignment the controlling checkout/base is the one above, not the older planning checkout or earlier active-feature descriptions quoted inside the historical brief. Only `scripts/check-workspace-policy.sh`, `scripts/test-workspace-policy.sh` and #404 evidence/spec are authorized. No Wasm checker/suite, workflow or shared-helper change is needed. The existing workflow invokes the workspace suite at qualification.yml:289, and that suite already invokes test-gate-lib.sh at its end. Checked local captures can preserve separate stdout/stderr and NUL streams where existing ordinary-search helpers do not provide those exact representations; do not feed merged helper diagnostics into parser input or expand the helper API for this slice.

The finite directed table and two actual production counter-controls are sufficiently explicit: otherwise-valid real payloads, late-stage selection, operation/status/sentinel diagnostics, and the SAME unexpected-success assertion distinguish status-loss acceptance from setup or unrelated policy failure. Existing semantic tests and executable modes remain. Retain the current primary/suffixed bin-name predicate in this status repair; this approval does not resolve or waive the separate standing naming-policy discrepancy.

No additional amendment is needed before assignment. Luna gets one coherent pass; any failed verdict routes the bounded retry to Sol, with the existing three-attempt hard stop. Focused acceptance, inherited full-workspace comparison, actual PR Astra review and required CI remain outstanding. Closing #404 will require this complete workspace remainder plus already delivered #427, without closing unrelated #306/#349 obligations or rerunning Wasm/artifact/browser qualification.

Review was read-only apart from this /tmp report. No tests, builds, timing or repository/GitHub mutations were performed.

Root assigns Luna attempt 1 on this approved delivered base. One coherent implementation pass is authorized in the two named workspace scripts and this spec, followed by a root checkpoint and Astra verdict. No further implementation may layer onto a ready checkpoint before root commits and pushes it.

## Luna attempt 1 implementation evidence

The workspace checker now captures each find/sort population before consumption, checks producer
and parser statuses, and preserves stdout/stderr in operation-specific diagnostics. The Cargo
manifest population requires all four roots to exist and at least one discovered manifest. The
tracked-path chain checks Git classification, Git/non-Git listing, NUL conversion, normalization,
manifest filtering and sorting before scanning quote-aware stripped manifests. Optional ISA
searches distinguish clean status 1 from execution errors, including the late global `[build]`
search. Existing primary/suffixed tool-bin naming semantics and all prior policy predicates remain
unchanged.

Validation from the dedicated worktree, with `PATH=/home/bl/.cargo/bin:$PATH`:

* `bash -n scripts/check-workspace-policy.sh scripts/test-workspace-policy.sh` — PASS.
* `bash scripts/test-workspace-policy.sh` — PASS (`workspace policy mutation tests: ok`; shared
  gate-library tests also PASS).
* `bash scripts/check-workspace-policy.sh .` — PASS (`workspace policy: ok`).
* `git diff --check` — PASS.

This is an implementation checkpoint awaiting Astra source review. No commit, push, GitHub mutation, build, benchmark,
Wasm qualification, or browser/artifact rebuild was performed in this attempt.

Root checkpoint audit: the attempt changes the checker and this record only; the existing workspace suite has no delta. The reported existing-suite positives are not evidence that the new finite directed failure table or two actual production same-assertion counter-controls have been implemented. No source PASS or complete acceptance is claimed. Astra will issue one consolidated verdict against the full frozen scope.

## Astra attempt 1 verdict and Sol attempt 2 assignment

# Astra #404 Luna attempt 1 — FAIL

Exact checkpoint: `70ff15b26558477fdf5b97b2cdc9dd1a850c1f46`, `/home/bl/misofm/engine-404-workspace`, against approved scope/base `2d57d4efd2881d28af805271a3d21fd53159fccc` and the complete adopted workspace remainder brief.

FAIL. Capturing the main external operations separately is useful progress, but several concrete acceptance defects remain and the mandatory new directed suite/control evidence is entirely absent. These four finite groups define one coherent Sol attempt 2; do not broaden policy, change the shared helper/workflow, or duplicate delivered #427.

## 1. Correct required population and unchanged empty-name semantics

`capture cargo-manifests-sort sort <<<"$cargo_manifests"` writes a newline even when successful discovery returned zero paths. The following `[[ -s "$CAPTURE_OUT" ]]` therefore passes. The consumer skips that blank line, so a successful empty Cargo population can reach success: file byte size is not proof of a manifest row. Require the actual nonempty population before consuming it, while retaining optional empty npm/locks and requiring all four original roots. Do not impose a manifest per root.

The new bin loop also skips every empty row. Successful extraction of NO bin rows is valid, but the existing extractor prints an empty row for an explicit `name = ""`; the old predicate rejected that row. Preserve that distinction by consuming the actual captured extractor file without manufacturing/ignoring a blank name. Do not change the original line-oriented TOML grammar or introduce a new parser. Keep package/lib and primary/suffixed bin policy as frozen.

## 2. Complete positive searches and Git classification

Every required_search call still supplies `-q` (`-qx` for license checks, `-q` for inventory and scoped ISA table). Checking rg's returned status does not prove complete search when quiet mode can stop after a match. The brief expressly requires nonquiet completed positive searches. Remove only quiet behavior while preserving each exact existing expression and whole-line `-x` where present; distinguish predicate absence1 from execution failure>=2 with full status/diagnostic evidence, including a matching valid payload followed by error.

Git fallback is not the frozen delivered environment-checker classification. It uses `grep -Fxq` to find one matching line in stderr, without deterministic locale or rejecting explicit GIT_DIR/GIT_WORK_TREE overrides. An error containing the ordinary message plus another fatal line can enable fallback; an explicit repository selector must never be treated as ordinary absence. Use the already-approved exact ordinary exit128 classification with no explicit overrides and exact diagnostic content under deterministic locale. Check any external classifier operation or use shell equality; do not hide classifier errors as a different clean mode. Preserve ordinary Git and non-Git fixtures and the existing tracked-deletion skip.

Restore the original `LC_ALL=C` for the tracked manifest sort. Source and parser stdout/stderr must stay separate throughout the NUL-to-lines, normalize, manifest-filter and sort chain.

## 3. Finish explicit local producer handling without dead duplicate paths

`package_directory="$(basename "$(dirname "$manifest")")"` remains an unchecked nested producer chain. A failed dirname can be masked by a successful basename; both are explicitly named in the frozen scope. Use equivalent shell path expansion or individually checked commands, preserving the current path policy.

The new checked_find and tracked_paths functions export captured data with unchecked `cat`, and their caller assignments rely on surrounding errexit for rejection. Retain explicit caller-status handling for these local compound producers (or consume their captured files directly), rather than claiming producer completion from set-e. Preserve original stdout/stderr/status if an invoked reader fails. Keep scratch outside the inspected tree, as required by the brief; `${TMPDIR}` must not silently place the gate's own discovery intermediates under the fixture being enumerated.

Remove the disabled `if false` duplicate npm/Cargo implementation and unused checked_sort/checked_rg or obsolete duplicate stripping code when consolidating this repair. This is local completion of the single checker, not permission for a generic framework. The old explanatory policy comments should remain attached to the actual predicates, not stranded inside disabled code. Existing scanner regexes, legal artifacts, root scope, optional ISA interpretation and helper forbidden scans remain unchanged.

## 4. Implement the already-frozen finite fixture table and two SAME-assertion controls

The only changes are checker/spec. The unchanged workspace suite (including root's independently observed exit0 `/tmp/engine-404-root-attempt1-suite.log`) preserves prior behavior but proves none of the new operation table or actual production counter-controls. Extend the existing suite and its valid fixture; no new corpus/helper campaign/workflow is required.

Use the binding table in the adopted brief as the finite inventory: all five find populations and their sort/readers; required roots and collective-empty Cargo; SHA/digest and jq truth/read errors; each required positive search; package/lib/bin extraction including later manifests; Git classification, NUL listing/fallback, conversion/normalization/filter/sort; per-manifest comment reader and retired scan; optional ISA source/comment/allowlist/conditional table and late global-build query. For each invoked failure-prone stage exercise empty/error and otherwise-valid complete-looking output/error when that result exists. Retired/fingerprint nonempty rows are inherently violations; test error precedence honestly, not a fictitious valid nonempty row.

Retain Rust-only/no npm, optional lib/bin absence, optional .cargo absent/empty/comment-only/approved-pin positives; Git/non-Git and nested tracked-manifest coverage; precise missing-root and empty-Cargo diagnostics after valid licenses. Preserve explicit empty-name rejection described above. Inject actual NUL bytes for NUL producers. Use saved real tools for unselected calls, target later consumers, and demand the intended operation, injected status and stderr sentinel. A broad any-nonzero assertion is insufficient.

Execute the two authorized disposable mutations of the actual production checker: swallow a selected otherwise-valid population failure, and swallow one late parser/filter/scan error whose ordinary output permits the fixture to pass. The SAME original targeted assertion must reject each at the distinct unexpected-success status (e.g.97), not setup/missing-diagnostic failure. Retain exact patches, commands, statuses and restored positive evidence. No extra shared-helper mutants are needed because it is unchanged.

After completing this single coherent pass run the affected existing suite, actual checker, unchanged helper suite, shell syntax and diff hygiene; record failures and final source identity honestly. Full-workspace delivery comparison and actual PR/required CI remain after source acceptance. No Wasm rerun, Rust tests, artifact/browser work or timing is part of this revision.

## Accepted boundaries

The active implementation separately captures the main find/sort/SHA/awk/jq/ISA operations and correctly rejects many nonzero statuses before loops; optional empty npm populations are skipped rather than passed as fabricated jq filenames. Retired-directory/fingerprint traversal status is checked before policy interpretation. The shared helper and CI are unchanged. Preserve that narrow approach while correcting the concrete holes above.

Review was source/diff/spec inspection only. Root's retained existing-suite result is credited only as baseline regression evidence. No tests, builds, timing, source changes or Git/GitHub mutations were performed by this reviewer. Only this /tmp verdict was written. Luna attempt1 is now one FAIL; Sol attempts2/3 remain under the existing hard stop. #404/#306/#349 remain open and delivered #427 is not reopened.

Root assigns Sol attempt 2 for these four finite correction groups under the original complete workspace scope. One coherent pass and one adversarial verdict; attempt 3 remains available only after a FAIL, then hard stop/rescope. No further Luna revision is authorized under the current user workflow. Root owns the exact-path checkpoint and push before additional work.
