# Issue #184 — the first floor-annotated console record

This directory is the first console qualification record to carry cycle columns. It exists to be
the measurement `docs/rulings/effect-floor-accounting.md` quotes, and to prove the new columns
end to end: a runner that measures the pinned core's clock, a subject that derives from it, and
two validators that recompute every derived column from the columns it came from.

**The authority does not move.** `artifacts/issue175/` remains the standing 64-track qualification
record for *what* the strip renders. This directory re-measures *what it costs*, on the merged
round-1 tree, and adds *how far that is from the arithmetic's floor*.

| question | answer |
|---|---|
| Did any rendered bit move? | **No.** Nothing in this branch touches an audio path; the change is measurement-plane only. Every `output_sha256` matches the pins in `artifacts/issue175/`. |
| What is new in the record? | Eleven additive columns on every `console_session` row. Sealed records that predate them validate unchanged. |
| Where did the cycles come from? | `perf stat -e cycles,task-clock` over the untimed warmup launch, on the pinned core, under the runner's existing preconditions. |
| How far is the DSP from its floor? | 19.0 % (compressor), 33.5 % (limiter), 34.6 % (EQ), 30.7 % (builtins row). |

## Attempts

| arm | attempt | status | launches |
|---|---|---|---|
| native, `--issue184` | 1 | PASS `controlled` | 3 |

## The instrument

`console-benchmark.core-clock.csv` is the perf-counter evidence, one `perf stat` block per launch:

| launch | cycles | task-clock | derived clock |
|---|---:|---:|---:|
| warmup (exported to the measured rounds) | 12 699 317 490 | 2 327.78 ms | **5 455 548 845 Hz** |
| round 1 | 12 700 773 977 | 2 331.94 ms | 5 446 054 703 Hz |
| round 2 | 12 689 410 476 | 2 331.28 ms | 5 443 122 651 Hz |

A spread of 0.23 %, against the runner's 3 % refusal ceiling. The measured rounds are counted for
exactly this reason: the clock the records were told about has to be the clock that was in force
while the numbers that are kept were being taken, and saying so requires measuring it twice.

## The table

48 kHz, 128-frame quantum, 1 000 observations, p50 µs/block, minimum of the two rounds. `cyc/ls` is
cycles per lane-sample; a block of the 64-track fixture is 16 384 lane-samples.

| row | p50 µs | cyc/ls | floor | % of floor | isolate | isolated % |
|---|---:|---:|---:|---:|---:|---:|
| **console — the intended strip** | 123.685 | 41.185 | 11.892 | 28.9 % | **13.918** *(limiter)* | **33.5 %** |
| console, synthetic, 128 tracks | 246.258 | 41.000 | 11.892 | 29.0 % | — | — |
| console legacy | 86.054 | 28.654 | 7.230 | 25.2 % | 21.051 | 23.3 % |
| eq+compressor on simd1 | 81.816 | 27.243 | 7.230 | 26.5 % | 19.593 | 25.0 % |
| **compressor only** | 72.959 | 24.294 | 5.507 | 22.7 % | **16.691** *(compressor)* | **19.0 %** |
| **eq only** | 37.942 | 12.634 | 4.054 | 32.1 % | **4.984** *(eq)* | **34.6 %** |
| idle (silence) | 38.974 | 12.978 | 2.331 | **18.0 %** | — | — |
| builtins only | 22.833 | 7.603 | 2.331 | 30.7 % | — | — |
| dispatch only (identity) | 21.962 | 7.313 | 2.331 | 31.9 % | — | — |
| nine-track ragged strip | 24.978 | 59.144 | 21.141 | 35.7 % | — | — |
| nine-track eq fixture | 6.092 | 14.425 | *not derived* | — | — | — |

`nine_track_baseline` is the one row with null floors: it is rendered from
`fixtures/session/v1/parametric-eq-nine-track.toml`, which was never inventoried, and its
`floor_basis` says `not_derived` rather than borrowing the console fixture's numbers.

## What the two percentages are

`percent_of_floor` compares the row's **whole** cost — graph dispatch, AoSoA transposes and all —
against the arithmetic its strip requires, so it is a lower bound on how close the row's arithmetic
is to its floor. `isolated_percent_of_floor` is the per-effect number: the row's cost minus the
control row named in `floor_control_row`, against the difference of the two rows' floors. The
control rows are the #163 item 0c decomposition rows, which is what makes the subtraction a
subtraction rather than a comparison of two sessions.

## Boundaries

* **This is not a comparison with an earlier record.** The tree, the fixture and the workloads are
  the merged round-1 ones; the p50 column is directly readable against `artifacts/compressor-round1/`
  only for the rows that exist in both, and this run was not taken as a paired arm of anything.
* **The floors are host-derived.** Every floor is `lane-ops / (8 x 3.7)`, and the 3.7 is a
  measurement of this machine. `docs/rulings/effect-floor-accounting.md` states the sensitivity and
  reproduces the probe.
* **The cycle columns are wall time times a measured clock**, not a per-block counter read. The
  per-block counter read does not exist: reading one inside `timing::timed` would need a counter
  in the timed region, and the region is one block of the production render entry and nothing else.
  What the runner can do, and does, is measure the clock the region ran under and refuse the run if
  it moved.
* **Nothing here is a threshold.** Every record says `descriptive_only: true`.
