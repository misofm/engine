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
    GraphRackBoundSlot,
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
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
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
    .unwrap_or_else(|failure| panic!("graph: {:?}", failure.diagnostics));
    artifact.report().rack_cohorts.clone()
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
        BankUtilisation::of(&[a.clone(), b.clone()], &slots, lanes, &no_routes, None)
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
            None
        )
        .mergeable_partial_pairs,
        0,
        "two groups of mixed fold targets are not one key"
    );
}
