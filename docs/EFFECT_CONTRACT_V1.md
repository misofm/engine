# Native effect runtime contract V1

Issue 011 defines semantic Rust runtime interfaces only. Descriptor wire/C records, package and
artifact bytes, CID identity, persisted state envelopes, and migration belong to issue 029,
**Canonical effect interchange, state migration, and CID package identity**. They are neither
runtime identities nor issue-011 gates.

The contract crate's Rust types are deliberately **not** `repr(C)` and it publishes no C header.
The only C ABI for descriptors is
`crates/miso-engine-effect-package/include/miso_engine_effect_descriptor_v1.h` (80/24/64/16-byte
records, asserted in `effect-package`). A second, orphaned header once sat at
`include/miso_engine_effect_contract_v1.h` describing 32-byte ports and 48-byte quality rows that
nothing implemented; issue #95 deleted it and `scripts/check-effect-runtime-policy.sh` keeps it
gone.

Factories validate static descriptors and allocate/design all processor resources off render.
Prepared metadata fixes sample rate, quantum, quality, bypass, link mode, ports, exact integer
latency, tail, state-section sizes, scratch bytes, and automation capacity. The compiler caches
that metadata; graph/PDC consumers never query a live processor. The semantic `EffectProgramKeyV1`
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
  per sample *inside* the kernel (`miso_engine_lane::flush`). A subnormal *input* sample is no
  longer replaced by zero; it renders, and it cannot reach a recurrence because the flush band
  strictly contains the subnormal band.
* **Divergence** — output finiteness is checked **once per block per bank** with one vector
  compare, `x == x` and `|x| < 1e30` (`miso_engine_effect_runtime::bank::check_block`). A failing
  block zeroes its output, resets that effect's state to prepared defaults, and increments a
  **block** counter. The contract's report counts blocks, never samples.
* **Input sanitisation** — once per track per block at the track input stage, never inside an
  effect.

Signed finite zero is retained on every non-recursive path. Bypass is an immutable prepared
configuration, is **not** part of `EffectProgramKeyV1`, and outputs the dry input delayed by
exactly the declared latency.

## Parameters and automation

Linear mapping is `min + x(max-min)`, logarithmic mapping is `min(max/min)^x`, and exponential
mapping is `min + (max-min)x^2`; exact endpoints are assigned explicitly. Stepped mapping selects
the closest legal value and resolves ties toward the lower value. Inputs outside finite `[0,1]`
and invalid domain values reject rather than clamp.

Smoothing length is `smoothing_samples` from the parameter descriptor; it is binding, and no
effect may substitute a literal.

For `N` smoothing updates, **linear precomputes its increment once, at the moment the target
changes** (master plan decision D11): `step = (target - current) / N`, then `current += step` per
update, and the exact target is assigned on update `N`. There is no per-sample division anywhere
in the engine. The audited rule — "linear adds `(target-current)/remaining`" — is withdrawn by
issue #95 finding F2: it cost one integer-to-float convert and one `fdiv` per parameter per lane
per sample for the whole length of every ramp. One-pole-99 likewise precomputes `a =
exp(ln(0.01)/N)` and `1-a` once, then `y = a*y_previous + (1-a)*target`, and assigns the exact
target on update `N`. `None` assigns immediately. A new target restarts from the current value.

`miso_engine_effect_runtime::ramp::LinearRamp` is the one render-path implementation;
`miso_engine_effect_contract::ParameterSmoother` states the same law for the control plane, and
`miso-engine-effect-runtime/tests/contract_ramp_identity.rs` proves the two agree bit for bit.

V1 runtime automation is `Point` spans whose `start_sample` equals the block's first sample,
validated off render by `validate_automation_block`; an effect trusts the slice it is given.
`Step`, `Linear` and `Exponential` spans and `AutomationRate::Sample` are descriptor and protocol
vocabulary — `valid_runtime_span` and `automation_segment_value` define their meaning for the
control plane and for the conformance reference mock — whose sample-accurate render-path delivery
is a later protocol capability. Malformed render spans are ignored, counted once, and do not
change the last valid target.

### Named nudge sizes (issue #127)

Every parameter that can express one declares a `NudgeLadderV1`: an `xs` rung in the parameter's
own unit, plus a ratio class that derives `sm`, `md`, `lg` and `xl` from it. `Human` -- the class
every launch parameter declares -- multiplies `xs` by `{1, 3, 5, 10, 30}`; `Wide` multiplies by
`{1, 4, 16, 64, 256}` and exists so that adopting a coarse-to-fine search ladder later is a
per-parameter edit rather than a vocabulary change. There are no per-frontend step preferences:
one vocabulary everywhere.

The declared `xs` is measured in a `NudgeStepUnitV1`, and each unit is legal on exactly one
mapping, because that is the mapping whose arithmetic gives the unit its meaning:

| step unit | mapping | meaning |
|---|---|---|
| `Absolute` | `Linear` | `xs` units of the parameter's own unit |
| `Cents` | `Logarithmic` | multiply by `2^(xs/1200)` |
| `Percent` | `Logarithmic` | multiply by `1 + xs/100` |
| `Steps` | `Stepped` (enumeration) | advance `xs` whole choices |

A ladder is *resolved* into the mapping's normalized `[0,1]` domain, where the arithmetic is exact
at both endpoints and a logarithmic parameter gets equal-ratio stepping out of the mapping itself
rather than out of a per-decade banding table. A nudge rounds the current position to the nearest
multiple of the resolved `xs` and then moves a whole number of rungs from there:

```text
k  = round(x / xs)
x' = clamp((k + count * multiplier) * xs, 0, 1)
```

Two consequences, both intended. Nudged values land on a fixed declared grid, so a frequency nudge
produces the same handful of values every session instead of an endless supply of `1005.79 Hz`.
And from any grid point the operation is exactly reversible: `+1 * size` then `-1 * size` restores
the starting bits. The *first* nudge from an arbitrary value snaps by at most half an `xs` rung,
and at a domain edge the clamp is one-way; both are documented asymmetries, not rounding drift.

`validate_descriptor_v1` enforces three rules, each with its own diagnostic code:

| code | rule |
|---|---|
| `effect.descriptor.nudge_step` | `xs` is finite, strictly positive, not `-0.0`, and a whole number for a `Steps` ladder |
| `effect.descriptor.nudge_domain` | the step unit fits the mapping, the resolved `xs` is inside `(0,1]`, a continuous parameter's `xl` rung does not cross its whole domain, and a `Boolean` parameter declares no ladder at all |
| `effect.descriptor.nudge_order` | the five resolved rungs strictly ascend |

A stepped parameter is exempt from the `xl` half of the domain rule on purpose: `lg` and `xl` are
ten and thirty choices, they are meant to run off the end of a six-choice enumeration, and the
clamp is exact when they do.

Class defaults are keyed by `(unit, mapping)` and anchored at the just-noticeable difference for
that kind of quantity -- 0.5 dB for a level, 20 cents for a frequency, 5 % for a time constant,
2.5 % for a ratio, 0.01 for a normalized control. A class is a starting point, never a ruling:
`default_nudge_ladder_v1` is the table and every effect may override per parameter. The launch set
overrides in three places (`miso.delay`'s `delay time`, `miso.gate-expander`'s `hold` and the EQ's
four `shelf-slope` parameters), each with its reason written at the override and listed in
`effect-compiler/tests/nudge_launch_set.rs`.

Resolution is memoized at registry construction (`NativeEffectRegistry::nudge_ladders`) and
allocates nothing anywhere.

Two kinds of parameter declare no ladder, and nothing else may: a `Boolean` domain has nothing
between its two values, and an `Exponential` mapping has no constant-unit step. No launch parameter
uses an exponential mapping. Builtin parameters (trim, cutoffs, fader, pan/matrix) are a separate
descriptor path and do not carry ladders yet; the metadata JSON marks every one of them `null`.

## State and lane isolation

State is three exact caller buffers: common, left, and right. Snapshot is deterministic and
all-or-none. Restore accepts only the current nonzero `state_layout_version` and exact prepared
sizes; the compiler restores only into an unpublished temporary.

**A version or length word inside the payload outranks the caller's claim.** The
`state_layout_version` argument of `restore_state_payload` arrives out of band, from the
descriptor the caller *believes* wrote the bytes, and is trustworthy only while caller and writer
are the same build — which a persisted session is not. Where a payload carries a header, the
restore compares the two and rejects on the payload's own evidence; the argument never overrides
the bytes. Where a payload carries none, the argument is checked against the descriptor's
`state_layout_version` and the prepared sizes. The header is two little-endian words at the front
of the common section — layout version, then the effect's data word count — implemented once in
`miso_engine_effect_runtime::state_payload`. Adopting it moves `maximum_state.common_bytes` from 0
to 8, which is a canonical descriptor byte and an effect CID, so adoption travels with a
`state_layout_version` bump (decision W2-D2): the crates that had to bump anyway carry a header
today, the rest adopt one in a coordinated identity change. The **rule** above is frozen now for
all of them.

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
effect.descriptor.nudge_step
effect.descriptor.nudge_domain
effect.descriptor.nudge_order
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
```

## Evidence contract

The correct dual-accumulator/three-sample-delay mock has separate L/R delay, accumulator,
automation, and payload state. Its enabled and bypass impulse index is exactly three. Every
declared quality must have 44,100, 48,000, 88,200, and 96,000 Hz rows; unique ordered rows for
176,400, 192,000, 352,800, and 384,000 Hz are optional compatibility evidence only. Conformance
launch gates cover the first four rates and report optional extended rows separately; neither
descriptor rows nor probes for the latter four create engine, host, or release support. It checks
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
`miso-engine-compressor` (882 samples of lookahead, linked detector, a ring index that advances on
silence) and `miso-engine-parametric-eq` (zero latency, header-carrying payload) each have a
`tests/conformance.rs` asserting `report.launch_gates.failures.is_empty()`. A contract whose only
conforming implementation is its own mock is not evidence. Deterministic tests execute at least 10,000
descriptor, span, and session mutations. The release audit performs 1,000,000 128-frame calls
under allocation/deallocation hooks and native syscall tracing.

The bounded benchmark is descriptive, runs exactly two internal rounds after all nonbenchmark
gates pass, and has no hardware-independent timing threshold. Production effects 012–021 must add
their own equations, coefficient/stability bounds, latency/tail, fixtures, objective comparisons,
benchmarks, and documented listening evidence.
