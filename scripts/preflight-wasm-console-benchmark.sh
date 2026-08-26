#!/usr/bin/env bash
# Issue #163 phase 2 step 1 wasm console preflight: everything that can fail without launching the
# workload.
#
# AGENTS.md requires benchmark infrastructure to preflight arguments, schema, output persistence,
# shell exit semantics and overwrite refusal *before* the timed workload runs, so that a runner
# defect cannot consume the one authorised measurement. Nothing here is timed, and nothing here
# instantiates the guest for anything but a shape check.
set -euo pipefail
[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"

fail() { printf 'wasm console preflight failure: %s\n' "$1" >&2; exit 1; }

# Mirrors `run-wasm-console-benchmark.sh`'s two arms: default is the pre-change browser baseline,
# `--after` is the same nine rows on the unfused tree (issue #163 phase 2).
arm=baseline
if [[ "${1:-}" == "--after" ]]; then
    arm=after
    shift
fi
[[ "$#" == 0 ]] || fail "usage: $0 [--after]"

if [[ "$arm" == after ]]; then
    artifact_dir="$root/artifacts/issue163-phase2"
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
    --arg runner_sha256 "$(sha256sum scripts/run-wasm-console-benchmark.sh | awk '{print $1}')" \
    --arg validator_sha256 "$(sha256sum scripts/wasm-console-benchmark-validator.jq | awk '{print $1}')" \
    --arg preconditions_sha256 "$(sha256sum scripts/check-bench-preconditions.sh | awk '{print $1}')" \
    '{schema_version: 1, issue: 163, phase: "2-step1",
      kind: "wasm_console_benchmark_preflight",
      workload_launches: 0, warmup_rounds: 1, measured_rounds: 2, records_required: 18,
      candidate_commit: $commit, candidate_commit_sha256: $commit_sha256,
      binary_sha256: $binary_sha256, guest_module_sha256: $guest_sha256,
      host_source_sha256: $host_sha256, guest_source_sha256: $guest_source_sha256,
      subject_source_sha256: $subject_sha256, fixture_sha256: $fixture_sha256,
      runner_sha256: $runner_sha256, validator_sha256: $validator_sha256,
      preconditions_sha256: $preconditions_sha256}'

printf 'wasm console benchmark preflight: PASS (workload launches 0)\n' >&2
