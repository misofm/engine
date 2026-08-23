//! The polyphase half-band kernels: table shape, window addressing and lane identity.
//!
//! The *bit-identity against the frozen 63-tap graph* is proved where that graph lives, in
//! `miso-engine-soft-clip`'s `tests/polyphase_identity.rs`. What is proved here is what this
//! module owns: the table is the symmetric half-band the decomposition assumes, the double-written
//! history addresses the right sample at every position, and the three widths agree by bits.

use miso_engine_lane::Lane;
use miso_engine_lane::kernels::halfband::{
    HALFBAND63_CENTER, HALFBAND63_CENTER_SPLIT, HALFBAND63_EVEN, HALFBAND63_EVEN_TAPS,
    HALFBAND63_LIVE_ROWS, HALFBAND63_ROWS, halfband2x_decim_even, halfband2x_interp_even,
    history_advance, history_push, history_row,
};
use miso_engine_lane::{Simd4, Simd8};

/// A deterministic, target-independent sample source.
fn sample(index: usize) -> f32 {
    let mut state = 0x9E37_79B9_7F4A_7C15_u64 ^ (index as u64).wrapping_mul(0x2545_F491_4F6C_DD1D);
    state ^= state >> 12;
    state ^= state << 25;
    state ^= state >> 27;
    f32::from(((state >> 40) as u16 >> 1) as i16) * (1.0 / 16_384.0) - 1.0
}

#[test]
fn the_even_tap_table_is_the_symmetric_half_band() {
    assert_eq!(HALFBAND63_EVEN_TAPS, 30);
    assert_eq!(HALFBAND63_CENTER.to_bits(), 0.5_f32.to_bits());
    assert_eq!(HALFBAND63_CENTER_SPLIT, 15);
    for k in 0..HALFBAND63_EVEN_TAPS {
        assert_eq!(
            HALFBAND63_EVEN[k].to_bits(),
            HALFBAND63_EVEN[HALFBAND63_EVEN_TAPS - 1 - k].to_bits(),
            "tap {k} breaks the half-band symmetry h[2k] = h[62-2k]"
        );
    }
    // A half-band's off-centre taps sum to the centre tap, which is what makes the pair a
    // unit-gain interpolator. `f32` rounding of a 30-term sum leaves a few ulp.
    let sum: f64 = HALFBAND63_EVEN.iter().map(|tap| f64::from(*tap)).sum();
    assert!(
        (sum - 0.5).abs() < 1.0e-7,
        "off-centre taps sum to {sum}, not one half"
    );
}

/// Row `base - k` must hold the sample pushed `k` frames ago, at every position of the ring.
#[test]
fn the_double_written_history_addresses_every_age_at_every_position() {
    let mut history = vec![0.0_f32; HALFBAND63_ROWS];
    let mut pos = 0_usize;
    for frame in 0..(4 * HALFBAND63_LIVE_ROWS) {
        history_push::<f32>(&mut history, pos, sample(frame));
        let base = pos + HALFBAND63_LIVE_ROWS;
        for age in 0..=31_usize {
            if age > frame {
                continue;
            }
            let stored = history_row::<f32>(&history, base - age)[0];
            assert_eq!(
                stored.to_bits(),
                sample(frame - age).to_bits(),
                "frame {frame}, position {pos}, age {age}"
            );
        }
        pos = history_advance(pos);
    }
}

/// The kernels are one body; the widths must agree bit for bit (master plan §1.1).
#[test]
fn interpolation_and_decimation_are_width_independent() {
    const FRAMES: usize = 512;
    const LANES: usize = 8;

    fn run<L: Lane>() -> Vec<[f32; 2]> {
        let mut history = vec![0.0_f32; HALFBAND63_ROWS * L::WIDTH];
        let mut pos = 0_usize;
        let mut out = Vec::with_capacity(FRAMES * L::WIDTH);
        let mut frame = [0.0_f32; LANES];
        for index in 0..FRAMES {
            for (lane, value) in frame.iter_mut().take(L::WIDTH).enumerate() {
                *value = sample(index * LANES + lane);
            }
            let x = L::load(&frame[..L::WIDTH]);
            history_push::<L>(&mut history, pos, x);
            let base = pos + HALFBAND63_LIVE_ROWS;
            let interp = halfband2x_interp_even::<L>(&history, base);
            let odd = L::splat(0.5).mul(L::load(history_row::<L>(&history, base - 31)));
            let decim = halfband2x_decim_even::<L>(&history, base, odd);
            let mut a = [0.0_f32; LANES];
            let mut b = [0.0_f32; LANES];
            interp.store(&mut a[..L::WIDTH]);
            decim.store(&mut b[..L::WIDTH]);
            for lane in 0..L::WIDTH {
                out.push([a[lane], b[lane]]);
            }
            pos = history_advance(pos);
        }
        out
    }

    let scalar = run::<f32>();
    // The scalar run advances a one-lane ring; comparing widths means comparing the same lane
    // stream, so each width is compared against a scalar run of the same signal.
    for (width, values) in [(4_usize, run::<Simd4>()), (8, run::<Simd8>())] {
        assert_eq!(values.len(), FRAMES * width);
        for index in 0..FRAMES {
            for lane in 0..width {
                // The scalar run used LANES-strided sample indices; rebuild the same lane stream.
                let mut history = vec![0.0_f32; HALFBAND63_ROWS];
                let mut pos = 0_usize;
                let (mut interp, mut decim) = (0.0_f32, 0.0_f32);
                for frame in 0..=index {
                    history_push::<f32>(&mut history, pos, sample(frame * LANES + lane));
                    let base = pos + HALFBAND63_LIVE_ROWS;
                    interp = halfband2x_interp_even::<f32>(&history, base);
                    let odd = 0.5_f32 * history_row::<f32>(&history, base - 31)[0];
                    decim = halfband2x_decim_even::<f32>(&history, base, odd);
                    pos = history_advance(pos);
                }
                let actual = values[index * width + lane];
                assert_eq!(
                    actual[0].to_bits(),
                    interp.to_bits(),
                    "interp {index}/{lane}"
                );
                assert_eq!(actual[1].to_bits(), decim.to_bits(), "decim {index}/{lane}");
            }
        }
    }
    assert_eq!(scalar.len(), FRAMES);
}
