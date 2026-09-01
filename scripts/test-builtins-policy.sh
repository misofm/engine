#!/usr/bin/env bash
set -euo pipefail

root="$(cd "${1:-.}" && pwd)"
temp="$(mktemp -d)"
trap 'rm -rf -- "$temp"' EXIT
make_fixture() {
    local fixture="$1"
    mkdir -p "$fixture/crates/builtins/src" \
        "$fixture/crates/builtins-compiler/src" \
        "$fixture/crates/engine" \
        "$fixture/crates/effect-contract" \
        "$fixture/crates/session" \
        "$fixture/crates/graph" \
        "$fixture/crates/rack"
    printf '[workspace]\nmembers = []\n' >"$fixture/Cargo.toml"
    printf '[package]\nname = "builtins"\n[lib]\nname = "builtins"\n[dependencies]\nengine.workspace = true\neffect-contract.workspace = true\nlane.workspace = true\nmath.workspace = true\n' >"$fixture/crates/builtins/Cargo.toml"
    printf '[package]\nname = "builtins-compiler"\n[lib]\nname = "builtins_compiler"\n[dependencies]\nbuiltins.workspace = true\nengine.workspace = true\neffect-contract.workspace = true\ngraph.workspace = true\nlane.workspace = true\nrack.workspace = true\nrack-compiler.workspace = true\nsession.workspace = true\nsha2.workspace = true\n' >"$fixture/crates/builtins-compiler/Cargo.toml"
    printf '[package]\nname = "engine"\n' >"$fixture/crates/engine/Cargo.toml"
    printf '[package]\nname = "effect-contract"\n' >"$fixture/crates/effect-contract/Cargo.toml"
    printf '[package]\nname = "session"\n' >"$fixture/crates/session/Cargo.toml"
    printf '[package]\nname = "graph"\n' >"$fixture/crates/graph/Cargo.toml"
    printf '[package]\nname = "rack"\n' >"$fixture/crates/rack/Cargo.toml"
    printf '//! fixture\n' >"$fixture/crates/builtins/src/lib.rs"
    printf '//! fixture\n' >"$fixture/crates/builtins-compiler/src/lib.rs"
}

valid="$temp/valid"
make_fixture "$valid"
MISO_ENGINE_BUILTINS_SKIP_METADATA=1 bash "$root/scripts/check-builtins-policy.sh" "$valid" >/dev/null
printf 'unsafe\n' >>"$valid/crates/builtins/src/lib.rs"
if MISO_ENGINE_BUILTINS_SKIP_METADATA=1 bash "$root/scripts/check-builtins-policy.sh" "$valid" >/dev/null 2>&1; then
    printf 'builtins policy mutation escaped\n' >&2
    exit 1
fi
# The compiler's dependency list is a pinned boundary: dropping the planner edge must fail.
missing_planner="$temp/missing-planner"
make_fixture "$missing_planner"
sed -i '/^rack-compiler\.workspace/d' \
    "$missing_planner/crates/builtins-compiler/Cargo.toml"
if MISO_ENGINE_BUILTINS_SKIP_METADATA=1 bash "$root/scripts/check-builtins-policy.sh" "$missing_planner" >/dev/null 2>&1; then
    printf 'builtins policy mutation escaped: compiler dependency boundary\n' >&2
    exit 1
fi

printf 'builtins policy mutations: ok\n'
