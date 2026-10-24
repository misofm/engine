//! The render-half capture of one strip's response owners (#1577, decision 17 C1 and C13).
//!
//! The control side arms a capture with [`ResponseCaptureClient::arm`]: it allocates one
//! [`ResponseCaptureJob`], sized from the committed model, and moves it to the render thread
//! through a move-only queue of capacity 1. After each successful block the render thread runs
//! [`ResponseCaptureServer::after_block`]: it copies the strip's owners into the job's
//! preallocated collector once, from the first block whose active plan is the newest plan the
//! control side knew of when it armed, and moves the job back. The control side then finishes the
//! collector and composes the track response with [`compose_track_response`]
//! ([`ResponseCaptureClient::poll`]).
//!
//! The render step allocates, frees, locks and logs nothing: the job and every byte the copy
//! writes were allocated by the control side, and the job only moves.
//!
//! One session has at most one capture in flight: from `arm` until [`ResponseCaptureClient::release`]
//! (after `poll` delivered the result), `arm` answers [`ResponseCaptureRefusal::Pending`].

use core::num::NonZeroUsize;

use engine::realtime::{
    Consumer, Producer, QueueGeneration, RealtimePlanOwner, RealtimeResponseSnapshot,
    ResponseSnapshotRequest, SpscError, bounded_spsc_move,
};
use session::SessionModel;

use crate::response::{
    ResponseChannels, ResponsePreviewGrid, ResponseSnapshotCollector, ResponseSnapshotQueryError,
    TrackResponse, check_response_request, compose_track_response,
};
use crate::{ResponseSnapshotCapture, ResponseSnapshotError};

/// The most sections one owner copies per side.
const SECTIONS: usize = effect_contract::RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS;
/// The longest identity a collector holds: a stable ID's bound, which also holds every native ID.
const IDENTITY_BYTES: usize = session::STABLE_ID_MAXIMUM_BYTES;

/// What the render step made of a job.
enum CaptureOutcome {
    /// Not yet captured.
    Pending,
    /// Copied from the plan the job names.
    Captured(RealtimeResponseSnapshot),
    /// The active plan is newer than the job's: the control side arms it again.
    Stale,
    /// The plan refused the copy.
    Failed(ResponseSnapshotError),
}

/// One armed capture: allocated by the control side, moved to the render thread and back.
pub struct ResponseCaptureJob {
    tag: u64,
    plan_epoch: u64,
    strip_id: String,
    collector: ResponseSnapshotCollector,
    outcome: CaptureOutcome,
}

/// The channel could not be built.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResponseCaptureSetupError;

/// Why [`ResponseCaptureClient::arm`] refused a request. A refusal changes nothing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseCaptureRefusal {
    /// The committed model has no track or submix of that ID.
    UnknownStrip,
    /// A capture is in flight, or delivered and not yet released.
    Pending,
    /// The point count or the frequency axis is invalid at the session rate (the #1572 rules).
    Grid,
}

/// Why a delivered capture has no track response.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseCaptureFailure {
    /// The strip left the session before a plan that holds it rendered.
    StripRemoved,
    /// The strip has no owner with a linear response provider.
    Unsupported,
    /// A capacity bound was exceeded.
    Capacity,
    /// The capture or the composition failed for no reason a request can cure.
    Internal,
}

impl ResponseCaptureFailure {
    /// The dotted code every host reports for this failure.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::StripRemoved => "response.strip.removed",
            Self::Unsupported => "response.effect.unsupported",
            Self::Capacity => "response.limit",
            Self::Internal => "response.capture",
        }
    }

    const fn of_capture(error: ResponseSnapshotError) -> Self {
        match error {
            ResponseSnapshotError::MissingTrack => Self::StripRemoved,
            ResponseSnapshotError::Unsupported => Self::Unsupported,
            ResponseSnapshotError::Capacity => Self::Capacity,
            ResponseSnapshotError::InvalidShape | ResponseSnapshotError::Owner => Self::Internal,
        }
    }

    const fn of_composition(error: ResponseSnapshotQueryError) -> Self {
        match error {
            ResponseSnapshotQueryError::UnsupportedProvider => Self::Unsupported,
            ResponseSnapshotQueryError::Capacity => Self::Capacity,
            ResponseSnapshotQueryError::InvalidGrid
            | ResponseSnapshotQueryError::OutputShape
            | ResponseSnapshotQueryError::MalformedSnapshot
            | ResponseSnapshotQueryError::Numerical => Self::Internal,
        }
    }
}

/// One request's result, delivered once by [`ResponseCaptureClient::poll`].
#[derive(Clone, Debug, PartialEq)]
pub struct ResponseReady {
    /// The tag [`ResponseCaptureClient::arm`] returned.
    pub tag: u64,
    /// The strip the request named.
    pub strip_id: Box<str>,
    /// The composed response, or why there is none.
    pub result: Result<TrackResponse, ResponseCaptureFailure>,
}

/// The request the client holds while it is in flight.
#[derive(Clone, Copy, Debug)]
struct Armed {
    tag: u64,
    grid: ResponsePreviewGrid,
    channels: ResponseChannels,
}

/// The client's one request.
#[derive(Clone, Copy, Debug)]
enum ClientState {
    /// Nothing armed: `arm` may arm.
    Idle,
    /// Armed and not yet delivered.
    InFlight(Armed),
    /// Delivered by `poll`, not yet released.
    Delivered,
}

/// The control half: arms one capture at a time and composes its result.
pub struct ResponseCaptureClient {
    requests: Producer<Box<ResponseCaptureJob>>,
    replies: Consumer<Box<ResponseCaptureJob>>,
    newest_plan_epoch: u64,
    next_tag: u64,
    state: ClientState,
}

/// The render half: holds at most one job between blocks.
pub struct ResponseCaptureServer {
    requests: Consumer<Box<ResponseCaptureJob>>,
    replies: Producer<Box<ResponseCaptureJob>>,
    held: Option<Box<ResponseCaptureJob>>,
}

/// Build the session's capture channel: two move-only queues of capacity 1, requests (control to
/// render) and replies (render to control). Once per session, at boot, on the control thread.
///
/// # Errors
///
/// [`ResponseCaptureSetupError`] when a queue cannot be built.
pub fn response_capture_channel()
-> Result<(ResponseCaptureClient, ResponseCaptureServer), ResponseCaptureSetupError> {
    let one = NonZeroUsize::MIN;
    let (request_producer, request_consumer) = bounded_spsc_move(one, QueueGeneration(0))
        .map_err(|_: SpscError| ResponseCaptureSetupError)?;
    let (reply_producer, reply_consumer) = bounded_spsc_move(one, QueueGeneration(0))
        .map_err(|_: SpscError| ResponseCaptureSetupError)?;
    Ok((
        ResponseCaptureClient {
            requests: request_producer,
            replies: reply_consumer,
            newest_plan_epoch: 0,
            next_tag: 1,
            state: ClientState::Idle,
        },
        ResponseCaptureServer {
            requests: request_consumer,
            replies: reply_producer,
            held: None,
        },
    ))
}

/// The number of owners `strip_id` has in `model` (its input filters, every console slot and its
/// inserts), or `None` when the model has no such track or submix.
fn strip_owners(model: &SessionModel, strip_id: &str) -> Option<usize> {
    let inserts = model
        .tracks
        .iter()
        .find(|track| track.id.as_str() == strip_id)
        .map(|track| track.inserts.effects.len())
        .or_else(|| {
            model
                .submixes
                .iter()
                .find(|submix| submix.id.as_str() == strip_id)
                .map(|submix| submix.inserts.effects.len())
        })?;
    Some(1 + model.console.slots().count() + inserts)
}

impl ResponseCaptureClient {
    /// Record the newest plan epoch: at boot and whenever a replacement plan's epoch is
    /// published. It never decreases. A withdrawn candidate is either published again with its
    /// own epoch or superseded by a newer one, so a held capture never waits on an epoch render
    /// will not reach.
    pub fn set_newest_plan_epoch(&mut self, epoch: u64) {
        debug_assert!(
            epoch >= self.newest_plan_epoch,
            "the newest plan epoch never decreases"
        );
        self.newest_plan_epoch = epoch;
    }

    /// Arm one capture of `strip_id` from the newest plan; its tag (from 1, per session).
    ///
    /// # Errors
    ///
    /// In this order, before any write: [`ResponseCaptureRefusal::UnknownStrip`],
    /// [`ResponseCaptureRefusal::Pending`] (also for a full request queue) and
    /// [`ResponseCaptureRefusal::Grid`].
    pub fn arm(
        &mut self,
        model: &SessionModel,
        strip_id: &str,
        grid: ResponsePreviewGrid,
        channels: ResponseChannels,
    ) -> Result<u64, ResponseCaptureRefusal> {
        let owners = strip_owners(model, strip_id).ok_or(ResponseCaptureRefusal::UnknownStrip)?;
        if !matches!(self.state, ClientState::Idle) {
            return Err(ResponseCaptureRefusal::Pending);
        }
        check_response_request(model.sample_rate_hz, grid, 0, true)
            .map_err(|_| ResponseCaptureRefusal::Grid)?;
        let tag = self.next_tag;
        let mut id = String::with_capacity(IDENTITY_BYTES);
        id.push_str(strip_id);
        let job = Box::new(ResponseCaptureJob {
            tag,
            plan_epoch: self.newest_plan_epoch,
            strip_id: id,
            collector: collector(model, owners),
            outcome: CaptureOutcome::Pending,
        });
        self.requests
            .try_push(job)
            .map_err(|_| ResponseCaptureRefusal::Pending)?;
        self.next_tag += 1;
        self.state = ClientState::InFlight(Armed {
            tag,
            grid,
            channels,
        });
        Ok(tag)
    }

    /// Take the render step's reply, if one is there, and turn it into the request's result.
    ///
    /// A stale reply (the active plan moved past the job's) is armed again from the newest plan,
    /// with the same tag and a collector sized from `model`, and gives `None`; if `model` no
    /// longer has the strip, the result is [`ResponseCaptureFailure::StripRemoved`]. Each
    /// request's result is returned once.
    pub fn poll(&mut self, model: &SessionModel) -> Option<ResponseReady> {
        let ClientState::InFlight(armed) = self.state else {
            return None;
        };
        let mut job = self.take_reply()?;
        debug_assert_eq!(job.tag, armed.tag, "the reply is the armed request's");
        let result = match core::mem::replace(&mut job.outcome, CaptureOutcome::Pending) {
            CaptureOutcome::Pending => Err(ResponseCaptureFailure::Internal),
            CaptureOutcome::Failed(error) => Err(ResponseCaptureFailure::of_capture(error)),
            CaptureOutcome::Captured(snapshot) => {
                let ResponseCaptureJob {
                    strip_id,
                    collector,
                    ..
                } = *job;
                let snapshot = collector.finish(ResponseSnapshotCapture {
                    captured_sample: snapshot.captured_sample,
                    owners: snapshot.owners,
                });
                let result =
                    compose_track_response(&strip_id, &snapshot, armed.grid, armed.channels)
                        .map_err(ResponseCaptureFailure::of_composition);
                return Some(self.deliver(armed.tag, &strip_id, result));
            }
            CaptureOutcome::Stale => match strip_owners(model, &job.strip_id) {
                None => Err(ResponseCaptureFailure::StripRemoved),
                Some(owners) => {
                    job.plan_epoch = self.newest_plan_epoch;
                    job.collector = collector(model, owners);
                    match self.requests.try_push(job) {
                        Ok(()) => return None,
                        // The job was the only one in the channel and render has given it back,
                        // so the request queue is empty; a full one would be a defect.
                        Err(full) => {
                            let strip_id = full.value.strip_id.clone();
                            return Some(self.deliver(
                                armed.tag,
                                &strip_id,
                                Err(ResponseCaptureFailure::Internal),
                            ));
                        }
                    }
                }
            },
        };
        Some(self.deliver(armed.tag, &job.strip_id, result))
    }

    /// End the delivered state, so `arm` may arm again. A no-op unless `poll` has delivered the
    /// armed request's result and it is not yet released.
    pub fn release(&mut self) {
        if matches!(self.state, ClientState::Delivered) {
            self.state = ClientState::Idle;
        }
    }

    fn deliver(
        &mut self,
        tag: u64,
        strip_id: &str,
        result: Result<TrackResponse, ResponseCaptureFailure>,
    ) -> ResponseReady {
        self.state = ClientState::Delivered;
        ResponseReady {
            tag,
            strip_id: strip_id.into(),
            result,
        }
    }
}

/// A collector for `owners` owners at the session rate, 6 sections and 127-byte identities.
fn collector(model: &SessionModel, owners: usize) -> ResponseSnapshotCollector {
    ResponseSnapshotCollector::new(model.sample_rate_hz, owners, SECTIONS, IDENTITY_BYTES)
}

// REALTIME_POLICY_BEGIN
impl ResponseCaptureClient {
    /// One pop of the reply queue: a single bounded pop, on the control thread.
    fn take_reply(&mut self) -> Option<Box<ResponseCaptureJob>> {
        self.replies.try_pop().ok()
    }
}

impl ResponseCaptureServer {
    /// The render step after a successful block, on the thread that renders.
    ///
    /// With no job held it tries one pop (one atomic load when the queue is empty). A held job
    /// whose plan epoch the active plan has not reached waits; one the active plan has passed is
    /// `Stale`; at its epoch it is captured once, `Captured` or `Failed`. A job whose outcome is
    /// set is pushed to the replies; a full push keeps it for the next block, so a job is never
    /// dropped here and never captured twice. Its cost is bounded by the collector's capacity.
    pub fn after_block(&mut self, owner: &mut RealtimePlanOwner) {
        if self.held.is_none() {
            match self.requests.try_pop() {
                Ok(job) => self.hold(job),
                Err(_) => return,
            }
        }
        let Some(job) = self.held.as_mut() else {
            return;
        };
        if matches!(job.outcome, CaptureOutcome::Pending) {
            let active = owner.active_epoch().0;
            if active < job.plan_epoch {
                return;
            }
            job.outcome = if active > job.plan_epoch {
                CaptureOutcome::Stale
            } else {
                match owner.copy_response_snapshot(ResponseSnapshotRequest {
                    track_id: &job.strip_id,
                    sink: &mut job.collector,
                }) {
                    Ok(snapshot) => CaptureOutcome::Captured(snapshot),
                    Err(error) => CaptureOutcome::Failed(error),
                }
            };
        }
        if let Some(job) = self.held.take()
            && let Err(full) = self.replies.try_push(job)
        {
            self.hold(full.value);
        }
    }

    /// Hold `job` in the empty slot. Every caller has just found the slot empty, so the
    /// displaced value is `None`; forgetting it keeps any drop of a job off the render thread.
    fn hold(&mut self, job: Box<ResponseCaptureJob>) {
        let displaced = self.held.replace(job);
        debug_assert!(displaced.is_none(), "a held job is never displaced");
        core::mem::forget(displaced);
    }
}
// REALTIME_POLICY_END
