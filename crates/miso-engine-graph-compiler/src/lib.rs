//! Deterministic control-plane lowering of an accepted session and prepared native effects.
#![allow(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};

use miso_engine_core::realtime::RenderEnvelope;
use miso_engine_effect_compiler::{EffectPreparedEntry, EffectPreparedSession, EffectRack};
use miso_engine_effect_contract::{LatencySamples, PreparedSidechainPort, TailSamples};
use miso_engine_graph::{
    BufferAssignment, DependencyLevel, EffectNodeId, GraphCompileCaps, GraphDiagnostic,
    GraphDiagnosticSet, GraphEdge, GraphEdgeId, GraphNode, GraphNodeId, GraphPortId, GraphPortKind,
    GraphPreparedEffect, GraphResourceEstimate, GraphSpec, InsertedDelay, PreparedGraphPlan,
    PreparedGraphPlanParts, PreparedRoute, RackId, RouteTiming, RouteTransform, StableGraphId,
    TrackStage,
};
use miso_engine_session::{
    ChannelMatrix, RouteDestination, RouteSource, SendTap, SidechainDeclaration,
};
use sha2::{Digest, Sha256};

pub struct GraphCompileRequest {
    pub plan_id: u64,
    pub effects: EffectPreparedSession,
    pub caps: GraphCompileCaps,
}
pub struct GraphCompiler;
pub struct PreparedGraphArtifact {
    pub graph: PreparedGraphPlan,
    pub report: GraphCompileReport,
}
pub struct GraphCompileFailure {
    pub effects: EffectPreparedSession,
    pub diagnostics: GraphDiagnosticSet,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphCompileReport {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub sequential_schedule: Vec<GraphNodeId>,
    pub dependency_levels: Vec<DependencyLevel>,
    pub route_timings: Vec<RouteTiming>,
    pub inserted_delays: Vec<InsertedDelay>,
    pub buffer_assignments: Vec<BufferAssignment>,
    pub estimate: GraphResourceEstimate,
    pub canonical_debug_bytes: Vec<u8>,
    pub sha256: String,
    pub dot: String,
}

impl GraphCompiler {
    // The frozen transactional API returns the complete prepared-effect input by value on
    // failure. Boxing it would change that ownership contract solely to optimize a cold path.
    #[allow(clippy::result_large_err)]
    pub fn compile(
        request: GraphCompileRequest,
    ) -> Result<PreparedGraphArtifact, GraphCompileFailure> {
        let GraphCompileRequest {
            plan_id,
            effects,
            caps,
        } = request;
        let mut diagnostics = Vec::new();
        if !caps.all_nonzero() {
            diagnostics.push(diag("graph.resource.limit", "$.graph_compile_caps"));
        }
        let session = effects.session.clone();
        let model = session.normalized_model();
        if model.outputs.len() != 1 {
            diagnostics.push(diag("graph.output.cardinality", "$.outputs"));
        }
        if !diagnostics.is_empty() {
            return Err(failure(effects, diagnostics));
        }

        let mut prepared = BTreeMap::<(String, RackId, String), usize>::new();
        for (index, entry) in effects.entries.iter().enumerate() {
            let key = (
                entry.track_id.clone(),
                rack_id(entry.rack),
                entry.effect_id.clone(),
            );
            if prepared.insert(key, index).is_some() {
                diagnostics.push(diag("graph.effect.duplicate_prepared", "$.effects"));
            }
        }
        let mut declared = BTreeSet::new();
        for track in &model.tracks {
            for (rack, values) in [
                (RackId::Simd1, &track.simd1.effects),
                (RackId::Dynamic, &track.dynamic.effects),
                (RackId::Simd2, &track.simd2.effects),
            ] {
                for effect in values {
                    let key = (
                        track.id.as_str().to_owned(),
                        rack,
                        effect.id.as_str().to_owned(),
                    );
                    declared.insert(key.clone());
                    let Some(index) = prepared.get(&key).copied() else {
                        diagnostics.push(diag(
                            "graph.effect.missing_prepared",
                            &effect_path(track.id.as_str(), rack, effect.id.as_str()),
                        ));
                        continue;
                    };
                    if !sidechain_matches(&effect.sidechain, &effects.entries[index]) {
                        diagnostics.push(diag(
                            "graph.effect.metadata_mismatch",
                            &effect_path(track.id.as_str(), rack, effect.id.as_str()),
                        ));
                    }
                }
            }
        }
        for key in prepared.keys() {
            if !declared.contains(key) {
                diagnostics.push(diag("graph.effect.unexpected_prepared", "$.effects"));
            }
        }
        if !diagnostics.is_empty() {
            return Err(failure(effects, diagnostics));
        }

        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut node_latency = BTreeMap::new();
        let mut node_tail = BTreeMap::new();
        let mut effect_ids = BTreeMap::new();
        let mut route_transforms = Vec::new();
        for track in &model.tracks {
            for stage in stages() {
                let id = track_node(track.id.as_str(), stage);
                add_node(
                    &mut nodes,
                    &mut node_latency,
                    &mut node_tail,
                    id,
                    LatencySamples(0),
                    TailSamples::Finite(0),
                );
            }
            let mut preceding = track_node(track.id.as_str(), TrackStage::Input);
            let builtins = track_node(track.id.as_str(), TrackStage::PostInputBuiltins);
            add_main_edge(
                &mut edges,
                preceding.clone(),
                builtins.clone(),
                "$.tracks".to_owned(),
            );
            preceding = builtins;
            for (rack, values, boundary) in [
                (RackId::Simd1, &track.simd1.effects, TrackStage::PostSimd1),
                (
                    RackId::Dynamic,
                    &track.dynamic.effects,
                    TrackStage::PostDynamic,
                ),
                (
                    RackId::Simd2,
                    &track.simd2.effects,
                    TrackStage::PostSimd2PreFader,
                ),
            ] {
                for effect in values {
                    let id = EffectNodeId {
                        track_id: gid(track.id.as_str()),
                        rack,
                        effect_id: gid(effect.id.as_str()),
                    };
                    let node = GraphNodeId::Effect(id.clone());
                    let index = prepared[&(
                        track.id.as_str().to_owned(),
                        rack,
                        effect.id.as_str().to_owned(),
                    )];
                    let metadata = effects.entries[index].metadata;
                    add_node(
                        &mut nodes,
                        &mut node_latency,
                        &mut node_tail,
                        node.clone(),
                        metadata.latency,
                        metadata.tail,
                    );
                    add_main_edge(
                        &mut edges,
                        preceding.clone(),
                        node.clone(),
                        effect_path(track.id.as_str(), rack, effect.id.as_str()),
                    );
                    preceding = node.clone();
                    effect_ids.insert(
                        (
                            track.id.as_str().to_owned(),
                            rack,
                            effect.id.as_str().to_owned(),
                        ),
                        id,
                    );
                }
                let end = track_node(track.id.as_str(), boundary);
                add_main_edge(
                    &mut edges,
                    preceding.clone(),
                    end.clone(),
                    "$.tracks".to_owned(),
                );
                preceding = end;
            }
            let fader = track_node(track.id.as_str(), TrackStage::PostFader);
            let matrix = track_node(track.id.as_str(), TrackStage::PostMatrix);
            add_main_edge(&mut edges, preceding, fader.clone(), "$.tracks".to_owned());
            add_main_edge(&mut edges, fader, matrix, "$.tracks".to_owned());
        }
        for submix in &model.submixes {
            let id = GraphNodeId::Submix {
                submix_id: gid(submix.id.as_str()),
            };
            add_node(
                &mut nodes,
                &mut node_latency,
                &mut node_tail,
                id,
                LatencySamples(0),
                TailSamples::Finite(0),
            );
        }
        for output in &model.outputs {
            let id = GraphNodeId::Output {
                output_id: gid(output.id.as_str()),
            };
            add_node(
                &mut nodes,
                &mut node_latency,
                &mut node_tail,
                id,
                LatencySamples(0),
                TailSamples::Finite(0),
            );
        }
        for route in &model.routes {
            let Some(transform) = route_transform(route.gain_db, &route.channel_matrix) else {
                diagnostics.push(diag(
                    "graph.gain.non_finite",
                    &format!("$.routes[id={}].gain_db", route.id),
                ));
                continue;
            };
            let route_node = GraphNodeId::Route {
                route_id: gid(route.id.as_str()),
            };
            add_node(
                &mut nodes,
                &mut node_latency,
                &mut node_tail,
                route_node.clone(),
                LatencySamples(0),
                TailSamples::Finite(0),
            );
            let Some(source) = route_source_node(&route.source) else {
                diagnostics.push(diag(
                    "graph.port.unknown",
                    &format!("$.routes[id={}].source", route.id),
                ));
                continue;
            };
            let Some(destination) = route_destination_node(&route.destination) else {
                diagnostics.push(diag(
                    "graph.port.unknown",
                    &format!("$.routes[id={}].destination", route.id),
                ));
                continue;
            };
            add_route_source_edge(&mut edges, source, route_node.clone(), route.id.as_str());
            add_route_destination_edge(&mut edges, route_node, destination, route.id.as_str());
            route_transforms.push(PreparedRoute {
                node: GraphNodeId::Route {
                    route_id: gid(route.id.as_str()),
                },
                transform,
            });
        }
        for track in &model.tracks {
            for (rack, values) in [
                (RackId::Simd1, &track.simd1.effects),
                (RackId::Dynamic, &track.dynamic.effects),
                (RackId::Simd2, &track.simd2.effects),
            ] {
                for effect in values {
                    let SidechainDeclaration::Routed(sidechain) = &effect.sidechain else {
                        continue;
                    };
                    let Some(source) = route_source_node(&sidechain.source) else {
                        diagnostics.push(diag(
                            "graph.port.unknown",
                            &format!("$.tracks[id={}].sidechain", track.id),
                        ));
                        continue;
                    };
                    let key = (
                        track.id.as_str().to_owned(),
                        rack,
                        effect.id.as_str().to_owned(),
                    );
                    let id = effect_ids[&key].clone();
                    let destination = GraphNodeId::Effect(id.clone());
                    edges.push(GraphEdge {
                        id: GraphEdgeId::EffectSidechain {
                            effect: id,
                            port: sidechain.port_id.as_str().to_owned(),
                        },
                        source: port(source, GraphPortKind::MainOutput),
                        destination: GraphPortId {
                            node: destination,
                            kind: GraphPortKind::SidechainInput,
                            effect_port: Some(sidechain.port_id.as_str().to_owned()),
                        },
                        path: format!("$.tracks[id={}].sidechain", track.id),
                    });
                }
            }
        }
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        edges.sort_by(|a, b| a.id.cmp(&b.id));
        if nodes.len() as u64 > caps.maximum_nodes || edges.len() as u64 > caps.maximum_edges {
            diagnostics.push(diag("graph.resource.limit", "$.graph_compile_caps"));
        }
        if let Some(cycle) = cycle_witness(&nodes, &edges) {
            diagnostics.push(GraphDiagnostic {
                code: "graph.cycle",
                path: cycle.1.first().cloned().unwrap_or_else(|| "$".to_owned()),
                cycle: cycle.0,
                cycle_edge_paths: cycle.1,
            });
        }
        if !diagnostics.is_empty() {
            return Err(failure(effects, diagnostics));
        }
        let (schedule, levels) = topo(&nodes, &edges).expect("acyclic graph has schedule");
        if schedule.len() as u64 > caps.maximum_schedule_items
            || levels.len() as u64 > caps.maximum_dependency_levels
        {
            return Err(failure(
                effects,
                vec![diag("graph.resource.limit", "$.graph_compile_caps")],
            ));
        }
        let timing = match timings(&schedule, &edges, &node_latency, &node_tail, &caps) {
            Ok(value) => value,
            Err(diagnostic) => return Err(failure(effects, vec![diagnostic])),
        };
        let buffers = buffer_assignments(&schedule);
        let Some(estimate) = resource_estimate(
            session.quantum().0,
            session.resource_estimate().requested_runtime_bytes,
            &nodes,
            &edges,
            &schedule,
            &levels,
            &buffers,
            &timing,
            &effects.entries,
        ) else {
            return Err(failure(
                effects,
                vec![diag("graph.resource.arithmetic_overflow", "$.graph")],
            ));
        };
        if !estimate_fits_platform(&estimate) {
            return Err(failure(
                effects,
                vec![diag("graph.resource.arithmetic_overflow", "$.graph")],
            ));
        }
        if estimate.materialized_nodes > caps.maximum_nodes
            || estimate.edges > caps.maximum_edges
            || estimate.schedule_items > caps.maximum_schedule_items
            || estimate.dependency_levels > caps.maximum_dependency_levels
            || estimate.audio_buffer_samples > caps.maximum_audio_buffer_samples
            || estimate.graph_metadata_bytes > caps.maximum_graph_bytes
            || estimate.incremental_plan_bytes > caps.maximum_plan_bytes
            || estimate.largest_allocation_bytes > caps.maximum_single_allocation_bytes
        {
            return Err(failure(
                effects,
                vec![diag("graph.resource.limit", "$.graph_compile_caps")],
            ));
        }
        if estimate.session_plus_plan_bytes > model.limits.memory_bytes {
            return Err(failure(
                effects,
                vec![diag("graph.resource.limit", "$.limits.memory_bytes")],
            ));
        }
        let debug = canonical_bytes(CanonicalParts {
            rate: session.sample_rate().0,
            quantum: session.quantum().0,
            nodes: &nodes,
            edges: &edges,
            schedule: &schedule,
            levels: &levels,
            routes: &timing.routes,
            buffers: &buffers,
            estimate: &estimate,
        });
        let sha256 = hex_sha256(&debug);
        let dot = dot(&nodes, &edges);
        let effect_nodes = into_effects(effects.entries, &effect_ids);
        let spec = GraphSpec {
            ports: ports_for(&nodes, &edges),
            nodes: nodes.clone(),
            edges: edges.clone(),
        };
        let required_bindings = schedule
            .iter()
            .filter(|node| {
                matches!(
                    node,
                    GraphNodeId::TrackStage {
                        stage: TrackStage::Input
                            | TrackStage::PostInputBuiltins
                            | TrackStage::PostFader
                            | TrackStage::PostMatrix,
                        ..
                    } | GraphNodeId::Output { .. }
                )
            })
            .cloned()
            .collect();
        let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
            plan_id,
            spec,
            sequential_schedule: schedule.clone(),
            dependency_levels: levels.clone(),
            route_timings: timing.routes.clone(),
            inserted_delays: timing.delays.clone(),
            buffer_assignments: buffers.clone(),
            estimate: estimate.clone(),
            envelope: RenderEnvelope {
                sample_rate: session.sample_rate(),
                quantum: session.quantum(),
                input_channels: None,
                output_channels: core::num::NonZeroUsize::new(2).expect("constant"),
            },
            required_bindings,
            routes: route_transforms,
            effects: effect_nodes,
        });
        Ok(PreparedGraphArtifact {
            graph,
            report: GraphCompileReport {
                nodes,
                edges,
                sequential_schedule: schedule,
                dependency_levels: levels,
                route_timings: timing.routes,
                inserted_delays: timing.delays,
                buffer_assignments: buffers,
                estimate,
                canonical_debug_bytes: debug,
                sha256,
                dot,
            },
        })
    }
}

struct TimingResult {
    routes: Vec<RouteTiming>,
    delays: Vec<InsertedDelay>,
    total_delay: u64,
    delay_count: u64,
}

#[allow(clippy::too_many_arguments)]
fn resource_estimate(
    quantum: u32,
    session_bytes: u64,
    nodes: &[GraphNode],
    edges: &[GraphEdge],
    schedule: &[GraphNodeId],
    levels: &[DependencyLevel],
    buffers: &[BufferAssignment],
    timing: &TimingResult,
    effects: &[EffectPreparedEntry],
) -> Option<GraphResourceEstimate> {
    let count = |value: usize| u64::try_from(value).ok();
    let logical_nodes = count(nodes.len())?;
    let logical_edges = count(edges.len())?;
    let materialized_nodes = logical_nodes.checked_add(timing.delay_count)?;
    let materialized_edges = logical_edges.checked_add(timing.delay_count)?;
    let schedule_items = count(schedule.len())?.checked_add(timing.delay_count)?;
    let dependency_levels = count(levels.len())?;
    let reductions = count(
        nodes
            .iter()
            .filter(|node| {
                matches!(
                    node.id,
                    GraphNodeId::Submix { .. } | GraphNodeId::Output { .. }
                ) && edges
                    .iter()
                    .filter(|edge| {
                        edge.destination.node == node.id
                            && edge.destination.kind == GraphPortKind::MainInput
                    })
                    .nth(1)
                    .is_some()
            })
            .count(),
    )?;
    let routes = count(
        nodes
            .iter()
            .filter(|node| matches!(node.id, GraphNodeId::Route { .. }))
            .count(),
    )?;
    let effect_count = count(effects.len())?;
    let maximum_inputs = nodes
        .iter()
        .map(|node| {
            edges
                .iter()
                .filter(|edge| edge.destination.node == node.id)
                .count()
        })
        .max()
        .unwrap_or(0);
    let maximum_inputs = count(maximum_inputs)?;
    let quantum = u64::from(quantum);
    // The scalar executor owns one dual-mono output per logical node and one dual-mono
    // contribution buffer per logical edge, plus one scalar pairwise-reduction work array.
    let audio_buffer_samples = logical_nodes
        .checked_add(logical_edges)?
        .checked_mul(2)?
        .checked_mul(quantum)?
        .checked_add(maximum_inputs)?;
    let audio_bytes = audio_buffer_samples.checked_mul(4)?;
    let delay_bytes = timing.total_delay.checked_mul(8)?;
    let mut declared_effect_bytes = 0_u64;
    for effect in effects {
        declared_effect_bytes = declared_effect_bytes
            .checked_add(effect.metadata.state_sizes.total()?)?
            .checked_add(effect.metadata.scratch_bytes)?;
    }
    let graph_metadata_bytes =
        graph_metadata_bytes(nodes, edges, schedule, levels, buffers, timing)?;
    let incremental_plan_bytes = audio_bytes
        .checked_add(delay_bytes)?
        .checked_add(declared_effect_bytes)?
        .checked_add(graph_metadata_bytes)?;
    let lane_bytes = quantum.checked_mul(4)?;
    let mut delay_lane_bytes = 0_u64;
    for delay in &timing.delays {
        delay_lane_bytes = delay_lane_bytes.max(delay.samples.0.checked_mul(4)?);
    }
    let reduction_bytes = maximum_inputs.checked_mul(4)?;
    let largest_allocation_bytes = graph_metadata_bytes
        .max(lane_bytes)
        .max(delay_lane_bytes)
        .max(reduction_bytes);
    Some(GraphResourceEstimate {
        logical_nodes,
        materialized_nodes,
        edges: materialized_edges,
        schedule_items,
        dependency_levels,
        reductions,
        routes,
        effects: effect_count,
        audio_buffer_samples,
        total_delay_samples: timing.total_delay,
        delay_bytes,
        graph_metadata_bytes,
        declared_effect_bytes,
        largest_allocation_bytes,
        incremental_plan_bytes,
        session_plus_plan_bytes: session_bytes.checked_add(incremental_plan_bytes)?,
    })
}

fn graph_metadata_bytes(
    nodes: &[GraphNode],
    edges: &[GraphEdge],
    schedule: &[GraphNodeId],
    levels: &[DependencyLevel],
    buffers: &[BufferAssignment],
    timing: &TimingResult,
) -> Option<u64> {
    let sized = |count: usize, bytes: usize| {
        u64::try_from(count)
            .ok()?
            .checked_mul(u64::try_from(bytes).ok()?)
    };
    let mut total = sized(nodes.len(), core::mem::size_of::<GraphNode>())?
        .checked_add(sized(edges.len(), core::mem::size_of::<GraphEdge>())?)?
        .checked_add(sized(schedule.len(), core::mem::size_of::<GraphNodeId>())?)?
        .checked_add(sized(
            levels.len(),
            core::mem::size_of::<DependencyLevel>(),
        )?)?
        .checked_add(sized(
            buffers.len(),
            core::mem::size_of::<BufferAssignment>(),
        )?)?
        .checked_add(sized(
            timing.routes.len(),
            core::mem::size_of::<RouteTiming>(),
        )?)?
        .checked_add(sized(
            timing.delays.len(),
            core::mem::size_of::<InsertedDelay>(),
        )?)?;
    for node in nodes {
        total = total.checked_add(u64::try_from(node_text(&node.id).len()).ok()?)?;
    }
    for edge in edges {
        total = total
            .checked_add(u64::try_from(edge.path.len()).ok()?)?
            .checked_add(u64::try_from(node_text(&edge.source.node).len()).ok()?)?
            .checked_add(u64::try_from(node_text(&edge.destination.node).len()).ok()?)?;
    }
    Some(total)
}

fn estimate_fits_platform(estimate: &GraphResourceEstimate) -> bool {
    [
        estimate.materialized_nodes,
        estimate.edges,
        estimate.schedule_items,
        estimate.audio_buffer_samples,
        estimate.total_delay_samples,
        estimate.delay_bytes,
        estimate.graph_metadata_bytes,
        estimate.declared_effect_bytes,
        estimate.largest_allocation_bytes,
        estimate.incremental_plan_bytes,
        estimate.session_plus_plan_bytes,
    ]
    .into_iter()
    .all(|value| usize::try_from(value).is_ok() && isize::try_from(value).is_ok())
}

fn timings(
    schedule: &[GraphNodeId],
    edges: &[GraphEdge],
    latencies: &BTreeMap<GraphNodeId, LatencySamples>,
    tails: &BTreeMap<GraphNodeId, TailSamples>,
    caps: &GraphCompileCaps,
) -> Result<TimingResult, GraphDiagnostic> {
    let mut arrivals = BTreeMap::<GraphNodeId, u64>::new();
    let mut extents = BTreeMap::<GraphNodeId, TailSamples>::new();
    let mut total_delay: u64 = 0;
    let mut delay_count: u64 = 0;
    let mut routes = Vec::new();
    let mut delays = Vec::new();
    for node in schedule {
        let incoming: Vec<_> = edges
            .iter()
            .filter(|edge| edge.destination.node == *node)
            .collect();
        let max = incoming
            .iter()
            .filter_map(|edge| arrivals.get(&edge.source.node).copied())
            .max()
            .unwrap_or(0);
        for edge in &incoming {
            let source = arrivals.get(&edge.source.node).copied().unwrap_or(0);
            let delay = max
                .checked_sub(source)
                .ok_or_else(|| diag("graph.pdc.arithmetic_overflow", &edge.path))?;
            if delay > caps.maximum_delay_samples_per_edge {
                return Err(diag("graph.pdc.edge_limit", &edge.path));
            }
            total_delay = total_delay
                .checked_add(delay)
                .ok_or_else(|| diag("graph.pdc.arithmetic_overflow", &edge.path))?;
            if total_delay > caps.maximum_total_delay_samples {
                return Err(diag("graph.pdc.total_limit", &edge.path));
            }
            if delay > 0 {
                delay_count += 1;
                delays.push(InsertedDelay {
                    node: GraphNodeId::CompensationDelay {
                        edge_id: Box::new(edge.id.clone()),
                    },
                    edge_id: edge.id.clone(),
                    samples: LatencySamples(delay),
                });
            }
            if let GraphEdgeId::RouteDestination { route_id } = &edge.id {
                routes.push(RouteTiming {
                    route_id: route_id.clone(),
                    source_arrival: LatencySamples(source),
                    compensation_delay: LatencySamples(delay),
                    destination_arrival: LatencySamples(max),
                });
            }
        }
        let latency = latencies.get(node).copied().unwrap_or(LatencySamples(0)).0;
        arrivals.insert(
            node.clone(),
            max.checked_add(latency)
                .ok_or_else(|| diag("graph.pdc.arithmetic_overflow", "$.graph"))?,
        );
        let mut extent = TailSamples::Finite(0);
        for edge in &incoming {
            let source_arrival = arrivals.get(&edge.source.node).copied().unwrap_or(0);
            let compensation_delay = max
                .checked_sub(source_arrival)
                .ok_or_else(|| diag("graph.pdc.arithmetic_overflow", &edge.path))?;
            extent = max_tail(
                extent,
                shifted_tail(
                    *extents
                        .get(&edge.source.node)
                        .unwrap_or(&TailSamples::Finite(0)),
                    compensation_delay,
                )?,
            );
        }
        extent = shifted_tail(extent, latency)?;
        extent = match (
            extent,
            tails.get(node).copied().unwrap_or(TailSamples::Finite(0)),
        ) {
            (TailSamples::Infinite, _) | (_, TailSamples::Infinite) => TailSamples::Infinite,
            (TailSamples::Finite(value), TailSamples::Finite(declared_tail)) => value
                .checked_add(declared_tail)
                .map(TailSamples::Finite)
                .ok_or_else(|| diag("graph.tail.arithmetic_overflow", "$.graph"))?,
        };
        if let TailSamples::Finite(value) = extent
            && value > caps.maximum_finite_tail_samples
        {
            return Err(diag("graph.tail.limit", "$.graph"));
        }
        extents.insert(node.clone(), extent);
    }
    routes.sort_by(|a, b| a.route_id.cmp(&b.route_id));
    delays.sort_by(|left, right| left.edge_id.cmp(&right.edge_id));
    Ok(TimingResult {
        routes,
        delays,
        total_delay,
        delay_count,
    })
}
fn shifted_tail(value: TailSamples, add: u64) -> Result<TailSamples, GraphDiagnostic> {
    match value {
        TailSamples::Infinite => Ok(TailSamples::Infinite),
        TailSamples::Finite(v) => v
            .checked_add(add)
            .map(TailSamples::Finite)
            .ok_or_else(|| diag("graph.tail.arithmetic_overflow", "$.graph")),
    }
}
fn max_tail(a: TailSamples, b: TailSamples) -> TailSamples {
    match (a, b) {
        (TailSamples::Infinite, _) | (_, TailSamples::Infinite) => TailSamples::Infinite,
        (TailSamples::Finite(a), TailSamples::Finite(b)) => TailSamples::Finite(a.max(b)),
    }
}
fn topo(
    nodes: &[GraphNode],
    edges: &[GraphEdge],
) -> Option<(Vec<GraphNodeId>, Vec<DependencyLevel>)> {
    let mut degree: BTreeMap<_, u64> = nodes.iter().map(|node| (node.id.clone(), 0)).collect();
    for edge in edges {
        *degree.get_mut(&edge.destination.node)? += 1;
    }
    let mut ready: BTreeSet<_> = degree
        .iter()
        .filter_map(|(id, degree)| (*degree == 0).then_some(id.clone()))
        .collect();
    let mut schedule = Vec::new();
    let mut levels = BTreeMap::<u64, Vec<GraphNodeId>>::new();
    let mut node_levels = BTreeMap::new();
    while let Some(node) = ready.pop_first() {
        let level = edges
            .iter()
            .filter(|edge| edge.destination.node == node)
            .filter_map(|edge| node_levels.get(&edge.source.node))
            .copied()
            .max()
            .map_or(0, |value| value + 1);
        node_levels.insert(node.clone(), level);
        levels.entry(level).or_default().push(node.clone());
        schedule.push(node.clone());
        for edge in edges.iter().filter(|edge| edge.source.node == node) {
            let degree = degree.get_mut(&edge.destination.node)?;
            *degree -= 1;
            if *degree == 0 {
                ready.insert(edge.destination.node.clone());
            }
        }
    }
    if schedule.len() != nodes.len() {
        None
    } else {
        Some((
            schedule,
            levels
                .into_iter()
                .map(|(level, nodes)| DependencyLevel { level, nodes })
                .collect(),
        ))
    }
}
fn cycle_witness(
    nodes: &[GraphNode],
    edges: &[GraphEdge],
) -> Option<(Vec<GraphNodeId>, Vec<String>)> {
    let mut degree: BTreeMap<_, u64> = nodes.iter().map(|node| (node.id.clone(), 0)).collect();
    for edge in edges {
        *degree.get_mut(&edge.destination.node)? += 1;
    }
    let mut ready: BTreeSet<_> = degree
        .iter()
        .filter_map(|(id, degree)| (*degree == 0).then_some(id.clone()))
        .collect();
    while let Some(node) = ready.pop_first() {
        for edge in edges.iter().filter(|edge| edge.source.node == node) {
            let degree = degree.get_mut(&edge.destination.node)?;
            *degree -= 1;
            if *degree == 0 {
                ready.insert(edge.destination.node.clone());
            }
        }
    }
    let remaining: BTreeSet<_> = degree
        .into_iter()
        .filter_map(|(node, degree)| (degree != 0).then_some(node))
        .collect();
    let adjacency: BTreeMap<_, Vec<_>> = remaining
        .iter()
        .map(|node| {
            let mut outgoing: Vec<_> = edges
                .iter()
                .filter(|edge| {
                    edge.source.node == *node && remaining.contains(&edge.destination.node)
                })
                .collect();
            outgoing.sort_by(|left, right| left.id.cmp(&right.id));
            (node.clone(), outgoing)
        })
        .collect();

    // Kahn's residual can include acyclic nodes downstream of a cycle. Search each residual node
    // in semantic order instead of assuming the smallest residual node itself lies on a cycle.
    for start in &remaining {
        let mut nodes_path = vec![start.clone()];
        let mut edge_path = Vec::new();
        let mut on_path = BTreeSet::from([start.clone()]);
        let mut stack = vec![(start.clone(), 0usize)];
        while let Some((node, next_edge)) = stack.last_mut() {
            let outgoing = &adjacency[node];
            if *next_edge == outgoing.len() {
                stack.pop();
                if let Some(removed) = nodes_path.pop() {
                    on_path.remove(&removed);
                }
                if !edge_path.is_empty() {
                    edge_path.pop();
                }
                continue;
            }
            let edge = outgoing[*next_edge];
            *next_edge += 1;
            if edge.destination.node == *start {
                let mut witness = nodes_path.clone();
                witness.push(start.clone());
                let mut witness_edges = edge_path.clone();
                witness_edges.push(edge.path.clone());
                return Some((witness, witness_edges));
            }
            if on_path.insert(edge.destination.node.clone()) {
                nodes_path.push(edge.destination.node.clone());
                edge_path.push(edge.path.clone());
                stack.push((edge.destination.node.clone(), 0));
            }
        }
    }
    None
}
fn buffer_assignments(schedule: &[GraphNodeId]) -> Vec<BufferAssignment> {
    schedule
        .iter()
        .enumerate()
        .map(|(index, node)| BufferAssignment {
            port: port(node.clone(), GraphPortKind::MainOutput),
            buffer_index: index as u64,
        })
        .collect()
}
fn ports_for(nodes: &[GraphNode], edges: &[GraphEdge]) -> Vec<GraphPortId> {
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
fn add_node(
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
fn add_main_edge(
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
fn add_route_source_edge(
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
fn add_route_destination_edge(
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
fn port(node: GraphNodeId, kind: GraphPortKind) -> GraphPortId {
    GraphPortId {
        node,
        kind,
        effect_port: None,
    }
}
fn gid(value: &str) -> StableGraphId {
    StableGraphId::parse(value).expect("accepted stable session ID")
}
fn track_node(track: &str, stage: TrackStage) -> GraphNodeId {
    GraphNodeId::TrackStage {
        track_id: gid(track),
        stage,
    }
}
fn route_source_node(source: &RouteSource) -> Option<GraphNodeId> {
    match source {
        RouteSource::Track { track_id, tap } => Some(track_node(track_id.as_str(), stage(*tap))),
        RouteSource::SubmixOutput { submix_id } => Some(GraphNodeId::Submix {
            submix_id: gid(submix_id.as_str()),
        }),
    }
}
fn route_destination_node(destination: &RouteDestination) -> Option<GraphNodeId> {
    match destination {
        RouteDestination::SubmixInput { submix_id } => Some(GraphNodeId::Submix {
            submix_id: gid(submix_id.as_str()),
        }),
        RouteDestination::OutputInput { output_id } => Some(GraphNodeId::Output {
            output_id: gid(output_id.as_str()),
        }),
    }
}
fn stage(tap: SendTap) -> TrackStage {
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
fn stages() -> [TrackStage; 7] {
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
fn rack_id(rack: EffectRack) -> RackId {
    match rack {
        EffectRack::Simd1 => RackId::Simd1,
        EffectRack::Dynamic => RackId::Dynamic,
        EffectRack::Simd2 => RackId::Simd2,
    }
}
fn effect_path(track: &str, rack: RackId, effect: &str) -> String {
    format!(
        "$.tracks[id={track}].{}.effects[id={effect}]",
        match rack {
            RackId::Simd1 => "simd1",
            RackId::Dynamic => "dynamic",
            RackId::Simd2 => "simd2",
        }
    )
}
fn sidechain_matches(declaration: &SidechainDeclaration, entry: &EffectPreparedEntry) -> bool {
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
fn route_transform(gain_db: f32, matrix: &ChannelMatrix) -> Option<RouteTransform> {
    let gain = 10_f64.powf(f64::from(gain_db) / 20.0) as f32;
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
fn into_effects(
    entries: Vec<EffectPreparedEntry>,
    ids: &BTreeMap<(String, RackId, String), EffectNodeId>,
) -> Vec<GraphPreparedEffect> {
    entries
        .into_iter()
        .map(|entry| {
            let key = (
                entry.track_id.clone(),
                rack_id(entry.rack),
                entry.effect_id.clone(),
            );
            GraphPreparedEffect {
                id: ids[&key].clone(),
                metadata: entry.metadata,
                processor: entry.processor,
            }
        })
        .collect()
}
fn diag(code: &'static str, path: &str) -> GraphDiagnostic {
    GraphDiagnostic {
        code,
        path: path.to_owned(),
        cycle: Vec::new(),
        cycle_edge_paths: Vec::new(),
    }
}
fn failure(
    effects: EffectPreparedSession,
    diagnostics: Vec<GraphDiagnostic>,
) -> GraphCompileFailure {
    GraphCompileFailure {
        effects,
        diagnostics: GraphDiagnosticSet::sorted(diagnostics),
    }
}
fn node_text(node: &GraphNodeId) -> String {
    match node {
        GraphNodeId::TrackStage { track_id, stage } => {
            format!("track:{}:{stage:?}", track_id.as_str())
        }
        GraphNodeId::Effect(effect) => format!(
            "effect:{}:{:?}:{}",
            effect.track_id.as_str(),
            effect.rack,
            effect.effect_id.as_str()
        ),
        GraphNodeId::Route { route_id } => format!("route:{}", route_id.as_str()),
        GraphNodeId::Submix { submix_id } => format!("submix:{}", submix_id.as_str()),
        GraphNodeId::Output { output_id } => format!("output:{}", output_id.as_str()),
        GraphNodeId::CompensationDelay { edge_id } => format!("delay:{edge_id:?}"),
    }
}
struct CanonicalParts<'a> {
    rate: u32,
    quantum: u32,
    nodes: &'a [GraphNode],
    edges: &'a [GraphEdge],
    schedule: &'a [GraphNodeId],
    levels: &'a [DependencyLevel],
    routes: &'a [RouteTiming],
    buffers: &'a [BufferAssignment],
    estimate: &'a GraphResourceEstimate,
}

fn canonical_bytes(parts: CanonicalParts<'_>) -> Vec<u8> {
    let mut text = format!(
        "MISO-GRAPH-V1\nenvelope\t{}\t{}\n",
        parts.rate, parts.quantum
    );
    for node in parts.nodes {
        text.push_str(&format!(
            "node\t{}\t{}\t{:?}\n",
            node_text(&node.id),
            node.latency.0,
            node.tail
        ));
    }
    for edge in parts.edges {
        text.push_str(&format!(
            "edge\t{:?}\t{}\t{}\n",
            edge.id,
            node_text(&edge.source.node),
            node_text(&edge.destination.node)
        ));
    }
    for node in parts.schedule {
        text.push_str(&format!("order\t{}\n", node_text(node)));
    }
    for level in parts.levels {
        for node in &level.nodes {
            text.push_str(&format!("level\t{}\t{}\n", level.level, node_text(node)));
        }
    }
    for route in parts.routes {
        text.push_str(&format!(
            "route\t{}\t{}\t{}\t{}\n",
            route.route_id.as_str(),
            route.source_arrival.0,
            route.compensation_delay.0,
            route.destination_arrival.0
        ));
    }
    for buffer in parts.buffers {
        text.push_str(&format!(
            "buffer\t{}\t{}\n",
            node_text(&buffer.port.node),
            buffer.buffer_index
        ));
    }
    text.push_str(&format!(
        "estimate\t{}\t{}\t{}\n",
        parts.estimate.logical_nodes, parts.estimate.edges, parts.estimate.incremental_plan_bytes
    ));
    text.into_bytes()
}
fn dot(nodes: &[GraphNode], edges: &[GraphEdge]) -> String {
    let mut text = String::from("digraph miso_engine_graph_v1 {\n");
    for node in nodes {
        text.push_str(&format!("  \"{}\";\n", node_text(&node.id)));
    }
    for edge in edges {
        text.push_str(&format!(
            "  \"{}\" -> \"{}\";\n",
            node_text(&edge.source.node),
            node_text(&edge.destination.node)
        ));
    }
    text.push_str("}\n");
    text
}

fn hex_sha256(bytes: &[u8]) -> String {
    use core::fmt::Write;
    let mut output = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        write!(&mut output, "{byte:02x}").expect("writing to String");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_core::realtime::{PlanarBufferMut, RenderIo, RenderTime};
    use miso_engine_effect_compiler::EffectPreparedSession;
    use miso_engine_graph::{
        GraphBindingBlock, GraphNodeBinding, GraphRuntimeBindings, GraphRuntimeProcessor,
    };
    use miso_engine_session::{CompileCaps, compile_session, parse_session_toml};

    const SESSION_FIXTURE: &str = include_str!("../../../fixtures/session/v1/canonical.toml");

    struct IdentityBinding;
    impl GraphRuntimeProcessor for IdentityBinding {
        fn process(
            &mut self,
            _block: GraphBindingBlock<'_>,
        ) -> Result<(), miso_engine_core::realtime::RenderError> {
            Ok(())
        }
    }

    struct ImpulseBinding;
    impl GraphRuntimeProcessor for ImpulseBinding {
        fn process(
            &mut self,
            block: GraphBindingBlock<'_>,
        ) -> Result<(), miso_engine_core::realtime::RenderError> {
            block.left.fill(0.0);
            block.right.fill(0.0);
            block.left[0] = 1.0;
            block.right[0] = -1.0;
            Ok(())
        }
    }

    fn node(name: &str) -> GraphNodeId {
        GraphNodeId::Submix {
            submix_id: gid(name),
        }
    }

    fn graph_node(name: &str, latency: u64, tail: TailSamples) -> GraphNode {
        GraphNode {
            id: node(name),
            latency: LatencySamples(latency),
            tail,
        }
    }

    fn edge(name: &str, source: &str, destination: &str) -> GraphEdge {
        GraphEdge {
            id: GraphEdgeId::RouteDestination {
                route_id: gid(name),
            },
            source: port(node(source), GraphPortKind::MainOutput),
            destination: port(node(destination), GraphPortKind::MainInput),
            path: format!("$.routes[id={name}]"),
        }
    }

    fn caps(maximum_finite_tail_samples: u64) -> GraphCompileCaps {
        GraphCompileCaps {
            maximum_nodes: 100,
            maximum_edges: 100,
            maximum_schedule_items: 100,
            maximum_dependency_levels: 100,
            maximum_audio_buffer_samples: 100,
            maximum_delay_samples_per_edge: 100,
            maximum_total_delay_samples: 100,
            maximum_graph_bytes: 100,
            maximum_plan_bytes: 100,
            maximum_single_allocation_bytes: 100,
            maximum_finite_tail_samples,
        }
    }

    fn integration_caps() -> GraphCompileCaps {
        GraphCompileCaps {
            maximum_nodes: 10_000,
            maximum_edges: 10_000,
            maximum_schedule_items: 10_000,
            maximum_dependency_levels: 10_000,
            maximum_audio_buffer_samples: 10_000_000,
            maximum_delay_samples_per_edge: 1_000_000,
            maximum_total_delay_samples: 10_000_000,
            maximum_graph_bytes: 10_000_000,
            maximum_plan_bytes: 100_000_000,
            maximum_single_allocation_bytes: 10_000_000,
            maximum_finite_tail_samples: 10_000_000,
        }
    }

    #[test]
    fn cycle_witness_skips_acyclic_residual_nodes_downstream_of_cycle() {
        let nodes = [
            graph_node("a", 0, TailSamples::Finite(0)),
            graph_node("b", 0, TailSamples::Finite(0)),
            graph_node("c", 0, TailSamples::Finite(0)),
        ];
        // `a` sorts first and is downstream of the b/c cycle. Kahn leaves all three residual.
        let edges = [
            edge("to-a", "b", "a"),
            edge("to-b", "c", "b"),
            edge("to-c", "b", "c"),
        ];
        let (witness, paths) = cycle_witness(&nodes, &edges).expect("cycle");
        assert_eq!(witness, [node("b"), node("c"), node("b")]);
        assert_eq!(paths, ["$.routes[id=to-c]", "$.routes[id=to-b]"]);
    }

    #[test]
    fn timing_applies_declared_tail_after_node_latency() {
        let nodes = [
            graph_node("source", 0, TailSamples::Finite(0)),
            graph_node("effect", 3, TailSamples::Finite(5)),
        ];
        let edges = [edge("serial", "source", "effect")];
        let (schedule, _) = topo(&nodes, &edges).expect("acyclic");
        let latencies = nodes
            .iter()
            .map(|node| (node.id.clone(), node.latency))
            .collect();
        let tails = nodes
            .iter()
            .map(|node| (node.id.clone(), node.tail))
            .collect();
        let error = timings(&schedule, &edges, &latencies, &tails, &caps(7))
            .err()
            .expect("latency plus tail exceeds cap");
        assert_eq!(error.code, "graph.tail.limit");
    }

    #[test]
    fn accepted_session_compiles_binds_and_renders_direct_route() {
        let mut model = parse_session_toml(SESSION_FIXTURE).expect("session fixture");
        model.tracks[0].dynamic.effects.clear();
        model.automation.clear();
        let compiled = compile_session(
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
        .expect("compiled session");
        let artifact = GraphCompiler::compile(GraphCompileRequest {
            plan_id: 123,
            effects: EffectPreparedSession {
                session: compiled,
                entries: Vec::new(),
            },
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("graph diagnostics: {:?}", failure.diagnostics));
        assert_eq!(artifact.report.estimate.routes, 1);
        assert_eq!(artifact.report.estimate.effects, 0);
        assert_eq!(artifact.report.estimate.reductions, 0);
        assert!(artifact.report.estimate.audio_buffer_samples > 0);
        assert!(artifact.report.estimate.graph_metadata_bytes > 0);
        assert!(artifact.report.estimate.incremental_plan_bytes > 0);
        assert_eq!(artifact.graph.required_bindings.len(), 5);
        let envelope = artifact.graph.envelope;
        let nodes = artifact
            .graph
            .required_bindings
            .iter()
            .cloned()
            .map(|node| {
                let processor: Box<dyn GraphRuntimeProcessor> = if matches!(
                    node,
                    GraphNodeId::TrackStage {
                        stage: TrackStage::Input,
                        ..
                    }
                ) {
                    Box::new(ImpulseBinding)
                } else {
                    Box::new(IdentityBinding)
                };
                GraphNodeBinding::new(node, processor)
            })
            .collect();
        let mut plan = match artifact
            .graph
            .bind(GraphRuntimeBindings { envelope, nodes })
        {
            Ok(plan) => plan,
            Err(failure) => panic!("bind: {}", failure.code),
        };
        let frames = envelope.quantum.0 as usize;
        let mut pcm = vec![0.0_f32; frames * 2];
        let output = PlanarBufferMut::try_new(&mut pcm, 2, frames, frames).expect("output");
        plan.render(
            RenderIo {
                input: None,
                output,
            },
            RenderTime { absolute_sample: 0 },
        )
        .expect("render");
        assert_eq!(pcm[0], 1.0);
        assert_eq!(pcm[frames], -1.0);
        assert!(pcm[1..frames].iter().all(|sample| *sample == 0.0));
        assert!(pcm[frames + 1..].iter().all(|sample| *sample == 0.0));
    }
}
