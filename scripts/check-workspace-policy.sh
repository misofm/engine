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
    # sidecars/<name> is a deliberate exception to the directory-prefix rule: a sidecar
    # ships as its own artifact with its own ABI and is disjoint from the render engine's
    # dependency graph (AGENTS.md: "delivery codecs are external sidecars"). Its directory
    # is named by its short sidecar identity (e.g. sidecars/flac-decoder) rather than
    # repeating the miso-engine- prefix. The package name, [lib] name, and [[bin]] name
    # rules below are unchanged for sidecars -- only the directory prefix is relaxed for
    # this one tree.
    if [[ "$manifest" != sidecars/*/Cargo.toml ]]; then
        [[ "$package_directory" == miso-engine-* ]] || {
            fail "$manifest directory must start miso-engine-"
        }
    fi

    package_name="$(toml_name package "$manifest")"
    [[ "$package_name" == miso-engine-* ]] || fail "$manifest package name must start miso-engine-"
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

if rg -n '^[[:space:]]*(simd128|neon|avx2|fma)[[:space:]]*=' \
    --glob Cargo.toml crates hosts tools sidecars; then
    fail "hardware ISA Cargo features are forbidden"
fi

if rg -n '\b(MAX_TRACKS|MAX_TRACK_COUNT|DEFAULT_MAX_TRACKS|TRACK_LIMIT)\b' \
    --glob '*.rs' crates hosts tools sidecars; then
    fail "compiled track-capacity identifiers are forbidden"
fi

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
