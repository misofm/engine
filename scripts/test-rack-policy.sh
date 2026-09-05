#!/usr/bin/env bash
# Mutation probes for the narrow Issue-008 rack policy.
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
policy="$root/scripts/check-rack-policy.sh"
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT

make_fixture() {
    local fixture=$1
    mkdir -p "$fixture/crates/rack/src" "$fixture/crates/rack-compiler/src"
    printf '[workspace]\nmembers = []\n' >"$fixture/Cargo.toml"
    printf '[package]\nname = "rack"\n[dependencies]\nengine.workspace = true\neffect-contract.workspace = true\n' >"$fixture/crates/rack/Cargo.toml"
    printf '[package]\nname = "rack-compiler"\n[dependencies]\nengine.workspace = true\neffect-contract.workspace = true\nrack.workspace = true\n' >"$fixture/crates/rack-compiler/Cargo.toml"
    printf '//! fixture\n' >"$fixture/crates/rack/src/lib.rs"
    printf '//! fixture\n' >"$fixture/crates/rack-compiler/src/lib.rs"
}

expect_failure() {
    local name=$1
    local mutation=$2
    local expected=${3:-'rack policy failure:'}
    local fixture="$scratch/$name"
    local test_path=$PATH
    make_fixture "$fixture"
    eval "$mutation"
    local output
    if output="$(PATH="$test_path" bash "$policy" "$fixture" 2>&1)"; then
        printf 'rack policy mutation unexpectedly passed: %s\n' "$name" >&2
        exit 1
    fi
    [[ "$output" == *"$expected"* ]] || {
        printf 'rack policy mutation failed in wrong class (%s): %s\n' "$name" "$output" >&2
        exit 1
    }
}

valid="$scratch/valid"
make_fixture "$valid"
bash "$policy" "$valid" >/dev/null
expect_failure unsafe 'printf "unsafe fn bad() {}\n" >>"$fixture/crates/rack/src/lib.rs"'
# The MAX_TRACKS ban itself moved to scripts/check-workspace-policy.sh (P12, one copy instead of
# five); scripts/test-workspace-policy.sh covers its mutation. This script's job is now only the
# unsafe ban and the dependency boundary.
expect_failure dependency 'printf "session.workspace = true\n" >>"$fixture/crates/rack/Cargo.toml"'
expect_failure control-io 'printf "fn bad() { std::fs::read(\"x\"); }\n" >>"$fixture/crates/rack/src/lib.rs"' \
    'control-plane, I/O, threading, synchronization, or logging leaked into rack render code'
expect_failure feature-detection 'printf "is_x86_feature_detected!(\"avx2\");\n" >>"$fixture/crates/rack-compiler/src/lib.rs"' \
    'feature detection or target-feature specialization leaked out of core dispatch'
expect_failure missing-rack-src 'rm -rf -- "$fixture/crates/rack/src"' \
    'missing search path(s): crates/rack/src'
expect_failure missing-manifest 'rm -f -- "$fixture/crates/rack/Cargo.toml"' \
    'missing rack manifests'
expect_failure extractor-error '
    mkdir -p "$fixture/fake-bin"
    printf "#!/usr/bin/env bash\nprintf partial\\n\nexit 7\\n" >"$fixture/fake-bin/awk"
    chmod +x "$fixture/fake-bin/awk"
    test_path="$fixture/fake-bin:$PATH"
' 'dependency extraction failed for crates/rack/Cargo.toml (awk status 7)'

foreign="$scratch/foreign-fixture"
make_fixture "$foreign"
mkdir -p "$foreign/scripts/lib" "$scratch/foreign-cwd"
printf 'return 99\n' >"$foreign/scripts/lib/gate.sh"
(cd "$scratch/foreign-cwd" && bash "$policy" "$foreign" >/dev/null)
printf 'rack policy mutations: ok\n'
