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
//! [`RouteControlProducer::record`] is the only builder of a [`RouteControlRecord`], and
//! [`RouteControlProducer::push`] accepts nothing else (issue #1416). The record is host-core's
//! own type, not a re-export of `graph::RouteControlRecord`, whose public constructor does not check
//! the route domain; so a host cannot queue a target that `route_coefficients` would refuse.
//!
//! # No ack precedes a drop
//!
//! Every refusal is decided before the queue is touched: [`RouteControlProducer::record`] is pure,
//! and [`RouteControlProducer::push`] decides [`RouteControlError::Full`] from
//! [`RouteControlProducer::free`] before it pushes. A host admitting several routes at once builds
//! every record and checks every queue's room first, and only then pushes.

use graph::{GraphRouteControlProducer, ROUTE_RAMP_LENGTH_MAXIMUM, RouteGate};
use graph_compiler::route_coefficients;

pub use graph::RouteControlResources;

/// One validated live send record, built only by [`RouteControlProducer::record`] (issue #1416).
///
/// It wraps `graph::RouteControlRecord`, whose public constructor checks the ramp length and the
/// mute rule but not the route domain. This type has a private field and no public constructor, so
/// every record a host can push has passed `route_coefficients`.
///
/// The fences below check three doors, each beside a plain twin that differs only in the one
/// forbidden construct. A public associated `new` of any signature (or a re-export of graph's
/// record, which has one) turns the first red; rustc reports E0599 today (no function `new`). The
/// fences carry no code because stable rustdoc does not check one (issue #1422 D2):
///
/// ```compile_fail
/// fn send(
///     producer: &mut host_core::RouteControlProducer,
///     inner: graph::RouteControlRecord,
/// ) -> Result<(), host_core::RouteControlError> {
///     let _ = host_core::RouteControlRecord::new;
///     let record: host_core::RouteControlRecord =
///         producer.record(0.0, [1.0, 0.0, 0.0, 1.0], false, [false; 2], 0).unwrap();
///     producer.push(record)
/// }
/// ```
///
/// A host cannot wrap graph's record with the tuple constructor. rustc reports E0423 (the
/// constructor is private):
///
/// ```compile_fail
/// fn send(
///     producer: &mut host_core::RouteControlProducer,
///     inner: graph::RouteControlRecord,
/// ) -> Result<(), host_core::RouteControlError> {
///     let record: host_core::RouteControlRecord = host_core::RouteControlRecord(inner);
///     producer.push(record)
/// }
/// ```
///
/// Nor can it convert graph's record with `From`/`Into`. rustc reports E0277 (no
/// `From<graph::RouteControlRecord>`):
///
/// ```compile_fail
/// fn send(
///     producer: &mut host_core::RouteControlProducer,
///     inner: graph::RouteControlRecord,
/// ) -> Result<(), host_core::RouteControlError> {
///     let record: host_core::RouteControlRecord = inner.into();
///     producer.push(record)
/// }
/// ```
///
/// The twin of all three: identical except that the producer builds the record, so a renamed item
/// in the shared code turns it red:
///
/// ```
/// fn send(
///     producer: &mut host_core::RouteControlProducer,
///     inner: graph::RouteControlRecord,
/// ) -> Result<(), host_core::RouteControlError> {
///     let record: host_core::RouteControlRecord =
///         producer.record(0.0, [1.0, 0.0, 0.0, 1.0], false, [false; 2], 0).unwrap();
///     producer.push(record)
/// }
/// ```
///
/// No fence can see a public constructor under another name, or a named public field; privacy and
/// review hold those doors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RouteControlRecord(graph::RouteControlRecord);

impl RouteControlRecord {
    /// The coefficients `[ll, lr, rl, rr]` the ramp ends on.
    #[must_use]
    pub const fn target(&self) -> [f32; 4] {
        self.0.target()
    }
    /// Whether the route is silenced once the ramp ends.
    #[must_use]
    pub const fn mute(&self) -> bool {
        self.0.mute()
    }
    /// The ramp's length in samples; `0` is a step.
    #[must_use]
    pub const fn length(&self) -> u32 {
        self.0.length()
    }
}

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
        graph::RouteControlRecord::new(target, silenced, length)
            .map(RouteControlRecord)
            .ok_or(RouteControlError::Domain)
    }

    /// Pushes a record [`Self::record`] built. A full queue refuses with
    /// [`RouteControlError::Full`], decided before the push, and nothing is queued.
    pub fn push(&mut self, record: RouteControlRecord) -> Result<(), RouteControlError> {
        if self.producer.free() == 0 {
            return Err(RouteControlError::Full);
        }
        self.producer
            .try_push(record.0)
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
