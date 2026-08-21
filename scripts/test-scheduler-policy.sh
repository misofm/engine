#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
policy="$root/scripts/check-scheduler-policy.sh"
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT

make_fixture() {
    local fixture=$1
    mkdir -p "$fixture/crates/miso-engine-native-scheduler/src" "$fixture/crates/miso-engine-graph/src"
    printf '[package]\nname = "miso-engine-native-scheduler"\n[dependencies]\nmiso-engine-core.workspace = true\n' >"$fixture/crates/miso-engine-native-scheduler/Cargo.toml"
    printf '[package]\nname = "miso-engine-graph"\n[target.'"'"'cfg(not(target_arch = "wasm32"))'"'"'.dependencies]\nmiso-engine-native-scheduler.workspace = true\n' >"$fixture/crates/miso-engine-graph/Cargo.toml"
    printf '// REALTIME_POLICY_BEGIN\nfn render() { core::hint::spin_loop(); }\n// REALTIME_POLICY_END\n' >"$fixture/crates/miso-engine-native-scheduler/src/lib.rs"
    printf '// REALTIME_POLICY_BEGIN\nfn render() {}\n// REALTIME_POLICY_END\n' >"$fixture/crates/miso-engine-graph/src/lib.rs"
}

expect_failure() {
    local name=$1
    local mutation=$2
    local fixture="$scratch/$name"
    make_fixture "$fixture"
    eval "$mutation"
    if bash "$policy" "$fixture" >/dev/null 2>&1; then
        printf 'scheduler policy mutation unexpectedly passed: %s\n' "$name" >&2
        exit 1
    fi
}

valid="$scratch/valid"
make_fixture "$valid"
bash "$policy" "$valid" >/dev/null
expect_failure unsafe 'printf "unsafe fn bad() {}\n" >>"$fixture/crates/miso-engine-native-scheduler/src/lib.rs"'
expect_failure mutex 'sed -i "s/core::hint::spin_loop()/std::sync::Mutex::new(())/" "$fixture/crates/miso-engine-native-scheduler/src/lib.rs"'
expect_failure allocation 'sed -i "s/fn render() {}/fn render() { let _ = Vec::new(); }/" "$fixture/crates/miso-engine-graph/src/lib.rs"'
expect_failure reverse-dependency 'printf "miso-engine-native-scheduler.workspace = true\n" >>"$fixture/crates/miso-engine-native-scheduler/Cargo.toml"'
expect_failure wasm-reachability 'printf "[package]\nname = \\"miso-engine-graph\\"\n[target.\"cfg(target_arch = \\\"wasm32\\\")\".dependencies]\nmiso-engine-native-scheduler.workspace = true\n" >"$fixture/crates/miso-engine-graph/Cargo.toml"'
expect_failure track-limit 'printf "const MAX_TRACKS: usize = 8;\n" >>"$fixture/crates/miso-engine-graph/src/lib.rs"'
printf 'scheduler policy mutations: PASS\n'
