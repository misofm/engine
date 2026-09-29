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
    cp "$root/.github/workflows/qualification.yml" "$case_root/.github/workflows/"
    cp "$root/scripts/check-artifact-evidence-leak.sh" "$case_root/scripts/"
    cp "$root/scripts/check-cross-targets.sh" "$case_root/scripts/"
    cp "$root/scripts/build-web-audioworklet.sh" "$case_root/scripts/"
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

# design #359 §12 stage 3: qualification.yml is now the gate's one mandatory scan target (ci.yml
# and its own conditional qualification.yml cases were retired along with the workflow itself).
# Its absence must be a hard failure, not a skip.
new_case missing-qualification
rm "$case_root/.github/workflows/qualification.yml"
expect_failure missing-qualification

# 1. The exact regression this gate exists for: conformance back in the shipped wasm invocation.
#    Since #1062 retired the workflow's scalar wasm build, that invocation is the delivery script's.
new_case conformance-back-in-the-shipped-wasm-artifact
sed -i 's|--target wasm32-unknown-unknown -p host-web$|--target wasm32-unknown-unknown -p host-web -p conformance|' \
    "$case_root/scripts/build-web-audioworklet.sh"
expect_failure conformance-back-in-the-shipped-wasm-artifact

# 1b. The delivery script is what makes wasm32 an artifact target at all (#1062): with its cargo
#     invocation gone, the gate has no wasm32 artifact invocation to gate and must say so rather
#     than pass vacuously.
new_case shipped-wasm-artifact-invocation-gone
sed -i '/--target wasm32-unknown-unknown -p host-web$/d' "$case_root/scripts/build-web-audioworklet.sh"
expect_failure shipped-wasm-artifact-invocation-gone

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
sed -i '/Evidence crates compile for Wasm/,+2d' "$case_root/.github/workflows/qualification.yml"
expect_failure wasm-compile-coverage-deleted

# (The iOS counterpart of case 3 retired with #66 for the same reason; the Wasm case above is what
#  still pins this half of the gate.)

# 4. RETIRED by #66 for the same reason as case 2: this mutated the android coverage invocation,
#    which no longer existed even in the retired ci.yml.

# 5. RETIRED by design #359 §12 stage 3: ci.yml's cases above (1 and 3) used to be duplicated here
#    against qualification.yml, conditionally, while both workflows carried the same wasm-guests
#    contract during the stage-1/stage-2 dual-run window. Now that qualification.yml is the only
#    workflow this gate scans, cases 1 and 3 above already cover it directly and the duplicate is
#    gone.

# 6. N1: scripts/check-cross-targets.sh mixes the shipped crate effect-compiler with the evidence
#    crate conformance in one invocation (the exact regression the split under N1 fixed; the row
#    named effect-package too until #1037).
new_case cross-targets-script-mixes-effect-compiler-with-conformance
sed -i 's|^\( *\)-p effect-compiler$|\1-p effect-compiler -p conformance|' \
    "$case_root/scripts/check-cross-targets.sh"
expect_failure cross-targets-script-mixes-effect-compiler-with-conformance

# 7. The same regression, but in a workflow YAML cargo line rather than check-cross-targets.sh --
#    proves effect-compiler's membership in `shipped` is enforced wherever a cross-target
#    invocation names it, not only inside the script this gate was extended to scan: the mutated
#    line names no host, so only effect-compiler makes it an artifact invocation. (#1062: the
#    evidence-crate line is the workflow's wasm32 line left to mutate.)
new_case workflow-mixes-effect-compiler-with-conformance
sed -i 's|--target wasm32-unknown-unknown -p dsp-reference -p conformance|--target wasm32-unknown-unknown -p effect-compiler -p dsp-reference -p conformance|' \
    "$case_root/.github/workflows/qualification.yml"
expect_failure workflow-mixes-effect-compiler-with-conformance

printf 'artifact evidence gate mutations: ok\n'
