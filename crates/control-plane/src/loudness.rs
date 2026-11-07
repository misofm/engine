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
//! **Arming.** The gate is one word: inactive, or the current arming's generation, which the
//! control half advances at every start and never reuses (#2265: never a sample, which a stop
//! and a start with no block between them would share). The first block that loads a new
//! generation restarts the observer at its first sample (the hop grid's origin). That block
//! starts at or after the `render_sample` the start read: render publishes `render_sample` after
//! the gate load of the block before it, and the start stores the gate after reading it. Each hop
//! carries the gate word it was observed under, and the control half ingests only hops of its
//! current arming, so a block in flight across a stop and a start can never feed the new
//! arming's analysis: its hops are discarded whole, and the new arming begins at the next block.
//!
//! **Acked-batch question: can an ack ever precede a drop? No.** A start acknowledges the arming
//! only, never a delivery. A hop is dropped on render before any reply can mention it; a reply
//! reports only hops already drained into the analysis, and carries the exact, monotone
//! `hops_dropped`. A reset or a stop changes only control state.

use std::num::NonZeroUsize;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use engine::realtime::{
    Consumer, Producer, QueueGeneration, SpscError, SpscRetainedPayload, bounded_spsc,
    bounded_spsc_retained_payload,
};
use loudness::{HopRecord, LoudnessAnalysis, LoudnessObserver};

/// A shared word that `--cfg loom` models: loom's atomic under loom, `std`'s otherwise, so
/// `loudness_arming_loom` explores the real gate and drop count.
#[cfg(not(loom))]
type ModelledWord = std::sync::atomic::AtomicU64;
#[cfg(loom)]
type ModelledWord = loom::sync::atomic::AtomicU64;

/// The hop record queue's depth: 5.12 s of 10 ms hops at 100 records per second, which outlasts
/// any service cadence either host runs (one tick per quantum in the browser, throttled to about
/// 1 s in a background tab; the C ABI's host-driven `miso_engine_v1_service`, which every session
/// control call also runs).
pub(crate) const LOUDNESS_QUEUE_DEPTH: usize = 512;

/// The gate word of a disarmed meter; every arming generation is above it.
const INACTIVE: u64 = 0;

/// One hop as the queue carries it: the record and the gate word (the arming generation) it was
/// observed under.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ArmedHop {
    arm: u64,
    record: HopRecord,
}

/// The render half of the session's loudness meter (see the module notes).
pub(crate) struct LoudnessTap {
    gate: Arc<ModelledWord>,
    hops: Producer<ArmedHop>,
    dropped: Arc<ModelledWord>,
    /// The arming generation the observer runs under, or [`INACTIVE`] after a block the gate
    /// refused.
    armed_under: u64,
    observer: LoudnessObserver,
}

/// The control half of the session's loudness meter (see the module notes).
pub(crate) struct LoudnessMonitor {
    gate: Arc<ModelledWord>,
    hops: Consumer<ArmedHop>,
    dropped: Arc<ModelledWord>,
    analysis: LoudnessAnalysis,
    /// The current arming's generation while armed, `None` while disarmed.
    arm: Option<u64>,
    /// Starts so far: the last generation issued, so the next start's is one above it.
    armings: u64,
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
    let gate = Arc::new(ModelledWord::new(INACTIVE));
    let dropped = Arc::new(ModelledWord::new(0));
    Some((
        LoudnessTap {
            gate: Arc::clone(&gate),
            hops: producer,
            dropped: Arc::clone(&dropped),
            armed_under: INACTIVE,
            observer,
        },
        LoudnessMonitor {
            gate,
            hops: consumer,
            dropped,
            analysis: LoudnessAnalysis::new(),
            arm: None,
            armings: 0,
            observed_sample: 0,
            hop_true_peak: None,
        },
    ))
}

// REALTIME_POLICY_BEGIN: #2115 S2, the loudness meter's render half.
impl LoudnessTap {
    /// Observes one rendered block of the master pair, which starts at `first_sample`.
    ///
    /// Loads the gate once. While it is inactive, nothing else runs. Otherwise a new arming
    /// generation restarts the observer at this block, and each hop the block completes goes
    /// into the queue, tagged with the generation, or, when it is full, is dropped and counted.
    #[inline]
    pub(crate) fn observe(&mut self, left: &[f32], right: &[f32], first_sample: u64) {
        let gate = self.gate.load(Ordering::Acquire);
        if gate == INACTIVE {
            self.armed_under = INACTIVE;
            return;
        }
        if self.armed_under != gate {
            self.armed_under = gate;
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
            if self.arm == Some(hop.arm) {
                self.analysis.ingest(&hop.record);
                self.observed_sample = hop.record.end_sample;
                self.hop_true_peak = Some(hop.record.true_peak);
            }
        }
    }

    /// Runs one `LOUDNESS` operation at `render_sample`, the next block's first sample as render
    /// last published it, and answers the meter's state after it.
    ///
    /// * start: refused with [`LoudnessActive`] while armed; otherwise the analysis restarts at
    ///   `render_sample` and the gate arms a new generation.
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
                if self.arm.is_some() {
                    return Err(LoudnessActive);
                }
                // Disarmed, so every queued hop is discarded.
                self.drain();
                self.analysis.restart();
                // One start per request: a u64 count of them does not wrap.
                self.armings += 1;
                self.arm = Some(self.armings);
                self.observed_sample = render_sample;
                self.hop_true_peak = None;
                self.gate.store(self.armings, Ordering::Release);
            }
            protocol::LoudnessRequest::Read => self.drain(),
            protocol::LoudnessRequest::Reset => {
                self.drain();
                self.analysis.reset(render_sample);
            }
            protocol::LoudnessRequest::Stop => {
                self.drain();
                self.gate.store(INACTIVE, Ordering::Release);
                self.arm = None;
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
            armed: self.arm.is_some(),
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

#[cfg(test)]
mod tests {
    use super::*;

    /// A hop of an earlier arming that reaches the queue after a stop and a new start at the
    /// same render sample (a block in flight across them on the render thread: no block was
    /// published between the two starts, so both read one `render_sample`) is discarded, never
    /// ingested.
    ///
    /// Test value: red if an arming's identity is anything two armings can share, such as the
    /// start's render sample (#2265), so that a hop of the earlier arming passes the current
    /// arming's check; only a render block racing a stop and a start produces such a hop, so no
    /// single-thread session test reaches it.
    #[test]
    fn a_record_of_an_earlier_arming_is_discarded() {
        let (mut tap, mut monitor) = loudness_meter(48_000).expect("loudness meter");
        monitor
            .apply(protocol::LoudnessRequest::Start, 960)
            .expect("start at 960");
        // The word a block that loads the gate now observes under, and tags its hops with.
        let earlier = monitor.gate.load(Ordering::Acquire);
        monitor
            .apply(protocol::LoudnessRequest::Stop, 960)
            .expect("stop");
        monitor
            .apply(protocol::LoudnessRequest::Start, 960)
            .expect("start at 960 again");
        let record = HopRecord {
            first_sample: 960,
            end_sample: 1_440,
            energy: [10.0; 2],
            true_peak: [0.5; 2],
            sanitized: 0,
        };
        for _ in 0..crate::loudness::LOUDNESS_QUEUE_DEPTH {
            tap.hops
                .try_push(ArmedHop {
                    arm: earlier,
                    record,
                })
                .expect("room in the queue");
        }
        let reply = monitor
            .apply(protocol::LoudnessRequest::Read, 1_440)
            .expect("read");
        assert_eq!(reply.observed_sample, 960, "no stale hop was delivered");
        assert_eq!(reply.integrating_since, None);
        assert_eq!(reply.gated_block_count, 0);
        assert_eq!(reply.momentary_lufs, None);
        assert_eq!(reply.hop_true_peak_dbtp, None);
    }

    /// Issue #2265: a block in flight across a stop and a start at the same render sample, under
    /// `loom`.
    ///
    /// The real [`LoudnessTap`] and [`LoudnessMonitor`], their real gate and drop words (loom's
    /// `AtomicU64` under `--cfg loom`) and the real `engine` SPSC (built under `--cfg loom`) run
    /// on two threads. Control arms at sample 0; render observes the block `[0, 960)`, two 10 ms
    /// hops at 48 kHz, while control stops and starts again at the same `render_sample` (no block
    /// was published between them), then reads. In every interleaving the new arming takes the
    /// block whole or not at all, and nothing of it when render observed it under the earlier
    /// arming.
    #[cfg(loom)]
    mod loudness_arming_loom {
        use super::{INACTIVE, loudness_meter};

        /// Loom runs the model's own closure on a small default stack, and the meter's observer
        /// and analysis are built and moved on the threads below, so each is spawned with an
        /// explicit stack.
        fn spawn_with_stack<T: Send + 'static>(
            body: impl FnOnce() -> T + Send + 'static,
        ) -> loom::thread::JoinHandle<T> {
            loom::thread::Builder::new()
                .stack_size(1 << 22)
                .spawn(body)
                .expect("loom thread")
        }

        #[test]
        fn loudness_arming_loom() {
            loom::model(|| spawn_with_stack(stop_and_start).join().expect("driver"));
        }

        fn stop_and_start() {
            const BLOCK: usize = 960;
            let (mut tap, mut monitor) = loudness_meter(48_000).expect("loudness meter");
            monitor
                .apply(protocol::LoudnessRequest::Start, 0)
                .expect("first start");
            let earlier = monitor.arm.expect("the first arming");
            let render = spawn_with_stack(move || {
                let left = [0.25_f32; BLOCK];
                let right = [-0.5_f32; BLOCK];
                tap.observe(&left, &right, 0);
                tap
            });
            monitor
                .apply(protocol::LoudnessRequest::Stop, 0)
                .expect("stop");
            monitor
                .apply(protocol::LoudnessRequest::Start, 0)
                .expect("a start at the same render sample");
            let current = monitor.arm.expect("the second arming");
            let tap = render.join().expect("render thread");
            let reply = monitor
                .apply(protocol::LoudnessRequest::Read, BLOCK as u64)
                .expect("read");
            let whole = (reply.integrating_since, reply.observed_sample) == (Some(0), BLOCK as u64);
            let none = (reply.integrating_since, reply.observed_sample) == (None, 0);
            assert!(
                whole || none,
                "the new arming took part of a block: {:?} to {}",
                reply.integrating_since,
                reply.observed_sample
            );
            if tap.armed_under == earlier || tap.armed_under == INACTIVE {
                assert!(none, "a block of the earlier arming fed the new one");
            } else {
                assert_eq!(tap.armed_under, current);
                assert!(whole, "a block of the new arming was lost");
            }
        }
    }
}
