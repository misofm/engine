# Repair the console benchmark's mono symmetry witness premise after the census drift

## Defect

`scripts/run-console-benchmark.sh` cannot complete on `main` (a16bc5b6, 2026-09-25): the warmup launch aborts in `tools/bench/src/console.rs` at the `sixty_four_track_console_mono` premise `symmetry[index][0] == tracks` with `left: 65, right: 64`. The last sealed record (`artifacts/issue420-rt3`, candidate `1fc6ed1e`, 2026-09-05) carried `symmetric_lanes 64, lanes 129` for that row, so a change since then added one channel-symmetric op unit to the census (`PreparedPlanExecutor::symmetry_counters` sums `[eligible, 1]` per `RuntimeUnit::Op` plus each bank chain's counters) without the benchmark's premise following it. No CI job runs this benchmark, so it drifted silently.

Reproduce: on a clean checkout of `main`, `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1 bash scripts/run-console-benchmark.sh --pure-path-baseline` (any arm); see `console-benchmark.stderr.log`. The refused attempt is preserved in `artifacts/pure-path-baseline/` on branch `codex/pure-path-measurement`.

## Smallest closable slice

Tooling only. Authorized paths: `tools/bench/src/console.rs`, `tools/console-workload/src/lib.rs` (evidence accessors only), their tests, `scripts/test-console-benchmark.sh`, `scripts/console-benchmark-record-lib.jq` (the column comment only; added by Sol's attempt-1 verdict), and this spec.

1. Identify which unit gained eligibility (dump `unit_eligibility()` for the mono fixture; name the op kind and the commit that changed `channel_symmetry()` or the lowering).
2. Decide whether the extra symmetric unit is a legitimate census change (an op that is genuinely channel-symmetric and was previously mis-classified, or a new op) or an engine regression. If legitimate, restate the premise so it counts the mono fixture's *bank lanes* (the quantity the assertion is about) rather than the whole census, or subtract the named non-track units explicitly with a comment naming them. If it is an engine regression, stop and report; do not paper over it.
3. Add a test in `tools/bench` or `tools/console-workload` that pins the mono fixture's per-unit eligibility rows so the next census drift fails at `cargo test`, not at the next benchmark run.

## Non-goals

No engine change, no benchmark run, no fixture change, no new rows.

## Objective gates

1. `cargo test -p bench -p console-workload` green, including the new pin.
2. `bash scripts/test-console-benchmark.sh` and `bash scripts/check-bench-policy.sh` green.
3. A warmup launch of `scripts/run-console-benchmark.sh` on the fixed tree gets past the mono premise (run with the uncontrolled escape hatch into a throwaway arm directory and delete the artifact directory afterwards, or use the runner's preflight if one exists); no record is claimed.

## Dependencies

Blocks the paired measurement of the pure-path batch (#885, #886, #898, #900).

## Attempt 1 diagnosis and decision (Terra)

Dump of `SessionRuntime::new(..).unit_eligibility()` for both mono arms at `Simd8` on a16bc5b6
(the two arms are identical):

| units | banked | stages | upstream of seam | lane tracks | eligible | census |
| --- | --- | --- | --- | --- | --- | --- |
| 0-63 | no | 1 | 1 | `chNN`, one each | no | 0 of 64 |
| 64-71 | yes | 6 | 4 | 8 tracks each, `ch00`..`ch63` in order | all | 64 of 64 |
| 72 | no | 1 | 0 | `""` | yes | 1 of 1 |

Census `[65, 129]`. No unit was added (`lanes` is still 129); unit 72 flipped from ineligible to
eligible. It is the `main-out` output node. Units 0-63 are the tracks' `Input` stages, bound to the
harness's `FrozenGraphSource`, a host `GraphRuntimeProcessor` whose default `channel_symmetry()`
is `DECLINED`.

Responsible commit: `d1cb3653` (issue #221 attempt 1, 2026-09-11) changed
`tools/console-workload/src/lib.rs::source_binding` so the non-input nodes bind through
`GraphNodeBinding::identity(node)` instead of the harness's own `GraphIdentity`, a do-nothing
`Box<dyn GraphRuntimeProcessor>` (#221 asked for this, following audit #103 F1). The engine lowers
an identity binding to `NodeKind::Identity`, which reports `SYMMETRIC`. The opaque processor had
lowered to `NodeKind::Bound` and declined. `13c974cf` then re-pinned
`chain_shape.rs` to `(64, [65, 129])` with a master-row discriminant, but `tools/bench/src/console.rs`
still read the census's eligible half, and no CI job runs the benchmark.

Counterfactual, run and then reverted: binding the output through an opaque do-nothing processor
again gives `[64, 129]` for both arms, unit 72 goes back to `[false]`, and every other row stays
the same. `NodeKind::channel_symmetry()` and the `DECLINED` default match between the sealed
candidate `1fc6ed1e` and `main`, so the engine's classification did not change.

Verdict: a legitimate census change, not an engine regression. The unit is not a track, so its
lane names no track. It renders nothing upstream of the seam, so its witness is vacuous. The
collapse never acts on it, because `arm_mono_collapse` arms bank chains only. `SYMMETRIC` is true
for an identity. The old premise was right only by coincidence, while every non-bank unit declined.

Decision: the premise counts the bank chains' track lanes. The new evidence accessor
`SessionRuntime::bank_symmetry_counters()` returns `[eligible, lanes]` over the lanes of banked,
non-vacuous units that name a track. Its doc lists what it leaves out and why: single ops (the
host-bound inputs and the identity output, plus route ops where a fold declines), vacuous chains,
and nameless lanes. The bench asserts `bank_symmetry_counters() == [tracks, tracks]` for each arm.
The `console_mono` record now emits `symmetric_lanes` from that count and `lanes` from the whole
census. That gives `64`/`129`, field for field the same as every sealed mono record (`strip4`,
`mono2`, `mono3`, `issue420-rt3`), so the jq validator is unchanged.
`chain_shape.rs::the_mono_row_pairs_unit_rows_are_pinned` pins every field of every row of both
arms. It reads the chain count from the plan and fixes everything else. It also pins the census
`[65, 129]`, rows summing to the census, `bank_symmetry_counters() == [64, 64]` and 64
structural mono tracks. The validator suite gains the mutation `.symmetric_lanes = 65`.

Red mutations, run and reverted:
- Opaque output binding: the pin fails at unit 72 (`lane_eligible [false]`).
- `banked` filter dropped from the accessor: every row still matches, and the premise count fails
  at `[64, 128]`.

## Attempt 1 gate evidence (Terra)

Code under test: `39c727a2`.

1. `cargo test -p bench -p console-workload` passed:
   - bench: 61
   - console-workload `automation`: 4
   - console-workload `chain_shape`: 22, including the new pin
   - console-workload `placement`: 3

   The following also passed:
   - `cargo clippy -p bench -p console-workload --all-targets --all-features -- -D warnings`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `cargo fmt --all --check`
2. Both scripts passed:
   - `bash scripts/test-console-benchmark.sh`: `PASS (real runner/workload/timing invocations: 0/0/0)`
   - `bash scripts/check-bench-policy.sh`: `ok`
3. On `main`, every runner arm's artifact directory already holds runner output, so every arm
   refuses to overwrite, and the runner has no preflight. The proof therefore ran on a throwaway
   local branch. Its one extra commit, `fd636608`, added the arm `--issue911-warmup-proof` to
   `scripts/run-console-benchmark.sh` and nothing else. It ran with
   `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1`.
   - The warmup got past the mono premise (`warmup_launches: 1`).
   - The whole invocation then finished in about 90 seconds, before it could be stopped after the
     warmup. The disposition reads `PASS complete`, `measured_rounds_completed: 2`,
     `measurement_control: uncontrolled`. The aggregate validator accepted all 46 records.
   - Both `console_mono` records carry `mono_source_tracks 64`, `symmetric_lanes 64` and
     `lanes 129`, with equal arm digests, and the record validator accepted each one.
   - This is an uncontrolled run on a throwaway candidate, so no number from it is claimed or
     recorded.
   - The artifact directory and the throwaway branch were deleted afterwards. The worktree is
     clean.

## Sol attempt 1 verdict: PASS

Adversarial review (Fable 5.1, high effort) against `39c727a2` and `ea400d63` on base `a16bc5b6`.
No blocking findings. Verified: `d1cb3653` changed the harness's `source_binding` from an opaque
`GraphIdentity` processor to `GraphNodeBinding::identity`, so the output node lowers to
`NodeKind::Identity` and reports SYMMETRIC; `NodeKind::channel_symmetry()` is byte-identical
between the sealed candidate `1fc6ed1e` and `main`; `arm_mono_collapse` skips every non-bank
unit, so the identity output's witness is vacuous and the census change is legitimate; no file
under `crates/` or `hosts/` changed. The restated premise (`bank_symmetry_counters() ==
[tracks, tracks]`) is strictly stronger for the property it is about (it also fails if a track
stops banking, lands on a vacuous chain, on a nameless lane, or is duplicated). Both mutations
re-applied and reverted, red at the named pin-test lines. Every sealed mono record (`strip4`,
`mono2`, `mono3`, `mono3-baseline`, `issue368`, `issue420-rt3`) reads `64/129`, so the record
columns are numerically unchanged; the one should-fix was that the record-lib comment still
called `symmetric_lanes` a census half, corrected in this commit. Reviewer-run gates: `cargo fmt
--all --check`, `cargo test -p bench -p console-workload`, the two-crate clippy, `test-console-
benchmark.sh`, `check-bench-policy.sh`, all green. Not re-verified: the throwaway warmup-proof
run (its artifact directory was deleted by design) and the workspace-wide clippy.
