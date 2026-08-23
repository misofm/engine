#!/usr/bin/env bash
# Hermetic mutations for the Issue-114 qualification checker.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf -- "$scratch"' EXIT

copy_case() {
    case_root="$scratch/$1"
    mkdir -p "$case_root/fixtures/capi-qualification/v1" "$case_root/scripts"
    cp -R "$root/fixtures/capi-qualification/v1/." \
        "$case_root/fixtures/capi-qualification/v1/"
    cp "$root/scripts/check-capi-qualification-v1.sh" "$case_root/scripts/"
    cp "$root/scripts/check-capi-object-symbols-v1.py" "$case_root/scripts/"
    cp "$root/scripts/check-capi-qualification-evidence-v1.py" "$case_root/scripts/"
    cp "$root/scripts/run-capi-qualification-v1.sh" "$case_root/scripts/"
    while read -r _ path; do
        mkdir -p "$case_root/$(dirname "$path")"
        cp "$root/$path" "$case_root/$path"
    done <"$root/fixtures/capi-qualification/v1/AUTHORITIES.sha256"
}

refresh_evidence() {
    changed="$1"
    relative="${changed#"$case_root/"}"
    replacement="$(sha256sum "$changed" | awk '{print $1}')"
    sed -i -E "s|^[0-9a-f]{64}  $relative$|$replacement  $relative|" \
        "$case_root/fixtures/capi-qualification/v1/EVIDENCE.sha256"
}

expect_failure() {
    name="$1"
    shift
    if "$@" >/dev/null 2>&1; then
        printf 'CAPI qualification mutation escaped: %s\n' "$name" >&2
        exit 1
    fi
}

copy_case baseline
bash "$case_root/scripts/check-capi-qualification-v1.sh" "$case_root" preflight >/dev/null

copy_case authority
printf mutation >>"$case_root/crates/miso-engine-capi/include/miso_engine_v2.h"
expect_failure authority-drift bash "$case_root/scripts/check-capi-qualification-v1.sh" \
    "$case_root" preflight

copy_case symbol
printf '%s\n' miso_engine_v2_extra >>"$case_root/fixtures/capi-qualification/v1/EXPECTED_SYMBOLS.txt"
expect_failure extra-symbol bash "$case_root/scripts/check-capi-qualification-v1.sh" \
    "$case_root" preflight

copy_case missing_symbol
sed -i '/miso_engine_v2_abi_version/d' \
    "$case_root/fixtures/capi-qualification/v1/EXPECTED_SYMBOLS.txt"
expect_failure missing-symbol bash "$case_root/scripts/check-capi-qualification-v1.sh" \
    "$case_root" preflight

copy_case target
sed -i '/target\taarch64-linux-android/d' \
    "$case_root/fixtures/capi-qualification/v1/TOOLCHAINS.tsv"
expect_failure target-omission bash "$case_root/scripts/check-capi-qualification-v1.sh" \
    "$case_root" preflight

copy_case fabricated
sed -i 's/tool\txcrun\tABSENT/tool\txcrun\tPRESENT/' \
    "$case_root/fixtures/capi-qualification/v1/TOOLCHAINS.tsv"
expect_failure fabricated-tool bash "$case_root/scripts/check-capi-qualification-v1.sh" \
    "$case_root" preflight

copy_case fabricated_unavailable
sed -i 's|tool\tcc\tPRESENT|tool\tcc\tABSENT|' \
    "$case_root/fixtures/capi-qualification/v1/TOOLCHAINS.tsv"
expect_failure fabricated-unavailable bash "$case_root/scripts/check-capi-qualification-v1.sh" \
    "$case_root" preflight

copy_case timer
printf '%s\n' 'elapsed()' >>"$case_root/fixtures/capi-qualification/v1/runtime_consumer.c"
expect_failure timer-surface bash "$case_root/scripts/check-capi-qualification-v1.sh" \
    "$case_root" preflight

copy_case stale_stage
mkdir -p "$case_root/target/capi-qualification/v1"
expect_failure stale-staging bash "$case_root/scripts/run-capi-qualification-v1.sh" "$case_root"

copy_case source_artifact
touch "$case_root/fixtures/capi-qualification/v1/stale.so"
expect_failure generated-source-artifact bash \
    "$case_root/scripts/check-capi-qualification-v1.sh" "$case_root" final

copy_case artifact_evidence
sed -i 's/linux-x86_64-static-library\t33321950/linux-x86_64-static-library\t33321951/' \
    "$case_root/fixtures/capi-qualification/v1/ARTIFACTS.tsv"
refresh_evidence "$case_root/fixtures/capi-qualification/v1/ARTIFACTS.tsv"
expect_failure correlated-artifact-evidence bash \
    "$case_root/scripts/check-capi-qualification-v1.sh" "$case_root" final

copy_case symbol_evidence
sed -i 's/static-defined-count\t14/static-defined-count\t13/' \
    "$case_root/fixtures/capi-qualification/v1/SYMBOLS.tsv"
refresh_evidence "$case_root/fixtures/capi-qualification/v1/SYMBOLS.tsv"
expect_failure correlated-symbol-evidence bash \
    "$case_root/scripts/check-capi-qualification-v1.sh" "$case_root" final

copy_case audit_evidence
sed -i 's/"calls":100000/"calls":99999/' \
    "$case_root/fixtures/capi-qualification/v1/AUDITS.jsonl"
refresh_evidence "$case_root/fixtures/capi-qualification/v1/AUDITS.jsonl"
expect_failure correlated-audit-evidence bash \
    "$case_root/scripts/check-capi-qualification-v1.sh" "$case_root" final

copy_case count_evidence
sed -i 's/runner_tests\t18/runner_tests\t17/' \
    "$case_root/fixtures/capi-qualification/v1/QUALIFICATION.tsv"
refresh_evidence "$case_root/fixtures/capi-qualification/v1/QUALIFICATION.tsv"
expect_failure correlated-count-evidence bash \
    "$case_root/scripts/check-capi-qualification-v1.sh" "$case_root" final

copy_case prohibited_count
sed -i 's/benchmark_invocations\t0/benchmark_invocations\t1/' \
    "$case_root/fixtures/capi-qualification/v1/QUALIFICATION.tsv"
refresh_evidence "$case_root/fixtures/capi-qualification/v1/QUALIFICATION.tsv"
expect_failure correlated-prohibited-count bash \
    "$case_root/scripts/check-capi-qualification-v1.sh" "$case_root" final

copy_case consumer_evidence
sed -i '/result\tlinux-c11-static/s/\t0\tPASS/\t1\tPASS/' \
    "$case_root/fixtures/capi-qualification/v1/CONSUMER_RESULTS.tsv"
refresh_evidence "$case_root/fixtures/capi-qualification/v1/CONSUMER_RESULTS.tsv"
expect_failure correlated-consumer-evidence bash \
    "$case_root/scripts/check-capi-qualification-v1.sh" "$case_root" final

copy_case raw_evidence
sed -i '/logs\/runner-corpus.log/s/ed2dd1/0d2dd1/' \
    "$case_root/fixtures/capi-qualification/v1/RAW_EVIDENCE.tsv"
refresh_evidence "$case_root/fixtures/capi-qualification/v1/RAW_EVIDENCE.tsv"
expect_failure correlated-raw-evidence bash \
    "$case_root/scripts/check-capi-qualification-v1.sh" "$case_root" final

copy_case gate_evidence
sed -i '/gate\tcargo-fmt/s/\tPASS\t/\tFAIL\t/' \
    "$case_root/fixtures/capi-qualification/v1/GATES.tsv"
refresh_evidence "$case_root/fixtures/capi-qualification/v1/GATES.tsv"
expect_failure correlated-gate-evidence bash \
    "$case_root/scripts/check-capi-qualification-v1.sh" "$case_root" final

copy_case result_evidence
sed -i '/linux-c11-static/s/\tPASS\t/\tFAIL\t/' \
    "$case_root/fixtures/capi-qualification/v1/MATRIX.tsv"
refresh_evidence "$case_root/fixtures/capi-qualification/v1/MATRIX.tsv"
expect_failure correlated-result-evidence bash \
    "$case_root/scripts/check-capi-qualification-v1.sh" "$case_root" final

symbol_scratch="$scratch/object-parser"
mkdir -p "$symbol_scratch"
awk '{print "00000000 T " $0}' "$root/fixtures/capi-qualification/v1/EXPECTED_SYMBOLS.txt" \
    >"$symbol_scratch/static.nm"
cp "$symbol_scratch/static.nm" "$symbol_scratch/shared.nm"
python3 -I -B "$root/scripts/check-capi-object-symbols-v1.py" \
    "$root/fixtures/capi-qualification/v1/EXPECTED_SYMBOLS.txt" \
    "$symbol_scratch/static.nm" "$symbol_scratch/shared.nm"
sed -i 's/00000000 T miso_engine_v2_abi_version/         U miso_engine_v2_abi_version/' \
    "$symbol_scratch/static.nm"
expect_failure undefined-reference-is-not-export python3 -I -B \
    "$root/scripts/check-capi-object-symbols-v1.py" \
    "$root/fixtures/capi-qualification/v1/EXPECTED_SYMBOLS.txt" \
    "$symbol_scratch/static.nm" "$symbol_scratch/shared.nm"

printf 'CAPI qualification V1 policy mutations: ok\n'
