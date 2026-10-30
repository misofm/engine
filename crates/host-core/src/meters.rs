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
//!
//! The meter delivery ([`MeterDelivery`], #1582 D1) is a session's one fold of the strip meters'
//! and the output meter's closed windows into that frame, with the meter level, the window grid,
//! the generation and the loss count; a host calls it and copies the frame it publishes.

use core::num::{NonZeroU32, NonZeroUsize};
use std::collections::TryReserveError;
use std::sync::{
    Arc,
    atomic::{AtomicU32, AtomicU64, Ordering},
};

use builtins::{MeterMetricSet, MeterSnapshot, MeterTap};
use builtins_compiler::MeterConsumer;
use effect_contract::{BankWidth, ObservationDescriptor, ObservationKind, ParameterUnit};
use engine::realtime::{
    Consumer, ObservationReader, ObservationWindow, Producer, QueueGeneration, SpscRetainedPayload,
    bounded_spsc, bounded_spsc_move, bounded_spsc_retained_payload,
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

/// The render half of a requested output gate (#1583): a host whose meter delivery runs on a
/// control thread hands the delivery the [`OutputPeakGate`] of [`output_peak_gate_relay`], which
/// writes only a *requested* gate word, and keeps this relay with the output meter on the render
/// thread. [`Self::apply`] copies a changed request into the meter's own gate once, before a block,
/// so the gate moves only on the render thread and between blocks, and the strip meters and the
/// output meter judge every block by the one gate value.
///
/// The relay is the gate's one writer, and it applies a request only when the request *changes*:
/// a render-side move of the gate (#1587a: the adoption block of a swap that carried no strip
/// meter, [`Self::restart_at`]) writes the gate and leaves `applied` as it is, so an unchanged
/// request never undoes it, and the control thread's next request (its `align` at the adoption
/// sample) asks for the value the gate holds.
///
/// **On the strip meters' grid (#1587a).** The relay keeps the grid the strip meters tile on the
/// render thread: its origin (sample 0, or the adoption block of the last swap that carried no
/// strip meter) and the window. The control thread computes a request from a render clock it read
/// earlier, and on the grid it last adopted, so a request can be behind the block it is applied
/// in, or on a grid render has already restarted. [`Self::apply`] puts every gate it writes on the
/// render grid, at or after the block, so the output meter never opens a window off the strip
/// meters' grid.
pub struct OutputPeakGateRelay {
    requested: Arc<AtomicU64>,
    gate: OutputPeakGate,
    /// The request the relay last copied into the gate.
    applied: u64,
    /// The first sample of the strip meters' window grid on the render thread.
    origin: u64,
    /// The meter window in frames, `0` with none.
    window_frames: u64,
}

/// A requested gate over the output meter's `gate` (#1583): the [`OutputPeakGate`] a control-thread
/// delivery writes (a request word that starts inactive) and the [`OutputPeakGateRelay`] the render
/// thread applies it with, on a window grid of `window_frames` frames from sample 0 (`0` with no
/// window). Allocates one shared word; call it on a control thread.
#[must_use]
pub fn output_peak_gate_relay(
    gate: OutputPeakGate,
    window_frames: u64,
) -> (OutputPeakGate, OutputPeakGateRelay) {
    let requested = Arc::new(AtomicU64::new(INACTIVE));
    (
        OutputPeakGate(Arc::clone(&requested)),
        OutputPeakGateRelay {
            requested,
            gate,
            applied: INACTIVE,
            origin: 0,
            window_frames,
        },
    )
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

    /// A second reader of [`Self::dropped_windows`] (#1588 R4), for a control half that hands the
    /// reader to a render-half delivery and still reports the windows a full queue dropped.
    #[must_use]
    pub fn drop_count(&self) -> OutputPeakDrops {
        OutputPeakDrops(Arc::clone(&self.dropped))
    }
}

/// A reader of the output meter's dropped-window count ([`OutputPeakReader::drop_count`]).
#[derive(Clone, Debug)]
pub struct OutputPeakDrops(Arc<AtomicU64>);

impl OutputPeakDrops {
    /// How many closed windows render dropped because the queue was full: monotone, never reset.
    #[must_use]
    pub fn get(&self) -> u64 {
        self.0.load(Ordering::Acquire)
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
impl OutputPeakGateRelay {
    /// Apply a changed gate request before the block that starts at `block_start` (#1583,
    /// #1587a). Call it on the render thread before the block, before
    /// [`OutputPeakObserver::admits`]. A request that did not change since the last call does
    /// nothing. Otherwise:
    ///
    /// * an inactive request shuts the gate;
    /// * a request at or after the block opens the gate at the render grid's first boundary at or
    ///   after it (the request itself when the control thread's grid is render's);
    /// * a request behind the block keeps a gate that is already open at or before the block (it
    ///   is on the render grid: this relay put it there), and otherwise opens the gate at the
    ///   grid's first boundary at or after the block.
    ///
    /// Integer arithmetic on the grid's origin and window, one `Acquire` load of the request, at
    /// most one of the gate and one `Release` store; no allocation.
    pub fn apply(&mut self, block_start: u64) {
        let requested = self.requested.load(Ordering::Acquire);
        if requested == self.applied {
            return;
        }
        self.applied = requested;
        let current = self.gate.0.load(Ordering::Acquire);
        let gate = if requested == INACTIVE {
            INACTIVE
        } else if requested >= block_start {
            self.grid_boundary(requested)
        } else if current != INACTIVE && current <= block_start {
            current
        } else {
            self.grid_boundary(block_start)
        };
        if gate != current {
            self.gate.0.store(gate, Ordering::Release);
        }
    }

    /// The render grid's first boundary at or after `sample`: `origin + k * window_frames`, or
    /// `sample` itself with no window.
    fn grid_boundary(&self, sample: u64) -> u64 {
        if self.window_frames == 0 {
            return sample;
        }
        if sample <= self.origin {
            return self.origin;
        }
        let remainder = (sample - self.origin) % self.window_frames;
        if remainder == 0 {
            sample
        } else {
            sample.saturating_add(self.window_frames - remainder)
        }
    }

    /// Restart the grid at `adoption_sample`, the first sample of a block that adopted a successor
    /// whose strip meters were not carried (#1587a): the strip meters open their first window
    /// there. Call it on the render thread after the swap and before the block's output fold. The
    /// grid's origin moves there; an active gate, open or waiting, moves there too, and an
    /// inactive one stays shut. This is the control thread's
    /// rule too: [`MeterDelivery::adopt`] moves the grid's origin there, and its `align` at the
    /// same sample asks for that same value. `applied` does not change, so an unchanged request
    /// never undoes the move, and the request the control thread makes after it asks for the
    /// value the gate already holds. One `Acquire` load and at most one `Release` store.
    pub fn restart_at(&mut self, adoption_sample: u64) {
        self.origin = adoption_sample;
        if self.gate.0.load(Ordering::Acquire) != INACTIVE {
            self.gate.0.store(adoption_sample, Ordering::Release);
        }
    }
}

impl OutputPeakObserver {
    /// Whether [`Self::observe`] would fold the block that starts at `first_sample` (#882): the
    /// gate is active and the block starts at or after it. One `Acquire` load; the render thread's
    /// owner asks it before the block so the strip meters observe exactly the blocks the output
    /// meter does. The gate moves only on the render thread between blocks
    /// ([`MeterDelivery::align`]), except at a swap that restarts the grid, which
    /// [`Self::admits_restart_block`] answers.
    #[must_use]
    pub fn admits(&self, first_sample: u64) -> bool {
        let gate = self.gate.load(Ordering::Acquire);
        gate != INACTIVE && first_sample >= gate
    }

    /// Whether [`Self::observe`] will fold a block that adopts a successor whose strip meters were
    /// not carried (#882): such a swap restarts the meter grid at its adoption block, and a gate
    /// that is active, open or still waiting for its activation, moves there before the block is
    /// observed ([`MeterDelivery::adopt`]), so the block is admitted exactly when the gate is
    /// active. One `Acquire` load, asked before the block beside [`Self::admits`].
    #[must_use]
    pub fn admits_restart_block(&self) -> bool {
        self.gate.load(Ordering::Acquire) != INACTIVE
    }

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
    /// The provider epoch of the plan whose strips the frame covers (#1588 D3): its
    /// [`MeterTables`]' epoch ([`MeterTables::set_plan_epoch`]), `0` for the boot plan. A reader
    /// that holds the strip list of one epoch resolves its strips again when a frame names another.
    /// Both hosts fill it at each adoption; the browser's render half also checks it against the
    /// epoch of the plan ring it pushes the frame into (#1588b).
    pub plan_epoch: u64,
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
            plan_epoch: 0,
            validity: MeterValidity::NONE,
            loss_count: 0,
            strip_peaks: filled(strip_count, [0.0; 2])?,
            output_peak: [0.0; 2],
            strip_gain_reduction_db: filled(strip_count, 0.0)?,
            strip_gain_reduction_present: filled(strip_count, false)?,
        })
    }

    /// The master strip's gain reduction (#1690 D1): strip `master`'s word when that strip is in
    /// the frame and an armed tap's window folded into it, else `None` -- absent, not zero, when
    /// no strip is the master or the master published nothing (zero reads as "not reducing").
    /// The browser's lease frame reads it through this function; every `METER_BATCH`, on both
    /// hosts, reads the shared gain row of [`MeterBatchFold::push_with_strips`], with the same
    /// predicate.
    #[must_use]
    #[inline]
    pub fn master_gain_reduction(&self, master: Option<u32>) -> Option<f32> {
        let strip = usize::try_from(master?).ok()?;
        if self.strip_gain_reduction_present.get(strip).copied()? {
            self.strip_gain_reduction_db.get(strip).copied()
        } else {
            None
        }
    }

    /// The heap bytes [`Self::new`] allocated, so an owner charges them.
    #[must_use]
    pub fn heap_bytes(&self) -> usize {
        size_of_val::<[[f32; 2]]>(&self.strip_peaks)
            + size_of_val::<[f32]>(&self.strip_gain_reduction_db)
            + size_of_val::<[bool]>(&self.strip_gain_reduction_present)
    }

    /// [`Self::heap_bytes`] of `Self::new(strip_count)`, before it is built (issue #2199).
    #[must_use]
    pub const fn projected_heap_bytes(strip_count: usize) -> usize {
        strip_count * (size_of::<[f32; 2]>() + size_of::<f32>() + size_of::<bool>())
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
    /// The frame's strip index of the strip the tap's effect sits on: its position in
    /// [`MeterFrame::strip_peaks`]. An index outside the frame is skipped.
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
/// frame is skipped. [`MeterValidity::GAIN_REDUCTION`] is set exactly when some armed tap had a
/// window since its arm, including a tap whose strip is outside the frame, which marks no strip; the
/// master's reading is [`MeterFrame::master_gain_reduction`] of the folded frame.
/// Allocation-free, no runtime-checked index, bounded by the visited taps.
#[inline]
pub fn fold_gain_reduction(frame: &mut MeterFrame, taps: &mut impl GainReductionTaps) {
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
    frame.validity = if observed_any {
        frame.validity.union(MeterValidity::GAIN_REDUCTION)
    } else {
        // The caller sets `validity` afresh before each fold ([`MeterDelivery::poll`]).
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

/// One plan's meter delivery tables (#1582 R1): per strip meter a pending snapshot, a pop budget
/// and the handle its carried predecessor published under, and the plan's [`MeterFrame`].
///
/// A host builds a plan's tables with the plan, on a control thread ([`Self::new`] allocates),
/// and hands them to [`MeterDelivery::adopt`] at the block that adopts the plan, which exchanges
/// them by pointer: nothing allocates or frees on the render thread when a swap changes the strip
/// count.
pub struct MeterTables {
    /// One bounded pending snapshot per strip meter. Fixed at preparation; the meter's queue is
    /// the only producer-side buffer.
    pending: Box<[Option<MeterSnapshot>]>,
    /// One count per strip meter, read from its queue at each poll's entry and decremented before
    /// each pop (issue #1448 D2). Scratch for one poll: every poll sets every entry before it
    /// pops, so delivery resets do not touch it.
    remaining: Box<[usize]>,
    /// Per meter, the handle its carried predecessor published under, or `0` (#1290d1). A swap
    /// pairs a meter with the predecessor's at a strip identity, and a strip added or removed in
    /// front shifts the handles (a handle is a position), so a window the predecessor closed
    /// before the swap and the poll finds after it names the predecessor's handle. The poll
    /// accepts it for this meter, for the plan's life: a meter's queue is its strip's alone. A
    /// window two swaps old names a handle this table no longer holds and is dropped with a count.
    carried_from: Box<[u64]>,
    /// The plan's frame: the delivery's last publication while the plan renders.
    frame: MeterFrame,
}

impl MeterTables {
    /// The tables of a plan with `meter_count` strip meters (one per strip, or none when the plan
    /// meters nothing) and `strip_count` strips. Allocates; call it on a control thread.
    ///
    /// # Errors
    ///
    /// A table could not be allocated.
    pub fn new(meter_count: usize, strip_count: usize) -> Result<Self, TryReserveError> {
        Ok(Self {
            pending: filled(meter_count, None)?,
            remaining: filled(meter_count, 0)?,
            carried_from: filled(meter_count, 0)?,
            frame: MeterFrame::new(strip_count)?,
        })
    }

    /// The tables of a plan that meters nothing and has no strip: they allocate nothing, so
    /// they cannot fail.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            pending: Box::default(),
            remaining: Box::default(),
            carried_from: Box::default(),
            frame: MeterFrame {
                first_sample: 0,
                end_sample: 0,
                windows: 0,
                sequence: 0,
                generation: 0,
                plan_epoch: 0,
                validity: MeterValidity::NONE,
                loss_count: 0,
                strip_peaks: Box::default(),
                output_peak: [0.0; 2],
                strip_gain_reduction_db: Box::default(),
                strip_gain_reduction_present: Box::default(),
            },
        }
    }

    /// Name the plan the tables serve by its provider epoch (#1588 D3): every frame published
    /// from them carries it in [`MeterFrame::plan_epoch`]. Set before or at the adoption: the C
    /// ABI sets it on its control thread before the tables reach [`MeterDelivery::adopt`], the
    /// browser on its render thread at the adoption block, just before. An adoption keeps the
    /// successor's epoch. A scalar store; allocation-free.
    pub fn set_plan_epoch(&mut self, epoch: u64) {
        self.frame.plan_epoch = epoch;
    }

    /// The strip meters the tables serve.
    #[must_use]
    pub fn meter_count(&self) -> usize {
        self.pending.len()
    }

    /// The plan's frame.
    #[must_use]
    pub fn frame(&self) -> &MeterFrame {
        &self.frame
    }

    /// The strip meters' pending snapshots: a group the poll has begun to assemble.
    #[must_use]
    pub fn pending(&self) -> &[Option<MeterSnapshot>] {
        &self.pending
    }

    /// The heap bytes [`Self::new`] allocated, so an owner charges them.
    #[must_use]
    pub fn heap_bytes(&self) -> usize {
        size_of_val::<[Option<MeterSnapshot>]>(&self.pending)
            + size_of_val::<[usize]>(&self.remaining)
            + size_of_val::<[u64]>(&self.carried_from)
            + self.frame.heap_bytes()
    }

    /// [`Self::heap_bytes`] and [`Self::largest_allocation_bytes`] of
    /// `Self::new(meter_count, strip_count)`, before it is built (issue #2199).
    #[must_use]
    pub const fn projected_bytes(meter_count: usize, strip_count: usize) -> (usize, usize) {
        let pending = meter_count * size_of::<Option<MeterSnapshot>>();
        let heap = pending
            + meter_count * (size_of::<usize>() + size_of::<u64>())
            + MeterFrame::projected_heap_bytes(strip_count);
        let frame_largest = strip_count * size_of::<[f32; 2]>();
        (
            heap,
            if pending > frame_largest {
                pending
            } else {
                frame_largest
            },
        )
    }

    /// The largest single allocation of [`Self::heap_bytes`].
    #[must_use]
    pub fn largest_allocation_bytes(&self) -> usize {
        size_of_val::<[Option<MeterSnapshot>]>(&self.pending)
            .max(size_of_val::<[usize]>(&self.remaining))
            .max(size_of_val::<[u64]>(&self.carried_from))
            .max(size_of_val::<[[f32; 2]]>(&self.frame.strip_peaks))
            .max(size_of_val::<[f32]>(&self.frame.strip_gain_reduction_db))
            .max(size_of_val::<[bool]>(
                &self.frame.strip_gain_reduction_present,
            ))
    }
}

/// One strip meter a swap carried (#1582 R1, #1290d1): the successor plan's meter position, the
/// predecessor's, and the handles the two publish under.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MeterCarry {
    /// The meter's position in the successor plan.
    pub successor: u32,
    /// The meter's position in the predecessor plan.
    pub predecessor: u32,
    /// The handle the successor's meter publishes under.
    pub successor_handle: u64,
    /// The handle the predecessor's meter published under.
    pub predecessor_handle: u64,
}

/// The one meter delivery of a session (#1582 D1, amended by R1): it folds the plan's strip
/// meters' closed windows and the output meter's into one [`MeterFrame`] per window, and holds
/// the meter level (on or off), its activation on the strip meters' window grid, the delivery
/// generation and the loss count.
///
/// It owns the output meter's reader and gate and the rendering plan's [`MeterTables`]; it
/// borrows the plan's meter consumers per [`Self::poll`], because they are the plan's.
pub struct MeterDelivery {
    output: OutputPeakReader,
    gate: OutputPeakGate,
    /// The first sample the gate admits, or `u64::MAX` while it is inactive: the delivery's own
    /// copy of the word it last stored.
    gate_from: u64,
    /// Closed output windows at the queue's head that a delivery reset discarded: the next poll
    /// pops them unread and uncounted, as the reset of the ring they replace cleared it.
    output_discard: usize,
    /// The reader's dropped-window count already folded into `loss_count`.
    output_dropped_seen: u64,
    /// The meter window in frames, `0` with none.
    window_frames: u64,
    /// The first sample of the strip meters' window grid (#1581 D5): the strip meters tile
    /// `[origin + k * window, ..)` from the block their plan first rendered, the boot block or the
    /// adoption block of the last swap that carried no meter (a carried swap keeps the grid).
    grid_origin: u64,
    /// The first sample the poll accepts strip windows from (#1581 D5).
    activation: u64,
    /// Whether the meter level is on.
    level: bool,
    /// The delivery generation, advanced on level transitions, producer resets and uncarried
    /// swaps.
    generation: u64,
    /// Losses and rejected stale or mismatched windows waiting to be reported in the next
    /// published frame.
    loss_count: u64,
    snapshot_drops_seen: u64,
    snapshot_generation: Option<u64>,
    /// Windows folded since the level last turned on or off.
    folded_windows: u64,
    tables: MeterTables,
}

impl MeterDelivery {
    /// A delivery over the output meter's `output` reader and `gate`, with windows of
    /// `window_frames` frames (`0` with no meter window) and the boot plan's `tables`. The level
    /// starts off and the grid at sample 0, the boot plan's first block. Allocates nothing.
    #[must_use]
    pub fn new(
        output: OutputPeakReader,
        gate: OutputPeakGate,
        window_frames: u64,
        tables: MeterTables,
    ) -> Self {
        Self {
            output,
            gate,
            gate_from: INACTIVE,
            output_discard: 0,
            output_dropped_seen: 0,
            window_frames,
            grid_origin: 0,
            activation: 0,
            level: false,
            generation: 0,
            loss_count: 0,
            snapshot_drops_seen: 0,
            snapshot_generation: None,
            folded_windows: 0,
            tables,
        }
    }

    /// The last published frame (the rendering plan's).
    #[must_use]
    pub fn frame(&self) -> &MeterFrame {
        &self.tables.frame
    }

    /// The rendering plan's tables.
    #[must_use]
    pub fn tables(&self) -> &MeterTables {
        &self.tables
    }

    /// Whether the level is on.
    #[must_use]
    pub fn level(&self) -> bool {
        self.level
    }

    /// The delivery generation.
    #[must_use]
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// The losses waiting for the next published frame.
    #[must_use]
    pub fn loss_count(&self) -> u64 {
        self.loss_count
    }

    /// The first sample the output meter's gate admits, or `u64::MAX` while it is inactive.
    #[must_use]
    pub fn output_gate_from(&self) -> u64 {
        self.gate_from
    }

    /// Windows folded since the level last turned on or off.
    #[must_use]
    pub fn folded_windows(&self) -> u64 {
        self.folded_windows
    }

    /// How many closed output windows render dropped because the queue was full: the reader's
    /// monotone count, never reset (#1583 D5).
    #[must_use]
    pub fn output_dropped_windows(&self) -> u64 {
        self.output.dropped_windows()
    }

    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn test_only_set_loss_count(&mut self, losses: u64) {
        self.loss_count = losses;
    }

    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn test_only_pending_mut(&mut self) -> &mut [Option<MeterSnapshot>] {
        &mut self.tables.pending
    }

    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn test_only_output_mut(&mut self) -> &mut OutputPeakReader {
        &mut self.output
    }

    /// Turn the level on or off from `next_sample`, the next block's first sample (#1582 D1).
    ///
    /// Idempotent: setting the level it has returns `false` and changes nothing. A real transition
    /// advances the generation, clears the pending state and the publication (the frame's span,
    /// sequence, words, validity and loss count, keeping the new generation), pops the output
    /// windows queued under the previous level unread, and [`Self::align`]s at `next_sample`: on
    /// opens the output gate at the grid's first boundary at or after it, off shuts it. Returns
    /// `true`; the host then republishes [`Self::frame`]. Allocation-free.
    pub fn set_level(&mut self, on: bool, next_sample: u64) -> bool {
        if on == self.level {
            return false;
        }
        self.level = on;
        self.begin_generation(next_sample);
        true
    }

    /// Restart the delivery epoch at `next_sample` while the level stays on (#1588 D1, R3): one
    /// client of a shared [`MeterLevel`] changed while another kept the level on. The same as
    /// [`Self::set_level`] off and on again at `next_sample`, with one gate move: the generation
    /// advances, the pending state and the publication clear, the output windows queued under the
    /// old generation pop unread, and the output gate opens at the grid's first boundary at or
    /// after `next_sample`. A level that is off changes nothing and returns `false`.
    /// Allocation-free.
    pub fn restart(&mut self, next_sample: u64) -> bool {
        if !self.level {
            return false;
        }
        self.begin_generation(next_sample);
        true
    }

    /// The new generation of a level transition or a restart, from `next_sample`.
    fn begin_generation(&mut self, next_sample: u64) {
        self.activation = next_sample;
        self.generation = self.generation.saturating_add(1);
        self.reset(true);
        let discard = core::mem::take(&mut self.output_discard);
        self.pop_output_windows(discard);
        self.tables.frame.generation = self.generation;
        self.align(next_sample);
    }
}

/// The one meter level word of a session whose meter delivery serves two clients (#1588 D1): the
/// low-latency meter lease ([`Self::LEASE`]) and the protocol's meter table ([`Self::TELEMETRY`]).
/// Each client sets or clears its own bit from its own thread; only the delivery's owner reads
/// the word, once per poll, and the level is on while any bit is set. A clone shares the word.
#[derive(Clone, Debug, Default)]
pub struct MeterLevel(Arc<AtomicU32>);

impl MeterLevel {
    /// The bit of the low-latency meter lease.
    pub const LEASE: u32 = 1;
    /// The bit of a nonempty protocol meter table.
    pub const TELEMETRY: u32 = 1 << 1;

    /// A word with no client on. Allocates the shared word; call it on a control thread.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `client`'s bit (`on`) or clear it, leaving the other client's bit as it is.
    pub fn set(&self, client: u32, on: bool) {
        if on {
            self.0.fetch_or(client, Ordering::Release);
        } else {
            self.0.fetch_and(!client, Ordering::Release);
        }
    }

    /// The clients that are on.
    #[must_use]
    pub fn load(&self) -> u32 {
        self.0.load(Ordering::Acquire)
    }
}

/// The frames a [`MeterFrameRing`] holds (#1588 D2).
pub const METER_FRAME_RING_DEPTH: usize = 8;

/// A [`MeterFrameRing`] could not be built: a frame's tables or a queue could not be allocated.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MeterFrameRingSetupError;

/// The ring that carries a render-half meter delivery's published frames to a control half that
/// stages them (#1588 D2): [`METER_FRAME_RING_DEPTH`] preallocated frames that circulate between
/// two bounded SPSC queues, the free frames (control to render) and the published ones (render to
/// control), so a push copies a frame's elements into a free frame and nothing allocates or frees
/// after [`Self::new`]. A push that finds no free frame drops the frame and counts it.
pub struct MeterFrameRing {
    /// The render half, which the delivery's owner pushes each published frame into.
    pub producer: MeterFrameProducer,
    /// The control half, which stages the frames in order.
    pub consumer: MeterFrameConsumer,
}

/// The render half of a [`MeterFrameRing`].
pub struct MeterFrameProducer {
    /// The strips of every ring frame.
    strips: usize,
    free: Consumer<Box<MeterFrame>>,
    published: Producer<Box<MeterFrame>>,
    dropped: Arc<AtomicU64>,
}

/// The control half of a [`MeterFrameRing`].
pub struct MeterFrameConsumer {
    published: Consumer<Box<MeterFrame>>,
    free: Producer<Box<MeterFrame>>,
    dropped: Arc<AtomicU64>,
}

impl MeterFrameRing {
    /// A ring of frames of `strip_count` strips: the most strips a plan of the delivery can have,
    /// so no swap reallocates it. Allocates; call it on a control thread.
    ///
    /// # Errors
    ///
    /// [`MeterFrameRingSetupError`] when a frame or a queue cannot be allocated.
    pub fn new(strip_count: usize) -> Result<Self, MeterFrameRingSetupError> {
        let depth = NonZeroUsize::new(METER_FRAME_RING_DEPTH).unwrap_or(NonZeroUsize::MIN);
        let (mut free_producer, free_consumer) =
            bounded_spsc_move(depth, QueueGeneration(0)).map_err(|_| MeterFrameRingSetupError)?;
        let (published_producer, published_consumer) =
            bounded_spsc_move(depth, QueueGeneration(0)).map_err(|_| MeterFrameRingSetupError)?;
        for _ in 0..METER_FRAME_RING_DEPTH {
            let frame = MeterFrame::new(strip_count).map_err(|_| MeterFrameRingSetupError)?;
            free_producer
                .try_push(Box::new(frame))
                .map_err(|_| MeterFrameRingSetupError)?;
        }
        let dropped = Arc::new(AtomicU64::new(0));
        Ok(Self {
            producer: MeterFrameProducer {
                strips: strip_count,
                free: free_consumer,
                published: published_producer,
                dropped: Arc::clone(&dropped),
            },
            consumer: MeterFrameConsumer {
                published: published_consumer,
                free: free_producer,
                dropped,
            },
        })
    }

    /// The heap bytes [`Self::new`] allocates for `strip_count` strips, so an owner charges them:
    /// the frames and their tables, the two queues' headers and slots, and the shared drop count.
    /// `None` on overflow.
    #[must_use]
    pub fn heap_bytes(strip_count: usize) -> Option<usize> {
        let depth = NonZeroUsize::new(METER_FRAME_RING_DEPTH)?;
        let queue = bounded_spsc_retained_payload::<Box<MeterFrame>>(depth).ok()?;
        let queue_bytes = queue
            .ring_header_bytes
            .checked_add(queue.slot_payload_bytes)?;
        let strip_bytes = size_of::<[f32; 2]>() + size_of::<f32>() + size_of::<bool>();
        let frame_bytes = strip_count
            .checked_mul(strip_bytes)?
            .checked_add(size_of::<MeterFrame>())?;
        frame_bytes
            .checked_mul(METER_FRAME_RING_DEPTH)?
            .checked_add(queue_bytes.checked_mul(2)?)?
            .checked_add(3 * size_of::<u64>())
    }

    /// The largest single allocation of [`Self::heap_bytes`]: a frame's strip peak table, or a
    /// queue's slots or header. `None` on overflow.
    #[must_use]
    pub fn largest_allocation_bytes(strip_count: usize) -> Option<usize> {
        let depth = NonZeroUsize::new(METER_FRAME_RING_DEPTH)?;
        let queue = bounded_spsc_retained_payload::<Box<MeterFrame>>(depth).ok()?;
        Some(
            strip_count
                .checked_mul(size_of::<[f32; 2]>())?
                .max(size_of::<MeterFrame>())
                .max(queue.slot_payload_bytes)
                .max(queue.ring_header_bytes),
        )
    }
}

// REALTIME_POLICY_BEGIN: #1588 D2, the meter frame ring's two halves.
impl MeterFrameConsumer {
    /// The published frames the control half has not released, as one acquire load sees them:
    /// a later push only adds to it.
    #[must_use]
    pub fn available(&self) -> usize {
        self.published.available_at_entry()
    }

    /// The oldest published frame the control half has not released, if any.
    #[must_use]
    pub fn peek(&self) -> Option<&MeterFrame> {
        self.published.peek(0).map(|frame| &**frame)
    }

    /// Release the oldest published frame back to the render half. Does nothing on an empty
    /// ring. Allocation-free.
    pub fn release_frame(&mut self) {
        if let Ok(frame) = self.published.try_pop() {
            // The free queue holds every frame the published queue does not, so it has room.
            let _ = self.free.try_push(frame);
        }
    }

    /// The meter windows of the frames a push dropped (each frame's [`MeterFrame::windows`], at
    /// least one): monotone, never reset.
    #[must_use]
    pub fn dropped_windows(&self) -> u64 {
        self.dropped.load(Ordering::Acquire)
    }
}

impl MeterFrameProducer {
    /// Copy `frame` into a free frame and publish it (#1588 D2): every field and every strip
    /// table element, by element. A ring frame has the ring's strip count; strips past `frame`'s
    /// read as absent (zero peaks, no gain reduction). A frame with more strips than the ring's,
    /// or a ring with no free frame, drops `frame` and counts its windows ([`MeterFrameConsumer::
    /// dropped_windows`]); returns whether `frame` was published. Allocation-free, bounded by the
    /// ring's strip count.
    pub fn publish_frame(&mut self, frame: &MeterFrame) -> bool {
        let fits = frame.strip_peaks.len() <= self.strips;
        let slot = if fits { self.free.try_pop().ok() } else { None };
        let Some(mut slot) = slot else {
            self.dropped
                .fetch_add(u64::from(frame.windows.max(1)), Ordering::Release);
            return false;
        };
        copy_frame(&mut slot, frame);
        // The published queue holds every frame the free queue does not, so it has room. The
        // unreachable refusal is forgotten, not dropped: no free joins the render thread's code.
        if let Err(full) = self.published.try_push(slot) {
            core::mem::forget(full);
        }
        true
    }
}

fn copy_frame(into: &mut MeterFrame, from: &MeterFrame) {
    into.first_sample = from.first_sample;
    into.end_sample = from.end_sample;
    into.windows = from.windows;
    into.sequence = from.sequence;
    into.generation = from.generation;
    into.plan_epoch = from.plan_epoch;
    into.validity = from.validity;
    into.loss_count = from.loss_count;
    into.output_peak = from.output_peak;
    let mut peaks = from.strip_peaks.iter();
    for slot in &mut into.strip_peaks {
        *slot = peaks.next().copied().unwrap_or([0.0; 2]);
    }
    let mut reductions = from.strip_gain_reduction_db.iter();
    for slot in &mut into.strip_gain_reduction_db {
        *slot = reductions.next().copied().unwrap_or(0.0);
    }
    let mut present = from.strip_gain_reduction_present.iter();
    for slot in &mut into.strip_gain_reduction_present {
        *slot = present.next().copied().unwrap_or(false);
    }
}
// REALTIME_POLICY_END

/// The rendering plan's strip meter consumers, as [`MeterDelivery::poll`] borrows them. A field
/// place, because the realtime policy's per-queue drain form names its queue collection as one.
struct StripMeters<'a> {
    consumers: &'a mut [MeterConsumer],
}

// REALTIME_POLICY_BEGIN: #1582 D1, the meter delivery's render-thread half.
impl MeterDelivery {
    /// The one activation rule (#1581 D5), for every event that moves the grid or the level (a
    /// level change, a producer reset, a swap that carries no meter): the activation is the
    /// grid's first boundary at or after `from`, and the output meter's gate opens there while the
    /// level is on and is shut otherwise. The strip windows the poll accepts and the output
    /// windows the gate admits are then one grid's.
    pub fn align(&mut self, from: u64) {
        self.activation = self.grid_boundary(from);
        if self.level {
            self.gate_from = self.activation;
            self.gate.activate(self.activation);
        } else {
            self.gate_from = INACTIVE;
            self.gate.deactivate();
        }
    }

    /// The closed output windows the next poll can read: the queue less the windows a reset
    /// discarded.
    pub fn output_windows_queued(&mut self) -> usize {
        self.output
            .windows_mut()
            .available_at_entry()
            .saturating_sub(self.output_discard)
    }

    /// Adopt a successor plan's `successor` tables at its adoption block, whose first sample is
    /// `adoption_sample` (#1582 R1, #1290d1, #1581 D5). A pointer exchange: on return `successor`
    /// holds the retired tables, for the host to drop off the render thread.
    ///
    /// The publication continues in the successor's frame (its span, sequence, generation,
    /// validity and loss are the delivery's, its strip tables the plan's). Each of `carries`
    /// moves its predecessor meter's pending snapshot to its successor position and records the
    /// predecessor's handle there when it differs, so a window the predecessor closed before the
    /// swap is accepted by the strip it was carried to. `carried` is whether the swap carried any
    /// strip meter: a swap that carried one keeps the delivery (the carried meters keep their
    /// grid); one that carried none restarts it, because its meters open their first window at
    /// the adoption block: the windows it discards unpublished (the output windows queued and a
    /// group the poll had begun to assemble) are counted as losses, the generation advances, the
    /// grid restarts at `adoption_sample`, and a level that is on [`Self::align`]s there.
    /// Allocation-free, bounded by `carries`.
    pub fn adopt(
        &mut self,
        successor: &mut MeterTables,
        carries: impl Iterator<Item = MeterCarry>,
        carried: bool,
        adoption_sample: u64,
    ) {
        let discarded = u64::try_from(self.output_windows_queued())
            .unwrap_or(u64::MAX)
            .saturating_add(u64::from(self.tables.pending.iter().any(Option::is_some)));
        let (from, into) = (&mut self.tables.frame, &mut successor.frame);
        into.first_sample = from.first_sample;
        into.end_sample = from.end_sample;
        into.windows = from.windows;
        into.sequence = from.sequence;
        into.generation = from.generation;
        into.validity = from.validity;
        into.loss_count = from.loss_count;
        into.output_peak = from.output_peak;
        for carry in carries {
            let (Ok(next), Ok(previous)) = (
                usize::try_from(carry.successor),
                usize::try_from(carry.predecessor),
            ) else {
                continue;
            };
            if let (Some(into), Some(from)) = (
                successor.pending.get_mut(next),
                self.tables.pending.get_mut(previous),
            ) {
                core::mem::swap(into, from);
            }
            if let Some(from) = successor.carried_from.get_mut(next) {
                *from = if carry.predecessor_handle == carry.successor_handle {
                    0
                } else {
                    carry.predecessor_handle
                };
            }
        }
        core::mem::swap(&mut self.tables, successor);
        if !carried {
            self.fold_output_drops();
            let lost = self.loss_count.saturating_add(discarded);
            self.reset(false);
            self.loss_count = lost;
            self.generation = self.generation.saturating_add(1);
            self.grid_origin = adoption_sample;
            // The restarted strip meters open their first window at this block, a new grid
            // (#1581 D5). A level that is on, active or still waiting for its activation, moves
            // to that grid's first boundary at or after both, and the gate with it, before the
            // block is observed.
            if self.level {
                self.align(adoption_sample);
            }
        }
    }

    /// Fold the closed windows into the frame (#1582 D1): at most one published window per call.
    ///
    /// `meters` are the rendering plan's strip meter consumers, in strip order; `taps` its
    /// observation taps; `next_sample` the next block's first sample. Returns the frame when this
    /// call published one, else `None` and leaves the frame as it was.
    ///
    /// Groups the strip meters' heads: equal groups (handle or carried handle, reset generation,
    /// sequence, span and frames) are one candidate window; a sequence gap drops only the older
    /// heads, other mismatches the whole group, each counted. A window before the activation, or
    /// empty, is a loss. A producer reset (a new reset generation) starts a new generation,
    /// realigns the output gate and publishes nothing. The candidate pairs with the output window
    /// of exactly its span, and older output windows are losses; a plan with no strip publishes
    /// the output's own windows (#1682). The commit writes every strip's peaks, the output peak,
    /// the validity bits, the gain reduction ([`fold_gain_reduction`]), the span, sequence,
    /// generation and loss count.
    ///
    /// Allocation-free, no lock and no runtime-checked index of its own; bounded by
    /// `1 + sum of the entry counts` (#1448 D2) and the output queue's depth. The one checked
    /// index is the SPSC ring's slot index, inlined here, which the ring keeps in range. Never
    /// inlined into its caller, so a host's render-thread call-graph gate names this function as
    /// that index's one admitted owner (#1582 R3), and a checked index the host adds is its own.
    #[inline(never)]
    pub fn poll(
        &mut self,
        meters: &mut [MeterConsumer],
        taps: &mut impl GainReductionTaps,
        next_sample: u64,
    ) -> Option<&MeterFrame> {
        if !self.level {
            return None;
        }
        // The fold reads the output windows in place and counts the ones it is done with; they are
        // popped here, at one site, so the queue's checked slot index inlines once.
        let mut output_pops = 0_usize;
        let strips = StripMeters { consumers: meters };
        let published = self.fold_meter_windows(strips, taps, next_sample, &mut output_pops);
        self.pop_output_windows(output_pops);
        published.then_some(&self.tables.frame)
    }

    /// [`Self::poll`]'s fold. It pops no output window: it reads them in place, from the head on,
    /// and leaves in `output_pops` how many from the head the caller pops.
    #[inline(always)]
    #[allow(clippy::too_many_lines)]
    fn fold_meter_windows(
        &mut self,
        strips: StripMeters<'_>,
        taps: &mut impl GainReductionTaps,
        next_sample: u64,
        output_pops: &mut usize,
    ) -> bool {
        let meter_count = strips.consumers.len();
        // The output meter's windows a reset discarded go first, unread, and the windows its full
        // queue dropped are losses.
        *output_pops = self.output_discard;
        self.output_discard = 0;
        self.fold_output_drops();
        // The render path's output meter is the sole producer of this fixed-capacity queue and
        // pushes only when the configured master interval closes. An empty queue therefore proves
        // that no coherent strip/output publication can exist. Preserve pending and published
        // state and avoid walking every strip consumer on incomplete quanta.
        if self.output.windows_mut().available_at_entry() <= *output_pops {
            return false;
        }
        let tables = &mut self.tables;
        let mut windows = 0_u32;
        let mut folded_start = 0_u64;
        let mut folded_end = 0_u64;
        let mut folded = false;
        // A plan with no strip (#1682) has no strip window to pair the output with, so the
        // output's own windows are the stream. The passes below find no strip and fold nothing;
        // the output's front window is read where every output window is read, after them.
        let master_only = meter_count == 0;
        // Set by a producer reset, which ends the passes and publishes nothing.
        let mut producer_reset = false;
        // Issue #1448 D2: each meter's queue pops at most its own count read here, at entry,
        // capped at the configured depth, whatever a producer on another thread publishes
        // meanwhile. A pass that does not end the loop has cleared at least one pending slot, and
        // the next pass either refills it (spending one count) or ends the loop at the empty-slot
        // check, so `1 + sum(remaining)` passes always suffice: the bound never cuts a recovery
        // short.
        for (meter, remaining) in strips.consumers.iter().zip(&mut tables.remaining) {
            *remaining = meter.consumer.available_at_entry().min(METER_QUEUE_DEPTH);
        }
        let mut passes = 1_usize;
        for remaining in tables.remaining.iter() {
            passes = passes.saturating_add(*remaining);
        }
        for _ in 0..passes {
            for ((meter, pending), remaining) in strips
                .consumers
                .iter_mut()
                .zip(&mut tables.pending)
                .zip(&mut tables.remaining)
            {
                if pending.is_some() || *remaining == 0 {
                    continue;
                }
                // On a single-consumer queue a count read at entry is a lower bound on what this
                // consumer can pop, so a pop under a count above zero does not fail.
                *remaining -= 1;
                if let Ok(snapshot) = meter.consumer.try_pop() {
                    *pending = Some(snapshot);
                }
            }
            if tables.pending.iter().any(Option::is_none) {
                break;
            }
            let Some(first) = tables.pending.first().and_then(Option::as_ref).copied() else {
                break;
            };
            let carried_from = &tables.carried_from;
            let all_equal = tables.pending.iter().enumerate().all(|(index, entry)| {
                let Some(snapshot) = entry else { return false };
                // A window the predecessor closed before a swap names the predecessor's handle for
                // the strip the meter was carried from (#1290d1).
                strips.consumers.get(index).is_some_and(|meter| {
                    snapshot.handle == meter.handle
                        || carried_from
                            .get(index)
                            .is_some_and(|from| *from != 0 && snapshot.handle.0.get() == *from)
                }) && snapshot.reset_generation == first.reset_generation
                    && snapshot.window_sequence == first.window_sequence
                    && snapshot.start_sample == first.start_sample
                    && snapshot.end_sample == first.end_sample
                    && snapshot.frames == first.frames
            });
            if !all_equal {
                // A sequence gap can be recovered by dropping only the older stream head. Equal
                // sequences with different spans/generations are a discontinuity: drop the whole
                // candidate set so no mixed interval reaches the published frame.
                let same_generation = tables.pending.iter().all(|entry| {
                    entry
                        .is_some_and(|snapshot| snapshot.reset_generation == first.reset_generation)
                });
                let same_sequence = tables.pending.iter().all(|entry| {
                    entry.is_some_and(|snapshot| snapshot.window_sequence == first.window_sequence)
                });
                if same_generation && !same_sequence {
                    let minimum = tables
                        .pending
                        .iter()
                        .filter_map(|entry| entry.as_ref().map(|snapshot| snapshot.window_sequence))
                        .min()
                        .unwrap_or(first.window_sequence);
                    for pending in &mut tables.pending {
                        if pending
                            .as_ref()
                            .is_some_and(|snapshot| snapshot.window_sequence == minimum)
                        {
                            *pending = None;
                            self.loss_count = self.loss_count.saturating_add(1);
                        }
                    }
                } else {
                    for pending in &mut tables.pending {
                        *pending = None;
                        self.loss_count = self.loss_count.saturating_add(1);
                    }
                }
                continue;
            }
            if first.start_sample < self.activation
                || first.end_sample <= first.start_sample
                || u64::from(first.frames) != first.end_sample - first.start_sample
            {
                for pending in &mut tables.pending {
                    *pending = None;
                }
                self.loss_count = self.loss_count.saturating_add(1);
                continue;
            }
            if self
                .snapshot_generation
                .is_some_and(|generation| generation != first.reset_generation)
            {
                // A producer reset is a new delivery epoch. Reject this group under the old
                // published identity; the state is cleared below, outside the passes, so the next
                // render opens a clean output/strip interval. The published frame stays as it was
                // because a producer reset never publishes.
                producer_reset = true;
                break;
            }
            if tables.frame.sequence > 0
                && tables.frame.generation == self.generation
                && first.start_sample != tables.frame.end_sample
            {
                self.loss_count = self.loss_count.saturating_add(1);
            }
            folded_start = first.start_sample;
            folded = true;
            folded_end = first.end_sample;
            windows = windows.saturating_add(1);
            // Bounded per-window delivery: further complete windows remain queued for the next
            // poll, preserving one exact span per published frame.
            break;
        }
        if producer_reset {
            self.generation = self.generation.saturating_add(1);
            self.reset(false);
            self.loss_count = 1;
            // #1581 D5: every queued output window is the old epoch's and is popped by this call,
            // so the next render finds the room the cleared ring it replaces had, and the output
            // meter restarts at the strips' boundary (a moved gate discards the window it has
            // open).
            *output_pops = self.output_discard;
            self.output_discard = 0;
            self.align(next_sample);
            return false;
        }
        if !folded && !master_only {
            return false;
        }
        let stale_master_budget = self
            .output
            .windows_mut()
            .available_at_entry()
            .saturating_sub(*output_pops);
        // The first window that ends after the strips' start is the candidate; each earlier one is
        // stale, counted and popped. Without a strip the front window is the candidate, or, when
        // it predates the activation or is empty, a loss. One read site, so the queue's peek
        // inlines here.
        let mut master_measurement = None;
        for _ in 0..stale_master_budget {
            let Some(measurement) = self.output_window_at(*output_pops) else {
                break;
            };
            if master_only {
                if measurement.start_sample < self.activation
                    || measurement.end_sample <= measurement.start_sample
                {
                    *output_pops += 1;
                    self.loss_count = self.loss_count.saturating_add(1);
                    return false;
                }
                let frame = &self.tables.frame;
                if frame.sequence > 0
                    && frame.generation == self.generation
                    && measurement.start_sample != frame.end_sample
                {
                    self.loss_count = self.loss_count.saturating_add(1);
                }
                folded_start = measurement.start_sample;
                folded_end = measurement.end_sample;
                folded = true;
                windows = 1;
                master_measurement = Some(measurement);
                break;
            }
            if measurement.end_sample > folded_start {
                master_measurement = Some(measurement);
                break;
            }
            *output_pops += 1;
            self.loss_count = self.loss_count.saturating_add(1);
        }
        if !folded {
            return false;
        }
        let master_valid = master_measurement.is_some_and(|measurement| {
            measurement.start_sample == folded_start && measurement.end_sample == folded_end
        });
        let tables = &mut self.tables;
        if !master_valid {
            // The delivery cannot recover an interval peak from one envelope once its boundary is
            // lost. Consume the strip group, count the discontinuity, and wait for the next clean
            // output/strip span rather than publishing a mixed or guessed output value.
            for pending in &mut tables.pending {
                if let Some(snapshot) = pending.take()
                    && snapshot.cumulative_dropped_snapshots > self.snapshot_drops_seen
                {
                    self.loss_count = self.loss_count.saturating_add(
                        snapshot
                            .cumulative_dropped_snapshots
                            .saturating_sub(self.snapshot_drops_seen),
                    );
                    self.snapshot_drops_seen = snapshot.cumulative_dropped_snapshots;
                }
            }
            self.loss_count = self.loss_count.saturating_add(1);
            return false;
        }
        *output_pops += 1;
        let master_peak = master_measurement.map_or([0.0, 0.0], |measurement| measurement.peak);
        // All candidate validation is complete. Only this commit section mutates the frame; every
        // earlier return leaves it as it was.
        let frame = &mut tables.frame;
        for (index, pending) in tables.pending.iter_mut().enumerate() {
            let Some(snapshot) = pending.take() else {
                continue;
            };
            if snapshot.cumulative_dropped_snapshots > self.snapshot_drops_seen {
                self.loss_count = self.loss_count.saturating_add(
                    snapshot
                        .cumulative_dropped_snapshots
                        .saturating_sub(self.snapshot_drops_seen),
                );
                self.snapshot_drops_seen = snapshot.cumulative_dropped_snapshots;
            }
            if let Some(peak) = frame.strip_peaks.get_mut(index) {
                *peak = [snapshot.left.sample_peak, snapshot.right.sample_peak];
            }
            self.snapshot_generation = Some(snapshot.reset_generation);
        }
        frame.output_peak = master_peak;
        let mut validity = MeterValidity::COMPLETE.union(MeterValidity::OUTPUT);
        if self.loss_count != 0 {
            validity = validity.union(MeterValidity::LOSS);
        }
        frame.validity = validity;
        // Issue #143 D5: the gain-reduction section. The poll runs after the render, with no
        // allocation, lock or unbounded loop, so the one unit conversion the design permits, a
        // linear tap's magnitude to decibels, runs here once per closed window.
        fold_gain_reduction(frame, taps);
        frame.windows = windows;
        frame.sequence = frame.sequence.saturating_add(1);
        frame.first_sample = folded_start;
        frame.end_sample = folded_end;
        frame.generation = self.generation;
        frame.loss_count = self.loss_count;
        self.folded_windows = self.folded_windows.saturating_add(u64::from(windows));
        true
    }

    /// Clear the delivery's pending state. Producer-side accumulators continue rendering; the
    /// activation makes the already bounded poll reject queued pre-activation windows before it
    /// accepts a clean interval. `clear_publication` also clears the published frame (all but its
    /// generation) and the folded-window count.
    fn reset(&mut self, clear_publication: bool) {
        for pending in &mut self.tables.pending {
            *pending = None;
        }
        // The output windows queued now are the previous delivery's: they are popped unread (by
        // the caller at once off the render thread, by the next poll after a swap), and the drops
        // counted so far are forgotten with the loss count. The window the meter has open is its
        // own; each caller moves the gate, which discards it.
        self.output_discard = self.output.windows_mut().available_at_entry();
        self.output_dropped_seen = self.output.dropped_windows();
        self.snapshot_generation = None;
        self.snapshot_drops_seen = 0;
        self.loss_count = 0;
        if clear_publication {
            let frame = &mut self.tables.frame;
            for peak in &mut frame.strip_peaks {
                *peak = [0.0; 2];
            }
            for slot in &mut frame.strip_gain_reduction_db {
                *slot = 0.0;
            }
            for present in &mut frame.strip_gain_reduction_present {
                *present = false;
            }
            frame.output_peak = [0.0; 2];
            frame.validity = MeterValidity::NONE;
            frame.loss_count = 0;
            frame.sequence = 0;
            frame.windows = 0;
            frame.first_sample = 0;
            frame.end_sample = 0;
            self.folded_windows = 0;
        }
    }

    /// The first boundary at or after `sample` of the strip meters' window grid (#1581 D5):
    /// `grid_origin + k * window_frames`, or `sample` itself with no window.
    fn grid_boundary(&self, sample: u64) -> u64 {
        let (origin, period) = (self.grid_origin, self.window_frames);
        if period == 0 {
            return sample;
        }
        if sample <= origin {
            return origin;
        }
        let remainder = (sample - origin) % period;
        sample.saturating_add(if remainder == 0 {
            0
        } else {
            period - remainder
        })
    }

    /// Add the output windows a full queue dropped since the last fold to the loss count: a
    /// dropped window is a loss, as the engine meter queue's drop-newest contract reports it.
    fn fold_output_drops(&mut self) {
        let dropped = self.output.dropped_windows();
        self.loss_count = self
            .loss_count
            .saturating_add(dropped.saturating_sub(self.output_dropped_seen));
        self.output_dropped_seen = dropped;
    }

    /// Pop `count` output windows from the head, unread: at most the count read at entry, which no
    /// pop of a single-consumer queue can exceed. Inlined, so the poll's call graph keeps the
    /// queue's checked slot index under its own admitted owner.
    #[inline(always)]
    fn pop_output_windows(&mut self, count: usize) {
        let windows = self.output.windows_mut();
        let count = windows.available_at_entry().min(count);
        for _ in 0..count {
            let _ = windows.try_pop();
        }
    }

    /// The output window `index` places from the head, not popped. The queue's one read site, so
    /// its checked slot index inlines into the poll once.
    #[inline(always)]
    fn output_window_at(&mut self, index: usize) -> Option<OutputPeakWindow> {
        self.output.windows_mut().peek(index).copied()
    }
}
// REALTIME_POLICY_END

/// One completed meter batch (#1583 D5): `windows` consecutive published frames folded into one
/// span, with its sequence in the fold.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeterBatchWindow {
    /// The first frame's first sample.
    pub first_sample: u64,
    /// The last frame's end sample.
    pub end_sample: u64,
    /// The batch's sequence since the fold last restarted, from `1`.
    pub sequence: u64,
    /// The output's left and right peaks: the frames' maxima.
    pub output_peak: [f32; 2],
    /// Whether every folded frame's output peak covered exactly its span
    /// ([`MeterValidity::OUTPUT`]).
    pub output_valid: bool,
}

/// One meter table entry's strip readings over a batch's frames, its peak (#1587a) and its gain
/// reduction (#1587b), as [`MeterBatchFold::push_with_strips`] folds them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StripPeakFold {
    /// The strip's left and right peaks: the frames' maxima.
    pub peak: [f32; 2],
    /// Whether every frame of the batch was complete and had the strip.
    pub present: bool,
    /// The strip's gain reduction in decibels: the last frame's reading that is present, `0.0`
    /// when none is.
    pub gain_reduction_db: f32,
    /// Whether any frame of the batch had a gain-reduction reading for the strip.
    pub gain_reduction_present: bool,
}

impl StripPeakFold {
    /// A fold before its batch's first frame.
    pub const START: Self = Self {
        peak: [0.0; 2],
        present: true,
        gain_reduction_db: 0.0,
        gain_reduction_present: false,
    };
}

/// The fold of a meter delivery's published frames into meter batches (#1583 D5; shared with
/// #1587 and #1588): `windows_per_batch` consecutive frames make one batch. The peaks are the
/// maxima, the span is the first frame's start to the last frame's end, and a gap between two
/// frames (a frame whose start is not the previous one's end, or of another delivery generation)
/// discards the partial batch and counts its windows in [`Self::discarded_windows`]. Allocates
/// nothing.
#[derive(Clone, Debug)]
pub struct MeterBatchFold {
    windows_per_batch: u32,
    folded: u32,
    first_sample: u64,
    end_sample: u64,
    generation: u64,
    output_peak: [f32; 2],
    output_valid: bool,
    sequence: u64,
    discarded: u64,
}

impl MeterBatchFold {
    /// A fold of `windows_per_batch` frames per batch, with no frame folded and sequence 0.
    #[must_use]
    pub const fn new(windows_per_batch: NonZeroU32) -> Self {
        Self {
            windows_per_batch: windows_per_batch.get(),
            folded: 0,
            first_sample: 0,
            end_sample: 0,
            generation: 0,
            output_peak: [0.0; 2],
            output_valid: true,
            sequence: 0,
            discarded: 0,
        }
    }

    /// Start again with `windows_per_batch` frames per batch (an accepted configuration): the
    /// partial batch is discarded and counted, and the next batch is sequence `1`.
    pub fn restart(&mut self, windows_per_batch: NonZeroU32) {
        self.discard();
        self.windows_per_batch = windows_per_batch.get();
        self.sequence = 0;
    }

    /// The frames one batch folds.
    #[must_use]
    pub const fn windows_per_batch(&self) -> u32 {
        self.windows_per_batch
    }

    /// The windows of partial batches discarded so far: monotone.
    #[must_use]
    pub const fn discarded_windows(&self) -> u64 {
        self.discarded
    }

    /// Fold one published `frame`. Returns the batch it completes, if any.
    pub fn push(&mut self, frame: &MeterFrame) -> Option<MeterBatchWindow> {
        self.push_with_strips(frame, &[], &mut [])
    }

    /// Fold one published `frame`, and into `folds` the strip readings the meter table's entries
    /// read (#1587a, #1587b). `strips[i]` is entry `i`'s strip in `frame` (its index in
    /// [`MeterFrame::strip_peaks`]), or `None` when the plan that published `frame` has no such
    /// strip or the entry reads no strip; `folds[i]` is that entry's fold, which the caller
    /// reads when the call returns a batch. A batch's first frame resets every fold. An entry's
    /// peaks are the maxima of its strip's peaks over the batch's frames, and they are present only
    /// when every frame of the batch is complete ([`MeterValidity::COMPLETE`]) and has its strip.
    /// Its gain reduction is the last frame's reading for its strip that is present, and it is
    /// present when any frame's is. Allocates nothing.
    pub fn push_with_strips(
        &mut self,
        frame: &MeterFrame,
        strips: &[Option<u32>],
        folds: &mut [StripPeakFold],
    ) -> Option<MeterBatchWindow> {
        if self.folded != 0
            && (frame.generation != self.generation || frame.first_sample != self.end_sample)
        {
            self.discard();
        }
        if self.folded == 0 {
            self.first_sample = frame.first_sample;
            self.generation = frame.generation;
            self.output_peak = [0.0; 2];
            self.output_valid = true;
            folds.fill(StripPeakFold::START);
        }
        let complete = frame.validity.contains(MeterValidity::COMPLETE);
        for (strip, fold) in strips.iter().zip(folds.iter_mut()) {
            let peak = strip
                .and_then(|strip| usize::try_from(strip).ok())
                .and_then(|strip| frame.strip_peaks.get(strip))
                .filter(|_| complete);
            if let Some(peak) = peak {
                fold.peak = [fold.peak[0].max(peak[0]), fold.peak[1].max(peak[1])];
            } else {
                fold.present = false;
            }
            // #1587b D3: the gain reduction's timing is its taps' own, so a frame's reading
            // counts whether or not the frame is complete.
            let index = strip.and_then(|strip| usize::try_from(strip).ok());
            if let Some(index) = index
                && frame.strip_gain_reduction_present.get(index) == Some(&true)
                && let Some(&value) = frame.strip_gain_reduction_db.get(index)
            {
                fold.gain_reduction_db = value;
                fold.gain_reduction_present = true;
            }
        }
        self.output_peak = [
            self.output_peak[0].max(frame.output_peak[0]),
            self.output_peak[1].max(frame.output_peak[1]),
        ];
        self.output_valid &= frame.validity.contains(MeterValidity::OUTPUT);
        self.end_sample = frame.end_sample;
        self.folded = self.folded.saturating_add(frame.windows.max(1));
        if self.folded < self.windows_per_batch {
            return None;
        }
        self.folded = 0;
        self.sequence = self.sequence.saturating_add(1);
        Some(MeterBatchWindow {
            first_sample: self.first_sample,
            end_sample: self.end_sample,
            sequence: self.sequence,
            output_peak: self.output_peak,
            output_valid: self.output_valid,
        })
    }

    /// Drop the partial batch without counting it: the meters were turned off, so its windows are
    /// owed to no one. The next [`Self::restart`] then discards nothing.
    pub fn drop_partial(&mut self) {
        self.folded = 0;
    }

    fn discard(&mut self) {
        self.discarded = self.discarded.saturating_add(u64::from(self.folded));
        self.folded = 0;
    }
}

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
    use super::{
        MeterBatchFold, MeterFrame, MeterValidity, StripPeakFold, default_meter_window_blocks,
        output_peak_gate_relay, output_peak_meter,
    };

    /// #1587b D3: in a batch of three frames, an entry's gain reduction is the last frame's
    /// reading for its strip that is present, `VALID` when any is. Strip 0 reads 3 dB, then 2 dB,
    /// then nothing: the batch reports 2 dB. Strip 1 reads nothing: absent. In the next batch strip
    /// 0 reads nothing and strip 1 only in its first frame: strip 0 is absent again, strip 1 reads
    /// that frame's 1 dB. An entry with no strip reads nothing.
    ///
    /// Test value: red if the fold takes the batch's maximum (3 dB), its first or last frame's
    /// reading (3 dB, or absent), or keeps a previous batch's reading past the batch's first frame.
    #[test]
    fn a_batch_reports_each_strips_last_present_gain_reduction() {
        let mut fold = MeterBatchFold::new(core::num::NonZeroU32::new(3).expect("three"));
        let strips = [Some(0), Some(1), None];
        let mut folds = [StripPeakFold::START; 3];
        let mut frame = MeterFrame::new(2).expect("frame");
        let readings: [[Option<f32>; 2]; 6] = [
            [Some(3.0), None],
            [Some(2.0), None],
            [None, None],
            [None, Some(1.0)],
            [None, None],
            [None, None],
        ];
        let mut batches = Vec::new();
        for (index, reading) in (0_u64..).zip(readings) {
            frame.first_sample = index * 256;
            frame.end_sample = (index + 1) * 256;
            frame.windows = 1;
            frame.validity = MeterValidity::COMPLETE.union(MeterValidity::OUTPUT);
            for (strip, value) in reading.into_iter().enumerate() {
                frame.strip_gain_reduction_db[strip] = value.unwrap_or(0.0);
                frame.strip_gain_reduction_present[strip] = value.is_some();
            }
            if fold.push_with_strips(&frame, &strips, &mut folds).is_some() {
                batches
                    .push(folds.map(|fold| (fold.gain_reduction_present, fold.gain_reduction_db)));
            }
        }
        assert_eq!(
            batches,
            [
                [(true, 2.0), (false, 0.0), (false, 0.0)],
                [(false, 0.0), (true, 1.0), (false, 0.0)],
            ]
        );
    }

    /// #1587a, root ruling 3: a request whose boundary is behind the block render applies it in
    /// (the control thread read the render clock blocks earlier) opens the gate at the grid's
    /// next boundary, not at that block. On a grid of 256-frame windows from 0, a request for
    /// sample 0 applied before the block at 384 opens the output window at 512, so it closes as
    /// `[512, 768)`, the strip meters' window of the same span, and the two pair.
    ///
    /// Test value: red if the relay copies a request behind the block into the gate, so the output
    /// meter opens `[384, 640)`, off the strip meters' grid, and no strip window pairs with it again.
    #[test]
    fn a_gate_request_behind_its_block_opens_on_the_next_grid_boundary() {
        const QUANTUM: u64 = 128;
        let window = core::num::NonZeroU32::new(256).expect("window");
        let (mut observer, mut reader, gate) = output_peak_meter(window).expect("output meter");
        let (requested, mut relay) = output_peak_gate_relay(gate, u64::from(window.get()));
        requested.activate(0);
        let plane = [0.5_f32; QUANTUM as usize];
        for block in 3..6 {
            let first = block * QUANTUM;
            relay.apply(first);
            assert_eq!(observer.admits(first), first >= 512, "block {block}");
            observer.observe(&plane, &plane, first, first + QUANTUM);
        }
        let windows = reader.windows_mut();
        let closed = windows.try_pop().expect("one closed window");
        assert_eq!((closed.start_sample, closed.end_sample), (512, 768));
        assert!(windows.try_pop().is_err(), "and no other");
    }

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
