# Definitions for the browser arm of `console_mixing_automation` (#1003, #1011): the two measured
# rounds `scripts/run-web-mixing-automation-benchmark.sh run` writes, slurped (`jq -s`).
# `web-mixing-automation-validator.jq` is the verdict; the runner prints
# `web_mixing_refusal_reasons` when the verdict is a refusal (#1011 follow-up), so a refused run
# says which claim failed rather than only that one did.
#
# The browser record is descriptive, like every record in the console stream, but it states claims
# a later edit could quietly drop, so they are refused here rather than trusted:
#
# * the row: the mono fixture, the eight controls in the host's lowerings (the native row's pin),
#   the traffic every arm was admitted, and the digest rules the harness asserted in-run --
#   `quiet == restated`, `restated != automated`, and, in the preflight, every effect moving bits
#   on its own and the EQ-only restatement rendering `quiet`'s bits;
# * the input: this arm streams one continuous tone, in phase on every track, where the native row
#   binds each track a frozen block phase-offset by track. The two feeds share the tone's rate and
#   amplitude, and differ in exactly those two facts;
# * the protocol: two measured rounds after a warmup launch, which agree on every digest, on the
#   module, and on the commit the module and the control table were prepared at, which must be the
#   commit the rounds were run at.
include "console-benchmark-record-lib";

def web_mixing_keys: ["arms","automated_output_sha256","automated_p50_ns","automated_p95_ns","automated_p99_ns","bank_collapse_counters_exported","bit_identity","candidate_commit","command_records_per_block","console_command_queue_records","controls","cpu_affinity","descriptive_only","fixture_id","input_feed","input_signal","issue","loadavg_end","loadavg_start","measurement_control","module_matches_pin","module_sha256","native_input_feed","node_flags","node_version","observations","paired_collapse_delta_median_ns","paired_ramp_delta_median_ns","pairing","percentile_method","pinned_sha256","preflight_output_sha256","prepared_commit","preroll_blocks","quantum_frames","quiet_output_sha256","quiet_p50_ns","quiet_p95_ns","quiet_p99_ns","record","records_admitted","restated_output_sha256","restated_p50_ns","restated_p95_ns","restated_p99_ns","round","sample_rate_hz","schema_version","source_frames","source_ring_frames","statistical_method","tracks","units","v8_version","workload_kind"];
def web_feed_keys: ["amplitude","block_frames","continuous_across_blocks","delivery","radians_per_frame","track_phase_radians","waveform"];
def commit_id: type == "string" and test("^[0-9a-f]{40}$");

# One named claim of a record. A claim that cannot be evaluated on this record (a field of the wrong
# type) is a failed claim, not an error, so every claim is judged and every failure can be named.
def claim($name; f): {name: $name, ok: ((try f catch false) == true)};

# Every claim a measured round makes. The record is valid when all of them hold; the names are what
# a refusal prints.
def web_mixing_record_claims: [
  claim("key set"; (keys | sort) == web_mixing_keys),
  claim("identity"; .schema_version == 1 and .issue == 1003 and .record == "web_mixing_automation" and
    .workload_kind == "sixty_four_track_console_mono_mixing_automation" and
    .fixture_id == mono_console_fixture and .tracks == 64 and
    .sample_rate_hz == 48000 and .quantum_frames == 128 and
    (.source_frames | type == "string" and test("^[1-9][0-9]*$"))),
  # The input, and how it differs from the native row's.
  claim("input feeds"; .input_signal == "tone" and
    ([.input_feed, .native_input_feed] | all(type == "object" and (keys | sort) == web_feed_keys)) and
    .input_feed.waveform == "sine" and .native_input_feed.waveform == "sine" and
    .input_feed.radians_per_frame == .native_input_feed.radians_per_frame and
    .input_feed.amplitude == .native_input_feed.amplitude and
    ([.input_feed.radians_per_frame, .input_feed.amplitude] | all(type == "number" and . > 0)) and
    .input_feed.block_frames == .quantum_frames and .native_input_feed.block_frames == .quantum_frames and
    .input_feed.delivery == "streamed_source" and .input_feed.continuous_across_blocks == true and
    .input_feed.track_phase_radians == 0 and
    .native_input_feed.delivery == "frozen_block_per_track" and
    .native_input_feed.continuous_across_blocks == false and
    (.native_input_feed.track_phase_radians | type == "number" and . > 0)),
  # The module: its digest, and whether it is the committed pin.
  claim("module"; ([.module_sha256, .pinned_sha256] | all(sha256)) and
    .module_matches_pin == (.module_sha256 == .pinned_sha256)),
  claim("host"; (.node_version | type == "string" and startswith("v")) and
    (.v8_version | type == "string" and length > 0) and
    (.node_flags | type == "array" and any(.[]; . == "--no-liftoff")) and
    .console_command_queue_records == 64 and
    (.source_ring_frames | positive_integer) and (.source_ring_frames % .quantum_frames) == 0),
  # The row.
  claim("row"; .observations == 1000 and .preroll_blocks == 64 and
    .pairing == "alternating_per_observation" and .arms == ["quiet","restated","automated"] and
    (.controls | type == "array" and
      map([.track_id,.slot_id,.effect,.parameter,.parameter_index,.lowering,.step])
        == mixing_automation_controls and
      all(.[]; ([.base,.even_value,.odd_value] | all(type == "number")) and
               .even_value > .base and .base > .odd_value and (.parameter_id | positive_integer))) and
    .command_records_per_block == (.controls | length) and
    .records_admitted == {quiet: 0, restated: (.observations * .command_records_per_block),
                          automated: (.observations * .command_records_per_block)}),
  claim("percentiles"; .units == "ns_per_block" and .percentile_method == "nearest_rank" and
    ([.quiet_p50_ns,.quiet_p95_ns,.quiet_p99_ns,.restated_p50_ns,.restated_p95_ns,.restated_p99_ns,.automated_p50_ns,.automated_p95_ns,.automated_p99_ns] | all(positive_integer)) and
    ordered_percentiles([.quiet_p50_ns,.quiet_p95_ns,.quiet_p99_ns]) and
    ordered_percentiles([.restated_p50_ns,.restated_p95_ns,.restated_p99_ns]) and
    ordered_percentiles([.automated_p50_ns,.automated_p95_ns,.automated_p99_ns]) and
    ([.paired_ramp_delta_median_ns,.paired_collapse_delta_median_ns] | all(type == "number" and floor == .))),
  claim("digests"; ([.quiet_output_sha256,.restated_output_sha256,.automated_output_sha256] | all(sha256)) and
    .quiet_output_sha256 == .restated_output_sha256 and
    .restated_output_sha256 != .automated_output_sha256 and
    .bit_identity == "quiet == restated, asserted in-run"),
  claim("preflight digests"; .preflight_output_sha256 | type == "object" and
    (keys_unsorted == mixing_automation_preflight_arms) and all(.[]; sha256) and
    .quiet == .restated and .restated_eq_only == .restated and
    .automated != .restated and .automated_eq_only != .restated and
    .automated_compressor_only != .restated and .automated_limiter_only != .restated),
  claim("statements"; .bank_collapse_counters_exported == false and
    ([.loadavg_start, .loadavg_end, .statistical_method] | all(type == "string" and length > 0)) and
    .descriptive_only == true),
  # Provenance: the module and the control table were prepared at the commit the round ran at.
  claim("provenance"; (.candidate_commit | commit_id) and .prepared_commit == .candidate_commit),
  claim("admissibility"; (.measurement_control | type == "string" and
      (startswith("controlled;") or
       (startswith("uncontrolled;") and test("MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1")))) and
    (.cpu_affinity | type == "string" and test("^[0-9]+$")))
];

def web_mixing_record_valid: all(web_mixing_record_claims[]; .ok);

# The fields two measured rounds of one prepared module must agree on. A field a record claim
# already ties to another is compared through that one: `quiet` digests equal `restated`'s in every
# record, and the streamed feed's rate and level are the native feed's.
def web_mixing_agreement_fields: ["module_sha256","candidate_commit","controls","native_input_feed","node_version","v8_version","node_flags","source_frames","source_ring_frames","measurement_control","cpu_affinity","restated_output_sha256","automated_output_sha256","preflight_output_sha256"];

def web_mixing_rounds_agree_on($field): (map(try .[$field] catch null) | unique | length) == 1;

# Two measured rounds of one prepared module, which render the same bits. The rounds are exactly
# 1 and 2, which also refuses a warmup record, a single round and a third one.
def web_mixing_rounds_valid:
  all(.[]; web_mixing_record_valid) and
  (map(.round) | sort) == [1,2] and
  . as $rounds |
  all(web_mixing_agreement_fields[]; . as $field | $rounds | web_mixing_rounds_agree_on($field));

# Why a slurped pair of rounds is refused: one line per failed claim, round count or disagreeing
# field, and nothing for a valid pair.
def web_mixing_refusal_reasons:
  . as $rounds |
  if type != "array" then "the rounds are not a list of records" else
    ( $rounds[] | . as $record | (try .round catch null) as $round |
      ( web_mixing_record_claims[] | select(.ok | not) | "round \($round | tojson): \(.name) refused" ),
      ( try (((keys - web_mixing_keys) | select(length > 0)
              | "round \($round | tojson): unexpected keys \(tojson)"),
             ((web_mixing_keys - keys) | select(length > 0)
              | "round \($round | tojson): missing keys \(tojson)"))
        catch "round \($round | tojson): not a record" ) ),
    ( [$rounds[] | try .round catch null] | sort | select(. != [1,2])
      | "the rounds are \(tojson), not exactly [1,2]" ),
    ( web_mixing_agreement_fields[] as $field
      | select(($rounds | web_mixing_rounds_agree_on($field)) | not)
      | "the rounds disagree on \($field)" )
  end;
