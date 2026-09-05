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
    printf '[package]\nname = "later-fixture"\nversion = "0.1.0"\nedition = "2021"\n' \
        >"$case_root/hosts/fixture/Cargo.toml"
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
    local label="$1" shim="$2" expected=${3:-} witness=${4:-}
    local output status
    if output="$(PATH="$shim:$PATH" check 2>&1)"; then status=0; else status=$?; fi
    if ((status == 0)); then
        printf 'bench policy mutation escaped: %s\n' "$label" >&2
        exit 97
    fi
    if [[ -n "$expected" && "$output" != *"$expected"* ]]; then
        printf 'bench policy wrong diagnostic: %s\n%s\n' "$label" "$output" >&2
        exit 96
    fi
    if [[ -n "$witness" && "$output" != *"$witness"* ]]; then
        printf 'bench policy missing diagnostic witness: %s: %s\n%s\n' "$label" "$witness" "$output" >&2
        exit 96
    fi
}

new_case baseline
check >/dev/null

for required_root in crates hosts sidecars; do
    new_case "missing-$required_root"
    rm -rf "$case_root/$required_root"
    output="$(check 2>&1)" && status=0 || status=$?
    ((status != 0)) || { printf 'bench policy mutation escaped: missing-%s\n' "$required_root" >&2; exit 1; }
    [[ "$output" == *"missing required root: $required_root"* ]] || { printf 'bench policy wrong missing-root diagnostic: %s\n' "$output" >&2; exit 1; }
done

new_case empty-manifest-population
rm -rf "$case_root/crates" "$case_root/hosts" "$case_root/sidecars"
mkdir -p "$case_root/crates" "$case_root/hosts" "$case_root/sidecars"
output="$(check 2>&1)" && status=0 || status=$?
((status != 0)) || { echo 'bench policy empty manifest population escaped' >&2; exit 1; }
[[ "$output" == *'manifest discovery produced no packages'* ]] || { printf 'bench policy wrong empty-population diagnostic: %s\n' "$output" >&2; exit 1; }

new_case manifest-discovery-status-loss
mkdir -p "$case_root/shim"
printf '#!/usr/bin/env bash\n/usr/bin/find "$@"\nprintf "find-error-sentinel\\n" >&2\nexit 7\n' \
    >"$case_root/shim/find"
chmod +x "$case_root/shim/find"
expect_failure_with_path manifest-discovery-status-loss "$case_root/shim" 'manifest discovery failed with status 7; output: crates/fixture/Cargo.toml
hosts/fixture/Cargo.toml; stderr: find-error-sentinel'

new_case manifest-discovery-empty-error
mkdir -p "$case_root/shim"
printf '#!/usr/bin/env bash\nprintf "find-empty-error-sentinel\\n" >&2\nexit 7\n' >"$case_root/shim/find"
chmod +x "$case_root/shim/find"
expect_failure_with_path manifest-discovery-empty-error "$case_root/shim" 'manifest discovery failed with status 7; output: <empty>; stderr: find-empty-error-sentinel'

new_case manifest-sort-error
mkdir -p "$case_root/shim"
printf '#!/usr/bin/env bash\n/usr/bin/sort "$@"\nprintf "sort-error-sentinel\\n" >&2\nexit 2\n' >"$case_root/shim/sort"
chmod +x "$case_root/shim/sort"
expect_failure_with_path manifest-sort-error "$case_root/shim" 'manifest sort failed with status 2; output: crates/fixture/Cargo.toml
hosts/fixture/Cargo.toml; input:' 'sort-error-sentinel'

new_case manifest-sort-empty-error
mkdir -p "$case_root/shim"
printf '#!/usr/bin/env bash\nprintf "sort-empty-error-sentinel\\n" >&2\nexit 2\n' >"$case_root/shim/sort"
chmod +x "$case_root/shim/sort"
expect_failure_with_path manifest-sort-empty-error "$case_root/shim" 'manifest sort failed with status 2; output: <empty>; input:' 'sort-empty-error-sentinel'

new_case manifest-awk-error
mkdir -p "$case_root/shim"
printf '#!/usr/bin/env bash\nif [[ " $* " == *"file=hosts/fixture/Cargo.toml"* ]]; then printf "manifest-awk-error\\n" >&2; exit 2; fi\nexec /usr/bin/awk "$@"\n' >"$case_root/shim/awk"
chmod +x "$case_root/shim/awk"
expect_failure_with_path manifest-awk-error "$case_root/shim" 'dependency parser failed for hosts/fixture/Cargo.toml with status 2; output: <empty>; stderr: manifest-awk-error'

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

# Selective execution faults cover every remaining grep/parser/owner/count stage. Unselected
# invocations delegate to the real tool, so each case reaches the named operation on a valid tree.
grep_fault() {
    local label=$1 selector=$2 expected=$3 mode=${4:-real}
    new_case "$label"
    mkdir -p "$case_root/shim"
    printf '#!/usr/bin/env bash\nselector=%q\nmode=%q\nif [[ " $* " == *"$selector"* ]]; then [[ "$mode" == real ]] && /usr/bin/grep "$@"; printf "grep-error-sentinel\\n" >&2; exit 7; fi\nexec /usr/bin/grep "$@"\n' \
        "$selector" "$mode" >"$case_root/shim/grep"
    chmod +x "$case_root/shim/grep"
    expect_failure_with_path "$label" "$case_root/shim" "$expected" 'grep-error-sentinel'
}

multifile_grep_fault() {
    local label=$1 selector=$2 operation=$3 sentinel=$4 expected_files=$5 mode case_label
    local output status prefix suffix bounded replay expected_sorted actual_sorted line_count
    for mode in real reversed; do
        case_label=$label
        [[ "$mode" == real ]] || case_label="$label-reversed"
        new_case "$case_label"
        mkdir -p "$case_root/shim"
        printf '#!/usr/bin/env bash\nselector=%q\nmode=%q\npayload=%q\nreplay=%q\nif [[ " $* " == *"--include=*.rs"* && " $* " == *"$selector"* ]]; then\n    if /usr/bin/grep "$@" >"$payload"; then producer_status=0; else producer_status=$?; fi\n    printf "%%s\\n" "$producer_status" >"$payload.status"\n    ((producer_status == 0)) || { printf "selected-grep-setup-status=%%s\\n" "$producer_status" >&2; exit 96; }\n    if [[ "$mode" == reversed ]]; then /usr/bin/tac "$payload" >"$replay"; else cp "$payload" "$replay"; fi\n    /usr/bin/cat "$replay"\n    printf "%%s\\n" %q >&2\n    exit 7\nfi\nexec /usr/bin/grep "$@"\n' \
            "$selector" "$mode" "$case_root/selected.payload" "$case_root/replayed.payload" "$sentinel" \
            >"$case_root/shim/grep"
        chmod +x "$case_root/shim/grep"
        if output="$(PATH="$case_root/shim:$PATH" check 2>&1)"; then status=0; else status=$?; fi
        ((status != 0)) || { printf 'bench policy mutation escaped: %s\n' "$case_label" >&2; exit 97; }
        [[ "$(<"$case_root/selected.payload.status")" == 0 ]] || { printf 'selected grep was not successful: %s\n' "$case_label" >&2; exit 96; }
        replay="$(<"$case_root/replayed.payload")"
        prefix="$operation failed with status 7; output: "
        suffix="; stderr: $sentinel"
        [[ "$output" == *"$prefix"*"$suffix"* ]] || { printf 'bench policy wrong diagnostic: %s\n%s\n' "$case_label" "$output" >&2; exit 96; }
        bounded=${output#*"$prefix"}
        bounded=${bounded%%"$suffix"*}
        [[ "$bounded" == "$replay" ]] || { printf 'bench policy truncated/reordered diagnostic payload: %s\nexpected:\n%s\nactual:\n%s\n' "$case_label" "$replay" "$bounded" >&2; exit 96; }
        expected_sorted="$(printf '%s\n' "$expected_files" | LC_ALL=C sort)"
        actual_sorted="$(LC_ALL=C sort "$case_root/selected.payload")"
        [[ "$actual_sorted" == "$expected_sorted" ]] || { printf 'bench policy incomplete selected grep payload: %s\nexpected:\n%s\nactual:\n%s\n' "$case_label" "$expected_sorted" "$actual_sorted" >&2; exit 96; }
        line_count="$(wc -l <"$case_root/selected.payload" | tr -d ' ')"
        if [[ "$mode" == reversed ]]; then
            ((line_count > 1)) || { printf 'bench policy reversal needs multiple lines: %s\n' "$case_label" >&2; exit 96; }
            [[ "$(<"$case_root/selected.payload")" != "$replay" ]] || { printf 'bench policy reversal did not change order: %s\n' "$case_label" >&2; exit 96; }
        fi
        printf 'bench policy selected grep evidence: %s producer-status=0 lines=%s\npayload:\n%s\nreplay:\n%s\ndiagnostic-output:\n%s\n' \
            "$case_label" "$line_count" "$(<"$case_root/selected.payload")" "$replay" "$bounded"
    done
}

grep_fault owner-grep-error '^unsafe impl GlobalAlloc' 'grep failed with status 7; output: tools/bench-support/src/alloc.rs; stderr: grep-error-sentinel'
grep_fault owner-grep-empty-error '^unsafe impl GlobalAlloc' 'grep failed with status 7; output: <empty>; stderr: grep-error-sentinel' empty
grep_fault allocator-registration-grep-error 'global_allocator' 'grep failed with status 7; output: tools/bench-support/src/alloc.rs; stderr: grep-error-sentinel'
grep_fault allocator-registration-grep-empty-error 'global_allocator' 'grep failed with status 7; output: <empty>; stderr: grep-error-sentinel' empty
grep_fault percentile-grep-error 'nearest_rank' 'grep failed with status 7; output: tools/bench-support/src/stats.rs; stderr: grep-error-sentinel'
grep_fault percentile-grep-empty-error 'nearest_rank' 'grep failed with status 7; output: <empty>; stderr: grep-error-sentinel' empty
grep_fault digest-grep-error 'Sha256Sink' 'grep failed with status 7; output: tools/bench-support/src/digest.rs; stderr: grep-error-sentinel'
grep_fault digest-grep-empty-error 'Sha256Sink' 'grep failed with status 7; output: <empty>; stderr: grep-error-sentinel' empty
grep_fault escaper-presence-error 'tools/bench-support/src/json.rs' 'shared-definition grep failed or is empty for tools/bench-support/src/json.rs; status 7; output:'
grep_fault escaper-presence-empty-error 'tools/bench-support/src/json.rs' 'shared-definition grep failed or is empty for tools/bench-support/src/json.rs; status 7; output: <empty>; stderr: grep-error-sentinel' empty
multifile_grep_fault escaper-candidate-grep-error 'json_(escape|string|quote)' grep grep-error-sentinel $'tools/bench/src/effect_interchange.rs\ntools/bench/src/builtins.rs\ntools/bench-support/src/json.rs'
new_case escaper-candidate-grep-empty-error
mkdir -p "$case_root/shim"
printf '#!/usr/bin/env bash\nif [[ " $* " == *"--include=*.rs"* && " $* " == *"json_(escape|string|quote)"* ]]; then printf "grep-error-sentinel\\n" >&2; exit 7; fi\nexec /usr/bin/grep "$@"\n' >"$case_root/shim/grep"
chmod +x "$case_root/shim/grep"
expect_failure_with_path escaper-candidate-grep-empty-error "$case_root/shim" 'grep failed with status 7; output: <empty>; stderr: grep-error-sentinel'
grep_fault private-sha-grep-error '0x6a09_' 'grep failed with status 7; output: <empty>; stderr: grep-error-sentinel' empty
multifile_grep_fault unsafe-owner-grep-error 'unsafe_code' 'unsafe-owner scan' grep-error-sentinel $'tools/bench-support/src/alloc.rs\ntools/audit/src/capi.rs\ntools/native-pcm-runner/src/lib.rs\ntools/bench/src/protocol.rs\ntools/wasm-gate-guest/src/lib.rs\ntools/wasm-console-guest/src/lib.rs'
grep_fault unsafe-owner-grep-empty-error 'unsafe_code' 'unsafe-owner scan failed with status 7; output: <empty>; stderr: grep-error-sentinel' empty
multifile_grep_fault environment-reader-grep-error 'env::var' 'environment-reader scan' grep-error-sentinel $'tools/audit/src/main.rs\ntools/bench/src/main.rs'
grep_fault environment-reader-grep-empty-error 'env::var' 'environment-reader scan failed with status 7; output: <empty>; stderr: grep-error-sentinel' empty

new_case delegate-parser-output-error
mkdir -p "$case_root/shim"
printf '#!/usr/bin/env bash\nif [[ -n "${MISO_ENGINE_BENCH_POLICY_NEEDLE:-}" && "${@: -1}" == "tools/bench/src/effect_interchange.rs" ]]; then /usr/bin/awk "$@"; printf "delegate-error-sentinel\\n" >&2; exit 6; fi\nexec /usr/bin/awk "$@"\n' >"$case_root/shim/awk"
chmod +x "$case_root/shim/awk"
expect_failure_with_path delegate-parser-output-error "$case_root/shim" 'delegate parser failed for tools/bench/src/effect_interchange.rs with status 6; output: delegate; stderr: delegate-error-sentinel'

new_case delegate-parser-empty-error
mkdir -p "$case_root/shim"
printf '#!/usr/bin/env bash\nif [[ -n "${MISO_ENGINE_BENCH_POLICY_NEEDLE:-}" && "${@: -1}" == "tools/bench/src/effect_interchange.rs" ]]; then printf "delegate-empty-error-sentinel\\n" >&2; exit 6; fi\nexec /usr/bin/awk "$@"\n' >"$case_root/shim/awk"
chmod +x "$case_root/shim/awk"
expect_failure_with_path delegate-parser-empty-error "$case_root/shim" 'delegate parser failed for tools/bench/src/effect_interchange.rs with status 6; output: <empty>; stderr: delegate-empty-error-sentinel'

new_case later-timed-marker-error
mkdir -p "$case_root/shim"
printf '#!/usr/bin/env bash\nif [[ " $* " == *"timing::timed"* && "${@: -1}" == "tools/wasm-console/src/main.rs" ]]; then /usr/bin/grep "$@"; printf "later-timer-error-sentinel\\n" >&2; exit 7; fi\nexec /usr/bin/grep "$@"\n' >"$case_root/shim/grep"
chmod +x "$case_root/shim/grep"
expect_failure_with_path later-timed-marker-error "$case_root/shim" 'converted subject timer scan failed or is empty for tools/wasm-console/src/main.rs; status 7; output:' 'later-timer-error-sentinel'

new_case later-timed-marker-empty-error
mkdir -p "$case_root/shim"
printf '#!/usr/bin/env bash\nif [[ " $* " == *"timing::timed"* && "${@: -1}" == "tools/wasm-console/src/main.rs" ]]; then printf "later-timer-empty-error-sentinel\\n" >&2; exit 7; fi\nexec /usr/bin/grep "$@"\n' >"$case_root/shim/grep"
chmod +x "$case_root/shim/grep"
expect_failure_with_path later-timed-marker-empty-error "$case_root/shim" 'converted subject timer scan failed or is empty for tools/wasm-console/src/main.rs; status 7; output: <empty>; stderr: later-timer-empty-error-sentinel'

new_case later-timed-forbidden-error
mkdir -p "$case_root/shim"
printf '#!/usr/bin/env bash\nif [[ " $* " == *"Instant::now"* && "${@: -1}" == "tools/wasm-console/src/main.rs" ]]; then printf "later-forbidden-error-sentinel\\n" >&2; exit 7; fi\nexec /usr/bin/grep "$@"\n' >"$case_root/shim/grep"
chmod +x "$case_root/shim/grep"
expect_failure_with_path later-timed-forbidden-error "$case_root/shim" 'subject scan failed with status 7; output: <empty>; stderr: later-forbidden-error-sentinel'

sort_fault() {
    local label=$1 ordinal=$2 expected=$3 mode=${4:-real}
    new_case "$label"
    mkdir -p "$case_root/shim"
    printf '#!/usr/bin/env bash\nstate=${TMPDIR:-/tmp}/bench-sort-fault-%s\nmode=%q\nn=0; [[ -f "$state" ]] && n=$(<"$state"); n=$((n+1)); printf "%%s" "$n" >"$state"\nif ((n == %s)); then [[ "$mode" == real ]] && /usr/bin/sort "$@"; printf "sort-error-sentinel\\n" >&2; exit 8; fi\nexec /usr/bin/sort "$@"\n' "$label" "$mode" "$ordinal" >"$case_root/shim/sort"
    chmod +x "$case_root/shim/sort"
    rm -f "/tmp/bench-sort-fault-$label"
    expect_failure_with_path "$label" "$case_root/shim" "$expected" 'sort-error-sentinel'
    rm -f "/tmp/bench-sort-fault-$label"
}

sort_fault allocator-owner-sort-error 2 'allocator has more than one implementation (sort failed with status 8; output: tools/bench-support/src/alloc.rs'
sort_fault allocator-owner-sort-empty-error 2 'allocator has more than one implementation (sort failed with status 8; output: <empty>' empty
sort_fault allocator-registration-sort-error 3 'global allocator is registered under tools/ (sort failed with status 8; output: tools/bench-support/src/alloc.rs'
sort_fault allocator-registration-sort-empty-error 3 'global allocator is registered under tools/ (sort failed with status 8; output: <empty>' empty
sort_fault escaper-candidate-sort-error 4 'JSON string escaper has more than one implementation (sort failed with status 8; output:'
sort_fault escaper-candidate-sort-empty-error 4 'JSON string escaper has more than one implementation (sort failed with status 8; output: <empty>' empty
sort_fault percentile-sort-error 5 'nearest-rank percentile has more than one implementation (sort failed with status 8; output: tools/bench-support/src/stats.rs'
sort_fault percentile-sort-empty-error 5 'nearest-rank percentile has more than one implementation (sort failed with status 8; output: <empty>' empty
sort_fault digest-sort-error 6 'counted SHA-256 sink has more than one implementation (sort failed with status 8; output: tools/bench-support/src/digest.rs'
sort_fault digest-sort-empty-error 6 'counted SHA-256 sink has more than one implementation (sort failed with status 8; output: <empty>' empty
sort_fault private-sha-sort-error 7 'private SHA-256 initial hash word (H0) or round-constant table (K) reappeared under tools/ (sort failed with status 8; output: <empty>'
sort_fault unsafe-expected-sort-error 8 'unsafe expected-owner sort failed with status 8; output:'
sort_fault unsafe-expected-sort-empty-error 8 'unsafe expected-owner sort failed with status 8; output: <empty>' empty
sort_fault unsafe-discovered-sort-error 9 'unsafe-owner sort failed with status 8; output:'
sort_fault unsafe-discovered-sort-empty-error 9 'unsafe-owner sort failed with status 8; output: <empty>' empty
sort_fault environment-expected-sort-error 10 'environment expected-owner sort failed with status 8; output:'
sort_fault environment-expected-sort-empty-error 10 'environment expected-owner sort failed with status 8; output: <empty>' empty
sort_fault environment-discovered-sort-error 11 'environment-reader sort failed with status 8; output:'
sort_fault environment-discovered-sort-empty-error 11 'environment-reader sort failed with status 8; output: <empty>' empty

new_case count-error
mkdir -p "$case_root/shim"
printf '#!/usr/bin/env bash\n/usr/bin/wc "$@"\nprintf "count-error-sentinel\\n" >&2\nexit 9\n' >"$case_root/shim/wc"
chmod +x "$case_root/shim/wc"
expect_failure_with_path count-error "$case_root/shim" 'unsafe-owner count failed with status 9; output: 6; stderr: count-error-sentinel'

new_case count-empty-error
mkdir -p "$case_root/shim"
printf '#!/usr/bin/env bash\nprintf "count-empty-error-sentinel\\n" >&2\nexit 9\n' >"$case_root/shim/wc"
chmod +x "$case_root/shim/wc"
expect_failure_with_path count-empty-error "$case_root/shim" 'unsafe-owner count failed with status 9; output: <empty>; stderr: count-empty-error-sentinel'

new_case count-formatter-error
mkdir -p "$case_root/shim"
printf '#!/usr/bin/env bash\n/usr/bin/tr "$@"\nprintf "formatter-error-sentinel\\n" >&2\nexit 10\n' >"$case_root/shim/tr"
chmod +x "$case_root/shim/tr"
expect_failure_with_path count-formatter-error "$case_root/shim" 'unsafe-owner count formatter failed with status 10; output: 6; input: 6; stderr: formatter-error-sentinel'

new_case count-formatter-empty-error
mkdir -p "$case_root/shim"
printf '#!/usr/bin/env bash\nprintf "formatter-empty-error-sentinel\\n" >&2\nexit 10\n' >"$case_root/shim/tr"
chmod +x "$case_root/shim/tr"
expect_failure_with_path count-formatter-empty-error "$case_root/shim" 'unsafe-owner count formatter failed with status 10; output: <empty>; input: 6; stderr: formatter-empty-error-sentinel'

printf 'bench policy mutations: ok\n'
