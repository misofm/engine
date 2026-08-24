#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
policy="$root/scripts/check-scheduler-policy.sh"
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT

make_fixture() {
    local fixture=$1
    mkdir -p "$fixture/crates/miso-engine-native-scheduler/src/platform" \
        "$fixture/crates/miso-engine-graph/src" "$fixture/tools/miso-engine-audit" \
        "$fixture/hosts/miso-engine-host-native"
    printf '[package]\nname = "miso-engine-native-scheduler"\n[features]\nfault-injection = []\n[dependencies]\nmiso-engine-core.workspace = true\n' >"$fixture/crates/miso-engine-native-scheduler/Cargo.toml"
    printf '[package]\nname = "miso-engine-graph"\n[target.'"'"'cfg(not(target_arch = "wasm32"))'"'"'.dependencies]\nmiso-engine-native-scheduler.workspace = true\n[dev-dependencies]\nmiso-engine-native-scheduler = { workspace = true, features = ["fault-injection"] }\n' >"$fixture/crates/miso-engine-graph/Cargo.toml"
    printf '[package]\nname = "miso-engine-audit"\n' >"$fixture/tools/miso-engine-audit/Cargo.toml"
    printf '[package]\nname = "miso-engine-host-native"\n' >"$fixture/hosts/miso-engine-host-native/Cargo.toml"
    printf '// REALTIME_POLICY_BEGIN\nfn render() { core::hint::spin_loop(); }\n// REALTIME_POLICY_END\n' >"$fixture/crates/miso-engine-native-scheduler/src/lib.rs"
    printf 'fn wake_root() {\n// REALTIME_POLICY_BEGIN\n    root.thread.unpark();\n// REALTIME_POLICY_END\n}\nfn wake_children() {\n// REALTIME_POLICY_BEGIN\n    worker.thread.unpark();\n// REALTIME_POLICY_END\n}\nfn worker_loop() {\n    thread::park();\n}\nfn render_wave() {\n// REALTIME_POLICY_BEGIN\n    core::hint::spin_loop();\n// REALTIME_POLICY_END\n}\n' >"$fixture/crates/miso-engine-native-scheduler/src/platform/native.rs"
    printf '// REALTIME_POLICY_BEGIN\nfn render() {}\n// REALTIME_POLICY_END\n' >"$fixture/crates/miso-engine-native-scheduler/src/platform/browser.rs"
    printf '// REALTIME_POLICY_BEGIN\nfn render() {}\n// REALTIME_POLICY_END\n' >"$fixture/crates/miso-engine-graph/src/lib.rs"
    printf '// REALTIME_POLICY_BEGIN\nfn execute_op() {}\n// REALTIME_POLICY_END\n' >"$fixture/crates/miso-engine-graph/src/runtime.rs"
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
expect_failure second-coordinator-unpark 'sed -i "s|^    core::hint::spin_loop();$|    worker.thread.unpark();|" "$fixture/crates/miso-engine-native-scheduler/src/platform/native.rs"'
expect_failure park-outside-worker-loop 'printf "fn render_wave_two() {\n    thread::park();\n}\n" >>"$fixture/crates/miso-engine-native-scheduler/src/platform/native.rs"'
expect_failure fault-injection-in-dependencies 'printf "[dependencies]\nmiso-engine-native-scheduler = { workspace = true, features = [\"fault-injection\"] }\n" >>"$fixture/tools/miso-engine-audit/Cargo.toml"'
expect_failure fault-injection-in-a-host 'printf "[dependencies]\nmiso-engine-graph = { workspace = true, features = [\"fault-injection\"] }\n" >>"$fixture/hosts/miso-engine-host-native/Cargo.toml"'
printf 'scheduler policy mutations: PASS\n'
