//! Off-render preparation adapter for issue-007 builtins.
#![allow(missing_docs)]

use core::num::NonZeroUsize;
use std::collections::{BTreeMap, BTreeSet};

#[cfg(feature = "test-support")]
use core::sync::atomic::{AtomicBool, Ordering};

#[cfg(test)]
use builtins::builtin_filter_cutoff_maximum_hz;

#[cfg(feature = "test-support")]
use std::sync::Mutex;

use sha2::{Digest, Sha256};

use builtins::{
    BuiltinChain, BuiltinFaderBank, BuiltinInputBank, BuiltinLaneSelector, BuiltinMatrixBank,
    BuiltinParameterError, BuiltinParameters, BuiltinTail, ChannelParameters, DualMonoBlock,
    FaderMuteBuiltins, FaderMuteRampBuiltins, InputBuiltins, Matrix2x2, MatrixBuiltins,
    MeterAccumulator, MeterConfig, MeterConfigError, MeterHandle, MeterSnapshot, MeterTap,
    PreparedMeter, pan_matrix, validate_builtin_filter_cutoff,
};
use effect_contract::{
    BankWidth, ChannelSymmetryWitness, LiveConsoleRecord, SeamSide, SymmetryEvent,
};
use engine::realtime::{
    Consumer, PreparedRenderPlan, Producer, QueueGeneration, RenderEnvelope, RenderError,
    bounded_spsc, bounded_spsc_retained_payload,
};
use graph::{
    DependencyLevel, GraphBindingBlock, GraphBuiltinBankResourceEstimate, GraphNodeId,
    GraphNodeObserverBinding, GraphObservationBlock, GraphPreparedBuiltinBank,
    GraphPreparedBuiltinBankInfo, GraphPreparedBuiltinBankProcessor, GraphPreparedSourceSet,
    GraphRuntimeBindings, GraphRuntimeObserver, GraphRuntimeProcessor, PreparedGraphPlan,
    StableGraphId, TrackStage,
};
use lane::Backend;
use rack::{AoSoaScratch, BankSlotKey, RackLocation, RackProgram};
use rack_compiler::{CohortCandidate, CohortLevel, CohortPoolClass, plan_bank_groups};
use session::{CompiledSession, MatrixOrPan, Track};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BuiltinCompileCaps {
    pub maximum_total_state_bytes: u64,
    pub maximum_total_retained_payload_bytes: u64,
    pub maximum_total_meter_items: u64,
    pub maximum_total_meter_bytes: u64,
    pub maximum_single_allocation_bytes: u64,
    pub maximum_meter_streams: u64,
    pub maximum_period_frames: u32,
    pub maximum_peak_hold_frames: u32,
    pub maximum_smoothing_samples: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MeterRequest {
    pub handle: MeterHandle,
    pub track_id: String,
    pub tap: MeterTap,
    pub config: MeterConfig,
}

/// One live-console control record for a track's smoothed 2x2 matrix/pan stage (issue #137 D1).
///
/// # Why the matrix stage, and what the sentence here used to say
///
/// `BUILTIN_PARAMETER_DESCRIPTORS` is the builtin parameter ABI, and it is explicit about which
/// builtin parameters may move after preparation. When #137 D1 wrote this channel, the four
/// `matrix_ll/lr/rl/rr` rows were the **only** ones declaring
/// `BuiltinParameterUpdateRate::BlockTarget` and this comment said so, listing every other row as
/// `PreparedOnly`. That list has been overtaken twice and is not maintained here any more: #140 B
/// made `fader_db` and `mute` live ([`TrackFaderRecord`]), and #210 phase 3 made `trim_db` and
/// `polarity_invert` live ([`TrackInputRecord`]). The rows that remain `PreparedOnly` are
/// `hpf_hz`, `lpf_hz` and `delay_samples`, and the descriptor table is the authority rather than
/// this paragraph.
///
/// What is still true of *this* record, and is the reason it is its own type: it addresses the
/// matrix stage and nothing else, because a bounded SPSC queue has exactly one consumer and the
/// matrix stage is a different graph node from the fader and the input chain.
///
/// The record is `Copy` and fixed-size, so the queue is a plain `bounded_spsc` and the render-side
/// drain allocates nothing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrackControlRecord {
    /// New 2x2 target, already domain-checked by the producer.
    pub matrix: Matrix2x2,
    /// Ramp length in sample updates for this retarget.
    pub smoothing_samples: u32,
}

/// One live-console fader or mute record for a track's fader/mute stage (issue #140 B).
///
/// # Why this is a second record type and a second queue
///
/// The matrix stage and the fader stage are two different graph nodes, so one SPSC queue cannot
/// serve both -- a queue has exactly one consumer, and that consumer is the processor bound to the
/// node. `TrackControlRecord` therefore stays exactly the 20-byte matrix record #137 froze, and
/// this rides its own bounded queue to `TrackStage::PostFader`.
///
/// `fader_db` and `mute` declare `BuiltinParameterUpdateRate::BlockTarget` in
/// `BUILTIN_PARAMETER_DESCRIPTORS`. (This paragraph read "still declare `PreparedOnly`" until
/// the rows were flipped; the ABI table is the authority.) The *prepared* fader section,
/// `FaderMuteBuiltins`, genuinely has no post-preparation write path and is unchanged; what #140
/// adds is a distinct live section, `FaderMuteRampBuiltins`, bound only where a console asked
/// for one, and the parameter-metadata `liveUpdatable` flag is what tells a caller which of the
/// two a session is running.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrackFaderRecord {
    /// Retarget the addressed lanes' fader gain, in decibels, over an explicit ramp window.
    FaderDb {
        /// Which lane(s) this record addresses.
        lanes: BuiltinLaneSelector,
        /// New fader value in decibels, already domain-checked by the producer.
        db: f32,
        /// Ramp length in sample updates for this retarget.
        smoothing_samples: u32,
    },
    /// Set or clear the addressed lanes' mute, as a retarget of the same gain to `0`.
    Mute {
        /// Which lane(s) this record addresses.
        lanes: BuiltinLaneSelector,
        /// The new mute state.
        muted: bool,
        /// Ramp length in sample updates for the fade.
        smoothing_samples: u32,
    },
}

/// One live-console trim or polarity record for a track's **input** stage (#210 phase 3).
///
/// # Why this is a third record type and a third queue
///
/// The same reason [`TrackFaderRecord`] is a second one, one stage earlier: a queue has exactly
/// one consumer, and that consumer is whoever renders the addressed stage. The input section is a
/// different stage from the fader, on a different node and in a different bank, so it needs its
/// own channel. `TrackControlRecord` and `TrackFaderRecord` are unchanged.
///
/// # Why one record carries `Both` rather than two per-lane records
///
/// This is a deliberate departure from the effect-parameter lowering, where a `channel = both`
/// command on a `PerLane` parameter becomes *two* records
/// (`CommandRecord::into_effect_records`). It is the fader/mute lowering that is copied here
/// instead, and the reason is the channel-symmetry witness rather than economy: this record type
/// is **upstream of the seam**, so [`ChannelSymmetryWitness::admit`] reads it, and a pair of
/// per-lane records would present as two `Desymmetrize` events and retire the track's mono
/// collapse for the life of the plan -- on a command that changes both channels identically, at
/// one block boundary, over one window. `BuiltinLaneSelector::Both` says exactly what happened and
/// is admitted as [`SymmetryEvent::Preserve`], which is what
/// `SymmetryEvent::Preserve`'s own documentation says a both-channel retarget is. The effect
/// path splits because a launch effect counts a policy-violating span as invalid rather than
/// applying it; the builtins have no such constraint, and `BuiltinLaneSelector` exists precisely
/// to carry the distinction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrackInputRecord {
    /// Retarget the addressed lanes' input trim, in decibels, over an explicit ramp window.
    ///
    /// The lane's polarity is preserved: `trim_db` and `polarity_invert` are two parameters that
    /// share one coefficient, and a gain ride must not silently clear a flip.
    TrimDb {
        /// Which lane(s) this record addresses.
        lanes: BuiltinLaneSelector,
        /// New trim value in decibels, domain-checked against `trim_db`'s own declared range by
        /// the producer and again by the bank.
        db: f32,
        /// Ramp length in sample updates for this retarget.
        smoothing_samples: u32,
    },
    /// Set or clear the addressed lanes' polarity, as a retarget of the same coefficient to its
    /// own negation.
    ///
    /// The declick is the trim ramp's: the linear ramp carries the coefficient through zero over
    /// the requested window. There is no second DSP path.
    PolarityInvert {
        /// Which lane(s) this record addresses.
        lanes: BuiltinLaneSelector,
        /// The new polarity state.
        inverted: bool,
        /// Ramp length in sample updates for the flip.
        smoothing_samples: u32,
    },
}

impl LiveConsoleRecord for TrackInputRecord {
    /// The input chain is the first stage of the strip, before the fader and the matrix: a
    /// collapsed track runs it **once**, so every record on this queue gates the collapse.
    const SEAM: SeamSide = SeamSide::UpstreamOfSeam;

    fn symmetry_event(&self) -> SymmetryEvent {
        // Exhaustive, with no wildcard arm on purpose: a new variant is a compile error here,
        // which is the structural half of the hook rule.
        let lanes = match *self {
            Self::TrimDb { lanes, .. } | Self::PolarityInvert { lanes, .. } => lanes,
        };
        match lanes {
            BuiltinLaneSelector::Both => SymmetryEvent::Preserve,
            BuiltinLaneSelector::Left | BuiltinLaneSelector::Right => SymmetryEvent::Desymmetrize,
        }
    }
}

/// One requested live-console control channel, addressed by session track ID (issue #137 D1).
#[derive(Clone, Debug, PartialEq)]
pub struct TrackControlRequest {
    /// Session-stable track identity. It must name a track of the compiled session.
    pub track_id: String,
    /// Exact bounded depth of this track's control queue. A full queue is typed backpressure.
    pub queue_capacity: NonZeroUsize,
}

/// The control-side producer half of one prepared live-console control channel.
///
/// The consumer half is owned by the track's matrix processor inside the render plan, exactly as
/// `MeterConsumer` is the mirror image for metering. A producer must be dropped before the plan
/// that owns its consumer.
pub struct TrackControlProducer {
    /// Session-stable track identity this channel addresses.
    pub track_id: Box<str>,
    /// Bounded producer endpoint for the matrix/pan stage; `try_push` returns the record on a
    /// full queue.
    pub producer: Producer<TrackControlRecord>,
    /// Bounded producer endpoint for the fader/mute stage (issue #140 B), at the same depth.
    ///
    /// It is a field of the same struct rather than a second vector so that "one track, one
    /// console channel" stays one object with one lifetime: all three halves are created together,
    /// handed to the caller together, and dropped before the plan that owns their consumers.
    pub fader: Producer<TrackFaderRecord>,
    /// Bounded producer endpoint for the input trim/polarity stage (#210 phase 3), at the same
    /// depth, for the reason the field above states.
    pub input: Producer<TrackInputRecord>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BuiltinDiagnostic {
    pub code: &'static str,
    pub path: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuiltinDiagnosticSet(pub Vec<BuiltinDiagnostic>);

impl BuiltinDiagnosticSet {
    pub fn sorted(mut values: Vec<BuiltinDiagnostic>) -> Self {
        values.sort();
        values.dedup();
        Self(values)
    }
}

pub struct MeterConsumer {
    pub handle: MeterHandle,
    pub track_id: Box<str>,
    pub tap: MeterTap,
    pub consumer: Consumer<MeterSnapshot>,
}

/// The live console's three consumers for one track.
///
/// They travel together because they are leased together (`TrackControlProducer` holds all three
/// producers) and because whichever owner ends up rendering the track's fader also ends up
/// rendering its matrix and its input section -- a per-node triple, or three slots of one cohort
/// chain. Keeping them in one value is what makes "one track, one console channel" hold on the
/// consumer side too.
struct StripControlConsumers {
    /// `None` once a strip bank has claimed this side. The three sides are claimed together --
    /// `planned_strip_banks` plans all three stages over one track list -- so a partly claimed
    /// strip is unreachable, and `strip_bindings` says so rather than papering over it.
    input: Option<Consumer<TrackInputRecord>>,
    fader: Option<Consumer<TrackFaderRecord>>,
    matrix: Option<Consumer<TrackControlRecord>>,
}

/// Everything one track's fader and matrix stages need, before their binding form is decided.
///
/// Both shapes are built from this: the per-node processors [`PreparedBuiltinsSession::\
/// strip_bindings`] makes, and the bank lanes `into_graph_artifact_with_banks` gathers. The
/// prepared `fader`/`matrix` sections are the scalar fallback and `parameters` is what a bank
/// lane is built from -- prepared independently, exactly as `bank_inputs` is prepared beside the
/// scalar `InputBuiltins`, so selecting one never mutates the other.
struct StripPreparation {
    track_id: Box<str>,
    graph_id: StableGraphId,
    parameters: BuiltinParameters,
    /// The scalar input section, held here rather than bound eagerly since #210 phase 3, for the
    /// same reason the fader and the matrix are: whether a track's input is a per-node processor
    /// or one lane of a strip bank is a *lowering* decision, and the console consumer has to move
    /// to whichever owner wins. A track the input bank claims keeps this value as dead storage the
    /// bank never renders, exactly as a partly claimed fader does.
    input: InputBuiltins,
    fader: FaderMuteBuiltins,
    matrix: MatrixBuiltins,
    control: Option<StripControlConsumers>,
}

/// Opaque, sealed builtin payload. It can only be lowered into a graph once.
pub struct PreparedBuiltinsSession {
    seal: BuiltinSessionSeal,
    /// Post-input builtin bindings, one per track.
    ///
    /// The fader and matrix bindings are deliberately **not** here. Whether a track's fader is a
    /// per-node processor or one lane of a strip bank is a *lowering* decision -- it needs the
    /// selected dispatch backend and the graph's dependency levels, neither of which exists when
    /// a session is prepared -- so both stages are held as [`StripPreparation`] until then and
    /// bound by [`PreparedBuiltinsSession::strip_bindings`] (issue #212). The seal is unchanged:
    /// it still records three stages per track, and `processors_match` still proves all three are
    /// owned, counting the strip preparations alongside these bindings.
    processors: Vec<graph::GraphNodeBinding>,
    /// The fader and matrix section of every track, in normalized track order, with the live
    /// console's consumers where one drives the track.
    ///
    /// Declared after `processors` and before `track_controls` so a dropped preparation releases
    /// the producers before the consumers that own the ring storage.
    strips: Vec<StripPreparation>,
    bank_inputs: Vec<(Box<str>, InputBuiltins)>,
    observers: Vec<GraphNodeObserverBinding>,
    meter_consumers: Vec<MeterConsumer>,
    /// Issue #137 D1: control-side producers, sealed alongside the consumers bound into the
    /// matrix processors. Declared after `processors` so a dropped preparation releases the
    /// producers before the consumers that own the ring storage.
    track_controls: Vec<TrackControlProducer>,
    tails: Vec<(Box<str>, BuiltinTail)>,
    requests: Vec<MeterRequestSeal>,
    resources: BuiltinResourceEstimate,
}

/// The witness of a stage that is **seam-side by design**: fader, mute, pan and matrix.
///
/// Every term holds, unconditionally and without looking at a single word. That is not an
/// optimism: a collapsed track duplicates its one plane *into* these stages, so their per-channel
/// words are free to differ and must never gate the collapse. The measured evidence for the seam
/// sitting exactly here is a session with per-track asymmetric faders and non-identity pans whose
/// collapsed and dual renders are byte-identical.
const SEAM_SIDE_WITNESS: ChannelSymmetryWitness = ChannelSymmetryWitness::SYMMETRIC;

/// One strip input bank and the console channels of its member lanes (#210 phase 3).
///
/// # The third drain, and why it is at bank level
///
/// `TrackInputRecord` rides a bounded SPSC queue with exactly one consumer, and that consumer is
/// whoever renders the track's input section. The input builtins are **banked** --
/// `BuiltinInputBank` over eight member tracks -- so the consumer is this processor, draining
/// its members' queues in one loop, and not a per-track node sibling. The single-consumer property
/// is preserved structurally, exactly as it is for the fader and the matrix:
/// `into_graph_artifact_with_banks` moves each `Consumer` out of its `StripPreparation` once, so a
/// track's input is either a bank lane or a per-node [`ConsoleInputProcessor`] and never both.
///
/// # Why the drain is `begin_block` and not the first paragraph of `process`
///
/// This is the difference from [`FaderBankProcessor`], and it is load-bearing. The fader and the
/// matrix are **seam-side**: a collapsed chain duplicates its one plane *into* them, so a record
/// that writes one of their channels changes nothing the collapse dispatch needs to know, and
/// draining inside `process` is fine. The input section is **upstream** of the seam. A collapsed
/// block publishes the left plane on both channels, so a `Left` trim retarget admitted at block
/// `N` must be visible to the collapse dispatch *before* block `N` decides whether to run one
/// plane -- or the right channel would receive a retarget addressed to the left one, and the bits
/// would be wrong on the one block nobody would think to look at.
/// `rack::BankStage::begin_block` states the rule; `BankChain::run` drains every slot,
/// then reads the witness, then gathers. This drain is on the correct side of that ordering.
///
/// `try_pop` moves one `Copy` record and a retarget performs at most one division per channel;
/// neither allocates, locks, nor drops, which is what keeps the shipped artifact's render
/// call-graph gate green.
struct BuiltinBankProcessor {
    bank: BuiltinInputBank,
    /// One channel per bank lane; `None` for a lane no console addresses. Always `lanes` long, so
    /// the lane index is the array index and no lane map is stored.
    controls: Box<[Option<Consumer<TrackInputRecord>>]>,
    /// Each lane's **live** channel-symmetry terms, retained across blocks for the reason
    /// `EffectControlLane::symmetry` gives: the terms describe what the drained records did, so
    /// they have to survive the block that drained them.
    ///
    /// Inline and fixed at the widest bank rather than boxed: it is one byte per lane, and a
    /// second heap block per bank would cost more to retain than the array does.
    ///
    /// Only `LIVE` moves here. It is cleared by an admitted per-lane record and, like every other
    /// `LIVE` term in the tree, **never restored within a plan** -- see
    /// [`Self::lane_symmetry`] for why that is the conservative half of the M3 re-engage rule
    /// rather than a missing feature.
    live: [ChannelSymmetryWitness; MAXIMUM_BANK_LANES],
    process_calls: u64,
    frames_processed: u64,
}

/// Widest bank any backend selects (`BankWidth::Eight`), as a plain array bound.
const MAXIMUM_BANK_LANES: usize = 8;

impl GraphPreparedBuiltinBankProcessor for BuiltinBankProcessor {
    /// The third drain. Runs before the collapse dispatch reads the witness -- see the type's
    /// documentation for why that ordering is the whole reason this is not folded into `process`.
    fn begin_block(&mut self, first_sample: u64) -> Result<(), RenderError> {
        let _ = first_sample;
        let Self {
            bank,
            controls,
            live,
            ..
        } = self;
        for (lane, control) in controls.iter_mut().enumerate() {
            let Some(control) = control.as_mut() else {
                continue;
            };
            while let Ok(record) = control.try_pop() {
                // The one hook. `admit` takes the record by trait, not by kind, so a record type
                // added to this queue later cannot reach the render state without declaring what
                // it does to the witness (`effect_contract::symmetry::LiveConsoleRecord`).
                if let Some(witness) = live.get_mut(lane) {
                    witness.admit(&record);
                }
                match record {
                    TrackInputRecord::TrimDb {
                        lanes,
                        db,
                        smoothing_samples,
                    } => bank
                        .set_trim_db(lane, lanes, db, smoothing_samples)
                        .map_err(render_error)?,
                    TrackInputRecord::PolarityInvert {
                        lanes,
                        inverted,
                        smoothing_samples,
                    } => bank
                        .set_polarity_invert(lane, lanes, inverted, smoothing_samples)
                        .map_err(render_error)?,
                }
            }
        }
        Ok(())
    }

    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: u32,
        first_sample: u64,
    ) -> Result<(), RenderError> {
        let _ = first_sample;
        self.bank.process(left, right, frames);
        self.process_calls = self.process_calls.saturating_add(1);
        self.frames_processed = self.frames_processed.saturating_add(u64::from(frames));
        Ok(())
    }

    fn qualification_counters(&self) -> [u64; 2] {
        [self.process_calls, self.frames_processed]
    }

    /// The prepared designed-word comparison, conjoined with this lane's live terms.
    ///
    /// # Two independent guards on the same fact, and why both are kept
    ///
    /// `BuiltinInputBank::lane_symmetry` compares every word the input kernel loads per channel,
    /// and since #210 phase 3 that includes the **live** trim ramp record -- current, target, step
    /// and countdown -- so an asymmetric retarget is visible in the words on the very block it is
    /// admitted. The `LIVE` term this conjoins says the same thing from the other end: a record
    /// that addressed one lane was drained, whatever the words now say.
    ///
    /// They are kept side by side because they fail differently. The word comparison would call a
    /// no-op asymmetric retarget symmetric (retargeting the left channel to the value it already
    /// holds, with the right channel's window) and let the block collapse -- which is *correct*,
    /// but correct by an argument about arithmetic rather than by the structural rule the witness
    /// exists to enforce. The `LIVE` latch would allow a collapse the words forbid only if a
    /// designed word moved without a record, which cannot happen. A wrong collapse therefore needs
    /// both to be wrong at once.
    ///
    /// # What this means for re-engage (M3)
    ///
    /// `LIVE` is a latch: cleared by the drain, never set again within a plan. That is the same
    /// law `EffectControlLane` has carried since the witness existed, and it makes this bank
    /// **strictly stronger** than the M3 re-engage rule requires -- a track whose channels were
    /// driven apart by a per-lane trim ride does not come back even after the two words are made
    /// equal again and even if the integrators would prove agreement. Re-equal parameter words
    /// alone must not re-engage a collapse (M3's rule); here they cannot re-engage one at all.
    /// Making `LIVE` recoverable would be a change to the M-series machinery -- the proof would
    /// have to be consulted before the witness rather than after it -- and is deliberately not one
    /// this phase makes.
    fn lane_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
        let live = self
            .live
            .get(lane)
            .copied()
            .unwrap_or(ChannelSymmetryWitness::DECLINED);
        self.bank.lane_symmetry(lane).and(live)
    }

    fn supports_mono_collapse(&self) -> bool {
        self.bank.supports_mono_collapse()
    }

    fn process_mono(
        &mut self,
        left: &mut [f32],
        frames: u32,
        first_sample: u64,
    ) -> Result<(), RenderError> {
        let _ = first_sample;
        self.bank.process_mono(left, frames);
        self.process_calls = self.process_calls.saturating_add(1);
        self.frames_processed = self.frames_processed.saturating_add(u64::from(frames));
        Ok(())
    }

    fn desymmetrize(&mut self) {
        self.bank.desymmetrize();
    }

    /// Whether this bank can prove its two channels' state bit-equal (M3's way back).
    ///
    /// The walk covers the four integrators per channel and the whole trim-ramp record, which is
    /// this kernel's entire per-channel state -- exactly the words `desymmetrize` copies. It is
    /// asked only inside a recovery window, so a session that never disagrees never calls it.
    fn channels_agree(&self) -> bool {
        self.bank.channels_agree()
    }
}

/// One strip fader bank and the console channels of its member lanes (issue #212).
///
/// # The drain contract, at bank level
///
/// `TrackFaderRecord` rides a bounded SPSC queue with exactly one consumer, and that consumer is
/// whoever renders the track's fader. Banking moves the renderer from a per-track node to one lane
/// of this bank, so the consumer moves with it -- the queue, its depth, its record semantics and
/// its slot in the host's frozen queue-slot layout are all untouched. The single-consumer property
/// is preserved because it is preserved *structurally*: `into_graph_artifact_with_banks` moves
/// each `Consumer` out of its `StripPreparation` exactly once, so a track is either a bank lane or
/// a per-node processor and never both.
///
/// This is the bank-level wiring the plan calls for, and not a per-track node sibling: there is no
/// second drainer, and the drain is one loop over member lanes at the top of the block, before a
/// single sample is touched. An admitted move therefore takes effect at exactly the block boundary
/// the control side was acknowledged with, which is the same rule `ConsoleFaderProcessor` followed.
///
/// `try_pop` moves one `Copy` record and a retarget performs at most one division per channel;
/// neither allocates, locks, nor drops, which is what keeps the shipped artifact's render
/// call-graph gate green.
struct FaderBankProcessor {
    bank: BuiltinFaderBank,
    /// One channel per bank lane; `None` for a lane no console addresses. Always `lanes` long, so
    /// the lane index is the array index and no lane map is stored.
    controls: Box<[Option<Consumer<TrackFaderRecord>>]>,
    process_calls: u64,
    frames_processed: u64,
}

impl GraphPreparedBuiltinBankProcessor for FaderBankProcessor {
    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: u32,
        first_sample: u64,
    ) -> Result<(), RenderError> {
        let _ = first_sample;
        let Self { bank, controls, .. } = self;
        for (lane, control) in controls.iter_mut().enumerate() {
            let Some(control) = control.as_mut() else {
                continue;
            };
            while let Ok(record) = control.try_pop() {
                match record {
                    TrackFaderRecord::FaderDb {
                        lanes,
                        db,
                        smoothing_samples,
                    } => bank
                        .set_fader_db(lane, lanes, db, smoothing_samples)
                        .map_err(render_error)?,
                    TrackFaderRecord::Mute {
                        lanes,
                        muted,
                        smoothing_samples,
                    } => bank
                        .set_mute(lane, lanes, muted, smoothing_samples)
                        .map_err(render_error)?,
                }
            }
        }
        bank.process(left, right, frames);
        self.process_calls = self.process_calls.saturating_add(1);
        self.frames_processed = self.frames_processed.saturating_add(u64::from(frames));
        Ok(())
    }

    fn qualification_counters(&self) -> [u64; 2] {
        [self.process_calls, self.frames_processed]
    }

    /// Seam-side: see [`SEAM_SIDE_WITNESS`]. `TrackFaderRecord` is a seam-side record type, so
    /// its drain above deliberately folds nothing into a witness -- and could not, because
    /// `LiveConsoleRecord::SEAM` compiles the seam-side arm away.
    fn lane_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
        let _ = lane;
        SEAM_SIDE_WITNESS
    }

    /// The fader is the first stage that reads the duplicated plane, so the seam is immediately
    /// before it. It never runs one-plane: its two channels' gains and mutes are free to differ,
    /// and on the standing mono fixture 49 of 64 tracks' faders do.
    fn seam_side(&self) -> SeamSide {
        SeamSide::SeamSide
    }
}

/// One strip matrix/pan bank and the console channels of its member lanes (issue #212).
///
/// The mirror of [`FaderBankProcessor`], on `TrackControlRecord`, and it carries the same drain
/// contract for the same reason -- see that type for the argument.
struct MatrixBankProcessor {
    bank: BuiltinMatrixBank,
    /// One channel per bank lane; `None` for a lane no console addresses.
    controls: Box<[Option<Consumer<TrackControlRecord>>]>,
    process_calls: u64,
    frames_processed: u64,
}

impl GraphPreparedBuiltinBankProcessor for MatrixBankProcessor {
    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: u32,
        first_sample: u64,
    ) -> Result<(), RenderError> {
        let _ = first_sample;
        let Self { bank, controls, .. } = self;
        for (lane, control) in controls.iter_mut().enumerate() {
            let Some(control) = control.as_mut() else {
                continue;
            };
            while let Ok(record) = control.try_pop() {
                bank.set_target_smoothed(lane, record.matrix, record.smoothing_samples)
                    .map_err(render_error)?;
            }
        }
        bank.process(left, right, frames);
        self.process_calls = self.process_calls.saturating_add(1);
        self.frames_processed = self.frames_processed.saturating_add(u64::from(frames));
        Ok(())
    }

    fn qualification_counters(&self) -> [u64; 2] {
        [self.process_calls, self.frames_processed]
    }

    /// Seam-side: see [`SEAM_SIDE_WITNESS`]. The 2x2 matrix **is** the seam -- it is the earliest
    /// genuinely cross-channel operation in the strip -- so it is the one stage that is
    /// structurally guaranteed to sit on the free side of it.
    fn lane_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
        let _ = lane;
        SEAM_SIDE_WITNESS
    }

    /// Irreducibly cross-plane: `yl = ll*l + lr*r` reads both. See [`Self::lane_symmetry`].
    fn seam_side(&self) -> SeamSide {
        SeamSide::SeamSide
    }
}

/// Retained bytes of one strip bank's per-lane console-consumer array.
///
/// Charged whether or not a console is attached: the array is `lanes` long either way, and a lane
/// with no channel holds `None`. That is deliberate -- it is what makes a banked session's
/// retained payload independent of whether the host leased a console, rather than merely equal to
/// what it was before consoles existed.
fn strip_control_bytes<T: Copy + Send + 'static>(width: effect_contract::BankWidth) -> Option<u64> {
    u64::try_from(core::mem::size_of::<Option<Consumer<T>>>())
        .ok()?
        .checked_mul(u64::from(width.lanes()))
}

/// The whole per-bank processor cost of one bankable stage: the struct plus the heap it owns.
fn strip_processor_bytes(stage: TrackStage, width: effect_contract::BankWidth) -> Option<u64> {
    let inline = match stage {
        TrackStage::PostInputBuiltins => core::mem::size_of::<BuiltinBankProcessor>(),
        TrackStage::PostFader => core::mem::size_of::<FaderBankProcessor>(),
        TrackStage::PostMatrix => core::mem::size_of::<MatrixBankProcessor>(),
        _ => return None,
    };
    let inline = u64::try_from(inline).ok()?;
    match stage {
        TrackStage::PostInputBuiltins => {
            inline.checked_add(strip_control_bytes::<TrackInputRecord>(width)?)
        }
        TrackStage::PostFader => {
            inline.checked_add(strip_control_bytes::<TrackFaderRecord>(width)?)
        }
        _ => inline.checked_add(strip_control_bytes::<TrackControlRecord>(width)?),
    }
}

/// Folds one kind's estimate into a running total over every builtin bank kind.
fn add_bank_resource(
    total: GraphBuiltinBankResourceEstimate,
    add: GraphBuiltinBankResourceEstimate,
) -> Option<GraphBuiltinBankResourceEstimate> {
    Some(GraphBuiltinBankResourceEstimate {
        bank_count: total.bank_count.checked_add(add.bank_count)?,
        payload_bytes: total.payload_bytes.checked_add(add.payload_bytes)?,
        scratch_bytes: total.scratch_bytes.checked_add(add.scratch_bytes)?,
        scratch_samples: total.scratch_samples.checked_add(add.scratch_samples)?,
        metadata_bytes: total.metadata_bytes.checked_add(add.metadata_bytes)?,
        // A ceiling on the single largest allocation, so the maximum is taken and never summed.
        largest_allocation_bytes: total
            .largest_allocation_bytes
            .max(add.largest_allocation_bytes),
    })
}

/// The whole strip's bank plan: one group list per bankable stage, in render order.
///
/// The three stages are planned over the *same* track list with the *same* planner and the *same*
/// pool classes, so lane `i` of a cohort's fader bank is the same track as lane `i` of its
/// builtins bank wherever the dependency levels line up. Nothing relies on that: `runtime::chains_into` proves the lane-wise
/// relation on the lowered program and simply declines the merge where the orders disagree. What
/// planning them alike buys is that on a homogeneous session they *do* agree, and the strip fuses
/// into one chain per cohort.
/// The fader and the matrix are **seam-side** stages and their pool class is not about them: a
/// collapsed track duplicates its one plane *into* them, so their own per-channel words never gate
/// anything ([`SEAM_SIDE_WITNESS`]). They are pooled by the class anyway, and deliberately -- the
/// class here is the *track's*, not the stage's, so partitioning all three stages alike is exactly
/// what keeps a cohort's builtins, fader and matrix banks covering the same lanes in the same
/// order, which is the condition `runtime::chains_into` needs to fuse them into one chain. Pooling
/// only the upstream stage would have split the strip's chain on every mixed session.
fn planned_strip_banks(
    tracks: &[Box<str>],
    dispatch: Backend,
    levels: &[DependencyLevel],
    classes: &SessionPoolClasses,
) -> [(TrackStage, Vec<Box<[GraphNodeId]>>); 3] {
    [
        TrackStage::PostInputBuiltins,
        TrackStage::PostFader,
        TrackStage::PostMatrix,
    ]
    .map(|stage| {
        (
            stage,
            planned_builtin_bank_members(tracks, stage, dispatch, levels, classes),
        )
    })
}

/// The cohort key of the fixed post-input builtin stage.
///
/// It carries no fields on purpose: the stage is not selectable, its backend and width are fixed
/// for the whole artifact, and its rate and quantum come from the session envelope. Every
/// post-input node is therefore co-bankable with every other one at its dependency level, which
/// is exactly the cohort the planner forms. If a per-track variant is ever added (a quality, a
/// second section order), it becomes a field here and the planner splits the cohorts for free.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct BuiltinStageKey;

impl BankSlotKey for BuiltinStageKey {}

/// Groups every node of one bankable track stage, per dependency level, into `ceil(n / W)` banks.
///
/// The last bank of a level is short: it holds `1..=W` members and the bank pads the remaining
/// lanes with identity lanes.  Every post-input node on a vector host is therefore a bank member,
/// and scalar post-input bindings survive only when the backend has no bank width at all.
///
/// The grouping itself is **not implemented here**: it delegates to
/// [`rack_compiler::plan_bank_groups`], the workspace's single cohort planner (#96 F1).
/// Padding, level partitioning and the trailing-`None` lane order are that planner's rules, so
/// this function only turns each planned group back into the member list the graph attaches.
fn planned_builtin_bank_members(
    tracks: &[Box<str>],
    stage: TrackStage,
    dispatch: Backend,
    levels: &[DependencyLevel],
    classes: &SessionPoolClasses,
) -> Vec<Box<[GraphNodeId]>> {
    let Some(width) = BankWidth::for_backend(dispatch) else {
        return Vec::new();
    };
    let level_by_node: BTreeMap<_, _> = levels
        .iter()
        .flat_map(|level| {
            level
                .nodes
                .iter()
                .cloned()
                .map(move |node| (node, level.level))
        })
        .collect();
    let mut by_level = BTreeMap::<u64, Vec<CohortCandidate<GraphNodeId, BuiltinStageKey>>>::new();
    for track in tracks {
        let node = GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(track).expect("prepared stable ID"),
            stage,
        };
        if let Some(level) = level_by_node.get(&node).copied() {
            by_level.entry(level).or_default().push(CohortCandidate {
                id: node,
                program: RackProgram::new(RackLocation::Simd1, vec![BuiltinStageKey]),
                class: classes.class_of(track),
            });
        }
    }
    let levels_in: Vec<_> = by_level
        .into_iter()
        .map(|(level, candidates)| CohortLevel { level, candidates })
        .collect();
    let plan = plan_bank_groups(&levels_in, width)
        .expect("one node per track per stage, so ids are unique");
    // Every strip node is bankable, so the planner's scalar list is empty; if a future stage key
    // ever blocks banking, those tracks simply keep their scalar bindings.
    debug_assert!(plan.scalar.is_empty());
    plan.groups
        .into_iter()
        .map(|group| group.members.into_vec().into_iter().flatten().collect())
        .collect()
}

/// Builds one padded bank from `inputs.len()` tracks in member order.
///
/// Lanes `inputs.len()..width.lanes()` become the bank's identity lanes: the builtins crate owns
/// that contract (`BuiltinInputBank::new` accepts `1..=W` inputs and pads the rest), and this
/// is the single call site, so the compiler never builds a mask of its own.
fn build_input_bank(
    dispatch: Backend,
    width: effect_contract::BankWidth,
    inputs: Vec<InputBuiltins>,
) -> BuiltinInputBank {
    BuiltinInputBank::new(dispatch, width, inputs)
        .expect("planner emits 1..=W members at the width the selected backend chose")
}

/// Exact retained storage of one *kind* of builtin bank.
///
/// `processor_bytes` is the whole per-bank processor cost of that kind: the inline struct plus any
/// heap it owns. The three strip kinds differ only there -- an input bank owns nothing beyond its
/// kernel, while a fader or matrix bank owns a `lanes`-long array of optional console consumers --
/// so the rest of the accounting is shared rather than restated three times.
fn builtin_bank_resource(
    groups: &[Box<[GraphNodeId]>],
    width: effect_contract::BankWidth,
    quantum: u32,
    processor_bytes: u64,
) -> Option<GraphBuiltinBankResourceEstimate> {
    let bank_count = u64::try_from(groups.len()).ok()?;
    let lanes = u64::from(width.lanes());
    if groups
        .iter()
        .any(|members| members.is_empty() || members.len() as u64 > lanes)
    {
        return None;
    }
    let node_bytes = u64::try_from(core::mem::size_of::<GraphNodeId>()).ok()?;
    // Two planes: `AoSoaScratch` has no sidechain surface at all (#96 F9 deleted it).
    let scratch_plane_samples = u64::from(quantum).checked_mul(lanes)?;
    let scratch_plane_bytes = scratch_plane_samples.checked_mul(4)?;
    let scratch_samples_per_bank = scratch_plane_samples.checked_mul(2)?;
    let scratch_bytes_per_bank = scratch_samples_per_bank.checked_mul(4)?;
    let mut member_string_bytes = 0_u64;
    let mut largest_member_string = 0_u64;
    let mut payload_bytes = 0_u64;
    let mut largest_member_array = 0_u64;
    for members in groups {
        // A padded bank owns exactly the ids it holds, not a full-width array.
        let member_array_bytes = node_bytes.checked_mul(u64::try_from(members.len()).ok()?)?;
        largest_member_array = largest_member_array.max(member_array_bytes);
        payload_bytes = payload_bytes
            .checked_add(member_array_bytes)?
            .checked_add(processor_bytes)?;
        for member in members.iter() {
            let GraphNodeId::TrackStage { track_id, .. } = member else {
                return None;
            };
            let bytes = u64::try_from(track_id.as_str().len()).ok()?;
            member_string_bytes = member_string_bytes.checked_add(bytes)?;
            largest_member_string = largest_member_string.max(bytes);
        }
    }
    let payload_bytes = payload_bytes.checked_add(member_string_bytes)?;
    let scratch_samples = scratch_samples_per_bank.checked_mul(bank_count)?;
    let scratch_bytes = scratch_bytes_per_bank.checked_mul(bank_count)?;
    let metadata_bytes = u64::try_from(core::mem::size_of::<GraphPreparedBuiltinBank>())
        .ok()?
        .checked_mul(bank_count)?;
    Some(GraphBuiltinBankResourceEstimate {
        bank_count,
        payload_bytes,
        scratch_bytes,
        scratch_samples,
        metadata_bytes,
        largest_allocation_bytes: largest_member_array
            .max(processor_bytes)
            .max(largest_member_string)
            .max(scratch_plane_bytes)
            .max(metadata_bytes),
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BuiltinSessionSeal {
    session_sha256: [u8; 32],
    sample_rate: u32,
    quantum: u32,
    tracks: Vec<Box<str>>,
    processors: Vec<(Box<str>, TrackStage)>,
    tails: Vec<(Box<str>, BuiltinTail)>,
    requests: Vec<MeterRequestSeal>,
    observers: Vec<(Box<str>, TrackStage, u64)>,
    consumers: Vec<(u64, Box<str>, MeterTap)>,
    /// Issue #137 D1: `(track_id, queue_capacity)` per live-console control channel, sorted.
    controls: Vec<(Box<str>, usize)>,
    resources: BuiltinResourceEstimate,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct MeterRequestSeal {
    handle: u64,
    track_id: Box<str>,
    tap: MeterTap,
    reset_generation: u64,
    period_frames: u32,
    peak_hold_frames: u32,
    peak_decay_bits: u32,
    queue_capacity: usize,
}

type ObserverSeal = (Box<str>, TrackStage, u64);
type ConsumerSeal = (u64, Box<str>, MeterTap);

/// Test-only phase-two allocation accounting.  The production resource report deliberately
/// remains a layout calculation; this probe independently observes the allocator requests made
/// after phase-one validation has accepted the artifact.
#[cfg(feature = "test-support")]
static TEST_PHASE_TWO_ACTIVE: AtomicBool = AtomicBool::new(false);
#[cfg(feature = "test-support")]
static TEST_PHASE_TWO_LAYOUTS: Mutex<TestPhaseTwoLayoutTable> =
    Mutex::new(TestPhaseTwoLayoutTable::new());

#[cfg(feature = "test-support")]
struct TestPhaseTwoLayoutTable {
    values: [BuiltinRetainedLayout; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
    len: usize,
    overflowed: bool,
}

#[cfg(feature = "test-support")]
impl TestPhaseTwoLayoutTable {
    const fn new() -> Self {
        Self {
            values: [BuiltinRetainedLayout::ZERO; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
            len: 0,
            overflowed: false,
        }
    }

    fn clear(&mut self) {
        *self = Self::new();
    }

    fn record(&mut self, layout: core::alloc::Layout) {
        let Ok(size_bytes) = u64::try_from(layout.size()) else {
            self.overflowed = true;
            return;
        };
        let Ok(align_bytes) = u64::try_from(layout.align()) else {
            self.overflowed = true;
            return;
        };
        if let Some(value) = self.values[..self.len]
            .iter_mut()
            .find(|value| value.size_bytes == size_bytes && value.align_bytes == align_bytes)
        {
            let Some(count) = value.allocation_count.checked_add(1) else {
                self.overflowed = true;
                return;
            };
            value.allocation_count = count;
            return;
        }
        let Some(slot) = self.values.get_mut(self.len) else {
            self.overflowed = true;
            return;
        };
        *slot = BuiltinRetainedLayout {
            size_bytes,
            align_bytes,
            allocation_count: 1,
        };
        self.len += 1;
    }
}

/// Independent test-only phase-two allocation observation.
#[cfg(feature = "test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
#[doc(hidden)]
pub struct TestPhaseTwoAllocationSnapshot {
    pub total_bytes: u64,
    pub largest_allocation_bytes: u64,
    pub allocation_count: u64,
    pub layouts: Vec<BuiltinRetainedLayout>,
    pub overflowed: bool,
}

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_reset_phase_two_allocation_tracker() {
    TEST_PHASE_TWO_ACTIVE.store(false, Ordering::SeqCst);
    if let Ok(mut table) = TEST_PHASE_TWO_LAYOUTS.lock() {
        table.clear();
    }
}

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_record_phase_two_allocation(layout: core::alloc::Layout) {
    if !TEST_PHASE_TWO_ACTIVE.load(Ordering::Relaxed) {
        return;
    }
    if let Ok(mut table) = TEST_PHASE_TWO_LAYOUTS.lock() {
        table.record(layout);
    }
}

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_phase_two_allocation_snapshot() -> TestPhaseTwoAllocationSnapshot {
    let table = TEST_PHASE_TWO_LAYOUTS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut layouts = table.values[..table.len].to_vec();
    layouts.sort();
    let mut total_bytes = 0_u64;
    let mut allocation_count = 0_u64;
    let mut largest_allocation_bytes = 0_u64;
    let mut overflowed = table.overflowed;
    for layout in &layouts {
        let Some(bytes) = layout.size_bytes.checked_mul(layout.allocation_count) else {
            overflowed = true;
            continue;
        };
        let Some(total) = total_bytes.checked_add(bytes) else {
            overflowed = true;
            continue;
        };
        total_bytes = total;
        let Some(count) = allocation_count.checked_add(layout.allocation_count) else {
            overflowed = true;
            continue;
        };
        allocation_count = count;
        largest_allocation_bytes = largest_allocation_bytes.max(layout.size_bytes);
    }
    TestPhaseTwoAllocationSnapshot {
        total_bytes,
        largest_allocation_bytes,
        allocation_count,
        layouts,
        overflowed,
    }
}

#[cfg(feature = "test-support")]
struct TestPhaseTwoAllocationGuard;
#[cfg(feature = "test-support")]
impl TestPhaseTwoAllocationGuard {
    fn begin() -> Self {
        TEST_PHASE_TWO_ACTIVE.store(true, Ordering::SeqCst);
        Self
    }
}
#[cfg(feature = "test-support")]
impl Drop for TestPhaseTwoAllocationGuard {
    fn drop(&mut self) {
        TEST_PHASE_TWO_ACTIVE.store(false, Ordering::SeqCst);
    }
}

/// A graph plus genuine compiler-owned builtin bindings with no public parts-extraction seam.
///
/// `R` is caller-owned immutable graph-report metadata. All provenance-bearing fields stay
/// private to this crate, including the unbound graph and the concrete processor/observer parts.
pub struct PreparedBuiltinsGraphArtifact<R> {
    graph: PreparedGraphPlan,
    builtin_processors: Vec<graph::GraphNodeBinding>,
    builtin_observers: Vec<GraphNodeObserverBinding>,
    report: R,
    /// Issue #137 D1: control producers travel with the artifact so the one-way binding hands
    /// them to the caller together with the plan that owns their consumers.
    track_controls: Vec<TrackControlProducer>,
    meter_consumers: Vec<MeterConsumer>,
}

/// The one-way result of consuming and binding a sealed builtin graph artifact.
///
/// Field order is drop order: `track_controls` producers are released before the `plan` that owns
/// their consumer endpoints, and `meter_consumers` outlive the plan that owns their producers.
pub struct PreparedBuiltinsGraphBound {
    pub track_controls: Vec<TrackControlProducer>,
    pub plan: PreparedRenderPlan,
    pub meter_consumers: Vec<MeterConsumer>,
}

/// A rejected external binding preserves the opaque artifact and caller-owned bindings.
pub struct PreparedBuiltinsGraphBindFailure<R> {
    pub artifact: PreparedBuiltinsGraphArtifact<R>,
    pub bindings: GraphRuntimeBindings,
    pub code: &'static str,
}

/// A rejected source-set binding preserves the opaque artifact and every caller-owned input.
pub struct PreparedBuiltinsGraphSourceBindFailure<R> {
    pub artifact: PreparedBuiltinsGraphArtifact<R>,
    pub bindings: GraphRuntimeBindings,
    pub source_set: GraphPreparedSourceSet,
    pub code: &'static str,
}

/// Deliberate seal corruption available only to the graph compiler's adversarial tests.
///
/// This is not a wire format and is compiled out of production artifacts.  Keeping each seal
/// field independently reachable lets the graph boundary prove that it rejects the exact
/// corrupted tuple before it consumes either prepared input.
#[cfg(feature = "test-support")]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[doc(hidden)]
pub enum PreparedBuiltinsCorruption {
    /// The prepared session identity does not match the graph session.
    SessionIdentity,
    /// The sealed track set does not match the prepared bindings.
    Tracks,
    /// The sealed processor set does not match the prepared bindings.
    Processors,
    /// The retained tail records do not match their seal.
    Tails,
    /// The sealed meter request records do not match their seal.
    Requests,
    /// The sealed observer records do not match their bindings.
    Observers,
    /// The sealed consumer records do not match their queues.
    Consumers,
    /// The sealed resource report does not match the retained report.
    Resources,
}

/// Frozen corruption subcases within the eight seal categories.
#[cfg(feature = "test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc(hidden)]
pub enum PreparedBuiltinsCorruptionCase {
    SessionHash,
    SessionRate,
    SessionQuantum,
    TrackMissing,
    TrackExtra,
    TrackDuplicate,
    ProcessorMissing,
    ProcessorExtra,
    ProcessorChangedStage,
    TailMissing,
    TailExtra,
    TailChanged,
    RequestMissing,
    RequestExtra,
    RequestDuplicate,
    ObserverMissing,
    ObserverExtra,
    ObserverChangedNode,
    ConsumerMissing,
    ConsumerExtra,
    ConsumerChangedMetadata,
    ConsumerDuplicateHandle,
    ResourceReport,
}

#[cfg(feature = "test-support")]
impl PreparedBuiltinsCorruptionCase {
    #[must_use]
    pub const fn category(self) -> PreparedBuiltinsCorruption {
        match self {
            Self::SessionHash | Self::SessionRate | Self::SessionQuantum => {
                PreparedBuiltinsCorruption::SessionIdentity
            }
            Self::TrackMissing | Self::TrackExtra | Self::TrackDuplicate => {
                PreparedBuiltinsCorruption::Tracks
            }
            Self::ProcessorMissing | Self::ProcessorExtra | Self::ProcessorChangedStage => {
                PreparedBuiltinsCorruption::Processors
            }
            Self::TailMissing | Self::TailExtra | Self::TailChanged => {
                PreparedBuiltinsCorruption::Tails
            }
            Self::RequestMissing | Self::RequestExtra | Self::RequestDuplicate => {
                PreparedBuiltinsCorruption::Requests
            }
            Self::ObserverMissing | Self::ObserverExtra | Self::ObserverChangedNode => {
                PreparedBuiltinsCorruption::Observers
            }
            Self::ConsumerMissing
            | Self::ConsumerExtra
            | Self::ConsumerChangedMetadata
            | Self::ConsumerDuplicateHandle => PreparedBuiltinsCorruption::Consumers,
            Self::ResourceReport => PreparedBuiltinsCorruption::Resources,
        }
    }
}

/// The stable-ID grammar admits at most 127 distinct string allocation sizes. The remaining
/// entries cover every fixed processor, vector, endpoint and queue layout class without imposing
/// any limit on track, meter or allocation counts.
pub const BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY: usize = 160;

/// One exact `(size, alignment)` class in the retained allocation multiset.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct BuiltinRetainedLayout {
    pub size_bytes: u64,
    pub align_bytes: u64,
    pub allocation_count: u64,
}

impl BuiltinRetainedLayout {
    const ZERO: Self = Self {
        size_bytes: 0,
        align_bytes: 0,
        allocation_count: 0,
    };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BuiltinResourceEstimate {
    /// Exact engine-owned processor, seal and binding payload bytes retained by this artifact.
    pub engine_owned_processor_payload_bytes: u64,
    /// Exact engine-owned meter and queue payload bytes retained by this artifact.
    pub engine_owned_meter_payload_bytes: u64,
    /// Exact total of all engine-owned retained payload bytes in this artifact.
    pub engine_owned_retained_payload_bytes: u64,
    pub meter_items: u64,
    /// Largest requested engine-owned payload allocation retained by this artifact.
    pub maximum_single_allocation_bytes: u64,
    /// Count of retained engine-owned payload allocations represented by this report.
    pub retained_allocation_count: u64,
    /// Number of populated entries in [`Self::retained_layouts`].
    pub retained_layout_class_count: u16,
    /// Exact ordered multiset classes for all retained allocation requests.
    pub retained_layouts: [BuiltinRetainedLayout; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
}

impl Default for BuiltinResourceEstimate {
    fn default() -> Self {
        Self {
            engine_owned_processor_payload_bytes: 0,
            engine_owned_meter_payload_bytes: 0,
            engine_owned_retained_payload_bytes: 0,
            meter_items: 0,
            maximum_single_allocation_bytes: 0,
            retained_allocation_count: 0,
            retained_layout_class_count: 0,
            retained_layouts: [BuiltinRetainedLayout::ZERO; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
        }
    }
}

impl BuiltinResourceEstimate {
    /// Populated exact retained layout classes in deterministic `(size, align)` order.
    #[must_use]
    pub fn retained_layouts(&self) -> &[BuiltinRetainedLayout] {
        &self.retained_layouts[..usize::from(self.retained_layout_class_count)]
    }
}

/// Versioned public name for the exact engine-owned retained-payload report.
///
/// This is deliberately an alias rather than a duplicate accounting type: a caller cannot
/// accidentally read one report while the compiler validates another.
pub type BuiltinResourceReport = BuiltinResourceEstimate;

#[derive(Clone, Copy, Debug)]
struct ResourceAccumulator {
    total: u64,
    largest: u64,
    allocations: u64,
    layout_class_count: u16,
    layouts: [BuiltinRetainedLayout; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
}

impl Default for ResourceAccumulator {
    fn default() -> Self {
        Self {
            total: 0,
            largest: 0,
            allocations: 0,
            layout_class_count: 0,
            layouts: [BuiltinRetainedLayout::ZERO; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
        }
    }
}

impl ResourceAccumulator {
    fn add_layout(&mut self, layout: core::alloc::Layout) -> Option<()> {
        let size_bytes = u64::try_from(layout.size()).ok()?;
        let align_bytes = u64::try_from(layout.align()).ok()?;
        self.add_layout_count(size_bytes, align_bytes, 1)?;
        Some(())
    }

    fn add_layout_count(
        &mut self,
        size_bytes: u64,
        align_bytes: u64,
        allocation_count: u64,
    ) -> Option<()> {
        let retained_bytes = size_bytes.checked_mul(allocation_count)?;
        self.total = self.total.checked_add(retained_bytes)?;
        self.largest = self.largest.max(size_bytes);
        self.allocations = self.allocations.checked_add(allocation_count)?;
        let populated = usize::from(self.layout_class_count);
        if let Some(layout) = self.layouts[..populated]
            .iter_mut()
            .find(|layout| layout.size_bytes == size_bytes && layout.align_bytes == align_bytes)
        {
            layout.allocation_count = layout.allocation_count.checked_add(allocation_count)?;
            return Some(());
        }
        let slot = self.layouts.get_mut(populated)?;
        *slot = BuiltinRetainedLayout {
            size_bytes,
            align_bytes,
            allocation_count,
        };
        self.layout_class_count = self.layout_class_count.checked_add(1)?;
        Some(())
    }

    fn merge(&mut self, other: Self) -> Option<()> {
        for layout in &other.layouts[..usize::from(other.layout_class_count)] {
            self.add_layout_count(
                layout.size_bytes,
                layout.align_bytes,
                layout.allocation_count,
            )?;
        }
        Some(())
    }

    fn sorted_layouts(mut self) -> Self {
        self.layouts[..usize::from(self.layout_class_count)].sort();
        self
    }

    fn add_bytes(&mut self, bytes: usize) -> Option<()> {
        self.add_layout(core::alloc::Layout::from_size_align(bytes, 1).ok()?)
    }
}

#[derive(Clone, Copy, Debug)]
struct BuiltinResourcePlan {
    report: BuiltinResourceEstimate,
}

impl PreparedBuiltinsSession {
    /// Read-only retained-payload resource report.
    #[must_use]
    pub const fn resource_report(&self) -> BuiltinResourceEstimate {
        self.resources
    }

    /// Number of sealed builtin processor bindings.
    ///
    /// Three per track, as it has always been -- but all three are now held as a
    /// `StripPreparation` until their binding form is chosen (issue #212 for the fader and the
    /// matrix, #210 phase 3 for the input) and are counted here in the shape they will take. The
    /// `processors` term is what a lowering has already built, and is zero for a session
    /// preparation has only just returned.
    #[must_use]
    pub fn processor_count(&self) -> usize {
        self.processors.len() + self.strips.len() * 3
    }

    /// Number of sealed builtin tails.
    #[must_use]
    pub fn tail_count(&self) -> usize {
        self.tails.len()
    }

    /// Number of sealed meter observer bindings.
    #[must_use]
    pub fn observer_count(&self) -> usize {
        self.observers.len()
    }

    /// Number of sealed meter consumer endpoints.
    #[must_use]
    pub fn meter_consumer_count(&self) -> usize {
        self.meter_consumers.len()
    }

    /// Number of sealed live-console control channels (issue #137 D1).
    #[must_use]
    pub fn track_control_count(&self) -> usize {
        self.track_controls.len()
    }

    /// Read-only builtin tails used by graph lowering.
    pub fn tails(&self) -> impl Iterator<Item = (&str, BuiltinTail)> {
        self.tails
            .iter()
            .map(|(track, tail)| (track.as_ref(), *tail))
    }

    /// Validate the immutable payload against the exact effect-prepared session.
    pub fn validate_for_session(&self, session: &CompiledSession) -> BuiltinDiagnosticSet {
        let mut diagnostics = Vec::new();
        if self.seal.session_sha256 != session_identity(session)
            || self.seal.sample_rate != session.sample_rate().0
            || self.seal.quantum != session.quantum().0
        {
            diagnostics.push(diag("builtin.session.mismatch", "$.session"));
        }
        let expected_tracks: Vec<Box<str>> = session
            .normalized_model()
            .tracks
            .iter()
            .map(|track| track.id.as_str().into())
            .collect();
        if self.seal.tracks != expected_tracks {
            diagnostics.push(diag("builtin.prepared.track_set", "$.builtins.tracks"));
        }
        let expected_processors = processor_seal(&expected_tracks);
        if self.seal.processors != expected_processors
            || !processors_match(&self.processors, &self.strips, &expected_processors)
        {
            diagnostics.push(diag(
                "builtin.prepared.processor_set",
                "$.builtins.processors",
            ));
        }
        let expected_tails = match expected_tails(session) {
            Ok(value) => value,
            Err(()) => {
                diagnostics.push(diag("builtin.prepared.tail_set", "$.builtins.tails"));
                Vec::new()
            }
        };
        if self.seal.tails != expected_tails || self.tails != expected_tails {
            diagnostics.push(diag("builtin.prepared.tail_set", "$.builtins.tails"));
        }
        let (actual_observers, actual_consumers) =
            actual_meter_seals(&self.observers, &self.meter_consumers);
        if self.seal.requests != self.requests {
            diagnostics.push(diag(
                "builtin.prepared.request_set",
                "$.builtins.meter_requests",
            ));
        }
        if self.seal.observers != actual_observers {
            diagnostics.push(diag(
                "builtin.prepared.observer_set",
                "$.builtins.observers",
            ));
        }
        if self.seal.consumers != actual_consumers {
            diagnostics.push(diag(
                "builtin.prepared.consumer_set",
                "$.builtins.meter_consumers",
            ));
        }
        // Issue #137 D1: the control seal is the producer set, so a lost, duplicated, or
        // retargeted control channel is a prepared-set mismatch exactly like a lost meter.
        let mut actual_controls: Vec<(Box<str>, usize)> = self
            .track_controls
            .iter()
            .map(|control| (control.track_id.clone(), control.producer.capacity()))
            .collect();
        actual_controls.sort_unstable();
        if self.seal.controls != actual_controls {
            diagnostics.push(diag(
                "builtin.prepared.control_set",
                "$.builtins.track_controls",
            ));
        }
        if self.seal.resources != self.resources {
            diagnostics.push(diag(
                "builtin.prepared.resource_report",
                "$.builtins.resources",
            ));
        }
        BuiltinDiagnosticSet::sorted(diagnostics)
    }

    /// Seal an already session-validated graph around these genuine compiler-owned bindings.
    ///
    /// This is deliberately a one-way conversion: callers may carry and bind the resulting
    /// artifact, but cannot extract, replace, or clone its provenance-bearing parts.
    pub fn into_graph_artifact<R>(
        mut self,
        graph: PreparedGraphPlan,
        report: R,
    ) -> PreparedBuiltinsGraphArtifact<R> {
        let mut processors = core::mem::take(&mut self.processors);
        processors.append(&mut self.strip_bindings());
        PreparedBuiltinsGraphArtifact {
            graph,
            builtin_processors: processors,
            builtin_observers: self.observers,
            report,
            track_controls: self.track_controls,
            meter_consumers: self.meter_consumers,
        }
    }

    /// The per-node input, fader and matrix bindings of every track still holding a
    /// [`StripPreparation`].
    ///
    /// A track a strip bank claimed had its parts moved out first, so it is simply not here. This
    /// is the *only* place the per-node shapes are built, which is what keeps "a track is a bank
    /// lane or a per-node processor, never both" a structural fact rather than a rule.
    ///
    /// The input stage joined this list in #210 phase 3, for the reason the stage comment on
    /// [`StripPreparation::input`] gives: once the input section has a live console channel, which
    /// owner drains it is a lowering decision, and a binding built at preparation could not hand
    /// its consumer to a bank.
    fn strip_bindings(&mut self) -> Vec<graph::GraphNodeBinding> {
        let mut bindings = Vec::with_capacity(self.strips.len() * 3);
        for strip in core::mem::take(&mut self.strips) {
            let StripPreparation {
                graph_id,
                parameters,
                input,
                fader,
                matrix,
                control,
                ..
            } = strip;
            let (input_processor, fader_processor, matrix_processor): (
                Box<dyn GraphRuntimeProcessor>,
                Box<dyn GraphRuntimeProcessor>,
                Box<dyn GraphRuntimeProcessor>,
            ) = match control {
                None => (
                    Box::new(InputProcessor(input)),
                    Box::new(FaderProcessor(fader)),
                    Box::new(MatrixProcessor(matrix)),
                ),
                Some(control) => {
                    // Issue #140 B: the live fader is a *separate* section, built from the same
                    // prepared parameters, and preparation already proved the domain.
                    let ramped = FaderMuteRampBuiltins::new(parameters)
                        .expect("preparation validated the ramped fader's gain domain");
                    (
                        Box::new(ConsoleInputProcessor {
                            input,
                            control: control
                                .input
                                .expect("a strip is banked on all three stages or on none"),
                            live: ChannelSymmetryWitness::SYMMETRIC,
                        }),
                        Box::new(ConsoleFaderProcessor {
                            fader: ramped,
                            control: control
                                .fader
                                .expect("a strip is banked on all three stages or on none"),
                        }),
                        Box::new(ConsoleMatrixProcessor {
                            matrix,
                            control: control
                                .matrix
                                .expect("a strip is banked on all three stages or on none"),
                        }),
                    )
                }
            };
            bindings.push(graph::GraphNodeBinding::new(
                stage_node(graph_id.clone(), TrackStage::PostInputBuiltins),
                input_processor,
            ));
            bindings.push(graph::GraphNodeBinding::new(
                stage_node(graph_id.clone(), TrackStage::PostFader),
                fader_processor,
            ));
            bindings.push(graph::GraphNodeBinding::new(
                stage_node(graph_id, TrackStage::PostMatrix),
                matrix_processor,
            ));
        }
        bindings
    }

    /// Every track's prepared **input-builtins** channel-symmetry witness, in track order.
    ///
    /// The upstream-of-seam half of this crate's contribution to a track's pool class: the input
    /// section is the one stage here that a collapse would run once, so its designed-word
    /// comparison gates the class. The fader and the matrix are seam-side (`SEAM_SIDE_WITNESS`)
    /// and are deliberately absent -- a collapsed track duplicates its plane *into* them, so
    /// folding their vacuously-symmetric witness in here would add a term that can never be false
    /// and read, to a later caller, as if the seam had been checked.
    ///
    /// Read from the **bank** copy of each track's input section, which is the one a banked plan
    /// renders. It is prepared from the same parameters as the scalar fallback by the same
    /// `BuiltinChain::new` call shape, so the two cannot design different words.
    pub fn input_channel_symmetry(&self) -> impl Iterator<Item = (&str, ChannelSymmetryWitness)> {
        self.bank_inputs
            .iter()
            .map(|(track, input)| (track.as_ref(), input.channel_symmetry()))
    }

    /// Exact retained resource addition for the selected production bank layout.
    ///
    /// This is a read-only transactional preflight: graph/session caps can reject the final
    /// artifact while both prepared inputs are still owned by their caller.
    pub fn graph_builtin_bank_resource(
        &self,
        dispatch: Backend,
        levels: &[DependencyLevel],
        classes: &SessionPoolClasses,
    ) -> Option<GraphBuiltinBankResourceEstimate> {
        let Some(width) = BankWidth::for_backend(dispatch) else {
            return Some(GraphBuiltinBankResourceEstimate::default());
        };
        let mut total = GraphBuiltinBankResourceEstimate::default();
        for (stage, groups) in planned_strip_banks(&self.seal.tracks, dispatch, levels, classes) {
            let processor_bytes = strip_processor_bytes(stage, width)?;
            let kind = builtin_bank_resource(&groups, width, self.seal.quantum, processor_bytes)?;
            total = add_bank_resource(total, kind)?;
        }
        Some(total)
    }

    /// Materialize post-input builtin banks using the already-selected host dispatch.
    ///
    /// Every post-input node in a level with a vector backend is banked; the last bank of a
    /// level is padded with identity lanes.  Scalar `InputProcessor` bindings remain only when
    /// `BankWidth::for_backend(dispatch)` is `None`.
    ///
    /// Lowering is infallible after `graph_builtin_bank_resource`: `with_builtin_banks` consumes
    /// the plan on error, so the read-only preflight is what makes the attach transactional and
    /// the `expect`s here are this crate's own planner invariants.
    pub fn into_graph_artifact_with_banks<R>(
        mut self,
        graph: PreparedGraphPlan,
        report: R,
        dispatch: Backend,
        levels: &[DependencyLevel],
        classes: &SessionPoolClasses,
    ) -> PreparedBuiltinsGraphArtifact<R> {
        let Some(width) = BankWidth::for_backend(dispatch) else {
            return self.into_graph_artifact(graph, report);
        };
        let plan = planned_strip_banks(&self.seal.tracks, dispatch, levels, classes);
        if plan.iter().all(|(_, groups)| groups.is_empty()) {
            return self.into_graph_artifact(graph, report);
        }
        let mut resource = GraphBuiltinBankResourceEstimate::default();
        for (stage, groups) in &plan {
            let kind = builtin_bank_resource(
                groups,
                width,
                self.seal.quantum,
                strip_processor_bytes(*stage, width).expect("bankable stage"),
            )
            .expect("preflighted builtin-bank resource");
            resource =
                add_bank_resource(resource, kind).expect("preflighted builtin-bank resource");
        }

        let mut bank_inputs: BTreeMap<Box<str>, InputBuiltins> =
            core::mem::take(&mut self.bank_inputs).into_iter().collect();
        // The strip preparations are indexed by track and *consumed* by whichever bank claims
        // them, so a track's console consumers can only ever reach one owner. Whatever is left
        // afterwards keeps its per-node bindings.
        let mut strips: BTreeMap<Box<str>, StripPreparation> = core::mem::take(&mut self.strips)
            .into_iter()
            .map(|strip| (strip.track_id.clone(), strip))
            .collect();
        let lanes = width.lanes() as usize;
        let mut selected = BTreeSet::new();
        let mut claimed_input: BTreeSet<Box<str>> = BTreeSet::new();
        let mut claimed_fader: BTreeSet<Box<str>> = BTreeSet::new();
        let mut claimed_matrix: BTreeSet<Box<str>> = BTreeSet::new();
        let mut graph_banks = Vec::new();

        for (stage, groups) in plan {
            for members in groups {
                let track_of = |member: &GraphNodeId| -> Box<str> {
                    let GraphNodeId::TrackStage { track_id, .. } = member else {
                        unreachable!("prepared strip member shape")
                    };
                    Box::<str>::from(track_id.as_str())
                };
                let processor: Box<dyn GraphPreparedBuiltinBankProcessor> = match stage {
                    TrackStage::PostInputBuiltins => {
                        let mut controls: Vec<Option<Consumer<TrackInputRecord>>> =
                            (0..lanes).map(|_| None).collect();
                        let mut inputs = Vec::with_capacity(members.len());
                        for (lane, member) in members.iter().enumerate() {
                            inputs.push(
                                bank_inputs
                                    .remove(track_of(member).as_ref())
                                    .expect("planner members are owned prepared builtin tracks"),
                            );
                            let strip = strips
                                .get_mut(track_of(member).as_ref())
                                .expect("planner members are owned prepared strip tracks");
                            controls[lane] = strip
                                .control
                                .as_mut()
                                .and_then(|control| control.input.take());
                            claimed_input.insert(strip.track_id.clone());
                        }
                        Box::new(BuiltinBankProcessor {
                            bank: build_input_bank(dispatch, width, inputs),
                            controls: controls.into_boxed_slice(),
                            live: [ChannelSymmetryWitness::SYMMETRIC; MAXIMUM_BANK_LANES],
                            process_calls: 0,
                            frames_processed: 0,
                        })
                    }
                    TrackStage::PostFader => {
                        let mut controls: Vec<Option<Consumer<TrackFaderRecord>>> =
                            (0..lanes).map(|_| None).collect();
                        let mut parameters = Vec::with_capacity(members.len());
                        for (lane, member) in members.iter().enumerate() {
                            let strip = strips
                                .get_mut(track_of(member).as_ref())
                                .expect("planner members are owned prepared strip tracks");
                            parameters.push(strip.parameters);
                            controls[lane] = strip
                                .control
                                .as_mut()
                                .and_then(|control| control.fader.take());
                            claimed_fader.insert(strip.track_id.clone());
                        }
                        Box::new(FaderBankProcessor {
                            bank: BuiltinFaderBank::new(dispatch, width, parameters)
                                .expect("planner emits 1..=W members at the selected width"),
                            controls: controls.into_boxed_slice(),
                            process_calls: 0,
                            frames_processed: 0,
                        })
                    }
                    TrackStage::PostMatrix => {
                        let mut controls: Vec<Option<Consumer<TrackControlRecord>>> =
                            (0..lanes).map(|_| None).collect();
                        let mut targets = Vec::with_capacity(members.len());
                        for (lane, member) in members.iter().enumerate() {
                            let strip = strips
                                .get_mut(track_of(member).as_ref())
                                .expect("planner members are owned prepared strip tracks");
                            targets.push((
                                strip.parameters.matrix,
                                strip.parameters.smoothing_samples,
                            ));
                            controls[lane] = strip
                                .control
                                .as_mut()
                                .and_then(|control| control.matrix.take());
                            claimed_matrix.insert(strip.track_id.clone());
                        }
                        Box::new(MatrixBankProcessor {
                            bank: BuiltinMatrixBank::new(dispatch, width, targets)
                                .expect("preparation validated every matrix coefficient"),
                            controls: controls.into_boxed_slice(),
                            process_calls: 0,
                            frames_processed: 0,
                        })
                    }
                    _ => unreachable!("planned_strip_banks yields only bankable stages"),
                };
                selected.extend(members.iter().cloned());
                graph_banks.push(GraphPreparedBuiltinBank {
                    backend: dispatch,
                    members,
                    processor,
                    scratch: AoSoaScratch::new(width, self.seal.quantum)
                        .expect("prepared nonzero graph quantum"),
                });
            }
        }

        // A track whose input, fader and matrix all went into banks has nothing left to bind; one
        // whose stages were only partly claimed keeps all three per-node bindings, and the parts
        // that were moved out are dead storage the banks never render. That case cannot arise from
        // `planned_strip_banks` -- it plans all three stages over one track list -- and the drain
        // ownership is still single, because a `take`n consumer is `None` on the side that lost it.
        // `planned_strip_banks` plans all three stages over one track list, so the three claimed
        // sets are the same set. Stated as an assertion rather than assumed: the retain below is
        // written for the partly-claimed case because that case must not silently bind a track
        // twice, and this is what says the case is unreachable rather than merely unhandled.
        debug_assert_eq!(
            claimed_input, claimed_fader,
            "the planner claims stages together"
        );
        debug_assert_eq!(
            claimed_input, claimed_matrix,
            "the planner claims stages together"
        );
        strips.retain(|track, _| {
            !(claimed_input.contains(track)
                && claimed_fader.contains(track)
                && claimed_matrix.contains(track))
        });
        self.strips = strips.into_values().collect();
        let mut processors = core::mem::take(&mut self.processors);
        processors.retain(|binding| !selected.contains(&binding.node));
        processors.append(&mut self.strip_bindings());

        let graph = graph
            .with_builtin_banks(graph_banks, resource)
            .expect("validated fixed builtin member shape");
        PreparedBuiltinsGraphArtifact {
            graph,
            builtin_processors: processors,
            builtin_observers: self.observers,
            report,
            track_controls: self.track_controls,
            meter_consumers: self.meter_consumers,
        }
    }

    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn test_only_corrupt_for_compiler_test(
        &mut self,
        corruption: PreparedBuiltinsCorruptionCase,
    ) {
        match corruption {
            PreparedBuiltinsCorruptionCase::SessionHash => self.seal.session_sha256[0] ^= 1,
            PreparedBuiltinsCorruptionCase::SessionRate => {
                self.seal.sample_rate = self.seal.sample_rate.checked_add(1).unwrap_or(0);
            }
            PreparedBuiltinsCorruptionCase::SessionQuantum => {
                self.seal.quantum = self.seal.quantum.checked_add(1).unwrap_or(0);
            }
            PreparedBuiltinsCorruptionCase::TrackMissing => self.seal.tracks.clear(),
            PreparedBuiltinsCorruptionCase::TrackExtra => {
                self.seal.tracks.push("forged-track".into());
            }
            PreparedBuiltinsCorruptionCase::TrackDuplicate => {
                if let Some(track) = self.seal.tracks.first().cloned() {
                    self.seal.tracks.push(track);
                }
            }
            // The seal records three stages per track and preparation owns them in two places
            // since #212 -- one binding plus one `StripPreparation` -- so dropping either has to
            // be caught. `strips.pop()` removes a track's fader *and* matrix stages at once,
            // which is the shape a lost strip actually takes.
            PreparedBuiltinsCorruptionCase::ProcessorMissing => {
                if self.processors.pop().is_none() {
                    self.strips.pop();
                }
            }
            PreparedBuiltinsCorruptionCase::ProcessorExtra => self
                .seal
                .processors
                .push(("forged-processor".into(), TrackStage::PostMatrix)),
            PreparedBuiltinsCorruptionCase::ProcessorChangedStage => {
                if let Some((_, stage)) = self.seal.processors.first_mut() {
                    *stage = TrackStage::Input;
                }
            }
            PreparedBuiltinsCorruptionCase::TailMissing => {
                self.tails.pop();
            }
            PreparedBuiltinsCorruptionCase::TailExtra => self
                .seal
                .tails
                .push(("forged-tail".into(), BuiltinTail::FiniteZero)),
            PreparedBuiltinsCorruptionCase::TailChanged => {
                if let Some((_, tail)) = self.tails.first_mut() {
                    *tail = match *tail {
                        BuiltinTail::FiniteZero => BuiltinTail::Infinite,
                        BuiltinTail::Infinite => BuiltinTail::FiniteZero,
                    };
                }
            }
            PreparedBuiltinsCorruptionCase::RequestMissing => {
                self.requests.pop();
            }
            PreparedBuiltinsCorruptionCase::RequestExtra => {
                self.seal.requests.push(forged_request_seal());
            }
            PreparedBuiltinsCorruptionCase::RequestDuplicate => {
                if let Some(request) = self.requests.first().cloned() {
                    self.requests.push(request);
                }
            }
            PreparedBuiltinsCorruptionCase::ObserverMissing => {
                self.observers.pop();
            }
            PreparedBuiltinsCorruptionCase::ObserverExtra => {
                self.seal
                    .observers
                    .push(("forged-observer".into(), TrackStage::Input, u64::MAX))
            }
            PreparedBuiltinsCorruptionCase::ObserverChangedNode => {
                if let Some(observer) = self.observers.first_mut() {
                    observer.node = GraphNodeId::Output {
                        output_id: StableGraphId::parse("forged-output").expect("stable test ID"),
                    };
                }
            }
            PreparedBuiltinsCorruptionCase::ConsumerMissing => {
                self.meter_consumers.pop();
            }
            PreparedBuiltinsCorruptionCase::ConsumerExtra => {
                self.seal
                    .consumers
                    .push((u64::MAX, "forged-consumer".into(), MeterTap::Input))
            }
            PreparedBuiltinsCorruptionCase::ConsumerChangedMetadata => {
                if let Some(consumer) = self.meter_consumers.first_mut() {
                    consumer.track_id = "forged-consumer".into();
                    consumer.tap = MeterTap::PostMatrix;
                }
            }
            PreparedBuiltinsCorruptionCase::ConsumerDuplicateHandle => {
                if self.meter_consumers.len() >= 2 {
                    self.meter_consumers[1].handle = self.meter_consumers[0].handle;
                }
            }
            PreparedBuiltinsCorruptionCase::ResourceReport => {
                self.resources.engine_owned_retained_payload_bytes = self
                    .resources
                    .engine_owned_retained_payload_bytes
                    .checked_add(1)
                    .unwrap_or(0);
            }
        }
    }
}

#[cfg(feature = "test-support")]
fn forged_request_seal() -> MeterRequestSeal {
    MeterRequestSeal {
        handle: u64::MAX,
        track_id: "forged-request".into(),
        tap: MeterTap::Input,
        reset_generation: 0,
        period_frames: 1,
        peak_hold_frames: 0,
        peak_decay_bits: 0,
        queue_capacity: 1,
    }
}

impl<R> PreparedBuiltinsGraphArtifact<R> {
    /// Immutable caller-owned graph report.
    #[must_use]
    /// The sealed graph, by shared reference.
    ///
    /// Read-only, and deliberately so: #99 F5 stopped `GraphCompileReport` from carrying its own
    /// copy of the plan's vectors, so the callers that used to read them from the report read
    /// them here instead. The seal's compile-fail doctests still hold -- a `&` cannot extract,
    /// clone or mutate the artifact's provenance.
    pub const fn graph(&self) -> &PreparedGraphPlan {
        &self.graph
    }
    pub const fn report(&self) -> &R {
        &self.report
    }

    /// Envelope required by the still-unbound graph.
    #[must_use]
    pub const fn envelope(&self) -> RenderEnvelope {
        self.graph.envelope
    }

    /// Number of sealed production post-input builtin banks retained by this artifact.
    #[must_use]
    pub const fn prepared_builtin_bank_count(&self) -> usize {
        self.graph.prepared_builtin_bank_count()
    }

    /// Address-free backend, width, member and active-mask metadata for qualification.
    pub fn prepared_builtin_banks(&self) -> impl Iterator<Item = GraphPreparedBuiltinBankInfo<'_>> {
        self.graph.builtin_bank_info()
    }

    /// Exact graph-owned storage after retained builtin-bank attachment.
    #[must_use]
    pub const fn graph_resource_estimate(&self) -> &graph::GraphResourceEstimate {
        &self.graph.estimate
    }

    /// Ordinary external nodes required in addition to compiler-owned builtin processors.
    pub fn external_binding_nodes(&self) -> impl Iterator<Item = &GraphNodeId> {
        let builtin_nodes: BTreeSet<_> = self
            .builtin_processors
            .iter()
            .map(|binding| &binding.node)
            .collect();
        let bank_nodes: BTreeSet<_> = self.graph.builtin_bank_members().collect();
        self.graph
            .required_bindings
            .iter()
            .filter(move |node| !builtin_nodes.contains(node) && !bank_nodes.contains(node))
    }

    /// Consume the sealed wrapper and attach its private builtin bindings exactly once.
    #[allow(clippy::result_large_err)]
    pub fn into_bound(
        mut self,
        mut bindings: GraphRuntimeBindings,
    ) -> Result<PreparedBuiltinsGraphBound, PreparedBuiltinsGraphBindFailure<R>> {
        let builtin_nodes: BTreeSet<_> = self
            .builtin_processors
            .iter()
            .map(|binding| binding.node.clone())
            .collect();
        let bank_nodes: BTreeSet<_> = self.graph.builtin_bank_members().collect();
        let expected: BTreeSet<_> = self
            .graph
            .required_bindings
            .iter()
            .filter(|node| !builtin_nodes.contains(*node) && !bank_nodes.contains(*node))
            .cloned()
            .collect();
        let supplied: BTreeSet<_> = bindings
            .nodes
            .iter()
            .map(|binding| binding.node.clone())
            .collect();
        let duplicate_nodes = supplied.len() != bindings.nodes.len();
        let overlaps_builtin = supplied.iter().any(|node| builtin_nodes.contains(node));
        let mut observer_pairs = BTreeSet::new();
        let valid_observers = bindings
            .observers
            .iter()
            .chain(self.builtin_observers.iter())
            .all(|observer| {
                matches!(observer.node, GraphNodeId::TrackStage { .. })
                    && observer_pairs.insert((observer.node.clone(), observer.handle))
            });
        if bindings.envelope != self.graph.envelope
            || duplicate_nodes
            || overlaps_builtin
            || supplied != expected
            || !valid_observers
        {
            let code = if !valid_observers {
                "graph.plan.observer"
            } else if bindings.envelope != self.graph.envelope {
                "graph.plan.envelope_mismatch"
            } else {
                "graph.plan.binding"
            };
            return Err(PreparedBuiltinsGraphBindFailure {
                artifact: self,
                bindings,
                code,
            });
        }
        bindings.nodes.append(&mut self.builtin_processors);
        bindings.observers.append(&mut self.builtin_observers);
        let plan = match self.graph.bind(bindings) {
            Ok(plan) => plan,
            Err(_) => unreachable!("sealed wrapper prevalidated its complete graph bindings"),
        };
        Ok(PreparedBuiltinsGraphBound {
            track_controls: self.track_controls,
            plan,
            meter_consumers: self.meter_consumers,
        })
    }

    /// Consume the sealed wrapper and bind one coordinator-owned source set.
    ///
    /// The wrapper first applies the same builtin-node and observer prevalidation as
    /// [`Self::into_bound`]. It then appends only its genuine private bindings and delegates the
    /// source claims to the graph's transactional source-set bind. Every rejection returns the
    /// opaque artifact, caller bindings, and source set without cloning or exposing sealed parts.
    #[allow(clippy::result_large_err)]
    pub fn into_bound_with_source_set(
        mut self,
        mut bindings: GraphRuntimeBindings,
        source_set: GraphPreparedSourceSet,
    ) -> Result<PreparedBuiltinsGraphBound, PreparedBuiltinsGraphSourceBindFailure<R>> {
        let builtin_nodes: BTreeSet<_> = self
            .builtin_processors
            .iter()
            .map(|binding| binding.node.clone())
            .collect();
        let bank_nodes: BTreeSet<_> = self.graph.builtin_bank_members().collect();
        let expected: BTreeSet<_> = self
            .graph
            .required_bindings
            .iter()
            .filter(|node| !builtin_nodes.contains(*node) && !bank_nodes.contains(*node))
            .cloned()
            .collect();
        let supplied: BTreeSet<_> = bindings
            .nodes
            .iter()
            .map(|binding| binding.node.clone())
            .collect();
        let source_nodes: BTreeSet<_> = source_set
            .claims()
            .iter()
            .map(|claim| claim.node.clone())
            .collect();
        let mut all_supplied = supplied.clone();
        all_supplied.extend(source_nodes);
        let duplicate_nodes = supplied.len() != bindings.nodes.len();
        let overlaps_builtin = supplied.iter().any(|node| builtin_nodes.contains(node));
        let builtin_observer_pairs: BTreeSet<_> = self
            .builtin_observers
            .iter()
            .map(|observer| (observer.node.clone(), observer.handle))
            .collect();
        let mut observer_pairs = BTreeSet::new();
        let valid_observers = bindings
            .observers
            .iter()
            .chain(self.builtin_observers.iter())
            .all(|observer| {
                matches!(observer.node, GraphNodeId::TrackStage { .. })
                    && observer_pairs.insert((observer.node.clone(), observer.handle))
            });
        if bindings.envelope != self.graph.envelope
            || duplicate_nodes
            || overlaps_builtin
            || all_supplied != expected
            || !valid_observers
        {
            let code = if !valid_observers {
                "graph.plan.observer"
            } else if bindings.envelope != self.graph.envelope {
                "graph.plan.envelope_mismatch"
            } else if duplicate_nodes || overlaps_builtin {
                "graph.plan.binding"
            } else if all_supplied != expected {
                "source.graph.binding_mismatch"
            } else {
                "graph.plan.binding"
            };
            return Err(PreparedBuiltinsGraphSourceBindFailure {
                artifact: self,
                bindings,
                source_set,
                code,
            });
        }
        bindings.nodes.append(&mut self.builtin_processors);
        bindings.observers.append(&mut self.builtin_observers);
        match self.graph.bind_with_source_set(bindings, source_set) {
            Ok(plan) => Ok(PreparedBuiltinsGraphBound {
                track_controls: self.track_controls,
                plan,
                meter_consumers: self.meter_consumers,
            }),
            Err(failure) => {
                let mut builtin_processors = Vec::new();
                let mut external_processors = Vec::new();
                for binding in failure.bindings.nodes {
                    if builtin_nodes.contains(&binding.node) {
                        builtin_processors.push(binding);
                    } else {
                        external_processors.push(binding);
                    }
                }
                let mut builtin_observers = Vec::new();
                let mut external_observers = Vec::new();
                for observer in failure.bindings.observers {
                    if builtin_observer_pairs.contains(&(observer.node.clone(), observer.handle)) {
                        builtin_observers.push(observer);
                    } else {
                        external_observers.push(observer);
                    }
                }
                Err(PreparedBuiltinsGraphSourceBindFailure {
                    artifact: PreparedBuiltinsGraphArtifact {
                        graph: *failure.plan,
                        builtin_processors,
                        builtin_observers,
                        report: self.report,
                        track_controls: self.track_controls,
                        meter_consumers: self.meter_consumers,
                    },
                    bindings: GraphRuntimeBindings {
                        envelope: failure.bindings.envelope,
                        nodes: external_processors,
                        observers: external_observers,
                    },
                    source_set: failure.source_set,
                    code: failure.code,
                })
            }
        }
    }
}

fn session_identity(session: &CompiledSession) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(session.canonical_json().as_bytes());
    hash.update(session.sample_rate().0.to_le_bytes());
    hash.update(session.quantum().0.to_le_bytes());
    hash.finalize().into()
}

fn processor_seal(tracks: &[Box<str>]) -> Vec<(Box<str>, TrackStage)> {
    let capacity = tracks
        .len()
        .checked_mul(3)
        .expect("session preparation preflighted processor count");
    let mut values = Vec::with_capacity(capacity);
    for track in tracks {
        for stage in [
            TrackStage::PostInputBuiltins,
            TrackStage::PostFader,
            TrackStage::PostMatrix,
        ] {
            values.push((track.clone(), stage));
        }
    }
    values.sort_unstable();
    values
}

/// Whether a prepared session owns exactly the stages its seal records.
///
/// A prepared session holds all three of a track's stages as one [`StripPreparation`] (issue #212
/// for the fader and the matrix, #210 phase 3 for the input), so ownership is proven over the
/// strips and over whatever bindings a lowering has already produced. The seal itself is unchanged
/// -- three stages per track -- and so is what this refuses: a missing stage, a duplicated one, or
/// a binding on a node that is not a track stage at all.
fn processors_match(
    processors: &[graph::GraphNodeBinding],
    strips: &[StripPreparation],
    expected: &[(Box<str>, TrackStage)],
) -> bool {
    let mut actual: Vec<_> = processors
        .iter()
        .filter_map(|binding| match &binding.node {
            GraphNodeId::TrackStage { track_id, stage } => {
                Some((Box::<str>::from(track_id.as_str()), *stage))
            }
            _ => None,
        })
        .collect();
    let bindings = actual.len();
    for strip in strips {
        actual.push((strip.track_id.clone(), TrackStage::PostInputBuiltins));
        actual.push((strip.track_id.clone(), TrackStage::PostFader));
        actual.push((strip.track_id.clone(), TrackStage::PostMatrix));
    }
    actual.sort();
    bindings == processors.len() && actual == expected
}

fn expected_tails(session: &CompiledSession) -> Result<Vec<(Box<str>, BuiltinTail)>, ()> {
    let mut values: Vec<(Box<str>, BuiltinTail)> =
        Vec::with_capacity(session.normalized_model().tracks.len());
    for track in &session.normalized_model().tracks {
        let parameters = track_parameters(track, u32::MAX).map_err(|_| ())?;
        let chain = BuiltinChain::new(session.sample_rate().0, parameters).map_err(|_| ())?;
        values.push((track.id.as_str().into(), chain.tail()));
    }
    values.sort_unstable_by(|left, right| left.0.cmp(&right.0));
    Ok(values)
}

fn actual_meter_seals(
    observers: &[GraphNodeObserverBinding],
    consumers: &[MeterConsumer],
) -> (Vec<ObserverSeal>, Vec<ConsumerSeal>) {
    let mut observer_values = Vec::with_capacity(observers.len());
    for observer in observers {
        if let GraphNodeId::TrackStage { track_id, stage } = &observer.node {
            observer_values.push((Box::<str>::from(track_id.as_str()), *stage, observer.handle));
        }
    }
    observer_values.sort_unstable();
    let mut consumer_values = Vec::with_capacity(consumers.len());
    for consumer in consumers {
        consumer_values.push((
            consumer.handle.0.get(),
            Box::<str>::from(&*consumer.track_id),
            consumer.tap,
        ));
    }
    consumer_values.sort_unstable();
    (observer_values, consumer_values)
}

pub fn prepare_session_builtins(
    session: &CompiledSession,
    requests: &[MeterRequest],
    caps: BuiltinCompileCaps,
) -> Result<PreparedBuiltinsSession, BuiltinDiagnosticSet> {
    prepare_session_builtins_with_console(session, requests, &[], caps)
}

/// Prepare builtins with live-console control channels attached (issue #137 D1).
///
/// `controls` requests one bounded control channel per named track; the consumer half is bound
/// into that track's matrix processor and the producer half is returned in the prepared session
/// for the host to drive off render. A track named twice, or a track the session does not
/// declare, is a preparation diagnostic -- never a silently ignored request.
///
/// [`prepare_session_builtins`] is exactly this call with no control channels, so a host that does
/// not want a console pays nothing: no queue is allocated and the matrix processors carry `None`.
pub fn prepare_session_builtins_with_console(
    session: &CompiledSession,
    requests: &[MeterRequest],
    controls: &[TrackControlRequest],
    caps: BuiltinCompileCaps,
) -> Result<PreparedBuiltinsSession, BuiltinDiagnosticSet> {
    let mut diagnostics = Vec::new();
    if [
        caps.maximum_total_state_bytes,
        caps.maximum_total_retained_payload_bytes,
        caps.maximum_total_meter_items,
        caps.maximum_total_meter_bytes,
        caps.maximum_single_allocation_bytes,
        caps.maximum_meter_streams,
    ]
    .into_iter()
    .any(|value| value == 0)
    {
        diagnostics.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
    }
    match u64::try_from(requests.len()) {
        Ok(count) if count > caps.maximum_meter_streams => {
            diagnostics.push(diag("builtin.resource.limit", "$.meter_requests"));
        }
        Err(_) => diagnostics.push(diag(
            "builtin.resource.arithmetic_overflow",
            "$.meter_requests",
        )),
        Ok(_) => {}
    }
    let mut request_keys = BTreeSet::new();
    let mut request_handles = BTreeSet::new();
    for request in requests {
        if !request_handles.insert(request.handle) {
            diagnostics.push(diag("builtin.meter.duplicate_handle", &meter_path(request)));
        }
        let key = (request.track_id.clone(), request.tap);
        if !request_keys.insert(key) {
            diagnostics.push(diag("builtin.meter.duplicate", &meter_path(request)));
        }
        if request.config.period_frames.get() > caps.maximum_period_frames
            || request.config.peak_hold_frames > caps.maximum_peak_hold_frames
        {
            diagnostics.push(diag("builtin.resource.limit", &meter_path(request)));
        }
        if !request.config.peak_decay_db_per_second.is_finite()
            || !(0.0..=120.0).contains(&request.config.peak_decay_db_per_second)
        {
            diagnostics.push(diag("builtin.meter.config", &meter_path(request)));
        }
    }
    let known_tracks: BTreeSet<_> = session
        .normalized_model()
        .tracks
        .iter()
        .map(|track| track.id.as_str())
        .collect();
    for request in requests {
        if !known_tracks.contains(request.track_id.as_str()) {
            diagnostics.push(diag("builtin.meter.unknown_track", &meter_path(request)));
        }
    }
    let mut control_tracks = BTreeSet::new();
    for control in controls {
        if !control_tracks.insert(control.track_id.as_str()) {
            diagnostics.push(diag("builtin.control.duplicate", &control_path(control)));
        }
        if !known_tracks.contains(control.track_id.as_str()) {
            diagnostics.push(diag(
                "builtin.control.unknown_track",
                &control_path(control),
            ));
        }
    }
    for track in &session.normalized_model().tracks {
        match track_parameters(track, caps.maximum_smoothing_samples)
            .and_then(|parameters| BuiltinChain::new(session.sample_rate().0, parameters))
        {
            Ok(_) => {}
            Err(error) => {
                diagnostics.push(parameter_diagnostic(track, error, session.sample_rate().0))
            }
        }
    }
    let resource_plan = match resource_plan(session, requests, controls) {
        Ok(value) => Some(value),
        Err(error) => {
            diagnostics.push(error);
            None
        }
    };
    if let Some(plan) = resource_plan {
        let report = plan.report;
        if report.engine_owned_processor_payload_bytes > caps.maximum_total_state_bytes
            || report.engine_owned_retained_payload_bytes
                > caps.maximum_total_retained_payload_bytes
            || report.meter_items > caps.maximum_total_meter_items
            || report.engine_owned_meter_payload_bytes > caps.maximum_total_meter_bytes
            || report.maximum_single_allocation_bytes > caps.maximum_single_allocation_bytes
        {
            diagnostics.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
        }
    }
    if !diagnostics.is_empty() {
        return Err(BuiltinDiagnosticSet::sorted(diagnostics));
    }
    let resources = resource_plan.expect("validated resource plan").report;
    #[cfg(feature = "test-support")]
    let _phase_two_tracker = TestPhaseTwoAllocationGuard::begin();
    let track_count = session.normalized_model().tracks.len();
    // Empty, and it stays empty: every track stage is held as a `StripPreparation` until lowering
    // (#212 for the fader and the matrix, #210 phase 3 for the input). The vector exists so that
    // `into_graph_artifact` has somewhere to put the bindings it builds, and so that
    // `processors_match` keeps refusing a binding on a node that is not a track stage.
    let mut processors: Vec<graph::GraphNodeBinding> = Vec::new();
    let _ = &mut processors;
    let mut strips = Vec::with_capacity(track_count);
    let mut bank_inputs = Vec::with_capacity(track_count);
    let mut tails = Vec::with_capacity(track_count);
    let mut control_capacity: BTreeMap<&str, NonZeroUsize> = BTreeMap::new();
    for control in controls {
        control_capacity.insert(control.track_id.as_str(), control.queue_capacity);
    }
    let mut track_controls = Vec::with_capacity(controls.len());
    let mut control_seal: Vec<(Box<str>, usize)> = Vec::with_capacity(controls.len());
    for track in &session.normalized_model().tracks {
        let parameters = track_parameters(track, caps.maximum_smoothing_samples)
            .expect("preflighted parameters");
        let chain = BuiltinChain::new(session.sample_rate().0, parameters)
            .expect("preflighted coefficients");
        let tail = chain.tail();
        let (input, fader, matrix) = chain.into_sections();
        // The bank candidate is prepared independently from the scalar fallback.  The selected
        // full-bank artifact consumes this copy and removes the corresponding scalar binding;
        // the scalar input remains the transactional fallback until that point.
        let bank_input = BuiltinChain::new(session.sample_rate().0, parameters)
            .expect("preflighted bank coefficients")
            .into_input_builtins();
        tails.push((Box::<str>::from(track.id.as_str()), tail));
        bank_inputs.push((Box::<str>::from(track.id.as_str()), bank_input));
        let graph_id = StableGraphId::parse(track.id.as_str()).expect("preflighted stable ID");
        let control_failure = || {
            BuiltinDiagnosticSet::sorted(vec![diag(
                "builtin.control.prepare",
                &format!("$.controls[track_id={}]", track.id.as_str()),
            )])
        };
        let control = match control_capacity.get(track.id.as_str()) {
            None => None,
            Some(capacity) => {
                let (producer, control) =
                    bounded_spsc::<TrackControlRecord>(*capacity, QueueGeneration(0))
                        .map_err(|_| control_failure())?;
                let (fader_producer, fader_control) =
                    bounded_spsc::<TrackFaderRecord>(*capacity, QueueGeneration(0))
                        .map_err(|_| control_failure())?;
                let (input_producer, input_control) =
                    bounded_spsc::<TrackInputRecord>(*capacity, QueueGeneration(0))
                        .map_err(|_| control_failure())?;
                // The ramped fader is validated here rather than at binding, so a declared
                // `fader_db` outside the live domain is a preparation diagnostic on the track that
                // carries it -- the same failure, at the same place, as before the strip banked.
                FaderMuteRampBuiltins::new(parameters).map_err(|_| {
                    BuiltinDiagnosticSet::sorted(vec![parameter_diagnostic(
                        track,
                        BuiltinParameterError::GainDomain,
                        session.sample_rate().0,
                    )])
                })?;
                track_controls.push(TrackControlProducer {
                    track_id: Box::<str>::from(track.id.as_str()),
                    producer,
                    fader: fader_producer,
                    input: input_producer,
                });
                control_seal.push((Box::<str>::from(track.id.as_str()), capacity.get()));
                Some(StripControlConsumers {
                    input: Some(input_control),
                    fader: Some(fader_control),
                    matrix: Some(control),
                })
            }
        };
        strips.push(StripPreparation {
            track_id: Box::<str>::from(track.id.as_str()),
            graph_id,
            parameters,
            input,
            fader,
            matrix,
            control,
        });
    }
    control_seal.sort_unstable();
    let mut observers = Vec::with_capacity(requests.len());
    let mut meter_consumers = Vec::with_capacity(requests.len());
    let mut request_seals = Vec::with_capacity(requests.len());
    for request in requests {
        let handle = request.handle;
        let PreparedMeter {
            accumulator,
            consumer,
        } = MeterAccumulator::prepare(handle, request.config, session.sample_rate().0).map_err(
            |error| BuiltinDiagnosticSet::sorted(vec![meter_diagnostic(request, error)]),
        )?;
        let graph_id = StableGraphId::parse(&request.track_id).expect("known accepted session ID");
        observers.push(GraphNodeObserverBinding::new(
            stage_node(graph_id, stage(request.tap)),
            handle.0.get(),
            Box::new(MeterObserver(accumulator)),
        ));
        meter_consumers.push(MeterConsumer {
            handle,
            track_id: request.track_id.as_str().into(),
            tap: request.tap,
            consumer,
        });
        request_seals.push(MeterRequestSeal {
            handle: handle.0.get(),
            track_id: request.track_id.as_str().into(),
            tap: request.tap,
            reset_generation: request.config.reset_generation,
            period_frames: request.config.period_frames.get(),
            peak_hold_frames: request.config.peak_hold_frames,
            peak_decay_bits: request.config.peak_decay_db_per_second.to_bits(),
            queue_capacity: request.config.queue_capacity.get(),
        });
    }
    tails.sort_unstable_by(|left, right| left.0.cmp(&right.0));
    request_seals.sort_unstable();
    let tracks: Vec<Box<str>> = session
        .normalized_model()
        .tracks
        .iter()
        .map(|track| track.id.as_str().into())
        .collect();
    let processor_seal = processor_seal(&tracks);
    let (observer_seal, consumer_seal) = actual_meter_seals(&observers, &meter_consumers);
    Ok(PreparedBuiltinsSession {
        seal: BuiltinSessionSeal {
            session_sha256: session_identity(session),
            sample_rate: session.sample_rate().0,
            quantum: session.quantum().0,
            tracks,
            processors: processor_seal,
            tails: tails.clone(),
            requests: request_seals.clone(),
            observers: observer_seal,
            consumers: consumer_seal,
            controls: control_seal,
            resources,
        },
        processors,
        strips,
        bank_inputs,
        observers,
        meter_consumers,
        track_controls,
        tails,
        requests: request_seals,
        resources,
    })
}

fn control_path(request: &TrackControlRequest) -> String {
    format!("$.controls[track_id={}]", request.track_id)
}

fn resource_plan(
    session: &CompiledSession,
    requests: &[MeterRequest],
    controls: &[TrackControlRequest],
) -> Result<BuiltinResourcePlan, BuiltinDiagnostic> {
    let track_count = session.normalized_model().tracks.len();
    // The seal still records three stages per track even though preparation binds one and defers
    // two, so the seal vector below is charged at the seal's own count and not at the bindings'.
    let sealed_stage_count = track_count
        .checked_mul(3)
        .ok_or_else(|| diag("builtin.resource.arithmetic_overflow", "$.tracks"))?;
    let request_count = requests.len();
    let mut processor = ResourceAccumulator::default();
    let mut meter = ResourceAccumulator::default();
    // **No** bindings per track: all three stages are held as `StripPreparation` until their
    // binding form is chosen at lowering (#212 for the fader and the matrix, #210 phase 3 for the
    // input), so preparation allocates the strip vector and no boxed processors at all. The
    // binding vector is empty at preparation and charges nothing; the vector it will be at
    // lowering is charged where lowering's own storage is, which is the graph's estimate and not
    // this one.
    add_vector_layout::<StripPreparation>(&mut processor, track_count)?;
    add_vector_layout::<(Box<str>, InputBuiltins)>(&mut processor, track_count)?;
    add_vector_layout::<(Box<str>, BuiltinTail)>(&mut processor, track_count)?;
    add_vector_layout::<Box<str>>(&mut processor, track_count)?;
    add_vector_layout::<(Box<str>, TrackStage)>(&mut processor, sealed_stage_count)?;
    add_vector_layout::<(Box<str>, BuiltinTail)>(&mut processor, track_count)?;
    for track in &session.normalized_model().tracks {
        let bytes = track.id.as_str().len();
        // Nine independently retained copies of the track's ID: the strip's own ID and its graph
        // ID, the retained tail, the compact track seal, the three processor-seal rows, the seal's
        // cloned tail ID, and the independently retained bank-input candidate ID. The tenth was
        // the post-input binding's node ID, and it is gone: #210 phase 3 defers that binding to
        // lowering with the other two.
        for _ in 0..9 {
            processor
                .add_bytes(bytes)
                .ok_or_else(|| diag("builtin.resource.arithmetic_overflow", "$.tracks"))?;
        }
        // Nothing is boxed here any more. Issue #140 B's rule -- a controlled track holds the
        // ramped section, an uncontrolled one the prepared section -- still holds, and all three
        // sections now live *inline* in the strip vector charged above rather than behind a `Box`,
        // so they are charged by that vector's layout and not per track. `controlled` therefore
        // changes no byte of the *section* storage, and the only console-dependent rows left are
        // the three bounded rings charged below.
    }
    add_vector_layout::<GraphNodeObserverBinding>(&mut meter, request_count)?;
    add_vector_layout::<MeterConsumer>(&mut meter, request_count)?;
    add_vector_layout::<MeterRequestSeal>(&mut meter, request_count)?;
    add_vector_layout::<MeterRequestSeal>(&mut meter, request_count)?;
    add_vector_layout::<(Box<str>, TrackStage, u64)>(&mut meter, request_count)?;
    add_vector_layout::<(u64, Box<str>, MeterTap)>(&mut meter, request_count)?;
    // Issue #137 D1: the live-console control channels are charged to the processor accumulator,
    // because they are per-track processor storage rather than meter storage: the producer vector,
    // its seal, and one bounded ring per requested track.
    add_vector_layout::<TrackControlProducer>(&mut processor, controls.len())?;
    add_vector_layout::<(Box<str>, usize)>(&mut processor, controls.len())?;
    for control in controls {
        // Three bounded rings per controlled track at the same depth: #137 D1's matrix channel,
        // #140 B's fader/mute channel and #210 phase 3's input trim/polarity channel. All three
        // are charged here, in the same accumulator, for the same reason -- they are per-track
        // processor storage, not meter storage.
        let matrix_queue = bounded_spsc_retained_payload::<TrackControlRecord>(
            control.queue_capacity,
        )
        .map_err(|_| {
            diag(
                "builtin.resource.arithmetic_overflow",
                &control_path(control),
            )
        })?;
        let fader_queue = bounded_spsc_retained_payload::<TrackFaderRecord>(control.queue_capacity)
            .map_err(|_| {
                diag(
                    "builtin.resource.arithmetic_overflow",
                    &control_path(control),
                )
            })?;
        let input_queue = bounded_spsc_retained_payload::<TrackInputRecord>(control.queue_capacity)
            .map_err(|_| {
                diag(
                    "builtin.resource.arithmetic_overflow",
                    &control_path(control),
                )
            })?;
        for queue in [matrix_queue, fader_queue, input_queue] {
            processor
                .add_layout(
                    core::alloc::Layout::from_size_align(
                        queue.ring_header_bytes,
                        queue.ring_header_align,
                    )
                    .map_err(|_| {
                        diag(
                            "builtin.resource.arithmetic_overflow",
                            &control_path(control),
                        )
                    })?,
                )
                .and_then(|_| {
                    core::alloc::Layout::from_size_align(
                        queue.slot_payload_bytes,
                        queue.slot_payload_align,
                    )
                    .ok()
                    .and_then(|layout| processor.add_layout(layout))
                })
                .ok_or_else(|| {
                    diag(
                        "builtin.resource.arithmetic_overflow",
                        &control_path(control),
                    )
                })?;
        }
        // The public producer identity and the sealed control identity are retained separately.
        for _ in 0..2 {
            processor.add_bytes(control.track_id.len()).ok_or_else(|| {
                diag(
                    "builtin.resource.arithmetic_overflow",
                    &control_path(control),
                )
            })?;
        }
    }
    let mut meter_items = 0_u64;
    for request in requests {
        let queue =
            bounded_spsc_retained_payload::<MeterSnapshot>(request.config.queue_capacity)
                .map_err(|_| diag("builtin.resource.arithmetic_overflow", &meter_path(request)))?;
        meter_items = meter_items
            .checked_add(
                u64::try_from(queue.slot_count).map_err(|_| {
                    diag("builtin.resource.arithmetic_overflow", &meter_path(request))
                })?,
            )
            .ok_or_else(|| diag("builtin.resource.arithmetic_overflow", &meter_path(request)))?;
        meter
            .add_layout(
                core::alloc::Layout::from_size_align(
                    queue.ring_header_bytes,
                    queue.ring_header_align,
                )
                .map_err(|_| diag("builtin.resource.arithmetic_overflow", &meter_path(request)))?,
            )
            .and_then(|_| {
                core::alloc::Layout::from_size_align(
                    queue.slot_payload_bytes,
                    queue.slot_payload_align,
                )
                .ok()
                .and_then(|layout| meter.add_layout(layout))
            })
            .and_then(|_| meter.add_layout(core::alloc::Layout::new::<MeterObserver>()))
            .ok_or_else(|| diag("builtin.resource.arithmetic_overflow", &meter_path(request)))?;
        let bytes = request.track_id.len();
        // Observer graph ID, public consumer ID, retained request, and three seal identities.
        for _ in 0..6 {
            meter.add_bytes(bytes).ok_or_else(|| {
                diag("builtin.resource.arithmetic_overflow", &meter_path(request))
            })?;
        }
    }
    let total = processor.total.checked_add(meter.total).ok_or_else(|| {
        diag(
            "builtin.resource.arithmetic_overflow",
            "$.builtin_compile_caps",
        )
    })?;
    let allocations = processor
        .allocations
        .checked_add(meter.allocations)
        .ok_or_else(|| {
            diag(
                "builtin.resource.arithmetic_overflow",
                "$.builtin_compile_caps",
            )
        })?;
    let mut retained = processor;
    retained.merge(meter).ok_or_else(|| {
        diag(
            "builtin.resource.arithmetic_overflow",
            "$.builtin_compile_caps",
        )
    })?;
    let retained = retained.sorted_layouts();
    Ok(BuiltinResourcePlan {
        report: BuiltinResourceEstimate {
            engine_owned_processor_payload_bytes: processor.total,
            engine_owned_meter_payload_bytes: meter.total,
            engine_owned_retained_payload_bytes: total,
            meter_items,
            maximum_single_allocation_bytes: processor.largest.max(meter.largest),
            retained_allocation_count: allocations,
            retained_layout_class_count: retained.layout_class_count,
            retained_layouts: retained.layouts,
        },
    })
}

fn add_vector_layout<T>(
    accumulator: &mut ResourceAccumulator,
    items: usize,
) -> Result<(), BuiltinDiagnostic> {
    if items == 0 {
        return Ok(());
    }
    let layout = core::alloc::Layout::array::<T>(items).map_err(|_| {
        diag(
            "builtin.resource.arithmetic_overflow",
            "$.builtin_compile_caps",
        )
    })?;
    accumulator.add_layout(layout).ok_or_else(|| {
        diag(
            "builtin.resource.arithmetic_overflow",
            "$.builtin_compile_caps",
        )
    })
}

/// The `SOURCE` term of one track's channel-symmetry witness.
///
/// True exactly when the track's two dual-mono lanes read the **same** source channel, so the two
/// planes are filled with identical samples. That is the structural precondition of a mono
/// collapse: a collapsed track computes one plane, so the two planes have to have been fed
/// identically in the first place.
///
/// # Why one condition and not two
///
/// The mono-collapse design states the predicate as "`left_source_channel ==
/// right_source_channel`, **or** a one-channel source". The second is not a second case: session
/// validation refuses any channel index that is not below the source's `channel_count`
/// (`SourceChannelIndexOutOfRange`, `$.tracks[i].left_source_channel`), so a one-channel source can
/// only ever be mapped as `0, 0` -- which the first condition already admits. A separate branch
/// for it would be code no validated session can reach, and the honest form of an unreachable
/// branch is not writing it.
///
/// `session` is taken so this stays the one predicate both bank planners call even after the rule
/// grows a term the track alone cannot answer.
#[must_use]
pub fn track_mono_source(session: &CompiledSession, track: &Track) -> bool {
    let _ = session;
    track.left_source_channel == track.right_source_channel
}

/// The `DESIGNED` term a track's **input-side delay** contributes (#210 phase 2).
///
/// True exactly when the two lanes declare the same `delay_samples`. A collapsed track computes one
/// plane and duplicates it at the seam; two lanes with different delays turn one source channel
/// into two genuinely different signals upstream of that seam, so duplicating either one is wrong
/// audio, not a saved multiply.
///
/// # Why this term is decided here and not by the input stage's word list
///
/// `InputStage::lane_channel_symmetry` compares the words its **kernel** reads -- `trim_signed` and
/// the two SVF sections' six coefficients each. `delay_samples` is not one of them: the delay is
/// applied by a graph node at `TrackStage::Input`, upstream of the bank, and the bank never sees
/// it. Adding it to that list would be claiming the kernel reads a word it does not read. It is a
/// prepared-only session word, so the honest owner of the verdict is the prepare-time structural
/// witness -- here -- which is also the only one that runs before the chain is armed at all.
/// Named without a `_v1` suffix on purpose, ahead of its two neighbours: issue #215's owner ruling
/// is that pre-launch internal implementation names are born unversioned, and that issue has since
/// taken the suffix off the neighbours too.
#[must_use]
pub fn track_input_delay_symmetric(track: &Track) -> bool {
    track.builtins.left.delay_samples == track.builtins.right.delay_samples
}

/// Every track's structural channel-symmetry witness, in normalized track order.
///
/// # Why this is a function over the compiled session and not a field of the prepared plan
///
/// The structural term is what the **planner** needs: the mono-collapse design pools cohorts by
/// class -- "mono-symmetric at prepare" against "stereo" -- and that decision is taken while the
/// plan is being built, from the compiled session, before any prepared object exists. Both bank
/// planners must derive the class from the same predicate or the lane-alignment check declines
/// the chain merges silently, so the predicate is written once, here, and called rather than
/// copied.
///
/// The prepared side is deliberately source-agnostic: the SOURCE term lives only in this
/// bit into each track's input stage, so the runtime witness carries it too. This function is what
/// a caller with a `CompiledSession` and no plan asks.
#[must_use]
pub fn session_structural_symmetry(
    session: &CompiledSession,
) -> Vec<(Box<str>, ChannelSymmetryWitness)> {
    session
        .normalized_model()
        .tracks
        .iter()
        .map(|track| {
            let mut witness = ChannelSymmetryWitness::SYMMETRIC;
            witness.set(
                ChannelSymmetryWitness::SOURCE,
                track_mono_source(session, track),
            );
            // Issue #210 phase 2. Prepared-only state, so the whole verdict is available here and
            // needs no per-block maintenance: an asymmetric delay declines this track's collapse
            // for the life of the plan, and a symmetric one -- including the overwhelmingly common
            // zero -- leaves the witness exactly as it was before the feature existed.
            witness.set(
                ChannelSymmetryWitness::DESIGNED,
                track_input_delay_symmetric(track),
            );
            (Box::<str>::from(track.id.as_str()), witness)
        })
        .collect()
}

/// Every track's pool class for one compile, derived **once** and handed to both bank planners.
///
/// # Why this exists as an object rather than as a predicate each planner calls
///
/// The obligation stated on [`CohortPoolClass::of_prepare_witness`] is that the builtin-stage
/// planner and the rack-chain planner must classify every track *identically*: issue #208's chain
/// merges are proved lane by lane on the lowered program, and two planners whose pools hold
/// different track sets produce banks whose lane sets slide out of step, so every
/// `builtins -> EQ -> compressor -> limiter` merge declines **silently** -- the same rendered
/// bits, no diagnostic, one planar/AoSoA round-trip per stage where the strip used to pay one per
/// cohort.
///
/// A predicate each planner calls would make that agreement a property of two call sites staying
/// in step. This makes it a property of there being one value: `GraphCompiler` derives this once
/// per compile, `bind_rack_banks` reads it, and `into_graph_artifact_with_banks` reads the same
/// object. `the_two_planners_agree_on_every_track_class` is the gate on the *observable*
/// consequence -- the two planners' lane sets line up track for track -- because that, and not
/// the map, is what the merge actually needs.
///
/// # What a track's class is derived from
///
/// The conjunction of every prepare-time witness that speaks for the track, then
/// [`CohortPoolClass::of_prepare_witness`]:
///
/// * `SOURCE` from [`track_mono_source`], through [`session_structural_symmetry`]. This is
///   the term the M0 phase built and the only one the compiled session can answer alone.
/// * `DESIGNED` from each prepared upstream-of-seam stage the compile actually prepared: the
///   track's input builtins ([`InputBuiltins::channel_symmetry`]) and each of its prepared native
///   effects. A compile that prepared no builtins (`GraphCompiler::compile`) simply has one fewer
///   contributor -- honest, because there is no input stage in that plan to be asymmetric.
///
/// Absent terms are never assumed: an unknown track answers [`CohortPoolClass::Stereo`], which
/// is the class that cannot over-claim.
#[derive(Clone, Debug, Default)]
pub struct SessionPoolClasses {
    by_track: BTreeMap<Box<str>, ChannelSymmetryWitness>,
}

impl SessionPoolClasses {
    /// Seeds every track's witness with its structural (`SOURCE`) term.
    #[must_use]
    pub fn from_session(session: &CompiledSession) -> Self {
        Self {
            by_track: session_structural_symmetry(session).into_iter().collect(),
        }
    }

    /// Conjoins one prepared stage's witness into its track's.
    ///
    /// Conjunction, never assignment: a track has several upstream stages and a single declining
    /// one declines the track, which is the same rule `ChannelSymmetryWitness::and` states for
    /// every other composition of these witnesses. A stage naming a track the session does not
    /// have is ignored rather than inserted, so a mismatched caller cannot invent a class.
    pub fn conjoin(&mut self, track_id: &str, witness: ChannelSymmetryWitness) {
        if let Some(existing) = self.by_track.get_mut(track_id) {
            *existing = existing.and(witness);
        }
    }

    /// This track's pool class. An unknown track is [`CohortPoolClass::Stereo`].
    #[must_use]
    pub fn class_of(&self, track_id: &str) -> CohortPoolClass {
        self.by_track
            .get(track_id)
            .copied()
            .map_or(CohortPoolClass::Stereo, |witness| {
                CohortPoolClass::of_prepare_witness(witness)
            })
    }

    /// Every track's class, in normalized track order. Evidence and diagnosis only.
    pub fn classes(&self) -> impl Iterator<Item = (&str, CohortPoolClass)> {
        self.by_track.iter().map(|(track, witness)| {
            (
                track.as_ref(),
                CohortPoolClass::of_prepare_witness(*witness),
            )
        })
    }

    /// How many tracks fall in [`CohortPoolClass::MonoSymmetricAtPrepare`].
    #[must_use]
    pub fn mono_track_count(&self) -> usize {
        self.classes()
            .filter(|(_, class)| *class == CohortPoolClass::MonoSymmetricAtPrepare)
            .count()
    }
}

struct InputProcessor(InputBuiltins);
impl GraphRuntimeProcessor for InputProcessor {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        let block = DualMonoBlock::new(block.left, block.right, block.first_sample)
            .map_err(render_error)?;
        self.0.process(block);
        Ok(())
    }
    fn channel_symmetry(&self) -> ChannelSymmetryWitness {
        self.0.channel_symmetry()
    }
}
struct FaderProcessor(FaderMuteBuiltins);
impl GraphRuntimeProcessor for FaderProcessor {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        let block = DualMonoBlock::new(block.left, block.right, block.first_sample)
            .map_err(render_error)?;
        self.0.process(block);
        Ok(())
    }
    fn channel_symmetry(&self) -> ChannelSymmetryWitness {
        SEAM_SIDE_WITNESS
    }
}
struct MatrixProcessor(MatrixBuiltins);
impl GraphRuntimeProcessor for MatrixProcessor {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        let block = DualMonoBlock::new(block.left, block.right, block.first_sample)
            .map_err(render_error)?;
        self.0.process(block);
        Ok(())
    }
    fn channel_symmetry(&self) -> ChannelSymmetryWitness {
        SEAM_SIDE_WITNESS
    }
}

/// The input trim/polarity stage of one track that a live console drives (#210 phase 3).
///
/// The per-node sibling of [`BuiltinBankProcessor`]'s drain, for the tracks a strip bank did not
/// claim -- a host on a scalar backend, or a plan whose planner emitted no input bank. It is a
/// separate type from [`InputProcessor`] for the reason [`ConsoleMatrixProcessor`] gives: a
/// session prepared without a console keeps `InputProcessor` and therefore keeps its exact
/// processor storage and its exact rendered bits.
///
/// # The witness, and why this one has to carry it
///
/// The input section is upstream of the fader/matrix seam, so an admitted per-lane record clears
/// the `LIVE` term. A per-node processor is not a bank slot and the collapse dispatch never asks
/// it anything -- `GraphRuntimeProcessor::channel_symmetry` is the scalar-tail surface, read by
/// the prepare-time pool classification -- but the term is folded anyway, by the same
/// `admit` call the banked drain makes, so the two shapes cannot disagree about what a record
/// meant. If a later phase gives the scalar tail a collapse, the answer is already correct here.
struct ConsoleInputProcessor {
    input: InputBuiltins,
    control: Consumer<TrackInputRecord>,
    /// This track's live channel-symmetry terms, retained across blocks exactly as
    /// `EffectControlLane::symmetry` is and for the same reason.
    live: ChannelSymmetryWitness,
}
impl GraphRuntimeProcessor for ConsoleInputProcessor {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        while let Ok(record) = self.control.try_pop() {
            self.live.admit(&record);
            match record {
                TrackInputRecord::TrimDb {
                    lanes,
                    db,
                    smoothing_samples,
                } => self
                    .input
                    .set_trim_db(lanes, db, smoothing_samples)
                    .map_err(render_error)?,
                TrackInputRecord::PolarityInvert {
                    lanes,
                    inverted,
                    smoothing_samples,
                } => self
                    .input
                    .set_polarity_invert(lanes, inverted, smoothing_samples),
            }
        }
        let block = DualMonoBlock::new(block.left, block.right, block.first_sample)
            .map_err(render_error)?;
        self.input.process(block);
        Ok(())
    }
    fn channel_symmetry(&self) -> ChannelSymmetryWitness {
        self.input.channel_symmetry().and(self.live)
    }
}

/// The matrix/pan stage of one track that a live console drives (issue #137 D1).
///
/// It is a separate type from [`MatrixProcessor`] on purpose: a session prepared without a console
/// keeps the exact processor storage, and therefore the exact `engine_owned_processor_payload_bytes`
/// row, it had before this channel existed. "Metering and control off cost nothing" is a byte
/// identity here, not a figure of speech.
///
/// The drain runs at the top of the block, before any audio is touched, so an admitted retarget
/// takes effect at exactly the block boundary the control side was told it would: every sample of
/// the block starting at `block.first_sample` is rendered by the post-command ramp. `try_pop`
/// moves one `Copy` record and `set_target_smoothed` performs four divisions; neither allocates,
/// locks, nor drops, which is what keeps the shipped artifact's render call-graph gate green.
struct ConsoleMatrixProcessor {
    matrix: MatrixBuiltins,
    control: Consumer<TrackControlRecord>,
}
impl GraphRuntimeProcessor for ConsoleMatrixProcessor {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        while let Ok(record) = self.control.try_pop() {
            self.matrix
                .set_target_smoothed(record.matrix, record.smoothing_samples)
                .map_err(render_error)?;
        }
        let block = DualMonoBlock::new(block.left, block.right, block.first_sample)
            .map_err(render_error)?;
        self.matrix.process(block);
        Ok(())
    }
}
/// The fader/mute stage of one track that a live console drives (issue #140 B).
///
/// The same shape as [`ConsoleMatrixProcessor`], for the same reason: a session prepared without
/// a console keeps `FaderProcessor` and therefore keeps its exact processor storage and its exact
/// rendered bits. The drain runs at the top of the block, before any audio is touched, so an
/// admitted fader move or mute takes effect at exactly the block boundary the control side was
/// acknowledged with.
struct ConsoleFaderProcessor {
    fader: FaderMuteRampBuiltins,
    control: Consumer<TrackFaderRecord>,
}
impl GraphRuntimeProcessor for ConsoleFaderProcessor {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        while let Ok(record) = self.control.try_pop() {
            match record {
                TrackFaderRecord::FaderDb {
                    lanes,
                    db,
                    smoothing_samples,
                } => self
                    .fader
                    .set_fader_db(lanes, db, smoothing_samples)
                    .map_err(render_error)?,
                TrackFaderRecord::Mute {
                    lanes,
                    muted,
                    smoothing_samples,
                } => self.fader.set_mute(lanes, muted, smoothing_samples),
            }
        }
        let block = DualMonoBlock::new(block.left, block.right, block.first_sample)
            .map_err(render_error)?;
        self.fader.process(block);
        Ok(())
    }
}
struct MeterObserver(MeterAccumulator);
impl GraphRuntimeObserver for MeterObserver {
    fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
        self.0
            .observe(block.left, block.right, block.first_sample)
            .map_err(|error| match error {
                builtins::MeterObservationError::SampleTimeOverflow => RenderError::TimeOverflow,
                builtins::MeterObservationError::LaneLength => RenderError::InvalidEnvelope,
            })
    }
}

fn render_error(error: BuiltinParameterError) -> RenderError {
    match error {
        BuiltinParameterError::SampleTimeOverflow => RenderError::TimeOverflow,
        _ => RenderError::InvalidEnvelope,
    }
}

fn track_parameters(
    track: &Track,
    maximum_smoothing: u32,
) -> Result<BuiltinParameters, BuiltinParameterError> {
    let left = ChannelParameters {
        polarity_invert: track.builtins.left.polarity_invert,
        trim_db: track.builtins.left.trim_db,
        hpf_hz: track.builtins.left.hpf_hz,
        lpf_hz: track.builtins.left.lpf_hz,
        fader_db: track.fader.left_db,
        muted: track.fader.left_mute,
    };
    let right = ChannelParameters {
        polarity_invert: track.builtins.right.polarity_invert,
        trim_db: track.builtins.right.trim_db,
        hpf_hz: track.builtins.right.hpf_hz,
        lpf_hz: track.builtins.right.lpf_hz,
        fader_db: track.fader.right_db,
        muted: track.fader.right_mute,
    };
    let (matrix, smoothing_samples) = match track.matrix_or_pan {
        MatrixOrPan::Pan {
            left,
            right,
            smoothing_samples,
        } => (pan_matrix(left, right)?, smoothing_samples),
        MatrixOrPan::Matrix {
            ll,
            lr,
            rl,
            rr,
            smoothing_samples,
        } => (Matrix2x2 { ll, lr, rl, rr }.checked()?, smoothing_samples),
    };
    if smoothing_samples > maximum_smoothing {
        return Err(BuiltinParameterError::MatrixSmoothing);
    }
    Ok(BuiltinParameters {
        left,
        right,
        matrix,
        smoothing_samples,
    })
}

fn stage(tap: MeterTap) -> TrackStage {
    match tap {
        MeterTap::Input => TrackStage::Input,
        MeterTap::PostInputBuiltins => TrackStage::PostInputBuiltins,
        MeterTap::PostSimd1 => TrackStage::PostSimd1,
        MeterTap::PostDynamic => TrackStage::PostDynamic,
        MeterTap::PostSimd2PreFader => TrackStage::PostSimd2PreFader,
        MeterTap::PostFader => TrackStage::PostFader,
        MeterTap::PostMatrix => TrackStage::PostMatrix,
    }
}
fn stage_node(track_id: StableGraphId, stage: TrackStage) -> GraphNodeId {
    GraphNodeId::TrackStage { track_id, stage }
}
fn diag(code: &'static str, path: &str) -> BuiltinDiagnostic {
    BuiltinDiagnostic {
        code,
        path: path.to_owned(),
    }
}
fn meter_path(request: &MeterRequest) -> String {
    format!(
        "$.meters[track_id={},tap={:?}]",
        request.track_id, request.tap
    )
}
fn parameter_diagnostic(
    track: &Track,
    error: BuiltinParameterError,
    sample_rate: u32,
) -> BuiltinDiagnostic {
    let code = match error {
        BuiltinParameterError::GainDomain => "builtin.gain.domain",
        BuiltinParameterError::FilterCutoff => "builtin.filter.cutoff",
        BuiltinParameterError::FilterOrder => "builtin.filter.order",
        BuiltinParameterError::FilterCoefficients => "builtin.filter.coefficients",
        BuiltinParameterError::MatrixCoefficient => "builtin.matrix.coefficient",
        BuiltinParameterError::MatrixSmoothing => "builtin.matrix.smoothing",
        _ => "builtin.resource.arithmetic_overflow",
    };
    let track_path = format!("$.tracks[id={}]", track.id);
    let path = match error {
        BuiltinParameterError::GainDomain => gain_path(track, &track_path),
        BuiltinParameterError::FilterCutoff => cutoff_path(track, &track_path, sample_rate),
        BuiltinParameterError::FilterOrder => filter_order_path(track, &track_path),
        BuiltinParameterError::MatrixCoefficient => matrix_path(track, &track_path),
        BuiltinParameterError::MatrixSmoothing => {
            format!("{track_path}.matrix_or_pan.smoothing_samples")
        }
        _ => format!("{track_path}.builtins"),
    };
    diag(code, &path)
}

fn gain_path(track: &Track, track_path: &str) -> String {
    for (lane, builtins, fader) in [
        ("left", &track.builtins.left, track.fader.left_db),
        ("right", &track.builtins.right, track.fader.right_db),
    ] {
        if !builtins.trim_db.is_finite() || !(-144.0..=24.0).contains(&builtins.trim_db) {
            return format!("{track_path}.builtins.{lane}.trim_db");
        }
        if !fader.is_finite() || !(-144.0..=24.0).contains(&fader) {
            return format!("{track_path}.fader.{lane}_db");
        }
    }
    format!("{track_path}.builtins")
}

fn cutoff_path(track: &Track, track_path: &str, sample_rate: u32) -> String {
    for (lane, builtins) in [
        ("left", &track.builtins.left),
        ("right", &track.builtins.right),
    ] {
        if invalid_cutoff(builtins.hpf_hz, sample_rate) {
            return format!("{track_path}.builtins.{lane}.hpf_hz");
        }
        if invalid_cutoff(builtins.lpf_hz, sample_rate) {
            return format!("{track_path}.builtins.{lane}.lpf_hz");
        }
    }
    format!("{track_path}.builtins")
}

fn filter_order_path(track: &Track, track_path: &str) -> String {
    if track.builtins.left.hpf_hz > 0.0
        && track.builtins.left.lpf_hz > 0.0
        && track.builtins.left.hpf_hz >= track.builtins.left.lpf_hz
    {
        format!("{track_path}.builtins.left.lpf_hz")
    } else {
        format!("{track_path}.builtins.right.lpf_hz")
    }
}

fn invalid_cutoff(value: f32, sample_rate: u32) -> bool {
    validate_builtin_filter_cutoff(value, sample_rate, 0.0, 10.0).is_err()
}

fn matrix_path(track: &Track, track_path: &str) -> String {
    match track.matrix_or_pan {
        MatrixOrPan::Pan { left, .. } if !left.is_finite() || !(-1.0..=1.0).contains(&left) => {
            format!("{track_path}.matrix_or_pan.left")
        }
        MatrixOrPan::Pan { .. } => format!("{track_path}.matrix_or_pan.right"),
        MatrixOrPan::Matrix { ll, lr, rl, rr, .. } => {
            for (field, value) in [("ll", ll), ("lr", lr), ("rl", rl), ("rr", rr)] {
                if !value.is_finite() || !(-1.0..=1.0).contains(&value) {
                    return format!("{track_path}.matrix_or_pan.{field}");
                }
            }
            format!("{track_path}.matrix_or_pan")
        }
    }
}
fn meter_diagnostic(request: &MeterRequest, error: MeterConfigError) -> BuiltinDiagnostic {
    diag(
        match error {
            MeterConfigError::DecayDomain => "builtin.meter.config",
            MeterConfigError::Queue => "builtin.resource.arithmetic_overflow",
        },
        &meter_path(request),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};
    use core::sync::atomic::{AtomicUsize, Ordering};
    use engine::{QuantumFrames, SampleRateHz};
    use graph::{
        GraphEdge, GraphEdgeId, GraphNode, GraphNodeBinding, GraphPortId, GraphPortKind,
        GraphPreparedSourceSetDriver, GraphResourceEstimate, GraphSourceInputClaim,
        GraphSourceSetResourceReport, PreparedGraphPlanParts,
    };

    /// The compiler always emits `spec.nodes` sorted by id; hand-built fixtures list them in
    /// reading order, so they sort here (`program::lower` interns ids by binary search).
    fn sorted_nodes(mut nodes: Vec<GraphNode>) -> Vec<GraphNode> {
        nodes.sort_by(|left, right| left.id.cmp(&right.id));
        nodes
    }
    use session::{CompileCaps, compile_session, parse_session_json};
    use std::sync::Arc;

    fn session() -> CompiledSession {
        let document = include_str!("../../../fixtures/session/v1/canonical.json");
        compile_session(
            &parse_session_json(document).expect("parse"),
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compile")
    }
    fn caps() -> BuiltinCompileCaps {
        BuiltinCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_total_retained_payload_bytes: u64::MAX,
            maximum_total_meter_items: u64::MAX,
            maximum_total_meter_bytes: u64::MAX,
            maximum_single_allocation_bytes: u64::MAX,
            maximum_meter_streams: u64::MAX,
            maximum_period_frames: u32::MAX,
            maximum_peak_hold_frames: u32::MAX,
            maximum_smoothing_samples: u32::MAX,
        }
    }

    struct SourceSetDriver {
        claim_count: usize,
        marker: u64,
        drops: Arc<AtomicUsize>,
    }

    impl Drop for SourceSetDriver {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }

    impl GraphPreparedSourceSetDriver for SourceSetDriver {
        fn claim_count(&self) -> usize {
            self.claim_count
        }

        fn begin_block(&mut self, _first_sample: u64, _frames: u32) -> Result<(), RenderError> {
            Ok(())
        }

        fn copy_track_input(
            &mut self,
            _claim_index: usize,
            left: &mut [f32],
            right: &mut [f32],
        ) -> Result<(), RenderError> {
            left.fill(0.0);
            right.fill(0.0);
            Ok(())
        }

        fn copy_after_disarm_telemetry(&self, output: &mut [u64]) -> usize {
            if let Some(first) = output.first_mut() {
                *first = self.marker;
                1
            } else {
                0
            }
        }
    }

    struct DropProcessor(Arc<AtomicUsize>);

    impl Drop for DropProcessor {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    impl GraphRuntimeProcessor for DropProcessor {
        fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            Ok(())
        }
    }

    struct NoopObserver;

    impl GraphRuntimeObserver for NoopObserver {
        fn observe(&mut self, _block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
            Ok(())
        }
    }

    struct SourceBindFixture {
        artifact: PreparedBuiltinsGraphArtifact<u64>,
        bindings: GraphRuntimeBindings,
        source_set: GraphPreparedSourceSet,
        input: GraphNodeId,
        builtin: GraphNodeId,
        output: GraphNodeId,
        builtin_drops: Arc<AtomicUsize>,
        external_drops: Arc<AtomicUsize>,
        source_drops: Arc<AtomicUsize>,
    }

    struct SourceBindOwnership {
        input: GraphNodeId,
        builtin_drops: Arc<AtomicUsize>,
        external_drops: Arc<AtomicUsize>,
        source_drops: Arc<AtomicUsize>,
    }

    impl SourceBindFixture {
        fn ownership(&self) -> SourceBindOwnership {
            SourceBindOwnership {
                input: self.input.clone(),
                builtin_drops: Arc::clone(&self.builtin_drops),
                external_drops: Arc::clone(&self.external_drops),
                source_drops: Arc::clone(&self.source_drops),
            }
        }
    }

    fn zero_graph_estimate() -> GraphResourceEstimate {
        GraphResourceEstimate {
            logical_nodes: 0,
            materialized_nodes: 0,
            edges: 0,
            schedule_items: 0,
            dependency_levels: 0,
            reductions: 0,
            routes: 0,
            effects: 0,
            audio_buffer_samples: 0,
            total_delay_samples: 0,
            delay_bytes: 0,
            graph_metadata_bytes: 0,
            declared_effect_bytes: 0,
            effect_bank_count: 0,
            effect_bank_scratch_bytes: 0,
            effect_bank_runtime_buffer_bytes: 0,
            effect_bank_metadata_bytes: 0,
            builtin_bank_bytes: 0,
            builtin_bank_scratch_bytes: 0,
            builtin_bank_count: 0,
            largest_allocation_bytes: 0,
            incremental_plan_bytes: 0,
            session_plus_plan_bytes: 0,
        }
    }

    fn source_bind_fixture() -> SourceBindFixture {
        let envelope = RenderEnvelope {
            sample_rate: SampleRateHz(48_000),
            quantum: QuantumFrames(4),
            input_channels: None,
            output_channels: NonZeroUsize::new(2).expect("two output channels"),
        };
        let track_id = StableGraphId::parse("source-track").expect("test ID");
        let input = GraphNodeId::TrackStage {
            track_id: track_id.clone(),
            stage: TrackStage::Input,
        };
        let builtin = GraphNodeId::TrackStage {
            track_id,
            stage: TrackStage::PostInputBuiltins,
        };
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main-output").expect("test ID"),
        };
        let edge = |source: GraphNodeId, target: GraphNodeId| GraphEdge {
            id: GraphEdgeId::TrackMain {
                target: target.clone(),
            },
            source: GraphPortId {
                node: source,
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: target,
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: "$.source-bind-test".to_owned(),
        };
        let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
            plan_id: 22,
            spec: graph::GraphSpec {
                nodes: sorted_nodes(
                    [input.clone(), builtin.clone(), output.clone()]
                        .into_iter()
                        .map(|id| GraphNode {
                            id,
                            latency: effect_contract::LatencySamples(0),
                            tail: effect_contract::TailSamples::Finite(0),
                        })
                        .collect(),
                ),
                ports: Vec::new(),
                edges: vec![
                    edge(input.clone(), builtin.clone()),
                    edge(builtin.clone(), output.clone()),
                ],
            },
            sequential_schedule: vec![input.clone(), builtin.clone(), output.clone()],
            dependency_levels: vec![
                DependencyLevel {
                    level: 0,
                    nodes: vec![input.clone()],
                },
                DependencyLevel {
                    level: 1,
                    nodes: vec![builtin.clone()],
                },
                DependencyLevel {
                    level: 2,
                    nodes: vec![output.clone()],
                },
            ],
            route_timings: Vec::new(),
            inserted_delays: Vec::new(),
            buffer_assignments: Vec::new(),
            estimate: zero_graph_estimate(),
            envelope,
            required_bindings: vec![input.clone(), builtin.clone(), output.clone()],
            routes: Vec::new(),
            track_delays: Vec::new(),
            effects: Vec::new(),
            effect_controls: Vec::new(),
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
            effect_observations: Vec::new(),
        });
        let builtin_drops = Arc::new(AtomicUsize::new(0));
        let external_drops = Arc::new(AtomicUsize::new(0));
        let source_drops = Arc::new(AtomicUsize::new(0));
        let artifact = PreparedBuiltinsGraphArtifact {
            graph,
            builtin_processors: vec![GraphNodeBinding::new(
                builtin.clone(),
                Box::new(DropProcessor(Arc::clone(&builtin_drops))),
            )],
            builtin_observers: vec![GraphNodeObserverBinding::new(
                builtin.clone(),
                0x22_73,
                Box::new(NoopObserver),
            )],
            report: 0x22_73,
            track_controls: Vec::new(),
            meter_consumers: Vec::new(),
        };
        let bindings = GraphRuntimeBindings {
            envelope,
            nodes: vec![GraphNodeBinding::new(
                output.clone(),
                Box::new(DropProcessor(Arc::clone(&external_drops))),
            )],
            observers: Vec::new(),
        };
        let source_set = GraphPreparedSourceSet::new(
            envelope,
            vec![GraphSourceInputClaim {
                node: input.clone(),
            }],
            GraphSourceSetResourceReport {
                pcm_payload_already_charged_bytes: 0,
                overhead_bytes: 0,
                total_engine_owned_bytes: 0,
                largest_allocation_bytes: 0,
            },
            Box::new(SourceSetDriver {
                claim_count: 1,
                marker: 0x22_73,
                drops: Arc::clone(&source_drops),
            }),
        );
        SourceBindFixture {
            artifact,
            bindings,
            source_set,
            input,
            builtin,
            output,
            builtin_drops,
            external_drops,
            source_drops,
        }
    }

    fn assert_source_bind_failure_ownership(
        failure: &PreparedBuiltinsGraphSourceBindFailure<u64>,
        ownership: &SourceBindOwnership,
        expected_binding_nodes: &[GraphNodeId],
    ) {
        assert_eq!(*failure.artifact.report(), 0x22_73);
        assert_eq!(
            failure
                .bindings
                .nodes
                .iter()
                .map(|binding| binding.node.clone())
                .collect::<Vec<_>>(),
            expected_binding_nodes
        );
        assert_eq!(
            failure.source_set.claims(),
            [GraphSourceInputClaim {
                node: ownership.input.clone()
            }]
        );
        let mut telemetry = [0];
        assert_eq!(
            failure
                .source_set
                .copy_after_disarm_telemetry(&mut telemetry),
            1
        );
        assert_eq!(telemetry, [0x22_73]);
        assert_eq!(ownership.builtin_drops.load(Ordering::SeqCst), 0);
        assert_eq!(ownership.external_drops.load(Ordering::SeqCst), 0);
        assert_eq!(ownership.source_drops.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn source_set_bind_succeeds_with_private_builtin_and_external_ownership() {
        let fixture = source_bind_fixture();
        let SourceBindFixture {
            artifact,
            bindings,
            source_set,
            builtin_drops,
            external_drops,
            source_drops,
            ..
        } = fixture;
        let bound = artifact
            .into_bound_with_source_set(bindings, source_set)
            .unwrap_or_else(|failure| panic!("source-set bind rejected: {}", failure.code));
        assert!(bound.meter_consumers.is_empty());
        assert_eq!(builtin_drops.load(Ordering::SeqCst), 0);
        assert_eq!(external_drops.load(Ordering::SeqCst), 0);
        assert_eq!(source_drops.load(Ordering::SeqCst), 0);
        drop(bound);
        assert_eq!(builtin_drops.load(Ordering::SeqCst), 1);
        assert_eq!(external_drops.load(Ordering::SeqCst), 1);
        assert_eq!(source_drops.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn source_set_bind_prevalidation_returns_all_ownership_for_each_code() {
        {
            let mut fixture = source_bind_fixture();
            fixture.bindings.envelope.quantum = QuantumFrames(8);
            let ownership = fixture.ownership();
            let expected = [fixture.output.clone()];
            let failure = match fixture
                .artifact
                .into_bound_with_source_set(fixture.bindings, fixture.source_set)
            {
                Ok(_) => panic!("envelope mismatch must reject"),
                Err(failure) => failure,
            };
            assert_eq!(failure.code, "graph.plan.envelope_mismatch");
            assert_source_bind_failure_ownership(&failure, &ownership, &expected);
        }
        {
            let mut fixture = source_bind_fixture();
            fixture.bindings.nodes.push(GraphNodeBinding::new(
                fixture.output.clone(),
                Box::new(DropProcessor(Arc::clone(&fixture.external_drops))),
            ));
            let ownership = fixture.ownership();
            let expected = [fixture.output.clone(), fixture.output.clone()];
            let failure = match fixture
                .artifact
                .into_bound_with_source_set(fixture.bindings, fixture.source_set)
            {
                Ok(_) => panic!("duplicate external binding must reject"),
                Err(failure) => failure,
            };
            assert_eq!(failure.code, "graph.plan.binding");
            assert_source_bind_failure_ownership(&failure, &ownership, &expected);
        }
        {
            let mut fixture = source_bind_fixture();
            fixture.bindings.nodes[0].node = fixture.builtin.clone();
            let ownership = fixture.ownership();
            let expected = [fixture.builtin.clone()];
            let failure = match fixture
                .artifact
                .into_bound_with_source_set(fixture.bindings, fixture.source_set)
            {
                Ok(_) => panic!("external builtin overlap must reject"),
                Err(failure) => failure,
            };
            assert_eq!(failure.code, "graph.plan.binding");
            assert_source_bind_failure_ownership(&failure, &ownership, &expected);
        }
        {
            let mut fixture = source_bind_fixture();
            fixture.bindings.nodes.clear();
            fixture.external_drops.store(0, Ordering::SeqCst);
            let ownership = fixture.ownership();
            let failure = match fixture
                .artifact
                .into_bound_with_source_set(fixture.bindings, fixture.source_set)
            {
                Ok(_) => panic!("missing external binding must reject"),
                Err(failure) => failure,
            };
            assert_eq!(failure.code, "source.graph.binding_mismatch");
            assert_source_bind_failure_ownership(&failure, &ownership, &[]);
        }
        {
            let mut fixture = source_bind_fixture();
            fixture
                .bindings
                .observers
                .push(GraphNodeObserverBinding::new(
                    fixture.output.clone(),
                    1,
                    Box::new(NoopObserver),
                ));
            let ownership = fixture.ownership();
            let expected = [fixture.output.clone()];
            let failure = match fixture
                .artifact
                .into_bound_with_source_set(fixture.bindings, fixture.source_set)
            {
                Ok(_) => panic!("invalid observer node must reject"),
                Err(failure) => failure,
            };
            assert_eq!(failure.code, "graph.plan.observer");
            assert_eq!(failure.bindings.observers.len(), 1);
            assert_source_bind_failure_ownership(&failure, &ownership, &expected);
        }
    }

    #[test]
    fn delegated_source_rejection_restores_private_and_external_ownership() {
        let mut fixture = source_bind_fixture();
        fixture.bindings.nodes.push(GraphNodeBinding::new(
            fixture.input.clone(),
            Box::new(DropProcessor(Arc::clone(&fixture.external_drops))),
        ));
        let ownership = fixture.ownership();
        let expected = [fixture.output.clone(), fixture.input.clone()];
        let expected_builtin = fixture.builtin.clone();
        let failure = match fixture
            .artifact
            .into_bound_with_source_set(fixture.bindings, fixture.source_set)
        {
            Ok(_) => panic!("source/external overlap must reject in graph bind"),
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "source.graph.binding_mismatch");
        assert_eq!(failure.artifact.builtin_processors.len(), 1);
        assert_eq!(
            failure.artifact.builtin_processors[0].node,
            expected_builtin
        );
        assert_eq!(failure.artifact.builtin_observers.len(), 1);
        assert_eq!(failure.artifact.builtin_observers[0].handle, 0x22_73);
        assert_source_bind_failure_ownership(&failure, &ownership, &expected);
    }

    #[test]
    fn builtin_bank_layout_regroups_by_dependency_wave_and_scalar_falls_back() {
        let inputs: Vec<Box<str>> = (0..17)
            .map(|index| Box::<str>::from(format!("bank{index}")))
            .collect();
        let node = |index| GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(&format!("bank{index}")).expect("id"),
            stage: TrackStage::PostInputBuiltins,
        };
        let levels = vec![
            DependencyLevel {
                level: 0,
                nodes: vec![node(0)],
            },
            DependencyLevel {
                level: 1,
                nodes: (1..10).map(node).collect(),
            },
            DependencyLevel {
                level: 2,
                nodes: (10..17).map(node).collect(),
            },
        ];
        // The 1 / 9 / 7 layout, banked per level with the last bank of each level padded:
        // W4 -> 1 | 4 4 1 | 4 3 and W8 -> 1 | 8 1 | 7.  Hand counts of `n.div_ceil(W)` banks per
        // level; every post-input node is a member, so no level contributes a scalar tail.
        for (dispatch, expected_sizes) in [
            (Backend::Simd4, &[1, 4, 4, 1, 4, 3][..]),
            (Backend::Simd8, &[1, 8, 1, 7][..]),
            // D4: the scalar backend has no bank width at all -- one arithmetic graph everywhere
            // means fusion is written, not inferred, so those tracks stay on the scalar `Lane`.
            (Backend::Scalar, &[][..]),
        ] {
            let groups = planned_builtin_bank_members(
                &inputs,
                TrackStage::PostInputBuiltins,
                dispatch,
                &levels,
                &SessionPoolClasses::default(),
            );
            let sizes: Vec<_> = groups.iter().map(|members| members.len()).collect();
            assert_eq!(sizes, expected_sizes, "{:?}", dispatch);
            assert_eq!(
                sizes.iter().sum::<usize>(),
                if expected_sizes.is_empty() { 0 } else { 17 },
                "every post-input node is banked once"
            );
            let mut group_levels = Vec::new();
            assert!(groups.iter().all(|members| {
                let member_levels: BTreeSet<_> = members
                    .iter()
                    .map(|member| {
                        levels
                            .iter()
                            .find(|level| level.nodes.contains(member))
                            .expect("member level")
                            .level
                    })
                    .collect();
                group_levels.extend(member_levels.iter().copied());
                member_levels.len() == 1 && members.windows(2).all(|pair| pair[0] < pair[1])
            }));
            assert!(
                group_levels.windows(2).all(|pair| pair[0] <= pair[1]),
                "banks are emitted in dependency-level order"
            );
        }
        let scalar = Backend::Scalar;
        assert!(
            planned_builtin_bank_members(
                &inputs,
                TrackStage::PostInputBuiltins,
                scalar,
                &levels,
                &SessionPoolClasses::default(),
            )
            .is_empty()
        );
    }
    /// F4/F11: a bank is charged for the two main planes it actually owns and for the member ids
    /// it actually holds -- a padded bank is not charged a full-width id array, and no bank is
    /// charged the two sidechain planes a fixed stage can never reach.
    #[test]
    fn builtin_bank_resource_charges_two_planes_and_actual_members() {
        let node = |index: usize| GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(&format!("bank{index}")).expect("id"),
            stage: TrackStage::PostInputBuiltins,
        };
        let quantum = 64_u32;
        for (width, sizes) in [
            (effect_contract::BankWidth::Four, &[4, 3][..]),
            (effect_contract::BankWidth::Eight, &[8, 1][..]),
            (effect_contract::BankWidth::Eight, &[5][..]),
        ] {
            let mut next = 0_usize;
            let groups: Vec<Box<[GraphNodeId]>> = sizes
                .iter()
                .map(|size| {
                    (0..*size)
                        .map(|_| {
                            next += 1;
                            node(next - 1)
                        })
                        .collect::<Vec<_>>()
                        .into_boxed_slice()
                })
                .collect();
            let processor_bytes =
                strip_processor_bytes(TrackStage::PostInputBuiltins, width).expect("bankable");
            let resource = builtin_bank_resource(&groups, width, quantum, processor_bytes)
                .expect("padded layout is chargeable");

            // Hand formula, written from `size_of` rather than from the function under test.
            let lanes = u64::from(width.lanes());
            let banks = sizes.len() as u64;
            let node_bytes = core::mem::size_of::<GraphNodeId>() as u64;
            // The struct **plus** the per-lane console-consumer array it owns. #210 phase 3 gave
            // the input bank the same shape the fader and matrix banks already had: one
            // `Option<Consumer<_>>` per lane, allocated whether or not a console is attached, so
            // that a banked session's retained payload does not depend on whether the host leased
            // one. Written out here rather than read off `strip_processor_bytes`, which is the
            // function under test.
            let processor_bytes = core::mem::size_of::<BuiltinBankProcessor>() as u64
                + core::mem::size_of::<Option<Consumer<TrackInputRecord>>>() as u64 * lanes;
            let plane_bytes = u64::from(quantum) * lanes * 4;
            let string_lengths: Vec<u64> = groups
                .iter()
                .flat_map(|members| members.iter())
                .map(|member| match member {
                    GraphNodeId::TrackStage { track_id, .. } => track_id.as_str().len() as u64,
                    _ => unreachable!("member kind"),
                })
                .collect();
            let strings: u64 = string_lengths.iter().sum();
            let largest_string = string_lengths.iter().copied().max().expect("a member");
            assert_eq!(resource.bank_count, banks);
            assert_eq!(
                resource.scratch_samples,
                banks * u64::from(quantum) * lanes * 2
            );
            assert_eq!(resource.scratch_bytes, banks * plane_bytes * 2);
            assert_eq!(
                resource.payload_bytes,
                sizes
                    .iter()
                    .map(|size| node_bytes * *size as u64 + processor_bytes)
                    .sum::<u64>()
                    + strings
            );
            assert_eq!(
                resource.metadata_bytes,
                banks * core::mem::size_of::<GraphPreparedBuiltinBank>() as u64
            );
            assert_eq!(
                resource.largest_allocation_bytes,
                [
                    node_bytes * *sizes.iter().max().expect("a bank") as u64,
                    processor_bytes,
                    largest_string,
                    plane_bytes,
                    banks * core::mem::size_of::<GraphPreparedBuiltinBank>() as u64,
                ]
                .into_iter()
                .max()
                .expect("a term")
            );
        }
        // Only an empty or oversized group is unchargeable now.
        assert!(
            builtin_bank_resource(
                &[Vec::new().into_boxed_slice()],
                effect_contract::BankWidth::Four,
                64,
                0
            )
            .is_none()
        );
        assert!(
            builtin_bank_resource(
                &[(0..5).map(node).collect::<Vec<_>>().into_boxed_slice()],
                effect_contract::BankWidth::Four,
                64,
                0
            )
            .is_none()
        );
    }

    // ---------------------------------------------------------------------------------------
    // #86 A7: the bit-identity harness.
    //
    // These tests render through the production path -- `prepare_session_builtins` ->
    // `into_graph_artifact_with_banks` -> `into_bound` -> `PreparedRenderPlan::render` -- and
    // compare `to_bits()` of the post-input-builtins output, per track, per channel, per block.
    // The oracle is the same generic kernel body at `L = f32` reached through `Scalar` dispatch,
    // which is independent of the vector wrapper, the AoSoA transposes and the padding.
    // ---------------------------------------------------------------------------------------

    /// One session of `n` tracks with deliberately distinct per-track builtins.
    fn n_track_session(n: usize) -> CompiledSession {
        let mut model =
            parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json"))
                .expect("fixture parse");
        let mut template = model.tracks[0].clone();
        template.simd1.effects.clear();
        template.dynamic.effects.clear();
        template.simd2.effects.clear();
        model.automation.clear();
        model.tracks.clear();
        for index in 0..n {
            let mut track = template.clone();
            track.id = session::StableId::parse(&track_name(index)).expect("generated stable ID");
            let scale = index as f32;
            track.builtins.left.hpf_hz = 20.0 + 5.0 * scale;
            track.builtins.left.lpf_hz = 18_000.0 - 100.0 * scale;
            track.builtins.left.trim_db = -3.0 + 0.5 * scale;
            track.builtins.left.polarity_invert = index % 4 == 3;
            track.builtins.right.hpf_hz = if index % 2 == 0 {
                0.0
            } else {
                30.0 + 3.0 * scale
            };
            track.builtins.right.lpf_hz = if index % 3 == 0 {
                0.0
            } else {
                17_000.0 - 250.0 * scale
            };
            track.builtins.right.polarity_invert = index % 2 == 1;
            track.builtins.right.trim_db = 1.0 - 0.25 * scale;
            track.fader.left_db = -1.0 + 0.125 * scale;
            track.fader.right_db = 0.5 - 0.125 * scale;
            model.tracks.push(track);
        }
        model.routes.truncate(1);
        model.routes[0].source = session::RouteSource::Track {
            track_id: session::StableId::parse(&track_name(0)).expect("route track"),
            tap: session::SendTap::PostMatrix,
        };
        compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("harness session compile")
    }

    fn track_name(index: usize) -> String {
        format!("t{index:02}")
    }

    const HARNESS_QUANTUM: u32 = 64;
    const HARNESS_BLOCKS: u64 = 3;

    /// A deterministic per-track input signal: an LCG seeded by `(track, first_sample)`.
    ///
    /// It is a plain `GraphRuntimeProcessor` bound at the `Input` stage, so every dispatch under
    /// test sees byte-identical input.
    struct SeededInput {
        seed: u64,
    }

    impl GraphRuntimeProcessor for SeededInput {
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            let mut state = self.seed ^ block.first_sample.wrapping_mul(0x9e37_79b9_7f4a_7c15);
            let mut next = || {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                let unit = ((state >> 40) as f32) / ((1_u32 << 24) as f32);
                (unit * 2.0 - 1.0) * 0.8
            };
            for (left, right) in block.left.iter_mut().zip(block.right.iter_mut()) {
                *left = next();
                *right = next();
            }
            Ok(())
        }
    }

    /// Records `to_bits()` of both channels after its node completes.
    struct Capture(Arc<std::sync::Mutex<Vec<u32>>>);

    impl GraphRuntimeObserver for Capture {
        fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
            let mut sink = self.0.lock().expect("harness capture");
            sink.extend(block.left.iter().map(|sample| sample.to_bits()));
            sink.extend(block.right.iter().map(|sample| sample.to_bits()));
            Ok(())
        }
    }

    /// Builds the five-level graph the harness renders: one level per track stage.
    ///
    /// `Output` is fed by track 0's `PostMatrix` only: `GraphEdgeId::TrackMain { target }` is not
    /// unique for fan-in, and a reduction is not what this harness measures.
    fn track_graph(n: usize) -> (PreparedGraphPlan, Vec<DependencyLevel>) {
        let envelope = RenderEnvelope {
            sample_rate: SampleRateHz(48_000),
            quantum: QuantumFrames(HARNESS_QUANTUM),
            input_channels: None,
            output_channels: NonZeroUsize::new(2).expect("two output channels"),
        };
        let stage = |index: usize, stage: TrackStage| GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(&track_name(index)).expect("harness ID"),
            stage,
        };
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main-out").expect("harness ID"),
        };
        let stages = [
            TrackStage::Input,
            TrackStage::PostInputBuiltins,
            TrackStage::PostFader,
            TrackStage::PostMatrix,
        ];
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for index in 0..n {
            for pair in stages.windows(2) {
                let source = stage(index, pair[0]);
                let target = stage(index, pair[1]);
                edges.push(GraphEdge {
                    id: GraphEdgeId::TrackMain {
                        target: target.clone(),
                    },
                    source: GraphPortId {
                        node: source,
                        kind: GraphPortKind::MainOutput,
                        effect_port: None,
                    },
                    destination: GraphPortId {
                        node: target,
                        kind: GraphPortKind::MainInput,
                        effect_port: None,
                    },
                    path: format!("$.tracks[{index}].chain"),
                });
            }
        }
        edges.push(GraphEdge {
            id: GraphEdgeId::TrackMain {
                target: output.clone(),
            },
            source: GraphPortId {
                node: stage(0, TrackStage::PostMatrix),
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: output.clone(),
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: "$.routes[0]".to_owned(),
        });
        let mut levels = Vec::new();
        for (level, kind) in stages.iter().enumerate() {
            let level_nodes: Vec<_> = (0..n).map(|index| stage(index, *kind)).collect();
            nodes.extend(level_nodes.iter().cloned());
            levels.push(DependencyLevel {
                level: level as u64,
                nodes: level_nodes,
            });
        }
        nodes.push(output.clone());
        levels.push(DependencyLevel {
            level: stages.len() as u64,
            nodes: vec![output.clone()],
        });
        let schedule: Vec<_> = levels
            .iter()
            .flat_map(|level| level.nodes.iter().cloned())
            .collect();
        let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
            plan_id: 8_600 + n as u64,
            spec: graph::GraphSpec {
                nodes: sorted_nodes(
                    nodes
                        .iter()
                        .cloned()
                        .map(|id| GraphNode {
                            id,
                            latency: effect_contract::LatencySamples(0),
                            tail: effect_contract::TailSamples::Finite(0),
                        })
                        .collect(),
                ),
                ports: Vec::new(),
                edges,
            },
            sequential_schedule: schedule,
            dependency_levels: levels.clone(),
            route_timings: Vec::new(),
            inserted_delays: Vec::new(),
            buffer_assignments: Vec::new(),
            estimate: zero_graph_estimate(),
            envelope,
            required_bindings: nodes,
            routes: Vec::new(),
            track_delays: Vec::new(),
            effects: Vec::new(),
            effect_controls: Vec::new(),
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
            effect_observations: Vec::new(),
        });
        (graph, levels)
    }

    /// Renders `HARNESS_BLOCKS` blocks and returns the post-input-builtins output bits per track.
    fn render_post_input_bits(n: usize, dispatch: Backend) -> (Vec<Vec<u32>>, usize) {
        let compiled = n_track_session(n);
        let builtins = prepare_session_builtins(&compiled, &[], caps()).expect("harness builtins");
        let (graph, levels) = track_graph(n);
        let classes = SessionPoolClasses::from_session(&compiled);
        let mut artifact =
            builtins.into_graph_artifact_with_banks(graph, (), dispatch, &levels, &classes);
        let bank_count = artifact.graph.prepared_builtin_bank_count();
        let captures: Vec<_> = (0..n)
            .map(|_| Arc::new(std::sync::Mutex::new(Vec::new())))
            .collect();
        for (index, capture) in captures.iter().enumerate() {
            artifact
                .builtin_observers
                .push(GraphNodeObserverBinding::new(
                    GraphNodeId::TrackStage {
                        track_id: StableGraphId::parse(&track_name(index)).expect("harness ID"),
                        stage: TrackStage::PostInputBuiltins,
                    },
                    0x8600 + index as u64,
                    Box::new(Capture(Arc::clone(capture))),
                ));
        }
        let envelope = artifact.graph.envelope;
        let mut nodes: Vec<_> = (0..n)
            .map(|index| {
                GraphNodeBinding::new(
                    GraphNodeId::TrackStage {
                        track_id: StableGraphId::parse(&track_name(index)).expect("harness ID"),
                        stage: TrackStage::Input,
                    },
                    Box::new(SeededInput {
                        seed: 0x5eed_0000 ^ index as u64,
                    }) as Box<dyn GraphRuntimeProcessor>,
                )
            })
            .collect();
        nodes.push(GraphNodeBinding::new(
            GraphNodeId::Output {
                output_id: StableGraphId::parse("main-out").expect("harness ID"),
            },
            Box::new(HarnessSink) as Box<dyn GraphRuntimeProcessor>,
        ));
        let mut plan = match artifact.into_bound(GraphRuntimeBindings {
            envelope,
            nodes,
            observers: Vec::new(),
        }) {
            Ok(bound) => bound.plan,
            Err(failure) => panic!("harness bind: {}", failure.code),
        };
        let frames = HARNESS_QUANTUM as usize;
        let mut pcm = vec![0.0_f32; frames * 2];
        for block in 0..HARNESS_BLOCKS {
            plan.render(
                engine::realtime::RenderIo {
                    input: None,
                    output: engine::realtime::PlanarBufferMut::try_new(&mut pcm, 2, frames, frames)
                        .expect("harness output"),
                },
                engine::realtime::RenderTime {
                    absolute_sample: block * HARNESS_QUANTUM as u64,
                },
            )
            .expect("harness render");
        }
        let bits = captures
            .into_iter()
            .map(|capture| {
                let taken = capture.lock().expect("harness capture").clone();
                assert_eq!(taken.len(), frames * 2 * HARNESS_BLOCKS as usize);
                taken
            })
            .collect();
        (bits, bank_count)
    }

    struct HarnessSink;

    impl GraphRuntimeProcessor for HarnessSink {
        fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            Ok(())
        }
    }

    fn host_dispatch() -> Backend {
        Backend::current()
    }

    fn scalar_dispatch() -> Backend {
        Backend::Scalar
    }

    /// E2 (#86 F2 proof, F3): a track's bits do not depend on whether it renders in a bank.
    ///
    /// `n = 1..=9` covers the short first bank, the exactly-full bank and the padded second bank
    /// at both widths. The oracle is the `Scalar` dispatch, which banks nothing at all.
    #[test]
    fn banked_tracks_are_bit_identical_to_their_scalar_tails() {
        let host = host_dispatch();
        for n in 1..=9 {
            let (banked, bank_count) = render_post_input_bits(n, host);
            let (scalar, scalar_banks) = render_post_input_bits(n, scalar_dispatch());
            assert_eq!(scalar_banks, 0, "the scalar oracle banks nothing");
            match BankWidth::for_backend(host) {
                // Three bankable stages per track since #212 -- post-input builtins, fader,
                // matrix -- each grouping the same `n` tracks into the same padded banks.
                Some(width) => assert_eq!(
                    bank_count,
                    3 * n.div_ceil(width.lanes() as usize),
                    "padded bank count for {n} tracks over three bankable stages"
                ),
                None => assert_eq!(bank_count, 0),
            }
            // Distinctness guard: a harness whose tracks all render silence, or render the same
            // thing, would pass every identity assertion below without testing anything.
            assert!(
                banked
                    .iter()
                    .all(|bits| bits.iter().any(|bits| *bits != 0 && *bits != 0x8000_0000)),
                "every track must carry signal"
            );
            let distinct: BTreeSet<_> = banked.iter().collect();
            assert_eq!(distinct.len(), n, "every track must render differently");
            for (track, (banked, scalar)) in banked.iter().zip(&scalar).enumerate() {
                assert_eq!(
                    banked, scalar,
                    "track {track} of {n} differs between the bank and the scalar tail"
                );
            }
        }
    }

    /// E3 (#86 F2 partition invariance): adding a track never moves an existing track's bits.
    ///
    /// 7, 8 and 9 tracks straddle the W8 bank boundary in both directions, and 3/4/5 straddle W4.
    #[test]
    fn track_bits_do_not_depend_on_session_track_count() {
        let host = host_dispatch();
        let renders: BTreeMap<usize, Vec<Vec<u32>>> = [3, 4, 5, 7, 8, 9]
            .into_iter()
            .map(|n| (n, render_post_input_bits(n, host).0))
            .collect();
        let reference = render_post_input_bits(7, scalar_dispatch()).0;
        for (n, bits) in &renders {
            for (track, (bits, reference)) in bits.iter().zip(&reference).enumerate() {
                assert_eq!(
                    bits, reference,
                    "track {track} moved between a {n}-track session and the 7-track scalar oracle"
                );
            }
        }
        for smaller in [3, 4, 5, 7, 8] {
            for larger in [4, 5, 7, 8, 9] {
                if larger <= smaller {
                    continue;
                }
                for (track, (small, large)) in
                    renders[&smaller].iter().zip(&renders[&larger]).enumerate()
                {
                    assert_eq!(
                        small, large,
                        "track {track} moved from a {smaller}-track to a {larger}-track session"
                    );
                }
            }
        }
    }

    fn handle(value: u64) -> MeterHandle {
        MeterHandle(NonZeroU64::new(value).expect("nonzero test meter handle"))
    }
    #[test]
    fn prepares_three_sections_and_each_named_meter_tap() {
        let config = MeterConfig {
            period_frames: NonZeroU32::new(16).expect("constant"),
            peak_hold_frames: 8,
            peak_decay_db_per_second: 12.0,
            queue_capacity: NonZeroUsize::new(4).expect("constant"),
            reset_generation: 3,
        };
        let requests: Vec<_> = [
            MeterTap::Input,
            MeterTap::PostInputBuiltins,
            MeterTap::PostSimd1,
            MeterTap::PostDynamic,
            MeterTap::PostSimd2PreFader,
            MeterTap::PostFader,
            MeterTap::PostMatrix,
        ]
        .into_iter()
        .enumerate()
        .map(|(index, tap)| MeterRequest {
            handle: handle(u64::try_from(index).expect("bounded") + 1),
            track_id: "vocal".to_owned(),
            tap,
            config,
        })
        .collect();
        let prepared = prepare_session_builtins(&session(), &requests, caps()).expect("prepare");
        // All three of the track's stages are deferred strip stages now (#212 for the fader and
        // the matrix, #210 phase 3 for the input), so preparation holds no bindings at all.
        assert_eq!(prepared.processor_count(), 3);
        assert_eq!(prepared.processors.len(), 0);
        assert_eq!(prepared.strips.len(), 1);
        assert_eq!(prepared.observers.len(), 7);
        assert_eq!(prepared.meter_consumers.len(), 7);
        assert_eq!(prepared.resources.meter_items, 35);
        assert!(prepared.resources.engine_owned_processor_payload_bytes > 0);
        assert!(
            prepared.resources.engine_owned_meter_payload_bytes
                > 35 * core::mem::size_of::<MeterSnapshot>() as u64
        );
        assert!(
            prepared.resources.maximum_single_allocation_bytes
                >= 5 * core::mem::size_of::<MeterSnapshot>() as u64
        );
        assert_eq!(
            prepared.tails().collect::<Vec<_>>(),
            vec![("vocal", BuiltinTail::Infinite)]
        );
    }
    #[test]
    fn rejects_duplicate_and_unknown_meter_transactionally() {
        let config = MeterConfig {
            period_frames: NonZeroU32::new(16).expect("constant"),
            peak_hold_frames: 0,
            peak_decay_db_per_second: 0.0,
            queue_capacity: NonZeroUsize::new(1).expect("constant"),
            reset_generation: 0,
        };
        let result = prepare_session_builtins(
            &session(),
            &[
                MeterRequest {
                    handle: handle(1),
                    track_id: "missing".to_owned(),
                    tap: MeterTap::Input,
                    config,
                },
                MeterRequest {
                    handle: handle(1),
                    track_id: "missing".to_owned(),
                    tap: MeterTap::Input,
                    config,
                },
            ],
            caps(),
        );
        let Err(error) = result else {
            panic!("must reject");
        };
        assert_eq!(
            error.0.iter().map(|item| item.code).collect::<Vec<_>>(),
            vec![
                "builtin.meter.duplicate",
                "builtin.meter.duplicate_handle",
                "builtin.meter.unknown_track"
            ]
        );
    }

    #[test]
    fn resource_estimate_enforces_the_actual_largest_retained_payload() {
        let config = MeterConfig {
            period_frames: NonZeroU32::new(16).expect("constant"),
            peak_hold_frames: 0,
            peak_decay_db_per_second: 0.0,
            queue_capacity: NonZeroUsize::new(1).expect("constant"),
            reset_generation: 0,
        };
        let requests = [MeterRequest {
            handle: handle(1),
            track_id: "vocal".to_owned(),
            tap: MeterTap::PostMatrix,
            config,
        }];
        let baseline = prepare_session_builtins(&session(), &requests, caps()).expect("prepare");
        let mut constrained = caps();
        constrained.maximum_single_allocation_bytes = baseline
            .resources
            .maximum_single_allocation_bytes
            .saturating_sub(1);
        let Err(error) = prepare_session_builtins(&session(), &requests, constrained) else {
            panic!("largest retained payload must be capped");
        };
        assert_eq!(
            error.0,
            vec![diag("builtin.resource.limit", "$.builtin_compile_caps")]
        );
    }

    #[test]
    fn retained_payload_boundaries_reject_in_phase_one() {
        let config = MeterConfig {
            period_frames: NonZeroU32::new(16).expect("constant"),
            peak_hold_frames: 0,
            peak_decay_db_per_second: 0.0,
            queue_capacity: NonZeroUsize::new(1).expect("constant"),
            reset_generation: 0,
        };
        let requests = [MeterRequest {
            handle: handle(1),
            track_id: "vocal".to_owned(),
            tap: MeterTap::PostMatrix,
            config,
        }];
        let baseline = prepare_session_builtins(&session(), &requests, caps()).expect("prepare");
        let report = baseline.resource_report();
        let mut state_limited = caps();
        state_limited.maximum_total_state_bytes = report
            .engine_owned_processor_payload_bytes
            .checked_sub(1)
            .expect("nonzero processor payload");
        let mut meter_limited = caps();
        meter_limited.maximum_total_meter_bytes = report
            .engine_owned_meter_payload_bytes
            .checked_sub(1)
            .expect("nonzero meter payload");
        for limited in [state_limited, meter_limited] {
            let Err(error) = prepare_session_builtins(&session(), &requests, limited) else {
                panic!("one byte below a retained-payload boundary must reject");
            };
            assert_eq!(
                error.0,
                vec![diag("builtin.resource.limit", "$.builtin_compile_caps")]
            );
        }
    }

    #[test]
    fn cutoff_boundaries_match_compiler_diagnostics_at_every_launch_rate_and_section() {
        let document = include_str!("../../../fixtures/session/v1/canonical.json");
        let base_model = parse_session_json(document).expect("parse boundary session");
        for (rate, maximum_bits) in [
            (44_100, 0x46ac_42f7),
            (48_000, 0x46bb_7ede),
            (88_200, 0x472c_42f7),
            (96_000, 0x473b_7ede),
        ] {
            for (high_pass, path) in [
                (true, "$.tracks[id=vocal].builtins.left.hpf_hz"),
                (false, "$.tracks[id=vocal].builtins.left.lpf_hz"),
            ] {
                let prepare = |cutoff: f32| {
                    let mut model = base_model.clone();
                    model.sample_rate_hz = rate;
                    for track in &mut model.tracks {
                        track.builtins.left.hpf_hz = 0.0;
                        track.builtins.left.lpf_hz = 0.0;
                        track.builtins.right.hpf_hz = 0.0;
                        track.builtins.right.lpf_hz = 0.0;
                    }
                    if high_pass {
                        model.tracks[0].builtins.left.hpf_hz = cutoff;
                    } else {
                        model.tracks[0].builtins.left.lpf_hz = cutoff;
                    }
                    let compiled = compile_session(
                        &model,
                        CompileCaps {
                            max_compiled_model_bytes: u64::MAX,
                            max_requested_runtime_bytes: u64::MAX,
                            max_single_allocation_bytes: u64::MAX,
                            max_queue_items: u64::MAX,
                            max_source_ring_frames: u64::MAX,
                            max_source_ring_bytes: u64::MAX,
                        },
                    )
                    .expect("launch-rate boundary session compiles");
                    prepare_session_builtins(&compiled, &[], caps())
                };
                prepare(f32::from_bits(maximum_bits)).unwrap_or_else(|error| {
                    panic!(
                        "maximum must prepare: rate={rate}, high_pass={high_pass}, error={error:?}"
                    )
                });
                let Err(successor_error) = prepare(f32::from_bits(maximum_bits + 1)) else {
                    panic!("the immediate successor must reject before coefficient preparation");
                };
                assert_eq!(
                    successor_error,
                    BuiltinDiagnosticSet(vec![diag("builtin.filter.cutoff", path)]),
                    "rate={rate}, high_pass={high_pass}"
                );
            }
        }
    }

    /// Frozen issue-034 compiler-mutation seed. This exercises complete preparation requests and
    /// their prepared block/target contract, never a timed workload.
    const BUILTIN_COMPILER_MUTATION_SEED: u64 = 0x34_007_c10_u64;
    const BUILTIN_COMPILER_MUTATION_CLASSES: usize = 49;

    #[test]
    fn deterministic_builtin_compiler_mutation_matrix_has_exactly_ten_thousand_cases() {
        let mut base_model =
            parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json"))
                .expect("parse baseline mutation session");
        base_model.tracks[0].dynamic.effects.clear();
        base_model.automation.clear();
        let base_config = MeterConfig {
            period_frames: NonZeroU32::new(16).expect("constant"),
            peak_hold_frames: 0,
            peak_decay_db_per_second: 0.0,
            queue_capacity: NonZeroUsize::new(4).expect("constant"),
            reset_generation: 0,
        };
        let baseline_request = MeterRequest {
            handle: handle(1),
            track_id: "vocal".to_owned(),
            tap: MeterTap::Input,
            config: base_config,
        };
        let accepted = compile_session(
            &base_model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compile baseline mutation session");
        let baseline =
            prepare_session_builtins(&accepted, std::slice::from_ref(&baseline_request), caps())
                .expect("baseline preparation");
        let report = baseline.resource_report();
        let mut state = BUILTIN_COMPILER_MUTATION_SEED;
        let mut transcript_hash = 0xcbf2_9ce4_8422_2325_u64;
        let mut seen_taps = BTreeSet::new();
        let mut seen_rates = BTreeSet::new();
        let mut seen_quanta = BTreeSet::new();
        let mut seen_smoothing = BTreeSet::new();
        let mut classes = [false; BUILTIN_COMPILER_MUTATION_CLASSES];
        let mut completed = 0_u32;
        for case in 0_u32..10_000 {
            // xorshift64* is intentionally local and fixed; case descriptions are mixed into a
            // transcript hash so accidental coverage drift is visible in this test.
            state ^= state >> 12;
            state ^= state << 25;
            state ^= state >> 27;
            let value = state.wrapping_mul(0x2545_f491_4f6c_dd1d);
            let class =
                usize::try_from(case).expect("u32 fits usize") % BUILTIN_COMPILER_MUTATION_CLASSES;
            classes[class] = true;
            let taps = [
                MeterTap::Input,
                MeterTap::PostInputBuiltins,
                MeterTap::PostSimd1,
                MeterTap::PostDynamic,
                MeterTap::PostSimd2PreFader,
                MeterTap::PostFader,
                MeterTap::PostMatrix,
            ];
            let tap = taps[usize::try_from(case % 7).expect("bounded tap index")];
            seen_taps.insert(tap);
            let rate = [44_100, 48_000, 88_200, 96_000]
                [usize::try_from(case % 4).expect("bounded rate index")];
            let quantum = [1, 127, 128, 255, 1_024]
                [usize::try_from(case % 5).expect("bounded quantum index")];
            let smoothing = [0, 1, 2, 127, 128, u32::MAX]
                [usize::try_from(case % 6).expect("bounded smoothing index")];
            seen_rates.insert(rate);
            seen_quanta.insert(quantum);
            seen_smoothing.insert(smoothing);

            if class == 0 {
                let invalid = include_str!("../../../fixtures/session/v1/canonical.json").replacen(
                    "\"polarity_invert\": false",
                    "\"polarity_invert\": 0.5",
                    1,
                );
                let diagnostics = parse_session_json(&invalid)
                    .expect_err("numeric boolean encoding must reject before preparation");
                let observed: Vec<_> = diagnostics
                    .diagnostics()
                    .iter()
                    .map(|diagnostic| (diagnostic.code.as_str(), diagnostic.path.to_string()))
                    .collect();
                assert_eq!(
                    observed,
                    vec![(
                        "schema.wrong_type",
                        "$.tracks[0].builtins.left.polarity_invert".to_owned()
                    )]
                );
                for byte in
                    format!("case={case};class={class};invalid_boolean={observed:?};seed={value}")
                        .bytes()
                {
                    transcript_hash ^= u64::from(byte);
                    transcript_hash = transcript_hash.wrapping_mul(0x100_0000_01b3);
                }
                completed = completed.checked_add(1).expect("fixed case count");
                continue;
            }

            let mut model = base_model.clone();
            model.sample_rate_hz = rate;
            model.quantum_frames = quantum;
            model.tracks[0].matrix_or_pan = MatrixOrPan::Matrix {
                ll: 1.0,
                lr: 0.0,
                rl: 0.0,
                rr: 1.0,
                smoothing_samples: smoothing,
            };
            let mut requests = vec![MeterRequest {
                handle: handle(1),
                track_id: "vocal".to_owned(),
                tap,
                config: base_config,
            }];
            let mut mutation_caps = caps();
            let mut expected = Vec::new();
            let mut expected_session = Vec::new();
            let mut target = Matrix2x2::IDENTITY;
            let mut block_probe = 0_u8;
            match class {
                1 => model.tracks[0].builtins.left.polarity_invert = value & 1 != 0,
                2 => model.tracks[0].builtins.left.trim_db = -144.0,
                3 => model.tracks[0].builtins.right.trim_db = 24.0,
                4 => {
                    model.tracks[0].builtins.left.trim_db = f32::NAN;
                    expected_session.push((
                        "numeric.non_finite",
                        "$.tracks[0].builtins.left.trim_db".to_owned(),
                    ));
                }
                5 => {
                    model.tracks[0].fader.right_db = 24.001;
                    expected.push(diag(
                        "builtin.gain.domain",
                        "$.tracks[id=vocal].fader.right_db",
                    ));
                }
                6 => model.tracks[0].builtins.left.hpf_hz = 0.0,
                7 => model.tracks[0].builtins.left.hpf_hz = 10.0,
                8 => {
                    model.tracks[0].builtins.left.hpf_hz = rate as f32 / 2.0;
                    model.tracks[0].builtins.left.lpf_hz = 0.0;
                    expected.push(diag(
                        "builtin.filter.cutoff",
                        "$.tracks[id=vocal].builtins.left.hpf_hz",
                    ));
                }
                9 => {
                    model.tracks[0].builtins.left.hpf_hz = 1_000.0;
                    model.tracks[0].builtins.left.lpf_hz = 100.0;
                    expected.push(diag(
                        "builtin.filter.order",
                        "$.tracks[id=vocal].builtins.left.lpf_hz",
                    ));
                }
                10 => {
                    if let MatrixOrPan::Matrix { ll, .. } = &mut model.tracks[0].matrix_or_pan {
                        *ll = -1.0;
                    }
                }
                11 => {
                    if let MatrixOrPan::Matrix { rr, .. } = &mut model.tracks[0].matrix_or_pan {
                        *rr = 1.001;
                    }
                    expected.push(diag(
                        "builtin.matrix.coefficient",
                        "$.tracks[id=vocal].matrix_or_pan.rr",
                    ));
                }
                12 => {
                    requests.push(MeterRequest {
                        handle: handle(1),
                        track_id: "vocal".to_owned(),
                        tap: if tap == MeterTap::Input {
                            MeterTap::PostMatrix
                        } else {
                            MeterTap::Input
                        },
                        config: base_config,
                    });
                    expected.push(diag(
                        "builtin.meter.duplicate_handle",
                        &meter_path(&requests[1]),
                    ));
                }
                13 => {
                    requests.push(MeterRequest {
                        handle: handle(2),
                        ..requests[0].clone()
                    });
                    expected.push(diag("builtin.meter.duplicate", &meter_path(&requests[1])));
                }
                14 => {
                    requests[0].track_id = "unknown-track".to_owned();
                    expected.push(diag(
                        "builtin.meter.unknown_track",
                        &meter_path(&requests[0]),
                    ));
                }
                15 => requests[0].config.period_frames = NonZeroU32::new(1).expect("constant"),
                16 => {
                    requests[0].config.period_frames = NonZeroU32::new(u32::MAX).expect("constant")
                }
                17 => {
                    requests[0].config.period_frames = NonZeroU32::new(128).expect("constant");
                    mutation_caps.maximum_period_frames = 127;
                    expected.push(diag("builtin.resource.limit", &meter_path(&requests[0])));
                }
                18 => requests[0].config.peak_hold_frames = u32::MAX,
                19 => {
                    requests[0].config.peak_hold_frames = 128;
                    mutation_caps.maximum_peak_hold_frames = 127;
                    expected.push(diag("builtin.resource.limit", &meter_path(&requests[0])));
                }
                20 => requests[0].config.peak_decay_db_per_second = 120.0,
                21 => {
                    requests[0].config.peak_decay_db_per_second = f32::NAN;
                    expected.push(diag("builtin.meter.config", &meter_path(&requests[0])));
                }
                22 => requests[0].config.reset_generation = u64::MAX,
                23 => {
                    requests[0].config.queue_capacity =
                        NonZeroUsize::new(usize::MAX).expect("constant");
                    expected.push(diag(
                        "builtin.resource.arithmetic_overflow",
                        &meter_path(&requests[0]),
                    ));
                }
                24 => {
                    target = Matrix2x2 {
                        ll: -1.0,
                        lr: 1.0,
                        rl: 1.0,
                        rr: -1.0,
                    }
                }
                25 => target.ll = f32::NAN,
                26 => target.lr = f32::INFINITY,
                27 => target.rr = 1.001,
                28 => block_probe = 1,
                29 => block_probe = 2,
                30 => block_probe = 3,
                31 => block_probe = 4,
                32 => {
                    mutation_caps.maximum_total_state_bytes =
                        report.engine_owned_processor_payload_bytes
                }
                33 => {
                    mutation_caps.maximum_total_state_bytes = report
                        .engine_owned_processor_payload_bytes
                        .checked_sub(1)
                        .expect("nonzero");
                    expected.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
                }
                34 => {
                    mutation_caps.maximum_total_retained_payload_bytes =
                        report.engine_owned_retained_payload_bytes
                }
                35 => {
                    mutation_caps.maximum_total_retained_payload_bytes = report
                        .engine_owned_retained_payload_bytes
                        .checked_sub(1)
                        .expect("nonzero");
                    expected.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
                }
                36 => {
                    mutation_caps.maximum_total_meter_bytes =
                        report.engine_owned_meter_payload_bytes
                }
                37 => {
                    mutation_caps.maximum_total_meter_bytes = report
                        .engine_owned_meter_payload_bytes
                        .checked_sub(1)
                        .expect("nonzero");
                    expected.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
                }
                38 => {
                    mutation_caps.maximum_single_allocation_bytes =
                        report.maximum_single_allocation_bytes
                }
                39 => {
                    mutation_caps.maximum_single_allocation_bytes = report
                        .maximum_single_allocation_bytes
                        .checked_sub(1)
                        .expect("nonzero");
                    expected.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
                }
                40 => mutation_caps.maximum_total_meter_items = report.meter_items,
                41 => {
                    mutation_caps.maximum_total_meter_items =
                        report.meter_items.checked_sub(1).expect("nonzero");
                    expected.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
                }
                42 => mutation_caps.maximum_meter_streams = 1,
                43 => {
                    requests.push(MeterRequest {
                        handle: handle(2),
                        track_id: "vocal".to_owned(),
                        tap: if tap == MeterTap::Input {
                            MeterTap::PostMatrix
                        } else {
                            MeterTap::Input
                        },
                        config: base_config,
                    });
                    mutation_caps.maximum_meter_streams = 1;
                    expected.push(diag("builtin.resource.limit", "$.meter_requests"));
                }
                44 => {
                    let limited = if smoothing == 0 { 0 } else { smoothing - 1 };
                    mutation_caps.maximum_smoothing_samples = limited;
                    if smoothing > 0 {
                        expected.push(diag(
                            "builtin.matrix.smoothing",
                            "$.tracks[id=vocal].matrix_or_pan.smoothing_samples",
                        ));
                    }
                }
                45 => {
                    let coefficient = (u32::try_from(value >> 32).expect("masked") as f32
                        / u32::MAX as f32)
                        * 2.0
                        - 1.0;
                    if let MatrixOrPan::Matrix { lr, rl, .. } = &mut model.tracks[0].matrix_or_pan {
                        *lr = coefficient;
                        *rl = -coefficient;
                    }
                }
                46 => {
                    let maximum = builtin_filter_cutoff_maximum_hz(rate)
                        .expect("matrix only uses launch rates");
                    let successor = f32::from_bits(maximum.to_bits() + 1);
                    model.tracks[0].builtins.left.hpf_hz = 0.0;
                    model.tracks[0].builtins.left.lpf_hz = 0.0;
                    match case % 4 {
                        0 => model.tracks[0].builtins.left.hpf_hz = maximum,
                        1 => model.tracks[0].builtins.left.lpf_hz = maximum,
                        2 => {
                            model.tracks[0].builtins.left.hpf_hz = successor;
                            expected.push(diag(
                                "builtin.filter.cutoff",
                                "$.tracks[id=vocal].builtins.left.hpf_hz",
                            ));
                        }
                        3 => {
                            model.tracks[0].builtins.left.lpf_hz = successor;
                            expected.push(diag(
                                "builtin.filter.cutoff",
                                "$.tracks[id=vocal].builtins.left.lpf_hz",
                            ));
                        }
                        _ => unreachable!("case remainder is bounded"),
                    }
                }
                47 => target.rl = f32::NEG_INFINITY,
                48 => {
                    model.tracks[0].builtins.right.lpf_hz = f32::NAN;
                    expected_session.push((
                        "numeric.non_finite",
                        "$.tracks[0].builtins.right.lpf_hz".to_owned(),
                    ));
                }
                _ => unreachable!("frozen class range"),
            }

            let compiled_result = compile_session(
                &model,
                CompileCaps {
                    max_compiled_model_bytes: u64::MAX,
                    max_requested_runtime_bytes: u64::MAX,
                    max_single_allocation_bytes: u64::MAX,
                    max_queue_items: u64::MAX,
                    max_source_ring_frames: u64::MAX,
                    max_source_ring_bytes: u64::MAX,
                },
            );
            if !expected_session.is_empty() {
                let Err(diagnostics) = compiled_result else {
                    panic!("invalid complete session must reject: case={case}, class={class}");
                };
                let observed: Vec<_> = diagnostics
                    .diagnostics()
                    .iter()
                    .map(|diagnostic| (diagnostic.code.as_str(), diagnostic.path.to_string()))
                    .collect();
                assert_eq!(observed, expected_session, "case={case}, class={class}");
                for byte in format!(
                    "case={case};class={class};seed={value};rate={rate};quantum={quantum};tap={tap:?};smoothing={smoothing};session={observed:?}"
                )
                .bytes()
                {
                    transcript_hash ^= u64::from(byte);
                    transcript_hash = transcript_hash.wrapping_mul(0x100_0000_01b3);
                }
                completed = completed.checked_add(1).expect("fixed case count");
                continue;
            }
            let compiled = compiled_result
                .expect("complete generated session compiles before builtin preparation");
            let expected = BuiltinDiagnosticSet::sorted(expected);
            let result = prepare_session_builtins(&compiled, &requests, mutation_caps);
            if !expected.0.is_empty() {
                let Err(observed) = result else {
                    panic!("frozen invalid class must reject: case={case}, class={class}");
                };
                assert_eq!(observed, expected, "case={case}, class={class}");
            } else {
                let prepared = match result {
                    Ok(prepared) => prepared,
                    Err(error) => {
                        panic!(
                            "frozen valid preparation class failed: case={case}, class={class}, error={error:?}"
                        )
                    }
                };
                let accepted_report = prepared.resource_report();
                assert!(
                    accepted_report.engine_owned_processor_payload_bytes
                        <= mutation_caps.maximum_total_state_bytes
                );
                assert!(
                    accepted_report.engine_owned_retained_payload_bytes
                        <= mutation_caps.maximum_total_retained_payload_bytes
                );
                assert!(
                    accepted_report.engine_owned_meter_payload_bytes
                        <= mutation_caps.maximum_total_meter_bytes
                );
                assert!(
                    accepted_report.maximum_single_allocation_bytes
                        <= mutation_caps.maximum_single_allocation_bytes
                );
                assert_eq!(
                    accepted_report
                        .retained_layouts()
                        .iter()
                        .try_fold(0_u64, |total, layout| {
                            total.checked_add(
                                layout.size_bytes.checked_mul(layout.allocation_count)?,
                            )
                        }),
                    Some(accepted_report.engine_owned_retained_payload_bytes)
                );

                let parameters = track_parameters(&model.tracks[0], u32::MAX)
                    .expect("accepted class parameters");
                let mut chain = BuiltinChain::new(rate, parameters).expect("accepted chain");
                let target_result = chain.set_matrix_target(target);
                if matches!(class, 25 | 26 | 27 | 47) {
                    assert_eq!(target_result, Err(BuiltinParameterError::MatrixCoefficient));
                } else {
                    target_result.expect("valid target");
                }
                let frames = usize::try_from(quantum).expect("supported quantum fits usize");
                match block_probe {
                    1 => {
                        let mut left = Vec::<f32>::new();
                        let mut right = Vec::<f32>::new();
                        assert!(matches!(
                            DualMonoBlock::new(&mut left, &mut right, 0),
                            Err(BuiltinParameterError::EmptyBlock)
                        ));
                    }
                    2 => {
                        let mut left = vec![0.0_f32; frames];
                        let mut right = vec![0.0_f32; frames.checked_sub(1).expect("nonzero")];
                        assert!(matches!(
                            DualMonoBlock::new(&mut left, &mut right, 0),
                            Err(BuiltinParameterError::LaneLength)
                        ));
                    }
                    3 => {
                        let mut left = [0.0_f32];
                        let mut right = [0.0_f32];
                        assert!(matches!(
                            DualMonoBlock::new(&mut left, &mut right, u64::MAX),
                            Err(BuiltinParameterError::SampleTimeOverflow)
                        ));
                    }
                    4 => {
                        let PreparedMeter {
                            mut accumulator, ..
                        } = MeterAccumulator::prepare(handle(99), base_config, rate)
                            .expect("valid discontinuity meter");
                        accumulator.observe(&[0.0], &[0.0], 0).expect("first block");
                        accumulator
                            .observe(&[0.0], &[0.0], 100)
                            .expect("discontinuous block is bounded and accepted");
                    }
                    _ => {
                        let mut left = vec![0.0_f32; frames];
                        let mut right = vec![0.0_f32; frames];
                        let block = DualMonoBlock::new(&mut left, &mut right, u64::from(case))
                            .expect("valid generated block");
                        chain.process_dual_mono(block);
                    }
                }
            }

            let description = format!(
                "case={case};class={class};seed={value};rate={rate};quantum={quantum};tap={tap:?};smoothing={smoothing};handle={};period={};hold={};decay={:08x};reset={};queue={};caps={mutation_caps:?};expected={:?};target={:08x},{:08x},{:08x},{:08x};block={block_probe}",
                requests[0].handle.0,
                requests[0].config.period_frames,
                requests[0].config.peak_hold_frames,
                requests[0].config.peak_decay_db_per_second.to_bits(),
                requests[0].config.reset_generation,
                requests[0].config.queue_capacity,
                expected.0,
                target.ll.to_bits(),
                target.lr.to_bits(),
                target.rl.to_bits(),
                target.rr.to_bits(),
            );
            for byte in description.bytes() {
                transcript_hash ^= u64::from(byte);
                transcript_hash = transcript_hash.wrapping_mul(0x100_0000_01b3);
            }
            completed = completed.checked_add(1).expect("fixed case count");
        }
        assert_eq!(completed, 10_000);
        assert_eq!(seen_taps.len(), 7);
        assert_eq!(seen_rates, BTreeSet::from([44_100, 48_000, 88_200, 96_000]));
        assert_eq!(seen_quanta, BTreeSet::from([1, 127, 128, 255, 1_024]));
        assert_eq!(
            seen_smoothing,
            BTreeSet::from([0, 1, 2, 127, 128, u32::MAX])
        );
        assert!(classes.into_iter().all(core::convert::identity));
        // Moved by issue #212, and only through the resource dimension. Classes 32-35 derive
        // their caps *from* `report.engine_owned_processor_payload_bytes`, so the reported payload
        // is part of every case's description -- and that payload moved by exactly +48 bytes per
        // track: two `GraphNodeBinding`s (2 x 72) and the boxed `FaderProcessor` (16) and
        // `MatrixProcessor` (136) left preparation, and the 344-byte `StripPreparation` vector
        // entry replaced them. 906 -> 954 at one track, and one fewer allocation (19 -> 18).
        //
        // Moved again by #210 phase 3, through the same one dimension, by **+171 bytes per track**
        // on a console-free preparation -- 954 -> 1_125 on this one-track fixture. Every term is a
        // `size_of` and they sum exactly:
        //
        // | term | bytes |
        // |---|---|
        // | the `GraphNodeBinding` vector leaves preparation entirely | -72 |
        // | the boxed `InputProcessor` leaves with it | -168 |
        // | one fewer clone of the track ID (ten copies became nine) | -5 (`"vocal"`) |
        // | `StripPreparation` gains the input section and a third console consumer, 344 -> 656 | +312 |
        // | the `bank_inputs` vector entry grows with `InputBuiltins`, 168 -> 272 | +104 |
        //
        // `InputBuiltins` grew by 104 because `InputStage<f32>` gained the trim ramp: four `f32`
        // ramp words per channel (32), the authoritative `[[u32; 8]; 2]` countdown (64), and the
        // `ramping` flag with its padding (8).
        //
        // A *console-leased* preparation moves by +575 per controlled track at depth 8 instead:
        // the same +171, plus 40 for the wider `TrackControlProducer` vector entry (96 -> 136, the
        // third producer) and 364 for the third bounded ring -- a 256-byte header at 64-byte
        // alignment plus 108 bytes of slot payload, which is byte-for-byte what the fader ring
        // costs, because `TrackInputRecord` and `TrackFaderRecord` are both 12 bytes. 1_884 ->
        // 2_459 on this fixture with one depth-8 channel. `maximum_single_allocation_bytes` moves
        // 344 -> 656 with `StripPreparation`, which is the largest single allocation at one track.
        //
        // Nothing else in the transcript moved: `expected` is declared per frozen class rather
        // than read off the report, and the boundary classes stay exact because they are stated
        // relative to the report rather than as literals -- case 32 admits at the payload and case
        // 33 rejects one byte below it, whatever the payload is.
        assert_eq!(
            transcript_hash, 4_741_579_849_300_275_697,
            "updated only through a deliberate frozen-case change"
        );
    }

    /// The per-node console input processor: the arm a scalar-backend host binds (#210 phase 3).
    ///
    /// `Backend::current()` is a compile-time constant, so on every architecture the workspace's
    /// tests run on the strip is banked and `ConsoleInputProcessor` is unreachable from an
    /// end-to-end fixture. It is not unreachable in *production* -- a target with no SIMD binds
    /// it, and so does any lowering the planner leaves unbanked -- so it is driven directly here,
    /// through the same `GraphRuntimeProcessor::process` the runtime calls.
    ///
    /// The two facts it owes, and the two the banked drain owes for the same reason: the record
    /// reaches the section, and the witness records what the record did.
    ///
    /// Red mutation: make the drain loop `continue` past every record without applying it -> the
    /// coefficient never moves. Red mutation: drop `self.live.admit(&record)` -> the per-lane arm
    /// keeps claiming symmetry.
    #[test]
    fn the_per_node_console_input_processor_drains_and_folds_its_witness() {
        let parameters = BuiltinParameters {
            left: ChannelParameters {
                trim_db: 0.0,
                ..ChannelParameters::default()
            },
            right: ChannelParameters {
                trim_db: 0.0,
                ..ChannelParameters::default()
            },
            ..BuiltinParameters::default()
        };
        let build = || {
            let input = BuiltinChain::new(48_000, parameters)
                .expect("chain")
                .into_input_builtins();
            let (producer, control) = bounded_spsc::<TrackInputRecord>(
                NonZeroUsize::new(8).expect("depth"),
                QueueGeneration(0),
            )
            .expect("queue");
            (
                producer,
                ConsoleInputProcessor {
                    input,
                    control,
                    live: ChannelSymmetryWitness::SYMMETRIC,
                },
            )
        };

        // A `Both` retarget moves the coefficient and preserves every term.
        let (mut producer, mut processor) = build();
        producer
            .try_push(TrackInputRecord::TrimDb {
                lanes: BuiltinLaneSelector::Both,
                db: -144.0,
                smoothing_samples: 0,
            })
            .expect("room");
        let mut left = [1.0_f32; 4];
        let mut right = [1.0_f32; 4];
        processor
            .process(GraphBindingBlock {
                left: &mut left,
                right: &mut right,
                first_sample: 0,
            })
            .expect("render");
        assert!(
            left.iter().all(|value| *value < 1.0e-6) && right.iter().all(|value| *value < 1.0e-6),
            "the record reached the section: {left:?} {right:?}"
        );
        assert!(
            processor.channel_symmetry().eligible(),
            "a `Both` retarget is symmetry-preserving on the per-node arm too"
        );

        // A per-lane retarget declines the `LIVE` term and moves one plane only.
        let (mut producer, mut processor) = build();
        producer
            .try_push(TrackInputRecord::PolarityInvert {
                lanes: BuiltinLaneSelector::Right,
                inverted: true,
                smoothing_samples: 0,
            })
            .expect("room");
        let mut left = [1.0_f32; 4];
        let mut right = [1.0_f32; 4];
        processor
            .process(GraphBindingBlock {
                left: &mut left,
                right: &mut right,
                first_sample: 0,
            })
            .expect("render");
        assert!(
            left.iter().all(|value| *value > 0.0),
            "the left plane is untouched"
        );
        assert!(
            right.iter().all(|value| *value < 0.0),
            "the right plane is inverted"
        );
        assert!(
            !processor
                .channel_symmetry()
                .holds(ChannelSymmetryWitness::LIVE),
            "a one-channel write clears the LIVE term on the per-node arm"
        );
    }

    /// Issue #212: a prepared session that lost a *strip* is caught, not only one that lost a
    /// binding.
    ///
    /// Preparation owns three stages per track in two places -- one `GraphNodeBinding` for the
    /// post-input stage, and one `StripPreparation` carrying the fader and the matrix -- while the
    /// seal still records three stages per track flat. So `processors_match` has to count both,
    /// and this is the half that did not exist before the strip's binding form became a lowering
    /// decision. Without the strip term, a session missing a whole track's fader and matrix would
    /// validate clean.
    #[test]
    fn a_prepared_session_that_lost_a_strip_fails_validation() {
        let session = session();
        let mut prepared =
            prepare_session_builtins(&session, &[], caps()).expect("prepared builtins");
        assert!(
            prepared.validate_for_session(&session).0.is_empty(),
            "the untouched preparation validates"
        );
        assert!(prepared.strips.pop().is_some(), "the fixture has a strip");
        let diagnostics = prepared.validate_for_session(&session);
        assert!(
            diagnostics
                .0
                .iter()
                .any(|diagnostic| diagnostic.code == "builtin.prepared.processor_set"),
            "a lost strip must be diagnosed as a processor-set mismatch, got {diagnostics:?}"
        );
    }

    /// Issue #137 D1: a live-console control request is validated like a meter request, sealed
    /// like a meter consumer, and charges only the tracks that asked for one.
    ///
    /// Red mutation: delete the `control_tracks.insert` / `known_tracks.contains` legs in
    /// `prepare_session_builtins_with_console` -> the duplicate and unknown-track requests are
    /// accepted, and the assertions below on `builtin.control.duplicate` /
    /// `builtin.control.unknown_track` fail with an `Ok` preparation.
    #[test]
    fn console_control_requests_are_validated_sealed_and_charged_per_track() {
        let compiled = session();
        let track = compiled.normalized_model().tracks[0].id.as_str().to_owned();
        let depth = NonZeroUsize::new(3).expect("nonzero");

        let baseline = prepare_session_builtins(&compiled, &[], caps()).expect("baseline");
        let attached = prepare_session_builtins_with_console(
            &compiled,
            &[],
            &[TrackControlRequest {
                track_id: track.clone(),
                queue_capacity: depth,
            }],
            caps(),
        )
        .expect("attached console");
        assert_eq!(baseline.track_control_count(), 0);
        assert_eq!(attached.track_control_count(), 1);
        assert!(
            attached
                .resource_report()
                .engine_owned_processor_payload_bytes
                > baseline
                    .resource_report()
                    .engine_owned_processor_payload_bytes,
            "an attached channel is charged"
        );
        assert!(
            attached.validate_for_session(&compiled).0.is_empty(),
            "the control seal matches the producers it was built from"
        );
        assert_eq!(
            attached.processor_count(),
            baseline.processor_count(),
            "a console changes no processor count, only one processor's type"
        );

        let duplicate = prepare_session_builtins_with_console(
            &compiled,
            &[],
            &[
                TrackControlRequest {
                    track_id: track.clone(),
                    queue_capacity: depth,
                },
                TrackControlRequest {
                    track_id: track.clone(),
                    queue_capacity: depth,
                },
            ],
            caps(),
        )
        .map(|_| ())
        .expect_err("a track may hold one control channel");
        assert!(
            duplicate
                .0
                .iter()
                .any(|value| value.code == "builtin.control.duplicate"),
            "{duplicate:?}"
        );

        let unknown = prepare_session_builtins_with_console(
            &compiled,
            &[],
            &[TrackControlRequest {
                track_id: "no-such-track".to_owned(),
                queue_capacity: depth,
            }],
            caps(),
        )
        .map(|_| ())
        .expect_err("an undeclared track has no channel");
        assert!(
            unknown
                .0
                .iter()
                .any(|value| value.code == "builtin.control.unknown_track"),
            "{unknown:?}"
        );
    }
}
