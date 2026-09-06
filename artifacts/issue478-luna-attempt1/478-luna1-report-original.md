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
`.command.json` records the contemporaneous HEAD, clean status, source hashes, argv and the
whitelisted environment.

## Exact gates

Root's final formatted captures `finalfix-{shape,trace,mechanism,physical}` each selected one
test and passed. The earlier post-correction captures
`/tmp/478-root-postfix-debug-{shape,trace,mechanism,physical}.*` also passed; their formatting
check was followed by the pushed formatting correction.

Release exact captures all selected one test and passed:

* `/tmp/478-root-release-{shape,trace,mechanism,physical}.*`
* `/tmp/478-root-finalfix-{shape,trace,mechanism,physical}.*` (root's contemporaneous final set)

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
`Box<dyn BankStage>`, `B = 32` bytes for `BankSlot`, `P = 24` bytes for the prepared stage-plus-u8
layout, and `W = 8` bytes for the width-eight bool mask. The prepared allocation request was 72
bytes for three slots, matching `3*P`; the public slot request was 96 bytes. For the fixture's
`N=3`, the unchanged reservation is `C = N*(F+3B+3W) = 408` and `L = max(NF,NB,W) = 96`.
The retained attributed bytes are `N*P+W = 80`; the conservative coexistence bound used by the
existing test is `NF+NB+2NP+2W = 304 <= C`. The test also checks `P <= B` and each request `<= L`.

The current native layout probe captured `BankChain = 200` bytes, alignment 8, and
`AoSoaScratch = 40` bytes. The existing graph test binary's DWARF reports current
`RuntimeUnit = 248` bytes, alignment 8. No pre-change BankChain or RuntimeUnit baseline was
captured before implementation, and no wasm layout run was made; these are current facts only,
not before/after claims. The required baseline gap is left explicit for review.

## Mechanism mutation

Exactly one mutation was applied at all three dispatch sites, replacing `has_active_lanes()` with
the test-only bounded packed-byte scan `has_active_lanes_scan()`. The source identity and dirty
status at mutation invocation are recorded in
`/tmp/478-root-mechanism-mutant-actual.command.json`. The same frozen mechanism test failed with
status 101 and preserved semantic execution before failing its activity-query assertion; stdout
and stderr are in `/tmp/478-root-mechanism-mutant-actual.{stdout,stderr}`. The failure occurred at
the query-count assertion before the subsequent lane-inspection assertion, so this report does
not overclaim that the exact final assertion ordering was exercised. The three calls were restored
exactly, and `/tmp/478-root-mechanism-restored-after-mutant.*` passed one selected test.

## Candid execution notes

The first pre-correction logs are preserved at `/tmp/478-luna1-20260906-*` and the succeeding
formatted-source logs at `/tmp/478-luna1-20260906b-*`; none were overwritten. The first policy pass
failed `check-rack-policy` and workspace policy tests because test fixtures contained
`std::sync` references and the workspace mutant search also reported its expected directed-fault
issues. The test-only counters were changed to thread-local `Cell` state; corrected rack and
workspace policy checks passed. The first post-correction fmt check reported three formatting
diffs; `postfix-format-fix` ran `cargo fmt --all`, and root's final formatted exact captures passed.

The first capi release wrapper capture (`/tmp/478-root-capi-release-suite.*`) was interrupted
during compilation and has no status file. It was preserved. A distinct rerun
`/tmp/478-root-capi-release-suite-2.*` passed; a later accidental reuse of that label was rejected
by the wrapper before execution because its exclusive output files already existed. The final
release capi suite was then captured under `final-release-capi-suite` and passed.

No full qualification, benchmark, artifact repin, target matrix, or timing claim was performed.
