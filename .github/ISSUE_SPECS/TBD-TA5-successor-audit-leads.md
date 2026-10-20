# Record the lead the last test-audit PR found and did not fix (TA5 successor)

TA5 (`TBD-TA5-delete-the-batch-2-redundant-tests.md`) deleted 97 tests of the batch 2 sample and
did not add scope. It found one lead. It does not block TA5.

## Lead

**The redundancy measurement over-reports on its own.** Of batch 2's 162 MEASURED-REDUNDANT
tests, reading kept 65. Some carry a claim that another record states and the name triage did not
read: the only red test of a `MUTATIONS.md` row (gate-expander row 3, effect-contract 140-1,
parametric-eq recorded failures 1 and 1b, graph 916-1 to 916-15), a known AArch64 release failure
in `scripts/lib/aarch64-known-defects.py` (`m2_exp2_lane_identity`, #1019), and a randomized
differential named `run_seeds` (host-core `a_bus_renders_the_bits_of_a_track_fed_its_sum`).
Others were the only test of a claim that only reading against named siblings shows; a first pass
that cited siblings only by file deleted five of those, and review restored them. B2's
projection of about 2,647 deletable pool tests therefore does not stand. If a pool-wide cut is
ever reopened, its brief must first match every candidate against the `MUTATIONS.md`
sole-catcher rows, the known-defect lists and the seeded-differential helpers, and every DELETE
answer must name, by its full name, the remaining test that asserts the same claim.

## Gate

No work unless a pool-wide cut is reopened; then the reopening brief cites this item.
