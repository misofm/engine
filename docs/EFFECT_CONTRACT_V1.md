# Native effect runtime contract V1

Issue 011 defines semantic Rust runtime interfaces only. Descriptor wire/C records, package and
artifact bytes, CID identity, persisted state envelopes, and migration belong to issue 029,
**Canonical effect interchange, state migration, and CID package identity**. They are neither
runtime identities nor issue-011 gates.

Factories validate static descriptors and allocate/design all processor resources off render.
Prepared metadata fixes sample rate, quantum, quality, bypass, link mode, ports, exact integer
latency, tail, state-section sizes, scratch bytes, and automation capacity. The compiler caches
that metadata; graph/PDC consumers never query a live processor. The semantic `EffectProgramKeyV1`
contains these fields directly and is not a digest or persistence identity.

The callback receives disjoint in-place planar L/R slices and optional planar sidechain slices.
It performs no allocation/free, synchronization, I/O, network, logging, syscall, dynamic loading,
feature detection, panic, callbacks, or unbounded work. Nonfinite and subnormal input, sidechain,
internal, and output values become `+0.0`; signed finite zero is retained. `ProcessReport` uses
saturating counters. Bypass is an immutable prepared configuration and delays dry input by the same
declared latency as enabled processing.

## Parameters and automation

Linear mapping is `min + x(max-min)`, logarithmic mapping is `min(max/min)^x`, and exponential
mapping is `min + (max-min)x^2`; exact endpoints are assigned explicitly. Stepped mapping selects
the closest legal value and resolves ties toward the lower value. Inputs outside finite `[0,1]`
and invalid domain values reject rather than clamp.

For `N` smoothing updates, linear adds `(target-current)/remaining` and assigns the exact target
on update `N`. One-pole-99 uses `a = exp(ln(0.01)/N)` and
`y = a*y_previous + (1-a)*target`, then assigns the exact target on update `N`. A new target
restarts from the current value. Linear/exponential spans bypass the smoother while active and
assign their exact endpoint at `end_sample`. Malformed render spans are ignored, counted once, and
do not change the last valid target.

## State and lane isolation

State is three exact caller buffers: common, left, and right. Snapshot is deterministic and
all-or-none. Restore accepts only the current nonzero `state_layout_version` and exact prepared
sizes; the compiler restores only into an unpublished temporary. Per-lane audio, delay, filter,
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
automation, and payload state. Its enabled and bypass impulse index is exactly three. Conformance
checks all eight rates, every declared quality/link mode, enabled/bypass, metadata immutability,
sanitization, deterministic state restore, and lane isolation. Separate faulty mocks exercise
allocation/free/lock/file/network/log/syscall hooks, panic, shared lane state, changing
latency/tail/resources, bypass latency, malformed automation, NaN propagation, partial or
nondeterministic snapshot, and rejected restore. Deterministic tests execute at least 10,000
descriptor, span, and session mutations. The release audit performs 1,000,000 128-frame calls
under allocation/deallocation hooks and native syscall tracing.

The bounded benchmark is descriptive, runs exactly two internal rounds after all nonbenchmark
gates pass, and has no hardware-independent timing threshold. Production effects 012–021 must add
their own equations, coefficient/stability bounds, latency/tail, fixtures, objective comparisons,
benchmarks, and documented listening evidence.
