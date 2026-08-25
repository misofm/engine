# Aggregate validator: four session workloads and two hoist workloads, each in rounds one and two.
include "console-benchmark-record-lib";
. as $records |
(type == "array") and length == 12 and
all(.[]; console_benchmark_record_valid_lib) and
([.[] | select(.record == "console_session") | .workload_kind] | unique | sort)
  == ["nine_track_baseline","nine_track_ragged_strip","one_twenty_eight_track_stretch","sixty_four_track_console"] and
([.[] | select(.record == "console_hoist") | .workload_kind] | unique | sort)
  == ["nine_track_ragged_strip","sixty_four_track_console"] and
([.[] | select(.record == "console_session")] | length) == 8 and
([.[] | select(.record == "console_hoist")] | length) == 4 and
([.[] | .round] | unique | sort) == [1,2] and
([.[] | [.record,.workload_kind,.round] | join(":")] | unique | length) == 12 and
(group_by([.record,.workload_kind]) | all(map(.round) | sort == [1,2])) and
# Round one and round two are two measurements of one frozen workload, so the rendered output must
# be identical across them. A drifting digest means the rounds are not measuring the same thing.
(group_by([.record,.workload_kind]) | all(map(.output_sha256 // .restated_output_sha256) | unique | length == 1)) and
([.[] | .backend] | unique | length) == 1 and
# Ragged versus full is the whole point of the fixture, so the per-track costs must be comparable
# numbers taken on one host in one run: same binary, same commit, same metadata.
(metadata_names | all(. as $key | ([$records[] | .[$key]] | unique | length) == 1)) and
([$records[] | .missing_metadata] | unique | length) == 1
