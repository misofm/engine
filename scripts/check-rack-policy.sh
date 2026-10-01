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
[[ "$compiler_dependencies" == $'effect-contract\nrack' ]] || fail 'rack compiler dependency boundary changed'

# The MAX_TRACKS ban lives once, in scripts/check-workspace-policy.sh, which scans the whole
# {crates,hosts,tools} tree -- rack/rack-compiler included -- rather than five copies
# of the same regex over five different root lists.
GATE_FAILURE_PREFIX='rack policy failure' gate_scan_forbidden 'rack source has unsafe code' \
    '\bunsafe\b' '*.rs' crates/rack crates/rack-compiler || exit 1
GATE_FAILURE_PREFIX='rack policy failure' gate_scan_forbidden \
    'control-plane, I/O, threading, synchronization, or logging leaked into rack render code' \
    '\b(session|effect_compiler|graph|builtins)::|std::(fs|net|thread|sync)|log::|tracing::' '' \
    crates/rack/src crates/rack/Cargo.toml || exit 1
# Feature detection and target-feature specialization stay in core dispatch. One spelling is
# exempt (issue #1112): the 8-lane (AVX2) width predicate itself, on a line of its own exactly as
# `#[cfg(target_feature = "avx2")]`. Rack's eight-lane bank views exist only where
# `lane::Simd8` and `effect_contract::BankWidth::Eight` do, so they carry the same attribute; it
# names a lane width, not an instruction set, and every other target-feature use still fails.
width_predicate='^[^:]+:[0-9]+:[[:space:]]*#\[cfg\(target_feature = "avx2"\)\]$'
if feature_hits="$(rg -n 'target_feature|is_x86_feature_detected!' crates/rack crates/rack-compiler 2>&1)"; then
    rc=0
else
    rc=$?
fi
case "$rc" in
    0) feature_hits="$(grep -Ev "$width_predicate" <<<"$feature_hits" || true)" ;;
    1) feature_hits='' ;;
    *) printf '%s\n' "$feature_hits" >&2; fail "feature detection scan errored (rg exit $rc)" ;;
esac
[[ -z "$feature_hits" ]] || {
    printf '%s\n' "$feature_hits" >&2
    fail 'feature detection or target-feature specialization leaked out of core dispatch'
}

printf 'rack policy: PASS\n'
