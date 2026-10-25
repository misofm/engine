# Aggregate validator for the swap benchmark (#1286, `bench swap`): the two measured rounds'
# records, slurped (`jq -s -e -f scripts/swap-benchmark-validator.jq raw.jsonl`). Frozen with the
# subject before any timing (AGENTS.md); `tools/bench/src/swap.rs`'s tests run it on a record the
# subject printed and on each refusal below.
#
# It refuses: a missing distribution or D3 value; a swap, adoption, cycle or growth count other
# than the frozen one; a swap that did not carry, or an adoption whose prime is empty; an adoption
# later than its `not_before` block, or a chain whose ΣP is not p_max_samples(96 kHz, 128); a
# silent timed block, or an input other than the tone; and a D3 row the record's own numbers do
# not give. The carry refusal count is read, never asserted zero (root ruling 2026-10-12): a
# refusal is a finding, not a malformed record.

def frozen_interval: 32;
def frozen_swaps: 32;
def frozen_cycles: 32;
def rate: 96000;
def quantum: 128;
def p_max: 4096;
def lead: 1024;
def ring: 14080;
def nat: type == "number" and . >= 0 and . == floor;

# One timed distribution of `count` observations: every rank present, ordered, and positive (a
# timed render call takes time; an untimed record has zeros).
def distribution($count):
  type == "object" and (keys == ["count","max","min","p50","p90","p99"]) and
  .count == $count and all(.min, .p50, .p90, .p99, .max; nat) and .min > 0 and
  .min <= .p50 and .p50 <= .p90 and .p90 <= .p99 and .p99 <= .max;

def growth_valid($step):
  .step == $step and .lead_samples == lead and .sum_lead_samples == $step * lead and
  (.prime_bytes | nat) and .prime_bytes > 0 and
  .adoptions == frozen_cycles and .carried == .adoptions and .late_adoptions == 0 and
  (.adoption_ns | distribution(frozen_cycles));

# D3 item 3 over the record's own numbers, evaluated in the subject's order of operations.
def d3_valid($record):
  ($record.move.ordinary_ns.p99) as $o |
  ($record.prime.growths[3].adoption_ns.p99) as $a |
  ($record.prime.growths[3].prime_bytes) as $b |
  (128 * 1e9 / 96000) as $quantum_ns |
  (keys == ["a_p_ns","b_browser_bytes","b_native_bytes","b_p_bytes","beta_ps_per_byte",
            "browser_headroom","browser_headroom_engine","browser_sample_rate_hz","margin",
            "o_ns","prime_bytes_max","prime_bytes_max_refusal","rates"]) and
  .o_ns == $o and .a_p_ns == $a and .b_p_bytes == $b and
  .margin == 0.7 and .browser_headroom == 13.7 and .browser_headroom_engine == "webkit" and
  .browser_sample_rate_hz == 48000 and
  (if $a > $o then
     (((0.7 * $quantum_ns - $o) * $b / ($a - $o)) | floor) as $native |
     (((0.7 * 13.7 - 1) * $o * $b / ($a - $o)) | floor) as $browser |
     (.beta_ps_per_byte | type == "number") and
     .b_native_bytes == $native and .b_browser_bytes == $browser and
     (if $native >= $b and $browser >= $b then
        .prime_bytes_max == ([$native, $browser] | min) and .prime_bytes_max_refusal == null
      else
        .prime_bytes_max == null and (.prime_bytes_max_refusal | type == "string")
      end)
   else
     .beta_ps_per_byte == null and .b_native_bytes == null and .b_browser_bytes == null and
     .prime_bytes_max == null and .prime_bytes_max_refusal == "adoption_not_slower_than_ordinary"
   end) and
  # D3 items 1-2 at every launch rate, from the formulas.
  ([.rates[].sample_rate_hz] == [44100, 48000, 88200, 96000]) and
  all(.rates[];
    .l_max_samples == ((.sample_rate_hz / 100 | floor) + 6) and
    .p_max_samples == 4 * ((.l_max_samples / quantum | ceil) * quantum) and
    .stall_ring_frames == ((((.sample_rate_hz / 10 | floor) / quantum | ceil) + 2) * quantum) and
    .default_ring_frames == .stall_ring_frames + .p_max_samples + quantum and
    .ring_pcm_bytes_before == .stall_ring_frames * 8 and
    .ring_pcm_bytes_after == .default_ring_frames * 8);

def record_valid:
  . as $record |
  type == "object" and .schema_version == 1 and .issue == 1286 and .record == "swap_carry" and
  .benchmark == "swap" and .workload_kind == "sixty_four_track_app_swap" and
  .fixture == "fixtures/session/v1/console-sixty-four-track-app.json" and
  .input == "tone" and .sample_rate_hz == rate and .quantum_frames == quantum and .tracks == 64 and
  .source_ring_frames == ring and .l_max_samples == 966 and .soft_clippers_per_growth == 1 and
  .p_max_samples == p_max and .interval_blocks == frozen_interval and
  (.move | type == "object") and
  .move.swaps == frozen_swaps and .move.carried == .move.swaps and
  (.move.restarted_strips | nat) and (.move.carry_refusal_count | nat) and
  (.move.ordinary_ns | distribution(frozen_swaps * (frozen_interval - 1))) and
  (.move.move_ns | distribution(frozen_swaps)) and
  (.prime | type == "object") and
  .prime.cycles == frozen_cycles and .prime.sum_lead_samples == p_max and
  (.prime.carry_refusal_count | nat) and
  (.prime.growths | type == "array" and length == 4) and
  ([range(4) as $index | .prime.growths[$index] | growth_valid($index + 1)] | all) and
  .timed_blocks == frozen_swaps * frozen_interval + 4 * frozen_cycles and
  .silent_timed_blocks == 0 and (.min_timed_output_peak | type == "number") and
  .min_timed_output_peak > 0 and
  (.d3 | type == "object" and d3_valid($record)) and
  .profile == "release" and
  (.measurement_control == "controlled" or .measurement_control == "uncontrolled") and
  .missing_metadata == [] and
  all(.cpu_model, .rust_version, .target_triple, .candidate_commit; type == "string");

(type == "array") and length == 2 and
([.[].round] == [1, 2]) and
all(.[]; record_valid) and
# One host, one build, one admissibility state across both rounds.
([.[] | [.backend, .cpu_model, .candidate_commit, .target_triple, .measurement_control]]
  | unique | length) == 1
