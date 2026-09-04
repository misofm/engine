#!/usr/bin/env bash
# One harness under tools/ (#104 F4, phase B).
#
# Audit #104 counted, at `ae02d2a`, fourteen copies of the audited `GlobalAlloc` wrapper in three
# behavioural variants, nine JSON string escapers (two of which emitted invalid JSON) and eight
# nearest-rank percentiles with three edge behaviours. F4's rule is "the second copy is the
# defect", and F2 is what the second copy cost: a runner and its binary drifted until every
# accepted benchmark record carried all-null environment metadata and still passed validation.
#
# Each rule below names exactly the file allowed to own the thing, so a reappearing copy fails
# here rather than in a review six months later.
set -euo pipefail

root="$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)"
cd "$root"

fail() { printf 'bench policy failure: %s\n' "$1" >&2; exit 1; }

sole_owner() {
    local label=$1 owner=$2 pattern=$3
    local found
    found="$(grep -rlE "$pattern" tools --include='*.rs' 2>/dev/null | LC_ALL=C sort || true)"
    [[ "$found" == "$owner" ]] || {
        printf 'expected only %s\nfound:\n%s\n' "$owner" "$found" >&2
        fail "$label"
    }
}

# Like `sole_owner`, but a function outside `owner` matching `def_pattern` is not itself a
# violation if it delegates to the shared `escape(`. Brace-depth tracking turned out to be the
# wrong tool for this (four different ways to desynchronise it, caught in review): this instead
# takes the window from the definition line to the first following line whose content is exactly
# the definition line's own leading indentation followed by `}` -- that is how rustfmt closes a
# `fn` at any nesting depth, so an `if`/`match`/loop's own more-indented `}` inside the body does
# not end the window early -- or a 40-line cap, whichever comes first; if the window runs off the
# end of the file first, it is still judged on what it saw. The window is a delegate iff it
# contains `escape(` on a non-comment line and contains neither `.replace(` nor the char literal
# `'\\'` (four characters: Rust's own spelling of a backslash char literal, not the one-backslash
# value it holds) -- the shape of the exact partial-escaper defect this rule exists to catch
# (`vectorization.rs`'s old `json_string`: it called nothing and hand-escaped `\` and `"` itself,
# and a hypothetical `.chars().flat_map(...)` rewrite of the same defect that never calls
# `.replace(` at all but still hand-rolls a `'\\'` branch). The definition line's own signature is
# exempt from the `escape(` search up through its matched prefix, so a function literally named
# `json_escape` does not "delegate" to itself by spelling `escape(` in its own name -- but any call
# after that prefix, including on a one-line signature-and-body, still counts.
sole_owner_or_delegate() {
    local label=$1 owner=$2 def_pattern=$3
    grep -qE "$def_pattern" "$owner" 2>/dev/null || fail "$label (the shared definition in $owner is gone)"
    local matches offenders=()
    matches="$(grep -rlE "$def_pattern" tools --include='*.rs' 2>/dev/null | LC_ALL=C sort || true)"
    local file awk_pattern="${def_pattern//\\/\\\\}"
    # The 4-character Rust *source* spelling of the backslash char literal -- apostrophe,
    # backslash, backslash, apostrophe, as it appears in `.replace('\\', ...)` -- built from
    # single unambiguous pieces and handed to awk as an environment variable rather than through
    # `-v` (which applies its own C-style backslash processing to `-v` values a second time, and
    # spelling it directly inside the awk script text below is not an option either: that script
    # is itself inside a single-quoted bash string, so an embedded apostrophe would close it
    # early). `ENVIRON` is the process environment verbatim, with no escape processing on either
    # side, so what is built here is exactly what awk sees.
    local sq="'" bs='\'
    local backslash_char_literal="$sq$bs$bs$sq"
    while IFS= read -r file; do
        [[ -z "$file" || "$file" == "$owner" ]] && continue
        if [[ "$(MISO_BENCH_POLICY_NEEDLE="$backslash_char_literal" awk -v pat="$awk_pattern" '
            BEGIN { needle = ENVIRON["MISO_BENCH_POLICY_NEEDLE"] }
            { lines[NR] = $0 }
            $0 ~ pat { starts[NR] = 1 }
            END {
                for (i = 1; i <= NR; i++) {
                    if (!(i in starts)) continue
                    match(lines[i], /^[ \t]*/)
                    closer = substr(lines[i], RSTART, RLENGTH) "}"
                    escapes = 0; replaces = 0; backslash_literal = 0
                    for (j = i; j < i + 40 && j <= NR; j++) {
                        line = lines[j]
                        search_from = 1
                        if (j == i) {
                            match(line, pat)
                            search_from = RSTART + RLENGTH
                        }
                        rest = substr(line, search_from)
                        trimmed = line
                        sub(/^[ \t]+/, "", trimmed)
                        is_comment = (trimmed ~ /^\/\//)
                        if (!is_comment && index(rest, "escape(") > 0) escapes = 1
                        if (index(line, ".replace(") > 0) replaces = 1
                        if (index(line, needle) > 0) backslash_literal = 1
                        if (line == closer) break
                    }
                    print (escapes && !replaces && !backslash_literal ? "delegate" : "own")
                }
            }
        ' "$file")" == *own* ]]; then
            offenders+=("$file")
        fi
    done <<<"$matches"
    ((${#offenders[@]} == 0)) || {
        printf 'expected only %s (or a wrapper whose body calls escape())\noffenders:\n%s\n' \
            "$owner" "$(printf '%s\n' "${offenders[@]}")" >&2
        fail "$label"
    }
}

# Unlike `sole_owner`, this one names nothing that is allowed to hold the pattern: `bench-support`
# hashes through the `sha2` crate, not a hand-rolled initial-hash-word/round-constant table, so the
# pattern below must appear nowhere at all under `tools/`.
forbidden_under_tools() {
    local label=$1 pattern=$2
    local found
    found="$(grep -rlE "$pattern" tools --include='*.rs' 2>/dev/null | LC_ALL=C sort || true)"
    [[ -z "$found" ]] || {
        printf 'found:\n%s\n' "$found" >&2
        fail "$label"
    }
}

support=tools/bench-support
[[ -d "$support" ]] || fail "missing the shared harness: $support"

sole_owner 'the audited allocator has more than one implementation' \
    "$support/src/alloc.rs" '^unsafe impl GlobalAlloc'
sole_owner 'more than one global allocator is registered under tools/' \
    "$support/src/alloc.rs" '^#\[global_allocator\]'
# `\s` is a GNU extension to POSIX ERE, not portable to every `grep -E`/awk in general -- but this
# repo's CI and every host this script is meant to run on use GNU grep and gawk, both of which
# support it, so it is fine here.
sole_owner_or_delegate 'the JSON string escaper has more than one implementation' \
    "$support/src/json.rs" '^\s*(pub(\([a-z]+\))? )?fn (json_(escape|string|quote)|escape)\('
sole_owner 'the nearest-rank percentile has more than one implementation' \
    "$support/src/stats.rs" '^(pub )?fn (percentile|nearest_rank|per_mille|percentile_nearest_rank)'
sole_owner 'the counted SHA-256 sink has more than one implementation' \
    "$support/src/digest.rs" '^(pub )?struct Sha256Sink'
forbidden_under_tools 'a private SHA-256 initial hash word (H0) or round-constant table (K) reappeared under tools/' \
    '0x6a09_?e667|const K: ?\[u32; ?64\]'

# Subjects converted to the shared timer (#104 F1, made structural). `timing::timed` samples the
# digest sink's update counter on both sides of the clock and panics if the timed body hashed
# anything, so "the timer body is arithmetic and the workload, never evidence collection" is a
# property of the run rather than of a review. A converted subject owns no clock and no digest of
# its own: it cannot reintroduce the defect without leaving this list.
#
# The list is the conversion ratchet. It grows as the remaining benchmark subjects move onto the
# shared harness; it never shrinks.
timed_subjects=(tools/bench/src/rack.rs tools/audit/src/fp_env.rs
    tools/wasm-console/src/main.rs)
for subject in "${timed_subjects[@]}"; do
    [[ -f "$subject" ]] || fail "converted subject is missing: $subject"
    grep -q 'timing::timed' "$subject" ||
        fail "converted subject does not measure through timing::timed: $subject"
    if grep -nE 'Instant::now|Sha256::new|sha2::' "$subject"; then
        fail "converted subject owns a clock or a digest of its own: $subject"
    fi
done

# `allow(unsafe_code)` is denied workspace-wide; these six files are the approved exceptions under
# `tools/`, and `scripts/check-realtime-policy.sh` holds the matching list for `crates/` and
# `hosts/`. A seventh file is a new unsafe ownership boundary and needs a decision, not a grep.
#
# The decision for the sixth, `wasm-console-guest` (#163 phase 2 step 1): it is the
# *same* boundary this list already grants `wasm-gate-guest`, for the same reason and
# with the same shape. Exporting a function from a `cdylib` requires `#[unsafe(no_mangle)]` under
# edition 2024 and there is no safe spelling of it. Both guests are `u32`-in/`u32`-out, neither
# dereferences a pointer, neither declares a memory contract with its host, and no engine crate
# links either. This is not a new *kind* of exception; it is a second instance of the one already
# approved, and it is named here rather than absorbed by a pattern so that a genuinely new
# boundary still has to come back for a decision.
expected_unsafe="$(printf '%s\n' \
    tools/bench-support/src/alloc.rs \
    tools/audit/src/capi.rs \
    tools/native-pcm-runner/src/lib.rs \
    tools/bench/src/protocol.rs \
    tools/wasm-gate-guest/src/lib.rs \
    tools/wasm-console-guest/src/lib.rs | LC_ALL=C sort)"
actual_unsafe="$(grep -rlE '^#!\[allow\(unsafe_code\)\]' tools --include='*.rs' 2>/dev/null | LC_ALL=C sort || true)"
[[ "$actual_unsafe" == "$expected_unsafe" ]] || {
    diff -u <(printf '%s\n' "$expected_unsafe") <(printf '%s\n' "$actual_unsafe") >&2 || true
    fail 'the approved unsafe ownership set under tools/ changed'
}

# F2's metadata boundary: only the two dispatchers may inspect their private re-exec selector.
# Every subject reads runner metadata from one memoized in-process `Metadata::gather()` snapshot.
expected_environment_readers="$(printf '%s\n' \
    tools/audit/src/main.rs \
    tools/bench/src/main.rs | LC_ALL=C sort)"
actual_environment_readers="$(grep -rlE '(std::)?env::var\(' tools/{audit,bench,bench-support}/src \
    --include='*.rs' 2>/dev/null | LC_ALL=C sort || true)"
[[ "$actual_environment_readers" == "$expected_environment_readers" ]] || {
    diff -u <(printf '%s\n' "$expected_environment_readers") \
        <(printf '%s\n' "$actual_environment_readers") >&2 || true
    fail 'a subject bypassed the shared in-process metadata snapshot'
}

# The shared harness is test scaffolding. A production package depending on it would put a
# `#[global_allocator]` and an abort-on-allocation policy into a shipped artifact.
#
# Amended by #105 phase 2, deliberately and narrowly: a `[dev-dependencies]` edge from a crate
# under `crates/` is allowed, because it cannot reach a shipped artifact (dev dependencies are not
# part of any non-test build graph -- `scripts/check-realtime-audit-leak.sh` proves that with
# `cargo tree -e features,no-dev`), and because it is what makes the effect-contract conformance
# allocation gate real. `run_effect_conformance` can only observe an allocation inside an armed
# render scope if the *test binary* that calls it installs the audited counting allocator; the
# harness refuses to run with `harness.allocator_not_installed` when it is missing, so the edge is
# load-bearing rather than convenient. `hosts/` and `sidecars/` keep the absolute ban in both
# sections: a host adapter and a sidecar are each the artifact that ships.
while IFS= read -r manifest; do
    violation="$(awk -v file="$manifest" '
        /^\[/ { section = $0 }
        /bench-support/ {
            dev = section ~ /dev-dependencies/ && file ~ /^crates\//
            if (!dev) { print file ": " $0; exit }
        }
    ' "$manifest")"
    [[ -z "$violation" ]] || fail "a production package depends on the bench support crate: $violation"
done < <(find crates hosts sidecars -mindepth 2 -maxdepth 2 -name Cargo.toml 2>/dev/null | sort)

printf 'bench policy: ok (1 allocator, 1 escaper, 1 percentile, 1 digest sink, %s unsafe owners, %s subjects on the shared timer)\n' \
    "$(printf '%s\n' "$expected_unsafe" | wc -l | tr -d ' ')" "${#timed_subjects[@]}"
