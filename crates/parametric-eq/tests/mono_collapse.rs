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
    PrepareEffectBankRequest, PreparedNativeEffectBank,
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

fn bind(width: BankWidth, backend: Backend, lanes: usize) -> Box<dyn PreparedNativeEffectBank> {
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
        })
        .expect("valid bank request")
        .expect("the native width must bind")
}

fn bind_lpf(width: BankWidth, backend: Backend, lanes: usize) -> Box<dyn PreparedNativeEffectBank> {
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
                dual.as_mut(),
                &mut dual_left,
                &mut dual_right,
                width,
                first,
                step,
                ramping,
                false,
            );
            run_block(
                collapsed.as_mut(),
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
            mixed.desymmetrize_channels();
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
            never.as_mut(),
            &mut never_left,
            &mut never_right,
            width,
            first,
            step,
            step == 2,
            false,
        );
        run_block(
            mixed.as_mut(),
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
        dual.as_mut(),
        &mut dual_left,
        &mut dual_right,
        width,
        0,
        0,
        false,
        false,
    );
    run_block(
        collapsed.as_mut(),
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
