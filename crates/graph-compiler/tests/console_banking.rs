//! Issue #2132 gate 1: the bank report's utilisation (`BankUtilisation`), computed at compile from
//! the plan.
//!
//! The standing 64-track console fills every bank, so it reports no mergeable pair and full lane
//! use. Two partial banks of one key class at one dependency level are one mergeable pair, and the
//! same two at two levels are none (this slice's one-level rule; slice 6b, #2136, widens it to
//! intersecting windows). The current planner never emits two partial groups of one key at one
//! level (`plan_bank_groups` pools every compatible candidate of a level together), so that pair is
//! built from the compiled console's own groups rather than from a session.

use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry, prepare_native_session_effects,
};
use graph::GraphCompileCaps;
use graph_compiler::{
    Backend, BankUtilisation, GraphBuiltinsCompileRequest, GraphCompiler, GraphRackBankReport,
    GraphRackBoundSlot, PreparedGraphBuiltinsArtifact,
};
use rack_compiler::{BankGroup, CohortPoolClass};
use session::{CompileCaps, SessionModel, compile_session, parse_session_json};
use std::collections::BTreeMap;

const INTENDED: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track-intended.json");

/// The widest width this build runs.
fn width() -> Backend {
    *Backend::VECTOR.last().expect("a vector width")
}

fn report(model: &SessionModel) -> GraphRackBankReport {
    compile(model).report().rack_cohorts.clone()
}

fn compile(model: &SessionModel) -> PreparedGraphBuiltinsArtifact {
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
    .unwrap_or_else(|error| panic!("session: {:?}", error.diagnostics()));
    let effects = prepare_native_session_effects(
        &session,
        launch_native_effect_registry().expect("launch registry"),
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 30,
            maximum_scratch_bytes: 1 << 28,
            maximum_automation_spans_per_block: 128,
        },
    )
    .unwrap_or_else(|error| panic!("effects: {}", error.0[0].code));
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
    .expect("the session's builtins");
    GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        input_floors: Default::default(),
        plan_id: 2132,
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
        dispatch: width(),
        gain_liveness: graph_compiler::GainLiveness::FIXED,
    })
    .unwrap_or_else(|failure| panic!("graph: {:?}", failure.diagnostics))
}

/// `group` with only the active lanes `range` kept, moved to lanes `0..`, and the rest padding.
fn partial(
    group: &BankGroup<graph_compiler::RackChainId>,
    range: core::ops::Range<usize>,
) -> BankGroup<graph_compiler::RackChainId> {
    let lanes = group.members.len();
    let mut members = vec![None; lanes];
    let mut active_mask = vec![false; lanes];
    let mut active_slots = vec![vec![false; group.program.len()].into_boxed_slice(); lanes];
    for (lane, source) in range.enumerate() {
        members[lane] = group.members[source].clone();
        active_mask[lane] = true;
        active_slots[lane] = group.active_slots[source].clone();
    }
    BankGroup {
        level: group.level,
        rack: group.rack,
        class: group.class,
        program: group.program.clone(),
        members: members.into_boxed_slice(),
        active_mask: active_mask.into_boxed_slice(),
        active_slots: active_slots.into_boxed_slice(),
    }
}

/// Gate 1. Test value: red when the report stops seeing a pair the planner could have merged --
/// a count that ignores the pool class, or one that counts padded lanes as active (then two
/// half-full banks no longer fit one, and the lane use reads full).
#[test]
fn the_bank_report_counts_mergeable_pairs() {
    let console = report(&parse_session_json(INTENDED).expect("the standing console parses"));
    let lanes = console.plan.groups[0].members.len();
    assert_eq!(
        console.utilisation,
        BankUtilisation {
            banks: console.plan.groups.len() as u32,
            mergeable_partial_pairs: 0,
            lane_use_permille: 1000,
        },
        "64 tracks fill every bank at {lanes} lanes"
    );
    assert!(
        console.utilisation.banks >= 2,
        "one group per console rack at least"
    );

    // Two half-full banks of the first console group: one rack, one class, one leader program,
    // one level, and no route on either (an empty fold map), so together they fit one bank.
    let full = &console.plan.groups[0];
    let half = lanes / 2;
    let first = partial(full, 0..half);
    let second = partial(full, half..lanes - 1);
    let bound = |group: usize, members: usize| GraphRackBoundSlot {
        group,
        slot: 0,
        members: console.bound_slots[0].members[..members].to_vec(),
    };
    let slots = [bound(0, half), bound(1, lanes - 1 - half)];
    let no_routes = BTreeMap::new();
    let measure = |a: &BankGroup<_>, b: &BankGroup<_>| {
        BankUtilisation::of(
            &[a.clone(), b.clone()],
            &slots,
            lanes,
            &no_routes,
            &BTreeMap::new(),
            None,
        )
    };
    let one_level = measure(&first, &second);
    assert_eq!(
        one_level,
        BankUtilisation {
            banks: 2,
            mergeable_partial_pairs: 1,
            lane_use_permille: ((lanes as u32 - 1) * 1000 + lanes as u32) / (2 * lanes as u32),
        },
        "two partial banks of one key at one level are one mergeable pair"
    );
    let mut later = second.clone();
    later.level += 1;
    assert_eq!(
        measure(&first, &later).mergeable_partial_pairs,
        0,
        "at two levels they are none under this slice's rule"
    );
    let mut other_class = second.clone();
    other_class.class = match second.class {
        CohortPoolClass::Stereo => CohortPoolClass::MonoSymmetricAtPrepare,
        CohortPoolClass::MonoSymmetricAtPrepare => CohortPoolClass::Stereo,
    };
    assert_eq!(
        measure(&first, &other_class).mergeable_partial_pairs,
        0,
        "another pool class is another key"
    );
    let mut routed = BTreeMap::new();
    for member in first.members.iter().flatten() {
        routed.insert(member.track_id.clone(), vec!["bus".to_owned()]);
    }
    assert_eq!(
        BankUtilisation::of(
            &[first.clone(), second.clone()],
            &slots,
            lanes,
            &routed,
            &BTreeMap::new(),
            None
        )
        .mergeable_partial_pairs,
        0,
        "another fold target is another key"
    );
    // Both groups route their members to different places: neither has one fold target, and two
    // "mixed" groups are never one key.
    let mut mixed = BTreeMap::new();
    for (index, member) in first
        .members
        .iter()
        .chain(second.members.iter())
        .flatten()
        .enumerate()
    {
        mixed.insert(member.track_id.clone(), vec![format!("bus{}", index % 2)]);
    }
    assert_eq!(
        BankUtilisation::of(
            &[first.clone(), second.clone()],
            &slots,
            lanes,
            &mixed,
            &BTreeMap::new(),
            None
        )
        .mergeable_partial_pairs,
        0,
        "two groups of mixed fold targets are not one key"
    );
}

/// Issue #2134 gate 3: on every standing session fixture whose banks each sit at one dependency
/// level, the executor order -- the op order `graph::program::lower` emits, the graph contracted
/// by bank -- is the canonical schedule minus its elided nodes: no op moved. On such a plan the
/// contraction must reproduce today's unit order exactly, because #2142's lane map builds lane
/// chains and pack groups from unit adjacency and order. Since #2136 the C1 consoles bank across
/// levels (their submix `post_insert` cohort), so they are the fixtures this claim leaves out.
///
/// Test value: red when the contracted order moves a unit that did not need to move -- a key that
/// emits a bank's members together, or one that sinks a one-level bank past a consumer.
#[test]
fn the_contracted_order_is_the_schedule_on_every_one_level_fixture() {
    let mut one_level = Vec::new();
    for (name, text) in [
        (
            "parametric-eq-nine-track",
            include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json"),
        ),
        (
            "parametric-eq-bank-console",
            include_str!("../../../fixtures/session/v1/parametric-eq-bank-console.json"),
        ),
        (
            "console-sixty-four-track",
            include_str!("../../../fixtures/session/v1/console-sixty-four-track.json"),
        ),
        ("console-sixty-four-track-intended", INTENDED),
        (
            "console-sixty-four-track-app",
            include_str!("../../../fixtures/session/v1/console-sixty-four-track-app.json"),
        ),
        (
            "console-sixty-four-track-mono",
            include_str!("../../../fixtures/session/v1/console-sixty-four-track-mono.json"),
        ),
        (
            "console-sixty-four-track-sends",
            include_str!("../../../fixtures/session/v1/console-sixty-four-track-sends.json"),
        ),
        (
            "console-c1-16",
            include_str!("../../../fixtures/session/v1/console-c1-16.json"),
        ),
        (
            "console-c1-32",
            include_str!("../../../fixtures/session/v1/console-c1-32.json"),
        ),
        (
            "console-c1-64",
            include_str!("../../../fixtures/session/v1/console-c1-64.json"),
        ),
        (
            "irregular-graph",
            include_str!("../../../fixtures/session/v1/irregular-graph.json"),
        ),
    ] {
        let artifact = compile(&parse_session_json(text).expect("the fixture parses"));
        let plan = artifact.graph();
        let level_of: BTreeMap<&graph::GraphNodeId, u64> = plan
            .dependency_levels
            .iter()
            .flat_map(|level| level.nodes.iter().map(move |node| (node, level.level)))
            .collect();
        let effect_banks = artifact
            .report()
            .rack_cohorts
            .bound_slots
            .iter()
            .map(|bound| {
                bound
                    .members
                    .iter()
                    .map(|member| level_of[&graph::GraphNodeId::Effect(member.clone())])
                    .collect::<Vec<u64>>()
            });
        let builtin_banks = artifact.prepared_builtin_banks().map(|bank| {
            bank.members
                .iter()
                .map(|member| level_of[member])
                .collect::<Vec<u64>>()
        });
        let cross_level = effect_banks
            .chain(builtin_banks)
            .any(|levels| levels.iter().any(|level| *level != levels[0]));
        if cross_level {
            continue;
        }
        one_level.push(name);
        let program = plan.program().expect("the plan lowers");
        let canonical: Vec<u32> = plan
            .sequential_schedule
            .iter()
            .map(|id| graph::program::node_index(&plan.spec, id).expect("a node"))
            .filter(|index| program.node_op[*index as usize].is_some())
            .collect();
        assert!(
            program.ops.len() == canonical.len(),
            "{name}: one op per unelided node"
        );
        let moved = program
            .ops
            .iter()
            .zip(&canonical)
            .filter(|(op, node)| op.node != **node)
            .count();
        assert_eq!(moved, 0, "{name}: the executor order moved {moved} ops");
    }
    // The consoles without submixes have nothing to place across levels, at any width.
    for name in [
        "parametric-eq-nine-track",
        "console-sixty-four-track",
        "console-sixty-four-track-intended",
        "console-sixty-four-track-mono",
    ] {
        assert!(one_level.contains(&name), "{name} banks at one level each");
    }
}

/// Issue #2136 gate 3: the C1 console at 16, 32 and 64 tracks, planned over level windows.
///
/// The four groups take the tracks' post-pan routes and the two aux returns their post-fader
/// sends, so the auxes' strips start a level earlier than the groups'. Aux A's pre segment feeds
/// its per-node delay, which puts aux A on the longest path: its `pre_insert` window is a single
/// level, and it shares its bank with aux B there (as before). Aux B's `post_insert` window
/// reaches the groups' and aux A's, so the six submix drives are one cohort at one placement level
/// -- one bank at eight lanes, two at four, where the level plan bound two and three. Each strip's
/// fader and matrix follow its drive, so those builtin banks hold the same six strips and the
/// chain still fuses. Static bank-slot runs: `4 ceil(n / W)` for the tracks, 6 for the two submix
/// `pre_insert` banks, `ceil(6 / W)` for the drives and 4 for the master (19, 27 and 43 at eight
/// lanes; 28, 44 and 76 at four). No two partial banks of one key have intersecting windows.
///
/// Test value: red when two partial banks of one key with intersecting windows are left unmerged
/// -- a level bucket leaves aux B's drive apart from the others' (20, 28, 44 at eight lanes).
#[test]
fn the_c1_submix_drives_are_one_cohort_over_level_windows() {
    let lanes = width().width();
    for (tracks, text) in [
        (
            16_usize,
            include_str!("../../../fixtures/session/v1/console-c1-16.json"),
        ),
        (
            32,
            include_str!("../../../fixtures/session/v1/console-c1-32.json"),
        ),
        (
            64,
            include_str!("../../../fixtures/session/v1/console-c1-64.json"),
        ),
    ] {
        let report = report(&parse_session_json(text).expect("the C1 fixture parses"));
        let submix = |track: &str| track.starts_with("aux.") || track.starts_with("group.");
        let mut submix_groups: Vec<(graph::RackId, Vec<String>)> = report
            .plan
            .groups
            .iter()
            .filter(|group| {
                group
                    .members
                    .iter()
                    .flatten()
                    .all(|chain| submix(&chain.track_id))
            })
            .filter(|group| group.rack != rack::RackLocation::Dynamic)
            .map(|group| {
                let members: Vec<String> = group
                    .members
                    .iter()
                    .flatten()
                    .map(|chain| chain.track_id.clone())
                    .collect();
                let rack = group
                    .members
                    .iter()
                    .flatten()
                    .next()
                    .expect("a member")
                    .rack;
                (rack, members)
            })
            .collect();
        submix_groups.sort();
        let groups = ["group.drums", "group.fx", "group.music", "group.vox"].map(String::from);
        let mut drives = vec!["aux.a".to_owned(), "aux.b".to_owned()];
        drives.extend(groups.iter().cloned());
        let mut expected = vec![
            (
                graph::RackId::Simd1,
                vec!["aux.a".to_owned(), "aux.b".to_owned()],
            ),
            (graph::RackId::Simd1, groups.to_vec()),
        ];
        for chunk in drives.chunks(lanes) {
            expected.push((graph::RackId::Simd2, chunk.to_vec()));
        }
        expected.sort();
        assert_eq!(
            submix_groups, expected,
            "C1-{tracks}: the submix console groups"
        );
        assert_eq!(
            report.bound_slots.len(),
            4 * tracks.div_ceil(lanes) + 6 + 6_usize.div_ceil(lanes) + 4,
            "C1-{tracks}: static bank-slot runs"
        );
        assert_eq!(
            report.utilisation.mergeable_partial_pairs, 0,
            "C1-{tracks}: no mergeable pair"
        );
    }
    let irregular = report(
        &parse_session_json(include_str!(
            "../../../fixtures/session/v1/irregular-graph.json"
        ))
        .expect("the irregular graph parses"),
    );
    assert_eq!(
        irregular.utilisation.mergeable_partial_pairs, 0,
        "irregular graph: no mergeable pair"
    );
}
