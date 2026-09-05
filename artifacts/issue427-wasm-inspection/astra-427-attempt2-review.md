# Astra #427 Sol attempt2 source review — FAIL

Exact checkpoint: `aff69a77895bb0cb409cde71f4d5cbba859faf74`, `/home/bl/misofm/engine-404-plan`.

FAIL. Two finite groups remain for the one final Sol attempt3. Most prior corrections are accepted; do not reopen them or build a new framework. A third FAIL requires hard stop/rescope. Real Cargo qualification remains after source PASS.

## 1. Restore relative/default target correctness

The original realpath step was removed without replacing its path resolution. `target_directory` and `inspection_target` remain relative for the preserved default or the actual CI argument `target/ci/wasm-scalar` (checker lines3–6). find consequently emits relative archive paths, which are retained unchanged at line44. Listing works from the repository root, but line60 changes into the family scratch directory before `ar x "$archive"`; that archive is then resolved under scratch rather than under the original repository working directory. The real default and CI invocations therefore fail extraction despite correct archives.

The hermetic `assert_case` always passes `$test_root/target-$name`, an absolute path, so its green result does not exercise the preserved CLI. Normalize the caller target/owned child to an absolute path once on the control plane with explicitly checked resolution, or otherwise retain an absolute archive path before changing directory. Do not reintroduce the unchecked array command substitution. Keep source-fallback paths rooted at the original checker working directory and preserve caller-cache ownership.

Add faithful hermetic positives for an explicit relative target (including the CI-shaped path) and omitted default argument from a disposable fixture working directory with the minimal source fallback only when required. Do not execute them in the real workspace default target. Both must reach real ar extraction under the existing fake build, not stop at a mocked successful extraction. Keep the existing absolute positive. This tests the original CLI contract, not an expanded path API.

## 2. Complete the small remaining frozen directed cases

The existing finite table now covers most required classes, but three omissions from the original/attempt1 frozen contract remain:

- Archive and object in-place sort injections simply return nonzero without touching the pre-existing valid list. Those are valid complete-looking-result/error cases, but there is no zero-result/error counterpart. Add the complementary case that empties the sort output file before returning its operation-specific error/sentinel. The separate archive-member sort is checked in production but has no injected failure case at all; cover its empty and delegated valid-looking output/error forms. Preserve explicit operation/status/sentinel assertions.
- `cfg-error` emits the plausible pointer-support row/error; add its empty-output/error counterpart as already requested for cfg production. This must fail cfg production rather than the later missing-pointer predicate.
- The caller-cache preservation fixture now has a real parent sentinel, which is useful. It still lacks the originally required stale unrelated/fat-LTO archive outside the fresh child. Add one tiny deliberately non-inspectable stale archive under the caller cache beside that sentinel, require the positive inspection to ignore it, and verify it remains unchanged. No real toolchain or second corpus is necessary.

These are a finite completion of the already-frozen producer/profile-isolation table, not a request for exhaustive new branch coverage. Keep paired output/error cases meaningful: inplace sort data lives in its `-o` file, while member-sort data is stdout. Fake invalid paths/content that independently fail policy are not substitutes for clean-looking producer payloads.

## Accepted work and controls

Real library family spelling is fixed to engine/source/target_smoke while Cargo package target-smoke remains correct. The fake build checks the exact scalar/non-LTO command and environment and creates faithful library names. Discovery and sort have individual status capture. The expected object list is now required nonempty, duplicate detection is restricted to `.o` members, and exact sorted expected/discovered identities are compared before consumption. The previous unchecked sort-u/wc duplicate pipeline is gone. Missing and extra extraction cases are tested.

Required cfg and source searches are nonquiet with explicit predicate/error distinctions. Producer errors now name operation, status and identity and emit captured partial stdout/stderr before scratch cleanup. Decoder errors cannot be masked by clean opcode scans. Successful empty decoding remains allowed. Object observation searches continue after earlier matches and retain exact returned-status handling; genuine source fallback and source-not-needed positives now discriminate those paths. The opcode fault targets the decoded opcode scan instead of the earlier cfg search. The real-find delegation now preserves unselected failures.

The two causal controls now execute the reusable targeted assertion against uniquely verified actual production mutants. The originals fail at the intended decoder/archive-discovery operation and sentinel; each mutant swallows only that selected status check, permits the otherwise-valid checker run to succeed, and the SAME assertion returns the distinguished unexpected-success status97. The retained exact diffs/log output supports this; setup/wrong-diagnostic failures use96 and cannot satisfy the control. This closes the previous false printed-label proof gap. Retain these controls without adding another mutation campaign.

The suite and proportional policy/syntax/diff records pass on their tested absolute-path setup. The fresh owned child, existing CLI names, exact three-family scope, opcode policy, optional observation fallback and narrow existing CI step remain within scope. No helper, workspace-policy, Rust or artifact redesign appears. Source PASS must still precede one real available-toolchain scalar non-LTO inspection, followed by actual PR Astra/required CI delivery. #404 retains its workspace remainder and #306/#349 their other obligations.

Read the full current #427 contract/attempt record, prior verdict, final checker/suite and `/tmp/sol-427-attempt2-suite.log` plus policy evidence. No source edits, Git/GitHub mutations, real Cargo/builds or timing were performed. Only this `/tmp` verdict was written. The relative-path failure follows directly from the two working directories and retained relative archive names; no real build is needed to establish it.
