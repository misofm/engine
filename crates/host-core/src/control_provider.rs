//! Session-backed implementation of the transport-neutral control-provider boundary.

use std::sync::Arc;

use controls::{LiveRampRow, LiveRamps, MonitorOverlay, Motion, Owner, Section};
use effect_compiler::launch_native_effect_registry;
use effect_contract::control::RebuildReason;
use effect_contract::{EffectDescriptor, ParameterUnit, StepUnit};
use protocol::{
    ControlAddress, ControlBody, ControlClass, ControlEntry, ControlFamily, ControlKind,
    ControlListPage, ControlListRequest, ControlOwnerHeader, ControlOwnerHeaderKind,
    ControlProvider, ControlRampRow, ControlReason, ControlRouteEnds, ControlRow, ControlScope,
    ControlSection, ControlSectionHeader, ControlStatePage, ControlStateRecord, ControlTaper,
    ControllerRetainedCapacity, CounterId, CounterSnapshot, CounterValue, CountersRequest,
    Diagnostic, DiagnosticSeverity, DiagnosticsPage, DiagnosticsRequest, EnumChoice,
    ParameterHandle, ParameterProviderError, ParameterUnit as ProtocolUnit, SampleTime,
    TelemetryConfiguration, TelemetryCounters, TransportSetRequest, TransportSnapshot,
    TransportState,
};
use session::{EffectIdentity, SessionModel, StripKind};

/// The counters the provider serves, in ascending ID order (#1351 D1): the two telemetry
/// counters, the canceled-automation count, `live_values_superseded` (#1312 D8) and
/// `transition_reprepare_refusals` (D6). Each reads 0 from construction.
const SERVED_COUNTERS: [CounterId; 5] = [
    CounterId::TelemetryCoalesced,
    CounterId::TelemetryDropped,
    CounterId::CanceledAutomation,
    CounterId::LiveValuesSuperseded,
    CounterId::TransitionReprepareRefusals,
];

/// The slots the provider's counter snapshot keeps even when the controller retains fewer IDs.
const PROVIDER_COUNTER_SLOTS: usize = SERVED_COUNTERS.len();

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

/// The control list of one compiled session, fully allocated before a commit publishes it
/// (decision 17, C4): the entries of `controls::list` with an owner header before each owner's
/// rows and a section header before each section's, each at its handle, and the committed value
/// of every control row.
///
/// Handles count from 1 in list order, so an entry's handle is its index plus one and no handle
/// is stored twice. `values` holds the committed value of each entry, in step with `entries`; the
/// `value` inside a stored control row is always `0.0`, so that two catalogs' entries compare equal
/// exactly when their addresses, headers and descriptors do (a page fills the value in from
/// `values`).
pub struct ControlCatalog {
    entries: Vec<ControlEntry>,
    values: Vec<f32>,
    resources: CatalogResources,
}

/// The retained bytes of one catalog and its largest single allocation, the catalog-only part of
/// [`SessionControlProvider::resource_report`] (decision 17, C4). It is computed once, when the
/// catalog is built, so a live commit reads the current catalog's figure without a walk.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CatalogResources {
    /// Bytes the catalog retains: entries, values, choices, ladders and strings.
    pub retained_bytes: u64,
    /// The largest single allocation among them.
    pub largest_allocation_bytes: u64,
}

impl CatalogResources {
    fn of(entries: &[ControlEntry], values: &[f32]) -> Result<Self, SessionControlProviderError> {
        let row_count = entries.len();
        let mut enum_count = 0_usize;
        let mut ladder_count = 0_usize;
        let mut string_bytes = 0_usize;
        let mut largest_string = 0_u64;
        let mut largest_enum = 0_u64;
        let mut largest_ladder = 0_u64;
        let mut charge = |value: &str| -> Result<(), SessionControlProviderError> {
            string_bytes = string_bytes
                .checked_add(value.len())
                .ok_or(SessionControlProviderError)?;
            largest_string = largest_string
                .max(u64::try_from(value.len()).map_err(|_| SessionControlProviderError)?);
            Ok(())
        };
        for entry in entries {
            charge(entry.address.owner_id.as_deref().unwrap_or_default())?;
            charge(entry.address.slot_id.as_deref().unwrap_or_default())?;
            match &entry.body {
                ControlBody::OwnerHeader(header) => {
                    charge(&header.name)?;
                    if let Some(route) = &header.route {
                        charge(&route.source)?;
                        charge(&route.destination)?;
                    }
                }
                ControlBody::SectionHeader(header) => {
                    charge(&header.name)?;
                    charge(header.effect.as_deref().unwrap_or_default())?;
                }
                ControlBody::Control(row) => {
                    enum_count = enum_count
                        .checked_add(row.choices.len())
                        .ok_or(SessionControlProviderError)?;
                    ladder_count = ladder_count
                        .checked_add(row.ladder.len())
                        .ok_or(SessionControlProviderError)?;
                    largest_enum = largest_enum.max(bytes::<EnumChoice>(row.choices.len())?);
                    largest_ladder = largest_ladder.max(bytes::<f32>(row.ladder.len())?);
                    charge(&row.name)?;
                    charge(&row.display_name)?;
                    charge(&row.display_unit)?;
                    charge(row.reopen.as_deref().unwrap_or_default())?;
                    for choice in &row.choices {
                        charge(&choice.label)?;
                    }
                }
            }
        }
        let metadata_bytes = bytes::<ControlEntry>(row_count)?;
        let values_bytes = bytes::<f32>(values.len())?;
        let catalog_retained_bytes = [
            metadata_bytes,
            values_bytes,
            bytes::<EnumChoice>(enum_count)?,
            bytes::<f32>(ladder_count)?,
            u64::try_from(string_bytes).map_err(|_| SessionControlProviderError)?,
        ]
        .into_iter()
        .try_fold(0_u64, u64::checked_add)
        .ok_or(SessionControlProviderError)?;
        Ok(Self {
            retained_bytes: catalog_retained_bytes,
            largest_allocation_bytes: metadata_bytes
                .max(values_bytes)
                .max(largest_enum)
                .max(largest_ladder)
                .max(largest_string),
        })
    }
}

/// Production control provider whose catalog and state are derived from a compiled session.
pub struct SessionControlProvider {
    sample_source: Arc<dyn PlanSampleSource>,
    catalog: ControlCatalog,
    /// The structure the catalog's handles belong to: `1` for the first catalog, one more for
    /// each catalog whose address sequence differs from the one before.
    structure_epoch: u64,
    /// The committed session revision the catalog's values were read at.
    revision: u64,
    counter_snapshot: Vec<CounterValue>,
    diagnostics: Vec<Diagnostic>,
    diagnostic_live: Vec<bool>,
    transport_state: TransportState,
    transport_position: SampleTime,
    telemetry_configuration: TelemetryConfiguration,
}

impl SessionControlProvider {
    /// Project the allocations made by [`Self::try_new`] from its already prepared catalog.
    pub fn resource_report(
        catalog: &ControlCatalog,
        retained: ControllerRetainedCapacity,
        diagnostic_capacity: usize,
    ) -> Result<SessionControlProviderResources, SessionControlProviderError> {
        let catalog_resources = catalog.resources();
        let catalog_retained_bytes = catalog_resources.retained_bytes;
        let diagnostics_bytes = bytes::<Diagnostic>(diagnostic_capacity)?;
        let diagnostic_live_bytes = bytes::<bool>(diagnostic_capacity)?;
        let counter_snapshot_bytes =
            bytes::<CounterValue>(retained.counter_ids.max(PROVIDER_COUNTER_SLOTS))?;
        let diagnostic_code_bytes = "capi.render.activity"
            .len()
            .checked_mul(diagnostic_capacity)
            .ok_or(SessionControlProviderError)?;
        let diagnostic_code_bytes =
            u64::try_from(diagnostic_code_bytes).map_err(|_| SessionControlProviderError)?;
        let fixed_retained_bytes = diagnostics_bytes
            .checked_add(diagnostic_live_bytes)
            .and_then(|value| value.checked_add(diagnostic_code_bytes))
            .and_then(|value| value.checked_add(counter_snapshot_bytes))
            .ok_or(SessionControlProviderError)?;
        Ok(SessionControlProviderResources {
            catalog_retained_bytes,
            fixed_retained_bytes,
            largest_allocation_bytes: catalog_resources
                .largest_allocation_bytes
                .max(diagnostics_bytes)
                .max(diagnostic_live_bytes)
                .max(counter_snapshot_bytes)
                .max(
                    diagnostic_code_bytes
                        / u64::try_from(diagnostic_capacity.max(1))
                            .map_err(|_| SessionControlProviderError)?,
                ),
        })
    }

    /// Build the provider around its first catalog, read at `revision`, before the endpoint is
    /// published.
    pub fn try_new(
        catalog: ControlCatalog,
        revision: u64,
        sample_source: Arc<dyn PlanSampleSource>,
        retained: ControllerRetainedCapacity,
        diagnostic_capacity: usize,
    ) -> Result<Self, SessionControlProviderError> {
        let mut provider = Self {
            sample_source,
            catalog,
            structure_epoch: 1,
            revision,
            counter_snapshot: Vec::new(),
            diagnostics: Vec::new(),
            diagnostic_live: Vec::new(),
            transport_state: TransportState::Stopped,
            transport_position: SampleTime(0),
            telemetry_configuration: retained_telemetry(retained)?,
        };
        provider
            .counter_snapshot
            .try_reserve_exact(retained.counter_ids.max(PROVIDER_COUNTER_SLOTS))
            .map_err(|_| SessionControlProviderError)?;
        for id in SERVED_COUNTERS {
            provider
                .counter_snapshot
                .push(CounterValue { id, value: 0 });
        }
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
        Ok(provider)
    }

    /// Publish one fully prepared catalog, committed as session revision `revision`, without
    /// allocation or failure (the old one drops here, on the control thread). A live commit
    /// publishes the catalog of the committed model as well, which refreshes every value.
    ///
    /// The structure epoch advances by one when the new catalog's entries differ from the current
    /// ones in anything but a value: an address, a header or any control row's descriptor (a
    /// `matrix_mode` switch flips `active` on live). A client therefore never holds a stale
    /// descriptor under a current epoch. A commit that changes only values keeps the epoch.
    pub fn replace_session_catalog(&mut self, catalog: ControlCatalog, revision: u64) {
        if self.catalog.entries != catalog.entries {
            self.structure_epoch = self.structure_epoch.saturating_add(1);
        }
        self.catalog = catalog;
        self.revision = revision;
    }

    /// The retained bytes and largest allocation of the catalog the provider holds now.
    #[must_use]
    pub const fn catalog_resources(&self) -> CatalogResources {
        self.catalog.resources
    }

    /// The structure epoch the catalog's handles belong to.
    #[must_use]
    pub const fn structure_epoch(&self) -> u64 {
        self.structure_epoch
    }

    /// The entries of one page as the index range `start..end` of the catalog, and whether the
    /// list ends with it. Entries are contiguous per owner, so an owner filter is one run.
    fn window(
        &self,
        request: &ControlListRequest,
    ) -> Result<(usize, usize, bool), ParameterProviderError> {
        if request.structure_epoch != 0 && request.structure_epoch != self.structure_epoch {
            return Err(ParameterProviderError::Stale);
        }
        let entries = &self.catalog.entries;
        // Handle `h` is entry `h - 1`, so the entries after the cursor start at index `h`.
        let from = usize::try_from(request.after_handle)
            .unwrap_or(usize::MAX)
            .min(entries.len());
        let limit = usize::from(request.limit);
        let Some(owner) = &request.owner else {
            let end = from.saturating_add(limit).min(entries.len());
            return Ok((from, end, end == entries.len()));
        };
        let owned = |entry: &ControlEntry| {
            entry.address.owner_kind == owner.kind && entry.address.owner_id == owner.id
        };
        let start = entries[from..]
            .iter()
            .position(owned)
            .map_or(entries.len(), |offset| from + offset);
        let mut end = start;
        while end < entries.len() && end - start < limit && owned(&entries[end]) {
            end += 1;
        }
        Ok((start, end, end == entries.len() || !owned(&entries[end])))
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

    /// Record the session's `live_values_superseded` count (#1312 D8): the writes to adopted
    /// plans that a later write to the same cell replaced before a covering snapshot. The control
    /// plane copies it in beside the telemetry counters, so a counters read after any control
    /// call reports it.
    pub fn set_live_values_superseded(&mut self, count: u64) {
        set_counter(
            &mut self.counter_snapshot,
            CounterId::LiveValuesSuperseded,
            count,
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

    fn control_list(
        &mut self,
        request: &ControlListRequest,
    ) -> Result<ControlListPage, ParameterProviderError> {
        let (start, end, eof) = self.window(request)?;
        let mut entries = Vec::new();
        entries
            .try_reserve_exact(end - start)
            .map_err(|_| ParameterProviderError::Unavailable)?;
        for index in start..end {
            let mut entry = self.catalog.entries[index].clone();
            if let ControlBody::Control(row) = &mut entry.body {
                row.value = self.catalog.values[index];
            }
            entries.push(entry);
        }
        Ok(ControlListPage {
            revision: self.revision,
            structure_epoch: self.structure_epoch,
            last_handle: page_cursor(request, start, end),
            eof,
            entries,
        })
    }

    fn control_state(
        &mut self,
        request: &ControlListRequest,
    ) -> Result<ControlStatePage, ParameterProviderError> {
        let (start, end, eof) = self.window(request)?;
        let mut records = Vec::new();
        records
            .try_reserve_exact(end - start)
            .map_err(|_| ParameterProviderError::Unavailable)?;
        for index in start..end {
            if matches!(self.catalog.entries[index].body, ControlBody::Control(_)) {
                records.push(ControlStateRecord {
                    handle: handle_of(index),
                    value: self.catalog.values[index],
                });
            }
        }
        Ok(ControlStatePage {
            revision: self.revision,
            structure_epoch: self.structure_epoch,
            last_handle: page_cursor(request, start, end),
            eof,
            records,
        })
    }

    fn control_row(
        &mut self,
        handle: ParameterHandle,
    ) -> Result<&ControlEntry, ParameterProviderError> {
        let entry = usize::try_from(handle.0)
            .ok()
            .and_then(|handle| handle.checked_sub(1))
            .and_then(|index| self.catalog.entries.get(index))
            .ok_or(ParameterProviderError::NotFound)?;
        // Until #1567 only an effect parameter is a target of `AUTOMATION_ENQUEUE`: no other
        // row has a render record that a timed value could reach.
        match &entry.body {
            ControlBody::Control(row)
                if row.family == ControlFamily::Effect
                    && entry.address.control != Some(controls::BYPASS_CONTROL)
                    && entry.address.control != Some(controls::LINK_MODE_CONTROL) =>
            {
                Ok(entry)
            }
            _ => Err(ParameterProviderError::UnsupportedTarget),
        }
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
    ) -> Result<TelemetryConfiguration, ParameterProviderError> {
        // A counter this provider does not serve can never report; refuse it before anything
        // changes (#1351 D2).
        if configuration
            .counter_ids
            .iter()
            .any(|id| !SERVED_COUNTERS.contains(id))
        {
            return Err(ParameterProviderError::NotFound);
        }
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
        Ok(configuration)
    }
}

impl ControlCatalog {
    /// Build the catalog of `model` at `sample_rate`: one `controls::list` pass with the owner and
    /// section headers inserted, every allocation reserved up front.
    ///
    /// The derived rows show the default monitor overlay (no solo), as the committed model does.
    pub fn build(
        model: &SessionModel,
        sample_rate: u32,
    ) -> Result<Self, SessionControlProviderError> {
        let rows =
            controls::list(model, &MonitorOverlay::default(), sample_rate).collect::<Vec<_>>();
        // Every owner opens with a header and every section of it with another.
        let mut headers = 0_usize;
        let mut previous: Option<&controls::ControlAddress> = None;
        for row in &rows {
            let (owner_opens, section_opens) = openings(previous, &row.address);
            headers += usize::from(owner_opens) + usize::from(section_opens);
            previous = Some(&row.address);
        }
        let total = rows
            .len()
            .checked_add(headers)
            .filter(|total| u32::try_from(*total).is_ok())
            .ok_or(SessionControlProviderError)?;
        let mut entries = Vec::new();
        entries
            .try_reserve_exact(total)
            .map_err(|_| SessionControlProviderError)?;
        let mut values = Vec::new();
        values
            .try_reserve_exact(total)
            .map_err(|_| SessionControlProviderError)?;

        let ramps = LiveRamps::for_session(model);
        let mut strips = model.strips();
        let mut strip = None;
        let mut routes = model.routes.iter();
        let mut vcas = model.vcas.iter();
        let mut effect: Option<&'static EffectDescriptor> = None;
        let mut previous: Option<&controls::ControlAddress> = None;
        for row in &rows {
            let address = &row.address;
            let (owner_opens, section_opens) = openings(previous, address);
            previous = Some(address);
            if owner_opens {
                let header = match &address.owner {
                    Owner::Session => {
                        strip = None;
                        ControlOwnerHeader {
                            kind: ControlOwnerHeaderKind::Session,
                            name: try_string("session")?,
                            route: None,
                        }
                    }
                    Owner::Strip(id) => {
                        let found = strips
                            .by_ref()
                            .find(|candidate| candidate.id == id)
                            .ok_or(SessionControlProviderError)?;
                        strip = Some(found);
                        ControlOwnerHeader {
                            kind: match found.kind {
                                StripKind::Track(_) => ControlOwnerHeaderKind::Track,
                                StripKind::Submix(_) => ControlOwnerHeaderKind::Submix,
                            },
                            name: try_string(id.as_str())?,
                            route: None,
                        }
                    }
                    Owner::Route(id) => {
                        let found = routes
                            .by_ref()
                            .find(|candidate| candidate.id == *id)
                            .ok_or(SessionControlProviderError)?;
                        let (source, tap) = match &found.source {
                            session::RouteSource::Track { track_id, tap } => (track_id, *tap),
                            session::RouteSource::Submix { submix_id, tap } => (submix_id, *tap),
                        };
                        let destination = match &found.destination {
                            session::RouteDestination::SubmixInput { submix_id } => submix_id,
                            session::RouteDestination::OutputInput { output_id } => output_id,
                        };
                        ControlOwnerHeader {
                            kind: ControlOwnerHeaderKind::Route,
                            name: try_string(id.as_str())?,
                            route: Some(ControlRouteEnds {
                                source: try_string(source.as_str())?,
                                destination: try_string(destination.as_str())?,
                                tap,
                            }),
                        }
                    }
                    Owner::Vca(id) => {
                        vcas.by_ref()
                            .find(|candidate| candidate.id == *id)
                            .ok_or(SessionControlProviderError)?;
                        ControlOwnerHeader {
                            kind: ControlOwnerHeaderKind::Vca,
                            name: try_string(id.as_str())?,
                            route: None,
                        }
                    }
                };
                let mut owner_address = wire_address(address)?;
                owner_address.section = ControlSection::Own;
                owner_address.slot_id = None;
                owner_address.control = None;
                owner_address.lane = None;
                entries.push(ControlEntry {
                    handle: handle_of(entries.len()),
                    address: owner_address,
                    body: ControlBody::OwnerHeader(header),
                });
                values.push(0.0);
            }
            if section_opens {
                let identity = match &address.section {
                    Section::Console(slot) => model
                        .console
                        .slots()
                        .find(|candidate| candidate.slot == *slot)
                        .map(|candidate| &candidate.identity),
                    Section::Inserts(id) => strip.and_then(|strip| {
                        strip
                            .inserts
                            .effects
                            .iter()
                            .find(|candidate| candidate.id == *id)
                            .map(|candidate| &candidate.identity)
                    }),
                    Section::Own | Section::Builtins => None,
                };
                effect = identity.and_then(contract);
                let (name, effect_identity) = match (&address.section, identity) {
                    (Section::Own, _) => ("own", None),
                    (Section::Builtins, _) => ("builtins", None),
                    (Section::Console(id) | Section::Inserts(id), identity) => (
                        id.as_str(),
                        match identity {
                            Some(EffectIdentity::Native { effect_id }) => Some(effect_id.as_str()),
                            _ => None,
                        },
                    ),
                };
                let mut section_address = wire_address(address)?;
                section_address.control = None;
                section_address.lane = None;
                entries.push(ControlEntry {
                    handle: handle_of(entries.len()),
                    address: section_address,
                    body: ControlBody::SectionHeader(ControlSectionHeader {
                        name: try_string(name)?,
                        effect: effect_identity.map(try_string).transpose()?,
                    }),
                });
                values.push(0.0);
            }
            entries.push(ControlEntry {
                handle: handle_of(entries.len()),
                address: wire_address(address)?,
                body: ControlBody::Control(control_row(row, ramps, effect)?),
            });
            values.push(row.value);
        }
        let resources = CatalogResources::of(&entries, &values)?;
        Ok(Self {
            entries,
            values,
            resources,
        })
    }

    /// The catalog's retained bytes and largest single allocation, as
    /// [`SessionControlProvider::resource_report`] charges them.
    #[must_use]
    pub const fn resources(&self) -> CatalogResources {
        self.resources
    }
}

/// Whether `address` opens a new owner and whether it opens a new section, after `previous`.
fn openings(
    previous: Option<&controls::ControlAddress>,
    address: &controls::ControlAddress,
) -> (bool, bool) {
    let owner_opens = previous.is_none_or(|previous| previous.owner != address.owner);
    let section_opens =
        owner_opens || previous.is_none_or(|previous| previous.section != address.section);
    (owner_opens, section_opens)
}

/// The launch descriptor of a native effect identity, or `None` for any other.
fn contract(identity: &EffectIdentity) -> Option<&'static EffectDescriptor> {
    let EffectIdentity::Native { effect_id } = identity else {
        return None;
    };
    launch_native_effect_registry()
        .ok()?
        .get_ascii(effect_id.as_str())
        .map(|factory| factory.descriptor())
}

/// The wire address of a control row's address, from the protocol's one conversion with a
/// fallible copy of each ID.
fn wire_address(
    address: &controls::ControlAddress,
) -> Result<ControlAddress, SessionControlProviderError> {
    ControlAddress::try_from_controls(address, try_string)
}

/// The wire row of one control: its descriptor, with the value left at `0.0`. `effect` is the
/// contract of the effect section the row is in, which gives an effect parameter's own ramp.
fn control_row(
    row: &controls::ControlRow,
    ramps: LiveRamps,
    effect: Option<&'static EffectDescriptor>,
) -> Result<ControlRow, SessionControlProviderError> {
    let descriptor = &row.descriptor;
    // Reserved exactly, as every other catalog allocation is: `resource_report` charges these by
    // length, and collecting through `Result<Vec<_>, _>` (which has no lower size bound) grew six
    // choices to a capacity of eight, 64 retained bytes per EQ kind descriptor that no row
    // charged (#1060).
    let mut choices = Vec::new();
    choices
        .try_reserve_exact(descriptor.choices.len())
        .map_err(|_| SessionControlProviderError)?;
    for choice in descriptor.choices {
        choices.push(EnumChoice {
            value: choice.value,
            label: try_string(choice.label)?,
        });
    }
    // The named step sizes are the lattice multiples of the step; an index lattice (a toggle or
    // an enumeration) has none.
    let mut ladder = Vec::new();
    if descriptor.lattice.step_unit != StepUnit::Index {
        ladder
            .try_reserve_exact(descriptor.lattice.ladder.multiples.len())
            .map_err(|_| SessionControlProviderError)?;
        ladder.extend(
            descriptor
                .lattice
                .ladder
                .multiples
                .iter()
                .map(|multiple| f32::from(*multiple)),
        );
    }
    let (ramp_row, default_ramp_samples) = match descriptor.motion {
        Motion::Ramp(live) => (Some(ramp_row(live)), ramps.for_row(live)),
        // A coefficient ramp that the effect's own kernel owns.
        Motion::Fixed => (
            None,
            effect
                .filter(|_| descriptor.family == controls::Family::Effect)
                .and_then(|effect| {
                    effect
                        .parameters
                        .iter()
                        .find(|parameter| parameter.id.0 == row.address.control)
                })
                .map_or(0, |parameter| parameter.smoothing_samples),
        ),
    };
    let (class, reason, reason_issue, reopen) = match descriptor.class {
        controls::ControlClass::Live => (ControlClass::Live, None, None, None),
        controls::ControlClass::Derived => (ControlClass::Derived, None, None, None),
        controls::ControlClass::RebuildOnly { reason, reopen } => (
            ControlClass::RebuildOnly,
            Some(control_reason(reason)),
            match reason {
                RebuildReason::RuledLiveNotBuilt { issue } => Some(issue),
                _ => None,
            },
            Some(try_string(reopen)?),
        ),
    };
    Ok(ControlRow {
        name: try_string(descriptor.name)?,
        display_name: try_string(descriptor.display_name)?,
        display_unit: try_string(descriptor.display_unit)?,
        unit: protocol_unit(descriptor.unit),
        kind: match descriptor.kind {
            controls::ControlKind::Continuous => ControlKind::Continuous,
            controls::ControlKind::Toggle => ControlKind::Toggle,
            controls::ControlKind::Enumeration => ControlKind::Enumeration,
        },
        minimum: descriptor.minimum,
        maximum: descriptor.maximum,
        disabled_value: descriptor.disabled_value,
        default: descriptor.default,
        taper: match descriptor.taper {
            controls::Taper::Linear => ControlTaper::Linear,
            controls::Taper::Logarithmic => ControlTaper::Logarithmic,
            controls::Taper::Exponential => ControlTaper::Exponential,
            controls::Taper::Decibel => ControlTaper::Decibel,
            controls::Taper::Stepped => ControlTaper::Stepped,
        },
        step: descriptor.lattice.step,
        precision: descriptor.lattice.precision,
        ladder,
        choices,
        ramp_row,
        default_ramp_samples,
        class,
        reason,
        reason_issue,
        reopen,
        scope: match descriptor.scope {
            controls::ControlScope::Model => ControlScope::Model,
            controls::ControlScope::Monitor => ControlScope::Monitor,
        },
        automatable: descriptor.automatable(),
        family: match descriptor.family {
            controls::Family::StripLevel => ControlFamily::StripLevel,
            controls::Family::StripMatrix => ControlFamily::StripMatrix,
            controls::Family::StripInput => ControlFamily::StripInput,
            controls::Family::Route => ControlFamily::Route,
            controls::Family::Effect => ControlFamily::Effect,
            controls::Family::Setting => ControlFamily::Setting,
        },
        active: descriptor.active,
        // The committed value lives in the catalog's `values`.
        value: 0.0,
    })
}

const fn ramp_row(value: LiveRampRow) -> ControlRampRow {
    match value {
        LiveRampRow::Fader => ControlRampRow::Fader,
        LiveRampRow::InputTrim => ControlRampRow::InputTrim,
        LiveRampRow::SendGain => ControlRampRow::SendGain,
        LiveRampRow::VcaOffset => ControlRampRow::VcaOffset,
        LiveRampRow::LinkGlide => ControlRampRow::LinkGlide,
        LiveRampRow::Mute => ControlRampRow::Mute,
        LiveRampRow::Solo => ControlRampRow::Solo,
        LiveRampRow::PolarityInvert => ControlRampRow::PolarityInvert,
        LiveRampRow::SendMute => ControlRampRow::SendMute,
        LiveRampRow::SendFollowsMute => ControlRampRow::SendFollowsMute,
        LiveRampRow::VcaMute => ControlRampRow::VcaMute,
        LiveRampRow::BypassCrossfade => ControlRampRow::BypassCrossfade,
        LiveRampRow::Pan => ControlRampRow::Pan,
        LiveRampRow::Matrix => ControlRampRow::Matrix,
        LiveRampRow::SendMatrix => ControlRampRow::SendMatrix,
    }
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

/// The handle of the entry at `index`: handles count from 1 in list order.
fn handle_of(index: usize) -> u32 {
    u32::try_from(index + 1).expect("the catalog holds at most u32::MAX entries")
}

/// A page's `last_handle`: its last entry's, or the request's cursor when it holds none.
fn page_cursor(request: &ControlListRequest, start: usize, end: usize) -> u32 {
    if end > start {
        handle_of(end - 1)
    } else {
        request.after_handle
    }
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

const fn control_reason(value: RebuildReason) -> ControlReason {
    match value {
        RebuildReason::NewMemory => ControlReason::NewMemory,
        RebuildReason::NewGraph => ControlReason::NewGraph,
        RebuildReason::ProcessingOrder => ControlReason::ProcessingOrder,
        RebuildReason::Latency => ControlReason::Latency,
        RebuildReason::Glitch => ControlReason::Glitch,
        RebuildReason::Optimisation => ControlReason::Optimisation,
        RebuildReason::Correctness => ControlReason::Correctness,
        RebuildReason::RuledLiveNotBuilt { .. } => ControlReason::RuledLiveNotBuilt,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU64, Ordering};

    use effect_contract::{AutomationRate, ParameterChannel as PreparedChannel};
    use effect_contract::{ParameterChannelPolicy, ParameterDomain};

    use protocol::{ControlOwnerKind, ParameterChannel};

    use super::*;

    const RATE: u32 = 48_000;

    /// Every live-ramp row maps to the wire ramp row of the same name and code. The match has no
    /// wildcard, so a new row must be classified here.
    ///
    /// Red if `ramp_row` swaps two rows: the list would then tell a client that a fader's default
    /// ramp comes from another `control_smoothing` key than the one the core reads.
    #[test]
    fn every_live_ramp_row_maps_to_its_wire_code() {
        const fn expected(row: LiveRampRow) -> u8 {
            match row {
                LiveRampRow::Fader => 1,
                LiveRampRow::InputTrim => 2,
                LiveRampRow::SendGain => 3,
                LiveRampRow::VcaOffset => 4,
                LiveRampRow::LinkGlide => 5,
                LiveRampRow::Mute => 6,
                LiveRampRow::Solo => 7,
                LiveRampRow::PolarityInvert => 8,
                LiveRampRow::SendMute => 9,
                LiveRampRow::SendFollowsMute => 10,
                LiveRampRow::VcaMute => 11,
                LiveRampRow::BypassCrossfade => 12,
                LiveRampRow::Pan => 13,
                LiveRampRow::Matrix => 14,
                LiveRampRow::SendMatrix => 15,
            }
        }
        let rows = [
            LiveRampRow::Fader,
            LiveRampRow::InputTrim,
            LiveRampRow::SendGain,
            LiveRampRow::VcaOffset,
            LiveRampRow::LinkGlide,
            LiveRampRow::Mute,
            LiveRampRow::Solo,
            LiveRampRow::PolarityInvert,
            LiveRampRow::SendMute,
            LiveRampRow::SendFollowsMute,
            LiveRampRow::VcaMute,
            LiveRampRow::BypassCrossfade,
            LiveRampRow::Pan,
            LiveRampRow::Matrix,
            LiveRampRow::SendMatrix,
        ];
        assert_eq!(rows.len(), protocol::ControlRampRow::CODES.len());
        for row in rows {
            assert_eq!(ramp_row(row).code(), expected(row), "{row:?}");
        }
    }

    fn compile(model: &session::SessionModel) -> session::CompiledSession {
        session::compile_session(
            model,
            session::CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled fixture")
    }

    fn prepare_effects(model: &session::SessionModel) -> effect_compiler::EffectPreparedSession {
        let compiled = compile(model);
        let registry = effect_compiler::launch_native_effect_registry().expect("registry");
        effect_compiler::prepare_native_session_effects(
            &compiled,
            registry,
            effect_compiler::EffectCompileCaps {
                maximum_total_state_bytes: u64::MAX,
                maximum_scratch_bytes: u64::MAX,
                maximum_automation_spans_per_block: u32::MAX,
            },
        )
        .expect("prepared effects")
    }

    struct Clock(AtomicU64);

    impl PlanSampleSource for Clock {
        fn next_absolute_sample(&self) -> u64 {
            self.0.load(Ordering::Acquire)
        }
    }

    fn nine_track() -> session::SessionModel {
        session::parse_session_json(include_str!(
            "../../../fixtures/session/v1/parametric-eq-nine-track.json"
        ))
        .expect("session fixture")
    }

    fn provider_of(model: &session::SessionModel) -> (SessionControlProvider, Arc<Clock>) {
        let catalog = ControlCatalog::build(model, RATE).expect("catalog");
        let clock = Arc::new(Clock(AtomicU64::new(0)));
        let sample_source: Arc<dyn PlanSampleSource> = Arc::clone(&clock) as Arc<_>;
        let provider = SessionControlProvider::try_new(
            catalog,
            1,
            sample_source,
            ControllerRetainedCapacity {
                meter_handles: 4,
                counter_ids: 0,
            },
            2,
        )
        .expect("provider");
        (provider, clock)
    }

    fn page(
        provider: &mut SessionControlProvider,
        epoch: u64,
        after_handle: u32,
        limit: u16,
        owner: Option<protocol::ControlOwnerFilter>,
    ) -> ControlListPage {
        provider
            .control_list(&ControlListRequest {
                structure_epoch: epoch,
                after_handle,
                limit,
                owner,
            })
            .expect("a page")
    }

    fn whole_list(provider: &mut SessionControlProvider, limit: u16) -> Vec<ControlEntry> {
        let mut entries = Vec::new();
        let mut after_handle = 0;
        loop {
            let next = page(provider, 0, after_handle, limit, None);
            after_handle = next.last_handle;
            entries.extend(next.entries);
            if next.eof {
                return entries;
            }
        }
    }

    fn control_value(entry: &ControlEntry) -> f32 {
        match &entry.body {
            ControlBody::Control(row) => row.value,
            _ => panic!("a control row"),
        }
    }

    fn find<'a>(
        entries: &'a [ControlEntry],
        owner: &str,
        section: ControlSection,
        slot: Option<&str>,
        control: u32,
        lane: ParameterChannel,
    ) -> &'a ControlEntry {
        entries
            .iter()
            .find(|entry| {
                let address = &entry.address;
                address.owner_id.as_deref() == Some(owner)
                    && address.section == section
                    && address.slot_id.as_deref() == slot
                    && address.control == Some(control)
                    && address.lane == Some(lane)
            })
            .expect("the row is listed")
    }

    fn fader_address(model: &session::SessionModel, strip: &str) -> controls::ControlAddress {
        let row = controls::list(model, &MonitorOverlay::default(), RATE)
            .find(|row| {
                row.address.owner == Owner::Strip(session::StableId::parse(strip).expect("ID"))
                    && row.address.section == Section::Builtins
                    && row.descriptor.name == "fader_db"
                    && row.address.lane == session::ParameterChannel::Left
            })
            .expect("the fader row");
        row.address
    }

    #[test]
    fn the_catalog_lists_every_control_of_the_model_after_its_headers() {
        let mut model = nine_track();
        // A route, a VCA and a second console section make every owner and section kind appear.
        model.vcas.push(session::Vca {
            id: session::StableId::parse("vca0").expect("ID"),
            fader: model.tracks[0].fader.clone(),
            members: vec![model.tracks[0].id.clone()],
        });
        let catalog = ControlCatalog::build(&model, RATE).expect("catalog");
        let expected = controls::list(&model, &MonitorOverlay::default(), RATE).collect::<Vec<_>>();
        let controls_listed = catalog
            .entries
            .iter()
            .filter(|entry| matches!(entry.body, ControlBody::Control(_)))
            .count();
        assert_eq!(controls_listed, expected.len());
        assert_eq!(catalog.values.len(), catalog.entries.len());

        let mut next_control = expected.iter();
        let mut owner: Option<&ControlAddress> = None;
        let mut section: Option<&ControlAddress> = None;
        for (index, entry) in catalog.entries.iter().enumerate() {
            assert_eq!(entry.handle as usize, index + 1, "handles count from 1");
            match &entry.body {
                ControlBody::OwnerHeader(_) => {
                    assert_eq!(entry.address.section, ControlSection::Own);
                    owner = Some(&entry.address);
                    section = None;
                }
                ControlBody::SectionHeader(_) => {
                    let owner = owner.expect("a section header follows its owner header");
                    assert_eq!(entry.address.owner_kind, owner.owner_kind);
                    assert_eq!(entry.address.owner_id, owner.owner_id);
                    section = Some(&entry.address);
                }
                ControlBody::Control(row) => {
                    let expected = next_control.next().expect("a listed control");
                    let section = section.expect("a control follows its section header");
                    assert_eq!(entry.address.owner_id, section.owner_id);
                    assert_eq!(entry.address.section, section.section);
                    assert_eq!(entry.address.slot_id, section.slot_id);
                    assert_eq!(entry.address.control, Some(expected.address.control));
                    assert_eq!(row.name, expected.descriptor.name);
                    assert_eq!(row.automatable, expected.descriptor.automatable());
                    assert_eq!(
                        catalog.values[index].to_bits(),
                        expected.value.to_bits(),
                        "{}",
                        row.name
                    );
                }
            }
        }
        assert!(
            next_control.next().is_none(),
            "every listed control is held"
        );
        // Every owner kind has a header: strips, the VCA, routes and the session.
        let kinds = catalog
            .entries
            .iter()
            .filter_map(|entry| match &entry.body {
                ControlBody::OwnerHeader(header) => Some(header.kind),
                _ => None,
            })
            .collect::<Vec<_>>();
        for kind in [
            ControlOwnerHeaderKind::Track,
            ControlOwnerHeaderKind::Route,
            ControlOwnerHeaderKind::Vca,
            ControlOwnerHeaderKind::Session,
        ] {
            assert!(kinds.contains(&kind), "{kind:?}");
        }
    }

    #[test]
    fn an_effect_parameter_row_agrees_with_the_accepted_preparation_authority() {
        let mut model = nine_track();
        // One track carries every launch effect as an insert and every console-eligible one as a
        // slot in each console section (decision 12), so all three lowered racks are exercised.
        let template = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
        let first_track = model.tracks[0].id.clone();
        model.tracks.truncate(1);
        model.routes.retain(|route| {
            matches!(&route.source, session::RouteSource::Track { track_id, .. } if *track_id == first_track)
        });
        model.automation.clear();
        model.console.pre_insert.clear();
        model.console.post_insert.clear();
        model.tracks[0].console.clear();
        model.tracks[0].inserts.effects.clear();
        let registry = effect_compiler::launch_native_effect_registry().expect("registry");
        let descriptor_ids = registry
            .descriptors()
            .map(|descriptor| descriptor.id.as_str().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(descriptor_ids.len(), 8, "complete launch registry");
        let eligible = descriptor_ids
            .iter()
            .filter(|id| effect_compiler::CONSOLE_ELIGIBLE_EFFECTS.contains(&id.as_str()))
            .count();
        let mut entries = [Vec::new(), Vec::new()];
        for rack_index in 0..3 {
            for (effect_index, descriptor_id) in descriptor_ids.iter().enumerate() {
                let mut effect = template.clone();
                effect.id = session::StableId::parse(&format!("r{rack_index}e{effect_index}"))
                    .expect("stable effect ID");
                effect.identity = session::EffectIdentity::Native {
                    effect_id: session::StableId::parse(descriptor_id).expect("native effect ID"),
                };
                if rack_index != 0 || descriptor_id != "miso.parametric-eq" {
                    effect.params.clear();
                }
                if rack_index == 1 {
                    model.tracks[0].inserts.effects.push(effect);
                    continue;
                }
                if !effect_compiler::CONSOLE_ELIGIBLE_EFFECTS.contains(&descriptor_id.as_str()) {
                    continue;
                }
                let slot = session::ConsoleSlot {
                    slot: effect.id.clone(),
                    identity: effect.identity.clone(),
                    quality: effect.quality,
                    link_mode: effect.link_mode,
                };
                let entry = session::ConsoleEntry {
                    slot: effect.id.clone(),
                    bypass: false,
                    params: effect.params.clone(),
                };
                if rack_index == 0 {
                    model.console.pre_insert.push(slot);
                    entries[0].push(entry);
                } else {
                    model.console.post_insert.push(slot);
                    entries[1].push(entry);
                }
            }
        }
        // Entries follow the slots in session order: `pre_insert`, then `post_insert`.
        let [pre_entries, post_entries] = entries;
        model.tracks[0].console = pre_entries.into_iter().chain(post_entries).collect();

        let effects = prepare_effects(&model);
        assert_eq!(effects.entries.len(), descriptor_ids.len() + 2 * eligible);
        let catalog = ControlCatalog::build(&model, RATE).expect("catalog");
        let mut saw_both = false;
        let mut saw_left = false;
        let mut saw_right = false;
        let mut saw_override = false;
        let mut checked = 0_usize;
        for entry in &effects.entries {
            let section = match entry.rack {
                effect_compiler::EffectRack::Simd1 | effect_compiler::EffectRack::Simd2 => {
                    ControlSection::Console
                }
                effect_compiler::EffectRack::Dynamic => ControlSection::Inserts,
            };
            for (parameter_index, parameter) in
                entry.metadata.descriptor.parameters.iter().enumerate()
            {
                let channels: &[PreparedChannel] = match parameter.channel_policy {
                    ParameterChannelPolicy::Shared => &[PreparedChannel::Both],
                    ParameterChannelPolicy::PerLane => {
                        &[PreparedChannel::Left, PreparedChannel::Right]
                    }
                };
                for channel in channels {
                    let lane = match channel {
                        PreparedChannel::Left => ParameterChannel::Left,
                        PreparedChannel::Right => ParameterChannel::Right,
                        PreparedChannel::Both => ParameterChannel::Both,
                    };
                    let listed = find(
                        &catalog.entries,
                        &entry.track_id,
                        section,
                        Some(&entry.effect_id),
                        parameter.id.0,
                        lane,
                    );
                    let ControlBody::Control(control) = &listed.body else {
                        panic!("a parameter is a control row");
                    };
                    let initial = entry
                        .bank_preparation
                        .initial_values
                        .iter()
                        .find(|value| {
                            value.parameter_index as usize == parameter_index
                                && value.channel == *channel
                        })
                        .expect("accepted initial value");
                    assert_eq!(control.name, parameter.name);
                    assert_eq!(control.family, ControlFamily::Effect);
                    assert_eq!(
                        control.automatable,
                        parameter.automatable && parameter.automation_rate != AutomationRate::None
                    );
                    assert_eq!(
                        catalog.values[listed.handle as usize - 1].to_bits(),
                        initial.value.to_bits(),
                        "the listed value is the accepted initial value"
                    );
                    assert_eq!(control.default.to_bits(), parameter.default_value.to_bits());
                    assert_eq!(
                        control.default_ramp_samples, parameter.smoothing_samples,
                        "an effect parameter's own ramp"
                    );
                    if parameter.domain == ParameterDomain::Enumeration {
                        assert_eq!(control.choices.len(), parameter.enum_choices.len());
                    }
                    saw_override |= initial.value.to_bits() != parameter.default_value.to_bits();
                    saw_both |= *channel == PreparedChannel::Both;
                    saw_left |= *channel == PreparedChannel::Left;
                    saw_right |= *channel == PreparedChannel::Right;
                    checked += 1;
                }
            }
        }
        assert!(checked > 0 && saw_both && saw_left && saw_right && saw_override);
    }

    #[test]
    fn pages_are_the_same_list_at_any_limit_and_an_owner_filter_keeps_global_handles() {
        let model = nine_track();
        let (mut provider, _clock) = provider_of(&model);
        let whole = whole_list(&mut provider, 256);
        assert_eq!(whole_list(&mut provider, 1), whole);
        assert_eq!(whole_list(&mut provider, 7), whole);

        let first = page(&mut provider, 0, 0, 10, None);
        assert_eq!(first.structure_epoch, 1);
        assert_eq!(first.revision, 1);
        assert_eq!(first.last_handle, 10);
        assert!(!first.eof);
        let end = page(&mut provider, 1, whole.len() as u32, 10, None);
        assert!(end.eof && end.entries.is_empty());
        assert_eq!(
            end.last_handle,
            whole.len() as u32,
            "an empty page keeps the cursor"
        );

        // One owner's header, section headers and rows, with their global handles, in pages.
        let owner = protocol::ControlOwnerFilter {
            kind: ControlOwnerKind::Strip,
            id: Some("eq3".to_owned()),
        };
        let expected = whole
            .iter()
            .filter(|entry| entry.address.owner_id.as_deref() == Some("eq3"))
            .cloned()
            .collect::<Vec<_>>();
        assert!(expected.len() > 10);
        assert!(matches!(expected[0].body, ControlBody::OwnerHeader(_)));
        let mut listed = Vec::new();
        let mut after_handle = 0;
        loop {
            let next = page(&mut provider, 0, after_handle, 5, Some(owner.clone()));
            after_handle = next.last_handle;
            listed.extend(next.entries);
            if next.eof {
                break;
            }
        }
        assert_eq!(listed, expected);
        // A cursor inside the owner's run resumes there; an unknown owner is an empty page.
        let resumed = page(&mut provider, 0, expected[3].handle, 256, Some(owner));
        assert_eq!(resumed.entries, expected[4..]);
        assert!(resumed.eof);
        let unknown = page(
            &mut provider,
            0,
            0,
            256,
            Some(protocol::ControlOwnerFilter {
                kind: ControlOwnerKind::Vca,
                id: Some("nobody".to_owned()),
            }),
        );
        assert!(unknown.eof && unknown.entries.is_empty());

        // A state page covers the same rows and has no header rows.
        let state = provider
            .control_state(&ControlListRequest {
                structure_epoch: 1,
                after_handle: 0,
                limit: 256,
                owner: None,
            })
            .expect("state");
        assert_eq!(state.last_handle, 256);
        assert_eq!(
            state
                .records
                .iter()
                .map(|record| (record.handle, record.value.to_bits()))
                .collect::<Vec<_>>(),
            whole[..256]
                .iter()
                .filter(|entry| matches!(entry.body, ControlBody::Control(_)))
                .map(|entry| (entry.handle, control_value(entry).to_bits()))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            provider
                .control_list(&ControlListRequest {
                    structure_epoch: 2,
                    after_handle: 0,
                    limit: 1,
                    owner: None,
                })
                .err(),
            Some(ParameterProviderError::Stale)
        );
    }

    #[test]
    fn a_commit_refreshes_values_and_only_a_changed_address_sequence_advances_the_epoch() {
        let mut model = nine_track();
        let (mut provider, _clock) = provider_of(&model);
        let before = whole_list(&mut provider, 256);
        let address = fader_address(&model, "eq0");
        let handle = before
            .iter()
            .find(|entry| {
                entry.address.owner_id.as_deref() == Some("eq0")
                    && entry.address.section == ControlSection::Builtins
                    && entry.address.control == Some(address.control)
                    && entry.address.lane == Some(ParameterChannel::Left)
            })
            .expect("the fader row")
            .handle;

        // A live-style commit: the values move, the handles and the epoch do not.
        controls::set(&mut model, &address, -6.0, None, RATE).expect("set the fader");
        provider.replace_session_catalog(ControlCatalog::build(&model, RATE).expect("catalog"), 2);
        let after = whole_list(&mut provider, 256);
        assert_eq!(provider.structure_epoch(), 1);
        assert_eq!(
            page(&mut provider, 1, 0, 1, None).revision,
            2,
            "pages report the committed revision"
        );
        assert_eq!(control_value(&after[handle as usize - 1]), -6.0);
        for (old, new) in before.iter().zip(&after) {
            assert_eq!(old.handle, new.handle);
            assert_eq!(old.address, new.address);
        }

        // A rebuild that keeps the address sequence keeps the epoch, and one that changes it does
        // not.
        let delay = controls::list(&model, &MonitorOverlay::default(), RATE)
            .find(|row| {
                row.descriptor.name == "delay_samples"
                    && row.address.lane == session::ParameterChannel::Left
            })
            .expect("a rebuild-only value")
            .address;
        controls::set(&mut model, &delay, 48.0, None, RATE).expect("set the delay");
        let same = ControlCatalog::build(&model, RATE).expect("catalog");
        provider.replace_session_catalog(same, 3);
        assert_eq!(provider.structure_epoch(), 1);
        // A switch to matrix mode flips `active` on the pan and matrix rows and no address: the
        // epoch still advances, since a client's descriptors would be stale.
        let mode = controls::list(&model, &MonitorOverlay::default(), RATE)
            .find(|row| {
                row.descriptor.name == "matrix_mode"
                    && row.address.owner
                        == Owner::Strip(session::StableId::parse("eq0").expect("ID"))
            })
            .expect("the mode row")
            .address;
        controls::set(&mut model, &mode, 1.0, None, RATE).expect("switch to matrix");
        let before_switch = whole_list(&mut provider, 256);
        provider.replace_session_catalog(ControlCatalog::build(&model, RATE).expect("catalog"), 4);
        assert_eq!(provider.structure_epoch(), 2, "a descriptor changed");
        let after_switch = whole_list(&mut provider, 256);
        let active = |list: &[ControlEntry]| {
            list.iter()
                .map(|entry| match &entry.body {
                    ControlBody::Control(row) => row.active,
                    _ => true,
                })
                .collect::<Vec<_>>()
        };
        assert_ne!(active(&before_switch), active(&after_switch));
        assert_eq!(
            before_switch
                .iter()
                .map(|entry| &entry.address)
                .collect::<Vec<_>>(),
            after_switch
                .iter()
                .map(|entry| &entry.address)
                .collect::<Vec<_>>(),
            "only descriptors moved"
        );
        model.tracks.pop();
        let changed = ControlCatalog::build(&model, RATE).expect("catalog");
        provider.replace_session_catalog(changed, 5);
        assert_eq!(provider.structure_epoch(), 3);
        assert_eq!(
            provider
                .control_list(&ControlListRequest {
                    structure_epoch: 2,
                    after_handle: 0,
                    limit: 1,
                    owner: None,
                })
                .err(),
            Some(ParameterProviderError::Stale)
        );
    }

    #[test]
    fn only_an_effect_parameter_is_an_automation_target() {
        let model = nine_track();
        let (mut provider, _clock) = provider_of(&model);
        let list = whole_list(&mut provider, 256);
        let parameter = list
            .iter()
            .find(|entry| {
                matches!(&entry.body, ControlBody::Control(row)
                    if row.family == ControlFamily::Effect && row.automatable)
                    && entry.address.control != Some(controls::BYPASS_CONTROL)
            })
            .expect("a live effect parameter");
        assert_eq!(
            provider
                .control_row(ParameterHandle(parameter.handle))
                .expect("a parameter")
                .address,
            parameter.address
        );
        let others = [
            // The bypass, the fader (a strip builtin), a header and a handle past the end.
            list.iter().find(|entry| {
                entry.address.control == Some(controls::BYPASS_CONTROL)
                    && matches!(entry.body, ControlBody::Control(_))
            }),
            list.iter().find(|entry| {
                matches!(&entry.body, ControlBody::Control(row)
                    if row.family == ControlFamily::StripLevel)
            }),
            list.first(),
        ];
        for entry in others {
            let entry = entry.expect("a row");
            assert_eq!(
                provider.control_row(ParameterHandle(entry.handle)).err(),
                Some(ParameterProviderError::UnsupportedTarget),
                "{:?}",
                entry.address
            );
        }
        for handle in [0, list.len() as u32 + 1] {
            assert_eq!(
                provider.control_row(ParameterHandle(handle)).err(),
                Some(ParameterProviderError::NotFound)
            );
        }
    }

    #[test]
    fn clock_transport_counters_and_diagnostics_are_session_backed() {
        let (mut provider, clock) = provider_of(&nine_track());
        clock.0.store(256, Ordering::Release);
        assert_eq!(provider.current_sample(), SampleTime(256));
        let transport = provider.transport_set(TransportSetRequest {
            state: TransportState::Playing,
            position: Some(SampleTime(64)),
        });
        assert_eq!(transport.effective_sample, SampleTime(256));
        assert_eq!(transport.position, SampleTime(64));

        let counter_capacity = provider.retained_capacities().1;
        assert_eq!(counter_capacity, PROVIDER_COUNTER_SLOTS);
        provider.set_telemetry_counters(TelemetryCounters {
            telemetry_coalesced: 7,
            telemetry_dropped: 9,
        });
        provider.record_canceled_automation(11);
        provider.set_live_values_superseded(13);
        let counters = provider
            .counters(&CountersRequest {
                all: true,
                ids: Vec::new(),
            })
            .expect("counters");
        assert_eq!(counters.observed_sample, SampleTime(256));
        assert_eq!(counters.values[0].value, 7);
        assert_eq!(counters.values[1].value, 9);
        assert_eq!(counters.values[2].value, 11);
        assert_eq!(counters.values[3].id, CounterId::LiveValuesSuperseded);
        assert_eq!(counters.values[3].value, 13);
        assert_eq!(
            counters.values[4].id,
            CounterId::TransitionReprepareRefusals
        );
        assert_eq!(counters.values[4].value, 0);
        assert_eq!(provider.retained_capacities().1, counter_capacity);

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
