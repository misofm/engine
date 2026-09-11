# Nightly workflow failure: precise budget selection and saved-crash replay

## Approved current scope — 2026-09-11

Latest failed nightly34453343273 reports two failure groups. session_parse and
session_compile crash in json-syntax object/mod.rs795, before delivered #387.
The budget job runs blanket --ignored and accidentally selects the opt-in
selective_meter_and_readiness_descriptive_timing test without its required mode.

Smallest correction: select exactly the four intended release budget tests in
.github/workflows/nightly.yml, preserving their existing thresholds/commands and
all other jobs. Add a small hermetic selection regression through existing CI
policy machinery: the four tests run once, the unrelated opt-in timing test cannot
be selected, and command failure propagates. Do not silently skip intended budgets.
Allowed: nightly workflow, directly relevant existing CI routing/policy tests (or
one narrow script if needed), and this spec. No production source correction is
authorized without a new bounded review if saved crashes still fail.

Retrieve the saved nightly-fuzz artifact10143260782 from run34453343273 and replay
both recorded crashing inputs against current source, with actual sanitizer/target/
source/argv/exits preserved outside the repo. Do not treat #387 as proof without
replay. Run each intended release-budget test once after confirming exact selection;
this is existing product-budget qualification, not a new benchmark campaign. Do not
run #522 timing, unrelated benchmarks, or broad random fuzzing for this slice. A full
nightly dispatch is unnecessary if exact repaired job behavior and both old failures
are independently verified; normal schedule remains unchanged. Preserve failed
nightly logs/crash hashes and report any remaining failure before widening scope.

Astra LOW implements; Astra XHIGH verifies. Five attempts maximum. Root checkpoints
coherent compiling/focused-green exact paths and pushes before another tranche.
At most two active issues; #228 verification closes before #213 implementation starts.
Isolated worktree; no #213 builtin corpus or #522 timing-fixture edits. Required
reviewed-head PR/main CI, upstream evidence, verified remote closure and clean
worktree removal remain mandatory. Root owns artifact pins; no pin change expected.
No compiler captures or performance improvement claim.

## Attempt 1 workflow checkpoint

The release-budget step now targets the four intended test functions with their
explicit package and lib/integration-test target, `--ignored --exact`, and shell
failure propagation. Thresholds and test bodies are unchanged. Existing required
CI policy machinery checks the exact selection; its hermetic regression captures
all four stub invocations, rejects broad/missing selectors and missing commands,
and injects a failing exit at each position to prove no failure is hidden.

This checkpoint contains selection and policy changes only. Actual one-shot budget
qualification and replay of both saved crashes follow after the root checkpoint;
neither is claimed complete by the hermetic test.

## Historical issue body

The scheduled nightly workflow failed.

| job | result |
| --- | --- |
| native production-kernel release-probe report | success |
| bounded descriptive benchmarks | success |
| deep fuzzing over every target | failure |

Run: https://github.com/misofm/engine/actions/runs/33851070103

Close this issue once the failure is dealt with. The next failing night after that opens a fresh one; while it is open, further failures are added as comments.