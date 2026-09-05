# Astra numbered scope review: #456

**PASS for queued scope; implementation still waits for #455 delivery, actual-base confirmation and root assignment.** Reviewed stable delivered `aba905c0a5ae0bc747a65d1052ba76811fcee3c5`. The new stable `engine-456-plan` checkpoint `36704c31` changes only parent #403 accounting; it does not change this checker, suite or helper. Preserve that newer parent record. This is scope approval, not source acceptance or qualification.

The numbered outcome remains one small, independently closable static fixture-inspection fix. No split, new helper API, workflow change, runner repair, fixture/seal refresh or timing authority is needed. Only `scripts/check-rack-benchmark-fixture.sh`, `scripts/test-rack-benchmark.sh` and numbered evidence are implementation paths. Copied sibling context in the spec does not authorize the interchange/108 work mentioned there.

## Frozen policy and operation inventory

Preserve optional `FIXTURE_ROOT` as the fixture directory, the at-most-one-argument CLI and usage status 2. Manifest and payload must each remain regular, non-symlink files. Discovery remains immediate-child `find -type f`, sorted to exactly `MANIFEST.tsv workloads.toml`; unrelated directories/symlinks or other nonregular entries are not newly prohibited. The existing diagnostic's broader wording is not authority to change selection semantics.

Preserve manifest SHA `2d6b8c4b11bb00a17185d7777300194bf53ab30d86cf581a55886f07c5273985`, exact tab header, two lines, existing tab-separated `read` grammar and numeric/64-lowercase-hex predicates. Payload length stays 456 and SHA stays `1f67ed9960e5a6728f02442b65af70704957d5f6056865d8b44555637273188d`. Preserve the three exact fixed rows (observations 1000, sample rate 48000, quantum 128) and existing anchored workload-name expression/count 3. No TOML interpretation or pin regeneration.

Individually complete and check both manifest/payload `sha256sum` producers and their `awk` extraction; `find` and `sort` before consuming discovery; header `sed`, manifest `wc -l`, record `sed` and subsequent builtin `read`; payload `wc -c`; each required `grep -Fqx`; final `rg -c`. A successful consumer cannot erase an earlier producer failure. Required predicate no-match remains refusal, distinct from tool failure. In particular successful quiet grep has empty stdout: do not impose a generic nonempty-output rule on predicates. Header, record, digest and count captures must satisfy their existing concrete predicates, not merely be nonempty. Explicit status propagation must work when functions are called conditionally; `set -e`, process substitution and negation alone do not establish it.

The existing shared library is `scripts/lib/gate.sh`. Its checked find/sort/search helpers may be reused where their flags and output contract fit; small local checked captures are sufficient for hash, record and count operations. No library edit is justified. Any source path must resolve from the physical checker, not the fixture directory.

## Finite proof and two controls

Retain the actual sealed positive and existing corruption, unlisted-file and missing-file tests. Missing fixture root is legitimately intercepted by the earlier manifest check: report that exact refusal and additionally exercise discovery failure with an otherwise valid manifest, rather than claiming deletion reaches find.

Cover the spec's named operations selectively, including later payload hash/count, later required literal and final workload count after preceding checks succeed. For find/sort, include both empty output plus error and the complete real two-name output plus error. For required read/hash/count captures, use the correct actual output plus error and retain required-empty/malformed refusals. Verify any delegating shim's underlying real command succeeded and its complete output matches the baseline before appending the injected status/stderr sentinel; do not fabricate a plausible digest or count. `read` completion and field predicates must be explicit, including failure to obtain a complete record. No giant new mutation campaign is needed.

Freeze the two representative actual checker mutations as follows:

1. Remove only the checked discovery producer's status propagation, keeping complete correct two-name output; run the same original find-failure assertion.
2. Remove only the late payload `wc -c` status propagation, keeping its real correct byte count; run the same original payload-count failure assertion.

Original and restored checker must pass the suite; each mutant must reach that assertion's named **unexpected checker success** branch with status **97**. Wrong operation/status/sentinel/payload, syntax errors, missing tools or unreachable setup must yield distinct **96**, never mutant credit. Existing unrelated validator mutations stay intact; they do not replace these controls.

## Existing execution seam

Required `qualification.yml` already invokes both the real fixture checker (fixture-integrity step) and `test-rack-benchmark.sh` (benchmark validator mutation step). No additional CI call is required. Run proportional shell syntax, the real checker and that affected suite after implementation; root retains the existing workspace/actual-PR/required-CI delivery boundary.

The existing suite's lifecycle section builds a scratch repository containing fake cargo/git/rustc and a synthetic emitter, then executes the copied runner with scratch `PATH`. Keep this isolation and verify the selected fake commands before lifecycle invocation. Its early invalid-argument runner checks must continue to refuse before launching anything. Do not run a real valid runner invocation, benchmark or preflight as evidence for this static checker change.

No tests, builds, timing, repository edits or GitHub mutations were performed for this review. Parent #403/#306/#349 closure remains governed by delivery of all outstanding children.
