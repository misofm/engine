#!/usr/bin/env bash
set -euo pipefail
cd "${1:-.}"
fail() { printf 'effect state migration policy failure: %s\n' "$1" >&2; exit 1; }

doc=docs/EFFECT_STATE_MIGRATION_V1.md
source=crates/miso-engine-effect-compiler/src/migration.rs
test -f "$doc" || fail 'missing exact migration documentation'
test -f "$source" || fail 'missing compiler migration module'
for required in '56-byte' 'zero-step' '0xa5' 'Arc' 'Buffer details' 'Overflow details' 'no benchmark'; do
    rg -q "$required" "$doc" || fail "documentation missing $required"
done
for api in inspect_effect_state_selector_v1 bind_effect_state_migration_edge_v1 resolve_effect_state_migration_v1 restore_scalar_effect_state_with_migration_v1 restore_unpublished_effect_bank_track_state_with_migration_v1; do
    rg -q "$api" crates/miso-engine-effect-{package,compiler}/src || fail "missing API $api"
done
if rg -n 'effect_state_migration|EffectStateMigration' crates/miso-engine-{core,session,graph,builtins-compiler,rack-compiler}/src 2>/dev/null; then
    fail 'migration reached a runtime/render-owned crate'
fi
if rg -n 'validate_descriptor_v1|effect_descriptor_identity_v1|bind_effect_descriptor_wire_v1' "$source"; then
    fail 'migration repeats descriptor validation or identity binding'
fi
if rg -n 'serde|Serialize|Deserialize|migration_wire|encode_migration' "$source"; then
    fail 'migration serialization surface is forbidden'
fi
printf 'effect state migration policy: ok\n'
