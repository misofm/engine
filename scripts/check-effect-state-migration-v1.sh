#!/usr/bin/env bash
set -euo pipefail
cd "${1:-.}"
fail() { printf 'effect state migration policy failure: %s\n' "$1" >&2; exit 1; }

doc=docs/EFFECT_STATE_MIGRATION_V1.md
source=crates/effect-compiler/src/migration.rs
test -f "$doc" || fail 'missing exact migration documentation'
test -f "$source" || fail 'missing compiler migration module'
for required in '56-byte' 'zero-step' '0xa5' 'Arc' 'Buffer details' 'Overflow details' 'no benchmark'; do
    rg -q "$required" "$doc" || fail "documentation missing $required"
done
for api in inspect_effect_state_selector bind_effect_state_migration_edge resolve_effect_state_migration restore_scalar_effect_state_with_migration restore_unpublished_effect_bank_track_state_with_migration; do
    rg -q "$api" crates/effect-{package,compiler}/src || fail "missing API $api"
done
if rg -n 'effect_state_migration|EffectStateMigration' crates/{engine,session,graph,builtins-compiler,rack-compiler}/src 2>/dev/null; then
    fail 'migration reached a runtime/render-owned crate'
fi
if rg -n 'validate_descriptor|effect_descriptor_identity|bind_effect_descriptor_wire' "$source"; then
    fail 'migration repeats descriptor validation or identity binding'
fi
if rg -n 'serde|Serialize|Deserialize|migration_wire|encode_migration' "$source"; then
    fail 'migration serialization surface is forbidden'
fi
printf 'effect state migration policy: ok\n'
