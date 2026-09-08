#!/usr/bin/env bash
# Enforce the master plan #83 lane boundary: fusion, SIMD vocabulary and raw architecture
# intrinsics exist in exactly one crate, and that crate's dependency surface is pinned.
#
# D3: fusion exists only where `Lane::fma` is written. Rust never contracts `a * b + c`, so the
# rule is mechanical: `mul_add` and the fused intrinsics may appear only inside the lane crate.
# D4: `wide` is the lane crate's private vocabulary; nothing else may name it, and no runtime SIMD
# dispatch may reappear. D8: `max`/`min` are the select form, never a library `max`/`min`.
set -euo pipefail

workspace_root="${1:-.}"
script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GATE_FAILURE_PREFIX='lane policy failure'
source "$script_directory/lib/gate.sh"
cd "$workspace_root"

fail() {
    printf 'lane policy failure: %s\n' "$1" >&2
    exit 1
}

policy_file="${LANE_POLICY_FILE:-$script_directory/policies/lane-source.toml}"
rule_loader="${LANE_RULE_LOADER:-$script_directory/lib/gate-rules.py}"
if rule_output="$(python3 "$rule_loader" "$policy_file" 2>&1)"; then
    loader_status=0
else
    loader_status=$?
fi
[[ $loader_status == 0 ]] || {
    printf '%s\n' "$rule_output" >&2
    fail "lane source rule loader failed"
}

decode_rule_field() {
    local encoded="$1" decoded
    if decoded="$(printf '%s' "$encoded" | base64 --decode 2>&1)"; then
        printf '%s' "$decoded"
    else
        fail "lane source rule loader output is invalid"
    fi
}

rule_index=0
while IFS='|' read -r kind id_encoded scan_encoded pattern_encoded glob_encoded \
    root1_encoded root2_encoded root3_encoded root4_encoded exclude_description_encoded \
    exclude_regex_encoded failure_encoded extra; do
    [[ -n "$kind" && -z "$extra" ]] || fail "lane source rule loader output is invalid"
    [[ "$kind" == RULE ]] || fail "lane source rule loader output is invalid"
    rule_index=$((rule_index + 1))
    id="$(decode_rule_field "$id_encoded")"
    scan_description="$(decode_rule_field "$scan_encoded")"
    pattern="$(decode_rule_field "$pattern_encoded")"
    glob="$(decode_rule_field "$glob_encoded")"
    roots=(
        "$(decode_rule_field "$root1_encoded")"
        "$(decode_rule_field "$root2_encoded")"
        "$(decode_rule_field "$root3_encoded")"
        "$(decode_rule_field "$root4_encoded")"
    )
    exclude_description="$(decode_rule_field "$exclude_description_encoded")"
    exclude_regex="$(decode_rule_field "$exclude_regex_encoded")"
    failure_diagnostic="$(decode_rule_field "$failure_encoded")"
    [[ -n "$id" && -n "$scan_description" && -n "$pattern" && -n "$glob" ]] ||
        fail "lane source rule loader output is invalid"
    [[ -n "${roots[0]}" && -n "${roots[1]}" && -n "${roots[2]}" && -n "${roots[3]}" ]] ||
        fail "lane source rule loader output is invalid"
    case "$rule_index:$id" in
        1:fusion|2:relaxed|3:architecture|4:detection) ;;
        *) fail "lane source rule loader output is invalid" ;;
    esac
    raw_matches="$(gate_scan_collect "$scan_description" "$pattern" "$glob" "${roots[@]}")" || exit $?
    if [[ -n "$exclude_regex" ]]; then
        matches="$(gate_filter_exclude "$exclude_description" "$exclude_regex" "$raw_matches")" || exit $?
    else
        matches="$raw_matches"
    fi
    [[ -z "$matches" ]] || {
        printf '%s\n' "$matches" >&2
        fail "$failure_diagnostic"
    }
done <<<"$rule_output"
[[ $rule_index == 4 ]] || fail "lane source rule loader output is invalid"

# Inside the lane crate, a `wide` or `std` float method whose meaning differs per target may only
# be called with an explicit `LANE-OP-OK` marker on it or in the three lines above it. `max`,
# `min` and `mul_add` are the ones that actually diverge (§3.3); the trait forms are the default.
# The marker may sit on the call or in the four lines above it, so that it can carry a reason.
marker_hits=""
lane_sources_raw="$(gate_find_collect 'lane source discovery' crates/lane/src -name '*.rs' -type f)" || exit $?
[[ -n "$lane_sources_raw" ]] || fail 'lane source discovery returned no Rust files'
lane_sources="$(gate_sort_lines 'lane source discovery' "$lane_sources_raw")" || exit $?
while IFS= read -r source; do
    if hits="$(awk -v file="$source" '
        {
            fifth = fourth
            fourth = third
            third = second
            second = first
            first = $0
        }
        /\.(max|min|fast_max|fast_min|mul_add|mul_sub|mul_neg_add|mul_neg_sub|recip|recip_sqrt|sqrt|floor|powf|exp|exp2|ln|log2|log10|sin|cos|tan|tanh)\(/ ||
        /(f32|f64)::(max|min|mul_add|sqrt|floor|powf|exp|exp2|ln|log2|log10|sin|cos|tan|tanh)\(/ {
            if (first !~ /LANE-OP-OK/ && second !~ /LANE-OP-OK/ && third !~ /LANE-OP-OK/ &&
                fourth !~ /LANE-OP-OK/ && fifth !~ /LANE-OP-OK/) {
                print file ":" FNR ":" $0
            }
        }
    ' "$source" 2>&1)"; then :; else rc=$?; printf '%s\n' "$hits" >&2; fail "lane marker-window extraction failed for $source (awk status $rc)"; fi
    [[ -z "$hits" ]] || marker_hits="$marker_hits$hits"$'\n'
done <<<"$lane_sources"
marker_hits="$(printf '%s' "$marker_hits")"
[[ -z "$marker_hits" ]] || {
    printf '%s\n' "$marker_hits" >&2
    fail "a per-target library float method inside the lane crate needs a LANE-OP-OK marker (D8)"
}

# Dependency surface. `wide` is pinned exactly, brings only `bytemuck` and `safe_arch`, and the
# lane crate depends on nothing else: the numeric contract cannot be changed by a caret upgrade.
manifest='Cargo.toml'
lockfile='Cargo.lock'
[[ -f "$manifest" && -f "$lockfile" ]] || fail "missing $manifest or $lockfile"

if pin_output="$(rg -nF 'wide = { version = "=1.6.1", default-features = false }' "$manifest" 2>&1)"; then pin_rc=0; else pin_rc=$?; fi
[[ $pin_rc == 0 ]] || { [[ $pin_rc == 1 ]] && fail "$manifest must pin wide as: wide = { version = \"=1.6.1\", default-features = false }"; printf '%s\n' "$pin_output" >&2; fail "wide manifest pin search failed (rg exit $pin_rc)"; }

locked_version() {
    awk -v package="$1" '
        $0 == "[[package]]" { name = ""; version = ""; next }
        /^name = / { name = $3; gsub(/"/, "", name) }
        /^version = / { version = $3; gsub(/"/, "", version); if (name == package) { print version } }
    ' "$lockfile"
}

locked_dependencies() {
    awk -v package="$1" '
        $0 == "[[package]]" { name = ""; inside = 0; next }
        /^name = / { name = $3; gsub(/"/, "", name) }
        /^dependencies = \[/ { if (name == package) { inside = 1 }; next }
        inside && /^\]/ { inside = 0; next }
        inside { value = $1; gsub(/[",]/, "", value); print value }
    ' "$lockfile"
}

wide_versions="$(locked_version wide)" || { rc=$?; printf '%s\n' "$wide_versions" >&2; fail "wide locked version extraction failed (awk status $rc)"; }
[[ "$wide_versions" == "1.6.1" ]] || fail "$lockfile must contain exactly one wide 1.6.1, found: ${wide_versions:-none}"

for dependency in bytemuck safe_arch; do
    versions="$(locked_version "$dependency")" || { rc=$?; printf '%s\n' "$versions" >&2; fail "$dependency locked version extraction failed (awk status $rc)"; }
    [[ -n "$versions" ]] || fail "$lockfile is missing $dependency, which wide requires"
done

# The old miso-engine- prefix used to distinguish "a workspace crate" from "an external crate"
# by naming convention alone (`wide | miso-engine-*`); the prefix-strip rename retired that
# convention (docs/rulings/prefix-strip-inventory.md), so this now checks against the real list
# of workspace crate names instead of a pattern that can no longer tell the two apart.
# Process substitution (`< <(find ...)`), not a plain pipe: under `pipefail`, a plain
# `find ... | while ...` reports find's own exit status (2 when one of the roots, e.g. a
# hermetic fixture's absent sidecars/, does not exist) as the whole pipeline's status, which
# would trip `set -e` even though the while loop itself completed and produced correct output
# from the roots that do exist.
workspace_manifests="$(gate_find_collect 'workspace manifest discovery' crates hosts tools sidecars -name Cargo.toml -type f)" || exit $?
workspace_crate_names=''
while IFS= read -r crate_manifest; do
    [[ -z "$crate_manifest" ]] && continue
    if names="$(awk '
            /^\[package\]$/ { in_package = 1; next }
            /^\[/ { in_package = 0 }
            in_package && /^name[[:space:]]*=/ {
                value = $0
                sub(/^name[[:space:]]*=[[:space:]]*"/, "", value)
                sub(/".*/, "", value)
                print value
            }
        ' "$crate_manifest" 2>&1)"; then :; else rc=$?; printf '%s\n' "$names" >&2; fail "workspace package-name extraction failed for $crate_manifest (awk status $rc)"; fi
    [[ -z "$names" ]] || workspace_crate_names+="$names"$'\n'
done <<<"$workspace_manifests"
workspace_crate_names="${workspace_crate_names%$'\n'}"
[[ -n "$workspace_crate_names" ]] || fail 'workspace package-name extraction returned no package names'
lane_dependencies="$(locked_dependencies lane)" || { rc=$?; printf '%s\n' "$lane_dependencies" >&2; fail "lane locked dependency extraction failed (awk status $rc)"; }
while IFS= read -r dependency; do
    [[ -n "$dependency" ]] || continue
    [[ "$dependency" == wide ]] && continue
    if membership="$(rg -nx -- "$dependency" <<<"$workspace_crate_names" 2>&1)"; then membership_rc=0; else membership_rc=$?; fi
    if [[ $membership_rc == 0 ]]; then
        continue
    fi
    [[ $membership_rc == 1 ]] || { printf '%s\n' "$membership" >&2; fail "workspace dependency membership search failed for $dependency (rg exit $membership_rc)"; }
    fail "crates/lane may depend only on wide and workspace crates, found $dependency"
done <<<"$lane_dependencies"

wide_dependencies="$(locked_dependencies wide)" || { rc=$?; printf '%s\n' "$wide_dependencies" >&2; fail "wide locked dependency extraction failed (awk status $rc)"; }
while IFS= read -r dependency; do
    [[ -n "$dependency" ]] || continue
    case "$dependency" in
        bytemuck | safe_arch) ;;
        *) fail "wide must pull only bytemuck and safe_arch, found $dependency" ;;
    esac
done <<<"$wide_dependencies"

printf 'lane policy: ok\n'
