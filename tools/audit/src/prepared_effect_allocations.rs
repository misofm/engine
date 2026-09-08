//! Compile-only allocation records for issue #650.

use bench_support::alloc as bench_alloc;
use conformance::DualAccumulatorDelayFactory;
use effect_compiler::{EffectCompileCaps, EffectPreparedSession, prepare_native_session_effects};
use effect_contract::NativeEffectRegistry;
use graph::{GraphCompileCaps, GraphDiagnosticSet};
use graph_compiler::{Backend, GraphCompileRequest, GraphCompiler};
use session::{
    CompileCaps, EffectIdentity, EffectQuality, LinkMode, StableId, compile_session,
    parse_session_json,
};
use sha2::{Digest, Sha256};
use std::hint::black_box;

const SESSION: &str = include_str!("../../../fixtures/session/v1/canonical.json");
const ROUNDS: [u8; 2] = [1, 2];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Variant {
    Candidate,
    Counterfactual,
}

impl Variant {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "candidate" => Ok(Self::Candidate),
            "counterfactual" => Ok(Self::Counterfactual),
            other => Err(format!("unknown --variant value: {other}")),
        }
    }

    const fn text(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::Counterfactual => "counterfactual",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Corpus {
    Zero64,
    CrossedSmall,
    Banks64,
}

impl Corpus {
    const fn all() -> [Self; 3] {
        [Self::Zero64, Self::CrossedSmall, Self::Banks64]
    }

    const fn name(self) -> &'static str {
        match self {
            Self::Zero64 => "zero64",
            Self::CrossedSmall => "crossed-small",
            Self::Banks64 => "banks64",
        }
    }

    const fn tracks(self) -> usize {
        match self {
            Self::CrossedSmall => 4,
            Self::Zero64 | Self::Banks64 => 64,
        }
    }

    const fn with_effects(self) -> bool {
        !matches!(self, Self::Zero64)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Identity {
    graph: String,
    diagnostic: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Counters {
    allocations: u64,
    deallocations: u64,
    reallocations: u64,
    requested_bytes: u64,
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

fn session_caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

fn effect_caps() -> EffectCompileCaps {
    EffectCompileCaps {
        maximum_total_state_bytes: u64::MAX,
        maximum_scratch_bytes: u64::MAX,
        maximum_automation_spans_per_block: u32::MAX,
    }
}

fn fixture_session(corpus: Corpus) -> session::CompiledSession {
    let mut model = parse_session_json(SESSION).expect("issue-650 session fixture");
    let mut template = model.tracks[0].clone();
    template.id = StableId::parse("track-0").expect("track id");
    template.simd1.effects.clear();
    template.dynamic.effects.clear();
    template.simd2.effects.clear();
    if corpus.with_effects() {
        let effect = |id: &str| session::Effect {
            id: StableId::parse(id).expect("effect slot id"),
            identity: EffectIdentity::Native {
                effect_id: StableId::parse("conformance.delay").expect("effect id"),
            },
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            params: Vec::new(),
            sidechain: session::SidechainDeclaration::None,
        };
        match corpus {
            Corpus::CrossedSmall => {
                template.simd1.effects = vec![effect("slot1")];
                template.dynamic.effects = vec![effect("slot0")];
            }
            Corpus::Banks64 => {
                template.simd1.effects = vec![effect("slot0"), effect("slot1")];
            }
            Corpus::Zero64 => unreachable!(),
        }
    }
    model.tracks.clear();
    model.tracks.reserve(corpus.tracks());
    for index in 0..corpus.tracks() {
        let mut track = template.clone();
        track.id = StableId::parse(&format!("track-{index}")).expect("generated track id");
        if matches!(corpus, Corpus::CrossedSmall) && index % 2 == 1 {
            track.simd1.effects.reverse();
            track.dynamic.effects.reverse();
        }
        model.tracks.push(track);
    }
    model.routes[0].source = session::RouteSource::Track {
        track_id: StableId::parse("track-0").expect("route track id"),
        tap: session::SendTap::PostMatrix,
    };
    model.automation.clear();
    compile_session(&model, session_caps()).expect("issue-650 session compiles")
}

fn prepared(corpus: Corpus) -> EffectPreparedSession {
    let session = fixture_session(corpus);
    if !corpus.with_effects() {
        return EffectPreparedSession {
            session,
            entries: Vec::new(),
        };
    }
    let registry = NativeEffectRegistry::new([Box::new(DualAccumulatorDelayFactory::correct())
        as Box<dyn effect_contract::NativeEffectFactory>])
    .expect("issue-650 native registry");
    let mut effects = prepare_native_session_effects(&session, &registry, effect_caps())
        .expect("effects prepare");
    if matches!(corpus, Corpus::CrossedSmall) {
        effects.entries.reverse();
    }
    effects
}

fn diagnostic_hash(diagnostics: &GraphDiagnosticSet) -> String {
    let mut text = String::new();
    for diagnostic in diagnostics.diagnostics() {
        text.push_str(diagnostic.code);
        text.push('\n');
        text.push_str(&diagnostic.path);
        text.push('\n');
        text.push_str(&format!(
            "{:?}\n{:?}\n",
            diagnostic.cycle, diagnostic.cycle_edge_paths
        ));
    }
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn identity(corpus: Corpus) -> Identity {
    let valid = match GraphCompiler::compile(GraphCompileRequest {
        plan_id: 1,
        effects: prepared(corpus),
        caps: graph_caps(),
        dispatch: Backend::Scalar,
    }) {
        Ok(value) => value,
        Err(_) => panic!("issue-650 valid graph"),
    };
    let graph = GraphCompiler::sha256(&valid.graph, &valid.report);
    drop(valid);

    let mut caps = graph_caps();
    caps.maximum_nodes = 0;
    let invalid = match GraphCompiler::compile(GraphCompileRequest {
        plan_id: 2,
        effects: prepared(corpus),
        caps,
        dispatch: Backend::Scalar,
    }) {
        Ok(_) => panic!("issue-650 invalid twin unexpectedly compiled"),
        Err(value) => value,
    };
    let diagnostic = diagnostic_hash(&invalid.diagnostics);
    drop(invalid);
    Identity { graph, diagnostic }
}

fn measure(corpus: Corpus) -> Counters {
    bench_alloc::assert_installed();
    let control = Box::new([0_u8; 64]);
    black_box(&control);
    drop(control);
    let mark = bench_alloc::current_thread_counters();
    let artifact = match GraphCompiler::compile(GraphCompileRequest {
        plan_id: 7,
        effects: prepared(corpus),
        caps: graph_caps(),
        dispatch: Backend::Scalar,
    }) {
        Ok(value) => value,
        Err(_) => panic!("issue-650 measured graph"),
    };
    let delta = bench_alloc::current_thread_delta_since(mark);
    drop(artifact);
    Counters {
        allocations: delta.allocations,
        deallocations: delta.deallocations,
        reallocations: delta.reallocations,
        requested_bytes: delta.requested_bytes,
    }
}

fn parse_variant() -> Result<Variant, String> {
    let mut args = std::env::args().skip(1);
    if args.next().as_deref() != Some("--variant") {
        return Err(
            "usage: audit prepared-effect-allocations --variant candidate|counterfactual"
                .to_owned(),
        );
    }
    let variant = Variant::parse(
        args.next()
            .ok_or_else(|| "missing --variant value".to_owned())?
            .as_str(),
    )?;
    if args.next().is_some() {
        return Err("duplicate or unknown arguments".to_owned());
    }
    Ok(variant)
}

fn emit(variant: Variant, corpus: Corpus, round: u8, identity: &Identity, counters: Counters) {
    println!(
        "{{\"variant\":\"{}\",\"corpus\":\"{}\",\"round\":{},\"graph_sha256\":\"{}\",\"diagnostic_sha256\":\"{}\",\"allocations\":{},\"deallocations\":{},\"reallocations\":{},\"requested_bytes\":{}}}",
        variant.text(),
        corpus.name(),
        round,
        identity.graph,
        identity.diagnostic,
        counters.allocations,
        counters.deallocations,
        counters.reallocations,
        counters.requested_bytes,
    );
}

pub(crate) fn main() {
    let variant = match parse_variant() {
        Ok(value) => value,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    };
    bench_alloc::assert_installed();
    for corpus in Corpus::all() {
        let identity = identity(corpus);
        black_box(measure(corpus));
        for round in ROUNDS {
            emit(variant, corpus, round, &identity, measure(corpus));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_parser_is_strict() {
        assert_eq!(Variant::parse("candidate"), Ok(Variant::Candidate));
        assert_eq!(
            Variant::parse("counterfactual"),
            Ok(Variant::Counterfactual)
        );
        assert!(Variant::parse("unknown").is_err());
        assert!(Variant::parse("").is_err());
    }

    #[test]
    fn corpus_shapes_are_frozen() {
        assert_eq!(Corpus::Zero64.tracks(), 64);
        assert_eq!(Corpus::CrossedSmall.tracks(), 4);
        assert_eq!(Corpus::Banks64.tracks(), 64);
        assert!(!Corpus::Zero64.with_effects());
        assert!(Corpus::CrossedSmall.with_effects());
        assert!(Corpus::Banks64.with_effects());
    }
}
