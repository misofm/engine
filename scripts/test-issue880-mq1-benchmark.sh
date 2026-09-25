#!/usr/bin/env bash
# Scratch-only checks for Issue-880 MQ-1 argument, schema, and persistence preflight.
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
# shellcheck source=scripts/issue880-mq1-benchmark-lib.sh
source scripts/issue880-mq1-benchmark-lib.sh

usage() {
    printf 'usage: %s [--effect-source-commit COMMIT --effect-revision e1|mb2-fast-db]\n' "$0" >&2
}

effect_source_commit=
effect_revision=
while (($#)); do
    case "$1" in
        --effect-source-commit)
            [[ -z "$effect_source_commit" && $# -ge 2 && -n "$2" ]] || { usage; exit 2; }
            effect_source_commit=$2
            shift 2
            ;;
        --effect-revision)
            [[ -z "$effect_revision" && $# -ge 2 && -n "$2" ]] || { usage; exit 2; }
            effect_revision=$2
            shift 2
            ;;
        *)
            usage
            exit 2
            ;;
    esac
done
if [[ -n "$effect_source_commit" || -n "$effect_revision" ]]; then
    [[ "$effect_source_commit" =~ ^[0-9a-f]{40}$ &&
        ( "$effect_revision" == e1 || "$effect_revision" == mb2-fast-db ) ]] || {
        usage
        exit 2
    }
fi
effect_selection_args=()
if [[ -n "$effect_source_commit" ]]; then
    effect_selection_args=(--effect-source-commit "$effect_source_commit" --effect-revision "$effect_revision")
fi

scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT
runner=scripts/run-issue880-mq1-benchmark.sh
fixture=scripts/fixtures/issue880-mq1-record.json
validator=scripts/issue880-mq1-record-validator.jq
raw_fixture=artifacts/issue880/mq1-baseline/mq1-baseline.raw.log
failure_fixture=artifacts/issue880/mq1-baseline/mq1-baseline.failure.json
runner_failure_fixture=artifacts/issue880/mq1-baseline/mq1-baseline.runner-failure.json
sha256sum "$raw_fixture" "$failure_fixture" "$runner_failure_fixture" >"$scratch/original-artifacts.sha256"

if output=$(bash "$runner" --bad --output "$scratch/bad.json" 2>&1); then
    printf 'MQ-1 runner accepted an unknown option\n' >&2
    exit 1
else
    status=$?
fi
[[ "$status" == 2 && "$output" == *usage:* ]]
baseline_effect_commit=6f662fee7b47a5eb38b67e0ddc6d007edd438cfa
if output=$(bash "$runner" --preflight --effect-source-commit "$baseline_effect_commit" \
    --effect-revision unknown --output "$scratch/invalid-revision.json" 2>&1); then
    printf 'MQ-1 runner accepted an unknown effect revision\n' >&2
    exit 1
else
    status=$?
fi
[[ "$status" == 2 && "$output" == *usage:* ]]

fixture_repo="$scratch/mb2-source"
mkdir -p "$fixture_repo/crates/math/src" "$fixture_repo/crates/transient-shaper/src"
git -C "$fixture_repo" init -q
git -C "$fixture_repo" config user.name 'MQ-1 preflight fixture'
git -C "$fixture_repo" config user.email 'mq1-fixture@example.invalid'
printf 'pub fn lane_math() {}\n' >"$fixture_repo/crates/math/src/lane_math.rs"
printf 'pub fn fast_db() {}\n' >"$fixture_repo/crates/math/src/fast_db.rs"
printf 'pub fn exact_shaper() {}\n' >"$fixture_repo/crates/transient-shaper/src/lib.rs"
git -C "$fixture_repo" add crates
git -C "$fixture_repo" commit -qm 'fixture E1 source'
fixture_baseline=$(git -C "$fixture_repo" rev-parse HEAD)
cat >"$fixture_repo/crates/transient-shaper/src/lib.rs" <<'EOF'
pub fn frame() {
    // FAST-DB-CROSSING X7
    fast_level_db(ratio);
    // FAST-DB-CROSSING X8
    fast_gain_from_db(shape);
}
EOF
git -C "$fixture_repo" add crates/transient-shaper/src/lib.rs
git -C "$fixture_repo" commit -qm 'fixture MB-2 fast-tier source'
fixture_mb2=$(git -C "$fixture_repo" rev-parse HEAD)
git -C "$fixture_repo" commit --allow-empty -qm 'fixture later metadata-only commit'
validate_effect_source_selection "$fixture_repo" "$fixture_mb2" mb2-fast-db "$fixture_baseline"
jq --arg commit "$fixture_mb2" \
    '.engine_effect_commit = $commit | .engine_effect_revision = "MB-2 (R2 fast dB tier)"' \
    "$fixture" >"$scratch/mb2-record.json"
jq -e -L scripts -f "$validator" "$scratch/mb2-record.json" >/dev/null
if jq '.engine_effect_revision = "unapproved"' "$scratch/mb2-record.json" | \
    jq -e -L scripts -f "$validator" >/dev/null; then
    printf 'MQ-1 record validator accepted an unknown effect revision\n' >&2
    exit 1
fi
if validate_effect_source_selection "$fixture_repo" "$fixture_baseline" mb2-fast-db "$fixture_baseline"; then
    printf 'MQ-1 source validator accepted E1 source as MB-2\n' >&2
    exit 1
fi
printf '// uncommitted source drift\n' >>"$fixture_repo/crates/transient-shaper/src/lib.rs"
if validate_effect_source_selection "$fixture_repo" "$fixture_mb2" mb2-fast-db "$fixture_baseline"; then
    printf 'MQ-1 source validator accepted source drift from the cited commit\n' >&2
    exit 1
fi
git -C "$fixture_repo" checkout -- crates/transient-shaper/src/lib.rs
git -C "$fixture_repo" checkout --orphan unrelated-source >/dev/null 2>&1
git -C "$fixture_repo" add crates
git -C "$fixture_repo" commit --allow-empty -qm 'fixture unrelated source root'
if validate_effect_source_selection "$fixture_repo" "$fixture_mb2" mb2-fast-db "$fixture_baseline"; then
    printf 'MQ-1 source validator accepted a non-ancestor source commit\n' >&2
    exit 1
fi

extract_mq1_result "$raw_fixture" "$scratch/prefixed-result.json"
cmp -s "$scratch/prefixed-result.json" <(sed -n 's/.*MQ1_RESULT //p' "$raw_fixture")
printf 'MQ1_RESULT {"fixture_sha256":"%064d","bank_ns_per_lane_sample":[1.0,2.0],"scalar_ns_per_lane_sample":[3.0,4.0]}\n' 0 \
    >"$scratch/unprefixed.log"
extract_mq1_result "$scratch/unprefixed.log" "$scratch/unprefixed-result.json"
jq -e '.bank_ns_per_lane_sample == [1, 2] and .scalar_ns_per_lane_sample == [3, 4]' \
    "$scratch/unprefixed-result.json" >/dev/null

: >"$scratch/empty.log"
if extract_mq1_result "$scratch/empty.log" "$scratch/rejected-result.json"; then
    printf 'MQ-1 result extractor accepted zero records\n' >&2
    exit 1
fi
printf 'MQ1_RESULT {"fixture_sha256":"%064d","bank_ns_per_lane_sample":[1,2],"scalar_ns_per_lane_sample":[3,4]}\nMQ1_RESULT {"fixture_sha256":"%064d","bank_ns_per_lane_sample":[1,2],"scalar_ns_per_lane_sample":[3,4]}\n' 0 0 \
    >"$scratch/duplicate.log"
if extract_mq1_result "$scratch/duplicate.log" "$scratch/rejected-result.json"; then
    printf 'MQ-1 result extractor accepted duplicate records\n' >&2
    exit 1
fi
printf 'test mq1_transient_shaper_ns_per_lane_sample ... MQ1_RESULT {broken json}\n' >"$scratch/malformed.log"
if extract_mq1_result "$scratch/malformed.log" "$scratch/rejected-result.json" 2>/dev/null; then
    printf 'MQ-1 result extractor accepted malformed JSON\n' >&2
    exit 1
fi
printf 'MQ1_RESULT {"fixture_sha256":"%064d","bank_ns_per_lane_sample":[NaN,2],"scalar_ns_per_lane_sample":[3,4]}\n' 0 \
    >"$scratch/nonfinite.log"
if extract_mq1_result "$scratch/nonfinite.log" "$scratch/rejected-result.json"; then
    printf 'MQ-1 result extractor accepted a nonfinite measurement\n' >&2
    exit 1
fi

jq -e -L scripts -f "$validator" "$fixture" >/dev/null
if jq '.measured_rounds_per_arm = 3' "$fixture" | jq -e -L scripts -f "$validator" >/dev/null; then
    printf 'MQ-1 record validator accepted a wrong round count\n' >&2
    exit 1
fi

bash "$runner" --recover-preserved \
    --raw-input "$raw_fixture" \
    --failure-input "$failure_fixture" \
    --output "$scratch/recovered.json" >"$scratch/recovery.out"
jq -e -L scripts -f "$validator" "$scratch/recovered.json" >/dev/null
jq -e '
    .status == "measured" and
    .candidate_commit == "2a8977f5f0fb9b3384e2d71632f21c7f9896dce4" and
    .engine_effect_commit == "6f662fee7b47a5eb38b67e0ddc6d007edd438cfa" and
    .bank_ns_per_lane_sample == [6.281854, 6.281717] and
    .scalar_ns_per_lane_sample == [40.841292, 40.803620] and
    .recovery.workload_invocations_during_recovery == 0 and
    .recovery.source_failure_record_sha256 == "725b7aaff22970a96771023aa4fa4b86ac5685b53d1551bb59e3a164dd67a3cf" and
    .recovery.source_raw_log_sha256 == "f77b1db698c8248d82cf73a433032c8cb1482bff98886a7eeca9c5c3e9adb565"
' "$scratch/recovered.json" >/dev/null
sha256sum --check "$scratch/original-artifacts.sha256" >/dev/null

printf 'persist probe\n' >"$scratch/source"
persist_no_clobber "$scratch/source" "$scratch/persisted"
cmp -s "$scratch/source" "$scratch/persisted"
printf 'keep existing bytes\n' >"$scratch/occupied"
if persist_no_clobber "$scratch/source" "$scratch/occupied" 2>/dev/null; then
    printf 'MQ-1 persistence helper overwrote an existing record\n' >&2
    exit 1
fi
[[ "$(<"$scratch/occupied")" == 'keep existing bytes' ]]

if run_and_capture "$scratch/exit-status" bash -c 'printf captured; exit 37'; then
    printf 'MQ-1 capture helper swallowed a command failure\n' >&2
    exit 1
else
    status=$?
fi
[[ "$status" == 37 && "$(<"$scratch/exit-status")" == captured ]]

bash "$runner" --preflight "${effect_selection_args[@]}" --output "$scratch/record.json" >"$scratch/preflight.out"
rg -q 'workload launches 0' "$scratch/preflight.out"
[[ ! -e "$scratch/record.json" && ! -e "$scratch/record.json.raw.log" ]]

for suffix in '' '.raw.log' '.failure.json'; do
    target="$scratch/no-clobber.json$suffix"
    printf 'existing\n' >"$target"
    if bash "$runner" --preflight "${effect_selection_args[@]}" --output "$scratch/no-clobber.json" >/dev/null 2>&1; then
        printf 'MQ-1 runner accepted existing output path %s\n' "$target" >&2
        exit 1
    else
        status=$?
    fi
    [[ "$status" == 1 && "$(<"$target")" == existing ]]
    rm -- "$target"
done

printf 'Issue-880 MQ-1 parser and promotion self-test: PASS (timed workload launches 0)\n'
