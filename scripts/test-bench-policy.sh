#!/usr/bin/env bash
# Mutations for the one-harness gate (#104 F4, phase B). Each one reintroduces a copy the audit
# counted, in a scratch tree.
set -euo pipefail

root="$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf -- "$scratch"' EXIT

new_case() {
    case_root="$scratch/$1"
    mkdir -p "$case_root/scripts"
    cp -R "$root/tools" "$case_root/"
    mkdir -p "$case_root/crates" "$case_root/hosts"
    cp "$root/scripts/check-bench-policy.sh" "$case_root/scripts/"
}

check() { bash "$case_root/scripts/check-bench-policy.sh" "$case_root"; }

expect_failure() {
    if check >/dev/null 2>&1; then
        printf 'bench policy mutation escaped: %s\n' "$1" >&2
        exit 1
    fi
}

new_case baseline
check >/dev/null

new_case second-allocator
printf '\nunsafe impl GlobalAlloc for Second {}\n' \
    >>"$case_root/tools/miso-engine-rack-bench/src/main.rs"
expect_failure second-allocator

new_case second-global-allocator-attribute
printf '\n#[global_allocator]\nstatic A: X = X;\n' \
    >>"$case_root/tools/miso-engine-scheduler-bench/src/main.rs"
expect_failure second-global-allocator-attribute

new_case second-escaper
printf '\nfn json_escape(value: &str) -> String {\n    value.to_owned()\n}\n' \
    >>"$case_root/tools/miso-engine-conformance-bench/src/main.rs"
expect_failure second-escaper

new_case second-percentile
printf '\nfn percentile(sorted: &[u64], p: usize) -> u64 {\n    sorted[p]\n}\n' \
    >>"$case_root/tools/miso-engine-graph-bench/src/main.rs"
expect_failure second-percentile

new_case second-digest-sink
printf '\nstruct Sha256Sink;\n' \
    >>"$case_root/tools/miso-engine-builtins-bench/src/main.rs"
expect_failure second-digest-sink

new_case removed-escaper
rm "$case_root/tools/miso-engine-bench-support/src/json.rs"
expect_failure removed-escaper

new_case new-unsafe-owner
printf '#![allow(unsafe_code)]\n' \
    >>"$case_root/tools/miso-engine-source-audit/src/main.rs"
expect_failure new-unsafe-owner

new_case retired-unsafe-owner
sed -i '/^#!\[allow(unsafe_code)\]$/d' "$case_root/tools/miso-engine-capi-audit/src/main.rs"
expect_failure retired-unsafe-owner

new_case converted-subject-loses-the-shared-timer
sed -i 's/timing::timed/inline_timed/' "$case_root/tools/miso-engine-rack-bench/src/main.rs"
expect_failure converted-subject-loses-the-shared-timer

new_case converted-subject-regrows-a-clock
printf '\nfn t() { let _ = Instant::now(); }\n' \
    >>"$case_root/tools/miso-engine-rack-bench/src/main.rs"
expect_failure converted-subject-regrows-a-clock

new_case converted-subject-regrows-a-digest
printf '\nfn h() { let _ = Sha256::new(); }\n' \
    >>"$case_root/tools/miso-engine-rack-bench/src/main.rs"
expect_failure converted-subject-regrows-a-digest

new_case production-dependency
mkdir -p "$case_root/crates/miso-engine-core"
printf '[dependencies]\nmiso-engine-bench-support.workspace = true\n' \
    >"$case_root/crates/miso-engine-core/Cargo.toml"
expect_failure production-dependency

new_case host-dependency
mkdir -p "$case_root/hosts/miso-engine-host-native"
printf '[dev-dependencies]\nmiso-engine-bench-support.workspace = true\n' \
    >"$case_root/hosts/miso-engine-host-native/Cargo.toml"
expect_failure host-dependency

printf 'bench policy mutations: ok\n'
