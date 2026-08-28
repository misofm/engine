//! Graph identity: node and edge construction, stable-ID helpers, and diagnostics.

use super::*;

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
pub(crate) fn reduction_records(nodes: &[GraphNode], edges: &[GraphEdge]) -> Vec<ReductionRecord> {
    let mut contributions_by_node: BTreeMap<_, Vec<_>> = nodes
        .iter()
        .filter(|node| {
            matches!(
                node.id,
                GraphNodeId::Submix { .. } | GraphNodeId::Output { .. }
            )
        })
        .map(|node| (node.id.clone(), Vec::new()))
        .collect();
    for edge in edges {
        if edge.destination.kind == GraphPortKind::MainInput
            && let Some(contributions) = contributions_by_node.get_mut(&edge.destination.node)
        {
            contributions.push(edge.id.clone());
        }
    }
    nodes
        .iter()
        .filter_map(|node| {
            let mut contributions = contributions_by_node.remove(&node.id)?;
            contributions.sort();
            (contributions.len() > 1).then(|| ReductionRecord {
                node: node.id.clone(),
                contributions,
            })
        })
        .collect()
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
pub(crate) fn route_source_node(source: &RouteSource) -> Option<GraphNodeId> {
    match source {
        RouteSource::Track { track_id, tap } => Some(track_node(track_id.as_str(), stage(*tap))),
        RouteSource::SubmixOutput { submix_id } => Some(GraphNodeId::Submix {
            submix_id: gid(submix_id.as_str()),
        }),
    }
}
pub(crate) fn route_destination_node(destination: &RouteDestination) -> Option<GraphNodeId> {
    match destination {
        RouteDestination::SubmixInput { submix_id } => Some(GraphNodeId::Submix {
            submix_id: gid(submix_id.as_str()),
        }),
        RouteDestination::OutputInput { output_id } => Some(GraphNodeId::Output {
            output_id: gid(output_id.as_str()),
        }),
    }
}
pub(crate) fn stage(tap: SendTap) -> TrackStage {
    match tap {
        SendTap::Input => TrackStage::Input,
        SendTap::PostInputBuiltins => TrackStage::PostInputBuiltins,
        SendTap::PostSimd1 => TrackStage::PostSimd1,
        SendTap::PostDynamic => TrackStage::PostDynamic,
        SendTap::PostSimd2PreFader => TrackStage::PostSimd2PreFader,
        SendTap::PostFader => TrackStage::PostFader,
        SendTap::PostMatrix => TrackStage::PostMatrix,
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
pub(crate) fn effect_path(track: &str, rack: RackId, effect: &str) -> String {
    format!(
        "$.tracks[id={track}].{}.effects[id={effect}]",
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
/// The conversion is `miso_engine_math::db_to_gain_f32` -- the workspace's single dB->linear
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
    let gain = miso_engine_math::db_to_gain_f32(gain_db);
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
/// Lower the prepared entries into the plan's effects, and -- separately -- the live-console
/// control channels of whichever of them a console drives (issue #140 A).
///
/// The two vectors are returned side by side rather than as one because
/// `core::mem::size_of::<RuntimeOp>()` is a reported byte: see
/// [`miso_engine_graph::GraphEffectControlBinding`].
pub(crate) fn into_effects(
    entries: Vec<EffectPreparedEntry>,
    ids: &BTreeMap<(String, RackId, String), EffectNodeId>,
) -> (
    Vec<GraphPreparedEffect>,
    Vec<miso_engine_graph::GraphEffectControlBinding>,
    Vec<miso_engine_graph::GraphEffectObservationBinding>,
) {
    let mut effects = Vec::with_capacity(entries.len());
    let mut controls = Vec::new();
    let mut observations = Vec::new();
    for entry in entries {
        let key = (
            entry.track_id.clone(),
            rack_id(entry.rack),
            entry.effect_id.clone(),
        );
        let node = ids[&key].clone();
        if let Some(control) = entry.control {
            controls.push(miso_engine_graph::GraphEffectControlBinding {
                node: node.clone(),
                control,
            });
        }
        if let Some(observation) = entry.observation {
            observations.push(miso_engine_graph::GraphEffectObservationBinding {
                node: node.clone(),
                observation,
            });
        }
        effects.push(GraphPreparedEffect {
            id: node,
            metadata: entry.metadata,
            processor: entry.processor,
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
pub(crate) fn failure(
    effects: EffectPreparedSession,
    diagnostics: Vec<GraphDiagnostic>,
) -> GraphCompileFailure {
    GraphCompileFailure {
        effects,
        diagnostics: GraphDiagnosticSet::sorted(diagnostics),
    }
}
