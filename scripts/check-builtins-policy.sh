#!/usr/bin/env bash
set -euo pipefail
script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_directory/lib/gate.sh"

root=$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)
cd "$root"
GATE_FAILURE_PREFIX='builtins policy failure'

paths=(crates/builtins crates/builtins-compiler)
dependencies() { gate_toml_dependencies "$1"; }
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
unsafe_matches="$(gate_scan_collect 'builtins unsafe scan' 'unsafe' '' "${paths[@]}")" || exit 1
unsafe_matches="$(printf '%s\n' "$unsafe_matches" | rg -v '^crates/builtins-compiler/tests/allocation_tracker.rs:' || true)"
[[ -z "$unsafe_matches" ]] || { printf '%s\n' "$unsafe_matches" >&2; exit 1; }
# The MAX_TRACKS ban lives once, in scripts/check-workspace-policy.sh (P12), which scans the
# whole {crates,hosts,tools,sidecars} tree rather than one of five copies of the same check.
gate_scan_required 'workspace builtins declaration is missing' 'builtins' '' Cargo.toml crates/builtins/Cargo.toml crates/builtins-compiler/Cargo.toml
gate_scan_required 'builtins compiler declarations are missing' 'builtins' '' crates/builtins/Cargo.toml crates/builtins-compiler/Cargo.toml

if [[ "${MISO_ENGINE_BUILTINS_SKIP_METADATA:-0}" != 1 ]]; then
    cargo metadata --no-deps --format-version 1 >/dev/null
fi
printf 'builtins policy: ok\n'
