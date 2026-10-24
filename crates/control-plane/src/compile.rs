//! The control plane's own resource projection, and the children a compile returns.

use super::*;

/// What [`compile_children`] returns: the control-protocol session and the render plan it drives.
pub struct CompiledChildren<A: ControlAdapter> {
    /// The control-thread session.
    pub session: SessionState<A>,
    /// The render-thread plan.
    pub plan: PlanState<A>,
}

pub(crate) struct PreparedRuntime {
    pub(crate) sources: SourceControlSet,
    pub(crate) strips: StripLanes,
    /// One live-control producer per prepared effect instance (#1263 D2).
    pub(crate) effects: Box<[host_core::EffectControlProducer]>,
    /// The plan's meter consumers (#1581 D6), one per strip in strip order: kept, not dropped.
    pub(crate) meters: Box<[host_core::MeterConsumer]>,
    /// The plan's effect observation readers (#1584 D7), one per observed effect instance in
    /// `HostLiveControlHandles::effect_observations` order: kept, not dropped. Empty for a host
    /// whose preparer takes them for itself (the browser's).
    pub(crate) observations: Box<[host_core::EffectObservationHandle]>,
    pub(crate) plan: PreparedRenderPlan,
    /// The plan's one revision-gate handle, taken before the plan is published (#1502 D6).
    pub(crate) revision_gate: PlanRevisionGate,
    pub(crate) resources: PlanResources,
    /// The part of `resources`' source rows the plan carries from its predecessor.
    pub(crate) carried: CarriedSourceBytes,
    /// What the plan holds, for preparing its successor (issue #1273 D1).
    pub(crate) inventory: PlanStateInventory,
    pub(crate) control_catalog: ControlCatalog,
    pub(crate) capi: CapiResources,
    /// The captures of the spectrum observers the plan was prepared with (#1593 D3), or `None`.
    pub(crate) spectrum: Option<host_core::PreparedSpectrumCaptures>,
}

/// A zeroed byte buffer of exactly `bytes` bytes, allocated fallibly.
pub(crate) fn boxed_zeroed(bytes: u64) -> Result<Box<[u8]>, CompileFailure> {
    let capacity = usize::try_from(bytes).map_err(|_| failure(ResourceFault::Platform))?;
    let mut buffer = Vec::new();
    buffer
        .try_reserve_exact(capacity)
        .map_err(|_| failure(ResourceFault::Allocation))?;
    buffer.resize(capacity, 0);
    Ok(buffer.into_boxed_slice())
}

pub(crate) fn checked_layout<T>(count: usize) -> Result<u64, CompileFailure> {
    let layout = Layout::array::<T>(count).map_err(|_| failure(ResourceFault::Arithmetic))?;
    u64::try_from(layout.size()).map_err(|_| failure(ResourceFault::Platform))
}

pub(crate) fn checked_byte_layout(bytes: u64) -> Result<u64, CompileFailure> {
    checked_layout::<u8>(usize::try_from(bytes).map_err(|_| failure(ResourceFault::Platform))?)
}

/// The control plane's own retained rows for one prepared plan.
///
/// Public only to the adapters' tests, under `test-support`; crate-private otherwise.
#[derive(Clone, Copy)]
#[cfg_attr(not(feature = "test-support"), allow(unreachable_pub))]
pub struct CapiResources {
    /// Every retained byte while the plan is active.
    pub active_retained: u64,
    /// The bytes each epoch retains on its own: control tables, inventory, producers.
    pub epoch_retained: u64,
    /// The bytes a prepared structural command retains until it commits.
    pub prepared_protocol_retained: u64,
    /// The largest single allocation among them.
    pub largest: u64,
    /// The kept bases' edit-ramp tables, part of `active_retained` (#1696 root ruling): every kept
    /// base at most, each at the newest committed model's bounds. Not an epoch row, since a kept
    /// base's ramps follow the newest commit, not the epoch that holds them; a live commit moves it
    /// with the committed model (`follow_edit_ramps`).
    pub edit_ramps: u64,
    /// The session's spectrum service, analysis and one-shot result (#1594 R6, #1595 R7), part of
    /// `active_retained`: one per session, beside the plan whose captures it reads. Not an epoch
    /// row: a successor that keeps spectrum keeps the same service, so the replacement peak adds
    /// only a candidate's bytes beyond the newest plan's.
    pub spectrum_service: u64,
}

/// Source-ring bytes a successor plan carries from the plan it displaces (issue #1273 D5).
///
/// The successor's [`PlanResources`] count them in its source rows, because it owns those
/// rings once active; until the swap the running plan owns them, so the double-live admission
/// subtracts them from the successor's rows and counts each carried ring once. A pending
/// candidate keeps its own (`LiveEpochResources::carried`), so a ring two held plans and the
/// candidate share is counted once across all three (#1398 D4, batch 15a review fold M2).
///
/// Public only to the adapters' tests, under `test-support`; crate-private otherwise.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(not(feature = "test-support"), allow(unreachable_pub))]
pub struct CarriedSourceBytes {
    /// The carried rings' share of `source_total_bytes`.
    pub total: u64,
    /// The carried rings' share of `source_overhead_bytes`.
    pub overhead: u64,
}

/// Every compiled model's charge while a prospective session lives beside the ones the control
/// plane holds (#1398 D4): the committed model, the model an epoch keeps while a successor of it
/// is pending (#1310 D3),
/// and the prospective one.
///
/// Public only to the adapters' tests, under `test-support`; crate-private otherwise.
#[derive(Clone, Copy)]
#[cfg_attr(not(feature = "test-support"), allow(unreachable_pub))]
pub struct CompiledModelAdmission {
    /// Every model's retained bytes.
    pub retained_bytes: u64,
    /// The largest of the models' largest single allocation.
    pub largest_allocation_bytes: u64,
}

/// [`CompiledModelAdmission`] over every model the control plane holds, `held`, and the
/// prospective one (#1398 D4): their retained bytes summed, and the largest of their largest
/// allocations.
pub(crate) fn compiled_model_admission_over(
    held: &[session::ResourceEstimate],
    prospective: session::ResourceEstimate,
) -> Result<CompiledModelAdmission, CompileFailure> {
    held.iter().try_fold(
        CompiledModelAdmission {
            retained_bytes: prospective.compiled_model_bytes,
            largest_allocation_bytes: prospective.single_allocation_bytes,
        },
        |admission, model| {
            Ok(CompiledModelAdmission {
                retained_bytes: admission
                    .retained_bytes
                    .checked_add(model.compiled_model_bytes)
                    .ok_or_else(|| failure(ResourceFault::Arithmetic))?,
                largest_allocation_bytes: admission
                    .largest_allocation_bytes
                    .max(model.single_allocation_bytes),
            })
        },
    )
}

/// What the committed model's control catalog adds to a live commit's peak (#1558 D6).
///
/// A live commit replaces the provider's catalog by the committed model's, and a VCA added live
/// adds rows to it (and a VCA replaced by one with a longer ID, longer strings), so the new
/// catalog can be larger than the one the plans were prepared with. The epochs' retained rows hold
/// the current catalog once, and the prepared-protocol row once more as the stand-in for a
/// successor's; this charges the growth beyond the current catalog, and the new catalog's largest
/// single allocation, which no row holds yet.
///
/// Public only to the adapters' tests, under `test-support`; crate-private otherwise.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(not(feature = "test-support"), allow(unreachable_pub))]
pub struct LiveCatalogAdmission {
    /// The new catalog's retained bytes less the current catalog's, at least zero.
    pub growth_bytes: u64,
    /// The new catalog's largest single allocation.
    pub largest_allocation_bytes: u64,
    /// The committed model's edit-ramp row beyond the newest epoch's, at least zero (#1696 root
    /// ruling): a VCA added live raises the bound of the ramps a kept base may hold.
    pub ramp_growth_bytes: u64,
}

impl CapiResources {
    /// This epoch's rows after a live commit replaced the provider's catalog of `previous`'s
    /// figures by `committed`'s: both rows that hold a catalog follow it, and the largest
    /// allocation only grows. Infallible, as the commit's last steps are.
    pub(crate) fn follow_catalog(
        &mut self,
        previous: host_core::CatalogResources,
        committed: host_core::CatalogResources,
    ) {
        let follow = |row: u64| {
            row.saturating_sub(previous.retained_bytes)
                .saturating_add(committed.retained_bytes)
        };
        self.active_retained = follow(self.active_retained);
        self.prepared_protocol_retained = follow(self.prepared_protocol_retained);
        self.largest = self.largest.max(committed.largest_allocation_bytes);
    }

    /// This epoch's rows after a live commit moved the edit-ramp bound to `committed`, the
    /// committed model's [`MonitorRows::edit_ramps`] (#1696 root ruling): a VCA added live is
    /// charged at once. Infallible, as the commit's last steps are.
    pub(crate) fn follow_edit_ramps(&mut self, committed: u64) {
        self.active_retained = self
            .active_retained
            .saturating_sub(self.edit_ramps)
            .saturating_add(committed);
        self.edit_ramps = committed;
    }
}

impl LiveCatalogAdmission {
    /// The charge of replacing the catalog whose figures are `current` by the one of `new`.
    pub(crate) fn between(
        current: host_core::CatalogResources,
        new: host_core::CatalogResources,
    ) -> Self {
        Self {
            growth_bytes: new.retained_bytes.saturating_sub(current.retained_bytes),
            largest_allocation_bytes: new.largest_allocation_bytes,
            ramp_growth_bytes: 0,
        }
    }
}

#[repr(C)]
pub(crate) struct SharedArcAllocation<T> {
    pub(crate) strong: core::sync::atomic::AtomicUsize,
    pub(crate) weak: core::sync::atomic::AtomicUsize,
    pub(crate) value: T,
}

#[allow(dead_code)]
pub(crate) enum RetainedDiagnosticSlotMirror {
    Empty,
    Owned(protocol::Diagnostic),
}

pub(crate) fn checked_sum(rows: &[u64]) -> Result<u64, CompileFailure> {
    rows.iter().try_fold(0_u64, |total, row| {
        total
            .checked_add(*row)
            .ok_or_else(|| failure(ResourceFault::Arithmetic))
    })
}

pub(crate) fn protocol_queue_config(
    limits: ControlLimits,
) -> Result<ProtocolQueueConfig, CompileFailure> {
    let one = NonZeroUsize::new(1).expect("one is nonzero");
    Ok(ProtocolQueueConfig {
        control_command_slots: one,
        control_command_bytes: NonZeroUsize::new(
            usize::try_from(limits.maximum_control_frame_bytes)
                .map_err(|_| failure(ResourceFault::Platform))?,
        )
        .ok_or_else(|| failure(ResourceFault::Limit))?,
        reliable_response_slots: one,
        reliable_event_slots: NonZeroUsize::new(RELIABLE_EVENT_SLOTS).expect("two is nonzero"),
        telemetry_slots: one,
    })
}

/// The reliable event queue's slots: the transaction's `SESSION_COMMITTED` and its one
/// `OBSERVATION_ENDED` (#1584 D6a).
const RELIABLE_EVENT_SLOTS: usize = 2;

/// The plan-resource report table's rows (#1398 D1): a lagging epoch's row (the atomic names a
/// plan render has already displaced), the running plan's, a withdrawn candidate's and its
/// successor's.
pub(crate) const REPORT_ROWS: usize = 4;

/// The pending provider epochs (#1398 D1): a withdrawn candidate's and its successor's coexist
/// until the former is dropped (#1310 D5).
pub(crate) const PENDING_PROVIDERS: usize = 2;

/// The kept-base ramp tables that can be live at once (#1696 root ruling): two, and not derived
/// from [`PENDING_PROVIDERS`]. At most one kept base is stored: a rebuild creates it only when
/// nothing else is pending, and promotion drops it. The second table is its replacement, which a
/// commit builds (`merged_ramps`, then `keep_within`) while the stored one still lives, and moves
/// in once the commit cannot fail. Both are pruned to the newest committed model.
pub(crate) const KEPT_BASES: usize = 2;

/// The provider epochs parked until render's displaced plan is reclaimed (#1398 D1): one per
/// retirement credit, because each parked epoch's plan holds one until the service step reclaims
/// it (batch 15a review fold 2, B-1). With two, a `Taken` withdrawal in each of two commands while
/// render lagged its retirements parked two epochs, and a third promotion found the list full.
pub(crate) const RETIRED_PROVIDERS: usize = RETIREMENT_CAPACITY;

/// The plan exchange's retirement credits (#1398 D1), the exchange and its resource report alike:
/// a displaced plan not yet reclaimed, a withdrawn candidate's credit (held until the candidate
/// is dropped or republished) and its successor's reservation.
pub(crate) const RETIREMENT_CAPACITY: usize = 3;

pub(crate) fn retirement_capacity() -> NonZeroUsize {
    NonZeroUsize::new(RETIREMENT_CAPACITY).expect("three is nonzero")
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn capi_resources<A: ControlAdapter>(
    limits: ControlLimits,
    source_count: usize,
    source_id_bytes: usize,
    strip_table_bytes: u64,
    provider: host_core::SessionControlProviderResources,
    inventory_bytes: u64,
    effect_controls: host_core::EffectControlResources,
    route_controls: host_core::RouteControlResources,
    observation: ObservationResources,
    monitor: MonitorRows,
) -> Result<CapiResources, CompileFailure> {
    let queue_config = protocol_queue_config(limits)?;
    let queue = ProtocolQueues::resource_report_for_config(queue_config)
        .map_err(|_| failure(ResourceFault::Arithmetic))?;
    let replay_config = ReplayCacheConfig {
        entries: NonZeroUsize::new(
            usize::try_from(limits.maximum_replay_entries)
                .map_err(|_| failure(ResourceFault::Platform))?,
        )
        .ok_or_else(|| failure(ResourceFault::Limit))?,
        bytes: NonZeroUsize::new(
            usize::try_from(limits.maximum_replay_bytes)
                .map_err(|_| failure(ResourceFault::Platform))?,
        )
        .ok_or_else(|| failure(ResourceFault::Limit))?,
        max_response_bytes: usize::try_from(limits.maximum_control_frame_bytes)
            .map_err(|_| failure(ResourceFault::Platform))?,
    };
    let replay = ReplayCache::resource_report_for_config(replay_config)
        .map_err(|_| failure(ResourceFault::Arithmetic))?;
    let output_meter = output_meter_rows()?;
    let exchange = plan_exchange_resource_report(PlanExchangeConfig {
        retirement_capacity: retirement_capacity(),
    })
    .map_err(|_| failure(ResourceFault::Arithmetic))?;
    // The control-source table and ID arena are the facade's own layout; capi reads the mirror
    // (`control_table_bytes` / `source_id_arena_bytes`) rather than restating the struct, so this
    // pre-flight cannot drift when that struct changes.
    //
    // The session's canonical JSON is not a capi row. The compiled session owns it, and its
    // `compiled_model_bytes` already charges it to the graph cap, current and prospective alike
    // (`validate_replacement_peak`); charging it here too counted one allocation twice (#1060).
    //
    // The strip producer table (#1256 D3) is the epoch's too, through host-core's mirror. Its
    // queues and cells are builtins' rows, charged in `builtin_retained_payload_bytes`. Builtins' processor
    // accumulator also charges the producer vector itself (`add_vector_layout::<
    // TrackControlProducer>`), so that vector is charged twice, here against the capi cap and
    // there against the builtin cap: #1256 D4's recorded, conservative double charge. Removing
    // either side is a deliberate decision, not a cleanup. Each producer's own heap owners (the
    // automated faders' offsets-writer boxes, #1916) are charged here only, through the producers'
    // `retained_heap_bytes`.
    //
    // The effect producer table and its owned payload (#1263 D3) are the epoch's too: capi keeps
    // the producers, and with them each parametric EQ's prepared-target owner. host-core walks
    // them over the built producers (`HostPrepareReport::effect_control_resources`). Their rings
    // and target staging are the graph estimate's rows (`effect_control_resource`), so they are
    // not charged again here. The payload is admitted twice all the same: host-core's preparation
    // admission also adds `effect_control_resources.total_bytes()` to the graph row and the
    // compiled model against `maximum_graph_session_plus_plan_bytes`. That is a recorded,
    // conservative double admission (#1263 verdict MINOR 1), like #1256 D4's above: the initial
    // compile needs graph row + compiled model + this payload under the graph cap.
    let effect_row = effect_controls
        .total_bytes()
        .ok_or_else(|| failure(ResourceFault::Arithmetic))?;
    // The send producers (#1557 D5): `graph::route_control_resources` of the attached lanes, the
    // producer table, the route IDs, the cells and the render-side owners. As the effect row
    // above, a conservative upper bound: host-core's preparation admission adds the same total to
    // the graph row, and the plan-side parts are charged here too. Zero without a route lane.
    let route_row = route_controls.total_bytes;
    let control_table_row = host_core::control_table_bytes(source_count)
        .ok_or_else(|| failure(ResourceFault::Arithmetic))?;
    let source_id_row = host_core::source_id_arena_bytes(source_id_bytes)
        .ok_or_else(|| failure(ResourceFault::Arithmetic))?;
    let epoch_rows = [
        control_table_row,
        source_id_row,
        // Issue #1273 D5: each epoch keeps its plan's state inventory for the successor.
        inventory_bytes,
        strip_table_bytes,
        effect_row,
        route_row,
        // #1584 D7: the epoch's observation reader handles: the boxed slice, each handle's ID text
        // and its boxed readers. The readers' slots and the lanes they read are the plan's own
        // rows (`observation_retained_bytes`).
        observation.handle_bytes,
        // #1696 D3: the monitor overlay, at this epoch's model bounds. The newest epoch's row
        // covers the committed overlay; an older epoch's covers the overlay its kept base holds
        // (`KeptBase::monitor`, the overlay committed on that epoch's model: no live commit
        // changes a strip). The transient (`MonitorOverlay::after`'s result until its commit) is
        // the same row one commit ahead and is not charged twice. The edit ramps are not an epoch
        // row: see `CapiResources::edit_ramps`.
        monitor.overlay,
    ];
    let ObservationReserve {
        ended_event,
        cell_heap,
        bindings_table,
        bindings_transient,
    } = observation_reserve(
        limits.maximum_control_frame_bytes,
        observation.reserve_slots,
    )?;
    let maximum_configuration_items = usize::try_from(limits.maximum_control_frame_bytes)
        .map_err(|_| failure(ResourceFault::Platform))?
        / size_of::<u16>();
    let fixed_allocation_rows = [
        checked_byte_layout(limits.maximum_diagnostic_bytes)?,
        checked_byte_layout(limits.maximum_control_frame_bytes)?,
        checked_byte_layout(limits.maximum_control_frame_bytes)?,
        checked_layout::<SharedArcAllocation<AtomicU64>>(1)?,
        // #1312 D2: the session's `live_values_superseded` count, one `Arc<AtomicU64>`.
        checked_layout::<SharedArcAllocation<AtomicU64>>(1)?,
        checked_layout::<SharedArcAllocation<SharedPlanState<A::Row>>>(1)?,
        // #1581 D4: the session's output meter (`output_meter_rows`). The meter-bytes cap charges
        // the same bytes (`prepare_caps`): a recorded, conservative double charge, as #1256 D4's.
        output_meter[0],
        output_meter[1],
        output_meter[2],
        checked_layout::<RetainedDiagnosticSlotMirror>(2)?,
        // #1584 D6a: the observation-ended cells, one more than the reliable queue's slots (an
        // event popped for egress keeps its cell until its frame is delivered), and the heap the
        // events in them own. A cell can hold the keys of any earlier plan's subscriptions (a
        // successor that removed producers still holds their keys), so the ceiling follows the
        // most slots any plan of the session has had, and the admitted keys' encoded bytes
        // bound their count and text (`ended_event_ceiling`, #1676 D2, #1685 D4), not this
        // plan's producers. The heap is allocated when a transaction prepares, never at the
        // commit.
        checked_layout::<Option<protocol::ObservationEnded>>(
            queue_config.reliable_event_slots.get() + 1,
        )?,
        checked_byte_layout(cell_heap)?,
        // #1584 D6a: the binding table the session keeps for the newest plan and the transient
        // copies `ObservationBindings::after` makes of it while a rebuild prepares: at most one
        // binding per observation slot and no more than one ended event carries, kept or
        // ended, and as many protocol keys for the ended ones. A binding owns the text of its two
        // IDs (#1685 D4).
        checked_byte_layout(bindings_table)?,
        checked_byte_layout(bindings_transient)?,
        // #1584 D6a: the newest plan's pending-subscription slots and the first slot of each
        // producer (one entry past the last).
        checked_layout::<crate::control::ObservationSlot>(observation.slots)?,
        checked_layout::<u32>(observation.producers + 1)?,
        // #1676 batch 13b fold: the per-producer admission scratch, one count per producer.
        checked_layout::<crate::control::ObservationBatchCount>(observation.producers)?,
        checked_layout::<RenderDiagnosticSlot>(RENDER_DIAGNOSTIC_SLOTS)?,
        checked_layout::<u8>(RENDER_DIAGNOSTIC_CODE.len() * RENDER_DIAGNOSTIC_SLOTS)?,
        // ProtocolController and SessionControlProvider each retain their own telemetry config.
        checked_layout::<u32>(maximum_configuration_items)?,
        checked_layout::<protocol::CounterId>(maximum_configuration_items)?,
        provider.fixed_retained_bytes,
        checked_layout::<u32>(maximum_configuration_items)?,
        checked_layout::<protocol::CounterId>(maximum_configuration_items)?,
        // #1398 D1: the pending and retired provider lists, and the report table.
        checked_layout::<ProviderEpoch>(PENDING_PROVIDERS)?,
        checked_layout::<ProviderEpoch>(RETIRED_PROVIDERS)?,
        checked_layout::<(u64, A::Row)>(REPORT_ROWS)?,
    ];
    // #1309 D5: the adapter's own per-session handles (capi's `Session` and `Plan`) follow the
    // report table, where capi's two rows stood.
    let adapter_allocation_rows = A::ADAPTER_ALLOCATIONS;
    let fixed_aggregate_rows = [
        queue.retained_payload_bytes,
        replay.retained_payload_bytes,
        exchange.retained_payload_bytes,
    ];
    // The affine token (#1615): the response frame it stages (the measured length, never more than
    // a control frame), its own layout, and a copy of the request the commit writes into the
    // replay entry (never more than a control frame). It holds a plan against the committed replay
    // cache, not a clone of it, so no replay cache is charged here.
    let prepared_protocol_allocation_rows = [
        checked_byte_layout(limits.maximum_control_frame_bytes)?,
        checked_layout::<protocol::PreparedStructuralCommand>(1)?,
        checked_byte_layout(limits.maximum_control_frame_bytes)?,
        // The token holds the commit's one `OBSERVATION_ENDED` event until the commit moves it.
        checked_byte_layout(ended_event)?,
    ];
    let epoch_retained = checked_sum(&epoch_rows)?;
    let fixed_allocations = checked_sum(&fixed_allocation_rows)?
        .checked_add(checked_sum(adapter_allocation_rows)?)
        .ok_or_else(|| failure(ResourceFault::Arithmetic))?;
    let active_retained = fixed_allocations
        .checked_add(checked_sum(&fixed_aggregate_rows)?)
        .and_then(|value| value.checked_add(epoch_retained))
        .and_then(|value| value.checked_add(provider.catalog_retained_bytes))
        .and_then(|value| value.checked_add(monitor.edit_ramps))
        .ok_or_else(|| failure(ResourceFault::Arithmetic))?;
    let prepared_protocol_retained = checked_sum(&prepared_protocol_allocation_rows)?
        .checked_add(provider.catalog_retained_bytes)
        .ok_or_else(|| failure(ResourceFault::Arithmetic))?;
    // Conservative: the strip row is the producer slice plus every `track_id`, which are separate
    // allocations, so feeding the whole row overstates the largest single allocation when that
    // row is the maximum. The error only ever refuses earlier.
    // The effect row is several allocations; its own largest one stands for it.
    let largest = [
        control_table_row,
        source_id_row,
        strip_table_bytes,
        monitor.overlay_vector,
    ]
    .into_iter()
    .chain([
        effect_controls.largest_allocation_bytes(),
        route_controls.largest_allocation_bytes,
    ])
    .chain(fixed_allocation_rows)
    .chain(adapter_allocation_rows.iter().copied())
    .chain(prepared_protocol_allocation_rows)
    .chain([
        queue.largest_allocation_bytes,
        replay.largest_allocation_bytes,
        exchange.largest_allocation_bytes,
        provider.largest_allocation_bytes,
    ])
    .max()
    .unwrap_or(0);
    Ok(CapiResources {
        active_retained,
        epoch_retained,
        prepared_protocol_retained,
        largest,
        edit_ramps: monitor.edit_ramps,
        spectrum_service: 0,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn prepared_capi_resources<A: ControlAdapter>(
    compiled: &CompiledSession,
    catalog: &ControlCatalog,
    inventory: &PlanStateInventory,
    effect_controls: host_core::EffectControlResources,
    route_controls: host_core::RouteControlResources,
    observation: ObservationResources,
    limits: ControlLimits,
    strip_heap_bytes: u64,
) -> Result<CapiResources, CompileFailure> {
    let source_id_bytes =
        compiled
            .normalized_model()
            .sources
            .iter()
            .try_fold(0_usize, |total, source| {
                total
                    .checked_add(source.id.as_str().len())
                    .ok_or_else(|| failure(ResourceFault::Arithmetic))
            })?;
    // Every strip, tracks and submixes alike, carries one producer whose `track_id` is its ID.
    let (strip_count, strip_id_bytes) = compiled.normalized_model().strips().try_fold(
        (0_usize, 0_usize),
        |(count, bytes), strip| {
            bytes
                .checked_add(strip.id.as_str().len())
                .map(|bytes| (count + 1, bytes))
                .ok_or_else(|| failure(ResourceFault::Arithmetic))
        },
    )?;
    let provider = SessionControlProvider::resource_report(
        catalog,
        controller_retained_capacity(limits)?,
        RENDER_DIAGNOSTIC_SLOTS,
    )
    .map_err(|_| failure(ResourceFault::Arithmetic))?;
    let monitor = monitor_rows(compiled.normalized_model())?;
    capi_resources::<A>(
        limits,
        compiled.source_count(),
        source_id_bytes,
        host_core::strip_control_table_bytes(strip_count, strip_id_bytes, strip_heap_bytes)
            .ok_or_else(|| failure(ResourceFault::Arithmetic))?,
        provider,
        inventory.retained_bytes(),
        effect_controls,
        route_controls,
        observation,
        monitor,
    )
}

/// The monitor overlay's and the edit-ramp table's rows of one epoch (#1696 D1-D3), bounded by
/// the epoch's model, never by its edits.
#[derive(Clone, Copy)]
pub(crate) struct MonitorRows {
    /// `controls::MonitorOverlay::retained_ceiling`: the solo vector at its reserve, the soloed
    /// IDs' text and the master's.
    pub(crate) overlay: u64,
    /// The solo vector alone, one allocation, for the largest-allocation check.
    pub(crate) overlay_vector: u64,
    /// [`KEPT_BASES`] times `session::EditRamps::retained_ceiling` of the model's strips, routes
    /// and VCAs: the kept bases' ramp tables, pruned to this (the newest committed) model's owners
    /// whenever they are stored (#1696 root ruling).
    pub(crate) edit_ramps: u64,
}

/// The summed text of `ids`.
fn id_text_bytes<'a>(
    ids: impl Iterator<Item = &'a session::StableId>,
) -> Result<usize, CompileFailure> {
    ids.into_iter()
        .try_fold(0_usize, |total, id| total.checked_add(id.as_str().len()))
        .ok_or_else(|| failure(ResourceFault::Arithmetic))
}

pub(crate) fn monitor_rows(model: &session::SessionModel) -> Result<MonitorRows, CompileFailure> {
    let arithmetic = || failure(ResourceFault::Arithmetic);
    let strips = model.strips().count();
    let strip_id_bytes = id_text_bytes(model.strips().map(|strip| strip.id))?;
    let longest_strip_id = model
        .strips()
        .map(|strip| strip.id.as_str().len())
        .max()
        .unwrap_or(0);
    let route_id_bytes = id_text_bytes(model.routes.iter().map(|route| &route.id))?;
    let vca_id_bytes = id_text_bytes(model.vcas.iter().map(|vca| &vca.id))?;
    let to_bytes =
        |bytes: usize| u64::try_from(bytes).map_err(|_| failure(ResourceFault::Platform));
    Ok(MonitorRows {
        overlay: to_bytes(
            controls::MonitorOverlay::retained_ceiling(strips, strip_id_bytes, longest_strip_id)
                .ok_or_else(arithmetic)?,
        )?,
        overlay_vector: checked_layout::<session::StableId>(strips)?,
        edit_ramps: to_bytes(
            session::EditRamps::retained_ceiling(
                strips,
                model.routes.len(),
                model.vcas.len(),
                strip_id_bytes,
                route_id_bytes,
                vca_id_bytes,
            )
            .and_then(|table| table.checked_mul(KEPT_BASES))
            .ok_or_else(arithmetic)?,
        )?,
    })
}

/// The monitor overlay's epoch row and the kept bases' edit-ramp row of `model` (#1696), so the
/// C ABI's oracle can hold `capi_retained_bytes` and the admissions to them.
///
/// # Errors
///
/// The arithmetic overflowed.
#[cfg(feature = "test-support")]
pub fn monitor_and_ramp_bytes(model: &session::SessionModel) -> Result<(u64, u64), CompileFailure> {
    let rows = monitor_rows(model)?;
    Ok((rows.overlay, rows.edit_ramps))
}

/// What a plan's observation subscriptions retain (#1584 D6a).
#[derive(Clone, Copy)]
pub(crate) struct ObservationResources {
    /// Observation slots of the plan: one per declared tap of every producer that has a queue.
    /// The binding table holds at most one binding per slot.
    pub(crate) slots: usize,
    /// The slot count the ended-event cells and the binding table are charged for: the most
    /// observation slots any plan of this session has had (#1676 D2). A cell holds an event
    /// until its frame is delivered, so an event a larger earlier plan emitted can outlive that
    /// plan, and a plan with fewer slots must still charge the room for it.
    pub(crate) reserve_slots: usize,
    /// Effect producers of the plan.
    pub(crate) producers: usize,
    /// The heap of the observation reader handles the plan's epoch keeps (#1584 D7):
    /// [`observation_handle_bytes`].
    pub(crate) handle_bytes: u64,
}

/// The heap `handles` own (#1584 D7): the boxed slice, and each handle's strip and effect ID text
/// and its boxed readers. The readers' slots are `Arc`s shared with the plan's lanes, whose bytes
/// are the plan's `observation_retained_bytes`.
pub(crate) fn observation_handle_bytes(
    handles: &[host_core::EffectObservationHandle],
) -> Result<u64, CompileFailure> {
    let arithmetic = || failure(ResourceFault::Arithmetic);
    handles
        .iter()
        .try_fold(size_of_val(handles), |total, handle| {
            total
                .checked_add(handle.track_id.len())?
                .checked_add(handle.effect_id.len())?
                .checked_add(size_of_val(&*handle.readers))
        })
        .ok_or_else(arithmetic)
        .and_then(|bytes| u64::try_from(bytes).map_err(|_| failure(ResourceFault::Platform)))
}

/// The observation rows of `effects`, the newest plan's producers, and the `handle_bytes` of the
/// reader handles its epoch keeps.
pub(crate) fn observation_resources(
    effects: &[host_core::EffectControlProducer],
    handle_bytes: u64,
    slots_high_water: usize,
) -> Result<ObservationResources, CompileFailure> {
    let mut slots = 0_usize;
    for producer in effects {
        slots = slots
            .checked_add(producer.observe_capacity())
            .ok_or_else(|| failure(ResourceFault::Arithmetic))?;
    }
    Ok(ObservationResources {
        producers: effects.len(),
        handle_bytes,
        slots,
        reserve_slots: slots.max(slots_high_water),
    })
}

/// The encoded size of an `OBSERVATION_ENDED` frame as a function of its keys (#1685 D1, D2): the
/// frame with no key, plus each key's own bytes. A key's bytes are a fixed part and the text of its
/// strip ID and effect ID, each padded to the codec's eight-byte value alignment; the tap ID and the
/// section are fixed width, and so are the frame header, the revision and the event sequence. The
/// fixed parts are measured with the real codec when the session compiles, and the model is checked
/// against the codec there ([`observation_wire`]), so admission computes a key's size from its two
/// lengths alone, allocating nothing, in the browser worklet's message handler.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ObservationWire {
    /// The control frame limit: an event frame larger than this cannot be sent.
    frame: usize,
    /// The frame with no key.
    base: usize,
    /// A key's bytes besides the padded text of its two IDs.
    key_fixed: usize,
}

impl ObservationWire {
    /// The encoded bytes of one key whose strip ID and effect ID are `strip` and `effect` bytes
    /// long.
    pub(crate) const fn key_bytes(self, strip: usize, effect: usize) -> usize {
        self.key_fixed + strip.next_multiple_of(8) + effect.next_multiple_of(8)
    }

    /// The encoded bytes of the frame that carries keys of `keys` bytes in all, or `0` for no key
    /// (a commit that ends nothing emits no event).
    pub(crate) const fn event_bytes(self, keys: usize) -> usize {
        if keys == 0 {
            0
        } else {
            self.base.saturating_add(keys)
        }
    }

    /// Whether keys of `keys` bytes in all fit one frame.
    pub(crate) const fn fits(self, keys: usize) -> bool {
        self.event_bytes(keys) <= self.frame
    }

    /// The control frame limit.
    pub(crate) const fn frame(self) -> usize {
        self.frame
    }

    /// The bytes the keys of one frame may take in all.
    const fn room(self) -> usize {
        self.frame.saturating_sub(self.base)
    }

    /// The most keys one frame carries: every key at its shortest.
    const fn max_keys(self) -> usize {
        self.room() / self.key_bytes(1, 1)
    }
}

/// Measures [`ObservationWire`] for a control frame of `frame_bytes` with the real codec (#1685
/// D2): the frame with one key and with two keys of eight-byte IDs give the fixed parts, and the
/// model is then checked against the codec for IDs that need padding and for IDs at the grammar's
/// longest. A codec whose sizes the model does not predict fails the compile, so admission can never
/// trust a wrong size. Control thread, once per session.
pub(crate) fn observation_wire(frame_bytes: u64) -> Result<ObservationWire, CompileFailure> {
    let arithmetic = || failure(ResourceFault::Arithmetic);
    let codec = protocol::ProtocolCodec::default();
    let size = |lengths: &[(usize, usize)]| -> Option<usize> {
        let event = protocol::ObservationEnded {
            event_sequence: u64::MAX,
            // Keys are strictly ascending on the wire; the tap ID, fixed width, orders keys
            // whose IDs are equal.
            keys: lengths
                .iter()
                .enumerate()
                .map(|(tap, &(strip, effect))| protocol::ObservationTapKey {
                    strip_id: "a".repeat(strip),
                    section: protocol::ObservationSection::Inserts,
                    effect_id: "a".repeat(effect),
                    tap_id: u32::try_from(tap).unwrap_or(u32::MAX),
                })
                .collect(),
        };
        match codec.encode_event_frame_into(
            &protocol::TypedEventFrame {
                revision: protocol::SessionRevision(u64::MAX),
                payload: protocol::EventPayload::ObservationEnded(&event),
            },
            &mut [],
        ) {
            Err(protocol::EncodeError::OutputTooSmall { required }) => Some(required),
            Err(_) | Ok(_) => None,
        }
    };
    let one = size(&[(8, 8)]).ok_or_else(arithmetic)?;
    let two = size(&[(8, 8), (8, 8)]).ok_or_else(arithmetic)?;
    let key = two.checked_sub(one).ok_or_else(arithmetic)?;
    let wire = ObservationWire {
        frame: usize::try_from(frame_bytes).map_err(|_| failure(ResourceFault::Platform))?,
        base: one.checked_sub(key).ok_or_else(arithmetic)?,
        key_fixed: key.checked_sub(16).ok_or_else(arithmetic)?,
    };
    let longest = session::STABLE_ID_MAXIMUM_BYTES;
    for lengths in [
        // In wire order: strip, then effect (an ID of `a`s orders by its length).
        &[(1, 1)][..],
        &[(1, 127), (9, 16)],
        &[(15, longest), (longest, 7), (longest, longest)],
    ] {
        let keys = lengths
            .iter()
            .map(|&(strip, effect)| wire.key_bytes(strip, effect))
            .sum::<usize>();
        if size(lengths) != Some(wire.base + keys) {
            return Err(arithmetic());
        }
    }
    Ok(wire)
}

/// The largest of `f` over `0..=keys` (#1685 D4), for an `f` that is linear between its
/// breakpoints: none, the four a table never holds fewer than, the counts on each side of the point
/// where the frame, not the longest IDs, starts to bound the text ([`admitted_text`]), and `keys`;
/// each clipped to `keys`. A piecewise-linear function peaks at a breakpoint or an end.
fn peak_over_keys(
    wire: ObservationWire,
    keys: usize,
    f: impl Fn(usize) -> Option<usize>,
) -> Result<u64, CompileFailure> {
    // `admitted_text` turns where `count` longest IDs' text meets the frame's room less `count`
    // fixed parts; the integers on each side of it.
    let turn = wire.room() / (wire.key_fixed + 2 * session::STABLE_ID_MAXIMUM_BYTES);
    let mut peak = 0_usize;
    for count in [0, 4, turn, turn + 1, keys] {
        peak = peak.max(f(count.min(keys)).ok_or_else(|| failure(ResourceFault::Arithmetic))?);
    }
    u64::try_from(peak).map_err(|_| failure(ResourceFault::Platform))
}

/// The ID text `count` admitted keys can own, at most (#1685 D4): two IDs of the grammar's longest
/// each, and never more than the frame's room leaves after their fixed parts, because an ID's text
/// is at most its padded encoded length and every admitted key fits one frame together.
fn admitted_text(wire: ObservationWire, count: usize) -> Option<usize> {
    let longest = count.checked_mul(2 * session::STABLE_ID_MAXIMUM_BYTES)?;
    let framed = wire
        .room()
        .saturating_sub(count.checked_mul(wire.key_fixed)?);
    Some(longest.min(framed))
}

/// The observation rows that are bounds, not allocations: nothing holds them until a host arms a
/// tap, a transaction ends one, or an event waits for egress.
struct ObservationReserve {
    /// The heap one event can own ([`ended_event_ceiling`]); the token holds one.
    ended_event: u64,
    /// That heap in every cell, one more cell than the reliable queue has slots.
    cell_heap: u64,
    /// The binding table's capacity bound.
    bindings_table: u64,
    /// `ObservationBindings::after`'s transient copies.
    bindings_transient: u64,
}

fn observation_reserve(
    frame_bytes: u64,
    slots: usize,
) -> Result<ObservationReserve, CompileFailure> {
    // The table never holds more bindings than a plan has slots, and every binding it holds was
    // admitted, so all of them together fit one ended event (#1685 D1).
    let wire = observation_wire(frame_bytes)?;
    let keys = slots.min(wire.max_keys());
    let ended_event = ended_event_ceiling(wire, keys)?;
    let cell_heap = ended_event
        .checked_mul(
            u64::try_from(RELIABLE_EVENT_SLOTS + 1)
                .map_err(|_| failure(ResourceFault::Platform))?,
        )
        .ok_or_else(|| failure(ResourceFault::Arithmetic))?;
    let (bindings_table, bindings_transient) = observation_binding_rows(wire, keys)?;
    Ok(ObservationReserve {
        ended_event,
        cell_heap,
        bindings_table,
        bindings_transient,
    })
}

/// What the active row charges for observation subscriptions beyond the allocations a compile
/// makes, as `(active, token)` bytes: the cells' event heap, the binding table and its transient
/// copies, and the one event the prepared token holds (#1584 D6a). The adapters' resource tests
/// add the first to the allocations they observe, as they add the send producers' upper bound.
///
/// # Errors
///
/// The arithmetic overflowed.
#[cfg(feature = "test-support")]
pub fn observation_reserve_bytes(
    frame_bytes: u64,
    slots: usize,
) -> Result<(u64, u64), CompileFailure> {
    let reserve = observation_reserve(frame_bytes, slots)?;
    Ok((
        checked_sum(&[
            reserve.cell_heap,
            reserve.bindings_table,
            reserve.bindings_transient,
        ])?,
        reserve.ended_event,
    ))
}

/// The binding table's charge alone (`observation_binding_rows`) for a control frame of
/// `frame_bytes` and `slots` observation slots, so a test can hold a measured table to it.
///
/// # Errors
///
/// The arithmetic overflowed.
#[cfg(feature = "test-support")]
pub fn observation_binding_table_bytes(
    frame_bytes: u64,
    slots: usize,
) -> Result<u64, CompileFailure> {
    Ok(observation_reserve(frame_bytes, slots)?.bindings_table)
}

/// The heap one `OBSERVATION_ENDED` event can own when it ends at most `keys` taps (#1584 D6a,
/// #1685 D4): its key list, one `protocol::ObservationTapKey` each, and the text of the two IDs
/// every key owns, at most [`admitted_text`]. `keys` is what the session can hold, the smaller of
/// the observation slots it has had and the keys one frame carries. It does not depend on any
/// plan's producers: a cell can hold the keys of an earlier plan.
fn ended_event_ceiling(wire: ObservationWire, keys: usize) -> Result<u64, CompileFailure> {
    peak_over_keys(wire, keys, |count| {
        count
            .checked_mul(size_of::<protocol::ObservationTapKey>())?
            .checked_add(admitted_text(wire, count)?)
    })
}

/// What the binding table and a rebuild's transient copies of it hold for at most `keys` admitted
/// bindings, as `(table, transient)` bytes (#1584 D6a, #1685 D4).
///
/// The table is two peaks added, not one peak of their sum (#1685 review fold): its `Vec` keeps
/// its high-water capacity (`ObservationBindings::clear` removes and never shrinks), while the ID
/// text belongs to whatever set is armed now, so a table that once held many short IDs can later
/// hold few long ones in the same capacity. The `Vec` grows only by `insert`, one entry at a time,
/// from empty or from `ObservationBindings::after`'s exact reservation: std's amortized growth
/// doubles the capacity, or takes four for an entry of at most 1 KiB, so the capacity is at most
/// `2 * max(keys, 4)` entries. The text is at most the peak of [`admitted_text`].
///
/// The transient is what `ObservationBindings::after` builds from the table: each binding goes to
/// the kept table or the ended list, so at most `keys` of them together, and the protocol keys the
/// ended ones are copied into. The bindings' ID text and the keys' copy of it are each at most
/// [`admitted_text`].
fn observation_binding_rows(
    wire: ObservationWire,
    keys: usize,
) -> Result<(u64, u64), CompileFailure> {
    let arithmetic = || failure(ResourceFault::Arithmetic);
    let binding = size_of::<controls::ObservationBinding>();
    let key = size_of::<protocol::ObservationTapKey>();
    let capacity = if keys == 0 { 0 } else { keys.max(4) }
        .checked_mul(2)
        .and_then(|entries| entries.checked_mul(binding))
        .ok_or_else(arithmetic)?;
    let text = peak_over_keys(wire, keys, |count| admitted_text(wire, count))?;
    let table = u64::try_from(capacity)
        .map_err(|_| failure(ResourceFault::Platform))?
        .checked_add(text)
        .ok_or_else(arithmetic)?;
    let transient = peak_over_keys(wire, keys, |count| {
        count
            .checked_mul(binding.checked_add(key)?)?
            .checked_add(admitted_text(wire, count)?.checked_mul(2)?)
    })?;
    Ok((table, transient))
}

pub(crate) fn controller_retained_capacity(
    limits: ControlLimits,
) -> Result<ControllerRetainedCapacity, CompileFailure> {
    let control_bytes = usize::try_from(limits.maximum_control_frame_bytes)
        .map_err(|_| failure(ResourceFault::Platform))?;
    let maximum_tlvs = control_bytes / size_of::<u16>();
    Ok(ControllerRetainedCapacity {
        meter_handles: maximum_tlvs,
        counter_ids: maximum_tlvs,
    })
}

/// The candidate a structural command prepared, as the replacement admission reads it.
#[derive(Clone, Copy)]
pub(crate) struct ProspectivePlan {
    /// The candidate's plan row: what it owns once active, carried rings included.
    pub(crate) resources: PlanResources,
    /// The part of its source rows it carries from the plan it was prepared to displace (#1273
    /// D5): the newest held plan, or across a withdrawn candidate the running one (#1310 D2).
    pub(crate) carried: CarriedSourceBytes,
    /// The capi resources it was prepared with.
    pub(crate) capi: CapiResources,
}

/// The replacement admission (#1042, #1398 D4): may the prospective plan live beside every plan
/// the control plane holds unreclaimed until each is reclaimed?
///
/// `held` is those plans in epoch order, the oldest first and the newest last: every parked epoch
/// whose plan render displaced but the service step has not reclaimed (batch 15a review fold 2,
/// M-1), the running plan, then every pending or withdrawn candidate. It is never empty. `compiled_models` charges
/// every model the control plane holds plus the prospective one. Each term is the sum over the
/// held plans plus the candidate, against its own cap: graph plus models, the source totals and
/// overheads (a carried ring counted once, in the oldest held plan that holds it: the candidate's
/// and every held epoch's but the oldest one's carried bytes leave their rows), the
/// replacement peak (graph, models and allocated source bytes, #1290a D3), effect state and
/// scratch, builtins, and the control plane's retained bytes (the newest plan's row, which holds
/// the session's fixed rows once, every older held epoch's own rows, and the candidate's epoch and
/// prepared-protocol rows). The largest-allocation check folds every held row.
///
/// Nothing is committed on a refusal.
pub(crate) fn validate_replacement_peak(
    held: &[LiveEpochResources],
    prospective: ProspectivePlan,
    compiled_models: CompiledModelAdmission,
    limits: ControlLimits,
) -> Result<(), CompileFailure> {
    let arithmetic = || failure(ResourceFault::Arithmetic);
    let newest = held.last().ok_or_else(arithmetic)?;
    // Each held row's `field`, summed, plus the candidate's `candidate`.
    let peak = |field: fn(&PlanResources) -> u64, candidate: u64| {
        held.iter()
            .try_fold(0_u64, |total, epoch| {
                total.checked_add(field(&epoch.report))
            })
            .and_then(|total| total.checked_add(candidate))
            .ok_or_else(arithmetic)
    };
    // Issue #1273 D5: a carried ring is the plan's it was carried from until the swap, so the peak
    // counts it once, there: the candidate's carried bytes leave its row, and every held epoch
    // after the oldest leaves its own (#1398 D4: a ring a pending candidate carried from the
    // running plan is the running plan's, and one the running plan carried from a parked epoch
    // is the parked epoch's; each epoch carries only from the one before it in `held` or from
    // a plan already reclaimed).
    let allocated = |row: u64, carried: u64| row.checked_sub(carried).ok_or_else(arithmetic);
    let source_peak = |field: fn(&PlanResources) -> u64,
                       carried: fn(&CarriedSourceBytes) -> u64,
                       candidate: u64| {
        held.iter()
            .enumerate()
            .try_fold(0_u64, |total, (index, epoch)| {
                let row = field(&epoch.report);
                let own = if index == 0 {
                    Some(row)
                } else {
                    row.checked_sub(carried(&epoch.carried))
                };
                own.and_then(|own| total.checked_add(own))
            })
            .and_then(|total| total.checked_add(candidate))
            .ok_or_else(arithmetic)
    };
    let candidate = prospective.resources;
    let graph_peak = peak(
        |row| row.graph_session_plus_plan_bytes,
        candidate.graph_session_plus_plan_bytes,
    )?
    .checked_add(compiled_models.retained_bytes)
    .ok_or_else(arithmetic)?;
    if graph_peak > limits.maximum_graph_session_plus_plan_bytes {
        return Err(diagnostic("graph.resource.limit"));
    }
    let source_total = source_peak(
        |row| row.source_total_bytes,
        |carried| carried.total,
        allocated(candidate.source_total_bytes, prospective.carried.total)?,
    )?;
    if source_total > limits.maximum_source_total_bytes
        || source_peak(
            |row| row.source_overhead_bytes,
            |carried| carried.overhead,
            allocated(
                candidate.source_overhead_bytes,
                prospective.carried.overhead,
            )?,
        )? > limits.maximum_source_overhead_bytes
    {
        return Err(diagnostic("source.resource.limit"));
    }
    // #1290a D3: the categories above each fit alone; their sum must fit an adapter's one budget.
    if graph_peak
        .checked_add(source_total)
        .ok_or_else(arithmetic)?
        > limits.maximum_replacement_peak_bytes
    {
        return Err(diagnostic("control.resource.replacement_peak"));
    }
    if peak(
        |row| row.effect_scalar_state_bytes,
        candidate.effect_scalar_state_bytes,
    )? > limits.maximum_effect_state_bytes
        || peak(
            |row| row.effect_scalar_scratch_bytes,
            candidate.effect_scalar_scratch_bytes,
        )? > limits.maximum_effect_scratch_bytes
    {
        return Err(diagnostic("effect.resource.limit"));
    }
    if peak(
        |row| row.builtin_retained_payload_bytes,
        candidate.builtin_retained_payload_bytes,
    )? > limits.maximum_builtin_retained_bytes
    {
        return Err(failure(ResourceFault::Limit));
    }
    // The newest held row holds the session's fixed rows once and its own epoch's; every older
    // held epoch adds its own rows, and the candidate its epoch and prepared-protocol rows.
    let older_epochs = held[..held.len() - 1]
        .iter()
        .try_fold(0_u64, |total, epoch| {
            total.checked_add(epoch.capi.epoch_retained)
        })
        .ok_or_else(arithmetic)?;
    let capi_peak = prospective
        .capi
        .epoch_retained
        .checked_add(prospective.capi.prepared_protocol_retained)
        .and_then(|candidate| candidate.checked_add(newest.report.control_retained_bytes))
        .and_then(|value| value.checked_add(older_epochs))
        // #1696: the kept base this commit stores holds ramps at the candidate's model, which the
        // newest row charges at its own.
        .and_then(|value| {
            value.checked_add(
                prospective
                    .capi
                    .edit_ramps
                    .saturating_sub(newest.capi.edit_ramps),
            )
        })
        // #1594 R6: the spectrum service the candidate's promotion makes, when the newest plan
        // has none.
        .and_then(|value| {
            value.checked_add(
                prospective
                    .capi
                    .spectrum_service
                    .saturating_sub(newest.capi.spectrum_service),
            )
        })
        .ok_or_else(arithmetic)?;
    if capi_peak > limits.maximum_control_retained_bytes {
        return Err(failure(ResourceFault::Limit));
    }
    let largest = held
        .iter()
        .map(|epoch| epoch.report.largest_named_allocation_bytes)
        .fold(0, u64::max);
    if largest
        .max(prospective.resources.largest_named_allocation_bytes)
        .max(prospective.capi.largest)
        .max(compiled_models.largest_allocation_bytes)
        > limits.maximum_named_allocation_bytes
    {
        return Err(failure(ResourceFault::Limit));
    }
    Ok(())
}

/// One provider epoch's resources as the live and replacement admissions read them: its plan's
/// report row and the capi resources it was prepared with (#1257 D4, #1398 D4).
///
/// Public only to the adapters' tests, under `test-support`; crate-private otherwise.
#[derive(Clone, Copy)]
#[cfg_attr(not(feature = "test-support"), allow(unreachable_pub))]
pub struct LiveEpochResources {
    /// The epoch's plan row, read back from the adapter's report table.
    pub report: PlanResources,
    /// The capi resources the epoch's plan was prepared with.
    pub capi: CapiResources,
    /// The part of the report's source rows the epoch's plan carries from the plan it was
    /// prepared to displace (#1273 D5): zero for the boot plan. The replacement admission
    /// subtracts it for every held epoch after the oldest, whose carried rings belong to the
    /// plan before it while both are held (#1398 D4, batch 15a review fold M2).
    pub carried: CarriedSourceBytes,
}

/// The live arm's admission (#1053 D8, #1257 D5, #1398 D4): may the prospective compiled model
/// live beside the current one, next to the plans that exist, until the commit?
///
/// The live arm prepares no plan, so the plans are those the control plane holds unreclaimed,
/// `held`, in epoch order: every parked epoch whose plan is not yet reclaimed, the running plan,
/// then every pending or withdrawn candidate; the newest of them is the last. It is never empty. The terms, each against its own cap:
///
/// - **graph:** every held plan's `graph_session_plus_plan_bytes` plus every compiled model's
///   `retained_bytes` (the held ones and the prospective one);
/// - **capi:** the newest plan's `control_retained_bytes`, plus every older held epoch's
///   `epoch_retained`, plus the newest epoch's `prepared_protocol_retained`. That last term
///   includes a provider catalog (`catalog_retained_bytes`), which a live commit replaces by the
///   committed model's (#1552 D3), so the term is the same one a rebuild charges;
/// - **catalog growth:** the new catalog's retained bytes beyond the current catalog's are added to
///   the capi term, and its largest single allocation joins the largest-allocation term, so a VCA
///   added live that grows the catalog meets the ceiling a rebuild would (#1558 D6); the
///   committed model's edit-ramp row beyond the newest epoch's is added the same way (#1696);
/// - **largest allocation:** the largest of every held plan's `largest_named_allocation_bytes`,
///   the newest epoch's capi `largest`, the compiled models' largest allocation and the new
///   catalog's.
///
/// Public only to the adapters' tests, under `test-support`; crate-private otherwise.
///
/// # Errors
///
/// The cap a term exceeds, with its diagnostic; arithmetic overflow, or an empty `held`.
#[cfg_attr(not(feature = "test-support"), allow(unreachable_pub))]
pub fn validate_live_peak(
    held: &[LiveEpochResources],
    compiled_models: CompiledModelAdmission,
    catalog: LiveCatalogAdmission,
    limits: ControlLimits,
) -> Result<(), CompileFailure> {
    let arithmetic = || failure(ResourceFault::Arithmetic);
    let (newest, older) = held.split_last().ok_or_else(arithmetic)?;
    let graph = held
        .iter()
        .try_fold(compiled_models.retained_bytes, |total, epoch| {
            total.checked_add(epoch.report.graph_session_plus_plan_bytes)
        })
        .ok_or_else(arithmetic)?;
    if graph > limits.maximum_graph_session_plus_plan_bytes {
        return Err(diagnostic("graph.resource.limit"));
    }
    let capi = older
        .iter()
        .try_fold(newest.report.control_retained_bytes, |total, epoch| {
            total.checked_add(epoch.capi.epoch_retained)
        })
        .and_then(|value| value.checked_add(newest.capi.prepared_protocol_retained))
        .and_then(|value| value.checked_add(catalog.growth_bytes))
        .and_then(|value| value.checked_add(catalog.ramp_growth_bytes))
        .ok_or_else(arithmetic)?;
    if capi > limits.maximum_control_retained_bytes {
        return Err(failure(ResourceFault::Limit));
    }
    if held
        .iter()
        .map(|epoch| epoch.report.largest_named_allocation_bytes)
        .fold(0, u64::max)
        .max(newest.capi.largest)
        .max(compiled_models.largest_allocation_bytes)
        .max(catalog.largest_allocation_bytes)
        > limits.maximum_named_allocation_bytes
    {
        return Err(failure(ResourceFault::Limit));
    }
    Ok(())
}

/// Translate the control limits into the facade's caps, field for field, except
/// `maximum_submixes` and `maximum_vcas`, whose zero means `maximum_tracks` (#1206 D2, #1243 D2).
///
/// This is the only place the mapping is spelled. `AnyLaunchRate`: the C ABI compiles whatever
/// launch rate the session declares (issue 032), unlike the browser host which is pinned to its
/// `AudioContext`.
///
/// Public only to the adapters' tests, under `test-support`; crate-private otherwise.
#[cfg_attr(not(feature = "test-support"), allow(unreachable_pub))]
pub fn prepare_caps(limits: ControlLimits) -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: limits.source_ring_frames,
        maximum_automation_spans_per_block: limits.maximum_automation_spans_per_block,
        maximum_tracks: limits.maximum_tracks,
        // #1206 D2: zero is every pre-#1206 caller's value, and means "bound submixes by tracks".
        maximum_submixes: if limits.maximum_submixes == 0 {
            limits.maximum_tracks
        } else {
            limits.maximum_submixes
        },
        // #1243 D2: likewise, zero means "bound VCA groups by tracks".
        maximum_vcas: if limits.maximum_vcas == 0 {
            limits.maximum_tracks
        } else {
            limits.maximum_vcas
        },
        maximum_vca_reach_pairs: limits.maximum_vca_reach_pairs,
        maximum_sources: limits.maximum_sources,
        maximum_routes: limits.maximum_routes,
        maximum_effects: limits.maximum_effects,
        maximum_graph_session_plus_plan_bytes: limits.maximum_graph_session_plus_plan_bytes,
        maximum_source_total_bytes: limits.maximum_source_total_bytes,
        maximum_source_overhead_bytes: limits.maximum_source_overhead_bytes,
        maximum_effect_state_bytes: limits.maximum_effect_state_bytes,
        maximum_effect_scratch_bytes: limits.maximum_effect_scratch_bytes,
        maximum_builtin_retained_bytes: limits.maximum_builtin_retained_bytes,
        maximum_named_allocation_bytes: limits.maximum_named_allocation_bytes,
        // The caller's meter caps bound every meter stream, the session's output meter included
        // (#1581, root caps ruling): the plan's meters get what the output meter leaves.
        // `check_output_meter_caps` refuses caps that cannot hold the output meter itself.
        maximum_meter_streams: limits.maximum_meter_streams.saturating_sub(1),
        maximum_meter_items: limits
            .maximum_meter_items
            .saturating_sub(host_core::METER_QUEUE_DEPTH as u64),
        maximum_meter_bytes: limits
            .maximum_meter_bytes
            .saturating_sub(output_meter_bytes().unwrap_or(u64::MAX)),
    }
}

/// The session's output meter's allocations (#1581 D4): its gate and its dropped-window count,
/// one `Arc<AtomicU64>` each, and its window queue's header and slots.
fn output_meter_rows() -> Result<[u64; 3], CompileFailure> {
    let queue =
        host_core::output_peak_queue_payload().map_err(|_| failure(ResourceFault::Arithmetic))?;
    let row = |bytes: usize| u64::try_from(bytes).map_err(|_| failure(ResourceFault::Platform));
    Ok([
        checked_layout::<SharedArcAllocation<AtomicU64>>(2)?,
        row(queue.ring_header_bytes)?,
        row(queue.slot_payload_bytes)?,
    ])
}

/// The output meter's bytes, as the meter-bytes cap charges them.
fn output_meter_bytes() -> Result<u64, CompileFailure> {
    checked_sum(&output_meter_rows()?)
}

/// Refuse caps that cannot hold the session's output meter (#1581, root caps ruling), with the
/// builtin compiler's own diagnostics: one meter stream (`$.meter_requests`), and its queue's
/// items and bytes (`$.builtin_compile_caps`). The plan's own meters are bounded by what is left
/// ([`prepare_caps`]).
fn check_output_meter_caps(limits: ControlLimits) -> Result<(), CompileFailure> {
    let refuse = |path: &str| {
        CompileFailure::Diagnostics(format!("builtin.resource.limit\t{path}\n").into_bytes())
    };
    if limits.maximum_meter_streams == 0 {
        return Err(refuse("$.meter_requests"));
    }
    if limits.maximum_meter_items < host_core::METER_QUEUE_DEPTH as u64
        || limits.maximum_meter_bytes < output_meter_bytes()?
    {
        return Err(refuse("$.builtin_compile_caps"));
    }
    Ok(())
}

pub(crate) fn prepare_failure(diagnostics: PrepareDiagnostics) -> CompileFailure {
    CompileFailure::Diagnostics(diagnostics.into_bytes())
}

/// The adapter's choice in preparing a plan (issue #1400 D1): how the host-core pipeline is called,
/// and nothing else.
///
/// The session prepares every plan through the preparer its adapter supplies, at boot and for
/// every structural transaction. The control plane chooses the live lanes
/// ([`controls::live_lanes`], decision 17 C6) and passes them in; the preparer chooses the live
/// request (its queue depth, its meters, its spectrum capture) and which host-core entry to call.
/// Everything else stays in the crate's `prepare_runtime`, in this order: the shape check before
/// the call, then the strip lanes, the effect producers, the resource projection and the limits
/// after it.
///
/// A preparer holds no per-plan state: the session calls it once for the boot plan and once for
/// each structural successor, always on a control thread. `successor` is `Some` for a successor,
/// and the preparer then prepares through the host-core entry that takes the base, so the
/// unchanged sources carry their rings. `superseded` is the session's `live_values_superseded`
/// count; the preparer passes it to host-core as it is.
///
/// `spectrum` is the session's spectrum preparation (#1593 D3), the same for the boot plan and
/// every successor; the preparer passes it to host-core and returns the captures beside the host
/// and the handles. A preparer whose host keeps the captures elsewhere returns `None`.
pub trait RuntimePreparer: Send {
    /// Prepare the plan for `compiled`, or refuse with host-core's diagnostics.
    ///
    /// # Errors
    ///
    /// The host-core preparation's diagnostics, unchanged: the session maps them to its
    /// adapter's failure.
    fn prepare(
        &self,
        compiled: &CompiledSession,
        caps: &HostPrepareCaps,
        lanes: HostLiveLanes,
        spectrum: Option<&host_core::SpectrumPreparation>,
        successor: Option<SuccessorBase<'_>>,
        superseded: &host_core::LiveValuesSuperseded,
    ) -> Result<
        (
            host_core::PreparedHost,
            host_core::HostLiveControlHandles,
            Option<host_core::PreparedSpectrumCaptures>,
        ),
        PrepareDiagnostics,
    >;

    /// The meter window, in blocks, this adapter prepares its plans' strip meters with (#1581 D2):
    /// `0`, the default, is host-core's [`host_core::default_meter_window_blocks`]. The session's
    /// output meter closes its windows at the same length, so its windows and the strip meters'
    /// tile one grid.
    fn meter_window_blocks(&self) -> u32 {
        0
    }
}

/// Prepare one plan plus its source producers, and project its resource accounting.
///
/// The shared pipeline is `host-core`; the control plane adds only what is its own: its retained
/// rows (protocol queues, replay storage, the adapter's handle structs), and the resource shape.
///
/// With a `successor` base (a structural transaction, issue #1273 D1) every source the
/// transaction leaves unchanged is prepared vacant, to carry the displaced plan's ring; the
/// report's source rows count those rings too (D5). `compile_session` passes `None`.
///
/// `spectrum` is the session's spectrum preparation (#1593 D3), passed to the preparer as it is.
pub(crate) fn prepare_runtime<A: ControlAdapter>(
    preparer: &dyn RuntimePreparer,
    compiled: &CompiledSession,
    limits: ControlLimits,
    spectrum: Option<&host_core::SpectrumPreparation>,
    successor: Option<SuccessorBase<'_>>,
    superseded: &host_core::LiveValuesSuperseded,
    observation_slots_high_water: usize,
) -> Result<PreparedRuntime, CompileFailure> {
    check_output_meter_caps(limits)?;
    let caps = prepare_caps(limits);
    // Shape still runs before host/runtime allocation. The exact provider catalog exists only
    // after shared host preparation, so CAPI retained-resource admission necessarily follows the
    // full plan allocation; #369 records that allocation and diagnostic-precedence consequence.
    caps.validate_shape(compiled).map_err(prepare_failure)?;
    // Decision 17, C6: the core chooses the live lanes from the session's live control rows. Every
    // plan carries each strip's fader/mute and matrix/pan lanes (#1256 D1); the others follow the
    // rows, so a row that becomes live gets its lane with no change here.
    //
    // With a `successor` base (issue #1273 D1) the same lanes attach, and every source the
    // transaction left unchanged is prepared vacant to carry its predecessor's ring.
    let lanes: HostLiveLanes = controls::live_lanes(compiled.normalized_model()).into();
    let (prepared, handles, spectrum) = preparer
        .prepare(compiled, &caps, lanes, spectrum, successor, superseded)
        .map_err(prepare_failure)?;
    // #1396 deliverable 3: a test gives the next plan a source-read offset before publication.
    #[cfg(feature = "test-support")]
    let prepared = {
        let mut prepared = prepared;
        if let Some(offset) = crate::control::take_test_next_source_read_offset() {
            host_core::test_only_set_source_read_offset(&mut prepared, offset);
        }
        prepared
    };
    // #1256 D2: keep only the producers. The strip ID list and the vectors this selection leaves
    // empty drop here, on the control thread; each producer keeps its own `track_id`. The
    // producer vector is built at exactly one entry per strip, so the boxed slice does not
    // reallocate.
    let host_core::HostLiveControlHandles {
        strip_controls,
        track_count,
        effect_controls,
        route_controls,
        follow_route_controls,
        jump_lengths,
        meters,
        effect_observations,
        ..
    } = handles;
    // #1639 D1: every prepared plan has a jump-length cell, so a handle set without its writer is
    // a preparation failure, with the code and stage of the cell's own attach failure; the epoch
    // then holds the writer unconditionally.
    let jump_lengths = jump_lengths.ok_or_else(|| {
        prepare_failure(PrepareDiagnostics::fixed(
            host_core::PrepareRejection::Graph,
            "host.jump_lengths.attach",
        ))
    })?;
    // #1916: the producers' own heap owners, summed before the table is boxed, for the strip
    // table's charge.
    let strip_heap_bytes = strip_controls
        .iter()
        .map(host_core::TrackControlProducer::retained_heap_bytes)
        .try_fold(0_u64, u64::checked_add)
        .ok_or_else(|| failure(ResourceFault::Arithmetic))?;
    let strips = StripLanes {
        controls: strip_controls.into_boxed_slice(),
        track_count,
        // #1557 D4: the send producers, at exactly one entry per route lane the plan attached.
        routes: route_controls.into_boxed_slice(),
        // #1653: the following routes' seven-word producers, one per attached seven-word lane.
        follow_routes: follow_route_controls.into_boxed_slice(),
        // #1639 D1: the plan's jump-length cell writer.
        jump_lengths,
    };
    // #1263 D2: likewise one entry per effect instance, built at exactly that capacity, so the
    // boxed slice is the allocation host-core walked for `effect_control_resources`.
    let effects = effect_controls.into_boxed_slice();
    // #1581 D6: the meter consumers stay with the plan's epoch. builtins builds the vector at
    // exactly one entry per request and charges it in `builtin_meter_payload_bytes`, with each
    // consumer's ring, so the boxed slice does not reallocate and is not charged again here.
    let meters = meters.into_boxed_slice();
    // #1584 D7: the observation readers stay with the plan's epoch, the reader of every tap an
    // `Observe` edit arms. Shrunk to their length first, so the charge below is the allocation.
    let observations = effect_observations.into_boxed_slice();
    let mut capi = prepared_capi_resources::<A>(
        compiled,
        &prepared.control_catalog,
        &prepared.inventory,
        prepared.report.effect_control_resources,
        prepared.report.route_control_resources,
        observation_resources(
            &effects,
            observation_handle_bytes(&observations)?,
            observation_slots_high_water,
        )?,
        limits,
        strip_heap_bytes,
    )?;
    // #1594 R6, #1595 R7: a plan with spectrum captures has a session spectrum service, analysis
    // and one-shot result beside it (made at boot or at the promotion that installs the first
    // captures), charged here, with the plan, so the admission of the configure that creates them
    // sees them.
    if spectrum.is_some() {
        capi.spectrum_service = crate::spectrum::SPECTRUM_MONITOR_RETAINED_BYTES;
        capi.active_retained = capi
            .active_retained
            .checked_add(capi.spectrum_service)
            .ok_or_else(|| failure(ResourceFault::Arithmetic))?;
    }
    if capi.active_retained > limits.maximum_control_retained_bytes
        || capi.largest > limits.maximum_named_allocation_bytes
    {
        return Err(failure(ResourceFault::Limit));
    }
    let host = prepared.report;
    // #1502 D1: the plan's control handle moves into its provider epoch, so the gate, the cells
    // and the producers that write them follow one routing.
    let mut plan = prepared.plan;
    let revision_gate = plan
        .take_revision_gate()
        .expect("a freshly prepared plan hands out its revision gate");
    let largest_named = host.largest_engine_allocation_bytes.max(capi.largest);
    let carried = CarriedSourceBytes {
        total: host.carried_source_total_bytes,
        overhead: host.carried_source_overhead_bytes,
    };
    // Issue #1273 D5: the rows count what the plan owns once active, carried rings included,
    // and the carry program it retains with its graph.
    let with_carried = |row: u64, carried: u64| {
        row.checked_add(carried)
            .ok_or_else(|| failure(ResourceFault::Arithmetic))
    };
    let carried_payload = carried
        .total
        .checked_sub(carried.overhead)
        .ok_or_else(|| failure(ResourceFault::Arithmetic))?;
    Ok(PreparedRuntime {
        sources: prepared.sources,
        strips,
        effects,
        meters,
        observations,
        plan,
        revision_gate,
        resources: PlanResources {
            sample_rate_hz: host.sample_rate_hz,
            quantum_frames: host.quantum_frames,
            source_count: host.source_count,
            track_count: host.track_count,
            latency_samples: host.latency_samples,
            output_tail_every_peak: host.output_tail_every_peak,
            // The carry program's bytes are the plan's own allocation that the host-core row does
            // not count. The plan's revision gate is in the host-core row (#1502 D1), shared with
            // the browser.
            graph_session_plus_plan_bytes: with_carried(
                host.graph_session_plus_plan_bytes,
                host.carry_program_retained_bytes,
            )?,
            graph_incremental_plan_bytes: host.graph_incremental_plan_bytes,
            graph_metadata_bytes: host.graph_metadata_bytes,
            graph_delay_bytes: host.graph_delay_bytes,
            effect_bank_scratch_bytes: host.effect_bank_scratch_bytes,
            effect_bank_runtime_buffer_bytes: host.effect_bank_runtime_buffer_bytes,
            effect_bank_metadata_bytes: host.effect_bank_metadata_bytes,
            builtin_bank_bytes: host.builtin_bank_bytes,
            builtin_bank_scratch_bytes: host.builtin_bank_scratch_bytes,
            source_pcm_payload_bytes: with_carried(host.source_pcm_payload_bytes, carried_payload)?,
            source_overhead_bytes: with_carried(host.source_overhead_bytes, carried.overhead)?,
            source_total_bytes: with_carried(host.source_total_bytes, carried.total)?,
            effect_scalar_state_bytes: host.effect_scalar_state_bytes,
            effect_scalar_scratch_bytes: host.effect_scalar_scratch_bytes,
            builtin_processor_payload_bytes: host.builtin_processor_payload_bytes,
            builtin_meter_payload_bytes: host.builtin_meter_payload_bytes,
            builtin_retained_payload_bytes: host.builtin_retained_payload_bytes,
            control_retained_bytes: capi.active_retained,
            largest_named_allocation_bytes: largest_named,
        },
        carried,
        inventory: prepared.inventory,
        control_catalog: prepared.control_catalog,
        capi,
        spectrum,
    })
}

/// Compile `document` under `limits` into a control session and the plan it drives.
///
/// The adapter keeps its own per-session diagnostic storage; it builds that after this returns.
///
/// `preparer` (issue #1400 D2) is the adapter's preparation: the session stores it and prepares
/// the boot plan and every structural successor through it, and through nothing else.
///
/// `spectrum` is the session's spectrum monitor configuration (#1593 D2): the first plan is
/// prepared with it, and the session's monitor starts at configuration epoch 1 with one and 0
/// without. It is monitor state: no revision, model or snapshot holds it.
pub fn compile_children<A: ControlAdapter>(
    document: &str,
    mut limits: ControlLimits,
    preparer: Box<dyn RuntimePreparer>,
    spectrum: Option<host_core::SpectrumConfiguration>,
) -> Result<CompiledChildren<A>, CompileFailure>
where
    A::Row: Send + 'static,
{
    // #1709 D1: the boot preparation, the catalog and every effective value composed here run in
    // the canonical floating-point environment, whatever the host's control thread has set. The
    // guard restores the caller's word on every return.
    let _canonical = lane::CanonicalFpEnv::enter();
    // The C ABI needs the transactional `SessionStore` for the control protocol, so it parses and
    // caps through the facade and builds the store itself; the facade never sees the protocol.
    let model = parse_host_session(document).map_err(prepare_failure)?;
    if limits.source_ring_frames == 0 {
        limits.source_ring_frames =
            host_core::default_source_ring_frames(model.sample_rate_hz, model.quantum_frames);
        if limits.source_ring_frames == 0 {
            return Err(failure(ResourceFault::Arithmetic));
        }
    }
    let compile_caps = prepare_caps(limits)
        .compile_caps(model.sources.len())
        .map_err(prepare_failure)?;
    let store =
        SessionStore::new(model, compile_caps).map_err(|value| session_diagnostics(&value))?;
    // #1312 D2: the session's `live_values_superseded` count, one for every plan it prepares.
    // Made here, with the session's other fixed rows, and charged by `capi_resources`.
    let live_values_superseded = host_core::LiveValuesSuperseded::default();
    let runtime = prepare_runtime::<A>(
        &*preparer,
        store.compiled(),
        limits,
        spectrum
            .as_ref()
            .map(|configuration| &configuration.preparation),
        None,
        &live_values_superseded,
        0,
    )?;
    // #1689 D3: a boot designates no master; the app writes the strip's `master` row after boot.
    let monitor = controls::MonitorOverlay::new();

    let control_bytes = usize::try_from(limits.maximum_control_frame_bytes)
        .map_err(|_| failure(ResourceFault::Platform))?;
    let replay_bytes = usize::try_from(limits.maximum_replay_bytes)
        .map_err(|_| failure(ResourceFault::Platform))?;
    let replay_entries = usize::try_from(limits.maximum_replay_entries)
        .map_err(|_| failure(ResourceFault::Platform))?;
    let maximum_tlvs = u32::try_from(control_bytes / size_of::<u16>()).unwrap_or(u32::MAX);
    let codec = ProtocolCodec::new(ProtocolLimits {
        max_frame_bytes: control_bytes,
        max_tlv_count: maximum_tlvs,
        max_string_bytes: control_bytes,
        max_nesting: 4,
    });
    let queues = ProtocolQueues::prepare(protocol_queue_config(limits)?)
        .map_err(|_| failure(ResourceFault::ProtocolQueue))?;
    let replay = ReplayCache::try_new(ReplayCacheConfig {
        entries: NonZeroUsize::new(replay_entries).ok_or_else(|| failure(ResourceFault::Limit))?,
        bytes: NonZeroUsize::new(replay_bytes).ok_or_else(|| failure(ResourceFault::Limit))?,
        max_response_bytes: control_bytes,
    })
    .map_err(|_| failure(ResourceFault::Allocation))?;
    let retained_capacity = controller_retained_capacity(limits)?;
    let PreparedRuntime {
        sources,
        strips,
        effects,
        meters,
        observations,
        plan,
        mut revision_gate,
        resources,
        carried: _,
        inventory,
        control_catalog,
        capi,
        spectrum: captures,
    } = runtime;
    let spectrum = crate::spectrum::SpectrumMonitor::new(
        spectrum,
        captures,
        resources.sample_rate_hz,
        resources.quantum_frames,
    );
    // #1314 D1, #1502 D5: the initial plan carries the store's initial revision, published on its
    // own gate, and the watermark starts there.
    let (publisher, owner, retirer) = plan_exchange_at_revision(
        plan,
        &mut revision_gate,
        store.revision().0,
        PlanExchangeConfig {
            retirement_capacity: retirement_capacity(),
        },
    )
    .map_err(|_| failure(ResourceFault::PlanExchange))?;
    let mut reports = Vec::new();
    reports
        .try_reserve_exact(REPORT_ROWS)
        .map_err(|_| failure(ResourceFault::Allocation))?;
    reports.push((0, A::Row::from(resources)));
    let shared = Arc::new(SharedPlanState {
        plan_alive: AtomicBool::new(true),
        active_epoch: AtomicU64::new(0),
        reports: Mutex::new(reports),
        render_sequence: AtomicU64::new(0),
        render_sample: AtomicU64::new(0),
        render_peak_bits: AtomicU32::new(0),
        // No endpoint has configured telemetry yet, so nothing can read a peak.
        render_peak_observed: AtomicBool::new(false),
        live_block_errors: AtomicU64::new(0),
        source_underrun_blocks: AtomicU64::new(0),
        watermark: publisher.watermark_reader(),
    });
    // #1581 D4: the session's one output meter, made with its first plan and part of no plan. Its
    // windows are as long as the strip meters'.
    let (output_observer, output_reader, output_gate) = host_core::output_peak_meter(
        output_meter_window_frames(store.compiled(), preparer.meter_window_blocks())?,
    )
    .map_err(|_| failure(ResourceFault::Allocation))?;
    let sample_source: Arc<dyn host_core::PlanSampleSource> = Arc::clone(&shared) as Arc<_>;
    // #1577 D5: the session's one response capture channel, its server beside the plan.
    let (mut capture_client, capture_server) =
        host_core::response_capture_channel().map_err(|_| failure(ResourceFault::Allocation))?;
    capture_client.set_newest_plan_epoch(owner.active_epoch().0);
    let provider = SessionControlProvider::try_new(
        control_catalog,
        store.revision().0,
        sample_source,
        retained_capacity,
        RENDER_DIAGNOSTIC_SLOTS,
        // A reply must fit the endpoint's frame and the replay cache's response bound, both the
        // host's control frame limit.
        control_bytes,
        capture_client,
    )
    .map_err(|_| failure(ResourceFault::Allocation))?;
    let controller = ProtocolController::try_with_config_and_retained_capacity(
        store,
        queues,
        provider,
        replay,
        codec,
        ProtocolControllerConfig {
            maximum_transaction_edits: maximum_tlvs,
            maximum_response_diagnostics: u16::MAX,
            provider_features: ProviderFeatures::ALL,
            // #1386 D3: an adapter with a document transport serves `SESSION_REPLACE` through this
            // one hook, bounded by the boot's document bound (0 is unbounded, as in every zero
            // control limit); without one the command is unadvertised and `UNAVAILABLE`.
            document_replace: A::DOCUMENT_TRANSPORT.then_some(protocol::DocumentReplace {
                replace: crate::document::replace,
                maximum_document_bytes: limits.maximum_document_bytes,
            }),
        },
        retained_capacity,
    )
    .map_err(|_| failure(ResourceFault::Allocation))?;
    let mut pending_providers = Vec::new();
    pending_providers
        .try_reserve_exact(PENDING_PROVIDERS)
        .map_err(|_| failure(ResourceFault::Allocation))?;
    let mut retired_providers = Vec::new();
    retired_providers
        .try_reserve_exact(RETIRED_PROVIDERS)
        .map_err(|_| failure(ResourceFault::Allocation))?;
    let decode_field_count = control_bytes / size_of::<u16>();
    let mut decode_fields = Vec::new();
    decode_fields
        .try_reserve_exact(decode_field_count)
        .map_err(|_| failure(ResourceFault::Allocation))?;
    decode_fields.resize(decode_field_count, 0);
    let mut session = SessionState {
        controller: ObservedController::new(controller),
        providers: ProviderEpoch::current(
            sources,
            inventory,
            strips,
            effects,
            meters,
            capi,
            revision_gate,
        )
        .with_observations(observations),
        pending_providers,
        retired_providers,
        publisher,
        retirer,
        limits,
        preparer,
        live_values_superseded,
        strips_restarted: 0,
        source_frames_discarded: 0,
        decode_fields: decode_fields.into_boxed_slice(),
        response_scratch: boxed_zeroed(limits.maximum_control_frame_bytes)?,
        shared: Arc::clone(&shared),
        observed_render_sequence: 0,
        counter_snapshot_sequence: 0,
        render_diagnostics: prepare_render_diagnostic_slots()?,
        render_diagnostic_head: 0,
        render_diagnostic_len: 0,
        protocol_reliable_pending: false,
        monitor,
        observation: controls::ObservationBindings::new(),
        observation_pending: Box::default(),
        observation_offsets: Box::default(),
        observation_batch_counts: Box::default(),
        observation_batch_mark: 0,
        observation_armed: 0,
        observation_wire: observation_wire(limits.maximum_control_frame_bytes)?,
        observation_armed_bytes: 0,
        observation_slots_high_water: 0,
        observation_forwarded: Vec::new(),
        send_moves: None,
        spectrum,
        output_meter: Some((output_reader, output_gate)),
        response_ready: None,
    };
    let (offsets, pending, counts, armed, armed_bytes) = crate::control::observation_slots(
        &session.providers.effects,
        &session.observation,
        session.observation_wire,
    )
    .map_err(|_| failure(ResourceFault::Allocation))?;
    session.observation_offsets = offsets;
    session.observation_pending = pending;
    session.observation_batch_counts = counts;
    session.observation_armed = armed;
    session.observation_armed_bytes = armed_bytes;
    session.observation_slots_high_water = session.observation_pending.len();
    Ok(CompiledChildren {
        session,
        plan: PlanState::new(owner, shared, output_observer, capture_server),
    })
}

/// The output meter's window in frames (#1581 D4): `window_blocks` blocks of the session's quantum,
/// `0` meaning host-core's default, as the strip meters' window is.
fn output_meter_window_frames(
    compiled: &CompiledSession,
    window_blocks: u32,
) -> Result<core::num::NonZeroU32, CompileFailure> {
    let quantum = compiled.quantum().0;
    let blocks = core::num::NonZeroU32::new(window_blocks).unwrap_or_else(|| {
        host_core::default_meter_window_blocks(compiled.sample_rate().0, quantum)
    });
    blocks
        .get()
        .checked_mul(quantum)
        .and_then(core::num::NonZeroU32::new)
        .ok_or_else(|| failure(ResourceFault::Arithmetic))
}

#[cfg(test)]
mod admission_tests {
    //! #1398 gate 2: the replacement admission sums every held plan and model.

    use super::*;

    /// The caps the replacement admission checks, each with the diagnostic it refuses with.
    const CAPS: [(&str, &str); 9] = [
        ("graph", "graph.resource.limit\t$\n"),
        ("source total", "source.resource.limit\t$\n"),
        ("source overhead", "source.resource.limit\t$\n"),
        ("replacement peak", "control.resource.replacement_peak\t$\n"),
        ("effect state", "effect.resource.limit\t$\n"),
        ("effect scratch", "effect.resource.limit\t$\n"),
        ("builtin", "capi.resource.limit\t$\n"),
        ("control retained", "capi.resource.limit\t$\n"),
        ("largest allocation", "capi.resource.limit\t$\n"),
    ];

    /// One plan whose every term is a multiple of `base`, so a sum that misses or repeats a plan
    /// misses or repeats a distinct number.
    fn epoch(base: u64) -> LiveEpochResources {
        LiveEpochResources {
            report: PlanResources {
                sample_rate_hz: 48_000,
                quantum_frames: 128,
                source_count: 1,
                track_count: 1,
                latency_samples: 0,
                output_tail_every_peak: TailSamples::Finite(0),
                graph_session_plus_plan_bytes: base,
                graph_incremental_plan_bytes: 0,
                graph_metadata_bytes: 0,
                graph_delay_bytes: 0,
                effect_bank_scratch_bytes: 0,
                effect_bank_runtime_buffer_bytes: 0,
                effect_bank_metadata_bytes: 0,
                builtin_bank_bytes: 0,
                builtin_bank_scratch_bytes: 0,
                source_pcm_payload_bytes: base,
                source_overhead_bytes: 2 * base,
                source_total_bytes: 3 * base,
                effect_scalar_state_bytes: 4 * base,
                effect_scalar_scratch_bytes: 5 * base,
                builtin_processor_payload_bytes: 6 * base,
                builtin_meter_payload_bytes: 0,
                builtin_retained_payload_bytes: 6 * base,
                control_retained_bytes: 7 * base,
                largest_named_allocation_bytes: base,
            },
            capi: CapiResources {
                active_retained: 7 * base,
                epoch_retained: 8 * base,
                prepared_protocol_retained: 9 * base,
                largest: base,
                edit_ramps: 0,
                spectrum_service: 0,
            },
            carried: CarriedSourceBytes::default(),
        }
    }

    /// [`epoch`] carrying `total` source bytes, `overhead` of them overhead, from the plan before.
    fn carrying(base: u64, total: u64, overhead: u64) -> LiveEpochResources {
        LiveEpochResources {
            carried: CarriedSourceBytes { total, overhead },
            ..epoch(base)
        }
    }

    fn model(retained: u64, largest: u64) -> session::ResourceEstimate {
        session::ResourceEstimate {
            entity_count: 0,
            source_count: 0,
            track_count: 0,
            effect_count: 0,
            route_count: 0,
            automation_segment_count: 0,
            retained_string_bytes: 0,
            parameter_count: 0,
            queue_items: 0,
            queue_bytes: 0,
            source_ring_frames: 0,
            source_ring_bytes: 0,
            canonical_upper_bound_bytes: 0,
            canonical_bytes: 0,
            compiled_model_bytes: retained,
            requested_runtime_bytes: 0,
            single_allocation_bytes: largest,
            platform_allocation_limit_bytes: 0,
            estimated_bytes: 0,
        }
    }

    fn unlimited() -> ControlLimits {
        ControlLimits {
            source_ring_frames: 0,
            maximum_automation_spans_per_block: 0,
            maximum_document_bytes: 0,
            maximum_diagnostic_bytes: 0,
            maximum_tracks: 0,
            maximum_sources: 0,
            maximum_routes: 0,
            maximum_effects: 0,
            maximum_graph_session_plus_plan_bytes: u64::MAX,
            maximum_source_total_bytes: u64::MAX,
            maximum_source_overhead_bytes: u64::MAX,
            maximum_effect_state_bytes: u64::MAX,
            maximum_effect_scratch_bytes: u64::MAX,
            maximum_builtin_retained_bytes: u64::MAX,
            maximum_control_retained_bytes: u64::MAX,
            maximum_named_allocation_bytes: u64::MAX,
            maximum_meter_streams: 0,
            maximum_meter_items: 0,
            maximum_meter_bytes: 0,
            maximum_control_frame_bytes: 0,
            maximum_replay_bytes: 0,
            maximum_replay_entries: 0,
            maximum_submixes: 0,
            maximum_vcas: 0,
            maximum_vca_reach_pairs: 0,
            maximum_replacement_peak_bytes: u64::MAX,
        }
    }

    /// `unlimited` with cap `index` of [`CAPS`] set to `value`.
    fn with_cap(index: usize, value: u64) -> ControlLimits {
        let mut limits = unlimited();
        *match index {
            0 => &mut limits.maximum_graph_session_plus_plan_bytes,
            1 => &mut limits.maximum_source_total_bytes,
            2 => &mut limits.maximum_source_overhead_bytes,
            3 => &mut limits.maximum_replacement_peak_bytes,
            4 => &mut limits.maximum_effect_state_bytes,
            5 => &mut limits.maximum_effect_scratch_bytes,
            6 => &mut limits.maximum_builtin_retained_bytes,
            7 => &mut limits.maximum_control_retained_bytes,
            _ => &mut limits.maximum_named_allocation_bytes,
        } = value;
        limits
    }

    fn candidate(base: u64) -> ProspectivePlan {
        let epoch = epoch(base);
        ProspectivePlan {
            resources: epoch.report,
            carried: CarriedSourceBytes::default(),
            capi: epoch.capi,
        }
    }

    fn verdict(
        held: &[LiveEpochResources],
        prospective: ProspectivePlan,
        models: CompiledModelAdmission,
        limits: ControlLimits,
    ) -> Result<(), String> {
        validate_replacement_peak(held, prospective, models, limits)
            .map_err(|failure| String::from_utf8(failure.diagnostic().into_owned()).expect("UTF-8"))
    }

    /// The peak of each cap in [`CAPS`] for `held` (in base units, oldest first) beside a
    /// candidate of base `candidate`, with `models` retained model bytes, spelled out term by
    /// term. `carried` is the source total and overhead bytes every held epoch but the oldest
    /// one carries from the plan before it, summed: each carried ring is counted once.
    fn peaks(held: &[u64], candidate: u64, models: u64, carried: (u64, u64)) -> [u64; 9] {
        let sum =
            |factor: u64| held.iter().map(|base| factor * base).sum::<u64>() + factor * candidate;
        let graph = sum(1) + models;
        let (older, newest) = (&held[..held.len() - 1], held[held.len() - 1]);
        let control =
            7 * newest + older.iter().map(|base| 8 * base).sum::<u64>() + (8 + 9) * candidate;
        [
            graph,
            sum(3) - carried.0,
            sum(2) - carried.1,
            graph + sum(3) - carried.0,
            sum(4),
            sum(5),
            sum(6),
            control,
            0,
        ]
    }

    fn assert_each_cap(
        label: &str,
        held: &[LiveEpochResources],
        prospective: ProspectivePlan,
        models: CompiledModelAdmission,
        expected: [u64; 9],
    ) {
        for (index, (cap, code)) in CAPS.iter().enumerate().take(8) {
            assert_eq!(
                verdict(held, prospective, models, with_cap(index, expected[index])),
                Ok(()),
                "{label}, {cap}: accepted at the peak {}",
                expected[index]
            );
            assert_eq!(
                verdict(
                    held,
                    prospective,
                    models,
                    with_cap(index, expected[index] - 1)
                ),
                Err((*code).to_owned()),
                "{label}, {cap}: refused one byte below"
            );
        }
    }

    /// Gate 2. Three held plans and three models (two held, one prospective): each cap one byte
    /// below the sum refuses with its diagnostic, and at the sum passes; the largest allocation
    /// folds every held row. A held candidate's carried rings are counted once, in the plan they
    /// were carried from: the middle row's carried bytes leave the source sums, the running plan's
    /// do not (it owns its rings). With one held plan the peaks are today's: the running plan
    /// beside the candidate. Test value: an admission that forgets a held plan or model (a missing
    /// third term) passes one byte over the cap; one that subtracts only the candidate's carried
    /// bytes (batch 15a review fold M2) refuses the source caps at the peak, and one that also
    /// subtracts the running plan's admits one byte below it.
    #[test]
    fn the_replacement_admission_sums_every_held_plan_and_model() {
        let models = compiled_model_admission_over(
            &[model(1 << 20, 3), model(1 << 21, 5)],
            model(1 << 22, 7),
        )
        .expect("models");
        assert_eq!(models.retained_bytes, (1 << 20) + (1 << 21) + (1 << 22));
        assert_eq!(models.largest_allocation_bytes, 7);
        let held = [epoch(1), epoch(10), epoch(100)];
        let prospective = candidate(1_000);
        assert_each_cap(
            "three held",
            &held,
            prospective,
            models,
            peaks(&[1, 10, 100], 1_000, models.retained_bytes, (0, 0)),
        );
        // The middle row carries 13 source bytes (5 of them overhead) and the running row 2 (1):
        // only the middle row's leave the sums.
        let carrying_held = [carrying(1, 2, 1), carrying(10, 13, 5), epoch(100)];
        assert_each_cap(
            "three held, a carrying candidate",
            &carrying_held,
            prospective,
            models,
            peaks(&[1, 10, 100], 1_000, models.retained_bytes, (13, 5)),
        );
        // Today's check, for the same candidate beside the running plan alone.
        let today =
            compiled_model_admission_over(&[model(1 << 20, 3)], model(1 << 22, 7)).expect("models");
        assert_each_cap(
            "one held",
            &held[..1],
            prospective,
            today,
            peaks(&[1], 1_000, today.retained_bytes, (0, 0)),
        );
        // The largest allocation: a middle held plan's is the strict maximum.
        let mut middle = epoch(10);
        middle.report.largest_named_allocation_bytes = 50_000;
        let held = [epoch(1), middle, epoch(100)];
        for (cap, expected) in [(50_000, Ok(())), (49_999, Err(CAPS[8].1.to_owned()))] {
            assert_eq!(
                verdict(&held, prospective, models, with_cap(8, cap)),
                expected
            );
        }
    }

    /// #1594 R6. Turns red if the replacement admission does not charge the spectrum service a
    /// candidate's promotion makes: a candidate with the service beside a newest plan without it
    /// raises the control-retained peak by the service's bytes (accepted at the peak, refused one
    /// byte below), and beside a newest plan that has it already, by nothing.
    #[test]
    fn the_replacement_admission_charges_a_new_spectrum_service_once() {
        let models =
            compiled_model_admission_over(&[model(1 << 20, 3)], model(1 << 22, 7)).expect("models");
        let control = peaks(&[1], 1_000, models.retained_bytes, (0, 0))[7];
        let service = crate::spectrum::SPECTRUM_MONITOR_RETAINED_BYTES;
        let mut prospective = candidate(1_000);
        prospective.capi.spectrum_service = service;
        let mut newest = epoch(1);
        for (label, newest_service, peak) in [
            ("a new service", 0, control + service),
            ("a kept service", service, control),
        ] {
            newest.capi.spectrum_service = newest_service;
            let held = [newest];
            assert_eq!(
                verdict(&held, prospective, models, with_cap(7, peak)),
                Ok(()),
                "{label}: accepted at the peak"
            );
            assert_eq!(
                verdict(&held, prospective, models, with_cap(7, peak - 1)),
                Err(CAPS[7].1.to_owned()),
                "{label}: refused one byte below"
            );
        }
    }
}
