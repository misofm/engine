//! The control-protocol session: commands, events, source control and plan replacement.

use super::*;

/// One plan's live fader/mute, matrix/pan, input and send producers (#1256 D2, #1557 D4), kept with
/// its provider epoch.
///
/// `controls` is in `HostLiveControlHandles::strips` order: the tracks, then the submixes. A
/// producer's `input` is `Some` when the plan carries the input lane (#1556). `routes` is in
/// canonical route-ID order, one producer per route into a submix when the plan carries the route
/// lane (#1557). `commit_live` pushes a value-only transaction's records through them (#1257). The rings are `Arc`s shared with the
/// plan's consumers, so they outlive whichever of the plan and the epoch drops first, and the last
/// owner to drop frees them, on the control thread: `synchronize_plan_epochs` drops a reclaimed
/// plan before its provider.
pub(crate) struct StripLanes {
    pub(crate) controls: Box<[host_core::TrackControlProducer]>,
    /// How many leading entries of `controls` are tracks.
    pub(crate) track_count: usize,
    /// One producer per route into a submix, in canonical route-ID order; empty when the plan
    /// carries no route lane.
    pub(crate) routes: Box<[host_core::RouteControlProducer]>,
    /// One seven-word producer per route the plan gives the cell of a following route (#1652 D1),
    /// in canonical route-ID order; empty when the plan has none. A route is in `routes` or here,
    /// never in both (#1653).
    pub(crate) follow_routes: Box<[host_core::GraphFollowRouteControlProducer]>,
    /// The writer of the plan's jump-length cell (#1639 D1), held inline; every prepared plan has
    /// one, and preparation refuses a handle set without it.
    pub(crate) jump_lengths: host_core::GraphJumpLengthProducer,
}

/// One epoch's worth of host-owned source producers, strip live-control producers and effect
/// live-control producers.
///
/// The tables themselves live in `host-core`; this wrapper adds the epoch tag, the plan's state
/// inventory its successor is prepared from (issue #1273 D1), the epoch's own capi resource rows
/// (`capi`, which the live admission reads, #1257 D4) and the lifecycle counters the
/// structural-replacement tests observe.
///
/// Public only to the adapters' tests, under `test-support`; crate-private otherwise.
#[cfg_attr(not(feature = "test-support"), allow(unreachable_pub))]
pub struct ProviderEpoch {
    pub(crate) epoch: u64,
    pub(crate) sources: SourceControlSet,
    /// What this epoch's plan holds, for preparing its successor (issue #1273 D1).
    pub(crate) inventory: PlanStateInventory,
    /// The live fader and matrix producers of this epoch's plan (#1256 D2).
    pub(crate) strips: StripLanes,
    /// One live-control producer per prepared effect instance of this epoch's plan, console slots
    /// and inserts alike, in `HostLiveControlHandles::effect_controls` order (#1263 D2). A
    /// parametric EQ's producer also owns its prepared-target owner. `commit_live` writes a
    /// value-only transaction's effect values to their cells through them (#1264). As for the strip
    /// producers, the cells are `Arc`s shared with the plan's lanes, so the epoch may drop after
    /// its plan, and does: `synchronize_plan_epochs` drops a reclaimed plan before its provider.
    pub(crate) effects: Box<[host_core::EffectControlProducer]>,
    /// The consumers of this epoch's plan's strip meters (#1581 D6), one per strip in strip order
    /// (`host_core::meter_selection`). Nothing reads them until #1587; a full strip queue drops
    /// and counts on the render side. Their rings are `Arc`s shared with the plan's observers, as
    /// the producers' cells are, so the epoch may drop after its plan. At promotion a carried
    /// meter's consumer moves from the displaced epoch to this one, at the entry the plan's
    /// `meter_pairing` names (#1674 R1), so a consumer stays with the observer that renders on.
    pub(crate) meters: Box<[host_core::MeterConsumer]>,
    /// This epoch's plan's meter tables (#1587a R2), built with the plan for a control-plane meter
    /// delivery: held here while the epoch is pending, in the session's delivery while it renders
    /// (its promotion exchanges them by pointer, `MeterDelivery::adopt`), and here again once it
    /// is parked. The running epoch holds empty tables, which allocate nothing.
    pub(crate) meter_tables: host_core::MeterTables,
    /// The control half of this epoch's plan's meter frame ring (#1588 D2), for a render-half
    /// meter fold: the frames the render half published while the plan rendered, staged in epoch
    /// order (`stage_meter_batches`). `None` for any other adapter.
    pub(crate) meter_frames: Option<host_core::MeterFrameConsumer>,
    /// The render half of the same ring until the adapter takes it for the plan's render half
    /// ([`SessionState::take_meter_frame_producer`]).
    pub(crate) meter_frame_producer: Option<host_core::MeterFrameProducer>,
    /// The readers of this epoch's plan's effect observation taps (#1584 D7), one per observed
    /// effect instance in `HostLiveControlHandles::effect_observations` order: the windows a tap an
    /// `Observe` edit armed publishes. Empty for a host whose preparer keeps them itself (the
    /// browser's). Their slots are `Arc`s shared with the plan's lanes, as the meters' rings are.
    /// The meter delivery's gain reduction reads and acknowledges them (#1587b);
    /// `OBSERVATION_READ` reads the same slots through [`Self::observation_index`]'s read-only
    /// views (#1586 D6, #1332 S6b).
    pub(crate) observations: Box<[host_core::EffectObservationHandle]>,
    /// Each observation handle's strip (#1587b), in `observations` order: its frame strip by
    /// [`strip_meter_index`], the rule a meter entry's strip resolves by, `u32::MAX` for none;
    /// built with the plan.
    pub(crate) observation_strips: Box<[u32]>,
    /// The observation readers the swap block that adopted this epoch's plan carried (#1587b),
    /// shared with the plan's attachment (the control plane's, or the preparer's own, #1332 S6b);
    /// `None` for a plan with no reader or an adapter without a meter delivery.
    pub(crate) carried_taps: Option<std::sync::Arc<CarriedTaps>>,
    /// The epoch this one displaced, while their observation readers wait to be paired: this
    /// epoch was promoted before render wrote `carried_taps` ([`CarriedTaps`]'s three states).
    pub(crate) observations_pending: Option<u64>,
    /// The master strip of this epoch's plan (#1690 D2): its frame strip by
    /// [`strip_meter_index`], or `None` when no strip is the master or the plan has none of its
    /// ID. A master meter entry reads a frame through its own epoch's value. Kept by the browser's
    /// `sync_master` rule ([`sync_master_meters`]): a pending epoch takes every
    /// master the commits write, and the rendering epoch only while nothing waits, so a frame the
    /// rendering plan publishes after a waiting replacement moved the master reads its own.
    pub(crate) master_meter: Option<u32>,
    /// The plan's effect instances by stable key, with each one's position in
    /// [`Self::observations`] (#1586 D3, R1): built with the epoch, on the control thread, so a
    /// read resolves a stable address without allocating. For a host whose preparer keeps the
    /// readers in its plan (the browser's), the positions are in that list
    /// ([`crate::RuntimePreparer::kept_observations`], #1589 R2): the plan's one index. It holds a
    /// read-only view of every reader (#1332 S6b), paired with the readers from `carried_taps` at
    /// promotion, so the read runs on the control thread wherever the readers are.
    pub(crate) observation_index: host_core::observation::ObservationIndex,
    /// The capi resources this epoch's plan was prepared with (#1257 D4).
    pub(crate) capi: CapiResources,
    /// The handle of this epoch's plan's revision gate (#1502 D1). Control publishes every
    /// revision the plan carries on it, and #1312 stamps the plan's cell writes from it, so the
    /// gate, the cells and the producers that write them follow one routing. It stays here for
    /// the epoch's life and moves only with the epoch, which holds no open stamp between
    /// commands.
    pub(crate) revision_gate: PlanRevisionGate,
    /// The part of this epoch's plan's source rows it carries from the plan it was prepared to
    /// displace (#1273 D5), kept for the replacement admission while the epoch is a held
    /// candidate (#1398 D4, batch 15a review fold M2). Zero for the boot plan.
    pub(crate) carried: CarriedSourceBytes,
    /// This epoch's committed base while a successor prepared from it is pending (#1310 D3 and the
    /// batch 15 root ruling): the model its plan was prepared from with every live record pushed
    /// to it, the monitor overlay its cells hold, and the ramp lengths of every edit committed
    /// after its last revision. A successor of a withdrawn candidate is prepared against it.
    /// Set by the structural commit that displaces this epoch's model, extended by every later
    /// commit until this epoch stops being the running one, and dropped then.
    pub(crate) kept: Option<KeptBase>,
    /// This epoch's resources as the admissions charge them, captured when the epoch is parked in
    /// `retired_providers` (batch 15a review fold 2, M-1): its report row as it stood when it was
    /// promoted out, which no later commit changes, its capi resources and its carried bytes. The
    /// report table keeps no row for a parked epoch, but its plan lives until the service step
    /// reclaims it, so both admissions charge it from here until then. `None` while the epoch is
    /// running or pending.
    pub(crate) parked: Option<LiveEpochResources>,
    /// This epoch's plan was prepared at a host-declared discontinuity (#1323 D2): no floors, only
    /// its source consumers carried. A successor that supersedes it unadopted is prepared at the
    /// same discontinuity, since render never played this plan.
    pub(crate) discontinuity: bool,
    /// A candidate's spectrum captures until its promotion installs them (#1593 D5), and what
    /// that promotion displaced once this epoch is parked.
    pub(crate) spectrum: crate::spectrum::EpochSpectrum,
}

/// The committed base an epoch keeps while a successor prepared from it is pending (#1310 D3,
/// root ruling of 2026-10-15 on D2 and D3).
pub(crate) struct KeptBase {
    /// The compiled session the epoch's plan renders: the model it was prepared from plus every
    /// live record pushed to it. The structural commit that displaced it returned it. Both
    /// admissions charge it as a held model (#1398 D4).
    pub(crate) model: CompiledSession,
    /// The monitor overlay the epoch's cells hold: the session's overlay when the displacing
    /// commit landed. A later solo is written to the newer epoch's cells, not to these.
    pub(crate) monitor: controls::MonitorOverlay,
    /// The ramp length of every row an edit committed after the epoch's last revision gave one,
    /// the latest edit winning on every row it wrote (#1394 D2's rule across transactions).
    /// Pruned to the newest committed model's owners, with no rewritten targets, whenever it is
    /// stored (`EditRamps::keep_within`, #1696 root ruling): its bound is that model's, not this
    /// epoch's, and `CapiResources::edit_ramps` charges it so, `KEPT_BASES` times.
    pub(crate) edit_ramps: session::EditRamps,
    /// The ramp of the latest overlay change committed after the epoch's last revision, or `None`
    /// while no commit since wrote a monitor row: a replacement that writes none renders the
    /// overlay change between `monitor` and its own with it, as one transaction holding every
    /// write would (`MonitorChange::ramp`).
    pub(crate) monitor_ramp: Option<Option<u32>>,
    /// What phase 1 of a strip removal wrote to the epoch's cells while a successor waits (#1325
    /// D6): the strips and routes ramped to `+0.0`, the removed sources that still feed the
    /// epoch's plan, and `S`. Empty when no pending edit removed a strip. It drops with this base.
    pub(crate) duck: host_core::DuckOverlay,
}

/// The overlay of a rebuild that supersedes nothing: no strip ducks yet (#1325 D6).
static NO_DUCK: host_core::DuckOverlay = host_core::DuckOverlay::new();

/// Where phase 1's records land in the running plan's epoch (#1325 D2): one strip producer index
/// per strip record and one route producer index per route record, in record order.
struct DuckWrites {
    strips: Vec<usize>,
    routes: Vec<usize>,
}

/// Resolve every producer phase 1 writes in `held`, the running plan's epoch, and check the stamp
/// its writes take (#1325 D2, D8). Before the commit: every refusal is an internal fault, with
/// nothing written. The stamp check is the caller's last fallible step: it leaves the stamp's key
/// open on the gate, and [`write_duck`] takes the same key again.
fn resolve_duck_writes(
    held: &mut ProviderEpoch,
    transition: &host_core::StripTransition,
) -> Result<DuckWrites, CommandError> {
    let lanes = &held.strips;
    let controls = &lanes.controls;
    let mut strips = Vec::new();
    let mut routes = Vec::new();
    if strips
        .try_reserve_exact(transition.strip_records().len())
        .is_err()
        || routes.try_reserve_exact(transition.routes().len()).is_err()
    {
        return Err(CommandError::Internal);
    }
    for strip in transition.strip_records() {
        let index = host_core::strip_position(
            controls.len(),
            lanes.track_count,
            |index| &*controls[index].track_id,
            &strip.strip_id,
        )
        .ok_or(CommandError::Internal)?;
        let fader = &controls[index].fader;
        if strip
            .offsets
            .iter()
            .enumerate()
            .any(|(channel, record)| record.is_some() && !fader.has_offsets(channel))
        {
            return Err(CommandError::Internal);
        }
        strips.push(index);
    }
    for route in transition.route_records() {
        let index = match route.record {
            host_core::SendRecord::Folded(_) => {
                host_core::route_producer_position(&lanes.routes, route.route_id)
            }
            host_core::SendRecord::Following(record) => {
                host_core::follow_route_producer_for(&lanes.follow_routes, route.route_id, &record)
            }
        }
        .ok_or(CommandError::Internal)?;
        routes.push(index);
    }
    let last = held.revision_gate.last_published();
    if held.revision_gate.stamp_monitor(last).is_err() {
        return Err(CommandError::Internal);
    }
    Ok(DuckWrites { strips, routes })
}

/// Write phase 1's ramped mutes to the running plan's cells and publish them on its gate (#1325
/// D2): after the commit, and nothing here can fail. The stamp is a monitor publication under the
/// gate's last published revision ([`resolve_duck_writes`] checked it), so render applies the
/// writes at its next block and the gate still reports the revision it reported.
fn write_duck(
    held: &mut ProviderEpoch,
    transition: &host_core::StripTransition,
    writes: &DuckWrites,
) {
    let ProviderEpoch {
        strips: lanes,
        revision_gate,
        ..
    } = held;
    let last = revision_gate.last_published();
    let stamp = revision_gate
        .stamp_monitor(last)
        .expect("the phase-1 stamp was checked before the commit");
    let key = stamp.key();
    for (strip, &index) in transition.strip_records().iter().zip(&writes.strips) {
        let producer = &mut lanes.controls[index];
        for record in strip.fader_records() {
            producer.fader.write(stamp, record);
        }
        for (channel, record) in strip.offsets.iter().enumerate() {
            if let Some(record) = *record {
                producer.fader.write_offsets(stamp, channel, record);
            }
        }
    }
    for (route, &index) in transition.route_records().zip(&writes.routes) {
        match route.record {
            host_core::SendRecord::Folded(record) => lanes.routes[index].write(stamp, record),
            host_core::SendRecord::Following(record) => {
                lanes.follow_routes[index].write(stamp, record);
            }
        }
    }
    revision_gate.publish_key(key);
}

/// What a rebuild publishes a successor for (#1323).
enum Rebuild {
    /// A structural transaction: the successor is compiled from the token's prospective model and
    /// the protocol commit acknowledges it.
    Transaction(ObservedPreparedToken),
    /// A host-declared discontinuity (#1323 D2): the successor is the committed model, of which
    /// this is the control plane's own copy, prepared with no floors and only its source
    /// consumers carried. Nothing is committed.
    Discontinuity(Box<CompiledSession>),
    /// A `SPECTRUM_CONFIGURE` whose taps differ in form or entries (#1594 D6, R1): the successor
    /// is the committed model (the control plane's own copy) prepared with `configuration`, which
    /// replaces the session's. It carries like a transaction with no edit and commits like a
    /// discontinuity: nothing is committed, the revision does not change, no stamp is taken and
    /// the candidate's gate publishes the committed revision only when it is above 0.
    Monitor {
        model: Box<CompiledSession>,
        configuration: Option<host_core::SpectrumConfiguration>,
    },
    /// The transition fallback across a withdrawn `Primed` donor (#1397 D2): the committed model
    /// (the control plane's own copy) re-prepared as an ordinary successor of the plan the donor
    /// succeeds, with the duck set as its forced restarts, and published as a duck-swap that
    /// reports `TRANSITION_FALLBACK`. It carries like a transaction that supersedes the donor and
    /// commits like a monitor rebuild: nothing is committed and the revision does not change, but
    /// the candidate's gate is stamped with the committed revision, which the donor carried.
    Transition {
        model: Box<CompiledSession>,
        /// The injected refusal of this re-preparation (#1397 deliverable 3), taken from the
        /// thread's hook when the fallback starts.
        #[cfg(feature = "test-support")]
        injected_refusal: bool,
    },
}

/// The edit ramps of a discontinuity successor: no edit gave a row a length (#1323).
static NO_EDIT_RAMPS: session::EditRamps = session::EditRamps::new();

/// `earlier`'s ramp lengths with `later`'s laid over them: `later` wins on every row it wrote, with
/// or without a length (`EditRamps::overlay`). The
/// merge of the kept base's lengths with a newer transaction's (#1310, root ruling B15-Q3).
fn merged_ramps(earlier: &session::EditRamps, later: &session::EditRamps) -> session::EditRamps {
    let mut merged = earlier.clone();
    merged.overlay(later);
    merged
}

/// A structural successor's preparation failure as the command answers it (#1593 D4): a
/// `host.spectrum.target` refusal is an invariant fault, because the session prepares a successor
/// only with entries whose targets its model has; every other failure is the compile's own.
fn spectrum_invariant(failure: CompileFailure) -> CommandError {
    let refused = failure
        .diagnostic()
        .split(|byte| *byte == b'\n')
        .any(|line| line.starts_with(b"host.spectrum.target\t"));
    if refused {
        CommandError::Internal
    } else {
        CommandError::CompileRejected(failure)
    }
}

/// The plan a host lends one command ([`SessionState::command_with_plan`]).
struct LentPlan<'a, A: ControlAdapter> {
    plan: &'a mut PlanState<A>,
}

/// How a `SPECTRUM_CONFIGURE` is refused (#1594 D4): a typed reply, recorded for replay, or a
/// retryable or internal error that records nothing.
enum SpectrumRefusal {
    Reply(protocol::StatusCode, String),
    Error(CommandError),
}

impl SpectrumRefusal {
    fn reply(status: protocol::StatusCode, code: &str) -> Self {
        Self::Reply(status, code.to_owned())
    }
}

/// The record of every spectrum `BACKPRESSURE` refusal, which only `Busy` answers (#1944 D2,
/// #1595 D6): the capture's one-shot arm slot, one entry, held. A collection without a selection
/// also answers `Busy` and so reports the slot as held: a known imprecision, reopened only with D6.
const SPECTRUM_ARM_BACKPRESSURE: protocol::Backpressure = protocol::Backpressure {
    queue_kind: protocol::BackpressureQueueKind::SpectrumArm,
    capacity: 1,
    occupancy: 1,
    requested_items: 1,
    generation: None,
    retry_boundary: None,
    requested_bytes: None,
    available_bytes: None,
};

/// The configuration a `SPECTRUM_CONFIGURE` request sets, by #1591 D2's constructors: `None` for
/// an empty collection with budget 0. A constructor's refusal is `INVALID_FIELD` with its code.
fn spectrum_configuration_of(
    request: &protocol::SpectrumConfigureRequest,
) -> Result<Option<host_core::SpectrumConfiguration>, SpectrumRefusal> {
    let invalid = |code: &str| SpectrumRefusal::reply(protocol::StatusCode::InvalidField, code);
    let hop = request
        .hop_frames
        .map(host_core::SpectrumHop::new)
        .transpose()
        .map_err(|_| invalid("spectrum.hop"))?;
    let raw = |entry: &protocol::SpectrumEntry| {
        (
            u32::from(entry.target_kind.code()),
            match entry.channels {
                protocol::ParameterChannel::Left => host_core::SPECTRUM_CHANNEL_LEFT,
                protocol::ParameterChannel::Right => host_core::SPECTRUM_CHANNEL_RIGHT,
                protocol::ParameterChannel::Both => host_core::SPECTRUM_CHANNEL_BOTH,
            },
        )
    };
    let preparation = match request.form {
        protocol::SpectrumForm::Single => {
            let [entry] = request.entries.as_slice() else {
                return Err(invalid("spectrum.form"));
            };
            let (kind, channels) = raw(entry);
            Some(
                host_core::SpectrumPreparation::single(
                    kind,
                    &entry.target_id,
                    channels,
                    request.maximum_capture_bytes,
                )
                .map_err(|error| invalid(error.code()))?,
            )
        }
        protocol::SpectrumForm::Collection => {
            let mut entries = Vec::new();
            entries
                .try_reserve_exact(request.entries.len())
                .map_err(|_| invalid(host_core::SpectrumRequestError::Budget.code()))?;
            entries.extend(request.entries.iter().map(|entry| {
                let (kind, channels) = raw(entry);
                (kind, entry.target_id.as_str(), channels)
            }));
            host_core::SpectrumPreparation::collection(&entries, request.maximum_capture_bytes)
                .map_err(|error| invalid(error.code()))?
        }
    };
    Ok(preparation.map(|preparation| host_core::SpectrumConfiguration { preparation, hop }))
}

/// The configuration a `SPECTRUM_CONFIGURE` request sets against `model`, with every check the
/// command runs before any write (#1594 D4), in order: the request's own rules
/// ([`spectrum_configuration_of`]); each target names a strip or an output of `model` (`NOT_FOUND`,
/// `spectrum.target.unknown`); no entry twice (`VALIDATION_FAILED`, `spectrum.duplicate`); every
/// tap's reads fit the frame `limit` (#1595 R6, `LIMIT_EXCEEDED`, `spectrum.frame`). The command
/// and the boot (#2166) both run it, so a boot accepts exactly the configurations the command does.
fn checked_spectrum_configuration(
    request: &protocol::SpectrumConfigureRequest,
    model: &session::SessionModel,
    limit: u64,
) -> Result<Option<host_core::SpectrumConfiguration>, SpectrumRefusal> {
    use protocol::StatusCode;
    let configuration = spectrum_configuration_of(request)?;
    for entry in &request.entries {
        let id = entry.target_id.as_str();
        let present = match entry.target_kind {
            protocol::SpectrumTargetKind::TrackPostInput
            | protocol::SpectrumTargetKind::TrackPostPan => {
                model.tracks.iter().any(|track| track.id.as_str() == id)
                    || model.submixes.iter().any(|submix| submix.id.as_str() == id)
            }
            protocol::SpectrumTargetKind::Output => {
                model.outputs.iter().any(|output| output.id.as_str() == id)
            }
        };
        if !present {
            return Err(SpectrumRefusal::reply(
                StatusCode::NotFound,
                "spectrum.target.unknown",
            ));
        }
    }
    for (index, entry) in request.entries.iter().enumerate() {
        if request.entries[..index].contains(entry) {
            return Err(SpectrumRefusal::reply(
                StatusCode::ValidationFailed,
                "spectrum.duplicate",
            ));
        }
    }
    // #1595 R6: a tap whose one-plane one-shot read (without the window) or whose stream read
    // (of the tap's channels) the session's frame limit cannot carry would answer
    // `LIMIT_EXCEEDED` on every such read; it is refused here.
    if request
        .entries
        .iter()
        .any(|entry| !tap_fits(entry.channels, entry.target_id.len(), limit))
    {
        return Err(SpectrumRefusal::reply(
            StatusCode::LimitExceeded,
            "spectrum.frame",
        ));
    }
    Ok(configuration)
}

/// The spectrum configuration a boot's `SPECTRUM_CONFIGURE` sets (#2166), checked as the command
/// checks it ([`checked_spectrum_configuration`]) against the boot's model. A refusal is the
/// boot's: one `code\t$\n` line with the command's code.
pub(crate) fn boot_spectrum_configuration(
    request: &protocol::SpectrumConfigureRequest,
    model: &session::SessionModel,
    limit: u64,
) -> Result<Option<host_core::SpectrumConfiguration>, CompileFailure> {
    checked_spectrum_configuration(request, model, limit).map_err(|refusal| match refusal {
        SpectrumRefusal::Reply(_, code) => prepare_failure(host_core::PrepareDiagnostics::fixed(
            host_core::PrepareRejection::Session,
            &code,
        )),
        SpectrumRefusal::Error(_) => failure(ResourceFault::Platform),
    })
}

/// Whether two preparations hold the same taps (form, targets and channels), whatever their
/// budgets (#1594 D5).
fn same_taps(
    next: &host_core::SpectrumPreparation,
    current: &host_core::SpectrumPreparation,
) -> bool {
    use host_core::SpectrumPreparation::{Collection, Single};
    match (next, current) {
        (Single(next), Single(current)) => {
            next.target == current.target && next.channels == current.channels
        }
        (Collection(next), Collection(current)) => next.entries == current.entries,
        _ => false,
    }
}

/// A preparation's byte budget.
const fn spectrum_budget(preparation: &host_core::SpectrumPreparation) -> u64 {
    match preparation {
        host_core::SpectrumPreparation::Single(request) => request.maximum_capture_bytes,
        host_core::SpectrumPreparation::Collection(request) => request.maximum_capture_bytes,
    }
}

/// What a `SPECTRUM_CAPTURE` did, before its reply is built (#1595 D2).
enum CaptureOutcome {
    Done,
    Pending,
    /// The popped window, in the monitor's one-shot slot with its analysis.
    Ready {
        include_window: bool,
    },
}

/// A service or analysis refusal with its status (#1595 D6). `WindowInvalid` and
/// `MetadataInvalid` (outside D6's table) are `UNAVAILABLE`, as `CaptureInvalid` is.
fn spectrum_service_refusal(error: host_core::SpectrumServiceError) -> SpectrumRefusal {
    use host_core::SpectrumServiceError as E;
    use protocol::StatusCode;
    let status = match error {
        E::Unprepared | E::UnknownEntry => StatusCode::NotFound,
        E::Busy => StatusCode::Backpressure,
        E::Dropped | E::CaptureInvalid | E::WindowInvalid | E::MetadataInvalid => {
            StatusCode::Unavailable
        }
        E::Exhausted => StatusCode::LimitExceeded,
        E::SelectSingle
        | E::NotArmed
        | E::Channels
        | E::StreamActive
        | E::StreamInactive
        | E::CadenceRate
        | E::CadenceShape => StatusCode::ValidationFailed,
        E::Smoothing => StatusCode::InvalidField,
    };
    SpectrumRefusal::reply(status, error.code())
}

const fn spectrum_channels(channels: protocol::ParameterChannel) -> host_core::SpectrumChannels {
    match channels {
        protocol::ParameterChannel::Left => host_core::SpectrumChannels::Left,
        protocol::ParameterChannel::Right => host_core::SpectrumChannels::Right,
        protocol::ParameterChannel::Both => host_core::SpectrumChannels::Stereo,
    }
}

const fn protocol_channels(channels: host_core::SpectrumChannels) -> protocol::ParameterChannel {
    match channels {
        host_core::SpectrumChannels::Left => protocol::ParameterChannel::Left,
        host_core::SpectrumChannels::Right => protocol::ParameterChannel::Right,
        host_core::SpectrumChannels::Stereo => protocol::ParameterChannel::Both,
    }
}

fn spectrum_target_id(target: &host_core::SpectrumTarget) -> &str {
    match target {
        host_core::SpectrumTarget::TrackPostInputBuiltins(id)
        | host_core::SpectrumTarget::TrackPostMatrix(id)
        | host_core::SpectrumTarget::Output(id) => id,
    }
}

/// Whether a ready `SPECTRUM_CAPTURE` reply of this shape fits a session frame limit of `limit`
/// bytes (#1595 D5, R3): measured by the codec, so it cannot drift from the encoder.
fn reply_fits(
    channels: protocol::ParameterChannel,
    include_window: bool,
    target_id_bytes: usize,
    limit: u64,
) -> bool {
    protocol::ProtocolCodec::default()
        .spectrum_capture_ready_frame_len(channels, include_window, target_id_bytes)
        .is_ok_and(|bytes| u64::try_from(bytes).is_ok_and(|bytes| bytes <= limit))
}

/// Whether a ready `SPECTRUM_STREAM` reply of an entry of `channels` whose identity is
/// `target_id_bytes` long fits a session frame limit of `limit` bytes (#1595 D5, R3), as
/// [`reply_fits`] measures a capture.
fn stream_reply_fits(
    channels: protocol::ParameterChannel,
    target_id_bytes: usize,
    limit: u64,
) -> bool {
    protocol::ProtocolCodec::default()
        .spectrum_stream_ready_frame_len(channels, target_id_bytes)
        .is_ok_and(|bytes| u64::try_from(bytes).is_ok_and(|bytes| bytes <= limit))
}

/// #1595 R6: whether a session frame limit of `limit` bytes carries the reads every accepted tap
/// must have, for a tap of `channels` whose identity is `target_id_bytes` long: a one-plane
/// one-shot read without the window, and a stream read of the tap's own channels (the entry fixes
/// them; the stream reply also carries more scalar fields). A tap that fails would answer
/// `LIMIT_EXCEEDED` on every one-plane read, or on every stream read.
fn tap_fits(channels: protocol::ParameterChannel, target_id_bytes: usize, limit: u64) -> bool {
    reply_fits(
        protocol::ParameterChannel::Left,
        false,
        target_id_bytes,
        limit,
    ) && stream_reply_fits(channels, target_id_bytes, limit)
}

const fn protocol_target_kind(target: &host_core::SpectrumTarget) -> protocol::SpectrumTargetKind {
    match target {
        host_core::SpectrumTarget::TrackPostInputBuiltins(_) => {
            protocol::SpectrumTargetKind::TrackPostInput
        }
        host_core::SpectrumTarget::TrackPostMatrix(_) => protocol::SpectrumTargetKind::TrackPostPan,
        host_core::SpectrumTarget::Output(_) => protocol::SpectrumTargetKind::Output,
    }
}

/// What a `SPECTRUM_STREAM` did (#1595 D3): the stream's metadata after it, with the analysis
/// facts, and whether the monitor's result holds the bins of a ready read.
struct StreamOutcome {
    metadata: host_core::SpectrumStreamMetadata,
    ready: bool,
}

/// The analyzed window of a ready read: `result`'s words, borrowed until the reply is encoded,
/// and with `include_window` the popped window's planes of its mask.
fn capture_window<'a>(
    result: &'a host_core::SpectrumAnalysisResult,
    target: &'a host_core::SpectrumTarget,
    snapshot: &'a host_core::SpectrumSnapshot,
    include_window: bool,
) -> protocol::SpectrumCaptureWindow<'a> {
    use std::borrow::Cow;
    let left = matches!(
        result.channels,
        host_core::SpectrumChannels::Left | host_core::SpectrumChannels::Stereo
    );
    let right = matches!(
        result.channels,
        host_core::SpectrumChannels::Right | host_core::SpectrumChannels::Stereo
    );
    let plane = |held: bool, words: &'a [f32]| held.then_some(Cow::Borrowed(words));
    protocol::SpectrumCaptureWindow {
        target_kind: protocol_target_kind(target),
        target_id: Cow::Borrowed(spectrum_target_id(target)),
        channels: protocol_channels(result.channels),
        sample_rate_hz: result.sample_rate_hz,
        window_frames: protocol::SPECTRUM_CAPTURE_WINDOW_FRAMES,
        bin_count: protocol::SPECTRUM_CAPTURE_BIN_COUNT,
        floor_db: host_core::SPECTRUM_FLOOR_DB,
        captured_sample: result.captured_sample,
        end_sample: result.end_sample,
        snapshot_token: result.token,
        source_underrun: result.source_underrun,
        frequencies_hz: Cow::Borrowed(&result.frequencies_hz),
        left_db: plane(left, &result.left_dbfs),
        right_db: plane(right, &result.right_dbfs),
        left_window: plane(left && include_window, &snapshot.window.left),
        right_window: plane(right && include_window, &snapshot.window.right),
    }
}

/// The smoothed bins of a ready stream read: `result`'s words, borrowed until the reply is
/// encoded.
fn stream_bins(result: &host_core::SpectrumAnalysisResult) -> protocol::SpectrumStreamBins<'_> {
    use std::borrow::Cow;
    let left = matches!(
        result.channels,
        host_core::SpectrumChannels::Left | host_core::SpectrumChannels::Stereo
    );
    let right = matches!(
        result.channels,
        host_core::SpectrumChannels::Right | host_core::SpectrumChannels::Stereo
    );
    protocol::SpectrumStreamBins {
        frequencies_hz: Cow::Borrowed(&result.frequencies_hz),
        left_db: left.then_some(Cow::Borrowed(&result.left_dbfs)),
        right_db: right.then_some(Cow::Borrowed(&result.right_dbfs)),
    }
}

/// The first diagnostic code of a refused preparation: the text before the first tab of its
/// first line.
fn first_diagnostic_code(failure: &CompileFailure) -> String {
    let diagnostic = failure.diagnostic();
    let line = diagnostic
        .split(|byte| *byte == b'\n')
        .next()
        .unwrap_or_default();
    let code = line.split(|byte| *byte == b'\t').next().unwrap_or_default();
    String::from_utf8_lossy(code).into_owned()
}

impl ProviderEpoch {
    /// This epoch's tag: the plan epoch its producers feed.
    #[cfg(feature = "test-support")]
    pub fn test_epoch(&self) -> u64 {
        self.epoch
    }

    /// The source total and overhead bytes this epoch's plan carries from the plan it was
    /// prepared to displace (#1273 D5).
    #[cfg(feature = "test-support")]
    pub fn test_carried(&self) -> (u64, u64) {
        (self.carried.total, self.carried.overhead)
    }

    /// The edit ramps this epoch's kept base holds, or `None` while it keeps no base (#1696).
    #[cfg(feature = "test-support")]
    pub fn test_kept_ramps(&self) -> Option<&session::EditRamps> {
        self.kept.as_ref().map(|kept| &kept.edit_ramps)
    }

    /// What this epoch's plan holds, for preparing its successor (#1323's gates read its floors).
    #[cfg(feature = "test-support")]
    pub fn test_inventory(&self) -> &PlanStateInventory {
        &self.inventory
    }

    /// This epoch's source producers.
    #[cfg(feature = "test-support")]
    pub fn test_sources(&self) -> &SourceControlSet {
        &self.sources
    }

    /// How many leading strip producers are tracks.
    #[cfg(feature = "test-support")]
    pub fn test_track_count(&self) -> usize {
        self.strips.track_count
    }

    /// This epoch's strip producers: the tracks, then the submixes.
    #[cfg(feature = "test-support")]
    pub fn test_strip_controls(&self) -> &[host_core::TrackControlProducer] {
        &self.strips.controls
    }

    /// This epoch's effect producers.
    #[cfg(feature = "test-support")]
    pub fn test_effects(&self) -> &[host_core::EffectControlProducer] {
        &self.effects
    }

    /// This epoch's strip meter consumers (#1581 D6).
    #[cfg(feature = "test-support")]
    pub fn test_meters(&self) -> &[host_core::MeterConsumer] {
        &self.meters
    }

    /// This epoch's effect observation readers (#1584 D7).
    #[cfg(feature = "test-support")]
    pub fn test_observations(&self) -> &[host_core::EffectObservationHandle] {
        &self.observations
    }

    /// Whether this epoch's set holds source `id` vacant (`Some(true)`), holds its producer
    /// (`Some(false)`), or does not hold it (`None`) (#1310 gate 6). It probes with a submission
    /// at a sample rate of 1, which the set refuses before its ring sees it, so nothing changes.
    #[cfg(feature = "test-support")]
    pub fn test_source_vacant(&mut self, id: &[u8]) -> Option<bool> {
        let empty: [f32; 0] = [];
        match self.sources.submit(
            id,
            host_core::SourceSubmission {
                generation: 1,
                start_frame: 0,
                sample_rate_hz: 1,
                planes: &[&empty, &empty],
                frames: 0,
                end_of_region: false,
            },
        ) {
            Err(SourceControlError::UnknownSource) => None,
            Err(SourceControlError::Vacated) => Some(true),
            _ => Some(false),
        }
    }

    pub(crate) fn current(
        sources: SourceControlSet,
        inventory: PlanStateInventory,
        strips: StripLanes,
        effects: Box<[host_core::EffectControlProducer]>,
        meters: Box<[host_core::MeterConsumer]>,
        capi: CapiResources,
        revision_gate: PlanRevisionGate,
    ) -> Self {
        let owner = Self {
            epoch: 0,
            sources,
            inventory,
            strips,
            effects,
            meters,
            meter_tables: host_core::MeterTables::empty(),
            meter_frames: None,
            meter_frame_producer: None,
            observations: Box::default(),
            observation_strips: Box::default(),
            carried_taps: None,
            observations_pending: None,
            master_meter: None,
            observation_index: host_core::observation::ObservationIndex::default(),
            capi,
            revision_gate,
            carried: CarriedSourceBytes::default(),
            kept: None,
            parked: None,
            discontinuity: false,
            spectrum: crate::spectrum::EpochSpectrum::None,
        };
        #[cfg(feature = "test-support")]
        update_test_owners(|owners| owners.current_provider_constructed += 1);
        owner
    }

    /// This epoch with its plan's meter frame ring (#1588 D2), if it has one: the control half,
    /// and the render half unless the preparer moved it into the plan (#1332).
    pub(crate) fn with_meter_frames(
        mut self,
        ring: Option<(
            host_core::MeterFrameConsumer,
            Option<host_core::MeterFrameProducer>,
        )>,
    ) -> Self {
        if let Some((consumer, producer)) = ring {
            self.meter_frames = Some(consumer);
            self.meter_frame_producer = producer;
        }
        self
    }

    /// This epoch with its plan's observation readers (#1584 D7), their index (#1586 D6), their
    /// strips and the record of the ones its swap block carries (#1587b).
    pub(crate) fn with_observations(
        mut self,
        observations: Box<[host_core::EffectObservationHandle]>,
        observation_index: host_core::observation::ObservationIndex,
        observation_strips: Box<[u32]>,
        carried_taps: Option<std::sync::Arc<CarriedTaps>>,
    ) -> Self {
        self.observations = observations;
        self.observation_index = observation_index;
        self.observation_strips = observation_strips;
        self.carried_taps = carried_taps;
        self
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn candidate(
        sources: SourceControlSet,
        inventory: PlanStateInventory,
        carried: CarriedSourceBytes,
        strips: StripLanes,
        effects: Box<[host_core::EffectControlProducer]>,
        meters: Box<[host_core::MeterConsumer]>,
        capi: CapiResources,
        revision_gate: PlanRevisionGate,
    ) -> Self {
        let owner = Self {
            epoch: u64::MAX,
            sources,
            inventory,
            strips,
            effects,
            meters,
            meter_tables: host_core::MeterTables::empty(),
            meter_frames: None,
            meter_frame_producer: None,
            observations: Box::default(),
            observation_strips: Box::default(),
            carried_taps: None,
            observations_pending: None,
            master_meter: None,
            observation_index: host_core::observation::ObservationIndex::default(),
            capi,
            revision_gate,
            carried,
            kept: None,
            parked: None,
            discontinuity: false,
            spectrum: crate::spectrum::EpochSpectrum::None,
        };
        #[cfg(feature = "test-support")]
        update_test_owners(|owners| owners.candidate_provider_constructed += 1);
        owner
    }
}

impl Drop for ProviderEpoch {
    fn drop(&mut self) {
        #[cfg(feature = "test-support")]
        update_test_owners(|owners| {
            if self.epoch == 0 {
                owners.current_provider_disposed += 1;
            } else {
                owners.candidate_provider_disposed += 1;
            }
        });
    }
}

pub(crate) const RENDER_DIAGNOSTIC_SLOTS: usize = 2;
/// The render-activity diagnostic's code (#1309 D7). It keeps the C ABI's text because `host-core`
/// sizes the provider's render diagnostics by this literal and it is protocol-visible; choosing the
/// browser's code is #1332's.
pub(crate) const RENDER_DIAGNOSTIC_CODE: &str = "capi.render.activity";

/// A structural command's fault-injection point, in the order `command` reaches them.
#[cfg(feature = "test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TestStructuralFaultPhase {
    /// After the protocol prepared the token.
    AfterProtocolPrepare,
    /// Before the candidate runtime is prepared.
    BeforeRuntimePrepare,
    /// After the candidate runtime is prepared.
    AfterRuntimePrepare,
    /// After the replacement admission.
    AfterAdmission,
    /// After the plan replacement is reserved.
    AfterPlanReservation,
    /// Before the protocol commit.
    BeforeProtocolCommit,
    /// The protocol commit itself returns `Err` (issue #1273 gate 3).
    ProtocolCommit,
    /// The live arm, after every fallible check and before its first write (#1257 D7).
    BeforeLivePush,
    /// A panic inside a rebuild, after the pending candidate was withdrawn and the successor
    /// prepared, before admission (#2010).
    PanicAfterWithdrawal,
    /// A panic just after a rebuild publishes a warm candidate's `Primed` schedule, before the
    /// protocol commit (#2044): it stands in for every step there that can panic (that
    /// publication's `expect`, the commit's own), the last before the candidate is taken.
    PanicAtPrimedPublication,
}

/// How many of each owner the structural and live paths constructed, published and disposed.
#[cfg(feature = "test-support")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TestOwnerCounters {
    /// Lifecycle count: current provider constructed.
    pub current_provider_constructed: u64,
    /// Lifecycle count: current provider disposed.
    pub current_provider_disposed: u64,
    /// Lifecycle count: candidate provider constructed.
    pub candidate_provider_constructed: u64,
    /// Lifecycle count: candidate provider disposed.
    pub candidate_provider_disposed: u64,
    /// Lifecycle count: candidate provider published.
    pub candidate_provider_published: u64,
    /// Lifecycle count: current plan constructed.
    pub current_plan_constructed: u64,
    /// Lifecycle count: current plan disposed.
    pub current_plan_disposed: u64,
    /// Lifecycle count: candidate plan constructed.
    pub candidate_plan_constructed: u64,
    /// Lifecycle count: candidate plan disposed.
    pub candidate_plan_disposed: u64,
    /// Lifecycle count: candidate plan published.
    pub candidate_plan_published: u64,
    /// Lifecycle count: token constructed.
    pub token_constructed: u64,
    /// Lifecycle count: token disposed.
    pub token_disposed: u64,
    /// Lifecycle count: replay current constructed.
    pub replay_current_constructed: u64,
    /// Lifecycle count: replay current disposed.
    pub replay_current_disposed: u64,
    /// Lifecycle count: replay candidate constructed.
    pub replay_candidate_constructed: u64,
    /// Lifecycle count: replay candidate disposed.
    pub replay_candidate_disposed: u64,
    /// Lifecycle count: replay candidate published.
    pub replay_candidate_published: u64,
    /// Lifecycle count: reservation constructed.
    pub reservation_constructed: u64,
    /// Lifecycle count: reservation canceled.
    pub reservation_canceled: u64,
    /// Lifecycle count: reservation committed.
    pub reservation_committed: u64,
    /// Withdrawal outcome (#1310 D1): a structural command took a pending candidate back before
    /// render adopted it.
    pub candidate_withdrawn: u64,
    /// Withdrawal outcome (#1310 D1): render had adopted the pending candidate first.
    pub candidate_taken: u64,
    /// A withdrawn candidate published again, unchanged, because its replacement was refused
    /// (#1310 D4).
    pub candidate_republished: u64,
    /// A withdrawn candidate superseded: its replacement committed and was published, and it was
    /// dropped on the control thread (#1310 D5).
    pub candidate_superseded: u64,
}

/// One effect producer's row of `SessionControlSnapshot::effect_cells`: `(epoch, strip ID,
/// effect instance ID, writes per cell)`.
#[cfg(feature = "test-support")]
pub type EffectCellRow = (u64, Box<str>, Box<str>, Vec<u64>);

/// Everything a transaction may change, so a test can prove a refusal changed none of it.
#[cfg(feature = "test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestTransactionSnapshot<Row> {
    /// The committed session revision.
    pub revision: u64,
    /// The committed canonical JSON.
    pub canonical: Vec<u8>,
    /// The committed compiled model's resource estimate.
    pub model: session::ResourceEstimate,
    /// Replay cache entries.
    pub replay_entries: usize,
    /// The current provider epoch.
    pub provider_epoch: u64,
    /// Each pending provider's epoch.
    pub pending_provider_epochs: Vec<u64>,
    /// Each retired provider's epoch.
    pub retired_provider_epochs: Vec<u64>,
    /// The epoch the render thread last published.
    pub active_plan_epoch: u64,
    /// The plan resource table: each epoch's adapter row.
    pub resource_rows: Vec<(u64, Row)>,
    /// The reliable event queue.
    pub reliable_event: protocol::QueueReport,
    /// The reliable response queue.
    pub reliable_response: protocol::QueueReport,
    /// The telemetry queue.
    pub telemetry: protocol::QueueReport,
    /// The telemetry counters.
    pub telemetry_counters: protocol::TelemetryCounters,
    /// The controller's, provider's and replay cache's retained capacities.
    pub retained_capacities: [usize; 7],
    /// How many writes every strip producer's cells have taken, current epoch first, then each
    /// pending candidate: `(epoch, strip ID, fader left, fader right, mute left, mute right,
    /// matrix)`. A cell's count is its write sequence (#1312 D3).
    pub strip_cells: Vec<(u64, Box<str>, [u64; 4], u64)>,
    /// How many writes every strip producer's offsets cells have taken (#1649), in the order of
    /// `strip_cells`: `(epoch, strip ID, [left, right])`, `None` for a channel with no cell.
    pub strip_offsets_cells: Vec<(u64, Box<str>, [Option<u64>; 2])>,
    /// How many writes every send producer's cell has taken, current epoch first, then each
    /// pending candidate: `(epoch, route ID, writes)` (#1557).
    pub route_cells: Vec<(u64, Box<str>, u64)>,
    /// The same for every following route's seven-word cell (#1653), in the order of
    /// `route_cells`: `(epoch, route ID, writes)`.
    pub follow_route_cells: Vec<(u64, Box<str>, u64)>,
    /// How many writes every effect producer's bypass cell, parameter cells, then target cells,
    /// have taken, current epoch first, then each pending candidate: `(epoch, strip ID, effect
    /// instance ID, writes per cell)`. A cell's count is its write sequence (#1345 D1).
    pub effect_cells: Vec<EffectCellRow>,
    /// Every EQ owner's state, current epoch first, then each pending candidate: `(epoch, strip
    /// ID, effect instance ID, committed revision, phase, committed value bits, candidate value
    /// bits)`. The candidate is empty while the owner is idle: it exists only between `begin` and
    /// `commit` or `discard` (#1624).
    pub effect_owners: Vec<TestOwnerState>,
    /// The newest window each observation reader the epochs keep holds (#1584 D7), current epoch
    /// first, then each pending candidate: `(epoch, strip ID, effect instance ID, one window per
    /// declared tap as (sequence, first sample, blocks, the left and right lanes' bits))`, read by
    /// the reader itself. `None` for a tap that has published nothing.
    pub observation_windows: Vec<TestObservationWindows>,
}

/// One observed effect instance's newest windows in a [`TestTransactionSnapshot`].
#[cfg(feature = "test-support")]
pub type TestObservationWindows = (
    u64,
    Box<str>,
    Box<str>,
    Vec<Option<(u64, u64, u32, [u32; 2])>>,
);

/// One EQ owner's observable state in a [`TestTransactionSnapshot`].
#[cfg(feature = "test-support")]
pub type TestOwnerState = (u64, Box<str>, Box<str>, u64, String, Vec<u32>, Vec<u32>);

#[cfg(feature = "test-support")]
thread_local! {
    static TEST_FAULT_STATE: core::cell::Cell<([Option<TestStructuralFaultPhase>; 2], usize)> =
        const { core::cell::Cell::new(([None; 2], 0)) };
    static TEST_PUBLISHED_ADOPTION: core::cell::Cell<Option<PlanAdoption>> =
        const { core::cell::Cell::new(None) };
    static TEST_NEXT_ADOPTION: core::cell::Cell<Option<PlanAdoption>> =
        const { core::cell::Cell::new(None) };
    /// The source-read offset the next plan this thread prepares is given (#1396 deliverable 3).
    static TEST_NEXT_SOURCE_READ_OFFSET: core::cell::Cell<Option<u64>> =
        const { core::cell::Cell::new(None) };
    static FORCE_FULL_PREPARATION: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
    /// Whether the next transition re-preparation across a donor refuses (#1397 deliverable 3).
    static TEST_REFUSE_REPREPARE: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
    /// The strips the next `warm_lead` answer restarts whole with a zero lead, in place of the
    /// answer itself (#1397a: no edit of the gates' session family reaches that answer).
    static TEST_NEXT_ZERO_LEAD: core::cell::RefCell<Option<Vec<&'static str>>> =
        const { core::cell::RefCell::new(None) };
    /// Whether the service step leaves displaced plans in the retirement queue (batch 15a review
    /// fold M1): a render preempted between claiming a candidate and retiring the plan it displaced.
    static TEST_HOLD_RECLAMATION: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
    /// The key the newest epoch's gate had last published when this thread last queued an
    /// observation record (#1968): a commit's records are queued before its publication.
    static TEST_OBSERVE_PUSH_KEY: core::cell::Cell<Option<engine::realtime::GateKey>> =
        const { core::cell::Cell::new(None) };
    /// `(commits made from an in-place token, in-place tokens materialized)` on this thread.
    static IN_PLACE_COUNTS: core::cell::Cell<(usize, usize)> = const { core::cell::Cell::new((0, 0)) };
    static TEST_OWNER_STATE: core::cell::Cell<TestOwnerCounters> =
        const { core::cell::Cell::new(TestOwnerCounters {
            current_provider_constructed: 0,
            current_provider_disposed: 0,
            candidate_provider_constructed: 0,
            candidate_provider_disposed: 0,
            candidate_provider_published: 0,
            current_plan_constructed: 0,
            current_plan_disposed: 0,
            candidate_plan_constructed: 0,
            candidate_plan_disposed: 0,
            candidate_plan_published: 0,
            token_constructed: 0,
            token_disposed: 0,
            replay_current_constructed: 0,
            replay_current_disposed: 0,
            replay_candidate_constructed: 0,
            replay_candidate_disposed: 0,
            replay_candidate_published: 0,
            reservation_constructed: 0,
            reservation_canceled: 0,
            reservation_committed: 0,
            candidate_withdrawn: 0,
            candidate_taken: 0,
            candidate_republished: 0,
            candidate_superseded: 0,
        }) };
}

#[cfg(feature = "test-support")]
pub(crate) fn update_test_owners(update: impl FnOnce(&mut TestOwnerCounters)) {
    TEST_OWNER_STATE.with(|state| {
        let mut value = state.get();
        update(&mut value);
        state.set(value);
    });
}

#[cfg(feature = "test-support")]
pub(crate) fn take_test_fault_state(phase: TestStructuralFaultPhase) -> bool {
    TEST_FAULT_STATE.with(|state| {
        let (faults, index) = state.get();
        let matched = faults.get(index).copied().flatten() == Some(phase);
        if matched {
            state.set((faults, index + 1));
        }
        matched
    })
}

/// Clears the thread's fault plan and lifecycle counters.
#[cfg(feature = "test-support")]
pub fn test_reset_lifecycle_observer() {
    TEST_FAULT_STATE.with(|state| state.set(([None; 2], 0)));
    TEST_OWNER_STATE.with(|state| state.set(TestOwnerCounters::default()));
    TEST_NEXT_ADOPTION.with(|state| state.set(None));
    TEST_PUBLISHED_ADOPTION.with(|state| state.set(None));
    TEST_NEXT_SOURCE_READ_OFFSET.with(|state| state.set(None));
    TEST_HOLD_RECLAMATION.with(|state| state.set(false));
    TEST_REFUSE_REPREPARE.with(|state| state.set(false));
    TEST_NEXT_ZERO_LEAD.with(|state| state.replace(None));
}

/// Takes the source-read offset [`SessionState::test_set_next_source_read_offset`] scheduled for
/// the next plan this thread prepares (#1396 deliverable 3), once.
#[cfg(feature = "test-support")]
pub(crate) fn take_test_next_source_read_offset() -> Option<u64> {
    TEST_NEXT_SOURCE_READ_OFFSET.with(core::cell::Cell::take)
}

/// Makes this thread's control commands prepare their transactions by the full path (#1613): a
/// clone of the model with the edits applied, compiled, as before the in-place preparation. A test
/// runs one script both ways and compares the two sessions.
#[cfg(feature = "test-support")]
pub fn test_force_full_preparation(force: bool) {
    FORCE_FULL_PREPARATION.with(|state| state.set(force));
}

/// Makes this thread's service steps leave every displaced plan in the retirement queue while
/// `hold` is set (batch 15a review fold M1): the control side then sees what it sees while render,
/// having claimed a candidate, has not yet retired the plan it displaced. A test interleaves a
/// structural edit with that window deterministically; clearing it lets the next step reclaim.
#[cfg(feature = "test-support")]
pub fn test_hold_reclamation(hold: bool) {
    TEST_HOLD_RECLAMATION.with(|state| state.set(hold));
}

/// Whether a live commit on this thread checks, in a debug build, that its catalog stage equals a
/// full build of the committed model (on by default; [`host_core::verify_stages`]). The check
/// builds the whole catalog, so a test that counts the allocations of a command turns it off.
#[cfg(feature = "test-support")]
pub fn test_verify_catalog_stages(on: bool) {
    host_core::verify_stages(on);
}

/// How many commits this thread made from a token prepared in place, and how many such tokens it
/// had to materialize into the full preparation, since the last call (#1613), and resets both.
#[cfg(feature = "test-support")]
pub fn test_take_in_place_counts() -> (usize, usize) {
    IN_PLACE_COUNTS.with(core::cell::Cell::take)
}

/// The thread's lifecycle counters.
#[cfg(feature = "test-support")]
pub fn test_lifecycle_counters() -> TestOwnerCounters {
    TEST_OWNER_STATE.with(core::cell::Cell::get)
}

/// The most plans the control plane can hold unreclaimed (#1398 D4; batch 15a review fold 2,
/// M-1): every parked epoch, the running one and every pending candidate.
const HELD_EPOCHS: usize = RETIRED_PROVIDERS + 1 + PENDING_PROVIDERS;

/// Every plan the control plane holds unreclaimed, oldest first: the parked epochs, the running
/// plan, the pending candidates (#1398 D4), without an allocation.
pub(crate) struct HeldEpochs {
    rows: [LiveEpochResources; HELD_EPOCHS],
    len: usize,
}

impl HeldEpochs {
    pub(crate) fn as_slice(&self) -> &[LiveEpochResources] {
        &self.rows[..self.len]
    }
}

/// Every compiled model the control plane holds (#1398 D4, #1310 D3): the committed one, then the
/// model each held epoch keeps for a pending successor, without an allocation.
pub(crate) struct HeldModels {
    rows: [session::ResourceEstimate; 2 + PENDING_PROVIDERS],
    len: usize,
}

impl HeldModels {
    pub(crate) fn as_slice(&self) -> &[session::ResourceEstimate] {
        &self.rows[..self.len]
    }
}

/// One session's control plane: the control-protocol controller, the provider epochs, the plan
/// exchange's control ends and the shared plan projection, generic over the adapter (#1309 D4).
pub struct SessionState<A: ControlAdapter> {
    pub(crate) controller: ObservedController,
    pub(crate) providers: ProviderEpoch,
    pub(crate) pending_providers: Vec<ProviderEpoch>,
    pub(crate) retired_providers: Vec<ProviderEpoch>,
    pub(crate) publisher: PlanPublisher,
    pub(crate) retirer: PlanRetirer,
    pub(crate) limits: ControlLimits,
    /// The adapter's preparation (issue #1400 D2): the boot plan and every structural successor
    /// are prepared through it.
    pub(crate) preparer: Box<dyn RuntimePreparer>,
    /// The session's `live_values_superseded` count (#1312 D2): one for every plan the session
    /// prepares. Render adds to it; the control plane copies it into the provider's counters in
    /// the service step (D8).
    pub(crate) live_values_superseded: host_core::LiveValuesSuperseded,
    /// The session's `strips_restarted` count (#1768): for every plan render adopted, the strips
    /// that plan started at rest ([`PlanStateInventory::restarted_strips`]). It grows only where
    /// a pending provider is promoted, which happens only once render has adopted its plan, so a
    /// withdrawn candidate adds nothing; the control plane copies it into the provider's counters
    /// with `live_values_superseded`. Render does not touch it.
    pub(crate) strips_restarted: u64,
    /// The session's `transition_reprepare_refusals` count (#1397 D2 step 3): every transition
    /// re-preparation across a withdrawn donor that was refused, a defect (the donor is kept
    /// whole and republished). Saturating; copied into the provider's counters with
    /// `strips_restarted`.
    pub(crate) transition_reprepare_refusals: u64,
    /// The session's `source_frames_discarded` count (#1699 D1): the accepted source frames
    /// render never read because the caller's transaction removed or redeclared their source.
    /// [`Self::synchronize_plan_epochs`] adds each retired plan's unread frames before the plan is
    /// dropped (a carried ring left that plan's entry vacant, so it adds 0); session destroy adds
    /// nothing. Monotone, saturating, never reset. Render does not touch it.
    pub(crate) source_frames_discarded: u64,
    /// `u16` scratch the protocol decoder writes field offsets into. It is exactly
    /// `max_frame_bytes / 2` entries; the odd trailing byte of an odd frame limit used to buy a
    /// one-byte box that nothing ever read (audit F7).
    pub(crate) decode_fields: Box<[u16]>,
    pub(crate) response_scratch: Box<[u8]>,
    /// The rows of an `OBSERVATION_READ` reply (#1586 D6): one per selection the codec admits, made
    /// at boot for an adapter that serves the read (empty otherwise), so a read writes its rows
    /// here and allocates none.
    pub(crate) observation_rows: Box<[protocol::ObservationReadRow]>,
    pub(crate) shared: Arc<SharedPlanState<A::Row>>,
    pub(crate) observed_render_sequence: u64,
    /// The render sample of [`Self::observed_render_sequence`].
    pub(crate) observed_render_sample: u64,
    /// The committed revision when [`Self::observed_render_sequence`] was observed.
    pub(crate) observed_render_revision: protocol::SessionRevision,
    /// The render sequence the newest staged render diagnostic carries, or that no diagnostic is
    /// owed for (diagnostics off). A block observed while both diagnostic slots are full stays
    /// owed: the next service step with a free slot stages the newest observed block (#2198).
    pub(crate) diagnosed_render_sequence: u64,
    /// Owed render diagnostics a newer block's replaced (#2198), served in `TELEMETRY_COALESCED`
    /// beside the controller's own telemetry coalescing.
    pub(crate) render_diagnostics_coalesced: u64,
    /// The render sequence at which the counter snapshot was last staged, or while no counter is
    /// configured the latest one the service step saw: the baseline of the configured period
    /// (#1351 D4).
    pub(crate) counter_snapshot_sequence: u64,
    pub(crate) render_diagnostics: Box<[RenderDiagnosticSlot]>,
    pub(crate) render_diagnostic_head: usize,
    pub(crate) render_diagnostic_len: usize,
    pub(crate) protocol_reliable_pending: bool,
    /// The monitor overlay (decision 17, C8, #1559 D1): the soloed strips. Host-local, never saved
    /// and never in a snapshot; empty at boot. The newest plan's cells hold it (D10), the control
    /// catalog shows it (D11), and it is written only after every fallible step of the
    /// transaction that sets it has passed (D8).
    pub(crate) monitor: controls::MonitorOverlay,
    /// The observation taps the host has armed (#1584 D6a), kept beside the monitor overlay and
    /// apart from it: solo and subscriptions are different things. Host-local, never saved and
    /// never in a snapshot; empty at boot. Written after a successful push to a tap's queue, and
    /// by a rebuild with the commit, which drops each binding whose target the transaction removed
    /// and tells the subscribers. Every successor is prepared from it, so a subscription the host
    /// acknowledged survives a rebuild that restarts its effect.
    pub(crate) observation: controls::ObservationBindings,
    /// The subscription state each observation slot of the newest plan was last pushed, waiting
    /// to be folded into [`Self::observation`] (#1584 D6a). The browser admits a subscription in
    /// the worklet's message handler between render quanta, on the audio thread, where nothing may
    /// allocate, and a binding owns its key's text, so the
    /// push records the absolute state here, in storage sized when the plan is published, and the
    /// next service step folds it into the table. A rebuild folds first: it only starts from
    /// [`Self::command`], which services.
    pub(crate) observation_pending: Box<[ObservationSlot]>,
    /// For each position of the newest plan's effect producers, the first of its slots in
    /// `observation_pending`, and one entry past the last: a producer's slots are
    /// `offsets[position]..offsets[position + 1]`, one per declared tap in declaration order.
    pub(crate) observation_offsets: Box<[u32]>,
    /// How many of [`Self::observation_pending`]'s slots are armed now, pushes not yet folded
    /// included.
    pub(crate) observation_armed: usize,
    /// The encoded bytes of the armed slots' keys in an `OBSERVATION_ENDED` event, pushes not yet
    /// folded included: the figure a subscription is admitted against (#1685 D1). With the frame's
    /// own bytes, never above the control frame limit.
    pub(crate) observation_armed_bytes: usize,
    /// The encoded size of an `OBSERVATION_ENDED` frame as a function of its keys, and the control
    /// frame limit it must fit, so the commit that ends every binding can always say so (#1685
    /// D1). Fixed by the session's frame limit when it compiles.
    pub(crate) observation_wire: crate::compile::ObservationWire,
    /// For each position of the newest plan's effect producers, the records of the batch being
    /// admitted that name it (#1676, batch 13b fold): the room check reads one count per producer
    /// instead of counting the batch again for each record. Sized with
    /// [`Self::observation_offsets`], when the plan's slots are built, so admission allocates
    /// nothing.
    pub(crate) observation_batch_counts: Box<[ObservationBatchCount]>,
    /// The stamp of the batch being admitted: a slot's or a producer count's `mark` equal to it
    /// belongs to this batch, any other value to an earlier one, so the scratch needs no clearing
    /// between batches.
    pub(crate) observation_batch_mark: u32,
    /// The most observation slots any committed plan has had; the ended-event cells and the
    /// binding table are charged for it (#1676 D2).
    pub(crate) observation_slots_high_water: usize,
    /// The most owners one strip of any model this session committed has had (#1577): the
    /// response capture is charged at it, since a job or result sized under a displaced model can
    /// outlive the rebuild that displaced it.
    pub(crate) capture_owners_high_water: usize,
    /// The next arm tag this session issues (#1585 D4): `1` at compile, one more per queued
    /// observation record, so tags are monotone per session and `0` is never one. A record is
    /// tagged when it is queued, after its batch was admitted, so a refused submission issues none.
    pub(crate) next_arm_tag: u64,
    /// The live send moves that may still ramp, as one conservative window (root ruling B19-R7,
    /// the B17-R14 pattern; #1718 deletes it): `None` when none may.
    pub(crate) send_moves: Option<SendMoveWindow>,
    /// The session's spectrum monitor state (#1593 D1): host-local, never saved, never in a
    /// snapshot, and no change of it advances the revision. Every plan is prepared with its
    /// configuration.
    pub(crate) spectrum: crate::spectrum::SpectrumMonitor,
    /// The control half of the session's output loudness meter (#2115): host-local monitor
    /// state, never saved and in no revision, drained by [`Self::service`] and before every
    /// `LOUDNESS` reply.
    pub(crate) loudness: crate::loudness::LoudnessMonitor,
    /// The output meter's reader and gate (#1581 D4) until the adapter takes them: the browser's
    /// render half does. `None` from boot when the session serves the meter batches itself
    /// ([`Self::meters`]).
    pub(crate) output_meter: Option<(host_core::OutputPeakReader, host_core::OutputPeakGate)>,
    /// The response capture's delivered result, as its `RESPONSE_READY` event, while the
    /// controller's one cell cannot take it yet (#1578 D4): `service` retries until it is queued.
    pub(crate) response_ready: Option<PendingResponse>,
    /// The session's meter delivery when its control plane serves the meter batches (#1583 D5,
    /// R2: the C ABI's), built at boot over the output meter's reader and a requested gate.
    pub(crate) meters: Option<SessionMeters>,
    /// The session's warm-successor bounds (#1403 D1), set when the session is created: with
    /// `None`, no rebuild is classified and every rebuild takes today's path. Every product
    /// session holds `None` until #1360 and #1361 build the production bounds.
    pub(crate) warm: Option<host_core::WarmConfig>,
    /// The record of the pending candidate published `Primed` (#1403 D4), at most one because at
    /// most one candidate is pending. Set by the publication; dropped when render adopts it
    /// (`synchronize_plan_epochs`), by a withdrawal that does not return its candidate
    /// (`withdraw_pending_candidate`), and when a supersession or a declared stop drops the
    /// candidate (#1310 D5 step 6). A refused supersession republishes the candidate and keeps it.
    pub(crate) primed: Option<host_core::PrimedCandidate>,
}

/// A delivered response capture that `service` has not yet queued (#1578 D4).
pub(crate) enum PendingResponse {
    /// The `RESPONSE_READY` event of a strip `RESPONSE_GET`, waiting for the controller's cell.
    Event(protocol::ResponseReady),
    /// Test seam (#1577): a capture armed by [`SessionState::test_arm_track_response`], which
    /// issued no protocol tag and so owes no event.
    #[cfg(feature = "test-support")]
    SeamArmed,
    /// Test seam (#1577): the seam-armed capture's result, until the test takes it.
    #[cfg(feature = "test-support")]
    SeamResult(host_core::ResponseReady),
}

/// The meter batches a control plane serves (#1583 D5, #1587a, #1588 D4): where its frames come
/// from, and the one fold of them into batches that both sources share.
pub(crate) struct SessionMeters {
    source: MeterSource,
    batches: MeterBatches,
}

/// Where a control plane's meter frames come from (#1588 R1). One per session, inline in the
/// session as the delivery was before, so the variants' size difference costs no allocation.
#[allow(clippy::large_enum_variant)]
enum MeterSource {
    /// The C ABI's: the session's own delivery over the output meter and the strip meters of the
    /// epoch whose tables it holds.
    Delivery {
        delivery: host_core::MeterDelivery,
        /// The provider epoch whose meter tables the delivery holds (#1587a): the plan whose
        /// frames it folds. It follows render one reclaimed plan at a time (`adopt_meters`), and
        /// the poll runs only while it is the running epoch.
        epoch: u64,
    },
    /// The browser's: the render half folds and pushes the frames the table needs into each
    /// plan's frame ring, which its provider epoch holds; the session sets the level word's
    /// [`host_core::MeterLevel::TELEMETRY`] bit.
    Rendered {
        level: host_core::MeterLevel,
        /// The output meter's dropped-window count, whose reader the render half holds.
        output_drops: host_core::OutputPeakDrops,
        /// The windows the rings of reclaimed epochs dropped.
        retired_drops: u64,
    },
}

/// The fold of a control plane's meter frames into `METER_BATCH` events (#1583 D5, #1587a), the
/// same for both sources.
struct MeterBatches {
    fold: host_core::meters::MeterBatchFold,
    /// The provider epoch `resolution` was resolved against (#1588 D3), `None` after a configure
    /// and after an epoch's master moves (#1690 D2): a frame of another epoch, or the next frame
    /// after either, resolves the entries again first.
    resolved_epoch: Option<u64>,
    /// Each table entry's strip in `resolved_epoch`'s plan (#1587a D2, R4): its index in the
    /// frame's strip peaks, or `None` for an absent strip and for every entry that reads no strip;
    /// a master entry's is the master strip's (#1690 D2). Rebuilt at a frame of another epoch, never searched by a batch. Reserved at boot for
    /// the most entries a table holds, so no rebuild allocates.
    resolution: Vec<Option<u32>>,
    /// Each table entry's peak fold over the open batch, as long as `resolution`.
    folds: Vec<host_core::meters::StripPeakFold>,
    /// The meter window in blocks, which the provider echoes and the period divides.
    window_blocks: core::num::NonZeroU32,
    /// The controller's telemetry generation the level and the fold were set for.
    generation: u64,
    /// The windows of completed batches the lossy lane refused to stage.
    unstaged: u64,
}

impl SessionMeters {
    /// A delivery over the boot epoch's tables, with room for `entries` table entries.
    pub(crate) fn new(
        delivery: host_core::MeterDelivery,
        window_blocks: core::num::NonZeroU32,
        entries: usize,
    ) -> Result<Self, std::collections::TryReserveError> {
        Ok(Self {
            source: MeterSource::Delivery { delivery, epoch: 0 },
            batches: MeterBatches::new(window_blocks, entries)?,
        })
    }

    /// The batches of a render-half fold (#1588 R1) under `level`, reporting `output_drops`, with
    /// room for `entries` table entries.
    pub(crate) fn rendered(
        level: host_core::MeterLevel,
        output_drops: host_core::OutputPeakDrops,
        window_blocks: core::num::NonZeroU32,
        entries: usize,
    ) -> Result<Self, std::collections::TryReserveError> {
        Ok(Self {
            source: MeterSource::Rendered {
                level,
                output_drops,
                retired_drops: 0,
            },
            batches: MeterBatches::new(window_blocks, entries)?,
        })
    }

    /// Every closed meter window that never reached a `METER_BATCH` (#1583 D5, #1588 R4): the
    /// output windows render dropped at a full queue, the windows a full ring dropped (`rings`, the
    /// live epochs' rings' count, plus the reclaimed ones'), the windows of partial batches the
    /// fold discarded (at a gap, or when a nonempty configuration replaced the one it folded
    /// for), and the windows of batches the lane refused. When a configure empties the table,
    /// neither the windows the delivery pops unread nor the open partial batch are counted: the
    /// client turned the meters off.
    ///
    /// `unstaged` is defensive: no service step can see a refusal. The step runs at the start of
    /// every session call, before a structural token exists (none outlives its call), so
    /// `stage_meter_batch_event` is never refused as structural-outstanding; the level is on only
    /// with a nonempty table, which is not `Disabled`; and an accepted table's batch encodes,
    /// because the provider checks the 256-record bound at the configure (`meter.records`,
    /// #1587a). A stage that is refused all the same counts its batch's windows here.
    fn windows_lost(&self, rings: u64) -> u64 {
        let render = match &self.source {
            MeterSource::Delivery { delivery, .. } => delivery.output_dropped_windows(),
            MeterSource::Rendered {
                output_drops,
                retired_drops,
                ..
            } => output_drops
                .get()
                .saturating_add(*retired_drops)
                .saturating_add(rings),
        };
        render
            .saturating_add(self.batches.fold.discarded_windows())
            .saturating_add(self.batches.unstaged)
    }
}

impl MeterBatches {
    fn new(
        window_blocks: core::num::NonZeroU32,
        entries: usize,
    ) -> Result<Self, std::collections::TryReserveError> {
        let mut resolution = Vec::new();
        resolution.try_reserve_exact(entries)?;
        let mut folds = Vec::new();
        folds.try_reserve_exact(entries)?;
        Ok(Self {
            fold: host_core::meters::MeterBatchFold::new(core::num::NonZeroU32::MIN),
            resolved_epoch: None,
            resolution,
            folds,
            window_blocks,
            generation: 0,
            unstaged: 0,
        })
    }

    /// Resolve every entry of `table` against `meters`, the meter readers of the epoch whose
    /// frames are folded (`PlanStateInventory::meter_targets`: meter `i` is the frame's strip `i`;
    /// `None` when no epoch held is that one, and then no entry has a strip): a strip's peak or gain
    /// reduction entry (#1587b) reads the first post-matrix sample-peak meter of its strip, so a caller selection (a
    /// subset, another order, several taps on one strip) never reads another meter's peak. A
    /// master's gain reduction entry (#1690 D2, D3) reads `master`, the master strip the epoch
    /// holds ([`ProviderEpoch::master_meter`], resolved by the same rule): no strip when no strip
    /// is the master or the epoch has none of that ID. Then
    /// size the folds to the table. The folds keep their
    /// values: they are indexed by table entry, so a batch an adoption interrupts goes on folding
    /// into them, and `push_with_strips` resets them at a batch's first frame. A table never
    /// holds more entries than were reserved, so this allocates nothing.
    fn resolve(
        &mut self,
        table: &[protocol::MeterEntry],
        meters: Option<&host_core::PlanStateInventory>,
        master: Option<u32>,
    ) {
        self.resolution.clear();
        self.resolution.extend(
            table
                .iter()
                .map(|entry| match (&entry.target, entry.metric) {
                    (protocol::MeterTarget::Strip(id), _) => strip_meter_index(meters?, id),
                    (protocol::MeterTarget::Master, protocol::MeterMetric::GainReduction) => master,
                    _ => None,
                }),
        );
        self.folds.resize(
            self.resolution.len(),
            host_core::meters::StripPeakFold::START,
        );
    }

    /// Fold one published `frame` of provider epoch `epoch` and stage the `METER_BATCH` it
    /// completes (#1583 D5, #1587a, #1588 D4), the one staging both sources share. A frame of
    /// another epoch than the last one resolved resolves the entries against `epochs`' strips of
    /// its epoch first (#1588 D3), a master entry with that epoch's own master (#1690 D2).
    ///
    /// Records follow the table (D3): the output's peak entry gives the batch's left and right
    /// output peak, `VALID` when the output peak covered the batch's span; a strip's peak entry
    /// gives its strip's left and right peak, the maxima over the batch's frames, `VALID` when
    /// every frame had the strip; `CLIPPED` marks a valid peak above `1.0`. A strip's gain
    /// reduction entry gives one `Aggregate` record (#1587b), the last frame's reading that is
    /// present, `VALID` when any frame's is: only an armed tap's window from at or after its arm
    /// sample is a reading. The master's gain reduction entry (#1690 D3) is the same record of the
    /// master strip. A value that is not valid is `0.0` with flags `0`. An accepted table needs at most
    /// [`host_core::MAXIMUM_METER_BATCH_RECORDS`] records, so the records stay on the stack.
    fn stage(
        &mut self,
        frame: &host_core::MeterFrame,
        epoch: u64,
        epochs: &ProviderEpochs<'_>,
        controller: &mut ObservedController,
        revision: protocol::SessionRevision,
    ) {
        /// `METER_BATCH` record flags (registry): valid and clipped.
        const VALID: u16 = 1;
        const CLIPPED: u16 = 1 << 1;
        if self.resolved_epoch != Some(epoch) {
            self.resolve(
                &controller.telemetry_configuration().meters,
                epochs.meters(epoch),
                epochs.master(epoch),
            );
            self.resolved_epoch = Some(epoch);
        }
        let Some(batch) = self
            .fold
            .push_with_strips(frame, &self.resolution, &mut self.folds)
        else {
            return;
        };
        let blank = protocol::MeterRecord {
            handle: 0,
            component: protocol::MeterComponent::Left,
            flags: 0,
            value: 0.0,
        };
        let mut records = [blank; host_core::MAXIMUM_METER_BATCH_RECORDS];
        let table = &controller.telemetry_configuration().meters;
        let mut count = 0;
        for ((entry, fold), handle) in table.iter().zip(&self.folds).zip(1_u32..) {
            let (valid, peak) = match (&entry.target, entry.metric) {
                (protocol::MeterTarget::Output, protocol::MeterMetric::Peak) => {
                    (batch.output_valid, batch.output_peak)
                }
                // The wire refuses a master peak entry, which resolves no strip.
                (
                    protocol::MeterTarget::Strip(_) | protocol::MeterTarget::Master,
                    protocol::MeterMetric::Peak,
                ) => (fold.present, fold.peak),
                // #1587b D3, #1690 D3: one aggregate record, never clipped. The wire refuses an
                // output gain-reduction entry.
                (_, protocol::MeterMetric::GainReduction) => {
                    if let Some(slot) = records.get_mut(count) {
                        let valid = fold.gain_reduction_present;
                        *slot = protocol::MeterRecord {
                            handle,
                            component: protocol::MeterComponent::Aggregate,
                            flags: if valid { VALID } else { 0 },
                            value: if valid { fold.gain_reduction_db } else { 0.0 },
                        };
                        count += 1;
                    }
                    continue;
                }
            };
            for (component, value) in [
                (protocol::MeterComponent::Left, peak[0]),
                (protocol::MeterComponent::Right, peak[1]),
            ] {
                let Some(slot) = records.get_mut(count) else {
                    continue;
                };
                let mut flags = 0;
                if valid {
                    flags |= VALID;
                    if value > 1.0 {
                        flags |= CLIPPED;
                    }
                }
                *slot = protocol::MeterRecord {
                    handle,
                    component,
                    flags,
                    value: if valid { value } else { 0.0 },
                };
                count += 1;
            }
        }
        if count == 0 {
            return;
        }
        let span = protocol::MeterWindowSpan {
            first_sample: protocol::SampleTime(batch.first_sample),
            end_sample: protocol::SampleTime(batch.end_sample),
            window_sequence: batch.sequence,
        };
        if controller
            .stage_meter_batch_event(revision, span, &records[..count])
            .is_err()
        {
            self.unstaged = self
                .unstaged
                .saturating_add(u64::from(self.fold.windows_per_batch()));
        }
    }
}

/// Keep each epoch's master strip ([`ProviderEpoch::master_meter`]) by the browser's `sync_master`
/// rule (#1690 D2): every pending epoch takes the master the core's `master` row names now
/// (`monitor`), and the running epoch takes it only while nothing waits, so the plan still
/// rendering keeps its own until a replacement displaces it. Each is resolved by ID against its own
/// plan. When any changes, the meter entries resolve again at the next frame. Called after every
/// commit that writes the monitor overlay, after a successor is published and after each
/// promotion. Control thread; allocates nothing; `O(epochs x meters)`.
fn sync_master_meters(
    monitor: &controls::MonitorOverlay,
    running: &mut ProviderEpoch,
    pending: &mut [ProviderEpoch],
    meters: &mut Option<SessionMeters>,
) {
    let master = monitor.master().map(session::StableId::as_str);
    let resolve = |provider: &mut ProviderEpoch| {
        let index = master.and_then(|id| strip_meter_index(&provider.inventory, id));
        core::mem::replace(&mut provider.master_meter, index) != index
    };
    let mut moved = false;
    for provider in pending.iter_mut() {
        moved |= resolve(provider);
    }
    if pending.is_empty() {
        moved |= resolve(running);
    }
    if moved && let Some(meters) = meters.as_mut() {
        meters.batches.resolved_epoch = None;
    }
}

/// The frame strip of strip `id` in a plan whose inventory is `meters`: the index of its first
/// post-matrix sample-peak meter (`PlanStateInventory::meter_targets`), or `None`. The one rule
/// for a meter entry's strip (#1588 D3) and an observation tap's (#1587b), so both read the same
/// frame strip.
pub(crate) fn strip_meter_index(meters: &host_core::PlanStateInventory, id: &str) -> Option<u32> {
    meters
        .meter_targets()
        .find(|&(_, strip, tap, metrics)| {
            strip == id
                && tap == host_core::MeterTap::PostMatrix
                && metrics.contains(host_core::MeterMetricSet::SAMPLE_PEAK)
        })
        .and_then(|(index, ..)| u32::try_from(index).ok())
}

/// The provider epochs a session holds: the running one, the parked ones and the pending ones.
struct ProviderEpochs<'a> {
    current: &'a ProviderEpoch,
    retired: &'a [ProviderEpoch],
    pending: &'a [ProviderEpoch],
}

impl<'a> ProviderEpochs<'a> {
    fn all(&self) -> impl Iterator<Item = &'a ProviderEpoch> {
        core::iter::once(self.current)
            .chain(self.retired)
            .chain(self.pending)
    }

    /// The master strip provider epoch `epoch` holds (#1690 D2), its frame strip, or `None` when
    /// no epoch held is `epoch` or it has no master.
    fn master(&self, epoch: u64) -> Option<u32> {
        self.all()
            .find(|provider| provider.epoch == epoch)
            .and_then(|provider| provider.master_meter)
    }

    /// The plan inventory of provider epoch `epoch`, whose meter rows are its meter readers in
    /// reader order, or `None` when no epoch held is `epoch`.
    fn meters(&self, epoch: u64) -> Option<&'a host_core::PlanStateInventory> {
        self.all()
            .find(|provider| provider.epoch == epoch)
            .map(|provider| &provider.inventory)
    }
}

/// The observation reader pairs the swap block that adopted a plan carried (#1674's observation
/// half, #1587b): the swap record's [`engine::realtime::ObserverKind::Observation`] entries,
/// which render copies here in that block ([`Self::record`], through the plan's host attachment:
/// the control plane's own or, #1332 S6b, the preparer's) and the control thread
/// reads when it promotes the plan's epoch. The record is what actually carried: a tap enters it
/// only when its effect's restore succeeded at the swap, which preparation cannot know (#1674 R1).
///
/// Sized at preparation, one pair per observation handle of the plan (a successor handle is
/// paired at most once), so render writes into storage it never grows. A plan is adopted at most
/// once, so the pairs are written once; `count` is stored last, with `Release`.
///
/// **Three states (#1587b attempt 4).** `count` starts at `UNWRITTEN`: render has not run
/// the plan's swap block. Render claims a candidate a little before it carries into it
/// (`plan_exchange`'s claim, then `carry_from`), and the control thread promotes a claimed
/// candidate (`withdraw_pending_candidate`) without waiting, so a promotion can find the record
/// unwritten. It then marks the epoch's pairing pending (`ProviderEpoch::observations_pending`),
/// and later steps settle it:
/// - **Early read.** Each service step reads `count` with `Acquire`; once it is written, that
///   load synchronizes with render's `Release` store, after the pairs, so the pairs read whole.
/// - **Reclaim fallback.** Render retires the displaced plan into the retirement queue only after
///   the carry returned (or after a swap whose hook never ran: a plan with no attachment), and the
///   control thread's reclaim takes it from the queue, which orders the hook before the reclaim.
///   A record still unwritten there means nothing carried, and the mark clears.
///
/// While the mark stands the epoch's readers are the successor's own, which no carried tap
/// publishes to: the strip shows no reading, never a wrong one.
pub struct CarriedTaps {
    pairs: Box<[[core::sync::atomic::AtomicU32; 2]]>,
    count: core::sync::atomic::AtomicU32,
}

impl CarriedTaps {
    /// Room for `handles` pairs. Allocates; control thread.
    pub(crate) fn new(handles: usize) -> Result<Self, std::collections::TryReserveError> {
        let mut pairs = Vec::new();
        pairs.try_reserve_exact(handles)?;
        pairs.resize_with(handles, || {
            [
                core::sync::atomic::AtomicU32::new(0),
                core::sync::atomic::AtomicU32::new(0),
            ]
        });
        Ok(Self {
            pairs: pairs.into_boxed_slice(),
            count: core::sync::atomic::AtomicU32::new(Self::UNWRITTEN),
        })
    }

    /// The heap a record of `handles` pairs holds: the shared allocation (its two counts and the
    /// record), the pairs and, with `attachment`, the [`CarriedTapsAttachment`]'s box (a preparer
    /// that keeps the record in its own attachment charges its field with that attachment).
    pub(crate) fn retained_bytes(handles: usize, attachment: bool) -> Option<usize> {
        handles
            .checked_mul(size_of::<[core::sync::atomic::AtomicU32; 2]>())?
            .checked_add(if attachment {
                size_of::<CarriedTapsAttachment>()
            } else {
                0
            })?
            .checked_add(2 * size_of::<usize>() + size_of::<Self>())
    }

    // REALTIME_POLICY_BEGIN: #1587b, #1332 S6b, the swap block's carried observation readers.
    /// Copy the swap record's observation entries into this record: the render side's half of the
    /// control side's pairing at promotion. Render thread, in the swap block of the plan this
    /// record belongs to, through its host attachment's
    /// [`engine::realtime::HostAttachment::adopt_predecessor`]: atomic stores into storage sized at
    /// preparation; an entry past it, or whose index does not fit a `u32`, is skipped.
    pub fn record(&self, carries: &[engine::realtime::ObserverCarry]) {
        let mut written = 0_u32;
        let mut slots = self.pairs.iter();
        for carry in carries {
            if carry.kind != engine::realtime::ObserverKind::Observation {
                continue;
            }
            let (Ok(successor), Ok(predecessor)) = (
                u32::try_from(carry.successor),
                u32::try_from(carry.predecessor),
            ) else {
                continue;
            };
            let Some([into, from]) = slots.next() else {
                break;
            };
            into.store(successor, Ordering::Relaxed);
            from.store(predecessor, Ordering::Relaxed);
            written += 1;
        }
        self.count.store(written, Ordering::Release);
    }
    // REALTIME_POLICY_END

    /// `count` before render ran the plan's swap block.
    const UNWRITTEN: u32 = u32::MAX;

    /// The carried pairs, as carry records, or `None` while render has not written the record.
    /// Allocates nothing.
    fn carries(&self) -> Option<impl Iterator<Item = engine::realtime::ObserverCarry> + '_> {
        let count = self.count.load(Ordering::Acquire);
        if count == Self::UNWRITTEN {
            return None;
        }
        Some(
            self.pairs
                .iter()
                .take(count as usize)
                .map(|[successor, predecessor]| engine::realtime::ObserverCarry {
                    kind: engine::realtime::ObserverKind::Observation,
                    successor: successor.load(Ordering::Relaxed) as usize,
                    predecessor: predecessor.load(Ordering::Relaxed) as usize,
                }),
        )
    }
}

/// Pair `successor`'s observation readers with `predecessor`'s from `successor`'s carry record
/// (#1587b), if render has written it; returns whether the pairing is settled. An unwritten
/// record leaves it pending: [`SessionState::settle_observation_pairings`] settles it once the
/// record is written, or as nothing carried once the predecessor's epoch is gone.
fn pair_observations(successor: &mut ProviderEpoch, predecessor: &mut ProviderEpoch) -> bool {
    let Some(record) = successor.carried_taps.as_deref() else {
        return true;
    };
    let Some(carries) = record.carries() else {
        return false;
    };
    // One record at a time: `pair_carried_observations` takes a slice, and the record's pairs
    // are atomics, so collecting them would allocate; a one-element slice per pair does not.
    // The epochs' acknowledging readers (the C ABI's; none for a preparer that keeps them in the
    // plan) and their indexes' read-only views (#1332 S6b), at the same positions.
    for carry in carries {
        let carry = core::slice::from_ref(&carry);
        host_core::pair_carried_observations(
            &mut successor.observations,
            &mut predecessor.observations,
            carry,
        );
        host_core::observation::ObservationIndex::pair_carried(
            &mut successor.observation_index,
            &mut predecessor.observation_index,
            carry,
        );
    }
    true
}

/// The C ABI plan's host attachment (#1587b): the plan's [`CarriedTaps`], shared with its
/// provider epoch.
pub(crate) struct CarriedTapsAttachment(pub(crate) std::sync::Arc<CarriedTaps>);

impl engine::realtime::HostAttachment for CarriedTapsAttachment {
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }

    // REALTIME_POLICY_BEGIN: #1587b, the swap block's carried observation readers.
    /// Copy the swap record's observation entries into the plan's [`CarriedTaps`]
    /// ([`CarriedTaps::record`]). Render thread, in the swap block.
    fn adopt_predecessor(
        &mut self,
        _predecessor: &mut dyn engine::realtime::HostAttachment,
        carries: &[engine::realtime::ObserverCarry],
    ) {
        self.0.record(carries);
    }
    // REALTIME_POLICY_END
}

/// The observation taps of a provider epoch, as the meter delivery's gain reduction reads them
/// (#1587b): every declared tap of every observed instance, on its instance's strip, with the arm
/// state render publishes in the tap's slot (#1585 D2). A torn read is an unarmed tap for this
/// poll.
struct CapiGainReductionTaps<'a> {
    observations: &'a [host_core::EffectObservationHandle],
    strips: &'a [u32],
}

impl host_core::GainReductionTaps for CapiGainReductionTaps<'_> {
    fn for_each_tap(&mut self, visit: &mut impl FnMut(host_core::TapView<'_>)) {
        for (handle, &strip) in self.observations.iter().zip(self.strips) {
            for (descriptor, reader) in handle.descriptor.observations.iter().zip(&handle.readers) {
                let (armed, arm_sample) = reader
                    .read_state()
                    .map_or((false, 0), |state| (state.armed, state.arm_sample));
                visit(host_core::TapView {
                    strip,
                    descriptor,
                    reader,
                    armed,
                    arm_sample,
                });
            }
        }
    }
}

/// Every live send move (a following route's record with a ramp, written by a live commit or by a
/// rebuild's send join) and every landing at once a rebuild's carry starts, folded into one
/// bound (B19-R7): the newest revision among them and the longest ramp any of them takes. A move
/// is in effect from at most the watermark's `first_sample` once the watermark covers its revision
/// (the watermark is monotone), so every move has ended once the watermark covers `revision` and
/// the render clock is at or past `first_sample + ramp`. The bound only errs toward "may still
/// ramp", and it is one word pair: nothing grows with the moves.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SendMoveWindow {
    revision: u64,
    ramp: u32,
}

impl SendMoveWindow {
    /// Whether some move of the window may still ramp, given the watermark (`None` when a read
    /// gave up: then it may) and the render clock read after it.
    fn may_ramp(self, watermark: Option<PlanWatermark>, render_clock: u64) -> bool {
        watermark.is_none_or(|watermark| {
            watermark.revision < self.revision
                || render_clock < watermark.first_sample.saturating_add(u64::from(self.ramp))
        })
    }
}

/// Fold a send move of `revision` over `ramp` into the window (B19-R7): the newest revision and
/// the longest ramp. A zero ramp is a step, which nothing can cut, and folds nothing.
fn fold_send_move(window: &mut Option<SendMoveWindow>, revision: u64, ramp: u32) {
    if ramp == 0 {
        return;
    }
    *window = Some(match *window {
        Some(window) => SendMoveWindow {
            revision: window.revision.max(revision),
            ramp: window.ramp.max(ramp),
        },
        None => SendMoveWindow { revision, ramp },
    });
}

/// The length a landing at once takes at its successor's first block (B19-R7): a carried
/// following route's column that lost a follow curve (its own source mute or a reaching VCA mute)
/// or that composes otherwise lands over the `Mute` or `VcaMute` row's length, both the session's
/// mute length.
fn send_landing_ramp(model: &session::SessionModel) -> u32 {
    controls::LiveRamps::for_session(model).for_row(controls::LiveRampRow::Mute)
}

/// The stable key of one observation tap, borrowed from the bytes that name it (#1677): the strip's
/// ID, the rack, the console slot's or insert's ID and the effect-local tap ID. The same key
/// `OBSERVATION_ENDED` names ([`controls::TapKey`]), in a form a host can build from its staging
/// without allocating.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationKeyRef<'a> {
    /// The ID of the track or submix.
    pub strip_id: &'a [u8],
    /// The console slot or the insert section.
    pub rack: host_core::LiveEffectRack,
    /// The console slot's or the insert's ID.
    pub effect_id: &'a [u8],
    /// The effect-local tap ID.
    pub tap_id: u32,
}

/// The core form of one `OBSERVATION_READ` selection (#1586 D3): it borrows the selection's IDs.
fn core_selection<'a>(
    selection: &protocol::ObservationSelectionRef<'a>,
) -> host_core::observation::ObservationSelection<'a> {
    host_core::observation::ObservationSelection {
        strip_id: selection.strip_id,
        section: match selection.section {
            protocol::ObservationSection::Console => host_core::LiveEffectRack::Console,
            protocol::ObservationSection::Inserts => host_core::LiveEffectRack::Inserts,
        },
        effect_id: selection.slot_id,
        tap_id: selection.tap_id,
        channels: match selection.channels {
            protocol::ParameterChannel::Left => {
                host_core::observation::ObservationReadChannels::Left
            }
            protocol::ParameterChannel::Right => {
                host_core::observation::ObservationReadChannels::Right
            }
            protocol::ParameterChannel::Both => {
                host_core::observation::ObservationReadChannels::Both
            }
        },
    }
}

/// The wire row of one core row (#1586 D1): the arm state is the slot's published state, or none
/// when render has not applied the tap's last record or the read tore.
fn wire_row(row: &host_core::observation::ObservationRow) -> protocol::ObservationReadRow {
    use host_core::observation::ObservationReadStatus as Status;
    let state = row.state;
    protocol::ObservationReadRow {
        status: match row.status {
            Status::Pending => protocol::ObservationRowStatus::Pending,
            Status::Unarmed => protocol::ObservationRowStatus::Unarmed,
            Status::Ready => protocol::ObservationRowStatus::Ready,
        },
        armed: state.is_some_and(|state| state.armed),
        arm_sample: state.map_or(0, |state| state.arm_sample),
        arm_tag: state.map_or(0, |state| state.arm_tag),
        window: row.window.map(|window| protocol::ObservationRowWindow {
            first_sample: window.first_sample,
            end_sample: window.end_sample,
            sequence: window.sequence,
            blocks: window.blocks,
        }),
        left: row.left,
        right: row.right,
        unit: row
            .tap
            .map_or(protocol::ParameterUnit::Db, |tap| wire_unit(tap.unit)),
    }
}

const fn wire_unit(unit: effect_contract::ParameterUnit) -> protocol::ParameterUnit {
    match unit {
        effect_contract::ParameterUnit::Db => protocol::ParameterUnit::Db,
        effect_contract::ParameterUnit::Hz => protocol::ParameterUnit::Hz,
        effect_contract::ParameterUnit::Milliseconds => protocol::ParameterUnit::Milliseconds,
        effect_contract::ParameterUnit::Samples => protocol::ParameterUnit::Samples,
        effect_contract::ParameterUnit::Linear => protocol::ParameterUnit::Linear,
        effect_contract::ParameterUnit::Ratio => protocol::ParameterUnit::Ratio,
    }
}

/// D4's status and code for a refusal the codec cannot rule out; `None` for one it does (a batch
/// of more than 256, or a row table shorter than the batch), which is an internal fault here.
const fn observation_refusal(
    refusal: host_core::observation::ObservationRefusal,
) -> Option<(protocol::StatusCode, &'static str)> {
    use host_core::observation::ObservationRefusal as Refusal;
    match refusal {
        Refusal::UnknownAddress => Some((
            protocol::StatusCode::NotFound,
            "observation.address.unknown",
        )),
        Refusal::UnknownTap => Some((protocol::StatusCode::NotFound, "observation.tap.unknown")),
        Refusal::UnsupportedTap => Some((
            protocol::StatusCode::Unavailable,
            "observation.tap.unsupported",
        )),
        Refusal::Unbound => Some((protocol::StatusCode::Unavailable, "observation.unbound")),
        Refusal::Duplicate => Some((
            protocol::StatusCode::InvalidField,
            "observation.selection.duplicate",
        )),
        Refusal::TooManySelections | Refusal::RowsTooShort => None,
    }
}

/// Where the session resolved an observation tap's stable key, on the newest plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedObservationTap {
    /// The position of the tap's effect in the newest plan's effect producers.
    pub producer: usize,
    /// The tap's index in its effect's declared menu, which the render side arms.
    pub tap_index: u32,
}

/// Why a key names no tap of the newest plan (#1677). Nothing was written.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationKeyError {
    /// The strip ID is no track or submix of the plan.
    UnknownStrip,
    /// The strip has no console slot or insert of that ID.
    UnknownEffect,
    /// The effect declares no tap of that ID.
    UnknownTap,
    /// The tap is declared and its cost is not resident; V1 binds no lane for one.
    ComputedTap,
}

/// A subscription batch would leave armed taps whose keys one `OBSERVATION_ENDED` event cannot
/// carry in one control frame (#1676 D1, #1685 D1): the frame is full. Nothing was written.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationLimit {
    /// The position, in the batch, of the first change whose arm makes the event outgrow the
    /// frame.
    pub index: usize,
    /// The encoded bytes the event would need with that arm, every removal of the batch counted.
    pub event_bytes: usize,
    /// The control frame limit.
    pub frame_bytes: usize,
}

/// One subscription change of a batch handed to
/// the session's observation admission (#1676), which the core's `Observe` edit runs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationChange {
    /// The producer's position in the newest plan's effect producers.
    pub position: usize,
    /// The tap's index in the producer's observation menu.
    pub tap_index: u32,
    /// `true` arms the tap, `false` disarms it.
    pub armed: bool,
    /// Render blocks per published window; ignored by a disarm.
    pub window_blocks: u32,
}

/// Why the session's observation admission refused a batch. Nothing was queued.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationRefusal {
    /// The batch would leave armed taps whose ended event outgrows one control frame. Not
    /// retryable as sent: a tap must be disarmed first.
    Limit(ObservationLimit),
    /// The change at `index` needs more room than its producer's observation queue has free.
    /// Retryable once a render drains the queue.
    Full {
        /// The change's position in the batch.
        index: usize,
    },
    /// The change at `index` names a position past the newest plan's effect producers.
    NoProducer {
        /// The change's position in the batch.
        index: usize,
    },
    /// The change at `index` names a tap an earlier change of the same batch already names. A
    /// change sets its tap's whole state and the batch drains in one block, so a second change
    /// would supersede the first, which an observation record never is (D15-2). Not retryable as
    /// sent.
    Duplicate {
        /// The position in the batch of the second change that names the tap.
        index: usize,
    },
    /// The change at `index` names a producer whose observation queue has no slot for its tap:
    /// the tap index is past the producer's observation menu, or the plan bound the producer no
    /// observation queue at all (capacity `0`). Such a record would be acknowledged and never
    /// applied, so it is refused before anything is queued. Not retryable as sent.
    Unbound {
        /// The change's position in the batch.
        index: usize,
    },
}

/// The index in `descriptor`'s observation menu of the tap with ID `tap_id`, the one the render side
/// arms (#1677). A tap the menu does not declare is [`ObservationKeyError::UnknownTap`], and a
/// declared tap that is not resident is [`ObservationKeyError::ComputedTap`]: V1 ships no
/// implementation for one, and saying so is better than binding a lane that would never publish.
///
/// # Errors
///
/// As above.
pub fn resolve_observation_tap(
    descriptor: &effect_contract::EffectDescriptor,
    tap_id: u32,
) -> Result<u32, ObservationKeyError> {
    let (index, tap) = descriptor
        .observations
        .iter()
        .enumerate()
        .find(|(_, tap)| tap.id.0 == tap_id)
        .ok_or(ObservationKeyError::UnknownTap)?;
    if !matches!(tap.cost, effect_contract::ObservationCost::Resident) {
        return Err(ObservationKeyError::ComputedTap);
    }
    u32::try_from(index).map_err(|_| ObservationKeyError::UnknownTap)
}

/// A plan's observation slot offsets, its slot storage, its per-producer admission scratch and how
/// many of its slots are armed.
pub(crate) type ObservationSlots = (
    Box<[u32]>,
    Box<[ObservationSlot]>,
    Box<[ObservationBatchCount]>,
    usize,
    usize,
);

/// The slot storage for `producers`, the newest plan's: the first slot of each producer (and one
/// past the last), and one slot per declared tap (#1584 D6a), each armed when `bindings` holds its
/// tap (#1676), and each slot carries its key's encoded bytes in an `OBSERVATION_ENDED` event
/// (#1685 D2), from the producer's two IDs. The last two figures are the armed slots and their keys'
/// bytes. Built when a plan is prepared, fallibly and before anything commits; the session takes it
/// after the commit and after the fold, so nothing pending is lost to a change of positions.
pub(crate) fn observation_slots(
    producers: &[host_core::EffectControlProducer],
    bindings: &controls::ObservationBindings,
    wire: crate::compile::ObservationWire,
) -> Result<ObservationSlots, std::collections::TryReserveError> {
    let mut offsets = Vec::new();
    offsets.try_reserve_exact(producers.len() + 1)?;
    let mut total = 0_u32;
    for producer in producers {
        offsets.push(total);
        total =
            total.saturating_add(u32::try_from(producer.observe_capacity()).unwrap_or(u32::MAX));
    }
    offsets.push(total);
    let mut counts = Vec::new();
    counts.try_reserve_exact(producers.len())?;
    counts.resize(producers.len(), ObservationBatchCount::default());
    let mut slots = Vec::new();
    slots.try_reserve_exact(total as usize)?;
    let mut armed = 0_usize;
    let mut armed_bytes = 0_usize;
    for producer in producers {
        // Every ID is at most `STABLE_ID_MAXIMUM_BYTES`, so a key is a few hundred bytes.
        let key_bytes = wire.key_bytes(producer.track_id.len(), producer.effect_id.len());
        let key_bytes = u16::try_from(key_bytes).unwrap_or(u16::MAX);
        for tap in 0..producer.observe_capacity() {
            let binding = if bindings.is_empty() {
                None
            } else {
                producer
                    .descriptor
                    .observations
                    .get(tap)
                    .and_then(|declared| {
                        bindings.binding_by_parts(
                            &producer.track_id,
                            producer.address.rack == host_core::LiveEffectRack::Console,
                            &producer.effect_id,
                            declared.id.0,
                        )
                    })
            };
            let is_armed = binding.is_some();
            armed += usize::from(is_armed);
            if is_armed {
                armed_bytes += usize::from(key_bytes);
            }
            slots.push(ObservationSlot {
                armed: is_armed,
                pending: PendingArm::None,
                mark: 0,
                key_bytes,
                queued: false,
                // An armed tap's prepared arm publishes its binding's tag (#1585 R2).
                tag: binding.map_or(0, |binding| binding.arm_tag),
            });
        }
    }
    Ok((
        offsets.into_boxed_slice(),
        slots.into_boxed_slice(),
        counts.into_boxed_slice(),
        armed,
        armed_bytes,
    ))
}

/// One producer's share of the batch being admitted (#1676, batch 13b fold): how many of its
/// records name the producer, valid while `mark` is the session's batch stamp.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ObservationBatchCount {
    /// The batch stamp `needed` was counted under.
    pub(crate) mark: u32,
    /// Records of that batch that name the producer.
    pub(crate) needed: u32,
}

/// One observation slot: whether its tap is armed now, and what the host pushed to it since the
/// last fold (#1584 D6a, #1676).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ObservationSlot {
    /// Armed now: the table's state with every push since folded in. Lets a subscription be
    /// admitted against the frame without a key lookup, on the audio thread between quanta.
    pub(crate) armed: bool,
    /// The push waiting to be folded into the table.
    pub(crate) pending: PendingArm,
    /// The stamp of the last batch that named this slot (#1676, batch 13b fold): a change whose
    /// slot already carries the current stamp is the batch's second naming of the tap.
    pub(crate) mark: u32,
    /// The encoded bytes of the tap's key in an `OBSERVATION_ENDED` event (#1685 D2): what arming
    /// it adds to the event that would end every armed tap. Set when the plan's slots are built.
    pub(crate) key_bytes: u16,
    /// A record for this tap waits in the queue of a plan render has not adopted (#1310, root
    /// ruling B15-Q4): pushed while the newest plan was a pending candidate, or forwarded to it from
    /// a candidate it superseded. A replacement of that candidate re-queues the tap's bound state
    /// on its own instance, because a tap it carries from the running plan takes that plan's
    /// state and would lose the record.
    pub(crate) queued: bool,
    /// The tag of the last record queued for this tap on the newest plan, or of the binding its
    /// prepared arm publishes (#1585 D4); `0` for none. Its kind is [`Self::armed`], which every
    /// push and every slot build sets with it. A reader reports the tap pending while the record
    /// is an arm whose tag its slot has not published, and unarmed at once after a disarm.
    pub(crate) tag: u64,
}

/// What the host last pushed to one observation slot (#1584 D6a).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum PendingArm {
    /// Nothing since the last fold.
    #[default]
    None,
    /// Armed, with this window length in blocks (`0` is the plan's meter window).
    Arm(u32),
    /// Disarmed.
    Disarm,
}

/// The borrowed form of an observation tap's stable key, as the session resolves it.
fn observation_key_ref(key: &controls::TapKey) -> ObservationKeyRef<'_> {
    let (rack, effect) = match &key.section {
        controls::TapSection::Console(id) => (host_core::LiveEffectRack::Console, id),
        controls::TapSection::Inserts(id) => (host_core::LiveEffectRack::Inserts, id),
    };
    ObservationKeyRef {
        strip_id: key.strip.as_str().as_bytes(),
        rack,
        effect_id: effect.as_str().as_bytes(),
        tap_id: key.tap_id,
    }
}

/// The stable key of the tap `tap_index` of the effect `producer` addresses, or `None` for an index
/// the effect does not declare.
fn tap_key_of(
    producer: &host_core::EffectControlProducer,
    tap_index: u32,
) -> Option<controls::TapKey> {
    let tap = producer.descriptor.observations.get(tap_index as usize)?;
    let effect = session::StableId::parse(&producer.effect_id)?;
    let section = match producer.address.rack {
        host_core::LiveEffectRack::Console => controls::TapSection::Console(effect),
        host_core::LiveEffectRack::Inserts => controls::TapSection::Inserts(effect),
    };
    Some(controls::TapKey {
        strip: session::StableId::parse(&producer.track_id)?,
        section,
        tap_id: tap.id.0,
    })
}

/// One observation record a superseded candidate's queue held, as it is queued again on its
/// replacement's matching instance (#1310, root ruling B15-Q4). No host mirrors it: render
/// publishes each tap's arm state in its slot when it drains the record (#1585 D3).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationForward {
    /// The replacement's effect producer, a position in the newest plan's effect producers.
    pub position: usize,
    /// The tap's index in that producer's observation menu.
    pub tap_index: u32,
    /// The tap's bound state now: armed, with its window, or disarmed.
    pub armed: bool,
    /// The bound window, `0` for the plan's meter window.
    pub window_blocks: u32,
    /// The record's tag (#1585 D4): the donor slot's for a forward the transaction does not
    /// change, so render publishes the tag the record was acknowledged with; `0` asks the commit
    /// for a fresh one.
    pub arm_tag: u64,
}

/// The records a withdrawn candidate's observation queues hold, as the replacement queues them
/// (#1310, root ruling B15-Q4): one per tap whose slot in `slots` (the candidate's, `offsets` its
/// producers' first slots) is `queued`, on the `successor` instance with the same strip, rack,
/// effect ID and tap ID, carrying the tap's state in `bindings`, which every admitted record has
/// been folded into. An observation queue holds only `Observe` records (arm, with a window, or
/// disarm), so the bound state is what the last record asked for, and draining it at the
/// replacement's first block arms the tap where the candidate's first block would have. A tap the
/// replacement no longer has is skipped: the transaction ended its binding, and the
/// `OBSERVATION_ENDED` event says so.
///
/// # Errors
///
/// [`CommandError::Internal`] if a successor queue lacks the room, which cannot happen: its queue
/// is fresh and holds one record per declared tap, and at most one is forwarded per tap.
/// [`CommandError::CompileRejected`] if the list cannot be allocated.
pub(crate) fn observation_forwards(
    donor: &[host_core::EffectControlProducer],
    offsets: &[u32],
    slots: &[ObservationSlot],
    successor: &[host_core::EffectControlProducer],
    bindings: &controls::ObservationBindings,
) -> Result<Vec<ObservationForward>, CommandError> {
    let mut forwards = Vec::new();
    for (position, producer) in donor.iter().enumerate() {
        let (Some(&first), Some(&end)) = (offsets.get(position), offsets.get(position + 1)) else {
            break;
        };
        for slot in first..end {
            if !slots.get(slot as usize).is_some_and(|entry| entry.queued) {
                continue;
            }
            let Some(key) = tap_key_of(producer, slot - first) else {
                continue;
            };
            let target = successor.iter().enumerate().find_map(|(into, candidate)| {
                if candidate.track_id != producer.track_id
                    || candidate.address.rack != producer.address.rack
                    || candidate.effect_id != producer.effect_id
                {
                    return None;
                }
                let tap = candidate
                    .descriptor
                    .observations
                    .iter()
                    .position(|declared| declared.id.0 == key.tap_id)
                    .filter(|tap| *tap < candidate.observe_capacity())?;
                Some((into, u32::try_from(tap).ok()?))
            });
            let Some((position, tap_index)) = target else {
                continue;
            };
            let window = bindings.window_of(&key);
            forwards
                .try_reserve(1)
                .map_err(|_| CommandError::CompileRejected(failure(ResourceFault::Allocation)))?;
            forwards.push(ObservationForward {
                position,
                tap_index,
                armed: window.is_some(),
                window_blocks: window.unwrap_or(0),
                arm_tag: slots.get(slot as usize).map_or(0, |entry| entry.tag),
            });
        }
    }
    for forward in &forwards {
        let needed = forwards
            .iter()
            .filter(|other| other.position == forward.position)
            .count();
        if needed > successor[forward.position].observe_available_capacity() {
            debug_assert!(false, "a fresh observation queue holds one record per tap");
            return Err(CommandError::Internal);
        }
    }
    Ok(forwards)
}

/// The records of a structural transaction's `Observe` edits on its successor's producers (#1584
/// T2): each net record's tap resolved by its stable key among `successor`, skipping a tap with no
/// queue slot there (the transaction removed its target) and a tap a forward in `forwards` already
/// re-queues with its bound state.
///
/// # Errors
///
/// [`CommandError::Internal`] if a successor queue lacks the room, which cannot happen: its queue
/// is fresh, holds one record per declared tap, and at most one record per tap is queued between
/// the forwards and these. [`CommandError::CompileRejected`] if the list cannot be allocated.
fn observe_successor_pushes(
    writes: &[controls::ObserveWrite],
    successor: &[host_core::EffectControlProducer],
    forwards: &mut [ObservationForward],
) -> Result<Vec<ObservationForward>, CommandError> {
    let mut pushes = Vec::new();
    pushes
        .try_reserve_exact(writes.len())
        .map_err(|_| CommandError::CompileRejected(failure(ResourceFault::Allocation)))?;
    for write in writes {
        let key = observation_key_ref(&write.key);
        let target = successor
            .iter()
            .enumerate()
            .find_map(|(position, producer)| {
                if producer.address.rack != key.rack
                    || producer.track_id.as_bytes() != key.strip_id
                    || producer.effect_id.as_bytes() != key.effect_id
                {
                    return None;
                }
                let tap = producer
                    .descriptor
                    .observations
                    .iter()
                    .position(|declared| declared.id.0 == key.tap_id)
                    .filter(|tap| *tap < producer.observe_capacity())?;
                Some((position, u32::try_from(tap).ok()?))
            });
        let Some((position, tap_index)) = target else {
            continue;
        };
        // A forward re-queues the bound state, this write's included: it carries this write's
        // tag, which the preparation issued (#1585 D4).
        if let Some(forward) = forwards
            .iter_mut()
            .find(|forward| forward.position == position && forward.tap_index == tap_index)
        {
            forward.arm_tag = write.arm_tag;
            continue;
        }
        pushes.push(ObservationForward {
            position,
            tap_index,
            armed: write.armed,
            window_blocks: write.window_blocks,
            arm_tag: write.arm_tag,
        });
    }
    for push in &pushes {
        let needed = forwards
            .iter()
            .chain(&pushes)
            .filter(|other| other.position == push.position)
            .count();
        if needed > successor[push.position].observe_available_capacity() {
            debug_assert!(false, "a fresh observation queue holds one record per tap");
            return Err(CommandError::Internal);
        }
    }
    Ok(pushes)
}

/// The wire key of an ended binding.
fn ended_key(key: &controls::TapKey) -> protocol::ObservationTapKey {
    protocol::ObservationTapKey {
        strip_id: key.strip.as_str().to_owned(),
        section: if key.section.is_console() {
            protocol::ObservationSection::Console
        } else {
            protocol::ObservationSection::Inserts
        },
        effect_id: key.section.id().as_str().to_owned(),
        tap_id: key.tap_id,
    }
}

pub(crate) struct ObservedController {
    pub(crate) inner: ProtocolController<SessionControlProvider>,
}

impl ObservedController {
    pub(crate) fn new(inner: ProtocolController<SessionControlProvider>) -> Self {
        #[cfg(feature = "test-support")]
        update_test_owners(|owners| owners.replay_current_constructed += 1);
        Self { inner }
    }
}

impl core::ops::Deref for ObservedController {
    type Target = ProtocolController<SessionControlProvider>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ObservedController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Drop for ObservedController {
    fn drop(&mut self) {
        #[cfg(feature = "test-support")]
        update_test_owners(|owners| owners.replay_current_disposed += 1);
    }
}

pub(crate) struct RenderDiagnosticSlot {
    pub(crate) diagnostic: protocol::Diagnostic,
    pub(crate) reservation: Option<protocol::ReliableEventReservation>,
    pub(crate) protocol_events_before: u64,
    pub(crate) revision: protocol::SessionRevision,
    pub(crate) occupied: bool,
}

impl RenderDiagnosticSlot {
    pub(crate) fn try_new() -> Result<Self, CompileFailure> {
        let mut code = String::new();
        code.try_reserve_exact(RENDER_DIAGNOSTIC_CODE.len())
            .map_err(|_| failure(ResourceFault::Allocation))?;
        code.push_str(RENDER_DIAGNOSTIC_CODE);
        Ok(Self {
            diagnostic: protocol::Diagnostic {
                code,
                severity: protocol::DiagnosticSeverity::Info,
                path: Vec::new(),
                detail: None,
                operation_index: None,
                sample_time: None,
                provider_sequence: None,
            },
            reservation: None,
            protocol_events_before: 0,
            revision: protocol::SessionRevision(0),
            occupied: false,
        })
    }
}

pub(crate) fn prepare_render_diagnostic_slots()
-> Result<Box<[RenderDiagnosticSlot]>, CompileFailure> {
    let mut slots = Vec::new();
    slots
        .try_reserve_exact(RENDER_DIAGNOSTIC_SLOTS)
        .map_err(|_| failure(ResourceFault::Allocation))?;
    for _ in 0..RENDER_DIAGNOSTIC_SLOTS {
        slots.push(RenderDiagnosticSlot::try_new()?);
    }
    Ok(slots.into_boxed_slice())
}

pub(crate) struct ObservedPreparedToken {
    pub(crate) inner: Option<Box<protocol::PreparedStructuralCommand>>,
}

impl ObservedPreparedToken {
    pub(crate) fn new(inner: Box<protocol::PreparedStructuralCommand>) -> Self {
        #[cfg(feature = "test-support")]
        update_test_owners(|owners| {
            owners.token_constructed += 1;
            owners.replay_candidate_constructed += 1;
        });
        Self { inner: Some(inner) }
    }

    pub(crate) fn get(&self) -> &protocol::PreparedStructuralCommand {
        self.inner.as_deref().expect("observed token is live")
    }

    pub(crate) fn get_mut(&mut self) -> &mut protocol::PreparedStructuralCommand {
        self.inner.as_deref_mut().expect("observed token is live")
    }

    /// Commit the token. `path` is how a model transaction was applied (#1313); a token of
    /// monitor writes only commits no model and takes the monitor path inside the protocol commit.
    pub(crate) fn commit(
        mut self,
        controller: &mut ObservedController,
        path: protocol::EditPath,
    ) -> Result<protocol::CommittedCommandFrame, ()> {
        let prepared = self.inner.take().expect("observed token commits once");
        #[cfg(feature = "test-support")]
        let committed = if take_test_fault_state(TestStructuralFaultPhase::ProtocolCommit) {
            drop(prepared);
            Err(())
        } else {
            controller
                .commit_prepared_structural(*prepared, path)
                .map_err(|_| ())
        };
        #[cfg(not(feature = "test-support"))]
        let committed = controller.commit_prepared_structural(*prepared, path);
        #[cfg(feature = "test-support")]
        update_test_owners(|owners| {
            owners.token_disposed += 1;
            if committed.is_ok() {
                owners.replay_candidate_published += 1;
                owners.replay_current_disposed += 1;
            } else {
                owners.replay_candidate_disposed += 1;
            }
        });
        committed.map_err(|_| ())
    }
}

impl Drop for ObservedPreparedToken {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            drop(inner);
            #[cfg(feature = "test-support")]
            update_test_owners(|owners| {
                owners.token_disposed += 1;
                owners.replay_candidate_disposed += 1;
            });
        }
    }
}

/// The diagnostic code of a transaction that changes the session's sample rate or quantum (#1290a):
/// the plan exchange admits a replacement plan only at the envelope of the plan it replaces, so
/// the session must be re-prepared from its document instead. An adapter maps it to its own
/// "re-prepare" result.
pub const REPREPARE_REQUIRED: &str = "control.apply.reprepare_required";

/// What one transition fallback across the pending `Primed` candidate did (#1397 D2).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransitionStep {
    /// No warm candidate was pending, or render had already adopted it: nothing to do, and its
    /// record is gone.
    NotPending,
    /// The transition was published in the donor's place, `TRANSITION_FALLBACK`.
    Published,
    /// The re-preparation was refused (a defect): the donor was republished whole with its record,
    /// `transition_reprepare_refusals` rose, and its revision stays pending.
    Refused,
}

/// Why a control command was refused.
#[derive(Debug)]
pub enum CommandError {
    /// The request frame cannot be correlated.
    Invalid,
    /// The response needs `required` bytes.
    BufferTooSmall {
        /// Bytes the response needs.
        required: u64,
    },
    /// A plan replacement or a queue has no room now; retry.
    Backpressure,
    /// The prospective session does not prepare or is not admitted.
    CompileRejected(CompileFailure),
    /// The control plane's own bookkeeping failed.
    Internal,
}

impl CommandError {
    /// The diagnostic text of this refusal, the same on every host (decision 17 C1). An adapter
    /// owns only its result-code numbering.
    #[must_use]
    pub fn diagnostic(&self) -> std::borrow::Cow<'_, [u8]> {
        match self {
            Self::Invalid => std::borrow::Cow::Borrowed(b"control.frame.invalid"),
            Self::BufferTooSmall { .. } => std::borrow::Cow::Borrowed(b"control.output.too_small"),
            Self::Backpressure => std::borrow::Cow::Borrowed(b"control.plan.backpressure"),
            Self::CompileRejected(failure) => failure.diagnostic(),
            Self::Internal => std::borrow::Cow::Borrowed(b"control.internal"),
        }
    }
}

impl EventError {
    /// The diagnostic text of this refusal, the same on every host (decision 17 C1). An adapter
    /// owns only its result-code numbering.
    #[must_use]
    pub const fn diagnostic(self) -> &'static [u8] {
        match self {
            Self::BufferTooSmall { .. } => b"control.event.output.too_small",
            Self::Backpressure => b"control.event.backpressure",
            Self::Internal => b"control.event.internal",
        }
    }
}

/// What the live arm did with a structural token (#1257 D2).
pub(crate) enum LiveCommit {
    /// The delta was live: the arm committed it, or refused it with nothing changed.
    Done(Result<usize, CommandError>),
    /// The delta needs a plan rebuild; the token comes back untouched.
    Rebuild(ObservedPreparedToken),
    /// The token was prepared in place (#1613) and the fast classification could not judge it:
    /// the token comes back untouched, and the arm goes on with its full preparation. Never
    /// returned for a token that holds a compilation, and never leaves `commit_live`.
    NeedsFull(ObservedPreparedToken),
}

/// The EQ owners a live commit has begun (#1265 D2). Dropping it discards every owner still in
/// `begun`, so a refusal on any path after the first `begin_owner` leaves each owner idle at its
/// committed revision; a commit takes `begun` first.
struct OpenOwners<'a> {
    effects: &'a mut [host_core::EffectControlProducer],
    begun: Vec<usize>,
}

impl Drop for OpenOwners<'_> {
    fn drop(&mut self) {
        for &index in &self.begun {
            // An open candidate always discards; an error would mean it was never begun.
            let _ = self.effects[index].discard_owner();
        }
    }
}

/// Which event lane a dequeue reads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventLane {
    /// Reliable events, render diagnostics included.
    Reliable,
    /// Lossy telemetry.
    Lossy,
}

/// Why an event dequeue was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventError {
    /// The event needs `required` bytes.
    BufferTooSmall {
        /// Bytes the event needs.
        required: u64,
    },
    /// A queue has no room now; retry.
    Backpressure,
    /// The control plane's own bookkeeping failed.
    Internal,
}

/// Write one preflighted effect record as it is; a bypass record carries its own crossfade ramp
/// (#1341 D1, #1622 D1).
fn write_effect_record(
    producer: &mut host_core::EffectControlProducer,
    stamp: engine::realtime::LiveStamp<'_>,
    record: effect_contract::EffectControlRecord,
) {
    producer
        .write(stamp, record)
        .unwrap_or_else(|_| unreachable!("every effect record passed its producer's preflight"));
}

pub(crate) fn map_encode_error(error: EncodeError) -> CommandError {
    match error {
        EncodeError::OutputTooSmall { required } => CommandError::BufferTooSmall {
            required: u64::try_from(required).unwrap_or(u64::MAX),
        },
        EncodeError::MessageKindMismatch
        | EncodeError::LimitExceeded
        | EncodeError::InvalidField => CommandError::Internal,
    }
}

pub(crate) fn map_command_process_error(error: CommandFrameProcessError) -> CommandError {
    match error {
        CommandFrameProcessError::Uncorrelatable(_) => CommandError::Invalid,
        CommandFrameProcessError::Encode(error) => map_encode_error(error),
        CommandFrameProcessError::OutputReservationTooSmall { required } => {
            CommandError::BufferTooSmall {
                required: u64::try_from(required).unwrap_or(u64::MAX),
            }
        }
        CommandFrameProcessError::PreparedCommandOutstanding => CommandError::Backpressure,
        CommandFrameProcessError::Internal => CommandError::Internal,
    }
}

pub(crate) fn map_event_egress_error(error: EventEgressError) -> EventError {
    match error {
        EventEgressError::Encode(EncodeError::OutputTooSmall { required }) => {
            EventError::BufferTooSmall {
                required: u64::try_from(required).unwrap_or(u64::MAX),
            }
        }
        EventEgressError::ReliableQueueFull(_) | EventEgressError::ResponseReadyOccupied => {
            EventError::Backpressure
        }
        EventEgressError::Disabled
        | EventEgressError::DiagnosticStorageFull
        | EventEgressError::ObservationEndedCellMissing
        | EventEgressError::ResponseReadyMissing
        | EventEgressError::Encode(_) => EventError::Internal,
    }
}

impl<A: ControlAdapter> SessionState<A>
where
    PlanResources: From<A::Row>,
{
    /// Plans up to two structural faults for this thread, taken in order.
    #[cfg(feature = "test-support")]
    pub fn test_set_structural_faults(&mut self, faults: [Option<TestStructuralFaultPhase>; 2]) {
        let _ = self;
        TEST_FAULT_STATE.with(|state| state.set((faults, 0)));
    }

    /// The thread's lifecycle counters.
    #[cfg(feature = "test-support")]
    pub fn test_owner_counters(&self) -> TestOwnerCounters {
        let _ = self;
        TEST_OWNER_STATE.with(core::cell::Cell::get)
    }

    #[cfg(feature = "test-support")]
    pub(crate) fn take_test_fault(&mut self, phase: TestStructuralFaultPhase) -> bool {
        let _ = self;
        take_test_fault_state(phase)
    }

    /// Set the meter level and the batch fold for the accepted telemetry configuration once per
    /// accepted `TELEMETRY_CONFIGURE` (#1583 D5, #1588 D1): a nonempty table turns the level on
    /// from the next block, an empty one off, and the fold restarts with
    /// `meter_period_blocks / meter_window_blocks` windows per batch and sequence `1` next. The
    /// entries resolve again at the next frame (#1588 D3). The C ABI's level writes the requested
    /// gate, which render applies before its next block; a render-half fold's sets or clears the
    /// level word's telemetry bit, which the render half applies at its next poll. An exact
    /// replay answers from the cache and changes nothing here.
    pub(crate) fn sync_meter_configuration(&mut self) {
        let generation = self.controller.telemetry_configuration_generation();
        let Some(meters) = self.meters.as_mut() else {
            return;
        };
        let batches = &mut meters.batches;
        if batches.generation == generation {
            return;
        }
        batches.generation = generation;
        let telemetry = self.controller.telemetry_configuration();
        let on = !telemetry.meters.is_empty() && telemetry.meter_period_blocks != 0;
        let windows =
            core::num::NonZeroU32::new(telemetry.meter_period_blocks / batches.window_blocks.get())
                .unwrap_or(core::num::NonZeroU32::MIN);
        // One turn-off rule: an empty table owes the client nothing, so neither the open partial
        // batch nor the windows the delivery pops unread are counted lost.
        if !on {
            batches.fold.drop_partial();
        }
        batches.fold.restart(windows);
        batches.resolved_epoch = None;
        match &mut meters.source {
            MeterSource::Delivery { delivery, .. } => {
                let next_sample = self.shared.render_sample.load(Ordering::Acquire);
                delivery.set_level(on, next_sample);
            }
            MeterSource::Rendered { level, .. } => {
                level.set(host_core::MeterLevel::TELEMETRY, on);
            }
        }
    }

    /// Stage one `METER_BATCH` per batch the closed windows complete (#1583 D5, #1587a, #1588
    /// D4), from either source through [`MeterBatches::stage`].
    ///
    /// The C ABI's delivery publishes at most one frame per poll, so the loop polls until it
    /// publishes none, bounded by the window queue's depth and one more. It polls only while the
    /// delivery holds the running epoch's tables, with that epoch's strip meters.
    ///
    /// A render-half fold's frames are in the rings of the epochs render has run. Render fills a
    /// plan's ring only before it adopts the next plan, so the frames are staged in epoch order:
    /// every ring's count is loaded newest epoch first (whatever a newer ring holds, the older
    /// rings' frames before it are then seen), and each ring is drained oldest epoch first up to
    /// its count. With an empty table the frames are released unstaged. Bounded by the rings'
    /// depths.
    fn stage_meter_batches(&mut self, revision: protocol::SessionRevision) {
        let SessionState {
            meters,
            controller,
            providers,
            retired_providers,
            pending_providers,
            shared,
            ..
        } = self;
        let Some(meters) = meters.as_mut() else {
            return;
        };
        match &mut meters.source {
            MeterSource::Delivery { delivery, epoch } => {
                if *epoch != providers.epoch {
                    return;
                }
                let next_sample = shared.render_sample.load(Ordering::Acquire);
                for _ in 0..=host_core::METER_QUEUE_DEPTH {
                    let mut taps = CapiGainReductionTaps {
                        observations: &providers.observations,
                        strips: &providers.observation_strips,
                    };
                    let Some(frame) = delivery.poll(&mut providers.meters, &mut taps, next_sample)
                    else {
                        break;
                    };
                    let epochs = ProviderEpochs {
                        current: providers,
                        retired: retired_providers,
                        pending: pending_providers,
                    };
                    meters
                        .batches
                        .stage(frame, frame.plan_epoch, &epochs, controller, revision);
                }
            }
            MeterSource::Rendered { .. } => {
                let on = {
                    let telemetry = controller.telemetry_configuration();
                    !telemetry.meters.is_empty() && telemetry.meter_period_blocks != 0
                };
                let mut counts = [(0_u64, 0_usize); 1 + RETIRED_PROVIDERS + PENDING_PROVIDERS];
                let mut held = 0;
                let mut below = None;
                while let Some(provider) = (ProviderEpochs {
                    current: providers,
                    retired: retired_providers,
                    pending: pending_providers,
                })
                .all()
                .filter(|provider| below.is_none_or(|below| provider.epoch < below))
                .max_by_key(|provider| provider.epoch)
                {
                    below = Some(provider.epoch);
                    let available = provider
                        .meter_frames
                        .as_ref()
                        .map_or(0, host_core::MeterFrameConsumer::available);
                    if let Some(slot) = counts.get_mut(held) {
                        *slot = (provider.epoch, available);
                        held += 1;
                    }
                }
                for &(epoch, available) in counts[..held].iter().rev() {
                    for _ in 0..available {
                        let epochs = ProviderEpochs {
                            current: providers,
                            retired: retired_providers,
                            pending: pending_providers,
                        };
                        let Some(frame) = epochs
                            .all()
                            .find(|provider| provider.epoch == epoch)
                            .and_then(|provider| provider.meter_frames.as_ref())
                            .and_then(host_core::MeterFrameConsumer::peek)
                        else {
                            break;
                        };
                        if on {
                            meters
                                .batches
                                .stage(frame, epoch, &epochs, controller, revision);
                        }
                        if let Some(ring) = core::iter::once(&mut *providers)
                            .chain(retired_providers.iter_mut())
                            .chain(pending_providers.iter_mut())
                            .find(|provider| provider.epoch == epoch)
                            .and_then(|provider| provider.meter_frames.as_mut())
                        {
                            ring.release_frame();
                        }
                    }
                }
            }
        }
    }

    /// The windows every held epoch's ring dropped (#1588 R4).
    fn ring_drops(&self) -> u64 {
        core::iter::once(&self.providers)
            .chain(&self.retired_providers)
            .chain(&self.pending_providers)
            .filter_map(|provider| provider.meter_frames.as_ref())
            .map(host_core::MeterFrameConsumer::dropped_windows)
            .fold(0, u64::saturating_add)
    }

    /// Stages the `COUNTER_SNAPSHOT` event once render has completed at least the configured
    /// `counter_period_blocks` blocks since the last staged one (#1351 D4; the render sequence
    /// counts blocks). Its values are, for each configured ID in order, the provider's current
    /// value for that ID, which the service step refreshed first (D3). Blocks rendered with no
    /// control call in between yield one snapshot at the next call: the lane is lossy and skipped
    /// periods are not replayed.
    fn stage_counter_snapshot(
        &mut self,
        sequence: u64,
        observed_sample: protocol::SampleTime,
        revision: protocol::SessionRevision,
    ) {
        let telemetry = self.controller.telemetry_configuration();
        if telemetry.counter_ids.is_empty()
            || telemetry.counter_period_blocks == 0
            || sequence.saturating_sub(self.counter_snapshot_sequence)
                < u64::from(telemetry.counter_period_blocks)
        {
            return;
        }
        let ids = telemetry.counter_ids.iter().map(|&id| id as u32).collect();
        self.counter_snapshot_sequence = sequence;
        let Ok(snapshot) = protocol::ControlProvider::counters(
            self.controller.provider_mut(),
            &protocol::CountersRequest { all: false, ids },
        ) else {
            return;
        };
        let _ = self.controller.stage_counter_snapshot_event(
            revision,
            &protocol::CounterSnapshot {
                observed_sample,
                values: snapshot.values,
            },
        );
    }

    /// Stage a render-half fold's ring frames at a reclaim (#1588 D4), with the configuration the
    /// next render activity would stage them under. Nothing for the C ABI's delivery, whose
    /// adoption carries its frames.
    fn stage_reclaimed_frames(&mut self) {
        if !matches!(
            self.meters,
            Some(SessionMeters {
                source: MeterSource::Rendered { .. },
                ..
            })
        ) {
            return;
        }
        self.sync_meter_configuration();
        let revision = self.controller.session().revision();
        self.stage_meter_batches(revision);
    }

    pub(crate) fn collect_render_activity(&mut self) {
        self.sync_meter_configuration();
        let sequence = self.shared.render_sequence.load(Ordering::Acquire);
        // While no counter snapshot is configured the baseline follows render, so a
        // configuration counts its period from the call that accepted it, never from the first
        // block ever rendered.
        {
            let telemetry = self.controller.telemetry_configuration();
            if telemetry.counter_ids.is_empty() || telemetry.counter_period_blocks == 0 {
                self.counter_snapshot_sequence = sequence;
            }
        }
        if sequence != self.observed_render_sequence {
            if self.observed_render_sequence != self.diagnosed_render_sequence {
                // The owed block's diagnostic is replaced by this newer block's (#2198).
                self.render_diagnostics_coalesced =
                    self.render_diagnostics_coalesced.saturating_add(1);
            }
            self.observed_render_sequence = sequence;
            self.observed_render_sample = self.shared.render_sample.load(Ordering::Acquire);
            let observed_sample = protocol::SampleTime(self.observed_render_sample);
            let revision = self.controller.session().revision();
            self.observed_render_revision = revision;
            self.stage_meter_batches(revision);
            self.stage_counter_snapshot(sequence, observed_sample, revision);
        }
        self.stage_render_diagnostic();
    }

    /// Stage the render-activity diagnostic of the newest observed block, if one is owed. The
    /// diagnostic rides the reliable lane, but only through its two slots, which keep the newest
    /// (#2198): blocks observed while both slots are full coalesce into the one staged when a
    /// slot frees, which carries the newest block's sample, so the last block's activity is never
    /// lost. Each replaced owed block counts in `TELEMETRY_COALESCED`.
    fn stage_render_diagnostic(&mut self) {
        let sequence = self.observed_render_sequence;
        if sequence == self.diagnosed_render_sequence {
            return;
        }
        let diagnostics_enabled = {
            let configuration = self.controller.telemetry_configuration();
            configuration.diagnostics_enabled
                && (protocol::DiagnosticSeverity::Info as u8)
                    >= (configuration.minimum_diagnostic_severity as u8)
        };
        if !diagnostics_enabled {
            // A block rendered while diagnostics are off owes none.
            self.diagnosed_render_sequence = sequence;
            return;
        }
        if self.render_diagnostic_len == self.render_diagnostics.len() {
            return;
        }

        // A reliable-event reservation is the capacity credit for this CAPI-owned event. The
        // barrier records already-published protocol events, preserving their FIFO order while
        // the diagnostic itself remains in fixed, eagerly allocated CAPI storage.
        let protocol_events_before = self
            .controller
            .queues()
            .report(protocol::QueueKind::ReliableEvent)
            .occupancy
            .saturating_add(u64::from(self.protocol_reliable_pending));
        let Ok(reservation) = self.controller.queues_mut().reserve_reliable_event() else {
            return;
        };
        let sample = self.observed_render_sample;
        let revision = self.observed_render_revision;
        let tail = (self.render_diagnostic_head + self.render_diagnostic_len)
            % self.render_diagnostics.len();
        let slot = &mut self.render_diagnostics[tail];
        debug_assert!(!slot.occupied);
        debug_assert!(slot.reservation.is_none());
        slot.diagnostic.sample_time = Some(sample);
        slot.diagnostic.provider_sequence = Some(sequence);
        slot.revision = revision;
        slot.protocol_events_before = protocol_events_before;
        slot.reservation = Some(reservation);
        slot.occupied = true;
        self.controller
            .provider_mut()
            .set_render_diagnostic(tail, sample, sequence, true);
        self.render_diagnostic_len += 1;
        self.diagnosed_render_sequence = sequence;
    }

    /// `(revision, replay entries, current provider epoch, pending providers)`.
    #[cfg(feature = "test-support")]
    pub fn test_state_summary(&self) -> (u64, usize, u64, usize) {
        (
            self.controller.session().revision().0,
            self.controller.replay().len(),
            self.providers.epoch,
            self.pending_providers.len(),
        )
    }

    /// Everything a transaction may change.
    #[cfg(feature = "test-support")]
    pub fn test_transaction_snapshot(&self) -> TestTransactionSnapshot<A::Row> {
        let controller = self.controller.retained_configuration_capacity();
        let (provider, provider_counters) = self.controller.provider().retained_capacities();
        let replay = self.controller.replay().retained_storage_capacities();
        TestTransactionSnapshot {
            revision: self.controller.session().revision().0,
            canonical: self
                .controller
                .session()
                .canonical_snapshot()
                .as_bytes()
                .to_vec(),
            model: self.controller.session().compiled().resource_estimate(),
            replay_entries: self.controller.replay().len(),
            provider_epoch: self.providers.epoch,
            pending_provider_epochs: self
                .pending_providers
                .iter()
                .map(|provider| provider.epoch)
                .collect(),
            retired_provider_epochs: self
                .retired_providers
                .iter()
                .map(|provider| provider.epoch)
                .collect(),
            active_plan_epoch: self.shared.active_epoch.load(Ordering::Acquire),
            resource_rows: self
                .shared
                .reports
                .lock()
                .expect("test report lock")
                .clone(),
            reliable_event: self
                .controller
                .queues()
                .report(protocol::QueueKind::ReliableEvent),
            reliable_response: self
                .controller
                .queues()
                .report(protocol::QueueKind::ReliableResponse),
            telemetry: self
                .controller
                .queues()
                .report(protocol::QueueKind::Telemetry),
            telemetry_counters: self.controller.queues().telemetry_counters(),
            retained_capacities: [
                controller.meters,
                controller.counter_ids,
                provider.meters,
                provider.counter_ids,
                provider_counters,
                replay.0,
                replay.1,
            ],
            strip_cells: core::iter::once(&self.providers)
                .chain(&self.pending_providers)
                .flat_map(|provider| {
                    provider.strips.controls.iter().map(|producer| {
                        (
                            provider.epoch,
                            producer.track_id.clone(),
                            producer.fader.test_only_write_counts(),
                            producer.producer.test_only_write_count(),
                        )
                    })
                })
                .collect(),
            strip_offsets_cells: core::iter::once(&self.providers)
                .chain(&self.pending_providers)
                .flat_map(|provider| {
                    provider.strips.controls.iter().map(|producer| {
                        (
                            provider.epoch,
                            producer.track_id.clone(),
                            producer.fader.test_only_offsets_write_counts(),
                        )
                    })
                })
                .collect(),
            route_cells: core::iter::once(&self.providers)
                .chain(&self.pending_providers)
                .flat_map(|provider| {
                    provider.strips.routes.iter().map(|producer| {
                        (
                            provider.epoch,
                            Box::<str>::from(producer.route_id()),
                            producer.test_only_write_count(),
                        )
                    })
                })
                .collect(),
            follow_route_cells: core::iter::once(&self.providers)
                .chain(&self.pending_providers)
                .flat_map(|provider| {
                    provider.strips.follow_routes.iter().map(|producer| {
                        (
                            provider.epoch,
                            producer.route_id.clone(),
                            producer.test_only_write_count(),
                        )
                    })
                })
                .collect(),
            effect_cells: core::iter::once(&self.providers)
                .chain(&self.pending_providers)
                .flat_map(|provider| {
                    provider.effects.iter().map(|producer| {
                        (
                            provider.epoch,
                            producer.track_id.clone(),
                            producer.effect_id.clone(),
                            producer.writer().test_only_write_counts(),
                        )
                    })
                })
                .collect(),
            effect_owners: core::iter::once(&self.providers)
                .chain(&self.pending_providers)
                .flat_map(|provider| {
                    provider.effects.iter().filter_map(|producer| {
                        let owner = producer.owner()?;
                        let bits = |rows: &[effect_contract::InitialParameterValue]| {
                            rows.iter().map(|row| row.value.to_bits()).collect()
                        };
                        Some((
                            provider.epoch,
                            producer.track_id.clone(),
                            producer.effect_id.clone(),
                            owner.committed_revision(),
                            format!("{:?}", owner.phase()),
                            bits(owner.committed()),
                            bits(owner.candidate()),
                        ))
                    })
                })
                .collect(),
            observation_windows: core::iter::once(&self.providers)
                .chain(&self.pending_providers)
                .flat_map(|provider| {
                    provider.observations.iter().map(|handle| {
                        (
                            provider.epoch,
                            handle.track_id.clone(),
                            handle.effect_id.clone(),
                            handle
                                .readers
                                .iter()
                                .map(|reader| {
                                    reader.read().map(|window| {
                                        (
                                            window.sequence,
                                            window.first_sample,
                                            window.blocks,
                                            [window.left.to_bits(), window.right.to_bits()],
                                        )
                                    })
                                })
                                .collect(),
                        )
                    })
                })
                .collect(),
        }
    }

    /// The telemetry counters.
    #[cfg(feature = "test-support")]
    pub fn test_telemetry_counters(&self) -> protocol::TelemetryCounters {
        self.controller.queues().telemetry_counters()
    }

    /// The controller's, provider's and replay cache's retained capacities.
    #[cfg(feature = "test-support")]
    pub fn test_retained_capacities(&self) -> [usize; 7] {
        let controller = self.controller.retained_configuration_capacity();
        let (provider, provider_counters) = self.controller.provider().retained_capacities();
        let replay = self.controller.replay().retained_storage_capacities();
        [
            controller.meters,
            controller.counter_ids,
            provider.meters,
            provider.counter_ids,
            provider_counters,
            replay.0,
            replay.1,
        ]
    }

    /// The protocol controller.
    #[cfg(feature = "test-support")]
    pub fn test_controller(&self) -> &ProtocolController<SessionControlProvider> {
        &self.controller
    }

    /// The session's spectrum monitor (#1593): its configuration, epochs, service and analysis.
    /// Test support (#1597): every host reads spectrum through the session's frames.
    #[cfg(feature = "test-support")]
    #[must_use]
    pub const fn spectrum_monitor(&self) -> &crate::spectrum::SpectrumMonitor {
        &self.spectrum
    }

    /// The session's spectrum monitor, for a test's direct arm, read and stream calls on the
    /// service (#1593). Test support (#1597): no host calls the service but through the frames.
    #[cfg(feature = "test-support")]
    pub fn spectrum_monitor_mut(&mut self) -> &mut crate::spectrum::SpectrumMonitor {
        &mut self.spectrum
    }

    /// The output meter's reader and gate (#1581 D4), handed out once: `None` on every later call.
    /// The observer is the plan state's, so the windows the reader returns are the session's
    /// output over every plan it renders.
    pub fn take_output_meter(
        &mut self,
    ) -> Option<(host_core::OutputPeakReader, host_core::OutputPeakGate)> {
        self.output_meter.take()
    }

    /// Promote the oldest pending candidate now, before render has run its swap block (#1587b):
    /// what `withdraw_pending_candidate` does for a candidate render has claimed but not yet
    /// carried into, so a test can render the carry after the promotion.
    #[cfg(feature = "test-support")]
    pub fn test_promote_pending_before_carry(&mut self) -> Result<(), CommandError> {
        let epoch = self
            .pending_providers
            .first()
            .ok_or(CommandError::Internal)?
            .epoch;
        self.promote_pending_provider(epoch)
    }

    /// The render half of provider epoch `epoch`'s meter frame ring (#1588 D2), handed out once
    /// for the plan's render half: `None` for an epoch held no more or not yet, and on every later
    /// call. Only a render-half meter fold's plans have one. The running or a pending epoch.
    pub fn take_meter_frame_producer(
        &mut self,
        epoch: u64,
    ) -> Option<host_core::MeterFrameProducer> {
        core::iter::once(&mut self.providers)
            .chain(self.pending_providers.iter_mut())
            .find(|provider| provider.epoch == epoch)
            .and_then(|provider| provider.meter_frame_producer.take())
    }

    /// The meter level word of a render-half meter fold (#1588 D1), which the render half reads
    /// at each poll and its lease sets: a second handle on the session's word. `None` for any
    /// other adapter.
    #[must_use]
    pub fn meter_level(&self) -> Option<host_core::MeterLevel> {
        match &self.meters.as_ref()?.source {
            MeterSource::Rendered { level, .. } => Some(level.clone()),
            MeterSource::Delivery { .. } => None,
        }
    }

    /// The current provider epoch.
    #[cfg(feature = "test-support")]
    pub fn test_providers(&self) -> &ProviderEpoch {
        &self.providers
    }

    /// The key the newest epoch's gate had last published when this thread last queued an
    /// observation record, and resets it (#1968): `None` if no record was queued since.
    #[cfg(feature = "test-support")]
    pub fn test_take_observe_push_key(&self) -> Option<engine::realtime::GateKey> {
        TEST_OBSERVE_PUSH_KEY.with(core::cell::Cell::take)
    }

    /// The key the newest epoch's gate published last (#1968).
    #[cfg(feature = "test-support")]
    pub fn test_newest_gate_key(&self) -> engine::realtime::GateKey {
        self.pending_providers
            .last()
            .unwrap_or(&self.providers)
            .revision_gate
            .last_published_key()
    }

    /// The pending provider epochs, oldest first.
    #[cfg(feature = "test-support")]
    pub fn test_pending_providers(&self) -> &[ProviderEpoch] {
        &self.pending_providers
    }

    /// The retired provider epochs, waiting for their plans' reclamation.
    #[cfg(feature = "test-support")]
    pub fn test_retired_providers(&self) -> &[ProviderEpoch] {
        &self.retired_providers
    }

    /// The provider epoch `epoch`, current or pending, mutably (#1310 gate 6).
    #[cfg(feature = "test-support")]
    pub fn test_epoch_mut(&mut self, epoch: u64) -> Option<&mut ProviderEpoch> {
        core::iter::once(&mut self.providers)
            .chain(&mut self.pending_providers)
            .find(|provider| provider.epoch == epoch)
    }

    /// Gives the next plan this thread prepares the source-read offset `offset` (#1396
    /// deliverable 3): [`host_core::test_only_set_source_read_offset`] on that plan and its
    /// inventory row, before it is published. Taken once, by the next preparation on this thread.
    #[cfg(feature = "test-support")]
    pub fn test_set_next_source_read_offset(&mut self, offset: u64) {
        let _ = self;
        TEST_NEXT_SOURCE_READ_OFFSET.with(|state| state.set(Some(offset)));
    }

    /// The running plan's render clock, as render stored it after its last block
    /// ([`PlanPublisher::render_clock`], #1396 D4).
    #[cfg(feature = "test-support")]
    pub fn test_render_clock(&self) -> u64 {
        self.publisher.render_clock()
    }

    /// The running plan's source-read clock, as render stored it after its last block
    /// ([`PlanPublisher::source_read_clock`], #1396 D4).
    #[cfg(feature = "test-support")]
    pub fn test_source_read_clock(&self) -> u64 {
        self.publisher.source_read_clock()
    }

    /// Schedules the next structural publication's adoption (#1348 gate 4): `command` reserves
    /// that one candidate with `adoption` instead of [`PlanAdoption::Next`]. Taken once, by the
    /// next structural publication on this thread.
    #[cfg(feature = "test-support")]
    pub fn test_set_next_adoption(&mut self, adoption: PlanAdoption) {
        let _ = self;
        TEST_NEXT_ADOPTION.with(|state| state.set(Some(adoption)));
    }

    /// Makes the next transition re-preparation across a donor refuse after its preparation, as
    /// a defect would (#1397 deliverable 3). Taken once, by that re-preparation on this thread.
    #[cfg(feature = "test-support")]
    pub fn test_refuse_next_reprepare(&mut self) {
        let _ = self;
        TEST_REFUSE_REPREPARE.with(|state| state.set(true));
    }

    /// Makes the next `warm_lead` answer on this thread `Warm` with a zero lead that restarts
    /// `restart_whole` (#1397 D1's second arm), whatever the edit (#1397a). Taken once.
    #[cfg(feature = "test-support")]
    pub fn test_set_next_zero_lead(&mut self, restart_whole: Vec<&'static str>) {
        let _ = self;
        TEST_NEXT_ZERO_LEAD.with(|state| state.replace(Some(restart_whole)));
    }

    /// The warm-successor deadline step (#1358 D3), as the hosts' service steps will run it
    /// (#1360 D2, #1361).
    ///
    /// # Errors
    ///
    /// As the step itself (`check_prime_deadline`): an internal fault.
    #[cfg(feature = "test-support")]
    pub fn test_check_prime_deadline(&mut self) -> Result<Option<TransitionStep>, CommandError> {
        self.check_prime_deadline()
    }

    /// The session's `transition_reprepare_refusals` count (#1397 D2 step 3).
    #[cfg(feature = "test-support")]
    pub fn test_transition_reprepare_refusals(&self) -> u64 {
        self.transition_reprepare_refusals
    }

    /// The adoption the last structural publication on this thread was committed with (#1403
    /// gates), taken once: `None` when nothing was published since the last call.
    #[cfg(feature = "test-support")]
    pub fn test_take_published_adoption(&mut self) -> Option<PlanAdoption> {
        let _ = self;
        TEST_PUBLISHED_ADOPTION.with(core::cell::Cell::take)
    }

    /// The record of the pending warm candidate (#1403 D4), if one is pending.
    #[cfg(feature = "test-support")]
    pub fn test_primed(&self) -> Option<host_core::PrimedCandidate> {
        self.primed
    }

    /// The provider's counter snapshot, every counter, as `COUNTERS_GET` would read it now, without
    /// running a control call (and so without the service step's refresh).
    #[cfg(feature = "test-support")]
    pub fn test_provider_counters(&mut self) -> Vec<protocol::CounterValue> {
        use protocol::ControlProvider as _;
        self.controller
            .provider_mut()
            .counters(&protocol::CountersRequest {
                all: true,
                ids: Vec::new(),
            })
            .expect("every counter")
            .values
    }

    /// Every plan the control plane holds unreclaimed, as the admissions read them (#1398 D4), in
    /// epoch order, oldest first: every parked epoch whose plan render displaced but the service
    /// step has not yet reclaimed, then the running plan, then every pending or withdrawn
    /// candidate, the newest last.
    ///
    /// The running and pending rows are the provider's own epoch's, read from the report table,
    /// never the atomic's. A parked epoch's row is the one it captured when it was promoted out
    /// (`ProviderEpoch::parked`; batch 15a review fold 2, M-1): its plan is alive until the
    /// service step reclaims it from the retirement queue, so it is charged until then. The
    /// service step runs first in every command and reclaims every plan render has already
    /// handed back, so what stays parked is a plan render displaced but has not yet retired (a
    /// render preempted between its claim and its retirement), or one a `Taken` withdrawal
    /// promoted past before render retired it. A plan the atomic names while it lags is the
    /// running provider's or a parked one's, so it is charged once (#1042).
    pub(crate) fn held_epochs(&self) -> Result<HeldEpochs, CommandError> {
        let reports = self
            .shared
            .reports
            .lock()
            .map_err(|_| CommandError::Internal)?;
        let row = |provider: &ProviderEpoch| {
            reports
                .iter()
                .find_map(|(epoch, report)| (*epoch == provider.epoch).then_some(*report))
                .map(|report| LiveEpochResources {
                    report: PlanResources::from(report),
                    capi: provider.capi,
                    carried: provider.carried,
                })
                .ok_or(CommandError::Internal)
        };
        if self.pending_providers.len() > PENDING_PROVIDERS
            || self.retired_providers.len() > RETIRED_PROVIDERS
        {
            return Err(CommandError::Internal);
        }
        // No allocation: the live arm's admission runs it too.
        let running = row(&self.providers)?;
        let mut rows = [running; HELD_EPOCHS];
        let mut len = 0;
        for provider in &self.retired_providers {
            rows[len] = provider.parked.ok_or(CommandError::Internal)?;
            len += 1;
        }
        rows[len] = running;
        len += 1;
        for provider in &self.pending_providers {
            rows[len] = row(provider)?;
            len += 1;
        }
        Ok(HeldEpochs { rows, len })
    }

    /// The replacement admission of a candidate before it is prepared (issue #2194):
    /// [`validate_prospective_projection`] over every held plan and model and `prospective`'s
    /// effect projection, with the retryable or permanent split of the post-preparation
    /// admission: a refusal that fits beside the newest plan and the committed model alone is
    /// backpressure a render frees, and any other is permanent.
    fn admit_prospective_projection(
        &self,
        prospective: &session::CompiledSession,
    ) -> Result<(), CommandError> {
        let effects = host_core::project_effects(prospective)
            .map_err(|diagnostics| CommandError::CompileRejected(prepare_failure(diagnostics)))?;
        let held = self.held_epochs()?;
        let models = self.held_models()?;
        let admit = |held: &[LiveEpochResources], models: &[session::ResourceEstimate]| {
            validate_prospective_projection(
                held,
                effects,
                compiled_model_admission_over(models, prospective.resource_estimate())?,
                self.limits,
            )
        };
        let Err(failure) = admit(held.as_slice(), models.as_slice()) else {
            return Ok(());
        };
        Err(
            match held
                .as_slice()
                .split_last()
                .filter(|(_, older)| !older.is_empty())
            {
                Some((newest, _))
                    if admit(core::slice::from_ref(newest), &models.as_slice()[..1]).is_ok() =>
                {
                    CommandError::Backpressure
                }
                _ => CommandError::CompileRejected(failure),
            },
        )
    }

    /// Every compiled model the control plane holds, as both admissions charge them (#1398 D4,
    /// #1310 D3): the committed model, then each held epoch's kept model. A kept model lives only
    /// while a successor of its epoch is pending, so while one waits and a live edit is admitted,
    /// three models are charged with the prospective one. No allocation: the live arm runs it.
    ///
    /// Every held epoch has a row, so a kept model never lacks one; were it to, the charge would
    /// miss a model, so it is an accounting fault, never a skipped row.
    pub(crate) fn held_models(&self) -> Result<HeldModels, CommandError> {
        let committed = self.controller.session().compiled().resource_estimate();
        let mut models = HeldModels {
            rows: [committed; 2 + PENDING_PROVIDERS],
            len: 1,
        };
        for kept in core::iter::once(&self.providers)
            .chain(&self.pending_providers)
            .filter_map(|provider| provider.kept.as_ref())
        {
            let Some(slot) = models.rows.get_mut(models.len) else {
                debug_assert!(false, "a row for every held epoch's kept model");
                return Err(CommandError::Internal);
            };
            *slot = kept.model.resource_estimate();
            models.len += 1;
        }
        Ok(models)
    }

    /// Brings the provider epochs and the resource-report table up to the render thread.
    ///
    /// A render call swaps plans and retires the outgoing one at its *start*, but publishes
    /// `active_epoch` only after it returns (issue #1042). A control call in between reads an
    /// atomic that lags the plan actually rendering, so the atomic alone cannot drive promotion:
    /// a reclaimed plan whose epoch is still the current provider's proves that the oldest pending
    /// candidate has replaced it, and promotes that candidate there. After such a promotion the
    /// atomic lags the providers until the render call returns, so the atomic promotes only when
    /// it is ahead of them.
    ///
    /// **Drain first (batch 15a review fold 2, B-1).** Every reclaimable plan is reclaimed, and
    /// its parked epoch released, before the atomic promotes: a promotion parks epochs, and the
    /// parked list holds only as many as the retirement queue has credits, so promoting before
    /// the drain could find it full of epochs whose plans are already reclaimable and refuse
    /// forever. This order is defence in depth, not a tested gate: at the parked list's capacity
    /// of three each parked plan still holds a retirement credit, so the list has room for every
    /// epoch a promotion parks whichever order runs, and no test is red without the order (batch
    /// 15a fold 3, m-B).
    ///
    /// **Atomic before the drain (#1587a R1').** The atomic is loaded before the drain: render
    /// retires a displaced plan before it publishes the epoch that displaced it, so every plan
    /// displaced up to the loaded epoch is reclaimable by then, and the drain promotes through
    /// each one. Outside the test hold, then, every promotion is a reclaim's, which knows its
    /// successor's adoption sample (the reclaimed plan's own clock), and the atomic promotes
    /// nothing more.
    fn synchronize_plan_epochs(&mut self) -> Result<(), CommandError> {
        #[cfg(feature = "test-support")]
        let hold = TEST_HOLD_RECLAMATION.with(core::cell::Cell::get);
        #[cfg(not(feature = "test-support"))]
        let hold = false;
        let active_epoch = self.shared.active_epoch.load(Ordering::Acquire);
        while !hold && let Ok((retired_epoch, retired_plan)) = self.retirer.try_reclaim() {
            // #1699 D1/D3: a source the successor carried left a vacant entry, so what the
            // retired plan still holds unread is a removed or redeclared source's discarded PCM.
            self.source_frames_discarded = self
                .source_frames_discarded
                .saturating_add(retired_plan.unread_source_frames());
            // #1587a R1': the displaced plan's clock stopped where its successor's first block
            // started.
            let adoption_sample = retired_plan.next_absolute_sample();
            drop(ObservedRetiredPlan::new(retired_plan));
            if retired_epoch.0 == self.providers.epoch {
                let replacement = self
                    .pending_providers
                    .first()
                    .ok_or(CommandError::Internal)?
                    .epoch;
                self.promote_pending_provider(replacement)?;
            }
            self.adopt_meters(retired_epoch.0, adoption_sample);
            self.settle_observation_pairings();
            // #1588 D4: render filled the reclaimed plan's ring before it retired the plan, so
            // its frames are staged now, while their epoch's strips are still held.
            self.stage_reclaimed_frames();
            let index = self
                .retired_providers
                .iter()
                .position(|provider| provider.epoch == retired_epoch.0)
                .ok_or(CommandError::Internal)?;
            let reclaimed = self.retired_providers.remove(index);
            if let (
                Some(ring),
                Some(SessionMeters {
                    source: MeterSource::Rendered { retired_drops, .. },
                    ..
                }),
            ) = (reclaimed.meter_frames.as_ref(), self.meters.as_mut())
            {
                *retired_drops = retired_drops.saturating_add(ring.dropped_windows());
            }
        }

        if active_epoch > self.providers.epoch {
            self.promote_pending_provider(active_epoch)?;
        }
        self.settle_observation_pairings();
        // #1403 D4: render adopted the warm candidate, or a candidate published after it.
        if self
            .primed
            .is_some_and(|record| self.providers.epoch >= record.epoch)
        {
            self.primed = None;
        }

        // Keep exactly the rows a reader can still ask for: the atomic's (possibly lagging), the
        // current provider's, and every pending candidate's. The atomic is read under the lock,
        // and `retain` shrinks in place.
        let current = self.providers.epoch;
        let pending = &self.pending_providers;
        let mut reports = self
            .shared
            .reports
            .lock()
            .map_err(|_| CommandError::Internal)?;
        let active = self.shared.active_epoch.load(Ordering::Acquire);
        reports.retain(|(epoch, _)| {
            *epoch == active
                || *epoch == current
                || pending.iter().any(|provider| provider.epoch == *epoch)
        });
        Ok(())
    }

    /// Settle the pending observation pairings ([`CarriedTaps`]'s three states), oldest epoch
    /// first: one whose record render has written pairs as at promotion, and one whose displaced
    /// epoch is gone (its plan was reclaimed, so the hook has run or never will) settles as
    /// nothing carried. An epoch whose displaced epoch is itself still pending waits, so the
    /// readers always pair in adoption order. Run at each reclaim before the parked epoch drops
    /// (a record written by then pairs), at the end of each service step (after the drop, an
    /// unwritten one clears) and before each promotion. Bounded by the parked list; allocates
    /// nothing.
    fn settle_observation_pairings(&mut self) {
        for index in 0..=self.retired_providers.len() {
            let (successor, older) = if index == self.retired_providers.len() {
                (&mut self.providers, &mut self.retired_providers[..])
            } else {
                let (older, rest) = self.retired_providers.split_at_mut(index);
                let Some(successor) = rest.first_mut() else {
                    continue;
                };
                (successor, older)
            };
            let Some(displaced) = successor.observations_pending else {
                continue;
            };
            let settled = match older
                .iter_mut()
                .find(|provider| provider.epoch == displaced)
            {
                // Its own readers are not paired yet: pairing now would take the wrong ones.
                Some(predecessor) if predecessor.observations_pending.is_some() => false,
                Some(predecessor) => pair_observations(successor, predecessor),
                // The displaced epoch is gone: nothing is left to pair with.
                None => true,
            };
            if settled {
                successor.observations_pending = None;
            }
        }
    }

    /// Hand the meter delivery from the plan of `displaced`, whose reclaim this is, to its
    /// successor, adopted at `adoption_sample` (#1587a D1, R1', R3). Nothing happens unless the
    /// delivery holds `displaced`'s tables; the successor is the oldest promoted epoch after it,
    /// running or parked, since render adopts in publication order and a withdrawn candidate is
    /// never promoted. The successor's tables go to the delivery by pointer
    /// ([`host_core::MeterDelivery::adopt`]), with a [`host_core::MeterCarry`] for each strip
    /// meter its plan's swap block carried (`meter_pairing`, and the handles the two epochs'
    /// consumers publish under); the delivery's displaced tables go back to `displaced`'s parked
    /// epoch and drop with it, on this thread. A swap that carried no strip meter restarts the
    /// grid at `adoption_sample`, where render restarted the gate (`OutputPeakGateRelay::
    /// restart_at`). The entries are then resolved against the successor's strips.
    ///
    /// Run once per reclaimed plan, in reclaim order, so after a test hold's lagging promotions
    /// the delivery catches up one epoch per reclaim. Allocates nothing.
    fn adopt_meters(&mut self, displaced: u64, adoption_sample: u64) {
        let Some(SessionMeters {
            source: MeterSource::Delivery { delivery, epoch },
            ..
        }) = self.meters.as_mut()
        else {
            return;
        };
        if *epoch != displaced {
            return;
        }
        let Some(successor) = core::iter::once(&mut self.providers)
            .chain(self.retired_providers.iter_mut())
            .filter(|provider| provider.epoch > displaced)
            .min_by_key(|provider| provider.epoch)
        else {
            return;
        };
        let mut tables =
            core::mem::replace(&mut successor.meter_tables, host_core::MeterTables::empty());
        let successor_epoch = successor.epoch;
        let (Some(next), Some(previous)) = (
            core::iter::once(&self.providers)
                .chain(&self.retired_providers)
                .find(|provider| provider.epoch == successor_epoch),
            core::iter::once(&self.providers)
                .chain(&self.retired_providers)
                .find(|provider| provider.epoch == displaced),
        ) else {
            return;
        };
        let pairing = next
            .inventory
            .meter_pairing()
            .iter()
            .filter(|carry| carry.kind == engine::realtime::ObserverKind::Meter);
        let carried = pairing.clone().next().is_some();
        let carries = pairing.filter_map(|carry| {
            Some(host_core::MeterCarry {
                successor: u32::try_from(carry.successor).ok()?,
                predecessor: u32::try_from(carry.predecessor).ok()?,
                successor_handle: next.meters.get(carry.successor)?.handle.0.get(),
                predecessor_handle: previous.meters.get(carry.predecessor)?.handle.0.get(),
            })
        });
        // #1588 D3: the frames the successor's tables publish name its epoch.
        tables.set_plan_epoch(successor_epoch);
        delivery.adopt(&mut tables, carries, carried, adoption_sample);
        *epoch = successor_epoch;
        if let Some(previous) = core::iter::once(&mut self.providers)
            .chain(self.retired_providers.iter_mut())
            .find(|provider| provider.epoch == displaced)
        {
            previous.meter_tables = tables;
        }
    }

    /// Makes the pending provider of `epoch` current and parks the previous one until its plan
    /// is reclaimed.
    ///
    /// Every pending candidate older than `epoch` is parked with it (#1310): render adopts
    /// candidates in publication order, so one it adopted before `epoch` was displaced in turn.
    /// The service step promotes the candidate the atomic or a reclaimed plan names; a structural
    /// edit that finds the newest candidate taken promotes up to it before it rebuilds
    /// (`withdraw_pending_candidate`), so after that edit's commit one candidate at most is
    /// pending. Each parked epoch captures its report row first (`ProviderEpoch::parked`): the
    /// table drops the row, but the admissions charge the plan until it is reclaimed.
    fn promote_pending_provider(&mut self, epoch: u64) -> Result<(), CommandError> {
        let index = self
            .pending_providers
            .iter()
            .position(|provider| provider.epoch == epoch)
            .ok_or(CommandError::Internal)?;
        if self.retired_providers.len() + index + 1 > self.retired_providers.capacity() {
            return Err(CommandError::Internal);
        }
        // Fold 2, M-1: each parked epoch keeps the row the admissions charge until its plan is
        // reclaimed. Every row is read before anything moves, so a missing one moves nothing.
        let mut parked = [None; 1 + PENDING_PROVIDERS];
        {
            let reports = self
                .shared
                .reports
                .lock()
                .map_err(|_| CommandError::Internal)?;
            let rows = core::iter::once(&self.providers).chain(&self.pending_providers[..index]);
            for (slot, provider) in parked.iter_mut().zip(rows) {
                let report = reports
                    .iter()
                    .find_map(|(epoch, report)| (*epoch == provider.epoch).then_some(*report))
                    .ok_or(CommandError::Internal)?;
                *slot = Some(LiveEpochResources {
                    report: PlanResources::from(report),
                    capi: provider.capi,
                    carried: provider.carried,
                });
            }
        }
        for row in parked.into_iter().take(index + 1) {
            let mut next = self.pending_providers.remove(0);
            // #1768: render adopted `next`'s plan, so the strips it starts at rest restarted.
            let restarted =
                u64::try_from(next.inventory.restarted_strips().len()).unwrap_or(u64::MAX);
            self.strips_restarted = self.strips_restarted.saturating_add(restarted);
            // #1593 D5: render adopted `next`'s plan, so the service pairs the readers of the
            // observers its swap block moved and settles the others. A superseded candidate never
            // reaches here: it is dropped unpaired.
            let displaced = match core::mem::take(&mut next.spectrum) {
                crate::spectrum::EpochSpectrum::Candidate {
                    captures,
                    hop,
                    epoch,
                } => crate::spectrum::EpochSpectrum::Displaced(self.spectrum.adopt(
                    captures,
                    next.inventory.spectrum_pairing(),
                    hop,
                    epoch,
                )),
                other => other,
            };
            // #1674 R1: likewise its strip meters. The swap block moved each carried meter's
            // observer, producer included, into `next`'s plan, so the consumer the running epoch
            // holds follows it, and the parked epoch keeps `next`'s fresh one, whose producer
            // retires with the displaced plan.
            host_core::pair_carried_meters(
                &mut next.meters,
                &mut self.providers.meters,
                next.inventory.meter_pairing(),
            );
            // #1674's observation half (#1587b): likewise the observation readers of the taps the
            // swap block carried, from its record (what actually carried), so a carried tap's
            // reader, and its arm state, follow the tap that renders on. A record render has not
            // written yet leaves the pairing pending, which a later step settles. The pending
            // pairings settle first, and `next` waits while the running epoch's own still stands:
            // the narrow race is render carrying into B's plan and then claiming C's between an
            // edit's service step and its `withdraw_pending_candidate`, which promotes B and C in
            // one call. So the readers pair in adoption order without relying on claim order.
            self.settle_observation_pairings();
            if self.providers.observations_pending.is_some()
                || !pair_observations(&mut next, &mut self.providers)
            {
                next.observations_pending = Some(self.providers.epoch);
            }
            let mut previous = core::mem::replace(&mut self.providers, next);
            // #1690 D2: the promoted epoch brings its own master; the running one follows the
            // row again once nothing waits.
            sync_master_meters(
                &self.monitor,
                &mut self.providers,
                &mut self.pending_providers,
                &mut self.meters,
            );
            // Dropped with the parked epoch, when its plan is reclaimed.
            previous.spectrum = displaced;
            // #1310 D3: render no longer runs the previous plan, so no successor is prepared
            // against its kept base again; the base drops here, on the control thread.
            previous.kept = None;
            previous.parked = row;
            self.retired_providers.push(previous);
        }
        Ok(())
    }

    /// One bounded step of control work between edits (#1348 D1): the engine owns no thread, so
    /// the host drives it, through `miso_engine_v1_service` or any other session call, each of
    /// which runs it first (D2). In order, it
    ///
    /// 1. brings the provider epochs up to render and reclaims retired plans
    ///    (`synchronize_plan_epochs`);
    /// 2. refreshes the provider's counters that a source outside it serves (D8);
    /// 3. stages the render observation (`collect_render_activity`);
    /// 4. refreshes those counters again, so a record that staging coalesced or dropped is
    ///    counted in this same call.
    ///
    /// **Bounded (D3).** Every retired plan holds one retirement credit, the credits number the
    /// retirement queue's capacity, and only a publication -- never this step -- reserves one, so
    /// one call reclaims at most that capacity of plans. It stages at most one render observation
    /// and at most one meter batch per closed window the meter queue holds (#1583: its depth and
    /// one), never loops on render progress and never waits, so its cost does not grow with the
    /// time since the last call.
    ///
    /// **Pending candidates (D7).** Nothing here takes a pending candidate: it stays in its
    /// mailbox cell, with its plan, epoch, revision gate and retirement credit, until render adopts
    /// it or a publication withdraws it. A provider is promoted only once render has adopted its
    /// plan.
    ///
    /// **Acked-batch question (D6): can an ack ever precede a drop? No.** The step commits and
    /// acknowledges nothing and adds no queue. The only things it discards are plans render has
    /// already displaced; a committed revision's plan is never among them.
    pub fn service(&mut self) -> Result<(), CommandError> {
        self.fold_observation_pending();
        // #2115: at most the record queue's depth of hops.
        self.loudness.drain();
        self.synchronize_plan_epochs()?;
        self.poll_response_capture();
        self.refresh_provider_counters();
        self.collect_render_activity();
        self.refresh_provider_counters();
        Ok(())
    }

    /// Delivers the response capture (#1578 D4, R1): releases the capture once its event has
    /// left the controller's cell, takes the render step's reply (#1577 D5) as its
    /// `RESPONSE_READY` event, and moves the event into the controller.
    ///
    /// **Acked-batch question: can an ack precede a drop? No.** Every strip `RESPONSE_GET` that
    /// was answered a tag armed one capture, and the client delivers each capture's result once
    /// (a rebuild or a supersession re-arms it from the newest plan, and a strip the newest model
    /// no longer has is `NOT_FOUND`). The result becomes one event, held here until the controller
    /// queues it: a full queue, a held cell or an outstanding structural transaction keeps it for
    /// the next call, and an event the endpoint cannot encode is replaced by its refusal, never
    /// dropped. The capture is released, so a new request can arm, only after the event's frame
    /// reached the host.
    fn poll_response_capture(&mut self) {
        match &self.response_ready {
            #[cfg(feature = "test-support")]
            Some(PendingResponse::SeamArmed) => {
                let (provider, store) = self.controller.provider_and_session_mut();
                if let Some(ready) =
                    provider.poll_response_capture(store.compiled().normalized_model())
                {
                    self.response_ready = Some(PendingResponse::SeamResult(ready));
                }
                return;
            }
            #[cfg(feature = "test-support")]
            Some(PendingResponse::SeamResult(_)) => return,
            Some(PendingResponse::Event(_)) => {}
            None => {
                if !self.controller.response_ready_pending() {
                    // A no-op unless a delivered event has reached the host.
                    self.controller.provider_mut().release_response_capture();
                }
                let (provider, store) = self.controller.provider_and_session_mut();
                if let Some(ready) =
                    provider.poll_response_capture(store.compiled().normalized_model())
                {
                    self.response_ready = Some(PendingResponse::Event(
                        host_core::response_ready_event(ready),
                    ));
                }
            }
        }
        self.queue_response_event();
    }

    /// Moves a held `RESPONSE_READY` into the controller's cell (#1578 D4). A refusal keeps the
    /// event for the next `service`; an event the endpoint cannot encode becomes its refusal
    /// (`LIMIT_EXCEEDED` `response.reply_too_large` for one over the frame limit, `INTERNAL`
    /// `response.capture` otherwise), which is queued in its place.
    fn queue_response_event(&mut self) {
        for _ in 0..2 {
            let Some(PendingResponse::Event(event)) = self.response_ready.take() else {
                return;
            };
            let revision = self.controller.session().revision();
            let Err(refused) = self
                .controller
                .enqueue_response_ready_event(revision, event)
            else {
                return;
            };
            let mut event = refused.event;
            let retry = if let EventEgressError::Encode(error) = refused.reason {
                event.outcome = protocol::ResponseReadyOutcome::Refused {
                    status: if error == EncodeError::LimitExceeded {
                        protocol::StatusCode::LimitExceeded
                    } else {
                        protocol::StatusCode::Internal
                    },
                    diagnostic: if error == EncodeError::LimitExceeded {
                        "response.reply_too_large"
                    } else {
                        "response.capture"
                    }
                    .to_owned(),
                };
                true
            } else {
                false
            };
            self.response_ready = Some(PendingResponse::Event(event));
            if !retry {
                return;
            }
        }
    }

    /// Test seam (#1577 D5): arm one capture of `strip_id`'s response owners from the newest plan.
    ///
    /// # Errors
    ///
    /// The capture's refusal; a refusal changes nothing.
    #[cfg(feature = "test-support")]
    pub fn test_arm_track_response(
        &mut self,
        strip_id: &str,
        grid: host_core::ResponsePreviewGrid,
        channels: host_core::ResponseChannels,
    ) -> Result<u64, host_core::ResponseCaptureRefusal> {
        let (provider, store) = self.controller.provider_and_session_mut();
        let tag = provider.arm_response_capture(
            store.compiled().normalized_model(),
            strip_id,
            grid,
            channels,
        )?;
        self.response_ready = Some(PendingResponse::SeamArmed);
        Ok(tag)
    }

    /// Test seam (#1577): the response capture row the newest epoch's plan was charged
    /// (`CapiResources::response_capture`), the pending candidate's when one is pending.
    #[cfg(feature = "test-support")]
    pub fn test_response_capture_charge(&self) -> u64 {
        self.pending_providers
            .last()
            .unwrap_or(&self.providers)
            .capi
            .response_capture
    }

    /// Test seam (#1577 D5): take the delivered capture result and release the capture, as #1578
    /// does when the host dequeues its event.
    #[cfg(feature = "test-support")]
    pub fn test_take_track_response(&mut self) -> Option<host_core::ResponseReady> {
        let Some(PendingResponse::SeamResult(_)) = &self.response_ready else {
            return None;
        };
        let Some(PendingResponse::SeamResult(ready)) = self.response_ready.take() else {
            return None;
        };
        self.controller.provider_mut().release_response_capture();
        Some(ready)
    }

    /// Sets every counter the provider serves from a source outside it (#1348 D8): the
    /// controller's telemetry counters, the session's `live_values_superseded` count (#1312
    /// D8), its `strips_restarted` (#1768), `source_frames_discarded` (#1699) and
    /// `meter_windows_lost` (#1583 D5) counts and the render-side `live_block_errors` (#1521 D3)
    /// and `source_underrun_blocks` (#1537) mirrors.
    /// A counter added later joins this refresh, never a single control call.
    fn refresh_provider_counters(&mut self) {
        let mut telemetry_counters = self.controller.queues().telemetry_counters();
        telemetry_counters.telemetry_coalesced = telemetry_counters
            .telemetry_coalesced
            .saturating_add(self.render_diagnostics_coalesced);
        let superseded = self.live_values_superseded.get();
        let restarted = self.strips_restarted;
        let refusals = self.transition_reprepare_refusals;
        let errors = self.shared.live_block_errors.load(Ordering::Acquire);
        let underruns = self.shared.source_underrun_blocks.load(Ordering::Acquire);
        let discarded = self.source_frames_discarded;
        let rings = self.ring_drops();
        let meter_windows_lost = self
            .meters
            .as_ref()
            .map_or(0, |meters| meters.windows_lost(rings));
        let provider = self.controller.provider_mut();
        provider.set_telemetry_counters(telemetry_counters);
        provider.set_live_values_superseded(superseded);
        provider.set_strips_restarted(restarted);
        provider.set_transition_reprepare_refusals(refusals);
        provider.set_live_block_errors(errors);
        provider.set_source_underrun_blocks(underruns);
        provider.set_source_frames_discarded(discarded);
        provider.set_meter_windows_lost(meter_windows_lost);
    }

    /// Runs one control command frame. A success returns the response's length in the response
    /// scratch ([`Self::command_response`]); a structural command either rides the newest plan's
    /// live lanes or prepares, admits and publishes a replacement plan.
    ///
    /// **Floating-point environment (#1709 D1).** The whole command runs in
    /// `lane::CanonicalFpEnv`: the live classification composes each strip's effective fader
    /// (`controls::effective`, the VCA sum), the catalog stages its derived rows, and a rebuild
    /// prepares, all as a fresh preparation of the committed value does, whatever the host's
    /// control thread has set. Both hosts reach a transaction through here and nowhere else. The
    /// guard restores the caller's word on every return, an error included; a nested guard (the
    /// preparation's) saves and restores the canonical word. Render never calls this: it pins its
    /// own environment.
    pub fn command(&mut self, request: &[u8], output_capacity: u64) -> Result<usize, CommandError> {
        self.command_with_document(request, &[], output_capacity)
    }

    /// [`Self::command`] with the document a `SESSION_REPLACE` frame names (#1386 D3), borrowed for
    /// this call only: nothing here or in the controller keeps it. Every command reaches this one
    /// path; `document` is read only by a `SESSION_REPLACE`, after its replay preflight, its
    /// exact-revision check and its output reservation. The controller's document hook checks its
    /// length (against the frame and against `maximum_document_bytes`), its SHA-256, parses it
    /// and diffs it against the committed model ([`crate::replace_document`]); the edits then
    /// take this function's transaction path exactly: the live commit or the rebuild, the rate and quantum
    /// refusal, the plan admission and the `SESSION_COMMITTED` event. A document equal to the
    /// committed model answers `EditPath::Unchanged` and commits nothing.
    pub fn command_with_document(
        &mut self,
        request: &[u8],
        document: &[u8],
        output_capacity: u64,
    ) -> Result<usize, CommandError> {
        self.command_with_readers(request, document, output_capacity, None)
    }

    /// [`Self::command_with_document`] for a host that holds the session's plan beside it, between
    /// blocks (the browser, whose one instance renders and serves control), lent for this call
    /// only:
    ///
    /// - A `SPECTRUM_CONFIGURE` that prepares a successor before the plan's first block has the
    ///   plan adopt it in place (#1597, through
    ///   [`engine::realtime::RealtimePlanOwner::adopt_before_first_block`]) and promotes it, so
    ///   its reply reports `adopted_epoch == configured_epoch` and a capture armed now covers
    ///   the first block. No render has adopted anything yet, so this is the plan the first block
    ///   would have adopted: a boot configures its taps through the command and still gets them in
    ///   the plan that renders sample 0.
    ///
    /// Every other command ignores the plan.
    pub fn command_with_plan(
        &mut self,
        request: &[u8],
        document: &[u8],
        output_capacity: u64,
        plan: &mut PlanState<A>,
    ) -> Result<usize, CommandError> {
        self.command_with_readers(request, document, output_capacity, Some(LentPlan { plan }))
    }

    /// The `OBSERVATION_READ` index of the plan of epoch `epoch` (#1586 D6, #1589 R2): the
    /// running plan's or a pending candidate's, `None` for an epoch the session does not hold. A
    /// host that keeps the readers in its plan reads them through it, with the epoch of the plan
    /// whose readers it reads.
    #[must_use]
    pub fn observation_index(
        &self,
        epoch: u64,
    ) -> Option<&host_core::observation::ObservationIndex> {
        core::iter::once(&self.providers)
            .chain(&self.pending_providers)
            .find(|provider| provider.epoch == epoch)
            .map(|provider| &provider.observation_index)
    }

    /// The body of [`Self::command_with_document`], with the plan a host lends.
    fn command_with_readers(
        &mut self,
        request: &[u8],
        document: &[u8],
        output_capacity: u64,
        lent: Option<LentPlan<'_, A>>,
    ) -> Result<usize, CommandError> {
        let _canonical = lane::CanonicalFpEnv::enter();
        self.service()?;
        let output_capacity = usize::try_from(output_capacity).unwrap_or(usize::MAX);
        // #1613: a transaction of values is prepared in place (no clone, no compile); every other
        // command is prepared as it always was. A test can force the full preparation.
        #[cfg(feature = "test-support")]
        let in_place = !FORCE_FULL_PREPARATION.with(core::cell::Cell::get);
        #[cfg(not(feature = "test-support"))]
        let in_place = true;
        let mut scratch = DecodeScratch::new(&mut self.decode_fields);
        // #1584 D3, D7a: a transaction's `Observe` edits are checked against the binding table the
        // service step just folded, which the session keeps; the adapter says whether it serves
        // them at all.
        let observe = protocol::ObserveContext {
            bindings: &self.observation,
            // #1589 R1: only a session whose plans bind every tap's queue serves the edit.
            enabled: A::OBSERVE_EDIT && self.preparer.observation_queues(),
            // #1585 D4: the records are tagged from the session's one counter.
            next_arm_tag: self.next_arm_tag,
        };
        let prepared = if in_place {
            self.controller.prepare_command_frame_in_place_observed(
                request,
                document,
                &mut scratch,
                output_capacity,
                observe,
            )
        } else {
            self.controller.prepare_command_frame_observed(
                request,
                document,
                &mut scratch,
                output_capacity,
                observe,
            )
        }
        .map_err(map_command_process_error)?;
        match prepared {
            PreparedCommandFrame::Immediate(response) => {
                let written = response
                    .write_into(&mut self.response_scratch)
                    .map_err(map_encode_error);
                // #1583 D5: an accepted `TELEMETRY_CONFIGURE` sets the meter level and the fold.
                self.sync_meter_configuration();
                written
            }
            // #1594 D9: a command the session executes.
            PreparedCommandFrame::Session(prepared) => {
                self.session_command(*prepared, lent.map(|lent| lent.plan))
            }
            // #1586 D6: the read, which allocates nothing.
            PreparedCommandFrame::ObservationRead(prepared) => {
                self.observation_read(prepared, request)
            }
            PreparedCommandFrame::Structural(prepared) => {
                let prepared = ObservedPreparedToken::new(prepared);
                #[cfg(feature = "test-support")]
                {
                    if self.take_test_fault(TestStructuralFaultPhase::AfterProtocolPrepare) {
                        drop(prepared);
                        return Err(CommandError::Backpressure);
                    }
                }
                if !self.shared.plan_alive.load(Ordering::Acquire) {
                    return Err(CommandError::Backpressure);
                }
                let response_len = prepared.get().response_len();
                if response_len > self.response_scratch.len() || response_len > output_capacity {
                    return Err(CommandError::BufferTooSmall {
                        required: u64::try_from(response_len).unwrap_or(u64::MAX),
                    });
                }
                // #1257 D1: a value-only delta of track faders, mutes, pans and live effect
                // parameters (#1264) rides the newest plan's live lanes; every other delta takes
                // the rebuild below, unchanged.
                let mut prepared = match self.commit_live(prepared) {
                    LiveCommit::Done(result) => return result,
                    LiveCommit::Rebuild(prepared) => prepared,
                    // `commit_live` materializes an in-place token before it returns.
                    LiveCommit::NeedsFull(_) => return Err(CommandError::Internal),
                };
                // #1290a: a plan's rate and quantum are fixed for its session: the plan exchange
                // admits a replacement only at the same envelope, and would hand a different one
                // back as retryable backpressure. A transaction that changes either is refused
                // here, typed and permanent, before anything is prepared, stamped or queued.
                {
                    let prospective = prepared
                        .get()
                        .prospective_session()
                        .compiled()
                        .ok_or(CommandError::Internal)?;
                    let committed = self.controller.session().compiled();
                    if prospective.sample_rate() != committed.sample_rate()
                        || prospective.quantum() != committed.quantum()
                    {
                        drop(prepared);
                        return Err(CommandError::CompileRejected(crate::error::diagnostic(
                            REPREPARE_REQUIRED,
                        )));
                    }
                }
                #[cfg(feature = "test-support")]
                if self.take_test_fault(TestStructuralFaultPhase::BeforeRuntimePrepare) {
                    drop(prepared);
                    return Err(CommandError::Backpressure);
                }
                // #1277 D10: `r`, the revision this rebuild commits as, is read once, here: the
                // successor's base carries it, the preparation stamps its carry retargets with it
                // on the candidate's own gate pin, and the stamp below is the same value again.
                // #1503 D1 holds a prepared transaction's revision in range, so this cannot
                // refuse; were it to, nothing has been built. A transaction of monitor writes
                // only never takes this arm (`commit_live` refuses it), so this revision is
                // always the one the commit gives (#1603).
                let revision = prepared.get().prospective_session().revision().0;
                if !(1..=engine::realtime::GATE_REVISION_MAX).contains(&revision) {
                    return Err(CommandError::Internal);
                }
                // #1559 D9: the successor renders under the overlay the transaction leaves, and
                // its carry joins the effective values under both overlays.
                let monitor_change = self.monitor.after(
                    prepared.get().monitor_edits(),
                    prepared
                        .get()
                        .prospective_session()
                        .compiled()
                        .ok_or(CommandError::Internal)?
                        .normalized_model(),
                );
                // #1584 D6a: the subscriptions the successor is prepared with, and those the
                // transaction ends. The room for their one `OBSERVATION_ENDED` event (every ended
                // key in it) is reserved with the token's `SESSION_COMMITTED`, before anything is
                // prepared, published or queued: a lane holding unread events is retryable
                // backpressure and nothing has changed (the acked-batch rule: a subscription is
                // never dropped without its event). An event over one control frame can never
                // be sent, so it is a resource refusal, not backpressure.
                // #1584 T2: the table the transaction's `Observe` edits leave, when it holds any, so
                // the successor's prepared arms are the post-transaction subscriptions and a target
                // the same transaction removes is ended, not armed.
                let observation_change = prepared
                    .get()
                    .prospective_session()
                    .observe_bindings()
                    .unwrap_or(&self.observation)
                    .after(
                        prepared
                            .get()
                            .prospective_session()
                            .compiled()
                            .ok_or(CommandError::Internal)?
                            .normalized_model(),
                    )
                    .map_err(|_| {
                        CommandError::CompileRejected(failure(ResourceFault::Allocation))
                    })?;
                if !observation_change.ended.is_empty() {
                    let mut ended = Vec::new();
                    ended
                        .try_reserve_exact(observation_change.ended.len())
                        .map_err(|_| {
                            CommandError::CompileRejected(failure(ResourceFault::Allocation))
                        })?;
                    ended.extend(observation_change.ended.iter().map(ended_key));
                    match self
                        .controller
                        .attach_observation_ended(prepared.get_mut(), ended)
                    {
                        Ok(_) => {}
                        Err(protocol::AttachObservationEndedError::NoRoom(_)) => {
                            drop(prepared);
                            return Err(CommandError::Backpressure);
                        }
                        Err(protocol::AttachObservationEndedError::Unencodable) => {
                            drop(prepared);
                            return Err(CommandError::CompileRejected(failure(
                                ResourceFault::Limit,
                            )));
                        }
                        Err(_) => return Err(CommandError::Internal),
                    }
                }
                self.rebuild_superseding(
                    Rebuild::Transaction(prepared),
                    revision,
                    monitor_change,
                    observation_change,
                )
            }
        }
    }

    /// Declares a discontinuity (#1323 D1, D2): the next block the host renders need not continue
    /// the previous output, because the host has stopped or is about to seek every source. Both
    /// hosts call this one function: `miso_engine_v1_declare_discontinuity` and
    /// `miso_engine_web_v1_declare_discontinuity` only forward to it.
    ///
    /// After the service step, it rebuilds only if the newest plan (the pending candidate, else
    /// the running plan) runs behind its natural timing
    /// ([`PlanStateInventory::runs_behind_natural_timing`]): a floor raised above a natural
    /// arrival, or a source-read offset above 0 (#1396 D2), which a plan can hold with no raised
    /// floor. The successor's offset is 0. It then prepares the committed model
    /// as a successor at a discontinuity (`SuccessorBase::discontinuity`): no floors, only the
    /// source consumers carried, every other owner at rest. It is published
    /// [`PlanAdoption::Next`] with the committed revision and its retirement credit reserved, as a
    /// structural transaction's plan is. A pending candidate is superseded first (#1310): withdrawn,
    /// the donor of the rings it created, its persisting producers returned to the running epoch,
    /// then dropped; its revisions complete when the successor is adopted, `EXACT`, with
    /// `SUPERSEDED` for the revisions it had itself folded in (D4), never as a fallback. A `Primed`
    /// warm candidate always runs behind its natural timing, so a declaration always supersedes
    /// one. Otherwise it publishes nothing and returns `Ok`; so does a session at revision 0,
    /// which has no successor and so no floor (root ruling B15-R6).
    ///
    /// **Acked-batch question (D4): can an ack ever precede a drop? No.** The declaration
    /// acknowledges no edit and changes neither the committed model nor the revision. A refusal
    /// (a preparation or admission error, backpressure) changes nothing: a withdrawn candidate is
    /// published again whole, and the running plan keeps its floors. A superseded candidate's
    /// edits are in the committed model the successor is compiled from, and its fed rings move
    /// by the checked donation or the supersession is refused (`drops_acked_fresh_source`).
    ///
    /// # Errors
    ///
    /// [`CommandError::CompileRejected`] when the preparation or the replacement admission
    /// refuses; [`CommandError::Backpressure`] when the plan is gone, when a retry can fit, or
    /// when a superseded candidate's fed fresh ring could not move; [`CommandError::Internal`]
    /// for an accounting fault.
    pub fn declare_discontinuity(&mut self) -> Result<(), CommandError> {
        self.service()?;
        self.declare_discontinuity_serviced()
    }

    /// [`Self::declare_discontinuity`] after its service step: the one core path of a declaration,
    /// which a plain session seek also runs (B16-R9).
    fn declare_discontinuity_serviced(&mut self) -> Result<(), CommandError> {
        // #1709 D1: the successor's carried level values are composed as `Self::command`'s are.
        let _canonical = lane::CanonicalFpEnv::enter();
        if !self
            .newest_providers()
            .inventory
            .runs_behind_natural_timing()
        {
            return Ok(());
        }
        if !self.shared.plan_alive.load(Ordering::Acquire) {
            return Err(CommandError::Backpressure);
        }
        // D4: the successor carries the committed revision. A plan behind its natural timing is a
        // successor, so the committed revision is at least 1 (B15-R6).
        let revision = self.controller.session().revision().0;
        if !(1..=engine::realtime::GATE_REVISION_MAX).contains(&revision) {
            return Err(CommandError::Internal);
        }
        let model = Box::new(self.controller.session().compiled().clone());
        // Nothing changes the overlay or the subscriptions: the successor renders under the ones
        // the newest plan holds, and every binding keeps its target.
        let monitor_change = self.monitor.after(&[], model.normalized_model());
        let observation_change = self
            .observation
            .after(model.normalized_model())
            .map_err(|_| CommandError::CompileRejected(failure(ResourceFault::Allocation)))?;
        if !observation_change.ended.is_empty() {
            return Err(CommandError::Internal);
        }
        self.rebuild_superseding(
            Rebuild::Discontinuity(model),
            revision,
            monitor_change,
            observation_change,
        )
        .map(|_| ())
    }

    /// Executes a command the session runs (#1594 D9) and completes it: a reply or a typed
    /// refusal is recorded for replay with the response, so an exact replay answers the same
    /// bytes and reaches here no second time. A retryable or internal failure drops the token:
    /// nothing is recorded, nothing is acknowledged, and the same request may be sent again.
    fn session_command(
        &mut self,
        prepared: protocol::PreparedSessionCommand,
        plan: Option<&mut PlanState<A>>,
    ) -> Result<usize, CommandError> {
        let request = match prepared.command() {
            protocol::SessionCommand::SpectrumConfigure(request) => request,
            protocol::SessionCommand::SpectrumCapture(request) => {
                let outcome = self.spectrum_capture(request);
                return self.complete_spectrum_capture(prepared, outcome);
            }
            protocol::SessionCommand::SpectrumStream(request) => {
                let outcome = self.spectrum_stream(request);
                return self.complete_spectrum_stream(prepared, outcome);
            }
            protocol::SessionCommand::Loudness(request) => {
                let render_sample = self.shared.render_sample.load(Ordering::Acquire);
                return match self.loudness.apply(*request, render_sample) {
                    Ok(reply) => self
                        .controller
                        .complete_session_command(
                            prepared,
                            protocol::SessionCommandReply::Loudness(reply),
                            &mut self.response_scratch,
                        )
                        .map_err(map_command_process_error),
                    // #2115: a start while armed changes nothing.
                    Err(crate::loudness::LoudnessActive) => self.complete_spectrum_refusal(
                        prepared,
                        protocol::StatusCode::ValidationFailed,
                        "loudness.active",
                    ),
                };
            }
        };
        let outcome = self.spectrum_configure(request, plan);
        match outcome {
            Ok(reply) => self
                .controller
                .complete_session_command(
                    prepared,
                    protocol::SessionCommandReply::SpectrumConfigured(reply),
                    &mut self.response_scratch,
                )
                .map_err(map_command_process_error),
            Err(SpectrumRefusal::Reply(status, code)) => {
                self.complete_spectrum_refusal(prepared, status, &code)
            }
            Err(SpectrumRefusal::Error(error)) => {
                drop(prepared);
                Err(error)
            }
        }
    }

    /// Executes an `OBSERVATION_READ` (#1586 D3, D4, D6) on the running plan's readers and
    /// completes it. The selections are decoded from `request` into a table on the stack, borrowed
    /// from its bytes. The whole batch is checked before a row is written: a reply larger than the
    /// frame limit is `LIMIT_EXCEEDED` (`observation.frame`), and the first selection that does not
    /// resolve is refused as D4 says, naming `selection[i]`; then every row is read, through
    /// [`host_core::observation::read_with`], into the session's row table, and the reply borrows
    /// it until it is encoded. The read writes no slot and acknowledges nothing, so it consumes no
    /// window. A read that answers a page allocates nothing (gate 6).
    ///
    /// **Acked-batch question: can a reply claim a row that was not written? No.** The reply
    /// covers exactly the rows `read_with` wrote, which it writes only once the whole batch
    /// passed, and a row's arm state is the slot's published state (render applied it) or none.
    fn observation_read(
        &mut self,
        prepared: protocol::PreparedObservationRead,
        request: &[u8],
    ) -> Result<usize, CommandError> {
        let mut selections = [protocol::ObservationSelectionRef::EMPTY;
            protocol::OBSERVATION_READ_MAXIMUM_SELECTIONS];
        let codec = *self.controller.codec();
        let Ok(count) = codec.decode_observation_read_frame(request, &mut selections) else {
            // The controller decoded the same bytes before it prepared the read.
            drop(prepared);
            return Err(CommandError::Internal);
        };
        let fits = codec
            .observation_read_largest_frame_len(count)
            .is_ok_and(|bytes| bytes <= self.response_scratch.len());
        if !fits {
            return self
                .controller
                .complete_observation_read_refused(
                    prepared,
                    request,
                    protocol::StatusCode::LimitExceeded,
                    "observation.frame",
                    None,
                    &mut self.response_scratch,
                )
                .map_err(map_command_process_error);
        }
        let mut rows = core::mem::take(&mut self.observation_rows);
        let outcome = match rows.get_mut(..count) {
            Some(table) => self.read_observation_rows(&selections[..count], table),
            None => Err(None),
        };
        let observed_sample = self.shared.render_sample.load(Ordering::Acquire);
        let result = match outcome {
            Ok(()) => self
                .controller
                .complete_observation_read(
                    prepared,
                    request,
                    protocol::ObservationReadPage {
                        observed_sample,
                        rows: std::borrow::Cow::Borrowed(&rows[..count]),
                    },
                    &mut self.response_scratch,
                )
                .map_err(map_command_process_error),
            Err(Some(error)) => match observation_refusal(error.refusal) {
                Some((status, code)) => self
                    .controller
                    .complete_observation_read_refused(
                        prepared,
                        request,
                        status,
                        code,
                        Some((
                            "selection",
                            u64::try_from(error.selection).unwrap_or(u64::MAX),
                        )),
                        &mut self.response_scratch,
                    )
                    .map_err(map_command_process_error),
                None => {
                    drop(prepared);
                    Err(CommandError::Internal)
                }
            },
            Err(None) => {
                drop(prepared);
                Err(CommandError::Internal)
            }
        };
        self.observation_rows = rows;
        result
    }

    /// Reads one row per selection into `rows` through the running plan's index, whose read-only
    /// views follow the slots that render (#1332 S6b: on any control thread, whichever thread the
    /// acknowledging readers are on), with the whole batch checked first (#1586 D4): the core read
    /// and the wire rows, no allocation. `Err(None)` for a refusal the codec rules out (an internal
    /// fault).
    fn read_observation_rows(
        &self,
        selections: &[protocol::ObservationSelectionRef<'_>],
        rows: &mut [protocol::ObservationReadRow],
    ) -> Result<(), Option<host_core::observation::ObservationReadError>> {
        host_core::observation::read_with(
            &self.providers.observation_index,
            selections,
            |selection| {
                Ok::<_, host_core::observation::ObservationReadError>(core_selection(selection))
            },
            |key| {
                self.observation_issued(&ObservationKeyRef {
                    strip_id: key.strip_id.as_bytes(),
                    rack: key.section,
                    effect_id: key.effect_id.as_bytes(),
                    tap_id: key.tap_id,
                })
            },
            |position, _, row| {
                if let Some(slot) = rows.get_mut(position) {
                    *slot = wire_row(&row);
                }
            },
        )
        .map_err(Some)
    }

    /// Completes a spectrum command's typed refusal, recorded for replay: a `BACKPRESSURE` with
    /// the arm slot's record (#1944), any other status without one.
    fn complete_spectrum_refusal(
        &mut self,
        prepared: protocol::PreparedSessionCommand,
        status: protocol::StatusCode,
        code: &str,
    ) -> Result<usize, CommandError> {
        if status == protocol::StatusCode::Backpressure {
            self.controller
                .complete_session_command_refused_backpressure(
                    prepared,
                    code,
                    SPECTRUM_ARM_BACKPRESSURE,
                    &mut self.response_scratch,
                )
        } else {
            self.controller.complete_session_command_refused(
                prepared,
                status,
                code,
                &mut self.response_scratch,
            )
        }
        .map_err(map_command_process_error)
    }

    /// Completes a `SPECTRUM_CAPTURE` with its reply or its refusal, as [`Self::session_command`]
    /// completes a configure. A ready reply borrows the monitor's result and the window's planes
    /// until it is encoded.
    fn complete_spectrum_capture(
        &mut self,
        prepared: protocol::PreparedSessionCommand,
        outcome: Result<CaptureOutcome, SpectrumRefusal>,
    ) -> Result<usize, CommandError> {
        let outcome = match outcome {
            Ok(outcome) => outcome,
            Err(SpectrumRefusal::Reply(status, code)) => {
                return self.complete_spectrum_refusal(prepared, status, &code);
            }
            Err(SpectrumRefusal::Error(error)) => {
                drop(prepared);
                return Err(error);
            }
        };
        let monitor = &self.spectrum;
        let service = monitor.service();
        let (status, window) = match &outcome {
            CaptureOutcome::Done => (protocol::SpectrumCaptureStatus::Done, None),
            CaptureOutcome::Pending => (protocol::SpectrumCaptureStatus::Pending, None),
            CaptureOutcome::Ready { include_window } => {
                let (Some(result), Some(snapshot), Some(target)) = (
                    monitor.result(),
                    monitor.snapshot(),
                    service.and_then(host_core::SpectrumService::target),
                ) else {
                    drop(prepared);
                    return Err(CommandError::Internal);
                };
                (
                    protocol::SpectrumCaptureStatus::Ready,
                    Some(capture_window(result, target, snapshot, *include_window)),
                )
            }
        };
        let reply = protocol::SpectrumCaptureReply {
            adopted_epoch: monitor.adopted_epoch(),
            selection_epoch: service.map_or(0, host_core::SpectrumService::selection_epoch),
            status,
            window,
        };
        self.controller
            .complete_session_command(
                prepared,
                protocol::SessionCommandReply::SpectrumCaptured(reply),
                &mut self.response_scratch,
            )
            .map_err(map_command_process_error)
    }

    /// Completes a `SPECTRUM_STREAM` with its reply or its refusal, as
    /// [`Self::complete_spectrum_capture`] completes a capture. A ready reply borrows the
    /// monitor's result until it is encoded.
    fn complete_spectrum_stream(
        &mut self,
        prepared: protocol::PreparedSessionCommand,
        outcome: Result<StreamOutcome, SpectrumRefusal>,
    ) -> Result<usize, CommandError> {
        let outcome = match outcome {
            Ok(outcome) => outcome,
            Err(SpectrumRefusal::Reply(status, code)) => {
                return self.complete_spectrum_refusal(prepared, status, &code);
            }
            Err(SpectrumRefusal::Error(error)) => {
                drop(prepared);
                return Err(error);
            }
        };
        let monitor = &self.spectrum;
        let service = monitor.service();
        let metadata = &outcome.metadata;
        let status = u8::try_from(metadata.status.code())
            .ok()
            .and_then(|code| protocol::SpectrumStreamStatus::from_code(code).ok());
        let bins = match (outcome.ready, monitor.result()) {
            (false, _) => None,
            (true, Some(result)) => Some(stream_bins(result)),
            (true, None) => None,
        };
        let (Some(status), Some(service)) = (status, service) else {
            drop(prepared);
            return Err(CommandError::Internal);
        };
        if outcome.ready && bins.is_none() {
            drop(prepared);
            return Err(CommandError::Internal);
        }
        let target = service
            .target()
            .zip(service.channels())
            .map(|(target, channels)| protocol::SpectrumStreamTarget {
                target_kind: protocol_target_kind(target),
                target_id: std::borrow::Cow::Borrowed(spectrum_target_id(target)),
                channels: protocol_channels(channels),
            });
        let reply = protocol::SpectrumStreamReply {
            adopted_epoch: monitor.adopted_epoch(),
            selection_epoch: service.selection_epoch(),
            status,
            target,
            sample_rate_hz: metadata.sample_rate_hz,
            quantum_frames: metadata.quantum_frames,
            hop_frames: metadata.hop_frames,
            capture_epoch: metadata.capture_epoch,
            sequence: metadata.sequence,
            dropped_captures: metadata.dropped_captures,
            windows: metadata.windows,
            captured_sample: metadata.captured_sample,
            end_sample: metadata.end_sample,
            analysis_epoch: metadata.analysis_epoch,
            history_start_sample: metadata.history_start_sample,
            smoothing_ms: metadata.smoothing_ms,
            source_underrun: metadata.source_underrun,
            bins,
        };
        self.controller
            .complete_session_command(
                prepared,
                protocol::SessionCommandReply::SpectrumStreamed(reply),
                &mut self.response_scratch,
            )
            .map_err(map_command_process_error)
    }

    /// `SPECTRUM_STREAM` (#1595 D3-D6) on the monitor's service and analysis, on this control
    /// thread: start, select, reset and stop answer the stream's metadata after the operation; a
    /// read answers its status (`inactive`, `warming`, `pending`, `gap`, `failed`, `stopped` or
    /// `ready`, none a refusal), and a ready read analyzes the service's held window with the
    /// stream's smoothing (`SpectrumAnalysis::analyze_stream_into`) into the monitor's one
    /// result. Every refusal is the service's error with its status (D6).
    ///
    /// **Acked-batch question for a stream: can a reply acknowledge a window it did not deliver,
    /// or lose a drop count? No.** R6 admits only entries whose stream read fits the session's
    /// frame limit, at boot and at configure. In depth, a read also measures the frame a ready
    /// reply of the selected entry takes (D5) before it reads and refuses `LIMIT_EXCEEDED`
    /// (`spectrum.frame`) when the limit cannot carry it: such a read pops no window and consumes
    /// no `gap` or `failed` status. A `gap` reply carries
    /// `dropped_captures`, the windows the bounded queue dropped in the epoch. As for a capture,
    /// the output is reserved for the frame limit before the session runs and the reply is
    /// recorded for replay in the call that writes it. The one consume-then-fail path is an
    /// analyzer refusal of a popped window (`WindowInvalid`, `UNAVAILABLE`), as for a capture.
    fn spectrum_stream(
        &mut self,
        request: &protocol::SpectrumStreamRequest,
    ) -> Result<StreamOutcome, SpectrumRefusal> {
        let limit = self.limits.maximum_control_frame_bytes;
        let Some(parts) = self.spectrum.read_parts() else {
            return Err(spectrum_service_refusal(
                host_core::SpectrumServiceError::Unprepared,
            ));
        };
        let crate::spectrum::ReadParts {
            service,
            analysis,
            one_shot,
        } = parts;
        match request {
            protocol::SpectrumStreamRequest::Start { smoothing_ms } => {
                let smoothing = host_core::spectrum_smoothing(*smoothing_ms)
                    .map_err(spectrum_service_refusal)?;
                service
                    .start_stream(smoothing, Some(&mut *analysis))
                    .map_err(spectrum_service_refusal)?;
            }
            protocol::SpectrumStreamRequest::Select {
                target_kind,
                target_id,
                channels,
                smoothing_ms,
            } => {
                let target =
                    host_core::spectrum_target_ref(u32::from(target_kind.code()), target_id)
                        .map_err(|error| {
                            SpectrumRefusal::reply(protocol::StatusCode::InvalidField, error.code())
                        })?;
                let smoothing = smoothing_ms
                    .map(host_core::spectrum_smoothing)
                    .transpose()
                    .map_err(spectrum_service_refusal)?;
                service
                    .select_target(
                        target,
                        spectrum_channels(*channels),
                        smoothing,
                        Some(&mut *analysis),
                    )
                    .map_err(spectrum_service_refusal)?;
            }
            protocol::SpectrumStreamRequest::Read => {
                // D5, defence in depth: R6 (at boot and at configure) already admits only entries
                // whose stream read fits, so this refusal guards the rule rather than a known path;
                // a read that did not fit must still pop no window and consume no status.
                if let (Some(target), Some(channels)) = (service.target(), service.channels())
                    && !stream_reply_fits(
                        protocol_channels(channels),
                        spectrum_target_id(target).len(),
                        limit,
                    )
                {
                    return Err(SpectrumRefusal::reply(
                        protocol::StatusCode::LimitExceeded,
                        "spectrum.frame",
                    ));
                }
                service
                    .read_stream(Some(&mut *analysis))
                    .map_err(spectrum_service_refusal)?;
                if let Some(window) = service.stream_window() {
                    let mut metadata = *service.stream_metadata();
                    analysis
                        .analyze_stream_into(window, &mut one_shot.result, &mut metadata)
                        .map_err(spectrum_service_refusal)?;
                    return Ok(StreamOutcome {
                        metadata,
                        ready: true,
                    });
                }
            }
            protocol::SpectrumStreamRequest::Reset => {
                analysis.reset_stream().map_err(spectrum_service_refusal)?;
            }
            protocol::SpectrumStreamRequest::Stop => service.stop_stream(Some(&mut *analysis)),
        }
        // The service's metadata with the analysis's facts: a reset changes only the analysis.
        // Only a read delivers a window, so a reply to anything else is never `ready`: after a
        // ready read, a reset discards the window at hand, and the stream runs with none queued
        // for this reader (`pending`) until the next read.
        let mut metadata = *service.stream_metadata();
        metadata.analysis_epoch = analysis.analysis_epoch();
        metadata.history_start_sample = analysis.history_start_sample();
        if metadata.status == host_core::SpectrumStreamStatus::Ready {
            metadata.status = host_core::SpectrumStreamStatus::Pending;
        }
        Ok(StreamOutcome {
            metadata,
            ready: false,
        })
    }

    /// `SPECTRUM_CAPTURE` (#1595 D4-D6) on the monitor's service, on this control thread: arm,
    /// cancel and select answer `done`; a read answers `pending` or analyzes the window it pops
    /// into the monitor's one result (`SpectrumAnalysis::analyze_into`). Every refusal is the
    /// service's error with its status (D6) and changes nothing.
    ///
    /// **Acked-batch question for a read: can a reply ever acknowledge a window it did not
    /// return? No.** Before it pops, a read measures the frame a ready reply of its shape takes
    /// (D5) and refuses `LIMIT_EXCEEDED` (`spectrum.frame`) when the session's frame limit cannot
    /// carry it; the service's own refusals (`Channels`, `NotArmed`, `Exhausted`) also come before
    /// the pop. The controller reserved the caller's output for a frame of that limit before the
    /// session ran, so a measured reply always fits, and the reply is recorded for replay in the
    /// same call that writes it: a replay returns those bytes and pops nothing.
    fn spectrum_capture(
        &mut self,
        request: &protocol::SpectrumCaptureRequest,
    ) -> Result<CaptureOutcome, SpectrumRefusal> {
        let limit = self.limits.maximum_control_frame_bytes;
        let Some(parts) = self.spectrum.read_parts() else {
            return Err(spectrum_service_refusal(
                host_core::SpectrumServiceError::Unprepared,
            ));
        };
        let crate::spectrum::ReadParts {
            service,
            analysis,
            one_shot,
        } = parts;
        match request {
            protocol::SpectrumCaptureRequest::Arm => service
                .arm()
                .map(|()| CaptureOutcome::Done)
                .map_err(spectrum_service_refusal),
            protocol::SpectrumCaptureRequest::Cancel => service
                .cancel()
                .map(|()| CaptureOutcome::Done)
                .map_err(spectrum_service_refusal),
            protocol::SpectrumCaptureRequest::Select {
                target_kind,
                target_id,
                channels,
            } => {
                let target =
                    host_core::spectrum_target_ref(u32::from(target_kind.code()), target_id)
                        .map_err(|error| {
                            SpectrumRefusal::reply(protocol::StatusCode::InvalidField, error.code())
                        })?;
                service
                    .select(target, spectrum_channels(*channels))
                    .map(|_| CaptureOutcome::Done)
                    .map_err(spectrum_service_refusal)
            }
            protocol::SpectrumCaptureRequest::Read {
                channels,
                include_window,
            } => {
                // D5: without a selection the read refuses `NotArmed` before it pops; with one,
                // the frame a ready reply of this shape takes must fit the session's limit.
                if let Some(target) = service.target()
                    && !reply_fits(
                        *channels,
                        *include_window,
                        spectrum_target_id(target).len(),
                        limit,
                    )
                {
                    return Err(SpectrumRefusal::reply(
                        protocol::StatusCode::LimitExceeded,
                        "spectrum.frame",
                    ));
                }
                let Some(snapshot) = service
                    .read(spectrum_channels(*channels))
                    .map_err(spectrum_service_refusal)?
                else {
                    return Ok(CaptureOutcome::Pending);
                };
                // The popped window goes into the slot the monitor made with its service: a read
                // allocates only its reply. An analyzer refusal here (`WindowInvalid`) has
                // consumed the window and ended the arm: the one exception to D5.
                one_shot.snapshot = snapshot;
                analysis
                    .analyze_into(&one_shot.snapshot, &mut one_shot.result)
                    .map_err(spectrum_service_refusal)?;
                Ok(CaptureOutcome::Ready {
                    include_window: *include_window,
                })
            }
        }
    }

    /// `SPECTRUM_CONFIGURE` (#1594 D4-D7). Every check runs before any write, in order: the
    /// request's own rules (#1591 D2's constructors, `INVALID_FIELD` with the
    /// `SpectrumRequestError` code); each target names a strip or an output of the committed
    /// model (`NOT_FOUND`, `spectrum.target.unknown`); no entry twice (`VALIDATION_FAILED`,
    /// `spectrum.duplicate`). Then:
    ///
    /// - an equal configuration changes nothing (`rebuild` false, the epochs unchanged);
    /// - a configuration with the same taps and another hop or budget publishes no plan: it is
    ///   stored and both epochs advance by one ([`crate::spectrum::SpectrumMonitor::retune`]);
    ///   a budget below what the captures retain is `LIMIT_EXCEEDED`
    ///   (`spectrum.budget.retained`);
    /// - any other change is a monitor rebuild ([`Rebuild::Monitor`]) at the committed revision.
    ///   A pending candidate is superseded first, as a transaction supersedes it (#1594 R2): its
    ///   edits are in the committed model the successor is compiled from.
    ///
    /// **Acked-batch question: can an ack ever precede a drop? No.** The command acknowledges no
    /// edit and changes neither the model nor the revision. A refusal changes nothing: a
    /// withdrawn candidate is published again whole, and a superseded candidate's fed rings move
    /// by the checked donation or the supersession is refused as backpressure.
    fn spectrum_configure(
        &mut self,
        request: &protocol::SpectrumConfigureRequest,
        plan: Option<&mut PlanState<A>>,
    ) -> Result<protocol::SpectrumConfigured, SpectrumRefusal> {
        use protocol::StatusCode;
        let configuration = checked_spectrum_configuration(
            request,
            self.controller.session().compiled().normalized_model(),
            self.limits.maximum_control_frame_bytes,
        )?;
        let revision = self.controller.session().revision().0;
        let reply = |monitor: &crate::spectrum::SpectrumMonitor, plan_epoch: Option<u64>| {
            protocol::SpectrumConfigured {
                revision,
                configured_epoch: monitor.configured_epoch(),
                adopted_epoch: monitor.adopted_epoch(),
                rebuild: plan_epoch.is_some(),
                plan_epoch,
            }
        };
        let current = self.spectrum.configuration();
        if configuration.as_ref() == current {
            return Ok(reply(&self.spectrum, None));
        }
        if let (Some(next), Some(current)) = (configuration.as_ref(), current)
            && same_taps(&next.preparation, &current.preparation)
        {
            let retained = next
                .preparation
                .retained_bytes()
                .map_err(|_| SpectrumRefusal::Error(CommandError::Internal))?;
            if spectrum_budget(&next.preparation) < retained {
                return Err(SpectrumRefusal::reply(
                    StatusCode::LimitExceeded,
                    "spectrum.budget.retained",
                ));
            }
            let hop = next.hop;
            let replaced = self.spectrum.retune(next.clone());
            let configured = self.spectrum.configured_epoch();
            // A pending candidate prepared with the replaced configuration holds the same taps: it
            // now carries the new one, with its hop.
            for provider in &mut self.pending_providers {
                if let crate::spectrum::EpochSpectrum::Candidate {
                    hop: candidate_hop,
                    epoch,
                    ..
                } = &mut provider.spectrum
                    && *epoch == replaced
                {
                    *epoch = configured;
                    *candidate_hop = hop;
                }
            }
            return Ok(reply(&self.spectrum, None));
        }
        let epoch = self.monitor_rebuild(configuration)?;
        // #1597: before the first block, a lent plan adopts the successor in place, and the
        // promotion installs its captures now. Not an ack before a drop: the candidate is adopted
        // and nothing is lost if the promotion below fails, which only an `Internal` invariant
        // fault can make it do; the reply is then not sent, and the next service step meets the
        // same fault.
        if let Some(plan) = plan
            && plan.adopt_before_first_block()
        {
            self.synchronize_plan_epochs()
                .map_err(SpectrumRefusal::Error)?;
        }
        Ok(reply(&self.spectrum, Some(epoch)))
    }

    /// The monitor rebuild of a `SPECTRUM_CONFIGURE` (#1594 D6): the committed model, prepared
    /// with `configuration` as a successor of the newest plan at the committed revision. Returns
    /// the epoch of the plan it published.
    fn monitor_rebuild(
        &mut self,
        configuration: Option<host_core::SpectrumConfiguration>,
    ) -> Result<u64, SpectrumRefusal> {
        if !self.shared.plan_alive.load(Ordering::Acquire) {
            return Err(SpectrumRefusal::Error(CommandError::Backpressure));
        }
        let revision = self.controller.session().revision().0;
        if revision > engine::realtime::GATE_REVISION_MAX {
            return Err(SpectrumRefusal::Error(CommandError::Internal));
        }
        let model = Box::new(self.controller.session().compiled().clone());
        // Nothing changes the overlay or the subscriptions: the successor renders under the ones
        // the newest plan holds, and every binding keeps its target.
        let monitor_change = self.monitor.after(&[], model.normalized_model());
        let observation_change =
            self.observation
                .after(model.normalized_model())
                .map_err(|_| {
                    SpectrumRefusal::Error(CommandError::CompileRejected(failure(
                        ResourceFault::Allocation,
                    )))
                })?;
        if !observation_change.ended.is_empty() {
            return Err(SpectrumRefusal::Error(CommandError::Internal));
        }
        // This assumes a withdrawal refuses only with `Internal` (`promote_pending_provider`),
        // which the arm below answers as an error, never as a `LimitExceeded` reply.
        let result = self.rebuild_superseding(
            Rebuild::Monitor {
                model,
                configuration,
            },
            revision,
            monitor_change,
            observation_change,
        );
        match result {
            Ok(_) => self
                .pending_providers
                .last()
                .map(|provider| provider.epoch)
                .ok_or(SpectrumRefusal::Error(CommandError::Internal)),
            // D4: a preparation or resource refusal is the command's typed answer.
            Err(CommandError::CompileRejected(failure)) => Err(SpectrumRefusal::Reply(
                protocol::StatusCode::LimitExceeded,
                first_diagnostic_code(&failure),
            )),
            Err(error) => Err(SpectrumRefusal::Error(error)),
        }
    }

    /// The warm-successor deadline step (#1358 D3): run once per control call while a `Primed`
    /// record is held. `None` when there is no record, its deadline has not passed
    /// ([`host_core::warm::prime_deadline_samples`] render samples past its `not_before`; a paused
    /// render clock never passes it), or the record is stale (not the newest epoch: it is dropped
    /// and nothing is withdrawn). Past the deadline it runs the transition fallback, whose one
    /// withdrawal answers it: render adopted W first (`Taken`, `Nothing`: the record drops and the
    /// revision completes `EXACT` through the watermark), the transition was published in W's
    /// place, or the re-preparation was refused and W was republished whole with its record, so
    /// the next call's step is past the deadline at once.
    ///
    /// **Acked-batch question (D4): can an ack ever precede a drop? No.** The step drops only a
    /// record; every candidate it withdraws is either the donor of a transition prepared from the
    /// committed model or republished whole, and it adds no queue. One step does at most one
    /// withdrawal, one re-preparation and one republication, and never waits on render.
    #[cfg_attr(
        not(feature = "test-support"),
        expect(
            dead_code,
            reason = "#1360 and #1361 wire the hosts' service steps to it"
        )
    )]
    pub(crate) fn check_prime_deadline(&mut self) -> Result<Option<TransitionStep>, CommandError> {
        let Some(record) = self.primed else {
            return Ok(None);
        };
        let model = self.controller.session().compiled().normalized_model();
        let (sample_rate_hz, quantum_frames) = (model.sample_rate_hz, model.quantum_frames);
        match host_core::warm::check_prime_deadline(
            &self.publisher,
            self.newest_plan_epoch(),
            &record,
            sample_rate_hz,
            quantum_frames,
        ) {
            host_core::warm::DeadlineStep::Pending => Ok(None),
            host_core::warm::DeadlineStep::Stale => {
                self.primed = None;
                Ok(None)
            }
            host_core::warm::DeadlineStep::Due => self.fall_back_pending_warm().map(Some),
        }
    }

    /// Withdraws the pending candidate, rebuilds, and republishes the candidate when the rebuild
    /// does not take it: the one supersession path of a transaction, a declared discontinuity and
    /// a monitor change.
    ///
    /// #1310 D1: a pending candidate is taken back before anything is prepared, so the replacement
    /// knows the plan render runs and the candidate's host-fed rings can move while the control
    /// thread owns them. The preparation reads the withdrawn candidate (its carried base, the
    /// donation check), so the withdrawal cannot wait until after it (#2010). #1310 D4: a refusal
    /// leaves the withdrawn candidate here, whole; it is published again with its own epoch,
    /// credit, revision gate and words. Nothing was published since it was withdrawn, so this
    /// cannot fail.
    ///
    /// #2010: a panic inside the rebuild before its commit republishes the candidate too, then
    /// resumes the panic, so a host that catches it (the C ABI answers `RESULT_INTERNAL`) keeps
    /// the candidate whose revisions it acked. Under `panic = "abort"` nothing unwinds and this
    /// changes nothing. #2044: every step of the rebuild that can panic runs before the protocol
    /// commit (`prepared.commit`), and the candidate is taken right after it, before any write to
    /// the controller's own fields; only `take_displaced`, which cannot panic, runs between them.
    /// So `withdrawn` is still `Some` exactly when the protocol commit did not happen, and a
    /// republished candidate always meets the controller it was published under. A panic after
    /// the take, which has taken the candidate, is not covered.
    fn rebuild_superseding(
        &mut self,
        kind: Rebuild,
        revision: u64,
        monitor_change: controls::MonitorChange,
        observation_change: controls::ObservationChange,
    ) -> Result<usize, CommandError> {
        let mut withdrawn = self.withdraw_pending_candidate()?;
        self.rebuild_withdrawn(
            kind,
            revision,
            monitor_change,
            observation_change,
            &mut withdrawn,
        )
    }

    /// [`Self::rebuild_superseding`] after its withdrawal: rebuilds with `withdrawn` and
    /// republishes a candidate the rebuild did not take, on a refusal and on a panic before the
    /// commit, which it then resumes (#2010). The transition fallback (#1397 D2) runs it on the
    /// candidate it withdrew itself; `withdrawn` is `None` on return.
    fn rebuild_withdrawn(
        &mut self,
        kind: Rebuild,
        revision: u64,
        monitor_change: controls::MonitorChange,
        observation_change: controls::ObservationChange,
        withdrawn: &mut Option<engine::realtime::UnadoptedCandidate>,
    ) -> Result<usize, CommandError> {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.rebuild(
                kind,
                revision,
                monitor_change,
                observation_change,
                withdrawn,
            )
        }));
        if let Some(candidate) = withdrawn.take() {
            // #2044: a panic after the successor's `publish_primed` wrote its prime flags (the
            // protocol commit's own steps) leaves the flags of the rings the candidate carries
            // as the successor set them. A `Primed` candidate is judged by them, so every
            // republished one gets its own flags back first (idempotent: a refusal before
            // `publish_primed` writes the same values again).
            let epoch = candidate.epoch().0;
            if self.primed.is_some_and(|record| record.epoch == epoch) {
                self.restore_prime_flags(epoch);
            }
            self.publisher.republish(candidate);
            #[cfg(feature = "test-support")]
            update_test_owners(|owners| owners.candidate_republished += 1);
        }
        result.unwrap_or_else(|payload| std::panic::resume_unwind(payload))
    }

    /// Sets the prime flags of the newest pending candidate's carry again, as its own
    /// `publish_primed` set them (#1403 D3, #2044), before it is republished.
    ///
    /// Its publication moved every producer it held vacant in from the epoch it succeeds, which
    /// then holds those entries vacant. So the candidate carries exactly the rings whose
    /// producers it holds and whose IDs that epoch holds vacant (a redeclared source's entry
    /// there keeps its own producer); every producer that epoch still holds is of a ring the
    /// candidate does not carry. No allocation.
    fn restore_prime_flags(&self, epoch: u64) {
        let len = self.pending_providers.len();
        let Some(candidate) = self.pending_providers.last() else {
            return;
        };
        debug_assert_eq!(
            candidate.epoch, epoch,
            "the republished candidate is the newest pending epoch"
        );
        let holder = match len {
            0 | 1 => &self.providers,
            _ => &self.pending_providers[len - 2],
        };
        holder
            .sources
            .set_prime_flags(|id| candidate.sources.holds_vacant(id));
        candidate
            .sources
            .set_prime_flags(|id| holder.sources.holds_vacant(id));
    }

    /// The transition fallback across the pending `Primed` candidate W, the donor (#1397 D2): the
    /// step the deadline step (`check_prime_deadline`, #1358 D3) runs once W is not ready by its
    /// deadline.
    ///
    /// It withdraws W. A `Taken` or `Nothing` withdrawal means render adopted W, and nothing is
    /// done. A withdrawn W is the donor of [`Rebuild::Transition`]: the committed model is
    /// re-prepared against the plan W succeeds (#1344 D2), with the duck set as its forced
    /// restarts, W's added rings are donated only after that preparation succeeds, W drops on this
    /// thread, and the transition is published with `TRANSITION_FALLBACK`. The cross-plan
    /// admission is not run again: W's submit admitted this re-preparation with the reprepare
    /// peak (#1398 D4, #1403).
    ///
    /// **A refused re-preparation is a defect (D2 step 3).** W is republished whole, its record is
    /// set again unchanged and `transition_reprepare_refusals` rises (counted, not asserted: root
    /// ruling on #1358). W's revision stays pending and the next step retries. **Acked-batch
    /// question (D4): can an ack ever precede a drop? No:** W drops only after its replacement is prepared from the committed model, which holds
    /// every acked edit, and its rings move by a donation checked before anything is published.
    #[cfg_attr(
        not(feature = "test-support"),
        expect(
            dead_code,
            reason = "its caller, the deadline step, is not yet wired (#1360)"
        )
    )]
    fn fall_back_pending_warm(&mut self) -> Result<TransitionStep, CommandError> {
        let Some(record) = self.primed else {
            return Ok(TransitionStep::NotPending);
        };
        if !self.shared.plan_alive.load(Ordering::Acquire) {
            return Err(CommandError::Backpressure);
        }
        let revision = self.controller.session().revision().0;
        let model = Box::new(self.controller.session().compiled().clone());
        // The transition renders under the overlay and the subscriptions the donor holds.
        let monitor_change = self.monitor.after(&[], model.normalized_model());
        let observation_change = self
            .observation
            .after(model.normalized_model())
            .map_err(|_| CommandError::CompileRejected(failure(ResourceFault::Allocation)))?;
        if !observation_change.ended.is_empty() {
            return Err(CommandError::Internal);
        }
        let mut withdrawn = self.withdraw_pending_candidate()?;
        match withdrawn.as_ref() {
            None => return Ok(TransitionStep::NotPending),
            Some(candidate) if candidate.epoch().0 != record.epoch => {
                // A record of another epoch is stale and was dropped with the withdrawal; the
                // deadline step's epoch check makes this unreachable (#1358 D3).
                if let Some(candidate) = withdrawn.take() {
                    self.publisher.republish(candidate);
                }
                debug_assert!(
                    false,
                    "#1358 D3: a withdrawal of another epoch than the record's"
                );
                return Ok(TransitionStep::NotPending);
            }
            Some(_) => {}
        }
        #[cfg(feature = "test-support")]
        let injected = TEST_REFUSE_REPREPARE.with(|state| state.replace(false));
        // A refusal republishes W whole, as a panic before the commit does (#2010).
        let result = self.rebuild_withdrawn(
            Rebuild::Transition {
                model,
                #[cfg(feature = "test-support")]
                injected_refusal: injected,
            },
            revision,
            monitor_change,
            observation_change,
            &mut withdrawn,
        );
        match result {
            Ok(_) => Ok(TransitionStep::Published),
            Err(_) => {
                // W's record stays: the withdrawal of the epoch it names keeps it.
                debug_assert_eq!(self.primed, Some(record));
                // Root ruling (#1358): a refusal is counted, never asserted, since a real one
                // reaches this path once the deadline step is wired (#1360).
                self.transition_reprepare_refusals =
                    self.transition_reprepare_refusals.saturating_add(1);
                Ok(TransitionStep::Refused)
            }
        }
    }

    /// Takes back the newest pending candidate if render has not adopted it (#1310 D1). `None`
    /// while nothing is pending, or when render adopted it first (`Taken`), which is the path a
    /// rebuild on the newest plan has always taken.
    ///
    /// **Render took the newest candidate (batch 15a review fold, M1).** `Taken`, or an empty
    /// mailbox while a candidate is still pending, means render has claimed the newest one; it
    /// adopts candidates in publication order, so it has adopted (or is adopting) every pending
    /// one. Each is promoted here, up to the newest, before the rebuild runs: the promotion would
    /// otherwise wait for the displaced plan's retirement or the atomic, which lag render (a render
    /// preempted between its claim and its retirement), and a second `Taken` in that window would
    /// leave two adopted candidates pending beside a third (#1398 D3's accounting fault). So the
    /// rebuild always finds at most one pending candidate, the one it may supersede. The epochs
    /// it parks keep their resource rows (`ProviderEpoch::parked`), so the rebuild's admission
    /// still charges a plan render displaced but has not yet retired (batch 15a review fold 2,
    /// M-1).
    fn withdraw_pending_candidate(
        &mut self,
    ) -> Result<Option<engine::realtime::UnadoptedCandidate>, CommandError> {
        let Some(newest) = self.pending_providers.last().map(|provider| provider.epoch) else {
            return Ok(None);
        };
        match self.publisher.withdraw() {
            engine::realtime::Withdrawal::Withdrawn(candidate) => {
                #[cfg(feature = "test-support")]
                update_test_owners(|owners| owners.candidate_withdrawn += 1);
                // #1403 D4: the record goes with its candidate to the path that withdrew it; a
                // record of another epoch is stale.
                if self
                    .primed
                    .is_some_and(|record| record.epoch != candidate.epoch().0)
                {
                    self.primed = None;
                }
                Ok(Some(candidate))
            }
            engine::realtime::Withdrawal::Taken => {
                #[cfg(feature = "test-support")]
                update_test_owners(|owners| owners.candidate_taken += 1);
                self.primed = None;
                self.promote_pending_provider(newest)?;
                Ok(None)
            }
            // An earlier withdrawal found it taken: render runs it.
            engine::realtime::Withdrawal::Nothing => {
                self.primed = None;
                self.promote_pending_provider(newest)?;
                Ok(None)
            }
        }
    }

    /// The rebuild of a structural transaction: prepare, admit, reserve and commit a successor plan
    /// and publish it (#1273, #1398, #1310).
    ///
    /// **Supersession (#1310).** With `withdrawn` holding a candidate A taken back unadopted, the
    /// successor B is prepared against the plan A succeeds, P0 (D2): A's carried base, and P0's
    /// kept model, monitor overlay and edit ramps merged with B's own (D3, root ruling B15-Q3). Every
    /// check runs while A is withdrawn, before the commit (D4): preparation, admission,
    /// reservation, the source donation from A and the room for A's queued observation records
    /// on B (root ruling B15-Q4), then the protocol's commit predicate. A refusal returns with
    /// `withdrawn` untouched, and the caller republishes A. After the commit, infallibly and in
    /// order (D5): the donation and the producer moves, B's `superseded` word and revision, B's
    /// publication, the return to P0's epoch of every producer A still holds that P0 holds
    /// vacant, and A's drop.
    ///
    /// **Acked-batch question (D8): can an ack ever precede a drop? No.** A's revisions stay
    /// committed: their edits are in the model B is compiled from, A's host-fed rings move into
    /// B by a donation checked before the commit, the producers of sources P0 still renders go
    /// back to P0 before A drops, and every observation record queued on A is queued on B before
    /// B is published. B is acked only after every fallible step; A's revisions complete as
    /// `superseded` when B is adopted (D6).
    ///
    /// **A declared discontinuity (#1323).** With [`Rebuild::Discontinuity`] the successor is the
    /// committed model prepared at a discontinuity: no floors, only its source consumers carried,
    /// published [`PlanAdoption::Next`]. Nothing is committed: the model and the revision do not
    /// change, and the successor carries the committed revision. A successor that supersedes a
    /// discontinuity successor unadopted is prepared at the same discontinuity: render never
    /// played the withdrawn plan, so the pair is the one transaction "declare, then edit" against
    /// the running plan.
    #[allow(clippy::too_many_lines)]
    fn rebuild(
        &mut self,
        kind: Rebuild,
        revision: u64,
        monitor_change: controls::MonitorChange,
        observation_change: controls::ObservationChange,
        withdrawn: &mut Option<engine::realtime::UnadoptedCandidate>,
    ) -> Result<usize, CommandError> {
        let superseding = withdrawn.is_some();
        if let Some(candidate) = withdrawn.as_ref()
            && self.pending_providers.last().map(|newest| newest.epoch) != Some(candidate.epoch().0)
        {
            // The mailbox's last publication is the newest pending candidate by construction.
            return Err(CommandError::Internal);
        }
        let declared = matches!(kind, Rebuild::Discontinuity(_));
        // #1323: a successor of a withdrawn discontinuity successor is prepared at it too.
        let discontinuity = declared
            || (superseding
                && self
                    .pending_providers
                    .last()
                    .is_some_and(|newest| newest.discontinuity));
        let own_ramps = match &kind {
            Rebuild::Transaction(prepared) => prepared.get().prospective_session().edit_ramps(),
            Rebuild::Discontinuity(_) | Rebuild::Monitor { .. } | Rebuild::Transition { .. } => {
                &NO_EDIT_RAMPS
            }
        };
        // Whether B writes a monitor row: then its overlay change's ramp is the latest.
        let writes_monitor = match &kind {
            Rebuild::Transaction(prepared) => !prepared.get().monitor_edits().is_empty(),
            Rebuild::Discontinuity(_) | Rebuild::Monitor { .. } | Rebuild::Transition { .. } => {
                false
            }
        };
        // #1594 D6: a monitor rebuild keeps the committed revision, which may be 0. Against the
        // running plan its model is the base's, so nothing retargets; across a withdrawn
        // candidate at a newer revision the base is the older plan's, whose retargets the
        // preparation stamps with that revision as it does for a transaction.
        let monitor = matches!(kind, Rebuild::Monitor { .. });
        // #1397 D2: the transition fallback across a donor.
        let fallback = matches!(kind, Rebuild::Transition { .. });
        let unchanged_revision = monitor && (!superseding || revision == 0);
        // The ramp lengths the epoch B succeeds keeps from now on: B's own, or, across a
        // withdrawn candidate, the kept base's with B's laid over them (B wins on a shared row).
        let mut kept_ramps = if superseding {
            let kept = self
                .kept_holder()
                .and_then(|holder| holder.kept.as_ref())
                .ok_or(CommandError::Internal)?;
            merged_ramps(&kept.edit_ramps, own_ramps)
        } else {
            own_ramps.clone()
        };
        let prospective = match &kind {
            Rebuild::Transaction(prepared) => prepared
                .get()
                .prospective_session()
                .compiled()
                .ok_or(CommandError::Internal)?,
            Rebuild::Discontinuity(model)
            | Rebuild::Monitor { model, .. }
            | Rebuild::Transition { model, .. } => &**model,
        };
        // #1696 root ruling: a kept base holds only the rows of owners B's model has, and no
        // rewritten targets, so the ramp row of B's model bounds it.
        kept_ramps.keep_within(prospective.normalized_model());
        // #1325 D6: the duck overlay of the running plan's cells while phase 1 runs, empty
        // outside it. The successor's base composes it, so a strip or source this transaction
        // restores carries from its ducked state and ramps back in from `S`.
        let duck = if superseding {
            &self
                .kept_holder()
                .and_then(|holder| holder.kept.as_ref())
                .ok_or(CommandError::Internal)?
                .duck
        } else {
            &NO_DUCK
        };
        // #1698 (amending #1310 D2 and #1344 D1): across a withdrawn candidate A, P0's own
        // inventory, with only the source rows A carries from P0, and P0's kept base. An owner B
        // keeps unchanged from P0 carries from P0 as a direct P0 -> B swap carries it; a source A
        // added or redeclared restarted the host's feed against A, so B starts it fresh.
        let withdrawn_base = match withdrawn.as_ref() {
            Some(candidate) => {
                let holder = self.kept_holder().ok_or(CommandError::Internal)?;
                let donor = self
                    .pending_providers
                    .last()
                    .ok_or(CommandError::Internal)?;
                Some(holder.inventory.across_withdrawn_in_phase_one(
                    &donor.inventory,
                    candidate.plan(),
                    duck,
                ))
            }
            None => None,
        };
        let base = if let Some(inventory) = withdrawn_base.as_ref() {
            let kept = self
                .kept_holder()
                .and_then(|holder| holder.kept.as_ref())
                .ok_or(CommandError::Internal)?;
            SuccessorBase {
                edit_ramps: &kept_ramps,
                monitor: host_core::MonitorBase {
                    committed: &kept.monitor,
                    next: &monitor_change.overlay,
                    ramp: if writes_monitor {
                        monitor_change.ramp
                    } else {
                        kept.monitor_ramp.unwrap_or(monitor_change.ramp)
                    },
                },
                observation: &observation_change.bindings,
                discontinuity,
                unchanged_revision,
                duck,
                ..SuccessorBase::new(inventory, kept.model.normalized_model(), revision)
            }
        } else {
            // Issue #1273 D1: the candidate succeeds the newest plan, so every source the
            // transaction leaves unchanged is prepared to carry that plan's ring.
            SuccessorBase {
                // #1394 D6: a carry retarget is the record of the transaction's own edit, so it
                // takes that edit's ramp length.
                edit_ramps: own_ramps,
                monitor: host_core::MonitorBase {
                    committed: &self.monitor,
                    next: &monitor_change.overlay,
                    ramp: monitor_change.ramp,
                },
                observation: &observation_change.bindings,
                discontinuity,
                unchanged_revision,
                ..SuccessorBase::new(
                    &self.newest_providers().inventory,
                    self.controller.session().compiled().normalized_model(),
                    revision,
                )
            }
        };
        // #1593 D4: the successor keeps every spectrum entry whose target the prospective model
        // still has. It is stored only beside the candidate push, after the protocol commit.
        // #1594 D6: a monitor rebuild is prepared with the request's configuration instead.
        let spectrum_configuration = match &kind {
            Rebuild::Monitor { configuration, .. } => configuration.clone(),
            _ => self
                .spectrum
                .successor_configuration(prospective.normalized_model()),
        };
        let prepare = |base: SuccessorBase<'_>| {
            prepare_runtime::<A>(
                &*self.preparer,
                prospective,
                self.limits,
                spectrum_configuration
                    .as_ref()
                    .map(|configuration| &configuration.preparation),
                Some(base),
                &self.live_values_superseded,
                self.observation_slots_high_water,
                self.capture_owners_high_water,
            )
            .map_err(spectrum_invariant)
        };
        // Issue #2194: the candidate's projected effect bytes beside every plan and model held,
        // before any of it is prepared. The transition across a donor is not admitted again (the
        // donor's submit admitted it, below).
        if !fallback {
            self.admit_prospective_projection(prospective)?;
        }
        let mut prepared_runtime = prepare(base)?;
        // #1325 D7: after preparation, the pure transition step against the plan the successor
        // displaces. A declared discontinuity need not continue the running plan's output
        // (#1323 D2): it ducks nothing and is adopted at the next block. Everything phase 1 needs
        // is built here, before anything is reserved or committed.
        let plan_transition = |restarted: &[Box<str>]| {
            host_core::plan_strip_transition(&base, duck, prospective.normalized_model(), restarted)
                .map_err(|_| CommandError::Internal)
        };
        let mut transition = if declared {
            None
        } else {
            // #1324 D4: the strips the successor restarts duck-swap.
            Some(plan_transition(
                prepared_runtime.inventory.restarted_strips(),
            )?)
        };
        let quantum = prospective.normalized_model().quantum_frames;
        // #1403 D2, #1397 D1: with the session's warm bounds, the edit is classified against the
        // plan it displaces, from the ordinary successor's inventory (#1354a ruling 1). While a
        // phase 1 of an earlier transaction still ducks a strip on that plan, a `Warm` or
        // `Unavailable` answer takes the transition fallback (below). Otherwise a `Warm` answer
        // is prepared with its lead (#1354 D4), so the lead's
        // whole restarts join the successor's `restarted_strips()`, and the transition is planned
        // again over that set: the duck set is the strips the edit removes plus every strip the
        // warm successor restarts (#1324 D4), and each restarted strip is restarted whole and
        // armed by the preparation (#1324 D2-D3). With `lead_samples > 0` it is admitted with the
        // reprepare peak and published `Primed`, no earlier than the duck's `S` when it ducks a
        // strip. With `lead_samples == 0` there is no lead and no prime: it is an ordinary
        // duck-swap, admitted at the single peak and published as one.
        let mut warm_lead_blocks = None;
        // #1397 D2: whether this successor is the transition fallback: at submit (`warm_lead`
        // answered `Unavailable`) or across a withdrawn donor (`Rebuild::Transition`).
        let mut transition_fallback = false;
        let mut replanned = false;
        let fall_back = |ordinary: PreparedRuntime| {
            host_core::fall_back_to_transition(
                base,
                prospective.normalized_model(),
                ordinary,
                |runtime: &PreparedRuntime| &runtime.inventory,
                prepare,
            )
        };
        if fallback {
            prepared_runtime = fall_back(prepared_runtime)?;
            (transition_fallback, replanned) = (true, true);
        } else if let Some(config) = self.warm
            && transition.is_some()
        {
            let decision = host_core::warm_lead(
                prospective,
                &prepare_caps(self.limits),
                base,
                &prepared_runtime.inventory,
                &config,
            )
            .map_err(|diagnostics| spectrum_invariant(prepare_failure(diagnostics)))?;
            #[cfg(feature = "test-support")]
            let decision = match TEST_NEXT_ZERO_LEAD.with(|state| state.borrow_mut().take()) {
                Some(whole) => host_core::WarmDecision::Warm(
                    host_core::WarmLead::test_only_zero_lead(&prepared_runtime.inventory, &whole),
                ),
                None => decision,
            };
            // #1397a attempt 2 (root ruling): while a phase 1 that ducks a strip is pending, a
            // growth is not prepared warm (its successor would wait for the inherited `S` with
            // carried nodes a lead ahead), and an ordinary successor would step a carried strip
            // whose line changes length: it takes the transition, whose duck joins the overlay.
            match decision {
                host_core::WarmDecision::Warm(lead) if duck.strips().is_empty() => {
                    let lead_blocks = u32::try_from(lead.lead_samples / u64::from(quantum))
                        .map_err(|_| CommandError::Internal)?;
                    drop(prepared_runtime);
                    prepared_runtime = prepare(SuccessorBase {
                        warm: Some(&lead),
                        ..base
                    })?;
                    replanned = true;
                    if lead.lead_samples > 0 {
                        warm_lead_blocks = Some(lead_blocks);
                    }
                }
                // #1397 D2 at submit, with no donor: a failure refuses the edit before its commit.
                host_core::WarmDecision::Warm(_) | host_core::WarmDecision::Unavailable(_) => {
                    prepared_runtime = fall_back(prepared_runtime)?;
                    (transition_fallback, replanned) = (true, true);
                }
                host_core::WarmDecision::Ordinary => {}
            }
        }
        // The duck set is the strips the edit removes plus every strip the successor restarts:
        // the warm successor's (#1397 D1), or the transition's, its forced restarts (D2 step 4).
        if replanned {
            transition = Some(plan_transition(
                prepared_runtime.inventory.restarted_strips(),
            )?);
        }
        // #1397 deliverable 3: the injected refusal of a transition re-preparation, after it.
        #[cfg(feature = "test-support")]
        if matches!(
            kind,
            Rebuild::Transition {
                injected_refusal: true,
                ..
            }
        ) {
            drop(prepared_runtime);
            drop(kind);
            return Err(CommandError::Backpressure);
        }
        let next_duck = transition
            .as_ref()
            .map(|transition| duck.joined(transition));
        let PreparedRuntime {
            sources,
            strips,
            effects,
            meters,
            meter_tables,
            meter_frames,
            observations,
            observation_strips,
            carried_taps,
            observation_index,
            plan: candidate_plan,
            resources,
            carried,
            inventory,
            control_catalog: candidate_catalog,
            capi: prospective_capi,
            revision_gate,
            // #1593 D5, D7: held by the candidate until its promotion pairs them; until then the
            // service reads the running plan's captures.
            spectrum: candidate_captures,
        } = prepared_runtime;
        // #1652 U2a fold 2 (MAJOR-A; #1713 deletes it): read before the inventory moves into the
        // candidate's provider epoch.
        let follow_into_six_word = inventory.carries_a_following_route_into_a_six_word_route();
        // B19-R7/B19-R8 (#1718 deletes them): the same, for any carried following route that
        // stays following, and for one that loses a curve (its landing joins the window).
        let follow_into_following = inventory.carries_a_following_route_into_a_following_route();
        let follow_loses_a_curve = inventory.carries_a_following_route_that_loses_a_curve();
        let follow_retarget_ramp = inventory.following_route_retarget_ramp();
        let mut candidate_provider = ProviderEpoch::candidate(
            sources,
            inventory,
            carried,
            strips,
            effects,
            meters,
            prospective_capi,
            revision_gate,
        );
        candidate_provider.discontinuity = discontinuity;
        candidate_provider.observations = observations;
        candidate_provider.observation_strips = observation_strips;
        candidate_provider.carried_taps = carried_taps;
        candidate_provider.observation_index = observation_index;
        candidate_provider.meter_tables = meter_tables;
        candidate_provider = candidate_provider.with_meter_frames(meter_frames);
        // The configuration epoch is set beside the push, after the commit.
        candidate_provider.spectrum = crate::spectrum::EpochSpectrum::Candidate {
            captures: candidate_captures,
            hop: spectrum_configuration
                .as_ref()
                .and_then(|configuration| configuration.hop),
            epoch: 0,
        };
        let candidate_plan = ObservedCandidatePlan::new(candidate_plan);
        #[cfg(feature = "test-support")]
        {
            if self.take_test_fault(TestStructuralFaultPhase::AfterRuntimePrepare) {
                drop(candidate_plan);
                drop(candidate_provider);
                drop(kind);
                return Err(CommandError::Backpressure);
            }
            if self.take_test_fault(TestStructuralFaultPhase::PanicAfterWithdrawal) {
                panic!("test fault: a rebuild panics after the withdrawal");
            }
        }
        // #1398 D4: the candidate beside every plan the control plane holds unreclaimed, a
        // withdrawn candidate included, and every model it holds, the kept ones included (#1310
        // D3), checked before anything is reserved or committed.
        let held = self.held_epochs()?;
        let models = self.held_models()?;
        let limits = self.limits;
        // #1403 D2: a warm candidate is admitted with the reprepare peak too, beside the held
        // plans that are not candidates (every parked epoch and the running plan).
        let peak = |running: usize| {
            if warm_lead_blocks.is_some() {
                AdmissionPeak::WithReprepare { running }
            } else {
                AdmissionPeak::Single
            }
        };
        let admit = |held: &[LiveEpochResources],
                     models: &[session::ResourceEstimate],
                     admission: AdmissionPeak| {
            validate_replacement_peak(
                held,
                ProspectivePlan {
                    resources,
                    carried,
                    capi: prospective_capi,
                },
                compiled_model_admission_over(models, prospective.resource_estimate())?,
                limits,
                admission,
            )
        };
        let running = held
            .as_slice()
            .len()
            .checked_sub(self.pending_providers.len())
            .ok_or(CommandError::Internal)?;
        // #1397 D2: the transition across a donor is not admitted again: the donor's submit
        // admitted this re-preparation with the reprepare peak, from the same committed model.
        if !fallback && let Err(failure) = admit(held.as_slice(), models.as_slice(), peak(running))
        {
            // Batch 15a review fold, item 2: a refusal is permanent only if the edit cannot fit
            // even beside the newest plan alone and the committed model, which is all a retry
            // meets once render has adopted every pending candidate (the plans before it are
            // then reclaimed and their kept models dropped). Otherwise the peak is the pending
            // plans', which a render frees, and the refusal is retryable backpressure. The
            // second check charges the candidate's rows as prepared: a ring it takes from a
            // withdrawn candidate by donation is counted in both, so it errs toward the
            // permanent refusal by at most those rings' bytes, never toward an admission.
            let held = held.as_slice();
            return Err(
                match held.split_last().filter(|(_, older)| !older.is_empty()) {
                    Some((newest, _))
                        if admit(
                            core::slice::from_ref(newest),
                            &models.as_slice()[..1],
                            peak(1),
                        )
                        .is_ok() =>
                    {
                        CommandError::Backpressure
                    }
                    _ => CommandError::CompileRejected(failure),
                },
            );
        }
        #[cfg(feature = "test-support")]
        if self.take_test_fault(TestStructuralFaultPhase::AfterAdmission) {
            drop(candidate_plan);
            drop(candidate_provider);
            drop(kind);
            return Err(CommandError::Backpressure);
        }
        // #1310 D1: B is published with the adoption of its own classification, never A's. A
        // discontinuity successor is always `Next` (#1323 D2): no lead, readiness, deadline or
        // transition.
        #[cfg(feature = "test-support")]
        let adoption = if declared {
            PlanAdoption::Next
        } else {
            TEST_NEXT_ADOPTION
                .with(core::cell::Cell::take)
                .unwrap_or(PlanAdoption::Next)
        };
        #[cfg(not(feature = "test-support"))]
        let adoption = PlanAdoption::Next;
        // #1652 U2a fold 2 (MAJOR-A; #1713 deletes this): a successor whose send join carries a
        // following route into a six-word route (a route whose source strip automates a mute
        // lane in the committed plan and automates none in the successor: the rebuild removes
        // that strip's last mute entry, or retargets it) may re-time a carried column's move, which
        // render cannot report: one ramp of the six-word route takes the session mute length for
        // a join record, and folds two columns' moves into one. Such a rebuild completes as the
        // transition fallback, whether or not a column moves at the swap: conservative, it
        // over-reports.
        //
        // B19-R8 (#1718 deletes this; it widens B19-R7's removed-curve term): a successor whose
        // send join carries a following route into a following route may land a carried column
        // at once at the swap (a removed follow curve, or a column that composes otherwise),
        // cutting a live send move still ramping there, which render cannot
        // report. Control cannot tell which column lands at once or whether its move is in
        // flight, so such a rebuild completes as the transition fallback whenever any live send
        // move may still ramp: the transaction's own join records with a ramp (MAJOR-2: they are
        // not yet in the window, which folds them only after the commit), or a move of the
        // window (read here, before the reservation; a watermark read that gives up counts as
        // "may"). A declared discontinuity carries nothing. Conservative, it over-reports.
        // #1594 (attempt 2): a monitor rebuild that supersedes a withdrawn candidate A carries A's
        // edits against the plan A succeeds, so its join can carry a following route as a
        // transaction that supersedes A would: it takes the same outcome terms. A monitor rebuild
        // on the newest plan has an equal model and carries no route change. It inherits only when
        // it covers a revision the plan it succeeds has not published: a withdrawn monitor
        // candidate at the same revision carried no edit, and a claim at a revision the watermark
        // already reached would pass its outcome to the next advance.
        let carries_withdrawn_edits = (monitor || fallback)
            && superseding
            && self
                .kept_holder()
                .is_some_and(|holder| revision > holder.revision_gate.last_published());
        let transaction = matches!(kind, Rebuild::Transaction(_)) || carries_withdrawn_edits;
        let send_fallback = transaction
            && follow_into_following
            && (follow_retarget_ramp > 0
                || self.send_moves.is_some_and(|window| {
                    window.may_ramp(
                        self.shared.watermark.read().ok(),
                        self.publisher.render_clock(),
                    )
                }));
        // The rebuild's own send moves join the window with the commit: its join's records, and
        // a landing at once at the swap (a removed curve's, or one a flagged rebuild may make,
        // B19-R9), so a second rebuild inside that landing is flagged too.
        let rebuild_send_ramp = match &kind {
            Rebuild::Transaction(prepared) if follow_loses_a_curve || send_fallback => {
                let session = prepared.get().prospective_session();
                let next = session
                    .compiled()
                    .ok_or(CommandError::Internal)?
                    .normalized_model();
                follow_retarget_ramp.max(send_landing_ramp(next))
            }
            Rebuild::Transaction(_) => follow_retarget_ramp,
            Rebuild::Monitor { model, .. } | Rebuild::Transition { model, .. }
                if carries_withdrawn_edits =>
            {
                if follow_loses_a_curve || send_fallback {
                    follow_retarget_ramp.max(send_landing_ramp(model.normalized_model()))
                } else {
                    follow_retarget_ramp
                }
            }
            Rebuild::Discontinuity(_) | Rebuild::Monitor { .. } | Rebuild::Transition { .. } => 0,
        };
        // #1397 D5: the transition is the one writer of `TRANSITION_FALLBACK` beside these.
        let outcome =
            if transition_fallback || (transaction && (follow_into_six_word || send_fallback)) {
                engine::realtime::CandidateOutcome::TransitionFallback
            } else {
                engine::realtime::CandidateOutcome::Exact
            };
        let reservation = match self
            .publisher
            .reserve_replacement(candidate_plan.take(), adoption)
        {
            Ok(reservation) => reservation,
            Err(error) => {
                // #1398 D3: the exchange's publication cells and its retirement credits (#1398
                // D1) hold every plan a candidate can find there, so a full one is an accounting
                // fault, not backpressure. An incompatible envelope and an exhausted epoch stay as
                // they were.
                let (returned, refusal) = match error {
                    PlanReplacementReservationError::PublicationFull(plan)
                    | PlanReplacementReservationError::RetirementFull(plan) => {
                        debug_assert!(false, "#1398 D1 sizes the exchange for every candidate");
                        (plan, CommandError::Internal)
                    }
                    PlanReplacementReservationError::Incompatible(plan)
                    | PlanReplacementReservationError::EpochExhausted(plan) => {
                        (plan, CommandError::Backpressure)
                    }
                };
                drop(ObservedCandidatePlan::returned(returned));
                return Err(refusal);
            }
        };
        let mut reservation = ObservedReservation::new(reservation);
        #[cfg(feature = "test-support")]
        {
            if take_test_fault_state(TestStructuralFaultPhase::AfterPlanReservation) {
                drop(reservation);
                drop(candidate_provider);
                drop(kind);
                return Err(CommandError::Backpressure);
            }
        }
        // #1310 D4 (#1344 D3): the rings A created fresh that B keeps, checked while A is
        // withdrawn; nothing moves until after the commit. A withdrawn candidate was never
        // rendered, so neither refusal can arise: either is an internal fault.
        let donation = match withdrawn.as_ref() {
            Some(candidate) => {
                let donor = self
                    .pending_providers
                    .last()
                    .ok_or(CommandError::Internal)?;
                Some(
                    host_core::check_donation(
                        host_core::DonationSide {
                            plan: reservation.plan(),
                            sources: &candidate_provider.sources,
                            inventory: &candidate_provider.inventory,
                            model: prospective.normalized_model(),
                        },
                        host_core::DonationSide {
                            plan: candidate.plan(),
                            sources: &donor.sources,
                            inventory: &donor.inventory,
                            model: self.controller.session().compiled().normalized_model(),
                        },
                    )
                    .map_err(|_| CommandError::Internal)?,
                )
            }
            None => None,
        };
        // #1310, root rulings of 2026-10-16 (no ack before a drop; M-A of fold 3): a ring A
        // created (A added or redeclared the source) that the host fed and the donation above does
        // not move to B would drop with A, and with it PCM the host was acked for. Until an issue
        // lets B read it, such a supersession is refused here as backpressure (#1698 keeps the
        // reverted case refused: B starts a reverted source fresh, as the host contract says). The
        // guard reads the donation plan itself, so it sits after the check, still before the
        // commit: the reservation it follows is cancelled by the return (it returns its credit and
        // consumes no epoch), nothing else has changed, the caller republishes A, and once render
        // adopts A the same edit commits on it.
        if let (Some(plan), Some(candidate)) = (donation.as_ref(), withdrawn.as_ref()) {
            let donor = self
                .pending_providers
                .last()
                .ok_or(CommandError::Internal)?;
            if host_core::drops_acked_fresh_source(
                host_core::DonationSide {
                    plan: candidate.plan(),
                    sources: &donor.sources,
                    inventory: &donor.inventory,
                    model: self.controller.session().compiled().normalized_model(),
                },
                plan,
            ) {
                return Err(CommandError::Backpressure);
            }
        }
        // #1310 (root ruling B15-Q4): the observation records queued on A, re-queued on B's matching
        // instances, with their room checked here. A tap B carries from P0 takes P0's state at
        // the swap, which never saw them; arming from the bindings covers only a tap B starts.
        let mut forwards = match self.pending_providers.last() {
            Some(donor) if superseding => observation_forwards(
                &donor.effects,
                &self.observation_offsets,
                &self.observation_pending,
                &candidate_provider.effects,
                &observation_change.bindings,
            )?,
            _ => Vec::new(),
        };
        // #1584 T2: the records of this transaction's `Observe` edits, queued on the successor's
        // own fresh queues and never on the predecessor's, so a refused rebuild leaves no record
        // anywhere. A tap the successor carries takes its predecessor's state at the swap and then
        // drains the record (a new arm arms it, a disarm disarms it, a window change re-arms it); a
        // tap it starts is armed from the bindings already, and the record arms it at the same
        // block. A record a forward above already carries the bound state for is not queued twice,
        // and one whose target the transaction removed has no successor tap and is skipped (its
        // binding ended with the event).
        let observe_pushes = match &kind {
            Rebuild::Transaction(prepared) => observe_successor_pushes(
                prepared.get().prospective_session().observe_writes(),
                &candidate_provider.effects,
                &mut forwards,
            )?,
            Rebuild::Discontinuity(_) | Rebuild::Monitor { .. } | Rebuild::Transition { .. } => {
                Vec::new()
            }
        };
        // #1585 D4: the counter past the tags the preparation issued, installed at the commit.
        let next_arm_tag_after = match &kind {
            Rebuild::Transaction(prepared) => {
                prepared.get().prospective_session().next_arm_tag_after()
            }
            Rebuild::Discontinuity(_) | Rebuild::Monitor { .. } | Rebuild::Transition { .. } => {
                None
            }
        };
        let epoch = reservation.epoch();
        candidate_provider.epoch = epoch;
        // #1398 D3: the report table and the pending list hold every row a candidate can meet
        // (#1398 D1), so a full one is an accounting fault. A superseded candidate's row and
        // entry leave before B's arrive (#1310 D5). #2044: the lock is held for this check only,
        // so no step that can panic before the commit runs under it, and is taken again for the
        // row's push after the take; only the control plane writes the table (through `&mut
        // self`), so the check still holds there.
        {
            let reports = self
                .shared
                .reports
                .lock()
                .map_err(|_| CommandError::Internal)?;
            let freed = usize::from(superseding);
            if reports.len().saturating_sub(freed) >= reports.capacity()
                || self.pending_providers.len().saturating_sub(freed)
                    >= self.pending_providers.capacity()
            {
                debug_assert!(
                    false,
                    "#1398 D1 sizes the report table and the pending list"
                );
                return Err(CommandError::Internal);
            }
        }

        #[cfg(feature = "test-support")]
        if take_test_fault_state(TestStructuralFaultPhase::BeforeProtocolCommit) {
            drop(reservation);
            drop(candidate_provider);
            drop(kind);
            return Err(CommandError::Backpressure);
        }

        // #1312 D7: the candidate's gate is stamped with the structural revision, as the last
        // fallible step before the protocol commit. A fresh gate takes any revision in
        // `1..=GATE_REVISION_MAX`, which #1503 holds for a prepared transaction, so this cannot
        // refuse; were it to, the candidate and its handle are dropped here with nothing
        // published. Nothing else is stamped on the handle before the publication below. The
        // handle inherits the open stamp of the carry retargets preparation wrote (#1277 D5), so
        // this is the same revision again, which the pin allows.
        // #1594 D6: a monitor rebuild takes no stamp: it spends no revision, and the committed
        // one may be 0, which a gate never stamps.
        if !monitor && candidate_provider.revision_gate.stamp(revision).is_err() {
            drop(reservation);
            drop(candidate_provider);
            drop(kind);
            return Err(CommandError::Internal);
        }
        // #1584 D6a: the newest plan's pending-subscription storage is built here, with the other
        // fallible steps, so no allocation follows the commit; the commit then only assigns it. A
        // refusal drops what was built and changes nothing.
        let Ok(mut slots) = observation_slots(
            &candidate_provider.effects,
            &observation_change.bindings,
            self.observation_wire,
        ) else {
            drop(reservation);
            drop(candidate_provider);
            drop(kind);
            return Err(CommandError::CompileRejected(failure(
                ResourceFault::Allocation,
            )));
        };
        reservation.set_outcome(outcome);
        // #1310 D4: the protocol's own commit predicate, the last check.
        if let Rebuild::Transaction(prepared) = &kind
            && self
                .controller
                .check_prepared_structural(prepared.get())
                .is_err()
        {
            return Err(CommandError::Internal);
        }
        // #1325 D2: the producers phase 1 writes, resolved in the epoch of the running plan, P0
        // (the epoch the successor displaces), before the commit; a strip or route with no
        // producer, or an offsets record for a lane with no offsets cell, is an internal fault
        // found here, so no ack precedes a dropped write. Then the stamp P0's writes take: a
        // monitor publication under P0's last published revision, which spends none, so P0's
        // gate never reports this revision and the watermark reports it first when render
        // adopts the successor (D5). The stamp check is the last fallible step before the commit.
        let duck_writes = match transition.as_ref().filter(|transition| transition.ducks()) {
            Some(transition) => {
                let held = if superseding {
                    let len = self.pending_providers.len();
                    match len {
                        0 => return Err(CommandError::Internal),
                        1 => &mut self.providers,
                        _ => &mut self.pending_providers[len - 2],
                    }
                } else {
                    self.pending_providers
                        .last_mut()
                        .unwrap_or(&mut self.providers)
                };
                Some(resolve_duck_writes(held, transition)?)
            }
            None => None,
        };
        // #1403 D3: a warm candidate's flags and `Primed` schedule, before its set takes any
        // producer or donated ring: each entry it holds vacant is then a ring it carries from the
        // plan it displaces, whose producer is still in that plan's set, or, across a withdrawn
        // candidate, in the withdrawn candidate's. It sets the prime flags of rings render reads,
        // so it runs after the last refusal: the commit below cannot refuse once its predicate
        // passed above, with no controller write between. #2044: it runs before the protocol
        // commit, so a panic here or in the commit (no lock is held) leaves the controller as it
        // was and republishes the withdrawn candidate (#2010). Its flags are then the
        // successor's: `rebuild_withdrawn` sets a `Primed` candidate's again before it republishes
        // it (`restore_prime_flags`).
        let mut primed = warm_lead_blocks.map(|lead_blocks| {
            let (running, withdrawn_sources) = if superseding {
                let len = self.pending_providers.len();
                // A superseding rebuild holds the withdrawn candidate as its newest pending
                // epoch: the check at the top of `rebuild` refused anything else.
                debug_assert!(len > 0, "a withdrawn candidate is pending");
                let holder = match len {
                    0 | 1 => &self.providers,
                    _ => &self.pending_providers[len - 2],
                };
                (holder, self.pending_providers.last())
            } else {
                (
                    self.pending_providers.last().unwrap_or(&self.providers),
                    None,
                )
            };
            host_core::publish_primed(
                reservation.inner.as_mut().expect("reservation is live"),
                &running.sources,
                &candidate_provider.sources,
                withdrawn_sources.map(|withdrawn| &withdrawn.sources),
                lead_blocks,
            )
        });
        #[cfg(feature = "test-support")]
        if take_test_fault_state(TestStructuralFaultPhase::PanicAtPrimedPublication) {
            panic!("test fault: a rebuild panics after it publishes the primed schedule");
        }
        let (committed, displaced) = match kind {
            Rebuild::Transaction(prepared) => {
                // #1313 D2: a transaction that takes this arm is a rebuild, whatever else it holds.
                let mut committed = prepared
                    .commit(&mut self.controller, protocol::EditPath::Rebuild)
                    .map_err(|_| CommandError::Internal)?;
                // #1310 D3: the model the commit displaced, never one committed in place (#1613).
                // `take_displaced` is `Option::take`, the one step between the commit and the
                // candidate's take (#2044), and it cannot panic. It is `Some`: the preparation
                // refused a token without a compilation (`compiled().ok_or`), a compiled token
                // always displaces the committed model, and a monitor-only token, which displaces
                // none, never takes this arm: it commits live (`commit_live`, #1603 D7).
                let displaced = committed.take_displaced();
                (Some(committed), displaced)
            }
            // #1323 D4: nothing is committed. The copy of the committed model is the base the
            // epoch this successor displaces keeps (the model that epoch renders), or, across a
            // withdrawn candidate, drops below as a commit's displaced model does.
            Rebuild::Discontinuity(model)
            | Rebuild::Monitor { model, .. }
            | Rebuild::Transition { model, .. } => (None, Some(*model)),
        };
        // #2044: the candidate is taken first, so no write below precedes the step after which a
        // panic no longer republishes it.
        let withdrawn = withdrawn.take();
        // B19-R7: the rebuild's own send moves ramp from its successor's first block.
        fold_send_move(&mut self.send_moves, revision, rebuild_send_ramp);
        // #1559 D8: the overlay is written after the last fallible step, with the commit.
        let monitor_ramp = writes_monitor.then_some(monitor_change.ramp);
        let previous_monitor = core::mem::replace(&mut self.monitor, monitor_change.overlay);
        // #1584 D6a: and the subscriptions the successor was prepared with.
        self.observation = observation_change.bindings;
        // From here on nothing fails.
        let mut superseded = None;
        let superseded_revisions = match withdrawn {
            Some(candidate) => {
                // #1310 D5 step 2: the checked donation, then the producers of the sources B
                // carries from P0, which A's commit had moved into A's set. A's own model, which
                // B's commit displaced, drops here.
                drop(displaced);
                let mut donor = self
                    .pending_providers
                    .pop()
                    .expect("the withdrawn candidate is the newest pending epoch");
                let mut donor_plan = candidate.into_plan();
                if let Some(donation) = donation {
                    host_core::apply_donation(
                        host_core::DonationSideMut {
                            plan: reservation.plan_mut(),
                            sources: &mut candidate_provider.sources,
                        },
                        host_core::DonationSideMut {
                            plan: &mut donor_plan,
                            sources: &mut donor.sources,
                        },
                        donation,
                    );
                }
                candidate_provider
                    .sources
                    .adopt_persisting(&mut donor.sources);
                // P0, the epoch B succeeds, keeps its base with B's edit ramps merged in.
                let holder = self
                    .pending_providers
                    .last_mut()
                    .unwrap_or(&mut self.providers);
                // #1325 D6: a source B restores in phase 1 carries P0's ring, so its producer
                // moves from P0's set, where phase 1 kept it, into B's. Every other entry B holds
                // vacant took its producer from A's set above, so only restored ones move here.
                let restored = candidate_provider
                    .sources
                    .adopt_persisting(&mut holder.sources);
                debug_assert!(
                    restored
                        <= transition
                            .as_ref()
                            .map_or(0, |transition| transition.restored_sources().len()),
                    "only a restored source's producer moves from the running plan's set"
                );
                if let Some(kept) = holder.kept.as_mut() {
                    kept.edit_ramps = kept_ramps;
                    if monitor_ramp.is_some() {
                        kept.monitor_ramp = monitor_ramp;
                    }
                }
                // #1310 D6: every revision after the last one P0's gate published and before B's
                // was committed onto A or a candidate A superseded, and its content is in B.
                let last_running = holder.revision_gate.last_published();
                superseded = Some((donor_plan, donor));
                revision.saturating_sub(1).saturating_sub(last_running)
            }
            None => {
                // Issue #1273 D2: the protocol commit was the last fallible step, so the
                // persisting producers move now and never move back.
                let newest = self
                    .pending_providers
                    .last_mut()
                    .unwrap_or(&mut self.providers);
                candidate_provider
                    .sources
                    .adopt_persisting(&mut newest.sources);
                // #1310 D3: the epoch B succeeds keeps the model the commit displaced, the
                // overlay its cells hold and B's edit ramps, while B is pending.
                if let Some(model) = displaced {
                    newest.kept = Some(KeptBase {
                        model,
                        monitor: previous_monitor,
                        edit_ramps: kept_ramps,
                        monitor_ramp,
                        duck: host_core::DuckOverlay::new(),
                    });
                }
                // Nothing was withdrawn, so nothing is pending but B: a taken candidate was
                // promoted before the rebuild (`withdraw_pending_candidate`), and promotion drops
                // every older epoch's kept base.
                debug_assert_eq!(
                    self.pending_providers.len(),
                    0,
                    "a rebuild that supersedes nothing meets no pending candidate"
                );
                0
            }
        };
        // #1325 D2, D3, D6: phase 1 on P0, which is now the newest epoch the control plane holds
        // (a withdrawn candidate's epoch was popped above; the successor is pushed below). The
        // ramped mutes go to P0's cells under the stamp checked before the commit, then `p` is
        // read, then the successor's schedule is set: `S` from this transaction's writes, raised
        // to the overlay's, which a superseding candidate inherits.
        if let Some(mut next_duck) = next_duck {
            let held = self
                .pending_providers
                .last_mut()
                .unwrap_or(&mut self.providers);
            if let (Some(transition), Some(writes)) = (transition.as_ref(), duck_writes.as_ref()) {
                write_duck(held, transition, writes);
                let p = reservation
                    .inner
                    .as_ref()
                    .expect("reservation is live")
                    .render_clock();
                next_duck.raise_not_before(transition.not_before(p, quantum));
            }
            // #1397 D1: a warm candidate whose transition ducks a strip stays `Primed`, no earlier
            // than `S`; one whose overlay holds only removed sources no strip read keeps its
            // render-clock `not_before`, which is never earlier. Every other candidate waits for
            // `S` as a two-phase removal or a duck-swap does.
            if let Some(not_before) = next_duck.not_before() {
                let reservation = reservation.inner.as_mut().expect("reservation is live");
                match primed.as_mut() {
                    Some(record) => record.hold_until(reservation, not_before),
                    None => reservation.set_adoption(PlanAdoption::NoEarlierThan(not_before)),
                }
            }
            let held = self
                .pending_providers
                .last_mut()
                .unwrap_or(&mut self.providers);
            if let Some(kept) = held.kept.as_mut() {
                kept.duck = next_duck;
            }
        }
        self.controller
            .provider_mut()
            .replace_session_catalog(candidate_catalog, revision);
        // #1502 D6: the candidate carries the revision just committed. It goes on the candidate's
        // own gate before the candidate is published, so render reports it when it adopts the
        // candidate and never earlier, and nothing is stamped on the handle in between.
        debug_assert_eq!(
            revision,
            self.controller.session().revision().0,
            "the revision stamped is the revision committed (#1312 D7)"
        );
        // #1594 D6: a monitor rebuild's revision is the committed one, published only above 0, as
        // the first plan's is: a fresh gate holds 0 already.
        if !monitor || revision > 0 {
            candidate_provider.revision_gate.publish(revision);
        }
        // #1310 D5 step 3, D6: B's `superseded` word, written before B is published.
        reservation.set_superseded(superseded_revisions);
        // #1310 (root ruling B15-Q4): A's queued observation records, on B's queues before B is
        // published. Their room was checked before the commit.
        // #1584 T2: then the transaction's own records, likewise, their room checked before the
        // commit. #1585 D4: the transaction's records carry the tags its preparation issued, and
        // the counter moves past them only here, after the commit, so a refused rebuild issues
        // none; a record with no tag takes a fresh one. The slot and an armed binding take the tag
        // its prepared arm or a reader compares against.
        if let Some(next) = next_arm_tag_after {
            self.next_arm_tag = next;
        }
        for record in forwards.iter().chain(&observe_pushes) {
            let arm_tag = if record.arm_tag == 0 {
                let tag = self.next_arm_tag;
                self.next_arm_tag = tag.saturating_add(1);
                tag
            } else {
                record.arm_tag
            };
            let producer = &mut candidate_provider.effects[record.position];
            producer
                .try_push_observe(
                    record.tap_index,
                    record.armed,
                    record.window_blocks,
                    arm_tag,
                )
                .expect("the room for every observe record was checked before the commit");
            if record.armed
                && let Some(key) = tap_key_of(producer, record.tap_index)
            {
                self.observation.set_arm_tag(&key, arm_tag);
            }
            let first = slots.0[record.position];
            if let Some(slot) = slots.1.get_mut((first + record.tap_index) as usize) {
                slot.queued = true;
                slot.tag = arm_tag;
            }
        }
        // #1593 D4: the configuration the successor was prepared with is the session's from now
        // on; its epoch is the one the candidate's captures report at promotion.
        let configured_epoch = self.spectrum.commit(spectrum_configuration);
        if let crate::spectrum::EpochSpectrum::Candidate { epoch, .. } =
            &mut candidate_provider.spectrum
        {
            *epoch = configured_epoch;
        }
        let candidate_epoch = candidate_provider.epoch;
        self.pending_providers.push(candidate_provider);
        // #1690 D2: the successor takes the master this commit leaves; the plan still rendering
        // keeps its own until it is displaced, as the browser's `sync_master` does.
        sync_master_meters(
            &self.monitor,
            &mut self.providers,
            &mut self.pending_providers,
            &mut self.meters,
        );
        // #1577 D4: a capture armed from here on waits for the candidate.
        self.controller
            .provider_mut()
            .set_newest_plan_epoch(candidate_epoch);
        // #1584 D6a: the newest plan's slots, built above. The fold ran when this command was
        // serviced and nothing has been pushed since (one thread), so none is lost.
        (
            self.observation_offsets,
            self.observation_pending,
            self.observation_batch_counts,
            self.observation_armed,
            self.observation_armed_bytes,
        ) = slots;
        self.observation_slots_high_water = self
            .observation_slots_high_water
            .max(self.observation_pending.len());
        // #1577: the committed model's owners join the capture's high-water; the successor's
        // charge already counted them (`response_capture_bytes`), so this can only hold or rise.
        self.capture_owners_high_water = self.capture_owners_high_water.max(
            host_core::response_capture_owners(
                self.controller.session().compiled().normalized_model(),
            )
            // Unreachable: the successor's preparation counted this model's owners before the
            // commit (`prepared_capi_resources` -> `response_capture_bytes`) and refused the
            // transaction on an overflow, so the count is `Some` here.
            .unwrap_or(usize::MAX),
        );
        // #2044: the report lock again, held to the publication so the row is in the table
        // before render can adopt B. A poisoned lock is taken as it is: only the control plane
        // writes the table (through `&mut self`), and no write of it can stop half done; every
        // other holder only reads it (`active_resources`, the admissions' reads), so a panic that
        // poisoned it left the rows whole. The step after the take must not fail.
        let mut reports = self
            .shared
            .reports
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some((_, donor)) = superseded.as_ref() {
            // #1310 D5 step 6: A's report row leaves; `retain` shrinks in place.
            let donor_epoch = donor.epoch;
            reports.retain(|(row, _)| *row != donor_epoch);
        }
        reports.push((epoch, A::Row::from(resources)));
        #[cfg(feature = "test-support")]
        TEST_PUBLISHED_ADOPTION.with(|state| {
            state.set(
                reservation
                    .inner
                    .as_ref()
                    .map(PlanReplacementReservation::adoption),
            );
        });
        // #1332 S5: B carries the master and the structure epoch this commit leaves, stamped
        // before it is published, so a render half on another thread that adopts B reads them
        // from B whatever a later commit has done since.
        self.preparer.stamp_before_publication(
            reservation.plan_mut(),
            self.controller.session().compiled(),
            self.monitor.master(),
            self.controller.provider().structure_epoch(),
        );
        // #1310 D5 step 4: B is published only now, owning every donated ring and record. Its
        // payload carries the revision its gate carries, for the watermark's claim settle (#1482
        // R1).
        reservation.commit(revision);
        // #1403 D4: B is the one pending candidate now. A record that named a candidate B
        // superseded drops with it (#1310 D5 step 6, #1323's stop); B's own replaces it.
        debug_assert!(
            self.primed.is_none_or(|record| superseded
                .as_ref()
                .is_some_and(|(_, donor)| donor.epoch == record.epoch)),
            "a record names the one pending candidate"
        );
        self.primed = primed;
        drop(reports);
        #[cfg(feature = "test-support")]
        {
            update_test_owners(|owners| {
                owners.candidate_provider_published += 1;
            });
        }
        if let Some((donor_plan, mut donor)) = superseded {
            // #1310 D5 step 5: P0 still renders until B is adopted, so every producer A still
            // holds that P0 holds vacant (a source B removed or redeclared) goes back to P0's
            // epoch before A drops. Infallible and allocation-free; keyed by source ID.
            let holder = self
                .kept_holder_mut()
                .expect("B succeeds the epoch A succeeded");
            holder.sources.adopt_persisting(&mut donor.sources);
            // Step 6: A's plan, provider epoch (with its gate handle) and credit drop here, on the
            // control thread. The credit went back when the plan was taken out.
            drop(ObservedCandidatePlan::returned(donor_plan));
            drop(donor);
            #[cfg(feature = "test-support")]
            update_test_owners(|owners| owners.candidate_superseded += 1);
        }
        Ok(committed.map_or(0, |committed| {
            committed
                .write_into(&mut self.response_scratch)
                .expect("prepared response capacity was admitted before protocol commit")
        }))
    }

    /// The live arm of a structural command (#1257 D2, #1053 D6).
    ///
    /// Classifies the committed model against the token's prospective one. A rebuild hands the
    /// token back untouched. A live delta runs, in order: the live admission (#1053 D8), the
    /// producer resolution by strip ID, and by strip ID and live address for an effect, in the
    /// newest epoch (#1053 D7), each EQ owner's transaction -- `begin_owner`, `edit_owner` per
    /// edit and `preflight_candidate_targets` for the classifier's designed targets (#1265 D2) --
    /// every effect record's preflight (#1264 D3),
    /// and the protocol's own commit predicate. Each of them refuses before any cell or the model
    /// changes, and a refusal after an owner was begun discards every begun owner (`OpenOwners`).
    ///
    /// Every live value is a latest-target cell (#1312, #1345): a strip's fader, mute and
    /// matrix/pan values, its input trim, polarity and filter targets (#1556), a send's folded
    /// target and gate (#1557), an effect's parameter values, its bypass and an EQ's targets. A write
    /// needs no room, so nothing refuses a delta for room and nothing turns it into a rebuild
    /// because it outgrew a queue. After the predicate and the test fault hook, the commit takes
    /// its stamp from the newest epoch's gate as its last fallible step (D7). Only then does it
    /// write every strip cell, every effect cell and every EQ owner's target cells, commit the
    /// token and every owner, and publish the stamped revision on the gate, and none of them can
    /// fail: a cell has no room to want, every effect record passed its producer's preflight and
    /// every target set its owner's, and the predicate is the commit's own under this `&mut`
    /// borrow. So no acked edit is ever dropped, and no refused edit leaves a cell write, a
    /// target, an open stamp or an open owner behind. An instance's `Bypass` record (#1266 D2) is
    /// a cell write on every instance, the EQ's included: render reads the bypass cell first and
    /// the parameter and target cells after it (#1345 D3).
    ///
    /// It never reserves a plan or adds a report row. It stages the committed model's control
    /// catalog before the first write and publishes it after the commit, so the control list
    /// reports what a rebuild's catalog would (#1264 D4, #1265 D3, #1552 D3). The stage reads only
    /// the rows of the owners whose values changed and the derived rows, unless an address, a
    /// descriptor or a setting changed, which stages the whole catalog (#1600). The structure
    /// epoch advances if a descriptor or an address changed: a VCA added or removed live adds or
    /// removes its rows (#1558 D5), a member change, an offset or a mute changes none.
    ///
    /// **A token prepared in place (#1613).** A transaction of `SetControl`s, on existing values or
    /// adding parameter entries (#1620), holds the owners it edited and no compilation
    /// (`PreparedSessionTransaction::is_in_place`). The classification, the admission, the catalog stage and the producers' preflight read the
    /// committed model and those owners as one view (`controls::ModelView`), and nothing writes the
    /// committed model before the commit: the protocol commit swaps the owners in as the last write.
    /// If the fast classification cannot judge the transaction, the token is materialized (the full
    /// preparation of its edits) and this arm goes on as for any other token, a rebuild included.
    fn commit_live(&mut self, prepared: ObservedPreparedToken) -> LiveCommit {
        match self.commit_live_once(prepared) {
            LiveCommit::NeedsFull(mut prepared) => {
                // #1613: the fast classification could not judge the in-place transaction. Its
                // edits are prepared by the full path now (nothing was written), and the arm goes
                // on as it always did, a rebuild included.
                #[cfg(feature = "test-support")]
                IN_PLACE_COUNTS.with(|counts| {
                    let (commits, materialized) = counts.get();
                    counts.set((commits, materialized + 1));
                });
                if self
                    .controller
                    .materialize_prepared(prepared.get_mut())
                    .is_err()
                {
                    return LiveCommit::Done(Err(CommandError::Internal));
                }
                self.commit_live_once(prepared)
            }
            other => other,
        }
    }

    /// One attempt of [`Self::commit_live`]: the live arm over the token as it is. A token prepared
    /// in place is classified from its rows over the view of the committed model and its edited
    /// owners; if that cannot judge, the token is handed back as [`LiveCommit::NeedsFull`].
    fn commit_live_once(&mut self, prepared: ObservedPreparedToken) -> LiveCommit {
        // #1584 D4: the records of the transaction's `Observe` edits, resolved by stable key on the
        // newest plan and admitted whole (one naming per tap, the ended event's frame, the room)
        // before any write; nothing is queued until after the protocol commit, the last fallible
        // step, so no acknowledgement precedes a dropped record. Admission writes nothing, so its
        // refusal is held until the classification below says the transaction is live: a rebuild
        // queues its records on its successor instead (T2). The table the edits leave is cloned
        // now, so the commit only moves it.
        let observe_bindings = prepared
            .get()
            .prospective_session()
            .observe_bindings()
            .cloned();
        // #1585 D4: the counter after the records' tags, installed with the table.
        let next_arm_tag_after = prepared.get().prospective_session().next_arm_tag_after();
        let observe_changes =
            self.admit_observe_writes(prepared.get().prospective_session().observe_writes());
        let transaction = prepared.get().prospective_session();
        let in_place = transaction.is_in_place();
        let committed = self.controller.session().compiled().normalized_model();
        // The model after the transaction, read as the committed model and the owners the
        // transaction edited (#1613), or the prospective compilation's model, whole.
        let view = match (transaction.edited_owners(), in_place) {
            (Some(owners), _) => controls::ModelView::edited(committed, owners),
            (None, true) => controls::ModelView::whole(committed),
            (None, false) => controls::ModelView::whole(
                transaction
                    .compiled()
                    .expect("a compiled transaction")
                    .normalized_model(),
            ),
        };
        let next = view.structure();
        // #1696: only a transaction compiled whole can add or remove an owner (an in-place or an
        // edited view reads the committed model's structure); every other keeps the owner set,
        // and with it the ramp row's bound and a kept base's pruning, as they are.
        let owners_change = matches!((transaction.edited_owners(), in_place), (None, false));
        // #1559 D6, D7: the overlay the transaction leaves, and the classifier's view of both.
        let monitor_change = self.monitor.after(prepared.get().monitor_edits(), next);
        let monitor_only = prepared.get().is_monitor_only();
        let monitor = host_core::MonitorBase {
            committed: &self.monitor,
            next: &monitor_change.overlay,
            ramp: monitor_change.ramp,
        };
        let ramps = host_core::LiveRamps::for_session(next);
        let delta = if in_place {
            // #1613: classified from the rows the edits name over the view, with no model built.
            let rows = transaction.set_control_rows().unwrap_or_default();
            match host_core::classify_live_staged(
                committed,
                view,
                ramps,
                transaction.edit_ramps(),
                monitor,
                rows,
            ) {
                Some(delta) => delta,
                None => return LiveCommit::NeedsFull(prepared),
            }
        } else {
            // #1612: a transaction of `SetControl`s is classified from the rows it names, with no
            // model clone and no canonical JSON; any other, and any row the fast path cannot
            // judge, runs the full classifier inside this one call.
            match host_core::classify_live_edit(
                committed,
                next,
                ramps,
                transaction.edit_ramps(),
                monitor,
                transaction.set_control_rows(),
            ) {
                Ok(delta) => delta,
                Err(_) => {
                    // A monitor-only transaction never rebuilds (#1603 D7): its records are strip
                    // mute and route gate records, which every plan carries, so a rebuild verdict
                    // is a defect.
                    if monitor_only {
                        return LiveCommit::Done(Err(CommandError::Internal));
                    }
                    return LiveCommit::Rebuild(prepared);
                }
            }
        };
        // #1584 D4: a live transaction's subscriptions are refused here, with nothing written, when
        // their admission failed: a full observation queue is retryable backpressure.
        let observe_changes = match observe_changes {
            Ok(changes) => changes,
            Err(error) => return LiveCommit::Done(Err(error)),
        };
        // Whether the commit publishes on the plan gate: every model transaction does (a delta
        // with no record is a revision render has nothing to do for), and a transaction of
        // monitor writes does when the overlay change produced a record (#1559 D4, #1603 D7).
        let publishes = !monitor_only || !delta.is_empty();
        // #1310 (root ruling B15-Q3): while a successor is pending, the ramp lengths this edit gives
        // its rows join the kept base of the epoch it succeeds, so a replacement prepared across a
        // withdrawn candidate ramps them as this edit does. Built before any check, so the commit
        // below only moves it; `None` when nothing is pending or the edit wrote no live row.
        let kept_ramps = self
            .kept_holder()
            .and_then(|holder| holder.kept.as_ref())
            .filter(|_| {
                !prepared
                    .get()
                    .prospective_session()
                    .edit_ramps()
                    .is_untouched()
            })
            .map(|kept| {
                let mut ramps = merged_ramps(
                    &kept.edit_ramps,
                    prepared.get().prospective_session().edit_ramps(),
                );
                // #1696 root ruling: pruned to the committed model, as a rebuild stores them.
                if owners_change {
                    ramps.keep_within(next);
                }
                ramps
            });
        // And the ramp of the overlay change, when the edit writes a monitor row: a replacement
        // that writes none renders the solo with it (#1310, kept monitor overlay).
        let kept_monitor_ramp =
            (!prepared.get().monitor_edits().is_empty()).then_some(monitor_change.ramp);

        // 1. Admission (#1053 D8), with the catalog of the committed model built first, so that
        // its growth is charged and the commit's last steps cannot fail for room (#1552 D3,
        // #1558 D6). Its refusal is the compile diagnostic a rebuild would give, before any write.
        // #1696 root ruling: the committed model's edit-ramp row, which the newest epoch's follows.
        let committed_ramps = if owners_change {
            match monitor_rows(next) {
                Ok(rows) => rows.edit_ramps,
                Err(failure) => {
                    return LiveCommit::Done(Err(CommandError::CompileRejected(failure)));
                }
            }
        } else {
            self.pending_providers
                .last()
                .unwrap_or(&self.providers)
                .capi
                .edit_ramps
        };
        let catalog = match self.live_admission(
            &prepared,
            &monitor_change.overlay,
            view,
            &delta,
            committed_ramps,
        ) {
            Ok(catalog) => catalog,
            Err(error) => return LiveCommit::Done(Err(error)),
        };

        // 2. Resolve each strip's producer by ID in the newest epoch (#1053 D7), among all strips,
        // tracks then submixes (#1554 D6): a binary search per segment of the strip table, whose
        // IDs are sorted within each segment.
        let ProviderEpoch {
            epoch: newest_epoch,
            strips,
            effects,
            revision_gate,
            capi: newest_capi,
            ..
        } = match self.pending_providers.last_mut() {
            Some(provider) => provider,
            None => &mut self.providers,
        };
        let track_count = strips.track_count;
        let strip_lanes = &mut strips.controls;
        let strip_rank = |id: &str| {
            host_core::strip_position(
                strip_lanes.len(),
                track_count,
                |index| &*strip_lanes[index].track_id,
                id,
            )
        };
        let mut resolved = Vec::new();
        if resolved.try_reserve_exact(delta.strips.len()).is_err() {
            return LiveCommit::Done(Err(CommandError::Internal));
        }
        for strip in &delta.strips {
            let Some(index) = strip_rank(strip.strip_id) else {
                return LiveCommit::Done(Err(CommandError::Internal));
            };
            resolved.push(index);
        }

        // The effect instances, by strip ID and live address (#1264 D3), through the producer
        // table's sort (#1554 D6).
        let mut resolved_effects = Vec::new();
        let mut begun = Vec::new();
        if resolved_effects
            .try_reserve_exact(delta.effects.len())
            .is_err()
            || begun.try_reserve_exact(delta.effects.len()).is_err()
        {
            return LiveCommit::Done(Err(CommandError::Internal));
        }
        for instance in &delta.effects {
            let Some(index) = host_core::effect_producer_position(
                effects,
                strip_rank,
                instance.strip_id,
                instance.address,
            ) else {
                return LiveCommit::Done(Err(CommandError::Internal));
            };
            // A delta that carries targets addresses an instance that owns them (#1265 D2).
            if instance.targets.is_some() && !effects[index].has_owner() {
                return LiveCommit::Done(Err(CommandError::Internal));
            }
            resolved_effects.push(index);
        }

        // 2a. Each route's producer, by route ID (#1557 D4): a binary search over the canonical
        // route-ID order, among the six-word producers for a folded record and among the
        // seven-word ones for a following route's record (#1653 D1). A record for a route with no
        // producer of its kind, or a seven-word record whose automated lanes (flags bits 4-5)
        // differ from the plan's (render takes them from its own cursors), is an internal fault, found before the first write, so no ack
        // precedes a drop (#1653 D5).
        let route_lanes = &mut strips.routes;
        let follow_lanes = &mut strips.follow_routes;
        let mut resolved_routes = Vec::new();
        if resolved_routes
            .try_reserve_exact(delta.routes.len())
            .is_err()
        {
            return LiveCommit::Done(Err(CommandError::Internal));
        }
        for route in &delta.routes {
            let index = match route.record {
                host_core::SendRecord::Folded(_) => {
                    host_core::route_producer_position(route_lanes, route.route_id)
                }
                host_core::SendRecord::Following(record) => {
                    host_core::follow_route_producer_for(follow_lanes, route.route_id, &record)
                }
            };
            let Some(index) = index else {
                return LiveCommit::Done(Err(CommandError::Internal));
            };
            resolved_routes.push(index);
        }
        // 2a'. A delta with new jump lengths writes them to the newest plan's cell (#1639 D3),
        // which every epoch holds (D1).
        let jump_lane = &mut strips.jump_lengths;

        // 2b. A strip with input records has an input writer, and every filter target passes the
        // contract the writer assumes (#1556 D5). Both are checked before the first write: the
        // cells' write cannot fail and panics on a target it was never validated for. A strip
        // with offsets-cell writes has an offsets cell on each lane it writes (#1649 D4, D7): the
        // newest epoch's plan automates the lanes the committed model does, so a missing cell is
        // an internal fault, refused before any write, never acknowledged and then dropped.
        for (strip, &index) in delta.strips.iter().zip(&resolved) {
            let fader = &strip_lanes[index].fader;
            if strip
                .offsets
                .iter()
                .enumerate()
                .any(|(channel, record)| record.is_some() && !fader.has_offsets(channel))
            {
                return LiveCommit::Done(Err(CommandError::Internal));
            }
            if strip.input.is_empty() {
                continue;
            }
            if strip_lanes[index].input.is_none() {
                return LiveCommit::Done(Err(CommandError::Internal));
            }
            if !strip.input_records_valid() {
                return LiveCommit::Done(Err(CommandError::Internal));
            }
        }

        // 3. Each EQ owner's transaction (#1265 D2): begin at its committed revision, apply the
        // edits, and preflight the designed targets (revision and validation; the targets are
        // cells and want no room). From here on, every refusal discards every owner begun, as
        // `OpenOwners` drops.
        let mut owners = OpenOwners { effects, begun };
        for (instance, &index) in delta.effects.iter().zip(&resolved_effects) {
            let Some(targets) = &instance.targets else {
                continue;
            };
            let producer = &mut owners.effects[index];
            let Some(base_revision) = producer.owner().map(|owner| owner.committed_revision())
            else {
                return LiveCommit::Done(Err(CommandError::Internal));
            };
            // `begin` allocates the candidate fallibly (#1624): its `Allocation` refusal maps to
            // `INTERNAL`, as every failed `try_reserve_exact` of this commit does, before the
            // stamp. Nothing is written or acknowledged, and `OpenOwners` discards the owners
            // begun so far.
            if producer.begin_owner(base_revision).is_err() {
                return LiveCommit::Done(Err(CommandError::Internal));
            }
            owners.begun.push(index);
            for &record in &instance.records {
                let (parameter_index, channel, value) = match record {
                    effect_contract::EffectControlRecord::Parameter {
                        parameter_index,
                        channel,
                        value,
                    } => (parameter_index, channel, value),
                    // Pushed, not an owner edit (#1266 D2).
                    effect_contract::EffectControlRecord::Bypass(_) => continue,
                    _ => return LiveCommit::Done(Err(CommandError::Internal)),
                };
                if producer
                    .edit_owner(parameter_index, channel, value)
                    .is_err()
                {
                    return LiveCommit::Done(Err(CommandError::Internal));
                }
            }
            if producer
                .preflight_candidate_targets(base_revision, targets)
                .is_err()
            {
                return LiveCommit::Done(Err(CommandError::Internal));
            }
        }

        // 4. Every written effect record passes its producer's preflight before the first write
        // (#1264 D3). The classifier refuses every record a producer would, so a refusal here is
        // an internal fault. An EQ's parameter records are its owner's edits, never written: its
        // targets were preflighted above.
        for (instance, &index) in delta.effects.iter().zip(&resolved_effects) {
            let producer = &owners.effects[index];
            for &record in &instance.records {
                match record {
                    // Written on every instance, the EQ's too; the record carries the session's
                    // crossfade ramp (#1341 D1). A link switch (#1371) is written as it is; no
                    // target owner has a link cell, so its producer refuses one.
                    effect_contract::EffectControlRecord::Bypass(_)
                    | effect_contract::EffectControlRecord::Link(_) => {
                        if producer.preflight(record).is_err() {
                            return LiveCommit::Done(Err(CommandError::Internal));
                        }
                    }
                    effect_contract::EffectControlRecord::Parameter { .. } => {
                        if instance.targets.is_none() && producer.preflight(record).is_err() {
                            return LiveCommit::Done(Err(CommandError::Internal));
                        }
                    }
                    _ => return LiveCommit::Done(Err(CommandError::Internal)),
                }
            }
        }

        // 5. The protocol commit's own predicate.
        if self
            .controller
            .check_prepared_structural(prepared.get())
            .is_err()
        {
            return LiveCommit::Done(Err(CommandError::Internal));
        }
        #[cfg(feature = "test-support")]
        if take_test_fault_state(TestStructuralFaultPhase::BeforeLivePush) {
            drop(owners);
            drop(prepared);
            return LiveCommit::Done(Err(CommandError::Backpressure));
        }

        // 6. The stamp is the last fallible step (#1312 D7): the revision this transaction commits
        // as, from the gate of the newest epoch, whose producers the writes below target. A
        // refusal writes nothing and uses no revision. Every step from here to the revision's
        // publication cannot fail: a cell write has no room to want, every record passed its
        // preflight, and the commit's predicate held under this borrow.
        // A model transaction stamps its own revision (`(r + 1, 0)` in the gate's order); a
        // monitor-only one spends no revision and stamps a monitor publication under the committed
        // revision `r` (#1603 D7). A monitor-only transaction with no record takes no stamp and
        // publishes nothing. An exhausted publication number refuses, as `INTERNAL`.
        let revision = prepared.get().prospective_session().revision().0;
        let stamp = if !publishes {
            None
        } else {
            let stamped = if monitor_only {
                revision_gate.stamp_monitor(revision)
            } else {
                revision_gate.stamp(revision)
            };
            let Ok(stamp) = stamped else {
                drop(owners);
                drop(prepared);
                return LiveCommit::Done(Err(CommandError::Internal));
            };
            Some(stamp)
        };
        #[cfg(feature = "test-support")]
        if in_place {
            IN_PLACE_COUNTS.with(|counts| {
                let (commits, materialized) = counts.get();
                counts.set((commits + 1, materialized));
            });
        }
        // The key is read into a local now, so the publication below is the stamped one.
        let key = stamp.map(|stamp| stamp.key());
        // Write the strips' cells, then each effect's cells and each EQ owner's target cells.
        // Nothing above changed anything but the owners' open candidates.
        if let Some(stamp) = stamp {
            for (strip, &index) in delta.strips.iter().zip(&resolved) {
                let producer = &mut strip_lanes[index];
                for record in strip.fader_records() {
                    producer.fader.write(stamp, record);
                }
                // #1649 D4: each automated lane's new `S`, the whole slot at once.
                for (channel, record) in strip.offsets.iter().enumerate() {
                    if let Some(record) = *record {
                        producer.fader.write_offsets(stamp, channel, record);
                    }
                }
                if let Some(record) = strip.matrix {
                    producer.producer.write(stamp, record);
                }
                if let Some(input) = producer.input.as_mut() {
                    for &record in &strip.input {
                        input.write(stamp, record);
                    }
                }
            }
            // #1653: a following route's cell under the same stamp as its source strip's terms
            // write, so both apply in one block.
            for (route, &index) in delta.routes.iter().zip(&resolved_routes) {
                match route.record {
                    host_core::SendRecord::Folded(record) => {
                        route_lanes[index].write(stamp, record);
                    }
                    host_core::SendRecord::Following(record) => {
                        follow_lanes[index].write(stamp, record);
                    }
                }
            }
            // #1639 D3: the new jump lengths, in the same block as the records.
            if let Some(lengths) = delta.jump_lengths {
                jump_lane.write(stamp, lengths);
            }
            for (instance, &index) in delta.effects.iter().zip(&resolved_effects) {
                let producer = &mut owners.effects[index];
                match &instance.targets {
                    None => {
                        for &record in &instance.records {
                            write_effect_record(producer, stamp, record);
                        }
                    }
                    Some(targets) => {
                        for &record in &instance.records {
                            if matches!(record, effect_contract::EffectControlRecord::Bypass(_)) {
                                write_effect_record(producer, stamp, record);
                            }
                        }
                        let base_revision = producer
                            .owner()
                            .map(|owner| owner.committed_revision())
                            .expect("an owner was begun");
                        producer
                            .publish_candidate_targets(stamp, base_revision, targets)
                            .expect("the owner's targets were preflighted");
                    }
                }
            }
        }
        // #1313 D2: the path is the classifier's result. A delta with no record of any kind is a
        // commit render has nothing to do for.
        // #1559 D5: a transaction with a model edit is `live` when the model part or the overlay
        // change produced a record, else `model_only`; the commit gives a token of monitor
        // writes only its own path.
        let path = if delta.is_empty() {
            protocol::EditPath::ModelOnly
        } else {
            protocol::EditPath::Live
        };
        // B19-R7: the longest ramp of the following routes' records this commit writes, folded
        // into the send-move window once committed.
        let send_ramp = delta
            .routes
            .iter()
            .filter_map(|route| {
                route
                    .record
                    .following()
                    .map(host_core::FollowRouteControlRecord::length)
            })
            .max()
            .unwrap_or(0);
        drop(delta);
        let published = core::mem::take(&mut owners.begun);

        // 7. Commit, then each EQ owner; the catalog's values follow (#1264 D4, #1265 D3);
        // 8. respond.
        let committed = prepared
            .commit(&mut self.controller, path)
            .expect("the commit predicate was checked under this borrow");
        debug_assert_eq!(
            revision,
            self.controller.session().revision().0,
            "the revision stamped is the revision committed (#1312 D7)"
        );
        // #1559 D8: the overlay is written after the last fallible step, with the commit.
        self.monitor = monitor_change.overlay;
        fold_send_move(&mut self.send_moves, revision, send_ramp);
        for index in published {
            owners.effects[index]
                .commit_owner()
                .expect("the owner's targets were published");
        }
        drop(owners);
        // The catalog of the committed model replaces the current one (decision 17, #1552 D3): the
        // values are read back through the same pass a rebuild uses, so a derived row follows its
        // inputs, and the structure epoch advances if any descriptor changed (a `matrix_mode`
        // switch flips `active`). Nothing here allocates or fails.
        let previous_catalog = self.controller.provider().catalog_resources();
        self.controller
            .provider_mut()
            .publish_staged_catalog(catalog, revision);
        let committed_catalog = self.controller.provider().catalog_resources();
        // The newest epoch's rows follow the committed catalog (#1558 D6): it holds the catalog
        // once in its retained bytes and once in the prepared-protocol stand-in, which later
        // admissions charge, and `plan_resources` reports the retained bytes. An older epoch's row
        // keeps the catalog it was prepared with until its plan is gone. Nothing here fails.
        newest_capi.follow_catalog(previous_catalog, committed_catalog);
        // #1696 root ruling: and the committed model's edit-ramp row, so a VCA added live is
        // charged at once, in the admissions and in the report.
        let previous_ramps = newest_capi.edit_ramps;
        newest_capi.follow_edit_ramps(committed_ramps);
        {
            let mut reports = self
                .shared
                .reports
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some((_, row)) = reports.iter_mut().find(|(epoch, _)| epoch == newest_epoch) {
                let mut resources = PlanResources::from(*row);
                resources.control_retained_bytes = resources
                    .control_retained_bytes
                    .saturating_sub(previous_catalog.retained_bytes)
                    .saturating_add(committed_catalog.retained_bytes)
                    .saturating_sub(previous_ramps)
                    .saturating_add(committed_ramps);
                resources.largest_named_allocation_bytes = resources
                    .largest_named_allocation_bytes
                    .max(committed_catalog.largest_allocation_bytes);
                *row = A::Row::from(resources);
            }
        }
        // #1310: the kept base's lengths, built above, now that the edit is committed.
        if let Some(kept) = self
            .kept_holder_mut()
            .and_then(|holder| holder.kept.as_mut())
        {
            if let Some(ramps) = kept_ramps {
                kept.edit_ramps = ramps;
            }
            if kept_monitor_ramp.is_some() {
                kept.monitor_ramp = kept_monitor_ramp;
            }
        }
        // #1584 D4: the subscriptions' records, admitted above with no observation record queued
        // since, then the table the edits left, before the reply exists. A record cannot be refused
        // here: the control thread is the queues' only producer. #1968: they are pushed before the
        // revision is published below, on the queues of the plan whose gate publishes it, so a
        // block whose snapshot covers the revision drains them in the same block (the drain pops
        // the queue after the snapshot), and a watermark that covers the revision means the tap
        // is armed. Render drains the queues outside the gate, so a record may apply one block
        // early: a subscription has no revision (CONTROL_PROTOCOL_SEMANTICS.md, "Observation
        // subscriptions"), so it is not one of the transaction's live values, and it applies at
        // whatever block drains next, as an `Observe`-only transaction's does. So the first window
        // may open one block before the revision's values apply; its `first_sample` says so, and a
        // reader that needs the changed audio takes the windows whose `first_sample` is at or
        // after the watermark's.
        if !observe_changes.is_empty() {
            self.push_tagged_observation_batch(observe_changes.iter().copied());
        }
        if let Some(bindings) = observe_bindings {
            self.observation = bindings;
        }
        if let Some(next) = next_arm_tag_after {
            self.next_arm_tag = next;
        }
        // 8. The revision, as the commit's last write (#1314 D2): published on the gate of the plan
        // that carries its content, the newest epoch's (#1502 D6), which is the routing the
        // producers were chosen by, so the cells a commit writes and the gate that reveals them
        // are one plan's. Every record push (the observation records above included, #1968), cell
        // write, target publication and owner commit is before it, so render's `Acquire` snapshot
        // of the gate sees them all and the watermark never reports the revision before its values
        // can apply. The stamp's revision is published, read into a local, so the always-on bound
        // of `stamp` covers the publication. A monitor publication carries the committed revision
        // (#1603). The observation pushes borrowed the session, so the newest epoch's gate is
        // borrowed again: the same epoch, since nothing in between adds or promotes one.
        if let Some(key) = key {
            self.pending_providers
                .last_mut()
                .unwrap_or(&mut self.providers)
                .revision_gate
                .publish_key(key);
        }
        // #1690 D2: the master this commit leaves, by the `sync_master` rule.
        sync_master_meters(
            &self.monitor,
            &mut self.providers,
            &mut self.pending_providers,
            &mut self.meters,
        );
        LiveCommit::Done(Ok(committed.write_into(&mut self.response_scratch).expect(
            "prepared response capacity was admitted before the live commit",
        )))
    }

    /// The live admission's inputs, read from the report table and the epochs (#1053 D8). It
    /// returns the catalog of the committed model, which it built to charge (#1558 D6).
    fn live_admission(
        &self,
        prepared: &ObservedPreparedToken,
        overlay: &controls::MonitorOverlay,
        view: controls::ModelView<'_>,
        delta: &host_core::LiveDelta<'_>,
        committed_ramps: u64,
    ) -> Result<host_core::StagedCatalog, CommandError> {
        let transaction = prepared.get().prospective_session();
        let committed = self.controller.session().compiled();
        let models = if transaction.is_in_place() {
            // #1613: a transaction prepared in place changes values and at most adds parameter
            // entries (#1620), never a count a rebuild checks (the preparation proved it), so those
            // counts are the committed model's, which passed them when it was admitted. The model
            // charge is the one the full path would make: the committed estimate and the next one,
            // which charges the added entries.
            let next = transaction
                .in_place_estimate()
                .unwrap_or_else(|| committed.resource_estimate());
            compiled_model_admission_over(self.held_models()?.as_slice(), next)
                .map_err(CommandError::CompileRejected)?
        } else {
            // A live delta may add a VCA, so it meets the counts a rebuild would (#1558 D6): the
            // same function, the same diagnostic, and nothing is committed on a refusal.
            host_core::check_counts(view.structure(), &prepare_caps(self.limits)).map_err(
                |diagnostics| CommandError::CompileRejected(prepare_failure(diagnostics)),
            )?;
            compiled_model_admission_over(
                self.held_models()?.as_slice(),
                transaction
                    .compiled()
                    .ok_or(CommandError::Internal)?
                    .resource_estimate(),
            )
            .map_err(CommandError::CompileRejected)?
        };
        // The new catalog, before anything is written. A VCA added live adds rows to it, so it
        // can outgrow the catalog the epochs were prepared with.
        let catalog = self
            .controller
            .provider()
            .stage_live_catalog(view, overlay, view.structure().sample_rate_hz, delta)
            .map_err(|_| CommandError::Internal)?;
        // A values-only stage holds no table: the catalog stays as large as it is, and only a
        // full catalog is held beside the current one until the commit publishes it.
        let current_catalog = self.controller.provider().catalog_resources();
        let mut catalog_admission = LiveCatalogAdmission::between(
            current_catalog,
            match &catalog {
                host_core::StagedCatalog::Values(_) => current_catalog,
                host_core::StagedCatalog::Full(full) => full.resources(),
            },
        );
        // #1696 root ruling: a VCA added live raises the bound of the ramps a kept base holds.
        let newest = self.pending_providers.last().unwrap_or(&self.providers);
        catalog_admission.ramp_growth_bytes =
            committed_ramps.saturating_sub(newest.capi.edit_ramps);
        validate_live_peak(
            self.held_epochs()?.as_slice(),
            models,
            catalog_admission,
            self.limits,
        )
        .map_err(CommandError::CompileRejected)?;
        Ok(catalog)
    }

    /// The response bytes of the last successful command.
    pub fn command_response(&self, bytes: usize) -> &[u8] {
        &self.response_scratch[..bytes]
    }

    /// Encodes the next event of `lane` into the event scratch, and returns its length, or `None`
    /// when the lane is empty.
    pub fn dequeue_event(
        &mut self,
        lane: EventLane,
        output_capacity: u64,
    ) -> Result<Option<usize>, EventError> {
        self.service().map_err(|error| match error {
            CommandError::Backpressure => EventError::Backpressure,
            _ => EventError::Internal,
        })?;
        let capacity = usize::try_from(output_capacity)
            .unwrap_or(usize::MAX)
            .min(self.response_scratch.len());
        let result = match lane {
            EventLane::Reliable => self.dequeue_reliable_event(capacity),
            EventLane::Lossy => self
                .controller
                .dequeue_lossy_event_frame_into(&mut self.response_scratch[..capacity]),
        };
        result.map_err(map_event_egress_error)
    }

    pub(crate) fn dequeue_reliable_event(
        &mut self,
        output_capacity: usize,
    ) -> Result<Option<usize>, EventEgressError> {
        if self.render_diagnostic_len == 0 {
            let result = self
                .controller
                .dequeue_reliable_event_frame_into(&mut self.response_scratch[..output_capacity]);
            self.protocol_reliable_pending = matches!(
                result,
                Err(EventEgressError::Encode(EncodeError::OutputTooSmall { .. }))
            );
            return result;
        }

        let head = self.render_diagnostic_head;
        if self.render_diagnostics[head].protocol_events_before != 0 {
            let result = self
                .controller
                .dequeue_reliable_event_frame_into(&mut self.response_scratch[..output_capacity]);
            match result {
                Ok(Some(bytes)) => {
                    self.render_diagnostics[head].protocol_events_before -= 1;
                    self.protocol_reliable_pending = false;
                    return Ok(Some(bytes));
                }
                Ok(None) => {
                    self.protocol_reliable_pending = false;
                    return Ok(None);
                }
                Err(error) => {
                    self.protocol_reliable_pending = matches!(
                        error,
                        EventEgressError::Encode(EncodeError::OutputTooSmall { .. })
                    );
                    return Err(error);
                }
            }
        }

        let slot = &mut self.render_diagnostics[head];
        let result = ProtocolCodec::default().encode_event_frame_into(
            &protocol::TypedEventFrame {
                revision: slot.revision,
                payload: protocol::EventPayload::Diagnostic(&slot.diagnostic),
            },
            &mut self.response_scratch[..output_capacity],
        );
        match result {
            Ok(bytes) => {
                drop(slot.reservation.take());
                slot.diagnostic.sample_time = None;
                slot.diagnostic.provider_sequence = None;
                slot.protocol_events_before = 0;
                slot.occupied = false;
                self.controller
                    .provider_mut()
                    .set_render_diagnostic(head, 0, 0, false);
                self.render_diagnostic_head =
                    (self.render_diagnostic_head + 1) % self.render_diagnostics.len();
                self.render_diagnostic_len -= 1;
                Ok(Some(bytes))
            }
            Err(error) => Err(EventEgressError::Encode(error)),
        }
    }

    /// The bytes of the last dequeued event.
    pub fn event_response(&self, bytes: usize) -> &[u8] {
        &self.response_scratch[..bytes]
    }

    /// The epoch the newest pending candidate succeeds, which keeps the committed base a
    /// replacement of that candidate is prepared from (#1310 D3): the epoch before the newest, or
    /// `None` while nothing is pending.
    fn kept_holder(&self) -> Option<&ProviderEpoch> {
        match self.pending_providers.len() {
            0 => None,
            1 => Some(&self.providers),
            len => self.pending_providers.get(len - 2),
        }
    }

    /// [`Self::kept_holder`], mutably.
    fn kept_holder_mut(&mut self) -> Option<&mut ProviderEpoch> {
        match self.pending_providers.len() {
            0 => None,
            1 => Some(&mut self.providers),
            len => self.pending_providers.get_mut(len - 2),
        }
    }

    /// The producers of the newest committed session: the pending candidate's while one waits
    /// for its swap, else the current plan's (issue #1273 D3).
    pub(crate) fn newest_providers(&self) -> &ProviderEpoch {
        self.pending_providers.last().unwrap_or(&self.providers)
    }

    /// Whether the control plane still holds the plan of `epoch`: the running plan's, or a pending
    /// candidate's (#1310, batch 15a review fold B1). A candidate a later structural edit
    /// superseded is dropped with that edit's commit, so a host that keeps tables per plan drops
    /// the superseded plan's when this turns false, on the core's word, never its own.
    pub fn holds_plan_epoch(&self, epoch: u64) -> bool {
        self.providers.epoch == epoch
            || self
                .pending_providers
                .iter()
                .any(|provider| provider.epoch == epoch)
    }

    /// The index of source `id` in the graph source set of the plan of epoch `epoch`, as that
    /// plan's state inventory records it, or `None` when the plan has no such source or the
    /// control plane holds no epoch `epoch` (#1325). A host that owns the render plan on its own
    /// thread resolves a source against the plan that renders with it: while a successor waits,
    /// and through a strip removal's phase 1, that plan's source set is not the newest committed
    /// session's.
    pub fn rendering_source_index(&self, epoch: u64, id: &[u8]) -> Option<usize> {
        let id = core::str::from_utf8(id).ok()?;
        core::iter::once(&self.providers)
            .chain(&self.pending_providers)
            .chain(&self.retired_providers)
            .find(|provider| provider.epoch == epoch)?
            .inventory
            .source(id)
            .map(|row| row.index as usize)
    }

    /// Visit each source the plan of `epoch` renders (#2205): its index in that plan's source set
    /// and the frame a session seek to `timeline_sample` sends it, the sample clamped to the
    /// newest session's region of the source. A source the newest session does not hold is
    /// skipped. Control thread; it allocates nothing.
    pub fn for_each_rendering_session_frame(
        &self,
        epoch: u64,
        timeline_sample: u64,
        mut visit: impl FnMut(usize, u64),
    ) {
        let Some(rendering) = core::iter::once(&self.providers)
            .chain(&self.pending_providers)
            .chain(&self.retired_providers)
            .find(|provider| provider.epoch == epoch)
        else {
            return;
        };
        let sources = &self.newest_providers().sources;
        for row in rendering.inventory.sources() {
            if let Some(region) = sources.region(row.id.as_bytes()) {
                visit(
                    row.index as usize,
                    timeline_sample.min(region.end).max(region.start),
                );
            }
        }
    }

    /// Sets the replacement peak ceiling ([`ControlLimits::maximum_replacement_peak_bytes`]) once
    /// the adapter knows its own fixed bytes exactly (#1290a D3): they are known only after the
    /// first plan is prepared, and the limits the session was built with are a plain value.
    pub fn set_replacement_peak_limit(&mut self, bytes: u64) {
        self.limits.maximum_replacement_peak_bytes = bytes;
    }

    /// The bytes the plans and compiled models the control plane holds retain now (issue #2194):
    /// the replacement peak's held terms (`held_peak_bytes`), every held plan's graph and
    /// allocated source rows and every held model. A host adds its own fixed rows to project what
    /// a replacement's parse may grow beside the live session.
    ///
    /// # Errors
    ///
    /// [`CommandError::Internal`] for an accounting fault, [`CommandError::CompileRejected`] for an
    /// arithmetic overflow.
    pub fn held_retained_bytes(&self) -> Result<u64, CommandError> {
        let held = self.held_epochs()?;
        let models = self.held_models()?;
        let models_retained = models
            .as_slice()
            .iter()
            .try_fold(0_u64, |total, model| {
                total.checked_add(model.compiled_model_bytes)
            })
            .ok_or(CommandError::CompileRejected(
                crate::error::CompileFailure::Resource(ResourceFault::Arithmetic),
            ))?;
        held_peak_bytes(held.as_slice(), models_retained).map_err(CommandError::CompileRejected)
    }

    /// The epoch of the newest plan (#1290a): the pending candidate's while one waits for its
    /// swap, else the current plan's. It advances by a published rebuild, so a host that compares
    /// it across a command knows whether the command published a replacement plan, and, against
    /// the epoch of the plan it renders, whether a replacement is still waiting for its block. It
    /// is the epoch of the newest committed plan, whose producers an `Observe` edit reaches.
    #[must_use]
    pub fn newest_plan_epoch(&self) -> u64 {
        self.newest_providers().epoch
    }

    /// The epoch whose producer a source call for `id` goes to (#1325 D4): the running plan's
    /// while a pending successor removes the source and the running plan still renders it (the
    /// duck overlay lists it), so phase 1's ramp plays the PCM the host submits; else the newest
    /// committed session's. One binary search, no allocation and no lock. Once render adopts the
    /// successor, the service step promotes it and the overlay drops with the old epoch's kept
    /// base, so the call is refused as an unknown source, as before.
    fn source_providers_mut(&mut self, id: &[u8]) -> &mut ProviderEpoch {
        // The overlay ends when render adopts the successor: from then on the running epoch's
        // plan no longer renders, even before the service step promotes the successor (the
        // browser's unserviced feed), so a removed source is refused as unknown (D4). One
        // `Acquire` load of the epoch render published, no lock.
        let rendered = self.shared.active_epoch.load(Ordering::Acquire);
        let ducked = self.kept_holder().is_some_and(|holder| {
            rendered <= holder.epoch
                && holder
                    .kept
                    .as_ref()
                    .is_some_and(|kept| kept.duck.has_source(id))
        });
        if ducked {
            return self.kept_holder_mut().expect("a kept base has a holder");
        }
        self.newest_providers_mut()
    }

    fn newest_providers_mut(&mut self) -> &mut ProviderEpoch {
        self.pending_providers
            .last_mut()
            .unwrap_or(&mut self.providers)
    }

    /// The committed session in its compiled form (#1381a): what the plan was prepared from. A
    /// single-instance adapter reads its addressing tables and source shapes from it, so it keeps
    /// no second copy of the model beside the control plane's own.
    #[must_use]
    pub fn committed_session(&self) -> &CompiledSession {
        self.controller.session().compiled()
    }

    /// The structure epoch of the committed control list (#1562b): the number the `CONTROL_LIST_GET`
    /// pages carry. It advances when a commit changes an address, a header or a descriptor, a
    /// rebuild that adds or removes a strip included. An adapter that publishes a plan's strip
    /// order beside its meter frames stamps the plan with the epoch the commit left, so a client
    /// that holds a list of another epoch knows to read it again.
    #[must_use]
    pub fn structure_epoch(&self) -> u64 {
        self.controller.provider().structure_epoch()
    }

    /// What the session's one control catalog holds (#1381a): the control plane's provider owns it,
    /// and an adapter that charges its own report reads the figures here, not from a catalog of its
    /// own.
    #[must_use]
    pub fn catalog_resources(&self) -> host_core::CatalogResources {
        self.controller.provider().catalog_resources()
    }

    /// The newest plan's effect producers, in `HostLiveControlHandles::effect_controls` order
    /// (#1381a): the pending candidate's while one waits for its swap, else the current plan's.
    /// Test support (#1590): no host reaches a producer; the core's `Observe` edit queues the
    /// records.
    #[cfg(feature = "test-support")]
    #[must_use]
    pub fn test_effect_producers(&self) -> &[host_core::EffectControlProducer] {
        &self.newest_providers().effects
    }

    /// Resolves the stable key of an observation tap on the newest committed plan (#1677), the
    /// pending candidate's while one waits for its swap. An `Observe` edit names its tap by this
    /// key and no position of any plan, so a rebuild between a client's last read and the record
    /// cannot make it arm another effect's tap.
    ///
    /// The strip is looked up in the committed model, so a strip with no effect is still an
    /// unknown effect and not an unknown strip. The effect is the producer of the strip's console
    /// slot or insert of that ID. The comparison is by bytes and the call allocates nothing.
    ///
    /// # Errors
    ///
    /// The first of [`ObservationKeyError`] that applies, in the order strip, effect, tap. Nothing
    /// is written.
    pub(crate) fn resolve_observation_key(
        &self,
        key: &ObservationKeyRef<'_>,
    ) -> Result<ResolvedObservationTap, ObservationKeyError> {
        let model = self.committed_session().normalized_model();
        let strip_known = model
            .tracks
            .binary_search_by(|track| track.id.as_str().as_bytes().cmp(key.strip_id))
            .is_ok()
            || model
                .submixes
                .binary_search_by(|submix| submix.id.as_str().as_bytes().cmp(key.strip_id))
                .is_ok();
        if !strip_known {
            return Err(ObservationKeyError::UnknownStrip);
        }
        let (producer, found) = self
            .newest_providers()
            .effects
            .iter()
            .enumerate()
            .find(|(_, producer)| {
                producer.address.rack == key.rack
                    && producer.track_id.as_bytes() == key.strip_id
                    && producer.effect_id.as_bytes() == key.effect_id
            })
            .ok_or(ObservationKeyError::UnknownEffect)?;
        Ok(ResolvedObservationTap {
            producer,
            tap_index: resolve_observation_tap(found.descriptor, key.tap_id)?,
        })
    }

    /// The tag of the last record this session queued for the tap `key` names on the newest plan,
    /// or of the binding a prepared arm publishes there, `0` for none or for a key the plan does
    /// not have (#1585 D4). Tags are monotone, so a reader whose slot publishes a lower tag knows
    /// render has not applied the tap's last record yet. Allocates nothing.
    #[must_use]
    pub fn observation_issued_tag(&self, key: &ObservationKeyRef<'_>) -> u64 {
        self.observation_issued(key).0
    }

    /// [`Self::observation_issued_tag`] and the kind of that record: `true` for an arm or a
    /// re-arm (or a binding's prepared arm), `false` for a disarm or for none (#1585, root ruling
    /// on #1977). A disarm is in effect for a reader at once: no later window belongs to the
    /// subscription it ends. Allocates nothing.
    #[must_use]
    pub fn observation_issued(&self, key: &ObservationKeyRef<'_>) -> (u64, bool) {
        self.resolve_observation_key(key)
            .ok()
            .and_then(|tap| self.observation_slot(tap.producer, tap.tap_index))
            .and_then(|slot| self.observation_pending.get(slot))
            .map_or((0, false), |entry| (entry.tag, entry.armed))
    }

    /// The encoded bytes of the one `OBSERVATION_ENDED` frame that would end every armed tap,
    /// pushes the table has not folded yet included, or `0` when none is armed (#1685 D1). A
    /// subscription is admitted only while this is at most [`Self::observation_frame_bytes`].
    #[must_use]
    pub(crate) fn observation_ended_bytes(&self) -> usize {
        self.observation_wire
            .event_bytes(self.observation_armed_bytes)
    }

    /// The control frame limit the ended event must fit (#1685 D1), the session's
    /// `maximum_control_frame_bytes`.
    #[must_use]
    pub(crate) fn observation_frame_bytes(&self) -> usize {
        self.observation_wire.frame()
    }

    /// Test support (#1590): `resolve_observation_key`, which no host calls.
    ///
    /// # Errors
    ///
    /// As `resolve_observation_key`.
    #[cfg(feature = "test-support")]
    pub fn test_resolve_observation_key(
        &self,
        key: &ObservationKeyRef<'_>,
    ) -> Result<ResolvedObservationTap, ObservationKeyError> {
        self.resolve_observation_key(key)
    }

    /// Test support (#1590): `observation_ended_bytes`, which no host calls.
    #[cfg(feature = "test-support")]
    #[must_use]
    pub fn test_observation_ended_bytes(&self) -> usize {
        self.observation_ended_bytes()
    }

    /// Test support (#1590): `observation_frame_bytes`, which no host calls.
    #[cfg(feature = "test-support")]
    #[must_use]
    pub fn test_observation_frame_bytes(&self) -> usize {
        self.observation_frame_bytes()
    }

    /// How many taps are armed now, pushes the table has not folded yet included.
    #[must_use]
    pub fn observation_armed(&self) -> usize {
        self.observation_armed
    }

    /// Admits a batch of subscription changes with nothing queued (#1676 D1, attempt 2; #1584 D4):
    /// the core's `Observe` edit, which must finish other checks before the first write, admits its
    /// net changes here and pushes them after, with nothing pushed to any observation queue in
    /// between. A change names the producer's position in the newest plan's effect producers and
    /// the tap, as [`Self::resolve_observation_key`] returns them.
    ///
    /// Three checks run before anything is written, in this order, and a refusal names the first
    /// change in the iterator's order that breaks the first check that fails:
    ///
    /// 1. **A slot per tap, and one naming per tap.** A change whose producer has no observation
    ///    slot for its tap (a tap index past the menu, or a producer with no observation queue) is
    ///    [`ObservationRefusal::Unbound`]. A change that names a tap an earlier change of the
    ///    batch named is [`ObservationRefusal::Duplicate`], at the second naming. The frame check
    ///    below counts each tap once, so it is only sound for a batch that names each tap once.
    /// 2. **The frame.** The batch is judged by the state it leaves: arms of taps that are not
    ///    armed add their key's encoded bytes, disarms of armed taps remove theirs, and the rest
    ///    change nothing (#1685 D1). A batch that would leave armed taps whose one
    ///    `OBSERVATION_ENDED` event, every key with its real IDs, outgrows the control frame
    ///    ([`Self::observation_frame_bytes`]) is [`ObservationRefusal::Limit`], at the arm that
    ///    crosses after every removal of the batch is counted. A retry of the same batch never
    ///    succeeds until another tap is disarmed.
    /// 3. **The room.** Each producer's observation queue holds one record per declared tap until
    ///    a render drains it; a batch that needs more than the free room of a producer is
    ///    [`ObservationRefusal::Full`] (retryable after a render), and a position past the list is
    ///    [`ObservationRefusal::NoProducer`]. These two are one check: the first change in order
    ///    that names no producer or a producer without the room the batch needs is refused.
    ///
    /// A refusal writes nothing observable, so it is whole and precedes every acknowledgement. The
    /// check is linear in the batch and reads the per-slot state the pushes keep and per-producer
    /// scratch sized with the plan; it allocates nothing itself (the edit that calls it builds its
    /// list of changes on the control thread).
    ///
    /// # Errors
    ///
    /// [`ObservationRefusal`], as above. Nothing was queued and no state moved.
    pub(crate) fn admit_observation_batch(
        &mut self,
        changes: impl Iterator<Item = ObservationChange> + Clone,
    ) -> Result<(), ObservationRefusal> {
        let mark = self.next_observation_batch_mark();
        // Pass 1: a tap the producer has no slot for, the second naming of a tap, and each
        // producer's record count for pass 3. A position past the list is pass 3's `NoProducer`.
        for (index, change) in changes.clone().enumerate() {
            if change.position < self.newest_providers().effects.len()
                && self
                    .observation_slot(change.position, change.tap_index)
                    .is_none()
            {
                return Err(ObservationRefusal::Unbound { index });
            }
            if let Some(count) = self.observation_batch_counts.get_mut(change.position) {
                if count.mark != mark {
                    *count = ObservationBatchCount { mark, needed: 0 };
                }
                count.needed = count.needed.saturating_add(1);
            }
            if let Some(slot) = self.observation_slot(change.position, change.tap_index) {
                let entry = &mut self.observation_pending[slot];
                if entry.mark == mark {
                    return Err(ObservationRefusal::Duplicate { index });
                }
                entry.mark = mark;
            }
        }
        // Pass 2: the frame.
        self.admit_observation_changes(changes.clone())
            .map_err(ObservationRefusal::Limit)?;
        // Pass 3: the room, one count per producer from pass 1.
        let producers = &self.newest_providers().effects;
        for (index, change) in changes.clone().enumerate() {
            let Some(producer) = producers.get(change.position) else {
                return Err(ObservationRefusal::NoProducer { index });
            };
            let needed = self
                .observation_batch_counts
                .get(change.position)
                .filter(|count| count.mark == mark)
                .map_or(0, |count| count.needed as usize);
            if needed > producer.observe_available_capacity() {
                return Err(ObservationRefusal::Full { index });
            }
        }
        Ok(())
    }

    /// Queues an admitted batch with each record's tag (#1585 D4): the tag a
    /// transaction preparation issued, or `0` for a fresh one from the counter.
    fn push_tagged_observation_batch(
        &mut self,
        changes: impl Iterator<Item = (ObservationChange, u64)>,
    ) {
        for (change, arm_tag) in changes {
            self.push_effect_observe(
                change.position,
                change.tap_index,
                change.armed,
                change.window_blocks,
                arm_tag,
            )
            .expect("the room check found the producer")
            .expect("the room check found room for every record of the batch");
        }
        debug_assert!(
            self.observation_wire.fits(self.observation_armed_bytes),
            "an admitted batch leaves an ended event of {} bytes past the frame of {}",
            self.observation_ended_bytes(),
            self.observation_frame_bytes()
        );
    }

    /// A fresh stamp for the batch about to be admitted. A stamp of `0` is what new scratch
    /// holds, so it is never handed out; when the counter wraps, every mark is cleared first (a
    /// bounded pass over storage sized with the plan, which allocates nothing).
    fn next_observation_batch_mark(&mut self) -> u32 {
        self.observation_batch_mark = self.observation_batch_mark.wrapping_add(1);
        if self.observation_batch_mark == 0 {
            for slot in &mut self.observation_pending {
                slot.mark = 0;
            }
            for count in &mut self.observation_batch_counts {
                *count = ObservationBatchCount::default();
            }
            self.observation_batch_mark = 1;
        }
        self.observation_batch_mark
    }

    /// The index in [`Self::observation_pending`] of the tap at `tap_index` of the producer at
    /// `position`, if both name one.
    fn observation_slot(&self, position: usize, tap_index: u32) -> Option<usize> {
        let first = *self.observation_offsets.get(position)?;
        let end = *self.observation_offsets.get(position + 1)?;
        let slot = first.checked_add(tap_index).filter(|slot| *slot < end)?;
        Some(slot as usize).filter(|slot| *slot < self.observation_pending.len())
    }

    /// The frame check of [`Self::admit_observation_batch`] (#1685 D1): moves nothing and
    /// allocates nothing; each key's bytes are its slot's. Sound only for a batch that names each
    /// tap once, which the caller checks first.
    fn admit_observation_changes(
        &self,
        changes: impl Iterator<Item = ObservationChange> + Clone,
    ) -> Result<(), ObservationLimit> {
        let slot = |change: &ObservationChange| -> Option<&ObservationSlot> {
            self.observation_pending
                .get(self.observation_slot(change.position, change.tap_index)?)
        };
        let removed = changes
            .clone()
            .filter(|change| !change.armed)
            .filter_map(|change| slot(&change).filter(|slot| slot.armed))
            .map(|slot| usize::from(slot.key_bytes))
            .sum::<usize>();
        let mut bytes_after = self.observation_armed_bytes.saturating_sub(removed);
        for (index, change) in changes.enumerate() {
            if let Some(slot) = slot(&change).filter(|slot| change.armed && !slot.armed) {
                bytes_after = bytes_after.saturating_add(usize::from(slot.key_bytes));
                if !self.observation_wire.fits(bytes_after) {
                    return Err(ObservationLimit {
                        index,
                        event_bytes: self.observation_wire.event_bytes(bytes_after),
                        frame_bytes: self.observation_wire.frame(),
                    });
                }
            }
        }
        Ok(())
    }

    /// Queues one observation record past every admission check, the over-frame defence's only
    /// way in (#1676 attempt 2): a test arms taps past the frame to prove that a commit whose
    /// ended event would not fit one frame is still refused.
    ///
    /// # Errors
    ///
    /// As the queue push: `None` for a position past the list, else the producer's refusal.
    #[cfg(feature = "test-support")]
    pub fn test_push_observe_unadmitted(
        &mut self,
        position: usize,
        tap_index: u32,
        armed: bool,
        window_blocks: u32,
    ) -> Option<Result<(), effect_contract::ObservePushError>> {
        self.push_effect_observe(position, tap_index, armed, window_blocks, 0)
    }

    /// Queues one observation record on the newest plan's effect producer at `position`. Private
    /// to the crate: a host reaches it only through the core's `Observe` edit, which admits first,
    /// so no record is queued without the frame check (#1676 attempt 2).
    ///
    /// # Errors
    ///
    /// `None` for a position past the list, else the producer's refusal (no queue, or full).
    ///
    /// `arm_tag` is the tag a transaction preparation issued for the record (#1585 D4), which the
    /// caller installs the counter past when it commits; `0` issues the counter's next tag here.
    pub(crate) fn push_effect_observe(
        &mut self,
        position: usize,
        tap_index: u32,
        armed: bool,
        window_blocks: u32,
        arm_tag: u64,
    ) -> Option<Result<(), effect_contract::ObservePushError>> {
        let producer = self
            .pending_providers
            .last_mut()
            .unwrap_or(&mut self.providers)
            .effects
            .get_mut(position)?;
        // #1585 D4: a fresh record's tag, issued only when the push succeeds.
        let fresh = arm_tag == 0;
        let arm_tag = if fresh { self.next_arm_tag } else { arm_tag };
        let pushed = producer.try_push_observe(tap_index, armed, window_blocks, arm_tag);
        if pushed.is_ok() && fresh {
            self.next_arm_tag = arm_tag.saturating_add(1);
        }
        #[cfg(feature = "test-support")]
        if pushed.is_ok() {
            let newest = self.pending_providers.last().unwrap_or(&self.providers);
            let key = newest.revision_gate.last_published_key();
            TEST_OBSERVE_PUSH_KEY.with(|cell| cell.set(Some(key)));
        }
        // #1310 (root ruling B15-Q4): a record on a plan render has not adopted.
        let on_candidate = !self.pending_providers.is_empty();
        if pushed.is_ok() {
            // #1584 D6a: the push is the acknowledgement, so the state every later successor is
            // prepared from is recorded with it, after the only fallible step and without
            // allocating: this runs on the browser's audio thread, between quanta. A record the
            // lane refused records nothing. The next service step folds it into the table.
            let slots = self
                .observation_offsets
                .get(position)
                .copied()
                .zip(self.observation_offsets.get(position + 1).copied());
            if let Some((first, end)) = slots
                && let Some(slot) = first.checked_add(tap_index).filter(|slot| *slot < end)
                && let Some(entry) = self.observation_pending.get_mut(slot as usize)
            {
                entry.pending = if armed {
                    PendingArm::Arm(window_blocks)
                } else {
                    PendingArm::Disarm
                };
                entry.queued |= on_candidate;
                entry.tag = arm_tag;
                // The count and the bytes follow the slot's state (#1676, #1685): an arm of an
                // armed tap only changes its window, and a disarm of a tap that is not armed
                // removes nothing.
                let key_bytes = usize::from(entry.key_bytes);
                match (entry.armed, armed) {
                    (false, true) => {
                        self.observation_armed += 1;
                        self.observation_armed_bytes += key_bytes;
                    }
                    (true, false) => {
                        self.observation_armed -= 1;
                        self.observation_armed_bytes -= key_bytes;
                    }
                    _ => {}
                }
                entry.armed = armed;
            }
        }
        Some(pushed)
    }

    /// Folds what the host pushed to the observation slots into the table of armed taps, in slot
    /// order, and clears it. Control thread: it allocates for a new binding's key.
    pub(crate) fn fold_observation_pending(&mut self) {
        if self
            .observation_pending
            .iter()
            .all(|slot| slot.pending == PendingArm::None)
        {
            return;
        }
        let producers = &self
            .pending_providers
            .last()
            .unwrap_or(&self.providers)
            .effects;
        for (position, producer) in producers.iter().enumerate() {
            let (Some(&first), Some(&end)) = (
                self.observation_offsets.get(position),
                self.observation_offsets.get(position + 1),
            ) else {
                break;
            };
            for slot in first..end {
                let entry = &mut self.observation_pending[slot as usize];
                let arm = core::mem::take(&mut entry.pending);
                let arm_tag = entry.tag;
                let Some(key) = tap_key_of(producer, slot - first) else {
                    continue;
                };
                match arm {
                    PendingArm::None => {}
                    PendingArm::Arm(window) => self.observation.set_tagged(key, window, arm_tag),
                    PendingArm::Disarm => {
                        self.observation.clear(&key);
                    }
                }
            }
        }
    }

    /// The designated master strip of the newest commit (#1683): the strip whose `master`
    /// monitor row is on, or `None`. Host-local, never saved; a boot designates no master (#1689),
    /// and a `SET_CONTROL` of the row sets or changes it. A host that reports a master reading resolves
    /// it by ID in each plan it holds.
    #[must_use]
    pub fn master_strip(&self) -> Option<&session::StableId> {
        self.monitor.master()
    }

    /// The taps the host has armed (#1584 D6a), for a test or an adapter that reports them.
    #[must_use]
    pub fn observation_bindings(&mut self) -> &controls::ObservationBindings {
        self.fold_observation_pending();
        &self.observation
    }

    /// The queue changes of a transaction's `Observe` records on the newest plan (#1584 D4): each
    /// tap resolved by its stable key ([`Self::resolve_observation_key`]), then the batch admitted
    /// whole ([`Self::admit_observation_batch`]), with nothing queued.
    ///
    /// # Errors
    ///
    /// [`CommandError::Backpressure`] when a queue lacks the room (retryable after a render
    /// drains it); [`CommandError::CompileRejected`] with a resource limit when the armed taps'
    /// ended event would outgrow one control frame, as the browser's record is refused; and
    /// [`CommandError::Internal`] for a key the newest plan does not resolve or a tap with no
    /// queue slot: the committed model is the plan's, and every plan of a host that serves the
    /// edit binds a queue slot for every declared tap.
    fn admit_observe_writes(
        &mut self,
        writes: &[controls::ObserveWrite],
    ) -> Result<Vec<(ObservationChange, u64)>, CommandError> {
        if writes.is_empty() {
            return Ok(Vec::new());
        }
        let mut changes = Vec::new();
        changes
            .try_reserve_exact(writes.len())
            .map_err(|_| CommandError::CompileRejected(failure(ResourceFault::Allocation)))?;
        for write in writes {
            let resolved = self
                .resolve_observation_key(&observation_key_ref(&write.key))
                .map_err(|_| CommandError::Internal)?;
            changes.push((
                ObservationChange {
                    position: resolved.producer,
                    tap_index: resolved.tap_index,
                    armed: write.armed,
                    window_blocks: write.window_blocks,
                },
                write.arm_tag,
            ));
        }
        self.admit_observation_batch(changes.iter().map(|(change, _)| *change))
            .map_err(|refusal| match refusal {
                ObservationRefusal::Full { .. } => CommandError::Backpressure,
                ObservationRefusal::Limit(_) => {
                    CommandError::CompileRejected(failure(ResourceFault::Limit))
                }
                ObservationRefusal::NoProducer { .. }
                | ObservationRefusal::Duplicate { .. }
                | ObservationRefusal::Unbound { .. } => CommandError::Internal,
            })?;
        Ok(changes)
    }

    /// Feeds a source of the newest committed session. A source that persists across a pending
    /// structural transaction is fed through its moved producer into the ring the running plan
    /// still renders and its successor carries (issue #1273 D3).
    ///
    /// Runs the service step first, so it is a control-thread call: [`Self::service`] takes the
    /// shared report lock, drops retired plans and may allocate. A call made while render is
    /// locked uses [`Self::submit_unserviced`].
    pub fn submit(
        &mut self,
        id: &[u8],
        submission: SourceSubmission<'_>,
    ) -> Result<source::SubmitReport, SourceFailure> {
        self.service().map_err(|_| SourceFailure::Internal)?;
        self.submit_unserviced(id, submission)
    }

    /// [`Self::submit`] without the service step: the same routing to the newest committed
    /// session's sources, and nothing else.
    ///
    /// The browser's PCM feed drains its rings inside the AudioWorklet's `process()` and submits
    /// from there, where taking the report lock, dropping a retired plan and staging a counter
    /// snapshot are all forbidden (batch 7 review B1). The path allocates nothing and takes no
    /// lock. The service step still runs, in the host's message handler, before the next control
    /// call, so no state the step maintains is skipped, only deferred.
    pub fn submit_unserviced(
        &mut self,
        id: &[u8],
        submission: SourceSubmission<'_>,
    ) -> Result<source::SubmitReport, SourceFailure> {
        self.source_providers_mut(id)
            .sources
            .submit(id, submission)
            .map_err(SourceFailure::Control)
    }

    /// Queues a seek on a source of the newest committed session, after the service step.
    pub fn seek(
        &mut self,
        id: &[u8],
        generation: u64,
        source_frame: u64,
    ) -> Result<(), SourceFailure> {
        self.service().map_err(|_| SourceFailure::Internal)?;
        self.seek_unserviced(id, generation, source_frame)
    }

    /// [`Self::seek`] without the service step; see [`Self::submit_unserviced`].
    pub fn seek_unserviced(
        &mut self,
        id: &[u8],
        generation: u64,
        source_frame: u64,
    ) -> Result<(), SourceFailure> {
        self.source_providers_mut(id)
            .sources
            .seek(id, generation, source_frame)
            .map_err(SourceFailure::Control)
    }

    /// Queues an anchored seek on a source of the newest committed session: `source_frame` enters
    /// the graph in the block that starts at absolute render sample `anchor_sample` (issue #1275).
    pub fn seek_at(
        &mut self,
        id: &[u8],
        generation: u64,
        source_frame: u64,
        anchor_sample: u64,
    ) -> Result<(), SourceFailure> {
        self.service().map_err(|_| SourceFailure::Internal)?;
        self.source_providers_mut(id)
            .sources
            .seek_at(id, generation, source_frame, anchor_sample)
            .map_err(SourceFailure::Control)
    }

    /// Seeks the timeline and every source of the newest committed session to timeline sample
    /// `timeline_sample` under one new `generation`, all or nothing, after the service step
    /// (#1642 D1, D2): [`host_core::SourceControlSet::seek_session`] on the newest session's
    /// producers, which while a structural transaction's candidate waits for its swap are the
    /// candidate's (its carried producers feed the rings the running plan renders, and a source
    /// it adds has its own new ring). The one core function behind both hosts' session seek.
    ///
    /// **A plain session seek is a declared discontinuity (root ruling B16-R9).** With no
    /// `anchor_sample`, the engine declares the discontinuity in the same call, in this order:
    /// every producer checks its command ([`host_core::SourceControlSet::check_session_seek`]),
    /// then the declaration runs ([`Self::declare_discontinuity`]'s core path, all or nothing),
    /// then the commands are pushed. A refused check declares nothing and pushes nothing; a
    /// refused declaration pushes nothing and returns its own error,
    /// [`SessionSeekFailure::Declaration`]. A host therefore never declares separately before a
    /// session seek.
    ///
    /// **An anchored session seek keeps continuity.** With `anchor_sample`, every command is
    /// anchored there, as [`Self::seek_at`]'s, and nothing is declared: it is the loop and
    /// exact-start form (#1643), and a declaration publishes its successor at the next block,
    /// before the anchor, so the effects would go to rest while the old pass still plays. A host
    /// that wants the floors reset declares at a stop.
    ///
    /// # Errors
    ///
    /// [`SessionSeekFailure::Seek`] with the facade's refusal, or `Internal` for an epoch fault;
    /// [`SessionSeekFailure::Declaration`] with the declaration's refusal (plain form only).
    pub fn session_seek(
        &mut self,
        generation: u64,
        timeline_sample: u64,
        anchor_sample: Option<u64>,
    ) -> Result<(), SessionSeekFailure> {
        self.session_seek_landing(generation, timeline_sample, anchor_sample, false)
    }

    /// [`Self::session_seek`]'s plain form for a seek with no live audio to keep (#2205): a
    /// headless engine or a stopped transport. It is a declared discontinuity as the plain form
    /// is, and lands at the first block that observes it, without waiting for a runway
    /// ([`host_core::SourceControlSet::seek_session_at_once`]).
    ///
    /// # Errors
    ///
    /// As [`Self::session_seek`]'s plain form.
    pub fn session_seek_at_once(
        &mut self,
        generation: u64,
        timeline_sample: u64,
    ) -> Result<(), SessionSeekFailure> {
        self.session_seek_landing(generation, timeline_sample, None, true)
    }

    fn session_seek_landing(
        &mut self,
        generation: u64,
        timeline_sample: u64,
        anchor_sample: Option<u64>,
        at_once: bool,
    ) -> Result<(), SessionSeekFailure> {
        self.service()
            .map_err(|_| SessionSeekFailure::Seek(SourceFailure::Internal))?;
        if anchor_sample.is_none() {
            self.newest_providers()
                .sources
                .check_session_seek(generation, timeline_sample, None)
                .map_err(|error| SessionSeekFailure::Seek(SourceFailure::Control(error)))?;
            self.declare_discontinuity_serviced()
                .map_err(SessionSeekFailure::Declaration)?;
            // The declaration may have superseded a pending candidate, so the newest providers
            // may now be a new epoch's. Its set holds the same producer objects the check ran
            // on: a discontinuity successor carries every source of the committed model, the
            // candidate's model, and its commit moves each producer it carries from the set it
            // supersedes (the checked donation of the candidate's fresh rings, then
            // `adopt_persisting` for the persisting producers and the timeline), so no command
            // was pushed in between and the check's verdict still holds. `seek_session` checks
            // again; a refusal there is that invariant broken.
            let sources = &mut self.newest_providers_mut().sources;
            let pushed = if at_once {
                sources.seek_session_at_once(generation, timeline_sample)
            } else {
                sources.seek_session(generation, timeline_sample, None)
            };
            return pushed.map_err(|_| {
                debug_assert!(false, "a checked session seek holds across its declaration");
                SessionSeekFailure::Seek(SourceFailure::Control(SourceControlError::Chunk(
                    source::HostChunkError::InternalInvariant,
                )))
            });
        }
        self.newest_providers_mut()
            .sources
            .seek_session(generation, timeline_sample, anchor_sample)
            .map_err(|error| SessionSeekFailure::Seek(SourceFailure::Control(error)))
    }

    /// Where a source of the newest committed session landed, as render last published it, after
    /// the service step (#1316 D6, D7): [`host_core::SourceControlSet::seek_report`] of the ring
    /// the newest session's producer feeds, which is the one `seek` queues on.
    pub fn seek_report(&mut self, id: &[u8]) -> Result<source::SourceSeekReport, SourceFailure> {
        self.service().map_err(|_| SourceFailure::Internal)?;
        self.newest_providers()
            .sources
            .seek_report(id)
            .map_err(SourceFailure::Control)
    }

    /// The ring occupancy of a source of the newest committed session, as its producer sees it
    /// (issue #1387): [`host_core::SourceControlSet::admission`] of the ring `submit` feeds. A
    /// pure read: it runs no service step.
    pub fn source_admission(&self, id: &[u8]) -> Result<source::SourceAdmission, SourceFailure> {
        self.newest_providers()
            .sources
            .admission(id)
            .map_err(SourceFailure::Control)
    }
}

/// A source submission or seek that the adapter's boundary must report.
///
/// `Control` carries the facade's typed rejection unchanged (audit F6: the boundary used to
/// collapse every one of the seventeen source failures to `RESULT_INVALID_ARGUMENT` with no
/// diagnostic); `Internal` is the control plane's own epoch-synchronisation failure. The adapter
/// maps each to its own result code and diagnostic (#1309 D6).
#[derive(Clone, Copy, Debug)]
pub enum SourceFailure {
    /// The facade rejected the submission or seek.
    Control(SourceControlError),
    /// The control plane could not synchronise its plan epochs.
    Internal,
}

/// The kind of result a refused source call reports (#1706 D1): each host maps the three kinds to
/// its own result codes, and neither holds the rule that classifies a [`SourceFailure`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceRefusalKind {
    /// A caller mistake: the call is refused and retrying it unchanged is refused again.
    InvalidArgument,
    /// Bounded backpressure: the host retries.
    Backpressure,
    /// An engine invariant or the control plane's epoch synchronisation failed.
    Internal,
}

impl SourceFailure {
    /// The kind of result this refusal reports on every host.
    #[must_use]
    pub const fn kind(self) -> SourceRefusalKind {
        match self {
            Self::Internal => SourceRefusalKind::Internal,
            Self::Control(error) if error.is_backpressure() => SourceRefusalKind::Backpressure,
            Self::Control(error) if error.is_internal() => SourceRefusalKind::Internal,
            Self::Control(_) => SourceRefusalKind::InvalidArgument,
        }
    }

    /// The diagnostic text this refusal reports on every host: the facade's text for a typed
    /// rejection, and `source.epoch` when the plan epochs could not synchronise.
    #[must_use]
    pub const fn diagnostic(self) -> &'static str {
        match self {
            Self::Internal => "source.epoch",
            Self::Control(error) => error.diagnostic(),
        }
    }
}

/// Why a session seek was refused (root ruling B16-R9): the seek's own refusal, or, for the
/// plain form, the refusal of the discontinuity it declares. Either way nothing was pushed. The
/// adapter maps `Seek` as it maps a source seek's [`SourceFailure`] and `Declaration` as it maps
/// its declaration's [`CommandError`].
#[derive(Debug)]
pub enum SessionSeekFailure {
    /// A producer refused the seek, or the epochs could not synchronise; nothing was declared.
    Seek(SourceFailure),
    /// The plain form's declaration refused; nothing was pushed.
    Declaration(CommandError),
}

#[cfg(test)]
mod tests {
    use super::EventError;

    /// #1570a D2: the three event refusal texts are the C ABI's `last_error` bytes, which hosts
    /// match on; a reworded text would silently change every adapter's diagnostic.
    #[test]
    fn event_refusals_spell_the_texts_every_host_reports() {
        assert_eq!(
            EventError::BufferTooSmall { required: 9 }.diagnostic(),
            b"control.event.output.too_small"
        );
        assert_eq!(
            EventError::Backpressure.diagnostic(),
            b"control.event.backpressure"
        );
        assert_eq!(EventError::Internal.diagnostic(), b"control.event.internal");
    }

    /// #1588 D2, D4, R4: a render-half meter fold's frames reach the batches through the plan's
    /// ring, and a frame the full ring dropped reaches `METER_WINDOWS_LOST`.
    #[cfg(feature = "test-support")]
    mod rendered_meters {
        use crate::{
            CompiledChildren, ControlAdapter, ControlLimits, EventLane, PlanResources,
            RuntimePreparer, compile_children,
        };
        use engine::realtime::PlanarBufferMut;
        use host_core::{
            CompiledSession, HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes,
            HostMeterRequest, HostPrepareCaps, LiveValuesSuperseded, PrepareDiagnostics,
            PreparedHost, SuccessorBase, prepare_host_runtime_with_live_lanes,
            prepare_host_runtime_with_live_lanes_successor,
            prepare_host_runtime_with_selected_meters,
            prepare_host_runtime_with_selected_meters_successor,
        };
        use protocol::{
            CommandPayload, CounterId, DiagnosticSeverity, ExpectedRevision, MeterEntry,
            MeterMetric, MeterTarget, ProtocolCodec, RequestId, SessionEdit,
            TelemetryConfiguration, TypedCommandFrame,
        };
        use session::{RouteSource, StableId, parse_session_json};

        const SESSION: &str =
            include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
        const QUANTUM: u64 = 128;

        struct Adapter;

        impl ControlAdapter for Adapter {
            type Row = PlanResources;
            const ADAPTER_ALLOCATIONS: &'static [u64] = &[];
            const METER_DELIVERY: bool = true;
            const RENDER_METER_FOLD: bool = true;
        }

        /// host-core's live-lanes entries, with one-block meter windows: the default meters, or
        /// the caller's `selection`.
        struct Preparer {
            selection: Option<Vec<HostMeterRequest>>,
        }

        impl RuntimePreparer for Preparer {
            fn prepare(
                &self,
                compiled: &CompiledSession,
                caps: &HostPrepareCaps,
                lanes: HostLiveLanes,
                _spectrum: Option<&host_core::SpectrumPreparation>,
                successor: Option<SuccessorBase<'_>>,
                superseded: &LiveValuesSuperseded,
            ) -> Result<
                (
                    PreparedHost,
                    HostLiveControlHandles,
                    Option<host_core::PreparedSpectrumCaptures>,
                ),
                PrepareDiagnostics,
            > {
                let live = HostLiveControlRequest {
                    meter_window_blocks: 1,
                    ..HostLiveControlRequest::default()
                };
                let (host, handles) = match (successor, &self.selection) {
                    (Some(base), None) => prepare_host_runtime_with_live_lanes_successor(
                        compiled, caps, &live, lanes, base, superseded,
                    ),
                    (None, None) => prepare_host_runtime_with_live_lanes(
                        compiled, caps, &live, lanes, superseded,
                    ),
                    (Some(base), Some(selection)) => {
                        prepare_host_runtime_with_selected_meters_successor(
                            compiled, caps, &live, selection, lanes, superseded, base,
                        )
                    }
                    (None, Some(selection)) => prepare_host_runtime_with_selected_meters(
                        compiled, caps, &live, selection, lanes, superseded,
                    ),
                }?;
                Ok((host, handles, None))
            }

            fn meter_window_blocks(&self) -> u32 {
                1
            }
        }

        fn limits() -> ControlLimits {
            ControlLimits {
                source_ring_frames: 1_024,
                maximum_document_bytes: 1_000_000,
                maximum_diagnostic_bytes: 4_096,
                maximum_tracks: 100,
                maximum_sources: 100,
                maximum_routes: 100,
                maximum_effects: 100,
                maximum_graph_session_plus_plan_bytes: 100_000_000,
                maximum_source_total_bytes: 10_000_000,
                maximum_source_overhead_bytes: 10_000_000,
                maximum_effect_state_bytes: 100_000_000,
                maximum_effect_scratch_bytes: 100_000_000,
                maximum_builtin_retained_bytes: 100_000_000,
                maximum_control_retained_bytes: 10_000_000,
                maximum_named_allocation_bytes: 100_000_000,
                maximum_meter_streams: 4_096,
                maximum_meter_items: 1 << 20,
                maximum_meter_bytes: 1 << 26,
                maximum_control_frame_bytes: 4_096,
                maximum_replay_bytes: 8_192,
                maximum_replay_entries: 16,
                maximum_submixes: 0,
                maximum_vcas: 0,
                maximum_vca_reach_pairs: u64::MAX,
                maximum_replacement_peak_bytes: u64::MAX,
            }
        }

        /// A session booted with `selection` (or the default meters) and the meter table `meters`
        /// at one window per batch.
        fn boot(
            selection: Option<Vec<HostMeterRequest>>,
            meters: Vec<MeterEntry>,
        ) -> CompiledChildren<Adapter> {
            let mut children =
                compile_children::<Adapter>(SESSION, limits(), Box::new(Preparer { selection }))
                    .expect("boot");
            command(
                &mut children,
                1,
                CommandPayload::TelemetryConfigure(&TelemetryConfiguration {
                    meters,
                    meter_period_blocks: 1,
                    meter_window_blocks: None,
                    counter_ids: Vec::new(),
                    counter_period_blocks: 0,
                    diagnostics_enabled: false,
                    minimum_diagnostic_severity: DiagnosticSeverity::Info,
                }),
            );
            children.session.service().expect("service");
            children
        }

        fn output_peak() -> MeterEntry {
            MeterEntry {
                target: MeterTarget::Output,
                metric: MeterMetric::Peak,
            }
        }

        /// Run `payload` as request `request`; the response bytes.
        fn command(
            children: &mut CompiledChildren<Adapter>,
            request: u64,
            payload: CommandPayload<'_>,
        ) -> Vec<u8> {
            let codec = ProtocolCodec::default();
            let frame = TypedCommandFrame {
                request_id: RequestId::new(request).expect("nonzero request"),
                expected_revision: ExpectedRevision::Exact(
                    children.session.test_controller().session().revision(),
                ),
                payload,
            };
            let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
            let length = codec
                .encode_command_frame_into(&frame, &mut bytes)
                .expect("typed command");
            let response = children
                .session
                .command(&bytes[..length], 4_096)
                .expect("command");
            children.session.command_response(response).to_vec()
        }

        /// A complete one-window frame of `window` with output peak `[0.5, 0.25]` and `peaks` as
        /// its strips' peaks.
        fn frame(window: u64, peaks: &[[f32; 2]]) -> host_core::MeterFrame {
            let mut frame = host_core::MeterFrame::new(peaks.len()).expect("frame");
            frame.first_sample = window * QUANTUM;
            frame.end_sample = (window + 1) * QUANTUM;
            frame.windows = 1;
            frame.sequence = window + 1;
            frame.generation = 1;
            frame.validity =
                host_core::MeterValidity::COMPLETE.union(host_core::MeterValidity::OUTPUT);
            frame.output_peak = [0.5, 0.25];
            frame.strip_peaks.copy_from_slice(peaks);
            frame
        }

        /// Render block `block` and observe it, then run the service step.
        fn render(children: &mut CompiledChildren<Adapter>, block: u64) {
            let mut samples = [0.0_f32; 2 * QUANTUM as usize];
            let output =
                PlanarBufferMut::try_new(&mut samples, 2, QUANTUM as usize, QUANTUM as usize)
                    .expect("output");
            children
                .plan
                .render(block * QUANTUM, output)
                .expect("render");
            let (left, right) = samples.split_at(QUANTUM as usize);
            children.plan.observe_rendered_block(
                [left, right],
                block * QUANTUM,
                (block + 1) * QUANTUM,
            );
            children.session.service().expect("service");
        }

        /// Every staged `METER_BATCH`, oldest first: its first sample and its records' values.
        fn batches(children: &mut CompiledChildren<Adapter>) -> Vec<(u64, Vec<f32>)> {
            let mut batches = Vec::new();
            while let Some(length) = children
                .session
                .dequeue_event(EventLane::Lossy, 4_096)
                .expect("an event")
            {
                let bytes = children.session.event_response(length).to_vec();
                let mut fields = [0_u16; 64];
                let decoded = ProtocolCodec::default()
                    .decode_typed_event(&bytes, &mut protocol::DecodeScratch::new(&mut fields))
                    .expect("a typed event");
                if let protocol::DecodedEventPayload::MeterBatch(batch) = decoded.payload {
                    let values = (0..usize::from(batch.count))
                        .map(|index| batch.record(index).expect("a record").value)
                        .collect();
                    batches.push((batch.first_sample.0, values));
                }
            }
            batches
        }

        /// The `METER_WINDOWS_LOST` counter, through `COUNTERS_GET`.
        fn windows_lost(children: &mut CompiledChildren<Adapter>, request: u64) -> u64 {
            counter(children, request, CounterId::MeterWindowsLost)
        }

        /// The counter `id`, through `COUNTERS_GET`.
        fn counter(children: &mut CompiledChildren<Adapter>, request: u64, id: CounterId) -> u64 {
            let response = command(
                children,
                request,
                CommandPayload::CountersGet(&protocol::CountersRequest {
                    all: false,
                    ids: vec![id as u32],
                }),
            );
            let mut fields = [0_u16; 64];
            let protocol::DecodedTypedResponseFrame::Success {
                payload: protocol::DecodedSuccessResponsePayload::CounterSnapshot(snapshot),
                ..
            } = ProtocolCodec::default()
                .decode_typed_response(&response, &mut protocol::DecodeScratch::new(&mut fields))
                .expect("counters response")
            else {
                panic!("expected counters")
            };
            snapshot.values[0].value
        }

        /// A structural transaction: a muted copy of the first track and its route.
        fn add_muted_track(children: &mut CompiledChildren<Adapter>, request: u64) {
            let model = parse_session_json(SESSION).expect("fixture");
            let mut track = model.tracks[0].clone();
            track.id = StableId::parse("a-added").expect("track ID");
            track.fader.left_mute = true;
            track.fader.right_mute = true;
            let mut route = model.routes[0].clone();
            route.id = StableId::parse("a-added-main").expect("route ID");
            let RouteSource::Track { track_id, .. } = &mut route.source else {
                panic!("track route")
            };
            *track_id = track.id.clone();
            let edits = [
                SessionEdit::UpsertTrack { track },
                SessionEdit::UpsertRoute { route },
            ];
            command(
                children,
                request,
                CommandPayload::SessionTransactionApply(&edits),
            );
        }

        /// Test value: red if a frame the full ring dropped never reaches `METER_WINDOWS_LOST`
        /// (the browser's client would see a gap in the batches with no count), or if the
        /// control half stages no batch from a plan's ring (the browser would accept a meter
        /// table and serve nothing).
        #[test]
        fn a_full_ring_drop_reaches_meter_windows_lost_and_its_frames_become_batches() {
            let mut children = boot(None, vec![output_peak()]);
            let level = children.session.meter_level().expect("a meter level");
            assert_eq!(level.load(), host_core::MeterLevel::TELEMETRY);
            let epoch = children.session.newest_plan_epoch();
            let mut producer = children
                .session
                .take_meter_frame_producer(epoch)
                .expect("the boot plan's ring");
            let published = (0..=host_core::METER_FRAME_RING_DEPTH as u64)
                .filter(|&window| producer.publish_frame(&frame(window, &[])))
                .count();
            assert_eq!(published, host_core::METER_FRAME_RING_DEPTH);
            render(&mut children, 0);
            assert_eq!(
                batches(&mut children).len(),
                host_core::METER_FRAME_RING_DEPTH,
                "one batch a frame"
            );
            assert_eq!(windows_lost(&mut children, 2), 1);
        }

        /// #1588 D2, D4: the frames of two epochs' rings are staged in window order, oldest
        /// epoch first, and a reclaimed ring's drop stays counted.
        ///
        /// Test value: red if the rings are drained newest epoch first (the successor's windows
        /// would reach the client before the predecessor's), or if a reclaimed ring's drops leave
        /// `METER_WINDOWS_LOST` with its epoch.
        #[test]
        fn two_epochs_rings_are_staged_in_window_order_and_a_reclaimed_drop_stays_counted() {
            let mut children = boot(None, vec![output_peak()]);
            let first = children.session.newest_plan_epoch();
            let mut older = children
                .session
                .take_meter_frame_producer(first)
                .expect("the boot plan's ring");
            add_muted_track(&mut children, 2);
            let second = children.session.newest_plan_epoch();
            assert!(second > first, "the transaction published a successor");
            let mut newer = children
                .session
                .take_meter_frame_producer(second)
                .expect("the successor's ring");
            let depth = host_core::METER_FRAME_RING_DEPTH as u64;
            for window in 0..=depth {
                older.publish_frame(&frame(window, &[]));
            }
            for window in depth + 1..depth + 3 {
                assert!(newer.publish_frame(&frame(window, &[])));
            }
            render(&mut children, 0);
            render(&mut children, 1);
            assert_eq!(
                children.session.test_providers().epoch,
                second,
                "the boot epoch is reclaimed"
            );
            let starts: Vec<u64> = batches(&mut children)
                .into_iter()
                .map(|(first_sample, _)| first_sample)
                .collect();
            let expected: Vec<u64> = (0..depth)
                .chain(depth + 1..depth + 3)
                .map(|window| window * QUANTUM)
                .collect();
            assert_eq!(starts, expected);
            assert_eq!(windows_lost(&mut children, 3), 1, "the boot ring's drop");
        }

        /// #1588 D3, MINOR-2: a strip entry reads its strip's own meter in a plan prepared with a
        /// caller selection: here one meter, on the fourth track, so the frame's strip 0 is
        /// `eq3`'s.
        ///
        /// Test value: red if an entry resolves against the plan's strip order instead of its
        /// meters' (`eq3` would read the frame's strip 3, another meter's slot or none).
        #[test]
        fn a_strip_entry_reads_its_own_meter_under_a_caller_selection() {
            let selection = vec![HostMeterRequest {
                strip_id: "eq3".into(),
                tap: host_core::MeterTap::PostMatrix,
                metrics: host_core::MeterMetricSet::SAMPLE_PEAK,
            }];
            let mut children = boot(
                Some(selection),
                vec![MeterEntry {
                    target: MeterTarget::Strip("eq3".into()),
                    metric: MeterMetric::Peak,
                }],
            );
            let epoch = children.session.newest_plan_epoch();
            let mut producer = children
                .session
                .take_meter_frame_producer(epoch)
                .expect("the boot plan's ring");
            assert!(producer.publish_frame(&frame(0, &[[0.75, 0.5]])));
            render(&mut children, 0);
            assert_eq!(batches(&mut children), vec![(0, vec![0.75, 0.5])]);
        }

        /// #1690b attempt 2 (the browser's `sync_master` rule): with `eq1` the master, a rebuild
        /// waits for adoption, a live `SET_CONTROL` moves the master to `eq2`, and a configure
        /// re-resolves the entries. A frame the plan still rendering published (into its ring;
        /// on the browser the render thread's frames reach the control half after its commits)
        /// reads that plan's own master, `eq1`.
        ///
        /// Test value: red if a frame of the rendering plan resolves the master the row names now
        /// (the replacement's) instead of the master its own epoch held, after the move or after a
        /// configure that re-resolves the entries.
        #[test]
        fn a_master_moved_while_a_rebuild_waits_leaves_the_rendering_plans_master() {
            let master_entry = || MeterEntry {
                target: MeterTarget::Master,
                metric: MeterMetric::GainReduction,
            };
            let mut children = boot(None, vec![master_entry()]);
            let master = |strip: &str| SessionEdit::SetControl {
                address: protocol::control_core::ControlAddress {
                    owner: protocol::control_core::Owner::Strip(
                        StableId::parse(strip).expect("strip ID"),
                    ),
                    section: protocol::control_core::Section::Own,
                    control: 4,
                    lane: protocol::control_core::Lane::Both,
                },
                value: 1.0,
                ramp_samples: None,
                expected_effect: None,
            };
            command(
                &mut children,
                2,
                CommandPayload::SessionTransactionApply(&[master("eq1")]),
            );
            let epoch = children.session.newest_plan_epoch();
            let mut producer = children
                .session
                .take_meter_frame_producer(epoch)
                .expect("the boot plan's ring");
            add_muted_track(&mut children, 3);
            assert!(
                children.session.newest_plan_epoch() > epoch,
                "the rebuild waits"
            );
            command(
                &mut children,
                4,
                CommandPayload::SessionTransactionApply(&[master("eq2")]),
            );
            // `eq1` reduces 3 dB; `eq2` has no reading.
            let reducing = |window: u64| {
                let mut frame = frame(window, &[[0.0; 2]; 9]);
                frame.strip_gain_reduction_db[1] = 3.0;
                frame.strip_gain_reduction_present[1] = true;
                frame
            };
            // A configure while the rebuild waits re-resolves the entries.
            command(
                &mut children,
                5,
                CommandPayload::TelemetryConfigure(&TelemetryConfiguration {
                    meters: vec![master_entry()],
                    meter_period_blocks: 1,
                    meter_window_blocks: None,
                    counter_ids: Vec::new(),
                    counter_period_blocks: 0,
                    diagnostics_enabled: false,
                    minimum_diagnostic_severity: DiagnosticSeverity::Info,
                }),
            );
            assert!(producer.publish_frame(&reducing(0)));
            render(&mut children, 0);
            assert_eq!(batches(&mut children), vec![(0, vec![3.0])]);
        }

        /// #2198: four blocks observed by service steps with no reliable-lane read in between
        /// (the browser's control Worker runs a service step in every `control_event` call, so
        /// a lossy-lane drain observes blocks the worklet renders meanwhile) fill the two
        /// render-diagnostic slots, so the third block's diagnostic waits and the fourth's
        /// replaces it. The two slots keep the newest: once a slot frees, the next service step
        /// stages the newest observed block's diagnostic, so the last block's activity always
        /// arrives, and the replaced block counts in `TELEMETRY_COALESCED`.
        ///
        /// Test value: red if a block observed while both slots are full is lost for good (the
        /// browser's last diagnostic would carry an earlier block's sample, the #2198 flake), if
        /// the first waiting block wins over the newest, or if the coalescing moves no counter.
        #[test]
        fn a_block_observed_while_the_diagnostic_slots_are_full_is_staged_when_one_frees() {
            let mut children = boot(None, Vec::new());
            command(
                &mut children,
                2,
                CommandPayload::TelemetryConfigure(&TelemetryConfiguration {
                    meters: Vec::new(),
                    meter_period_blocks: 0,
                    meter_window_blocks: None,
                    counter_ids: Vec::new(),
                    counter_period_blocks: 0,
                    diagnostics_enabled: true,
                    minimum_diagnostic_severity: DiagnosticSeverity::Info,
                }),
            );
            for block in 0..4 {
                render(&mut children, block);
            }
            let mut samples = Vec::new();
            while let Some(length) = children
                .session
                .dequeue_event(EventLane::Reliable, 4_096)
                .expect("an event")
            {
                let bytes = children.session.event_response(length).to_vec();
                let mut fields = [0_u16; 64];
                let decoded = ProtocolCodec::default()
                    .decode_typed_event(&bytes, &mut protocol::DecodeScratch::new(&mut fields))
                    .expect("a typed event");
                if let protocol::DecodedEventPayload::Diagnostic(diagnostic) = decoded.payload {
                    samples.push(diagnostic.diagnostic.sample_time);
                }
            }
            assert_eq!(
                samples,
                vec![Some(QUANTUM), Some(2 * QUANTUM), Some(4 * QUANTUM)],
                "the two staged blocks, then the newest"
            );
            assert_eq!(
                counter(&mut children, 3, CounterId::TelemetryCoalesced),
                1,
                "the third block's diagnostic was replaced"
            );
        }
    }

    /// #1676 the admission rules an `Observe` edit cannot reach (#1590 moved them here from the
    /// integration tests, where they ran through a public batch entry no host calls): the edit
    /// resolves every key to a producer and a declared tap, and nets each tap to one change, so a
    /// tap with no slot, a position past the producers and a second naming of a tap reach
    /// `admit_observation_batch` only through an engine fault. The edit maps each to
    /// `CommandError::Internal`; these tests hold that the check itself refuses, at the right
    /// index, before anything is queued.
    #[cfg(feature = "test-support")]
    mod observation_admission {
        use crate::{
            CompiledChildren, ControlAdapter, ControlLimits, ObservationChange, ObservationRefusal,
            PlanResources, RuntimePreparer, compile_children,
        };
        use host_core::{
            CompiledSession, HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes,
            HostPrepareCaps, LiveValuesSuperseded, PrepareDiagnostics, PreparedHost, SuccessorBase,
            prepare_host_runtime_with_live_lanes, prepare_host_runtime_with_live_lanes_successor,
        };

        /// Three tracks, `comp1` to `comp3`, each with one compressor insert `comp` (one tap).
        const SESSION: &str =
            include_str!("../../../fixtures/session/v1/compressor-dynamic-bank-observation.json");

        struct Adapter;

        impl ControlAdapter for Adapter {
            type Row = PlanResources;
            const ADAPTER_ALLOCATIONS: &'static [u64] = &[];
            const OBSERVE_EDIT: bool = true;
        }

        /// A preparer that binds a queue slot for every declared tap.
        struct Preparer;

        impl RuntimePreparer for Preparer {
            fn prepare(
                &self,
                compiled: &CompiledSession,
                caps: &HostPrepareCaps,
                lanes: HostLiveLanes,
                _spectrum: Option<&host_core::SpectrumPreparation>,
                successor: Option<SuccessorBase<'_>>,
                superseded: &LiveValuesSuperseded,
            ) -> Result<
                (
                    PreparedHost,
                    HostLiveControlHandles,
                    Option<host_core::PreparedSpectrumCaptures>,
                ),
                PrepareDiagnostics,
            > {
                let live = HostLiveControlRequest {
                    observation_taps: 4,
                    ..HostLiveControlRequest::default()
                };
                let (host, handles) = match successor {
                    Some(base) => prepare_host_runtime_with_live_lanes_successor(
                        compiled, caps, &live, lanes, base, superseded,
                    ),
                    None => prepare_host_runtime_with_live_lanes(
                        compiled, caps, &live, lanes, superseded,
                    ),
                }?;
                Ok((host, handles, None))
            }
        }

        fn limits() -> ControlLimits {
            ControlLimits {
                source_ring_frames: 1_024,
                maximum_document_bytes: 1_000_000,
                maximum_diagnostic_bytes: 4_096,
                maximum_tracks: 100,
                maximum_sources: 100,
                maximum_routes: 100,
                maximum_effects: 100,
                maximum_graph_session_plus_plan_bytes: 1_000_000_000,
                maximum_source_total_bytes: 100_000_000,
                maximum_source_overhead_bytes: 100_000_000,
                maximum_effect_state_bytes: 1_000_000_000,
                maximum_effect_scratch_bytes: 1_000_000_000,
                maximum_builtin_retained_bytes: 1_000_000_000,
                maximum_control_retained_bytes: 100_000_000,
                maximum_named_allocation_bytes: 1_000_000_000,
                maximum_meter_streams: 4_096,
                maximum_meter_items: 1 << 20,
                maximum_meter_bytes: 1 << 26,
                maximum_control_frame_bytes: 65_536,
                maximum_replay_bytes: 262_144,
                maximum_replay_entries: 16,
                maximum_submixes: 0,
                maximum_vcas: 0,
                maximum_vca_reach_pairs: u64::MAX,
                maximum_replacement_peak_bytes: u64::MAX,
            }
        }

        const fn change(position: usize, tap_index: u32) -> ObservationChange {
            ObservationChange {
                position,
                tap_index,
                armed: true,
                window_blocks: 4,
            }
        }

        /// The free room of every producer's observation queue, and the armed count: what a refused
        /// admission must leave as it was.
        fn state(children: &CompiledChildren<Adapter>) -> (Vec<usize>, usize) {
            (
                children
                    .session
                    .test_effect_producers()
                    .iter()
                    .map(host_core::EffectControlProducer::observe_available_capacity)
                    .collect(),
                children.session.observation_armed(),
            )
        }

        /// Test value: red if admission takes a tap index past the producer's menu (the record
        /// would be queued and never applied), takes a position past the producers, or lets a
        /// batch name one tap twice (the frame check counts each tap once, so `[disarm A, arm A,
        /// arm B]` with the frame full would admit an event over it), or if a refusal names
        /// another change or moves the queues or the armed count.
        #[test]
        fn a_tap_with_no_slot_a_missing_producer_and_a_second_naming_are_refused() {
            let mut children =
                compile_children::<Adapter>(SESSION, limits(), Box::new(Preparer)).expect("boot");
            let producers = children.session.test_effect_producers().len();
            let menu =
                u32::try_from(children.session.test_effect_producers()[0].observe_capacity())
                    .expect("a small menu");
            let before = state(&children);
            let session = &mut children.session;
            assert_eq!(
                session.admit_observation_batch([change(0, 0), change(0, menu)].into_iter()),
                Err(ObservationRefusal::Unbound { index: 1 }),
                "the tap past the menu, at its own index"
            );
            assert_eq!(
                session.admit_observation_batch([change(0, 0), change(producers, 0)].into_iter()),
                Err(ObservationRefusal::NoProducer { index: 1 }),
                "the position past the producers"
            );
            assert_eq!(
                session.admit_observation_batch(
                    [change(0, 0), change(1, 0), change(0, 0)].into_iter()
                ),
                Err(ObservationRefusal::Duplicate { index: 2 }),
                "the second naming of a tap"
            );
            assert_eq!(
                state(&children),
                before,
                "a refusal queues and moves nothing"
            );
            // The marks of the refused batches do not leak: the same taps named once are admitted.
            assert_eq!(
                children
                    .session
                    .admit_observation_batch([change(0, 0), change(1, 0)].into_iter()),
                Ok(())
            );
        }
    }

    /// #1396 gate 5: a declaration resets a source-read offset that no raised floor goes with.
    #[cfg(feature = "test-support")]
    mod source_read_offset {
        use crate::{
            CompiledChildren, ControlAdapter, ControlLimits, PlanResources, RuntimePreparer,
            compile_children,
        };
        use engine::realtime::PlanarBufferMut;
        use host_core::{
            CompiledSession, HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes,
            HostPrepareCaps, LiveValuesSuperseded, PrepareDiagnostics, PreparedHost, SuccessorBase,
            prepare_host_runtime_with_live_lanes, prepare_host_runtime_with_live_lanes_successor,
        };
        use protocol::{
            CommandPayload, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit,
            TypedCommandFrame,
        };
        use session::{RouteSource, StableId, parse_session_json};

        /// 48 kHz, quantum 128.
        const SESSION: &str =
            include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
        const QUANTUM: u64 = 128;
        const OFFSET: u64 = 3 * QUANTUM;

        struct Adapter;

        impl ControlAdapter for Adapter {
            type Row = PlanResources;
            const ADAPTER_ALLOCATIONS: &'static [u64] = &[];
        }

        /// The C ABI's own preparation: host-core's live-lanes entries.
        struct Preparer;

        impl RuntimePreparer for Preparer {
            fn prepare(
                &self,
                compiled: &CompiledSession,
                caps: &HostPrepareCaps,
                lanes: HostLiveLanes,
                _spectrum: Option<&host_core::SpectrumPreparation>,
                successor: Option<SuccessorBase<'_>>,
                superseded: &LiveValuesSuperseded,
            ) -> Result<
                (
                    PreparedHost,
                    HostLiveControlHandles,
                    Option<host_core::PreparedSpectrumCaptures>,
                ),
                PrepareDiagnostics,
            > {
                let live = HostLiveControlRequest::default();
                let (host, handles) = match successor {
                    Some(base) => prepare_host_runtime_with_live_lanes_successor(
                        compiled, caps, &live, lanes, base, superseded,
                    ),
                    None => prepare_host_runtime_with_live_lanes(
                        compiled, caps, &live, lanes, superseded,
                    ),
                }?;
                Ok((host, handles, None))
            }
        }

        fn limits() -> ControlLimits {
            ControlLimits {
                source_ring_frames: 1_024,
                maximum_document_bytes: 1_000_000,
                maximum_diagnostic_bytes: 4_096,
                maximum_tracks: 100,
                maximum_sources: 100,
                maximum_routes: 100,
                maximum_effects: 100,
                maximum_graph_session_plus_plan_bytes: 100_000_000,
                maximum_source_total_bytes: 10_000_000,
                maximum_source_overhead_bytes: 10_000_000,
                maximum_effect_state_bytes: 100_000_000,
                maximum_effect_scratch_bytes: 100_000_000,
                maximum_builtin_retained_bytes: 100_000_000,
                maximum_control_retained_bytes: 10_000_000,
                maximum_named_allocation_bytes: 100_000_000,
                // One meter stream per strip (#1581 D2): generous for every test session.
                maximum_meter_streams: 4_096,
                maximum_meter_items: 1 << 20,
                maximum_meter_bytes: 1 << 26,
                maximum_control_frame_bytes: 4_096,
                maximum_replay_bytes: 8_192,
                maximum_replay_entries: 16,
                maximum_submixes: 0,
                maximum_vcas: 0,
                maximum_vca_reach_pairs: u64::MAX,
                maximum_replacement_peak_bytes: u64::MAX,
            }
        }

        /// A structural transaction that raises no floor: a muted copy of the first track and
        /// its route, so every natural arrival stays where it was.
        fn add_muted_track(children: &mut CompiledChildren<Adapter>) {
            let model = parse_session_json(SESSION).expect("fixture");
            let mut track = model.tracks[0].clone();
            track.id = StableId::parse("a-added").expect("track ID");
            track.fader.left_mute = true;
            track.fader.right_mute = true;
            let mut route = model.routes[0].clone();
            route.id = StableId::parse("a-added-main").expect("route ID");
            let RouteSource::Track { track_id, .. } = &mut route.source else {
                panic!("track route")
            };
            *track_id = track.id.clone();
            let edits = [
                SessionEdit::UpsertTrack { track },
                SessionEdit::UpsertRoute { route },
            ];
            let codec = ProtocolCodec::default();
            let frame = TypedCommandFrame {
                request_id: RequestId::new(1).expect("nonzero request"),
                expected_revision: ExpectedRevision::Exact(
                    children.session.test_controller().session().revision(),
                ),
                payload: CommandPayload::SessionTransactionApply(&edits),
            };
            let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
            let length = codec
                .encode_command_frame_into(&frame, &mut bytes)
                .expect("typed command");
            children
                .session
                .command(&bytes[..length], 4_096)
                .expect("structural transaction");
        }

        /// Render the next block, then run the service step.
        fn render(children: &mut CompiledChildren<Adapter>, start: u64) {
            let mut samples = [0.0_f32; 2 * QUANTUM as usize];
            let output =
                PlanarBufferMut::try_new(&mut samples, 2, QUANTUM as usize, QUANTUM as usize)
                    .expect("output");
            children.plan.render(start, output).expect("render");
            children.session.service().expect("service");
        }

        /// One run: the running plan has no raised floor and offset `offset`; then declare.
        /// Returns the plan epochs the declaration published.
        fn declare_after(offset: u64) -> u64 {
            super::super::test_reset_lifecycle_observer();
            let mut children =
                compile_children::<Adapter>(SESSION, limits(), Box::new(Preparer)).expect("boot");
            if offset > 0 {
                children.session.test_set_next_source_read_offset(offset);
            }
            add_muted_track(&mut children);
            render(&mut children, 0);
            let inventory = children.session.test_providers().test_inventory();
            assert!(inventory.raised_arrivals().is_empty(), "no floor is raised");
            assert_eq!(inventory.source_read_offset(), offset);
            assert_eq!(children.session.test_render_clock(), QUANTUM);
            assert_eq!(children.session.test_source_read_clock(), QUANTUM + offset);

            let epoch = children.session.newest_plan_epoch();
            children
                .session
                .declare_discontinuity()
                .expect("declaration");
            let published = children.session.newest_plan_epoch() - epoch;
            render(&mut children, QUANTUM);
            assert_eq!(
                children
                    .session
                    .test_providers()
                    .test_inventory()
                    .source_read_offset(),
                0
            );
            assert_eq!(children.session.test_render_clock(), 2 * QUANTUM);
            assert_eq!(
                children.session.test_source_read_clock(),
                children.session.test_render_clock()
            );
            published
        }

        /// Test value: red if the declaration tests floors only (a plan that reads ahead with no
        /// raised floor publishes nothing, so `ΣP` never resets and the source-read clock stays
        /// ahead), or if a discontinuity successor inherits its predecessor's offset.
        #[test]
        fn a_declaration_resets_an_offset_with_no_raised_floor() {
            assert_eq!(declare_after(OFFSET), 1, "the offset alone rebuilds");
            assert_eq!(declare_after(0), 0, "nothing to reset (#1323 gate 3)");
        }
    }

    /// #1403 gates: a latency growth classified and published `Primed` through the control
    /// plane, at 48 kHz, quantum 128, on this build's backend. A is two tracks, `eq0` on
    /// [`SOURCE`] and `eq1` on [`AUX`], every console section and insert empty; every ring is
    /// 7,296 frames (`stall_ring_frames` 5,120, plus `p_max_samples` 2,048, plus one quantum).
    /// The growth W adds a muted track with a true-peak limiter (`P = 512`, four blocks). "A
    /// continued" is A rendered on with no growth and the same live commands, bit for bit.
    #[cfg(feature = "test-support")]
    mod warm_successor {
        use super::super::{CommandError, SourceFailure, TransitionStep};
        use crate::EventLane;
        use crate::{
            CompiledChildren, ControlAdapter, ControlLimits, PlanResources, RuntimePreparer,
            compile_children_warm,
        };
        use engine::realtime::{
            OUTCOME_EXACT, OUTCOME_SUPERSEDED, OUTCOME_TRANSITION_FALLBACK, PlanAdoption,
            PlanarBufferMut, RenderError,
        };
        use host_core::{
            CompiledSession, HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes,
            HostPrepareCaps, LiveValuesSuperseded, PrepareDiagnostics, PreparedHost,
            PrimedCandidate, SourceControlError, SourceSubmission, SuccessorBase, WarmConfig,
            prepare_host_runtime_with_live_lanes, prepare_host_runtime_with_live_lanes_successor,
        };
        use protocol::{
            CommandPayload, DecodeScratch, DecodedSuccessResponsePayload,
            DecodedTypedResponseFrame, EditPath, ExpectedRevision, ProtocolCodec, RequestId,
            SessionEdit, TypedCommandFrame,
        };
        use session::{
            ControlAddress, Effect, EffectIdentity, EffectQuality, LinkMode, Owner,
            ParameterChannel, RouteSource, Section, SendTap, SessionModel, SidechainDeclaration,
            Source, StableId, Track, parse_session_json,
        };

        const SESSION: &str =
            include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
        const SOURCE: &str = "fixture-source";
        const AUX: &str = "aux-source";
        const UNUSED: &str = "unused-source";
        const QUANTUM: u64 = 128;
        const RING: u32 = 7_296;
        /// `P` of the one-limiter growth.
        const LEAD: u64 = 512;
        /// Blocks rendered before an edit.
        const BEFORE: usize = 6;
        /// `prime_deadline_samples(48_000, 128)` (#1358 D1): one stall tolerance in whole quanta,
        /// 4,864, plus `P_MAX` 2,048, plus one quantum.
        const DEADLINE: u64 = 7_040;
        /// How far ahead a ready host feeds: `P / q + 1` blocks, and one more.
        const AHEAD: u64 = LEAD / QUANTUM + 2;
        const CONFIG: WarmConfig = WarmConfig {
            p_max: 2_048,
            prime_bytes_max: u64::MAX,
        };

        struct Adapter;

        impl ControlAdapter for Adapter {
            type Row = PlanResources;
            const ADAPTER_ALLOCATIONS: &'static [u64] = &[];
        }

        /// The C ABI's own preparation: host-core's live-lanes entries; or, on an explicit bank
        /// width, host-core's width seam (#1272 D2), which attaches every live lane.
        struct Preparer(Option<lane::Backend>);

        impl RuntimePreparer for Preparer {
            fn prepare(
                &self,
                compiled: &CompiledSession,
                caps: &HostPrepareCaps,
                lanes: HostLiveLanes,
                _spectrum: Option<&host_core::SpectrumPreparation>,
                successor: Option<SuccessorBase<'_>>,
                superseded: &LiveValuesSuperseded,
            ) -> Result<
                (
                    PreparedHost,
                    HostLiveControlHandles,
                    Option<host_core::PreparedSpectrumCaptures>,
                ),
                PrepareDiagnostics,
            > {
                let live = HostLiveControlRequest::default();
                let (host, handles) = match (self.0, successor) {
                    (None, Some(base)) => prepare_host_runtime_with_live_lanes_successor(
                        compiled, caps, &live, lanes, base, superseded,
                    ),
                    (None, None) => prepare_host_runtime_with_live_lanes(
                        compiled, caps, &live, lanes, superseded,
                    ),
                    (Some(backend), Some(base)) => {
                        host_core::test_only_prepare_host_runtime_with_live_controls_successor_on(
                            compiled, caps, &live, base, backend,
                        )
                    }
                    (Some(backend), None) => {
                        host_core::test_only_prepare_host_runtime_with_live_controls_on(
                            compiled, caps, &live, backend,
                        )
                    }
                }?;
                Ok((host, handles, None))
            }
        }

        fn limits() -> ControlLimits {
            ControlLimits {
                source_ring_frames: RING,
                maximum_document_bytes: 1_000_000,
                maximum_diagnostic_bytes: 4_096,
                maximum_tracks: 100,
                maximum_sources: 100,
                maximum_routes: 100,
                maximum_effects: 100,
                maximum_graph_session_plus_plan_bytes: 1_000_000_000,
                maximum_source_total_bytes: 100_000_000,
                maximum_source_overhead_bytes: 100_000_000,
                maximum_effect_state_bytes: 1_000_000_000,
                maximum_effect_scratch_bytes: 1_000_000_000,
                maximum_builtin_retained_bytes: 1_000_000_000,
                maximum_control_retained_bytes: 100_000_000,
                maximum_named_allocation_bytes: 1_000_000_000,
                maximum_meter_streams: 4_096,
                maximum_meter_items: 1 << 20,
                maximum_meter_bytes: 1 << 26,
                maximum_control_frame_bytes: 65_536,
                maximum_replay_bytes: 262_144,
                maximum_replay_entries: 16,
                maximum_submixes: 0,
                maximum_vcas: 0,
                maximum_vca_reach_pairs: u64::MAX,
                maximum_replacement_peak_bytes: u64::MAX,
            }
        }

        fn sid(text: &str) -> StableId {
            StableId::parse(text).expect("stable ID")
        }

        /// Session A.
        fn session_a() -> SessionModel {
            let mut model = parse_session_json(SESSION).expect("fixture");
            assert_eq!(model.sample_rate_hz, 48_000);
            model.console.pre_insert.clear();
            model.console.post_insert.clear();
            model
                .tracks
                .retain(|track| matches!(track.id.as_str(), "eq0" | "eq1"));
            for track in &mut model.tracks {
                track.console.clear();
                track.inserts.effects.clear();
                for channel in [&mut track.builtins.left, &mut track.builtins.right] {
                    channel.hpf_hz = 0.0;
                    channel.lpf_hz = 0.0;
                }
            }
            model
                .routes
                .retain(|route| matches!(route.id.as_str(), "eq0-main" | "eq1-main"));
            let template = model.sources[0].clone();
            model.sources.push(Source {
                id: sid(AUX),
                content: template.content.replace("7e94", "0a0a"),
                ..template
            });
            model
                .tracks
                .iter_mut()
                .find(|track| track.id.as_str() == "eq1")
                .expect("eq1")
                .source_id = sid(AUX);
            model
        }

        fn limiter(id: &str) -> Effect {
            Effect {
                id: sid(id),
                identity: EffectIdentity::Native {
                    effect_id: sid("miso.true-peak-limiter"),
                },
                quality: EffectQuality::Normal,
                bypass: false,
                link_mode: LinkMode::Maximum,
                params: Vec::new(),
                sidechain: SidechainDeclaration::None,
            }
        }

        /// A compressor insert `id` at its defaults: no latency.
        fn compressor(id: &str) -> Effect {
            Effect {
                identity: EffectIdentity::Native {
                    effect_id: sid("miso.compressor"),
                },
                ..limiter(id)
            }
        }

        /// A muted copy of `eq0` named `id`, with `limiters` true-peak limiters in series, and
        /// its route to the output: a growth of `P` for one limiter, `2P` for two.
        fn muted_track(id: &str, limiters: usize) -> [SessionEdit; 2] {
            let model = session_a();
            let mut track: Track = model.tracks[0].clone();
            track.id = sid(id);
            track.fader.left_mute = true;
            track.fader.right_mute = true;
            track.inserts.effects = (0..limiters)
                .map(|index| limiter(&format!("limiter-{index}")))
                .collect();
            let mut route = model.routes[0].clone();
            route.id = sid(&format!("{id}-main"));
            let RouteSource::Track { track_id, .. } = &mut route.source else {
                panic!("a track route")
            };
            *track_id = track.id.clone();
            [
                SessionEdit::UpsertTrack { track },
                SessionEdit::UpsertRoute { route },
            ]
        }

        /// The growth W: a muted limiter track `t3`.
        fn growth() -> Vec<SessionEdit> {
            muted_track("t3", 1).into()
        }

        fn remove_track(id: &str) -> [SessionEdit; 2] {
            [
                SessionEdit::RemoveRoute {
                    route_id: sid(&format!("{id}-main")),
                },
                SessionEdit::RemoveTrack { track_id: sid(id) },
            ]
        }

        fn builtin(track: &str, control: u32, value: f32) -> SessionEdit {
            SessionEdit::SetControl {
                expected_effect: None,
                address: ControlAddress {
                    owner: Owner::Strip(sid(track)),
                    section: Section::Builtins,
                    control,
                    lane: ParameterChannel::Both,
                },
                value,
                ramp_samples: None,
            }
        }

        /// A source's PCM: two planes, no exact-zero sample, distinct per `seed`.
        fn pcm(seed: u32) -> [Vec<f32>; 2] {
            let plane = |channel: u32| {
                (0..48_000_u32)
                    .map(|frame| {
                        let step = frame
                            .wrapping_mul(7_919)
                            .wrapping_add(seed.wrapping_mul(104_729))
                            .wrapping_add(channel * 31)
                            % 1_999;
                        (step as f32 - 999.5) / 2_000.0
                    })
                    .collect::<Vec<f32>>()
            };
            [plane(0), plane(1)]
        }

        /// One session as a host drives it: feed, command, render one block at a time.
        struct Rig {
            children: CompiledChildren<Adapter>,
            request: u64,
            /// The next block's render sample.
            at: u64,
            /// Each fed source: its ID, PCM and next frame.
            feeds: Vec<(&'static str, [Vec<f32>; 2], u64)>,
            /// Blocks the feed keeps past the next block's source-read sample.
            ahead: u64,
            /// A source fed no frame at or past the given frame.
            stop_at: Option<(&'static str, u64)>,
            /// Each rendered block's output bits, and the epoch render published after it.
            blocks: Vec<Vec<u32>>,
            epochs: Vec<u64>,
            /// The boot plan's epoch.
            boot: u64,
            /// Whether [`Self::render`] runs the service step after each block.
            serviced: bool,
        }

        impl Rig {
            fn new(warm: Option<WarmConfig>) -> Self {
                Self::with_limits(warm, limits())
            }

            fn with_limits(warm: Option<WarmConfig>, limits: ControlLimits) -> Self {
                Self::boot(warm, limits, &session_a())
            }

            /// A with a third source, [`UNUSED`], that no track reads; it is fed as the others.
            fn with_unused(warm: Option<WarmConfig>) -> Self {
                let mut model = session_a();
                let template = model.sources[0].clone();
                model.sources.push(Source {
                    id: sid(UNUSED),
                    content: template.content.replace("7e94", "0b0b"),
                    ..template
                });
                let mut rig = Self::boot(warm, limits(), &model);
                rig.feeds.push((UNUSED, pcm(13), 0));
                rig
            }

            fn boot(warm: Option<WarmConfig>, limits: ControlLimits, model: &SessionModel) -> Self {
                Self::boot_on(warm, limits, model, None)
            }

            /// [`Self::boot`], prepared on `backend`'s bank width when it is `Some`.
            fn boot_on(
                warm: Option<WarmConfig>,
                limits: ControlLimits,
                model: &SessionModel,
                backend: Option<lane::Backend>,
            ) -> Self {
                super::super::test_reset_lifecycle_observer();
                let document = session::canonical_session_json(model).expect("canonical");
                let children = compile_children_warm::<Adapter>(
                    &document,
                    limits,
                    Box::new(Preparer(backend)),
                    warm,
                    None,
                )
                .expect("boot");
                let boot = children
                    .plan
                    .test_active_epoch()
                    .load(std::sync::atomic::Ordering::Acquire);
                Self {
                    boot,
                    children,
                    request: 0,
                    at: 0,
                    feeds: vec![(SOURCE, pcm(7), 0), (AUX, pcm(11), 0)],
                    ahead: AHEAD,
                    stop_at: None,
                    blocks: Vec::new(),
                    epochs: Vec::new(),
                    serviced: true,
                }
            }

            /// Feed whole blocks until every source holds `ahead` blocks past the frame the next
            /// block reads, then render it and run the service step.
            fn render(&mut self, count: usize) {
                for _ in 0..count {
                    let read = self.children.session.test_source_read_clock();
                    let until = read + (self.ahead + 1) * QUANTUM;
                    for (id, planes, next) in &mut self.feeds {
                        let until = match self.stop_at {
                            Some((stopped, frame)) if stopped == *id => until.min(frame),
                            _ => until,
                        };
                        while *next < until {
                            let range = *next as usize..(*next + QUANTUM) as usize;
                            match self.children.session.submit(
                                id.as_bytes(),
                                SourceSubmission {
                                    generation: 1,
                                    start_frame: *next,
                                    sample_rate_hz: 48_000,
                                    planes: &[&planes[0][range.clone()], &planes[1][range]],
                                    frames: QUANTUM as u32,
                                    end_of_region: false,
                                },
                            ) {
                                Ok(_) => *next += QUANTUM,
                                // A source added after boot (W's `c-source`) queues from its
                                // frame 0 while W waits, so its ring fills before any plan reads
                                // it and stays full once one does (its consumer starts at that
                                // backlog, behind the shared read clock): the chunk is refused,
                                // not taken, and offered again at the next block. A full ring of a
                                // source every plan carries from boot is a defect.
                                Err(SourceFailure::Control(SourceControlError::Chunk(
                                    ::source::HostChunkError::Full { .. },
                                ))) if !matches!(*id, SOURCE | AUX | UNUSED) => break,
                                Err(error) => panic!("{id} at {next}: {error:?}"),
                            }
                        }
                    }
                    let mut samples = [f32::NAN; 2 * QUANTUM as usize];
                    let output = PlanarBufferMut::try_new(
                        &mut samples,
                        2,
                        QUANTUM as usize,
                        QUANTUM as usize,
                    )
                    .expect("output");
                    self.children.plan.render(self.at, output).expect("render");
                    self.at += QUANTUM;
                    self.blocks
                        .push(samples.iter().map(|sample| sample.to_bits()).collect());
                    self.epochs.push(
                        self.children
                            .plan
                            .test_active_epoch()
                            .load(std::sync::atomic::Ordering::Acquire),
                    );
                    if self.serviced {
                        self.children.session.service().expect("service");
                    }
                }
            }

            /// [`Self::render`] until the render clock reads `clock`, a block boundary not behind
            /// it.
            fn render_to(&mut self, clock: u64) {
                assert!(clock >= self.at && clock.is_multiple_of(QUANTUM));
                self.render(usize::try_from((clock - self.at) / QUANTUM).expect("blocks"));
                assert_eq!(self.children.session.test_render_clock(), clock);
            }

            /// One control call's deadline step (#1358 D3).
            fn deadline(&mut self) -> Option<TransitionStep> {
                self.children
                    .session
                    .test_check_prime_deadline()
                    .expect("deadline step")
            }

            /// The committed revision.
            fn revision(&self) -> u64 {
                self.children
                    .session
                    .test_controller()
                    .session()
                    .revision()
                    .0
            }

            /// A render call at the wrong sample: render adopts a due, ready candidate at the
            /// block boundary, then refuses the call, so the adopted plan renders nothing.
            fn refused_call(&mut self) {
                let mut samples = [f32::NAN; 2 * QUANTUM as usize];
                let output =
                    PlanarBufferMut::try_new(&mut samples, 2, QUANTUM as usize, QUANTUM as usize)
                        .expect("output");
                let refused = self.children.plan.render(self.at + 1, output);
                assert!(
                    matches!(refused, Err(RenderError::TimeDiscontinuity { .. })),
                    "the call is refused: {refused:?}"
                );
            }

            /// Commit `edits` as one transaction: its edit path, or the refusal.
            fn transact(&mut self, edits: &[SessionEdit]) -> Result<EditPath, CommandError> {
                self.request += 1;
                let codec = ProtocolCodec::default();
                let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
                let length = codec
                    .encode_command_frame_into(
                        &TypedCommandFrame {
                            request_id: RequestId::new(self.request).expect("request"),
                            expected_revision: ExpectedRevision::Exact(
                                self.children.session.test_controller().session().revision(),
                            ),
                            payload: CommandPayload::SessionTransactionApply(edits),
                        },
                        &mut bytes,
                    )
                    .expect("a transaction frame");
                let length = self.children.session.command(&bytes[..length], 65_536)?;
                let mut fields = [0_u16; 64];
                let path = match codec
                    .decode_typed_response(
                        self.children.session.command_response(length),
                        &mut DecodeScratch::new(&mut fields),
                    )
                    .expect("a transaction response")
                {
                    DecodedTypedResponseFrame::Success {
                        payload: DecodedSuccessResponsePayload::SessionTransactionApplied(applied),
                        ..
                    } => applied.edit_path,
                    _ => panic!("not applied"),
                };
                while self
                    .children
                    .session
                    .dequeue_event(EventLane::Reliable, 65_536)
                    .expect("an event")
                    .is_some()
                {}
                Ok(path)
            }

            /// [`Self::transact`], which must commit as a rebuild; the adoption it published.
            fn rebuild(&mut self, edits: &[SessionEdit]) -> PlanAdoption {
                assert!(matches!(self.transact(edits), Ok(EditPath::Rebuild)));
                self.children
                    .session
                    .test_take_published_adoption()
                    .expect("a publication")
            }

            fn primed(&self) -> Option<PrimedCandidate> {
                self.children.session.test_primed()
            }

            /// The blocks render adopted a new plan at.
            fn swaps(&self) -> Vec<usize> {
                let boot = self.boot;
                (0..self.epochs.len())
                    .filter(|&index| {
                        self.epochs[index] != index.checked_sub(1).map_or(boot, |i| self.epochs[i])
                    })
                    .collect()
            }

            fn underruns(&self) -> u64 {
                self.children.plan.queries().source_underrun_blocks()
            }
        }

        /// `S = ceil_q(p + q + N + C)` (#1325 D3) of a duck published at render clock `p` on A,
        /// whose route lines are all 0 samples long (`C = 0`), with A's mute ramp `N`.
        fn duck_sample(p: u64) -> u64 {
            let n = u64::from(session_a().control_smoothing_samples().mute);
            (p + QUANTUM + n).div_ceil(QUANTUM) * QUANTUM
        }

        /// The first block whose bits differ between two runs, if any.
        fn first_divergence(left: &Rig, right: &Rig) -> Option<usize> {
            assert_eq!(left.blocks.len(), right.blocks.len());
            left.blocks
                .iter()
                .zip(&right.blocks)
                .position(|(left, right)| left != right)
        }

        /// Gate 1, classification and publication. With the bounds, the growth is published
        /// `Primed { not_before: render_clock(), lead_blocks: 4 }` and recorded as such; it waits
        /// while the carried rings hold less than its prime (every ring A renders is flagged), is
        /// adopted once fed, and renders A continued. Without the bounds the same growth is
        /// published `Next` with no record; with them, an edit that grows nothing is too.
        ///
        /// Test value: a classifier that publishes the warm candidate `Next` (or with no flags
        /// written) adopts it before render is ready -- its prime underruns or it repeats `P`
        /// frames; a session with no bounds that classifies changes today's path.
        #[test]
        fn a_growth_is_published_primed_and_recorded() {
            let mut rig = Rig::new(Some(CONFIG));
            rig.ahead = 1;
            rig.render(BEFORE);
            let adoption = rig.rebuild(&growth());
            let not_before = rig.children.session.test_render_clock();
            assert_eq!(not_before, BEFORE as u64 * QUANTUM);
            assert_eq!(
                adoption,
                PlanAdoption::Primed {
                    not_before,
                    lead_blocks: 4
                }
            );
            assert_eq!(
                rig.primed(),
                Some(PrimedCandidate {
                    epoch: rig.children.session.newest_plan_epoch(),
                    not_before,
                    lead_blocks: 4,
                })
            );
            rig.render(3);
            assert!(rig.swaps().is_empty(), "held while the prime is not fed");
            rig.ahead = AHEAD;
            rig.render(6);
            assert_eq!(rig.swaps(), [BEFORE + 3], "adopted once fed");
            assert_eq!(rig.primed(), None, "the adoption drops the record");
            let mut continued = Rig::new(None);
            continued.render(rig.blocks.len());
            assert_eq!(first_divergence(&rig, &continued), None, "A continued");

            let mut plain = Rig::new(None);
            plain.render(BEFORE);
            assert_eq!(plain.rebuild(&growth()), PlanAdoption::Next, "no bounds");
            assert_eq!(plain.primed(), None);
            let mut flat = Rig::new(Some(CONFIG));
            flat.render(BEFORE);
            assert_eq!(
                flat.rebuild(&muted_track("t4", 0)),
                PlanAdoption::Next,
                "no growth"
            );
            assert_eq!(flat.primed(), None);

            // The growth with the removal of a source no track reads ducks no strip (#1403 D2,
            // attempt 2): it stays `Primed`, and the removed source's frames go to A until W is
            // adopted.
            let mut rig = Rig::with_unused(Some(CONFIG));
            rig.ahead = 1;
            rig.render(BEFORE);
            let mut edits = growth();
            edits.push(SessionEdit::RemoveSource {
                source_id: sid(UNUSED),
            });
            let adoption = rig.rebuild(&edits);
            let not_before = rig.children.session.test_render_clock();
            assert_eq!(
                adoption,
                PlanAdoption::Primed {
                    not_before,
                    lead_blocks: 4
                },
                "a removed unused source"
            );
            assert_eq!(
                rig.primed(),
                Some(PrimedCandidate {
                    epoch: rig.children.session.newest_plan_epoch(),
                    not_before,
                    lead_blocks: 4,
                })
            );
            rig.render(3);
            assert!(rig.swaps().is_empty(), "held while the prime is not fed");
            rig.ahead = AHEAD;
            rig.render(1);
            assert_eq!(rig.swaps(), [BEFORE + 3]);
            rig.feeds.truncate(2);
            rig.render(5);
            let mut continued = Rig::with_unused(None);
            continued.render(BEFORE + 4);
            continued.feeds.truncate(2);
            continued.render(5);
            assert_eq!(first_divergence(&rig, &continued), None, "A continued");
        }

        /// #1397 gates 1 and 2, a latent insert on an audible strip, on both bank widths: a
        /// true-peak limiter added to track 1 grows the output's arrival and restarts track 1, so
        /// the edit is published `Primed` no earlier than the duck's `S`, and with every source
        /// fed ahead from the start it is adopted at exactly the block that starts at `S`. Every
        /// block before track 1's fire block (the first at or after the adoption plus the
        /// limiter's latency `D`) equals A with track 1 live-muted at the edit's block; from it,
        /// for 64 blocks, the output is the sum of the same swap on A with track 2 muted and that
        /// reference; the adoption reports `EXACT`.
        ///
        /// Test value: an adoption before the duck has settled moves the output before the fire;
        /// a fire at the adoption instead of `D` later differs early; a carried track 2 that
        /// drifts after the fire, or a track 1 left out or counted twice, breaks the sum.
        #[test]
        fn a_latent_insert_on_an_audible_strip_is_duck_swapped_warm() {
            const AFTER: usize = 64;
            /// The true-peak limiter's latency at 48 kHz, `D` of track 1's fade.
            const LIMITER: u64 = 486;
            let mut limited = session_a().tracks[0].clone();
            limited.inserts.effects = vec![limiter("limiter-0")];
            let edit = [SessionEdit::UpsertTrack { track: limited }];
            let mut quiet = session_a();
            let eq1 = quiet
                .tracks
                .iter_mut()
                .find(|track| track.id.as_str() == "eq1")
                .expect("eq1");
            eq1.fader.left_mute = true;
            eq1.fader.right_mute = true;
            for &backend in lane::Backend::VECTOR {
                let boot = |model: &SessionModel| {
                    let mut rig = Rig::boot_on(Some(CONFIG), limits(), model, Some(backend));
                    rig.render(BEFORE);
                    rig
                };
                let swap = |model: &SessionModel| {
                    let mut rig = boot(model);
                    let s = duck_sample(rig.children.session.test_render_clock());
                    assert_eq!(
                        rig.rebuild(&edit),
                        PlanAdoption::Primed {
                            not_before: s,
                            lead_blocks: 4
                        },
                        "{backend:?}"
                    );
                    (rig, s)
                };
                let (mut swapped, s) = swap(&session_a());
                let (mut alone, _) = swap(&quiet);
                let mut reference = boot(&session_a());
                assert!(matches!(
                    reference.transact(&[builtin("eq0", 6, 1.0)]),
                    Ok(EditPath::Live)
                ));
                let s_block = usize::try_from(s / QUANTUM).expect("block");
                let fire = usize::try_from((s + LIMITER).div_ceil(QUANTUM)).expect("block");
                for rig in [&mut swapped, &mut alone, &mut reference] {
                    rig.render(fire + AFTER - BEFORE);
                }
                assert_eq!(swapped.swaps(), [s_block], "{backend:?}: adopted at S");
                let watermark = swapped
                    .children
                    .plan
                    .queries()
                    .watermark()
                    .expect("watermark");
                assert_eq!(watermark.outcome_flags, OUTCOME_EXACT, "{backend:?}");
                assert_eq!(
                    first_divergence(&swapped, &reference),
                    Some(fire),
                    "{backend:?}: the reference until the fire"
                );
                for block in fire..fire + AFTER {
                    let sum: Vec<u32> = alone.blocks[block]
                        .iter()
                        .zip(&reference.blocks[block])
                        .map(|(&left, &right)| {
                            (f32::from_bits(left) + f32::from_bits(right)).to_bits()
                        })
                        .collect();
                    assert_eq!(swapped.blocks[block], sum, "{backend:?}: block {block}");
                }
            }
        }

        /// #1397 gate 3, the transition at submit. A holds a compressor on track 2; the edit adds a
        /// true-peak limiter to track 1 with `P_MAX` one quantum below `P`, so `warm_lead` answers
        /// `LeadBound`. The edit commits as a rebuild, `grown_strips` names both tracks (the
        /// output's arrival grows) and so does the successor's `restarted_strips()`, and it is
        /// published `NoEarlierThan(S)` with no record. Up to `S` every block equals A with both
        /// tracks live-muted at the edit's block; from `S` every block equals a fresh plan of the
        /// edited session with both tracks muted, fed from frame `S`, and live-unmuted at their
        /// fire blocks (track 2 at once, track 1 the limiter's latency later). The adoption reports
        /// `TRANSITION_FALLBACK` and `transition_fallback` counts its revision.
        ///
        /// Test value: a bound that refuses the edit, a transition reported `EXACT`, a duck set
        /// without track 2, or a successor that carries track 2 (its compressor's envelope or its
        /// ducked fader) breaks the references.
        #[test]
        fn a_growth_past_the_bound_takes_the_transition_at_submit() {
            const AFTER: usize = 64;
            const LIMITER: u64 = 486;
            let bound = WarmConfig {
                p_max: LEAD - QUANTUM,
                prime_bytes_max: u64::MAX,
            };
            let mut a = session_a();
            a.tracks[1].inserts.effects = vec![compressor("comp")];
            let mut b = a.clone();
            b.tracks[0].inserts.effects = vec![limiter("limiter-0")];
            let edit = [SessionEdit::UpsertTrack {
                track: b.tracks[0].clone(),
            }];
            let mut rig = Rig::boot(Some(bound), limits(), &a);
            rig.render(BEFORE);
            let s = duck_sample(rig.children.session.test_render_clock());
            assert_eq!(rig.rebuild(&edit), PlanAdoption::NoEarlierThan(s));
            assert_eq!(rig.primed(), None);
            // Track 1's limiter restarts it; only `grown_strips` adds track 2.
            let newest = rig
                .children
                .session
                .pending_providers
                .last()
                .expect("the transition");
            assert_eq!(
                newest
                    .inventory
                    .restarted_strips()
                    .iter()
                    .map(|id| &**id)
                    .collect::<Vec<_>>(),
                ["eq0", "eq1"]
            );
            let s_block = usize::try_from(s / QUANTUM).expect("block");
            let fire = usize::try_from(LIMITER.div_ceil(QUANTUM)).expect("blocks");
            let total = s_block + fire + AFTER;
            rig.render(total - BEFORE);
            assert_eq!(rig.swaps(), [s_block]);
            let watermark = rig.children.plan.queries().watermark().expect("watermark");
            assert_eq!(
                (watermark.outcome_flags, watermark.transition_fallback),
                (OUTCOME_TRANSITION_FALLBACK, 1)
            );
            let mut reference = Rig::boot(None, limits(), &a);
            reference.render(BEFORE);
            assert!(matches!(
                reference.transact(&[builtin("eq0", 6, 1.0), builtin("eq1", 6, 1.0)]),
                Ok(EditPath::Live)
            ));
            reference.render(s_block - BEFORE);
            assert_eq!(
                rig.blocks[..s_block],
                reference.blocks[..],
                "A with both tracks live-muted until S"
            );
            let mut muted = b.clone();
            for track in &mut muted.tracks {
                track.fader.left_mute = true;
                track.fader.right_mute = true;
            }
            let mut fresh = Rig::boot(None, limits(), &muted);
            let offset = usize::try_from(s).expect("frames");
            for (_, planes, _) in &mut fresh.feeds {
                for plane in planes.iter_mut() {
                    plane.drain(..offset);
                }
            }
            for block in 0..fire + AFTER {
                for (track, at) in [("eq1", 0), ("eq0", fire)] {
                    if block == at {
                        assert!(matches!(
                            fresh.transact(&[builtin(track, 6, 0.0)]),
                            Ok(EditPath::Live)
                        ));
                    }
                }
                fresh.render(1);
            }
            assert_eq!(
                rig.blocks[s_block..],
                fresh.blocks[..],
                "a fresh plan of the edit from S"
            );
        }

        /// The gate-6 session (#1397b): track `t1` routed from its `post_fader` tap into submix
        /// `g`, `g` into submix `h`, `h` to the output; track `t2` with two soft-clip inserts
        /// (31 samples each) into `h`; track `k` with a true-peak limiter (486 samples) and then a
        /// compressor keyed from `g`'s `key` tap, to the output. `t1` reads [`SOURCE`], `t2` and
        /// `k` read [`AUX`]. With `clipped`, `t1` holds one soft-clip insert.
        fn keyed_bus(key: SendTap, clipped: bool) -> SessionModel {
            use session::{Route, RouteDestination, Sidechain, Submix};
            let mut model = session_a();
            let template_track = model.tracks[0].clone();
            let template_route = model.routes[0].clone();
            let soft_clip = |id: &str| Effect {
                identity: EffectIdentity::Native {
                    effect_id: sid("miso.soft-clip"),
                },
                link_mode: LinkMode::DualMono,
                ..limiter(id)
            };
            let track = |id: &str, source: &str, inserts: Vec<Effect>| Track {
                id: sid(id),
                source_id: sid(source),
                inserts: session::Rack { effects: inserts },
                ..template_track.clone()
            };
            let mut keyed = compressor("comp");
            keyed.link_mode = LinkMode::DualMono;
            keyed.sidechain = SidechainDeclaration::Routed(Sidechain {
                source: RouteSource::Submix {
                    submix_id: sid("g"),
                    tap: key,
                },
                port_id: sid("sidechain-in"),
            });
            model.tracks = vec![
                track("k", AUX, vec![limiter("limiter-0"), keyed]),
                track(
                    "t1",
                    SOURCE,
                    if clipped {
                        vec![soft_clip("clip-1")]
                    } else {
                        Vec::new()
                    },
                ),
                track("t2", AUX, vec![soft_clip("clip-a"), soft_clip("clip-b")]),
            ];
            model.submixes = vec![
                Submix::unity(sid("g"), &model.console),
                Submix::unity(sid("h"), &model.console),
            ];
            let route = |id: &str, source: RouteSource, into: Option<&str>| Route {
                id: sid(id),
                source,
                destination: into.map_or_else(
                    || template_route.destination.clone(),
                    |submix| RouteDestination::SubmixInput {
                        submix_id: sid(submix),
                    },
                ),
                ..template_route.clone()
            };
            let from_track = |id: &str, tap| RouteSource::Track {
                track_id: sid(id),
                tap,
            };
            let from_submix = |id: &str, tap| RouteSource::Submix {
                submix_id: sid(id),
                tap,
            };
            model.routes = vec![
                route("g-h", from_submix("g", SendTap::PostFader), Some("h")),
                route("h-main", from_submix("h", SendTap::PostPan), None),
                route("k-main", from_track("k", SendTap::PostPan), None),
                route("t1-g", from_track("t1", SendTap::PostFader), Some("g")),
                route("t2-h", from_track("t2", SendTap::PostFader), Some("h")),
            ];
            model
        }

        /// #1397 gate 6, a sidechain key out of a grown strip, on both bank widths. A soft-clip
        /// insert added to `t1` of [`keyed_bus`] with `P_MAX` 0 takes the transition at submit
        /// (`warm_lead` answers `LeadBound`); `g`'s arrival grows by 31, so the `g`-to-`h` route
        /// line shrinks from 62 to 31 samples and the `g`-to-`k` key line from 486 to 455.
        /// - (a) Keyed from `g`'s `post_fader` tap: the successor restarts `[g, t1]`, and `S`
        ///   counts the key line, `C = 486`.
        /// - (b) Keyed from `g`'s `pre_fader` tap: #1324 D1's closure restarts `k` too,
        ///   `[g, k, t1]`, and `C` is `g`-to-`h`'s 62.
        ///
        /// Each is published `NoEarlierThan(S)`, adopted at `S`, reports `TRANSITION_FALLBACK`,
        /// and every block before `t1`'s fire block (the first at or after `S + 31`) equals the
        /// session with no structural edit and the ducked strips live-muted at the edit's block.
        ///
        /// Test value: (a) a transition whose `C` misses the key line out of the forced restart
        /// `g` publishes at the route lines' `S` (62, not 486); (b) a duck set that is not closed
        /// over sidechains after the forced restarts join it (the preparation's closure run
        /// before them) leaves `k` carried and audible through the duck, so the output differs
        /// from the edit's block. (a)'s output part does not see that `S`: a shorter carried line
        /// keeps its oldest samples and drops the newest δ (`CompensationDelay::adopt`), so the
        /// route lines' `S` also drops only `+0.0`, and any drop would show only after the
        /// comparison window, which ends at `t1`'s fire block.
        #[test]
        fn a_sidechain_key_out_of_a_grown_strip_is_ducked() {
            const CLIP: u64 = 31;
            let bound = WarmConfig {
                p_max: 0,
                prime_bytes_max: u64::MAX,
            };
            let limits = ControlLimits {
                maximum_submixes: 2,
                ..limits()
            };
            for (key, restarted, c) in [
                (SendTap::PostFader, &["g", "t1"][..], 486),
                (SendTap::PreFader, &["g", "k", "t1"][..], 62),
            ] {
                let a = keyed_bus(key, false);
                let b = keyed_bus(key, true);
                let edit = [SessionEdit::UpsertTrack {
                    track: b.tracks[1].clone(),
                }];
                let n = u64::from(a.control_smoothing_samples().mute);
                for &backend in lane::Backend::VECTOR {
                    let boot = |warm| {
                        let mut rig = Rig::boot_on(warm, limits, &a, Some(backend));
                        rig.render(BEFORE);
                        rig
                    };
                    let mut rig = boot(Some(bound));
                    let p = rig.children.session.test_render_clock();
                    let s = (p + QUANTUM + n + c).div_ceil(QUANTUM) * QUANTUM;
                    assert_eq!(
                        rig.rebuild(&edit),
                        PlanAdoption::NoEarlierThan(s),
                        "{key:?} {backend:?}"
                    );
                    assert_eq!(rig.primed(), None);
                    let newest = rig
                        .children
                        .session
                        .pending_providers
                        .last()
                        .expect("the transition");
                    assert_eq!(
                        newest
                            .inventory
                            .restarted_strips()
                            .iter()
                            .map(|id| &**id)
                            .collect::<Vec<_>>(),
                        restarted,
                        "{key:?} {backend:?}"
                    );
                    let s_block = usize::try_from(s / QUANTUM).expect("block");
                    let fire = usize::try_from((s + CLIP).div_ceil(QUANTUM)).expect("block");
                    rig.render(fire - BEFORE);
                    assert_eq!(rig.swaps(), [s_block], "{key:?} {backend:?}");
                    let watermark = rig.children.plan.queries().watermark().expect("watermark");
                    assert_eq!(watermark.outcome_flags, OUTCOME_TRANSITION_FALLBACK);
                    let mut reference = boot(None);
                    let mutes: Vec<SessionEdit> = restarted
                        .iter()
                        .map(|strip| builtin(strip, 6, 1.0))
                        .collect();
                    assert!(matches!(reference.transact(&mutes), Ok(EditPath::Live)));
                    reference.render(fire - BEFORE);
                    assert_eq!(
                        first_divergence(&rig, &reference),
                        None,
                        "{key:?} {backend:?}: the reference until t1's fire block"
                    );
                }
            }
        }

        /// #1397 D1's second arm: a `Warm` answer with a zero lead (injected: the restarts confine
        /// a growth, and this session family reaches no such edit) is prepared with its lead, so
        /// its whole restart, track 2, joins the successor's `restarted_strips()` and ducks; it is
        /// published as an ordinary duck-swap, `NoEarlierThan(S)` with no record, and no block
        /// before `S` differs from A with track 2 live-muted; its adoption reports `EXACT`.
        ///
        /// Test value: a zero lead published `Primed` (no prime to run), or a transition not
        /// planned again over the warm successor's restarts (track 2 restarted unducked, published
        /// `Next`), is red.
        #[test]
        fn a_zero_lead_is_published_as_an_ordinary_duck_swap() {
            let mut rig = Rig::new(Some(CONFIG));
            rig.render(BEFORE);
            let s = duck_sample(rig.children.session.test_render_clock());
            rig.children.session.test_set_next_zero_lead(vec!["eq1"]);
            assert_eq!(
                rig.rebuild(&muted_track("t4", 0)),
                PlanAdoption::NoEarlierThan(s)
            );
            assert_eq!(rig.primed(), None);
            let newest = rig
                .children
                .session
                .pending_providers
                .last()
                .expect("the successor");
            assert_eq!(newest.inventory.restarted_strips(), [Box::from("eq1")]);
            let s_block = usize::try_from(s / QUANTUM).expect("block");
            rig.render(s_block + 1 - BEFORE);
            assert_eq!(rig.swaps(), [s_block]);
            let watermark = rig.children.plan.queries().watermark().expect("watermark");
            assert_eq!(watermark.outcome_flags, OUTCOME_EXACT);
            let mut reference = Rig::new(None);
            reference.render(BEFORE);
            assert!(matches!(
                reference.transact(&[builtin("eq1", 6, 1.0)]),
                Ok(EditPath::Live)
            ));
            reference.render(s_block - BEFORE);
            assert_eq!(rig.blocks[..s_block], reference.blocks[..]);
        }

        /// The growth W of #1397 gate 4: [`growth`], plus a source `c-source` and an audible track
        /// `tc` on it, so W adds a ring the host feeds.
        fn growth_with_source() -> Vec<SessionEdit> {
            let model = session_a();
            let template = model.sources[0].clone();
            let source = Source {
                id: sid("c-source"),
                content: template.content.replace("7e94", "0c0c"),
                ..template
            };
            let mut track: Track = model.tracks[0].clone();
            track.id = sid("tc");
            track.source_id = sid("c-source");
            let mut route = model.routes[0].clone();
            route.id = sid("tc-main");
            let RouteSource::Track { track_id, .. } = &mut route.source else {
                panic!("a track route")
            };
            *track_id = track.id.clone();
            let mut edits = growth();
            edits.extend([
                SessionEdit::UpsertSource { source },
                SessionEdit::UpsertTrack { track },
                SessionEdit::UpsertRoute { route },
            ]);
            edits
        }

        /// #1358 gate 5's setup: W ([`growth_with_source`]) waits `Primed` with frames withheld
        /// until its deadline, and the deadline step there, with #1397's injected refusal,
        /// withdraws W, is refused and republishes W whole. Returns the rig, W's record and its
        /// revision, after asserting the refusal: W is back in the mailbox with its record, one
        /// `transition_reprepare_refusals`, the revision pending, nothing adopted.
        fn refused_at_the_deadline() -> (Rig, PrimedCandidate, u64) {
            let mut rig = Rig::new(Some(CONFIG));
            rig.ahead = 1;
            rig.render(BEFORE);
            assert!(matches!(
                rig.rebuild(&growth_with_source()),
                PlanAdoption::Primed { .. }
            ));
            rig.feeds.push(("c-source", pcm(17), 0));
            let record = rig.primed().expect("W's record");
            let revision = rig.revision();
            rig.render_to(record.not_before + DEADLINE);
            rig.children.session.test_refuse_next_reprepare();
            assert_eq!(rig.deadline(), Some(TransitionStep::Refused));
            assert_eq!(rig.children.session.test_take_published_adoption(), None);
            assert_eq!(rig.children.session.test_transition_reprepare_refusals(), 1);
            assert_eq!(rig.primed(), Some(record), "W keeps its record");
            assert_eq!(rig.children.session.newest_plan_epoch(), record.epoch);
            rig.render(1);
            assert!(rig.swaps().is_empty(), "W is still pending, unready");
            let watermark = rig.children.plan.queries().watermark().expect("watermark");
            assert!(watermark.revision < revision, "W's revision is pending");
            (rig, record, revision)
        }

        /// #1358 gate 5 (and #1397 gate 4), a refused re-preparation at the deadline: the setup
        /// [`refused_at_the_deadline`], then three runs.
        /// 1. The next deadline step withdraws W again and publishes the transition
        ///    `NoEarlierThan(S)` and drops the record; a structural edit refused while it waits
        ///    republishes it. Adopted at `S`, it completes W's revision as `TRANSITION_FALLBACK`,
        ///    with `c` played from the donor's ring and no underrun.
        /// 2. W's frames are queued and render adopts W before the next step, which finds it
        ///    taken: nothing is published and the revision completes `EXACT`.
        /// 3. A structural edit supersedes W (#1310) before the next step: its adoption reports
        ///    `EXACT | SUPERSEDED`, no transition is published, and `c` plays with no underrun.
        ///
        /// Test value: a refusal that keeps the donor outside the mailbox or drops the record
        /// leaves the revision pending forever (no step runs without a record, and a superseding
        /// edit never takes the donor, so `c`'s acked chunks are lost); a fallback that drops the
        /// donor before preparing loses `c`'s acked frames (an underrun); a republish that resets
        /// the words reports `EXACT`; a step that ignores `Taken` publishes over the adopted W.
        #[test]
        fn a_refused_fallback_at_the_deadline_keeps_the_donor_pending() {
            // Run 1.
            let (mut rig, _, revision) = refused_at_the_deadline();
            assert_eq!(rig.deadline(), Some(TransitionStep::Published));
            let Some(PlanAdoption::NoEarlierThan(s)) =
                rig.children.session.test_take_published_adoption()
            else {
                panic!("the transition is a duck-swap")
            };
            assert_eq!(rig.primed(), None);
            // A structural edit refused while the transition waits (#1310 D4).
            let held = rig.children.session.held_epochs().expect("held");
            let cap = rig.children.session.limits.maximum_effect_state_bytes;
            rig.children.session.limits.maximum_effect_state_bytes = held
                .as_slice()
                .iter()
                .map(|epoch| epoch.report.effect_scalar_state_bytes)
                .sum();
            assert!(rig.transact(&muted_track("t4", 2)).is_err(), "B is refused");
            rig.children.session.limits.maximum_effect_state_bytes = cap;
            rig.ahead = AHEAD;
            let s_block = usize::try_from(s / QUANTUM).expect("block");
            rig.render(s_block + 8 - rig.blocks.len());
            assert_eq!(rig.swaps(), [s_block]);
            let watermark = rig.children.plan.queries().watermark().expect("watermark");
            assert_eq!(
                (watermark.revision, watermark.outcome_flags),
                (revision, OUTCOME_TRANSITION_FALLBACK)
            );
            assert_eq!(watermark.transition_fallback, 1);
            assert_eq!(rig.underruns(), 0, "c plays from the donor's ring");

            // Run 2.
            let (mut rig, _, revision) = refused_at_the_deadline();
            rig.ahead = AHEAD;
            rig.serviced = false;
            rig.render(1);
            rig.serviced = true;
            let adopted = rig.blocks.len() - 1;
            assert_eq!(rig.swaps(), [adopted], "render adopts W");
            assert_eq!(rig.deadline(), Some(TransitionStep::NotPending));
            assert_eq!(rig.children.session.test_take_published_adoption(), None);
            assert_eq!(rig.primed(), None);
            rig.render(4);
            let watermark = rig.children.plan.queries().watermark().expect("watermark");
            assert_eq!(
                (watermark.revision, watermark.outcome_flags),
                (revision, OUTCOME_EXACT)
            );
            assert_eq!(watermark.transition_fallback, 0);
            assert_eq!(rig.underruns(), 0);

            // Run 3.
            let (mut rig, _, revision) = refused_at_the_deadline();
            assert_eq!(rig.rebuild(&remove_track("t3")), PlanAdoption::Next);
            assert_eq!(rig.primed(), None);
            assert_eq!(rig.deadline(), None, "no record, no step");
            rig.ahead = AHEAD;
            rig.render(8);
            assert_eq!(rig.swaps().len(), 1, "B is adopted");
            let watermark = rig.children.plan.queries().watermark().expect("watermark");
            assert_eq!(
                (watermark.revision, watermark.outcome_flags),
                (revision + 1, OUTCOME_EXACT | OUTCOME_SUPERSEDED)
            );
            assert_eq!(watermark.transition_fallback, 0);
            assert_eq!(rig.underruns(), 0, "every chunk submitted for c is played");
        }

        /// #1358 gates 1 and 3, never fed, in two arms: the growth W, whose `not_before` is the
        /// render clock at publication, and W with track 2's removal, which ducks track 2, so its
        /// `not_before` is the duck's `S`, past publication. The carried sources are fed no frame
        /// past what A needs. With render paused, 1,000 deadline steps publish nothing. The step at
        /// render clock `not_before + 7,040 - q` publishes nothing; the step at `not_before +
        /// 7,040` withdraws W and publishes the transition, whose adoption completes W's revision
        /// as `TRANSITION_FALLBACK`, `transition_fallback` 1. Until the step, the output is A
        /// continued (with track 2 live-muted at the edit's block in the second arm).
        ///
        /// Test value: a deadline that never fires leaves the edit pending forever; one counted
        /// from publication instead of `not_before` (second arm), one quantum short, or one taken
        /// as the stall ring (7,296) fires at the wrong call; one counted in control calls falls
        /// back on a paused host (no wall-clock sleep: scope note R2 refuses one).
        #[test]
        fn a_warm_candidate_not_ready_by_its_deadline_takes_the_transition() {
            for ducks in [false, true] {
                let mut rig = Rig::new(Some(CONFIG));
                rig.ahead = 1;
                rig.render(BEFORE);
                let mut edits = growth();
                if ducks {
                    edits.extend(remove_track("eq1"));
                }
                let published = rig.children.session.test_render_clock();
                let PlanAdoption::Primed { not_before, .. } = rig.rebuild(&edits) else {
                    panic!("W is warm")
                };
                assert_eq!(not_before > published, ducks);
                let record = rig.primed().expect("W's record");
                let revision = rig.revision();
                // Gate 3: a paused host.
                for _ in 0..1_000 {
                    assert_eq!(rig.deadline(), None, "paused");
                }
                assert_eq!(rig.children.session.test_take_published_adoption(), None);
                rig.render_to(not_before + DEADLINE - QUANTUM);
                assert_eq!(rig.deadline(), None, "one quantum short");
                assert_eq!(rig.children.session.test_take_published_adoption(), None);
                assert_eq!(rig.primed(), Some(record));
                rig.render(1);
                let stepped = rig.blocks.len();
                assert_eq!(rig.deadline(), Some(TransitionStep::Published));
                let Some(PlanAdoption::NoEarlierThan(s)) =
                    rig.children.session.test_take_published_adoption()
                else {
                    panic!("the transition is a duck-swap")
                };
                assert_eq!(rig.primed(), None);
                rig.ahead = AHEAD;
                let s_block = usize::try_from(s / QUANTUM).expect("block");
                rig.render(s_block + 1 - rig.blocks.len());
                assert_eq!(rig.swaps(), [s_block]);
                let watermark = rig.children.plan.queries().watermark().expect("watermark");
                assert_eq!(
                    (watermark.revision, watermark.outcome_flags),
                    (revision, OUTCOME_TRANSITION_FALLBACK)
                );
                assert_eq!(watermark.transition_fallback, 1);
                let mut continued = Rig::new(None);
                continued.render(BEFORE);
                if ducks {
                    assert!(matches!(
                        continued.transact(&[builtin("eq1", 6, 1.0)]),
                        Ok(EditPath::Live)
                    ));
                }
                continued.render(stepped - BEFORE);
                assert_eq!(rig.blocks[..stepped], continued.blocks[..], "A continued");
            }
        }

        /// #1358 gate 2, render adopts first: W's deadline passes with its frames withheld, then
        /// the frames are queued and render adopts W in one block before the next control call.
        /// That call's deadline step finds W taken: it publishes nothing and drops the record, the
        /// revision completes `EXACT`, `transition_fallback` stays 0, and the output is A
        /// continued.
        ///
        /// Test value: a step that ignores `Taken`, or falls back without withdrawing, publishes a
        /// second swap over the adopted plan.
        #[test]
        fn a_candidate_render_adopts_past_its_deadline_completes_exact() {
            let mut rig = Rig::new(Some(CONFIG));
            rig.ahead = 1;
            rig.render(BEFORE);
            let PlanAdoption::Primed { not_before, .. } = rig.rebuild(&growth()) else {
                panic!("W is warm")
            };
            let revision = rig.revision();
            rig.render_to(not_before + DEADLINE + QUANTUM);
            rig.ahead = AHEAD;
            rig.serviced = false;
            rig.render(1);
            rig.serviced = true;
            assert_eq!(rig.swaps(), [rig.blocks.len() - 1], "render adopts W");
            assert!(rig.primed().is_some(), "no service step ran");
            let taken = super::super::test_lifecycle_counters().candidate_taken;
            assert_eq!(rig.deadline(), Some(TransitionStep::NotPending));
            assert_eq!(
                super::super::test_lifecycle_counters().candidate_taken,
                taken + 1,
                "the withdrawal found W taken"
            );
            assert_eq!(rig.children.session.test_take_published_adoption(), None);
            assert_eq!(rig.primed(), None);
            rig.render(4);
            assert_eq!(rig.swaps().len(), 1);
            let watermark = rig.children.plan.queries().watermark().expect("watermark");
            assert_eq!(
                (watermark.revision, watermark.outcome_flags),
                (revision, OUTCOME_EXACT)
            );
            assert_eq!(watermark.transition_fallback, 0);
            let mut continued = Rig::new(None);
            continued.render(rig.blocks.len());
            assert_eq!(first_divergence(&rig, &continued), None, "A continued");
        }

        /// #1358 gate 4, a superseded candidate's deadline is not used. The session's mute ramp
        /// is 24,000 samples. W (the growth) waits `Primed`, frames withheld; before W's deadline
        /// a structural edit B removes W's track and track 2. B grows nothing over the running
        /// plan, so it is a two-phase removal published `NoEarlierThan(S)`, `S` past W's
        /// `not_before + 7,040`. Deadline steps after every block from W's deadline to `S`
        /// publish nothing and withdraw nothing, and render adopts B at `S` with
        /// `transition_fallback` 0. The same with W adopted by render first. Then, with frames
        /// withheld, B's supersession is refused by the effect-state cap set one below what B's
        /// admission needs: W is republished with its record, and the step at W's deadline
        /// withdraws W and publishes the transition, which completes W's revision as
        /// `TRANSITION_FALLBACK`.
        ///
        /// Test value: an unkeyed step together with a record kept across the supersession
        /// withdraws B at W's deadline and loses its `S`; a record dropped on the refused
        /// supersession leaves W pending forever, since the step runs only while a record exists.
        #[test]
        fn a_superseded_candidates_deadline_is_not_used() {
            let mut slow = session_a();
            slow.control_smoothing = Some(session::ControlSmoothing {
                mute_ms: 500.0,
                ..session::CONTROL_SMOOTHING_DEFAULT
            });
            assert_eq!(slow.control_smoothing_samples().mute, 24_000);
            let mut b = remove_track("t3").to_vec();
            b.extend(remove_track("eq1"));
            for adopted in [false, true] {
                let mut rig = Rig::boot(Some(CONFIG), limits(), &slow);
                rig.ahead = if adopted { AHEAD } else { 1 };
                rig.render(BEFORE);
                let PlanAdoption::Primed { not_before, .. } = rig.rebuild(&growth()) else {
                    panic!("W is warm")
                };
                rig.render(if adopted { 2 } else { 1 });
                assert_eq!(rig.swaps().len(), usize::from(adopted));
                let p = rig.children.session.test_render_clock();
                let s = (p + QUANTUM + 24_000).div_ceil(QUANTUM) * QUANTUM;
                assert!(s > not_before + DEADLINE);
                assert_eq!(rig.rebuild(&b), PlanAdoption::NoEarlierThan(s));
                let epoch = rig.children.session.newest_plan_epoch();
                let withdrawn = super::super::test_lifecycle_counters().candidate_withdrawn;
                rig.ahead = 1;
                rig.render_to(not_before + DEADLINE);
                while rig.at < s {
                    assert_eq!(rig.deadline(), None);
                    assert_eq!(rig.children.session.test_take_published_adoption(), None);
                    rig.render(1);
                }
                assert_eq!(rig.children.session.newest_plan_epoch(), epoch, "B stays");
                assert_eq!(
                    super::super::test_lifecycle_counters().candidate_withdrawn,
                    withdrawn,
                    "no step withdraws B"
                );
                let s_block = usize::try_from(s / QUANTUM).expect("block");
                assert_eq!(rig.swaps().len(), usize::from(adopted), "B waits for S");
                rig.render(1);
                assert_eq!(rig.swaps().last(), Some(&s_block), "B at S");
                let watermark = rig.children.plan.queries().watermark().expect("watermark");
                assert_eq!(watermark.transition_fallback, 0);
            }

            let mut rig = Rig::boot(Some(CONFIG), limits(), &slow);
            rig.ahead = 1;
            rig.render(BEFORE);
            assert!(matches!(
                rig.rebuild(&growth()),
                PlanAdoption::Primed { .. }
            ));
            let record = rig.primed().expect("W's record");
            let revision = rig.revision();
            rig.render(1);
            let held = rig.children.session.held_epochs().expect("held");
            let needs: u64 = held
                .as_slice()
                .iter()
                .map(|epoch| epoch.report.effect_scalar_state_bytes)
                .sum();
            let cap = rig.children.session.limits.maximum_effect_state_bytes;
            rig.children.session.limits.maximum_effect_state_bytes = needs - 1;
            assert!(rig.transact(&b).is_err(), "B is refused");
            rig.children.session.limits.maximum_effect_state_bytes = cap;
            assert_eq!(rig.primed(), Some(record), "W keeps its record");
            assert_eq!(rig.children.session.newest_plan_epoch(), record.epoch);
            rig.render_to(record.not_before + DEADLINE - QUANTUM);
            assert_eq!(rig.deadline(), None);
            rig.render(1);
            assert_eq!(rig.deadline(), Some(TransitionStep::Published));
            let Some(PlanAdoption::NoEarlierThan(s)) =
                rig.children.session.test_take_published_adoption()
            else {
                panic!("the transition is a duck-swap")
            };
            let s_block = usize::try_from(s / QUANTUM).expect("block");
            rig.render(s_block + 1 - rig.blocks.len());
            assert_eq!(rig.swaps(), [s_block]);
            let watermark = rig.children.plan.queries().watermark().expect("watermark");
            assert_eq!(
                (watermark.revision, watermark.outcome_flags),
                (revision, OUTCOME_TRANSITION_FALLBACK)
            );
        }

        /// Gate 1, the admission: a warm candidate C is admitted against the reprepare peak. With
        /// the effect-state cap at `R + 2C - 1` the growth is refused before commit with that
        /// cap's diagnostic, and nothing is published; at `R + 2C` it is published `Primed`.
        /// Test value: a warm candidate admitted at the single peak passes one byte below.
        #[test]
        fn a_warm_candidate_is_admitted_with_the_reprepare_peak() {
            let mut probe = Rig::new(Some(CONFIG));
            probe.render(BEFORE);
            probe.rebuild(&growth());
            let held = probe.children.session.held_epochs().expect("held");
            let [running, candidate] = held.as_slice() else {
                panic!("the running plan and the candidate")
            };
            let (r, c) = (
                running.report.effect_scalar_state_bytes,
                candidate.report.effect_scalar_state_bytes,
            );
            assert!(c > 0, "the limiter holds state");
            for (cap, expected) in [(r + 2 * c - 1, false), (r + 2 * c, true)] {
                let mut rig = Rig::with_limits(
                    Some(CONFIG),
                    ControlLimits {
                        maximum_effect_state_bytes: cap,
                        ..limits()
                    },
                );
                rig.render(BEFORE);
                let epoch = rig.children.session.newest_plan_epoch();
                match rig.transact(&growth()) {
                    Ok(path) => {
                        assert!(expected, "admitted at {cap}");
                        assert_eq!(path, EditPath::Rebuild);
                        assert!(matches!(
                            rig.children.session.test_take_published_adoption(),
                            Some(PlanAdoption::Primed { .. })
                        ));
                    }
                    Err(CommandError::CompileRejected(failure)) => {
                        assert!(!expected, "refused at {cap}");
                        assert_eq!(&*failure.diagnostic(), b"effect.resource.limit\t$\n");
                        assert_eq!(rig.children.session.newest_plan_epoch(), epoch);
                        assert_eq!(rig.primed(), None);
                    }
                    Err(other) => panic!("{other:?}"),
                }
            }
        }

        /// Gate 2, routing on the duck set (rewritten by #1397 D1, deliverable 5): the growth with
        /// track 2's removal ducks track 2, so it is published `Primed` no earlier than the duck's
        /// `S`, its record names its epoch, and no block before `S` differs from A continued with
        /// track 2 live-muted at the same block. Test value: a classifier that routes on
        /// `restarted_strips()` alone publishes it at the render clock, and one that takes today's
        /// path publishes it `NoEarlierThan(S)` with no record, so no deadline step can find it.
        #[test]
        fn a_growth_that_removes_a_strip_is_primed_at_the_ducks_sample() {
            let mut rig = Rig::new(Some(CONFIG));
            rig.render(BEFORE);
            let mut edits = growth();
            edits.extend(remove_track("eq1"));
            let s = duck_sample(rig.children.session.test_render_clock());
            assert_eq!(
                rig.rebuild(&edits),
                PlanAdoption::Primed {
                    not_before: s,
                    lead_blocks: 4
                }
            );
            assert_eq!(
                rig.primed(),
                Some(PrimedCandidate {
                    epoch: rig.children.session.newest_plan_epoch(),
                    not_before: s,
                    lead_blocks: 4,
                })
            );
            let s_block = usize::try_from(s / QUANTUM).expect("block");
            assert!(s_block > BEFORE);
            rig.render(s_block - BEFORE);
            let mut continued = Rig::new(None);
            continued.render(BEFORE);
            assert!(matches!(
                continued.transact(&[builtin("eq1", 6, 1.0)]),
                Ok(EditPath::Live)
            ));
            continued.render(s_block - BEFORE);
            assert_eq!(first_divergence(&rig, &continued), None);

            // A structural edit superseding a `Primed` W whose overlay ducks a strip (W is the
            // growth with track 2's removal): B still grows over A, so it takes the transition,
            // no earlier than W's `S`, with no record, and reports `TRANSITION_FALLBACK`. Red on
            // attempt 1's routing, which published B as an ordinary successor (`EXACT`).
            let mut rig = Rig::new(Some(CONFIG));
            rig.render(BEFORE);
            let mut edits = growth();
            edits.extend(remove_track("eq1"));
            let PlanAdoption::Primed { not_before: s, .. } = rig.rebuild(&edits) else {
                panic!("W is warm")
            };
            let adoption = rig.rebuild(&muted_track("t4", 0));
            let PlanAdoption::NoEarlierThan(later) = adoption else {
                panic!("published {adoption:?}")
            };
            assert!(later >= s);
            assert_eq!(rig.primed(), None);
            let s_block = usize::try_from(later / QUANTUM).expect("block");
            rig.render(s_block + 1 - rig.blocks.len());
            assert_eq!(rig.swaps(), [s_block]);
            let watermark = rig.children.plan.queries().watermark().expect("watermark");
            assert_ne!(watermark.outcome_flags & OUTCOME_TRANSITION_FALLBACK, 0);

            // The growth while phase 1 of a removal is pending (#1397a attempt 2, root ruling):
            // it takes the transition, which ducks track 1 too (the output's arrival grows) and
            // inherits the overlay's `S`, with no record. Up to its `S'` the output is A with
            // track 2 and then track 1 live-muted at the two edits' blocks; from `S'`, for 64
            // blocks, a fresh plan of the committed session with track 1 muted, fed from frame
            // `S'` and live-unmuted at once; the adoption reports `TRANSITION_FALLBACK`. Red on an
            // ordinary successor, which carries track 1 across its changed line (`EXACT`).
            let mut rig = Rig::new(Some(CONFIG));
            rig.render(BEFORE);
            let PlanAdoption::NoEarlierThan(s) = rig.rebuild(&remove_track("eq1")) else {
                panic!("a two-phase removal")
            };
            rig.render(1);
            let adoption = rig.rebuild(&growth());
            let PlanAdoption::NoEarlierThan(later) = adoption else {
                panic!("published {adoption:?}")
            };
            assert!(later >= s);
            assert_eq!(rig.primed(), None);
            let s_block = usize::try_from(later / QUANTUM).expect("block");
            rig.render(s_block + 64 - rig.blocks.len());
            assert_eq!(rig.swaps(), [s_block]);
            let watermark = rig.children.plan.queries().watermark().expect("watermark");
            assert_ne!(watermark.outcome_flags & OUTCOME_TRANSITION_FALLBACK, 0);
            let mut continued = Rig::new(None);
            continued.render(BEFORE);
            for track in ["eq1", "eq0"] {
                assert!(matches!(
                    continued.transact(&[builtin(track, 6, 1.0)]),
                    Ok(EditPath::Live)
                ));
                continued.render(1);
            }
            continued.render(s_block - BEFORE - 2);
            assert_eq!(rig.blocks[..s_block], continued.blocks[..], "A continued");
            let mut muted = rig
                .children
                .session
                .controller
                .session()
                .compiled()
                .normalized_model()
                .clone();
            for track in &mut muted.tracks {
                if track.id.as_str() == "eq0" {
                    track.fader.left_mute = true;
                    track.fader.right_mute = true;
                }
            }
            let mut fresh = Rig::boot(None, limits(), &muted);
            let offset = usize::try_from(later).expect("frames");
            for (_, planes, _) in &mut fresh.feeds {
                for plane in planes.iter_mut() {
                    plane.drain(..offset);
                }
            }
            assert!(matches!(
                fresh.transact(&[builtin("eq0", 6, 0.0)]),
                Ok(EditPath::Live)
            ));
            fresh.render(64);
            assert_eq!(
                rig.blocks[s_block..],
                fresh.blocks[..],
                "the fresh plan from S'"
            );
        }

        /// Gate 3, edits while W is pending, frames withheld. A live fader edit on track 2 goes
        /// to W's cells and applies at `S`, as the same value written to A's cell just before
        /// block `S`. A structural edit that removes track 2's source supersedes W: the source's
        /// producer is back in the running epoch, its submits are accepted and A plays them with
        /// no underrun, and the adoption reports `EXACT | SUPERSEDED`. A declared stop supersedes
        /// W, which folded in nothing: its revision completes `EXACT` alone.
        ///
        /// Test value: a supersession that drops W's producers with W leaves the removed source
        /// with no producer, so its submits are refused and A underruns; a stop reported
        /// `SUPERSEDED` for a candidate that folded in nothing miscounts it.
        #[test]
        fn edits_while_the_warm_candidate_waits() {
            // A live edit.
            let mut rig = Rig::new(Some(CONFIG));
            rig.ahead = 1;
            rig.render(BEFORE);
            rig.rebuild(&growth());
            rig.render(1);
            assert!(matches!(
                rig.transact(&[builtin("eq1", 5, -6.0)]),
                Ok(EditPath::Live)
            ));
            rig.render(2);
            rig.ahead = AHEAD;
            rig.render(8);
            let s = BEFORE + 3;
            assert_eq!(rig.swaps(), [s]);
            let mut continued = Rig::new(None);
            continued.render(s);
            assert!(matches!(
                continued.transact(&[builtin("eq1", 5, -6.0)]),
                Ok(EditPath::Live)
            ));
            continued.render(8);
            assert_eq!(first_divergence(&rig, &continued), None, "applied at S");

            // A structural edit that removes the carried source `aux-source`.
            let mut rig = Rig::new(Some(CONFIG));
            rig.ahead = 1;
            rig.render(BEFORE);
            rig.rebuild(&growth());
            rig.render(1);
            let mut edits: Vec<SessionEdit> = remove_track("eq1").into();
            edits.push(SessionEdit::RemoveSource {
                source_id: sid(AUX),
            });
            // #1397 D1 and gate 5: B still grows over A and ducks track 2, so it is `Primed` no
            // earlier than the duck's `S`. `aux-source` is fed exactly the frames A renders up to
            // `S` and none past them, the carried source its prime: B's publication cleared the
            // flag W set on the removed source's ring, so B is adopted at `S`, and up to it and
            // for 64 blocks on, the output is A continued with track 2 live-muted at the same
            // block. A `publish_primed` that leaves the withdrawn candidate's flags alone keeps
            // B waiting for frames the host never sends.
            let s = duck_sample(rig.children.session.test_render_clock());
            assert_eq!(
                rig.rebuild(&edits),
                PlanAdoption::Primed {
                    not_before: s,
                    lead_blocks: 4
                }
            );
            let s_block = usize::try_from(s / QUANTUM).expect("block");
            rig.stop_at = Some((AUX, s));
            rig.ahead = AHEAD;
            rig.render(s_block - rig.blocks.len());
            assert!(rig.swaps().is_empty(), "A renders until S");
            assert_eq!(rig.underruns(), 0, "A plays the removed source's frames");
            rig.render(1);
            assert_eq!(rig.swaps(), [s_block]);
            let watermark = rig.children.plan.queries().watermark().expect("watermark");
            assert_eq!(
                (watermark.outcome_flags, watermark.superseded),
                (OUTCOME_EXACT | OUTCOME_SUPERSEDED, 1)
            );
            rig.render(63);
            assert_eq!(rig.underruns(), 0);
            let mut continued = Rig::new(None);
            continued.render(BEFORE + 1);
            assert!(matches!(
                continued.transact(&[builtin("eq1", 6, 1.0)]),
                Ok(EditPath::Live)
            ));
            continued.render(rig.blocks.len() - BEFORE - 1);
            assert_eq!(first_divergence(&rig, &continued), None, "A continued");

            // A declared stop.
            let mut rig = Rig::new(Some(CONFIG));
            rig.ahead = 1;
            rig.render(BEFORE);
            rig.rebuild(&growth());
            rig.render(1);
            let revision = rig
                .children
                .session
                .test_controller()
                .session()
                .revision()
                .0;
            rig.children
                .session
                .declare_discontinuity()
                .expect("declaration");
            rig.render(1);
            assert_eq!(rig.swaps(), [BEFORE + 1]);
            let watermark = rig.children.plan.queries().watermark().expect("watermark");
            assert_eq!(
                (
                    watermark.revision,
                    watermark.outcome_flags,
                    watermark.superseded
                ),
                (revision, OUTCOME_EXACT, 0)
            );
        }

        /// #2044: a warm B that does not carry W's [`UNUSED`] ring panics after its primed
        /// publication wrote that ring's flag, before its commit. W is republished and, its own
        /// flags set again, waits while that ring is not fed, as it does when no B ran; once fed,
        /// it is adopted.
        ///
        /// Test value: red if the republish keeps B's flags: W is then judged without the ring
        /// it carries, so it is adopted before that ring is primed.
        #[test]
        fn a_republished_warm_candidate_keeps_its_own_prime_flags() {
            let mut b: Vec<SessionEdit> = muted_track("t4", 0).into();
            b.push(SessionEdit::RemoveSource {
                source_id: sid(UNUSED),
            });
            let run = |panics: bool| {
                let mut rig = Rig::with_unused(Some(CONFIG));
                rig.ahead = 1;
                rig.render(BEFORE);
                assert!(matches!(
                    rig.rebuild(&growth()),
                    PlanAdoption::Primed { .. }
                ));
                let record = rig.primed().expect("W's record");
                if panics {
                    rig.children.session.test_set_structural_faults([
                        Some(super::super::TestStructuralFaultPhase::PanicAtPrimedPublication),
                        None,
                    ]);
                    let caught =
                        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| rig.transact(&b)));
                    assert!(caught.is_err(), "B panics");
                    assert_eq!(rig.primed(), Some(record), "W keeps its record");
                    assert_eq!(rig.children.session.newest_plan_epoch(), record.epoch);
                }
                let unused = rig
                    .feeds
                    .iter()
                    .find(|(id, ..)| *id == UNUSED)
                    .map(|(.., next)| *next)
                    .expect("fed");
                rig.stop_at = Some((UNUSED, unused));
                rig.ahead = AHEAD;
                rig.render(6);
                let held = rig.swaps();
                rig.stop_at = None;
                rig.render(6);
                (held, rig.swaps().len())
            };
            let reference = run(false);
            assert_eq!(
                reference,
                (Vec::new(), 1),
                "W waits for the ring, then is adopted"
            );
            assert_eq!(run(true), reference, "the republished W");
        }

        /// Gate 4, the record's lifecycle: each step that ends W's pendency drops W's record,
        /// and a warm successor's publication records its own epoch; a refused supersession
        /// republishes W and keeps W's record unchanged, and W is then adopted.
        ///
        /// Test value: a record kept after W's adoption or supersession names W's epoch (#1358's
        /// deadline step would then act on B); a record dropped on a refused supersession leaves
        /// W pending with no record; a `not_before` read from the source-read clock is `P` late
        /// once a warm plan runs.
        #[test]
        fn the_record_names_the_one_pending_warm_candidate() {
            /// W published and pending, frames withheld; its record.
            fn pending() -> (Rig, PrimedCandidate) {
                let mut rig = Rig::new(Some(CONFIG));
                rig.ahead = 1;
                rig.render(BEFORE);
                rig.rebuild(&growth());
                rig.render(1);
                let record = rig.primed().expect("W's record");
                (rig, record)
            }
            let warm_b: Vec<SessionEdit> = muted_track("t4", 2).into();
            let plain_b: Vec<SessionEdit> = muted_track("t4", 0).into();

            // Adopted, then a `Next` B or a warm B.
            for (b, primed) in [(&plain_b, false), (&warm_b, true)] {
                let (mut rig, _) = pending();
                rig.ahead = AHEAD;
                rig.render(1);
                assert_eq!(rig.swaps(), [BEFORE + 1]);
                assert_eq!(rig.primed(), None, "adopted");
                let adoption = rig.rebuild(b);
                assert_eq!(matches!(adoption, PlanAdoption::Primed { .. }), primed);
                // W runs `P` ahead on the source-read clock: B's `not_before` is the render
                // clock's.
                let render_clock = rig.children.session.test_render_clock();
                assert_eq!(
                    rig.children.session.test_source_read_clock(),
                    render_clock + LEAD
                );
                assert_eq!(
                    rig.primed().map(|record| (record.epoch, record.not_before)),
                    primed.then(|| (rig.children.session.newest_plan_epoch(), render_clock)),
                    "after an adopted W, B warm: {primed}"
                );
            }

            // Superseded by a `Next` B (W's growth reverted) or a warm B.
            let mut reverted: Vec<SessionEdit> = remove_track("t3").into();
            reverted.extend(plain_b.iter().cloned());
            for (b, primed) in [(&reverted, false), (&plain_b, true)] {
                let (mut rig, record) = pending();
                let adoption = rig.rebuild(b);
                assert_eq!(matches!(adoption, PlanAdoption::Primed { .. }), primed);
                let newest = rig.children.session.newest_plan_epoch();
                assert_ne!(newest, record.epoch);
                assert_eq!(
                    rig.primed().map(|record| record.epoch),
                    primed.then_some(newest),
                    "W superseded, B warm: {primed}"
                );
            }
            // The warm B's admission at the effect-state cap `max(R + W + B, R + 2B)`: its
            // reprepare row is the running plan alone, never the withdrawn W.
            let state = |rig: &Rig| -> Vec<u64> {
                let held = rig.children.session.held_epochs().expect("held");
                held.as_slice()
                    .iter()
                    .map(|epoch| epoch.report.effect_scalar_state_bytes)
                    .collect()
            };
            let (mut probe, _) = pending();
            let [r, w] = state(&probe)[..] else {
                panic!("R and W")
            };
            probe.rebuild(&plain_b);
            let [_, b] = state(&probe)[..] else {
                panic!("R and B")
            };
            let (mut rig, _) = pending();
            rig.children.session.limits.maximum_effect_state_bytes = (r + w + b).max(r + 2 * b);
            assert!(
                matches!(rig.rebuild(&plain_b), PlanAdoption::Primed { .. }),
                "B fits beside R and W, and twice beside R"
            );

            // A warm B superseding a W whose overlay holds only a removed source no track read
            // (#1397 D1, #1403 NIT-1): the overlay ducks no strip, so B is warm and recorded.
            let mut rig = Rig::with_unused(Some(CONFIG));
            rig.ahead = 1;
            rig.render(BEFORE);
            let mut edits = growth();
            edits.push(SessionEdit::RemoveSource {
                source_id: sid(UNUSED),
            });
            rig.rebuild(&edits);
            rig.render(1);
            let render_clock = rig.children.session.test_render_clock();
            assert_eq!(
                rig.rebuild(&plain_b),
                PlanAdoption::Primed {
                    not_before: render_clock,
                    lead_blocks: 4
                },
                "B over a source-only overlay"
            );
            assert_eq!(
                rig.primed().map(|record| record.epoch),
                Some(rig.children.session.newest_plan_epoch())
            );

            // A declared stop.
            let (mut rig, _) = pending();
            rig.children
                .session
                .declare_discontinuity()
                .expect("declaration");
            assert_eq!(rig.primed(), None, "a stop");

            // A refused supersession: B's admission does not fit the effect-state cap.
            let (mut rig, record) = pending();
            let held = rig.children.session.held_epochs().expect("held");
            rig.children.session.limits.maximum_effect_state_bytes = held
                .as_slice()
                .iter()
                .map(|epoch| epoch.report.effect_scalar_state_bytes)
                .sum();
            let epoch = rig.children.session.newest_plan_epoch();
            assert!(rig.transact(&warm_b).is_err(), "B is refused");
            assert_eq!(rig.children.session.newest_plan_epoch(), epoch);
            assert_eq!(rig.primed(), Some(record), "W keeps its record");
            rig.ahead = AHEAD;
            rig.render(1);
            assert_eq!(rig.swaps(), [BEFORE + 1], "W is still pending");
            assert_eq!(rig.primed(), None);
        }

        /// #1355 MAJOR-1 at the control plane: render adopts the `Primed` W in a call it then
        /// refuses, so W never renders; the next structural edit's successor C, prepared from
        /// W's inventory and published `Next`, displaces W at its first block, and the output
        /// equals A continued. Test value: a carry that takes W's lines and rings without its
        /// pending prime shifts every carried track by `P` from C's adoption.
        #[test]
        fn a_primed_candidate_displaced_by_a_refused_call_continues() {
            let mut rig = Rig::new(Some(CONFIG));
            rig.render(BEFORE);
            assert!(matches!(
                rig.rebuild(&growth()),
                PlanAdoption::Primed { .. }
            ));
            rig.refused_call();
            assert_eq!(
                rig.rebuild(&muted_track("t4", 0)),
                PlanAdoption::Next,
                "C grows nothing over W"
            );
            assert_eq!(rig.primed(), None, "W was adopted");
            rig.render(10);
            assert_eq!(rig.swaps(), [BEFORE]);
            let mut continued = Rig::new(None);
            continued.render(rig.blocks.len());
            assert_eq!(first_divergence(&rig, &continued), None);
        }
    }
}
