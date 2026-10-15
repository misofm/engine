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
}

/// Source-ring bytes a successor plan carries from the plan it displaces (issue #1273 D5).
///
/// The successor's [`PlanResources`] count them in its source rows, because it owns those
/// rings once active; until the swap the running plan owns them, so the double-live admission
/// subtracts them from the successor's rows and counts each carried ring once.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct CarriedSourceBytes {
    pub(crate) total: u64,
    pub(crate) overhead: u64,
}

/// Both compiled models' charge while a prospective session lives beside the committed one.
///
/// Public only to the adapters' tests, under `test-support`; crate-private otherwise.
#[derive(Clone, Copy)]
#[cfg_attr(not(feature = "test-support"), allow(unreachable_pub))]
pub struct CompiledModelAdmission {
    /// Both models' retained bytes.
    pub retained_bytes: u64,
    /// The larger of the two models' largest single allocation.
    pub largest_allocation_bytes: u64,
}

pub(crate) fn compiled_model_admission(
    current: &CompiledSession,
    prospective: &CompiledSession,
) -> Result<CompiledModelAdmission, CompileFailure> {
    compiled_model_admission_between(current.resource_estimate(), prospective.resource_estimate())
}

/// [`compiled_model_admission`] from the two models' resource estimates, for a prospective model
/// that is not held as a compilation (#1613).
pub(crate) fn compiled_model_admission_between(
    current: session::ResourceEstimate,
    prospective: session::ResourceEstimate,
) -> Result<CompiledModelAdmission, CompileFailure> {
    Ok(CompiledModelAdmission {
        retained_bytes: current
            .compiled_model_bytes
            .checked_add(prospective.compiled_model_bytes)
            .ok_or_else(|| failure(ResourceFault::Arithmetic))?,
        largest_allocation_bytes: current
            .single_allocation_bytes
            .max(prospective.single_allocation_bytes),
    })
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
    let exchange = plan_exchange_resource_report(PlanExchangeConfig {
        retirement_capacity: NonZeroUsize::new(1).expect("one is nonzero"),
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
    // either side is a deliberate decision, not a cleanup.
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
    ];
    let ObservationReserve {
        ended_event,
        cell_heap,
        bindings_table,
        bindings_transient,
    } = observation_reserve(limits.maximum_control_frame_bytes, observation.slots)?;
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
        checked_layout::<RetainedDiagnosticSlotMirror>(2)?,
        // #1584 D6a: the observation-ended cells, one more than the reliable queue's slots (an
        // event popped for egress keeps its cell until its frame is delivered), and the heap the
        // events in them own. A cell can hold the keys of any earlier plan's subscriptions, so
        // the ceiling is not a function of this plan's producers (a successor that removed
        // producers still holds their keys): an event must fit one control frame, which bounds
        // its key count and its text (`ended_event_ceiling`). The heap is allocated when a
        // transaction prepares, never at the commit.
        checked_layout::<Option<protocol::ObservationEnded>>(
            queue_config.reliable_event_slots.get() + 1,
        )?,
        checked_byte_layout(cell_heap)?,
        // #1584 D6a: the binding table the session keeps for the newest plan and the transient
        // copies `ObservationBindings::after` makes of it while a rebuild prepares: at most one
        // binding per observation slot, so at most `slots` bindings kept or ended, and as many
        // protocol keys for the ended ones. A binding owns the text of its two IDs.
        checked_byte_layout(bindings_table)?,
        checked_byte_layout(bindings_transient)?,
        // #1584 D6a: the newest plan's pending-subscription slots and the first slot of each
        // producer (one entry past the last).
        checked_layout::<crate::control::PendingArm>(observation.slots)?,
        checked_layout::<u32>(observation.producers + 1)?,
        checked_layout::<RenderDiagnosticSlot>(RENDER_DIAGNOSTIC_SLOTS)?,
        checked_layout::<u8>(RENDER_DIAGNOSTIC_CODE.len() * RENDER_DIAGNOSTIC_SLOTS)?,
        // ProtocolController and SessionControlProvider each retain their own telemetry config.
        checked_layout::<u32>(maximum_configuration_items)?,
        checked_layout::<protocol::CounterId>(maximum_configuration_items)?,
        provider.fixed_retained_bytes,
        checked_layout::<u32>(maximum_configuration_items)?,
        checked_layout::<protocol::CounterId>(maximum_configuration_items)?,
        checked_layout::<ProviderEpoch>(2)?,
        checked_layout::<(u64, A::Row)>(2)?,
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
        .ok_or_else(|| failure(ResourceFault::Arithmetic))?;
    let prepared_protocol_retained = checked_sum(&prepared_protocol_allocation_rows)?
        .checked_add(provider.catalog_retained_bytes)
        .ok_or_else(|| failure(ResourceFault::Arithmetic))?;
    // Conservative: the strip row is the producer slice plus every `track_id`, which are separate
    // allocations, so feeding the whole row overstates the largest single allocation when that
    // row is the maximum. The error only ever refuses earlier.
    // The effect row is several allocations; its own largest one stands for it.
    let largest = [control_table_row, source_id_row, strip_table_bytes]
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
    })
}

pub(crate) fn prepared_capi_resources<A: ControlAdapter>(
    compiled: &CompiledSession,
    catalog: &ControlCatalog,
    inventory: &PlanStateInventory,
    effect_controls: host_core::EffectControlResources,
    route_controls: host_core::RouteControlResources,
    observation: ObservationResources,
    limits: ControlLimits,
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
    capi_resources::<A>(
        limits,
        compiled.source_count(),
        source_id_bytes,
        host_core::strip_control_table_bytes(strip_count, strip_id_bytes)
            .ok_or_else(|| failure(ResourceFault::Arithmetic))?,
        provider,
        inventory.retained_bytes(),
        effect_controls,
        route_controls,
        observation,
    )
}

/// What a plan's observation subscriptions retain (#1584 D6a).
#[derive(Clone, Copy)]
pub(crate) struct ObservationResources {
    /// Observation slots of the plan: one per declared tap of every producer that has a queue.
    /// The binding table holds at most one binding per slot.
    pub(crate) slots: usize,
    /// Effect producers of the plan.
    pub(crate) producers: usize,
}

/// The observation rows of `effects`, the newest plan's producers.
pub(crate) fn observation_resources(
    effects: &[host_core::EffectControlProducer],
) -> Result<ObservationResources, CompileFailure> {
    let mut slots = 0_usize;
    for producer in effects {
        slots = slots
            .checked_add(producer.observe_capacity())
            .ok_or_else(|| failure(ResourceFault::Arithmetic))?;
    }
    Ok(ObservationResources {
        producers: effects.len(),
        slots,
    })
}

/// The smallest encoded size of one key of an `OBSERVATION_ENDED` event: two one-character IDs,
/// the section and the tap ID with their field headers, measured as what a second key adds to the
/// frame. It bounds how many keys fit one frame.
fn minimum_ended_key_bytes() -> Result<u64, CompileFailure> {
    let codec = protocol::ProtocolCodec::default();
    let size = |ids: &[&str]| -> Option<usize> {
        let event = protocol::ObservationEnded {
            event_sequence: 0,
            keys: ids
                .iter()
                .map(|id| protocol::ObservationTapKey {
                    strip_id: (*id).to_owned(),
                    section: protocol::ObservationSection::Console,
                    effect_id: (*id).to_owned(),
                    tap_id: 0,
                })
                .collect(),
        };
        match codec.encode_event_frame_into(
            &protocol::TypedEventFrame {
                revision: protocol::SessionRevision(1),
                payload: protocol::EventPayload::ObservationEnded(&event),
            },
            &mut [],
        ) {
            Err(protocol::EncodeError::OutputTooSmall { required }) => Some(required),
            Err(_) | Ok(_) => None,
        }
    };
    let (one, two) = (size(&["a"]), size(&["a", "b"]));
    one.zip(two)
        .and_then(|(one, two)| two.checked_sub(one))
        .filter(|bytes| *bytes > 0)
        .and_then(|bytes| u64::try_from(bytes).ok())
        .ok_or_else(|| failure(ResourceFault::Arithmetic))
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
    let ended_event = ended_event_ceiling(frame_bytes)?;
    let cell_heap = ended_event
        .checked_mul(
            u64::try_from(RELIABLE_EVENT_SLOTS + 1)
                .map_err(|_| failure(ResourceFault::Platform))?,
        )
        .ok_or_else(|| failure(ResourceFault::Arithmetic))?;
    let (bindings_table, bindings_transient) = observation_binding_rows(slots)?;
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

/// The heap one `OBSERVATION_ENDED` event can own, bounded by the control frame it must fit: its
/// text is encoded in the frame, so it is at most the frame; its key count is at most the frame
/// over the smallest key, and each key is a `protocol::ObservationTapKey` (#1584 D6a). The bound
/// does not depend on any plan's producers: a cell can hold the keys of an earlier plan.
pub(crate) fn ended_event_ceiling(frame_bytes: u64) -> Result<u64, CompileFailure> {
    let keys = frame_bytes / minimum_ended_key_bytes()?;
    u64::try_from(size_of::<protocol::ObservationTapKey>())
        .ok()
        .and_then(|key| key.checked_mul(keys))
        .and_then(|table| table.checked_add(frame_bytes))
        .ok_or_else(|| failure(ResourceFault::Arithmetic))
}

/// What the binding table and a rebuild's transient copies of it hold for a plan with `slots`
/// observation slots, as `(table, transient)` bytes (#1584 D6a). The table's `Vec` grows by
/// insertion, so its capacity is at most twice its length (and at least four entries). The
/// transient is what `ObservationBindings::after` builds from the table: each binding goes to the
/// kept table or the ended list, so at most `slots` of them together, and the protocol keys the
/// ended ones are copied into. Every binding and key owns the text of two IDs.
fn observation_binding_rows(slots: usize) -> Result<(u64, u64), CompileFailure> {
    let arithmetic = || failure(ResourceFault::Arithmetic);
    let text = 2 * session::STABLE_ID_MAXIMUM_BYTES;
    let binding = size_of::<controls::ObservationBinding>()
        .checked_add(text)
        .ok_or_else(arithmetic)?;
    let key = size_of::<protocol::ObservationTapKey>()
        .checked_add(text)
        .ok_or_else(arithmetic)?;
    let table = if slots == 0 { 0 } else { slots.max(4) }
        .checked_mul(2)
        .and_then(|capacity| capacity.checked_mul(binding))
        .ok_or_else(arithmetic)?;
    let transient = slots
        .checked_mul(binding)
        .and_then(|bytes| bytes.checked_add(slots.checked_mul(key)?))
        .ok_or_else(arithmetic)?;
    Ok((
        u64::try_from(table).map_err(|_| failure(ResourceFault::Platform))?,
        u64::try_from(transient).map_err(|_| failure(ResourceFault::Platform))?,
    ))
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

pub(crate) fn validate_replacement_peak(
    current: PlanResources,
    prospective: PlanResources,
    prospective_carried: CarriedSourceBytes,
    prospective_capi: CapiResources,
    compiled_models: CompiledModelAdmission,
    limits: ControlLimits,
) -> Result<(), CompileFailure> {
    let combined = |left: u64, right: u64| {
        left.checked_add(right)
            .ok_or_else(|| failure(ResourceFault::Arithmetic))
    };
    let graph_peak = combined(
        current.graph_session_plus_plan_bytes,
        prospective.graph_session_plus_plan_bytes,
    )?
    .checked_add(compiled_models.retained_bytes)
    .ok_or_else(|| failure(ResourceFault::Arithmetic))?;
    if graph_peak > limits.maximum_graph_session_plus_plan_bytes {
        return Err(diagnostic("graph.resource.limit"));
    }
    // Issue #1273 D5: a carried ring is the current plan's until the swap, so the double-live
    // peak counts it once, there.
    let allocated = |row: u64, carried: u64| {
        row.checked_sub(carried)
            .ok_or_else(|| failure(ResourceFault::Arithmetic))
    };
    if combined(
        current.source_total_bytes,
        allocated(prospective.source_total_bytes, prospective_carried.total)?,
    )? > limits.maximum_source_total_bytes
        || combined(
            current.source_overhead_bytes,
            allocated(
                prospective.source_overhead_bytes,
                prospective_carried.overhead,
            )?,
        )? > limits.maximum_source_overhead_bytes
    {
        return Err(diagnostic("source.resource.limit"));
    }
    // #1290a D3: the categories above each fit alone; their sum must fit an adapter's one budget.
    if combined(
        graph_peak,
        combined(
            current.source_total_bytes,
            allocated(prospective.source_total_bytes, prospective_carried.total)?,
        )?,
    )? > limits.maximum_replacement_peak_bytes
    {
        return Err(diagnostic("control.resource.replacement_peak"));
    }
    if combined(
        current.effect_scalar_state_bytes,
        prospective.effect_scalar_state_bytes,
    )? > limits.maximum_effect_state_bytes
        || combined(
            current.effect_scalar_scratch_bytes,
            prospective.effect_scalar_scratch_bytes,
        )? > limits.maximum_effect_scratch_bytes
    {
        return Err(diagnostic("effect.resource.limit"));
    }
    if combined(
        current.builtin_retained_payload_bytes,
        prospective.builtin_retained_payload_bytes,
    )? > limits.maximum_builtin_retained_bytes
    {
        return Err(failure(ResourceFault::Limit));
    }
    let capi_peak = current
        .control_retained_bytes
        .checked_add(prospective_capi.epoch_retained)
        .and_then(|value| value.checked_add(prospective_capi.prepared_protocol_retained))
        .ok_or_else(|| failure(ResourceFault::Arithmetic))?;
    if capi_peak > limits.maximum_control_retained_bytes {
        return Err(failure(ResourceFault::Limit));
    }
    if current
        .largest_named_allocation_bytes
        .max(prospective.largest_named_allocation_bytes)
        .max(prospective_capi.largest)
        .max(compiled_models.largest_allocation_bytes)
        > limits.maximum_named_allocation_bytes
    {
        return Err(failure(ResourceFault::Limit));
    }
    Ok(())
}

/// One provider epoch's resources as the live admission reads them: its plan's report row and the
/// capi resources it was prepared with (#1257 D4).
///
/// Public only to the adapters' tests, under `test-support`; crate-private otherwise.
#[derive(Clone, Copy)]
#[cfg_attr(not(feature = "test-support"), allow(unreachable_pub))]
pub struct LiveEpochResources {
    /// The epoch's plan row, read back from the adapter's report table.
    pub report: PlanResources,
    /// The capi resources the epoch's plan was prepared with.
    pub capi: CapiResources,
}

/// The live arm's admission (#1053 D8, #1257 D5): may the prospective compiled model live beside
/// the current one, next to the plans that exist, until the commit?
///
/// The live arm prepares no plan, so the plans are the current epoch's and, when a candidate is
/// pending, that candidate's; the newest of them is the pending one if any, else the current one.
/// The terms, each against its own cap:
///
/// - **graph:** both plans' `graph_session_plus_plan_bytes` plus both compiled models'
///   `retained_bytes`;
/// - **capi:** the newest plan's `control_retained_bytes`, plus the current epoch's `epoch_retained`
///   while a candidate is pending, plus the newest epoch's `prepared_protocol_retained`. That last
///   term includes a provider catalog (`catalog_retained_bytes`), which a live commit replaces by
///   the committed model's (#1552 D3), so the term is the same one a rebuild charges;
/// - **catalog growth:** the new catalog's retained bytes beyond the current catalog's are added to
///   the capi term, and its largest single allocation joins the largest-allocation term, so a VCA
///   added live that grows the catalog meets the ceiling a rebuild would (#1558 D6);
/// - **largest allocation:** the largest of both plans' `largest_named_allocation_bytes`, the
///   newest epoch's capi `largest`, the compiled models' largest allocation and the new catalog's.
///
/// Public only to the adapters' tests, under `test-support`; crate-private otherwise.
#[cfg_attr(not(feature = "test-support"), allow(unreachable_pub))]
pub fn validate_live_peak(
    current: LiveEpochResources,
    pending: Option<LiveEpochResources>,
    compiled_models: CompiledModelAdmission,
    catalog: LiveCatalogAdmission,
    limits: ControlLimits,
) -> Result<(), CompileFailure> {
    let arithmetic = || failure(ResourceFault::Arithmetic);
    let newest = pending.unwrap_or(current);
    let graph = current
        .report
        .graph_session_plus_plan_bytes
        .checked_add(pending.map_or(0, |pending| pending.report.graph_session_plus_plan_bytes))
        .and_then(|value| value.checked_add(compiled_models.retained_bytes))
        .ok_or_else(arithmetic)?;
    if graph > limits.maximum_graph_session_plus_plan_bytes {
        return Err(diagnostic("graph.resource.limit"));
    }
    let capi = newest
        .report
        .control_retained_bytes
        .checked_add(pending.map_or(0, |_| current.capi.epoch_retained))
        .and_then(|value| value.checked_add(newest.capi.prepared_protocol_retained))
        .and_then(|value| value.checked_add(catalog.growth_bytes))
        .ok_or_else(arithmetic)?;
    if capi > limits.maximum_control_retained_bytes {
        return Err(failure(ResourceFault::Limit));
    }
    if current
        .report
        .largest_named_allocation_bytes
        .max(pending.map_or(0, |pending| pending.report.largest_named_allocation_bytes))
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
        maximum_meter_streams: limits.maximum_meter_streams,
        maximum_meter_items: limits.maximum_meter_items,
        maximum_meter_bytes: limits.maximum_meter_bytes,
    }
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
        successor: Option<SuccessorBase<'_>>,
        superseded: &host_core::LiveValuesSuperseded,
    ) -> Result<(host_core::PreparedHost, host_core::HostLiveControlHandles), PrepareDiagnostics>;
}

/// Prepare one plan plus its source producers, and project its resource accounting.
///
/// The shared pipeline is `host-core`; the control plane adds only what is its own: its retained
/// rows (protocol queues, replay storage, the adapter's handle structs), and the resource shape.
///
/// With a `successor` base (a structural transaction, issue #1273 D1) every source the
/// transaction leaves unchanged is prepared vacant, to carry the displaced plan's ring; the
/// report's source rows count those rings too (D5). `compile_session` passes `None`.
pub(crate) fn prepare_runtime<A: ControlAdapter>(
    preparer: &dyn RuntimePreparer,
    compiled: &CompiledSession,
    limits: ControlLimits,
    successor: Option<SuccessorBase<'_>>,
    superseded: &host_core::LiveValuesSuperseded,
) -> Result<PreparedRuntime, CompileFailure> {
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
    let (prepared, handles) = preparer
        .prepare(compiled, &caps, lanes, successor, superseded)
        .map_err(prepare_failure)?;
    // #1256 D2: keep only the producers. The strip ID list and the vectors this selection leaves
    // empty drop here, on the control thread; each producer keeps its own `track_id`. The
    // producer vector is built at exactly one entry per strip, so the boxed slice does not
    // reallocate.
    let host_core::HostLiveControlHandles {
        strip_controls,
        track_count,
        effect_controls,
        route_controls,
        ..
    } = handles;
    let strips = StripLanes {
        controls: strip_controls.into_boxed_slice(),
        track_count,
        // #1557 D4: the send producers, at exactly one entry per route lane the plan attached.
        routes: route_controls.into_boxed_slice(),
    };
    // #1263 D2: likewise one entry per effect instance, built at exactly that capacity, so the
    // boxed slice is the allocation host-core walked for `effect_control_resources`.
    let effects = effect_controls.into_boxed_slice();
    let capi = prepared_capi_resources::<A>(
        compiled,
        &prepared.control_catalog,
        &prepared.inventory,
        prepared.report.effect_control_resources,
        prepared.report.route_control_resources,
        observation_resources(&effects)?,
        limits,
    )?;
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
        plan,
        revision_gate,
        resources: PlanResources {
            sample_rate_hz: host.sample_rate_hz,
            quantum_frames: host.quantum_frames,
            source_count: host.source_count,
            track_count: host.track_count,
            latency_samples: host.latency_samples,
            output_tail: host.output_tail,
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
    })
}

/// Compile `document` under `limits` into a control session and the plan it drives.
///
/// The adapter keeps its own per-session diagnostic storage; it builds that after this returns.
///
/// `preparer` (issue #1400 D2) is the adapter's preparation: the session stores it and prepares
/// the boot plan and every structural successor through it, and through nothing else.
pub fn compile_children<A: ControlAdapter>(
    document: &str,
    mut limits: ControlLimits,
    preparer: Box<dyn RuntimePreparer>,
) -> Result<CompiledChildren<A>, CompileFailure>
where
    A::Row: Send + 'static,
{
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
        None,
        &live_values_superseded,
    )?;

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
    let one = NonZeroUsize::new(1).expect("one is nonzero");
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
        plan,
        mut revision_gate,
        resources,
        carried: _,
        inventory,
        control_catalog,
        capi,
    } = runtime;
    // #1314 D1, #1502 D5: the initial plan carries the store's initial revision, published on its
    // own gate, and the watermark starts there.
    let (publisher, owner, retirer) = plan_exchange_at_revision(
        plan,
        &mut revision_gate,
        store.revision().0,
        PlanExchangeConfig {
            retirement_capacity: one,
        },
    )
    .map_err(|_| failure(ResourceFault::PlanExchange))?;
    let mut reports = Vec::new();
    reports
        .try_reserve_exact(2)
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
        watermark: publisher.watermark_reader(),
    });
    let sample_source: Arc<dyn host_core::PlanSampleSource> = Arc::clone(&shared) as Arc<_>;
    let provider = SessionControlProvider::try_new(
        control_catalog,
        store.revision().0,
        sample_source,
        retained_capacity,
        RENDER_DIAGNOSTIC_SLOTS,
        // A reply must fit the endpoint's frame and the replay cache's response bound, both the
        // host's control frame limit.
        control_bytes,
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
        },
        retained_capacity,
    )
    .map_err(|_| failure(ResourceFault::Allocation))?;
    let mut pending_providers = Vec::new();
    pending_providers
        .try_reserve_exact(1)
        .map_err(|_| failure(ResourceFault::Allocation))?;
    let mut retired_providers = Vec::new();
    retired_providers
        .try_reserve_exact(1)
        .map_err(|_| failure(ResourceFault::Allocation))?;
    let decode_field_count = control_bytes / size_of::<u16>();
    let mut decode_fields = Vec::new();
    decode_fields
        .try_reserve_exact(decode_field_count)
        .map_err(|_| failure(ResourceFault::Allocation))?;
    decode_fields.resize(decode_field_count, 0);
    let mut session = SessionState {
        controller: ObservedController::new(controller),
        providers: ProviderEpoch::current(sources, inventory, strips, effects, capi, revision_gate),
        pending_providers,
        retired_providers,
        publisher,
        retirer,
        limits,
        preparer,
        live_values_superseded,
        decode_fields: decode_fields.into_boxed_slice(),
        response_scratch: boxed_zeroed(limits.maximum_control_frame_bytes)?,
        shared: Arc::clone(&shared),
        observed_render_sequence: 0,
        counter_snapshot_sequence: 0,
        render_diagnostics: prepare_render_diagnostic_slots()?,
        render_diagnostic_head: 0,
        render_diagnostic_len: 0,
        protocol_reliable_pending: false,
        monitor: controls::MonitorOverlay::new(),
        observation: controls::ObservationBindings::new(),
        observation_pending: Box::default(),
        observation_offsets: Box::default(),
    };
    (session.observation_offsets, session.observation_pending) =
        crate::control::observation_slots(&session.providers.effects)
            .map_err(|_| failure(ResourceFault::Allocation))?;
    Ok(CompiledChildren {
        session,
        plan: PlanState::new(owner, shared),
    })
}
