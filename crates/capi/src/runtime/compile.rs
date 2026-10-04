//! capi's own resource projection, and the children `miso_engine_v1_compile_session` returns.

use super::*;

pub(crate) struct CompiledChildren {
    pub(crate) session: SessionState,
    pub(crate) session_error: FixedBytes,
    pub(crate) plan: PlanState,
}

/// Every live lane's depth: each strip's fader/mute and matrix/pan rings hold 16 records (#1053 D4).
pub(crate) const LIVE_QUEUE_DEPTH: NonZeroUsize =
    NonZeroUsize::new(16).expect("sixteen is nonzero");

pub(crate) struct PreparedRuntime {
    pub(crate) sources: SourceControlSet,
    pub(crate) strips: StripLanes,
    pub(crate) plan: PreparedRenderPlan,
    pub(crate) resources: PlanResourceReport,
    pub(crate) control_catalog: PreparedSessionControlCatalog,
    pub(crate) capi: CapiResources,
}

pub(crate) fn boxed_zeroed(bytes: u64) -> Result<Box<[u8]>, CompileFailure> {
    Ok(FixedBytes::try_new(bytes)?.bytes)
}

pub(crate) fn checked_layout<T>(count: usize) -> Result<u64, CompileFailure> {
    let layout = Layout::array::<T>(count).map_err(|_| failure("capi.resource.arithmetic"))?;
    u64::try_from(layout.size()).map_err(|_| failure("capi.resource.platform"))
}

pub(crate) fn checked_byte_layout(bytes: u64) -> Result<u64, CompileFailure> {
    checked_layout::<u8>(usize::try_from(bytes).map_err(|_| failure("capi.resource.platform"))?)
}

#[derive(Clone, Copy)]
pub(crate) struct CapiResources {
    pub(crate) active_retained: u64,
    pub(crate) epoch_retained: u64,
    pub(crate) prepared_protocol_retained: u64,
    pub(crate) largest: u64,
}

#[derive(Clone, Copy)]
pub(crate) struct CompiledModelAdmission {
    pub(crate) retained_bytes: u64,
    pub(crate) largest_allocation_bytes: u64,
}

pub(crate) fn compiled_model_admission(
    current: &CompiledSession,
    prospective: &CompiledSession,
) -> Result<CompiledModelAdmission, CompileFailure> {
    Ok(CompiledModelAdmission {
        retained_bytes: current
            .resource_estimate()
            .compiled_model_bytes
            .checked_add(prospective.resource_estimate().compiled_model_bytes)
            .ok_or_else(|| failure("capi.resource.arithmetic"))?,
        largest_allocation_bytes: current
            .resource_estimate()
            .single_allocation_bytes
            .max(prospective.resource_estimate().single_allocation_bytes),
    })
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
            .ok_or_else(|| failure("capi.resource.arithmetic"))
    })
}

pub(crate) fn protocol_queue_config(
    limits: CompileLimits,
    quantum_frames: usize,
) -> Result<ProtocolQueueConfig, CompileFailure> {
    let one = NonZeroUsize::new(1).expect("one is nonzero");
    Ok(ProtocolQueueConfig {
        control_command_slots: one,
        control_command_bytes: NonZeroUsize::new(
            usize::try_from(limits.maximum_control_frame_bytes)
                .map_err(|_| failure("capi.resource.platform"))?,
        )
        .ok_or_else(|| failure("capi.resource.limit"))?,
        automation_batch_slots: one,
        reliable_response_slots: one,
        reliable_event_slots: NonZeroUsize::new(2).expect("two is nonzero"),
        telemetry_slots: one,
        per_block_automation_density: NonZeroUsize::new(
            limits.maximum_automation_spans_per_block as usize,
        )
        .ok_or_else(|| failure("capi.resource.limit"))?,
        quantum_frames: NonZeroUsize::new(quantum_frames)
            .ok_or_else(|| failure("capi.resource.limit"))?,
    })
}

pub(crate) fn capi_resources(
    limits: CompileLimits,
    source_count: usize,
    source_id_bytes: usize,
    strip_count: usize,
    strip_id_bytes: usize,
    quantum_frames: usize,
    provider: host_core::SessionControlProviderResources,
) -> Result<CapiResources, CompileFailure> {
    let queue_config = protocol_queue_config(limits, quantum_frames)?;
    let queue = ProtocolQueues::resource_report_for_config(queue_config)
        .map_err(|_| failure("capi.resource.arithmetic"))?;
    let replay_config = ReplayCacheConfig {
        entries: NonZeroUsize::new(
            usize::try_from(limits.maximum_replay_entries)
                .map_err(|_| failure("capi.resource.platform"))?,
        )
        .ok_or_else(|| failure("capi.resource.limit"))?,
        bytes: NonZeroUsize::new(
            usize::try_from(limits.maximum_replay_bytes)
                .map_err(|_| failure("capi.resource.platform"))?,
        )
        .ok_or_else(|| failure("capi.resource.limit"))?,
        max_response_bytes: usize::try_from(limits.maximum_control_frame_bytes)
            .map_err(|_| failure("capi.resource.platform"))?,
    };
    let replay = ReplayCache::resource_report_for_config(replay_config)
        .map_err(|_| failure("capi.resource.arithmetic"))?;
    let exchange = plan_exchange_resource_report(PlanExchangeConfig {
        publication_capacity: NonZeroUsize::new(1).expect("one is nonzero"),
        retirement_capacity: NonZeroUsize::new(1).expect("one is nonzero"),
    })
    .map_err(|_| failure("capi.resource.arithmetic"))?;
    // The control-source table and ID arena are the facade's own layout; capi reads the mirror
    // (`control_table_bytes` / `source_id_arena_bytes`) rather than restating the struct, so this
    // pre-flight cannot drift when that struct changes.
    //
    // The session's canonical JSON is not a capi row. The compiled session owns it, and its
    // `compiled_model_bytes` already charges it to the graph cap, current and prospective alike
    // (`validate_replacement_peak`); charging it here too counted one allocation twice (#1060).
    //
    // The strip producer table (#1256 D3) is the epoch's too, through host-core's mirror. Its
    // rings are builtins' rows, charged in `builtin_retained_payload_bytes`. Builtins' processor
    // accumulator also charges the producer vector itself (`add_vector_layout::<
    // TrackControlProducer>`), so that vector is charged twice, here against the capi cap and
    // there against the builtin cap: #1256 D4's recorded, conservative double charge. Removing
    // either side is a deliberate decision, not a cleanup.
    let epoch_rows = [
        host_core::control_table_bytes(source_count)
            .ok_or_else(|| failure("capi.resource.arithmetic"))?,
        host_core::source_id_arena_bytes(source_id_bytes)
            .ok_or_else(|| failure("capi.resource.arithmetic"))?,
        host_core::strip_control_table_bytes(strip_count, strip_id_bytes)
            .ok_or_else(|| failure("capi.resource.arithmetic"))?,
    ];
    let maximum_configuration_items = usize::try_from(limits.maximum_control_frame_bytes)
        .map_err(|_| failure("capi.resource.platform"))?
        / size_of::<u16>();
    let fixed_allocation_rows = [
        checked_byte_layout(limits.maximum_diagnostic_bytes)?,
        checked_byte_layout(limits.maximum_control_frame_bytes)?,
        checked_byte_layout(limits.maximum_control_frame_bytes)?,
        checked_layout::<SharedArcAllocation<AtomicU64>>(1)?,
        checked_layout::<SharedArcAllocation<SharedPlanState>>(1)?,
        checked_layout::<RetainedDiagnosticSlotMirror>(2)?,
        checked_layout::<RenderDiagnosticSlot>(RENDER_DIAGNOSTIC_SLOTS)?,
        checked_layout::<u8>(RENDER_DIAGNOSTIC_CODE.len() * RENDER_DIAGNOSTIC_SLOTS)?,
        // ProtocolController and SessionControlProvider each retain their own telemetry config.
        checked_layout::<u32>(maximum_configuration_items)?,
        checked_layout::<protocol::CounterId>(maximum_configuration_items)?,
        provider.fixed_retained_bytes,
        checked_layout::<u32>(maximum_configuration_items)?,
        checked_layout::<protocol::CounterId>(maximum_configuration_items)?,
        checked_layout::<ProviderEpoch>(2)?,
        checked_layout::<(u64, PlanResourceReport)>(2)?,
        checked_layout::<crate::Session>(1)?,
        checked_layout::<crate::Plan>(1)?,
    ];
    let fixed_aggregate_rows = [
        queue.retained_payload_bytes,
        replay.retained_payload_bytes,
        exchange.retained_payload_bytes,
    ];
    let prepared_protocol_allocation_rows = [
        checked_byte_layout(limits.maximum_control_frame_bytes)?,
        checked_layout::<protocol::PreparedStructuralCommand>(1)?,
    ];
    let prepared_protocol_aggregate_rows = [replay.retained_payload_bytes];
    let epoch_retained = checked_sum(&epoch_rows)?;
    let active_retained = checked_sum(&fixed_allocation_rows)?
        .checked_add(checked_sum(&fixed_aggregate_rows)?)
        .and_then(|value| value.checked_add(epoch_retained))
        .and_then(|value| value.checked_add(provider.catalog_retained_bytes))
        .ok_or_else(|| failure("capi.resource.arithmetic"))?;
    let prepared_protocol_retained = checked_sum(&prepared_protocol_allocation_rows)?
        .checked_add(checked_sum(&prepared_protocol_aggregate_rows)?)
        .and_then(|value| value.checked_add(provider.catalog_retained_bytes))
        .ok_or_else(|| failure("capi.resource.arithmetic"))?;
    // Conservative: the strip row is the producer slice plus every `track_id`, which are separate
    // allocations, so feeding the whole row overstates the largest single allocation when that
    // row is the maximum. The error only ever refuses earlier.
    let largest = epoch_rows
        .into_iter()
        .chain(fixed_allocation_rows)
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

pub(crate) fn prepared_capi_resources(
    compiled: &CompiledSession,
    catalog: &PreparedSessionControlCatalog,
    limits: CompileLimits,
) -> Result<CapiResources, CompileFailure> {
    let source_id_bytes =
        compiled
            .normalized_model()
            .sources
            .iter()
            .try_fold(0_usize, |total, source| {
                total
                    .checked_add(source.id.as_str().len())
                    .ok_or_else(|| failure("capi.resource.arithmetic"))
            })?;
    // Every strip, tracks and submixes alike, carries one producer whose `track_id` is its ID.
    let (strip_count, strip_id_bytes) = compiled.normalized_model().strips().try_fold(
        (0_usize, 0_usize),
        |(count, bytes), strip| {
            bytes
                .checked_add(strip.id.as_str().len())
                .map(|bytes| (count + 1, bytes))
                .ok_or_else(|| failure("capi.resource.arithmetic"))
        },
    )?;
    let provider = SessionControlProvider::resource_report(
        catalog,
        controller_retained_capacity(limits)?,
        RENDER_DIAGNOSTIC_SLOTS,
    )
    .map_err(|_| failure("capi.resource.arithmetic"))?;
    capi_resources(
        limits,
        compiled.source_count(),
        source_id_bytes,
        strip_count,
        strip_id_bytes,
        compiled.quantum().0 as usize,
        provider,
    )
}

pub(crate) fn controller_retained_capacity(
    limits: CompileLimits,
) -> Result<ControllerRetainedCapacity, CompileFailure> {
    let control_bytes = usize::try_from(limits.maximum_control_frame_bytes)
        .map_err(|_| failure("capi.resource.platform"))?;
    let maximum_tlvs = control_bytes / size_of::<u16>();
    Ok(ControllerRetainedCapacity {
        meter_handles: maximum_tlvs,
        counter_ids: maximum_tlvs,
    })
}

pub(crate) fn validate_replacement_peak(
    current: PlanResourceReport,
    prospective: PlanResourceReport,
    prospective_capi: CapiResources,
    compiled_models: CompiledModelAdmission,
    limits: CompileLimits,
) -> Result<(), CompileFailure> {
    let combined = |left: u64, right: u64| {
        left.checked_add(right)
            .ok_or_else(|| failure("capi.resource.arithmetic"))
    };
    if combined(
        current.graph_session_plus_plan_bytes,
        prospective.graph_session_plus_plan_bytes,
    )?
    .checked_add(compiled_models.retained_bytes)
    .ok_or_else(|| failure("capi.resource.arithmetic"))?
        > limits.maximum_graph_session_plus_plan_bytes
    {
        return Err(failure("graph.resource.limit"));
    }
    if combined(current.source_total_bytes, prospective.source_total_bytes)?
        > limits.maximum_source_total_bytes
        || combined(
            current.source_overhead_bytes,
            prospective.source_overhead_bytes,
        )? > limits.maximum_source_overhead_bytes
    {
        return Err(failure("source.resource.limit"));
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
        return Err(failure("effect.resource.limit"));
    }
    if combined(
        current.builtin_retained_payload_bytes,
        prospective.builtin_retained_payload_bytes,
    )? > limits.maximum_builtin_retained_bytes
    {
        return Err(failure("capi.resource.limit"));
    }
    let capi_peak = current
        .capi_retained_bytes
        .checked_add(prospective_capi.epoch_retained)
        .and_then(|value| value.checked_add(prospective_capi.prepared_protocol_retained))
        .ok_or_else(|| failure("capi.resource.arithmetic"))?;
    if capi_peak > limits.maximum_capi_retained_bytes {
        return Err(failure("capi.resource.limit"));
    }
    if current
        .largest_named_allocation_bytes
        .max(prospective.largest_named_allocation_bytes)
        .max(prospective_capi.largest)
        .max(compiled_models.largest_allocation_bytes)
        > limits.maximum_named_allocation_bytes
    {
        return Err(failure("capi.resource.limit"));
    }
    Ok(())
}

/// One provider epoch's resources as the live admission reads them: its plan's report row and the
/// capi resources it was prepared with (#1257 D4).
#[derive(Clone, Copy)]
pub(crate) struct LiveEpochResources {
    pub(crate) report: PlanResourceReport,
    pub(crate) capi: CapiResources,
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
/// - **capi:** the newest plan's `capi_retained_bytes`, plus the current epoch's `epoch_retained`
///   while a candidate is pending, plus the newest epoch's `prepared_protocol_retained`. That last
///   term includes a provider catalog (`catalog_retained_bytes`) that the live arm never builds,
///   because it never replaces the catalog: a conservative overcount, kept so that the term is the
///   same one a rebuild charges;
/// - **largest allocation:** the largest of both plans' `largest_named_allocation_bytes`, the
///   newest epoch's capi `largest` and the compiled models' largest allocation.
pub(crate) fn validate_live_peak(
    current: LiveEpochResources,
    pending: Option<LiveEpochResources>,
    compiled_models: CompiledModelAdmission,
    limits: CompileLimits,
) -> Result<(), CompileFailure> {
    let arithmetic = || failure("capi.resource.arithmetic");
    let newest = pending.unwrap_or(current);
    let graph = current
        .report
        .graph_session_plus_plan_bytes
        .checked_add(pending.map_or(0, |pending| pending.report.graph_session_plus_plan_bytes))
        .and_then(|value| value.checked_add(compiled_models.retained_bytes))
        .ok_or_else(arithmetic)?;
    if graph > limits.maximum_graph_session_plus_plan_bytes {
        return Err(failure("graph.resource.limit"));
    }
    let capi = newest
        .report
        .capi_retained_bytes
        .checked_add(pending.map_or(0, |_| current.capi.epoch_retained))
        .and_then(|value| value.checked_add(newest.capi.prepared_protocol_retained))
        .ok_or_else(arithmetic)?;
    if capi > limits.maximum_capi_retained_bytes {
        return Err(failure("capi.resource.limit"));
    }
    if current
        .report
        .largest_named_allocation_bytes
        .max(pending.map_or(0, |pending| pending.report.largest_named_allocation_bytes))
        .max(newest.capi.largest)
        .max(compiled_models.largest_allocation_bytes)
        > limits.maximum_named_allocation_bytes
    {
        return Err(failure("capi.resource.limit"));
    }
    Ok(())
}

pub(crate) fn all_limits_nonzero(limits: CompileLimits) -> bool {
    limits.maximum_automation_spans_per_block != 0
        && [
            limits.maximum_document_bytes,
            limits.maximum_diagnostic_bytes,
            limits.maximum_tracks,
            limits.maximum_sources,
            limits.maximum_routes,
            limits.maximum_effects,
            limits.maximum_graph_session_plus_plan_bytes,
            limits.maximum_source_total_bytes,
            limits.maximum_source_overhead_bytes,
            limits.maximum_effect_state_bytes,
            limits.maximum_effect_scratch_bytes,
            limits.maximum_builtin_retained_bytes,
            limits.maximum_capi_retained_bytes,
            limits.maximum_named_allocation_bytes,
            limits.maximum_meter_streams,
            limits.maximum_meter_items,
            limits.maximum_meter_bytes,
            limits.maximum_control_frame_bytes,
            limits.maximum_replay_bytes,
            limits.maximum_replay_entries,
        ]
        .into_iter()
        .all(|value| value != 0)
}

pub(crate) fn limits_are_valid(limits: CompileLimits) -> bool {
    limits.struct_size == crate::COMPILE_LIMITS_SIZE
        && limits.reserved0 == 0
        && limits.reserved == [0; 2]
        && all_limits_nonzero(limits)
}

/// Translate the frozen C ABI limits into the facade's caps, field for field, except
/// `maximum_submixes` and `maximum_vcas`, whose zero means `maximum_tracks` (#1206 D2, #1243 D2).
///
/// This is the only place the mapping is spelled. `AnyLaunchRate`: the C ABI compiles whatever
/// launch rate the session declares (issue 032), unlike the browser host which is pinned to its
/// `AudioContext`. `maximum_source_channels: None`: the C ABI has no such limit field.
pub(crate) fn prepare_caps(limits: CompileLimits) -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: limits.source_ring_frames,
        maximum_source_channels: None,
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
    CompileFailure {
        diagnostics: diagnostics.into_bytes(),
    }
}

/// Prepare one plan plus its source producers, and project the frozen ABI resource report.
///
/// The shared pipeline is `host-core`; capi adds only what is capi's: its own retained
/// rows (protocol queues, replay storage, handle structs), and the ABI report shape.
pub(crate) fn prepare_runtime(
    compiled: &CompiledSession,
    limits: CompileLimits,
) -> Result<PreparedRuntime, CompileFailure> {
    let caps = prepare_caps(limits);
    // Shape still runs before host/runtime allocation. The exact provider catalog exists only
    // after shared host preparation, so CAPI retained-resource admission necessarily follows the
    // full plan allocation; #369 records that allocation and diagnostic-precedence consequence.
    caps.validate_shape(compiled).map_err(prepare_failure)?;
    // #1256 D1: every plan carries each strip's fader/mute and matrix/pan lanes, and nothing else
    // live: no input lane (its tail would turn infinite), no effect or route lane.
    let (prepared, handles) = prepare_host_runtime_with_live_lanes(
        compiled,
        &caps,
        &HostLiveControlRequest {
            control_queue_depth: Some(LIVE_QUEUE_DEPTH),
            ..HostLiveControlRequest::default()
        },
        HostLiveLanes::FADER_AND_MATRIX,
    )
    .map_err(prepare_failure)?;
    // #1256 D2: keep only the producers. The strip ID list and the vectors this selection leaves
    // empty drop here, on the control thread; each producer keeps its own `track_id`. The
    // producer vector is built at exactly one entry per strip, so the boxed slice does not
    // reallocate.
    let host_core::HostLiveControlHandles {
        strip_controls,
        track_count,
        ..
    } = handles;
    let strips = StripLanes {
        controls: strip_controls.into_boxed_slice(),
        track_count,
    };
    let capi = prepared_capi_resources(compiled, &prepared.control_catalog, limits)?;
    if capi.active_retained > limits.maximum_capi_retained_bytes
        || capi.largest > limits.maximum_named_allocation_bytes
    {
        return Err(failure("capi.resource.limit"));
    }
    let host = prepared.report;
    let largest_named = host.largest_engine_allocation_bytes.max(capi.largest);
    let (tail_kind, tail_samples) = match host.output_tail {
        TailSamples::Finite(samples) => (TAIL_FINITE, samples),
        TailSamples::Infinite => (TAIL_INFINITE, 0),
    };
    Ok(PreparedRuntime {
        sources: prepared.sources,
        strips,
        plan: prepared.plan,
        resources: PlanResourceReport {
            struct_size: crate::PLAN_RESOURCE_REPORT_SIZE,
            abi_version: ABI_VERSION,
            sample_rate_hz: host.sample_rate_hz,
            quantum_frames: host.quantum_frames,
            source_count: host.source_count,
            track_count: host.track_count,
            latency_samples: host.latency_samples,
            tail_kind,
            tail_samples,
            graph_session_plus_plan_bytes: host.graph_session_plus_plan_bytes,
            graph_incremental_plan_bytes: host.graph_incremental_plan_bytes,
            graph_metadata_bytes: host.graph_metadata_bytes,
            graph_delay_bytes: host.graph_delay_bytes,
            effect_bank_scratch_bytes: host.effect_bank_scratch_bytes,
            effect_bank_runtime_buffer_bytes: host.effect_bank_runtime_buffer_bytes,
            effect_bank_metadata_bytes: host.effect_bank_metadata_bytes,
            builtin_bank_bytes: host.builtin_bank_bytes,
            builtin_bank_scratch_bytes: host.builtin_bank_scratch_bytes,
            source_pcm_payload_bytes: host.source_pcm_payload_bytes,
            source_overhead_bytes: host.source_overhead_bytes,
            source_total_bytes: host.source_total_bytes,
            effect_scalar_state_bytes: host.effect_scalar_state_bytes,
            effect_scalar_scratch_bytes: host.effect_scalar_scratch_bytes,
            builtin_processor_payload_bytes: host.builtin_processor_payload_bytes,
            builtin_meter_payload_bytes: host.builtin_meter_payload_bytes,
            builtin_retained_payload_bytes: host.builtin_retained_payload_bytes,
            capi_retained_bytes: capi.active_retained,
            largest_named_allocation_bytes: largest_named,
            reserved: [0; 4],
        },
        control_catalog: prepared.control_catalog,
        capi,
    })
}

pub(crate) fn compile_children(
    document: &str,
    mut limits: CompileLimits,
) -> Result<CompiledChildren, CompileFailure> {
    // The C ABI needs the transactional `SessionStore` for the control protocol, so it parses and
    // caps through the facade and builds the store itself; the facade never sees the protocol.
    let model = parse_host_session(document).map_err(prepare_failure)?;
    if limits.source_ring_frames == 0 {
        limits.source_ring_frames =
            host_core::default_source_ring_frames(model.sample_rate_hz, model.quantum_frames);
        if limits.source_ring_frames == 0 {
            return Err(failure("capi.resource.arithmetic"));
        }
    }
    let compile_caps = prepare_caps(limits)
        .compile_caps(model.sources.len())
        .map_err(prepare_failure)?;
    let store =
        SessionStore::new(model, compile_caps).map_err(|value| session_diagnostics(&value))?;
    let runtime = prepare_runtime(store.compiled(), limits)?;

    let control_bytes = usize::try_from(limits.maximum_control_frame_bytes)
        .map_err(|_| failure("capi.resource.platform"))?;
    let replay_bytes = usize::try_from(limits.maximum_replay_bytes)
        .map_err(|_| failure("capi.resource.platform"))?;
    let replay_entries = usize::try_from(limits.maximum_replay_entries)
        .map_err(|_| failure("capi.resource.platform"))?;
    let quantum_frames = usize::try_from(store.compiled().quantum().0)
        .map_err(|_| failure("capi.resource.platform"))?;
    let maximum_tlvs = u32::try_from(control_bytes / size_of::<u16>()).unwrap_or(u32::MAX);
    let codec = ProtocolCodec::new(ProtocolLimits {
        max_frame_bytes: control_bytes,
        max_tlv_count: maximum_tlvs,
        max_string_bytes: control_bytes,
        max_nesting: 4,
    });
    let one = NonZeroUsize::new(1).expect("one is nonzero");
    let queues = ProtocolQueues::prepare(protocol_queue_config(limits, quantum_frames)?)
        .map_err(|_| failure("capi.protocol.queue"))?;
    let replay = ReplayCache::try_new(ReplayCacheConfig {
        entries: NonZeroUsize::new(replay_entries).ok_or_else(|| failure("capi.resource.limit"))?,
        bytes: NonZeroUsize::new(replay_bytes).ok_or_else(|| failure("capi.resource.limit"))?,
        max_response_bytes: control_bytes,
    })
    .map_err(|_| failure("capi.resource.allocation"))?;
    let retained_capacity = controller_retained_capacity(limits)?;
    let PreparedRuntime {
        sources,
        strips,
        plan,
        resources,
        control_catalog,
        capi,
    } = runtime;
    let (publisher, owner, retirer) = plan_exchange(
        plan,
        PlanExchangeConfig {
            publication_capacity: one,
            retirement_capacity: one,
        },
    )
    .map_err(|_| failure("capi.plan.exchange"))?;
    let mut reports = Vec::new();
    reports
        .try_reserve_exact(2)
        .map_err(|_| failure("capi.resource.allocation"))?;
    reports.push((0, resources));
    let shared = Arc::new(SharedPlanState {
        plan_alive: AtomicBool::new(true),
        active_epoch: AtomicU64::new(0),
        reports: Mutex::new(reports),
        render_sequence: AtomicU64::new(0),
        render_sample: AtomicU64::new(0),
        render_peak_bits: AtomicU32::new(0),
        // No endpoint has configured telemetry yet, so nothing can read a peak.
        render_peak_observed: AtomicBool::new(false),
    });
    let sample_source: Arc<dyn host_core::PlanSampleSource> = Arc::clone(&shared) as Arc<_>;
    let provider = SessionControlProvider::try_new(
        control_catalog,
        sample_source,
        retained_capacity,
        RENDER_DIAGNOSTIC_SLOTS,
    )
    .map_err(|_| failure("capi.resource.allocation"))?;
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
    .map_err(|_| failure("capi.resource.allocation"))?;
    let mut pending_providers = Vec::new();
    pending_providers
        .try_reserve_exact(1)
        .map_err(|_| failure("capi.resource.allocation"))?;
    let mut retired_providers = Vec::new();
    retired_providers
        .try_reserve_exact(1)
        .map_err(|_| failure("capi.resource.allocation"))?;
    let decode_field_count = control_bytes / size_of::<u16>();
    let mut decode_fields = Vec::new();
    decode_fields
        .try_reserve_exact(decode_field_count)
        .map_err(|_| failure("capi.resource.allocation"))?;
    decode_fields.resize(decode_field_count, 0);
    Ok(CompiledChildren {
        session: SessionState {
            controller: ObservedController::new(controller),
            providers: ProviderEpoch::current(sources, strips, capi),
            pending_providers,
            retired_providers,
            publisher,
            retirer,
            limits,
            decode_fields: decode_fields.into_boxed_slice(),
            response_scratch: boxed_zeroed(limits.maximum_control_frame_bytes)?,
            shared: Arc::clone(&shared),
            observed_render_sequence: 0,
            render_diagnostics: prepare_render_diagnostic_slots()?,
            render_diagnostic_head: 0,
            render_diagnostic_len: 0,
            protocol_reliable_pending: false,
        },
        session_error: FixedBytes::try_new(limits.maximum_diagnostic_bytes)?,
        plan: PlanState::new(owner, shared),
    })
}

#[cfg(test)]
mod live_peak_tests {
    //! #1257 gate 5: `validate_live_peak` charges each term of #1053 D8 against the right plan.

    use super::*;
    use crate::runtime::tests::{SESSION, limits};

    /// Every term gets its own bit, so a missing or swapped term moves every sum it is part of.
    struct Terms {
        graph: u64,
        capi_retained: u64,
        largest_named: u64,
        epoch_retained: u64,
        prepared_protocol: u64,
        capi_largest: u64,
    }

    fn epoch(base: PlanResourceReport, terms: &Terms) -> LiveEpochResources {
        LiveEpochResources {
            report: PlanResourceReport {
                graph_session_plus_plan_bytes: terms.graph,
                capi_retained_bytes: terms.capi_retained,
                largest_named_allocation_bytes: terms.largest_named,
                ..base
            },
            capi: CapiResources {
                active_retained: u64::MAX,
                epoch_retained: terms.epoch_retained,
                prepared_protocol_retained: terms.prepared_protocol,
                largest: terms.capi_largest,
            },
        }
    }

    fn caps(graph: u64, capi: u64, largest: u64) -> CompileLimits {
        CompileLimits {
            maximum_graph_session_plus_plan_bytes: graph,
            maximum_capi_retained_bytes: capi,
            maximum_named_allocation_bytes: largest,
            ..limits()
        }
    }

    fn verdict(
        current: LiveEpochResources,
        pending: Option<LiveEpochResources>,
        models: CompiledModelAdmission,
        limits: CompileLimits,
    ) -> Result<(), String> {
        validate_live_peak(current, pending, models, limits)
            .map_err(|failure| String::from_utf8(failure.diagnostics).expect("UTF-8"))
    }

    /// Accepts with `cap` at `peak` and refuses with it one byte below, with `code`.
    fn assert_cap(label: &str, peak: u64, code: &str, check: impl Fn(u64) -> Result<(), String>) {
        assert_eq!(check(peak), Ok(()), "{label}: accepted at the cap");
        assert_eq!(
            check(peak - 1),
            Err(format!("{code}\t$\n")),
            "{label}: refused one byte below"
        );
    }

    #[test]
    fn the_live_admission_accepts_each_cap_and_refuses_one_byte_below() {
        let base = compile_children(SESSION, limits())
            .unwrap_or_else(|_| panic!("fixture compiles"))
            .plan
            .resources();
        let current_terms = Terms {
            graph: 1 << 0,
            capi_retained: 1 << 1,
            largest_named: 3,
            epoch_retained: 1 << 2,
            prepared_protocol: 1 << 3,
            capi_largest: 5,
        };
        let pending_terms = Terms {
            graph: 1 << 4,
            capi_retained: 1 << 5,
            largest_named: 7,
            epoch_retained: 1 << 6,
            prepared_protocol: 1 << 7,
            capi_largest: 9,
        };
        let models = CompiledModelAdmission {
            retained_bytes: 1 << 8,
            largest_allocation_bytes: 11,
        };
        let current = epoch(base, &current_terms);
        let pending = epoch(base, &pending_terms);
        let huge = u64::MAX;

        // Graph: both plans plus both compiled models.
        for (label, pending, peak) in [
            ("graph alone", None, 1 + (1 << 8)),
            ("graph pending", Some(pending), 1 + (1 << 4) + (1 << 8)),
        ] {
            assert_cap(label, peak, "graph.resource.limit", |cap| {
                verdict(current, pending, models, caps(cap, huge, huge))
            });
        }
        // capi: the newest row, the current epoch's own rows while a candidate waits, and the
        // newest epoch's prepared-protocol rows.
        for (label, pending, peak) in [
            ("capi alone", None, (1 << 1) + (1 << 3)),
            (
                "capi pending",
                Some(pending),
                (1 << 5) + (1 << 2) + (1 << 7),
            ),
        ] {
            assert_cap(label, peak, "capi.resource.limit", |cap| {
                verdict(current, pending, models, caps(huge, cap, huge))
            });
        }

        // Largest allocation: make each term in turn the strict maximum. The current epoch's capi
        // `largest` is a decoy while a candidate is pending: only the newest epoch's counts.
        let big = 1_000;
        for term in 0..4 {
            let mut current_terms = Terms { ..current_terms };
            let mut pending_terms = Terms { ..pending_terms };
            let mut models = models;
            match term {
                0 => current_terms.largest_named = big,
                1 => pending_terms.largest_named = big,
                2 => pending_terms.capi_largest = big,
                _ => models.largest_allocation_bytes = big,
            }
            current_terms.capi_largest = big * 2;
            let current = epoch(base, &current_terms);
            let pending = epoch(base, &pending_terms);
            assert_cap(
                &format!("largest pending, term {term}"),
                big,
                "capi.resource.limit",
                |cap| verdict(current, Some(pending), models, caps(huge, huge, cap)),
            );
        }
        for term in 0..3 {
            let mut current_terms = Terms { ..current_terms };
            let mut models = models;
            match term {
                0 => current_terms.largest_named = big,
                1 => current_terms.capi_largest = big,
                _ => models.largest_allocation_bytes = big,
            }
            let current = epoch(base, &current_terms);
            assert_cap(
                &format!("largest alone, term {term}"),
                big,
                "capi.resource.limit",
                |cap| verdict(current, None, models, caps(huge, huge, cap)),
            );
        }
    }
}
