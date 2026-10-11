//! The adapter boundary: what an adapter tells the control plane, and the plain values it passes.

use super::*;

/// What an adapter supplies to the control plane at the type level (#1309 D4, D5).
///
/// The control plane is [`SessionState<A>`](crate::SessionState). An adapter implements this once.
/// Nothing of it is stored: the values cannot differ between calls, and the session row keeps its
/// size.
pub trait ControlAdapter {
    /// The row the plan resource table stores for each plan epoch: the adapter's own report shape,
    /// built once when a row is inserted, and copied out unchanged by the adapter's query.
    ///
    /// The control plane reads stored rows back for its replacement and live admissions, so every
    /// item that does also requires `PlanResources: From<Self::Row>`, and that round trip must be
    /// lossless.
    type Row: From<PlanResources> + Copy;
    /// The byte sizes of the fixed allocations the adapter keeps per session. They are charged to
    /// the session's fixed rows and largest-allocation fold on every preparation.
    const ADAPTER_ALLOCATIONS: &'static [u64];
}

/// Plain compile limits (#1309 D3): every numeric field of the C ABI's `CompileLimits`, with the
/// same names and types, and the adapter's own retained bytes bounded by
/// `maximum_control_retained_bytes`.
///
/// The adapter validates its own limits struct and converts it once.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ControlLimits {
    /// Frames of each source ring; zero derives the value from the session's rate and quantum.
    pub source_ring_frames: u32,
    /// Automation spans the protocol admits per render block.
    pub maximum_automation_spans_per_block: u32,
    /// Bytes of the canonical session document.
    pub maximum_document_bytes: u64,
    /// Bytes of one diagnostic report.
    pub maximum_diagnostic_bytes: u64,
    /// Tracks of one session.
    pub maximum_tracks: u64,
    /// Sources of one session.
    pub maximum_sources: u64,
    /// Routes of one session.
    pub maximum_routes: u64,
    /// Effect instances of one session.
    pub maximum_effects: u64,
    /// Graph, session and plan bytes, both plans and both compiled models included at a swap.
    pub maximum_graph_session_plus_plan_bytes: u64,
    /// Source ring bytes, payload and overhead.
    pub maximum_source_total_bytes: u64,
    /// Source ring overhead bytes.
    pub maximum_source_overhead_bytes: u64,
    /// Effect scalar state bytes.
    pub maximum_effect_state_bytes: u64,
    /// Effect scalar scratch bytes.
    pub maximum_effect_scratch_bytes: u64,
    /// Builtin retained payload bytes.
    pub maximum_builtin_retained_bytes: u64,
    /// The control plane's own retained bytes, the adapter's fixed allocations included.
    pub maximum_control_retained_bytes: u64,
    /// The largest single named allocation.
    pub maximum_named_allocation_bytes: u64,
    /// Meter streams.
    pub maximum_meter_streams: u64,
    /// Meter items.
    pub maximum_meter_items: u64,
    /// Meter bytes.
    pub maximum_meter_bytes: u64,
    /// Bytes of one control frame.
    pub maximum_control_frame_bytes: u64,
    /// Bytes of the replay cache.
    pub maximum_replay_bytes: u64,
    /// Entries of the replay cache.
    pub maximum_replay_entries: u64,
    /// Submixes of one session; zero means `maximum_tracks` (#1206 D2).
    pub maximum_submixes: u64,
    /// VCA groups of one session; zero means `maximum_tracks` (#1243 D2).
    pub maximum_vcas: u64,
}

/// One plan's frozen resource accounting (#1309 D4): every field of the C ABI's
/// `PlanResourceReport` except its ABI header and reserved words, with the output tail as
/// [`TailSamples`] and the control plane's own retained bytes as `control_retained_bytes`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanResources {
    /// The plan's sample rate.
    pub sample_rate_hz: u32,
    /// The plan's render quantum.
    pub quantum_frames: u32,
    /// Declared sources.
    pub source_count: u64,
    /// Tracks.
    pub track_count: u64,
    /// Output latency in samples.
    pub latency_samples: u64,
    /// Output tail.
    pub output_tail: TailSamples,
    /// Graph, session and plan bytes, the carry program included.
    pub graph_session_plus_plan_bytes: u64,
    /// The plan's incremental graph bytes.
    pub graph_incremental_plan_bytes: u64,
    /// Graph metadata bytes.
    pub graph_metadata_bytes: u64,
    /// Graph delay-line bytes.
    pub graph_delay_bytes: u64,
    /// Effect bank scratch bytes.
    pub effect_bank_scratch_bytes: u64,
    /// Effect bank runtime buffer bytes.
    pub effect_bank_runtime_buffer_bytes: u64,
    /// Effect bank metadata bytes.
    pub effect_bank_metadata_bytes: u64,
    /// Builtin bank bytes.
    pub builtin_bank_bytes: u64,
    /// Builtin bank scratch bytes.
    pub builtin_bank_scratch_bytes: u64,
    /// Source PCM payload bytes, carried rings included.
    pub source_pcm_payload_bytes: u64,
    /// Source overhead bytes, carried rings included.
    pub source_overhead_bytes: u64,
    /// Source total bytes, carried rings included.
    pub source_total_bytes: u64,
    /// Effect scalar state bytes.
    pub effect_scalar_state_bytes: u64,
    /// Effect scalar scratch bytes.
    pub effect_scalar_scratch_bytes: u64,
    /// Builtin processor payload bytes.
    pub builtin_processor_payload_bytes: u64,
    /// Builtin meter payload bytes.
    pub builtin_meter_payload_bytes: u64,
    /// Builtin retained payload bytes.
    pub builtin_retained_payload_bytes: u64,
    /// The control plane's own retained bytes, the adapter's fixed allocations included.
    pub control_retained_bytes: u64,
    /// The largest single named allocation.
    pub largest_named_allocation_bytes: u64,
}
