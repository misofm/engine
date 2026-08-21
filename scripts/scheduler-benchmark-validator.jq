include "scheduler-benchmark-record-lib";
. as $records |
type == "array" and length == 6 and all(.[]; scheduler_benchmark_record_valid) and
([.[] | .mode] | unique | sort) == ["four_lane","sequential","two_lane"] and
([.[] | .round] | unique | sort) == [1,2] and
([.[] | [.mode,.round] | join(":")] | unique | length) == 6 and
(group_by(.mode) | all(map(.round) | sort == [1,2])) and
([.[] | .candidate_sha256] | unique | length) == 1 and
([.[] | .binary_sha256] | unique | length) == 1 and
([.[] | .output_hash] | unique | length) == 1 and
([.[] | .wave_count] | unique | length) == 1 and
([.[] | .unit_count] | unique | length) == 1 and
([.[] | .cpu_model] | unique | length) == 1 and
([.[] | .os] | unique | length) == 1 and
([.[] | .kernel] | unique | length) == 1
