//! Graph identity: node and edge construction, stable-ID helpers, and diagnostics.

use super::*;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct PreparedEffectSlot(usize);

impl PreparedEffectSlot {
    pub(crate) const fn index(self) -> usize {
        self.0
    }
}

pub(crate) struct PreparedEffectIndex<'a> {
    by_key: BTreeMap<(&'a str, RackId, &'a str), PreparedEffectSlot>,
}

impl<'a> PreparedEffectIndex<'a> {
    pub(crate) fn from_entries(entries: &'a [EffectPreparedEntry]) -> (Self, usize) {
        let mut by_key = BTreeMap::new();
        let mut duplicates = 0;
        for (index, entry) in entries.iter().enumerate() {
            if by_key
                .insert(
                    (
                        entry.track_id.as_str(),
                        rack_id(entry.rack),
                        entry.effect_id.as_str(),
                    ),
                    PreparedEffectSlot(index),
                )
                .is_some()
            {
                duplicates += 1;
            }
        }
        (Self { by_key }, duplicates)
    }

    pub(crate) fn get(
        &self,
        track_id: &str,
        rack: RackId,
        effect_id: &str,
    ) -> Option<PreparedEffectSlot> {
        self.by_key.get(&(track_id, rack, effect_id)).copied()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &(&'a str, RackId, &'a str)> {
        self.by_key.keys()
    }
}

pub(crate) fn prepared_effect_node(
    nodes: &[Option<EffectNodeId>],
    slot: PreparedEffectSlot,
) -> Option<&EffectNodeId> {
    nodes.get(slot.0).and_then(Option::as_ref)
}

pub(crate) fn ports_for(nodes: &[GraphNode], edges: &[GraphEdge]) -> Vec<GraphPortId> {
    let mut ports = Vec::new();
    for node in nodes {
        ports.push(port(node.id.clone(), GraphPortKind::MainInput));
        ports.push(port(node.id.clone(), GraphPortKind::MainOutput));
    }
    ports.extend(
        edges
            .iter()
            .filter(|edge| edge.destination.kind == GraphPortKind::SidechainInput)
            .map(|edge| edge.destination.clone()),
    );
    ports.sort();
    ports.dedup();
    ports
}
/// Every summing node with more than one main input: an output, and a submix strip's `Input`
/// stage (#1200 D3), which took the bare `Submix` node's place. A track's `Input` has no main
/// input -- its audio is its source binding, and a route can target only a submix or an output --
/// so admitting every `Input` stage records exactly the submix sums.
pub(crate) fn reduction_records(nodes: &[GraphNode], edges: &[GraphEdge]) -> Vec<ReductionRecord> {
    let mut contributions_by_node: BTreeMap<_, Vec<_>> = nodes
        .iter()
        .filter(|node| is_summing_node(&node.id))
        .map(|node| (&node.id, Vec::new()))
        .collect();
    for edge in edges {
        if edge.destination.kind == GraphPortKind::MainInput
            && let Some(contributions) = contributions_by_node.get_mut(&edge.destination.node)
        {
            contributions.push(&edge.id);
        }
    }
    nodes
        .iter()
        .filter_map(|node| {
            let mut contributions = contributions_by_node.remove(&node.id)?;
            contributions.sort();
            (contributions.len() > 1).then(|| ReductionRecord {
                node: node.id.clone(),
                contributions: contributions.into_iter().cloned().collect(),
            })
        })
        .collect()
}
/// Whether `node` is one the canonical text and the estimate count reductions on (#1200 D3).
pub(crate) fn is_summing_node(node: &GraphNodeId) -> bool {
    matches!(
        node,
        GraphNodeId::Submix { .. }
            | GraphNodeId::Output { .. }
            | GraphNodeId::TrackStage {
                stage: TrackStage::Input,
                ..
            }
    )
}
pub(crate) fn add_node(
    nodes: &mut Vec<GraphNode>,
    latencies: &mut BTreeMap<GraphNodeId, LatencySamples>,
    tails: &mut BTreeMap<GraphNodeId, TailSamples>,
    id: GraphNodeId,
    latency: LatencySamples,
    tail: TailSamples,
) {
    latencies.insert(id.clone(), latency);
    tails.insert(id.clone(), tail);
    nodes.push(GraphNode { id, latency, tail });
}
pub(crate) fn add_main_edge(
    edges: &mut Vec<GraphEdge>,
    source: GraphNodeId,
    destination: GraphNodeId,
    path: String,
) {
    edges.push(GraphEdge {
        id: GraphEdgeId::TrackMain {
            target: destination.clone(),
        },
        source: port(source, GraphPortKind::MainOutput),
        destination: port(destination, GraphPortKind::MainInput),
        path,
    });
}
pub(crate) fn add_route_source_edge(
    edges: &mut Vec<GraphEdge>,
    source: GraphNodeId,
    destination: GraphNodeId,
    id: &str,
) {
    edges.push(GraphEdge {
        id: GraphEdgeId::RouteSource { route_id: gid(id) },
        source: port(source, GraphPortKind::MainOutput),
        destination: port(destination, GraphPortKind::MainInput),
        path: format!("$.routes[id={id}].source"),
    });
}
pub(crate) fn add_route_destination_edge(
    edges: &mut Vec<GraphEdge>,
    source: GraphNodeId,
    destination: GraphNodeId,
    id: &str,
) {
    edges.push(GraphEdge {
        id: GraphEdgeId::RouteDestination { route_id: gid(id) },
        source: port(source, GraphPortKind::MainOutput),
        destination: port(destination, GraphPortKind::MainInput),
        path: format!("$.routes[id={id}].destination"),
    });
}
pub(crate) fn port(node: GraphNodeId, kind: GraphPortKind) -> GraphPortId {
    GraphPortId {
        node,
        kind,
        effect_port: None,
    }
}
pub(crate) fn gid(value: &str) -> StableGraphId {
    StableGraphId::parse(value).expect("accepted stable session ID")
}
pub(crate) fn track_node(track: &str, stage: TrackStage) -> GraphNodeId {
    GraphNodeId::TrackStage {
        track_id: gid(track),
        stage,
    }
}
pub(crate) fn route_source_node(source: &RouteSource) -> GraphNodeId {
    match source {
        RouteSource::Track { track_id, tap } => track_node(track_id.as_str(), stage(*tap)),
        // #1203 D3: a submix strip offers the seven taps a track does, at the same stages.
        RouteSource::Submix { submix_id, tap } => track_node(submix_id.as_str(), stage(*tap)),
    }
}
pub(crate) fn route_destination_node(destination: &RouteDestination) -> GraphNodeId {
    match destination {
        // #1200 D1: the submix strip's `Input` stage, which sums its route inputs.
        RouteDestination::SubmixInput { submix_id } => {
            track_node(submix_id.as_str(), TrackStage::Input)
        }
        RouteDestination::OutputInput { output_id } => GraphNodeId::Output {
            output_id: gid(output_id.as_str()),
        },
    }
}
/// A session tap's internal stage. Decision 12 renamed the taps and kept their positions, so the
/// internal `TrackStage` names (and the sealed graph text that spells them) are unchanged: the
/// insert send is the stage after the first internal rack (`console.pre_insert`), the insert
/// return the stage after the second (the track's inserts), and pre-fader the stage after the third
/// (`console.post_insert`).
pub(crate) fn stage(tap: SendTap) -> TrackStage {
    match tap {
        SendTap::Input => TrackStage::Input,
        SendTap::PostInput => TrackStage::PostInputBuiltins,
        SendTap::InsertSend => TrackStage::PostSimd1,
        SendTap::InsertReturn => TrackStage::PostDynamic,
        SendTap::PreFader => TrackStage::PostSimd2PreFader,
        SendTap::PostFader => TrackStage::PostFader,
        SendTap::PostPan => TrackStage::PostMatrix,
    }
}
pub(crate) fn stages() -> [TrackStage; 7] {
    [
        TrackStage::Input,
        TrackStage::PostInputBuiltins,
        TrackStage::PostSimd1,
        TrackStage::PostDynamic,
        TrackStage::PostSimd2PreFader,
        TrackStage::PostFader,
        TrackStage::PostMatrix,
    ]
}
pub(crate) fn rack_id(rack: EffectRack) -> RackId {
    match rack {
        EffectRack::Simd1 => RackId::Simd1,
        EffectRack::Dynamic => RackId::Dynamic,
        EffectRack::Simd2 => RackId::Simd2,
    }
}
/// A graph diagnostic and edge path for one lowered effect node. It names the internal rack
/// (`simd1` holds `console.pre_insert`, `dynamic` the inserts and `simd2` `console.post_insert`)
/// because these paths are part of the sealed `MISO-GRAPH-V1` canonical text, which decision 12's
/// class-A-by-lowering keeps byte-identical.
///
/// `strip` is the owning strip's `StripRef::path_prefix` (`$.tracks[id=<track>]` for a track).
pub(crate) fn effect_path(strip: &str, rack: RackId, effect: &str) -> String {
    format!(
        "{strip}.{}.effects[id={effect}]",
        match rack {
            RackId::Simd1 => "simd1",
            RackId::Dynamic => "dynamic",
            RackId::Simd2 => "simd2",
        }
    )
}
pub(crate) fn sidechain_matches(
    declaration: &SidechainDeclaration,
    entry: &EffectPreparedEntry,
) -> bool {
    match (declaration, entry.metadata.ports.sidechain) {
        (
            SidechainDeclaration::None,
            PreparedSidechainPort::None
            | PreparedSidechainPort::Unconnected {
                required: false, ..
            },
        ) => true,
        (SidechainDeclaration::Routed(value), PreparedSidechainPort::Connected { id, .. }) => {
            value.port_id.as_str() == id.as_str()
        }
        _ => false,
    }
}
/// Linear route gain from decibels, and the 2x2 matrix that carries it.
///
/// The conversion is `math::db_to_gain_f32` -- the workspace's single dB->linear
/// routine (master plan #83 D6/S5.1). The platform `f64::powf` this replaced resolved to the host
/// libm (glibc/musl natively, compiler-builtins' libm on wasm32), is not correctly rounded, and
/// differed in the last ulp between targets; the `as f32` narrowing rounded a second time. Those
/// bits reach both the render multiply and the semantic SHA-256 (#99 F4), so they were the one
/// place in this crate that could break the native/wasm bit-identity contract (D5).
///
/// `db_to_gain_f32(0.0)` is `exp2f(0.0) == 1.0` exactly, so a 0 dB route keeps `0x3f80_0000` and
/// every checked-in graph fixture is byte-identical. A session with a **non-zero** route gain gets
/// a one-time semantic-hash change: its `route-transform` canonical line now carries the
/// deterministic coefficient instead of the host's.
pub(crate) fn route_transform(gain_db: f32, matrix: &ChannelMatrix) -> Option<RouteTransform> {
    let gain = math::db_to_gain_f32(gain_db);
    (gain_db.is_finite()
        && gain.is_finite()
        && !gain.is_subnormal()
        && [matrix.ll, matrix.lr, matrix.rl, matrix.rr]
            .into_iter()
            .all(|v| v.is_finite() && !v.is_subnormal()))
    .then_some(RouteTransform {
        gain,
        ll: matrix.ll,
        lr: matrix.lr,
        rl: matrix.rl,
        rr: matrix.rr,
    })
}
/// Lower the prepared entries into the plan's effects, and -- separately -- the live-control
/// channels of whichever of them live controls drive (issue #140 A).
///
/// The two vectors are returned side by side rather than as one because
/// `core::mem::size_of::<RuntimeOp>()` is a reported byte: see
/// [`graph::GraphEffectControlBinding`].
pub(crate) fn into_effects(
    entries: Vec<EffectPreparedEntry>,
    ids: &[Option<EffectNodeId>],
) -> (
    Vec<GraphPreparedEffect>,
    Vec<graph::GraphEffectControlBinding>,
    Vec<graph::GraphEffectObservationBinding>,
) {
    let mut effects = Vec::with_capacity(entries.len());
    let mut controls = Vec::new();
    let mut observations = Vec::new();
    for (index, entry) in entries.into_iter().enumerate() {
        let node = ids[index]
            .as_ref()
            .expect("prepared effect node assigned")
            .clone();
        if let Some(control) = entry.control {
            controls.push(graph::GraphEffectControlBinding {
                node: node.clone(),
                control,
            });
        }
        if let Some(observation) = entry.observation {
            observations.push(graph::GraphEffectObservationBinding {
                node: node.clone(),
                observation,
            });
        }
        effects.push(GraphPreparedEffect {
            id: node,
            metadata: entry.metadata,
            processor: entry.processor,
            response_snapshot_declared: entry.factory.response_analysis().is_some(),
            native_id: entry.factory.descriptor().id.as_str(),
        });
    }
    (effects, controls, observations)
}
pub(crate) fn diag(code: &'static str, path: &str) -> GraphDiagnostic {
    GraphDiagnostic {
        code,
        path: path.to_owned(),
        cycle: Vec::new(),
        cycle_edge_paths: Vec::new(),
    }
}
