//! The compile entry point and its orchestration.
//!
//! [`GraphCompiler::compile_with_builtins`] is the one entry: it validates the prepared builtins
//! against the session, runs `compile_graph`, and attaches the builtin banks. `compile_graph` is
//! the graph pipeline: validate the prepared effects against the session model, materialise nodes
//! and edges, detect cycles, order and level the graph, resolve PDC, colour output buffers, plan
//! and bind the SIMD-rack banks, and check the resource estimate against the caps --
//! transactionally, handing every caller-owned input back on any failure.
//!
//! Evidence -- the canonical text, its SHA-256 and the Graphviz rendering -- is **not** produced
//! here (#99 F5); see [`GraphCompiler::evidence`].

use super::*;
use crate::banks::{bind_rack_banks_indexed, checked_add_effect_banks, effect_bank_resource};
use crate::canonical::{
    Sha256Writer, canonical_parts, dot, hex_digest, hex_sha256, reductions_of, write_canonical,
};
use crate::estimate::{effect_control_resource, estimate_fits_platform, resource_estimate};
use crate::ids::{
    PreparedEffectIndex, add_main_edge, add_node, add_route_destination_edge,
    add_route_source_edge, diag, effect_path, gid, into_effects, port, ports_for,
    prepared_effect_node, route_coefficients, route_destination_node, route_source_node,
    route_transform, sidechain_matches, stages, track_node,
};
use crate::pdc::timings;
use crate::schedule::{buffer_assignments, cycle_primary_path, cycle_witnesses, topo};

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

    /// Compile an effect-prepared session and its prepared builtins into one bindable plan.
    ///
    /// The builtins are required: every host renders a track's input section, fader and pan
    /// matrix, and the builtins-less entry that compiled a plan without them was deleted (issue
    /// #959). On any failure the prepared effects and builtins are handed back with the sorted
    /// diagnostics.
    // The transactional API returns the complete prepared-effect and builtin inputs by value on
    // failure. Boxing them would change that ownership contract solely to optimize a cold path.
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
        let compiled = match Self::compile_graph(plan_id, effects, caps, dispatch, &builtins) {
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

    /// The graph pipeline, run against builtins `compile_with_builtins` has already validated
    /// for this session. The builtin banks are attached by the caller; this charges their
    /// reservation, lists their three stages for binding, and takes each track's input-section
    /// tail from them.
    #[allow(clippy::result_large_err)]
    fn compile_graph(
        plan_id: u64,
        effects: EffectPreparedSession,
        caps: GraphCompileCaps,
        dispatch: Backend,
        builtins: &PreparedBuiltinsSession,
    ) -> Result<CompiledGraph, GraphFailure> {
        // `validate_for_session` has checked the tail set against the session's tracks, so every
        // track has exactly one entry here.
        let builtin_tails: BTreeMap<&str, TailSamples> = builtins
            .tails()
            .map(|(track_id, tail)| {
                (
                    track_id,
                    match tail {
                        BuiltinTail::FiniteZero => TailSamples::Finite(0),
                        BuiltinTail::Infinite => TailSamples::Infinite,
                    },
                )
            })
            .collect();
        let mut diagnostics = Vec::new();
        if !caps.all_nonzero() {
            diagnostics.push(diag("graph.resource.limit", "$.graph_compile_caps"));
        }
        // Borrow only the session field: rejecting branches end the borrow before returning
        // all of `effects`, while success consumes the disjoint entries field after validation.
        let session = &effects.session;
        let model = session.normalized_model();
        if model.outputs.len() != 1 {
            diagnostics.push(diag("graph.output.cardinality", "$.outputs"));
        }
        if !diagnostics.is_empty() {
            return Err(failure(effects, diagnostics));
        }

        let (prepared, duplicate_prepared) = PreparedEffectIndex::from_entries(&effects.entries);
        for _ in 0..duplicate_prepared {
            diagnostics.push(diag("graph.effect.duplicate_prepared", "$.effects"));
        }
        // Decision 12, class A by lowering: `console.pre_insert` is `RackId::Simd1`, the track's
        // inserts `Dynamic` and `console.post_insert` `Simd2`, each console entry an ordinary
        // effect. Lowered once here and borrowed by every pass below.
        let strips: Vec<_> = model.strips().collect();
        let lowered: Vec<_> = strips
            .iter()
            .map(|strip| model.lower_strip(strip))
            .collect();
        let racks = |index: usize| {
            let [pre_insert, inserts, post_insert] = lowered[index].in_chain_order();
            [
                (RackId::Simd1, pre_insert, TrackStage::PostSimd1),
                (RackId::Dynamic, inserts, TrackStage::PostDynamic),
                (RackId::Simd2, post_insert, TrackStage::PostSimd2PreFader),
            ]
        };
        let mut declared = BTreeSet::<(&str, RackId, &str)>::new();
        for (index, strip) in strips.iter().enumerate() {
            for (rack, values, _) in racks(index) {
                for effect in values {
                    let key = (strip.id.as_str(), rack, effect.id.as_str());
                    declared.insert(key);
                    let Some(slot) = prepared.get(key.0, key.1, key.2) else {
                        diagnostics.push(diag(
                            "graph.effect.missing_prepared",
                            &effect_path(&strip.path_prefix(), rack, effect.id.as_str()),
                        ));
                        continue;
                    };
                    if !sidechain_matches(&effect.sidechain, &effects.entries[slot.index()]) {
                        diagnostics.push(diag(
                            "graph.effect.metadata_mismatch",
                            &effect_path(&strip.path_prefix(), rack, effect.id.as_str()),
                        ));
                    }
                }
            }
        }
        for key in prepared.iter() {
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
        let mut effect_ids = vec![None; effects.entries.len()];
        let mut route_transforms = Vec::new();
        for (index, strip) in strips.iter().enumerate() {
            for stage in stages() {
                let id = track_node(strip.id.as_str(), stage);
                let tail = if stage == TrackStage::PostInputBuiltins {
                    *builtin_tails
                        .get(strip.id.as_str())
                        .expect("validated builtins carry one tail per track")
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
            let mut preceding = track_node(strip.id.as_str(), TrackStage::Input);
            let builtins = track_node(strip.id.as_str(), TrackStage::PostInputBuiltins);
            add_main_edge(
                &mut edges,
                preceding.clone(),
                builtins.clone(),
                strip.collection_path().to_owned(),
            );
            preceding = builtins;
            for (rack, values, boundary) in racks(index) {
                for effect in values {
                    let id = EffectNodeId {
                        track_id: gid(strip.id.as_str()),
                        rack,
                        effect_id: gid(effect.id.as_str()),
                    };
                    let node = GraphNodeId::Effect(id.clone());
                    let slot = prepared
                        .get(strip.id.as_str(), rack, effect.id.as_str())
                        .expect("validated prepared effect");
                    let index = slot.index();
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
                        effect_path(&strip.path_prefix(), rack, effect.id.as_str()),
                    );
                    preceding = node.clone();
                    effect_ids[index] = Some(id);
                }
                let end = track_node(strip.id.as_str(), boundary);
                add_main_edge(
                    &mut edges,
                    preceding.clone(),
                    end.clone(),
                    strip.collection_path().to_owned(),
                );
                preceding = end;
            }
            let fader = track_node(strip.id.as_str(), TrackStage::PostFader);
            let matrix = track_node(strip.id.as_str(), TrackStage::PostMatrix);
            add_main_edge(
                &mut edges,
                preceding,
                fader.clone(),
                strip.collection_path().to_owned(),
            );
            add_main_edge(
                &mut edges,
                fader,
                matrix,
                strip.collection_path().to_owned(),
            );
        }
        // #1200 D1: a submix is a strip above, keyed by its own ID (tracks, submixes and outputs
        // share one ID namespace). Its `Input` stage has no source binding; the route edges that
        // name it are its inputs, reduced in D9 edge order. `GraphNodeId::Submix` is no longer
        // emitted, though the graph crate keeps the variant.
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
            let matrix = &route.channel_matrix;
            let matrix = [matrix.ll, matrix.lr, matrix.rl, matrix.rr];
            // The domain check is the live producer's own (issue #1215 D1): a value it would
            // refuse never compiles, and the plan keeps the unfolded transform it hashes. The
            // session's mute rides the gate (issue #1216 D2); it is not structural.
            let gate = RouteGate {
                mute: route.mute,
                follow_zeroed: [false; 2],
            };
            let transform =
                route_coefficients(route.gain_db, matrix, gate.mute, gate.follow_zeroed)
                    .ok()
                    .and_then(|_| route_transform(route.gain_db, matrix));
            let Some(transform) = transform else {
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
            let source = route_source_node(&route.source);
            let destination = route_destination_node(&route.destination);
            add_route_source_edge(&mut edges, source, route_node.clone(), route.id.as_str());
            add_route_destination_edge(&mut edges, route_node, destination, route.id.as_str());
            route_transforms.push(PreparedRoute {
                node: GraphNodeId::Route {
                    route_id: gid(route.id.as_str()),
                },
                transform,
                gate,
            });
        }
        for (index, strip) in strips.iter().enumerate() {
            for (rack, values, _) in racks(index) {
                for effect in values {
                    let SidechainDeclaration::Routed(sidechain) = &effect.sidechain else {
                        continue;
                    };
                    let source = route_source_node(&sidechain.source);
                    let slot = prepared
                        .get(strip.id.as_str(), rack, effect.id.as_str())
                        .expect("validated prepared effect");
                    let id = prepared_effect_node(&effect_ids, slot)
                        .expect("prepared effect node assigned")
                        .clone();
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
                        path: format!("{}.sidechain", strip.path_prefix()),
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
                path: cycle_primary_path(&cycle.0, &cycle.1),
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
        // prepared: `SOURCE` from the compiled session, and `DESIGNED` from each prepared native
        // effect and from each track's prepared input section.
        //
        // `bind_rack_banks_indexed` is the one place the map then changes (issue #971): a mono
        // track that every effect group would strand is pooled as stereo, before the rack plan it
        // keeps and before the builtin-stage planner below reads the map.
        let mut pool_classes = SessionPoolClasses::from_session(session);
        for entry in &effects.entries {
            let mut witness = ChannelSymmetryWitness::SYMMETRIC;
            witness.set(
                ChannelSymmetryWitness::DESIGNED,
                entry.processor.channel_symmetry(),
            );
            pool_classes.conjoin(&entry.track_id, witness);
        }
        for (track, witness) in builtins.input_channel_symmetry() {
            pool_classes.conjoin(track, witness);
        }
        let (banks, rack_cohorts) = match bind_rack_banks_indexed(
            &effects,
            &prepared,
            &effect_ids,
            &levels,
            dispatch,
            &mut pool_classes,
        ) {
            Ok(value) => value,
            Err(diagnostic) => return Err(failure(effects, vec![diagnostic])),
        };
        // Every strip, in `strips()` order: tracks, then submixes (#1201 D1). The entry is keyed
        // by the strip's `Input` node either way, and the runtime tells the two apart by what
        // that node is. A track's `Input` is a source input, so its entry lowers to the
        // `TrackDelay` arm, which delays the source in place. A submix's `Input` has no source and
        // reduces the routes that target it, so its entry lowers to the `SumDelay` arm, which
        // delays that sum after the reduction. Neither is latency, and PDC compensates neither.
        // Issue #210 phase 2. Only strips that actually declared a delay appear: an undelayed
        // session produces an empty vector, and every downstream consumer -- the estimate term,
        // the lowering, the runtime's line vector -- is then exactly what it was before this
        // feature existed.
        let track_delays: Vec<PreparedTrackDelay> = model
            .strips()
            .filter(|strip| {
                strip.builtins.left.delay_samples != 0 || strip.builtins.right.delay_samples != 0
            })
            .map(|strip| PreparedTrackDelay {
                node: track_node(strip.id.as_str(), TrackStage::Input),
                left_samples: strip.builtins.left.delay_samples,
                right_samples: strip.builtins.right.delay_samples,
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
        let Some(control_resource) = effect_control_resource(&effects.entries, &banks) else {
            return Err(failure(
                effects,
                vec![diag(
                    "graph.resource.arithmetic_overflow",
                    "$.graph.effect_controls",
                )],
            ));
        };
        if estimate
            .checked_add_scalar_owners(control_resource)
            .is_none()
        {
            return Err(failure(
                effects,
                vec![diag(
                    "graph.resource.arithmetic_overflow",
                    "$.graph.effect_controls",
                )],
            ));
        }
        let Some(emitted_op_count) = levels.iter().try_fold(0_u64, |total, level| {
            total.checked_add(u64::try_from(level.nodes.len()).ok()?)
        }) else {
            return Err(failure(
                effects,
                vec![diag(
                    "graph.resource.arithmetic_overflow",
                    "$.graph.runtime_metadata",
                )],
            ));
        };
        // The prepared runtime retains one ordered response owner for each input-filter stage
        // and prepared effect. Charge the table and both cloned identity strings here, while the
        // semantic model is still borrowed; the runtime itself is deliberately opaque to this
        // compiler after lowering.
        let Some(response_binding_count) = u64::try_from(strips.len())
            .ok()
            .and_then(|tracks| tracks.checked_add(u64::try_from(effects.entries.len()).ok()?))
        else {
            return Err(failure(
                effects,
                vec![diag(
                    "graph.resource.arithmetic_overflow",
                    "$.graph.runtime_metadata.response_bindings",
                )],
            ));
        };
        let Some((response_binding_string_bytes, largest_response_binding_string_bytes)) = strips
            .iter()
            .try_fold((0_u64, 0_u64), |(total, largest), strip| {
                let track_bytes = u64::try_from(strip.id.as_str().len()).ok()?;
                let stable_bytes = u64::try_from("input-filters".len()).ok()?;
                let native_bytes = u64::try_from("miso.builtin.input-filters".len()).ok()?;
                let total = total
                    .checked_add(track_bytes)?
                    .checked_add(stable_bytes)?
                    .checked_add(native_bytes)?;
                Some((
                    total,
                    largest.max(track_bytes).max(stable_bytes).max(native_bytes),
                ))
            })
            .and_then(|(total, largest)| {
                effects
                    .entries
                    .iter()
                    .try_fold((total, largest), |(total, largest), entry| {
                        let track_bytes = u64::try_from(entry.track_id.len()).ok()?;
                        let stable_bytes = u64::try_from(entry.effect_id.len()).ok()?;
                        let native_bytes =
                            u64::try_from(entry.factory.descriptor().id.as_str().len()).ok()?;
                        let total = total
                            .checked_add(track_bytes)?
                            .checked_add(stable_bytes)?
                            .checked_add(native_bytes)?;
                        Some((
                            total,
                            largest.max(track_bytes).max(stable_bytes).max(native_bytes),
                        ))
                    })
            })
        else {
            return Err(failure(
                effects,
                vec![diag(
                    "graph.resource.arithmetic_overflow",
                    "$.graph.runtime_metadata.response_bindings",
                )],
            ));
        };
        let Some(runtime_resource) =
            graph::GraphRuntimeMetadataResourceEstimate::checked_for_with_response_bindings(
                emitted_op_count,
                response_binding_count,
                response_binding_string_bytes,
                largest_response_binding_string_bytes,
            )
        else {
            return Err(failure(
                effects,
                vec![diag(
                    "graph.resource.arithmetic_overflow",
                    "$.graph.runtime_metadata",
                )],
            ));
        };
        if estimate
            .checked_add_runtime_metadata(runtime_resource)
            .is_none()
        {
            return Err(failure(
                effects,
                vec![diag(
                    "graph.resource.arithmetic_overflow",
                    "$.graph.runtime_metadata",
                )],
            ));
        }
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
        let Some(scalar_owner_resource) =
            builtins.graph_scalar_owner_resource(dispatch, &levels, &pool_classes)
        else {
            return Err(failure(
                effects,
                vec![diag(
                    "graph.resource.arithmetic_overflow",
                    "$.graph.scalar_owners",
                )],
            ));
        };
        if estimate
            .checked_add_scalar_owners(scalar_owner_resource)
            .is_none()
        {
            return Err(failure(
                effects,
                vec![diag(
                    "graph.resource.arithmetic_overflow",
                    "$.graph.scalar_owners",
                )],
            ));
        }
        // The runtime chain retains one slot and one cloned mask per prepared membership.  Fold
        // its conservative coexistence reservation into the published estimate before caps; the
        // builtin payload itself is attached later and must not carry this term a second time.
        let Some(builtin_bank_resource) =
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
        let mut capped_estimate = estimate.clone();
        if capped_estimate
            .checked_add_builtin_banks(builtin_bank_resource)
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
        let bank_count = bank_resource
            .bank_count
            .checked_add(builtin_bank_resource.bank_count);
        let Some(bank_count) = bank_count else {
            return Err(failure(
                effects,
                vec![diag(
                    "graph.resource.arithmetic_overflow",
                    "$.graph.bank_slots",
                )],
            ));
        };
        let effect_mask_bytes = banks
            .iter()
            .map(|bank| {
                u64::try_from(bank.active_mask.len())
                    .ok()?
                    .checked_mul(u64::try_from(core::mem::size_of::<bool>()).ok()?)
            })
            .try_fold(0_u64, |maximum, value| Some(maximum.max(value?)));
        let Some(effect_mask_bytes) = effect_mask_bytes else {
            return Err(failure(
                effects,
                vec![diag(
                    "graph.resource.arithmetic_overflow",
                    "$.graph.bank_slots",
                )],
            ));
        };
        let mask_bytes = effect_mask_bytes.max(builtin_bank_resource.maximum_mask_bytes);
        let Some(slot_resource) =
            graph::GraphBankSlotResourceEstimate::checked_for_mask(bank_count, mask_bytes)
        else {
            return Err(failure(
                effects,
                vec![diag(
                    "graph.resource.arithmetic_overflow",
                    "$.graph.bank_slots",
                )],
            ));
        };
        if capped_estimate
            .checked_add_bank_slot_owners(slot_resource)
            .is_none()
        {
            return Err(failure(
                effects,
                vec![diag(
                    "graph.resource.arithmetic_overflow",
                    "$.graph.bank_slots",
                )],
            ));
        }
        if estimate
            .checked_add_bank_slot_owners(slot_resource)
            .is_none()
        {
            return Err(failure(
                effects,
                vec![diag(
                    "graph.resource.arithmetic_overflow",
                    "$.graph.bank_slots",
                )],
            ));
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
        // The nodes the host must bind: a track's `Input` (its source), the session output, and
        // the three builtin stages -- `PostInputBuiltins`, `PostFader`, `PostMatrix` -- each a
        // compiler-owned binding the builtins artifact fills with a bank member or a scalar owner
        // and keeps as an op. The builtins-less entry left the three builtin stages out of this
        // set, which `program::lower` read to elide them as aliases (issue #925). Issue #959
        // deleted that entry and issue #958 reverted the elision, so `program::lower` no longer
        // reads this set.
        //
        // #1200 D2: a submix's `Input` has no source, so it is not required; unbound, it lowers to
        // the identity reduction of its route inputs.
        let submix_inputs: BTreeSet<GraphNodeId> = model
            .submixes
            .iter()
            .map(|submix| track_node(submix.id.as_str(), TrackStage::Input))
            .collect();
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
                ) && !submix_inputs.contains(*node)
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
        Ok(CompiledGraph {
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

/// The graph half of a compile: the plan before the builtin banks are attached, its report, and
/// the pool classes the second bank planner must read.
struct CompiledGraph {
    graph: PreparedGraphPlan,
    report: GraphCompileReport,
    /// Every track's cohort pool class, as this compile derived it (mono-collapse M1).
    ///
    /// `bind_rack_banks` read it inside the compile, and
    /// `PreparedBuiltinsSession::into_graph_artifact_with_banks` reads the same value rather than
    /// re-deriving one. See [`builtins_compiler::SessionPoolClasses`].
    pool_classes: SessionPoolClasses,
}

/// A rejected graph half: the prepared effects handed back, and why.
struct GraphFailure {
    effects: EffectPreparedSession,
    diagnostics: GraphDiagnosticSet,
}

fn failure(effects: EffectPreparedSession, diagnostics: Vec<GraphDiagnostic>) -> GraphFailure {
    GraphFailure {
        effects,
        diagnostics: GraphDiagnosticSet::sorted(diagnostics),
    }
}
