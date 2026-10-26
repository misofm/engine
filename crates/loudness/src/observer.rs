//! The render half: K-weighted energy and Annex-2 true peak per 10 ms hop.

use lane::{Lane, Native};
use true_peak_limiter::annex2::{ANNEX2_FIR, HISTORY_WORDS, History, detector_peak};

use crate::{KWeighting, LoudnessError};

/// One completed 10 ms hop of the rendered pair.
///
/// `Copy` and heap-free, so a render thread can hand it to a bounded queue.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HopRecord {
    /// The hop's first sample.
    pub first_sample: u64,
    /// One past the hop's last sample: `first_sample + rate / 100`.
    pub end_sample: u64,
    /// Sum, not mean, of the squared K-weighted samples of each channel, `[left, right]`.
    pub energy: [f64; 2],
    /// Largest Annex-2 true-peak estimate of each channel in the hop, linear, `[left, right]`.
    pub true_peak: [f32; 2],
    /// Non-finite input samples (both channels) replaced by `0.0` in the hop.
    pub sanitized: u32,
}

/// Lanes of the true-peak bank: L is lane 0, R lane 1, the rest are zero padding.
const PEAK_LANES: usize = <Native as Lane>::WIDTH;

/// Below this magnitude a biquad state word is set to zero at a hop end.
///
/// After long silence the f64 states decay towards the subnormal range (`< 2.2e-308`), where some
/// targets take a slow path per operation. A state this small adds less than `1e-490` to any hop's
/// energy, which f64 cannot even represent, so flushing it changes no reading.
const STATE_FLUSH: f64 = 1e-250;

/// Two channels of one direct-form-II-transposed biquad, `[left, right]` per word.
///
/// The two channels are written as element-wise `[f64; 2]` arithmetic, which is the one shape on
/// every target: each lane is its own IEEE binary64 multiply and add, rounded separately (Rust
/// never fuses), so the result is the same bits whether the backend emits one 2-lane vector op or
/// two scalar ones. That is why this is a `[f64; 2]` and not a `lane::LaneF64` value: the trait
/// has no 2-lane implementation, and padding to four or eight lanes would spend the work of two
/// to eight channels on two.
#[derive(Clone, Copy, Debug)]
struct Biquad {
    coefficients: [f64; 5],
    z1: [f64; 2],
    z2: [f64; 2],
}

/// The live state of one observer, across hops and blocks.
#[derive(Clone, Copy)]
struct State {
    shelf: Biquad,
    highpass: Biquad,
    history: History<Native>,
    energy: [f64; 2],
    peak: Native,
    sanitized: u32,
    hop_start: u64,
    filled: u32,
}

/// The loudness meter's render half for one stereo pair.
///
/// Built on the control thread by [`LoudnessObserver::new`]; [`LoudnessObserver::observe`] then
/// runs on the render thread without allocating, locking, logging or panicking on any sample
/// value. Per sample and channel it replaces a non-finite sample by `0.0` (and counts it), runs the
/// two K-weighting biquads in `f64`, adds the squared result to the open hop's energy, and runs
/// the limiter's own Annex-2 detector (`true_peak_limiter::annex2`, `f32`) with a running per-hop
/// maximum. Every `rate / 100` samples it emits one [`HopRecord`].
///
/// # Continuity
///
/// The observer expects each block to start where the last one ended. A block whose
/// `first_sample` is anything else (a seek, a dropped block, the first block) restarts the
/// observer at it, exactly as [`LoudnessObserver::restart`]: filter and detector state cleared,
/// the open partial hop discarded, the hop grid restarted at `first_sample`.
///
/// # True peak
///
/// The detector's sample term is the history's centre tap, six samples behind the newest input,
/// so the last six samples of a hop count towards the next hop's peak. A peak word the detector
/// makes `NaN` (possible only for finite inputs near `f32::MAX`, whose products overflow) is
/// ignored by the running maximum, which is `new.max(running)`: [`Lane::max`] keeps its second
/// operand when the first is `NaN`.
pub struct LoudnessObserver {
    hop: u32,
    weighting: KWeighting,
    fir: [[Native; 4]; HISTORY_WORDS],
    state: State,
    next_sample: Option<u64>,
}

impl LoudnessObserver {
    /// An observer at `rate`, waiting for its first block.
    ///
    /// # Errors
    ///
    /// [`LoudnessError::UnsupportedRate`] unless `rate` is 44.1, 48, 88.2 or 96 kHz.
    pub fn new(rate: u32) -> Result<Self, LoudnessError> {
        let weighting = KWeighting::new(rate)?;
        let mut fir = [[Native::zero(); 4]; HISTORY_WORDS];
        for (row, table) in fir.iter_mut().zip(ANNEX2_FIR) {
            for (word, coefficient) in row.iter_mut().zip(table) {
                *word = Native::splat(coefficient);
            }
        }
        Ok(Self {
            hop: rate / 100,
            weighting,
            fir,
            state: State::rest(&weighting, 0),
            next_sample: None,
        })
    }

    /// Samples per hop: `rate / 100`.
    #[must_use]
    pub fn hop_samples(&self) -> u32 {
        self.hop
    }
}

// REALTIME_POLICY_BEGIN: #2115 S1, the loudness meter's render half.
impl State {
    fn rest(weighting: &KWeighting, at_sample: u64) -> Self {
        Self {
            shelf: Biquad::rest(weighting.stage1()),
            highpass: Biquad::rest(weighting.stage2()),
            history: History::zero(),
            energy: [0.0; 2],
            peak: Native::zero(),
            sanitized: 0,
            hop_start: at_sample,
            filled: 0,
        }
    }
}

impl Biquad {
    fn rest(coefficients: [f64; 5]) -> Self {
        Self {
            coefficients,
            z1: [0.0; 2],
            z2: [0.0; 2],
        }
    }

    /// One sample of both channels, direct form II transposed.
    #[inline(always)]
    fn run(&mut self, x: [f64; 2]) -> [f64; 2] {
        let [b0, b1, b2, a1, a2] = self.coefficients;
        let mut y = [0.0; 2];
        for c in 0..2 {
            y[c] = b0 * x[c] + self.z1[c];
            self.z1[c] = b1 * x[c] - a1 * y[c] + self.z2[c];
            self.z2[c] = b2 * x[c] - a2 * y[c];
        }
        y
    }

    /// Sets every state word below [`STATE_FLUSH`] in magnitude to zero.
    #[inline(always)]
    fn flush(&mut self) {
        for word in self.z1.iter_mut().chain(self.z2.iter_mut()) {
            if word.abs() < STATE_FLUSH {
                *word = 0.0;
            }
        }
    }
}

impl LoudnessObserver {
    /// Restarts the observer at `at_sample`: filter and detector state cleared, the open partial
    /// hop discarded, and the next hop starting at `at_sample`.
    pub fn restart(&mut self, at_sample: u64) {
        self.state = State::rest(&self.weighting, at_sample);
        self.next_sample = Some(at_sample);
    }

    /// Observes one block of the rendered pair, which starts at `first_sample`, and calls `emit`
    /// once for each hop the block completes, in order.
    ///
    /// The planes should be the same length; in a debug build a mismatch panics, and in a release
    /// build only the first `min(left.len(), right.len())` frames are observed and the block is
    /// taken to be that long. A block that does not continue the last one restarts the observer
    /// first (see [Continuity](LoudnessObserver#continuity)).
    pub fn observe(
        &mut self,
        left: &[f32],
        right: &[f32],
        first_sample: u64,
        emit: &mut impl FnMut(HopRecord),
    ) {
        debug_assert_eq!(left.len(), right.len(), "loudness planes differ in length");
        let frames = left.len().min(right.len());
        if self.next_sample != Some(first_sample) {
            self.restart(first_sample);
        }
        self.next_sample = Some(first_sample.wrapping_add(frames as u64));

        let hop = self.hop;
        let fir = &self.fir;
        let state = &mut self.state;
        let mut done = 0;
        while done < frames {
            let room = (hop - state.filled) as usize;
            let take = room.min(frames - done);
            // `get`, never an index: the slices always exist (`take <= frames - done`), and no
            // checked index (a `slice_index_fail` trap) joins a host's render call graph.
            let (Some(left), Some(right)) = (
                left.get(done..done + take),
                right.get(done..done + take),
            ) else {
                return;
            };
            state.run(left, right, fir);
            done += take;
            state.filled += take as u32;
            if state.filled == hop {
                emit(state.close(hop));
            }
        }
    }
}

impl State {
    /// The samples of one hop's open part.
    #[inline(always)]
    fn run(&mut self, left: &[f32], right: &[f32], fir: &[[Native; 4]; HISTORY_WORDS]) {
        for (&l, &r) in left.iter().zip(right) {
            let l_finite = l.is_finite();
            let r_finite = r.is_finite();
            self.sanitized += u32::from(!l_finite) + u32::from(!r_finite);
            let l = if l_finite { l } else { 0.0 };
            let r = if r_finite { r } else { 0.0 };

            let mut lanes = [0.0_f32; PEAK_LANES];
            lanes[0] = l;
            lanes[1] = r;
            let peak = detector_peak(&mut self.history, Native::load(&lanes), fir);
            self.peak = peak.max(self.peak);

            let y = self
                .highpass
                .run(self.shelf.run([f64::from(l), f64::from(r)]));
            for (energy, y) in self.energy.iter_mut().zip(y) {
                *energy += y * y;
            }
        }
    }

    /// Closes the full open hop: its record, then a fresh hop after it.
    #[inline(always)]
    fn close(&mut self, hop: u32) -> HopRecord {
        let mut lanes = [0.0_f32; PEAK_LANES];
        self.peak.store(&mut lanes);
        let end_sample = self.hop_start.wrapping_add(u64::from(hop));
        let record = HopRecord {
            first_sample: self.hop_start,
            end_sample,
            energy: self.energy,
            true_peak: [lanes[0], lanes[1]],
            sanitized: self.sanitized,
        };
        self.shelf.flush();
        self.highpass.flush();
        self.energy = [0.0; 2];
        self.peak = Native::zero();
        self.sanitized = 0;
        self.hop_start = end_sample;
        self.filled = 0;
        record
    }
}
// REALTIME_POLICY_END

#[cfg(test)]
mod tests {
    use super::*;

    fn records(left: &[f32], right: &[f32]) -> Vec<HopRecord> {
        let mut observer = LoudnessObserver::new(48_000).expect("launch rate");
        let mut out = Vec::new();
        observer.observe(left, right, 0, &mut |record| out.push(record));
        out
    }

    #[test]
    fn a_block_that_does_not_continue_the_last_restarts_the_observer() {
        let block = |offset: usize, frames: usize| -> Vec<f32> {
            (0..frames)
                .map(|n| (((n + offset) * 13) % 64) as f32 / 64.0 - 0.5)
                .collect()
        };
        let first = block(0, 700);
        let second = block(3, 960);
        let mut observer = LoudnessObserver::new(48_000).expect("launch rate");
        let mut out = Vec::new();
        observer.observe(&first, &first, 0, &mut |record| out.push(record));
        out.clear();
        observer.observe(&second, &second, 10_000, &mut |record| out.push(record));
        let mut fresh = LoudnessObserver::new(48_000).expect("launch rate");
        let mut expected = Vec::new();
        fresh.observe(&second, &second, 10_000, &mut |record| {
            expected.push(record)
        });
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].first_sample, 10_000);
        assert_eq!(out, expected);
    }

    #[test]
    fn a_non_finite_sample_reads_as_zero_and_is_counted() {
        let tone: Vec<f32> = (0..1_920)
            .map(|n| ((n * 7) % 96) as f32 / 96.0 - 0.5)
            .collect();
        let mut left = tone.clone();
        let mut right = tone.clone();
        left[5] = f32::NAN;
        right[5] = f32::INFINITY;
        left[700] = f32::NEG_INFINITY;
        right[1_919] = f32::NAN;
        let mut zeroed_left = left.clone();
        let mut zeroed_right = right.clone();
        for sample in zeroed_left.iter_mut().chain(zeroed_right.iter_mut()) {
            if !sample.is_finite() {
                *sample = 0.0;
            }
        }
        let dirty = records(&left, &right);
        let clean = records(&zeroed_left, &zeroed_right);
        assert_eq!(dirty.len(), 4);
        let counts: Vec<u32> = dirty.iter().map(|record| record.sanitized).collect();
        assert_eq!(counts, [2, 1, 0, 1]);
        for (dirty, clean) in dirty.iter().zip(&clean) {
            assert_eq!(clean.sanitized, 0);
            assert_eq!(
                HopRecord {
                    sanitized: 0,
                    ..*dirty
                },
                *clean
            );
        }
    }
}
