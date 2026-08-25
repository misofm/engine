# Issue #163 phase 3 — bank interleave — measurement record

Attempt 1 (`console-benchmark.attempt-1-refused.*`) is kept here deliberately. It is a
**precondition refusal**, not a measurement: the runner declined before launching the workload
because the one-minute load average was still decaying from the candidate's own build
(`workload_process_launches: 0`, `measured_rounds_completed: 0`, `raw_sha256: null`). Nothing was
timed, so nothing was discarded when the canonical file names were freed for the run that follows.
It is committed rather than deleted so that the count of runner invocations against this phase is
on the record.
