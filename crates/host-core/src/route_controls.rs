//! Live send records (issue #1221): the control-plane half of a live route into a submix.
//!
//! One [`RouteControlProducer`] per route into a submix, in canonical route-ID order
//! ([`crate::HostLiveControlHandles::route_controls`]). A producer turns a send's gain, matrix,
//! mute and source-lane mutes into one ramp record, or refuses with nothing pushed.
//!
//! # One authority
//!
//! The record's target is `graph_compiler::route_coefficients` for the same arguments -- the
//! function the compiler lowers a prepared route's constants with -- and its `mute` is
//! `graph::RouteGate::silences` of the same gate, the predicate a prepared route binds its own
//! mute with. Nothing here re-derives a coefficient or a domain rule, so a settled live send and a
//! freshly prepared one carry the same bits.
//!
//! # No ack precedes a drop
//!
//! Every refusal is decided before the queue is touched: [`RouteControlProducer::record`] is pure,
//! and [`RouteControlProducer::push`] decides [`RouteControlError::Full`] from
//! [`RouteControlProducer::free`] before it pushes. A host admitting several routes at once builds
//! every record and checks every queue's room first, and only then pushes.

use graph::{GraphRouteControlProducer, ROUTE_RAMP_LENGTH_MAXIMUM, RouteGate};
use graph_compiler::route_coefficients;

pub use graph::{RouteControlRecord, RouteControlResources};

/// Why a live send record was refused. Every refusal leaves the queue unchanged.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteControlError {
    /// `graph_compiler::route_coefficients` refused the gain or matrix.
    Domain,
    /// The ramp is longer than `graph::ROUTE_RAMP_LENGTH_MAXIMUM` (`1 << 22`) samples.
    Length,
    /// The route's queue has no room; nothing was pushed.
    Full,
}

/// The control-plane half of one live send's record queue.
///
/// A transparent wrapper over the graph's producer, so the producer table host-core retains is
/// exactly the bytes `graph::route_control_resources` charges for it: one graph producer per
/// route, its route ID included and never copied. The ID is read through [`Self::route_id`].
///
/// # Lifetime (#1221 verdict NIT-3)
///
/// - A push's `Ok` means queued, not applied: the render thread applies the record at the route's
///   op in a later block.
/// - A producer belongs to the `PreparedHost` plan it was prepared with. A host drops or replaces
///   it together with that plan; a producer kept past its plan accepts records until its ring
///   fills, and nothing ever drains them.
/// - A record still queued when the plan is replaced is discarded with it, so the replacement must
///   be prepared from the committed state that already holds every acked value.
///
/// The strip and effect producers share this contract.
#[repr(transparent)]
pub struct RouteControlProducer {
    producer: GraphRouteControlProducer,
}

impl RouteControlProducer {
    pub(crate) const fn new(producer: GraphRouteControlProducer) -> Self {
        Self { producer }
    }

    /// The route's session ID.
    #[must_use]
    pub fn route_id(&self) -> &str {
        &self.producer.route_id
    }

    /// How many records the queue can accept now.
    #[must_use]
    pub fn free(&self) -> usize {
        self.producer.free()
    }

    /// Validates the values and builds their record. Pure: pushes nothing.
    ///
    /// Refused with [`RouteControlError::Domain`] when `route_coefficients` refuses, then with
    /// [`RouteControlError::Length`] when `length > ROUTE_RAMP_LENGTH_MAXIMUM`.
    pub fn record(
        &self,
        gain_db: f32,
        matrix: [f32; 4],
        mute: bool,
        source_lane_muted: [bool; 2],
        length: u32,
    ) -> Result<RouteControlRecord, RouteControlError> {
        let target = route_coefficients(gain_db, matrix, mute, source_lane_muted)
            .map_err(|_| RouteControlError::Domain)?;
        if length > ROUTE_RAMP_LENGTH_MAXIMUM {
            return Err(RouteControlError::Length);
        }
        let silenced = RouteGate {
            mute,
            follow_zeroed: source_lane_muted,
        }
        .silences();
        // A silencing gate's coefficients are the four `+0.0` the record requires, so with the
        // length in bounds `new` cannot refuse; the arm is the domain rule's, never a drop.
        RouteControlRecord::new(target, silenced, length).ok_or(RouteControlError::Domain)
    }

    /// Pushes a record [`Self::record`] built. A full queue refuses with
    /// [`RouteControlError::Full`], decided before the push, and nothing is queued.
    pub fn push(&mut self, record: RouteControlRecord) -> Result<(), RouteControlError> {
        if self.producer.free() == 0 {
            return Err(RouteControlError::Full);
        }
        self.producer
            .try_push(record)
            .map_err(|_| RouteControlError::Full)
    }

    /// [`Self::record`], then [`Self::push`]: either one record is queued, or nothing is.
    pub fn set(
        &mut self,
        gain_db: f32,
        matrix: [f32; 4],
        mute: bool,
        source_lane_muted: [bool; 2],
        length: u32,
    ) -> Result<(), RouteControlError> {
        let record = self.record(gain_db, matrix, mute, source_lane_muted, length)?;
        self.push(record)
    }
}
