//! The bypass crossfade of *Crossfade the bypass switch over the session ramp* (#1341), emulated.
//!
//! The engine does not crossfade yet: today the shunt selects whole blocks of dry or wet
//! (`crates/effect-contract/src/live.rs:841-843`). This module renders the law #1341 freezes, so
//! that its click can be measured before it is built (#1055 questions 5 and 6):
//!
//! * D2: the mix `m` is word 0 of an `IndexedRamp` (`lane::kernels::IndexedRamp`, the shipped type,
//!   so `m(k)` here is the engine's own `round(round(k * step) + start)`), `m = 1` wet, `m = 0` dry.
//! * The record lands at a block boundary `at`; frame `f` of the run from `at` takes ramp index
//!   `k = f + 1`, as the live route kernel numbers its frames (`route_mix_ramp_block`).
//! * D3: a ramping frame is `dry + (wet - dry) * m(k)`, one `f32` subtract, multiply and add, no
//!   fused multiply-add.
//! * D3: both ends are exact copies. Before the record the output is the settled source (wet for a
//!   bypass, dry for an un-bypass), and from `k = length` on it is the settled target. Neither is
//!   computed through the formula, so a settled crossfade is today's bits, `-0.0` included.
//!
//! A length of 0 is today's step: the source up to `at`, the target from `at`.

use lane::kernels::IndexedRamp;

/// The settled side a crossfade moves to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Toward {
    /// `m: 1 -> 0`, wet to dry: engaging a bypass.
    Dry,
    /// `m: 0 -> 1`, dry to wet: lifting a bypass.
    Wet,
}

impl Toward {
    /// The ramp #1341 D2 starts for this direction from a settled lane.
    pub fn ramp(self, length: u32) -> IndexedRamp {
        match self {
            Self::Dry => IndexedRamp::new([1.0, 0.0, 0.0, 0.0], [0.0; 4], length),
            Self::Wet => IndexedRamp::new([0.0; 4], [1.0, 0.0, 0.0, 0.0], length),
        }
    }
}

/// Renders the crossfade of one plane: `dry` and `wet` are the shunt's latency-matched dry signal
/// and the effect's output over the same frames, and the record applies at frame `at`.
pub fn crossfade(dry: &[f32], wet: &[f32], at: usize, toward: Toward, length: u32) -> Vec<f32> {
    assert_eq!(dry.len(), wet.len());
    let ramp = toward.ramp(length);
    let (source, target) = match toward {
        Toward::Dry => (wet, dry),
        Toward::Wet => (dry, wet),
    };
    let mut out = Vec::with_capacity(dry.len());
    for i in 0..dry.len() {
        let Some(f) = i.checked_sub(at) else {
            out.push(source[i]);
            continue;
        };
        let k = u32::try_from(f + 1).unwrap_or(u32::MAX);
        if k >= ramp.length {
            out.push(target[i]);
        } else {
            let m = ramp.coefficients_at(k)[0];
            out.push(dry[i] + (wet[i] - dry[i]) * m);
        }
    }
    out
}

/// The mix trajectory `m` a crossfade applies, one word per frame: the crossfade of a dry plane of
/// `+0.0` and a wet plane of `1.0`, for which `dry + (wet - dry) * m` is exactly `m`.
pub fn mix_trajectory(frames: usize, at: usize, toward: Toward, length: u32) -> Vec<f32> {
    crossfade(&vec![0.0; frames], &vec![1.0; frames], at, toward, length)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A D11 accumulation of the same ramp: `m += step` each frame, the alternative #1341 rejects.
    fn accumulated(ramp: &IndexedRamp) -> Vec<f32> {
        let mut m = ramp.start[0];
        (1..ramp.length)
            .map(|_| {
                m += ramp.step[0];
                m
            })
            .collect()
    }

    /// Every ramping frame mixes by the indexed law, `c(k) = fma(k, step, start)` with `k` the
    /// frame's index from the record, `k = 1` on the first frame of the applying block; the frame
    /// at `k = length` is the target.
    ///
    /// The lengths are the default mute ramp at the four launch rates (441, 480, 882, 960), and
    /// for each of them the test first checks that an accumulated `m` differs from the indexed one
    /// on some frame, so an emulation that accumulated would turn it red.
    #[test]
    fn the_mix_follows_the_indexed_law_from_the_first_frame_after_the_record() {
        let at = 3 * 128;
        for length in [441_u32, 480, 882, 960] {
            for toward in [Toward::Dry, Toward::Wet] {
                let ramp = toward.ramp(length);
                let indexed: Vec<f32> = (1..length).map(|k| ramp.coefficients_at(k)[0]).collect();
                assert!(
                    indexed
                        .iter()
                        .zip(accumulated(&ramp))
                        .any(|(a, b)| a.to_bits() != b.to_bits()),
                    "length {length}: the accumulated and indexed laws agree, so the test is blind"
                );
                let frames = at + length as usize + 256;
                let m = mix_trajectory(frames, at, toward, length);
                let (start, end) = match toward {
                    Toward::Dry => (1.0_f32, 0.0_f32),
                    Toward::Wet => (0.0, 1.0),
                };
                assert!(m[..at].iter().all(|v| v.to_bits() == start.to_bits()));
                for (f, want) in indexed.iter().enumerate() {
                    assert_eq!(
                        m[at + f].to_bits(),
                        want.to_bits(),
                        "length {length}, {toward:?}, frame {f} (k = {})",
                        f + 1
                    );
                }
                assert!(
                    m[at + length as usize - 1..]
                        .iter()
                        .all(|v| v.to_bits() == end.to_bits())
                );
            }
        }
    }

    /// Deterministic planes in which the formula, evaluated at a settled end, does not return the
    /// settled plane's bits: `dry + (wet - dry) * 1` rounds away from `wet` on many words, and
    /// `-0.0 + (wet - dry) * 0` is `+0.0`, never `-0.0`.
    fn planes(frames: usize) -> (Vec<f32>, Vec<f32>) {
        let mut state = 0x1055_0005_u64;
        let mut next = || {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            ((state >> 40) as f32 / (1_u64 << 24) as f32) * 2.0 - 1.0
        };
        let mut dry = Vec::with_capacity(frames);
        let mut wet = Vec::with_capacity(frames);
        for i in 0..frames {
            let d = next();
            let w = next() * 0.7;
            dry.push(if i % 5 == 0 { -0.0 } else { d });
            wet.push(if i % 7 == 0 { -0.0 } else { w });
        }
        (dry, wet)
    }

    /// Both ends are exact copies: before the record the settled source, from `k = length` on the
    /// settled target, bit for bit, `-0.0` included. The planes are chosen so that computing either
    /// end through the formula changes some word's bits, which the test checks first.
    #[test]
    fn both_ends_are_exact_copies_of_the_settled_planes() {
        let (at, length) = (2 * 128, 480_u32);
        let frames = at + length as usize + 512;
        let (dry, wet) = planes(frames);
        let computed = |m: f32, i: usize| dry[i] + (wet[i] - dry[i]) * m;
        assert!((0..frames).any(|i| computed(1.0, i).to_bits() != wet[i].to_bits()));
        assert!((0..frames).any(|i| computed(0.0, i).to_bits() != dry[i].to_bits()));
        for toward in [Toward::Dry, Toward::Wet] {
            let out = crossfade(&dry, &wet, at, toward, length);
            let (source, target) = match toward {
                Toward::Dry => (&wet, &dry),
                Toward::Wet => (&dry, &wet),
            };
            for i in 0..at {
                assert_eq!(
                    out[i].to_bits(),
                    source[i].to_bits(),
                    "{toward:?} before, {i}"
                );
            }
            for i in at + length as usize - 1..frames {
                assert_eq!(
                    out[i].to_bits(),
                    target[i].to_bits(),
                    "{toward:?} after, {i}"
                );
            }
        }
        // A zero length is today's whole-block select: the target from the record's block on.
        let step = crossfade(&dry, &wet, at, Toward::Dry, 0);
        assert!((0..frames).all(|i| {
            let want = if i < at { wet[i] } else { dry[i] };
            step[i].to_bits() == want.to_bits()
        }));
    }

    /// Inside the ramp a frame is the three-operation `f32` mix with the indexed `m`.
    #[test]
    fn a_ramping_frame_is_dry_plus_the_scaled_difference() {
        let (at, length) = (128, 441_u32);
        let frames = at + length as usize;
        let (dry, wet) = planes(frames);
        let out = crossfade(&dry, &wet, at, Toward::Wet, length);
        let ramp = Toward::Wet.ramp(length);
        for f in 0..length as usize - 1 {
            let i = at + f;
            let m = ramp.coefficients_at(f as u32 + 1)[0];
            let difference = wet[i] - dry[i];
            let scaled = difference * m;
            assert_eq!(out[i].to_bits(), (dry[i] + scaled).to_bits(), "frame {f}");
        }
    }
}
