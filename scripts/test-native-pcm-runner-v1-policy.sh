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
        "$case_root/hosts" "$case_root/sidecars"
    cp -R "$root/tools/native-pcm-runner" "$case_root/tools/"
    cp -R "$root/fixtures/native-pcm-runner/v1" "$case_root/fixtures/native-pcm-runner/"
    cp "$root/fixtures/session/v1/parametric-eq-nine-track.json" "$case_root/fixtures/session/v1/"
    cp "$root/scripts/check-native-pcm-runner.sh" "$case_root/scripts/"
}

copy_case baseline
"$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 >/dev/null

copy_case fixture-drift
printf mutation >>"$case_root/fixtures/native-pcm-runner/v1/riff-48000.wav"
if "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 >/dev/null 2>&1; then
    printf 'native PCM runner mutation escaped: fixture drift\n' >&2
    exit 1
fi

copy_case bypass
printf '\ngraph.workspace = true\n' >>"$case_root/tools/native-pcm-runner/Cargo.toml"
if "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 >/dev/null 2>&1; then
    printf 'native PCM runner mutation escaped: graph bypass\n' >&2
    exit 1
fi

copy_case reverse-dependency
cat >"$case_root/crates/fake/Cargo.toml" <<'EOF'
[package]
name = "miso-engine-fake"
version = "0.0.0"
[dependencies]
native-pcm-runner = { path = "../../tools/native-pcm-runner" }
EOF
if "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 >/dev/null 2>&1; then
    printf 'native PCM runner mutation escaped: reverse dependency\n' >&2
    exit 1
fi

# Required roots are a checked population: each may be empty, but a missing root is an error.
for required_root in crates hosts sidecars; do
    copy_case "missing-$required_root"
    rm -rf -- "$case_root/$required_root"
    output="$($case_root/scripts/check-native-pcm-runner.sh "$case_root" v1 2>&1)" && status=0 || status=$?
    [[ "$status" != 0 && "$output" == *"reachability root is missing: $required_root"* ]] || {
        printf 'missing required root was not rejected: %s\n%s\n' "$required_root" "$output" >&2; exit 1;
    }
done
copy_case missing-tools
rm -rf -- "$case_root/tools/native-pcm-runner"
output="$($case_root/scripts/check-native-pcm-runner.sh "$case_root" v1 2>&1)" && status=0 || status=$?
[[ "$status" != 0 && "$output" == *'tool surface is incomplete'* ]] || { printf 'missing tool surface was not rejected: %s\n' "$output" >&2; exit 1; }

# Required discovery must report execution errors even when a shim emits all four valid paths.
copy_case find-error
mkdir -p "$case_root/bin"
cat >"$case_root/bin/find" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' one two three four
exit 9
EOF
chmod +x "$case_root/bin/find"
output="$(PATH="$case_root/bin:$PATH" "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 2>&1)" && status=0 || status=$?
[[ "$status" != 0 && "$output" == *'RIFF corpus discovery failed (find status 9)'* ]] || { printf 'find error escaped: %s\n' "$output" >&2; exit 1; }

# Reachability status loss is injected after all valid metadata and payload are available. The
# unchanged filters therefore still produce a clean result in the disposable mutant, and the
# same assertion must identify that unexpected success with status 97.
copy_case reachability-error
mkdir -p "$case_root/bin"
cat >"$case_root/bin/rg" <<'EOF'
#!/usr/bin/env bash
if [[ "$*" == *'native-pcm-runner'* && "$*" == *'crates'* ]]; then
    printf 'tools/native-pcm-runner/src/lib.rs:1: native-pcm-runner\n'
    printf 'REACHABILITY_SENTINEL\n' >&2
    exit 7
fi
exec /usr/bin/rg "$@"
EOF
chmod +x "$case_root/bin/rg"
output="$(PATH="$case_root/bin:$PATH" "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 2>&1)" && status=0 || status=$?
[[ "$status" != 0 && "$output" == *'reachability scan failed (rg status 7)'* && "$output" == *'REACHABILITY_SENTINEL'* ]] || { printf 'reachability error was misclassified: %s\n' "$output" >&2; exit 1; }
mutant="$case_root/scripts/check-native-pcm-runner-mutant.sh"
cp "$case_root/scripts/check-native-pcm-runner.sh" "$mutant"
sed -i 's/\[\[ "$source_status" == 0 || "$source_status" == 1 \]\] || fail "native runner reachability scan failed (rg status $source_status): $(<"$scan_tmp\/reachable.err")"/true # MUTANT swallow reachability source status/' "$mutant"
assert_counter_mutant() {
    if output="$(PATH="$case_root/bin:$PATH" "$mutant" "$case_root" v1 2>&1)"; then
        printf 'counter-mutant unexpectedly passed: reachability status was swallowed\n%s\n' "$output" >&2
        return 97
    fi
    return 0
}
if assert_counter_mutant; then printf 'reachability counter-mutant did not reach unexpected-success assertion\n' >&2; exit 1; else status=$?; fi
[[ "$status" == 97 ]] || { printf 'reachability counter-mutant assertion status %s (expected 97)\n' "$status" >&2; exit 1; }

printf 'native PCM runner V1 policy mutations: ok\n'
