#!/usr/bin/env bash
# Enforce the issue-002 oracle boundary: production code cannot use the f64 reference/harness.
set -euo pipefail
cd "${1:-.}"
script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_directory/lib/gate.sh"
GATE_FAILURE_PREFIX='conformance boundary failure'

if gate_scan_collect 'reference dependency heading scan' '^\[dependencies\]' '' crates/dsp-reference/Cargo.toml >/dev/null; then :; else rc=$?; exit "$rc"; fi
if rg -q '^\[dependencies\]' crates/dsp-reference/Cargo.toml; then
    printf 'conformance boundary failure: f64 reference must have zero dependencies\n' >&2
    exit 1
fi

# The prefix-strip rename (docs/rulings/prefix-strip-inventory.md) retired the shared
# `miso-engine-`/`miso_engine_` literal that used to make "is this a harness/production crate"
# a cheap substring test. There is no shared prefix left, so both the crate directories and the
# `use` scan below are driven off the workspace's own manifests instead of a hand-maintained
# string -- a hardcoded list here would just be a new instance of the same staleness hazard.
workspace_crate_dir() {
    local crate="$1" candidate
    for candidate in crates hosts tools sidecars; do
        [[ -d "$candidate/$crate" ]] && { printf '%s\n' "$candidate/$crate"; return; }
    done
    return 1
}

workspace_lib_names() {
    local manifests manifest found rc names=''
    manifests="$(gate_find_collect 'workspace library manifest discovery' crates hosts tools sidecars -name Cargo.toml -type f)" || return $?
    [[ -n "$manifests" ]] || { gate_fail 'workspace library manifest discovery returned no manifests'; return 1; }
    manifests="$(gate_sort_lines 'workspace library manifest discovery' "$manifests")" || return $?
    while IFS= read -r manifest; do
        if found="$(awk '
            /^\[lib\]$/ { in_lib = 1; next }
            /^\[/ { in_lib = 0 }
            in_lib && /^name[[:space:]]*=/ {
                value = $0
                sub(/^name[[:space:]]*=[[:space:]]*"/, "", value)
                sub(/".*/, "", value)
                print value
                exit
            }
        ' "$manifest")"; then rc=0; else rc=$?; fi
        [[ "$rc" == 0 ]] || { gate_fail "library name extraction failed for $manifest (awk status $rc)"; return "$rc"; }
        [[ -z "$found" ]] || names+="$found"$'\n'
    done <<<"$manifests"
    gate_sort_lines 'workspace library name aggregation' "$names" | awk 'NF && !seen[$0]++'
}

# Manifests carry the package name (hyphens); code carries the crate identifier (underscores).
# Scoped to `Cargo.toml` and `src/` rather than the whole crate directory: a `tests/MUTATIONS.md`
# that *names* the harness while recording a red mutation is evidence, not a dependency, and a
# test target that uses the harness is exactly what issue #95's eval E6 requires. Both forms are
# checked, so this is stricter about production code than the directory scan it replaces.
production_crates=(engine session protocol capi target-smoke effect-contract effect-compiler effect-package lane math)
for production in "${production_crates[@]}"; do
    crate_dir="$(workspace_crate_dir "$production")" || {
        printf 'conformance boundary failure: no crate directory found for %s\n' "$production" >&2
        exit 1
    }
    manifest="$crate_dir/Cargo.toml"
    [[ -f "$manifest" ]] || { printf 'conformance boundary failure: missing manifest for %s\n' "$production" >&2; exit 1; }
    [[ -r "$crate_dir/src" ]] || { printf 'conformance boundary failure: unreadable source root for %s\n' "$production" >&2; exit 1; }
    if rg -n '^(dsp-reference|conformance)([[:space:]]|\.workspace)' "$manifest"; then
        printf 'conformance boundary failure: %s must not depend on a harness crate\n' \
            "$manifest" >&2
        exit 1
    fi
    # Comment lines are excluded: naming the independent f64 oracle in a doc comment is how a
    # kernel cites what it was derived from, which is required evidence, not a dependency. A
    # harness name that is *also* one of this crate's own local module names (protocol's own
    # `mod conformance` wire feature, unrelated to the harness crate) is excluded too: the
    # manifest check above already forbids the harness as an actual dependency, so a bare
    # `conformance::`/`dsp_reference::` in a crate that does not depend on it can only be that
    # crate's own same-named item, not the external crate -- Rust would refuse to compile the
    # extern-crate reading without a declared dependency.
    harness_names=(dsp_reference conformance)
    filtered_harness_names=()
    for harness_name in "${harness_names[@]}"; do
        module_probe="$(gate_scan_collect "${production} ${harness_name} module probe" \
            "^[[:space:]]*(pub(\\([^)]*\\))?[[:space:]]+)?mod[[:space:]]+${harness_name}\\b" '*.rs' "$crate_dir/src")" || exit $?
        if [[ -n "$module_probe" ]]; then
            continue
        fi
        filtered_harness_names+=("$harness_name")
    done
    harness_pattern="$(IFS='|'; printf '%s' "${filtered_harness_names[*]}")"
    if [[ -n "$harness_pattern" ]]; then
        harness_uses="$(gate_scan_collect "${production} harness use scan" "\\b(${harness_pattern})::" '' "$crate_dir/src")" || exit $?
        filtered_uses="$(gate_filter_exclude "${production} harness comment filter" ':[0-9]+:[[:space:]]*//' "$harness_uses")" || exit $?
    else
        filtered_uses=''
    fi
    if [[ -n "$filtered_uses" ]]; then
        printf 'conformance boundary failure: %s production code must not use a harness crate\n' \
            "$crate_dir" >&2
        exit 1
    fi
done
# Captured rather than gated directly on rg's own exit code: `rg` exits 2 (not just the usual
# 0/1) when a search root does not exist, e.g. a hermetic fixture with no sidecars/, and a bare
# `if rg ...; then` reads that the same as "no match" instead of "the scan could not run".
[[ -d hosts && -d sidecars ]] || { printf 'conformance boundary failure: required hosts/sidecars roots missing\n' >&2; exit 1; }
harness_matches="$(gate_scan_collect 'hosts/sidecars harness scan' '\b(dsp-reference|dsp_reference|conformance)\b' '' hosts sidecars)" || exit $?
if [[ -n "$harness_matches" ]]; then
    printf '%s\n' "$harness_matches" >&2
    printf 'conformance boundary failure: hosts/sidecars must not depend on harness crates\n' >&2
    exit 1
fi

# The f64 reference must not `use` any workspace production crate at all -- not just the
# once-shared `miso_engine_` stem, which no longer exists as a single substring to key on.
# `dsp-reference` and `conformance` are excluded: the former is itself, the latter is the harness
# that is allowed to depend on the reference (never the reverse), so neither belongs in a list of
# crates the reference must not call.
library_names="$(workspace_lib_names)" || exit $?
[[ -n "$library_names" ]] || { printf 'conformance boundary failure: no workspace library names found\n' >&2; exit 1; }
forbidden_uses="$(gate_filter_exclude 'reference library-name filter' '^(dsp_reference|conformance)$' "$library_names")" || exit $?
forbidden_uses="$(printf '%s\n' "$forbidden_uses" | paste -sd '|' -)"
[[ -n "$forbidden_uses" ]] || { printf 'conformance boundary failure: no production library names found\n' >&2; exit 1; }
gate_scan_forbidden 'reference production use scan' "^use[[:space:]]+($forbidden_uses)\\b" '' crates/dsp-reference/src || exit $?

dependency_names() { gate_toml_dependencies "$1" plain-target; }

# #84 phase A: conformance drives lane-generic effect checks, so the Lane trait is in-boundary.
# Sorted alphabetically by the *current* (post-prefix-strip) name -- `engine` (formerly `core`,
# which sorted first under the old miso-engine- prefix) now sorts third, not first.
expected_conformance=$'dsp-reference\neffect-contract\nengine\nlane'
[[ "$(dependency_names crates/conformance/Cargo.toml)" == "$expected_conformance" ]] || {
    printf 'conformance boundary failure: conformance dependencies changed\n' >&2
    exit 1
}
expected_conformance_bench=$'bench-support\nbuiltins\nbuiltins-compiler\nconformance\nconsole-workload\neffect-compiler\neffect-contract\neffect-package\nengine\nflatbuffers\ngraph\ngraph-compiler\nlane\nprotocol\nrack\nsession\nsha2'
[[ "$(dependency_names tools/bench/Cargo.toml)" == "$expected_conformance_bench" ]] || {
    printf 'conformance boundary failure: consolidated benchmark dependency union changed\n' >&2
    exit 1
}

# The workspace lock may contain dependencies introduced by later issues (issue 003 adds Loom as a
# test-only race model). The exact manifest allowlists above are the durable issue-002 boundary;
# treating the global lockfile as issue-002-owned would make that boundary reject unrelated work.

printf 'conformance boundaries: ok\n'
