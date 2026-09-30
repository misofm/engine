//! Issue #1096 (S1c): a native live control reaches the right lane when it addresses a console slot
//! by its slot index or an insert by its index.
//!
//! The session is the EQ bank console widened to two `pre_insert` slots, one `post_insert` slot and
//! one insert on each of its eight tracks, every instance a parametric EQ at its own frequency and a
//! gain that differs by track, so bypassing any one instance on any one track moves the mix in its
//! own way. Every track carries every slot, so the console slots bank.
//!
//! - A live bypass on `(track, address)` renders exactly what the same document renders with that
//!   instance's session `bypass` set. A session bypass lowers to the same per-lane shunt a live
//!   bypass engages (issue #1087), and the shunt selects whole blocks, so the two are bit-identical
//!   when the record is drained before the first block.
//! - A live parameter change on `(track, address)` renders exactly what the same record renders when
//!   it is pushed to the channel whose `effect_id` names the intended instance, and differs from the
//!   base render.
//!
//! Red mutations (each turns `a_live_bypass_reaches_the_addressed_lane` and
//! `a_live_parameter_reaches_the_addressed_lane` red):
//! - in `effect_compiler::declared_live_addresses`, give `post_insert` the base `0` instead of the
//!   `pre_insert` length, which addresses the `post_insert` slot as `pre_insert` slot 0: no channel
//!   answers `console(2)`;
//! - index the inserts from the console's slot count: no channel answers `insert(0)`.

use core::num::{NonZeroU32, NonZeroUsize};

use builtins::MeterTap;
use effect_contract::{EffectControlRecord, ParameterChannel};
use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use host_core::{
    HostLiveControlHandles, HostLiveControlRequest, HostPrepareCaps, HostShapePolicy,
    LiveEffectAddress, PreparedHost, SourceSubmission, prepare_host_session_with_live_controls,
};
use session::{
    ConsoleEntry, ConsoleSlot, Effect, EffectParam, SidechainDeclaration, StableId,
    canonical_session_json, parse_session_json,
};

mod support;

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-bank-console.json");
const QUANTUM: usize = 128;
const BLOCKS: usize = 8;
/// `band-1-gain` is parameter id 4 of `miso.parametric-eq`, index 3 of its table.
const BAND_GAIN_INDEX: u32 = 3;
/// The track the tests address: not the first, so a track off-by-one cannot pass.
const TRACK: &str = "eq5";

/// `(address, the instance it names on every track)`, from the session's own slot order:
/// `pre_insert` [eq, pre-eq2], then `post_insert` [post-eq], and the inserts [ins-eq].
const ADDRESSES: [(LiveEffectAddress, &str); 4] = [
    (LiveEffectAddress::console(0), "eq"),
    (LiveEffectAddress::console(1), "pre-eq2"),
    (LiveEffectAddress::console(2), "post-eq"),
    (LiveEffectAddress::insert(0), "ins-eq"),
];

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 100_000_000,
        maximum_source_total_bytes: 10_000_000,
        maximum_source_overhead_bytes: 10_000_000,
        maximum_effect_state_bytes: 100_000_000,
        maximum_effect_scratch_bytes: 100_000_000,
        maximum_builtin_retained_bytes: 100_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        maximum_meter_streams: 64,
        maximum_meter_items: 1 << 16,
        maximum_meter_bytes: 1 << 24,
    }
}

fn live_controls() -> HostLiveControlRequest {
    HostLiveControlRequest {
        control_queue_depth: Some(NonZeroUsize::new(8).expect("depth")),
        meter_period_frames: Some(NonZeroU32::new(QUANTUM as u32).expect("period")),
        meter_queue_depth: NonZeroUsize::new(16).expect("meter depth"),
        meter_tap: MeterTap::PostMatrix,
        observation_taps: 0,
        master_track: None,
    }
}

/// The EQ's params with band 1 moved to `frequency_hz` and `gain_db`.
fn eq_params(template: &[EffectParam], frequency_hz: f32, gain_db: f32) -> Vec<EffectParam> {
    template
        .iter()
        .map(|param| {
            let mut param = param.clone();
            match param.parameter_id {
                3 => param.value = frequency_hz,
                4 => param.value = gain_db,
                _ => {}
            }
            param
        })
        .collect()
}

/// The addressing session, with `bypassed` (a track and an instance) set to session `bypass`.
fn document(bypassed: Option<(&str, &str)>) -> String {
    let mut model = parse_session_json(SESSION).expect("EQ bank console fixture");
    let template = model.console.pre_insert[0].clone();
    let slot = |id: &str| ConsoleSlot {
        slot: StableId::parse(id).expect("slot id"),
        ..template.clone()
    };
    model.console.pre_insert = vec![slot("eq"), slot("pre-eq2")];
    model.console.post_insert = vec![slot("post-eq")];
    for (index, track) in model.tracks.iter_mut().enumerate() {
        let spread = index as f32;
        let params = track.console[0].params.clone();
        let entry = |id: &str, frequency_hz: f32, gain_db: f32| ConsoleEntry {
            slot: StableId::parse(id).expect("slot id"),
            bypass: false,
            params: eq_params(&params, frequency_hz, gain_db),
        };
        track.console = vec![
            entry("eq", 1_000.0, 1.0 + spread),
            entry("pre-eq2", 300.0, -2.0 - spread),
            entry("post-eq", 5_000.0, 3.0 + spread),
        ];
        track.inserts.effects = vec![Effect {
            id: StableId::parse("ins-eq").expect("insert id"),
            identity: template.identity.clone(),
            quality: template.quality,
            bypass: false,
            link_mode: template.link_mode,
            params: eq_params(&params, 150.0, 12.0 - spread),
            sidechain: SidechainDeclaration::None,
        }];
        if let Some((track_id, instance)) = bypassed
            && track.id.as_str() == track_id
        {
            let mut found = false;
            for entry in &mut track.console {
                if entry.slot.as_str() == instance {
                    entry.bypass = true;
                    found = true;
                }
            }
            for effect in &mut track.inserts.effects {
                if effect.id.as_str() == instance {
                    effect.bypass = true;
                    found = true;
                }
            }
            assert!(found, "{track_id} carries {instance}");
        }
    }
    canonical_session_json(&model).expect("canonical addressing session")
}

fn prepare(document: &str) -> (PreparedHost, HostLiveControlHandles) {
    let (_session, prepared, handles) =
        prepare_host_session_with_live_controls(document, &caps(), &live_controls())
            .unwrap_or_else(|failure| {
                panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
            });
    assert_eq!(handles.tracks.len(), 8);
    assert_eq!(handles.effect_controls.len(), 8 * ADDRESSES.len());
    // The eight identical chains bank, so a live bypass here is a lane of a bank, not a node.
    assert!(
        prepared.report.effect_bank_scratch_bytes > 0,
        "the cohort planner bound at least one homogeneous bank on this host"
    );
    (prepared, handles)
}

/// A deterministic broadband block, so every EQ band is audible.
fn noise(block: usize, channel: u32) -> [f32; QUANTUM] {
    let mut state = (block as u32).wrapping_mul(0x9e37_79b9) ^ channel.wrapping_mul(0x85eb_ca6b);
    core::array::from_fn(|_| {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        ((state >> 8) as f32 / (1_u32 << 24) as f32 - 0.5) * 0.5
    })
}

/// Render [`BLOCKS`] quanta and return every output sample's bits.
fn render(prepared: &mut PreparedHost) -> Vec<u32> {
    let mut bits = Vec::with_capacity(BLOCKS * QUANTUM * 2);
    for block in 0..BLOCKS {
        let (left, right) = (noise(block, 0), noise(block, 1));
        prepared
            .sources
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 1,
                    start_frame: (block * QUANTUM) as u64,
                    sample_rate_hz: 48_000,
                    planes: &[&left, &right],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut samples = [0.0_f32; QUANTUM * 2];
        let output =
            PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM).expect("output planes");
        prepared
            .plan
            .render(
                RenderIo { output },
                RenderTime {
                    absolute_sample: (block * QUANTUM) as u64,
                },
            )
            .expect("render");
        bits.extend(samples.iter().map(|sample| sample.to_bits()));
    }
    bits
}

/// The channel the session address names, checked against the instance the address must name.
fn addressed<'a>(
    handles: &'a mut HostLiveControlHandles,
    address: LiveEffectAddress,
    instance: &str,
) -> &'a mut host_core::EffectControlProducer {
    let producer = handles
        .effect_control_mut(TRACK, address)
        .unwrap_or_else(|| panic!("{TRACK} has a channel at {address:?}"));
    assert_eq!(
        &*producer.effect_id, instance,
        "{address:?} names {instance}"
    );
    producer
}

#[test]
fn a_live_bypass_reaches_the_addressed_lane() {
    let (mut base, _) = prepare(&document(None));
    let base = render(&mut base);
    let mut renders = Vec::new();
    for (address, instance) in ADDRESSES {
        let (mut prepared, mut handles) = prepare(&document(None));
        addressed(&mut handles, address, instance)
            .try_push(EffectControlRecord::Bypass(true))
            .expect("room in the bounded queue");
        let live = render(&mut prepared);
        let (mut oracle, _oracle_handles) = prepare(&document(Some((TRACK, instance))));
        let oracle = render(&mut oracle);
        assert!(
            live == oracle,
            "a live bypass at {address:?} renders the session bypass of {TRACK}/{instance}"
        );
        assert!(live != base, "bypassing {TRACK}/{instance} is audible");
        drop(handles);
        renders.push(live);
    }
    for (index, render) in renders.iter().enumerate() {
        for later in &renders[index + 1..] {
            assert!(
                render != later,
                "each addressed instance moves the mix in its own way, so a wrong address shows"
            );
        }
    }
}

#[test]
fn a_live_parameter_reaches_the_addressed_lane() {
    let (mut base, _) = prepare(&document(None));
    let base = render(&mut base);
    let ride = |producer: &mut host_core::EffectControlProducer| {
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            support::push_eq_parameter(producer, BAND_GAIN_INDEX, channel, -18.0);
        }
    };
    let mut renders = Vec::new();
    for (address, instance) in ADDRESSES {
        let (mut prepared, mut handles) = prepare(&document(None));
        ride(addressed(&mut handles, address, instance));
        let live = render(&mut prepared);
        // The same ride, sent to the channel found by the instance's own identity.
        let (mut oracle, mut oracle_handles) = prepare(&document(None));
        ride(
            oracle_handles
                .effect_controls
                .iter_mut()
                .find(|producer| &*producer.track_id == TRACK && &*producer.effect_id == instance)
                .expect("a channel for the instance"),
        );
        let oracle = render(&mut oracle);
        assert!(
            live == oracle,
            "a live ride at {address:?} reaches {TRACK}/{instance}"
        );
        assert!(live != base, "riding {TRACK}/{instance} is audible");
        drop((handles, oracle_handles));
        renders.push(live);
    }
    for (index, render) in renders.iter().enumerate() {
        for later in &renders[index + 1..] {
            assert!(
                render != later,
                "each addressed instance rides its own lane"
            );
        }
    }
}

/// Every channel's address reads back against the session itself: console slot `k` is the
/// track's `console[k]` entry and insert `i` is `inserts.effects[i]`, on every track.
#[test]
fn every_channel_is_addressed_by_its_position_in_the_session() {
    let document = document(None);
    let model = parse_session_json(&document).expect("addressing session");
    let (_prepared, handles) = prepare(&document);
    for producer in &handles.effect_controls {
        let track = model
            .tracks
            .iter()
            .find(|track| track.id.as_str() == &*producer.track_id)
            .expect("the channel's track");
        let index = producer.address.index as usize;
        let named = match producer.address.rack {
            host_core::LiveEffectRack::Console => track.console[index].slot.as_str(),
            host_core::LiveEffectRack::Inserts => track.inserts.effects[index].id.as_str(),
        };
        assert_eq!(named, &*producer.effect_id, "{:?}", producer.address);
    }
}
