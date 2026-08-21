# Aggregate validator: exactly three workload kinds, exactly measured rounds one and two.
include "rack-benchmark-record-lib";
def valid_record: rack_benchmark_record_valid_lib;
. as $records |
(type == "array") and length == 6 and
all(.[]; valid_record) and
([.[] | .workload_kind] | unique | sort) == ["host_selected_eight_track_bank","mixed_twelve_track_graph","scalar_eight_tracks"] and
([.[] | .round] | unique | sort) == [1,2] and
([.[] | [.workload_kind,.round] | join(":" )] | unique | length) == 6 and
(group_by(.workload_kind) | all(map(.round) | sort == [1,2])) and
(group_by(.workload_kind) | all(map(.bank_backend) | unique | length == 1)) and
([.[] | .candidate_commit_sha256] | unique | length) == 1 and
([.[] | .binary_sha256] | unique | length) == 1 and
([.[] | .fixture_sha256] | unique | length) == 1 and
(group_by(.workload_kind) | all(map(.input_sha256) | unique | length == 1)) and
(group_by(.workload_kind) | all(map(.output_sha256) | unique | length == 1)) and
([.[] | select(.workload_kind == "scalar_eight_tracks") | .input_sha256][0] == [.[] | select(.workload_kind == "host_selected_eight_track_bank") | .input_sha256][0]) and
(metadata_names | all(. as $key | ([$records[] | .[$key]] | unique | length) == 1)) and
([$records[] | .missing_metadata] | unique | length) == 1
