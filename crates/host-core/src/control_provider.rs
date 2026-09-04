//! Session-backed implementation of the transport-neutral control-provider boundary.

use std::sync::Arc;

use effect_compiler::launch_native_effect_registry;
use effect_contract::{
    AutomationRate, ParameterChannelPolicy, ParameterDomain, ParameterMapping, ParameterUnit,
};
use protocol::{
    ControlProvider, ControllerRetainedCapacity, CounterId, CounterSnapshot, CounterValue,
    CountersRequest, Diagnostic, DiagnosticSeverity, DiagnosticsPage, DiagnosticsRequest,
    EnumChoice, ParameterAutomationRate, ParameterChannel, ParameterDescriptor,
    ParameterDomain as ProtocolDomain, ParameterHandle, ParameterMapping as ProtocolMapping,
    ParameterMetadataPage, ParameterMetadataRequest, ParameterProviderError, ParameterRack,
    ParameterStatePage, ParameterStateRecord, ParameterStateRequest, ParameterUnit as ProtocolUnit,
    ParameterValueKind, SampleTime, TelemetryConfiguration, TelemetryCounters, TransportSetRequest,
    TransportSnapshot, TransportState,
};
use session::{CompiledSession, EffectIdentity, ParameterChannel as SessionChannel, RackName};

/// Read-only, thread-safe projection of the active render plan's next absolute sample.
pub trait PlanSampleSource: Send + Sync {
    /// Sample at which the next contiguous render block begins.
    fn next_absolute_sample(&self) -> u64;
}

/// A session catalog could not be prepared within its typed or allocation contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionControlProviderError;

/// Address-free retained-allocation projection for a session-backed provider.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionControlProviderResources {
    /// Per-session descriptor/state allocations replaced at a structural commit.
    pub catalog_retained_bytes: u64,
    /// Fixed diagnostic projection allocations retained for the endpoint lifetime.
    pub fixed_retained_bytes: u64,
    /// Largest single allocation in either group.
    pub largest_allocation_bytes: u64,
}

/// Fully allocated replacement catalog prepared before a structural commit is published.
pub struct PreparedSessionControlCatalog {
    parameter_metadata: Vec<ParameterDescriptor>,
    parameter_state: Vec<ParameterStateRecord>,
}

/// Production control provider whose catalog and state are derived from a compiled session.
pub struct SessionControlProvider {
    sample_source: Arc<dyn PlanSampleSource>,
    parameter_metadata: Vec<ParameterDescriptor>,
    parameter_state: Vec<ParameterStateRecord>,
    counter_snapshot: Vec<CounterValue>,
    diagnostics: Vec<Diagnostic>,
    diagnostic_live: Vec<bool>,
    transport_state: TransportState,
    transport_position: SampleTime,
    telemetry_configuration: TelemetryConfiguration,
}

impl SessionControlProvider {
    /// Project the allocations made by [`Self::try_new`] from the same descriptor authority.
    pub fn resource_report(
        compiled: &CompiledSession,
        diagnostic_capacity: usize,
    ) -> Result<SessionControlProviderResources, SessionControlProviderError> {
        let registry = launch_native_effect_registry().map_err(|_| SessionControlProviderError)?;
        let mut descriptor_count = 0_usize;
        let mut enum_count = 0_usize;
        let mut string_bytes = 0_usize;
        let mut largest_string = 0_u64;
        let mut largest_enum = 0_u64;
        for track in &compiled.normalized_model().tracks {
            for effects in [
                &track.simd1.effects,
                &track.dynamic.effects,
                &track.simd2.effects,
            ] {
                for effect in effects {
                    let EffectIdentity::Native { effect_id } = &effect.identity else {
                        continue;
                    };
                    let descriptor = registry
                        .get_ascii(effect_id.as_str())
                        .ok_or(SessionControlProviderError)?
                        .descriptor();
                    for parameter in descriptor.parameters {
                        let channels = match parameter.channel_policy {
                            ParameterChannelPolicy::Shared => 1,
                            ParameterChannelPolicy::PerLane => 2,
                        };
                        descriptor_count = descriptor_count
                            .checked_add(channels)
                            .ok_or(SessionControlProviderError)?;
                        enum_count = enum_count
                            .checked_add(
                                parameter
                                    .enum_choices
                                    .len()
                                    .checked_mul(channels)
                                    .ok_or(SessionControlProviderError)?,
                            )
                            .ok_or(SessionControlProviderError)?;
                        largest_enum =
                            largest_enum.max(bytes::<EnumChoice>(parameter.enum_choices.len())?);
                        for value in [
                            track.id.as_str(),
                            effect.id.as_str(),
                            parameter.display_name,
                            parameter.display_unit,
                        ] {
                            let bytes = value
                                .len()
                                .checked_mul(channels)
                                .ok_or(SessionControlProviderError)?;
                            string_bytes = string_bytes
                                .checked_add(bytes)
                                .ok_or(SessionControlProviderError)?;
                            largest_string = largest_string.max(
                                u64::try_from(value.len())
                                    .map_err(|_| SessionControlProviderError)?,
                            );
                        }
                        for choice in parameter.enum_choices {
                            let bytes = choice
                                .label
                                .len()
                                .checked_mul(channels)
                                .ok_or(SessionControlProviderError)?;
                            string_bytes = string_bytes
                                .checked_add(bytes)
                                .ok_or(SessionControlProviderError)?;
                            largest_string = largest_string.max(
                                u64::try_from(choice.label.len())
                                    .map_err(|_| SessionControlProviderError)?,
                            );
                        }
                    }
                }
            }
        }
        let metadata_bytes = bytes::<ParameterDescriptor>(descriptor_count)?;
        let state_bytes = bytes::<ParameterStateRecord>(descriptor_count)?;
        let enum_bytes = bytes::<EnumChoice>(enum_count)?;
        let catalog_retained_bytes = metadata_bytes
            .checked_add(state_bytes)
            .and_then(|value| value.checked_add(enum_bytes))
            .and_then(|value| {
                u64::try_from(string_bytes)
                    .ok()
                    .and_then(|bytes| value.checked_add(bytes))
            })
            .ok_or(SessionControlProviderError)?;
        let diagnostics_bytes = bytes::<Diagnostic>(diagnostic_capacity)?;
        let diagnostic_live_bytes = bytes::<bool>(diagnostic_capacity)?;
        let diagnostic_code_bytes = "capi.render.activity"
            .len()
            .checked_mul(diagnostic_capacity)
            .ok_or(SessionControlProviderError)?;
        let diagnostic_code_bytes =
            u64::try_from(diagnostic_code_bytes).map_err(|_| SessionControlProviderError)?;
        let fixed_retained_bytes = diagnostics_bytes
            .checked_add(diagnostic_live_bytes)
            .and_then(|value| value.checked_add(diagnostic_code_bytes))
            .ok_or(SessionControlProviderError)?;
        Ok(SessionControlProviderResources {
            catalog_retained_bytes,
            fixed_retained_bytes,
            largest_allocation_bytes: metadata_bytes
                .max(state_bytes)
                .max(largest_enum)
                .max(largest_string)
                .max(diagnostics_bytes)
                .max(diagnostic_live_bytes)
                .max(
                    diagnostic_code_bytes
                        / u64::try_from(diagnostic_capacity.max(1))
                            .map_err(|_| SessionControlProviderError)?,
                ),
        })
    }

    /// Build the complete revision-scoped effect catalog before the endpoint is published.
    pub fn try_new(
        compiled: &CompiledSession,
        sample_source: Arc<dyn PlanSampleSource>,
        retained: ControllerRetainedCapacity,
        diagnostic_capacity: usize,
    ) -> Result<Self, SessionControlProviderError> {
        let mut provider = Self {
            sample_source,
            parameter_metadata: Vec::new(),
            parameter_state: Vec::new(),
            counter_snapshot: Vec::new(),
            diagnostics: Vec::new(),
            diagnostic_live: Vec::new(),
            transport_state: TransportState::Stopped,
            transport_position: SampleTime(0),
            telemetry_configuration: retained_telemetry(retained)?,
        };
        provider
            .counter_snapshot
            .try_reserve_exact(retained.counter_ids)
            .map_err(|_| SessionControlProviderError)?;
        provider
            .diagnostics
            .try_reserve_exact(diagnostic_capacity)
            .map_err(|_| SessionControlProviderError)?;
        provider
            .diagnostic_live
            .try_reserve_exact(diagnostic_capacity)
            .map_err(|_| SessionControlProviderError)?;
        for _ in 0..diagnostic_capacity {
            provider.diagnostics.push(Diagnostic {
                code: try_string("capi.render.activity")?,
                severity: DiagnosticSeverity::Info,
                path: Vec::new(),
                detail: None,
                operation_index: None,
                sample_time: None,
                provider_sequence: None,
            });
            provider.diagnostic_live.push(false);
        }
        provider.replace_session_catalog(Self::prepare_session(compiled)?);
        Ok(provider)
    }

    /// Allocate a complete candidate catalog without mutating the live provider.
    pub fn prepare_session(
        compiled: &CompiledSession,
    ) -> Result<PreparedSessionControlCatalog, SessionControlProviderError> {
        let (metadata, state) = build_parameter_catalog(compiled)?;
        Ok(PreparedSessionControlCatalog {
            parameter_metadata: metadata,
            parameter_state: state,
        })
    }

    /// Publish one fully prepared revision-scoped catalog without allocation or failure.
    pub fn replace_session_catalog(&mut self, catalog: PreparedSessionControlCatalog) {
        self.parameter_metadata = catalog.parameter_metadata;
        self.parameter_state = catalog.parameter_state;
    }

    /// Record the two queue-owned telemetry counters exposed by the provider boundary.
    pub fn set_telemetry_counters(&mut self, counters: TelemetryCounters) {
        set_counter(
            &mut self.counter_snapshot,
            CounterId::TelemetryCoalesced,
            counters.telemetry_coalesced,
        );
        set_counter(
            &mut self.counter_snapshot,
            CounterId::TelemetryDropped,
            counters.telemetry_dropped,
        );
    }

    /// Update one preallocated C-ABI render-diagnostic projection.
    pub fn set_render_diagnostic(&mut self, slot: usize, sample: u64, sequence: u64, live: bool) {
        let Some(diagnostic) = self.diagnostics.get_mut(slot) else {
            return;
        };
        diagnostic.sample_time = live.then_some(sample);
        diagnostic.provider_sequence = live.then_some(sequence);
        self.diagnostic_live[slot] = live;
    }

    /// Capacities used by the C ABI's retained-resource qualification.
    #[must_use]
    pub fn retained_capacities(&self) -> (ControllerRetainedCapacity, usize) {
        (
            ControllerRetainedCapacity {
                meter_handles: self.telemetry_configuration.meter_handles.capacity(),
                counter_ids: self.telemetry_configuration.counter_ids.capacity(),
            },
            self.counter_snapshot.capacity(),
        )
    }
}

impl ControlProvider for SessionControlProvider {
    fn current_sample(&mut self) -> SampleTime {
        SampleTime(self.sample_source.next_absolute_sample())
    }

    fn parameter_metadata(
        &mut self,
        request: ParameterMetadataRequest,
    ) -> Result<ParameterMetadataPage, ParameterProviderError> {
        let descriptors = self
            .parameter_metadata
            .iter()
            .filter(|descriptor| descriptor.handle > request.after_handle)
            .take(usize::from(request.limit))
            .cloned()
            .collect::<Vec<_>>();
        let last_handle = descriptors
            .last()
            .map_or(request.after_handle, |descriptor| descriptor.handle);
        Ok(ParameterMetadataPage {
            last_handle,
            eof: self
                .parameter_metadata
                .iter()
                .all(|descriptor| descriptor.handle <= last_handle),
            descriptors,
        })
    }

    fn parameter_state(
        &mut self,
        request: &ParameterStateRequest,
    ) -> Result<ParameterStatePage, ParameterProviderError> {
        let mut records = Vec::with_capacity(request.handles.len());
        for handle in &request.handles {
            let record = self
                .parameter_state
                .iter()
                .find(|record| record.handle == *handle)
                .ok_or(ParameterProviderError::NotFound)?;
            records.push(*record);
        }
        Ok(ParameterStatePage {
            observed_sample: self.sample_source.next_absolute_sample(),
            records,
        })
    }

    fn parameter_descriptor(
        &mut self,
        handle: ParameterHandle,
    ) -> Result<&ParameterDescriptor, ParameterProviderError> {
        self.parameter_metadata
            .iter()
            .find(|descriptor| descriptor.handle == handle.0)
            .ok_or(ParameterProviderError::NotFound)
    }

    fn counters(
        &mut self,
        request: &CountersRequest,
    ) -> Result<CounterSnapshot, ParameterProviderError> {
        let values = if request.all {
            self.counter_snapshot.clone()
        } else {
            let mut values = Vec::with_capacity(request.ids.len());
            for id in &request.ids {
                let value = self
                    .counter_snapshot
                    .iter()
                    .find(|value| value.id as u32 == *id)
                    .ok_or(ParameterProviderError::NotFound)?;
                values.push(*value);
            }
            values
        };
        Ok(CounterSnapshot {
            observed_sample: self.current_sample(),
            values,
        })
    }

    fn record_canceled_automation(&mut self, records: u64) {
        let current = self
            .counter_snapshot
            .iter()
            .find(|value| value.id == CounterId::CanceledAutomation)
            .map_or(0, |value| value.value);
        set_counter(
            &mut self.counter_snapshot,
            CounterId::CanceledAutomation,
            current.saturating_add(records),
        );
    }

    fn diagnostics(
        &mut self,
        request: DiagnosticsRequest,
    ) -> Result<DiagnosticsPage, ParameterProviderError> {
        let oldest = self
            .diagnostics
            .iter()
            .zip(&self.diagnostic_live)
            .filter_map(|(diagnostic, live)| live.then_some(diagnostic))
            .filter_map(|diagnostic| diagnostic.provider_sequence)
            .min();
        if request.after_sequence != 0
            && oldest.is_some_and(|sequence| request.after_sequence.saturating_add(1) < sequence)
        {
            return Err(ParameterProviderError::ReplayExpired);
        }
        let available = self
            .diagnostics
            .iter()
            .zip(&self.diagnostic_live)
            .filter_map(|(diagnostic, live)| live.then_some(diagnostic))
            .filter(|diagnostic| {
                diagnostic
                    .provider_sequence
                    .is_some_and(|sequence| sequence > request.after_sequence)
                    && diagnostic.severity as u8 >= request.minimum_severity as u8
            });
        let mut diagnostics = available.cloned().collect::<Vec<_>>();
        diagnostics.sort_by_key(|diagnostic| diagnostic.provider_sequence);
        let eof = diagnostics.len() <= usize::from(request.limit);
        diagnostics.truncate(usize::from(request.limit));
        let last_sequence = diagnostics
            .last()
            .and_then(|diagnostic| diagnostic.provider_sequence)
            .unwrap_or(request.after_sequence);
        Ok(DiagnosticsPage {
            last_sequence,
            eof,
            diagnostics,
        })
    }

    fn transport_get(&mut self) -> TransportSnapshot {
        TransportSnapshot {
            state: self.transport_state,
            position: self.transport_position,
            effective_sample: self.current_sample(),
        }
    }

    fn transport_set(&mut self, request: TransportSetRequest) -> TransportSnapshot {
        self.transport_state = request.state;
        if let Some(position) = request.position {
            self.transport_position = position;
        }
        self.transport_get()
    }

    fn telemetry_configure(
        &mut self,
        configuration: TelemetryConfiguration,
    ) -> TelemetryConfiguration {
        self.telemetry_configuration.meter_handles.clear();
        self.telemetry_configuration
            .meter_handles
            .extend_from_slice(&configuration.meter_handles);
        self.telemetry_configuration.counter_ids.clear();
        self.telemetry_configuration
            .counter_ids
            .extend_from_slice(&configuration.counter_ids);
        self.telemetry_configuration.meter_period_blocks = configuration.meter_period_blocks;
        self.telemetry_configuration.counter_period_blocks = configuration.counter_period_blocks;
        self.telemetry_configuration.diagnostics_enabled = configuration.diagnostics_enabled;
        self.telemetry_configuration.minimum_diagnostic_severity =
            configuration.minimum_diagnostic_severity;
        configuration
    }
}

fn build_parameter_catalog(
    compiled: &CompiledSession,
) -> Result<(Vec<ParameterDescriptor>, Vec<ParameterStateRecord>), SessionControlProviderError> {
    let registry = launch_native_effect_registry().map_err(|_| SessionControlProviderError)?;
    let count = compiled
        .normalized_model()
        .tracks
        .iter()
        .flat_map(|track| [&track.simd1, &track.dynamic, &track.simd2])
        .flat_map(|rack| &rack.effects)
        .try_fold(0_usize, |total, effect| {
            let EffectIdentity::Native { effect_id } = &effect.identity else {
                return Ok(total);
            };
            let descriptor = registry
                .get_ascii(effect_id.as_str())
                .ok_or(SessionControlProviderError)?
                .descriptor();
            descriptor
                .parameters
                .iter()
                .try_fold(total, |total, parameter| {
                    total
                        .checked_add(match parameter.channel_policy {
                            ParameterChannelPolicy::Shared => 1,
                            ParameterChannelPolicy::PerLane => 2,
                        })
                        .ok_or(SessionControlProviderError)
                })
        })?;
    if count > u32::MAX as usize {
        return Err(SessionControlProviderError);
    }
    let mut metadata = Vec::new();
    let mut state = Vec::new();
    metadata
        .try_reserve_exact(count)
        .map_err(|_| SessionControlProviderError)?;
    state
        .try_reserve_exact(count)
        .map_err(|_| SessionControlProviderError)?;
    let mut handle = 1_u32;
    for track in &compiled.normalized_model().tracks {
        for (rack, effects) in [
            (RackName::Simd1, &track.simd1.effects),
            (RackName::Dynamic, &track.dynamic.effects),
            (RackName::Simd2, &track.simd2.effects),
        ] {
            for effect in effects {
                let EffectIdentity::Native { effect_id } = &effect.identity else {
                    continue;
                };
                let descriptor = registry
                    .get_ascii(effect_id.as_str())
                    .ok_or(SessionControlProviderError)?
                    .descriptor();
                for parameter in descriptor.parameters {
                    let channels: &[ParameterChannel] = match parameter.channel_policy {
                        ParameterChannelPolicy::Shared => &[ParameterChannel::Both],
                        ParameterChannelPolicy::PerLane => {
                            &[ParameterChannel::Left, ParameterChannel::Right]
                        }
                    };
                    for channel in channels {
                        let matching = effect
                            .params
                            .iter()
                            .filter(|item| item.parameter_id == parameter.id.0);
                        let requested = matching
                            .clone()
                            .find(|item| ParameterChannel::from(item.channel) == *channel)
                            .or_else(|| {
                                (parameter.channel_policy == ParameterChannelPolicy::PerLane)
                                    .then(|| {
                                        matching
                                            .clone()
                                            .find(|item| item.channel == SessionChannel::Both)
                                    })
                                    .flatten()
                            });
                        let value = effect_contract::normalize_zero(
                            requested.map_or(parameter.default_value, |item| item.value),
                        );
                        let flags = u32::from(parameter.readable)
                            | (u32::from(parameter.automatable) << 1)
                            | (u32::from(
                                parameter.channel_policy == ParameterChannelPolicy::PerLane,
                            ) << 2);
                        metadata.push(ParameterDescriptor {
                            handle,
                            track_id: try_string(track.id.as_str())?,
                            rack: ParameterRack::from(rack),
                            effect_id: try_string(effect.id.as_str())?,
                            parameter_id: parameter.id.0,
                            channel: *channel,
                            value_kind: ParameterValueKind::F32,
                            unit: protocol_unit(parameter.unit),
                            domain: protocol_domain(parameter.domain),
                            minimum: parameter.minimum,
                            maximum: parameter.maximum,
                            default: parameter.default_value,
                            mapping: protocol_mapping(parameter.mapping),
                            automation_rate: protocol_rate(parameter.automation_rate),
                            smoothing_samples: parameter.smoothing_samples,
                            flags,
                            display_name: Some(try_string(parameter.display_name)?),
                            display_unit: Some(try_string(parameter.display_unit)?),
                            enum_choices: parameter
                                .enum_choices
                                .iter()
                                .map(|choice| {
                                    Ok(EnumChoice {
                                        value: choice.value,
                                        label: try_string(choice.label)?,
                                    })
                                })
                                .collect::<Result<Vec<_>, SessionControlProviderError>>()?,
                        });
                        state.push(ParameterStateRecord {
                            handle,
                            flags: 1,
                            value,
                        });
                        if metadata.len() < count {
                            handle = handle.checked_add(1).ok_or(SessionControlProviderError)?;
                        }
                    }
                }
            }
        }
    }
    Ok((metadata, state))
}

fn retained_telemetry(
    retained: ControllerRetainedCapacity,
) -> Result<TelemetryConfiguration, SessionControlProviderError> {
    let mut meter_handles = Vec::new();
    meter_handles
        .try_reserve_exact(retained.meter_handles)
        .map_err(|_| SessionControlProviderError)?;
    let mut counter_ids = Vec::new();
    counter_ids
        .try_reserve_exact(retained.counter_ids)
        .map_err(|_| SessionControlProviderError)?;
    Ok(TelemetryConfiguration {
        meter_handles,
        meter_period_blocks: 0,
        counter_ids,
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: DiagnosticSeverity::Info,
    })
}

fn try_string(value: &str) -> Result<String, SessionControlProviderError> {
    let mut output = String::new();
    output
        .try_reserve_exact(value.len())
        .map_err(|_| SessionControlProviderError)?;
    output.push_str(value);
    Ok(output)
}

fn set_counter(values: &mut Vec<CounterValue>, id: CounterId, value: u64) {
    if let Some(existing) = values.iter_mut().find(|existing| existing.id == id) {
        existing.value = value;
        return;
    }
    let index = values
        .iter()
        .position(|existing| existing.id > id)
        .unwrap_or(values.len());
    values.insert(index, CounterValue { id, value });
}

fn bytes<T>(count: usize) -> Result<u64, SessionControlProviderError> {
    let bytes = core::mem::size_of::<T>()
        .checked_mul(count)
        .ok_or(SessionControlProviderError)?;
    u64::try_from(bytes).map_err(|_| SessionControlProviderError)
}

const fn protocol_unit(value: ParameterUnit) -> ProtocolUnit {
    match value {
        ParameterUnit::Db => ProtocolUnit::Db,
        ParameterUnit::Hz => ProtocolUnit::Hz,
        ParameterUnit::Milliseconds => ProtocolUnit::Milliseconds,
        ParameterUnit::Samples => ProtocolUnit::Samples,
        ParameterUnit::Linear => ProtocolUnit::Linear,
        ParameterUnit::Ratio => ProtocolUnit::Ratio,
    }
}

const fn protocol_domain(value: ParameterDomain) -> ProtocolDomain {
    match value {
        ParameterDomain::Continuous => ProtocolDomain::Continuous,
        ParameterDomain::Boolean => ProtocolDomain::Boolean,
        ParameterDomain::Enumeration => ProtocolDomain::Enumeration,
    }
}

const fn protocol_mapping(value: ParameterMapping) -> ProtocolMapping {
    match value {
        ParameterMapping::Linear => ProtocolMapping::Linear,
        ParameterMapping::Logarithmic => ProtocolMapping::Logarithmic,
        ParameterMapping::Exponential => ProtocolMapping::Exponential,
        ParameterMapping::Stepped => ProtocolMapping::Stepped,
    }
}

const fn protocol_rate(value: AutomationRate) -> ParameterAutomationRate {
    match value {
        AutomationRate::Sample => ParameterAutomationRate::Sample,
        AutomationRate::Block => ParameterAutomationRate::Block,
        AutomationRate::None => ParameterAutomationRate::None,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;

    struct Clock(AtomicU64);

    impl PlanSampleSource for Clock {
        fn next_absolute_sample(&self) -> u64 {
            self.0.load(Ordering::Acquire)
        }
    }

    fn provider() -> (SessionControlProvider, Arc<Clock>) {
        let model = session::parse_session_json(include_str!(
            "../../../fixtures/session/v1/parametric-eq-nine-track.json"
        ))
        .expect("session fixture");
        let compiled = session::compile_session(
            &model,
            session::CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled fixture");
        let clock = Arc::new(Clock(AtomicU64::new(0)));
        let sample_source: Arc<dyn PlanSampleSource> = Arc::clone(&clock) as Arc<_>;
        let provider = SessionControlProvider::try_new(
            &compiled,
            sample_source,
            ControllerRetainedCapacity {
                meter_handles: 4,
                counter_ids: 4,
            },
            2,
        )
        .expect("provider");
        (provider, clock)
    }

    #[test]
    fn catalog_state_clock_transport_counters_and_diagnostics_are_session_backed() {
        let (mut provider, clock) = provider();
        let page = provider
            .parameter_metadata(ParameterMetadataRequest {
                after_handle: 0,
                limit: u16::MAX,
            })
            .expect("metadata");
        assert_eq!(page.descriptors.len(), 9 * 24 * 2);
        assert!(page.eof);
        let first = &page.descriptors[0];
        assert_eq!(first.handle, 1);
        assert_eq!(first.track_id, "eq0");
        assert_eq!(first.rack, ParameterRack::Simd1);
        assert_eq!(first.effect_id, "eq");
        assert_eq!(first.parameter_id, 1);
        assert_eq!(first.channel, ParameterChannel::Left);
        assert_eq!(first.flags, 5);
        let state = provider
            .parameter_state(&ParameterStateRequest {
                handles: vec![1, 2, 5],
            })
            .expect("state");
        assert_eq!(state.records[0].value, 1.0);
        assert_eq!(state.records[1].value, 1.0);
        assert_eq!(state.records[2].value, 120.0);

        clock.0.store(256, Ordering::Release);
        assert_eq!(provider.current_sample(), SampleTime(256));
        let transport = provider.transport_set(TransportSetRequest {
            state: TransportState::Playing,
            position: Some(SampleTime(64)),
        });
        assert_eq!(transport.effective_sample, SampleTime(256));
        assert_eq!(transport.position, SampleTime(64));

        provider.set_telemetry_counters(TelemetryCounters {
            telemetry_coalesced: 7,
            telemetry_dropped: 9,
        });
        let counters = provider
            .counters(&CountersRequest {
                all: true,
                ids: Vec::new(),
            })
            .expect("counters");
        assert_eq!(counters.observed_sample, SampleTime(256));
        assert_eq!(counters.values[0].value, 7);
        assert_eq!(counters.values[1].value, 9);

        provider.set_render_diagnostic(0, 256, 3, true);
        provider.set_render_diagnostic(1, 128, 2, true);
        let diagnostics = provider
            .diagnostics(DiagnosticsRequest {
                after_sequence: 0,
                limit: 2,
                minimum_severity: DiagnosticSeverity::Info,
            })
            .expect("diagnostics");
        assert_eq!(
            diagnostics
                .diagnostics
                .iter()
                .map(|diagnostic| diagnostic.provider_sequence)
                .collect::<Vec<_>>(),
            vec![Some(2), Some(3)]
        );
    }
}
