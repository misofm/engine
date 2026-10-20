# Read every host-core test once and cut the defended duplicates (TA read, host-core)

Owner decision 2026-10-20, relayed by root: read every remaining test once, crate by crate, and
delete each test that a named remaining test already defends (AGENTS.md "Test value"). This spec
covers the `host-core` package. Branch `codex/batch-test-read-host-core`, from `main` at
`64fe78c46`. No new scope: findings that need work are listed under "Gaps and successors".

## Method

- Four readers (r1a to r1d) read all 611 `host-core` tests in 53 files (`src/` test modules and
  `tests/`). Per test they answered "which plausible defect turns this red that no remaining test
  catches?" and wrote KEEP, DELETE or MERGE.
- DELETE needs a named defender by full path that the reader read and that asserts the same claim.
  MERGE folds near-duplicates into one table-driven or loop test that keeps every claim.
- Never-delete classes were kept: realtime and allocation gates, counts and ceilings, ABI and wire
  pins, regression reproducers, randomized or seeded differentials, sole red tests of a
  MUTATIONS.md row, the acked-batch question, and tests a workflow or script runs by name.
- The integrator checked every DELETE defender across all four readers (none is deleted or merged
  away), grepped every removed name across the repository, built, removed what clippy reported
  unused, ran a mutation spot-check, and ran the full lint-job gate list.

## Result per file

Tests and test lines as the readers counted them (lines of the test function bodies).

| file | tests before -> after | kept | deleted (lines) | merged rows (lines) -> new tests |
|---|---|---|---|---|
| `tests/successor_swap.rs` | 166 -> 113 | 66 | 2 (13) | 98 (686) -> 47 |
| `tests/live_delta.rs` | 67 -> 62 | 62 | 4 (48) | 1 (42) -> into an existing test |
| `tests/stored_fader_automation.rs` | 37 -> 35 | 35 | 0 | 2 (85) -> into existing tests |
| `src/spectrum.rs` (tests) | 53 -> 47 | 46 | 5 (166) | 2 (44) -> 1 |
| `src/response.rs` (tests) | 4 -> 3 | 3 | 1 (75) | 0 |
| `tests/fp_environment.rs` | 3 -> 2 | 2 | 1 (9) | 0 |
| `tests/live_lanes.rs` | 3 -> 2 | 2 | 1 (36) | 0 |
| `tests/response_frame.rs` | 5 -> 3 | 2 | 1 (4) | 2 (131) -> 1 |
| 45 other files | unchanged | 273 | 0 | 0 |
| **total** | **611 -> 540** | **491** | **15 (351)** | **105 (988) -> 49 new** |

Diff against `64fe78c46`: 9 files, +399 / -1260 lines.

## Restores by the integrator

- `host_core::response::tests::explicit_eq_and_input_filter_queries_use_owner_outputs_at_launch_rates`:
  the only test of configuration ID 2^53+1. The named defender
  (`tests/randomized.rs::randomized_response_queries_agree_across_paths_and_with_the_grid_law`)
  echoes IDs only up to 2^53. Restored exactly as at `64fe78c46`.
- `host_core::spectrum::tests::selected_channels_reports_the_selected_entrys_mask`: the sole red
  test of a row in `hosts/host-web/MUTATIONS.md` (never-delete class). The reader grepped only the
  host-core ledger. Restored exactly as at `64fe78c46`.

## Lane-width merges in successor_swap.rs

98 tests named `*_at_eight_lanes` / `*_at_four_lanes` became 47 tests:
`<base>_on_every_width` loops over `backends()` (Simd8 under `avx2`, then Simd4), and
`<base>_at_the_build_width` runs `Backend::current()`. A static check over the base and the new
file compared, per merged test, the widths its parts ran with the widths it runs now:

- On an AVX2 build, all 47 run exactly the old widths: 23 `on_every_width` tests run {8, 4} as
  their parts did; 24 `at_the_build_width` tests run {8}, because their four-lane part was ignored
  outside NEON/simd128 (`cfg_attr(..., ignore)`).
- On a NEON or simd128 build, `backends()` is {4} and `Backend::current()` is Simd4, which is what
  the old four-lane parts ran there (the eight-lane parts were `cfg(target_feature = "avx2")`).
- No merged test carries a width `cfg` or `ignore`. Every kept test's width set is unchanged.

## Never-delete keeps by class

Readers' class labels on KEEP rows (one class per test): counts or ceilings 66; allocation or
realtime gates 64; regression reproducers 38; MUTATIONS.md sole red tests 24 (25 with the restore);
the acked-batch question 20; randomized or seeded differentials 13; ABI or wire 4; run by a
workflow or script 2. The other 258 KEEP rows name a unique claim without a class.

## Mutation spot-check

Seed `20261020`; `random.sample` of 30 from the 1,130 current `host-core` mutants
(`cargo mutants --list --json -p host-core`, cargo-mutants 27.1.0) whose function the 15 deleted
tests covered (coverage map `cov.tsv`/`fns.tsv` at `81a3074fc`, joined by file and function name).
`--diff` lists no product mutants here, because the diff touches tests only. Each mutant was
applied with `git apply`, the full `cargo test --locked -p host-core --all-features` ran
(fail-fast), and the file was reverted (signal-safe; the tree was clean at the end).

- 26 of 30 caught by a remaining `host-core` test.
- 4 not caught. For each one the 15 deleted tests ran at `64fe78c46` with the mutant in place:
  none of them is red. So the cut lost no catcher; these are older gaps in the `host-core` suite.
  Tests in other packages (host-web, capi) cover these functions but were not run.

| # | function (file:line) | mutant |
|---|---|---|
| 7 | `prepare_host_runtime_with_live_controls_policy_and_spectrum` (`src/prepare.rs:2564`) | `&&` to `\|\|` before the spectrum-row match in `spectrum_may_carry` (Single) |
| 19 | same function (`src/prepare.rs:2572`) | `&&` to `\|\|` in the Collection arm's rate/quantum test |
| 8 | `<impl PartialEq for Choices>::eq` (`src/control_provider.rs:1943`) | `==` to `!=` on the choice value bits |
| 24 | `following_route_lanes` (`src/prepare.rs:5319`) | `==` to `!=` on `own == [true; 2]` (source mute automation on both lanes) |

## References fixed

- `.github/ISSUE_SPECS/1584-...`: two gate citations now name
  `a_restarted_effects_subscription_survives_the_rebuild_at_the_build_width` and
  `a_pending_disarm_leaves_the_restarted_tap_unarmed_at_the_build_width`.
- No MUTATIONS.md row names a deleted test after the restores (`crates/host-core/tests/MUTATIONS.md`
  had none; the host-web row's test is restored). `scripts/`, `.github/workflows/` and `tools/`
  name none. Dated handoff records under `docs/handoffs/` stay as written.

## Gaps and successors

- The four spot-check survivors above (older gaps, not caused by this cut). Successor: run the
  host-web and capi tests that cover them; where none is red, add one host-core test per claim
  (spectrum carry needs the same rate and quantum; equal choices share one stored descriptor; a
  route whose source automates its mute on both lanes takes its own lanes).
- `successor_swap.rs::a_changed_control_kind_carries_no_effect_lane_at_four_lanes` is ignored
  outside NEON/simd128 and has no eight-lane twin, so it never runs on the AVX2 rows. Successor:
  give it a `Backend::current()` form or an eight-lane case.

## Gates

- All 31 `run:` steps of the qualification `lint` job (fmt, workspace clippy `-D warnings`,
  `cargo doc -D warnings`, every hermetic policy gate and the three x86 lane-guard probes), from
  `policy-job.sh` pointed at this worktree: PASS.
- `cargo test --locked -p host-core --all-features`: PASS (host-core is the only package touched).

The per-test verdict list (611 rows) is in two parts, posted as issue comments.
