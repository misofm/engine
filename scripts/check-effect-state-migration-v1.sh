#!/usr/bin/env bash
set -euo pipefail
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "${1:-.}" && pwd)"
cd "$root"
source "$script_dir/lib/gate.sh"
GATE_FAILURE_PREFIX='effect state migration policy failure'
fail() { printf 'effect state migration policy failure: %s\n' "$1" >&2; exit 1; }

doc=docs/EFFECT_STATE_MIGRATION_V1.md
source=crates/effect-compiler/src/migration.rs
test -f "$doc" || fail 'missing exact migration documentation'
test -f "$source" || fail 'missing compiler migration module'
for required in '56-byte' 'zero-step' '0xa5' 'Arc' 'Buffer details' 'Overflow details' 'no benchmark'; do
    gate_scan_required "documentation $required" "$required" '' "$doc" >/dev/null || exit $?
done
for api in inspect_effect_state_selector bind_effect_state_migration_edge resolve_effect_state_migration restore_scalar_effect_state_with_migration restore_unpublished_effect_bank_track_state_with_migration; do
    gate_scan_required "API $api" "$api" '' crates/effect-package/src crates/effect-compiler/src >/dev/null || exit $?
done
gate_scan_forbidden 'runtime-owned migration scan' 'effect_state_migration|EffectStateMigration' '' crates/engine/src crates/session/src crates/graph/src crates/builtins-compiler/src crates/rack-compiler/src >/dev/null || exit $?
gate_scan_forbidden 'migration descriptor validation scan' 'validate_descriptor|effect_descriptor_identity|bind_effect_descriptor_wire' '' "$source" >/dev/null || exit $?
gate_scan_forbidden 'migration serialization' 'serde|Serialize|Deserialize|migration_wire|encode_migration' '' "$source" >/dev/null || exit $?
printf 'effect state migration policy: ok\n'
