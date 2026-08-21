#!/usr/bin/env bash
set -euo pipefail
cd "${1:-.}"
fail() { printf 'effect runtime policy failure: %s\n' "$1" >&2; exit 1; }
dependencies() {
    awk '/^\[dependencies\]$/ { in_deps=1; next } /^\[/ { in_deps=0 } in_deps && /^[A-Za-z0-9_-]+(\.workspace)?[[:space:]]*=/ { line=$0; sub(/[[:space:]]*=.*/, "", line); sub(/\.workspace$/, "", line); print line }' "$1" | sort
}
[[ "$(dependencies crates/miso-engine-effect-contract/Cargo.toml)" == 'miso-engine-core' ]] || fail 'effect-contract dependency boundary changed'
expected_compiler=$'miso-engine-compressor\nmiso-engine-core\nmiso-engine-effect-contract\nmiso-engine-gate-expander\nmiso-engine-parametric-eq\nmiso-engine-session\nmiso-engine-true-peak-limiter'
[[ "$(dependencies crates/miso-engine-effect-compiler/Cargo.toml)" == "$expected_compiler" ]] || fail 'effect-compiler dependency boundary changed'
if rg -n 'miso-engine-effect-(contract|compiler)' crates/miso-engine-{core,session}/Cargo.toml; then fail 'core/session reverse dependency'; fi
if rg -n 'miso_engine_effect_package|miso-engine-effect-package|sha2' crates/miso-engine-{effect-contract,effect-compiler,conformance} tools/miso-engine-effect-contract-bench; then fail 'issue-029 package/hash leaked into issue-011 path'; fi
if rg -n 'descriptor_schema_hash|EffectProgramSignature|canonical_effect_descriptor|encode_lane_payload' crates/miso-engine-effect-contract/src crates/miso-engine-effect-compiler/src; then fail 'wire/hash/persistence identity leaked into runtime API'; fi
for required in effect.native.unavailable effect.descriptor.invalid effect.quality.unsupported effect.link_mode.unsupported effect.parameter.unknown effect.parameter.unit_mismatch effect.parameter.domain effect.parameter.channel effect.parameter.duplicate_channel effect.sidechain.missing effect.sidechain.unknown_port effect.sidechain.unexpected effect.resource.limit effect.prepare.failed effect.metadata.mismatch effect.state.invalid effect.third_party.unavailable_at_launch; do
    rg -q "$required" crates/miso-engine-effect-{contract,compiler} docs/EFFECT_CONTRACT_V1.md || fail "missing diagnostic $required"
done
printf 'effect runtime policy: ok\n'
