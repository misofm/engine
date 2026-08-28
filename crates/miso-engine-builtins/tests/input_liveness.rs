//! Issue #210 phase 3: the live input trim and polarity.
//!
//! Five properties matter and all five are gated here.
//!
//! 1. **Class-A OFF.** A bank no command has ever addressed renders exactly the bits it rendered
//!    before the feature existed -- the same call, on the same prepared coefficient words, in the
//!    same order. The settled state is initialised *from* `InputLane::trim_signed`, so "the same
//!    words" is a bit comparison and not a re-derivation.
//! 2. **The ramp obeys the smoother law.** The trim coefficient's per-sample sequence is
//!    bit-identical to `miso_engine_effect_contract::ParameterSmoother` under
//!    `SmoothingRule::Linear` -- the same D11 form the fader and the matrix already obey, and the
//!    one the metadata's `linearNUpdates` policy declares.
//! 3. **The ramping body is the settled body when the coefficient does not move.** This is the
//!    load-bearing half of "the ramping path ignores the elision plan": a retarget to the value a
//!    lane already holds makes every frame's trim the settled trim, so a block rendered through
//!    `input_chain_ramp_block` must be bit-identical to one rendered through
//!    `input_chain_block_elided` -- **including on a bank whose sections are all elided**, where
//!    the two paths run structurally different code.
//! 4. **A polarity flip is the trim ramp passing through zero.** It crosses zero monotonically,
//!    it settles bit-identical to a session re-prepared with the inverted polarity, and it leaves
//!    the trim magnitude alone.
//! 5. **The elision plan does not go stale.** A ramping block runs the *unelided* body over an
//!    elidable bank's identity sections, and those sections' integrators must come back exactly
//!    `+0.0` -- which is what makes the plan Job 1 decided still true afterwards.
//!
//! Everything here goes through the shipped surface: `BuiltinInputBank` and `InputBuiltins`.

use miso_engine_builtins::test_support::{
    bank_elision_plan, bank_lane_state_words, bank_trim_ramp_words, input_elision_plan,
    input_state_words, input_trim_ramp_words, input_trim_words,
};
use miso_engine_builtins::*;
use miso_engine_effect_contract::{BankWidth, ParameterSmoother, SmoothingRule};
use miso_engine_lane::Backend;

const BANKS: [(Backend, BankWidth); 2] = [
    (Backend::Simd4, BankWidth::Four),
    (Backend::Simd8, BankWidth::Eight),
];

fn channel(trim_db: f32, polarity_invert: bool, hpf_hz: f32, lpf_hz: f32) -> ChannelParameters {
    ChannelParameters {
        polarity_invert,
        trim_db,
        hpf_hz,
        lpf_hz,
        ..ChannelParameters::default()
    }
}

fn parameters(left: ChannelParameters, right: ChannelParameters) -> BuiltinParameters {
    BuiltinParameters {
        left,
        right,
        matrix: Matrix2x2::IDENTITY,
        smoothing_samples: 0,
    }
}

fn input(parameters: BuiltinParameters) -> InputBuiltins {
    BuiltinChain::new(48_000, parameters)
        .expect("accepted input builtins")
        .into_input_builtins()
}

/// A block with a `-0.0`, a denormal-adjacent value and a sign flip in every lane: the shapes that
/// make an "equivalent" arithmetic path stop being bit-identical.
fn probe_block(frames: usize) -> Vec<f32> {
    const PATTERN: [f32; 8] = [
        1.0,
        -0.0,
        0.5,
        -0.5,
        0.0,
        -1.0,
        1.192_092_9e-7,
        -3.402_823_5e38,
    ];
    (0..frames).map(|index| PATTERN[index % 8]).collect()
}

fn bits(values: &[f32]) -> Vec<u32> {
    values.iter().map(|value| value.to_bits()).collect()
}

// ---------------------------------------------------------------------------------------------
// 1. Class-A OFF.
// ---------------------------------------------------------------------------------------------

/// Red mutation: initialise `InputStage::ramp.current` from `L::splat(1.0)` instead of `coef.trim`
/// -> every uncommanded lane renders at unit trim and this comparison fails on the first block.
///
/// Red mutation: make `InputStage::process` take the ramping arm unconditionally (drop the
/// `self.ramping` test) -> an all-elided bank runs the unelided body, which is bit-identical, but
/// a bank with a real filter runs it too and the *state* comparison below still holds. The
/// discriminating case is `settled_and_ramping_paths_agree_on_an_elided_bank`, which forces the
/// question on the bank where the two paths differ structurally.
#[test]
fn an_uncommanded_live_input_renders_the_prepared_coefficients() {
    for (left_db, right_db, left_invert, right_invert, hpf, lpf) in [
        (0.0_f32, 0.0_f32, false, false, 0.0_f32, 0.0_f32),
        (-6.0, 3.0, false, false, 80.0, 12_000.0),
        (0.0, 0.0, true, true, 0.0, 0.0),
        (-144.0, 24.0, true, false, 20.0, 20_000.0),
    ] {
        let params = parameters(
            channel(left_db, left_invert, hpf, lpf),
            channel(right_db, right_invert, hpf, lpf),
        );
        let mut live = input(params);
        // The settled words are the prepared words, bit for bit. This is the whole of the class-A
        // OFF claim's premise: the ramp is initialised *from* `trim_signed`, not re-derived.
        assert_eq!(
            [live.trim_signed(0).to_bits(), live.trim_signed(1).to_bits()],
            input_trim_words(&live),
            "the settled ramp words are the prepared trim words"
        );
        assert_eq!(
            [live.trim_target(0).to_bits(), live.trim_target(1).to_bits()],
            input_trim_words(&live),
            "the settled target words are the prepared trim words"
        );

        // And the rendered block is the one the prepared chain renders, over the whole probe.
        let mut reference = input(params);
        let mut left = probe_block(64);
        let mut right: Vec<f32> = probe_block(64).iter().map(|value| -*value).collect();
        let mut reference_left = left.clone();
        let mut reference_right = right.clone();
        reference.process(
            DualMonoBlock::new(&mut reference_left, &mut reference_right, 0).expect("block"),
        );
        live.process(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
        assert_eq!(bits(&reference_left), bits(&left), "left plane");
        assert_eq!(bits(&reference_right), bits(&right), "right plane");
        assert_eq!(
            input_state_words(&reference),
            input_state_words(&live),
            "retained integrators"
        );
    }
}

/// The settled arm writes **no** ramp word.
///
/// This is the one observable the class-A OFF dispatch has. The two arms are bit-identical in the
/// plane -- that is the point, and `settled_and_ramping_paths_agree_on_an_elided_bank` proves it --
/// so no digest can tell them apart and the ramp's own state is what says which one ran. The
/// countdown words are written only by `input_chain_ramp_block`, so a settled block that leaves
/// them at exactly `+0.0` is a settled block that took the settled arm.
///
/// Red mutation: make `InputStage::process` take the ramping arm unconditionally (`if true` in
/// place of `if self.ramping`) -> the countdown words come back as `-frames` and this fails, while
/// every digest in the tree stays green. That asymmetry is why this test exists.
#[test]
fn the_settled_arm_leaves_the_ramp_words_untouched() {
    for (hpf, lpf) in [(0.0_f32, 0.0_f32), (100.0, 9_000.0)] {
        let params = parameters(channel(-7.5, true, hpf, lpf), channel(2.0, false, hpf, lpf));
        let mut live = input(params);
        let settled = input_trim_ramp_words(&live);
        let prepared = input_trim_words(&live);
        assert_eq!(
            settled,
            [
                prepared[0],
                prepared[1],
                prepared[0],
                prepared[1],
                0,
                0,
                0,
                0
            ],
            "a prepared chain starts settled at its prepared coefficients, step and countdown zero"
        );
        for block in 0..4 {
            let mut left = probe_block(32);
            let mut right: Vec<f32> = probe_block(32).iter().map(|value| -*value).collect();
            live.process(DualMonoBlock::new(&mut left, &mut right, block * 32).expect("block"));
            assert_eq!(
                input_trim_ramp_words(&live),
                settled,
                "block {block}: a settled render writes no ramp word"
            );
        }

        // And the mirror: a commanded lane *does* write them, so the assertion above is not
        // vacuous.
        live.set_trim_db(BuiltinLaneSelector::Left, -20.0, 16)
            .expect("trim domain");
        let mut left = probe_block(8);
        let mut right: Vec<f32> = probe_block(8).iter().map(|value| -*value).collect();
        live.process(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
        assert_ne!(
            input_trim_ramp_words(&live),
            settled,
            "a ramping render writes the ramp words"
        );
    }
}

/// The elision plan an uncommanded bank carries is the plan Job 1 decides, at every width.
#[test]
fn an_uncommanded_bank_keeps_its_elision_plan() {
    for (backend, width) in BANKS {
        // Every section disabled: the all-elided plan.
        let disabled = parameters(
            channel(-3.0, false, 0.0, 0.0),
            channel(-3.0, false, 0.0, 0.0),
        );
        let inputs: Vec<InputBuiltins> = (0..width.lanes() as usize)
            .map(|_| input(disabled))
            .collect();
        let mut bank = BuiltinInputBank::new(backend, width, inputs).expect("bank");
        assert_eq!(
            bank_elision_plan(&bank),
            [[true; 2]; 2],
            "an all-disabled bank elides every section at {width:?}"
        );
        // The banked half of `the_settled_arm_leaves_the_ramp_words_untouched`: a bank no console
        // addressed renders through the settled arm on every lane, including its padding lanes.
        let lanes = width.lanes() as usize;
        let before: Vec<[u32; 8]> = (0..lanes)
            .map(|lane| bank_trim_ramp_words(&bank, lane))
            .collect();
        let mut left = probe_block(32 * lanes);
        let mut right: Vec<f32> = left.iter().map(|value| -*value).collect();
        bank.process(&mut left, &mut right, 32);
        for (lane, expected) in before.iter().enumerate() {
            assert_eq!(
                &bank_trim_ramp_words(&bank, lane),
                expected,
                "lane {lane} at {width:?}: an uncommanded bank writes no ramp word"
            );
        }
    }
}

// ---------------------------------------------------------------------------------------------
// 2. The smoother law.
// ---------------------------------------------------------------------------------------------

/// The trim coefficient's per-sample sequence, read out of the **rendered plane** of one block.
///
/// Every section is disabled, so the chain is `sanitise, multiply by trim, add(+0.0), add(+0.0),
/// boundary scan` and a frame of exactly `1.0` comes out as exactly that frame's trim word: `+0.0`
/// added to a finite non-`-0.0` value is that value. So the plane *is* the coefficient sequence,
/// with no readback in the way.
///
/// It is deliberately **one** block rather than a block per frame. A per-frame partition would put
/// `InputStage::settle` between every pair of updates, and `settle` restates the D11 snap -- so a
/// kernel whose countdown was off by one would be repaired between frames and the oracle would
/// measure `settle` rather than the kernel. One block is the kernel alone.
fn observed_trim_sequence(from_db: f32, to_db: f32, samples: u32, frames: usize) -> Vec<u32> {
    let mut live = input(parameters(
        channel(from_db, false, 0.0, 0.0),
        channel(from_db, false, 0.0, 0.0),
    ));
    live.set_trim_db(BuiltinLaneSelector::Both, to_db, samples)
        .expect("trim domain");
    let mut left = vec![1.0_f32; frames];
    let mut right = vec![1.0_f32; frames];
    live.process(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
    assert_eq!(
        bits(&left),
        bits(&right),
        "a `Both` command moves both lanes"
    );
    bits(&left)
}

/// Red mutation: change `input_chain_ramp_block`'s step 3 to `current + step` without the
/// `select(done, target, ...)` -> the final update accumulates instead of assigning the exact
/// target, and the last word of the sequence differs from the oracle's by one ulp.
///
/// Red mutation: move `remaining = remaining - 1` after the `done` compare -> the ramp runs one
/// frame long and the whole tail shifts.
#[test]
fn the_trim_ramp_is_bit_identical_to_the_parameter_smoother() {
    for (from_db, to_db) in [
        (0.0_f32, -12.0_f32),
        (-12.0, 0.0),
        (-144.0, 24.0),
        (24.0, -144.0),
        (-6.0, -6.0),
        (0.0, -0.5),
    ] {
        for samples in [1_u32, 2, 3, 7, 64, 128] {
            let frames = samples as usize + 4;
            let observed = observed_trim_sequence(from_db, to_db, samples, frames);

            // The oracle. `ParameterSmoother` is the contract's own statement of the linear-N law
            // and is proven bit-identical to `LinearRamp`
            // (`effect-runtime/tests/contract_ramp_identity.rs`), so this is the law and not a
            // second implementation of the kernel.
            //
            // The endpoints are the *coefficients*, because that is what ramps: `trim_db` is a
            // decibel control and the chain multiplies by a gain, so the smoother is seeded and
            // aimed with the same `db_gain` conversion preparation uses.
            let from_gain = f32::from_bits(
                input_trim_words(&input(parameters(
                    channel(from_db, false, 0.0, 0.0),
                    channel(from_db, false, 0.0, 0.0),
                )))[0],
            );
            let to_gain = f32::from_bits(
                input_trim_words(&input(parameters(
                    channel(to_db, false, 0.0, 0.0),
                    channel(to_db, false, 0.0, 0.0),
                )))[0],
            );
            let mut smoother = ParameterSmoother::new(from_gain, SmoothingRule::Linear, samples)
                .expect("a nonzero linear window");
            assert!(smoother.set_target(to_gain), "finite target");
            let expected: Vec<u32> = (0..frames)
                .map(|_| smoother.next_value().to_bits())
                .collect();
            assert_eq!(
                observed, expected,
                "trim ramp {from_db} -> {to_db} dB over {samples} updates"
            );
        }
    }
}

/// The ramp is partition-invariant: a lane's coefficient sequence does not depend on where the
/// block boundaries fall.
///
/// Red mutation: carry the kernel's `f32` countdown across blocks instead of recomputing it from
/// the authoritative `u32` at the top of every ramping block -> a ramp split across two blocks
/// settles at a different frame from one rendered in a single block.
#[test]
fn the_trim_ramp_is_partition_invariant() {
    const SAMPLES: u32 = 48;
    const FRAMES: usize = 64;
    let render = |partition: &[usize]| -> Vec<u32> {
        let mut live = input(parameters(
            channel(0.0, false, 120.0, 8_000.0),
            channel(0.0, false, 120.0, 8_000.0),
        ));
        live.set_trim_db(BuiltinLaneSelector::Both, -18.0, SAMPLES)
            .expect("trim domain");
        let mut left = probe_block(FRAMES);
        let mut right: Vec<f32> = probe_block(FRAMES).iter().map(|value| -*value).collect();
        let mut offset = 0;
        for chunk in partition {
            let end = offset + chunk;
            live.process(
                DualMonoBlock::new(
                    &mut left[offset..end],
                    &mut right[offset..end],
                    offset as u64,
                )
                .expect("block"),
            );
            offset = end;
        }
        assert_eq!(offset, FRAMES);
        bits(&left).into_iter().chain(bits(&right)).collect()
    };
    let whole = render(&[FRAMES]);
    for partition in [
        vec![32, 32],
        vec![1, 63],
        vec![47, 1, 16],
        vec![16, 16, 16, 16],
    ] {
        assert_eq!(
            render(&partition),
            whole,
            "a ramp split {partition:?} must render the bits of one whole block"
        );
    }
}

// ---------------------------------------------------------------------------------------------
// 3. The ramping body is the settled body when the coefficient does not move.
// ---------------------------------------------------------------------------------------------

/// The equivalence the "ramping ignores the elision plan" decision rests on.
///
/// A retarget to the value a lane already holds gives `step == +0.0` and `target == current`, so
/// every frame's trim is the settled trim -- but the block goes through
/// `input_chain_ramp_block`, which is the **unelided** body, while the settled arm goes through
/// `input_chain_block_elided`. On an all-disabled bank those two run structurally different code
/// (`identity_chain_block`'s explicit `add(+0.0)` against a full `svf_step` pair over identity
/// coefficients), and the whole Job-1 argument is that they agree. This forces that question over
/// a `-0.0`-rich signal, which is the one input where an "equivalent" identity section is not.
///
/// Red mutation: give `input_chain_ramp_block` the three-shape dispatch
/// `input_chain_block_elided` has, but read the plan at the *wrong* channel -> the arms disagree
/// on an asymmetric-plan bank. Red mutation: drop the `.add(zero)` from `identity_chain_block` ->
/// the elided arm stops washing `-0.0` and this comparison fails on the second sample.
#[test]
fn settled_and_ramping_paths_agree_on_an_elided_bank() {
    for (hpf, lpf, label) in [
        (0.0_f32, 0.0_f32, "every section elided"),
        (80.0, 0.0, "the high-pass only"),
        (0.0, 9_000.0, "the low-pass only"),
        (80.0, 9_000.0, "nothing elided"),
    ] {
        for polarity in [false, true] {
            let params = parameters(
                channel(-4.5, polarity, hpf, lpf),
                channel(-4.5, polarity, hpf, lpf),
            );
            let mut settled = input(params);
            let mut ramping = input(params);
            // A retarget to the value already in force. The window is nonzero, so this genuinely
            // takes the ramping arm.
            ramping
                .set_trim_db(BuiltinLaneSelector::Both, -4.5, 32)
                .expect("trim domain");

            let mut settled_left = probe_block(96);
            let mut settled_right: Vec<f32> = probe_block(96).iter().map(|value| -*value).collect();
            let mut ramping_left = settled_left.clone();
            let mut ramping_right = settled_right.clone();
            settled.process(
                DualMonoBlock::new(&mut settled_left, &mut settled_right, 0).expect("block"),
            );
            ramping.process(
                DualMonoBlock::new(&mut ramping_left, &mut ramping_right, 0).expect("block"),
            );
            assert_eq!(
                bits(&settled_left),
                bits(&ramping_left),
                "{label}, polarity={polarity}: left plane"
            );
            assert_eq!(
                bits(&settled_right),
                bits(&ramping_right),
                "{label}, polarity={polarity}: right plane"
            );
            assert_eq!(
                input_state_words(&settled),
                input_state_words(&ramping),
                "{label}, polarity={polarity}: retained integrators"
            );
            // 5. And the plan is still the plan: the unelided body left every elided section's
            // integrators at exactly `+0.0`, so Job 1's decision has not gone stale.
            assert_eq!(
                input_elision_plan(&settled),
                input_elision_plan(&ramping),
                "{label}: the elision plan is a function of words a trim ramp does not write"
            );
        }
    }
}

/// The banked form of the same equivalence, at both widths, with the plan read back.
#[test]
fn a_ramping_block_leaves_an_elidable_banks_integrators_at_positive_zero() {
    for (backend, width) in BANKS {
        let lanes = width.lanes() as usize;
        let disabled = parameters(
            channel(-2.0, false, 0.0, 0.0),
            channel(-2.0, false, 0.0, 0.0),
        );
        let inputs: Vec<InputBuiltins> = (0..lanes).map(|_| input(disabled)).collect();
        let mut bank = BuiltinInputBank::new(backend, width, inputs).expect("bank");
        assert_eq!(bank_elision_plan(&bank), [[true; 2]; 2]);
        bank.set_trim_db(0, BuiltinLaneSelector::Both, -20.0, 16)
            .expect("trim domain");
        let frames = 64;
        let mut left = probe_block(frames * lanes);
        let mut right: Vec<f32> = left.iter().map(|value| -*value).collect();
        bank.process(&mut left, &mut right, frames as u32);
        for lane in 0..lanes {
            assert_eq!(
                bank_lane_state_words(&bank, lane),
                [0; 8],
                "lane {lane} at {width:?}: an identity section's integrators stay +0.0 through a \
                 ramping block, which is what keeps the elision plan valid"
            );
        }
        assert_eq!(
            bank_elision_plan(&bank),
            [[true; 2]; 2],
            "the plan a trim ramp cannot invalidate"
        );
    }
}

// ---------------------------------------------------------------------------------------------
// 4. Polarity.
// ---------------------------------------------------------------------------------------------

/// A flip settles at exactly the coefficient a re-prepared inverted session designs, and it gets
/// there through zero, monotonically.
///
/// Red mutation: make `set_polarity_invert` negate the *current* word rather than the target's
/// magnitude -> a flip issued mid-ramp settles at a value that is not `-trim_signed`, and the
/// re-prepared comparison fails.
///
/// Red mutation: make `set_polarity_invert` reuse `set_trim_db`'s sign-preserving rule -> the
/// flip is a no-op and the monotone crossing never happens.
#[test]
fn a_polarity_flip_crosses_zero_and_settles_at_the_reprepared_coefficient() {
    for trim_db in [0.0_f32, -6.0, 12.0, -144.0, 24.0] {
        for samples in [2_u32, 8, 64] {
            let mut live = input(parameters(
                channel(trim_db, false, 0.0, 0.0),
                channel(trim_db, false, 0.0, 0.0),
            ));
            let start = live.trim_signed(0);
            live.set_polarity_invert(BuiltinLaneSelector::Both, true, samples);

            let mut observed = vec![start];
            for _ in 0..samples {
                let mut left = [1.0_f32];
                let mut right = [1.0_f32];
                live.process(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
                observed.push(live.trim_signed(0));
            }

            // Monotone descent, and it passes through zero rather than jumping the sign.
            for pair in observed.windows(2) {
                assert!(
                    pair[1] <= pair[0],
                    "a flip from +{trim_db} dB descends monotonically: {observed:?}"
                );
            }
            let crossings = observed
                .windows(2)
                .filter(|pair| pair[0] > 0.0 && pair[1] <= 0.0)
                .count();
            assert_eq!(
                crossings, 1,
                "the coefficient crosses zero exactly once over {samples} updates: {observed:?}"
            );

            // And it settles at the coefficient a re-prepared session designs, bit for bit.
            let inverted = input(parameters(
                channel(trim_db, true, 0.0, 0.0),
                channel(trim_db, true, 0.0, 0.0),
            ));
            assert_eq!(
                [live.trim_signed(0).to_bits(), live.trim_signed(1).to_bits()],
                input_trim_words(&inverted),
                "a settled flip is the re-prepared inverted coefficient"
            );

            // A settled flip renders the re-prepared session's bits, over the whole probe.
            let mut flipped = probe_block(48);
            let mut flipped_right: Vec<f32> = probe_block(48).iter().map(|value| -*value).collect();
            let mut reference_left = flipped.clone();
            let mut reference_right = flipped_right.clone();
            let mut reference = inverted;
            reference.process(
                DualMonoBlock::new(&mut reference_left, &mut reference_right, 0).expect("block"),
            );
            live.process(DualMonoBlock::new(&mut flipped, &mut flipped_right, 0).expect("block"));
            assert_eq!(bits(&reference_left), bits(&flipped), "settled left plane");
            assert_eq!(
                bits(&reference_right),
                bits(&flipped_right),
                "settled right plane"
            );
        }
    }
}

/// The two parameters share one coefficient and do not overwrite each other.
///
/// Red mutation: drop the sign-preserving branch from `set_trim_db` -> a trim ride silently clears
/// a flip. Red mutation: drop the `.abs()` from `set_polarity_invert` -> a second `true` flip
/// re-negates instead of being idempotent.
#[test]
fn trim_and_polarity_do_not_overwrite_each_other() {
    let mut live = input(parameters(
        channel(0.0, true, 0.0, 0.0),
        channel(0.0, true, 0.0, 0.0),
    ));
    // A trim ride on an inverted lane keeps the inversion.
    live.set_trim_db(BuiltinLaneSelector::Both, -12.0, 0)
        .expect("trim domain");
    let inverted_minus_twelve = input(parameters(
        channel(-12.0, true, 0.0, 0.0),
        channel(-12.0, true, 0.0, 0.0),
    ));
    assert_eq!(
        live.trim_signed(0).to_bits(),
        input_trim_words(&inverted_minus_twelve)[0],
        "a trim ride preserves polarity"
    );

    // A redundant flip is idempotent, not a second negation.
    live.set_polarity_invert(BuiltinLaneSelector::Both, true, 0);
    assert_eq!(
        live.trim_signed(0).to_bits(),
        input_trim_words(&inverted_minus_twelve)[0],
        "setting an already-set polarity changes nothing"
    );

    // And clearing it returns the magnitude untouched.
    live.set_polarity_invert(BuiltinLaneSelector::Both, false, 0);
    let upright_minus_twelve = input(parameters(
        channel(-12.0, false, 0.0, 0.0),
        channel(-12.0, false, 0.0, 0.0),
    ));
    assert_eq!(
        live.trim_signed(0).to_bits(),
        input_trim_words(&upright_minus_twelve)[0],
        "clearing polarity preserves the trim magnitude"
    );
}

/// A lane selector addresses exactly the lanes it names.
///
/// Red mutation: make `set_trim_signed` ignore `channels.covers` -> a `Left` command moves the
/// right lane too, and the right channel's assertion fails.
#[test]
fn a_lane_selector_addresses_exactly_the_lanes_it_names() {
    for (selector, moves_left, moves_right) in [
        (BuiltinLaneSelector::Left, true, false),
        (BuiltinLaneSelector::Right, false, true),
        (BuiltinLaneSelector::Both, true, true),
    ] {
        let mut live = input(parameters(
            channel(0.0, false, 0.0, 0.0),
            channel(0.0, false, 0.0, 0.0),
        ));
        let before = [live.trim_signed(0), live.trim_signed(1)];
        live.set_trim_db(selector, -30.0, 0).expect("trim domain");
        assert_eq!(
            live.trim_signed(0) != before[0],
            moves_left,
            "{selector:?} on the left lane"
        );
        assert_eq!(
            live.trim_signed(1) != before[1],
            moves_right,
            "{selector:?} on the right lane"
        );
    }
}

/// The live domain is `trim_db`'s declared domain, refused on the same terms preparation refuses a
/// declared value.
#[test]
fn the_live_trim_domain_is_the_declared_one() {
    let mut live = input(parameters(
        channel(0.0, false, 0.0, 0.0),
        channel(0.0, false, 0.0, 0.0),
    ));
    for accepted in [-144.0_f32, -0.0, 0.0, 23.999, 24.0] {
        assert!(
            live.set_trim_db(BuiltinLaneSelector::Both, accepted, 8)
                .is_ok(),
            "{accepted} dB is inside `trim_db`'s declared domain"
        );
    }
    for refused in [
        -144.001_f32,
        24.001,
        f32::NAN,
        f32::INFINITY,
        -f32::INFINITY,
    ] {
        assert!(
            matches!(
                live.set_trim_db(BuiltinLaneSelector::Both, refused, 8),
                Err(BuiltinParameterError::GainDomain)
            ),
            "{refused} dB is outside `trim_db`'s declared domain"
        );
    }
}

/// A retarget on a padding lane, or on a lane index past the bank's members, is refused rather
/// than silently written into a lane no track owns.
#[test]
fn a_bank_refuses_a_retarget_addressed_past_its_members() {
    for (backend, width) in BANKS {
        let params = parameters(channel(0.0, false, 0.0, 0.0), channel(0.0, false, 0.0, 0.0));
        let mut bank = BuiltinInputBank::new(backend, width, vec![input(params), input(params)])
            .expect("a two-member bank");
        assert!(
            bank.set_trim_db(1, BuiltinLaneSelector::Both, -6.0, 4)
                .is_ok()
        );
        for lane in 2..width.lanes() as usize + 2 {
            assert!(
                matches!(
                    bank.set_trim_db(lane, BuiltinLaneSelector::Both, -6.0, 4),
                    Err(BuiltinParameterError::LaneLength)
                ),
                "lane {lane} is padding or absent at {width:?}"
            );
            assert!(
                matches!(
                    bank.set_polarity_invert(lane, BuiltinLaneSelector::Both, true, 4),
                    Err(BuiltinParameterError::LaneLength)
                ),
                "lane {lane} is padding or absent at {width:?}"
            );
        }
    }
}

/// A banked lane's coefficient sequence is the sequence that track produces alone.
///
/// Cohort invariance for the new ramp: a lane's ramp evolves by its own additions, so its bits do
/// not depend on which tracks share its bank.
#[test]
fn a_banked_lane_ramps_exactly_as_the_same_track_alone() {
    const FRAMES: usize = 40;
    const SAMPLES: u32 = 24;
    for (backend, width) in BANKS {
        let lanes = width.lanes() as usize;
        let params = |index: usize| {
            let value = channel(index as f32 - 3.0, index.is_multiple_of(3), 90.0, 7_000.0);
            parameters(value, value)
        };
        let inputs: Vec<InputBuiltins> = (0..lanes).map(|index| input(params(index))).collect();
        let mut bank = BuiltinInputBank::new(backend, width, inputs).expect("bank");
        for lane in 0..lanes {
            bank.set_trim_db(lane, BuiltinLaneSelector::Both, -9.0 - lane as f32, SAMPLES)
                .expect("trim domain");
        }
        let planar = probe_block(FRAMES);
        let mut left = vec![0.0_f32; FRAMES * lanes];
        let mut right = vec![0.0_f32; FRAMES * lanes];
        for frame in 0..FRAMES {
            for lane in 0..lanes {
                left[frame * lanes + lane] = planar[frame];
                right[frame * lanes + lane] = -planar[frame];
            }
        }
        bank.process(&mut left, &mut right, FRAMES as u32);

        for lane in 0..lanes {
            let mut alone = input(params(lane));
            alone
                .set_trim_db(BuiltinLaneSelector::Both, -9.0 - lane as f32, SAMPLES)
                .expect("trim domain");
            let mut alone_left = planar.clone();
            let mut alone_right: Vec<f32> = planar.iter().map(|value| -*value).collect();
            alone.process(DualMonoBlock::new(&mut alone_left, &mut alone_right, 0).expect("block"));
            let banked_left: Vec<u32> = (0..FRAMES)
                .map(|frame| left[frame * lanes + lane].to_bits())
                .collect();
            let banked_right: Vec<u32> = (0..FRAMES)
                .map(|frame| right[frame * lanes + lane].to_bits())
                .collect();
            assert_eq!(
                banked_left,
                bits(&alone_left),
                "lane {lane} at {width:?}, left"
            );
            assert_eq!(
                banked_right,
                bits(&alone_right),
                "lane {lane} at {width:?}, right"
            );
            assert_eq!(
                bank.trim_signed(lane, 0).to_bits(),
                alone.trim_signed(0).to_bits(),
                "lane {lane} at {width:?}, settled coefficient"
            );
        }
    }
}
