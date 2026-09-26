# Regenerate and gate the graph resource-report fixtures

## Product outcome

`fixtures/graph/v1/direct-route.resources.json` is a checked-in resource report that no test or tool checks, so it drifts silently. It was already stale before #936 and #936's executor-table charge moves it by another 180 bytes. A resource fixture that nothing verifies is a claim, not evidence.

## Smallest closable slice

1. Find every `fixtures/graph/v1/*.resources.json` and the code path that would produce each (the graph resource estimate for that fixture's plan).
2. Add one test that regenerates each report in process and compares it byte for byte with the checked-in file, with a documented regeneration command (an env var such as `MISO_ENGINE_REGENERATE_FIXTURES=1`) that rewrites them.
3. Regenerate the stale files once.

## Objective gates

- The new test fails on the current stale fixture and passes after regeneration.
- A red mutation (change one charged byte in the estimate) turns the test red.
- fmt, clippy `-D warnings`, doc `-D warnings`.

## Non-goals

No change to what the estimate charges.

Found by the #936 attempt 1 verification.
