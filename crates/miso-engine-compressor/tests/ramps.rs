//! E6 — D11: one division at the event, iterated additions, an exact snap on the last sample.
//!
//! The ramp state is visible through the payload: word `3 + 3i` is parameter `i`'s `current`,
//! `4 + 3i` its `target` and `5 + 3i` its `remaining`. `step` is deliberately **not** serialised —
//! the layout is a frozen contract fixture — which is what makes a mid-ramp restore class B
//! (`tests/payload.rs`).

mod support;

use miso_engine_effect_contract::{
    AutomationSpanKind, ParameterChannel, PreparedAutomationSpan, StatePayloadOutput,
};
use miso_engine_effect_runtime::state_payload::{read_f32, read_u32};

use support::{initial_values, prepare, render_scalar, request_with_quantum};

const THRESHOLD_DEFAULT: f32 = -18.0;
const THRESHOLD_TARGET: f32 = -80.0;
const SMOOTHING: u32 = 64;

fn point(parameter: u32, channel: ParameterChannel, value: f32) -> PreparedAutomationSpan {
    PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel,
        parameter_index: parameter,
        start_sample: 0,
        end_sample: 0,
        start_value: value,
        end_value: value,
    }
}

fn state(effect: &dyn miso_engine_effect_contract::PreparedNativeEffect) -> Vec<u8> {
    let sizes = effect.metadata().state_sizes;
    let mut left = vec![0_u8; sizes.left_bytes as usize];
    let mut right = vec![0_u8; sizes.right_bytes as usize];
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("payload"),
        )
        .expect("snapshot");
    left
}

/// A block-rate Point steps by a precomputed increment and lands exactly on its target.
///
/// The expected value is computed here with the same `f32` operations the ramp performs — one
/// division at the event, then iterated additions — so this is a bit equality. The pre-audit law,
/// `current += (target - current) / remaining` every sample, produces a *different* sequence, so
/// it is this test that pins which of the two the crate implements.
///
/// Red mutations (MUTATIONS.md rows 8 and 16), both proven: never run the ramping body
/// (`ramping = 0`), and ramp over 63 samples instead of the descriptor's 64.
#[test]
fn block_point_steps_by_a_precomputed_increment_and_snaps_exactly() {
    let step = (THRESHOLD_TARGET - THRESHOLD_DEFAULT) / SMOOTHING as f32;
    for updates in [1_usize, 2, 17, 63] {
        let values = initial_values();
        let mut effect = prepare(request_with_quantum(&values, 128));
        let span = point(0, ParameterChannel::Left, THRESHOLD_TARGET);
        let mut left = vec![0.0_f32; updates];
        let mut right = vec![0.0_f32; updates];
        render_scalar(
            effect.as_mut(),
            &mut left,
            &mut right,
            updates,
            128,
            &[(0, span)],
        );
        let mut expected = THRESHOLD_DEFAULT;
        for _ in 0..updates {
            expected += step;
        }
        let payload = state(effect.as_ref());
        assert_eq!(
            read_f32(&payload, 3).to_bits(),
            expected.to_bits(),
            "after {updates} updates"
        );
        assert_eq!(read_u32(&payload, 5), SMOOTHING - updates as u32);
    }

    // The 64th update assigns the target exactly, whatever the accumulated sum was.
    let values = initial_values();
    let mut effect = prepare(request_with_quantum(&values, 128));
    let span = point(0, ParameterChannel::Left, THRESHOLD_TARGET);
    let mut left = vec![0.0_f32; 64];
    let mut right = vec![0.0_f32; 64];
    render_scalar(
        effect.as_mut(),
        &mut left,
        &mut right,
        64,
        128,
        &[(0, span)],
    );
    let payload = state(effect.as_ref());
    assert_eq!(
        read_f32(&payload, 3).to_bits(),
        THRESHOLD_TARGET.to_bits(),
        "the last update is an assignment, not an addition"
    );
    assert_eq!(read_u32(&payload, 5), 0);

    // And it did not arrive early: 63 additions do not reach the target.
    let mut sum = THRESHOLD_DEFAULT;
    for _ in 0..63 {
        sum += step;
    }
    assert_ne!(sum.to_bits(), THRESHOLD_TARGET.to_bits());
}

/// A Point that arrives while a ramp is in flight restarts from the value reached, not from the
/// original one.
#[test]
fn a_restarting_point_ramps_from_the_current_value() {
    let values = initial_values();
    let mut effect = prepare(request_with_quantum(&values, 128));
    let mut left = vec![0.0_f32; 20];
    let mut right = vec![0.0_f32; 20];
    render_scalar(
        effect.as_mut(),
        &mut left,
        &mut right,
        20,
        128,
        &[(0, point(0, ParameterChannel::Left, THRESHOLD_TARGET))],
    );
    let mid = read_f32(&state(effect.as_ref()), 3);

    let mut left = vec![0.0_f32; 1];
    let mut right = vec![0.0_f32; 1];
    render_scalar(
        effect.as_mut(),
        &mut left,
        &mut right,
        1,
        128,
        &[(0, point(0, ParameterChannel::Left, -30.0))],
    );
    let payload = state(effect.as_ref());
    let restarted_step = (-30.0_f32 - mid) / SMOOTHING as f32;
    assert_eq!(
        read_f32(&payload, 3).to_bits(),
        (mid + restarted_step).to_bits()
    );
    assert_eq!(read_u32(&payload, 4), (-30.0_f32).to_bits());
    assert_eq!(read_u32(&payload, 5), SMOOTHING - 1);
}

/// Automation is per channel and per parameter, and an out-of-order or duplicate span is counted
/// and ignored rather than partly applied.
#[test]
fn automation_validation_is_unchanged() {
    let values = initial_values();
    let mut effect = prepare(request_with_quantum(&values, 128));
    let mut left = vec![0.0_f32; 1];
    let mut right = vec![0.0_f32; 1];

    // Out of order: parameter 2 before parameter 0.
    let spans = [
        point(2, ParameterChannel::Left, 12.0),
        point(0, ParameterChannel::Left, THRESHOLD_TARGET),
    ];
    let report = effect.process(
        miso_engine_effect_contract::EffectProcessBlock::new(
            &mut left, &mut right, None, 0, &spans, 128,
        )
        .expect("block"),
    );
    assert_eq!(report.invalid_spans, 1, "the out-of-order span is rejected");
    let payload = state(effect.as_ref());
    // Parameter 2 was applied, parameter 0 was not.
    assert_eq!(read_u32(&payload, 5 + 3 * 2), SMOOTHING - 1);
    assert_eq!(read_u32(&payload, 5), 0);

    // `Both` is not a channel this effect accepts.
    let mut effect = prepare(request_with_quantum(&values, 128));
    let report = effect.process(
        miso_engine_effect_contract::EffectProcessBlock::new(
            &mut left,
            &mut right,
            None,
            0,
            &[point(0, ParameterChannel::Both, THRESHOLD_TARGET)],
            128,
        )
        .expect("block"),
    );
    assert_eq!(report.invalid_spans, 1);
    assert_eq!(read_u32(&state(effect.as_ref()), 5), 0);

    // `lookahead` is not automatable: parameter index 7 is out of the ramped range.
    let mut effect = prepare(request_with_quantum(&values, 128));
    let report = effect.process(
        miso_engine_effect_contract::EffectProcessBlock::new(
            &mut left,
            &mut right,
            None,
            0,
            &[point(7, ParameterChannel::Left, 0.0)],
            128,
        )
        .expect("block"),
    );
    assert_eq!(report.invalid_spans, 1);
    assert_eq!(
        read_f32(&state(effect.as_ref()), 1).to_bits(),
        5.0_f32.to_bits()
    );
}

/// A finished ramp leaves exactly the coefficients a fresh preparation at that value would.
///
/// This is the "one design function" property: the ramping body and the preparation path share
/// `design::design_lane`, so a rendered instance whose ramp has completed and a freshly prepared
/// instance at the target value must render identical bits from then on.
///
/// Red mutation (MUTATIONS.md row 11): design the ballistic coefficients from an `f32`
/// `0.001 * ms * fs` product instead of the `f64` one.
#[test]
fn a_finished_ramp_equals_a_fresh_preparation() {
    let mut values = initial_values();
    for entry in values.iter_mut() {
        if entry.parameter_index == 7 {
            entry.value = 0.0;
        }
    }
    let mut ramped = prepare(request_with_quantum(&values, 128));
    let mut warm_left = vec![0.0_f32; 64];
    let mut warm_right = vec![0.0_f32; 64];
    render_scalar(
        ramped.as_mut(),
        &mut warm_left,
        &mut warm_right,
        64,
        128,
        &[
            (0, point(0, ParameterChannel::Left, -36.0)),
            (0, point(0, ParameterChannel::Right, -36.0)),
        ],
    );

    let mut target_values = values;
    target_values[0].value = -36.0;
    target_values[1].value = -36.0;
    let mut fresh = prepare(request_with_quantum(&target_values, 128));
    let mut discard_left = vec![0.0_f32; 64];
    let mut discard_right = vec![0.0_f32; 64];
    render_scalar(
        fresh.as_mut(),
        &mut discard_left,
        &mut discard_right,
        64,
        128,
        &[],
    );

    let signal = support::noise(2_048, 0x5A_11_9E_01, 0.8);
    let mut ramped_left = signal.clone();
    let mut ramped_right = signal.clone();
    let mut fresh_left = signal.clone();
    let mut fresh_right = signal.clone();
    render_scalar(
        ramped.as_mut(),
        &mut ramped_left,
        &mut ramped_right,
        128,
        128,
        &[],
    );
    render_scalar(
        fresh.as_mut(),
        &mut fresh_left,
        &mut fresh_right,
        128,
        128,
        &[],
    );
    assert_eq!(
        ramped_left.iter().map(|s| s.to_bits()).collect::<Vec<_>>(),
        fresh_left.iter().map(|s| s.to_bits()).collect::<Vec<_>>(),
        "a finished ramp must leave the coefficients a fresh preparation has"
    );
}

// -------------------------------------------------------------------------------------------
// The idle-lane guard: what a lane that is *not* ramping pays for a neighbour that is.
// -------------------------------------------------------------------------------------------

const BANK_FRAMES: usize = 128;
/// Long enough that the comparison has audio to compare.
///
/// Measured rather than assumed, and the same trap `tests/silent_fixed_point.rs` records: the
/// prepared latency in front of the output is around nine hundred samples, so a run of six blocks
/// renders exact `+0.0` throughout and every arm of every comparison agrees on nothing at all.
const BANK_BLOCKS: usize = 20;
/// The block the moving Point is delivered on.
///
/// Past the delay line, so the ramp's effect is inside the rendered output rather than still in
/// flight when the run ends, and past every lane's transient.
const RAMP_BLOCK: usize = 12;
/// The lane the Point addresses. Not lane 0, so a kernel that scattered to lane 0 would be caught.
const RAMP_LANE: usize = 1;
/// Where the moving Point sends the threshold. Far from every lane's held value.
const RAMP_TARGET_DB: f32 = -40.0;

/// Threshold this track is prepared at. Distinct per lane, so no lane can borrow another's word.
fn bank_threshold(track: usize) -> f32 {
    -12.0 - 3.0 * track as f32
}

/// Per-track parameters: no two lanes agree on anything the kernel reads per lane.
///
/// Lane 2 is prepared with a makeup of exactly `+0.0` and every lane with a mix strictly between
/// `0` and `1`, so the step-8 identity masks are genuinely mixed across the bank rather than
/// uniformly true or uniformly false.
fn bank_track_values(track: usize) -> [miso_engine_effect_contract::InitialParameterValue; 16] {
    support::values_with(&[
        (0, bank_threshold(track)),
        (1, 2.0 + track as f32),
        (2, 2.0 * (track % 3) as f32),
        (3, 1.0 + track as f32),
        (4, 30.0 + 20.0 * track as f32),
        (5, -1.0 + 0.5 * track as f32),
        (6, 0.4 + 0.05 * (track % 4) as f32),
        (7, 1.0 * (track % 3) as f32),
    ])
}

/// Renders a bank for [`BANK_BLOCKS`] blocks and returns each lane's output bits separately.
///
/// `points` are `(block, lane, value)` left-channel threshold Points, the common case.
fn bank_lane_bits(points: &[(usize, usize, f32)]) -> Vec<Vec<u32>> {
    let full: Vec<_> = points
        .iter()
        .map(|(block, lane, value)| (*block, *lane, 0_u32, *value))
        .collect();
    bank_lane_bits_with(&full)
}

/// As [`bank_lane_bits`], with the automated parameter named per point.
fn bank_lane_bits_with(points: &[(usize, usize, u32, f32)]) -> Vec<Vec<u32>> {
    let (_, width) = support::native_bank_width().expect("a native bank width");
    let lanes = width.lanes() as usize;
    let values: Vec<_> = (0..lanes).map(bank_track_values).collect();
    bank_lane_bits_from(&values, points)
}

/// The core: `values` prepares the bank, `points` are `(block, lane, parameter, value)` Points.
///
/// Every lane is fed the same input, so any difference between two lanes' bits is a difference in
/// what the kernel did with that lane's parameters and never a difference in what it was asked to
/// compress. The Points ride the left channel, so the right channel of an automated track is
/// itself an idle channel dragged through the ramping body -- the asymmetry the guard exists for.
fn bank_lane_bits_from(
    values: &[[miso_engine_effect_contract::InitialParameterValue; 16]],
    points: &[(usize, usize, u32, f32)],
) -> Vec<Vec<u32>> {
    let (_, width) = support::native_bank_width().expect("a native bank width");
    let lanes = width.lanes() as usize;
    assert_eq!(values.len(), lanes);
    let requests: Vec<_> = values
        .iter()
        .map(|value| request_with_quantum(value, BANK_FRAMES as u32))
        .collect();
    let mut bank = support::bind_bank(&requests).expect("bank must bind at this build's width");

    let mut out = vec![Vec::new(); lanes];
    for block in 0..BANK_BLOCKS {
        let first_sample = (block * BANK_FRAMES) as u64;
        let mut left: Vec<f32> = (0..BANK_FRAMES * lanes)
            .map(|index| ((((block * BANK_FRAMES) + index / lanes) as f32) * 0.031).sin() * 0.5)
            .collect();
        let mut right: Vec<f32> = left.iter().map(|value| -value * 0.75).collect();

        let mut flat = Vec::new();
        let mut offsets = vec![0_u32; lanes + 1];
        // Ascending parameter index within a track, because the contract's canonical span order is
        // `(start_sample, parameter_index, channel)` and a block that presents two spans the other
        // way round has its second one counted as invalid and dropped rather than applied. Sorted
        // here rather than demanded of every caller: a silently dropped span is a test that quietly
        // stops testing what it says it does, which is exactly what it cost to find this once.
        let mut ordered: Vec<_> = points.to_vec();
        ordered.sort_by_key(|(_, _, parameter, _)| *parameter);
        for track in 0..lanes {
            for (at, lane, parameter, value) in &ordered {
                if *lane == track && *at == block {
                    flat.push(PreparedAutomationSpan {
                        kind: AutomationSpanKind::Point,
                        channel: ParameterChannel::Left,
                        parameter_index: *parameter,
                        start_sample: first_sample,
                        end_sample: first_sample,
                        start_value: *value,
                        end_value: *value,
                    });
                }
            }
            offsets[track + 1] = flat.len() as u32;
        }
        bank.process_bank(
            miso_engine_effect_contract::EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                BANK_FRAMES as u32,
                width,
                first_sample,
                &flat,
                &offsets,
                BANK_FRAMES as u32,
            )
            .expect("bank block"),
        );
        for (lane, bits) in out.iter_mut().enumerate() {
            for frame in 0..BANK_FRAMES {
                bits.push(left[frame * lanes + lane].to_bits());
                bits.push(right[frame * lanes + lane].to_bits());
            }
        }
    }
    out
}

/// A lane with no ramp in flight renders the same bits whether or not a neighbour is ramping.
///
/// This is the property the idle-lane guard in `Channel::advance_ramps` has to preserve, and it is
/// worth being precise about why it is not obvious. `process_block` cuts its ramping prefix from
/// the longest ramp anywhere in *either* channel of the bank, so one automated lane drags every
/// other lane of that bank — and both channels of the automated track itself — through the ramping
/// body for the length of the window. Those lanes have nothing to advance and nothing to redesign;
/// the guard is the statement that they therefore need not be walked at all.
///
/// Both directions are asserted, because each catches a different way of being wrong:
///
/// * the ramping lane must render the same bits whether it ramps alone or the whole bank ramps
///   with it — a kernel that let one lane's window change another lane's arithmetic fails here;
/// * every *other* lane must be bit-identical to a bank in which nothing ramped at all, even
///   though those lanes did spend the window inside the ramping body.
#[test]
fn an_idle_lane_is_untouched_by_a_neighbours_ramp() {
    let Some((_, width)) = support::native_bank_width() else {
        println!("scalar-only build: there is no neighbour lane to be dragged");
        return;
    };
    let lanes = width.lanes() as usize;
    let quiet = bank_lane_bits(&[]);
    let one = bank_lane_bits(&[(RAMP_BLOCK, RAMP_LANE, RAMP_TARGET_DB)]);
    let every: Vec<_> = (0..lanes)
        .map(|lane| (RAMP_BLOCK, lane, RAMP_TARGET_DB))
        .collect();
    let every = bank_lane_bits(&every);

    assert_eq!(
        one[RAMP_LANE], every[RAMP_LANE],
        "the ramping lane rendered differently depending on what its neighbours were doing"
    );
    assert!(
        quiet.iter().any(|lane| lane.iter().any(|bits| *bits != 0)),
        "the run is entirely inside the delay line: this comparison saw only silence"
    );
    assert_ne!(
        one[RAMP_LANE], quiet[RAMP_LANE],
        "the moving Point changed nothing: this test is not exercising a ramp at all"
    );
    for lane in 0..lanes {
        if lane == RAMP_LANE {
            continue;
        }
        assert_eq!(
            one[lane], quiet[lane],
            "lane {lane} was dragged through the ramping body and came out different"
        );
        assert_ne!(
            every[lane], quiet[lane],
            "lane {lane} ignored a Point addressed to it"
        );
    }
}

/// A ramp that finishes inside a block is not observable at the block boundary it finishes on.
///
/// The window is sixty-four samples and the quantum is a hundred and twenty-eight, so one block
/// contains the whole ramping prefix *and* the idle remainder, with the finish exactly halfway.
/// Rendering the same input as two sixty-four-frame blocks puts that finish on a block boundary
/// instead, and the second run's later blocks have no ramp in flight at all.
///
/// This is the finished-ramp identity stated where it can be observed: `LinearRamp::next_value`
/// on a ramp with `remaining == 0` returns `current` and mutates nothing, so a body that calls it
/// and a body that skips it must agree — and if they did not, splitting the block exactly on the
/// sample the ramp completes is where the two would part company.
#[test]
fn a_ramp_that_finishes_mid_block_is_invisible_to_the_partition() {
    let values = initial_values();
    let signal = support::noise(256, 0x51_9E_04_11, 0.8);
    let span = point(0, ParameterChannel::Left, THRESHOLD_TARGET);

    let mut whole = prepare(request_with_quantum(&values, 128));
    let mut whole_left = signal.clone();
    let mut whole_right = signal.clone();
    render_scalar(
        whole.as_mut(),
        &mut whole_left,
        &mut whole_right,
        128,
        128,
        &[(0, span)],
    );

    let mut split = prepare(request_with_quantum(&values, 128));
    let mut split_left = signal.clone();
    let mut split_right = signal.clone();
    render_scalar(
        split.as_mut(),
        &mut split_left,
        &mut split_right,
        SMOOTHING as usize,
        128,
        &[(0, span)],
    );

    assert_eq!(
        whole_left.iter().map(|s| s.to_bits()).collect::<Vec<_>>(),
        split_left.iter().map(|s| s.to_bits()).collect::<Vec<_>>(),
        "the sample the ramp finishes on became observable as a block boundary"
    );
    assert_eq!(
        whole_right.iter().map(|s| s.to_bits()).collect::<Vec<_>>(),
        split_right.iter().map(|s| s.to_bits()).collect::<Vec<_>>(),
    );
}

/// A Point that restates the value in force arms no ramp, so no lane enters the ramping body.
///
/// The #144 item 6 stationary hoist decides the case by bit compare and settles the ramp instead
/// of arming it. The interaction that matters here is the one the hoist's own documentation names:
/// because the block's ramping decision is taken across *all* lanes, an unhoisted no-op ramp on one
/// lane would drag the whole bank onto the ramping path for sixty-four samples. With the hoist, a
/// restated Point must leave every lane of the bank bit-identical to a bank that received no span
/// at all — the automated lane included, which is what separates this from the idle-lane guard.
#[test]
fn a_restated_point_leaves_every_lane_on_the_idle_body() {
    let Some((_, width)) = support::native_bank_width() else {
        println!("scalar-only build: the bank-wide ramping decision has one lane to take");
        return;
    };
    let lanes = width.lanes() as usize;
    let quiet = bank_lane_bits(&[]);
    let restated = bank_lane_bits(&[(RAMP_BLOCK, RAMP_LANE, bank_threshold(RAMP_LANE))]);
    for lane in 0..lanes {
        assert_eq!(
            quiet[lane], restated[lane],
            "lane {lane} moved when lane {RAMP_LANE} was restated at the value it already held"
        );
    }
}

// -------------------------------------------------------------------------------------------
// The interaction with the phase-4 silent fixed point.
// -------------------------------------------------------------------------------------------

/// Above the tone's level, so the detector rests at an exact `0.0` dB reduction and the silent
/// fast path is reachable at all. `tests/silent_fixed_point.rs` records why that is required.
const SILENT_THRESHOLD_DB: f32 = -6.0;
/// Tone amplitude, ~-26 dBFS: comfortably under [`SILENT_THRESHOLD_DB`].
const SILENT_TONE: f32 = 0.05;
const SILENT_RELEASE_MS: f32 = 50.0;
const SILENT_BLOCKS: usize = 160;
/// Well inside the settled silence, so the claim is certainly held when the Point arrives.
const SILENT_AUTOMATED_BLOCK: usize = 120;
/// Enough trailing tone for the tone to clear the lookahead line and be compared.
const SILENT_TRAILING_BLOCKS: usize = 12;

/// Renders tone, a long silence carrying one Point on one lane, then tone again.
///
/// `suppress` restates the threshold on **every** block, which `process_bank` treats as grounds to
/// withdraw the silent claim, so that arm never takes the fast path anywhere. Both arms receive the
/// same moving Point on the same block on the same lane.
fn silent_ramp_bits(suppress: bool) -> Vec<u32> {
    let (_, width) = support::native_bank_width().expect("a native bank width");
    let lanes = width.lanes() as usize;
    let values =
        support::values_with(&[(0, SILENT_THRESHOLD_DB), (1, 8.0), (4, SILENT_RELEASE_MS)]);
    let requests: Vec<_> = (0..lanes)
        .map(|_| request_with_quantum(&values, BANK_FRAMES as u32))
        .collect();
    let mut bank = support::bind_bank(&requests).expect("a native bank");

    let total = 1 + SILENT_BLOCKS + SILENT_TRAILING_BLOCKS;
    let mut bits = Vec::new();
    for block in 0..total {
        let silent = block > 0 && block <= SILENT_BLOCKS;
        let first_sample = (block * BANK_FRAMES) as u64;
        let mut left: Vec<f32> = (0..BANK_FRAMES * lanes)
            .map(|index| {
                if silent {
                    0.0
                } else {
                    ((((block * BANK_FRAMES) + index / lanes) as f32) * 0.031).sin() * SILENT_TONE
                }
            })
            .collect();
        let mut right: Vec<f32> = left.iter().map(|value| -value).collect();

        let mut flat = Vec::new();
        let mut offsets = vec![0_u32; lanes + 1];
        for track in 0..lanes {
            if suppress {
                flat.push(PreparedAutomationSpan {
                    kind: AutomationSpanKind::Point,
                    channel: ParameterChannel::Left,
                    parameter_index: 0,
                    start_sample: first_sample,
                    end_sample: first_sample,
                    start_value: SILENT_THRESHOLD_DB,
                    end_value: SILENT_THRESHOLD_DB,
                });
            }
            if track == RAMP_LANE && block == SILENT_AUTOMATED_BLOCK {
                flat.push(PreparedAutomationSpan {
                    kind: AutomationSpanKind::Point,
                    channel: ParameterChannel::Right,
                    parameter_index: 0,
                    start_sample: first_sample,
                    end_sample: first_sample,
                    start_value: RAMP_TARGET_DB,
                    end_value: RAMP_TARGET_DB,
                });
            }
            offsets[track + 1] = flat.len() as u32;
        }
        bank.process_bank(
            miso_engine_effect_contract::EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                BANK_FRAMES as u32,
                width,
                first_sample,
                &flat,
                &offsets,
                BANK_FRAMES as u32,
            )
            .expect("bank block"),
        );
        bits.extend(left.iter().map(|value| value.to_bits()));
        bits.extend(right.iter().map(|value| value.to_bits()));
    }
    bits
}

/// A ramp arriving inside the silent fixed point renders exactly what a bank that never claimed it
/// renders.
///
/// Two phase-4 mechanisms meet here. The bank is sitting on the settled-silent claim, skipping its
/// blocks outright; a Point then arrives on one lane, which withdraws the claim for that block, and
/// the withdrawal drops the bank onto the ramping body with exactly one lane in flight and every
/// other lane idle. That is the idle-lane guard executing inside the withdrawal path, which is
/// precisely the composition neither test file covered on its own.
///
/// The control arm restates the threshold on every block, which suppresses the fast path
/// everywhere while arming no ramp — the same suppression `tests/silent_fixed_point.rs` uses. If
/// the two arms ever disagree, either the withdrawal or the guard is wrong.
#[test]
fn a_ramp_inside_the_silent_fixed_point_withdraws_it_and_moves_no_bit() {
    if support::native_bank_width().is_none() {
        println!("scalar-only build: `process_bank` is not reachable here");
        return;
    }
    let fast = silent_ramp_bits(false);
    let never = silent_ramp_bits(true);
    assert_eq!(
        fast.len(),
        never.len(),
        "the two arms rendered different amounts of audio"
    );
    let first = fast
        .iter()
        .zip(never.iter())
        .position(|(left, right)| left != right);
    assert!(
        first.is_none(),
        "the silent fast path and the never-fast-path arm diverged at word {:?}",
        first
    );
    assert!(
        fast.iter().any(|bits| *bits != 0),
        "the trailing tone was never rendered: this comparison saw only silence"
    );
}

/// A ramp that lands exactly on a step-8 identity boundary leaves the lane rendering the identity.
///
/// The three masks step 8 selects on — `mix == 1`, `mix == 0` and `makeup == 0` — are functions of
/// coefficient words alone, so they are built once per `Coef` load rather than once per frame. That
/// is only sound because the idle body loads `Coef` once and never redesigns, while the ramping
/// body reloads it on every frame after `advance_ramps`. A ramp that *crosses* one of those
/// boundaries is where the distinction is observable: the D11 snap assigns the target exactly on
/// the final sample, so `mix` becomes bit-exactly `1.0` on that sample and the wet identity has to
/// be true from that sample on, not from the next `Coef` load.
///
/// The assertion is the one-design-function property at the boundary: once the window has closed,
/// the ramped lane must render exactly what a lane prepared at the target renders, on every
/// subsequent block. A gain of exactly zero dB converts to exactly unity on the tier this kernel
/// uses -- the property `one_frame` documents where it applies the makeup -- so a makeup that has
/// ramped to `+0.0` really is an identity and not merely a very small gain.
#[test]
fn a_ramp_onto_an_identity_boundary_lands_on_the_identity() {
    let Some((_, width)) = support::native_bank_width() else {
        println!("scalar-only build: `process_bank` is not reachable here");
        return;
    };
    let lanes = width.lanes() as usize;

    // Lane `RAMP_LANE` is prepared with a mix of 0.4-something and a nonzero makeup, and is ramped
    // onto both identities at once: mix to exactly 1.0 and makeup to exactly +0.0.
    let ramped = bank_lane_bits_with(&[
        (RAMP_BLOCK, RAMP_LANE, 6, 1.0),
        (RAMP_BLOCK, RAMP_LANE, 5, 0.0),
    ]);
    assert_ne!(
        ramped[RAMP_LANE],
        bank_lane_bits(&[])[RAMP_LANE],
        "the identity ramp changed nothing: the lane was already on both identities"
    );

    // The same bank, prepared at the target values for that lane and never automated.
    let fresh = {
        let values: Vec<_> = (0..lanes)
            .map(|track| {
                let mut value = bank_track_values(track);
                if track == RAMP_LANE {
                    // Parameter 5 is makeup, parameter 6 is mix, and `[p * 2]` is that
                    // parameter's *left* channel. Only the left channel is set, because only the
                    // left channel is automated: the right channel of the automated track keeps
                    // its prepared mix and makeup in both arms, and is the idle channel this
                    // comparison also depends on being left alone.
                    value[5 * 2].value = 0.0;
                    value[6 * 2].value = 1.0;
                }
                value
            })
            .collect();
        bank_lane_bits_from(&values, &[])
    };

    // Only the blocks after the window has closed: during the window the two are legitimately
    // different, because one of them is ramping and the other has always been at the target.
    let per_block = ramped[RAMP_LANE].len() / BANK_BLOCKS;
    let settled = (RAMP_BLOCK + 1) * per_block;
    assert_eq!(
        &ramped[RAMP_LANE][settled..],
        &fresh[RAMP_LANE][settled..],
        "a ramp that landed on the wet/makeup identities does not render the identity"
    );
}
