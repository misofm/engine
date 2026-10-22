#![allow(missing_docs)]

//! Issue #890: the fused ramping fader/mute/matrix pass against the three kernels it replaces.

use lane::{
    CanonicalFpEnv, Lane,
    kernels::builtins::{
        FaderMatrixArm, GainMuteRamp, Matrix2x2Coef, Matrix2x2Ramp, fader_matrix_ramp_block,
        gain_mute_ramp_block, mask_from_flags, matrix2x2_block, matrix2x2_block_without_identity,
        matrix2x2_ramp_block, no_lanes,
    },
};

const WIDTH: usize = 8;

/// SplitMix64: a seeded, portable word stream.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }

    /// A finite value in `[-2, 2)`, or one of the exact words a fader or matrix meets often.
    fn value(&mut self) -> f32 {
        match self.below(8) {
            0 => 0.0,
            1 => -0.0,
            2 => 1.0,
            _ => (self.next() >> 40) as f32 / (1_u64 << 22) as f32 - 2.0,
        }
    }

    /// A finite sample: signed zeros often, otherwise a sign, mantissa and exponent in
    /// `2^-20 ..= 2^9`.
    fn sample(&mut self) -> f32 {
        match self.below(6) {
            0 => -0.0,
            1 => 0.0,
            _ => {
                let bits = self.next();
                let sign = (bits as u32) & 0x8000_0000;
                let mantissa = (bits >> 8) as u32 & 0x007f_ffff;
                let exponent = (127 - 20 + ((bits >> 40) % 30) as i32) as u32;
                f32::from_bits(sign | (exponent << 23) | mantissa)
            }
        }
    }

    /// A ramp countdown relative to `frames`: zero, inside the block, exactly the block, or past
    /// it.
    fn remaining(&mut self, frames: usize) -> usize {
        match self.below(4) {
            0 => 0,
            1 if frames > 1 => 1 + self.below(frames as u64 - 1) as usize,
            1 | 2 => frames,
            _ => frames + 1 + self.below(12) as usize,
        }
    }

    /// One lane's `(current, target, step, remaining)`: a `(target - start) / n` step, or an
    /// arbitrary one on one draw in eight. A settled lane holds its target with a zero step.
    fn ramp_lane(&mut self, frames: usize, ramping: bool) -> (f32, f32, f32, f32) {
        let target = self.value();
        let n = if ramping { self.remaining(frames) } else { 0 };
        if n == 0 {
            return (target, target, 0.0, 0.0);
        }
        let start = self.value();
        let step = if self.below(8) == 0 {
            self.value() * 0.25
        } else {
            (target - start) / n as f32
        };
        (start, target, step, n as f32)
    }
}

fn words<L: Lane>(value: L) -> Vec<u32> {
    let mut out = [0.0_f32; WIDTH];
    value.store(&mut out[..L::WIDTH]);
    out[..L::WIDTH].iter().map(|word| word.to_bits()).collect()
}

fn fader_channel<L: Lane>(
    rng: &mut Rng,
    frames: usize,
    ramping: bool,
    mute: bool,
) -> GainMuteRamp<L> {
    let mut current = [0.0_f32; WIDTH];
    let mut target = [0.0_f32; WIDTH];
    let mut step = [0.0_f32; WIDTH];
    let mut remaining = [0.0_f32; WIDTH];
    let mut muted = [0.0_f32; WIDTH];
    for lane in 0..L::WIDTH {
        (current[lane], target[lane], step[lane], remaining[lane]) = rng.ramp_lane(frames, ramping);
        muted[lane] = if mute && rng.below(2) == 0 { 1.0 } else { 0.0 };
    }
    GainMuteRamp {
        current: L::load(&current[..L::WIDTH]),
        target: L::load(&target[..L::WIDTH]),
        step: L::load(&step[..L::WIDTH]),
        remaining: L::load(&remaining[..L::WIDTH]),
        mute: mask_from_flags::<L>(&muted[..L::WIDTH]),
    }
}

fn matrix_ramp<L: Lane>(rng: &mut Rng, frames: usize) -> Matrix2x2Ramp<L> {
    let mut current = [[0.0_f32; WIDTH]; 4];
    let mut target = [[0.0_f32; WIDTH]; 4];
    let mut step = [[0.0_f32; WIDTH]; 4];
    let mut remaining = [0.0_f32; WIDTH];
    for lane in 0..L::WIDTH {
        let n = rng.remaining(frames);
        remaining[lane] = n as f32;
        for coefficient in 0..4 {
            let t = rng.value();
            let (c, s) = if n == 0 {
                (t, 0.0)
            } else {
                let c = rng.value();
                let s = if rng.below(8) == 0 {
                    rng.value() * 0.25
                } else {
                    (t - c) / n as f32
                };
                (c, s)
            };
            current[coefficient][lane] = c;
            target[coefficient][lane] = t;
            step[coefficient][lane] = s;
        }
    }
    Matrix2x2Ramp {
        current: current.map(|words| L::load(&words[..L::WIDTH])),
        target: target.map(|words| L::load(&words[..L::WIDTH])),
        step: step.map(|words| L::load(&words[..L::WIDTH])),
        remaining: L::load(&remaining[..L::WIDTH]),
    }
}

/// A settled coefficient set. With `identity`, about half the lanes are exactly the identity
/// matrix and carry the mask; without, no lane carries it.
fn matrix_settled<L: Lane>(rng: &mut Rng, identity: bool) -> Matrix2x2Coef<L> {
    let mut coef = [[0.0_f32; WIDTH]; 4];
    let mut flags = [0.0_f32; WIDTH];
    for lane in 0..L::WIDTH {
        if identity && rng.below(2) == 0 {
            flags[lane] = 1.0;
            for (coefficient, value) in [1.0, 0.0, 0.0, 1.0].into_iter().enumerate() {
                coef[coefficient][lane] = value;
            }
        } else {
            for row in &mut coef {
                row[lane] = rng.value();
            }
        }
    }
    let [ll, lr, rl, rr] = coef.map(|words| L::load(&words[..L::WIDTH]));
    Matrix2x2Coef {
        ll,
        lr,
        rl,
        rr,
        identity: if identity {
            mask_from_flags::<L>(&flags[..L::WIDTH])
        } else {
            no_lanes::<L>()
        },
    }
}

#[derive(Clone, Copy, Debug)]
enum Arm {
    Ramp,
    Settled,
    SettledWithoutIdentity,
}

fn compare_case<L: Lane>(seed: u64, frames: usize, fader_ramps: bool, arm: Arm, mute: bool) {
    let mut rng = Rng(seed);
    let label = format!(
        "width={} seed={seed:#x} frames={frames} fader_ramps={fader_ramps} arm={arm:?} mute={mute}",
        L::WIDTH
    );
    let samples = frames * L::WIDTH;
    let mut left: Vec<f32> = (0..samples).map(|_| rng.sample()).collect();
    let mut right: Vec<f32> = (0..samples).map(|_| rng.sample()).collect();
    let mut fader = [
        fader_channel::<L>(&mut rng, frames, fader_ramps, mute),
        fader_channel::<L>(&mut rng, frames, fader_ramps, mute),
    ];
    let mut ramp = matrix_ramp::<L>(&mut rng, frames);
    let settled = matrix_settled::<L>(&mut rng, matches!(arm, Arm::Settled));

    let (mut old_left, mut old_right) = (left.clone(), right.clone());
    let mut old_fader = fader;
    let mut old_ramp = ramp;
    gain_mute_ramp_block(&mut old_left, frames, &mut old_fader[0]);
    gain_mute_ramp_block(&mut old_right, frames, &mut old_fader[1]);
    match arm {
        Arm::Ramp => matrix2x2_ramp_block(&mut old_left, &mut old_right, frames, &mut old_ramp),
        Arm::Settled => matrix2x2_block(&mut old_left, &mut old_right, frames, &settled),
        Arm::SettledWithoutIdentity => {
            matrix2x2_block_without_identity(&mut old_left, &mut old_right, frames, &settled);
        }
    }

    let matrix = match arm {
        Arm::Ramp => FaderMatrixArm::Ramp(&mut ramp),
        Arm::Settled => FaderMatrixArm::Settled(&settled),
        Arm::SettledWithoutIdentity => FaderMatrixArm::SettledWithoutIdentity(&settled),
    };
    fader_matrix_ramp_block(&mut left, &mut right, frames, &mut fader, matrix);

    for (plane, new, old) in [("L", &left, &old_left), ("R", &right, &old_right)] {
        for (index, (new, old)) in new.iter().zip(old.iter()).enumerate() {
            assert_eq!(
                new.to_bits(),
                old.to_bits(),
                "{label}: {plane}[{index}] new={new:e} old={old:e}"
            );
        }
    }
    for channel in 0..2 {
        assert_eq!(
            words(fader[channel].current),
            words(old_fader[channel].current),
            "{label}: fader[{channel}].current"
        );
        assert_eq!(
            words(fader[channel].remaining),
            words(old_fader[channel].remaining),
            "{label}: fader[{channel}].remaining"
        );
    }
    for coefficient in 0..4 {
        assert_eq!(
            words(ramp.current[coefficient]),
            words(old_ramp.current[coefficient]),
            "{label}: matrix.current[{coefficient}]"
        );
    }
    assert_eq!(
        words(ramp.remaining),
        words(old_ramp.remaining),
        "{label}: matrix.remaining"
    );
}

fn compare_width<L: Lane>() {
    let mut seed = 0x0890_0000_u64 + L::WIDTH as u64;
    for frames in [1, 2, 3, 7, 16] {
        for (fader_ramps, arm) in [
            (true, Arm::Settled),
            (true, Arm::SettledWithoutIdentity),
            (false, Arm::Ramp),
            (true, Arm::Ramp),
        ] {
            for mute in [false, true] {
                for _ in 0..24 {
                    seed = seed.wrapping_add(0x1_0001);
                    compare_case::<L>(seed, frames, fader_ramps, arm, mute);
                }
            }
        }
    }
}

/// The fused pass computes every output word and every ramp word of `gain_mute_ramp_block` on
/// each plane followed by the matrix kernel its arm names.
#[test]
fn fused_ramping_fader_matrix_matches_the_three_kernel_sequence() {
    let _canonical = CanonicalFpEnv::enter();
    lane::each_lane!(|L| compare_width::<L>());
}
