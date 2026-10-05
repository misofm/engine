//! Exported-C retained-allocation and disposal ownership evidence.

#![allow(unsafe_code)]

use core::{
    alloc::Layout,
    cell::Cell,
    mem::size_of,
    ptr,
    sync::atomic::{AtomicBool, AtomicU64},
};
use std::alloc::{GlobalAlloc, System};

use capi::*;
use lane::Backend;
use protocol::{
    CommandPayload, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    StatusCode, TypedCommandFrame,
};
use session::StableId;

struct LifecycleAllocator;

#[global_allocator]
static LIFECYCLE_ALLOCATOR: LifecycleAllocator = LifecycleAllocator;

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static ALLOCATIONS: Cell<u64> = const { Cell::new(0) };
    static DEALLOCATIONS: Cell<u64> = const { Cell::new(0) };
    static ALLOCATED_BYTES: Cell<u64> = const { Cell::new(0) };
    static DEALLOCATED_BYTES: Cell<u64> = const { Cell::new(0) };
}

fn record_allocation(bytes: usize) {
    ACTIVE.with(|active| {
        if active.get() {
            ALLOCATIONS.set(ALLOCATIONS.get() + 1);
            ALLOCATED_BYTES.set(ALLOCATED_BYTES.get() + bytes as u64);
        }
    });
}

fn record_deallocation(bytes: usize) {
    ACTIVE.with(|active| {
        if active.get() {
            DEALLOCATIONS.set(DEALLOCATIONS.get() + 1);
            DEALLOCATED_BYTES.set(DEALLOCATED_BYTES.get() + bytes as u64);
        }
    });
}

// SAFETY: Every operation delegates the original pointer/layout unchanged to `System`; the
// thread-local counters are observational and enabled only around this isolated test thread.
unsafe impl GlobalAlloc for LifecycleAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: The allocator-provided layout is forwarded unchanged.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            record_allocation(layout.size());
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: The allocator-provided layout is forwarded unchanged.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            record_allocation(layout.size());
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        record_deallocation(layout.size());
        // SAFETY: The original pointer and layout are forwarded unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: The original allocation arguments and requested size are forwarded unchanged.
        let replacement = unsafe { System.realloc(pointer, layout, new_size) };
        if !replacement.is_null() {
            record_deallocation(layout.size());
            record_allocation(new_size);
        }
        replacement
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Snapshot {
    allocations: u64,
    deallocations: u64,
    allocated_bytes: u64,
    deallocated_bytes: u64,
}

impl Snapshot {
    fn delta(self, earlier: Self) -> Self {
        Self {
            allocations: self.allocations - earlier.allocations,
            deallocations: self.deallocations - earlier.deallocations,
            allocated_bytes: self.allocated_bytes - earlier.allocated_bytes,
            deallocated_bytes: self.deallocated_bytes - earlier.deallocated_bytes,
        }
    }

    fn assert_balanced(self, label: &str) {
        assert_eq!(self.allocations, self.deallocations, "{label} owners");
        assert_eq!(
            self.allocated_bytes, self.deallocated_bytes,
            "{label} bytes"
        );
    }
}

fn snapshot() -> Snapshot {
    Snapshot {
        allocations: ALLOCATIONS.get(),
        deallocations: DEALLOCATIONS.get(),
        allocated_bytes: ALLOCATED_BYTES.get(),
        deallocated_bytes: DEALLOCATED_BYTES.get(),
    }
}

/// Initialize the process-lifetime statics that the JSON frontend's dependencies create lazily,
/// so the first observed window is not charged for them.
///
/// `json-syntax` 0.12.5 indexes every object through `hashbrown` 0.12's `DefaultHashBuilder`,
/// which is `ahash` 0.7's `RandomState`. Its first construction in a process boxes three
/// `once_cell::race::OnceBox` statics (`RAND_SOURCE`, its inner `Box<dyn RandomSource>`, and the
/// `SEEDS` array: 8 + 16 + 64 bytes) that live until process exit and belong to no capi owner.
/// ahash's `build.rs` forces `runtime-rng` on every hosted target, so no Cargo feature removes
/// them. They land on whichever thread parses the first object, which under the parallel
/// harness is a race between this file's tests; parsing a trivial object here makes every
/// window start after that initialization on this thread, independent of sibling scheduling.
fn warm_process_lifetime_statics() {
    let _ = session::parse_session_json(r#"{"warm":0}"#);
}

fn begin() {
    warm_process_lifetime_statics();
    // Initialize the thread-local keys before observation is armed.
    ACTIVE.set(false);
    ALLOCATIONS.set(0);
    DEALLOCATIONS.set(0);
    ALLOCATED_BYTES.set(0);
    DEALLOCATED_BYTES.set(0);
    ACTIVE.set(true);
}

fn finish() -> Snapshot {
    ACTIVE.set(false);
    snapshot()
}

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

fn limits() -> CompileLimits {
    CompileLimits {
        struct_size: COMPILE_LIMITS_SIZE,
        source_ring_frames: 1_024,
        maximum_automation_spans_per_block: 128,
        reserved0: 0,
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
        maximum_capi_retained_bytes: 10_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
        maximum_control_frame_bytes: 4_096,
        maximum_replay_bytes: 8_192,
        maximum_replay_entries: 16,
        maximum_submixes: 0,
        maximum_vcas: 0,
        reserved: [0; 2],
    }
}

/// `SESSION`'s source content identity, `blake3:` and `tag` repeated as hex: the same length as
/// the fixture's.
fn content(tag: u8) -> String {
    format!("blake3:{}", format!("{tag:02x}").repeat(32))
}

/// A structural transaction that renders identically (#1260 D3): it changes only `SESSION`'s
/// source content identity to `content(tag)`. A session ID edit is model-only and commits without
/// a rebuild, so it no longer triggers one. Successive rebuilds need distinct tags.
fn command(request_id: u64, revision: u64, tag: u8) -> Vec<u8> {
    let model = session::parse_session_json(SESSION).expect("fixture");
    let source = &model.sources[0];
    assert_ne!(source.content, content(tag), "the edit changes the content");
    let edit = SessionEdit::SetSourceContent {
        source_id: source.id.clone(),
        content: content(tag),
        channels: source.channels,
        bit_depth: source.bit_depth,
        frames: source.frames,
    };
    let mut bytes = vec![0_u8; 4_096];
    let len = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(request_id).expect("nonzero request"),
                expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                payload: CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
            },
            &mut bytes,
        )
        .expect("structural command");
    bytes.truncate(len);
    bytes
}

/// A structural transaction that renders identically and carries the source (#1273): `UpsertTrack`
/// of a muted copy of `document`'s first track as `track_id`, plus a copy of that track's route.
/// The new strip needs a new plan, and the source is unchanged, so the successor carries its ring.
fn muted_track_command(document: &str, request_id: u64, revision: u64, track_id: &str) -> Vec<u8> {
    let model = session::parse_session_json(document).expect("the session");
    let mut track = model.tracks[0].clone();
    let first = track.id.clone();
    track.id = StableId::parse(track_id).expect("track ID");
    track.fader.left_mute = true;
    track.fader.right_mute = true;
    let mut route = model
        .routes
        .iter()
        .find(|route| {
            matches!(&route.source, session::RouteSource::Track { track_id, .. } if *track_id == first)
        })
        .expect("the first track's route")
        .clone();
    route.id = StableId::parse(&format!("{track_id}-route")).expect("route ID");
    if let session::RouteSource::Track { track_id, .. } = &mut route.source {
        *track_id = track.id.clone();
    }
    let edits = [
        SessionEdit::UpsertTrack { track },
        SessionEdit::UpsertRoute { route },
    ];
    let mut bytes = vec![0_u8; 4_096];
    let len = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(request_id).expect("nonzero request"),
                expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                payload: CommandPayload::SessionTransactionApply(&edits),
            },
            &mut bytes,
        )
        .expect("structural command");
    bytes.truncate(len);
    bytes
}

fn capability_command() -> Vec<u8> {
    let mut bytes = vec![0_u8; 4_096];
    let len = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(1).expect("request"),
                expected_revision: ExpectedRevision::Any,
                payload: CommandPayload::CapabilitiesGet,
            },
            &mut bytes,
        )
        .expect("capability command");
    bytes.truncate(len);
    bytes
}

unsafe fn submit(session: *mut Session, request: &[u8], response: &mut [u8; 4_096]) -> u32 {
    let mut output = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: response.as_mut_ptr(),
        capacity_bytes: response.len() as u64,
        required_bytes: 0,
    };
    // SAFETY: All pointers identify live handles or complete caller-owned buffers for this call.
    unsafe {
        miso_engine_v1_submit_command(session, request.as_ptr(), request.len() as u64, &mut output)
    }
}

fn lifecycle(plan_first: bool) -> (Snapshot, Snapshot, Snapshot, Snapshot) {
    let capability = capability_command();
    let first = command(2, 42, 0x01);
    let second = command(3, 43, 0x02);
    let config = EngineConfig {
        struct_size: ENGINE_CONFIG_SIZE,
        abi_version: ABI_VERSION,
        reserved: [0; 4],
    };
    let mut diagnostics_storage = [0_u8; 4_096];
    let mut diagnostics = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: diagnostics_storage.as_mut_ptr(),
        capacity_bytes: diagnostics_storage.len() as u64,
        required_bytes: 0,
    };
    let mut response = [0_u8; 4_096];
    let mut pcm = [f32::NAN; 256];
    let output = PlanarOutput {
        struct_size: PLANAR_OUTPUT_SIZE,
        channels: 2,
        samples: pcm.as_mut_ptr(),
        sample_capacity: pcm.len() as u64,
        frames: 128,
        plane_stride_samples: 128,
        reserved: [0; 2],
    };
    let mut engine = ptr::null_mut();
    let mut session = ptr::null_mut();
    let mut plan = ptr::null_mut();

    begin();
    // SAFETY: All ABI arguments remain live and uniquely owned for the complete lifecycle.
    let codes = unsafe {
        let create = miso_engine_v1_engine_create(&config, &mut engine);
        let compile = miso_engine_v1_compile_session(
            engine,
            SESSION.as_ptr(),
            SESSION.len() as u64,
            &limits(),
            &mut diagnostics,
            &mut session,
            &mut plan,
        );
        let immediate = submit(session, &capability, &mut response);
        let before_cached = snapshot();
        let cached = submit(session, &capability, &mut response);
        let cached_delta = snapshot().delta(before_cached);
        let first_structural = submit(session, &first, &mut response);
        let before_full = snapshot();
        let full = submit(session, &second, &mut response);
        let full_delta = snapshot().delta(before_full);
        let before_render = snapshot();
        let render = miso_engine_v1_render_f32_planar(plan, 0, &output);
        let render_delta = snapshot().delta(before_render);
        let retry = submit(session, &second, &mut response);
        let second_render = miso_engine_v1_render_f32_planar(plan, 128, &output);
        if plan_first {
            miso_engine_v1_plan_destroy(plan);
            miso_engine_v1_session_destroy(session);
        } else {
            miso_engine_v1_session_destroy(session);
            miso_engine_v1_plan_destroy(plan);
        }
        miso_engine_v1_engine_destroy(engine);
        (
            [
                create,
                compile,
                immediate,
                cached,
                first_structural,
                full,
                render,
                retry,
                second_render,
            ],
            cached_delta,
            full_delta,
            render_delta,
        )
    };
    let total = finish();
    assert_eq!(
        codes.0,
        [
            RESULT_OK,
            RESULT_OK,
            RESULT_OK,
            RESULT_OK,
            RESULT_OK,
            RESULT_BACKPRESSURE,
            RESULT_OK,
            RESULT_OK,
            RESULT_OK,
        ]
    );
    (total, codes.1, codes.2, codes.3)
}

fn rejected_compile_lifecycle() -> Snapshot {
    let config = EngineConfig {
        struct_size: ENGINE_CONFIG_SIZE,
        abi_version: ABI_VERSION,
        reserved: [0; 4],
    };
    let mut constrained = limits();
    constrained.maximum_capi_retained_bytes = 1;
    let mut diagnostic_storage = [0_u8; 4_096];
    let mut diagnostics = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: diagnostic_storage.as_mut_ptr(),
        capacity_bytes: diagnostic_storage.len() as u64,
        required_bytes: 0,
    };
    let mut engine = ptr::null_mut();
    let mut session = ptr::dangling_mut();
    let mut plan = ptr::dangling_mut();

    begin();
    // SAFETY: All ABI arguments remain live for each call and no rejected child is published.
    let codes = unsafe {
        let create = miso_engine_v1_engine_create(&config, &mut engine);
        let compile = miso_engine_v1_compile_session(
            engine,
            SESSION.as_ptr(),
            SESSION.len() as u64,
            &constrained,
            &mut diagnostics,
            &mut session,
            &mut plan,
        );
        miso_engine_v1_engine_destroy(engine);
        (create, compile)
    };
    let total = finish();
    assert_eq!(codes, (RESULT_OK, RESULT_COMPILE_REJECTED));
    assert!(session.is_null());
    assert!(plan.is_null());
    assert!(diagnostics.required_bytes > 0);
    total
}

#[test]
fn exported_c_candidates_replay_render_and_both_destroy_orders_balance_exactly() {
    rejected_compile_lifecycle().assert_balanced("rejected compile provisional owners");

    let plan_first = lifecycle(true);
    plan_first
        .0
        .assert_balanced("plan-first complete lifecycle");
    plan_first.1.assert_balanced("cached replay");
    plan_first
        .2
        .assert_balanced("publication-full canceled candidate");
    assert_eq!(
        plan_first.3,
        Snapshot {
            allocations: 0,
            deallocations: 0,
            allocated_bytes: 0,
            deallocated_bytes: 0,
        }
    );

    let session_first = lifecycle(false);
    session_first
        .0
        .assert_balanced("session-first complete lifecycle");
    session_first.1.assert_balanced("cached replay");
    session_first
        .2
        .assert_balanced("publication-full canceled candidate");
    assert_eq!(session_first.3, plan_first.3);
    assert_eq!(session_first.0, plan_first.0, "destroy-order ownership");
}

fn scratch_session() -> String {
    let mut model = session::parse_session_json(SESSION).expect("oracle fixture");
    // The fixture's console slot becomes a soft-clip on every track (decision 12: the slot is
    // declared once, and each track's entry carries its knobs).
    let slot = &mut model.console.pre_insert[0];
    slot.slot = StableId::parse("soft-clip").expect("effect slot");
    slot.identity = session::EffectIdentity::Native {
        effect_id: StableId::parse("miso.soft-clip").expect("effect ID"),
    };
    for track in &mut model.tracks {
        let effect = &mut track.console[0];
        effect.slot = StableId::parse("soft-clip").expect("effect slot");
        effect.params = vec![
            session::EffectParam {
                parameter_id: 1,
                channel: session::ParameterChannel::Left,
                unit: session::ParameterUnit::Db,
                value: -6.0,
            },
            session::EffectParam {
                parameter_id: 1,
                channel: session::ParameterChannel::Right,
                unit: session::ParameterUnit::Db,
                value: -6.0,
            },
        ];
    }
    session::canonical_session_json(&model).expect("oracle canonical fixture")
}

// --- #1060: budgets plus one allocator-observed completeness oracle ----------------------------
//
// Owner decision 4 (`docs/rulings/engine-footprint-2026-09-28.md`): memory tests assert budgets
// (upper limits) plus one independent completeness check that every allocation is counted, instead
// of exact byte counts. The exact totals this file used to carry were a hand-maintained mirror of
// about 1,850 lines of retained layouts, re-pinned on every layout change; a ceiling cannot replace
// them alone, because a ceiling never sees a *decrease* -- an owner row dropped from the accounting
// under-reports, admission then accepts a session above the host's configured cap, and on a phone
// that is an OOM kill (#1060 amendment 3). So the completeness claim moved to the allocator: this
// file's counting allocator observes every byte a C ABI compile leaves live, owner by owner, and
// compares each owner's observed bytes with its charge. No charge is taken on trust from the
// accounting under test (#1060 amendment 2; attempt 1's oracle subtracted two charges it did not
// observe, and both hid under-counts that attempt 2 fixes).

/// The C limits as the host-core preparation caps capi derives from them (`runtime::prepare_caps`),
/// field for field. Only the ring length and the automation span bound change what preparation
/// allocates; the byte and count caps are upper bounds the reference sessions sit far below.
fn host_caps(limits: &CompileLimits) -> host_core::HostPrepareCaps {
    host_core::HostPrepareCaps {
        shape: host_core::HostShapePolicy::AnyLaunchRate,
        source_ring_frames: limits.source_ring_frames,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: limits.maximum_automation_spans_per_block,
        maximum_tracks: limits.maximum_tracks,
        maximum_submixes: if limits.maximum_submixes == 0 {
            limits.maximum_tracks
        } else {
            limits.maximum_submixes
        },
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

/// The host-core half of one `miso_engine_v1_compile_session`, replayed through the public API.
///
/// capi parses the document into a `SessionStore` and prepares the plan, its source producers,
/// its strip live-control producers and its parameter catalog through `host-core` before it
/// allocates anything of its own; it then moves all of them into its two handles without copying
/// them. What the compile leaves live beyond this half is therefore exactly what capi itself
/// allocated.
struct HostHalf {
    store: protocol::SessionStore,
    prepared: host_core::PreparedHost,
    /// Each strip's fader/mute and matrix/pan producers, kept as capi keeps them (#1256 D2).
    strips: Box<[host_core::TrackControlProducer]>,
    /// One producer per prepared effect instance, with each EQ's owner, kept as capi keeps them
    /// (#1263 D2).
    effects: Box<[host_core::EffectControlProducer]>,
}

/// capi's live-lane depth, `LIVE_QUEUE_DEPTH` (#1053 D4), which is crate-private.
const LIVE_QUEUE_DEPTH: core::num::NonZeroUsize = core::num::NonZeroUsize::new(16).unwrap();

fn host_half(document: &str, compile_limits: &CompileLimits) -> HostHalf {
    let caps = host_caps(compile_limits);
    let model = host_core::parse_host_session(document)
        .unwrap_or_else(|_| panic!("the reference session parses"));
    let compile_caps = caps
        .compile_caps(model.sources.len())
        .unwrap_or_else(|_| panic!("the reference session's compile caps"));
    let store = protocol::SessionStore::new(model, compile_caps)
        .unwrap_or_else(|_| panic!("the reference session compiles"));
    // The same request capi makes (#1256 D1, #1263 D1: fader/mute, matrix/pan and effect lanes,
    // no input or route lane), spelled here independently, and inside the caller's observed
    // window the same disposal (D2): the strip ID list and the vectors this selection leaves empty
    // drop here, so only the producers survive, as two boxed slices. Keeping the ID list would
    // count bytes capi does not keep.
    let (prepared, handles) = host_core::prepare_host_runtime_with_live_lanes(
        store.compiled(),
        &caps,
        &host_core::HostLiveControlRequest {
            control_queue_depth: Some(LIVE_QUEUE_DEPTH),
            ..host_core::HostLiveControlRequest::default()
        },
        host_core::HostLiveLanes {
            strip_input: false,
            effects: true,
            routes: false,
        },
    )
    .unwrap_or_else(|_| panic!("the reference session prepares"));
    let host_core::HostLiveControlHandles {
        strip_controls,
        effect_controls,
        ..
    } = handles;
    HostHalf {
        store,
        prepared,
        strips: strip_controls.into_boxed_slice(),
        effects: effect_controls.into_boxed_slice(),
    }
}

impl HostHalf {
    /// The compiled model's own estimate: what the graph cap charges for one live session model.
    fn model_estimate(&self) -> session::ResourceEstimate {
        self.store.compiled().resource_estimate()
    }

    /// The parameter catalog's charge, from the provider's own resource report. The retained
    /// telemetry and diagnostic capacities are the provider's fixed rows, not the catalog's.
    fn catalog_charge(&self) -> u64 {
        host_core::SessionControlProvider::resource_report(
            &self.prepared.control_catalog,
            protocol::ControllerRetainedCapacity {
                meter_handles: 0,
                counter_ids: 0,
            },
            0,
        )
        .unwrap_or_else(|_| panic!("the catalog's resource report"))
        .catalog_retained_bytes
    }
}

impl HostHalf {
    /// The two capi terms an epoch adds to a replacement's or a live edit's peak, each from its
    /// owning crate's report: `epoch_retained` and `prepared_protocol_retained`.
    ///
    /// The session's canonical JSON is charged once, with its model in the graph row. The epoch
    /// term is the source producers' control table and ID arena, the strip producers' table
    /// and track IDs (#1256), measured from the kept slice, and the effect producers' table and
    /// owned payload (#1263), from host-core's walk over them, and the plan state inventory the
    /// epoch keeps for its successor (#1273 D5). The prepared-protocol term is the
    /// response buffer, the affine token, the replay cache and the parameter catalog.
    fn capi_epoch_terms(&self, compile_limits: &CompileLimits) -> (u64, u64) {
        let replay =
            protocol::ReplayCache::resource_report_for_config(protocol::ReplayCacheConfig {
                entries: core::num::NonZeroUsize::new(
                    compile_limits.maximum_replay_entries as usize,
                )
                .expect("replay entries"),
                bytes: core::num::NonZeroUsize::new(compile_limits.maximum_replay_bytes as usize)
                    .expect("replay bytes"),
                max_response_bytes: compile_limits.maximum_control_frame_bytes as usize,
            })
            .unwrap_or_else(|_| panic!("replay resource report"));
        let strips = size_of_val(&*self.strips) as u64
            + self
                .strips
                .iter()
                .map(|producer| producer.track_id.len() as u64)
                .sum::<u64>();
        let effects = self
            .prepared
            .report
            .effect_control_resources
            .total_bytes()
            .expect("the effect producers' charge");
        let epoch = self.prepared.report.control_retained_bytes
            + strips
            + effects
            + self.prepared.report.inventory_retained_bytes;
        let prepared_protocol = compile_limits.maximum_control_frame_bytes
            + size_of::<protocol::PreparedStructuralCommand>() as u64
            + replay.retained_payload_bytes
            + self.catalog_charge();
        (epoch, prepared_protocol)
    }
}

fn live_bytes(window: Snapshot) -> u64 {
    window
        .allocated_bytes
        .checked_sub(window.deallocated_bytes)
        .expect("an observed window frees nothing it did not allocate")
}

/// The bytes dropping `value` frees, observed by this file's allocator.
fn freed_by_drop<T>(value: T) -> u64 {
    begin();
    drop(value);
    let window = finish();
    assert_eq!(window.allocated_bytes, 0, "dropping allocates nothing");
    window.deallocated_bytes
}

/// What each owner the host-core half hands capi retains, observed by dropping it.
///
/// The drop order is the attribution: the effect, strip and source producers go first, so a ring
/// the producers share with the plan is freed -- and counted -- with the plan that renders from
/// it, and the producers' own bytes are their tables, IDs and (an EQ's) owners alone.
struct HostOwners {
    effects: u64,
    strips: u64,
    sources: u64,
    catalog: u64,
    plan: u64,
    store: u64,
    /// The plan's state inventory (#1272 D1), which capi keeps with the plan's source producers
    /// to prepare its successor from (#1273 D1), and charges with them (D5).
    inventory: u64,
}

/// One C ABI compile and its replayed host-core half, each observed by this file's allocator.
struct CompileObservation {
    report: PlanResourceReport,
    host_report: host_core::HostPrepareReport,
    model: session::ResourceEstimate,
    /// Bytes the compile left live in its session and plan handles.
    compile_live: u64,
    /// Bytes the replayed host-core half left live, and the same bytes owner by owner.
    host_live: u64,
    owners: HostOwners,
    /// `strip_control_table_bytes` at the compiled session's strip count and strip ID bytes.
    strip_table_charge: u64,
}

fn observe_compile(document: &str, compile_limits: &CompileLimits) -> CompileObservation {
    begin();
    // SAFETY: The returned handles are uniquely owned until the matching destroy calls below.
    let (session, plan) = unsafe { compile_c(document, compile_limits) };
    let compile_live = live_bytes(finish());
    // SAFETY: `plan` is live; both handles are destroyed exactly once, after the report is read.
    let report = unsafe {
        let report = resources_c(plan);
        miso_engine_v1_session_destroy(session);
        miso_engine_v1_plan_destroy(plan);
        report
    };
    begin();
    let host = host_half(document, compile_limits);
    let host_live = live_bytes(finish());
    let model = host.model_estimate();
    let HostHalf {
        store,
        prepared,
        strips,
        effects,
    } = host;
    let (strip_count, strip_id_bytes) = store
        .compiled()
        .normalized_model()
        .strips()
        .fold((0, 0), |(count, bytes), strip| {
            (count + 1, bytes + strip.id.as_str().len())
        });
    let strip_table_charge = host_core::strip_control_table_bytes(strip_count, strip_id_bytes)
        .expect("the strip table's charge");
    let host_core::PreparedHost {
        plan,
        sources,
        report: host_report,
        inventory,
        control_catalog,
    } = prepared;
    let owners = HostOwners {
        effects: freed_by_drop(effects),
        strips: freed_by_drop(strips),
        sources: freed_by_drop(sources),
        catalog: freed_by_drop(control_catalog),
        plan: freed_by_drop(plan),
        store: freed_by_drop(store),
        inventory: freed_by_drop(inventory),
    };
    assert_eq!(
        owners.effects
            + owners.strips
            + owners.sources
            + owners.catalog
            + owners.plan
            + owners.store
            + owners.inventory,
        host_live,
        "the seven owners hold every byte the host-core half retains"
    );
    CompileObservation {
        report,
        host_report,
        model,
        compile_live,
        host_live,
        owners,
        strip_table_charge,
    }
}

impl CompileObservation {
    /// The completeness claim for capi's own row, to the byte, from observations only.
    ///
    /// `capi_retained_bytes` charges what capi allocates itself -- everything the compile left live
    /// beyond its host-core half -- plus the five host-allocated owners capi keeps: the effect and
    /// strip live-control producers (#1263, #1256), the source control table and ID arena, the
    /// parameter catalog, and the plan state inventory (#1273 D5). Each of the six terms on the
    /// left is observed. An owner row dropped from capi's accounting
    /// (the verifier's `checked_layout::<Plan>(1)`) lowers the right side alone; a new owner capi
    /// allocates but does not charge, or spare capacity in a charged one (the catalog's enum-choice
    /// vectors before #1060 attempt 2), raises the left side alone. Either is red. Nothing is a
    /// byte literal, so a charged layout change moves both sides together and needs no edit.
    ///
    /// The one difference is derived, not pinned: capi charges its decode-field scratch at the
    /// control frame's byte length but allocates whole `u16` fields, so an odd frame length is
    /// charged one byte it does not allocate. That is the safe direction.
    fn assert_capi_retained_bytes_are_complete(&self, label: &str, compile_limits: &CompileLimits) {
        let capi_allocated = self
            .compile_live
            .checked_sub(self.host_live)
            .unwrap_or_else(|| {
                panic!(
                    "{label}: the compile left {} bytes live, fewer than its host-core half's {}",
                    self.compile_live, self.host_live
                )
            });
        let decode_field_rounding =
            compile_limits.maximum_control_frame_bytes % size_of::<u16>() as u64;
        assert_eq!(
            capi_allocated
                + self.owners.effects
                + self.owners.strips
                + self.owners.sources
                + self.owners.catalog
                + self.owners.inventory
                + decode_field_rounding,
            self.report.capi_retained_bytes,
            "{label}: capi's own allocations + the effect producers + the strip producers + the \
             source producers + the parameter catalog + the plan state inventory + the \
             decode-field rounding, all observed (left), against \
             `capi_retained_bytes` (right)"
        );
    }

    /// The host-core owners against the rows that charge them.
    ///
    /// * The source producers are exact: `control_retained_bytes` is walked over the built set.
    /// * The session store is bounded by its compiled-model estimate, the graph cap's model row.
    ///   The estimate is exact for the model's vectors, its strings and its canonical JSON, and
    ///   charges 128 bytes per entity for indexes, of which the compiled session builds one, over
    ///   its sources: the slack is that allowance less the source index's nodes (1,026 bytes on
    ///   the EQ fixture, 2 on the browser identity fixture). The canonical JSON's spare capacity
    ///   (16,056 bytes on the EQ fixture before #1060 attempt 2 shrank it) was invisible to the
    ///   charge and is red here.
    /// * The prepared plan is bounded by its engine rows. Those rows are the graph compiler's
    ///   admission estimate, which charges the preparation, not only what the bound plan keeps:
    ///   the compile-time graph metadata, bank scratch per slot where a merged chain keeps one
    ///   slot's, and #511's and #936's reservations at their bound. The slack, eight lanes / four,
    ///   is 128,317 / 128,405 bytes on the EQ fixture and 12,233 / 8,109 on the browser identity
    ///   fixture, so an uncharged plan allocation smaller than that is not seen here: an uncharged
    ///   16-word-per-unit table in the graph runtime (1,808 bytes on the EQ fixture) stays green,
    ///   and the same table at 2,048 words per unit is red. Seeing the small one needs per-owner
    ///   attribution inside the prepared plan -- a retained-bytes walk over the bound runtime, as
    ///   `observation_retained_bytes` already is for its lanes -- which no public API exposes.
    ///   That is a successor to #1060 (root files it; the spec's attempt-2 evidence names it).
    ///   Builtins have their own allocator oracle (`builtins-compiler/tests/allocation_tracker.rs`).
    fn assert_host_owners_are_charged(&self, label: &str) {
        let owners = &self.owners;
        assert_eq!(
            owners.effects,
            self.host_report
                .effect_control_resources
                .total_bytes()
                .expect("the effect producers' charge"),
            "{label}: effect producer table, its IDs and the EQ owners"
        );
        assert_eq!(
            owners.strips, self.strip_table_charge,
            "{label}: strip producer table and its track IDs"
        );
        assert_eq!(
            owners.sources, self.host_report.control_retained_bytes,
            "{label}: source control table and ID arena"
        );
        assert_eq!(
            owners.inventory, self.host_report.inventory_retained_bytes,
            "{label}: the plan state inventory"
        );
        assert!(
            owners.store <= self.model.compiled_model_bytes,
            "{label}: the session store retains {} bytes, above its compiled-model estimate {}",
            owners.store,
            self.model.compiled_model_bytes
        );
        let report = &self.host_report;
        let engine_rows = report.graph_session_plus_plan_bytes
            + report.source_overhead_bytes
            + report.effect_scalar_state_bytes
            + report.effect_scalar_scratch_bytes
            + report.builtin_retained_payload_bytes;
        assert!(
            owners.plan <= engine_rows,
            "{label}: the prepared plan retains {} bytes, above its charged engine rows \
             {engine_rows}",
            owners.plan
        );
        println!(
            "{label}: capi {} observed; store {} of {}; plan {} of {} (slack {})",
            self.report.capi_retained_bytes,
            owners.store,
            self.model.compiled_model_bytes,
            owners.plan,
            engine_rows,
            engine_rows - owners.plan
        );
    }
}

/// The browser identity fixture: one track, no effect. It joins the oracle so the smallest session
/// the product boots is observed too.
const BROWSER_IDENTITY: &str =
    include_str!("../../../hosts/host-web/tests/browser-v1/session.json");

#[test]
fn capi_retained_bytes_charge_every_byte_the_compile_retains() {
    for (label, document) in [
        ("parametric-eq-nine-track", SESSION.to_owned()),
        ("soft-clip nine-track", scratch_session()),
        ("browser identity", BROWSER_IDENTITY.to_owned()),
        // #1256 MINOR-1, folded into #1258: submix strips and a route into a submix, so the strip
        // table's submix entries and the C ABI's lane selection (no route lanes) are observed.
        ("routed submix", two_track_routed_submix_session()),
    ] {
        let observed = observe_compile(&document, &limits());
        observed.assert_capi_retained_bytes_are_complete(label, &limits());
        observed.assert_host_owners_are_charged(label);
    }
}

/// The parameter catalog's charge against what it allocates, on its own.
///
/// Before #1060 attempt 2 the catalog collected each descriptor's enum choices through
/// `Result<Vec<_>, _>`, whose unknown lower size bound grew the vector to capacity 8 for the EQ's six
/// filter kinds while `resource_report` charged by length: 72 kind descriptors x 2 spare 32-byte
/// choices = 4,608 retained bytes `capi_retained_bytes` did not charge. The catalog now reserves
/// each vector exactly.
#[test]
fn prepared_parameter_catalog_charge_covers_its_allocations() {
    let host = host_half(SESSION, &limits());
    let charge = host.catalog_charge();
    let HostHalf {
        store, prepared, ..
    } = host;
    let host_core::PreparedHost {
        control_catalog, ..
    } = prepared;
    assert_eq!(
        freed_by_drop(control_catalog),
        charge,
        "the prepared parameter catalog's allocations against its charge"
    );
    drop(store);
}

/// One retained-memory budget: a report row and its ceiling at each native lane width.
///
/// The lane width is a compile-time function of the target (`lane::Backend::current`): eight lanes
/// on x86-64-v3, four on AArch64 NEON. The bank and graph rows are AoSoA payloads and per-bank
/// metadata, so each width carries its own ceiling; every other row is width-independent and its
/// two ceilings are equal.
struct Budget {
    row: &'static str,
    value: fn(&PlanResourceReport) -> u64,
    eight_lanes: u64,
    four_lanes: u64,
}

impl Budget {
    fn ceiling(&self) -> u64 {
        // By width, not by variant: the scalar variant exists only under `lane/test-support`
        // (#1059), and no native product build selects it.
        match Backend::current().width() {
            8 => self.eight_lanes,
            4 => self.four_lanes,
            width => panic!("no retained budget is declared for a {width}-lane native build"),
        }
    }
}

/// The retained-memory budgets of the nine-track parametric-EQ reference session (`SESSION`) at
/// `limits()` (#1060, owner decision 4).
///
/// **Headroom.** Each ceiling is the row's value on 2026-09-28 (`a509b681`) at that width, plus
/// 10 %, rounded up to a 64-byte multiple. The owner ruled budgets are ceilings rather than exact
/// counts, and 10 % is where the history puts the line: #1060's review sampled four past moves of
/// these retained rows, +1.2 %, +2.3 %, +11 % and +48 %. A 10 % budget lets the two routine moves
/// through unseen and stops the two structural ones, which then raise the budget with their reason
/// in the same commit. A zero ceiling is a claim, not a budget: this session
/// declares no track delay, no scalar-scratch effect and no meter, so a nonzero row there is a new
/// retained class, not growth. The measured baselines, x86-64 / AArch64 (AArch64 under qemu-user):
///
/// | row | eight lanes | four lanes |
/// |---|---|---|
/// | graph session+plan, graph incremental | 237,481 | 230,845 |
/// | graph metadata | 56,068 | 56,840 |
/// | effect bank scratch, runtime buffer | 16,384 | 12,288 |
/// | effect bank metadata | 805 | 921 |
/// | builtin bank | 14,233 | 19,113 |
/// | builtin bank scratch | 49,152 | 36,864 |
/// | source PCM payload | 8,192 | 8,192 |
/// | source overhead / total | 3,934 / 12,126 | the same |
/// | effect scalar state | 8,424 | the same |
/// | builtin processor payload, builtin retained payload | 17,451 | the same |
/// | capi retained | 256,740 | the same |
///
/// capi retained is the one row #1060 attempt 2 moved: 273,452 -> 256,740, -16,712, the EQ
/// session's canonical JSON, which capi's epoch row charged a second time beside the compiled
/// model's graph-cap charge.
///
/// #1098 raised the three effect-bank rows, a structural move: the session's EQ is its console
/// slot, and a console slot's remainder now binds as a padded bank instead of rendering per node.
/// The ninth track's EQ is one more bank at eight lanes (8,192 -> 16,384 scratch and runtime
/// buffer, 616 -> 805 metadata) and at four (8,192 -> 12,288, 736 -> 921). A padded bank charges
/// scratch for every lane and member metadata for its members only. The four-lane values are
/// derived, not measured: the old four-lane baseline plus one four-lane bank (4,096 bytes of scratch
/// and of runtime buffer; one bank record, four mask bytes and one member, 185 bytes, which is the
/// eight-lane move less four mask bytes). The graph rows moved by the same amounts and stay inside
/// their budgets (253,934 of 261,248 at eight lanes).
/// | largest named allocation | 90,720 | the same |
///
/// #1256 raised the two builtin payload rows, a structural move: every C ABI plan now carries each
/// strip's fader/mute and matrix/pan lanes, two 16-record rings per strip plus builtins' producer
/// vector and control seal, 17,451 -> 28,521 (+11,070, 1,230 per strip on nine strips). The new
/// ceiling is 28,521 plus 10 %, rounded up to 64, at both widths. capi retained moved 256,812 ->
/// 258,135 (+1,323) inside its budget: the strip table, nine 136-byte producers and their 27 ID
/// bytes (1,251), and 24 bytes in each of the three provider-epoch slots (72).
///
/// #1263 raised the three graph rows, a structural move: every C ABI plan now carries one live
/// lane per prepared effect instance, here the nine console-slot EQs. The graph estimate charges
/// each lane's ring and target staging and the banked live-control owner the lanes make bind build
/// (`effect_control_resource`): 2,104 bytes per member plus 55,024 per eight-lane bank, measured
/// on x86-64 by track count. graph session+plan and incremental move 253,934 -> 382,918 and graph
/// metadata 56,137 -> 185,121 (+128,984 each); the prepared plan's observed bytes move by exactly
/// as much, so the slack above is unchanged. The eight-lane ceilings are the new values plus 10 %,
/// rounded up to 64. The four-lane ceilings are derived, not measured: the four-lane baseline
/// (230,845 + #1098's 16,453 for session+plan; 56,840 + 69 for metadata) plus the eight-lane
/// move, plus 10 %. That move is an upper bound at four lanes, because three four-lane banks
/// hold fewer lanes (12) than two eight-lane banks (16) and a bank's per-lane terms (the packed
/// span window, the lane array, the shunt over the AoSoA block) outweigh its per-bank staging
/// window. capi retained moved 258,231 -> 273,640 (+15,409) inside its budget: the effect
/// producer table (nine 104-byte producers) and its owned payload, the EQ owners and the effect
/// and strip IDs (14,425), and 16 bytes in each of the three provider-epoch slots (48).
const REFERENCE_BUDGETS: [Budget; 19] = [
    Budget {
        row: "graph_session_plus_plan_bytes",
        value: |report| report.graph_session_plus_plan_bytes,
        eight_lanes: 421_248,
        four_lanes: 413_952,
    },
    Budget {
        row: "graph_incremental_plan_bytes",
        value: |report| report.graph_incremental_plan_bytes,
        eight_lanes: 421_248,
        four_lanes: 413_952,
    },
    Budget {
        row: "graph_metadata_bytes",
        value: |report| report.graph_metadata_bytes,
        eight_lanes: 203_648,
        four_lanes: 204_544,
    },
    Budget {
        row: "graph_delay_bytes",
        value: |report| report.graph_delay_bytes,
        eight_lanes: 0,
        four_lanes: 0,
    },
    Budget {
        row: "effect_bank_scratch_bytes",
        value: |report| report.effect_bank_scratch_bytes,
        eight_lanes: 18_048,
        four_lanes: 13_568,
    },
    Budget {
        row: "effect_bank_runtime_buffer_bytes",
        value: |report| report.effect_bank_runtime_buffer_bytes,
        eight_lanes: 18_048,
        four_lanes: 13_568,
    },
    Budget {
        row: "effect_bank_metadata_bytes",
        value: |report| report.effect_bank_metadata_bytes,
        eight_lanes: 896,
        four_lanes: 1_024,
    },
    Budget {
        row: "builtin_bank_bytes",
        value: |report| report.builtin_bank_bytes,
        eight_lanes: 15_680,
        four_lanes: 21_056,
    },
    Budget {
        row: "builtin_bank_scratch_bytes",
        value: |report| report.builtin_bank_scratch_bytes,
        eight_lanes: 54_080,
        four_lanes: 40_576,
    },
    Budget {
        row: "source_pcm_payload_bytes",
        value: |report| report.source_pcm_payload_bytes,
        eight_lanes: 9_024,
        four_lanes: 9_024,
    },
    Budget {
        row: "source_overhead_bytes",
        value: |report| report.source_overhead_bytes,
        eight_lanes: 4_352,
        four_lanes: 4_352,
    },
    Budget {
        row: "source_total_bytes",
        value: |report| report.source_total_bytes,
        eight_lanes: 13_376,
        four_lanes: 13_376,
    },
    Budget {
        row: "effect_scalar_state_bytes",
        value: |report| report.effect_scalar_state_bytes,
        eight_lanes: 9_280,
        four_lanes: 9_280,
    },
    Budget {
        row: "effect_scalar_scratch_bytes",
        value: |report| report.effect_scalar_scratch_bytes,
        eight_lanes: 0,
        four_lanes: 0,
    },
    Budget {
        row: "builtin_processor_payload_bytes",
        value: |report| report.builtin_processor_payload_bytes,
        eight_lanes: 31_424,
        four_lanes: 31_424,
    },
    Budget {
        row: "builtin_meter_payload_bytes",
        value: |report| report.builtin_meter_payload_bytes,
        eight_lanes: 0,
        four_lanes: 0,
    },
    Budget {
        row: "builtin_retained_payload_bytes",
        value: |report| report.builtin_retained_payload_bytes,
        eight_lanes: 31_424,
        four_lanes: 31_424,
    },
    Budget {
        row: "capi_retained_bytes",
        value: |report| report.capi_retained_bytes,
        eight_lanes: 282_432,
        four_lanes: 282_432,
    },
    Budget {
        row: "largest_named_allocation_bytes",
        value: |report| report.largest_named_allocation_bytes,
        eight_lanes: 99_840,
        four_lanes: 99_840,
    },
];

/// The compiled session model's budget, the one retained figure the graph cap charges that the C
/// report does not carry: its estimate is 23,039 bytes at both widths, plus 10 %, rounded to 64.
const REFERENCE_MODEL_BUDGET: u64 = 25_344;

/// Sets one C cap row.
type SetCap = fn(&mut CompileLimits, u64);

#[test]
fn reference_session_retained_rows_stay_within_their_budgets() {
    let report = {
        // SAFETY: The returned handles are uniquely owned until the matching destroy calls.
        unsafe {
            let (session, plan) = compile_c(SESSION, &limits());
            let report = resources_c(plan);
            miso_engine_v1_session_destroy(session);
            miso_engine_v1_plan_destroy(plan);
            report
        }
    };
    let model = host_half(SESSION, &limits()).model_estimate();
    let budget = |row: &str| {
        REFERENCE_BUDGETS
            .iter()
            .find(|budget| budget.row == row)
            .unwrap_or_else(|| panic!("no budget row {row}"))
            .ceiling()
    };

    // Every row against its ceiling, reported whole before any assertion so one red run shows
    // every row's headroom.
    let mut over = Vec::new();
    for entry in &REFERENCE_BUDGETS {
        let value = (entry.value)(&report);
        let ceiling = entry.ceiling();
        println!("{}: {value} of {ceiling}", entry.row);
        if value > ceiling {
            over.push(format!("{} {value} > {ceiling}", entry.row));
        }
    }
    println!(
        "compiled model estimate: {} of {REFERENCE_MODEL_BUDGET}",
        model.compiled_model_bytes
    );
    if model.compiled_model_bytes > REFERENCE_MODEL_BUDGET {
        over.push(format!(
            "compiled model estimate {} > {REFERENCE_MODEL_BUDGET}",
            model.compiled_model_bytes
        ));
    }
    assert!(
        over.is_empty(),
        "retained rows over their budget (raise the budget with its reason, or find the \
         regression): {over:?}"
    );

    // The budgets bind through the product's own admission. A host configured at each budget
    // admits the reference session; the same cap lowered to one byte below the session's
    // requirement refuses it, so the row the budget bounds is the row admission enforces.
    let caps: [(&str, SetCap, u64, u64); 7] = [
        (
            "graph",
            |caps, value| caps.maximum_graph_session_plus_plan_bytes = value,
            report.graph_session_plus_plan_bytes + model.compiled_model_bytes,
            budget("graph_session_plus_plan_bytes") + REFERENCE_MODEL_BUDGET,
        ),
        (
            "source-total",
            |caps, value| caps.maximum_source_total_bytes = value,
            report.source_total_bytes,
            budget("source_total_bytes"),
        ),
        (
            "source-overhead",
            |caps, value| caps.maximum_source_overhead_bytes = value,
            report.source_overhead_bytes,
            budget("source_overhead_bytes"),
        ),
        (
            "effect-state",
            |caps, value| caps.maximum_effect_state_bytes = value,
            report.effect_scalar_state_bytes,
            budget("effect_scalar_state_bytes"),
        ),
        (
            "builtin",
            |caps, value| caps.maximum_builtin_retained_bytes = value,
            report.builtin_retained_payload_bytes,
            budget("builtin_retained_payload_bytes"),
        ),
        (
            "capi",
            |caps, value| caps.maximum_capi_retained_bytes = value,
            report.capi_retained_bytes,
            budget("capi_retained_bytes"),
        ),
        (
            "largest",
            |caps, value| caps.maximum_named_allocation_bytes = value,
            report
                .largest_named_allocation_bytes
                .max(model.single_allocation_bytes),
            budget("largest_named_allocation_bytes"),
        ),
    ];
    for (row, set_cap, requirement, ceiling) in caps {
        let mut at_budget = limits();
        set_cap(&mut at_budget, ceiling);
        // SAFETY: The returned handles are uniquely owned until the matching destroy calls.
        unsafe {
            let (session, plan) = compile_c(SESSION, &at_budget);
            assert_eq!(resources_c(plan), report, "{row}: the cap moves no row");
            miso_engine_v1_session_destroy(session);
            miso_engine_v1_plan_destroy(plan);
        }
        let mut one_below = limits();
        set_cap(&mut one_below, requirement - 1);
        // SAFETY: The helper verifies atomic rejection without published children.
        unsafe { compile_rejected_c(SESSION, &one_below) };
    }
}

unsafe fn compile_c(
    session_document: &str,
    compile_limits: &CompileLimits,
) -> (*mut Session, *mut Plan) {
    let config = EngineConfig {
        struct_size: ENGINE_CONFIG_SIZE,
        abi_version: ABI_VERSION,
        reserved: [0; 4],
    };
    let mut engine = ptr::null_mut();
    let mut session = ptr::null_mut();
    let mut plan = ptr::null_mut();
    let mut diagnostic_storage = [0_u8; 4_096];
    let mut diagnostics = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: diagnostic_storage.as_mut_ptr(),
        capacity_bytes: diagnostic_storage.len() as u64,
        required_bytes: 0,
    };
    // SAFETY: Every descriptor and output location remains live for the complete call.
    unsafe {
        assert_eq!(
            miso_engine_v1_engine_create(&config, &mut engine),
            RESULT_OK
        );
        assert_eq!(
            miso_engine_v1_compile_session(
                engine,
                session_document.as_ptr(),
                session_document.len() as u64,
                compile_limits,
                &mut diagnostics,
                &mut session,
                &mut plan,
            ),
            RESULT_OK,
            "{}",
            String::from_utf8_lossy(&diagnostic_storage[..diagnostics.required_bytes as usize])
        );
        miso_engine_v1_engine_destroy(engine);
    }
    (session, plan)
}

unsafe fn resources_c(plan: *const Plan) -> PlanResourceReport {
    // SAFETY: The caller supplies a live plan and this is a complete writable report.
    unsafe {
        let mut report: PlanResourceReport = core::mem::zeroed();
        report.struct_size = PLAN_RESOURCE_REPORT_SIZE;
        assert_eq!(miso_engine_v1_plan_resources(plan, &mut report), RESULT_OK);
        report
    }
}

unsafe fn compile_rejected_c(session_document: &str, compile_limits: &CompileLimits) {
    let config = EngineConfig {
        struct_size: ENGINE_CONFIG_SIZE,
        abi_version: ABI_VERSION,
        reserved: [0; 4],
    };
    let mut engine = ptr::null_mut();
    let mut session = ptr::dangling_mut();
    let mut plan = ptr::dangling_mut();
    let mut diagnostic_storage = [0_u8; 4_096];
    let mut diagnostics = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: diagnostic_storage.as_mut_ptr(),
        capacity_bytes: diagnostic_storage.len() as u64,
        required_bytes: 0,
    };
    // SAFETY: Every descriptor and output location remains live for the complete call.
    unsafe {
        assert_eq!(
            miso_engine_v1_engine_create(&config, &mut engine),
            RESULT_OK
        );
        assert_eq!(
            miso_engine_v1_compile_session(
                engine,
                session_document.as_ptr(),
                session_document.len() as u64,
                compile_limits,
                &mut diagnostics,
                &mut session,
                &mut plan,
            ),
            RESULT_COMPILE_REJECTED
        );
        assert!(session.is_null());
        assert!(plan.is_null());
        assert!(diagnostics.required_bytes > 0);
        miso_engine_v1_engine_destroy(engine);
    }
}

#[test]
fn render_diagnostic_egress_reuses_eager_capi_storage_without_allocation() {
    // SAFETY: The returned handles are uniquely owned until the matching destroy calls below.
    let (session, plan) = unsafe { compile_c(SESSION, &limits()) };
    let mut pcm = [f32::NAN; 256];
    let output = PlanarOutput {
        struct_size: PLANAR_OUTPUT_SIZE,
        channels: 2,
        samples: pcm.as_mut_ptr(),
        sample_capacity: pcm.len() as u64,
        frames: 128,
        plane_stride_samples: 128,
        reserved: [0; 2],
    };
    let mut event_storage = [0xa5_u8; 4_096];
    let mut event = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: event_storage.as_mut_ptr(),
        capacity_bytes: event_storage.len() as u64,
        required_bytes: 0,
    };
    let configuration = protocol::TelemetryConfiguration {
        meter_handles: Vec::new(),
        meter_period_blocks: 0,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: true,
        minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
    };
    let mut configure = vec![0_u8; 4_096];
    let configure_len = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(1).expect("request ID"),
                expected_revision: ExpectedRevision::Exact(SessionRevision(42)),
                payload: CommandPayload::TelemetryConfigure(&configuration),
            },
            &mut configure,
        )
        .expect("telemetry configuration");
    configure.truncate(configure_len);
    let mut response = [0_u8; 4_096];
    // SAFETY: The session and all caller-owned command buffers remain live for the call.
    let configure_result = unsafe { submit(session, &configure, &mut response) };
    assert_eq!(configure_result, RESULT_OK);

    begin();
    // SAFETY: Both live handles and caller-owned buffers remain valid for each complete call.
    let (render_result, event_result) = unsafe {
        (
            miso_engine_v1_render_f32_planar(plan, 0, &output),
            miso_engine_v1_dequeue_event(session, EVENT_LANE_RELIABLE, &mut event),
        )
    };
    let observed = finish();
    assert_eq!(render_result, RESULT_OK);
    assert_eq!(event_result, RESULT_OK);
    assert!(
        event.required_bytes > 0,
        "render diagnostic crossed C egress"
    );
    assert_eq!(
        observed,
        Snapshot {
            allocations: 0,
            deallocations: 0,
            allocated_bytes: 0,
            deallocated_bytes: 0,
        },
        "render observation and diagnostic egress use only eager retained storage"
    );

    // SAFETY: These are the exact live handles returned by `compile_c` and are destroyed once.
    unsafe {
        miso_engine_v1_session_destroy(session);
        miso_engine_v1_plan_destroy(plan);
    }
}

/// A structural replacement holds two plans live at once, so its admission charges both: every cap
/// row admits at exactly the double-live requirement and refuses one byte below it, atomically.
///
/// The requirement is taken from live values, never literals, so it holds at every lane width
/// (#1017 ran this eight-lane-only until #1060):
///
/// * the five payload rows are the sum of the two live reports, the current plan's before the
///   replacement and the prospective plan's after the render that swaps it in;
/// * the graph row adds both compiled models, each from its own `CompiledSession` estimate;
/// * the capi row adds the prospective epoch (its source control table and ID arena) and the
///   prepared protocol owner (its response buffer, affine token, replay cache and parameter
///   catalog), each from its owning crate's resource report;
/// * the largest row is the largest single allocation either plan or either model makes.
///
/// That the reports themselves charge every allocation is the allocator oracle's claim,
/// `capi_retained_bytes_charge_every_byte_the_compile_retains`; this test's claim is the admission
/// arithmetic: both plans are charged, on every row, to the byte.
#[test]
fn double_live_oracle_drives_exact_and_one_below_c_caps() {
    let session_document = scratch_session();
    // The trigger adds a muted strip and its route and leaves the source unchanged, so the
    // replacement carries the source's ring (#1273). A source-content edit would restart the
    // source in a new ring, and a session-ID edit commits with no replacement (#1260).
    let command = |request_id, revision, track_id| {
        muted_track_command(&session_document, request_id, revision, track_id)
    };

    // The two live reports, read through the C ABI at roomy caps, and the committed snapshot the
    // replacement prepared from.
    let mut snapshot_request = 100;
    // SAFETY: These handles are uniquely owned until their matching destroy calls.
    let (current, prospective, prospective_document) = unsafe {
        let (session, plan) = compile_c(&session_document, &limits());
        let current = resources_c(plan);
        let request = command(1, 42, "a-muted");
        let mut response = [0xa5_u8; 4_096];
        assert_eq!(submit(session, &request, &mut response), RESULT_OK);
        drain_events_c(session).unwrap_or_else(|failure| panic!("{failure}"));
        let (revision, prospective_document) = snapshot_c(session, &mut snapshot_request);
        assert_eq!(revision, 43, "the trigger committed");
        let mut pcm = [f32::NAN; 256];
        let output = PlanarOutput {
            struct_size: PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames: 128,
            plane_stride_samples: 128,
            reserved: [0; 2],
        };
        assert_eq!(
            miso_engine_v1_render_f32_planar(plan, 0, &output),
            RESULT_OK
        );
        let prospective = resources_c(plan);
        assert_eq!(
            plan_carry_counts(plan),
            (1, 0),
            "vacuous: the swap carried no ring"
        );
        miso_engine_v1_session_destroy(session);
        miso_engine_v1_plan_destroy(plan);
        (current, prospective, prospective_document)
    };
    assert_ne!(prospective_document, session_document);
    // The swap is also proved by admission: with one publication slot, a second replacement is
    // admitted only once the render has swapped the first one in.
    // SAFETY: These handles are uniquely owned until their matching destroy calls.
    unsafe {
        let (session, plan) = compile_c(&session_document, &limits());
        let request = command(1, 42, "a-muted");
        let mut response = [0xa5_u8; 4_096];
        assert_eq!(submit(session, &request, &mut response), RESULT_OK);
        let again = muted_track_command(&prospective_document, 2, 43, "a-muted-2");
        assert_eq!(
            submit(session, &again, &mut response),
            RESULT_BACKPRESSURE,
            "a second replacement waits for the render"
        );
        let mut pcm = [f32::NAN; 256];
        let output = PlanarOutput {
            struct_size: PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames: 128,
            plane_stride_samples: 128,
            reserved: [0; 2],
        };
        assert_eq!(
            miso_engine_v1_render_f32_planar(plan, 0, &output),
            RESULT_OK
        );
        assert_eq!(
            submit(session, &again, &mut response),
            RESULT_OK,
            "the render swapped the prospective plan in"
        );
        miso_engine_v1_session_destroy(session);
        miso_engine_v1_plan_destroy(plan);
    }

    let current_host = host_half(&session_document, &limits());
    let prospective_host = host_half(&prospective_document, &limits());
    let current_model = current_host.model_estimate();
    let prospective_model = prospective_host.model_estimate();
    // The replacement leaves the source unchanged, so the prospective plan is prepared to carry
    // the current plan's ring (#1273): its report counts that ring, which the current plan owns
    // until the swap, and the double-live peak counts once. It is prepared as capi prepares a
    // successor, with the same live lanes (`host_half`'s request).
    let successor = host_core::prepare_host_runtime_with_live_lanes_successor(
        prospective_host.store.compiled(),
        &host_caps(&limits()),
        &host_core::HostLiveControlRequest {
            control_queue_depth: Some(LIVE_QUEUE_DEPTH),
            ..host_core::HostLiveControlRequest::default()
        },
        host_core::HostLiveLanes {
            strip_input: false,
            effects: true,
            routes: false,
        },
        host_core::SuccessorBase {
            inventory: &current_host.prepared.inventory,
            committed: current_host.store.compiled().normalized_model(),
        },
    )
    .unwrap_or_else(|_| panic!("the prospective session prepares as a successor"))
    .0
    .report;
    assert!(
        successor.carried_source_total_bytes > 0,
        "vacuous: the replacement carries no ring"
    );
    assert_eq!(
        prospective.source_total_bytes, prospective_host.prepared.report.source_total_bytes,
        "the swapped-in plan's source rows count the ring it carries"
    );
    assert_eq!(
        prospective.graph_session_plus_plan_bytes,
        prospective_host
            .prepared
            .report
            .graph_session_plus_plan_bytes
            + successor.carry_program_retained_bytes,
        "the swapped-in plan's graph row counts its carry program"
    );
    println!(
        "carried ring {} bytes ({} overhead); carry program {} bytes; inventory {} bytes",
        successor.carried_source_total_bytes,
        successor.carried_source_overhead_bytes,
        successor.carry_program_retained_bytes,
        successor.inventory_retained_bytes
    );
    // The prospective session's canonical JSON is charged once, with its model in the graph row;
    // capi's epoch row is the prospective epoch's producers and the plan state inventory it keeps
    // (#1273 D5), and the prepared-protocol term is unchanged (`capi_epoch_terms`).
    let (prospective_epoch, prepared_protocol) = prospective_host.capi_epoch_terms(&limits());

    let rows: [(&str, u64); 8] = [
        (
            "graph",
            current.graph_session_plus_plan_bytes
                + prospective.graph_session_plus_plan_bytes
                + current_model.compiled_model_bytes
                + prospective_model.compiled_model_bytes,
        ),
        (
            "source-total",
            current.source_total_bytes + prospective.source_total_bytes
                - successor.carried_source_total_bytes,
        ),
        (
            "source-overhead",
            current.source_overhead_bytes + prospective.source_overhead_bytes
                - successor.carried_source_overhead_bytes,
        ),
        (
            "effect-state",
            current.effect_scalar_state_bytes + prospective.effect_scalar_state_bytes,
        ),
        (
            "effect-scratch",
            current.effect_scalar_scratch_bytes + prospective.effect_scalar_scratch_bytes,
        ),
        (
            "builtin",
            current.builtin_retained_payload_bytes + prospective.builtin_retained_payload_bytes,
        ),
        (
            "capi",
            current.capi_retained_bytes + prospective_epoch + prepared_protocol,
        ),
        (
            "largest",
            current
                .largest_named_allocation_bytes
                .max(prospective.largest_named_allocation_bytes)
                .max(current_model.single_allocation_bytes)
                .max(prospective_model.single_allocation_bytes),
        ),
    ];
    // The current plan's own admission. A double-live requirement above it leaves the initial
    // compile admitted one byte below; one that the current plan alone reaches refuses it there.
    let current_largest = current
        .largest_named_allocation_bytes
        .max(current_model.single_allocation_bytes);
    for (row, required) in rows {
        println!("{row}: double-live requirement {required}");
        let set_cap = |compile_limits: &mut CompileLimits, value: u64| match row {
            "graph" => compile_limits.maximum_graph_session_plus_plan_bytes = value,
            "source-total" => compile_limits.maximum_source_total_bytes = value,
            "source-overhead" => compile_limits.maximum_source_overhead_bytes = value,
            "effect-state" => compile_limits.maximum_effect_state_bytes = value,
            "effect-scratch" => compile_limits.maximum_effect_scratch_bytes = value,
            "builtin" => compile_limits.maximum_builtin_retained_bytes = value,
            "capi" => compile_limits.maximum_capi_retained_bytes = value,
            "largest" => compile_limits.maximum_named_allocation_bytes = value,
            _ => unreachable!(),
        };

        let mut exact_limits = limits();
        set_cap(&mut exact_limits, required);
        // SAFETY: These handles are uniquely owned until their matching destroy calls.
        unsafe {
            let (session, plan) = compile_c(&session_document, &exact_limits);
            assert_eq!(resources_c(plan), current, "{row}: the cap moves no row");
            let request = command(1, 42, "a-muted");
            let mut response = [0xa5_u8; 4_096];
            assert_eq!(submit(session, &request, &mut response), RESULT_OK, "{row}");
            let mut pcm = [f32::NAN; 256];
            let output = PlanarOutput {
                struct_size: PLANAR_OUTPUT_SIZE,
                channels: 2,
                samples: pcm.as_mut_ptr(),
                sample_capacity: pcm.len() as u64,
                frames: 128,
                plane_stride_samples: 128,
                reserved: [0; 2],
            };
            assert_eq!(
                miso_engine_v1_render_f32_planar(plan, 0, &output),
                RESULT_OK
            );
            assert_eq!(resources_c(plan), prospective, "{row}: the swapped-in plan");
            miso_engine_v1_session_destroy(session);
            miso_engine_v1_plan_destroy(plan);
        }

        let mut below_limits = limits();
        set_cap(&mut below_limits, required - 1);
        if row == "largest" && required == current_largest {
            // The same named owner is already live during initial construction, so one-below is
            // atomically rejected before either child handle can be published.
            // SAFETY: The helper owns every handle through rejection and destroys the engine.
            unsafe { compile_rejected_c(&session_document, &below_limits) };
            continue;
        }
        if row == "largest" && required == prospective_model.single_allocation_bytes {
            // The prospective model's own largest allocation is the requirement (the added strip
            // grows its canonical bound), so the protocol's compile caps refuse the transaction
            // before capi prepares a plan: a typed `ValidationFailed` response, with the revision
            // and the report unchanged.
            // SAFETY: These handles are uniquely owned until their matching destroy calls.
            unsafe {
                let (session, plan) = compile_c(&session_document, &below_limits);
                let before = resources_c(plan);
                let (code, header) = submit_header(session, &command(1, 42, "a-muted"));
                assert_eq!(code, RESULT_OK, "{row} one-below");
                let header = header.expect("a response header");
                assert_eq!(
                    (header.status, header.revision),
                    (StatusCode::ValidationFailed, SessionRevision(42)),
                    "{row} one-below"
                );
                assert_eq!(resources_c(plan), before, "{row} atomic report");
                miso_engine_v1_session_destroy(session);
                miso_engine_v1_plan_destroy(plan);
            }
            continue;
        }
        // SAFETY: These handles are uniquely owned until their matching destroy calls.
        unsafe {
            let (session, plan) = compile_c(&session_document, &below_limits);
            let before = resources_c(plan);
            let request = command(1, 42, "a-muted");
            let mut response = [0xa5_u8; 4_096];
            assert_eq!(
                submit(session, &request, &mut response),
                RESULT_COMPILE_REJECTED,
                "{row} one-below"
            );
            assert!(response.iter().all(|byte| *byte == 0xa5), "{row} canary");
            assert_eq!(resources_c(plan), before, "{row} atomic report");
            miso_engine_v1_session_destroy(session);
            miso_engine_v1_plan_destroy(plan);
        }
    }
}

/// `SESSION` with the first track's left fader at `left_db` and, when given, the source's content
/// string replaced (#1258 D2: a structural edit that renders identically).
fn fader_variant(left_db: f32, content: Option<&str>) -> String {
    let mut model = session::parse_session_json(SESSION).expect("fixture");
    model.tracks[0].fader.left_db = left_db;
    if let Some(content) = content {
        model.sources[0].content = content.to_owned();
    }
    session::canonical_session_json(&model).expect("canonical variant")
}

/// The first track's fader with `left_db` on the left lane, as the live cap edits set it.
fn eq0_fader_edit(left_db: f32) -> SessionEdit {
    SessionEdit::SetTrackFader {
        track_id: StableId::parse("eq0").expect("eq0"),
        fader: session::DualMonoFader {
            left_db,
            right_db: 0.0,
            left_mute: false,
            right_mute: false,
        },
    }
}

/// The structural edit of the pending-candidate case: a new content string on the source.
const LIVE_CAP_CONTENT: &str =
    "blake3:abababababababababababababababababababababababababababababababab";

fn live_cap_structural_edit() -> SessionEdit {
    let model = session::parse_session_json(SESSION).expect("fixture");
    let source = &model.sources[0];
    SessionEdit::SetSourceContent {
        source_id: source.id.clone(),
        content: LIVE_CAP_CONTENT.to_owned(),
        channels: source.channels,
        bit_depth: source.bit_depth,
        frames: source.frames,
    }
}

/// Compiles the `-6.0` base at `compile_limits`, commits the structural edit when `pending`, and
/// returns the handles and the committed revision.
///
/// # Safety
///
/// The returned handles are uniquely owned by the caller, who destroys them.
unsafe fn live_cap_session(
    compile_limits: &CompileLimits,
    pending: bool,
) -> (*mut Session, *mut Plan, u64) {
    // SAFETY: The handles are returned to the caller, who owns them.
    unsafe {
        let (session, plan) = compile_c(&fader_variant(-6.0, None), compile_limits);
        let mut revision = 42;
        if pending {
            let request = transaction(900, revision, &[live_cap_structural_edit()]);
            let (code, header) = submit_header(session, &request);
            assert_eq!(code, RESULT_OK, "the structural edit");
            assert_eq!(header.expect("a header").status, StatusCode::Ok);
            revision += 1;
            drain_events_c(session).unwrap_or_else(|failure| panic!("{failure}"));
        }
        (session, plan, revision)
    }
}

/// Renders four fed blocks and destroys both handles.
///
/// # Safety
///
/// Both handles are uniquely owned by the caller and destroyed here.
unsafe fn render_and_destroy(session: *mut Session, plan: *mut Plan) -> Vec<Vec<f32>> {
    let mut feed = ConstantFeed::new();
    // SAFETY: As the caller guarantees.
    unsafe {
        let blocks = (0..4)
            .map(|block| fed_render_c(session, plan, &mut feed, block))
            .collect();
        miso_engine_v1_session_destroy(session);
        miso_engine_v1_plan_destroy(plan);
        blocks
    }
}

/// #1258 gate 3: the live arm holds the caller's graph and capi caps to the byte (#1053 D8).
///
/// The edit moves the first track's left fader from `-6.0` to `-6.0123`, which grows the canonical
/// JSON, so the prospective compiled model is larger than the current one. Each peak is derived
/// from observations, never from the arm under test: the C report of the current plan (and, with a
/// candidate pending, of the candidate, read after the render that swaps it in), each compiled
/// model's own estimate, and the capi epoch terms from their owning crates' reports.
///
/// At each peak the edit commits. One byte below it is `RESULT_COMPILE_REJECTED` with the row's
/// diagnostic, the committed snapshot and its revision are unchanged, and the plan renders exactly
/// what a plan that never saw the edit renders, so no record reached a lane.
///
/// Test value: red if the live arm admits an edit whose peak is over the caller's cap, or refuses
/// one within it, on either row, or if a cap refusal leaves a record or a revision behind.
#[test]
fn live_edits_are_admitted_at_their_exact_graph_and_capi_peaks_and_refused_one_byte_below() {
    let base = fader_variant(-6.0, None);
    let grown = fader_variant(-6.0123, None);
    let structural = fader_variant(-6.0, Some(LIVE_CAP_CONTENT));
    let structural_grown = fader_variant(-6.0123, Some(LIVE_CAP_CONTENT));
    let model = |document: &str| {
        host_half(document, &limits())
            .model_estimate()
            .compiled_model_bytes
    };
    assert!(
        model(&grown) > model(&base),
        "the edit grows the compiled model"
    );

    // The current and the candidate plan's reports, read at roomy caps.
    // SAFETY: These handles are uniquely owned until their destroy calls.
    let (current, candidate) = unsafe {
        let (session, plan, _) = live_cap_session(&limits(), true);
        let current = resources_c(plan);
        let mut feed = ConstantFeed::new();
        fed_render_c(session, plan, &mut feed, 0);
        drain_events_c(session).unwrap_or_else(|failure| panic!("{failure}"));
        let candidate = resources_c(plan);
        miso_engine_v1_session_destroy(session);
        miso_engine_v1_plan_destroy(plan);
        (current, candidate)
    };
    let (_, base_protocol) = host_half(&base, &limits()).capi_epoch_terms(&limits());
    let rebuild_graph_peak = current.graph_session_plus_plan_bytes
        + candidate.graph_session_plus_plan_bytes
        + model(&base)
        + model(&structural);

    let cases: [(&str, bool, SetCap, u64, &str); 3] = [
        (
            "graph",
            false,
            |caps, value| caps.maximum_graph_session_plus_plan_bytes = value,
            current.graph_session_plus_plan_bytes + model(&base) + model(&grown),
            "graph.resource.limit",
        ),
        (
            "capi",
            false,
            |caps, value| caps.maximum_capi_retained_bytes = value,
            current.capi_retained_bytes + base_protocol,
            "capi.resource.limit",
        ),
        (
            "graph with a candidate pending",
            true,
            |caps, value| caps.maximum_graph_session_plus_plan_bytes = value,
            current.graph_session_plus_plan_bytes
                + candidate.graph_session_plus_plan_bytes
                + model(&structural)
                + model(&structural_grown),
            "graph.resource.limit",
        ),
    ];
    for (row, pending, set_cap, peak, code) in cases {
        println!("{row}: live peak {peak}");
        if pending {
            assert!(
                rebuild_graph_peak < peak,
                "{row}: the structural edit itself is admitted one byte below the live peak"
            );
        }
        // An unedited plan's PCM, the reference both runs below are held to.
        // SAFETY: These handles are uniquely owned and destroyed by the helper.
        let unedited = unsafe {
            let (session, plan, _) = live_cap_session(&limits(), pending);
            render_and_destroy(session, plan)
        };

        let mut at_peak = limits();
        set_cap(&mut at_peak, peak);
        // SAFETY: These handles are uniquely owned and destroyed by the helper.
        let edited = unsafe {
            let (session, plan, revision) = live_cap_session(&at_peak, pending);
            let (code, header) = submit_header(
                session,
                &transaction(901, revision, &[eq0_fader_edit(-6.0123)]),
            );
            assert_eq!(code, RESULT_OK, "{row}: admitted at the peak");
            let header = header.expect("a header");
            assert_eq!(header.status, StatusCode::Ok, "{row}");
            assert_eq!(header.revision, SessionRevision(revision + 1), "{row}");
            drain_events_c(session).unwrap_or_else(|failure| panic!("{failure}"));
            render_and_destroy(session, plan)
        };
        assert_ne!(
            edited.last().map(|pcm| bits(pcm)),
            unedited.last().map(|pcm| bits(pcm)),
            "{row}: the admitted edit is audible, so the comparison below would see a record"
        );

        let mut below = limits();
        set_cap(&mut below, peak - 1);
        // SAFETY: These handles are uniquely owned and destroyed by the helper.
        let refused = unsafe {
            let (session, plan, revision) = live_cap_session(&below, pending);
            let mut request_id = 1_000;
            let before = snapshot_c(session, &mut request_id);
            assert_eq!(before.0, revision, "{row}");
            // Request IDs increase across the session's commands.
            request_id += 1;
            let (result, _) = submit_header(
                session,
                &transaction(request_id, revision, &[eq0_fader_edit(-6.0123)]),
            );
            assert_eq!(result, RESULT_COMPILE_REJECTED, "{row}: one byte below");
            assert_eq!(
                last_error_c(session),
                format!("{code}\t$\n").into_bytes(),
                "{row}: the row's diagnostic"
            );
            assert_eq!(
                snapshot_c(session, &mut request_id),
                before,
                "{row}: the committed snapshot and its revision"
            );
            render_and_destroy(session, plan)
        };
        assert!(
            refused
                .iter()
                .zip(&unedited)
                .all(|(refused, unedited)| bits(refused) == bits(unedited)),
            "{row}: the refused edit reached a lane"
        );
    }
}

#[test]
fn tiny_control_frame_still_accounts_three_provider_counters_exactly() {
    let mut roomy = limits();
    roomy.maximum_control_frame_bytes = 1;
    // A one-byte frame leaves the controller no telemetry configuration capacity, and the provider
    // still retains its three-slot counter minimum. The allocator oracle checks the charge against
    // what the compile actually allocates (#1060 amendment 2), never against the report it tests.
    let observed = observe_compile(SESSION, &roomy);
    observed.assert_capi_retained_bytes_are_complete("tiny-frame retained authority", &roomy);
    let required = observed.report.capi_retained_bytes;
    let mut exact = roomy;
    exact.maximum_capi_retained_bytes = required;
    // SAFETY: Exact admission returns two uniquely owned children.
    unsafe {
        let (session, plan) = compile_c(SESSION, &exact);
        assert_eq!(resources_c(plan).capi_retained_bytes, required);
        miso_engine_v1_session_destroy(session);
        miso_engine_v1_plan_destroy(plan);
    }
    let mut below = roomy;
    below.maximum_capi_retained_bytes = required - 1;
    // SAFETY: The helper verifies atomic rejection without published children.
    unsafe { compile_rejected_c(SESSION, &below) };
}

/// Stops the render thread of the #1258 live-edit race however the control thread leaves the
/// scope, so a failed assertion cannot hang the join. A copy of
/// `bench_support::producer::StopOnDrop`, kept because naming anything from `bench_support` links
/// its `#[global_allocator]`, which conflicts with this file's counting allocator at compile time
/// (#1251 D4). The plan-swap race keeps its own copy in `plan_swap_race.rs` (#1273).
struct StopOnDrop<'a>(&'a AtomicBool);

impl Drop for StopOnDrop<'_> {
    fn drop(&mut self) {
        self.0.store(true, core::sync::atomic::Ordering::Release);
    }
}

// --- #1258: live C ABI edits against a concurrently rendering plan -------------------------------
//
// The helpers below drive one session the way a phone app does: a control thread feeds the source,
// commits edits and drains events while another thread renders. Every wait has a deadline.

/// The constant source the race and the live-cap tests feed: never zero, and different on the two
/// lanes (#1258 D1).
const CONSTANT_PCM: [f32; 2] = [0.375, -0.218_75];

/// The fixtures' render quantum.
const QUANTUM: usize = 128;

/// How long a control-thread wait for render progress may take before it fails.
const PROGRESS_DEADLINE: std::time::Duration = std::time::Duration::from_secs(10);

/// One submitted command: the C result and, on `RESULT_OK`, the decoded response header.
///
/// # Safety
///
/// `session` must be live and used by this thread alone for the call.
unsafe fn submit_header(
    session: *mut Session,
    request: &[u8],
) -> (u32, Option<protocol::ResponseHeader>) {
    let mut response = [0_u8; 4_096];
    let mut output = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: response.as_mut_ptr(),
        capacity_bytes: response.len() as u64,
        required_bytes: 0,
    };
    // SAFETY: The caller guarantees a live session; both buffers are owned and complete.
    let code = unsafe {
        miso_engine_v1_submit_command(session, request.as_ptr(), request.len() as u64, &mut output)
    };
    if code != RESULT_OK {
        return (code, None);
    }
    let mut fields = [0_u16; 64];
    let header = match ProtocolCodec::default()
        .decode_typed_response(
            &response[..output.required_bytes as usize],
            &mut protocol::DecodeScratch::new(&mut fields),
        )
        .expect("a decodable response")
    {
        protocol::DecodedTypedResponseFrame::Success { header, .. }
        | protocol::DecodedTypedResponseFrame::NonOk { header, .. } => header,
    };
    (code, Some(header))
}

/// The session's last error bytes.
///
/// # Safety
///
/// `session` must be live and used by this thread alone for the call.
unsafe fn last_error_c(session: *mut Session) -> Vec<u8> {
    let mut storage = [0_u8; 4_096];
    let mut output = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: storage.as_mut_ptr(),
        capacity_bytes: storage.len() as u64,
        required_bytes: 0,
    };
    // SAFETY: The caller guarantees a live session; the descriptor names complete owned storage.
    let code = unsafe { miso_engine_v1_last_error(session.cast_const().cast(), &mut output) };
    assert_eq!(code, RESULT_OK, "last error");
    storage[..output.required_bytes as usize].to_vec()
}

/// Dequeues every frame waiting on both event lanes.
///
/// # Safety
///
/// `session` must be live and used by this thread alone for the call.
unsafe fn drain_events_c(session: *mut Session) -> Result<(), String> {
    let mut storage = [0_u8; 4_096];
    for lane in [EVENT_LANE_RELIABLE, EVENT_LANE_LOSSY] {
        // Bounded: each lane holds a handful of frames.
        for _ in 0..1_024 {
            let mut event = BytesOut {
                struct_size: BYTES_OUT_SIZE,
                reserved0: 0,
                data: storage.as_mut_ptr(),
                capacity_bytes: storage.len() as u64,
                required_bytes: 0,
            };
            // SAFETY: As above.
            let code = unsafe { miso_engine_v1_dequeue_event(session, lane, &mut event) };
            if code != RESULT_OK {
                return Err(format!("dequeue lane {lane}: {code}"));
            }
            if event.required_bytes == 0 {
                break;
            }
        }
    }
    Ok(())
}

/// The committed session, paged through `SessionSnapshotGet`, and the revision its header reports.
///
/// # Safety
///
/// `session` must be live and used by this thread alone for the call.
unsafe fn snapshot_c(session: *mut Session, request_id: &mut u64) -> (u64, String) {
    let mut json = Vec::new();
    let mut fields = [0_u16; 64];
    let mut response = [0_u8; 4_096];
    loop {
        *request_id += 1;
        let mut bytes = vec![0_u8; 4_096];
        let len = ProtocolCodec::default()
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(*request_id).expect("nonzero request"),
                    expected_revision: ExpectedRevision::Any,
                    payload: CommandPayload::SessionSnapshotGet(protocol::SessionSnapshotRequest {
                        offset: json.len() as u64,
                        maximum_bytes: 2_048,
                    }),
                },
                &mut bytes,
            )
            .expect("snapshot command");
        bytes.truncate(len);
        let mut output = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: response.as_mut_ptr(),
            capacity_bytes: response.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: The caller guarantees a live session; the buffers are owned and complete.
        let code = unsafe {
            miso_engine_v1_submit_command(session, bytes.as_ptr(), bytes.len() as u64, &mut output)
        };
        assert_eq!(code, RESULT_OK, "snapshot");
        let Ok(protocol::DecodedTypedResponseFrame::Success {
            header,
            payload: protocol::DecodedSuccessResponsePayload::SessionSnapshot(page),
        }) = ProtocolCodec::default().decode_typed_response(
            &response[..output.required_bytes as usize],
            &mut protocol::DecodeScratch::new(&mut fields),
        )
        else {
            panic!("expected a snapshot page")
        };
        json.extend_from_slice(page.canonical_json_chunk);
        if page.eof {
            return (
                header.revision.0,
                String::from_utf8(json).expect("UTF-8 snapshot"),
            );
        }
    }
}

/// One constant quantum offered to the fixture source.
///
/// # Safety
///
/// `session` must be live and used by this thread alone for the call.
unsafe fn submit_constant_c(session: *mut Session, generation: u64, start_frame: u64) -> u32 {
    let left = [CONSTANT_PCM[0]; QUANTUM];
    let right = [CONSTANT_PCM[1]; QUANTUM];
    let planes = [left.as_ptr(), right.as_ptr()];
    let chunk = SourceChunk {
        struct_size: SOURCE_CHUNK_SIZE,
        sample_rate_hz: 48_000,
        generation,
        start_frame,
        planes: planes.as_ptr(),
        plane_count: 2,
        frames: QUANTUM as u32,
        end_of_region: 0,
        reserved0: 0,
    };
    let mut report = SubmitReport {
        struct_size: SUBMIT_REPORT_SIZE,
        reserved0: 0,
        accepted_frames: 0,
        cumulative_written_frames: 0,
        active_generation: 0,
    };
    let id = b"fixture-source";
    // SAFETY: The caller guarantees a live session; every borrowed buffer outlives the call.
    unsafe {
        miso_engine_v1_source_submit_planar_f32(
            session,
            id.as_ptr(),
            id.len() as u64,
            &chunk,
            &mut report,
        )
    }
}

/// The control thread's source feed: constant quanta, contiguous from the last seek.
struct ConstantFeed {
    generation: u64,
    fed: u64,
}

impl ConstantFeed {
    const fn new() -> Self {
        Self {
            generation: 1,
            fed: 0,
        }
    }

    /// Submits constant quanta until the ring is full.
    ///
    /// A stale generation means a transaction changed the source's declaration, which restarts it
    /// in a new ring at generation 1, frame 0 (#1273): the feed seeks to frame 0 under its next
    /// generation, as a host restarts its feed, and keeps feeding. The feed seeks once before its
    /// first submission, so a restarted source's generation 1 is always stale to it.
    ///
    /// # Safety
    ///
    /// `session` must be live and used by this thread alone for the call.
    unsafe fn fill(&mut self, session: *mut Session) -> Result<(), String> {
        let id = b"fixture-source";
        if self.generation == 1 {
            self.generation = 2;
            // SAFETY: As the caller guarantees.
            let code =
                unsafe { miso_engine_v1_source_seek(session, id.as_ptr(), id.len() as u64, 2, 0) };
            if code != RESULT_OK {
                return Err(format!("first seek: {code}"));
            }
        }
        // Bounded: the ring holds a few quanta.
        for _ in 0..64 {
            // SAFETY: As the caller guarantees.
            let code = unsafe { submit_constant_c(session, self.generation, self.fed) };
            // SAFETY: As the caller guarantees.
            let stale = code == RESULT_INVALID_ARGUMENT
                && unsafe { last_error_c(session) } == b"source.generation.stale";
            match code {
                RESULT_OK => self.fed += QUANTUM as u64,
                RESULT_BACKPRESSURE => return Ok(()),
                RESULT_INVALID_ARGUMENT if stale => {
                    self.generation += 1;
                    // SAFETY: As the caller guarantees.
                    let code = unsafe {
                        miso_engine_v1_source_seek(
                            session,
                            id.as_ptr(),
                            id.len() as u64,
                            self.generation,
                            0,
                        )
                    };
                    match code {
                        RESULT_OK => self.fed = 0,
                        RESULT_BACKPRESSURE => return Ok(()),
                        code => return Err(format!("seek after a swap: {code}")),
                    }
                }
                code => {
                    // SAFETY: As the caller guarantees.
                    let error = unsafe { last_error_c(session) };
                    return Err(format!(
                        "submit at frame {}: {code} {}",
                        self.fed,
                        String::from_utf8_lossy(&error)
                    ));
                }
            }
        }
        Ok(())
    }
}

/// Feeds the source and renders one block on this thread; returns the block's planar PCM.
///
/// # Safety
///
/// Both handles must be live and used by this thread alone for the call.
unsafe fn fed_render_c(
    session: *mut Session,
    plan: *mut Plan,
    feed: &mut ConstantFeed,
    block: u64,
) -> Vec<f32> {
    // SAFETY: As the caller guarantees.
    unsafe { feed.fill(session) }.unwrap_or_else(|failure| panic!("{failure}"));
    let mut pcm = vec![f32::NAN; QUANTUM * 2];
    let output = PlanarOutput {
        struct_size: PLANAR_OUTPUT_SIZE,
        channels: 2,
        samples: pcm.as_mut_ptr(),
        sample_capacity: pcm.len() as u64,
        frames: QUANTUM as u32,
        plane_stride_samples: QUANTUM as u32,
        reserved: [0; 2],
    };
    // SAFETY: As the caller guarantees; `output` names owned storage.
    let code = unsafe { miso_engine_v1_render_f32_planar(plan, block * QUANTUM as u64, &output) };
    assert_eq!(code, RESULT_OK, "render block {block}");
    pcm
}

fn bits(pcm: &[f32]) -> Vec<u32> {
    pcm.iter().map(|sample| sample.to_bits()).collect()
}

/// One transaction's bytes.
fn transaction(request_id: u64, revision: u64, edits: &[SessionEdit]) -> Vec<u8> {
    let mut bytes = vec![0_u8; 4_096];
    let len = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(request_id).expect("nonzero request"),
                expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                payload: CommandPayload::SessionTransactionApply(edits),
            },
            &mut bytes,
        )
        .expect("transaction command");
    bytes.truncate(len);
    bytes
}

/// The race session (#1258 D1): the nine-track fixture with an empty console, no inserts and no
/// input filter, every delay zero, and a source long enough for any run. Nothing on it keeps
/// state that depends on history, so the final block depends only on the final values.
fn race_session() -> String {
    let mut model = session::parse_session_json(SESSION).expect("fixture");
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    model.sources[0].frames = 48_000 * 3_600;
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
        for lane in [&mut track.builtins.left, &mut track.builtins.right] {
            lane.hpf_hz = 0.0;
            lane.lpf_hz = 0.0;
            lane.delay_samples = 0;
        }
    }
    session::canonical_session_json(&model).expect("canonical race session")
}

/// The largest pan smoothing the race edits use: one quantum (D1).
const RACE_MAX_SMOOTHING: u32 = QUANTUM as u32;

/// The control thread's edit sequence: a live fader, pan or mute edit on each of the nine tracks
/// in turn, and every eighth edit a structural one that renders identically (D2: a new content
/// string on the source, which the race never reads by content).
struct RaceEdits {
    tracks: Vec<StableId>,
    faders: Vec<session::DualMonoFader>,
    source: session::Source,
}

impl RaceEdits {
    fn new(document: &str) -> Self {
        let model = session::parse_session_json(document).expect("race session");
        Self {
            tracks: model.tracks.iter().map(|track| track.id.clone()).collect(),
            faders: model
                .tracks
                .iter()
                .map(|track| track.fader.clone())
                .collect(),
            source: model.sources[0].clone(),
        }
    }

    /// Edit `index`. A fader or pan edit takes a value no earlier edit of the same kind on the
    /// same track used, and a mute edit mostly flips its track's left mute, so a lost edit leaves
    /// its track's value wrong unless a later edit overwrites it.
    fn edit(&mut self, index: u64) -> (bool, SessionEdit) {
        if index % 8 == 7 {
            let digits = if (index / 8).is_multiple_of(2) {
                "ab"
            } else {
                "cd"
            };
            return (
                false,
                SessionEdit::SetSourceContent {
                    source_id: self.source.id.clone(),
                    content: format!("blake3:{}", digits.repeat(32)),
                    channels: self.source.channels,
                    bit_depth: self.source.bit_depth,
                    frames: self.source.frames,
                },
            );
        }
        let track = (index % self.tracks.len() as u64) as usize;
        let track_id = self.tracks[track].clone();
        let step = index / self.tracks.len() as u64;
        let edit = match step % 3 {
            0 => {
                let fader = &mut self.faders[track];
                fader.left_db = -0.5 - 0.25 * (index % 37) as f32;
                fader.right_db = fader.left_db - 1.5;
                SessionEdit::SetTrackFader {
                    track_id,
                    fader: fader.clone(),
                }
            }
            1 => SessionEdit::SetTrackMatrixOrPan {
                track_id,
                matrix_or_pan: session::MatrixOrPan::Pan {
                    left: 0.25 + 0.0625 * ((index * 7) % 11) as f32,
                    right: 1.0 - 0.0625 * ((index * 5) % 13) as f32,
                    smoothing_samples: 16 * (1 + (index % 8) as u32),
                },
            },
            _ => {
                let fader = &mut self.faders[track];
                // Mute edits flip the left lane of each track in turn (alternating with each
                // third step, offset by track), and the right lane is never muted, so the final
                // mix always carries every track.
                let mute = (step / 3 + track as u64).is_multiple_of(2);
                fader.left_mute = mute;
                fader.right_mute = false;
                SessionEdit::SetTrackFader {
                    track_id,
                    fader: fader.clone(),
                }
            }
        };
        (true, edit)
    }
}

/// What one race run counted.
#[derive(Debug, Default)]
struct RaceCounts {
    live: u64,
    structural: u64,
    live_backpressure: u64,
    plan_backpressure: u64,
    event_backpressure: u64,
    retries: u64,
    /// Plan replacements the render thread swapped in (`plan_replacement_count`).
    replacements: u64,
    overlapped: u64,
    blocks: u64,
}

/// Edits per run, and edits between two waits for render progress (several per render period).
///
/// The last structural edit is edit 151, so the run ends with four live edits that commit while
/// its candidate is pending or just swapped in, and no later rebuild re-prepares their values
/// from the committed model: a live edit lost to the retiring plan stays lost in the final block.
/// They are a pan (152) and three mutes (153-155), of which 153 and 155 flip a value and push a
/// record; 154 repeats its track's value and commits with no record.
const RACE_EDITS: u64 = 156;
const RACE_EDITS_PER_BLOCK: u64 = 3;
/// Render blocks after an edit's first refusal beyond which another refusal fails the run, as
/// `WEDGE_BLOCKS` bounds a structural replacement in `plan_swap_race.rs`'s `race_plan_swaps`.
const RACE_WEDGE_BLOCKS: u64 = 256;

/// One submission's outcome in the race.
enum Submitted {
    Committed,
    /// `RESULT_BACKPRESSURE` with `control.live.backpressure`.
    LiveBackpressure,
    /// `RESULT_BACKPRESSURE` with `control.plan.backpressure`.
    PlanBackpressure,
    /// `RESULT_OK` with the protocol's own `Backpressure` status.
    EventBackpressure,
}

/// The control thread's side of one race run.
struct RaceControl<'a> {
    session: *mut Session,
    rendered: &'a AtomicU64,
    in_render: &'a AtomicBool,
    stop: &'a AtomicBool,
    feed: ConstantFeed,
    counts: RaceCounts,
    revision: u64,
    request_id: u64,
}

impl RaceControl<'_> {
    /// Waits for one more rendered block, feeding the source and draining the events meanwhile.
    fn await_block(&mut self) -> Result<(), String> {
        use core::sync::atomic::Ordering;
        let target = self.rendered.load(Ordering::Acquire) + 1;
        let deadline = std::time::Instant::now() + PROGRESS_DEADLINE;
        loop {
            if self.stop.load(Ordering::Acquire) {
                return Err("the render thread stopped".to_owned());
            }
            // SAFETY: This thread is the session's only control caller.
            unsafe {
                self.feed.fill(self.session)?;
                drain_events_c(self.session)?;
            }
            if self.rendered.load(Ordering::Acquire) >= target {
                return Ok(());
            }
            if std::time::Instant::now() > deadline {
                return Err(format!("no block rendered within {PROGRESS_DEADLINE:?}"));
            }
            std::thread::yield_now();
        }
    }

    /// Submits `edit` once under a new request ID; any outcome but a commit or a typed
    /// backpressure is an error.
    fn submit_once(&mut self, edit: &SessionEdit, live: bool) -> Result<Submitted, String> {
        use core::sync::atomic::Ordering;
        self.request_id += 1;
        let request = transaction(self.request_id, self.revision, core::slice::from_ref(edit));
        let overlapping = self.in_render.load(Ordering::SeqCst);
        // SAFETY: This thread is the session's only control caller.
        let (code, header) = unsafe { submit_header(self.session, &request) };
        self.counts.overlapped += u64::from(overlapping && self.in_render.load(Ordering::SeqCst));
        let outcome = match (code, header) {
            (RESULT_OK, Some(header)) if header.status == StatusCode::Ok => {
                if header.revision != SessionRevision(self.revision + 1) {
                    return Err(format!(
                        "committed at revision {}, not {}",
                        header.revision.0,
                        self.revision + 1
                    ));
                }
                self.revision += 1;
                if live {
                    self.counts.live += 1;
                } else {
                    self.counts.structural += 1;
                }
                Submitted::Committed
            }
            (RESULT_OK, Some(header)) if header.status == StatusCode::Backpressure => {
                self.counts.event_backpressure += 1;
                Submitted::EventBackpressure
            }
            (RESULT_BACKPRESSURE, _) => {
                // SAFETY: This thread is the session's only control caller.
                match unsafe { last_error_c(self.session) }.as_slice() {
                    b"control.live.backpressure" => {
                        self.counts.live_backpressure += 1;
                        Submitted::LiveBackpressure
                    }
                    b"control.plan.backpressure" => {
                        self.counts.plan_backpressure += 1;
                        Submitted::PlanBackpressure
                    }
                    other => {
                        return Err(format!(
                            "RESULT_BACKPRESSURE with {}",
                            String::from_utf8_lossy(other)
                        ));
                    }
                }
            }
            (code, header) => {
                // SAFETY: This thread is the session's only control caller.
                let error = unsafe { last_error_c(self.session) };
                return Err(format!(
                    "{} edit: result {code}, header {header:?}, last error {}",
                    if live { "live" } else { "structural" },
                    String::from_utf8_lossy(&error)
                ));
            }
        };
        // SAFETY: This thread is the session's only control caller.
        unsafe { drain_events_c(self.session) }?;
        Ok(outcome)
    }

    /// Commits `edit`, retrying a refusal under a new request ID once a block has rendered, until
    /// [`RACE_WEDGE_BLOCKS`] blocks after its first refusal.
    fn commit(&mut self, edit: &SessionEdit, live: bool) -> Result<(), String> {
        use core::sync::atomic::Ordering;
        let mut first_refusal = None;
        loop {
            if let Submitted::Committed = self.submit_once(edit, live)? {
                return Ok(());
            }
            let now = self.rendered.load(Ordering::Acquire);
            let since = *first_refusal.get_or_insert(now);
            if now - since > RACE_WEDGE_BLOCKS {
                return Err(format!(
                    "still refused {} blocks after its first refusal",
                    now - since
                ));
            }
            self.counts.retries += 1;
            self.await_block()?;
        }
    }
}

/// #1258 gate 1, one run: live fader, mute and pan edits and a structural edit every eighth,
/// committed on this thread through `miso_engine_v1_submit_command` while a scoped thread renders
/// back-to-back blocks through `miso_engine_v1_render_f32_planar`; then the final block against a
/// fresh plan of the final committed snapshot.
///
/// Halfway through, the render thread parks (as a paused audio session does) while this thread
/// fills one track's fader lane to typed live backpressure and submits a second structural edit
/// behind a pending candidate, which is typed plan backpressure; both are retried once the render
/// thread resumes. Without the pause the render thread drains the lanes faster than this thread
/// fills them, and no retry would ever run.
///
/// The render thread never asserts: it records a refused block and stops. Every wait on this
/// thread has a deadline, and `StopOnDrop` stops the render thread however this thread leaves the
/// scope, so a failure on either side ends the run instead of hanging the join (#1251, lesson d).
/// `bench_support::producer::render_while_producing` cannot serve here: `bench_support` installs
/// its own global allocator, and this file's counting allocator is the one the counts come from.
fn race_live_edits(run: usize) -> RaceCounts {
    use core::sync::atomic::Ordering;

    let document = race_session();
    // SAFETY: The returned handles are uniquely owned until the matching destroy calls below.
    let (session, plan) = unsafe { compile_c(&document, &limits()) };
    let mut feed = ConstantFeed::new();
    // Warm-up on this thread: the first blocks, fed, before the race starts.
    for block in 0..2 {
        // SAFETY: No other thread uses either handle yet.
        unsafe { fed_render_c(session, plan, &mut feed, block) };
    }
    let first_race_block = 2_u64;
    let plan_address = plan as usize;
    let stop = AtomicBool::new(false);
    let paused = AtomicBool::new(false);
    let parked = AtomicBool::new(false);
    let in_render = AtomicBool::new(false);
    let rendered = AtomicU64::new(first_race_block);
    let mut edits = RaceEdits::new(&document);

    let (control, (render_observed, render_failure)) = std::thread::scope(|scope| {
        let render = scope.spawn(|| {
            let plan = plan_address as *mut Plan;
            let mut pcm = [f32::NAN; QUANTUM * 2];
            let output = PlanarOutput {
                struct_size: PLANAR_OUTPUT_SIZE,
                channels: 2,
                samples: pcm.as_mut_ptr(),
                sample_capacity: pcm.len() as u64,
                frames: QUANTUM as u32,
                plane_stride_samples: QUANTUM as u32,
                reserved: [0; 2],
            };
            let mut block = first_race_block;
            let mut failure = None;
            begin();
            while !stop.load(Ordering::Acquire) {
                if paused.load(Ordering::Acquire) {
                    parked.store(true, Ordering::Release);
                    std::thread::yield_now();
                    continue;
                }
                parked.store(false, Ordering::Release);
                in_render.store(true, Ordering::SeqCst);
                // SAFETY: This thread is the plan's only renderer; `output` is owned storage.
                let code = unsafe {
                    miso_engine_v1_render_f32_planar(plan, block * QUANTUM as u64, &output)
                };
                in_render.store(false, Ordering::SeqCst);
                if code != RESULT_OK {
                    failure = Some((block, code));
                    stop.store(true, Ordering::Release);
                    break;
                }
                block += 1;
                rendered.store(block, Ordering::Release);
            }
            (finish(), failure)
        });

        let _stop = StopOnDrop(&stop);
        let mut control = RaceControl {
            session,
            rendered: &rendered,
            in_render: &in_render,
            stop: &stop,
            feed,
            counts: RaceCounts::default(),
            revision: 42,
            request_id: 1,
        };
        let mut run_edits = || -> Result<(), String> {
            for index in 0..RACE_EDITS {
                if index.is_multiple_of(RACE_EDITS_PER_BLOCK) {
                    control.await_block()?;
                }
                if index == RACE_EDITS / 2 {
                    paused_bursts(&mut control, &mut edits, &paused, &parked)?;
                }
                let (live, edit) = edits.edit(index);
                control
                    .commit(&edit, live)
                    .map_err(|error| format!("edit {index}: {error}"))?;
            }
            Ok(())
        };
        let failure = run_edits().err();
        stop.store(true, Ordering::Release);
        let render_result = render.join().expect("the render thread does not panic");
        let RaceControl {
            feed,
            counts,
            revision,
            request_id,
            ..
        } = control;
        ((feed, counts, revision, request_id, failure), render_result)
    });
    let (mut feed, mut counts, revision, mut request_id, failure) = control;
    assert_eq!(render_failure, None, "run {run}: render refused a block");
    assert_eq!(failure, None, "run {run}: a control call failed");
    assert_eq!(
        render_observed,
        Snapshot {
            allocations: 0,
            deallocations: 0,
            allocated_bytes: 0,
            deallocated_bytes: 0,
        },
        "run {run}: every render call allocated and freed nothing"
    );

    // Settle on this thread: a candidate still pending swaps in at the first block; then render
    // the plan's latency plus one quantum (every pan ramp is at most one quantum), and the next
    // block is the final one.
    let mut block = rendered.load(Ordering::Acquire);
    counts.blocks = block - first_race_block;
    // SAFETY: The race is over; this thread alone uses both handles from here on.
    let (committed_revision, final_document, latency, final_block) = unsafe {
        fed_render_c(session, plan, &mut feed, block);
        block += 1;
        drain_events_c(session).unwrap_or_else(|failure| panic!("{failure}"));
        let latency = resources_c(plan).latency_samples;
        let settle = (latency + u64::from(RACE_MAX_SMOOTHING)).div_ceil(QUANTUM as u64) + 1;
        for _ in 0..settle {
            fed_render_c(session, plan, &mut feed, block);
            block += 1;
        }
        let final_block = fed_render_c(session, plan, &mut feed, block);
        let (committed_revision, final_document) = snapshot_c(session, &mut request_id);
        counts.replacements = plan_replacement_count(plan);
        miso_engine_v1_session_destroy(session);
        miso_engine_v1_plan_destroy(plan);
        (committed_revision, final_document, latency, final_block)
    };
    assert_eq!(
        committed_revision, revision,
        "run {run}: the snapshot's revision"
    );

    // A fresh plan of the final committed snapshot, fed the same constant source.
    // SAFETY: These handles are uniquely owned until the matching destroy calls.
    let reference = unsafe {
        let (session, plan) = compile_c(&final_document, &limits());
        assert_eq!(resources_c(plan).latency_samples, latency);
        let mut feed = ConstantFeed::new();
        let settle = (latency + u64::from(RACE_MAX_SMOOTHING)).div_ceil(QUANTUM as u64) + 1;
        let mut last = Vec::new();
        for block in 0..=settle {
            last = fed_render_c(session, plan, &mut feed, block);
        }
        miso_engine_v1_session_destroy(session);
        miso_engine_v1_plan_destroy(plan);
        last
    };
    assert!(
        reference.iter().all(|sample| *sample != 0.0),
        "run {run}: the final mix carries signal on both lanes"
    );
    // The block is constant on each lane, so a mismatch reports each lane's first sample.
    assert!(
        bits(&final_block) == bits(&reference),
        "run {run}: the raced plan's final block {:?} against a fresh plan of the final snapshot \
         {:?} (first left and right samples)",
        [final_block[0], final_block[QUANTUM]],
        [reference[0], reference[QUANTUM]],
    );
    counts
}

/// The paused phase of [`race_live_edits`]: with the render thread parked, live edits on the
/// first track until its fader lane refuses with `control.live.backpressure` (at most one lane's
/// depth plus one), then a structural edit and a second one behind it, which
/// `control.plan.backpressure` refuses. The render thread resumes and both refusals are retried
/// through [`RaceControl::commit`].
fn paused_bursts(
    control: &mut RaceControl<'_>,
    edits: &mut RaceEdits,
    paused: &AtomicBool,
    parked: &AtomicBool,
) -> Result<(), String> {
    use core::sync::atomic::Ordering;
    // Two whole blocks first, so a candidate the previous structural edit left pending has swapped
    // in and its epoch is published before the render thread parks.
    control.await_block()?;
    control.await_block()?;
    paused.store(true, Ordering::Release);
    let deadline = std::time::Instant::now() + PROGRESS_DEADLINE;
    while !parked.load(Ordering::Acquire) {
        if std::time::Instant::now() > deadline {
            return Err("the render thread never parked".to_owned());
        }
        std::thread::yield_now();
    }
    let track_id = edits.tracks[0].clone();
    let mut refused_live = None;
    for step in 0..=16 {
        edits.faders[0].left_db = -20.0 - 0.25 * step as f32;
        let edit = SessionEdit::SetTrackFader {
            track_id: track_id.clone(),
            fader: edits.faders[0].clone(),
        };
        match control.submit_once(&edit, true)? {
            Submitted::Committed => {}
            Submitted::LiveBackpressure => {
                refused_live = Some(edit);
                break;
            }
            _ => return Err(format!("paused live edit {step}: an unexpected refusal")),
        }
    }
    let refused_live = refused_live.ok_or("a parked render never filled a 16-record fader lane")?;
    let structural = |digits: &str| SessionEdit::SetSourceContent {
        source_id: edits.source.id.clone(),
        content: format!("blake3:{}", digits.repeat(32)),
        channels: edits.source.channels,
        bit_depth: edits.source.bit_depth,
        frames: edits.source.frames,
    };
    let (first, second) = (structural("ef"), structural("01"));
    if !matches!(control.submit_once(&first, false)?, Submitted::Committed) {
        return Err("the paused structural edit was refused".to_owned());
    }
    if !matches!(
        control.submit_once(&second, false)?,
        Submitted::PlanBackpressure
    ) {
        return Err("a structural edit behind a pending candidate was not refused".to_owned());
    }
    paused.store(false, Ordering::Release);
    // The refused live edit goes to the candidate's fresh lanes if the swap has not happened yet,
    // or to the swapped-in plan's; the second structural edit waits for the swap. `retries`
    // counts only the resubmissions `commit` finds refused again.
    control.commit(&refused_live, true)?;
    control.commit(&second, false)
}

/// #1258 gate 1: live fader, mute and pan edits, several per render period, and a structural edit
/// every eighth, against a plan rendering on another thread, twenty times.
///
/// Test value: red if a fader or matrix drain allocates or frees on the render thread, if a live
/// edit that races a plan swap reaches the retiring plan or is lost (the final block differs from
/// a fresh plan of the final snapshot), if epoch synchronization returns `RESULT_INTERNAL` under
/// live traffic, if a `RESULT_BACKPRESSURE` carries any other diagnostic or never clears, or if a
/// live edit rebuilds the plan or a structural one swaps in other than exactly one plan.
#[test]
fn live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free() {
    let mut total = RaceCounts::default();
    for run in 0..20 {
        let counts = race_live_edits(run);
        assert_eq!(counts.structural, RACE_EDITS / 8 + 2, "run {run}");
        assert!(counts.live >= RACE_EDITS - RACE_EDITS / 8, "run {run}");
        assert!(counts.live_backpressure >= 1, "run {run}");
        assert!(counts.plan_backpressure >= 1, "run {run}");
        // Every admitted structural transaction swapped in exactly one plan, and no live edit
        // swapped in one of its own: a live edit that silently rebuilt is counted here.
        assert_eq!(
            counts.replacements, counts.structural,
            "run {run}: plan replacements against admitted structural transactions"
        );
        total.live += counts.live;
        total.structural += counts.structural;
        total.live_backpressure += counts.live_backpressure;
        total.plan_backpressure += counts.plan_backpressure;
        total.event_backpressure += counts.event_backpressure;
        total.retries += counts.retries;
        total.replacements += counts.replacements;
        total.overlapped += counts.overlapped;
        total.blocks += counts.blocks;
    }
    println!("twenty race runs: {total:?}");
    assert!(
        total.overlapped > 0,
        "vacuous: no control call overlapped a render call"
    );
}

// Issue #1206 D2: a C ABI caller bounds submix strips through `maximum_submixes`, the word of the
// compile limits that was `reserved[0]`. Zero means "use `maximum_tracks`" -- what every caller
// written before the word was named passes -- and the two remaining reserved words (#1243 named
// `maximum_vcas` after it) still refuse when nonzero. It lives here, not in a file of its own,
// because this file is the C ABI's approved owner of exported-C `unsafe` calls
// (`scripts/check-realtime-policy.sh`).

const SUBMIX_FIXTURE: &str =
    include_str!("../../../fixtures/session/v1/observation-frame-shape.json");

/// Two tracks routed to the main output, plus three transparent, unrouted submix strips.
fn two_track_three_submix_session() -> String {
    let mut model = session::parse_session_json(SUBMIX_FIXTURE).expect("fixture parses");
    model.tracks.truncate(2);
    let kept: Vec<StableId> = model.tracks.iter().map(|track| track.id.clone()).collect();
    model.routes.retain(|route| match &route.source {
        session::RouteSource::Track { track_id, .. } => kept.contains(track_id),
        _ => true,
    });
    for index in 0..3 {
        let id = StableId::parse(&format!("bus{index}")).expect("stable id");
        model
            .submixes
            .push(session::Submix::unity(id, &model.console));
    }
    session::canonical_session_json(&model).expect("canonical session")
}

/// [`two_track_three_submix_session`] with the first track routed through `bus0` to the main
/// output: three submix strips beside two tracks, and one route into a submix, the route kind that
/// would get a live lane if the C ABI selected route lanes (`HostLiveLanes::routes`).
fn two_track_routed_submix_session() -> String {
    let mut model =
        session::parse_session_json(&two_track_three_submix_session()).expect("submix session");
    let bus = StableId::parse("bus0").expect("stable id");
    let session::RouteSource::Track { track_id, .. } = &model.routes[0].source else {
        panic!("the first route is a track's")
    };
    assert_eq!(
        track_id, &model.tracks[0].id,
        "the first route is the first track's"
    );
    let mut out = model.routes[0].clone();
    out.id = StableId::parse("bus0-main").expect("stable id");
    out.source = session::RouteSource::Submix {
        submix_id: bus.clone(),
        tap: session::SendTap::PostPan,
    };
    let send = &mut model.routes[0];
    send.id = StableId::parse("track-bus0").expect("stable id");
    send.destination = session::RouteDestination::SubmixInput { submix_id: bus };
    model.routes.push(out);
    session::canonical_session_json(&model).expect("canonical session")
}

fn submix_limits(maximum_tracks: u64, maximum_submixes: u64) -> CompileLimits {
    CompileLimits {
        struct_size: COMPILE_LIMITS_SIZE,
        source_ring_frames: 1_024,
        maximum_automation_spans_per_block: 128,
        reserved0: 0,
        maximum_document_bytes: 1_000_000,
        maximum_diagnostic_bytes: 4_096,
        maximum_tracks,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 1_000_000_000,
        maximum_source_total_bytes: 100_000_000,
        maximum_source_overhead_bytes: 100_000_000,
        maximum_effect_state_bytes: 1_000_000_000,
        maximum_effect_scratch_bytes: 1_000_000_000,
        maximum_builtin_retained_bytes: 1_000_000_000,
        maximum_capi_retained_bytes: 100_000_000,
        maximum_named_allocation_bytes: 1_000_000_000,
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
        maximum_control_frame_bytes: 4_096,
        maximum_replay_bytes: 8_192,
        maximum_replay_entries: 16,
        maximum_submixes,
        maximum_vcas: 0,
        reserved: [0; 2],
    }
}

/// One `miso_engine_v1_compile_session`: the result code and the diagnostic bytes. Every handle it
/// publishes is destroyed before it returns.
fn compile_result_c(document: &str, compile_limits: &CompileLimits) -> (u32, Vec<u8>) {
    let config = EngineConfig {
        struct_size: ENGINE_CONFIG_SIZE,
        abi_version: ABI_VERSION,
        reserved: [0; 4],
    };
    let mut engine = ptr::null_mut();
    let mut session = ptr::null_mut();
    let mut plan = ptr::null_mut();
    let mut storage = [0_u8; 4_096];
    let mut diagnostics = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: storage.as_mut_ptr(),
        capacity_bytes: storage.len() as u64,
        required_bytes: 0,
    };
    // SAFETY: Every descriptor and output location stays live for each call, and every handle
    // published here is destroyed exactly once before the engine.
    unsafe {
        assert_eq!(
            miso_engine_v1_engine_create(&config, &mut engine),
            RESULT_OK
        );
        let result = miso_engine_v1_compile_session(
            engine,
            document.as_ptr(),
            document.len() as u64,
            compile_limits,
            &mut diagnostics,
            &mut session,
            &mut plan,
        );
        assert_eq!(session.is_null(), result != RESULT_OK);
        assert_eq!(plan.is_null(), result != RESULT_OK);
        if result == RESULT_OK {
            miso_engine_v1_plan_destroy(plan);
            miso_engine_v1_session_destroy(session);
        }
        miso_engine_v1_engine_destroy(engine);
        let written = usize::try_from(diagnostics.required_bytes)
            .unwrap_or(storage.len())
            .min(storage.len());
        (result, storage[..written].to_vec())
    }
}

fn assert_prepares(document: &str, compile_limits: &CompileLimits, case: &str) {
    let (result, diagnostics) = compile_result_c(document, compile_limits);
    assert_eq!(
        result,
        RESULT_OK,
        "{case}: {}",
        String::from_utf8_lossy(&diagnostics)
    );
}

fn assert_count_refusal(document: &str, compile_limits: &CompileLimits, case: &str) {
    let (result, diagnostics) = compile_result_c(document, compile_limits);
    assert_eq!(result, RESULT_COMPILE_REJECTED, "{case}");
    assert_eq!(diagnostics, b"host.resource.count\t$\n", "{case}");
}

/// Test value: turns red if the C bound ignores the new word, treats zero as "no submixes" (or as
/// unbounded), or stops refusing the remaining reserved words.
#[test]
fn maximum_submixes_bounds_submixes_and_zero_defers_to_maximum_tracks() {
    let document = two_track_three_submix_session();

    // Zero: the submix bound is `maximum_tracks`.
    assert_count_refusal(
        &document,
        &submix_limits(2, 0),
        "zero word, three submixes over two tracks",
    );
    assert_prepares(
        &document,
        &submix_limits(3, 0),
        "zero word, three submixes within three tracks",
    );

    // Nonzero: the word is the bound, whatever `maximum_tracks` is.
    assert_prepares(
        &document,
        &submix_limits(2, 3),
        "three submixes at a cap of three",
    );
    assert_count_refusal(
        &document,
        &submix_limits(2, 2),
        "three submixes over a cap of two",
    );
    assert_count_refusal(
        &document,
        &submix_limits(3, 2),
        "the word overrides a larger track cap",
    );

    // The two remaining reserved words still refuse, each on its own.
    for word in 0..2 {
        let mut nonzero = submix_limits(2, 3);
        nonzero.reserved[word] = 1;
        let (result, _) = compile_result_c(&document, &nonzero);
        assert_eq!(result, RESULT_INVALID_ARGUMENT, "reserved[{word}]");
    }
}

// Issue #1243 D2: a C ABI caller bounds VCA groups through `maximum_vcas`, the word after
// `maximum_submixes` that was `reserved[0]`. Zero means "use `maximum_tracks`", exactly as for
// submixes; the word is a bound of its own, never `maximum_submixes` (held at zero below, which
// for this submix-free session is any track cap).

/// Two tracks routed to the main output, plus three empty, unity VCA groups.
fn two_track_three_vca_session() -> String {
    let mut model = session::parse_session_json(SUBMIX_FIXTURE).expect("fixture parses");
    model.tracks.truncate(2);
    let kept: Vec<StableId> = model.tracks.iter().map(|track| track.id.clone()).collect();
    model.routes.retain(|route| match &route.source {
        session::RouteSource::Track { track_id, .. } => kept.contains(track_id),
        _ => true,
    });
    for index in 0..3 {
        model.vcas.push(session::Vca {
            id: StableId::parse(&format!("vca{index}")).expect("stable id"),
            fader: session::DualMonoFader {
                left_db: 0.0,
                right_db: 0.0,
                left_mute: false,
                right_mute: false,
            },
            members: Vec::new(),
        });
    }
    session::canonical_session_json(&model).expect("canonical session")
}

fn vca_limits(maximum_tracks: u64, maximum_vcas: u64) -> CompileLimits {
    CompileLimits {
        maximum_vcas,
        ..submix_limits(maximum_tracks, 0)
    }
}

/// Test value: turns red if the C bound ignores the new word, treats zero as "no VCAs" (or as
/// unbounded), reads `maximum_submixes` in its place, or stops refusing the remaining reserved
/// words while the named word is set.
#[test]
fn maximum_vcas_bounds_vcas_and_zero_defers_to_maximum_tracks() {
    let document = two_track_three_vca_session();

    // Zero: the VCA bound is `maximum_tracks`.
    assert_count_refusal(
        &document,
        &vca_limits(2, 0),
        "zero word, three VCAs over two tracks",
    );
    assert_prepares(
        &document,
        &vca_limits(3, 0),
        "zero word, three VCAs within three tracks",
    );

    // Nonzero: the word is the bound, whatever `maximum_tracks` is.
    assert_prepares(&document, &vca_limits(2, 3), "three VCAs at a cap of three");
    assert_count_refusal(&document, &vca_limits(2, 2), "three VCAs over a cap of two");
    assert_count_refusal(
        &document,
        &vca_limits(3, 2),
        "the word overrides a larger track cap",
    );

    // Only the named word is freed: each remaining reserved word still refuses on its own.
    for word in 0..2 {
        let mut nonzero = vca_limits(2, 3);
        nonzero.reserved[word] = 1;
        let (result, _) = compile_result_c(&document, &nonzero);
        assert_eq!(result, RESULT_INVALID_ARGUMENT, "reserved[{word}]");
    }
}
