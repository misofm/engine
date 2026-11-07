//! Issue #2316 (#2258 FX-5): a block's ramping prefix is read from the ramp words themselves.

use super::*;
use effect_contract::{PrepareEffectLimits, PreparedPorts};

const LANES: usize = 4;
const BLOCK: u32 = 64;

/// A four-lane bank at the descriptor defaults, built directly so a test can reach its words.
fn bank() -> DeEsser<Simd4, false> {
    let values: Vec<InitialParameterValue> = (0..PARAMETER_COUNT * 2)
        .map(|index| InitialParameterValue {
            parameter_index: (index / 2) as u32,
            channel: if index % 2 == 0 {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: DE_ESSER_PARAMETERS[index / 2].default_value,
        })
        .collect();
    let quality = &QUALITIES[1];
    let request = PrepareEffectRequest {
        sample_rate: 48_000,
        quantum: 128,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::Unconnected {
                id: port_id("sidechain-in"),
                required: false,
            },
        },
        initial_values: &values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: quality.maximum_state.total().expect("state total"),
            maximum_scratch_bytes: 64,
            maximum_automation_spans_per_block: 32,
        },
        tail_bound: conformance::tail_bound_of(
            Box::new(DeEsserFactory),
            48_000,
            EffectQuality::Normal,
        ),
    };
    let metadata = expected_prepared_metadata(&DE_ESSER_DESCRIPTOR, request).expect("metadata");
    let defaults = [initial_defaults(&values).expect("defaults"); MAX_WIDTH];
    DeEsser::new(&metadata, Some(BankWidth::Four), defaults).expect("a bank")
}

/// Points one ramp's four words at `target` over `samples` frames, the way `LinearRamp` does.
fn glide([current, _, step, remaining]: [f32; 4], to: f32, samples: u32) -> [f32; 4] {
    let mut ramp = LinearRamp {
        current,
        target: to,
        step,
        remaining: remaining as u32,
    };
    ramp.set_target(to, samples);
    [ramp.current, ramp.target, ramp.step, ramp.remaining as f32]
}

/// `blocks` blocks of `+0.0` with no automation.
fn render(bank: &mut DeEsser<Simd4, false>, blocks: u32) {
    let offsets = [0_u32; LANES + 1];
    for block in 0..blocks {
        let mut left = [0.0_f32; BLOCK as usize * LANES];
        let mut right = left;
        PreparedNativeEffectBank::process_bank(
            bank,
            EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                BLOCK,
                BankWidth::Four,
                u64::from(block * BLOCK),
                &[],
                &offsets,
                128,
            )
            .expect("a block"),
        );
    }
}

/// A ramp that a writer starts without announcing it to anything still arrives on its target.
///
/// Two plants, each on a fresh bank and so each the bank's only ramp: the threshold ramp on
/// lane 2's right channel, then on lane 1's left channel, pointed 10 dB down over 64 frames
/// straight in the lane words, with nothing else told; one 64-frame block must land each. Red if
/// the ramping prefix is sized by anything but the words in force at the block's start: a
/// hand-kept bound that the writer did not raise (the shape before #2316, red there), or a
/// derivation that reads one channel alone on a dual block, either one. Each leaves the ramp
/// frozen mid-flight on the settled segment, which advances no ramp. A derivation that skips the
/// link weights, or every lane but the first, is already red in `glide_tests`.
#[test]
fn a_ramp_no_writer_announced_still_arrives() {
    for (channel, lane) in [(1, 2), (0, 1)] {
        let mut planted = bank();
        let at = SIGNAL_WORDS + kernel::THRESHOLD * 4;
        let mut words = lane_words(&planted.state[channel], lane);
        let target = words[at] - 10.0;
        let chunk: [f32; 4] = words[at..at + 4].try_into().expect("a ramp");
        words[at..at + 4].copy_from_slice(&glide(chunk, target, RAMP_SAMPLES));
        set_lane_words(&mut planted.state[channel], lane, &words);
        render(&mut planted, 1);
        let words = lane_words(&planted.state[channel], lane);
        let arrived: [f32; 4] = words[at..at + 4].try_into().expect("a ramp");
        assert_eq!(
            arrived.map(f32::to_bits),
            [target, target, 0.0, 0.0].map(f32::to_bits),
            "channel {channel}'s threshold ramp on lane {lane} froze in flight"
        );
    }
}
