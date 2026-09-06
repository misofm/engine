#!/usr/bin/env bash
# Exhaustive bounded schema mutations and scratch-only runner lifecycle tests. No audio launches.
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
record=scripts/fixtures/rack-benchmark-validator-record.json
single_validator=scripts/rack-benchmark-record-validator.jq
aggregate_validator=scripts/rack-benchmark-validator.jq
jq -e -L scripts -f "$single_validator" "$record" >/dev/null
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT

fixture_checker=scripts/check-rack-benchmark-fixture.sh
fixture_copy="$scratch/fixture"
cp -a fixtures/rack/issue038-v1 "$fixture_copy"
bash "$fixture_checker" "$fixture_copy"
cp "$fixture_copy/workloads.toml" "$scratch/workloads.original"
printf 'mutation\n' >>"$fixture_copy/workloads.toml"
if bash "$fixture_checker" "$fixture_copy" >/dev/null 2>&1; then
    printf 'Issue-038 fixture checker accepted content corruption\n' >&2
    exit 1
fi
cp "$scratch/workloads.original" "$fixture_copy/workloads.toml"

# Preserve the exact regular-file policy and optional fixture-root semantics.
mv "$fixture_copy/MANIFEST.tsv" "$scratch/manifest.saved"
if missing_manifest_output=$(bash "$fixture_checker" "$fixture_copy" 2>&1); then exit 96; else missing_manifest_status=$?; fi
[[ "$missing_manifest_status" == 1 && "$missing_manifest_output" == 'missing regular Issue-038 manifest' ]] || exit 96
mv "$scratch/manifest.saved" "$fixture_copy/MANIFEST.tsv"
if missing_root_output=$(bash "$fixture_checker" "$scratch/absent-fixture" 2>&1); then exit 96; else missing_root_status=$?; fi
[[ "$missing_root_status" == 1 && "$missing_root_output" == 'missing regular Issue-038 manifest' ]] || exit 96
mv "$fixture_copy/MANIFEST.tsv" "$scratch/manifest.saved"; ln -s "$scratch/manifest.saved" "$fixture_copy/MANIFEST.tsv"
if bash "$fixture_checker" "$fixture_copy" >/dev/null 2>&1; then exit 96; fi
rm "$fixture_copy/MANIFEST.tsv"; mv "$scratch/manifest.saved" "$fixture_copy/MANIFEST.tsv"
mv "$fixture_copy/workloads.toml" "$scratch/workloads.saved"; ln -s "$scratch/workloads.saved" "$fixture_copy/workloads.toml"
if bash "$fixture_checker" "$fixture_copy" >/dev/null 2>&1; then exit 96; fi
rm "$fixture_copy/workloads.toml"; mv "$scratch/workloads.saved" "$fixture_copy/workloads.toml"
mkdir "$fixture_copy/unrelated-directory"; ln -s unrelated-directory "$fixture_copy/unrelated-symlink"
bash "$fixture_checker" "$fixture_copy"
rm "$fixture_copy/unrelated-symlink"; rmdir "$fixture_copy/unrelated-directory"
cp "$fixture_copy/MANIFEST.tsv" "$scratch/manifest.original"
printf 'mutation\n' >>"$fixture_copy/MANIFEST.tsv"
if bash "$fixture_checker" "$fixture_copy" >/dev/null 2>&1; then
    printf 'Issue-038 fixture checker accepted manifest corruption\n' >&2
    exit 1
fi
cp "$scratch/manifest.original" "$fixture_copy/MANIFEST.tsv"
printf 'unlisted\n' >"$fixture_copy/unlisted"
if bash "$fixture_checker" "$fixture_copy" >/dev/null 2>&1; then
    printf 'Issue-038 fixture checker accepted an unlisted file\n' >&2
    exit 1
fi
rm "$fixture_copy/unlisted"
rm "$fixture_copy/workloads.toml"
if bash "$fixture_checker" "$fixture_copy" >/dev/null 2>&1; then
    printf 'Issue-038 fixture checker accepted a missing payload\n' >&2
    exit 1
fi
cp "$scratch/workloads.original" "$fixture_copy/workloads.toml"

# Exercise every frozen producer/predicate after all earlier checks have succeeded.  Each shim
# first runs the resolved real command, records its complete result, and only then injects the
# selected failure.  The mode is a private scratch file, not a runtime environment surface.
scan_bin="$scratch/scan-bin"
mkdir -p "$scan_bin"
for scan_tool in sha256sum awk find sort sed wc grep rg; do
    scan_real=$(command -v "$scan_tool")
    [[ -x "$scan_real" ]] || { printf 'missing real scan delegate: %s\n' "$scan_tool" >&2; exit 96; }
    printf '%s\n' "$scan_real" >"$scan_bin/real-$scan_tool"
done
cat >"$scan_bin/scan-shim" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
shim_root=$(cd "$(dirname "$0")" && pwd)
tool=$(basename "$0")
real=$(<"$shim_root/real-$tool")
count_file="$shim_root/count-$tool"
count=0; [[ ! -f "$count_file" ]] || count=$(<"$count_file")
count=$((count + 1)); printf '%s\n' "$count" >"$count_file"
delegate_out="$shim_root/delegate-$tool.stdout"
delegate_err="$shim_root/delegate-$tool.stderr"
if "$real" "$@" >"$delegate_out" 2>"$delegate_err"; then delegate_status=0; else delegate_status=$?; fi
[[ "$delegate_status" == 0 ]] || { cat "$delegate_out"; cat "$delegate_err" >&2; exit 95; }
mode=none mode_tool= mode_nth= behavior=full injected_status=0 sentinel=
[[ ! -f "$shim_root/mode" ]] || read -r mode mode_tool mode_nth behavior injected_status sentinel <"$shim_root/mode"
if [[ "$tool" == "$mode_tool" && "$count" == "$mode_nth" ]]; then
    case "$behavior" in full) cat "$delegate_out" ;; empty) : ;; bad) printf 'malformed\n' ;; *) exit 94 ;; esac
    cat "$delegate_err" >&2
    printf '%s delegate_status=%s\n' "$sentinel" "$delegate_status" >&2
    exit "$injected_status"
fi
cat "$delegate_out"
cat "$delegate_err" >&2
EOF
chmod 755 "$scan_bin/scan-shim"
for scan_tool in sha256sum awk find sort sed wc grep rg; do ln -s scan-shim "$scan_bin/$scan_tool"; done

configure_scan_failure() {
    local mode=$1 tool=$2 nth=$3 behavior=$4 status=$5 sentinel=$6
    rm -f "$scan_bin"/count-* "$scan_bin"/delegate-*
    printf '%s %s %s %s %s %s\n' "$mode" "$tool" "$nth" "$behavior" "$status" "$sentinel" >"$scan_bin/mode"
}
validate_delegate_payload() {
    local mode=$1 tool=$2 payload=$3 normalized
    [[ ! -s "$scan_bin/delegate-$tool.stderr" ]] || return 1
    case "$mode" in
        manifest_sha) [[ "$payload" == "2d6b8c4b11bb00a17185d7777300194bf53ab30d86cf581a55886f07c5273985  $fixture_copy/MANIFEST.tsv" ]] ;;
        manifest_awk) [[ "$payload" == 2d6b8c4b11bb00a17185d7777300194bf53ab30d86cf581a55886f07c5273985 ]] ;;
        find_full|find_empty|conditional_find|discovery_mutant)
            normalized=$(printf '%s\n' "$payload" | sort)
            [[ "$normalized" == $'MANIFEST.tsv\nworkloads.toml' && "$(printf '%s\n' "$payload" | wc -l)" == 2 ]]
            ;;
        sort_full|sort_empty) [[ "$payload" == $'MANIFEST.tsv\nworkloads.toml' ]] ;;
        header_sed) [[ "$payload" == $'path\tlength\tsha256' ]] ;;
        manifest_wc) [[ "$payload" =~ ^[[:space:]]*2[[:space:]]*$ ]] ;;
        record_sed) [[ "$payload" == $'workloads.toml\t456\t1f67ed9960e5a6728f02442b65af70704957d5f6056865d8b44555637273188d' ]] ;;
        payload_wc|payload_wc_mutant) [[ "$payload" =~ ^[[:space:]]*456[[:space:]]*$ ]] ;;
        payload_sha) [[ "$payload" == "1f67ed9960e5a6728f02442b65af70704957d5f6056865d8b44555637273188d  $fixture_copy/workloads.toml" ]] ;;
        payload_awk) [[ "$payload" == 1f67ed9960e5a6728f02442b65af70704957d5f6056865d8b44555637273188d ]] ;;
        observations_grep|sample_rate_grep|quantum_grep|grep_nomatch) [[ -z "$payload" ]] ;;
        workload_rg) [[ "$payload" == 3 ]] ;;
        *) return 1 ;;
    esac
}
assert_checker_rejects_producer_failure() {
    local checker=$1 mode=$2 tool=$3 nth=$4 behavior=$5 injected_status=$6 description=$7 sentinel=$8
    local output status delegate_payload expected
    configure_scan_failure "$mode" "$tool" "$nth" "$behavior" "$injected_status" "$sentinel"
    if output=$(PATH="$scan_bin:$PATH" bash "$checker" "$fixture_copy" 2>&1); then
        printf 'ASSERT %s unexpected checker success\n' "$mode" >&2
        return 97
    else
        status=$?
    fi
    [[ "$status" == 1 ]] || { printf 'ASSERT %s wrong checker status=%s\n%s\n' "$mode" "$status" "$output" >&2; return 96; }
    delegate_payload=$(<"$scan_bin/delegate-$tool.stdout")
    validate_delegate_payload "$mode" "$tool" "$delegate_payload" || { printf 'ASSERT %s unfaithful delegate payload\n' "$mode" >&2; return 96; }
    if [[ "$behavior" == empty ]]; then delegate_payload=; fi
    expected="$description failed (status $injected_status)"
    [[ -z "$delegate_payload" ]] || expected+=$'\nstdout:\n'"$delegate_payload"
    expected+=$'\nstderr:\n'"$sentinel delegate_status=0"
    [[ "$output" == "$expected" ]] || { printf 'ASSERT %s wrong exact diagnostic\nEXPECTED:\n%s\nACTUAL:\n%s\n' "$mode" "$expected" "$output" >&2; return 96; }
}

# Complete full-output/error table.  The command occurrence is fixed by the checker sequence.
while IFS='|' read -r mode tool nth behavior injected_status description sentinel; do
    assert_checker_rejects_producer_failure "$fixture_checker" "$mode" "$tool" "$nth" "$behavior" "$injected_status" "$description" "$sentinel"
done <<'PRODUCER_FAILURES'
manifest_sha|sha256sum|1|full|71|Issue-038 manifest sha256sum|MANIFEST_SHA_SENTINEL
manifest_awk|awk|1|full|72|Issue-038 manifest sha256sum awk|MANIFEST_AWK_SENTINEL
find_full|find|1|full|73|Issue-038 fixture discovery|FIND_FULL_SENTINEL
find_empty|find|1|empty|74|Issue-038 fixture discovery|FIND_EMPTY_SENTINEL
sort_full|sort|1|full|75|Issue-038 fixture discovery sort|SORT_FULL_SENTINEL
sort_empty|sort|1|empty|76|Issue-038 fixture discovery sort|SORT_EMPTY_SENTINEL
header_sed|sed|1|full|77|Issue-038 manifest header sed|HEADER_SED_SENTINEL
manifest_wc|wc|1|full|78|Issue-038 manifest cardinality wc|MANIFEST_WC_SENTINEL
record_sed|sed|2|full|79|Issue-038 manifest record sed|RECORD_SED_SENTINEL
payload_wc|wc|2|full|80|Issue-038 workload length wc|PAYLOAD_WC_SENTINEL
payload_sha|sha256sum|2|full|81|Issue-038 workload sha256sum|PAYLOAD_SHA_SENTINEL
payload_awk|awk|2|full|82|Issue-038 workload sha256sum awk|PAYLOAD_AWK_SENTINEL
observations_grep|grep|1|empty|83|observations|OBSERVATIONS_GREP_SENTINEL
sample_rate_grep|grep|2|empty|84|sample-rate|SAMPLE_RATE_GREP_SENTINEL
quantum_grep|grep|3|empty|85|quantum|QUANTUM_GREP_SENTINEL
workload_rg|rg|1|full|86|Issue-038 workload name count rg|WORKLOAD_RG_SENTINEL
PRODUCER_FAILURES

assert_bad_capture_value() {
    local mode=$1 tool=$2 nth=$3 behavior=$4 expected=$5 output status
    configure_scan_failure "$mode" "$tool" "$nth" "$behavior" 0 VALUE_SENTINEL
    if output=$(PATH="$scan_bin:$PATH" bash "$fixture_checker" "$fixture_copy" 2>&1); then exit 96; else status=$?; fi
    [[ "$status" == 1 && "$output" == "$expected" ]] || { printf 'ASSERT %s wrong value rejection\n%s\n' "$mode" "$output" >&2; exit 96; }
}
assert_bad_capture_value manifest_hash_empty awk 1 empty 'Issue-038 manifest identity mismatch'
assert_bad_capture_value manifest_hash_malformed awk 1 bad 'Issue-038 manifest identity mismatch'
assert_bad_capture_value header_empty sed 1 empty 'Issue-038 manifest header mismatch'
assert_bad_capture_value header_malformed sed 1 bad 'Issue-038 manifest header mismatch'
assert_bad_capture_value lines_empty wc 1 empty 'Issue-038 manifest cardinality mismatch'
assert_bad_capture_value lines_malformed wc 1 bad 'Issue-038 manifest cardinality mismatch'
assert_bad_capture_value payload_count_empty wc 2 empty 'Issue-038 workload length mismatch'
assert_bad_capture_value payload_count_malformed wc 2 bad 'Issue-038 workload length mismatch'
assert_bad_capture_value payload_hash_empty awk 2 empty 'Issue-038 workload hash mismatch'
assert_bad_capture_value payload_hash_malformed awk 2 bad 'Issue-038 workload hash mismatch'
assert_bad_capture_value workload_count_empty rg 1 empty ''
assert_bad_capture_value workload_count_malformed rg 1 bad ''

# A status-1 required predicate is a missing literal, distinct from an operational failure.
configure_scan_failure grep_nomatch grep 3 empty 1 GREP_NOMATCH_SENTINEL
if grep_nomatch_output=$(PATH="$scan_bin:$PATH" bash "$fixture_checker" "$fixture_copy" 2>&1); then exit 96; else grep_nomatch_status=$?; fi
[[ "$grep_nomatch_status" == 1 && "$grep_nomatch_output" == 'Issue-038 required literal missing: quantum' ]] || exit 96

# The builtin read and its field predicates reject incomplete and malformed successful records.
for record_mode in incomplete malformed; do
    rm -f "$scan_bin"/count-* "$scan_bin"/delegate-*; printf 'none x 0 full 0 NONE\n' >"$scan_bin/mode"
    record_checker="$scratch/checker-record-$record_mode.sh"
    cp "$fixture_checker" "$record_checker"
    if [[ "$record_mode" == incomplete ]]; then
        sed -i '/IFS=.*read -r path/i\\printf '\''workloads.toml\\t456'\'' >"$record_stdout"' "$record_checker"
        expected_record='Issue-038 manifest record is incomplete'
    else
        sed -i '/IFS=.*read -r path/i\\printf '\''workloads.toml\\tbad\\tnot-a-hash\\n'\'' >"$record_stdout"' "$record_checker"
        expected_record='Issue-038 manifest record mismatch'
    fi
    if record_output=$(PATH="$scan_bin:$PATH" bash "$record_checker" "$fixture_copy" 2>&1); then exit 96; else record_status=$?; fi
    [[ "$record_status" == 1 && "$record_output" == "$expected_record" ]] || exit 96
done

# Source the exact checker as the conditional command in an isolated child shell.  This is the
# context in which Bash disables inherited errexit for the sourced body.
source_harness="$scratch/source-checker-conditionally.sh"
printf '%s\n' "$root/$fixture_checker" >"$scratch/source-checker-path"
cat >"$source_harness" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
harness_root=$(cd "$(dirname "$0")" && pwd)
checker=$(<"$harness_root/source-checker-path")
if source "$checker" "$@"; then
    exit 0
else
    status=$?
    exit "$status"
fi
EOF
chmod 755 "$source_harness"
bash "$source_harness" "$fixture_copy"
assert_checker_rejects_producer_failure "$source_harness" conditional_find find 1 full 87 \
    'Issue-038 fixture discovery' CONDITIONAL_FIND_SENTINEL

# Cleanup has an explicit final-status rule.  The selective rm shim leaves only checker capture
# directories behind; the resolved real rm removes them after each assertion.
cleanup_bin="$scratch/cleanup-bin"
cleanup_tmp="$scratch/cleanup-tmp"
mkdir -p "$cleanup_bin" "$cleanup_tmp"
real_rm=$(command -v rm)
[[ -x "$real_rm" ]] || exit 96
printf '%s\n' "$real_rm" >"$cleanup_bin/real-rm"
printf '%s\n' "$cleanup_tmp" >"$cleanup_bin/target-root"
cat >"$cleanup_bin/rm" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
shim_root=$(cd "$(dirname "$0")" && pwd)
real=$(<"$shim_root/real-rm")
target_root=$(<"$shim_root/target-root")
if [[ "${1:-}" == -rf && "${2:-}" == -- && "${3:-}" == "$target_root"/* ]]; then
    printf 'RM_CLEANUP_SENTINEL status=88\n' >&2
    exit 88
fi
exec "$real" "$@"
EOF
chmod 755 "$cleanup_bin/rm"

rm -f "$scan_bin"/count-* "$scan_bin"/delegate-*
printf 'none x 0 full 0 NONE\n' >"$scan_bin/mode"
if cleanup_valid_output=$(TMPDIR="$cleanup_tmp" PATH="$cleanup_bin:$scan_bin:$PATH" \
    bash "$source_harness" "$fixture_copy" 2>&1); then exit 96; else cleanup_valid_status=$?; fi
cleanup_valid_expected=$'RM_CLEANUP_SENTINEL status=88\nIssue-038 capture directory cleanup failed'
[[ "$cleanup_valid_status" == 1 && "$cleanup_valid_output" == "$cleanup_valid_expected" ]] || exit 96
"$real_rm" -rf -- "$cleanup_tmp"/*

configure_scan_failure conditional_find_cleanup find 1 full 87 CONDITIONAL_FIND_CLEANUP_SENTINEL
if cleanup_failed_output=$(TMPDIR="$cleanup_tmp" PATH="$cleanup_bin:$scan_bin:$PATH" \
    bash "$source_harness" "$fixture_copy" 2>&1); then exit 96; else cleanup_failed_status=$?; fi
cleanup_find_payload=$(<"$scan_bin/delegate-find.stdout")
validate_delegate_payload conditional_find find "$cleanup_find_payload" || exit 96
cleanup_failed_expected=$'Issue-038 fixture discovery failed (status 87)\nstdout:\n'"$cleanup_find_payload"$'\nstderr:\nCONDITIONAL_FIND_CLEANUP_SENTINEL delegate_status=0\nRM_CLEANUP_SENTINEL status=88\nIssue-038 capture directory cleanup failed'
[[ "$cleanup_failed_status" == 1 && "$cleanup_failed_output" == "$cleanup_failed_expected" ]] || exit 96
"$real_rm" -rf -- "$cleanup_tmp"/*

# Exactly two one-site production mutants use the same assertion as original and restored runs.
discovery_line=$(rg -n -F '    discovery_status=$?' "$fixture_checker" | cut -d: -f1)
payload_wc_line=$(rg -n -F '    payload_wc_status=$?' "$fixture_checker" | cut -d: -f1)
[[ "$(printf '%s\n' "$discovery_line" | wc -l)" == 1 && -n "$discovery_line" ]] || exit 96
[[ "$(printf '%s\n' "$payload_wc_line" | wc -l)" == 1 && -n "$payload_wc_line" ]] || exit 96
for mutant_case in discovery payload_wc; do
    if [[ "$mutant_case" == discovery ]]; then
        mutant="$scratch/checker-discovery-status.sh"; sed "${discovery_line}c\\    :" "$fixture_checker" >"$mutant"
        mode=discovery_mutant tool=find nth=1 injected=73 description='Issue-038 fixture discovery' sentinel=DISCOVERY_MUTANT_SENTINEL
        expected_assignment='-    discovery_status=$?'
    else
        mutant="$scratch/checker-payload-wc-status.sh"; sed "${payload_wc_line}c\\    :" "$fixture_checker" >"$mutant"
        mode=payload_wc_mutant tool=wc nth=2 injected=74 description='Issue-038 workload length wc' sentinel=PAYLOAD_WC_MUTANT_SENTINEL
        expected_assignment='-    payload_wc_status=$?'
    fi
    chmod 755 "$mutant"
    mutant_diff=$(diff -U0 "$fixture_checker" "$mutant" || :)
    [[ "$(printf '%s\n' "$mutant_diff" | rg -c '^@@')" == 1 ]] || exit 96
    [[ "$(printf '%s\n' "$mutant_diff" | rg -Fxc -- "$expected_assignment")" == 1 ]] || exit 96
    [[ "$(printf '%s\n' "$mutant_diff" | rg -Fxc -- '+    :')" == 1 ]] || exit 96
    assert_checker_rejects_producer_failure "$fixture_checker" "$mode" "$tool" "$nth" full "$injected" "$description" "$sentinel"
    if mutant_output=$(assert_checker_rejects_producer_failure "$mutant" "$mode" "$tool" "$nth" full "$injected" "$description" "$sentinel" 2>&1); then exit 96; else mutant_status=$?; fi
    [[ "$mutant_status" == 97 && "$mutant_output" == "ASSERT $mode unexpected checker success" ]] || exit 96
    assert_checker_rejects_producer_failure "$fixture_checker" "$mode" "$tool" "$nth" full "$injected" "$description" "$sentinel"
done

reject_single() {
    local mutation=$1
    jq "$mutation" "$record" >"$scratch/mutated.json"
    if jq -e -L scripts -f "$single_validator" "$scratch/mutated.json" >/dev/null; then
        printf 'rack benchmark validator accepted mutation: %s\n' "$mutation" >&2
        exit 1
    fi
}

# Every schema key is individually required and rejects an object in place of its declared type.
while IFS= read -r key; do
    reject_single "del(.[\"$key\"])"
    reject_single ".[\"$key\"]={\"wrong_type\":true}"
done < <(jq -r 'keys[]' "$record")

# Each semantic family has a bounded valid-type mutation, including matched nonzero audit totals.
while IFS= read -r mutation; do
    [[ -n "$mutation" ]] && reject_single "$mutation"
done <<'MUTATIONS'
.schema_version=0
.issue=8
.workload_kind="bad"
.workload_id="issue038.bad.48000hz.q128"
.round=3
.sample_rate_hz=96000
.quantum_frames=64
.tracks=7
.bank_backend="Simd8"
.bank_width=8
.bank_count=1
.scalar_tail_count=7
.scalar_fallback_count=1
.identity_lane_count=1
.observations=999
.percentile_method="linear"
.units="ns"
.min_ns_per_frame=-1
.p50_ns_per_frame=1.5
.p95_ns_per_frame=1
.p99_ns_per_frame=2
.p99_9_ns_per_frame=3
.max_ns_per_frame=4
.descriptive_only=false
.candidate_commit_sha256="bad"
.binary_sha256="bad"
.fixture_id="wrong"
.fixture_sha256="bad"
.input_sha256="bad"
.output_sha256="bad"
.render_errors=1
.render_allocations=1|.forbidden_operation_total=1
.render_deallocations=1|.forbidden_operation_total=1
.render_locks=1|.forbidden_operation_total=1
.render_feature_detection_calls=1|.forbidden_operation_total=1
.render_logs=1|.forbidden_operation_total=1
.render_file_io=1|.forbidden_operation_total=1
.render_network_io=1|.forbidden_operation_total=1
.render_syscalls=1|.forbidden_operation_total=1
.render_panic_unwinds=1|.forbidden_operation_total=1
.forbidden_operation_total=1
.cpu_model=""
.cpu_model="unknown"
.cpu_model="default"
.missing_metadata|=reverse
.missing_metadata += ["architecture"]
.architecture="x86_64"
.extra=true
MUTATIONS

jq -n --slurpfile scalar "$record" '
  $scalar[0] as $r |
  [$r,
   ($r|.round=2),
   ($r|.workload_kind="host_selected_eight_track_bank"|.workload_id="issue038.host_selected_eight_track_bank.48000hz.q128"|.bank_backend="Simd8"|.bank_width=8|.bank_count=1|.scalar_tail_count=0|.output_sha256="5555555555555555555555555555555555555555555555555555555555555555"),
   ($r|.round=2|.workload_kind="host_selected_eight_track_bank"|.workload_id="issue038.host_selected_eight_track_bank.48000hz.q128"|.bank_backend="Simd8"|.bank_width=8|.bank_count=1|.scalar_tail_count=0|.output_sha256="5555555555555555555555555555555555555555555555555555555555555555"),
   ($r|.workload_kind="mixed_twelve_track_graph"|.workload_id="issue038.mixed_twelve_track_graph.48000hz.q128"|.tracks=12|.bank_backend="Simd8"|.bank_width=8|.bank_count=1|.scalar_tail_count=2|.scalar_fallback_count=2|.identity_lane_count=2|.input_sha256="6666666666666666666666666666666666666666666666666666666666666666"|.output_sha256="7777777777777777777777777777777777777777777777777777777777777777"),
   ($r|.round=2|.workload_kind="mixed_twelve_track_graph"|.workload_id="issue038.mixed_twelve_track_graph.48000hz.q128"|.tracks=12|.bank_backend="Simd8"|.bank_width=8|.bank_count=1|.scalar_tail_count=2|.scalar_fallback_count=2|.identity_lane_count=2|.input_sha256="6666666666666666666666666666666666666666666666666666666666666666"|.output_sha256="7777777777777777777777777777777777777777777777777777777777777777")]
' >"$scratch/aggregate.json"
jq -e -L scripts -f "$aggregate_validator" "$scratch/aggregate.json" >/dev/null

reject_aggregate() {
    local mutation=$1
    jq "$mutation" "$scratch/aggregate.json" >"$scratch/aggregate-mutated.json"
    if jq -e -L scripts -f "$aggregate_validator" "$scratch/aggregate-mutated.json" >/dev/null; then
        printf 'rack benchmark aggregate accepted mutation: %s\n' "$mutation" >&2
        exit 1
    fi
}
while IFS= read -r mutation; do
    [[ -n "$mutation" ]] && reject_aggregate "$mutation"
done <<'AGGREGATE_MUTATIONS'
.[0].round=2
del(.[5])
. + [.[0]]
.[0].candidate_commit_sha256="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
.[0].binary_sha256="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
.[0].fixture_sha256="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
.[0].input_sha256="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
.[0].output_sha256="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
.[2].bank_backend="Simd4"
.[0].architecture="x86_64"|.[0].missing_metadata -= ["architecture"]
AGGREGATE_MUTATIONS

runner=scripts/run-rack-benchmark.sh
for argument in --retry '--rounds 2' extra; do
    if bash "$runner" "$argument" >/dev/null 2>&1; then
        printf 'runner accepted argument: %s\n' "$argument" >&2
        exit 1
    fi
done

# Build a hermetic scratch repository with fake cargo/git/rustc and a synthetic record emitter.
# These process launches never enter engine DSP and therefore keep audio workload_launches at zero.
template="$scratch/runner-template"
mkdir -p "$template/scripts/fixtures" "$template/bin" "$template/fixtures/rack/issue038-v1"
cp "$runner" "$template/scripts/run-rack-benchmark.sh"
cp scripts/rack-benchmark-record-lib.jq scripts/rack-benchmark-record-validator.jq \
    scripts/rack-benchmark-validator.jq "$template/scripts/"
cp "$record" "$template/scripts/fixtures/rack-benchmark-validator-record.json"
cp fixtures/rack/issue038-v1/MANIFEST.tsv "$template/fixtures/rack/issue038-v1/MANIFEST.tsv"

cat >"$template/bin/git" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "$1" in
    status) exit 0 ;;
    rev-parse) printf '%s\n' 0123456789abcdef0123456789abcdef01234567 ;;
    *) exit 90 ;;
esac
EOF
cat >"$template/bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/release
cp "$MISO_ENGINE_TEST_FAKE_BENCH" target/release/bench
chmod 755 target/release/bench
EOF
cat >"$template/bin/rustc" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${MISO_ENGINE_TEST_RUSTC_PIPE_FAIL:-0}" == 1 && "${1:-}" == -vV ]]; then
    printf 'host: x86_64-unknown-linux-gnu\nLLVM version: 22.1.6\n'
    exit 73
fi
exec "$MISO_ENGINE_TEST_REAL_RUSTC" "$@"
EOF
cat >"$template/scripts/fixtures/fake-bench.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$MISO_ENGINE_BENCH_ROUND" >>"$MISO_ENGINE_TEST_LAUNCH_LOG"
mode=${MISO_ENGINE_TEST_BENCH_MODE:-success}
round=$MISO_ENGINE_BENCH_ROUND
if [[ "$mode" == interrupt && "$round" == warmup ]]; then
    kill -TERM "$PPID"
    exit 143
fi
if [[ "$mode" == warmup_fail && "$round" == warmup ]]; then
    printf 'synthetic warmup failure\n' >&2
    exit 71
fi
if [[ "$mode" == round1_fail && "$round" == 1 ]]; then
    printf '{"partial":"round1"}\n'
    exit 72
fi
if [[ "$mode" == round2_fail && "$round" == 2 ]]; then
    printf '{"partial":"round2"}\n'
    exit 74
fi
if [[ "$round" == warmup ]]; then
    round=1
fi
if [[ "$mode" == invalid ]]; then
    printf '{}\n{}\n{}\n'
    exit 0
fi
root=$(cd "$(dirname "$0")/../.." && pwd)
base="$root/scripts/fixtures/rack-benchmark-validator-record.json"
common=(--argjson round "$round" --arg candidate "$MISO_ENGINE_BENCH_CANDIDATE_SHA256" --arg binary "$MISO_ENGINE_BENCH_BINARY_SHA256")
jq -c "${common[@]}" '.round=$round|.candidate_commit_sha256=$candidate|.binary_sha256=$binary' "$base"
jq -c "${common[@]}" '.round=$round|.candidate_commit_sha256=$candidate|.binary_sha256=$binary|.workload_kind="host_selected_eight_track_bank"|.workload_id="issue038.host_selected_eight_track_bank.48000hz.q128"|.bank_backend="Simd8"|.bank_width=8|.bank_count=1|.scalar_tail_count=0|.output_sha256="5555555555555555555555555555555555555555555555555555555555555555"' "$base"
jq -c "${common[@]}" '.round=$round|.candidate_commit_sha256=$candidate|.binary_sha256=$binary|.workload_kind="mixed_twelve_track_graph"|.workload_id="issue038.mixed_twelve_track_graph.48000hz.q128"|.tracks=12|.bank_backend="Simd8"|.bank_width=8|.bank_count=1|.scalar_tail_count=2|.scalar_fallback_count=2|.identity_lane_count=2|.input_sha256="6666666666666666666666666666666666666666666666666666666666666666"|.output_sha256="7777777777777777777777777777777777777777777777777777777777777777"' "$base"
EOF
chmod 755 "$template/bin/git" "$template/bin/cargo" "$template/bin/rustc" \
    "$template/scripts/fixtures/fake-bench.sh"

case_number=0
new_case() {
    local name=$1
    case_number=$((case_number + 1))
    case_root="$scratch/case-$case_number-$name"
    cp -a "$template" "$case_root"
    launch_log="$case_root/synthetic-launches.log"
}
run_case() {
    local mode=$1
    local selected
    for selected_tool in cargo git rustc; do
        selected=$(PATH="$case_root/bin:$PATH" command -v "$selected_tool")
        [[ "$selected" == "$case_root/bin/$selected_tool" ]] || { printf 'lifecycle selected non-fake %s: %s\n' "$selected_tool" "$selected" >&2; exit 96; }
    done
    [[ -x "$case_root/scripts/fixtures/fake-bench.sh" ]] || { printf 'missing synthetic emitter\n' >&2; exit 96; }
    MISO_ENGINE_TEST_BENCH_MODE="$mode" \
    MISO_ENGINE_TEST_LAUNCH_LOG="$launch_log" \
    MISO_ENGINE_TEST_FAKE_BENCH="$case_root/scripts/fixtures/fake-bench.sh" \
    MISO_ENGINE_TEST_REAL_RUSTC="$(command -v rustc)" \
    PATH="$case_root/bin:$PATH" \
    bash "$case_root/scripts/run-rack-benchmark.sh"
}
expect_failure_reason() {
    local reason=$1
    jq -e --arg reason "$reason" '.status == "FAIL" and .reason == $reason' \
        "$case_root/artifacts/issue038/rack-benchmark.disposition.json" >/dev/null
}

new_case overwrite
mkdir -p "$case_root/artifacts/issue038"
printf 'protected raw bytes\n' >"$case_root/artifacts/issue038/rack-benchmark.raw.jsonl"
if run_case success >/dev/null 2>&1; then
    printf 'runner overwrote a pre-existing raw artifact\n' >&2
    exit 1
fi
[[ ! -e "$launch_log" ]]
[[ "$(<"$case_root/artifacts/issue038/rack-benchmark.raw.jsonl")" == 'protected raw bytes' ]]

new_case warmup-failure
if run_case warmup_fail >/dev/null 2>&1; then
    printf 'runner swallowed warmup failure\n' >&2
    exit 1
fi
expect_failure_reason warmup_failed
[[ "$(wc -l <"$launch_log")" == 1 ]]
[[ ! -e "$case_root/artifacts/issue038/rack-benchmark.raw.jsonl" ]]

new_case round1-failure
if run_case round1_fail >/dev/null 2>&1; then
    printf 'runner swallowed measured round-1 failure\n' >&2
    exit 1
fi
expect_failure_reason round_1_failed
[[ "$(wc -l <"$launch_log")" == 2 ]]
grep -Fqx '{"partial":"round1"}' "$case_root/artifacts/issue038/rack-benchmark.raw.jsonl"
[[ ! -e "$case_root/artifacts/issue038/rack-benchmark.accepted.jsonl" ]]

new_case round2-failure
if run_case round2_fail >/dev/null 2>&1; then
    printf 'runner swallowed measured round-2 failure\n' >&2
    exit 1
fi
expect_failure_reason round_2_failed
[[ "$(wc -l <"$launch_log")" == 3 ]]
[[ "$(tail -n 1 "$case_root/artifacts/issue038/rack-benchmark.raw.jsonl")" == '{"partial":"round2"}' ]]

new_case validation-failure
if run_case invalid >/dev/null 2>&1; then
    printf 'runner promoted invalid synthetic output\n' >&2
    exit 1
fi
expect_failure_reason validation_failed
[[ "$(wc -l <"$launch_log")" == 3 ]]
[[ "$(wc -l <"$case_root/artifacts/issue038/rack-benchmark.raw.jsonl")" == 6 ]]
[[ ! -e "$case_root/artifacts/issue038/rack-benchmark.accepted.jsonl" ]]

new_case pipe-failure
if MISO_ENGINE_TEST_RUSTC_PIPE_FAIL=1 run_case success >/dev/null 2>&1; then
    printf 'runner swallowed metadata pipeline failure\n' >&2
    exit 1
fi
expect_failure_reason metadata_failed
[[ ! -e "$launch_log" ]]

new_case interruption
if run_case interrupt >/dev/null 2>&1; then
    printf 'runner swallowed interruption\n' >&2
    exit 1
fi
expect_failure_reason interrupted
[[ "$(wc -l <"$launch_log")" == 1 ]]
[[ ! -e "$case_root/artifacts/issue038/rack-benchmark.accepted.jsonl" ]]

new_case success
accepted_path=$(run_case success)
raw_path="$case_root/artifacts/issue038/rack-benchmark.raw.jsonl"
[[ "$accepted_path" == "$case_root/artifacts/issue038/rack-benchmark.accepted.jsonl" ]]
cmp -s "$raw_path" "$accepted_path"
[[ "$(wc -l <"$launch_log")" == 3 ]]
raw_sha=$(sha256sum "$raw_path" | awk '{print $1}')
accepted_sha=$(sha256sum "$accepted_path" | awk '{print $1}')
jq -e --arg raw "$raw_sha" --arg accepted "$accepted_sha" '
    .status == "PASS" and .reason == "complete" and
    .runner_invocations == 1 and .workload_process_launches == 3 and
    .warmup_launches == 1 and .measured_rounds_completed == 2 and
    .raw_sha256 == $raw and .accepted_sha256 == $accepted and .raw_sha256 == .accepted_sha256
' "$case_root/artifacts/issue038/rack-benchmark.disposition.json" >/dev/null
if run_case success >/dev/null 2>&1; then
    printf 'runner resumed or overwrote a completed lifecycle\n' >&2
    exit 1
fi
[[ "$(wc -l <"$launch_log")" == 3 ]]

rg -q 'set -euo pipefail' "$runner"
rg -q 'set -o noclobber' "$runner"
[[ "$(rg -c '^run_round (warmup|1|2) ' "$runner")" == 3 ]]
[[ "$(rg -c '^run_round [12] ' "$runner")" == 2 ]]
! rg -n 'retry|resume|eval|source ' "$runner" >/dev/null
printf 'rack benchmark validators/lifecycle: PASS (audio workload launches: 0)\n'
