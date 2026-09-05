#!/usr/bin/env bash
# Guard the Issue-008 rack's narrow, render-reachable dependency and safety boundary.
set -euo pipefail

root=$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)
script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_directory/lib/gate.sh"
cd "$root"

fail() {
    GATE_FAILURE_PREFIX='rack policy failure' gate_fail "$1"
    exit 1
}

dependencies() {
    gate_toml_dependencies "$1"
}

rack_manifest=crates/rack/Cargo.toml
compiler_manifest=crates/rack-compiler/Cargo.toml
[[ -f "$rack_manifest" && -f "$compiler_manifest" ]] || fail 'missing rack manifests'
if ! rack_dependencies="$(dependencies "$rack_manifest")"; then fail 'rack dependency extraction failed'; fi
[[ "$rack_dependencies" == $'effect-contract\nengine' ]] || fail 'rack render dependency boundary changed'
if ! compiler_dependencies="$(dependencies "$compiler_manifest")"; then fail 'rack compiler dependency extraction failed'; fi
[[ "$compiler_dependencies" == $'effect-contract\nengine\nrack' ]] || fail 'rack compiler dependency boundary changed'

# The MAX_TRACKS ban lives once, in scripts/check-workspace-policy.sh, which scans the whole
# {crates,hosts,tools,sidecars} tree -- rack/rack-compiler included -- rather than five copies
# of the same regex over five different root lists.
GATE_FAILURE_PREFIX='rack policy failure' gate_scan_forbidden 'rack source has unsafe code' \
    '\bunsafe\b' '*.rs' crates/rack crates/rack-compiler || exit 1
GATE_FAILURE_PREFIX='rack policy failure' gate_scan_forbidden \
    'control-plane, I/O, threading, synchronization, or logging leaked into rack render code' \
    '\b(session|effect_compiler|graph|builtins)::|std::(fs|net|thread|sync)|log::|tracing::' '' \
    crates/rack/src crates/rack/Cargo.toml || exit 1
GATE_FAILURE_PREFIX='rack policy failure' gate_scan_forbidden \
    'feature detection or target-feature specialization leaked out of core dispatch' \
    'target_feature|is_x86_feature_detected!' '' crates/rack crates/rack-compiler || exit 1

printf 'rack policy: PASS\n'
