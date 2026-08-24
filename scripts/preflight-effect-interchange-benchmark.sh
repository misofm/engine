#!/usr/bin/env bash
# Builds and seals Issue 081's benchmark without executing the benchmark binary.
set -euo pipefail
[[ $# -eq 0 ]] || { printf 'usage: preflight-effect-interchange-benchmark.sh\n' >&2; exit 2; }
script_directory=${0%/*}
[[ "$script_directory" != "$0" ]] || script_directory=.
root="$(cd "$script_directory/.." && pwd)"
cd "$root"

for tool in awk bash cargo chmod cp git ln mkdir mktemp python3 rm sha256sum; do
    command -v "$tool" >/dev/null 2>&1 || {
        printf 'effect interchange benchmark preflight: missing tool %s\n' "$tool" >&2
        exit 1
    }
done

artifact_dir="$root/target/issue081"
nonbenchmark_seal="$artifact_dir/nonbenchmark.seal.json"
binary="$artifact_dir/miso_engine_effect_interchange_bench"
preflight_seal="$artifact_dir/benchmark-preflight.seal.json"
raw="$artifact_dir/benchmark.raw.jsonl"
accepted="$artifact_dir/benchmark.accepted.jsonl"
disposition="$artifact_dir/benchmark.disposition.json"
prelaunch_disposition="$artifact_dir/benchmark.prelaunch.disposition.json"
stderr_log="$artifact_dir/benchmark.stderr.log"
mkdir -p "$artifact_dir"
[[ ! -L "$artifact_dir" ]] || { printf 'effect interchange benchmark preflight: artifact directory symlink\n' >&2; exit 1; }
[[ -f "$nonbenchmark_seal" && ! -L "$nonbenchmark_seal" ]] || {
    printf 'effect interchange benchmark preflight: missing nonbenchmark seal\n' >&2
    exit 1
}
for path in "$binary" "$preflight_seal" "$raw" "$accepted" "$disposition" "$prelaunch_disposition" "$stderr_log"; do
    [[ ! -e "$path" && ! -L "$path" ]] || {
        printf 'effect interchange benchmark preflight: refusing existing artifact %s\n' "$path" >&2
        exit 1
    }
done

[[ -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || {
    printf 'effect interchange benchmark preflight: candidate is not clean\n' >&2
    exit 1
}
branch="$(git branch --show-current)"
commit="$(git rev-parse --verify HEAD)"
tree="$(git rev-parse HEAD^{tree})"
[[ "$branch" == codex/batch-qualification-081 ]] || {
    printf 'effect interchange benchmark preflight: wrong branch\n' >&2
    exit 1
}
accepted_sha="$(sha256sum fixtures/effect-interchange/v1/ACCEPTED.sha256 | awk '{print $1}')"
[[ "$accepted_sha" == 1aaa96dc731c0da3dabb2f8ecd7c2bf803078b580a38cccfccf1ffe280c83588 ]] || {
    printf 'effect interchange benchmark preflight: accepted manifest identity changed\n' >&2
    exit 1
}
lock_sha="$(sha256sum Cargo.lock | awk '{print $1}')"
qualification_sha="$(sha256sum scripts/check-effect-interchange-qualification.sh | awk '{print $1}')"
target_sha="$(sha256sum scripts/check-effect-interchange-targets.sh | awk '{print $1}')"
python3 -I -B - "$nonbenchmark_seal" "$branch" "$commit" "$tree" "$accepted_sha" "$lock_sha" "$qualification_sha" "$target_sha" <<'PY'
import json, pathlib, sys
path = pathlib.Path(sys.argv[1])
actual = json.loads(path.read_text(encoding="utf-8"))
keys = {
    "schema_version", "issue", "kind", "branch", "candidate_commit", "candidate_tree",
    "accepted_manifest_sha256", "cargo_lock_sha256", "qualification_checker_sha256",
    "target_checker_sha256", "reference_processes", "mutation_trials", "migration_rows",
    "target_rows", "benchmark_preflight_invocations", "benchmark_runner_invocations",
    "benchmark_workload_invocations", "timed_benchmark_invocations",
}
if set(actual) != keys:
    raise SystemExit("nonbenchmark seal key set")
expected = {
    "schema_version": 1, "issue": 81, "kind": "effect_interchange_nonbenchmark_seal",
    "branch": sys.argv[2], "candidate_commit": sys.argv[3], "candidate_tree": sys.argv[4],
    "accepted_manifest_sha256": sys.argv[5], "cargo_lock_sha256": sys.argv[6],
    "qualification_checker_sha256": sys.argv[7], "target_checker_sha256": sys.argv[8],
    "reference_processes": 100, "mutation_trials": 30000, "migration_rows": 48,
    "target_rows": 5, "benchmark_preflight_invocations": 0,
    "benchmark_runner_invocations": 0, "benchmark_workload_invocations": 0,
    "timed_benchmark_invocations": 0,
}
if actual != expected:
    raise SystemExit("nonbenchmark seal identity or counters")
PY

python3 -I -B scripts/effect-interchange-benchmark-validator.py --self-test
if [[ "${MISO_INTERCHANGE_HERMETIC_CHILD:-0}" != 1 ]]; then
    bash scripts/test-effect-interchange-benchmark.sh
fi

scratch="$(mktemp -d "$artifact_dir/.benchmark-preflight.XXXXXX")"
trap 'rm -rf -- "$scratch"' EXIT
CARGO_TARGET_DIR="$scratch/target" RUSTFLAGS='-D warnings' \
    cargo build --locked --release -p miso-engine-effect-interchange-bench
built="$scratch/target/release/miso_engine_effect_interchange_bench"
[[ -f "$built" && -x "$built" && ! -L "$built" ]] || {
    printf 'effect interchange benchmark preflight: missing built binary\n' >&2
    exit 1
}
cp_binary="$scratch/miso_engine_effect_interchange_bench"
cp -- "$built" "$cp_binary"
chmod 755 "$cp_binary"
ln "$cp_binary" "$binary" || {
    printf 'effect interchange benchmark preflight: binary publication race\n' >&2
    exit 1
}

binary_sha="$(sha256sum "$binary" | awk '{print $1}')"
source_sha="$(sha256sum tools/miso-engine-effect-interchange-bench/src/main.rs | awk '{print $1}')"
tool_manifest_sha="$(sha256sum tools/miso-engine-effect-interchange-bench/Cargo.toml | awk '{print $1}')"
validator_sha="$(sha256sum scripts/effect-interchange-benchmark-validator.py | awk '{print $1}')"
preflight_sha="$(sha256sum scripts/preflight-effect-interchange-benchmark.sh | awk '{print $1}')"
runner_sha="$(sha256sum scripts/run-effect-interchange-benchmark.sh | awk '{print $1}')"
lifecycle_sha="$(sha256sum scripts/test-effect-interchange-benchmark.sh | awk '{print $1}')"
seal_tmp="$scratch/benchmark-preflight.seal.json"
printf '{"schema_version":1,"issue":81,"kind":"effect_interchange_benchmark_preflight","branch":"%s","candidate_commit":"%s","candidate_tree":"%s","binary_sha256":"%s","tool_manifest_sha256":"%s","tool_source_sha256":"%s","fixture_manifest_sha256":"%s","cargo_lock_sha256":"%s","validator_sha256":"%s","preflight_sha256":"%s","runner_sha256":"%s","lifecycle_sha256":"%s","output_sha256":{"descriptor_verify_identity_a":"865a0a5a01ba157bea7f3279ad68cc17db0296655998a9b5307cf759c38656f1","package_verify_cid_select_a":"02e944154ccdc0315b96a7f493a11f6c60f70993750fb26ed766bc3273685d0f","state_verify_reencode_current":"b38a9abad3da50b0c38bd02b9de19b641e79f9a8f48099fbb67d1ec3d481cf48","migration_two_step_bank_restore":"350acfa6e348c27a01afcb9efbd40c51a697aac8bbb6a5fe19dc1eb3c52bf441"},"runner_invocations":0,"workload_invocations":0,"timed_benchmark_invocations":0,"warmup_passes":1,"measured_rounds":2,"records_required":8}\n' \
    "$branch" "$commit" "$tree" "$binary_sha" "$tool_manifest_sha" "$source_sha" "$accepted_sha" "$lock_sha" \
    "$validator_sha" "$preflight_sha" "$runner_sha" "$lifecycle_sha" >"$seal_tmp"
ln "$seal_tmp" "$preflight_seal" || {
    printf 'effect interchange benchmark preflight: seal publication race\n' >&2
    exit 1
}
trap - EXIT
rm -rf -- "$scratch"
printf 'effect interchange benchmark preflight: ok workload_invocations=0 timed_benchmark_invocations=0\n'
