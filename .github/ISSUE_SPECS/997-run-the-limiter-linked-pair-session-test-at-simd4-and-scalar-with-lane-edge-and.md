# Run the limiter linked-pair session test at Simd4 and Scalar, with lane-edge and linked-ramp coverage

## Product outcome

#996 pins the true-peak limiter's linked-pair path (#990) with a hot, non-repeating 64-track console session (`crates/host-core/tests/limiter_linked_session.rs`). It runs only at the host's native bank width (W8). The browser runs 4-lane banks, so the product's width is untested at session level. The #996 verification found three gaps, all test-only:

1. The test's claim that W4 needs engine or tooling changes is wrong. Host-core's existing test-only seam `prepare_host_runtime_with_console_backend` (`crates/host-core/src/prepare.rs`) renders the same pin at `Simd4` and `Scalar`, and all six recorded mutation rows are red there.
2. #990's M9 (the right ramp leaves the left stale) stays green: there is one linked-ramp trial, and it is red at only 3 of 8 seeds.
3. No one-sided retarget lands on W8 lanes 0, 6 or 7, so a mutation confined to a bank's edge lanes passes at W8.

## Smallest closable slice

Test code only, in `crates/host-core/tests/limiter_linked_session.rs` (and `crates/true-peak-limiter/tests/MUTATIONS.md` for the record):

- run the #996 scenario at `Simd8`, `Simd4` and `Scalar` through the existing seam, each against the same pinned digest (the pin is width-independent because banking never changes per-lane arithmetic);
- add linked-ramp retargets (both channels of a linked pair moving a ramped control together) so that M9 is red at every seed tried;
- place one-sided retargets so each of W8 lanes 0 and 7, and W4 lanes 0 and 3, receives at least one.

## Objective gates

- The pin (`b22ef17c...`) is unchanged, re-recorded on the pre-#990 limiter source (`bbcf8ce1`) at all three widths in dev and release, or a new pin recorded there if the scenario must change.
- Red at every width under the six #996 mutation rows (M1, K1, K2, M2, M6, M7), and red under M9 at 8 of 8 seeds.
- A mutation that corrupts only lane 0 and one that corrupts only the last lane of a bank are red at W8 and W4.
- Dev runtime of the test stays under 10 s; every standing console digest is unchanged.

## Out of scope

Wasm session coverage of the same scenario needs tooling (a guest that takes per-block source content). That is an owner ruling, recorded in the #996 verdict; do not build it here.
