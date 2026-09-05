#!/usr/bin/env bash
# Verify bootstrap naming and target-policy guardrails. This is a guard, not permission to hide a
# capacity limit or a global ISA choice elsewhere.
set -euo pipefail

workspace_root="${1:-.}"
script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_directory/lib/gate.sh"
cd "$workspace_root"

fail() {
    GATE_FAILURE_PREFIX='workspace policy failure' gate_fail "$1"
    exit 1
}

scratch_dir="$(mktemp -d "${TMPDIR:-/tmp}/workspace-policy.XXXXXX")"
trap 'rm -rf -- "$scratch_dir"' EXIT

# Run a producer to completion while keeping its stdout and stderr separate.  Callers inspect the
# status explicitly; a clean-looking partial result is never evidence that the producer finished.
capture() {
    local label="$1" out="$scratch_dir/$1.out" err="$scratch_dir/$1.err"
    shift
    if "$@" >"$out" 2>"$err"; then
        CAPTURE_STATUS=0
    else
        CAPTURE_STATUS=$?
    fi
    CAPTURE_OUT="$out"
    CAPTURE_ERR="$err"
}

execution_failure() {
    local operation="$1" status="$2" out="$3" err="$4"
    fail "$operation failed (status $status): stdout=$(<"$out") stderr=$(<"$err")"
}

checked_find() {
    local label="$1"; shift
    capture "$label" find "$@"
    (( CAPTURE_STATUS == 0 )) || execution_failure "find $label" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    cat "$CAPTURE_OUT"
}

checked_sort() {
    local label="$1" input="$2"
    capture "$label" sort "$input"
    (( CAPTURE_STATUS == 0 )) || execution_failure "sort $label" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    cat "$CAPTURE_OUT"
}

checked_rg() {
    local label="$1"; shift
    capture "$label" rg "$@"
    printf '%s\n' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
}

# `rg` exits 0 on a match, 1 when the pattern is clean, and 2 (or higher) on a search error --
# most commonly a search root that does not exist. `if rg ...; then fail; fi` reads both 1 and 2
# as "no violation", so a scan root that silently stops existing (a directory rename, a fixture
# missing a mkdir) reads as a clean pass instead of the scan never having run. This wrapper keeps
# the three outcomes distinct: 0 is a real violation, 1 is genuinely clean, and >=2 is a scan
# failure that must be loud, naming whichever of the given roots is actually missing.
scan_forbidden() {
    GATE_FAILURE_PREFIX='workspace policy failure' gate_scan_forbidden "$@" || exit 1
}

toml_name() {
    local section="$1"
    local manifest="$2"
    awk -v section="$section" '
        $0 == "[" section "]" { in_section = 1; next }
        in_section && /^\[/ { exit }
        in_section && /^[[:space:]]*name[[:space:]]*=/ {
            value = $0
            sub(/^[[:space:]]*name[[:space:]]*=[[:space:]]*"/, "", value)
            sub(/"[[:space:]]*$/, "", value)
            print value
            exit
        }
    ' "$manifest"
}

toml_array_names() {
    local section="$1"
    local manifest="$2"
    awk -v section="$section" '
        $0 == "[[" section "]]" { in_section = 1; next }
        in_section && /^\[/ { in_section = 0 }
        in_section && /^[[:space:]]*name[[:space:]]*=/ {
            value = $0
            sub(/^[[:space:]]*name[[:space:]]*=[[:space:]]*"/, "", value)
            sub(/"[[:space:]]*$/, "", value)
            print value
            in_section = 0
        }
    ' "$manifest"
}

# Issue #314: Apache-2.0 is the default license for original project work. The digest protects the
# legal text itself, not a prose claim: a truncated or edited LICENSE grants different rights.
apache_license_sha256='cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30'
for required_license_file in LICENSE NOTICE THIRD_PARTY_LICENSES.md crates/math/LICENSE-libm.txt; do
    [[ -s "$required_license_file" ]] || fail "required license artifact is missing or empty: $required_license_file"
done
capture license_sha256 sha256sum LICENSE
(( CAPTURE_STATUS == 0 )) || execution_failure 'sha256sum LICENSE' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
capture license_digest awk '{print $1}' "$CAPTURE_OUT"
(( CAPTURE_STATUS == 0 )) || execution_failure 'LICENSE digest extraction' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
actual_license_sha256="$(<"$CAPTURE_OUT")"
[[ "$actual_license_sha256" == "$apache_license_sha256" ]] || {
    fail "LICENSE is not the canonical Apache License 2.0 text"
}
required_search() {
    local label="$1" message="$2"; shift 2
    capture "$label" rg "$@"
    case "$CAPTURE_STATUS" in
        0) ;;
        1) fail "$message" ;;
        *) execution_failure "rg $label" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR" ;;
    esac
}
required_search workspace-license 'Cargo.toml workspace package license must be Apache-2.0' -qx 'license = "Apache-2.0"' Cargo.toml
required_search fuzz-license 'fuzz/Cargo.toml license must be Apache-2.0' -qx 'license = "Apache-2.0"' fuzz/Cargo.toml
required_search libm-inventory 'third-party inventory must retain the vendored libm license record' -q 'crates/math/LICENSE-libm\.txt' THIRD_PARTY_LICENSES.md

npm_manifests="$(checked_find npm-manifests . -name package.json -type f -not -path '*/node_modules/*')"
capture npm-manifests-sort sort <<<"$npm_manifests"
(( CAPTURE_STATUS == 0 )) || execution_failure 'sort npm manifests' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
sorted_npm_manifests="$CAPTURE_OUT"
while IFS= read -r npm_manifest; do
    [[ -n "$npm_manifest" ]] || continue
    capture "jq-npm-$RANDOM" jq -e '.license == "Apache-2.0"' "$npm_manifest"
    case "$CAPTURE_STATUS" in
        0) ;;
        1) fail "$npm_manifest license must be Apache-2.0" ;;
        *) execution_failure "jq $npm_manifest" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR" ;;
    esac
done <"$sorted_npm_manifests"

npm_locks="$(checked_find npm-locks . -name package-lock.json -type f -not -path '*/node_modules/*')"
capture npm-locks-sort sort <<<"$npm_locks"
(( CAPTURE_STATUS == 0 )) || execution_failure 'sort npm locks' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
sorted_npm_locks="$CAPTURE_OUT"
while IFS= read -r npm_lock; do
    [[ -n "$npm_lock" ]] || continue
    capture "jq-lock-$RANDOM" jq -e '.packages[""].license == "Apache-2.0"' "$npm_lock"
    case "$CAPTURE_STATUS" in
        0) ;;
        1) fail "$npm_lock root package license must be Apache-2.0" ;;
        *) execution_failure "jq $npm_lock" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR" ;;
    esac
done <"$sorted_npm_locks"

cargo_manifests="$(checked_find cargo-manifests crates hosts tools sidecars -name Cargo.toml -type f)"
capture cargo-manifests-sort sort <<<"$cargo_manifests"
(( CAPTURE_STATUS == 0 )) || execution_failure 'sort Cargo manifests' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
sorted_cargo_manifests="$CAPTURE_OUT"
[[ -s "$CAPTURE_OUT" ]] || fail 'Cargo manifest discovery returned an empty workspace'
while IFS= read -r manifest; do
    [[ -n "$manifest" ]] || continue
    package_directory="$(basename "$(dirname "$manifest")")"
    capture "toml-package-$RANDOM" toml_name package "$manifest"
    (( CAPTURE_STATUS == 0 )) || execution_failure "package name extraction $manifest" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    package_name="$(<"$CAPTURE_OUT")"

    required_search "package-license-$RANDOM" "$manifest must inherit the Apache-2.0 workspace license" -qx 'license\.workspace = true' "$manifest"

    [[ "$package_name" != miso-engine-* && "$package_name" != miso_engine_* ]] || fail "$manifest package name must not carry the retired miso-engine- prefix"
    [[ "$package_directory" == "$package_name" ]] || fail "$manifest directory ($package_directory) must equal its package name ($package_name)"
    case "$package_name" in core|std|alloc|proc_macro|test) fail "$manifest package name '$package_name' collides with a Rust sysroot/prelude crate name";; esac
    case "$package_name" in flac-decoder|stem-publisher|catalog-migrate|flacenc|symphonia) fail "$manifest package name is a retired delivery-codec identity: $package_name";; esac
    expected_crate_name="${package_name//-/_}"
    capture "toml-lib-$RANDOM" toml_name lib "$manifest"
    (( CAPTURE_STATUS == 0 )) || execution_failure "lib name extraction $manifest" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    lib_name="$(<"$CAPTURE_OUT")"
    [[ -z "$lib_name" || "$lib_name" == "$expected_crate_name" ]] || fail "$manifest lib name must be $expected_crate_name"
    capture "toml-bin-$RANDOM" toml_array_names bin "$manifest"
    (( CAPTURE_STATUS == 0 )) || execution_failure "bin name extraction $manifest" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    while IFS= read -r bin_name; do
        [[ -z "$bin_name" ]] && continue
        [[ "$bin_name" == "$expected_crate_name" || "$bin_name" == "$expected_crate_name"_* ]] || fail "$manifest bin name must be $expected_crate_name or its underscored audit/tool suffix"
    done <"$CAPTURE_OUT"
done <"$sorted_cargo_manifests"

if false; then
while IFS= read -r npm_manifest; do
    jq -e '.license == "Apache-2.0"' "$npm_manifest" >/dev/null || {
        fail "$npm_manifest license must be Apache-2.0"
    }
done < <(find . -name package.json -type f -not -path '*/node_modules/*' | sort)

while IFS= read -r npm_lock; do
    jq -e '.packages[""].license == "Apache-2.0"' "$npm_lock" >/dev/null || {
        fail "$npm_lock root package license must be Apache-2.0"
    }
done < <(find . -name package-lock.json -type f -not -path '*/node_modules/*' | sort)

while IFS= read -r manifest; do
    package_directory="$(basename "$(dirname "$manifest")")"
    package_name="$(toml_name package "$manifest")"

    rg -qx 'license\.workspace = true' "$manifest" || {
        fail "$manifest must inherit the Apache-2.0 workspace license"
    }

    # The `miso-engine-` prefix convention was retired by the prefix-strip rename (see
    # docs/rulings/prefix-strip-inventory.md): every package under crates/, hosts/, tools/ and
    # sidecars/ now carries a short, unprefixed name, and the directory basename equals the
    # package name exactly -- there is no longer a sidecars/-only exemption to reason about,
    # because there is no prefix left for it to be exempt from. This also forbids regressing
    # back to the old prefix on a new or renamed crate.
    [[ "$package_name" != miso-engine-* && "$package_name" != miso_engine_* ]] || {
        fail "$manifest package name must not carry the retired miso-engine- prefix"
    }
    [[ "$package_directory" == "$package_name" ]] || {
        fail "$manifest directory ($package_directory) must equal its package name ($package_name)"
    }

    # `core` silently shadows Rust's sysroot `core` crate for every dependent: it compiles, then
    # fails downstream with no diagnostic that explains itself, and collapses the prelude and the
    # `derive` attribute outright in a `no_std` crate -- proven when this repo chose `engine` over
    # `core` for its former `miso-engine-core` (docs/rulings/prefix-strip-inventory.md). Neither
    # `::core::` nor `extern crate core as x` escapes it: cargo's `--extern core=<path>` overrides
    # the sysroot crate of that name unconditionally. `std`, `alloc`, `proc_macro` and `test` are
    # the same hazard in kind (sysroot/prelude crate names), so they are forbidden alongside it.
    case "$package_name" in
        core | std | alloc | proc_macro | test)
            fail "$manifest package name '$package_name' collides with a Rust sysroot/prelude crate name"
            ;;
    esac

    # Issue #356/#359: the in-repository FLAC delivery stack (and its migration tooling) was
    # retired; PCM is this repository's output and delivery codecs live outside it. This was
    # `scripts/check-delivery-codec-boundary.py`, folded in here because #358's claim ("no
    # retired delivery-codec identity in the live workspace") is a name-absence claim over
    # exactly the same manifests, lockfile and tree this loop already walks.
    case "$package_name" in
        flac-decoder | stem-publisher | catalog-migrate | flacenc | symphonia)
            fail "$manifest package name is a retired delivery-codec identity: $package_name"
            ;;
    esac
    # N15: a directory-name case here is dead code -- by this point in the loop the earlier
    # `[[ "$package_directory" == "$package_name" ]]` check (above) has already exited unless the
    # two are equal, so any input that could trip a directory-name match here already tripped the
    # package-name case immediately above it. The bare-directory-stub scan below (no Cargo.toml at
    # all) is what actually catches a retired directory name.

    expected_crate_name="${package_name//-/_}"

    lib_name="$(toml_name lib "$manifest")"
    if [[ -n "$lib_name" ]]; then
        [[ "$lib_name" == "$expected_crate_name" ]] || {
            fail "$manifest lib name must be $expected_crate_name"
        }
    fi

    while IFS= read -r bin_name; do
        [[ "$bin_name" == "$expected_crate_name" || "$bin_name" == "$expected_crate_name"_* ]] || {
            fail "$manifest bin name must be $expected_crate_name or its underscored audit/tool suffix"
        }
    done < <(toml_array_names bin "$manifest")
done < <(find crates hosts tools sidecars -name Cargo.toml -type f | sort)
fi

scan_forbidden "hardware ISA Cargo features are forbidden" \
    '^[[:space:]]*(simd128|neon|avx2|fma)[[:space:]]*=' Cargo.toml \
    crates hosts tools sidecars

# S8: every tracked (or freshly added, untracked-but-not-ignored) path in the tree, used below to
# find every Cargo.toml regardless of location -- not the prior six hard-coded roots, which missed
# a nested manifest under e.g. sdk/. `git ls-files` is the tree's own ground truth for "what exists
# here"; the `find` fallback keeps this working against a synthetic (non-git) fixture tree, exactly
# as `scripts/check-env-vocabulary.sh`'s `sources()` does.
tracked_paths() {
    local classify_status
    capture git-classify git rev-parse --is-inside-work-tree
    classify_status="$CAPTURE_STATUS"
    if (( classify_status == 0 )); then
        capture git-list git ls-files -z --cached --others --exclude-standard
        (( CAPTURE_STATUS == 0 )) || execution_failure 'git tracked-path listing' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
        local nul_file="$CAPTURE_OUT"
    elif (( classify_status == 128 )) && grep -Fxq 'fatal: not a git repository (or any of the parent directories): .git' "$CAPTURE_ERR"; then
        capture fallback-list find . -type f -not -path './.git/*' -not -path './target/*' -print0
        (( CAPTURE_STATUS == 0 )) || execution_failure 'fallback tracked-path find' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
        local nul_file="$CAPTURE_OUT"
    else
        execution_failure 'git repository classification' "$classify_status" "$CAPTURE_OUT" "$CAPTURE_ERR"
    fi
    capture nul-to-lines tr '\0' '\n' <"$nul_file"
    (( CAPTURE_STATUS == 0 )) || execution_failure 'tracked-path NUL conversion' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    local lines="$CAPTURE_OUT"
    capture normalize-paths sed 's|^\./||' "$lines"
    (( CAPTURE_STATUS == 0 )) || execution_failure 'tracked-path normalization' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    local normalized="$CAPTURE_OUT"
    capture cargo-path-filter awk -F/ '$NF == "Cargo.toml"' "$normalized"
    (( CAPTURE_STATUS == 0 )) || execution_failure 'tracked Cargo manifest filter' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    capture tracked-path-sort sort "$CAPTURE_OUT"
    (( CAPTURE_STATUS == 0 )) || execution_failure 'tracked Cargo manifest sort' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    cat "$CAPTURE_OUT"
}

# Issue #356/#359: a bare retired-directory stub (no Cargo.toml, so invisible to the manifest
# loop above) is still a regression -- these five names carried the retired FLAC delivery stack
# and its migration tooling. A whole-tree `find -type d` (N16), not `-mindepth 1 -maxdepth 2`
# under four hard-coded roots: it reaches the repo root, sdk/, and any depth, and -- unlike a scan
# derived from `git ls-files`/tracked file paths -- it also catches a directory with nothing
# tracked inside it yet, which is exactly the shape of a freshly-created leftover stub.
capture retired-dirs find . \( -path './.git' -o -path './target' \) -prune -o -type d \( \
    -name flac-decoder -o -name stem-publisher -o -name catalog-migrate \
    -o -name flacenc -o -name symphonia \) -print
(( CAPTURE_STATUS == 0 )) || execution_failure 'retired-directory discovery' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
while IFS= read -r retired_directory; do
    [[ -z "$retired_directory" ]] && continue
    fail "retired delivery-codec directory remains: ${retired_directory#./}"
done <"$CAPTURE_OUT"

# Every Cargo.toml in the tree (S8: every tracked path named `Cargo.toml`, at any depth,
# excluding nothing -- not the prior six hard-coded roots) and the lockfile: a retired identity
# can also appear as a dependency key, a `package = "..."` rename, or (in Cargo.lock) a resolved
# package entry, none of which the manifest loop above sees.
retired_delivery_codec_pattern='\b(flac-decoder|stem-publisher|catalog-migrate|flacenc|symphonia)\b'

# N17: strip TOML comments before matching, line by line, tracking whether each character is
# inside a quoted string -- a retired name's mere mention in prose ("# we deliberately do not
# depend on symphonia") must not trip this gate, while a real dependency/rename entry (which is
# never itself preceded by an unquoted '#' on its own line) still does. This is intentionally
# line-oriented: Cargo.toml does not use multi-line strings for dependency names or package
# identifiers, which is everything this scan cares about.
strip_toml_comments() {
    awk '
        {
            in_string = 0
            out = ""
            for (i = 1; i <= length($0); i++) {
                c = substr($0, i, 1)
                if (c == "\"") { in_string = !in_string }
                if (c == "#" && !in_string) { break }
                out = out c
            }
            print out
        }
    '
}

tracked_manifest_paths="$(tracked_paths)"
while IFS= read -r manifest_path; do
    [[ -z "$manifest_path" ]] && continue
    [[ -f "$manifest_path" ]] || continue
    capture strip-comments awk ' {
        in_string=0; out=""
        for (i=1; i<=length($0); i++) { c=substr($0,i,1); if (c=="\"") in_string=!in_string; if (c=="#" && !in_string) break; out=out c }
        print out
    }' "$manifest_path"
    (( CAPTURE_STATUS == 0 )) || execution_failure "comment stripping $manifest_path" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    capture retired-scan rg -n "$retired_delivery_codec_pattern" "$CAPTURE_OUT"
    case "$CAPTURE_STATUS" in
      0) printf '%s\n' "$(<"$CAPTURE_OUT")" >&2; fail "retired delivery-codec Cargo identity is forbidden: $manifest_path";;
      1) ;;
      *) execution_failure "retired identity scan $manifest_path" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR";;
    esac
done <<<"$tracked_manifest_paths"

scan_forbidden "retired delivery-codec Cargo identity is forbidden in the lockfile" \
    "$retired_delivery_codec_pattern" 'Cargo.lock' \
    Cargo.lock

scan_forbidden "compiled track-capacity identifiers are forbidden" \
    '\b(MAX_TRACKS|MAX_TRACK_COUNT|DEFAULT_MAX_TRACKS|TRACK_LIMIT)\b' '*.rs' \
    crates hosts tools sidecars

# #313 owner ruling: this is the first prelaunch engine identity. Internal names are unversioned,
# and a boundary that genuinely needs a generation is V1. Build the expression in fragments so
# this policy file does not contain the forbidden spellings it scans for.
prelaunch_later_generation_pattern='(miso_engine_'v'2|MISO_ENGINE_'V'2|miso-engine-'v'2|ENGINE_'V'2|Engine 'V'2|boot[- ]'v'2|Boot 'v'2|schema-'v'2|@miso/engine-'v'2)'
scan_forbidden "prelaunch live-product identities must not claim a later generation" \
    "$prelaunch_later_generation_pattern" '*' crates hosts tools sidecars

# Implementation class names are private even when their containing script is shipped. Only the
# registered processor token is boundary identity; an internal JavaScript class is born
# unversioned under #215's rule.
versioned_worklet_implementation_pattern='class[[:space:]]+MisoEngine'V'[0-9]+AudioWorkletProcessor'
scan_forbidden "AudioWorklet processor implementation classes must be unversioned" \
    "$versioned_worklet_implementation_pattern" '*.js' crates hosts tools sidecars

# Master plan #83 D4 (revision 4): exactly one global ISA configuration is approved, the
# x86-64-v3 pin that lets `wide` lower `Lane` to AVX2 and `Lane::fma` to `vfmadd` with no runtime
# dispatch (crates/lane refuses to compile without it, and every host attests the CPU
# at boot). Anything else -- `target-cpu`, a global `[build]` table, another feature set -- stays
# forbidden: it would make the shipped ISA implicit again.
approved_isa_pin='^\.cargo/config\.toml:[0-9]+:rustflags = \["-C", "target-feature=\+avx2,\+fma"\]$'
if [[ -d .cargo ]]; then
    capture isa-directives rg -n '(target-cpu|target-feature|rustflags|RUSTFLAGS)' .cargo
    (( CAPTURE_STATUS == 0 || CAPTURE_STATUS == 1 )) || execution_failure 'ISA directive search' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    capture isa-comments-filter rg -v ':[0-9]+:[[:space:]]*#' "$CAPTURE_OUT"
    (( CAPTURE_STATUS == 0 || CAPTURE_STATUS == 1 )) || execution_failure 'ISA comment filtering' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    isa_directives="$(<"$CAPTURE_OUT")"
    capture isa-allowlist-filter rg -v "$approved_isa_pin" <<<"$isa_directives"
    (( CAPTURE_STATUS == 0 || CAPTURE_STATUS == 1 )) || execution_failure 'ISA allowlist filtering' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    unapproved_directives="$(<"$CAPTURE_OUT")"
    [[ -z "$unapproved_directives" ]] || {
        printf '%s\n' "$unapproved_directives" >&2
        fail "global native CPU or ISA configuration is forbidden outside the approved x86-64-v3 pin"
    }

    if [[ -n "$isa_directives" ]]; then
        required_search isa-target-scope "the approved ISA pin must stay scoped to [target.'cfg(target_arch = \"x86_64\")']" -q "^\[target\.'cfg\(target_arch = \"x86_64\"\)'\]\$" .cargo/config.toml
    fi

    capture build-table rg -n '^\[build\]' .cargo
    case "$CAPTURE_STATUS" in
      0) fail "a global [build] rustflags table is forbidden";;
      1) ;;
      *) execution_failure 'global build-table search' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR";;
    esac
fi


# #104 phase D. `CARGO_TARGET_DIR=.` (or a stray `--target-dir .`) writes a cargo target tree next
# to `Cargo.toml`: `.rustc_info.json`, `CACHEDIR.TAG`, and one directory per profile/target, each
# holding `.fingerprint/`. 235 such files were committed to `main`. `.gitignore` stops them being
# added again; this gate stops them existing at all, because an ignored spill still poisons every
# `find`/`rg` gate that walks the tree from the workspace root.
for marker in .rustc_info.json CACHEDIR.TAG; do
    [[ ! -e "$marker" ]] || fail "cargo target-dir spill at the workspace root: $marker"
done
capture fingerprints find . -mindepth 2 -maxdepth 2 -type d -name .fingerprint -not -path './target/*' -printf '%P\n'
(( CAPTURE_STATUS == 0 )) || execution_failure 'fingerprint discovery' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
while IFS= read -r fingerprint; do
    [[ -z "$fingerprint" ]] && continue
    fail "cargo target-dir spill at the workspace root: ${fingerprint%/.fingerprint}"
done <"$CAPTURE_OUT"

printf 'workspace policy: ok\n'
