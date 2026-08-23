# Builtins and metering V1

Issue 007 defines three fixed scalar graph sections per dual-mono track: input processing at
`post_input_builtins`, fader/mute at `post_fader`, and a declared 2x2 matrix at `post_matrix`.
The compiler binds these internally, so hosts continue to supply only source/input and output
bindings. No rack, graph topology, or session-schema semantics are introduced here.

Each input lane applies polarity, trim, an optional RBJ-second-order-Butterworth-response HPF,
then an optional LPF. The production realization is the topology-preserving two-integrator
state-variable recurrence of master plan #83 §4.2, and there is exactly **one** of it: the block
kernel `miso_engine_lane::kernels::svf_block`, generic over `Lane` and instantiated at `f32`,
`Simd4` and `Simd8` from one source. A scalar track is that body at `WIDTH = 1` over planar
slices; a bank is the same body at four or eight lanes over an AoSoA block. Design is `f64` and
stores `c1 = t / (1 + t)`, `a2` and `a3` as `f32`, cast once; `c1` is prepared directly rather than
reconstructed from a rounded complement.

The frozen operation order is the kernel's, `fma` at the three recurrence sites (master plan D3:
fusion exists only where `Lane::fma` is written). The filter kind is a per-lane output mix
`(m0, m1, m2)` — high-pass `(1, -k, -1)`, low-pass `(0, 0, 1)` — so a bank needs no per-lane
high-pass mask, and a **disabled section is the arithmetic identity** `(1, 0, 0)` with zero
coefficients rather than a branch. That identity is exact for every finite input except a negative
zero, which its trailing `+ 0.0` normalizes to positive zero; the behaviour is uniform across every
width and target, which is the property the determinism claim buys. The pre-#83 preparation-time
Jury check and cutoff-response gate are gone: the public cutoff domain is the frozen issue-036
table, enforced before preparation, and preparation now rejects only a coefficient that is not
representable in `f32`. Enabled filters declare an infinite tail; all other builtin parts declare a
zero finite tail and zero latency.

Checks go where the hazard is (master plan D7). Input is sanitized **once per channel per block**,
at the input stage: a sample whose magnitude is not below `1e30` — which includes every NaN,
because an ordered compare against NaN is false — becomes exact positive zero and increments
`sanitized_input`. A subnormal input is no longer sanitized: it is a legal finite sample. The two
recursive state words of each section are flushed to positive zero below `1e-20` inside the kernel,
which is the only denormal mechanism and strictly contains the band hardware FTZ acts on. Output
finiteness is checked **once per block, per lane**, on the output of the recursive stage: a failing
lane has its block zeroed and both of its sections reset, and increments `recovered_left_state` or
`recovered_right_state` — which therefore count lane-blocks, not samples. No other lane's bits
move, so a track's output never depends on its cohort. `sanitized_output` is retained for API
stability and is always zero. Fader and matrix are feed-forward with bounded coefficients, so
finite in implies finite out and they carry no checks and no counters.

L and R state never aliases. Fader/mute occurs after racks; mute clears every bit, so a muted lane
is exact positive zero even for a negative input, while an unmuted lane at unity gain preserves a
negative zero. Matrix coefficients are bounded finite values in `[-1, 1]`; a settled lane whose
matrix is exactly the identity passes its samples through untouched. A retarget computes each
coefficient's per-sample increment **once**, at the event (`step = (target - current) / n`), then
iterates `current += step` and assigns the target exactly on the last sample (master plan D11).
The pre-#83 law, which divided by the remaining count on every sample, is not the same arithmetic
for windows longer than two samples.

A bank accepts one to `width` prepared tracks. Lanes at or above that count are **padding lanes**:
they carry identity coefficients and unit trim, they are sanitized like any other lane so nothing
left in the scratch buffer can poison the recurrence, they are excluded from every counter and from
the boundary check, and their samples are never observed. The caller assigns lanes in sorted member
order and never gathers into or scatters from a padding lane.

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
