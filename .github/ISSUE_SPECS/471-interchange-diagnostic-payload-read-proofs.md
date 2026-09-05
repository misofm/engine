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
