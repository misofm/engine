# #478 Luna1 evidence report

## Source and ownership

The accepted implementation checkpoint is `f5d5e3c7aa7cd475f170f997414bcb70a2938308`,
on `codex/rt8-prepared-slot-activity`. The worktree is clean and the branch was pushed by
root. Current source identities are:

| path | SHA-256 |
| --- | --- |
| `crates/rack/src/lib.rs` | `bc1276d0018a1a443f2b21fa318dbfe5d061588ef7c3aa576c133064ea061bfe` |
| `crates/builtins-compiler/tests/allocation_tracker.rs` | `93d3bdc8668300d9d95394c21b190e520d9993f75c6af409161fabd0721f5b91` |

Every post-checkpoint gate below was invoked through `/tmp/478-root-capture.py`; its
`.command.json` records the contemporaneous HEAD, source status, source hashes, argv and the
whitelisted environment. Accepted-checkpoint gates have clean status; the intentional mutation
capture records the expected dirty source status.

## Exact gates

Root's final formatted captures `finalfix-{shape,trace,mechanism,physical}` each selected one
debug test and passed. The earlier post-correction captures
`/tmp/478-root-postfix-debug-{shape,trace,mechanism,physical}.*` also passed; their formatting
check was followed by the pushed formatting correction.

Release exact captures all selected one test and passed:

* `/tmp/478-root-release-{shape,trace,mechanism,physical}.*`

The `release-*` exact captures predate the final formatting checkpoint but have the same
production implementation; the post-checkpoint final release suites below are the authoritative
release-source qualification.

The affected final release suites passed:

* `final-release-rack-suite`: 30 lib, 10 console-bank, 4 mono-reengage tests.
* `final-release-builtins-suite`: 7 tests.
* `final-release-graph-suite`: 1 test.
* `final-release-capi-suite`: 4 tests.

The preceding debug affected suites also passed: rack 30/10/4, builtins allocation 7, graph
direct-bank 1, capi resource lifecycle 4, graph-compiler exact 1 and full lib 64.

Strict final checks passed: `final-clippy-rack`, `final-clippy-integrated`,
`final-check-builtins-consumer`, and all final rack, builtins, graph, realtime, lane and workspace
policy checks. `final-policy-rack` and `final-policy-workspace` are the corrected-source captures.

## Actual layout and reservation facts

The native target facts observed by the existing allocation seam are `F = 16` bytes for
`Box<dyn BankStage>`, `B = 32` bytes for `BankSlot`, an actual prepared destination request of 72
bytes for three slots, and `W = 8` bytes for the width-eight bool mask. The 72-byte request matches
`3*P` for the test-local tuple layout mirror (`P = 24`); the private type itself has no public
layout probe. The public slot request was 96 bytes. For the fixture's
`N=3`, the unchanged reservation is `C = N*(F+3B+3W) = 408` and `L = max(NF,NB,W) = 96`.
The retained attributed bytes are `N*P+W = 80`; the conservative coexistence bound used by the
existing test is `NF+NB+2NP+2W = 304 <= C`. The test also checks `P <= B` and each request `<= L`.

The current native layout probe output `BankSlot=32`, `BankChain=200`, and `AoSoaScratch=40`;
that command did not print alignment. The existing graph test binary's DWARF reports current
`RuntimeUnit = 248` bytes, alignment 8. Both probes selected the first matching cached rlib/test
binary and did not capture a binary path or binary hash, so they are qualified current layout
facts rather than an exact compiled-head artifact proof. No pre-change BankChain or RuntimeUnit
baseline was captured before implementation, and no wasm layout run was made; these are current
facts only, not before/after claims. The required baseline gap is left explicit for review.

## Mechanism mutation

Exactly one mutation was applied at all three dispatch sites, replacing `has_active_lanes()` with
the test-only bounded packed-byte scan `has_active_lanes_scan()`. The source identity and dirty
status at mutation invocation are recorded in
`/tmp/478-root-mechanism-mutant-actual.command.json`. The same frozen mechanism test failed with
status 101 and preserved semantic execution before failing its activity-query assertion; stdout
and stderr are in `/tmp/478-root-mechanism-mutant-actual.{stdout,stderr}`. The failure occurred at
the query-count assertion before the subsequent lane-inspection assertion, so this report does
not overclaim that the exact final assertion ordering was exercised. The original live diff was
not retained; root later reconstructed the exact three-site diff in
`/tmp/478-root-reconstructed-mechanism.diff`, with provenance in its accompanying `origin.json`,
and verified its candidate source hash against the contemporaneous mutant metadata. This is a
post-execution reconstruction, not a claim that the original diff artifact was retained. The
three calls were restored exactly, and
`/tmp/478-root-mechanism-restored-after-mutant.*` passed one selected test.

## Candid execution notes

The first pre-correction, unwrapped logs are preserved at `/tmp/478-luna1-20260906-*` and the
succeeding formatted-source logs at `/tmp/478-luna1-20260906b-*`; none were overwritten. The first policy pass
failed `check-rack-policy` and workspace policy tests because test fixtures contained
`std::sync` references and the workspace mutant search also reported its expected directed-fault
issues. The test-only counters were changed to thread-local `Cell` state; corrected rack and
workspace policy checks passed. The first post-correction fmt check reported three formatting
diffs; `postfix-format-fix` ran `cargo fmt --all`, and root's final formatted exact captures passed.

The first capi release wrapper capture (`/tmp/478-root-capi-release-suite.*`) completed with status
0 and four passing tests; intermediate polling incorrectly appeared incomplete, and its files were
preserved. A distinct redundant rerun `/tmp/478-root-capi-release-suite-2.*` also passed. A later
accidental reuse of that label was rejected by the wrapper before execution because its exclusive
output files already existed. The final release capi suite was then captured under
`final-release-capi-suite` and passed.

No full qualification, benchmark, artifact repin, target matrix, or timing claim was performed.
