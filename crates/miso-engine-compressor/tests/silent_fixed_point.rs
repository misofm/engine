//! Issue #163 phase 4 item 1 at the compressor boundary: the silent fast path moves no bit.
//!
//! # Why this effect is the one that pins the state-unchanged requirement
//!
//! A compressor's output is `input * gain`, so with an all-`+0.0` input the output is `+0.0` from
//! the *first* silent sample — long before the detector has finished releasing. That makes it the
//! effect where "the output is silent" and "the state has settled" come apart the furthest, and it
//! is why the fast path's claim is earned on the recursive word coming out of a block bit-identical
//! to the way it went in, and never on the output alone.
//!
//! A fast path that engaged on silent output would freeze `gain_reduction_db` part-way through its
//! release. Nothing about the silence would reveal it: the silent blocks are `+0.0` either way.
//! The tone that follows is what exposes it, because it would be compressed by a gain that was
//! never allowed to finish releasing. `a_settled_silent_bank_renders_exactly_the_never_fast_path`
//! is red for exactly that mutation.
//!
//! The control arm suppresses the fast path with a redundant automation point on every block,
//! which `process_bank` treats as grounds to withdraw the claim.

mod support;

use miso_engine_effect_contract::{
    AutomationSpanKind, EffectBankProcessBlock, ParameterChannel, PreparedAutomationSpan,
};
use support::{bind_bank, native_bank_width, request, values_with};

const FRAMES: usize = 128;
/// Release is 50 ms ~= 19 blocks at 48 kHz and a 128-frame quantum, so the detector is still
/// moving well into the silent stretch and has ample room to finish inside it.
const RELEASE_MS: f32 = 50.0;
const SILENT_BLOCKS: usize = 160;
/// Enough trailing tone blocks for the tone to clear the lookahead delay line.
///
/// Measured rather than assumed: a fresh bank at the descriptor defaults renders exact `+0.0` for
/// its first **seven** blocks and only becomes nonzero on block 7, so the delay in front of the
/// output is around 896 frames rather than the 240 the 5 ms default lookahead suggests. Four
/// trailing blocks left the comparison with nothing but silence to compare, which is precisely
/// what `the_trailing_tone_is_actually_rendered` caught.
const TRAILING_TONE_BLOCKS: usize = 12;
/// Deliberately **above** the tone's level, so the detector rests at an exact `0.0` dB reduction.
///
/// This is what makes the fast path reachable at all here, and the reason is worth stating: a
/// compressor that has actually been reducing gain releases *asymptotically*. The recursive word
/// approaches `0.0` dB geometrically and only becomes bit-stable once the flush catches it, which
/// at a 50 ms release is on the order of a thousand blocks away. The claim in `Instance::render`
/// requires the word to come out of a block bit-identical, so it correctly refuses to engage for
/// all of that time -- which is the "where the fixed point is not exactly reachable, do not
/// engage" rule doing its job, and is pinned by
/// `a_compressing_detector_never_earns_the_claim_while_it_releases`.
const THRESHOLD_DB: f32 = -6.0;
const RATIO: f32 = 8.0;
/// Tone amplitude, ~-26 dBFS: comfortably under `THRESHOLD_DB`.
const TONE_AMPLITUDE: f32 = 0.05;

/// A block of loud tone, or a block of exact `+0.0`.
fn plane(block: usize, lanes: usize, silent: bool, negate: bool) -> Vec<f32> {
    (0..FRAMES * lanes)
        .map(|index| {
            if silent {
                0.0
            } else {
                let value =
                    (((block * FRAMES * lanes + index) as f32) * 0.031).sin() * TONE_AMPLITUDE;
                if negate { -value } else { value }
            }
        })
        .collect()
}

/// Renders tone, then a long silence, then tone again, returning every output bit.
///
/// `restate` delivers a redundant point on every block, which forces the slow path everywhere.
fn render(lanes: usize, restate: bool) -> Vec<u32> {
    render_with(lanes, restate, RELEASE_MS, SILENT_BLOCKS, THRESHOLD_DB)
}

/// The parameterised form: `release_ms` sets how long the detector keeps moving into the silence,
/// and `silent_blocks` how much silence there is for it to move through.
fn render_with(
    lanes: usize,
    restate: bool,
    release_ms: f32,
    silent_blocks: usize,
    threshold_db: f32,
) -> Vec<u32> {
    let values = values_with(&[(0, threshold_db), (1, RATIO), (4, release_ms)]);
    let requests: Vec<_> = (0..lanes).map(|_| request(&values)).collect();
    let mut bank = bind_bank(&requests).expect("a native bank width");

    let full_offsets: Vec<u32> = (0..=lanes).map(|track| track as u32).collect();
    let empty_offsets = vec![0_u32; lanes + 1];

    let total = 1 + silent_blocks + TRAILING_TONE_BLOCKS;
    let mut bits = Vec::new();
    for block in 0..total {
        let silent = block > 0 && block <= silent_blocks;
        let mut left = plane(block, lanes, silent, false);
        let mut right = plane(block, lanes, silent, true);
        let first_sample = (block * FRAMES) as u64;
        // The redundant point restates the threshold at the value it already holds.
        let restated: Vec<PreparedAutomationSpan> = if restate {
            (0..lanes)
                .map(|_| PreparedAutomationSpan {
                    kind: AutomationSpanKind::Point,
                    channel: ParameterChannel::Left,
                    parameter_index: 0,
                    start_sample: first_sample,
                    end_sample: first_sample,
                    start_value: threshold_db,
                    end_value: threshold_db,
                })
                .collect()
        } else {
            Vec::new()
        };
        let offsets: &[u32] = if restate {
            &full_offsets
        } else {
            &empty_offsets
        };
        bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                FRAMES as u32,
                native_bank_width().expect("width").1,
                first_sample,
                &restated,
                offsets,
                128,
            )
            .expect("bank block"),
        );
        bits.extend(left.iter().map(|value| value.to_bits()));
        bits.extend(right.iter().map(|value| value.to_bits()));
    }
    bits
}

/// **A bank allowed to skip settled silence renders exactly the bank that is never allowed to.**
///
/// Red mutations this holds against: dropping the `block_is_positive_zero` input test, and
/// dropping `rings_are_positive_zero`. The `recursive_bits` term is *not* exercised here, because
/// this arm's tone is under the threshold and the detector therefore rests at an exact `0.0` dB
/// the whole way through — `a_detector_still_releasing_through_the_silence_is_never_frozen` is the
/// test that makes that term load-bearing.
///
/// The cursor advance in the fast path is deliberately **not** claimed to be covered by any red
/// mutation, and removing it passes every test in this file. That is honest rather than
/// convenient: once both rings are entirely `+0.0`, every read out of them returns `+0.0` from any
/// cursor position, so the cursor's value is genuinely unobservable for as long as the claim
/// holds. It is advanced anyway so that the skipped block leaves the state *bit-identical* to the
/// block that ran, rather than merely observationally equivalent to it — a weaker invariant would
/// have to be re-proved every time this kernel's ring handling changed.
#[test]
fn a_settled_silent_bank_renders_exactly_the_never_fast_path() {
    let Some((_, width)) = native_bank_width() else {
        return;
    };
    let lanes = width.lanes() as usize;

    let fast = render(lanes, false);
    let forced_slow = render(lanes, true);

    assert_eq!(
        fast.len(),
        forced_slow.len(),
        "both arms must render the same shape"
    );
    assert_eq!(
        fast, forced_slow,
        "the silent fast path moved a rendered bit against the never-fast-path arm"
    );
}

/// The trailing tone is not silence: without it the test above would pass on a fast path that
/// simply stopped rendering, so this pins that the comparison has something to compare.
#[test]
fn the_trailing_tone_is_actually_rendered() {
    let Some((_, width)) = native_bank_width() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let bits = render(lanes, false);
    let per_block = FRAMES * lanes * 2;
    let tail = &bits[bits.len() - per_block..];
    assert!(
        tail.iter().any(|word| *word != 0),
        "the block after the silence rendered nothing at all"
    );
}

/// Isolates the **recursive-word** leg of the claim: a detector that is still releasing when the
/// tone returns must not have been frozen by the fast path.
///
/// A 2 000 ms release is ~750 blocks at this quantum, so across 24 blocks of silence the detector
/// is still moving the whole way and the correct code never earns the claim at all. A fast path
/// that dropped the `recursive_bits` before/after comparison would earn it on the *first* silent
/// block — the output is `+0.0` from the first silent sample, because a compressor multiplies —
/// and would then hold `gain_reduction_db` at its compressed value for the rest of the silence.
/// The returning tone is then compressed by a gain that never finished releasing.
///
/// Red mutation: remove `left_before == self.left.recursive_bits() && right_before == ...` from
/// the claim in `Instance::render`.
#[test]
fn a_detector_still_releasing_through_the_silence_is_never_frozen() {
    let Some((_, width)) = native_bank_width() else {
        return;
    };
    let lanes = width.lanes() as usize;
    // -40 dB against a ~-26 dBFS tone, so this arm really is reducing gain and really is still
    // releasing when the tone returns.
    let fast = render_with(lanes, false, 2_000.0, 24, -40.0);
    let forced_slow = render_with(lanes, true, 2_000.0, 24, -40.0);
    assert_eq!(
        fast, forced_slow,
        "a detector mid-release was frozen by the silent fast path"
    );
}

/// Isolates the **lookahead-ring** leg: silence shorter than the delay line must not let the fast
/// path skip over tone that is still inside it.
///
/// The minimum 5 ms release settles the detector within about two blocks, so by the third silent
/// block the recursive word is at its fixed point — while the ~896-frame delay line still holds
/// seven blocks of the tone that has not reached the output yet. That is the one window where the
/// recursive word says "settled" and the rings say "not yet", and `rings_are_positive_zero` is
/// what keeps the fast path out of it. Without that term the cursor jumps over the tone still in
/// the line and it is never rendered.
///
/// Red mutation: remove `self.left.rings_are_positive_zero() && self.right...` from the claim.
#[test]
fn silence_shorter_than_the_lookahead_line_still_drains_it() {
    let Some((_, width)) = native_bank_width() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let fast = render_with(lanes, false, 5.0, 4, THRESHOLD_DB);
    let forced_slow = render_with(lanes, true, 5.0, 4, THRESHOLD_DB);
    assert_eq!(
        fast, forced_slow,
        "the fast path skipped tone that was still inside the lookahead line"
    );
}
