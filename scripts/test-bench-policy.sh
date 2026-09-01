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
    >>"$case_root/tools/bench/src/rack.rs"
expect_failure second-allocator

new_case second-global-allocator-attribute
printf '\n#[global_allocator]\nstatic A: X = X;\n' \
    >>"$case_root/tools/bench/src/session.rs"
expect_failure second-global-allocator-attribute

new_case second-escaper
printf '\nfn json_escape(value: &str) -> String {\n    value.to_owned()\n}\n' \
    >>"$case_root/tools/bench/src/conformance.rs"
expect_failure second-escaper

new_case second-percentile
printf '\nfn percentile(sorted: &[u64], p: usize) -> u64 {\n    sorted[p]\n}\n' \
    >>"$case_root/tools/bench/src/graph.rs"
expect_failure second-percentile

new_case second-digest-sink
printf '\nstruct Sha256Sink;\n' \
    >>"$case_root/tools/bench/src/builtins.rs"
expect_failure second-digest-sink

new_case removed-escaper
rm "$case_root/tools/bench-support/src/json.rs"
expect_failure removed-escaper

new_case converted-subject-loses-the-shared-timer
sed -i 's/timing::timed/inline_timed/' "$case_root/tools/bench/src/rack.rs"
expect_failure converted-subject-loses-the-shared-timer

new_case converted-subject-regrows-a-clock
printf '\nfn t() { let _ = Instant::now(); }\n' \
    >>"$case_root/tools/bench/src/rack.rs"
expect_failure converted-subject-regrows-a-clock

new_case converted-subject-regrows-a-digest
printf '\nfn h() { let _ = Sha256::new(); }\n' \
    >>"$case_root/tools/bench/src/rack.rs"
expect_failure converted-subject-regrows-a-digest

new_case subject-bypasses-metadata-snapshot
printf '\nfn bypass() { let _ = std::env::var("CPU"); }\n' \
    >>"$case_root/tools/bench/src/rack.rs"
expect_failure subject-bypasses-metadata-snapshot

new_case production-dependency
mkdir -p "$case_root/crates/engine"
printf '[dependencies]\nbench-support.workspace = true\n' \
    >"$case_root/crates/engine/Cargo.toml"
expect_failure production-dependency

new_case host-dependency
mkdir -p "$case_root/hosts/host-native"
printf '[dev-dependencies]\nbench-support.workspace = true\n' \
    >"$case_root/hosts/host-native/Cargo.toml"
expect_failure host-dependency

# #105 phase 2 amended the rule to be section-aware. These two cases pin both halves of it: the
# dev edge the conformance allocation gate needs is legal under crates/, and the same crate
# growing a real dependency on the harness is not.
new_case crates-dev-dependency-is-allowed
mkdir -p "$case_root/crates/compressor"
printf '[dev-dependencies]\nbench-support.workspace = true\n' \
    >"$case_root/crates/compressor/Cargo.toml"
check >/dev/null || {
    printf 'bench policy rejects the #105 conformance dev edge under crates/\n' >&2
    exit 1
}

new_case crates-dev-dependency-promoted-to-a-real-one
mkdir -p "$case_root/crates/compressor"
printf '[dependencies]\nbench-support.workspace = true\n\n[dev-dependencies]\nsha2.workspace = true\n' \
    >"$case_root/crates/compressor/Cargo.toml"
expect_failure crates-dev-dependency-promoted-to-a-real-one

printf 'bench policy mutations: ok\n'
