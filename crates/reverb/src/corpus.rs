//! The reverb's cross-target determinism corpus (gate G5).
//!
//! Two cases render a fixed input through the real prepared effect and yield one `u32` result word
//! per output sample. Gate G5 (`tools/wasm-gates/tests/g5_native_corpus.rs`) compares each case
//! against [`G5_DIGESTS`] natively at every width the build has, and `tools/wasm-gate-corpus`
//! replays the identical cases under wasmtime, at the wasm simd128 backend, against these same
//! pins. Together they are the cross-target half of decision D5 for this effect: the reverb
//! renders the same bits in a browser as on a native host.
//!
//! # Why the input is built from integers
//!
//! Every sample is produced by a `xorshift64*` and exact conversions, so a target cannot differ on
//! the *input* and make a digest mismatch look like a numerics difference.
//!
//! # Widths
//!
//! The kernel is one body generic over [`Lane`]: the sixteen lines are the vector lanes inside one
//! instance, and the butterfly re-lays them out so every width computes the same `f32` values. A
//! case is therefore run at the width the caller names, and its digest must not depend on it.
//!
//! # Coverage
//!
//! Each case renders 16,384 frames at 48 kHz in 64-frame blocks: past the 12,129-sample history
//! cap, so both kernel instantiations (`FULL` false, then true) are in the digest. The automated
//! case moves `size` and `decay_ms` every 1,024 frames, with a second `size` event 64 frames
//! later on every other step, so a tap crossfade is queued behind a running one.
//!
//! # No NaN
//!
//! The determinism claim excludes NaN payloads. The loop is contracting and the input bounded, so
//! every sample is finite; `tests/determinism.rs` checks that rather than assuming it.

use effect_contract::{
    EffectProcessBlock, EffectQuality, InitialParameterValue, LinkMode, NativeEffectFactory,
    NativeEffectRegistry, ParameterChannel, PrepareEffectLimits, PrepareEffectRequest,
    PreparedAutomationSpan, PreparedPorts, PreparedSidechainPort,
};
use lane::Lane;

use crate::{
    DECAY, PARAMETER_COUNT, REVERB_DESCRIPTOR, REVERB_PARAMETERS, ReverbFactory, SIZE,
    prepare_reverb, state_sizes,
};

/// Sample rate every case renders at.
pub const SAMPLE_RATE: u32 = 48_000;

/// Frames each case renders.
pub const FRAMES: usize = 16_384;

/// Block length each case renders in. Fixed, because the digest describes the arithmetic; that it
/// does not depend on the block length is the unit test `partition_invariance_over_1_7_64_128_512`.
const BLOCK_FRAMES: usize = 64;

/// Frames between automation events in the automated case.
const EVENT_FRAMES: usize = 1_024;

/// Result words per case: the left output, then the right.
pub const POINTS: usize = FRAMES * 2;

/// Number of corpus cases.
pub const CASE_COUNT: usize = 2;

/// Human-readable name of each case, indexed by case number.
pub const CASE_NAMES: [&str; CASE_COUNT] = ["default_hall", "automated_size_decay"];

/// `xorshift64*` (Vigna 2016). Integer-only, so every target builds the same sequence.
struct Rng(u64);

impl Rng {
    const fn new(case: usize) -> Self {
        Self(0x9e37_79b9_7f4a_7c15 ^ ((case as u64).wrapping_mul(0x2545_f491_4f6c_dd1d) | 1))
    }

    fn next(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        (x.wrapping_mul(0x2545_f491_4f6c_dd1d) >> 32) as u32
    }

    /// A sample in `[-0.5, 0.5)` from the top sixteen bits, by exact conversion.
    fn sample(&mut self) -> f32 {
        f32::from((self.next() >> 16) as u16) * (1.0 / 65_536.0) - 0.5
    }
}

/// The initial parameter values of one case, in descriptor order.
fn initial_values(case: usize) -> [InitialParameterValue; PARAMETER_COUNT] {
    let mut values: [InitialParameterValue; PARAMETER_COUNT] =
        core::array::from_fn(|index| InitialParameterValue {
            parameter_index: index as u32,
            channel: ParameterChannel::Both,
            value: REVERB_PARAMETERS[index].default_value,
        });
    if case == 1 {
        // A short pre-delay, so the wet path starts early; partial diffusion, a narrowed image and
        // a dry/wet blend, so every output stage is engaged.
        values[0].value = 3.0;
        values[1].value = 900.0;
        values[2].value = 0.6;
        values[3].value = 9_000.0;
        values[4].value = 80.0;
        values[5].value = 0.45;
        values[6].value = 0.7;
        values[7].value = 0.8;
    }
    values
}

/// The automation events at block start `first_sample` of the automated case.
fn events(first_sample: usize) -> ([PreparedAutomationSpan; 2], usize) {
    let point = |parameter_index: usize, value| PreparedAutomationSpan {
        channel: ParameterChannel::Both,
        parameter_index: parameter_index as u32,
        sample: first_sample as u64,
        value,
    };
    let sizes = [2.0_f32, 0.25, 1.3, 0.7];
    let decays = [20_000.0_f32, 100.0, 4_000.0, 1_200.0];
    if first_sample > 0 && first_sample.is_multiple_of(EVENT_FRAMES) {
        let step = first_sample / EVENT_FRAMES;
        (
            [point(DECAY, decays[step % 4]), point(SIZE, sizes[step % 4])],
            2,
        )
    } else if first_sample > EVENT_FRAMES
        && (first_sample - BLOCK_FRAMES).is_multiple_of(2 * EVENT_FRAMES)
    {
        // Inside the running 128-sample crossfade: this size change queues behind it.
        ([point(SIZE, 1.1), point(SIZE, 1.1)], 1)
    } else {
        ([point(SIZE, 1.0), point(SIZE, 1.0)], 0)
    }
}

/// Runs one case at lane type `L` and writes [`POINTS`] result words: the left output, then the
/// right.
///
/// # Panics
///
/// Panics if `case >= CASE_COUNT`, if `out` is not [`POINTS`] words, or if the frozen prepare
/// request is rejected — all of which are corpus bugs, not runtime conditions.
pub fn run_case<L: Lane>(case: usize, out: &mut [u32]) {
    assert!(case < CASE_COUNT, "corpus case index out of range");
    assert_eq!(out.len(), POINTS, "corpus result buffer length");

    let values = initial_values(case);
    // Issue #1462 D1: the request carries the registry's tail-bound entry.
    let registry =
        NativeEffectRegistry::new([Box::new(ReverbFactory) as Box<dyn NativeEffectFactory>])
            .expect("the reverb is admitted");
    let tail_bound = registry
        .tail_bound(REVERB_DESCRIPTOR.id, SAMPLE_RATE, EffectQuality::Normal)
        .expect("a declared row");
    let (mut effect, _) = prepare_reverb(PrepareEffectRequest {
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
            maximum_total_state_bytes: state_sizes(SAMPLE_RATE).total().expect("state total"),
            maximum_scratch_bytes: 1,
            maximum_automation_spans_per_block: 16,
        },
        tail_bound,
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
        let (spans, count) = events(offset);
        let automation: &[PreparedAutomationSpan] = if case == 1 { &spans[..count] } else { &[] };
        let report = effect.process_at::<L>(
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
/// never from copying a vector width's or a wasm run's output).
///
/// A mismatch is never fixed by re-pinning. It means either the corpus changed or a target stopped
/// agreeing with the scalar oracle, and the second is what this gate exists to catch.
pub const G5_DIGESTS: [[u8; 32]; CASE_COUNT] = [
    // default_hall
    [
        0x74, 0xB0, 0xB5, 0x27, 0x6F, 0xE1, 0xE0, 0x7E, 0xD3, 0xB0, 0x2B, 0x8E, 0x75, 0x25, 0xA5,
        0xAF, 0x71, 0x01, 0x6F, 0x64, 0x6E, 0x07, 0xCC, 0x5C, 0xDE, 0xA7, 0x9D, 0xB2, 0xBD, 0x6C,
        0xCA, 0xA0,
    ],
    // automated_size_decay
    [
        0x96, 0x72, 0x20, 0x4A, 0x2D, 0xFA, 0x58, 0x8A, 0x83, 0x89, 0x15, 0xEB, 0xF0, 0x74, 0x2C,
        0x56, 0x06, 0xB7, 0xE2, 0x8D, 0x5A, 0xBD, 0x9B, 0xD6, 0x04, 0x42, 0xD1, 0xF3, 0xC9, 0x75,
        0x78, 0xF5,
    ],
];
