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
//!
//! The meter frame ([`MeterFrame`], #1582 D2) is what a host publishes per closed window, and
//! [`fold_gain_reduction`] (#1582 D3) is its one gain-reduction fold: the strips' reduction words
//! from the armed observation taps a host visits through [`GainReductionTaps`].

use core::num::{NonZeroU32, NonZeroUsize};
use std::collections::TryReserveError;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use builtins::{MeterMetricSet, MeterTap};
use effect_contract::{BankWidth, ObservationDescriptor, ObservationKind, ParameterUnit};
use engine::realtime::{
    Consumer, ObservationReader, ObservationWindow, Producer, QueueGeneration, SpscRetainedPayload,
    bounded_spsc, bounded_spsc_retained_payload,
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
    /// The queue of closed windows, oldest first. The reader's owner pops it where it reads the
    /// windows, under its own drain bound (`available_at_entry`) when that is a render thread.
    pub fn windows_mut(&mut self) -> &mut Consumer<OutputPeakWindow> {
        &mut self.windows
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

/// The validity bits of a [`MeterFrame`] (#1582 D2). The browser's `METER_VALID_*` header bits are
/// these values.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MeterValidity(u32);

impl MeterValidity {
    /// No bit.
    pub const NONE: Self = Self(0);
    /// The frame is one complete window of every strip.
    pub const COMPLETE: Self = Self(1);
    /// The output peak covers exactly the frame's span.
    pub const OUTPUT: Self = Self(1 << 1);
    /// One or more windows were dropped or rejected since the previous frame of the delivery.
    pub const LOSS: Self = Self(1 << 2);
    /// At least one strip's gain reduction is present, from its taps' own latest windows, whose
    /// timing is independent of the frame's span.
    pub const GAIN_REDUCTION: Self = Self(1 << 3);

    /// The bits as a word.
    #[must_use]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Both sets of bits.
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Whether every bit of `other` is set.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

/// One published meter window (#1582 D2): every strip's peak pair, the output peak pair, every
/// strip's gain reduction, the span, the delivery's sequence, generation and loss count.
///
/// Strips are in [`SessionModel::strips`] order (the tracks, then the submixes), as the meter
/// selection ([`meter_selection`]) builds them. [`Self::new`] allocates the three per-strip
/// tables; nothing else does, so a frame is rewritten in place on a render thread.
#[derive(Clone, Debug, PartialEq)]
pub struct MeterFrame {
    /// The first sample of the window.
    pub first_sample: u64,
    /// One past the last sample of the window.
    pub end_sample: u64,
    /// The windows the frame folds (one per published frame today).
    pub windows: u32,
    /// The publication's sequence in its delivery, from `1`.
    pub sequence: u64,
    /// The delivery generation: a level transition or a producer reset advances it.
    pub generation: u64,
    /// The frame's validity bits.
    pub validity: MeterValidity,
    /// The losses counted since the previous frame of the delivery generation, saturating.
    pub loss_count: u64,
    /// Each strip's left and right sample peak.
    pub strip_peaks: Box<[[f32; 2]]>,
    /// The output's left and right sample peak.
    pub output_peak: [f32; 2],
    /// Each strip's gain reduction in non-negative decibels; `0.0` where no armed tap published.
    pub strip_gain_reduction_db: Box<[f32]>,
    /// Whether each strip's gain reduction came from at least one armed tap's window.
    pub strip_gain_reduction_present: Box<[bool]>,
    /// The designated strip's gain reduction, or `None` when no strip is designated or the
    /// designated strip published none (absent, not zero: zero reads as "not reducing").
    pub designated_gain_reduction_db: Option<f32>,
}

impl MeterFrame {
    /// An empty frame of `strip_count` strips. Allocates; call it on a control thread.
    ///
    /// # Errors
    ///
    /// A per-strip table could not be allocated.
    pub fn new(strip_count: usize) -> Result<Self, TryReserveError> {
        Ok(Self {
            first_sample: 0,
            end_sample: 0,
            windows: 0,
            sequence: 0,
            generation: 0,
            validity: MeterValidity::NONE,
            loss_count: 0,
            strip_peaks: filled(strip_count, [0.0; 2])?,
            output_peak: [0.0; 2],
            strip_gain_reduction_db: filled(strip_count, 0.0)?,
            strip_gain_reduction_present: filled(strip_count, false)?,
            designated_gain_reduction_db: None,
        })
    }

    /// The heap bytes [`Self::new`] allocated, so an owner charges them.
    #[must_use]
    pub fn heap_bytes(&self) -> usize {
        size_of_val::<[[f32; 2]]>(&self.strip_peaks)
            + size_of_val::<[f32]>(&self.strip_gain_reduction_db)
            + size_of_val::<[bool]>(&self.strip_gain_reduction_present)
    }
}

fn filled<T: Clone>(count: usize, value: T) -> Result<Box<[T]>, TryReserveError> {
    let mut table = Vec::new();
    table.try_reserve_exact(count)?;
    table.resize(count, value);
    Ok(table.into_boxed_slice())
}

/// One observation tap a host shows [`fold_gain_reduction`] (#1582 D3).
#[derive(Clone, Copy)]
pub struct TapView<'a> {
    /// The strip the tap's effect sits on, in [`SessionModel::strips`] order.
    pub strip: u32,
    /// The tap's declaration.
    pub descriptor: &'static ObservationDescriptor,
    /// The tap's reader.
    pub reader: &'a ObservationReader,
    /// Whether the tap is subscribed.
    pub armed: bool,
    /// The sample the tap's last subscribe applied at (`0` when none): an earlier window is no
    /// window ([`window_since_arm`]).
    pub arm_sample: u64,
}

/// The observation taps whose windows fold into a meter frame's gain reduction (#1582 D3). A host
/// implements it over its own tap state.
pub trait GainReductionTaps {
    /// Visit every observed tap of every strip once. Bounded by the plan's tap count, with no
    /// allocation, lock or panic: [`fold_gain_reduction`] calls it on a render thread.
    fn for_each_tap(&mut self, visit: &mut impl FnMut(TapView<'_>));
}

// REALTIME_POLICY_BEGIN: #1582 D3, the meter frame's gain-reduction fold.
/// Rewrite `frame`'s gain reduction from the armed taps' latest windows (#1582 D3).
///
/// Clears every strip's word and presence, then, for each armed tap whose latest window opened at
/// or after its arm sample ([`window_since_arm`]), acknowledges the window and folds its
/// [`strip_reduction_db`] into the tap's strip word by maximum (from `0.0`). A strip outside the
/// frame is skipped. `designated_gain_reduction_db` becomes the `designated` strip's word when
/// that strip is present, and [`MeterValidity::GAIN_REDUCTION`] is set exactly when some tap
/// folded. Allocation-free, no runtime-checked index, bounded by the visited taps.
#[inline]
pub fn fold_gain_reduction(
    frame: &mut MeterFrame,
    taps: &mut impl GainReductionTaps,
    designated: Option<u32>,
) {
    for slot in &mut frame.strip_gain_reduction_db {
        *slot = 0.0;
    }
    for present in &mut frame.strip_gain_reduction_present {
        *present = false;
    }
    let mut observed_any = false;
    let gain = &mut frame.strip_gain_reduction_db;
    let presence = &mut frame.strip_gain_reduction_present;
    taps.for_each_tap(&mut |tap| {
        if !tap.armed {
            return;
        }
        // Issue #1534 D1-D4: a window from before the tap's last subscribe is no window. It
        // writes no slot, marks nothing present and is not acknowledged; the first fresh window's
        // acknowledgement covers its sequence.
        let Some(window) = tap
            .reader
            .read()
            .and_then(|window| window_since_arm(window, tap.arm_sample))
        else {
            return;
        };
        tap.reader.acknowledge(window.sequence);
        let value = strip_reduction_db(*tap.descriptor, window.left, window.right);
        // Several armed taps on one strip fold max-magnitude into the strip's one word.
        if let Some(slot) = gain.get_mut(tap.strip as usize)
            && value > *slot
        {
            *slot = value;
        }
        if let Some(present) = presence.get_mut(tap.strip as usize) {
            *present = true;
        }
        observed_any = true;
    });
    // A designation, not a discovery: absent, not zero, when no strip is designated or the
    // designated strip published nothing.
    frame.designated_gain_reduction_db = designated.and_then(|strip| {
        let strip = strip as usize;
        if frame
            .strip_gain_reduction_present
            .get(strip)
            .copied()
            .unwrap_or(false)
        {
            frame.strip_gain_reduction_db.get(strip).copied()
        } else {
            None
        }
    });
    frame.validity = if observed_any {
        frame.validity.union(MeterValidity::GAIN_REDUCTION)
    } else {
        MeterValidity(frame.validity.0 & !MeterValidity::GAIN_REDUCTION.0)
    };
}

/// The one rule for whether an armed tap's latest window counts (issue #1534 D5): it counts only
/// if it opened at or after the tap's last subscribe, `arm_sample` (`0` when none). An
/// observation cell conflates and keeps its last window for ever, so a window from before a
/// re-subscribe or a re-arm is a stale reading of the previous arm and is the same as no window.
/// The meter frame's fold and a host's selected read both call it, so they cannot drift apart.
#[inline]
#[must_use]
pub fn window_since_arm(window: ObservationWindow, arm_sample: u64) -> Option<ObservationWindow> {
    (window.first_sample >= arm_sample).then_some(window)
}

/// One closed window's contribution to its strip's gain-reduction word: non-negative decibels of
/// reduction, the deeper of the two lanes.
///
/// A `GainReductionDb` tap contributes its reduction as published. A `GainChangeDb` tap
/// (issue #1918) publishes a signed change, positive for a boost; only a cut is a reduction, so it
/// contributes the cut's depth `max(0, -x)` and a boost or a non-finite lane contributes `0`. The
/// signed value itself reaches a consumer through the selected read, never through this word.
#[must_use]
pub fn strip_reduction_db(tap: ObservationDescriptor, left: f32, right: f32) -> f32 {
    match tap.kind {
        ObservationKind::GainReductionDb => {
            reduction_db(tap, if left > right { left } else { right })
        }
        // A non-finite lane is no reading: the clamp in `reduction_db` would turn `+inf` into the
        // declared minimum, a full-depth cut the effect never made.
        ObservationKind::GainChangeDb => {
            let cut = |change: f32| {
                if !change.is_finite() {
                    return 0.0;
                }
                let change = reduction_db(tap, change);
                if change < 0.0 { -change } else { 0.0 }
            };
            let (left, right) = (cut(left), cut(right));
            if left > right { left } else { right }
        }
    }
}

/// One published observation value in decibels: a reduction for a `GainReductionDb` tap, a signed
/// change for a `GainChangeDb` tap.
///
/// The tap declares what crosses the transport (`unit`) and what a consumer reads (`display_unit`,
/// `minimum`, `maximum`); this is the one place the two are reconciled, and it runs once per closed
/// window on the control plane. That placement is the whole of #143 R4: the true-peak limiter's
/// `d` is a linear reduction word and turning it into decibels needs a logarithm, which a render
/// kernel may not take.
///
/// The result is clamped into the tap's own declared range, so a consumer never has to guess what
/// a number outside it would have meant.
#[must_use]
pub fn reduction_db(tap: ObservationDescriptor, magnitude: f32) -> f32 {
    let decibels = match tap.unit {
        // Already decibels: a `PeakMagnitude` fold made it non-negative, a `PeakSigned` fold kept
        // its sign, and the clamp below honours either declared range.
        ParameterUnit::Db => magnitude,
        // `gain = 1 - d`, so the reduction in decibels is `-20 log10(1 - d)`. `d == 1` is total
        // reduction and has no finite decibel value; the declared maximum is what a meter draws.
        ParameterUnit::Linear => {
            if magnitude <= 0.0 {
                0.0
            } else if magnitude >= 1.0 {
                tap.maximum
            } else {
                -20.0 * math::log10f(1.0 - magnitude)
            }
        }
        // No launch tap declares another unit, and a menu that did would be describing something
        // this word has no meaning for.
        _ => 0.0,
    };
    if !decibels.is_finite() || decibels < tap.minimum {
        tap.minimum
    } else if decibels > tap.maximum {
        tap.maximum
    } else {
        decibels
    }
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
