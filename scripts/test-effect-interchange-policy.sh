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

printf '\nbench.workspace = true\n' \
    >>"$temp/crates/engine/Cargo.toml"
expect_failure production-dependency
cp "$root/crates/engine/Cargo.toml" "$temp/crates/engine/Cargo.toml"

printf '\npub fn effect_interchange_qualification_render_leak() {}\n' \
    >>"$temp/crates/engine/src/realtime/plan.rs"
expect_failure render-reachability
cp "$root/crates/engine/src/realtime/plan.rs" \
    "$temp/crates/engine/src/realtime/plan.rs"

printf '\n#[unsafe(no_mangle)] pub extern "C" fn miso_engine_effect_state_v1_new_export() {}\n' \
    >>"$temp/crates/effect-package/src/ffi.rs"
expect_failure new-export
cp "$root/crates/effect-package/src/ffi.rs" \
    "$temp/crates/effect-package/src/ffi.rs"

sed -i 's/campaigns(4)/campaigns(3)/' \
    "$temp/crates/effect-package/tests/effect_interchange_mutation.rs"
expect_failure mutation-count
cp "$root/crates/effect-package/tests/effect_interchange_mutation.rs" \
    "$temp/crates/effect-package/tests/effect_interchange_mutation.rs"

sed -i 's/fn tiny_deterministic_mutation_smoke/fn tiny_deterministic_mutation_smoke_renamed/' \
    "$temp/crates/effect-package/tests/effect_interchange_mutation.rs"
expect_failure mutation-smoke-renamed
cp "$root/crates/effect-package/tests/effect_interchange_mutation.rs" \
    "$temp/crates/effect-package/tests/effect_interchange_mutation.rs"

sed -i 's/const OBSERVATIONS: usize = 256/const OBSERVATIONS: usize = 255/' \
    "$temp/tools/bench/src/effect_interchange.rs"
expect_failure benchmark-observations
cp "$root/tools/bench/src/effect_interchange.rs" \
    "$temp/tools/bench/src/effect_interchange.rs"

# B2: the qualification gate must police the real matrix in check-cross-targets.sh, not a
# decorative copy of its literals -- dropping a required target triple must fail. #378 retired the
# aarch64 rows this used to mutate (owner ruling: native AArch64 is unsupported, no claim); the
# equivalent mutation on a remaining target is erasing wasm32-unknown-unknown everywhere it
# appears in the real matrix, which the qualification gate's target-row loop must still catch.
sed -i 's/wasm32-unknown-unknown/wasm-target-erased/g' \
    "$temp/scripts/check-cross-targets.sh"
expect_failure cross-target-dropped-wasm
cp "$root/scripts/check-cross-targets.sh" "$temp/scripts/check-cross-targets.sh"

# B2: turning the Wasm simd leg scalar (dropping the +simd128 feature row from the real matrix)
# must also fail.
sed -i 's/feature=+simd128/feature=-simd128/' \
    "$temp/scripts/check-cross-targets.sh"
expect_failure cross-target-simd-leg-scalar
cp "$root/scripts/check-cross-targets.sh" "$temp/scripts/check-cross-targets.sh"

printf 'effect interchange qualification policy mutations: ok\n'

# Required CI reaches the successor authority and the hermetic lifecycle from this original
# repository root. Both suites use their own scratch roots; neither is run against a mutation.
bash "$root/scripts/test-effect-interchange-benchmark-108-policy.sh"
bash "$root/scripts/test-effect-interchange-benchmark.sh"
