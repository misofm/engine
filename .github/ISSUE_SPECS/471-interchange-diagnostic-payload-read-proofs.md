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
