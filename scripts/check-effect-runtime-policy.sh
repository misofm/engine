#!/usr/bin/env bash
set -euo pipefail
cd "${1:-.}"
fail() { printf 'effect runtime policy failure: %s\n' "$1" >&2; exit 1; }
dependencies() {
    awk '/^\[dependencies\]$/ { in_deps=1; next } /^\[/ { in_deps=0 } in_deps && /^[A-Za-z0-9_-]+(\.workspace)?[[:space:]]*=/ { line=$0; sub(/[[:space:]]*=.*/, "", line); sub(/\.workspace$/, "", line); print line }' "$1" | sort
}
# #84 phase A: PrepareEffectBankRequest.backend is miso_engine_lane::Backend now.
expected_contract=$'miso-engine-core\nmiso-engine-lane\nmiso-engine-math'
[[ "$(dependencies crates/miso-engine-effect-contract/Cargo.toml)" == "$expected_contract" ]] || fail 'effect-contract dependency boundary changed'
expected_compiler=$'miso-engine-compressor\nmiso-engine-core\nmiso-engine-delay\nmiso-engine-effect-contract\nmiso-engine-effect-package\nmiso-engine-gate-expander\nmiso-engine-lane\nmiso-engine-multiband-compressor\nmiso-engine-parametric-eq\nmiso-engine-session\nmiso-engine-soft-clip\nmiso-engine-transient-shaper\nmiso-engine-true-peak-limiter'
[[ "$(dependencies crates/miso-engine-effect-compiler/Cargo.toml)" == "$expected_compiler" ]] || fail 'effect-compiler dependency boundary changed'
if rg -n 'miso-engine-effect-(contract|compiler)' crates/miso-engine-{core,session}/Cargo.toml; then fail 'core/session reverse dependency'; fi
package_references="$(
    rg -n 'miso_engine_effect_package|miso-engine-effect-package' crates hosts tools fuzz 2>/dev/null |
        rg -v '^crates/miso-engine-effect-package/' |
        rg -v '^crates/miso-engine-effect-compiler/(Cargo.toml|src/(prepare|migration)[.]rs|tests/(scalar_state|bank_state|migration|migration_terminal)[.]rs):' || true
)"
package_references="$(printf '%s\n' "$package_references" |
    rg -v '^fuzz/(Cargo.toml|Cargo.lock|fuzz_targets/effect_(package|state)[.]rs):' || true)"
package_references="$(printf '%s\n' "$package_references" |
    rg -v '^tools/miso-engine-effect-interchange-bench/(Cargo.toml|src/main[.]rs):' || true)"
if [[ -n "$package_references" ]]; then
    printf '%s\n' "$package_references" >&2
    fail 'effect-package reference escaped the issue-079/080 compiler state boundary'
fi
if rg -n 'descriptor_schema_hash|EffectProgramSignature|canonical_effect_descriptor|encode_lane_payload' crates/miso-engine-effect-contract/src crates/miso-engine-effect-compiler/src; then fail 'wire/hash/persistence identity leaked into runtime API'; fi
# Issue #95 finding F6: the root `include/miso_engine_effect_contract_v1.h` had no Rust mirror and
# disagreed with the real wire ABI (it claimed 32-byte ports and 48-byte quality rows against the
# implemented 24 and 64). The contract crate's Rust types are deliberately not `repr(C)`; the only
# C ABI for descriptors is the effect-package header, which `check-effect-descriptor-v1.sh` gates.
[[ ! -e include/miso_engine_effect_contract_v1.h ]] || fail 'orphan contract header is back (issue #95 F6)'
[[ ! -d include ]] || fail 'root include/ is the deleted orphan header directory (issue #95 F6)'
if rg -n 'repr\(C\)' crates/miso-engine-effect-contract/src; then fail 'the contract crate has no C ABI; descriptors are effect-package'"'"'s'; fi
if rg -n 'effect_state_migration|EffectStateMigration' crates/miso-engine-{core,session,graph,builtins-compiler,rack-compiler}/src 2>/dev/null; then fail 'effect-state migration reached a runtime/render-owned crate'; fi
for required in effect.native.unavailable effect.descriptor.invalid effect.quality.unsupported effect.link_mode.unsupported effect.parameter.unknown effect.parameter.unit_mismatch effect.parameter.domain effect.parameter.channel effect.parameter.duplicate_channel effect.sidechain.missing effect.sidechain.unknown_port effect.sidechain.unexpected effect.resource.limit effect.prepare.failed effect.metadata.mismatch effect.state.invalid effect.third_party.unavailable_at_launch; do
    rg -q "$required" crates/miso-engine-effect-{contract,compiler} docs/EFFECT_CONTRACT_V1.md || fail "missing diagnostic $required"
done
# ---------------------------------------------------------------------------------------------
# Issue #95 eval E4: the audit's duplicated-helper list has one home each.
#
# The #83 audit found the same handful of helpers copied into seven and eight effect crates:
# `sanitize`, `normalize_zero`, `parameter_value_valid`, `validate_state_lengths`, `state_error`,
# `Ramp`, `apply_automation`, `discontinuity_reset`. Wave 2 moved most of them into
# `miso-engine-effect-runtime` and wave 3 deleted the contract's dead ones. This manifest is what
# keeps them collapsed: it is a **ratchet**, exactly like `check-math-policy.sh`'s allowlist. A
# count that goes up fails, and a count that reaches its target must have its row updated in the
# same change. A name with no home left is pinned at 0, so a copy cannot come back quietly.
#
# Scope is `crates/*/src` minus two structural exemptions: `miso-engine-effect-package` is the
# wire/persistence domain (its `parameter_value_valid` validates a borrowed wire view, issue #97)
# and `miso-engine-dsp-reference` is the independent f64 oracle.
#
# The rows still above 1 are not laziness, they are one blocked change. `miso-engine-effect-
# contract` is `std` and control-plane; `miso-engine-effect-runtime` is `no_std` and lane-generic;
# neither may depend on the other, so each defines its own `normalize_zero`, `is_negative_zero`,
# parameter-domain predicate, mapping pair and `StatePayloadError`, and every effect crate carries
# a small `state_error` bridge between the two error types. Collapsing them needs one edge -
# `effect-runtime` depending on `effect-contract`, which requires making the contract `no_std`
# (its only `std` uses are `BTreeMap`, `BTreeSet` and `Arc`, all of them `alloc`) - and then a
# mechanical sweep of all nine effect crates. That is a coordinated cross-crate change, not a
# contract cleanup, and #95 hands it over with the counts pinned here so it cannot drift first.
#
#   pattern                          count  owner
duplicated_helper_manifest() {
    cat <<'MANIFEST'
fn\s+sanitize\(                          0  95
fn\s+sanitize_sample\(                   0  95
fn\s+parameter_value_is_valid\(          0  95
fn\s+flushed\(                           0  95
fn\s+recover\(                           0  95
struct\s+Ramp\b                          0  95
fn\s+valid_runtime_span\(                1  95
fn\s+automation_segment_value\(          1  95
fn\s+canonical_bits\(                    1  95
struct\s+ParameterSmoother\b             1  95
struct\s+LinearRamp\b                    1  95
fn\s+clamp_to_domain\(                   1  95
fn\s+advance_ramps\(                     1  95
fn\s+check_block[(<]                     1  95
fn\s+normalize_zero\(                    2  95
fn\s+is_negative_zero\(                  2  95
fn\s+map_normalized\(                    2  95
fn\s+inverse_map_normalized\(            2  95
fn\s+parameter_value_valid\(             3  95
fn\s+validate_state_lengths\(            2  95
fn\s+state_error\(                       6  95
fn\s+apply_automation\(                  4  95
fn\s+discontinuity_reset\(               4  95
MANIFEST
}

if [[ "${MISO_PRINT_HELPER_MANIFEST:-}" == "1" ]]; then
    duplicated_helper_manifest
    exit 0
fi

helper_exempt='^crates/miso-engine-(effect-package|dsp-reference)/'
helper_definitions() {
    { rg -n --glob '*.rs' "$1" crates/*/src 2>/dev/null || true; } | { rg -v "$helper_exempt" || true; }
}
while read -r pattern expected owner; do
    [[ -n "$pattern" ]] || continue
    found="$(helper_definitions "$pattern" | wc -l | tr -d ' ')"
    if [[ "$found" -gt "$expected" ]]; then
        helper_definitions "$pattern" >&2
        fail "$pattern has $found definitions, the manifest pins at most $expected (issue #$owner)"
    fi
    if [[ "$found" -lt "$expected" ]]; then
        fail "$pattern is down to $found definitions from $expected; update its manifest row in the same change (issue #$owner)"
    fi
done < <(duplicated_helper_manifest)

printf 'effect runtime policy: ok\n'
