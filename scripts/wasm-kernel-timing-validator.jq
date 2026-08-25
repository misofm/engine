# Aggregate validator for the issue #163 phase 0b wasm kernel timing arm.
#
# Eight records: four legs (native at simd4 and simd8, wasm-simd128 at both widths) in each of the
# two measured rounds. The rules that matter are not the percentiles -- they are descriptive -- but
# the three claims a reader would otherwise have to take on trust:
#
#   1. every record says it is not comparable with a native console record;
#   2. every leg computed the *pinned* corpus, proven by the digests being identical across every
#      leg and every width for a given case, so a timing difference is a target difference and not
#      two different computations; and
#   3. the paired delta the arm publishes is a difference from a baseline arm whose own delta is
#      zero by construction.
def sha256: type == "string" and test("^[0-9a-f]{64}$");
def positive_integer: type == "number" and floor == . and . > 0;
def arm_valid:
  (keys | sort) == ["case","case_index","digest","p50_ns","p95_ns","p99_ns","paired_delta_median_ns"] and
  (.case | type == "string" and length > 0) and
  (.case_index | type == "number" and floor == . and . >= 0) and
  ([.p50_ns,.p95_ns,.p99_ns] | all(positive_integer)) and
  (.p50_ns <= .p95_ns and .p95_ns <= .p99_ns) and
  (.paired_delta_median_ns | type == "number" and floor == .) and
  (.digest | sha256);
def record_valid:
  (keys | sort) == ["arms","backend","baseline","common_term","comparable_with_console_records","descriptive_only","issue","leg","observations","pairing","percentile_method","phase","record","round","runtime","schema_version","statistical_method","units","width"] and
  .schema_version == 1 and .issue == 163 and .phase == "0b" and
  .record == "wasm_kernel_timing" and
  (.round == 1 or .round == 2) and
  (.leg == "native" or .leg == "wasm") and
  .pairing == "alternating_per_observation" and
  .percentile_method == "nearest_rank" and .units == "ns_per_case" and
  .descriptive_only == true and
  (.observations | positive_integer) and
  (.runtime | test("^wasmtime ")) and
  # The whole reason this family exists separately. A record that dropped this could be read beside
  # a 222 us native console block as though the two measured the same thing.
  .comparable_with_console_records == false and
  # A wasm leg is labelled for the artifact the browser ships; a native leg names its width.
  (if .leg == "wasm" then .backend == "wasm-simd128"
   else .backend == ("native-" + .width) end) and
  (.width == "simd4" or .width == "simd8" or .width == "scalar") and
  (.baseline | arm_valid) and
  # The baseline is the subtrahend, so its own delta is zero by construction. A nonzero one means
  # the deltas were not taken against it.
  .baseline.paired_delta_median_ns == 0 and
  (.arms | type == "array" and length == 3 and all(arm_valid)) and
  # Every reported kernel must be more expensive than the baseline it is measured against, or the
  # difference is noise rather than a kernel cost.
  (.arms | all(.paired_delta_median_ns > 0)) and
  ([.baseline.case] + [.arms[].case] | unique | length) == 4;
. as $records |
(type == "array") and length == 8 and
all(.[]; record_valid) and
([.[] | .round] | unique | sort) == [1,2] and
([.[] | [.leg,.width,(.round|tostring)] | join(":")] | unique | length) == 8 and
([.[] | [.leg,.width] | join(":")] | unique | sort)
  == ["native:simd4","native:simd8","wasm:simd4","wasm:simd8"] and
# The claim that makes the comparison mean anything: every leg, at every width, in every round,
# produced the same digest for the same case. A timing difference is then a difference in how the
# target executed one pinned computation, never a difference in what it computed.
([$records[] | [.baseline.case, .baseline.digest]] | unique | length) == 1 and
(["svf_block_ramped/noise","one_pole_block/noise","lane_fma"] | all(. as $case |
  ([$records[] | .arms[] | select(.case == $case) | .digest] | unique | length) == 1))
