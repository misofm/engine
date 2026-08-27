#!/usr/bin/env bash
set -euo pipefail
root="$(cd "${1:-.}" && pwd)"
temp="$(mktemp -d)"
trap 'rm -rf -- "$temp"' EXIT
cp -R "$root/crates" "$root/hosts" "$root/fixtures" "$root/scripts" "$root/docs" \
    "$root/tools" "$temp/"

check() { bash "$temp/scripts/check-effect-interchange-qualification.sh" "$temp" >/dev/null; }
expect_failure() {
    if check 2>/dev/null; then
        printf 'effect interchange policy mutation escaped: %s\n' "$1" >&2
        exit 1
    fi
}

check
mkdir -p "$temp/sdk/assets"
printf 'deliberate stray wasm\n' >"$temp/sdk/assets/not-the-packaged-engine.wasm"
expect_failure generated-artifact
rm "$temp/sdk/assets/not-the-packaged-engine.wasm"

printf 'changed\n' >>"$temp/fixtures/effect-state/v1/canonical.state.hex"
expect_failure baseline
cp "$root/fixtures/effect-state/v1/canonical.state.hex" \
    "$temp/fixtures/effect-state/v1/canonical.state.hex"

printf 'changed\n' >>"$temp/fixtures/effect-state/v1/canonical.state.hex"
replacement="$(sha256sum "$temp/fixtures/effect-state/v1/canonical.state.hex" | awk '{print $1}')"
sed -i \
    "s/^4d00a6c3661d119dcf62d16e6c72a68a5f12283397610cfcf18ece7471a2b014 /$replacement /" \
    "$temp/fixtures/effect-interchange/v1/ACCEPTED.sha256"
expect_failure refreshed-baseline
cp "$root/fixtures/effect-state/v1/canonical.state.hex" \
    "$temp/fixtures/effect-state/v1/canonical.state.hex"
cp "$root/fixtures/effect-interchange/v1/ACCEPTED.sha256" \
    "$temp/fixtures/effect-interchange/v1/ACCEPTED.sha256"

printf '\nmiso-engine-bench.workspace = true\n' \
    >>"$temp/crates/miso-engine-core/Cargo.toml"
expect_failure production-dependency
cp "$root/crates/miso-engine-core/Cargo.toml" "$temp/crates/miso-engine-core/Cargo.toml"

printf '\npub fn effect_interchange_qualification_render_leak() {}\n' \
    >>"$temp/crates/miso-engine-core/src/realtime/plan.rs"
expect_failure render-reachability
cp "$root/crates/miso-engine-core/src/realtime/plan.rs" \
    "$temp/crates/miso-engine-core/src/realtime/plan.rs"

printf '\n#[unsafe(no_mangle)] pub extern "C" fn miso_engine_effect_state_v1_new_export() {}\n' \
    >>"$temp/crates/miso-engine-effect-package/src/ffi.rs"
expect_failure new-export
cp "$root/crates/miso-engine-effect-package/src/ffi.rs" \
    "$temp/crates/miso-engine-effect-package/src/ffi.rs"

sed -i 's/TRIALS: usize = 10_000/TRIALS: usize = 9_999/' \
    "$temp/crates/miso-engine-effect-package/tests/effect_interchange_mutation.rs"
expect_failure mutation-count
cp "$root/crates/miso-engine-effect-package/tests/effect_interchange_mutation.rs" \
    "$temp/crates/miso-engine-effect-package/tests/effect_interchange_mutation.rs"

sed -i 's/const OBSERVATIONS: usize = 256/const OBSERVATIONS: usize = 255/' \
    "$temp/tools/miso-engine-bench/src/effect_interchange.rs"
expect_failure benchmark-observations
printf 'effect interchange qualification policy mutations: ok\n'
