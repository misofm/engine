#!/usr/bin/env bash
# Issue #163 phase 2 step 1 wasm console preflight: everything that can fail without launching the
# workload.
#
# AGENTS.md requires benchmark infrastructure to preflight arguments, schema, output persistence,
# shell exit semantics and overwrite refusal *before* the timed workload runs, so that a runner
# defect cannot consume the one authorised measurement. Nothing here is timed, and nothing here
# instantiates the guest for anything but a shape check.
set -euo pipefail
[[ "$#" -le 1 ]] || { printf 'usage: %s [--after|--issue175|--issue182|--issue-loop-eq-r1|--compressor-round1|--compressor-round1-baseline]
' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"

fail() { printf 'wasm console preflight failure: %s\n' "$1" >&2; exit 1; }

# Mirrors `run-wasm-console-benchmark.sh`'s arms: default is the pre-change browser baseline,
# `--after` is the same rows on the unfused tree (issue #163 phase 2), and `--issue175` is the
# wasm half of the intended-placement family, whose row set the standing fixture changed, and
# `--issue-loop-eq-r1` is the wasm half of the effect-optimization loop's EQ round 1 (identity-
# section elision plus the two-slot cohort chain), which is class A against #175 on every leg.
# `--issue182` is the same strip re-measured after the limiter's effect-optimisation round, which
# is class A and must therefore reproduce every `output_sha256` of the #175 arm exactly.
# `--compressor-round1` and `--compressor-round1-baseline` are the paired arms of the compressor's
# effect-optimisation round: the same rows with and without two class-A kernel changes, so they
# must reproduce each other's digests exactly and differ only in time.
arm=baseline
case "${1:-}" in
    --after) arm=after; shift ;;
    --issue175) arm=issue175; shift ;;
    --issue182) arm=issue182; shift ;;
    --issue-loop-eq-r1) arm=issue-loop-eq-r1; shift ;;
    --compressor-round1) arm=compressor-round1; shift ;;
    --compressor-round1-baseline) arm=compressor-round1-baseline; shift ;;
esac
[[ "$#" == 0 ]] || fail "usage: $0 [--after|--issue175|--issue182|--issue-loop-eq-r1|--compressor-round1|--compressor-round1-baseline]"

if [[ "$arm" == after ]]; then
    artifact_dir="$root/artifacts/issue163-phase2"
elif [[ "$arm" == issue175 ]]; then
    artifact_dir="$root/artifacts/issue175"
elif [[ "$arm" == issue182 ]]; then
    artifact_dir="$root/artifacts/issue182"
elif [[ "$arm" == issue-loop-eq-r1 ]]; then
    artifact_dir="$root/artifacts/issue-loop-eq-r1"
elif [[ "$arm" == compressor-round1 ]]; then
    artifact_dir="$root/artifacts/compressor-round1"
elif [[ "$arm" == compressor-round1-baseline ]]; then
    artifact_dir="$root/artifacts/compressor-round1-baseline"
else
    artifact_dir="$root/artifacts/issue163-phase2-wasm-baseline"
fi
for name in wasm-console-benchmark.raw.jsonl wasm-console-benchmark.accepted.jsonl \
    wasm-console-benchmark.stderr.log wasm-console-benchmark.disposition.json; do
    path="$artifact_dir/$name"
    [[ ! -e "$path" && ! -L "$path" ]] || fail "phase-2 wasm artifact already exists: $path"
done

for tool in awk cmp cp git jq sha256sum wc; do
    command -v "$tool" >/dev/null 2>&1 || fail "required tool is unavailable: $tool"
done

bash scripts/check-console-benchmark-fixture.sh >/dev/null || fail 'fixture check failed'
bash scripts/check-intended-console-fixture.sh >/dev/null || fail 'intended fixture check failed'
bash scripts/test-wasm-console-benchmark.sh >/dev/null || fail 'validator mutation suite failed'
# The admissibility predicates the run is about to be refused by. A precondition whose own
# self-test is red would refuse or admit for the wrong reason, and the run is one-shot.
bash scripts/check-bench-preconditions.sh >/dev/null || fail 'bench precondition self-test failed'
bash scripts/check-bench-policy.sh >/dev/null || fail 'bench policy failed'

# The workspace/all-features form is what CI runs, and it is the form that matters: Cargo unifies
# features across the packages one invocation selects, so a single-package clippy resolves a
# different feature set than the shipped resolution.
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings >/dev/null 2>&1 ||
    fail 'workspace clippy failed'

guest_target=target/ci/issue163-phase2-guest
CARGO_TARGET_DIR="$guest_target" RUSTFLAGS="-C target-feature=+simd128" \
    cargo build --locked --release --quiet --target wasm32-unknown-unknown \
    -p miso-engine-wasm-console-guest || fail 'guest build failed'
guest="$guest_target/wasm32-unknown-unknown/release/miso_engine_wasm_console_guest.wasm"
[[ -f "$guest" ]] || fail 'guest module is missing'

cargo build --locked --release --quiet -p miso-engine-wasm-console || fail 'release build failed'
binary="$root/target/release/miso_engine_wasm_console"
[[ -x "$binary" ]] || fail 'release binary is missing'

# The host refuses to run without its runner's round marker. Proving that here means a direct
# invocation cannot quietly produce an unprovenanced record later.
if MISO_ENGINE_BENCH_ROUND= "$binary" "$guest" >/dev/null 2>&1; then
    fail 'the wasm console host accepted an empty round marker'
fi
if MISO_ENGINE_BENCH_ROUND=1 "$binary" >/dev/null 2>&1; then
    fail 'the wasm console host accepted a missing guest module argument'
fi
if MISO_ENGINE_BENCH_ROUND=1 "$binary" "$guest" extra-argument >/dev/null 2>&1; then
    fail 'the wasm console host accepted a surplus argument'
fi
if MISO_ENGINE_BENCH_ROUND=1 "$binary" /nonexistent.wasm >/dev/null 2>&1; then
    fail 'the wasm console host accepted a guest module that does not exist'
fi
# A guest built without `+simd128` must be refused rather than timed and reported as a simd128
# number. Built into its own target directory so the measured artifact above is not disturbed.
scalar_target=target/ci/issue163-phase2-guest-scalar
CARGO_TARGET_DIR="$scalar_target" cargo build --locked --release --quiet \
    --target wasm32-unknown-unknown -p miso-engine-wasm-console-guest ||
    fail 'scalar guest build failed'
scalar_guest="$scalar_target/wasm32-unknown-unknown/release/miso_engine_wasm_console_guest.wasm"
if MISO_ENGINE_BENCH_ROUND=1 "$binary" "$scalar_guest" >/dev/null 2>&1; then
    fail 'the wasm console host accepted a guest built without simd128'
fi

candidate_commit=$(git rev-parse --verify HEAD)
jq -n -S \
    --arg commit "$candidate_commit" \
    --arg commit_sha256 "$(printf '%s' "$candidate_commit" | sha256sum | awk '{print $1}')" \
    --arg binary_sha256 "$(sha256sum "$binary" | awk '{print $1}')" \
    --arg guest_sha256 "$(sha256sum "$guest" | awk '{print $1}')" \
    --arg host_sha256 "$(sha256sum tools/miso-engine-wasm-console/src/main.rs | awk '{print $1}')" \
    --arg guest_source_sha256 "$(sha256sum tools/miso-engine-wasm-console-guest/src/lib.rs | awk '{print $1}')" \
    --arg subject_sha256 "$(sha256sum tools/miso-engine-console-workload/src/lib.rs | awk '{print $1}')" \
    --arg fixture_sha256 "$(sha256sum fixtures/session/v1/console-sixty-four-track.toml | awk '{print $1}')" \
    --arg standing_fixture_sha256 "$(sha256sum fixtures/session/v1/console-sixty-four-track-intended.toml | awk '{print $1}')" \
    --arg fixture_generator_sha256 "$(sha256sum scripts/derive-intended-console-fixture.py | awk '{print $1}')" \
    --arg runner_sha256 "$(sha256sum scripts/run-wasm-console-benchmark.sh | awk '{print $1}')" \
    --arg validator_sha256 "$(sha256sum scripts/wasm-console-benchmark-validator.jq | awk '{print $1}')" \
    --arg preconditions_sha256 "$(sha256sum scripts/check-bench-preconditions.sh | awk '{print $1}')" \
    '{schema_version: 1, issue: 163, phase: "2-step1",
      kind: "wasm_console_benchmark_preflight",
      workload_launches: 0, warmup_rounds: 1, measured_rounds: 2, records_required: 22,
      candidate_commit: $commit, candidate_commit_sha256: $commit_sha256,
      binary_sha256: $binary_sha256, guest_module_sha256: $guest_sha256,
      host_source_sha256: $host_sha256, guest_source_sha256: $guest_source_sha256,
      subject_source_sha256: $subject_sha256, fixture_sha256: $fixture_sha256,
      standing_fixture_sha256: $standing_fixture_sha256,
      fixture_generator_sha256: $fixture_generator_sha256,
      runner_sha256: $runner_sha256, validator_sha256: $validator_sha256,
      preconditions_sha256: $preconditions_sha256}'

printf 'wasm console benchmark preflight: PASS (workload launches 0)\n' >&2
