# Builtins and metering V1

Issue 007 defines three fixed scalar graph sections per dual-mono track: input processing at
`post_input_builtins`, fader/mute at `post_fader`, and a declared 2x2 matrix at `post_matrix`.
The compiler binds these internally, so hosts continue to supply only source/input and output
bindings. No rack, graph topology, or session-schema semantics are introduced here.

Each input lane applies polarity, trim, an optional RBJ-second-order-Butterworth-response HPF,
then an optional LPF. The production realization is the conditioned two-integrator trapezoidal
state-variable recurrence: design uses `f64`, then stores the final `c1`, `a2`, `a3`, and `k` bits
as `f32`; audio, states, and every base operation are separately rounded non-fused `f32`.
`c1` is prepared directly rather than reconstructed from a rounded complement. The cast-bit state
transition receives a strict Jury check off render; the independent `f64` oracle uses separately
derived RBJ transfer equations. This preserves the normalized response documented by
[RBJ-COOKBOOK] while retaining the frozen scalar/SIMD operation contract. Enabled filters declare
an infinite tail; all other builtin parts declare a zero finite tail and zero latency.

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

The machine-qualified fixture corpus covers the declared filter, gain, matrix, graph-tap, meter,
diagnostic, and resource tuples. Its sorted manifest rejects changed, missing, unlisted, and
coverage-hole artifacts; the independent `f64` oracle never calls production builtins. The opaque
prepared artifact has an independently corruptible test-only seal probe, while phase-two allocator
tracking verifies the reported retained payload total and largest request. Direct and graph-backed
one-million-render audits cover the forbidden-operation hooks, bounded meter queues, swaps, and
retirement ownership; target, workspace, policy, mutation, formatting, lint, and rustdoc gates are
recorded by the issue preflight.

Human listening remains pending under Issue 033. The frozen Issue-007 benchmark runner is present
but has not been invoked; the authorized timing-launch count remains zero until its separate
authorization.
