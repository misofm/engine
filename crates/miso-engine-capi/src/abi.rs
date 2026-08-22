//! Fixed-width representations shared by the C header and FFI implementation.

use core::{cell::RefCell, marker::PhantomData};

use crate::runtime::{FixedBytes, PlanState, SessionState};

/// Frozen ABI V1 version (`major << 16 | minor`).
pub const ABI_VERSION: u32 = 0x0001_0000;

/// The operation completed successfully.
pub const RESULT_OK: u32 = 0;
/// An argument was null, malformed, out of range, or contained nonzero reserved data.
pub const RESULT_INVALID_ARGUMENT: u32 = 1;
/// The caller requested an incompatible ABI version.
pub const RESULT_ABI_MISMATCH: u32 = 2;
/// A live handle has a different opaque handle kind than the entrypoint requires.
pub const RESULT_WRONG_HANDLE: u32 = 3;
/// A caller output buffer cannot hold the complete value.
pub const RESULT_BUFFER_TOO_SMALL: u32 = 4;
/// Transactional session compilation rejected the input.
pub const RESULT_COMPILE_REJECTED: u32 = 5;
/// A bounded queue cannot currently accept the operation.
pub const RESULT_BACKPRESSURE: u32 = 6;
/// The requested operation is not supported by this ABI implementation.
pub const RESULT_UNSUPPORTED: u32 = 7;
/// Render validation rejected the call without advancing state.
pub const RESULT_RENDER_REJECTED: u32 = 8;
/// A panic or other internal failure was contained at the ABI boundary.
pub const RESULT_INTERNAL: u32 = 255;

/// A plan tail terminates after `tail_samples`.
pub const TAIL_FINITE: u64 = 0;
/// A plan tail is unbounded and reports zero `tail_samples`.
pub const TAIL_INFINITE: u64 = 1;

/// Launch support for 44,100 Hz.
pub const RATE_44_100: u64 = 1 << 0;
/// Launch support for 48,000 Hz.
pub const RATE_48_000: u64 = 1 << 1;
/// Launch support for 88,200 Hz.
pub const RATE_88_200: u64 = 1 << 2;
/// Launch support for 96,000 Hz.
pub const RATE_96_000: u64 = 1 << 3;
/// All exact launch sample-rate capability bits.
pub const EXACT_LAUNCH_RATE_MASK: u64 = RATE_44_100 | RATE_48_000 | RATE_88_200 | RATE_96_000;

/// Immutable-session compilation capability.
pub const FEATURE_IMMUTABLE_SESSION: u64 = 1 << 0;
/// Host-fed planar source capability.
pub const FEATURE_HOST_PLANAR_SOURCE: u64 = 1 << 1;
/// Typed source-seek capability.
pub const FEATURE_SOURCE_SEEK: u64 = 1 << 2;
/// Caller-owned planar stereo render capability.
pub const FEATURE_PLANAR_STEREO_RENDER: u64 = 1 << 3;
/// Issue-005 capability-command support.
pub const FEATURE_CAPABILITY_COMMAND: u64 = 1 << 4;
/// All ABI V1 feature capability bits.
pub const FEATURE_MASK: u64 = FEATURE_IMMUTABLE_SESSION
    | FEATURE_HOST_PLANAR_SOURCE
    | FEATURE_SOURCE_SEEK
    | FEATURE_PLANAR_STEREO_RENDER
    | FEATURE_CAPABILITY_COMMAND;

/// Engine creation configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct EngineConfig {
    /// Must equal [`ENGINE_CONFIG_SIZE`].
    pub struct_size: u32,
    /// Must equal [`ABI_VERSION`].
    pub abi_version: u32,
    /// Must be zero in ABI V1.
    pub reserved: [u64; 4],
}

/// Transactional compilation resource ceilings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct CompileLimits {
    /// Must equal [`COMPILE_LIMITS_SIZE`].
    pub struct_size: u32,
    /// Preallocated frames in every host-fed source ring.
    pub source_ring_frames: u32,
    /// Maximum accepted automation spans in one block.
    pub maximum_automation_spans_per_block: u32,
    /// Must be zero in ABI V1.
    pub reserved0: u32,
    /// Maximum borrowed strict-TOML input bytes.
    pub maximum_toml_bytes: u64,
    /// Maximum retained compile-diagnostic bytes.
    pub maximum_diagnostic_bytes: u64,
    /// Maximum session track count.
    pub maximum_tracks: u64,
    /// Maximum session source count.
    pub maximum_sources: u64,
    /// Maximum session route count.
    pub maximum_routes: u64,
    /// Maximum session effect count.
    pub maximum_effects: u64,
    /// Maximum graph session-plus-plan bytes.
    pub maximum_graph_session_plus_plan_bytes: u64,
    /// Maximum total source bytes.
    pub maximum_source_total_bytes: u64,
    /// Maximum source overhead bytes.
    pub maximum_source_overhead_bytes: u64,
    /// Maximum scalar effect state bytes.
    pub maximum_effect_state_bytes: u64,
    /// Maximum scalar effect scratch bytes.
    pub maximum_effect_scratch_bytes: u64,
    /// Maximum retained builtin bytes.
    pub maximum_builtin_retained_bytes: u64,
    /// Maximum retained C-wrapper bytes.
    pub maximum_capi_retained_bytes: u64,
    /// Maximum named allocation bytes.
    pub maximum_named_allocation_bytes: u64,
    /// Maximum meter stream count.
    pub maximum_meter_streams: u64,
    /// Maximum meter item count.
    pub maximum_meter_items: u64,
    /// Maximum meter storage bytes.
    pub maximum_meter_bytes: u64,
    /// Maximum binary control frame bytes.
    pub maximum_control_frame_bytes: u64,
    /// Maximum binary control replay bytes.
    pub maximum_replay_bytes: u64,
    /// Maximum retained capability-command replay records.
    pub maximum_replay_entries: u64,
    /// Must be zero in ABI V1.
    pub reserved: [u64; 4],
}

/// Caller-owned byte output using query/retry semantics.
#[derive(Debug)]
#[repr(C)]
pub struct BytesOut {
    /// Must equal [`BYTES_OUT_SIZE`].
    pub struct_size: u32,
    /// Must be zero in ABI V1.
    pub reserved0: u32,
    /// Caller-owned writable byte region, or null only when capacity is zero.
    pub data: *mut u8,
    /// Size of `data` in bytes.
    pub capacity_bytes: u64,
    /// Complete required byte length written by the callee.
    pub required_bytes: u64,
}

/// One borrowed host-fed planar source chunk.
#[derive(Debug)]
#[repr(C)]
pub struct SourceChunk {
    /// Must equal [`SOURCE_CHUNK_SIZE`].
    pub struct_size: u32,
    /// Exact source sample rate.
    pub sample_rate_hz: u32,
    /// Nonzero source generation.
    pub generation: u64,
    /// Absolute first source frame.
    pub start_frame: u64,
    /// Borrowed array of borrowed channel-plane pointers.
    pub planes: *const *const f32,
    /// Number of source channel planes.
    pub plane_count: u32,
    /// Valid frames in every plane.
    pub frames: u32,
    /// Zero or one; one marks the final chunk of the source region.
    pub end_of_region: u32,
    /// Must be zero in ABI V1.
    pub reserved0: u32,
}

/// Atomic source-submission result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SubmitReport {
    /// Must equal [`SUBMIT_REPORT_SIZE`].
    pub struct_size: u32,
    /// Must be zero in ABI V1.
    pub reserved0: u32,
    /// Frames accepted by this call.
    pub accepted_frames: u64,
    /// Frames written in the active generation.
    pub cumulative_written_frames: u64,
    /// Active generation after the call.
    pub active_generation: u64,
}

/// One caller-owned contiguous planar output region.
#[derive(Debug)]
#[repr(C)]
pub struct PlanarOutput {
    /// Must equal [`PLANAR_OUTPUT_SIZE`].
    pub struct_size: u32,
    /// Must be exactly two in ABI V1.
    pub channels: u32,
    /// Base of the contiguous caller-owned sample region.
    pub samples: *mut f32,
    /// Total sample elements available at `samples`.
    pub sample_capacity: u64,
    /// Requested frames; must equal the prepared quantum.
    pub frames: u32,
    /// Sample elements from the left plane to the right plane.
    pub plane_stride_samples: u32,
    /// Must be zero in ABI V1.
    pub reserved: [u64; 2],
}

/// Frozen ABI V1 capability masks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Capabilities {
    /// Must equal [`CAPABILITIES_SIZE`].
    pub struct_size: u32,
    /// Returned as [`ABI_VERSION`].
    pub abi_version: u32,
    /// Exact supported sample-rate bits.
    pub exact_launch_rate_mask: u64,
    /// Frozen product feature bits.
    pub feature_mask: u64,
    /// Zero in ABI V1 output.
    pub reserved: [u64; 4],
}

/// Address-free production resource projection for a prepared plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PlanResourceReport {
    /// Must equal [`PLAN_RESOURCE_REPORT_SIZE`].
    pub struct_size: u32,
    /// Returned as [`ABI_VERSION`].
    pub abi_version: u32,
    /// Prepared sample rate.
    pub sample_rate_hz: u32,
    /// Prepared render quantum.
    pub quantum_frames: u32,
    /// Prepared source count.
    pub source_count: u64,
    /// Prepared track count.
    pub track_count: u64,
    /// Exact plan latency in samples.
    pub latency_samples: u64,
    /// [`TAIL_FINITE`] or [`TAIL_INFINITE`].
    pub tail_kind: u64,
    /// Finite tail samples, or zero for an infinite tail.
    pub tail_samples: u64,
    /// Graph session-plus-plan bytes.
    pub graph_session_plus_plan_bytes: u64,
    /// Incremental graph plan bytes.
    pub graph_incremental_plan_bytes: u64,
    /// Graph metadata bytes.
    pub graph_metadata_bytes: u64,
    /// Graph delay bytes.
    pub graph_delay_bytes: u64,
    /// Effect-bank scratch bytes.
    pub effect_bank_scratch_bytes: u64,
    /// Effect-bank runtime buffer bytes.
    pub effect_bank_runtime_buffer_bytes: u64,
    /// Effect-bank metadata bytes.
    pub effect_bank_metadata_bytes: u64,
    /// Builtin-bank retained bytes.
    pub builtin_bank_bytes: u64,
    /// Builtin-bank scratch bytes.
    pub builtin_bank_scratch_bytes: u64,
    /// Source PCM payload bytes.
    pub source_pcm_payload_bytes: u64,
    /// Source overhead bytes.
    pub source_overhead_bytes: u64,
    /// Source total bytes.
    pub source_total_bytes: u64,
    /// Scalar effect state bytes.
    pub effect_scalar_state_bytes: u64,
    /// Scalar effect scratch bytes.
    pub effect_scalar_scratch_bytes: u64,
    /// Builtin processor payload bytes.
    pub builtin_processor_payload_bytes: u64,
    /// Builtin meter payload bytes.
    pub builtin_meter_payload_bytes: u64,
    /// Builtin retained payload bytes.
    pub builtin_retained_payload_bytes: u64,
    /// C-wrapper retained bytes.
    pub capi_retained_bytes: u64,
    /// Largest named allocation bytes.
    pub largest_named_allocation_bytes: u64,
    /// Zero in ABI V1 output.
    pub reserved: [u64; 4],
}

/// Frozen size of [`EngineConfig`] on the pinned 64-bit ABI.
pub const ENGINE_CONFIG_SIZE: u32 = 40;
/// Frozen size of [`CompileLimits`] on the pinned 64-bit ABI.
pub const COMPILE_LIMITS_SIZE: u32 = 208;
/// Frozen size of [`BytesOut`] on the pinned 64-bit ABI.
pub const BYTES_OUT_SIZE: u32 = 32;
/// Frozen size of [`SourceChunk`] on the pinned 64-bit ABI.
pub const SOURCE_CHUNK_SIZE: u32 = 48;
/// Frozen size of [`SubmitReport`] on the pinned 64-bit ABI.
pub const SUBMIT_REPORT_SIZE: u32 = 32;
/// Frozen size of [`PlanarOutput`] on the pinned 64-bit ABI.
pub const PLANAR_OUTPUT_SIZE: u32 = 48;
/// Frozen size of [`Capabilities`] on the pinned 64-bit ABI.
pub const CAPABILITIES_SIZE: u32 = 56;
/// Frozen size of [`PlanResourceReport`] on the pinned 64-bit ABI.
pub const PLAN_RESOURCE_REPORT_SIZE: u32 = 240;

const HANDLE_COOKIE: u64 = 0x4d49_534f_5632_4142;
const HANDLE_KIND_ENGINE: u32 = 1;
const HANDLE_KIND_SESSION: u32 = 2;
const HANDLE_KIND_PLAN: u32 = 3;

#[derive(Clone, Copy)]
#[repr(C)]
pub(crate) struct HandleHeader {
    cookie: u64,
    kind: u32,
    abi_version: u32,
}

impl HandleHeader {
    const fn new(kind: u32) -> Self {
        Self {
            cookie: HANDLE_COOKIE,
            kind,
            abi_version: ABI_VERSION,
        }
    }

    pub(crate) const fn is_kind(self, kind: u32) -> bool {
        self.cookie == HANDLE_COOKIE && self.kind == kind && self.abi_version == ABI_VERSION
    }

    pub(crate) const fn is_engine(self) -> bool {
        self.is_kind(HANDLE_KIND_ENGINE)
    }

    pub(crate) const fn is_session(self) -> bool {
        self.is_kind(HANDLE_KIND_SESSION)
    }

    pub(crate) const fn is_plan(self) -> bool {
        self.is_kind(HANDLE_KIND_PLAN)
    }
}

/// Opaque engine handle. C consumers only observe pointers to this type.
#[repr(C)]
pub struct Engine {
    header: HandleHeader,
    pub(crate) last_error: RefCell<[u8; 256]>,
    pub(crate) last_error_len: core::cell::Cell<usize>,
    not_sync: PhantomData<core::cell::Cell<()>>,
}

impl Engine {
    pub(crate) const fn new() -> Self {
        Self {
            header: HandleHeader::new(HANDLE_KIND_ENGINE),
            last_error: RefCell::new([0; 256]),
            last_error_len: core::cell::Cell::new(0),
            not_sync: PhantomData,
        }
    }
}

/// Opaque session-control handle. C consumers only observe pointers to this type.
#[repr(C)]
pub struct Session {
    header: HandleHeader,
    pub(crate) state: SessionState,
    pub(crate) last_error: RefCell<FixedBytes>,
    not_sync: PhantomData<core::cell::Cell<()>>,
}

impl Session {
    pub(crate) fn new(state: SessionState, last_error: FixedBytes) -> Self {
        Self {
            header: HandleHeader::new(HANDLE_KIND_SESSION),
            state,
            last_error: RefCell::new(last_error),
            not_sync: PhantomData,
        }
    }
}

/// Opaque render-plan handle. C consumers only observe pointers to this type.
#[repr(C)]
pub struct Plan {
    header: HandleHeader,
    pub(crate) state: PlanState,
    pub(crate) last_error: RefCell<FixedBytes>,
    not_sync: PhantomData<core::cell::Cell<()>>,
}

impl Plan {
    pub(crate) fn new(state: PlanState, last_error: FixedBytes) -> Self {
        Self {
            header: HandleHeader::new(HANDLE_KIND_PLAN),
            state,
            last_error: RefCell::new(last_error),
            not_sync: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn frozen_sizes_alignments_and_representative_offsets_match() {
        assert_eq!(size_of::<EngineConfig>(), ENGINE_CONFIG_SIZE as usize);
        assert_eq!(size_of::<CompileLimits>(), COMPILE_LIMITS_SIZE as usize);
        assert_eq!(size_of::<BytesOut>(), BYTES_OUT_SIZE as usize);
        assert_eq!(size_of::<SourceChunk>(), SOURCE_CHUNK_SIZE as usize);
        assert_eq!(size_of::<SubmitReport>(), SUBMIT_REPORT_SIZE as usize);
        assert_eq!(size_of::<PlanarOutput>(), PLANAR_OUTPUT_SIZE as usize);
        assert_eq!(size_of::<Capabilities>(), CAPABILITIES_SIZE as usize);
        assert_eq!(
            size_of::<PlanResourceReport>(),
            PLAN_RESOURCE_REPORT_SIZE as usize
        );

        assert_eq!(align_of::<EngineConfig>(), 8);
        assert_eq!(align_of::<CompileLimits>(), 8);
        assert_eq!(align_of::<BytesOut>(), 8);
        assert_eq!(align_of::<SourceChunk>(), 8);
        assert_eq!(align_of::<SubmitReport>(), 8);
        assert_eq!(align_of::<PlanarOutput>(), 8);
        assert_eq!(align_of::<Capabilities>(), 8);
        assert_eq!(align_of::<PlanResourceReport>(), 8);

        assert_eq!(offset_of!(CompileLimits, maximum_toml_bytes), 16);
        assert_eq!(offset_of!(CompileLimits, maximum_replay_entries), 168);
        assert_eq!(offset_of!(CompileLimits, reserved), 176);
        assert_eq!(offset_of!(BytesOut, data), 8);
        assert_eq!(offset_of!(BytesOut, required_bytes), 24);
        assert_eq!(offset_of!(SourceChunk, planes), 24);
        assert_eq!(offset_of!(SourceChunk, reserved0), 44);
        assert_eq!(offset_of!(PlanarOutput, samples), 8);
        assert_eq!(offset_of!(PlanarOutput, reserved), 32);
        assert_eq!(offset_of!(Capabilities, reserved), 24);
        assert_eq!(offset_of!(PlanResourceReport, source_count), 16);
        assert_eq!(offset_of!(PlanResourceReport, reserved), 208);
    }

    #[test]
    fn masks_and_result_codes_are_frozen() {
        assert_eq!(ABI_VERSION, 0x0001_0000);
        assert_eq!(EXACT_LAUNCH_RATE_MASK, 0x0f);
        assert_eq!(FEATURE_MASK, 0x1f);
        assert_eq!(
            [
                RESULT_OK,
                RESULT_INVALID_ARGUMENT,
                RESULT_ABI_MISMATCH,
                RESULT_WRONG_HANDLE,
                RESULT_BUFFER_TOO_SMALL,
                RESULT_COMPILE_REJECTED,
                RESULT_BACKPRESSURE,
                RESULT_UNSUPPORTED,
                RESULT_RENDER_REJECTED,
                RESULT_INTERNAL,
            ],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 255]
        );
    }
}
