//! Issue #1217 gate 4: a plan builds its route-activity table only when a route gate silences,
//! and the graph estimate charges what that table retains.
//!
//! The session is the nine-track fixture, every track routed to the session output by its own
//! route. One compile leaves every route open; the other mutes `eq5-main`, which PDC does not
//! delay. Both are bound with the route fold declined, so the two plans differ in the gate and
//! the table alone.

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry, prepare_native_session_effects,
};
use engine::realtime::RenderError;
use graph::{GraphBindingBlock, GraphCompileCaps, GraphNodeBinding, GraphRuntimeBindings};
use graph::{GraphRuntimeProcessor, test_only_route_activity_built};
use graph_compiler::{Backend, GraphBuiltinsCompileRequest, GraphCompiler};
use session::{CompileCaps, SessionModel, compile_session, parse_session_json};

/// Nine tracks, each routed to the session output by its own route.
const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

/// A bound source that writes nothing: the binding's audio is not this file's concern.
struct Silent;
impl GraphRuntimeProcessor for Silent {
    fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        Ok(())
    }
}

/// One compiled and bound plan.
struct Bound {
    /// The compile's `graph_metadata_bytes`.
    metadata: u64,
    /// Bytes the compile and bind left live (`requested - released` over a window that frees
    /// nothing it did not allocate): everything the bound plan retains.
    live: u64,
    /// Whether the bind built a route-activity table.
    activity: bool,
}

/// Compile `model` at this build's width and bind it with the route fold declined, measuring
/// what the preparation leaves live on this thread.
fn bind(model: &SessionModel) -> Bound {
    let session = compile_session(
        model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("the session compiles");
    let registry = launch_native_effect_registry().expect("launch registry");
    assert_installed();
    let mark = current_thread_counters();
    let effects = prepare_native_session_effects(
        &session,
        registry,
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 24,
            maximum_scratch_bytes: 1 << 24,
            maximum_automation_spans_per_block: 128,
        },
    )
    .expect("the native effects prepare");
    let builtins = prepare_session_builtins(
        &session,
        &[],
        BuiltinCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_total_retained_payload_bytes: u64::MAX,
            maximum_total_meter_items: u64::MAX,
            maximum_total_meter_bytes: u64::MAX,
            maximum_single_allocation_bytes: u64::MAX,
            maximum_meter_streams: u64::MAX,
            maximum_period_frames: u32::MAX,
            maximum_peak_hold_frames: u32::MAX,
            maximum_smoothing_samples: u32::MAX,
        },
    )
    .expect("the builtins prepare");
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch: Backend::current(),
        plan_id: 1_217,
        effects,
        builtins,
        caps: GraphCompileCaps {
            maximum_nodes: u64::MAX,
            maximum_edges: u64::MAX,
            maximum_schedule_items: u64::MAX,
            maximum_dependency_levels: u64::MAX,
            maximum_audio_buffer_samples: u64::MAX,
            maximum_delay_samples_per_edge: u64::MAX,
            maximum_total_delay_samples: u64::MAX,
            maximum_graph_bytes: u64::MAX,
            maximum_plan_bytes: u64::MAX,
            maximum_single_allocation_bytes: u64::MAX,
            maximum_finite_tail_samples: u64::MAX,
        },
    })
    .unwrap_or_else(|failure| panic!("graph: {:?}", failure.diagnostics));
    assert!(
        artifact.graph().inserted_delays.is_empty(),
        "no route is delayed, so a muted one is inactive"
    );
    let metadata = artifact.graph_resource_estimate().graph_metadata_bytes;
    let envelope = artifact.envelope();
    let nodes = artifact
        .external_binding_nodes()
        .map(|node| GraphNodeBinding::new(node.clone(), Box::new(Silent)))
        .collect();
    graph::test_only_set_route_fold_declined(true);
    let bound = artifact.into_bound(GraphRuntimeBindings {
        envelope,
        nodes,
        observers: Vec::new(),
    });
    graph::test_only_set_route_fold_declined(false);
    let plan = bound
        .unwrap_or_else(|failure| panic!("bind: {}", failure.code))
        .plan;
    let activity = test_only_route_activity_built();
    let window = current_thread_delta_since(mark);
    drop(plan);
    Bound {
        metadata,
        live: window
            .requested_bytes
            .checked_sub(window.released_bytes)
            .expect("the window frees nothing it did not allocate"),
        activity,
    }
}

/// #1217 gate 4. The open plan builds no route-activity table; the plan with one muted route
/// builds one, and its `graph_metadata_bytes` exceed the open plan's by at least the extra bytes
/// the audited allocator saw the muted plan retain -- which are the table's, since the two plans
/// differ in nothing else (that difference is asserted nonzero).
///
/// Red if the table is built for a plan that cannot need it, or is left out of the estimate.
#[test]
fn a_route_activity_table_is_built_only_for_a_silencing_gate_and_is_charged() {
    let mut model = parse_session_json(SESSION).expect("fixture");
    let open = bind(&model);
    assert!(
        !open.activity,
        "a plan without a silencing route builds no route-activity table"
    );
    let route = model
        .routes
        .iter_mut()
        .find(|route| route.id.as_str() == "eq5-main")
        .expect("eq5-main");
    route.mute = true;
    let muted = bind(&model);
    assert!(muted.activity, "a muted route builds the table");
    let retained = muted
        .live
        .checked_sub(open.live)
        .filter(|bytes| *bytes > 0)
        .unwrap_or_else(|| {
            panic!(
                "the muted plan retains more than the open one: {} against {}",
                muted.live, open.live
            )
        });
    let charged = muted
        .metadata
        .checked_sub(open.metadata)
        .expect("the muted plan charges no less than the open one");
    assert!(
        charged >= retained,
        "graph_metadata_bytes grew by {charged}, the bound plan by {retained}"
    );
    eprintln!("route_activity gate 4: charged {charged} bytes, retained {retained}");
}
