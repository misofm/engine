//! The canonical text, its SHA-256 and the Graphviz rendering.
//!
//! None of this runs during a compile (#99 F5): [`write_canonical`] is streamed on demand by
//! `GraphCompiler::evidence` and `GraphCompiler::sha256`.

use super::*;
#[allow(unused_imports)]
use crate::{banks::*, compile::*, estimate::*, ids::*, pdc::*, schedule::*};

pub(crate) fn stage_token(stage: TrackStage) -> &'static str {
    match stage {
        TrackStage::Input => "input",
        TrackStage::PostInputBuiltins => "post-input-builtins",
        TrackStage::PostSimd1 => "post-simd1",
        TrackStage::PostDynamic => "post-dynamic",
        TrackStage::PostSimd2PreFader => "post-simd2-pre-fader",
        TrackStage::PostFader => "post-fader",
        TrackStage::PostMatrix => "post-matrix",
    }
}
pub(crate) fn rack_token(rack: RackId) -> &'static str {
    match rack {
        RackId::Simd1 => "simd1",
        RackId::Dynamic => "dynamic",
        RackId::Simd2 => "simd2",
    }
}
pub(crate) fn node_text(node: &GraphNodeId) -> String {
    match node {
        GraphNodeId::TrackStage { track_id, stage } => {
            format!("track:{}:{}", track_id.as_str(), stage_token(*stage))
        }
        GraphNodeId::Effect(effect) => format!(
            "effect:{}:{}:{}",
            effect.track_id.as_str(),
            rack_token(effect.rack),
            effect.effect_id.as_str()
        ),
        GraphNodeId::Route { route_id } => format!("route:{}", route_id.as_str()),
        GraphNodeId::Submix { submix_id } => format!("submix:{}", submix_id.as_str()),
        GraphNodeId::Output { output_id } => format!("output:{}", output_id.as_str()),
        GraphNodeId::CompensationDelay { edge_id } => format!("delay:{}", edge_text(edge_id)),
    }
}
/// Byte length of [`node_text`] without building it (#99 F5).
///
/// `graph_metadata_bytes` needs three node lengths per edge and one per node, and used to reach
/// them by formatting a heap `String` and throwing it away -- four allocations per edge on every
/// production compile, at 65,537 tracks nearly 1.6 M of them, for a `usize`. Every arm mirrors
/// the corresponding `node_text` arm exactly; `node_text_len_matches_node_text_for_every_variant`
/// is the gate that keeps them in step.
pub(crate) fn node_text_len(node: &GraphNodeId) -> usize {
    match node {
        // "track:{track}:{stage}"
        GraphNodeId::TrackStage { track_id, stage } => {
            "track:".len() + track_id.as_str().len() + 1 + stage_token(*stage).len()
        }
        // "effect:{track}:{rack}:{effect}"
        GraphNodeId::Effect(effect) => {
            "effect:".len()
                + effect.track_id.as_str().len()
                + 1
                + rack_token(effect.rack).len()
                + 1
                + effect.effect_id.as_str().len()
        }
        GraphNodeId::Route { route_id } => "route:".len() + route_id.as_str().len(),
        GraphNodeId::Submix { submix_id } => "submix:".len() + submix_id.as_str().len(),
        GraphNodeId::Output { output_id } => "output:".len() + output_id.as_str().len(),
        GraphNodeId::CompensationDelay { edge_id } => "delay:".len() + edge_text_len(edge_id),
    }
}
/// Byte length of [`edge_text`] without building it. See [`node_text_len`].
pub(crate) fn edge_text_len(edge: &GraphEdgeId) -> usize {
    match edge {
        GraphEdgeId::TrackMain { target } => "track-main:".len() + node_text_len(target),
        GraphEdgeId::RouteSource { route_id } => "route-source:".len() + route_id.as_str().len(),
        GraphEdgeId::RouteDestination { route_id } => {
            "route-destination:".len() + route_id.as_str().len()
        }
        GraphEdgeId::EffectSidechain { effect, port } => {
            "effect-sidechain:".len()
                + effect.track_id.as_str().len()
                + 1
                + rack_token(effect.rack).len()
                + 1
                + effect.effect_id.as_str().len()
                + 1
                + port.len()
        }
    }
}
pub(crate) fn node_kind_token(node: &GraphNodeId) -> &'static str {
    match node {
        GraphNodeId::TrackStage { .. } => "track-stage",
        GraphNodeId::Effect(_) => "effect",
        GraphNodeId::Route { .. } => "route",
        GraphNodeId::Submix { .. } => "submix",
        GraphNodeId::Output { .. } => "output",
        GraphNodeId::CompensationDelay { .. } => "compensation-delay",
    }
}
pub(crate) fn port_kind_token(kind: GraphPortKind) -> &'static str {
    match kind {
        GraphPortKind::MainInput => "main-input",
        GraphPortKind::MainOutput => "main-output",
        GraphPortKind::SidechainInput => "sidechain-input",
    }
}
pub(crate) fn port_text(port: &GraphPortId) -> String {
    format!(
        "{}:{}:{}",
        node_text(&port.node),
        port_kind_token(port.kind),
        port.effect_port.as_deref().unwrap_or("-")
    )
}
pub(crate) fn edge_text(edge: &GraphEdgeId) -> String {
    match edge {
        GraphEdgeId::TrackMain { target } => format!("track-main:{}", node_text(target)),
        GraphEdgeId::RouteSource { route_id } => format!("route-source:{}", route_id.as_str()),
        GraphEdgeId::RouteDestination { route_id } => {
            format!("route-destination:{}", route_id.as_str())
        }
        GraphEdgeId::EffectSidechain { effect, port } => format!(
            "effect-sidechain:{}:{}:{}:{}",
            effect.track_id.as_str(),
            rack_token(effect.rack),
            effect.effect_id.as_str(),
            port
        ),
    }
}
pub(crate) fn tail_text(tail: TailSamples) -> String {
    match tail {
        TailSamples::Finite(samples) => format!("finite:{samples}"),
        TailSamples::Infinite => "infinite".to_owned(),
    }
}
pub(crate) struct CanonicalParts<'a> {
    pub(crate) rate: u32,
    pub(crate) quantum: u32,
    pub(crate) nodes: &'a [GraphNode],
    pub(crate) ports: &'a [GraphPortId],
    pub(crate) edges: &'a [GraphEdge],
    pub(crate) schedule: &'a [GraphNodeId],
    pub(crate) levels: &'a [DependencyLevel],
    pub(crate) routes: &'a [RouteTiming],
    pub(crate) route_transforms: &'a [PreparedRoute],
    pub(crate) delays: &'a [InsertedDelay],
    pub(crate) reductions: &'a [ReductionRecord],
    pub(crate) buffers: &'a [BufferAssignment],
    pub(crate) estimate: &'a GraphResourceEstimate,
}

/// Stream the canonical text into any `fmt::Write` sink.
///
/// #99 F5: this used to be `canonical_bytes`, which materialised the whole dump as one heap
/// `String` -- one `format!` allocation per node, port, edge, order, level, reduction and buffer
/// row -- on **every** production compile, whether or not anyone wanted the evidence. At 458,761
/// nodes that dump is tens of megabytes. It now has two callers, both off the compile path:
/// [`GraphCompiler::evidence`], which still wants the bytes, and [`GraphCompiler::sha256`], which
/// hashes straight through into `Sha256` and never materialises them.
/// Hash sink: `write_canonical` streams straight into SHA-256 with no intermediate `String`.
pub(crate) struct Sha256Writer(pub(crate) Sha256);
impl core::fmt::Write for Sha256Writer {
    fn write_str(&mut self, value: &str) -> core::fmt::Result {
        self.0.update(value.as_bytes());
        Ok(())
    }
}

pub(crate) fn reductions_of(graph: &PreparedGraphPlan) -> Vec<ReductionRecord> {
    reduction_records(&graph.spec.nodes, &graph.spec.edges)
}

pub(crate) fn canonical_parts<'a>(
    graph: &'a PreparedGraphPlan,
    report: &'a GraphCompileReport,
    reductions: &'a [ReductionRecord],
) -> CanonicalParts<'a> {
    CanonicalParts {
        rate: graph.envelope.sample_rate.0,
        quantum: graph.envelope.quantum.0,
        nodes: &graph.spec.nodes,
        ports: &graph.spec.ports,
        edges: &graph.spec.edges,
        schedule: &graph.sequential_schedule,
        levels: &graph.dependency_levels,
        routes: &graph.route_timings,
        route_transforms: graph.routes(),
        delays: &graph.inserted_delays,
        reductions,
        buffers: &graph.buffer_assignments,
        estimate: &report.semantic_estimate,
    }
}

pub(crate) fn write_canonical(out: &mut impl core::fmt::Write, parts: CanonicalParts<'_>) {
    let _ = writeln!(
        out,
        "MISO-GRAPH-V1\nenvelope\t{}\t{}",
        parts.rate, parts.quantum
    );
    for node in parts.nodes {
        let _ = writeln!(
            out,
            "node\t{}\t{}\t{}\t{}",
            node_text(&node.id),
            node_kind_token(&node.id),
            node.latency.0,
            tail_text(node.tail)
        );
    }
    for port in parts.ports {
        let _ = writeln!(out, "port\t{}", port_text(port));
    }
    for edge in parts.edges {
        let _ = writeln!(
            out,
            "edge\t{}\t{}\t{}\t{}",
            edge_text(&edge.id),
            port_text(&edge.source),
            port_text(&edge.destination),
            edge.path
        );
    }
    for delay in parts.delays {
        let _ = writeln!(
            out,
            "delay\t{}\t{}\t{}",
            node_text(&delay.node),
            edge_text(&delay.edge_id),
            delay.samples.0
        );
    }
    for (index, node) in parts.schedule.iter().enumerate() {
        let _ = writeln!(out, "order\t{index}\t{}", node_text(node));
    }
    for level in parts.levels {
        for node in &level.nodes {
            let _ = writeln!(out, "level\t{}\t{}", level.level, node_text(node));
        }
    }
    for reduction in parts.reductions {
        for (rank, edge) in reduction.contributions.iter().enumerate() {
            let _ = writeln!(
                out,
                "reduction\t{}\t{rank}\t{}",
                node_text(&reduction.node),
                edge_text(edge)
            );
        }
    }
    for route in parts.route_transforms {
        let _ = writeln!(
            out,
            "route-transform\t{}\t{:08x}\t{:08x}\t{:08x}\t{:08x}\t{:08x}",
            node_text(&route.node),
            route.transform.gain.to_bits(),
            route.transform.ll.to_bits(),
            route.transform.lr.to_bits(),
            route.transform.rl.to_bits(),
            route.transform.rr.to_bits()
        );
    }
    for route in parts.routes {
        let _ = writeln!(
            out,
            "route-timing\t{}\t{}\t{}\t{}",
            route.route_id.as_str(),
            route.source_arrival.0,
            route.compensation_delay.0,
            route.destination_arrival.0
        );
    }
    for node in parts.nodes {
        let _ = writeln!(
            out,
            "tail\t{}\t{}",
            node_text(&node.id),
            tail_text(node.tail)
        );
    }
    for buffer in parts.buffers {
        let _ = writeln!(
            out,
            "buffer\t{}\t{}",
            port_text(&buffer.port),
            buffer.buffer_index
        );
    }
    let estimate = parts.estimate;
    let _ = writeln!(
        out,
        "estimate\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        estimate.logical_nodes,
        estimate.materialized_nodes,
        estimate.edges,
        estimate.schedule_items,
        estimate.dependency_levels,
        estimate.reductions,
        estimate.routes,
        estimate.effects,
        estimate.audio_buffer_samples,
        estimate.total_delay_samples,
        estimate.delay_bytes,
        estimate.graph_metadata_bytes,
        estimate.declared_effect_bytes,
        estimate.effect_bank_count,
        estimate.effect_bank_scratch_bytes,
        estimate.effect_bank_runtime_buffer_bytes,
        estimate.effect_bank_metadata_bytes,
        estimate.largest_allocation_bytes,
        estimate.incremental_plan_bytes,
        estimate.session_plus_plan_bytes
    );
}
pub(crate) fn dot(nodes: &[GraphNode], edges: &[GraphEdge], delays: &[InsertedDelay]) -> String {
    let mut text = String::from("digraph miso_engine_graph_v1 {\n");
    for node in nodes {
        let id = dot_escape(&node_text(&node.id));
        let label = dot_escape(&format!(
            "{}|{}|latency={}|tail={}",
            node_text(&node.id),
            node_kind_token(&node.id),
            node.latency.0,
            tail_text(node.tail)
        ));
        text.push_str(&format!("  \"{id}\" [label=\"{label}\"];\n"));
    }
    let delay_by_edge: BTreeMap<_, _> =
        delays.iter().map(|delay| (&delay.edge_id, delay)).collect();
    for delay in delays {
        let id = dot_escape(&node_text(&delay.node));
        text.push_str(&format!(
            "  \"{id}\" [shape=box,label=\"pdc|{} samples\"];\n",
            delay.samples.0
        ));
    }
    for edge in edges {
        let source = dot_escape(&node_text(&edge.source.node));
        let destination = dot_escape(&node_text(&edge.destination.node));
        let style = if edge.destination.kind == GraphPortKind::SidechainInput {
            " [style=dashed]"
        } else {
            ""
        };
        if let Some(delay) = delay_by_edge.get(&edge.id) {
            let delay = dot_escape(&node_text(&delay.node));
            text.push_str(&format!("  \"{source}\" -> \"{delay}\"{style};\n"));
            text.push_str(&format!("  \"{delay}\" -> \"{destination}\"{style};\n"));
        } else {
            text.push_str(&format!("  \"{source}\" -> \"{destination}\"{style};\n"));
        }
    }
    text.push_str("}\n");
    text
}
pub(crate) fn dot_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

pub(crate) fn hex_sha256(bytes: &[u8]) -> String {
    hex_digest(&Sha256::digest(bytes))
}
pub(crate) fn hex_digest(digest: &[u8]) -> String {
    use core::fmt::Write;
    let mut output = String::with_capacity(64);
    for byte in digest {
        write!(&mut output, "{byte:02x}").expect("writing to String");
    }
    output
}
