# Record the stored-automation console benchmark baseline

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A4, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch Q.

## Product outcome

One accepted record of the `console_stored_automation` row exists under `artifacts/steps/<step>/`,
taken on the reference host by the standing runner: what holding and moving 512 automated cells
costs per block on the sixty-four-track console through the C ABI. It replaces note A4's
operation-count bound with a measured number per cell and block, as a descriptive baseline for the
weekly performance pass. No code changes.

## Context

- **The row** is built and frozen by draft 24a *Build the stored-automation console benchmark row*:
  three arms (`none`, `flat`, `moving`) alternated per observation, in-run class-A statement
  `flat == none`, record fields `paired_hold_delta_median_ns` and `paired_motion_delta_median_ns`,
  validators and record counts (64) updated before any timing.
- **The runner.** `scripts/operator/run-console-benchmark.sh --step NAME` (header `:1-40`) refuses
  an existing record, a dirty tree, a host without AVX2 and a failed fixture check, builds `bench`
  in release, refuses an unmet admissibility precondition (`scripts/check-bench-preconditions.sh`),
  takes one untimed warmup and exactly two measured rounds, validates the records with
  `scripts/console-benchmark-validator.jq`, and writes the raw, accepted, disposition, stderr and
  core-clock files under `artifacts/steps/NAME/` (`:51-60`).
  `scripts/operator/preflight-console-benchmark.sh --step NAME` checks everything that can fail
  without launching the workload.
- **Where records live.** `scripts/operator/README.md:54-72`: step records live in
  `artifacts/steps/` while an optimisation batch is open; anything else under `artifacts/` stays
  only while something live cites it.
- **The prior measurement.** The `console_mixing_automation` row's paired ramp delta is about
  205-228 ns per ramping control and block (`artifacts/steps/bus-send-base/console-benchmark.accepted.jsonl:31`,
  `:62`); it pushed live edits and never timed an EQ design.
- **AGENTS.md benchmark rules** (`AGENTS.md:182-189`): one invocation, one warmup, two measured
  rounds; no tuning or retry; a post-workload tooling failure keeps the raw output and becomes a
  tooling issue; a descriptive number drives no optimisation unless a named release budget is
  missed.

## Decisions frozen for this slice

- **D1. One run.** A person runs, on the reference host, with a clean tree at the merge commit of
  draft 24a and every slice it depends on:
  `scripts/operator/preflight-console-benchmark.sh --step <step>`, then
  `scripts/operator/run-console-benchmark.sh --step <step>`, exactly once. `<step>` is lowercase
  kebab-case and names the commit's batch (for example `stored-automation-base`).
- **D2. No retry, no tuning.** If the runner refuses before timing, fix the precondition and run
  again: nothing was timed. If it fails after timing, keep the raw output, record the failure, and
  open a tooling issue; do not rerun to obtain a better number.
- **D3. The record** goes into its own commit, containing only the files the runner wrote. The
  commit message names the host, the measurement control and the two medians of each delta.
- **D4. Report.** The issue's evidence records, per round, `paired_hold_delta_median_ns`,
  `paired_motion_delta_median_ns`, both per cell, and the in-run class-A statement, beside A4's
  operation-count estimate. A gap between the measured and the estimated cost is explained or
  named "not investigated"; it is not chased here.

## Deliverables

1. The accepted record and its sibling files under `artifacts/steps/<step>/`.
2. D4's evidence on the issue.

## Authorized paths

- `artifacts/steps/<step>/` (the runner's output only)

## Non-goals

- Any code, workload, validator or fixture change (draft 24a froze them).
- Any optimisation (weekly performance pass, after a measured miss).
- A browser (V8) measurement.

## Hazards

- **Uncontrolled hosts.** `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1` makes every record say
  `measurement_control: "uncontrolled"`; such a record is still descriptive and must say so in D4.
- **A stale binary.** The runner builds `bench` itself; never pass a prebuilt binary.
- **Overwrite refusal.** A `<step>` that already holds a record is refused; choose a new name, never
  delete an existing record.

## Objective gates

1. **Preflight.** `scripts/operator/preflight-console-benchmark.sh --step <step>` passes with
   `records_required: 64` and workload launches 0.
2. **The record.** `artifacts/steps/<step>/console-benchmark.accepted.jsonl` holds 64 records,
   among them `console_stored_automation` for round 1 and round 2, each with `flat == none` stated
   in-run, and `jq -s -e -L scripts -f scripts/console-benchmark-validator.jq` accepts the file.
3. **Disposition.** `artifacts/steps/<step>/console-benchmark.disposition.json` says `"status":
   "PASS"`, `"runner_invocations": 1`, `"warmup_launches": 1` and
   `"measured_rounds_completed": 2`.
4. **No code moves.** `git diff --stat` of the commit lists only files under `artifacts/steps/<step>/`.

## Test value

- No test is added: the slice adds evidence, not behaviour. Gate 2 is the validator draft 24a
  froze and tested; it turns red if the row is missing, if a round is missing, or if the in-run
  class-A statement failed.

## Dependencies

Batch Q. Direct dependencies:

- Draft 24a *Build the stored-automation console benchmark row*, merged.

Draft 20 *Render stored parametric EQ automation* and its predecessors arrive through draft 24a.
