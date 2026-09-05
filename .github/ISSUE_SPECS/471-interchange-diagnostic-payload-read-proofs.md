# Complete interchange diagnostic payload and late authority-read proofs

Parent #455, #403/#306 and audit #349 TOOL-11. This is a new bounded proof-completion outcome after #455's three-attempt hard stop, not permission for a fourth #455 repair. Preserved source: `c7172f12e13f07c00edbe1c50309a52b94c00a19`. Root must number/synchronize this body and reciprocal parent retention, freeze the actual integrated base, and obtain Astra scope approval before assigning Luna1; Sol2/3 fallback and a new explicit hard stop apply to this bounded child.

## Smallest closable outcome and exact paths

Complete exactly two previously required evidence surfaces against the accepted production checker implementation:

1. Complete diagnostic stdout payload equality, including duplicate multiplicity.
2. One actual late standalone108 Python cross-file authority read failure.

Only `scripts/test-effect-interchange-policy.sh`, `scripts/test-effect-interchange-benchmark-108-policy.sh`, this numbered spec and reciprocal parent evidence may change. Production checkers, shared helper, CI wiring, actual/fake runner implementation, validators, fixture pins and all other directed table rows/controls are frozen. No new producer matrix, production mutation, benchmark framework or corpus. Preserve existing historical failed attempts and two actual namespace/migration original/mutant/restored controls.

## A. Isolate and compare the complete payload

The existing producer wrapper independently captures real stdout/stderr, checks the actual expected status and shape, then forwards complete stdout or suppresses it according to the existing mode. Keep that mechanism and original input/argv/occurrence selectors.

Replace the current `Counter(actual)[row] >= Counter(expected)[row]` containment check with equality on a precisely bounded producer-output portion. The checker emits, in order, captured producer stdout, captured producer stderr plus the injected sentinel, then its operation/status diagnostic. In the suite, use the independently captured delegate stderr and uniquely injected sentinel plus the exact final operation/status line as the only removable framing. Validate that this framing occurs exactly where expected and exactly once; do not strip all lines matching a generic regex, filter to known expected rows, use set/uniq, ignore extras, or silently discard diagnostics. When an existing case has earlier fixed diagnostic output, identify that exact known framing from its clean fixture rather than accept arbitrary prefixes. Required tools' successful stdout may be empty, and their actual stderr must remain independently accounted.

A straightforward bounded implementation is a local Python assertion in the existing suite which accepts the captured delegate stdout/stderr, combined failure log, exact sentinel and exact operation/status diagnostic. It validates/removes only that known suffix/framing, splits the remaining payload into lines, and compares `Counter(actual_payload) == Counter(expected_stdout)`. Preserve newline handling consistently with the actual captured bytes, and preserve duplicate row counts. The checker failure status and exact operation/status/sentinel checks remain independent requirements. For error-only mode expected forwarded stdout is empty; the real delegate must still be executed/validated before suppression.

Exercise this SAME local assertion with its real valid captured diagnostic, then three tiny modified copies of that diagnostic: duplicate one real payload row, append one additional distinct payload row within the payload portion, and remove one real payload row. Each must fail payload equality at the intended assertion, not syntax/setup/framing. Retain a real nonempty capture to make these controls nonvacuous. These are assertion-level controls only, not additional production checker mutants. Preserve order-insensitive acceptance by one reversed-row payload using the same complete multiset if the chosen real capture contains multiple distinct rows; do not manufacture a new workload or producer case for this.

## B. Actual second Python authority read refusal

Use the existing small standalone108 scratch fixture. Its first benchmark-source validator must pass, and the required validator/checker files must remain present, regular and non-symlink so the pre-Python required-path loop passes. Target the SECOND Python invocation's read of `scripts/effect-interchange-benchmark-108-validator.py`; do not target the first source validator or claim the existing deleted-file refusal exercises this path.

Freeze a deterministic local Python shim for this test: delegate the first Python invocation unchanged and check its actual success; at the second invocation, capture the original stdin program into the scratch directory and execute that unchanged program with its original argv through the real interpreter, using a narrow test bootstrap that replaces `pathlib.Path.read_text` only for the exact resolved target file to raise `PermissionError` naming that path. All other reads delegate to the original method. Preserve `-I -B` behavior and the original script's argv shape. The failure must arise when the unchanged cross-file validator actually calls read_text for that file; the shim must not merely exit1 or print a forged traceback. A marker set by the failing read hook proves the intended read was reached. No production checker change or new public injection API is authorized.

The case must assert checker status1, exact `cross-file output authority validation failed (status 1)`, actual PermissionError/path evidence and reached-read marker. Unexpected checker success must produce named97; wrong status/operation/path, first-validator failure, missed injection, missing helper/tool or other setup must produce96. Run the unmodified standalone fixture before and after to establish clean success and restore the shim environment. This is ONE added existing-path case; retain existing standalone tool-status injections, missing-required-file cases, optional-directory cases and namespace control unchanged.

## Focused execution and evidence

At the actual frozen worktree root, with available Rust tooling PATH recorded for the existing fake-only lifecycle setup:

- `bash -n scripts/check-effect-interchange-qualification.sh scripts/check-effect-interchange-benchmark-108.sh scripts/test-effect-interchange-policy.sh scripts/test-effect-interchange-benchmark-108-policy.sh scripts/test-effect-interchange-benchmark.sh`
- `bash scripts/check-effect-interchange-qualification.sh .`
- `bash scripts/check-effect-interchange-benchmark-108.sh .`
- `bash scripts/test-effect-interchange-policy.sh .`
- `git diff --check` for source/prose (preserve verbatim raw logs separately).

The parent policy suite already invokes standalone108 policy and the existing hermetic lifecycle suite at its end. Verify those inherited fake cargo/git/emitter paths and recursion guard before running the parent; do not duplicate a real runner invocation. Record the existing child suite completions from that parent run, or run a child separately only for focused diagnosis. No real benchmark, reference-process campaign or preflight/runner main is authorized. Preserve exact command/status/log provenance and actual two historical production mutation diffs/results.

## Acceptance and closure

Astra must inspect the two new proof mechanisms and unchanged inherited table/control semantics. On source PASS, root freezes the integrated immutable candidate and retains #455's complete workspace, actual pushed-PR Astra review and required qualification SUCCESS before merge. This child and #455 can close together only when the entire inherited contract and remote evidence are complete. Parent #403/#306/#349 and unrelated siblings remain open. No runtime/artifact/browser regeneration or measurement authority arises from these two suite changes.

## Numbered delivery checkpoint

GitHub #471 has this exact title and scope. Preserved integrated base is `d23ee9f70292950b5de516e645bddac33dcf0fe0`; #455 remains open after its three-attempt hard stop. This child must receive numbered Astra scope approval before Luna attempt 1. No implementation is included in this checkpoint.

## Astra numbered scope approval and attempt 1

# Astra #471 numbered scope review

**PASS for numbered bounded scope/base; root may assign Luna attempt 1.** Reviewed clean pushed `2bceae9597689ad02dea16462c1b3d500509b245`, `engine-471-proof`, and the complete numbered471 body. Compared with preserved accepted-progress source `c7172f12e13f07c00edbe1c50309a52b94c00a19`: only parent455 retention and the new471 spec differ. No implementation drift is present. Root reports exact remote number/title/body synchronization; this read-only local review does not independently claim a GitHub query.

The body faithfully adopts `/tmp/astra-455-proof-successor-brief.md`. It is exactly two suite proof completions, not a fourth #455 repair: bounded diagnostic payload Counter equality after only exact known framing removal, with duplicate/extra/missing assertion controls; and one deterministic actual second-Python read-hook refusal, retaining unchanged validator program/argv, first-validator success, required regular files, actual PermissionError/path/reached marker and precise97/96 assertions.

The original two production namespace/migration controls and all accepted directed rows remain frozen. Production checkers/helper/CI/runner/validators/pins do not change. Existing parent-suite CI wiring covers both108 and fake-only lifecycle children; no workflow or benchmark authority is needed. Focused commands and attribution requirements are executable as written. Scratch Python support is confined to the existing suites, not a new repository framework.

#455 remains open with full inherited workspace, actual-head Astra PR review and required qualification SUCCESS until combined delivery. #403/#306/#349 and unrelated siblings remain open. Root retains checkpoint/remote mutation ownership; Luna1 then Sol2/3 and hard stop apply to this newly bounded child.

No tests, builds, timing, repository edits or Git/GitHub mutations were performed.

Root assigns Luna attempt 1 in `/home/bl/misofm/engine-471-proof` only. Root owns commits and GitHub. Pause after the coherent focused-green tranche for exact-path checkpoint; no additional pass may be layered before checkpoint.

## Luna attempt 1 focused evidence

The existing producer assertion now removes only one exact sentinel plus the final operation/status
line, and independently removes the captured delegate stderr before comparing
`Counter(payload)` with `Counter(expected)` for equality. Duplicate, extra, missing and (when the
capture has multiple rows) reversed-row assertion controls use the same real nonempty capture;
the first three fail at payload equality and the reversed multiset passes.

The standalone108 suite retains its existing Python status injections and adds one actual second
invocation hook. The first invocation delegates unchanged and must succeed; the second captures
the unchanged stdin program and original script arguments, runs them under `-I -B`, and replaces
`Path.read_text` only for the resolved validator path. The hook records a reached marker and raises
`PermissionError` naming that path. The suite requires status 1, the exact cross-file diagnostic,
path and marker evidence; unexpected success is 97 and setup/status/diagnostic mismatch is 96.

Focused commands and statuses are retained in `/tmp/471-luna1-{syntax,qualification,108,108-policy,policy}.{command,log,status}`. With `PATH=/home/bl/.cargo/bin:$PATH` where required, all five statuses are 0. The parent policy run invokes the existing standalone108 and fake-only lifecycle children. No real runner, timing, build, or Git/GitHub operation ran.

## Astra attempt 1 verdict and Sol attempt 2 assignment

# Astra #471 Luna attempt 1

**FAIL — bounded Sol attempt 2 required.** Reviewed exact `aa9cd8298a4f64acf2e5575a43f1866c672a12fa`, `engine-471-proof`, against the complete numbered child. Only the two suites and evidence differ; production and inherited two guard mutants remain unchanged. No tests/builds/timing or repository/Git/GitHub mutations were performed.

The actual second-Python hook follows the approved route: first invocation delegates unchanged; second captures the original stdin validator, preserves -I/-B and reconstructs original argv, and raises PermissionError only when that program reads the exact resolved validator path. Actual checker status1, cross-file operation/path/PermissionError and reached marker are checked. This is real targeted read execution, not a forged process exit. Preserve it. Add the expressly required clean standalone run after removing the shim; the current sequence moves directly into deletion/fault cases rather than explicitly proving restored clean success.

The payload implementation now uses Counter equality and retains duplicates, which fixes the original containment defect. However these finite child obligations remain:

1. **Exact framing:** the assertion accepts any final line containing the operation substring and `(status 73)`, then treats that observed line as trusted framing. It never uses the computed exact `diagnostic` variable. Require the actual fixed full operation/status line, exact sentinel and independently captured stderr suffix, not self-selected framing. Keep the check active for both existing complete and error-only modes: currently it is entirely inside `if mode == complete`, so injected empty-mode diagnostics are not checked for an empty forwarded payload.
2. **Same diagnostic controls at the intended assertion:** duplicate/extra/missing controls currently alter the expected payload while leaving a hand-assembled diagnostic constant. The numbered brief requires keeping the independently captured expected payload fixed and modifying copies of the actual valid diagnostic's payload portion. Do that for the three existing controls and reverse case; preserve exact real framing. Do not add producer rows or production mutants.
3. **Discriminate equality failure from setup:** `payload_control` accepts any nonzero Python result and discards both streams, so syntax/framing/setup errors can earn all three control results. Retain diagnostics and require the named equality-failure result with a distinct setup/framing failure outcome. First validate the actual unmodified diagnostic with the SAME assertion. Duplicate/extra/missing copies must fail equality specifically; reversed actual payload must pass. Keep the expected real nonempty capture immutable throughout.

These are small corrections to the two selected proof surfaces; no wider table, production scan, new helper/framework or third production mutant is authorized. Retain existing namespace/migration controls and all inherited #455 coverage. Record exact focused statuses and causal assertion diagnostics. Root should consolidate one Sol2 pass, checkpoint and request one verdict. #455 remains open with its full workspace/actual-PR/required-CI boundary; no qualification is authorized from this incomplete child checkpoint.

Root assigns one coherent Sol attempt 2 against the complete retained contract and these finite findings, in this worktree only. Preserve the accepted actual read hook and frozen production/table/control paths. Pause after focused-green checkpoint notification; root owns Git and GitHub.

## Sol attempt 2 focused evidence

The shared payload assertion now receives and requires the exact full
`effect interchange qualification policy failure: <operation> failed (status 73)` line. It
validates exactly one sentinel and diagnostic at the end of the combined log, removes only that
fixed framing and the independently captured delegate stderr suffix, and compares the remaining
payload multiset with independently captured expected stdout. Complete mode expects the real
delegate stdout; error-only mode expects empty forwarded stdout, while the wrapper still executes
and validates the real delegate first.

The unmodified real complete diagnostic is checked by that same assertion. Duplicate, extra,
missing and reversed controls are copies of its actual diagnostic with only payload rows changed;
the expected nonempty real stdout stays fixed. Duplicate, extra and missing require status 1 and
the exact `complete producer payload did not match exactly` assertion diagnostic, so framing,
syntax or setup failures cannot satisfy them. The reversed copy passes when multiple rows exist.

The accepted second-Python read hook remains unchanged. After its exact status, operation,
PermissionError, target path and reached-read marker checks, the shim is removed and the standalone
checker is run clean before any deletion or traversal fault case.

Exact commands, raw logs and statuses are retained as
`/tmp/471-sol2-{syntax,qualification,108,108-policy,policy,diff-check}.{command,log,status}`.
With `PATH=/home/bl/.cargo/bin:$PATH`, syntax, both production checkers, the focused standalone108
policy and the parent policy all returned status 0. The parent recorded both inherited child
completions, including the fake-only lifecycle. No real runner, timing, build, Git or GitHub
operation ran.

## Astra Sol attempt 2 acceptance and integrated qualification

# Astra #471 Sol attempt 2 review

**PASS for source acceptance and inherited #455 delivery qualification.** Exact reviewed head `c950ce45a42e4c938e4ab6ca33dfc57980461e34`, engine-471-proof. Full child contract and all finite Luna1 findings reviewed. Only the two allowed suites and child evidence changed; accepted production checkers, helper, runner/validators/pins, CI wiring, directed input selectors and two production guard mutants remain frozen.

Payload assertion now runs in both complete/error-only modes. It binds the exact full known operation/status diagnostic, unique sentinel and separately captured delegate stderr suffix, then compares the complete payload Counter with fixed expected bytes including duplicate multiplicity. Empty mode expects no forwarded stdout but still validates the actual delegate before suppression. Dynamic operation labels now include their exact workload/seed/target/API subject, and the081 wording is explicitly bound rather than derived from an inexact observed suffix.

The SAME assertion first accepts the actual valid captured diagnostic. Three copies alter only that diagnostic's payload (duplicate/extra/missing) while the independently captured expected payload and real framing remain fixed. Each must return1 with the precise equality-failure message; framing/setup errors cannot earn credit. Reversed actual payload preserves complete multiset and passes. No new production mutation or matrix was added.

The accepted second-Python read hook still executes the original cross-file validator program/argv under -I/-B, delegates the first invocation, and raises an actual target-specific PermissionError only at Path.read_text, with reached marker and exact status/operation/path checks. The shim is removed and a clean standalone checker run now explicitly succeeds afterward. Existing deletion/tool-fault/optional-find cases and the namespace control remain intact.

Read all six `/tmp/471-sol2-{syntax,qualification,108,108-policy,policy,diff-check}.status`:0. Parent policy log reaches both existing108 and fake-only lifecycle completion; its named status-loss messages are expected counter-control failures followed by suite success, not swallowed unexpected errors. No reviewer tests, builds, timing, source edits or Git/GitHub mutations were performed.

Root may freeze the integrated immutable candidate and complete parent #455's workspace, actual pushed-PR Astra review and required qualification SUCCESS. #471/#455 close only on combined delivery; #403/#306/#349 and unrelated siblings remain open. No runtime/artifact/browser or benchmark authority arises from this source PASS.

Root integrated delivered main 4a814f348136bc5ba1d77bd04388a3c7163a0e10 before qualification. No inherited interchange production/checker/helper/runner/validator/fixture input changed upstream. Freeze this source for the full workspace run; no tracked edits until terminal. No timing is authorized.

## Combined integrated qualification complete; PR review pending

Astra accepted child source c950ce45 after Sol attempt2, retaining the original #455 production/table/control contract. Integrated candidate1171d0e44710010125c0f6c4571b29a2fdc6754a passed full workspace: {"head": "1171d0e44710010125c0f6c4571b29a2fdc6754a", "exit_code": 0, "blocks": 276, "passed": 1611, "failed": 0, "ignored": 24}. Named test population is identical to the delivered #460 integrated baseline; complete runtime/build/fixture inputs match current main4a814f34. Focused checker/suite statuses pass, with original namespace/migration production mutants and exact payload/read proof retained. Evidence is tracked under `artifacts/issue455-interchange-completion/` with exact hash/byte manifest and candidate provenance. Actual-head Astra review and required CI must pass before #455/#471 close together; #403/#306/#349 remain open. No timing or artifact regeneration occurred.

## CI vocabulary qualification amendment

# #471 CI vocabulary qualification ruling

APPROVE the narrow mechanical correction before a renewed actual-PR review. PR474 head b15e2e992e2c6a5cb73c7952dafbc9df10335c48 cannot be approved for merge after the failed required CI. Reviewed the actual failure in /tmp/engine-474-policy-ci.log and the two suite sources; no tests/builds executed.

Job 101318847530 fails check-env-vocabulary.sh with 15 concrete names outside MISO_ENGINE_. The vocabulary's first rule applies to all tracked scripts; its second requires every tool/script name to be registered. Test shims are not exempt. This is a qualification defect in the delivered tests, not a runtime or diagnostic-proof redesign.

Root must amend/synchronize #471 before editing. Allow precisely the two existing suites, docs/ENGINE_ENV_VOCABULARY.md, and numbered qualification/evidence records. Replace each complete identifier below consistently in assignments, expanded and escaped generated shell, exec delegates, and Python os.environ lookups. Keep every value, quoting/escaping level, operation label, occurrence selector, assertion, test population and production mutant unchanged:

| Old identifier | Required new identifier |
|---|---|
| MISO_DELEGATE_ERROR | MISO_ENGINE_INTERCHANGE_TEST_DELEGATE_ERROR |
| MISO_DELEGATE_OUTPUT | MISO_ENGINE_INTERCHANGE_TEST_DELEGATE_OUTPUT |
| MISO_EXPECT_DELEGATE | MISO_ENGINE_INTERCHANGE_TEST_EXPECT_DELEGATE |
| MISO_FAULT_LABEL | MISO_ENGINE_INTERCHANGE_TEST_FAULT_LABEL |
| MISO_FAULT_MODE | MISO_ENGINE_INTERCHANGE_TEST_FAULT_MODE |
| MISO_FAULT_NEEDLE | MISO_ENGINE_INTERCHANGE_TEST_FAULT_NEEDLE |
| MISO_FAULT_OCCURRENCE | MISO_ENGINE_INTERCHANGE_TEST_FAULT_OCCURRENCE |
| MISO_FAULT_STATE | MISO_ENGINE_INTERCHANGE_TEST_FAULT_STATE |
| MISO_OPTIONAL_FIND_MODE | MISO_ENGINE_INTERCHANGE_TEST_OPTIONAL_FIND_MODE |
| MISO_OUTPUT_SHAPE | MISO_ENGINE_INTERCHANGE_TEST_OUTPUT_SHAPE |
| MISO_PYTHON_OCCURRENCE | MISO_ENGINE_INTERCHANGE_TEST_PYTHON_OCCURRENCE |
| MISO_READ_MARKER | MISO_ENGINE_INTERCHANGE_TEST_READ_MARKER |
| MISO_READ_PROGRAM | MISO_ENGINE_INTERCHANGE_TEST_READ_PROGRAM |
| MISO_READ_TARGET | MISO_ENGINE_INTERCHANGE_TEST_READ_TARGET |
| MISO_REAL_TOOL | MISO_ENGINE_INTERCHANGE_TEST_REAL_TOOL |

Register exactly these 15 names in the vocabulary with their actual test-only meanings: captured delegate stderr/stdout, expected real delegate exit, fault diagnostic label/output mode/argv selector/selected occurrence/counter file, optional-find shape, expected output shape, selected Python invocation, read-hook reachability marker/original program/target path, and resolved real executable. They do not authorize a runtime environment surface, benchmark override, policy exemption or compatibility aliases.

Mechanical equivalence proof: compare both new scripts after applying the inverse exact-token mapping with their pre-correction versions; require byte equality. This checks escaping and logic preservation without re-running Rust qualification. Confirm no old names remain in the actual scripts and exactly the 15 new registered names are consumed. Do not rewrite historical artifact bytes to make the scan pass.

Proportional gates after the checkpoint: bash -n on both suites; bash scripts/check-env-vocabulary.sh; bash scripts/test-env-vocabulary.sh; bash scripts/test-effect-interchange-policy.sh with the established Rust PATH (it must finish its #108 child and fake-only lifecycle); git diff --check on changed source/prose. Retain terminal statuses and complete outputs, including both original production mutant assertions and child completion. No timed runner, workspace rebuild, target rebuild or artifact/browser rerun is justified by this rename-only delta. Existing workspace/runtime equivalence remains applicable once the inverse comparison passes.

Preserve the authentic failed CI log at /tmp/engine-474-policy-ci.log and cite the job https://github.com/misofm/engine/actions/runs/<actual-run-id>/job/101318847530 using the actual run ID obtained from the PR (do not publish the placeholder). Keep the local raw log intact. It contains rejected historical identifiers, so do not copy it into normally scanned tracked artifact paths. The numbered issue record may quote the diagnostic and names under the existing documented historical-spec exemption; that is not a new exemption. Existing 107 retained artifacts remain immutable. Store this ruling with old spellings in /tmp or the numbered spec, not as a newly scanned artifact copy.

This is a bounded qualification repair within #471, not another #455 implementation attempt and not a waiver of its hardstop. Full inherited #455/#471 evidence and closure obligations remain. Root checkpoints/pushes the corrected head, requests renewed exact-head Astra actual-PR review, and waits for required qualification SUCCESS on that unchanged head before merge.

Actual failed run: https://github.com/misofm/engine/actions/runs/33970786546/job/101318847530 . Root retains its raw log at `/tmp/engine-474-policy-ci.log`; no historical artifacts are rewritten.

## Lossless historical log representation

# #471 historical debug-log representation ruling

APPROVE one lossless historical-log representation change, before re-running qualification and renewed actual-PR review. This is separate from the already approved mechanical rename of live test shims; it does not exempt live code from the vocabulary policy.

The remaining material is an authentic shell trace, artifacts/issue455-interchange-completion/455-sol3-debug.log. Inspection shows historical environment assignments in traced commands, not new executable configuration. Its original manifest agrees with the actual bytes: size 124156, SHA-256 6455c8155681c727e656d5455c769a09019a4b2ffb9a4ca6ddcb36d5e8a4eff6. No established base64 archive convention was found in the bounded path search. Authorize a one-file encoding record, not a generic archive framework or blanket historical policy exclusion.

Root may replace that tracked plaintext representation with exactly 455-sol3-debug.log.base64 in the same artifact directory. Encode the complete original bytes without editing, redaction, name substitution, newline normalization or truncation. Preserve an untouched original outside the repository and its existing committed history. Decode into a disposable path outside the scanned checkout, compare byte-for-byte with the preserved original, and independently verify the decoded size and SHA above. Keep the encoded file tracked and record its own actual size/SHA in the manifest as usual.

Replace only this manifest entry's path/encoded size/encoded SHA; augment it with encoding="base64", original_path="455-sol3-debug.log", decoded_bytes=124156 and decoded_sha256="6455c8155681c727e656d5455c769a09019a4b2ffb9a4ca6ddcb36d5e8a4eff6". The manifest's ordinary fields continue to describe stored bytes. Add a concise artifact README note with the source commit containing the original plaintext, decoding instructions, and this representation-only reason. Use the actual commit, not a guessed provenance hash. A reader must be able to recover and authenticate the original without using the current suite or regenerating the historical run.

Report accurately: all original logical evidence is retained, one historical payload now has a different stored representation; do not say every stored artifact byte is unchanged. Prior counts describe the pre-correction package. Recount the final package after fresh evidence is added. Verify all other existing manifest payloads remain byte-identical and all final paths are tracked. Do not create a second plaintext copy under another tracked path or encode executable scripts to conceal violations.

No source, helper, scanner, vocabulary exemption, production checker, runner, pin, CI wiring or acceptance assertion changes are authorized by this ruling. The old failed CI and the first local vocabulary failure remain authentic historical failures, with the remote job link and local raw logs preserved. Do not copy those raw logs into scanned artifact paths; a concise numbered-spec failure record and authentic remote link suffice. This report also remains in /tmp or the existing numbered-spec historical record.

After encoding, run the already frozen proportional gates: syntax, real vocabulary checker, its mutation suite, the complete interchange parent policy suite including #108 and fake-only lifecycle, and source/prose diff checks. The live-code inverse-renaming byte-equivalence proof remains required. No Rust/workspace/target/artifact build or timed execution is justified. Renew actual-PR exact-head Astra review and require current qualification SUCCESS before merge; earlier PR review cannot override the observed CI failure.

Root encoded the one historical debug log, verified exact decoded identity and preserved the original locally and in Git history. The first local vocabulary check found that historical trace after live names were corrected; the original failure stays at `/tmp/471-ci-vocabulary-vocabulary.log`. The separate environment mutation suite exposed two fixed old catalog-count expectations after adding the fifteen names; this is pending its narrow count ruling. No historical execution is regenerated.

## Exact vocabulary-count integration amendment

# #471 vocabulary fixture count integration

APPROVE exactly two expected payload-token changes in scripts/test-env-vocabulary.sh, plus the numbered scope/evidence record. Independently read the current rows: COUNT|documented-name count failed (wc status 7)|99 and COUNT_TR|documented-name count formatting failed (tr status 7)|99, at lines 222–223. The retained mutation log demonstrates the real completed producer now emits 114 before the injected wc status 7; the assertion still expects 99 and fails with “dropped partial output”. The approved registry adds exactly 15 valid consumed test names: 99+15=114.

Change only these two final expected tokens from 99 to 114. Keep the operations, actual producer capture/delegation, injected exit status, diagnostics, paired error/full modes and all assertions unchanged. This updates the exact successful-output expectation to the new registry population; it does not accept arbitrary output or suppress a producer error. Confirm the final registry count is 114 and these are the only test-env-vocabulary.sh changes. If the actual count differs, investigate rather than adjust it again by guesswork.

Add this one existing mutation-suite path to #471's qualification amendment before editing. Preserve the failed local mutation log as historical evidence. Re-run real environment vocabulary and its complete mutation suite, then the already required interchange parent suite (including #108 and fake-only lifecycle), syntax and source/prose diff checks. No production helper/scanner, CI wiring, framework, other table cases or Rust/workspace/target/timing change is authorized. Retain the inverse-rename proof and lossless historical-log proof. A renewed exact-head PR review and required qualification SUCCESS remain necessary before merge.

## Corrected vocabulary qualification terminal evidence

All five proportional checks pass: syntax, real vocabulary, existing vocabulary mutation suite, complete interchange parent (including108 and fake-only lifecycle), and diff. Two existing expected count payloads now match the fifteen-name registry addition, with status/payload assertions intact. Exact inverse token mapping proves both policy suites are byte-identical to b15e2e99 after undoing names alone. The archived trace decodes to its original124156bytes/SHA; all other preexisting evidence payloads except the explanatory README remain byte-identical. Final package contains 123 manifest-listed files plus manifest.json. Raw failed CI/local logs remain locally and the authentic GitHub job is linked above; no historical log is redacted or regenerated. Renewed actual-head review and required CI remain pending.
