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
    /// Whether the adapter passes a document beside a command frame, so the session serves and
    /// advertises `SESSION_REPLACE` (`0x0013`, #1386). An adapter with no document transport keeps
    /// the default: the command is then unadvertised and refused `UNAVAILABLE`, never advertised
    /// and refused on every call. The browser turns it on with its transport (#1697).
    const DOCUMENT_TRANSPORT: bool = false;
    /// Whether the session serves the `Observe` edit (#1584 D7a): a transaction's subscriptions are
    /// checked against the session's binding table and queued on the effects' observation queues
    /// by the core. An adapter that keeps the default refuses every `Observe` edit with
    /// `observation.unavailable`: the browser until #1589, because its observe kinds 7 and 8 still
    /// write the same queues, and one queue must never have two writers. Its plans must carry an
    /// observation queue for every declared tap.
    const OBSERVE_EDIT: bool = false;
    /// Whether the session advertises the spectrum commands (#1595 R2): `SPECTRUM_CONFIGURE`,
    /// `SPECTRUM_CAPTURE` and `SPECTRUM_STREAM` with capability bit 16
    /// (`ProviderFeatures::spectrum`).
    /// Every session serves the three; an adapter that keeps the default lists none of them, for a
    /// host that reads spectrum through another surface (the browser until #1596). An adapter that
    /// advertises them also refuses, at boot, a spectrum configuration with a tap no read can
    /// carry within the frame limit (`spectrum.frame`), as `SPECTRUM_CONFIGURE` does (#1595 R6).
    const SPECTRUM: bool = false;
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
    /// Graph, session and plan bytes, every held plan and compiled model included at a swap
    /// (#1398 D4).
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
    /// (Strip, VCA) reach pairs of one session (ruling V-Q3): the bound on one VCA move's work.
    /// Every adapter sets [`host_core::MAXIMUM_VCA_REACH_PAIRS`].
    pub maximum_vca_reach_pairs: u64,
    /// The most a replacement may hold at its peak, summed over the categories: the graph, session
    /// and plan bytes of every plan the control plane holds and of the replacement, every compiled
    /// model, and the source bytes of all those plans, a carried ring counted once (#1290a D3,
    /// #1398 D4). Each category also has its own
    /// ceiling above; an adapter that owns one aggregate budget (the browser's) sets this to what
    /// that budget leaves for them after its own fixed bytes, and one that has none sets
    /// `u64::MAX`.
    pub maximum_replacement_peak_bytes: u64,
}

/// The tail kind word of a tail that terminates after its sample count (#1688 D1).
pub const TAIL_FINITE: u64 = 0;
/// The tail kind word of an unbounded tail, which reports zero samples (#1688 D1).
pub const TAIL_INFINITE: u64 = 1;

/// The two-word encoding of an output tail every host reports: `(kind, samples)` with
/// [`TAIL_FINITE`] and the sample count, or [`TAIL_INFINITE`] and zero. The C ABI's report and
/// the browser's plan-latency record both call it, so they cannot encode a tail differently.
#[must_use]
pub const fn tail_words(tail: TailSamples) -> (u64, u64) {
    match tail {
        TailSamples::Finite(samples) => (TAIL_FINITE, samples),
        TailSamples::Infinite => (TAIL_INFINITE, 0),
    }
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
    /// The reported output tail, the graph's every-peak extent (#1379 H10): what the C ABI's
    /// `tail_samples` and the browser's plan-latency record carry.
    pub output_tail_every_peak: TailSamples,
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
