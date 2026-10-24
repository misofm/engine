//! The input chain's collapsed body: its plane, its state and its **accounting**.
//!
//! The plane and the state are already gated end to end -- `chain_shape`'s transition oracles
//! render the whole strip and compare digests, and `MUTATIONS.md` row M2-B1 is the disengage copy.
//! What no digest can see is the third thing a block produces: [`BuiltinProcessReport`], the
//! per-channel sanitised-sample and recovered-lane accounting that the boundary policy publishes.
//! A collapsed block runs one plane, so every counter it produces is a left-channel count, and the
//! right channel of a collapsed track *is* its left channel -- at the counter exactly as at the
//! fader. A body that left the right half at zero would halve the session's sanitised total while
//! rendering perfect audio, and every other gate in the tree would stay green.
//!
//! This is that gate, and it is crate-local because the report is dropped by the graph adapter --
//! the counters live in `lifetime_recovered` and in the audit's fixture check, neither of which the
//! console workload reads.
//!
//! Everything here goes through the shipped surface: `BuiltinInputBank::process`,
//! `process_mono`, `desymmetrize`, `supports_mono_collapse`.

use builtins::*;
use effect_contract::BankWidth;
use lane::Backend;

/// Every bank width this build has, with the backend that executes it: both in the 8-lane (AVX2)
/// build, four alone in a 4-lane (NEON/simd128) one (`BankWidth::ALL`, issue #1112).
fn banks() -> impl Iterator<Item = (Backend, BankWidth)> {
    BankWidth::ALL.iter().map(|&width| (width.backend(), width))
}

/// A symmetric track: the two channels are designed from one set of values, which is what makes
/// the bank collapse-eligible at all (`lane_channel_symmetry` is a bitwise word comparison).
fn symmetric_parameters(index: usize) -> BuiltinParameters {
    let channel = ChannelParameters {
        polarity_invert: index % 2 == 1,
        trim_db: index as f32 - 2.0,
        hpf_hz: 80.0 + index as f32 * 11.0,
        lpf_hz: 2_000.0 + index as f32 * 101.0,
        fader_db: 1.0 - index as f32,
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

/// A block whose every fourth sample is one the boundary policy has to sanitise.
///
/// This reaches the input scan, which is one of the boundary policy's two arms. The other -- the
/// recovery arm, which fires on a non-finite *output* -- is reached by poisoning a lane's retained
/// integrator at the call site, because no input can drive a stable section non-finite reliably
/// enough to be a gate.
fn poisoned_block(frames: usize, lanes: usize) -> Vec<f32> {
    const POISON: [u32; 4] = [
        0x7FC0_0000, // quiet NaN
        0x7F80_0000, // +inf
        0xFF80_0000, // -inf
        0x7F7F_FFFF, // f32::MAX, finite but past the boundary limit once trimmed
    ];
    (0..frames * lanes)
        .map(|index| {
            if index % 4 == 0 {
                f32::from_bits(POISON[(index / 4) % POISON.len()])
            } else {
                let step = (index % 97) as f32;
                0.125 + step / 512.0
            }
        })
        .collect()
}

/// The collapsed body's report is the dual body's report, for a block whose two planes agree.
///
/// # Why the comparison is against a dual run and not against a constant
///
/// "Duplicate the left half onto the right" is not the claim. The claim is the same one the fader
/// duplication and the disengage copy make: the counterfactual dual run's right channel *is* its
/// left channel, so the report a collapsed block publishes must be the report that dual run would
/// have published -- including `sanitized_input`, which is a **sum over both channels** and is
/// therefore twice the left count rather than equal to it. A body that duplicated the mask but
/// summed one channel would pass a duplication check and fail this one.
///
/// # Red mutation
///
/// In `InputStage::process_mono`, drop the `.saturating_add(self.members_sum(report.sanitized[1]))`
/// from `sanitized_input`, or set `recovered_right_state` to `0`, or add `recovered` to
/// `lifetime_recovered[0]` alone. Each fails here and nothing else in the tree moves: the audio is
/// untouched by all three.
#[test]
fn the_collapsed_body_publishes_the_dual_bodys_report() {
    const FRAMES: usize = 64;
    for (backend, width) in banks() {
        let lanes = width.lanes() as usize;
        let mut collapsed = bank(backend, width);
        let mut dual = bank(backend, width);
        assert!(
            collapsed.supports_mono_collapse(),
            "a symmetric bank must be collapsible, or this file tests nothing"
        );

        // The two arms of the boundary policy fire on different things: the sanitised counter is
        // raised by the input scan, and the recovered counter by an output that came out
        // non-finite. Poisoning one lane's retained state is what reaches the second -- a
        // non-finite integrator drives the recurrence non-finite however clean the input is -- so
        // both counters are non-zero below and neither assertion is vacuous.
        // Both channels' high-pass first integrators, because the counters are per channel and a
        // one-sided poison would leave the right one at zero -- which is the value the mutation
        // this asserts against also produces.
        for arm in [&mut collapsed, &mut dual] {
            let mut poisoned = test_support::bank_lane_state_words(arm, 1);
            poisoned[0] = f32::NAN.to_bits();
            poisoned[4] = f32::NAN.to_bits();
            test_support::set_bank_lane_state_words(arm, 1, poisoned);
        }

        let block = poisoned_block(FRAMES, lanes);
        let mut mono_plane = block.clone();
        let (mut left, mut right) = (block.clone(), block);

        let mono_report = collapsed.process_mono(&mut mono_plane, FRAMES as u32);
        let dual_report = dual.process(&mut left, &mut right, FRAMES as u32);

        assert!(
            dual_report.sanitized_input > 0,
            "width={lanes}: the corpus must actually reach the input scan"
        );
        assert!(
            dual_report.recovered_left_state > 0 && dual_report.recovered_right_state > 0,
            "width={lanes}: the corpus must actually reach the recovery arm, on both channels"
        );
        assert_eq!(
            mono_report.sanitized_input, dual_report.sanitized_input,
            "width={lanes}: the collapsed block's sanitised total is the dual block's, which \
             counts both channels"
        );
        assert_eq!(
            mono_report.sanitized_output, dual_report.sanitized_output,
            "width={lanes}"
        );
        assert_eq!(
            [
                mono_report.recovered_left_state,
                mono_report.recovered_right_state
            ],
            [
                dual_report.recovered_left_state,
                dual_report.recovered_right_state
            ],
            "width={lanes}: the collapsed block's per-channel recovery counts are the dual \
             block's, on both channels"
        );
        assert_eq!(
            test_support::bank_lifetime_recovered(&collapsed),
            test_support::bank_lifetime_recovered(&dual),
            "width={lanes}: the collapsed body must feed both channels' lifetime counters, which \
             outlive the report the graph adapter drops"
        );
        for (frame, (mono, dual)) in mono_plane.iter().zip(left.iter()).enumerate() {
            assert_eq!(
                mono.to_bits(),
                dual.to_bits(),
                "width={lanes} word={frame}: the collapsed plane is the dual body's left plane"
            );
        }
    }
}

/// A bank that ran collapsed and then de-symmetrised is a bank that never collapsed.
///
/// The crate-local form of `chain_shape::a_run_that_stops_collapsing_...`, and the reason it is
/// here as well as there: the end-to-end oracle proves the *strip* agrees, and this proves the
/// input chain's own contribution to that agreement -- so a regression names the crate rather than
/// the session. It is also the gate on the lifetime counters across the boundary, which the strip
/// does not read.
///
/// Red mutation: `InputStage::desymmetrize` drops the integrator copy. This fails on the first
/// dual block, on the right plane, for every lane whose high-pass is retaining anything.
#[test]
fn a_desymmetrized_bank_is_a_never_collapsed_bank() {
    const FRAMES: usize = 32;
    const BLOCKS: usize = 8;
    const SWITCH: usize = 4;
    for (backend, width) in banks() {
        let lanes = width.lanes() as usize;
        let mut mixed = bank(backend, width);
        let mut never = bank(backend, width);

        for block in 0..BLOCKS {
            let source = poisoned_block(FRAMES, lanes);
            let (mut mixed_left, mut mixed_right) = (source.clone(), source.clone());
            let (mut never_left, mut never_right) = (source.clone(), source);

            if block < SWITCH {
                mixed.process_mono(&mut mixed_left, FRAMES as u32);
                // The seam: everything downstream of the input chain reads the duplicated plane.
                mixed_right.copy_from_slice(&mixed_left);
            } else {
                if block == SWITCH {
                    mixed.desymmetrize();
                }
                mixed.process(&mut mixed_left, &mut mixed_right, FRAMES as u32);
            }
            never.process(&mut never_left, &mut never_right, FRAMES as u32);

            for word in 0..FRAMES * lanes {
                assert_eq!(
                    mixed_left[word].to_bits(),
                    never_left[word].to_bits(),
                    "width={lanes} block={block} word={word}: left plane"
                );
                assert_eq!(
                    mixed_right[word].to_bits(),
                    never_right[word].to_bits(),
                    "width={lanes} block={block} word={word}: right plane"
                );
            }
        }
    }
}

/// Issue #1894: a collapsed input stage whose left channel falls silent is at rest, and a skipped
/// block is exact.
///
/// Dual signal first, so the collapse freezes a right channel whose integrators are **not** at
/// rest; then collapsed signal, then collapsed `+0.0` until the left channel rests. The bank must
/// then answer every lane at rest (a predicate that still reads the frozen right integrators
/// answers `0` and never gets there), `advance_clocks` stands in for three zero blocks the
/// reference renders, and after the disengage the state, the silence counters and the next dual
/// blocks are the never-collapsed bank's, bit for bit (red if a skipped collapsed block advances
/// the wrong counter).
#[test]
fn a_collapsed_stage_that_falls_silent_is_at_rest_and_skips_exactly() {
    const FRAMES: usize = 128;
    const SIGNAL: usize = 4;
    const SKIPPED: usize = 3;
    const REST_BOUND: usize = 5_000;
    for (backend, width) in banks() {
        let lanes = width.lanes() as usize;
        let all = (1_u32 << lanes) - 1;
        let mut mixed = bank(backend, width);
        let mut never = bank(backend, width);
        let mut step = 0_usize;
        // `skip`: the reference renders the `+0.0` block, the collapsed bank advances its clocks.
        let mut run = |mixed: &mut BuiltinInputBank,
                       never: &mut BuiltinInputBank,
                       signal: bool,
                       mono: bool,
                       skip: bool| {
            let mut never_left: Vec<f32> = (0..FRAMES * lanes)
                .map(|word| {
                    if signal {
                        0.5 - ((step * FRAMES * lanes + word) % 37) as f32 / 37.0
                    } else {
                        0.0
                    }
                })
                .collect();
            let mut never_right = never_left.clone();
            let mut mixed_left = never_left.clone();
            let mut mixed_right = never_left.clone();
            never.process(&mut never_left, &mut never_right, FRAMES as u32);
            let context = format!("width={lanes} block={step}");
            step += 1;
            if skip {
                mixed.advance_clocks(all, FRAMES as u32);
                return;
            }
            if mono {
                mixed.process_mono(&mut mixed_left, FRAMES as u32);
            } else {
                mixed.process(&mut mixed_left, &mut mixed_right, FRAMES as u32);
            }
            let bits = |plane: &[f32]| plane.iter().map(|word| word.to_bits()).collect::<Vec<_>>();
            assert_eq!(
                bits(&mixed_left),
                bits(&never_left),
                "{context}: left plane"
            );
            if !mono {
                assert_eq!(bits(&mixed_right), bits(&never_right), "{context}: right");
            }
        };
        for mono in [false, true] {
            for _ in 0..SIGNAL {
                run(&mut mixed, &mut never, true, mono, false);
            }
        }
        let rested = (0..REST_BOUND).any(|_| {
            run(&mut mixed, &mut never, false, true, false);
            mixed.lanes_at_rest() == all
        });
        assert!(
            rested,
            "width={lanes}: the collapsed left channel rests within the bound"
        );
        assert_eq!(never.lanes_at_rest(), all, "width={lanes}: the reference");
        for _ in 0..SKIPPED {
            run(&mut mixed, &mut never, false, true, true);
        }
        mixed.desymmetrize();
        for lane in 0..lanes {
            assert_eq!(
                (
                    test_support::bank_lane_state_words(&mixed, lane),
                    test_support::bank_lane_silence_words(&mixed, lane),
                ),
                (
                    test_support::bank_lane_state_words(&never, lane),
                    test_support::bank_lane_silence_words(&never, lane),
                ),
                "width={lanes} lane={lane}: the state after the skip and the disengage"
            );
        }
        for _ in 0..2 {
            run(&mut mixed, &mut never, true, false, false);
        }
    }
}
