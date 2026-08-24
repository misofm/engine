#!/usr/bin/env bash
# Mutations proving check-artifact-evidence-leak.sh discriminates (#105 phase 2 C2).
set -euo pipefail

root="$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf -- "$scratch"' EXIT

case_root=""
new_case() {
    case_root="$scratch/$1"
    mkdir -p "$case_root/.github/workflows" "$case_root/scripts"
    cp "$root/.github/workflows/ci.yml" "$case_root/.github/workflows/"
    cp "$root/scripts/check-artifact-evidence-leak.sh" "$case_root/scripts/"
}

check() { bash "$case_root/scripts/check-artifact-evidence-leak.sh" "$case_root"; }

expect_failure() {
    if check >/dev/null 2>&1; then
        printf 'test-artifact-evidence-leak: mutation escaped: %s\n' "$1" >&2
        exit 1
    fi
    printf 'test-artifact-evidence-leak: red as required: %s\n' "$1"
}

new_case baseline
check >/dev/null || { printf 'test-artifact-evidence-leak: baseline is red\n' >&2; exit 1; }

# 1. The exact regression this gate exists for: conformance back in the shipped wasm invocation.
new_case conformance-back-in-the-scalar-wasm-artifact
sed -i 's|-p miso-engine-host-web -p miso-engine-lane|-p miso-engine-host-web -p miso-engine-conformance -p miso-engine-lane|' \
    "$case_root/.github/workflows/ci.yml"
expect_failure conformance-back-in-the-scalar-wasm-artifact

# 2. The f64 oracle back in the mobile check.
new_case dsp-reference-back-in-the-android-check
sed -i 's|-p miso-engine-host-mobile -p miso-engine-lane|-p miso-engine-host-mobile -p miso-engine-dsp-reference -p miso-engine-lane|' \
    "$case_root/.github/workflows/ci.yml"
expect_failure dsp-reference-back-in-the-android-check

# 3. Removing the evidence crates from an artifact list without keeping their cross-target compile
#    coverage is the other way to break this: the gate would go green while the wasm32 build of the
#    oracle stopped being checked at all.
new_case wasm-compile-coverage-deleted
sed -i '/Evidence crates compile for Wasm/,+3d' "$case_root/.github/workflows/ci.yml"
expect_failure wasm-compile-coverage-deleted

new_case ios-compile-coverage-deleted
sed -i '/Evidence crates compile for iOS/,+1d' "$case_root/.github/workflows/ci.yml"
expect_failure ios-compile-coverage-deleted

# 4. A coverage invocation that quietly drops one of the two evidence crates no longer counts.
new_case coverage-invocation-loses-the-oracle
sed -i 's|--target aarch64-linux-android -p miso-engine-dsp-reference -p miso-engine-conformance|--target aarch64-linux-android -p miso-engine-conformance|' \
    "$case_root/.github/workflows/ci.yml"
expect_failure coverage-invocation-loses-the-oracle

printf 'artifact evidence gate mutations: ok\n'
