#!/usr/bin/env bash
# Mutation tests proving that the workspace-policy guard rejects each protected failure class.
set -euo pipefail

script_directory="$(cd "$(dirname "$0")" && pwd)"
policy_script="$script_directory/check-workspace-policy.sh"
scratch_root="$(mktemp -d)"
trap 'rm -rf -- "$scratch_root"' EXIT

create_valid_fixture() {
    local fixture_root="$1"
    mkdir -p \
        "$fixture_root/crates/miso-engine-library with spaces/src" \
        "$fixture_root/hosts/miso-engine-binary/src" \
        "$fixture_root/tools"

    printf '%s\n' \
        '[package]' \
        'name = "miso-engine-library"' \
        '' \
        '[lib]' \
        'name = "miso_engine_library"' \
        '' \
        '[features]' \
        'default = []' \
        >"$fixture_root/crates/miso-engine-library with spaces/Cargo.toml"
    printf '//! fixture\n' >"$fixture_root/crates/miso-engine-library with spaces/src/lib.rs"

    printf '%s\n' \
        '[package]' \
        'name = "miso-engine-binary"' \
        '' \
        '[[bin]]' \
        'name = "miso_engine_binary"' \
        'path = "src/main.rs"' \
        '' \
        '[features]' \
        'default = []' \
        >"$fixture_root/hosts/miso-engine-binary/Cargo.toml"
    printf 'fn main() {}\n' >"$fixture_root/hosts/miso-engine-binary/src/main.rs"
}

expect_failure() {
    local fixture_name="$1"
    local fixture_root="$scratch_root/$fixture_name"
    shift
    create_valid_fixture "$fixture_root"
    "$@" "$fixture_root"

    if bash "$policy_script" "$fixture_root" >/dev/null 2>&1; then
        printf 'policy mutation unexpectedly passed: %s\n' "$fixture_name" >&2
        exit 1
    fi
}

mutate_package_prefix() {
    local root="$1"
    sed -i 's/name = "miso-engine-library"/name = "engine-library"/' \
        "$root/crates/miso-engine-library with spaces/Cargo.toml"
}

mutate_lib_identifier() {
    local root="$1"
    sed -i 's/name = "miso_engine_library"/name = "miso_engine_wrong"/' \
        "$root/crates/miso-engine-library with spaces/Cargo.toml"
}

mutate_bin_identifier() {
    local root="$1"
    sed -i 's/name = "miso_engine_binary"/name = "miso_engine_wrong"/' \
        "$root/hosts/miso-engine-binary/Cargo.toml"
}

mutate_hardware_feature() {
    local root="$1"
    printf 'avx2 = []\n' >>"$root/crates/miso-engine-library with spaces/Cargo.toml"
}

mutate_track_limit() {
    local root="$1"
    printf 'const MAX_TRACKS: usize = 64;\n' \
        >>"$root/crates/miso-engine-library with spaces/src/lib.rs"
}

mutate_global_isa() {
    local root="$1"
    mkdir -p "$root/.cargo"
    printf '%s\n' '[build]' 'rustflags = ["-C", "target-cpu=native"]' \
        >"$root/.cargo/config.toml"
}

valid_root="$scratch_root/valid root"
create_valid_fixture "$valid_root"
bash "$policy_script" "$valid_root" >/dev/null

expect_failure package-prefix mutate_package_prefix
expect_failure lib-identifier mutate_lib_identifier
expect_failure bin-identifier mutate_bin_identifier
expect_failure hardware-feature mutate_hardware_feature
expect_failure track-limit mutate_track_limit
expect_failure global-isa mutate_global_isa

printf 'workspace policy mutation tests: ok\n'
