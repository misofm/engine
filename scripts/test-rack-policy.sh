#!/usr/bin/env bash
# Mutation probes for the narrow Issue-008 rack policy.
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
policy="$root/scripts/check-rack-policy.sh"
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT

make_fixture() {
    local fixture=$1
    mkdir -p "$fixture/crates/miso-engine-rack/src" "$fixture/crates/miso-engine-rack-compiler/src"
    printf '[workspace]\nmembers = []\n' >"$fixture/Cargo.toml"
    printf '[package]\nname = "miso-engine-rack"\n[dependencies]\nmiso-engine-core.workspace = true\nmiso-engine-effect-contract.workspace = true\n' >"$fixture/crates/miso-engine-rack/Cargo.toml"
    printf '[package]\nname = "miso-engine-rack-compiler"\n[dependencies]\nmiso-engine-core.workspace = true\nmiso-engine-effect-contract.workspace = true\nmiso-engine-rack.workspace = true\n' >"$fixture/crates/miso-engine-rack-compiler/Cargo.toml"
    printf '//! fixture\n' >"$fixture/crates/miso-engine-rack/src/lib.rs"
    printf '//! fixture\n' >"$fixture/crates/miso-engine-rack-compiler/src/lib.rs"
}

expect_failure() {
    local name=$1
    local mutation=$2
    local fixture="$scratch/$name"
    make_fixture "$fixture"
    eval "$mutation"
    if bash "$policy" "$fixture" >/dev/null 2>&1; then
        printf 'rack policy mutation unexpectedly passed: %s\n' "$name" >&2
        exit 1
    fi
}

valid="$scratch/valid"
make_fixture "$valid"
bash "$policy" "$valid" >/dev/null
expect_failure unsafe 'printf "unsafe fn bad() {}\n" >>"$fixture/crates/miso-engine-rack/src/lib.rs"'
expect_failure track-limit 'printf "const MAX_TRACKS: usize = 1;\n" >>"$fixture/crates/miso-engine-rack/src/lib.rs"'
expect_failure dependency 'printf "miso-engine-session.workspace = true\n" >>"$fixture/crates/miso-engine-rack/Cargo.toml"'
printf 'rack policy mutations: ok\n'
