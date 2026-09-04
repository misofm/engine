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
