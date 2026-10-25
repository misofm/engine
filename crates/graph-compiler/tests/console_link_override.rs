//! Issue #1369 gate 5: a console entry's link-mode override renders as an insert at that mode.
//!
//! Eight tracks of the standing console (`eq -> comp` before the inserts, the limiter after them).
//! In session A, track X's `comp` entry overrides the slot's `dual_mono` with `maximum`. In
//! session B, X's `comp` entry stays on `slot` and is bypassed, and X carries a compressor insert
//! at `maximum` with the same params. The compressor has no latency, so the bypassed slot is the
//! identity and X's signal after the limiter must be bit-identical in A and B; every other track
//! must be bit-identical too, and in A the `comp` slot still binds its one bank over all eight
//! tracks (#1368: the bank holds the link mode per lane).

use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry, prepare_native_session_effects,
};
use effect_contract::BankWidth;
use engine::realtime::{PlanarBufferMut, RenderError, RenderIo, RenderTime};
use graph::{
    GraphBindingBlock, GraphCompileCaps, GraphNodeBinding, GraphNodeId, GraphNodeObserverBinding,
    GraphObservationBlock, GraphRuntimeBindings, GraphRuntimeObserver, GraphRuntimeProcessor,
    StableGraphId, TrackStage,
};
use graph_compiler::{Backend, GraphBuiltinsCompileRequest, GraphCompiler};
use session::{
    CompileCaps, ConsoleLinkMode, Effect, EffectIdentity, LinkMode, SessionModel,
    SidechainDeclaration, StableId, compile_session, parse_session_json,
};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

const INTENDED: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track-intended.json");
const TRACKS: usize = 8;
const BLOCKS: u64 = 12;
/// The loudest of the eight sources (about -4.6 dBFS), so its compressor works on it.
const X: &str = "ch07";

struct Source(f32);
impl GraphRuntimeProcessor for Source {
    fn retained_bytes(&self) -> u64 {
        0
    }
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        for (index, (left, right)) in block
            .left
            .iter_mut()
            .zip(block.right.iter_mut())
            .enumerate()
        {
            // Unequal lanes, so a linked detector differs from two independent ones.
            let t = (block.first_sample + index as u64) as f32;
            *left = self.0 * math::sinf(t * 0.013);
            *right = self.0 * 0.3 * math::cosf(t * 0.021);
        }
        Ok(())
    }
}

struct Identity;
impl GraphRuntimeProcessor for Identity {
    fn retained_bytes(&self) -> u64 {
        0
    }
    fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        Ok(())
    }
}

struct Capture(Arc<Mutex<Vec<u32>>>);
impl GraphRuntimeObserver for Capture {
    fn retained_bytes(&self) -> u64 {
        effect_contract::retained::arc(&self.0)
            + self
                .0
                .lock()
                .map_or(0, |v| effect_contract::retained::vec(&v))
    }
    fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
        let mut sink = self.0.lock().expect("capture");
        sink.extend(block.left.iter().map(|sample| sample.to_bits()));
        sink.extend(block.right.iter().map(|sample| sample.to_bits()));
        Ok(())
    }
}

/// The bound banks as their members' `(track, effect)` pairs, and each track's words after the
/// limiter.
struct Rendered {
    banks: Vec<Vec<(String, String)>>,
    tracks: BTreeMap<String, Vec<u32>>,
}

fn base() -> SessionModel {
    let mut model = parse_session_json(INTENDED).expect("console fixture");
    model.tracks.truncate(TRACKS);
    model.routes.truncate(TRACKS);
    model
}

fn render(model: &SessionModel) -> Rendered {
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
    .expect("compiled session");
    let effects = prepare_native_session_effects(
        &session,
        launch_native_effect_registry().expect("launch registry"),
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 30,
            maximum_scratch_bytes: 1 << 28,
            maximum_automation_spans_per_block: 32,
        },
    )
    .expect("prepared effects");
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
    .expect("builtins");
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        input_floors: Default::default(),
        plan_id: 1369,
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
        dispatch: Backend::current(),
        gain_liveness: graph_compiler::GainLiveness::FIXED,
    })
    .unwrap_or_else(|failure| panic!("graph: {:?}", failure.diagnostics));
    let banks = artifact
        .graph()
        .effect_bank_members()
        .map(|members| {
            members
                .iter()
                .map(|member| {
                    (
                        member.track_id.as_str().to_owned(),
                        member.effect_id.as_str().to_owned(),
                    )
                })
                .collect()
        })
        .collect();
    let envelope = artifact.envelope();
    let frames = envelope.quantum.0 as usize;
    let nodes = artifact
        .external_binding_nodes()
        .map(|node| {
            let processor: Box<dyn GraphRuntimeProcessor> = match node {
                GraphNodeId::TrackStage {
                    track_id,
                    stage: TrackStage::Input,
                } => {
                    let index: u32 = track_id.as_str()[2..].parse().expect("chNN");
                    Box::new(Source(0.1 + index as f32 * 0.07))
                }
                _ => Box::new(Identity),
            };
            GraphNodeBinding::new(node.clone(), processor)
        })
        .collect();
    let mut sinks = BTreeMap::new();
    let mut observers = Vec::new();
    for track in &model.tracks {
        let sink = Arc::new(Mutex::new(Vec::new()));
        observers.push(GraphNodeObserverBinding::new(
            GraphNodeId::TrackStage {
                track_id: StableGraphId::parse(track.id.as_str()).expect("track id"),
                stage: TrackStage::PostSimd2PreFader,
            },
            observers.len() as u64 + 1,
            Box::new(Capture(Arc::clone(&sink))),
        ));
        sinks.insert(track.id.as_str().to_owned(), sink);
    }
    let mut plan = artifact
        .into_bound(GraphRuntimeBindings {
            envelope,
            nodes,
            observers,
        })
        .unwrap_or_else(|failure| panic!("bind: {}", failure.code))
        .plan;
    let mut pcm = vec![0.0_f32; frames * 2];
    for block in 0..BLOCKS {
        plan.render(
            RenderIo {
                output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames).expect("output"),
            },
            RenderTime {
                absolute_sample: block * frames as u64,
            },
        )
        .expect("render");
    }
    drop(plan);
    let tracks = sinks
        .into_iter()
        .map(|(track, sink)| {
            let words = Arc::try_unwrap(sink)
                .expect("the plan is gone")
                .into_inner()
                .expect("capture");
            (track, words)
        })
        .collect();
    Rendered { banks, tracks }
}

/// Value: a lowering that ignores the override renders X's slot at `dual_mono` (X differs from
/// B), and one that applies it to every strip renders every other track at `maximum` (they differ
/// from B); a bank split by the override leaves X out of the `comp` slot's bank.
#[test]
fn a_console_link_override_renders_as_an_insert_at_that_mode() {
    let lanes = BankWidth::for_backend(Backend::current())
        .expect("the delivery host has a bank width")
        .lanes() as usize;
    let x = |model: &mut SessionModel| -> usize {
        model
            .tracks
            .iter()
            .position(|track| track.id.as_str() == X)
            .expect("track X")
    };
    let comp = |model: &SessionModel, track: usize| {
        model.tracks[track]
            .console
            .iter()
            .position(|entry| entry.slot.as_str() == "comp")
            .expect("comp entry")
    };
    assert_eq!(base().console.pre_insert[1].link_mode, LinkMode::DualMono);

    let mut a = base();
    let index = x(&mut a);
    let slot = comp(&a, index);
    a.tracks[index].console[slot].link_mode = ConsoleLinkMode::Maximum;

    let mut b = base();
    let entry = &mut b.tracks[index].console[slot];
    entry.bypass = true;
    let params = entry.params.clone();
    let declared = b.console.pre_insert[1].clone();
    b.tracks[index].inserts.effects.push(Effect {
        id: StableId::parse("comp-max").expect("id"),
        identity: declared.identity.clone(),
        quality: declared.quality,
        bypass: false,
        link_mode: LinkMode::Maximum,
        params,
        sidechain: SidechainDeclaration::None,
    });
    assert!(matches!(
        &declared.identity,
        EffectIdentity::Native { effect_id } if effect_id.as_str() == "miso.compressor"
    ));

    let (rendered_a, rendered_b, rendered_base) = (render(&a), render(&b), render(&base()));
    assert_eq!(rendered_a.tracks.len(), TRACKS);
    for (track, words) in &rendered_a.tracks {
        assert!(
            words == &rendered_b.tracks[track],
            "{track}: the override renders as the insert at its mode"
        );
    }
    // Not vacuous: the override moves X's bits and nothing else's.
    for (track, words) in &rendered_base.tracks {
        assert_eq!(
            words != &rendered_a.tracks[track],
            track == X,
            "{track}: only X's detector changes"
        );
    }
    let comp_banks = rendered_a
        .banks
        .iter()
        .filter(|members| members.iter().any(|(_, effect)| effect == "comp"))
        .collect::<Vec<_>>();
    assert_eq!(comp_banks.len(), TRACKS.div_ceil(lanes), "{comp_banks:?}");
    let mut members = comp_banks
        .iter()
        .flat_map(|members| members.iter().map(|(track, _)| track.clone()))
        .collect::<Vec<_>>();
    members.sort();
    assert_eq!(
        members,
        a.tracks
            .iter()
            .map(|track| track.id.as_str().to_owned())
            .collect::<Vec<_>>(),
        "the comp slot binds its bank over all eight tracks"
    );
}
