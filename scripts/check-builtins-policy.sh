#!/usr/bin/env bash
set -euo pipefail

root=$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)
cd "$root"

paths=(crates/miso-engine-builtins crates/miso-engine-builtins-compiler)
dependencies() {
    awk '/^\[dependencies\]$/ { in_deps=1; next } /^\[/ { in_deps=0 } in_deps && /^[A-Za-z0-9_-]+(\.workspace)?[[:space:]]*=/ { line=$0; sub(/[[:space:]]*=.*/, "", line); sub(/\.workspace$/, "", line); print line }' "$1" | sort
}
expected_builtins=$'miso-engine-core\nmiso-engine-effect-contract\nmiso-engine-lane\nmiso-engine-math'
[[ "$(dependencies crates/miso-engine-builtins/Cargo.toml)" == "$expected_builtins" ]] || {
    printf 'builtins policy failure: builtins dependency boundary changed\n' >&2; exit 1;
}
expected_compiler=$'miso-engine-builtins\nmiso-engine-core\nmiso-engine-effect-contract\nmiso-engine-graph\nmiso-engine-rack\nmiso-engine-rack-compiler\nmiso-engine-session\nsha2'
[[ "$(dependencies crates/miso-engine-builtins-compiler/Cargo.toml)" == "$expected_compiler" ]] || {
    printf 'builtins policy failure: builtins compiler dependency boundary changed\n' >&2; exit 1;
}
if rg -n 'miso-engine-builtins' crates/miso-engine-{core,session,graph}/Cargo.toml; then
    printf 'builtins policy failure: reverse dependency\n' >&2; exit 1
fi
rg --fixed-strings 'unsafe' "${paths[@]}" \
    | rg -v '^crates/miso-engine-builtins-compiler/tests/allocation_tracker.rs:' \
    && exit 1 || true
rg --fixed-strings 'MAX_TRACKS' "${paths[@]}" && exit 1 || true
rg --fixed-strings 'miso-engine-builtins' Cargo.toml crates/miso-engine-builtins/Cargo.toml crates/miso-engine-builtins-compiler/Cargo.toml >/dev/null
rg --fixed-strings 'miso_engine_builtins' crates/miso-engine-builtins/Cargo.toml crates/miso-engine-builtins-compiler/Cargo.toml >/dev/null

if [[ "${MISO_ENGINE_BUILTINS_SKIP_METADATA:-0}" != 1 ]]; then
    cargo metadata --no-deps --format-version 1 >/dev/null
fi
printf 'builtins policy: ok\n'
