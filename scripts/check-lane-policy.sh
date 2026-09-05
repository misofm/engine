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

lane_source='^crates/lane/src/'
lane_tests='^crates/lane/tests/'
# `crates/dsp-reference` is the workspace's oracle/twin crate: it is a dev dependency
# only, never links into an engine, host or artifact, and its whole job is to reproduce a frozen
# operation order -- including its single roundings -- independently of the lane crate. It is
# exempt here for the same structural reason `clippy.toml`'s `disallowed-methods` (formerly `scripts/check-math-policy.sh`) exempts it: matching
# the platform is the point, not a leak of it. The exemption is the crate, not a wildcard: a
# render path cannot reach it, because nothing in `crates/` or `hosts/` depends on it.
oracle_crate='^crates/dsp-reference/'
lane_softfma='^crates/lane/src/softfma\.rs:'
# Issue #146: the canonical floating-point environment. `core::arch::asm!` is the only way to reach
# AArch64's FPCR -- there is no stable intrinsic for it -- so this second lane file is named here
# for the same reason `softfma.rs` is, and for nothing else.
lane_fpenv='^crates/lane/src/fpenv\.rs:'
# #84 phase A deleted `crates/engine/src/arch/` and its runtime detection, so the two
# temporary exemptions that stood here are gone: there is no legacy kernel file and no second
# backend enum left to exempt.
#
# Issue #163 phase 2 made the numeric contract unfused everywhere, which leaves exactly two places
# that must still be able to *compute* a fused multiply-add in order to prove the contract is not
# one: the audit that justified the change (its bounds are measured against the arm it replaced)
# and gate G5's native leg (its `lane_fma` case would pass vacuously without a fused reference to
# contrast against). Both are evidence code on no render path.
#
# This is not a second, looser roster: `scripts/check-unfused-seal.sh` registers the same two files
# with an exact per-file call count, checks the registry in both directions, and requires an
# `UNFUSED-SEAL-EXEMPT` marker within six lines of every call. Naming them here keeps this policy's
# question ("does fused arithmetic live outside the lane crate?") answerable while that seal owns
# the harder question ("how many fused calls exist at all, and are they the ones we admitted?").
fused_evidence='^tools/audit/src/unfused_fma\.rs:|^tools/wasm-gates/tests/g5_native_corpus\.rs:'

fusion_raw="$(gate_scan_collect 'fusion source scan' 'mul_add|_mm256_fmadd|_mm256_fmsub|_mm256_fnmadd|_mm_fmadd|vfmaq|vfmsq|wide::' '*.rs' crates hosts tools sidecars)" || exit $?
fusion_matches="$(gate_filter_exclude 'fusion source exclusions' "$lane_source|$lane_tests|$oracle_crate|$fused_evidence" "$fusion_raw")" || exit $?
[[ -z "$fusion_matches" ]] || {
    printf '%s\n' "$fusion_matches" >&2
    fail "fused multiply-add and the SIMD vocabulary belong to crates/lane (D3, D4)"
}

# Relaxed SIMD is forbidden everywhere, the lane crate included: correctness must never depend on
# an instruction whose rounding the runtime is free to choose (D3). Only instruction and intrinsic
# names are scanned -- `crates/effect-package` carries `"relaxed-simd"` as a *capability
# string* describing what a third-party Wasm package declares, which is data, not engine code.
relaxed_matches="$(gate_scan_collect 'relaxed SIMD source scan' 'f32x4_relaxed|f64x2_relaxed|relaxed_madd|relaxed_nmadd|relaxed_dot|i8x16_relaxed' '*.rs' crates hosts tools sidecars)" || exit $?
[[ -z "$relaxed_matches" ]] || {
    printf '%s\n' "$relaxed_matches" >&2
    fail "relaxed SIMD is forbidden on every target (D3)"
}

architecture_raw="$(gate_scan_collect 'architecture source scan' '(core|std)::arch::' '*.rs' crates hosts tools sidecars)" || exit $?
architecture_matches="$(gate_filter_exclude 'architecture source exclusions' "$lane_softfma|$lane_fpenv" "$architecture_raw")" || exit $?
[[ -z "$architecture_matches" ]] || {
    printf '%s\n' "$architecture_matches" >&2
    fail "raw architecture intrinsics belong to crates/lane/src/{softfma,fpenv}.rs"
}

# Runtime SIMD dispatch is gone (D4, revision 4): the ISA is pinned at compile time and attested
# once at boot. `Backend::current()` is a constant, so a new detection site is a regression.
detection_raw="$(gate_scan_collect 'detection source scan' 'is_x86_feature_detected|is_aarch64_feature_detected' '*.rs' crates hosts tools sidecars)" || exit $?
detection_matches="$(gate_filter_exclude 'detection source exclusions' '^crates/lane/src/backend\.rs:|^crates/lane/src/lib\.rs:' "$detection_raw")" || exit $?
[[ -z "$detection_matches" ]] || {
    printf '%s\n' "$detection_matches" >&2
    fail "runtime SIMD detection is forbidden outside the enumerated legacy sites (D4)"
}

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
