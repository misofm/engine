#!/usr/bin/env bash
set -euo pipefail

root=$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)
cd "$root"

paths=(crates/builtins crates/builtins-compiler)
dependencies() {
    awk '/^\[dependencies\]$/ { in_deps=1; next } /^\[/ { in_deps=0 } in_deps && /^[A-Za-z0-9_-]+(\.workspace)?[[:space:]]*=/ { line=$0; sub(/[[:space:]]*=.*/, "", line); sub(/\.workspace$/, "", line); print line }' "$1" | sort
}
expected_builtins=$'effect-contract\nengine\nlane\nmath'
[[ "$(dependencies crates/builtins/Cargo.toml)" == "$expected_builtins" ]] || {
    printf 'builtins policy failure: builtins dependency boundary changed\n' >&2; exit 1;
}
# #84 phase A: the compiler names lane::Backend instead of the deleted core enums.
expected_compiler=$'builtins\neffect-contract\nengine\ngraph\nlane\nrack\nrack-compiler\nsession\nsha2'
[[ "$(dependencies crates/builtins-compiler/Cargo.toml)" == "$expected_compiler" ]] || {
    printf 'builtins policy failure: builtins compiler dependency boundary changed\n' >&2; exit 1;
}
if rg -n 'builtins' crates/{engine,session,graph}/Cargo.toml; then
    printf 'builtins policy failure: reverse dependency\n' >&2; exit 1
fi
rg --fixed-strings 'unsafe' "${paths[@]}" \
    | rg -v '^crates/builtins-compiler/tests/allocation_tracker.rs:' \
    && exit 1 || true
rg --fixed-strings 'MAX_TRACKS' "${paths[@]}" && exit 1 || true
rg --fixed-strings 'builtins' Cargo.toml crates/builtins/Cargo.toml crates/builtins-compiler/Cargo.toml >/dev/null
rg --fixed-strings 'builtins' crates/builtins/Cargo.toml crates/builtins-compiler/Cargo.toml >/dev/null

if [[ "${MISO_ENGINE_BUILTINS_SKIP_METADATA:-0}" != 1 ]]; then
    cargo metadata --no-deps --format-version 1 >/dev/null
fi
printf 'builtins policy: ok\n'
