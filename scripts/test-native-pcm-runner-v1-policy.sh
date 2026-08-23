#!/usr/bin/env bash
# Hermetic mutations for the Issue-073 dependency and fixture checker.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
temp="$(mktemp -d)"
trap 'rm -rf -- "$temp"' EXIT

copy_case() {
    case_root="$temp/$1"
    mkdir -p "$case_root/tools" "$case_root/fixtures/native-pcm-runner" \
        "$case_root/fixtures/session/v1" "$case_root/scripts" "$case_root/crates/fake/src" \
        "$case_root/hosts"
    cp -R "$root/tools/miso-engine-native-pcm-runner" "$case_root/tools/"
    cp -R "$root/fixtures/native-pcm-runner/v1" "$case_root/fixtures/native-pcm-runner/"
    cp "$root/fixtures/session/v1/parametric-eq-nine-track.toml" "$case_root/fixtures/session/v1/"
    cp "$root/scripts/check-native-pcm-runner-v1.sh" "$case_root/scripts/"
}

copy_case baseline
"$case_root/scripts/check-native-pcm-runner-v1.sh" "$case_root" >/dev/null

copy_case fixture-drift
printf mutation >>"$case_root/fixtures/native-pcm-runner/v1/riff-48000.wav"
if "$case_root/scripts/check-native-pcm-runner-v1.sh" "$case_root" >/dev/null 2>&1; then
    printf 'native PCM runner mutation escaped: fixture drift\n' >&2
    exit 1
fi

copy_case bypass
printf '\nmiso-engine-graph.workspace = true\n' >>"$case_root/tools/miso-engine-native-pcm-runner/Cargo.toml"
if "$case_root/scripts/check-native-pcm-runner-v1.sh" "$case_root" >/dev/null 2>&1; then
    printf 'native PCM runner mutation escaped: graph bypass\n' >&2
    exit 1
fi

copy_case reverse-dependency
cat >"$case_root/crates/fake/Cargo.toml" <<'EOF'
[package]
name = "miso-engine-fake"
version = "0.0.0"
[dependencies]
miso-engine-native-pcm-runner = { path = "../../tools/miso-engine-native-pcm-runner" }
EOF
if "$case_root/scripts/check-native-pcm-runner-v1.sh" "$case_root" >/dev/null 2>&1; then
    printf 'native PCM runner mutation escaped: reverse dependency\n' >&2
    exit 1
fi

printf 'native PCM runner V1 policy mutations: ok\n'
