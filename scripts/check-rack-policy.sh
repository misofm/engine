#!/usr/bin/env bash
# Guard the Issue-008 rack's narrow, render-reachable dependency and safety boundary.
set -euo pipefail

root=$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)
cd "$root"

fail() {
    printf 'rack policy failure: %s\n' "$1" >&2
    exit 1
}

dependencies() {
    awk '
        /^\[dependencies\]$/ { in_dependencies = 1; next }
        /^\[/ { in_dependencies = 0 }
        in_dependencies && /^[A-Za-z0-9_-]+(\.workspace)?[[:space:]]*=/ {
            value = $1
            sub(/\.workspace$/, "", value)
            print value
        }
    ' "$1" | sort
}

rack_manifest=crates/rack/Cargo.toml
compiler_manifest=crates/rack-compiler/Cargo.toml
[[ -f "$rack_manifest" && -f "$compiler_manifest" ]] || fail 'missing rack manifests'
[[ "$(dependencies "$rack_manifest")" == $'effect-contract\nengine' ]] ||
    fail 'rack render dependency boundary changed'
[[ "$(dependencies "$compiler_manifest")" == $'effect-contract\nengine\nrack' ]] ||
    fail 'rack compiler dependency boundary changed'

if rg -n '\bunsafe\b' crates/rack crates/rack-compiler --glob '*.rs'; then
    fail 'rack source has unsafe code (compiled track ceilings are check-workspace-policy.sh'"'"'s job)'
fi
if rg -n '\b(session|effect_compiler|graph|builtins)::|std::(fs|net|thread|sync)|log::|tracing::' \
    crates/rack/src crates/rack/Cargo.toml; then
    fail 'control-plane, I/O, threading, synchronization, or logging leaked into rack render code'
fi
if rg -n 'target_feature|is_x86_feature_detected!' crates/rack crates/rack-compiler; then
    fail 'feature detection or target-feature specialization leaked out of core dispatch'
fi

printf 'rack policy: PASS\n'
