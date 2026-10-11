//! The mono-collapse body renders the dual body's left plane, and the disengage copy is complete.
//!
//! The crate-level statement of what the strip's console gates check end to end. It is here as well
//! as there for one reason: the console fixture carries no automation, so **no ramp is ever in
//! flight while a track is collapse-eligible** -- every parameter write in this engine is addressed
//! to one channel and therefore clears the witness' `LIVE` term on the block it lands. That makes
//! the ramp-carrying entries of this effect's disengage copy list unreachable from the engine, and
//! reachable from here, where the contract call is made directly.
//!
//! Everything goes through the shipped contract calls: `bind_homogeneous_bank`, `process_bank`,
//! `process_bank_mono`, `desymmetrize_channels`.

mod support;

use effect_contract::{
    BankWidth, EffectBankProcessBlock, NativeEffectFactory, ParameterChannel,
    PrepareEffectBankRequest, PreparedEffectBank, PreparedNativeEffectBank,
};
use lane::Backend;
use parametric_eq::ParametricEqFactory;
use support::{apply_prepared_targets_lane, request, set_initial, values};

const BLOCKS: usize = 24;
const FRAMES: usize = 128;

fn native_bank() -> Option<(BankWidth, Backend)> {
    let backend = Backend::current();
    BankWidth::for_backend(backend).map(|width| (width, backend))
}

/// Four live bands per track, distinct per lane so no two lanes share coefficients.
fn configured(track: usize) -> Vec<effect_contract::InitialParameterValue> {
    let mut configured = values();
    for band in 0..4 {
        let base = band * 6;
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            set_initial(&mut configured, base, channel, 1.0);
            set_initial(&mut configured, base + 1, channel, (band % 5 + 1) as f32);
            set_initial(
                &mut configured,
                base + 2,
                channel,
                200.0 * (band + 1) as f32 + 13.0 * track as f32,
            );
            set_initial(&mut configured, base + 3, channel, 3.0 - band as f32);
            set_initial(&mut configured, base + 4, channel, 0.9);
            set_initial(&mut configured, base + 5, channel, 1.0);
        }
    }
    configured
}

/// A prepared-only LPF with every original band and the HPF disabled. This isolates the final
/// physical section so a mono collapse regression cannot pass by filtering only the first section.
fn configured_lpf() -> Vec<effect_contract::InitialParameterValue> {
    let mut configured = values();
    for channel in [ParameterChannel::Left, ParameterChannel::Right] {
        set_initial(&mut configured, 27, channel, 1.0);
        set_initial(&mut configured, 28, channel, 1_000.0);
        set_initial(&mut configured, 29, channel, 0.7);
    }
    configured
}

/// Adversarial content: `-0.0`, both zeroes and ordinary signal, repeating every five frames.
///
/// No subnormals: the cascade's identity-elision gate refuses a block that carries one, so a
/// corpus of subnormals would take the un-elided path on every block and say nothing about the
/// elided one. The `-0.0` is the value the elision proof turns on and it is here for that.
fn sample(index: usize) -> f32 {
    match index % 5 {
        0 => -0.0,
        1 => 0.0,
        2 => 0.6,
        3 => -0.35,
        _ => 0.125,
    }
}

fn block(base: usize, lanes: usize) -> Vec<f32> {
    (0..FRAMES * lanes)
        .map(|word| sample(base + word / lanes))
        .collect()
}

/// [`block`] without its `-0.0` frames: `+0.0` and ordinary signal, repeating every four frames, so
/// the elision gate can admit the block.
fn clean_block(base: usize, lanes: usize) -> Vec<f32> {
    (0..FRAMES * lanes)
        .map(|word| match (base + word / lanes) % 4 {
            0 => 0.0,
            1 => 0.6,
            2 => -0.35,
            _ => 0.125,
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn run_block(
    bank: &mut dyn PreparedNativeEffectBank,
    left: &mut [f32],
    right: &mut [f32],
    width: BankWidth,
    first_sample: u64,
    step: usize,
    retarget: bool,
    mono: bool,
) {
    let lanes = width.lanes() as usize;
    if retarget {
        let gain = -6.0 + (step % 4) as f32 * 4.0;
        for lane in 0..lanes {
            let mut target_values = configured(lane);
            set_initial(&mut target_values, 3, ParameterChannel::Left, gain);
            set_initial(&mut target_values, 3, ParameterChannel::Right, gain);
            let mut changed = vec![false; target_values.len()];
            changed[3 * 2] = true;
            changed[3 * 2 + 1] = true;
            apply_prepared_targets_lane(bank, lane, 48_000, &target_values, &changed);
        }
    }
    let offsets = vec![0_u32; lanes + 1];
    let block = EffectBankProcessBlock::new(
        left,
        right,
        None,
        FRAMES as u32,
        width,
        first_sample,
        &[],
        &offsets,
        FRAMES as u32,
    )
    .expect("bounded bank block");
    if mono {
        bank.process_bank_mono(block);
    } else {
        bank.process_bank(block);
    }
}

fn bind(width: BankWidth, backend: Backend, lanes: usize) -> PreparedEffectBank {
    let values_by_track: Vec<_> = (0..lanes).map(configured).collect();
    let requests: Vec<_> = values_by_track
        .iter()
        .map(|values| request(values, false))
        .collect();
    ParametricEqFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        })
        .expect("valid bank request")
        .expect("the native width must bind")
}

fn bind_lpf(width: BankWidth, backend: Backend, lanes: usize) -> PreparedEffectBank {
    let values_by_track: Vec<_> = (0..lanes).map(|_| configured_lpf()).collect();
    let requests: Vec<_> = values_by_track
        .iter()
        .map(|values| request(values, false))
        .collect();
    ParametricEqFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        })
        .expect("valid LPF bank request")
        .expect("the native width must bind")
}

/// The collapsed cascade renders the dual cascade's left plane, ramping and stationary.
#[test]
fn the_collapsed_body_renders_the_dual_bodys_left_plane() {
    let Some((width, backend)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    for ramping in [true, false] {
        let mut dual = bind(width, backend, lanes);
        let mut collapsed = bind(width, backend, lanes);
        for step in 0..BLOCKS {
            let mut dual_left = block(step * FRAMES, lanes);
            let mut dual_right = dual_left.clone();
            let mut mono_left = dual_left.clone();
            // The plane the collapsed chain never gathers, filled with a value the dual run cannot
            // produce so that a body which reads it is visibly wrong rather than plausibly so.
            let mut stale = vec![f32::from_bits(0x7F7F_FFFF); FRAMES * lanes];
            let first = (step * FRAMES) as u64;
            run_block(
                dual.processor.as_mut(),
                &mut dual_left,
                &mut dual_right,
                width,
                first,
                step,
                ramping,
                false,
            );
            run_block(
                collapsed.processor.as_mut(),
                &mut mono_left,
                &mut stale,
                width,
                first,
                step,
                ramping,
                true,
            );
            for (word, (collapsed_word, dual_word)) in
                mono_left.iter().zip(dual_left.iter()).enumerate()
            {
                assert_eq!(
                    collapsed_word.to_bits(),
                    dual_word.to_bits(),
                    "ramping {ramping} block {step} word {word}"
                );
            }
        }
    }
}

/// After `desymmetrize_channels`, a bank that ran collapsed renders what a never-collapsed one does.
///
/// The retarget schedule is deliberate: **one** span, early, and then none. A `Point` span smooths
/// over one block, so the ramp opens and closes well before the transition, and during the
/// collapsed blocks only the left channel's `remaining` counts down and only its coefficient words
/// are advanced. Nothing re-derives the right channel's afterwards, so a copy list missing
/// `remaining` or `sections` renders the pre-ramp design on the right channel forever.
#[test]
fn a_desymmetrized_bank_is_a_never_collapsed_bank() {
    let Some((width, backend)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let mut mixed = bind(width, backend, lanes);
    let mut never = bind(width, backend, lanes);

    for step in 0..BLOCKS {
        let collapsed_half = step < BLOCKS / 2;
        if step == BLOCKS / 2 {
            mixed.processor.desymmetrize_channels();
        }
        let mut never_left = block(step * FRAMES, lanes);
        let mut never_right = never_left.clone();
        let mut mixed_left = never_left.clone();
        let mut mixed_right = if collapsed_half {
            vec![f32::from_bits(0x7F7F_FFFF); FRAMES * lanes]
        } else {
            never_left.clone()
        };
        let first = (step * FRAMES) as u64;
        run_block(
            never.processor.as_mut(),
            &mut never_left,
            &mut never_right,
            width,
            first,
            step,
            step == 2,
            false,
        );
        run_block(
            mixed.processor.as_mut(),
            &mut mixed_left,
            &mut mixed_right,
            width,
            first,
            step,
            step == 2,
            collapsed_half,
        );
        for (word, (mixed_word, never_word)) in mixed_left.iter().zip(never_left.iter()).enumerate()
        {
            assert_eq!(
                mixed_word.to_bits(),
                never_word.to_bits(),
                "block {step} word {word}: left plane"
            );
        }
        if !collapsed_half {
            for (word, (mixed_word, never_word)) in
                mixed_right.iter().zip(never_right.iter()).enumerate()
            {
                assert_eq!(
                    mixed_word.to_bits(),
                    never_word.to_bits(),
                    "block {step} word {word}: the right plane after the disengage"
                );
            }
        }
    }
}

/// The HPF's enable parameter index (`hpf-enabled`); 25 and 26 are its frequency and Q.
const HPF_ENABLED: usize = 24;

/// [`configured`] with the dedicated HPF at 120 Hz, switched on or off on both channels.
fn configured_with_hpf(track: usize, enabled: bool) -> Vec<effect_contract::InitialParameterValue> {
    let mut configured = configured(track);
    for channel in [ParameterChannel::Left, ParameterChannel::Right] {
        set_initial(
            &mut configured,
            HPF_ENABLED,
            channel,
            f32::from(u8::from(enabled)),
        );
        set_initial(&mut configured, HPF_ENABLED + 1, channel, 120.0);
        set_initial(&mut configured, HPF_ENABLED + 2, channel, 0.7);
    }
    configured
}

/// After `desymmetrize_channels`, the right channel's derived identity flags and dry masks are the
/// left channel's, when a ramp that changes them ended while the bank was collapsed.
///
/// Every lane switches the dedicated HPF (physical section 0) on both channels in block 2, while
/// the bank runs collapsed. Only the left channel's ramp advances, so only the left channel's
/// snap re-derives `identity` and `dry`; the right channel keeps what `start_ramp` derived when the
/// ramp began (section 0 identity or not as before, and no dry lane, since a ramp is in flight).
///
/// * **Off to on.** The left channel's section 0 stops being the identity. A right channel that
///   kept its own flag claims a live section is the identity.
/// * **On to off.** The left channel's section 0 becomes the identity, and every lane is dry.
///   A right channel that kept its own flag and mask claims section 0 is not the identity and has
///   no dry lane.
///
/// From block 12 the bank runs dual. Its stationary path asserts in debug builds that each
/// channel's flags and masks equal what its words say (`identity_flags_agree`), and both planes
/// are compared bit for bit with a bank that never collapsed. Dropping `identity`'s copy from
/// `desymmetrize`, or `dry`'s, turns this test red in a debug build, through that assertion on the
/// first dual block (issue #1328). `a_desymmetrized_bank_is_a_never_collapsed_bank` stays green on
/// both, since its ramp moves a general band's gain and changes neither.
///
/// A stale `identity` flag is not only a schedule choice: the dual elision gate drops a section
/// when *both* channels' flags say identity, so a stale right flag beside a fresh left one can
/// skip a section that is live on the right. Up to block 24 the input carries `-0.0` in every
/// block, which refuses elision on every block, and directly after `desymmetrize` the left
/// channel's correct flag also guards the AND, so release builds stay green there. From block 24
/// the input carries no `-0.0` ([`clean_block`]), and in block 26 a **left-only** retarget switches
/// the left HPF back to where it started. In the off -> on case the left channel's section 0 is
/// then the identity again while the right one is live, and a right channel still holding the flag
/// from before the collapsed ramp makes the gate skip it: the identity mutant is red in a release
/// build too (from block 27). A stale `dry` runs the HPF's lanes wet at the identity words, which
/// can move at most a `-0.0` to `+0.0` and, for this input and state, moves none, so only the debug
/// assertion catches that one.
#[test]
fn a_desymmetrized_bank_carries_the_collapsed_channels_identity_flags_and_dry_masks() {
    let Some((width, backend)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    for (before, after) in [(false, true), (true, false)] {
        let bind_hpf = |enabled: bool| {
            let values_by_track: Vec<_> = (0..lanes)
                .map(|track| configured_with_hpf(track, enabled))
                .collect();
            let requests: Vec<_> = values_by_track
                .iter()
                .map(|values| request(values, false))
                .collect();
            ParametricEqFactory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend,
                    width,
                    requests: &requests,
                    active_mask: width.full_mask(),
                })
                .expect("valid HPF bank request")
                .expect("the native width must bind")
        };
        let mut mixed = bind_hpf(before);
        let mut never = bind_hpf(before);
        for step in 0..2 * BLOCKS {
            let collapsed_half = step < BLOCKS / 2;
            if step == BLOCKS / 2 {
                mixed.processor.desymmetrize_channels();
            }
            // Both channels in block 2 (collapsed); the left channel alone in block 26 (dual).
            if step == 2 || step == BLOCKS + 2 {
                let both = step == 2;
                let enabled = if both { after } else { before };
                for bank in [mixed.processor.as_mut(), never.processor.as_mut()] {
                    for lane in 0..lanes {
                        let target_values = configured_with_hpf(lane, enabled);
                        let mut changed = vec![false; target_values.len()];
                        changed[HPF_ENABLED * 2] = true;
                        changed[HPF_ENABLED * 2 + 1] = both;
                        apply_prepared_targets_lane(bank, lane, 48_000, &target_values, &changed);
                    }
                }
            }
            let mut never_left = if step < BLOCKS {
                block(step * FRAMES, lanes)
            } else {
                clean_block(step * FRAMES, lanes)
            };
            let mut never_right = never_left.clone();
            let mut mixed_left = never_left.clone();
            let mut mixed_right = if collapsed_half {
                vec![f32::from_bits(0x7F7F_FFFF); FRAMES * lanes]
            } else {
                never_left.clone()
            };
            let first = (step * FRAMES) as u64;
            run_block(
                never.processor.as_mut(),
                &mut never_left,
                &mut never_right,
                width,
                first,
                step,
                false,
                false,
            );
            run_block(
                mixed.processor.as_mut(),
                &mut mixed_left,
                &mut mixed_right,
                width,
                first,
                step,
                false,
                collapsed_half,
            );
            let planes = if collapsed_half {
                vec![("left", &mixed_left, &never_left)]
            } else {
                vec![
                    ("left", &mixed_left, &never_left),
                    ("right", &mixed_right, &never_right),
                ]
            };
            for (plane, mixed_plane, never_plane) in planes {
                for (word, (mixed_word, never_word)) in
                    mixed_plane.iter().zip(never_plane.iter()).enumerate()
                {
                    assert_eq!(
                        mixed_word.to_bits(),
                        never_word.to_bits(),
                        "HPF {before} -> {after}, block {step} word {word}: {plane} plane"
                    );
                }
            }
        }
    }
}

/// The final LPF remains in the mono-collapse body and affects the gathered left plane.
#[test]
fn the_last_lpf_is_reached_when_the_bank_collapses_to_mono() {
    let Some((width, backend)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let mut dual = bind_lpf(width, backend, lanes);
    let mut collapsed = bind_lpf(width, backend, lanes);
    let mut dual_left = vec![0.0_f32; FRAMES * lanes];
    let mut dual_right = vec![0.0_f32; FRAMES * lanes];
    for lane in 0..lanes {
        dual_left[lane] = 1.0;
        dual_right[lane] = 1.0;
    }
    let input = dual_left.clone();
    let mut collapsed_left = input.clone();
    let mut stale_right = vec![f32::from_bits(0x7F7F_FFFF); FRAMES * lanes];
    run_block(
        dual.processor.as_mut(),
        &mut dual_left,
        &mut dual_right,
        width,
        0,
        0,
        false,
        false,
    );
    run_block(
        collapsed.processor.as_mut(),
        &mut collapsed_left,
        &mut stale_right,
        width,
        0,
        0,
        false,
        true,
    );
    for (word, (collapsed_word, dual_word)) in
        collapsed_left.iter().zip(dual_left.iter()).enumerate()
    {
        assert_eq!(
            collapsed_word.to_bits(),
            dual_word.to_bits(),
            "mono LPF collapse differs from dual left at word {word}"
        );
    }
    assert!(
        dual_left
            .iter()
            .zip(input.iter())
            .any(|(output, input)| output.to_bits() != input.to_bits()),
        "the final LPF must affect a collapsed impulse"
    );
    assert!(
        dual_left.iter().all(|sample| sample.is_finite()),
        "the collapsed final LPF must remain finite"
    );
}

/// The disengage copy carries the input's silence counter (issue #1328, amendment A9).
///
/// Collapsed blocks advance the left channel's counter alone; after `desymmetrize_channels` every
/// lane's right payload section must equal its left one, word for word, the counter included. A
/// copy list without it leaves the right channel's counter where the collapse froze it, so its
/// joint flush would arm late on a dual block after the disengage.
#[test]
fn a_desymmetrized_bank_carries_the_collapsed_channels_silence_counter() {
    let Some((width, backend)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let mut bank = bind(width, backend, lanes);
    let mut first = 0_u64;
    for step in 0..6 {
        let mut left = vec![0.0_f32; FRAMES * lanes];
        let mut right = left.clone();
        run_block(
            bank.processor.as_mut(),
            &mut left,
            &mut right,
            width,
            first,
            step,
            false,
            step >= 2,
        );
        first += FRAMES as u64;
    }
    bank.processor.desymmetrize_channels();
    for lane in 0..lanes {
        let mut common = vec![0_u8; support::COMMON_BYTES];
        let mut left = vec![0_u8; support::LANE_BYTES];
        let mut right = vec![0_u8; support::LANE_BYTES];
        bank.processor
            .snapshot_track_state_payload(
                lane as u32,
                effect_contract::StatePayloadOutput::new(
                    &mut common,
                    &mut left,
                    &mut right,
                    bank.metadata.program_key.state_sizes,
                )
                .expect("state output"),
            )
            .expect("snapshot");
        assert_eq!(
            support::word(&left, support::SILENCE_WORD),
            (6.0 * FRAMES as f32).to_bits(),
            "lane {lane}: the left counter counts every zero frame"
        );
        assert_eq!(
            left, right,
            "lane {lane}: the right channel after the disengage copy"
        );
    }
}

/// Issue #1328 (the root's cost ruling): the collapsed body arms its joint flush on the blocks the
/// dual body does. Every lane's band one holds a pair in the joint band, `(1e-15, -1e-15)`, on both
/// channels, with the counter one frame short of the 48 kHz window; one collapsed block of zeros
/// must clear the pair on every lane. Red if the collapsed render never takes the armed form, or
/// decides it from anything but its one channel's counter and state.
#[test]
fn the_collapsed_body_arms_its_joint_flush() {
    let Some((width, backend)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let mut bank = bind(width, backend, lanes);
    let sizes = bank.metadata.program_key.state_sizes;
    let band_one = support::WORDS_PER_BAND * 4;
    let snapshot = |bank: &dyn PreparedNativeEffectBank, lane: usize| {
        let mut common = vec![0_u8; support::COMMON_BYTES];
        let mut left = vec![0_u8; support::LANE_BYTES];
        let mut right = vec![0_u8; support::LANE_BYTES];
        bank.snapshot_track_state_payload(
            lane as u32,
            effect_contract::StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes)
                .expect("state output"),
        )
        .expect("snapshot");
        (common, left, right)
    };
    for lane in 0..lanes {
        let (common, mut left, mut right) = snapshot(bank.processor.as_ref(), lane);
        for channel in [&mut left, &mut right] {
            channel[band_one..band_one + 4].copy_from_slice(&1.0e-15_f32.to_bits().to_le_bytes());
            channel[band_one + 4..band_one + 8]
                .copy_from_slice(&(-1.0e-15_f32).to_bits().to_le_bytes());
            let at = support::SILENCE_WORD * 4;
            channel[at..at + 4].copy_from_slice(&4_095.0_f32.to_bits().to_le_bytes());
        }
        bank.processor
            .restore_track_state_payload(
                lane as u32,
                1,
                effect_contract::StatePayloadInput::new(&common, &left, &right, sizes)
                    .expect("state input"),
            )
            .expect("the payload restores");
    }
    let mut left = vec![0.0_f32; FRAMES * lanes];
    let mut stale = vec![f32::from_bits(0x7F7F_FFFF); FRAMES * lanes];
    run_block(
        bank.processor.as_mut(),
        &mut left,
        &mut stale,
        width,
        0,
        0,
        false,
        true,
    );
    for lane in 0..lanes {
        let (_, left, _) = snapshot(bank.processor.as_ref(), lane);
        assert_eq!(
            [
                support::word(&left, support::WORDS_PER_BAND),
                support::word(&left, support::WORDS_PER_BAND + 1)
            ],
            [0, 0],
            "lane {lane}: the collapsed block arms and clears band one's pair"
        );
    }
}
