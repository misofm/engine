//! Issue #2199: a graph projected before any effect processor exists, then compiled.
//!
//! `GraphCompiler::project` plans the graph and its banks from the projected effects alone, and
//! `GraphCompiler::compile_projected` binds the planned banks and prepares a processor only for an
//! instance the plan leaves per node (the #2194 residual (c)). The console fixture banks its console
//! EQ and its insert compressor on every track; one delay insert renders per node.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use bench_support::alloc::{assert_installed, current_thread_peak, current_thread_peak_reset};
use builtins_compiler::{BuiltinCompileCaps, PreparedBuiltinsSession, prepare_session_builtins};
use effect_compiler::{
    EffectCompileCaps, EffectProjectedSession, launch_native_effect_registry,
    project_native_session_effects,
};
use effect_contract::{
    EffectDescriptor, EffectPrepareError, NativeEffectFactory, NativeEffectRegistry,
    PrepareEffectBankRequest, PrepareEffectRequest, PreparedEffect, PreparedEffectBank,
    QualityDescriptor,
};
use graph::GraphCompileCaps;
use graph_compiler::{Backend, GainLiveness, GraphCompiler, GraphProjectionRequest};
use session::{
    CompileCaps, CompiledSession, Effect, EffectIdentity, EffectQuality, LinkMode,
    SidechainDeclaration, StableId, compile_session, parse_session_json,
};

const CONSOLE: &str = include_str!("../../../fixtures/session/v1/console-sixty-four-track.json");

fn graph_caps() -> GraphCompileCaps {
    GraphCompileCaps {
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
    }
}

fn builtins(session: &CompiledSession) -> PreparedBuiltinsSession {
    prepare_session_builtins(
        session,
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
    .expect("the console's builtins")
}

/// The console fixture, with a delay inserted after the first track's compressor: the delay never
/// banks, so it renders per node beside the banked EQs and compressors.
fn console() -> CompiledSession {
    let mut model = parse_session_json(CONSOLE).expect("the console fixture");
    model.tracks[0].inserts.effects.push(Effect {
        id: StableId::parse("delay").expect("an effect ID"),
        identity: EffectIdentity::Native {
            effect_id: StableId::parse("miso.delay").expect("a native ID"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        params: Vec::new(),
        sidechain: SidechainDeclaration::None,
    });
    compile_session(
        &model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("the compiled console")
}

/// A launch factory that counts the processors it prepares.
struct Counting {
    delegate: Arc<dyn NativeEffectFactory>,
    prepared: Arc<AtomicU64>,
}

impl NativeEffectFactory for Counting {
    fn descriptor(&self) -> &'static EffectDescriptor {
        self.delegate.descriptor()
    }
    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<PreparedEffect, EffectPrepareError> {
        self.prepared.fetch_add(1, Ordering::SeqCst);
        self.delegate.prepare(request)
    }
    fn designed_channel_symmetry(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<bool, EffectPrepareError> {
        self.delegate.designed_channel_symmetry(request)
    }
    fn prepared_inline_bytes(&self) -> u64 {
        self.delegate.prepared_inline_bytes()
    }
    fn prepared_undeclared_heap_bytes(&self, quality: &QualityDescriptor) -> u64 {
        self.delegate.prepared_undeclared_heap_bytes(quality)
    }
    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<PreparedEffectBank>, EffectPrepareError> {
        self.delegate.bind_homogeneous_bank(request)
    }
}

/// The launch registry, every factory counting into one counter.
fn counting_registry() -> (NativeEffectRegistry, Arc<AtomicU64>) {
    let launch = launch_native_effect_registry().expect("launch registry");
    let prepared = Arc::new(AtomicU64::new(0));
    let factories: Vec<Box<dyn NativeEffectFactory>> = launch
        .descriptors()
        .map(|descriptor| {
            Box::new(Counting {
                delegate: launch
                    .get_shared_ascii(descriptor.id.as_str())
                    .expect("a registered factory"),
                prepared: Arc::clone(&prepared),
            }) as Box<dyn NativeEffectFactory>
        })
        .collect();
    let registry = NativeEffectRegistry::new(factories).expect("the counting registry");
    (registry, prepared)
}

fn projected(session: &CompiledSession, registry: &NativeEffectRegistry) -> EffectProjectedSession {
    project_native_session_effects(
        session,
        registry,
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 30,
            maximum_scratch_bytes: 1 << 28,
            maximum_automation_spans_per_block: u32::MAX,
        },
    )
    .expect("the projected console effects")
}

fn request(
    effects: EffectProjectedSession,
    builtins: &PreparedBuiltinsSession,
) -> GraphProjectionRequest<'_> {
    GraphProjectionRequest {
        plan_id: 2_199,
        effects,
        builtins,
        caps: graph_caps(),
        dispatch: Backend::current(),
        input_floors: Default::default(),
        gain_liveness: GainLiveness::FIXED,
    }
}

/// A projected compile prepares a processor for every instance its plan leaves per node, once,
/// and for no banked member, so its peak stays under the plan's retained estimate plus every
/// instance's processor.
///
/// Test value: red when a banked member gets a per-instance processor, as before #2199, when the
/// effect compiler prepared every instance and the graph compiler held them all while it bound the
/// banks: the prepare count is then every instance's, and the peak holds every processor beside
/// the plan.
#[test]
fn a_projected_compile_prepares_no_banked_members_processor() {
    assert_installed();
    let session = console();
    let (registry, prepared) = counting_registry();
    let builtins = builtins(&session);
    let effects = projected(&session, &registry);
    let instances = effects.entries.len();
    assert_eq!(
        prepared.load(Ordering::SeqCst),
        0,
        "a projection prepares nothing"
    );
    current_thread_peak_reset();
    let projection = GraphCompiler::project(request(effects, &builtins))
        .unwrap_or_else(|failure| panic!("the console projects: {:?}", failure.diagnostics));
    let retained = projection.resource_estimate().session_plus_plan_bytes;
    let artifact = GraphCompiler::compile_projected(projection, builtins)
        .unwrap_or_else(|failure| panic!("the console compiles: {:?}", failure.diagnostics));
    let peak = current_thread_peak().peak_bytes;
    let banked: usize = artifact.graph().effect_bank_members().map(<[_]>::len).sum();
    let per_node = instances - banked;
    assert!(
        banked > 0 && per_node > 0,
        "the console banks some instances and leaves some per node ({banked}, {per_node})"
    );
    assert_eq!(
        prepared.load(Ordering::SeqCst),
        per_node as u64,
        "one processor per per-node instance, none for a banked member"
    );
    // What every instance's processor holds, measured off the window.
    let processors: u64 = projected(&session, &registry)
        .entries
        .iter()
        .map(|entry| {
            let processor = entry.prepare().expect("a launch processor");
            size_of_val(&*processor) as u64 + processor.retained_bytes()
        })
        .sum();
    assert!(
        peak < retained + processors,
        "the compile peaked at {peak} bytes, at least the {retained}-byte plan plus every \
         instance's {processors} processor bytes"
    );
}

/// The estimate a projection reports is the one the plan compiled from it reports.
///
/// Test value: red when the back half charges a row the projection did not, or charges one
/// differently: a host admits a session's graph row from the projection before it prepares a
/// processor, so a plan whose estimate exceeded it would be admitted past the budget it was checked
/// against.
#[test]
fn a_projections_estimate_is_its_compiled_plans() {
    let session = console();
    let registry = launch_native_effect_registry().expect("launch registry");
    let builtins = builtins(&session);
    let projection = GraphCompiler::project(request(projected(&session, registry), &builtins))
        .unwrap_or_else(|failure| panic!("the console projects: {:?}", failure.diagnostics));
    let projected = projection.resource_estimate().clone();
    let artifact = GraphCompiler::compile_projected(projection, builtins)
        .unwrap_or_else(|failure| panic!("the console compiles: {:?}", failure.diagnostics));
    assert_eq!(artifact.graph_resource_estimate(), &projected);
}
