include "builtins-benchmark-record-validator";

def expected: ["full_chain_filters", "identity_chain", "matrix_ramp", "meter_success_full", "prepare_256_tracks"];

length == 20 and
all(builtins_benchmark_record_valid) and
([.[] | .workload_kind] | unique | sort) == expected and
([.[] | .sample_rate_hz] | unique | sort) == [48000, 96000] and
([.[] | [.workload_kind, .sample_rate_hz, .round] | @json] | unique | length) == 20 and
(group_by([.workload_kind, .sample_rate_hz]) |
 all(length == 2 and ([.[].round] | sort) == [1, 2] and
     ([.[].fixture_manifest_sha256] | unique | length) == 1 and
     ([.[].input_fixture_sha256] | unique | length) == 1 and
     ([.[].workload_id] | unique | length) == 1))
