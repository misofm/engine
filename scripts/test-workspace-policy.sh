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
        "$fixture_root/crates/library/src" \
        "$fixture_root/hosts/binary/src" \
        "$fixture_root/tools" \
        "$fixture_root/sidecars"

    printf '%s\n' \
        '[package]' \
        'name = "library"' \
        '' \
        '[lib]' \
        'name = "library"' \
        '' \
        '[features]' \
        'default = []' \
        >"$fixture_root/crates/library/Cargo.toml"
    printf '//! fixture\n' >"$fixture_root/crates/library/src/lib.rs"

    printf '%s\n' \
        '[package]' \
        'name = "binary"' \
        '' \
        '[[bin]]' \
        'name = "binary"' \
        'path = "src/main.rs"' \
        '' \
        '[features]' \
        'default = []' \
        >"$fixture_root/hosts/binary/Cargo.toml"
    printf 'fn main() {}\n' >"$fixture_root/hosts/binary/src/main.rs"
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

# The miso-engine- prefix convention was retired by the prefix-strip rename
# (docs/rulings/prefix-strip-inventory.md). A package that reintroduces it must still fail --
# this is the regression the gate exists to catch now that the prefix is no longer required.
mutate_package_prefix() {
    local root="$1"
    # `0,/pat/{s/pat/x/}` limits the substitution to the first match in the file (the
    # [package] name line), so it does not also clobber the [lib] name line below it, which
    # is textually identical for a single-word crate name.
    sed -i '0,/name = "library"/{s/name = "library"/name = "miso-engine-library"/}' \
        "$root/crates/library/Cargo.toml"
}

# Same regression, spelled with the underscore form, which the check also forbids explicitly
# (a partially-reverted rename could leave the package name underscored instead of hyphenated).
mutate_package_prefix_underscore_form() {
    local root="$1"
    sed -i '0,/name = "library"/{s/name = "library"/name = "miso_engine_library"/}' \
        "$root/crates/library/Cargo.toml"
}

# The directory basename must equal the package name exactly now that there is no prefix left to
# distinguish "close enough" from correct.
mutate_directory_mismatch() {
    local root="$1"
    sed -i '0,/name = "library"/{s/name = "library"/name = "elsewhere"/}' \
        "$root/crates/library/Cargo.toml"
}

mutate_lib_identifier() {
    local root="$1"
    # Scoped to the [lib] section onward so this does not also clobber the [package] name
    # line above it, which is textually identical for a single-word crate name.
    sed -i '/^\[lib\]$/,$ s/name = "library"/name = "wrong"/' \
        "$root/crates/library/Cargo.toml"
}

mutate_bin_identifier() {
    local root="$1"
    sed -i '/^\[\[bin\]\]$/,$ s/name = "binary"/name = "wrong"/' \
        "$root/hosts/binary/Cargo.toml"
}

allow_secondary_tool_bin() {
    local root="$1"
    create_valid_fixture "$root"
    sed -i '/^\[\[bin\]\]$/,$ s/name = "binary"/name = "binary_probe"/' \
        "$root/hosts/binary/Cargo.toml"
    bash "$policy_script" "$root" >/dev/null
}

mutate_hardware_feature() {
    local root="$1"
    printf 'avx2 = []\n' >>"$root/crates/library/Cargo.toml"
}

mutate_track_limit() {
    local root="$1"
    printf 'const MAX_TRACKS: usize = 64;\n' \
        >>"$root/crates/library/src/lib.rs"
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

# sidecars/<name> is no longer special-cased: it is subject to exactly the same bare-name,
# directory-equals-package-name and sysroot-collision rules as crates/hosts/tools.
allow_sidecar_valid() {
    local root="$1"
    create_valid_fixture "$root"
    mkdir -p "$root/sidecars/flac-decoder/src"
    printf '%s\n' \
        '[package]' \
        'name = "flac-decoder"' \
        '' \
        '[lib]' \
        'name = "flac_decoder"' \
        '' \
        '[features]' \
        'default = []' \
        >"$root/sidecars/flac-decoder/Cargo.toml"
    printf '//! fixture\n' >"$root/sidecars/flac-decoder/src/lib.rs"
    bash "$policy_script" "$root" >/dev/null
}

mutate_sidecar_package_prefix() {
    local root="$1"
    mkdir -p "$root/sidecars/flac-decoder/src"
    printf '%s\n' \
        '[package]' \
        'name = "miso-engine-flac-decoder"' \
        '' \
        '[lib]' \
        'name = "miso_engine_flac_decoder"' \
        '' \
        '[features]' \
        'default = []' \
        >"$root/sidecars/flac-decoder/Cargo.toml"
    printf '//! fixture\n' >"$root/sidecars/flac-decoder/src/lib.rs"
}

# The directory-equals-package-name rule applies regardless of nesting depth: a manifest two
# directories under sidecars/ whose package name does not match its own directory basename must
# still fail, exactly as it would under crates/hosts/tools.
mutate_sidecar_nested_directory_mismatch() {
    local root="$1"
    mkdir -p "$root/sidecars/vendor/anything/src"
    printf '%s\n' \
        '[package]' \
        'name = "vendored-thing"' \
        '' \
        '[lib]' \
        'name = "vendored_thing"' \
        '' \
        '[features]' \
        'default = []' \
        >"$root/sidecars/vendor/anything/Cargo.toml"
    printf '//! fixture\n' >"$root/sidecars/vendor/anything/src/lib.rs"
}

# `core`, and the rest of the Rust sysroot/prelude names, must never be a package name: `core`
# silently shadows the sysroot `core` crate for every dependent (docs/rulings/prefix-strip-inventory.md).
mutate_sysroot_collision() {
    local root="$1"
    mkdir -p "$root/crates/core/src"
    printf '%s\n' \
        '[package]' \
        'name = "core"' \
        '' \
        '[lib]' \
        'name = "core"' \
        '' \
        '[features]' \
        'default = []' \
        >"$root/crates/core/Cargo.toml"
    printf '//! fixture\n' >"$root/crates/core/src/lib.rs"
}

mutate_prelude_collision() {
    local root="$1"
    mkdir -p "$root/crates/std/src"
    printf '%s\n' \
        '[package]' \
        'name = "std"' \
        '' \
        '[lib]' \
        'name = "std"' \
        '' \
        '[features]' \
        'default = []' \
        >"$root/crates/std/Cargo.toml"
    printf '//! fixture\n' >"$root/crates/std/src/lib.rs"
}

valid_root="$scratch_root/valid_root"
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
expect_failure package-prefix-underscore-form mutate_package_prefix_underscore_form
expect_failure directory-mismatch mutate_directory_mismatch
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
expect_failure sidecar-package-prefix mutate_sidecar_package_prefix
expect_failure sidecar-nested-directory-mismatch mutate_sidecar_nested_directory_mismatch
expect_failure sysroot-collision mutate_sysroot_collision
expect_failure prelude-collision mutate_prelude_collision

# `rg` exits 2 (not 1) when a search root does not exist, and `if rg ...; then fail; fi` reads
# both 1 and 2 as "no violation". A fixture whose sidecars/ directory is removed after creation
# must still fail loudly (naming the missing root), not pass as if the scan had run clean.
mutate_missing_sidecars_root() {
    local root="$1"
    rm -rf -- "$root/sidecars"
}

expect_failure missing-sidecars-root mutate_missing_sidecars_root
allow_secondary_tool_bin "$scratch_root/secondary-tool-bin"
allow_approved_isa_pin "$scratch_root/approved-isa-pin"
allow_sidecar_valid "$scratch_root/sidecar-valid"

printf 'workspace policy mutation tests: ok\n'
