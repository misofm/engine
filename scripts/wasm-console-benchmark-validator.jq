# Aggregate validator for the issue #163 phase 2 step 1 wasm console arm.
#
# Eighteen records: the nine console workloads in each of the two measured rounds. Each record
# carries three legs of the *same* subject -- native at Simd8, native at Simd4, and
# `wasm32-unknown-unknown` with `+simd128` under the pinned wasmtime -- interleaved observation by
# observation so their ratio is a paired statistic rather than a quotient of two summaries.
#
# The percentiles are descriptive and this file does not judge them. What it judges are the claims
# a reader would otherwise have to take on trust:
#
#   1. **The family boundary.** Every record says it is not comparable with a native console
#      record, and says it is not a browser field measurement. A `wasm-simd128` number and the
#      91.0 us native console number differ in target, in lane width, and in whether one
#      multiply-add is one instruction or fifty-four; and wasmtime's Cranelift is an ahead-of-time
#      compiler while a browser JIT tiers. A record that dropped either flag could be read beside
#      a native record, or quoted as a phone budget, without anything catching it.
#
#   2. **One subject, three targets.** Every leg of a record rendered byte-identical output. This
#      is what makes the ratio a statement about a *target* rather than about two different
#      computations -- and it is a real result, not a formality: the first run of this arm was
#      divergent, because the benchmark's tone is a libm sine and libm differs between the two
#      targets. The runner now injects one target's samples into both. `digest_identity` must
#      agree with the digests actually present, so the summary cannot say "identical" over legs
#      that are not.
#
#   3. **The ratios are the legs'.** Each published `ratio_of_p50` is recomputed here from the p50s
#      of the two legs it names. A ratio table that drifted from the samples under it would be the
#      one thing in this record family a reader cannot check by eye.
#
#   4. **Every row is the row it claims.** A decomposition row that quietly kept a rack it says it
#      emptied would make every subtraction between rows wrong, so the strip content, the track
#      count, the input signal and the synthetic-fixture flag are pinned per workload kind.
def sha256: type == "string" and test("^[0-9a-f]{64}$");
def positive_integer: type == "number" and floor == . and . > 0;
def nonnegative_integer: type == "number" and floor == . and . >= 0;
def close($a; $b): (($a - $b) | if . < 0 then -. else . end) < 0.002;

# workload kind -> [tracks, synthetic, strip_content, strip_layout, input_signal, fixture_id]
#
# `strip_layout` joined the pin in #175. Two rows in this table now carry the same
# `strip_content` (`eq+compressor`) and differ only in where those two effects sit, so without it
# the chain-shape row-pair would be two indistinguishable rows.
def workload_pins:
  {
    "nine_track_baseline":
      [9, false, "eq", "simd1:eq", "tone", "fixtures/session/v1/parametric-eq-nine-track.toml"],
    "nine_track_ragged_strip":
      [9, true, "eq+compressor+limiter", "simd1:eq+compressor,simd2:limiter", "tone", "fixtures/session/v1/console-sixty-four-track-intended.toml"],
    "sixty_four_track_console":
      [64, false, "eq+compressor+limiter", "simd1:eq+compressor,simd2:limiter", "tone", "fixtures/session/v1/console-sixty-four-track-intended.toml"],
    "one_twenty_eight_track_stretch":
      [128, true, "eq+compressor+limiter", "simd1:eq+compressor,simd2:limiter", "tone", "fixtures/session/v1/console-sixty-four-track-intended.toml"],
    "sixty_four_track_eq_only":
      [64, true, "eq", "simd1:eq", "tone", "fixtures/session/v1/console-sixty-four-track-intended.toml"],
    "sixty_four_track_compressor_only":
      [64, true, "compressor", "simd1:compressor", "tone", "fixtures/session/v1/console-sixty-four-track-intended.toml"],
    "sixty_four_track_builtins_only":
      [64, true, "builtins", "builtins", "tone", "fixtures/session/v1/console-sixty-four-track-intended.toml"],
    "sixty_four_track_dispatch_only":
      [64, true, "identity", "builtins", "tone", "fixtures/session/v1/console-sixty-four-track-intended.toml"],
    "sixty_four_track_idle":
      [64, true, "eq+compressor+limiter", "simd1:eq+compressor,simd2:limiter", "silence", "fixtures/session/v1/console-sixty-four-track-intended.toml"],
    "sixty_four_track_console_legacy":
      [64, false, "eq+compressor", "simd1:eq,dynamic:compressor", "tone", "fixtures/session/v1/console-sixty-four-track.toml"],
    "sixty_four_track_eq_comp_simd1":
      [64, true, "eq+compressor", "simd1:eq+compressor", "tone", "fixtures/session/v1/console-sixty-four-track-intended.toml"]
  };

# The three legs, in the fixed order they are rendered and emitted.
def leg_pins:
  [
    ["native_simd8", "native", "Simd8", "host_process_heap"],
    ["native_simd4", "native", "Simd4", "host_process_heap"],
    ["wasm_simd128", "wasm32-unknown-unknown", "Simd4", "not_observable_guest_linear_memory"]
  ];

def leg_valid($pin):
  (keys | sort) == ["audit_scope","backend","leg","max_ns_per_block","min_ns_per_block",
                    "output_sha256","p50_ns_per_block","p50_us_per_block",
                    "p50_us_per_block_per_track","p95_ns_per_block","p99_ns_per_block",
                    "render_errors","target"] and
  .leg == $pin[0] and .target == $pin[1] and .backend == $pin[2] and .audit_scope == $pin[3] and
  ([.min_ns_per_block,.p50_ns_per_block,.p95_ns_per_block,.p99_ns_per_block,.max_ns_per_block]
     | all(positive_integer)) and
  (.min_ns_per_block <= .p50_ns_per_block) and
  (.p50_ns_per_block <= .p95_ns_per_block) and
  (.p95_ns_per_block <= .p99_ns_per_block) and
  (.p99_ns_per_block <= .max_ns_per_block) and
  # The microsecond projection must be the nanosecond one, or a reader comparing the convenient
  # field against the authoritative field would find two different numbers.
  close(.p50_us_per_block; .p50_ns_per_block / 1000) and
  (.render_errors | nonnegative_integer) and
  (.output_sha256 | sha256);

def ratio_valid($legs):
  (keys | sort) == ["denominator","numerator","paired_ratio_median","ratio_of_p50"] and
  .numerator == "wasm_simd128" and
  (.denominator == "native_simd8" or .denominator == "native_simd4") and
  (.ratio_of_p50 | type == "number" and . > 0) and
  (.paired_ratio_median | type == "number" and . > 0) and
  # Recomputed from the legs this ratio names.
  (. as $ratio |
    ([$legs[] | select(.leg == $ratio.numerator) | .p50_ns_per_block] | first) as $top |
    ([$legs[] | select(.leg == $ratio.denominator) | .p50_ns_per_block] | first) as $bottom |
    close($ratio.ratio_of_p50; $top / $bottom));

def record_valid:
  (keys | sort) == ["background_load_note","browser_field_measurement","candidate_commit",
                    "comparable_with_console_records","cpu_affinity","cpu_model",
                    "descriptive_only","digest_identity","fixture_id","governor_or_power_mode",
                    "guest_call_overhead_p50_ns","guest_module_sha256","guest_target",
                    "guest_target_features","input_signal","issue","legs","llvm_version",
                    "measurement_control","missing_metadata","observations","os","percentile_method",
                    "phase","profile","quantum_frames","ratios","record",
                    "render_total_forbidden_operations","round","runtime","rust_version",
                    "sample_rate_hz","schema_version","statistical_method","strip_content",
                    "strip_layout","synthetic_fixture","target_features","target_triple","tracks","units",
                    "warmup_blocks","workload_kind"] and
  .schema_version == 1 and .issue == 163 and .phase == "2-step1" and
  .record == "wasm_console_session" and
  (.round == 1 or .round == 2) and
  .sample_rate_hz == 48000 and .quantum_frames == 128 and
  (.observations | positive_integer) and
  (.warmup_blocks | positive_integer) and
  .units == "us_per_block" and .percentile_method == "nearest_rank" and
  .descriptive_only == true and
  (.runtime | test("^wasmtime ")) and
  .guest_target == "wasm32-unknown-unknown" and
  .guest_target_features == "+simd128" and
  (.guest_module_sha256 | sha256) and
  (.guest_call_overhead_p50_ns | nonnegative_integer) and
  (.render_total_forbidden_operations | nonnegative_integer) and
  (.measurement_control == "controlled" or .measurement_control == "uncontrolled") and
  (.missing_metadata | type == "array") and
  # Claim 1: the family boundary, both halves.
  .comparable_with_console_records == false and
  .browser_field_measurement == false and
  # Claim 4: the row is the row it claims.
  (. as $record | workload_pins[$record.workload_kind] as $pin |
    ($pin != null) and
    $record.tracks == $pin[0] and
    $record.synthetic_fixture == $pin[1] and
    $record.strip_content == $pin[2] and
    $record.strip_layout == $pin[3] and
    $record.input_signal == $pin[4] and
    $record.fixture_id == $pin[5]) and
  # The three legs, in their fixed order, each internally consistent.
  (.legs | type == "array" and length == 3) and
  ([.legs, leg_pins] | transpose | all(. as $pair | $pair[0] | leg_valid($pair[1]))) and
  # Claim 2: one subject, three targets -- and a summary that cannot lie about it.
  (. as $record |
    ([$record.legs[] | .output_sha256] | unique | length) as $distinct |
    if $record.digest_identity == "all_legs_identical" then $distinct == 1
    elif $record.digest_identity == "divergent" then $distinct > 1
    else false end) and
  # Claim 3: the published ratios are the legs' own.
  (. as $record |
    ($record.ratios | type == "array" and length == 2) and
    ($record.ratios | all(ratio_valid($record.legs))) and
    ([$record.ratios[] | .denominator] | unique | sort) == ["native_simd4","native_simd8"]);

. as $records |
(type == "array") and length == 22 and
all(.[]; record_valid) and
([.[] | .round] | unique | sort) == [1,2] and
# Eleven workloads, each measured exactly once per round, and no workload measured twice.
([.[] | [.workload_kind,(.round|tostring)] | join(":")] | unique | length) == 22 and
([.[] | .workload_kind] | unique | length) == 11 and
([.[] | .workload_kind] | unique | sort) == (workload_pins | keys | sort) and
# One guest module produced every row. Two modules in one record set would mean two different
# machine codes were timed and reported as one measurement.
([.[] | .guest_module_sha256] | unique | length) == 1 and
([.[] | .runtime] | unique | length) == 1 and
# The whole arm's central result, asserted over the set rather than per record: every workload,
# in every round, on every target, computed the same bits.
all(.[]; .digest_identity == "all_legs_identical") and
# The same workload in the two rounds must render the same output on every leg. A round that
# rendered something else did not re-measure the row it claims to have re-measured.
([.[] | [.workload_kind, (.legs[0].output_sha256)] | join(":")] | unique | length) == 11
