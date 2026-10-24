# Bring the DSP crates debug test job back under 15 minutes

Filed by root (backlog sprint, 2026-10-24).

## Defect

The required job "DSP crates debug tests" (`.github/workflows/qualification.yml`, `test-debug-b`) now takes 13 to 15 minutes and hit its 15-minute limit on main once and on four PR branches on 2026-10-24 (#1951, #1964, #1965, #1967), with no test failure. Root raised the limit to 25 minutes as a stop-gap in the housekeeping PR that files this issue; this issue owns the real fix.

The tests that run longer than 60 s in that job include:
- `a_long_polarity_flip_on_a_bank_member_stays_inside_its_trim`
- `live_bound_carries_every_term_an_independent_recomputation_requires`
- `padding_tests::every_padded_bank_renders_its_members_per_node_bits`
- `padding_tests::padded_banks_render_the_fixtures_per_node`

## Smallest closable slice

1. Measure the per-test wall time of the job's test binaries once (debug profile, as CI runs them) and list the ten slowest.
2. For each slow test, cut its cost without losing its claim: fewer blocks, a smaller grid, one backend where the claim is not per lane width, or move a long sweep to `nightly.yml` when it is a sweep and not a per-PR claim (one test per claim; a test that moves keeps its claim). A test that loses its claim is not acceptable.
3. Bring the job back under 10 minutes on the CI runner, and set `timeout-minutes` back to 15.

## Paths

The slow tests' files (crates/builtins, crates/*/src/padding_tests.rs, and those the measurement names), `.github/workflows/qualification.yml` (the timeout line) and `nightly.yml` only if a sweep moves, this spec.
