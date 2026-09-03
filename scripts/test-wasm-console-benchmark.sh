#!/usr/bin/env bash
# Mutation coverage for the issue #163 phase 2 step 1 wasm console arm's validator.
#
# Hermetic: no wasmtime, no guest module, no timing, no measurement. It builds one frozen
# thirty-two-record set with `jq -cn`, asserts the validator accepts it, then destroys one claim at
# a time and asserts the validator rejects each. Every case carries a prose label naming the claim
# it breaks, because a mutation suite whose cases are unlabelled is a list of jq expressions rather
# than a statement of what the validator is for.
#
# There are two frozen shapes, because there are two record shapes. The three-leg set is every arm
# up to and including #184; the paired four-leg set is issue #183's W4/W8 arm, derived from the
# first so the two cannot drift apart here, and it carries its own block of cases at the bottom.
#
# The no-pipe shape is deliberate and is the same one `test-wasm-kernel-timing.sh` uses: the
# helpers take JSON as an *argument*, never on stdin, because a helper on the right of a pipe runs
# in a subshell and would discard the failure counter.
set -euo pipefail
[[ "$#" -le 1 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
export LC_ALL=C
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"

valid() { printf '%s\n' "$1" | jq -s -e -f "$root/scripts/wasm-console-benchmark-validator.jq" >/dev/null 2>&1; }
failures=0
expect_accept() { if ! valid "$1"; then printf 'expected accept: %s\n' "$2" >&2; failures=$((failures + 1)); fi; }
expect_reject() { if valid "$1"; then printf 'expected reject: %s\n' "$2" >&2; failures=$((failures + 1)); fi; }

# One frozen record set. The numbers are plausible rather than measured -- this suite tests the
# validator, not the engine -- but every internal consistency the validator checks holds in it, so
# a rejection below is caused by the mutation and by nothing else.
records=$(jq -cn '
  def module_digest: ("1" * 64);
  def out_digest($kind): (($kind | explode | add | tostring) + ("0" * 60))[0:64];
  def leg($name; $target; $backend; $scope; $p50; $kind):
    {
      leg: $name, target: $target, backend: $backend, audit_scope: $scope,
      min_ns_per_block: ($p50 - 100),
      p50_ns_per_block: $p50,
      p95_ns_per_block: ($p50 + 100),
      p99_ns_per_block: ($p50 + 200),
      max_ns_per_block: ($p50 + 900),
      p50_us_per_block: (($p50 / 1000 * 1000 | round) / 1000),
      p50_us_per_block_per_track: 1.0,
      output_sha256: out_digest($kind),
      render_errors: 0
    };
  # [kind, tracks, synthetic, strip_content, strip_layout, input_signal, fixture_id]
  def pins:
    [
      ["nine_track_baseline", 9, false, "eq", "simd1:eq", "tone", "fixtures/session/v1/parametric-eq-nine-track.json"],
      ["nine_track_ragged_strip", 9, true, "eq+compressor+limiter", "simd1:eq+compressor,simd2:limiter", "tone", "fixtures/session/v1/console-sixty-four-track-intended.json"],
      ["sixty_four_track_console", 64, false, "eq+compressor+limiter", "simd1:eq+compressor,simd2:limiter", "tone", "fixtures/session/v1/console-sixty-four-track-intended.json"],
      ["one_twenty_eight_track_stretch", 128, true, "eq+compressor+limiter", "simd1:eq+compressor,simd2:limiter", "tone", "fixtures/session/v1/console-sixty-four-track-intended.json"],
      ["sixty_four_track_eq_only", 64, true, "eq", "simd1:eq", "tone", "fixtures/session/v1/console-sixty-four-track-intended.json"],
      ["sixty_four_track_compressor_only", 64, true, "compressor", "simd1:compressor", "tone", "fixtures/session/v1/console-sixty-four-track-intended.json"],
      ["sixty_four_track_builtins_only", 64, true, "builtins", "builtins", "tone", "fixtures/session/v1/console-sixty-four-track-intended.json"],
      ["sixty_four_track_dispatch_only", 64, true, "identity", "builtins", "tone", "fixtures/session/v1/console-sixty-four-track-intended.json"],
      ["sixty_four_track_idle", 64, true, "eq+compressor+limiter", "simd1:eq+compressor,simd2:limiter", "silence", "fixtures/session/v1/console-sixty-four-track-intended.json"],
      ["sixty_four_track_console_legacy", 64, false, "eq+compressor", "simd1:eq,dynamic:compressor", "tone", "fixtures/session/v1/console-sixty-four-track.json"],
      ["sixty_four_track_eq_comp_simd1", 64, true, "eq+compressor", "simd1:eq+compressor", "tone", "fixtures/session/v1/console-sixty-four-track-intended.json"],
      ["sixty_four_track_plumbing_only", 64, true, "plumbing", "plumbing", "tone", "fixtures/session/v1/console-sixty-four-track-intended.json"],
      ["sixty_four_track_gain_pan_only", 64, true, "gain+pan", "builtins", "tone", "fixtures/session/v1/console-sixty-four-track-intended.json"],
      ["sixty_four_track_console_mono", 64, false, "eq+compressor+limiter", "simd1:eq+compressor,simd2:limiter", "tone", "fixtures/session/v1/console-sixty-four-track-mono.json"],
      ["sixty_four_track_console_mono_dual", 64, false, "eq+compressor+limiter", "simd1:eq+compressor,simd2:limiter", "tone", "fixtures/session/v1/console-sixty-four-track-mono.json"],
      ["sixty_four_track_console_half_mono", 64, true, "eq+compressor+limiter", "simd1:eq+compressor,simd2:limiter", "tone", "fixtures/session/v1/console-sixty-four-track-mono.json"]
    ];
  [ (1, 2) as $round | pins[] | . as $pin |
    ([ leg("native_simd8"; "native"; "Simd8"; "host_process_heap"; 89000; $pin[0]),
       leg("native_simd4"; "native"; "Simd4"; "host_process_heap"; 303000; $pin[0]),
       leg("wasm_simd128"; "wasm32-unknown-unknown"; "Simd4"; "not_observable_guest_linear_memory"; 952000; $pin[0])
     ]) as $legs |
    {
      schema_version: 1, issue: 163, phase: "2-step1", record: "wasm_console_session",
      workload_kind: $pin[0], tracks: $pin[1], synthetic_fixture: $pin[2],
      strip_content: $pin[3], strip_layout: $pin[4],
      input_signal: $pin[5], fixture_id: $pin[6],
      round: $round, sample_rate_hz: 48000, quantum_frames: 128,
      observations: 1000, warmup_blocks: 64,
      units: "us_per_block", percentile_method: "nearest_rank",
      runtime: "wasmtime 47.0.3", guest_target: "wasm32-unknown-unknown",
      guest_target_features: "+simd128", guest_module_sha256: module_digest,
      guest_call_overhead_p50_ns: 20,
      legs: $legs,
      ratios: [
        { numerator: "wasm_simd128", denominator: "native_simd8",
          ratio_of_p50: ((952000 / 89000 * 1000 | round) / 1000), paired_ratio_median: 10.7 },
        { numerator: "wasm_simd128", denominator: "native_simd4",
          ratio_of_p50: ((952000 / 303000 * 1000 | round) / 1000), paired_ratio_median: 3.14 }
      ],
      digest_identity: "all_legs_identical",
      render_total_forbidden_operations: 0,
      comparable_with_console_records: false,
      browser_field_measurement: false,
      cpu_model: "a cpu", os: "linux", governor_or_power_mode: "powersave",
      rust_version: "rustc 1.97.1", llvm_version: "22.1.6",
      target_triple: "x86_64-unknown-linux-gnu", target_features: "runtime-avx2,fma;baseline",
      profile: "release", background_load_note: "controlled", measurement_control: "controlled",
      cpu_affinity: "15", candidate_commit: ("0" * 40), missing_metadata: [],
      descriptive_only: true, statistical_method: "descriptive only; no threshold"
    } ]')

set_all() { printf '%s' "$records" | jq -c "[.[] | $1] | .[]"; }
mutate() { expect_reject "$(printf '%s' "$records" | jq -c "$1 | .[]")" "$2"; }

expect_accept "$(printf '%s' "$records" | jq -c '.[]')" 'the frozen thirty-two-record set'

# Shape of the set.
mutate 'del(.[0])' 'thirty-one records'
mutate '. as $r | ($r + [$r[0]])' 'a duplicated workload row'
mutate '.[0].round = 2' 'a workload measured twice in one round and never in the other'
mutate '[.[] | .round = 1]' 'a set with only one measured round'

# Claim 1: the family boundary. Both halves, separately.
expect_reject "$(set_all '.comparable_with_console_records = true')" \
    'a record claiming a wasm-simd128 number is comparable with a native console record'
expect_reject "$(set_all '.browser_field_measurement = true')" \
    'a determinism-pinned wasmtime number claiming to be a browser field measurement'
expect_reject "$(set_all 'del(.comparable_with_console_records)')" \
    'a record that dropped its family boundary entirely'

# Claim 2: one subject, three targets.
mutate '.[0].legs[2].output_sha256 = ("f" * 64)' \
    'a wasm leg that rendered different bits than the native legs it is compared against'
mutate '.[0] |= (.legs[2].output_sha256 = ("f" * 64) | .digest_identity = "all_legs_identical")' \
    'a summary claiming identity over legs whose digests differ'
mutate '.[0].digest_identity = "divergent"' \
    'a summary claiming divergence over legs whose digests agree'
mutate '.[0].digest_identity = "probably_fine"' \
    'a digest identity outside the two words the field may carry'
mutate '.[16].legs[0].output_sha256 = ("e" * 64)' \
    'a second round that rendered something other than what the first round rendered'

# Claim 3: the published ratios are the legs own.
mutate '.[0].ratios[0].ratio_of_p50 = 2.0' \
    'a ratio that does not follow from the two legs it names'
mutate '.[0].legs[2].p50_ns_per_block = 500000' \
    'a leg p50 moved out from under the ratio computed from it'
mutate '.[0].ratios[1].denominator = "native_simd8"' \
    'a ratio table that compares against one leg twice and never against the other'
mutate '.[0].ratios[0].numerator = "native_simd4"' \
    'a ratio whose numerator is not the wasm leg'
mutate '.[0].ratios = [.[0].ratios[0]]' 'a ratio table missing the same-width comparison'

# Claim 4: every row is the row it claims.
mutate '.[0].tracks = 64' 'a nine-track row claiming sixty-four tracks'
mutate '.[4].strip_content = "eq+compressor"' \
    'an eq-only decomposition row claiming it still carries the compressor'
mutate '.[8].input_signal = "tone"' 'the idle row claiming it renders a tone'
mutate '.[2].synthetic_fixture = true' \
    'the checked-in console fixture reported as derived in code'
mutate '.[5].workload_kind = "sixty_four_track_mystery"' 'a workload kind with no pinned shape'
# #175: the two rows that carry the same `strip_content` and differ only in rack layout. Without
# `strip_layout` in the pin these two are the same row, and the chain-shape comparison the wasm
# arm reports would be a comparison of one layout with itself.
mutate '.[9].strip_layout = "simd1:eq+compressor"' \
    'the transition row claiming the merged chain shape'
mutate '.[10].strip_layout = "simd1:eq,dynamic:compressor"' \
    'the chain-shape row claiming the retired layout'
mutate '.[9].fixture_id = "fixtures/session/v1/console-sixty-four-track-intended.json"' \
    'the transition row rendered from the standing fixture'
mutate '.[2].strip_layout = "simd1:eq,dynamic:compressor"' \
    'the standing console row claiming the retired layout'
mutate '.[2].strip_content = "eq+compressor"' \
    'the standing console row claiming it carries no limiter'
# The rows this arm gained with the strip round's job 4. The overhead pair separates on
# `strip_layout` alone -- `plumbing` against `builtins` -- which is the same trap #175's row-pair
# set, seen from the other end: two rows whose whole difference is what is *not* prepared.
mutate '.[11].strip_layout = "builtins"' \
    'the overhead floor row claiming the builtins layout it is the floor of'
mutate '.[12].strip_content = "identity"' \
    'the gain-and-pan row claiming the identity fader and pan'
mutate '.[13].fixture_id = "fixtures/session/v1/console-sixty-four-track-intended.json"' \
    'a mono row rendered from the standing stereo fixture'
mutate '.[13].synthetic_fixture = true' \
    'a mono row reported as derived in code'
mutate '.[15].synthetic_fixture = false' \
    'the mixed-cohort row reported as a checked-in fixture'
mutate '.[2].fixture_id = "fixtures/session/v1/console-sixty-four-track-mono.json"' \
    'the standing console row rendered from the mono fixture'

# Leg labelling and leg internals.
mutate '.[0].legs[2].target = "native"' 'a wasm leg labelled as a native one'
mutate '.[0].legs[0].backend = "Simd4"' 'the production native leg labelled at the wrong width'
mutate '.[0].legs[2].audit_scope = "host_process_heap"' \
    'a wasm leg claiming this process audit can see inside the guest'
mutate '.[0].legs = [.[0].legs[0], .[0].legs[1]]' 'a record that dropped its wasm leg'
mutate '.[0].legs[0].p95_ns_per_block = 1' 'percentiles that do not increase'
mutate '.[0].legs[0].p50_us_per_block = 1.0' \
    'a microsecond field that disagrees with the nanosecond field beside it'
mutate '.[0].legs[0].render_errors = -1' 'a negative render-error count'

# The measurement frame.
expect_reject "$(set_all '.quantum_frames = 256')" 'a block size that is not the engine quantum'
expect_reject "$(set_all '.sample_rate_hz = 44100')" 'a sample rate the fixture is not prepared at'
expect_reject "$(set_all '.guest_target_features = ""')" \
    'a guest built without simd128 reported as the shipped artifact'
expect_reject "$(set_all '.runtime = "some other runtime"')" 'a runtime that is not wasmtime'
expect_reject "$(set_all '.measurement_control = null')" \
    'a record that does not say whether its measurement was controlled'
expect_reject "$(set_all '.descriptive_only = false')" 'a descriptive record claiming a threshold'
mutate '.[0].guest_module_sha256 = ("a" * 64)' \
    'two different guest modules timed and reported as one measurement'

# Shape.
expect_reject "$(set_all '.unexpected_key = 1')" 'an extra key'
expect_reject "$(set_all 'del(.guest_call_overhead_p50_ns)')" \
    'a record that dropped the host-to-guest crossing cost it does not subtract'

# ------------------------------------------------------------------------------------------
# The issue #183 paired arm: the same twenty-two rows with a fourth leg, the eight-lane wasm
# guest, and the width ratio the switch decision is read off. The set is derived from the frozen
# one above rather than written out again, so the two shapes cannot drift apart in this file.
# ------------------------------------------------------------------------------------------
paired=$(printf '%s' "$records" | jq -c '
  def w8_p50: 1120000;
  [ .[] | . as $record |
    ($record.legs[2]) as $w4 |
    $record
    + { guest_simd8_module_sha256: ("2" * 64) }
    | .legs += [ $w4 + {
          leg: "wasm_simd128_w8", backend: "Simd8",
          min_ns_per_block: (w8_p50 - 100),
          p50_ns_per_block: w8_p50,
          p95_ns_per_block: (w8_p50 + 100),
          p99_ns_per_block: (w8_p50 + 200),
          max_ns_per_block: (w8_p50 + 900),
          p50_us_per_block: ((w8_p50 / 1000 * 1000 | round) / 1000)
        } ]
    | .ratios += [ { numerator: "wasm_simd128_w8", denominator: "wasm_simd128",
                     ratio_of_p50: ((w8_p50 / $w4.p50_ns_per_block * 1000 | round) / 1000),
                     paired_ratio_median: 1.18 } ] ]')

paired_set_all() { printf '%s' "$paired" | jq -c "[.[] | $1] | .[]"; }
paired_mutate() { expect_reject "$(printf '%s' "$paired" | jq -c "$1 | .[]")" "$2"; }

expect_accept "$(printf '%s' "$paired" | jq -c '.[]')" \
    'the frozen thirty-two-record paired W4/W8 set'

# The pairing itself: a record set is paired in every row or in none.
paired_mutate '.[0:16] = ([.[0:16][] | del(.guest_simd8_module_sha256)
                          | .legs = .legs[0:3] | .ratios = .ratios[0:2]])' \
    'half a set carrying the eight-lane leg and half not'
paired_mutate '[.[] | del(.guest_simd8_module_sha256)]' \
    'a fourth leg timed and reported with no second guest module behind it'
paired_mutate '[.[] | .guest_simd8_module_sha256 = .guest_module_sha256]' \
    'one guest module named twice and reported as a paired width measurement'
paired_mutate '.[0].guest_simd8_module_sha256 = ("3" * 64)' \
    'two different eight-lane guests timed and reported as one measurement'
paired_mutate '[.[] | .legs = .legs[0:3]]' \
    'a paired record that dropped the eight-lane leg it says it carries'

# The eight-lane leg is the eight-lane leg.
paired_mutate '.[0].legs[3].backend = "Simd4"' \
    'the eight-lane wasm leg labelled at the width it is being compared against'
paired_mutate '.[0].legs[3].target = "native"' 'the eight-lane wasm leg labelled as a native one'
paired_mutate '.[0].legs[3].leg = "wasm_simd128"' \
    'a record whose two wasm legs carry the same name'
paired_mutate '.[0].legs[3].output_sha256 = ("f" * 64)' \
    'an eight-lane guest that rendered different bits than the four-lane guest beside it'

# The width ratio: the number the switch decision is read off.
paired_mutate '.[0].ratios[2].ratio_of_p50 = 1.0' \
    'a width ratio that does not follow from the two wasm legs it names'
paired_mutate '.[0].legs[3].p50_ns_per_block = 2000000' \
    'an eight-lane p50 moved out from under the width ratio computed from it'
paired_mutate '.[0].ratios[2].denominator = "native_simd8"' \
    'a width ratio taken against a native leg, which is a target comparison and not a width one'
paired_mutate '.[0].ratios[2].numerator = "wasm_simd128"' \
    'a width ratio of the four-lane leg against itself'
paired_mutate '[.[] | .ratios = .ratios[0:2]]' \
    'a paired record that publishes both wasm legs and no ratio between them'

expect_reject "$(paired_set_all '.unexpected_key = 1')" 'an extra key on a paired record'

if [[ "$failures" != 0 ]]; then
    printf 'wasm console benchmark validator suite: %s FAILED case(s)\n' "$failures" >&2
    exit 1
fi
printf 'wasm console benchmark validators: PASS (real runtime/timing invocations: 0/0)\n'
