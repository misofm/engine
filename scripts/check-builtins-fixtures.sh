#!/usr/bin/env bash
# Usage: check-builtins-fixtures.sh [repo-root] [path/to/audit]
set -euo pipefail

# The manifest is byte-sorted (C collation). `[[ a > b ]]` and `sort` both follow the caller's
# collation, so a UTF-8 locale reordered `pcm/matrix-ramp-1.f32le` against
# `pcm/matrix-ramp-127.f32le` and this gate failed on an unchanged tree (#104 phase A).
export LC_ALL=C

# S3: resolve a relative `[path/to/audit]` against the caller's cwd BEFORE `cd "${1:-.}"` below --
# otherwise `check-builtins-fixtures.sh subdir target/release/audit` would resolve the binary
# under `subdir` instead of the caller's own working directory.
audit_binary="${2:-}"
if [[ -n "$audit_binary" ]]; then
    case "$audit_binary" in
        /*) : ;;
        *) audit_binary="$(realpath -m -- "$audit_binary")" ;;
    esac
    # S1/S2: an explicit binary path must be an existing executable file, never a directory or a
    # missing path -- and an explicit-but-missing path is a hard error, not a build trigger.
    [[ -f "$audit_binary" && -x "$audit_binary" ]] || {
        printf 'missing audit binary: %s\n' "$audit_binary" >&2
        exit 1
    }
fi

cd "${1:-.}"
root="fixtures/builtins/v1"
manifest="$root/MANIFEST.tsv"
workspace_root="$(cd "$(dirname "$0")/.." && pwd)"
[[ -f "$manifest" ]] || { printf 'missing builtins fixture manifest\n' >&2; exit 1; }
[[ "$(head -n 1 "$manifest")" == $'path\tlength\tsha256' ]] || {
    printf 'invalid builtins fixture manifest header\n' >&2; exit 1;
}
previous=""
listed="$(mktemp)"
actual="$(mktemp)"
canary_root="$(mktemp -d)"
trap 'rm -f -- "$listed" "$actual"; rm -rf -- "$canary_root"' EXIT
while IFS=$'\t' read -r path length hash; do
    [[ "$path" == path ]] && continue
    [[ "$path" > "$previous" ]] || { printf 'builtins fixture manifest is not strictly sorted\n' >&2; exit 1; }
    previous="$path"
    file="$root/$path"
    [[ -f "$file" ]] || { printf 'missing builtins fixture: %s\n' "$path" >&2; exit 1; }
    [[ "$length" =~ ^[0-9]+$ ]] || { printf 'invalid builtins fixture length: %s\n' "$path" >&2; exit 1; }
    [[ "$(wc -c <"$file" | tr -d ' ')" == "$length" ]] || { printf 'builtins fixture length mismatch: %s\n' "$path" >&2; exit 1; }
    [[ "$hash" =~ ^[0-9a-f]{64}$ ]] || { printf 'invalid builtins fixture hash: %s\n' "$path" >&2; exit 1; }
    [[ "$(sha256sum "$file" | awk '{print $1}')" == "$hash" ]] || { printf 'builtins fixture hash mismatch: %s\n' "$path" >&2; exit 1; }
    printf '%s\n' "$path" >>"$listed"
done <"$manifest"
find "$root" -type f ! -name MANIFEST.tsv -printf '%P\n' | sort >"$actual"
cmp -s "$listed" "$actual" || { printf 'builtins fixture missing/unlisted file\n' >&2; exit 1; }

# B1: `fixture-builtins --check` is silent on success (it only ever prints on failure), so a
# stale/wrong binary -- or a stand-in like /bin/true -- exits 0 with nothing to distinguish it
# from a real pass. Prove the binary actually performs the check: corrupt a scratch copy's first
# manifest-listed payload byte and require the same `--check` invocation to reject it.
run_fixture_check() {
    local binary_desc="$1" target="$2"
    if [[ -n "$audit_binary" ]]; then
        "$audit_binary" fixture-builtins --check "$target"
    else
        cargo run --quiet --bin audit \
            --manifest-path "$workspace_root/tools/audit/Cargo.toml" \
            -- fixture-builtins --check "$target"
    fi
}
if [[ -n "$audit_binary" ]] || [[ -d "$workspace_root/tools/audit" ]]; then
    run_fixture_check positive "$(pwd)/$root" || {
        printf 'builtins fixture expected-output check failed\n' >&2
        exit 1
    }
    cp -R "$root" "$canary_root/builtins-v1"
    canary_target_rel="$(head -n 1 "$listed")"
    printf '\x00' >>"$canary_root/builtins-v1/$canary_target_rel"
    if run_fixture_check canary "$canary_root/builtins-v1" >/dev/null 2>&1; then
        printf 'builtins fixture audit binary accepted a corrupted fixture root -- refusing to trust its "ok"\n' >&2
        exit 1
    fi
fi
printf 'builtins fixtures: ok (%s files)\n' "$(wc -l <"$actual" | tr -d ' ')"
