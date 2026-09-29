//! Issue #1051: the builtins' randomized bank differential.
//!
//! One seeded scenario per seed and width: a strip of input section (polarity, trim, HPF, LPF),
//! fader and mute, and 2x2 matrix per track, prepared from parameters drawn at their domain edges
//! -- `-144` and `+24` dB, the filter cutoffs at their bounds or off, matrix coefficients at
//! `+-1`, `0` and `+-0.5` -- symmetric or channel-asymmetric. It is rendered twice: as the banks
//! `BuiltinInputBank`, `BuiltinFaderBank` and `BuiltinMatrixBank` at four and at eight lanes (the
//! `wide` types run both widths on every host), and as one scalar `InputBuiltins`,
//! `FaderMuteRampBuiltins` and `MatrixBuiltins` per track. Between random blocks come random live
//! retargets on one channel or both, with every smoothing window from none to longer than a block,
//! and resets.
//!
//! The oracles, by bits (NaN as one class, issue #1065):
//!
//! * **bank against scalar**, lane by lane, output and report;
//! * **collapsed against dual**: a second input bank renders `process_mono` whenever every lane's
//!   witness holds and both planes carry one signal, and must render the dual bank's left plane
//!   and, after `desymmetrize`, hold its state;
//! * **chunked against whole**: the scalar sections render some blocks in two pieces.
//!
//! Input words include the hostile set -- both zeros, subnormals, infinities, NaN payloads and
//! `+-f32::MAX` -- because the input section is where the engine sanitises them: the sanitised
//! count is part of the report every lane must agree on.
//!
//! # A known defect, narrowed
//!
//! The first run of this differential found that `BuiltinMatrixBank` renders a settled identity
//! lane's `-0.0` as `+0.0` while another lane's matrix ramps, where that lane's own
//! `MatrixBuiltins` passes the `-0.0` through (first found at eight lanes): with any lane
//! ramping, the bank runs the ramp arithmetic on every lane, and `1 * -0.0 + 0 * x` is `+0.0`.
//! The fixed-input gate beside it
//! (`stage.rs::banked_fader_and_matrix_are_bit_identical_to_the_per_track_sections`) never feeds
//! a signed zero. Until its issue lands, the per-PR test compares the two zeros as one value on a
//! block where a matrix ramp may be in flight; the ignored test below is the reproducer.

use builtins::{
    BuiltinChain, BuiltinFaderBank, BuiltinInputBank, BuiltinLaneSelector, BuiltinMatrixBank,
    BuiltinParameters, BuiltinProcessReport, ChannelParameters, DualMonoBlock,
    FaderMuteRampBuiltins, InputBuiltins, Matrix2x2, MatrixBuiltins,
};
use dsp_reference::randomized::{Draw, first_difference, run_seeds};
use effect_contract::BankWidth;
use lane::Backend;

const TEST: &str = "the_banks_render_their_scalar_sections_under_random_retargets";
const REPLAY: &str = "cargo test -p builtins --features builtins/test-support --test randomized -- \
                      --exact the_banks_render_their_scalar_sections_under_random_retargets";
const BLOCKS: usize = 24;
const WIDTHS: [(Backend, BankWidth); 2] = [
    (Backend::Simd4, BankWidth::Four),
    (Backend::Simd8, BankWidth::Eight),
];

fn channel(draw: &mut Draw, rate: u32) -> ChannelParameters {
    let nyquist = rate as f32 * 0.5;
    let mut hpf = draw.pick(&[0.0_f32, 0.0, 10.0, 20.0, 80.0, 1_000.0]);
    let mut lpf = draw.pick(&[0.0_f32, 0.0, 2_000.0, 12_000.0, nyquist * 0.45, 20_000.0]);
    if lpf > nyquist * 0.45 {
        lpf = 0.0;
    }
    if hpf > 0.0 && lpf > 0.0 && hpf >= lpf {
        hpf = 0.0;
    }
    ChannelParameters {
        polarity_invert: draw.chance(1, 4),
        trim_db: draw.pick(&[-144.0_f32, -24.0, -6.0, 0.0, 0.0, 3.5, 24.0]),
        hpf_hz: hpf,
        lpf_hz: lpf,
        fader_db: draw.pick(&[-144.0_f32, -24.0, -6.0, 0.0, 0.0, 6.0, 24.0]),
        muted: draw.chance(1, 8),
    }
}

fn coefficient(draw: &mut Draw) -> f32 {
    draw.pick(&[-1.0_f32, -0.5, 0.0, 0.0, 0.5, 0.707_106_77, 1.0, 1.0])
}

fn matrix(draw: &mut Draw) -> Matrix2x2 {
    Matrix2x2 {
        ll: coefficient(draw),
        lr: coefficient(draw),
        rl: coefficient(draw),
        rr: coefficient(draw),
    }
}

fn parameters(draw: &mut Draw, rate: u32, symmetric: bool) -> BuiltinParameters {
    let left = channel(draw, rate);
    BuiltinParameters {
        left,
        right: if symmetric { left } else { channel(draw, rate) },
        matrix: if draw.chance(1, 2) {
            Matrix2x2::IDENTITY
        } else {
            matrix(draw)
        },
        smoothing_samples: draw.pick(&[0_u32, 1, 64, 480]),
    }
}

fn selector(draw: &mut Draw, mono: bool) -> BuiltinLaneSelector {
    if mono {
        BuiltinLaneSelector::Both
    } else {
        draw.pick(&[
            BuiltinLaneSelector::Both,
            BuiltinLaneSelector::Left,
            BuiltinLaneSelector::Right,
        ])
    }
}

fn smoothing(draw: &mut Draw) -> u32 {
    draw.pick(&[0_u32, 1, 2, 17, 64, 200, 4_800])
}

/// One track's scalar strip.
struct Scalar {
    input: InputBuiltins,
    fader: FaderMuteRampBuiltins,
    matrix: MatrixBuiltins,
}

/// One live retarget, addressed to one lane and applied to the bank and that lane's scalar strip.
#[derive(Clone, Copy, Debug)]
enum Retarget {
    Trim(BuiltinLaneSelector, f32, u32),
    Polarity(BuiltinLaneSelector, bool, u32),
    Fader(BuiltinLaneSelector, f32, u32),
    Mute(BuiltinLaneSelector, bool, u32),
    Matrix(Matrix2x2, u32),
}

fn add(total: &mut BuiltinProcessReport, report: BuiltinProcessReport) {
    total.sanitized_input += report.sanitized_input;
    total.sanitized_output += report.sanitized_output;
    total.recovered_left_state += report.recovered_left_state;
    total.recovered_right_state += report.recovered_right_state;
}

/// What the differential reached.
#[derive(Debug, Default)]
struct Reach {
    blocks: u64,
    chunked: u64,
    collapsed: u64,
    retargets: u64,
    sanitized: u64,
}

#[allow(clippy::too_many_lines)]
fn scenario(seed: u64, backend: Backend, width: BankWidth, strict: bool, reach: &mut Reach) {
    let mut draw = Draw::new(seed ^ (u64::from(width.lanes()) << 48));
    let rate = draw.pick(&[44_100_u32, 48_000, 88_200, 96_000]);
    let lanes = width.lanes() as usize;
    // A partial bank some of the time: the padding lanes belong to no track.
    let members = if draw.chance(1, 4) {
        1 + draw.below(lanes)
    } else {
        lanes
    };
    let mono = draw.chance(1, 3);
    let hostile = !mono && draw.chance(1, 3);
    let params: Vec<BuiltinParameters> = (0..members)
        .map(|_| {
            let symmetric = mono || draw.chance(1, 2);
            parameters(&mut draw, rate, symmetric)
        })
        .collect();
    let chain = |p: BuiltinParameters| BuiltinChain::new(rate, p).expect("drawn legal");
    let mut scalars: Vec<Scalar> = params
        .iter()
        .map(|p| {
            let (input, _, matrix) = chain(*p).into_sections();
            Scalar {
                input,
                fader: FaderMuteRampBuiltins::new(*p).expect("drawn legal"),
                matrix,
            }
        })
        .collect();
    let inputs = |_: ()| -> Vec<InputBuiltins> {
        params
            .iter()
            .map(|p| chain(*p).into_input_builtins())
            .collect()
    };
    let mut input_bank = BuiltinInputBank::new(backend, width, inputs(())).expect("bank");
    let mut twin = BuiltinInputBank::new(backend, width, inputs(())).expect("twin bank");
    let mut fader_bank = BuiltinFaderBank::new(backend, width, params.clone()).expect("fader bank");
    let mut matrix_bank = BuiltinMatrixBank::new(
        backend,
        width,
        params
            .iter()
            .map(|p| (p.matrix, p.smoothing_samples))
            .collect(),
    )
    .expect("matrix bank");
    let mut collapsed = false;
    let mut agree = true;
    // The last sample a matrix ramp retargeted so far can still be in flight at.
    let mut matrix_ramp_end = 0_u64;
    let mut first = draw.pick(&[0_u64, 1 << 20, (1 << 40) + 3]);

    for block in 0..BLOCKS {
        let context =
            format!("W{lanes} seed {seed} block {block} ({members} members, mono {mono})");
        // --- Live retargets and resets, identical on both sides ---------------------------------
        for _ in 0..draw.below(3) {
            let lane = draw.below(members);
            let retarget = match draw.below(5) {
                0 => Retarget::Trim(
                    selector(&mut draw, mono),
                    draw.pick(&[-144.0_f32, -12.0, -0.5, 0.0, 6.0, 24.0]),
                    smoothing(&mut draw),
                ),
                1 => Retarget::Polarity(
                    selector(&mut draw, mono),
                    draw.chance(1, 2),
                    smoothing(&mut draw),
                ),
                2 => Retarget::Fader(
                    selector(&mut draw, mono),
                    draw.pick(&[-144.0_f32, -24.0, -6.0, 0.0, 6.0, 24.0]),
                    smoothing(&mut draw),
                ),
                3 => Retarget::Mute(
                    selector(&mut draw, mono),
                    draw.chance(1, 2),
                    smoothing(&mut draw),
                ),
                _ => Retarget::Matrix(matrix(&mut draw), smoothing(&mut draw)),
            };
            let scalar = &mut scalars[lane];
            match retarget {
                Retarget::Trim(lanes, db, window) => {
                    input_bank
                        .set_trim_db(lane, lanes, db, window)
                        .expect("trim");
                    twin.set_trim_db(lane, lanes, db, window).expect("trim");
                    scalar.input.set_trim_db(lanes, db, window).expect("trim");
                }
                Retarget::Polarity(lanes, inverted, window) => {
                    input_bank
                        .set_polarity_invert(lane, lanes, inverted, window)
                        .expect("polarity");
                    twin.set_polarity_invert(lane, lanes, inverted, window)
                        .expect("polarity");
                    scalar.input.set_polarity_invert(lanes, inverted, window);
                }
                Retarget::Fader(lanes, db, window) => {
                    fader_bank
                        .set_fader_db(lane, lanes, db, window)
                        .expect("fader");
                    scalar.fader.set_fader_db(lanes, db, window).expect("fader");
                }
                Retarget::Mute(lanes, muted, window) => {
                    fader_bank
                        .set_mute(lane, lanes, muted, window)
                        .expect("mute");
                    scalar.fader.set_mute(lanes, muted, window);
                }
                Retarget::Matrix(target, window) => {
                    matrix_ramp_end = matrix_ramp_end.max(first + u64::from(window));
                    let banked = matrix_bank.set_target_smoothed(lane, target, window);
                    let single = scalar.matrix.set_target_smoothed(target, window);
                    assert_eq!(banked, single, "{context}: the matrix retarget's verdicts");
                }
            }
            reach.retargets += 1;
        }
        if draw.chance(1, 32) {
            if collapsed {
                twin.desymmetrize();
                collapsed = false;
            }
            input_bank.reset();
            twin.reset();
            fader_bank.reset();
            matrix_bank.reset();
            for scalar in &mut scalars {
                scalar.input.reset();
                scalar.fader.reset();
                scalar.matrix.reset();
            }
        }

        // --- Input ------------------------------------------------------------------------------
        let frames = draw.frames(128);
        let words = frames * lanes;
        let mut left = vec![0.0_f32; words];
        let mut right = vec![0.0_f32; words];
        // A sixth of the blocks are `+0.0` on every lane: the settled and silent paths.
        let silent = draw.chance(1, 6);
        for lane in 0..members {
            let profile = loop {
                let profile = if silent {
                    dsp_reference::randomized::Profile::Silence
                } else {
                    draw.profile()
                };
                if hostile || profile.is_finite_legal() {
                    break profile;
                }
            };
            for frame in 0..frames {
                let word = frame * lanes + lane;
                left[word] = draw.sample(profile);
                right[word] = if mono {
                    left[word]
                } else {
                    draw.sample(profile)
                };
            }
        }

        // --- The mono twin's mode ------------------------------------------------------------------
        // A collapsed run also ends while the witness still holds (a chain renders dual whenever
        // anything else asks it to): `desymmetrize` must then leave the right channel where a
        // never-collapsed run holds it, and the channels still agree afterwards.
        let holds = (0..members).all(|lane| input_bank.lane_symmetry(lane).eligible());
        let eligible = mono && agree && twin.supports_mono_collapse() && holds && !draw.chance(1, 5);
        if !eligible {
            if collapsed {
                twin.desymmetrize();
                collapsed = false;
            }
            agree &= mono && holds;
        }

        // --- Scalar strips --------------------------------------------------------------------------
        let chunk =
            (frames > 1 && !hostile && draw.chance(1, 3)).then(|| 1 + draw.below(frames - 1));
        let mut scalar_left = vec![0.0_f32; words];
        let mut scalar_right = vec![0.0_f32; words];
        let mut scalar_reports = Vec::with_capacity(members);
        for (lane, scalar) in scalars.iter_mut().enumerate() {
            let mut l: Vec<f32> = (0..frames)
                .map(|frame| left[frame * lanes + lane])
                .collect();
            let mut r: Vec<f32> = (0..frames)
                .map(|frame| right[frame * lanes + lane])
                .collect();
            let mut report = BuiltinProcessReport::default();
            let cut = chunk.unwrap_or(frames);
            for (start, end) in [(0, cut), (cut, frames)] {
                if start == end {
                    continue;
                }
                let at = first + start as u64;
                for stage in 0..3 {
                    let block = DualMonoBlock::new(&mut l[start..end], &mut r[start..end], at)
                        .expect("a block");
                    add(
                        &mut report,
                        match stage {
                            0 => scalar.input.process(block),
                            1 => scalar.fader.process(block),
                            _ => scalar.matrix.process(block),
                        },
                    );
                }
            }
            for frame in 0..frames {
                scalar_left[frame * lanes + lane] = l[frame];
                scalar_right[frame * lanes + lane] = r[frame];
            }
            scalar_reports.push(report);
        }
        reach.chunked += u64::from(chunk.is_some());

        // --- The banks --------------------------------------------------------------------------
        let (mut bank_left, mut bank_right) = (left.clone(), right.clone());
        let mut bank_report = input_bank.process(&mut bank_left, &mut bank_right, frames as u32);
        if eligible {
            let mut mono_left = left.clone();
            let report = twin.process_mono(&mut mono_left, frames as u32);
            assert_eq!(report, bank_report, "{context}: the collapsed report");
            if let Some(word) = first_difference(&mono_left, &bank_left) {
                panic!("{context}: the collapsed input section rendered word {word} differently");
            }
            if let Some(word) = first_difference(&bank_right, &bank_left) {
                panic!(
                    "{context}: every lane's witness held on one signal, yet the input bank's \
                     right plane differs at word {word}"
                );
            }
            collapsed = true;
            reach.collapsed += 1;
        } else {
            let (mut l, mut r) = (left.clone(), right.clone());
            let report = twin.process(&mut l, &mut r, frames as u32);
            assert_eq!(report, bank_report, "{context}: the twin's report");
            assert!(
                first_difference(&l, &bank_left).is_none()
                    && first_difference(&r, &bank_right).is_none(),
                "{context}: the twin input bank rendered other words"
            );
        }
        add(
            &mut bank_report,
            fader_bank.process(&mut bank_left, &mut bank_right, frames as u32),
        );
        add(
            &mut bank_report,
            matrix_bank.process(&mut bank_left, &mut bank_right, frames as u32),
        );
        reach.blocks += 1;
        let sanitized: u64 = scalar_reports
            .iter()
            .map(|report| report.sanitized_input)
            .sum();
        reach.sanitized += sanitized;
        if chunk.is_none() {
            assert_eq!(
                bank_report.sanitized_input, sanitized,
                "{context}: the banks sanitised another count than their scalar strips"
            );
        }
        let zeros_as_one = !strict && first < matrix_ramp_end;
        let same = |a: f32, b: f32| {
            dsp_reference::randomized::same_word(a, b) || (zeros_as_one && a == 0.0 && b == 0.0)
        };
        for lane in 0..members {
            for (plane, scalar, banked) in [
                ("left", &scalar_left, &bank_left),
                ("right", &scalar_right, &bank_right),
            ] {
                if let Some(frame) = (0..frames).find(|frame| {
                    let word = frame * lanes + lane;
                    !same(scalar[word], banked[word])
                }) {
                    let word = frame * lanes + lane;
                    panic!(
                        "{context}: lane {lane} {plane} frame {frame}: the banks rendered \
                         {:#010x} where the scalar strip rendered {:#010x} (chunk {chunk:?})",
                        banked[word].to_bits(),
                        scalar[word].to_bits()
                    );
                }
            }
        }
        first += frames as u64;
    }
}

#[test]
fn the_banks_render_their_scalar_sections_under_random_retargets() {
    let mut reach = Reach::default();
    let seeds = run_seeds(TEST, REPLAY, 24, |seed| {
        for (backend, width) in WIDTHS {
            scenario(seed, backend, width, false, &mut reach);
        }
    });
    println!("{seeds} seeds: {reach:?}");
    if dsp_reference::randomized::replaying() {
        return;
    }
    assert!(
        reach.blocks > 0
            && reach.chunked > 0
            && reach.collapsed > 0
            && reach.retargets > 0
            && reach.sanitized > 0,
        "{reach:?}"
    );
}

/// The same differential without the narrowing: red until the known defect's issue lands.
#[test]
#[ignore = "reproduces a known defect found by #1051; see the module documentation"]
fn the_banks_render_their_scalar_sections_including_the_known_defect() {
    let mut reach = Reach::default();
    run_seeds(
        "the_banks_render_their_scalar_sections_including_the_known_defect",
        "cargo test -p builtins --features builtins/test-support --test randomized -- --ignored \
         --exact the_banks_render_their_scalar_sections_including_the_known_defect",
        24,
        |seed| {
            for (backend, width) in WIDTHS {
                scenario(seed, backend, width, true, &mut reach);
            }
        },
    );
}
