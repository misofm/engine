**Approve the bounded isolation correction:** add `.arg("--test-threads=1")` to **both existing child commands**. No stop/rebrief is required by this diagnostic evidence.

Verified clean HEAD/local upstream/remote branch at `44f9e9c73abeaaf778507db9ce83d0eaa7b99e53`. GitHub #536 is OPEN with matching title and exact spec body. The current test-source SHA-256 matches all four captures: `eafda5a261d900b89f463c91bbf12a050195c83f891755fd57aad0d4064add90`.

Evidence supporting this decision:

- All four pre-correction comparisons returned status0 and executed their child tests. Scalar direct/wrapped both recorded 27 allocations and 102496 bytes; controller preserved the exact +2 allocations/+2 frees/+9 bytes. TLS allocation/free counts agree with global counts.
- Warm/reset occurs before measurement; owners survive the windows; printing occurs afterward. Count mode records allocations while retaining the same System allocator and preparation calls. The audited scope does not select a different preparation control path. Original strict global, realtime, resource and admission assertions remain intact.
- The synchronized foreign-thread control recorded global **1 allocation/2 frees/4096 bytes**, with measured-thread TLS **0/0**. Worker teardown can overlap the snapshot; the extra free is correctly retained without subtraction or exact attribution.
- Pinned libtest starts the worker before concurrent-branch map/queue bookkeeping. Explicit concurrency1 bypasses that branch, providing a concrete reduction in interference opportunity.

The comparisons did **not** reproduce or explain the historical 31-versus-27 failure. The synthetic control demonstrates susceptibility, not historical causation. The correction also does not make the child process entirely single-threaded.

Luna high/xhigh may make only the two argument additions and necessary formatting. Complete the spec’s finite gates: both exact selectors through parent/child in debug and release, inherited concurrency2 override verification, the full nine-test binary once, affected strict Clippy and formatting, using `--locked --features control-provider`. Preserve complete captures and every failure.

This is the diagnostic decision only; consolidated attempt review and required PR qualification remain outstanding.