//! The control half: momentary, short-term, integrated, loudness range and true peak from hop
//! records.

use crate::HopRecord;

/// Hops in the momentary window (400 ms).
pub const MOMENTARY_HOPS: usize = 40;
/// Hops in the short-term window (3 s).
pub const SHORT_TERM_HOPS: usize = 300;
/// Hops in the 100 ms step of gating blocks and short-term blocks.
pub const BLOCK_STEP_HOPS: u64 = 10;
/// The level reported for a window whose mean square is zero or would read below it, LUFS.
pub const LOUDNESS_FLOOR_LUFS: f64 = -120.0;
/// Lower edge of the histograms, LUFS: the absolute gate of BS.1770-5 and Tech 3342.
pub const HISTOGRAM_FLOOR_LUFS: f64 = -70.0;
/// Bins per LU.
pub const BINS_PER_LU: f64 = 10.0;
/// Histogram bins: 0.1 LU each over `[-70, +10)` LUFS; a block at or above +10 LUFS lands in the
/// top bin.
pub const HISTOGRAM_BINS: usize = 800;

/// The level of a mean square: `-0.691 + 10 log10 z` LUFS (both channel weights 1).
fn level(mean_square: f64) -> f64 {
    -0.691 + 10.0 * math::log10(mean_square)
}

/// A level for display: [`LOUDNESS_FLOOR_LUFS`] for silence or anything below it.
fn floored_level(mean_square: f64) -> f64 {
    if mean_square > 0.0 {
        level(mean_square).max(LOUDNESS_FLOOR_LUFS)
    } else {
        LOUDNESS_FLOOR_LUFS
    }
}

/// Block levels in 0.1 LU bins, each with its exact block count and exact sum of block mean squares.
///
/// Bounded memory for any programme length. A bin's *mean level* is the level of its energy sum
/// over its count, which is exact for every block in a bin of equal blocks and within the bin
/// (0.1 LU) otherwise.
struct Histogram {
    count: Box<[u64]>,
    energy: Box<[f64]>,
}

impl Histogram {
    fn new() -> Self {
        Self {
            count: vec![0; HISTOGRAM_BINS].into_boxed_slice(),
            energy: vec![0.0; HISTOGRAM_BINS].into_boxed_slice(),
        }
    }

    fn clear(&mut self) {
        self.count.fill(0);
        self.energy.fill(0.0);
    }

    /// Adds a block of mean square `z` whose level `l` the caller has already gated.
    fn add(&mut self, z: f64, l: f64) {
        let bin = ((l - HISTOGRAM_FLOOR_LUFS) * BINS_PER_LU) as usize;
        let bin = bin.min(HISTOGRAM_BINS - 1);
        self.count[bin] += 1;
        self.energy[bin] += z;
    }

    /// `(count, energy, mean level)` of every non-empty bin, ascending.
    fn bins(&self) -> impl Iterator<Item = (u64, f64, f64)> + '_ {
        self.count
            .iter()
            .zip(self.energy.iter())
            .filter(|(count, _)| **count > 0)
            .map(|(&count, &energy)| (count, energy, level(energy / count as f64)))
    }

    /// Count and energy of every block.
    fn total(&self) -> (u64, f64) {
        self.bins().fold((0, 0.0), |(n, e), (count, energy, _)| {
            (n + count, e + energy)
        })
    }
}

/// The meter's control half: drains [`HopRecord`]s and keeps every reading.
///
/// # Windows
///
/// Momentary (M) is the last [`MOMENTARY_HOPS`] hops, short-term (S) the last
/// [`SHORT_TERM_HOPS`]; each is `None` until that many continuous hops have arrived. Both are
/// recomputed exactly from a ring of the last 300 records at every hop (340 additions), which
/// costs nothing at 100 hops a second and cannot drift the way a running sum would.
///
/// A record whose `first_sample` is not the previous record's `end_sample` is a **gap**: M and S
/// restart at it (a window across a gap would mix unrelated audio), and so does the assembly of
/// gating and short-term blocks, whose 100 ms grid restarts at the gap. The histograms keep the
/// blocks they already hold.
///
/// # Reset and integration
///
/// [`LoudnessAnalysis::reset`] drops every hop that starts before its sample, from records already
/// held in the ring and from records still to come. [`LoudnessAnalysis::integrating_since`] is
/// then the first sample of the first kept hop, the first hop boundary at or after the reset
/// sample. A reset clears the integrated and loudness-range histograms, the true peak and the
/// M/S maxima; M and S themselves keep running.
///
/// Everything "since reset" counts only windows that lie wholly in kept, continuous hops: the
/// maxima are of M and S windows whose hops are all kept, gating blocks (400 ms) and short-term
/// blocks (3 s) start on the 100 ms grid that starts at `integrating_since` (or the last gap
/// after it), and an incomplete block is not counted.
///
/// # Integrated loudness (BS.1770-5)
///
/// Gating blocks with level `l > -70` go into a 0.1 LU histogram. The relative gate is
/// `gamma_r = -0.691 + 10 log10(mean z of those blocks) - 10`, exact from the bin sums. A bin is
/// above `gamma_r` when its mean level is: the one bin that straddles `gamma_r` is taken or left
/// whole by that rule, which is the meter's only approximation and is bounded by the oracle test.
/// I is the level of the mean `z` over the bins above `gamma_r`.
///
/// # Loudness range (EBU Tech 3342)
///
/// Short-term blocks with `l >= -70` go into a second histogram. The relative gate is the
/// histogram's power mean less 20 LU, with Tech 3342's `>=`, and LRA is `P95 - P10` of the blocks
/// in bins at or above it, with Tech 3342's MATLAB ranks `round((n - 1) p / 100 + 1)` (1-based).
/// A percentile's value is the mean level of the bin that holds its rank, so each percentile is
/// within half a bin of the exact sorted value for a bin of equal blocks, and within a bin
/// otherwise.
pub struct LoudnessAnalysis {
    ring: Box<[HopRecord]>,
    /// Records in the ring since its last restart (at a gap), wrapping only after `2^64` hops.
    held: u64,
    previous_end: Option<u64>,
    reset_at: u64,
    since: Option<u64>,
    /// Kept, continuous hops since `integrating_since` or the last gap after it.
    run: u64,
    integrated: Histogram,
    range: Histogram,
    true_peak: [f32; 2],
    sanitized: u64,
    momentary_max: Option<f64>,
    short_term_max: Option<f64>,
}

/// An empty ring slot.
const EMPTY: HopRecord = HopRecord {
    first_sample: 0,
    end_sample: 0,
    energy: [0.0; 2],
    true_peak: [0.0; 2],
    sanitized: 0,
};

impl Default for LoudnessAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

impl LoudnessAnalysis {
    /// An empty analysis integrating from sample 0. Allocates its ring and histograms, once.
    #[must_use]
    pub fn new() -> Self {
        Self {
            ring: vec![EMPTY; SHORT_TERM_HOPS].into_boxed_slice(),
            held: 0,
            previous_end: None,
            reset_at: 0,
            since: None,
            run: 0,
            integrated: Histogram::new(),
            range: Histogram::new(),
            true_peak: [0.0; 2],
            sanitized: 0,
            momentary_max: None,
            short_term_max: None,
        }
    }

    /// Takes one hop record, in render order.
    pub fn ingest(&mut self, record: &HopRecord) {
        if self.previous_end != Some(record.first_sample) {
            self.held = 0;
            self.run = 0;
        }
        self.previous_end = Some(record.end_sample);
        let position = self.held;
        self.ring[(position % SHORT_TERM_HOPS as u64) as usize] = *record;
        self.held += 1;
        if record.first_sample >= self.reset_at {
            self.integrate(position);
        }
    }

    /// Drops every hop that starts before `at_sample` and restarts integration at the first hop
    /// boundary at or after it.
    pub fn reset(&mut self, at_sample: u64) {
        self.integrated.clear();
        self.range.clear();
        self.true_peak = [0.0; 2];
        self.sanitized = 0;
        self.momentary_max = None;
        self.short_term_max = None;
        self.reset_at = at_sample;
        self.since = None;
        self.run = 0;
        let oldest = self.held.saturating_sub(SHORT_TERM_HOPS as u64);
        for position in oldest..self.held {
            if self.slot(position).first_sample >= at_sample {
                self.integrate(position);
            }
        }
    }

    fn slot(&self, position: u64) -> &HopRecord {
        &self.ring[(position % SHORT_TERM_HOPS as u64) as usize]
    }

    /// Mean square of the `length` held records that end at ring position `last`.
    fn window(&self, last: u64, length: usize) -> f64 {
        let mut energy = 0.0;
        let mut samples = 0_u64;
        for position in last + 1 - length as u64..=last {
            let record = self.slot(position);
            energy += record.energy[0] + record.energy[1];
            samples += record.end_sample.wrapping_sub(record.first_sample);
        }
        if samples == 0 {
            0.0
        } else {
            energy / samples as f64
        }
    }

    /// Adds the kept record at ring position `position` to everything since reset.
    fn integrate(&mut self, position: u64) {
        let record = *self.slot(position);
        if self.since.is_none() {
            self.since = Some(record.first_sample);
        }
        self.run += 1;
        for (peak, hop) in self.true_peak.iter_mut().zip(record.true_peak) {
            *peak = hop.max(*peak);
        }
        self.sanitized += u64::from(record.sanitized);
        let on_grid = self.run.is_multiple_of(BLOCK_STEP_HOPS);
        if self.run >= MOMENTARY_HOPS as u64 {
            let z = self.window(position, MOMENTARY_HOPS);
            self.momentary_max = Some(self.momentary_max.map_or(z, |max| max.max(z)));
            if on_grid && z > 0.0 {
                let l = level(z);
                if l > HISTOGRAM_FLOOR_LUFS {
                    self.integrated.add(z, l);
                }
            }
        }
        if self.run >= SHORT_TERM_HOPS as u64 {
            let z = self.window(position, SHORT_TERM_HOPS);
            self.short_term_max = Some(self.short_term_max.map_or(z, |max| max.max(z)));
            if on_grid && z > 0.0 {
                let l = level(z);
                if l >= HISTOGRAM_FLOOR_LUFS {
                    self.range.add(z, l);
                }
            }
        }
    }

    /// Momentary loudness of the last 400 ms, LUFS, floored at [`LOUDNESS_FLOOR_LUFS`]; `None`
    /// until 40 continuous hops have arrived.
    #[must_use]
    pub fn momentary(&self) -> Option<f64> {
        (self.held >= MOMENTARY_HOPS as u64)
            .then(|| floored_level(self.window(self.held - 1, MOMENTARY_HOPS)))
    }

    /// Short-term loudness of the last 3 s, LUFS, floored; `None` until 300 continuous hops.
    #[must_use]
    pub fn short_term(&self) -> Option<f64> {
        (self.held >= SHORT_TERM_HOPS as u64)
            .then(|| floored_level(self.window(self.held - 1, SHORT_TERM_HOPS)))
    }

    /// Largest momentary loudness since reset, LUFS, floored.
    #[must_use]
    pub fn momentary_max(&self) -> Option<f64> {
        self.momentary_max.map(floored_level)
    }

    /// Largest short-term loudness since reset, LUFS, floored.
    #[must_use]
    pub fn short_term_max(&self) -> Option<f64> {
        self.short_term_max.map(floored_level)
    }

    /// `(count, energy)` of the gating blocks above the relative gate, or `None` with no block
    /// above the absolute gate.
    fn relative_gated(&self) -> Option<(u64, f64)> {
        let (count, energy) = self.integrated.total();
        if count == 0 {
            return None;
        }
        let gamma = level(energy / count as f64) - 10.0;
        Some(
            self.integrated
                .bins()
                .filter(|(_, _, mean)| *mean > gamma)
                .fold((0, 0.0), |(n, e), (count, energy, _)| {
                    (n + count, e + energy)
                }),
        )
    }

    /// BS.1770-5 integrated loudness since reset, LUFS; `None` until a gating block passes both
    /// gates.
    #[must_use]
    pub fn integrated(&self) -> Option<f64> {
        let (count, energy) = self.relative_gated()?;
        (count > 0).then(|| level(energy / count as f64))
    }

    /// Gating blocks since reset above both gates: the blocks the integrated loudness averages.
    #[must_use]
    pub fn gated_block_count(&self) -> u64 {
        self.relative_gated().map_or(0, |(count, _)| count)
    }

    /// EBU Tech 3342 loudness range since reset, LU; `None` until a short-term block passes both
    /// gates.
    #[must_use]
    pub fn loudness_range(&self) -> Option<f64> {
        let (count, energy) = self.range.total();
        if count == 0 {
            return None;
        }
        let gamma = level(energy / count as f64) - 20.0;
        let kept = || self.range.bins().filter(move |(_, _, mean)| *mean >= gamma);
        let n: u64 = kept().map(|(count, _, _)| count).sum();
        if n == 0 {
            return None;
        }
        // MATLAB `round((n - 1) p / 100 + 1)`, half away from zero, in integers.
        let rank = |percent: u64| ((n - 1) * percent + 100 + 50) / 100;
        let value_at = |rank: u64| {
            let mut seen = 0;
            for (count, _, mean) in kept() {
                seen += count;
                if seen >= rank {
                    return mean;
                }
            }
            f64::NAN
        };
        Some(value_at(rank(95)) - value_at(rank(10)))
    }

    /// Largest true-peak estimate per channel since reset, linear, `[left, right]`.
    #[must_use]
    pub fn true_peak(&self) -> [f32; 2] {
        self.true_peak
    }

    /// The first sample integration counts from, or `None` before the first kept hop.
    #[must_use]
    pub fn integrating_since(&self) -> Option<u64> {
        self.since
    }

    /// Non-finite input samples the render half replaced in kept hops since reset.
    #[must_use]
    pub fn sanitized_since_reset(&self) -> u64 {
        self.sanitized
    }
}
