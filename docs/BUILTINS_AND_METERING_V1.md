# Builtins and metering V1

Issue 007 defines three fixed scalar graph sections per dual-mono track: input processing at
`post_input_builtins`, fader/mute at `post_fader`, and a declared 2x2 matrix at `post_matrix`.
The compiler binds these internally, so hosts continue to supply only source/input and output
bindings. No rack, graph topology, or session-schema semantics are introduced here.

Each input lane applies polarity, trim, an optional RBJ second-order Butterworth HPF, then an
optional RBJ second-order Butterworth LPF. Coefficients are designed in `f64`, checked for the
strict second-order Jury conditions, cast before render, and run in independent transposed
direct-form-II state. This is the normalized coefficient family documented by [RBJ-COOKBOOK];
the realization and stability checks follow the implementation guidance summarized by
[SMITH-SASP] and [ORFANIDIS-ISP]. Enabled filters declare an infinite tail; all other builtin
parts declare a zero finite tail and zero latency.

All lane input, intermediate output, and filter state are sanitized to exact positive zero when
nonfinite or subnormal. L and R state never aliases. Fader/mute occurs after racks; mute writes
exact positive zero without suspending filter state. Matrix coefficients are bounded finite
values in `[-1, 1]`; a target advances once per sample as
`current += (target - current) / remaining_updates`, reaching the target exactly on update N.
The pan adapter is a V2 product definition using cosine/sine gains over `[-1, 1]`; it is not an
implicit stereo mode.

Meters are post-node observers at the seven stable `TrackStage` boundaries. A meter owns its
bounded SPSC producer and reports exact sample windows, per-lane peak, energy, RMS, held peak,
interval/cumulative clipping and sanitization counts, discontinuities, and dropped snapshots.
Queue-full drops one snapshot and increments a saturating counter; it never blocks or retries.
Raw energy/RMS observations are only *loudness-ready*: they are not BS.1770 K-weighted, gated,
LUFS/LKFS, true-peak, or certified loudness measurements. [ITU-BS1770-5] and [EBU-R128] delimit
those explicitly out-of-scope claims.

## Current evidence status

The implementation tests verify scalar gain/matrix/meter basics, all named tap preparation,
transactional duplicate/unknown meter rejection, graph installation of exactly the three internal
sections with infinite filter-tail propagation, and 10,000 deterministic bounded parameter/block
mutations. An independent `f64` oracle re-derives RBJ HPF/LPF coefficients and verifies the
cascaded impulse response at every required rate. Release scalar-builtins compilation also passes
Android/iOS AArch64 and baseline/`simd128` Wasm targets. This does not yet constitute the full
issue acceptance corpus: independent `f64` sweep fixtures, manifest-checked fixtures, allocation
trace, one-million-call audit, full workspace target builds, and real blinded listening records
remain required.
The exactly-once benchmark has not been invoked.
