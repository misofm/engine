# Aggregate validator: seventeen session workloads (fifteen bound-feed rows, the driver-fed
# gain/pan row, #928 and #956, and the metered console row, #881), two hoist workloads, one meters
# arm, one observation arm, one placement row-pair, one automation-active row and one mono
# row-pair, each in rounds one and two -- forty-eight records. #956 retired the builtins-less
# plumbing row and re-based the driver-fed row, two records fewer than the fifty before it.
include "console-benchmark-record-lib";
. as $records |
(type == "array") and length == 48 and
all(.[]; console_benchmark_record_valid_lib) and
([.[] | select(.record == "console_session") | .workload_kind] | unique | sort) == session_kinds and
([.[] | select(.record == "console_hoist") | .workload_kind] | unique | sort)
  == ["nine_track_ragged_strip","sixty_four_track_console"] and
([.[] | select(.record == "console_session")] | length) == 34 and
([.[] | select(.record == "console_hoist")] | length) == 4 and
([.[] | select(.record == "console_meters")] | length) == 2 and
([.[] | select(.record == "console_observation")] | length) == 2 and
([.[] | select(.record == "console_placement")] | length) == 2 and
([.[] | select(.record == "console_automation")] | length) == 2 and
([.[] | select(.record == "console_mono")] | length) == 2 and
([.[] | select(.record == "console_mono") | .workload_kind] | unique)
  == ["sixty_four_track_mono_pair"] and
([.[] | select(.record == "console_automation") | .workload_kind] | unique)
  == ["sixty_four_track_compressor_automation"] and
([.[] | .round] | unique | sort) == [1,2] and
([.[] | [.record,.workload_kind,.round] | join(":")] | unique | length) == 48 and
(group_by([.record,.workload_kind]) | all(map(.round) | sort == [1,2])) and
# Round one and round two are two measurements of one frozen workload, so the rendered output must
# be identical across them. A drifting digest means the rounds are not measuring the same thing.
(group_by([.record,.workload_kind]) | all(map(.output_sha256 // .restated_output_sha256 // .meters_off_output_sha256 // .absent_output_sha256 // .split_chains_output_sha256 // .collapse_eligible_output_sha256) | unique | length == 1)) and
# #928, re-based by #956: the driver-fed gain/pan row is the bound-feed gain/pan row fed through a
# prepared source set. The two feeds deliver the same frozen words, so the two rows render the same
# bits in both rounds; a difference is a harness defect, never a finding, and the run is refused
# rather than published with two numbers for two different computations.
([.[] | select(.record == "console_session" and
               (.workload_kind == "sixty_four_track_gain_pan_only" or
                .workload_kind == "sixty_four_track_gain_pan_ring"))
      | .output_sha256] | length == 4 and (unique | length) == 1) and
# #881: the metered console row is the standing console session prepared as the default web boot
# prepares it -- a meter on every track and between-render-calls delivery, which fuses each
# cohort's fader and matrix into one stage. A meter observes and the fused stage renders the split
# pair's bits, so the two rows render the same bits in both rounds, and a difference is refused
# rather than published as the price of metering.
#
# Its #914 counters are deliberately *not* compared with any other record's. No record in the run
# is its baseline: the `console_meters` arms and the standing row are `Concurrent` plans, a
# different delivery and a different chain shape, so agreement with them today would be a
# coincidence and disagreement tomorrow would refuse a truthful record. The fold count is pinned per
# record instead (every route of the console folds, on every plan that states one), and the
# redirect count is the metered plan's own, pinned in `console-workload`'s pair test.
([.[] | select(.record == "console_session" and
               (.workload_kind == "sixty_four_track_console" or
                .workload_kind == "sixty_four_track_console_metered"))
      | .output_sha256] | length == 4 and (unique | length) == 1) and
([.[] | .backend] | unique | length) == 1 and
# Ragged versus full, and every decomposition subtraction, are the whole point of the fixture set,
# so the per-track costs must be comparable numbers taken on one host in one run: same binary,
# same commit, same metadata, same admissibility.
(metadata_names | all(. as $key | ([$records[] | .[$key]] | unique | length) == 1)) and
([$records[] | .missing_metadata] | unique | length) == 1 and
# #144 item 13: the run as a whole has to have stated whether it was controlled. A record whose
# runner never exported the name is admissible only as a *record*; a whole accepted run of them is
# not, because nothing downstream could then tell a pinned quiet host from a shared busy one.
all(.[]; .measurement_control != null) and
# #184: floor accounting is a property of the *run*. A stream with cycle columns on some session
# rows and not others is a runner that lost its performance counter half way through, and the two
# halves would not be comparable; and every column that a single record cannot check itself --
# the isolate, which is a subtraction between two rows -- is recomputed here.
([$records[] | select(.record == "console_session") | has("cycles_per_lane_sample")]
  | unique | length) == 1 and
(if ([$records[] | select(.record == "console_session") | has("cycles_per_lane_sample")]
      | all) then
   ([$records[] | select(.record == "console_session") | .core_clock_hz] | unique | length) == 1 and
   ([$records[] | select(.record == "console_session") | .core_clock_source]
     | unique | length) == 1 and
   # Per round, because a subtraction between rounds is a subtraction between two clocks.
   (map(select(.record == "console_session")) | group_by(.round) | all(
      (map({key: .workload_kind, value: .}) | from_entries) as $by |
      all(.[];
        .floor_control_row == "none" or
        (($by[.floor_control_row]) as $control |
         ($control != null) and
         near(.isolated_cycles_per_lane_sample;
              .cycles_per_lane_sample - $control.cycles_per_lane_sample; 0.003) and
         near(.isolated_percent_of_floor;
              100 * (.floor_cycles_per_lane_sample - $control.floor_cycles_per_lane_sample)
                / .isolated_cycles_per_lane_sample; 0.05)))))
 else true end)
