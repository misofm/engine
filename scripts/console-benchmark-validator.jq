# Aggregate validator: eleven session workloads, two hoist workloads, one meters arm, one
# observation arm and one placement row-pair, each in rounds one and two -- thirty-two records.
include "console-benchmark-record-lib";
. as $records |
(type == "array") and length == 32 and
all(.[]; console_benchmark_record_valid_lib) and
([.[] | select(.record == "console_session") | .workload_kind] | unique | sort) == session_kinds and
([.[] | select(.record == "console_hoist") | .workload_kind] | unique | sort)
  == ["nine_track_ragged_strip","sixty_four_track_console"] and
([.[] | select(.record == "console_session")] | length) == 22 and
([.[] | select(.record == "console_hoist")] | length) == 4 and
([.[] | select(.record == "console_meters")] | length) == 2 and
([.[] | select(.record == "console_observation")] | length) == 2 and
([.[] | select(.record == "console_placement")] | length) == 2 and
([.[] | .round] | unique | sort) == [1,2] and
([.[] | [.record,.workload_kind,.round] | join(":")] | unique | length) == 32 and
(group_by([.record,.workload_kind]) | all(map(.round) | sort == [1,2])) and
# Round one and round two are two measurements of one frozen workload, so the rendered output must
# be identical across them. A drifting digest means the rounds are not measuring the same thing.
(group_by([.record,.workload_kind]) | all(map(.output_sha256 // .restated_output_sha256 // .meters_off_output_sha256 // .absent_output_sha256 // .split_chains_output_sha256) | unique | length == 1)) and
([.[] | .backend] | unique | length) == 1 and
# Ragged versus full, and every decomposition subtraction, are the whole point of the fixture set,
# so the per-track costs must be comparable numbers taken on one host in one run: same binary,
# same commit, same metadata, same admissibility.
(metadata_names | all(. as $key | ([$records[] | .[$key]] | unique | length) == 1)) and
([$records[] | .missing_metadata] | unique | length) == 1 and
# #144 item 13: the run as a whole has to have stated whether it was controlled. A record whose
# runner never exported the name is admissible only as a *record*; a whole accepted run of them is
# not, because nothing downstream could then tell a pinned quiet host from a shared busy one.
all(.[]; .measurement_control != null)
