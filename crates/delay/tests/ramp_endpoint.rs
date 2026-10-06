//! Issue #1409 gate 2: every ramped word of this effect stays inside its endpoints.
//!
//! The render site: site 7 (`delay_chunk`, fed by `LaneChunk` and `CrossChunk`): each lane's
//! feedback, damping coefficient `g` and mix, and the shared cross feedback. Each word has one
//! in-domain move, found by the issue's scan over the parameter's domain edges (for a designed
//! word, over parameter pairs whose words are 33 to 4096 ulps apart), whose word the unclamped D11
//! law (`current + step` before the snap) took past its target. Every move runs at the 1 ms delay
//! time, so the taps carry signal inside the ramp window: feedback, damping `g` and cross feedback
//! reach the ring (and the damping state), and mix reaches the output. The move is rendered in
//! one-frame blocks; after each, the effect's own state snapshot must hold every ramp word between
//! its value at rest and its target. The same window rendered as one block must give the same ramp
//! words, output and final snapshot.
//!
//! Test value: a render site left on (or reverted to) the unclamped update passes its target on its
//! move; the partition half catches a site clamped in one block shape only. Mutation evidence is in
//! the issue's attempt record.

use effect_contract::EffectProcessBlock;
use effect_contract::{
    AutomationSpanKind, InitialParameterValue, NativeEffectFactory, ParameterChannel,
    ParameterChannelPolicy, PortRole, PrepareEffectLimits, PrepareEffectRequest,
    PreparedAutomationSpan, PreparedNativeEffect, PreparedPorts, PreparedSidechainPort,
    ProcessReport, StatePayloadOutput, StatePayloadSizes, default_initial_values,
};

/// The payload section that holds a ramp.
#[derive(Clone, Copy, Debug)]
enum Section {
    /// The common section (a shared parameter).
    Common,
    /// Both channel sections, the same word in each (a per-lane parameter).
    Channels,
}

/// One ramped word: the parameter that moves it, and where its four payload words start.
#[derive(Clone, Copy, Debug)]
struct RampWord {
    name: &'static str,
    parameter: u32,
    section: Section,
    word: usize,
}

/// One in-domain move of the probe's scan: `parameter` (of `word`) from `start` to `target`, whose
/// word the unclamped law took past its target before the snap.
#[derive(Clone, Copy, Debug)]
struct Move {
    word: RampWord,
    start: f32,
    target: f32,
    /// Whether the move's output and whole snapshot are partition-invariant. A word that feeds a
    /// coefficient the effect refreshes once per segment is not always, by that effect's frozen
    /// design (the multiband compressor's high ratio and high attack moves); there only the ramp
    /// words are compared.
    whole: bool,
}

/// Frames rendered per move: the 64-sample window and a few settled frames after it.
const FRAMES: usize = 72;

/// The preparation request: the 48 kHz quality row (`qualities[1]`, the rate the moves were
/// scanned at), quantum 128, the dual-mono link mode, the sidechain connected when `connected` and
/// the effect has one.
fn request<'a>(
    factory: &dyn NativeEffectFactory,
    values: &'a [InitialParameterValue],
    connected: bool,
) -> PrepareEffectRequest<'a> {
    let descriptor = factory.descriptor();
    let quality = descriptor.qualities[1];
    assert_eq!(quality.sample_rate, 48_000, "the moves are 48 kHz moves");
    let sidechain = descriptor
        .ports
        .iter()
        .find(|port| port.role == PortRole::SidechainInput)
        .map_or(PreparedSidechainPort::None, |port| {
            if connected {
                PreparedSidechainPort::Connected {
                    id: port.id,
                    required: port.required,
                }
            } else {
                PreparedSidechainPort::Unconnected {
                    id: port.id,
                    required: port.required,
                }
            }
        });
    let link = effect_contract::LinkMode::DualMono;
    assert!(descriptor.supported_link_modes.contains(link));
    PrepareEffectRequest {
        sample_rate: quality.sample_rate,
        quantum: 128,
        quality: quality.quality,
        bypass: false,
        link_mode: link,
        ports: PreparedPorts { sidechain },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: quality.maximum_state.total().unwrap_or(u64::MAX),
            maximum_scratch_bytes: (quality.scratch_bytes_per_frame * 128
                + quality.scratch_fixed_bytes)
                .max(1),
            maximum_automation_spans_per_block: 4,
        },
    }
}

type Payload = (Vec<u8>, Vec<u8>, Vec<u8>);

/// Snapshots `effect`, whose prepare result reported the state sizes `sizes`.
fn snapshot(effect: &dyn PreparedNativeEffect, sizes: StatePayloadSizes) -> Payload {
    let mut common = vec![0_u8; sizes.common_bytes as usize];
    let mut left = vec![0_u8; sizes.left_bytes as usize];
    let mut right = vec![0_u8; sizes.right_bytes as usize];
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("sizes"),
        )
        .expect("a prepared instance snapshots");
    (common, left, right)
}

fn word_u32(bytes: &[u8], word: usize) -> u32 {
    u32::from_le_bytes(
        bytes[word * 4..word * 4 + 4]
            .try_into()
            .expect("four bytes"),
    )
}

fn word_f32(bytes: &[u8], word: usize) -> f32 {
    f32::from_bits(word_u32(bytes, word))
}

/// The `(current, target, step, remaining)` of `ramp` in each section that holds it.
fn ramps(payload: &Payload, ramp: RampWord) -> Vec<(f32, f32, f32, u32)> {
    let sections: Vec<&Vec<u8>> = match ramp.section {
        Section::Common => vec![&payload.0],
        Section::Channels => vec![&payload.1, &payload.2],
    };
    sections
        .into_iter()
        .map(|bytes| {
            (
                word_f32(bytes, ramp.word),
                word_f32(bytes, ramp.word + 1),
                word_f32(bytes, ramp.word + 2),
                word_u32(bytes, ramp.word + 3),
            )
        })
        .collect()
}

/// The delay time (parameter 0) every move starts at: its 1 ms minimum, 48 samples at 48 kHz. The
/// taps then carry signal from frame 48, inside the 64-sample ramp, so feedback, damping `g` and
/// cross feedback reach the ring while they move (their writes are read back 48 frames later,
/// after the window, so a defect in them shows in the final snapshot, not the window's output).
/// At the 250 ms default the taps stay silent for the whole window, and only the mix word, which
/// reaches the output, would be exercised.
const DELAY_TIME_MS: f32 = 1.0;

/// The descriptor defaults, with the delay time at [`DELAY_TIME_MS`] and `parameter` at `value`,
/// on every channel.
fn values_with(
    factory: &dyn NativeEffectFactory,
    parameter: u32,
    value: f32,
) -> Vec<InitialParameterValue> {
    default_initial_values(factory.descriptor())
        .map(|mut initial| {
            if initial.parameter_index == 0 {
                initial.value = DELAY_TIME_MS;
            }
            if initial.parameter_index == parameter {
                initial.value = value;
            }
            initial
        })
        .collect()
}

/// `Point` spans moving `parameter` to `value` at sample 0, on every channel it has.
fn move_spans(
    factory: &dyn NativeEffectFactory,
    parameter: u32,
    value: f32,
) -> Vec<PreparedAutomationSpan> {
    let channels: &[ParameterChannel] =
        match factory.descriptor().parameters[parameter as usize].channel_policy {
            ParameterChannelPolicy::Shared => &[ParameterChannel::Both],
            ParameterChannelPolicy::PerLane => &[ParameterChannel::Left, ParameterChannel::Right],
        };
    channels
        .iter()
        .map(|&channel| PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel,
            parameter_index: parameter,
            start_sample: 0,
            end_sample: 0,
            start_value: value,
            end_value: value,
        })
        .collect()
}

/// A deterministic test signal.
fn signal(frame: usize, channel: usize) -> f32 {
    ((frame * 37 + channel * 11) % 97) as f32 / 97.0 - 0.5
}

fn render(
    effect: &mut dyn PreparedNativeEffect,
    first: usize,
    frames: usize,
    spans: &[PreparedAutomationSpan],
    connected: bool,
) -> (Vec<f32>, Vec<f32>, ProcessReport) {
    let mut left: Vec<f32> = (first..first + frames)
        .map(|frame| signal(frame, 0))
        .collect();
    let mut right: Vec<f32> = (first..first + frames)
        .map(|frame| signal(frame, 1))
        .collect();
    let side: Vec<f32> = (first..first + frames)
        .map(|frame| 0.7 * signal(frame, 2))
        .collect();
    let block = EffectProcessBlock::new(
        &mut left,
        &mut right,
        connected.then_some((&side[..], &side[..])),
        first as u64,
        spans,
        128,
    )
    .expect("a well-shaped block");
    let report = effect.process(block);
    (left, right, report)
}

/// The first frame (1-based) at which the unclamped law's word `w0 + step + ... + step` leaves
/// `[min(w0, target), max(w0, target)]` before the snap of a `samples`-sample ramp.
fn unclamped_first_out(w0: f32, target: f32, step: f32, samples: u32) -> Option<u32> {
    let (low, high) = (w0.min(target), w0.max(target));
    let mut word = w0;
    (1..samples).find(|_| {
        word += step;
        !(low..=high).contains(&word)
    })
}

/// Renders `mv` in one-frame blocks and checks, after every frame, that every ramp word of `words`
/// lies between its value at rest and its target; checks that the unclamped law would have taken
/// the moved word past its target (so the move reaches the clamp); and checks that the same window
/// rendered as one block gives the same output and final snapshot.
fn check_move(factory: &dyn NativeEffectFactory, mv: Move, words: &[RampWord], connected: bool) {
    let context = format!(
        "{} from {:e} ({:#010x}) to {}, sidechain connected {connected}",
        mv.word.name,
        mv.start,
        mv.start.to_bits(),
        mv.target
    );
    let samples = factory.descriptor().parameters[mv.word.parameter as usize].smoothing_samples;
    let values = values_with(factory, mv.word.parameter, mv.start);
    let spans = move_spans(factory, mv.word.parameter, mv.target);
    let prepared = factory
        .prepare(request(factory, &values, connected))
        .expect("prepares");
    let sizes = prepared.metadata.state_sizes;
    let mut framewise = prepared.processor;
    let rest: Vec<_> = words
        .iter()
        .map(|word| ramps(&snapshot(framewise.as_ref(), sizes), *word))
        .collect();
    let (mut left, mut right) = (Vec::new(), Vec::new());
    for frame in 0..FRAMES {
        let first = if frame == 0 { &spans[..] } else { &[] };
        let (l, r, _) = render(framewise.as_mut(), frame, 1, first, connected);
        left.extend(l);
        right.extend(r);
        let now = snapshot(framewise.as_ref(), sizes);
        for (word, rest) in words.iter().zip(&rest) {
            for (section, (rest, now)) in rest.iter().zip(ramps(&now, *word)).enumerate() {
                let (low, high) = (rest.0.min(now.1), rest.0.max(now.1));
                assert!(
                    (low..=high).contains(&now.0),
                    "{context}: after frame {frame}, `{}` (section {section}) is {:e} \
                     ({:#010x}), outside [{low:e}, {high:e}]",
                    word.name,
                    now.0,
                    now.0.to_bits()
                );
            }
        }
        if frame == 0 {
            let moved = words
                .iter()
                .position(|word| word.word == mv.word.word && word.name == mv.word.name)
                .expect("the moved word is in the table");
            for (section, (rest, now)) in rest[moved].iter().zip(ramps(&now, mv.word)).enumerate() {
                assert!(
                    unclamped_first_out(rest.0, now.1, now.2, samples).is_some(),
                    "{context}: section {section}: the unclamped law stays inside, so this move \
                     does not reach the clamp"
                );
            }
        }
    }
    let mut whole = factory
        .prepare(request(factory, &values, connected))
        .expect("prepares")
        .processor;
    let (whole_left, whole_right, _) = render(whole.as_mut(), 0, FRAMES, &spans, connected);
    let (whole_state, framewise_state) = (
        snapshot(whole.as_ref(), sizes),
        snapshot(framewise.as_ref(), sizes),
    );
    for word in words {
        assert_eq!(
            ramps(&whole_state, *word),
            ramps(&framewise_state, *word),
            "{context}: `{}` ends on a partition-dependent ramp",
            word.name
        );
    }
    if mv.whole {
        let bits = |samples: &[f32]| samples.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
        assert_eq!(bits(&whole_left), bits(&left), "{context}: left output");
        assert_eq!(bits(&whole_right), bits(&right), "{context}: right output");
        assert!(
            whole_state == framewise_state,
            "{context}: the final snapshot depends on the partition"
        );
    }
}

/// Every ramp word of this effect's payload.
const WORDS: [RampWord; 4] = [
    RampWord {
        name: "feedback",
        parameter: 1,
        section: Section::Channels,
        word: 7,
    },
    RampWord {
        name: "damping coefficient",
        parameter: 2,
        section: Section::Channels,
        word: 11,
    },
    RampWord {
        name: "mix",
        parameter: 3,
        section: Section::Channels,
        word: 15,
    },
    RampWord {
        name: "cross feedback",
        parameter: 4,
        section: Section::Common,
        word: 1,
    },
];

/// One overshooting move per word (#1409 gate 2, recorded in the issue).
const MOVES: [Move; 4] = [
    Move {
        word: WORDS[0],
        start: f32::from_bits(0x3f733312),
        target: 0.95,
        whole: true,
    },
    Move {
        word: WORDS[1],
        start: f32::from_bits(0x3e800021),
        target: 0.25,
        whole: true,
    },
    Move {
        word: WORDS[2],
        start: f32::from_bits(0x3f7fffa0),
        target: 1.0,
        whole: true,
    },
    Move {
        word: WORDS[3],
        start: f32::from_bits(0x3f7fffa0),
        target: 1.0,
        whole: true,
    },
];

#[test]
fn every_ramped_word_stays_inside_its_endpoints() {
    for connected in [false] {
        for mv in MOVES {
            check_move(&delay::DelayFactory, mv, &WORDS, connected);
        }
    }
}
