//! The host's live VCA composition (issue #1244, slice V5 of #1239).
//!
//! A VCA has no audio path: its per-lane dB offset adds to every member's own fader, and its
//! per-lane mute mutes every member. Preparation composes both once, through
//! `session::SessionModel::effective_strip_faders` (#1242). [`LiveVcaState`] is the same
//! composition kept live on the control plane, so a VCA move, or a member's own fader move, can be
//! told to the render plane as the member fader and mute records a fresh plan prepared from the
//! edited session would bake.
//!
//! # What it keeps
//!
//! - Per VCA, its `[left, right]` offset in dB and its `[left, right]` mute.
//! - Per strip, its **own** fader dB -- the member's intent, never an effective value, so a VCA
//!   that clamps a member and is then restored returns it to its own balance -- and the effective
//!   dB the render plane was last told, seeded with what the prepared fader bakes.
//! - The reach, flattened once at preparation from `SessionModel::vca_reach`: per strip, its VCAs
//!   in ascending VCA-ID order (the order the offsets are summed in), and per VCA, the strips it
//!   reaches in ascending strip order (the inverse table, [`LiveVcaState::reached_by`]).
//!
//! The effective dB is [`session::vca_effective_db`] over the strip's own value and its reach's
//! offsets, in that order: the one composition, never a re-spelling. The effective VCA mute is the
//! OR of the reach's mutes; it is handed to the one strip-mute owner,
//! [`crate::LiveControlSoloState::set_vca_mute`], whose `strip_delta` and the follow pass then emit
//! the changed lanes.
//!
//! # Indices
//!
//! A strip index is the position in `strips()` of the normalized model (the tracks, then the
//! submixes), the index every live-control queue and the solo state use. A VCA index is the
//! position in the normalized model's `vcas`, which is canonical VCA-ID order.
//!
//! # Never a redundant record
//!
//! [`LiveVcaState::fader_delta`] yields only the lanes whose effective dB differs from what the
//! render plane was last told. A retarget of a settled lane re-enters the ramp kernel, which moves
//! bits, so this is a correctness rule, not an optimization (see the solo module).
//!
//! # The transaction and the allocation rule
//!
//! Like the solo state and the route mirror, the state carries its own shadow: the first mutation
//! of a submission copies the mutable arrays aside, [`LiveVcaState::rollback`] restores them, and
//! [`LiveVcaState::commit`] closes the transaction. Every array, the shadow included, is reserved
//! in [`LiveVcaState::try_new`]; nothing afterwards allocates, and nothing runs on a render
//! thread. A model with no VCA keeps nothing at all.
//!
//! # Size
//!
//! The reach tables hold one entry per (strip, reaching VCA) pair, twice (forward and inverse), so
//! they grow as strips times nesting depth. The #1243 verdict measured the browser's worst case at
//! about 1.7 M pairs for a 1 MiB document of chained VCAs; [`LiveVcaState::retained_bytes`] is
//! what a host charges for it.

use std::collections::TryReserveError;

use builtins::BuiltinLaneSelector;
use session::{SessionModel, vca_effective_db};

use crate::solo::covers;

/// The fader records one reached strip still owes the render plane: at most two, one `Both` when
/// both lanes change to one value.
pub type LiveVcaFaderDelta = [Option<(BuiltinLaneSelector, f32)>; 2];

/// Live VCA values, member values and the emitted mirror of one prepared session, with a
/// transaction shadow.
#[derive(Debug)]
pub struct LiveVcaState {
    /// Per VCA, `[left, right]` offset in dB.
    vca_db: Box<[[f32; 2]]>,
    /// Per VCA, `[left, right]` mute.
    vca_mute: Box<[[bool; 2]]>,
    /// Per strip, its own `[left, right]` fader dB.
    own_db: Box<[[f32; 2]]>,
    /// Per strip, the effective `[left, right]` dB the render plane was last told.
    emitted_db: Box<[[f32; 2]]>,
    /// `reach[reach_start[s]..reach_start[s + 1]]` are strip `s`'s VCAs, ascending by VCA ID.
    reach_start: Box<[usize]>,
    reach: Box<[usize]>,
    /// `reached[reached_start[v]..reached_start[v + 1]]` are VCA `v`'s strips, ascending.
    reached_start: Box<[usize]>,
    reached: Box<[usize]>,
    /// Strips some VCA reaches.
    reached_strip_count: usize,
    vca_db_shadow: Box<[[f32; 2]]>,
    vca_mute_shadow: Box<[[bool; 2]]>,
    own_db_shadow: Box<[[f32; 2]]>,
    emitted_db_shadow: Box<[[f32; 2]]>,
    open: bool,
}

impl LiveVcaState {
    /// Build the state from a normalized model (`CompiledSession::normalized_model`). A model
    /// with no VCA returns [`Self::empty`] and allocates nothing.
    ///
    /// # Errors
    ///
    /// The allocator's own error when an array cannot be reserved. Every allocation happens here;
    /// none can happen again later.
    pub fn try_new(model: &SessionModel) -> Result<Self, TryReserveError> {
        if model.vcas.is_empty() {
            return Ok(Self::empty());
        }
        let vca_count = model.vcas.len();
        let reach_lists = model.vca_reach();
        let strip_count = reach_lists.len();

        let mut reach_start = try_vec(strip_count + 1)?;
        let total: usize = reach_lists.iter().map(Vec::len).sum();
        let mut reach = try_vec(total)?;
        reach_start.push(0);
        for list in &reach_lists {
            reach.extend_from_slice(list);
            reach_start.push(reach.len());
        }

        // The inverse table: count each VCA's strips, prefix-sum, then place every strip in
        // ascending strip order through a transient cursor.
        let mut reached_start = try_vec(vca_count + 1)?;
        reached_start.extend(core::iter::repeat_n(0_usize, vca_count + 1));
        for &vca in &reach {
            reached_start[vca + 1] += 1;
        }
        for vca in 0..vca_count {
            reached_start[vca + 1] += reached_start[vca];
        }
        let mut cursor = try_vec(vca_count)?;
        cursor.extend_from_slice(&reached_start[..vca_count]);
        let mut reached = try_vec(total)?;
        reached.extend(core::iter::repeat_n(0_usize, total));
        for (strip, list) in reach_lists.iter().enumerate() {
            for &vca in list {
                reached[cursor[vca]] = strip;
                cursor[vca] += 1;
            }
        }
        drop(cursor);
        let reached_strip_count = reach_lists.iter().filter(|list| !list.is_empty()).count();
        drop(reach_lists);

        let vcas = model.vcas.iter();
        let vca_db = try_boxed(
            vca_count,
            vcas.clone()
                .map(|vca| [vca.fader.left_db, vca.fader.right_db]),
        )?;
        let vca_mute = try_boxed(
            vca_count,
            vcas.map(|vca| [vca.fader.left_mute, vca.fader.right_mute]),
        )?;
        let own_db = try_boxed(
            strip_count,
            model
                .strips()
                .map(|strip| [strip.fader.left_db, strip.fader.right_db]),
        )?;
        // What the prepared fader bakes: the render plane has already been told exactly this.
        let effective = model.effective_strip_faders();
        let emitted_db = try_boxed(strip_count, effective.iter().map(|fader| fader.db))?;
        drop(effective);
        let vca_db_shadow = try_boxed(vca_count, vca_db.iter().copied())?;
        let vca_mute_shadow = try_boxed(vca_count, vca_mute.iter().copied())?;
        let own_db_shadow = try_boxed(strip_count, own_db.iter().copied())?;
        let emitted_db_shadow = try_boxed(strip_count, emitted_db.iter().copied())?;
        Ok(Self {
            vca_db,
            vca_mute,
            own_db,
            emitted_db,
            reach_start: reach_start.into_boxed_slice(),
            reach: reach.into_boxed_slice(),
            reached_start: reached_start.into_boxed_slice(),
            reached: reached.into_boxed_slice(),
            reached_strip_count,
            vca_db_shadow,
            vca_mute_shadow,
            own_db_shadow,
            emitted_db_shadow,
            open: false,
        })
    }

    /// The state of a plan with no VCA: it reaches nothing and keeps nothing.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            vca_db: Box::new([]),
            vca_mute: Box::new([]),
            own_db: Box::new([]),
            emitted_db: Box::new([]),
            reach_start: Box::new([]),
            reach: Box::new([]),
            reached_start: Box::new([]),
            reached: Box::new([]),
            reached_strip_count: 0,
            vca_db_shadow: Box::new([]),
            vca_mute_shadow: Box::new([]),
            own_db_shadow: Box::new([]),
            emitted_db_shadow: Box::new([]),
            open: false,
        }
    }

    /// VCAs of the plan.
    #[must_use]
    pub const fn vca_count(&self) -> usize {
        self.vca_db.len()
    }

    /// Whether any VCA reaches this strip. `false` for an index the plan has no strip for.
    #[must_use]
    pub fn reaches(&self, strip: usize) -> bool {
        !self.reach_of(strip).is_empty()
    }

    /// The strips this VCA reaches, ascending (the inverse of the reach table, built once). Empty
    /// for an index the plan has no VCA for.
    #[must_use]
    pub fn reached_by(&self, vca: usize) -> &[usize] {
        span(&self.reached_start, &self.reached, vca)
    }

    /// Strips some VCA reaches (the browser sizes its staging by it, #1245).
    #[must_use]
    pub const fn reached_strip_count(&self) -> usize {
        self.reached_strip_count
    }

    /// Set the covered lanes of one VCA's offset. `false`, changing nothing, for an unknown VCA.
    pub fn set_vca_db(&mut self, vca: usize, lanes: BuiltinLaneSelector, db: f32) -> bool {
        if vca >= self.vca_db.len() {
            return false;
        }
        self.shadow();
        set_lanes(&mut self.vca_db[vca], lanes, db);
        true
    }

    /// Set the covered lanes of one VCA's mute. `false`, changing nothing, for an unknown VCA.
    pub fn set_vca_mute(&mut self, vca: usize, lanes: BuiltinLaneSelector, muted: bool) -> bool {
        if vca >= self.vca_mute.len() {
            return false;
        }
        self.shadow();
        set_lanes(&mut self.vca_mute[vca], lanes, muted);
        true
    }

    /// Set the covered lanes of a reached strip's own fader value. `false`, changing nothing, for
    /// a strip no VCA reaches: its own value is its effective one, which the host stages as is.
    pub fn set_member_db(&mut self, strip: usize, lanes: BuiltinLaneSelector, db: f32) -> bool {
        if !self.reaches(strip) {
            return false;
        }
        self.shadow();
        set_lanes(&mut self.own_db[strip], lanes, db);
        true
    }

    /// The effective dB of one lane of a reached strip: [`vca_effective_db`] of its own value and
    /// its reach's offsets, in ascending VCA-ID order. For a strip no VCA reaches it is the value
    /// preparation baked, and `0.0` for an index the plan has no strip or lane for.
    #[must_use]
    pub fn effective_db(&self, strip: usize, lane: usize) -> f32 {
        let Some(own) = self.own_db.get(strip).and_then(|own| own.get(lane)) else {
            return 0.0;
        };
        vca_effective_db(
            *own,
            self.reach_of(strip)
                .iter()
                .map(|&vca| self.vca_db[vca][lane]),
        )
    }

    /// `[left, right]`: whether any VCA reaching the strip mutes the lane.
    #[must_use]
    pub fn vca_mute(&self, strip: usize) -> [bool; 2] {
        let mut muted = [false; 2];
        for &vca in self.reach_of(strip) {
            muted[0] |= self.vca_mute[vca][0];
            muted[1] |= self.vca_mute[vca][1];
        }
        muted
    }

    /// The fader records a reached strip still owes: the lanes whose [`Self::effective_db`]
    /// differs (`!=`) from what the render plane was last told; one `Both` when both change to
    /// one value. Nothing for a strip no VCA reaches.
    #[must_use]
    pub fn fader_delta(&self, strip: usize) -> LiveVcaFaderDelta {
        if !self.reaches(strip) {
            return [None, None];
        }
        let left = self.effective_db(strip, 0);
        let right = self.effective_db(strip, 1);
        let emitted = self.emitted_db[strip];
        match (left != emitted[0], right != emitted[1]) {
            (false, false) => [None, None],
            (true, false) => [Some((BuiltinLaneSelector::Left, left)), None],
            (false, true) => [Some((BuiltinLaneSelector::Right, right)), None],
            (true, true) if left == right => [Some((BuiltinLaneSelector::Both, left)), None],
            (true, true) => [
                Some((BuiltinLaneSelector::Left, left)),
                Some((BuiltinLaneSelector::Right, right)),
            ],
        }
    }

    /// Record that a fader record for `lanes` carrying `db` has been staged for this strip, so
    /// [`Self::fader_delta`] does not stage it again. Nothing for an unknown strip.
    pub fn record_emitted_db(&mut self, strip: usize, lanes: BuiltinLaneSelector, db: f32) {
        if strip >= self.emitted_db.len() {
            return;
        }
        self.shadow();
        set_lanes(&mut self.emitted_db[strip], lanes, db);
    }

    /// Whether a transaction has mutated anything since the last commit or rollback.
    #[must_use]
    pub const fn transaction_open(&self) -> bool {
        self.open
    }

    /// Close the transaction: the staged records reached their queues.
    pub const fn commit(&mut self) {
        self.open = false;
    }

    /// Undo everything this transaction did. A refused submission leaves no trace.
    pub fn rollback(&mut self) {
        if !self.open {
            return;
        }
        self.vca_db.copy_from_slice(&self.vca_db_shadow);
        self.vca_mute.copy_from_slice(&self.vca_mute_shadow);
        self.own_db.copy_from_slice(&self.own_db_shadow);
        self.emitted_db.copy_from_slice(&self.emitted_db_shadow);
        self.open = false;
    }

    /// Every byte [`Self::try_new`] keeps: the tables, the mirrors and the shadow. 0 for
    /// [`Self::empty`]. A host charges it as retained bridge or C ABI state (#1245, #1247).
    #[must_use]
    pub fn retained_bytes(&self) -> u64 {
        self.allocation_sizes()
            .iter()
            .map(|&bytes| bytes as u64)
            .sum()
    }

    /// The largest single allocation among [`Self::retained_bytes`]; 0 for [`Self::empty`].
    #[must_use]
    pub fn largest_allocation_bytes(&self) -> u64 {
        self.allocation_sizes()
            .iter()
            .map(|&bytes| bytes as u64)
            .max()
            .unwrap_or(0)
    }

    /// The byte size of every boxed array.
    fn allocation_sizes(&self) -> [usize; 12] {
        [
            size_of_val(&*self.vca_db),
            size_of_val(&*self.vca_mute),
            size_of_val(&*self.own_db),
            size_of_val(&*self.emitted_db),
            size_of_val(&*self.reach_start),
            size_of_val(&*self.reach),
            size_of_val(&*self.reached_start),
            size_of_val(&*self.reached),
            size_of_val(&*self.vca_db_shadow),
            size_of_val(&*self.vca_mute_shadow),
            size_of_val(&*self.own_db_shadow),
            size_of_val(&*self.emitted_db_shadow),
        ]
    }

    /// One strip's reaching VCAs, ascending by VCA ID; empty for an unknown strip.
    fn reach_of(&self, strip: usize) -> &[usize] {
        span(&self.reach_start, &self.reach, strip)
    }

    /// Take the transaction shadow, once, before the first mutation of a submission.
    fn shadow(&mut self) {
        if self.open {
            return;
        }
        self.vca_db_shadow.copy_from_slice(&self.vca_db);
        self.vca_mute_shadow.copy_from_slice(&self.vca_mute);
        self.own_db_shadow.copy_from_slice(&self.own_db);
        self.emitted_db_shadow.copy_from_slice(&self.emitted_db);
        self.open = true;
    }
}

/// `table[start[index]..start[index + 1]]`, or empty for an index past the last.
fn span<'a>(start: &[usize], table: &'a [usize], index: usize) -> &'a [usize] {
    match (start.get(index), start.get(index + 1)) {
        (Some(&from), Some(&to)) => &table[from..to],
        _ => &[],
    }
}

fn set_lanes<T: Copy>(lanes_of: &mut [T; 2], lanes: BuiltinLaneSelector, value: T) {
    for (lane, slot) in lanes_of.iter_mut().enumerate() {
        if covers(lanes, lane) {
            *slot = value;
        }
    }
}

fn try_vec<T>(count: usize) -> Result<Vec<T>, TryReserveError> {
    let mut buffer = Vec::new();
    buffer.try_reserve_exact(count)?;
    Ok(buffer)
}

/// `count` values into an exactly reserved boxed slice; `values` yields exactly `count`.
fn try_boxed<T>(
    count: usize,
    values: impl Iterator<Item = T>,
) -> Result<Box<[T]>, TryReserveError> {
    let mut buffer = try_vec(count)?;
    buffer.extend(values.take(count));
    Ok(buffer.into_boxed_slice())
}
