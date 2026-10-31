//! Issue #2199: a graph projected before any effect processor exists, then compiled.
//!
//! `GraphCompiler::project` plans the graph and its banks from the projected effects alone, and
//! `GraphCompiler::compile_projected` binds the planned banks and prepares a processor only for an
//! instance the plan leaves per node (the #2194 residual (c)). The console fixture banks its console
//! EQ and its insert compressor on every track; one delay insert renders per node.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use bench_support::alloc::{
    assert_installed, current_thread_counters, current_thread_delta_since, current_thread_peak,
    current_thread_peak_reset,
};
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
const NINE_TRACK: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

/// Every launch effect that banks as an insert, beside the console EQ (issue #2223).
const BANKING_INSERTS: [&str; 8] = [
    "miso.compressor",
    "miso.gate-expander",
    "miso.de-esser",
    "miso.dynamic-eq",
    "miso.multiband-compressor",
    "miso.soft-clip",
    "miso.transient-shaper",
    "miso.true-peak-limiter",
];

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

/// A dual-mono native insert of `effect_id` at its defaults.
fn insert(id: &str, effect_id: &str) -> Effect {
    Effect {
        id: StableId::parse(id).expect("an effect ID"),
        identity: EffectIdentity::Native {
            effect_id: StableId::parse(effect_id).expect("a native ID"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        params: Vec::new(),
        sidechain: SidechainDeclaration::None,
    }
}

/// The console fixture, with a delay inserted after the first track's compressor: the delay never
/// banks, so it renders per node beside the banked EQs and compressors.
fn console() -> CompiledSession {
    let mut model = parse_session_json(CONSOLE).expect("the console fixture");
    model.tracks[0]
        .inserts
        .effects
        .push(insert("delay", "miso.delay"));
    compile(&model)
}

/// Eleven tracks of the nine-track EQ console, each with one insert of every other launch effect
/// that banks, and a delay on the first (issue #2223). Eleven is `8 + 3` and `4 + 4 + 3`, so every
/// banking effect binds a padded bank at both widths; the delay never banks and renders per node.
fn every_banking_effect() -> CompiledSession {
    use session::{RouteSource, SendTap};
    let mut model = parse_session_json(NINE_TRACK).expect("the nine-track fixture");
    let mut track = model.tracks[1].clone();
    track.inserts.effects = BANKING_INSERTS
        .iter()
        .enumerate()
        .map(|(index, effect_id)| insert(&format!("insert-{index}"), effect_id))
        .collect();
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    for index in 0..11 {
        let track_id = StableId::parse(&format!("track-{index:02}")).expect("a track ID");
        let mut next = track.clone();
        next.id = track_id.clone();
        if index == 0 {
            next.inserts.effects.push(insert("delay", "miso.delay"));
        }
        model.tracks.push(next);
        let mut next_route = route.clone();
        next_route.id = StableId::parse(&format!("route-{index:02}")).expect("a route ID");
        next_route.source = RouteSource::Track {
            track_id,
            tap: SendTap::PostPan,
        };
        model.routes.push(next_route);
    }
    compile(&model)
}

fn compile(model: &session::SessionModel) -> CompiledSession {
    compile_session(
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
    .expect("the compiled session")
}

/// What one armed bind or prepare left live: the effect, the bytes the call left allocated on this
/// thread (`requested_bytes - released_bytes`, issue #1100), and for a bind whether it was padded.
#[derive(Clone, Copy, Debug)]
struct Allocated {
    effect: &'static str,
    live_bytes: u64,
    bank: Option<Bank>,
}

#[derive(Clone, Copy, Debug)]
struct Bank {
    padded: bool,
    /// `size_of_val` of the bound processor plus its `retained_bytes()`: what the bank row
    /// charges for it.
    charged_bytes: u64,
}

/// What every factory of a measuring registry saw.
#[derive(Default)]
struct Log {
    prepared: AtomicU64,
    armed: AtomicBool,
    allocated: Mutex<Vec<Allocated>>,
}

impl Log {
    /// Runs `call` and, when armed, records what it left live.
    fn measure<T>(
        &self,
        effect: &'static str,
        call: impl FnOnce() -> T,
        bank: impl FnOnce(&T) -> Option<Bank>,
    ) -> T {
        if !self.armed.load(Ordering::SeqCst) {
            return call();
        }
        let mark = current_thread_counters();
        let value = call();
        let delta = current_thread_delta_since(mark);
        let live_bytes = delta.requested_bytes - delta.released_bytes;
        let bank = bank(&value);
        self.allocated.lock().expect("the log").push(Allocated {
            effect,
            live_bytes,
            bank,
        });
        value
    }
}

/// A launch factory that counts the processors it prepares and, once armed, measures what each
/// prepare and bind leaves allocated.
struct Measuring {
    delegate: Arc<dyn NativeEffectFactory>,
    log: Arc<Log>,
}

impl NativeEffectFactory for Measuring {
    fn descriptor(&self) -> &'static EffectDescriptor {
        self.delegate.descriptor()
    }
    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<PreparedEffect, EffectPrepareError> {
        self.log.prepared.fetch_add(1, Ordering::SeqCst);
        let effect = self.descriptor().id.as_str();
        self.log
            .measure(effect, || self.delegate.prepare(request), |_| None)
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
        let effect = self.descriptor().id.as_str();
        let padded = request.active_mask.contains(&false);
        self.log.measure(
            effect,
            || self.delegate.bind_homogeneous_bank(request),
            |bound| {
                let processor = &bound.as_ref().ok()?.as_ref()?.processor;
                Some(Bank {
                    padded,
                    charged_bytes: size_of_val(&**processor) as u64 + processor.retained_bytes(),
                })
            },
        )
    }
}

/// The launch registry, every factory logging into one log.
fn measuring_registry() -> (NativeEffectRegistry, Arc<Log>) {
    let launch = launch_native_effect_registry().expect("launch registry");
    let log = Arc::new(Log::default());
    let factories: Vec<Box<dyn NativeEffectFactory>> = launch
        .descriptors()
        .map(|descriptor| {
            Box::new(Measuring {
                delegate: launch
                    .get_shared_ascii(descriptor.id.as_str())
                    .expect("a registered factory"),
                log: Arc::clone(&log),
            }) as Box<dyn NativeEffectFactory>
        })
        .collect();
    let registry = NativeEffectRegistry::new(factories).expect("the measuring registry");
    (registry, log)
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

/// The processor bytes a projection charges the instances it leaves per node
/// (`EffectProjectedEntry::processor_bytes`, issue #2219), and those entries' declared state and
/// scratch, which every entry's effect row charges besides.
fn per_node_charge(projection: &graph_compiler::GraphProjection) -> (u64, u64) {
    projection
        .effects()
        .entries
        .iter()
        .zip(projection.per_node_entries())
        .filter(|(_, per_node)| *per_node)
        .fold((0, 0), |(processor, declared), (entry, _)| {
            (
                processor + entry.processor_bytes().expect("a projected entry's bytes"),
                declared
                    + entry.metadata.state_sizes.total().expect("declared state")
                    + entry.metadata.scratch_bytes,
            )
        })
}

/// A projected compile prepares a processor for every instance its plan leaves per node, once,
/// and for no banked member, so its peak stays under the plan's retained estimate, its bank rows
/// included (issue #2223), plus the processors of the instances the projection leaves per node;
/// and the projection names those instances before any processor exists
/// (`GraphProjection::per_node_entries`, issue #2219).
///
/// Test value: red when a banked member gets a per-instance processor, as before #2199, when the
/// effect compiler prepared every instance and the graph compiler held them all while it bound the
/// banks: the prepare count is then every instance's. Red too when the projection's per-node
/// entries are not the ones the compile prepares (a banked member counted per node): a host
/// charges processor bytes on them before it prepares. The peak is bounded by exactly what a host
/// charges the compile -- the plan, its bank rows included, and the per-node processors -- so it is
/// red when the compile's peak passes that charge.
#[test]
fn a_projected_compile_prepares_no_banked_members_processor() {
    assert_installed();
    let session = console();
    let (registry, log) = measuring_registry();
    let builtins = builtins(&session);
    let effects = projected(&session, &registry);
    let instances = effects.entries.len();
    assert_eq!(
        log.prepared.load(Ordering::SeqCst),
        0,
        "a projection prepares nothing"
    );
    let projection = GraphCompiler::project(request(effects, &builtins))
        .unwrap_or_else(|failure| panic!("the console projects: {:?}", failure.diagnostics));
    let retained = projection.resource_estimate().session_plus_plan_bytes;
    let projected_per_node = projection
        .per_node_entries()
        .filter(|per_node| *per_node)
        .count();
    let (per_node_processors, _) = per_node_charge(&projection);
    current_thread_peak_reset();
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
        log.prepared.load(Ordering::SeqCst),
        per_node as u64,
        "one processor per per-node instance, none for a banked member"
    );
    assert_eq!(
        projected_per_node, per_node,
        "the projection's per-node entries are the compile's"
    );
    assert!(
        peak < retained + per_node_processors,
        "the compile peaked at {peak} bytes, at least the {retained}-byte plan plus the \\
         {per_node_processors} processor bytes of its per-node instances"
    );
}

/// Every bank a projected compile binds retains at most what the bank row charges, and every
/// per-node processor at most its entry's charge (issue #2223): the allocator's live bytes after
/// each bind and each prepare, over every launch effect that banks, each binding padded. It runs
/// at the build's bank width: eight lanes on x86-64-v3 and four on the AArch64 legs, the width the
/// browser runs; a factory binds no other width (`console.slot.foreign_width`).
///
/// Test value: red when a bank's heap is left out of the bank row -- the row charging the box
/// alone, or an effect's `retained_bytes()` under-reporting a heap its bank holds -- since the
/// allocator sees the true-peak limiter's rings, the EQ's planes and the soft clipper's tables
/// that such a row omits; and red when a banking effect stops binding padded here, so the check
/// no longer covers it.
#[test]
fn a_banks_bytes_are_within_its_row() {
    assert_installed();
    let session = every_banking_effect();
    let (registry, log) = measuring_registry();
    // One whole compile first, so a static an effect allocates on first use is warm.
    let warm = GraphCompiler::project(request(projected(&session, &registry), &builtins(&session)))
        .unwrap_or_else(|failure| panic!("the session projects: {:?}", failure.diagnostics));
    drop(GraphCompiler::compile_projected(warm, builtins(&session)));
    let builtins = builtins(&session);
    let projection = GraphCompiler::project(request(projected(&session, &registry), &builtins))
        .unwrap_or_else(|failure| panic!("the session projects: {:?}", failure.diagnostics));
    let row = projection.resource_estimate().effect_bank_processor_bytes;
    let (per_node_processors, per_node_declared) = per_node_charge(&projection);
    log.armed.store(true, Ordering::SeqCst);
    let artifact = GraphCompiler::compile_projected(projection, builtins);
    log.armed.store(false, Ordering::SeqCst);
    artifact.unwrap_or_else(|failure| panic!("the session compiles: {:?}", failure.diagnostics));
    // Per effect: the bytes its banks left live, what they charge, and whether one was padded.
    let mut banks: BTreeMap<&str, (u64, u64, bool)> = BTreeMap::new();
    let mut per_node_live = 0;
    for record in log.allocated.lock().expect("the log").iter() {
        match record.bank {
            Some(bank) => {
                let entry = banks.entry(record.effect).or_default();
                entry.0 += record.live_bytes;
                entry.1 += bank.charged_bytes;
                entry.2 |= bank.padded;
            }
            None => per_node_live += record.live_bytes,
        }
    }
    let banks_live: u64 = banks.values().map(|bank| bank.0).sum();
    assert!(
        banks_live <= row,
        "the banks left {banks_live} bytes live, more than the {row}-byte bank row: {banks:?}"
    );
    for effect in BANKING_INSERTS
        .iter()
        .copied()
        .chain(["miso.parametric-eq"])
    {
        let Some(&(live, charged, padded)) = banks.get(effect) else {
            panic!("{effect} binds no bank: {banks:?}");
        };
        assert!(padded, "{effect} binds no padded bank");
        assert!(
            live <= charged,
            "{effect}'s banks left {live} bytes live, more than the {charged} bytes charged"
        );
    }
    assert!(
        per_node_live > 0 && per_node_live <= per_node_processors + per_node_declared,
        "the per-node processors left {per_node_live} bytes live, against {per_node_processors} \
         processor and {per_node_declared} declared bytes charged"
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
