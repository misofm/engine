//! Clear-on-read taps: a ring word enters the loop at most once, so a size increase opens a gap
//! and never replays ring audio (root ruling OQ2b of #2114).

mod common;

use common::{DAMPING, DIFFUSION, PRE_DELAY, QUANTUM, SIZE, defaults, prepare};
use effect_contract::{
    EffectProcessBlock, ParameterChannel, PreparedAutomationSpan, ProcessReport,
};

const RATE: u32 = 48_000;

/// The impulse reaches the lines through the four diffusers, pure delays at diffusion 0:
/// `229 + 173 + 613 + 449` samples plus the three pipeline registers. It is written into lines
/// 0-7 at this sample.
const INJECTED: usize = 1_467;

/// Line 0's length at size 0.25 and 2.0 at 48 kHz (`Prime(953 s + 0.5)`); line 0 has the
/// smallest base, so `LONG_0 - SHORT_0` is the smallest size-2-minus-size-0.25 difference of the
/// sixteen lines.
const SHORT_0: usize = 239;
const LONG_0: usize = 1_907;

/// The crossfade length.
const FADE: usize = 128;

/// The size decrease (2 to 0.25): its crossfade, `[DOWN, DOWN + 128)`, is when lines 0, 1 and 2
/// (short lengths 239, 293, 359) first read the impulse, through the new tap. `DOWN - 239` is
/// the impulse sample, so every word the decrease skips (written in `[DOWN + 128 - long_i,
/// DOWN - short_i)`) predates the impulse and is `+0.0`.
const DOWN: usize = INJECTED + SHORT_0;

/// The size increase (0.25 to 2), after the decrease's crossfade and early enough that lines
/// 0-2 would replay the impulse word (at `INJECTED + long_i`: 3,374, 3,808 and 4,328) inside
/// [`window`].
const UP: usize = 2_600;

/// The samples after the increase's crossfade where every tap reads a word that an earlier tap
/// already read (or one that predates the impulse): a tap at `long_i` reads, at `t`, the word
/// written at `t - long_i`, which the short tap read at `t - long_i + short_i < UP + 128` while
/// `t < UP + 128 + (long_i - short_i)`. The first two samples are skipped: the damping low-pass
/// (at the `fs / 4` cap, coefficient exactly 1/2) still holds the last read word for one sample.
fn window() -> core::ops::Range<usize> {
    UP + FADE + 2..UP + FADE + (LONG_0 - SHORT_0)
}

fn span(sample: usize, value: f32) -> PreparedAutomationSpan {
    PreparedAutomationSpan {
        channel: ParameterChannel::Both,
        parameter_index: SIZE as u32,
        sample: sample as u64,
        value,
    }
}

/// With clear-on-read, every ring word is read into the loop at most once. The probe: an impulse
/// at size 2 and diffusion 0, a size decrease to 0.25 whose crossfade's new taps read the impulse
/// on lines 0-2, then a size increase back to 2 at sample 2,600. For 1,668 samples after the
/// increase's crossfade every tap of every line reads a word that was read before (by the steady
/// short taps, by the decrease's new taps, or by the increase's old taps in its crossfade), or
/// one written before the impulse arrived. Those words were cleared, so every tap reads `+0.0`
/// and the output is only the free decay of the in-loop high-pass states: each output sample is
/// `1 - 2 g2` times the one before, to `f32` rounding.
///
/// Red for: a tap that does not clear the word it read (the steady tap, the old tap in a
/// crossfade or the new tap in a crossfade). The increase's new taps then replay ring audio a
/// second time, a ghost of the impulse or of the early reflections, which breaks the free decay.
/// No other test makes a tap read a word twice: the tail contract's size steps only bound the
/// energy, and the oracle test runs at fixed size.
#[test]
fn a_size_increase_never_replays_a_word_already_read() {
    let mut values = defaults();
    values[SIZE] = 2.0;
    values[PRE_DELAY] = 0.0;
    values[DIFFUSION] = 0.0;
    values[DAMPING] = 20_000.0;
    let mut effect = prepare(&values, RATE);

    let frames = window().end + 64;
    let mut left = vec![0.0_f32; frames];
    let mut right = vec![0.0_f32; frames];
    left[0] = 1.0;
    let events = [span(DOWN, 0.25), span(UP, 2.0)];
    let mut offset = 0;
    while offset < frames {
        let mut end = (offset + QUANTUM as usize).min(frames);
        for event in &events {
            let at = event.sample as usize;
            if offset < at && at < end {
                end = at;
            }
        }
        let automation: Vec<PreparedAutomationSpan> = events
            .iter()
            .copied()
            .filter(|event| event.sample as usize == offset)
            .collect();
        let report = effect.process(
            EffectProcessBlock::new(
                &mut left[offset..end],
                &mut right[offset..end],
                None,
                offset as u64,
                &automation,
                QUANTUM,
            )
            .expect("block"),
        );
        assert_eq!(report, ProcessReport::default(), "block at {offset}");
        offset = end;
    }

    // The low-cut coefficient as the kernel designs it: `tan` in `f64`, rounded once to `f32`.
    let warped = math::tan(core::f64::consts::PI * 20.0 / f64::from(RATE));
    let g2 = f64::from((warped / (1.0 + warped)) as f32);
    let ratio = 1.0 - 2.0 * g2;
    let before = left[..UP].iter().fold(0.0_f32, |peak, x| peak.max(x.abs()));
    let mut peak = 0.0_f64;
    let mut worst = 0.0_f64;
    for output in [&left, &right] {
        for t in window().start..window().end - 1 {
            let (now, next) = (f64::from(output[t]), f64::from(output[t + 1]));
            peak = peak.max(now.abs());
            worst = worst.max((next - ratio * now).abs());
        }
    }
    println!(
        "peak before the increase {before:e}; in the window: peak {peak:e}, worst departure from the free decay {worst:e}"
    );
    assert!(before > 0.05, "the probe must carry the impulse");
    assert!(
        worst <= 1.0e-8,
        "a tap read a word twice: departure {worst:e} from the free decay"
    );
}
