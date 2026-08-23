#!/usr/bin/env bash
# Static and fixture seal for the native-only Issue-073 C-ABI runner.
set -euo pipefail

root="${1:-.}"
cd "$root"

fail() {
    printf 'native PCM runner V1 check failure: %s\n' "$1" >&2
    exit 1
}

tool=tools/miso-engine-native-pcm-runner
fixture=fixtures/native-pcm-runner/v1
[[ -f "$tool/Cargo.toml" && -f "$tool/src/lib.rs" && -f "$tool/src/main.rs" ]] \
    || fail 'tool surface is incomplete'
[[ -f "$fixture/MANIFEST.tsv" && -f "$fixture/generate.py" ]] \
    || fail 'fixture surface is incomplete'

python3 -I -B "$fixture/generate.py" --check \
    || fail 'independent fixture corpus or manifest drifted'

[[ $(find "$fixture" -maxdepth 1 -type f -name 'riff-*.wav' | wc -l) -eq 4 ]] \
    || fail 'exact four-rate RIFF corpus missing'
[[ -f "$fixture/rf64-48000.wav" ]] || fail 'RF64 corpus row missing'

for dependency in miso-engine-capi miso-engine-session miso-engine-source sha2; do
    rg -q "^$dependency\\.workspace = true$" "$tool/Cargo.toml" \
        || fail "missing exact direct dependency $dependency"
done

for forbidden in miso-engine-core miso-engine-graph miso-engine-graph-compiler; do
    ! rg -q "^$forbidden" "$tool/Cargo.toml" \
        || fail "forbidden product bypass dependency $forbidden"
done

for operation in \
    miso_engine_v2_engine_create miso_engine_v2_compile_session \
    miso_engine_v2_source_submit_planar_f32 miso_engine_v2_render_f32_planar \
    miso_engine_v2_plan_resources miso_engine_v2_plan_destroy \
    miso_engine_v2_session_destroy miso_engine_v2_engine_destroy; do
    rg -q "$operation" "$tool/src/lib.rs" || fail "missing frozen ABI operation $operation"
done

! rg -n 'compile_session\(|GraphCompiler|PcmSourceRing|PreparedRenderPlan' "$tool/src" \
    | rg -v 'miso_engine_v2_compile_session' \
    || fail 'Rust product bypass is reachable from the tool'

# The native decoder is cfg-excluded on Wasm, so no crate may make this tool a dependency. The
# workspace membership row is intentionally the sole reference outside the package and lockfile.
if rg -n 'miso-engine-native-pcm-runner|miso_engine_native_pcm_runner' crates hosts tools \
    --glob Cargo.toml --glob '*.rs' | rg -v '^tools/miso-engine-native-pcm-runner/'; then
    fail 'native-only runner is reachable from another package or Wasm surface'
fi

printf 'native PCM runner V1 check: ok\n'
