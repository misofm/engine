//! The one strip meter selection and its window (issue #1581 D1/D2, decision 17 C1/C13).
//!
//! Every preparation entry that attaches live controls meters the same strips the same way: one
//! [`MeterTap::PostMatrix`] [`MeterMetricSet::SAMPLE_PEAK`] meter per strip, in
//! [`SessionModel::strips`] order (the tracks, then the submixes), so a host reads strip `i`'s
//! left and right peak from meter `i`. A host does not choose its own list.
//!
//! The output meter ([`output_peak_meter`], #1581 D3) is the core's one windowed left and right
//! peak of the rendered output: the control plane's plan state owns its render half, and a host
//! reads its windows through the reader.

use core::num::{NonZeroU32, NonZeroUsize};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use builtins::{MeterMetricSet, MeterTap};
use effect_contract::BankWidth;
use engine::realtime::{
    Consumer, Producer, QueueGeneration, SpscRetainedPayload, bounded_spsc,
    bounded_spsc_retained_payload,
};
use lane::{Backend, Lane, kernels::builtins::meter_sample_peak_block};
use session::SessionModel;

use crate::prepare::HostMeterRequest;

/// The depth of the output meter's window queue (#1581 D3): eight closed windows.
pub const METER_QUEUE_DEPTH: usize = 8;

/// The gate word's value while the output meter is inactive: no sample is at or past it.
const INACTIVE: u64 = u64::MAX;

/// One closed output meter window (#1581 D3): the samples `[start_sample, end_sample)` and the
/// largest sanitized magnitude of each output plane over them, left then right.
///
/// A sanitized magnitude is `|x|` for a finite, non-subnormal `x` and `+0.0` for anything else
/// (NaN, an infinity or a subnormal), the builtin meter's rule
/// ([`meter_sample_peak_block`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OutputPeakWindow {
    /// The first sample of the window.
    pub start_sample: u64,
    /// One past the last sample of the window.
    pub end_sample: u64,
    /// The left and right peaks.
    pub peak: [f32; 2],
}

/// The control half of the output meter's gate (#1581 D3): the first absolute sample the meter
/// may observe, or [`u64::MAX`] while it is inactive (the start value).
pub struct OutputPeakGate(Arc<AtomicU64>);

impl OutputPeakGate {
    /// Let the meter observe every block that starts at or after `from_sample`. A window opens at
    /// the first such block. A new activation discards a window the meter has open.
    pub fn activate(&self, from_sample: u64) {
        self.0.store(from_sample, Ordering::Release);
    }

    /// Stop the meter. Its next block discards the window it has open and folds nothing.
    pub fn deactivate(&self) {
        self.0.store(INACTIVE, Ordering::Release);
    }
}

/// The reading half of the output meter (#1581 D3): the closed windows, oldest first, and how many
/// closed windows a full queue dropped.
pub struct OutputPeakReader {
    windows: Consumer<OutputPeakWindow>,
    dropped: Arc<AtomicU64>,
}

impl OutputPeakReader {
    /// The oldest closed window, left in the queue.
    #[must_use]
    pub fn peek(&self) -> Option<&OutputPeakWindow> {
        self.windows.peek(0)
    }

    /// Take the oldest closed window.
    pub fn try_pop(&mut self) -> Option<OutputPeakWindow> {
        self.windows.try_pop().ok()
    }

    /// How many closed windows the queue holds now.
    #[must_use]
    pub fn available(&self) -> usize {
        self.windows.available_at_entry()
    }

    /// How many closed windows render dropped because the queue was full: monotone, never reset.
    #[must_use]
    pub fn dropped_windows(&self) -> u64 {
        self.dropped.load(Ordering::Acquire)
    }
}

/// The render half of the output meter (#1581 D3). Its owner calls [`Self::observe`] once after
/// every rendered block.
pub struct OutputPeakObserver {
    gate: Arc<AtomicU64>,
    windows: Producer<OutputPeakWindow>,
    dropped: Arc<AtomicU64>,
    window_frames: u64,
    /// The gate value the open window was opened under, or [`INACTIVE`] while no window is open.
    open_gate: u64,
    start_sample: u64,
    peak: [f32; 2],
}

/// The output meter's three halves (#1581 D3), with windows of at least `window_frames` frames.
///
/// The gate starts inactive. Allocates the window queue and two shared words; call it on a control
/// thread.
///
/// # Errors
///
/// The window queue could not be made.
pub fn output_peak_meter(
    window_frames: NonZeroU32,
) -> Result<(OutputPeakObserver, OutputPeakReader, OutputPeakGate), engine::realtime::SpscError> {
    let (producer, consumer) = bounded_spsc(output_peak_queue_depth(), QueueGeneration(0))?;
    let gate = Arc::new(AtomicU64::new(INACTIVE));
    let dropped = Arc::new(AtomicU64::new(0));
    Ok((
        OutputPeakObserver {
            gate: Arc::clone(&gate),
            windows: producer,
            dropped: Arc::clone(&dropped),
            window_frames: u64::from(window_frames.get()),
            open_gate: INACTIVE,
            start_sample: 0,
            peak: [0.0; 2],
        },
        OutputPeakReader {
            windows: consumer,
            dropped,
        },
        OutputPeakGate(gate),
    ))
}

/// The exact layouts of the output meter's window queue, so an owner charges what
/// [`output_peak_meter`] allocates beside its two shared words (`Arc<AtomicU64>` each).
///
/// # Errors
///
/// The layout arithmetic overflowed.
pub fn output_peak_queue_payload() -> Result<SpscRetainedPayload, engine::realtime::SpscError> {
    bounded_spsc_retained_payload::<OutputPeakWindow>(output_peak_queue_depth())
}

fn output_peak_queue_depth() -> NonZeroUsize {
    NonZeroUsize::new(METER_QUEUE_DEPTH).unwrap_or(NonZeroUsize::MIN)
}

// REALTIME_POLICY_BEGIN: #1581 D3, the output meter's render half.
impl OutputPeakObserver {
    /// Observe one rendered block, `[first_sample, next_sample)`, from its two output planes.
    ///
    /// Loads the gate once. While it is inactive, or the block starts before it, the call folds
    /// nothing and discards a window that is open. Otherwise it folds each plane's sanitized peak
    /// into the open window (opening one at this block if none is), and closes the window once
    /// `next_sample - start >= window_frames`: a closed window goes into the queue, or, when the
    /// queue is full, is dropped and counted. Allocation-free and bounded by the block length.
    pub fn observe(&mut self, left: &[f32], right: &[f32], first_sample: u64, next_sample: u64) {
        let gate = self.gate.load(Ordering::Acquire);
        if gate == INACTIVE || first_sample < gate {
            self.open_gate = INACTIVE;
            return;
        }
        if self.open_gate != gate {
            self.open_gate = gate;
            self.start_sample = first_sample;
            self.peak = [0.0; 2];
        }
        self.peak = [
            plane_peak(left, self.peak[0]),
            plane_peak(right, self.peak[1]),
        ];
        if next_sample.saturating_sub(self.start_sample) >= self.window_frames {
            let window = OutputPeakWindow {
                start_sample: self.start_sample,
                end_sample: next_sample,
                peak: self.peak,
            };
            if self.windows.try_push(window).is_err() {
                self.dropped.fetch_add(1, Ordering::AcqRel);
            }
            self.open_gate = INACTIVE;
        }
    }
}

/// The largest of `seed` and every sanitized magnitude of `plane`, at the build's own lane width.
#[inline]
fn plane_peak(plane: &[f32], seed: f32) -> f32 {
    // The build's own width as a constant, so only its fold is instantiated (as the native effect
    // banks choose theirs); `None` is a build with no vector width, which no shipped target is.
    const NATIVE: Option<BankWidth> = BankWidth::for_backend(Backend::current());
    match NATIVE {
        Some(width) => effect_contract::match_bank_width!(width, |L| fold_peak::<L>(plane, seed)),
        None => fold_peak::<f32>(plane, seed),
    }
}

/// [`meter_sample_peak_block`] over `plane` read as consecutive `L::WIDTH`-sample chunks, the
/// lanes' maximum, then the tail with the same kernel at `f32`. The select-form maximum over
/// sanitized magnitudes is commutative and associative, so the order is free and the result is the
/// scalar sample-serial fold's, bit for bit.
///
/// Every slice comes from `chunks_exact` and every kernel call reads exactly one frame of it, so
/// no bounds check or slice-index panic joins the render call graph.
#[inline(always)]
fn fold_peak<L: Lane>(plane: &[f32], seed: f32) -> f32 {
    let mut chunks = plane.chunks_exact(L::WIDTH);
    let mut lanes_peak = L::zero();
    for chunk in &mut chunks {
        lanes_peak = meter_sample_peak_block::<L>(chunk, 1, lanes_peak);
    }
    let mut lanes = [0.0_f32; 8];
    lanes_peak.store(&mut lanes);
    // Lanes past `L::WIDTH` stay `+0.0`, the fold's identity.
    let mut peak = seed;
    for &lane in &lanes {
        peak = <f32 as Lane>::max(lane, peak);
    }
    for sample in chunks.remainder() {
        peak = meter_sample_peak_block::<f32>(core::slice::from_ref(sample), 1, peak);
    }
    peak
}
// REALTIME_POLICY_END

/// The strip meters every live-control preparation builds (issue #1581 D1): one per strip, in
/// [`SessionModel::strips`] order, each a post-matrix left and right sample peak.
#[must_use]
pub fn meter_selection(model: &SessionModel) -> Vec<HostMeterRequest> {
    model
        .strips()
        .map(|strip| HostMeterRequest {
            strip_id: strip.id.as_str().into(),
            tap: MeterTap::PostMatrix,
            metrics: MeterMetricSet::SAMPLE_PEAK,
        })
        .collect()
}

/// The meter window, in blocks, that a request's `meter_window_blocks == 0` stands for (issue
/// #1581 D2): the whole number of blocks nearest to 32 ms, and at least one.
///
/// `max(1, (rate_hz * 32 + 500 * quantum) / (1000 * quantum))`, in `u64`: 12 blocks at 48 kHz and
/// 128 frames (the browser's present default), 11 at 44.1 kHz, 24 at 96 kHz, 3 at 512 frames.
/// A `quantum_frames` of zero, which no prepared plan has, gives one block.
#[must_use]
pub fn default_meter_window_blocks(rate_hz: u32, quantum_frames: u32) -> NonZeroU32 {
    let quantum = u64::from(quantum_frames);
    let blocks = (u64::from(rate_hz) * 32 + 500 * quantum)
        .checked_div(1000 * quantum)
        .unwrap_or(1);
    u32::try_from(blocks)
        .ok()
        .and_then(NonZeroU32::new)
        .unwrap_or(NonZeroU32::MIN)
}

#[cfg(test)]
mod tests {
    use super::default_meter_window_blocks;

    /// Gate 5: the default window is the nearest whole block count to 32 ms.
    #[test]
    fn the_default_window_is_the_nearest_block_count_to_32_ms() {
        for (rate, quantum, blocks) in [
            (48_000, 128, 12),
            (44_100, 128, 11),
            (88_200, 128, 22),
            (96_000, 128, 24),
            (48_000, 512, 3),
            (48_000, 4096, 1),
        ] {
            assert_eq!(
                default_meter_window_blocks(rate, quantum).get(),
                blocks,
                "{rate} Hz at {quantum} frames"
            );
        }
    }
}
