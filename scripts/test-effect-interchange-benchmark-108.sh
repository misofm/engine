#!/usr/bin/env bash
# Hermetic Issue 108 validator/preflight/runner lifecycle. Never executes benchmark main.
set -euo pipefail
[[ $# -eq 0 ]] || { printf 'usage: test-effect-interchange-benchmark-108.sh\n' >&2; exit 2; }
script_directory=${0%/*}
[[ "$script_directory" != "$0" ]] || script_directory=.
root="$(cd "$script_directory/.." && pwd)"
cd "$root"

python3 -I -B scripts/effect-interchange-benchmark-108-validator.py --self-test
scratch="$(mktemp -d)"
trap 'rm -rf -- "$scratch"' EXIT
template="$scratch/template"
mkdir -p "$template/scripts" "$template/tools/miso-engine-effect-interchange-bench/src" \
    "$template/fixtures/effect-interchange/v1" "$template/target/issue081" \
    "$template/target/issue108" "$template/bin"
cp scripts/effect-interchange-benchmark-108-validator.py \
    scripts/preflight-effect-interchange-benchmark-108.sh \
    scripts/run-effect-interchange-benchmark-108.sh \
    scripts/test-effect-interchange-benchmark-108.sh \
    scripts/check-effect-interchange-benchmark-108.sh \
    scripts/test-effect-interchange-benchmark-108-policy.sh "$template/scripts/"
cp tools/miso-engine-effect-interchange-bench/src/main.rs \
    "$template/tools/miso-engine-effect-interchange-bench/src/main.rs"
cp tools/miso-engine-effect-interchange-bench/Cargo.toml \
    "$template/tools/miso-engine-effect-interchange-bench/Cargo.toml"
cp fixtures/effect-interchange/v1/ACCEPTED.sha256 \
    "$template/fixtures/effect-interchange/v1/ACCEPTED.sha256"
cp Cargo.lock "$template/Cargo.lock"
for name in nonbenchmark.seal.json miso_engine_effect_interchange_bench \
    benchmark-preflight.seal.json benchmark.raw.jsonl benchmark.stderr.log \
    benchmark.disposition.json; do
    cp "target/issue081/$name" "$template/target/issue081/$name"
done

cat >"$template/bin/git" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "${1:-}" in
    status)
        [[ "${MISO_TEST_GIT_DIRTY:-0}" != 1 ]] || printf '%s\n' ' M synthetic-dirty'
        exit 0
        ;;
    branch) printf '%s\n' codex/batch-benchmark-108 ;;
    rev-parse)
        if [[ "${2:-}" == --verify ]]; then
            printf '%s\n' 1111111111111111111111111111111111111111
        else
            printf '%s\n' 2222222222222222222222222222222222222222
        fi
        ;;
    *) exit 91 ;;
esac
EOF
cat >"$template/bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
[[ "${MISO_TEST_CARGO_FAIL:-0}" != 1 ]] || exit 73
mkdir -p "$CARGO_TARGET_DIR/release"
cp "$MISO_TEST_FAKE_BENCH" "$CARGO_TARGET_DIR/release/miso_engine_effect_interchange_bench"
chmod 755 "$CARGO_TARGET_DIR/release/miso_engine_effect_interchange_bench"
EOF
cat >"$template/fake-benchmark.py" <<'PY'
#!/usr/bin/env python3
import json, os, pathlib, sys
if len(sys.argv) != 1:
    raise SystemExit(2)
log = pathlib.Path(os.environ["MISO_TEST_LAUNCH_LOG"])
with log.open("a", encoding="utf-8") as stream:
    stream.write("launch\n")
mode = os.environ.get("MISO_TEST_BENCH_MODE", "success")
def phase(value):
    print("MISO_INTERCHANGE_BENCH_PHASE " + value, file=sys.stderr, flush=True)
phase("workload_started")
if mode in ("nonzero", "panic_before_warmup"):
    print('{"partial":true}')
    print("synthetic failure before warmup", file=sys.stderr)
    raise SystemExit(71)
phase("warmup_complete")
if mode == "panic_after_warmup":
    print("synthetic failure after warmup", file=sys.stderr)
    raise SystemExit(72)
phase("timed_started")
phase("round_1_complete")
if mode == "panic_after_round1":
    print("synthetic failure after round 1", file=sys.stderr)
    raise SystemExit(73)
phase("round_2_complete")
workloads = (
    "descriptor_verify_identity_a", "package_verify_cid_select_a",
    "state_verify_reencode_current", "migration_two_step_bank_restore",
)
outputs = {
    "descriptor_verify_identity_a": "865a0a5a01ba157bea7f3279ad68cc17db0296655998a9b5307cf759c38656f1",
    "package_verify_cid_select_a": "02e944154ccdc0315b96a7f493a11f6c60f70993750fb26ed766bc3273685d0f",
    "state_verify_reencode_current": "b38a9abad3da50b0c38bd02b9de19b641e79f9a8f48099fbb67d1ec3d481cf48",
    "migration_two_step_bank_restore": "5f23e630182137426fdfe01b74861bdff779b6738bfae8f670359ad0e9ea2777",
}
missing = ["background_load", "cpu_model", "governor", "kernel", "logical_cores", "os", "physical_cores", "power_mode"]
records = []
for round_ in (1, 2):
    for index, workload in enumerate(workloads):
        records.append({
            "schema_version": 1, "issue": 108, "workload_id": workload, "round": round_,
            "observation_count": 256, "unit": "ns_per_operation",
            "candidate_commit": os.environ["MISO_INTERCHANGE_CANDIDATE_COMMIT"],
            "candidate_tree": os.environ["MISO_INTERCHANGE_CANDIDATE_TREE"],
            "binary_sha256": os.environ["MISO_INTERCHANGE_BINARY_SHA256"],
            "tool_manifest_sha256": os.environ["MISO_INTERCHANGE_TOOL_MANIFEST_SHA256"],
            "tool_source_sha256": os.environ["MISO_INTERCHANGE_TOOL_SOURCE_SHA256"],
            "fixture_manifest_sha256": os.environ["MISO_INTERCHANGE_FIXTURE_MANIFEST_SHA256"],
            "output_sha256": outputs[workload],
            "rust_version": os.environ["MISO_INTERCHANGE_RUST_VERSION"],
            "llvm_version": os.environ["MISO_INTERCHANGE_LLVM_VERSION"],
            "target_triple": os.environ["MISO_INTERCHANGE_TARGET_TRIPLE"],
            "profile": os.environ["MISO_INTERCHANGE_PROFILE"],
            "cpu_model": os.environ.get("CPU_MODEL", ""),
            "logical_cores": os.environ.get("LOGICAL_CORES", ""),
            "physical_cores": os.environ.get("PHYSICAL_CORES", ""),
            "os": os.environ.get("OS", ""), "kernel": os.environ.get("KERNEL", ""),
            "power_mode": os.environ.get("POWER_MODE", ""),
            "governor": os.environ.get("GOVERNOR", ""),
            "background_load": os.environ.get("BACKGROUND_LOAD", ""),
            "timer_method": "std::time::Instant", "percentile_method": "nearest-rank",
            "total_ns": 2560, "min_ns_per_operation": 10, "p50_ns_per_operation": 10,
            "p95_ns_per_operation": 10, "p99_ns_per_operation": 10,
            "p99_9_ns_per_operation": 10, "max_ns_per_operation": 10,
            "descriptive_only": True, "metadata_incomplete": False, "missing_metadata": [],
        })
for record in records:
    record["missing_metadata"] = sorted(key for key in missing if not record[key])
    record["metadata_incomplete"] = bool(record["missing_metadata"])
if mode == "truncated": records.pop()
if mode == "extra": records.append(dict(records[-1], round=1))
if mode == "duplicate": records[-1] = records[0]
if mode == "wrong_digest": records[-1]["output_sha256"] = "f" * 64
if mode == "all_wrong":
    for record in records:
        record["output_sha256"] = "f" * 64
if mode == "malformed":
    print("{")
    raise SystemExit(0)
for record in records:
    print(json.dumps(record, sort_keys=True, separators=(",", ":")))
if mode == "finalization_fail":
    print("synthetic stdout finalization failure", file=sys.stderr)
    raise SystemExit(74)
PY
chmod 755 "$template/bin/git" "$template/bin/cargo" "$template/fake-benchmark.py"
cat >"$template/fake-loader-fail" <<'EOF'
#!/no/such/issue108/interpreter
EOF
chmod 755 "$template/fake-loader-fail"

case_number=0
new_case() {
    case_number=$((case_number + 1))
    case_root="$scratch/case-$case_number-$1"
    cp -a "$template" "$case_root"
    launch_log="$case_root/launch.log"
    local accepted_sha lock_sha tool_manifest_sha source_sha validator_sha checker_sha
    local mutation_sha lifecycle_sha preflight_sha runner_sha
    accepted_sha="$(sha256sum "$case_root/fixtures/effect-interchange/v1/ACCEPTED.sha256" | awk '{print $1}')"
    lock_sha="$(sha256sum "$case_root/Cargo.lock" | awk '{print $1}')"
    tool_manifest_sha="$(sha256sum "$case_root/tools/miso-engine-effect-interchange-bench/Cargo.toml" | awk '{print $1}')"
    source_sha="$(sha256sum "$case_root/tools/miso-engine-effect-interchange-bench/src/main.rs" | awk '{print $1}')"
    validator_sha="$(sha256sum "$case_root/scripts/effect-interchange-benchmark-108-validator.py" | awk '{print $1}')"
    checker_sha="$(sha256sum "$case_root/scripts/check-effect-interchange-benchmark-108.sh" | awk '{print $1}')"
    mutation_sha="$(sha256sum "$case_root/scripts/test-effect-interchange-benchmark-108-policy.sh" | awk '{print $1}')"
    lifecycle_sha="$(sha256sum "$case_root/scripts/test-effect-interchange-benchmark-108.sh" | awk '{print $1}')"
    preflight_sha="$(sha256sum "$case_root/scripts/preflight-effect-interchange-benchmark-108.sh" | awk '{print $1}')"
    runner_sha="$(sha256sum "$case_root/scripts/run-effect-interchange-benchmark-108.sh" | awk '{print $1}')"
    printf '{"schema_version":1,"issue":108,"kind":"effect_interchange_benchmark_repair","branch":"codex/batch-benchmark-108","candidate_commit":"1111111111111111111111111111111111111111","candidate_tree":"2222222222222222222222222222222222222222","accepted_manifest_sha256":"%s","cargo_lock_sha256":"%s","tool_manifest_sha256":"%s","tool_source_sha256":"%s","validator_sha256":"%s","checker_sha256":"%s","mutation_sha256":"%s","lifecycle_sha256":"%s","preflight_sha256":"%s","runner_sha256":"%s","output_sha256":{"descriptor_verify_identity_a":"865a0a5a01ba157bea7f3279ad68cc17db0296655998a9b5307cf759c38656f1","package_verify_cid_select_a":"02e944154ccdc0315b96a7f493a11f6c60f70993750fb26ed766bc3273685d0f","state_verify_reencode_current":"b38a9abad3da50b0c38bd02b9de19b641e79f9a8f48099fbb67d1ec3d481cf48","migration_two_step_bank_restore":"5f23e630182137426fdfe01b74861bdff779b6738bfae8f670359ad0e9ea2777"},"issue081_artifacts":{"nonbenchmark.seal.json":"6d08e2089e806dc366f5c1171398c241f8dfdc520f97808c4e2f6c7f6b83363c","miso_engine_effect_interchange_bench":"fad8e39ecd9efa6908b51e7e98c25984f9d97f88b32971581c9a880228758b4c","benchmark-preflight.seal.json":"da3c537c16d55b1e71b8aa9f8e4d011796b243e4c6c7969020097098a75035a3","benchmark.raw.jsonl":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855","benchmark.stderr.log":"442f071fb23e57a9cb4616c6df7683bee669d8114eacce43b16af812e86d1a93","benchmark.disposition.json":"8c833293bb3e9f2e981e0be1d379819786d92706627b3fa3fbc64e93b188a5de","benchmark.accepted.jsonl":null,"benchmark.prelaunch.disposition.json":null},"focused_regressions":1,"benchmark_preflight_invocations":0,"benchmark_runner_invocations":0,"benchmark_workload_invocations":0,"timed_benchmark_invocations":0}\n' \
        "$accepted_sha" "$lock_sha" "$tool_manifest_sha" "$source_sha" "$validator_sha" \
        "$checker_sha" "$mutation_sha" "$lifecycle_sha" "$preflight_sha" "$runner_sha" \
        >"$case_root/target/issue108/repair.seal.json"
}
run_preflight() {
    local benchmark=${1:-"$case_root/fake-benchmark.py"}
    MISO_INTERCHANGE_HERMETIC_CHILD=1 MISO_TEST_FAKE_BENCH="$benchmark" \
        PATH="$case_root/bin:$PATH" bash "$case_root/scripts/preflight-effect-interchange-benchmark-108.sh"
}
run_runner() {
    local mode=$1
    MISO_TEST_BENCH_MODE="$mode" MISO_TEST_LAUNCH_LOG="$launch_log" \
        PATH="$case_root/bin:$PATH" bash "$case_root/scripts/run-effect-interchange-benchmark-108.sh"
}
assert_disposition() {
    python3 -I -B - "$1" "$2" "$3" "$4" "$5" "$6" "$7" "$8" <<'PY'
import hashlib, json, pathlib, sys
value = json.loads(pathlib.Path(sys.argv[1]).read_text(encoding="utf-8"))
assert value["kind"] == sys.argv[2]
assert value["status"] == sys.argv[3]
assert value["reason"] == sys.argv[4]
assert value["benchmark_runner_invocations"] == 1
assert value["benchmark_workload_invocations"] == int(sys.argv[5])
assert value["timed_benchmark_invocations"] == int(sys.argv[6])
assert value["warmup_passes_completed"] == int(sys.argv[7])
assert value["measured_rounds_completed"] == int(sys.argv[8])
for name in ("raw", "accepted", "stderr"):
    path = pathlib.Path(sys.argv[1]).with_name(f"benchmark.{name}.jsonl" if name != "stderr" else "benchmark.stderr.log")
    if name == "accepted":
        path = pathlib.Path(sys.argv[1]).with_name("benchmark.accepted.jsonl")
    if path.is_file():
        content = path.read_bytes()
        assert value[f"{name}_sha256"] == hashlib.sha256(content).hexdigest()
        assert value[f"{name}_bytes"] == len(content)
    else:
        assert value[f"{name}_sha256"] is None
        assert value[f"{name}_bytes"] == 0
PY
}
assert_final() {
    assert_disposition "$case_root/target/issue108/benchmark.disposition.json" \
        effect_interchange_benchmark_disposition "$@"
}
assert_prelaunch() {
    assert_disposition "$case_root/target/issue108/benchmark.prelaunch.disposition.json" \
        effect_interchange_benchmark_prelaunch_disposition "$@"
}

new_case success
run_preflight >/dev/null
[[ ! -e "$launch_log" ]]
run_runner success >/dev/null
[[ "$(wc -l <"$launch_log")" == 1 ]]
[[ "$(wc -l <"$case_root/target/issue108/benchmark.raw.jsonl")" == 8 ]]
[[ "$(wc -l <"$case_root/target/issue108/benchmark.accepted.jsonl")" == 8 ]]
assert_final PASS complete 1 1 1 2
python3 -I -B - "$case_root/target/issue108/benchmark.raw.jsonl" \
    "$case_root/target/issue108/benchmark.accepted.jsonl" \
    "$case_root/target/issue108/benchmark.disposition.json" <<'PY'
import hashlib, json, os, pathlib, sys
raw, accepted, disposition = map(pathlib.Path, sys.argv[1:])
assert os.stat(raw).st_ino != os.stat(accepted).st_ino
accepted_before = accepted.read_bytes()
identity = json.loads(disposition.read_text(encoding="utf-8"))["accepted_sha256"]
assert hashlib.sha256(accepted_before).hexdigest() == identity
with raw.open("ab") as stream:
    stream.write(b'{"post_publication_raw_mutation":true}\n')
assert accepted.read_bytes() == accepted_before
assert hashlib.sha256(accepted.read_bytes()).hexdigest() == identity
PY
python3 -I -B - "$case_root/target/issue108" <<'PY'
import os, pathlib, stat, sys
root = pathlib.Path(sys.argv[1])
for name in (
    "repair.seal.json", "miso_engine_effect_interchange_bench",
    "benchmark-preflight.seal.json", "benchmark.raw.jsonl", "benchmark.stderr.log",
    "benchmark.accepted.jsonl", "benchmark.disposition.json",
):
    path = root / name
    value = os.lstat(path)
    assert stat.S_ISREG(value.st_mode) and value.st_nlink == 1
PY
if run_runner success >/dev/null 2>&1; then
    printf 'effect interchange benchmark lifecycle: no-clobber runner passed twice\n' >&2
    exit 1
fi
[[ "$(wc -l <"$launch_log")" == 1 ]]

new_case optional-metadata-tools-absent
run_preflight >/dev/null
runner_bin="$case_root/runner-bin"
mkdir "$runner_bin"
for tool in awk bash ln mkdir mktemp python3 rm rustc sha256sum stat wc; do
    ln -s "$(command -v "$tool")" "$runner_bin/$tool"
done
ln -s "$case_root/bin/git" "$runner_bin/git"
MISO_TEST_BENCH_MODE=success MISO_TEST_LAUNCH_LOG="$launch_log" \
    PATH="$runner_bin" /bin/bash "$case_root/scripts/run-effect-interchange-benchmark-108.sh" >/dev/null
[[ "$(wc -l <"$launch_log")" == 1 ]]
assert_final PASS complete 1 1 1 2

for kind in regular symlink hardlink; do
    new_case "overwrite-$kind"
    run_preflight >/dev/null
    case "$kind" in
        regular) printf protected >"$case_root/target/issue108/benchmark.raw.jsonl" ;;
        symlink) ln -s protected "$case_root/target/issue108/benchmark.raw.jsonl" ;;
        hardlink)
            printf protected >"$case_root/protected"
            ln "$case_root/protected" "$case_root/target/issue108/benchmark.raw.jsonl"
            ;;
    esac
    if run_runner success >/dev/null 2>&1; then
        printf 'effect interchange benchmark lifecycle: %s output overwrite passed\n' "$kind" >&2
        exit 1
    fi
    [[ ! -e "$launch_log" ]]
    assert_prelaunch FAIL existing_output 0 0 0 0
done

new_case cargo-failure
if MISO_TEST_CARGO_FAIL=1 run_preflight >/dev/null 2>&1; then
    printf 'effect interchange benchmark lifecycle: preflight swallowed cargo failure\n' >&2
    exit 1
fi
[[ ! -e "$launch_log" ]]
[[ ! -e "$case_root/target/issue108/benchmark-preflight.seal.json" ]]

new_case dirty-preflight
if MISO_TEST_GIT_DIRTY=1 run_preflight >/dev/null 2>&1; then
    printf 'effect interchange benchmark lifecycle: dirty preflight candidate passed\n' >&2
    exit 1
fi
[[ ! -e "$launch_log" ]]

new_case inherited-evidence-mutated
printf 'mutation\n' >>"$case_root/target/issue081/benchmark.stderr.log"
if run_preflight >/dev/null 2>&1; then
    printf 'effect interchange benchmark lifecycle: mutated inherited evidence passed\n' >&2
    exit 1
fi
[[ ! -e "$launch_log" ]]

new_case inherited-absence-violated
printf 'forbidden\n' >"$case_root/target/issue081/benchmark.accepted.jsonl"
if run_preflight >/dev/null 2>&1; then
    printf 'effect interchange benchmark lifecycle: inherited absence violation passed\n' >&2
    exit 1
fi
[[ ! -e "$launch_log" ]]

for mutation in candidate_commit candidate_tree accepted_manifest_sha256 cargo_lock_sha256 \
    tool_manifest_sha256 tool_source_sha256 validator_sha256 checker_sha256 mutation_sha256 \
    lifecycle_sha256 preflight_sha256 runner_sha256 output_sha256 issue081_artifacts \
    focused_regressions benchmark_preflight_invocations; do
    new_case "repair-$mutation"
    python3 -I -B - "$case_root/target/issue108/repair.seal.json" "$mutation" <<'PY'
import json, pathlib, sys
path = pathlib.Path(sys.argv[1])
value = json.loads(path.read_text(encoding="utf-8"))
key = sys.argv[2]
if key == "output_sha256":
    value[key]["migration_two_step_bank_restore"] = "f" * 64
elif key == "issue081_artifacts":
    value[key]["benchmark.stderr.log"] = "f" * 64
elif key in ("focused_regressions", "benchmark_preflight_invocations"):
    value[key] = 2
else:
    value[key] = "f" * len(value[key])
path.write_text(json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n", encoding="utf-8")
PY
    if run_preflight >/dev/null 2>&1; then
        printf 'effect interchange benchmark lifecycle: repair mutation passed: %s\n' "$mutation" >&2
        exit 1
    fi
    [[ ! -e "$launch_log" ]]
done

for kind in regular symlink hardlink; do
    new_case "preflight-overwrite-$kind"
    case "$kind" in
        regular) printf protected >"$case_root/target/issue108/miso_engine_effect_interchange_bench" ;;
        symlink) ln -s protected "$case_root/target/issue108/miso_engine_effect_interchange_bench" ;;
        hardlink)
            printf protected >"$case_root/protected"
            ln "$case_root/protected" "$case_root/target/issue108/miso_engine_effect_interchange_bench"
            ;;
    esac
    if run_preflight >/dev/null 2>&1; then
        printf 'effect interchange benchmark lifecycle: preflight %s overwrite passed\n' "$kind" >&2
        exit 1
    fi
    [[ ! -e "$launch_log" ]]
done

new_case missing-seal
rm "$case_root/target/issue108/repair.seal.json"
if run_preflight >/dev/null 2>&1; then
    printf 'effect interchange benchmark lifecycle: missing dependency passed\n' >&2
    exit 1
fi
[[ ! -e "$launch_log" ]]

new_case missing-tool
rm "$case_root/bin/cargo"
ln -s "$(command -v awk)" "$case_root/bin/awk"
ln -s "$(command -v bash)" "$case_root/bin/bash"
if PATH="$case_root/bin" /bin/bash \
    "$case_root/scripts/preflight-effect-interchange-benchmark-108.sh" >/dev/null 2>&1; then
    printf 'effect interchange benchmark lifecycle: missing tool passed\n' >&2
    exit 1
fi
[[ ! -e "$launch_log" ]]

new_case prelaunch-no-retry
run_preflight >/dev/null
if MISO_TEST_GIT_DIRTY=1 run_runner success >/dev/null 2>&1; then
    printf 'effect interchange benchmark lifecycle: dirty runner candidate passed\n' >&2
    exit 1
fi
[[ ! -e "$launch_log" ]]
assert_prelaunch FAIL dirty_candidate 0 0 0 0
prelaunch_sha="$(sha256sum "$case_root/target/issue108/benchmark.prelaunch.disposition.json" | awk '{print $1}')"
if run_runner success >/dev/null 2>&1; then
    printf 'effect interchange benchmark lifecycle: prelaunch retry passed\n' >&2
    exit 1
fi
[[ ! -e "$launch_log" ]]
[[ "$(sha256sum "$case_root/target/issue108/benchmark.prelaunch.disposition.json" | awk '{print $1}')" == "$prelaunch_sha" ]]
[[ ! -e "$case_root/target/issue108/benchmark.disposition.json" ]]

new_case loader-failure
run_preflight "$case_root/fake-loader-fail" >/dev/null
if run_runner success >/dev/null 2>&1; then
    printf 'effect interchange benchmark lifecycle: loader failure passed\n' >&2
    exit 1
fi
[[ ! -e "$launch_log" ]]
assert_final FAIL workload_failed 0 0 0 0
[[ -s "$case_root/target/issue108/benchmark.stderr.log" ]]

new_case workload-failure
run_preflight >/dev/null
if run_runner nonzero >/dev/null 2>&1; then
    printf 'effect interchange benchmark lifecycle: workload failure passed\n' >&2
    exit 1
fi
assert_final FAIL workload_failed 1 0 0 0
grep -Fqx '{"partial":true}' "$case_root/target/issue108/benchmark.raw.jsonl"
grep -Fq 'synthetic failure before warmup' "$case_root/target/issue108/benchmark.stderr.log"
[[ ! -e "$case_root/target/issue108/benchmark.accepted.jsonl" ]]

for row in \
    'panic_after_warmup 1 0 1 0' \
    'panic_after_round1 1 1 1 1' \
    'finalization_fail 1 1 1 2'; do
    set -- $row
    mode=$1
    new_case "$mode"
    run_preflight >/dev/null
    if run_runner "$mode" >/dev/null 2>&1; then
        printf 'effect interchange benchmark lifecycle: phase failure passed: %s\n' "$mode" >&2
        exit 1
    fi
    assert_final FAIL workload_failed "$2" "$3" "$4" "$5"
    [[ -s "$case_root/target/issue108/benchmark.stderr.log" ]]
    [[ ! -e "$case_root/target/issue108/benchmark.accepted.jsonl" ]]
done

for mode in malformed truncated extra duplicate wrong_digest all_wrong; do
    new_case "$mode"
    run_preflight >/dev/null
    if run_runner "$mode" >/dev/null 2>&1; then
        printf 'effect interchange benchmark lifecycle: validator accepted %s\n' "$mode" >&2
        exit 1
    fi
    assert_final FAIL validation_failed 1 1 1 2
    [[ -s "$case_root/target/issue108/benchmark.raw.jsonl" ]]
    [[ ! -e "$case_root/target/issue108/benchmark.accepted.jsonl" ]]
done

for script in scripts/preflight-effect-interchange-benchmark-108.sh scripts/run-effect-interchange-benchmark-108.sh; do
    if bash "$script" extra >/dev/null 2>&1; then
        printf 'effect interchange benchmark lifecycle: argument accepted by %s\n' "$script" >&2
        exit 1
    fi
done
printf 'effect interchange benchmark hermetic lifecycle: ok fake_launches_only=1-per-launched-case\n'
