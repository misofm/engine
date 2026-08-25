//! Issue #163 phase 4 item 1 at the effect boundary: the silent fast path moves no rendered bit.
//!
//! The bank may stop running its four sections once it has *observed* that an all-`+0.0` input
//! left its integrators bit-identical and produced an all-`+0.0` output. Everything downstream of
//! that claim is class A, so the property is stated the way a session observes it: **a bank that
//! is allowed to take the fast path must render exactly what the same bank renders when it is
//! never allowed to take it.**
//!
//! # How the control arm suppresses the fast path
//!
//! `process_bank` withdraws the claim on any block that carries automation at all, because a span
//! with no smoothing snaps the coefficient words while leaving `remaining` at zero. So delivering
//! a *redundant* point on every block — one that restates a parameter at the value it already
//! holds — forces the slow path on every block without moving a bit. That the redundant point is
//! itself bit-neutral is not assumed here: it is exactly what `stationary_hoist.rs` proves, and
//! that test is the one that fails if the assumption ever stops holding.
//!
//! The signal is deliberately tone-then-silence-then-tone. The leading tone drives the
//! integrators somewhere nonzero, the silence is long enough for them to settle and for the claim
//! to be earned, and the trailing tone is what catches a fast path that corrupted or froze the
//! state it skipped over: if the skipped blocks left the filter in the wrong place, the tone that
//! follows them renders differently.

mod support;

use miso_engine_effect_contract::{
    BankWidth, EffectBankProcessBlock, NativeEffectFactory, ParameterChannel,
    PrepareEffectBankRequest, PreparedAutomationSpan, StatePayloadOutput, StatePayloadSizes,
};
use miso_engine_lane::Backend;
use miso_engine_parametric_eq::ParametricEqFactory;
use support::{COMMON_BYTES, LANE_BYTES, point, request, set_initial, values};

const FRAMES: usize = 128;
/// Long enough that every integrator reaches its silent fixed point with room to spare, so the
/// steady state — not the decay — is what the fast path is exercised on.
const SILENT_BLOCKS: usize = 64;

fn band0_left_gain(track: usize) -> f32 {
    -9.0 + track as f32
}

fn configured(track: usize) -> Vec<miso_engine_effect_contract::InitialParameterValue> {
    let mut prepared = values();
    set_initial(&mut prepared, 0, ParameterChannel::Left, 1.0);
    set_initial(&mut prepared, 0, ParameterChannel::Right, 1.0);
    set_initial(&mut prepared, 1, ParameterChannel::Left, 3.0);
    set_initial(&mut prepared, 1, ParameterChannel::Right, 3.0);
    set_initial(
        &mut prepared,
        2,
        ParameterChannel::Left,
        400.0 + track as f32 * 37.0,
    );
    set_initial(
        &mut prepared,
        2,
        ParameterChannel::Right,
        1500.0 + track as f32 * 53.0,
    );
    set_initial(
        &mut prepared,
        3,
        ParameterChannel::Left,
        band0_left_gain(track),
    );
    set_initial(&mut prepared, 4, ParameterChannel::Left, 0.9);
    prepared
}

fn native_bank() -> Option<(BankWidth, Backend)> {
    let backend = Backend::current();
    BankWidth::for_backend(backend).map(|width| (width, backend))
}

/// One block of tone, or one block of exact `+0.0`.
fn plane(block: usize, lanes: usize, silent: bool, negate: bool) -> Vec<f32> {
    (0..FRAMES * lanes)
        .map(|index| {
            if silent {
                0.0
            } else {
                let value = (((block * FRAMES * lanes + index) as f32) * 0.017).sin() * 0.4;
                if negate { -value } else { value }
            }
        })
        .collect()
}

/// Renders tone, then silence, then tone, returning every output bit of every block.
///
/// `restate` delivers a redundant point on every block, which forces the slow path.
fn render(lanes: usize, restate: bool) -> Vec<u32> {
    let (width, backend) = native_bank().expect("a native bank width");
    let factory = ParametricEqFactory;
    let prepared: Vec<_> = (0..lanes).map(configured).collect();
    let requests: Vec<_> = prepared.iter().map(|v| request(v, false)).collect();
    let mut bank = factory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
        })
        .expect("valid bank request")
        .expect("the native width must bind");

    let redundant: Vec<PreparedAutomationSpan> = (0..lanes)
        .map(|track| point(3, ParameterChannel::Left, 0, band0_left_gain(track)))
        .collect();
    let full_offsets: Vec<u32> = (0..=lanes).map(|track| track as u32).collect();
    let empty_offsets = vec![0_u32; lanes + 1];

    let total = SILENT_BLOCKS + 2;
    let mut bits = Vec::new();
    for block in 0..total {
        let silent = block > 0 && block < total - 1;
        let mut left = plane(block, lanes, silent, false);
        let mut right = plane(block, lanes, silent, true);
        let (spans, offsets): (&[PreparedAutomationSpan], &[u32]) = if restate {
            (&redundant, &full_offsets)
        } else {
            (&[], &empty_offsets)
        };
        // A redundant point must be restated at *this* block's first sample to be admitted.
        let restated: Vec<PreparedAutomationSpan> = spans
            .iter()
            .map(|span| PreparedAutomationSpan {
                start_sample: (block * FRAMES) as u64,
                end_sample: (block * FRAMES) as u64,
                ..*span
            })
            .collect();
        bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                FRAMES as u32,
                width,
                (block * FRAMES) as u64,
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
/// Red mutation this holds against: engaging the fast path without the `block_is_positive_zero`
/// input test, which skips the trailing tone and renders it as silence.
///
/// The state-unchanged term is **not** covered by a red mutation here, and dropping it leaves this
/// file green. The reason is specific to this effect: an SVF's output only reaches exact `+0.0`
/// once its integrators have, so "the output was silent" and "the state had settled" arrive
/// together and the extra term never changes the verdict. It is kept because that coincidence is a
/// property of this kernel rather than of the rule, and it is exactly what fails for a compressor,
/// where the output is `+0.0` from the first silent sample while the detector is still releasing.
/// `miso-engine-compressor`'s `a_detector_still_releasing_through_the_silence_is_never_frozen` is
/// the test that holds that term down, and it is red without it.
#[test]
fn a_settled_silent_bank_renders_exactly_the_bank_that_never_fast_paths() {
    let Some((width, _)) = native_bank() else {
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

/// The silent stretch really is exactly `+0.0`, which is the claim the fast path relies on when it
/// leaves the buffer untouched. A tail that merely decayed toward zero would fail this.
#[test]
fn the_settled_stretch_is_exactly_positive_zero() {
    let Some((width, _)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let bits = render(lanes, false);
    let per_block = FRAMES * lanes * 2;
    // Skip the leading tone and the first silent blocks, which carry the decaying tail.
    let settled_from = per_block * 16;
    let settled_to = per_block * (SILENT_BLOCKS + 1);
    assert!(settled_to > settled_from);
    for (index, word) in bits[settled_from..settled_to].iter().enumerate() {
        assert_eq!(
            *word, 0,
            "word {index} of the settled stretch is not exactly +0.0"
        );
    }
}

/// The **input** side of the signed-zero rule (#163 phase 4, adversarial pass).
///
/// `block_is_positive_zero` reduces raw bit patterns, so a block of `-0.0` is *not* silence by its
/// test and the fast path declines it. That strictness had no pin: every release test stayed green
/// under a mutation that masked the sign bit (`bits |= value.to_bits() & 0x7fff_ffff`), which makes
/// a `-0.0` block count as silence and lets a claim earned on `+0.0` engage on it.
///
/// It is not merely conservative — it is load-bearing, and this is the measurement that settles it.
/// A settled bank fed one block of all `-0.0` **writes all `+0.0`**: the SVF's `x - ic2` at
/// `ic2 = +0.0` gives `-0.0`, but the output sum `m0*x + m1*v1 + m2*v2` mixes signed zeros and
/// `(-0.0) + (+0.0)` is `+0.0` under round-to-nearest. So the kernel changes the bits, while a
/// sign-blind fast path would skip and leave the buffer holding the `-0.0`s it was handed. Those
/// are different bit patterns for the same block, which is exactly the class-A promise being
/// broken.
///
/// Red under the sign-masked mutation; green on the strict predicate, which declines to engage and
/// therefore renders the `-0.0` block through the real kernel like any other non-silent input.
///
/// Both the rendered output and the per-track state payload are compared, so a divergence that
/// happened to leave the audio alone but moved an integrator would still be caught.
#[test]
fn a_negative_zero_input_block_is_not_treated_as_silence() {
    let Some((width, backend)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;

    fn run(
        lanes: usize,
        width: BankWidth,
        backend: miso_engine_lane::Backend,
        restate: bool,
    ) -> (Vec<u32>, Vec<[u8; LANE_BYTES]>) {
        let factory = ParametricEqFactory;
        let prepared: Vec<_> = (0..lanes).map(configured).collect();
        let requests: Vec<_> = prepared.iter().map(|v| request(v, false)).collect();
        let mut bank = factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &requests,
            })
            .expect("valid bank request")
            .expect("the native width must bind");
        let redundant: Vec<PreparedAutomationSpan> = (0..lanes)
            .map(|track| point(3, ParameterChannel::Left, 0, band0_left_gain(track)))
            .collect();
        let full_offsets: Vec<u32> = (0..=lanes).map(|t| t as u32).collect();
        let empty_offsets = vec![0_u32; lanes + 1];

        // 40 blocks of exact `+0.0` earn the claim, then exactly one block of all `-0.0`, then
        // four more `+0.0` blocks so a state divergence has somewhere to show up.
        const SETTLE: usize = 40;
        const TOTAL: usize = SETTLE + 5;
        let mut bits = Vec::new();
        for block in 0..TOTAL {
            let fill = if block == SETTLE { -0.0_f32 } else { 0.0_f32 };
            let mut left = vec![fill; FRAMES * lanes];
            let mut right = vec![fill; FRAMES * lanes];
            let first_sample = (block * FRAMES) as u64;
            let restated: Vec<PreparedAutomationSpan> = if restate {
                redundant
                    .iter()
                    .map(|span| PreparedAutomationSpan {
                        start_sample: first_sample,
                        end_sample: first_sample,
                        ..*span
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
                    width,
                    first_sample,
                    &restated,
                    offsets,
                    128,
                )
                .expect("bank block"),
            );
            bits.extend(left.iter().map(|v| v.to_bits()));
            bits.extend(right.iter().map(|v| v.to_bits()));
        }
        let state: Vec<[u8; LANE_BYTES]> = (0..lanes)
            .map(|track| {
                let mut common = [0_u8; COMMON_BYTES];
                let mut lane_left = [0_u8; LANE_BYTES];
                let mut lane_right = [0_u8; LANE_BYTES];
                bank.snapshot_track_state_payload(
                    track as u32,
                    StatePayloadOutput::new(
                        &mut common,
                        &mut lane_left,
                        &mut lane_right,
                        StatePayloadSizes {
                            common_bytes: COMMON_BYTES as u32,
                            left_bytes: LANE_BYTES as u32,
                            right_bytes: LANE_BYTES as u32,
                        },
                    )
                    .expect("state output"),
                )
                .expect("snapshot");
                lane_left
            })
            .collect();
        (bits, state)
    }

    let (fast_bits, fast_state) = run(lanes, width, backend, false);
    let (slow_bits, slow_state) = run(lanes, width, backend, true);

    // The measurement this pin rests on: the real kernel turns a `-0.0` block into `+0.0`.
    let per_block = FRAMES * lanes * 2;
    let negative_zero_block = &slow_bits[40 * per_block..41 * per_block];
    assert!(
        negative_zero_block.iter().all(|word| *word == 0),
        "the kernel is expected to write +0.0 for a -0.0 input block; if this ever stops being \
         true the reasoning in `block_is_positive_zero` needs re-deriving, not this assertion \
         relaxing"
    );
    assert_eq!(
        fast_bits, slow_bits,
        "a -0.0 input block was treated as silence and skipped, leaving -0.0 where the kernel \
         writes +0.0"
    );
    assert_eq!(
        fast_state, slow_state,
        "a -0.0 input block was skipped and the integrators diverged from the rendered arm"
    );
}
