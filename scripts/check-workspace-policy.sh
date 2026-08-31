#!/usr/bin/env bash
# Verify bootstrap naming and target-policy guardrails. This is a guard, not permission to hide a
# capacity limit or a global ISA choice elsewhere.
set -euo pipefail

workspace_root="${1:-.}"
cd "$workspace_root"

fail() {
    printf 'workspace policy failure: %s\n' "$1" >&2
    exit 1
}

# `rg` exits 0 on a match, 1 when the pattern is clean, and 2 (or higher) on a search error --
# most commonly a search root that does not exist. `if rg ...; then fail; fi` reads both 1 and 2
# as "no violation", so a scan root that silently stops existing (a directory rename, a fixture
# missing a mkdir) reads as a clean pass instead of the scan never having run. This wrapper keeps
# the three outcomes distinct: 0 is a real violation, 1 is genuinely clean, and >=2 is a scan
# failure that must be loud, naming whichever of the given roots is actually missing.
scan_forbidden() {
    local description="$1" pattern="$2" glob="$3"
    shift 3
    local roots=("$@")
    local output rc
    if output="$(rg -n "$pattern" --glob "$glob" "${roots[@]}" 2>&1)"; then
        rc=0
    else
        rc=$?
    fi
    case "$rc" in
        0)
            printf '%s\n' "$output" >&2
            fail "$description"
            ;;
        1)
            ;;
        *)
            local missing=() root
            for root in "${roots[@]}"; do
                [[ -e "$root" ]] || missing+=("$root")
            done
            printf '%s\n' "$output" >&2
            if [[ "${#missing[@]}" -gt 0 ]]; then
                fail "$description scan could not run (rg exit $rc): missing search path(s): ${missing[*]}"
            else
                fail "$description scan errored (rg exit $rc)"
            fi
            ;;
    esac
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

while IFS= read -r manifest; do
    package_directory="$(basename "$(dirname "$manifest")")"
    package_name="$(toml_name package "$manifest")"

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

scan_forbidden "hardware ISA Cargo features are forbidden" \
    '^[[:space:]]*(simd128|neon|avx2|fma)[[:space:]]*=' Cargo.toml \
    crates hosts tools sidecars

scan_forbidden "compiled track-capacity identifiers are forbidden" \
    '\b(MAX_TRACKS|MAX_TRACK_COUNT|DEFAULT_MAX_TRACKS|TRACK_LIMIT)\b' '*.rs' \
    crates hosts tools sidecars

# Master plan #83 D4 (revision 4): exactly one global ISA configuration is approved, the
# x86-64-v3 pin that lets `wide` lower `Lane` to AVX2 and `Lane::fma` to `vfmadd` with no runtime
# dispatch (crates/miso-engine-lane refuses to compile without it, and every host attests the CPU
# at boot). Anything else -- `target-cpu`, a global `[build]` table, another feature set -- stays
# forbidden: it would make the shipped ISA implicit again.
approved_isa_pin='^\.cargo/config\.toml:[0-9]+:rustflags = \["-C", "target-feature=\+avx2,\+fma"\]$'
if [[ -d .cargo ]]; then
    isa_directives="$({
        rg -n '(target-cpu|target-feature|rustflags|RUSTFLAGS)' .cargo || true
    } | rg -v ':[0-9]+:[[:space:]]*#' || true)"

    unapproved_directives="$(printf '%s' "$isa_directives" | rg -v "$approved_isa_pin" || true)"
    [[ -z "$unapproved_directives" ]] || {
        printf '%s\n' "$unapproved_directives" >&2
        fail "global native CPU or ISA configuration is forbidden outside the approved x86-64-v3 pin"
    }

    if [[ -n "$isa_directives" ]]; then
        rg -q "^\[target\.'cfg\(target_arch = \"x86_64\"\)'\]\$" .cargo/config.toml || {
            fail "the approved ISA pin must stay scoped to [target.'cfg(target_arch = \"x86_64\")']"
        }
    fi

    if rg -n '^\[build\]' .cargo; then
        fail "a global [build] rustflags table is forbidden"
    fi
fi


# #104 phase D. `CARGO_TARGET_DIR=.` (or a stray `--target-dir .`) writes a cargo target tree next
# to `Cargo.toml`: `.rustc_info.json`, `CACHEDIR.TAG`, and one directory per profile/target, each
# holding `.fingerprint/`. 235 such files were committed to `main`. `.gitignore` stops them being
# added again; this gate stops them existing at all, because an ignored spill still poisons every
# `find`/`rg` gate that walks the tree from the workspace root.
for marker in .rustc_info.json CACHEDIR.TAG; do
    [[ ! -e "$marker" ]] || fail "cargo target-dir spill at the workspace root: $marker"
done
while IFS= read -r fingerprint; do
    [[ -z "$fingerprint" ]] && continue
    fail "cargo target-dir spill at the workspace root: ${fingerprint%/.fingerprint}"
done < <(find . -mindepth 2 -maxdepth 2 -type d -name .fingerprint -not -path './target/*' -printf '%P\n')

printf 'workspace policy: ok\n'
