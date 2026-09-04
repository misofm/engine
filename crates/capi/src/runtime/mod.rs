//! Safe control-plane orchestration behind the raw FFI boundary.
//!
//! Audit #103 F7 found one file doing five jobs. Each is now its own module, and the boundaries
//! are the ones the C ABI actually has:
//!
//! | module | job |
//! |---|---|
//! | [`error`] | bounded diagnostic storage and the failure vocabularies the boundary reports |
//! | [`compile`] | capi's own resource projection and the children `compile_session` returns |
//! | [`plan`] | render-thread plan ownership and the any-thread query projection |
//! | [`control`] | the control-protocol session: commands, events, sources, plan replacement |
//!
//! The compile *pipeline* is not here at all: it is `host-core`, shared with every
//! other host.

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
pub(crate) use host_core::SourceSubmission;
pub(crate) use host_core::{
    HostPrepareCaps, HostShapePolicy, PrepareDiagnostics, SessionControlProvider,
    SourceControlError, SourceControlSet, parse_host_session, prepare_host_runtime,
};
pub(crate) use protocol::{
    CommandFrameProcessError, ControllerRetainedCapacity, DecodeScratch, EncodeError,
    EventEgressError, PreparedCommandFrame, ProtocolCodec, ProtocolController,
    ProtocolControllerConfig, ProtocolLimits, ProtocolQueueConfig, ProtocolQueues,
    ProviderFeatures, ReplayCache, ReplayCacheConfig, SessionStore,
};
pub(crate) use session::{CompiledSession, DiagnosticSet};

pub(crate) use crate::{
    ABI_VERSION, CompileLimits, PlanResourceReport, RESULT_BACKPRESSURE, RESULT_INTERNAL,
    RESULT_INVALID_ARGUMENT, TAIL_FINITE, TAIL_INFINITE,
};

pub(crate) mod compile;
pub(crate) mod control;
pub(crate) mod error;
pub(crate) mod plan;
#[cfg(test)]
mod tests;

pub(crate) use compile::*;
pub(crate) use control::*;
pub(crate) use error::*;
pub(crate) use plan::*;
