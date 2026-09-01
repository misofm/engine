//! The delay's cross-target determinism corpus (gate G5).
//!
//! Two cases render a fixed input through the real prepared effect and yield one `u32` result word
//! per output sample. `tests/determinism.rs`-style assertions live in this crate's own test module
//! and compare each case against [`G5_DIGESTS`]; `tools/wasm-gate-corpus` replays the
//! identical cases under wasmtime, at the wasm scalar and simd128 backends, against these same
//! pins. Together they are the cross-target half of decision D5 for this effect: the delay renders
//! the same bits in a browser as on a native host.
//!
//! # Why the input is built from integers
//!
//! Every sample is produced by a `xorshift64*` and exact conversions, so a target cannot differ on
//! the *input* and make a digest mismatch look like a numerics difference.
//!
//! # Why there is no width axis
//!
//! A gathered two-second ring has no `W4`/`W8` kernel: the delay is a `W = 1` effect (master plan
//! #83 §4.1), so a case is run once and its digest cannot depend on a backend width. What the wasm
//! leg is actually testing here is the software FMA of §3.5, which the kernel reaches six times per
//! stereo frame.
//!
//! # No NaN
//!
//! The determinism claim excludes NaN payloads. Both cases keep `|feedback| <= 0.95` and a bounded
//! input, so every sample is finite; the test module checks that rather than assuming it.

use effect_contract::{
    AutomationSpanKind, EffectProcessBlock, EffectQuality, InitialParameterValue, LinkMode,
    NativeEffectFactory, ParameterChannel, PrepareEffectLimits, PrepareEffectRequest,
    PreparedAutomationSpan, PreparedPorts, PreparedSidechainPort,
};

use crate::{DELAY_PARAMETERS, DelayFactory};

/// Sample rate every case renders at.
pub const SAMPLE_RATE: u32 = 48_000;

/// Frames each case renders.
pub const FRAMES: usize = 8_192;

/// Block length each case renders in. Fixed, because the digest describes the arithmetic; that it
/// does not depend on the block length is gate `partition_invariance_over_1_7_64_128_512`.
const BLOCK_FRAMES: usize = 128;

/// Frames between automation events in the automated case.
const EVENT_FRAMES: usize = 1_024;

/// Result words per case: the left output, then the right.
pub const POINTS: usize = FRAMES * 2;

/// Number of corpus cases.
pub const CASE_COUNT: usize = 2;

/// Human-readable name of each case, indexed by case number.
pub const CASE_NAMES: [&str; CASE_COUNT] = ["dual_mono", "ping_pong_automated"];

/// `xorshift64*` (Vigna 2016). Integer-only, so every target builds the same sequence.
struct Rng(u64);

impl Rng {
    const fn new(case: usize) -> Self {
        Self(0x2545_f491_4f6c_dd1d ^ ((case as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1))
    }

    fn next(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        (x.wrapping_mul(0x2545_f491_4f6c_dd1d) >> 32) as u32
    }

    /// A sample in `[-1, 1)` from the top sixteen bits, by exact conversion.
    fn sample(&mut self) -> f32 {
        f32::from((self.next() >> 16) as u16) * (2.0 / 65_536.0) - 1.0
    }
}

/// The initial parameter values of one case.
fn initial_values(case: usize) -> [InitialParameterValue; 9] {
    let mut values: [InitialParameterValue; 9] =
        core::array::from_fn(|index| InitialParameterValue {
            parameter_index: (index / 2) as u32,
            channel: if index % 2 == 0 {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: DELAY_PARAMETERS[index / 2].default_value,
        });
    values[8] = InitialParameterValue {
        parameter_index: 4,
        channel: ParameterChannel::Both,
        value: 0.0,
    };
    if case == 1 {
        // Short taps, so a crossfade completes inside the corpus; asymmetric feedback and damping,
        // so the two lanes are genuinely different signals; the matrix engaged, so a difference in
        // one lane cannot hide in the other.
        values[0].value = 3.0;
        values[1].value = 5.0;
        values[2].value = 0.75;
        values[3].value = -0.6;
        values[4].value = 0.25;
        values[5].value = 0.9;
        values[6].value = 0.5;
        values[7].value = 0.75;
        values[8].value = 0.5;
    } else {
        values[0].value = 7.0;
        values[1].value = 11.0;
        values[2].value = 0.5;
        values[3].value = -0.5;
        values[4].value = 0.0;
        values[5].value = 0.995;
        values[6].value = 1.0;
        values[7].value = 0.35;
    }
    values
}

/// The automation event that lands on sample `first_sample` of the automated case.
fn event(first_sample: u64) -> [PreparedAutomationSpan; 5] {
    let step = (first_sample as usize / EVENT_FRAMES) % 4;
    let delay = [3.0_f32, 9.0, 2.0, 6.0];
    let damping = [0.0_f32, 0.4, 0.995, 0.15];
    let mix = [1.0_f32, 0.0, 0.25, 0.6];
    let cross = [0.5_f32, 1.0, 0.0, 0.75];
    let point = |parameter_index: u32, channel, value| PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel,
        parameter_index,
        start_sample: first_sample,
        end_sample: first_sample,
        start_value: value,
        end_value: value,
    };
    [
        point(0, ParameterChannel::Left, delay[step]),
        point(0, ParameterChannel::Right, delay[(step + 2) % 4]),
        point(2, ParameterChannel::Left, damping[step]),
        point(3, ParameterChannel::Right, mix[step]),
        point(4, ParameterChannel::Both, cross[step]),
    ]
}

/// Runs one case and writes [`POINTS`] result words: the left output, then the right.
///
/// # Panics
///
/// Panics if `case >= CASE_COUNT`, if `out` is not [`POINTS`] words, or if the frozen prepare
/// request is rejected — all of which are corpus bugs, not runtime conditions.
pub fn run_case(case: usize, out: &mut [u32]) {
    assert!(case < CASE_COUNT, "corpus case index out of range");
    assert_eq!(out.len(), POINTS, "corpus result buffer length");

    let values = initial_values(case);
    let mut effect = DelayFactory
        .prepare(PrepareEffectRequest {
            sample_rate: SAMPLE_RATE,
            quantum: BLOCK_FRAMES as u32,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::None,
            },
            initial_values: &values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 768_168,
                maximum_scratch_bytes: 36,
                maximum_automation_spans_per_block: 16,
            },
        })
        .expect("frozen corpus prepare request");

    let mut random = Rng::new(case);
    let mut left = vec![0.0_f32; FRAMES];
    let mut right = vec![0.0_f32; FRAMES];
    for frame in 0..FRAMES {
        left[frame] = random.sample();
        right[frame] = random.sample();
        // Impulses, so a tap-timing difference is visible and not averaged away by noise.
        if frame % 1_531 == 0 {
            left[frame] = 1.0;
            right[frame] = -1.0;
        }
    }

    let mut offset = 0;
    while offset < FRAMES {
        let end = offset + BLOCK_FRAMES;
        let spans = event(offset as u64);
        let automation: &[PreparedAutomationSpan] =
            if case == 1 && offset > 0 && offset % EVENT_FRAMES == 0 {
                &spans
            } else {
                &[]
            };
        let report = effect.process(
            EffectProcessBlock::new(
                &mut left[offset..end],
                &mut right[offset..end],
                None,
                offset as u64,
                automation,
                BLOCK_FRAMES as u32,
            )
            .expect("frozen corpus block"),
        );
        assert_eq!(report.invalid_spans, 0, "corpus automation must be legal");
        assert_eq!(
            report.nonfinite_left_blocks + report.nonfinite_right_blocks,
            0,
            "corpus must stay finite"
        );
        offset = end;
    }

    for (word, value) in out.iter_mut().zip(left.iter().chain(right.iter())) {
        *word = value.to_bits();
    }
}

/// SHA-256 of each case's result words, little-endian, generated once from the `W = 1` scalar
/// `Lane` instantiation on `x86_64` and frozen (master plan #83 §8: a pin comes from the oracle,
/// never from copying production output).
///
/// A mismatch is never fixed by re-pinning. It means either the corpus changed or a target stopped
/// agreeing with the scalar oracle, and the second is what this gate exists to catch.
pub const G5_DIGESTS: [[u8; 32]; CASE_COUNT] = [
    // dual_mono
    [
        0x66, 0x15, 0xD8, 0x9B, 0xD1, 0x21, 0xD3, 0x0C, 0xAD, 0x22, 0x60, 0xBE, 0xDA, 0xCB, 0xD7,
        0x4A, 0x2B, 0x59, 0x10, 0xD9, 0x31, 0x98, 0x00, 0xB8, 0xE3, 0xFB, 0x97, 0x46, 0x87, 0x4F,
        0x1F, 0x2E,
    ],
    // ping_pong_automated
    [
        0x2B, 0xDB, 0x23, 0x50, 0xB4, 0x59, 0xAE, 0x19, 0x54, 0x09, 0x36, 0x38, 0x59, 0x82, 0xD3,
        0xD8, 0xB9, 0xBE, 0x98, 0x87, 0x4D, 0x8B, 0x7E, 0x6E, 0x2E, 0x52, 0x6F, 0x77, 0x6E, 0x94,
        0x9A, 0xA5,
    ],
];
