# Make the nightly vectorization report pass on main and report its failures

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Filed
2026-10-08 by root from the stream G2 batch notes (the #1457 attempt-3 verdict). The nightly
native vectorization report has failed on `main` every night since 2026-10-07, and nobody was told:
its step is `continue-on-error`, so the nightly failure notice never fires for it. This issue also
carries one unrelated two-line hygiene item from the stream B leftovers (D5).

No engine source changes. No rendered bit moves.

## Problem (verified on `main` at `1e78d7820`)

- **The failure.** Nightly run 37605868115 (2026-10-07, `main` at `7e8379523`), job "native
  production-kernel release-probe report (non-blocking)", writes
  `"status":"fail" ... "failures":["recursive-svf / probe_svf_simd8: forbidden call 'call|callq'
  is present"]`. The nightly of 2026-10-06 (`f5e377d0d`) and every earlier one in the last two
  weeks passed. The same failure reproduces locally on `1e78d7820` (`bash
  scripts/run-native-vectorization-report.sh`).
- **The call** (local `objdump -d` of `target/release/audit` at `1e78d7820`): the body of
  `audit::vectorization::probe_svf_simd8` compares the rest plane's length with `0x100` before the
  frame loop and, if it is shorter, calls `core::slice::index::slice_index_fail`. The frame loop
  itself is the eight-lane `vmovups`/`vsubps`/... body the rule requires.
- **Where it comes from.** `svf_block_form::<L, true>` slices `&rest[..io.len()]` once per block
  (`crates/lane/src/kernels.rs:301`). #1328 (attempt 5 A9 and its follow-up, `c58c4e55f`,
  `4264bb190`) gave the probe a rest plane: `probe_svf_simd8` passes `black_box(rest)` as a
  `&[f32]` (`tools/audit/src/vectorization.rs:70-85`). `black_box` hides the slice's length, so the
  bound check cannot be proven and its panic call stays in the probe body. The allowlist rule
  forbids any call in that body (`tools/audit/vectorization-allowlist.tsv:4`, last column). The
  same shape is in `probe_svf_simd4` (`:105-120`), whose NEON report no nightly runs. #372 solved
  the same problem for the gain probe by keeping the array's static length (comment at `:47-50`).
- **The silent failure.** The step "Generate report and run red mutations"
  (`.github/workflows/nightly.yml:63-70`) is `continue-on-error: true`, so the job's result is
  `success`. The `failure-notice` job lists `native-vectorization-report` in its `needs` and its
  issue body (`:292-330`) but fires only on `failure`, which this job never reports.
- **Hygiene (stream B leftover).** `scripts/test-test-support-ci.py` has two lines over 100
  columns: `:149` (103) and `:152` (104).

## Decisions

- **D1. The probes keep the rest plane's static length.** `probe_svf_simd8` and `probe_svf_simd4`
  hide the values behind `black_box` but pass the plane with its static length, as the gain probe
  does (#372), so the per-block bound check is proven away and the probe body measures only the
  kernel. The kernel is not changed: its per-block check is correct in production, where the plane
  length is a runtime value, and it runs once per block outside the frame loop.
- **D2. The rule is not relaxed.** The `call|callq` column of the SVF row stays.
- **D3. A red report reaches the failure notice.** Remove the step's `continue-on-error`. The
  upload step already runs `if: always()`, so the receipt is still uploaded. The nightly gates no
  merge, so this changes only who is told: a red report now opens or comments on the "nightly
  workflow is failing" issue, as every other nightly job's failure does. Update the step's comment
  (`:63-64`) to say so.
- **D4. Unarmed form.** If the unarmed SVF form (`svf_block_form::<L, false>`) is also a production
  instantiation, the record states whether the report covers it; adding a probe for it is out of
  scope unless the existing probe no longer reaches the armed form's loop.
- **D5. Hygiene.** Rewrap `scripts/test-test-support-ci.py:149` and `:152` to 100 columns. No
  behaviour change.

## Authorized paths

- `tools/audit/src/vectorization.rs` (the two SVF probes and their callers in `execute_probes`)
- `.github/workflows/nightly.yml` (the `native-vectorization-report` step's `continue-on-error`
  line and its comment only)
- `scripts/test-test-support-ci.py` (lines `:149` and `:152` only)
- this spec

## Non-goals

- Any change to `crates/lane` or to any kernel.
- The allowlist's rows.
- Running the NEON report in CI.
- Any other nightly job.

## Hazards

- `.github/workflows/*.yml` is a hot file (STREAMS hot-file row): this slice edits one step of
  `nightly.yml` and lands in any order with the others; the later slice rebases.
- `scripts/check-ci-path-routing.py` checks parts of `nightly.yml` (`check_nightly_self_tests`,
  `:1060-1080`); run it after D3.
- A static-length probe must still be `#[inline(never)]` and must still hide the values, or the
  compiler may fold the loop and the rule then certifies nothing.

## Objective gates

1. `bash scripts/run-native-vectorization-report.sh` writes `"status":"pass"` with no failures,
   and `bash scripts/test-native-vectorization-report.sh target/release/audit` passes (its red
   mutations still go red).
2. **The fix is the cause (PR evidence).** Revert D1 only: the report fails with the same
   `probe_svf_simd8` forbidden-call message; restore it: pass.
3. **The rule still bites (PR evidence).** With D1 in place, add a call to the probe body (for
   example an out-of-line `black_box` function call inside the loop): the report fails with the
   forbidden-call message.
4. `cargo clippy --locked -p audit --all-targets -- -D warnings`, `cargo fmt --all -- --check`,
   `python3 scripts/check-ci-path-routing.py` (and its self-test), `python3 -B
   scripts/test-test-support-ci.py` and `bash scripts/check-workspace-policy.sh` pass.
5. After the batch lands, the next nightly's vectorization job reports `success` with
   `"status":"pass"`; record its run ID in the issue.

*Test value.* No new test. Gates 2 and 3 show the existing rule is red for the probe's own call and
for a real call in the body, and green only with the fix.

## Evidence

- Gate 1's report JSON, gate 2's and gate 3's runs, the objdump of the fixed probe's body, gate
  5's nightly run ID.

## Dependencies

- After: none open (#1328 is on `main`).

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: two attempts, one adversarial verdict each.
- Size: under two hours.
