#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
revision=dc581f3470b40678301d9504f1be4b1fd6be7173
source_records="$root/artifacts/mono3/console-benchmark.accepted.jsonl"
checked_records="$root/artifacts/issue368-floor-recount/historical-mono3-repriced.jsonl"
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
for name in console-benchmark-record-lib.jq console-benchmark-record-validator.jq console-benchmark-validator.jq; do
    git -C "$root" show "$revision:scripts/$name" >"$work/$name"
done
mkdir "$work/repriced"
cp "$work/console-benchmark-record-validator.jq" "$work/console-benchmark-validator.jq" "$work/repriced/"
sed -e 's/def compressor_lane_ops: 94;/def compressor_lane_ops: 81.5;/' \
    -e 's/def limiter_lane_ops: 138;/def limiter_lane_ops: 129.5;/' \
    "$work/console-benchmark-record-lib.jq" >"$work/repriced/console-benchmark-record-lib.jq"
diff -u "$work/console-benchmark-record-lib.jq" "$work/repriced/console-benchmark-record-lib.jq" >"$work/library.diff" || status=$?
[[ ${status:-0} == 1 && $(grep -c '^[+-]def .*_lane_ops:' "$work/library.diff") == 4 ]] || {
    printf 'historical library changed beyond two inventory constants\n' >&2; exit 1; }
validate_records() {
    local directory=$1 records=$2
    while IFS= read -r record; do
        if printf '%s\n' "$record" |
            jq -e -L "$directory" -f "$directory/console-benchmark-record-validator.jq" >/dev/null; then
            :
        else
            return $?
        fi
    done <"$records"
}
validate_aggregate() { jq -s -e -L "$1" -f "$1/console-benchmark-validator.jq" "$2" >/dev/null; }
expect_aggregate_reject() {
    local label=$1 directory=$2 records=$3 result
    if validate_aggregate "$directory" "$records"; then
        printf '%s unexpectedly validated\n' "$label" >&2; return 1
    else result=$?; fi
    if [[ $result != 1 ]]; then printf '%s validator execution failed: %s\n' "$label" "$result" >&2; return 2; fi
}
expect_records_reject() {
    local label=$1 directory=$2 records=$3 result
    if validate_records "$directory" "$records"; then
        printf '%s records unexpectedly validated\n' "$label" >&2; return 1
    else result=$?; fi
    if [[ $result != 1 ]]; then printf '%s record validator execution failed: %s\n' "$label" "$result" >&2; return 2; fi
}
validate_records "$work" "$source_records"
validate_aggregate "$work" "$source_records"
jq -cs -L "$work/repriced" 'include "console-benchmark-record-lib"; . as $all |
 ([ $all[]|select(.record=="console_session")|{key:(.workload_kind+":"+(.round|tostring)),value:.}]|from_entries) as $by |
 [ $all[]|if .record=="console_session" then (floor_pins[.workload_kind]) as $pin |
 .floor_cycles_per_lane_sample=(if $pin[0]==null then null else $pin[0]*$pin[1]/lane_ops_per_cycle end) |
 .percent_of_floor=(if .floor_cycles_per_lane_sample==null then null else 100*.floor_cycles_per_lane_sample/.cycles_per_lane_sample end) |
 .isolated_percent_of_floor=(if .floor_control_row=="none" then null else
 ($by[.floor_control_row+":"+(.round|tostring)]) as $c | (floor_pins[$c.workload_kind]) as $cp |
 100*(.floor_cycles_per_lane_sample-$cp[0]*$cp[1]/lane_ops_per_cycle)/.isolated_cycles_per_lane_sample end)
 else . end]|.[]' "$source_records" >"$work/repriced.jsonl"
cmp "$work/repriced.jsonl" "$checked_records"
jq -e -n --slurpfile old "$source_records" --slurpfile new "$work/repriced.jsonl" '
 def strip: del(.floor_cycles_per_lane_sample,.percent_of_floor,.isolated_percent_of_floor);
 [$old[]|strip] == [$new[]|strip]' >/dev/null
validate_records "$work/repriced" "$work/repriced.jsonl"
validate_aggregate "$work/repriced" "$work/repriced.jsonl"
expect_aggregate_reject stale_floors "$work/repriced" "$source_records"
expect_records_reject stale_floors "$work/repriced" "$source_records"
jq -c 'if .record=="console_session" and .workload_kind=="sixty_four_track_console" then .cycles_per_lane_sample=-1 else . end' \
    "$work/repriced.jsonl" >"$work/malformed.jsonl"
expect_aggregate_reject malformed_measurement "$work/repriced" "$work/malformed.jsonl"
expect_records_reject malformed_measurement "$work/repriced" "$work/malformed.jsonl"
set +e
( expect_aggregate_reject stale_self_check "$work/repriced" "$work/repriced.jsonl" ) 2>/dev/null; stale_aggregate_self=$?
( expect_records_reject stale_self_check "$work/repriced" "$work/repriced.jsonl" ) 2>/dev/null; stale_record_self=$?
( expect_aggregate_reject malformed_self_check "$work/repriced" "$work/repriced.jsonl" ) 2>/dev/null; malformed_aggregate_self=$?
( expect_records_reject malformed_self_check "$work/repriced" "$work/repriced.jsonl" ) 2>/dev/null; malformed_record_self=$?
set -e
[[ $stale_aggregate_self == 1 && $stale_record_self == 1 &&
   $malformed_aggregate_self == 1 && $malformed_record_self == 1 ]] || {
    printf 'negative assertion self-check did not discriminate acceptance\n' >&2; exit 1; }
mkdir "$work/broken"
cp "$work/repriced/"*.jq "$work/broken/"
printf 'this is not jq\n' >>"$work/broken/console-benchmark-record-validator.jq"
printf 'this is not jq\n' >>"$work/broken/console-benchmark-validator.jq"
set +e
expect_records_reject execution_self_check "$work/broken" "$work/malformed.jsonl" 2>/dev/null
broken_record_status=$?
expect_aggregate_reject execution_self_check "$work/broken" "$work/malformed.jsonl" 2>/dev/null
broken_aggregate_status=$?
set -e
[[ $broken_record_status == 2 && $broken_aggregate_status == 2 ]] || {
    printf 'validator execution errors were mistaken for predicate rejection\n' >&2; exit 1; }
printf 'historical repricing: PASS; revision=%s records=%s original_lib_sha256=%s repriced_lib_sha256=%s record_validator_sha256=%s aggregate_validator_sha256=%s\n' \
    "$revision" "$(wc -l <"$checked_records")" \
    "$(sha256sum "$work/console-benchmark-record-lib.jq"|awk '{print $1}')" \
    "$(sha256sum "$work/repriced/console-benchmark-record-lib.jq"|awk '{print $1}')" \
    "$(sha256sum "$work/console-benchmark-record-validator.jq"|awk '{print $1}')" \
    "$(sha256sum "$work/console-benchmark-validator.jq"|awk '{print $1}')"
