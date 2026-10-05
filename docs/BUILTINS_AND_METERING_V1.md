# Builtins and metering V1

Issue 007 defines three fixed scalar graph sections per dual-mono track: input processing at
`post_input`, fader/mute at `post_fader`, and a declared 2x2 matrix at `post_pan` (the session
tap tokens decision 12 renamed; the internal stages keep their names).
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
then an optional LPF.

**Polarity and trim are live since issue #210 phase 3** (command kinds 11 and 10, builtin
parameter rows 1 and 2, both `BlockTarget` with `LinearNUpdates`). They are one coefficient:
`InputLane::trim_signed` carries the trim magnitude in its value and the polarity in its sign, so a
trim ride retargets the magnitude, a polarity flip retargets the sign, and both declick through the
same linear ramp -- a flip is the ramp passing through zero, which is why it costs no DSP of its
own. The ramping kernel is `input_chain_ramp_block`; the settled path, which is what a lane no
command has ever addressed runs, is the untouched `input_chain_block_elided` behind one `bool`.
Both ramping bodies -- dual and collapsed -- carry cross-target determinism pins of their own, as
corpus cases `input_stage/trim_ramp` and `input_stage/trim_ramp_mono`.
`hpf_hz` and `lpf_hz` remain prepared-only at the price recorded in
`docs/rulings/builtins-input-liveness-d2.md`, and that ruling also carries the obligation any
future filter liveness inherits: it must invalidate the prepared-identity elision plan, which
trim liveness does not touch because the plan reads only the SVF section words. The production realization is the topology-preserving two-integrator
state-variable recurrence of master plan #83 §4.2, and there is exactly **one** of it: the block
kernel `lane::kernels::svf_block`, generic over `Lane` and instantiated at `f32`,
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
which is the only denormal mechanism and strictly contains the band hardware FTZ acts on; and when
both are below `1e-14` (`REST_EPS`) they are flushed to positive zero together (`lane::flush_pair`,
issue #1328), so an enabled filter reaches exact rest after its input stops instead of holding a
limit cycle near the top of the cutoff domain. Output
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

The pan adapter is the product definition using cosine/sine gains over `[-1, 1]`; it is not an
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
reaches the live controls through a separate mechanism with its own declared menu, cost classes and
conflating transport. `docs/EFFECT_OBSERVATION_V1.md` is that mechanism in full.

What belongs here is where the two meet: **one frame**. Gain reduction rides the existing
`miso.meter.v1` post rather than a second message, so the pinned-occurrence rule for the render
callback is unchanged. It does **not** share the peak window: the header's
`first_sample`/`end_sample` timestamp only the peaks, and each gain-reduction word is the latest
fold of its effect's own observation window, aged independently of the peak window.

The frame is `3 * (trackCount + submixCount) + 3` `f32` words (issue #1209): one peak pair per
strip -- the tracks, then the submix strips in canonical submix order -- the master pair, then one
**non-negative decibel magnitude** per strip in the same order and the designated master's. With no
submixes every word is where it was before submixes existed. The sample window and the submix count
ride a fixed 72-byte `WebMeterHeader` structure (`submix_count` at offset 64), because a `u64` does
not survive an `f32` and splitting one across two lanes would put a decoding rule in the app that
nothing could check. With meters on, host-web meters every strip at `PostMatrix`.

The `miso.meter.v1` message keeps its track fields unchanged -- `peaks` is the `2T + 2` track and
master peaks, `trackGrDb` the `T` track magnitudes, `masterGrDb` the master's or `null` -- and
appends `submixCount`, `submixPeaks` (`[bus0 L, bus0 R, ..]`, `2S` words) and `submixGrDb` (`S`
non-negative magnitudes). Bus names are not in the frame; they are positional in canonical submix
order.

A session that asks for no observation capacity allocates none of it, renders byte-identical audio,
and reports `observation_retained_bytes == 0` — walked over the built runtime, not derived from the
request.

## Solo in place (issue #210 phase 1)

Solo is **live-control state, composed at command admission, with no render-plane code at all**. The
strip already carries a per-lane declicked gate whose target is `0.0` or the lane's fader gain, fed
by a bounded per-track queue of mute records. Solo-in-place adds a state machine above that queue —
`LiveControlSoloState` in `host-core` — which composes

```
effective_mute(strip, lane) = user_mute || (any_solo && !solo_safe(strip) && !soloed(strip))
```

Submixes are solo-safe: a submix is never soloed (a solo at its strip index refuses with
`notSoloable`) and never solo-muted, so a soloed track stays audible through every bus and return it
feeds (issue #1213).

and emits the *existing* mute records into the *existing* queues. The render thread cannot tell a
solo-derived mute from a user mute, so every property the mute path already has is inherited
whole: allocation-free admission, no cross-lane audio coupling (the `||` is computed over booleans
on the control plane and never from audio), the per-sample D11 linear declick with the caller's own
`smoothing_samples`, and the all-or-nothing admission transaction.

**User mute and solo are separate states and neither overwrites the other.** That is the hardware
semantics, and it is what makes snapshot and restore correct by construction: muting a soloed strip
silences it, and clearing solo restores exactly the mutes the user had — *per lane*, because a
lane's mute is a lane's mute and one record carries one bool. The host keeps a mirror of user-mute
intent, initialized at preparation from the session's own `fader.left_mute` / `fader.right_mute`,
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

### VCA groups (issue #1242)

A VCA mute composes into the same state, as a third term:

```
effective_mute(strip, lane) = user_mute || vca_mute || (any_solo && !solo_safe(strip) && !soloed(strip))
```

`vca_mute` is whether any VCA reaching the strip mutes the lane, from the composition preparation
bakes (`session::SessionModel::effective_strip_faders`); the user mute stays the member's own
intent, so the solo state is seeded with both and the emitted mirror with their OR, and seeding
emits nothing. Precedence is fixed:

- **Mute wins.** Solo never clears a user or VCA mute: a soloed member of a muted VCA stays muted,
  and so do its following sends. An explicit unmute of a VCA-muted member records the intent but
  stages the lane still muted. A both-lanes kind 4 on a strip whose VCA mutes one lane composes to
  two lane values, so it stages one `Left` and one `Right` record and needs two slots in that
  strip's fader queue: at a queue depth of 1 it is refused whole as typed backpressure (a caller
  can send the lanes separately), as the other two-record lowerings are.
- **Solo-safe is not VCA-safe.** A submix is never solo-muted, but a VCA that reaches it mutes it.
- **A VCA has no solo.** A VCA mute neither engages a solo nor counts toward `any_solo`.

A VCA's offset is baked into each member's prepared fader gain.

### Live VCA groups (issue #1245)

The browser rides and mutes a VCA live with two command kinds, 16 `vcaFaderDb` and 17 `vcaMute`.
Their index word is a **VCA index** (the VCA's position in the session's `vcas`, canonical VCA-ID
order), refused `unknownVca` (reason 14) at or past the VCA count; their shape is `faderDb`'s and
`mute`'s plus a zero `effect_index` and `parameter_id`. A VCA has no audio path and no render code:
admission composes every move through `host_core::LiveVcaState`, the live copy of the composition
preparation bakes, into the records the members already take.

- **A ride** moves the VCA's offset and stages nothing itself. After the batch, one VCA fader pass
  stages, for every strip a VCA reaches, a `TrackFaderRecord::FaderDb` on the member's own fader
  queue for each lane whose effective value
  `clamp(own + sum of the reaching VCAs' offsets, -144, 24)` changed, with the last ride's ramp, so
  every member follows through its existing declicked fader ramp, together. A member pushed past
  +24 dB clamps there and returns to its own balance when the VCA comes back: the state keeps the
  member's own value, never the clamped one.
- **A member's own `faderDb`** on a reached strip moves its own value and stages its effective
  value, so it lands on top of the VCA and keeps its balance. It always stages, like a `faderDb` on
  a strip no VCA reaches: a member clamped before and after the move re-stages its unchanged
  clamped target, which moves no bit. A strip no VCA reaches lowers exactly as before.
- **A mute** sets the VCA's mute term in the one strip-mute owner, so the solo coalescing pass emits
  every member's changed lanes and the follow pass every following send's, with the last kind 9 or
  17 record's ramp. Mute wins over solo, a solo-safe submix is muted too, and un-muting a VCA leaves
  a member's own mute on. A kind 4 later in the same batch composes with the new VCA mute.
- **Never a redundant record**, for the digest reason above: a ride that changes no member's
  effective value (every member clamped at -144 dB) and a mute of already-muted members stage
  nothing.
- **All or nothing.** VCA fader records, then strip mute records, then follow records are staged and
  room-checked together before any push; a full member or send queue refuses the whole submission
  as typed backpressure, and the VCA state rolls back with the solo state and the send mirror. The
  decode staging grows by two entries per strip a VCA reaches, and the bridge's exact retained
  report charges the VCA state.
- **Bounded per-command work** (amendment A1, a planner decision subject to owner review). A batch's
  VCA work grows with the (strip, reaching VCA) pairs, and it runs on the AudioWorklet thread, so a
  browser session may declare at most 256 VCAs and 16,384 pairs; past either it is refused at boot
  (`web.vca.maximum_vcas`, `web.vca.reach_pairs`, `RESULT_REFUSED_BUDGET`) before any VCA table is
  built, and the VCA state's retained bytes and construction transient are projected against the
  memory budget. At the bound, the worst batch (a ride and a mute of a VCA reaching every pair plus
  254 member moves) admits in about 0.15 ms native and 0.18 ms (median) in the shipped simd128
  module under V8, against a 2.67 ms quantum.

### Sends follow mute (issue #1224)

A send with `follows_mute: true` follows its source strip's **effective** mute live: user mute, VCA
mute, or solo-derived mute for a track. A submix is solo-safe, so a bus source follows only its own
mute and its VCA mute. A plan prepared with a VCA-muted source zeroes the send's muted columns
(#1242), and the live mirror starts from that same effective mute.
Muting a track or a bus, or soloing another track, silences every such send from that strip through
the same declicked ramp, in the same submission; unmuting or un-soloing reopens it. A soloed vocal
therefore no longer carries a muted drum track's pre-fader reverb send.

- **One composition.** `host_core::LiveRouteMuteFollow::delta` names the live sends whose
  follow-zeroed source lanes, `[effective_mute(source, 0), effective_mute(source, 1)]`, differ from
  what their mirror last sent. It takes the effective mute as a function, so the browser reads its
  solo state and the C ABI its committed model's mutes (#1226). A send without `follows_mute` is
  never touched.
- **After the batch's last mute.** The browser runs the follow pass once per submission, after
  every kind 4 record and the solo coalescing pass, so it reads the batch's final effective mutes.
  Each yielded send gets one record from its mirror through `RouteControlProducer::record`, the
  prepared route's own coefficient function, so a settled follow equals a plan freshly prepared
  with those mutes. Its ramp is the `smoothing_samples` of the last strip mute record staged for
  the source strip in that submission that covers a lane whose effective mute changed, so the send
  fades with its strip; a no-op record on the other lane never sets it.
- **Never a redundant record.** `delta` yields only a change, for the same digest reason as the
  strip records above.
- **All or nothing.** Follow records are room-checked with the strip mute records before any push.
  A full send queue refuses the whole submission as typed backpressure, at the wire index of the
  first record staged on that send's queue (which can be an earlier send command, kinds 13-15, on
  the same send), and the solo state and the send mirror both roll back. When a mute change moves
  a following send, a window that exceeds the send's longest ramp (`2^22` samples) is refused
  `domain` rather than clamped; a solo's window counts the same way and is reported at the batch's
  first solo record. The same kind 4 is admitted when no following send moves, as before #1224. The decode staging grows by one entry per live send.
- **A delayed send stays mixed.** A follow-muted send that carries a compensation delay mixes zero
  coefficients through its line rather than going inactive (#1217), exactly as an explicit
  `routeMute` does.

### Ruling D1 — solo is not persisted in Session V1

**Solo is monitoring state, not mix state, and no session key carries it.** A session reloads with
every solo bit clear, and an offline or stem render of a session can never come out soloed — an
offline render takes the session with no command stream at all, so a persisted solo bit would
silence stems that the session, read as a document, says are audible.

This does not violate the standing "protocol mutations update the typed session model and must be
snapshot-able" law, because solo deliberately does not mutate the session model — exactly as live
fader, pan, mute and effect-parameter moves already do not. Live-control state is rebuilt from the
session on reload and is never written back.

Persisted solo-safe or monitor-scene semantics, if the product ever wants them, are a future session
monitor-scene concept and not a V1 key. Nothing here forecloses that.

### Metering and observation while soloed

The code is unchanged; the semantics are worth stating because a console user will ask.

The gate applies at the fader, so taps at `input`, `post_input`, `insert_send`,
`insert_return` and `pre_fader` keep reading the **un-gated** signal — input and
pre-fader metering survives a solo, which is console-correct and is what makes gain-riding a
silenced strip possible. Taps at `post_fader` and `post_pan`, and everything downstream of them
(submixes, outputs, the designated master's peak and gain-reduction rows), read the **gated** mix.
A send from a pre-fader tap follows its strip's mute only with `follows_mute: true`, which only a
route into a submix may set (#1218). A pre-fader send without `follows_mute` stays audible under
solo, and only a route into a submix can follow.

A gain-reduction tap on a strip that solo has silenced falls toward zero reduction, because its
effects are seeing silence. That is the true state of that signal path, not an artifact of the
observation surface.

## Live routes (issue #1220)

A send's level, on/off and 2x2 change on the running plan, declicked, without a rebuild. Once the
change settles, the route renders exactly the bits of a plan freshly prepared with the new values.
This is the portable render core. The host-core producer (#1221) and the browser and C ABI
bindings follow it.

- **Which routes are live (D1).** `PreparedGraphPlan::attach_route_controls`, and its sealed
  delegate `PreparedBuiltinsGraphArtifact::attach_route_live_controls`, run after compile and
  before bind. They give every route whose destination is a **submix input** one bounded SPSC queue
  of `RouteControlRecord`s, in canonical route-ID order, and return the producers. Routes into an
  output keep their prepared constants, their single-master fold and their structural edits. A
  second attach is refused. With no attach the plan binds exactly what it bound before.
- **The record (D2).** A record holds `target`, `mute` and `length`:
  - `target` is what `graph_compiler::route_coefficients` returns, the same function the compiler
    binds through;
  - `mute` is set when the gate silences the route, and then `target` must be four `+0.0`;
  - `length` is the ramp in samples, at most `ROUTE_RAMP_LENGTH_MAXIMUM = 2^22`, and `0` is a step
    at the block boundary.
- **State and the law (D3).** Each live route holds an **indexed ramp** (`lane::kernels::IndexedRamp`),
  a `position` and a `mute`.
  - At bind the ramp is settled on the route's prepared gated coefficients, with `position = 0`
    and `mute` set to the gate's `silences()`. An idle live route therefore mixes the prepared
    bits.
  - A record replaces the ramp with
    `IndexedRamp::new(ramp.coefficients_at(position), target, length)` and resets `position` to 0.
    Several records in one drain apply in order, and the last one wins.
  - Frame `f` of a block mixes with `c(k)` at `k = position + f + 1`, and then `position`
    advances by the quantum, saturating at the ramp's length.
- **Why the indexed ramp and not D11.** D11 (the faders, matrices and effects) carries
  `current += step` from frame to frame, so a coefficient's bits depend on its history. Here
  `c(k) = round(round(k * step) + start)` is a pure function of the frame index, and the settled
  value is `target` itself, assigned rather than computed. That has three consequences:
  - the ramp vectorises over frames;
  - its result does not depend on the lane width;
  - a settled route is bit-identical to a fresh plan.

  D11 is unchanged everywhere else.

  The ramp does not overshoot its target before the snap while the step's rounding error is
  relative. When `|target - start| < length * 2^-126`, the step is subnormal (or exactly
  `2^-126`), and its rounding error is absolute, up to `2^-150`.
  - `k * step` can then exceed `target - start` by less than `(length - 1) * 2^-150`.
  - The sum's rounding onto the target's grid can double that. So `c(k)` can pass the target by
    less than `(length - 1) * 2^-149`, and never by more than `2^-128`, which is reached at
    `length = 2^22`. That is inaudible.
  - `c(k)` stays monotone for `k < length`. The snap then assigns `target` exactly, stepping back
    by the overshoot.
- **Activity, once per block (D4).** After its drain, the route op decides whether the route is
  active for the block. It is inactive only when `mute && position >= length && !delayed`. The op
  writes that bit into the route-activity table before the destination's reduction, a later unit
  of the same block, reads it.
  - The block in which a mute ramp ends is mixed whole.
  - A `length == 0` mute is inactive from the block that drained it.
  - An unmute is active in the block that drains it, and ramps from the current coefficients.
  - A plan with a live route always builds the route-activity table.
- **The delayed-route rule.** A live route whose consumer input carries plugin-delay compensation
  is never inactive. Muted, it keeps mixing its zero target into its own buffer, and its consumer
  keeps staging it. The fade reaches the bus whole, `d` samples later and aligned, and the line
  holds only zero-coefficient output when an unmute arrives. Skipping it would cut the fade still
  in its line and later release stale arena samples.
- **The drain (D5).** At the op's start, every block, including while the route is inactive, the
  op pops exactly the records available at entry and applies each in order. Nothing is dropped: a
  record that arrives later is applied in a later block, and a full queue refuses a push and hands
  the record back.
- **Shape (D6).** A live route is `NodeKind::LiveRoute`, never a `GraphNodeBinding`. It never
  folds: the fold planner's metadata answers `has_route_control`, and `plain_route_gains` declines
  such a route, because an epilogue would never drain its queue.
- **Resources (D8).** `route_control_resources` is an upper bound on what the attach adds before
  bind and on what the bound plan retains:
  - each queue's retained payload;
  - the lane box, the plan's binding entry and the render-side owner box, per route;
  - the producer table;
  - the route IDs, twice: the producer's copy and the binding's node-ID copy, which the plan holds
    from attach until bind;
  - the route-activity table, when the compile-time estimate did not already charge it (no
    prepared gate silences). This part is `route_activity_bound_bytes`, an upper bound; every
    other part is the exact size of what it names.

  The next slice admits these bytes against the host's caps.

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
