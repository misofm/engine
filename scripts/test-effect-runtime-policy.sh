#!/usr/bin/env bash
set -euo pipefail
root="$(cd "${1:-.}" && pwd)"
temp="$(mktemp -d)"
trap 'rm -rf -- "$temp"' EXIT
cp -R "$root/crates" "$root/docs" "$root/scripts" "$temp/"
compiler_manifest="$temp/crates/effect-compiler/Cargo.toml"
contract_manifest="$temp/crates/effect-contract/Cargo.toml"

bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null

mv "$temp/crates/effect-compiler/src" "$temp/crates/effect-compiler/src.saved"
missing_root_output="$(bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" 2>&1)" && missing_root_rc=0 || missing_root_rc=$?
[[ "$missing_root_rc" -ne 0 && "$missing_root_output" == *'missing search path(s)'*'crates/effect-compiler/src'* ]] || {
    printf 'effect runtime missing source root escaped: %s\n' "$missing_root_output" >&2; exit 1;
}
mv "$temp/crates/effect-compiler/src.saved" "$temp/crates/effect-compiler/src"

restore_compiler_manifest() {
    cp "$root/crates/effect-compiler/Cargo.toml" "$compiler_manifest"
}

expect_dependency_failure() {
    local mutation="$1"
    if bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null 2>&1; then
        printf 'effect runtime dependency mutation escaped: %s\n' "$mutation" >&2
        exit 1
    fi
    restore_compiler_manifest
}

sed -i '/^\[dependencies\]$/a graph.workspace = true' "$compiler_manifest"
expect_dependency_failure arbitrary-extra

sed -i '/^parametric-eq[.]workspace = true$/d' "$compiler_manifest"
expect_dependency_failure missing-parametric-eq
sed -i 's/^parametric-eq[.]workspace = true$/graph.workspace = true/' "$compiler_manifest"
expect_dependency_failure substituted-parametric-eq
sed -i '/^compressor[.]workspace = true$/d' "$compiler_manifest"
expect_dependency_failure missing-compressor
sed -i 's/^compressor[.]workspace = true$/graph.workspace = true/' "$compiler_manifest"
expect_dependency_failure substituted-compressor
sed -i '/^gate-expander[.]workspace = true$/d' "$compiler_manifest"
expect_dependency_failure missing-gate-expander
sed -i 's/^gate-expander[.]workspace = true$/graph.workspace = true/' "$compiler_manifest"
expect_dependency_failure substituted-gate-expander
sed -i '/^multiband-compressor[.]workspace = true$/d' "$compiler_manifest"
expect_dependency_failure missing-multiband-compressor
sed -i 's/^multiband-compressor[.]workspace = true$/graph.workspace = true/' "$compiler_manifest"
expect_dependency_failure substituted-multiband-compressor
sed -i '/^true-peak-limiter[.]workspace = true$/d' "$compiler_manifest"
expect_dependency_failure missing-true-peak-limiter
sed -i 's/^true-peak-limiter[.]workspace = true$/graph.workspace = true/' "$compiler_manifest"
expect_dependency_failure substituted-true-peak-limiter
sed -i '/^soft-clip[.]workspace = true$/d' "$compiler_manifest"
expect_dependency_failure missing-soft-clip
sed -i 's/^soft-clip[.]workspace = true$/graph.workspace = true/' "$compiler_manifest"
expect_dependency_failure substituted-soft-clip
sed -i '/^transient-shaper[.]workspace = true$/d' "$compiler_manifest"
expect_dependency_failure missing-transient-shaper
sed -i 's/^transient-shaper[.]workspace = true$/graph.workspace = true/' "$compiler_manifest"
expect_dependency_failure substituted-transient-shaper
sed -i '/^delay[.]workspace = true$/d' "$compiler_manifest"
expect_dependency_failure missing-delay
sed -i 's/^delay[.]workspace = true$/graph.workspace = true/' "$compiler_manifest"
expect_dependency_failure substituted-delay

for reverse_dependency in engine session; do
    reverse_manifest="$temp/crates/$reverse_dependency/Cargo.toml"
    printf '\neffect-contract.workspace = true\n' >>"$reverse_manifest"
    if bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null 2>&1; then
        printf 'effect runtime reverse-dependency mutation escaped: %s\n' \
            "$reverse_dependency" >&2
        exit 1
    fi
    cp "$root/crates/$reverse_dependency/Cargo.toml" "$reverse_manifest"
done

# Issue #95: the contract's dependency boundary is `engine` plus `math` and
# nothing else. `math` is there because decision D6 forbids the platform libm, and it
# must stay; `lane` must never appear, because it would pin every control-plane
# consumer of the contract to an AVX2+FMA build.
restore_contract_manifest() {
    cp "$root/crates/effect-contract/Cargo.toml" "$contract_manifest"
}

expect_contract_dependency_failure() {
    local mutation="$1"
    if bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null 2>&1; then
        printf 'effect contract dependency mutation escaped: %s\n' "$mutation" >&2
        exit 1
    fi
    restore_contract_manifest
}

sed -i '/^\[dependencies\]$/a lane.workspace = true' "$contract_manifest"
expect_contract_dependency_failure contract-gains-lane

sed -i '/^math[.]workspace = true$/d' "$contract_manifest"
expect_contract_dependency_failure contract-loses-math

# Issue #95 F6: the orphan root header must not come back, and the contract must stay non-`repr(C)`.
mkdir -p "$temp/include"
printf '#define MISO_ENGINE_EFFECT_CONTRACT_V1_H\n' >"$temp/include/miso_engine_effect_contract_v1.h"
if bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null 2>&1; then
    printf 'orphan contract header mutation escaped\n' >&2
    exit 1
fi
rm -rf "$temp/include"

printf '\n#[repr(C)]\npub struct LeakedAbiRecord {\n    pub a: u32,\n}\n' >>"$temp/crates/effect-contract/src/lib.rs"
if bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null 2>&1; then
    printf 'contract repr(C) mutation escaped\n' >&2
    exit 1
fi
cp "$root/crates/effect-contract/src/lib.rs" "$temp/crates/effect-contract/src/lib.rs"

# Issue #95 eval E4: the duplicated-helper manifest is a ratchet in both directions.
helper_mutation() {
    local name="$1"
    local file="$2"
    local body="$3"
    printf '\n%s\n' "$body" >>"$temp/$file"
    if bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null 2>&1; then
        printf 'duplicated-helper mutation escaped: %s\n' "$name" >&2
        exit 1
    fi
    cp "$root/$file" "$temp/$file"
}

helper_mutation normalize_zero-copy-in-an-effect \
    crates/delay/src/lib.rs \
    'fn normalize_zero(v: f32) -> f32 { v }'
helper_mutation sanitize-comes-back \
    crates/delay/src/lib.rs \
    'fn sanitize(v: f32, c: &mut u64) -> f32 { *c += 1; v }'
helper_mutation private-ramp-struct-comes-back \
    crates/compressor/src/lib.rs \
    'struct Ramp { current: f32, target: f32, remaining: u32 }'
helper_mutation second-linear-ramp \
    crates/effect-runtime/src/ramp.rs \
    'pub struct LinearRamp2 { pub current: f32 }
pub struct LinearRamp { pub current: f32 }'

# Down is a failure too: a row that reaches its target must be updated, not silently satisfied.
sed -i 's/^fn advance_ramps(\&mut self, sample_rate: u32) {/fn advance_ramps_renamed(\&mut self, sample_rate: u32) {/' \
    "$temp/crates/compressor/src/kernel.rs" 2>/dev/null || true
sed -i 's/fn advance_ramps(/fn advance_ramps_renamed(/' "$temp/crates/compressor/src/kernel.rs"
if bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null 2>&1; then
    printf 'duplicated-helper manifest accepted a stale row (count went down)\n' >&2
    exit 1
fi
cp "$root/crates/compressor/src/kernel.rs" "$temp/crates/compressor/src/kernel.rs"

printf '\npub struct EffectProgramSignature(pub [u8; 32]);\n' >>"$temp/crates/effect-contract/src/lib.rs"
if bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null 2>&1; then
    printf 'effect runtime identity mutation escaped\n' >&2
    exit 1
fi
cp "$root/crates/effect-contract/src/lib.rs" "$temp/crates/effect-contract/src/lib.rs"

real_rg="$(command -v rg)"
mkdir -p "$temp/rg-helper-filter-fail"
cat >"$temp/rg-helper-filter-fail/rg" <<EOF
#!/usr/bin/env bash
if [[ "\$1" == -v && "\$2" == '^crates/dsp-reference/' ]]; then cat >/dev/null; printf 'valid partial output\n' >&2; exit 9; fi
exec "$real_rg" "\$@"
EOF
chmod +x "$temp/rg-helper-filter-fail/rg"
helper_output="$(PATH="$temp/rg-helper-filter-fail:$PATH" bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" 2>&1)" && helper_rc=0 || helper_rc=$?
[[ "$helper_rc" -ne 0 && "$helper_output" == *'helper definition exemption filter errored (rg exit 9)'* && "$helper_output" == *'valid partial output'* ]] || {
    printf 'effect helper filter error escaped: %s\n' "$helper_output" >&2; exit 1;
}
mkdir -p "$temp/rg-helper-source-fail"
cat >"$temp/rg-helper-source-fail/rg" <<EOF
#!/usr/bin/env bash
if [[ "\$*" == *'fn\\s+sanitize\\('* ]]; then printf 'crates/delay/src/lib.rs:1:fn sanitize(\n'; exit 7; fi
exec "$real_rg" "\$@"
EOF
chmod +x "$temp/rg-helper-source-fail/rg"
helper_source_output="$(PATH="$temp/rg-helper-source-fail:$PATH" bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" 2>&1)" && helper_source_rc=0 || helper_source_rc=$?
[[ "$helper_source_rc" -ne 0 && "$helper_source_output" == *'helper definition scan scan errored (rg exit 7)'* && "$helper_source_output" == *'crates/delay/src/lib.rs'* ]] || {
    printf 'effect zero-pin helper source error escaped: %s\n' "$helper_source_output" >&2; exit 1;
}
mkdir -p "$temp/wc-fail"
cat >"$temp/wc-fail/wc" <<'EOF'
#!/usr/bin/env bash
cat >/dev/null
printf '0\n'
exit 6
EOF
chmod +x "$temp/wc-fail/wc"
helper_count_output="$(PATH="$temp/wc-fail:$PATH" bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" 2>&1)" && helper_count_rc=0 || helper_count_rc=$?
[[ "$helper_count_rc" -ne 0 && "$helper_count_output" == *'helper definition count errored (wc exit 6)'* && "$helper_count_output" == *$'0\n'* ]] || {
    printf 'effect helper count partial error escaped: %s\n' "$helper_count_output" >&2; exit 1;
}

printf 'effect runtime policy mutations: ok\n'
