#!/usr/bin/env bash
# Mutation tests proving the host-facade policy (audit #103 F1) is enforced, not decorative.
set -euo pipefail

script_directory="$(cd "$(dirname "$0")" && pwd)"
policy_script="$script_directory/check-host-core-policy.sh"
scratch_root="$(mktemp -d)"
trap 'rm -rf -- "$scratch_root"' EXIT

create_fixture() {
    local root="$1"
    mkdir -p "$root/crates/host-core/src" \
        "$root/crates/capi/src" \
        "$root/hosts/host-web/src" \
        "$root/hosts/host-native/src"
    printf '%s\n' \
        '[package]' \
        'name = "host-core"' \
        '' \
        '[lib]' \
        'crate-type = ["rlib"]' \
        '' \
        '[dependencies]' \
        'graph.workspace = true' \
        >"$root/crates/host-core/Cargo.toml"
    printf '%s\n' \
        'pub fn prepare_host_runtime() {}' \
        >"$root/crates/host-core/src/lib.rs"
    printf '%s\n' \
        'fn compile_children() { host_core::prepare_host_runtime(); }' \
        >"$root/crates/capi/src/runtime.rs"
    # host-web is no longer exempt (issue #106 is done: it depends on host-core like every
    # other host) and must use the facade exactly as host-native does below.
    printf '%s\n' \
        'fn compile() { host_core::prepare_host_runtime(); }' \
        >"$root/hosts/host-web/src/lib.rs"
    printf '%s\n' \
        'fn compile() { host_core::prepare_host_runtime(); }' \
        >"$root/hosts/host-native/src/main.rs"
}

expect_failure() {
    local name="$1"
    local mutation="$2"
    local root="$scratch_root/$name"
    create_fixture "$root"
    eval "$mutation"
    if bash "$policy_script" "$root" >/dev/null 2>&1; then
        printf 'host-core policy mutation unexpectedly passed: %s\n' "$name" >&2
        exit 1
    fi
}

valid="$scratch_root/valid"
create_fixture "$valid"
bash "$policy_script" "$valid" >/dev/null

expect_failure capi-recompiles-the-pipeline \
    'printf "%s\n" "fn x() { let _ = compile_session(&model, caps); }" >>"$root/crates/capi/src/runtime.rs"'
expect_failure host-recompiles-the-pipeline \
    'printf "%s\n" "fn x() { let _ = prepare_session_builtins(&compiled, &[], caps); }" >>"$root/hosts/host-native/src/main.rs"'
expect_failure host-binds-its-own-source-set \
    'printf "%s\n" "fn x() { let _ = artifact.into_bound_with_source_set(bindings, set); }" >>"$root/hosts/host-native/src/main.rs"'
expect_failure capi-reinvents-the-identity-processor \
    'printf "%s\n" "struct IdentityProcessor;" >>"$root/crates/capi/src/runtime.rs"'
expect_failure host-reinvents-the-identity-processor \
    'printf "%s\n" "impl GraphRuntimeProcessor for Passthrough {}" >>"$root/hosts/host-native/src/main.rs"'
expect_failure capi-hand-decodes-the-control-wire \
    'printf "%s\n" "const MAGIC: &[u8] = b\"MISOCTL\";" >>"$root/crates/capi/src/runtime.rs"'
expect_failure capi-reimplements-the-replay-cache \
    'printf "%s\n" "struct ReplayEntryRecord { id: u64 }" >>"$root/crates/capi/src/runtime.rs"'
expect_failure facade-depends-on-the-protocol \
    'printf "%s\n" "protocol.workspace = true" >>"$root/crates/host-core/Cargo.toml"'
expect_failure facade-exports-a-c-symbol \
    'printf "%s\n" "#[unsafe(no_mangle)] pub extern \"C\" fn miso_engine_v1_prepare() {}" >>"$root/crates/host-core/src/lib.rs"'
expect_failure facade-becomes-a-cdylib \
    'sed -i "s/crate-type = \[\"rlib\"\]/crate-type = [\"rlib\", \"cdylib\"]/" "$root/crates/host-core/Cargo.toml"'

# Issue #106/#359: host-web is no longer exempt. The stale "pending #106" allowance is gone, so
# host-web recompiling the pipeline directly must fail exactly like any other host.
expect_failure host-web-recompiles-the-pipeline \
    'printf "%s\n" "fn x() { let _ = compile_with_builtins(&model, caps); }" >>"$root/hosts/host-web/src/lib.rs"'
expect_failure host-web-reinvents-the-identity-processor \
    'printf "%s\n" "struct IdentityBinding;" >>"$root/hosts/host-web/src/lib.rs"'

# S9: `! rg ...` fails open on rg exit 2 (a missing search root), and `[ -d "$host" ] || continue`
# silently drops a host whose src/ directory disappears -- both must now be hard failures rather
# than a vacuous "ok".
expect_failure host-web-src-directory-deleted \
    'rm -rf -- "$root/hosts/host-web/src"'
expect_failure capi-src-directory-deleted \
    'rm -rf -- "$root/crates/capi/src"'

printf 'host-core policy mutation tests: ok\n'
