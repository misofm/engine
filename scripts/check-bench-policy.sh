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

support=tools/miso-engine-bench-support
[[ -d "$support" ]] || fail "missing the shared harness: $support"

sole_owner 'the audited allocator has more than one implementation' \
    "$support/src/alloc.rs" '^unsafe impl GlobalAlloc'
sole_owner 'more than one global allocator is registered under tools/' \
    "$support/src/alloc.rs" '^#\[global_allocator\]'
sole_owner 'the JSON string escaper has more than one implementation' \
    "$support/src/json.rs" '^(pub )?fn (json_)?escape\('
sole_owner 'the nearest-rank percentile has more than one implementation' \
    "$support/src/stats.rs" '^(pub )?fn (percentile|nearest_rank|per_mille|percentile_nearest_rank)'
sole_owner 'the counted SHA-256 sink has more than one implementation' \
    "$support/src/digest.rs" '^(pub )?struct Sha256Sink'

# Subjects converted to the shared timer (#104 F1, made structural). `timing::timed` samples the
# digest sink's update counter on both sides of the clock and panics if the timed body hashed
# anything, so "the timer body is arithmetic and the workload, never evidence collection" is a
# property of the run rather than of a review. A converted subject owns no clock and no digest of
# its own: it cannot reintroduce the defect without leaving this list.
#
# The list is the conversion ratchet. It grows as the remaining benchmark subjects move onto the
# shared harness; it never shrinks.
timed_subjects=(tools/miso-engine-rack-bench)
for subject in "${timed_subjects[@]}"; do
    [[ -d "$subject" ]] || fail "converted subject is missing: $subject"
    grep -rq 'timing::timed' "$subject" --include='*.rs' ||
        fail "converted subject does not measure through timing::timed: $subject"
    if grep -rnE 'Instant::now|Sha256::new|sha2::' "$subject" --include='*.rs'; then
        fail "converted subject owns a clock or a digest of its own: $subject"
    fi
done

# `allow(unsafe_code)` is denied workspace-wide; these five files are the approved exceptions under
# `tools/`, and `scripts/check-realtime-policy.sh` holds the matching list for `crates/` and
# `hosts/`. A sixth file is a new unsafe ownership boundary and needs a decision, not a grep.
expected_unsafe="$(printf '%s\n' \
    tools/miso-engine-bench-support/src/alloc.rs \
    tools/miso-engine-capi-audit/src/main.rs \
    tools/miso-engine-native-pcm-runner/src/lib.rs \
    tools/miso-engine-protocol-bench/src/main.rs \
    tools/miso-engine-wasm-gate-guest/src/lib.rs | LC_ALL=C sort)"
actual_unsafe="$(grep -rlE '^#!\[allow\(unsafe_code\)\]' tools --include='*.rs' 2>/dev/null | LC_ALL=C sort || true)"
[[ "$actual_unsafe" == "$expected_unsafe" ]] || {
    diff -u <(printf '%s\n' "$expected_unsafe") <(printf '%s\n' "$actual_unsafe") >&2 || true
    fail 'the approved unsafe ownership set under tools/ changed'
}

# The shared harness is test scaffolding. A production package depending on it would put a
# `#[global_allocator]` and an abort-on-allocation policy into a shipped artifact.
if grep -rn 'miso-engine-bench-support' crates hosts --include='Cargo.toml' 2>/dev/null; then
    fail 'a production package depends on the bench support crate'
fi

printf 'bench policy: ok (1 allocator, 1 escaper, 1 percentile, 1 digest sink, %s unsafe owners, %s subjects on the shared timer)\n' \
    "$(printf '%s\n' "$expected_unsafe" | wc -l | tr -d ' ')" "${#timed_subjects[@]}"
