#!/usr/bin/env bash
# Native console benchmark preflight: everything that can fail without launching the workload.
#
# AGENTS.md requires benchmark infrastructure to preflight arguments, schema, output persistence,
# shell exit semantics and overwrite refusal *before* the timed workload runs, so that a runner
# defect cannot consume the one authorised measurement. Nothing here is timed.
#
# Usage: preflight-console-benchmark.sh [--swap] --step NAME
#
# It takes the runner's one form and checks the overwrite refusal against the directory that run
# would actually write, `artifacts/steps/NAME`, so it is runnable immediately before the run it
# protects. Any other invocation, including none, is a usage error (exit 2). #1025 retired the
# historical one-shot arms; the runner's header points at where they are kept.
#
# `--swap` (#1286) preflights the runner's `--swap` mode: the `bench swap` subject's artifact names,
# its validator (whose refusals `cargo test -p bench` runs on a record the subject printed) and its
# untimed self-test (`bench swap --preflight`, gate 1), in place of the console subject's.
set -euo pipefail
subject=console
if [[ "$#" == 3 && "$1" == --swap ]]; then
    subject=swap
    shift
fi
if [[ "$#" != 2 || "$1" != --step ]]; then
    printf 'usage: %s [--swap] --step NAME\n' "$0" >&2
    exit 2
fi
[[ "$2" =~ ^[a-z0-9][a-z0-9-]{0,63}$ ]] || { printf 'invalid --step name: %s\n' "$2" >&2; exit 2; }
step_directory="steps/$2"
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$root"

fail() { printf 'console preflight failure: %s\n' "$1" >&2; exit 1; }

artifact_dir="$root/artifacts/$step_directory"
artifact=console-benchmark
[[ "$subject" == swap ]] && artifact=swap-benchmark
for name in raw.jsonl accepted.jsonl stderr.log disposition.json core-clock.csv; do
    path="$artifact_dir/$artifact.$name"
    [[ ! -e "$path" && ! -L "$path" ]] || fail "console artifact already exists: $path"
done

for tool in awk cmp cp git jq sha256sum wc; do
    command -v "$tool" >/dev/null 2>&1 || fail "required tool is unavailable: $tool"
done

bash scripts/check-console-benchmark-fixture.sh >/dev/null || fail 'fixture check failed'
if [[ "$subject" == console ]]; then
    bash scripts/check-console-fixtures.sh >/dev/null || fail 'console fixture check failed'
    bash scripts/test-console-benchmark.sh >/dev/null || fail 'validator mutation suite failed'
else
    # The validator parses and refuses an empty run; its refusals of real records are the bench
    # crate's tests, run below.
    verdict=$(jq -s -f scripts/swap-benchmark-validator.jq </dev/null 2>/dev/null) ||
        fail 'the swap validator does not parse'
    [[ "$verdict" == false ]] || fail 'the swap validator accepted an empty run'
fi
# The admissibility predicates the run is about to be refused by. A precondition whose own
# self-test is red would refuse or admit for the wrong reason, and the run is one-shot.
bash scripts/check-bench-preconditions.sh >/dev/null || fail 'bench precondition self-test failed'
# #938: the cores the run will pin, chosen as the runner chooses them: the benchmark core and the
# live-producer row's core, one of its L3 that is not its SMT sibling. A host with none refuses
# here rather than after the build.
# shellcheck source=scripts/check-bench-preconditions.sh
source scripts/check-bench-preconditions.sh
bench_cpu=$(bench_highest_cpu "$(< /sys/devices/system/cpu/online)") ||
    fail 'the online CPU list is unreadable'
producer_cpu=$(bench_read_producer_cpu "$bench_cpu") ||
    fail "no live-producer core shares cpu $bench_cpu's L3 outside its SMT siblings"

cargo test --locked -p bench >/dev/null || fail 'bench crate tests failed'
# The workspace/all-features form is what CI runs, and it is the form that matters: Cargo unifies
# features across the packages one invocation selects, so a single-package clippy resolves a
# different feature set and reports lints that the shipped resolution does not have.
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings >/dev/null 2>&1 ||
    fail 'workspace clippy failed'
cargo build --locked --release --quiet -p bench || fail 'release build failed'

binary="$root/target/release/bench"
[[ -x "$binary" ]] || fail 'release binary is missing'
# The subject refuses to run without its runner's round marker. Proving that here means a direct
# invocation cannot quietly produce an unprovenanced record later.
if MISO_ENGINE_BENCH_ROUND= "$binary" "$subject" >/dev/null 2>&1; then
    fail "the $subject subject accepted an empty round marker"
fi
if "$binary" "$subject" extra-argument >/dev/null 2>&1; then
    fail "the $subject subject accepted an argument"
fi
if [[ "$subject" == console ]]; then
    # Issue #1003: the mixing-automation row's premises, untimed -- every control resolves by id,
    # every automated effect moves bits, restating is bit-exact and the EQ's restatement keeps the
    # collapse.
    "$binary" console --preflight >/dev/null || fail 'the mixing-automation preflight refused'
else
    # Gate 1 (#1286): one move swap and one growth cycle through the release binary, untimed --
    # every swap carries, every adoption is at its `not_before` block with `P == ceil_q(L_max)`,
    # `C_4`'s ΣP is p_max_samples(96 kHz, 128), and no block is silent.
    "$binary" swap --preflight >/dev/null 2>&1 || fail 'the swap self-test refused'
fi

candidate_commit=$(git rev-parse --verify HEAD)
if [[ "$subject" == swap ]]; then
    jq -n -S \
        --arg commit "$candidate_commit" \
        --arg binary_sha256 "$(sha256sum "$binary" | awk '{print $1}')" \
        --arg subject_sha256 "$(sha256sum tools/bench/src/swap.rs | awk '{print $1}')" \
        --arg fixture_sha256 "$(sha256sum fixtures/session/v1/console-sixty-four-track-app.json | awk '{print $1}')" \
        --arg runner_sha256 "$(sha256sum scripts/operator/run-console-benchmark.sh | awk '{print $1}')" \
        --arg validator_sha256 "$(sha256sum scripts/swap-benchmark-validator.jq | awk '{print $1}')" \
        --arg preconditions_sha256 "$(sha256sum scripts/check-bench-preconditions.sh | awk '{print $1}')" \
        --arg bench_cpu "$bench_cpu" --arg producer_cpu "$producer_cpu" \
        '{schema_version: 1, issue: 1286, kind: "swap_benchmark_preflight",
          workload_launches: 0, warmup_rounds: 1, measured_rounds: 2, records_required: 2,
          bench_cpu: $bench_cpu, producer_cpu: $producer_cpu,
          taskset_mask: ($bench_cpu + "," + $producer_cpu),
          candidate_commit: $commit, binary_sha256: $binary_sha256,
          benchmark_source_sha256: $subject_sha256, fixture_sha256: $fixture_sha256,
          runner_sha256: $runner_sha256, validator_sha256: $validator_sha256,
          preconditions_sha256: $preconditions_sha256}'
    printf 'swap benchmark preflight: PASS (workload launches 0)\n' >&2
    exit 0
fi
jq -n -S \
    --arg commit "$candidate_commit" \
    --arg commit_sha256 "$(printf '%s' "$candidate_commit" | sha256sum | awk '{print $1}')" \
    --arg binary_sha256 "$(sha256sum "$binary" | awk '{print $1}')" \
    --arg subject_sha256 "$(sha256sum tools/bench/src/console.rs | awk '{print $1}')" \
    --arg floor_table_sha256 "$(sha256sum tools/bench/src/floor.rs | awk '{print $1}')" \
    --arg fixture_sha256 "$(sha256sum fixtures/session/v1/console-sixty-four-track.json | awk '{print $1}')" \
    --arg standing_fixture_sha256 "$(sha256sum fixtures/session/v1/console-sixty-four-track-intended.json | awk '{print $1}')" \
    --arg fixture_generator_sha256 "$(sha256sum scripts/derive-intended-console-fixture.py | awk '{print $1}')" \
    --arg mono_fixture_sha256 "$(sha256sum fixtures/session/v1/console-sixty-four-track-mono.json | awk '{print $1}')" \
    --arg mono_fixture_generator_sha256 "$(sha256sum scripts/derive-mono-console-fixture.py | awk '{print $1}')" \
    --arg sends_fixture_sha256 "$(sha256sum fixtures/session/v1/console-sixty-four-track-sends.json | awk '{print $1}')" \
    --arg sends_fixture_generator_sha256 "$(sha256sum scripts/derive-sends-console-fixture.py | awk '{print $1}')" \
    --arg runner_sha256 "$(sha256sum scripts/operator/run-console-benchmark.sh | awk '{print $1}')" \
    --arg record_validator_sha256 "$(sha256sum scripts/console-benchmark-record-validator.jq | awk '{print $1}')" \
    --arg aggregate_validator_sha256 "$(sha256sum scripts/console-benchmark-validator.jq | awk '{print $1}')" \
    --arg library_sha256 "$(sha256sum scripts/console-benchmark-record-lib.jq | awk '{print $1}')" \
    --arg preconditions_sha256 "$(sha256sum scripts/check-bench-preconditions.sh | awk '{print $1}')" \
    --arg bench_cpu "$bench_cpu" --arg producer_cpu "$producer_cpu" \
    '{schema_version: 1, issue: 149, kind: "console_benchmark_preflight",
      workload_launches: 0, warmup_rounds: 1, measured_rounds: 2, records_required: 78,
      bench_cpu: $bench_cpu, producer_cpu: $producer_cpu,
      taskset_mask: ($bench_cpu + "," + $producer_cpu),
      candidate_commit: $commit, candidate_commit_sha256: $commit_sha256,
      binary_sha256: $binary_sha256, benchmark_source_sha256: $subject_sha256,
      floor_table_sha256: $floor_table_sha256,
      fixture_sha256: $fixture_sha256,
      standing_fixture_sha256: $standing_fixture_sha256,
      fixture_generator_sha256: $fixture_generator_sha256,
      mono_fixture_sha256: $mono_fixture_sha256,
      mono_fixture_generator_sha256: $mono_fixture_generator_sha256,
      sends_fixture_sha256: $sends_fixture_sha256,
      sends_fixture_generator_sha256: $sends_fixture_generator_sha256, runner_sha256: $runner_sha256,
      record_validator_sha256: $record_validator_sha256,
      aggregate_validator_sha256: $aggregate_validator_sha256,
      validator_library_sha256: $library_sha256,
      preconditions_sha256: $preconditions_sha256}'

printf 'console benchmark preflight: PASS (workload launches 0)\n' >&2
