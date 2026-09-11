//! The cross-target determinism corpus for the builtin chain.
//!
//! Each case renders eight independent tracks through the production stages and yields the audio
//! lane by lane, followed by the report counters. `tests/determinism.rs` hashes each case at every
//! width and compares it against [`BUILTINS_DIGESTS`]; `tools/wasm-gates` replays the
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

use lane::Lane;

use crate::{
    BuiltinChain, BuiltinLaneSelector, BuiltinParameters, ChannelParameters, FaderStage,
    InputStage, Matrix2x2, MatrixStage, PreparedInputTrack,
};

/// Independent tracks in every case; a multiple of the widest backend.
pub const LANES: usize = 8;

/// Frames per case: long enough for the filters to settle and the 129-frame ramp to snap.
pub const FRAMES: usize = 256;

/// Number of corpus cases.
pub const CASE_COUNT: usize = 10;

/// Human-readable name of each case, indexed by case number.
pub const CASE_NAMES: [&str; CASE_COUNT] = [
    "input_stage/noise",
    "input_stage/impulse",
    "input_stage/subnormal",
    "input_stage/nonfinite",
    "fader_mute",
    "matrix_ramp",
    // Issue #210 phase 3. The two ramping input bodies had no cross-target coverage at all until
    // these landed: every case above renders the *settled* chain, and `input_chain_ramp_block`
    // and `input_chain_ramp_block_mono` are the only kernels in this crate a wasm build could
    // have got wrong without moving a single pin.
    "input_stage/trim_ramp",
    "input_stage/trim_ramp_mono",
    "input_stage/identity_sections",
    "input_stage/mixed_sections",
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

/// The prepared parameters of one lane with the two channels **designed identically**.
///
/// The collapse-eligible shape, and the one case 7 needs: `lane_channel_symmetry` is a bitwise
/// comparison of every designed word, so a bank built from [`lane_parameters`] -- whose whole point
/// is that the two channels differ -- can never be dispatched one-plane. Every lane still differs
/// from every other lane, which is what keeps a cross-lane leak visible.
fn symmetric_lane_parameters(lane: usize) -> BuiltinParameters {
    let index = lane as f32;
    let channel = ChannelParameters {
        polarity_invert: lane % 2 == 1,
        trim_db: index - 3.0,
        hpf_hz: 60.0 + index * 17.0,
        lpf_hz: 1_500.0 + index * 211.0,
        fader_db: 2.0 - index,
        muted: lane % 5 == 3,
    };
    BuiltinParameters {
        left: channel,
        right: channel,
        matrix: Matrix2x2::IDENTITY,
        smoothing_samples: 129,
    }
}

/// Settled section-elision cases. Zero-dB trim gives an independently exact signed
/// unit gain; lane/channel polarity and the enabled cutoff still differ per lane.
fn elided_lane_parameters(case: usize, lane: usize) -> BuiltinParameters {
    let mut parameters = BuiltinParameters::default();
    parameters.left.polarity_invert = lane % 2 == 1;
    parameters.right.polarity_invert = lane % 3 == 1;
    if case == 9 {
        // Identity before a real section on left, after a real section on right.
        parameters.left.lpf_hz = 3_000.0 + 200.0 * lane as f32;
        parameters.right.hpf_hz = 600.0 + 100.0 * lane as f32;
    }
    parameters
}

/// The per-lane input signal of one case.
fn lane_signal(case: usize, lane: usize, channel: usize) -> Vec<f32> {
    let mut rng = Rng::new(0x8500_0000 ^ (case as u64) << 16 ^ (lane as u64) << 8 ^ channel as u64);
    (0..FRAMES)
        .map(|frame| match case {
            0 | 4 | 5 | 6 | 7 => rng.next_sample(),
            8 | 9 => match frame % 32 {
                0 => -0.0,
                1 => 0.0,
                2 => f32::NAN,
                3 => f32::INFINITY,
                4 => f32::NEG_INFINITY,
                5 => 1.0e30,
                6 => f32::from_bits(1),
                7 => -f32::from_bits(1),
                _ => rng.next_sample(),
            },
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
            BuiltinChain::new(
                48_000,
                if case >= 8 {
                    elided_lane_parameters(case, lane)
                } else {
                    lane_parameters(lane)
                },
            )
            .expect("corpus parameters prepare")
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
            8 | 9 => {
                let tracks: Vec<_> = (first..first + L::WIDTH)
                    .map(|lane| prepared[lane].input.stage.lane_track(0))
                    .collect();
                let mut stage = InputStage::<L>::new(&tracks);
                let report = process_elided_case(&mut stage, &mut left_block, &mut right_block);
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
            6 | 7 => {
                // Issue #210 phase 3: the ramping input chain, dual (case 6) and collapsed
                // (case 7).
                //
                // Every value is keyed by the *global* lane, so a group's contents do not depend
                // on the width. The windows are per lane and coprime with the split below, so each
                // lane's ramp settles on a different frame and some settle *inside* the second
                // block -- which is what puts the D11 snap, the countdown reload and the tail on
                // both sides of a block boundary and inside the digest.
                // Case 7's tracks are **symmetric**: a collapsed body is dispatched only under a
                // witness that compares every designed word between the two channels, so a
                // one-plane case built on this corpus's deliberately asymmetric `lane_parameters`
                // would be describing a dispatch the engine cannot make. Case 6 keeps the
                // asymmetric set, which is what a dual block is for.
                let tracks: Vec<PreparedInputTrack> = (first..first + L::WIDTH)
                    .map(|lane| {
                        if case == 6 {
                            prepared[lane].input.stage.lane_track(0)
                        } else {
                            BuiltinChain::new(48_000, symmetric_lane_parameters(lane))
                                .expect("corpus parameters prepare")
                                .input
                                .stage
                                .lane_track(0)
                        }
                    })
                    .collect();
                let mut stage = InputStage::<L>::new(&tracks);
                for slot in 0..L::WIDTH {
                    let lane = first + slot;
                    let window = 37_u32 + 11 * lane as u32;
                    // Case 7 is the collapsed body, which is only ever dispatched under a witness
                    // that requires the two channels' ramp records to agree -- so its retargets
                    // are `Both`, exactly as a real collapsed chain's must be. Case 6 rides the
                    // two lanes apart, which is what a dual block is for.
                    let (trim_channels, flip_channels) = if case == 6 {
                        (BuiltinLaneSelector::Left, BuiltinLaneSelector::Right)
                    } else {
                        (BuiltinLaneSelector::Both, BuiltinLaneSelector::Both)
                    };
                    let gain = crate::db_gain(6.0 - 2.5 * lane as f32).expect("corpus trim gain");
                    stage.set_trim_db(slot, trim_channels, gain, window);
                    // A polarity flip on some lanes: the same coefficient, retargeted through
                    // zero, which is the phase's whole declick story and is arithmetic no other
                    // case reaches.
                    if lane % 3 != 2 {
                        stage.set_polarity_invert(slot, flip_channels, lane % 2 == 0, window + 5);
                    }
                }
                // Three blocks, so a ramp crosses two boundaries and the countdown is reloaded
                // from the authoritative `u32` twice.
                let first_split = (FRAMES / 4) * L::WIDTH;
                let second_split = (FRAMES / 2) * L::WIDTH;
                if case == 6 {
                    let report = stage.process(
                        &mut left_block[..first_split],
                        &mut right_block[..first_split],
                        FRAMES / 4,
                    );
                    counters[0] += report.sanitized_input as f32;
                    let report = stage.process(
                        &mut left_block[first_split..second_split],
                        &mut right_block[first_split..second_split],
                        FRAMES / 4,
                    );
                    counters[0] += report.sanitized_input as f32;
                    let report = stage.process(
                        &mut left_block[second_split..],
                        &mut right_block[second_split..],
                        FRAMES - FRAMES / 2,
                    );
                    counters[0] += report.sanitized_input as f32;
                } else {
                    for (offset, end, frames) in [
                        (0, first_split, FRAMES / 4),
                        (first_split, second_split, FRAMES / 4),
                        (second_split, left_block.len(), FRAMES - FRAMES / 2),
                    ] {
                        let report = stage.process_mono(&mut left_block[offset..end], frames);
                        counters[0] += report.sanitized_input as f32;
                        counters[1] += report.recovered_left_state as f32;
                        counters[2] += report.recovered_right_state as f32;
                    }
                    // The seam. A collapsed chain duplicates its one plane before anything
                    // downstream reads the other, so the case publishes what the strip publishes
                    // rather than the ungathered scratch.
                    right_block.copy_from_slice(&left_block);
                }
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

/// The shared two-block render used by both appended cases and their dispatch witness.
fn process_elided_case<L: Lane>(
    stage: &mut InputStage<L>,
    left: &mut [f32],
    right: &mut [f32],
) -> crate::BuiltinProcessReport {
    // An odd split retains recursive state across the block boundary.
    let split = 73 * L::WIDTH;
    let mut total = crate::BuiltinProcessReport::default();
    for (start, end, frames) in [(0, split, 73), (split, left.len(), FRAMES - 73)] {
        let report = stage.process(&mut left[start..end], &mut right[start..end], frames);
        total.sanitized_input += report.sanitized_input;
        total.recovered_left_state += report.recovered_left_state;
        total.recovered_right_state += report.recovered_right_state;
    }
    total
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
    // input_stage/trim_ramp (#210 phase 3), pinned from the scalar `Lane` instantiation
    [
        0x02, 0x37, 0xec, 0xaf, 0x96, 0x40, 0x27, 0xaa, 0x10, 0xb9, 0x74, 0xe6, 0xf3, 0xb6, 0x9a,
        0x8e, 0x8c, 0x15, 0x8e, 0x51, 0x2b, 0xed, 0xe7, 0xd9, 0x28, 0x5d, 0xf7, 0x55, 0xc9, 0x04,
        0x27, 0xbe,
    ],
    // input_stage/trim_ramp_mono (#210 phase 3), pinned from the scalar `Lane` instantiation
    [
        0x5d, 0xe8, 0xcd, 0xe7, 0x32, 0x31, 0x77, 0xdc, 0x04, 0xb5, 0xbf, 0x6c, 0x76, 0xf5, 0xd7,
        0x76, 0x44, 0x18, 0x92, 0x02, 0xf3, 0x4d, 0x16, 0xce, 0x9a, 0xe2, 0xaa, 0xc8, 0x9b, 0x56,
        0xff, 0xaa,
    ],
    // Derived only from the independent scalar expectation below (issue #213).
    [
        0xd8, 0xd7, 0x81, 0xde, 0x39, 0x22, 0x04, 0xec, 0x36, 0xb6, 0x0b, 0x0b, 0x70, 0xa7, 0x68,
        0x6e, 0x76, 0x01, 0x29, 0xc3, 0x39, 0x7a, 0x4a, 0x5d, 0xff, 0xe5, 0xda, 0x27, 0x93, 0x49,
        0xc9, 0x22,
    ],
    [
        0x3b, 0x8b, 0x3c, 0x72, 0xf2, 0xcc, 0x22, 0x74, 0x20, 0x6e, 0x32, 0xe3, 0x68, 0xde, 0x90,
        0x56, 0x5d, 0xa6, 0xf7, 0x74, 0x61, 0x6b, 0xb8, 0x5a, 0xff, 0xd4, 0x68, 0x08, 0xdb, 0xfe,
        0xb4, 0xce,
    ],
];

#[cfg(test)]
mod elision_tests {
    use super::*;
    use core::cell::Cell;
    use dsp_reference::{ReferenceRetainedTptF32, ReferenceTptOutput};
    use lane::{Simd4, Simd8};
    use sha2::{Digest, Sha256};

    fn digest(values: &[f32]) -> [u8; 32] {
        let mut hash = Sha256::new();
        for value in values {
            hash.update(value.to_bits().to_le_bytes());
        }
        hash.finalize().into()
    }

    /// Independent scalar expectation: signed unit trim, input sanitisation, then
    /// the identity map x + +0 and/or the existing equation-derived scalar TPT twin.
    /// No production preparation, Lane arithmetic or corpus output supplies this oracle.
    fn expected(case: usize, omit_identity_add: bool, bypass_real: bool) -> Vec<f32> {
        let mut out = Vec::with_capacity(LANES * FRAMES * 2 + 3);
        let mut sanitized = 0;
        for channel in 0..2 {
            for lane in 0..LANES {
                let sign = if (channel == 0 && lane % 2 == 1) || (channel == 1 && lane % 3 == 1) {
                    -1.0
                } else {
                    1.0
                };
                let (cutoff, kind) = if channel == 0 {
                    (3_000.0 + 200.0 * lane as f32, ReferenceTptOutput::LowPass)
                } else {
                    (600.0 + 100.0 * lane as f32, ReferenceTptOutput::HighPass)
                };
                let mut filter =
                    ReferenceRetainedTptF32::conditioned_butterworth(48_000, cutoff, kind).unwrap();
                for x in lane_signal(case, lane, channel) {
                    let clean = if !x.is_finite() || x.abs() >= 1.0e30 {
                        sanitized += 1;
                        0.0
                    } else {
                        x
                    };
                    let mut y = clean * sign;
                    if !omit_identity_add && (case == 8 || channel == 0) {
                        y += 0.0;
                    }
                    if case == 9 && !bypass_real {
                        y = f32::from_bits(filter.process(y).output_bits);
                    }
                    if !omit_identity_add && case == 9 && channel == 1 {
                        y += 0.0;
                    }
                    out.push(y);
                }
            }
        }
        out.extend([sanitized as f32, 0.0, 0.0]);
        out
    }

    #[test]
    fn derive_new_scalar_expectations() {
        for case in 8..10 {
            let oracle = expected(case, false, false);
            // Print only independently derived expectations, never production/SIMD output.
            println!("{} {:?}", CASE_NAMES[case], digest(&oracle));
            assert_eq!(
                digest(&oracle),
                BUILTINS_DIGESTS[case],
                "independently derived new pin"
            );
            assert_eq!(oracle[LANES * FRAMES * 2..], [512.0, 0.0, 0.0]);
            assert_eq!(
                case_values::<f32>(case)
                    .iter()
                    .map(|v| v.to_bits())
                    .collect::<Vec<_>>(),
                oracle.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
            );
        }
        assert_ne!(
            digest(&expected(8, false, false)),
            digest(&expected(8, true, false)),
            "signed-zero input must detect dropping the identity add"
        );
        assert_ne!(
            digest(&expected(9, false, false)),
            digest(&expected(9, false, true)),
            "mixed input must detect bypassing the real sections"
        );
    }

    thread_local! { static FMAS: Cell<usize> = const { Cell::new(0) }; }

    /// Test-only observation of actual arithmetic in the generic production kernel.
    /// Every value/mask operation delegates unchanged to the real backend.
    #[derive(Clone, Copy)]
    struct Observed<L>(L);

    macro_rules! unary {
        ($($name:ident),*) => { $(fn $name(self) -> Self { Self(self.0.$name()) })* };
    }
    macro_rules! binary {
        ($($name:ident),*) => { $(fn $name(self, rhs: Self) -> Self { Self(self.0.$name(rhs.0)) })* };
    }
    macro_rules! comparison {
        ($($name:ident),*) => { $(fn $name(self, rhs: Self) -> Self::Mask { self.0.$name(rhs.0) })* };
    }
    impl<L: Lane> Lane for Observed<L> {
        const WIDTH: usize = L::WIDTH;
        const SVF_CASCADE_DEPTH: usize = L::SVF_CASCADE_DEPTH;
        type Mask = L::Mask;
        fn splat(x: f32) -> Self {
            Self(L::splat(x))
        }
        fn zero() -> Self {
            Self(L::zero())
        }
        fn load(src: &[f32]) -> Self {
            Self(L::load(src))
        }
        fn store(self, dst: &mut [f32]) {
            self.0.store(dst);
        }
        fn store_bits(self, dst: &mut [u32]) {
            self.0.store_bits(dst);
        }
        unary!(sqrt, neg, abs, floor);
        binary!(add, sub, mul, div);
        comparison!(lt, le, gt, ge, eq);
        fn fma(self, b: Self, c: Self) -> Self {
            FMAS.with(|count| count.set(count.get() + 1));
            Self(self.0.fma(b.0, c.0))
        }
        fn mask_and(a: Self::Mask, b: Self::Mask) -> Self::Mask {
            L::mask_and(a, b)
        }
        fn mask_or(a: Self::Mask, b: Self::Mask) -> Self::Mask {
            L::mask_or(a, b)
        }
        fn mask_not(a: Self::Mask) -> Self::Mask {
            L::mask_not(a)
        }
        fn mask_any(a: Self::Mask) -> bool {
            L::mask_any(a)
        }
        fn select(m: Self::Mask, a: Self, b: Self) -> Self {
            Self(L::select(m, a.0, b.0))
        }
        fn andnot(self, m: Self::Mask) -> Self {
            Self(self.0.andnot(m))
        }
        fn exp2_int(n: Self) -> Self {
            Self(L::exp2_int(n.0))
        }
        fn exp2_int_in_range(n: Self) -> Self {
            Self(L::exp2_int_in_range(n.0))
        }
        fn frexp(self) -> (Self, Self) {
            let (m, e) = self.0.frexp();
            (Self(m), Self(e))
        }
    }

    fn dispatch_at<L: Lane>(case: usize) {
        let signals: [Vec<_>; 2] =
            core::array::from_fn(|ch| (0..LANES).map(|lane| lane_signal(case, lane, ch)).collect());
        let oracle = expected(case, false, false);
        for first in (0..LANES).step_by(L::WIDTH) {
            let tracks: Vec<_> = (first..first + L::WIDTH)
                .map(|lane| {
                    BuiltinChain::new(48_000, elided_lane_parameters(case, lane))
                        .unwrap()
                        .input
                        .stage
                        .lane_track(0)
                })
                .collect();
            for force_full in [false, true] {
                let mut stage = InputStage::<Observed<L>>::new(&tracks);
                assert!(!stage.ramping, "settled corpus must enter elided dispatch");
                assert_eq!(
                    stage.plan.elided,
                    if case == 8 {
                        [[true, true]; 2]
                    } else {
                        [[true, false], [false, true]]
                    }
                );
                if force_full {
                    stage.plan = crate::InputChainPlan::NONE;
                }
                let mut left = interleave::<L>(&signals[0], first);
                let mut right = interleave::<L>(&signals[1], first);
                FMAS.with(|count| count.set(0));
                let report = process_elided_case(&mut stage, &mut left, &mut right);
                let actual = FMAS.with(Cell::get);
                // Each executed section performs two recurrence and two mix multiply-adds.
                let sections = if force_full {
                    4
                } else if case == 8 {
                    0
                } else {
                    2
                };
                assert_eq!(
                    actual,
                    sections * 4 * FRAMES,
                    "case {case}, width {}, force_full {force_full}",
                    L::WIDTH
                );
                assert_eq!(report.sanitized_input, (64 * L::WIDTH) as u64);
                assert_eq!(
                    (report.recovered_left_state, report.recovered_right_state),
                    (0, 0)
                );
                for (ch, block) in [&left, &right].into_iter().enumerate() {
                    for slot in 0..L::WIDTH {
                        for frame in 0..FRAMES {
                            assert_eq!(
                                block[frame * L::WIDTH + slot].to_bits(),
                                oracle[(ch * LANES + first + slot) * FRAMES + frame].to_bits()
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn appended_cases_execute_identity_and_mixed_dispatch_with_full_controls() {
        for case in 8..10 {
            dispatch_at::<f32>(case);
            dispatch_at::<Simd4>(case);
            dispatch_at::<Simd8>(case);
        }
    }
}
