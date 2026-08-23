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
