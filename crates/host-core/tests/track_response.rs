//! #1576: `compose_track_response` composes one strip's captured response and names its members.
//!
//! * **Members and curves.** A 48 kHz strip with asymmetric input filters, an EQ pre-insert
//!   console slot, a bypassed EQ insert with one disabled band, a compressor insert and an EQ
//!   post-insert console slot: five members in signal order with their sections, kinds, bypass
//!   flags and enabled masks; curves bit-equal to `query_response_snapshot_into`.
//! * **Refusals.** One point too many, an owner of another strip and an unknown rack.

use effect_compiler::launch_native_effect_registry;
use host_core::{
    HostPrepareCaps, HostShapePolicy, RESPONSE_MAXIMUM_POINTS, ResponseChannels, ResponseMember,
    ResponseMemberKind, ResponseMemberSection, ResponsePreviewGrid, ResponseSnapshot,
    ResponseSnapshotCollector, ResponseSnapshotOutput, ResponseSnapshotQueryError,
    compile_host_session, compose_track_response, prepare_host_runtime,
    query_response_snapshot_into,
};
use session::{
    ConsoleEntry, ConsoleSlot, Effect, EffectIdentity, EffectParam, EffectQuality, LinkMode,
    ParameterChannel, ParameterUnit, SidechainDeclaration, StableId, canonical_session_json,
    parse_session_json,
};

const BANK: &str = include_str!("../../../fixtures/session/v1/parametric-eq-bank-console.json");
const EQ: &str = "miso.parametric-eq";
const COMPRESSOR: &str = "miso.compressor";
const POINTS: usize = 48;

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 4_096,
        maximum_tracks: 16,
        maximum_submixes: 16,
        maximum_vcas: 16,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_sources: 16,
        maximum_routes: 16,
        maximum_effects: 16,
        maximum_graph_session_plus_plan_bytes: 1_000_000_000,
        maximum_source_total_bytes: 100_000_000,
        maximum_source_overhead_bytes: 100_000_000,
        maximum_effect_state_bytes: 1_000_000_000,
        maximum_effect_scratch_bytes: 1_000_000_000,
        maximum_builtin_retained_bytes: 1_000_000_000,
        maximum_named_allocation_bytes: 1_000_000_000,
        maximum_meter_streams: 64,
        maximum_meter_items: 1 << 16,
        maximum_meter_bytes: 1 << 24,
    }
}

fn sid(id: &str) -> StableId {
    StableId::parse(id).expect("stable id")
}

fn grid(points: usize) -> ResponsePreviewGrid {
    ResponsePreviewGrid::Logarithmic {
        points,
        minimum_hz: 20.0,
        maximum_hz: 20_000.0,
    }
}

/// One `both` row for the EQ parameter `name`, in its declared unit.
fn eq_param(name: &str, value: f32) -> EffectParam {
    let registry = launch_native_effect_registry().expect("registry");
    let descriptor = registry.get_ascii(EQ).expect("EQ").descriptor();
    let parameter = descriptor
        .parameters
        .iter()
        .find(|parameter| parameter.name == name)
        .unwrap_or_else(|| panic!("EQ parameter {name}"));
    use effect_contract::ParameterUnit as Contract;
    EffectParam {
        parameter_id: parameter.id.0,
        channel: ParameterChannel::Both,
        unit: match parameter.unit {
            Contract::Db => ParameterUnit::Db,
            Contract::Hz => ParameterUnit::Hz,
            Contract::Milliseconds => ParameterUnit::Milliseconds,
            Contract::Samples => ParameterUnit::Samples,
            Contract::Linear => ParameterUnit::Linear,
            Contract::Ratio => ParameterUnit::Ratio,
        },
        value,
    }
}

/// An EQ with band 1 on at `gain_db`, and band 2 on or off.
fn eq_params(gain_db: f32, band_2: bool) -> Vec<EffectParam> {
    vec![
        eq_param("band_1_enabled", 1.0),
        eq_param("band_1_gain_db", gain_db),
        eq_param("band_2_enabled", if band_2 { 1.0 } else { 0.0 }),
        eq_param("band_2_gain_db", -gain_db),
    ]
}

fn insert(id: &str, effect_id: &str, bypass: bool, params: Vec<EffectParam>) -> Effect {
    Effect {
        id: sid(id),
        identity: EffectIdentity::Native {
            effect_id: sid(effect_id),
        },
        quality: EffectQuality::Normal,
        bypass,
        link_mode: LinkMode::DualMono,
        params,
        sidechain: SidechainDeclaration::None,
    }
}

fn console_slot(slot: &str) -> ConsoleSlot {
    ConsoleSlot {
        slot: sid(slot),
        identity: EffectIdentity::Native { effect_id: sid(EQ) },
        quality: EffectQuality::Normal,
        link_mode: LinkMode::DualMono,
    }
}

/// The fixture's first strip in the five-member shape, captured at its first boundary.
fn snapshot() -> (String, ResponseSnapshot) {
    let mut model = parse_session_json(BANK).expect("fixture parses");
    assert_eq!(model.sample_rate_hz, 48_000);
    model.tracks.truncate(1);
    let strip = model.tracks[0].id.as_str().to_owned();
    model.routes.retain(|route| {
        matches!(&route.source, session::RouteSource::Track { track_id, .. }
            if track_id.as_str() == strip)
    });
    model.console.pre_insert = vec![console_slot("pre-eq")];
    model.console.post_insert = vec![console_slot("post-eq")];
    let track = &mut model.tracks[0];
    track.builtins.left.hpf_hz = 80.0;
    track.builtins.left.lpf_hz = 12_000.0;
    track.builtins.right.hpf_hz = 160.0;
    track.builtins.right.lpf_hz = 6_000.0;
    track.console = vec![
        ConsoleEntry {
            slot: sid("pre-eq"),
            bypass: false,
            link_mode: session::ConsoleLinkMode::Slot,
            params: eq_params(4.0, true),
        },
        ConsoleEntry {
            slot: sid("post-eq"),
            bypass: false,
            link_mode: session::ConsoleLinkMode::Slot,
            params: eq_params(-3.0, true),
        },
    ];
    track.inserts.effects = vec![
        insert("insert-eq", EQ, true, eq_params(6.0, false)),
        insert("comp", COMPRESSOR, false, Vec::new()),
    ];
    let document = canonical_session_json(&model).expect("canonical");
    let compiled = compile_host_session(&document, &caps()).unwrap_or_else(|failure| {
        panic!("compile: {}", String::from_utf8_lossy(failure.as_bytes()))
    });
    let mut host = prepare_host_runtime(&compiled, &caps()).unwrap_or_else(|failure| {
        panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
    });
    let mut collector = ResponseSnapshotCollector::new(
        48_000,
        8,
        effect_contract::RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS,
        64,
    );
    let capture = host
        .copy_response_snapshot(&strip, &mut collector)
        .expect("a live snapshot copies");
    (strip, collector.finish(capture))
}

fn bits(values: &[f32]) -> Vec<u32> {
    values.iter().map(|value| value.to_bits()).collect()
}

#[test]
fn the_composer_names_every_member_in_signal_order_and_keeps_the_curves() {
    let (strip, snapshot) = snapshot();
    let response = compose_track_response(&strip, &snapshot, grid(POINTS), ResponseChannels::Both)
        .expect("the strip composes");

    let sections: Vec<_> = response
        .members
        .iter()
        .map(|member| (member.section.clone(), member.kind, member.bypassed))
        .collect();
    assert_eq!(
        sections,
        [
            (
                ResponseMemberSection::Builtins,
                ResponseMemberKind::InputFilters,
                false
            ),
            (
                ResponseMemberSection::Console("pre-eq".into()),
                ResponseMemberKind::ParametricEq,
                false
            ),
            (
                ResponseMemberSection::Inserts("insert-eq".into()),
                ResponseMemberKind::ParametricEq,
                true
            ),
            (
                ResponseMemberSection::Inserts("comp".into()),
                ResponseMemberKind::Unavailable,
                false
            ),
            (
                ResponseMemberSection::Console("post-eq".into()),
                ResponseMemberKind::ParametricEq,
                false
            ),
        ]
    );
    let native: Vec<_> = response
        .members
        .iter()
        .map(|member| &*member.native_id)
        .collect();
    assert_eq!(native[1..], [EQ, EQ, COMPRESSOR, EQ]);
    // Each mask is exactly the owner's enabled sections, bit `i` for section `i`; the bypassed
    // insert's band 2 is off and the console slots' band 2 is on.
    for (member, owner) in response.members.iter().zip(snapshot.owners.iter()) {
        let mask = |sections: &[effect_contract::ResponseSnapshotSection]| {
            sections
                .iter()
                .enumerate()
                .map(|(index, section)| u32::from(section.enabled) << index)
                .sum::<u32>()
        };
        assert_eq!(
            usize::from(member.section_count),
            owner.left.len(),
            "{member:?}"
        );
        assert_eq!(member.enabled_left, mask(&owner.left), "{member:?}");
        assert_eq!(member.enabled_right, mask(&owner.right), "{member:?}");
    }
    let band_mask = |member: &ResponseMember| member.enabled_left & 0b11;
    assert_eq!(band_mask(&response.members[1]), 0b11);
    assert_eq!(band_mask(&response.members[2]), 0b01);
    assert_eq!(response.members[2].enabled_right & 0b11, 0b01);
    assert_eq!(band_mask(&response.members[4]), 0b11);
    assert_eq!(response.excluded_count, 1);
    assert_eq!(response.floor_db, -120.0);
    assert_eq!(response.strip_id.as_ref(), strip);

    let mut frequencies = vec![0.0_f32; POINTS];
    let mut left = vec![0.0_f32; POINTS];
    let mut right = vec![0.0_f32; POINTS];
    let summary = query_response_snapshot_into(
        &snapshot,
        grid(POINTS),
        ResponseSnapshotOutput {
            frequencies_hz: &mut frequencies,
            total_left_db: &mut left,
            total_right_db: &mut right,
        },
    )
    .expect("the snapshot composes");
    assert_eq!(
        (response.captured_sample, response.sample_rate_hz),
        (summary.captured_sample, summary.sample_rate_hz)
    );
    assert_eq!(bits(&response.frequencies_hz), bits(&frequencies));
    assert_eq!(
        bits(response.left_db.as_deref().expect("left")),
        bits(&left)
    );
    assert_eq!(
        bits(response.right_db.as_deref().expect("right")),
        bits(&right)
    );
    assert_ne!(
        bits(&left),
        bits(&right),
        "the input filters are asymmetric"
    );

    let left_only = compose_track_response(&strip, &snapshot, grid(POINTS), ResponseChannels::Left)
        .expect("the left side composes");
    assert_eq!(left_only.right_db, None);
    assert_eq!(
        bits(left_only.left_db.as_deref().expect("left")),
        bits(&left)
    );
}

#[test]
fn the_composer_refuses_a_snapshot_it_cannot_interpret() {
    let (strip, snapshot) = snapshot();
    let compose = |snapshot: &ResponseSnapshot, points| {
        compose_track_response(&strip, snapshot, grid(points), ResponseChannels::Both)
            .map(|response| response.members.len())
    };
    assert_eq!(compose(&snapshot, RESPONSE_MAXIMUM_POINTS), Ok(5));
    assert_eq!(
        compose(&snapshot, RESPONSE_MAXIMUM_POINTS + 1),
        Err(ResponseSnapshotQueryError::InvalidGrid)
    );

    let mut foreign = snapshot.clone();
    let mut owners = foreign.owners.to_vec();
    owners[3].track_id = "another-strip".into();
    foreign.owners = owners.into_boxed_slice();
    assert_eq!(
        compose(&foreign, POINTS),
        Err(ResponseSnapshotQueryError::MalformedSnapshot)
    );

    let mut unknown_rack = snapshot.clone();
    let mut owners = unknown_rack.owners.to_vec();
    owners[1].rack = 5;
    unknown_rack.owners = owners.into_boxed_slice();
    assert_eq!(
        compose(&unknown_rack, POINTS),
        Err(ResponseSnapshotQueryError::MalformedSnapshot)
    );
}
