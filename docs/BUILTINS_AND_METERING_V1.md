# Builtins and metering V1

Issue 007 defines three fixed scalar graph sections per dual-mono track: input processing at
`post_input_builtins`, fader/mute at `post_fader`, and a declared 2x2 matrix at `post_matrix`.
The compiler binds these internally, so hosts continue to supply only source/input and output
bindings. No rack, graph topology, or session-schema semantics are introduced here.

Ahead of all of that, issue #210 phase 2 places the track's declared **input time alignment**:
`builtins.<lane>.delay_samples`, a per-lane sample count applied by a graph node at the track's
`Input` stage, before the fused input kernel. It sits there so that every downstream consumer sees
aligned audio -- the `input` send tap, sidechain sources reading that tap, and the input meter
included; anywhere later would leave those un-aligned. It is prepared-only (builtin parameter row
11, `PreparedOnly`, smoothing `None`) because changing a delay length mid-render re-times the ring
and glitches unavoidably. It is not latency: PDC never compensates it away. A track that declares
zero on both lanes -- almost every track -- is not lowered to a delay node at all, and its compiled
program is the one it had before the feature existed.

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

## Effect observation (issue #143)

Meters observe *boundaries*; observation taps observe *effects*. A track's peak is a fold over
samples the meter can see; a compressor's gain reduction is state only the compressor holds, and it
reaches a console through a separate mechanism with its own declared menu, cost classes and
conflating transport. `docs/EFFECT_OBSERVATION_V1.md` is that mechanism in full.

What belongs here is where the two meet: **one frame, one timeline**. Gain reduction rides the
existing `miso.meter.v1` post rather than a second message, so the pinned-occurrence rule for the
render callback is unchanged, and the window a gain-reduction value describes is the *same* meter
window the peak beside it describes — the observation window length is derived from
`console_meter_blocks`, not configured separately.

The frame is `3 * trackCount + 3` `f32` words: the frozen `2T + 2` peak section exactly where it
was, then one **non-negative decibel magnitude** per track and the designated master's. The sample
window rides a fixed `WebMeterHeaderV1` structure, because a `u64` does not survive an `f32` and
splitting one across two lanes would put a decoding rule in the app that nothing could check.

A session that asks for no observation capacity allocates none of it, renders byte-identical audio,
and reports `observation_retained_bytes == 0` — walked over the built runtime, not derived from the
request.

## Solo in place (issue #210 phase 1)

Solo is **console state, composed at command admission, with no render-plane code at all**. The
strip already carries a per-lane declicked gate whose target is `0.0` or the lane's fader gain, fed
by a bounded per-track queue of mute records. Solo-in-place adds a state machine above that queue —
`ConsoleSoloState` in `miso-engine-host-core` — which composes

```
effective_mute(track, lane) = user_mute(track, lane) || (any_solo_engaged && !this_track_soloed)
```

and emits the *existing* mute records into the *existing* queues. The render thread cannot tell a
solo-derived mute from a user mute, so every property the mute path already has is inherited
whole: allocation-free admission, no cross-lane audio coupling (the `||` is computed over booleans
on the control plane and never from audio), the per-sample D11 linear declick with the caller's own
`smoothing_samples`, and the all-or-nothing admission transaction.

**User mute and solo are separate states and neither overwrites the other.** That is the hardware
semantics, and it is what makes snapshot and restore correct by construction: muting a soloed strip
silences it, and clearing solo restores exactly the mutes the user had — *per lane*, because a
lane's mute is a lane's mute and one record carries one bool. The host keeps a mirror of user-mute
intent, initialized at preparation from the session's baked `fader.left_mute` / `fader.right_mute`,
because once solo exists the render side's flag holds the *effective* mute and there is no readback
of it.

Two rules of the admission path are load-bearing rather than incidental:

- **One coalesced net emission per submission.** Solo records stage nothing as they are read. The
  whole batch's state changes are applied first, and the difference between the composed effective
  mute and what the render plane was last told is staged once, at the end. Fanning out per command
  would put a gate record per track on the wire *per transition*, which a batch of alternating
  toggles turns into an overflow rather than a gesture.
- **Never a redundant record.** A lane whose effective mute did not change is not re-muted. The
  fader stage retargets unconditionally, so re-muting an already-*settled* muted lane with a
  nonzero window re-enters the ramp kernel — which multiplies by the current gain — instead of the
  settled kernel, which fills the plane. For a negative input that is the difference between an
  exact `+0.0` and a `-0.0`, and it is digest visible.

### Ruling D1 — solo is not persisted in Session V1

**Solo is monitoring state, not mix state, and no session key carries it.** A session reloads with
every solo bit clear, and an offline or stem render of a session can never come out soloed — the
reference PCM runner renders a session with no command stream at all, so a persisted solo bit would
silence stems that the session, read as a document, says are audible.

This does not violate the standing "protocol mutations update the typed session model and must be
snapshot-able" law, because solo deliberately does not mutate the session model — exactly as live
fader, pan, mute and effect-parameter moves already do not. Live console state is rebuilt from the
session on reload and is never written back.

Persisted solo-safe or monitor-scene semantics, if the product ever wants them, are a session-V2
monitor-scene concept and not a V1 key. Nothing here forecloses that.

### Metering and observation while soloed

The code is unchanged; the semantics are worth stating because a console user will ask.

The gate applies at the fader, so taps at `input`, `post_input_builtins`, `post_simd1`,
`post_dynamic` and `post_simd2_pre_fader` keep reading the **un-gated** signal — input and
pre-fader metering survives a solo, which is console-correct and is what makes gain-riding a
silenced strip possible. Taps at `post_fader` and `post_matrix`, and everything downstream of them
(submixes, outputs, the designated master's peak and gain-reduction rows), read the **gated** mix.

A gain-reduction tap on a strip that solo has silenced falls toward zero reduction, because its
effects are seeing silence. That is the true state of that signal path, not an artifact of the
observation surface.

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
