#!/usr/bin/env bash
set -euo pipefail

root="$(cd "${1:-.}" && pwd)"
temp="$(mktemp -d)"
trap 'rm -rf -- "$temp"' EXIT
make_fixture() {
    local fixture="$1"
    mkdir -p "$fixture/crates/miso-engine-builtins/src" \
        "$fixture/crates/miso-engine-builtins-compiler/src" \
        "$fixture/crates/miso-engine-core" \
        "$fixture/crates/miso-engine-session" \
        "$fixture/crates/miso-engine-graph"
    printf '[workspace]\nmembers = []\n' >"$fixture/Cargo.toml"
    printf '[package]\nname = "miso-engine-builtins"\n[lib]\nname = "miso_engine_builtins"\n[dependencies]\nmiso-engine-core.workspace = true\n' >"$fixture/crates/miso-engine-builtins/Cargo.toml"
    printf '[package]\nname = "miso-engine-builtins-compiler"\n[lib]\nname = "miso_engine_builtins_compiler"\n[dependencies]\nmiso-engine-builtins.workspace = true\nmiso-engine-core.workspace = true\nmiso-engine-graph.workspace = true\nmiso-engine-session.workspace = true\n' >"$fixture/crates/miso-engine-builtins-compiler/Cargo.toml"
    printf '[package]\nname = "miso-engine-core"\n' >"$fixture/crates/miso-engine-core/Cargo.toml"
    printf '[package]\nname = "miso-engine-session"\n' >"$fixture/crates/miso-engine-session/Cargo.toml"
    printf '[package]\nname = "miso-engine-graph"\n' >"$fixture/crates/miso-engine-graph/Cargo.toml"
    printf '//! fixture\n' >"$fixture/crates/miso-engine-builtins/src/lib.rs"
    printf '//! fixture\n' >"$fixture/crates/miso-engine-builtins-compiler/src/lib.rs"
}

valid="$temp/valid"
make_fixture "$valid"
MISO_ENGINE_BUILTINS_SKIP_METADATA=1 bash "$root/scripts/check-builtins-policy.sh" "$valid" >/dev/null
printf 'unsafe\n' >>"$valid/crates/miso-engine-builtins/src/lib.rs"
if MISO_ENGINE_BUILTINS_SKIP_METADATA=1 bash "$root/scripts/check-builtins-policy.sh" "$valid" >/dev/null 2>&1; then
    printf 'builtins policy mutation escaped\n' >&2
    exit 1
fi
printf 'builtins policy mutations: ok\n'
