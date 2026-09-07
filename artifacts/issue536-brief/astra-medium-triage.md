The failure is real, but its cause is **not established**. There is no inspected production path connecting #534’s gate changes to this allocation contract.

- The child failed at line1369 with endpoint allocations **31 versus direct 27**. It stopped before comparing requested bytes, reallocations or frees, so those values remain unknown. The controller child’s pass does not explain this failure.
- `asymmetric_effect()` constructs a **compressor**, before the endpoint measurement. Endpoint preparation validates metadata, reads two resident compressor parameters, constructs inline capabilities/resources, and calls the same `PreparedAutomationDelivery::prepare` as the direct comparator. Host, compressor, protocol, allocator and build inputs are unchanged.
- `bench_support::alloc` counts allocations process-wide. Child isolation excludes other test processes, but not the child’s own harness thread. Concurrent parent tests cannot directly increment these child counters.

The pinned Rust libtest source provides a concrete contamination opportunity: with concurrency greater than one, it spawns the test worker and then updates `running_tests` and `timeout_queue` while that worker can execute. Even an exact-filtered single test takes this scheduling path. `--test-threads=1` avoids that bookkeeping branch, **but still spawns a test thread**; it does not guarantee a wholly single-threaded process. These statements follow the [pinned libtest implementation](https://raw.githubusercontent.com/rust-lang/rust/8bab26f4f68e0e26f0bb7960be334d5b520ea452/library/test/src/lib.rs). The log does not identify which thread made the extra four allocations.

A separate bounded successor issue is justified before implementation:

**Proposed unnumbered brief: “Isolate scalar Point preparation allocation measurements from libtest bookkeeping.”**

Scope: `crates/host-core/tests/scalar_point_endpoint.rs`, its two existing child launchers and compact test-only diagnostics; issue/evidence records. Reuse `bench_support::alloc` and the existing thread-local realtime auditor. No allocator, production, Cargo, CI or policy changes.

Discriminating plan:

1. Preserve this CI failure. Run one frozen comparison of each exact child with explicit child concurrency **2 versus 1**, recording complete deltas and statuses. Parent Cargo options alone do not alter the arguments these child launchers construct. A passing comparison alone establishes neither cause nor correction.
2. In the same bounded test fixture, capture process-wide totals alongside existing thread-local audit counts around both preparation calls. Warm/reset outside each interval and print afterward. An excess confined to global counts distinguishes foreign-thread activity from preparation-thread allocation.
3. Add one deterministic control using a prestarted, explicitly synchronized foreign-thread allocation inside a diagnostic interval. It must move global counts without moving the measured thread’s audit counts. This proves contamination sensitivity, not attribution of the historical four allocations.
4. Only if supported by that evidence, add `--test-threads=1` to both existing child commands. Retain every original allocation/byte equality, controller offset, liveness probe, zero-realtime assertion and admission/cancellation contract. If same-thread excess appears or isolation remains insufficient, stop and report it rather than enlarge scope.

Luna high/xhigh should implement the separately briefed slice. Gates: both exact children debug/release, the full nine-test binary, affected lint/format checks, and the retained discriminating control. No sleeps, retry-to-green or relaxed budgets.

No commands beyond read-only inspection were run; no new source-attempt verdict is issued.