#!/usr/bin/env bash
# Enforce the issue-002 oracle boundary: production code cannot use the f64 reference/harness.
set -euo pipefail
cd "${1:-.}"

if rg -q '^\[dependencies\]' crates/miso-engine-dsp-reference/Cargo.toml; then
    printf 'conformance boundary failure: f64 reference must have zero dependencies\n' >&2
    exit 1
fi

# Manifests carry the package name (hyphens); code carries the crate identifier (underscores).
# Scoped to `Cargo.toml` and `src/` rather than the whole crate directory: a `tests/MUTATIONS.md`
# that *names* the harness while recording a red mutation is evidence, not a dependency, and a
# test target that uses the harness is exactly what issue #95's eval E6 requires. Both forms are
# checked, so this is stricter about production code than the directory scan it replaces.
production_crates=(core session protocol capi target-smoke effect-contract effect-compiler effect-package lane math)
for production in "${production_crates[@]}"; do
    manifest="crates/miso-engine-$production/Cargo.toml"
    [[ -f "$manifest" ]] || continue
    if rg -n 'miso-engine-(dsp-reference|conformance)' "$manifest"; then
        printf 'conformance boundary failure: %s must not depend on a harness crate\n' \
            "$manifest" >&2
        exit 1
    fi
    # Comment lines are excluded: naming the independent f64 oracle in a doc comment is how a
    # kernel cites what it was derived from, which is required evidence, not a dependency.
    if { rg -n 'miso_engine_(dsp_reference|conformance)' "crates/miso-engine-$production/src" 2>/dev/null || true; } |
        rg -v ':[0-9]+:[[:space:]]*//'; then
        printf 'conformance boundary failure: %s production code must not use a harness crate\n' \
            "crates/miso-engine-$production" >&2
        exit 1
    fi
done
if rg -n 'miso-engine-(dsp-reference|conformance)|miso_engine_(dsp_reference|conformance)' hosts; then
    printf 'conformance boundary failure: hosts must not depend on harness crates\n' >&2
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

# #84 phase A: conformance drives lane-generic effect checks, so the Lane trait is in-boundary.
expected_conformance=$'miso-engine-core\nmiso-engine-dsp-reference\nmiso-engine-effect-contract\nmiso-engine-lane'
[[ "$(dependency_names crates/miso-engine-conformance/Cargo.toml)" == "$expected_conformance" ]] || {
    printf 'conformance boundary failure: conformance dependencies changed\n' >&2
    exit 1
}
expected_conformance_bench=$'miso-engine-bench-support\nmiso-engine-conformance'
[[ "$(dependency_names tools/miso-engine-conformance-bench/Cargo.toml)" == "$expected_conformance_bench" ]] || {
    printf 'conformance boundary failure: benchmark may depend only on conformance and the shared bench harness\n' >&2
    exit 1
}

# The workspace lock may contain dependencies introduced by later issues (issue 003 adds Loom as a
# test-only race model). The exact manifest allowlists above are the durable issue-002 boundary;
# treating the global lockfile as issue-002-owned would make that boundary reject unrelated work.

printf 'conformance boundaries: ok\n'
