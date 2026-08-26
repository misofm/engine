#!/usr/bin/env bash
# Issue-149 console benchmark preflight: everything that can fail without launching the workload.
#
# AGENTS.md requires benchmark infrastructure to preflight arguments, schema, output persistence,
# shell exit semantics and overwrite refusal *before* the timed workload runs, so that a runner
# defect cannot consume the one authorised measurement. Nothing here is timed.
#
# It takes the same optional phase argument the runner does, and checks the overwrite refusal
# against the directory that run would actually write. Hardcoding phase 1's directory made this
# script unusable the moment phase 1's record was committed, which defeated the purpose: the
# preflight has to be runnable immediately before the run it protects.
#
# `--round2-lane` and `--round2-lane-baseline` are the paired class-A arms of round 2's lane
# lowerings; see the runner's header for what they measure and why their digests must match.
set -euo pipefail
phase_directory=issue149
if [[ "$#" == 1 ]]; then
    case "$1" in
        --phase2) phase_directory=issue149-phase2 ;;
        --phase3) phase_directory=issue149-phase3 ;;
        --issue163-phase0) phase_directory=issue163-phase0 ;;
        --issue163-phase1) phase_directory=issue163-phase1 ;;
        --issue163-phase2) phase_directory=issue163-phase2 ;;
        --issue163-phase3) phase_directory=issue163-phase3 ;;
        --issue163-phase4) phase_directory=issue163-phase4 ;;
        --issue175) phase_directory=issue175 ;;
        --issue182) phase_directory=issue182 ;;
        --issue-loop-eq-r1) phase_directory=issue-loop-eq-r1 ;;
        --compressor-round1) phase_directory=compressor-round1 ;;
        --compressor-round1-baseline) phase_directory=compressor-round1-baseline ;;
        --round1-composed) phase_directory=round1-composed ;;
        --issue184) phase_directory=issue184 ;;
        --round2-lane) phase_directory=round2-lane ;;
        --round2-lane-baseline) phase_directory=round2-lane-baseline ;;
        --round2-eqrack) phase_directory=round2-eqrack ;;
        --round2-eqrack-baseline) phase_directory=round2-eqrack-baseline ;;
        --round2-comp) phase_directory=round2-comp ;;
        --round2-comp-baseline) phase_directory=round2-comp-baseline ;;
        *) printf 'usage: %s [--phase2|--phase3|--issue163-phase0|--issue163-phase1|--issue163-phase2|--issue163-phase3|--issue163-phase4|--issue175|--issue182|--issue-loop-eq-r1|--compressor-round1|--compressor-round1-baseline|--round1-composed|--issue184|--round2-lane|--round2-lane-baseline|--round2-eqrack|--round2-eqrack-baseline|--round2-comp|--round2-comp-baseline]\n' "$0" >&2; exit 2 ;;
    esac
elif [[ "$#" != 0 ]]; then
    printf 'usage: %s [--phase2|--phase3|--issue163-phase0|--issue163-phase1|--issue163-phase2|--issue163-phase3|--issue163-phase4|--issue175|--issue182|--issue-loop-eq-r1|--compressor-round1|--compressor-round1-baseline|--round1-composed|--issue184|--round2-lane|--round2-lane-baseline|--round2-eqrack|--round2-eqrack-baseline|--round2-comp|--round2-comp-baseline]\n' "$0" >&2
    exit 2
fi
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"

fail() { printf 'console preflight failure: %s\n' "$1" >&2; exit 1; }

artifact_dir="$root/artifacts/$phase_directory"
for name in console-benchmark.raw.jsonl console-benchmark.accepted.jsonl \
    console-benchmark.stderr.log console-benchmark.disposition.json; do
    path="$artifact_dir/$name"
    [[ ! -e "$path" && ! -L "$path" ]] || fail "issue-149 artifact already exists: $path"
done

for tool in awk cmp cp git jq sha256sum wc; do
    command -v "$tool" >/dev/null 2>&1 || fail "required tool is unavailable: $tool"
done

bash scripts/check-console-benchmark-fixture.sh >/dev/null || fail 'fixture check failed'
bash scripts/check-intended-console-fixture.sh >/dev/null || fail 'intended fixture check failed'
bash scripts/test-console-benchmark.sh >/dev/null || fail 'validator mutation suite failed'
# The admissibility predicates the run is about to be refused by. A precondition whose own
# self-test is red would refuse or admit for the wrong reason, and the run is one-shot.
bash scripts/check-bench-preconditions.sh >/dev/null || fail 'bench precondition self-test failed'

cargo test --locked -p miso-engine-bench >/dev/null || fail 'bench crate tests failed'
# The workspace/all-features form is what CI runs, and it is the form that matters: Cargo unifies
# features across the packages one invocation selects, so a single-package clippy resolves a
# different feature set and reports lints that the shipped resolution does not have.
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings >/dev/null 2>&1 ||
    fail 'workspace clippy failed'
cargo build --locked --release --quiet -p miso-engine-bench || fail 'release build failed'

binary="$root/target/release/miso_engine_bench"
[[ -x "$binary" ]] || fail 'release binary is missing'
# The subject refuses to run without its runner's round marker. Proving that here means a direct
# invocation cannot quietly produce an unprovenanced record later.
if MISO_ENGINE_BENCH_ROUND= "$binary" console >/dev/null 2>&1; then
    fail 'the console subject accepted an empty round marker'
fi
if "$binary" console extra-argument >/dev/null 2>&1; then
    fail 'the console subject accepted an argument'
fi

candidate_commit=$(git rev-parse --verify HEAD)
jq -n -S \
    --arg commit "$candidate_commit" \
    --arg commit_sha256 "$(printf '%s' "$candidate_commit" | sha256sum | awk '{print $1}')" \
    --arg binary_sha256 "$(sha256sum "$binary" | awk '{print $1}')" \
    --arg subject_sha256 "$(sha256sum tools/miso-engine-bench/src/console.rs | awk '{print $1}')" \
    --arg floor_table_sha256 "$(sha256sum tools/miso-engine-bench/src/floor.rs | awk '{print $1}')" \
    --arg fixture_sha256 "$(sha256sum fixtures/session/v1/console-sixty-four-track.toml | awk '{print $1}')" \
    --arg standing_fixture_sha256 "$(sha256sum fixtures/session/v1/console-sixty-four-track-intended.toml | awk '{print $1}')" \
    --arg fixture_generator_sha256 "$(sha256sum scripts/derive-intended-console-fixture.py | awk '{print $1}')" \
    --arg runner_sha256 "$(sha256sum scripts/run-console-benchmark.sh | awk '{print $1}')" \
    --arg record_validator_sha256 "$(sha256sum scripts/console-benchmark-record-validator.jq | awk '{print $1}')" \
    --arg aggregate_validator_sha256 "$(sha256sum scripts/console-benchmark-validator.jq | awk '{print $1}')" \
    --arg library_sha256 "$(sha256sum scripts/console-benchmark-record-lib.jq | awk '{print $1}')" \
    --arg preconditions_sha256 "$(sha256sum scripts/check-bench-preconditions.sh | awk '{print $1}')" \
    '{schema_version: 1, issue: 149, kind: "console_benchmark_preflight",
      workload_launches: 0, warmup_rounds: 1, measured_rounds: 2, records_required: 34,
      candidate_commit: $commit, candidate_commit_sha256: $commit_sha256,
      binary_sha256: $binary_sha256, benchmark_source_sha256: $subject_sha256,
      floor_table_sha256: $floor_table_sha256,
      fixture_sha256: $fixture_sha256,
      standing_fixture_sha256: $standing_fixture_sha256,
      fixture_generator_sha256: $fixture_generator_sha256, runner_sha256: $runner_sha256,
      record_validator_sha256: $record_validator_sha256,
      aggregate_validator_sha256: $aggregate_validator_sha256,
      validator_library_sha256: $library_sha256,
      preconditions_sha256: $preconditions_sha256}'

printf 'console benchmark preflight: PASS (workload launches 0)\n' >&2
