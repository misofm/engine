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

fusion_matches="$({
    rg -n 'mul_add|_mm256_fmadd|_mm256_fmsub|_mm256_fnmadd|_mm_fmadd|vfmaq|vfmsq|wide::' \
        crates hosts tools sidecars --glob '*.rs' || true
} | rg -v "$lane_source|$lane_tests|$oracle_crate|$fused_evidence" || true)"
[[ -z "$fusion_matches" ]] || {
    printf '%s\n' "$fusion_matches" >&2
    fail "fused multiply-add and the SIMD vocabulary belong to crates/lane (D3, D4)"
}

# Relaxed SIMD is forbidden everywhere, the lane crate included: correctness must never depend on
# an instruction whose rounding the runtime is free to choose (D3). Only instruction and intrinsic
# names are scanned -- `crates/effect-package` carries `"relaxed-simd"` as a *capability
# string* describing what a third-party Wasm package declares, which is data, not engine code.
relaxed_matches="$({
    rg -n 'f32x4_relaxed|f64x2_relaxed|relaxed_madd|relaxed_nmadd|relaxed_dot|i8x16_relaxed' \
        crates hosts tools sidecars --glob '*.rs' || true
} || true)"
[[ -z "$relaxed_matches" ]] || {
    printf '%s\n' "$relaxed_matches" >&2
    fail "relaxed SIMD is forbidden on every target (D3)"
}

architecture_matches="$({
    rg -n '(core|std)::arch::' crates hosts tools sidecars --glob '*.rs' || true
} | rg -v "$lane_softfma|$lane_fpenv" || true)"
[[ -z "$architecture_matches" ]] || {
    printf '%s\n' "$architecture_matches" >&2
    fail "raw architecture intrinsics belong to crates/lane/src/{softfma,fpenv}.rs"
}

# Runtime SIMD dispatch is gone (D4, revision 4): the ISA is pinned at compile time and attested
# once at boot. `Backend::current()` is a constant, so a new detection site is a regression.
detection_matches="$({
    rg -n 'is_x86_feature_detected|is_aarch64_feature_detected' crates hosts tools sidecars --glob '*.rs' || true
} | rg -v '^crates/lane/src/backend\.rs:|^crates/lane/src/lib\.rs:' || true)"
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
lane_sources="$(gate_sort_lines 'lane source discovery' "$lane_sources_raw")" || exit $?
while IFS= read -r source; do
    hits="$(awk -v file="$source" '
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
    ' "$source")"
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

rg -qF 'wide = { version = "=1.6.1", default-features = false }' "$manifest" || {
    fail "$manifest must pin wide as: wide = { version = \"=1.6.1\", default-features = false }"
}

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

wide_versions="$(locked_version wide)"
[[ "$wide_versions" == "1.6.1" ]] || fail "$lockfile must contain exactly one wide 1.6.1, found: ${wide_versions:-none}"

for dependency in bytemuck safe_arch; do
    [[ -n "$(locked_version "$dependency")" ]] || fail "$lockfile is missing $dependency, which wide requires"
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
workspace_roots=(crates hosts tools)
[[ -d sidecars ]] && workspace_roots+=(sidecars)
workspace_manifests="$(gate_find_collect 'workspace manifest discovery' "${workspace_roots[@]}" -name Cargo.toml -type f)" || exit $?
workspace_crate_names="$(
    while IFS= read -r crate_manifest; do
        awk '
            /^\[package\]$/ { in_package = 1; next }
            /^\[/ { in_package = 0 }
            in_package && /^name[[:space:]]*=/ {
                value = $0
                sub(/^name[[:space:]]*=[[:space:]]*"/, "", value)
                sub(/".*/, "", value)
                print value
            }
        ' "$crate_manifest"
    done <<<"$workspace_manifests"
)"
while IFS= read -r dependency; do
    [[ -n "$dependency" ]] || continue
    if [[ "$dependency" == wide ]] || rg -qx -- "$dependency" <<<"$workspace_crate_names"; then
        continue
    fi
    fail "crates/lane may depend only on wide and workspace crates, found $dependency"
done < <(locked_dependencies lane)

while IFS= read -r dependency; do
    [[ -n "$dependency" ]] || continue
    case "$dependency" in
        bytemuck | safe_arch) ;;
        *) fail "wide must pull only bytemuck and safe_arch, found $dependency" ;;
    esac
done < <(locked_dependencies wide)

printf 'lane policy: ok\n'
