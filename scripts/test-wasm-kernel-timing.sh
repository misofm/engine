#!/usr/bin/env bash
# Mutation suite for the issue #163 phase 0b timing validator. Hermetic: no runtime, no timing.
#
# The three claims this family makes that a reader would otherwise have to take on trust are the
# three this suite has to be able to lose: that a wasm number is never comparable with a native
# console number, that every leg computed the *pinned* corpus rather than something else quickly,
# and that the published delta is a difference from a baseline arm whose own delta is zero.
set -euo pipefail
[[ "$#" -le 1 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
export LC_ALL=C

valid() { printf '%s\n' "$1" | jq -s -e -f "$root/scripts/wasm-kernel-timing-validator.jq" >/dev/null 2>&1; }
failures=0
expect_accept() { if ! valid "$1"; then printf 'expected accept: %s\n' "$2" >&2; failures=$((failures + 1)); fi; }
expect_reject() { if valid "$1"; then printf 'expected reject: %s\n' "$2" >&2; failures=$((failures + 1)); fi; }

digest_of() { printf '%s%s' "$(printf '0%.0s' {1..63})" "$1"; }

# One frozen eight-record set: four legs, two rounds. Case digests are shared across every leg,
# which is the property the aggregate asserts.
records=$(jq -cn \
  --arg base "$(digest_of 1)" --arg svf "$(digest_of 2)" \
  --arg pole "$(digest_of 3)" --arg fma "$(digest_of 4)" '
  def arm($case; $index; $p50; $delta; $digest):
    {case: $case, case_index: $index, p50_ns: $p50, p95_ns: ($p50 + 1000),
     p99_ns: ($p50 + 2000), paired_delta_median_ns: $delta, digest: $digest};
  def legs: [
    {leg: "native", width: "simd4", backend: "native-simd4", base: 30999, svf: 8435, pole: 4930, fma: 24486},
    {leg: "native", width: "simd8", backend: "native-simd8", base: 31229, svf: 4128, pole: 2365, fma: 47320},
    {leg: "wasm", width: "simd4", backend: "wasm-simd128", base: 77988, svf: 64573, pole: 22543, fma: 45386},
    {leg: "wasm", width: "simd8", backend: "wasm-simd128", base: 79882, svf: 60104, pole: 14137, fma: 60906}
  ];
  [ (1, 2) as $round | legs[] | {
      schema_version: 1, issue: 163, phase: "0b", record: "wasm_kernel_timing",
      round: $round, leg: .leg, backend: .backend, width: .width,
      runtime: "wasmtime 47.0.3", comparable_with_console_records: false,
      observations: 500, pairing: "alternating_per_observation",
      percentile_method: "nearest_rank", units: "ns_per_case",
      common_term: "every arm hashes the same 32768 bytes, so the SHA-256 cancels in the paired delta",
      baseline: arm("gain_block/noise"; 28; .base; 0; $base),
      arms: [arm("svf_block_ramped/noise"; 16; (.base + .svf); .svf; $svf),
             arm("one_pole_block/noise"; 24; (.base + .pole); .pole; $pole),
             arm("lane_fma"; 48; (.base + .fma); .fma; $fma)],
      descriptive_only: true,
      statistical_method: "arms alternated per observation; nearest-rank percentiles over per-call nanoseconds; paired delta is the arm minus the baseline arm per observation; descriptive only; no threshold"
  } ]')

set_all() { printf '%s' "$records" | jq -c "[.[] | $1] | .[]"; }
mutate() { expect_reject "$(printf '%s' "$records" | jq -c "$1 | .[]")" "$2"; }

expect_accept "$(printf '%s' "$records" | jq -c '.[]')" 'the eight-record set'

mutate 'del(.[0])' 'seven records'
mutate '. as $r | ($r + [$r[0]])' 'a duplicated leg'
mutate '.[4].round = 1' 'a leg measured twice in one round'
mutate '[.[] | select(.leg != "wasm")]' 'a set with no wasm leg'
mutate '[.[] | select(.leg != "native")]' 'a set with no native leg'
mutate '[.[] | select(.width != "simd8")]' 'a set at one width only'

# Claim 1: the family boundary.
expect_reject "$(set_all '.comparable_with_console_records = true')" \
    'a record claiming a wasm number is comparable with a native console number'
mutate '.[2].backend = "native-simd4"' 'a wasm leg labelled as native'
mutate '.[0].backend = "wasm-simd128"' 'a native leg labelled as the shipped wasm artifact'
mutate '.[1].backend = "native-simd4"' 'a native leg whose label contradicts its width'

# Claim 2: every leg computed the pinned corpus. A leg that quietly computed something else would
# be fast for a reason that has nothing to do with the target.
mutate '.[2].arms[0].digest = "'"$(digest_of 9)"'"' \
    'a wasm leg whose svf kernel produced a different digest from the native leg'
mutate '.[3].baseline.digest = "'"$(digest_of 9)"'"' \
    'a leg whose baseline arm produced a different digest'
mutate '.[0].arms[2].digest = "short"' 'a malformed digest'

# Claim 3: the delta is a difference from the baseline.
mutate '.[0].baseline.paired_delta_median_ns = 17' \
    'a baseline arm whose own delta is not zero'
mutate '.[0].arms[1].paired_delta_median_ns = 0' \
    'a kernel indistinguishable from the baseline it is measured against'
mutate '.[0].arms[1].paired_delta_median_ns = -500' 'a kernel cheaper than the baseline'
mutate '.[0].arms[1].case = "gain_block/noise"' 'an arm that is the baseline again'

# Shape.
expect_reject "$(set_all '.round = 3')" 'a third round'
expect_reject "$(set_all '.issue = 149')" 'a record from another issue'
expect_reject "$(set_all '.descriptive_only = false')" 'a record claiming to be a gate'
expect_reject "$(set_all '.pairing = "sequential"')" 'legs that were not alternated'
expect_reject "$(set_all '.percentile_method = "linear"')" 'an interpolating percentile'
expect_reject "$(set_all '.units = "ns_per_block"')" 'a unit this arm does not measure in'
expect_reject "$(set_all '.observations = 0')" 'a run with no observations'
expect_reject "$(set_all '.runtime = "v8"')" 'a runtime that is not the pinned wasmtime'
expect_reject "$(set_all '.unexpected_key = 1')" 'an extra key'
expect_reject "$(set_all 'del(.common_term)')" 'a record that dropped its cancellation statement'
mutate '.[0].arms[0].p50_ns = 999999999' 'percentiles out of order'
mutate '.[0].arms = [.[0].arms[0]]' 'a set of arms the runner does not emit'
mutate '.[0].baseline.p50_ns = 0' 'a zero-cost baseline'

if [[ "$failures" != 0 ]]; then
    printf 'wasm kernel timing validator suite: %s FAILED case(s)\n' "$failures" >&2
    exit 1
fi
printf 'wasm kernel timing validators: PASS (real runtime/timing invocations: 0/0)\n'
