//! Safe preallocated AoSoA rack primitives.
//!
//! The compiler owns all structural decisions. This crate only accepts prepared dimensions and
//! sequences already-prepared bank stages over owned, sample-major scratch.
//!
//! Master plan #83 §4.5: planar -> AoSoA and back happen **once per bank chain per block**, in the
//! rack executor, never per slot. [`BankChain`] is that executor: one gather, then every slot of
//! the chain over the resident block, then one scatter. [`BankChain::transposes`] counts the
//! round-trips so the audit tooling can prove the law.
#![allow(missing_docs)]

use effect_contract::{
    BankCrossfade, BankCrossfadeLane, BankWidth, BypassShunt, ChannelSymmetryWitness,
    EffectBankProcessBlock, EffectControlLane, EffectControlRecord, EffectId, EffectProgramKey,
    LatencySamples, ObservationLane, ObservationSample, PreparedAutomationSpan, PreparedEffectBank,
    PreparedNativeEffect, PreparedNativeEffectBank, PreparedSidechainPort, SeamSide,
    StatePayloadInput, StatePayloadOutput, StatePayloadSizes, apply_observe, transpose_tile_4,
};
use engine::realtime::{Consumer, LiveSnapshot, RenderError};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RackError {
    ZeroQuantum,
    Overflow,
    Shape,
    WidthMismatch,
    /// A live-control slot has a lane with more live parameter cells than the bank's
    /// `automation_capacity` spans: the window that holds them would exceed the capacity the
    /// effect cuts spans off at (issue #1012).
    AutomationWindow,
}

/// Where in a track's chain a bank cohort sits.
///
/// The two SIMD racks are the architecture's *declared* bank locations. [`Self::Dynamic`] is the
/// dynamic rack, which AGENTS.md permits to run track-locally -- and, for a **native** effect that
/// carries the homogeneous-bank kernel contract, to bank exactly like a SIMD rack. Session
/// placement decides where an effect runs in the signal chain; it never decides how wide the
/// arithmetic is. The boundary that does decide is opacity, not location: opaque third-party Wasm
/// has no homogeneous bank kernel and is per-instance wherever it sits (AGENTS.md, "Effects and
/// plugins").
///
/// The variant exists on the *cohort* type so that a dynamic chain can never share a cohort with a
/// SIMD chain: [`RackProgram::subsequence_mask`] compares `rack` first, and
/// `rack_compiler::plan_bank_groups` pools per location.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RackLocation {
    Simd1 = 1,
    Simd2 = 2,
    Dynamic = 3,
}

impl RackLocation {
    /// Every bank location, in cohort-planning order.
    ///
    /// `plan_bank_groups` iterates this, so a new location is planned by construction rather than
    /// by remembering to extend a literal array.
    pub const ALL: [Self; 3] = [Self::Simd1, Self::Simd2, Self::Dynamic];
}

/// What a cohort planner needs of one slot's key.
///
/// The planner treats a key as an opaque cohort token: two lanes share a slot exactly when their
/// keys are equal. The one thing it must ask is whether a key can bank at all.
pub trait BankSlotKey: Clone + Eq + Ord {
    /// `true` when a slot carrying this key can never join a homogeneous bank.
    ///
    /// The default is `false`: a fixed graph stage (the post-input builtins) always banks.
    fn blocks_banking(&self) -> bool {
        false
    }
}

impl BankSlotKey for EffectProgramKey {
    /// A connected sidechain reads a second graph buffer that a homogeneous bank has no port for
    /// (#96 F9), so such a program renders per node.
    fn blocks_banking(&self) -> bool {
        matches!(
            self.ports.sidechain,
            PreparedSidechainPort::Connected { .. }
        )
    }
}

/// The ordered per-track program of one SIMD rack.
///
/// There is no rate, quantum or routing field: every [`EffectProgramKey`] slot already carries
/// `sample_rate`, `quantum` and `ports.sidechain`, so a second copy could only disagree (#96 F5.4).
///
/// `K` is the slot key, defaulting to [`EffectProgramKey`] — the SIMD racks' key. A fixed graph
/// stage with a key of its own (the post-input builtin bank, #86) is planned by the same planner
/// without either side having to fabricate the other's key type.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RackProgram<K = EffectProgramKey> {
    pub rack: RackLocation,
    pub slots: Box<[K]>,
}

impl<K: BankSlotKey> RackProgram<K> {
    #[must_use]
    pub fn new(rack: RackLocation, slots: Vec<K>) -> Self {
        Self {
            rack,
            slots: slots.into_boxed_slice(),
        }
    }
    /// `true` iff the program is non-empty and no slot blocks banking.
    ///
    /// An empty program needs no bank at all (#96 F5.1).
    #[must_use]
    pub fn is_bankable(&self) -> bool {
        !self.slots.is_empty() && !self.slots.iter().any(BankSlotKey::blocks_banking)
    }
    /// Greedy leftmost subsequence match of `self.slots` inside `leader.slots`.
    ///
    /// Returns the leader-indexed activity mask (`true` = this track runs that slot, `false` = the
    /// slot is an identity for this lane), or `None` when no match exists. Slots compare by
    /// [`EffectProgramKey`] equality only - never by occurrence index (#96 F5.3). Greedy
    /// leftmost matching decides subsequence membership exactly: any match can be left-shifted
    /// into the greedy one.
    #[must_use]
    pub fn subsequence_mask(&self, leader: &Self) -> Option<Box<[bool]>> {
        if self.rack != leader.rack || self.slots.len() > leader.slots.len() {
            return None;
        }
        let mut cursor = 0usize;
        let mut mask = vec![false; leader.slots.len()];
        for slot in &self.slots {
            while cursor < leader.slots.len() && leader.slots[cursor] != *slot {
                cursor += 1;
            }
            if cursor == leader.slots.len() {
                return None;
            }
            mask[cursor] = true;
            cursor += 1;
        }
        Some(mask.into_boxed_slice())
    }
}

/// Owned left/right sample-major scratch. Its logical index is `sample * lanes + lane`.
///
/// Two planes, not four: the bank sidechain planes were never read and are gone (#96 F9). A bank
/// sidechain block, if one is ever needed, is a separate read-only block allocated by the owner
/// that declares it (master plan §4.1).
pub struct AoSoaScratch {
    width: BankWidth,
    quantum: u32,
    left: Box<[f32]>,
    right: Box<[f32]>,
}

impl AoSoaScratch {
    pub fn new(width: BankWidth, quantum: u32) -> Result<Self, RackError> {
        if quantum == 0 {
            return Err(RackError::ZeroQuantum);
        }
        let length = (quantum as usize)
            .checked_mul(width.lanes() as usize)
            .ok_or(RackError::Overflow)?;
        Ok(Self {
            width,
            quantum,
            left: vec![0.0; length].into_boxed_slice(),
            right: vec![0.0; length].into_boxed_slice(),
        })
    }
    #[must_use]
    pub const fn width(&self) -> BankWidth {
        self.width
    }
    #[must_use]
    pub const fn quantum(&self) -> u32 {
        self.quantum
    }

    // REALTIME_POLICY_BEGIN
    /// Copy frames `[from, frames)` of one planar track into its stable AoSoA lane. Shape is a
    /// `debug_assert`: the compiler fixed it once (master plan §4.3).
    ///
    /// This is the **per-lane scalar** move: one `f32` store per lane-sample, at the lane's stride
    /// through the block. It stays the whole path for a partial bank and the ragged tail of a full
    /// one; [`BankChain::gather`] explains why.
    fn gather_lane(&mut self, lane: usize, left: &[f32], right: &[f32], from: usize, frames: u32) {
        let lanes = self.width.lanes() as usize;
        let len = frames as usize * lanes;
        debug_assert!(lane < lanes);
        debug_assert!(left.len() == frames as usize && right.len() == frames as usize);
        debug_assert!(len <= self.left.len());
        debug_assert!(from <= frames as usize);
        for (chunk, &sample) in self.left[from * lanes..len]
            .chunks_exact_mut(lanes)
            .zip(&left[from..])
        {
            chunk[lane] = sample;
        }
        for (chunk, &sample) in self.right[from * lanes..len]
            .chunks_exact_mut(lanes)
            .zip(&right[from..])
        {
            chunk[lane] = sample;
        }
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    /// [`AoSoaScratch::gather_lane`] for the left plane alone: the collapsed cohort's move.
    fn gather_lane_left(&mut self, lane: usize, left: &[f32], from: usize, frames: u32) {
        let lanes = self.width.lanes() as usize;
        let len = frames as usize * lanes;
        debug_assert!(lane < lanes);
        debug_assert!(left.len() == frames as usize);
        debug_assert!(len <= self.left.len());
        debug_assert!(from <= frames as usize);
        for (chunk, &sample) in self.left[from * lanes..len]
            .chunks_exact_mut(lanes)
            .zip(&left[from..])
        {
            chunk[lane] = sample;
        }
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    /// Copy frames `[from, frames)` of one stable AoSoA lane back into its planar graph buffer.
    fn scatter_lane(
        &self,
        lane: usize,
        left: &mut [f32],
        right: &mut [f32],
        from: usize,
        frames: u32,
    ) {
        let lanes = self.width.lanes() as usize;
        let len = frames as usize * lanes;
        debug_assert!(lane < lanes);
        debug_assert!(left.len() == frames as usize && right.len() == frames as usize);
        debug_assert!(len <= self.left.len());
        debug_assert!(from <= frames as usize);
        for (chunk, sample) in self.left[from * lanes..len]
            .chunks_exact(lanes)
            .zip(left[from..].iter_mut())
        {
            *sample = chunk[lane];
        }
        for (chunk, sample) in self.right[from * lanes..len]
            .chunks_exact(lanes)
            .zip(right[from..].iter_mut())
        {
            *sample = chunk[lane];
        }
    }
    // REALTIME_POLICY_END
}

// REALTIME_POLICY_BEGIN
/// One plane's tiled planar -> AoSoA gather.
///
/// Per `W`-frame tile: `W` contiguous vector loads (one per planar lane), one whole-tile shuffle
/// transpose, `W` contiguous vector stores into sample-major scratch. The scalar path this
/// replaces moved every lane-sample one 32-bit word at a time, twice per chain block.
///
/// `W` is the bank width, never a literal: a four-lane bank tiles four frames at a time and an
/// eight-lane bank eight, so the wasm `simd128` build (four lanes) and the `x86-64-v3` build (eight)
/// each transpose at their own width (#183).
#[inline(always)]
fn tile_gather<const W: usize>(
    destination: &mut [f32],
    planes: &[&[f32]; W],
    transpose: impl Fn([[f32; W]; W]) -> [[f32; W]; W],
) {
    for (tile, block) in destination.chunks_exact_mut(W * W).enumerate() {
        let base = tile * W;
        let mut rows = [[0.0_f32; W]; W];
        for (row, plane) in rows.iter_mut().zip(planes.iter()) {
            row.copy_from_slice(&plane[base..base + W]);
        }
        for (chunk, row) in block.chunks_exact_mut(W).zip(transpose(rows)) {
            chunk.copy_from_slice(&row);
        }
    }
}
// REALTIME_POLICY_END

// REALTIME_POLICY_BEGIN
/// One plane's tiled AoSoA -> lane-major scatter, into an owned staging block of `W` lanes at
/// `stride` words each.
///
/// The inverse of [`tile_gather`], with one difference forced by [`BankMembers`]: a gather may hold
/// every lane's planar view at once (`plane` takes `&self`), but `plane_mut` hands out one lane at
/// a time, so the transpose lands in engine-owned staging and each lane is then handed its result
/// as **one contiguous copy** instead of one strided store per sample.
#[inline(always)]
fn tile_scatter<const W: usize>(
    source: &[f32],
    staging: &mut [f32],
    stride: usize,
    transpose: impl Fn([[f32; W]; W]) -> [[f32; W]; W],
) {
    for (tile, block) in source.chunks_exact(W * W).enumerate() {
        let base = tile * W;
        let mut rows = [[0.0_f32; W]; W];
        for (row, chunk) in rows.iter_mut().zip(block.chunks_exact(W)) {
            row.copy_from_slice(chunk);
        }
        for (lane, row) in transpose(rows).into_iter().enumerate() {
            let offset = lane * stride + base;
            staging[offset..offset + W].copy_from_slice(&row);
        }
    }
}

#[inline(always)]
fn tile_scatter_direct_plane<const W: usize>(
    source: &[f32],
    destinations: &mut [BankPlanePair<'_>; W],
    frames: usize,
    right_plane: bool,
    transpose: impl Fn([[f32; W]; W]) -> [[f32; W]; W],
) {
    let tiled = (frames / W) * W;
    for (tile, block) in source[..tiled * W].chunks_exact(W * W).enumerate() {
        let base = tile * W;
        let mut rows = [[0.0_f32; W]; W];
        for (row, chunk) in rows.iter_mut().zip(block.chunks_exact(W)) {
            row.copy_from_slice(chunk);
        }
        for ((left, right), row) in destinations.iter_mut().zip(transpose(rows)) {
            let plane = if right_plane { right } else { left };
            plane[base..base + W].copy_from_slice(&row);
        }
    }
    for frame in tiled..frames {
        for (lane, (left, right)) in destinations.iter_mut().enumerate() {
            let plane = if right_plane { right } else { left };
            plane[frame] = source[frame * W + lane];
        }
    }
}
// REALTIME_POLICY_END

/// The resident AoSoA block handed to one stage. `left.len() == right.len() == frames * lanes`.
pub struct BankBlock<'a> {
    pub left: &'a mut [f32],
    pub right: &'a mut [f32],
    pub frames: u32,
    pub first_sample: u64,
    pub lanes: usize,
}

/// Mutably borrowed, pairwise-disjoint planar destinations for one complete bank.
///
/// Providers construct this only after validating ownership and capacities.  The fixed upper
/// bound is the two launch bank widths and keeps the render path allocation-free.
pub type BankPlanePair<'a> = (&'a mut [f32], &'a mut [f32]);

pub struct BankPlaneViews<'a>(BankPlaneViewsInner<'a>);

/// The views of a four- or an eight-lane bank. `Eight`, `BankPlaneViews::from_eight` and their
/// arms exist only where `avx2` is enabled, the one build with an eight-lane bank (issues #1110
/// and #1112, [`BankWidth`](effect_contract::BankWidth)).
enum BankPlaneViewsInner<'a> {
    Four([BankPlanePair<'a>; 4], usize),
    #[cfg(target_feature = "avx2")]
    Eight([BankPlanePair<'a>; 8], usize),
}

impl<'a> BankPlaneViews<'a> {
    fn capacity<const W: usize>(pairs: &[BankPlanePair<'_>; W], frames: usize) -> bool {
        if pairs
            .iter()
            .any(|(left, right)| left.len() < frames || right.len() < frames)
        {
            return false;
        }
        true
    }

    pub fn from_four(pairs: [BankPlanePair<'a>; 4], frames: usize) -> Option<Self> {
        Self::capacity(&pairs, frames).then_some(Self(BankPlaneViewsInner::Four(pairs, frames)))
    }

    #[cfg(target_feature = "avx2")]
    pub fn from_eight(pairs: [BankPlanePair<'a>; 8], frames: usize) -> Option<Self> {
        Self::capacity(&pairs, frames).then_some(Self(BankPlaneViewsInner::Eight(pairs, frames)))
    }

    #[inline(always)]
    fn supports(&self, width: usize, frames: usize) -> bool {
        match &self.0 {
            BankPlaneViewsInner::Four(_, capacity) => width == 4 && *capacity >= frames,
            #[cfg(target_feature = "avx2")]
            BankPlaneViewsInner::Eight(_, capacity) => width == 8 && *capacity >= frames,
        }
    }
}

/// One stage of a bank chain.
///
/// This is the seam #86 (builtin banks) and #98 (graph bank execution) implement against: a stage
/// owns its prepared processor and sees only the resident block. Shape is infallible here - it was
/// validated once at prepare - so `Err` is the stage's own render failure.
pub trait BankStage: Send {
    fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError>;
    /// Copy one prepared lane's response sections without advancing render state.
    fn copy_response_snapshot_lane(
        &self,
        _lane: usize,
        _sample_rate_hz: u32,
        _request: effect_contract::ResponseSnapshotRequest<'_>,
    ) -> Result<effect_contract::ResponseSnapshotSummary, effect_contract::ResponseAnalysisError>
    {
        Err(effect_contract::ResponseAnalysisError::UnsupportedCapability)
    }
    /// Bypass state used when a lane has no native response provider.  It is read at the same
    /// control boundary as the ordered unavailable-owner marker.
    fn response_snapshot_bypassed(&self, _lane: usize) -> bool {
        false
    }
    /// Cumulative `[process_calls, kernel_calls]`, read only after render is disarmed.
    fn qualification_counters(&self) -> [u64; 2] {
        [0, 0]
    }
    /// `[observed lanes, declared taps, armed taps]` (issue #143 E5). Zero for every stage that
    /// carries no observation state at all, which is every stage in an unobserved plan.
    fn observation_binding_counts(&self) -> [u64; 3] {
        [0, 0, 0]
    }
    /// Exact engine-owned observation bytes this stage retains. Zero when unobserved.
    fn observation_retained_bytes(&self) -> usize {
        0
    }

    /// This stage's channel-symmetry witness for one lane of the cohort.
    ///
    /// The default is [`ChannelSymmetryWitness::DECLINED`], not `SYMMETRIC`: a stage that has
    /// not derived a witness from its kernel's read surface declines, so an unclassified stage in
    /// a chain makes the whole cohort decline rather than silently claiming eligibility for work
    /// nobody checked -- and since mono-collapse M2 the chain **does** read this to decide whether
    /// to render one plane or two, so the declining default is the difference between a missed
    /// optimisation and wrong audio.
    fn lane_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
        let _ = lane;
        ChannelSymmetryWitness::DECLINED
    }

    /// Publish one fixed-width witness bank for this block's valid, non-identity lanes.
    ///
    /// The default preserves existing `lane_symmetry` overrides: its calls are made through
    /// the concrete implementation of this default, after one chain-to-stage virtual call.
    /// A processor boxed inside that implementation may still forward its own virtual queries.
    /// Lanes outside `width` or absent from `active_lanes` remain declined and are never queried.
    // REALTIME_POLICY_BEGIN
    fn lane_symmetry_bank(
        &self,
        width: BankWidth,
        active_lanes: u8,
    ) -> [ChannelSymmetryWitness; 8] {
        let mut witnesses = [ChannelSymmetryWitness::DECLINED; 8];
        for (lane, witness) in witnesses[..width.lanes() as usize].iter_mut().enumerate() {
            if active_lanes & (1u8 << lane) != 0 {
                *witness = self.lane_symmetry(lane);
            }
        }
        witnesses
    }
    // REALTIME_POLICY_END

    /// Read this stage's live cells and drain its observation queues, before any lane of the block
    /// is dispatched.
    ///
    /// # Why the drain is a separate call and not the first paragraph of [`process`](Self::process)
    ///
    /// An admitted record takes effect on the **first sample of the block that drains it** -- that
    /// is #137 E1's rule and every live-control gate in the tree rests on it. A record that writes
    /// one channel's upstream word also clears the channel-symmetry witness' `LIVE` term. Those two
    /// facts have to be observed in that order: if the collapse dispatch read the witness *before*
    /// the drain, a `ParameterChannel::Left` retarget admitted at block `N` would take effect on
    /// block `N`'s first sample while block `N` still ran collapsed -- and a collapsed block
    /// publishes the left plane on both channels, so the right channel would receive a retarget
    /// addressed to the left one. The bits would be wrong, in the audible direction, on the one
    /// block nobody would think to look at.
    ///
    /// So [`BankChain::run`] drains **every** slot first, then reads the witness, then gathers. The
    /// default is a no-op, which is what a live-control-free plan pays.
    ///
    /// # The block's live snapshot (#1504)
    ///
    /// `snapshot` is the revision `S` render read from the running plan's gate at the start of the
    /// block. A drain that reads live cells reads under it; an observation queue is a FIFO and
    /// ignores it. There is no default snapshot: [`BankChain::run`] hands the block's own to every
    /// slot, and a direct caller names the one it means.
    ///
    /// # Every block, silence included
    ///
    /// [`BankChain::run`] calls this once per block on every slot that has an active lane, whether
    /// or not the block's audio is silent. A stage may skip *processing* on silence; it may never
    /// skip this drain, because the watermark covers `S` once the block ends, and a cell holding a
    /// value at or below `S` that was never read would make it early.
    fn begin_block(
        &mut self,
        first_sample: u64,
        snapshot: LiveSnapshot,
    ) -> Result<(), RenderError> {
        let _ = (first_sample, snapshot);
        Ok(())
    }

    /// Apply the pending `Observe` records of the lanes a plan swap's carry marked, as this
    /// stage's next block would have at `first_sample` (#1280 D3, #1816). The predecessor plan
    /// never renders again, so a record admitted after its last block would otherwise be lost
    /// with the retired queue.
    ///
    /// [`BankChain::carry_observes`] calls it once on every slot, after the carry moved every
    /// state. It is the carry's entry into the stage's own drain loop, the one the block path
    /// runs: a stage that marks no lane drains nothing. The default is a stage with no
    /// observation queue. Allocation-, lock- and syscall-free.
    fn carry_observes(&mut self, first_sample: u64) {
        let _ = first_sample;
    }

    /// The lanes (bit `n` is lane `n`) whose [`begin_block`](Self::begin_block) drains a live
    /// lane, or `0` for a stage with none.
    ///
    /// [`BankChain::run`] skips a slot with no active lane, so a slot with a live lane must be
    /// active: [`BankChain::new`] asserts it in a debug build, off the render thread (#1504 D3).
    /// The default `0` is a stage that holds no live lane.
    fn live_lanes(&self) -> u8 {
        0
    }

    /// The lanes of this stage at their **exact rest** for the coming block, as a bitmask: bit
    /// `l` set means lane `l` is at rest (issue #1781; the consumer is #1107's whole-bank skip).
    ///
    /// # The claim (#1107 D3)
    ///
    /// Lane `l` is at rest when one block of exact `+0.0` on both planes of the lane, processed
    /// as [`process`](Self::process) would process it, would leave every state word of the lane
    /// bit-identical and write exact `+0.0` to both of its output planes. Raw bits: `-0.0` is not
    /// `+0.0`. A **clock** -- a word a `+0.0` block advances by the block's length alone, never by
    /// the signal -- is not a state word for the claim, as in
    /// `PreparedNativeEffectBank::lanes_at_rest`; [`advance_clocks`](Self::advance_clocks)
    /// advances it for a skipped block. Work the block itself brings -- a staged span or target,
    /// a timed operation, a crossfade to advance, a tap to accumulate -- clears the bit.
    ///
    /// # When it is asked
    ///
    /// At the block boundary on the render thread, after this block's
    /// [`begin_block`](Self::begin_block) (the drain) and, for a builtin stage, after the graph's
    /// `set_block_time`, and before [`process`](Self::process): the answer is for the block the
    /// drain prepared. From state only: `&self`, allocation-free, lock-free and bounded by the
    /// lane count and the prepared state size.
    ///
    /// A padded lane is answered on its own state; the padding contract feeds it `+0.0`.
    ///
    /// The default `0` declines every lane: a stage that has not written the predicate is never
    /// skipped, and a wrong set bit is the only answer that could be unsound.
    fn lanes_at_rest(&self) -> u32 {
        0
    }

    /// Advance the clocks of the lanes in `lanes_mask` exactly as [`process`](Self::process) on
    /// `block` would, for a block the caller skips (issue #1781; the consumer is #1107).
    ///
    /// # The caller's contract
    ///
    /// `block`'s planes hold exact `+0.0` on every lane (`frames * lanes` words each);
    /// `lanes_mask` is every lane of the bank, padded lanes included, because a bank is skipped
    /// whole; and every bit is one [`lanes_at_rest`](Self::lanes_at_rest) reported for this block,
    /// after its drain. The caller calls this once for each block it skips, in place of
    /// `process` or `process_mono`.
    ///
    /// # What it owes
    ///
    /// After the call the stage's state is bit-identical to what `process(block)` on that zero
    /// block would leave: the effect's clocks advanced, and anything the stage stages for a later
    /// block fed (a live stage's dry shunt line takes the `+0.0` block). It takes the block, not
    /// only `frames`, so a stage that stages its input for a later block feeds those words rather
    /// than synthesising them. The planes are not written: the caller already holds the `+0.0`
    /// output.
    ///
    /// Two exceptions, both outside the audio state a later block reads:
    ///
    /// * evidence counters of real processing -- the builtin processors' `[process calls,
    ///   frames processed]` qualification counters -- do not count a skipped block, because no
    ///   processing ran;
    /// * on a collapsed chain the call advances both channels, so a prefix stage's frozen right
    ///   channel (the input's right silence counter, for one) moves where `process_mono` would
    ///   have left it. Nothing reads it before the disengage copies the left channel over it, and
    ///   the swap and carry paths disengage first.
    ///
    /// The default is a no-op: exact for a stage with no clock and no staged line, and sound for
    /// any stage whose predicate declines, because a declined lane is never skipped. On the render
    /// thread at a block boundary: allocation-free, lock-free and bounded by the lane count and
    /// the staged line's length.
    fn advance_clocks(&mut self, lanes_mask: u32, block: BankBlock<'_>) {
        let _ = (lanes_mask, block);
    }

    /// Which side of the fader/matrix seam this stage sits on.
    ///
    /// The default is [`SeamSide::UpstreamOfSeam`], which is the **conservative** answer here and
    /// not the permissive one: an upstream stage is one a collapsed chain would have to run
    /// one-plane, so a stage that has not spoken is one the collapse must be able to run through --
    /// and it cannot, because [`supports_mono_collapse`](Self::supports_mono_collapse) defaults to
    /// `false`. An unclassified stage therefore ends the collapsible prefix (issue #974) -- and
    /// declines the whole chain when it is the first slot -- rather than being silently treated as
    /// seam-side and left to read a plane nobody gathered.
    fn seam_side(&self) -> SeamSide {
        SeamSide::UpstreamOfSeam
    }

    /// Whether this stage implements [`process_mono`](Self::process_mono) and
    /// [`desymmetrize`](Self::desymmetrize).
    ///
    /// Decided off the render thread, at bind, and cached by [`BankChain::new`]: the first upstream
    /// stage that answers `false` ends the collapsible prefix, and runs dual with every slot after
    /// it (issue #974). A chain whose first slot answers `false` can never collapse.
    fn supports_mono_collapse(&self) -> bool {
        false
    }

    /// Render one block with the cohort's two channels collapsed onto `block.left`.
    ///
    /// `block.right` is the resident scratch the chain did **not** gather. It holds the previous
    /// block's words and reading it is a defect; at the seam, before the scatter, the first
    /// seam-side slot writes it from the left plane, or the chain copies the left plane over it
    /// (see [`process_from_left`](Self::process_from_left)).
    ///
    /// Never called unless [`supports_mono_collapse`](Self::supports_mono_collapse) is `true`.
    ///
    /// # What a collapsed block owes besides the plane
    ///
    /// The plane is the visible half and the gates that cover it are digests. Everything a block
    /// publishes *other* than samples is the half no digest can see, and a collapsed body owes the
    /// dual body's answer there too:
    ///
    /// * **Observations.** A resident tap reads bank state, and a collapsed bank's right-channel
    ///   state is frozen at the moment the collapse engaged. The stage substitutes the left
    ///   channel's reading for the right channel's after the bank runs -- see
    ///   [`LiveControlEffectBankStage::process_mono`] -- because the right channel of a collapsed
    ///   track *is* its left channel at the tap exactly as at the fader.
    /// * **Reports and counters.** Per-channel accounting is duplicated from the left, and a total
    ///   that sums both channels is twice the left count. See
    ///   [`PreparedNativeEffectBank::process_bank_mono`], which states the rule for the effects,
    ///   and `builtins/tests/mono_collapse.rs`, which is the worked gate on it.
    /// * **Latency lines.** Anything a stage stages for a *later* block -- a dry shunt's delay line
    ///   is the one in this tree -- must be fed the left plane rather than the ungathered right
    ///   scratch, because the seam is downstream of the line and cannot repair it.
    ///   `LiveControlEffectBankStage::process_inner` carries that argument in full.
    ///
    /// The common shape of all three is that the collapse is invisible to samples and visible to
    /// state, so a stage whose only mono gate is a digest has not been gated.
    fn process_mono(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
        self.process(block)
    }

    /// Render one dual block whose right input is its left plane: a collapsed chain's first
    /// seam-side slot with an active lane (issue #975). `block.right` is output only; it holds
    /// stale words on entry.
    ///
    /// Returns `Ok(true)` when the stage wrote the right plane from the left one itself, and
    /// `Ok(false)` when it copied the left plane into the right one first, as this default does:
    /// [`BankChain::seam_copies`] counts the `false` answers. Either way the output must be bit for
    /// bit what that copy followed by [`process`](Self::process) renders: an override reads the
    /// left words where the dual body reads the right ones, in the same arithmetic and order.
    fn process_from_left(&mut self, block: BankBlock<'_>) -> Result<bool, RenderError> {
        block.right.copy_from_slice(block.left);
        self.process(block)?;
        Ok(false)
    }

    /// Copy every lane's left-channel state onto the right channel.
    ///
    /// The collapse's disengage boundary. See
    /// [`PreparedNativeEffectBank::desymmetrize_channels`] for what the copy has to cover and why.
    fn desymmetrize(&mut self) {}

    /// Whether this stage can **prove**, right now, that its two channels' state is bit-equal.
    ///
    /// The re-engage rule's way back for a chain whose channels have already been driven apart.
    /// See [`PreparedNativeEffectBank::channels_agree`] for the contract, the cost rule and what
    /// each shipped effect proves it with; and [`BankChain::run`] for the one place it is asked.
    ///
    /// The default is `false`, for the same reason every default on this surface is the declining
    /// one: a stage that cannot prove equality must not be believed to have it, and a wrong `true`
    /// here re-engages a collapse onto a right channel that is not the left one.
    fn channels_agree(&self) -> bool {
        false
    }

    /// This stage as `Any`, so the crate that built it can reach its own stage type through
    /// [`BankChain::slot_stage_mut`] -- at bind, and at a plan swap's carry (issue #1276 D3).
    /// `None` (the default) offers nothing. Allocation-free.
    fn as_any_mut(&mut self) -> Option<&mut dyn core::any::Any> {
        None
    }

    /// This stage as `Any`, read-only: what [`Self::can_swap_lanes`] downcasts a peer stage
    /// through (issue #1747). `None` (the default) offers nothing. Allocation-free.
    fn as_any(&self) -> Option<&dyn core::any::Any> {
        None
    }

    /// The scratch bytes [`Self::swap_lanes`] needs, or `0` for a stage that cannot swap.
    fn lane_swap_scratch_bytes(&self) -> usize {
        0
    }

    /// Whether [`Self::swap_lanes`] would exchange lane `lane`'s whole state with `peer`'s lane,
    /// with a scratch of `scratch_bytes`. Decided without mutation, allocation-free and bounded.
    /// The default `false` is the stage that has no swap (issue #1747).
    fn can_swap_lanes(&self, lane: usize, peer: LanePeer<'_>, scratch_bytes: usize) -> bool {
        let _ = (lane, peer, scratch_bytes);
        false
    }

    /// Exchange lane `lane`'s whole per-lane state -- effect state, ramps in flight, control lane,
    /// taps, dry history -- with `peer`'s lane, at a block boundary on the render thread (issue
    /// #1747). Returns `false` and changes nothing unless [`Self::can_swap_lanes`] holds.
    /// Allocation-free and bounded by the prepared state size and the latency.
    fn swap_lanes(&mut self, lane: usize, peer: LanePeerMut<'_>, scratch: &mut [u8]) -> bool {
        let _ = (lane, peer, scratch);
        false
    }
}

/// The other lane of a lane swap, read-only: [`BankStage::can_swap_lanes`] (issue #1747).
#[derive(Clone, Copy)]
pub enum LanePeer<'a> {
    /// Another lane of the same stage.
    Same(usize),
    /// A lane of another stage.
    Other(&'a dyn BankStage, usize),
}

/// The other lane of a lane swap: [`BankStage::swap_lanes`] (issue #1747).
pub enum LanePeerMut<'a> {
    /// Another lane of the same stage.
    Same(usize),
    /// A lane of another stage.
    Other(&'a mut dyn BankStage, usize),
}

impl LanePeerMut<'_> {
    /// The same peer, read-only, for the check a swap makes before it writes.
    #[must_use]
    pub fn as_peer(&self) -> LanePeer<'_> {
        match self {
            Self::Same(lane) => LanePeer::Same(*lane),
            Self::Other(stage, lane) => LanePeer::Other(&**stage, *lane),
        }
    }
}

/// The scratch a lane swap of `processor`'s lanes under `layout` needs
/// ([`effect_contract::swap_bank_lanes_scratch_bytes`]: two payloads and one lane's prepared
/// defaults), or `None` -- the stage cannot swap -- when the bank has not declared its defaults or
/// the size overflows.
fn lane_swap_scratch(
    processor: &dyn PreparedNativeEffectBank,
    layout: &EffectLaneLayout,
) -> Option<usize> {
    effect_contract::swap_bank_lanes_scratch_bytes(processor, layout.state_sizes)
}

/// Whether `processor` can swap a lane with `peer`'s with a scratch of `scratch_bytes`: it
/// declares its defaults, `peer` declares the same size, and the scratch holds
/// [`lane_swap_scratch`]. The check [`effect_contract::swap_bank_lanes`] would refuse
/// ([`effect_contract::LaneSwapRefusal::Defaults`] or `Scratch`), made before any write.
fn lane_swap_scratch_fits(
    processor: &dyn PreparedNativeEffectBank,
    peer: &dyn PreparedNativeEffectBank,
    layout: &EffectLaneLayout,
    scratch_bytes: usize,
) -> bool {
    processor.lane_default_bytes() == peer.lane_default_bytes()
        && lane_swap_scratch(processor, layout).is_some_and(|needed| scratch_bytes >= needed)
}

// REALTIME_POLICY_BEGIN
/// Two distinct elements of `slice`, mutably; `None` when `first == second` or either is out of
/// range.
fn two_mut<T>(slice: &mut [T], first: usize, second: usize) -> Option<(&mut T, &mut T)> {
    if first < second {
        let (low, high) = slice.split_at_mut_checked(second)?;
        Some((low.get_mut(first)?, high.first_mut()?))
    } else if second < first {
        let (low, high) = slice.split_at_mut_checked(first)?;
        Some((high.first_mut()?, low.get_mut(second)?))
    } else {
        None
    }
}

/// Exchange bits `first` and `second` of `bits`.
fn swap_bits_within(bits: &mut u8, first: usize, second: usize) {
    let (first_bit, second_bit) = ((*bits >> first) & 1, (*bits >> second) & 1);
    *bits =
        (*bits & !((1 << first) | (1 << second))) | (second_bit << first) | (first_bit << second);
}

/// Exchange bit `first` of `own` with bit `second` of `other`.
fn swap_bits(own: &mut u8, first: usize, other: &mut u8, second: usize) {
    let (own_bit, other_bit) = ((*own >> first) & 1, (*other >> second) & 1);
    *own = (*own & !(1 << first)) | (other_bit << first);
    *other = (*other & !(1 << second)) | (own_bit << second);
}
// REALTIME_POLICY_END

/// What two bank lanes must share before a plan swap moves the state of one into the other
/// (#1279 D1): the effect, the layout of its state payload, its prepared bypass and its latency.
///
/// Read off the control side when a stage is built, so the successor's preparation can compare it
/// with the predecessor's without touching a bank, and the swap block can check it again before it
/// restores anything.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectLaneLayout {
    /// The native effect the lane runs.
    pub effect_id: EffectId,
    /// The descriptor's state payload layout version.
    pub state_layout_version: u32,
    /// The prepared sizes of the three payload sections.
    pub state_sizes: StatePayloadSizes,
    /// The bank's prepared bypass.
    pub bypass: bool,
    /// The bank's prepared latency.
    pub latency: LatencySamples,
}

impl EffectLaneLayout {
    /// The layout a bank bound with `key` has.
    #[must_use]
    pub const fn of(key: &EffectProgramKey) -> Self {
        Self {
            effect_id: key.effect_id,
            state_layout_version: key.state_layout_version,
            state_sizes: key.state_sizes,
            bypass: key.bypass,
            latency: key.latency,
        }
    }

    /// The bytes one lane's payload takes: the three sections together.
    #[must_use]
    pub fn payload_bytes(&self) -> u64 {
        self.state_sizes.total().unwrap_or(u64::MAX)
    }
}

/// What kind of stage and control lane an effect bank lane has (#1280): a plan swap moves a lane
/// into a successor lane of the same kind only. The kinds differ in what the lane holds besides
/// the effect's own state, so a pair of unlike kinds has nothing to join.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EffectLaneKind {
    /// A lane of an [`EffectBankStage`]: no live control and no lowered bypass.
    Plain,
    /// A lane of a [`LiveControlEffectBankStage`] that has no control lane: neither a live channel
    /// nor a lowered bypass applies to it.
    Uncontrolled,
    /// A lane of a [`LiveControlEffectBankStage`] whose control lane has no cells: it carries a
    /// session bypass lowered at preparation and nothing a control plane can write.
    BypassOnly,
    /// A lane of a [`LiveControlEffectBankStage`] with live cells (and a bypass cell).
    Cells,
}

impl EffectLaneKind {
    /// The kind of a per-node live-controlled instance, from its control lane: a live channel
    /// makes it [`Self::Cells`], and a lane with none [`Self::BypassOnly`] (#1281). A per-node
    /// instance always has a control lane, so it is never [`Self::Uncontrolled`].
    #[must_use]
    pub const fn of_instance(control: &EffectControlLane) -> Self {
        if control.has_channel() {
            Self::Cells
        } else {
            Self::BypassOnly
        }
    }
}

/// A per-node live-controlled effect instance, as a plan swap reads it to move its state into a
/// bank lane (#1281): the prepared processor, its control lane and its dry shunt.
pub struct EffectInstance<'a> {
    pub processor: &'a dyn PreparedNativeEffect,
    pub control: &'a EffectControlLane,
    pub shunt: &'a BypassShunt,
}

/// A per-node live-controlled effect instance, as a plan swap writes it to take the state of a
/// bank lane (#1281).
pub struct EffectInstanceMut<'a> {
    pub processor: &'a mut dyn PreparedNativeEffect,
    pub control: &'a mut EffectControlLane,
    pub shunt: &'a mut BypassShunt,
}

// REALTIME_POLICY_BEGIN
/// The three sections of a payload in `scratch`, cut at the prepared `sizes`; `None` when the
/// scratch is short.
fn payload_sections(
    scratch: &mut [u8],
    sizes: StatePayloadSizes,
) -> Option<(&mut [u8], &mut [u8], &mut [u8])> {
    let (common, rest) = scratch.split_at_mut_checked(sizes.common_bytes as usize)?;
    let (left, rest) = rest.split_at_mut_checked(sizes.left_bytes as usize)?;
    let right = rest.get_mut(..sizes.right_bytes as usize)?;
    Some((common, left, right))
}

/// The payload move of a plan swap's effect carry (#1279 D3), for both stage types: snapshot
/// `predecessor`'s lane `predecessor_lane` into `scratch` and restore it into `into`'s lane
/// `lane`. Returns whether the restored payload's left and right sections agree, or `None` when a
/// step refused (a restore validates the whole payload before it commits, so a refused one changes
/// nothing). Allocation-free and bounded by the prepared state size.
fn carry_payload(
    into: &mut dyn PreparedNativeEffectBank,
    lane: usize,
    predecessor: &dyn PreparedNativeEffectBank,
    predecessor_lane: usize,
    layout: &EffectLaneLayout,
    scratch: &mut [u8],
) -> Option<bool> {
    let sizes = layout.state_sizes;
    let (common, left, right) = payload_sections(scratch, sizes)?;
    predecessor
        .snapshot_track_state_payload(
            u32::try_from(predecessor_lane).ok()?,
            StatePayloadOutput::new(common, left, right, sizes).ok()?,
        )
        .ok()?;
    let (common, left, right): (&[u8], &[u8], &[u8]) = (common, left, right);
    into.restore_track_state_payload(
        u32::try_from(lane).ok()?,
        layout.state_layout_version,
        StatePayloadInput::new(common, left, right, sizes).ok()?,
    )
    .ok()?;
    Some(effect_contract::payload_sections_agree(left, right))
}

/// [`carry_payload`] from a per-node instance into a bank lane (#1281 D2): the instance's
/// `snapshot_state_payload` goes into `scratch`, and the bank's `restore_track_state_payload`
/// takes it. The two payloads share one layout (#1278).
fn carry_payload_from_instance(
    into: &mut dyn PreparedNativeEffectBank,
    lane: usize,
    from: &dyn PreparedNativeEffect,
    layout: &EffectLaneLayout,
    scratch: &mut [u8],
) -> Option<bool> {
    let sizes = layout.state_sizes;
    let (common, left, right) = payload_sections(scratch, sizes)?;
    from.snapshot_state_payload(StatePayloadOutput::new(common, left, right, sizes).ok()?)
        .ok()?;
    let (common, left, right): (&[u8], &[u8], &[u8]) = (common, left, right);
    into.restore_track_state_payload(
        u32::try_from(lane).ok()?,
        layout.state_layout_version,
        StatePayloadInput::new(common, left, right, sizes).ok()?,
    )
    .ok()?;
    Some(effect_contract::payload_sections_agree(left, right))
}

/// [`carry_payload`] from a bank lane into a per-node instance (#1281 D2): the bank's
/// `snapshot_track_state_payload` goes into `scratch`, and the instance's `restore_state_payload`
/// takes it.
fn carry_payload_into_instance(
    into: &mut dyn PreparedNativeEffect,
    from: &dyn PreparedNativeEffectBank,
    from_lane: usize,
    layout: &EffectLaneLayout,
    scratch: &mut [u8],
) -> Option<()> {
    let sizes = layout.state_sizes;
    let (common, left, right) = payload_sections(scratch, sizes)?;
    from.snapshot_track_state_payload(
        u32::try_from(from_lane).ok()?,
        StatePayloadOutput::new(common, left, right, sizes).ok()?,
    )
    .ok()?;
    let (common, left, right): (&[u8], &[u8], &[u8]) = (common, left, right);
    into.restore_state_payload(
        layout.state_layout_version,
        StatePayloadInput::new(common, left, right, sizes).ok()?,
    )
    .ok()
}
// REALTIME_POLICY_END

/// Adapter from the effect contract's prepared homogeneous bank to a chain stage.
///
/// Width, quantum and the automation offsets are fixed here once (#96 F8): the render path reads
/// no metadata and performs no `checked_add`. The bank's [`effect_contract::PreparedBankMetadata`]
/// is read at bind, on the control thread, and only the values a stage method reads -- the
/// prepared bypass, and the lane layout a plan swap checks -- are kept, as plain fields (issue
/// #1461, #1279).
pub struct EffectBankStage {
    processor: Box<dyn PreparedNativeEffectBank>,
    /// The bank's prepared `bypass`, which the response snapshot reports.
    bypassed: bool,
    width: BankWidth,
    quantum: u32,
    offsets: Box<[u32]>,
    /// One witness per lane: the terms this stage decides itself, taken at bind and refreshed
    /// after a plan swap restores a lane.
    ///
    /// # Why this is cached and why the cache cannot go stale
    ///
    /// `PreparedNativeEffectBank::lane_channel_symmetry` is a walk over every designed word the
    /// kernel reads -- eight coefficients and five four-field ramps per lane for the compressor,
    /// twenty-six words for the input chain. Pulling it once per lane per slot per block, which is
    /// what the collapse's dispatch needs, would cost more than the collapse saves.
    ///
    /// The cache is sound because the designed words of a **bound** bank move at exactly one
    /// boundary, and it refreshes the cache. A live-control-free slot hands the bank an empty
    /// automation slice on every block (`EffectBankStage::process`), so a block moves none, and
    /// `reset` is preparation-side and unreachable once a processor has been moved into a chain.
    /// `restore_track_state_payload` runs on a bound bank in one place: the plan-swap carry's swap
    /// block ([`EffectBankStage::carry_lane_from`], #1279). It overwrites a lane's designed words
    /// with the predecessor's, and it recomputes both terms this stage holds for the lane: the
    /// `DESIGNED` term from the bank and the `RESTORED` term from the payload it restored.
    /// `LiveControlEffectBankStage` is the slot that *does* drain writes, and it caches the same
    /// half for the same reason: a one-channel write clears the `LIVE` term, which that slot pulls
    /// live, and a `Both` write leaves the two channels bit-equal.
    witness: Box<[ChannelSymmetryWitness]>,
    /// The layout a plan swap checks before it moves a lane in or out.
    layout: EffectLaneLayout,
    /// The bank's whole program key, which a lane swap requires to be equal in both stages
    /// (issue #1747): the layout alone misses the link mode, quality, ports and automation
    /// capacity, any of which makes one lane's state mean something else in the other bank.
    program_key: EffectProgramKey,
    /// Restores a plan swap's carry refused on this stage: each left its lane as prepared, at rest
    /// (#1279 D6). Saturating; read after render.
    carry_refused: u64,
}

impl EffectBankStage {
    /// Errors with [`RackError::WidthMismatch`] if the bound bank is not this width.
    pub fn new(
        bank: PreparedEffectBank,
        width: BankWidth,
        quantum: u32,
    ) -> Result<Self, RackError> {
        let PreparedEffectBank {
            processor,
            metadata,
        } = bank;
        if metadata.width != width {
            return Err(RackError::WidthMismatch);
        }
        if quantum == 0 {
            return Err(RackError::ZeroQuantum);
        }
        let witness = (0..width.lanes() as usize)
            .map(|lane| lane_witness(processor.lane_channel_symmetry(lane), true))
            .collect();
        Ok(Self {
            processor,
            bypassed: metadata.program_key.bypass,
            width,
            quantum,
            offsets: vec![0_u32; width.lanes() as usize + 1].into_boxed_slice(),
            witness,
            layout: EffectLaneLayout::of(&metadata.program_key),
            program_key: metadata.program_key,
            carry_refused: 0,
        })
    }

    /// The layout a plan swap checks before it moves a lane in or out (#1279 D1).
    #[must_use]
    pub const fn layout(&self) -> EffectLaneLayout {
        self.layout
    }

    /// The bank's lanes, populated or padded.
    #[must_use]
    pub const fn lanes(&self) -> usize {
        self.width.lanes() as usize
    }

    /// Restores a plan swap's carry refused on this stage (#1279 D6). Saturating.
    #[must_use]
    pub const fn carry_refused(&self) -> u64 {
        self.carry_refused
    }

    // REALTIME_POLICY_BEGIN
    /// Whether [`Self::carry_lane_from`] would take `predecessor`'s lane `predecessor_lane` into
    /// this stage's lane `lane` with a scratch of `scratch_bytes`: both lanes exist, both banks
    /// run one layout, and the scratch holds the payload. Allocation-free and bounded.
    #[must_use]
    pub fn can_carry_lane_from(
        &self,
        lane: usize,
        predecessor: &Self,
        predecessor_lane: usize,
        scratch_bytes: usize,
    ) -> bool {
        lane < self.lanes()
            && predecessor_lane < predecessor.lanes()
            && self.layout == predecessor.layout
            && usize::try_from(self.layout.payload_bytes())
                .is_ok_and(|payload| payload <= scratch_bytes)
    }

    /// Move `predecessor`'s lane `predecessor_lane` -- its whole state payload, a ramp in flight
    /// included -- into this stage's lane `lane`, through `scratch`, at the swap block (#1279 D3):
    /// snapshot the predecessor lane, restore it here, then recompute the witness this stage holds
    /// for the lane (D5). Returns whether the lane was taken.
    ///
    /// The caller has checked [`Self::can_carry_lane_from`], and has disengaged `predecessor`'s
    /// chain ([`BankChain::disengage_for_carry`]) so a collapsed bank's right channel holds the
    /// state a dual run would (D4): the payload is then the one a dual run writes, and the restore
    /// continues it bit for bit.
    ///
    /// A snapshot or a restore that refuses leaves this lane as prepared, at rest -- a restore
    /// validates the whole payload before it commits, so a refused one changes nothing -- and
    /// counts in [`Self::carry_refused`] (D6). An own payload is never refused, so a count above
    /// zero is a defect, and it never panics here. Allocation-free and bounded by the prepared
    /// state size.
    pub fn carry_lane_from(
        &mut self,
        lane: usize,
        predecessor: &Self,
        predecessor_lane: usize,
        scratch: &mut [u8],
    ) -> bool {
        let taken = if self.can_carry_lane_from(lane, predecessor, predecessor_lane, scratch.len())
        {
            carry_payload(
                &mut *self.processor,
                lane,
                &*predecessor.processor,
                predecessor_lane,
                &self.layout,
                scratch,
            )
        } else {
            None
        };
        match taken {
            Some(restored) => {
                self.witness[lane] =
                    lane_witness(self.processor.lane_channel_symmetry(lane), restored);
                true
            }
            None => {
                self.carry_refused = self.carry_refused.saturating_add(1);
                false
            }
        }
    }
    // REALTIME_POLICY_END
}

impl BankStage for EffectBankStage {
    fn copy_response_snapshot_lane(
        &self,
        lane: usize,
        _sample_rate_hz: u32,
        request: effect_contract::ResponseSnapshotRequest<'_>,
    ) -> Result<effect_contract::ResponseSnapshotSummary, effect_contract::ResponseAnalysisError>
    {
        let effect_contract::ResponseSnapshotRequest { left, right, .. } = request;
        self.processor.copy_response_snapshot_lane(
            lane,
            effect_contract::ResponseSnapshotRequest {
                bypassed: self.bypassed,
                left,
                right,
            },
        )
    }

    fn response_snapshot_bypassed(&self, _lane: usize) -> bool {
        self.bypassed
    }

    /// A live-control-free bank has no live channel at all, so the two live terms cannot be false
    /// and the whole witness is the effect's own designed-word comparison.
    fn lane_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
        self.witness
            .get(lane)
            .copied()
            .unwrap_or(ChannelSymmetryWitness::DECLINED)
    }

    /// Every prepared effect sits in `simd1`, `dynamic` or `simd2`, all three upstream of the
    /// fader (`TrackStage` order), so an effect slot is never seam-side.
    fn seam_side(&self) -> SeamSide {
        SeamSide::UpstreamOfSeam
    }

    // REALTIME_POLICY_BEGIN
    /// The effect's own mask, forwarded (issue #1781). This stage hands the bank no span, no
    /// target and no sidechain on any block, so its block is exactly the one
    /// `PreparedNativeEffectBank::lanes_at_rest` answers for. The prepared `bypass` this stage
    /// keeps is the bank's own, from its program key, and is read here only by the response
    /// snapshot: the bank's `process_bank` runs its prepared bypass path itself, so the effect's
    /// predicate already answers for it, and no lane needs declining here.
    fn lanes_at_rest(&self) -> u32 {
        self.processor.lanes_at_rest()
    }

    /// The effect's clocks, forwarded with the block's length (issue #1781). The stage stages
    /// nothing for a later block.
    fn advance_clocks(&mut self, lanes_mask: u32, block: BankBlock<'_>) {
        self.processor.advance_clocks(lanes_mask, block.frames);
    }
    // REALTIME_POLICY_END

    fn supports_mono_collapse(&self) -> bool {
        self.processor.supports_mono_collapse()
    }

    fn desymmetrize(&mut self) {
        self.processor.desymmetrize_channels();
    }

    fn channels_agree(&self) -> bool {
        self.processor.channels_agree()
    }

    fn as_any_mut(&mut self) -> Option<&mut dyn core::any::Any> {
        Some(self)
    }

    fn as_any(&self) -> Option<&dyn core::any::Any> {
        Some(self)
    }

    fn lane_swap_scratch_bytes(&self) -> usize {
        lane_swap_scratch(&*self.processor, &self.layout).unwrap_or(0)
    }

    // REALTIME_POLICY_BEGIN
    /// Both lanes exist, the peer is an [`EffectBankStage`] of this program key (not only this
    /// layout: link mode, quality, ports and automation capacity too), both banks declare
    /// prepared defaults of one size, and the scratch holds
    /// [`effect_contract::swap_bank_lanes_scratch_bytes`].
    fn can_swap_lanes(&self, lane: usize, peer: LanePeer<'_>, scratch_bytes: usize) -> bool {
        lane < self.lanes()
            && match peer {
                LanePeer::Same(other_lane) => {
                    other_lane < self.lanes()
                        && lane_swap_scratch_fits(
                            &*self.processor,
                            &*self.processor,
                            &self.layout,
                            scratch_bytes,
                        )
                }
                LanePeer::Other(stage, other_lane) => stage
                    .as_any()
                    .and_then(|any| any.downcast_ref::<Self>())
                    .is_some_and(|other| {
                        other_lane < other.lanes()
                            && other.program_key == self.program_key
                            && lane_swap_scratch_fits(
                                &*self.processor,
                                &*other.processor,
                                &self.layout,
                                scratch_bytes,
                            )
                    }),
            }
    }

    /// The lane's whole state payload (running state, designed words, every ramp in flight)
    /// through [`effect_contract::swap_bank_lanes`], then the witness this stage caches for each
    /// lane, which describes that state and so moves with it. Every other field is per bank:
    /// the processor's shared words, the width, quantum, zero offsets, prepared bypass, layout,
    /// program key and the carry counter.
    fn swap_lanes(&mut self, lane: usize, peer: LanePeerMut<'_>, scratch: &mut [u8]) -> bool {
        if !self.can_swap_lanes(lane, peer.as_peer(), scratch.len()) {
            return false;
        }
        let (version, sizes) = (self.layout.state_layout_version, self.layout.state_sizes);
        match peer {
            LanePeerMut::Same(other_lane) => {
                if effect_contract::swap_bank_lanes(
                    &mut *self.processor,
                    lane,
                    None,
                    other_lane,
                    version,
                    sizes,
                    scratch,
                )
                .is_err()
                {
                    return false;
                }
                self.witness.swap(lane, other_lane);
            }
            LanePeerMut::Other(stage, other_lane) => {
                let Some(other) = stage
                    .as_any_mut()
                    .and_then(|any| any.downcast_mut::<Self>())
                else {
                    return false;
                };
                if effect_contract::swap_bank_lanes(
                    &mut *self.processor,
                    lane,
                    Some(&mut *other.processor),
                    other_lane,
                    version,
                    sizes,
                    scratch,
                )
                .is_err()
                {
                    return false;
                }
                core::mem::swap(&mut self.witness[lane], &mut other.witness[other_lane]);
            }
        }
        true
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    fn process_mono(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
        let block = EffectBankProcessBlock::new(
            block.left,
            block.right,
            None,
            block.frames,
            self.width,
            block.first_sample,
            &[],
            &self.offsets,
            self.quantum,
        )
        .map_err(|_| RenderError::InvalidEnvelope)?;
        let _ = self.processor.process_bank_mono(block);
        Ok(())
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
        let block = EffectBankProcessBlock::new(
            block.left,
            block.right,
            None,
            block.frames,
            self.width,
            block.first_sample,
            &[],
            &self.offsets,
            self.quantum,
        )
        .map_err(|_| RenderError::InvalidEnvelope)?;
        // D7's once-per-block boundary check and its counters belong to effect-runtime (#95); the
        // descriptive bank report is deliberately dropped here, exactly as it was before #96.
        let _ = self.processor.process_bank(block);
        Ok(())
    }
    // REALTIME_POLICY_END
}

/// The same witness from an already-taken designed-word comparison.
///
/// One body, so a cached answer and a freshly pulled one cannot disagree about which term a
/// `false` clears.
#[must_use]
pub const fn witness_of_designed(designed: bool) -> ChannelSymmetryWitness {
    if designed {
        ChannelSymmetryWitness::SYMMETRIC
    } else {
        ChannelSymmetryWitness::symmetric_except(ChannelSymmetryWitness::DESIGNED)
    }
}

/// The witness of a live-control-free bank lane: the designed-word comparison, and whether the
/// lane's last restored payload had equal left and right sections (`true` for a lane never
/// restored). The two terms an [`EffectBankStage`] decides for itself; the rest hold, or are
/// decided elsewhere.
const fn lane_witness(designed: bool, restored: bool) -> ChannelSymmetryWitness {
    let witness = witness_of_designed(designed);
    if restored {
        witness
    } else {
        witness.and(ChannelSymmetryWitness::symmetric_except(
            ChannelSymmetryWitness::RESTORED,
        ))
    }
}

/// The live-control twin of [`EffectBankStage`] (issue #140 A), and the stage of every bank slot
/// with a lane bypassed at preparation (issue #1087).
///
/// It is a **separate stage type** on purpose, exactly as `LiveControlMatrixProcessor` is a
/// separate processor from `MatrixProcessor` (#137 D1): a bank prepared without live controls and
/// without a bypassed lane keeps [`EffectBankStage`]'s storage and its `&[]`/zero-offset call, byte
/// for byte, so "control off costs nothing" stays an identity rather than a claim. Nothing in this
/// type is reachable from a session that asked for no control channel and bypasses nothing.
///
/// # A session bypass is a lane of this stage (issue #1087)
///
/// A session's `bypass`, on every effect (the multiband's since #1340), is lowered to a prepared `bypass = false` plus a lane
/// ([`EffectControlLane::without_channel`] when no live controls are attached) whose bypass is set,
/// so a bypassed track keeps its bank. Its lane runs the wet path with the others and step 4
/// mixes in its latency-matched dry signal: once settled, the same words the effect's own
/// prepared bypass emits, by a bit select, so `-0.0` survives; while a live switch ramps, the
/// per-node crossfade `dry + (wet - dry) * m(k)` along the lane's own ramp (#1341). The mix and the
/// select are per lane, so banking couples the lanes' cost and never their bits.
///
/// # What one block does, in order
///
/// 1. Read every lane's pending cells under the block's snapshot into that lane's window of the
///    packed span array, in canonical order (`EffectControlLane::stage`), then drain every
///    lane's observation queue. This happens **before** any audio is touched, so a committed
///    value takes effect on the first sample of this block -- the same block-boundary rule the
///    matrix stage proved for #137's E1.
/// 2. Capture the dry AoSoA block into the latency shunt, if any lane can be bypassed.
/// 3. Run the bank once with the packed spans and the per-lane offsets the effect contract
///    already defines (`EffectBankProcessBlock::automation_offsets`).
/// 4. Mix the latency-matched dry signal into exactly the bypassed or ramping lanes, each along its
///    own crossfade (`BypassShunt::crossfade_bank`).
///
/// # Partition invariance and `to_bits` identity extend to command timelines
///
/// The spans a lane receives are a pure function of that lane's own cells, the block's snapshot
/// and `first_sample`, and every lane is staged independently into a disjoint window. A bank therefore
/// applies a command timeline to lane `l` exactly as the per-node scalar path applies it to the
/// same effect: same spans, same block, same order.
pub struct LiveControlEffectBankStage {
    processor: Box<dyn PreparedNativeEffectBank>,
    width: BankWidth,
    quantum: u32,
    /// `lanes + 1` packed offsets into [`Self::spans`], rewritten every block.
    offsets: Box<[u32]>,
    /// One control lane per bank lane: a live channel, a channel-less lane that carries a
    /// session bypass (issue #1087), or `None` for a lane neither applies to.
    lanes: Box<[Option<EffectControlLane>]>,
    /// The observation queue of each lane that has one, with the lane's index, taken out of the
    /// lane at bind so that the drain loop holds each pop in its own body (decision 15, #1345 D8).
    observe_queues: Box<[(usize, Consumer<EffectControlRecord>)]>,
    /// The lanes (bit `n` is lane `n`) a plan swap's carry marked for its pending-`Observe` drain
    /// (#1816), taken by [`BankStage::carry_observes`]. Always `0` while the plan renders.
    carry_lanes: u8,
    /// One lane's staging window: the widest lane's live parameter cell count (at most the bank's
    /// `automation_capacity`), or none when no lane has a parameter cell (issue #1087; #1611 D3).
    ///
    /// One window serves every lane because a lane's staged prefix is copied into [`Self::packed`]
    /// the moment it is drained, before the next lane touches the window. The bank never sees this
    /// array; it sees `packed[..offsets[lanes]]`.
    staging: Box<[PreparedAutomationSpan]>,
    /// The packed, per-lane-partitioned span array the bank is handed. Lane `l` owns
    /// `[offsets[l], offsets[l + 1])`, which is the partition the effect contract already defines.
    packed: Box<[PreparedAutomationSpan]>,
    /// Latency-preserving dry shunt over the resident AoSoA block, or `None` when no lane of this
    /// slot can be bypassed: no live channel and no lane bypassed at preparation.
    shunt: Option<BypassShunt>,
    /// This block's per-lane bypass crossfade, rewritten by the process body before the shunt
    /// mixes (#1341 D4). Inline: a bank of at most eight lanes holds no allocation for it.
    crossfade: BankCrossfade,
    /// Issue #143 D3: one observation lane per bank lane, or `None` for the whole slot when the
    /// plan named no observation capacity. `None` is the byte-identical unobserved path.
    observations: Option<Box<[Option<ObservationLane>]>>,
    /// One reading per lane, filled by a single `observe_resident_bank` call per armed tap.
    ///
    /// Allocated at bind and only when the slot is observed at all, so an unobserved slot holds
    /// nothing and an observed one allocates nothing per block.
    samples: Box<[ObservationSample]>,
    /// One flag per lane: the effect's designed-word comparison, taken once at bind.
    ///
    /// See [`EffectBankStage::designed`] for why it is cached and why a drained write cannot make
    /// it stale -- a one-channel write clears the `LIVE` term this slot still pulls live.
    designed: Box<[bool]>,
    /// Spans this block's drain packed into [`Self::packed`]. Written by
    /// [`LiveControlEffectBankStage::drain`], read by the process body that follows it in the same
    /// block.
    staged_spans: usize,
    /// What this block's drain found over the lanes, read by the process body that follows it in
    /// the same block (issue #1763).
    walk: LaneWalk,
    /// The snapshot publication of the last drain that read every lane and found the bank quiet
    /// ([`LaneWalk::quiet`]), or `None`. A drain under the same publication skips the lane walk:
    /// see [`LiveControlEffectBankStage::drain`]. Every method that changes a lane outside the
    /// drain clears it.
    quiet_at: Option<u64>,
    /// `Observe` records this slot had no capacity to apply. Zero by construction.
    unbound: u64,
    /// One bit per lane, set when the lane's last restored payload had left and right sections that
    /// differ (clear for a lane never restored): the `RESTORED` term of the witness, held as
    /// [`EffectBankStage`] holds it and set by a plan swap's carry. A bank has at most eight
    /// lanes, so a byte holds it and the stage allocates nothing more.
    unrestored: u8,
    /// The layout a plan swap checks before it moves a lane in or out (#1280).
    layout: EffectLaneLayout,
    /// The bank's whole program key, which a lane swap requires to be equal in both stages
    /// (issue #1747; see [`EffectBankStage`]'s field of the same name).
    program_key: EffectProgramKey,
    /// Restores a plan swap's carry refused on this stage: each left its lane as prepared, at rest
    /// (#1279 D6). Saturating; read after render.
    carry_refused: u64,
}

impl LiveControlEffectBankStage {
    /// Builds the live-control stage for one bound bank slot.
    ///
    /// `latency` is the slot's declared [`effect_contract::PreparedEffectMetadata::latency`]. Every lane of a bank
    /// shares one [`EffectProgramKey`], so they share one latency and one AoSoA delay line:
    /// delaying the interleaved plane by `latency * lanes` words delays every lane by exactly
    /// `latency` frames.
    ///
    /// # Errors
    ///
    /// [`RackError::WidthMismatch`] if the processor is not this width or the lane count disagrees,
    /// [`RackError::ZeroQuantum`] for a zero quantum, and [`RackError::AutomationWindow`] if a
    /// lane has more live parameter cells than the bank's automation capacity -- the window that
    /// holds them would pass the cut-off the live drain's pairing rule depends on (issue #1012) --
    /// refused here, at bind, rather than discovered on the render thread.
    pub fn new(
        bank: PreparedEffectBank,
        width: BankWidth,
        quantum: u32,
        lanes: Vec<Option<EffectControlLane>>,
        observations: Vec<Option<ObservationLane>>,
        latency: usize,
    ) -> Result<Self, RackError> {
        let mut lanes = lanes;
        // Issue #1461: the bank's metadata is read here, at bind, and only what a stage method
        // reads is kept: the lane layout a plan swap checks (#1280).
        let PreparedEffectBank {
            processor,
            metadata,
        } = bank;
        if metadata.width != width
            || lanes.len() != width.lanes() as usize
            || observations.len() != width.lanes() as usize
        {
            return Err(RackError::WidthMismatch);
        }
        if quantum == 0 {
            return Err(RackError::ZeroQuantum);
        }
        let layout = EffectLaneLayout::of(&metadata.program_key);
        let lane_count = width.lanes() as usize;
        // Issue #1087: only a live channel ever stages a span. A slot whose lanes are all
        // channel-less -- session bypasses with no live controls attached -- holds no window at
        // all: its drain stages nothing into an empty one, and there is no twin pair for the
        // pairing rule to protect.
        //
        // #1611 D3: the window is what the lanes can stage, not the bank's `automation_capacity`.
        // A lane stages at most one span per live parameter cell
        // ([`EffectControlLane::parameter_cells`]), so the window is the widest lane's cell count,
        // and the packed array holds one such window per lane. A prepared-target owner (the
        // parametric EQ) has no parameter cell -- its parameters ride its targets -- so its slot
        // holds no span storage at all, however many parameters its descriptor declares. The window
        // never exceeds the capacity (checked here), so a staged twin pair is never cut off by the
        // effect's `span_index < automation_capacity` limit (issue #1012), and a window that holds
        // every cell drops none (#1345 D4).
        let capacity = lanes
            .iter()
            .flatten()
            .map(EffectControlLane::parameter_cells)
            .max()
            .unwrap_or(0);
        if capacity > metadata.program_key.automation_capacity as usize {
            return Err(RackError::AutomationWindow);
        }
        let total = capacity
            .checked_mul(lane_count)
            .ok_or(RackError::Overflow)?;
        let staging: Box<[PreparedAutomationSpan]> = vec![IDLE_SPAN; capacity].into_boxed_slice();

        let words = (quantum as usize)
            .checked_mul(lane_count)
            .ok_or(RackError::Overflow)?;
        let line = latency.checked_mul(lane_count).ok_or(RackError::Overflow)?;
        // Issue #1087: a shunt whenever any lane can be bypassed -- a live channel, which may
        // bypass at any block, or a lane bypassed at preparation, whether or not live controls
        // are attached. A channel-less lane that is not bypassed can never become so.
        let shunt = lanes
            .iter()
            .flatten()
            .any(|lane| lane.has_channel() || lane.bypassed())
            .then(|| BypassShunt::new(words, line));
        // Issue #143 level-1 zero: a slot no observation request touched holds neither the lane
        // vector nor the per-lane sample scratch.
        let observed = observations.iter().any(Option::is_some);
        let samples = if observed {
            vec![ObservationSample::default(); lane_count].into_boxed_slice()
        } else {
            Vec::new().into_boxed_slice()
        };
        let designed = (0..lane_count)
            .map(|lane| processor.lane_channel_symmetry(lane))
            .collect();
        // One window serves every lane: a lane's staged prefix is copied into `packed` the moment
        // it is drained. It holds a span for every live parameter cell of every lane (it is the
        // widest lane's cell count), so a block stages each cell it reads and none is deferred or
        // dropped (#1345 D4); it is never above the capacity (checked above), so the effect's
        // `span_index < automation_capacity` cut-off refuses none of them (issue #1012).
        let observe_queues: Box<[(usize, Consumer<EffectControlRecord>)]> = lanes
            .iter_mut()
            .enumerate()
            .filter_map(|(index, lane)| {
                lane.as_mut()
                    .and_then(EffectControlLane::take_observe_queue)
                    .map(|queue| (index, queue))
            })
            .collect();
        Ok(Self {
            processor,
            designed,
            width,
            quantum,
            offsets: vec![0_u32; lane_count + 1].into_boxed_slice(),
            lanes: lanes.into_boxed_slice(),
            observe_queues,
            carry_lanes: 0,
            staging,
            packed: vec![IDLE_SPAN; total].into_boxed_slice(),
            shunt,
            crossfade: BankCrossfade::new(),
            observations: observed.then(|| observations.into_boxed_slice()),
            samples,
            staged_spans: 0,
            walk: LaneWalk::default(),
            quiet_at: None,
            unbound: 0,
            unrestored: 0,
            layout,
            program_key: metadata.program_key,
            carry_refused: 0,
        })
    }

    /// The layout a plan swap checks before it moves a lane in or out (#1280).
    #[must_use]
    pub const fn layout(&self) -> EffectLaneLayout {
        self.layout
    }

    /// The bank's lanes, populated or padded.
    #[must_use]
    pub const fn lanes(&self) -> usize {
        self.width.lanes() as usize
    }

    /// Restores a plan swap's carry refused on this stage (#1279 D6). Saturating.
    #[must_use]
    pub const fn carry_refused(&self) -> u64 {
        self.carry_refused
    }

    /// The kind of lane `lane` is, if it exists: what it holds besides the effect's state, which
    /// a plan swap's join requires to agree between the two plans (#1280).
    #[must_use]
    pub fn lane_kind(&self, lane: usize) -> Option<EffectLaneKind> {
        let channel = self.lanes.get(lane)?;
        Some(match channel {
            None => EffectLaneKind::Uncontrolled,
            Some(channel) if channel.has_channel() => EffectLaneKind::Cells,
            Some(_) => EffectLaneKind::BypassOnly,
        })
    }

    // REALTIME_POLICY_BEGIN
    /// Whether [`Self::carry_lane_from`] would take `predecessor`'s lane `predecessor_lane` into
    /// this stage's lane `lane` with a scratch of `scratch_bytes`: both lanes exist and are of one
    /// kind, both banks run one layout, the two control lanes have one cell shape, and the scratch
    /// holds the payload. Allocation-free and bounded.
    #[must_use]
    pub fn can_carry_lane_from(
        &self,
        lane: usize,
        predecessor: &Self,
        predecessor_lane: usize,
        scratch_bytes: usize,
    ) -> bool {
        let shapes_agree = match (
            self.lanes.get(lane).and_then(Option::as_ref),
            predecessor
                .lanes
                .get(predecessor_lane)
                .and_then(Option::as_ref),
        ) {
            (Some(into), Some(from)) => into.can_carry_from(from),
            (None, None) => true,
            _ => false,
        };
        lane < self.lanes()
            && predecessor_lane < predecessor.lanes()
            && self.lane_kind(lane) == predecessor.lane_kind(predecessor_lane)
            && shapes_agree
            && self.layout == predecessor.layout
            && usize::try_from(self.layout.payload_bytes())
                .is_ok_and(|payload| payload <= scratch_bytes)
    }

    /// Move `predecessor`'s lane `predecessor_lane` into this stage's lane `lane` at the swap
    /// block (#1280): its whole state payload through `scratch` (#1279 D3, a ramp in flight
    /// included), then its control lane ([`EffectControlLane::carry_from`]: the applied bypass and
    /// its crossfade, the live symmetry terms and the writes the predecessor's cells held
    /// unread), then its words of the dry shunt's latency line, taken relative to each bank's
    /// shared cursor (D5), and last the witness this stage holds for the lane, recomputed after
    /// the restore. Returns whether the lane was taken.
    ///
    /// The caller has checked [`Self::can_carry_lane_from`], and has disengaged `predecessor`'s
    /// chain ([`BankChain::disengage_for_carry`]), as [`EffectBankStage::carry_lane_from`]
    /// requires. A snapshot or a restore that refuses leaves this lane as prepared, at rest, takes
    /// nothing else of the lane and counts in [`Self::carry_refused`] (D6). Allocation-free and
    /// bounded by the prepared state size and the latency.
    pub fn carry_lane_from(
        &mut self,
        lane: usize,
        predecessor: &Self,
        predecessor_lane: usize,
        scratch: &mut [u8],
    ) -> bool {
        let taken = if self.can_carry_lane_from(lane, predecessor, predecessor_lane, scratch.len())
        {
            carry_payload(
                &mut *self.processor,
                lane,
                &*predecessor.processor,
                predecessor_lane,
                &self.layout,
                scratch,
            )
        } else {
            None
        };
        let Some(restored) = taken else {
            self.carry_refused = self.carry_refused.saturating_add(1);
            return false;
        };
        self.quiet_at = None;
        if let (Some(into), Some(from)) = (
            self.lanes.get_mut(lane).and_then(Option::as_mut),
            predecessor
                .lanes
                .get(predecessor_lane)
                .and_then(Option::as_ref),
        ) {
            let carried = into.carry_from(from);
            debug_assert!(carried, "a checked control lane carries");
        }
        // A lane whose bank keeps no latency line in either plan has no dry history to move: the
        // lane's kind says it can never be bypassed live there (#1280 D5).
        if let (Some(into), Some(from)) = (
            self.shunt.as_mut().filter(|shunt| shunt.feeds_line()),
            predecessor
                .shunt
                .as_ref()
                .filter(|shunt| shunt.feeds_line()),
        ) {
            let carried = into.carry_line_lane_from(
                lane,
                self.width.lanes() as usize,
                from,
                predecessor_lane,
                predecessor.width.lanes() as usize,
            );
            debug_assert!(carried, "a checked shunt line carries");
        }
        self.designed[lane] = self.processor.lane_channel_symmetry(lane);
        if restored {
            self.unrestored &= !(1_u8 << lane);
        } else {
            self.unrestored |= 1_u8 << lane;
        }
        true
    }

    /// Whether the per-node instance with control lane `control` could be moved into this stage's
    /// lane `lane` in a plan swap (#1281): the lane exists and is of the instance's kind, the
    /// lane's carried slots are installed ([`EffectControlLane::install_carry_slots`]) and the two
    /// control lanes have one cell shape, and `scratch_bytes` holds the payload. The two layouts
    /// agree by the join that named the move, which compares them where both are known.
    /// Allocation-free and bounded.
    #[must_use]
    pub fn can_carry_instance(
        &self,
        lane: usize,
        control: &EffectControlLane,
        scratch_bytes: usize,
    ) -> bool {
        self.lanes
            .get(lane)
            .and_then(Option::as_ref)
            .is_some_and(|own| own.can_carry_from(control))
            && self.lane_kind(lane) == Some(EffectLaneKind::of_instance(control))
            && usize::try_from(self.layout.payload_bytes())
                .is_ok_and(|payload| payload <= scratch_bytes)
    }

    /// Whether this stage's lane `lane` could be moved into the per-node instance with control
    /// lane `control` in a plan swap (#1281): [`Self::can_carry_instance`] with the direction
    /// reversed, so the instance's carried slots are the ones that must be installed.
    #[must_use]
    pub fn can_carry_lane_into_instance(
        &self,
        lane: usize,
        control: &EffectControlLane,
        scratch_bytes: usize,
    ) -> bool {
        self.lanes
            .get(lane)
            .and_then(Option::as_ref)
            .is_some_and(|own| control.can_carry_from(own))
            && self.lane_kind(lane) == Some(EffectLaneKind::of_instance(control))
            && usize::try_from(self.layout.payload_bytes())
                .is_ok_and(|payload| payload <= scratch_bytes)
    }

    /// Move the per-node instance `from` into this stage's lane `lane` at the swap block (#1281
    /// D2): its whole state payload through `scratch` (the instance's snapshot into the bank's
    /// per-lane restore, a ramp in flight included), then its control lane
    /// ([`EffectControlLane::carry_from`]), then its dry shunt's latency line (D5, as one lane of
    /// one), and last the witness this stage holds for the lane, recomputed after the restore.
    /// Returns whether the lane was taken.
    ///
    /// The caller has checked [`Self::can_carry_instance`]. A per-node instance renders dual
    /// always, so both its channels hold real state and there is no collapse to take down first. A
    /// snapshot or a restore that refuses leaves this lane as prepared, at rest, takes nothing
    /// else of the instance and counts in [`Self::carry_refused`] (D6). Allocation-free and bounded
    /// by the prepared state size and the latency.
    pub fn carry_lane_from_instance(
        &mut self,
        lane: usize,
        from: EffectInstance<'_>,
        scratch: &mut [u8],
    ) -> bool {
        let taken = if self.can_carry_instance(lane, from.control, scratch.len()) {
            carry_payload_from_instance(
                &mut *self.processor,
                lane,
                from.processor,
                &self.layout,
                scratch,
            )
        } else {
            None
        };
        let Some(restored) = taken else {
            self.carry_refused = self.carry_refused.saturating_add(1);
            return false;
        };
        self.quiet_at = None;
        if let Some(into) = self.lanes.get_mut(lane).and_then(Option::as_mut) {
            let carried = into.carry_from(from.control);
            debug_assert!(carried, "a checked control lane carries");
        }
        if let Some(into) = self.shunt.as_mut().filter(|shunt| shunt.feeds_line())
            && from.shunt.feeds_line()
        {
            let carried =
                into.carry_line_lane_from(lane, self.width.lanes() as usize, from.shunt, 0, 1);
            debug_assert!(carried, "a checked shunt line carries");
        }
        self.designed[lane] = self.processor.lane_channel_symmetry(lane);
        if restored {
            self.unrestored &= !(1_u8 << lane);
        } else {
            self.unrestored |= 1_u8 << lane;
        }
        true
    }

    /// Move this stage's lane `lane` into the per-node instance `into` at the swap block (#1281
    /// D2): the bank's per-lane snapshot into the instance's restore, then the lane's control
    /// ([`EffectControlLane::carry_from`]) and its words of the dry shunt's line (D5). Returns
    /// whether the instance took it; `false` leaves the instance as prepared, at rest, and the
    /// caller counts the refusal (D6).
    ///
    /// The caller has checked [`Self::can_carry_lane_into_instance`], and has disengaged this stage's chain
    /// ([`BankChain::disengage_for_carry`], D3), so a collapsed bank's right channel holds the
    /// state a dual run would and the instance continues it bit for bit. Reads this stage only:
    /// it never renders again. Allocation-free and bounded by the prepared state size and the
    /// latency.
    pub fn carry_lane_into_instance(
        &self,
        lane: usize,
        into: EffectInstanceMut<'_>,
        scratch: &mut [u8],
    ) -> bool {
        if !self.can_carry_lane_into_instance(lane, into.control, scratch.len()) {
            return false;
        }
        if carry_payload_into_instance(
            into.processor,
            &*self.processor,
            lane,
            &self.layout,
            scratch,
        )
        .is_none()
        {
            return false;
        }
        if let Some(from) = self.lanes.get(lane).and_then(Option::as_ref) {
            let carried = into.control.carry_from(from);
            debug_assert!(carried, "a checked control lane carries");
        }
        if let Some(from) = self.shunt.as_ref().filter(|shunt| shunt.feeds_line())
            && into.shunt.feeds_line()
        {
            let carried =
                into.shunt
                    .carry_line_lane_from(0, 1, from, lane, self.width.lanes() as usize);
            debug_assert!(carried, "a checked shunt line carries");
        }
        true
    }
    // REALTIME_POLICY_END

    /// The bytes [`Self::install_carry_slots`] allocates for lane `lane`, and the lane holds once
    /// it has: none for a lane that is absent or has no cells (#1633 D1).
    #[must_use]
    pub fn carry_slot_bytes(&self, lane: usize) -> u64 {
        self.lanes
            .get(lane)
            .and_then(Option::as_ref)
            .map_or(0, EffectControlLane::carry_slot_bytes)
    }

    /// Allocate the carried slots of lane `lane`, the successor lane of a plan swap's move. Off the
    /// render thread; a lane that is absent or has no cells takes none (#1633 D1).
    pub fn install_carry_slots(&mut self, lane: usize) {
        self.quiet_at = None;
        if let Some(control) = self.lanes.get_mut(lane).and_then(Option::as_mut) {
            control.install_carry_slots();
        }
    }

    /// `Observe` records refused because this slot has no observation capacity. Read off render.
    #[must_use]
    pub const fn unbound_observations(&self) -> u64 {
        self.unbound
    }

    /// Whether any lane of this slot carries observation taps at all (issue #143 E5).
    #[must_use]
    pub fn is_observed(&self) -> bool {
        self.observations.is_some()
    }

    /// Declared taps and armed taps across every lane of this slot, for the structural gates.
    #[must_use]
    pub fn observation_tap_counts(&self) -> (u64, u64) {
        let mut declared = 0_u64;
        let mut armed = 0_u64;
        for lane in self.observations.iter().flat_map(|lanes| lanes.iter()) {
            if let Some(observation) = lane.as_ref() {
                declared += observation.len() as u64;
                armed += (0..observation.len())
                    .filter(|tap| observation.is_armed(*tap))
                    .count() as u64;
            }
        }
        (declared, armed)
    }

    /// The spans of storage this slot holds for live-control staging: one window plus the packed
    /// array of one window per lane (#1611 D3). Zero when no lane stages a span.
    #[must_use]
    pub fn span_storage_len(&self) -> usize {
        self.staging.len() + self.packed.len()
    }

    /// Exact engine-owned bytes this slot's observation lanes retain. Zero when unobserved.
    #[must_use]
    pub fn observation_retained_bytes(&self) -> usize {
        self.observations
            .iter()
            .flat_map(|lanes| lanes.iter())
            .filter_map(Option::as_ref)
            .map(ObservationLane::retained_bytes)
            .sum()
    }

    /// Drop every subscription this slot carries (issue #143 D7).
    pub fn disarm_observations(&mut self) {
        for lane in self
            .observations
            .iter_mut()
            .flat_map(|lanes| lanes.iter_mut())
        {
            if let Some(observation) = lane.as_mut() {
                observation.disarm_all();
            }
        }
    }
}

/// What one drain's cell walk found over a bank's lanes, after each lane staged (issue #1763): the
/// process body that follows it in the same block reads it instead of walking the lanes again.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct LaneWalk {
    /// Some lane staged a prepared target.
    targets: bool,
    /// Some lane reads the dry block: it is bypassed or its crossfade is running.
    needs_dry: bool,
    /// Some lane's bypass crossfade is running.
    ramping: bool,
}

impl LaneWalk {
    /// No target to apply and no crossfade to advance: a second walk under the same snapshot
    /// would find the same lanes and leave them as they are.
    const fn quiet(self) -> bool {
        !self.targets && !self.ramping
    }
}

/// The value an unused staging slot holds. It is never handed to a bank: only `[..offsets[lanes]]`
/// is, and every span inside that range was written by this block's drain.
const IDLE_SPAN: PreparedAutomationSpan = PreparedAutomationSpan {
    channel: effect_contract::ParameterChannel::Both,
    parameter_index: 0,
    sample: 0,
    value: 0.0,
};

impl BankStage for LiveControlEffectBankStage {
    fn copy_response_snapshot_lane(
        &self,
        lane: usize,
        _sample_rate_hz: u32,
        request: effect_contract::ResponseSnapshotRequest<'_>,
    ) -> Result<effect_contract::ResponseSnapshotSummary, effect_contract::ResponseAnalysisError>
    {
        let effect_contract::ResponseSnapshotRequest { left, right, .. } = request;
        let bypassed = self
            .lanes
            .get(lane)
            .and_then(Option::as_ref)
            .is_some_and(EffectControlLane::bypassed);
        self.processor.copy_response_snapshot_lane(
            lane,
            effect_contract::ResponseSnapshotRequest {
                bypassed,
                left,
                right,
            },
        )
    }

    fn response_snapshot_bypassed(&self, lane: usize) -> bool {
        self.lanes
            .get(lane)
            .and_then(Option::as_ref)
            .is_some_and(EffectControlLane::bypassed)
    }

    /// The designed-word comparison, conjoined with the lane's own live terms.
    ///
    /// The live half comes from [`EffectControlLane::symmetry`], which the drain in
    /// [`Self::process`] maintains: a lane with no live-control channel has no live writes at all
    /// and contributes only the designed term.
    fn lane_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
        let designed = lane_witness(
            self.designed.get(lane).copied().unwrap_or(false),
            u32::try_from(lane)
                .ok()
                .and_then(|lane| self.unrestored.checked_shr(lane))
                .is_none_or(|bits| bits & 1 == 0),
        );
        match self.lanes.get(lane).and_then(Option::as_ref) {
            Some(channel) => designed.and(channel.symmetry()),
            None => designed,
        }
    }

    fn seam_side(&self) -> SeamSide {
        SeamSide::UpstreamOfSeam
    }

    fn supports_mono_collapse(&self) -> bool {
        self.processor.supports_mono_collapse()
    }

    fn desymmetrize(&mut self) {
        self.processor.desymmetrize_channels();
    }

    fn channels_agree(&self) -> bool {
        self.processor.channels_agree()
    }

    fn as_any_mut(&mut self) -> Option<&mut dyn core::any::Any> {
        Some(self)
    }

    fn as_any(&self) -> Option<&dyn core::any::Any> {
        Some(self)
    }

    fn lane_swap_scratch_bytes(&self) -> usize {
        lane_swap_scratch(&*self.processor, &self.layout).unwrap_or(0)
    }

    // REALTIME_POLICY_BEGIN
    /// Both lanes exist, the peer is a [`LiveControlEffectBankStage`] of this program key, both
    /// banks declare prepared defaults of one size, the scratch holds
    /// [`effect_contract::swap_bank_lanes_scratch_bytes`], and every per-lane part of the two
    /// lanes has the same shape: see [`LiveControlEffectBankStage::lane_swap_fits`].
    fn can_swap_lanes(&self, lane: usize, peer: LanePeer<'_>, scratch_bytes: usize) -> bool {
        let fits = |other: &Self, other_lane: usize| {
            lane_swap_scratch_fits(
                &*self.processor,
                &*other.processor,
                &self.layout,
                scratch_bytes,
            ) && self.lane_swap_fits(lane, other, other_lane)
        };
        match peer {
            LanePeer::Same(other_lane) => fits(self, other_lane),
            LanePeer::Other(stage, other_lane) => stage
                .as_any()
                .and_then(|any| any.downcast_ref::<Self>())
                .is_some_and(|other| fits(other, other_lane)),
        }
    }

    /// Every per-lane field moves with the strip, in this order: the effect's state payload
    /// ([`effect_contract::swap_bank_lanes`], the only step that can refuse, so a refusal changes
    /// nothing), the control lane (cells, applied bypass and its crossfade ramp, live symmetry
    /// terms, carried writes), the observation queue's consumer (the indices stay with their
    /// positions), the observation taps ([`ObservationLane::swap_state`]), the `designed` flags,
    /// the `unrestored` bits, and the lane's words of the dry shunt's latency line
    /// ([`BypassShunt::swap_line_lanes`]).
    ///
    /// The rest is per block or per bank and does not move: `offsets`, `staging`, `packed`,
    /// `staged_spans`, `crossfade` and `samples` are rewritten by every block before they are read;
    /// the shunt's `dry_*` staging and its shared cursor, the processor's shared words, the width,
    /// quantum, layout, and the `unbound` and `carry_refused` counters belong to the bank. When
    /// both stages hold a latency line the two lanes' words are exchanged even for a lane that
    /// never reads it (one with no control lane, or a channel-less one not bypassed): its words
    /// are dead, and moving them keeps the rule one rule.
    fn swap_lanes(&mut self, lane: usize, peer: LanePeerMut<'_>, scratch: &mut [u8]) -> bool {
        if !self.can_swap_lanes(lane, peer.as_peer(), scratch.len()) {
            return false;
        }
        let (version, sizes) = (self.layout.state_layout_version, self.layout.state_sizes);
        let lanes = self.width.lanes() as usize;
        // A moved lane brings the state its own bank's drain left: the next drain reads it.
        self.quiet_at = None;
        match peer {
            LanePeerMut::Same(other_lane) => {
                if lane == other_lane {
                    return true;
                }
                if effect_contract::swap_bank_lanes(
                    &mut *self.processor,
                    lane,
                    None,
                    other_lane,
                    version,
                    sizes,
                    scratch,
                )
                .is_err()
                {
                    return false;
                }
                self.lanes.swap(lane, other_lane);
                if let (Some(own), Some(theirs)) = (
                    self.observe_queue_position(lane),
                    self.observe_queue_position(other_lane),
                ) && let Some((own, theirs)) = two_mut(&mut self.observe_queues, own, theirs)
                {
                    core::mem::swap(&mut own.1, &mut theirs.1);
                }
                if let Some((Some(own), Some(theirs))) = self
                    .observations
                    .as_deref_mut()
                    .and_then(|taps| two_mut(taps, lane, other_lane))
                {
                    let swapped = own.swap_state(theirs);
                    debug_assert!(swapped, "checked taps swap");
                }
                self.designed.swap(lane, other_lane);
                swap_bits_within(&mut self.unrestored, lane, other_lane);
                if let Some(shunt) = self.shunt.as_mut() {
                    let swapped = shunt.swap_line_lanes(lane, lanes, None, other_lane, lanes);
                    debug_assert!(swapped, "a checked shunt line swaps");
                }
            }
            LanePeerMut::Other(stage, other_lane) => {
                let Some(other) = stage
                    .as_any_mut()
                    .and_then(|any| any.downcast_mut::<Self>())
                else {
                    return false;
                };
                if effect_contract::swap_bank_lanes(
                    &mut *self.processor,
                    lane,
                    Some(&mut *other.processor),
                    other_lane,
                    version,
                    sizes,
                    scratch,
                )
                .is_err()
                {
                    return false;
                }
                core::mem::swap(&mut self.lanes[lane], &mut other.lanes[other_lane]);
                other.quiet_at = None;
                if let (Some(own), Some(theirs)) = (
                    self.observe_queue_position(lane),
                    other.observe_queue_position(other_lane),
                ) {
                    core::mem::swap(
                        &mut self.observe_queues[own].1,
                        &mut other.observe_queues[theirs].1,
                    );
                }
                if let (Some(own), Some(theirs)) = (
                    self.observation_mut(lane),
                    other.observation_mut(other_lane),
                ) {
                    let swapped = own.swap_state(theirs);
                    debug_assert!(swapped, "checked taps swap");
                }
                core::mem::swap(&mut self.designed[lane], &mut other.designed[other_lane]);
                swap_bits(
                    &mut self.unrestored,
                    lane,
                    &mut other.unrestored,
                    other_lane,
                );
                let other_lanes = other.width.lanes() as usize;
                if let (Some(own), Some(theirs)) = (self.shunt.as_mut(), other.shunt.as_mut()) {
                    let swapped =
                        own.swap_line_lanes(lane, lanes, Some(theirs), other_lane, other_lanes);
                    debug_assert!(swapped, "a checked shunt line swaps");
                }
            }
        }
        true
    }
    // REALTIME_POLICY_END

    fn begin_block(
        &mut self,
        first_sample: u64,
        snapshot: LiveSnapshot,
    ) -> Result<(), RenderError> {
        self.drain(first_sample, snapshot)
    }

    fn carry_observes(&mut self, first_sample: u64) {
        let lanes = core::mem::take(&mut self.carry_lanes);
        self.drain_observe_queues(first_sample, lanes);
    }

    fn live_lanes(&self) -> u8 {
        self.lanes
            .iter()
            .enumerate()
            .fold(0, |mask, (lane, channel)| {
                mask | (u8::from(channel.is_some()) << lane)
            })
    }

    // REALTIME_POLICY_BEGIN
    /// The effect's own mask minus every lane that has work of its own in this block, read after
    /// this block's drain (issue #1781):
    ///
    /// * a **staged span** (`offsets[l + 1] > offsets[l]`): the bank would apply it;
    /// * a **staged prepared target**: the process body would apply it before the bank runs;
    /// * a lane that **reads the dry block** -- bypassed, settled or crossfading -- whose output
    ///   is the shunt's latency-matched dry signal, not the wet `+0.0`, and a **running bypass
    ///   crossfade**, which the block advances;
    /// * an **armed observation tap**, which the block accumulates into.
    ///
    /// The effect's mask answers for a block with no span and no target, which is what is left.
    /// The quiet skip of #1763 is covered: a drain that skips the cell walk keeps the last walk's
    /// partition and `LaneWalk`, and a walk is skipped only after one that staged no span, no
    /// target and left no crossfade running, so every `offsets` window is empty and no target
    /// prefix is read (`walk.targets` is false). The shunt's latency line is not a term: a
    /// skipped block feeds it in [`Self::advance_clocks`], as a rendered one does.
    fn lanes_at_rest(&self) -> u32 {
        let mut rest = self.processor.lanes_at_rest();
        let observations = self.observations.as_deref();
        for (lane, control) in self.lanes.iter().enumerate() {
            let spans = self.offsets[lane + 1] > self.offsets[lane];
            let targets = self.walk.targets
                && control
                    .as_ref()
                    .is_some_and(|control| !control.prepared_targets().is_empty());
            let dry = control
                .as_ref()
                // Defensive: a running bypass ramp already implies `needs_dry`.
                .is_some_and(|control| control.needs_dry() || control.bypass_ramping());
            let tap = observations
                .and_then(|lanes| lanes.get(lane))
                .and_then(Option::as_ref)
                .is_some_and(ObservationLane::any_armed);
            if spans || targets || dry || tap {
                rest &= !(1 << lane);
            }
        }
        rest
    }

    /// The effect's clocks, forwarded with the block's length, and the shunt's latency line fed
    /// the `+0.0` block exactly as the process body feeds it on a block no lane reads dry
    /// ([`BypassShunt::feed_line`], issue #1781): a later bypass edge then reads the line words a
    /// never-skipped run holds. Every lane is at rest, so no lane reads dry and the process body
    /// would have fed the line and not captured. A collapsed process body feeds the left plane
    /// twice; this feeds both planes of the skipped block, which are both `+0.0`, so the line takes
    /// the same words either way and this method has no collapsed form.
    fn advance_clocks(&mut self, lanes_mask: u32, block: BankBlock<'_>) {
        // `feed_line` matches a rendered block only on the caller's `+0.0` planes.
        debug_assert!(
            block
                .left
                .iter()
                .chain(block.right.iter())
                .all(|word| word.to_bits() == 0),
            "a skipped block's planes are exact +0.0"
        );
        self.processor.advance_clocks(lanes_mask, block.frames);
        if let Some(shunt) = self.shunt.as_mut() {
            shunt.feed_line(block.left, block.right);
        }
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    /// [`LiveControlEffectBankStage::process`] with the bank call collapsed onto the left plane.
    ///
    /// Every other step is the dual one, in the same order and on the same words: the drain runs
    /// before a sample is touched, the shunt captures and mixes both planes, and the taps
    /// publish after the bank ran. The shunt's right plane is the ungathered scratch, which is
    /// sound for exactly one reason and it is worth stating: the seam overwrites the right plane
    /// after this slot, so nothing downstream ever reads what the shunt put there. The *left*
    /// restore is the one that matters and it is the dual one.
    ///
    /// A live bypass on any lane clears the witness' `UNBYPASSED` term, so a collapsed cohort has
    /// no bypassed or ramping lane and the mix is a no-op whenever this body runs at all. It is kept
    /// rather than asserted away because the two facts are maintained in different places and the
    /// cheap one is the one that should not be load-bearing.
    fn process_mono(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
        self.process_inner::<true>(block)
    }
    // REALTIME_POLICY_END

    fn observation_binding_counts(&self) -> [u64; 3] {
        let Some(lanes) = self.observations.as_deref() else {
            return [0, 0, 0];
        };
        let observed = lanes.iter().filter(|lane| lane.is_some()).count() as u64;
        let (declared, armed) = self.observation_tap_counts();
        [observed, declared, armed]
    }

    fn observation_retained_bytes(&self) -> usize {
        LiveControlEffectBankStage::observation_retained_bytes(self)
    }

    // REALTIME_POLICY_BEGIN
    fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
        self.process_inner::<false>(block)
    }
    // REALTIME_POLICY_END
}

impl LiveControlEffectBankStage {
    // REALTIME_POLICY_BEGIN
    /// Step 1 of the block, hoisted out of [`Self::process_inner`] so that it runs before the
    /// chain's collapse dispatch reads the witness. See [`BankStage::begin_block`].
    ///
    /// Two loops, and the split is the shape the realtime gate accepts (decision 15, #1345 D8,
    /// #1418 Amendment 1): the first reads each lane's cells under the block's snapshot `S` and
    /// pops nothing; the second drains each lane's observation queue, each pop counted at entry in
    /// the loop that holds it, and hands each record to the slot's observation lanes by the lane's
    /// index. The loop that stages and the loop that pops are not one loop because a queue drain
    /// inside a loop that also calls a method of the lane would need the lane named in the drain.
    ///
    /// # The quiet bank (issue #1763)
    ///
    /// The first loop is skipped when the last drain that ran it found the bank quiet under the
    /// same snapshot publication, `S`. That skip is exact, by the gate's own guarantee: every write
    /// of a publication at or below `S` is stored and marked before that publication, so the
    /// drain that read under `S` applied it, and no later write can carry a publication at or
    /// below `S`. So a second read under `S` applies nothing, and a write above `S` waits in its
    /// dirty word, or in the deferred mask the earlier read left, for the first drain under a newer
    /// snapshot, which runs the loop. A block's watermark covers `S` either way. Quiet means the
    /// last loop staged no span and no target and left no lane's bypass crossfade running, so the
    /// skipped loop would have left exactly what is there: an empty partition, empty target
    /// prefixes, and the same [`LaneWalk`]. [`LiveSnapshot::ALL`] is not a publication and never
    /// skips. The observation queues are FIFOs outside the gate, so the second loop always runs.
    fn drain(&mut self, first_sample: u64, snapshot: LiveSnapshot) -> Result<(), RenderError> {
        if snapshot == LiveSnapshot::ALL || self.quiet_at != Some(snapshot.publication()) {
            self.read_cells(first_sample, snapshot);
        }
        self.drain_observe_queues(first_sample, u8::MAX);
        Ok(())
    }

    /// The drain's second loop: the observation queue of every lane in `lanes` (bit `n` is lane
    /// `n`) popped as many times as its count at entry, each record applied to the lane's tap at
    /// `first_sample` and one the lane has no tap for counted in [`Self::unbound`]. The block path
    /// passes every lane; a plan swap's carry passes the lanes it marked (#1816), so the carry
    /// and the block run one loop.
    fn drain_observe_queues(&mut self, first_sample: u64, lanes: u8) {
        let mut observations = self.observations.as_deref_mut();
        let mut unbound = 0_u64;
        for (index, queue) in self.observe_queues.iter_mut() {
            if lanes & (1 << *index) == 0 {
                continue;
            }
            let mut available = queue.available_at_entry();
            while available != 0 {
                available -= 1;
                let Ok(record) = queue.try_pop() else {
                    // The entry snapshot and SPSC ownership guarantee this cannot happen. Keep
                    // the drain bounded if a malformed test double violates that invariant.
                    break;
                };
                let observation = observations
                    .as_deref_mut()
                    .and_then(|lanes| lanes.get_mut(*index))
                    .and_then(Option::as_mut);
                if !apply_observe(record, first_sample, observation) {
                    unbound = unbound.saturating_add(1);
                }
            }
        }
        self.unbound = self.unbound.saturating_add(unbound);
    }

    /// The drain's first loop: every lane's cells read under `snapshot`, its spans packed at its
    /// own offset, and the bank's [`LaneWalk`] taken after each lane staged.
    fn read_cells(&mut self, first_sample: u64, snapshot: LiveSnapshot) {
        let mut packed = 0_usize;
        let mut walk = LaneWalk::default();
        self.offsets[0] = 0;
        for (lane, channel) in self.lanes.iter_mut().enumerate() {
            if let Some(channel) = channel.as_mut() {
                let staged = channel.stage(&mut self.staging, first_sample, snapshot);
                // Packed at this lane's own offset, immediately: that offset is what makes the
                // window reusable and what gives the lane its private partition of `packed`.
                self.packed[packed..packed + staged.staged]
                    .copy_from_slice(&self.staging[..staged.staged]);
                packed += staged.staged;
                walk.targets |= staged.staged_targets != 0;
                walk.needs_dry |= channel.needs_dry();
                walk.ramping |= channel.bypass_ramping();
            }
            self.offsets[lane + 1] = packed as u32;
        }
        self.staged_spans = packed;
        self.walk = walk;
        self.quiet_at = (packed == 0 && walk.quiet()).then_some(snapshot.publication());
    }

    /// Mark lane `lane` for a plan swap's pending-`Observe` drain (#1280 D3, #1816), which
    /// [`BankStage::carry_observes`] runs once the carry moved every state. The predecessor never
    /// renders again, so a record admitted after its last block is applied then, as its next
    /// block would have. A lane past the bank, or with no queue, selects nothing. Pops nothing.
    pub fn mark_observe_carry(&mut self, lane: usize) {
        if lane < 8 {
            self.carry_lanes |= 1 << lane;
        }
    }

    /// The index into [`Self::observe_queues`] of lane `lane`'s queue, if it has one.
    fn observe_queue_position(&self, lane: usize) -> Option<usize> {
        self.observe_queues
            .iter()
            .position(|(index, _)| *index == lane)
    }

    /// Lane `lane`'s observation taps, if the slot is observed and the lane has taps.
    fn observation(&self, lane: usize) -> Option<&ObservationLane> {
        self.observations
            .as_deref()
            .and_then(|lanes| lanes.get(lane))
            .and_then(Option::as_ref)
    }

    /// Whether lane `lane` can ever read the dry shunt: it has a live channel, or a bypass set at
    /// preparation. A stage holding such a lane holds a shunt.
    fn reads_shunt(&self, lane: usize) -> bool {
        self.lanes
            .get(lane)
            .and_then(Option::as_ref)
            .is_some_and(|control| control.has_channel() || control.bypassed())
    }

    /// Whether lane `lane` of this stage and lane `other_lane` of `other` (which may be this
    /// stage) can trade every per-lane part (issue #1747): both lanes exist; both stages run one
    /// program key; the lanes are of one kind and, with two control lanes, of one cell shape;
    /// both or neither has an observation queue; both or neither has taps, and two lanes' taps are the
    /// same menu; and each lane that can read a dry shunt finds one in the other stage, with two
    /// shunts of one shape. Allocation-free and bounded by the lane and tap counts.
    #[must_use]
    pub fn lane_swap_fits(&self, lane: usize, other: &Self, other_lane: usize) -> bool {
        let controls = match (
            self.lanes.get(lane).and_then(Option::as_ref),
            other.lanes.get(other_lane).and_then(Option::as_ref),
        ) {
            (Some(own), Some(theirs)) => own.same_cell_shape(theirs),
            (None, None) => true,
            _ => false,
        };
        let taps = match (self.observation(lane), other.observation(other_lane)) {
            (Some(own), Some(theirs)) => own.same_taps(theirs),
            (None, None) => true,
            _ => false,
        };
        let shunts = match (self.shunt.as_ref(), other.shunt.as_ref()) {
            (Some(own), Some(theirs)) => own.same_shape(theirs),
            (Some(_), None) => !self.reads_shunt(lane),
            (None, Some(_)) => !other.reads_shunt(other_lane),
            (None, None) => true,
        };
        lane < self.lanes()
            && other_lane < other.lanes()
            && self.program_key == other.program_key
            && self.lane_kind(lane) == other.lane_kind(other_lane)
            && controls
            && self.observe_queue_position(lane).is_some()
                == other.observe_queue_position(other_lane).is_some()
            && taps
            && shunts
    }

    /// Lane `lane`'s observation tap state, if the slot has observation capacity and the lane has
    /// a tap (#1327 D2): the part of the lane a plan swap exchanges. Allocation-free.
    pub fn observation_mut(&mut self, lane: usize) -> Option<&mut ObservationLane> {
        self.observations
            .as_deref_mut()
            .and_then(|lanes| lanes.get_mut(lane))
            .and_then(Option::as_mut)
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    /// The one live-control slot body, dual or collapsed.
    ///
    /// `MONO` is a const generic rather than an argument so the two monomorphise: the dual
    /// instantiation is the code that shipped before the collapse existed, which is what keeps
    /// "adding a path does not move the path already there" a property of the build rather than a
    /// hope about the inliner. The parametric EQ's `process_bank_inner` carries the measurement
    /// that settled it.
    fn process_inner<const MONO: bool>(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
        let lane_count = self.width.lanes() as usize;
        // 1. The drain already ran, in `begin_block`, before the chain decided this block's mode.
        let packed = self.staged_spans;
        // The chain has already performed any collapsed-to-dual state restoration before it
        // dispatches this stage. Apply each lane's staged target prefix at that boundary;
        // applying in `begin_block` would let desymmetrization copy the old right state over a new
        // right-lane target. A drain that staged no target leaves every lane's prefix empty
        // (issue #1763).
        if self.walk.targets {
            for lane in 0..lane_count {
                let Some(channel) = self.lanes[lane].as_ref() else {
                    continue;
                };
                for target in channel.prepared_targets() {
                    self.processor
                        .apply_prepared_target_lane(lane, target)
                        .map_err(|_| RenderError::InvalidEnvelope)?;
                }
            }
        }
        // Issue #163 phase 4 item 4: which lanes, if any, are bypassed this block. Decided here
        // because it gates both the capture below and the restore in step 3, and because the
        // control drain in step 1 has already run — so this is the same verdict step 3 reaches,
        // read once instead of twice.
        //
        // `shunt.is_some()` means *some lane of this cohort has a control channel*, not that any
        // lane is bypassed. An eight-lane cohort with one live-control-driven lane and nothing
        // bypassed was paying a `quantum * lanes` two-plane copy every block for a dry block no
        // reader could observe.
        //
        // The drain took the disjunction after each lane staged, and nothing changes a lane
        // between the drain and this body (issue #1763).
        let any_bypassed = self.walk.needs_dry;
        // 2. Capture the dry block before the bank touches it. Skippable exactly when no lane
        //    will read it back and there is no latency line to keep fed; `dry_*` never crosses a
        //    block boundary, so the skip moves no rendered bit.
        //
        // A collapsed block captures the **left plane twice**, and this is the one place in the
        // collapse where "the seam overwrites the right plane after this slot" is not the whole
        // argument. `capture` does two things: it stages `dry_*`, which is read later in this same
        // block and never crosses a boundary, and -- when the slot declares latency -- it exchanges
        // that staging through a delay **line** that persists for `latency` samples. Handing the
        // line the ungathered resident scratch poisons it for as long as it holds those samples,
        // and a bypass engaged after the collapse disengages then restores that scratch into the
        // bypassed lane's right channel. The seam is downstream of the line, so it cannot repair it.
        //
        // The left plane is the right answer, not a stand-in for one: under the collapse the
        // counterfactual dual run's right plane **is** its left plane everywhere upstream of the
        // seam, so `capture(left, left)` writes into `dry_right` and `line_right` exactly the words
        // a never-collapsed run writes there, on every block. That is the same standard
        // `desymmetrize_channels` meets, met continuously instead of at a boundary.
        //
        // Covering the shunt in the disengage copy instead would be weaker twice over: the line
        // would hold ungathered scratch for the whole collapsed window, so any reader that is not
        // the disengage boundary would still be wrong; and it would leave a read of `block.right`
        // alive on the collapsed path, which is the exact class of defect this design excludes by
        // construction rather than by argument. **A collapsed block reads no right plane at all.**
        //
        // A block no lane reads dry feeds the line without the dry staging (issue #1763): the line
        // and its cursor end as `capture` leaves them, and the staging has no reader.
        if let Some(shunt) = self.shunt.as_mut() {
            let right = if MONO { &*block.left } else { &*block.right };
            if any_bypassed {
                shunt.capture(block.left, right);
            } else {
                shunt.feed_line(block.left, right);
            }
        }
        let frames = block.frames;
        let bank = EffectBankProcessBlock::new(
            block.left,
            block.right,
            None,
            frames,
            self.width,
            block.first_sample,
            &self.packed[..packed],
            &self.offsets,
            self.quantum,
        )
        .map_err(|_| RenderError::InvalidEnvelope)?;
        let _ = if MONO {
            self.processor.process_bank_mono(bank)
        } else {
            self.processor.process_bank(bank)
        };
        // 4. Publish every armed tap, after the bank ran. One `observe_resident_bank` call per
        // armed tap fills every lane, so a cohort of eight costs one vector extraction rather than
        // eight scalar reads.
        //
        // Issue #163 phase 4 item 3/6: the slot-level gate comes first. What the old comment here
        // called "one pass over a `bool` array" for a tap no lane armed was, for the slot as a
        // whole, an O(lanes) `max()` to find the tap count plus an O(taps x lanes) `.any()` walk
        // -- 4 096 flag loads per block for the 64-lane, 64-tap console shape, every block,
        // whether or not anything was subscribed. `ObservationLane::any_armed` is now O(1) per
        // lane, so this gate is O(lanes) and the whole publish section costs nothing at all until
        // a subscription exists.
        //
        // Semantics are unchanged by construction: this is the disjunction over lanes of the
        // per-tap `wants` disjunction below, so it can only skip work the inner loop would have
        // skipped tap by tap. `wants` and `accumulate`'s own armed guard both stay.
        if let Some(lanes) = self.observations.as_deref_mut().filter(|lanes| {
            lanes
                .iter()
                .filter_map(Option::as_ref)
                .any(ObservationLane::any_armed)
        }) {
            let taps = lanes
                .iter()
                .filter_map(Option::as_ref)
                .map(ObservationLane::len)
                .max()
                .unwrap_or(0);
            for tap in 0..taps {
                let wanted = lanes
                    .iter()
                    .filter_map(Option::as_ref)
                    .any(|observation| observation.wants(tap));
                if !wanted {
                    continue;
                }
                if !self
                    .processor
                    .observe_resident_bank(tap as u32, &mut self.samples)
                {
                    continue;
                }
                if MONO {
                    // A collapsed block evolves one channel, so this bank's right-channel state is
                    // whatever it was when the collapse engaged and a right-channel reading taken
                    // from it would be stale. The value a *dual* run would publish is the left
                    // one -- that is the induction the witness maintains and the same one the
                    // disengage copy rests on -- so the tap publishes it. This is the observation
                    // half of the seam: the right channel of a collapsed track **is** its left
                    // channel, at the tap exactly as at the fader.
                    for sample in self.samples.iter_mut() {
                        sample.right = sample.left;
                    }
                }
                for (lane, observation) in lanes.iter_mut().enumerate() {
                    let Some(observation) = observation.as_mut() else {
                        continue;
                    };
                    let Some(sample) = self.samples.get(lane) else {
                        continue;
                    };
                    observation.accumulate(tap, *sample, block.first_sample, u64::from(frames));
                }
            }
        }
        // 3. Latency-matched dry mix: each lane crossfades along its own bypass ramp, a settled
        //    bypassed lane takes the dry words and every other lane keeps the wet ones, in one
        //    lane-wide pass over the interleaved block (#1341 D4). `any_bypassed` is the
        //    disjunction of `needs_dry` over the lanes, decided before the capture; when it is
        //    false this whole step was already a no-op, and skipping it also proves the capture
        //    above had no reader. The lanes advance by the block afterwards (#1341 D6).
        if any_bypassed && let Some(shunt) = self.shunt.as_ref() {
            for (lane, channel) in self.lanes[..lane_count].iter().enumerate() {
                self.crossfade.set(
                    lane,
                    channel.as_ref().map_or(
                        BankCrossfadeLane::WET,
                        EffectControlLane::bypass_crossfade_lane,
                    ),
                );
            }
            let frames = frames as usize;
            effect_contract::match_bank_width!(self.width, |L| {
                shunt.crossfade_bank::<L>(block.left, block.right, frames, &self.crossfade);
            });
        }
        // A lane whose crossfade is settled does not advance (issue #1763).
        if self.walk.ramping {
            for channel in self.lanes[..lane_count].iter_mut().flatten() {
                channel.advance_bypass_mix(block.frames);
            }
        }
        Ok(())
    }
    // REALTIME_POLICY_END
}

/// One slot of a chain: a stage plus the lanes for which it is *not* an identity.
pub struct BankSlot {
    pub stage: Box<dyn BankStage>,
    pub active_lanes: Box<[bool]>,
}

/// The render-owned form of a slot. The public bool slice is consumed at bind time, after its
/// complete shape has been validated, and its immutable lane activity is packed into one byte.
struct PreparedSlot {
    stage: Box<dyn BankStage>,
    active_lanes: u8,
}

const _: () = assert!(
    core::mem::size_of::<PreparedSlot>() <= core::mem::size_of::<BankSlot>(),
    "prepared slot must not exceed the public slot layout"
);

impl PreparedSlot {
    #[inline(always)]
    fn has_active_lanes(&self) -> bool {
        #[cfg(test)]
        TEST_PREPARED_ACTIVITY_QUERIES.with(|count| count.set(count.get() + 1));
        self.active_lanes != 0
    }

    #[inline(always)]
    fn lane_active(&self, lane: usize) -> bool {
        self.active_lanes & (1u8 << lane) != 0
    }
}

#[cfg(test)]
thread_local! {
    static TEST_PREPARED_ACTIVITY_QUERIES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static TEST_PREPARED_DROPS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
fn reset_prepared_activity_observation() {
    TEST_PREPARED_ACTIVITY_QUERIES.with(|count| count.set(0));
}

#[cfg(test)]
fn prepared_activity_observation() -> usize {
    TEST_PREPARED_ACTIVITY_QUERIES.with(std::cell::Cell::get)
}

/// The staged planar tiles for one ordered folded cohort.
pub struct FoldCohort<'a> {
    lane_ids: &'a [usize],
    left: &'a mut [f32],
    right: &'a mut [f32],
    stride: usize,
    frames: usize,
}

impl<'a> FoldCohort<'a> {
    pub fn new(
        lane_ids: &'a [usize],
        left: &'a mut [f32],
        right: &'a mut [f32],
        stride: usize,
        frames: usize,
    ) -> Result<Self, RackError> {
        if lane_ids.is_empty() || lane_ids.len() > 8 || frames == 0 || stride < frames {
            return Err(RackError::Shape);
        }
        if lane_ids
            .iter()
            .enumerate()
            .any(|(index, lane)| lane_ids[..index].contains(lane))
        {
            return Err(RackError::Shape);
        }
        // The public cohort is a complete lane-major bank: every physical ID below the maximum
        // has a stride, including holes. This is the capacity both consumers already require.
        let max_lane = lane_ids.iter().copied().max().ok_or(RackError::Shape)?;
        let required = max_lane
            .checked_add(1)
            .and_then(|lanes| lanes.checked_mul(stride))
            .ok_or(RackError::Overflow)?;
        if left.len() < required || right.len() < required {
            return Err(RackError::Shape);
        }
        Ok(Self {
            lane_ids,
            left,
            right,
            stride,
            frames,
        })
    }

    #[must_use]
    pub fn lane_ids(&self) -> &[usize] {
        self.lane_ids
    }
    #[must_use]
    pub fn stride(&self) -> usize {
        self.stride
    }
    #[must_use]
    pub fn frames(&self) -> usize {
        self.frames
    }
    #[must_use]
    pub fn left(&self) -> &[f32] {
        self.left
    }
    #[must_use]
    pub fn right(&self) -> &[f32] {
        self.right
    }
    pub fn planes_mut(&mut self, index: usize) -> Option<(&mut [f32], &mut [f32])> {
        if !self.lane_ids.contains(&index) {
            return None;
        }
        let start = index.checked_mul(self.stride)?;
        let end = start.checked_add(self.frames)?;
        if end > self.left.len() || end > self.right.len() {
            return None;
        }
        Some((&mut self.left[start..end], &mut self.right[start..end]))
    }
}

/// The resident AoSoA block of a full bank whose **every** lane folds, offered whole to
/// [`BankMembers::fold_resident`] before the scatter transposes a single word (issue #915).
///
/// The cohort is always lanes `0..lanes()` in ascending order ([`Self::lane_ids`]): the same lanes,
/// in the same order, that the staged path's [`FoldCohort`] names for the same bank, which is the
/// order a scatter-accumulate into a destination several lanes share associates in. Both planes
/// hold exactly `frames() * lanes()` words, frame-major, and a lane's sample at frame `f` is at
/// `f * lanes() + lane` -- the layout every [`BankStage`] saw.
///
/// It is a shared view: an implementor reads the resident words and writes only its own
/// destination. The chain's staging block is not part of it, so accepting the offer is what keeps
/// the staging block from being written at all.
#[derive(Clone, Copy)]
pub struct ResidentFoldCohort<'a> {
    left: &'a [f32],
    right: &'a [f32],
    width: BankWidth,
    frames: usize,
}

impl<'a> ResidentFoldCohort<'a> {
    /// Checks the whole shape once, the way [`FoldCohort::new`] does: at least one frame, and each
    /// plane exactly `frames * width.lanes()` words.
    ///
    /// # Errors
    ///
    /// [`RackError::Overflow`] if `frames * width.lanes()` does not fit a `usize`;
    /// [`RackError::Shape`] for a zero frame count or a plane of any other length.
    pub fn new(
        left: &'a [f32],
        right: &'a [f32],
        width: BankWidth,
        frames: usize,
    ) -> Result<Self, RackError> {
        let words = frames
            .checked_mul(width.lanes() as usize)
            .ok_or(RackError::Overflow)?;
        if frames == 0 || left.len() != words || right.len() != words {
            return Err(RackError::Shape);
        }
        Ok(Self {
            left,
            right,
            width,
            frames,
        })
    }

    /// The resident left plane, `frames() * lanes()` words.
    #[must_use]
    pub fn left(&self) -> &'a [f32] {
        self.left
    }
    /// The resident right plane, `frames() * lanes()` words.
    #[must_use]
    pub fn right(&self) -> &'a [f32] {
        self.right
    }
    #[must_use]
    pub const fn width(&self) -> BankWidth {
        self.width
    }
    /// `W`: the bank's lane count, which is also the cohort's size.
    #[must_use]
    pub const fn lanes(&self) -> usize {
        self.width.lanes() as usize
    }
    /// The cohort in accumulation order: every lane of the bank, ascending.
    #[must_use]
    pub const fn lane_ids(&self) -> core::ops::Range<usize> {
        0..self.lanes()
    }
    #[must_use]
    pub const fn frames(&self) -> usize {
        self.frames
    }
}

/// Per-lane planar views a chain gathers from and scatters to. `lane < lanes` always.
pub trait BankMembers {
    fn plane(&self, lane: usize) -> (&[f32], &[f32]);
    fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]);

    /// Borrow every destination of a complete bank at once, or decline to use direct scatter.
    /// Implementors must validate IDs, pairwise disjointness, and capacities before creating any
    /// references. The default preserves the staged path for providers without that capability.
    fn distinct_planes_mut(&mut self, _lanes: usize, _frames: usize) -> Option<BankPlaneViews<'_>> {
        None
    }

    /// One lane's **accumulating auxiliary destination**, or `None` when it has none.
    ///
    /// # What this is for, and why it is not [`BankMembers::plane_mut`] again (issue #210)
    ///
    /// A chain scatters each lane's result into exactly one buffer, and those buffers must stay
    /// pairwise distinct -- the scatter writes them all in one pass, so two lanes sharing one
    /// would race within the pass. `BankChain::new` and the graph's `scatter_redirects` both
    /// enforce that, the latter by declining every redirect in a chain whose targets collide.
    ///
    /// A second destination is a different shape and must not be forced through the same hole. It
    /// is **accumulating**, so two lanes may name the same buffer and the result is their sum, and
    /// it is a *second* write rather than a relocation of the first -- the lane's own scatter
    /// target is untouched. That is what a pre-fade-listen bus needs: an arbitrary subset of lanes
    /// summing into one destination that the rest of the graph reads separately, with no lane
    /// losing its own output. Because such a destination is never a redirect target, the
    /// pairwise-distinct construction check does not see it and cannot decline on it.
    ///
    /// # The seam, and what is deliberately not here
    ///
    /// Nothing in the engine arms this today: every implementor takes the default, `None` on every
    /// lane, and a chain with no armed lane never runs the accumulation loop at all
    /// ([`BankChain::arm_aux`]). PFL itself -- which lanes, which buffer, the arena reservation
    /// that gives the destination a slot, the control surface that turns it on -- is a later
    /// phase's work. What exists now is the shape that phase needs: a per-lane optional
    /// destination on the chain's epilogue, outside the redirect-target set, that can be armed
    /// without restructuring the scatter.
    fn aux_plane_mut(&mut self, lane: usize) -> Option<(&mut [f32], &mut [f32])> {
        let _ = lane;
        None
    }

    /// Take one lane's planar result **instead of** writing it to [`BankMembers::plane_mut`].
    ///
    /// Called exactly once per block, for each lane [`BankChain::arm_fold`] named, at the point in
    /// the scatter where that lane's plane copy would have happened. `left` and `right` are the
    /// lane's own `frames`-word planar tile, already transposed out of the resident block and
    /// mutable, so the implementor may finish the lane in place and put the result wherever it
    /// belongs -- including into a destination several lanes share.
    ///
    /// # Why this is not [`BankMembers::plane_mut`] and not [`BankMembers::aux_plane_mut`]
    ///
    /// `plane_mut` is a *relocation* of the lane's one scatter target and the targets must stay
    /// pairwise distinct; `aux_plane_mut` is a *second*, accumulating destination that leaves the
    /// first standing. This is the third shape: the lane's planar buffer is not written at all, so
    /// there is no scatter target to keep distinct, and what the lane does with its own tile --
    /// apply a constant, sum it into a shared bus, both -- is the implementor's arithmetic and not
    /// the chain's. The chain contributes the transpose it was going to do anyway and nothing else.
    ///
    /// A chain that arms this is asserting that **nothing reads the lane's planar buffer**: it is
    /// left holding the previous block's words, exactly as a merged chain leaves its intermediate
    /// slots. Proving that is the caller's obligation, and the graph runtime's `route_fold` is
    /// where it is proved.
    fn fold_plane(&mut self, lane: usize, left: &mut [f32], right: &mut [f32]) {
        let _ = (lane, left, right);
    }

    /// Fold one ordered cohort. The default delegates to the established per-lane seam.
    fn fold_cohort(&mut self, cohort: FoldCohort<'_>) {
        // Private dimensions and lane IDs were validated by FoldCohort::new and cannot change.
        for lane in cohort.lane_ids.iter().copied() {
            let start = lane * cohort.stride;
            self.fold_plane(
                lane,
                &mut cohort.left[start..start + cohort.frames],
                &mut cohort.right[start..start + cohort.frames],
            );
        }
    }

    /// Fold a whole full bank **straight from its resident block**, or decline (issue #915).
    ///
    /// Offered by the tiled scatter only when every lane of the bank folds, and before it writes
    /// anything: before the transpose into the staging block, before [`Self::fold_cohort`], before
    /// any plane. Returning `true` asserts the implementor has finished every lane's fold -- the
    /// same work the staged path would hand [`Self::fold_cohort`] over the same words, in the same
    /// lane order -- and the chain then writes neither its staging block nor any lane's plane.
    /// Returning `false` asserts the implementor wrote **nothing**, and the chain takes the staged
    /// path exactly as if the offer had never been made.
    ///
    /// The default declines, which keeps every provider without a fused form on the staged path.
    fn fold_resident(&mut self, cohort: ResidentFoldCohort<'_>) -> bool {
        let _ = cohort;
        false
    }

    /// Whether lane `lane`'s gather source holds exact `+0.0` (raw bits zero) on both planes over
    /// its first `frames` words (issue #1107's whole-bank skip input test).
    ///
    /// It reads the same words [`Self::plane`] lends, and returns at the first non-zero word, so
    /// an audible lane pays its head. The default is built on [`Self::plane`]; an implementor
    /// whose `plane` counts reads for its own evidence overrides it with an uncounted read of the
    /// same words, so the test does not move those counts.
    fn input_is_positive_zero(&self, lane: usize, frames: usize) -> bool {
        let (left, right) = self.plane(lane);
        planes_are_positive_zero(left, right, frames)
    }

    /// Finish the fold of a skipped block whose every folded lane is exact `+0.0`, or decline
    /// (issue #1107).
    ///
    /// `lanes` is the mask of the lanes the chain folds, which is every active lane, in ascending
    /// order; their resident words are `+0.0` on both planes over `frames`. Returning `true`
    /// asserts the implementor has written exactly what [`Self::fold_plane`],
    /// [`Self::fold_cohort`] or [`Self::fold_resident`] would write for those lanes over those
    /// words. Returning `false` asserts it wrote nothing, and the chain then runs its ordinary
    /// scatter over the zeroed resident block, which is exact by construction. The default
    /// declines.
    fn fold_silent(&mut self, lanes: u32, frames: u32) -> bool {
        let _ = (lanes, frames);
        false
    }
}

// REALTIME_POLICY_BEGIN
/// Whether both planes hold exact `+0.0` (raw bits zero) on their first `frames` words; `false`
/// when a plane is shorter (issue #1107).
///
/// The shape of `effect_runtime::block_is_positive_zero`: a 32-word head, so an audible plane
/// returns after one short fold, and then 128-word chunks whose `|` fold vectorises.
#[inline]
#[must_use]
pub fn planes_are_positive_zero(left: &[f32], right: &[f32], frames: usize) -> bool {
    let (Some(left), Some(right)) = (left.get(..frames), right.get(..frames)) else {
        return false;
    };
    plane_is_positive_zero(left) && plane_is_positive_zero(right)
}

#[inline(always)]
fn plane_is_positive_zero(words: &[f32]) -> bool {
    let or_bits = |words: &[f32]| words.iter().fold(0_u32, |bits, word| bits | word.to_bits());
    let (head, rest) = words.split_at(words.len().min(32));
    if or_bits(head) != 0 {
        return false;
    }
    rest.chunks(128).all(|chunk| or_bits(chunk) == 0)
}
// REALTIME_POLICY_END

/// An immutable active lane of the final left/right resident bank output.
///
/// The borrowed words contain `frames * width.lanes()` entries per plane; a lane's
/// sample at frame `f` is at `f * width.lanes() + lane`. Only graph's observation
/// point after successful execution can establish current-block and node identity.
#[derive(Clone, Copy)]
pub struct ResidentOutputLane<'a> {
    left: &'a [f32],
    right: &'a [f32],
    frames: u32,
    width: BankWidth,
    lane: usize,
}

impl<'a> ResidentOutputLane<'a> {
    #[must_use]
    pub fn left(&self) -> &'a [f32] {
        self.left
    }
    #[must_use]
    pub fn right(&self) -> &'a [f32] {
        self.right
    }
    #[must_use]
    pub const fn frames(&self) -> u32 {
        self.frames
    }
    #[must_use]
    pub const fn width(&self) -> BankWidth {
        self.width
    }
    #[must_use]
    pub const fn lane(&self) -> usize {
        self.lane
    }
}

/// One bank chain: a resident L/R AoSoA block plus its ordered slots.
///
/// Exactly one gather and one scatter per [`run`](BankChain::run), whatever the slot count
/// (master plan §4.5).
///
/// # The mono collapse
///
/// A chain that gathers its tracks' inputs (issue #970) and whose every active lane renders a track
/// doing bit-identical work on both channels in its **collapsible prefix** runs that prefix over
/// one plane and duplicates the plane into every later slot. The prefix is the leading run of
/// upstream slots with a one-plane body (issue #974): an upstream slot without one, and every slot
/// after it, runs its dual body on the duplicated planes. The whole mechanism is four decisions
/// and one copy:
///
/// 1. **Bind time.** `BankChain::collapse_prefix_of` works out how many
///    leading slots a collapsed block would run one-plane, and answers `0` -- never collapse --
///    unless the seam-side slots are a suffix, the first slot has a one-plane body, and every slot
///    runs on exactly this chain's lanes.
/// 2. **Bind time, again.** [`arm_mono_collapse`](BankChain::arm_mono_collapse) records the
///    *structural* half of the witness, which a chain cannot see for itself. Unarmed is declining.
/// 3. **Per block, before a sample moves.** Every slot's live cells are read and its queues drained
///    ([`BankStage::begin_block`]) and *then* the prefix slots' witness is read, in that order and
///    for the reason `begin_block` documents. A slot after the prefix has no say: it runs dual on
///    the duplicated planes, so its own L/R words -- equal or not -- are the dual run's words.
/// 4. **The seam.** After the prefix, one copy of the resident block's left plane into its right.
///    Everything downstream -- a later upstream slot, the fader, the matrix, the scatter, the route
///    fold, the master accumulation -- then runs the arithmetic it runs on a dual block, in its own
///    operation order.
///
/// Why a partial prefix is exact: the prefix slots see bit-identical planes and symmetric words,
/// so their one-plane body is exact by the [`BankStage::process_mono`] contract; the seam copy
/// makes every later slot's input bit-identical to the dual run's, and that slot runs its dual
/// body, so every later word is the dual run's.
///
/// Leaving the collapsed mode costs one whole-state copy per prefix slot
/// ([`BankStage::desymmetrize`]) on the block that stops, which is what makes the first dual block
/// after it bit-identical to a run that never collapsed.
///
/// # Coming back (mono-collapse M3)
///
/// Engaging is sound only when the two channels' state already agrees, and the witness does not
/// say that. The witness is a statement about a *block's* inputs -- one source channel, bit-equal
/// designed words, no one-channel write, no divergent restore -- and a run whose witness has been
/// false for a while is a run whose two channels have been evolving apart under different words.
/// Re-equal words do not put the state back. Engaging on the witness alone would therefore be
/// engaging on the wrong premise, and it is the reason M2 shipped the disengage as a one-way
/// latch: a chain that stopped stayed stopped, because declining is always safe.
///
/// What the chain carries instead is [`collapse_channels_agree`](Self::collapse_channels_agree),
/// an invariant maintained at every block boundary and read by the dispatch alongside the witness:
///
/// * it is **true at bind**, where every prefix stage's two channels hold the state preparation
///   built them with, and every difference between them is a designed word the witness sees;
/// * it is **re-established by the disengage copy**, which is not an approximation of the
///   counterfactual dual run's right state but literally that state
///   (`Self::disengage_collapse`);
/// * it is **preserved by any dual block whose witness preserves agreement**
///   ([`ChannelSymmetryWitness::AGREEING`]) -- equal inputs over equal state with equal words
///   leave equal state, which is the same induction the engage direction has always rested on;
/// * it is **cleared by any dual block that does not**, and after that only a proof brings it
///   back ([`BankStage::channels_agree`]);
/// * at a plan swap it is **ANDed with what each carried lane's predecessor chain hands over**
///   ([`inherit_channel_agreement`](Self::inherit_channel_agreement), issue #1276 D5), and a
///   predecessor hands over [`carried_channel_agreement`](Self::carried_channel_agreement): its
///   flag only where the clauses above maintained it, `false` from a chain that can never
///   collapse, whose flag is never maintained and so says nothing about its channels.
///
/// So the reachable cycle -- collapse, disengage on a forced-off arm or a lifted-then-restored
/// bypass, re-engage when the switch or the term comes back -- is a cycle the chain takes, and the
/// one transition M2 could not justify is now justified by an invariant rather than by an argument
/// about why the witness went false. Counted in [`collapse_transitions`](Self::collapse_transitions).
pub struct BankChain {
    scratch: AoSoaScratch,
    lanes: usize,
    active: Box<[bool]>,
    slots: Box<[PreparedSlot]>,
    transposes: u64,
    /// Every lane of this chain is active, so the whole bank may transpose in `W`-frame tiles.
    /// Decided once in [`BankChain::new`]; a partial bank never takes the tiled path.
    full_bank: bool,
    /// Lane-major landing block for the tiled scatter, `lanes * stride` words per plane. Empty --
    /// and never allocated -- for a partial bank, which scatters per lane as it always did.
    staging_left: Box<[f32]>,
    /// See [`Self::staging_left`].
    staging_right: Box<[f32]>,
    /// Lanes with an armed accumulating auxiliary destination (issue #210's PFL seam).
    ///
    /// **Empty, and never allocated, on every chain the engine builds today.** The epilogue tests
    /// it once per block with `is_empty`, so an unarmed chain pays one predictable branch and
    /// touches no auxiliary state at all -- that is the "zero cost when absent" claim, and it is
    /// structural rather than measured. See [`BankMembers::aux_plane_mut`] for what arming buys.
    aux: Box<[bool]>,
    /// Lanes whose scatter is handed to [`BankMembers::fold_plane`] instead of written to the
    /// lane's plane (issue #218's route fold).
    ///
    /// Empty means unarmed, and an unarmed chain takes the byte-for-byte scatter it always did:
    /// the mask is tested once per plane per block with `is_empty`, never per lane. See
    /// [`BankChain::arm_fold`].
    fold: Box<[bool]>,
    /// Slots `0..collapse_prefix` are the upstream stages a collapsed block runs one-plane.
    ///
    /// Zero means this chain can never collapse, and it is zero for every chain that fails any
    /// clause of [`BankChain::collapse_prefix_of`] -- including the one that fails it for the most
    /// interesting reason, a chain of nothing but seam-side slots, whose witness is vacuously
    /// symmetric and must never be read as collapse evidence.
    collapse_prefix: usize,
    /// The force-off switch: the second arm of the mono measurement, and a kill switch.
    ///
    /// Bind-time, per chain, and deliberately **not** an environment variable: the paired
    /// measurement builds both arms in one process and alternates them per observation, so a
    /// process-global switch could not express it.
    collapse_forced_off: bool,
    /// The chain rendered its previous block collapsed.
    collapsed: bool,
    /// The two channels' state agrees at the boundary of the next **dual** block.
    ///
    /// The premise the engage direction needs and the witness does not supply. See the type's
    /// `Coming back` section for the five clauses that maintain it; the awkward "next *dual*
    /// block" in the sentence above is the one wrinkle and it is deliberate. A collapsed block
    /// freezes the right channel, so between the block that engages and the block that stops the
    /// two channels genuinely disagree -- but nothing dual reads them in that window, and
    /// [`disengage_collapse`](Self::disengage_collapse) puts them back before anything does. The
    /// flag therefore stays `true` across a collapsed run, which is what lets block `N + 1` collapse
    /// after block `N` did.
    collapse_channels_agree: bool,
    /// The **structural** half of every active lane's witness, joined at bind.
    ///
    /// `false` until [`BankChain::arm_mono_collapse`] says otherwise, and that default is the
    /// whole safety argument for this field. The witness has five terms and a chain can see only
    /// four of them: `SOURCE` -- "this track's two channels are fed by one source channel" -- is
    /// decided on the control plane from the compiled session, keyed by *track id*, before any
    /// prepared object exists, while a chain knows only anonymous lanes. `lane_symmetry` is
    /// therefore **source agnostic by construction** and reports every lane of a two-source track
    /// eligible.
    ///
    /// So a chain that nobody has performed the join for must decline, and does. The caller that
    /// holds both halves -- `PreparedRenderPlan::arm_mono_collapse`, which has the session's
    /// structural witnesses and the plan's per-lane track names -- is the only one that can arm it.
    collapse_source: bool,
    /// Blocks this chain rendered collapsed. Evidence only, read after render is disarmed.
    collapses: u64,
    /// Collapsed blocks whose seam still copied the left plane into the right one: the first
    /// seam-side slot declined [`BankStage::process_from_left`], or no seam-side slot ran (issue
    /// #975). Evidence only, read after render is disarmed.
    seam_copies: u64,
    /// `[disengages, re-engages, agreement proofs]`. Evidence only; see
    /// [`collapse_transitions`](Self::collapse_transitions) for what each one counts and why a
    /// block count alone cannot say it.
    transitions: [u64; 3],
    /// Whether the whole-bank silent skip may run (issue #1107). `true` from construction and in
    /// every shipped build; only the graph's test-support switch clears it, so a differential can
    /// render the same plan with every block processed.
    silent_skip: bool,
    /// Blocks this chain skipped whole (issue #1107). Evidence only.
    silent_skips: u64,
    /// The resident scratch holds exact `+0.0` on every word of both planes, whole quantum: set
    /// by a skipped block that zeroed it, cleared by every block that gathers, acquires or
    /// processes. A run of skips zeroes once, and a resident successor reads it as "this
    /// predecessor's output is silent" without a pass over the words.
    resident_zero: bool,
}

/// The other lane of a [`BankChain::swap_lanes`] (issue #1747).
pub enum ChainLanePeer<'a> {
    /// Another lane of the same chain.
    Same(usize),
    /// A lane of another chain.
    Other(&'a mut BankChain, usize),
}

/// Why [`BankChain::swap_lanes`] exchanged nothing. Every refusal but `Defect` leaves both chains
/// as they were; `Defect` may already have disengaged the collapse and lowered the channel
/// agreement, which is conservative. None carries an allocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChainLaneSwapRefusal {
    /// A lane is out of range or not active: a padded lane never swaps.
    Lane,
    /// The two chains have different slot counts or different mono-collapse arming
    /// ([`BankChain::arm_mono_collapse`]), the two lanes differ in a slot's activity or in the
    /// fold or auxiliary bit, or a slot that is an identity on both lanes has stages that cannot
    /// swap them.
    Shape,
    /// A slot that runs on the two lanes has stages that refused them (kind, program key, taps,
    /// shunt or scratch).
    Slot,
    /// A slot refused after its check passed, a defect: the slots already swapped were swapped
    /// back.
    Defect,
}

/// Owned chain masks validated before graph bindings transfer their processors. The constructor
/// consumes this configuration as its only active/fold source; it cannot arm an existing chain.
pub struct PreparedFoldConfiguration {
    width: BankWidth,
    active: Box<[bool]>,
    fold: Box<[bool]>,
}

impl PreparedFoldConfiguration {
    pub fn new(
        width: BankWidth,
        active: Box<[bool]>,
        fold: Box<[bool]>,
    ) -> Result<Self, RackError> {
        let lanes = width.lanes() as usize;
        if active.len() != lanes
            || !active.iter().any(|lane| *lane)
            || fold.len() != lanes
            || fold
                .iter()
                .zip(active.iter())
                .any(|(fold, active)| *fold && !*active)
        {
            return Err(RackError::Shape);
        }
        Ok(Self {
            width,
            active,
            fold,
        })
    }
}

impl BankChain {
    /// Validates the whole shape once, off the render thread: `active` and every slot mask have
    /// exactly `lanes` entries, a slot may only be active on an active lane, and at least one lane
    /// is active. Scratch lanes start at `+0.0`, so an inactive lane can never hand stale garbage
    /// to the block boundary check.
    pub fn new(
        scratch: AoSoaScratch,
        active: Box<[bool]>,
        slots: Vec<BankSlot>,
    ) -> Result<Self, RackError> {
        let lanes = scratch.width.lanes() as usize;
        if active.len() != lanes || !active.iter().any(|lane| *lane) {
            return Err(RackError::Shape);
        }
        if slots.iter().any(|slot| {
            slot.active_lanes.len() != lanes
                || slot
                    .active_lanes
                    .iter()
                    .zip(active.iter())
                    .any(|(slot_lane, chain_lane)| *slot_lane && !*chain_lane)
        }) {
            return Err(RackError::Shape);
        }
        // AoSoaScratch has no public mutable/reused constructor: these planes are already zeroed.
        // The tiled round trip needs every lane's planar view, and its scatter fully overwrites
        // every lane's planar buffer. Both are only true of a full bank, so a partial one keeps
        // the per-lane scalar path: that is what preserves the invariant above, that an inactive
        // lane is neither read from nor written to and can never hand stale garbage across the
        // block boundary. The standing 64-track fixture is all-full banks; the nine-track ragged
        // fixture is what exercises the fallback.
        let full_bank = active.iter().all(|lane| *lane);
        let staging = if full_bank { scratch.left.len() } else { 0 };
        let collapse_prefix = Self::collapse_prefix_of(&slots, &active);
        let mut prepared_slots = Vec::with_capacity(slots.len());
        for slot in slots {
            let active_lanes = slot
                .active_lanes
                .iter()
                .enumerate()
                .fold(0u8, |mask, (lane, active)| mask | ((*active as u8) << lane));
            debug_assert!(
                slot.stage.live_lanes() == 0 || active_lanes != 0,
                "a slot with a live lane is active: `run` skips the drain of a slot with no active lane"
            );
            prepared_slots.push(PreparedSlot {
                stage: slot.stage,
                active_lanes,
            });
        }
        Ok(Self {
            scratch,
            lanes,
            active,
            slots: prepared_slots.into_boxed_slice(),
            transposes: 0,
            full_bank,
            staging_left: vec![0.0; staging].into_boxed_slice(),
            staging_right: vec![0.0; staging].into_boxed_slice(),
            aux: Box::default(),
            fold: Box::default(),
            collapse_prefix,
            collapse_forced_off: false,
            collapsed: false,
            // True at bind: nothing has rendered, so every prefix stage's two channels hold the
            // state preparation gave them, and any way in which they differ is a designed word the
            // witness compares and declines on.
            collapse_channels_agree: true,
            collapse_source: false,
            collapses: 0,
            seam_copies: 0,
            transitions: [0; 3],
            silent_skip: true,
            silent_skips: 0,
            // Conservative: the planes are built zeroed, but the first skip zeroes them again
            // rather than resting on how the scratch was constructed.
            resident_zero: false,
        })
    }

    /// Construct from masks validated before processor ownership transfer. Ordinary scratch and
    /// slot checks remain those of `new`; installing the configuration introduces no fold error.
    pub fn new_with_prepared_fold(
        scratch: AoSoaScratch,
        configuration: PreparedFoldConfiguration,
        slots: Vec<BankSlot>,
    ) -> Result<Self, RackError> {
        if scratch.width() != configuration.width {
            return Err(RackError::WidthMismatch);
        }
        let mut chain = Self::new(scratch, configuration.active, slots)?;
        chain.install_fold(configuration.fold);
        Ok(chain)
    }

    /// How many leading slots a collapsed block of this chain would run one-plane, or `0`.
    ///
    /// Decided once, at bind, from the slots' own declarations. Four clauses, and every one of them
    /// is a way the collapse could otherwise read or write a plane that was never gathered:
    ///
    /// 1. **The upstream stages are a prefix.** The seam is a position in the strip, not a set:
    ///    everything up to the fader is per-channel arithmetic and everything from it on reads the
    ///    duplicated plane. A chain whose seam-side slots are not a suffix is one this executor
    ///    does not understand, and it declines rather than guessing where to duplicate.
    /// 2. **The prefix is non-empty.** A chain of nothing but fader and matrix slots reports every
    ///    lane symmetric on every session, mono or not (`SEAM_SIDE_WITNESS` is an unconditional
    ///    `SYMMETRIC`), so collapsing on its witness would be collapsing on an unconditional
    ///    `true`. This is `PlanUnitEligibility::witness_is_vacuous` enforced rather than
    ///    reported.
    /// 3. **The prefix ends at the first upstream slot without a one-plane body** (issue #974).
    ///    A dual body handed the ungathered right plane would not merely write garbage into a
    ///    plane the seam overwrites -- a linked detector reads *both* planes to compute the level
    ///    it applies to the left one, so the left output would be wrong too. So the seam sits
    ///    before that slot: it and every later slot run dual on the duplicated planes, which are
    ///    the dual run's inputs bit for bit. A chain whose first slot has no one-plane body has
    ///    an empty prefix and clause 2 declines it.
    /// 4. **Every slot runs on exactly this chain's active lanes.** The collapse is
    ///    all-lanes-or-nothing and the witness is conjoined over the slots a lane runs; a slot that
    ///    is an identity on some lanes would make "every active lane is eligible" and "every slot
    ///    agreed for every active lane" two different statements.
    fn collapse_prefix_of(slots: &[BankSlot], active: &[bool]) -> usize {
        let upstream = slots
            .iter()
            .take_while(|slot| slot.stage.seam_side() == SeamSide::UpstreamOfSeam)
            .count();
        let seam_side_is_a_suffix = slots[upstream..]
            .iter()
            .all(|slot| slot.stage.seam_side() == SeamSide::SeamSide);
        let prefix = slots[..upstream]
            .iter()
            .take_while(|slot| slot.stage.supports_mono_collapse())
            .count();
        let lanes_agree = slots
            .iter()
            .all(|slot| slot.active_lanes.as_ref() == active);
        if prefix > 0 && seam_side_is_a_suffix && lanes_agree {
            prefix
        } else {
            0
        }
    }

    /// Arm an accumulating auxiliary destination on `lanes` (issue #210's PFL seam).
    ///
    /// An armed lane's result is *added* into whatever [`BankMembers::aux_plane_mut`] returns for
    /// it, after the scatter and without touching it, so several lanes may share one destination
    /// and a lane keeps its own output either way. Passing an all-`false` mask disarms the chain
    /// back to its default and returns it to the zero-cost path.
    ///
    /// # Errors
    ///
    /// [`RackError::Shape`] if `lanes` is not exactly this chain's lane count, or if it arms a
    /// lane the chain does not render -- an inactive lane's scratch is never written, so summing
    /// it into a bus would publish the zero fill rather than audio.
    pub fn arm_aux(&mut self, lanes: Box<[bool]>) -> Result<(), RackError> {
        if lanes.len() != self.lanes {
            return Err(RackError::Shape);
        }
        if lanes
            .iter()
            .zip(self.active.iter())
            .any(|(armed, active)| *armed && !*active)
        {
            return Err(RackError::Shape);
        }
        self.aux = if lanes.iter().any(|lane| *lane) {
            lanes
        } else {
            Box::default()
        };
        Ok(())
    }

    /// Lanes with an armed auxiliary destination; empty when the chain has none. Evidence only.
    #[must_use]
    pub fn aux_lanes(&self) -> &[bool] {
        &self.aux
    }

    /// This cohort lane's channel-symmetry witness: the conjunction over every slot of the chain.
    ///
    /// Whole-chain evidence, and so stricter than the collapse decision, which conjoins only the
    /// collapsible prefix's slots (issue #974). The same holds for
    /// [`all_lanes_symmetric`](Self::all_lanes_symmetric),
    /// [`active_lane_eligibility`](Self::active_lane_eligibility) and
    /// [`symmetry_counters`](Self::symmetry_counters).
    ///
    /// Stages maintain designed and live terms at preparation or event-drain boundaries. This
    /// single-lane evidence helper pulls only the requested lane. Render and the bank-wide
    /// helpers publish all lanes once per active stage through `lane_symmetry_bank` instead.
    /// The aggregate is local to the decision, so a later block always sees its completed drains.
    ///
    /// An inactive lane declines: it renders no track, so there is nothing to collapse.
    #[must_use]
    pub fn lane_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
        if !self.active.get(lane).copied().unwrap_or(false) {
            return ChannelSymmetryWitness::DECLINED;
        }
        let mut witness = ChannelSymmetryWitness::SYMMETRIC;
        for slot in &self.slots {
            // A slot that is an identity on this lane renders nothing for it, so it has nothing
            // to say about whether the lane's two channels agree.
            if slot.lane_active(lane) {
                witness = witness.and(slot.stage.lane_symmetry(lane));
            }
        }
        witness
    }

    // REALTIME_POLICY_BEGIN
    /// One publication per active stage of the leading `slots` slots; slot identities contribute
    /// no terms. The collapse decision passes its prefix (issue #974); the evidence helpers pass
    /// the whole chain.
    fn lane_symmetry_bank(&self, slots: usize) -> [ChannelSymmetryWitness; 8] {
        let mut witnesses = [ChannelSymmetryWitness::DECLINED; 8];
        for (witness, active) in witnesses.iter_mut().zip(self.active.iter()) {
            if *active {
                *witness = ChannelSymmetryWitness::SYMMETRIC;
            }
        }
        for slot in &self.slots[..slots] {
            if slot.has_active_lanes() {
                let stage = slot
                    .stage
                    .lane_symmetry_bank(self.scratch.width, slot.active_lanes);
                for lane in 0..self.lanes {
                    if slot.lane_active(lane) {
                        witnesses[lane] = witnesses[lane].and(stage[lane]);
                    }
                }
            }
        }
        witnesses
    }

    fn all_active_witnesses_hold(
        &self,
        witnesses: &[ChannelSymmetryWitness; 8],
        terms: u8,
    ) -> bool {
        witnesses
            .iter()
            .zip(self.active.iter())
            .all(|(witness, active)| !active || witness.holds(terms))
    }
    // REALTIME_POLICY_END

    /// One flag per **active** lane, in lane order: does that lane's whole witness hold?
    ///
    /// The localisable form of [`symmetry_counters`](Self::symmetry_counters), and the form the
    /// plan surface publishes: a count says how many lanes lost a term and never which, while
    /// "exactly that track and no other" is the claim the witness tests actually make. Inactive
    /// lanes are skipped rather than reported false, so the result indexes the same way a bank's
    /// member list does.
    #[must_use]
    pub fn active_lane_eligibility(&self) -> Vec<bool> {
        let witnesses = self.lane_symmetry_bank(self.slots.len());
        (0..self.lanes)
            .filter(|lane| self.active[*lane])
            .map(|lane| witnesses[lane].eligible())
            .collect()
    }

    /// Whether **every** active lane of this cohort is collapse-eligible.
    ///
    /// All-lanes-or-nothing is not a simplification: the unit of savable work is a whole plane
    /// pass over the lane vector, and a SIMD op executes every lane whether or not that lane needs
    /// it, so masking the mono lanes inside a mixed cohort would save nothing. Mixing is the
    /// planner's problem, not the chain's.
    #[must_use]
    pub fn all_lanes_symmetric(&self) -> bool {
        self.all_active_witnesses_hold(
            &self.lane_symmetry_bank(self.slots.len()),
            ChannelSymmetryWitness::SYMMETRIC.terms(),
        )
    }

    /// Whether a **dual** block rendered under this cohort's witness leaves every active lane's two
    /// channels agreeing (mono-collapse M3).
    ///
    /// Strictly weaker than [`all_lanes_symmetric`](Self::all_lanes_symmetric), by exactly the
    /// `UNBYPASSED` term: see [`ChannelSymmetryWitness::AGREEING`] for why a bypass window is the
    /// one way to lose the witness without moving the two channels apart.
    ///
    /// The witnesses are the collapsible prefix's (issue #974): the invariant is over prefix
    /// state, and a later slot's words cannot move the prefix's channels apart.
    ///
    /// Reuses the eligibility publication when armed. The forced-off path publishes only if
    /// this maintenance step is reached, after any disengage has restored channel state.
    fn all_lanes_preserve_agreement(&self, witnesses: &[ChannelSymmetryWitness; 8]) -> bool {
        self.all_active_witnesses_hold(witnesses, ChannelSymmetryWitness::AGREEING)
    }

    /// `[eligible active lanes, active lanes]` for this chain. Evidence and gates only.
    #[must_use]
    pub fn symmetry_counters(&self) -> [u64; 2] {
        let witnesses = self.lane_symmetry_bank(self.slots.len());
        let mut counters = [0_u64; 2];
        for (lane, witness) in witnesses.iter().enumerate().take(self.lanes) {
            if !self.active[lane] {
                continue;
            }
            counters[1] += 1;
            if witness.eligible() {
                counters[0] += 1;
            }
        }
        counters
    }

    /// Hand `lanes`' scatter to [`BankMembers::fold_plane`] instead of writing their planes.
    ///
    /// An armed lane's planar tile is still produced -- the transpose is the one the scatter was
    /// going to do anyway -- but it lands in this chain's staging block and is handed over rather
    /// than copied into the lane's own buffer, which is left holding the previous block's words.
    /// (A full bank whose every lane is armed first offers its resident block to
    /// [`BankMembers::fold_resident`]; when that is accepted the tile never lands anywhere.)
    /// Passing an all-`false` mask disarms the chain back to the byte-for-byte scatter it had
    /// before, and to its zero-cost path.
    ///
    /// Arming allocates the staging block a partial bank does not otherwise own; a full bank
    /// already holds one for its tiled scatter and reuses it. This is bind-time work, never
    /// render-time: `run` allocates nothing whether the chain is armed or not.
    ///
    /// # Errors
    ///
    /// [`RackError::Shape`] if `lanes` is not exactly this chain's lane count, or if it arms a
    /// lane the chain does not render -- an inactive lane's scratch is never written, so folding
    /// it would publish the zero fill rather than audio.
    pub fn arm_fold(&mut self, lanes: Box<[bool]>) -> Result<(), RackError> {
        if lanes.len() != self.lanes {
            return Err(RackError::Shape);
        }
        if lanes
            .iter()
            .zip(self.active.iter())
            .any(|(armed, active)| *armed && !*active)
        {
            return Err(RackError::Shape);
        }
        self.install_fold(lanes);
        Ok(())
    }

    fn install_fold(&mut self, lanes: Box<[bool]>) {
        if !lanes.iter().any(|lane| *lane) {
            self.fold = Box::default();
            return;
        }
        if self.staging_left.len() != self.scratch.left.len() {
            self.staging_left = vec![0.0; self.scratch.left.len()].into_boxed_slice();
            self.staging_right = vec![0.0; self.scratch.right.len()].into_boxed_slice();
        }
        self.fold = lanes;
    }

    /// Lanes whose scatter this chain folds; empty when it folds none. Evidence only.
    #[must_use]
    pub fn fold_lanes(&self) -> &[bool] {
        &self.fold
    }

    // REALTIME_POLICY_BEGIN
    /// Borrow the actual final scratch planes for one active lane.
    ///
    /// This validates dimensions only. The caller must establish successful execution
    /// in the current block and an identical final-output member mapping before offering
    /// it to an observer. In particular, a previous collapse does not prove freshness,
    /// and a dual suffix can change the right plane after the collapse seam's copy.
    #[must_use]
    pub fn final_output_lane(&self, frames: u32, lane: usize) -> Option<ResidentOutputLane<'_>> {
        let width = self.scratch.width;
        let lanes = width.lanes() as usize;
        if frames == 0
            || frames > self.scratch.quantum
            || self.lanes != lanes
            || self.active.len() != lanes
            || !self.active.get(lane).copied().unwrap_or(false)
        {
            return None;
        }
        let words = usize::try_from(frames).ok()?.checked_mul(lanes)?;
        Some(ResidentOutputLane {
            left: self.scratch.left.get(..words)?,
            right: self.scratch.right.get(..words)?,
            frames,
            width,
            lane,
        })
    }

    /// Borrow the actual final scratch planes of the whole bank: `frames * width` words each, every
    /// lane interleaved, inactive lanes included (issue #943).
    ///
    /// [`Self::final_output_lane`]'s shape checks without its per-lane activity test, and under the
    /// same terms: this validates dimensions only, and the caller must establish successful
    /// execution in the current block before reading it. An inactive lane's words are scratch, so
    /// whatever the caller derives from them must never reach an observer.
    #[must_use]
    pub fn final_output_block(&self, frames: u32) -> Option<(&[f32], &[f32])> {
        let lanes = self.scratch.width.lanes() as usize;
        if frames == 0
            || frames > self.scratch.quantum
            || self.lanes != lanes
            || self.active.len() != lanes
        {
            return None;
        }
        let words = usize::try_from(frames).ok()?.checked_mul(lanes)?;
        Some((
            self.scratch.left.get(..words)?,
            self.scratch.right.get(..words)?,
        ))
    }

    /// Gather every active lane, run every non-identity slot over the resident block, scatter every
    /// active lane back. No allocation, no shape `Result`, one transpose round-trip.
    ///
    /// `snapshot` is the block's live snapshot `S` (#1504): every active slot's
    /// [`BankStage::begin_block`] receives it, once, before anything else of the block runs.
    pub fn run<M: BankMembers + ?Sized>(
        &mut self,
        members: &mut M,
        frames: u32,
        first_sample: u64,
        snapshot: LiveSnapshot,
    ) -> Result<(), RenderError> {
        self.run_with_input(members, frames, first_sample, snapshot, None)
    }

    /// Run after graph has proved that the immediately preceding, successfully observed unit
    /// produced these exact inputs in this block. Owners remain independent. Local shape declines
    /// use ordinary gather in this invocation, before the same stages and scatter.
    pub fn run_with_resident_input<M: BankMembers + ?Sized>(
        &mut self,
        predecessor: &BankChain,
        members: &mut M,
        frames: u32,
        first_sample: u64,
        snapshot: LiveSnapshot,
    ) -> Result<(), RenderError> {
        self.run_with_input(members, frames, first_sample, snapshot, Some(predecessor))
    }

    /// Copy one prepared lane's response sections without touching resident audio or render
    /// state. The graph supplies the chain slot and lane from its immutable op mapping.
    pub fn copy_response_snapshot_lane(
        &self,
        slot: usize,
        lane: usize,
        sample_rate_hz: u32,
        request: effect_contract::ResponseSnapshotRequest<'_>,
    ) -> Result<effect_contract::ResponseSnapshotSummary, effect_contract::ResponseAnalysisError>
    {
        if !self.active.get(lane).copied().unwrap_or(false) {
            return Err(effect_contract::ResponseAnalysisError::UnsupportedCapability);
        }
        let Some(prepared) = self.slots.get(slot) else {
            return Err(effect_contract::ResponseAnalysisError::UnsupportedCapability);
        };
        if !prepared.lane_active(lane) {
            return Err(effect_contract::ResponseAnalysisError::UnsupportedCapability);
        }
        prepared
            .stage
            .copy_response_snapshot_lane(lane, sample_rate_hz, request)
    }

    /// Read the lane's prepared/live bypass bit without touching render state.
    pub fn response_snapshot_bypassed(&self, slot: usize, lane: usize) -> bool {
        self.slots
            .get(slot)
            .filter(|prepared| prepared.lane_active(lane))
            .is_some_and(|prepared| prepared.stage.response_snapshot_bypassed(lane))
    }

    fn run_with_input<M: BankMembers + ?Sized>(
        &mut self,
        members: &mut M,
        frames: u32,
        first_sample: u64,
        snapshot: LiveSnapshot,
        predecessor: Option<&BankChain>,
    ) -> Result<(), RenderError> {
        debug_assert!(frames >= 1 && frames <= self.scratch.quantum);
        let len = frames as usize * self.lanes;
        // Validate stored shape before any successor drain or destination write. Graph owns
        // adjacency, current-block freshness, and the lane-wise dataflow/observation proof.
        let predecessor = predecessor.filter(|source| {
            source.width() == self.width()
                && source.quantum() == self.quantum()
                && source.active == self.active
                && source.fold.is_empty()
                && source.aux.is_empty()
                && len <= source.scratch.left.len()
                && len <= source.scratch.right.len()
                && len <= self.scratch.left.len()
                && len <= self.scratch.right.len()
        });
        // Step 0: every slot's live cells read and observation queue drained, before anything else. See
        // `BankStage::begin_block` for why this cannot be folded into `process`.
        for slot in &mut self.slots {
            // The same identity-slot guard `process` has always taken. A slot that is an identity
            // on every lane renders nothing, and draining its queues would consume records for a
            // block it does not run. A slot with a live lane is active: the guard skips a stage
            // that has no live lane to drain, never a drain (#1504 D3).
            if slot.has_active_lanes() {
                slot.stage.begin_block(first_sample, snapshot)?;
            }
        }
        // Publish after every drain. The armed decision combines each stage's lane bank once,
        // then reuses it if declining eligibility requires agreement maintenance below. Stages
        // outside rack may still forward nested processor queries; only this boundary is bulk.
        let armed = self.collapse_prefix > 0 && self.collapse_source && !self.collapse_forced_off;
        // Keep the #235 short-circuit: a chain that cannot collapse publishes nothing. Forced-off
        // chains publish lazily at the maintenance step, after any disengage, in the original order.
        // The witness is the prefix's (issue #974): a later slot runs dual on duplicated planes.
        let witnesses = armed.then(|| self.lane_symmetry_bank(self.collapse_prefix));
        let witness = witnesses.as_ref().is_some_and(|bank| {
            self.all_active_witnesses_hold(bank, ChannelSymmetryWitness::SYMMETRIC.terms())
        });
        // The way back for a chain the invariant has declined. Asked at most once per block per
        // prefix slot, and only inside a *recovery window* -- the chain is otherwise ready to
        // collapse and this is the only thing refusing it -- so a session that never disagrees
        // never calls it at all. See `BankStage::channels_agree`.
        if armed && witness && !self.collapse_channels_agree {
            let proven = self.slots[..self.collapse_prefix]
                .iter()
                .all(|slot| slot.stage.channels_agree());
            if proven {
                self.collapse_channels_agree = true;
                self.transitions[2] = self.transitions[2].saturating_add(1);
            }
        }
        let collapse = armed && witness && self.collapse_channels_agree;
        if self.collapsed && !collapse {
            self.disengage_collapse();
        } else if collapse && !self.collapsed && self.transitions[0] > 0 {
            // A re-engage, and only that: the first engage of a chain that has never disengaged is
            // not one, and neither is a second collapsed block in a row.
            self.transitions[1] = self.transitions[1].saturating_add(1);
        }
        self.collapsed = collapse;
        // The invariant, maintained for the block this dispatch just decided, and guarded so that
        // it costs nothing in the steady state of any session -- which is what lets it be an
        // unconditional rule rather than a mode.
        //
        // The four guards, in the order they are cheapest to refuse:
        //
        // * a **collapsed** block is skipped. It does freeze the right channel, but the disengage
        //   copy repairs that before any dual block reads it, which is exactly what the flag's
        //   "at the next dual block's boundary" wording says;
        // * a chain that **can never collapse** is skipped: nothing reads the flag, so nothing is
        //   owed. This is every stereo-source row and every chain with no collapsible prefix, and
        //   it is why the M3 dispatch adds no work at all to the sessions M2 left alone;
        // * a chain that has **already lost** agreement is skipped: only a proof brings it back,
        //   and the proof is above;
        // * a block whose witness was **eligible** is skipped, because eligible implies preserving.
        //   A declining armed decision reuses its publication. A forced-off decision publishes here
        //   once, only when agreement can still be lost, preserving the original short-circuit.
        //
        // What is left is the case the walk is for: a collapsible chain rendering dual under a
        // witness that is not eligible, with agreement still to lose. That is a bypass window, or
        // it is the episode after which the chain must not come back.
        //
        // `SOURCE` is deliberately not a clause here, and it is the one term that would have to be
        // argued about. It is decided at bind from the compiled session and never moves within a
        // plan, so it cannot be an *episode*: either it holds for every block, and the two planes
        // carry the same samples on all of them, or it does not, and `can_collapse` is false and
        // this chain never collapses at all. A term that cannot change cannot break an invariant
        // whose whole job is to survive change. `arm_mono_collapse` carries the obligation that
        // makes that reading true.
        if !collapse
            && self.can_collapse()
            && self.collapse_channels_agree
            && !witness
            && !self.all_lanes_preserve_agreement(
                &witnesses.unwrap_or_else(|| self.lane_symmetry_bank(self.collapse_prefix)),
            )
        {
            self.collapse_channels_agree = false;
        }
        if collapse {
            self.collapses = self.collapses.saturating_add(1);
        }
        // Issue #1107: the whole-bank skip. The drains and the collapse dispatch above have run,
        // so no acknowledgement or publication is held and the collapse bookkeeping is the
        // block's own. The input test runs first and returns at the first non-zero word, so an
        // audible block pays only its lanes' heads; the rest masks are asked only after it. An
        // armed auxiliary destination never skips.
        if self.silent_skip
            && self.aux.is_empty()
            && self.inputs_are_positive_zero(members, frames, predecessor)
            && self.lanes_at_rest() == self.all_lanes_mask()
        {
            self.skip_block(members, frames, first_sample);
            return Ok(());
        }
        // Every path below writes the resident scratch: the acquisition or gather, then the
        // stages.
        self.resident_zero = false;
        if let Some(source) = predecessor {
            self.acquire_resident_input(source, frames, collapse);
        } else if collapse {
            self.gather_mono(members, frames);
        } else {
            self.gather(members, frames);
        }
        self.transposes = self.transposes.saturating_add(1);
        let prefix = if collapse {
            self.collapse_prefix
        } else {
            self.slots.len()
        };
        for slot in &mut self.slots[..prefix] {
            if slot.has_active_lanes() {
                let block = BankBlock {
                    left: &mut self.scratch.left[..len],
                    right: &mut self.scratch.right[..len],
                    frames,
                    first_sample,
                    lanes: self.lanes,
                };
                if collapse {
                    slot.stage.process_mono(block)?;
                } else {
                    slot.stage.process(block)?;
                }
            }
        }
        if collapse {
            // The seam. The right plane is the left one from here on, and it is the whole of the
            // duplication: everything after it -- the fader, the matrix, the scatter, the route
            // fold, the master accumulation -- reads two planes and runs exactly the arithmetic it
            // runs on a dual block, in exactly its operation order. The fold's `ll*l + lr*r` is
            // *not* rewritten as `(ll + lr) * l` on the strength of `l == r`: the two round
            // differently, and the second is not what a dual run computes.
            //
            // The first slot after the prefix that runs takes the seam (issue #975): it reads the
            // left plane as its right input and writes the right plane itself, or copies first. No
            // reader needs the right plane before it. That slot may be an upstream slot with no
            // one-plane body (issue #974); it and every later slot run dual. When no slot after the
            // prefix runs, the chain copies before the scatter.
            let mut seam = true;
            for slot in &mut self.slots[self.collapse_prefix..] {
                if slot.has_active_lanes() {
                    let block = BankBlock {
                        left: &mut self.scratch.left[..len],
                        right: &mut self.scratch.right[..len],
                        frames,
                        first_sample,
                        lanes: self.lanes,
                    };
                    if seam {
                        seam = false;
                        if !slot.stage.process_from_left(block)? {
                            self.seam_copies = self.seam_copies.saturating_add(1);
                        }
                    } else {
                        slot.stage.process(block)?;
                    }
                }
            }
            if seam {
                let AoSoaScratch { left, right, .. } = &mut self.scratch;
                right[..len].copy_from_slice(&left[..len]);
                self.seam_copies = self.seam_copies.saturating_add(1);
            }
        }
        self.scatter(members, frames);
        // The epilogue. Empty on every chain the engine builds today, so this is one branch and
        // no auxiliary state; see `BankMembers::aux_plane_mut` for what arming it is for.
        if !self.aux.is_empty() {
            self.accumulate_aux(members, frames);
        }
        Ok(())
    }

    /// Whether every active lane's input for this block is exact `+0.0` on both planes over
    /// `frames` (issue #1107): for a resident-input run, the predecessor chain skipped this block
    /// and so holds a zeroed scratch; otherwise each active lane's gather source, lane by lane,
    /// returning at the first non-zero word. An inactive lane's scratch words are never gathered.
    fn inputs_are_positive_zero<M: BankMembers + ?Sized>(
        &self,
        members: &M,
        frames: u32,
        predecessor: Option<&BankChain>,
    ) -> bool {
        if let Some(source) = predecessor {
            return source.resident_zero;
        }
        let frames = frames as usize;
        self.active
            .iter()
            .enumerate()
            .all(|(lane, active)| !*active || members.input_is_positive_zero(lane, frames))
    }

    /// Skip one block whole (issue #1107): every lane's input is exact `+0.0` and every lane,
    /// padded lanes included, is at rest after this block's drains, so a rendered block would
    /// leave every state word as it is and write `+0.0` everywhere.
    ///
    /// The resident scratch is zeroed once per run of skips, so the final-output readers -- the
    /// observers, the meters and a resident successor -- read the `+0.0` a rendered block leaves
    /// there. Every slot with an active lane advances its clocks over that zero block in place of
    /// `process`, on every lane. On a collapsed block that includes the prefix stages' frozen
    /// right channel: nothing reads it before the disengage copies the left channel over it, and
    /// the swap and carry paths disengage first. No gather, no stage, no seam copy and no
    /// transpose; then the outputs: `+0.0` into each unfolded active lane's plane, and the folded
    /// lanes' master effect through [`BankMembers::fold_silent`], or the ordinary scatter over the
    /// zeroed block when that declines.
    fn skip_block<M: BankMembers + ?Sized>(
        &mut self,
        members: &mut M,
        frames: u32,
        first_sample: u64,
    ) {
        let len = frames as usize * self.lanes;
        if !self.resident_zero {
            self.scratch.left.fill(0.0);
            self.scratch.right.fill(0.0);
            self.resident_zero = true;
        }
        let all = self.all_lanes_mask();
        for slot in &mut self.slots {
            if slot.has_active_lanes() {
                slot.stage.advance_clocks(
                    all,
                    BankBlock {
                        left: &mut self.scratch.left[..len],
                        right: &mut self.scratch.right[..len],
                        frames,
                        first_sample,
                        lanes: self.lanes,
                    },
                );
            }
        }
        self.silent_skips = self.silent_skips.saturating_add(1);
        let frames_used = frames as usize;
        let mut folded = 0_u32;
        let mut unfolded = false;
        for lane in 0..self.lanes {
            if !self.active[lane] {
                continue;
            }
            if Self::mask_bit(&self.fold, lane) {
                folded |= 1 << lane;
            } else {
                unfolded = true;
            }
        }
        if folded != 0 && (unfolded || !members.fold_silent(folded, frames)) {
            // A chain that folds only some of its lanes, or members that decline: the ordinary
            // scatter over the zeroed resident block writes exactly what a rendered block would.
            self.scatter(members, frames);
            return;
        }
        if folded == 0 {
            for lane in 0..self.lanes {
                if self.active[lane] {
                    let (left, right) = members.plane_mut(lane);
                    left.iter_mut()
                        .take(frames_used)
                        .for_each(|word| *word = 0.0);
                    right
                        .iter_mut()
                        .take(frames_used)
                        .for_each(|word| *word = 0.0);
                }
            }
        }
    }
    // REALTIME_POLICY_END

    /// The disengage boundary: restore every collapsed stage's right-channel state and retire.
    ///
    /// A collapsed run evolves one channel. The counterfactual dual run's right state is, by the
    /// induction the witness maintains, the left one -- so copying it is not an approximation of
    /// the dual run, it **is** the dual run's state, and the first dual block after this renders
    /// exactly what a never-collapsed run would have.
    ///
    /// Only the prefix stages are copied, because only they ran one-plane: a seam-side slot, and
    /// every slot after the prefix, ran dual on every block, collapsed or not, and its two channels
    /// are already whatever the strip asked for.
    ///
    /// This is a one-off bounded copy of the prefix state per cohort on the block that stops
    /// collapsing, and it happens at most once per chain per plan. The amount follows each
    /// stage's prepared state rather than a compressor-specific ring layout.
    fn disengage_collapse(&mut self) {
        for slot in &mut self.slots[..self.collapse_prefix] {
            slot.stage.desymmetrize();
        }
        self.transitions[0] = self.transitions[0].saturating_add(1);
        // The copy **is** the proof, not evidence for one: every prefix stage's right channel now
        // holds its left channel's whole state, so the two agree by construction at this boundary.
        // M2 set a one-way latch here instead, because it had no invariant to hand this fact to.
        // The block that takes this branch still renders *dual*, and the maintenance step in `run`
        // will clear the flag again if that block's witness does not preserve agreement -- which is
        // precisely the disengage-for-cause case, and precisely the case that must not re-engage.
        //
        // The assignment is redundant today and is kept anyway, which is worth being explicit
        // about rather than quietly relying on: reaching here requires `self.collapsed`, and a
        // chain only collapses while the flag holds, so the flag is already `true`. What the line
        // states is the *reason* it stays true across a boundary where the right channel was
        // frozen, and it is the line a future disengage path -- one reached from somewhere other
        // than a collapsed block -- would otherwise have to remember to add. The debug assertion
        // is what keeps the redundancy honest: if it ever stops holding, this becomes mechanism.
        debug_assert!(
            self.collapse_channels_agree,
            "a chain only collapses while its channels agree, so a disengage cannot find them apart"
        );
        self.collapse_channels_agree = true;
    }

    // REALTIME_POLICY_BEGIN
    /// Planar -> AoSoA for the **left plane only**: the collapsed cohort's gather.
    ///
    /// [`Self::gather`]'s two shapes, one plane each. The right plane is not read and the right
    /// scratch is not written; the seam writes it, after the prefix and before anything reads it.
    /// Copy exactly the words ordinary acquisition overwrites. The mode is the successor's
    /// post-drain collapse decision, independent of how its predecessor rendered this block.
    fn acquire_resident_input(&mut self, source: &BankChain, frames: u32, mono: bool) {
        let len = frames as usize * self.lanes;
        if self.full_bank {
            self.scratch.left[..len].copy_from_slice(&source.scratch.left[..len]);
            if !mono {
                self.scratch.right[..len].copy_from_slice(&source.scratch.right[..len]);
            }
        } else {
            for lane in 0..self.lanes {
                if self.active[lane] {
                    for frame in 0..frames as usize {
                        let word = frame * self.lanes + lane;
                        self.scratch.left[word] = source.scratch.left[word];
                        if !mono {
                            self.scratch.right[word] = source.scratch.right[word];
                        }
                    }
                }
            }
        }
    }

    fn gather_mono<M: BankMembers + ?Sized>(&mut self, members: &M, frames: u32) {
        if self.full_bank {
            effect_contract::match_bank_width!(self.scratch.width, |_, W, transpose| {
                self.gather_mono_tiled::<M, W>(members, frames, transpose)
            });
            return;
        }
        for lane in 0..self.lanes {
            if self.active[lane] {
                let (left, _right) = members.plane(lane);
                self.scratch.gather_lane_left(lane, left, 0, frames);
            }
        }
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    #[inline(always)]
    fn gather_mono_tiled<M: BankMembers + ?Sized, const W: usize>(
        &mut self,
        members: &M,
        frames: u32,
        transpose: impl Fn([[f32; W]; W]) -> [[f32; W]; W] + Copy,
    ) {
        debug_assert_eq!(self.lanes, W);
        let frames_used = frames as usize;
        let tiled = (frames_used / W) * W;
        let mut left_planes: [&[f32]; W] = [&[]; W];
        for (lane, plane) in left_planes.iter_mut().enumerate() {
            let (left, right) = members.plane(lane);
            debug_assert!(left.len() == frames_used && right.len() == frames_used);
            *plane = left;
        }
        tile_gather(&mut self.scratch.left[..tiled * W], &left_planes, transpose);
        if tiled < frames_used {
            for (lane, plane) in left_planes.iter().enumerate() {
                self.scratch.gather_lane_left(lane, plane, tiled, frames);
            }
        }
    }
    // REALTIME_POLICY_END

    /// Record the structural half of this cohort's witness: the `SOURCE` term, joined at bind.
    ///
    /// `structural` is "every active lane of this chain renders a track whose two channels read
    /// one source channel **and this chain gathers that track's input**" (issue #970: a later chain
    /// of a split strip reads planes an earlier stage produced, which the source mapping says
    /// nothing about). Passing `false`, or never calling this at all, makes the chain decline
    /// forever. See the `collapse_source` field for why the chain cannot derive this itself.
    ///
    /// # The obligation, which M3's invariant rests on
    ///
    /// Bind-time: off the render thread, once, before this chain renders its first block.
    /// `PreparedRenderPlan::arm_mono_collapse` enforces the render-scope half with an assertion.
    /// The block-order half is a caller obligation because the chain cannot check it, and it is
    /// what lets [`run`](Self::run) leave `SOURCE` out of the agreement invariant: a chain armed
    /// before it renders has one answer to "do the two planes carry the same samples" for its whole
    /// life, so that term cannot be the cause of a *transient* divergence. Arming a chain that has
    /// already rendered blocks fed by two different source channels would break that reading, and
    /// nothing in the engine does it -- the join runs once, from the compiled session, at bind.
    pub const fn arm_mono_collapse(&mut self, structural: bool) {
        self.collapse_source = structural;
    }

    /// Whether this chain can collapse at all: it has an upstream prefix and the join was armed.
    #[must_use]
    pub const fn can_collapse(&self) -> bool {
        self.collapse_prefix > 0 && self.collapse_source
    }

    /// Force this chain's collapse off (or back on), for the paired measurement's second arm.
    ///
    /// Bind-time. Forcing it off on a chain that is currently collapsed does **not** skip the
    /// disengage copy: the next `run` sees `collapsed && !collapse` and takes it, which is the
    /// same boundary a witness going false takes.
    pub const fn force_mono_collapse_off(&mut self, forced: bool) {
        self.collapse_forced_off = forced;
    }

    /// Blocks this chain rendered collapsed. Read only after render is disarmed.
    #[must_use]
    pub const fn collapses(&self) -> u64 {
        self.collapses
    }

    /// Collapsed blocks whose seam still copied the left plane into the right one (issue #975):
    /// `0` while every collapsed block's first seam-side slot wrote the right plane from the left
    /// one itself. Read only after render is disarmed.
    #[must_use]
    pub const fn seam_copies(&self) -> u64 {
        self.seam_copies
    }

    /// Leading slots a collapsed block runs one-plane; `0` when this chain can never collapse.
    #[must_use]
    pub const fn collapse_prefix(&self) -> usize {
        self.collapse_prefix
    }

    /// Whether this chain rendered its last block collapsed. Evidence only.
    #[must_use]
    pub const fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    /// Whether the two channels' state agrees at the next dual block's boundary.
    ///
    /// The re-engage rule's premise, and `false` is the retirement M2 latched: a chain whose
    /// channels have been driven apart does not collapse again until something proves they agree.
    /// Maintained only while [`can_collapse`](Self::can_collapse) holds: a chain that can never
    /// collapse keeps the `true` it was bound with whatever its channels hold, so a reader outside
    /// this chain's own dispatch reads [`carried_channel_agreement`](Self::carried_channel_agreement)
    /// instead. Evidence only.
    #[must_use]
    pub const fn collapse_channels_agree(&self) -> bool {
        self.collapse_channels_agree
    }

    /// The channel agreement this chain hands a successor chain that takes carried lane state
    /// from it at a plan swap (issue #1276 D5): [`collapse_channels_agree`](Self::collapse_channels_agree)
    /// where the dispatch maintains it, and `false` on a chain that can never collapse.
    ///
    /// The second half is the premise the flag alone does not carry. The dispatch keeps the flag
    /// only on a chain that can collapse, so an unarmed chain -- a stereo-source neighbour in the
    /// bank, or a mono-source track with an asymmetric input delay -- still reads `true` after its
    /// two channels have been driven apart. Handing that `true` to an armed successor would let it
    /// collapse a lane whose channels disagree, which moves output bits. `false` is conservative:
    /// a successor whose collapse prefix holds only builtins re-arms after one proof
    /// ([`BankStage::channels_agree`]), but one with a console or insert effect in that prefix
    /// stays dual for the rest of its plan, since every launch effect declines the proof. `false`
    /// reaches an armed successor only from a chain that could not collapse, so in practice from
    /// a track whose class changed in the transaction, whose channels really may differ.
    #[must_use]
    pub const fn carried_channel_agreement(&self) -> bool {
        self.can_collapse() && self.collapse_channels_agree
    }

    // REALTIME_POLICY_BEGIN
    /// Lend slot `slot`'s stage mutably, or `None` past the last slot (issue #1276 D3).
    ///
    /// For the crate that built the stage, at bind and at a plan swap's carry, to reach its own
    /// stage type through [`BankStage::as_any_mut`]. Never called by [`run`](Self::run).
    pub fn slot_stage_mut(&mut self, slot: usize) -> Option<&mut dyn BankStage> {
        Some(self.slots.get_mut(slot)?.stage.as_mut())
    }

    /// Apply the pending `Observe` records of every lane a plan swap's carry marked, on every
    /// slot (#1280 D3, #1816): [`BankStage::carry_observes`] once per slot, after the carry moved
    /// every state. The marks select, so no slot is skipped for being inactive. The chain renders
    /// no more. Allocation-, lock- and syscall-free; bounded by the slots and each marked queue's
    /// count at entry.
    pub fn carry_observes(&mut self, first_sample: u64) {
        for slot in &mut self.slots {
            slot.stage.carry_observes(first_sample);
        }
    }

    /// Prepare this chain to hand lane state to a successor plan (issue #1276 D5): when it
    /// rendered its last block collapsed, take the disengage boundary now, so both channels of
    /// every prefix stage hold real state. The chain renders no more. Allocation-free; a no-op
    /// on a chain that rendered dual.
    pub fn disengage_for_carry(&mut self) {
        if self.collapsed {
            self.disengage_collapse();
            self.collapsed = false;
        }
    }

    /// Fold in the channel agreement of a chain this one takes carried lane state from (issue
    /// #1276 D5): this chain's flag becomes the AND of its own and `agree`, which is that chain's
    /// [`carried_channel_agreement`](Self::carried_channel_agreement). A chain built at bind
    /// agrees, and a lane at rest agrees, so a successor whose lanes came from chains that agreed
    /// may collapse at once; one that took a lane from a chain whose channels had been driven
    /// apart waits for a proof ([`BankStage::channels_agree`]), as that chain would have.
    pub fn inherit_channel_agreement(&mut self, agree: bool) {
        self.collapse_channels_agree &= agree;
    }

    /// The scratch bytes [`Self::swap_lanes`] needs: the most any slot's swap needs.
    #[must_use]
    pub fn lane_swap_scratch_bytes(&self) -> usize {
        self.slots
            .iter()
            .map(|slot| slot.stage.lane_swap_scratch_bytes())
            .max()
            .unwrap_or(0)
    }

    /// Whether lane `lane`'s bit of `mask` is set: an empty mask is an unarmed one.
    fn mask_bit(mask: &[bool], lane: usize) -> bool {
        mask.get(lane).copied().unwrap_or(false)
    }

    /// Whether lane `lane` of this chain and lane `other_lane` of `other` (`None`: this chain)
    /// can trade their whole state with a scratch of `scratch_bytes`; the refusal if not.
    fn lane_swap_check(
        &self,
        lane: usize,
        other: Option<&Self>,
        other_lane: usize,
        scratch_bytes: usize,
    ) -> Result<(), ChainLaneSwapRefusal> {
        let peer = other.unwrap_or(self);
        if !Self::mask_bit(&self.active, lane) || !Self::mask_bit(&peer.active, other_lane) {
            return Err(ChainLaneSwapRefusal::Lane);
        }
        let swaps = |own: &PreparedSlot, theirs: &PreparedSlot| {
            let peer = match other {
                Some(_) => LanePeer::Other(&*theirs.stage, other_lane),
                None => LanePeer::Same(other_lane),
            };
            own.stage.can_swap_lanes(lane, peer, scratch_bytes)
        };
        if self.slots.len() != peer.slots.len()
            || self.collapse_source != peer.collapse_source
            || Self::mask_bit(&self.fold, lane) != Self::mask_bit(&peer.fold, other_lane)
            || Self::mask_bit(&self.aux, lane) != Self::mask_bit(&peer.aux, other_lane)
            || self
                .slots
                .iter()
                .zip(peer.slots.iter())
                .any(|(own, theirs)| {
                    own.lane_active(lane) != theirs.lane_active(other_lane)
                        || (!own.lane_active(lane) && !swaps(own, theirs))
                })
        {
            return Err(ChainLaneSwapRefusal::Shape);
        }
        if self
            .slots
            .iter()
            .zip(peer.slots.iter())
            .all(|(own, theirs)| swaps(own, theirs))
        {
            Ok(())
        } else {
            Err(ChainLaneSwapRefusal::Slot)
        }
    }

    /// Exchange lane `lane`'s whole state with `peer`'s lane, in every slot that runs on it, at a
    /// block boundary on the render thread (issue #1747): the move #1107's packing makes when it
    /// trades an active strip into a silent strip's position.
    ///
    /// Every refusal is decided before the first write and leaves both chains as they were: a lane
    /// that is not active (a padded lane never swaps), two chains of different slot counts or of
    /// different mono-collapse arming, slot activity, fold or auxiliary bits that differ between
    /// the two lanes, or any slot whose stages refuse ([`BankStage::can_swap_lanes`]; `scratch`
    /// must hold [`Self::lane_swap_scratch_bytes`]). Every slot is asked and every slot swaps,
    /// one that is an identity on both lanes too: a stage that cannot swap is refused, never
    /// skipped, so no lane's state in any slot stays behind.
    ///
    /// The arming refusal is what keeps a cross-chain swap exact. The `SOURCE` half of the
    /// witness is joined per chain, at bind, by the graph (`arm_mono_collapse`, which knows the
    /// tracks a chain renders and a chain does not); a stereo-source strip moved into a chain
    /// armed for mono sources would have its right channel replaced by its left. Refusing the
    /// swap keeps that join where the track identity is. An in-chain swap has one arming.
    ///
    /// Then the two chains' channel agreement is taken -- each chain's
    /// [`carried_channel_agreement`](Self::carried_channel_agreement), so an unarmed chain hands
    /// over `false`, as at a plan swap (#1276 D5) -- before anything changes; a chain that
    /// rendered its last block collapsed takes the disengage boundary
    /// ([`Self::disengage_for_carry`]), so both channels of its prefix stages hold real state
    /// before any lane's right channel moves (the disengage changes no agreement: it runs only on
    /// a collapsed chain, whose flag is already `true`); the AND goes into both chains, since each
    /// now holds a lane of the other; and the slots swap in order. A slot that refuses after its
    /// check passed is a defect: the slots already swapped are swapped back and the call returns
    /// [`ChainLaneSwapRefusal::Defect`].
    ///
    /// The resident scratch -- the final output [`Self::final_output_lane`] and
    /// [`Self::final_output_block`] lend -- is not swapped: for the two lanes it is stale until the
    /// next [`Self::run`] rewrites it. Allocation-free and bounded by the slots' prepared state
    /// sizes and latencies.
    ///
    /// # Errors
    ///
    /// [`ChainLaneSwapRefusal`], with both chains unchanged.
    pub fn swap_lanes(
        &mut self,
        lane: usize,
        peer: ChainLanePeer<'_>,
        scratch: &mut [u8],
    ) -> Result<(), ChainLaneSwapRefusal> {
        match peer {
            ChainLanePeer::Same(other_lane) => {
                self.lane_swap_check(lane, None, other_lane, scratch.len())?;
                if lane == other_lane {
                    return Ok(());
                }
                self.disengage_for_carry();
                let slots = self.slots.len();
                for index in 0..slots {
                    if !self.slots[index].stage.swap_lanes(
                        lane,
                        LanePeerMut::Same(other_lane),
                        scratch,
                    ) {
                        debug_assert!(false, "a checked slot swaps its lanes");
                        for undo in self.slots[..index].iter_mut().rev() {
                            let undone =
                                undo.stage
                                    .swap_lanes(lane, LanePeerMut::Same(other_lane), scratch);
                            debug_assert!(undone, "a swapped slot swaps back");
                        }
                        return Err(ChainLaneSwapRefusal::Defect);
                    }
                }
            }
            ChainLanePeer::Other(other, other_lane) => {
                self.lane_swap_check(lane, Some(other), other_lane, scratch.len())?;
                let agree = self.carried_channel_agreement() && other.carried_channel_agreement();
                self.disengage_for_carry();
                other.disengage_for_carry();
                self.collapse_channels_agree = agree;
                other.collapse_channels_agree = agree;
                let slots = self.slots.len();
                for index in 0..slots {
                    let (own, theirs) = (&mut self.slots[index], &mut other.slots[index]);
                    if !own.stage.swap_lanes(
                        lane,
                        LanePeerMut::Other(&mut *theirs.stage, other_lane),
                        scratch,
                    ) {
                        debug_assert!(false, "a checked slot swaps its lanes");
                        for (own, theirs) in self.slots[..index]
                            .iter_mut()
                            .zip(other.slots[..index].iter_mut())
                            .rev()
                        {
                            let undone = own.stage.swap_lanes(
                                lane,
                                LanePeerMut::Other(&mut *theirs.stage, other_lane),
                                scratch,
                            );
                            debug_assert!(undone, "a swapped slot swaps back");
                        }
                        return Err(ChainLaneSwapRefusal::Defect);
                    }
                }
            }
        }
        Ok(())
    }
    // REALTIME_POLICY_END

    /// `[disengages, re-engages, agreement proofs]` for this chain. Evidence only.
    ///
    /// Three numbers because the cycle has three edges and a block count sees none of them: a
    /// chain that collapsed for every block and one that collapsed, stopped and started again can
    /// report the same [`collapses`](Self::collapses), and the second is the transition the
    /// milestone is about. A **re-engage** is an engage by a chain that has disengaged at least
    /// once; an **agreement proof** is a recovery through [`BankStage::channels_agree`], which is
    /// the only way the invariant comes back other than the disengage copy that sets it.
    #[must_use]
    pub const fn collapse_transitions(&self) -> [u64; 3] {
        self.transitions
    }

    // REALTIME_POLICY_BEGIN
    /// Adds every armed lane's resident result into its auxiliary destination.
    ///
    /// Reads the resident block rather than the scattered planes, so it is unaffected by whether
    /// a lane's scatter was redirected, and it *accumulates* rather than assigns, so two lanes may
    /// name one destination and a third may leave that destination's prior contents standing.
    fn accumulate_aux<M: BankMembers + ?Sized>(&mut self, members: &mut M, frames: u32) {
        let Self {
            scratch,
            lanes,
            aux,
            ..
        } = self;
        let lanes = *lanes;
        let frames = frames as usize;
        for lane in 0..lanes {
            if !aux[lane] {
                continue;
            }
            let Some((left, right)) = members.aux_plane_mut(lane) else {
                continue;
            };
            debug_assert!(left.len() == frames && right.len() == frames);
            for frame in 0..frames {
                left[frame] += scratch.left[frame * lanes + lane];
                right[frame] += scratch.right[frame * lanes + lane];
            }
        }
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    /// Planar -> AoSoA for the whole bank: one tiled transpose per plane when every lane is
    /// active, the per-lane scalar move otherwise.
    ///
    /// A tile is `W` frames wide, `W` being the bank width, so this is the same code at four lanes
    /// and at eight. Frames past the last whole tile are the ragged tail and take the scalar path
    /// too -- there is no partial-tile transpose and no read past `frames`.
    fn gather<M: BankMembers + ?Sized>(&mut self, members: &M, frames: u32) {
        if self.full_bank {
            effect_contract::match_bank_width!(self.scratch.width, |_, W, transpose| {
                self.gather_tiled::<M, W>(members, frames, transpose)
            });
            return;
        }
        for lane in 0..self.lanes {
            if self.active[lane] {
                let (left, right) = members.plane(lane);
                self.scratch.gather_lane(lane, left, right, 0, frames);
            }
        }
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    #[inline(always)]
    fn gather_tiled<M: BankMembers + ?Sized, const W: usize>(
        &mut self,
        members: &M,
        frames: u32,
        transpose: impl Fn([[f32; W]; W]) -> [[f32; W]; W] + Copy,
    ) {
        debug_assert_eq!(self.lanes, W);
        let frames_used = frames as usize;
        let tiled = (frames_used / W) * W;
        // `plane` takes `&self`, so every lane's planar view can be held at once: the gather needs
        // no staging block, only `W` live shared borrows.
        let mut left_planes: [&[f32]; W] = [&[]; W];
        let mut right_planes: [&[f32]; W] = [&[]; W];
        for lane in 0..W {
            let (left, right) = members.plane(lane);
            debug_assert!(left.len() == frames_used && right.len() == frames_used);
            left_planes[lane] = left;
            right_planes[lane] = right;
        }
        tile_gather(&mut self.scratch.left[..tiled * W], &left_planes, transpose);
        tile_gather(
            &mut self.scratch.right[..tiled * W],
            &right_planes,
            transpose,
        );
        if tiled < frames_used {
            for lane in 0..W {
                self.scratch.gather_lane(
                    lane,
                    left_planes[lane],
                    right_planes[lane],
                    tiled,
                    frames,
                );
            }
        }
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    /// AoSoA -> planar for the whole bank, the inverse of [`Self::gather`] under the same rule.
    ///
    /// A folded lane (see [`BankChain::arm_fold`]) takes the same transpose and then goes to
    /// [`BankMembers::fold_plane`] instead of to its own plane. On the per-lane scalar path that
    /// means transposing into this chain's staging block first, which is the one thing arming
    /// costs a partial bank; the tiled path already lands there -- unless every lane folds and the
    /// members accept the resident block instead ([`BankMembers::fold_resident`], issue #915).
    fn scatter<M: BankMembers + ?Sized>(&mut self, members: &mut M, frames: u32) {
        if self.full_bank {
            effect_contract::match_bank_width!(self.scratch.width, |_, W, transpose| {
                self.scatter_tiled::<M, W>(members, frames, transpose)
            });
            return;
        }
        if self.fold.is_empty() {
            for lane in 0..self.lanes {
                if self.active[lane] {
                    let (left, right) = members.plane_mut(lane);
                    self.scratch.scatter_lane(lane, left, right, 0, frames);
                }
            }
            return;
        }
        let all_active_folded = (0..self.lanes).any(|lane| self.active[lane])
            && (0..self.lanes).all(|lane| !self.active[lane] || self.fold[lane]);
        let Self {
            scratch,
            lanes,
            active,
            staging_left,
            staging_right,
            fold,
            ..
        } = self;
        let stride = scratch.quantum as usize;
        let used = frames as usize;
        let mut folded_lanes = [0usize; 8];
        let mut folded_count = 0;
        for lane in 0..*lanes {
            if !active[lane] {
                continue;
            }
            if fold[lane] {
                let left = &mut staging_left[lane * stride..lane * stride + used];
                let right = &mut staging_right[lane * stride..lane * stride + used];
                scratch.scatter_lane(lane, left, right, 0, frames);
                if all_active_folded {
                    folded_lanes[folded_count] = lane;
                    folded_count += 1;
                } else {
                    members.fold_plane(lane, left, right);
                }
            } else {
                let (left, right) = members.plane_mut(lane);
                scratch.scatter_lane(lane, left, right, 0, frames);
            }
        }
        if folded_count != 0
            && let Ok(cohort) = FoldCohort::new(
                &folded_lanes[..folded_count],
                staging_left,
                staging_right,
                stride,
                used,
            )
        {
            members.fold_cohort(cohort);
        }
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    #[inline(always)]
    fn scatter_tiled<M: BankMembers + ?Sized, const W: usize>(
        &mut self,
        members: &mut M,
        frames: u32,
        transpose: impl Fn([[f32; W]; W]) -> [[f32; W]; W] + Copy,
    ) {
        debug_assert_eq!(self.lanes, W);
        let frames_used = frames as usize;
        let stride = self.scratch.quantum as usize;
        if self.fold.is_empty()
            && let Some(mut destinations) = members.distinct_planes_mut(W, frames_used)
        {
            if !destinations.supports(W, frames_used) {
                // Drop every view before taking the established staged fallback.
            } else {
                match &mut destinations.0 {
                    BankPlaneViewsInner::Four(pairs, _) => {
                        tile_scatter_direct_plane(
                            &self.scratch.left,
                            pairs,
                            frames_used,
                            false,
                            transpose_tile_4,
                        );
                        tile_scatter_direct_plane(
                            &self.scratch.right,
                            pairs,
                            frames_used,
                            true,
                            transpose_tile_4,
                        );
                    }
                    #[cfg(target_feature = "avx2")]
                    BankPlaneViewsInner::Eight(pairs, _) => {
                        tile_scatter_direct_plane(
                            &self.scratch.left,
                            pairs,
                            frames_used,
                            false,
                            effect_contract::transpose_tile_8,
                        );
                        tile_scatter_direct_plane(
                            &self.scratch.right,
                            pairs,
                            frames_used,
                            true,
                            effect_contract::transpose_tile_8,
                        );
                    }
                }
                return;
            }
        }
        // Issue #915: a bank whose every lane folds is offered its resident block whole, before
        // anything is written. The test is every lane of `0..W`, not `all_active_folded` below:
        // `full_bank` makes the two agree today, but only this form guarantees the cohort is
        // exactly lanes `0..W` in order -- the lanes, in the order, the staged `FoldCohort` below
        // would name -- and that no lane is left needing a `plane_mut` copy. An accepted offer
        // finishes the fold; the staging block and every plane stay unwritten. A declined one
        // wrote nothing, and the staged path below runs as it always has.
        if self.fold.len() == W
            && self.fold.iter().all(|folded| *folded)
            && let Ok(cohort) = ResidentFoldCohort::new(
                &self.scratch.left[..frames_used * W],
                &self.scratch.right[..frames_used * W],
                self.scratch.width,
                frames_used,
            )
            && members.fold_resident(cohort)
        {
            return;
        }
        let tiled = (frames_used / W) * W;
        tile_scatter(
            &self.scratch.left[..tiled * W],
            &mut self.staging_left,
            stride,
            transpose,
        );
        tile_scatter(
            &self.scratch.right[..tiled * W],
            &mut self.staging_right,
            stride,
            transpose,
        );
        // The ragged tail, word by word, straight into the same staging block: one contiguous copy
        // per lane below then still covers the whole block.
        for frame in tiled..frames_used {
            for lane in 0..W {
                self.staging_left[lane * stride + frame] = self.scratch.left[frame * W + lane];
                self.staging_right[lane * stride + frame] = self.scratch.right[frame * W + lane];
            }
        }
        if self.fold.is_empty() {
            for lane in 0..W {
                let (left, right) = members.plane_mut(lane);
                debug_assert!(left.len() == frames_used && right.len() == frames_used);
                left.copy_from_slice(
                    &self.staging_left[lane * stride..lane * stride + frames_used],
                );
                right.copy_from_slice(
                    &self.staging_right[lane * stride..lane * stride + frames_used],
                );
            }
            return;
        }
        let all_active_folded = (0..W).any(|lane| self.active[lane])
            && (0..W).all(|lane| !self.active[lane] || self.fold[lane]);
        let Self {
            staging_left,
            staging_right,
            fold,
            ..
        } = self;
        let mut folded_lanes = [0usize; 8];
        let mut folded_count = 0;
        for lane in 0..W {
            let left = &mut staging_left[lane * stride..lane * stride + frames_used];
            let right = &mut staging_right[lane * stride..lane * stride + frames_used];
            if fold[lane] {
                if all_active_folded {
                    folded_lanes[folded_count] = lane;
                    folded_count += 1;
                } else {
                    members.fold_plane(lane, left, right);
                }
            } else {
                let (plane_left, plane_right) = members.plane_mut(lane);
                debug_assert!(plane_left.len() == frames_used && plane_right.len() == frames_used);
                plane_left.copy_from_slice(left);
                plane_right.copy_from_slice(right);
            }
        }
        if folded_count != 0
            && let Ok(cohort) = FoldCohort::new(
                &folded_lanes[..folded_count],
                staging_left,
                staging_right,
                stride,
                frames_used,
            )
        {
            members.fold_cohort(cohort);
        }
    }
    // REALTIME_POLICY_END

    /// Take the per-lane scalar path even on a full bank.
    ///
    /// Test-only, and the only way to reach the scalar transpose with an all-active mask: it is
    /// what lets `tiled_transpose_matches_the_scalar_path_bit_for_bit` run identical input through
    /// both implementations and compare words, instead of comparing the tiled path to itself.
    #[cfg(test)]
    fn force_scalar_transpose(&mut self) {
        self.full_bank = false;
    }

    #[must_use]
    pub const fn width(&self) -> BankWidth {
        self.scratch.width
    }
    #[must_use]
    pub const fn quantum(&self) -> u32 {
        self.scratch.quantum
    }
    #[must_use]
    pub fn active(&self) -> &[bool] {
        &self.active
    }

    // REALTIME_POLICY_BEGIN
    /// The lanes of this chain at exact rest for the coming block, bit `l` for lane `l`, within
    /// the chain's lanes (issue #1781; the consumer is #1107's whole-bank skip): the AND of
    /// [`BankStage::lanes_at_rest`] over every slot that has an active lane. Asked after this
    /// block's drains (`begin_block` on every slot), as each stage's answer is.
    ///
    /// A slot with no active lane never runs, so it contributes every lane. A slot active on some
    /// lanes still runs over the whole resident block, so it answers its inactive (identity)
    /// lanes from its own state, as it answers a padded lane: such a lane is fed `+0.0` by the
    /// chain whenever the chain is skipped, and the stage's predicate holds or clears its bit on
    /// what the stage would do to it. Allocation-free; one call per slot.
    #[must_use]
    pub fn lanes_at_rest(&self) -> u32 {
        self.slots
            .iter()
            .filter(|slot| slot.has_active_lanes())
            .fold(self.all_lanes_mask(), |rest, slot| {
                rest & slot.stage.lanes_at_rest()
            })
    }

    /// Every lane of the bank, padded lanes included: bit `l` for lane `l`.
    const fn all_lanes_mask(&self) -> u32 {
        if self.lanes >= 32 {
            u32::MAX
        } else {
            (1_u32 << self.lanes) - 1
        }
    }
    // REALTIME_POLICY_END
    /// Completed planar/AoSoA round-trips: exactly one per rendered block per chain. A block the
    /// chain skipped whole moves no word and is not one ([`Self::silent_skips`] counts it).
    #[must_use]
    pub const fn transposes(&self) -> u64 {
        self.transposes
    }

    /// Blocks this chain skipped whole because every lane's input was exact `+0.0` and every
    /// lane was at rest (issue #1107). Evidence only.
    #[must_use]
    pub const fn silent_skips(&self) -> u64 {
        self.silent_skips
    }

    /// Allow (`true`, the default) or forbid the whole-bank silent skip (issue #1107).
    ///
    /// For a test-support differential only: the graph's test switch forbids it to render the
    /// no-skip twin of a plan. Shipped builds never call it, so every shipped chain may skip.
    pub const fn set_silent_skip(&mut self, allowed: bool) {
        self.silent_skip = allowed;
    }
    #[must_use]
    pub fn qualification_counters(&self) -> [u64; 2] {
        self.slots.iter().fold([0, 0], |mut total, slot| {
            let counters = slot.stage.qualification_counters();
            total[0] = total[0].saturating_add(counters[0]);
            total[1] = total[1].saturating_add(counters[1]);
            total
        })
    }

    /// `[observed lanes, declared taps, armed taps]` across every slot (issue #143 E5).
    #[must_use]
    pub fn observation_binding_counts(&self) -> [u64; 3] {
        self.slots.iter().fold([0, 0, 0], |mut total, slot| {
            let counts = slot.stage.observation_binding_counts();
            for (value, add) in total.iter_mut().zip(counts) {
                *value = value.saturating_add(add);
            }
            total
        })
    }

    /// Exact engine-owned observation bytes every slot of this chain retains.
    #[must_use]
    pub fn observation_retained_bytes(&self) -> usize {
        self.slots
            .iter()
            .map(|slot| slot.stage.observation_retained_bytes())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The widest bank this build has: eight lanes on an 8-lane (AVX2) build, four on a 4-lane
    /// (NEON/simd128) one (`BankWidth::ALL`, issue #1112). A test whose claim holds at any one
    /// width runs here, so every build runs it at the width it ships.
    const WIDEST: BankWidth = BankWidth::ALL[BankWidth::ALL.len() - 1];

    #[test]
    fn prepared_fold_configuration_preserves_masks_and_constructor_checks() {
        for &width in BankWidth::ALL {
            let lanes = width.lanes() as usize;
            assert!(matches!(
                PreparedFoldConfiguration::new(
                    width,
                    vec![true; lanes - 1].into_boxed_slice(),
                    vec![false; lanes].into_boxed_slice()
                ),
                Err(RackError::Shape)
            ));
            assert!(matches!(
                PreparedFoldConfiguration::new(
                    width,
                    vec![true; lanes].into_boxed_slice(),
                    vec![false; lanes - 1].into_boxed_slice()
                ),
                Err(RackError::Shape)
            ));
            assert!(matches!(
                PreparedFoldConfiguration::new(
                    width,
                    vec![false; lanes].into_boxed_slice(),
                    vec![false; lanes].into_boxed_slice()
                ),
                Err(RackError::Shape)
            ));
            let mut active = vec![true; lanes];
            active[lanes - 1] = false;
            assert!(matches!(
                PreparedFoldConfiguration::new(
                    width,
                    active.clone().into_boxed_slice(),
                    vec![true; lanes].into_boxed_slice()
                ),
                Err(RackError::Shape)
            ));
            for fold in [vec![false; lanes], active.clone()] {
                let configuration = PreparedFoldConfiguration::new(
                    width,
                    active.clone().into_boxed_slice(),
                    fold.clone().into_boxed_slice(),
                )
                .expect("configuration");
                let chain = BankChain::new_with_prepared_fold(
                    AoSoaScratch::new(width, 16).expect("scratch"),
                    configuration,
                    Vec::new(),
                )
                .expect("chain");
                assert_eq!(&*chain.active, active);
                assert_eq!(
                    chain.fold_lanes(),
                    if fold.iter().any(|lane| *lane) {
                        &fold[..]
                    } else {
                        &[]
                    }
                );
                if fold.iter().any(|lane| *lane) {
                    assert_eq!(chain.staging_left.len(), chain.scratch.left.len());
                    assert_eq!(chain.staging_right.len(), chain.scratch.right.len());
                }
            }
            let configuration = PreparedFoldConfiguration::new(
                width,
                active.clone().into_boxed_slice(),
                active.into_boxed_slice(),
            )
            .expect("configuration");
            // Another width's scratch is refused; only an 8-lane (AVX2) build has another width.
            if let Some(&other) = BankWidth::ALL.iter().find(|&&other| other != width) {
                assert!(matches!(
                    BankChain::new_with_prepared_fold(
                        AoSoaScratch::new(other, 16).expect("scratch"),
                        configuration,
                        Vec::new()
                    ),
                    Err(RackError::WidthMismatch)
                ));
            }
        }
    }

    #[test]
    fn resident_output_lane_checks_shape_and_preserves_final_planes_after_collapse() {
        for &width in BankWidth::ALL {
            let lanes = width.lanes() as usize;
            for population in [1, lanes - 1, lanes] {
                let active: Vec<_> = (0..lanes).map(|lane| lane < population).collect();
                for suffix in [false, true] {
                    let mut slots =
                        vec![slot(active.clone(), Box::new(Scale::new([0.5; 2], true)))];
                    if suffix {
                        slots.push(slot(
                            active.clone(),
                            Box::new(Matrix {
                                coefficients: [0.75, 0.125, -0.25, 0.5],
                            }),
                        ));
                    }
                    let mut chain = BankChain::new(
                        AoSoaScratch::new(width, 17).unwrap(),
                        active.clone().into_boxed_slice(),
                        slots,
                    )
                    .unwrap();
                    chain.arm_mono_collapse(true);
                    // Engage, forced decline, and recovery all expose the final right plane.
                    for (block, forced) in [false, true, false].into_iter().enumerate() {
                        chain.force_mono_collapse_off(forced);
                        chain.scratch.left.fill(f32::from_bits(0x7fc0_0714));
                        chain.scratch.right.fill(f32::from_bits(0xffc0_0714));
                        let frames = 9;
                        let mut planes = Planes {
                            left: (0..lanes)
                                .map(|lane| vec![(lane + 1) as f32; frames as usize])
                                .collect(),
                            right: (0..lanes)
                                .map(|lane| vec![(lane + 1) as f32; frames as usize])
                                .collect(),
                        };
                        chain
                            .run(&mut planes, frames, block as u64 * 9, LiveSnapshot::ALL)
                            .unwrap();
                        assert_eq!(chain.is_collapsed(), !forced);
                        for lane in 0..lanes {
                            let view = chain.final_output_lane(frames, lane);
                            if lane >= population {
                                assert!(view.is_none());
                                continue;
                            }
                            let view = view.unwrap();
                            assert_eq!(
                                (view.frames(), view.width(), view.lane()),
                                (frames, width, lane)
                            );
                            assert_eq!(view.left().len(), frames as usize * lanes);
                            assert_eq!(view.right().len(), frames as usize * lanes);
                            for frame in 0..frames as usize {
                                assert_eq!(
                                    view.left()[frame * lanes + lane].to_bits(),
                                    planes.left[lane][frame].to_bits()
                                );
                                assert_eq!(
                                    view.right()[frame * lanes + lane].to_bits(),
                                    planes.right[lane][frame].to_bits()
                                );
                            }
                            if suffix {
                                assert_ne!(view.left()[lane], view.right()[lane]);
                            }
                        }
                        assert!(chain.final_output_lane(0, 0).is_none());
                        assert!(chain.final_output_lane(18, 0).is_none());
                        assert!(chain.final_output_lane(frames, lanes).is_none());
                    }
                    chain.scratch.right = Box::new([]);
                    assert!(chain.final_output_lane(1, 0).is_none());
                    chain.active = Box::new([]);
                    assert!(chain.final_output_lane(1, 0).is_none());
                }
            }
        }
    }

    #[test]
    fn rt9_resident_copy_matches_scalar_gather_and_preserves_poisoned_words() {
        for &width in BankWidth::ALL {
            let lanes = width.lanes() as usize;
            let quantum = 17;
            for partial in [false, true] {
                let active: Vec<_> = (0..lanes).map(|lane| !partial || lane % 3 != 1).collect();
                let make = || {
                    BankChain::new(
                        AoSoaScratch::new(width, quantum).expect("scratch"),
                        active.clone().into_boxed_slice(),
                        vec![slot(active.clone(), Box::new(PassThrough))],
                    )
                    .expect("chain")
                };
                for frames in [1, lanes as u32 - 1, lanes as u32, lanes as u32 + 1, quantum] {
                    let mut seed = 713;
                    let mut planes = Planes {
                        left: (0..lanes)
                            .map(|_| (0..frames).map(|_| seeded(&mut seed)).collect())
                            .collect(),
                        right: (0..lanes)
                            .map(|_| (0..frames).map(|_| seeded(&mut seed)).collect())
                            .collect(),
                    };
                    planes.left[0][0] = -0.0;
                    planes.right[0][0] = f32::from_bits(0x7fc0_0713);
                    let mut source = make();
                    source
                        .run(&mut planes, frames, 0, LiveSnapshot::ALL)
                        .expect("producer");
                    for mono in [false, true] {
                        let mut resident = make();
                        let mut reference = make();
                        for chain in [&mut resident, &mut reference] {
                            chain.scratch.left.fill(f32::from_bits(0x7fc0_aaaa));
                            chain.scratch.right.fill(f32::from_bits(0xffc0_bbbb));
                        }
                        reference.force_scalar_transpose();
                        if mono {
                            reference.gather_mono(&planes, frames);
                        } else {
                            reference.gather(&planes, frames);
                        }
                        resident.acquire_resident_input(&source, frames, mono);
                        for (actual, expected) in resident
                            .scratch
                            .left
                            .iter()
                            .chain(resident.scratch.right.iter())
                            .zip(
                                reference
                                    .scratch
                                    .left
                                    .iter()
                                    .chain(reference.scratch.right.iter()),
                            )
                        {
                            assert_eq!(
                                actual.to_bits(),
                                expected.to_bits(),
                                "{width:?} partial={partial} frames={frames} mono={mono}"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn rt9_resident_run_and_local_declines_preserve_drains_errors_and_collapse_order() {
        struct Reads {
            planes: Planes,
            calls: std::cell::Cell<usize>,
        }
        impl BankMembers for Reads {
            fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
                self.calls.set(self.calls.get() + 1);
                self.planes.plane(lane)
            }
            fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
                self.planes.plane_mut(lane)
            }
        }
        for mono in [false, true] {
            for failure in [
                None,
                Some(TraceKind::Begin),
                Some(if mono {
                    TraceKind::Mono
                } else {
                    TraceKind::Dual
                }),
            ] {
                // Mismatch 1, a source of another width, needs a build with two widths (#1112).
                for mismatch in (0..5).filter(|&mismatch| mismatch != 1 || BankWidth::ALL.len() > 1)
                {
                    let spec = TraceSpec {
                        id: 0,
                        mask: [true; 4],
                        seam: SeamSide::UpstreamOfSeam,
                        mono_capable: true,
                        fail: failure,
                        delta: 0.25,
                    };
                    let render = |resident: bool| {
                        TEST_TRACE.with(|trace| {
                            *trace.borrow_mut() = TraceObservation::with_queues(&[spec])
                        });
                        let mut source = BankChain::new(
                            AoSoaScratch::new(
                                if mismatch == 1 {
                                    WIDEST
                                } else {
                                    BankWidth::Four
                                },
                                if mismatch == 2 { 4 } else { 3 },
                            )
                            .expect("scratch"),
                            vec![
                                true;
                                if mismatch == 1 {
                                    WIDEST.lanes() as usize
                                } else {
                                    4
                                }
                            ]
                            .into_boxed_slice(),
                            vec![],
                        )
                        .expect("source");
                        if mismatch == 3 {
                            source.active[3] = false;
                        }
                        if mismatch == 4 {
                            source
                                .arm_fold(vec![true; 4].into_boxed_slice())
                                .expect("fold");
                        }
                        let mut inputs = trace_planes();
                        if mono {
                            inputs.right.clone_from(&inputs.left);
                        }
                        // Prepare identical resident and planar inputs without running the epilogue
                        // deliberately used to exercise the local decline.
                        if mismatch == 0 {
                            source.gather(&inputs, 3);
                        }
                        let mut chain = BankChain::new(
                            AoSoaScratch::new(BankWidth::Four, 3).expect("scratch"),
                            vec![true; 4].into_boxed_slice(),
                            vec![trace_slot(spec)],
                        )
                        .expect("successor");
                        chain.arm_mono_collapse(mono);
                        let mut reads = Reads {
                            planes: inputs,
                            calls: std::cell::Cell::new(0),
                        };
                        let mut results = Vec::new();
                        for block in 0..2 {
                            // Both arms receive the same predecessor output on each block.
                            if mismatch == 0 {
                                source.gather(&reads.planes, 3);
                            }
                            let result = if resident {
                                chain.run_with_resident_input(
                                    &source,
                                    &mut reads,
                                    3,
                                    block * 3,
                                    LiveSnapshot::ALL,
                                )
                            } else {
                                chain.run(&mut reads, 3, block * 3, LiveSnapshot::ALL)
                            };
                            results.push(result);
                        }
                        let counters = (
                            chain.transposes(),
                            chain.collapses(),
                            chain.collapse_transitions(),
                        );
                        let trace = TEST_TRACE.with(|trace| trace.borrow().clone());
                        (reads.planes, reads.calls.get(), results, counters, trace)
                    };
                    let old = render(false);
                    let new = render(true);
                    assert_planes_bit_equal(&new.0, &old.0, "resident run");
                    assert_eq!((&new.2, &new.3, &new.4), (&old.2, &old.3, &old.4));
                    if mismatch == 0 {
                        assert_eq!(new.1, 0, "resident acquisition must never gather");
                    } else {
                        assert_eq!(new.1, old.1, "decline gathers once, after identical drains");
                    }
                }
            }
        }
    }

    struct PassThrough;
    impl BankStage for PassThrough {
        fn process(&mut self, _block: BankBlock<'_>) -> Result<(), RenderError> {
            Ok(())
        }
    }

    struct DropWitness;
    impl Drop for DropWitness {
        fn drop(&mut self) {
            TEST_PREPARED_DROPS.with(|count| count.set(count.get() + 1));
        }
    }
    impl BankStage for DropWitness {
        fn process(&mut self, _block: BankBlock<'_>) -> Result<(), RenderError> {
            Ok(())
        }
    }

    #[derive(Clone)]
    struct Planes {
        left: Vec<Vec<f32>>,
        right: Vec<Vec<f32>>,
    }
    impl BankMembers for Planes {
        fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
            (&self.left[lane], &self.right[lane])
        }
        fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
            (&mut self.left[lane], &mut self.right[lane])
        }
        fn distinct_planes_mut(
            &mut self,
            lanes: usize,
            frames: usize,
        ) -> Option<BankPlaneViews<'_>> {
            if lanes != self.left.len()
                || lanes != self.right.len()
                || (lanes != 4 && lanes != 8)
                || self.left.iter().any(|p| p.len() < frames)
                || self.right.iter().any(|p| p.len() < frames)
            {
                return None;
            }
            let (left, right) = (&mut self.left, &mut self.right);
            match lanes {
                4 => {
                    let mut left = left.iter_mut();
                    let mut right = right.iter_mut();
                    let pairs: [BankPlanePair<'_>; 4] = std::array::from_fn(|_| {
                        (
                            &mut **left.next().expect("validated lane"),
                            &mut **right.next().expect("validated lane"),
                        )
                    });
                    BankPlaneViews::from_four(pairs, frames)
                }
                // `from_eight` exists where eight-lane banks do (issue #1112).
                #[cfg(target_feature = "avx2")]
                8 => {
                    let mut left = left.iter_mut();
                    let mut right = right.iter_mut();
                    let pairs: [BankPlanePair<'_>; 8] = std::array::from_fn(|_| {
                        (
                            &mut **left.next().expect("validated lane"),
                            &mut **right.next().expect("validated lane"),
                        )
                    });
                    BankPlaneViews::from_eight(pairs, frames)
                }
                _ => None,
            }
        }
    }

    struct StagedPlanes(Planes);
    impl BankMembers for StagedPlanes {
        fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
            self.0.plane(lane)
        }
        fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
            self.0.plane_mut(lane)
        }
    }

    struct ShortClaimPlanes {
        planes: Planes,
        per_lane_writes: usize,
    }
    impl BankMembers for ShortClaimPlanes {
        fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
            self.planes.plane(lane)
        }
        fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
            self.per_lane_writes += 1;
            self.planes.plane_mut(lane)
        }
        fn distinct_planes_mut(
            &mut self,
            lanes: usize,
            frames: usize,
        ) -> Option<BankPlaneViews<'_>> {
            assert_eq!(lanes, 4);
            let mut left = self.planes.left.iter_mut();
            let mut right = self.planes.right.iter_mut();
            let pairs: [BankPlanePair<'_>; 4] = core::array::from_fn(|_| {
                (
                    left.next().expect("four left lanes").as_mut_slice(),
                    right.next().expect("four right lanes").as_mut_slice(),
                )
            });
            BankPlaneViews::from_four(pairs, frames.saturating_sub(1))
        }
    }

    struct CountedDirectPlanes {
        planes: Planes,
        per_lane_writes: usize,
    }
    impl BankMembers for CountedDirectPlanes {
        fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
            self.planes.plane(lane)
        }
        fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
            self.per_lane_writes += 1;
            self.planes.plane_mut(lane)
        }
        fn distinct_planes_mut(
            &mut self,
            lanes: usize,
            frames: usize,
        ) -> Option<BankPlaneViews<'_>> {
            if lanes != 4
                || self.planes.left.len() != 4
                || self.planes.right.len() != 4
                || self.planes.left.iter().any(|plane| plane.len() < frames)
                || self.planes.right.iter().any(|plane| plane.len() < frames)
            {
                return None;
            }
            let mut left = self.planes.left.iter_mut();
            let mut right = self.planes.right.iter_mut();
            let pairs: [BankPlanePair<'_>; 4] = core::array::from_fn(|_| {
                (
                    left.next().expect("validated left lane").as_mut_slice(),
                    right.next().expect("validated right lane").as_mut_slice(),
                )
            });
            BankPlaneViews::from_four(pairs, frames)
        }
    }

    fn seeded(state: &mut u64) -> f32 {
        *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = *state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        f32::from_bits((z ^ (z >> 31)) as u32)
    }

    fn slot(active_lanes: Vec<bool>, stage: Box<dyn BankStage>) -> BankSlot {
        BankSlot {
            stage,
            active_lanes: active_lanes.into_boxed_slice(),
        }
    }

    const TRACE_STAGE_CAPACITY: usize = 16;
    const TRACE_EVENT_CAPACITY: usize = 64;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum TraceKind {
        Begin,
        Dual,
        Mono,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct TraceEvent {
        kind: TraceKind,
        stage: u8,
        first_sample: u64,
        frames: u32,
        queue_before: u8,
        arithmetic_before: u8,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TraceObservation {
        events: [Option<TraceEvent>; TRACE_EVENT_CAPACITY],
        event_count: usize,
        queues: [u8; TRACE_STAGE_CAPACITY],
        arithmetic: [u8; TRACE_STAGE_CAPACITY],
        drops: [u8; TRACE_STAGE_CAPACITY],
    }

    impl TraceObservation {
        const EMPTY: Self = Self {
            events: [None; TRACE_EVENT_CAPACITY],
            event_count: 0,
            queues: [0; TRACE_STAGE_CAPACITY],
            arithmetic: [0; TRACE_STAGE_CAPACITY],
            drops: [0; TRACE_STAGE_CAPACITY],
        };

        fn with_queues(specs: &[TraceSpec]) -> Self {
            let mut observation = Self::EMPTY;
            for spec in specs {
                observation.queues[usize::from(spec.id)] = 2;
            }
            observation
        }

        fn record(&mut self, kind: TraceKind, stage: u8, first_sample: u64, frames: u32) {
            let index = usize::from(stage);
            let event = TraceEvent {
                kind,
                stage,
                first_sample,
                frames,
                queue_before: self.queues[index],
                arithmetic_before: self.arithmetic[index],
            };
            self.events[self.event_count] = Some(event);
            self.event_count += 1;
        }

        fn ordered_events(&self) -> &[Option<TraceEvent>] {
            &self.events[..self.event_count]
        }
    }

    thread_local! {
        static TEST_TRACE: std::cell::RefCell<TraceObservation> =
            const { std::cell::RefCell::new(TraceObservation::EMPTY) };
    }

    #[derive(Clone, Copy)]
    struct TraceSpec {
        id: u8,
        mask: [bool; 4],
        seam: SeamSide,
        mono_capable: bool,
        fail: Option<TraceKind>,
        delta: f32,
    }

    struct PreparedTraceStage(TraceSpec);

    impl Drop for PreparedTraceStage {
        fn drop(&mut self) {
            TEST_TRACE.with(|trace| {
                let mut trace = trace.borrow_mut();
                let drops = &mut trace.drops[usize::from(self.0.id)];
                *drops += 1;
            });
        }
    }

    fn trace_arithmetic(block: BankBlock<'_>, delta: f32, mono: bool) {
        for sample in block.left.iter_mut() {
            *sample += delta;
        }
        if !mono {
            for sample in block.right.iter_mut() {
                *sample += delta;
            }
        }
    }

    impl BankStage for PreparedTraceStage {
        fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
            TEST_TRACE.with(|trace| {
                trace.borrow_mut().record(
                    TraceKind::Dual,
                    self.0.id,
                    block.first_sample,
                    block.frames,
                );
            });
            if self.0.fail == Some(TraceKind::Dual) {
                return Err(RenderError::InvalidEnvelope);
            }
            trace_arithmetic(block, self.0.delta, false);
            TEST_TRACE.with(|trace| {
                trace.borrow_mut().arithmetic[usize::from(self.0.id)] += 1;
            });
            Ok(())
        }

        fn begin_block(&mut self, first_sample: u64, _: LiveSnapshot) -> Result<(), RenderError> {
            TEST_TRACE.with(|trace| {
                let mut trace = trace.borrow_mut();
                trace.record(TraceKind::Begin, self.0.id, first_sample, 0);
                let queued = &mut trace.queues[usize::from(self.0.id)];
                *queued = queued.saturating_sub(1);
            });
            if self.0.fail == Some(TraceKind::Begin) {
                Err(RenderError::InvalidEnvelope)
            } else {
                Ok(())
            }
        }

        fn lane_symmetry(&self, _lane: usize) -> ChannelSymmetryWitness {
            ChannelSymmetryWitness::SYMMETRIC
        }

        fn seam_side(&self) -> SeamSide {
            self.0.seam
        }

        fn supports_mono_collapse(&self) -> bool {
            self.0.mono_capable
        }

        fn process_mono(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
            TEST_TRACE.with(|trace| {
                trace.borrow_mut().record(
                    TraceKind::Mono,
                    self.0.id,
                    block.first_sample,
                    block.frames,
                );
            });
            if self.0.fail == Some(TraceKind::Mono) {
                return Err(RenderError::InvalidEnvelope);
            }
            trace_arithmetic(block, self.0.delta, true);
            TEST_TRACE.with(|trace| {
                trace.borrow_mut().arithmetic[usize::from(self.0.id)] += 1;
            });
            Ok(())
        }

        fn channels_agree(&self) -> bool {
            true
        }
    }

    fn trace_slot(spec: TraceSpec) -> BankSlot {
        slot(spec.mask.to_vec(), Box::new(PreparedTraceStage(spec)))
    }

    fn trace_planes() -> Planes {
        Planes {
            left: (0..4)
                .map(|lane| (0..3).map(|frame| (lane * 8 + frame) as f32).collect())
                .collect(),
            right: (0..4)
                .map(|lane| (0..3).map(|frame| (64 + lane * 8 + frame) as f32).collect())
                .collect(),
        }
    }

    fn trace_plane_bits(planes: &Planes) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
        (
            planes
                .left
                .iter()
                .map(|plane| plane.iter().map(|sample| sample.to_bits()).collect())
                .collect(),
            planes
                .right
                .iter()
                .map(|plane| plane.iter().map(|sample| sample.to_bits()).collect())
                .collect(),
        )
    }

    /// Independent planar execution of the current drain, error, mono-prefix and seam laws.
    fn reference_dispatch(
        specs: &[TraceSpec],
        collapse_prefix: Option<usize>,
        planes: &mut Planes,
        frames: u32,
        first_sample: u64,
    ) -> (Result<(), RenderError>, TraceObservation) {
        let mut trace = TraceObservation::with_queues(specs);
        for spec in specs {
            if !spec.mask.iter().any(|active| *active) {
                continue;
            }
            trace.record(TraceKind::Begin, spec.id, first_sample, 0);
            let queued = &mut trace.queues[usize::from(spec.id)];
            *queued = queued.saturating_sub(1);
            if spec.fail == Some(TraceKind::Begin) {
                return (Err(RenderError::InvalidEnvelope), trace);
            }
        }

        // BankChain gathers into resident scratch and scatters only after every stage succeeds.
        // Keep that error boundary in the reference rather than incrementally publishing PCM.
        let mut scratch_planes = planes.clone();

        let process_one = |spec: &TraceSpec,
                           mono: bool,
                           planes: &mut Planes,
                           trace: &mut TraceObservation|
         -> Result<(), RenderError> {
            if !spec.mask.iter().any(|active| *active) {
                return Ok(());
            }
            let kind = if mono {
                TraceKind::Mono
            } else {
                TraceKind::Dual
            };
            trace.record(kind, spec.id, first_sample, frames);
            if spec.fail == Some(kind) {
                return Err(RenderError::InvalidEnvelope);
            }
            for plane in &mut planes.left {
                for sample in &mut plane[..frames as usize] {
                    *sample += spec.delta;
                }
            }
            if !mono {
                for plane in &mut planes.right {
                    for sample in &mut plane[..frames as usize] {
                        *sample += spec.delta;
                    }
                }
            }
            trace.arithmetic[usize::from(spec.id)] += 1;
            Ok(())
        };

        if let Some(prefix) = collapse_prefix {
            for spec in &specs[..prefix] {
                if let Err(error) = process_one(spec, true, &mut scratch_planes, &mut trace) {
                    return (Err(error), trace);
                }
            }
            for lane in 0..scratch_planes.left.len() {
                scratch_planes.right[lane][..frames as usize]
                    .copy_from_slice(&scratch_planes.left[lane][..frames as usize]);
            }
            for spec in &specs[prefix..] {
                if let Err(error) = process_one(spec, false, &mut scratch_planes, &mut trace) {
                    return (Err(error), trace);
                }
            }
        } else {
            for spec in specs {
                if let Err(error) = process_one(spec, false, &mut scratch_planes, &mut trace) {
                    return (Err(error), trace);
                }
            }
        }
        *planes = scratch_planes;
        (Ok(()), trace)
    }

    fn exercise_trace_case(specs: &[TraceSpec], collapse_prefix: Option<usize>, first_sample: u64) {
        let mut expected_planes = trace_planes();
        let mut actual_planes = trace_planes();
        let (expected_result, expected_trace) = reference_dispatch(
            specs,
            collapse_prefix,
            &mut expected_planes,
            3,
            first_sample,
        );
        TEST_TRACE.with(|trace| *trace.borrow_mut() = TraceObservation::with_queues(specs));
        let mut chain = BankChain::new(
            AoSoaScratch::new(BankWidth::Four, 3).expect("scratch"),
            vec![true; 4].into_boxed_slice(),
            specs.iter().copied().map(trace_slot).collect(),
        )
        .expect("trace chain");
        if collapse_prefix.is_some() {
            chain.arm_mono_collapse(true);
        } else {
            chain.force_mono_collapse_off(true);
        }
        let actual_result = chain.run(&mut actual_planes, 3, first_sample, LiveSnapshot::ALL);
        assert_eq!(actual_result, expected_result);
        let actual_trace = TEST_TRACE.with(|trace| trace.borrow().clone());
        assert_eq!(
            actual_trace.ordered_events(),
            expected_trace.ordered_events()
        );
        assert_eq!(actual_trace.queues, expected_trace.queues);
        assert_eq!(actual_trace.arithmetic, expected_trace.arithmetic);
        assert_eq!(
            trace_plane_bits(&actual_planes),
            trace_plane_bits(&expected_planes)
        );
        assert!(actual_trace.drops.iter().all(|drops| *drops == 0));
        drop(chain);
        let dropped = TEST_TRACE.with(|trace| trace.borrow().drops);
        for spec in specs {
            assert_eq!(dropped[usize::from(spec.id)], 1);
        }
    }

    #[test]
    fn prepared_slot_activity_preserves_shape_bits_and_ownership() {
        for &width in BankWidth::ALL {
            let lanes = width.lanes() as usize;
            let scratch = || AoSoaScratch::new(width, 4).expect("scratch");
            assert_eq!(
                BankChain::new(
                    scratch(),
                    vec![true; lanes].into_boxed_slice(),
                    vec![slot(vec![true; lanes - 1], Box::new(PassThrough))],
                )
                .err(),
                Some(RackError::Shape)
            );
            assert_eq!(
                BankChain::new(
                    scratch(),
                    (0..lanes)
                        .map(|lane| lane != lanes - 1)
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                    vec![slot(
                        (0..lanes).map(|lane| lane == lanes - 1).collect(),
                        Box::new(PassThrough),
                    )],
                )
                .err(),
                Some(RackError::Shape),
                "shape validation precedes packed conversion"
            );

            let partial: Vec<bool> = (0..lanes).map(|lane| lane.is_multiple_of(2)).collect();
            let zero = Vec::with_capacity(4);
            assert!(zero.capacity() > zero.len(), "spare caller capacity");
            let mut zero_chain =
                BankChain::new(scratch(), partial.clone().into_boxed_slice(), zero)
                    .expect("zero-slot chain remains legal");
            assert!(zero_chain.slots.is_empty());
            assert_eq!(zero_chain.collapse_prefix(), 0);
            let mut identity = Planes {
                left: (0..lanes).map(|lane| vec![lane as f32 + 1.0; 4]).collect(),
                right: (0..lanes).map(|lane| vec![lane as f32 + 17.0; 4]).collect(),
            };
            let identity_before = trace_plane_bits(&identity);
            zero_chain
                .run(&mut identity, 4, 17, LiveSnapshot::ALL)
                .expect("zero-slot identity");
            assert_eq!(trace_plane_bits(&identity), identity_before);
            drop(zero_chain);

            let changed_lane = if lanes == 4 { 2 } else { 4 };
            let one_mask: Vec<bool> = (0..lanes).map(|lane| lane == changed_lane).collect();
            let one_expected = if lanes == 4 { 0b0100 } else { 0b0001_0000 };
            let mut one = Vec::with_capacity(5);
            one.push(slot(vec![false; lanes], Box::new(PassThrough)));
            one[0].active_lanes[changed_lane] = true;
            assert_eq!(one[0].active_lanes.as_ref(), one_mask.as_slice());
            assert!(one.capacity() > one.len(), "spare caller capacity");
            let one_chain = BankChain::new(scratch(), partial.clone().into_boxed_slice(), one)
                .expect("one-bit slot");
            assert_eq!(one_chain.slots[0].active_lanes, one_expected);
            drop(one_chain);

            let three_masks = [vec![false; lanes], partial.clone(), vec![true; lanes]];
            let three_expected = if lanes == 4 {
                [0b0000, 0b0101, 0b1111]
            } else {
                [0x00, 0x55, 0xff]
            };
            let before_drop = TEST_PREPARED_DROPS.with(std::cell::Cell::get);
            let mut three = Vec::with_capacity(7);
            for (index, mask) in three_masks.into_iter().enumerate() {
                let stage: Box<dyn BankStage> = if index == 0 {
                    Box::new(DropWitness)
                } else {
                    Box::new(PassThrough)
                };
                three.push(slot(mask, stage));
            }
            assert!(three.capacity() > three.len(), "spare caller capacity");
            let mut three_chain =
                BankChain::new(scratch(), vec![true; lanes].into_boxed_slice(), three)
                    .expect("empty, holey and full slots");
            assert_eq!(
                three_chain
                    .slots
                    .iter()
                    .map(|slot| slot.active_lanes)
                    .collect::<Vec<_>>(),
                three_expected
            );
            let mut rendered = Planes {
                left: (0..lanes).map(|_| vec![3.0; 4]).collect(),
                right: (0..lanes).map(|_| vec![-7.0; 4]).collect(),
            };
            three_chain
                .run(&mut rendered, 4, 23, LiveSnapshot::ALL)
                .expect("shape render");
            assert_eq!(TEST_PREPARED_DROPS.with(std::cell::Cell::get), before_drop);
            drop(three_chain);
            assert_eq!(
                TEST_PREPARED_DROPS.with(std::cell::Cell::get),
                before_drop + 1
            );

            let nine_expected: &[u8] = if lanes == 4 {
                &[0, 1, 4, 5, 0, 1, 4, 5, 0]
            } else {
                &[0, 1, 4, 16, 64, 5, 80, 17, 85]
            };
            let mut nine = Vec::with_capacity(13);
            nine.extend(nine_expected.iter().map(|mask| {
                let active = (0..lanes).map(|lane| mask & (1 << lane) != 0).collect();
                slot(active, Box::new(PassThrough))
            }));
            assert!(nine.capacity() > nine.len(), "spare caller capacity");
            let nine_chain = BankChain::new(scratch(), partial.into_boxed_slice(), nine)
                .expect("slot count exceeds lane width");
            assert_eq!(nine_chain.slots.len(), 9);
            assert_eq!(
                nine_chain
                    .slots
                    .iter()
                    .map(|slot| slot.active_lanes)
                    .collect::<Vec<_>>(),
                nine_expected
            );
            assert_eq!(
                nine_chain.lane_symmetry(lanes),
                ChannelSymmetryWitness::DECLINED
            );
        }

        let active = [true; 4];
        let collapsible = [
            TraceSpec {
                id: 0,
                mask: active,
                seam: SeamSide::UpstreamOfSeam,
                mono_capable: true,
                fail: None,
                delta: 1.0,
            },
            TraceSpec {
                id: 1,
                mask: active,
                seam: SeamSide::SeamSide,
                mono_capable: false,
                fail: None,
                delta: 2.0,
            },
        ];
        let chain = BankChain::new(
            AoSoaScratch::new(BankWidth::Four, 1).expect("scratch"),
            active.to_vec().into_boxed_slice(),
            collapsible.into_iter().map(trace_slot).collect(),
        )
        .expect("collapse prefix");
        assert_eq!(chain.collapse_prefix(), 1);
        drop(chain);
        let mut inactive_prefix = collapsible;
        inactive_prefix[0].mask = [false; 4];
        let chain = BankChain::new(
            AoSoaScratch::new(BankWidth::Four, 1).expect("scratch"),
            active.to_vec().into_boxed_slice(),
            inactive_prefix.into_iter().map(trace_slot).collect(),
        )
        .expect("inactive slot remains owned");
        assert_eq!(
            chain.collapse_prefix(),
            0,
            "packed conversion preserves the original whole-mask collapse clause"
        );
    }

    #[test]
    fn prepared_slot_activity_preserves_dispatch_trace_and_errors() {
        let full = [true; 4];
        let empty = [false; 4];
        let ordinary = [
            TraceSpec {
                id: 0,
                mask: empty,
                seam: SeamSide::UpstreamOfSeam,
                mono_capable: false,
                fail: None,
                delta: 1.0,
            },
            TraceSpec {
                id: 1,
                mask: full,
                seam: SeamSide::UpstreamOfSeam,
                mono_capable: false,
                fail: None,
                delta: 2.0,
            },
            TraceSpec {
                id: 2,
                mask: empty,
                seam: SeamSide::UpstreamOfSeam,
                mono_capable: false,
                fail: None,
                delta: 4.0,
            },
            TraceSpec {
                id: 3,
                mask: full,
                seam: SeamSide::UpstreamOfSeam,
                mono_capable: false,
                fail: None,
                delta: 8.0,
            },
            TraceSpec {
                id: 4,
                mask: empty,
                seam: SeamSide::UpstreamOfSeam,
                mono_capable: false,
                fail: None,
                delta: 16.0,
            },
        ];
        exercise_trace_case(&ordinary, None, 41);

        let mut begin_failure = ordinary;
        begin_failure[3].fail = Some(TraceKind::Begin);
        exercise_trace_case(&begin_failure, None, 73);
        let mut process_failure = ordinary;
        process_failure[3].fail = Some(TraceKind::Dual);
        exercise_trace_case(&process_failure, None, 89);

        let collapsed = [
            TraceSpec {
                id: 5,
                mask: full,
                seam: SeamSide::UpstreamOfSeam,
                mono_capable: true,
                fail: None,
                delta: 1.0,
            },
            TraceSpec {
                id: 6,
                mask: full,
                seam: SeamSide::UpstreamOfSeam,
                mono_capable: true,
                fail: None,
                delta: 2.0,
            },
            TraceSpec {
                id: 7,
                mask: full,
                seam: SeamSide::SeamSide,
                mono_capable: false,
                fail: None,
                delta: 4.0,
            },
            TraceSpec {
                id: 8,
                mask: full,
                seam: SeamSide::SeamSide,
                mono_capable: false,
                fail: None,
                delta: 8.0,
            },
        ];
        exercise_trace_case(&collapsed, Some(2), 101);
        let mut mono_failure = collapsed;
        mono_failure[1].fail = Some(TraceKind::Mono);
        exercise_trace_case(&mono_failure, Some(2), 127);
        let mut seam_failure = collapsed;
        seam_failure[3].fail = Some(TraceKind::Dual);
        exercise_trace_case(&seam_failure, Some(2), 149);
    }

    #[test]
    fn prepared_slot_dispatch_uses_constant_activity_checks() {
        reset_prepared_activity_observation();
        let full = [true; 4];
        let specs = [
            TraceSpec {
                id: 0,
                mask: full,
                seam: SeamSide::UpstreamOfSeam,
                mono_capable: true,
                fail: None,
                delta: 1.0,
            },
            TraceSpec {
                id: 1,
                mask: full,
                seam: SeamSide::SeamSide,
                mono_capable: false,
                fail: None,
                delta: 2.0,
            },
        ];
        TEST_TRACE.with(|trace| *trace.borrow_mut() = TraceObservation::with_queues(&specs));
        let mut chain = BankChain::new(
            AoSoaScratch::new(BankWidth::Four, 4).expect("scratch"),
            vec![true; 4].into_boxed_slice(),
            specs.into_iter().map(trace_slot).collect(),
        )
        .expect("chain");
        let mut planes = Planes {
            left: (0..4).map(|lane| vec![lane as f32; 4]).collect(),
            right: (0..4).map(|lane| vec![32.0 + lane as f32; 4]).collect(),
        };
        chain.force_mono_collapse_off(true);
        chain
            .run(&mut planes, 4, 0, LiveSnapshot::ALL)
            .expect("ordinary run");
        assert!(!chain.is_collapsed());
        chain.force_mono_collapse_off(false);
        chain.arm_mono_collapse(true);
        chain
            .run(&mut planes, 4, 4, LiveSnapshot::ALL)
            .expect("collapsed run");
        assert!(chain.is_collapsed());
        assert_eq!(chain.collapses(), 1);
        let trace = TEST_TRACE.with(|trace| trace.borrow().clone());
        assert_eq!(
            trace.ordered_events(),
            &[
                Some(TraceEvent {
                    kind: TraceKind::Begin,
                    stage: 0,
                    first_sample: 0,
                    frames: 0,
                    queue_before: 2,
                    arithmetic_before: 0
                }),
                Some(TraceEvent {
                    kind: TraceKind::Begin,
                    stage: 1,
                    first_sample: 0,
                    frames: 0,
                    queue_before: 2,
                    arithmetic_before: 0
                }),
                Some(TraceEvent {
                    kind: TraceKind::Dual,
                    stage: 0,
                    first_sample: 0,
                    frames: 4,
                    queue_before: 1,
                    arithmetic_before: 0
                }),
                Some(TraceEvent {
                    kind: TraceKind::Dual,
                    stage: 1,
                    first_sample: 0,
                    frames: 4,
                    queue_before: 1,
                    arithmetic_before: 0
                }),
                Some(TraceEvent {
                    kind: TraceKind::Begin,
                    stage: 0,
                    first_sample: 4,
                    frames: 0,
                    queue_before: 1,
                    arithmetic_before: 1
                }),
                Some(TraceEvent {
                    kind: TraceKind::Begin,
                    stage: 1,
                    first_sample: 4,
                    frames: 0,
                    queue_before: 1,
                    arithmetic_before: 1
                }),
                Some(TraceEvent {
                    kind: TraceKind::Mono,
                    stage: 0,
                    first_sample: 4,
                    frames: 4,
                    queue_before: 0,
                    arithmetic_before: 1
                }),
                Some(TraceEvent {
                    kind: TraceKind::Dual,
                    stage: 1,
                    first_sample: 4,
                    frames: 4,
                    queue_before: 0,
                    arithmetic_before: 1
                }),
            ]
        );
        assert_eq!(trace.queues[..2], [0, 0]);
        assert_eq!(trace.arithmetic[..2], [2, 2]);
        assert_eq!(
            trace_plane_bits(&planes),
            (
                (0..4)
                    .map(|lane| vec![(lane as f32 + 6.0).to_bits(); 4])
                    .collect(),
                (0..4)
                    .map(|lane| vec![(lane as f32 + 6.0).to_bits(); 4])
                    .collect(),
            )
        );
        assert_eq!(
            prepared_activity_observation(),
            9,
            "ordinary, prefix-mono and seam-dual guards plus the prefix stage's publication"
        );
    }

    /// Every bit pattern a 32-bit word can carry that a permutation must not touch: quiet and
    /// signalling NaN payloads, both signed zeroes, both signed subnormals and both infinities.
    const HOSTILE: [f32; 10] = [
        f32::NAN,
        f32::from_bits(0x7fc0_dead),
        f32::from_bits(0xffc0_beef),
        f32::from_bits(0x7f80_0001),
        -0.0,
        0.0,
        f32::from_bits(1),
        f32::from_bits(0x8000_0001),
        f32::INFINITY,
        f32::NEG_INFINITY,
    ];

    /// Planar planes of arbitrary bit patterns, with [`HOSTILE`] salted across every lane so that
    /// no frame shape can miss them.
    fn hostile_planes(lanes: usize, frames: u32, state: &mut u64) -> Planes {
        let mut planes = Planes {
            left: (0..lanes)
                .map(|_| (0..frames).map(|_| seeded(state)).collect())
                .collect(),
            right: (0..lanes)
                .map(|_| (0..frames).map(|_| seeded(state)).collect())
                .collect(),
        };
        for lane in 0..lanes {
            for (index, value) in HOSTILE.into_iter().enumerate() {
                let frame = (lane + index) % frames as usize;
                planes.left[lane][frame] = value;
                planes.right[lane][frames as usize - 1 - frame] = value;
            }
        }
        planes
    }

    /// Frame counts that reach every shape of the tiled round trip at this width: no whole tile at
    /// all, exactly one, one plus a ragged tail, several plus a tail, and the standing quantum.
    fn frame_shapes(lanes: usize) -> [u32; 6] {
        let width = lanes as u32;
        [1, width - 1, width, width + 1, 3 * width + 2, 128]
    }

    fn assert_planes_bit_equal(actual: &Planes, expected: &Planes, what: &str) {
        for lane in 0..actual.left.len() {
            for frame in 0..actual.left[lane].len() {
                assert_eq!(
                    actual.left[lane][frame].to_bits(),
                    expected.left[lane][frame].to_bits(),
                    "{what}: left lane={lane} frame={frame}"
                );
                assert_eq!(
                    actual.right[lane][frame].to_bits(),
                    expected.right[lane][frame].to_bits(),
                    "{what}: right lane={lane} frame={frame}"
                );
            }
        }
    }

    #[test]
    fn direct_plane_views_encode_width_and_reject_short_capacity() {
        let mut left4: [Vec<f32>; 4] = core::array::from_fn(|_| vec![0.0; 7]);
        let mut right4: [Vec<f32>; 4] = core::array::from_fn(|_| vec![0.0; 8]);
        let mut left_iter = left4.iter_mut();
        let mut right_iter = right4.iter_mut();
        let short: [BankPlanePair<'_>; 4] = core::array::from_fn(|_| {
            (
                left_iter.next().expect("four left planes").as_mut_slice(),
                right_iter.next().expect("four right planes").as_mut_slice(),
            )
        });
        assert!(BankPlaneViews::from_four(short, 8).is_none());

        let mut left4: [Vec<f32>; 4] = core::array::from_fn(|_| vec![0.0; 8]);
        let mut right4: [Vec<f32>; 4] =
            core::array::from_fn(|lane| vec![0.0; if lane == 3 { 7 } else { 8 }]);
        let mut left_iter = left4.iter_mut();
        let mut right_iter = right4.iter_mut();
        let late_short_right: [BankPlanePair<'_>; 4] = core::array::from_fn(|_| {
            (
                left_iter.next().expect("four left planes").as_mut_slice(),
                right_iter.next().expect("four right planes").as_mut_slice(),
            )
        });
        assert!(BankPlaneViews::from_four(late_short_right, 8).is_none());

        let mut left4: [Vec<f32>; 4] = core::array::from_fn(|_| vec![0.0; 8]);
        let mut right4: [Vec<f32>; 4] = core::array::from_fn(|_| vec![0.0; 8]);
        let mut left_iter = left4.iter_mut();
        let mut right_iter = right4.iter_mut();
        let complete: [BankPlanePair<'_>; 4] = core::array::from_fn(|_| {
            (
                left_iter.next().expect("four left planes").as_mut_slice(),
                right_iter.next().expect("four right planes").as_mut_slice(),
            )
        });
        let views = BankPlaneViews::from_four(complete, 8).expect("complete four-wide view");
        assert!(views.supports(4, 8));
        assert!(!views.supports(8, 8));
        assert!(!views.supports(4, 9));
    }

    #[test]
    fn direct_scatter_never_calls_the_per_lane_fallback() {
        let mut state = 0x3990_3991_3992_3993;
        let source = hostile_planes(4, 11, &mut state);
        let expected = Planes {
            left: source.left.clone(),
            right: source.right.clone(),
        };
        let mut members = CountedDirectPlanes {
            planes: source,
            per_lane_writes: 0,
        };
        let active = vec![true; 4].into_boxed_slice();
        let mut chain = BankChain::new(
            AoSoaScratch::new(BankWidth::Four, 11).expect("scratch"),
            active.clone(),
            vec![slot(active.to_vec(), Box::new(PassThrough))],
        )
        .expect("chain");
        chain
            .run(&mut members, 11, 0, LiveSnapshot::ALL)
            .expect("run");
        assert_eq!(members.per_lane_writes, 0);
        assert_planes_bit_equal(&members.planes, &expected, "direct call sentinel");
    }

    #[test]
    fn inadequate_claimed_capacity_takes_complete_staged_fallback() {
        let mut state = 0x399a_399b_399c_399d;
        let source = hostile_planes(4, 11, &mut state);
        let expected = Planes {
            left: source.left.clone(),
            right: source.right.clone(),
        };
        let mut malformed = ShortClaimPlanes {
            planes: source,
            per_lane_writes: 0,
        };
        let active = vec![true; 4].into_boxed_slice();
        let mut chain = BankChain::new(
            AoSoaScratch::new(BankWidth::Four, 11).expect("scratch"),
            active.clone(),
            vec![slot(active.to_vec(), Box::new(PassThrough))],
        )
        .expect("chain");
        chain
            .run(&mut malformed, 11, 0, LiveSnapshot::ALL)
            .expect("staged fallback");
        assert_eq!(malformed.per_lane_writes, 4);
        assert_planes_bit_equal(&malformed.planes, &expected, "short-claim staged fallback");
    }

    /// T1b: on a **full** bank -- the tiled whole-bank transpose -- the round trip is bit-exact at
    /// both widths and at every frame shape, ragged tails included.
    ///
    /// T1 below is the partial bank, which is a different code path: a chain with any inactive
    /// lane keeps the per-lane scalar move.
    #[test]
    fn full_bank_gather_scatter_round_trip_is_bit_exact() {
        let mut state = 0x5eed_1234_abcd_0001_u64;
        for &width in BankWidth::ALL {
            let lanes = width.lanes() as usize;
            for frames in frame_shapes(lanes) {
                let mut planes = hostile_planes(lanes, frames, &mut state);
                let expected = Planes {
                    left: planes.left.clone(),
                    right: planes.right.clone(),
                };
                let active = vec![true; lanes];
                let mut chain = BankChain::new(
                    AoSoaScratch::new(width, 128).expect("scratch"),
                    active.clone().into_boxed_slice(),
                    vec![slot(active, Box::new(PassThrough))],
                )
                .expect("chain");
                chain.staging_left.fill(f32::from_bits(0x7fc0_3991));
                chain.staging_right.fill(f32::from_bits(0x7fc0_3992));
                chain
                    .run(&mut planes, frames, 0, LiveSnapshot::ALL)
                    .expect("run");
                assert_planes_bit_equal(
                    &planes,
                    &expected,
                    &format!("{lanes} lanes, {frames} frames"),
                );
                assert_eq!(chain.transposes(), 1);
                assert!(
                    chain
                        .staging_left
                        .iter()
                        .all(|word| word.to_bits() == 0x7fc0_3991)
                );
                assert!(
                    chain
                        .staging_right
                        .iter()
                        .all(|word| word.to_bits() == 0x7fc0_3992)
                );
            }
        }
    }

    /// T1c: the tiled transpose and the per-lane scalar move it replaces are the **same
    /// permutation**, at both widths and every frame shape.
    ///
    /// The stage in the middle rewrites every resident word as a function of its **index in the
    /// block**, so a gather that laid the block out differently would scatter differently: the
    /// planar comparison at the end therefore pins the gather layout as well as the scatter, and
    /// does it without reaching inside the boxed stage. "Agree" is `to_bits` equality, never a
    /// float comparison -- `NaN == NaN` is false and would pass vacuously.
    #[test]
    fn tiled_transpose_matches_the_scalar_path_bit_for_bit() {
        struct IndexXor;
        impl BankStage for IndexXor {
            fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
                for (index, sample) in block.left.iter_mut().enumerate() {
                    *sample = f32::from_bits(sample.to_bits() ^ (index as u32 | 1));
                }
                for (index, sample) in block.right.iter_mut().enumerate() {
                    *sample = f32::from_bits(sample.to_bits() ^ (index as u32 | 2));
                }
                Ok(())
            }
        }

        let mut state = 0x5eed_1234_abcd_0002_u64;
        for &width in BankWidth::ALL {
            let lanes = width.lanes() as usize;
            for frames in frame_shapes(lanes) {
                let source = hostile_planes(lanes, frames, &mut state);
                let mut outcomes = Vec::new();
                for scalar in [false, true] {
                    let mut planes = Planes {
                        left: source.left.clone(),
                        right: source.right.clone(),
                    };
                    let active = vec![true; lanes];
                    let mut chain = BankChain::new(
                        AoSoaScratch::new(width, 128).expect("scratch"),
                        active.clone().into_boxed_slice(),
                        vec![slot(active, Box::new(IndexXor))],
                    )
                    .expect("chain");
                    if scalar {
                        chain.force_scalar_transpose();
                    }
                    chain
                        .run(&mut planes, frames, 0, LiveSnapshot::ALL)
                        .expect("run");
                    outcomes.push(planes);
                }
                let mut staged = StagedPlanes(Planes {
                    left: source.left.clone(),
                    right: source.right.clone(),
                });
                let active = vec![true; lanes];
                let mut staged_chain = BankChain::new(
                    AoSoaScratch::new(width, 128).expect("scratch"),
                    active.clone().into_boxed_slice(),
                    vec![slot(active, Box::new(IndexXor))],
                )
                .expect("chain");
                staged_chain
                    .run(&mut staged, frames, 0, LiveSnapshot::ALL)
                    .expect("staged run");
                assert_planes_bit_equal(
                    &outcomes[0],
                    &outcomes[1],
                    &format!("tiled vs scalar, {lanes} lanes, {frames} frames"),
                );
                assert_planes_bit_equal(
                    &outcomes[0],
                    &staged.0,
                    &format!("direct vs staged, {lanes} lanes, {frames} frames"),
                );
            }
        }
    }

    /// T1: the gather/scatter round-trip is bit-exact for every active lane, including NaN
    /// payloads, `-0.0` and subnormals, and never writes an inactive lane's planar buffer.
    ///
    /// The mask below has inactive lanes, so this is the **partial-bank scalar path** -- the one a
    /// ragged-tail bank takes, and the one that carries the "an inactive lane is never read and
    /// never written" invariant.
    /// Planes with one shared accumulating auxiliary destination (issue #210's PFL seam).
    ///
    /// Every armed lane names *the same* `aux` buffer on purpose: that is the case the seam exists
    /// for, and the case the chain's pairwise-distinct scatter targets cannot express.
    struct PlanesWithSharedAux {
        planes: Planes,
        armed: Vec<bool>,
        aux_left: Vec<f32>,
        aux_right: Vec<f32>,
    }
    impl BankMembers for PlanesWithSharedAux {
        fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
            self.planes.plane(lane)
        }
        fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
            self.planes.plane_mut(lane)
        }
        fn aux_plane_mut(&mut self, lane: usize) -> Option<(&mut [f32], &mut [f32])> {
            self.armed[lane].then_some((&mut self.aux_left[..], &mut self.aux_right[..]))
        }
    }

    /// A stage that multiplies every lane by its lane index, so lanes are distinguishable.
    struct ScaleByLane;
    impl BankStage for ScaleByLane {
        fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
            for frame in 0..block.frames as usize {
                for lane in 0..block.lanes {
                    let index = frame * block.lanes + lane;
                    block.left[index] *= lane as f32;
                    block.right[index] *= lane as f32;
                }
            }
            Ok(())
        }
    }

    /// The PFL seam: absent by default, accumulating when armed, and shareable across lanes.
    ///
    /// Three claims, and the third is the one the seam exists for:
    ///
    /// 1. **Absent by default.** A chain built the way every chain in the engine is built has no
    ///    armed lane, runs no epilogue, and leaves the auxiliary buffers exactly as it found them.
    /// 2. **The lane's own output is untouched.** Arming adds a destination; it does not move one.
    /// 3. **Two lanes may share one destination**, and the result is their sum -- which is exactly
    ///    what a pairwise-distinct scatter target could not express, and why this is a second
    ///    write on the epilogue rather than a redirect.
    ///
    /// Red-mutation proven: making `accumulate_aux` assign (`=`) instead of accumulate (`+=`)
    /// leaves the sum equal to whichever armed lane runs last and fails claim 3.
    #[test]
    fn the_auxiliary_destination_seam_is_absent_by_default_and_accumulates_when_armed() {
        let frames = 64_u32;
        // The build's widest bank (#1112): lanes 2 and 5 of eight share the destination, and lanes
        // 1 and 2 of four.
        let lanes = WIDEST.lanes() as usize;
        let (first, second) = (lanes / 4, lanes * 5 / 8);
        let scratch = AoSoaScratch::new(WIDEST, frames).expect("scratch");
        let mut chain = BankChain::new(
            scratch,
            vec![true; lanes].into_boxed_slice(),
            vec![slot(vec![true; lanes], Box::new(ScaleByLane))],
        )
        .expect("chain");

        let build = |armed: Vec<bool>| PlanesWithSharedAux {
            planes: Planes {
                left: (0..lanes).map(|_| vec![1.0_f32; frames as usize]).collect(),
                right: (0..lanes).map(|_| vec![2.0_f32; frames as usize]).collect(),
            },
            armed,
            aux_left: vec![0.5_f32; frames as usize],
            aux_right: vec![-0.5_f32; frames as usize],
        };

        // (1) Absent by default: the members offer an aux plane on every lane, and the chain must
        // not touch it, because nothing armed it.
        assert!(chain.aux_lanes().is_empty(), "a fresh chain arms nothing");
        let mut absent = build(vec![true; lanes]);
        chain
            .run(&mut absent, frames, 0, LiveSnapshot::ALL)
            .expect("unarmed run");
        assert!(
            absent.aux_left.iter().all(|value| *value == 0.5)
                && absent.aux_right.iter().all(|value| *value == -0.5),
            "an unarmed chain must not write an auxiliary destination"
        );

        // (3) Arm two lanes, which share one destination.
        let mut armed_mask = vec![false; lanes];
        armed_mask[first] = true;
        armed_mask[second] = true;
        chain
            .arm_aux(armed_mask.clone().into_boxed_slice())
            .expect("arm");
        assert_eq!(chain.aux_lanes(), &armed_mask[..]);
        let mut armed = build(armed_mask.clone());
        chain
            .run(&mut armed, frames, 0, LiveSnapshot::ALL)
            .expect("armed run");

        for frame in 0..frames as usize {
            // (2) Every lane's own output is exactly what the unarmed run produced.
            for lane in 0..lanes {
                assert_eq!(
                    armed.planes.left[lane][frame].to_bits(),
                    absent.planes.left[lane][frame].to_bits(),
                    "lane {lane} frame {frame}: arming moved a lane's own output"
                );
            }
            // (3) The shared destination carries the *sum* of both armed lanes, on top of what it
            // already held. Assigning instead of accumulating would give the second lane's alone.
            assert_eq!(
                armed.aux_left[frame],
                0.5 + first as f32 + second as f32,
                "frame {frame}: two armed lanes must sum into one destination"
            );
            assert_eq!(
                armed.aux_right[frame],
                -0.5 + 2.0 * first as f32 + 2.0 * second as f32,
                "frame {frame}: two armed lanes must sum into one destination"
            );
        }

        // Disarming returns the chain to the zero-cost path rather than leaving an all-false mask.
        chain
            .arm_aux(vec![false; lanes].into_boxed_slice())
            .expect("disarm");
        assert!(chain.aux_lanes().is_empty(), "disarming empties the mask");
    }

    /// The seam refuses a mask it cannot render: wrong length, or a lane the chain does not run.
    ///
    /// An inactive lane's scratch is never written, so summing it into a bus would publish the
    /// zero fill as if it were audio. That is a shape error, not a silent no-op.
    /// Planes plus one shared destination every folded lane finishes into (issue #218).
    ///
    /// `fold_plane` applies a per-lane constant in place and then sums the tile into `bus`, which
    /// is the shape the graph runtime's route fold has: the route's 2x2, then the master.
    struct PlanesWithFold {
        planes: Planes,
        gains: Vec<f32>,
        bus_left: Vec<f32>,
        bus_right: Vec<f32>,
        /// Lanes `fold_plane` was called for, in call order.
        taken: Vec<usize>,
        trace: Vec<(char, usize)>,
        cohorts: Vec<Vec<usize>>,
    }
    impl BankMembers for PlanesWithFold {
        fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
            self.planes.plane(lane)
        }
        fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
            self.trace.push(('s', lane));
            self.planes.plane_mut(lane)
        }
        fn fold_plane(&mut self, lane: usize, left: &mut [f32], right: &mut [f32]) {
            self.taken.push(lane);
            self.trace.push(('f', lane));
            let gain = self.gains[lane];
            for (frame, sample) in left.iter_mut().enumerate() {
                *sample *= gain;
                self.bus_left[frame] += *sample;
            }
            for (frame, sample) in right.iter_mut().enumerate() {
                *sample *= gain;
                self.bus_right[frame] += *sample;
            }
        }
        fn fold_cohort(&mut self, mut cohort: FoldCohort<'_>) {
            let ids = cohort.lane_ids().to_vec();
            self.cohorts.push(ids.clone());
            for lane in ids {
                let (left, right) = cohort.planes_mut(lane).expect("valid cohort plane");
                self.fold_plane(lane, left, right);
            }
        }
    }

    struct DefaultFoldProvider(PlanesWithFold);
    impl BankMembers for DefaultFoldProvider {
        fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
            self.0.plane(lane)
        }
        fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
            self.0.plane_mut(lane)
        }
        fn fold_plane(&mut self, lane: usize, left: &mut [f32], right: &mut [f32]) {
            self.0.fold_plane(lane, left, right);
        }
    }

    /// The new constructor and the existing arming path render identical words for both bank
    /// widths, including active holes, prefix subsets, full/partial staging and disarmed masks.
    #[test]
    fn prepared_fold_constructor_preserves_pcm_and_disarm() {
        const FRAMES: u32 = 13;
        for &width in BankWidth::ALL {
            let lanes = width.lanes() as usize;
            for shape in 0..3 {
                let mut active = vec![true; lanes];
                if shape == 1 {
                    active[lanes - 1] = false;
                }
                if shape == 2 {
                    active[1] = false;
                }
                for subset in 0..3 {
                    let fold: Vec<bool> = active
                        .iter()
                        .enumerate()
                        .map(|(lane, active)| {
                            *active
                                && match subset {
                                    0 => false,
                                    1 => lane < 2,
                                    _ => true,
                                }
                        })
                        .collect();
                    let scratch = || AoSoaScratch::new(width, FRAMES).expect("scratch");
                    let slots = || vec![slot(active.clone(), Box::new(ScaleByLane))];
                    let mut old =
                        BankChain::new(scratch(), active.clone().into_boxed_slice(), slots())
                            .expect("old");
                    old.arm_fold(fold.clone().into_boxed_slice())
                        .expect("old arm");
                    let configuration = PreparedFoldConfiguration::new(
                        width,
                        active.clone().into_boxed_slice(),
                        fold.into_boxed_slice(),
                    )
                    .expect("configuration");
                    let mut new =
                        BankChain::new_with_prepared_fold(scratch(), configuration, slots())
                            .expect("new");
                    assert_eq!(old.active, new.active);
                    assert_eq!(old.fold_lanes(), new.fold_lanes());
                    assert_eq!(old.staging_left.len(), new.staging_left.len());
                    let provider = || PlanesWithFold {
                        planes: Planes {
                            left: (0..lanes)
                                .map(|lane| {
                                    (0..FRAMES)
                                        .map(|frame| (lane as f32 + 0.125) * (frame as f32 - 2.0))
                                        .collect()
                                })
                                .collect(),
                            right: (0..lanes)
                                .map(|lane| {
                                    (0..FRAMES)
                                        .map(|frame| (lane as f32 - 0.75) * (frame as f32 + 1.0))
                                        .collect()
                                })
                                .collect(),
                        },
                        gains: (0..lanes).map(|lane| lane as f32 + 0.5).collect(),
                        bus_left: vec![0.0; FRAMES as usize],
                        bus_right: vec![0.0; FRAMES as usize],
                        taken: Vec::new(),
                        trace: Vec::new(),
                        cohorts: Vec::new(),
                    };
                    for disarmed in [false, true] {
                        if disarmed {
                            old.arm_fold(vec![false; lanes].into_boxed_slice())
                                .expect("disarm old");
                            new.arm_fold(vec![false; lanes].into_boxed_slice())
                                .expect("disarm new");
                        }
                        let mut expected = provider();
                        let mut actual = provider();
                        old.run(&mut expected, FRAMES, 0, LiveSnapshot::ALL)
                            .expect("old run");
                        new.run(&mut actual, FRAMES, 0, LiveSnapshot::ALL)
                            .expect("new run");
                        let words = |provider: &PlanesWithFold| {
                            provider
                                .planes
                                .left
                                .iter()
                                .chain(&provider.planes.right)
                                .flat_map(|plane| plane.iter())
                                .chain(&provider.bus_left)
                                .chain(&provider.bus_right)
                                .map(|word| word.to_bits())
                                .collect::<Vec<_>>()
                        };
                        assert_eq!(
                            words(&expected),
                            words(&actual),
                            "{width:?}/{shape}/{subset}/{disarmed}"
                        );
                        assert_eq!(expected.trace, actual.trace);
                        assert_eq!(expected.taken, actual.taken);
                    }
                }
            }
        }
    }

    /// The fold epilogue: absent by default, and when armed it *replaces* the lane's plane write
    /// with the same words handed over.
    ///
    /// Four claims:
    ///
    /// 1. **Absent by default.** A chain built the way every chain in the engine is built folds
    ///    nothing and never calls `fold_plane`, whatever the members offer.
    /// 2. **The handed-over tile is the scatter's own words.** The unarmed run's planar output is
    ///    the oracle: what a folded lane receives, before it applies anything of its own, must be
    ///    bit-identical to what the plane would have been given.
    /// 3. **The lane's plane is not written at all.** A folded lane's buffer keeps the words it
    ///    held before the run -- which is what makes the caller's "nothing reads it" obligation the
    ///    load-bearing one.
    /// 4. **Lane order.** The epilogue visits lanes in ascending order, which is what a
    ///    scatter-accumulate into a shared destination associates in.
    ///
    /// Both transpose paths are exercised: the full bank takes the tiled scatter, the masked chain
    /// takes the per-lane scalar one, and a folded partial bank is the case that made `arm_fold`
    /// allocate a staging block a partial bank does not otherwise own.
    #[test]
    fn the_fold_epilogue_is_absent_by_default_and_replaces_the_lane_write_when_armed() {
        let frames = 48_u32;
        // The build's widest bank (#1112); the partial bank drops its last two lanes.
        let lanes = WIDEST.lanes() as usize;
        for full in [true, false] {
            let mut active = vec![true; lanes];
            if !full {
                active[lanes - 2] = false;
                active[lanes - 1] = false;
            }
            let build = || PlanesWithFold {
                planes: Planes {
                    left: (0..lanes)
                        .map(|lane| vec![1.0_f32 + lane as f32; frames as usize])
                        .collect(),
                    right: (0..lanes)
                        .map(|lane| vec![-2.0_f32 - lane as f32; frames as usize])
                        .collect(),
                },
                gains: (0..lanes).map(|lane| 0.5 + lane as f32).collect(),
                bus_left: vec![0.0; frames as usize],
                bus_right: vec![0.0; frames as usize],
                taken: Vec::new(),
                trace: Vec::new(),
                cohorts: Vec::new(),
            };
            let chain = |active: &[bool]| {
                BankChain::new(
                    AoSoaScratch::new(WIDEST, frames).expect("scratch"),
                    active.to_vec().into_boxed_slice(),
                    vec![slot(active.to_vec(), Box::new(ScaleByLane))],
                )
                .expect("chain")
            };

            // (1) Absent by default.
            let mut unarmed = chain(&active);
            assert!(
                unarmed.fold_lanes().is_empty(),
                "a fresh chain folds nothing"
            );
            let mut oracle = build();
            unarmed
                .run(&mut oracle, frames, 0, LiveSnapshot::ALL)
                .expect("unarmed run");
            assert!(
                oracle.taken.is_empty(),
                "an unarmed chain must never call fold_plane"
            );

            // Fold every rendered lane but lane 1, which keeps its plane write.
            let mut mask = active.clone();
            mask[1] = false;
            let mut folded = chain(&active);
            folded
                .arm_fold(mask.clone().into_boxed_slice())
                .expect("arm");
            assert_eq!(folded.fold_lanes(), &mask[..]);
            let mut armed = build();
            folded
                .run(&mut armed, frames, 0, LiveSnapshot::ALL)
                .expect("folded run");

            // (4) Ascending lane order.
            let expected: Vec<usize> = (0..lanes).filter(|lane| mask[*lane]).collect();
            assert_eq!(armed.taken, expected, "the epilogue visits lanes in order");
            let expected_trace: Vec<(char, usize)> = (0..lanes)
                .filter(|lane| active[*lane])
                .map(|lane| (if mask[lane] { 'f' } else { 's' }, lane))
                .collect();
            assert_eq!(
                armed.trace, expected_trace,
                "mixed folds retain callback/scatter order"
            );
            assert!(
                armed.cohorts.is_empty(),
                "mixed masks retain the per-lane callback path"
            );

            for lane in 0..lanes {
                if !active[lane] {
                    continue;
                }
                if mask[lane] {
                    // (3) A folded lane's plane is exactly what it was before the run.
                    assert!(
                        armed.planes.left[lane]
                            .iter()
                            .all(|value| { value.to_bits() == (1.0_f32 + lane as f32).to_bits() }),
                        "lane {lane}: a folded lane's own plane must not be written"
                    );
                } else {
                    // An unfolded lane still scatters, bit for bit.
                    assert_eq!(
                        armed.planes.left[lane], oracle.planes.left[lane],
                        "lane {lane}: an unfolded lane's scatter moved"
                    );
                }
            }

            // (2) The handed-over tile was the scatter's own words. The bus is the epilogue's own
            // arithmetic over them, so recomputing it from the *unarmed* run's planes -- in the
            // same lane order, with the same per-lane gain -- must reproduce it word for word.
            for frame in 0..frames as usize {
                let mut expected_left = 0.0_f32;
                let mut expected_right = 0.0_f32;
                for lane in (0..lanes).filter(|lane| mask[*lane]) {
                    expected_left += oracle.planes.left[lane][frame] * armed.gains[lane];
                    expected_right += oracle.planes.right[lane][frame] * armed.gains[lane];
                }
                assert_eq!(
                    armed.bus_left[frame].to_bits(),
                    expected_left.to_bits(),
                    "frame {frame}: a folded lane received words the scatter would not have written"
                );
                assert_eq!(
                    armed.bus_right[frame].to_bits(),
                    expected_right.to_bits(),
                    "frame {frame}: a folded lane received words the scatter would not have written"
                );
            }
        }
    }

    /// One recorded [`BankMembers::fold_resident`] offer: `(lanes, frames, lane ids, left bits,
    /// right bits)`.
    type ResidentOffer = (usize, usize, Vec<usize>, Vec<u32>, Vec<u32>);

    /// [`PlanesWithFold`] plus the resident offer (issue #915): records every offer's shape and
    /// words, then accepts or declines it. Accepting does nothing else, so anything the chain
    /// writes after an accepted offer is the chain's own doing.
    struct ResidentProvider {
        inner: PlanesWithFold,
        accept: bool,
        /// Every offer, in call order.
        offers: Vec<ResidentOffer>,
    }
    impl BankMembers for ResidentProvider {
        fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
            self.inner.plane(lane)
        }
        fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
            self.inner.plane_mut(lane)
        }
        fn fold_plane(&mut self, lane: usize, left: &mut [f32], right: &mut [f32]) {
            self.inner.fold_plane(lane, left, right);
        }
        fn fold_cohort(&mut self, cohort: FoldCohort<'_>) {
            self.inner.fold_cohort(cohort);
        }
        fn fold_resident(&mut self, cohort: ResidentFoldCohort<'_>) -> bool {
            self.offers.push((
                cohort.lanes(),
                cohort.frames(),
                cohort.lane_ids().collect(),
                cohort.left().iter().map(|word| word.to_bits()).collect(),
                cohort.right().iter().map(|word| word.to_bits()).collect(),
            ));
            self.accept
        }
    }

    /// Issue #915, the rack half of gate 1: a full bank whose every lane folds offers its resident
    /// block **before** it writes anything, and an accepted offer leaves the staging block and
    /// every plane unwritten.
    ///
    /// Claims, each at both widths and every tiled frame shape (ragged tails and the no-whole-tile
    /// shape included), with hostile words through a stage that makes lanes distinguishable:
    ///
    /// 1. **The offer is the scatter's own words.** The unarmed run's planar output is the oracle:
    ///    the offered block, read as lane `k` frame `f` at `f * W + k`, must be it bit for bit, for
    ///    lanes `0..W` in order and exactly `frames` frames.
    /// 2. **Accepted, nothing else is written.** The staging block keeps a NaN pattern written
    ///    before the run -- the observable proof that the transpose into it never ran -- no plane is
    ///    written, and neither `fold_plane` nor `fold_cohort` is called.
    /// 3. **Declined, the staged path runs exactly as before.** After one offer the chain renders
    ///    what a provider that never overrides `fold_resident` renders, through one `fold_cohort`
    ///    over lanes `0..W`, and the staging block now holds the scatter's words.
    /// 4. **Offered only to a fully folded full bank.** A mixed mask, an unarmed chain and a partial
    ///    bank are never offered the block.
    #[test]
    fn a_fully_folded_full_bank_offers_its_resident_block_before_writing_staging() {
        const STAGING_LEFT: u32 = 0x7fc0_3915;
        const STAGING_RIGHT: u32 = 0x7fc0_3916;
        let mut state = 0x5eed_0915_abcd_0001_u64;
        for &width in BankWidth::ALL {
            let lanes = width.lanes() as usize;
            for frames in frame_shapes(lanes) {
                let source = hostile_planes(lanes, frames, &mut state);
                let provider = |accept: bool| ResidentProvider {
                    inner: PlanesWithFold {
                        planes: source.clone(),
                        gains: (0..lanes).map(|lane| 0.5 + lane as f32).collect(),
                        bus_left: vec![0.0; frames as usize],
                        bus_right: vec![0.0; frames as usize],
                        taken: Vec::new(),
                        trace: Vec::new(),
                        cohorts: Vec::new(),
                    },
                    accept,
                    offers: Vec::new(),
                };
                let chain = |active: &[bool], fold: Option<&[bool]>| {
                    let mut chain = BankChain::new(
                        AoSoaScratch::new(width, 128).expect("scratch"),
                        active.to_vec().into_boxed_slice(),
                        vec![slot(active.to_vec(), Box::new(ScaleByLane))],
                    )
                    .expect("chain");
                    if let Some(fold) = fold {
                        chain
                            .arm_fold(fold.to_vec().into_boxed_slice())
                            .expect("arm");
                    }
                    chain.staging_left.fill(f32::from_bits(STAGING_LEFT));
                    chain.staging_right.fill(f32::from_bits(STAGING_RIGHT));
                    chain
                };
                let all = vec![true; lanes];
                let what = format!("{lanes} lanes, {frames} frames");

                // The oracle: the words the unarmed scatter hands every lane.
                let mut oracle = source.clone();
                chain(&all, None)
                    .run(&mut oracle, frames, 0, LiveSnapshot::ALL)
                    .expect("unarmed run");

                // (1) + (2): accepted.
                let mut folded = chain(&all, Some(&all));
                let mut accepted = provider(true);
                folded
                    .run(&mut accepted, frames, 0, LiveSnapshot::ALL)
                    .expect("accepted run");
                assert_eq!(accepted.offers.len(), 1, "{what}: one offer per block");
                let (offered_lanes, offered_frames, ids, left, right) = &accepted.offers[0];
                assert_eq!(*offered_lanes, lanes, "{what}");
                assert_eq!(*offered_frames, frames as usize, "{what}");
                assert_eq!(*ids, (0..lanes).collect::<Vec<_>>(), "{what}: lane order");
                assert_eq!(
                    left.len(),
                    frames as usize * lanes,
                    "{what}: exactly the used words"
                );
                assert_eq!(
                    right.len(),
                    frames as usize * lanes,
                    "{what}: exactly the used words"
                );
                for lane in 0..lanes {
                    for frame in 0..frames as usize {
                        assert_eq!(
                            left[frame * lanes + lane],
                            oracle.left[lane][frame].to_bits(),
                            "{what}: offered left lane={lane} frame={frame}"
                        );
                        assert_eq!(
                            right[frame * lanes + lane],
                            oracle.right[lane][frame].to_bits(),
                            "{what}: offered right lane={lane} frame={frame}"
                        );
                    }
                }
                assert!(
                    folded
                        .staging_left
                        .iter()
                        .all(|word| word.to_bits() == STAGING_LEFT),
                    "{what}: an accepted offer must leave the left staging block unwritten"
                );
                assert!(
                    folded
                        .staging_right
                        .iter()
                        .all(|word| word.to_bits() == STAGING_RIGHT),
                    "{what}: an accepted offer must leave the right staging block unwritten"
                );
                assert!(
                    accepted.inner.trace.is_empty(),
                    "{what}: no plane, no fold_plane"
                );
                assert!(accepted.inner.cohorts.is_empty(), "{what}: no fold_cohort");
                assert_planes_bit_equal(&accepted.inner.planes, &source, &what);
                assert_eq!(folded.transposes(), 1, "{what}: still one round trip");

                // (3): declined, against a provider that never overrides `fold_resident`.
                let mut declining_chain = chain(&all, Some(&all));
                let mut declined = provider(false);
                declining_chain
                    .run(&mut declined, frames, 0, LiveSnapshot::ALL)
                    .expect("declined run");
                let mut default_chain = chain(&all, Some(&all));
                let mut default = provider(false).inner;
                default_chain
                    .run(&mut default, frames, 0, LiveSnapshot::ALL)
                    .expect("default run");
                assert_eq!(declined.offers.len(), 1, "{what}: offered, then declined");
                assert_eq!(declined.inner.cohorts, vec![(0..lanes).collect::<Vec<_>>()]);
                assert_eq!(declined.inner.trace, default.trace, "{what}");
                assert_eq!(declined.inner.taken, default.taken, "{what}");
                let bits = |words: &[f32]| words.iter().map(|w| w.to_bits()).collect::<Vec<_>>();
                assert_eq!(bits(&declined.inner.bus_left), bits(&default.bus_left));
                assert_eq!(bits(&declined.inner.bus_right), bits(&default.bus_right));
                assert_planes_bit_equal(&declined.inner.planes, &default.planes, &what);
                // The staged path transposed into staging and `fold_plane` then scaled each lane's
                // tile there in place, so staging now holds the scatter's words times the gain.
                let stride = 128;
                for lane in 0..lanes {
                    let gain = 0.5 + lane as f32;
                    for frame in 0..frames as usize {
                        assert_eq!(
                            declining_chain.staging_left[lane * stride + frame].to_bits(),
                            (oracle.left[lane][frame] * gain).to_bits(),
                            "{what}: a declined offer takes the staged transpose"
                        );
                        assert_eq!(
                            declining_chain.staging_right[lane * stride + frame].to_bits(),
                            (oracle.right[lane][frame] * gain).to_bits(),
                            "{what}: a declined offer takes the staged transpose"
                        );
                    }
                }

                // (4): never offered to anything but a fully folded full bank.
                let mut mixed_mask = all.clone();
                mixed_mask[1] = false;
                let mut partial = all.clone();
                partial[lanes - 1] = false;
                for (active, fold, case) in [
                    (&all, Some(&mixed_mask), "mixed mask"),
                    (&all, None, "unarmed"),
                    (&partial, Some(&partial), "partial bank"),
                ] {
                    let mut unoffered = provider(true);
                    chain(active, fold.map(Vec::as_slice))
                        .run(&mut unoffered, frames, 0, LiveSnapshot::ALL)
                        .expect("unoffered run");
                    assert!(unoffered.offers.is_empty(), "{what}: {case} was offered");
                }
            }
        }
    }

    #[test]
    fn resident_fold_cohort_constructor_rejects_every_invalid_shape() {
        let left = [1.0_f32; 16];
        let right = [-1.0_f32; 16];
        for &width in BankWidth::ALL {
            // The frames that fill sixteen words.
            let frames = 16 / width.lanes() as usize;
            let cohort =
                ResidentFoldCohort::new(&left, &right, width, frames).expect("exact shape");
            assert_eq!(cohort.width(), width);
            assert_eq!(cohort.lanes(), width.lanes() as usize);
            assert_eq!(cohort.frames(), frames);
            assert_eq!(cohort.lane_ids(), 0..width.lanes() as usize);
            assert_eq!(cohort.left().len(), 16);
            assert_eq!(cohort.right().len(), 16);
            // A plane one word short or one word long, on either side.
            assert!(matches!(
                ResidentFoldCohort::new(&left[..15], &right, width, frames),
                Err(RackError::Shape)
            ));
            assert!(matches!(
                ResidentFoldCohort::new(&left, &right[..15], width, frames),
                Err(RackError::Shape)
            ));
            assert!(matches!(
                ResidentFoldCohort::new(&left, &right, width, frames - 1),
                Err(RackError::Shape)
            ));
        }
        assert!(matches!(
            ResidentFoldCohort::new(&[], &[], BankWidth::Four, 0),
            Err(RackError::Shape)
        ));
        assert!(matches!(
            ResidentFoldCohort::new(&left, &right, WIDEST, usize::MAX),
            Err(RackError::Overflow)
        ));
    }

    #[test]
    fn fold_cohort_constructor_rejects_every_invalid_shape_without_writes() {
        let ids = [0usize, 1];
        let mut left = [f32::from_bits(0x7fc0_4190); 8];
        let mut right = [f32::from_bits(0x7fc0_4191); 8];
        let left_before = left.map(f32::to_bits);
        let right_before = right.map(f32::to_bits);
        assert!(
            matches!(
                BankChain::new(
                    AoSoaScratch::new(BankWidth::Four, 4).expect("scratch"),
                    vec![false; 4].into_boxed_slice(),
                    vec![slot(vec![false; 4], Box::new(PassThrough))],
                ),
                Err(RackError::Shape)
            ),
            "an empty active set is unrepresentable and cannot invoke a callback"
        );
        assert!(FoldCohort::new(&[], &mut left, &mut right, 4, 4).is_err());
        assert!(FoldCohort::new(&[0; 9], &mut left, &mut right, 4, 4).is_err());
        assert!(FoldCohort::new(&[0, 0], &mut left, &mut right, 4, 4).is_err());
        assert!(FoldCohort::new(&ids, &mut left, &mut right, 3, 4).is_err());
        assert!(FoldCohort::new(&ids, &mut left[..7], &mut right, 4, 4).is_err());
        assert!(FoldCohort::new(&ids, &mut left, &mut right[..7], 4, 4).is_err());
        let poison = f32::from_bits(0x7fc0_4220);
        let mut boundary_left = [poison; 8];
        let mut boundary_right = [poison; 8];
        let boundary_left_before = boundary_left.map(f32::to_bits);
        let boundary_right_before = boundary_right.map(f32::to_bits);
        assert!(FoldCohort::new(&[0], &mut boundary_left[..2], &mut boundary_right, 4, 2).is_err());
        assert!(FoldCohort::new(&[0], &mut boundary_left, &mut boundary_right[..2], 4, 2).is_err());
        assert!(
            FoldCohort::new(
                &[0],
                &mut boundary_left[..4],
                &mut boundary_right[..4],
                4,
                2
            )
            .is_ok()
        );
        assert!(
            FoldCohort::new(&[0, 1], &mut boundary_left[..6], &mut boundary_right, 4, 2).is_err()
        );
        assert!(
            FoldCohort::new(&[0, 1], &mut boundary_left, &mut boundary_right[..6], 4, 2).is_err()
        );
        assert!(FoldCohort::new(&[0, 1], &mut boundary_left, &mut boundary_right, 4, 2).is_ok());
        assert!(FoldCohort::new(&[usize::MAX], &mut left, &mut right, 2, 1).is_err());
        assert!(FoldCohort::new(&[usize::MAX / 2], &mut left, &mut right, 3, 1).is_err());
        assert!(FoldCohort::new(&[0], &mut left[..0], &mut right[..0], 0, 0).is_err());
        assert_eq!(left.map(f32::to_bits), left_before);
        assert_eq!(right.map(f32::to_bits), right_before);
        assert_eq!(boundary_left.map(f32::to_bits), boundary_left_before);
        assert_eq!(boundary_right.map(f32::to_bits), boundary_right_before);

        let mut staged_left = [1.0, 2.0, 91.0, 92.0, 3.0, 4.0, 93.0, 94.0];
        let mut staged_right = [-1.0, -2.0, -91.0, -92.0, -3.0, -4.0, -93.0, -94.0];
        let mut provider = DefaultFoldProvider(PlanesWithFold {
            planes: Planes {
                left: vec![vec![0.0; 2]; 2],
                right: vec![vec![0.0; 2]; 2],
            },
            gains: vec![1.0; 2],
            bus_left: vec![0.0; 2],
            bus_right: vec![0.0; 2],
            taken: Vec::new(),
            trace: Vec::new(),
            cohorts: Vec::new(),
        });
        provider.fold_cohort(
            FoldCohort::new(&ids, &mut staged_left, &mut staged_right, 4, 2)
                .expect("valid strided cohort"),
        );
        assert_eq!(provider.0.taken, vec![0, 1]);
        assert_eq!(
            [
                staged_left[2],
                staged_left[3],
                staged_left[6],
                staged_left[7]
            ],
            [91.0, 92.0, 93.0, 94.0]
        );
        assert_eq!(
            [
                staged_right[2],
                staged_right[3],
                staged_right[6],
                staged_right[7]
            ],
            [-91.0, -92.0, -93.0, -94.0]
        );
    }

    #[test]
    fn default_and_override_fold_identical_complete_strided_cohorts() {
        for (ids, extra) in [
            (&[0usize, 1][..], 0usize),
            (&[0, 1][..], 3),
            (&[0, 2][..], 0),
            (&[0, 2][..], 3),
        ] {
            let capacity = (ids.iter().copied().max().unwrap() + 1) * 4 + extra;
            let make = || PlanesWithFold {
                planes: Planes {
                    left: vec![vec![0.0; 2]; 3],
                    right: vec![vec![0.0; 2]; 3],
                },
                gains: vec![2.0, 3.0, 4.0],
                bus_left: vec![0.0; 2],
                bus_right: vec![0.0; 2],
                taken: Vec::new(),
                trace: Vec::new(),
                cohorts: Vec::new(),
            };
            let sentinel = f32::from_bits(0x7fc0_4221);
            let mut default_left = vec![sentinel; capacity];
            let mut default_right = vec![sentinel; capacity];
            let mut override_left = default_left.clone();
            let mut override_right = default_right.clone();
            for &id in ids {
                default_left[id * 4..id * 4 + 2]
                    .copy_from_slice(&[id as f32 + 1.0, id as f32 + 2.0]);
                default_right[id * 4..id * 4 + 2]
                    .copy_from_slice(&[-(id as f32) - 1.0, -(id as f32) - 2.0]);
            }
            override_left.copy_from_slice(&default_left);
            override_right.copy_from_slice(&default_right);
            let before_left = default_left.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
            let before_right = default_right
                .iter()
                .map(|x| x.to_bits())
                .collect::<Vec<_>>();
            let mut default = DefaultFoldProvider(make());
            let mut optimized = make();
            default.fold_cohort(
                FoldCohort::new(ids, &mut default_left, &mut default_right, 4, 2)
                    .expect("default shape"),
            );
            optimized.fold_cohort(
                FoldCohort::new(ids, &mut override_left, &mut override_right, 4, 2)
                    .expect("override shape"),
            );
            assert_eq!(default.0.taken, ids);
            assert_eq!(optimized.taken, ids);
            assert_eq!(
                default
                    .0
                    .bus_left
                    .iter()
                    .map(|x| x.to_bits())
                    .collect::<Vec<_>>(),
                optimized
                    .bus_left
                    .iter()
                    .map(|x| x.to_bits())
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                default
                    .0
                    .bus_right
                    .iter()
                    .map(|x| x.to_bits())
                    .collect::<Vec<_>>(),
                optimized
                    .bus_right
                    .iter()
                    .map(|x| x.to_bits())
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                default_left.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                override_left
                    .iter()
                    .map(|x| x.to_bits())
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                default_right
                    .iter()
                    .map(|x| x.to_bits())
                    .collect::<Vec<_>>(),
                override_right
                    .iter()
                    .map(|x| x.to_bits())
                    .collect::<Vec<_>>()
            );
            for index in 0..capacity {
                if !ids.iter().any(|id| index >= id * 4 && index < id * 4 + 2) {
                    assert_eq!(default_left[index].to_bits(), before_left[index]);
                    assert_eq!(default_right[index].to_bits(), before_right[index]);
                }
            }
        }
    }

    /// The fold refuses a mask it cannot render, exactly as the auxiliary seam does.
    #[test]
    fn arming_a_fold_refuses_an_unrenderable_mask() {
        let frames = 32_u32;
        // The build's widest bank, its last lane not rendered (#1112).
        let lanes = WIDEST.lanes() as usize;
        let mut active = vec![true; lanes];
        active[lanes - 1] = false;
        let scratch = AoSoaScratch::new(WIDEST, frames).expect("scratch");
        let mut chain = BankChain::new(
            scratch,
            active.clone().into_boxed_slice(),
            vec![slot(active.clone(), Box::new(PassThrough))],
        )
        .expect("chain");
        assert_eq!(
            chain.arm_fold(vec![true; lanes - 1].into_boxed_slice()),
            Err(RackError::Shape),
            "a mask that is not the chain's lane count is refused"
        );
        assert_eq!(
            chain.arm_fold(vec![true; lanes].into_boxed_slice()),
            Err(RackError::Shape),
            "folding the last lane, which this chain does not render, is refused"
        );
        assert!(
            chain.fold_lanes().is_empty(),
            "a refused arm leaves the chain on the zero-cost path"
        );
        chain
            .arm_fold(active.into_boxed_slice())
            .expect("folding exactly the rendered lanes is admitted");
        chain
            .arm_fold(vec![false; lanes].into_boxed_slice())
            .expect("disarm");
        assert!(
            chain.fold_lanes().is_empty(),
            "disarming empties the mask rather than leaving an all-false one"
        );
    }

    #[test]
    fn gather_scatter_round_trip_is_bit_exact() {
        let frames = 128_u32;
        // The build's widest bank (#1112).
        let lanes = WIDEST.lanes() as usize;
        let mut state = 0x0123_4567_89ab_cdef_u64;
        let mut planes = Planes {
            left: (0..lanes)
                .map(|_| (0..frames).map(|_| seeded(&mut state)).collect())
                .collect(),
            right: (0..lanes)
                .map(|_| (0..frames).map(|_| seeded(&mut state)).collect())
                .collect(),
        };
        for (index, value) in [
            f32::NAN,
            f32::from_bits(0x7fc0_dead),
            f32::from_bits(0xffc0_beef),
            -0.0,
            f32::from_bits(1),
            f32::from_bits(0x8000_0001),
            f32::INFINITY,
            f32::NEG_INFINITY,
        ]
        .into_iter()
        .enumerate()
        {
            planes.left[index % lanes][index] = value;
            planes.right[index % lanes][index + 8] = value;
        }
        let expected_left = planes.left.clone();
        let expected_right = planes.right.clone();
        // Lanes 2 and 6 of eight inactive; lane 2 of four.
        let active: Vec<bool> = (0..lanes).map(|lane| lane % 4 != 2).collect();
        let mut chain = BankChain::new(
            AoSoaScratch::new(WIDEST, frames).expect("scratch"),
            active.clone().into_boxed_slice(),
            vec![slot(active.clone(), Box::new(PassThrough))],
        )
        .expect("chain");
        chain
            .run(&mut planes, frames, 0, LiveSnapshot::ALL)
            .expect("run");
        for lane in 0..lanes {
            for frame in 0..frames as usize {
                assert_eq!(
                    planes.left[lane][frame].to_bits(),
                    expected_left[lane][frame].to_bits(),
                    "left lane={lane} frame={frame}"
                );
                assert_eq!(
                    planes.right[lane][frame].to_bits(),
                    expected_right[lane][frame].to_bits(),
                    "right lane={lane} frame={frame}"
                );
            }
        }
        assert_eq!(chain.transposes(), 1);
    }

    /// T2: a stage sees the frame-major index law of master plan §4.1.
    #[test]
    fn stage_sees_frame_major_layout() {
        struct Checker {
            expected: Vec<Vec<f32>>,
        }
        impl BankStage for Checker {
            fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
                for (lane, plane) in self.expected.iter().enumerate() {
                    for (frame, sample) in plane.iter().enumerate() {
                        assert_eq!(
                            block.left[frame * block.lanes + lane].to_bits(),
                            sample.to_bits(),
                            "lane={lane} frame={frame}"
                        );
                    }
                }
                Ok(())
            }
        }
        let frames = 7_u32;
        let mut planes = Planes {
            left: (0..4)
                .map(|lane| {
                    (0..frames)
                        .map(|frame| (lane * 100 + frame) as f32)
                        .collect()
                })
                .collect(),
            right: (0..4).map(|_| vec![0.0; frames as usize]).collect(),
        };
        let expected = planes.left.clone();
        let active = vec![true; 4];
        let mut chain = BankChain::new(
            AoSoaScratch::new(BankWidth::Four, frames).expect("scratch"),
            active.clone().into_boxed_slice(),
            vec![slot(active, Box::new(Checker { expected }))],
        )
        .expect("chain");
        chain
            .run(&mut planes, frames, 0, LiveSnapshot::ALL)
            .expect("run");
    }

    /// #1781: a chain's rest mask is the AND of its running slots' masks, within its lanes. A
    /// slot with no active lane never runs and contributes every lane; a slot active on some
    /// lanes answers all of them from its own state.
    ///
    /// Test value: a chain that ANDs in a slot that never runs (declining a bank it could skip),
    /// skips a running slot's answer on its identity lanes, or reports a lane past its own.
    #[test]
    fn a_chain_rests_a_lane_every_running_slot_rests() {
        struct Rest(u32);
        impl BankStage for Rest {
            fn process(&mut self, _block: BankBlock<'_>) -> Result<(), RenderError> {
                Ok(())
            }
            fn lanes_at_rest(&self) -> u32 {
                self.0
            }
        }
        let chain = |slots: Vec<BankSlot>| {
            BankChain::new(
                AoSoaScratch::new(BankWidth::Four, 8).expect("scratch"),
                vec![true; 4].into_boxed_slice(),
                slots,
            )
            .expect("chain")
        };
        let rested = chain(vec![
            slot(vec![true; 4], Box::new(Rest(u32::MAX))),
            // Active on lanes 0 and 1 only, and it still answers lane 3.
            slot(vec![true, true, false, false], Box::new(Rest(0b0111))),
            // Never runs: its declining answer does not count.
            slot(vec![false; 4], Box::new(Rest(0))),
        ]);
        assert_eq!(rested.lanes_at_rest(), 0b0111);
        assert_eq!(chain(vec![]).lanes_at_rest(), 0b1111);
        assert_eq!(
            chain(vec![slot(vec![true; 4], Box::new(PassThrough))]).lanes_at_rest(),
            0,
            "a stage that has not written the predicate declines"
        );
    }

    /// #1107: a skipped block's outputs where no accepting silent fold takes them -- an unfolded
    /// chain writes `+0.0` into every active lane's plane, and a chain whose members decline
    /// [`BankMembers::fold_silent`] (the default), or that folds only some of its lanes, takes the
    /// ordinary scatter over the zeroed block -- with no stage run, one clock advance over every
    /// lane and no transpose; with the skip forbidden the same block processes. The graph's
    /// accepting fold is gate 1 of `graph-compiler/tests/silent_bank_skip.rs`.
    ///
    /// Test value: red if the unfolded skip leaves an output plane's previous words, if a declined
    /// or partial fold drops a lane's `fold_plane` call or hands it stale resident words, or if a
    /// skipped block runs a stage, advances fewer than every lane's clocks or counts a transpose.
    #[test]
    fn a_skipped_block_writes_zero_outputs_where_no_silent_fold_takes_them() {
        /// At rest on every lane; counts its blocks and its clock advances, and keeps the mask.
        /// The test reads the counts back through `as_any`.
        struct Resting([u32; 3]);
        impl BankStage for Resting {
            fn as_any(&self) -> Option<&dyn core::any::Any> {
                Some(self)
            }
            fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
                self.0[0] += 1;
                block.left.fill(1.0);
                block.right.fill(1.0);
                Ok(())
            }
            fn lanes_at_rest(&self) -> u32 {
                u32::MAX
            }
            fn advance_clocks(&mut self, lanes_mask: u32, block: BankBlock<'_>) {
                let zero = block.left.iter().chain(block.right.iter());
                assert!(zero.into_iter().all(|word| word.to_bits() == 0));
                self.0[1] += 1;
                self.0[2] = lanes_mask;
            }
        }
        /// Separate input and output planes; `fold_plane` records each lane and whether its tile
        /// was exact `+0.0`.
        struct Split {
            inputs: Vec<Vec<f32>>,
            outputs: Vec<Vec<f32>>,
            spare: Vec<Vec<f32>>,
            folded: Vec<(usize, bool)>,
        }
        impl BankMembers for Split {
            fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
                (&self.inputs[lane], &self.inputs[lane])
            }
            fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
                (&mut self.outputs[lane], &mut self.spare[lane])
            }
            fn fold_plane(&mut self, lane: usize, left: &mut [f32], right: &mut [f32]) {
                let zero = left
                    .iter()
                    .chain(right.iter())
                    .all(|word| word.to_bits() == 0);
                self.folded.push((lane, zero));
            }
        }
        const FRAMES: u32 = 8;
        let active = vec![true, true, true, false];
        for fold in [
            None,
            Some(vec![true, true, true, false]),
            Some(vec![true, true, false, false]),
        ] {
            let mut chain = BankChain::new(
                AoSoaScratch::new(BankWidth::Four, FRAMES).expect("scratch"),
                active.clone().into_boxed_slice(),
                vec![slot(active.clone(), Box::new(Resting([0; 3])))],
            )
            .expect("chain");
            let counts = |chain: &BankChain| -> [u32; 3] {
                chain.slots[0]
                    .stage
                    .as_any()
                    .and_then(|stage| stage.downcast_ref::<Resting>())
                    .expect("the resting stage")
                    .0
            };
            if let Some(fold) = &fold {
                chain
                    .arm_fold(fold.clone().into_boxed_slice())
                    .expect("fold");
            }
            let planes = |value: f32| vec![vec![value; FRAMES as usize]; 4];
            let mut members = Split {
                inputs: planes(0.0),
                outputs: planes(7.0),
                spare: planes(7.0),
                folded: Vec::new(),
            };
            // A processed block leaves `1.0` in the resident words, so a stale scatter shows.
            chain.set_silent_skip(false);
            chain
                .run(&mut members, FRAMES, 0, LiveSnapshot::ALL)
                .expect("processed block");
            assert_eq!(counts(&chain)[0], 1, "{fold:?}: a forbidden skip processes");
            assert_eq!(chain.silent_skips(), 0, "{fold:?}");
            members.outputs = planes(7.0);
            members.spare = planes(7.0);
            members.folded.clear();
            chain.set_silent_skip(true);
            let transposes = chain.transposes();
            chain
                .run(&mut members, FRAMES, u64::from(FRAMES), LiveSnapshot::ALL)
                .expect("skipped block");
            assert_eq!(chain.silent_skips(), 1, "{fold:?}: the block skipped");
            assert_eq!(counts(&chain)[0], 1, "{fold:?}: no stage ran");
            assert_eq!(counts(&chain)[1], 1, "{fold:?}: one clock advance");
            assert_eq!(counts(&chain)[2], 0b1111, "{fold:?}: every lane");
            assert_eq!(chain.transposes(), transposes, "{fold:?}: no transpose");
            let folds = |lane: usize| fold.as_ref().is_some_and(|fold| fold[lane]);
            assert_eq!(
                members.folded,
                (0..3)
                    .filter(|lane| folds(*lane))
                    .map(|lane| (lane, true))
                    .collect::<Vec<_>>(),
                "{fold:?}: every folded lane took a +0.0 tile"
            );
            for (lane, active) in active.iter().enumerate() {
                let written = !folds(lane) && *active;
                for plane in [&members.outputs[lane], &members.spare[lane]] {
                    assert!(
                        plane
                            .iter()
                            .all(|word| word.to_bits()
                                == if written { 0 } else { 7.0_f32.to_bits() }),
                        "{fold:?}: lane {lane}'s output plane"
                    );
                }
            }
            let (left, right) = chain.final_output_block(FRAMES).expect("resident");
            assert!(
                left.iter().chain(right).all(|word| word.to_bits() == 0),
                "{fold:?}: the resident block is +0.0"
            );
        }
    }

    /// #1504 D3: `run` skips a slot with no active lane, so a slot that holds a live lane while it
    /// is an identity on every lane would never drain it. Preparation refuses that in a debug build.
    ///
    /// Test value: a compiler change that binds a live-control lane into a slot it marks inactive,
    /// which would strand the lane's queue.
    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "a slot with a live lane is active")]
    fn a_slot_with_a_live_lane_must_have_an_active_lane() {
        struct Live;
        impl BankStage for Live {
            fn process(&mut self, _block: BankBlock<'_>) -> Result<(), RenderError> {
                Ok(())
            }
            fn live_lanes(&self) -> u8 {
                0b1
            }
        }
        let _ = BankChain::new(
            AoSoaScratch::new(BankWidth::Four, 8).expect("scratch"),
            vec![true; 4].into_boxed_slice(),
            vec![slot(vec![false; 4], Box::new(Live))],
        );
    }

    /// T4: `BankChain::new` rejects every mask-shape violation, including a slot active on a lane
    /// the chain never gathers.
    #[test]
    fn chain_new_rejects_mask_shape_and_lane_implication() {
        let scratch = || AoSoaScratch::new(BankWidth::Four, 8).expect("scratch");
        assert_eq!(
            BankChain::new(
                scratch(),
                vec![true, true, false, false].into_boxed_slice(),
                vec![slot(vec![true, true, true, false], Box::new(PassThrough))],
            )
            .err(),
            Some(RackError::Shape),
            "a slot may not be active on a lane the chain never gathers"
        );
        assert_eq!(
            BankChain::new(
                scratch(),
                vec![true; 3].into_boxed_slice(),
                vec![slot(vec![true; 4], Box::new(PassThrough))],
            )
            .err(),
            Some(RackError::Shape),
            "the chain mask must have exactly `lanes` entries"
        );
        assert_eq!(
            BankChain::new(
                scratch(),
                vec![true; 4].into_boxed_slice(),
                vec![slot(vec![true; 5], Box::new(PassThrough))],
            )
            .err(),
            Some(RackError::Shape),
            "a slot mask must have exactly `lanes` entries"
        );
        assert_eq!(
            BankChain::new(
                scratch(),
                vec![false; 4].into_boxed_slice(),
                vec![slot(vec![false; 4], Box::new(PassThrough))],
            )
            .err(),
            Some(RackError::Shape),
            "a chain with no active lane is not a bank"
        );
        assert!(
            BankChain::new(
                scratch(),
                vec![true, true, false, false].into_boxed_slice(),
                vec![slot(vec![true, false, false, false], Box::new(PassThrough))],
            )
            .is_ok()
        );
    }

    /// T5: scratch dimensions match the declared bank and a zero quantum is refused.
    #[test]
    fn scratch_dimensions_match_the_bank_and_zero_quantum_is_refused() {
        let scratch = AoSoaScratch::new(WIDEST, 128).expect("scratch");
        assert_eq!(scratch.left.len(), 128 * WIDEST.lanes() as usize);
        assert_eq!(scratch.right.len(), 128 * WIDEST.lanes() as usize);
        assert_eq!(
            AoSoaScratch::new(BankWidth::Four, 0).err(),
            Some(RackError::ZeroQuantum)
        );
    }

    /// T6: `BankChain::run` is partition invariant over the master plan's block sizes. The chain
    /// is the block API #96 introduces, so the gate lives here: a stateful stage driven in blocks
    /// of {1, 7, 64, 128, 512} produces bit-identical output to one 512-frame block.
    #[test]
    fn chain_run_is_partition_invariant() {
        /// Per-lane running sum over the resident AoSoA block: state must live in the stage, never
        /// in a per-block local.
        struct RunningSum {
            left: [f32; 8],
            right: [f32; 8],
        }
        impl BankStage for RunningSum {
            fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
                for frame in 0..block.frames as usize {
                    for lane in 0..block.lanes {
                        let index = frame * block.lanes + lane;
                        self.left[lane] += block.left[index];
                        block.left[index] = self.left[lane];
                        self.right[lane] += block.right[index];
                        block.right[index] = self.right[lane];
                    }
                }
                Ok(())
            }
        }
        let frames = 512_usize;
        // The build's widest bank (#1112).
        let lanes = WIDEST.lanes() as usize;
        let mut state = 0x0bad_c0de_u64;
        let source: Vec<Vec<f32>> = (0..lanes)
            .map(|_| {
                (0..frames)
                    .map(|_| f32::from_bits(seeded(&mut state).to_bits() & 0x3f7f_ffff))
                    .collect()
            })
            .collect();
        let render = |partition: usize| -> Vec<Vec<f32>> {
            let mut chain = BankChain::new(
                AoSoaScratch::new(WIDEST, 512).expect("scratch"),
                vec![true; lanes].into_boxed_slice(),
                vec![slot(
                    vec![true; lanes],
                    Box::new(RunningSum {
                        left: [0.0; 8],
                        right: [0.0; 8],
                    }),
                )],
            )
            .expect("chain");
            let mut out: Vec<Vec<f32>> = (0..lanes).map(|_| Vec::with_capacity(frames)).collect();
            let mut first = 0_usize;
            while first < frames {
                let count = partition.min(frames - first);
                let mut planes = Planes {
                    left: (0..lanes)
                        .map(|lane| source[lane][first..first + count].to_vec())
                        .collect(),
                    right: (0..lanes)
                        .map(|lane| source[lane][first..first + count].to_vec())
                        .collect(),
                };
                chain
                    .run(&mut planes, count as u32, first as u64, LiveSnapshot::ALL)
                    .expect("run");
                for (plane, gathered) in out.iter_mut().zip(planes.left.iter()) {
                    plane.extend_from_slice(gathered);
                }
                first += count;
            }
            out
        };
        let oracle = render(512);
        for partition in [1_usize, 7, 64, 128] {
            let observed = render(partition);
            for lane in 0..lanes {
                for frame in 0..frames {
                    assert_eq!(
                        observed[lane][frame].to_bits(),
                        oracle[lane][frame].to_bits(),
                        "partition={partition} lane={lane} frame={frame}"
                    );
                }
            }
        }
    }

    /// The chain never gathers into or scatters from an inactive lane, so a padded group cannot
    /// disturb an inactive member's planar buffer even when a stage writes the whole block.
    #[test]
    fn inactive_lanes_are_never_gathered_or_scattered() {
        struct FillOnes;
        impl BankStage for FillOnes {
            fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
                block.left.fill(1.0);
                block.right.fill(1.0);
                Ok(())
            }
        }
        let frames = 4_u32;
        let mut planes = Planes {
            left: (0..4)
                .map(|lane| vec![(lane * 10) as f32; frames as usize])
                .collect(),
            right: (0..4)
                .map(|lane| vec![-((lane * 10) as f32); frames as usize])
                .collect(),
        };
        let active = vec![true, false, true, false];
        let mut chain = BankChain::new(
            AoSoaScratch::new(BankWidth::Four, frames).expect("scratch"),
            active.clone().into_boxed_slice(),
            vec![slot(active, Box::new(FillOnes))],
        )
        .expect("chain");
        chain
            .run(&mut planes, frames, 0, LiveSnapshot::ALL)
            .expect("run");
        assert_eq!(planes.left[0], vec![1.0; frames as usize]);
        assert_eq!(planes.left[1], vec![10.0; frames as usize]);
        assert_eq!(planes.left[2], vec![1.0; frames as usize]);
        assert_eq!(planes.left[3], vec![30.0; frames as usize]);
        assert_eq!(planes.right[1], vec![-10.0; frames as usize]);
        assert_eq!(planes.right[3], vec![-30.0; frames as usize]);
    }

    // ---------------------------------------------------------------------------------------
    // The mono collapse.
    // ---------------------------------------------------------------------------------------

    /// A stage that scales the left plane and, in its **dual** body, the right one too.
    ///
    /// The two bodies are deliberately distinguishable: the collapsed body leaves the right plane
    /// alone, exactly as a real one-plane kernel does. So a chain that collapsed when it should not
    /// have produces a right plane that is the *duplicated left* one rather than the scaled right
    /// one, and the difference is visible in the output rather than only in a counter.
    struct Scale {
        gain: [f32; 2],
        symmetric: bool,
        mono: bool,
        desymmetrized: usize,
    }
    impl Scale {
        fn new(gain: [f32; 2], symmetric: bool) -> Self {
            Self {
                gain,
                symmetric,
                mono: false,
                desymmetrized: 0,
            }
        }
    }
    impl BankStage for Scale {
        fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
            for word in block.left.iter_mut() {
                *word *= self.gain[0];
            }
            for word in block.right.iter_mut() {
                *word *= self.gain[1];
            }
            Ok(())
        }
        fn process_mono(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
            self.mono = true;
            for word in block.left.iter_mut() {
                *word *= self.gain[0];
            }
            Ok(())
        }
        fn supports_mono_collapse(&self) -> bool {
            true
        }
        fn desymmetrize(&mut self) {
            self.desymmetrized += 1;
        }
        fn lane_symmetry(&self, _lane: usize) -> ChannelSymmetryWitness {
            if self.symmetric {
                ChannelSymmetryWitness::SYMMETRIC
            } else {
                ChannelSymmetryWitness::symmetric_except(ChannelSymmetryWitness::DESIGNED)
            }
        }
    }

    /// A seam-side 2x2, in the strip's frozen operation order.
    ///
    /// `yl = ll*l + lr*r` -- two multiplies and an add, never `(ll + lr) * l`. The two differ on
    /// `-0.0` content: with `ll = 1.0`, `lr = -1.0` and `l = r = -0.0`,
    /// `1.0*(-0.0) + (-1.0)*(-0.0)` is `(-0.0) + (+0.0) = +0.0`, while `(1.0 + (-1.0)) * (-0.0)`
    /// is `0.0 * (-0.0) = -0.0`. `a_collapsed_seam_keeps_the_matrixs_operation_order` is the gate.
    struct Matrix {
        coefficients: [f32; 4],
    }
    impl BankStage for Matrix {
        fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
            let [ll, lr, rl, rr] = self.coefficients;
            for (left, right) in block.left.iter_mut().zip(block.right.iter_mut()) {
                let (l, r) = (*left, *right);
                *left = ll * l + lr * r;
                *right = rl * l + rr * r;
            }
            Ok(())
        }
        fn seam_side(&self) -> SeamSide {
            SeamSide::SeamSide
        }
        fn lane_symmetry(&self, _lane: usize) -> ChannelSymmetryWitness {
            ChannelSymmetryWitness::SYMMETRIC
        }
    }

    fn mono_chain(lanes: usize, frames: u32, slots: Vec<Box<dyn BankStage>>) -> BankChain {
        let active: Vec<bool> = vec![true; lanes];
        let width = BankWidth::for_lanes(lanes).expect("a bank width");
        let slots = slots
            .into_iter()
            .map(|stage| slot(active.clone(), stage))
            .collect();
        BankChain::new(
            AoSoaScratch::new(width, frames).expect("scratch"),
            active.into_boxed_slice(),
            slots,
        )
        .expect("chain")
    }

    fn identical_planes(lanes: usize, frames: u32) -> Planes {
        let frames = frames as usize;
        let plane: Vec<Vec<f32>> = (0..lanes)
            .map(|lane| {
                (0..frames)
                    .map(|frame| HOSTILE[(lane + frame) % HOSTILE.len()])
                    .collect()
            })
            .collect();
        Planes {
            left: plane.clone(),
            right: plane,
        }
    }

    #[derive(Clone, Copy, Default)]
    struct WitnessCalls {
        bulk: [usize; 3],
        direct: [usize; 3],
        lanes: [[usize; 8]; 3],
        begun: u8,
    }

    thread_local! {
        static WITNESS_CALLS: std::cell::Cell<WitnessCalls> =
            std::cell::Cell::new(WitnessCalls::default());
    }

    fn record_witness_call(update: impl FnOnce(&mut WitnessCalls)) {
        WITNESS_CALLS.with(|calls| {
            let mut value = calls.get();
            update(&mut value);
            calls.set(value);
        });
    }

    fn reset_witness_calls() {
        WITNESS_CALLS.with(|calls| calls.set(WitnessCalls::default()));
    }

    /// Models an external implementation that only overrides the pre-existing lane method.
    struct DefaultWitnessStage {
        id: usize,
        witnesses: [ChannelSymmetryWitness; 8],
    }

    impl BankStage for DefaultWitnessStage {
        fn process(&mut self, _block: BankBlock<'_>) -> Result<(), RenderError> {
            Ok(())
        }

        fn lane_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
            record_witness_call(|calls| calls.lanes[self.id][lane] += 1);
            self.witnesses[lane]
        }
    }

    /// The two outer entry points count separately, so restoring the old lane/slot walk fails
    /// even if it returns the same bits. The concrete inner default also records every lane read.
    struct WitnessStage {
        inner: DefaultWitnessStage,
        width: BankWidth,
        active_lanes: u8,
        require_drains: u8,
        drain_change: Option<ChannelSymmetryWitness>,
    }

    impl BankStage for WitnessStage {
        fn process(&mut self, _block: BankBlock<'_>) -> Result<(), RenderError> {
            Ok(())
        }

        fn supports_mono_collapse(&self) -> bool {
            true
        }

        fn begin_block(&mut self, first_sample: u64, _: LiveSnapshot) -> Result<(), RenderError> {
            record_witness_call(|calls| calls.begun |= 1 << self.inner.id);
            if first_sample == 8
                && let Some(witness) = self.drain_change
            {
                self.inner.witnesses[0] = witness;
            }
            Ok(())
        }

        fn lane_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
            record_witness_call(|calls| calls.direct[self.inner.id] += 1);
            self.inner.lane_symmetry(lane)
        }

        fn lane_symmetry_bank(
            &self,
            width: BankWidth,
            active_lanes: u8,
        ) -> [ChannelSymmetryWitness; 8] {
            assert_eq!(width, self.width);
            assert_eq!(active_lanes, self.active_lanes);
            record_witness_call(|calls| {
                assert_eq!(calls.begun & self.require_drains, self.require_drains);
                calls.bulk[self.inner.id] += 1;
            });
            self.inner.lane_symmetry_bank(width, active_lanes)
        }
    }

    fn witness_slot(
        id: usize,
        width: BankWidth,
        mask: u8,
        witnesses: [ChannelSymmetryWitness; 8],
        require_drains: u8,
        drain_change: Option<ChannelSymmetryWitness>,
    ) -> BankSlot {
        slot(
            (0..width.lanes())
                .map(|lane| mask & (1 << lane) != 0)
                .collect(),
            Box::new(WitnessStage {
                inner: DefaultWitnessStage { id, witnesses },
                width,
                active_lanes: mask,
                require_drains,
                drain_change,
            }),
        )
    }

    #[test]
    fn stage_witness_bank_default_preserves_overrides_width_and_identity_bounds() {
        let values = core::array::from_fn(|lane| ChannelSymmetryWitness::from_terms(lane as u8));
        let classified: Box<dyn BankStage> = Box::new(DefaultWitnessStage {
            id: 0,
            witnesses: values,
        });
        let unclassified: Box<dyn BankStage> = Box::new(PassThrough);
        for &width in BankWidth::ALL {
            for mask in [0, 0x55, 0xff] {
                reset_witness_calls();
                let bank = classified.lane_symmetry_bank(width, mask);
                let queries = WITNESS_CALLS.with(|calls| calls.get().lanes[0]);
                for lane in 0..8 {
                    let active = lane < width.lanes() as usize && mask & (1 << lane) != 0;
                    assert_eq!(queries[lane], usize::from(active));
                    assert_eq!(
                        bank[lane],
                        if active {
                            values[lane]
                        } else {
                            ChannelSymmetryWitness::DECLINED
                        }
                    );
                }
                assert_eq!(
                    unclassified.lane_symmetry_bank(width, mask),
                    [ChannelSymmetryWitness::DECLINED; 8]
                );
            }
        }
    }

    #[test]
    fn stage_witness_bank_aggregates_exact_terms_for_full_partial_and_identity_lanes() {
        let symmetric = ChannelSymmetryWitness::SYMMETRIC;
        for &width in BankWidth::ALL {
            let lanes = width.lanes() as usize;
            let valid = if lanes == 4 { 0x0f } else { 0xff };
            for active_mask in [valid, valid & 0x55] {
                let active: Vec<bool> = (0..lanes)
                    .map(|lane| active_mask & (1 << lane) != 0)
                    .collect();
                let masks = [active_mask & 0x33, active_mask & 0x66, 0];
                let first = core::array::from_fn(|lane| {
                    if lane == 0 {
                        symmetric
                    } else {
                        ChannelSymmetryWitness::symmetric_except(ChannelSymmetryWitness::UNBYPASSED)
                    }
                });
                let second = core::array::from_fn(|lane| {
                    if lane == 2 {
                        ChannelSymmetryWitness::DECLINED
                    } else {
                        ChannelSymmetryWitness::symmetric_except(ChannelSymmetryWitness::LIVE)
                    }
                });
                let chain = BankChain::new(
                    AoSoaScratch::new(width, 8).expect("scratch"),
                    active.clone().into_boxed_slice(),
                    vec![
                        witness_slot(0, width, masks[0], first, 0, None),
                        witness_slot(1, width, masks[1], second, 0, None),
                        witness_slot(
                            2,
                            width,
                            masks[2],
                            [ChannelSymmetryWitness::DECLINED; 8],
                            0,
                            None,
                        ),
                    ],
                )
                .expect("chain");
                let expected: [ChannelSymmetryWitness; 8] = core::array::from_fn(|lane| {
                    if lane >= lanes || !active[lane] {
                        return ChannelSymmetryWitness::DECLINED;
                    }
                    let a = if masks[0] & (1 << lane) != 0 {
                        first[lane]
                    } else {
                        symmetric
                    };
                    let b = if masks[1] & (1 << lane) != 0 {
                        second[lane]
                    } else {
                        symmetric
                    };
                    a.and(b)
                });
                reset_witness_calls();
                assert_eq!(chain.lane_symmetry_bank(3), expected);
                let calls = WITNESS_CALLS.with(std::cell::Cell::get);
                assert_eq!(calls.bulk, [1, 1, 0]);
                assert_eq!(calls.direct, [0; 3]);
                for (stage, mask) in masks.iter().enumerate() {
                    for lane in 0..8 {
                        assert_eq!(
                            calls.lanes[stage][lane],
                            usize::from(mask & (1 << lane) != 0)
                        );
                    }
                }
                let eligible: Vec<bool> = (0..lanes)
                    .filter(|lane| active[*lane])
                    .map(|lane| expected[lane].eligible())
                    .collect();
                assert_eq!(chain.active_lane_eligibility(), eligible);
                assert_eq!(
                    chain.all_lanes_symmetric(),
                    eligible.iter().all(|value| *value)
                );
                assert_eq!(
                    chain.symmetry_counters(),
                    [
                        eligible.iter().filter(|value| **value).count() as u64,
                        eligible.len() as u64
                    ]
                );
                for (lane, witness) in expected.iter().enumerate() {
                    assert_eq!(chain.lane_symmetry(lane), *witness);
                }
                assert_eq!(
                    chain.lane_symmetry(usize::MAX),
                    ChannelSymmetryWitness::DECLINED
                );
            }
        }
    }

    #[test]
    fn stage_witness_bank_entire_armed_decision_publishes_once_even_when_declining() {
        let symmetric = ChannelSymmetryWitness::SYMMETRIC;
        for &width in BankWidth::ALL {
            let lanes = width.lanes() as usize;
            let valid = if lanes == 4 { 0x0f } else { 0xff };
            for mask in [valid, valid & 0x55] {
                for changed in [
                    symmetric,
                    ChannelSymmetryWitness::symmetric_except(ChannelSymmetryWitness::UNBYPASSED),
                    ChannelSymmetryWitness::symmetric_except(ChannelSymmetryWitness::LIVE),
                ] {
                    let active = (0..lanes).map(|lane| mask & (1 << lane) != 0).collect();
                    let mut chain = BankChain::new(
                        AoSoaScratch::new(width, 8).expect("scratch"),
                        active,
                        vec![
                            witness_slot(0, width, mask, [symmetric; 8], 0b11, None),
                            witness_slot(1, width, mask, [symmetric; 8], 0b11, Some(changed)),
                        ],
                    )
                    .expect("chain");
                    chain.arm_mono_collapse(true);
                    let mut planes = identical_planes(lanes, 8);
                    for sample in [0, 8] {
                        reset_witness_calls();
                        chain
                            .run(&mut planes, 8, sample, LiveSnapshot::ALL)
                            .expect("render");
                        let calls = WITNESS_CALLS.with(std::cell::Cell::get);
                        assert_eq!(
                            calls.bulk,
                            [1, 1, 0],
                            "whole decision, including preservation"
                        );
                        assert_eq!(calls.direct, [0; 3], "no outer per-lane virtual calls");
                        for stage in 0..2 {
                            for lane in 0..8 {
                                assert_eq!(
                                    calls.lanes[stage][lane],
                                    usize::from(mask & (1 << lane) != 0)
                                );
                            }
                        }
                        let current = if sample == 0 { symmetric } else { changed };
                        assert_eq!(chain.is_collapsed(), current.eligible());
                        assert_eq!(
                            chain.collapse_channels_agree(),
                            current.preserves_channel_agreement()
                        );
                    }
                    assert_eq!(chain.collapses(), 1 + u64::from(changed.eligible()));
                    assert_eq!(
                        chain.collapse_transitions(),
                        [u64::from(!changed.eligible()), 0, 0]
                    );
                }
            }
        }
    }

    /// A chain nobody armed never collapses, whatever its witness says.
    ///
    /// The `SOURCE` term is decided on the control plane, keyed by track id, and a chain sees only
    /// anonymous lanes -- so `lane_symmetry` admits a track whose two channels read two different
    /// source channels. `arm_mono_collapse` is the join, and the default is `false`.
    ///
    /// Red mutation: initialise `collapse_source` to `true`. This fails, and so does the console
    /// suite's `the_half_mono_cohort_banks_like_a_uniform_one`.
    #[test]
    fn an_unarmed_chain_never_collapses() {
        let mut chain = mono_chain(
            4,
            8,
            vec![
                Box::new(Scale::new([2.0, 2.0], true)),
                Box::new(Matrix {
                    coefficients: [1.0, 0.0, 0.0, 1.0],
                }),
            ],
        );
        assert_eq!(
            chain.collapse_prefix(),
            1,
            "one upstream slot, one seam slot"
        );
        assert!(!chain.can_collapse(), "unarmed until the join is performed");
        let mut planes = identical_planes(4, 8);
        chain
            .run(&mut planes, 8, 0, LiveSnapshot::ALL)
            .expect("render");
        assert_eq!(chain.collapses(), 0);
        chain.arm_mono_collapse(true);
        chain
            .run(&mut planes, 8, 0, LiveSnapshot::ALL)
            .expect("render");
        assert_eq!(chain.collapses(), 1, "armed, and every lane is symmetric");
    }

    /// An armed chain whose witness declines still renders the dual bits.
    ///
    /// This is the "force-collapse on an ineligible bank" gate, and it is stated as an output
    /// comparison rather than as a counter so that a dispatch that engaged wrongly is caught by
    /// what it *rendered*: the mock's collapsed body leaves the right plane alone, so a wrong
    /// engagement publishes the duplicated left plane where the right one's own gain belonged.
    ///
    /// Red mutation: drop `self.all_lanes_symmetric()` from the dispatch conjunction in
    /// `BankChain::run`. The right plane then comes back scaled by the *left* gain and this fails.
    #[test]
    fn an_ineligible_armed_chain_renders_the_dual_bits() {
        for symmetric in [false, true] {
            let mut chain = mono_chain(4, 8, vec![Box::new(Scale::new([2.0, 3.0], symmetric))]);
            chain.arm_mono_collapse(true);
            let mut planes = identical_planes(4, 8);
            let mut reference = identical_planes(4, 8);
            let mut dual = mono_chain(4, 8, vec![Box::new(Scale::new([2.0, 3.0], symmetric))]);
            chain
                .run(&mut planes, 8, 0, LiveSnapshot::ALL)
                .expect("render");
            dual.run(&mut reference, 8, 0, LiveSnapshot::ALL)
                .expect("render");
            assert_eq!(chain.collapses(), u64::from(symmetric));
            // No seam-side slot: a collapsed block's chain copies the left plane over the
            // ungathered right one before the scatter (issue #975), and counts it.
            assert_eq!(chain.seam_copies(), u64::from(symmetric));
            for lane in 0..4 {
                if symmetric {
                    let duplicated = planes.right[lane]
                        .iter()
                        .zip(&planes.left[lane])
                        .all(|(a, b)| a.to_bits() == b.to_bits());
                    assert!(
                        duplicated,
                        "a collapsed block publishes the left plane twice"
                    );
                }
                let left_agrees = planes.left[lane]
                    .iter()
                    .zip(&reference.left[lane])
                    .all(|(a, b)| a.to_bits() == b.to_bits());
                assert!(
                    left_agrees,
                    "the left plane is the dual left plane either way"
                );
                let right_agrees = planes.right[lane]
                    .iter()
                    .zip(&reference.right[lane])
                    .all(|(a, b)| a.to_bits() == b.to_bits());
                assert_eq!(
                    right_agrees, !symmetric,
                    "an ineligible chain must publish its own right plane; an eligible one \
                     publishes the duplicated left plane, which is the collapse"
                );
            }
        }
    }

    /// The seam duplicates the plane **into** the matrix; it does not simplify the matrix.
    ///
    /// With `l == r` the strip's `yl = ll*l + lr*r` is arithmetically `(ll + lr) * l`, and a reader
    /// looking for a saving at the seam would fold the two multiplies into one. It is not the same
    /// value: on `-0.0` content with `ll = 1.0` and `lr = -1.0` the frozen form gives
    /// `(-0.0) + (+0.0) = +0.0` and the folded one `0.0 * (-0.0) = -0.0`.
    ///
    /// So the expected words are written out here in the frozen order rather than taken from a
    /// second chain running the same mock: a comparison of two chains cannot see a change made to
    /// the arithmetic they share. Both the collapsed chain and the dual chain are required to match
    /// it, which is what makes this a statement about the *operation order* and not about the
    /// dispatch.
    ///
    /// Red mutations, both caught: rewrite `Matrix::process`'s lines as `(ll + lr) * l` /
    /// `(rl + rr) * l` (the seam simplification), or fold only the collapsed path by moving the
    /// matrix into the collapsed prefix.
    #[test]
    fn a_collapsed_seam_keeps_the_matrixs_operation_order() {
        let coefficients = [1.0_f32, -1.0, -1.0, 1.0];
        let [ll, lr, rl, rr] = coefficients;
        let lanes = 4;
        let frames = 16_u32;
        let input = identical_planes(lanes, frames);
        // The frozen strip arithmetic, written out. `l` and `r` are the same word here, which is
        // exactly the case the folded form would claim to be equivalent on.
        let expected: Vec<Vec<(f32, f32)>> = (0..lanes)
            .map(|lane| {
                input.left[lane]
                    .iter()
                    .zip(&input.right[lane])
                    .map(|(l, r)| (ll * l + lr * r, rl * l + rr * r))
                    .collect()
            })
            .collect();
        // The teeth: on this content the folded form is a different word.
        let folded_differs = input.left[0]
            .iter()
            .zip(&expected[0])
            .any(|(l, (yl, _))| ((ll + lr) * l).to_bits() != yl.to_bits());
        assert!(
            folded_differs,
            "the corpus must contain a frame where (ll + lr) * l differs from ll*l + lr*r"
        );

        for armed in [true, false] {
            let mut chain = mono_chain(
                lanes,
                frames,
                vec![
                    Box::new(Scale::new([1.0, 1.0], true)),
                    Box::new(Matrix { coefficients }),
                ],
            );
            chain.arm_mono_collapse(armed);
            let mut planes = identical_planes(lanes, frames);
            chain
                .run(&mut planes, frames, 0, LiveSnapshot::ALL)
                .expect("render");
            assert_eq!(chain.collapses(), u64::from(armed));
            for (lane, lane_expected) in expected.iter().enumerate() {
                for (frame, (yl, yr)) in lane_expected.iter().enumerate() {
                    assert_eq!(
                        planes.left[lane][frame].to_bits(),
                        yl.to_bits(),
                        "armed {armed} lane {lane} frame {frame}: left"
                    );
                    assert_eq!(
                        planes.right[lane][frame].to_bits(),
                        yr.to_bits(),
                        "armed {armed} lane {lane} frame {frame}: right"
                    );
                }
            }
        }
    }

    /// A chain whose seam-side slots are not a suffix declines, and so does one with no prefix;
    /// the prefix is the leading run of one-plane upstream slots.
    #[test]
    fn only_a_seam_suffix_over_a_collapsible_prefix_can_collapse() {
        let seam = || -> Box<dyn BankStage> {
            Box::new(Matrix {
                coefficients: [1.0, 0.0, 0.0, 1.0],
            })
        };
        let upstream = || -> Box<dyn BankStage> { Box::new(Scale::new([1.0, 1.0], true)) };
        // Seam-side only: the witness is vacuously symmetric and must not be read as evidence.
        assert_eq!(mono_chain(4, 8, vec![seam()]).collapse_prefix(), 0);
        // A seam-side slot in the middle: this executor does not know where to duplicate.
        assert_eq!(
            mono_chain(4, 8, vec![upstream(), seam(), upstream()]).collapse_prefix(),
            0
        );
        // A first upstream slot with no one-plane body leaves the prefix empty.
        assert_eq!(
            mono_chain(4, 8, vec![Box::new(PassThrough), seam()]).collapse_prefix(),
            0
        );
        assert_eq!(
            mono_chain(4, 8, vec![Box::new(PassThrough), upstream(), seam()]).collapse_prefix(),
            0
        );
        // A later one ends the prefix instead (issue #974): the seam moves before it.
        assert_eq!(
            mono_chain(4, 8, vec![upstream(), Box::new(PassThrough), seam()]).collapse_prefix(),
            1
        );
        // The strip's own shape.
        assert_eq!(
            mono_chain(4, 8, vec![upstream(), upstream(), seam(), seam()]).collapse_prefix(),
            2
        );
    }

    /// Planar planes whose two channels **differ**, for the cases where `L == R` would hide the bug.
    ///
    /// Every other collapse test in this module feeds identical planes, because that is what a
    /// collapse-eligible track looks like. The two clauses of `collapse_prefix_of` that gate a
    /// chain the collapse must *refuse* need the opposite: if the refusal failed and the seam ran,
    /// the right plane would come back as the duplicated left one, and identical inputs would make
    /// that invisible.
    fn distinct_planes(lanes: usize, frames: u32) -> Planes {
        let frames = frames as usize;
        Planes {
            left: (0..lanes)
                .map(|lane| (0..frames).map(|f| (lane * 100 + f) as f32 + 1.0).collect())
                .collect(),
            right: (0..lanes)
                .map(|lane| {
                    (0..frames)
                        .map(|f| -((lane * 100 + f) as f32) - 1.0)
                        .collect()
                })
                .collect(),
        }
    }

    /// An **armed** chain of nothing but seam-side slots must not collapse.
    ///
    /// Clause 2 of [`BankChain::collapse_prefix_of`], and the one whose failure is silent. A
    /// fader-or-matrix-only chain reports every lane symmetric on every session, mono or not,
    /// because `SEAM_SIDE_WITNESS` is an unconditional `SYMMETRIC` -- so `all_lanes_symmetric` and
    /// the structural join both say yes and only the empty prefix says no. If the dispatch dropped
    /// `collapse_prefix > 0`, the seam would publish the duplicated left plane as this chain's
    /// right output on a session whose two channels legitimately differ.
    ///
    /// Red mutation: `collapse_prefix > 0` becomes `collapse_prefix >= 0` in `BankChain::run`.
    #[test]
    fn an_armed_seam_side_only_chain_renders_the_dual_bits() {
        let coefficients = [1.0_f32, 0.5, 0.25, 2.0];
        let mut chain = mono_chain(4, 8, vec![Box::new(Matrix { coefficients })]);
        assert_eq!(
            chain.collapse_prefix(),
            0,
            "a seam-side-only chain has no prefix"
        );
        chain.arm_mono_collapse(true);
        let mut planes = distinct_planes(4, 8);
        let source = distinct_planes(4, 8);
        chain
            .run(&mut planes, 8, 0, LiveSnapshot::ALL)
            .expect("render");
        assert_eq!(
            chain.collapses(),
            0,
            "a vacuous witness must never read as collapse evidence"
        );
        let [ll, lr, rl, rr] = coefficients;
        for lane in 0..4 {
            for frame in 0..8 {
                let (l, r) = (source.left[lane][frame], source.right[lane][frame]);
                assert_eq!(
                    planes.left[lane][frame].to_bits(),
                    (ll * l + lr * r).to_bits(),
                    "lane {lane} frame {frame}: left"
                );
                assert_eq!(
                    planes.right[lane][frame].to_bits(),
                    (rl * l + rr * r).to_bits(),
                    "lane {lane} frame {frame}: the right plane must be the dual matrix output, \
                     not the duplicated left one"
                );
            }
        }
    }

    /// A chain holding a slot that runs on only some of its lanes must not collapse.
    ///
    /// Clause 4 of [`BankChain::collapse_prefix_of`]. The collapse is all-lanes-or-nothing and the
    /// witness is conjoined over the slots a lane actually runs, so a slot that is an identity on
    /// some lanes makes "every active lane is eligible" and "every slot agreed for every active
    /// lane" two different statements -- and the dispatch reads the first. Declining the whole
    /// chain is what keeps them one statement.
    ///
    /// Red mutation: drop the `lanes_agree` clause from `collapse_prefix_of`.
    #[test]
    fn a_partial_agreement_slot_declines_the_collapse() {
        let active: Box<[bool]> = vec![true; 4].into_boxed_slice();
        let slots = vec![
            slot(active.to_vec(), Box::new(Scale::new([2.0, 3.0], true))),
            slot(
                vec![true, false, true, false],
                Box::new(Scale::new([5.0, 7.0], true)),
            ),
        ];
        let mut chain = BankChain::new(
            AoSoaScratch::new(BankWidth::Four, 8).expect("scratch"),
            active,
            slots,
        )
        .expect("chain");
        assert_eq!(
            chain.collapse_prefix(),
            0,
            "a slot that does not run every lane of the chain zeroes the prefix"
        );
        chain.arm_mono_collapse(true);
        let mut planes = distinct_planes(4, 8);
        let source = distinct_planes(4, 8);
        chain
            .run(&mut planes, 8, 0, LiveSnapshot::ALL)
            .expect("render");
        assert_eq!(chain.collapses(), 0);
        // Both stages run the whole resident block whenever any lane of theirs is live -- the mask
        // gates the *slot*, not the words -- so the dual bits are both gains on every lane. What is
        // load-bearing here is only that the right plane is the right channel's own arithmetic.
        for lane in 0..4 {
            assert_eq!(
                planes.right[lane][0].to_bits(),
                (source.right[lane][0] * 3.0 * 7.0).to_bits(),
                "lane {lane}: the right plane must be its own, not the duplicated left"
            );
        }
    }

    /// The disengage copy runs once, on the block that stops collapsing, and only for the prefix.
    #[test]
    fn disengaging_copies_the_prefixs_state_once() {
        let mut chain = mono_chain(
            4,
            8,
            vec![
                Box::new(Scale::new([1.0, 1.0], true)),
                Box::new(Matrix {
                    coefficients: [1.0, 0.0, 0.0, 1.0],
                }),
            ],
        );
        chain.arm_mono_collapse(true);
        let mut planes = identical_planes(4, 8);
        chain
            .run(&mut planes, 8, 0, LiveSnapshot::ALL)
            .expect("render");
        assert!(chain.is_collapsed());
        chain.force_mono_collapse_off(true);
        chain
            .run(&mut planes, 8, 0, LiveSnapshot::ALL)
            .expect("render");
        assert!(!chain.is_collapsed());
        assert_eq!(
            chain.collapse_transitions(),
            [1, 0, 0],
            "one disengage, no re-engage yet and no proof: the copy is the premise, not a proof"
        );
        assert!(
            chain.collapse_channels_agree(),
            "the disengage copy re-establishes agreement; a witness that still holds keeps it"
        );
    }
}

/// The plan-swap carry of an [`EffectBankStage`] lane (#1279 D3, D5, D6), against a bank whose
/// payload holds exactly what the stage's witness reads.
#[cfg(test)]
mod effect_lane_carry_tests {
    use super::*;
    use effect_contract::{
        BankProcessReport, EffectQuality, LinkMode, PreparedBankMetadata, PreparedPorts, ResetKind,
        StatePayloadError, TailSamples,
    };

    const LANES: usize = 4;
    /// Per channel: one running word and one designed word, four bytes each.
    const SIZES: StatePayloadSizes = StatePayloadSizes {
        common_bytes: 0,
        left_bytes: 8,
        right_bytes: 8,
    };

    /// One channel of one lane: the running word a block advances and the designed word the
    /// witness compares between the channels.
    type Channel = [u32; 2];

    /// A bank whose lanes hold `[left, right]` channels, whose payload is those channels' bytes,
    /// whose `lane_channel_symmetry` is the designed words' equality, and whose block writes each
    /// lane's running words into the planes, so a test reads a lane's state off the output.
    struct WordBank {
        lanes: [[Channel; 2]; LANES],
        refuse_restore: bool,
    }

    impl WordBank {
        fn new(lanes: [[Channel; 2]; LANES]) -> Self {
            Self {
                lanes,
                refuse_restore: false,
            }
        }

        fn bound(self, layout_version: u32) -> PreparedEffectBank {
            PreparedEffectBank {
                processor: Box::new(self),
                metadata: PreparedBankMetadata {
                    width: BankWidth::Four,
                    program_key: EffectProgramKey {
                        effect_id: EffectId::parse("mock.words").expect("id"),
                        contract_major: 1,
                        state_layout_version: layout_version,
                        sample_rate: 48_000,
                        quantum: 8,
                        quality: EffectQuality::Normal,
                        bypass: false,
                        link_mode: LinkMode::DualMono,
                        ports: PreparedPorts {
                            sidechain: effect_contract::PreparedSidechainPort::None,
                        },
                        latency: LatencySamples(0),
                        tail: TailSamples::Finite(0),
                        tail_every_peak: TailSamples::Infinite,
                        rest: effect_contract::RestBound::Unstated,
                        composition: effect_contract::CompositionBound::Unstated,
                        state_sizes: SIZES,
                        scratch_bytes: 0,
                        automation_capacity: 0,
                    },
                },
            }
        }
    }

    fn bytes(channel: Channel) -> [u8; 8] {
        let mut out = [0_u8; 8];
        out[..4].copy_from_slice(&channel[0].to_le_bytes());
        out[4..].copy_from_slice(&channel[1].to_le_bytes());
        out
    }

    fn word(bytes: &[u8]) -> u32 {
        u32::from_le_bytes(bytes.try_into().expect("four bytes"))
    }

    impl PreparedNativeEffectBank for WordBank {
        fn reset(&mut self, _kind: ResetKind) {}

        fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
            for lane in 0..LANES {
                block.left[lane] = f32::from_bits(self.lanes[lane][0][0]);
                block.right[lane] = f32::from_bits(self.lanes[lane][1][0]);
            }
            BankProcessReport::empty(BankWidth::Four)
        }

        fn snapshot_track_state_payload(
            &self,
            track_index: u32,
            output: StatePayloadOutput<'_>,
        ) -> Result<(), StatePayloadError> {
            let lane = self
                .lanes
                .get(track_index as usize)
                .ok_or(StatePayloadError {
                    code: "effect.state.lane",
                })?;
            output.left.copy_from_slice(&bytes(lane[0]));
            output.right.copy_from_slice(&bytes(lane[1]));
            Ok(())
        }

        fn restore_track_state_payload(
            &mut self,
            track_index: u32,
            _state_layout_version: u32,
            input: StatePayloadInput<'_>,
        ) -> Result<(), StatePayloadError> {
            if self.refuse_restore {
                return Err(StatePayloadError {
                    code: "effect.state.refused",
                });
            }
            let lane = self
                .lanes
                .get_mut(track_index as usize)
                .ok_or(StatePayloadError {
                    code: "effect.state.lane",
                })?;
            *lane = [
                [word(&input.left[..4]), word(&input.left[4..])],
                [word(&input.right[..4]), word(&input.right[4..])],
            ];
            Ok(())
        }

        fn lane_channel_symmetry(&self, lane: usize) -> bool {
            self.lanes[lane][0][1] == self.lanes[lane][1][1]
        }
    }

    fn stage(lanes: [[Channel; 2]; LANES], layout_version: u32) -> EffectBankStage {
        EffectBankStage::new(
            WordBank::new(lanes).bound(layout_version),
            BankWidth::Four,
            8,
        )
        .expect("stage")
    }

    /// Lane `n` at rest: running word `n`, designed word `100 + n`, equal on both channels.
    fn at_rest() -> [[Channel; 2]; LANES] {
        core::array::from_fn(|lane| {
            let channel = [lane as u32 + 1, 100 + lane as u32];
            [channel, channel]
        })
    }

    /// The running word of `lane`'s left channel, read off one block.
    fn running(stage: &mut EffectBankStage) -> [u32; LANES] {
        let (mut left, mut right) = ([0.0_f32; LANES], [0.0_f32; LANES]);
        stage
            .process(BankBlock {
                left: &mut left,
                right: &mut right,
                frames: 1,
                first_sample: 0,
                lanes: LANES,
            })
            .expect("block");
        core::array::from_fn(|lane| left[lane].to_bits())
    }

    /// The lane moves to the lane index it is given, not the one it came from: successor lane 2
    /// takes predecessor lane 1's state, and no other lane changes.
    #[test]
    fn a_carried_lane_lands_on_the_lane_it_was_given() {
        let mut predecessor_lanes = at_rest();
        predecessor_lanes[1] = [[41, 100 + 1], [42, 100 + 1]];
        let predecessor = stage(predecessor_lanes, 1);
        let mut successor = stage(at_rest(), 1);
        let mut scratch = [0_u8; 16];
        assert!(successor.carry_lane_from(2, &predecessor, 1, &mut scratch));
        assert_eq!(running(&mut successor), [1, 2, 41, 4]);
        assert_eq!(successor.carry_refused(), 0);
    }

    /// D5, the `DESIGNED` term: the restored payload's designed words differ between the channels,
    /// so the stage's cached answer for the lane (symmetric at bind) is stale and the witness must
    /// say so. A stage that restores without recomputing keeps the bind-time answer and collapses
    /// a lane whose channels run on different words.
    #[test]
    fn a_restored_lane_recomputes_the_designed_term() {
        let mut predecessor_lanes = at_rest();
        predecessor_lanes[1] = [[7, 300], [7, 301]];
        let predecessor = stage(predecessor_lanes, 1);
        let mut successor = stage(at_rest(), 1);
        assert!(successor.lane_symmetry(2).eligible());
        let mut scratch = [0_u8; 16];
        assert!(successor.carry_lane_from(2, &predecessor, 1, &mut scratch));
        assert_eq!(
            successor.lane_symmetry(2).declined(),
            ChannelSymmetryWitness::DESIGNED | ChannelSymmetryWitness::RESTORED
        );
        // The lanes this stage did not restore keep their witness.
        for lane in [0, 1, 3] {
            assert!(successor.lane_symmetry(lane).eligible(), "lane {lane}");
        }
    }

    /// D5, the `RESTORED` term, alone: the designed words agree but the running words of the two
    /// channels do not, so only the payload's sections tell the stage the channels diverged. A
    /// restore that skips the section comparison collapses a lane whose state disagrees.
    #[test]
    fn a_restored_lane_with_divergent_sections_clears_the_restored_term() {
        let mut predecessor_lanes = at_rest();
        predecessor_lanes[0] = [[5, 100], [6, 100]];
        let predecessor = stage(predecessor_lanes, 1);
        let mut successor = stage(at_rest(), 1);
        let mut scratch = [0_u8; 16];
        assert!(successor.carry_lane_from(3, &predecessor, 0, &mut scratch));
        assert_eq!(
            successor.lane_symmetry(3).declined(),
            ChannelSymmetryWitness::RESTORED
        );
    }

    /// A lane restored from equal sections stays eligible: the carry never costs a lane its
    /// collapse (gate 2 of #1279 rests on it).
    #[test]
    fn a_restored_lane_with_equal_sections_stays_eligible() {
        let predecessor = stage(at_rest(), 1);
        let mut successor = stage(at_rest(), 1);
        let mut scratch = [0_u8; 16];
        assert!(successor.carry_lane_from(0, &predecessor, 3, &mut scratch));
        assert!(successor.lane_symmetry(0).eligible());
    }

    /// D6: a restore that refuses leaves the lane as prepared and counts, saturating, and never
    /// panics.
    #[test]
    fn a_refused_restore_leaves_the_lane_at_rest_and_counts() {
        let mut predecessor_lanes = at_rest();
        predecessor_lanes[1] = [[41, 101], [42, 101]];
        let predecessor = stage(predecessor_lanes, 1);
        let mut refusing = WordBank::new(at_rest());
        refusing.refuse_restore = true;
        let mut successor =
            EffectBankStage::new(refusing.bound(1), BankWidth::Four, 8).expect("stage");
        let mut scratch = [0_u8; 16];
        assert!(!successor.carry_lane_from(2, &predecessor, 1, &mut scratch));
        assert_eq!(successor.carry_refused(), 1);
        assert_eq!(
            running(&mut successor),
            [1, 2, 3, 4],
            "the lane is as prepared"
        );
        assert!(
            successor.lane_symmetry(2).eligible(),
            "its witness is unchanged"
        );
        assert!(!successor.carry_lane_from(2, &predecessor, 1, &mut scratch));
        assert_eq!(successor.carry_refused(), 2);
        successor.carry_refused = u64::MAX;
        assert!(!successor.carry_lane_from(2, &predecessor, 1, &mut scratch));
        assert_eq!(successor.carry_refused(), u64::MAX, "saturating");
    }

    /// D1: two lanes of different payload layouts, a lane past the bank and a scratch too small
    /// for the payload are not carried, and a carry that is asked for anyway counts and moves
    /// nothing.
    #[test]
    fn lanes_that_do_not_fit_are_not_carried() {
        let predecessor = stage(at_rest(), 1);
        let other_layout = stage(at_rest(), 2);
        let mut successor = stage(at_rest(), 1);
        assert!(successor.can_carry_lane_from(0, &predecessor, 3, 16));
        assert!(
            !successor.can_carry_lane_from(0, &other_layout, 3, 16),
            "layout version"
        );
        assert!(
            !successor.can_carry_lane_from(LANES, &predecessor, 0, 16),
            "successor lane"
        );
        assert!(
            !successor.can_carry_lane_from(0, &predecessor, LANES, 16),
            "predecessor lane"
        );
        assert!(
            !successor.can_carry_lane_from(0, &predecessor, 3, 15),
            "scratch"
        );
        let mut short = [0_u8; 15];
        assert!(!successor.carry_lane_from(0, &predecessor, 3, &mut short));
        assert!(!successor.carry_lane_from(0, &other_layout, 3, &mut [0_u8; 16]));
        assert_eq!(successor.carry_refused(), 2);
        assert_eq!(running(&mut successor), [1, 2, 3, 4]);
    }
}
