//! Issue #210 phase 3 against mono-collapse: what a live trim command does to a collapsed track.
//!
//! Live trim and polarity are **de-symmetrizing live commands upstream of the seam** -- the first
//! ones the input chain has ever had -- so three things had to be true and all three are gated
//! here.
//!
//! 1. **The gate compares the live words, not the prepared ones.** `coef.trim` is republished from
//!    the ramp after every retarget and every ramping block, and `lane_channel_symmetry` reads it
//!    along with the rest of the ramp record. A witness that read the prepared word would call an
//!    asymmetrically-ridden track symmetric for the whole ride.
//! 2. **A symmetric command preserves the collapse bit-identically.** A `Both` retarget gives both
//!    channels the same target over the same window at the same boundary, so a collapsed run
//!    through it must render exactly what a never-collapsed run renders.
//! 3. **The disengage boundary restores the whole per-channel state**, which since phase 3 is the
//!    integrators *and* the trim-ramp record.
//!
//! The chain-level dispatch -- the `LIVE` term, the drain ordering, the re-engage rule -- is gated
//! one level up, where the record type and the bank processor live
//! (`miso-engine-builtins-compiler/tests/input_drain.rs`). This file is the bank's own half.

use miso_engine_builtins::test_support::{bank_lane_state_words, bank_trim_ramp_words};
use miso_engine_builtins::*;
use miso_engine_effect_contract::{BankWidth, ChannelSymmetryWitness};
use miso_engine_lane::Backend;

const BANKS: [(Backend, BankWidth); 2] = [
    (Backend::Simd4, BankWidth::Four),
    (Backend::Simd8, BankWidth::Eight),
];

/// A symmetric track: both channels designed from one set of values, which is what makes the bank
/// collapse-eligible at all.
fn symmetric_parameters(index: usize) -> BuiltinParameters {
    let channel = ChannelParameters {
        polarity_invert: index % 2 == 1,
        trim_db: index as f32 - 2.0,
        hpf_hz: 80.0 + index as f32 * 11.0,
        lpf_hz: 2_000.0 + index as f32 * 101.0,
        fader_db: 0.0,
        muted: false,
    };
    BuiltinParameters {
        left: channel,
        right: channel,
        matrix: Matrix2x2::IDENTITY,
        smoothing_samples: 0,
    }
}

fn bank(backend: Backend, width: BankWidth) -> BuiltinInputBank {
    let inputs: Vec<InputBuiltins> = (0..width.lanes() as usize)
        .map(|index| {
            BuiltinChain::new(48_000, symmetric_parameters(index))
                .expect("accepted input builtins")
                .into_input_builtins()
        })
        .collect();
    BuiltinInputBank::new(backend, width, inputs).expect("bank")
}

/// A block whose content is asymmetric under the identity: signed zeros, a sign flip per frame.
fn source_block(frames: usize, lanes: usize, block: usize) -> Vec<f32> {
    (0..frames * lanes)
        .map(|index| {
            let frame = index / lanes + block * frames;
            match frame % 5 {
                0 => 0.75,
                1 => -0.75,
                2 => -0.0,
                3 => 0.125,
                _ => -0.5,
            }
        })
        .collect()
}

fn bits(values: &[f32]) -> Vec<u32> {
    values.iter().map(|value| value.to_bits()).collect()
}

// ---------------------------------------------------------------------------------------------
// 1. The gate reads the live words.
// ---------------------------------------------------------------------------------------------

/// An asymmetric retarget declines the lane's witness **on the block it is admitted**, before the
/// coefficient has moved a single sample.
///
/// This is the hazard the whole design turns on. At the retarget block `current` is untouched --
/// the ramp has not run -- so a witness that compared only the applied coefficient would still
/// call the two channels equal, let that block collapse, and publish the left channel's new ramp
/// on the right one. The comparison therefore covers the whole ramp record, and `target` moves at
/// the retarget.
///
/// Red mutation: drop `ramp.target`, `ramp.step` and the countdown from
/// `InputStage::lane_channel_symmetry`'s word list, leaving only `coef.trim` -> the lane is still
/// reported symmetric at the retarget block and this fails.
#[test]
fn an_asymmetric_retarget_declines_the_lane_on_the_admitting_block() {
    for (backend, width) in BANKS {
        for command in ["trim", "polarity"] {
            let mut bank = bank(backend, width);
            assert!(
                bank.lane_symmetry(0).eligible(),
                "a symmetric bank's lane 0 starts eligible, or this test proves nothing"
            );
            let before = bank.trim_signed(0, 0).to_bits();
            match command {
                "trim" => bank
                    .set_trim_db(0, BuiltinLaneSelector::Left, -24.0, 64)
                    .expect("trim domain"),
                _ => bank
                    .set_polarity_invert(0, BuiltinLaneSelector::Left, true, 64)
                    .expect("lane"),
            }
            assert_eq!(
                bank.trim_signed(0, 0).to_bits(),
                before,
                "{command}: the applied coefficient has not moved yet -- which is exactly why the \
                 witness cannot be a comparison of applied coefficients alone"
            );
            assert!(
                !bank
                    .lane_symmetry(0)
                    .holds(ChannelSymmetryWitness::DESIGNED),
                "{command} at {width:?}: an asymmetric retarget declines the lane immediately"
            );
            // And only that lane.
            for lane in 1..width.lanes() as usize {
                assert!(
                    bank.lane_symmetry(lane).eligible(),
                    "lane {lane} at {width:?} was not addressed and must be untouched"
                );
            }
        }
    }
}

/// A **symmetric** retarget keeps the lane eligible, at the admitting block and throughout the
/// ramp. Without this the test above would be satisfied by a witness that declined everything.
#[test]
fn a_symmetric_retarget_keeps_the_lane_eligible() {
    const FRAMES: usize = 32;
    for (backend, width) in BANKS {
        let lanes = width.lanes() as usize;
        let mut bank = bank(backend, width);
        bank.set_trim_db(0, BuiltinLaneSelector::Both, -18.0, 96)
            .expect("trim domain");
        bank.set_polarity_invert(1, BuiltinLaneSelector::Both, true, 96)
            .expect("lane");
        for block in 0..4 {
            assert!(
                (0..lanes).all(|lane| bank.lane_symmetry(lane).eligible()),
                "block {block} at {width:?}: a `Both` command is symmetry-preserving throughout \
                 its ramp"
            );
            let mut left = source_block(FRAMES, lanes, block);
            let mut right = left.clone();
            bank.process(&mut left, &mut right, FRAMES as u32);
        }
    }
}

// ---------------------------------------------------------------------------------------------
// 2. A symmetric command preserves the collapse, bit-identically.
// ---------------------------------------------------------------------------------------------

/// The oracle: a run that collapses through a symmetric trim ride renders the bits of a run that
/// never collapsed.
///
/// Red mutation: drop `InputStage::mirror_trim_ramp` from `process_mono` -> the right channel's
/// ramp freezes while the left one advances, the disengage copy restores a stale record, and the
/// first dual block after the switch renders the wrong right plane.
///
/// Red mutation: advance only `remaining[0]` in `process_mono`'s `settle` without mirroring ->
/// same failure, one block later.
#[test]
fn a_symmetric_ride_through_a_collapse_renders_never_collapsed_bits() {
    const FRAMES: usize = 32;
    const BLOCKS: usize = 10;
    const DISENGAGE: usize = 6;
    for (backend, width) in BANKS {
        let lanes = width.lanes() as usize;
        let mut collapsing = bank(backend, width);
        let mut never = bank(backend, width);
        assert!(collapsing.supports_mono_collapse());

        // The same command to both arms, at the same block boundary, over a window that outlives
        // the collapse so the ride is genuinely in flight across the disengage.
        for arm in [&mut collapsing, &mut never] {
            for lane in 0..lanes {
                arm.set_trim_db(lane, BuiltinLaneSelector::Both, -15.0, (FRAMES * 8) as u32)
                    .expect("trim domain");
            }
        }

        for block in 0..BLOCKS {
            let source = source_block(FRAMES, lanes, block);
            let mut never_left = source.clone();
            let mut never_right = source.clone();
            never.process(&mut never_left, &mut never_right, FRAMES as u32);

            if block < DISENGAGE {
                let mut plane = source.clone();
                collapsing.process_mono(&mut plane, FRAMES as u32);
                assert_eq!(
                    bits(&plane),
                    bits(&never_left),
                    "block {block} at {width:?}: the collapsed plane is the never-collapsed left \
                     plane"
                );
                if block + 1 == DISENGAGE {
                    collapsing.desymmetrize();
                }
            } else {
                let mut left = source.clone();
                let mut right = source.clone();
                collapsing.process(&mut left, &mut right, FRAMES as u32);
                assert_eq!(
                    bits(&left),
                    bits(&never_left),
                    "block {block} at {width:?}: left plane after the disengage"
                );
                assert_eq!(
                    bits(&right),
                    bits(&never_right),
                    "block {block} at {width:?}: right plane after the disengage -- the whole \
                     per-channel state, integrators and trim ramp, was restored"
                );
            }
        }

        // The two runs also agree on the state they retained, which is the half no plane shows.
        for lane in 0..lanes {
            assert_eq!(
                bank_lane_state_words(&collapsing, lane),
                bank_lane_state_words(&never, lane),
                "lane {lane} at {width:?}: retained integrators"
            );
            assert_eq!(
                bank_trim_ramp_words(&collapsing, lane),
                bank_trim_ramp_words(&never, lane),
                "lane {lane} at {width:?}: the whole trim-ramp record"
            );
        }
    }
}

// ---------------------------------------------------------------------------------------------
// 3. The disengage boundary and the M3 proof.
// ---------------------------------------------------------------------------------------------

/// `channels_agree` covers the trim-ramp record, not only the integrators.
///
/// M3's way back consults this, and a `true` that covered less would re-engage a collapse onto a
/// right channel whose coefficient is not the left one's.
///
/// Red mutation: make `InputStage::channels_agree` return only the integrator comparison (drop the
/// `trim_ramp_channels_agree()` conjunct) -> an asymmetrically-ridden bank claims agreement.
#[test]
fn channels_agree_covers_the_trim_ramp_record() {
    for (backend, width) in BANKS {
        let mut bank = bank(backend, width);
        assert!(
            bank.channels_agree(),
            "a freshly prepared symmetric bank's channels agree"
        );

        // A symmetric ride keeps them agreeing, mid-ramp and after it.
        bank.set_trim_db(0, BuiltinLaneSelector::Both, -9.0, 64)
            .expect("trim domain");
        assert!(
            bank.channels_agree(),
            "a `Both` retarget preserves agreement"
        );
        let lanes = width.lanes() as usize;
        let mut left = source_block(16, lanes, 0);
        let mut right = left.clone();
        bank.process(&mut left, &mut right, 16);
        assert!(bank.channels_agree(), "mid-ramp, under a `Both` retarget");

        // An asymmetric one breaks it at the retarget, before a sample moves.
        bank.set_trim_db(0, BuiltinLaneSelector::Right, 6.0, 64)
            .expect("trim domain");
        assert!(
            !bank.channels_agree(),
            "a one-channel retarget breaks agreement at the retarget, not at the first sample"
        );
    }
}

/// The disengage copy restores the trim ramp as well as the integrators.
///
/// `process_mono` mirrors the record per block, so by the time `desymmetrize` runs the two are
/// already equal -- this asserts that invariant rather than the copy, which is what makes the copy
/// a restatement of the law rather than a repair.
#[test]
fn a_collapsed_block_keeps_the_two_channels_ramp_records_equal() {
    const FRAMES: usize = 16;
    for (backend, width) in BANKS {
        let lanes = width.lanes() as usize;
        let mut bank = bank(backend, width);
        for lane in 0..lanes {
            bank.set_trim_db(lane, BuiltinLaneSelector::Both, -21.0, 128)
                .expect("trim domain");
        }
        for block in 0..6 {
            let mut plane = source_block(FRAMES, lanes, block);
            bank.process_mono(&mut plane, FRAMES as u32);
            for lane in 0..lanes {
                let words = bank_trim_ramp_words(&bank, lane);
                assert_eq!(
                    [words[0], words[2], words[4], words[6]],
                    [words[1], words[3], words[5], words[7]],
                    "lane {lane} at {width:?}, block {block}: a collapsed block advances the \
                     right channel's ramp record with the left one's"
                );
            }
            // `channels_agree` is deliberately **not** asserted here, and the reason is worth
            // stating rather than working around: a collapsed block advances one channel's
            // integrators and freezes the other's, so the state proof is legitimately false
            // mid-collapse. `BankChain::run` never asks it there -- it asks only inside a recovery
            // window, on a chain that is rendering *dual* with the agreement flag already lost --
            // and by then the disengage copy has run. The assertion belongs after that copy, and
            // it is one line below.
        }
        bank.desymmetrize();
        assert!(
            bank.channels_agree(),
            "the disengage boundary restores the whole per-channel state, so the M3 proof holds              at exactly the point M3 asks it"
        );
    }
}

/// An asymmetric ride makes the bank's own witness decline, and re-equalising the words restores
/// **the words** -- which is the fact the chain-level `LIVE` latch is layered on top of.
///
/// The rule this half states: the designed-word comparison is a statement about the words and
/// nothing else. It comes back when the words come back. What must *not* come back on that basis
/// alone is the collapse, and that is the `LIVE` term's job one level up
/// (`miso-engine-builtins-compiler/tests/input_drain.rs`).
#[test]
fn re_equalising_the_words_restores_the_designed_term_and_nothing_more() {
    for (backend, width) in BANKS {
        let mut bank = bank(backend, width);
        let settled = bank_trim_ramp_words(&bank, 0);
        bank.set_trim_db(0, BuiltinLaneSelector::Left, -30.0, 0)
            .expect("trim domain");
        assert!(
            !bank.lane_symmetry(0).eligible(),
            "the ride declines the lane"
        );
        assert!(
            !bank.channels_agree(),
            "and the state proof declines it too"
        );

        // Put the right channel where the left one is. The words are equal again.
        bank.set_trim_db(0, BuiltinLaneSelector::Right, -30.0, 0)
            .expect("trim domain");
        assert!(
            bank.lane_symmetry(0).eligible(),
            "the designed term is a statement about words, and the words agree again"
        );
        assert!(bank.channels_agree(), "so does the state proof");
        assert_ne!(
            bank_trim_ramp_words(&bank, 0),
            settled,
            "the lane is at a different coefficient from the one it was prepared with, which is \
             what makes this a re-equalisation rather than a no-op"
        );
    }
}

// ---------------------------------------------------------------------------------------------
// 4. The disengage-under-drain window, crate-local.
// ---------------------------------------------------------------------------------------------

/// A retarget applied between a collapsed block and the disengage copy survives the copy.
///
/// The crate-local form of `miso-engine-host-core`'s
/// `a_per_lane_record_drained_on_the_disengaging_block_reaches_one_channel`, and it is here as well
/// as there for the reason `a_desymmetrized_bank_is_a_never_collapsed_bank` gives: the end-to-end
/// oracle proves the *strip* agrees, and this proves the input chain's own contribution, so a
/// regression names the crate rather than the session.
///
/// The ordering below is `BankChain::run`'s, in miniature and in its exact order: every slot's
/// `begin_block` drains **first**, the collapse dispatch reads the witness **second**, and
/// `disengage_collapse` runs **third**. A retarget applied at step 1 is therefore already in the
/// stage when `desymmetrize` is reached, and it is the one thing the boundary must not clobber.
///
/// Red mutation: restore `self.mirror_trim_ramp()` in `InputStage::desymmetrize` -> the ridden
/// channel's record is cloned onto the other one and every dual block after the boundary renders
/// the wrong plane.
#[test]
fn a_retarget_between_a_collapsed_block_and_the_disengage_survives_the_copy() {
    const FRAMES: usize = 32;
    const BLOCKS: usize = 8;
    const COLLAPSED: usize = 3;
    for (backend, width) in BANKS {
        for lanes_addressed in [BuiltinLaneSelector::Left, BuiltinLaneSelector::Right] {
            for snap in [0_u32, 96] {
                let lane_count = width.lanes() as usize;
                let mut collapsing = bank(backend, width);
                let mut never = bank(backend, width);

                for block in 0..BLOCKS {
                    let source = source_block(FRAMES, lane_count, block);

                    // Step 1, on both arms: the drain. It happens at the top of every block, and
                    // on block `COLLAPSED` it is the drain that makes the two channels differ.
                    if block == COLLAPSED {
                        for arm in [&mut collapsing, &mut never] {
                            arm.set_trim_db(0, lanes_addressed, -21.0, snap)
                                .expect("trim domain");
                            arm.set_polarity_invert(1, lanes_addressed, true, snap)
                                .expect("lane");
                        }
                        // Step 2: the witness now declines, which is what ends the collapse.
                        assert!(
                            !collapsing.lane_symmetry(0).eligible(),
                            "{lanes_addressed:?} snap={snap} at {width:?}: the drain must decline \
                             the lane, or this test is not in the window it claims"
                        );
                    }

                    let mut never_left = source.clone();
                    let mut never_right = source.clone();
                    never.process(&mut never_left, &mut never_right, FRAMES as u32);

                    if block < COLLAPSED {
                        let mut plane = source.clone();
                        collapsing.process_mono(&mut plane, FRAMES as u32);
                        assert_eq!(bits(&plane), bits(&never_left), "block {block}");
                        if block + 1 == COLLAPSED {
                            // Step 3, but *before* the drain: this is the ordinary disengage, and
                            // it is not the one the regression was about.
                        }
                    } else {
                        if block == COLLAPSED {
                            // Step 3, after the drain: the boundary the regression was about.
                            collapsing.desymmetrize();
                        }
                        let mut left = source.clone();
                        let mut right = source.clone();
                        collapsing.process(&mut left, &mut right, FRAMES as u32);
                        assert_eq!(
                            bits(&left),
                            bits(&never_left),
                            "{lanes_addressed:?} snap={snap} at {width:?}, block {block}: left"
                        );
                        assert_eq!(
                            bits(&right),
                            bits(&never_right),
                            "{lanes_addressed:?} snap={snap} at {width:?}, block {block}: right -- \
                             a retarget addressed to one lane reached the other at the disengage \
                             boundary"
                        );
                    }
                }

                for lane in 0..lane_count {
                    assert_eq!(
                        bank_trim_ramp_words(&collapsing, lane),
                        bank_trim_ramp_words(&never, lane),
                        "lane {lane} at {width:?}: the retained trim-ramp record"
                    );
                    assert_eq!(
                        bank_lane_state_words(&collapsing, lane),
                        bank_lane_state_words(&never, lane),
                        "lane {lane} at {width:?}: the retained integrators"
                    );
                }
            }
        }
    }
}

/// The disengage copy still restores the **integrators**, which is the half a collapsed block does
/// freeze.
///
/// The complement of the test above, and the reason the fix narrowed the copy rather than deleting
/// it: `process_mono` advances channel `0`'s integrators and leaves channel `1`'s alone, so those
/// words genuinely need the boundary. Dropping that half is `a_desymmetrized_bank_is_a_never_-
/// collapsed_bank`'s standing red mutation (M2-B1); this states the same requirement in the
/// vocabulary of the narrowed rule.
///
/// Red mutation: make `InputStage::desymmetrize` a no-op -> the first dual block's right plane is
/// wrong for every lane whose high-pass is retaining anything.
#[test]
fn the_disengage_copy_still_restores_the_integrators() {
    const FRAMES: usize = 32;
    for (backend, width) in BANKS {
        let lane_count = width.lanes() as usize;
        let mut collapsing = bank(backend, width);
        let mut never = bank(backend, width);
        for block in 0..4 {
            let source = source_block(FRAMES, lane_count, block);
            let mut never_left = source.clone();
            let mut never_right = source.clone();
            never.process(&mut never_left, &mut never_right, FRAMES as u32);
            let mut plane = source.clone();
            collapsing.process_mono(&mut plane, FRAMES as u32);
        }
        // Frozen, and therefore different, before the boundary runs.
        let frozen: Vec<[u32; 8]> = (0..lane_count)
            .map(|lane| bank_lane_state_words(&collapsing, lane))
            .collect();
        let expected: Vec<[u32; 8]> = (0..lane_count)
            .map(|lane| bank_lane_state_words(&never, lane))
            .collect();
        assert_ne!(
            frozen, expected,
            "at {width:?} the collapsed arm's right integrators must actually be frozen, or the \
             assertion below is vacuous"
        );
        collapsing.desymmetrize();
        for (lane, expected) in expected.iter().enumerate() {
            assert_eq!(
                &bank_lane_state_words(&collapsing, lane),
                expected,
                "lane {lane} at {width:?}: the boundary restores the integrators"
            );
        }
    }
}
