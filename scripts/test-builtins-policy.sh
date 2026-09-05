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
rm -rf "$valid/crates/builtins"
if MISO_ENGINE_BUILTINS_SKIP_METADATA=1 bash "$root/scripts/check-builtins-policy.sh" "$valid" >/dev/null 2>&1; then
    printf 'builtins policy missing required source surface escaped\n' >&2; exit 1
fi
make_fixture "$valid"
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

producer="$temp/producer-error"
make_fixture "$producer"
mkdir -p "$temp/rg-fail"
real_rg="$(command -v rg)"
cat >"$temp/rg-fail/rg" <<EOF
#!/usr/bin/env bash
if [[ "\$*" == *'unsafe'* ]]; then printf 'valid partial output\n' >&2; exit 7; fi
exec "$real_rg" "\$@"
EOF
chmod +x "$temp/rg-fail/rg"
producer_output="$(PATH="$temp/rg-fail:$PATH" MISO_ENGINE_BUILTINS_SKIP_METADATA=1 bash "$root/scripts/check-builtins-policy.sh" "$producer" 2>&1)" && producer_rc=0 || producer_rc=$?
[[ "$producer_rc" -ne 0 && "$producer_output" == *'builtins unsafe scan scan errored (rg exit 7)'* && "$producer_output" == *'valid partial output'* ]] || {
    printf 'builtins unsafe producer error escaped: %s\n' "$producer_output" >&2; exit 1;
}
filter_fixture="$temp/filter-error"
make_fixture "$filter_fixture"
mkdir -p "$filter_fixture/crates/builtins-compiler/tests" "$temp/rg-filter-fail"
printf 'unsafe fixture allowance\n' >"$filter_fixture/crates/builtins-compiler/tests/allocation_tracker.rs"
cat >"$temp/rg-filter-fail/rg" <<EOF
#!/usr/bin/env bash
if [[ "\$1" == -v ]]; then cat >/dev/null; printf 'valid partial output\n' >&2; exit 8; fi
exec "$real_rg" "\$@"
EOF
chmod +x "$temp/rg-filter-fail/rg"
filter_output="$(PATH="$temp/rg-filter-fail:$PATH" MISO_ENGINE_BUILTINS_SKIP_METADATA=1 bash "$root/scripts/check-builtins-policy.sh" "$filter_fixture" 2>&1)" && filter_rc=0 || filter_rc=$?
[[ "$filter_rc" -ne 0 && "$filter_output" == *'builtins unsafe allowlist filter errored (rg exit 8)'* && "$filter_output" == *'valid partial output'* ]] || {
    printf 'builtins unsafe filter error escaped: %s\n' "$filter_output" >&2; exit 1;
}

for operation in reverse workspace-positive compiler-positive; do
    fixture="$temp/$operation"; shim="$temp/$operation-bin"; make_fixture "$fixture"; mkdir -p "$shim"
    cat >"$shim/rg" <<EOF
#!/usr/bin/env bash
case '$operation' in
reverse) if [[ "\$*" == *'crates/engine/Cargo.toml crates/session/Cargo.toml crates/graph/Cargo.toml'* ]]; then printf 'crates/engine/Cargo.toml:1:plausible partial\n'; exit 7; fi ;;
workspace-positive) if [[ "\$*" == *'Cargo.toml crates/builtins/Cargo.toml crates/builtins-compiler/Cargo.toml'* ]]; then printf 'Cargo.toml:1:builtins\n'; exit 7; fi ;;
compiler-positive) if [[ "\$*" == *'crates/builtins/Cargo.toml crates/builtins-compiler/Cargo.toml'* && "\$*" != *' Cargo.toml '* ]]; then printf 'crates/builtins/Cargo.toml:1:builtins\n'; exit 7; fi ;;
esac
exec "$real_rg" "\$@"
EOF
    chmod +x "$shim/rg"
    output="$(PATH="$shim:$PATH" MISO_ENGINE_BUILTINS_SKIP_METADATA=1 bash "$root/scripts/check-builtins-policy.sh" "$fixture" 2>&1)" && rc=0 || rc=$?
    case "$operation" in
        reverse) expected='reverse dependency scan errored (rg exit 7)' ;;
        workspace-positive) expected='workspace builtins declaration is missing search failed (rg exit 7)' ;;
        compiler-positive) expected='builtins compiler declarations are missing search failed (rg exit 7)' ;;
    esac
    [[ "$rc" -ne 0 && "$output" == *"$expected"* && "$output" == *'Cargo.toml'* ]] || {
        printf 'builtins %s search error escaped: %s\n' "$operation" "$output" >&2; exit 1;
    }
done

printf 'builtins policy mutations: ok\n'
