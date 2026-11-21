# host-web: no gate measures an admitted session's linear-memory growth near its budget

Since #2431 a nonzero boot budget caps the shared memory every browser instance runs on, so a session whose growth outruns its budget traps in `memory.grow` instead of finding 512 MiB of headroom. The cap adds an eighth of the budget and 35 reservation pages. That margin is a reasoned allowance, not a measured one.

## Evidence
- `scripts/check-web-boot-budget.mjs` measures refused boots and one admitted worst-document boot at the default budget against its parse projection. No gate boots a representative session (many sources, console slots, a reverb, meters) with a budget near its exact retained bytes and measures growth.
- The budget charges projected rows, not dlmalloc's chunk headers, minimum-chunk rounding, or `sys_alloc` growing by the whole request.
- A live replacement checks `graph_peak + source_total + transition_bytes` against `maximum_replacement_peak_bytes = budget - bridge_retained + session_model` (`hosts/host-web/src/lib.rs`, `crates/control-plane/src/compile.rs`). The candidate's control-plane rows and older epochs' rows are checked only on their own, so a replacement can be admitted above the budget.
- Every qualification row boots with `maximumMemoryBytes: 0n`.

## Contract
- A gate boots a representative mixer session on a memory capped by `sharedMemoryMaximumPages(budget)`, with the budget at the session's exact retained bytes plus a small epsilon. It calls `instance_reserve` as worker mode does, runs several live `SESSION_REPLACE`s, and asserts no trap and growth within the cap.
- Then either charge the replacement's candidate and older-epoch control rows inside the aggregate (a Rust change that moves the module digest), or keep the proportional margin and pin it to the measured worst case.

## Non-goals
- Changing the default budget or the module's declared maximum.

## Evidence / attempt record
(empty)
