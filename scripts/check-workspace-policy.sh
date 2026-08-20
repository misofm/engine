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
    [[ "$package_directory" == miso-engine-* ]] || {
        fail "$manifest directory must start miso-engine-"
    }

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
        [[ "$bin_name" == "$expected_crate_name" ]] || {
            fail "$manifest bin name must be $expected_crate_name"
        }
    done < <(toml_array_names bin "$manifest")
done < <(find crates hosts tools -name Cargo.toml -type f | sort)

if rg -n '^[[:space:]]*(simd128|neon|avx2|fma)[[:space:]]*=' \
    --glob Cargo.toml crates hosts tools; then
    fail "hardware ISA Cargo features are forbidden"
fi

if rg -n '\b(MAX_TRACKS|MAX_TRACK_COUNT|DEFAULT_MAX_TRACKS|TRACK_LIMIT)\b' \
    --glob '*.rs' crates hosts tools; then
    fail "compiled track-capacity identifiers are forbidden"
fi

if [[ -d .cargo ]] && rg -n '(target-cpu|target-feature|RUSTFLAGS)' .cargo; then
    fail "global native CPU or ISA configuration is forbidden"
fi

printf 'workspace policy: ok\n'
