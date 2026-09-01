#!/usr/bin/env bash
# Static and fixture seal for the native-only Issue-073 C-ABI runner.
set -euo pipefail

root="${1:-.}"
cd "$root"

fail() {
    printf 'native PCM runner V1 check failure: %s\n' "$1" >&2
    exit 1
}

tool=tools/native-pcm-runner
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

for dependency in capi session source sha2; do
    rg -q "^$dependency\\.workspace = true$" "$tool/Cargo.toml" \
        || fail "missing exact direct dependency $dependency"
done

for forbidden in engine graph graph-compiler; do
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
#
# The inner `rg` is wrapped in its own `|| true`: without it, `rg` exiting 2 on a missing search
# root (e.g. a hermetic test fixture with no sidecars/) would make the whole `if rg | rg; then`
# pipeline read as "no violation" under `pipefail`, even when a real match was printed to stdout
# by the roots that do exist.
# A doc-comment mention of the tool by name (e.g. a fixture crate citing which tool configures
# its fixtures) is documentation, not reachability -- the same distinction
# check-conformance-boundaries.sh draws for the f64 oracle. This only started mattering once the
# rename made the tool's real name (`native-pcm-runner`) the same bare spelling doc prose already
# used informally.
reachable="$({
    rg -n 'native-pcm-runner|native_pcm_runner' crates hosts tools sidecars \
        --glob Cargo.toml --glob '*.rs' || true
} | rg -v '^tools/native-pcm-runner/' | rg -v ':[0-9]+:[[:space:]]*///?[[:space:]]' || true)"
[[ -z "$reachable" ]] || {
    printf '%s\n' "$reachable" >&2
    fail 'native-only runner is reachable from another package or Wasm surface'
}

printf 'native PCM runner V1 check: ok\n'
