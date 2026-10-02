//! Untimed compile-shape claims that #1026 moved here from the retired one-shot benchmarks.
//!
//! The rack (#038), builtins (#072/#431) and graph-compiler (#006) benchmark subjects were
//! deleted with their runners. Their unit tests prepared production plans without timing them, and
//! three of those claims had no other gate in this crate:
//!
//! * a mixed twelve-track session, whose tracks do not all carry the same rack depth, banks every
//!   builtin stage and forms one two-slot rack-chain cohort, leaves the tracks that carry only a
//!   subsequence of the chain in the unbound remainder, and leaves connected-sidechain tracks as
//!   scalar fallbacks (from `rack::zero_launch_preflight_prepares_each_exact_production_workload`);
//! * a representative 256-track console with 1,024 routes, 32 submixes, 64 effects and 32 routed
//!   sidechains compiles with builtins and reports its own shape (from
//!   `graph::canonical_benchmark_fixture_prepares_and_compiles`);
//! * a plan with meters at all seven taps binds seven consumers in tap order, and a full queue
//!   drops exactly one window (from
//!   `builtins::real_meter_tap_plans_use_the_compiled_seven_taps_and_preserve_full_queue_state`).
//!
//! And one claim of #1200's (gate 4): a bus sum stays in the sealed canonical text once a submix
//! lowers to a strip.
//!
//! And one of #1203's (gate 5): a loop closed through a bus tap is a `graph.cycle`.
//!
//! The mixed session is compiled at both SIMD widths explicitly, so its expectations do not
//! depend on the development host.

use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};

use builtins::{MeterConfig, MeterHandle, MeterTap};
use builtins_compiler::{BuiltinCompileCaps, MeterRequest, prepare_session_builtins};
use conformance::DualAccumulatorDelayFactory;
use effect_compiler::{
    CONSOLE_ELIGIBLE_EFFECTS, EffectCompileCaps, EffectPreparedSession,
    prepare_native_session_effects_with_console_eligibility,
};
use effect_contract::{NativeEffectFactory, NativeEffectRegistry};
use engine::realtime::{PlanarBufferMut, PreparedRenderPlan, RenderError, RenderIo, RenderTime};
use graph::{
    GraphBindingBlock, GraphCompileCaps, GraphEdgeId, GraphNodeBinding, GraphNodeId,
    GraphRuntimeBindings, GraphRuntimeProcessor, StableGraphId, TrackStage,
};
use graph_compiler::{
    Backend, GraphBuiltinsCompileRequest, GraphCompiler, PreparedGraphBuiltinsArtifact,
    PreparedGraphBuiltinsBound,
};
use rack::RackLocation;
use session::{
    ChannelMatrix, CompileCaps, CompiledSession, ConsoleEntry, ConsoleSlot, EffectIdentity,
    EffectParam, ParameterChannel, ParameterUnit, Route, RouteDestination, RouteSource, SendTap,
    SessionModel, Sidechain, SidechainDeclaration, StableId, Submix, compile_session,
    parse_session_json,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/canonical.json");
const QUANTUM: usize = 128;

fn stable(value: &str) -> StableId {
    StableId::parse(value).expect("generated stable ID")
}

fn compile_caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

fn builtin_caps() -> BuiltinCompileCaps {
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
    }
}

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

/// The conformance delay, the one native effect every fixture here uses.
fn prepared_effects(session: &CompiledSession) -> EffectPreparedSession {
    let registry = NativeEffectRegistry::new([
        Box::new(DualAccumulatorDelayFactory::correct()) as Box<dyn NativeEffectFactory>
    ])
    .expect("conformance registry");
    // The seven-tap fixture's console slots are this test double, admitted beside the launch
    // list for this registry only.
    let mut console_eligible = CONSOLE_ELIGIBLE_EFFECTS.to_vec();
    console_eligible.push("conformance.delay");
    prepare_native_session_effects_with_console_eligibility(
        session,
        &registry,
        EffectCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_scratch_bytes: u64::MAX,
            maximum_automation_spans_per_block: u32::MAX,
        },
        &console_eligible,
    )
    .expect("prepared effects")
}

fn compile(
    session: &CompiledSession,
    requests: &[MeterRequest],
    dispatch: Backend,
) -> PreparedGraphBuiltinsArtifact {
    let effects = prepared_effects(session);
    let builtins =
        prepare_session_builtins(&effects.session, requests, builtin_caps()).expect("builtins");
    GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch,
        plan_id: 1026,
        effects,
        builtins,
        caps: graph_caps(),
    })
    .unwrap_or_else(|failure| panic!("graph compile: {:?}", failure.diagnostics))
}

/// Every input stage emits a fixed nonzero block; every other external node is the identity.
struct ConstantSource;
impl GraphRuntimeProcessor for ConstantSource {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        block.left.fill(0.25);
        block.right.fill(-0.125);
        Ok(())
    }
}
struct Identity;
impl GraphRuntimeProcessor for Identity {
    fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        Ok(())
    }
}

fn bind(artifact: PreparedGraphBuiltinsArtifact) -> PreparedGraphBuiltinsBound {
    let envelope = artifact.envelope();
    let nodes = artifact
        .external_binding_nodes()
        .cloned()
        .map(|node| {
            let processor: Box<dyn GraphRuntimeProcessor> = match node {
                GraphNodeId::TrackStage {
                    stage: TrackStage::Input,
                    ..
                } => Box::new(ConstantSource),
                _ => Box::new(Identity),
            };
            GraphNodeBinding::new(node, processor)
        })
        .collect();
    artifact
        .into_bound(GraphRuntimeBindings {
            envelope,
            nodes,
            observers: Vec::new(),
        })
        .unwrap_or_else(|_| panic!("graph bind"))
}

fn render(plan: &mut PreparedRenderPlan, output: &mut [f32; QUANTUM * 2], first_sample: u64) {
    plan.render(
        RenderIo {
            output: PlanarBufferMut::try_new(output, 2, QUANTUM, QUANTUM).expect("output"),
        },
        RenderTime {
            absolute_sample: first_sample,
        },
    )
    .expect("render");
}

/// Twelve tracks: `rack00`..`rack09` carry a two-slot SIMD-1 chain `[delay-leading (bypassed),
/// delay-main]`, except `rack02` and `rack05`, which carry only `delay-main`; `fallback10` and
/// `fallback11` carry `delay-main` with a connected sidechain. The rack depths differ, so the
/// fader and matrix stages straddle two dependency levels.
fn mixed_twelve_track_session() -> CompiledSession {
    let mut model = parse_session_json(SESSION).expect("canonical session");
    let template = model.tracks[0].clone();
    let route = model.routes[0].clone();
    model.automation.clear();
    model.tracks = (0..12)
        .map(|index| {
            let mut track = template.clone();
            let track_id = if index < 10 {
                format!("rack{index:02}")
            } else {
                format!("fallback{index}")
            };
            track.id = stable(&track_id);
            let lane = index as f32;
            track.builtins.left.trim_db = -3.0 + lane * 0.25;
            track.builtins.left.hpf_hz = 40.0 + lane * 3.0;
            track.builtins.left.lpf_hz = 15_000.0 - lane * 100.0;
            track.builtins.right.trim_db = 2.0 - lane * 0.2;
            track.builtins.right.hpf_hz = 60.0 + lane * 2.0;
            track.builtins.right.lpf_hz = 14_000.0 - lane * 80.0;
            track.inserts.effects.clear();
            let mut effect = template.inserts.effects[0].clone();
            effect.id = stable("delay-main");
            effect.identity = EffectIdentity::Native {
                effect_id: stable("conformance.delay"),
            };
            effect.params = vec![EffectParam {
                parameter_id: 1,
                channel: ParameterChannel::Both,
                unit: ParameterUnit::Linear,
                value: 0.75 + index as f32 * 0.031_25,
            }];
            effect.bypass = false;
            effect.sidechain = if index < 10 {
                SidechainDeclaration::None
            } else {
                SidechainDeclaration::Routed(Sidechain {
                    source: RouteSource::Track {
                        track_id: track.id.clone(),
                        tap: SendTap::Input,
                    },
                    port_id: stable("sidechain-in"),
                })
            };
            // Per-track chains of different depths, and keyed ones, are inserts: a console slot
            // runs on every track and takes no sidechain (decision 12).
            track.inserts.effects = if matches!(index, 2 | 5) || index >= 10 {
                vec![effect]
            } else {
                let mut leading = effect.clone();
                leading.id = stable("delay-leading");
                leading.bypass = true;
                vec![leading, effect]
            };
            track
        })
        .collect();
    model.routes = model
        .tracks
        .iter()
        .enumerate()
        .map(|(index, track)| {
            let mut next = route.clone();
            next.id = stable(&format!("bank-route{index}"));
            next.source = RouteSource::Track {
                track_id: track.id.clone(),
                tap: SendTap::PostPan,
            };
            next
        })
        .collect();
    compile_session(&model, compile_caps()).expect("mixed session")
}

#[test]
fn mixed_rack_depths_bank_every_stage_and_leave_subsequences_and_sidechains_scalar() {
    const TRACKS: usize = 12;
    const BANKABLE_STAGES: usize = 3;
    const FULL_CHAIN_TRACKS: usize = 8;
    let session = mixed_twelve_track_session();
    for &dispatch in Backend::VECTOR {
        let lanes = dispatch.width();
        let artifact = compile(&session, &[], dispatch);

        // Post-input builtins, fader and matrix each bank all twelve tracks: full banks first,
        // then the padded remainder, whichever dependency levels the stage straddles.
        let mut expected_sizes = vec![lanes; TRACKS / lanes];
        if !TRACKS.is_multiple_of(lanes) {
            expected_sizes.push(TRACKS % lanes);
        }
        expected_sizes.sort_unstable();
        assert_eq!(
            artifact.prepared_builtin_bank_count(),
            BANKABLE_STAGES * expected_sizes.len(),
            "{dispatch:?}: builtin banks"
        );
        let banks: Vec<_> = artifact.prepared_builtin_banks().collect();
        assert!(
            banks
                .iter()
                .all(|bank| bank.backend == dispatch && bank.width.lanes() as usize == lanes),
            "{dispatch:?}: every builtin bank runs at the compile's width"
        );
        let mut by_stage: std::collections::BTreeMap<u8, Vec<usize>> = Default::default();
        for bank in &banks {
            let GraphNodeId::TrackStage { stage, .. } = &bank.members[0] else {
                panic!("a builtin bank member is a track stage");
            };
            by_stage
                .entry(*stage as u8)
                .or_default()
                .push(bank.members.len());
        }
        assert_eq!(by_stage.len(), BANKABLE_STAGES, "{dispatch:?}: stages");
        for (stage, mut sizes) in by_stage {
            sizes.sort_unstable();
            assert_eq!(sizes, expected_sizes, "{dispatch:?}: stage {stage} banks");
        }

        // One cohort for the two-slot chain. The eight tracks that carry all of it fill the full
        // groups and bind one bank per slot; the two subsequence tracks land in the unbound
        // remainder and render per node, beside the two connected-sidechain fallbacks.
        let cohorts = &artifact.report().rack_cohorts;
        assert_eq!(cohorts.dispatch, dispatch);
        let bound_groups: Vec<_> = cohorts.bound_groups_in(RackLocation::Dynamic).collect();
        assert_eq!(
            bound_groups.len(),
            FULL_CHAIN_TRACKS / lanes,
            "{dispatch:?}"
        );
        assert!(bound_groups.iter().all(|group| group.program.len() == 2));
        assert!(bound_groups.iter().all(|group| group.is_full()));
        assert!(
            bound_groups
                .iter()
                .all(|group| group.active_count() == lanes)
        );
        let bound_slots: Vec<_> = cohorts.bound_slots_in(RackLocation::Dynamic).collect();
        assert_eq!(
            bound_slots.len(),
            2 * FULL_CHAIN_TRACKS / lanes,
            "{dispatch:?}"
        );
        assert!(bound_slots.iter().all(|bound| bound.members.len() == lanes));
        assert!(
            bound_slots
                .iter()
                .flat_map(|bound| &bound.members)
                .all(|member| !matches!(member.track_id.as_str(), "rack02" | "rack05")),
            "{dispatch:?}: a subsequence track joined a bound bank"
        );
        assert_eq!(
            bound_slots.len() as u64,
            artifact.graph_resource_estimate().effect_bank_count,
            "{dispatch:?}: the report is the bound plan"
        );
        let mut scalar: Vec<_> = cohorts
            .scalar_in(RackLocation::Dynamic)
            .into_iter()
            .map(|member| member.track_id.as_str().to_owned())
            .collect();
        scalar.sort_unstable();
        assert_eq!(
            scalar,
            ["fallback10", "fallback11", "rack02", "rack05"],
            "{dispatch:?}: compatible scalar tail and connected-sidechain fallbacks"
        );

        // The plan binds and renders.
        let mut bound = bind(artifact);
        render(&mut bound.plan, &mut [0.0; QUANTUM * 2], 0);
    }
}

fn taps() -> [SendTap; 7] {
    [
        SendTap::Input,
        SendTap::PostInput,
        SendTap::InsertSend,
        SendTap::InsertReturn,
        SendTap::PreFader,
        SendTap::PostFader,
        SendTap::PostPan,
    ]
}

/// 256 tracks, every fourth carrying an insert delay and every eighth a routed sidechain;
/// 992 track routes into 32 submixes from all seven taps, and 32 submix routes to the output.
fn representative_console() -> SessionModel {
    let mut model = parse_session_json(SESSION).expect("seed session");
    let track_template = model.tracks.pop().expect("seed track");
    let route_template = model.routes.pop().expect("seed route");
    model.automation.clear();
    model.submixes = (0..32)
        .map(|index| Submix::unity(stable(&format!("submix-{index:02}")), &model.console))
        .collect();
    model.tracks = (0..256)
        .map(|index| {
            let mut track = track_template.clone();
            track.id = stable(&format!("track-{index:03}"));
            track.console.clear();
            track.inserts.effects.clear();
            if index % 4 == 1 {
                let mut effect = track_template.inserts.effects[0].clone();
                effect.id = stable("delay");
                effect.identity = EffectIdentity::Native {
                    effect_id: stable("conformance.delay"),
                };
                effect.params = [ParameterChannel::Left, ParameterChannel::Right]
                    .map(|channel| EffectParam {
                        parameter_id: 1,
                        channel,
                        unit: ParameterUnit::Linear,
                        value: 1.0,
                    })
                    .to_vec();
                effect.sidechain = if index % 8 == 1 {
                    SidechainDeclaration::Routed(Sidechain {
                        source: RouteSource::Track {
                            track_id: stable("track-000"),
                            tap: SendTap::Input,
                        },
                        port_id: stable("sidechain-in"),
                    })
                } else {
                    SidechainDeclaration::None
                };
                track.inserts.effects.push(effect);
            }
            track
        })
        .collect();
    model.routes.clear();
    for index in 0..992 {
        let mut route = route_template.clone();
        route.id = stable(&format!("track-route-{index:04}"));
        route.source = RouteSource::Track {
            track_id: stable(&format!("track-{:03}", index % 256)),
            tap: taps()[index % taps().len()],
        };
        route.destination = RouteDestination::SubmixInput {
            submix_id: stable(&format!("submix-{:02}", index % 32)),
        };
        model.routes.push(route);
    }
    for index in 0..32 {
        model.routes.push(Route {
            id: stable(&format!("submix-route-{index:02}")),
            source: RouteSource::Submix {
                submix_id: stable(&format!("submix-{index:02}")),
                tap: SendTap::PostPan,
            },
            destination: RouteDestination::OutputInput {
                output_id: model.outputs[0].id.clone(),
            },
            channel_matrix: ChannelMatrix {
                ll: 1.0,
                lr: 0.0,
                rl: 0.0,
                rr: 1.0,
            },
            gain_db: 0.0,
        });
    }
    model
}

#[test]
fn representative_console_compiles_with_builtins_and_reports_its_shape() {
    let model = representative_console();
    assert_eq!(model.tracks.len(), 256);
    assert_eq!(model.routes.len(), 1_024);
    assert_eq!(model.submixes.len(), 32);
    let effects: Vec<_> = model
        .tracks
        .iter()
        .flat_map(|track| &track.inserts.effects)
        .collect();
    assert!(model.console.slots().next().is_none());
    assert_eq!(effects.len(), 64);
    assert_eq!(
        effects
            .iter()
            .filter(|effect| matches!(effect.sidechain, SidechainDeclaration::Routed(_)))
            .count(),
        32
    );
    let session = compile_session(&model, compile_caps()).expect("representative session");
    // The production compile, at the host's dispatch: every shipped target has a SIMD width.
    let dispatch = Backend::current();
    assert!(dispatch.width() > 1);
    let artifact = compile(&session, &[], dispatch);
    let estimate = artifact.graph_resource_estimate();
    assert_eq!(estimate.routes, 1_024);
    assert_eq!(estimate.effects, 64);
    assert!(estimate.builtin_bank_count > 0);
    // Every strip's post-input, fader and matrix stages are builtin bank members: the 256
    // tracks' and, since a submix lowers to a strip (#1200), the 32 submixes'.
    assert_eq!(
        artifact.graph().builtin_bank_members().count(),
        3 * (256 + 32)
    );
    let evidence = GraphCompiler::evidence(artifact.graph(), artifact.report());
    assert!(!evidence.canonical_bytes.is_empty());
    assert!(!evidence.dot.is_empty());
}

/// #1200 gate 4: a bus that three routes feed keeps its sum in the canonical text and in
/// `GraphCompiler::reductions`, now recorded on the submix strip's `Input` stage: exactly one
/// reduction, whose three contributions are the three route-destination edges in route-ID order.
#[test]
fn a_three_input_bus_keeps_one_reduction_on_its_input_stage() {
    let mut model = parse_session_json(SESSION).expect("canonical session");
    model.automation.clear();
    model.tracks[0].inserts.effects.clear();
    model.submixes = vec![Submix::unity(stable("bus"), &model.console)];
    let template = model.routes[0].clone();
    let output = model.outputs[0].id.clone();
    let track = model.tracks[0].id.clone();
    let mut routes: Vec<Route> = [
        ("in-a", SendTap::Input),
        ("in-b", SendTap::PreFader),
        ("in-c", SendTap::PostPan),
    ]
    .into_iter()
    .map(|(id, tap)| Route {
        id: stable(id),
        source: RouteSource::Track {
            track_id: track.clone(),
            tap,
        },
        destination: RouteDestination::SubmixInput {
            submix_id: stable("bus"),
        },
        ..template.clone()
    })
    .collect();
    routes.push(Route {
        id: stable("out"),
        source: RouteSource::Submix {
            submix_id: stable("bus"),
            tap: SendTap::PostPan,
        },
        destination: RouteDestination::OutputInput { output_id: output },
        ..template
    });
    model.routes = routes;
    let session = compile_session(&model, compile_caps()).expect("bus session");
    let artifact = compile(&session, &[], Backend::current());

    let bus_input = GraphNodeId::TrackStage {
        track_id: StableGraphId::parse("bus").expect("graph id"),
        stage: TrackStage::Input,
    };
    let reductions = GraphCompiler::reductions(artifact.graph());
    assert_eq!(
        reductions.len(),
        1,
        "the output has one input: only the bus sums"
    );
    assert_eq!(reductions[0].node, bus_input);
    assert_eq!(
        reductions[0].contributions,
        ["in-a", "in-b", "in-c"]
            .map(|route| GraphEdgeId::RouteDestination {
                route_id: StableGraphId::parse(route).expect("graph id"),
            })
            .to_vec()
    );
    assert_eq!(artifact.report().semantic_estimate.reductions, 1);

    let evidence = GraphCompiler::evidence(artifact.graph(), artifact.report());
    let text = String::from_utf8(evidence.canonical_bytes).expect("canonical text is UTF-8");
    let rows: Vec<&str> = text
        .lines()
        .filter(|line| line.starts_with("reduction\t"))
        .collect();
    assert_eq!(
        rows.len(),
        3,
        "one reduction, one row per contribution: {rows:?}"
    );
    for (rank, row) in rows.iter().enumerate() {
        assert!(
            row.starts_with(&format!("reduction\ttrack:bus:input\t{rank}\t")),
            "row {rank}: {row}"
        );
    }
    assert!(
        !text.lines().any(|line| line.contains("submix:")),
        "a session submix no longer compiles to a graph `Submix` node"
    );
}

/// The canonical session with the conformance delay in each of track `vocal`'s three lowered
/// racks -- a `pre_insert` slot, an insert and a `post_insert` slot -- and a meter at each of its
/// seven taps: one-quantum windows, a one-slot queue, reset generation 7.
fn seven_tap_artifact() -> PreparedGraphBuiltinsArtifact {
    let mut model = parse_session_json(SESSION).expect("canonical session");
    model.automation.clear();
    let mut delay = model.tracks[0].inserts.effects[0].clone();
    delay.identity = EffectIdentity::Native {
        effect_id: stable("conformance.delay"),
    };
    delay.params.clear();
    let slot = |name: &str| ConsoleSlot {
        slot: stable(name),
        identity: delay.identity.clone(),
        quality: delay.quality,
        link_mode: delay.link_mode,
    };
    let entry = |name: &str| ConsoleEntry {
        slot: stable(name),
        bypass: delay.bypass,
        params: Vec::new(),
    };
    model.console.pre_insert = vec![slot("simd1-delay")];
    model.console.post_insert = vec![slot("simd2-delay")];
    model.tracks[0].console = vec![entry("simd1-delay"), entry("simd2-delay")];
    delay.id = stable("dynamic-delay");
    model.tracks[0].inserts.effects = vec![delay];
    let session = compile_session(&model, compile_caps()).expect("canonical session");
    assert_eq!(session.quantum().0 as usize, QUANTUM);
    let config = MeterConfig {
        period_frames: NonZeroU32::new(QUANTUM as u32).expect("constant"),
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: NonZeroUsize::new(1).expect("constant"),
        reset_generation: 7,
    };
    let requests: Vec<_> = TAPS
        .into_iter()
        .enumerate()
        .map(|(index, tap)| MeterRequest {
            handle: MeterHandle(NonZeroU64::new(index as u64 + 1).expect("one-based handle")),
            track_id: model.tracks[0].id.as_str().to_owned(),
            tap,
            config,
        })
        .collect();
    compile(&session, &requests, Backend::current())
}

const TAPS: [MeterTap; 7] = [
    MeterTap::Input,
    MeterTap::PostInputBuiltins,
    MeterTap::PostSimd1,
    MeterTap::PostDynamic,
    MeterTap::PostSimd2PreFader,
    MeterTap::PostFader,
    MeterTap::PostMatrix,
];

#[test]
fn seven_meter_taps_bind_in_tap_order_and_a_full_queue_drops_one_window() {
    let (success, full) = (seven_tap_artifact(), seven_tap_artifact());
    assert_eq!(success.report(), full.report());
    let (mut success, mut full) = (bind(success), bind(full));
    for bound in [&success, &full] {
        assert_eq!(
            bound
                .meter_consumers
                .iter()
                .map(|consumer| consumer.tap)
                .collect::<Vec<_>>(),
            TAPS
        );
    }
    let mut output = [0.0; QUANTUM * 2];

    // Two windows into one-slot queues: the second is dropped, the first stays queued.
    render(&mut full.plan, &mut output, 0);
    render(&mut full.plan, &mut output, QUANTUM as u64);
    for consumer in &mut full.meter_consumers {
        let window = consumer.consumer.try_pop().expect("the first window");
        assert_eq!(window.end_sample, QUANTUM as u64);
        assert_eq!(window.reset_generation, 7);
        assert_eq!(window.cumulative_dropped_snapshots, 0);
    }
    render(&mut full.plan, &mut output, 2 * QUANTUM as u64);
    for consumer in &mut full.meter_consumers {
        let window = consumer
            .consumer
            .try_pop()
            .expect("the window after the drop");
        assert_eq!(window.cumulative_dropped_snapshots, 1, "{:?}", consumer.tap);
    }

    // A queue that is drained in time drops nothing.
    render(&mut success.plan, &mut output, QUANTUM as u64);
    for consumer in &mut success.meter_consumers {
        let window = consumer.consumer.try_pop().expect("one window");
        assert_eq!(window.end_sample, 2 * QUANTUM as u64);
        assert_eq!(window.reset_generation, 7);
        assert_eq!(window.cumulative_dropped_snapshots, 0);
    }
}

/// #1203 gate 5: bus `a`'s `pre_fader` feeds bus `b`, and `b`'s `post_pan` feeds `a`. No route
/// leaves `a`'s end, yet the two routes close a loop through `a`'s tap: the compile refuses with
/// one `graph.cycle` whose witness names both routes and runs through `a`'s pre-fader stage, not
/// through `a`'s end.
#[test]
fn a_loop_through_a_bus_tap_is_a_graph_cycle() {
    let mut model = parse_session_json(SESSION).expect("canonical session");
    model.automation.clear();
    model.tracks[0].inserts.effects.clear();
    model.submixes = vec![
        Submix::unity(stable("a"), &model.console),
        Submix::unity(stable("b"), &model.console),
    ];
    let template = model.routes[0].clone();
    let bus_route = |id: &str, from: &str, tap: SendTap, to: &str| Route {
        id: stable(id),
        source: RouteSource::Submix {
            submix_id: stable(from),
            tap,
        },
        destination: RouteDestination::SubmixInput {
            submix_id: stable(to),
        },
        ..template.clone()
    };
    model
        .routes
        .push(bus_route("ab", "a", SendTap::PreFader, "b"));
    model
        .routes
        .push(bus_route("ba", "b", SendTap::PostPan, "a"));
    let session =
        compile_session(&model, compile_caps()).expect("the session layer has no cycle check");
    let effects = prepared_effects(&session);
    let builtins =
        prepare_session_builtins(&effects.session, &[], builtin_caps()).expect("builtins");
    let failure = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch: Backend::current(),
        plan_id: 1203,
        effects,
        builtins,
        caps: graph_caps(),
    })
    .err()
    .expect("a loop through a tap refuses");
    let diagnostics = failure.diagnostics.diagnostics();
    assert_eq!(diagnostics.len(), 1, "{diagnostics:?}");
    let cycle = &diagnostics[0];
    assert_eq!(cycle.code, "graph.cycle");
    // The witness's own path is its first edge, a strip edge; the routes are named in its edges.
    let routes: Vec<&str> = cycle
        .cycle_edge_paths
        .iter()
        .map(String::as_str)
        .filter(|path| path.starts_with("$.routes"))
        .collect();
    assert_eq!(
        routes,
        [
            "$.routes[id=ab].source",
            "$.routes[id=ab].destination",
            "$.routes[id=ba].source",
            "$.routes[id=ba].destination",
        ],
        "the witness names both routes: {:?}",
        cycle.cycle_edge_paths
    );
    let stage = |bus: &str, stage| GraphNodeId::TrackStage {
        track_id: StableGraphId::parse(bus).expect("graph id"),
        stage,
    };
    assert!(
        cycle
            .cycle
            .contains(&stage("a", TrackStage::PostSimd2PreFader))
    );
    assert!(!cycle.cycle.contains(&stage("a", TrackStage::PostMatrix)));
}
