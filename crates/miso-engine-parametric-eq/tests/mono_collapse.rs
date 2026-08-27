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

use miso_engine_effect_contract::{
    BankWidth, EffectBankProcessBlock, NativeEffectFactory, ParameterChannel,
    PrepareEffectBankRequest, PreparedAutomationSpan, PreparedNativeEffectBank,
};
use miso_engine_lane::Backend;
use miso_engine_parametric_eq::ParametricEqFactory;
use support::{point, request, set_initial, values};

const BLOCKS: usize = 24;
const FRAMES: usize = 128;

fn native_bank() -> Option<(BankWidth, Backend)> {
    let backend = Backend::current();
    BankWidth::for_backend(backend).map(|width| (width, backend))
}

/// Four live bands per track, distinct per lane so no two lanes share coefficients.
fn configured(track: usize) -> Vec<miso_engine_effect_contract::InitialParameterValue> {
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

/// A band-gain retarget on both channels, at this block's first sample, for every lane.
///
/// This is what puts a ramp in flight. Both channels are addressed with the same value, so the
/// dual bank's two channels stay bit-equal -- the point is not asymmetry, it is that `remaining`,
/// `coef`, `step` and `target` are all *moving* when the collapse disengages.
fn automation(step: usize, lanes: usize) -> (Vec<PreparedAutomationSpan>, Vec<u32>) {
    let first = (step * FRAMES) as u64;
    let gain = -6.0 + (step % 4) as f32 * 4.0;
    let mut spans = Vec::new();
    let mut offsets = vec![0_u32; lanes + 1];
    for lane in 0..lanes {
        spans.push(point(3, ParameterChannel::Left, first, gain));
        spans.push(point(3, ParameterChannel::Right, first, gain));
        offsets[lane + 1] = spans.len() as u32;
    }
    (spans, offsets)
}

#[allow(clippy::too_many_arguments)]
fn run_block(
    bank: &mut dyn PreparedNativeEffectBank,
    left: &mut [f32],
    right: &mut [f32],
    width: BankWidth,
    first_sample: u64,
    spans: &[PreparedAutomationSpan],
    offsets: &[u32],
    mono: bool,
) {
    let block = EffectBankProcessBlock::new(
        left,
        right,
        None,
        FRAMES as u32,
        width,
        first_sample,
        spans,
        offsets,
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
        let idle = vec![0_u32; lanes + 1];

        for step in 0..BLOCKS {
            let mut dual_left = block(step * FRAMES, lanes);
            let mut dual_right = dual_left.clone();
            let mut mono_left = dual_left.clone();
            // The plane the collapsed chain never gathers, filled with a value the dual run cannot
            // produce so that a body which reads it is visibly wrong rather than plausibly so.
            let mut stale = vec![f32::from_bits(0x7F7F_FFFF); FRAMES * lanes];
            let first = (step * FRAMES) as u64;
            let (spans, packed) = automation(step, lanes);
            let (spans, packed): (&[PreparedAutomationSpan], &[u32]) = if ramping {
                (&spans, &packed)
            } else {
                (&[], &idle)
            };
            run_block(
                dual.as_mut(),
                &mut dual_left,
                &mut dual_right,
                width,
                first,
                spans,
                packed,
                false,
            );
            run_block(
                collapsed.as_mut(),
                &mut mono_left,
                &mut stale,
                width,
                first,
                spans,
                packed,
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
    let idle = vec![0_u32; lanes + 1];

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
        let (spans, packed) = if step == 2 {
            automation(step, lanes)
        } else {
            (Vec::new(), idle.clone())
        };
        run_block(
            never.as_mut(),
            &mut never_left,
            &mut never_right,
            width,
            first,
            &spans,
            &packed,
            false,
        );
        run_block(
            mixed.as_mut(),
            &mut mixed_left,
            &mut mixed_right,
            width,
            first,
            &spans,
            &packed,
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
