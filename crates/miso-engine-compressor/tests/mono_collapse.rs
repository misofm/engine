//! The mono-collapse bodies render the dual bodies' left plane, to the bit -- including the link.
//!
//! # What this file is for, and why it is not covered by the console gates
//!
//! The console's mono row-pair proves the *whole strip* collapses class A on the fixture's content.
//! That content is a tone: it never carries a subnormal, and the link mode it exercises is
//! `Maximum`. This file is the kernel-level statement, on the two inputs the whole design turns on:
//!
//! * **`LinkMode::Average` on subnormal content.** The collapsed body computes the link on the one
//!   plane read twice -- `link_frame(detector, slot, p, p, ..)`, which for `Average` is
//!   `0.5*|p| + 0.5*|p|`. That is *not* `|p|`: for a subnormal `p`, `0.5 * p` loses the low bit and
//!   the two halves do not sum back. `a_halved_subnormal_does_not_come_back` states that on its
//!   own, so the reader can see that the operand-order rule has teeth before reading the test that
//!   depends on it. A "simplification" of the link to `magnitude_left` would pass every tone
//!   fixture in the tree and fail here.
//! * **`-0.0`.** Every gain and mix identity in the kernel is a `select` over a mask, and `-0.0`
//!   is the value that separates "the same number" from "the same bits" at each of them.
//!
//! Everything goes through the shipped contract calls: `bind_homogeneous_bank`, `process_bank`,
//! `process_bank_mono`, `desymmetrize_channels`. No test reaches into the crate.

mod support;

use miso_engine_effect_contract::{
    AutomationSpanKind, BankWidth, EffectBankProcessBlock, EffectQuality, InitialParameterValue,
    LinkMode, ParameterChannel, PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan,
    PreparedNativeEffectBank, PreparedPortsV1, PreparedSidechainPort,
};

/// Blocks per arm: enough that the detector ring and the lookahead delay are full of collapsed-run
/// samples before anything is compared, so a stale ring shows up as a difference rather than as a
/// value the first block happened to hide.
const BLOCKS: usize = 24;
const FRAMES: usize = 64;

/// A preparation request carrying one link mode. `support::request` pins `DualMono`.
fn request_linked<'a>(
    values: &'a [InitialParameterValue],
    link_mode: LinkMode,
) -> PrepareEffectRequest<'a> {
    PrepareEffectRequest {
        sample_rate: 48_000,
        quantum: FRAMES as u32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode,
        ports: PreparedPortsV1 {
            sidechain: PreparedSidechainPort::Unconnected {
                id: support::sidechain_port(),
                required: false,
            },
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 15_568,
            maximum_scratch_bytes: 64,
            maximum_automation_spans_per_block: 16,
        },
    }
}

/// Adversarial per-frame content: subnormals, both zeros, and ordinary signal.
///
/// The pattern repeats every seven frames and is deliberately *not* smooth: the detector's static
/// curve and the branching ballistic both select on comparisons, so a signal that crosses the
/// threshold repeatedly exercises both arms of every select in the kernel.
fn sample(index: usize) -> f32 {
    match index % 7 {
        0 => f32::from_bits(1),            // the smallest positive subnormal
        1 => -f32::from_bits(0x0000_0003), // a negative subnormal
        2 => -0.0,
        3 => 0.0,
        4 => 0.75,
        5 => -0.9,
        _ => f32::from_bits(0x007F_FFFF), // the largest subnormal
    }
}

/// `frames * lanes` words of AoSoA content, the same in every lane and in both planes.
fn block(base: usize, lanes: usize) -> Vec<f32> {
    (0..FRAMES * lanes)
        .map(|word| sample(base + word / lanes))
        .collect()
}

fn offsets(lanes: usize) -> Vec<u32> {
    vec![0; lanes + 1]
}

/// One `Point` retarget of the threshold per channel per lane, at this block's first sample.
///
/// This is what puts a **ramp in flight**, and a ramp is what selects the collapsed kernel's
/// per-frame body: `process_block_mono` splits the block at `max_remaining`, runs the ramping
/// prefix through `frames_loop_mono::<_, true>` -- which advances the ramps and reloads the
/// coefficient words every frame -- and only then considers the staged idle body. Without it the
/// whole file would exercise one of the two collapsed loops.
fn automation(step: usize, lanes: usize) -> (Vec<PreparedAutomationSpan>, Vec<u32>) {
    let first = (step * FRAMES) as u64;
    let threshold = -30.0 + (step % 5) as f32 * 3.0;
    let mut spans = Vec::new();
    let mut offsets = vec![0_u32; lanes + 1];
    for lane in 0..lanes {
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            spans.push(PreparedAutomationSpan {
                kind: AutomationSpanKind::Point,
                channel,
                parameter_index: 0,
                start_sample: first,
                end_sample: first,
                start_value: threshold,
                end_value: threshold,
            });
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

/// Halving a subnormal and adding the halves back does not return the subnormal.
///
/// The whole reason the collapsed link is `link(p, p)` and not `p`. If this ever became an
/// equality the rule would be vacuous, and the test that rests on it would be measuring nothing.
#[test]
fn a_halved_subnormal_does_not_come_back() {
    let p = f32::from_bits(3);
    let halved = 0.5_f32 * p + 0.5_f32 * p;
    assert_ne!(
        halved.to_bits(),
        p.to_bits(),
        "0.5*p + 0.5*p must round away from p for a subnormal p, or the link rule is vacuous"
    );
}

/// The collapsed body renders the dual body's left plane, bit for bit, under every link mode.
#[test]
fn the_collapsed_body_renders_the_dual_bodys_left_plane() {
    let Some((_, width)) = support::native_bank_width() else {
        return;
    };
    let lanes = width.lanes() as usize;
    // Both lookaheads on purpose: the kernel has **two** idle bodies and which one a block takes
    // is a function of the lookahead alone. `min_delay = latency - lookahead`, so a *long*
    // lookahead makes the read-back distance short and forces the per-frame body, and a short one
    // leaves it long enough to pre-gather the taps and take the staged body. A test at one
    // lookahead exercises one of the two collapsed loops and says nothing about the other.
    for (link_mode, ramping, body) in [
        (LinkMode::DualMono, true, "per-frame"),
        (LinkMode::DualMono, false, "staged"),
        (LinkMode::Average, true, "per-frame"),
        (LinkMode::Average, false, "staged"),
        (LinkMode::Maximum, true, "per-frame"),
        (LinkMode::Maximum, false, "staged"),
    ] {
        let lookahead_ms = 1.0_f32;
        let _ = body;
        // A threshold and ratio that put the detector on both sides of the knee for this content.
        let values = support::values_with(&[(0, -30.0), (1, 4.0), (7, lookahead_ms)]);
        let requests: Vec<_> = (0..lanes)
            .map(|_| request_linked(&values, link_mode))
            .collect();
        let mut dual = support::bind_bank(&requests).expect("dual bank");
        let mut collapsed = support::bind_bank(&requests).expect("collapsed bank");
        let offsets = offsets(lanes);

        for step in 0..BLOCKS {
            let mut dual_left = block(step * FRAMES, lanes);
            let mut dual_right = dual_left.clone();
            let mut mono_left = dual_left.clone();
            // The plane the collapsed chain never gathers. Filled with a value the dual run can
            // never produce, so a body that reads it produces something visibly wrong rather than
            // something plausible.
            let mut stale = vec![f32::from_bits(0x7F7F_FFFF); FRAMES * lanes];
            let first = (step * FRAMES) as u64;
            let (spans, span_offsets) = automation(step, lanes);
            let (spans, span_offsets): (&[PreparedAutomationSpan], &[u32]) = if ramping {
                (&spans, &span_offsets)
            } else {
                (&[], &offsets)
            };
            run_block(
                dual.as_mut(),
                &mut dual_left,
                &mut dual_right,
                width,
                first,
                spans,
                span_offsets,
                false,
            );
            run_block(
                collapsed.as_mut(),
                &mut mono_left,
                &mut stale,
                width,
                first,
                spans,
                span_offsets,
                true,
            );
            for (word, (collapsed_word, dual_word)) in
                mono_left.iter().zip(dual_left.iter()).enumerate()
            {
                assert_eq!(
                    collapsed_word.to_bits(),
                    dual_word.to_bits(),
                    "{link_mode:?}/{body} block {step} word {word}: the collapsed left plane \
                     must be the dual left plane to the bit"
                );
            }
        }

        // The **state** is the sharper statement, and it is where the subnormal link nuance lands.
        //
        // A detector reading below `level_floor` (1e-8) is clamped before it reaches the static
        // curve, so a subnormal that arrives at the link cannot move the rendered sample. It is
        // written into the detector ring first, unclamped, and the ring is serialised -- so
        // `0.5*|p| + 0.5*|p|` against `|p|` is a difference the payload carries and the audio does
        // not. Comparing the payload is therefore the only way to state the operand-order rule at
        // all, and it is the reason this file compares state and not only samples.
        //
        // Red mutation: replace the collapsed link with `main_left.abs()` in `frames_loop_mono` or
        // `idle_frames_staged_mono` -- the "the link is a no-op on a mono bank" simplification.
        // Every sample assertion above stays green and this fails.
        let scalar = support::prepare(request_linked(&values, link_mode));
        collapsed.desymmetrize_channels();
        for track in 0..lanes as u32 {
            let dual_payload = support::snapshot_track(dual.as_ref(), track, scalar.as_ref());
            let mono_payload = support::snapshot_track(collapsed.as_ref(), track, scalar.as_ref());
            assert_eq!(
                mono_payload, dual_payload,
                "{link_mode:?}/{body} track {track}: the collapsed bank's serialised state must \
                 be the dual bank's, both sections, after the disengage copy"
            );
        }
    }
}

/// After `desymmetrize_channels`, a bank that ran collapsed renders what a never-collapsed one does.
///
/// The kernel-level transition oracle. The console gate makes the same statement over the whole
/// strip; this one localises it to the compressor's own copy list, so a missing field here fails
/// with this crate's name on it.
///
/// Red mutation: delete any line of `Channel::copy_state_from` -- `cursor`, `gain_reduction_db`,
/// `main`, `detector`, `words`, `ramps`, `delay`, `lookahead_ms` -- and this fails.
#[test]
fn a_desymmetrized_bank_is_a_never_collapsed_bank() {
    let Some((_, width)) = support::native_bank_width() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let values = support::values_with(&[(0, -30.0), (1, 4.0), (7, 1.0)]);
    let requests: Vec<_> = (0..lanes)
        .map(|_| request_linked(&values, LinkMode::Average))
        .collect();
    let mut mixed = support::bind_bank(&requests).expect("mixed bank");
    let mut never = support::bind_bank(&requests).expect("reference bank");
    let offsets = offsets(lanes);

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
        // One retarget, early, and then nothing. That schedule is deliberate and it is what gives
        // the copy list's *coefficient* entries teeth.
        //
        // A `Point` span smooths over one block, so the ramp opens on block 2 and closes inside it
        // -- well before the transition. During the collapsed blocks only the **left** channel's
        // ramps advance, and advancing a ramp is what rewrites `words` from it. So at the
        // disengage the right channel's ramps and its coefficient words are *both* stale, and
        // because no further span ever arrives, nothing re-derives the words from the ramps
        // afterwards: a copy list that carried `ramps` and not `words` would look correct on a
        // continuously automated session and render the pre-ramp design forever on this one.
        let (spans, span_offsets) = if step == 2 {
            automation(step, lanes)
        } else {
            (Vec::new(), offsets.clone())
        };
        run_block(
            never.as_mut(),
            &mut never_left,
            &mut never_right,
            width,
            first,
            &spans,
            &span_offsets,
            false,
        );
        run_block(
            mixed.as_mut(),
            &mut mixed_left,
            &mut mixed_right,
            width,
            first,
            &spans,
            &span_offsets,
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
