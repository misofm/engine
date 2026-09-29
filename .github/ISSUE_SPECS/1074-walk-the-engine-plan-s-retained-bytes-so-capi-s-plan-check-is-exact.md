# Walk the engine plan's retained bytes so capi's plan check is exact

Successor of #1060 (verdict attempt 2, MEDIUM). #1060 made memory tests "budgets plus a completeness check" (owner decision 4) and closed the catalog and canonical-JSON under-counts, but the engine plan is still bounded rather than observed: its bound has about 8-128 KB of slack, so an uncharged 1.8 KB graph table stays green.

## Smallest closable slice

Add an allocator-checked `retained_bytes()` walk over the graph runtime (plan, banks, schedule, tables), computed off the render thread. Then capi's plan check becomes: retained plan bytes == walked bytes <= the engine budget rows. A test with a counting allocator proves the walk equals what the plan actually holds.

## Gates

1. A planted uncharged table in the plan (the 1.8 KB case from #1060's verdict) goes red.
2. The walk equals the allocator's observed bytes for every console fixture, at Simd4 and Simd8, native and wasm32.
3. Render stays allocation-, lock- and syscall-free; console digests unchanged.
