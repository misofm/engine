//! The compile entry points and their orchestration.
//!
//! `compile_with_builtin_tails` is the whole pipeline: validate the prepared effects against the
//! session model, materialise nodes and edges, detect cycles, order and level the graph, resolve
//! PDC, colour output buffers, plan and bind the SIMD-rack banks, and check the resource estimate
//! against the caps -- transactionally, handing every caller-owned input back on any failure.
//!
//! Evidence -- the canonical text, its SHA-256 and the Graphviz rendering -- is **not** produced
//! here (#99 F5); see [`GraphCompiler::evidence`].

use super::*;
use crate::banks::{bind_rack_banks, checked_add_effect_banks, effect_bank_resource};
use crate::canonical::{
    Sha256Writer, canonical_parts, dot, hex_digest, hex_sha256, reductions_of, write_canonical,
};
use crate::estimate::{estimate_fits_platform, resource_estimate};
use crate::ids::{
    add_main_edge, add_node, add_route_destination_edge, add_route_source_edge, diag, effect_path,
    failure, gid, into_effects, port, ports_for, rack_id, route_destination_node,
    route_source_node, route_transform, sidechain_matches, stages, track_node,
};
use crate::pdc::timings;
use crate::schedule::{buffer_assignments, cycle_witnesses, topo};

impl GraphCompiler {
    /// The canonical text, its SHA-256 and the Graphviz rendering, produced on demand.
    ///
    /// #99 F5: never on the compile path. `report` supplies the pre-bank `semantic_estimate`;
    /// everything else is read straight off the finished plan, so the evidence cannot disagree
    /// with what was compiled.
    #[must_use]
    pub fn evidence(graph: &PreparedGraphPlan, report: &GraphCompileReport) -> GraphEvidence {
        let mut text = String::new();
        write_canonical(
            &mut text,
            canonical_parts(graph, report, &reductions_of(graph)),
        );
        let sha256 = hex_sha256(text.as_bytes());
        GraphEvidence {
            canonical_bytes: text.into_bytes(),
            sha256,
            dot: dot(&graph.spec.nodes, &graph.spec.edges, &graph.inserted_delays),
        }
    }

    /// The semantic SHA-256 alone, hashed straight through without materialising the text.
    ///
    /// This is the cheap path for the determinism gates and the fixture checker, which compare
    /// hashes and never look at the dump.
    #[must_use]
    pub fn sha256(graph: &PreparedGraphPlan, report: &GraphCompileReport) -> String {
        let mut hasher = Sha256Writer(Sha256::new());
        write_canonical(
            &mut hasher,
            canonical_parts(graph, report, &reductions_of(graph)),
        );
        hex_digest(&hasher.0.finalize())
    }

    /// The reductions the canonical text records, recomputed from the plan's own spec.
    #[must_use]
    pub fn reductions(graph: &PreparedGraphPlan) -> Vec<ReductionRecord> {
        reductions_of(graph)
    }

    #[allow(clippy::result_large_err)]
    pub fn compile_with_builtins(
        request: GraphBuiltinsCompileRequest,
    ) -> Result<PreparedGraphBuiltinsArtifact, GraphBuiltinsCompileFailure> {
        let GraphBuiltinsCompileRequest {
            plan_id,
            effects,
            builtins,
            caps,
            dispatch,
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
                dispatch,
            },
            &builtin_tails,
            Some(&builtins),
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
        let levels = compiled.graph.dependency_levels.clone();
        Ok(builtins.into_graph_artifact_with_banks(
            compiled.graph,
            compiled.report,
            dispatch,
            &levels,
            // The *same* object `bind_rack_banks` was handed inside the compile above. This is
            // the whole of the two-planner agreement mechanism (`SessionPoolClasses`): there
            // is one derivation and both planners read it, so they cannot form different
            // opinions about a track and silently decline the strip's chain merges.
            &compiled.pool_classes,
        ))
    }
    // The frozen transactional API returns the complete prepared-effect input by value on
    // failure. Boxing it would change that ownership contract solely to optimize a cold path.
    #[allow(clippy::result_large_err)]
    pub fn compile(
        request: GraphCompileRequest,
    ) -> Result<PreparedGraphArtifact, GraphCompileFailure> {
        Self::compile_with_builtin_tails(request, &BTreeMap::new(), None)
    }
    #[allow(clippy::result_large_err)]
    fn compile_with_builtin_tails(
        request: GraphCompileRequest,
        builtin_tails: &BTreeMap<String, TailSamples>,
        prepared_builtins: Option<&PreparedBuiltinsSession>,
    ) -> Result<PreparedGraphArtifact, GraphCompileFailure> {
        let GraphCompileRequest {
            plan_id,
            effects,
            caps,
            dispatch,
        } = request;
        let mut diagnostics = Vec::new();
        if !caps.all_nonzero() {
            diagnostics.push(diag("graph.resource.limit", "$.graph_compile_caps"));
        }
        // NOT YET REMOVED (#99 F5, deliberately; tracked by #162): this clones the whole
        // `CompiledSession`, canonical TOML included, purely to satisfy the borrow checker --
        // `model` borrows the session, and the transactional failure path must hand `effects`
        // back **by value** from inside the loops that read `model`. Removing it means
        // restructuring a 500-line function so every early `failure(effects, ..)` happens after
        // the borrow ends, and the failure path is a frozen API contract. Left as a bounded
        // successor rather than rushed: the shape is a
        // `build(&effects) -> Result<Built, Vec<GraphDiagnostic>>` that returns owned outputs,
        // with `failure(effects, ..)` called only on its `Err`. The dominant F5 cost -- the
        // canonical dump, its SHA and the Graphviz string on every compile -- is gone.
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
        let levels = topo(&nodes, &edges).expect("acyclic graph has schedule");
        let schedule: Vec<_> = levels
            .iter()
            .flat_map(|level| level.nodes.iter().cloned())
            .collect();
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
        // Reductions were only ever computed for the canonical text; `GraphCompiler::evidence`
        // recomputes them from the plan's spec when something asks (#99 F5).
        // Mono-collapse M1: one derivation of the pool class per compile, handed to **both** bank
        // planners. `SessionPoolClasses` states the obligation and why the map is an object
        // rather than a predicate each planner calls; the short version is that two planners that
        // disagreed about one track would slide their banks' lane sets out of step and every #208
        // chain merge would decline silently.
        //
        // The contributors are the prepare-time terms of every upstream-of-seam stage this compile
        // actually prepared: `SOURCE` from the compiled session, `DESIGNED` from each prepared
        // native effect and -- when this is the `compile_with_builtins` path -- from each track's
        // prepared input section. `GraphCompiler::compile` has no input sections in its plan at
        // all, so having one fewer contributor there is the honest answer rather than a gap.
        let mut pool_classes = SessionPoolClasses::from_session(&session);
        for entry in &effects.entries {
            let mut witness = ChannelSymmetryWitness::SYMMETRIC;
            witness.set(
                ChannelSymmetryWitness::DESIGNED,
                entry.processor.channel_symmetry(),
            );
            pool_classes.conjoin(&entry.track_id, witness);
        }
        if let Some(builtins) = prepared_builtins {
            for (track, witness) in builtins.input_channel_symmetry() {
                pool_classes.conjoin(track, witness);
            }
        }
        let (banks, rack_cohorts) =
            match bind_rack_banks(&effects, &effect_ids, &levels, dispatch, &pool_classes) {
                Ok(value) => value,
                Err(diagnostic) => return Err(failure(effects, vec![diagnostic])),
            };
        // Issue #210 phase 2. Only tracks that actually declared a delay appear, in normalized
        // track order: an undelayed session produces an empty vector, and every downstream
        // consumer -- the estimate term, the lowering, the runtime's line vector -- is then
        // exactly what it was before this feature existed.
        let track_delays: Vec<PreparedTrackDelay> = model
            .tracks
            .iter()
            .filter(|track| {
                track.builtins.left.delay_samples != 0 || track.builtins.right.delay_samples != 0
            })
            .map(|track| PreparedTrackDelay {
                node: track_node(track.id.as_str(), TrackStage::Input),
                left_samples: track.builtins.left.delay_samples,
                right_samples: track.builtins.right.delay_samples,
            })
            .collect();
        let Some(track_delay_bytes) = track_delays.iter().try_fold(0_u64, |total, delay| {
            total
                .checked_add(u64::from(delay.left_samples).checked_mul(4)?)?
                .checked_add(u64::from(delay.right_samples).checked_mul(4)?)
        }) else {
            return Err(failure(
                effects,
                vec![diag(
                    "graph.resource.arithmetic_overflow",
                    "$.graph.track_delays",
                )],
            ));
        };
        let Some(mut estimate) = resource_estimate(
            session.quantum().0,
            session.resource_estimate().requested_runtime_bytes,
            &nodes,
            &edges,
            &schedule,
            &levels,
            &buffers,
            &timing,
            &effects.entries,
            track_delay_bytes,
            &track_delays,
        ) else {
            return Err(failure(
                effects,
                vec![diag("graph.resource.arithmetic_overflow", "$.graph")],
            ));
        };
        // Runtime-selected banks do not change the target-neutral semantic graph hash. Preserve
        // the pre-bank estimate for canonical bytes while publishing and capping the exact
        // retained candidate estimate below.
        let semantic_estimate = estimate.clone();
        let Some(bank_resource) = effect_bank_resource(&banks, session.quantum().0) else {
            return Err(failure(
                effects,
                vec![diag(
                    "graph.resource.arithmetic_overflow",
                    "$.graph.effect_banks",
                )],
            ));
        };
        if checked_add_effect_banks(&mut estimate, bank_resource).is_none() {
            return Err(failure(
                effects,
                vec![diag(
                    "graph.resource.arithmetic_overflow",
                    "$.graph.effect_banks",
                )],
            ));
        }
        let mut capped_estimate = estimate.clone();
        if let Some(builtins) = prepared_builtins {
            let Some(resource) =
                builtins.graph_builtin_bank_resource(rack_cohorts.dispatch, &levels, &pool_classes)
            else {
                return Err(failure(
                    effects,
                    vec![diag(
                        "graph.resource.arithmetic_overflow",
                        "$.graph.builtin_banks",
                    )],
                ));
            };
            if capped_estimate
                .checked_add_builtin_banks(resource)
                .is_none()
            {
                return Err(failure(
                    effects,
                    vec![diag(
                        "graph.resource.arithmetic_overflow",
                        "$.graph.builtin_banks",
                    )],
                ));
            }
        }
        if !estimate_fits_platform(&capped_estimate) {
            return Err(failure(
                effects,
                vec![diag("graph.resource.arithmetic_overflow", "$.graph")],
            ));
        }
        if capped_estimate.materialized_nodes > caps.maximum_nodes
            || capped_estimate.edges > caps.maximum_edges
            || capped_estimate.schedule_items > caps.maximum_schedule_items
            || capped_estimate.dependency_levels > caps.maximum_dependency_levels
            || capped_estimate.audio_buffer_samples > caps.maximum_audio_buffer_samples
            || capped_estimate.graph_metadata_bytes > caps.maximum_graph_bytes
            || capped_estimate.incremental_plan_bytes > caps.maximum_plan_bytes
            || capped_estimate.largest_allocation_bytes > caps.maximum_single_allocation_bytes
        {
            return Err(failure(
                effects,
                vec![diag("graph.resource.limit", "$.graph_compile_caps")],
            ));
        }
        // No canonical text, no SHA-256 and no Graphviz here: they are evidence, not plan, and
        // `GraphCompiler::evidence` produces them from the finished plan when something asks
        // (#99 F5). Nothing on the structural-mutation path pays for them any more.
        let (effect_nodes, effect_controls, effect_observations) =
            into_effects(effects.entries, &effect_ids);
        let spec = GraphSpec {
            ports,
            nodes,
            edges,
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
            sequential_schedule: schedule,
            dependency_levels: levels,
            route_timings: timing.routes,
            inserted_delays: timing.delays,
            buffer_assignments: buffers,
            estimate: estimate.clone(),
            envelope: RenderEnvelope {
                sample_rate: session.sample_rate(),
                quantum: session.quantum(),
                input_channels: None,
                output_channels: core::num::NonZeroUsize::new(2).expect("constant"),
            },
            required_bindings,
            routes: route_transforms,
            track_delays,
            effects: effect_nodes,
            banks,
            effect_controls,
            builtin_banks: Vec::new(),
            observers: Vec::new(),
            effect_observations,
        });
        Ok(PreparedGraphArtifact {
            pool_classes,
            graph,
            report: GraphCompileReport {
                output_latency: timing.output_latency,
                output_tail: timing.output_tail,
                semantic_estimate,
                estimate,
                rack_cohorts,
            },
        })
    }
}
