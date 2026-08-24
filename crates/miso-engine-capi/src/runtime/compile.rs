//! capi's own resource projection, and the children `miso_engine_v2_compile_session` returns.

use super::*;

pub(crate) struct CompiledChildren {
    pub(crate) session: SessionState,
    pub(crate) session_error: FixedBytes,
    pub(crate) plan: PlanState,
}

pub(crate) struct PreparedRuntime {
    pub(crate) sources: SourceControlSet,
    pub(crate) plan: PreparedRenderPlan,
    pub(crate) resources: PlanResourceReport,
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

#[derive(Clone, Copy, Default)]
pub(crate) struct ParameterCatalogResources {
    pub(crate) retained: u64,
    pub(crate) largest: u64,
}

impl ParameterCatalogResources {
    fn add(&mut self, bytes: u64) -> Result<(), CompileFailure> {
        self.retained = self
            .retained
            .checked_add(bytes)
            .ok_or_else(|| failure("capi.resource.arithmetic"))?;
        self.largest = self.largest.max(bytes);
        Ok(())
    }
}

fn visit_effect_parameters(
    compiled: &CompiledSession,
    registry: &miso_engine_effect_contract::NativeEffectRegistry,
    mut visit: impl FnMut(
        &miso_engine_session::Track,
        miso_engine_protocol::ParameterRack,
        &miso_engine_session::Effect,
        &'static miso_engine_effect_contract::EffectDescriptorV1,
        &'static miso_engine_effect_contract::ParameterDescriptorV1,
        miso_engine_protocol::ParameterChannel,
    ) -> Result<(), CompileFailure>,
) -> Result<(), CompileFailure> {
    use miso_engine_effect_contract::ParameterChannelPolicy;
    for track in &compiled.normalized_model().tracks {
        for (rack, effects) in [
            (
                miso_engine_protocol::ParameterRack::Simd1,
                &track.simd1.effects,
            ),
            (
                miso_engine_protocol::ParameterRack::Dynamic,
                &track.dynamic.effects,
            ),
            (
                miso_engine_protocol::ParameterRack::Simd2,
                &track.simd2.effects,
            ),
        ] {
            for effect in effects {
                let miso_engine_session::EffectIdentity::Native { effect_id } = &effect.identity
                else {
                    return Err(failure("capi.parameter.catalog"));
                };
                let factory = registry
                    .get_ascii(effect_id.as_str())
                    .ok_or_else(|| failure("capi.parameter.catalog"))?;
                let descriptor = factory.descriptor();
                for parameter in descriptor.parameters {
                    match parameter.channel_policy {
                        ParameterChannelPolicy::Shared => visit(
                            track,
                            rack,
                            effect,
                            descriptor,
                            parameter,
                            miso_engine_protocol::ParameterChannel::Both,
                        )?,
                        ParameterChannelPolicy::PerLane => {
                            visit(
                                track,
                                rack,
                                effect,
                                descriptor,
                                parameter,
                                miso_engine_protocol::ParameterChannel::Left,
                            )?;
                            visit(
                                track,
                                rack,
                                effect,
                                descriptor,
                                parameter,
                                miso_engine_protocol::ParameterChannel::Right,
                            )?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn parameter_catalog_resources(
    compiled: &CompiledSession,
) -> Result<ParameterCatalogResources, CompileFailure> {
    let registry = miso_engine_effect_compiler::launch_native_effect_registry_v1()
        .map_err(|_| failure("capi.parameter.catalog"))?;
    let mut resources = ParameterCatalogResources::default();
    let mut descriptors = 0_usize;
    visit_effect_parameters(
        compiled,
        &registry,
        |track, _rack, effect, _effect_descriptor, parameter, _channel| {
            descriptors = descriptors
                .checked_add(1)
                .ok_or_else(|| failure("capi.resource.arithmetic"))?;
            resources.add(checked_layout::<miso_engine_protocol::NamedNudge>(5)?)?;
            resources.add(checked_layout::<miso_engine_protocol::EnumChoice>(
                parameter.enum_choices.len(),
            )?)?;
            for bytes in [
                track.id.as_str().len(),
                effect.id.as_str().len(),
                parameter.display_name.len(),
                parameter.display_unit.len(),
            ] {
                resources.add(checked_layout::<u8>(bytes)?)?;
            }
            for choice in parameter.enum_choices {
                resources.add(checked_layout::<u8>(choice.label.len())?)?;
            }
            Ok(())
        },
    )?;
    resources.add(checked_layout::<miso_engine_protocol::ParameterDescriptor>(
        descriptors,
    )?)?;
    resources.add(checked_layout::<miso_engine_protocol::ParameterStateRecord>(descriptors)?)?;
    Ok(resources)
}

fn protocol_parameter_unit(
    value: miso_engine_effect_contract::ParameterUnit,
) -> miso_engine_protocol::ParameterUnit {
    use miso_engine_effect_contract::ParameterUnit as Contract;
    match value {
        Contract::Db => miso_engine_protocol::ParameterUnit::Db,
        Contract::Hz => miso_engine_protocol::ParameterUnit::Hz,
        Contract::Milliseconds => miso_engine_protocol::ParameterUnit::Milliseconds,
        Contract::Samples => miso_engine_protocol::ParameterUnit::Samples,
        Contract::Linear => miso_engine_protocol::ParameterUnit::Linear,
        Contract::Ratio => miso_engine_protocol::ParameterUnit::Ratio,
    }
}

fn protocol_parameter_domain(
    value: miso_engine_effect_contract::ParameterDomain,
) -> miso_engine_protocol::ParameterDomain {
    use miso_engine_effect_contract::ParameterDomain as Contract;
    match value {
        Contract::Continuous => miso_engine_protocol::ParameterDomain::Continuous,
        Contract::Boolean => miso_engine_protocol::ParameterDomain::Boolean,
        Contract::Enumeration => miso_engine_protocol::ParameterDomain::Enumeration,
    }
}

fn protocol_parameter_mapping(
    value: miso_engine_effect_contract::ParameterMapping,
) -> miso_engine_protocol::ParameterMapping {
    use miso_engine_effect_contract::ParameterMapping as Contract;
    match value {
        Contract::Linear => miso_engine_protocol::ParameterMapping::Linear,
        Contract::Logarithmic => miso_engine_protocol::ParameterMapping::Logarithmic,
        Contract::Exponential => miso_engine_protocol::ParameterMapping::Exponential,
        Contract::Stepped => miso_engine_protocol::ParameterMapping::Stepped,
    }
}

fn protocol_automation_rate(
    value: miso_engine_effect_contract::AutomationRate,
) -> miso_engine_protocol::ParameterAutomationRate {
    use miso_engine_effect_contract::AutomationRate as Contract;
    match value {
        Contract::Sample => miso_engine_protocol::ParameterAutomationRate::Sample,
        Contract::Block => miso_engine_protocol::ParameterAutomationRate::Block,
        Contract::None => miso_engine_protocol::ParameterAutomationRate::None,
    }
}

pub(crate) fn build_parameter_catalog(
    compiled: &CompiledSession,
) -> Result<MockParameterCatalog, CompileFailure> {
    let registry = miso_engine_effect_compiler::launch_native_effect_registry_v1()
        .map_err(|_| failure("capi.parameter.catalog"))?;
    let mut descriptor_count = 0_usize;
    visit_effect_parameters(compiled, &registry, |_, _, _, _, _, _| {
        descriptor_count = descriptor_count
            .checked_add(1)
            .ok_or_else(|| failure("capi.resource.arithmetic"))?;
        Ok(())
    })?;
    let mut metadata = Vec::new();
    metadata
        .try_reserve_exact(descriptor_count)
        .map_err(|_| failure("capi.resource.allocation"))?;
    let mut state = Vec::new();
    state
        .try_reserve_exact(descriptor_count)
        .map_err(|_| failure("capi.resource.allocation"))?;
    visit_effect_parameters(
        compiled,
        &registry,
        |track, rack, effect, effect_descriptor, parameter, channel| {
            let handle = u32::try_from(metadata.len().saturating_add(1))
                .map_err(|_| failure("capi.parameter.catalog"))?;
            let session_channel = match channel {
                miso_engine_protocol::ParameterChannel::Left => {
                    miso_engine_session::ParameterChannel::Left
                }
                miso_engine_protocol::ParameterChannel::Right => {
                    miso_engine_session::ParameterChannel::Right
                }
                miso_engine_protocol::ParameterChannel::Both => {
                    miso_engine_session::ParameterChannel::Both
                }
            };
            let current = effect
                .params
                .iter()
                .find(|value| {
                    value.parameter_id == parameter.id.0 && value.channel == session_channel
                })
                .map_or(parameter.default_value, |value| value.value);
            let ladder = registry
                .nudge_ladder(effect_descriptor.id, parameter.id)
                .ok_or_else(|| failure("capi.parameter.catalog"))?;
            let mut named_nudges = Vec::new();
            named_nudges
                .try_reserve_exact(5)
                .map_err(|_| failure("capi.resource.allocation"))?;
            for size in [
                miso_engine_protocol::NudgeSize::Xs,
                miso_engine_protocol::NudgeSize::Sm,
                miso_engine_protocol::NudgeSize::Md,
                miso_engine_protocol::NudgeSize::Lg,
                miso_engine_protocol::NudgeSize::Xl,
            ] {
                named_nudges.push(miso_engine_protocol::NamedNudge {
                    size,
                    normalized_step: ladder.step(size),
                    decrement: 0.0,
                    increment: 0.0,
                });
            }
            let mut enum_choices = Vec::new();
            enum_choices
                .try_reserve_exact(parameter.enum_choices.len())
                .map_err(|_| failure("capi.resource.allocation"))?;
            enum_choices.extend(parameter.enum_choices.iter().map(|choice| {
                miso_engine_protocol::EnumChoice {
                    value: choice.value,
                    label: choice.label.to_owned(),
                }
            }));
            metadata.push(miso_engine_protocol::ParameterDescriptor {
                handle,
                track_id: track.id.as_str().to_owned(),
                rack,
                effect_id: effect.id.as_str().to_owned(),
                parameter_id: parameter.id.0,
                channel,
                value_kind: miso_engine_protocol::ParameterValueKind::F32,
                unit: protocol_parameter_unit(parameter.unit),
                domain: protocol_parameter_domain(parameter.domain),
                minimum: parameter.minimum,
                maximum: parameter.maximum,
                default: parameter.default_value,
                mapping: protocol_parameter_mapping(parameter.mapping),
                automation_rate: protocol_automation_rate(parameter.automation_rate),
                smoothing_samples: parameter.smoothing_samples,
                flags: u32::from(parameter.readable) | (u32::from(parameter.automatable) << 1),
                display_name: Some(parameter.display_name.to_owned()),
                display_unit: Some(parameter.display_unit.to_owned()),
                enum_choices,
                named_nudges,
            });
            state.push(miso_engine_protocol::ParameterStateRecord {
                handle,
                flags: 1,
                value: current,
            });
            Ok(())
        },
    )?;
    MockParameterCatalog::try_new(
        metadata,
        miso_engine_protocol::ParameterStatePage {
            observed_sample: 0,
            records: state,
        },
    )
    .map_err(|_| failure("capi.parameter.catalog"))
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
    Owned(miso_engine_protocol::Diagnostic),
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
    canonical_bytes: usize,
    source_count: usize,
    source_id_bytes: usize,
    quantum_frames: usize,
    parameter_catalog: ParameterCatalogResources,
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
    let epoch_rows = [
        checked_layout::<u8>(canonical_bytes)?,
        miso_engine_host_core::control_table_bytes(source_count)
            .ok_or_else(|| failure("capi.resource.arithmetic"))?,
        miso_engine_host_core::source_id_arena_bytes(source_id_bytes)
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
        // ProtocolController and MockProvider each retain their own complete telemetry config.
        checked_layout::<u32>(maximum_configuration_items)?,
        checked_layout::<miso_engine_protocol::CounterId>(maximum_configuration_items)?,
        checked_layout::<miso_engine_protocol::CounterValue>(maximum_configuration_items)?,
        checked_layout::<u32>(maximum_configuration_items)?,
        checked_layout::<miso_engine_protocol::CounterId>(maximum_configuration_items)?,
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
        checked_layout::<miso_engine_protocol::PreparedStructuralCommand>(1)?,
    ];
    let prepared_protocol_aggregate_rows = [replay.retained_payload_bytes];
    let epoch_retained = checked_sum(&epoch_rows)?
        .checked_add(parameter_catalog.retained)
        .ok_or_else(|| failure("capi.resource.arithmetic"))?;
    let active_retained = checked_sum(&fixed_allocation_rows)?
        .checked_add(checked_sum(&fixed_aggregate_rows)?)
        .and_then(|value| value.checked_add(epoch_retained))
        .ok_or_else(|| failure("capi.resource.arithmetic"))?;
    let prepared_protocol_retained = checked_sum(&prepared_protocol_allocation_rows)?
        .checked_add(checked_sum(&prepared_protocol_aggregate_rows)?)
        .ok_or_else(|| failure("capi.resource.arithmetic"))?;
    let largest = epoch_rows
        .into_iter()
        .chain(fixed_allocation_rows)
        .chain(prepared_protocol_allocation_rows)
        .chain([
            queue.largest_allocation_bytes,
            replay.largest_allocation_bytes,
            exchange.largest_allocation_bytes,
            parameter_catalog.largest,
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

pub(crate) fn compiled_capi_resources(
    compiled: &CompiledSession,
    limits: CompileLimits,
) -> Result<(CapiResources, usize), CompileFailure> {
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
    let parameter_catalog = parameter_catalog_resources(compiled)?;
    Ok((
        capi_resources(
            limits,
            compiled.canonical_toml().len(),
            compiled.source_count(),
            source_id_bytes,
            compiled.quantum().0 as usize,
            parameter_catalog,
        )?,
        source_id_bytes,
    ))
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

pub(crate) fn all_limits_nonzero(limits: CompileLimits) -> bool {
    limits.source_ring_frames != 0
        && limits.maximum_automation_spans_per_block != 0
        && [
            limits.maximum_toml_bytes,
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
        && limits.reserved == [0; 4]
        && all_limits_nonzero(limits)
}

/// Translate the frozen C ABI limits into the facade's caps, field for field.
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
/// The shared pipeline is `miso-engine-host-core`; capi adds only what is capi's: its own retained
/// rows (protocol queues, replay storage, handle structs), and the ABI report shape.
pub(crate) fn prepare_runtime(
    compiled: &CompiledSession,
    limits: CompileLimits,
) -> Result<PreparedRuntime, CompileFailure> {
    let caps = prepare_caps(limits);
    // Shape first, so an unsupported rate or a bad ring is reported before capi spends the
    // pre-flight projection on a session it will refuse anyway.
    caps.validate_shape(compiled).map_err(prepare_failure)?;
    let (capi, _) = compiled_capi_resources(compiled, limits)?;
    if capi.active_retained > limits.maximum_capi_retained_bytes
        || capi.largest > limits.maximum_named_allocation_bytes
    {
        return Err(failure("capi.resource.limit"));
    }
    let prepared = prepare_host_runtime(compiled, &caps).map_err(prepare_failure)?;
    let host = prepared.report;
    let largest_named = host.largest_engine_allocation_bytes.max(capi.largest);
    let (tail_kind, tail_samples) = match host.output_tail {
        TailSamples::Finite(samples) => (TAIL_FINITE, samples),
        TailSamples::Infinite => (TAIL_INFINITE, 0),
    };
    Ok(PreparedRuntime {
        sources: prepared.sources,
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
    })
}

pub(crate) fn compile_children(
    toml: &str,
    limits: CompileLimits,
) -> Result<CompiledChildren, CompileFailure> {
    // The C ABI needs the transactional `SessionStore` for the control protocol, so it parses and
    // caps through the facade and builds the store itself; the facade never sees the protocol.
    let model = parse_host_session(toml).map_err(prepare_failure)?;
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
    let retained_capacity = ControllerRetainedCapacity {
        meter_handles: maximum_tlvs as usize,
        counter_ids: maximum_tlvs as usize,
    };
    let parameter_catalog = build_parameter_catalog(store.compiled())?;
    let provider = MockProvider::try_with_retained_capacity_and_parameters(
        retained_capacity,
        parameter_catalog,
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

    let PreparedRuntime {
        sources,
        plan,
        resources,
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
    });
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
            providers: ProviderEpoch::current(sources),
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
