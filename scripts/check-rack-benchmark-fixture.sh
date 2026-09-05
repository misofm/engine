#!/usr/bin/env bash
# Verify the single frozen Issue-038 workload declaration and its exact manifest.
set -euo pipefail
[[ "$#" -le 1 ]] || { printf 'usage: %s [FIXTURE_ROOT]\n' "$0" >&2; exit 2; }
repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
fixture_root=${1:-"$repo_root/fixtures/rack/issue038-v1"}
manifest="$fixture_root/MANIFEST.tsv"
[[ -f "$manifest" && ! -L "$manifest" ]] || { printf 'missing regular Issue-038 manifest\n' >&2; exit 1; }
if capture_root=$(mktemp -d); then :; else
    status=$?
    printf 'Issue-038 capture directory creation failed (status %s)\n' "$status" >&2
    exit 1
fi
cleanup_capture_root() {
    rm -rf -- "$capture_root" || {
        printf 'Issue-038 capture directory cleanup failed\n' >&2
        return 1
    }
}
trap cleanup_capture_root EXIT
run_capture() {
    local stdout_file=$1 stderr_file=$2 input_file=$3
    shift 3
    if [[ -n "$input_file" ]]; then "$@" <"$input_file" >"$stdout_file" 2>"$stderr_file"; else "$@" >"$stdout_file" 2>"$stderr_file"; fi
}
report_capture_failure() {
    local description=$1 stdout_file=$2 stderr_file=$3 status=$4
    printf '%s failed (status %s)\n' "$description" "$status" >&2
    [[ ! -s "$stdout_file" ]] || { printf 'stdout:\n' >&2; cat "$stdout_file" >&2; }
    [[ ! -s "$stderr_file" ]] || { printf 'stderr:\n' >&2; cat "$stderr_file" >&2; }
    return 1
}
check_required_literal() {
    local label=$1 literal=$2 file=$3 out="$capture_root/$1.stdout" err="$capture_root/$1.stderr" status
    if run_capture "$out" "$err" '' grep -Fqx "$literal" "$file"; then status=0; else status=$?; fi
    if ((status == 1)); then printf 'Issue-038 required literal missing: %s\n' "$label" >&2; exit 1; fi
    if ((status != 0)); then report_capture_failure "$label" "$out" "$err" "$status"; exit 1; fi
}

sha_stdout="$capture_root/manifest-sha.stdout"; sha_stderr="$capture_root/manifest-sha.stderr"
if run_capture "$sha_stdout" "$sha_stderr" '' sha256sum "$manifest"; then sha_status=0; else sha_status=$?; fi
if ((sha_status != 0)); then report_capture_failure 'Issue-038 manifest sha256sum' "$sha_stdout" "$sha_stderr" "$sha_status"; exit 1; fi
hash_stdout="$capture_root/manifest-hash.stdout"; hash_stderr="$capture_root/manifest-hash.stderr"
if run_capture "$hash_stdout" "$hash_stderr" "$sha_stdout" awk '{print $1}'; then hash_status=0; else hash_status=$?; fi
if ((hash_status != 0)); then report_capture_failure 'Issue-038 manifest sha256sum awk' "$hash_stdout" "$hash_stderr" "$hash_status"; exit 1; fi
manifest_sha=$(<"$hash_stdout")
[[ "$manifest_sha" == "2d6b8c4b11bb00a17185d7777300194bf53ab30d86cf581a55886f07c5273985" ]] || { printf 'Issue-038 manifest identity mismatch\n' >&2; exit 1; }

find_stdout="$capture_root/find.stdout"; find_stderr="$capture_root/find.stderr"
discovery_status=0
if run_capture "$find_stdout" "$find_stderr" '' find "$fixture_root" -mindepth 1 -maxdepth 1 -type f -printf '%f\n'; then discovery_status=0
else
    discovery_status=$?
fi
if ((discovery_status != 0)); then report_capture_failure 'Issue-038 fixture discovery' "$find_stdout" "$find_stderr" "$discovery_status"; exit 1; fi
sort_stdout="$capture_root/sort.stdout"; sort_stderr="$capture_root/sort.stderr"
if run_capture "$sort_stdout" "$sort_stderr" "$find_stdout" sort; then sort_status=0; else sort_status=$?; fi
if ((sort_status != 0)); then report_capture_failure 'Issue-038 fixture discovery sort' "$sort_stdout" "$sort_stderr" "$sort_status"; exit 1; fi
mapfile -t actual_paths <"$sort_stdout"
[[ "${actual_paths[*]}" == "MANIFEST.tsv workloads.toml" ]] || { printf 'Issue-038 fixture has missing, non-regular, or unlisted entries\n' >&2; exit 1; }

header_stdout="$capture_root/header.stdout"; header_stderr="$capture_root/header.stderr"
if run_capture "$header_stdout" "$header_stderr" '' sed -n '1p' "$manifest"; then header_status=0; else header_status=$?; fi
if ((header_status != 0)); then report_capture_failure 'Issue-038 manifest header sed' "$header_stdout" "$header_stderr" "$header_status"; exit 1; fi
[[ "$(<"$header_stdout")" == $'path\tlength\tsha256' ]] || { printf 'Issue-038 manifest header mismatch\n' >&2; exit 1; }
lines_stdout="$capture_root/lines.stdout"; lines_stderr="$capture_root/lines.stderr"
if run_capture "$lines_stdout" "$lines_stderr" "$manifest" wc -l; then lines_status=0; else lines_status=$?; fi
if ((lines_status != 0)); then report_capture_failure 'Issue-038 manifest cardinality wc' "$lines_stdout" "$lines_stderr" "$lines_status"; exit 1; fi
[[ "$(<"$lines_stdout")" =~ ^[[:space:]]*2[[:space:]]*$ ]] || { printf 'Issue-038 manifest cardinality mismatch\n' >&2; exit 1; }
record_stdout="$capture_root/record.stdout"; record_stderr="$capture_root/record.stderr"
if run_capture "$record_stdout" "$record_stderr" '' sed -n '2p' "$manifest"; then record_status=0; else record_status=$?; fi
if ((record_status != 0)); then report_capture_failure 'Issue-038 manifest record sed' "$record_stdout" "$record_stderr" "$record_status"; exit 1; fi
IFS=$'\t' read -r path length expected_sha <"$record_stdout" || { printf 'Issue-038 manifest record is incomplete\n' >&2; exit 1; }
[[ "$path" == "workloads.toml" && "$length" =~ ^[0-9]+$ && "$expected_sha" =~ ^[0-9a-f]{64}$ ]] || { printf 'Issue-038 manifest record mismatch\n' >&2; exit 1; }
payload="$fixture_root/$path"
[[ -f "$payload" && ! -L "$payload" ]] || { printf 'missing regular Issue-038 workload payload\n' >&2; exit 1; }

wc_stdout="$capture_root/payload-wc.stdout"; wc_stderr="$capture_root/payload-wc.stderr"
payload_wc_status=0
if run_capture "$wc_stdout" "$wc_stderr" "$payload" wc -c; then payload_wc_status=0
else
    payload_wc_status=$?
fi
if ((payload_wc_status != 0)); then report_capture_failure 'Issue-038 workload length wc' "$wc_stdout" "$wc_stderr" "$payload_wc_status"; exit 1; fi
[[ "$(<"$wc_stdout")" =~ ^[[:space:]]*${length}[[:space:]]*$ ]] || { printf 'Issue-038 workload length mismatch\n' >&2; exit 1; }
payload_sha_stdout="$capture_root/payload-sha.stdout"; payload_sha_stderr="$capture_root/payload-sha.stderr"
if run_capture "$payload_sha_stdout" "$payload_sha_stderr" '' sha256sum "$payload"; then payload_sha_status=0; else payload_sha_status=$?; fi
if ((payload_sha_status != 0)); then report_capture_failure 'Issue-038 workload sha256sum' "$payload_sha_stdout" "$payload_sha_stderr" "$payload_sha_status"; exit 1; fi
payload_hash_stdout="$capture_root/payload-hash.stdout"; payload_hash_stderr="$capture_root/payload-hash.stderr"
if run_capture "$payload_hash_stdout" "$payload_hash_stderr" "$payload_sha_stdout" awk '{print $1}'; then payload_hash_status=0; else payload_hash_status=$?; fi
if ((payload_hash_status != 0)); then report_capture_failure 'Issue-038 workload sha256sum awk' "$payload_hash_stdout" "$payload_hash_stderr" "$payload_hash_status"; exit 1; fi
[[ "$(<"$payload_hash_stdout")" == "$expected_sha" ]] || { printf 'Issue-038 workload hash mismatch\n' >&2; exit 1; }
check_required_literal observations 'observations_per_round = 1000' "$payload"
check_required_literal sample-rate 'sample_rate_hz = 48000' "$payload"
check_required_literal quantum 'quantum_frames = 128' "$payload"
count_stdout="$capture_root/workload-count.stdout"; count_stderr="$capture_root/workload-count.stderr"
if run_capture "$count_stdout" "$count_stderr" '' rg -c '^  \"(scalar_eight_tracks|host_selected_eight_track_bank|mixed_twelve_track_graph):' "$payload"; then count_status=0; else count_status=$?; fi
if ((count_status != 0)); then report_capture_failure 'Issue-038 workload name count rg' "$count_stdout" "$count_stderr" "$count_status"; exit 1; fi
[[ "$(<"$count_stdout")" == 3 ]]
