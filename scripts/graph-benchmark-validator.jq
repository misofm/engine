include "graph-benchmark-record-validator";

length == 6 and
all(graph_benchmark_record_valid) and
([.[] | .benchmark_id] | unique | sort) == [
    "graph_compile_256t_1024r_32s",
    "graph_debug_sha_dot_256t_1024r_32s",
    "graph_validate_65537_tracks"
] and
(group_by(.benchmark_id) | all(
    length == 2 and
    ([.[] | .round] | sort) == [1, 2] and
    ([.[] | .fixture_sha256] | unique | length) == 1 and
    ([.[] | .fixture_bytes] | unique | length) == 1 and
    ([.[] | .fixture_counts] | unique | length) == 1 and
    ([.[] | .output_graph_sha256] | unique | length) == 1 and
    ([.[] | .output_counts] | unique | length) == 1
))
