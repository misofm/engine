# Record the leads the last test-audit PR found and did not fix (TA5 successor)

TA5 (`TBD-TA5-delete-the-batch-2-redundant-tests.md`) deleted 131 tests of the batch 2 sample and
did not add scope. It found two leads. Neither blocks TA5.

## Leads

1. **An orphaned test seam in `graph`.** TA5 deleted `test_only_set_host_master_declined`
   (`crates/graph/src/runtime.rs`; compiler-reported unused once its only caller, the #916 gate
   `the_host_planes_are_the_arena_oracles_master_bit_for_bit_at_every_stride`, went). The
   `#[cfg(test)]` thread-local `HOST_MASTER_DECLINED` it wrote stays: `preflight_sequential` still
   reads it, nothing sets it, so the clause is always false. The compiler does not report a read
   of a never-written cell. Delete the thread-local and its read, or record why it stays.
2. **The redundancy measurement over-reports on its own.** Of batch 2's 162 MEASURED-REDUNDANT
   tests, reading kept 31. Five of them carry a claim that another record states and the name
   triage did not read: three are the only red test of a `MUTATIONS.md` row (gate-expander row 3,
   effect-contract 140-1, parametric-eq recorded failures 1 and 1b), one is a known AArch64
   release failure in `scripts/lib/aarch64-known-defects.py` (`m2_exp2_lane_identity`, #1019),
   and one is a randomized differential named `run_seeds` (host-core
   `a_bus_renders_the_bits_of_a_track_fed_its_sum`). B2's projection of about 2,647 deletable pool
   tests therefore does not stand without that cross-check. If a pool-wide cut is ever reopened,
   its brief must first match every candidate against the `MUTATIONS.md` sole-catcher rows, the
   known-defect lists and the seeded-differential helpers, and must still read each body.

## Gate

Lead 1: the seam is gone or documented, with workspace clippy and the graph tests green. Lead 2:
no work unless a pool-wide cut is reopened; then the reopening brief cites this item.
