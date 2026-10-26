//! The session's output loudness meter (#2115): BS.1770-5 loudness, EBU Tech 3342 loudness range
//! and Annex-2 true peak of the rendered master pair, read through `LOUDNESS` (`0x0014`).
//!
//! The output peak meter's twin: made once per session at boot and part of no plan, so no plan
//! rebuild ever touches it and a plan swap carries it by construction (the render half lives in
//! the session's one [`crate::PlanState`], which continues the clock across a swap).
//!
//! * [`LoudnessTap`], the **render half**, runs after every rendered block. While the meter is
//!   disarmed it pays one `Acquire` load of the gate. Armed, it runs
//!   [`loudness::LoudnessObserver`] and pushes each completed 10 ms hop into a bounded SPSC queue
//!   of [`LOUDNESS_QUEUE_DEPTH`] records; a full queue drops the hop and counts it. It never
//!   blocks, allocates or logs.
//! * [`LoudnessMonitor`], the **control half**, owns the queue's consumer, the drop count, the
//!   gate writer and a [`loudness::LoudnessAnalysis`]. The session's service step drains it, and
//!   so does every `LOUDNESS` operation before it answers.
//!
//! **Arming.** The gate is one word: inactive, or the render sample a start armed the meter from.
//! The first block at or after it restarts the observer there (the hop grid's origin). Each hop
//! carries the gate word it was observed under, and the control half ingests only hops of its
//! current arming, so a block in flight across a stop and a start can never feed the new
//! arming's analysis.
//!
//! **Acked-batch question: can an ack ever precede a drop? No.** A start acknowledges the arming
//! only, never a delivery. A hop is dropped on render before any reply can mention it; a reply
//! reports only hops already drained into the analysis, and carries the exact, monotone
//! `hops_dropped`. A reset or a stop changes only control state.

use std::num::NonZeroUsize;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use engine::realtime::{
    Consumer, Producer, QueueGeneration, SpscError, SpscRetainedPayload, bounded_spsc,
    bounded_spsc_retained_payload,
};
use loudness::{HopRecord, LoudnessAnalysis, LoudnessObserver};

/// The hop record queue's depth: 5.12 s of 10 ms hops at 100 records per second, which outlasts
/// any service cadence either host runs (one tick per quantum in the browser, throttled to about
/// 1 s in a background tab; the C ABI's host-driven `miso_engine_v1_service`, which every session
/// control call also runs).
pub(crate) const LOUDNESS_QUEUE_DEPTH: usize = 512;

/// The gate word of a disarmed meter.
const INACTIVE: u64 = u64::MAX;

/// One hop as the queue carries it: the record and the gate word it was observed under.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ArmedHop {
    arm: u64,
    record: HopRecord,
}

/// The render half of the session's loudness meter (see the module notes).
pub(crate) struct LoudnessTap {
    gate: Arc<AtomicU64>,
    hops: Producer<ArmedHop>,
    dropped: Arc<AtomicU64>,
    /// The gate word the observer runs under, or [`INACTIVE`] after a block the gate refused.
    armed_from: u64,
    observer: LoudnessObserver,
}

/// The control half of the session's loudness meter (see the module notes).
pub(crate) struct LoudnessMonitor {
    gate: Arc<AtomicU64>,
    hops: Consumer<ArmedHop>,
    dropped: Arc<AtomicU64>,
    analysis: LoudnessAnalysis,
    /// The current arming's gate word; meaningful only while `armed`.
    arm: u64,
    armed: bool,
    observed_sample: u64,
    hop_true_peak: Option<[f32; 2]>,
}

/// A `LOUDNESS` operation the meter refuses: a start while it is armed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LoudnessActive;

fn queue_depth() -> NonZeroUsize {
    NonZeroUsize::new(LOUDNESS_QUEUE_DEPTH).unwrap_or(NonZeroUsize::MIN)
}

/// The exact layout of the meter's record queue, so the session charges what [`loudness_meter`]
/// allocates beside its two shared words and the analysis
/// ([`LoudnessAnalysis::ALLOCATION_BYTES`]).
pub(crate) fn loudness_queue_payload() -> Result<SpscRetainedPayload, SpscError> {
    bounded_spsc_retained_payload::<ArmedHop>(queue_depth())
}

/// The meter's two halves at the session rate, disarmed. Allocates the queue, two shared words
/// and the analysis; call it on a control thread.
pub(crate) fn loudness_meter(rate: u32) -> Option<(LoudnessTap, LoudnessMonitor)> {
    let observer = LoudnessObserver::new(rate).ok()?;
    let (producer, consumer) = bounded_spsc(queue_depth(), QueueGeneration(0)).ok()?;
    let gate = Arc::new(AtomicU64::new(INACTIVE));
    let dropped = Arc::new(AtomicU64::new(0));
    Some((
        LoudnessTap {
            gate: Arc::clone(&gate),
            hops: producer,
            dropped: Arc::clone(&dropped),
            armed_from: INACTIVE,
            observer,
        },
        LoudnessMonitor {
            gate,
            hops: consumer,
            dropped,
            analysis: LoudnessAnalysis::new(),
            arm: INACTIVE,
            armed: false,
            observed_sample: 0,
            hop_true_peak: None,
        },
    ))
}

// REALTIME_POLICY_BEGIN: #2115 S2, the loudness meter's render half.
impl LoudnessTap {
    /// Observes one rendered block of the master pair, which starts at `first_sample`.
    ///
    /// Loads the gate once. While it is inactive, or the block starts before it, nothing else
    /// runs. Otherwise a new gate word restarts the observer at this block, and each hop the
    /// block completes goes into the queue, or, when it is full, is dropped and counted.
    #[inline]
    pub(crate) fn observe(&mut self, left: &[f32], right: &[f32], first_sample: u64) {
        let gate = self.gate.load(Ordering::Acquire);
        if gate == INACTIVE || first_sample < gate {
            self.armed_from = INACTIVE;
            return;
        }
        if self.armed_from != gate {
            self.armed_from = gate;
            self.observer.restart(first_sample);
        }
        let hops = &mut self.hops;
        let dropped = &self.dropped;
        self.observer
            .observe(left, right, first_sample, &mut |record| {
                if hops.try_push(ArmedHop { arm: gate, record }).is_err() {
                    dropped.fetch_add(1, Ordering::AcqRel);
                }
            });
    }
}
// REALTIME_POLICY_END

/// A finite level for the wire, floored at [`protocol::LOUDNESS_LEVEL_FLOOR`]; `None` for a
/// non-finite one.
fn wire_level(level: f64) -> Option<f32> {
    #[allow(clippy::cast_possible_truncation)]
    let level = level as f32;
    level
        .is_finite()
        .then(|| level.max(protocol::LOUDNESS_LEVEL_FLOOR))
}

/// A linear true peak in dBTP, floored; a zero peak reads the floor.
fn wire_peak(peak: f32) -> f32 {
    let level = 20.0 * math::log10(f64::from(peak));
    wire_level(level).unwrap_or(protocol::LOUDNESS_LEVEL_FLOOR)
}

impl LoudnessMonitor {
    /// Moves every hop the queue holds into the analysis: at most the queue's depth, so the cost
    /// does not grow with the time since the last call. Hops of an earlier arming, or that arrive
    /// while the meter is disarmed, are discarded.
    pub(crate) fn drain(&mut self) {
        for _ in 0..LOUDNESS_QUEUE_DEPTH {
            let Ok(hop) = self.hops.try_pop() else {
                return;
            };
            if self.armed && hop.arm == self.arm {
                self.analysis.ingest(&hop.record);
                self.observed_sample = hop.record.end_sample;
                self.hop_true_peak = Some(hop.record.true_peak);
            }
        }
    }

    /// Runs one `LOUDNESS` operation at `render_sample`, the next block's first sample as render
    /// last published it, and answers the meter's state after it.
    ///
    /// * start: refused with [`LoudnessActive`] while armed; otherwise the analysis restarts and
    ///   the gate arms from `render_sample`.
    /// * read: drains, then answers.
    /// * reset: drains, then restarts integration at `render_sample`; M and S keep running.
    /// * stop: drains, then disarms; the readings stay readable.
    pub(crate) fn apply(
        &mut self,
        request: protocol::LoudnessRequest,
        render_sample: u64,
    ) -> Result<protocol::LoudnessReply, LoudnessActive> {
        match request {
            protocol::LoudnessRequest::Start => {
                if self.armed {
                    return Err(LoudnessActive);
                }
                // Disarmed, so every queued hop is discarded.
                self.drain();
                self.analysis.restart();
                self.arm = render_sample;
                self.armed = true;
                self.observed_sample = render_sample;
                self.hop_true_peak = None;
                self.gate.store(render_sample, Ordering::Release);
            }
            protocol::LoudnessRequest::Read => self.drain(),
            protocol::LoudnessRequest::Reset => {
                self.drain();
                self.analysis.reset(render_sample);
            }
            protocol::LoudnessRequest::Stop => {
                self.drain();
                self.gate.store(INACTIVE, Ordering::Release);
                self.armed = false;
            }
        }
        Ok(self.reply())
    }

    /// The meter's state as a `LOUDNESS` reply.
    fn reply(&self) -> protocol::LoudnessReply {
        let analysis = &self.analysis;
        let integrating_since = analysis.integrating_since();
        let [left, right] = analysis.true_peak();
        protocol::LoudnessReply {
            observed_sample: self.observed_sample,
            armed: self.armed,
            integrating_since,
            hops_dropped: self.dropped.load(Ordering::Acquire),
            sanitized_samples: analysis.sanitized_since_reset(),
            gated_block_count: analysis.gated_block_count(),
            momentary_lufs: analysis.momentary().and_then(wire_level),
            short_term_lufs: analysis.short_term().and_then(wire_level),
            integrated_lufs: analysis.integrated().and_then(wire_level),
            loudness_range_lu: analysis
                .loudness_range()
                .and_then(wire_level)
                .map(|range| range.max(0.0)),
            momentary_max_lufs: analysis.momentary_max().and_then(wire_level),
            short_term_max_lufs: analysis.short_term_max().and_then(wire_level),
            true_peak_dbtp: integrating_since.map(|_| [wire_peak(left), wire_peak(right)]),
            hop_true_peak_dbtp: self
                .hop_true_peak
                .map(|[left, right]| [wire_peak(left), wire_peak(right)]),
        }
    }
}
