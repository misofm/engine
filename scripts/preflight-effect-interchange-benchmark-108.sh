#!/usr/bin/env bash
# Builds and seals Issue 108's benchmark without executing the benchmark binary.
set -euo pipefail
[[ $# -eq 0 ]] || { printf 'usage: preflight-effect-interchange-benchmark-108.sh\n' >&2; exit 2; }
script_directory=${0%/*}
[[ "$script_directory" != "$0" ]] || script_directory=.
root="$(cd "$script_directory/.." && pwd)"
cd "$root"

for tool in awk bash cargo chmod cp git ln mkdir mktemp python3 rm sha256sum stat wc; do
    command -v "$tool" >/dev/null 2>&1 || {
        printf 'effect interchange benchmark preflight: missing tool %s\n' "$tool" >&2
        exit 1
    }
done

artifact_dir="$root/target/issue108"
repair_seal="$artifact_dir/repair.seal.json"
binary="$artifact_dir/miso_engine_effect_interchange_bench"
preflight_seal="$artifact_dir/benchmark-preflight.seal.json"
raw="$artifact_dir/benchmark.raw.jsonl"
accepted="$artifact_dir/benchmark.accepted.jsonl"
disposition="$artifact_dir/benchmark.disposition.json"
prelaunch_disposition="$artifact_dir/benchmark.prelaunch.disposition.json"
stderr_log="$artifact_dir/benchmark.stderr.log"
mkdir -p "$artifact_dir"
[[ ! -L "$artifact_dir" ]] || { printf 'effect interchange benchmark preflight: artifact directory symlink\n' >&2; exit 1; }
[[ -f "$repair_seal" && ! -L "$repair_seal" && "$(stat -c %h "$repair_seal")" == 1 ]] || {
    printf 'effect interchange benchmark preflight: missing repair seal\n' >&2
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
[[ "$branch" == codex/batch-benchmark-108 ]] || {
    printf 'effect interchange benchmark preflight: wrong branch\n' >&2
    exit 1
}
accepted_sha="$(sha256sum fixtures/effect-interchange/v1/ACCEPTED.sha256 | awk '{print $1}')"
[[ "$accepted_sha" == 6403ae6205dbc86a57483f44723cfc107f7f49654532fc648516b7cfed7ae3a5 ]] || {
    printf 'effect interchange benchmark preflight: accepted manifest identity changed\n' >&2
    exit 1
}
while read -r name bytes digest; do
    inherited="$root/target/issue081/$name"
    [[ -f "$inherited" && ! -L "$inherited" && "$(stat -c %h "$inherited")" == 1 && \
       "$(wc -c <"$inherited")" == "$bytes" && \
       "$(sha256sum "$inherited" | awk '{print $1}')" == "$digest" ]] || {
        printf 'effect interchange benchmark preflight: inherited Issue-081 artifact changed: %s\n' "$name" >&2
        exit 1
    }
done <<'EOF'
nonbenchmark.seal.json 833 6d08e2089e806dc366f5c1171398c241f8dfdc520f97808c4e2f6c7f6b83363c
miso_engine_effect_interchange_bench 827232 fad8e39ecd9efa6908b51e7e98c25984f9d97f88b32971581c9a880228758b4c
benchmark-preflight.seal.json 1577 da3c537c16d55b1e71b8aa9f8e4d011796b243e4c6c7969020097098a75035a3
benchmark.raw.jsonl 0 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
benchmark.stderr.log 361 442f071fb23e57a9cb4616c6df7683bee669d8114eacce43b16af812e86d1a93
benchmark.disposition.json 817 8c833293bb3e9f2e981e0be1d379819786d92706627b3fa3fbc64e93b188a5de
EOF
for name in benchmark.accepted.jsonl benchmark.prelaunch.disposition.json; do
    [[ ! -e "$root/target/issue081/$name" && ! -L "$root/target/issue081/$name" ]] || {
        printf 'effect interchange benchmark preflight: forbidden Issue-081 artifact appeared: %s\n' "$name" >&2
        exit 1
    }
done
lock_sha="$(sha256sum Cargo.lock | awk '{print $1}')"
qualification_sha="$(sha256sum scripts/check-effect-interchange-benchmark-108.sh | awk '{print $1}')"
mutation_sha="$(sha256sum scripts/test-effect-interchange-benchmark-108-policy.sh | awk '{print $1}')"
validator_sha="$(sha256sum scripts/effect-interchange-benchmark-108-validator.py | awk '{print $1}')"
preflight_sha="$(sha256sum scripts/preflight-effect-interchange-benchmark-108.sh | awk '{print $1}')"
runner_sha="$(sha256sum scripts/run-effect-interchange-benchmark-108.sh | awk '{print $1}')"
lifecycle_sha="$(sha256sum scripts/test-effect-interchange-benchmark-108.sh | awk '{print $1}')"
source_sha="$(sha256sum tools/miso-engine-effect-interchange-bench/src/main.rs | awk '{print $1}')"
tool_manifest_sha="$(sha256sum tools/miso-engine-effect-interchange-bench/Cargo.toml | awk '{print $1}')"
python3 -I -B - "$repair_seal" "$branch" "$commit" "$tree" "$accepted_sha" "$lock_sha" \
    "$tool_manifest_sha" "$source_sha" "$validator_sha" "$qualification_sha" "$mutation_sha" \
    "$lifecycle_sha" "$preflight_sha" "$runner_sha" <<'PY'
import json, pathlib, sys
path = pathlib.Path(sys.argv[1])
actual = json.loads(path.read_text(encoding="utf-8"))
keys = {
    "schema_version", "issue", "kind", "branch", "candidate_commit", "candidate_tree",
    "accepted_manifest_sha256", "cargo_lock_sha256", "tool_manifest_sha256", "tool_source_sha256",
    "validator_sha256", "checker_sha256", "mutation_sha256", "lifecycle_sha256",
    "preflight_sha256", "runner_sha256", "output_sha256", "issue081_artifacts",
    "focused_regressions", "benchmark_preflight_invocations", "benchmark_runner_invocations",
    "benchmark_workload_invocations", "timed_benchmark_invocations",
}
if set(actual) != keys:
    raise SystemExit("repair seal key set")
expected = {
    "schema_version": 1, "issue": 108, "kind": "effect_interchange_benchmark_repair",
    "branch": sys.argv[2], "candidate_commit": sys.argv[3], "candidate_tree": sys.argv[4],
    "accepted_manifest_sha256": sys.argv[5], "cargo_lock_sha256": sys.argv[6],
    "tool_manifest_sha256": sys.argv[7], "tool_source_sha256": sys.argv[8],
    "validator_sha256": sys.argv[9], "checker_sha256": sys.argv[10],
    "mutation_sha256": sys.argv[11], "lifecycle_sha256": sys.argv[12],
    "preflight_sha256": sys.argv[13], "runner_sha256": sys.argv[14],
    "output_sha256": {
        "descriptor_verify_identity_a": "865a0a5a01ba157bea7f3279ad68cc17db0296655998a9b5307cf759c38656f1",
        "package_verify_cid_select_a": "02e944154ccdc0315b96a7f493a11f6c60f70993750fb26ed766bc3273685d0f",
        "state_verify_reencode_current": "b38a9abad3da50b0c38bd02b9de19b641e79f9a8f48099fbb67d1ec3d481cf48",
        "migration_two_step_bank_restore": "5f23e630182137426fdfe01b74861bdff779b6738bfae8f670359ad0e9ea2777",
    },
    "issue081_artifacts": {
        "nonbenchmark.seal.json": "6d08e2089e806dc366f5c1171398c241f8dfdc520f97808c4e2f6c7f6b83363c",
        "miso_engine_effect_interchange_bench": "fad8e39ecd9efa6908b51e7e98c25984f9d97f88b32971581c9a880228758b4c",
        "benchmark-preflight.seal.json": "da3c537c16d55b1e71b8aa9f8e4d011796b243e4c6c7969020097098a75035a3",
        "benchmark.raw.jsonl": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "benchmark.stderr.log": "442f071fb23e57a9cb4616c6df7683bee669d8114eacce43b16af812e86d1a93",
        "benchmark.disposition.json": "8c833293bb3e9f2e981e0be1d379819786d92706627b3fa3fbc64e93b188a5de",
        "benchmark.accepted.jsonl": None,
        "benchmark.prelaunch.disposition.json": None,
    },
    "focused_regressions": 1, "benchmark_preflight_invocations": 0,
    "benchmark_runner_invocations": 0, "benchmark_workload_invocations": 0,
    "timed_benchmark_invocations": 0,
}
if actual != expected:
    raise SystemExit("repair seal identity or counters")
PY

python3 -I -B scripts/effect-interchange-benchmark-108-validator.py --self-test
if [[ "${MISO_INTERCHANGE_HERMETIC_CHILD:-0}" != 1 ]]; then
    bash scripts/test-effect-interchange-benchmark-108.sh
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
repair_sha="$(sha256sum "$repair_seal" | awk '{print $1}')"
seal_tmp="$scratch/benchmark-preflight.seal.json"
printf '{"schema_version":1,"issue":108,"kind":"effect_interchange_benchmark_preflight","branch":"%s","candidate_commit":"%s","candidate_tree":"%s","binary_sha256":"%s","tool_manifest_sha256":"%s","tool_source_sha256":"%s","fixture_manifest_sha256":"%s","cargo_lock_sha256":"%s","validator_sha256":"%s","checker_sha256":"%s","mutation_sha256":"%s","preflight_sha256":"%s","runner_sha256":"%s","lifecycle_sha256":"%s","repair_seal_sha256":"%s","output_sha256":{"descriptor_verify_identity_a":"865a0a5a01ba157bea7f3279ad68cc17db0296655998a9b5307cf759c38656f1","package_verify_cid_select_a":"02e944154ccdc0315b96a7f493a11f6c60f70993750fb26ed766bc3273685d0f","state_verify_reencode_current":"b38a9abad3da50b0c38bd02b9de19b641e79f9a8f48099fbb67d1ec3d481cf48","migration_two_step_bank_restore":"5f23e630182137426fdfe01b74861bdff779b6738bfae8f670359ad0e9ea2777"},"preflight_invocations":1,"runner_invocations":0,"workload_invocations":0,"timed_benchmark_invocations":0,"warmup_passes":1,"measured_rounds":2,"records_required":8}\n' \
    "$branch" "$commit" "$tree" "$binary_sha" "$tool_manifest_sha" "$source_sha" "$accepted_sha" "$lock_sha" \
    "$validator_sha" "$qualification_sha" "$mutation_sha" "$preflight_sha" "$runner_sha" "$lifecycle_sha" \
    "$repair_sha" >"$seal_tmp"
ln "$seal_tmp" "$preflight_seal" || {
    printf 'effect interchange benchmark preflight: seal publication race\n' >&2
    exit 1
}
trap - EXIT
rm -rf -- "$scratch"
printf 'effect interchange benchmark preflight: ok workload_invocations=0 timed_benchmark_invocations=0\n'
