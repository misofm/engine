#![allow(clippy::disallowed_methods)] // D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! E1 — the polyphase kernel is **bit-identical** to the frozen 63-tap graph it replaces.
//!
//! The reference below is written from `.github/ISSUE_SPECS/BRIEFS/019` ("Exact 2x pipeline" and
//! the tap literals), not copied from the production code the audit measured: two zero-stuffed
//! 63-word rings, four full 31-tap convolutions per input sample, the frozen ascending tap order
//! and the frozen `a, b, c, e, y` output order. Issue #91 F2 claims that the polyphase form does
//! the work of two of those convolutions and produces the same bits; this test is that claim.
//!
//! What is deliberately *not* compared here is the parameter conversion: the reference is handed
//! the same linear gains the kernel gets, because `powf` and `math::db_to_gain_f32`
//! differ in the last bits by design (D6) and that difference belongs to `contract.rs`'s oracle
//! test, not to a proof about the filter graph.

use soft_clip::kernel::{SoftClipCoef, SoftClipHistory, SoftClipState, soft_clip_block};

/// The 63 taps, from the brief's literals. `h[62-k] = h[k]`; odd indices other than 31 are zero.
///
/// The literals are copied from `.github/ISSUE_SPECS/BRIEFS/019` character for character, decimal
/// digits and all, so that this table is genuinely the brief's and not a transcription of the
/// production one. `f32` rounds them; that is the point of the comparison.
#[allow(clippy::excessive_precision)]
fn brief_taps() -> [f32; 63] {
    const LEFT: [(usize, f32); 16] = [
        (0, 0.0),
        (2, 4.117_896_605_7e-5),
        (4, -1.843_658_683_4e-4),
        (6, 4.762_265_307_4e-4),
        (8, -9.890_398_941_9e-4),
        (10, 1.823_257_887_7e-3),
        (12, -3.110_171_528_5e-3),
        (14, 5.017_224_699_3e-3),
        (16, -7.761_147_804_6e-3),
        (18, 1.163_983_624_4e-2),
        (20, -1.710_855_774_6e-2),
        (22, 2.496_969_886_1e-2),
        (24, -3.690_094_873_3e-2),
        (26, 5.726_340_785_6e-2),
        (28, -1.021_490_171_6e-1),
        (30, 3.169_724_345_2e-1),
    ];
    let mut taps = [0.0_f32; 63];
    for (index, value) in LEFT {
        taps[index] = value;
        taps[62 - index] = value;
    }
    taps[31] = 5.0e-1;
    taps
}

/// The ascending nonzero tap indices, as the brief freezes them.
fn nonzero_taps() -> Vec<usize> {
    let taps = brief_taps();
    (0..63).filter(|index| taps[*index] != 0.0).collect()
}

/// The 63-tap, zero-stuffed realization of the brief, with no checks and no flush.
struct BriefSoftClip {
    taps: [f32; 63],
    order: Vec<usize>,
    interpolation: [f32; 63],
    decimation: [f32; 63],
    dry: [f32; 32],
    high_cursor: usize,
    dry_cursor: usize,
    drive: f32,
    output: f32,
    mix: f32,
}

impl BriefSoftClip {
    fn new(drive: f32, output: f32, mix: f32) -> Self {
        Self {
            taps: brief_taps(),
            order: nonzero_taps(),
            interpolation: [0.0; 63],
            decimation: [0.0; 63],
            dry: [0.0; 32],
            high_cursor: 0,
            dry_cursor: 0,
            drive,
            output,
            mix,
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        self.dry[self.dry_cursor] = input;
        let delayed = self.dry[(self.dry_cursor + 1) % 32];
        self.dry_cursor = (self.dry_cursor + 1) % 32;
        let doubled = 2.0_f32 * self.drive;
        let wet = self.stage(doubled * input);
        let _discarded = self.stage(0.0);
        if self.mix.to_bits() == 0.0_f32.to_bits() && self.output.to_bits() == 1.0_f32.to_bits() {
            return delayed;
        }
        let a = 1.0_f32 - self.mix;
        let b = a * delayed;
        let c = self.mix * wet;
        let e = b + c;
        self.output * e
    }

    fn stage(&mut self, input: f32) -> f32 {
        self.interpolation[self.high_cursor] = input;
        let interpolated = self.convolve(&self.interpolation);
        self.decimation[self.high_cursor] = cubic(interpolated);
        let output = self.convolve(&self.decimation);
        self.high_cursor = (self.high_cursor + 1) % 63;
        output
    }

    fn convolve(&self, history: &[f32; 63]) -> f32 {
        let mut accumulator = 0.0_f32;
        for tap in &self.order {
            let product = self.taps[*tap] * history[(self.high_cursor + 63 - *tap) % 63];
            accumulator += product;
        }
        accumulator
    }
}

/// The brief's cubic, in its branching form.
fn cubic(value: f32) -> f32 {
    if value <= -1.0 {
        -2.0_f32 / 3.0_f32
    } else if value >= 1.0 {
        2.0_f32 / 3.0_f32
    } else {
        let p0 = value * value;
        let p1 = p0 * value;
        let p2 = p1 / 3.0_f32;
        value - p2
    }
}

/// `xorshift64*`, so the corpus is the same on every host and every run.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    fn next_f32(&mut self) -> f32 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        let bits = (x.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 40) as u32;
        f32::from(((bits & 0xFFFF) as u16 >> 1) as i16) * (1.0 / 16_384.0) - 1.0
    }
}

/// One corpus case: a signal generator and the linear gains it runs under.
struct Case {
    name: &'static str,
    drive: f32,
    output: f32,
    mix: f32,
    samples: Vec<f32>,
}

fn corpus() -> Vec<Case> {
    const PER_CASE: usize = 100_000;
    let mut cases = Vec::new();
    for (name, drive_db) in [
        ("noise/-24dB", -24.0_f32),
        ("noise/0dB", 0.0),
        ("noise/18dB", 18.0),
        ("noise/36dB", 36.0),
    ] {
        let mut rng = Rng::new(0xA5A5_5A5A_1234_0001 ^ (drive_db.to_bits() as u64));
        cases.push(Case {
            name,
            drive: 10.0_f32.powf(drive_db * 0.05),
            output: 10.0_f32.powf(6.0 * 0.05),
            mix: 0.25,
            samples: (0..PER_CASE).map(|_| rng.next_f32()).collect(),
        });
    }
    cases.push(Case {
        name: "impulse",
        drive: 1.0,
        output: 1.0,
        mix: 1.0,
        samples: (0..PER_CASE)
            .map(|index| if index == 0 { 0.001 } else { 0.0 })
            .collect(),
    });
    cases.push(Case {
        name: "dc",
        drive: 10.0_f32.powf(18.0 * 0.05),
        output: 10.0_f32.powf(-6.0 * 0.05),
        mix: 0.5,
        samples: vec![0.5; PER_CASE],
    });
    cases.push(Case {
        name: "signed-zero-runs",
        drive: 10.0_f32.powf(12.0 * 0.05),
        output: 1.0,
        mix: 0.75,
        samples: (0..PER_CASE)
            .map(|index| match index % 4 {
                0 => -0.0,
                1 => 0.0,
                2 => -0.0,
                _ => 0.25,
            })
            .collect(),
    });
    cases.push(Case {
        name: "spikes",
        drive: 10.0_f32.powf(36.0 * 0.05),
        output: 10.0_f32.powf(-24.0 * 0.05),
        mix: 1.0,
        samples: (0..PER_CASE)
            .map(|index| match index % 7 {
                0 => 1.5,
                3 => -1.5,
                5 => 0.999_999,
                _ => 0.01,
            })
            .collect(),
    });
    cases.push(Case {
        name: "identity-select",
        drive: 10.0_f32.powf(24.0 * 0.05),
        output: 1.0,
        mix: 0.0,
        samples: (0..PER_CASE)
            .map(|index| if index % 3 == 0 { -0.0 } else { 0.3 })
            .collect(),
    });
    let mut rng = Rng::new(0x0F0F_1111_2222_3333);
    cases.push(Case {
        name: "noise/bypassed-mix-and-output",
        drive: 10.0_f32.powf(30.0 * 0.05),
        output: 10.0_f32.powf(3.0 * 0.05),
        mix: 1.0,
        samples: (0..PER_CASE).map(|_| rng.next_f32() * 0.05).collect(),
    });
    cases
}

#[test]
fn polyphase_matches_the_frozen_63_tap_graph() {
    let all = <f32 as lane::Lane>::zero();
    let bypass = <f32 as lane::Lane>::mask_not(<f32 as lane::Lane>::eq(all, all));
    let mut total = 0_usize;
    for case in corpus() {
        let mut reference = BriefSoftClip::new(case.drive, case.output, case.mix);
        let mut state = SoftClipState::from_lanes(case.drive, case.output, case.mix);
        let mut history = SoftClipHistory::new(1);
        let coefficients = SoftClipCoef {
            drive_step: 0.0,
            output_step: 0.0,
            mix_step: 0.0,
            bypass,
        };
        // Split into odd-sized blocks so the block boundary is exercised too.
        let mut block = Vec::new();
        let mut expected = Vec::new();
        for (index, sample) in case.samples.iter().copied().enumerate() {
            expected.push(reference.process(sample));
            block.push(sample);
            if block.len() == 37 || index + 1 == case.samples.len() {
                let frames = block.len();
                soft_clip_block::<f32>(&mut block, frames, &coefficients, &mut state, &mut history);
                for (offset, actual) in block.iter().copied().enumerate() {
                    let want = expected[offset];
                    assert_eq!(
                        actual.to_bits(),
                        want.to_bits(),
                        "case {} sample {}: polyphase {actual:e} vs 63-tap {want:e}",
                        case.name,
                        index + 1 - frames + offset
                    );
                }
                total += frames;
                block.clear();
                expected.clear();
            }
        }
    }
    assert!(total >= 900_000, "corpus shrank to {total} samples");
}
