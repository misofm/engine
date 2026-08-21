include "builtins-benchmark-record-validator";

length == 20 and
all(builtins_benchmark_record_valid) and
([.[] | .workload] | unique | sort) == [
  "combined_1t_128",
  "combined_4t_128",
  "fader_mute_1t_128",
  "input_filters_1t_128",
  "input_identity_1t_128",
  "matrix_identity_1t_128",
  "matrix_ramp_1t_128",
  "meter_full_7taps_128",
  "meter_success_7taps_128",
  "prepare_65537t"
] and
(group_by(.workload) | all(length == 2 and ([.[].round] | sort) == [1, 2]))
