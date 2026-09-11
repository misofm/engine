# Qualify causal gate/expander CPU cost on an active-input workload

## Scope status

Astra medium read-only scope PASS on main fe9fc8d4e29ca1d6bca3e101b68b43b05f8078b9, conditional on the required no-workload preflight below. Root activates only after preceding issue delivery/synchronization. Luna xhigh implements; Astra medium adversarially verifies; maximum five coherent attempts. Root owns checkpoints, measurement authorization/invocation, raw evidence, GitHub and delivery. This replaces historical routing prose without relabeling earlier evidence.

Smallest closable outcome: one trustworthy descriptive measurement of actual prepared causal gate scalar and native W8 render, with demonstrated opening/closing on both channels, explicit counts/units and preserved raw two-round evidence. No production DSP changes, performance target, optimization, baseline/speedup, class-A floor, browser benchmark or listening claim.

## Existing entry and exact boundaries

There is no existing gate descriptive test entry in current `crates/gate-expander/tests/`; do not pretend one exists. Reuse the existing consolidated `tools/bench` binary, adding a narrow native `gate-active` subject through its existing dispatch. Use `effect_compiler::launch_native_effect_registry` and the current gate factory by ID, avoiding a new dependency: effect-compiler/effect-contract/lane/bench-support are already available to bench. The subject is one workload, not a generic effect runner. Reuse bench_support timing, JSON and metadata helpers; do not duplicate their implementations.

Allowed implementation paths:

- `tools/bench/src/main.rs`: subject registration/dispatch only.
- `tools/bench/src/gate_active.rs`: one bounded prepared scalar/W8 workload, untimed activity/preflight and measured modes, focused unit tests.
- `scripts/run-gate-active-benchmark.py`: small issue-specific process/evidence orchestration, not a configurable benchmark framework.
- `scripts/test-gate-active-benchmark.py`: hermetic argument/schema/runner failure controls with stub subject and stub perf, no timed DSP.
- `.github/ISSUE_SPECS/746-qualify-causal-gate-active-input-cpu-cost.md`: root installs scope/decision evidence.

No production gate/effect/runtime changes, manifest/dependency/lock changes, console-workload or broad console-runner mutation, global floor changes, generic counter library, artifacts/pins or existing benchmark resealing. If public registry access proves insufficient, report that precise API fact for a narrow scope amendment before adding dependencies.

## Frozen workload

48,000Hz;128frames per block; Normal; enabled; DualMono; no external sidechain; no automation. Actual scalar prepare and actual homogeneous native AVX2 W8 bind, rejecting fallback or mismatched backend. Bank lane count8 and channels2 are explicit; scalar width1 is a separate row, not eight independent scalar effects called a bank. Require actual metadata latency0 and current descriptor identity.

Parameters by stable ID: threshold1=-20dB; ratio2=20; range3=48dB; hysteresis4=6dB; attack5=1ms; hold6=0ms; release7=5ms. Resolve compact indices from descriptors rather than inventing a second ID map. Initial records are legal L/R ordered records, identical parameter values across tracks/channels; stimulus phases distinguish them.

Fresh planar/AoSoA input is written BEFORE each timed block, never recycled processed output. Define block phase `(block + 8*track + 32*channel) % 64`: phases0..31 have magnitude0.5, phases32..63 magnitude1/1024. Alternate sample sign by `(absolute_sample + track + channel) % 2`. Both input magnitudes are exactly representable f32 and nonzero. No PRNG, trigonometry or allocation is needed per block. Channel offset32 ensures opposite opening/closing phases; track offsets8 ensure active heterogeneous bank state while all lanes retain one program.

One warmup phase:8,192blocks for EACH width. Two measured phases:32,768blocks per width per round, order scalar then W8, no best-of selection. Start each phase/width from an independently fresh prepared state outside the timer with block numbering0; periods divide all frozen counts. Timing region is only the actual process call via `bench_support::timing::timed`; fill, parameter/control construction, report checks, activity statistics, hashing, serialization, I/O and snapshots are outside. Retain per-block elapsed-ns observations or aggregate sum plus standard shared percentiles; do not subtract an unmeasured empty-loop estimate. State preparation and syscall timing overhead are excluded except the normal process dispatch/call itself. Report small-block clock overhead as a limitation.

For each width and each track/channel require at least one completed high plateau whose final-block output/input magnitude ratio exceeds0.9 and a completed low plateau whose final-block ratio is below0.01. Require finite output and nonzero input/output; record witnessed high/low ratios and counts per channel/lane. Inspect the actual returned scalar/bank reports for every block and require all fault/invalid-span counters0; no hardcoded zero report. These two thresholds are broad activity discriminators supported by the frozen48dB range and1ms/5ms attack/release over32-block plateaus, not numerical DSP accuracy tolerances. Native causal/oracle gates remain owned by#738.

## CLI and preflight contract

Subject modes: `bench gate-active --preflight`, `--phase warmup`, `--phase 1`, `--phase 2`; reject unknown, duplicate, contradictory or extra arguments before preparing/rendering. Counts/rates/widths are frozen constants, not user-adjustable CLI surface. `--preflight` runs one128-block activity cycle per width with NO timing calls and emits its activity/count metadata; this is a correctness check, not warmup or measured evidence.

Runner modes: `python3 -B scripts/run-gate-active-benchmark.py --preflight --binary ABS --output ABS --cpu N` and the same with `--run` instead of `--preflight`. Runner preflight must not create/consume the final output directory or launch phase warmup/1/2. Probe output parent writability using a separate temporary sibling; reject an existing output path, bad executable/CPU, missing provenance, unsupported architecture or unavailable required perf clock before any measured phase. Read actual CPU affinity/topology/governor/current compiler facts; no hardcoded historical cpu15/7 assumption. Use the repository's existing exclusive heavy-work control if active, preserve host scheduling; do not alter system/cgroup/governor settings.

Before the sole run, build the subject, freeze its committed source identity and binary SHA256, complete subject activity preflight and runner/schema/failure self-tests, then obtain Astra verification of those preflight results. Root chooses one output path outside any disposable worktree, e.g. `/tmp/issue746-measurement-<full-source-commit>`, and preserves substantive final records in pushed issue evidence. Raw machine-local output location is disclosed, not treated as durable GitHub attachment.

## Exactly-once timing and cycle meaning

One root runner invocation creates the output directory exclusively and launches exactly one warmup phase and two measured phases (each includes both widths). This is one workload invocation, not a retryable benchmark loop. Capture every phase's argv, raw stdout/stderr, exit, source/binary identity and host facts separately. Keep a durable launch manifest and final status even on failure. Launch failure, nonzero child status, invalid output, missing activity, duplicate/missing round or persistence error fails; after workload starts, never repeat or tune timing under this issue.

Reuse the existing console runner's *method*, not its huge workload selection surface: `perf stat -x, -e cycles,task-clock` around each phase obtains counted core cycles/task-clock; the ratio is measured effective core Hz. A no-workload `perf ... -- true` probe belongs in preflight. Reject unsupported/unavailable/zero/nonfinite/ambiguous counters before timing; never substitute advertised CPU GHz or invariant TSC ticks as CPU cycles. Retain original perf CSV. Validate measured-phase effective clock against warmup within existing3% ceiling; a post-run drift failure is preserved and ends timing, no retry.

Per row, lane_samples = blocks *128 *2 *width. Mean render ns/lane-sample = sum(process_elapsed_ns)/lane_samples. `cycles_per_lane_sample` = that mean ns * measured effective core Hz /1e9. Record the phase's clock source/frequency and explicitly call this a wall-render-time conversion using a hardware-counter-derived effective clock; it is NOT a direct per-process-call PMU count, not an isolated_cycles_per_lane_sample floor record, and includes measured dispatch/timer limitations. Publish both original ns and derived cycles, both rounds separately. No floor, gap or speedup columns. If perf cannot supply credible cycles, preserve preflight refusal and report the concrete blocker before workload; do not silently claim a qualified CPU-cycle number.

## Schema and meaningful negative controls

Use one small JSONL schema with phase,width/backend,blocks,frames,channels,lane_samples,timed_call_count,process_elapsed_ns summary, activity arrays, actual report counts, and provenance. Warmup/preflight records must not masquerade as measured rows. Final accepted output contains exactly four measured rows: rounds1/2 x widths1/8, with unique identities and correct units/count arithmetic. Validate finite positive measured times/clocks and independently recompute normalization.

Hermetic runner tests must prove: unknown/extra args reject without child launch; existing output refuses without overwrite; unwritable parent refuses; malformed/missing/duplicate rows reject; wrong width/block/sample normalization rejects; failed child/failed perf exit propagates; output evidence survives post-launch failure; no subsequent round launches after failure. Stub phases record calls so exactly1warmup+2measured and no timed launch in preflight are testable. Activity controls must reject all-high/no-closing, all-low/no-opening and bypass/identity output using the same validator. Missing/nonzero report evidence fails. These are correctness/preflight tests, never timed benchmark invocations.

## Gates and delivery

- Focused bench gate_active unit tests and hermetic Python runner tests.
- `cargo check --locked -p bench`; `cargo clippy --locked -p bench --all-targets -- -D warnings`; fmt/diff check.
- `bash scripts/check-bench-policy.sh` and `bash scripts/test-bench-policy.sh`; existing environment-vocabulary checks only if new environment access is introduced (prefer CLI and existing metadata names).
- `cargo build --locked --release -p bench` before root's no-timing preflight; proportional checks once after source freezes.
- Astra medium verifies preflight and frozen workload before the one run, then verifies actual persisted records afterward as one coherent attempt verdict.

Luna pauses for root's exact-path checkpoint as soon as implementation/focused preflight tests are green. Root records numerical evidence only after the sole run succeeds, obtains required CI and synchronizes/closes746 after upstream evidence. A runner defect after timing creates a separately bounded tooling successor rather than a disguised rerun; performance qualification is not claimed if its required counter/activity/records fail. Human listening remains#26; no subjectively audible-quality PASS is inferred from numerical activity.

## Counter privilege amendment — root's no-workload preflight

Root reported ordinary `perf stat -x, -e cycles,task-clock -- true` refused under paranoid4, while the explicitly invoked existing privilege `sudo -n perf stat -x, -e cycles,task-clock -- true` returned real counters. No timed subject ran, and no sysctl/cgroup/governor setting changed. This removes the counter-availability blocker through existing authorization; do not automatically retry ordinary perf or alter machine policy.

The least invasive permitted integration is one optional runner CLI argument `--perf-executable ABS`, defaulting to the resolved absolute ordinary perf executable. For this host root supplies an external, explicitly selected executable wrapper outside the repository with exactly:

```sh
#!/bin/sh
exec sudo -n /usr/bin/perf "$@"
```

No shell command string or arbitrary extra counter arguments are accepted. The runner invokes the selected executable via an argv array, adding its existing frozen `stat -x, -e cycles,task-clock ... -- <absolute child>` arguments. Preflight and the warmup/measured phases MUST use that same selected executable. The runner stays unprivileged; only the explicitly chosen wrapper uses existing noninteractive sudo. Failure propagates with no password prompt, escalation fallback or machine-setting change. The root-selected binary/output/CPU paths remain absolute and checked before launch.

Freeze and preserve the wrapper absolute path, bytes/SHA256, selected perf absolute path and version, exact expanded logical counter command, and all actual process argv. Recheck wrapper identity at run start against preflight evidence. Record that perf and the measured child run through existing sudo privilege; preserve actual child metadata and ensure required existing environment metadata is forwarded explicitly if sudo's environment filter removes it. Do not pretend the ordinary unprivileged counter command succeeded. The wrapper is root-owned orchestration evidence, not a new installed repository utility or permanent system configuration.

The runner's argument/preflight tests gain the narrow executable-path acceptance/refusal and same-selection checks using a fake executable; they must not require sudo. Existing exactly-once workload, output/exit preservation and genuine measured-counter requirements remain unchanged. This amendment authorizes no timed invocation during scope or implementation and no automatic privilege attempt.

## Activation-base revalidation — after #288

Astra medium scope PASS is revalidated on actual merged main364666c5049418a2f6971ecdc3d4f8275142603a (#288 PR750; exact PR qualification34620150239PASS). The diff from scoped basefe9fc8d4 across tools/bench, bench-support, gate-expander, effect-compiler, existing console counter method, bench-policy gates, target config and Cargo manifest/lock is empty. No workload/API/numeric-contract amendment is needed. Root activates after#288's resulting-main qualification and issue synchronization complete.

Root's explicit external counter wrapper is `/tmp/engine-active-perf-wrapper`, bytes `#!/bin/sh\nexec sudo -n /usr/bin/perf "$@"\n`, SHA256 `aad0cd627c7f09430ff3578b007aeda0c17c752df50f2d7dd986be6ceaf443ba`. Read-only inspection confirmed that hash. Its recorded no-workload perf probe `/tmp/engine-active-perf-preflight.csv` contains positive cycles/task-clock evidence; this is counter access evidence only, not subject timing or measured CPU cost. Ordinary perf remains denied under paranoid4, which was not changed. Use the explicit wrapper path via the approved runner argument; no implicit privilege attempt.

Root reported current CPU15/core15, SMT sibling31, allowed CPUs0-31 and `schedutil` governor. These are provisional provenance to recapture during final preflight, not a claim of permanent exclusivity or a frequency guarantee. The scope does not require changing to performance governor, offline siblings, cgroups or any other system setting. Pin to the explicitly selected valid CPU, record actual sibling/background conditions and governor, and apply the already-frozen counter-derived clock consistency check. Do not reuse historical sibling7 assumptions. No tests or timed subject work were performed in this revalidation.


## Activation — 2026-09-11

Root verified #288 CLOSED afterPR750/main364666c5 and exact PR/main
qualificationPASS, synchronized evidence and removed its clean worktree.
No local numbered spec is missing its GitHub issue. Astra medium scope
revalidation applies to actual main364666c5. Luna xhigh owns implementation
on dedicated codex/gate-active-cpu-746 with checkpoint pushes.
No timed gate workload has run; the existing external wrapper's true-only
probe proves counter access, not benchmark qualification. Root must obtain
preflight review before the sole timed invocation.

## Attempt 1 — implementation checkpoint

Luna xhigh explicitly completed and paused after implementing the four approved
benchmark paths. Reported PASS: locked bench check, strict all-target Clippy,
focused gate_active tests (4), fmt/diff checks, bench-policy check/self-test,
and hermetic Python runner tests. The subject's actual scalar/W8 activity
preflight ran only 128 untimed blocks per width. No warmup or measured DSP ran.
Root inspected the exact path set and diff cleanliness before checkpointing.
Block construction is outside the timer; shared timed/untimed helpers wrap
actual prepared process calls. Failed phases preserve argv and exit records.
The runner forwards existing metadata after the explicit perf wrapper.
Astra source and actual release/preflight review remains pending; this is not
a measurement or final verification PASS.

## Attempt 1 — Astra medium FAIL before timing

Astra medium records FAIL for source90d7975d. Shared metadata ends in a comma;
placing it last emitted invalid JSON. Validator additionally accepted wrong
sample rate/round, nonfinite percentile and negative ratio evidence. Required
negative controls and an actual emitted-record parse check were incomplete.
No production DSP, frozen stimulus, bank binding or process timing-boundary
blocker was found. These findings authorize one bounded second attempt over
the same four implementation paths: repair serialization, enforce frozen
schema/finite valid numerics, and add the missing discriminating controls.
Do not change stimulus, counts, timers, dependencies, production code or scope.

Root release build PASS. Actual wrapper/CPU15 runner --preflight exited1:
`phase subject-preflight stdout line 1 is not JSON`. Raw evidence remains
`/tmp/.issue746-preflight-ne9i3y5b`; transcript is
`/tmp/issue746-a1-real-preflight.log`, hashes
`/tmp/issue746-a1-preflight-sha256.json`, full verdict
`/tmp/issue746-astra-a1-verdict.md`. Subject stdout SHA256
04dfc81297be0d8aae487a6724a8cf9427113bf868743900f5e3bd45dab45e18.
The final measurement directory was not created. No warmup or measured DSP
ran; the exactly-once measurement allowance is unused. This failure is retained
without relabeling the earlier focused test reports as sufficient qualification.

## Attempt 2 — correction checkpoint

Luna xhigh explicitly finished and paused. Corrected the shared-metadata splice
locally; strengthened exact rate/round/count, finite elapsed/activity and actual
report validation; added malformed/nonzero/fractional/NaN/wrong-dimension,
executable-refusal, failure-persistence and known-conversion controls.
Only the existing subject and two Python paths changed. Reported locked check,
strict Clippy, focused bench tests, Python controls, fmt/diff, bench policy and
release build PASS. Actual dirty-tree release preflight was untimed and PASS at
`/tmp/.issue746-preflight-whx04vo9`, with timed_subject_invocations0.
Root will recapture preflight against this clean committed correction, then
request Astra clearance before the still-unused sole measurement invocation.

## Sole measurement — 2026-09-11

Root invoked the runner exactly once after Astra medium preflight clearance,
against clean source `ce75a69bebe5fb6a1aa55d0c6f1a1c9ea7f05a77` and binary SHA256
`1b49e4212da773bdebd81cfc648328eb5600913d7791039140b6396b36846c25`.
The run completed exit0/PASS with exactly warmup8192 and rounds1/2 each32768
blocks per width, scalar thenW8, at48000Hz/128frames. All actual per-block
fault reports were zero; all lane/channel opening and closing witnesses passed.

| Round | Width | Lane-samples | Sum process ns | ns/lane-sample | Effective Hz | Derived cycles/lane-sample |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | 1 | 8388608 | 139377744 | 16.615121841 | 3713018730.405424 | 61.692258605 |
| 1 | 8 | 67108864 | 151567259 | 2.258528158 | 3713018730.405424 | 8.385957354 |
| 2 | 1 | 8388608 | 139480212 | 16.627336979 | 3710584628.522630 | 61.697141007 |
| 2 | 8 | 67108864 | 152223020 | 2.268299758 | 3710584628.522630 | 8.416718217 |

Counter evidence, preserved separately per phase:

| Phase | Counted cycles | task-clock ms | Effective Hz | Drift vs warmup |
| --- | --- | --- | --- | --- |
| warmup | 357982293 | 96.44 | 3711969027.374534 | 0.000000% |
| 1 | 1385661460 | 373.19 | 3713018730.405424 | 0.028279% |
| 2 | 1390430272 | 374.72 | 3710584628.522630 | 0.037296% |

Both measured drift values satisfy the frozen3% ceiling. These cycles are
wall-render-time conversions using phase-wide hardware-counter-derived clock,
not direct per-process PMU counts or isolated floor evidence. Timer/dispatch
cost and phase-wide clock estimation remain limitations. No speedup, capacity,
class-A floor, optimization target or listening claim follows.

Host: AMD EPYC7313P, CPU/core15, sibling31, schedutil, allowedCPUs0–31;
Rust1.97.1/LLVM22.1.6, x86_64-unknown-linux-gnu release, compile-time AVX2/FMA.
Host remains shared and sibling enabled. Root paused its own builds/agents and
waited for an unrelated compiler to finish before invocation; no system setting
changed. Explicit `/tmp/engine-active-perf-wrapper` uses existing `sudo -n
/usr/bin/perf "$@"`; perf and child use that privilege, metadata is explicitly
forwarded inside the command. Perf6.8.12 `/usr/bin/perf` SHA256
2d0953085bf720a25efbe24f853e97d27b1f12f18a398255ff82cbafde254dad;
wrapper SHA256aad0cd627c7f09430ff3578b007aeda0c17c752df50f2d7dd986be6ceaf443ba.
Ordinary perf was denied; no implicit fallback or policy change occurred.

Raw evidence is machine-local outside the disposable worktree at
`/tmp/issue746-measurement-ce75a69bebe5fb6a1aa55d0c6f1a1c9ea7f05a77`:
per-phase stdout/stderr/CSV/argv/exit, launch/final manifests, raw and accepted
JSONL and final status. Transcript `/tmp/issue746-sole-run.log`; all file hashes
`/tmp/issue746-measurement-sha256.json`; actual clean preflight
`/tmp/.issue746-preflight-5_27jd6x`; prelaunch host/process records
`/tmp/issue746-root-host-preparation.json` and
`/tmp/issue746-root-final-before-sole-run.json`. These local paths are disclosed
rather than represented as durable attachments; substantive values are above.

`accepted.jsonl` SHA256 `14193694483227ba3b42efe03fe2313e2f1471d0dfbcc9b9adcb574e9f312b5c`.

`launch-manifest.final.json` SHA256 `fe380608f83a7e56c5fca183e50c29d46768208eca0b15a05bfdb153a199cb2d`.

`status.json` SHA256 `7cbf1fb06a4cf0e93266a13101f0433c6de09741f14ed481e9425401dbbfbdb3`.

## Attempt 2 — Astra medium PASS

Astra independently verified all four measured rows, raw phase evidence,
artifact identities, per-lane/channel activity, actual zero report counts,
phase ordering, clock drift and independently recomputed normalization.
Final verdict `/tmp/issue746-astra-a2-verdict.md`: PASS for the scoped
active-input descriptive qualification. Attempt1 remains FAIL before timing.
The sole measurement allowance is now consumed; no rerun is needed or permitted
under this record. Required PR/main qualification and GitHub closure remain
pending delivery steps; no listening or floor claim is added.

## Delivery complete — 2026-09-11

PR751 merged reviewed d367b420 as main06167eb286f2d6ee46e82daf86936a24801347ee.
Exact PR qualification34625139373 and resulting-main34625814610 both PASS.
GitHub746 CLOSED verified; final evidence comment5638022249 and parent560
comment5638022539 synchronize delivery. Root removed the clean pushed worktree,
retaining branch/history and raw measurement/verdict evidence outside it.
The scoped descriptive gate capability is delivered; no rerun/floor/listening
claim follows. #748 activates separately from this delivered main.
