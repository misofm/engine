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
//! ([`CompileFailure`], [`SourceFailure`], [`CommandError`], [`EventError`]) that it maps to its
//! own codes. The crate never spells an adapter's diagnostic codes.
//!
//! | module | job |
//! |---|---|
//! | `adapter` | the adapter boundary: [`ControlAdapter`], [`ControlLimits`], [`PlanResources`] |
//! | `error` | the typed failures the boundary reports |
//! | `compile` | the control plane's own resource projection and the children a compile returns |
//! | `plan` | render-thread plan ownership and the any-thread query projection |
//! | `control` | the control-protocol session: commands, events, sources, plan replacement |
//!
//! The compile *pipeline* is not here at all: it is `host-core`, shared with every host.

pub(crate) use core::{alloc::Layout, mem::size_of, num::NonZeroUsize};
pub(crate) use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
};

pub(crate) use effect_contract::TailSamples;
pub(crate) use engine::realtime::{
    PlanExchangeConfig, PlanPublisher, PlanReplacementReservation, PlanReplacementReservationError,
    PlanRetirer, PlanarBufferMut, PreparedRenderPlan, RealtimePlanOwner, RenderError, RenderIo,
    plan_exchange, plan_exchange_resource_report,
};
pub(crate) use host_core::{
    HostLiveControlRequest, HostLiveLanes, HostPrepareCaps, HostShapePolicy, PlanStateInventory,
    PrepareDiagnostics, PreparedSessionControlCatalog, SessionControlProvider, SourceControlError,
    SourceControlSet, SuccessorBase, parse_host_session, prepare_host_runtime_with_live_lanes,
    prepare_host_runtime_with_live_lanes_successor,
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
mod error;
mod plan;

pub use adapter::{ControlAdapter, ControlLimits, PlanResources};
pub use compile::{CompiledChildren, compile_children};
pub use control::{CommandError, EventError, EventLane, SessionState, SourceFailure};
pub use error::{CompileFailure, ResourceFault};
/// One planar chunk a host submits to a source of the session ([`SessionState::submit`]).
pub use host_core::SourceSubmission;
pub use plan::{PlanQueries, PlanState};

#[cfg(feature = "test-support")]
pub use compile::{
    C_ABI_LIVE_LANES, CapiResources, CompiledModelAdmission, LIVE_QUEUE_DEPTH, LiveEpochResources,
    prepare_caps, validate_live_peak,
};
#[cfg(feature = "test-support")]
pub use control::{
    ProviderEpoch, TestOwnerCounters, TestOwnerState, TestStructuralFaultPhase,
    TestTransactionSnapshot, test_lifecycle_counters, test_reset_lifecycle_observer,
};

pub(crate) use compile::*;
pub(crate) use control::*;
pub(crate) use error::*;
pub(crate) use plan::*;
