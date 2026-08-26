//! The cross-target determinism corpus for the builtin chain.
//!
//! Each case renders eight independent tracks through the production stages and yields the audio
//! lane by lane, followed by the report counters. `tests/determinism.rs` hashes each case at every
//! width and compares it against [`BUILTINS_DIGESTS`]; `tools/miso-engine-wasm-gates` replays the
//! identical corpus under `wasmtime`, at the wasm `Simd4` and scalar backends, against these same
//! pins. That is the whole of the cross-target claim for this crate: the filter a track rides,
//! the fader under it and the matrix after it produce the same bits in a browser as on a native
//! host (master plan #83 D5).
//!
//! # Why the lanes are independent
//!
//! A recurrence at width `W` runs `W` interleaved sub-sequences, so a digest taken over the AoSoA
//! buffer would describe the layout rather than the arithmetic and would differ per width by
//! construction. Every case here gives each of the eight lanes its own parameters and its own
//! signal, renders in groups of `W`, and reads the results back **lane-major** — so one pin serves
//! every backend, which is what the wasm leg needs.
//!
//! # No NaN
//!
//! The determinism claim excludes NaN payloads, because wasm canonicalises them. The non-finite
//! case deliberately feeds NaN and infinity *in*: they are sanitised to `+0.0` at the input stage
//! before any arithmetic, so every output word is finite. `tests/determinism.rs` checks that.

use miso_engine_lane::Lane;

use crate::{
    BuiltinChain, BuiltinParameters, ChannelParameters, FaderStage, InputStage, Matrix2x2,
    MatrixStage, PreparedInputTrack,
};

/// Independent tracks in every case; a multiple of the widest backend.
pub const LANES: usize = 8;

/// Frames per case: long enough for the filters to settle and the 129-frame ramp to snap.
pub const FRAMES: usize = 256;

/// Number of corpus cases.
pub const CASE_COUNT: usize = 6;

/// Human-readable name of each case, indexed by case number.
pub const CASE_NAMES: [&str; CASE_COUNT] = [
    "input_stage/noise",
    "input_stage/impulse",
    "input_stage/subnormal",
    "input_stage/nonfinite",
    "fader_mute",
    "matrix_ramp",
];

/// `xorshift64*`. Integer-only, so every target builds the same input sequence.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(if seed == 0 {
            0x9E37_79B9_7F4A_7C15
        } else {
            seed
        })
    }

    fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        (x.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 32) as u32
    }

    /// A finite sample in `[-1, 1)`, built by an exact integer conversion.
    fn next_sample(&mut self) -> f32 {
        (self.next_u32() as i32 as f32) / 2_147_483_648.0
    }
}

/// The prepared parameters of one lane: every lane differs, so a cross-lane leak cannot hide.
fn lane_parameters(lane: usize) -> BuiltinParameters {
    let index = lane as f32;
    BuiltinParameters {
        left: ChannelParameters {
            polarity_invert: lane % 2 == 1,
            trim_db: index - 3.0,
            hpf_hz: 60.0 + index * 17.0,
            lpf_hz: 1_500.0 + index * 211.0,
            fader_db: 2.0 - index,
            muted: lane % 5 == 3,
        },
        right: ChannelParameters {
            polarity_invert: lane % 3 == 1,
            trim_db: 1.0 - index,
            hpf_hz: 90.0 + index * 23.0,
            lpf_hz: 2_400.0 + index * 173.0,
            fader_db: index - 2.0,
            muted: lane % 7 == 6,
        },
        matrix: Matrix2x2::IDENTITY,
        smoothing_samples: 129,
    }
}

/// The per-lane input signal of one case.
fn lane_signal(case: usize, lane: usize, channel: usize) -> Vec<f32> {
    let mut rng = Rng::new(0x8500_0000 ^ (case as u64) << 16 ^ (lane as u64) << 8 ^ channel as u64);
    (0..FRAMES)
        .map(|frame| match case {
            0 | 4 | 5 => rng.next_sample(),
            1 => f32::from(u8::from(frame == lane + channel)),
            2 => f32::from_bits(1 + (rng.next_u32() & 0x007F_FFFF)),
            _ => match (frame + lane) % 8 {
                0 => f32::NAN,
                1 => f32::INFINITY,
                2 => f32::NEG_INFINITY,
                // Straddles the D7 threshold: `5e30` is sanitised, `9e29` is not, so the
                // constant itself is load-bearing rather than merely present.
                3 => 5.0e30,
                4 => 9.0e29,
                5 => f32::from_bits(1),
                _ => rng.next_sample(),
            },
        })
        .collect()
}

/// Interleaves one group of `W` lanes into an AoSoA block.
fn interleave<L: Lane>(lanes: &[Vec<f32>], first: usize) -> Vec<f32> {
    let mut block = vec![0.0_f32; FRAMES * L::WIDTH];
    for frame in 0..FRAMES {
        for lane in 0..L::WIDTH {
            block[frame * L::WIDTH + lane] = lanes[first + lane][frame];
        }
    }
    block
}

/// Reads one lane back out of an AoSoA block.
fn deinterleave<L: Lane>(block: &[f32], lane: usize, out: &mut Vec<f32>) {
    for frame in 0..FRAMES {
        out.push(block[frame * L::WIDTH + lane]);
    }
}

/// Runs one corpus case at one width and returns its result words, lane-major.
///
/// # Panics
///
/// Panics if `case >= CASE_COUNT`.
#[must_use]
pub fn case_values<L: Lane>(case: usize) -> Vec<f32> {
    assert!(case < CASE_COUNT, "corpus case index out of range");
    let left: Vec<Vec<f32>> = (0..LANES).map(|lane| lane_signal(case, lane, 0)).collect();
    let right: Vec<Vec<f32>> = (0..LANES).map(|lane| lane_signal(case, lane, 1)).collect();
    let prepared: Vec<BuiltinChain> = (0..LANES)
        .map(|lane| {
            BuiltinChain::new(48_000, lane_parameters(lane)).expect("corpus parameters prepare")
        })
        .collect();

    // Left first for all eight lanes, then right, then the counters: a group is `W` lanes wide,
    // so interleaving the two channels group by group would order the words by width.
    let mut left_out = Vec::with_capacity(LANES * FRAMES);
    let mut right_out = Vec::with_capacity(LANES * FRAMES);
    // The counters are summed over the groups, not appended per group: a group is `W` lanes wide,
    // so appending them would make the *length* of the digest input depend on the width.
    let mut counters = [0.0_f32; 3];
    let mut first = 0;
    while first < LANES {
        let mut left_block = interleave::<L>(&left, first);
        let mut right_block = interleave::<L>(&right, first);
        match case {
            0..=3 => {
                let tracks: Vec<PreparedInputTrack> = (first..first + L::WIDTH)
                    .map(|lane| prepared[lane].input.stage.lane_track(0))
                    .collect();
                let mut stage = InputStage::<L>::new(&tracks);
                let report = stage.process(&mut left_block, &mut right_block, FRAMES);
                counters[0] += report.sanitized_input as f32;
                counters[1] += report.recovered_left_state as f32;
                counters[2] += report.recovered_right_state as f32;
            }
            4 => {
                let faders: Vec<_> = (first..first + L::WIDTH)
                    .map(|lane| {
                        let parameters = lane_parameters(lane);
                        (
                            crate::FaderLane {
                                gain: crate::db_gain(parameters.left.fader_db)
                                    .expect("corpus fader gain"),
                                muted: parameters.left.muted,
                            },
                            crate::FaderLane {
                                gain: crate::db_gain(parameters.right.fader_db)
                                    .expect("corpus fader gain"),
                                muted: parameters.right.muted,
                            },
                        )
                    })
                    .collect();
                let mut stage = FaderStage::<L>::new(&faders);
                stage.process(&mut left_block, &mut right_block, FRAMES);
            }
            _ => {
                // Every value is keyed by the *global* lane, so a group's contents do not depend
                // on the width -- which is the whole point of the corpus.
                let matrices: Vec<_> = (first..first + L::WIDTH)
                    .map(|lane| (Matrix2x2::IDENTITY, 129_u32 + lane as u32))
                    .collect();
                let mut stage = MatrixStage::<L>::new(&matrices);
                for slot in 0..L::WIDTH {
                    let lane = first + slot;
                    stage
                        .set_target(
                            slot,
                            Matrix2x2 {
                                ll: 0.25 * lane as f32 - 0.75,
                                lr: 0.5,
                                rl: -0.5,
                                rr: 0.75 - 0.125 * lane as f32,
                            },
                        )
                        .expect("corpus matrix target");
                }
                // Two blocks, so the ramp crosses a block boundary and some lanes snap inside it.
                let split = (FRAMES / 3) * L::WIDTH;
                stage.process(
                    &mut left_block[..split],
                    &mut right_block[..split],
                    FRAMES / 3,
                );
                stage.process(
                    &mut left_block[split..],
                    &mut right_block[split..],
                    FRAMES - FRAMES / 3,
                );
            }
        }
        for lane in 0..L::WIDTH {
            deinterleave::<L>(&left_block, lane, &mut left_out);
            deinterleave::<L>(&right_block, lane, &mut right_out);
        }
        first += L::WIDTH;
    }
    let mut output = left_out;
    output.append(&mut right_out);
    output.extend(counters);
    output
}

/// SHA-256 of each case's result words, little-endian by lane, pinned from the scalar `Lane`
/// instantiation (master plan §8: never from a vector or wasm run).
pub const BUILTINS_DIGESTS: [[u8; 32]; CASE_COUNT] = [
    // input_stage/noise
    [
        0xb9, 0xc1, 0x22, 0xdc, 0x09, 0x54, 0xcf, 0xb3, 0x45, 0x1b, 0x53, 0xd5, 0x0b, 0xe1, 0x7c,
        0x77, 0xfa, 0xf1, 0xa0, 0xae, 0x4a, 0xb3, 0x4b, 0xcd, 0x6f, 0x18, 0x61, 0x14, 0x16, 0x7e,
        0x1e, 0xfd,
    ],
    // input_stage/impulse
    [
        0x2d, 0x3e, 0xea, 0xde, 0xeb, 0x6e, 0xcf, 0xb2, 0x0f, 0xc6, 0xb4, 0x30, 0x5f, 0x34, 0xb4,
        0x21, 0xd1, 0x8f, 0x8c, 0x46, 0xbd, 0x03, 0x74, 0x50, 0x16, 0x5f, 0x34, 0x7a, 0x36, 0xf9,
        0x65, 0xb7,
    ],
    // input_stage/subnormal
    [
        0xfe, 0xdb, 0x98, 0xcc, 0x41, 0xa3, 0xca, 0x77, 0x18, 0x7f, 0xb3, 0x75, 0x9e, 0x37, 0x96,
        0x56, 0xc6, 0x34, 0x6f, 0x7e, 0x21, 0x2c, 0xbe, 0xc8, 0x11, 0x34, 0x43, 0x9a, 0xc6, 0x6d,
        0xd0, 0x09,
    ],
    // input_stage/nonfinite
    [
        0x6e, 0x7b, 0xab, 0x46, 0x7d, 0xc7, 0x11, 0xe0, 0xb3, 0x23, 0xaa, 0xee, 0xf3, 0x56, 0x35,
        0x68, 0x59, 0x62, 0x24, 0x6c, 0xed, 0x67, 0xa1, 0x58, 0x5d, 0x97, 0xe6, 0x5c, 0xe3, 0x0c,
        0x25, 0x4a,
    ],
    // fader_mute
    [
        0x04, 0x83, 0x75, 0xc0, 0x65, 0x36, 0xd3, 0x5b, 0x45, 0xa5, 0xbc, 0x8d, 0xf0, 0xe0, 0x4f,
        0x82, 0x76, 0x05, 0x4c, 0x18, 0xc4, 0x10, 0xb2, 0x12, 0x0c, 0xf9, 0x7c, 0x59, 0xfc, 0xe5,
        0xfe, 0x29,
    ],
    // matrix_ramp
    [
        0x8b, 0x41, 0x66, 0x6e, 0xbe, 0xcf, 0x76, 0x06, 0x1e, 0x91, 0x27, 0x88, 0xa2, 0x1c, 0x0a,
        0xcf, 0x87, 0x1f, 0xe0, 0x68, 0x5b, 0xdf, 0x62, 0x66, 0x3b, 0x8c, 0x11, 0x30, 0x87, 0xeb,
        0x2a, 0xee,
    ],
];
