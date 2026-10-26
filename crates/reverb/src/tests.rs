//! Unit tests: they reach the width-generic kernel and the private tap state.

use super::*;
use dsp_reference::{ReferenceReverb, ReferenceReverbParameters};
use effect_contract::{InitialParameterValue, LinkMode};

const QUANTUM: u32 = 512;

fn defaults() -> [f32; PARAMETER_COUNT] {
    core::array::from_fn(|index| REVERB_PARAMETERS[index].default_value)
}

fn initial(values: &[f32; PARAMETER_COUNT]) -> [InitialParameterValue; PARAMETER_COUNT] {
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: index as u32,
        channel: ParameterChannel::Both,
        value: values[index],
    })
}

fn prepare_at(values: &[f32; PARAMETER_COUNT], sample_rate: u32) -> PreparedReverb {
    let initial = initial(values);
    let sizes = state_sizes(sample_rate);
    let request = PrepareEffectRequest {
        sample_rate,
        quantum: QUANTUM,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: effect_contract::PreparedPorts {
            sidechain: effect_contract::PreparedSidechainPort::None,
        },
        initial_values: &initial,
        limits: effect_contract::PrepareEffectLimits {
            maximum_total_state_bytes: sizes.total().expect("state total"),
            maximum_scratch_bytes: 1,
            maximum_automation_spans_per_block: 16,
        },
        tail_bound: conformance::tail_bound_of(
            Box::new(ReverbFactory),
            sample_rate,
            EffectQuality::Normal,
        ),
    };
    prepare_reverb(request).expect("prepare").0
}

/// Deterministic noise with sparse impulses, peak 1, from an integer generator.
fn signal(frames: usize, seed: u64) -> (Vec<f32>, Vec<f32>) {
    let mut state = seed | 1;
    let mut next = || {
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        state.wrapping_mul(0x2545_f491_4f6c_dd1d) >> 32
    };
    let mut left = vec![0.0_f32; frames];
    let mut right = vec![0.0_f32; frames];
    for index in 0..frames {
        left[index] = f32::from((next() >> 16) as u16) * (1.0 / 65_536.0) - 0.5;
        right[index] = f32::from((next() >> 16) as u16) * (1.0 / 65_536.0) - 0.5;
        if index % 7_919 == 0 {
            left[index] = 1.0;
            right[index] = -1.0;
        }
    }
    (left, right)
}

fn point(parameter_index: usize, sample: u64, value: f32) -> PreparedAutomationSpan {
    PreparedAutomationSpan {
        channel: ParameterChannel::Both,
        parameter_index: parameter_index as u32,
        sample,
        value,
    }
}

/// `lcm(7, 512)`, and a multiple of 1, 64 and 128: every partition starts a block here.
const EVENT_EVERY: usize = 3_584;

/// The events at sample `sample` (a multiple of [`EVENT_EVERY`]): every parameter moves. A
/// second `size` and `pre_delay_ms` event follows [`QUEUED_AFTER`] samples later on renders of
/// block length 1 and 7, so a crossfade is queued behind a running one.
fn events(sample: usize) -> Vec<PreparedAutomationSpan> {
    let step = sample / EVENT_EVERY;
    let at = sample as u64;
    let pre = [0.0_f32, 3.0, 250.0, 17.5];
    let decay = [100.0_f32, 20_000.0, 900.0, 4_000.0];
    let size = [0.25_f32, 2.0, 1.3, 0.6, 1.0];
    let damping = [1_000.0_f32, 20_000.0, 3_000.0];
    let low_cut = [500.0_f32, 20.0, 120.0];
    let unit = [0.0_f32, 1.0, 0.3, 0.75];
    vec![
        point(PRE_DELAY, at, pre[step % 4]),
        point(DECAY, at, decay[step % 4]),
        point(SIZE, at, size[step % 5]),
        point(DAMPING, at, damping[step % 3]),
        point(LOW_CUT, at, low_cut[step % 3]),
        point(DIFFUSION, at, unit[step % 4]),
        point(WIDTH, at, unit[(step + 1) % 4]),
        point(MIX, at, unit[(step + 2) % 4]),
    ]
}

/// Samples from an [`events`] block start to its [`queued_events`]: a multiple of 7, so blocks of
/// 1 and 7 both start a block there, and below the 128-sample crossfade, so the change queues. The
/// running crossfade ends at sample 128, which is not a multiple of 7: the queued one starts
/// mid-block at block length 7 and on its own block at block length 1.
const QUEUED_AFTER: usize = 70;

/// The follow-up events [`QUEUED_AFTER`] samples after each [`events`] block start: a size and a
/// pre-delay change while the first crossfade is still running, so they queue.
fn queued_events(sample: usize) -> Vec<PreparedAutomationSpan> {
    let step = sample / EVENT_EVERY;
    let at = (sample + QUEUED_AFTER) as u64;
    let pre = [9.0_f32, 0.5, 120.0];
    let size = [1.7_f32, 0.4, 0.9];
    vec![
        point(PRE_DELAY, at, pre[step % 3]),
        point(SIZE, at, size[step % 3]),
    ]
}

/// Renders `frames` frames in blocks of `block` frames at lane type `L`; blocks of 1 and 7 also
/// carry the [`queued_events`].
fn render_with<L: Lane>(
    effect: &mut PreparedReverb,
    frames: usize,
    block: usize,
) -> (Vec<u32>, Vec<u32>, Vec<u8>) {
    let (mut left, mut right) = signal(frames, 0x2545_f491_4f6c_dd1d);
    let mut offset = 0;
    while offset < frames {
        let end = (offset + block).min(frames);
        let spans = if offset > 0 && offset % EVENT_EVERY == 0 {
            events(offset)
        } else if (block == 1 || block == 7)
            && offset > QUEUED_AFTER
            && (offset - QUEUED_AFTER).is_multiple_of(EVENT_EVERY)
        {
            queued_events(offset - QUEUED_AFTER)
        } else {
            Vec::new()
        };
        let report = effect.process_at::<L>(
            EffectProcessBlock::new(
                &mut left[offset..end],
                &mut right[offset..end],
                None,
                offset as u64,
                &spans,
                QUANTUM,
            )
            .expect("block"),
        );
        assert_eq!(report, ProcessReport::default(), "block at {offset}");
        offset = end;
    }
    (
        left.iter().map(|v| v.to_bits()).collect(),
        right.iter().map(|v| v.to_bits()).collect(),
        snapshot(effect),
    )
}

fn snapshot(effect: &PreparedReverb) -> Vec<u8> {
    let sizes = effect.state_sizes;
    let mut common = vec![0; sizes.common_bytes as usize];
    let mut left = vec![0; sizes.left_bytes as usize];
    let mut right = vec![0; sizes.right_bytes as usize];
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("output"),
        )
        .expect("snapshot");
    [common, left, right].concat()
}

/// Width identity: the kernel at `W = 1`, `4` and `8` (where the build has AVX2) gives the same
/// bits, through the history-filling start, the crossfades (queued ones included) and ramps of
/// the automation, and the full-history path.
///
/// Red for: a butterfly stage paired on the wrong vectors, a wrong re-layout gather or final
/// layout (a gain or the injection side on the wrong line, an output tap read from the wrong
/// position), or a diffuser lane mixed into another. Each changes `W = 4` or `8` against the
/// scalar leg; the oracle test runs one width only.
#[test]
fn every_lane_width_renders_the_same_bits() {
    const FRAMES: usize = 40_000;
    let reference = render_with::<f32>(&mut prepare_at(&defaults(), 48_000), FRAMES, 7);
    lane::each_vector_lane!(|L| {
        let produced = render_with::<L>(&mut prepare_at(&defaults(), 48_000), FRAMES, 7);
        assert!(
            produced == reference,
            "width {} differs from the scalar kernel",
            <L as Lane>::WIDTH
        );
    });
    // The run reached the full-history path, so both kernel instantiations were compared.
    let effect = prepare_at(&defaults(), 48_000);
    assert!(FRAMES as u32 > effect.geometry.history_cap + 512);
}

/// Partition invariance: block lengths 1, 7, 64, 128 and 512 give the same output bits and the
/// same final state, with automation, crossfades and ramps in flight.
///
/// Red for: a crossfade or ramp that starts, steps or snaps at a block boundary rather than at its
/// sample; a history counter or cursor advanced per block; a finiteness window computed from the
/// wrong cursor.
#[test]
fn partition_invariance_over_1_7_64_128_512() {
    const FRAMES: usize = 4 * EVENT_EVERY * 4;
    let render =
        |block| render_with::<lane::Native>(&mut prepare_at(&defaults(), 48_000), FRAMES, block);
    // Every block length starts a block at each `EVENT_EVERY` multiple.
    let reference = render(512);
    for block in [64_usize, 128] {
        assert!(
            render(block) == reference,
            "block {block} differs from block 512"
        );
    }
    // Blocks of 1 and 7 also carry `queued_events`: the queued crossfade starts mid-block at 7
    // and on its own sample at 1.
    assert!(render(1) == render(7), "block 1 differs from block 7");
}

fn reference_parameters(values: &[f32; PARAMETER_COUNT]) -> ReferenceReverbParameters {
    ReferenceReverbParameters {
        pre_delay_ms: f64::from(values[PRE_DELAY]),
        decay_ms: f64::from(values[DECAY]),
        size: f64::from(values[SIZE]),
        damping_hz: f64::from(values[DAMPING]),
        low_cut_hz: f64::from(values[LOW_CUT]),
        diffusion: f64::from(values[DIFFUSION]),
        width: f64::from(values[WIDTH]),
        mix: f64::from(values[MIX]),
    }
}

/// Largest `max |y_kernel - y_oracle|` relative to the input peak (`1.0`) over the first
/// [`ORACLE_SECONDS`] of a settled render. See [`kernel_matches_the_f64_oracle`] for why.
const ORACLE_TOLERANCE: f64 = 1.0e-6;
const ORACLE_SECONDS: f64 = 0.75;

/// The production kernel against the independent `f64` oracle, settled static parameters.
///
/// The tolerance. The kernel rounds every coefficient to `f32` (relative error at most
/// `u = 2^-24 = 6.0e-8`) and every operation; the oracle uses exact `f64` coefficients. The loop
/// is stable and contracting (every gain below 1, the matrix orthogonal, the filters passive), so
/// a rounding committed at one sample is not amplified: it decays with the tail. The difference
/// is then a sum of `O(u)` errors weighted by the decaying response, of the order of `u` times the
/// wet level times the number of trips a word has made, plus a slow `O(n u)` drift of the pole
/// positions times the response envelope. At these settings (decay 1.8 s, 0.75 s rendered) the
/// measured worst is `1.4e-7` (about `2.4 u`) of the input peak; the bound is about seven times
/// that. Any algorithmic departure (a wrong tap, sign, gain, filter coefficient, matrix entry,
/// output tap or mix law) gives an error of the order of the wet signal, `1e-3` or more.
///
/// Red for: any departure of the kernel from R1-R11 at a settled state (a wrong length rule, gain,
/// filter, matrix, output tap, width or mix law), which the width and partition tests cannot see
/// because they compare the kernel with itself.
#[test]
fn kernel_matches_the_f64_oracle() {
    let mut cases = Vec::new();
    for sample_rate in [48_000_u32, 96_000] {
        let base = defaults();
        cases.push((sample_rate, base));
        let mut small = base;
        small[SIZE] = 0.3;
        small[PRE_DELAY] = 0.0;
        cases.push((sample_rate, small));
        let mut bright = base;
        bright[DAMPING] = 20_000.0;
        bright[WIDTH] = 0.3;
        cases.push((sample_rate, bright));
        let mut dark = base;
        dark[DAMPING] = 1_000.0;
        dark[LOW_CUT] = 500.0;
        dark[MIX] = 0.5;
        dark[DIFFUSION] = 0.2;
        cases.push((sample_rate, dark));
    }
    let mut worst = 0.0_f64;
    for (sample_rate, values) in cases {
        let mut effect = prepare_at(&values, sample_rate);
        let mut oracle =
            ReferenceReverb::new(sample_rate, reference_parameters(&values)).expect("oracle");
        let lengths: Vec<usize> = effect.line_active.iter().map(|d| *d as usize).collect();
        assert_eq!(
            lengths,
            oracle.line_lengths().to_vec(),
            "line lengths at {sample_rate} Hz, {values:?}"
        );
        let frames = (ORACLE_SECONDS * f64::from(sample_rate)) as usize;
        let (mut left, mut right) = signal(frames, 0x9e37_79b9_7f4a_7c15);
        let input: Vec<(f32, f32)> = left.iter().copied().zip(right.iter().copied()).collect();
        for offset in (0..frames).step_by(QUANTUM as usize) {
            let end = (offset + QUANTUM as usize).min(frames);
            effect.process(
                EffectProcessBlock::new(
                    &mut left[offset..end],
                    &mut right[offset..end],
                    None,
                    offset as u64,
                    &[],
                    QUANTUM,
                )
                .expect("block"),
            );
        }
        let mut case_worst = 0.0_f64;
        for (frame, (x_left, x_right)) in input.into_iter().enumerate() {
            let (y_left, y_right) = oracle.process_sample(f64::from(x_left), f64::from(x_right));
            case_worst = case_worst
                .max((f64::from(left[frame]) - y_left).abs())
                .max((f64::from(right[frame]) - y_right).abs());
        }
        println!("oracle {sample_rate} Hz {values:?}: max error {case_worst:.3e}");
        worst = worst.max(case_worst);
    }
    assert!(worst <= ORACLE_TOLERANCE, "max error {worst:e}");
}

/// Non-finite recovery: a NaN or an infinity at the input zeroes both outputs of that block,
/// counts one non-finite block per channel and drops all history; the following blocks equal a
/// fresh instance's response to the same input, and no non-finite word is readable afterwards.
///
/// Red for: a check that misses a ring's write window (the poison reaches a later block's
/// output), a recovery that zeroes one channel only, a history drop that leaves a stale word
/// readable (the continuation differs from the fresh instance), or a counter that is not raised.
#[test]
fn nonfinite_input_recovers_both_channels_per_block() {
    for poison in [f32::NAN, f32::INFINITY] {
        let mut warm = prepare_at(&defaults(), 48_000);
        let (mut left, mut right) = signal(20_000, 7);
        for offset in (0..20_000).step_by(500) {
            warm.process(
                EffectProcessBlock::new(
                    &mut left[offset..offset + 500],
                    &mut right[offset..offset + 500],
                    None,
                    offset as u64,
                    &[],
                    QUANTUM,
                )
                .expect("warm"),
            );
        }
        let (mut left, mut right) = signal(256, 11);
        left[100] = poison;
        let report = warm.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 20_000, &[], QUANTUM)
                .expect("poisoned"),
        );
        assert_eq!(report.nonfinite_left_blocks, 1);
        assert_eq!(report.nonfinite_right_blocks, 1);
        assert_eq!(report.sanitized_main_samples, 0);
        assert!(left.iter().chain(&right).all(|v| v.to_bits() == 0));
        assert!(
            snapshot(&warm)
                .as_chunks::<4>()
                .0
                .iter()
                .all(|w| f32::from_le_bytes(*w).is_finite()),
            "a non-finite word is readable after recovery"
        );

        let mut fresh = prepare_at(&defaults(), 48_000);
        let (input_left, input_right) = signal(30_000, 13);
        let mut outputs = Vec::new();
        for (effect, start) in [(&mut warm, 20_256_u64), (&mut fresh, 0)] {
            let (mut left, mut right) = (input_left.clone(), input_right.clone());
            for offset in (0..30_000).step_by(500) {
                let report = effect.process(
                    EffectProcessBlock::new(
                        &mut left[offset..offset + 500],
                        &mut right[offset..offset + 500],
                        None,
                        start + offset as u64,
                        &[],
                        QUANTUM,
                    )
                    .expect("continue"),
                );
                assert_eq!(report, ProcessReport::default());
            }
            outputs.push((
                left.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                right.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            ));
        }
        assert!(
            outputs[0] == outputs[1],
            "recovery differs from a fresh instance ({poison})"
        );
    }
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// The first pair of lines whose lengths share a factor, if any.
fn common_factor(lengths: &[u32; LINES]) -> Option<(usize, usize)> {
    (0..LINES)
        .flat_map(|i| (i + 1..LINES).map(move |j| (i, j)))
        .find(|&(i, j)| gcd(lengths[i], lengths[j]) != 1)
}

/// The sixteen line lengths are pairwise coprime (the Schroeder/Jot criterion: no two lines
/// share a mode series) at every `size` an event or an initial value can set, at all four rates.
///
/// Exhaustive, not a sweep. The control is any `f32` in `[0.25, 2.0]` (`parameter_value_valid`),
/// and each length is a non-decreasing step function of it: widening to `f64` is exact,
/// `BASE_i * size` is exact (13 + 24 significant bits), each further correctly rounded product,
/// quotient and the `+ 1/2` are monotone, and so are `floor` and the nearest prime. So if the
/// production [`line_lengths`] gives the same vector at two `f32` sizes, it gives that vector at
/// every `f32` between them. The test bisects the domain's `f32` bit patterns until each interval
/// either has equal ends or ends at two adjacent `f32`s, so it evaluates the vector on both sides
/// of every breakpoint of every line, through the prepared instance's own sieve, and checks every
/// distinct vector with `gcd`.
///
/// Red for: two base lengths that coincide or round to a common prime at some size or rate, a
/// nearest-prime search or sieve that returns a composite, or a length rule that loses the prime
/// step. The spectral coloration test sees none of these: a duplicated base length passes it.
#[test]
fn line_lengths_are_pairwise_coprime_at_every_size_and_rate() {
    for sample_rate in [44_100_u32, 48_000, 88_200, 96_000] {
        let effect = prepare_at(&defaults(), sample_rate);
        let at = |bits: u32| {
            let size = f32::from_bits(bits);
            let lengths = line_lengths(size, sample_rate, &effect.sieve).expect("size in domain");
            if let Some((i, j)) = common_factor(&lengths) {
                panic!(
                    "{sample_rate} Hz, size {size:e}: lines {i} and {j} have lengths {} and {}",
                    lengths[i], lengths[j]
                );
            }
            lengths
        };
        let (low, high) = (0.25_f32.to_bits(), 2.0_f32.to_bits());
        let mut stack = vec![(low, at(low), high, at(high))];
        let mut breakpoints = 0_u32;
        while let Some((a, lengths_a, b, lengths_b)) = stack.pop() {
            if lengths_a == lengths_b {
                continue;
            }
            if b - a == 1 {
                breakpoints += 1;
                continue;
            }
            let middle = a + (b - a) / 2;
            let lengths_middle = at(middle);
            assert!(
                (0..LINES).all(|line| lengths_a[line] <= lengths_middle[line]
                    && lengths_middle[line] <= lengths_b[line]),
                "a length is not monotone in size at {sample_rate} Hz"
            );
            stack.push((a, lengths_a, middle, lengths_middle));
            stack.push((middle, lengths_middle, b, lengths_b));
        }
        println!("{sample_rate} Hz: {breakpoints} breakpoints");
        assert!(breakpoints > 0);
    }
}
