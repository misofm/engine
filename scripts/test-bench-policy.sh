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
    mkdir -p "$case_root/crates/fixture" "$case_root/hosts/fixture" "$case_root/sidecars/fixture"
    printf '[package]\nname = "fixture"\nversion = "0.1.0"\nedition = "2021"\n' \
        >"$case_root/crates/fixture/Cargo.toml"
    cp "$root/scripts/check-bench-policy.sh" "$case_root/scripts/"
}

check() { bash "$case_root/scripts/check-bench-policy.sh" "$case_root"; }

expect_failure() {
    if check >/dev/null 2>&1; then
        printf 'bench policy mutation escaped: %s\n' "$1" >&2
        exit 1
    fi
}

expect_failure_with_path() {
    local label="$1" shim="$2"
    if PATH="$shim:$PATH" check >/dev/null 2>&1; then
        printf 'bench policy mutation escaped: %s\n' "$label" >&2
        exit 1
    fi
}

new_case baseline
check >/dev/null

for required_root in crates hosts sidecars; do
    new_case "missing-$required_root"
    rm -rf "$case_root/$required_root"
    expect_failure "missing-$required_root"
done

new_case empty-manifest-population
rm -rf "$case_root/crates" "$case_root/hosts" "$case_root/sidecars"
mkdir -p "$case_root/crates" "$case_root/hosts" "$case_root/sidecars"
expect_failure empty-manifest-population

new_case manifest-discovery-status-loss
mkdir -p "$case_root/shim"
printf '#!/usr/bin/env bash\nprintf "crates/fixture/Cargo.toml\\n"\nexit 1\n' \
    >"$case_root/shim/find"
chmod +x "$case_root/shim/find"
expect_failure_with_path manifest-discovery-status-loss "$case_root/shim"

new_case manifest-sort-error
mkdir -p "$case_root/shim"
printf '#!/usr/bin/env bash\nexit 2\n' >"$case_root/shim/sort"
chmod +x "$case_root/shim/sort"
expect_failure_with_path manifest-sort-error "$case_root/shim"

new_case manifest-awk-error
mkdir -p "$case_root/shim"
printf '#!/usr/bin/env bash\nexit 2\n' >"$case_root/shim/awk"
chmod +x "$case_root/shim/awk"
expect_failure_with_path manifest-awk-error "$case_root/shim"

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

# #380: the widened escaper pattern also has to catch a `json_string`/`json_quote`-named
# reimplementation, not just `escape`/`json_escape` -- this is the exact shape
# `tools/audit/src/vectorization.rs`'s `json_string` had before this issue removed it.
new_case second-json-string-name
printf '\nfn json_string(value: &str) -> String {\n    value.replace('"'"'\\\\'"'"', "\\\\\\\\")\n}\n' \
    >>"$case_root/tools/bench/src/conformance.rs"
expect_failure second-json-string-name

# A local wrapper that only calls the shared `escape` is not the defect (`tools/bench/src/builtins.rs`
# and `tools/bench/src/effect_interchange.rs` both carry one); the baseline case above already
# proves that shape stays green.

# A delegating wrapper whose signature rustfmt has wrapped across multiple lines is still a
# delegate, not a reimplementation: the window scan has to reach the line that actually calls
# `escape(`, however many signature lines come first.
new_case json-string-multiline-signature-delegate-stays-green
printf '\nfn json_string(\n    value: &str,\n) -> String {\n    format!("\\"{}\\"", json::escape(value))\n}\n' \
    >>"$case_root/tools/bench/src/conformance.rs"
check >/dev/null || {
    printf 'bench policy rejects a delegating json_string whose signature spans multiple lines\n' >&2
    exit 1
}

# A one-line delegating wrapper immediately followed by unrelated code must still read as a
# delegate: the window closes on the next line at the definition's own indentation followed by
# `}`, or a 40-line cap, not on brace balance.
new_case json-string-one-liner-delegate-followed-by-other-code-stays-green
printf '\nfn json_string(value: &str) -> String { format!("\\"{}\\"", json::escape(value)) }\n\nfn something_else() -> u32 {\n    1\n}\n' \
    >>"$case_root/tools/bench/src/conformance.rs"
check >/dev/null || {
    printf 'bench policy rejects a one-line delegating json_string followed by other code\n' >&2
    exit 1
}

new_case json-string-non-delegating-one-liner-as-last-item
printf '\nfn json_string(value: &str) -> String { value.to_owned() }\n' \
    >>"$case_root/tools/bench/src/conformance.rs"
expect_failure json-string-non-delegating-one-liner-as-last-item

# The exact partial-escaper shape this rule exists to catch: it calls the shared `escape` and then
# keeps hand-rolling more escaping of its own, same as `vectorization.rs`'s old `json_string` did
# with `.replace('\\', ...)`.
new_case json-string-delegates-then-replaces
printf '\nfn json_string(value: &str) -> String {\n    let escaped = json::escape(value);\n    escaped.replace("<", "&lt;")\n}\n' \
    >>"$case_root/tools/bench/src/conformance.rs"
expect_failure json-string-delegates-then-replaces

# `escape(` appearing only in a comment is not a delegating call.
new_case json-string-escape-mentioned-only-in-a-comment
printf '\nfn json_string(value: &str) -> String {\n    // escape(value) used to be called here\n    value.to_owned()\n}\n' \
    >>"$case_root/tools/bench/src/conformance.rs"
expect_failure json-string-escape-mentioned-only-in-a-comment

# The widened definition anchor (`^\s*(pub(\([a-z]+\))? )?fn`) has to see a reimplementation that
# is indented (inside a module) or spelled `pub(crate)`, not just a column-zero `pub`/bare `fn`.
new_case json-string-indented-inside-a-module
printf '\nmod scratch {\n    fn json_string(value: &str) -> String {\n        value.to_owned()\n    }\n}\n' \
    >>"$case_root/tools/bench/src/conformance.rs"
expect_failure json-string-indented-inside-a-module

new_case json-string-pub-crate-non-delegating
printf '\npub(crate) fn json_string(value: &str) -> String {\n    value.to_owned()\n}\n' \
    >>"$case_root/tools/bench/src/conformance.rs"
expect_failure json-string-pub-crate-non-delegating

# A nested block (an `if` with its own more-indented `}`) inside a delegating wrapper's body must
# not close the window before the delegating call is seen: the window closer is the definition
# line's own indentation followed by `}`, not the first lone `}` at any depth.
new_case json-string-nested-if-before-delegating-call-stays-green
printf '\nfn json_string(value: &str) -> String {\n    if value.is_empty() {\n        return "\\"\\"".to_owned();\n    }\n    format!("\\"{}\\"", bench_support::json::escape(value))\n}\n' \
    >>"$case_root/tools/bench/src/conformance.rs"
check >/dev/null || {
    printf 'bench policy rejects a delegating json_string with a nested if before the call\n' >&2
    exit 1
}

# The backslash-literal needle has to be load-bearing on its own, not just redundant with
# `.replace(`: a reimplementation that calls the shared `escape` (so the escape( check alone would
# pass it) and then hand-rolls its own backslash handling through `.chars().flat_map(...)` -- no
# `.replace(` anywhere -- is still the partial-escaper defect and must still fail.
new_case json-string-delegates-then-hand-rolls-backslash-via-flat-map
printf '\nfn json_string(value: &str) -> String {\n    let escaped = bench_support::json::escape(value);\n    let doubled: String = escaped\n        .chars()\n        .flat_map(|c| if c == '"'"'\\\\'"'"' { vec!['"'"'\\\\'"'"', '"'"'\\\\'"'"'] } else { vec![c] })\n        .collect();\n    doubled\n}\n' \
    >>"$case_root/tools/bench/src/conformance.rs"
expect_failure json-string-delegates-then-hand-rolls-backslash-via-flat-map

# #380: the private SHA-256 round-constant table `tools/bench/src/session.rs` used to carry.
new_case second-sha256-round-constant
printf '\nconst K: [u32; 64] = [0; 64];\n' \
    >>"$case_root/tools/bench/src/conformance.rs"
expect_failure second-sha256-round-constant

new_case second-sha256-initial-constant
printf '\nconst INITIAL: u32 = 0x6a09_e667;\n' \
    >>"$case_root/tools/bench/src/conformance.rs"
expect_failure second-sha256-initial-constant

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

new_case new-unsafe-owner
printf '#![allow(unsafe_code)]\n' \
    >>"$case_root/tools/audit/src/source.rs"
expect_failure new-unsafe-owner

new_case retired-unsafe-owner
sed -i '/^#!\[allow(unsafe_code)\]$/d' "$case_root/tools/audit/src/capi.rs"
expect_failure retired-unsafe-owner

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
