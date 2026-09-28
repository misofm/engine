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
