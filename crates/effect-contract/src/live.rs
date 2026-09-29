//! The live-console control seam: admitted records in, prepared automation spans out.
//!
//! # Why this lives in the contract crate
//!
//! Issue #137 shipped the whole `miso.command.v1` ABI and discovered that no live parameter path
//! reaches a running plan: `graph::runtime` handed every effect a hardcoded empty
//! automation slice and `rack::EffectBankStage` handed every bank an empty one with
//! zero offsets. #140 closes that gap by feeding the admitted commands in as
//! [`PreparedAutomationSpan`]s -- which every effect's `process` already honours -- rather than by
//! inventing a second parameter path.
//!
//! Two render paths need exactly the same staging: the per-node dynamic rack (`graph`)
//! and the AoSoA SIMD racks (`rack`). Neither crate depends on the other's private
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

use core::num::NonZeroUsize;
use engine::realtime::{
    Consumer, ObservationPublisher, ObservationWindow, SpscRetainedPayload,
    bounded_spsc_retained_payload, observation_slot_retained_bytes,
};
use lane::kernels::pdc_delay_block;

use crate::{
    AutomationSpanKind, ChannelSymmetryWitness, ObservationDescriptor, ObservationFold,
    ObservationSample, ParameterChannel, PreparedAutomationSpan, PreparedEffectTarget,
};

/// One admitted, still-unapplied live control event for one prepared effect instance.
///
/// It is `Copy` and fixed-size so the channel is a plain
/// [`bounded_spsc`](engine::realtime::bounded_spsc) and the render-side drain allocates
/// nothing. Addressing is *not* carried: a channel belongs to exactly one effect instance (or, in
/// a bank, to exactly one lane of one slot), so the record only says what to change.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EffectControlRecord {
    /// Retarget one declared parameter of one channel.
    ///
    /// `parameter_index` is the index into `EffectDescriptor::parameters`, never the wire
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
    /// Apply one off-render prepared effect target at this block boundary.
    PreparedTarget(PreparedEffectTarget),
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
/// The consumer half of a [`bounded_spsc`](engine::realtime::bounded_spsc); the producer
/// stays with the host's control plane. A producer must be dropped before the plan that owns this.
///
/// # A lane without a channel (issue #1087)
///
/// A session `bypass` is per-lane state, not a prepared program: preparation lowers it to a
/// prepared `bypass = false` plus this lane's initial bypass, so a bypassed track keeps its place
/// in its effect bank and the rack's [`BypassShunt`] selects the dry signal for exactly that lane.
/// A session with no live console still needs somewhere to hold that bit, so
/// [`without_channel`](Self::without_channel) builds a lane with no queue: nothing can ever be
/// admitted to it, its drain stages nothing, and its bypass is the prepared one for the life of
/// the plan. A live console replaces it with a channel seeded from the same bit.
pub struct EffectControlLane {
    /// `None` for a lane that only carries a prepared bypass ([`Self::without_channel`]).
    control: Option<Consumer<EffectControlRecord>>,
    /// Optional FIFO target storage for an owner that has a prepared-target capability.
    ///
    /// This is deliberately lane-owned and sized from the queue's actual capacity. It is absent
    /// for ordinary effects, so unsupported owners do not retain an EQ-specific side buffer.
    targets: Option<Box<[PreparedEffectTarget]>>,
    /// Number of valid entries in the retained target prefix from the last stage call.
    staged_targets: usize,
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
    symmetry: ChannelSymmetryWitness,
}

impl EffectControlLane {
    /// Binds the consumer half of one prepared channel.
    #[must_use]
    pub fn new(control: Consumer<EffectControlRecord>, bypass: bool) -> Self {
        Self::with_target_staging(control, bypass, false)
    }

    /// A lane with no live channel that carries only this instance's prepared bypass (issue
    /// #1087).
    ///
    /// Off the render thread, and it allocates nothing. It exists so that a session-bypassed
    /// instance reaches the rack's latency-preserving [`BypassShunt`] whether or not a live
    /// console is attached: the rack builds a shunt for an instance or a bank slot that holds any
    /// lane, and restores the dry signal into every lane whose [`bypassed`](Self::bypassed) is
    /// true. The witness' `UNBYPASSED` term is seeded from `bypass` exactly as [`Self::new`]
    /// seeds it, so a statically bypassed lane declines the collapse from the plan's first block.
    #[must_use]
    pub fn without_channel(bypass: bool) -> Self {
        let mut symmetry = ChannelSymmetryWitness::SYMMETRIC;
        symmetry.set(ChannelSymmetryWitness::UNBYPASSED, !bypass);
        Self {
            control: None,
            targets: None,
            staged_targets: 0,
            bypass,
            symmetry,
        }
    }

    /// Binds a prepared-target lane and allocates its FIFO backing off the render thread.
    ///
    /// The backing is exactly the queue's logical capacity. Admission guarantees that every
    /// published target has a slot, so the render path never needs a fallback allocation or a
    /// drop policy for a valid target record.
    #[must_use]
    pub fn new_with_target_staging(control: Consumer<EffectControlRecord>, bypass: bool) -> Self {
        Self::with_target_staging(control, bypass, true)
    }

    fn with_target_staging(
        control: Consumer<EffectControlRecord>,
        bypass: bool,
        target_staging: bool,
    ) -> Self {
        let mut symmetry = ChannelSymmetryWitness::SYMMETRIC;
        symmetry.set(ChannelSymmetryWitness::UNBYPASSED, !bypass);
        let targets = target_staging.then(|| {
            vec![
                PreparedEffectTarget {
                    slot: 0,
                    channel: ParameterChannel::Both,
                    words: [0; crate::PREPARED_EFFECT_TARGET_WORDS],
                };
                control.capacity()
            ]
            .into_boxed_slice()
        });
        Self {
            control: Some(control),
            targets,
            staged_targets: 0,
            bypass,
            symmetry,
        }
    }

    /// Whether this lane has a live channel a control plane can publish to.
    ///
    /// `false` only for a [`without_channel`](Self::without_channel) lane, whose bypass is fixed
    /// at preparation.
    #[must_use]
    pub const fn has_channel(&self) -> bool {
        self.control.is_some()
    }

    // REALTIME_POLICY_BEGIN

    /// This lane's live channel-symmetry terms as of the last [`stage`](Self::stage).
    ///
    /// `LIVE` and `UNBYPASSED` are the only terms this value speaks to; the other three are set,
    /// so conjoining it with the stage's prepared witness gives the whole answer and never
    /// over-claims.
    #[must_use]
    pub const fn symmetry(&self) -> ChannelSymmetryWitness {
        self.symmetry
    }

    /// Whether this instance is bypassed as of the last [`stage`](Self::stage).
    #[must_use]
    pub const fn bypassed(&self) -> bool {
        self.bypass
    }

    /// Exact queue-owned payload layout, including the sentinel slot and shared header.
    ///
    /// A [`without_channel`](Self::without_channel) lane owns no queue, so its layout is the empty
    /// one: no slots and no bytes, which an accounting caller charges as nothing.
    ///
    /// This is a control-plane accounting query; it is never called from `stage` or any other
    /// realtime-marked method.
    #[must_use]
    pub fn retained_queue_payload(&self) -> Option<SpscRetainedPayload> {
        let Some(control) = self.control.as_ref() else {
            return Some(SpscRetainedPayload {
                slot_count: 0,
                ring_header_bytes: 0,
                ring_header_align: 1,
                slot_payload_bytes: 0,
                slot_payload_align: 1,
            });
        };
        let capacity = NonZeroUsize::new(control.capacity())?;
        bounded_spsc_retained_payload::<EffectControlRecord>(capacity).ok()
    }

    /// Exact target FIFO backing bytes, or zero for an unsupported owner.
    #[must_use]
    pub fn target_staging_retained_bytes(&self) -> usize {
        self.targets.as_ref().map_or(0, |targets| {
            targets.len() * core::mem::size_of::<PreparedEffectTarget>()
        })
    }

    /// Whether this lane owns optional prepared-target staging.
    #[must_use]
    pub fn has_target_staging(&self) -> bool {
        self.targets.is_some()
    }

    /// The retained FIFO target prefix produced by the most recent [`stage`](Self::stage).
    #[must_use]
    pub fn prepared_targets(&self) -> &[PreparedEffectTarget] {
        self.targets
            .as_deref()
            .map_or(&[], |targets| &targets[..self.staged_targets])
    }

    /// Drain every queued record into `staging`, in canonical span order, and return the count.
    ///
    /// `observation` is this instance's tap state when the plan is observation-capable and `None`
    /// otherwise. An [`EffectControlRecord::Observe`] emits **no span**: it changes what is read
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
    ///
    /// `staging` must be exactly the effect's `automation_capacity` spans long. A caller that
    /// allocates it checks that once, at preparation, with
    /// [`EffectProcessBlock::check_automation_window`](crate::EffectProcessBlock::check_automation_window)
    /// or
    /// [`EffectBankProcessBlock::check_automation_window`](crate::EffectBankProcessBlock::check_automation_window);
    /// the pairing rule below depends on it.
    ///
    /// # The witness, and the one record kind it is folded late for (issue #1004)
    ///
    /// Every drained record is folded into this lane's channel-symmetry witness through
    /// [`ChannelSymmetryWitness::admit`], in FIFO order, with one exception: a one-channel
    /// [`EffectControlRecord::Parameter`] is **deferred** to the end of the drain. It is folded
    /// then (clearing `LIVE`, as it always did) unless the drain's staged spans *pair*: every
    /// `Left` span is immediately followed by its `Right` twin -- same parameter, same kind, same
    /// samples, `start_value` and `end_value` equal by bits -- and every `Right` span is
    /// immediately preceded by its `Left` twin (`spans_pair`). That is the shape the web host's
    /// both-channel command on a `PerLane` parameter lowers to, and without the deferral the first
    /// knob touch on a mono stem retires its bank's mono collapse for the rest of the plan.
    ///
    /// Why a pair leaves the two channels in bit-equal state, which is the only premise the
    /// collapse rests on: the staging below is last-wins per `(parameter_index, channel)`, so the
    /// staged pair *is* the final value of each channel; an effect validates `pending[0][p]` and
    /// `pending[1][p]` by the same checks from the same value (kind, samples, value, parameter,
    /// order and capacity -- and the window is the capacity, so a twin is never cut off), and
    /// applies them by the same code onto channels the witness already holds equal. Every other
    /// record folds in FIFO order exactly as before. In particular a one-channel
    /// [`EffectControlRecord::PreparedTarget`] is **not** deferred: a target FIFO has no last-wins
    /// staging, and a `[Left A, Both C, Right A]` drain leaves the channels at `C` and `A` although
    /// its last one-channel targets agree. The rule never sets `LIVE`; it only declines to clear
    /// it, and a pair split across two drains is two lone writes, which clear it as before.
    pub fn stage(
        &mut self,
        staging: &mut [PreparedAutomationSpan],
        first_sample: u64,
        observation: Option<&mut ObservationLane>,
    ) -> Staged {
        // A lane without a channel has nothing to drain: the loop below never runs, and the
        // window, the target prefix and the witness stay exactly as the last block left them.
        let available = self
            .control
            .as_ref()
            .map_or(0, Consumer::available_at_entry);
        let mut staged = 0_usize;
        self.staged_targets = 0;
        let mut target_error = false;
        let mut dropped = 0_u32;
        let mut unbound = 0_u32;
        let mut observation = observation;
        let mut remaining = available;
        // The first one-channel parameter record of this drain, held back from the witness until
        // the drain's staged spans can say whether it was paired (issue #1004). It is folded
        // through the same `admit` as every other record, so the hook stays the one path.
        let mut deferred: Option<EffectControlRecord> = None;
        while remaining != 0 {
            remaining -= 1;
            let Some(Ok(record)) = self.control.as_mut().map(Consumer::try_pop) else {
                // The entry snapshot and SPSC ownership guarantee this cannot happen. Keep the
                // drain bounded if a malformed test double violates that invariant.
                break;
            };
            // The one hook. `admit` takes the record by trait, not by kind, so a record type
            // added to this queue later cannot reach the render state without declaring what it
            // does to the witness (`symmetry::LiveConsoleRecord`). The deferral beside it is the
            // one exception, and it is by record *shape*, not a second path: a one-channel
            // `Parameter` is admitted at the end of the drain unless its span pairs.
            if matches!(
                record,
                EffectControlRecord::Parameter { channel, .. } if channel.writes_one_channel()
            ) {
                deferred = deferred.or(Some(record));
            } else {
                self.symmetry.admit(&record);
            }
            let (parameter_index, channel, value) = match record {
                EffectControlRecord::PreparedTarget(target) => {
                    let Some(targets) = self.targets.as_mut() else {
                        target_error = true;
                        continue;
                    };
                    if self.staged_targets == targets.len() {
                        target_error = true;
                        continue;
                    }
                    targets[self.staged_targets] = target;
                    self.staged_targets += 1;
                    continue;
                }
                EffectControlRecord::Bypass(value) => {
                    self.bypass = value;
                    continue;
                }
                EffectControlRecord::Observe {
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
                EffectControlRecord::Parameter {
                    parameter_index,
                    channel,
                    value,
                } => {
                    if self.targets.is_some() {
                        // A prepared-target owner must never silently fall back to render-time
                        // semantic design. Its checked admission path supplies a companion target
                        // for every EQ edit; a missing one is an invariant failure.
                        //
                        // A one-channel record refused here was deferred above and is never
                        // staged, so when nothing else in the drain is staged the empty window
                        // pairs and `LIVE` survives it; before #1004 it cleared `LIVE`. That is
                        // not the pairing rule firing, and it is sound: the record reaches neither
                        // channel, and `target_error` fails this block's render
                        // (`RenderError::InvalidEnvelope` in both racks) before any sample moves
                        // (issue #1004 finding 4).
                        target_error = true;
                        continue;
                    }
                    (parameter_index, channel, value)
                }
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
        // `staged <= staging.len()` by construction, and the window is exactly the effect's
        // `automation_capacity`: every caller that allocates one refuses any other size at
        // preparation (`EffectProcessBlock::check_automation_window`,
        // `EffectBankProcessBlock::check_automation_window`; issue #1012). So every staged span
        // sits below the effect's `span_index < automation_capacity` cut-off, and that cut-off
        // cannot separate a twin that `spans_pair` counts as paired. (The assertion that stood here
        // restated the loop bound and could not fire; the bound it meant is enforced where both
        // numbers are known.)
        if let Some(record) = deferred
            && !spans_pair(&staging[..staged])
        {
            self.symmetry.admit(&record);
        }
        Staged {
            staged,
            staged_targets: self.staged_targets,
            target_error,
            dropped,
            unbound,
        }
    }
}

/// Whether every one-channel span of one drain's staged window has its other-channel twin.
///
/// The window is strictly increasing in `(parameter_index, channel)` with `Left < Right < Both`
/// ([`order_key`]), so a parameter's `Left` and `Right` spans, when both are present, are
/// **adjacent**, `Left` first. The rule is therefore local: a `Left` span must be immediately
/// followed by its twin `Right` span, and a `Right` span immediately preceded by its twin `Left`
/// span. `Both` spans need no twin. One pass over adjacent pairs, no indexing, no allocation:
/// O(n) in a window the effect's automation capacity bounds.
fn spans_pair(spans: &[PreparedAutomationSpan]) -> bool {
    let unpaired_end = spans
        .first()
        .is_some_and(|span| span.channel == ParameterChannel::Right)
        || spans
            .last()
            .is_some_and(|span| span.channel == ParameterChannel::Left);
    !unpaired_end
        && spans
            .iter()
            .zip(spans.iter().skip(1))
            .all(|(earlier, later)| {
                let twins = twin_spans(earlier, later);
                (earlier.channel != ParameterChannel::Left || twins)
                    && (later.channel != ParameterChannel::Right || twins)
            })
}

/// Whether `left` and `right` are one write's two halves: the same parameter, kind and samples,
/// and values equal by bits (so `-0.0` does not twin `+0.0`, and NaN payloads must match).
fn twin_spans(left: &PreparedAutomationSpan, right: &PreparedAutomationSpan) -> bool {
    left.channel == ParameterChannel::Left
        && right.channel == ParameterChannel::Right
        && left.parameter_index == right.parameter_index
        && left.kind == right.kind
        && left.start_sample == right.start_sample
        && left.end_sample == right.end_sample
        && left.start_value.to_bits() == right.start_value.to_bits()
        && left.end_value.to_bits() == right.end_value.to_bits()
}
// REALTIME_POLICY_END

/// What one [`EffectControlLane::stage`] call produced.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Staged {
    /// Spans written to the front of the staging window, in canonical order.
    pub staged: usize,
    /// Prepared targets retained in the lane-owned FIFO prefix.
    pub staged_targets: usize,
    /// A prepared target reached an owner without target staging or exceeded its FIFO backing.
    /// Production admission rejects this before publication; render callers must surface this as
    /// an invariant failure rather than treating the record as a successful no-op.
    pub target_error: bool,
    /// Records refused because the window was full of distinct targets. Zero by construction.
    pub dropped: u32,
    /// [`EffectControlRecord::Observe`] records this plan had no capacity to apply. Zero by
    /// construction: the control plane refuses them before they reach a queue.
    pub unbound: u32,
}

/// One declared tap's arm state and open window, for one prepared instance or one bank lane.
///
/// Sixteen bytes of accumulator (`peak_left`, `peak_right`, `blocks`, `window_blocks`) plus the
/// window's own bookkeeping. All of it is allocated at preparation from the **declared menu**, so
/// arming allocates nothing and disarming frees nothing: subscribe is a flag, not a resource.
#[derive(Debug)]
struct ObservationTap {
    publisher: ObservationPublisher,
    /// The declared fold, copied once at bind so the render path never walks the descriptor.
    fold: ObservationFold,
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
/// it always was. Level 2 is `ObservationTap::armed`: inside a capable plan, an unarmed tap's
/// effect state is never read, never folded and never stored, and the honest cost is one predicted
/// branch per driven effect per block.
#[derive(Debug)]
pub struct ObservationLane {
    taps: Box<[ObservationTap]>,
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

impl ObservationLane {
    /// Bind one publisher per declared tap. Off the render thread, once, at preparation.
    ///
    /// `publishers` must be one per entry of `observations`, in declaration order.
    #[must_use]
    pub fn new(
        observations: &'static [ObservationDescriptor],
        publishers: Vec<ObservationPublisher>,
        default_window_blocks: u32,
    ) -> Option<Self> {
        if publishers.len() != observations.len() {
            return None;
        }
        let default_window_blocks = default_window_blocks.max(1);
        let taps: Vec<ObservationTap> = observations
            .iter()
            .zip(publishers)
            .map(|(descriptor, publisher)| ObservationTap {
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
            * (core::mem::size_of::<ObservationTap>() + observation_slot_retained_bytes())
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

    /// Apply one drained [`EffectControlRecord::Observe`]; `false` for an index this lane has
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
impl ObservationLane {
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
        sample: ObservationSample,
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
            ObservationFold::PeakMagnitude => {
                let left = sample.left.abs();
                let right = sample.right.abs();
                if left > tap.peak_left {
                    tap.peak_left = left;
                }
                if right > tap.peak_right {
                    tap.peak_right = right;
                }
            }
            ObservationFold::Latest => {
                tap.peak_left = sample.left;
                tap.peak_right = sample.right;
            }
        }
        tap.blocks = tap.blocks.saturating_add(1);
        if tap.blocks < tap.window_blocks {
            return;
        }
        tap.sequence = tap.sequence.saturating_add(1);
        tap.publisher.publish(ObservationWindow {
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
/// [`EffectProgramKey`](crate::EffectProgramKey), it is byte 108 of the persisted state
/// envelope, and every effect's bank reads one flag for the whole bank. Moving it after
/// preparation would change a program key at render time -- which would re-cohort a bank -- and a
/// bank's lanes could not disagree about it anyway.
///
/// The shunt is the design the contract already documents on `EffectProgramKey`, minus the parts
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
/// # When a shunt exists (issue #1087)
///
/// A session `bypass` is per-lane shunt state, never a prepared program: preparation lowers it to
/// a prepared `bypass = false` plus the lane's initial bypass, carried by an
/// [`EffectControlLane`] (a live channel, or [`EffectControlLane::without_channel`] when no console
/// is attached). A shunt is built for every instance, and every bank slot, that holds such a lane:
/// a live channel, or a lane bypassed at preparation. A session with neither allocates none of
/// this and renders the byte-identical path it always did.
///
/// A shunt-bypassed lane is bit-identical to the same instance prepared with `bypass = true`:
/// every launch effect that banks emits, under a prepared bypass, its input delayed by exactly its
/// declared latency from a line that starts at `+0.0`, and checks only that output at its block
/// boundary (D7), and this shunt emits the same words from a line that starts at `+0.0`, by
/// copies, so `-0.0` survives. The two differ only where that D7 check fires on the dry block
/// itself: a non-finite sample, or one at least `1e30` in magnitude. An effect never receives one
/// in a compiled plan, because the track's input stage sanitises every such sample to `+0.0` and
/// every stage after it zeroes a block that leaves that range. The delay, whose D7 check also
/// reads its state, never banks and keeps its prepared bypass
/// (`effect_compiler::NEVER_BANKED_EFFECTS`). `graph-compiler`'s `bypass_shunt_identity` and
/// `bypass_cohorts` tests are the gates.
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
