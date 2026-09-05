#!/usr/bin/env bash
set -euo pipefail
script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_directory/lib/gate.sh"

root=$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)
cd "$root"
GATE_FAILURE_PREFIX='builtins policy failure'

paths=(crates/builtins crates/builtins-compiler)
dependencies() { gate_toml_dependencies "$1" plain; }
expected_builtins=$'effect-contract\nengine\nlane\nmath'
[[ "$(dependencies crates/builtins/Cargo.toml)" == "$expected_builtins" ]] || {
    printf 'builtins policy failure: builtins dependency boundary changed\n' >&2; exit 1;
}
# #84 phase A: the compiler names lane::Backend instead of the deleted core enums.
expected_compiler=$'builtins\neffect-contract\nengine\ngraph\nlane\nrack\nrack-compiler\nsession\nsha2'
[[ "$(dependencies crates/builtins-compiler/Cargo.toml)" == "$expected_compiler" ]] || {
    printf 'builtins policy failure: builtins compiler dependency boundary changed\n' >&2; exit 1;
}
gate_scan_forbidden 'reverse dependency' 'builtins' '' crates/{engine,session,graph}/Cargo.toml || exit 1
unsafe_matches="$(gate_scan_collect 'builtins unsafe scan' 'unsafe' '' "${paths[@]}")" || exit 1
unsafe_matches="$(gate_filter_exclude 'builtins unsafe allowlist' '^crates/builtins-compiler/tests/allocation_tracker.rs:' "$unsafe_matches")" || exit 1
[[ -z "$unsafe_matches" ]] || { printf '%s\n' "$unsafe_matches" >&2; exit 1; }
# The MAX_TRACKS ban lives once, in scripts/check-workspace-policy.sh (P12), which scans the
# whole {crates,hosts,tools,sidecars} tree rather than one of five copies of the same check.
gate_scan_required 'workspace builtins declaration is missing' 'builtins' '' Cargo.toml crates/builtins/Cargo.toml crates/builtins-compiler/Cargo.toml >/dev/null
gate_scan_required 'builtins compiler declarations are missing' 'builtins' '' crates/builtins/Cargo.toml crates/builtins-compiler/Cargo.toml >/dev/null

if [[ "${MISO_ENGINE_BUILTINS_SKIP_METADATA:-0}" != 1 ]]; then
    cargo metadata --no-deps --format-version 1 >/dev/null
fi
printf 'builtins policy: ok\n'
