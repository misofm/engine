# Native effect runtime contract V1

Issue 011 defines semantic Rust runtime interfaces only. The descriptor wire and its C records,
package and artifact bytes, CID identity, persisted state envelopes and state migration were issue
029's `effect-package` crate. #1037 removed that crate under owner rulings R6a ("third-party
effects are out of scope until a new issue reopens them") and R6b ("there is no persisted DSP
state or state migration without a product need"); git history keeps it. None of it was a runtime
identity or an issue-011 gate.

The contract crate's Rust types are deliberately **not** `repr(C)` and it publishes no C header;
descriptors have no C ABI. A second, orphaned header once sat at
`include/miso_engine_effect_contract_v1.h` describing 32-byte ports and 48-byte quality rows that
nothing implemented; issue #95 deleted it and `scripts/check-effect-runtime-policy.sh` keeps it
gone.

Factories validate static descriptors and allocate/design all processor resources off render.
Prepared metadata fixes sample rate, quantum, quality, bypass, link mode, ports, exact integer
latency, tail, state-section sizes, scratch bytes, and automation capacity. The compiler caches
that metadata; graph/PDC consumers never query a live processor. The semantic `EffectProgramKey`
contains these fields directly and is not a digest or persistence identity.

The callback receives disjoint in-place planar L/R slices and optional planar sidechain slices.
It performs no allocation/free, synchronization, I/O, network, logging, syscall, dynamic loading,
feature detection, panic, callbacks, or unbounded work.

**An effect classifies no individual sample** (master plan #83 decision D7). The audited V1 text
froze the opposite — "nonfinite and subnormal input, sidechain, internal, and output values become
`+0.0`", with saturating per-sample counters — and that is withdrawn by issue #95 finding F1: it
cost four to six scalar classify-and-branch sequences per frame per effect, it prevented the frame
loop from vectorising, and both production callers discarded the counters. The replacement is
three separate mechanisms, each where its hazard is:

* **Denormals** — `flush(x) = andnot(|x| < 1e-20, x)`, applied to each recursive state word once
  per sample *inside* the kernel (`lane::flush`). A subnormal *input* sample is no
  longer replaced by zero; it renders, and it cannot reach a recurrence because the flush band
  strictly contains the subnormal band.
* **Divergence** — output finiteness is checked **once per block per bank** with one vector
  compare, `x == x` and `|x| < 1e30` (`effect_runtime::bank::check_block`). A failing
  block zeroes its output, resets that effect's state to prepared defaults, and increments a
  **block** counter. The contract's report counts blocks, never samples. In a bank, every launch
  effect recovers only the failing lane and reports it alone (#1089-#1092), so a lane's fault never
  moves a bank-mate's bits. The one exception is the multiband, whose bank still zeroes and resets
  every lane; it therefore keeps a prepared session bypass (#1100).
* **Input sanitisation** — once per track per block at the track input stage, never inside an
  effect.

Signed finite zero is retained on every non-recursive path. A prepared bypass is an immutable
configuration and outputs the dry input delayed by exactly the declared latency. Every bank reads
one bypass flag for all its lanes, so the prepared flag stays in `EffectProgramKey`. A session's
bypass is not prepared (issue #1087): `effect-compiler` prepares an effect that can bank
enabled and carries the bypass as per-lane state to the rack's latency-preserving shunt
(`BypassShunt`), which copies the same delayed dry words into exactly the bypassed lanes, so a
bypassed track keeps its bank. Two effects keep their session bypass as a prepared one
(`effect_compiler::lowers_session_bypass`): the delay, which never banks, and the multiband
compressor, whose whole-bank D7 recovery would let a bypassed lane silence its bank-mates
(issue #1100).

Class-A identity, the same bits on every lane width and target, treats every NaN as one value
(owner decision 10, #1065): tests fold each NaN to `0x7FC00000` through `dsp_reference::class_a`
before comparing or hashing, the engine does not canonicalize NaNs at render, and finite input
must still render finite output, with each effect's documented NaN behaviour unchanged.

## Tail and exact rest

Decision 15 D15-4(b) (issue #1329) gives every node's tail one meaning. For an input of peak `P`
that is zero from sample `N` on, under any control history the node admits before `N` (a ramp may
be in flight at `N`) and with no control event at or after it, with `eps = 10^(-144/20)`, each
node states three values, all counted beyond its latency, all certified upper bounds computed on
the control thread at preparation and never pinned:

* **`TailSamples` -- `T_decay`, the tail.** `|y[n]| < P * eps` for every `n >= N + latency + T`,
  for every input peak `P >= P*`. `P*`, the node's **flush floor**, is the smallest peak for which
  the `f32` kernel's absolute deviation near the per-word state flush (`lane::FLUSH_EPS`) fits the
  `-144 dB` budget; below it the output is no longer relative to `P`. PDC composes `T_decay` along
  a path, and every tail report uses it (the C ABI and browser reports, #1261, #1262). `Infinite`
  states no bound and remains for nodes whose bound has not been derived (#1378 retires it).
* **The tail over every peak -- `T_rest = max(T_decay, R(P*))`, named `tail_every_peak`** (a
  `TailSamples`, beside `tail` wherever `tail` is stated: `builtins::InputSectionBound`, which
  `builtins-compiler`'s prepared session keeps per strip beside its tail,
  `PreparedBuiltinsSession::input_bounds`). From `N + latency + T_rest` on the output is below `P * eps` for
  `P >= P*` and exactly `+0.0` or `-0.0` for `P < P*`. `R(P*)` includes the joint-flush arming
  window `N_SILENCE` (#1328 A9), so `T_rest >= N_SILENCE` for every enabled filter section. It is
  the exact-zero branch for low peaks only.
* **`RestSamples` -- the exact-rest bound `R`.** With input peak at most +24 dBFS
  (`peak_plus_24_dbfs`) or any input the input sanitizer passes, below `1e30`
  (`any_sanitized_input`), from `N + latency + R` on every output is `+0.0` or `-0.0` and every
  signal-state word equals, under `f32` `==`, the node's rest state `Z` (a fixed point of the
  zero-input step; for the builtin input section the reset state, every integrator `+0.0`). Silence
  skipping (#1107) uses this bound: it holds for every input up to its stated peak.

Gain-only parts (trim, polarity, fader, mute, matrix) state `0` for all three. An absolute output
floor in place of the exact-zero branch is refused (Amendment 3, G2): it would make the tail a
fixed-level one, which #1328's A8 and A9 removed. Each native effect's bounds are its own slice
(#1372-#1376), carried in its prepared metadata by #1377; until then an effect reports `Infinite`.
The builtin input section's derivation is `docs/derivations/1329-input-section-tail-and-rest.md`.

## Parameters and automation

Persisted parameter values use descriptor-declared exact-decimal lattices. This follows the
binding [#239 section B ruling](https://github.com/misofm/engine/issues/239#issuecomment-5461507633):
arithmetic rows contain `min + k*step` interiors; logarithmic hertz rows use
`min * 2^(k*cents/1200)`; every other logarithmic row uses `min * r^k`, where `r` is that row's
exact-decimal ratio. Every continuous lattice additionally contains the declared minimum,
maximum, and default as intrinsic exact members. Thus a round maximum and the declared
`0.70710677` Butterworth-Q default are never made unreachable by a regular geometric interior.
Boolean and enumeration rows are index lattices: the step is identically one over the ordinals,
which are what the persist plane carries. The DOCUMENT, however, spells an enumeration as its
declared choice value, so the choice values -- not the ordinals -- are the canonical decimal
renderings a persisted value is matched against. Conflating the two would refuse the last choice
of every enumeration and silently relabel the rest.

A declared bound is a member because it was declared. A rate-keyed cutoff ceiling is not
declared -- it is the representable clamp for the prepared rate -- so `disabledOrRateKeyedHertz`
keeps S1's original semantics: the top of its lattice is the greatest generated point at or below
that rate's clamp, and the clamp itself need not be a legal value.

Because a bound is a member outright, the row's pinned precision must be able to spell it. A
declaration whose minimum, maximum or default does not survive its own canonical rendering
bit-for-bit is refused at descriptor validation rather than silently rounded: the shipped delay
`damping` row declares a maximum of `0.995`, which is why that row overrides the linear class
default to three decimals.

The complete default domain table is:

| domain / mapping | unit | base step | precision | ladder `xs/sm/md/lg/xl` |
|---|---|---:|---:|---|
| boolean or enumeration / stepped | any | 1 index | 0 | 1/3/5/10/30 |
| continuous / linear or exponential | dB | 0.1 | 1 | 1/3/5/10/30 |
| continuous / linear or exponential | Hz | 0.001 | 3 | 1/3/5/10/30 |
| continuous / linear or exponential | ms | 0.1 | 1 | 1/3/5/10/30 |
| continuous / linear or exponential | samples | 1 | 0 | 1/3/5/10/30 |
| continuous / linear or exponential | linear | 0.01 | 2 | 1/3/5/10/30 |
| continuous / linear or exponential | ratio/Q | 0.1 | 1 | 1/3/5/10/30 |
| continuous / logarithmic | Hz | 20 cents | 3 | 1/3/5/10/30 |
| continuous / logarithmic | ms | ratio 1.02 | 3 | 1/3/5/10/30 |
| continuous / logarithmic | ratio/Q | ratio 1.02 | 8 | 1/3/5/10/30 |

Every descriptor carries its own declaration and may override the table. The shipped fader is
the current override: base `0.1 dB`, precision 1, ladder `1/5/10/30/60`. Builtin rows are fully
declared as follows; rate-keyed cutoff maximum is the selected launch-rate representable clamp,
and disabled zero has the reserved `u32::MAX` index outside the enabled hertz lattice. `pan`
is a block target like the matrix rows whose coefficients it derives, so it joins the live set.

| stable ID | builtin | scope | domain | default | step / precision | ladder |
|---:|---|---|---|---:|---|---|
| 1 | polarity_invert | per lane | bool index 0..1 | 0 | 1 index / 0 | 1/3/5/10/30 |
| 2 | trim_db | per lane | -144..24 dB | 0 | 0.1 / 1 | 1/3/5/10/30 |
| 3 | hpf_hz | per lane | disabled 0 or 10..rate clamp | 0 | 20 cents / 3 | 1/3/5/10/30 |
| 4 | lpf_hz | per lane | disabled 0 or 10..rate clamp | 0 | 20 cents / 3 | 1/3/5/10/30 |
| 5 | fader_db | per lane | -144..24 dB | 0 | 0.1 / 1 | 1/5/10/30/60 |
| 6 | mute | per lane | bool index 0..1 | 0 | 1 index / 0 | 1/3/5/10/30 |
| 7 | matrix_ll | matrix shared | -1..1 linear | 1 | 0.01 / 2 | 1/3/5/10/30 |
| 8 | matrix_lr | matrix shared | -1..1 linear | 0 | 0.01 / 2 | 1/3/5/10/30 |
| 9 | matrix_rl | matrix shared | -1..1 linear | 0 | 0.01 / 2 | 1/3/5/10/30 |
| 10 | matrix_rr | matrix shared | -1..1 linear | 1 | 0.01 / 2 | 1/3/5/10/30 |
| 11 | delay_samples | per lane | 0..48000 samples | 0 | 1 / 0 | 1/3/5/10/30 |
| 12 | pan | per lane | -1..1 linear | 0 | 0.01 / 2 | 1/3/5/10/30 |

`pan` remains persisted pan intent. Matrix descriptors remain the authority only for documents
that explicitly store matrix coefficients.

Linear mapping is `min + x(max-min)`, logarithmic mapping is `min(max/min)^x`, and exponential
mapping is `min + (max-min)x^2`; exact endpoints are assigned explicitly. Stepped mapping selects
the closest legal value and resolves ties toward the lower value. Inputs outside finite `[0,1]`
and invalid domain values reject rather than clamp.

Smoothing length is `smoothing_samples` from the parameter descriptor; it is binding, and no
effect may substitute a literal.

For `N` smoothing updates, **linear precomputes its increment once, at the moment the target
changes** (master plan decision D11): `step = (target - current) / N`, then
`current = ramp_toward(current, step, target)` per update, and the exact target is assigned on
update `N`. `ramp_toward` (`lane::kernels::ramp_toward`, issues #1408 and #1409) is
`current + step` held inside `[min(current, target), max(current, target)]`, so no ramp word ever
passes its target: without it, accumulated rounding carries a 64-update ramp up to about 30 ulps
past its target before the snap, which can leave the parameter's domain. Every word therefore lies
between the value at the event (or at a restore) and the target, for any finite step; an in-range
word keeps the unadjusted sum's bits, signed zeros included, and a NaN step propagates. There is
no per-sample division anywhere in the engine. The audited rule — "linear adds
`(target-current)/remaining`" — is withdrawn by issue #95 finding F2: it cost one integer-to-float
convert and one `fdiv` per parameter per lane per sample for the whole length of every ramp.
One-pole-99 likewise precomputes `a = exp(ln(0.01)/N)` and `1-a` once, then
`y = a*y_previous + (1-a)*target`, and assigns the exact target on update `N`. `None` assigns immediately. A new target restarts from the current value.

`effect_runtime::ramp::LinearRamp` is the one render-path implementation;
`effect_contract::ParameterSmoother` states the same law for the control plane, and
`effect-runtime/tests/contract_ramp_identity.rs` proves the two agree bit for bit.

V1 runtime automation is `Point` spans whose `start_sample` equals the block's first sample,
validated off render by `validate_automation_block`; an effect trusts the slice it is given.
`Step`, `Linear` and `Exponential` spans and `AutomationRate::Sample` are descriptor and protocol
vocabulary — `valid_runtime_span` and `automation_segment_value` define their meaning for the
control plane and for the conformance reference mock — whose sample-accurate render-path delivery
is a later protocol capability. Malformed render spans are ignored, counted once, and do not
change the last valid target.

Effects may opt into `NativeEffectTargetPreparation` (#807). Their control owner validates
all original semantic edits against its accepted configuration and prepares fixed targets off
render. Whole-batch admission checks targets, revisions and every destination's capacity before
publishing; only successful admission commits the candidate. Parametric EQ uses this route for
its existing numeric controls and dedicated HPF/LPF enable, frequency and Q. Its scalar and bank
processors refuse raw semantic spans and apply only prepared targets. Other effects retain their
existing point-span route.

For EQ, acknowledgement sample A starts the transition from the current coefficients. Sample A
uses the old current words; the exact target is used at A+64. A response captured after A describes
the accepted target, not the instantaneous coefficients during that transition. Reset and mono
transitions use cached words and do not invoke the coefficient designer.

## State and lane isolation

State is three exact caller buffers: common, left, and right. Snapshot is deterministic and
all-or-none. Restore accepts only the current nonzero `state_layout_version` and exact prepared
sizes. Nothing persists a payload (R6b; the compiler's persisted envelope went with
`effect-package` in #1037). The payload calls -- `snapshot_state_payload`/`restore_state_payload`
and a bank's per-track pair -- are render-safe: they allocate, free, lock and log nothing and run
in time bounded by the prepared state, because the plan-swap carry (#1269) is to call them in the
swap block to move a lane's state into a rebuilt plan, and a restored lane continues bit for bit like
the lane it was taken from, mid-ramp included (#1278).

**A version or length word inside the payload outranks the caller's claim.** The
`state_layout_version` argument of `restore_state_payload` arrives out of band, from the
descriptor the caller *believes* wrote the bytes, and is trustworthy only while caller and writer
are the same build — which a persisted session is not. Where a payload carries a header, the
restore compares the two and rejects on the payload's own evidence; the argument never overrides
the bytes. Where a payload carries none, the argument is checked against the descriptor's
`state_layout_version` and the prepared sizes. The header is two little-endian words at the front
of the common section — layout version, then the effect's data word count — implemented once in
`effect_runtime::state_payload`. Adopting it moves `maximum_state.common_bytes` from 0
to 8, which is a descriptor identity change: the crates that carry a header today adopted it with
their layouts, the rest adopt one in a coordinated identity change. Before launch a layout change
does not bump `state_layout_version`: a genuine version's prelaunch identity is `1` (AGENTS.md,
the version-suffix rule). No payload is persisted, so only a payload of the same build can arrive,
and the exact prepared section lengths refuse one whose layout has another length (#1278 grew two
effects' layouts at version `1`). The **rule** above is frozen now for all of them.

`scratch_fixed_bytes` is an **admission ceiling an effect reserves, not a measurement of what it
uses**. A host admits a preparation by proving it can supply
`scratch_fixed_bytes + scratch_bytes_per_frame x quantum`; an effect that uses less is conforming.
A declared ceiling may be tightened toward measured use, but that moves canonical descriptor bytes
and is an effect-identity change, never a contract cleanup. Per-lane audio, delay, filter,
envelope, smoother, and dual-mono detector state stays in the corresponding lane section. Only
shared configuration and an explicitly linked detector may be common. Full reset restores
prepared defaults; discontinuity reset keeps targets but clears histories and active spans.

## Stable diagnostics

Descriptor errors use deterministic dotted codes documented by
`DescriptorDiagnosticCode::as_str`. Session preparation additionally freezes:

```text
effect.native.unavailable
effect.descriptor.invalid
effect.quality.unsupported
effect.link_mode.unsupported
effect.parameter.unknown
effect.parameter.unit_mismatch
effect.parameter.domain
effect.parameter.channel
effect.parameter.duplicate_channel
effect.sidechain.missing
effect.sidechain.unknown_port
effect.sidechain.unexpected
effect.resource.limit
effect.prepare.failed
effect.metadata.mismatch
effect.state.invalid
effect.third_party.unavailable_at_launch
effect.automation.rate
```

`effect.descriptor.invalid` is raised only by `NativeEffectRegistry::new`, once per effect type
when its factory enters the registry; `validate_prepare_request` no longer re-validates the
descriptor per prepared instance (issue #1330), and because session preparation builds the
registry before preparing any effect, a host observes the same refusal as before.

`effect.automation.rate` refuses a stored automation whose `inserts` or `console` target parameter
is not `automatable` or whose `automation_rate` is not `Block` (decision 15 E1, issue #1335). Its
path is `$.automation[id=<automation id>].target.parameter_id`.

## Evidence contract

The correct dual-accumulator/three-sample-delay mock has separate L/R delay, accumulator,
automation, and payload state. Its enabled and bypass impulse index is exactly three. Every
declared quality must have exactly the 44,100, 48,000, 88,200, and 96,000 Hz rows; a row at any
other rate, including the former extended research rates 176,400, 192,000, 352,800, and
384,000 Hz, refuses the descriptor with `Quality` (owner ruling R5, #1036). Conformance launch
gates cover every declared row. It checks
every declared quality/link mode, enabled/bypass, metadata immutability,
D7 output-block bounds under poisoned input and sidechain, deterministic state restore, and lane
isolation. Separate faulty mocks exercise
allocation/free/lock/file/network/log/syscall hooks, panic, shared lane state, changing
latency/tail/resources, bypass latency, malformed automation, NaN propagation, partial or
nondeterministic snapshot, and rejected restore.

The harness is built from the descriptor, not from the reference mock: the prepare request uses
`default_initial_values`, the ports come from the descriptor's own sidechain declaration (or
`PreparedSidechainPort::None`), the impulse probe renders as many blocks as the declared latency
needs, and lane isolation is compared against a silence-rendered control instance in dual-mono
only — a linked detector is exactly what `Maximum` and `Average` declare. Launch effects run it:
`compressor` (zero latency, linked detector, recursive state that settles on
silence) and `parametric-eq` (zero latency, header-carrying payload) each have a
`tests/conformance.rs` asserting `report.launch_gates.failures.is_empty()`. A contract whose only
conforming implementation is its own mock is not evidence. Deterministic tests execute at least 10,000
descriptor, span, and session mutations. The release audit performs 1,000,000 128-frame calls
under allocation/deallocation hooks and native syscall tracing.

The bounded benchmark is descriptive, runs exactly two internal rounds after all nonbenchmark
gates pass, and has no hardware-independent timing threshold. Production effects 012–021 must add
their own equations, coefficient/stability bounds, latency/tail, fixtures, objective comparisons,
benchmarks, and documented listening evidence.
