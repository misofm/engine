//! Issue #140 A: the automation-span feed reaches a **banked** effect, per lane.
//!
//! `hosts/miso-engine-host-web` proves the boundary for a per-node dynamic-rack effect, and
//! `miso-engine-rack` proves the lane partition against a mock bank. What neither of them proves
//! is the seam between them: that a real cohort-planned homogeneous bank, bound by the graph
//! compiler from a real session, hands lane `l` the spans of lane `l`'s own track and nothing
//! else.
//!
//! The fixture is eight identical SIMD-rack chains, so the planner forms a full bank at every
//! launch bank width, and a post-matrix meter per track is the per-lane observation point.

use core::num::{NonZeroU32, NonZeroUsize};

use miso_engine_builtins::MeterTap;
use miso_engine_core::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use miso_engine_effect_contract::{EffectControlRecord, ParameterChannel};
use miso_engine_host_core::{
    EffectRack, HostConsoleRequest, HostPrepareCaps, HostShapePolicy, PreparedHost,
    SourceSubmission, prepare_host_session_with_console,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-bank-console.toml");
const QUANTUM: usize = 128;
const TRACKS: usize = 8;
/// `band-1-gain` is parameter id 4 of `miso.parametric-eq`, which is index 3 of its table.
const BAND_GAIN_INDEX: u32 = 3;

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

fn console() -> HostConsoleRequest {
    HostConsoleRequest {
        control_queue_depth: Some(NonZeroUsize::new(8).expect("depth")),
        meter_period_frames: Some(NonZeroU32::new(QUANTUM as u32).expect("period")),
        meter_queue_depth: NonZeroUsize::new(16).expect("meter depth"),
        meter_tap: MeterTap::PostMatrix,
        observation_taps: 0,
        master_track: None,
    }
}

/// One prepared console session, its per-track meter consumers, and its effect producers.
struct Console {
    prepared: PreparedHost,
    handles: miso_engine_host_core::HostConsoleHandles,
    /// Absolute block cursor, so successive `render` calls stay contiguous in the source ring.
    block: usize,
}

fn prepare() -> Console {
    let (_session, prepared, handles) =
        prepare_host_session_with_console(SESSION, &caps(), &console()).unwrap_or_else(|failure| {
            panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
        });
    assert_eq!(handles.tracks.len(), TRACKS);
    assert_eq!(
        handles.effect_controls.len(),
        TRACKS,
        "one control channel per prepared effect instance"
    );
    // The whole point of this file: the eight chains actually banked. If the planner had left them
    // on the per-node scalar path, these tests would silently prove nothing about a bank.
    assert!(
        prepared.report.effect_bank_scratch_bytes > 0,
        "the cohort planner bound at least one homogeneous bank on this host"
    );
    Console {
        prepared,
        handles,
        block: 0,
    }
}

/// Renders `blocks` quanta of a constant signal and returns each track's post-matrix peak per
/// block, `[track][block]`.
fn render(console: &mut Console, blocks: usize) -> Vec<Vec<u32>> {
    let mut peaks = vec![Vec::new(); TRACKS];
    let left = [0.25_f32; QUANTUM];
    let right = [0.25_f32; QUANTUM];
    for step in 0..blocks {
        let block = console.block + step;
        console
            .prepared
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
        console
            .prepared
            .plan
            .render(
                RenderIo {
                    input: None,
                    output,
                },
                RenderTime {
                    absolute_sample: (block * QUANTUM) as u64,
                },
            )
            .expect("render");
        for (track, meter) in console.handles.meters.iter_mut().enumerate() {
            let snapshot = meter.consumer.try_pop().expect("one window per block");
            peaks[track].push(snapshot.left.sample_peak.to_bits());
        }
    }
    console.block += blocks;
    peaks
}

/// Pushes one band-gain retarget into the addressed track's simd1 effect channel.
fn command(console: &mut Console, track_id: &str, value: f32) {
    let producer = console
        .handles
        .effect_controls
        .iter_mut()
        .find(|producer| {
            producer.track_id.as_ref() == track_id
                && producer.rack == EffectRack::Simd1
                && producer.effect_index == 0
        })
        .expect("a control channel for the addressed effect");
    for channel in [ParameterChannel::Left, ParameterChannel::Right] {
        producer
            .producer
            .try_push(EffectControlRecord::Parameter {
                parameter_index: BAND_GAIN_INDEX,
                channel,
                value,
            })
            .expect("room in the bounded queue");
    }
}

/// Red mutation: in `ConsoleEffectBankStage::process`, pack every lane at `packed[..staged]`
/// instead of at that lane's own running offset -> lane 0 renders lane 2's command and the
/// "every other track is bit-identical" assertion fails.
#[test]
fn a_banked_effect_applies_each_lanes_own_command_and_no_others() {
    let mut control = prepare();
    let mut commanded = prepare();

    // Two blocks with no traffic: the two sessions are bit-identical, which is the baseline the
    // "an idle console changes nothing" claim rests on.
    let before_control = render(&mut control, 2);
    let before_commanded = render(&mut commanded, 2);
    assert_eq!(
        before_control, before_commanded,
        "an idle console renders the identical peaks"
    );

    command(&mut commanded, "eq2", -24.0);
    let after_control = render(&mut control, 4);
    let after_commanded = render(&mut commanded, 4);

    for track in 0..TRACKS {
        let id = commanded.handles.tracks[track].as_ref();
        if id == "eq2" {
            assert_ne!(
                after_control[track], after_commanded[track],
                "the commanded lane moved"
            );
        } else {
            assert_eq!(
                after_control[track], after_commanded[track],
                "track {id} was never addressed and must be bit-identical"
            );
        }
    }
}

/// Two lanes of the same bank take two different commands in the same block, and each gets its
/// own: a bank is not one instance with one parameter set.
#[test]
fn two_lanes_of_one_bank_take_two_different_commands() {
    let mut low = prepare();
    let mut high = prepare();
    let mut both = prepare();
    command(&mut low, "eq1", -24.0);
    command(&mut high, "eq5", 18.0);
    command(&mut both, "eq1", -24.0);
    command(&mut both, "eq5", 18.0);

    let low_peaks = render(&mut low, 4);
    let high_peaks = render(&mut high, 4);
    let both_peaks = render(&mut both, 4);

    for track in 0..TRACKS {
        let id = both.handles.tracks[track].as_ref();
        let expected = match id {
            "eq1" => &low_peaks[track],
            "eq5" => &high_peaks[track],
            _ => &low_peaks[track],
        };
        assert_eq!(
            &both_peaks[track], expected,
            "track {id}: each lane carries exactly the command addressed to it"
        );
    }
    assert_ne!(
        low_peaks[1], high_peaks[1],
        "the two commands are distinguishable at all"
    );
}
