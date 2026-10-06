//! Which blocks run the builtin input chain's **armed** form (issue #1328).
//!
//! Every input-chain kernel runs its body in one of two forms per block: the armed form (the
//! silence counter per frame and the joint flush) or the unarmed form (the per-word flush alone,
//! the counter advanced once per block). The two give the same bits on a block where no lane both
//! can arm and holds state (`lane::silence_armable_holding`), and that is the only block the
//! kernels may run unarmed. The decision is per channel, so a bank with any silent or padding lane
//! at rest stays on the unarmed form.
//!
//! These gates hold the decision's three inputs, each on every dual and mono entry point:
//!
//! * **Per channel.** Only the right channel's input is silent and only its state sits in the
//!   joint band: the block must arm, and the right pair must reach `+0.0`. A body that decides
//!   from channel 0's counter alone (#1328 attempt 5's verifier, mutant MI) leaves it ringing.
//! * **Every state word.** One state word of one section in the band, the rest `+0.0`: the block
//!   must arm. A "holding" test that skips a section or a word turns a case red.
//! * **A block longer than the window.** A lane at rest at block start that takes a tiny sample
//!   and then a full window of zeros inside the block must arm on that frame: the exactness proof
//!   of `silence_armable_holding` needs the block to be no longer than the window, and a decision
//!   that dropped that term, or moved it by one frame, leaves the tail. The block is also exactly
//!   one frame longer than the window, the boundary case.

use lane::Lane;
use lane::kernels::SvfCoef;
use lane::kernels::builtins::{
    InputChainCoef, InputChainPlan, InputChainState, InputTrimRamp, input_chain_block,
    input_chain_block_elided, input_chain_block_mono, input_chain_block_mono_elided,
    input_chain_ramp_block, input_chain_ramp_block_filter, input_chain_ramp_block_filter_mono,
    input_chain_ramp_block_mono,
};

/// Frames per block.
const FRAMES: usize = 32;
/// The chain's silence window (`InputChainCoef::silence`) for the per-channel and state-word
/// gates: longer than the block, so only a lane that holds state may arm it.
const WINDOW: f32 = 40.0;
/// The window for the long-block gate: shorter than the block.
const SHORT_WINDOW: f32 = 8.0;
/// The long-block gate's boundary window: the block is exactly one frame longer, so the lane's
/// tail arms on the block's last frame, the first block length the unarmed form is not exact for.
const BOUNDARY_WINDOW: f32 = (FRAMES - 1) as f32;
/// `tan(pi * 30 / 48000)`: the high-pass section's prewarped gain.
const G_30_HZ: f64 = 0.001_963_497_931_794_767_5;
/// `tan(pi * 1000 / 48000)`: the low-pass section's prewarped gain.
const G_1_KHZ: f64 = 0.065_543_462_815_238_22;
/// A state pair inside the joint band (both magnitudes below `REST_EPS`, above `FLUSH_EPS`).
const BAND: [f32; 2] = [5.0e-15, -4.0e-15];

/// One Butterworth section from its prewarped gain `g = tan(pi * cutoff / rate)`, in
/// `SvfSection::design`'s operation order.
fn section<L: Lane>(g: f64, high_pass: bool) -> SvfCoef<L> {
    let k = core::f64::consts::SQRT_2;
    let t1 = g * (g + k);
    let denominator = 1.0 + t1;
    let (m0, m1, m2) = if high_pass {
        (1.0, -(k as f32), -1.0)
    } else {
        (0.0, 0.0, 1.0)
    };
    SvfCoef {
        c1: L::splat((t1 / denominator) as f32),
        a2: L::splat((g / denominator) as f32),
        a3: L::splat((g * g / denominator) as f32),
        m0: L::splat(m0),
        m1: L::splat(m1),
        m2: L::splat(m2),
    }
}

/// The identity section (`SvfSection::IDENTITY`): what an elided section carries.
fn identity<L: Lane>() -> SvfCoef<L> {
    SvfCoef {
        c1: L::zero(),
        a2: L::zero(),
        a3: L::zero(),
        m0: L::splat(1.0),
        m1: L::zero(),
        m2: L::zero(),
    }
}

/// The entry points. `Mixed` runs the elided dispatch with section 0 elided on both channels (its
/// coefficients are the identity), which takes the per-channel mixed bodies.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Entry {
    Block,
    TrimRamp,
    FilterRamp,
    Mixed,
}

const ENTRIES: [Entry; 4] = [
    Entry::Block,
    Entry::TrimRamp,
    Entry::FilterRamp,
    Entry::Mixed,
];

/// A 30 Hz high-pass into a 1 kHz low-pass at 48 kHz on both channels, or the identity in
/// section 0 for [`Entry::Mixed`].
fn chain<L: Lane>(entry: Entry, window: f32) -> InputChainCoef<L> {
    let first = if entry == Entry::Mixed {
        identity::<L>()
    } else {
        section::<L>(G_30_HZ, true)
    };
    let second = section::<L>(G_1_KHZ, false);
    InputChainCoef {
        trim: [L::splat(1.0); 2],
        section: [[first, second]; 2],
        silence: L::splat(window),
    }
}

/// Runs one block through `entry`, both channels (`mono = false`) or the collapsed one.
fn run<L: Lane>(
    entry: Entry,
    mono: bool,
    left: &mut [f32],
    right: &mut [f32],
    c: &mut InputChainCoef<L>,
    s: &mut InputChainState<L>,
) {
    let mut trim = InputTrimRamp {
        current: c.trim,
        target: c.trim,
        step: [L::zero(); 2],
        remaining: [L::zero(); 2],
    };
    let target = c.section;
    let zero = SvfCoef {
        c1: L::zero(),
        a2: L::zero(),
        a3: L::zero(),
        m0: L::zero(),
        m1: L::zero(),
        m2: L::zero(),
    };
    let step = [[zero; 2]; 2];
    let mut remaining = [[L::zero(); 2]; 2];
    let plan = InputChainPlan {
        elided: [[true, false], [true, false]],
    };
    match (entry, mono) {
        (Entry::Block, false) => {
            input_chain_block(left, right, FRAMES, c, s);
        }
        (Entry::Block, true) => {
            input_chain_block_mono(left, FRAMES, c, s);
        }
        (Entry::TrimRamp, false) => {
            input_chain_ramp_block(left, right, FRAMES, c, s, &mut trim);
        }
        (Entry::TrimRamp, true) => {
            input_chain_ramp_block_mono(left, FRAMES, c, s, &mut trim);
        }
        (Entry::FilterRamp, false) => {
            input_chain_ramp_block_filter(
                left,
                right,
                FRAMES,
                c,
                s,
                &mut trim,
                false,
                &target,
                &step,
                &mut remaining,
            );
        }
        (Entry::FilterRamp, true) => {
            input_chain_ramp_block_filter_mono(
                left,
                FRAMES,
                c,
                s,
                &mut trim,
                false,
                &target,
                &step,
                &mut remaining,
            );
        }
        (Entry::Mixed, false) => {
            input_chain_block_elided(left, right, FRAMES, c, s, &plan);
        }
        (Entry::Mixed, true) => {
            input_chain_block_mono_elided(left, FRAMES, c, s, &plan);
        }
    }
}

/// Every state word of `channel`, as bits, lane by lane.
fn state_bits<L: Lane>(s: &InputChainState<L>, channel: usize) -> Vec<u32> {
    let mut out = Vec::new();
    for section in &s.section[channel] {
        for word in [section.ic1, section.ic2] {
            let mut bits = vec![0_u32; L::WIDTH];
            word.store_bits(&mut bits);
            out.extend(bits);
        }
    }
    out
}

/// A live input: a constant that no counter reads as silence.
fn live() -> Vec<f32> {
    vec![0.25; FRAMES * 8]
}

/// Per channel: only the right channel's input is silent, one frame short of its window, and only
/// its state sits in the joint band. The block must arm and clear the right pair; the left channel
/// keeps running live.
fn right_channel_alone_arms<L: Lane>(width: &str) {
    for entry in ENTRIES {
        let mut c = chain::<L>(entry, WINDOW);
        let mut s = InputChainState::<L>::default();
        let held = if entry == Entry::Mixed { 1..2 } else { 0..2 };
        for section in held {
            s.section[1][section].ic1 = L::splat(BAND[0]);
            s.section[1][section].ic2 = L::splat(BAND[1]);
        }
        s.silence[1] = L::splat(WINDOW - 1.0);
        let mut left = live()[..FRAMES * L::WIDTH].to_vec();
        let mut right = vec![0.0_f32; FRAMES * L::WIDTH];
        run(entry, false, &mut left, &mut right, &mut c, &mut s);
        assert!(
            state_bits(&s, 1).iter().all(|bits| *bits == 0),
            "{width}, {entry:?}: the right channel arms alone and clears its band pair"
        );
        assert!(
            state_bits(&s, 0).iter().any(|bits| *bits != 0),
            "{width}, {entry:?}: the left channel stays live"
        );
    }
}

#[test]
fn the_right_channel_arms_its_joint_flush_alone() {
    lane::each_lane!(|L| right_channel_alone_arms::<L>(core::any::type_name::<L>()));
}

/// Every state word: one word of one section of the silent channel in the band, every other word
/// `+0.0`. The block must arm and clear it, on both channels and in the collapsed bodies.
fn every_state_word_arms<L: Lane>(width: &str) {
    for entry in ENTRIES {
        for mono in [false, true] {
            for channel in 0..if mono { 1 } else { 2 } {
                let sections = if entry == Entry::Mixed { 1..2 } else { 0..2 };
                for section in sections {
                    for word in 0..2 {
                        let mut c = chain::<L>(entry, WINDOW);
                        let mut s = InputChainState::<L>::default();
                        let target = &mut s.section[channel][section];
                        if word == 0 {
                            target.ic1 = L::splat(BAND[0]);
                        } else {
                            target.ic2 = L::splat(BAND[1]);
                        }
                        s.silence[channel] = L::splat(WINDOW - 1.0);
                        let mut planes = [live(), live()];
                        planes[channel] = vec![0.0; FRAMES * 8];
                        let [left, right] = &mut planes;
                        run(
                            entry,
                            mono,
                            &mut left[..FRAMES * L::WIDTH],
                            &mut right[..FRAMES * L::WIDTH],
                            &mut c,
                            &mut s,
                        );
                        assert!(
                            state_bits(&s, channel).iter().all(|bits| *bits == 0),
                            "{width}, {entry:?}, mono {mono}: channel {channel}, section \
                             {section}, word {word} in the band arms the block"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn any_state_word_in_the_band_arms_the_block() {
    lane::each_lane!(|L| every_state_word_arms::<L>(core::any::type_name::<L>()));
}

/// A block longer than the window: the channel starts at rest with a fresh counter, takes one tiny
/// sample on frame 0 and zeros after it. Its counter reaches the window on frame `window`, inside
/// the block, with the tail's state in the band: the block must arm though no lane held state at
/// its start, and the tail must be cleared by the block's end. Two windows: `SHORT_WINDOW`, and
/// `BOUNDARY_WINDOW`, where the block is exactly `window + 1` frames and the tail arms on its last
/// frame (red if the block-length term is off by one, `frames > armed_after + 1`).
fn a_long_block_arms_from_rest<L: Lane>(width: &str) {
    for window in [SHORT_WINDOW, BOUNDARY_WINDOW] {
        for entry in ENTRIES {
            for mono in [false, true] {
                for channel in 0..if mono { 1 } else { 2 } {
                    let mut c = chain::<L>(entry, window);
                    let mut s = InputChainState::<L>::default();
                    let mut planes = [live(), live()];
                    planes[channel] = vec![0.0; FRAMES * 8];
                    planes[channel][..L::WIDTH].fill(1.0e-15);
                    let [left, right] = &mut planes;
                    run(
                        entry,
                        mono,
                        &mut left[..FRAMES * L::WIDTH],
                        &mut right[..FRAMES * L::WIDTH],
                        &mut c,
                        &mut s,
                    );
                    assert!(
                        state_bits(&s, channel).iter().all(|bits| *bits == 0),
                        "{width}, {entry:?}, mono {mono}, window {window}: channel {channel}'s \
                     tail arms inside a block longer than the window"
                    );
                }
            }
        }
    }
}

#[test]
fn a_block_longer_than_the_window_arms_from_rest() {
    lane::each_lane!(|L| a_long_block_arms_from_rest::<L>(core::any::type_name::<L>()));
}
