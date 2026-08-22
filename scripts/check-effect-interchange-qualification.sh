#!/usr/bin/env bash
# Static Issue 081 qualification boundary and immutable-baseline checker.
set -euo pipefail
root="$(cd "${1:-.}" && pwd)"
cd "$root"
fail() { printf 'effect interchange qualification policy failure: %s\n' "$1" >&2; exit 1; }

manifest=fixtures/effect-interchange/v1/ACCEPTED.sha256
accepted_manifest_sha256=6403ae6205dbc86a57483f44723cfc107f7f49654532fc648516b7cfed7ae3a5
[[ -f "$manifest" && ! -L "$manifest" ]] || fail 'missing immutable baseline manifest'
[[ "$(sha256sum "$manifest" | awk '{print $1}')" == "$accepted_manifest_sha256" ]] ||
    fail 'immutable baseline manifest changed or was refreshed'
LC_ALL=C sort -c -k2,2 "$manifest" || fail 'baseline manifest is not path-sorted'
sha256sum --check --strict "$manifest" >/dev/null || fail 'accepted baseline changed'
[[ $(wc -l <"$manifest" | tr -d ' ') -eq 36 ]] || fail 'baseline membership changed'

for path in \
    scripts/effect-interchange-v1-reference.py \
    scripts/run-effect-interchange-reference-processes.sh \
    scripts/test-effect-interchange-reference-runner.sh \
    scripts/check-effect-interchange-targets.sh \
    docs/EFFECT_INTERCHANGE_QUALIFICATION_V1.md; do
    [[ -f "$path" ]] || fail "missing qualification path $path"
done

for reference in scripts/effect-{descriptor,package,state}-v1-reference.py; do
    if rg -n 'subprocess|os[.]system|Popen|spawn' "$reference"; then
        fail "accepted reference launches a child: $reference"
    fi
done
rg -q '^if __name__ == "__main__":' scripts/effect-interchange-v1-reference.py ||
    fail 'aggregator is not import-safe'
rg -q 'range\(0, ?100\)|seq 0 99' scripts/run-effect-interchange-reference-processes.sh ||
    fail 'runner does not freeze indexes 0..99'
rg -q 'TRIALS: usize = 10_000' \
    crates/miso-engine-effect-package/tests/effect_interchange_mutation.rs ||
    fail 'mutation trial count changed'
for seed in 0001 0002 0003; do
    rg -q "0x081d_e5c0_0000_$seed" \
        crates/miso-engine-effect-package/tests/effect_interchange_mutation.rs ||
        fail "mutation seed $seed changed"
done
rg -q 'exact_portable_migration_qualification_matrix' \
    crates/miso-engine-effect-compiler/tests/migration_terminal.rs ||
    fail 'missing exact migration matrix'

for target in x86_64-unknown-linux-gnu aarch64-linux-android aarch64-apple-ios wasm32-unknown-unknown; do
    rg -q "$target" scripts/check-effect-interchange-targets.sh ||
        fail "target row missing: $target"
done
for feature in 'feature=-simd128' 'feature=+simd128'; do
    rg -Fq -- "$feature" scripts/check-effect-interchange-targets.sh ||
        fail "Wasm target feature row missing: $feature"
done

if rg -n 'miso-engine-effect-interchange|effect_interchange_qualification' \
    crates/*/Cargo.toml hosts/*/Cargo.toml 2>/dev/null; then
    fail 'qualification dependency reached a production package'
fi
if rg -n 'effect_interchange|EffectInterchange|effect_state_migration|EffectStateMigration' \
    crates/miso-engine-{core,session,graph,rack-compiler,builtins-compiler}/src 2>/dev/null; then
    fail 'interchange qualification or migration reached render-owned source'
fi
if rg -n 'Serialize|Deserialize|serde|migration_wire|encode_migration' \
    crates/miso-engine-effect-compiler/src/migration.rs; then
    fail 'migration registry serialization appeared'
fi
exports="$(rg -n '#\[unsafe\(no_mangle\)\]|#\[no_mangle\]' \
    crates/miso-engine-effect-package/src | wc -l | tr -d ' ')"
[[ "$exports" -eq 1 ]] || fail 'descriptor package gained a C export'
rg -q 'fn miso_engine_effect_descriptor_v1_inspect' \
    crates/miso-engine-effect-package/src/ffi.rs || fail 'sole descriptor export missing'
if find fixtures/effect-interchange/v1 -mindepth 1 -maxdepth 1 -type f \
    ! -name ACCEPTED.sha256 -print -quit | grep -q .; then
    fail 'untracked/generated corpus appeared in interchange fixture directory'
fi
if find . -path './target' -prune -o -type f \
    \( -name '*.o' -o -name '*.a' -o -name '*.so' -o -name '*.dylib' -o -name '*.wasm' \
       -o -name '*.profraw' -o -name '*.jsonl.raw' \) -print | grep -q .; then
    fail 'generated artifact exists under a source path'
fi
for api in verify_effect_descriptor_wire_v1 verify_effect_package_v1 inspect_effect_state_selector_v1 resolve_effect_state_migration_v1 restore_scalar_effect_state_with_migration_v1 restore_unpublished_effect_bank_track_state_with_migration_v1; do
    rg -q "$api" crates/miso-engine-effect-{package,compiler}/src || fail "stale API $api"
done
printf 'effect interchange qualification policy: ok\n'
