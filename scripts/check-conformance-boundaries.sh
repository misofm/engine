#!/usr/bin/env bash
# Enforce the issue-002 oracle boundary: production code cannot use the f64 reference/harness.
set -euo pipefail
cd "${1:-.}"

if rg -q '^\[dependencies\]' crates/miso-engine-dsp-reference/Cargo.toml; then
    printf 'conformance boundary failure: f64 reference must have zero dependencies\n' >&2
    exit 1
fi

if rg -n 'miso-engine-(dsp-reference|conformance)' crates/miso-engine-{core,session,protocol,capi,target-smoke,effect-contract,effect-compiler,effect-package,lane,math} hosts; then
    printf 'conformance boundary failure: production crates/hosts must not depend on harness crates\n' >&2
    exit 1
fi

if rg -n '^use[[:space:]]+miso_engine_' crates/miso-engine-dsp-reference/src; then
    printf 'conformance boundary failure: reference must not call a production kernel\n' >&2
    exit 1
fi

dependency_names() {
    awk '
        /^\[dependencies\]$/ { dependencies = 1; next }
        /^\[/ { dependencies = 0 }
        dependencies && /^[A-Za-z0-9_-]+(\.workspace)?[[:space:]]*=/ {
            name = $0
            sub(/[[:space:]]*=.*/, "", name)
            sub(/\.workspace$/, "", name)
            print name
        }
    ' "$1" | sort
}

expected_conformance=$'miso-engine-core\nmiso-engine-dsp-reference\nmiso-engine-effect-contract'
[[ "$(dependency_names crates/miso-engine-conformance/Cargo.toml)" == "$expected_conformance" ]] || {
    printf 'conformance boundary failure: conformance dependencies changed\n' >&2
    exit 1
}
[[ "$(dependency_names tools/miso-engine-conformance-bench/Cargo.toml)" == 'miso-engine-conformance' ]] || {
    printf 'conformance boundary failure: benchmark may depend only on conformance\n' >&2
    exit 1
}

# The workspace lock may contain dependencies introduced by later issues (issue 003 adds Loom as a
# test-only race model). The exact manifest allowlists above are the durable issue-002 boundary;
# treating the global lockfile as issue-002-owned would make that boundary reject unrelated work.

printf 'conformance boundaries: ok\n'
