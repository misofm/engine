//! The live-console control seam: admitted records in, prepared automation spans out.
//!
//! # Why this lives in the contract crate
//!
//! Issue #137 shipped the whole `miso.command.v1` ABI and discovered that no live parameter path
//! reaches a running plan: `miso_engine_graph::runtime` handed every effect a hardcoded empty
//! automation slice and `miso_engine_rack::EffectBankStage` handed every bank an empty one with
//! zero offsets. #140 closes that gap by feeding the admitted commands in as
//! [`PreparedAutomationSpan`]s -- which every effect's `process` already honours -- rather than by
//! inventing a second parameter path.
//!
//! Two render paths need exactly the same staging: the per-node dynamic rack (`miso-engine-graph`)
//! and the AoSoA SIMD racks (`miso-engine-rack`). Neither crate depends on the other's private
//! internals, and both already depend on this one, so the staging is written **once**, here, and
//! the two racks differ only in how many lanes they stage.
//!
//! # The frozen application rule
//!
//! A drained record takes effect at the **first sample of the next rendered block**, exactly as
//! #137's matrix retarget does: [`EffectControlLane::stage`] runs at the top of the block, before
//! a single sample is touched, and emits `AutomationSpanKind::Point` spans whose `start_sample`
//! and `end_sample` are that block's `first_sample`. Every launch effect accepts precisely that
//! shape (a `Point` at `first_sample` with bit-identical endpoints) and rejects anything else into
//! `ProcessReport::invalid_spans`, so the block boundary is proven by the effect contract rather
//! than asserted by the host.
//!
//! # Allocation-free by construction
//!
//! Every buffer here is sized at prepare from the effect's own
//! [`PreparedEffectMetadata::automation_capacity`](crate::PreparedEffectMetadata). Draining moves
//! `Copy` records out of a bounded queue into that buffer; there is no allocation, no lock, no
//! drop and no unbounded loop -- the drain is bounded by the queue capacity, which preparation
//! refuses to make larger than the automation capacity.

use miso_engine_core::realtime::{
    Consumer, ObservationPublisherV1, ObservationWindowV1, observation_slot_retained_bytes,
};
use miso_engine_lane::kernels::pdc_delay_block;

use crate::{
    AutomationSpanKind, ChannelSymmetryWitnessV1, ObservationDescriptorV1, ObservationFoldV1,
    ObservationSampleV1, ParameterChannel, PreparedAutomationSpan,
};

/// One admitted, still-unapplied live control event for one prepared effect instance.
///
/// It is `Copy` and fixed-size so the channel is a plain
/// [`bounded_spsc`](miso_engine_core::realtime::bounded_spsc) and the render-side drain allocates
/// nothing. Addressing is *not* carried: a channel belongs to exactly one effect instance (or, in
/// a bank, to exactly one lane of one slot), so the record only says what to change.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EffectControlRecordV1 {
    /// Retarget one declared parameter of one channel.
    ///
    /// `parameter_index` is the index into `EffectDescriptorV1::parameters`, never the wire
    /// `parameter_id`: the translation is an admission-time lookup, off the render thread.
    /// `channel` already obeys the parameter's
    /// [`ParameterChannelPolicy`](crate::ParameterChannelPolicy) -- a `Shared` parameter arrives as
    /// [`ParameterChannel::Both`] and a `PerLane` parameter arrives as one record per lane, because
    /// every launch effect counts a policy-violating span as invalid rather than applying it.
    Parameter {
        /// Index into the descriptor's parameter table.
        parameter_index: u32,
        /// Which lane this value addresses.
        channel: ParameterChannel,
        /// The new value, already domain-checked by the admitting host.
        value: f32,
    },
    /// Arm or disarm one declared observation tap (issue #143 D3, level 2).
    ///
    /// It rides this queue rather than a queue of its own for one reason: a subscription and a
    /// parameter command that arrive in the same submission are drained by the same call at the
    /// top of the same block, so STATE and IMPACT land on one sample timeline **by construction**
    /// rather than by two clocks agreeing. `tap_index` is the index into the descriptor's
    /// observation table, never the wire `tap_id`; `window_blocks` of `0` means "the plan's
    /// default".
    Observe {
        /// Index into the descriptor's observation table.
        tap_index: u32,
        /// Whether the tap is to be read at all after this block boundary.
        armed: bool,
        /// Render blocks per published window, or `0` for the plan's default.
        window_blocks: u32,
    },
    /// Set this instance's live bypass.
    ///
    /// Bypass is not a parameter and has no span: it is applied by the rack's latency-preserving
    /// shunt (see [`BypassShunt`]), never by the effect, because
    /// [`PreparedEffectMetadata::bypass`](crate::PreparedEffectMetadata) is baked into the
    /// program key and a bank's lanes must be able to disagree about it.
    Bypass(bool),
}

/// The sort key the contract's canonical span order uses, for spans that share a start sample.
///
/// [`validate_automation_block`](crate::validate_automation_block) orders by
/// `(start_sample, parameter_index, channel)`; every span this module stages carries the same
/// `start_sample`, so the key collapses to this pair. A parameter's channel policy is fixed, so a
/// single parameter never mixes [`ParameterChannel::Both`] with the per-lane values and the order
/// this produces is exactly the strictly-increasing order every effect's own validator demands.
const fn order_key(parameter_index: u32, channel: ParameterChannel) -> (u32, u32) {
    (parameter_index, channel as u32)
}

/// One prepared live-control channel for one effect instance, or one lane of one bank slot.
///
/// The consumer half of a [`bounded_spsc`](miso_engine_core::realtime::bounded_spsc); the producer
/// stays with the host's control plane. A producer must be dropped before the plan that owns this.
pub struct EffectControlLane {
    control: Consumer<EffectControlRecordV1>,
    /// Live bypass state, retained across blocks so a rendered block always knows it.
    bypass: bool,
    /// This instance's live channel-symmetry terms, retained across blocks exactly as `bypass`
    /// is, and for the same reason: the terms describe what the *drained* records did, so they
    /// have to survive the block that drained them.
    ///
    /// # Why the witness is maintained here and not at the host's admission call
    ///
    /// The queue **is** the admission boundary. A record is admitted into the rendered state at
    /// the drain, on the render thread, at the top of the block that first applies it -- so a
    /// witness folded in here cannot disagree with the state it describes, and it needs no
    /// atomic and no second channel to reach the collapse dispatch. A host-side bit could not:
    /// `PreparedRenderPlan` is `Send` and **not** `Sync`, so a value the control thread owns is
    /// unreadable from the render thread by construction, and the record it would have been
    /// derived from is already crossing this queue.
    ///
    /// Only the two live terms move here. `SOURCE`, `DESIGNED` and `RESTORED` are decided off
    /// render, at preparation and at restore, and are conjoined by the stage that owns this lane.
    symmetry: ChannelSymmetryWitnessV1,
}

impl EffectControlLane {
    /// Binds the consumer half of one prepared channel.
    #[must_use]
    pub fn new(control: Consumer<EffectControlRecordV1>, bypass: bool) -> Self {
        let mut symmetry = ChannelSymmetryWitnessV1::SYMMETRIC;
        symmetry.set(ChannelSymmetryWitnessV1::UNBYPASSED, !bypass);
        Self {
            control,
            bypass,
            symmetry,
        }
    }

    /// This lane's live channel-symmetry terms as of the last [`stage`](Self::stage).
    ///
    /// `LIVE` and `UNBYPASSED` are the only terms this value speaks to; the other three are set,
    /// so conjoining it with the stage's prepared witness gives the whole answer and never
    /// over-claims.
    #[must_use]
    pub const fn symmetry(&self) -> ChannelSymmetryWitnessV1 {
        self.symmetry
    }

    /// Whether this instance is bypassed as of the last [`stage`](Self::stage).
    #[must_use]
    pub const fn bypassed(&self) -> bool {
        self.bypass
    }

    /// The bounded capacity of the underlying queue.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.control.capacity()
    }

    /// Drain every queued record into `staging`, in canonical span order, and return the count.
    ///
    /// `observation` is this instance's tap state when the plan is observation-capable and `None`
    /// otherwise. An [`EffectControlRecordV1::Observe`] emits **no span**: it changes what is read
    /// after the block, never what the block renders, so it does not touch the staging window and
    /// cannot make it overflow. A plan with no observation capacity applies nothing and reports the
    /// record as refused, which is what the control plane turns into `ObservationUnbound`.
    ///
    /// `staging` is the caller's preallocated window for this lane. Records are collapsed
    /// last-wins per `(parameter_index, channel)` and inserted in canonical order, so the emitted
    /// slice is already the strictly increasing, non-overlapping block the effect contract
    /// requires -- a caller never has to sort or deduplicate on the render thread.
    ///
    /// A record that cannot fit (`staging` full of *distinct* targets) is dropped and counted in
    /// the returned overflow. Preparation makes that unreachable by refusing a queue deeper than
    /// the effect's automation capacity; the count exists so a violated invariant is observable
    /// rather than silent.
    pub fn stage(
        &mut self,
        staging: &mut [PreparedAutomationSpan],
        first_sample: u64,
        observation: Option<&mut ObservationLaneV1>,
    ) -> Staged {
        let mut staged = 0_usize;
        let mut dropped = 0_u32;
        let mut unbound = 0_u32;
        let mut observation = observation;
        while let Ok(record) = self.control.try_pop() {
            // The one hook. `admit` takes the record by trait, not by kind, so a record type
            // added to this queue later cannot reach the render state without declaring what it
            // does to the witness (`symmetry::LiveConsoleRecordV1`).
            self.symmetry.admit(&record);
            let (parameter_index, channel, value) = match record {
                EffectControlRecordV1::Bypass(value) => {
                    self.bypass = value;
                    continue;
                }
                EffectControlRecordV1::Observe {
                    tap_index,
                    armed,
                    window_blocks,
                } => {
                    let applied = observation.as_deref_mut().is_some_and(|lane| {
                        lane.arm(tap_index, armed, window_blocks, first_sample)
                    });
                    if !applied {
                        unbound = unbound.saturating_add(1);
                    }
                    continue;
                }
                EffectControlRecordV1::Parameter {
                    parameter_index,
                    channel,
                    value,
                } => (parameter_index, channel, value),
            };
            let key = order_key(parameter_index, channel);
            // Bounded linear placement over the already-sorted window: at most `staging.len()`
            // comparisons, which preparation bounds by the effect's automation capacity.
            let mut position = staged;
            let mut replace = false;
            for (index, span) in staging[..staged].iter().enumerate() {
                let existing = order_key(span.parameter_index, span.channel);
                if existing == key {
                    position = index;
                    replace = true;
                    break;
                }
                if existing > key {
                    position = index;
                    break;
                }
            }
            let span = PreparedAutomationSpan {
                kind: AutomationSpanKind::Point,
                channel,
                parameter_index,
                start_sample: first_sample,
                end_sample: first_sample,
                start_value: value,
                end_value: value,
            };
            if replace {
                staging[position] = span;
                continue;
            }
            if staged == staging.len() {
                dropped = dropped.saturating_add(1);
                continue;
            }
            staging[position..=staged].rotate_right(1);
            staging[position] = span;
            staged += 1;
        }
        Staged {
            staged,
            dropped,
            unbound,
        }
    }
}

/// What one [`EffectControlLane::stage`] call produced.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Staged {
    /// Spans written to the front of the staging window, in canonical order.
    pub staged: usize,
    /// Records refused because the window was full of distinct targets. Zero by construction.
    pub dropped: u32,
    /// [`EffectControlRecordV1::Observe`] records this plan had no capacity to apply. Zero by
    /// construction: the control plane refuses them before they reach a queue.
    pub unbound: u32,
}

/// One declared tap's arm state and open window, for one prepared instance or one bank lane.
///
/// Sixteen bytes of accumulator (`peak_left`, `peak_right`, `blocks`, `window_blocks`) plus the
/// window's own bookkeeping. All of it is allocated at preparation from the **declared menu**, so
/// arming allocates nothing and disarming frees nothing: subscribe is a flag, not a resource.
#[derive(Debug)]
struct ObservationTapV1 {
    publisher: ObservationPublisherV1,
    /// The declared fold, copied once at bind so the render path never walks the descriptor.
    fold: ObservationFoldV1,
    armed: bool,
    /// Blocks per published window; never zero once armed.
    window_blocks: u32,
    blocks: u32,
    first_sample: u64,
    end_sample: u64,
    sequence: u64,
    peak_left: f32,
    peak_right: f32,
}

/// Every declared tap of one prepared effect instance, or of one lane of one bank slot.
///
/// # The two-level zero (issue #143 D3)
///
/// Level 1 is that this type does not exist in a plan whose console request named no observation
/// capacity: there is no lane, no slot and no vector, and the render path is the byte-identical one
/// it always was. Level 2 is `ObservationTapV1::armed`: inside a capable plan, an unarmed tap's
/// effect state is never read, never folded and never stored, and the honest cost is one predicted
/// branch per driven effect per block.
#[derive(Debug)]
pub struct ObservationLaneV1 {
    taps: Box<[ObservationTapV1]>,
    default_window_blocks: u32,
    /// How many taps of this lane are armed, maintained by [`arm`](Self::arm) and
    /// [`disarm_all`](Self::disarm_all) (issue #163 phase 4 item 6).
    ///
    /// This exists so [`any_armed`](Self::any_armed) — which the doc has always described as "the
    /// one branch the block-top publish step takes" — is a single load rather than a walk over
    /// every tap. The publish gate runs on the render thread once per driven effect (or once per
    /// bank slot) per block, so a walk there would make the level-2 zero cost O(taps) exactly
    /// where #143 promises one predicted branch. `armed` on the tap stays the authority for what
    /// a tap *does*; this is only a redundant count of it, and `arm`/`disarm_all` are the only
    /// two writers of either.
    armed_count: u32,
}

impl ObservationLaneV1 {
    /// Bind one publisher per declared tap. Off the render thread, once, at preparation.
    ///
    /// `publishers` must be one per entry of `observations`, in declaration order.
    #[must_use]
    pub fn new(
        observations: &'static [ObservationDescriptorV1],
        publishers: Vec<ObservationPublisherV1>,
        default_window_blocks: u32,
    ) -> Option<Self> {
        if publishers.len() != observations.len() {
            return None;
        }
        let default_window_blocks = default_window_blocks.max(1);
        let taps: Vec<ObservationTapV1> = observations
            .iter()
            .zip(publishers)
            .map(|(descriptor, publisher)| ObservationTapV1 {
                publisher,
                fold: descriptor.fold,
                armed: false,
                window_blocks: default_window_blocks,
                blocks: 0,
                first_sample: 0,
                end_sample: 0,
                sequence: 0,
                peak_left: 0.0,
                peak_right: 0.0,
            })
            .collect();
        Some(Self {
            taps: taps.into_boxed_slice(),
            default_window_blocks,
            // Every tap is built unarmed, so the count starts where they do.
            armed_count: 0,
        })
    }

    /// Declared taps this lane carries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.taps.len()
    }

    /// Whether the lane carries no tap at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.taps.is_empty()
    }

    /// Exact engine-owned bytes this lane retains, including its slots.
    ///
    /// One tap is one accumulator row plus one shared conflating cell. Stated as a formula rather
    /// than a measured number so a row that grows has to move this line too (#143 R7).
    #[must_use]
    pub fn retained_bytes(&self) -> usize {
        self.taps.len()
            * (core::mem::size_of::<ObservationTapV1>() + observation_slot_retained_bytes())
    }

    /// Whether `tap_index` is armed right now. Off-thread introspection for the structural gates.
    #[must_use]
    pub fn is_armed(&self, tap_index: usize) -> bool {
        self.taps.get(tap_index).is_some_and(|tap| tap.armed)
    }

    /// The window length this lane would use for a request that names none.
    #[must_use]
    pub const fn default_window_blocks(&self) -> u32 {
        self.default_window_blocks
    }

    /// Apply one drained [`EffectControlRecordV1::Observe`]; `false` for an index this lane has
    /// no tap for.
    ///
    /// Arming opens a fresh window at `first_sample`, which is the first sample of the block that
    /// drained the record -- the exact `applied_at_sample` the subscription was acknowledged with.
    /// A re-arm is idempotent except that the newer `window_blocks` wins and the window restarts,
    /// so a consumer never folds two different window lengths into one reading.
    pub fn arm(
        &mut self,
        tap_index: u32,
        armed: bool,
        window_blocks: u32,
        first_sample: u64,
    ) -> bool {
        let default = self.default_window_blocks;
        let Some(tap) = usize::try_from(tap_index)
            .ok()
            .and_then(|index| self.taps.get_mut(index))
        else {
            return false;
        };
        // Maintained across the transition, not recomputed: a re-arm of an already-armed tap is
        // idempotent here exactly as it is for `armed` itself.
        match (tap.armed, armed) {
            (false, true) => self.armed_count = self.armed_count.saturating_add(1),
            (true, false) => self.armed_count = self.armed_count.saturating_sub(1),
            _ => {}
        }
        tap.armed = armed;
        tap.window_blocks = if window_blocks == 0 {
            default
        } else {
            window_blocks
        };
        tap.blocks = 0;
        tap.first_sample = first_sample;
        tap.end_sample = first_sample;
        tap.peak_left = 0.0;
        tap.peak_right = 0.0;
        true
    }

    /// Release every subscription without touching the published windows (issue #143 D7).
    ///
    /// A plan replacement drops subscriptions because the plan they addressed is gone; the app
    /// re-subscribes against the new plan. Disarming here is the same operation an explicit
    /// unsubscribe performs, so there is one code path and one meaning.
    pub fn disarm_all(&mut self) {
        self.armed_count = 0;
        for tap in self.taps.iter_mut() {
            tap.armed = false;
            tap.blocks = 0;
            tap.peak_left = 0.0;
            tap.peak_right = 0.0;
        }
    }
}

// REALTIME_POLICY_BEGIN
impl ObservationLaneV1 {
    /// Whether any tap is armed, which is the one branch the block-top publish step takes.
    ///
    /// One load and one compare, from the count [`arm`](Self::arm) and
    /// [`disarm_all`](Self::disarm_all) maintain. Wired into both publish sites by #163 phase 4
    /// item 6; before that it was `pub`, documented as the block-top gate, and called only from
    /// this crate's own tests, while the render path walked every tap instead.
    #[must_use]
    pub const fn any_armed(&self) -> bool {
        self.armed_count != 0
    }

    /// Whether tap `tap_index` should be read for this block at all.
    ///
    /// The whole of level-2 zero: one bounds check and one flag load. An unarmed tap's effect state
    /// is never touched, because this returns `false` before anything asks the effect for it.
    #[must_use]
    pub fn wants(&self, tap_index: usize) -> bool {
        match self.taps.get(tap_index) {
            Some(tap) => tap.armed,
            None => false,
        }
    }

    /// Fold one block's reading into tap `tap_index`, publishing the window if it closes here.
    ///
    /// `frames` is the block length, so `end_sample` advances by exactly what was rendered and
    /// consecutive windows tile with no gap. Called **after** `process` returns: the reading is the
    /// state at the end of the block, which is the only moment at which "resident" is true.
    pub fn accumulate(
        &mut self,
        tap_index: usize,
        sample: ObservationSampleV1,
        first_sample: u64,
        frames: u64,
    ) {
        let Some(tap) = self.taps.get_mut(tap_index) else {
            return;
        };
        if !tap.armed {
            return;
        }
        // `first_sample` is **not** taken from the block. It was set when the tap was armed and it
        // is set again to `end_sample` when a window closes, so consecutive windows tile with no
        // gap as a property of this type rather than of whatever the caller happens to pass.
        tap.end_sample = first_sample.saturating_add(frames);
        match tap.fold {
            // `max(|x|)` is what turns an effect's own negative-for-reduction convention into the
            // non-negative magnitude a meter reads. It is one `abs` and one compare per lane per
            // block, and it is the whole reason the app's `Math.max(0, x)` is a no-op rather than
            // a silent zeroing.
            ObservationFoldV1::PeakMagnitude => {
                let left = sample.left.abs();
                let right = sample.right.abs();
                if left > tap.peak_left {
                    tap.peak_left = left;
                }
                if right > tap.peak_right {
                    tap.peak_right = right;
                }
            }
            ObservationFoldV1::Latest => {
                tap.peak_left = sample.left;
                tap.peak_right = sample.right;
            }
        }
        tap.blocks = tap.blocks.saturating_add(1);
        if tap.blocks < tap.window_blocks {
            return;
        }
        tap.sequence = tap.sequence.saturating_add(1);
        tap.publisher.publish(ObservationWindowV1 {
            first_sample: tap.first_sample,
            end_sample: tap.end_sample,
            sequence: tap.sequence,
            blocks: tap.blocks,
            left: tap.peak_left,
            right: tap.peak_right,
        });
        tap.blocks = 0;
        tap.first_sample = tap.end_sample;
        tap.peak_left = 0.0;
        tap.peak_right = 0.0;
    }
}
// REALTIME_POLICY_END

/// The latency-preserving bypass shunt, applied **outside** the effect.
///
/// # Why outside
///
/// `PreparedEffectMetadata::bypass` is a *prepared* configuration: it is part of
/// [`EffectProgramKeyV1`](crate::EffectProgramKeyV1), it is byte 108 of the persisted state
/// envelope, and every effect's bank reads one flag for the whole bank. Moving it after
/// preparation would change a program key at render time -- which would re-cohort a bank -- and a
/// bank's lanes could not disagree about it anyway.
///
/// The shunt is the design the contract already documents on `EffectProgramKeyV1`, minus the parts
/// that only a per-lane kernel needs:
///
/// * **The wet path always runs.** A bypassed instance still processes, so its state stays
///   continuous and un-bypassing does not click, and a bank never re-cohorts.
/// * **Latency is preserved exactly.** The dry signal is delayed by exactly
///   `PreparedEffectMetadata::latency` -- the same integer the enabled path reports and the same
///   integer `graph-compiler` derived every route timing from -- so a bypassed instance's impulse
///   lands on the sample an enabled instance's would. PDC is unchanged by construction.
/// * **Selection is whole-block, never per sample.** #140's application rule is the block
///   boundary, so a rendered block is entirely dry or entirely wet and the select is a
///   `copy_from_slice`, not an arithmetic blend. `-0.0` survives it.
///
/// A shunt is only built for an instance that has a live control channel, so a session with no
/// console allocates none of this and renders the byte-identical path it always did.
pub struct BypassShunt {
    /// Dry copy of this block's input, taken before the effect runs.
    dry_left: Box<[f32]>,
    dry_right: Box<[f32]>,
    /// `latency` words per channel; empty when the effect reports zero latency.
    line_left: Box<[f32]>,
    line_right: Box<[f32]>,
    cursor: usize,
}

impl BypassShunt {
    /// Allocates a shunt for `frames` of block and `latency` samples of declared latency.
    ///
    /// Off the render thread: this is the only allocation the live path makes, and it happens once
    /// at plan preparation.
    #[must_use]
    pub fn new(frames: usize, latency: usize) -> Self {
        Self {
            dry_left: vec![0.0; frames].into_boxed_slice(),
            dry_right: vec![0.0; frames].into_boxed_slice(),
            line_left: vec![0.0; latency].into_boxed_slice(),
            line_right: vec![0.0; latency].into_boxed_slice(),
            cursor: 0,
        }
    }

    /// Retained bytes: the two dry planes and the two delay lines.
    #[must_use]
    pub fn retained_bytes(&self) -> usize {
        (self.dry_left.len() + self.dry_right.len() + self.line_left.len() + self.line_right.len())
            * core::mem::size_of::<f32>()
    }

    /// Whether this shunt carries a latency line that has to be fed on every block.
    ///
    /// # Why a caller needs to ask (issue #163 phase 4 item 4)
    ///
    /// [`capture`](Self::capture) does two separable things: it stages the dry block into
    /// `dry_*`, and — only when `latency > 0` — exchanges that staging through the delay line.
    /// The second is stateful and must happen on every block, bypassed or not, for the reason
    /// `capture` documents. The first is **not**: `dry_*` is read only by
    /// [`apply`](Self::apply) and [`dry`](Self::dry), both of which run later in the *same* block
    /// and only for a bypassed instance or lane. Nothing carries `dry_*` across a block boundary.
    ///
    /// So at zero latency a non-bypassed block's capture is provably dead work — two whole-block
    /// `copy_from_slice`s whose result no reader can observe — and a caller that knows it is not
    /// bypassed may skip it. At nonzero latency it may not: the staging buffer *is* the line's
    /// input, so skipping the copy would starve the line and the first bypassed block would emit
    /// stale samples. This predicate is the exact boundary between those two cases, and it is
    /// fixed at preparation rather than per block.
    #[must_use]
    pub const fn feeds_line(&self) -> bool {
        !self.line_left.is_empty()
    }

    /// Capture this block's input and advance the latency line, returning nothing.
    ///
    /// Called on every block whenever [`feeds_line`](Self::feeds_line) is true, bypassed or not:
    /// the line has to stay fed so that enabling bypass mid-stream produces the correctly delayed
    /// dry signal on its very first block rather than `latency` samples of stale zeros. When
    /// `feeds_line` is false there is no line, and a caller that is not bypassed this block may
    /// skip the call entirely (#163 phase 4 item 4).
    pub fn capture(&mut self, left: &[f32], right: &[f32]) {
        let frames = left.len().min(self.dry_left.len()).min(right.len());
        self.dry_left[..frames].copy_from_slice(&left[..frames]);
        self.dry_right[..frames].copy_from_slice(&right[..frames]);
        if self.line_left.is_empty() {
            return;
        }
        let mut offset = 0;
        let length = self.line_left.len();
        while offset < frames {
            let take = core::cmp::min(length, frames - offset);
            let cursor = self.cursor;
            let mut left_cursor = cursor;
            pdc_delay_block(
                &mut self.line_left,
                &mut left_cursor,
                &mut self.dry_left[offset..offset + take],
            );
            let mut right_cursor = cursor;
            pdc_delay_block(
                &mut self.line_right,
                &mut right_cursor,
                &mut self.dry_right[offset..offset + take],
            );
            debug_assert_eq!(left_cursor, right_cursor);
            self.cursor = left_cursor;
            offset += take;
        }
    }

    /// Replace the wet block with the latency-matched dry block.
    pub fn apply(&self, left: &mut [f32], right: &mut [f32]) {
        let frames = left.len().min(self.dry_left.len()).min(right.len());
        left[..frames].copy_from_slice(&self.dry_left[..frames]);
        right[..frames].copy_from_slice(&self.dry_right[..frames]);
    }

    /// The delayed dry block, for a lane-selective caller (the AoSoA racks).
    #[must_use]
    pub fn dry(&self) -> (&[f32], &[f32]) {
        (&self.dry_left, &self.dry_right)
    }
}
