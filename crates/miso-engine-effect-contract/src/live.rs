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

use miso_engine_core::realtime::Consumer;
use miso_engine_lane::kernels::pdc_delay_block;

use crate::{AutomationSpanKind, ParameterChannel, PreparedAutomationSpan};

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
}

impl EffectControlLane {
    /// Binds the consumer half of one prepared channel.
    #[must_use]
    pub fn new(control: Consumer<EffectControlRecordV1>, bypass: bool) -> Self {
        Self { control, bypass }
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
    /// `staging` is the caller's preallocated window for this lane. Records are collapsed
    /// last-wins per `(parameter_index, channel)` and inserted in canonical order, so the emitted
    /// slice is already the strictly increasing, non-overlapping block the effect contract
    /// requires -- a caller never has to sort or deduplicate on the render thread.
    ///
    /// A record that cannot fit (`staging` full of *distinct* targets) is dropped and counted in
    /// the returned overflow. Preparation makes that unreachable by refusing a queue deeper than
    /// the effect's automation capacity; the count exists so a violated invariant is observable
    /// rather than silent.
    pub fn stage(&mut self, staging: &mut [PreparedAutomationSpan], first_sample: u64) -> Staged {
        let mut staged = 0_usize;
        let mut dropped = 0_u32;
        while let Ok(record) = self.control.try_pop() {
            let (parameter_index, channel, value) = match record {
                EffectControlRecordV1::Bypass(value) => {
                    self.bypass = value;
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
        Staged { staged, dropped }
    }
}

/// What one [`EffectControlLane::stage`] call produced.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Staged {
    /// Spans written to the front of the staging window, in canonical order.
    pub staged: usize,
    /// Records refused because the window was full of distinct targets. Zero by construction.
    pub dropped: u32,
}

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

    /// Capture this block's input and advance the latency line, returning nothing.
    ///
    /// Always called, bypassed or not: the line has to stay fed so that enabling bypass mid-stream
    /// produces the correctly delayed dry signal on its very first block rather than `latency`
    /// samples of stale zeros.
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
