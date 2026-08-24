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

allow_secondary_tool_bin() {
    local root="$1"
    create_valid_fixture "$root"
    sed -i 's/name = "miso_engine_binary"/name = "miso_engine_binary_probe"/' \
        "$root/hosts/miso-engine-binary/Cargo.toml"
    bash "$policy_script" "$root" >/dev/null
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

mutate_unscoped_isa_pin() {
    local root="$1"
    mkdir -p "$root/.cargo"
    printf '%s\n' '[build]' 'rustflags = ["-C", "target-feature=+avx2,+fma"]' \
        >"$root/.cargo/config.toml"
}

mutate_extra_isa_feature() {
    local root="$1"
    mkdir -p "$root/.cargo"
    printf '%s\n' "[target.'cfg(target_arch = \"x86_64\")']" \
        'rustflags = ["-C", "target-feature=+avx2,+fma,+avx512f"]' \
        >"$root/.cargo/config.toml"
}

allow_approved_isa_pin() {
    local root="$1"
    create_valid_fixture "$root"
    mkdir -p "$root/.cargo"
    printf '%s\n' \
        '# Master plan #83 D4: the approved x86-64-v3 pin, with target-feature in a comment.' \
        "[target.'cfg(target_arch = \"x86_64\")']" \
        'rustflags = ["-C", "target-feature=+avx2,+fma"]' \
        >"$root/.cargo/config.toml"
    bash "$policy_script" "$root" >/dev/null
}

valid_root="$scratch_root/valid root"
create_valid_fixture "$valid_root"
bash "$policy_script" "$valid_root" >/dev/null

# #104 phase D: a cargo target-dir spill at the workspace root.
mutate_root_target_spill() {
    local root="$1"
    mkdir -p "$root/release/.fingerprint"
    printf '{}\n' >"$root/release/.fingerprint/lib-example.json"
}

mutate_root_rustc_info() {
    local root="$1"
    printf '{}\n' >"$root/.rustc_info.json"
}

mutate_root_cachedir_tag() {
    local root="$1"
    printf 'Signature: 8a477f597d28d172789f06886806bc55\n' >"$root/CACHEDIR.TAG"
}

expect_failure package-prefix mutate_package_prefix
expect_failure lib-identifier mutate_lib_identifier
expect_failure bin-identifier mutate_bin_identifier
expect_failure hardware-feature mutate_hardware_feature
expect_failure track-limit mutate_track_limit
expect_failure global-isa mutate_global_isa
expect_failure unscoped-isa-pin mutate_unscoped_isa_pin
expect_failure extra-isa-feature mutate_extra_isa_feature
expect_failure root-target-spill mutate_root_target_spill
expect_failure root-rustc-info mutate_root_rustc_info
expect_failure root-cachedir-tag mutate_root_cachedir_tag
allow_secondary_tool_bin "$scratch_root/secondary-tool-bin"
allow_approved_isa_pin "$scratch_root/approved-isa-pin"

printf 'workspace policy mutation tests: ok\n'
