//! The engine's portable control plane: one session's control-protocol state, its live-update
//! classification, successor-plan preparation, plan publication and retirement, and the provider
//! epochs that keep the host-fed producers matched to the plan that renders them.
//!
//! Decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`,
//! D15-11) gives the engine one control plane for every host. This crate is it, and it is portable:
//! it builds for `wasm32-unknown-unknown` with `simd128` as well as for the native targets.
//!
//! Two adapters sit on top of it. `capi`, the C ABI that iOS and Android embed, owns the FFI, its
//! fixed-width ABI types and its diagnostic vocabulary. The browser adapter (#1332) runs the same
//! control plane in a Worker. An adapter describes itself through [`ControlAdapter`]: the row type
//! of its plan resource table and the fixed allocations it keeps per session. It passes plain
//! [`ControlLimits`], and reads plain [`PlanResources`] and typed failures
//! ([`CompileFailure`], [`SourceFailure`], [`SessionSeekFailure`], [`CommandError`], [`EventError`]) that it maps to its
//! own codes. The crate owns every command and event refusal text (decision 17 C1), through
//! [`CommandError::diagnostic`], [`CompileFailure::diagnostic`] and [`EventError::diagnostic`]; an
//! adapter owns only its result-code numbering.
//!
//! | module | job |
//! |---|---|
//! | `adapter` | the adapter boundary: [`ControlAdapter`], [`ControlLimits`], [`PlanResources`] |
//! | `error` | the typed failures the boundary reports |
//! | `compile` | the control plane's own resource projection and the children a compile returns |
//! | `plan` | render-thread plan ownership and the any-thread query projection |
//! | `control` | the control-protocol session: commands, events, sources, plan replacement |
//! | `document` | `SESSION_REPLACE`: the check, parse and diff of a replacement document |
//! | `spectrum` | the session's spectrum monitor state: [`SpectrumMonitor`] |
//!
//! The compile *pipeline* is not here at all: it is `host-core`, shared with every host.

pub(crate) use core::{alloc::Layout, mem::size_of, num::NonZeroUsize};
pub(crate) use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU64, Ordering},
};

pub(crate) use effect_contract::TailSamples;
pub(crate) use engine::realtime::{
    PlanAdoption, PlanExchangeConfig, PlanPublisher, PlanReplacementReservation,
    PlanReplacementReservationError, PlanRetirer, PlanRevisionGate, PlanWatermark,
    PlanWatermarkReader, PlanarBufferMut, PreparedRenderPlan, RealtimePlanOwner, RenderError,
    RenderIo, WatermarkBusy, plan_exchange_at_revision, plan_exchange_resource_report,
};
pub(crate) use host_core::{
    ControlCatalog, HostLiveLanes, HostPrepareCaps, HostShapePolicy, PlanStateInventory,
    PrepareDiagnostics, SessionControlProvider, SourceControlError, SourceControlSet,
    SuccessorBase, parse_host_session,
};
pub(crate) use protocol::{
    CommandFrameProcessError, ControllerRetainedCapacity, DecodeScratch, EncodeError,
    EventEgressError, PreparedCommandFrame, ProtocolCodec, ProtocolController,
    ProtocolControllerConfig, ProtocolLimits, ProtocolQueueConfig, ProtocolQueues,
    ProviderFeatures, ReplayCache, ReplayCacheConfig, SessionStore,
};
pub(crate) use session::{CompiledSession, DiagnosticSet};

mod adapter;
mod compile;
mod control;
mod document;
mod error;
mod loudness;
mod plan;
mod spectrum;

pub use adapter::{
    ControlAdapter, ControlLimits, PlanResources, TAIL_FINITE, TAIL_INFINITE, tail_words,
};
pub use compile::{
    CompiledChildren, RuntimePreparer, compile_children, compile_children_with_boot_command,
    frame_scaled_retained_bytes,
};
pub use control::{
    CarriedTaps, CommandError, EventError, EventLane, ObservationChange, ObservationForward,
    ObservationKeyError, ObservationKeyRef, ObservationLimit, ObservationRefusal,
    REPREPARE_REQUIRED, ResolvedObservationTap, SessionSeekFailure, SessionState, SourceFailure,
    SourceRefusalKind, TransitionStep, resolve_observation_tap,
};
pub use document::{
    DOCUMENT_DIGEST, DOCUMENT_LENGTH, DOCUMENT_TOO_LARGE, DOCUMENT_UTF8, diff as diff_document,
    replace as replace_document,
};
pub use error::{CompileFailure, ResourceFault};
/// One planar chunk a host submits to a source of the session ([`SessionState::submit`]).
pub use host_core::SourceSubmission;
pub use plan::{PlanQueries, PlanState};
pub use source::SourceSeekReport;
pub use spectrum::SpectrumMonitor;

#[cfg(feature = "test-support")]
pub use compile::{
    CapiResources, CarriedSourceBytes, CompiledModelAdmission, LiveCatalogAdmission,
    LiveEpochResources, duck_overlay_bytes, meter_text_reserve_bytes, monitor_and_ramp_bytes,
    observation_binding_table_bytes, observation_reserve_bytes, prepare_caps,
    response_capture_bytes, validate_live_peak,
};
#[cfg(feature = "test-support")]
pub use control::{
    EffectCellRow, ProviderEpoch, TestObservationWindows, TestOwnerCounters, TestOwnerState,
    TestStructuralFaultPhase, TestTransactionSnapshot, test_force_full_preparation,
    test_hold_reclamation, test_lifecycle_counters, test_reset_lifecycle_observer,
    test_take_in_place_counts, test_verify_catalog_stages,
};

pub(crate) use compile::*;
pub(crate) use control::*;
pub(crate) use error::*;
pub(crate) use plan::*;
