# Make the realtime policy scan catch turbofish collects

## Product outcome

`scripts/check-realtime-policy.sh` scans realtime-policy regions for allocating calls, but it misses the turbofish form `.collect::<...>()`, so an allocation written that way inside a `REALTIME_POLICY_BEGIN/END` region passes the static gate. The native allocation tests still catch it at runtime, but the static gate should not have a hole a spelling change walks through.

## Smallest closable slice

1. Extend the scan's `collect` pattern to match `.collect::<` as well as `.collect(`.
2. Add a self-test fixture (the script's existing negative-fixture mechanism, or a new one) containing `.collect::<Vec<_>>()` inside a policy region, and assert the script rejects it.
3. Run the extended scan over the workspace; if it finds real hits, list them in the evidence and fix or allow-list each with a reason.

## Objective gates

- The negative fixture is rejected; the workspace passes.
- `bash scripts/check-realtime-policy.sh` and its self-test pass in CI.

Found by the #936 attempt 1 implementer and verification.

## Evidence

Closed by tool slice A2 (#1439), attempt 1, on `codex/d15-stream-j3`.

- Negative fixture: the case `forbidden-turbofish-collect` in `tools/realtime-policy/tests/policy/forbidden.rs`
  (`a_spelling_the_line_scan_missed_is_a_finding`). A marked function holding
  `items.iter().collect::<Vec<_>>()` gives the class "marked realtime forbidden-body predicate". The awk gate
  passes the same tree (parity run: awk exit 0, tool exit 1), so the hole was real. Mutation: matching only
  `.collect(` turns that test red.
- Workspace: `cargo run --locked --release -q -p realtime-policy` prints
  `realtime policy: ok (93 marked regions in 26 files)`, the same counts as
  `bash scripts/check-realtime-policy.sh`, so the real regions hold no turbofish collect.
- Verdict: `/home/bl/misofm/submix-verdicts/1439-attempt1.md` (written by the verifier; this issue closes
  only on its PASS).
- The closing steps that need GitHub (D6 steps 3 and 4) run after the batch push, not in this attempt.
