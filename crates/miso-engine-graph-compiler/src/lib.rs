//! Deterministic control-plane lowering of an accepted session and prepared native effects.
#![allow(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};

use miso_engine_builtins::BuiltinTail;
use miso_engine_builtins_compiler::{
    PreparedBuiltinsGraphArtifact, PreparedBuiltinsGraphBindFailure, PreparedBuiltinsGraphBound,
    PreparedBuiltinsSession,
};
use miso_engine_core::{realtime::RenderEnvelope, target_capabilities};
use miso_engine_effect_compiler::{EffectPreparedEntry, EffectPreparedSession, EffectRack};
use miso_engine_effect_contract::{
    LatencySamples, PrepareEffectBankRequest, PreparedSidechainPort, TailSamples,
};
use miso_engine_graph::{
    BufferAssignment, DependencyLevel, EffectNodeId, GraphCompileCaps, GraphDiagnostic,
    GraphDiagnosticSet, GraphEdge, GraphEdgeId, GraphNode, GraphNodeId, GraphPortId, GraphPortKind,
    GraphPreparedEffect, GraphResourceEstimate, GraphSpec, InsertedDelay, PreparedGraphPlan,
    PreparedGraphPlanParts, PreparedRoute, RackId, ReductionRecord, RouteTiming, RouteTransform,
    StableGraphId, TrackStage,
};
use miso_engine_rack::{KernelDispatch, RackLocationV1, RackProgramSignatureV1, RoutingClassV1};
use miso_engine_rack_compiler::{CompiledRackCohortsV1, RackTrackInputV1, compile_rack_cohorts_v1};
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
/// Compile a graph with internally prepared issue-007 processors and observers.
pub struct GraphBuiltinsCompileRequest {
    pub plan_id: u64,
    pub effects: EffectPreparedSession,
    pub builtins: PreparedBuiltinsSession,
    pub caps: GraphCompileCaps,
}
/// The one-way, sealed builtin attachment result.
///
/// ```compile_fail
/// use miso_engine_graph_compiler::PreparedGraphBuiltinsArtifact;
///
/// // The compiler-owned graph and builtin parts are private: external bindings cannot create
/// // a value carrying internal-builtin provenance.
/// let _ = PreparedGraphBuiltinsArtifact {};
/// ```
///
/// ```compile_fail
/// fn mutate(mut artifact: miso_engine_graph_compiler::PreparedGraphBuiltinsArtifact) {
///     artifact.graph = panic!("private provenance field");
/// }
/// ```
///
/// ```compile_fail
/// fn extract(artifact: miso_engine_graph_compiler::PreparedGraphBuiltinsArtifact) {
///     let miso_engine_graph_compiler::PreparedGraphBuiltinsArtifact { graph, .. } = artifact;
/// }
/// ```
///
/// ```compile_fail
/// fn clone_back(artifact: miso_engine_graph_compiler::PreparedGraphBuiltinsArtifact) {
///     let _ = artifact.clone();
/// }
/// ```
///
/// ```compile_fail
/// fn back_convert(artifact: miso_engine_graph_compiler::PreparedGraphBuiltinsArtifact) {
///     let _: miso_engine_graph::PreparedGraphPlan = artifact.into();
/// }
/// ```
///
/// ```compile_fail
/// fn generic_internal_attachment(plan: miso_engine_graph::PreparedGraphPlan) {
///     let _ = plan.attach_internal_bindings(Vec::new(), Vec::new());
/// }
/// ```
pub type PreparedGraphBuiltinsArtifact = PreparedBuiltinsGraphArtifact<GraphCompileReport>;
pub type PreparedGraphBuiltinsBound = PreparedBuiltinsGraphBound;
pub type GraphBuiltinsBindFailure = PreparedBuiltinsGraphBindFailure<GraphCompileReport>;
pub struct GraphBuiltinsCompileFailure {
    pub effects: EffectPreparedSession,
    pub builtins: PreparedBuiltinsSession,
    pub diagnostics: GraphDiagnosticSet,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GraphCompileReport {
    pub nodes: Vec<GraphNode>,
    pub ports: Vec<GraphPortId>,
    pub edges: Vec<GraphEdge>,
    pub sequential_schedule: Vec<GraphNodeId>,
    pub dependency_levels: Vec<DependencyLevel>,
    pub route_timings: Vec<RouteTiming>,
    pub inserted_delays: Vec<InsertedDelay>,
    pub reductions: Vec<ReductionRecord>,
    pub route_transforms: Vec<PreparedRoute>,
    pub buffer_assignments: Vec<BufferAssignment>,
    pub estimate: GraphResourceEstimate,
    pub canonical_debug_bytes: Vec<u8>,
    pub sha256: String,
    pub dot: String,
    /// Off-render SIMD-rack cohort decision. It is deliberately absent from graph identity,
    /// schedule, PDC and reductions: changing a host backend cannot change graph semantics.
    pub rack_cohorts: GraphRackCohortReport,
}

/// Stable rack cohorts for the two bankable graph boundaries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphRackCohortReport {
    pub dispatch: KernelDispatch,
    pub simd1: CompiledRackCohortsV1,
    pub simd2: CompiledRackCohortsV1,
}

impl GraphCompiler {
    #[allow(clippy::result_large_err)]
    pub fn compile_with_builtins(
        request: GraphBuiltinsCompileRequest,
    ) -> Result<PreparedGraphBuiltinsArtifact, GraphBuiltinsCompileFailure> {
        let GraphBuiltinsCompileRequest {
            plan_id,
            effects,
            builtins,
            caps,
        } = request;
        let builtin_diagnostics = builtins.validate_for_session(&effects.session);
        if !builtin_diagnostics.0.is_empty() {
            return Err(GraphBuiltinsCompileFailure {
                effects,
                builtins,
                diagnostics: GraphDiagnosticSet::sorted(
                    builtin_diagnostics
                        .0
                        .into_iter()
                        .map(|diagnostic| diag(diagnostic.code, &diagnostic.path))
                        .collect(),
                ),
            });
        }
        let builtin_tails: BTreeMap<_, _> = builtins
            .tails()
            .map(|(track_id, tail)| {
                (
                    track_id.to_owned(),
                    match tail {
                        BuiltinTail::FiniteZero => TailSamples::Finite(0),
                        BuiltinTail::Infinite => TailSamples::Infinite,
                    },
                )
            })
            .collect();
        let compiled = match Self::compile_with_builtin_tails(
            GraphCompileRequest {
                plan_id,
                effects,
                caps,
            },
            &builtin_tails,
        ) {
            Ok(value) => value,
            Err(failure) => {
                return Err(GraphBuiltinsCompileFailure {
                    effects: failure.effects,
                    builtins,
                    diagnostics: failure.diagnostics,
                });
            }
        };
        let dispatch = compiled.report.rack_cohorts.dispatch;
        let levels = compiled.report.dependency_levels.clone();
        Ok(builtins.into_graph_artifact_with_banks(
            compiled.graph,
            compiled.report,
            dispatch,
            &levels,
        ))
    }
    // The frozen transactional API returns the complete prepared-effect input by value on
    // failure. Boxing it would change that ownership contract solely to optimize a cold path.
    #[allow(clippy::result_large_err)]
    pub fn compile(
        request: GraphCompileRequest,
    ) -> Result<PreparedGraphArtifact, GraphCompileFailure> {
        Self::compile_with_builtin_tails(request, &BTreeMap::new())
    }
    #[allow(clippy::result_large_err)]
    fn compile_with_builtin_tails(
        request: GraphCompileRequest,
        builtin_tails: &BTreeMap<String, TailSamples>,
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
                let tail = if stage == TrackStage::PostInputBuiltins {
                    builtin_tails
                        .get(track.id.as_str())
                        .copied()
                        .unwrap_or(TailSamples::Finite(0))
                } else {
                    TailSamples::Finite(0)
                };
                add_node(
                    &mut nodes,
                    &mut node_latency,
                    &mut node_tail,
                    id,
                    LatencySamples(0),
                    tail,
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
        route_transforms.sort_by(|left, right| left.node.cmp(&right.node));
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        edges.sort_by(|a, b| a.id.cmp(&b.id));
        if nodes.len() as u64 > caps.maximum_nodes || edges.len() as u64 > caps.maximum_edges {
            diagnostics.push(diag("graph.resource.limit", "$.graph_compile_caps"));
        }
        for cycle in cycle_witnesses(&nodes, &edges) {
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
        let buffers = buffer_assignments(&schedule, &edges);
        let ports = ports_for(&nodes, &edges);
        let reductions = reduction_records(&nodes, &edges);
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
            ports: &ports,
            edges: &edges,
            schedule: &schedule,
            levels: &levels,
            routes: &timing.routes,
            route_transforms: &route_transforms,
            delays: &timing.delays,
            reductions: &reductions,
            buffers: &buffers,
            estimate: &estimate,
        });
        let sha256 = hex_sha256(&debug);
        let dot = dot(&nodes, &edges, &timing.delays);
        let rack_cohorts = rack_cohort_report(&effects);
        let banks = match bind_rack_banks(&effects, &effect_ids, &levels, rack_cohorts.dispatch) {
            Ok(value) => value,
            Err(diagnostic) => return Err(failure(effects, vec![diagnostic])),
        };
        let effect_nodes = into_effects(effects.entries, &effect_ids);
        let spec = GraphSpec {
            ports: ports.clone(),
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
            routes: route_transforms.clone(),
            effects: effect_nodes,
            banks,
            builtin_banks: Vec::new(),
            observers: Vec::new(),
        });
        Ok(PreparedGraphArtifact {
            graph,
            report: GraphCompileReport {
                nodes,
                ports,
                edges,
                sequential_schedule: schedule,
                dependency_levels: levels,
                route_timings: timing.routes,
                inserted_delays: timing.delays,
                reductions,
                route_transforms,
                buffer_assignments: buffers,
                estimate,
                canonical_debug_bytes: debug,
                sha256,
                dot,
                rack_cohorts,
            },
        })
    }
}

fn rack_cohort_report(effects: &EffectPreparedSession) -> GraphRackCohortReport {
    let dispatch = KernelDispatch::select(target_capabilities());
    let model = effects.session.normalized_model();
    let compile = |location: RackLocationV1, rack: RackId| {
        let tracks = model
            .tracks
            .iter()
            .map(|track| {
                let entries: Vec<_> = effects
                    .entries
                    .iter()
                    .filter(|entry| {
                        entry.track_id == track.id.as_str() && rack_id(entry.rack) == rack
                    })
                    .map(|entry| entry.metadata.program_key())
                    .collect();
                let routing = if effects.entries.iter().any(|entry| {
                    entry.track_id == track.id.as_str()
                        && rack_id(entry.rack) == rack
                        && matches!(
                            entry.metadata.ports.sidechain,
                            PreparedSidechainPort::Connected { .. }
                        )
                }) {
                    RoutingClassV1::SidechainConnected
                } else if effects.entries.iter().any(|entry| {
                    entry.track_id == track.id.as_str()
                        && rack_id(entry.rack) == rack
                        && matches!(
                            entry.metadata.ports.sidechain,
                            PreparedSidechainPort::Unconnected { .. }
                        )
                }) {
                    RoutingClassV1::SidechainUnconnected
                } else {
                    RoutingClassV1::MainOnly
                };
                RackTrackInputV1 {
                    track_id: track.id.as_str().into(),
                    signature: RackProgramSignatureV1::new(
                        location,
                        effects.session.sample_rate().0,
                        effects.session.quantum().0,
                        entries,
                        routing,
                    )
                    .expect("validated nonzero session envelope"),
                }
            })
            .collect();
        compile_rack_cohorts_v1(tracks, dispatch).expect("stable track IDs were session-validated")
    };
    GraphRackCohortReport {
        dispatch,
        simd1: compile(RackLocationV1::Simd1, RackId::Simd1),
        simd2: compile(RackLocationV1::Simd2, RackId::Simd2),
    }
}

fn bind_rack_banks(
    effects: &EffectPreparedSession,
    ids: &BTreeMap<(String, RackId, String), EffectNodeId>,
    levels: &[DependencyLevel],
    dispatch: KernelDispatch,
) -> Result<Vec<miso_engine_graph::GraphPreparedEffectBank>, GraphDiagnostic> {
    let Some(width) = dispatch.bank_width() else {
        return Ok(Vec::new());
    };
    let level_by_node: BTreeMap<_, _> = levels
        .iter()
        .flat_map(|level| {
            level
                .nodes
                .iter()
                .cloned()
                .map(move |node| (node, level.level))
        })
        .collect();
    let mut candidates = BTreeMap::new();
    for entry in &effects.entries {
        let rack = rack_id(entry.rack);
        if !matches!(rack, RackId::Simd1 | RackId::Simd2)
            || matches!(
                entry.metadata.ports.sidechain,
                PreparedSidechainPort::Connected { .. }
            )
        {
            continue;
        }
        candidates
            .entry((rack, entry.metadata.program_key()))
            .or_insert_with(Vec::new)
            .push(entry);
    }
    let mut banks = Vec::new();
    for ((rack, program), entries) in candidates {
        for members in entries.chunks(width.lanes() as usize) {
            if members.len() != width.lanes() as usize
                || members
                    .iter()
                    .any(|entry| !std::sync::Arc::ptr_eq(&entry.factory, &members[0].factory))
            {
                continue;
            }
            let member_ids: Vec<_> = members
                .iter()
                .map(|entry| ids[&(entry.track_id.clone(), rack, entry.effect_id.clone())].clone())
                .collect();
            let Some(first_level) = level_by_node
                .get(&GraphNodeId::Effect(member_ids[0].clone()))
                .copied()
            else {
                continue;
            };
            if member_ids.iter().any(|id| {
                level_by_node.get(&GraphNodeId::Effect(id.clone())).copied() != Some(first_level)
            }) {
                continue;
            }
            let requests: Vec<_> = members
                .iter()
                .map(|entry| entry.bank_preparation.request())
                .collect();
            let request = PrepareEffectBankRequest {
                backend: dispatch.backend(),
                width,
                requests: &requests,
            };
            let Some(processor) = members[0]
                .factory
                .bind_homogeneous_bank(request)
                .map_err(|error| diag(error.code, "$.effects"))?
            else {
                continue;
            };
            if processor.metadata().width != width || processor.metadata().program_key != program {
                return Err(diag("graph.effect.bank_metadata", "$.effects"));
            }
            let scratch = miso_engine_rack::AoSoaScratch::new(width, effects.session.quantum().0)
                .map_err(|_| diag("graph.resource.arithmetic_overflow", "$.graph"))?;
            banks.push(miso_engine_graph::GraphPreparedEffectBank {
                members: member_ids.into_boxed_slice(),
                processor,
                scratch,
            });
        }
    }
    Ok(banks)
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
    let mut input_counts: BTreeMap<_, u64> =
        nodes.iter().map(|node| (node.id.clone(), 0_u64)).collect();
    for edge in edges {
        let count = input_counts.get_mut(&edge.destination.node)?;
        *count = count.checked_add(1)?;
    }
    let reductions = count(
        nodes
            .iter()
            .filter(|node| {
                matches!(
                    node.id,
                    GraphNodeId::Submix { .. } | GraphNodeId::Output { .. }
                ) && input_counts[&node.id] > 1
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
    let maximum_inputs = input_counts.values().copied().max().unwrap_or(0);
    let quantum = u64::from(quantum);
    // Node outputs use the deterministic liveness coloring recorded in `buffers`. Edge
    // contributions remain distinct because they carry independent PDC state into reductions.
    let colored_outputs = buffers
        .iter()
        .map(|assignment| assignment.buffer_index)
        .max()
        .map_or(Some(0), |maximum| maximum.checked_add(1))?;
    let audio_buffer_samples = colored_outputs
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
        builtin_bank_bytes: 0,
        builtin_bank_scratch_bytes: 0,
        builtin_bank_count: 0,
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
    let mut incoming_by_node: BTreeMap<_, Vec<_>> = schedule
        .iter()
        .cloned()
        .map(|node| (node, Vec::new()))
        .collect();
    for edge in edges {
        incoming_by_node
            .get_mut(&edge.destination.node)
            .ok_or_else(|| diag("graph.internal.invariant", &edge.path))?
            .push(edge);
    }
    let mut arrivals = BTreeMap::<GraphNodeId, u64>::new();
    let mut extents = BTreeMap::<GraphNodeId, TailSamples>::new();
    let mut total_delay: u64 = 0;
    let mut delay_count: u64 = 0;
    let mut routes = Vec::new();
    let mut delays = Vec::new();
    for node in schedule {
        let incoming = &incoming_by_node[node];
        let max = incoming
            .iter()
            .filter_map(|edge| arrivals.get(&edge.source.node).copied())
            .max()
            .unwrap_or(0);
        for edge in incoming {
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
        for edge in incoming {
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
    let mut successors: BTreeMap<_, Vec<_>> = nodes
        .iter()
        .map(|node| (node.id.clone(), Vec::new()))
        .collect();
    let mut predecessors: BTreeMap<_, Vec<_>> = nodes
        .iter()
        .map(|node| (node.id.clone(), Vec::new()))
        .collect();
    for edge in edges {
        *degree.get_mut(&edge.destination.node)? += 1;
        successors
            .get_mut(&edge.source.node)?
            .push(edge.destination.node.clone());
        predecessors
            .get_mut(&edge.destination.node)?
            .push(edge.source.node.clone());
    }
    let mut ready: BTreeSet<_> = degree
        .iter()
        .filter_map(|(id, degree)| (*degree == 0).then_some(id.clone()))
        .collect();
    let mut schedule = Vec::new();
    let mut levels = BTreeMap::<u64, Vec<GraphNodeId>>::new();
    let mut node_levels = BTreeMap::new();
    while let Some(node) = ready.pop_first() {
        let level = predecessors[&node]
            .iter()
            .filter_map(|predecessor| node_levels.get(predecessor))
            .copied()
            .max()
            .map_or(0, |value| value + 1);
        node_levels.insert(node.clone(), level);
        levels.entry(level).or_default().push(node.clone());
        schedule.push(node.clone());
        for successor in &successors[&node] {
            let degree = degree.get_mut(successor)?;
            *degree -= 1;
            if *degree == 0 {
                ready.insert(successor.clone());
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
#[cfg(test)]
fn cycle_witness(
    nodes: &[GraphNode],
    edges: &[GraphEdge],
) -> Option<(Vec<GraphNodeId>, Vec<String>)> {
    cycle_witnesses(nodes, edges).into_iter().next()
}
fn cycle_witnesses(
    nodes: &[GraphNode],
    edges: &[GraphEdge],
) -> Vec<(Vec<GraphNodeId>, Vec<String>)> {
    let mut adjacency: BTreeMap<_, Vec<_>> = nodes
        .iter()
        .map(|node| (node.id.clone(), Vec::new()))
        .collect();
    let mut reverse: BTreeMap<_, Vec<_>> = nodes
        .iter()
        .map(|node| (node.id.clone(), Vec::new()))
        .collect();
    for edge in edges {
        let Some(outgoing) = adjacency.get_mut(&edge.source.node) else {
            return Vec::new();
        };
        outgoing.push(edge);
        let Some(incoming) = reverse.get_mut(&edge.destination.node) else {
            return Vec::new();
        };
        incoming.push(edge);
    }
    for outgoing in adjacency.values_mut() {
        outgoing.sort_by(|left, right| left.id.cmp(&right.id));
    }
    for incoming in reverse.values_mut() {
        incoming.sort_by(|left, right| left.id.cmp(&right.id));
    }

    let mut visited = BTreeSet::new();
    let mut finish = Vec::with_capacity(nodes.len());
    for start in nodes.iter().map(|node| &node.id) {
        if !visited.insert(start.clone()) {
            continue;
        }
        let mut stack = vec![(start.clone(), 0usize)];
        while let Some((node, next_edge)) = stack.last_mut() {
            let outgoing = &adjacency[node];
            if *next_edge == outgoing.len() {
                finish.push(stack.pop().expect("nonempty DFS stack").0);
                continue;
            }
            let destination = outgoing[*next_edge].destination.node.clone();
            *next_edge += 1;
            if visited.insert(destination.clone()) {
                stack.push((destination, 0));
            }
        }
    }

    let mut assigned = BTreeSet::new();
    let mut components = Vec::new();
    for start in finish.into_iter().rev() {
        if !assigned.insert(start.clone()) {
            continue;
        }
        let mut component = Vec::new();
        let mut stack = vec![start];
        while let Some(node) = stack.pop() {
            component.push(node.clone());
            for edge in reverse[&node].iter().rev() {
                let predecessor = edge.source.node.clone();
                if assigned.insert(predecessor.clone()) {
                    stack.push(predecessor);
                }
            }
        }
        component.sort();
        let cyclic = component.len() > 1
            || adjacency[&component[0]]
                .iter()
                .any(|edge| edge.destination.node == component[0]);
        if cyclic {
            components.push(component);
        }
    }
    components.sort_by(|left, right| left[0].cmp(&right[0]));
    components
        .into_iter()
        .filter_map(|component| cycle_witness_in_component(&component, &adjacency))
        .collect()
}
fn cycle_witness_in_component(
    component: &[GraphNodeId],
    adjacency: &BTreeMap<GraphNodeId, Vec<&GraphEdge>>,
) -> Option<(Vec<GraphNodeId>, Vec<String>)> {
    let members: BTreeSet<_> = component.iter().cloned().collect();
    let start = component.first()?;
    {
        let mut nodes_path = vec![start.clone()];
        let mut edge_path = Vec::new();
        let mut on_path = BTreeSet::from([start.clone()]);
        let mut stack = vec![(start.clone(), 0usize)];
        while let Some((node, next_edge)) = stack.last_mut() {
            let outgoing = &adjacency[node];
            while *next_edge < outgoing.len()
                && !members.contains(&outgoing[*next_edge].destination.node)
            {
                *next_edge += 1;
            }
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
fn buffer_assignments(schedule: &[GraphNodeId], edges: &[GraphEdge]) -> Vec<BufferAssignment> {
    let positions: BTreeMap<_, _> = schedule
        .iter()
        .cloned()
        .enumerate()
        .map(|(position, node)| (node, position))
        .collect();
    let mut consumer_counts = vec![0_usize; schedule.len()];
    let mut last_consumers: Vec<_> = schedule
        .iter()
        .enumerate()
        .map(|(position, node)| {
            if matches!(node, GraphNodeId::Output { .. }) {
                schedule.len()
            } else {
                position
            }
        })
        .collect();
    let mut main_input_counts = vec![0_usize; schedule.len()];
    let mut main_input_sources = vec![None; schedule.len()];
    for edge in edges {
        let source = positions[&edge.source.node];
        let destination = positions[&edge.destination.node];
        consumer_counts[source] += 1;
        last_consumers[source] = last_consumers[source].max(destination);
        if edge.destination.kind == GraphPortKind::MainInput {
            main_input_counts[destination] += 1;
            main_input_sources[destination] = Some(source);
        }
    }

    let mut next_buffer = 0_u64;
    let mut free = BTreeSet::new();
    let mut live_until = Vec::<usize>::new();
    let mut expirations = vec![Vec::<u64>::new(); schedule.len() + 1];
    let mut node_buffers = vec![0_u64; schedule.len()];
    let mut assignments = Vec::with_capacity(schedule.len());
    for (position, node) in schedule.iter().enumerate() {
        if position != 0 {
            for buffer in expirations[position - 1].drain(..) {
                if live_until[buffer as usize] == position - 1 {
                    free.insert(buffer);
                }
            }
        }

        let alias = is_identity_boundary(node)
            .then_some(position)
            .filter(|position| main_input_counts[*position] == 1)
            .and_then(|position| main_input_sources[position])
            .filter(|source| consumer_counts[*source] == 1)
            .map(|source| node_buffers[source]);
        let buffer_index = if let Some(buffer) = alias {
            free.remove(&buffer);
            buffer
        } else if let Some(buffer) = free.pop_first() {
            buffer
        } else {
            let buffer = next_buffer;
            next_buffer = next_buffer.checked_add(1).expect("node count fits u64");
            live_until.push(position);
            buffer
        };
        let last_consumer = last_consumers[position];
        live_until[buffer_index as usize] = last_consumer;
        expirations[last_consumer].push(buffer_index);
        node_buffers[position] = buffer_index;
        assignments.push(BufferAssignment {
            port: port(node.clone(), GraphPortKind::MainOutput),
            buffer_index,
        });
    }
    assignments
}

fn is_identity_boundary(node: &GraphNodeId) -> bool {
    matches!(
        node,
        GraphNodeId::TrackStage {
            stage: TrackStage::PostSimd1 | TrackStage::PostDynamic | TrackStage::PostSimd2PreFader,
            ..
        }
    )
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
fn reduction_records(nodes: &[GraphNode], edges: &[GraphEdge]) -> Vec<ReductionRecord> {
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
fn stage_token(stage: TrackStage) -> &'static str {
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
fn rack_token(rack: RackId) -> &'static str {
    match rack {
        RackId::Simd1 => "simd1",
        RackId::Dynamic => "dynamic",
        RackId::Simd2 => "simd2",
    }
}
fn node_text(node: &GraphNodeId) -> String {
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
fn node_kind_token(node: &GraphNodeId) -> &'static str {
    match node {
        GraphNodeId::TrackStage { .. } => "track-stage",
        GraphNodeId::Effect(_) => "effect",
        GraphNodeId::Route { .. } => "route",
        GraphNodeId::Submix { .. } => "submix",
        GraphNodeId::Output { .. } => "output",
        GraphNodeId::CompensationDelay { .. } => "compensation-delay",
    }
}
fn port_kind_token(kind: GraphPortKind) -> &'static str {
    match kind {
        GraphPortKind::MainInput => "main-input",
        GraphPortKind::MainOutput => "main-output",
        GraphPortKind::SidechainInput => "sidechain-input",
    }
}
fn port_text(port: &GraphPortId) -> String {
    format!(
        "{}:{}:{}",
        node_text(&port.node),
        port_kind_token(port.kind),
        port.effect_port.as_deref().unwrap_or("-")
    )
}
fn edge_text(edge: &GraphEdgeId) -> String {
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
fn tail_text(tail: TailSamples) -> String {
    match tail {
        TailSamples::Finite(samples) => format!("finite:{samples}"),
        TailSamples::Infinite => "infinite".to_owned(),
    }
}
struct CanonicalParts<'a> {
    rate: u32,
    quantum: u32,
    nodes: &'a [GraphNode],
    ports: &'a [GraphPortId],
    edges: &'a [GraphEdge],
    schedule: &'a [GraphNodeId],
    levels: &'a [DependencyLevel],
    routes: &'a [RouteTiming],
    route_transforms: &'a [PreparedRoute],
    delays: &'a [InsertedDelay],
    reductions: &'a [ReductionRecord],
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
            "node\t{}\t{}\t{}\t{}\n",
            node_text(&node.id),
            node_kind_token(&node.id),
            node.latency.0,
            tail_text(node.tail)
        ));
    }
    for port in parts.ports {
        text.push_str(&format!("port\t{}\n", port_text(port)));
    }
    for edge in parts.edges {
        text.push_str(&format!(
            "edge\t{}\t{}\t{}\t{}\n",
            edge_text(&edge.id),
            port_text(&edge.source),
            port_text(&edge.destination),
            edge.path
        ));
    }
    for delay in parts.delays {
        text.push_str(&format!(
            "delay\t{}\t{}\t{}\n",
            node_text(&delay.node),
            edge_text(&delay.edge_id),
            delay.samples.0
        ));
    }
    for (index, node) in parts.schedule.iter().enumerate() {
        text.push_str(&format!("order\t{index}\t{}\n", node_text(node)));
    }
    for level in parts.levels {
        for node in &level.nodes {
            text.push_str(&format!("level\t{}\t{}\n", level.level, node_text(node)));
        }
    }
    for reduction in parts.reductions {
        for (rank, edge) in reduction.contributions.iter().enumerate() {
            text.push_str(&format!(
                "reduction\t{}\t{rank}\t{}\n",
                node_text(&reduction.node),
                edge_text(edge)
            ));
        }
    }
    for route in parts.route_transforms {
        text.push_str(&format!(
            "route-transform\t{}\t{:08x}\t{:08x}\t{:08x}\t{:08x}\t{:08x}\n",
            node_text(&route.node),
            route.transform.gain.to_bits(),
            route.transform.ll.to_bits(),
            route.transform.lr.to_bits(),
            route.transform.rl.to_bits(),
            route.transform.rr.to_bits()
        ));
    }
    for route in parts.routes {
        text.push_str(&format!(
            "route-timing\t{}\t{}\t{}\t{}\n",
            route.route_id.as_str(),
            route.source_arrival.0,
            route.compensation_delay.0,
            route.destination_arrival.0
        ));
    }
    for node in parts.nodes {
        text.push_str(&format!(
            "tail\t{}\t{}\n",
            node_text(&node.id),
            tail_text(node.tail)
        ));
    }
    for buffer in parts.buffers {
        text.push_str(&format!(
            "buffer\t{}\t{}\n",
            port_text(&buffer.port),
            buffer.buffer_index
        ));
    }
    let estimate = parts.estimate;
    text.push_str(&format!(
        "estimate\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
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
        estimate.largest_allocation_bytes,
        estimate.incremental_plan_bytes,
        estimate.session_plus_plan_bytes
    ));
    text.into_bytes()
}
fn dot(nodes: &[GraphNode], edges: &[GraphEdge], delays: &[InsertedDelay]) -> String {
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
fn dot_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
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
    use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};
    use miso_engine_builtins::{MeterConfig, MeterHandle, MeterTap};
    use miso_engine_builtins_compiler::{
        BuiltinCompileCaps, MeterRequest, PreparedBuiltinsCorruption,
        PreparedBuiltinsCorruptionCase, prepare_session_builtins,
    };
    use miso_engine_conformance::DualAccumulatorDelayFactory;
    use miso_engine_core::{
        TargetCapabilities,
        realtime::{PlanarBufferMut, RenderIo, RenderTime, audit},
        target_capabilities,
    };
    use miso_engine_effect_compiler::{
        EffectCompileCaps, EffectPreparedSession, prepare_native_session_effects,
    };
    use miso_engine_effect_contract::{
        EffectPrepareError, NativeEffectFactory, NativeEffectRegistry, PrepareEffectBankRequest,
        PrepareEffectRequest, PreparedNativeEffect, PreparedNativeEffectBank,
    };
    use miso_engine_graph::{
        GraphBindingBlock, GraphNodeBinding, GraphNodeObserverBinding, GraphObservationBlock,
        GraphRuntimeBindings, GraphRuntimeObserver, GraphRuntimeProcessor,
    };
    use miso_engine_session::{
        CompileCaps, EffectIdentity, EffectParam, ParameterChannel, ParameterUnit, RouteSource,
        StableId, compile_session, parse_session_toml,
    };
    use std::sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    };

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

    struct AsymmetricTrackImpulseBinding {
        left: f32,
        right: f32,
    }
    impl GraphRuntimeProcessor for AsymmetricTrackImpulseBinding {
        fn process(
            &mut self,
            block: GraphBindingBlock<'_>,
        ) -> Result<(), miso_engine_core::realtime::RenderError> {
            block.left.fill(0.0);
            block.right.fill(0.0);
            block.left[0] = self.left;
            block.right[0] = self.right;
            Ok(())
        }
    }

    fn asymmetric_input_binding(node: &GraphNodeId) -> Box<dyn GraphRuntimeProcessor> {
        let GraphNodeId::TrackStage {
            track_id,
            stage: TrackStage::Input,
        } = node
        else {
            return Box::new(IdentityBinding);
        };
        let index = track_id
            .as_str()
            .strip_prefix("bank")
            .and_then(|value| value.parse::<u32>().ok())
            .expect("bank fixture track id");
        Box::new(AsymmetricTrackImpulseBinding {
            left: 0.125 * (index + 1) as f32,
            right: -0.0625 * 12_u32.saturating_sub(index) as f32,
        })
    }

    /// A deterministic factory failure used to prove the bank binder leaves its already prepared
    /// scalar ownership intact for the caller's transactional failure path.
    struct BankBindErrorFactory;
    impl NativeEffectFactory for BankBindErrorFactory {
        fn descriptor(&self) -> &'static miso_engine_effect_contract::EffectDescriptorV1 {
            DualAccumulatorDelayFactory::correct().descriptor()
        }
        fn prepare(
            &self,
            request: PrepareEffectRequest<'_>,
        ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
            DualAccumulatorDelayFactory::correct().prepare(request)
        }
        fn bind_homogeneous_bank(
            &self,
            _request: PrepareEffectBankRequest<'_>,
        ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
            Err(EffectPrepareError {
                code: "fixture.bank.bind_failure",
            })
        }
    }

    struct ScalarOnlyFactory;
    impl NativeEffectFactory for ScalarOnlyFactory {
        fn descriptor(&self) -> &'static miso_engine_effect_contract::EffectDescriptorV1 {
            DualAccumulatorDelayFactory::correct().descriptor()
        }
        fn prepare(
            &self,
            request: PrepareEffectRequest<'_>,
        ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
            DualAccumulatorDelayFactory::correct().prepare(request)
        }
        fn bind_homogeneous_bank(
            &self,
            _request: PrepareEffectBankRequest<'_>,
        ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
            Ok(None)
        }
    }

    struct OrderedPostBankObserver {
        expected_order: u64,
        order: Arc<AtomicU64>,
        observed_post_bank_audio: Arc<AtomicBool>,
    }
    impl GraphRuntimeObserver for OrderedPostBankObserver {
        fn observe(
            &mut self,
            block: GraphObservationBlock<'_>,
        ) -> Result<(), miso_engine_core::realtime::RenderError> {
            assert_eq!(
                self.order.fetch_add(1, Ordering::SeqCst),
                self.expected_order,
                "observers run in stable handle order"
            );
            self.observed_post_bank_audio.store(
                block.left.iter().any(|sample| *sample != 0.0)
                    && block.right.iter().any(|sample| *sample != 0.0),
                Ordering::SeqCst,
            );
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

    fn compile_fixture(plan_id: u64) -> PreparedGraphArtifact {
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
        GraphCompiler::compile(GraphCompileRequest {
            plan_id,
            effects: EffectPreparedSession {
                session: compiled,
                entries: Vec::new(),
            },
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("graph diagnostics: {:?}", failure.diagnostics))
    }

    #[test]
    fn mixed_twelve_track_plan_binds_renders_full_banks_and_scalar_tails_without_graph_changes() {
        let mut model = parse_session_toml(SESSION_FIXTURE).expect("fixture");
        let base_track = model.tracks[0].clone();
        let base_route = model.routes[0].clone();
        model.automation.clear();
        model.tracks = (0..12)
            .map(|index| {
                let mut track = base_track.clone();
                track.id = StableId::parse(&format!("bank{index}")).expect("id");
                track.dynamic.effects.clear();
                track.simd1.effects = base_track.dynamic.effects.clone();
                let effect = &mut track.simd1.effects[0];
                effect.id = StableId::parse("bank-delay").expect("id");
                effect.identity = EffectIdentity::Native {
                    effect_id: StableId::parse("conformance.delay").expect("id"),
                };
                effect.params = vec![EffectParam {
                    parameter_id: 1,
                    channel: ParameterChannel::Both,
                    unit: ParameterUnit::Linear,
                    value: 1.0 + index as f32 * 0.01,
                }];
                track
            })
            .collect();
        model.routes = model
            .tracks
            .iter()
            .enumerate()
            .map(|(index, track)| {
                let mut route = base_route.clone();
                route.id = StableId::parse(&format!("bank-route{index}")).expect("id");
                route.source = RouteSource::Track {
                    track_id: track.id.clone(),
                    tap: SendTap::PostMatrix,
                };
                route
            })
            .collect();
        let session = compile_session(
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
        .expect("compiled");
        let registry =
            NativeEffectRegistry::new([Box::new(DualAccumulatorDelayFactory::correct())
                as Box<dyn miso_engine_effect_contract::NativeEffectFactory>])
            .expect("registry");
        let effects = prepare_native_session_effects(
            &session,
            &registry,
            EffectCompileCaps {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 20,
                maximum_automation_spans_per_block: 32,
            },
        )
        .expect("effects");
        let artifact = GraphCompiler::compile(GraphCompileRequest {
            plan_id: 998,
            effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|_| panic!("graph"));
        let expected = KernelDispatch::select(target_capabilities())
            .bank_width()
            .map_or(0, |width| 12 / width.lanes() as usize);
        assert_eq!(artifact.graph.prepared_bank_count(), expected);
        let canonical = artifact.report.canonical_debug_bytes.clone();
        assert_eq!(canonical, artifact.report.canonical_debug_bytes);
        let bank_delays = artifact.report.inserted_delays.clone();
        let bank_tails: Vec<_> = artifact
            .report
            .nodes
            .iter()
            .map(|node| (node.id.clone(), node.tail))
            .collect();
        let envelope = artifact.graph.envelope;
        let nodes = artifact
            .graph
            .required_bindings
            .iter()
            .cloned()
            .map(|node| {
                let processor = asymmetric_input_binding(&node);
                GraphNodeBinding::new(node, processor)
            })
            .collect();
        let observer_order = Arc::new(AtomicU64::new(0));
        let observed_post_bank_audio = Arc::new(AtomicBool::new(false));
        let observed_stage = track_node("bank0", TrackStage::PostSimd1);
        let mut plan = artifact
            .graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes,
                // Reverse input order proves executor sorting by stable handle. The stage is only
                // reached after the bank's gather/process/scatter completion.
                observers: vec![
                    GraphNodeObserverBinding::new(
                        observed_stage.clone(),
                        2,
                        Box::new(OrderedPostBankObserver {
                            expected_order: 1,
                            order: Arc::clone(&observer_order),
                            observed_post_bank_audio: Arc::clone(&observed_post_bank_audio),
                        }),
                    ),
                    GraphNodeObserverBinding::new(
                        observed_stage,
                        1,
                        Box::new(OrderedPostBankObserver {
                            expected_order: 0,
                            order: Arc::clone(&observer_order),
                            observed_post_bank_audio: Arc::clone(&observed_post_bank_audio),
                        }),
                    ),
                ],
            })
            .unwrap_or_else(|failure| panic!("bind: {}", failure.code));
        let frames = envelope.quantum.0 as usize;
        let mut pcm = vec![0.0_f32; frames * 2];
        plan.render(
            RenderIo {
                input: None,
                output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames).expect("output"),
            },
            RenderTime { absolute_sample: 0 },
        )
        .expect("render full bank/tail graph");
        assert!(pcm.iter().any(|sample| *sample != 0.0));
        assert_eq!(observer_order.load(Ordering::SeqCst), 2);
        assert!(observed_post_bank_audio.load(Ordering::SeqCst));

        let scalar_registry =
            NativeEffectRegistry::new(
                [Box::new(ScalarOnlyFactory) as Box<dyn NativeEffectFactory>],
            )
            .expect("scalar registry");
        let scalar_effects = prepare_native_session_effects(
            &session,
            &scalar_registry,
            EffectCompileCaps {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 20,
                maximum_automation_spans_per_block: 32,
            },
        )
        .expect("scalar effects");
        let scalar_artifact = GraphCompiler::compile(GraphCompileRequest {
            plan_id: 999,
            effects: scalar_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("scalar graph: {:?}", failure.diagnostics));
        assert_eq!(scalar_artifact.report.canonical_debug_bytes, canonical);
        assert_eq!(scalar_artifact.report.inserted_delays, bank_delays);
        assert_eq!(
            scalar_artifact
                .report
                .nodes
                .iter()
                .map(|node| (node.id.clone(), node.tail))
                .collect::<Vec<_>>(),
            bank_tails
        );
        let scalar_envelope = scalar_artifact.graph.envelope;
        let scalar_nodes = scalar_artifact
            .graph
            .required_bindings
            .iter()
            .cloned()
            .map(|node| {
                let processor = asymmetric_input_binding(&node);
                GraphNodeBinding::new(node, processor)
            })
            .collect();
        let mut scalar_plan = scalar_artifact
            .graph
            .bind(GraphRuntimeBindings {
                envelope: scalar_envelope,
                nodes: scalar_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("scalar bind: {}", failure.code));
        let mut scalar_pcm = vec![0.0_f32; frames * 2];
        scalar_plan
            .render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(&mut scalar_pcm, 2, frames, frames)
                        .expect("scalar output"),
                },
                RenderTime { absolute_sample: 0 },
            )
            .expect("scalar render");
        let worst = pcm
            .iter()
            .zip(&scalar_pcm)
            .enumerate()
            .max_by(|(_, (bank_a, scalar_a)), (_, (bank_b, scalar_b))| {
                (*bank_a - *scalar_a)
                    .abs()
                    .total_cmp(&(*bank_b - *scalar_b).abs())
            })
            .expect("pcm");
        assert!(
            (worst.1.0 - worst.1.1).abs() <= 1.0e-6 + 2.0e-5 * worst.1.1.abs(),
            "worst output mismatch at {}: bank={} scalar={}",
            worst.0,
            worst.1.0,
            worst.1.1
        );

        // Host dispatch is deliberately detected only while preparing the normal artifact above.
        // These two direct, off-render binding probes exercise both legal factory widths on every
        // development host without pretending that a four-lane runtime was executed on x86.
        for dispatch in [
            KernelDispatch::select(TargetCapabilities::from_detected(true, false, false, false)),
            KernelDispatch::select(TargetCapabilities::from_detected(false, false, true, false)),
        ] {
            let rebound = prepare_native_session_effects(
                &session,
                &registry,
                EffectCompileCaps {
                    maximum_total_state_bytes: 1 << 20,
                    maximum_scratch_bytes: 1 << 20,
                    maximum_automation_spans_per_block: 32,
                },
            )
            .expect("reprepare effects");
            let ids = rebound
                .entries
                .iter()
                .map(|entry| {
                    (
                        (
                            entry.track_id.clone(),
                            rack_id(entry.rack),
                            entry.effect_id.clone(),
                        ),
                        EffectNodeId {
                            track_id: gid(&entry.track_id),
                            rack: rack_id(entry.rack),
                            effect_id: gid(&entry.effect_id),
                        },
                    )
                })
                .collect();
            let banks =
                bind_rack_banks(&rebound, &ids, &artifact.report.dependency_levels, dispatch)
                    .expect("off-render factory bind");
            assert_eq!(
                banks.len(),
                12 / dispatch.bank_width().expect("vector backend").lanes() as usize
            );
            assert!(banks.iter().all(|bank| {
                bank.members.len()
                    == dispatch.bank_width().expect("vector backend").lanes() as usize
            }));
        }

        let ids_for = |prepared: &EffectPreparedSession| {
            prepared
                .entries
                .iter()
                .map(|entry| {
                    (
                        (
                            entry.track_id.clone(),
                            rack_id(entry.rack),
                            entry.effect_id.clone(),
                        ),
                        EffectNodeId {
                            track_id: gid(&entry.track_id),
                            rack: rack_id(entry.rack),
                            effect_id: gid(&entry.effect_id),
                        },
                    )
                })
                .collect::<BTreeMap<_, _>>()
        };
        let eight =
            KernelDispatch::select(TargetCapabilities::from_detected(false, false, true, false));
        let mut connected_fallback = prepare_native_session_effects(
            &session,
            &registry,
            EffectCompileCaps {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 20,
                maximum_automation_spans_per_block: 32,
            },
        )
        .expect("reprepare connected fallback");
        connected_fallback.entries[0].metadata.ports.sidechain = PreparedSidechainPort::Connected {
            id: miso_engine_effect_contract::PortId::new("sidechain").expect("static port"),
            required: false,
        };
        let connected_ids = ids_for(&connected_fallback);
        let connected_banks = bind_rack_banks(
            &connected_fallback,
            &connected_ids,
            &artifact.report.dependency_levels,
            eight,
        )
        .expect("connected sidechain is scalar fallback, not failure");
        assert!(connected_banks.iter().all(|bank| {
            bank.members
                .iter()
                .all(|member| member.track_id.as_str() != "bank0")
        }));

        let same_wave = prepare_native_session_effects(
            &session,
            &registry,
            EffectCompileCaps {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 20,
                maximum_automation_spans_per_block: 32,
            },
        )
        .expect("reprepare same-wave fallback");
        let same_wave_ids = ids_for(&same_wave);
        let first = GraphNodeId::Effect(
            same_wave_ids[&("bank0".to_owned(), RackId::Simd1, "bank-delay".to_owned())].clone(),
        );
        let mut incompatible_levels = artifact.report.dependency_levels.clone();
        for level in &mut incompatible_levels {
            level.nodes.retain(|node| node != &first);
        }
        assert!(
            bind_rack_banks(&same_wave, &same_wave_ids, &incompatible_levels, eight)
                .expect("same-wave incompatibility is scalar fallback")
                .is_empty()
        );

        let rejecting_registry = NativeEffectRegistry::new([
            Box::new(BankBindErrorFactory) as Box<dyn NativeEffectFactory>
        ])
        .expect("registry");
        let rejected = prepare_native_session_effects(
            &session,
            &rejecting_registry,
            EffectCompileCaps {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 20,
                maximum_automation_spans_per_block: 32,
            },
        )
        .expect("prepare scalar ownership");
        let rejected_ids = ids_for(&rejected);
        let error = match bind_rack_banks(
            &rejected,
            &rejected_ids,
            &artifact.report.dependency_levels,
            eight,
        ) {
            Ok(_) => panic!("factory failure must reject transactionally"),
            Err(error) => error,
        };
        assert_eq!(error.code, "fixture.bank.bind_failure");
        assert_eq!(
            rejected.entries.len(),
            12,
            "factory failure retained every scalar input"
        );

        // The Issue-037 production audit is explicit-release-only. It intentionally binds the
        // sealed builtin artifact, rather than the old scalar fixture effect bank, and proves
        // that real TPT builtin-bank callbacks reached the prepared render plan.
        if std::env::var_os("MISO_ENGINE_ISSUE37_AUDIT").is_some() {
            let audit_effects = prepare_native_session_effects(
                &session,
                &registry,
                EffectCompileCaps {
                    maximum_total_state_bytes: 1 << 20,
                    maximum_scratch_bytes: 1 << 20,
                    maximum_automation_spans_per_block: 32,
                },
            )
            .expect("audit effects");
            let audit_builtins = prepare_session_builtins(
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
            .expect("audit builtins");
            let audit_artifact =
                GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
                    plan_id: 1_000,
                    effects: audit_effects,
                    builtins: audit_builtins,
                    caps: integration_caps(),
                })
                .unwrap_or_else(|_| panic!("audit graph"));
            let audit_backend = KernelDispatch::select(target_capabilities());
            let expected_effect_banks = audit_backend
                .bank_width()
                .map_or(0, |width| 12 / width.lanes() as usize);
            let expected_scalar_tails = audit_backend
                .bank_width()
                .map_or(12, |width| 12 % width.lanes() as usize);
            let expected_builtin_banks = expected_effect_banks;
            assert_eq!(
                audit_artifact.prepared_builtin_bank_count(),
                expected_builtin_banks
            );
            assert_eq!(
                expected_builtin_banks
                    * audit_backend
                        .bank_width()
                        .map_or(0, |width| width.lanes() as usize)
                    + expected_scalar_tails,
                12
            );
            assert!(
                expected_builtin_banks != 0,
                "audit host needs a selected SIMD backend"
            );
            let audit_envelope = audit_artifact.envelope();
            let audit_nodes = audit_artifact
                .external_binding_nodes()
                .map(|node| GraphNodeBinding::new(node.clone(), asymmetric_input_binding(node)))
                .collect();
            let bound = audit_artifact
                .into_bound(GraphRuntimeBindings {
                    envelope: audit_envelope,
                    nodes: audit_nodes,
                    observers: Vec::new(),
                })
                .unwrap_or_else(|_| panic!("audit bind"));
            let mut audit_plan = bound.plan;
            let mut audit_pcm = vec![0.0_f32; frames * 2];
            let output_address = audit_pcm.as_ptr() as usize;
            audit::warm_up();
            audit::reset();
            let mut output_hash = 0xcbf2_9ce4_8422_2325_u64;
            for block in 0..100_000_u64 {
                audit_plan
                    .render(
                        RenderIo {
                            input: None,
                            output: PlanarBufferMut::try_new(&mut audit_pcm, 2, frames, frames)
                                .expect("audit output"),
                        },
                        RenderTime {
                            absolute_sample: block * frames as u64,
                        },
                    )
                    .expect("audit render");
                assert_eq!(audit_pcm.as_ptr() as usize, output_address);
                for sample in &audit_pcm {
                    output_hash ^= u64::from(sample.to_bits());
                    output_hash = output_hash.wrapping_mul(0x0000_0100_0000_01b3);
                }
            }
            let audit_snapshot = audit::snapshot();
            assert_eq!(audit_snapshot.total(), 0);
            let counters = audit_plan.qualification_counters();
            assert_eq!(
                counters[0],
                100_000_u64 * expected_builtin_banks as u64,
                "exact retained builtin-bank process callbacks"
            );
            assert_eq!(
                counters[1],
                counters[0] * u64::from(audit_envelope.quantum.0) * 4,
                "exact real HPF/LPF TPT kernel calls for independent L/R lanes"
            );
            assert_eq!(
                output_hash, 0x9f30_db02_2065_6d79,
                "deterministic mixed output hash"
            );
        }
    }

    #[test]
    fn builtins_replace_only_the_three_internal_track_bindings() {
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
        .expect("compiled");
        let builtins = prepare_session_builtins(
            &compiled,
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
            plan_id: 77,
            effects: EffectPreparedSession {
                session: compiled,
                entries: Vec::new(),
            },
            builtins,
            caps: integration_caps(),
        })
        .unwrap_or_else(|_| panic!("graph"));
        assert_eq!(artifact.external_binding_nodes().count(), 2);
        let tail = artifact
            .report()
            .nodes
            .iter()
            .find(|node| node.id == track_node("vocal", TrackStage::PostInputBuiltins))
            .expect("input builtins node")
            .tail;
        assert_eq!(tail, TailSamples::Infinite);
    }

    #[test]
    fn production_builtin_banks_replace_full_post_input_groups_and_render() {
        let mut model = parse_session_toml(SESSION_FIXTURE).expect("session fixture");
        let base_track = model.tracks[0].clone();
        let base_route = model.routes[0].clone();
        model.automation.clear();
        model.tracks = (0..8)
            .map(|index| {
                let mut track = base_track.clone();
                track.id = StableId::parse(&format!("bank{index}")).expect("id");
                track.simd1.effects.clear();
                track.dynamic.effects.clear();
                track.simd2.effects.clear();
                track
            })
            .collect();
        model.routes = model
            .tracks
            .iter()
            .enumerate()
            .map(|(index, track)| {
                let mut route = base_route.clone();
                route.id = StableId::parse(&format!("builtin-route-{index}")).expect("route id");
                route.source = RouteSource::Track {
                    track_id: track.id.clone(),
                    tap: SendTap::PostMatrix,
                };
                route
            })
            .collect();
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
        .expect("compiled");
        let builtins = prepare_session_builtins(
            &compiled,
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
        let artifact = match GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
            plan_id: 78,
            effects: EffectPreparedSession {
                session: compiled,
                entries: Vec::new(),
            },
            builtins,
            caps: integration_caps(),
        }) {
            Ok(artifact) => artifact,
            Err(_) => panic!("graph"),
        };
        let expected_banks = KernelDispatch::select(target_capabilities())
            .bank_width()
            .map_or(0, |width| 8 / width.lanes() as usize);
        assert_eq!(artifact.prepared_builtin_bank_count(), expected_banks);
        let resource = artifact.graph_resource_estimate();
        assert_eq!(resource.builtin_bank_count, expected_banks as u64);
        if expected_banks != 0 {
            assert!(resource.builtin_bank_bytes != 0);
            assert!(resource.builtin_bank_scratch_bytes != 0);
        }
        let envelope = artifact.envelope();
        let nodes = artifact
            .external_binding_nodes()
            .cloned()
            .map(|node| {
                let processor = match node {
                    GraphNodeId::TrackStage {
                        stage: TrackStage::Input,
                        ..
                    } => asymmetric_input_binding(&node),
                    _ => Box::new(IdentityBinding) as Box<dyn GraphRuntimeProcessor>,
                };
                GraphNodeBinding::new(node, processor)
            })
            .collect();
        let bound = match artifact.into_bound(GraphRuntimeBindings {
            envelope,
            nodes,
            observers: Vec::new(),
        }) {
            Ok(bound) => bound,
            Err(_) => panic!("sealed builtin bank bind"),
        };
        let mut plan = bound.plan;
        let frames = envelope.quantum.0 as usize;
        let mut pcm = vec![0.0; frames * 2];
        plan.render(
            RenderIo {
                input: None,
                output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames).expect("output"),
            },
            RenderTime { absolute_sample: 0 },
        )
        .expect("production builtin-bank render");
        assert!(pcm.iter().any(|sample| *sample != 0.0));
    }

    #[test]
    fn frozen_issue_037_seeded_builtin_bank_layouts_have_exact_membership_and_counters() {
        const SEED: u64 = 0x0000_0000_8a05_0a08;
        const COUNTS: [usize; 9] = [1, 2, 3, 4, 5, 7, 8, 9, 17];
        let mut state = SEED;
        let mut transcript = 0xcbf2_9ce4_8422_2325_u64;
        let mut completed = 0_u32;
        for layout in 0..100_u32 {
            // SplitMix64, frozen locally so this suite has no dependency on host RNG state.
            state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut value = state;
            value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            value ^= value >> 31;
            let count = COUNTS[layout as usize % COUNTS.len()];
            let mut model = parse_session_toml(SESSION_FIXTURE).expect("fixture");
            let base_track = model.tracks[0].clone();
            let base_route = model.routes[0].clone();
            model.automation.clear();
            model.tracks = (0..count)
                .map(|index| {
                    let mut track = base_track.clone();
                    track.id = StableId::parse(&format!("bank{index}")).expect("id");
                    track.simd1.effects.clear();
                    track.dynamic.effects.clear();
                    track.simd2.effects.clear();
                    // The seeded corpus includes identity filters, enabled filters, and
                    // intentionally asymmetric L/R coefficients without changing topology.
                    if ((value >> (index % 31)) & 1) != 0 {
                        track.builtins.left.hpf_hz = 0.0;
                    }
                    if ((value >> ((index + 7) % 31)) & 1) != 0 {
                        track.builtins.right.lpf_hz = 0.0;
                    }
                    if ((value >> ((index + 13) % 31)) & 1) != 0 {
                        track.builtins.right.polarity_invert =
                            !track.builtins.right.polarity_invert;
                    }
                    track
                })
                .collect();
            model.routes = model
                .tracks
                .iter()
                .enumerate()
                .map(|(index, track)| {
                    let mut route = base_route.clone();
                    route.id =
                        StableId::parse(&format!("seed-route-{layout}-{index}")).expect("route id");
                    route.source = RouteSource::Track {
                        track_id: track.id.clone(),
                        tap: SendTap::PostMatrix,
                    };
                    route
                })
                .collect();
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
            .expect("compiled seeded layout");
            let builtins = prepare_session_builtins(
                &compiled,
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
            .expect("prepared seeded builtins");
            let artifact = match GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
                plan_id: u64::from(layout) + 50_000,
                effects: EffectPreparedSession {
                    session: compiled,
                    entries: Vec::new(),
                },
                builtins,
                caps: integration_caps(),
            }) {
                Ok(artifact) => artifact,
                Err(_) => panic!("seeded graph"),
            };
            let width = KernelDispatch::select(target_capabilities()).bank_width();
            let expected_banks = width.map_or(0, |width| count / width.lanes() as usize);
            let expected_tail = width.map_or(count, |width| count % width.lanes() as usize);
            assert_eq!(artifact.prepared_builtin_bank_count(), expected_banks);
            let envelope = artifact.envelope();
            let nodes = artifact
                .external_binding_nodes()
                .cloned()
                .map(|node| {
                    let processor = match node {
                        GraphNodeId::TrackStage {
                            stage: TrackStage::Input,
                            ..
                        } => asymmetric_input_binding(&node),
                        _ => Box::new(IdentityBinding) as Box<dyn GraphRuntimeProcessor>,
                    };
                    GraphNodeBinding::new(node, processor)
                })
                .collect();
            let mut plan = match artifact.into_bound(GraphRuntimeBindings {
                envelope,
                nodes,
                observers: Vec::new(),
            }) {
                Ok(bound) => bound.plan,
                Err(_) => panic!("seeded bind"),
            };
            let frames = envelope.quantum.0 as usize;
            let mut pcm = vec![0.0; frames * 2];
            plan.render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames)
                        .expect("seeded output"),
                },
                RenderTime { absolute_sample: 0 },
            )
            .expect("seeded render");
            let counters = plan.qualification_counters();
            assert_eq!(counters[0], expected_banks as u64);
            assert_eq!(counters[1], counters[0] * u64::from(envelope.quantum.0) * 4);
            let pcm_hash = pcm.iter().fold(0xcbf2_9ce4_8422_2325_u64, |hash, sample| {
                (hash ^ u64::from(sample.to_bits())).wrapping_mul(0x0000_0100_0000_01b3)
            });
            for byte in format!(
                "{layout}:{value:016x}:{count}:{expected_banks}:{expected_tail}:{pcm_hash:016x}:{:?}",
                counters
            )
            .bytes()
            {
                transcript ^= u64::from(byte);
                transcript = transcript.wrapping_mul(0x0000_0100_0000_01b3);
            }
            completed += 1;
        }
        assert_eq!(completed, 100);
        assert_eq!(
            transcript, 0xc85b_2209_8007_7824,
            "frozen Issue-037 seeded layout transcript"
        );
    }

    #[test]
    fn each_forged_builtin_seal_tuple_is_rejected_before_graph_attachment() {
        let cases = [
            (
                PreparedBuiltinsCorruptionCase::SessionHash,
                "builtin.session.mismatch",
            ),
            (
                PreparedBuiltinsCorruptionCase::SessionRate,
                "builtin.session.mismatch",
            ),
            (
                PreparedBuiltinsCorruptionCase::SessionQuantum,
                "builtin.session.mismatch",
            ),
            (
                PreparedBuiltinsCorruptionCase::TrackMissing,
                "builtin.prepared.track_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::TrackExtra,
                "builtin.prepared.track_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::TrackDuplicate,
                "builtin.prepared.track_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ProcessorMissing,
                "builtin.prepared.processor_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ProcessorExtra,
                "builtin.prepared.processor_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ProcessorChangedStage,
                "builtin.prepared.processor_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::TailMissing,
                "builtin.prepared.tail_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::TailExtra,
                "builtin.prepared.tail_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::TailChanged,
                "builtin.prepared.tail_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::RequestMissing,
                "builtin.prepared.request_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::RequestExtra,
                "builtin.prepared.request_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::RequestDuplicate,
                "builtin.prepared.request_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ObserverMissing,
                "builtin.prepared.observer_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ObserverExtra,
                "builtin.prepared.observer_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ObserverChangedNode,
                "builtin.prepared.observer_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ConsumerMissing,
                "builtin.prepared.consumer_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ConsumerExtra,
                "builtin.prepared.consumer_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ConsumerChangedMetadata,
                "builtin.prepared.consumer_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ConsumerDuplicateHandle,
                "builtin.prepared.consumer_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ResourceReport,
                "builtin.prepared.resource_report",
            ),
        ];
        let mut categories = BTreeSet::new();
        for (corruption, expected) in cases {
            categories.insert(corruption.category());
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
            .expect("compiled");
            let mut builtins = prepare_session_builtins(
                &compiled,
                &[
                    MeterRequest {
                        handle: MeterHandle(NonZeroU64::new(10).expect("constant")),
                        track_id: "vocal".to_owned(),
                        tap: MeterTap::Input,
                        config: MeterConfig {
                            period_frames: NonZeroU32::new(16).expect("constant"),
                            peak_hold_frames: 0,
                            peak_decay_db_per_second: 0.0,
                            queue_capacity: NonZeroUsize::new(4).expect("constant"),
                            reset_generation: 10,
                        },
                    },
                    MeterRequest {
                        handle: MeterHandle(NonZeroU64::new(11).expect("constant")),
                        track_id: "vocal".to_owned(),
                        tap: MeterTap::PostMatrix,
                        config: MeterConfig {
                            period_frames: NonZeroU32::new(32).expect("constant"),
                            peak_hold_frames: 4,
                            peak_decay_db_per_second: 12.0,
                            queue_capacity: NonZeroUsize::new(4).expect("constant"),
                            reset_generation: 11,
                        },
                    },
                ],
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
            builtins.test_only_corrupt_for_compiler_test(corruption);
            let Err(failure) = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
                plan_id: 78,
                effects: EffectPreparedSession {
                    session: compiled,
                    entries: Vec::new(),
                },
                builtins,
                caps: integration_caps(),
            }) else {
                panic!("forged builtin artifact must reject: {corruption:?}");
            };
            assert_eq!(
                failure
                    .diagnostics
                    .diagnostics()
                    .iter()
                    .map(|diagnostic| diagnostic.code)
                    .collect::<Vec<_>>(),
                vec![expected]
            );
            // Rejection is transactional: the compiler returns both inputs rather than consuming
            // either one into graph bindings.
            assert_eq!(failure.effects.entries.len(), 0);
            assert!(failure.builtins.processor_count() <= 3);
        }
        assert_eq!(
            categories,
            BTreeSet::from([
                PreparedBuiltinsCorruption::SessionIdentity,
                PreparedBuiltinsCorruption::Tracks,
                PreparedBuiltinsCorruption::Processors,
                PreparedBuiltinsCorruption::Tails,
                PreparedBuiltinsCorruption::Requests,
                PreparedBuiltinsCorruption::Observers,
                PreparedBuiltinsCorruption::Consumers,
                PreparedBuiltinsCorruption::Resources,
            ])
        );
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
    fn every_cyclic_scc_has_one_closed_sorted_witness_and_edge_paths() {
        let nodes: Vec<_> = ["a", "b", "c", "d", "e", "z"]
            .into_iter()
            .map(|name| graph_node(name, 0, TailSamples::Finite(0)))
            .collect();
        let mut edges = vec![
            edge("ab", "a", "b"),
            edge("ba", "b", "a"),
            edge("cc", "c", "c"),
            edge("de", "d", "e"),
            edge("ed", "e", "d"),
            edge("za", "a", "z"),
        ];
        edges.sort_by(|left, right| left.id.cmp(&right.id));
        let witnesses = cycle_witnesses(&nodes, &edges);
        assert_eq!(witnesses.len(), 3);
        assert_eq!(witnesses[0].0, [node("a"), node("b"), node("a")]);
        assert_eq!(witnesses[0].1, ["$.routes[id=ab]", "$.routes[id=ba]"]);
        assert_eq!(witnesses[1].0, [node("c"), node("c")]);
        assert_eq!(witnesses[1].1, ["$.routes[id=cc]"]);
        assert_eq!(witnesses[2].0, [node("d"), node("e"), node("d")]);
        assert_eq!(witnesses[2].1, ["$.routes[id=de]", "$.routes[id=ed]"]);
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
    fn buffer_coloring_aliases_identity_and_preserves_fanout_liveness() {
        let source = track_node("track", TrackStage::Input);
        let identity = track_node("track", TrackStage::PostSimd1);
        let route_a = GraphNodeId::Route { route_id: gid("a") };
        let route_b = GraphNodeId::Route { route_id: gid("b") };
        let output = GraphNodeId::Output {
            output_id: gid("main"),
        };
        let schedule = vec![
            source.clone(),
            identity.clone(),
            route_a.clone(),
            route_b.clone(),
            output.clone(),
        ];
        let make_edge = |id, source, destination| GraphEdge {
            id,
            source: port(source, GraphPortKind::MainOutput),
            destination: port(destination, GraphPortKind::MainInput),
            path: "$.coloring".to_owned(),
        };
        let edges = vec![
            make_edge(
                GraphEdgeId::TrackMain {
                    target: identity.clone(),
                },
                source,
                identity.clone(),
            ),
            make_edge(
                GraphEdgeId::RouteSource { route_id: gid("a") },
                identity.clone(),
                route_a.clone(),
            ),
            make_edge(
                GraphEdgeId::RouteSource { route_id: gid("b") },
                identity.clone(),
                route_b.clone(),
            ),
            make_edge(
                GraphEdgeId::RouteDestination { route_id: gid("a") },
                route_a.clone(),
                output.clone(),
            ),
            make_edge(
                GraphEdgeId::RouteDestination { route_id: gid("b") },
                route_b.clone(),
                output.clone(),
            ),
        ];
        let assigned: BTreeMap<_, _> = buffer_assignments(&schedule, &edges)
            .into_iter()
            .map(|assignment| (assignment.port.node, assignment.buffer_index))
            .collect();
        assert_eq!(assigned[&identity], 0);
        assert_eq!(assigned[&route_a], 1);
        assert_eq!(assigned[&route_b], 2);
        assert_eq!(assigned[&output], 0);
    }

    #[test]
    fn accepted_session_compiles_binds_and_renders_direct_route() {
        let artifact = compile_fixture(123);
        assert_eq!(artifact.report.estimate.routes, 1);
        assert_eq!(artifact.report.estimate.effects, 0);
        assert_eq!(artifact.report.estimate.reductions, 0);
        assert!(artifact.report.estimate.audio_buffer_samples > 0);
        assert!(artifact.report.estimate.graph_metadata_bytes > 0);
        assert!(artifact.report.estimate.incremental_plan_bytes > 0);
        let assigned: BTreeMap<_, _> = artifact
            .report
            .buffer_assignments
            .iter()
            .map(|assignment| (assignment.port.node.clone(), assignment.buffer_index))
            .collect();
        let track = |stage| track_node("vocal", stage);
        assert_eq!(
            assigned[&track(TrackStage::PostInputBuiltins)],
            assigned[&track(TrackStage::PostSimd1)]
        );
        assert_eq!(
            assigned[&track(TrackStage::PostSimd1)],
            assigned[&track(TrackStage::PostDynamic)]
        );
        assert_eq!(
            assigned[&track(TrackStage::PostDynamic)],
            assigned[&track(TrackStage::PostSimd2PreFader)]
        );
        let colored_buffer_count = assigned.values().copied().max().expect("buffers") + 1;
        assert_eq!(colored_buffer_count, 2);
        assert!(colored_buffer_count < artifact.report.estimate.logical_nodes);
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
        let mut plan = match artifact.graph.bind(GraphRuntimeBindings {
            envelope,
            nodes,
            observers: Vec::new(),
        }) {
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

    #[test]
    fn canonical_artifacts_are_complete_and_repeatable_100_times() {
        let baseline = compile_fixture(0).report;
        let canonical = core::str::from_utf8(&baseline.canonical_debug_bytes).expect("UTF-8");
        for section in [
            "envelope\t",
            "node\t",
            "port\t",
            "edge\t",
            "order\t",
            "level\t",
            "route-transform\t",
            "route-timing\t",
            "tail\t",
            "buffer\t",
            "estimate\t",
        ] {
            assert!(canonical.contains(section), "missing {section}");
        }
        assert!(!canonical.contains("Simd"));
        assert!(!canonical.contains("Finite"));
        assert!(
            baseline
                .sha256
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        );
        assert_eq!(baseline.sha256.len(), 64);
        assert!(baseline.dot.ends_with("}\n"));
        for plan_id in 1..=100 {
            let candidate = compile_fixture(plan_id).report;
            assert_eq!(
                candidate.canonical_debug_bytes,
                baseline.canonical_debug_bytes
            );
            assert_eq!(candidate.sha256, baseline.sha256);
            assert_eq!(candidate.sequential_schedule, baseline.sequential_schedule);
            assert_eq!(candidate.dependency_levels, baseline.dependency_levels);
            assert_eq!(candidate.route_timings, baseline.route_timings);
            assert_eq!(candidate.buffer_assignments, baseline.buffer_assignments);
            assert_eq!(candidate.dot, baseline.dot);
        }
    }

    #[test]
    fn route_transform_bits_participate_in_semantic_hash() {
        let baseline = compile_fixture(1).report;
        let mut model = parse_session_toml(SESSION_FIXTURE).expect("session fixture");
        model.tracks[0].dynamic.effects.clear();
        model.automation.clear();
        model.routes[0].gain_db = -6.0;
        let session = compile_session(
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
        .expect("session");
        let changed = GraphCompiler::compile(GraphCompileRequest {
            plan_id: 1,
            effects: EffectPreparedSession {
                session,
                entries: Vec::new(),
            },
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("graph diagnostics: {:?}", failure.diagnostics));
        assert_ne!(changed.report.sha256, baseline.sha256);
        assert_ne!(
            changed.report.canonical_debug_bytes,
            baseline.canonical_debug_bytes
        );
    }

    #[test]
    fn ten_thousand_graph_mutations_are_panic_free_and_repeatable() {
        let mut state = 0x6d69_736f_6d75_7461_u64;
        for mutation in 0..10_000_u32 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let node_count = (state as usize % 8) + 1;
            let nodes: Vec<_> = (0..node_count)
                .map(|index| {
                    graph_node(
                        &format!("n{index}"),
                        (state >> (index % 16)) & 7,
                        TailSamples::Finite((state >> ((index + 3) % 16)) & 7),
                    )
                })
                .collect();
            let mut edges = Vec::new();
            for edge_index in 0..node_count.saturating_mul(2) {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                let source = state as usize % node_count;
                let destination = (state >> 11) as usize % node_count;
                edges.push(edge(
                    &format!("m{mutation}-{edge_index}"),
                    &format!("n{source}"),
                    &format!("n{destination}"),
                ));
            }
            edges.sort_by(|left, right| left.id.cmp(&right.id));
            let first_cycle = cycle_witness(&nodes, &edges);
            let second_cycle = cycle_witness(&nodes, &edges);
            assert_eq!(first_cycle, second_cycle);
            if first_cycle.is_none() {
                let (first_schedule, first_levels) = topo(&nodes, &edges).expect("acyclic");
                let (second_schedule, second_levels) = topo(&nodes, &edges).expect("repeat");
                assert_eq!(first_schedule, second_schedule);
                assert_eq!(first_levels, second_levels);
                let latencies = nodes
                    .iter()
                    .map(|node| (node.id.clone(), node.latency))
                    .collect();
                let tails = nodes
                    .iter()
                    .map(|node| (node.id.clone(), node.tail))
                    .collect();
                let first = timings(
                    &first_schedule,
                    &edges,
                    &latencies,
                    &tails,
                    &caps(1_000_000),
                );
                let second = timings(
                    &second_schedule,
                    &edges,
                    &latencies,
                    &tails,
                    &caps(1_000_000),
                );
                assert_eq!(
                    first
                        .as_ref()
                        .map(|result| (&result.routes, &result.delays)),
                    second
                        .as_ref()
                        .map(|result| (&result.routes, &result.delays))
                );
                assert_eq!(first.err(), second.err());
            }
        }
    }
}
