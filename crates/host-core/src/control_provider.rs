//! Session-backed implementation of the transport-neutral control-provider boundary.

use std::collections::HashMap;
use std::hash::{BuildHasherDefault, DefaultHasher, Hash, Hasher};
use std::sync::Arc;

use controls::{LiveRampRow, LiveRamps, MonitorOverlay, Motion, Owner, Section};
use effect_compiler::launch_native_effect_registry;
use effect_contract::control::RebuildReason;
use effect_contract::{EffectDescriptor, ParameterUnit, StepUnit};
use protocol::{
    ControlAddress, ControlBody, ControlClass, ControlEntry, ControlFamily, ControlKind,
    ControlListPage, ControlListRequest, ControlOwnerHeader, ControlOwnerHeaderKind,
    ControlOwnerKind, ControlProvider, ControlRampRow, ControlReason, ControlRouteEnds, ControlRow,
    ControlScope, ControlSection, ControlSectionHeader, ControlStatePage, ControlStateRecord,
    ControlTaper, ControllerRetainedCapacity, CounterId, CounterSnapshot, CounterValue,
    CountersRequest, Diagnostic, DiagnosticSeverity, DiagnosticsPage, DiagnosticsRequest,
    EnumChoice, ParameterChannel, ParameterProviderError, ParameterUnit as ProtocolUnit,
    ResponseConfiguration, ResponseCurves, ResponseGetRequest, ResponseGrid, ResponseProviderError,
    ResponseResult, ResponseTarget, SampleTime, StatusCode, TelemetryConfiguration,
    TelemetryCounters, TelemetryRefusal, TelemetryRefusalPath, TransportSetRequest,
    TransportSnapshot, TransportState,
};
use session::{EffectIdentity, SessionModel, StripKind};

use crate::response::{
    ResponseAddressSection, ResponseChannels, ResponseParameterOverride, ResponsePreviewError,
    ResponsePreviewGrid, ResponsePreviewLimits, ResponsePreviewTarget, ResponseQuery,
    ResponseQueryError, ResponseRefusal, check_response_request, contract_link_mode,
    contract_quality, input_filter_target, prepare_response_query, resolve_response_target,
};

/// The counters the provider serves, in ascending ID order (#1351 D1): the two telemetry
/// counters, `live_values_superseded` (#1312 D8), `transition_reprepare_refusals` (D6),
/// `strips_restarted` (#1768), `live_block_errors` (#1521), `source_underrun_blocks` (#1537),
/// `source_frames_discarded` (#1699) and `meter_windows_lost` (#1583 D5). Each reads 0 from
/// construction.
const SERVED_COUNTERS: [CounterId; 9] = [
    CounterId::TelemetryCoalesced,
    CounterId::TelemetryDropped,
    CounterId::LiveValuesSuperseded,
    CounterId::TransitionReprepareRefusals,
    CounterId::StripsRestarted,
    CounterId::LiveBlockErrors,
    CounterId::SourceUnderrunBlocks,
    CounterId::SourceFramesDiscarded,
    CounterId::MeterWindowsLost,
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
/// The list is held in three shared tables and two vectors in step (#1599). `owners` holds each
/// owner once, `sections` each section once with its owner's index, and `descriptors` each
/// distinct control row once (every `f32` compared by bits). `rows` holds one fixed-size record
/// per entry and no heap allocation; a control's record names its section, its control and lane,
/// and its descriptor. `values` holds the committed value of each entry, in step with `rows`.
/// A page builds its wire entries from these tables, so the bytes a client reads are the ones a
/// full wire row per entry would give.
///
/// Handles count from 1 in list order, so an entry's handle is its index plus one and no handle
/// is stored twice. A descriptor's `value` is always `0.0`, so that two catalogs' tables compare
/// equal exactly when every entry's address, header and descriptor do (a page fills the value in
/// from `values`).
pub struct ControlCatalog {
    owners: Vec<OwnerRecord>,
    sections: Vec<SectionRecord>,
    descriptors: Vec<ControlRow>,
    rows: Vec<RowRecord>,
    values: Vec<f32>,
    resources: CatalogResources,
}

/// The catalog of a live commit's committed model, staged before the commit's first write
/// (#1600). [`SessionControlProvider::publish_staged_catalog`] applies it after the commit.
pub enum StagedCatalog {
    /// The catalog differs from the current one in values alone: the writes to apply.
    Values(StagedValues),
    /// The catalog differs in an address, a header or a descriptor, or a setting changed: the whole
    /// catalog, as [`ControlCatalog::build`] gives it.
    Full(ControlCatalog),
}

/// The value writes of a values-only refresh: pairs of an entry index and its new value, in a
/// vector reserved exactly before the commit.
pub struct StagedValues {
    writes: Vec<(u32, f32)>,
}

impl StagedValues {
    /// The number of values this refresh writes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.writes.len()
    }

    /// Whether this refresh writes no value.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.writes.is_empty()
    }
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static STAGED_OWNERS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    static FULL_STAGES: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    static STAGE_FAILURES: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    static VERIFY_STAGES: core::cell::Cell<bool> = const { core::cell::Cell::new(true) };
    static RESPONSE_PREPARATIONS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    static RESPONSE_EVALUATIONS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

/// How many response owners [`SessionControlProvider`] has prepared, and how many it has gone on
/// to evaluate, on this thread since the last call; resets both (#1573 gates 3 and 5). A test
/// shows that a replayed `RESPONSE_GET` prepares none and that a reply too large for the frame is
/// refused before any curve is computed.
#[cfg(any(test, feature = "test-support"))]
pub fn take_response_counts() -> (usize, usize) {
    (
        RESPONSE_PREPARATIONS.with(|count| count.replace(0)),
        RESPONSE_EVALUATIONS.with(|count| count.replace(0)),
    )
}

/// How many changed owners [`ControlCatalog::stage_live`] has read on this thread since the last
/// call, and how many of its results were [`StagedCatalog::Full`]; resets both (#1600 D7). A
/// test shows that one fader edit reads one owner and takes no full catalog.
#[cfg(any(test, feature = "test-support"))]
pub fn take_stage_counts() -> (usize, usize) {
    (
        STAGED_OWNERS.with(|count| count.replace(0)),
        FULL_STAGES.with(|count| count.replace(0)),
    )
}

/// Make the next `count` calls of [`ControlCatalog::stage_live`] on this thread fail, so a test
/// can show that a refusal before the first write commits nothing.
#[cfg(any(test, feature = "test-support"))]
pub fn fail_next_stages(count: usize) {
    STAGE_FAILURES.with(|failures| failures.set(count));
}

/// Whether a values-only [`ControlCatalog::stage_live`] on this thread checks, in a debug build,
/// that it equals a full build of the model it stages (on by default). The check builds the whole
/// catalog, so a test that counts the allocations of a live commit turns it off for its own thread.
#[cfg(any(test, feature = "test-support"))]
pub fn verify_stages(on: bool) {
    VERIFY_STAGES.with(|verify| verify.set(on));
}

/// What a catalog row is.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RowKind {
    /// An owner header; `parent` is the owner's index.
    Owner,
    /// A section header; `parent` is the section's index.
    Section,
    /// A control row; `parent` is its section's index.
    Control,
}

/// One entry of the list, with no heap allocation (16 bytes).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RowRecord {
    /// The owner index of an owner header, the section index of every other row.
    parent: u32,
    /// The control ID of a control row; `0` in a header.
    control: u32,
    /// The index into `descriptors` of a control row; `0` in a header.
    descriptor: u32,
    kind: RowKind,
    /// The lane of a control row; unused in a header.
    lane: ParameterChannel,
}

/// One owner of the list.
#[derive(Clone, Debug, Eq, PartialEq)]
struct OwnerRecord {
    kind: ControlOwnerKind,
    header: ControlOwnerHeaderKind,
    /// The owner's stable ID; absent exactly for the session. It is also the owner header's name
    /// (the session's is `session`).
    id: Option<String>,
    /// A route's ends.
    route: Option<ControlRouteEnds>,
}

impl OwnerRecord {
    fn name(&self) -> &str {
        self.id.as_deref().unwrap_or("session")
    }
}

/// One section of the list.
#[derive(Clone, Debug, Eq, PartialEq)]
struct SectionRecord {
    /// The index of the section's owner.
    owner: u32,
    section: ControlSection,
    /// The console slot or insert ID; it is also the section header's name for those two.
    slot_id: Option<String>,
    /// The native effect identity of a console slot or an insert.
    effect: Option<String>,
}

impl SectionRecord {
    fn name(&self) -> &str {
        match self.section {
            ControlSection::Own => "own",
            ControlSection::Builtins => "builtins",
            ControlSection::Console | ControlSection::Inserts => {
                self.slot_id.as_deref().unwrap_or_default()
            }
        }
    }
}

/// The retained bytes of one catalog and its largest single allocation, the catalog-only part of
/// [`SessionControlProvider::resource_report`] (decision 17, C4). It is computed once, when the
/// catalog is built, so a live commit reads the current catalog's figure without a walk.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CatalogResources {
    /// Bytes the catalog retains: its tables, values, choices, ladders and strings.
    pub retained_bytes: u64,
    /// The largest single allocation among them.
    pub largest_allocation_bytes: u64,
}

impl CatalogResources {
    fn of(
        owners: &[OwnerRecord],
        sections: &[SectionRecord],
        descriptors: &[ControlRow],
        rows: usize,
        values: usize,
    ) -> Result<Self, SessionControlProviderError> {
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
        for owner in owners {
            charge(owner.id.as_deref().unwrap_or_default())?;
            if let Some(route) = &owner.route {
                charge(&route.source)?;
                charge(&route.destination)?;
            }
        }
        for section in sections {
            charge(section.slot_id.as_deref().unwrap_or_default())?;
            charge(section.effect.as_deref().unwrap_or_default())?;
        }
        for row in descriptors {
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
        let tables = [
            bytes::<OwnerRecord>(owners.len())?,
            bytes::<SectionRecord>(sections.len())?,
            bytes::<ControlRow>(descriptors.len())?,
            bytes::<RowRecord>(rows)?,
            bytes::<f32>(values)?,
        ];
        let catalog_retained_bytes = tables
            .into_iter()
            .chain([
                bytes::<EnumChoice>(enum_count)?,
                bytes::<f32>(ladder_count)?,
                u64::try_from(string_bytes).map_err(|_| SessionControlProviderError)?,
            ])
            .try_fold(0_u64, u64::checked_add)
            .ok_or(SessionControlProviderError)?;
        Ok(Self {
            retained_bytes: catalog_retained_bytes,
            largest_allocation_bytes: tables
                .into_iter()
                .chain([largest_enum, largest_ladder, largest_string])
                .max()
                .unwrap_or_default(),
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
    telemetry_configuration: TelemetryConfiguration,
    /// The most reply frame bytes a `RESPONSE_GET` may answer: the smaller of the endpoint's
    /// frame bound and the replay cache's response bound.
    maximum_reply_bytes: usize,
    /// The control half of the session's response capture (#1577 D4).
    response_capture: crate::ResponseCaptureClient,
    /// The meter window in blocks of the host's meter delivery, or `None` while the host has none
    /// (#1583 D4): a browser booted with no meter window (#1588).
    meter_window_blocks: Option<core::num::NonZeroU32>,
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
        maximum_reply_bytes: usize,
        response_capture: crate::ResponseCaptureClient,
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
            telemetry_configuration: retained_telemetry(retained)?,
            maximum_reply_bytes,
            response_capture,
            meter_window_blocks: None,
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

    /// Record the newest plan epoch for the response capture (#1577 D4): at boot and whenever a
    /// replacement plan's epoch is published.
    pub fn set_newest_plan_epoch(&mut self, epoch: u64) {
        self.response_capture.set_newest_plan_epoch(epoch);
    }

    /// Arm one capture of `strip_id`'s response owners ([`crate::ResponseCaptureClient::arm`]).
    ///
    /// # Errors
    ///
    /// The client's [`crate::ResponseCaptureRefusal`]; a refusal changes nothing.
    pub fn arm_response_capture(
        &mut self,
        model: &session::SessionModel,
        strip_id: &str,
        grid: crate::ResponsePreviewGrid,
        channels: crate::ResponseChannels,
    ) -> Result<u64, crate::ResponseCaptureRefusal> {
        self.response_capture.arm(model, strip_id, grid, channels)
    }

    /// The armed capture's result, once ([`crate::ResponseCaptureClient::poll`]).
    pub fn poll_response_capture(
        &mut self,
        model: &session::SessionModel,
    ) -> Option<crate::ResponseReady> {
        self.response_capture.poll(model)
    }

    /// Serve a strip `RESPONSE_GET` (#1578 D3): the core's checks in the core's order (an empty
    /// override list: the codec refuses any), the session-rate rule, the strip, then the bound of
    /// its event (`response_ready_encoded_bound` at the strip's owner count, at most the smaller of
    /// the reply bound and `maximum_reply_bytes`), and only then [`Self::arm_response_capture`]. The
    /// reply is the capture's tag, written after its job is in the render queue; its result arrives
    /// as one `RESPONSE_READY`. A refusal arms nothing.
    fn arm_strip_response(
        &mut self,
        model: &SessionModel,
        strip_id: &str,
        request: &ResponseGetRequest,
    ) -> Result<ResponseResult, ResponseProviderError> {
        let grid = preview_grid(request);
        check_response_request(model.sample_rate_hz, grid, request.overrides.len(), true)
            .map_err(query_refusal)?;
        if request.sample_rate_hz.is_some() || request.quantum_frames.is_some() {
            return Err(refuse(StatusCode::InvalidField, "response.rate.session"));
        }
        let unknown = || refuse(StatusCode::NotFound, "control.address.unknown");
        let owners = crate::response_capture::strip_owners(model, strip_id).ok_or_else(unknown)?;
        let bound = request
            .maximum_reply_bytes
            .map_or(self.maximum_reply_bytes, |bytes| {
                self.maximum_reply_bytes
                    .min(usize::try_from(bytes).unwrap_or(usize::MAX))
            });
        let points = request.points as usize;
        if !protocol::response_ready_encoded_bound(points, request.channels, owners)
            .is_some_and(|length| length <= bound)
        {
            return Err(refuse(
                StatusCode::LimitExceeded,
                "response.reply_too_large",
            ));
        }
        let channels = ResponseChannels::from_raw(request.channels as u8)
            .ok_or_else(|| refuse(StatusCode::Internal, "response.internal"))?;
        let tag = self
            .response_capture
            .arm(model, strip_id, grid, channels)
            .map_err(|refusal| match refusal {
                crate::ResponseCaptureRefusal::UnknownStrip => unknown(),
                crate::ResponseCaptureRefusal::Pending => {
                    refuse(StatusCode::Backpressure, "response.capture.pending")
                }
                crate::ResponseCaptureRefusal::Grid
                    if !(2..=crate::RESPONSE_MAXIMUM_POINTS).contains(&points) =>
                {
                    refuse(StatusCode::InvalidField, "response.points")
                }
                crate::ResponseCaptureRefusal::Grid => {
                    refuse(StatusCode::InvalidField, "response.grid")
                }
            })?;
        Ok(ResponseResult::Tag(tag))
    }

    /// End a delivered capture ([`crate::ResponseCaptureClient::release`]).
    pub fn release_response_capture(&mut self) {
        self.response_capture.release();
    }

    /// Publish one fully prepared catalog, committed as session revision `revision`, without
    /// allocation or failure (the old one drops here, on the control thread). A live commit
    /// publishes the catalog of the committed model as well, which refreshes every value.
    ///
    /// The structure epoch advances by one when the new catalog's entries differ from the current
    /// ones in anything but a value: an address, a header or any control row's descriptor (a
    /// `matrix_mode` switch flips `active` on live). A client therefore never holds a stale
    /// descriptor under a current epoch. A commit that changes only values keeps the epoch.
    ///
    /// The comparison reads the tables, every `f32` by bits. Both catalogs fill `descriptors` in
    /// order of first use, so their tables are equal exactly when every entry's address, header
    /// and descriptor are.
    pub fn replace_session_catalog(&mut self, catalog: ControlCatalog, revision: u64) {
        if !self.catalog.same_structure(&catalog) {
            self.structure_epoch = self.structure_epoch.saturating_add(1);
        }
        self.catalog = catalog;
        self.revision = revision;
    }

    /// The staged catalog of a live commit's model, read before the commit's first write
    /// ([`ControlCatalog::stage_live`] on the catalog the provider holds).
    ///
    /// # Errors
    ///
    /// [`SessionControlProviderError`] as [`ControlCatalog::stage_live`].
    pub fn stage_live_catalog(
        &self,
        next: controls::ModelView<'_>,
        overlay: &MonitorOverlay,
        sample_rate: u32,
        delta: &crate::LiveDelta<'_>,
    ) -> Result<StagedCatalog, SessionControlProviderError> {
        self.catalog.stage_live(next, overlay, sample_rate, delta)
    }

    /// Publish a catalog staged by [`Self::stage_live_catalog`], committed as session revision
    /// `revision`, without allocation or failure. A values-only stage writes its values and keeps
    /// the structure epoch; a full catalog replaces the current one as
    /// [`Self::replace_session_catalog`] does, and so advances the epoch exactly when an address,
    /// a header or a descriptor differs.
    pub fn publish_staged_catalog(&mut self, staged: StagedCatalog, revision: u64) {
        match staged {
            StagedCatalog::Values(staged) => {
                for (index, value) in staged.writes {
                    self.catalog.values[index as usize] = value;
                }
                self.revision = revision;
            }
            StagedCatalog::Full(catalog) => self.replace_session_catalog(catalog, revision),
        }
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
    /// list ends with it. Entries are contiguous per owner, so an owner filter is one run. The
    /// page holds the rows of the range that the request serves ([`serves`]): every row, or with
    /// `headers_only` the header rows alone (#2206), and `limit` counts those rows.
    fn window(
        &self,
        request: &ControlListRequest,
    ) -> Result<(usize, usize, bool), ParameterProviderError> {
        if request.structure_epoch != 0 && request.structure_epoch != self.structure_epoch {
            return Err(ParameterProviderError::Stale);
        }
        let rows = &self.catalog.rows;
        // Handle `h` is entry `h - 1`, so the entries after the cursor start at index `h`.
        let from = usize::try_from(request.after_handle)
            .unwrap_or(usize::MAX)
            .min(rows.len());
        let owner = match &request.owner {
            None => None,
            Some(filter) => match self.catalog.owner_index(filter) {
                Some(owner) => Some(owner),
                None => return Ok((rows.len(), rows.len(), true)),
            },
        };
        let owned =
            |record: &RowRecord| owner.is_none_or(|owner| self.catalog.owner_of(record) == owner);
        let start = rows[from..]
            .iter()
            .position(owned)
            .map_or(rows.len(), |offset| from + offset);
        let limit = usize::from(request.limit);
        let (mut next, mut end, mut served) = (start, start, 0);
        // The owner's run, or the rest of the list, up to the row after the page's last served row;
        // the scan stops at a served row past the limit, so the list ends exactly when the run does.
        while next < rows.len() && owned(&rows[next]) {
            if serves(request, &rows[next]) {
                if served == limit {
                    return Ok((start, end, false));
                }
                served += 1;
                end = next + 1;
            }
            next += 1;
        }
        Ok((start, end, true))
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

    /// Record the session's `strips_restarted` count (#1768): for every plan render adopted, the
    /// strips it started at rest ([`crate::PlanStateInventory::restarted_strips`]). The control
    /// plane copies it in beside `live_values_superseded`.
    pub fn set_strips_restarted(&mut self, count: u64) {
        set_counter(
            &mut self.counter_snapshot,
            CounterId::StripsRestarted,
            count,
        );
    }

    /// Record the session's `transition_reprepare_refusals` count (#1397 D2 step 3): the
    /// transition re-preparations across a withdrawn donor that were refused, each a defect. The
    /// control plane copies it in beside `strips_restarted`; it stays 0 in a correct build.
    pub fn set_transition_reprepare_refusals(&mut self, count: u64) {
        set_counter(
            &mut self.counter_snapshot,
            CounterId::TransitionReprepareRefusals,
            count,
        );
    }

    /// Record the session's `live_block_errors` count (#1521): the blocks render refused after
    /// the block took its live snapshot, over every plan the session adopted. The control plane
    /// copies the render-side mirror in beside `strips_restarted`; it stays 0 in a correct build.
    pub fn set_live_block_errors(&mut self, count: u64) {
        set_counter(
            &mut self.counter_snapshot,
            CounterId::LiveBlockErrors,
            count,
        );
    }

    /// Record the session's `source_underrun_blocks` count (#1537 D1): the blocks render rendered
    /// in which a source played zeros because the host had not delivered PCM it owed (graph-wide,
    /// not per source), over every plan the session adopted. The control plane copies the
    /// render-side mirror in beside `live_block_errors`.
    pub fn set_source_underrun_blocks(&mut self, count: u64) {
        set_counter(
            &mut self.counter_snapshot,
            CounterId::SourceUnderrunBlocks,
            count,
        );
    }

    /// Record the session's `source_frames_discarded` count (#1699 D1): the accepted source
    /// frames render never read because the caller's own transaction removed or redeclared the
    /// source, summed when the control plane reclaims each retired plan.
    pub fn set_source_frames_discarded(&mut self, count: u64) {
        set_counter(
            &mut self.counter_snapshot,
            CounterId::SourceFramesDiscarded,
            count,
        );
    }

    /// Record the session's lost meter windows (#1583 D5): the windows the meter delivery lost
    /// and the partial batches its fold discarded, none of which reached a `METER_BATCH`.
    pub fn set_meter_windows_lost(&mut self, count: u64) {
        set_counter(
            &mut self.counter_snapshot,
            CounterId::MeterWindowsLost,
            count,
        );
    }

    /// Give the provider the host's meter delivery window in blocks (#1583 D4, R2): a host with a
    /// meter delivery sets it once at boot; until then the provider has none and refuses any meter
    /// entry.
    pub fn set_meter_window_blocks(&mut self, blocks: Option<core::num::NonZeroU32>) {
        self.meter_window_blocks = blocks;
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
                meters: self.telemetry_configuration.meters.capacity(),
                counter_ids: self.telemetry_configuration.counter_ids.capacity(),
            },
            self.counter_snapshot.capacity(),
        )
    }
}

fn refuse(status: StatusCode, code: &'static str) -> ResponseProviderError {
    ResponseProviderError { status, code }
}

/// A query refusal as the provider reports it, by the one refusal table of the core.
fn query_refusal(error: ResponseQueryError) -> ResponseProviderError {
    let (class, code) = error.refusal();
    let status = match class {
        ResponseRefusal::InvalidField => StatusCode::InvalidField,
        ResponseRefusal::LimitExceeded => StatusCode::LimitExceeded,
        ResponseRefusal::NotFound => StatusCode::NotFound,
        ResponseRefusal::Unavailable => StatusCode::Unavailable,
        ResponseRefusal::ValidationFailed => StatusCode::ValidationFailed,
        ResponseRefusal::Internal => StatusCode::Internal,
    };
    refuse(status, code)
}

fn contract_channel(lane: ParameterChannel) -> effect_contract::ParameterChannel {
    match lane {
        ParameterChannel::Left => effect_contract::ParameterChannel::Left,
        ParameterChannel::Right => effect_contract::ParameterChannel::Right,
        ParameterChannel::Both => effect_contract::ParameterChannel::Both,
    }
}

/// What a `RESPONSE_GET` that passed [`response_preflight`] is computed for.
#[derive(Clone, Copy)]
enum CheckedTarget<'a> {
    /// An explicit configuration, from the request alone.
    Configuration(&'a ResponseConfiguration),
    /// A control address of the session's own effect or input filters (#1575).
    Address(&'a ControlAddress),
}

/// The checks every `RESPONSE_GET` meets first, whatever the endpoint: the target is a
/// configuration or an address (a session provider serves the strip target before this, #1578
/// D3, so here it is `response.target.unavailable`: the session-less entry has no strip), then the
/// core's own checks in the core's order (rate, points, override count, axis, override values), so
/// the refusal of a request that is wrong in several ways is the same on every host. It returns the
/// target the request names and its grid.
fn response_preflight(
    sample_rate_hz: u32,
    request: &ResponseGetRequest,
) -> Result<(CheckedTarget<'_>, ResponsePreviewGrid), ResponseProviderError> {
    let target = match &request.target {
        ResponseTarget::Configuration(configuration) => CheckedTarget::Configuration(configuration),
        ResponseTarget::Address(address) => CheckedTarget::Address(address),
        ResponseTarget::Strip(_) => {
            return Err(refuse(
                StatusCode::Unavailable,
                "response.target.unavailable",
            ));
        }
    };
    let grid = preview_grid(request);
    check_response_request(
        sample_rate_hz,
        grid,
        request.overrides.len(),
        request.overrides.iter().all(|row| row.value.is_finite()),
    )
    .map_err(query_refusal)?;
    Ok((target, grid))
}

/// The axis a `RESPONSE_GET` asks for.
fn preview_grid(request: &ResponseGetRequest) -> ResponsePreviewGrid {
    let points = request.points as usize;
    match request.grid {
        ResponseGrid::Linear => ResponsePreviewGrid::Linear {
            points,
            minimum_hz: request.minimum_hz,
            maximum_hz: request.maximum_hz,
        },
        ResponseGrid::Logarithmic => ResponsePreviewGrid::Logarithmic {
            points,
            minimum_hz: request.minimum_hz,
            maximum_hz: request.maximum_hz,
        },
    }
}

/// The `RESPONSE_READY` event of one delivered capture (#1578 D5): a track response fills the
/// capture's fields; a failure is its status (`NOT_FOUND` for a removed strip, `UNAVAILABLE` for an
/// owner without a response, `LIMIT_EXCEEDED` for a capacity bound, `INTERNAL` otherwise) and its
/// code. The sequence is the controller's to write; it moves every buffer and copies none.
#[must_use]
pub fn response_ready_event(ready: crate::ResponseReady) -> protocol::ResponseReady {
    let outcome = match ready.result {
        Ok(response) => protocol::ResponseReadyOutcome::Ready(protocol::TrackResponseCapture {
            captured_sample: response.captured_sample,
            sample_rate_hz: response.sample_rate_hz,
            floor_db: response.floor_db,
            frequencies_hz: response.frequencies_hz,
            left_db: response.left_db,
            right_db: response.right_db,
            excluded_count: response.excluded_count,
            members: response
                .members
                .into_iter()
                .map(|member| protocol::ResponseMember {
                    section: match member.section {
                        crate::ResponseMemberSection::Builtins => {
                            protocol::ResponseMemberSection::Builtins
                        }
                        crate::ResponseMemberSection::Console(id) => {
                            protocol::ResponseMemberSection::Console(id.into_string())
                        }
                        crate::ResponseMemberSection::Inserts(id) => {
                            protocol::ResponseMemberSection::Inserts(id.into_string())
                        }
                    },
                    effect: member.native_id.into_string(),
                    kind: match member.kind {
                        crate::ResponseMemberKind::ParametricEq => {
                            protocol::ResponseMemberKind::ParametricEq
                        }
                        crate::ResponseMemberKind::InputFilters => {
                            protocol::ResponseMemberKind::InputFilters
                        }
                        crate::ResponseMemberKind::Unavailable => {
                            protocol::ResponseMemberKind::Unavailable
                        }
                    },
                    bypassed: member.bypassed,
                    enabled_left: member.enabled_left,
                    enabled_right: member.enabled_right,
                    section_count: member.section_count,
                })
                .collect(),
        }),
        Err(failure) => protocol::ResponseReadyOutcome::Refused {
            status: match failure {
                crate::ResponseCaptureFailure::StripRemoved => StatusCode::NotFound,
                crate::ResponseCaptureFailure::Unsupported => StatusCode::Unavailable,
                crate::ResponseCaptureFailure::Capacity => StatusCode::LimitExceeded,
                crate::ResponseCaptureFailure::Internal => StatusCode::Internal,
            },
            diagnostic: failure.code().to_owned(),
        },
    };
    protocol::ResponseReady {
        sequence: 0,
        request_tag: ready.tag,
        strip_id: ready.strip_id.into_string(),
        outcome,
    }
}

/// Serve one `RESPONSE_GET` for a session endpoint: the rate and quantum are the committed
/// model's, and a request that names its own is refused. An address target resolves against the
/// committed model.
fn serve_response(
    maximum_reply_bytes: usize,
    model: &SessionModel,
    request: &ResponseGetRequest,
) -> Result<ResponseResult, ResponseProviderError> {
    serve_response_at(
        model.sample_rate_hz,
        model.quantum_frames,
        maximum_reply_bytes,
        request,
        Some(model),
    )
}

/// Serve one `RESPONSE_GET` at a rate and a quantum the caller resolved: [`response_preflight`],
/// then the rate rule, then [`serve_checked`], so the refusal of a request that is wrong in
/// several ways is the same for every endpoint. This function reads neither
/// `request.sample_rate_hz` nor `request.quantum_frames` for its curves.
///
/// `session` is the committed model of an endpoint that has one rate and one quantum, the
/// session's: a request that names its own is then refused after the preflight
/// (`response.rate.session`), and an address target resolves against it. An endpoint that has no
/// session (#1574) passes `None` and has resolved the rate and the quantum from the request itself;
/// it has no address to resolve (`response.target.unavailable`).
pub(crate) fn serve_response_at(
    sample_rate_hz: u32,
    quantum_frames: u32,
    maximum_reply_bytes: usize,
    request: &ResponseGetRequest,
    session: Option<&SessionModel>,
) -> Result<ResponseResult, ResponseProviderError> {
    let (target, grid) = response_preflight(sample_rate_hz, request)?;
    if session.is_some() && (request.sample_rate_hz.is_some() || request.quantum_frames.is_some()) {
        return Err(refuse(StatusCode::InvalidField, "response.rate.session"));
    }
    serve_checked(
        sample_rate_hz,
        quantum_frames,
        maximum_reply_bytes,
        request,
        target,
        grid,
        session,
    )
}

/// The most payload bytes a session-less reply may carry: the bound the browser preview has
/// always had, lowered by the request's own `maximum_reply_bytes`.
const SESSIONLESS_REPLY_BYTES: usize = 16 << 20;

/// The most request frame bytes a session-less endpoint decodes.
const SESSIONLESS_REQUEST_FRAME_BYTES: usize = 1 << 20;

/// Why [`serve_response_frame`] wrote no reply.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseFrameError {
    /// The command header did not decode: the input names no request, so a reply could not be
    /// correlated.
    Malformed,
    /// The request was understood but not even the non-OK `INTERNAL` frame could be encoded, so
    /// `reply` is empty. The host reports an internal failure and never an empty success.
    Internal,
}

/// Serve one command frame without a session (#1574): the browser's response preview, which has no
/// session, sends the frame a C ABI host sends and reads the reply it reads.
///
/// The frame is decoded with a limit of 1 MiB. A frame whose header does not decode is
/// [`ResponseFrameError::Malformed`] and writes nothing. Any message but `RESPONSE_GET` is
/// answered with a non-OK `UNSUPPORTED_MESSAGE` frame, and a payload that does not decode with
/// the status of its decode error. A `RESPONSE_GET` needs a configuration target
/// (`response.target.unavailable` otherwise: an address names a session effect, and there is no
/// session) and both `sample_rate_hz` and `quantum_frames`
/// (`response.rate` otherwise); it then runs `serve_response_at`, the function a session
/// endpoint runs, so the reply payload equals a session endpoint's for the same request. The
/// reply echoes the request ID with revision 0. `reply` is cleared, then holds one complete
/// frame, OK or non-OK.
///
/// # Errors
///
/// [`ResponseFrameError::Malformed`] when the command header does not decode, and
/// [`ResponseFrameError::Internal`] when no reply frame, not even a non-OK one, could be encoded
/// (`reply` is then empty).
pub fn serve_response_frame(request: &[u8], reply: &mut Vec<u8>) -> Result<(), ResponseFrameError> {
    let decode_codec = protocol::ProtocolCodec::new(protocol::ProtocolLimits {
        max_frame_bytes: SESSIONLESS_REQUEST_FRAME_BYTES,
        ..protocol::ProtocolLimits::default()
    });
    reply.clear();
    let header = decode_codec
        .decode_command_header(request)
        .map_err(|_| ResponseFrameError::Malformed)?;
    // The reply may exceed the request's frame limit, so it has its own codec.
    let encode_codec = protocol::ProtocolCodec::new(protocol::ProtocolLimits {
        max_frame_bytes: protocol::OUTER_HEADER_BYTES + SESSIONLESS_REPLY_BYTES,
        ..protocol::ProtocolLimits::default()
    });
    let refused = |status: StatusCode, code: &str, reply: &mut Vec<u8>| {
        write_non_ok(&encode_codec, header, status, code, reply)
    };
    if header.message_id != protocol::MessageId::ResponseGet {
        return refused(StatusCode::UnsupportedMessage, "protocol.failure", reply);
    }
    let mut fields = [0_u16; 1024];
    let decoded = match decode_codec
        .decode_typed_command(request, &mut protocol::DecodeScratch::new(&mut fields))
    {
        Ok(decoded) => decoded,
        Err(error) => return refused(error.status(), "protocol.failure", reply),
    };
    let protocol::DecodedCommandPayload::ResponseGet(query) = decoded.payload else {
        return refused(StatusCode::Internal, "protocol.failure", reply);
    };
    let outcome = match (&query.target, query.sample_rate_hz, query.quantum_frames) {
        (ResponseTarget::Configuration(_), Some(rate), Some(quantum)) => {
            serve_response_at(rate, quantum, SESSIONLESS_REPLY_BYTES, &query, None)
        }
        (ResponseTarget::Configuration(_), _, _) => {
            Err(refuse(StatusCode::InvalidField, "response.rate"))
        }
        _ => Err(refuse(
            StatusCode::Unavailable,
            "response.target.unavailable",
        )),
    };
    match outcome {
        Ok(result) => write_response_frame(&encode_codec, header, &result, reply),
        Err(error) => refused(error.status, error.code, reply),
    }
}

/// Encode a `RESPONSE_GET` success frame; a result the codec cannot encode is `INTERNAL`, and an
/// `INTERNAL` frame that cannot be encoded either is [`ResponseFrameError::Internal`].
fn write_response_frame(
    codec: &protocol::ProtocolCodec,
    header: protocol::CommandHeader,
    result: &ResponseResult,
    reply: &mut Vec<u8>,
) -> Result<(), ResponseFrameError> {
    let frame = protocol::TypedSuccessResponseFrame {
        request_id: header.request_id,
        revision: protocol::SessionRevision(0),
        payload: protocol::SuccessResponsePayload::ResponseResult(result.clone()),
    };
    let Ok(payload) = codec.encoded_response_result_len(result) else {
        return write_non_ok(
            codec,
            header,
            StatusCode::Internal,
            "response.internal",
            reply,
        );
    };
    let mut capacity = protocol::OUTER_HEADER_BYTES.saturating_add(payload);
    for _ in 0..2 {
        if reply.try_reserve_exact(capacity).is_err() {
            return write_non_ok(
                codec,
                header,
                StatusCode::Internal,
                "response.internal",
                reply,
            );
        }
        reply.resize(capacity, 0);
        match codec.encode_success_response_frame_into(&frame, reply) {
            Ok(written) => {
                reply.truncate(written);
                return Ok(());
            }
            Err(protocol::EncodeError::OutputTooSmall { required }) => capacity = required,
            Err(_) => break,
        }
    }
    reply.clear();
    write_non_ok(
        codec,
        header,
        StatusCode::Internal,
        "response.internal",
        reply,
    )
}

/// Encode a non-OK reply that carries one error diagnostic, echoing the request's ID and message.
/// When the frame cannot be encoded `reply` is left empty and the error is
/// [`ResponseFrameError::Internal`].
fn write_non_ok(
    codec: &protocol::ProtocolCodec,
    header: protocol::CommandHeader,
    status: StatusCode,
    code: &str,
    reply: &mut Vec<u8>,
) -> Result<(), ResponseFrameError> {
    let payload = protocol::NonOkResponse {
        diagnostics: vec![Diagnostic {
            code: code.to_owned(),
            severity: DiagnosticSeverity::Error,
            path: Vec::new(),
            detail: None,
            operation_index: None,
            sample_time: None,
            provider_sequence: None,
        }],
        omitted_diagnostics: 0,
        backpressure: None,
    };
    let frame = protocol::TypedNonOkResponseFrame {
        request_id: header.request_id,
        revision: protocol::SessionRevision(0),
        message_id: header.message_id,
        status,
        payload: &payload,
    };
    reply.clear();
    reply.resize(protocol::OUTER_HEADER_BYTES + 512, 0);
    let Ok(written) = codec.encode_non_ok_response_frame_into(&frame, reply) else {
        reply.clear();
        return Err(ResponseFrameError::Internal);
    };
    reply.truncate(written);
    Ok(())
}

/// Everything after the rate rules: the limits, the target (a configuration's bypass rows, or an
/// address resolved against the committed model), then [`serve_target`]. The request has passed
/// [`response_preflight`], which returned its target and grid.
fn serve_checked(
    sample_rate_hz: u32,
    quantum_frames: u32,
    maximum_reply_bytes: usize,
    request: &ResponseGetRequest,
    target: CheckedTarget<'_>,
    grid: ResponsePreviewGrid,
    session: Option<&SessionModel>,
) -> Result<ResponseResult, ResponseProviderError> {
    let defaults = ResponsePreviewLimits::default();
    let maximum_prepared_bytes = request
        .maximum_prepared_bytes
        .map_or(defaults.maximum_prepared_bytes, |bytes| {
            usize::try_from(bytes).unwrap_or(usize::MAX)
        });
    if maximum_prepared_bytes == 0 {
        return Err(refuse(StatusCode::LimitExceeded, "response.limit"));
    }
    let limits = ResponsePreviewLimits {
        maximum_prepared_bytes,
        maximum_total_state_bytes: request
            .maximum_state_bytes
            .unwrap_or(defaults.maximum_total_state_bytes),
        maximum_scratch_bytes: request
            .maximum_scratch_bytes
            .unwrap_or(defaults.maximum_scratch_bytes),
        maximum_automation_spans_per_block: request
            .maximum_automation_spans
            .unwrap_or(defaults.maximum_automation_spans_per_block),
    };
    let serve = |target: ResponsePreviewTarget<'_>| {
        serve_target(
            sample_rate_hz,
            quantum_frames,
            maximum_reply_bytes,
            request,
            limits,
            target,
            grid,
        )
    };
    let configuration = match target {
        CheckedTarget::Configuration(configuration) => configuration,
        CheckedTarget::Address(address) => {
            let Some(model) = session else {
                return Err(refuse(
                    StatusCode::Unavailable,
                    "response.target.unavailable",
                ));
            };
            let (strip, section) = response_address(address).map_err(query_refusal)?;
            // An address override names a control of the addressed section by its row ID: the
            // bypass, the link-mode row, a parameter, or a cutoff; the resolver checks each as a
            // control write.
            let mut rows = Vec::new();
            rows.try_reserve_exact(request.overrides.len())
                .map_err(|_| refuse(StatusCode::Internal, "response.internal"))?;
            rows.extend(
                request
                    .overrides
                    .iter()
                    .map(|row| ResponseParameterOverride {
                        parameter_id: row.control,
                        channel: contract_channel(row.lane),
                        value: row.value,
                    }),
            );
            let resolved = resolve_response_target(model, strip, section, &rows, sample_rate_hz)
                .map_err(query_refusal)?;
            return serve(resolved.target());
        }
    };
    // The bypass (control 0) is the effect-wide switch; every other row is a parameter row.
    let mut bypass = None;
    let mut rows = Vec::new();
    rows.try_reserve_exact(request.overrides.len())
        .map_err(|_| refuse(StatusCode::Internal, "response.internal"))?;
    for row in &request.overrides {
        let is_bypass = matches!(configuration, ResponseConfiguration::Effect { .. })
            && row.control == protocol::RESPONSE_BYPASS_CONTROL;
        if !is_bypass {
            rows.push(ResponseParameterOverride {
                parameter_id: row.control,
                channel: contract_channel(row.lane),
                value: row.value,
            });
            continue;
        }
        if row.lane != ParameterChannel::Both {
            return Err(query_refusal(ResponseQueryError::Preview(
                ResponsePreviewError::ConflictingChannel,
            )));
        }
        if bypass.is_some() {
            return Err(query_refusal(ResponseQueryError::Preview(
                ResponsePreviewError::DuplicateParameter,
            )));
        }
        // The bypass is a switch: its value is exactly 0 or 1, and a value between is its own
        // refusal, not "no such parameter".
        bypass = Some(if row.value.to_bits() == 1.0_f32.to_bits() {
            true
        } else if row.value.to_bits() == 0.0_f32.to_bits() {
            false
        } else {
            return Err(refuse(StatusCode::InvalidField, "response.override.bypass"));
        });
    }
    let target = match configuration {
        ResponseConfiguration::Effect {
            effect,
            quality,
            link_mode,
        } => ResponsePreviewTarget::Effect {
            effect_id: effect,
            overrides: &rows,
            quality: contract_quality(*quality),
            bypass: bypass.unwrap_or(false),
            link_mode: contract_link_mode(*link_mode),
        },
        ResponseConfiguration::InputFilters => input_filter_target(&rows)
            .map_err(|error| query_refusal(ResponseQueryError::Preview(error)))?,
    };
    serve(target)
}

/// The strip and the section a response address names (#1575 D1): the owner is a strip and the
/// section its builtins, a console slot or an insert, with `control` and `lane` absent; anything
/// else is [`ResponseQueryError::Address`].
fn response_address(
    address: &ControlAddress,
) -> Result<(&str, ResponseAddressSection<'_>), ResponseQueryError> {
    let (ControlOwnerKind::Strip, Some(strip), None, None) = (
        address.owner_kind,
        address.owner_id.as_deref(),
        address.control,
        address.lane,
    ) else {
        return Err(ResponseQueryError::Address);
    };
    let section = match (address.section, address.slot_id.as_deref()) {
        (ControlSection::Builtins, None) => ResponseAddressSection::Builtins,
        (ControlSection::Console, Some(slot)) => ResponseAddressSection::Console(slot),
        (ControlSection::Inserts, Some(insert)) => ResponseAddressSection::Inserts(insert),
        _ => return Err(ResponseQueryError::Address),
    };
    Ok((strip, section))
}

/// The part every target shares: the preparation, the reply bound and the curves.
fn serve_target(
    sample_rate_hz: u32,
    quantum_frames: u32,
    maximum_reply_bytes: usize,
    request: &ResponseGetRequest,
    limits: ResponsePreviewLimits,
    target: ResponsePreviewTarget<'_>,
    grid: ResponsePreviewGrid,
) -> Result<ResponseResult, ResponseProviderError> {
    let channels = ResponseChannels::from_raw(request.channels as u8)
        .ok_or_else(|| refuse(StatusCode::Internal, "response.internal"))?;
    #[cfg(any(test, feature = "test-support"))]
    RESPONSE_PREPARATIONS.with(|count| count.set(count.get() + 1));
    let prepared = prepare_response_query(&ResponseQuery {
        configuration_id: request.configuration_id.unwrap_or(0),
        sample_rate_hz,
        quantum_frames,
        target,
        grid,
        channels,
        sections: request.sections,
        limits,
    })
    .map_err(query_refusal)?;
    // The reply is sized from its shape, before any curve is computed.
    let bound = request
        .maximum_reply_bytes
        .map_or(maximum_reply_bytes, |bytes| {
            maximum_reply_bytes.min(usize::try_from(bytes).unwrap_or(usize::MAX))
        });
    let fits = protocol::response_result_encoded_len(
        prepared.points(),
        request.channels,
        prepared.section_count(),
        request.sections,
    )
    .is_some_and(|length| length <= bound);
    if !fits {
        return Err(refuse(
            StatusCode::LimitExceeded,
            "response.reply_too_large",
        ));
    }
    #[cfg(any(test, feature = "test-support"))]
    RESPONSE_EVALUATIONS.with(|count| count.set(count.get() + 1));
    let curves = prepared.curves().map_err(query_refusal)?;
    Ok(ResponseResult::Curves(ResponseCurves {
        sample_rate_hz: curves.sample_rate_hz,
        configuration_id: curves.configuration_id,
        floor_db: curves.floor_db,
        bypass: curves.bypass,
        section_count: curves.section_count,
        enabled_left: curves.enabled_left,
        enabled_right: curves.enabled_right,
        retained_bytes: curves.retained_bytes,
        frequencies_hz: curves.frequencies_hz,
        left_db: curves.left_db,
        right_db: curves.right_db,
        sections_left_db: curves.sections_left_db,
        sections_right_db: curves.sections_right_db,
    }))
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
            .try_reserve_exact(served_bound(request, start, end))
            .map_err(|_| ParameterProviderError::Unavailable)?;
        for index in start..end {
            if serves(request, &self.catalog.rows[index]) {
                entries.push(self.catalog.entry(index));
            }
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
            .try_reserve_exact(if request.headers_only { 0 } else { end - start })
            .map_err(|_| ParameterProviderError::Unavailable)?;
        for index in start..end {
            // A headers-only page covers header rows alone, which have no record.
            if self.catalog.rows[index].kind == RowKind::Control && !request.headers_only {
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

    fn response_get(
        &mut self,
        model: &SessionModel,
        request: &ResponseGetRequest,
    ) -> Result<ResponseResult, ResponseProviderError> {
        if let ResponseTarget::Strip(strip_id) = &request.target {
            return self.arm_strip_response(model, strip_id, request);
        }
        serve_response(self.maximum_reply_bytes, model, request)
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
            effective_sample: self.current_sample(),
        }
    }

    fn transport_set(&mut self, request: TransportSetRequest) -> TransportSnapshot {
        self.transport_state = request.state;
        self.transport_get()
    }

    fn telemetry_configure(
        &mut self,
        configuration: TelemetryConfiguration,
    ) -> Result<TelemetryConfiguration, TelemetryRefusal> {
        // A counter this provider does not serve can never report; refuse it before anything
        // changes (#1351 D2).
        if configuration
            .counter_ids
            .iter()
            .any(|id| !SERVED_COUNTERS.contains(id))
        {
            return Err(ParameterProviderError::NotFound.into());
        }
        refuse_meter_table(&configuration, self.meter_window_blocks)?;
        self.telemetry_configuration
            .meters
            .clone_from(&configuration.meters);
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

    fn meter_window_blocks(&self) -> Option<core::num::NonZeroU32> {
        self.meter_window_blocks
    }
}

/// The most records one `METER_BATCH` carries (the wire's bound): a table whose entries need more
/// is refused, so every batch of an accepted table encodes whole (#1587a).
pub const MAXIMUM_METER_BATCH_RECORDS: usize = 256;

/// The meter table rules this provider owns (#1583 D3, #1587a), in order: with no meter delivery
/// any entry is `meter.unavailable`; the period must be a positive multiple of the window
/// (`meter.period`); and the entry past half of [`MAXIMUM_METER_BATCH_RECORDS`] is
/// `meter.records`: an entry has at most two records (a peak entry's left and right; a gain
/// reduction entry has one), so a batch of an accepted table never needs more, and the session's
/// stored tables are charged for that many entries (#1587a, #1587b). An empty table is always
/// served.
fn refuse_meter_table(
    configuration: &TelemetryConfiguration,
    window: Option<core::num::NonZeroU32>,
) -> Result<(), TelemetryRefusal> {
    if configuration.meters.is_empty() {
        return Ok(());
    }
    let refuse = |error, code, path| {
        Err(TelemetryRefusal {
            error,
            code: Some(code),
            path,
        })
    };
    let Some(window) = window else {
        return refuse(
            ParameterProviderError::Unavailable,
            "meter.unavailable",
            TelemetryRefusalPath::Meter(0),
        );
    };
    if configuration.meter_period_blocks == 0
        || !configuration
            .meter_period_blocks
            .is_multiple_of(window.get())
    {
        return refuse(
            ParameterProviderError::InvalidField,
            "meter.period",
            TelemetryRefusalPath::MeterPeriod,
        );
    }
    // At most two records per entry, the output's or a strip's.
    if configuration.meters.len() > MAXIMUM_METER_BATCH_RECORDS / 2 {
        return refuse(
            ParameterProviderError::InvalidField,
            "meter.records",
            TelemetryRefusalPath::Meter(MAXIMUM_METER_BATCH_RECORDS / 2),
        );
    }
    Ok(())
}

/// The most owners a staged live commit reads before it builds a
/// [`controls::AutomationIndex`] (#1710). Each owner's rows scan the automation table a few
/// times; the index is one pass plus its map, so it pays only past a few owners. A
/// 1024-strip, one-owner fader stage measured about 4 % slower with the index built. The live
/// classifier's diffs index past the same count of reads (#1844).
pub(crate) const STAGE_INDEX_OWNERS: usize = 4;

impl ControlCatalog {
    /// Build the catalog of `model` at `sample_rate`: one `controls::list` pass with the owner and
    /// section headers inserted, every allocation reserved up front.
    ///
    /// The derived rows show the effective values under `overlay`, and a strip's `solo` row shows
    /// whether the overlay solos it (decision 17, C4 and C8, #1559 D11).
    pub fn build(
        model: &SessionModel,
        overlay: &MonitorOverlay,
        sample_rate: u32,
    ) -> Result<Self, SessionControlProviderError> {
        Self::build_with(model, overlay, sample_rate, true)
    }

    /// The [`resources`](Self::resources) of the catalog [`Self::build`] builds, before it is
    /// built (issue #2199): the same walk, which keeps the owner, section and descriptor tables
    /// (one entry per owner, per section and per distinct descriptor) but neither allocates nor
    /// fills the row and value tables (one entry per control row), counting them instead.
    ///
    /// # Errors
    ///
    /// As [`Self::build`].
    pub fn project(
        model: &SessionModel,
        overlay: &MonitorOverlay,
        sample_rate: u32,
    ) -> Result<CatalogResources, SessionControlProviderError> {
        Self::build_with(model, overlay, sample_rate, false).map(|catalog| catalog.resources)
    }

    /// [`Self::build`] when `fill_rows`; with the row and value tables left empty and counted
    /// when not ([`Self::project`]).
    fn build_with(
        model: &SessionModel,
        overlay: &MonitorOverlay,
        sample_rate: u32,
        fill_rows: bool,
    ) -> Result<Self, SessionControlProviderError> {
        // #1709 D1: the derived rows are composed as preparation composes them, in the
        // canonical floating-point environment, so a row shows the value render applies.
        let _canonical = lane::CanonicalFpEnv::enter();
        // The list is walked one owner at a time through one reused buffer, twice: once to size
        // every table, once to fill them. No pass holds the whole list, so the transient is one
        // owner's rows, not every control of the session's rows at full size (#1400b).
        let effective = controls::effective(model, overlay);
        // The automation table is walked once, here, and every owner looks its driving entries up
        // (#1710). The index is O(entries) and freed with the build, as the scratch is.
        let automation = controls::AutomationIndex::of(model);
        let view = controls::ModelView::whole(model).indexed(&automation);
        let owner_total = controls::owner_count(model);
        let mut scratch: Vec<controls::ControlRow> = Vec::new();
        let mut listed_count = 0_usize;
        // Every owner opens with a header and every section of it with another.
        let mut owner_count = 0_usize;
        let mut section_count = 0_usize;
        for owner in 0..owner_total {
            scratch.clear();
            controls::owner_rows_in(view, &effective, overlay, owner, sample_rate, &mut scratch);
            // An owner's rows are contiguous and no two owners share an address, so the first row
            // of an owner's run is the one that opens it.
            let mut previous: Option<&controls::ControlAddress> = None;
            for row in &scratch {
                let (owner_opens, section_opens) = openings(previous, &row.address);
                owner_count += usize::from(owner_opens);
                section_count += usize::from(section_opens);
                previous = Some(&row.address);
            }
            listed_count += scratch.len();
        }
        let total = listed_count
            .checked_add(owner_count)
            .and_then(|total| total.checked_add(section_count))
            .filter(|total| u32::try_from(*total).is_ok())
            .ok_or(SessionControlProviderError)?;
        let mut owners = Vec::new();
        owners
            .try_reserve_exact(owner_count)
            .map_err(|_| SessionControlProviderError)?;
        let mut sections = Vec::new();
        sections
            .try_reserve_exact(section_count)
            .map_err(|_| SessionControlProviderError)?;
        let mut rows = Vec::new();
        let mut values = Vec::new();
        if fill_rows {
            rows.try_reserve_exact(total)
                .map_err(|_| SessionControlProviderError)?;
            values
                .try_reserve_exact(total)
                .map_err(|_| SessionControlProviderError)?;
        }
        // The distinct descriptors, found through a table that is freed before `build` returns.
        let mut shapes = ShapeTable::default();

        let ramps = LiveRamps::for_session(model);
        let mut strips = model.strips();
        let mut strip = None;
        let mut routes = model.routes.iter();
        let mut vcas = model.vcas.iter();
        let mut effect: Option<&'static EffectDescriptor> = None;
        for owner in 0..owner_total {
            scratch.clear();
            controls::owner_rows_in(view, &effective, overlay, owner, sample_rate, &mut scratch);
            let mut previous: Option<&controls::ControlAddress> = None;
            for row in &scratch {
                let address = &row.address;
                let (owner_opens, section_opens) = openings(previous, address);
                previous = Some(address);
                if owner_opens {
                    let (header, route) = match &address.owner {
                        Owner::Session => {
                            strip = None;
                            (ControlOwnerHeaderKind::Session, None)
                        }
                        Owner::Strip(id) => {
                            let found = strips
                                .by_ref()
                                .find(|candidate| candidate.id == id)
                                .ok_or(SessionControlProviderError)?;
                            strip = Some(found);
                            (
                                match found.kind {
                                    StripKind::Track(_) => ControlOwnerHeaderKind::Track,
                                    StripKind::Submix(_) => ControlOwnerHeaderKind::Submix,
                                },
                                None,
                            )
                        }
                        Owner::Route(id) => {
                            let found = routes
                                .by_ref()
                                .find(|candidate| candidate.id == *id)
                                .ok_or(SessionControlProviderError)?;
                            let (source, tap) = match &found.source {
                                session::RouteSource::Track { track_id, tap } => (track_id, *tap),
                                session::RouteSource::Submix { submix_id, tap } => {
                                    (submix_id, *tap)
                                }
                            };
                            let destination = match &found.destination {
                                session::RouteDestination::SubmixInput { submix_id } => submix_id,
                                session::RouteDestination::OutputInput { output_id } => output_id,
                            };
                            (
                                ControlOwnerHeaderKind::Route,
                                Some(ControlRouteEnds {
                                    source: try_string(source.as_str())?,
                                    destination: try_string(destination.as_str())?,
                                    tap,
                                }),
                            )
                        }
                        Owner::Vca(id) => {
                            vcas.by_ref()
                                .find(|candidate| candidate.id == *id)
                                .ok_or(SessionControlProviderError)?;
                            (ControlOwnerHeaderKind::Vca, None)
                        }
                    };
                    // The owner's wire kind and ID are the protocol's one address conversion's.
                    let owner_address = wire_address(address)?;
                    if fill_rows {
                        rows.push(RowRecord {
                            parent: index_of(owners.len())?,
                            control: 0,
                            descriptor: 0,
                            kind: RowKind::Owner,
                            lane: ParameterChannel::Both,
                        });
                        values.push(0.0);
                    }
                    owners.push(OwnerRecord {
                        kind: owner_address.owner_kind,
                        header,
                        id: owner_address.owner_id,
                        route,
                    });
                }
                if section_opens {
                    let identity = section_identity(model, strip.as_ref(), &address.section);
                    effect = identity.and_then(contract);
                    let effect_identity = match identity {
                        Some(EffectIdentity::Native { effect_id })
                            if matches!(
                                address.section,
                                Section::Console(_) | Section::Inserts(_)
                            ) =>
                        {
                            Some(effect_id.as_str())
                        }
                        _ => None,
                    };
                    let section_address = wire_address(address)?;
                    if fill_rows {
                        rows.push(RowRecord {
                            parent: index_of(sections.len())?,
                            control: 0,
                            descriptor: 0,
                            kind: RowKind::Section,
                            lane: ParameterChannel::Both,
                        });
                        values.push(0.0);
                    }
                    sections.push(SectionRecord {
                        owner: index_of(owners.len() - 1)?,
                        section: section_address.section,
                        slot_id: section_address.slot_id,
                        effect: effect_identity.map(try_string).transpose()?,
                    });
                }
                let descriptor = shapes.intern(Shape::of(row, ramps, effect))?;
                if fill_rows {
                    rows.push(RowRecord {
                        parent: index_of(sections.len() - 1)?,
                        control: address.control,
                        descriptor,
                        kind: RowKind::Control,
                        lane: ParameterChannel::from(address.lane),
                    });
                    values.push(row.value);
                }
            }
        }
        let descriptors = shapes.into_rows()?;
        let resources = CatalogResources::of(&owners, &sections, &descriptors, total, total)?;
        Ok(Self {
            owners,
            sections,
            descriptors,
            rows,
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

    /// The index of the owner a request filters on, if the list has it.
    fn owner_index(&self, filter: &protocol::ControlOwnerFilter) -> Option<u32> {
        self.owners
            .iter()
            .position(|owner| owner.kind == filter.kind && owner.id == filter.id)
            .and_then(|index| u32::try_from(index).ok())
    }

    /// The index of the owner that a row belongs to.
    fn owner_of(&self, record: &RowRecord) -> u32 {
        match record.kind {
            RowKind::Owner => record.parent,
            RowKind::Section | RowKind::Control => self.sections[record.parent as usize].owner,
        }
    }

    /// The wire entry at `index`, with the committed value of a control row filled in.
    fn entry(&self, index: usize) -> ControlEntry {
        let record = &self.rows[index];
        let handle = handle_of(index);
        match record.kind {
            RowKind::Owner => {
                let owner = &self.owners[record.parent as usize];
                ControlEntry {
                    handle,
                    address: ControlAddress {
                        owner_kind: owner.kind,
                        owner_id: owner.id.clone(),
                        section: ControlSection::Own,
                        slot_id: None,
                        control: None,
                        lane: None,
                    },
                    body: ControlBody::OwnerHeader(ControlOwnerHeader {
                        kind: owner.header,
                        name: owner.name().to_owned(),
                        route: owner.route.clone(),
                    }),
                }
            }
            RowKind::Section => {
                let section = &self.sections[record.parent as usize];
                let owner = &self.owners[section.owner as usize];
                ControlEntry {
                    handle,
                    address: ControlAddress {
                        owner_kind: owner.kind,
                        owner_id: owner.id.clone(),
                        section: section.section,
                        slot_id: section.slot_id.clone(),
                        control: None,
                        lane: None,
                    },
                    body: ControlBody::SectionHeader(ControlSectionHeader {
                        name: section.name().to_owned(),
                        effect: section.effect.clone(),
                    }),
                }
            }
            RowKind::Control => {
                let section = &self.sections[record.parent as usize];
                let owner = &self.owners[section.owner as usize];
                let mut row = self.descriptors[record.descriptor as usize].clone();
                row.value = self.values[index];
                ControlEntry {
                    handle,
                    address: ControlAddress {
                        owner_kind: owner.kind,
                        owner_id: owner.id.clone(),
                        section: section.section,
                        slot_id: section.slot_id.clone(),
                        control: Some(record.control),
                        lane: Some(record.lane),
                    },
                    body: ControlBody::Control(row),
                }
            }
        }
    }

    /// Whether the two catalogs list the same owners, sections and rows with the same descriptors,
    /// every `f32` of a descriptor compared by bits. Values are not compared.
    fn same_structure(&self, other: &Self) -> bool {
        self.owners == other.owners
            && self.sections == other.sections
            && self.rows == other.rows
            && self.descriptors.len() == other.descriptors.len()
            && self
                .descriptors
                .iter()
                .zip(&other.descriptors)
                .all(|(left, right)| same_descriptor(left, right))
    }

    /// Stage the catalog of the committed model `next` of a live commit, before the commit's first
    /// write (#1600). `overlay` is the monitor overlay the commit leaves, the one a full build of
    /// the committed catalog reads: the derived rows and a strip's `solo` row follow it (#1559
    /// D11). `self` is the catalog of the model the delta starts from, and `delta` is
    /// [`classify_live_delta`](crate::classify_live_delta)'s for the two.
    ///
    /// It returns [`StagedCatalog::Values`] when the committed catalog differs from `self` in
    /// values alone: no setting changed, the owners are the same, and every owner that has a changed
    /// value lists the same addresses and descriptors (every `f32` by bits) as before. The staged
    /// writes are the rows of the changed owners and every derived row, read from one effective
    /// pass, so the cost is the changed owners' rows plus the strips and routes, never the whole
    /// list. Any other delta returns [`StagedCatalog::Full`], the catalog [`Self::build`] gives.
    ///
    /// Either way the result is what a full build would give once applied (an `f32` by bits); in
    /// a test build the `Values` case builds the full catalog and checks it. Nothing is written
    /// and nothing is published: a refusal here leaves the commit unstarted.
    ///
    /// # Errors
    ///
    /// [`SessionControlProviderError`] if a reserved table cannot be allocated or the list
    /// outgrows the table indices. (The automation index of a stage that reads many owners
    /// allocates as an ordinary map does, with no fallible path.)
    pub fn stage_live(
        &self,
        next: controls::ModelView<'_>,
        overlay: &MonitorOverlay,
        sample_rate: u32,
        delta: &crate::LiveDelta<'_>,
    ) -> Result<StagedCatalog, SessionControlProviderError> {
        // #1709 D1: the staged derived rows are composed in the canonical environment, as
        // `Self::build` composes them.
        let _canonical = lane::CanonicalFpEnv::enter();
        #[cfg(any(test, feature = "test-support"))]
        if STAGE_FAILURES.with(|count| {
            let pending = count.get();
            count.set(pending.saturating_sub(1));
            pending > 0
        }) {
            return Err(SessionControlProviderError);
        }
        // A stage that reads the automation of more than a few owners walks the committed table
        // once for all of them (#1710); a one-owner hand move scans it per reader, which is
        // cheaper than building the index.
        let reads = delta.changed_owners.len() + delta.derived_owners.as_ref().map_or(0, Vec::len);
        let automation;
        let next = if reads > STAGE_INDEX_OWNERS {
            automation = controls::AutomationIndex::of(next.structure());
            next.indexed(&automation)
        } else {
            next
        };
        if let Some(staged) = self.stage_values(next, overlay, sample_rate, delta)? {
            #[cfg(any(test, feature = "test-support"))]
            if cfg!(debug_assertions) && VERIFY_STAGES.with(core::cell::Cell::get) {
                let dense = dense(next);
                let full = Self::build(&dense, overlay, sample_rate)?;
                assert!(
                    self.same_structure(&full),
                    "a values-only stage must keep every address, header and descriptor"
                );
                let mut values = self.values.clone();
                for &(index, value) in &staged.writes {
                    values[index as usize] = value;
                }
                assert!(
                    values.len() == full.values.len()
                        && values
                            .iter()
                            .zip(&full.values)
                            .all(|(left, right)| left.to_bits() == right.to_bits()),
                    "a values-only stage must give the values of a full build"
                );
            }
            return Ok(StagedCatalog::Values(staged));
        }
        #[cfg(any(test, feature = "test-support"))]
        FULL_STAGES.with(|count| count.set(count.get() + 1));
        Self::build(&dense(next), overlay, sample_rate).map(StagedCatalog::Full)
    }

    /// The writes that bring `self` to the catalog of `next`, or `None` when a values-only
    /// refresh cannot (a setting changed, the owners differ, or a changed owner lists other
    /// addresses or descriptors).
    ///
    /// A delta classified from the rows the edits name (`derived_owners` is `Some`) says which
    /// derived rows its edits reach, and the model keeps its owners: the stage then reads the
    /// effective values of the owners it stages only (`controls::LocalEffective`) and the derived
    /// rows of the owners the delta names, and none of the others. Without it, every derived row
    /// is read again.
    fn stage_values(
        &self,
        next: controls::ModelView<'_>,
        overlay: &MonitorOverlay,
        sample_rate: u32,
        delta: &crate::LiveDelta<'_>,
    ) -> Result<Option<StagedValues>, SessionControlProviderError> {
        let model = next.structure();
        let reached = delta.derived_owners.as_deref();
        // A delta that names the derived owners it reached was made from rows, which cannot add or
        // remove an owner; the catalog must still list the owners the model has.
        let owners_agree = if reached.is_some() {
            self.owners.len() == controls::owner_count(model)
        } else {
            self.lists_owners_of(model)
        };
        if delta.settings_changed || !owners_agree {
            return Ok(None);
        }
        let effective = match reached {
            Some(_) => StageEffective::Local(controls::LocalEffective::new(next, overlay)),
            None => StageEffective::Dense(controls::effective(model, overlay)),
        };
        let ramps = LiveRamps::for_session(model);
        let strip_count = model.tracks.len() + model.submixes.len();
        // The derived rows sit in the first section of a strip and of a route.
        let derived_owners = strip_count + model.routes.len();
        let derived = || -> Box<dyn Iterator<Item = usize> + '_> {
            match reached {
                Some(owners) => Box::new(owners.iter().copied()),
                None => Box::new(0..derived_owners),
            }
        };

        // The runs of the rows to read again, and a bound of the writes, before any is made.
        let mut runs = Vec::new();
        runs.try_reserve_exact(delta.changed_owners.len())
            .map_err(|_| SessionControlProviderError)?;
        let mut capacity = 0_usize;
        // The most rows one owner (or one owner's derived rows) makes at once: the scratch below is
        // reserved to it, exactly, so it never grows by doubling and copying (#1615).
        let mut longest = 0_usize;
        for &owner in &delta.changed_owners {
            let Some(run) = self.owner_run(owner) else {
                return Ok(None);
            };
            capacity += run.len();
            longest = longest.max(run.len());
            runs.push(run);
        }
        for owner in derived() {
            let count = self.derived_indices(owner).count();
            capacity += count;
            longest = longest.max(count);
        }
        let mut writes = Vec::new();
        writes
            .try_reserve_exact(capacity)
            .map_err(|_| SessionControlProviderError)?;
        // A run holds an owner's rows and its section headers, so it bounds the rows the owner makes
        // (a different count is refused below). The memo holds one shape per distinct descriptor.
        let mut fresh = Vec::new();
        fresh
            .try_reserve_exact(longest)
            .map_err(|_| SessionControlProviderError)?;
        let mut memo: HashMap<u32, Shape, BuildHasherDefault<DefaultHasher>> = HashMap::default();
        memo.try_reserve(longest.min(self.descriptors.len()))
            .map_err(|_| SessionControlProviderError)?;

        for (&owner, run) in delta.changed_owners.iter().zip(runs) {
            #[cfg(any(test, feature = "test-support"))]
            STAGED_OWNERS.with(|count| count.set(count.get() + 1));
            fresh.clear();
            controls::owner_rows_in(next, &effective, overlay, owner, sample_rate, &mut fresh);
            // The strip order is the tracks then the submixes; index them directly rather than
            // walking the chained iterator once per changed owner.
            let strip = (owner < strip_count).then(|| next.strip(owner));
            if !self.stage_run(
                model,
                strip.as_ref(),
                owner,
                run,
                &fresh,
                ramps,
                &mut memo,
                &mut writes,
            )? {
                return Ok(None);
            }
        }
        for owner in derived() {
            fresh.clear();
            controls::derived_rows_in(next, &effective, owner, &mut fresh);
            let indices = self.derived_indices(owner);
            if indices.clone().count() != fresh.len() {
                return Ok(None);
            }
            for (index, row) in indices.zip(&fresh) {
                if !self.address_matches(&self.rows[index], owner, &row.address) {
                    return Ok(None);
                }
                writes.push((index_of(index)?, row.value));
            }
        }
        Ok(Some(StagedValues { writes }))
    }

    /// Whether the catalog's owners are the owners of `next`, in the list's order: the strips,
    /// the routes, the VCAs and the session. A VCA added or removed fails it.
    fn lists_owners_of(&self, next: &SessionModel) -> bool {
        let strips = next
            .strips()
            .map(|strip| (ControlOwnerKind::Strip, Some(strip.id)));
        let routes = next
            .routes
            .iter()
            .map(|route| (ControlOwnerKind::Route, Some(&route.id)));
        let vcas = next
            .vcas
            .iter()
            .map(|vca| (ControlOwnerKind::Vca, Some(&vca.id)));
        let expected = strips
            .chain(routes)
            .chain(vcas)
            .chain([(ControlOwnerKind::Session, None)]);
        self.owners.len() == controls::owner_count(next)
            && self.owners.iter().zip(expected).all(|(owner, (kind, id))| {
                owner.kind == kind && owner.id.as_deref() == id.map(session::StableId::as_str)
            })
    }

    /// The index range of owner number `owner`'s entries: its header and everything after it
    /// until the next owner's. `None` when the list has no such owner.
    fn owner_run(&self, owner: usize) -> Option<core::ops::Range<usize>> {
        let owner = u32::try_from(owner).ok()?;
        let start = self.rows.partition_point(|row| self.owner_of(row) < owner);
        let length = self.rows[start..].partition_point(|row| self.owner_of(row) == owner);
        (length > 0 && self.rows[start].kind == RowKind::Owner).then_some(start..start + length)
    }

    /// The entries of the derived rows of strip or route number `owner`, in list order. They are
    /// control rows of the owner's first section.
    fn derived_indices(&self, owner: usize) -> impl Iterator<Item = usize> + Clone + '_ {
        let start = u32::try_from(owner).ok().map_or(self.rows.len(), |owner| {
            self.rows.partition_point(|row| self.owner_of(row) < owner)
        });
        // The owner's header, its first section's header, then that section's rows.
        let first = (start + 2).min(self.rows.len());
        (first..self.rows.len())
            .take_while(|&index| self.rows[index].kind == RowKind::Control)
            .filter(|&index| {
                self.descriptors[self.rows[index].descriptor as usize].class
                    == ControlClass::Derived
            })
    }

    /// Whether the entry `record` lists the control at `address`, of owner number `owner`.
    fn address_matches(
        &self,
        record: &RowRecord,
        owner: usize,
        address: &controls::ControlAddress,
    ) -> bool {
        if record.kind != RowKind::Control
            || record.control != address.control
            || record.lane != ParameterChannel::from(address.lane)
        {
            return false;
        }
        let section = &self.sections[record.parent as usize];
        let owner_record = &self.owners[section.owner as usize];
        let (owner_id, slot_id) = (
            match &address.owner {
                Owner::Session => None,
                Owner::Strip(id) | Owner::Route(id) | Owner::Vca(id) => Some(id.as_str()),
            },
            match &address.section {
                Section::Own | Section::Builtins => None,
                Section::Console(id) | Section::Inserts(id) => Some(id.as_str()),
            },
        );
        section.owner as usize == owner
            && owner_record.kind == ControlOwnerKind::from_controls(address.owner.kind())
            && owner_record.id.as_deref() == owner_id
            && section.section == ControlSection::from_controls(address.section.kind())
            && section.slot_id.as_deref() == slot_id
    }

    /// Check one changed owner's `fresh` rows against its stored `run`, and stage the value of
    /// each. `Ok(false)` when the owner lists other addresses, other section headers or other
    /// descriptors than the run holds, which a values-only refresh cannot express.
    #[allow(clippy::too_many_arguments)]
    fn stage_run(
        &self,
        model: &SessionModel,
        strip: Option<&session::StripRef<'_>>,
        owner: usize,
        run: core::ops::Range<usize>,
        fresh: &[controls::ControlRow],
        ramps: LiveRamps,
        memo: &mut HashMap<u32, Shape, BuildHasherDefault<DefaultHasher>>,
        writes: &mut Vec<(u32, f32)>,
    ) -> Result<bool, SessionControlProviderError> {
        // The run opens with the owner's header.
        let mut at = run.start + 1;
        let mut section: Option<&controls::Section> = None;
        let mut effect: Option<&'static EffectDescriptor> = None;
        for row in fresh {
            if section != Some(&row.address.section) {
                // A section opens with its header, whose section and slot are the row's.
                let Some(header) = self.rows.get(at).filter(|record| {
                    at < run.end
                        && record.kind == RowKind::Section
                        && self.sections[record.parent as usize].owner as usize == owner
                }) else {
                    return Ok(false);
                };
                let stored = &self.sections[header.parent as usize];
                let slot_id = match &row.address.section {
                    Section::Own | Section::Builtins => None,
                    Section::Console(id) | Section::Inserts(id) => Some(id.as_str()),
                };
                if stored.section != ControlSection::from_controls(row.address.section.kind())
                    || stored.slot_id.as_deref() != slot_id
                {
                    return Ok(false);
                }
                section = Some(&row.address.section);
                effect = section_identity(model, strip, &row.address.section).and_then(contract);
                at += 1;
            }
            let Some(record) = self.rows.get(at).filter(|_| at < run.end) else {
                return Ok(false);
            };
            if !self.address_matches(record, owner, &row.address) {
                return Ok(false);
            }
            let shape = Shape::of(row, ramps, effect);
            if memo.get(&record.descriptor) != Some(&shape) {
                let stored = &self.descriptors[record.descriptor as usize];
                if !same_descriptor(&shape.to_row()?, stored) {
                    return Ok(false);
                }
                memo.try_reserve(1)
                    .map_err(|_| SessionControlProviderError)?;
                memo.insert(record.descriptor, shape);
            }
            writes.push((index_of(at)?, row.value));
            at += 1;
        }
        // The run holds nothing that the owner no longer lists.
        Ok(at == run.end)
    }

    /// Every wire entry, with the committed values filled in.
    #[cfg(test)]
    fn entries(&self) -> Vec<ControlEntry> {
        (0..self.rows.len())
            .map(|index| self.entry(index))
            .collect()
    }
}

/// The effective values a stage reads: every strip's and send's, built once (the full stage), or
/// computed for the owners asked for (a stage from the rows an edit names).
enum StageEffective<'a> {
    Dense(controls::Effective),
    Local(controls::LocalEffective<'a>),
}

impl controls::EffectiveSource for StageEffective<'_> {
    fn strip(&self, index: usize) -> controls::EffectiveStrip {
        match self {
            Self::Dense(effective) => controls::EffectiveSource::strip(effective, index),
            Self::Local(effective) => controls::EffectiveSource::strip(effective, index),
        }
    }

    fn route_gate(&self, index: usize) -> [bool; 2] {
        match self {
            Self::Dense(effective) => controls::EffectiveSource::route_gate(effective, index),
            Self::Local(effective) => controls::EffectiveSource::route_gate(effective, index),
        }
    }
}

/// The dense model a view stands for, without a copy when the view reads one whole model.
fn dense<'a>(view: controls::ModelView<'a>) -> std::borrow::Cow<'a, SessionModel> {
    if view.is_whole() {
        std::borrow::Cow::Borrowed(view.structure())
    } else {
        std::borrow::Cow::Owned(view.to_model())
    }
}

/// The native effect identity of an effect section of `strip` (a console slot or an insert), or
/// `None` for any other section.
fn section_identity<'a>(
    model: &'a SessionModel,
    strip: Option<&session::StripRef<'a>>,
    section: &Section,
) -> Option<&'a EffectIdentity> {
    match section {
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
    }
}

/// `index` as a table index.
fn index_of(index: usize) -> Result<u32, SessionControlProviderError> {
    u32::try_from(index).map_err(|_| SessionControlProviderError)
}

/// Whether two descriptors are equal in every field, every `f32` by bits.
fn same_descriptor(left: &ControlRow, right: &ControlRow) -> bool {
    let bits = |value: f32| value.to_bits();
    left == right
        && bits(left.minimum) == bits(right.minimum)
        && bits(left.maximum) == bits(right.maximum)
        && left.disabled_value.map(bits) == right.disabled_value.map(bits)
        && bits(left.default) == bits(right.default)
        && bits(left.step) == bits(right.step)
        && bits(left.value) == bits(right.value)
        && left
            .ladder
            .iter()
            .map(|value| bits(*value))
            .eq(right.ladder.iter().map(|value| bits(*value)))
        && left
            .choices
            .iter()
            .map(|choice| bits(choice.value))
            .eq(right.choices.iter().map(|choice| bits(choice.value)))
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

/// The named choices of a control, compared by the bits of each value.
#[derive(Clone, Copy, Debug)]
struct Choices(&'static [controls::Choice]);

impl PartialEq for Choices {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
            && self.0.iter().zip(other.0).all(|(left, right)| {
                left.value.to_bits() == right.value.to_bits() && left.label == right.label
            })
    }
}

/// Every field of a control's wire descriptor in copyable form, with each `f32` as its bits: the
/// key that two rows share a stored descriptor by, and the source of the stored descriptor. No
/// field owns memory, so a row's shape is found without allocating.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Shape {
    name: &'static str,
    display_name: &'static str,
    display_unit: &'static str,
    unit: ProtocolUnit,
    kind: ControlKind,
    minimum: u32,
    maximum: u32,
    disabled_value: Option<u32>,
    default: u32,
    taper: ControlTaper,
    step: u32,
    precision: u8,
    /// The named step multiples; absent on an index lattice.
    ladder: Option<[u8; 5]>,
    choices: Choices,
    ramp_row: Option<ControlRampRow>,
    default_ramp_samples: u32,
    class: ControlClass,
    reason: Option<ControlReason>,
    reason_issue: Option<u32>,
    reopen: Option<&'static str>,
    scope: ControlScope,
    automatable: bool,
    family: ControlFamily,
    active: bool,
    /// #1650 D4: a rendered stored automation drives the row's lane.
    automated: bool,
}

impl Eq for Shape {}

impl Hash for Shape {
    /// The fields that tell rows apart cheaply; equal shapes hash equal.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.display_name.hash(state);
        self.minimum.hash(state);
        self.maximum.hash(state);
        self.default.hash(state);
        self.default_ramp_samples.hash(state);
        self.active.hash(state);
    }
}

impl Shape {
    /// The shape of one control row. `effect` is the contract of the effect section the row is in,
    /// which gives an effect parameter's own ramp.
    fn of(
        row: &controls::ControlRow,
        ramps: LiveRamps,
        effect: Option<&'static EffectDescriptor>,
    ) -> Self {
        let descriptor = &row.descriptor;
        let (ramp_row, default_ramp_samples) = match descriptor.motion {
            Motion::Ramp(live) => (Some(ramp_row(live)), ramps.for_row(live)),
            // The bypass crossfade has no per-edit length, so its row is fixed, and the length it
            // runs over is the session's `mute_ms` (#1341).
            Motion::Fixed
                if descriptor.family == controls::Family::Effect && descriptor.name == "bypass" =>
            {
                (None, ramps.for_row(LiveRampRow::BypassCrossfade))
            }
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
                Some(reopen),
            ),
        };
        Self {
            name: descriptor.name,
            display_name: descriptor.display_name,
            display_unit: descriptor.display_unit,
            unit: protocol_unit(descriptor.unit),
            kind: match descriptor.kind {
                controls::ControlKind::Continuous => ControlKind::Continuous,
                controls::ControlKind::Toggle => ControlKind::Toggle,
                controls::ControlKind::Enumeration => ControlKind::Enumeration,
            },
            minimum: descriptor.minimum.to_bits(),
            maximum: descriptor.maximum.to_bits(),
            disabled_value: descriptor.disabled_value.map(f32::to_bits),
            default: descriptor.default.to_bits(),
            taper: match descriptor.taper {
                controls::Taper::Linear => ControlTaper::Linear,
                controls::Taper::Logarithmic => ControlTaper::Logarithmic,
                controls::Taper::Exponential => ControlTaper::Exponential,
                controls::Taper::Decibel => ControlTaper::Decibel,
                controls::Taper::Stepped => ControlTaper::Stepped,
            },
            step: descriptor.lattice.step.to_bits(),
            precision: descriptor.lattice.precision,
            // The named step sizes are the lattice multiples of the step; an index lattice (a
            // toggle or an enumeration) has none.
            ladder: (descriptor.lattice.step_unit != StepUnit::Index)
                .then_some(descriptor.lattice.ladder.multiples),
            choices: Choices(descriptor.choices),
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
            automated: row.automated,
        }
    }

    /// The stored wire descriptor of this shape, with the value left at `0.0` (the committed
    /// value lives in the catalog's `values`).
    fn to_row(self) -> Result<ControlRow, SessionControlProviderError> {
        // Reserved exactly, as every other catalog allocation is: `resource_report` charges these
        // by length, and collecting through `Result<Vec<_>, _>` (which has no lower size bound)
        // grew six choices to a capacity of eight, 64 retained bytes per EQ kind descriptor that
        // no row charged (#1060).
        let mut choices = Vec::new();
        choices
            .try_reserve_exact(self.choices.0.len())
            .map_err(|_| SessionControlProviderError)?;
        for choice in self.choices.0 {
            choices.push(EnumChoice {
                value: choice.value,
                label: try_string(choice.label)?,
            });
        }
        let mut ladder = Vec::new();
        if let Some(multiples) = self.ladder {
            ladder
                .try_reserve_exact(multiples.len())
                .map_err(|_| SessionControlProviderError)?;
            ladder.extend(multiples.iter().map(|multiple| f32::from(*multiple)));
        }
        Ok(ControlRow {
            name: try_string(self.name)?,
            display_name: try_string(self.display_name)?,
            display_unit: try_string(self.display_unit)?,
            unit: self.unit,
            kind: self.kind,
            minimum: f32::from_bits(self.minimum),
            maximum: f32::from_bits(self.maximum),
            disabled_value: self.disabled_value.map(f32::from_bits),
            default: f32::from_bits(self.default),
            taper: self.taper,
            step: f32::from_bits(self.step),
            precision: self.precision,
            ladder,
            choices,
            ramp_row: self.ramp_row,
            default_ramp_samples: self.default_ramp_samples,
            class: self.class,
            reason: self.reason,
            reason_issue: self.reason_issue,
            reopen: self.reopen.map(try_string).transpose()?,
            scope: self.scope,
            automatable: self.automatable,
            family: self.family,
            active: self.active,
            value: 0.0,
            automated: self.automated,
        })
    }
}

/// The distinct shapes of one build, in order of first use, and the index that finds one. It is
/// transient: `build` turns it into the stored descriptors and frees it.
#[derive(Default)]
struct ShapeTable {
    index: HashMap<Shape, u32, BuildHasherDefault<DefaultHasher>>,
    shapes: Vec<Shape>,
}

impl ShapeTable {
    /// The index of `shape` among the distinct shapes, adding it when it is new.
    fn intern(&mut self, shape: Shape) -> Result<u32, SessionControlProviderError> {
        if let Some(index) = self.index.get(&shape) {
            return Ok(*index);
        }
        let index = index_of(self.shapes.len())?;
        self.index
            .try_reserve(1)
            .map_err(|_| SessionControlProviderError)?;
        self.shapes
            .try_reserve(1)
            .map_err(|_| SessionControlProviderError)?;
        self.index.insert(shape, index);
        self.shapes.push(shape);
        Ok(index)
    }

    /// The stored descriptors, one per distinct shape, in order of first use.
    fn into_rows(self) -> Result<Vec<ControlRow>, SessionControlProviderError> {
        let mut rows = Vec::new();
        rows.try_reserve_exact(self.shapes.len())
            .map_err(|_| SessionControlProviderError)?;
        for shape in self.shapes {
            rows.push(shape.to_row()?);
        }
        Ok(rows)
    }
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
    let mut meters = Vec::new();
    meters
        .try_reserve_exact(retained.meters)
        .map_err(|_| SessionControlProviderError)?;
    let mut counter_ids = Vec::new();
    counter_ids
        .try_reserve_exact(retained.counter_ids)
        .map_err(|_| SessionControlProviderError)?;
    Ok(TelemetryConfiguration {
        meters,
        meter_period_blocks: 0,
        meter_window_blocks: None,
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

/// Whether a page of `request` holds `record`: every row, or with `headers_only` (#2206) the owner
/// and section headers alone.
fn serves(request: &ControlListRequest, record: &RowRecord) -> bool {
    !request.headers_only || record.kind != RowKind::Control
}

/// The most rows a page of `request` over `start..end` holds: the span, or with `headers_only` at
/// most `limit`, since the span also covers the control rows between the headers.
fn served_bound(request: &ControlListRequest, start: usize, end: usize) -> usize {
    let span = end - start;
    if request.headers_only {
        span.min(usize::from(request.limit))
    } else {
        span
    }
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

    use crate::RESPONSE_MAXIMUM_OVERRIDES;
    use protocol::{ControlOwnerKind, MeterMetric, ParameterChannel, ResponseOverride};

    use super::*;

    const RATE: u32 = 48_000;

    /// #1587a: a meter table is served only when one batch can carry all its records. 128 peak
    /// entries need 256 records and pass; the 129th entry takes the table past the batch's bound
    /// and is refused, `INVALID_FIELD`, `meter.records`, at that entry.
    ///
    /// Test value: red if the bound is missing or off by one, so an accepted table's batches can
    /// never encode and every window it meters is lost.
    #[test]
    fn a_meter_table_past_one_batch_of_records_is_refused_at_its_first_extra_entry() {
        let window = core::num::NonZeroU32::new(4);
        let table = |entries: usize| protocol::TelemetryConfiguration {
            meters: (0..entries)
                .map(|index| protocol::MeterEntry {
                    target: protocol::MeterTarget::Strip(format!("s{index}").into()),
                    metric: MeterMetric::Peak,
                })
                .collect(),
            meter_period_blocks: 4,
            meter_window_blocks: None,
            counter_ids: Vec::new(),
            counter_period_blocks: 0,
            diagnostics_enabled: false,
            minimum_diagnostic_severity: DiagnosticSeverity::Info,
        };
        assert_eq!(
            refuse_meter_table(&table(MAXIMUM_METER_BATCH_RECORDS / 2), window),
            Ok(())
        );
        assert_eq!(
            refuse_meter_table(&table(MAXIMUM_METER_BATCH_RECORDS / 2 + 1), window),
            Err(TelemetryRefusal {
                error: ParameterProviderError::InvalidField,
                code: Some("meter.records"),
                path: TelemetryRefusalPath::Meter(MAXIMUM_METER_BATCH_RECORDS / 2),
            })
        );
    }

    /// A reply frame that cannot be encoded is a typed failure with an empty reply, never an empty
    /// success the host would hand on as a frame.
    ///
    /// Red if `write_non_ok` ignores the encode error: it returns `Ok` with `reply` truncated to
    /// zero bytes, and `miso_engine_web_v1_response_frame_submit` answers `RESULT_OK` with no frame.
    #[test]
    fn a_non_ok_frame_that_cannot_be_encoded_is_an_internal_failure() {
        let mut request = vec![0_u8; 1 << 10];
        let length = protocol::ProtocolCodec::default()
            .encode_command_frame_into(
                &protocol::TypedCommandFrame {
                    request_id: protocol::RequestId::new(7).expect("request ID"),
                    expected_revision: protocol::ExpectedRevision::Any,
                    payload: protocol::CommandPayload::CapabilitiesGet,
                },
                &mut request,
            )
            .expect("command frame");
        let header = protocol::ProtocolCodec::default()
            .decode_command_header(&request[..length])
            .expect("header");
        let too_small = protocol::ProtocolCodec::new(protocol::ProtocolLimits {
            max_frame_bytes: protocol::OUTER_HEADER_BYTES,
            ..protocol::ProtocolLimits::default()
        });
        let mut reply = vec![1, 2, 3];
        assert_eq!(
            write_non_ok(
                &too_small,
                header,
                StatusCode::Internal,
                "response.internal",
                &mut reply
            ),
            Err(ResponseFrameError::Internal)
        );
        assert!(reply.is_empty());
        let roomy = protocol::ProtocolCodec::default();
        assert_eq!(
            write_non_ok(
                &roomy,
                header,
                StatusCode::Internal,
                "response.internal",
                &mut reply
            ),
            Ok(())
        );
        assert!(!reply.is_empty());
    }

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

    fn response_request() -> ResponseGetRequest {
        ResponseGetRequest {
            target: ResponseTarget::Configuration(ResponseConfiguration::Effect {
                effect: "miso.parametric-eq".to_owned(),
                quality: session::EffectQuality::Normal,
                link_mode: session::LinkMode::DualMono,
            }),
            grid: ResponseGrid::Logarithmic,
            points: 64,
            minimum_hz: 20.0,
            maximum_hz: 20_000.0,
            channels: ParameterChannel::Both,
            sections: false,
            overrides: Vec::new(),
            configuration_id: None,
            sample_rate_hz: None,
            quantum_frames: None,
            maximum_prepared_bytes: None,
            maximum_state_bytes: None,
            maximum_scratch_bytes: None,
            maximum_automation_spans: None,
            maximum_reply_bytes: None,
        }
    }

    /// A request that is wrong in several ways is refused with the core's refusal first, in the
    /// core's order (rate, points, override count, axis, override values), before the provider's
    /// own rules (a rate on a session endpoint, a zero byte budget, the bypass); a bypass value
    /// between 0 and 1 is `response.override.bypass`, not "no such parameter"; and a NaN override
    /// value, a bypass row's included, is `response.override.value` (#1623).
    ///
    /// Red if the provider shapes the overrides before it asks the core, so a request with one bad
    /// point count and one bad bypass value answers the bypass, or the bypass shares the code of an
    /// unknown parameter.
    #[test]
    fn response_get_asks_the_core_first_and_names_a_bad_bypass() {
        let model = nine_track();
        let (mut provider, _clock) = provider_of(&model);
        let bypass = |value: f32| ResponseOverride {
            control: protocol::RESPONSE_BYPASS_CONTROL,
            lane: ParameterChannel::Both,
            value,
        };
        let mut refusal = |request: &ResponseGetRequest| {
            let error = provider
                .response_get(&model, request)
                .expect_err("a refusal");
            (error.status, error.code)
        };

        let mut both_wrong = response_request();
        both_wrong.overrides = vec![bypass(0.5)];
        assert_eq!(
            refusal(&both_wrong),
            (StatusCode::InvalidField, "response.override.bypass"),
            "a bypass between 0 and 1 has its own code"
        );
        both_wrong.points = 1;
        assert_eq!(
            refusal(&both_wrong),
            (StatusCode::InvalidField, "response.points"),
            "the point count is the core's and comes first"
        );
        both_wrong.points = 64;
        both_wrong.minimum_hz = f32::NAN;
        both_wrong.sample_rate_hz = Some(RATE);
        assert_eq!(
            refusal(&both_wrong),
            (StatusCode::InvalidField, "response.grid"),
            "the axis is the core's and precedes the session-rate rule"
        );
        both_wrong.minimum_hz = 20.0;
        both_wrong.overrides.push(bypass(f32::NAN));
        assert_eq!(
            refusal(&both_wrong),
            (StatusCode::InvalidField, "response.override.value"),
            "a non-finite override value is the core's, after the axis and before the session rate"
        );
        both_wrong.overrides.pop();
        assert_eq!(
            refusal(&both_wrong),
            (StatusCode::InvalidField, "response.rate.session"),
            "then the provider's own rules"
        );
        both_wrong.sample_rate_hz = None;
        both_wrong.overrides = vec![bypass(0.5); RESPONSE_MAXIMUM_OVERRIDES + 1];
        assert_eq!(
            refusal(&both_wrong),
            (StatusCode::LimitExceeded, "response.override.count"),
            "the override count precedes the bypass rows"
        );
        // An unknown parameter keeps the code it had.
        let mut unknown = response_request();
        unknown.overrides = vec![ResponseOverride {
            control: 9_999,
            lane: ParameterChannel::Both,
            value: 1.0,
        }];
        assert_eq!(
            refusal(&unknown),
            (StatusCode::InvalidField, "response.override.control")
        );
    }

    /// The core serves with the rate and quantum its caller resolves, and refuses as the session
    /// endpoint does: the curves at 44,100 Hz and 128 frames are the response query's own, and a
    /// 44,100 Hz session gives the same curves for the request without the rate field; a target
    /// that is no configuration, one point and a NaN axis are the session's three codes.
    ///
    /// Red if the core reads `request.sample_rate_hz` (here 48,000) instead of its argument, takes
    /// its rate or quantum from anywhere but its arguments, or skips the target or core checks
    /// that only the session wrapper ran before.
    #[test]
    fn the_core_serves_without_a_session_and_refuses_as_the_session_does() {
        const SESSION_RATE: u32 = 44_100;
        const SESSION_QUANTUM: u32 = 128;
        const BOUND: usize = 1 << 20;
        let mut request = response_request();
        request.configuration_id = Some(0x5eed_1610);

        let expected = prepare_response_query(&ResponseQuery {
            configuration_id: 0x5eed_1610,
            sample_rate_hz: SESSION_RATE,
            quantum_frames: SESSION_QUANTUM,
            target: ResponsePreviewTarget::Effect {
                effect_id: "miso.parametric-eq",
                overrides: &[],
                quality: effect_contract::EffectQuality::Normal,
                bypass: false,
                link_mode: effect_contract::LinkMode::DualMono,
            },
            grid: ResponsePreviewGrid::Logarithmic {
                points: 64,
                minimum_hz: 20.0,
                maximum_hz: 20_000.0,
            },
            channels: ResponseChannels::Both,
            sections: false,
            limits: ResponsePreviewLimits::default(),
        })
        .expect("the core prepares it")
        .curves()
        .expect("the core evaluates it");
        let bits = |values: &[f32]| values.iter().map(|v| v.to_bits()).collect::<Vec<_>>();
        let same = |result: ResponseResult| {
            let ResponseResult::Curves(curves) = result else {
                panic!("expected curves");
            };
            assert_eq!(curves.sample_rate_hz, SESSION_RATE);
            assert_eq!(curves.configuration_id, 0x5eed_1610);
            assert_eq!(bits(&curves.frequencies_hz), bits(&expected.frequencies_hz));
            assert_eq!(
                bits(curves.left_db.as_deref().expect("left")),
                bits(expected.left_db.as_deref().expect("left"))
            );
            assert_eq!(
                bits(curves.right_db.as_deref().expect("right")),
                bits(expected.right_db.as_deref().expect("right"))
            );
        };

        // The request names another rate; the core serves at its argument.
        let mut other_rate = request.clone();
        other_rate.sample_rate_hz = Some(RATE);
        other_rate.quantum_frames = Some(64);
        same(
            serve_response_at(SESSION_RATE, SESSION_QUANTUM, BOUND, &other_rate, None)
                .expect("the core serves"),
        );

        // A session endpoint refuses the rates the request names, after the preflight and before
        // anything else: one function serves both endpoints, so the order is the same.
        let named = serve_response_at(
            SESSION_RATE,
            SESSION_QUANTUM,
            BOUND,
            &other_rate,
            Some(&nine_track()),
        )
        .expect_err("a session has one rate");
        assert_eq!(
            (named.status, named.code),
            (StatusCode::InvalidField, "response.rate.session")
        );
        let mut named_and_short = other_rate.clone();
        named_and_short.points = 1;
        let named = serve_response_at(
            SESSION_RATE,
            SESSION_QUANTUM,
            BOUND,
            &named_and_short,
            Some(&nine_track()),
        )
        .expect_err("refused");
        assert_eq!(
            (named.status, named.code),
            (StatusCode::InvalidField, "response.points"),
            "the core's checks come before the session rule"
        );

        // A session at the same rate and quantum gives the same curves for the bare request.
        let mut model = nine_track();
        model.sample_rate_hz = SESSION_RATE;
        model.quantum_frames = SESSION_QUANTUM;
        same(serve_response(BOUND, &model, &request).expect("the session serves"));

        // The core's refusals are the session's, in the session's codes.
        let refusal = |request: &ResponseGetRequest| {
            let error = serve_response_at(SESSION_RATE, SESSION_QUANTUM, BOUND, request, None)
                .expect_err("refused");
            let session = serve_response(BOUND, &model, request).expect_err("refused");
            assert_eq!((error.status, error.code), (session.status, session.code));
            (error.status, error.code)
        };
        let mut address = request.clone();
        address.target = ResponseTarget::Strip("track-1".to_owned());
        assert_eq!(
            refusal(&address),
            (StatusCode::Unavailable, "response.target.unavailable")
        );
        let mut one_point = request.clone();
        one_point.points = 1;
        assert_eq!(
            refusal(&one_point),
            (StatusCode::InvalidField, "response.points")
        );
        let mut nan_axis = request;
        nan_axis.minimum_hz = f32::NAN;
        assert_eq!(
            refusal(&nan_axis),
            (StatusCode::InvalidField, "response.grid")
        );
    }

    /// #1575: a nine-track session with asymmetric input filters on `eq0`, a parametric EQ insert
    /// `ins` with its own values, a compressor insert `comp` and a `cid` insert `pkg` on `eq1`, and
    /// `eq2`'s own console entry of the session's EQ slot `eq`, and `eq3`'s bypassed entry.
    fn address_session() -> session::SessionModel {
        let mut model = nine_track();
        let builtins = &mut model.tracks[0].builtins;
        (builtins.left.hpf_hz, builtins.left.lpf_hz) = (40.0, 16_000.0);
        (builtins.right.hpf_hz, builtins.right.lpf_hz) = (90.0, 11_000.0);
        let param = |parameter_id: u32, channel, unit, value: f32| session::EffectParam {
            parameter_id,
            channel,
            unit,
            value,
        };
        let (left, right) = (
            session::ParameterChannel::Left,
            session::ParameterChannel::Right,
        );
        let mut entry = model.tracks[0].console[0].params.clone();
        for item in &mut entry {
            if item.parameter_id == 4 {
                item.value = if item.channel == left { 3.5 } else { -7.25 };
            }
        }
        model.tracks[2].console[0].params = entry;
        // `eq3` runs eq0's values with its entry bypassed.
        model.tracks[3].console[0] = session::ConsoleEntry {
            bypass: true,
            ..model.tracks[0].console[0].clone()
        };
        let mut eq = model.tracks[0].console[0].params.clone();
        eq.extend([
            param(
                17,
                session::ParameterChannel::Both,
                session::ParameterUnit::Linear,
                1.0,
            ),
            param(19, left, session::ParameterUnit::Hz, 5_000.0),
            param(19, right, session::ParameterUnit::Hz, 700.0),
            param(20, left, session::ParameterUnit::Db, -4.0),
            param(20, right, session::ParameterUnit::Db, 8.0),
        ]);
        let effect = |id: &str, identity, params| session::Effect {
            id: session::StableId::parse(id).expect("id"),
            identity,
            quality: session::EffectQuality::Normal,
            bypass: false,
            link_mode: session::LinkMode::DualMono,
            params,
            sidechain: session::SidechainDeclaration::None,
        };
        let native = |effect_id: &str| EffectIdentity::Native {
            effect_id: session::StableId::parse(effect_id).expect("effect"),
        };
        model.tracks[1].inserts.effects = vec![
            effect("ins", native("miso.parametric-eq"), eq),
            effect("comp", native("miso.compressor"), Vec::new()),
            effect(
                "pkg",
                EffectIdentity::ThirdPartyCid {
                    cid: "bafkreigh2akiscaildc".to_owned(),
                },
                Vec::new(),
            ),
        ];
        model
    }

    fn wire_lane(lane: session::ParameterChannel) -> ParameterChannel {
        match lane {
            session::ParameterChannel::Left => ParameterChannel::Left,
            session::ParameterChannel::Right => ParameterChannel::Right,
            session::ParameterChannel::Both => ParameterChannel::Both,
        }
    }

    fn strip_address(strip: &str, section: ControlSection, slot: Option<&str>) -> ControlAddress {
        ControlAddress {
            owner_kind: ControlOwnerKind::Strip,
            owner_id: Some(strip.to_owned()),
            section,
            slot_id: slot.map(str::to_owned),
            control: None,
            lane: None,
        }
    }

    fn address_request(
        address: ControlAddress,
        overrides: Vec<ResponseOverride>,
    ) -> ResponseGetRequest {
        ResponseGetRequest {
            target: ResponseTarget::Address(address),
            sections: true,
            overrides,
            configuration_id: Some(0x1575),
            ..response_request()
        }
    }

    /// The configuration request that names `effect` (an EQ) or the input filters with `rows` as
    /// its overrides.
    fn configuration_request(effect: bool, overrides: Vec<ResponseOverride>) -> ResponseGetRequest {
        let mut request = response_request();
        if !effect {
            request.target = ResponseTarget::Configuration(ResponseConfiguration::InputFilters);
        }
        request.sections = true;
        request.overrides = overrides;
        request.configuration_id = Some(0x1575);
        request
    }

    /// The committed values of an effect entry as override rows, read from the model as written:
    /// every declared parameter row, and the bypass.
    fn entry_rows(params: &[session::EffectParam], bypass: bool) -> Vec<ResponseOverride> {
        let mut rows = vec![ResponseOverride {
            control: protocol::RESPONSE_BYPASS_CONTROL,
            lane: ParameterChannel::Both,
            value: f32::from(u8::from(bypass)),
        }];
        rows.extend(params.iter().map(|item| ResponseOverride {
            control: item.parameter_id,
            lane: wire_lane(item.channel),
            value: item.value,
        }));
        rows
    }

    fn cutoff_rows(builtins: &session::DualMonoBuiltins) -> Vec<ResponseOverride> {
        let row = |control, lane, value| ResponseOverride {
            control,
            lane,
            value,
        };
        vec![
            row(3, ParameterChannel::Left, builtins.left.hpf_hz),
            row(4, ParameterChannel::Left, builtins.left.lpf_hz),
            row(3, ParameterChannel::Right, builtins.right.hpf_hz),
            row(4, ParameterChannel::Right, builtins.right.lpf_hz),
        ]
    }

    /// Every field of a curve reply, floats by their bits.
    fn reply_bits(result: &ResponseResult) -> Vec<Vec<u32>> {
        let ResponseResult::Curves(curves) = result else {
            panic!("expected curves");
        };
        let bits = |values: &[f32]| values.iter().map(|value| value.to_bits()).collect();
        let side = |values: &Option<Vec<f32>>| values.as_deref().map_or_else(Vec::new, bits);
        vec![
            vec![
                curves.sample_rate_hz,
                curves.configuration_id as u32,
                curves.floor_db.to_bits(),
                u32::from(curves.bypass),
                curves.section_count,
                curves.enabled_left,
                curves.enabled_right,
                curves.retained_bytes as u32,
            ],
            bits(&curves.frequencies_hz),
            side(&curves.left_db),
            side(&curves.right_db),
            side(&curves.sections_left_db),
            side(&curves.sections_right_db),
        ]
    }

    fn served(model: &session::SessionModel, request: &ResponseGetRequest) -> Vec<Vec<u32>> {
        reply_bits(&serve_response(1 << 20, model, request).expect("served"))
    }

    /// #1575 gate 1: the address of an insert, of a strip's console entry and of a strip's
    /// builtins is answered with the curves and masks of the configuration request that gives the
    /// same effect and every committed value as overrides, bit for bit.
    ///
    /// Red if an address resolves other values than the ones a rebuild prepares: the defaults,
    /// another strip's entry of the same slot, or the builtin defaults instead of the strip's
    /// cutoffs.
    #[test]
    fn an_address_answers_the_curves_of_its_committed_values() {
        let model = address_session();
        let insert = &model.tracks[1].inserts.effects[0];
        let entry = &model.tracks[2].console[0];
        for (name, address, expected) in [
            (
                "insert",
                strip_address("eq1", ControlSection::Inserts, Some("ins")),
                configuration_request(true, entry_rows(&insert.params, insert.bypass)),
            ),
            (
                "console entry",
                strip_address("eq2", ControlSection::Console, Some("eq")),
                configuration_request(true, entry_rows(&entry.params, entry.bypass)),
            ),
            (
                "builtins",
                strip_address("eq0", ControlSection::Builtins, None),
                configuration_request(false, cutoff_rows(&model.tracks[0].builtins)),
            ),
        ] {
            let reply = served(&model, &address_request(address, Vec::new()));
            assert_eq!(reply, served(&model, &expected), "{name}");
            assert_ne!(
                reply,
                served(
                    &model,
                    &configuration_request(name != "builtins", Vec::new())
                ),
                "{name}: the committed values are not the defaults"
            );
        }
    }

    /// #1575 gate 3: an override gives the curve of the committed values with that value, and
    /// neither the model nor anything else changes; `both` on a per-lane parameter is its `left`
    /// and its `right` row; each override is checked as a control write by the one validator.
    ///
    /// Red if an override bypasses `controls::check` (a gain outside its range is answered), is
    /// looked up anywhere but the section's control rows, replaces the committed values instead of
    /// one, or writes the model.
    #[test]
    fn an_address_override_is_a_checked_what_if_over_the_committed_values() {
        let model = address_session();
        let before = model.clone();
        let insert = &model.tracks[1].inserts.effects[0];
        let row = |control, lane, value| ResponseOverride {
            control,
            lane,
            value,
        };
        let ins = || strip_address("eq1", ControlSection::Inserts, Some("ins"));
        let mut committed = entry_rows(&insert.params, insert.bypass);
        committed.retain(|item| !(item.control == 20 && item.lane == ParameterChannel::Left));
        committed.push(row(20, ParameterChannel::Left, 11.0));
        assert_eq!(
            served(
                &model,
                &address_request(ins(), vec![row(20, ParameterChannel::Left, 11.0)])
            ),
            served(&model, &configuration_request(true, committed)),
            "one band gain over the committed values"
        );
        assert_eq!(
            served(
                &model,
                &address_request(ins(), vec![row(20, ParameterChannel::Both, 2.5)])
            ),
            served(
                &model,
                &address_request(
                    ins(),
                    vec![
                        row(20, ParameterChannel::Left, 2.5),
                        row(20, ParameterChannel::Right, 2.5),
                    ]
                )
            ),
            "`both` is `left` and `right`"
        );
        let bypassed = &model.tracks[3].console[0];
        let eq3 = || strip_address("eq3", ControlSection::Console, Some("eq"));
        assert_eq!(
            served(&model, &address_request(eq3(), Vec::new())),
            served(
                &model,
                &configuration_request(true, entry_rows(&bypassed.params, true))
            ),
            "the committed bypass"
        );
        assert_eq!(
            served(
                &model,
                &address_request(eq3(), vec![row(0, ParameterChannel::Both, 0.0)])
            ),
            served(
                &model,
                &configuration_request(true, entry_rows(&bypassed.params, false))
            ),
            "a bypass override over the committed bypass"
        );
        let mut cutoffs = cutoff_rows(&model.tracks[0].builtins);
        cutoffs[0].value = 200.0;
        assert_eq!(
            served(
                &model,
                &address_request(
                    strip_address("eq0", ControlSection::Builtins, None),
                    vec![row(3, ParameterChannel::Left, 200.0)]
                )
            ),
            served(&model, &configuration_request(false, cutoffs)),
            "one cutoff over the committed cutoffs"
        );
        for (name, address, overrides, code) in [
            (
                "a lane on the shared bypass",
                ins(),
                vec![row(0, ParameterChannel::Left, 1.0)],
                "control.lane",
            ),
            (
                "a gain outside its range",
                ins(),
                vec![row(20, ParameterChannel::Left, 1_000.0)],
                "control.domain",
            ),
            (
                "no such parameter",
                ins(),
                vec![row(9_999, ParameterChannel::Both, 1.0)],
                "control.address.unknown",
            ),
            (
                "a link mode the compressor does not have",
                strip_address("eq1", ControlSection::Inserts, Some("comp")),
                vec![row(
                    controls::LINK_MODE_CONTROL,
                    ParameterChannel::Both,
                    9.0,
                )],
                "control.domain",
            ),
            (
                "a link-mode row on the EQ, which has one link mode",
                ins(),
                vec![row(
                    controls::LINK_MODE_CONTROL,
                    ParameterChannel::Both,
                    1.0,
                )],
                "control.address.unknown",
            ),
            (
                "a builtin control other than the cutoffs",
                strip_address("eq0", ControlSection::Builtins, None),
                vec![row(5, ParameterChannel::Left, 1.0)],
                "response.override.control",
            ),
            (
                "`both` beside a lane",
                ins(),
                vec![
                    row(20, ParameterChannel::Left, 1.0),
                    row(20, ParameterChannel::Both, 1.0),
                ],
                "response.override.duplicate",
            ),
        ] {
            let error = serve_response(1 << 20, &model, &address_request(address, overrides))
                .expect_err(name);
            assert_eq!(
                (error.status, error.code),
                (StatusCode::InvalidField, code),
                "{name}"
            );
        }
        assert_eq!(model, before, "a response read writes nothing");

        // A console entry's link-mode override `0` (`slot`, #1369) is the slot's declared mode:
        // the session's EQ slot becomes a `maximum` compressor, whose entry the override passes
        // the validator and resolves, and the compressor (no response) is then refused as an
        // effect, not as an override.
        let mut comp_console = model.clone();
        let slot = &mut comp_console.console.pre_insert[0];
        slot.identity = EffectIdentity::Native {
            effect_id: session::StableId::parse("miso.compressor").expect("effect"),
        };
        slot.link_mode = session::LinkMode::Maximum;
        for track in &mut comp_console.tracks {
            track.console[0].params.clear();
        }
        let slot_override = row(controls::LINK_MODE_CONTROL, ParameterChannel::Both, 0.0);
        let error = serve_response(
            1 << 20,
            &comp_console,
            &address_request(
                strip_address("eq1", ControlSection::Console, Some("eq")),
                vec![slot_override],
            ),
        )
        .expect_err("a compressor has no response");
        assert_eq!(
            (error.status, error.code),
            (StatusCode::Unavailable, "response.effect.unsupported")
        );
        let resolved = resolve_response_target(
            &comp_console,
            "eq1",
            ResponseAddressSection::Console("eq"),
            &[ResponseParameterOverride {
                parameter_id: controls::LINK_MODE_CONTROL,
                channel: contract_channel(ParameterChannel::Both),
                value: 0.0,
            }],
            RATE,
        )
        .expect("the slot override resolves");
        assert!(
            matches!(
                resolved,
                crate::ResolvedResponseTarget::Effect {
                    link_mode: effect_contract::LinkMode::Maximum,
                    ..
                }
            ),
            "`slot` resolves the slot's declared mode"
        );
    }

    /// #1575 gate 4: an address that names no response owner is refused with its own status and
    /// code, and one that names one is never answered as another.
    ///
    /// Red if an unknown strip, slot or insert is answered or misclassified, a route owner or the
    /// own section is resolved, a compressor is answered, or a `cid` insert reaches preparation.
    #[test]
    fn an_address_that_names_no_response_owner_is_refused() {
        let model = address_session();
        let route = ControlAddress {
            owner_kind: ControlOwnerKind::Route,
            owner_id: Some("eq0-main".to_owned()),
            section: ControlSection::Own,
            slot_id: None,
            control: None,
            lane: None,
        };
        for (name, address, status, code) in [
            (
                "an unknown strip",
                strip_address("nobody", ControlSection::Builtins, None),
                StatusCode::NotFound,
                "control.address.unknown",
            ),
            (
                "an unknown console slot",
                strip_address("eq0", ControlSection::Console, Some("nope")),
                StatusCode::NotFound,
                "control.address.unknown",
            ),
            (
                "an insert the strip does not have",
                strip_address("eq0", ControlSection::Inserts, Some("ins")),
                StatusCode::NotFound,
                "control.address.unknown",
            ),
            (
                "a route owner",
                route,
                StatusCode::InvalidField,
                "response.address",
            ),
            (
                "the own section",
                strip_address("eq0", ControlSection::Own, None),
                StatusCode::InvalidField,
                "response.address",
            ),
            (
                "a compressor insert",
                strip_address("eq1", ControlSection::Inserts, Some("comp")),
                StatusCode::Unavailable,
                "response.effect.unsupported",
            ),
            (
                "a cid insert",
                strip_address("eq1", ControlSection::Inserts, Some("pkg")),
                StatusCode::Unavailable,
                "effect.third_party.unavailable_at_launch",
            ),
        ] {
            let error = serve_response(1 << 20, &model, &address_request(address, Vec::new()))
                .expect_err(name);
            assert_eq!((error.status, error.code), (status, code), "{name}");
        }
    }

    fn nine_track() -> session::SessionModel {
        session::parse_session_json(include_str!(
            "../../../fixtures/session/v1/parametric-eq-nine-track.json"
        ))
        .expect("session fixture")
    }

    fn provider_of(model: &session::SessionModel) -> (SessionControlProvider, Arc<Clock>) {
        provider_under(model, &MonitorOverlay::default())
    }

    fn provider_under(
        model: &session::SessionModel,
        overlay: &MonitorOverlay,
    ) -> (SessionControlProvider, Arc<Clock>) {
        let catalog = ControlCatalog::build(model, overlay, RATE).expect("catalog");
        let clock = Arc::new(Clock(AtomicU64::new(0)));
        let sample_source: Arc<dyn PlanSampleSource> = Arc::clone(&clock) as Arc<_>;
        let provider = SessionControlProvider::try_new(
            catalog,
            1,
            sample_source,
            ControllerRetainedCapacity {
                meters: 4,
                counter_ids: 0,
            },
            2,
            1 << 20,
            crate::response_capture_channel()
                .expect("capture channel")
                .0,
        )
        .expect("provider");
        (provider, clock)
    }

    /// #1578 D3: a strip `RESPONSE_GET` meets the core's checks, the session-rate rule, the
    /// strip and its event bound before it arms; each refusal arms nothing, so the first request
    /// that passes is tag 1; that one answers its tag, and the next, while the capture is in
    /// flight, is `BACKPRESSURE` `response.capture.pending`.
    ///
    /// Test value: red if a refused strip request arms a capture (leaving a tag no event will
    /// answer), if the event bound is not checked against `maximum_reply_bytes` before arming, or if
    /// a refusal maps to the wrong status or code.
    #[test]
    fn a_strip_request_is_checked_before_it_arms() {
        let model = nine_track();
        let (mut provider, _clock) = provider_of(&model);
        let strip = |strip: &str| ResponseGetRequest {
            target: ResponseTarget::Strip(strip.to_owned()),
            ..response_request()
        };
        let mut refusal = |request: &ResponseGetRequest| {
            let error = provider.response_get(&model, request).expect_err("refused");
            (error.status, error.code)
        };
        let mut points = strip("eq0");
        points.points = 1;
        assert_eq!(
            refusal(&points),
            (StatusCode::InvalidField, "response.points")
        );
        let mut rate = strip("eq0");
        rate.sample_rate_hz = Some(RATE);
        assert_eq!(
            refusal(&rate),
            (StatusCode::InvalidField, "response.rate.session")
        );
        assert_eq!(
            refusal(&strip("nope")),
            (StatusCode::NotFound, "control.address.unknown")
        );
        let mut small = strip("eq0");
        let owners = crate::response_capture::strip_owners(&model, "eq0").expect("eq0");
        let bound = protocol::response_ready_encoded_bound(64, ParameterChannel::Both, owners)
            .expect("a bound");
        small.maximum_reply_bytes = Some(u32::try_from(bound - 1).expect("bound"));
        assert_eq!(
            refusal(&small),
            (StatusCode::LimitExceeded, "response.reply_too_large")
        );
        small.maximum_reply_bytes = Some(u32::try_from(bound).expect("bound"));
        assert_eq!(
            provider.response_get(&model, &small),
            Ok(ResponseResult::Tag(1)),
            "a bound that fits arms tag 1: no refusal armed"
        );
        let error = provider
            .response_get(&model, &strip("eq1"))
            .expect_err("pending");
        assert_eq!(
            (error.status, error.code),
            (StatusCode::Backpressure, "response.capture.pending")
        );
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
                headers_only: false,
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
        // One insert, so the inserts section's header appears beside own, builtins and console.
        let mut insert = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
        insert.id = session::StableId::parse("insert-a").expect("insert ID");
        insert.params.clear();
        model.tracks[0].inserts.effects.push(insert);
        let catalog =
            ControlCatalog::build(&model, &MonitorOverlay::default(), RATE).expect("catalog");
        let expected = controls::list(&model, &MonitorOverlay::default(), RATE).collect::<Vec<_>>();
        let entries = catalog.entries();
        let controls_listed = entries
            .iter()
            .filter(|entry| matches!(entry.body, ControlBody::Control(_)))
            .count();
        assert_eq!(controls_listed, expected.len());
        assert_eq!(catalog.values.len(), entries.len());

        let mut next_control = expected.iter();
        let mut owner: Option<&ControlAddress> = None;
        let mut section: Option<&ControlAddress> = None;
        let mut sections = Vec::new();
        for (index, entry) in entries.iter().enumerate() {
            assert_eq!(entry.handle as usize, index + 1, "handles count from 1");
            match &entry.body {
                ControlBody::OwnerHeader(_) => {
                    assert_eq!(entry.address.section, ControlSection::Own);
                    owner = Some(&entry.address);
                    section = None;
                }
                ControlBody::SectionHeader(header) => {
                    let owner = owner.expect("a section header follows its owner header");
                    assert_eq!(entry.address.owner_kind, owner.owner_kind);
                    assert_eq!(entry.address.owner_id, owner.owner_id);
                    let name = match entry.address.section {
                        ControlSection::Own => "own",
                        ControlSection::Builtins => "builtins",
                        ControlSection::Console | ControlSection::Inserts => {
                            entry.address.slot_id.as_deref().expect("a slot ID")
                        }
                    };
                    assert_eq!(header.name, name);
                    sections.push(entry.address.section);
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
        for kind in [
            ControlSection::Own,
            ControlSection::Builtins,
            ControlSection::Console,
            ControlSection::Inserts,
        ] {
            assert!(sections.contains(&kind), "{kind:?}");
        }
        // Every owner kind has a header: strips, the VCA, routes and the session.
        let kinds = entries
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
        assert_eq!(descriptor_ids.len(), 11, "complete launch registry");
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
                    link_mode: session::ConsoleLinkMode::Slot,
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
        let catalog =
            ControlCatalog::build(&model, &MonitorOverlay::default(), RATE).expect("catalog");
        let entries = catalog.entries();
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
                        &entries,
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
    fn a_headers_only_request_round_trips_and_pages_exactly_the_lists_header_rows() {
        let model = nine_track();
        let (mut provider, _clock) = provider_of(&model);
        let whole = whole_list(&mut provider, 256);
        let codec = protocol::ProtocolCodec::default();
        let headers_of = |owner: Option<&str>| {
            whole
                .iter()
                .filter(|entry| !matches!(entry.body, ControlBody::Control(_)))
                .filter(|entry| {
                    owner.is_none_or(|id| entry.address.owner_id.as_deref() == Some(id))
                })
                .cloned()
                .collect::<Vec<_>>()
        };
        for (owner, limit) in [(None, 4), (Some("eq3"), 2)] {
            let expected = headers_of(owner);
            let (mut listed, mut pages, mut epoch, mut after_handle) = (Vec::new(), 0, 0, 0);
            loop {
                let request = ControlListRequest {
                    structure_epoch: epoch,
                    after_handle,
                    limit,
                    owner: owner.map(|id| protocol::ControlOwnerFilter {
                        kind: ControlOwnerKind::Strip,
                        id: Some(id.to_owned()),
                    }),
                    headers_only: true,
                };
                let mut bytes = vec![
                    0;
                    codec
                        .encoded_control_list_request_len(&request)
                        .expect("len")
                ];
                codec
                    .encode_control_list_request(&request, &mut bytes)
                    .expect("encode");
                let decoded = codec
                    .decode_control_list_request(&bytes, request.owner.as_ref().map_or(4, |_| 6))
                    .expect("decode");
                assert_eq!(decoded, request, "the field round-trips");
                let next = provider.control_list(&decoded).expect("a page");
                let state = provider.control_state(&decoded).expect("a state page");
                assert_eq!(
                    (state.last_handle, state.eof, state.records.len()),
                    (next.last_handle, next.eof, 0),
                    "a headers-only state page covers the header rows, which have no record"
                );
                pages += 1;
                epoch = next.structure_epoch;
                after_handle = next.last_handle;
                listed.extend(next.entries);
                if next.eof {
                    break;
                }
            }
            assert!(expected.len() > usize::from(limit) && expected.len() < whole.len());
            assert_eq!(listed, expected, "the header rows, their handles and order");
            assert_eq!(
                pages,
                expected.len().div_ceil(usize::from(limit)),
                "no trailing page"
            );
        }
        // The flag is written only as `true`: a present `false` is a second
        // spelling of absence and is refused.
        let request = ControlListRequest {
            structure_epoch: 0,
            after_handle: 0,
            limit: 1,
            owner: None,
            headers_only: true,
        };
        let mut bytes = vec![
            0;
            codec
                .encoded_control_list_request_len(&request)
                .expect("len")
        ];
        codec
            .encode_control_list_request(&request, &mut bytes)
            .expect("encode");
        // The flag is the last field, its value byte padded to eight.
        let flag = bytes.len() - 8;
        assert_eq!(bytes[flag], 1);
        bytes[flag] = 0;
        assert_eq!(
            codec.decode_control_list_request(&bytes, 4),
            Err(protocol::DecodeError::InvalidField)
        );
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
                headers_only: false,
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
                    headers_only: false,
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
        provider.replace_session_catalog(
            ControlCatalog::build(&model, &MonitorOverlay::default(), RATE).expect("catalog"),
            2,
        );
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
        let same =
            ControlCatalog::build(&model, &MonitorOverlay::default(), RATE).expect("catalog");
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
        provider.replace_session_catalog(
            ControlCatalog::build(&model, &MonitorOverlay::default(), RATE).expect("catalog"),
            4,
        );
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
        let changed =
            ControlCatalog::build(&model, &MonitorOverlay::default(), RATE).expect("catalog");
        provider.replace_session_catalog(changed, 5);
        assert_eq!(provider.structure_epoch(), 3);
        assert_eq!(
            provider
                .control_list(&ControlListRequest {
                    structure_epoch: 2,
                    after_handle: 0,
                    limit: 1,
                    owner: None,
                    headers_only: false,
                })
                .err(),
            Some(ParameterProviderError::Stale)
        );
    }

    #[test]
    fn clock_transport_counters_and_diagnostics_are_session_backed() {
        let (mut provider, clock) = provider_of(&nine_track());
        clock.0.store(256, Ordering::Release);
        assert_eq!(provider.current_sample(), SampleTime(256));
        let transport = provider.transport_set(TransportSetRequest {
            state: TransportState::Playing,
        });
        assert_eq!(transport.effective_sample, SampleTime(256));
        assert_eq!(transport.state, TransportState::Playing);
        assert_eq!(provider.transport_get(), transport);

        let counter_capacity = provider.retained_capacities().1;
        assert_eq!(counter_capacity, PROVIDER_COUNTER_SLOTS);
        provider.set_telemetry_counters(TelemetryCounters {
            telemetry_coalesced: 7,
            telemetry_dropped: 9,
        });
        provider.set_live_values_superseded(13);
        provider.set_strips_restarted(5);
        provider.set_live_block_errors(17);
        provider.set_source_underrun_blocks(19);
        provider.set_source_frames_discarded(23);
        provider.set_meter_windows_lost(29);
        let counters = provider
            .counters(&CountersRequest {
                all: true,
                ids: Vec::new(),
            })
            .expect("counters");
        assert_eq!(counters.observed_sample, SampleTime(256));
        assert_eq!(counters.values[0].value, 7);
        assert_eq!(counters.values[1].value, 9);
        assert_eq!(counters.values[2].id, CounterId::LiveValuesSuperseded);
        assert_eq!(counters.values[2].value, 13);
        assert_eq!(
            counters.values[3].id,
            CounterId::TransitionReprepareRefusals
        );
        assert_eq!(counters.values[3].value, 0);
        assert_eq!(counters.values[4].id, CounterId::StripsRestarted);
        assert_eq!(counters.values[4].value, 5);
        assert_eq!(counters.values[5].id, CounterId::LiveBlockErrors);
        assert_eq!(counters.values[5].value, 17);
        assert_eq!(counters.values[6].id, CounterId::SourceUnderrunBlocks);
        assert_eq!(counters.values[6].value, 19);
        assert_eq!(counters.values[7].id, CounterId::SourceFramesDiscarded);
        assert_eq!(counters.values[7].value, 23);
        assert_eq!(counters.values[8].id, CounterId::MeterWindowsLost);
        assert_eq!(counters.values[8].value, 29);
        assert_eq!(counters.values.len(), 9);
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
    /// The catalog holds a row's descriptor once for every row that converts to the same wire
    /// descriptor and holds a different one for every row whose conversion differs.
    ///
    /// Red if deduplication merges two descriptors that differ in one field (`active` of two
    /// strips' matrix rows, a `reopen` text, a default ramp), or merges nothing.
    #[test]
    fn rows_share_a_descriptor_exactly_when_their_wire_descriptors_are_equal() {
        let mut model = nine_track();
        let registry = effect_compiler::launch_native_effect_registry().expect("registry");
        let mut insert_ids = registry
            .descriptors()
            .map(|descriptor| descriptor.id.as_str().to_owned());
        let first_insert = insert_ids.next().expect("an effect");
        let second_insert = insert_ids
            .find(|id| *id != first_insert)
            .expect("a second effect");
        // Two tracks that differ in `matrix_mode`, and two different inserts on the first.
        let session::MatrixOrPan::Pan {
            smoothing_samples, ..
        } = model.tracks[1].matrix_or_pan
        else {
            panic!("the fixture's tracks pan");
        };
        model.tracks[1].matrix_or_pan = session::MatrixOrPan::Matrix {
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
            smoothing_samples,
        };
        let template = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
        for (name, identity) in [("insert-a", &first_insert), ("insert-b", &second_insert)] {
            let mut effect = template.clone();
            effect.id = session::StableId::parse(name).expect("insert ID");
            effect.identity = session::EffectIdentity::Native {
                effect_id: session::StableId::parse(identity).expect("native effect ID"),
            };
            effect.params.clear();
            model.tracks[0].inserts.effects.push(effect);
        }
        // A submix, a route into it and one into the output, and a VCA.
        let bus = session::StableId::parse("bus").expect("submix ID");
        let track = model.tracks[0].clone();
        model.submixes.push(session::Submix {
            id: bus.clone(),
            builtins: track.builtins.clone(),
            console: track.console.clone(),
            inserts: session::Rack {
                effects: Vec::new(),
            },
            fader: track.fader.clone(),
            matrix_or_pan: track.matrix_or_pan.clone(),
        });
        let mut into_submix = model.routes[0].clone();
        into_submix.id = session::StableId::parse("into-bus").expect("route ID");
        into_submix.destination = session::RouteDestination::SubmixInput {
            submix_id: bus.clone(),
        };
        let mut out_of_submix = model.routes[0].clone();
        out_of_submix.id = session::StableId::parse("bus-out").expect("route ID");
        out_of_submix.source = session::RouteSource::Submix {
            submix_id: bus,
            tap: session::SendTap::PostPan,
        };
        model.routes.push(into_submix);
        model.routes.push(out_of_submix);
        model.vcas.push(session::Vca {
            id: session::StableId::parse("vca0").expect("ID"),
            fader: model.tracks[0].fader.clone(),
            members: vec![model.tracks[0].id.clone()],
        });

        let catalog =
            ControlCatalog::build(&model, &MonitorOverlay::default(), RATE).expect("catalog");
        let listed = controls::list(&model, &MonitorOverlay::default(), RATE).collect::<Vec<_>>();
        let ramps = LiveRamps::for_session(&model);
        let control_rows = catalog
            .rows
            .iter()
            .filter(|record| record.kind == RowKind::Control)
            .collect::<Vec<_>>();
        assert_eq!(control_rows.len(), listed.len());
        // The oracle converts each listed row on its own, with its section's effect contract.
        let converted = listed
            .iter()
            .zip(&control_rows)
            .map(|(row, record)| {
                let effect = catalog.sections[record.parent as usize]
                    .effect
                    .as_deref()
                    .and_then(|id| registry.get_ascii(id))
                    .map(|factory| factory.descriptor());
                Shape::of(row, ramps, effect).to_row().expect("a wire row")
            })
            .collect::<Vec<_>>();
        let stored = control_rows
            .iter()
            .map(|record| &catalog.descriptors[record.descriptor as usize])
            .collect::<Vec<_>>();
        let (mut saw_active_split, mut saw_shared) = (false, false);
        for left in 0..converted.len() {
            for right in left + 1..converted.len() {
                let same = converted[left] == converted[right];
                assert_eq!(
                    stored[left] == stored[right],
                    same,
                    "{:?} against {:?}",
                    listed[left].address,
                    listed[right].address
                );
                saw_shared |= same;
                saw_active_split |= !same
                    && converted[left].name == converted[right].name
                    && converted[left].active != converted[right].active;
            }
        }
        assert!(
            saw_shared && saw_active_split,
            "the fixture shares rows and splits two rows that differ in `active` alone"
        );
        for (stored, converted) in stored.iter().zip(&converted) {
            assert!(same_descriptor(stored, converted));
        }
        assert!(
            catalog.descriptors.len() * 4 < listed.len(),
            "{} descriptors for {} rows",
            catalog.descriptors.len(),
            listed.len()
        );
    }

    /// Two descriptors that differ only in a float's bits (`0.0` and `-0.0`) are two descriptors.
    ///
    /// Red if deduplication or the epoch comparison reads an `f32` with `==`.
    #[test]
    fn descriptors_that_differ_in_float_bits_alone_stay_distinct() {
        let model = nine_track();
        let listed = controls::list(&model, &MonitorOverlay::default(), RATE)
            .next()
            .expect("a row");
        let base = Shape::of(&listed, LiveRamps::for_session(&model), None);
        let with_default = |default: f32| Shape {
            default: default.to_bits(),
            ..base
        };
        let mut table = ShapeTable::default();
        let first = table.intern(with_default(0.0)).expect("interned");
        let second = table.intern(with_default(-0.0)).expect("interned");
        assert_ne!(first, second);
        let rows = table.into_rows().expect("rows");
        assert!(!same_descriptor(&rows[0], &rows[1]));
        assert!(
            rows[0] == rows[1],
            "the derived `==` cannot tell them apart"
        );
    }

    /// A commit that changes only `control_smoothing` moves every fader row's default ramp and no
    /// address, and the epoch advances; a second catalog of the same model keeps it.
    ///
    /// Red if the epoch comparison reads descriptor indices (a rebuild may renumber them) and so
    /// misses a changed descriptor, or reads values and so advances on every commit.
    #[test]
    fn a_changed_default_ramp_advances_the_epoch_and_an_equal_model_keeps_it() {
        let mut model = nine_track();
        let smoothing = |fader_ms: f32| session::ControlSmoothing {
            mute_ms: 5.0,
            fader_ms,
            pan_ms: 20.0,
        };
        model.control_smoothing = Some(smoothing(10.0));
        let (mut provider, _clock) = provider_of(&model);
        let before = whole_list(&mut provider, 256);
        provider.replace_session_catalog(
            ControlCatalog::build(&model, &MonitorOverlay::default(), RATE).expect("catalog"),
            2,
        );
        assert_eq!(
            provider.structure_epoch(),
            1,
            "an equal model keeps the epoch"
        );

        model.control_smoothing = Some(smoothing(40.0));
        provider.replace_session_catalog(
            ControlCatalog::build(&model, &MonitorOverlay::default(), RATE).expect("catalog"),
            3,
        );
        assert_eq!(provider.structure_epoch(), 2, "a default ramp changed");
        let after = whole_list(&mut provider, 256);
        let ramps = |list: &[ControlEntry]| {
            list.iter()
                .filter_map(|entry| match &entry.body {
                    ControlBody::Control(row) if row.name == "fader_db" => {
                        Some(row.default_ramp_samples)
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
        };
        assert_ne!(ramps(&before), ramps(&after));
        provider.replace_session_catalog(
            ControlCatalog::build(&model, &MonitorOverlay::default(), RATE).expect("catalog"),
            4,
        );
        assert_eq!(
            provider.structure_epoch(),
            2,
            "a second equal catalog keeps it"
        );
    }

    // Issue #1600: a live commit refreshes only the changed owners' rows.

    /// Nine tracks, a submix that eq0's send feeds with `follows_mute`, and a VCA over eq1.
    fn staged_session() -> session::SessionModel {
        let mut model = nine_track();
        let track = model.tracks[0].clone();
        let bus = session::StableId::parse("bus").expect("ID");
        model.submixes.push(session::Submix {
            id: bus.clone(),
            builtins: track.builtins.clone(),
            console: track.console.clone(),
            inserts: session::Rack {
                effects: Vec::new(),
            },
            fader: track.fader.clone(),
            matrix_or_pan: track.matrix_or_pan.clone(),
        });
        let mut send = model.routes[0].clone();
        send.id = session::StableId::parse("eq0-bus").expect("ID");
        send.destination = session::RouteDestination::SubmixInput { submix_id: bus };
        send.follows_mute = true;
        model.routes.push(send);
        model.vcas.push(session::Vca {
            id: session::StableId::parse("vca0").expect("ID"),
            fader: track.fader.clone(),
            members: vec![model.tracks[1].id.clone()],
        });
        compile(&model).normalized_model().clone()
    }

    /// The control `name` of owner `owner` in `section`, on `lane`.
    fn address_of(
        model: &session::SessionModel,
        owner: &Owner,
        section: &Section,
        name: &str,
        lane: session::ParameterChannel,
    ) -> controls::ControlAddress {
        controls::list(model, &MonitorOverlay::default(), RATE)
            .find(|row| {
                row.address.owner == *owner
                    && row.address.section == *section
                    && row.descriptor.name == name
                    && row.address.lane == lane
            })
            .unwrap_or_else(|| panic!("{name}"))
            .address
    }

    fn strip_id(id: &str) -> Owner {
        Owner::Strip(session::StableId::parse(id).expect("ID"))
    }

    fn write(model: &mut session::SessionModel, address: &controls::ControlAddress, value: f32) {
        controls::set(model, address, value, None, RATE).expect("an accepted value");
    }

    /// What one staged live commit gave: the provider that published it, the owners the stage
    /// read and the full catalogs it took.
    struct Staged {
        provider: SessionControlProvider,
        owners_read: usize,
        full_stages: usize,
        /// The list of a provider built fresh from the committed model, and its epoch.
        fresh: (Vec<ControlEntry>, u64),
    }

    /// Commit `edit` on `before` as a live commit would: classify, stage from the current
    /// catalog, publish, and compare with a provider built from the committed model.
    fn live_commit(
        before: &session::SessionModel,
        edit: impl FnOnce(&mut session::SessionModel),
    ) -> Staged {
        let none = MonitorOverlay::default();
        live_commit_under(before, &none, &none, edit)
    }

    /// As [`live_commit`], with the monitor overlay the provider's catalog was built under
    /// (`committed`) and the one the commit leaves (`left`): the stage and the fresh build both
    /// read `left` (#1600 D1, #1559 D11).
    fn live_commit_under(
        before: &session::SessionModel,
        committed: &MonitorOverlay,
        left: &MonitorOverlay,
        edit: impl FnOnce(&mut session::SessionModel),
    ) -> Staged {
        let mut next = before.clone();
        edit(&mut next);
        let (mut provider, _clock) = provider_under(before, committed);
        let delta = crate::classify_live_delta_with_monitor(
            before,
            &next,
            LiveRamps::for_session(&next),
            &session::EditRamps::new(),
            crate::MonitorBase {
                committed,
                next: left,
                ramp: None,
            },
        )
        .expect("a live delta");
        take_stage_counts();
        let staged = provider
            .stage_live_catalog(controls::ModelView::whole(&next), left, RATE, &delta)
            .expect("staged");
        let (owners_read, full_stages) = take_stage_counts();
        assert_eq!(
            matches!(staged, StagedCatalog::Full(_)),
            full_stages == 1,
            "the counter agrees with the result"
        );
        provider.publish_staged_catalog(staged, 2);
        assert_eq!(provider.revision, 2);
        let (mut fresh_provider, _clock) = provider_under(&next, left);
        let fresh = (whole_list(&mut fresh_provider, 256), 1);
        Staged {
            provider,
            owners_read,
            full_stages,
            fresh,
        }
    }

    impl Staged {
        /// Every page at limit 256 equals the fresh provider's, entry by entry, and the epoch is
        /// `epoch`.
        fn assert_same_as_fresh(&mut self, epoch: u64) {
            let list = whole_list(&mut self.provider, 256);
            assert_eq!(list.len(), self.fresh.0.len());
            for (staged, fresh) in list.iter().zip(&self.fresh.0) {
                assert_eq!(staged.handle, fresh.handle);
                assert_eq!(staged.address, fresh.address);
                match (&staged.body, &fresh.body) {
                    (ControlBody::Control(left), ControlBody::Control(right)) => {
                        assert_eq!(left.value.to_bits(), right.value.to_bits(), "{}", left.name);
                        assert!(same_descriptor(left, right), "{}", left.name);
                    }
                    (left, right) => assert_eq!(left, right),
                }
            }
            assert_eq!(self.provider.structure_epoch(), epoch);
        }
    }

    /// Gate 1 of #1600. One lane of a fader, a submix's mute, a console EQ parameter, a VCA
    /// offset, two edits on two strips and a send gain each give the list
    /// of a fresh build, keep the epoch, and take no full catalog; a `control_smoothing` change and
    /// a `matrix_mode` switch take the full catalog and advance the epoch.
    ///
    /// Red if the patched list differs from a full rebuild in any value or descriptor, or the epoch
    /// moves when it should not, or stays when it should move.
    #[test]
    fn a_staged_live_commit_gives_the_list_of_a_full_build() {
        use session::ParameterChannel::{Both, Left, Right};
        let model = staged_session();
        type Edit = Box<dyn Fn(&mut session::SessionModel)>;
        let cases: [(&str, Edit); 6] = [
            (
                "a track fader lane",
                Box::new(|m| {
                    let a = address_of(m, &strip_id("eq3"), &Section::Builtins, "fader_db", Left);
                    write(m, &a, -4.5);
                }),
            ),
            (
                "a submix mute",
                Box::new(|m| {
                    let a = address_of(m, &strip_id("bus"), &Section::Builtins, "mute", Right);
                    write(m, &a, 1.0);
                }),
            ),
            (
                "a console EQ parameter",
                Box::new(|m| {
                    let slot = m.console.slots().next().expect("a slot").slot.clone();
                    let a = controls::list(m, &MonitorOverlay::default(), RATE)
                        .find(|row| {
                            row.address.owner == strip_id("eq2")
                                && row.address.section == Section::Console(slot.clone())
                                && row.descriptor.family == controls::Family::Effect
                                && row.descriptor.name != "bypass"
                                && row.descriptor.class == controls::ControlClass::Live
                        })
                        .expect("a live effect row")
                        .address;
                    let current =
                        controls::get(m, &MonitorOverlay::default(), &a, RATE).expect("a value");
                    let descriptor = controls::list(m, &MonitorOverlay::default(), RATE)
                        .find(|row| row.address == a)
                        .expect("row")
                        .descriptor;
                    let value = if current == descriptor.minimum {
                        descriptor.maximum
                    } else {
                        descriptor.minimum
                    };
                    write(m, &a, value);
                }),
            ),
            (
                "a VCA offset",
                Box::new(|m| {
                    let vca = Owner::Vca(session::StableId::parse("vca0").expect("ID"));
                    let a = address_of(m, &vca, &Section::Own, "offset_db", Left);
                    write(m, &a, -6.0);
                }),
            ),
            (
                "two edits on two strips",
                Box::new(|m| {
                    let a = address_of(m, &strip_id("eq0"), &Section::Builtins, "fader_db", Left);
                    write(m, &a, -1.0);
                    let b = address_of(m, &strip_id("eq5"), &Section::Builtins, "mute", Left);
                    write(m, &b, 1.0);
                }),
            ),
            (
                "a send gain",
                Box::new(|m| {
                    let route = Owner::Route(session::StableId::parse("eq0-bus").expect("ID"));
                    let a = address_of(m, &route, &Section::Own, "gain_db", Both);
                    write(m, &a, -3.0);
                }),
            ),
        ];
        for (name, edit) in cases {
            let mut staged = live_commit(&model, edit);
            assert_eq!(staged.full_stages, 0, "{name} takes no full catalog");
            assert!(staged.owners_read >= 1, "{name}");
            staged.assert_same_as_fresh(1);
        }

        let mut smoothing = live_commit(&model, |m| {
            let a = address_of(m, &Owner::Session, &Section::Own, "fader_ms", Both);
            write(m, &a, 55.0);
        });
        assert_eq!(smoothing.full_stages, 1, "a setting takes the full catalog");
        smoothing.assert_same_as_fresh(2);

        let mut matrix = live_commit(&model, |m| {
            let a = address_of(m, &strip_id("eq1"), &Section::Own, "matrix_mode", Both);
            write(m, &a, 1.0);
        });
        assert_eq!(
            matrix.full_stages, 1,
            "a `matrix_mode` switch takes the full catalog"
        );
        matrix.assert_same_as_fresh(2);
    }

    /// Gate 2 of #1600. A strip's mute reaches the gate row of a send that follows its mute and the
    /// strip's own effective rows, though the route is not in the delta's changed owners.
    ///
    /// Red if derived values are refreshed only for the changed owners.
    #[test]
    fn a_followed_mute_refreshes_the_send_gate_of_an_unchanged_route() {
        use session::ParameterChannel::Left;
        let model = staged_session();
        let mut staged = live_commit(&model, |m| {
            let a = address_of(m, &strip_id("eq0"), &Section::Builtins, "mute", Left);
            write(m, &a, 1.0);
        });
        assert_eq!(
            staged.owners_read, 1,
            "only the strip is read as a changed owner"
        );
        assert_eq!(staged.full_stages, 0);
        staged.assert_same_as_fresh(1);
        let list = whole_list(&mut staged.provider, 256);
        let gate = list
            .iter()
            .find(|entry| {
                entry.address.owner_id.as_deref() == Some("eq0-bus")
                    && matches!(&entry.body, ControlBody::Control(row) if row.name == "effective_gate")
                    && entry.address.lane == Some(ParameterChannel::Left)
            })
            .expect("the gate row");
        assert_eq!(
            control_value(gate),
            1.0,
            "the route follows the muted strip"
        );
    }

    /// A solo-only (monitor) commit changes no model row, yet the strip's `solo` row and every
    /// strip's and route's effective rows follow the overlay the commit leaves (#1600 D1, #1559).
    ///
    /// Red if the stage reads the default overlay or skips the strip whose solo changed: the
    /// `solo` row stays 0 and the other strips' effective mutes stay unmuted.
    #[test]
    fn a_solo_only_commit_refreshes_the_solo_row_and_the_derived_rows() {
        let model = staged_session();
        let none = MonitorOverlay::default();
        let eq0 = session::StableId::parse("eq0").expect("ID");
        let solo = MonitorOverlay::with_soloed([eq0]);
        // Solo on, then off again; the model is the same in both.
        for (committed, left) in [(&none, &solo), (&solo, &none)] {
            let mut staged = live_commit_under(&model, committed, left, |_| {});
            assert_eq!(staged.full_stages, 0, "a solo change takes no full catalog");
            assert_eq!(
                staged.owners_read, 1,
                "only the soloed strip is read as changed"
            );
            staged.assert_same_as_fresh(1);
        }
        let mut staged = live_commit_under(&model, &none, &solo, |_| {});
        let list = whole_list(&mut staged.provider, 256);
        let value_of = |owner: &str, name: &str| {
            let entry = list
                .iter()
                .find(|entry| {
                    entry.address.owner_id.as_deref() == Some(owner)
                        && entry.address.lane != Some(ParameterChannel::Right)
                        && matches!(&entry.body, ControlBody::Control(row) if row.name == name)
                })
                .unwrap_or_else(|| panic!("{owner} {name}"));
            control_value(entry)
        };
        assert_eq!(value_of("eq0", "solo"), 1.0);
        assert_eq!(value_of("eq1", "solo"), 0.0);
        assert_eq!(value_of("eq0", "effective_mute"), 0.0);
        assert_eq!(
            value_of("eq1", "effective_mute"),
            1.0,
            "another strip is solo-muted"
        );
    }

    /// Gate 4 of #1600. One fader edit on a 64-track session with 4 submixes reads exactly one
    /// owner and takes no full catalog.
    ///
    /// Red if the live commit still reads every owner.
    #[test]
    fn one_fader_edit_on_a_large_session_reads_one_owner() {
        let mut model = staged_session();
        let template = model.tracks[0].clone();
        for index in 0..55 {
            let mut track = template.clone();
            track.id = session::StableId::parse(&format!("zz{index:02}")).expect("ID");
            model.tracks.push(track);
        }
        for index in 0..3 {
            let mut submix = model.submixes[0].clone();
            submix.id = session::StableId::parse(&format!("bus{index}")).expect("ID");
            model.submixes.push(submix);
        }
        let model = compile(&model).normalized_model().clone();
        assert_eq!((model.tracks.len(), model.submixes.len()), (64, 4));
        let mut staged = live_commit(&model, |m| {
            let a = address_of(
                m,
                &strip_id("zz17"),
                &Section::Builtins,
                "fader_db",
                session::ParameterChannel::Left,
            );
            write(m, &a, -2.0);
        });
        assert_eq!((staged.owners_read, staged.full_stages), (1, 0));
        staged.assert_same_as_fresh(1);
    }

    /// Gate 5 of #1600. A stage that fails reports it before the provider is touched: nothing is
    /// published and the list is the list it was.
    ///
    /// Red if the stage moves after the first value write.
    #[test]
    fn a_failed_stage_leaves_the_catalog_as_it_was() {
        let model = staged_session();
        let mut next = model.clone();
        let a = address_of(
            &next,
            &strip_id("eq3"),
            &Section::Builtins,
            "fader_db",
            session::ParameterChannel::Left,
        );
        write(&mut next, &a, -9.0);
        let (mut provider, _clock) = provider_of(&model);
        let before = whole_list(&mut provider, 256);
        let delta = crate::classify_live_delta(
            &model,
            &next,
            LiveRamps::for_session(&next),
            &session::EditRamps::new(),
        )
        .expect("a live delta");
        fail_next_stages(1);
        assert!(
            provider
                .stage_live_catalog(
                    controls::ModelView::whole(&next),
                    &MonitorOverlay::default(),
                    RATE,
                    &delta,
                )
                .is_err()
        );
        assert_eq!(whole_list(&mut provider, 256), before);
        assert_eq!(provider.revision, 1);
        provider
            .stage_live_catalog(
                controls::ModelView::whole(&next),
                &MonitorOverlay::default(),
                RATE,
                &delta,
            )
            .expect("the failure was one-shot");
    }
}
