# Issue #368 floor recount evidence

Inspected baseline: 87926988. This accounting revision changes no rendered arithmetic.

Compressor 81.5 follows link_frame, one_frame/curve_target, ballistic, and gain_mix; current
max/min pricing removes 8.5 from 94, including the shared max at 0.5 per channel, and #367's
exp2_int_in_range synthesis removes the other 4. Limiter 129.5 follows
detector_peak/annex2_phases, the shared link, sliding_minimum_uniform, box operations and release;
repriced max/min removes 8.5 from 138. EQ remains 51 through process_channels and svf_step.
Builtins remains 69 through input sanitize, HPF/LPF svf_step, fader/matrix, mix2x2_block, and the
output reduction. The public clamped gain helper retains its clamps; only its admitted-domain
exp2_int_in_range leaf is the two-operation synthesis established by #367.

verify-historical-repricing.sh copies sealed artifacts/mono3 records into this namespace, changes
only the three authorized derived fields, proves exact equality after deleting those fields,
exercises current floor validation on cycle-bearing, ragged and unpriced rows, and proves stale
floors plus an unrelated malformed measured field reject. It refuses overwrite.

Attempt 1 (57a3f86c) received Astra FAIL for inconsistent cells, mixed historical/current
authority, and missing runner, repricing and derivation evidence. Attempt 2 is the bounded repair.

Attempt 2 (5a156c46) also received Astra FAIL: it used only the current library's `floor_shape`
rather than full historical record and aggregate validators, and its inverted jq checks did not
distinguish predicate rejection from execution errors. The final proof identifies the records'
candidate revision `dc581f3470b40678301d9504f1be4b1fd6be7173`, first proves its unchanged
validator pair and library accept the originals, and then changes only the compressor and limiter
inventory constants in a temporary library copy. Both full validators accept the repriced copy.
Explicit status checks prove stale floors and an unrelated measured-field mutation reject, valid
records make each negative assertion fail, and broken jq programs report execution failure.

## Final qualification

Astra passed attempt 3 at `6d64f11eded121761b5e835e7f98ce41dda6b892`. Committed-head operator preflight passed with zero workload launches, including workspace all-target/all-feature clippy. Formatting, realtime policy and diff checks passed. Workspace tests at implementation `5a156c46` passed 1546/0/24 (passed/failed/ignored), versus baseline `87926988` 1545/0/24: one new floor test. Later changes touch only evidence and its shell verifier.

The sole `--issue368-floor-recount` runner invocation at `6d64f11e` passed: one warmup, two measured rounds, 46 accepted records, raw and accepted bytes identical. Current full record and aggregate validators passed in the runner. CPU 63 was pinned with its sibling quiet; load average was 0.32, within the 0.50 ceiling, so the actual disposition is controlled even though the operator permitted an uncontrolled fallback. Hardware cycle counters were unavailable; the fresh records omit the complete optional cycle/floor group. The historical full-schema proof separately validates measured cycle-bearing repricing. No current cycle values or runtime speedup are claimed.

The preflight hashes its ordinary release binary; the runner explicitly freezes opt-level=3, LTO=false and codegen-units=16 before building and hashes that binary separately. Both identities are retained in preflight.json and the disposition; they are not asserted equal. No workload, source, validator or inventory changed between preflight and timing.

See console-benchmark.disposition.json for exact candidate/binary/artifact identities. The single run is consumed and must not be repeated.
