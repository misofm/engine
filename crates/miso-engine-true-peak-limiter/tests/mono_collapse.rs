//! The mono-collapse body renders the dual body's left plane, and the disengage copy is complete.
//!
//! The crate-level statement of what the strip's console gates check end to end, and it is here as
//! well as there for one reason: the console fixture carries no automation, so **no ramp is ever in
//! flight while a track is collapse-eligible** -- every parameter write in this engine is addressed
//! to one channel and therefore clears the witness' `LIVE` term on the block it lands. That makes
//! the ramp-carrying entries of this effect's disengage copy list unreachable from the engine, and
//! reachable from here, where the contract call is made directly.
//!
//! The content is deliberately **loud** and the ceiling deliberately low: a signal the limiter
//! never reduces leaves `required_ring` at `1.0` and makes the twelve oversampling history taps
//! unobservable, so a corpus below the ceiling would say nothing about half of this kernel's state.

use miso_engine_effect_contract::{
    AutomationSpanKind, BankWidth, EffectBankProcessBlock, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, ParameterChannel, PrepareEffectBankRequest, PrepareEffectLimits,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedNativeEffectBank, PreparedPorts,
    PreparedSidechainPort,
};
use miso_engine_lane::Backend;
use miso_engine_true_peak_limiter::{
    TRUE_PEAK_LIMITER_DESCRIPTOR, TRUE_PEAK_LIMITER_PARAMETERS, TruePeakLimiterFactory,
};

/// Enough blocks for the lookahead line, the van Herk window and the release to be full of
/// collapsed-run samples before anything is compared.
const BLOCKS: usize = 24;
const FRAMES: usize = 128;

fn native_bank() -> Option<(BankWidth, Backend)> {
    let backend = Backend::current();
    BankWidth::for_backend(backend).map(|width| (width, backend))
}

fn values() -> [InitialParameterValue; 6] {
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: TRUE_PEAK_LIMITER_PARAMETERS[index / 2].default_value,
    })
}

fn request(values: &[InitialParameterValue]) -> PrepareEffectRequest<'_> {
    let quality = TRUE_PEAK_LIMITER_DESCRIPTOR
        .qualities
        .iter()
        .find(|quality| quality.sample_rate == 48_000)
        .expect("launch rate");
    PrepareEffectRequest {
        sample_rate: 48_000,
        quantum: FRAMES as u32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: quality.maximum_state.total().expect("state total"),
            maximum_scratch_bytes: 24,
            maximum_automation_spans_per_block: 16,
        },
    }
}

fn bind(width: BankWidth, backend: Backend, lanes: usize) -> Box<dyn PreparedNativeEffectBank> {
    let values = values();
    let requests: Vec<_> = (0..lanes).map(|_| request(&values)).collect();
    TruePeakLimiterFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
        })
        .expect("valid bank request")
        .expect("the native width must bind")
}

/// Loud, sign-alternating content with `-0.0` frames salted through it.
fn sample(index: usize) -> f32 {
    match index % 6 {
        0 => 0.98,
        1 => -0.93,
        2 => -0.0,
        3 => 0.0,
        4 => 0.62,
        _ => -0.87,
    }
}

/// One block of content, with a per-block envelope.
///
/// The envelope matters: a steady loud signal drives the release recursion to a fixed point within
/// a few blocks, and a frozen recursive word that has converged to the same value as a live one is
/// a state difference no output can show. Alternating loud and quiet blocks keeps `reduction`
/// moving for the whole run, which is what makes it a word the disengage copy has to carry.
fn block(step: usize, lanes: usize) -> Vec<f32> {
    let envelope = match step % 4 {
        0 => 1.0,
        1 => 0.04,
        2 => 0.7,
        _ => 0.01,
    };
    (0..FRAMES * lanes)
        .map(|word| sample(step * FRAMES + word / lanes) * envelope)
        .collect()
}

/// A ceiling and a release retarget on both channels, at this block's first sample, per lane.
///
/// Both channels take the same value, so the dual bank's two channels stay bit-equal: the point is
/// not asymmetry, it is that `limit` and `release` are *moving* when the collapse disengages, and
/// that only the collapsed channel's copies of them advanced.
///
/// The span order is the one this effect validates: strictly ascending `parameter_index * 2 +
/// channel`.
fn automation(step: usize, lanes: usize) -> (Vec<PreparedAutomationSpan>, Vec<u32>) {
    let first = (step * FRAMES) as u64;
    let ceiling = -18.0 - (step % 3) as f32 * 2.0;
    let release = 260.0 + (step % 4) as f32 * 30.0;
    let mut spans = Vec::new();
    let mut offsets = vec![0_u32; lanes + 1];
    for lane in 0..lanes {
        for (parameter, value) in [(0_u32, ceiling), (1, release)] {
            for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                spans.push(PreparedAutomationSpan {
                    kind: AutomationSpanKind::Point,
                    channel,
                    parameter_index: parameter,
                    start_sample: first,
                    end_sample: first,
                    start_value: value,
                    end_value: value,
                });
            }
        }
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

/// The collapsed body renders the dual body's left plane, ramping and stationary.
///
/// The stationary arm is the one that exercises `limiter_block_uniform_mono` -- the van Herk
/// segment walk, whose collapsed form hands the walk one channel's window offsets for both of its
/// arguments -- and the ramping arm the per-lane body beside it.
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
            let mut dual_left = block(step, lanes);
            let mut dual_right = dual_left.clone();
            let mut mono_left = dual_left.clone();
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
/// One retarget, early, and then none: the ramps open and close well before the transition, so at
/// the disengage the right channel's `limit`, `release` and every ring are stale and nothing
/// re-derives them afterwards.
///
/// Red mutation: delete any line of `ChannelState::copy_state_from`.
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
        let mut never_left = block(step, lanes);
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
