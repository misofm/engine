# Tighten the four-lane reference graph ceilings from measured AArch64 rows

Stream J of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0).

Test-budget follow-up of *Prepare C ABI plans with live effect lanes* (#1263, slice of #1053),
from its verdict's NIT 2 (`docs/handoffs/live-updates-1053/1263-attempt1.md`, recorded as "#1263
N2" in that directory's `README.md`). No production code changes.

## Problem (verified on `main` at `d2fe0555a`)

`reference_session_retained_rows_stay_within_their_budgets`
(`crates/capi/tests/resource_lifecycle.rs:1179`) holds each retained row of the nine-track EQ
reference session under a ceiling per lane width (`Budget`, `:972`; `REFERENCE_BUDGETS`, `:1054`).
The rule (`:991-1003`) is: the measured row plus 10 %, rounded up to a 64-byte multiple. Eight
lanes run on x86-64-v3, four lanes on AArch64 NEON.

Some four-lane ceilings were never measured. They were derived:

- **The three graph rows (#1263).** The doc comment at `:1039-1053` says so. The ceilings are the
  old four-lane baseline plus the *eight-lane* move (+128,984), plus 10 %:
  `graph_session_plus_plan_bytes` and `graph_incremental_plan_bytes` 413,952 (`:1059`, `:1065`),
  `graph_metadata_bytes` 204,544 (`:1071`). The verdict computed the real four-lane move as
  109,416 (9 EQ members x 2,104 + 3 four-lane banks x 30,160), so these ceilings sit about
  16-23 % above the probable values. A structural move of up to that size passes unseen at four
  lanes, while the eight-lane ceilings stop one over 10 %.
- **The three effect-bank rows (#1098).** `effect_bank_scratch_bytes` and
  `effect_bank_runtime_buffer_bytes` 13,568 (`:1083`, `:1089`), `effect_bank_metadata_bytes`
  1,024 (`:1095`). The comment at `:1021-1029` says these four-lane values are "derived, not
  measured" (12,288, 12,288 and 921).

**Why nobody measured them.** The test prints every row (`println!` at `:1205`), but libtest
captures the output of a passing test. The AArch64 debug leg runs it inside the whole product-crate
run (`scripts/run-aarch64-tests.sh:144`) without `--nocapture`, so the rows never reach the log.

**Searched CI logs (none has the rows).**

| run | job | result |
|---|---|---|
| 37201085656 (PR #1299, after #1269 and #1273) | 111432858282, AArch64 debug | test `ok`, no rows |
| 37201085656 (PR #1299) | 111432858272, AArch64 release | does not run the test |
| 37192142223 (PR #1298, #1053 batch) | 111406512153, 111406512232 | log not retrievable |

The test passing there proves only that each four-lane row is under its loose ceiling.

**Eight-lane rows on `d2fe0555a`** (x86-64, run locally with `--nocapture`), for comparison: graph
session+plan and incremental 382,918 of 421,248; graph metadata 185,121 of 203,648; effect bank
scratch and runtime buffer 16,384 of 18,048; effect bank metadata 805 of 896. These graph values
equal the ones #1263 recorded, so #1269 and #1273 did not move the graph rows at eight lanes.
(capi retained moved 273,640 -> 274,153 and source overhead 3,934 -> 4,006 inside their budgets;
both are width-independent and out of scope here.)

## Decisions

- **D1. Print the rows on every AArch64 debug run.** In `scripts/run-aarch64-tests.sh`, debug
  mode, after the main `cargo test` (`:144`) and before the expected-failure loop, run the budget
  test alone with `--nocapture`:
  `cargo test --locked "${packages[@]}" --features "$features" --test resource_lifecycle --
  --exact reference_session_retained_rows_stay_within_their_budgets --nocapture`.
  Use the main run's package and feature arguments so nothing rebuilds. If cargo refuses
  `--test resource_lifecycle` across packages that lack it, use `-p capi` with the same
  `--features` and record the extra build time in the evidence. From then on every four-lane
  ceiling can be measured from a CI log.
- **D2. Measure first.** Get the four-lane rows from the AArch64 debug job of this PR's first
  qualification run (D1 prints them). Alternatively run the test under qemu-user on x86
  (`CARGO_BUILD_TARGET=aarch64-unknown-linux-gnu` plus that target's linker and runner, as the
  script's header says); byte sizes do not depend on real hardware. Record which source was used.
  The PR's own AArch64 job must then pass with the tightened ceilings.
- **D3. Ceilings by the existing rule.** For each of the six rows above, the four-lane ceiling
  becomes `ceil(measured x 1.1 / 64) x 64`. Do not change any eight-lane ceiling, any
  width-independent ceiling, or the rule.
- **D4. Expected values, to check, not to use.** If #1263's derivation holds, the four-lane rows
  are 356,714 (graph session+plan and incremental: 230,845 + 16,453 + 109,416) and 166,325 (graph
  metadata: 56,840 + 69 + 109,416), so the ceilings become **392,448** and **182,976**. The
  effect-bank rows should read 12,288 / 12,288 / 921, so their ceilings stay 13,568 / 13,568 /
  1,024. Use the measured rows. If a measured row differs from this prediction, say so and why if
  known.
- **D5. Rewrite the doc comment.** Replace "derived, not measured" for these six rows in the doc
  comment (`:1021-1029`, `:1039-1053`) with the measured values, the run and job IDs (or the qemu
  command), and the date.

## Authorized paths

- `crates/capi/tests/resource_lifecycle.rs`: the six `four_lanes` values of D3 and the
  `REFERENCE_BUDGETS` doc comment only.
- `scripts/run-aarch64-tests.sh`: the one D1 step in debug mode only.
- This spec.

## Non-goals

- Production code, any eight-lane ceiling, the width-independent rows, `REFERENCE_MODEL_BUDGET`.
- Changing the 10 % rule or the budget test's assertions.
- The C ABI's live-window cost itself (the owner question *Bound C ABI live effect windows by lane
  depth*).

## Hazards

- **CI routing.** `scripts/check-ci-path-routing.py` and `scripts/test-ci-path-routing.py` know
  `run-aarch64-tests.sh`. A change to it must still route to the AArch64 jobs.
- **Expected-failure rows.** The extra run is not a skip and must not name a known-defect row
  (`scripts/lib/aarch64-known-defects.py`). It runs one test that passes.
- **Two CI runs.** Without qemu, the first PR run gives the rows and a second push sets the
  ceilings. That is one more CI event. Do not add an empty commit to get it; the ceiling commit is
  the second push.

## Objective gates

1. **Rows recorded.** The PR body has a table of the six rows: measured four-lane value, its
   source (run ID and job ID, or qemu), old ceiling, new ceiling. It also quotes the log lines.
2. **The ceilings bind.** On the PR's AArch64 debug job, the budget test passes and its printed
   rows show each new ceiling within 10 % (rounded up to 64) of the row. As one-time PR evidence,
   not committed: lower one new four-lane graph ceiling by 64 bytes below the measured row under
   qemu, or reason from the printed row; the test turns red.
3. **Nothing else moved.**
   - `cargo test --locked -p capi --test resource_lifecycle` passes on x86 (eight-lane ceilings
     unchanged).
   - The AArch64 debug and release jobs pass.
   - `python3 -B scripts/test-ci-path-routing.py` and `python3 -B scripts/check-ci-path-routing.py`
     pass.
4. **Policy.**
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked -p capi --all-targets --all-features -- -D warnings`
   - `bash scripts/check-workspace-policy.sh`
   - `bash -n scripts/run-aarch64-tests.sh`

*Test value.* No new test. The tightened ceilings make the existing budget test catch, at four
lanes, a structural move of more than 10 %, which today passes up to about 16-23 % unseen.

## Evidence

- Gate 1's table and log lines.
- Whether D4's prediction held, row by row.
- D1's cargo form and whether it rebuilt anything.

## Dependencies

- None. #1263 and #1299 are on `main`.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Commit on its own branch from synchronized `main`.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
