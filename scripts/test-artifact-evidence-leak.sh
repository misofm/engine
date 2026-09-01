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
sed -i 's|-p host-web -p lane|-p host-web -p conformance -p lane|' \
    "$case_root/.github/workflows/ci.yml"
expect_failure conformance-back-in-the-scalar-wasm-artifact

# 2. RETIRED by #66, which removed the android and ios compile-only jobs. The mutation this case
#    applied — the f64 oracle back in the mobile check — has no surface left to land on: its `sed`
#    matches nothing, so the gate stayed green and the case reported "mutation escaped" rather than
#    catching anything. Retired rather than repaired because the coverage it guarded was itself
#    deliberately dropped by #66 (browser Wasm is now the mobile portability target). If a mobile
#    compile job ever returns, this case must return with it.

# 3. Removing the evidence crates from an artifact list without keeping their cross-target compile
#    coverage is the other way to break this: the gate would go green while the wasm32 build of the
#    oracle stopped being checked at all.
new_case wasm-compile-coverage-deleted
sed -i '/Evidence crates compile for Wasm/,+3d' "$case_root/.github/workflows/ci.yml"
expect_failure wasm-compile-coverage-deleted

# (The iOS counterpart of case 3 retired with #66 for the same reason; the Wasm case below is what
#  still pins this half of the gate.)

# 4. RETIRED by #66 for the same reason as case 2: this mutated the android coverage invocation,
#    which no longer exists in ci.yml.

printf 'artifact evidence gate mutations: ok\n'
