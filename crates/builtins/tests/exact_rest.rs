#![allow(missing_docs)]
//! Issue #1328 gate 1: a near-Nyquist input filter reaches exact rest after its input stops.
//!
//! Where the input LPF's `c1` rounds to `1.0f32` (from about 22,047.6 Hz at 44.1 kHz up to the
//! domain maximum), a per-word flush of the two integrators leaves a period-2 limit cycle: `ic1`
//! alternates in sign between about `1e-20` and `1.35e-16` and the output rings near `±2e-21`
//! forever, from sample 858,748 for this exact impulse. The joint SVF flush (`lane::flush_pair`)
//! zeroes the pair once both words are below `REST_EPS` on a sample after the chain's input has
//! been exactly zero for `N_SILENCE` samples (amendment A9), so after the impulse every integrator
//! reaches `+0.0` and the output reaches exactly `+0.0`.

use builtins::test_support::{chain_input, input_state_words};
use builtins::{BuiltinChain, BuiltinParameters, DualMonoBlock, builtin_filter_cutoff_maximum_hz};

const RATE: u32 = 44_100;
const FRAMES: usize = 128;
/// The bound the issue states: rest within 2,400,000 samples of the impulse.
const REST_BOUND_SAMPLES: usize = 2_400_000;
/// Blocks rendered after rest that must stay at rest.
const HELD_BLOCKS: usize = 4;

fn at_rest(chain: &BuiltinChain, left: &[f32], right: &[f32]) -> bool {
    input_state_words(chain_input(chain)) == [0; 8]
        && left.iter().chain(right).all(|sample| sample.to_bits() == 0)
}

#[test]
fn the_input_lpf_at_the_cutoff_maximum_reaches_exact_rest() {
    let cutoff = builtin_filter_cutoff_maximum_hz(RATE).expect("a launch rate");
    let mut parameters = BuiltinParameters::default();
    parameters.left.lpf_hz = cutoff;
    parameters.right.lpf_hz = cutoff;
    let mut chain = BuiltinChain::new(RATE, parameters).expect("the domain maximum prepares");

    let mut left = [0.0_f32; FRAMES];
    let mut right = [0.0_f32; FRAMES];
    left[0] = 1234.5;
    right[0] = 1234.5;
    let mut rest_at = None;
    let mut last_peak = 0.0_f32;
    let mut block = 0_usize;
    while block * FRAMES < REST_BOUND_SAMPLES {
        let first_sample = (block * FRAMES) as u64;
        chain.process_dual_mono(
            DualMonoBlock::new(&mut left, &mut right, first_sample).expect("block"),
        );
        if block > 0 && at_rest(&chain, &left, &right) {
            rest_at = Some(block);
            break;
        }
        last_peak = left
            .iter()
            .chain(&right)
            .fold(0.0, |peak, x| peak.max(x.abs()));
        left.fill(0.0);
        right.fill(0.0);
        block += 1;
    }
    let rest_at = rest_at.unwrap_or_else(|| {
        panic!(
            "no exact rest within {REST_BOUND_SAMPLES} samples: state {:08x?}, last block's \
             output peak {last_peak:e}",
            input_state_words(chain_input(&chain)),
        )
    });
    for held in 1..=HELD_BLOCKS {
        left.fill(0.0);
        right.fill(0.0);
        let first_sample = ((rest_at + held) * FRAMES) as u64;
        chain.process_dual_mono(
            DualMonoBlock::new(&mut left, &mut right, first_sample).expect("block"),
        );
        assert!(
            at_rest(&chain, &left, &right),
            "rest reached at block {rest_at} must hold, broke {held} blocks later"
        );
    }
}

/// Amendment A9's silence counter, carried: a lane exported mid-silence and imported into a fresh
/// bank arms its joint flush on the frame the exported lane would, so the two render the same bits.
///
/// A 10 Hz high-pass (low-pass off) at 48 kHz takes a `1e-11` impulse; its state then decays inside
/// the joint band (both words below `REST_EPS`, above `FLUSH_EPS`) for longer than `N_SILENCE`
/// (4,096 frames at 48 kHz), so the frame the counter arms is the frame the state reaches `+0.0`.
/// After eight blocks (counter 1,023) lane 0 is exported and imported into a fresh bank that has
/// rendered nothing, and both banks render to past the arming frame. Asserted: the state is in the band before the arming
/// frame and `+0.0` from it on, and the carried bank's output and state are the continuing bank's,
/// bit for bit, on every block. A carry that dropped or reset the counter arms 1,024 frames late.
#[test]
fn a_carried_lane_arms_its_joint_flush_on_the_frame_the_exported_lane_would() {
    use builtins::BuiltinInputBank;
    use builtins::test_support::{bank_lane_silence_words, bank_lane_state_words};
    use effect_contract::BankWidth;
    use lane::Backend;

    const LANES: usize = 4;
    let input = || {
        let mut parameters = BuiltinParameters::default();
        parameters.left.hpf_hz = 10.0;
        parameters.right.hpf_hz = 10.0;
        BuiltinChain::new(48_000, parameters)
            .expect("a 10 Hz high-pass prepares")
            .into_input_builtins()
    };
    let bank =
        || BuiltinInputBank::new(Backend::Simd4, BankWidth::Four, vec![input()]).expect("a bank");
    // The impulse on the block's first frame arms the counter on the first frame of a block; on
    // its last frame, on the last frame of a block, where a late armability test would skip it.
    for impulse in [0, FRAMES - 1] {
        let armed_at = impulse + 1 + 4_096;
        let mut continuing = bank();
        let mut carried = bank();
        let blocks = armed_at / FRAMES + 8;
        for block in 0..blocks {
            let mut left = vec![0.0_f32; FRAMES * LANES];
            if block == 0 {
                left[impulse * LANES] = 1.0e-11;
            }
            let mut right = left.clone();
            let (mut carried_left, mut carried_right) = (left.clone(), right.clone());
            if block == 8 {
                let state = continuing.export_lane(0).expect("a member lane");
                carried.import_lane(0, &state).expect("a member lane");
                assert_eq!(
                    bank_lane_silence_words(&carried, 0),
                    [((8 * FRAMES - 1 - impulse) as f32).to_bits(); 2],
                    "the carried counter"
                );
            }
            continuing.process(&mut left, &mut right, FRAMES as u32);
            // The carried bank renders nothing before the import: everything it holds then is
            // what the import wrote.
            if block >= 8 {
                carried.process(&mut carried_left, &mut carried_right, FRAMES as u32);
            }
            if block >= 8 {
                for (plane, (a, b)) in [(&left, &carried_left), (&right, &carried_right)]
                    .into_iter()
                    .enumerate()
                {
                    assert!(
                        a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits()),
                        "block {block}, plane {plane}: the carried lane must render the continuing \
                     lane's bits"
                    );
                }
                assert_eq!(
                    bank_lane_state_words(&carried, 0),
                    bank_lane_state_words(&continuing, 0),
                    "block {block}: the carried lane's integrators"
                );
            }
            let words = bank_lane_state_words(&continuing, 0);
            let end = (block + 1) * FRAMES;
            if end + FRAMES > armed_at && end < armed_at {
                let magnitudes = [words[0], words[1]].map(|bits| f32::from_bits(bits).abs());
                assert!(
                    magnitudes.iter().all(|m| *m < lane::REST_EPS)
                        && magnitudes.iter().any(|m| *m >= lane::FLUSH_EPS),
                    "the state must sit in the joint band before the counter arms: {magnitudes:?}"
                );
            }
            if end >= armed_at {
                assert_eq!(
                    [words[0], words[1]],
                    [0, 0],
                    "impulse at frame {impulse}, block {block}: the joint flush must fire once the \
                 counter arms"
                );
            }
        }
    }
}

/// The silence counter is per-channel state the mono collapse freezes on channel `1`: the
/// disengage copy carries it, and the agreement proof (M3) compares it.
///
/// A bank with no live filter renders one dual block of left zeros and right ones, so the
/// counters differ while every integrator stays `+0.0` and every other word agrees: the channels
/// must not be called equal. Then collapsed blocks advance channel `0`'s counter alone; after
/// `desymmetrize` the channels agree again, counters included.
#[test]
fn the_disengage_copy_and_the_agreement_proof_carry_the_silence_counter() {
    use builtins::BuiltinInputBank;
    use builtins::test_support::bank_lane_silence_words;
    use effect_contract::BankWidth;
    use lane::Backend;

    const LANES: usize = 4;
    let disabled = || {
        BuiltinChain::new(48_000, BuiltinParameters::default())
            .expect("prepared")
            .into_input_builtins()
    };
    let mut bank = BuiltinInputBank::new(
        Backend::Simd4,
        BankWidth::Four,
        (0..LANES).map(|_| disabled()).collect(),
    )
    .expect("a bank");
    let mut left = vec![0.0_f32; FRAMES * LANES];
    let mut right = vec![1.0_f32; FRAMES * LANES];
    bank.process(&mut left, &mut right, FRAMES as u32);
    assert_eq!(
        bank_lane_silence_words(&bank, 0),
        [(FRAMES as f32).to_bits(), 0],
        "the two channels' counters"
    );
    assert!(
        !bank.channels_agree(),
        "channels whose silence counters differ do not agree"
    );
    let mut both = vec![0.0_f32; FRAMES * LANES];
    let mut both_right = both.clone();
    bank.process(&mut both, &mut both_right, FRAMES as u32);
    for _ in 0..3 {
        let mut mono = vec![0.0_f32; FRAMES * LANES];
        bank.process_mono(&mut mono, FRAMES as u32);
    }
    bank.desymmetrize();
    assert_eq!(
        bank_lane_silence_words(&bank, 0),
        [(5.0 * FRAMES as f32).to_bits(); 2],
        "the disengage copy carries channel 0's counter"
    );
    assert!(bank.channels_agree(), "after the copy the channels agree");
}
