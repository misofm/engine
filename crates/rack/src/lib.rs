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
    BankWidth, BypassShunt, ChannelSymmetryWitness, EffectBankProcessBlock, EffectControlLane,
    EffectProgramKey, ObservationLane, ObservationSample, PreparedAutomationSpan,
    PreparedNativeEffectBank, PreparedSidechainPort, SeamSide, transpose_tile_4, transpose_tile_8,
};
use engine::realtime::RenderError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RackError {
    ZeroQuantum,
    Overflow,
    Shape,
    WidthMismatch,
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

enum BankPlaneViewsInner<'a> {
    Four([BankPlanePair<'a>; 4], usize),
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

    pub fn from_eight(pairs: [BankPlanePair<'a>; 8], frames: usize) -> Option<Self> {
        Self::capacity(&pairs, frames).then_some(Self(BankPlaneViewsInner::Eight(pairs, frames)))
    }

    #[inline(always)]
    fn supports(&self, width: usize, frames: usize) -> bool {
        matches!(&self.0, BankPlaneViewsInner::Four(_, capacity) if width == 4 && *capacity >= frames)
            || matches!(&self.0, BankPlaneViewsInner::Eight(_, capacity) if width == 8 && *capacity >= frames)
    }
}

/// One stage of a bank chain.
///
/// This is the seam #86 (builtin banks) and #98 (graph bank execution) implement against: a stage
/// owns its prepared processor and sees only the resident block. Shape is infallible here - it was
/// validated once at prepare - so `Err` is the stage's own render failure.
pub trait BankStage: Send {
    fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError>;
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
    /// Drop every subscription this stage carries (issue #143 D7).
    fn disarm_observations(&mut self) {}

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

    /// Drain this stage's live-console queues, before any lane of the block is dispatched.
    ///
    /// # Why the drain is a separate call and not the first paragraph of [`process`](Self::process)
    ///
    /// An admitted record takes effect on the **first sample of the block that drains it** -- that
    /// is #137 E1's rule and every console gate in the tree rests on it. A record that writes one
    /// channel's upstream word also clears the channel-symmetry witness' `LIVE` term. Those two
    /// facts have to be observed in that order: if the collapse dispatch read the witness *before*
    /// the drain, a `ParameterChannel::Left` retarget admitted at block `N` would take effect on
    /// block `N`'s first sample while block `N` still ran collapsed -- and a collapsed block
    /// publishes the left plane on both channels, so the right channel would receive a retarget
    /// addressed to the left one. The bits would be wrong, in the audible direction, on the one
    /// block nobody would think to look at.
    ///
    /// So [`BankChain::run`] drains **every** slot first, then reads the witness, then gathers. The
    /// default is a no-op, which is what a console-free plan pays.
    fn begin_block(&mut self, first_sample: u64) -> Result<(), RenderError> {
        let _ = first_sample;
        Ok(())
    }

    /// Which side of the fader/matrix seam this stage sits on.
    ///
    /// The default is [`SeamSide::UpstreamOfSeam`], which is the **conservative** answer here and
    /// not the permissive one: an upstream stage is one a collapsed chain would have to run
    /// one-plane, so a stage that has not spoken is one the collapse must be able to run through --
    /// and it cannot, because [`supports_mono_collapse`](Self::supports_mono_collapse) defaults to
    /// `false`. An unclassified stage therefore declines the whole chain rather than being silently
    /// treated as seam-side and left to read a plane nobody gathered.
    fn seam_side(&self) -> SeamSide {
        SeamSide::UpstreamOfSeam
    }

    /// Whether this stage implements [`process_mono`](Self::process_mono) and
    /// [`desymmetrize`](Self::desymmetrize).
    ///
    /// Decided off the render thread, at bind, and cached by [`BankChain::new`]: a chain whose
    /// upstream prefix contains one stage that answers `false` can never collapse.
    fn supports_mono_collapse(&self) -> bool {
        false
    }

    /// Render one block with the cohort's two channels collapsed onto `block.left`.
    ///
    /// `block.right` is the resident scratch the chain did **not** gather. It holds the previous
    /// block's words and reading it is a defect; the chain overwrites it with the left plane at the
    /// seam, before the first seam-side slot and before the scatter.
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
    ///   [`ConsoleEffectBankStage::process_mono`] -- because the right channel of a collapsed track
    ///   *is* its left channel at the tap exactly as at the fader.
    /// * **Reports and counters.** Per-channel accounting is duplicated from the left, and a total
    ///   that sums both channels is twice the left count. See
    ///   [`PreparedNativeEffectBank::process_bank_mono`], which states the rule for the effects,
    ///   and `builtins/tests/mono_collapse.rs`, which is the worked gate on it.
    /// * **Latency lines.** Anything a stage stages for a *later* block -- a dry shunt's delay line
    ///   is the one in this tree -- must be fed the left plane rather than the ungathered right
    ///   scratch, because the seam is downstream of the line and cannot repair it.
    ///   `ConsoleEffectBankStage::process_inner` carries that argument in full.
    ///
    /// The common shape of all three is that the collapse is invisible to samples and visible to
    /// state, so a stage whose only mono gate is a digest has not been gated.
    fn process_mono(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
        self.process(block)
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
}

/// Adapter from the effect contract's prepared homogeneous bank to a chain stage.
///
/// Width, quantum and the automation offsets are fixed here once (#96 F8): the render path calls
/// no `metadata()` and performs no `checked_add`.
pub struct EffectBankStage {
    processor: Box<dyn PreparedNativeEffectBank>,
    width: BankWidth,
    quantum: u32,
    offsets: Box<[u32]>,
    /// One flag per lane: the effect's own designed-word comparison, taken once at bind.
    ///
    /// # Why this is cached and why the cache cannot go stale
    ///
    /// `PreparedNativeEffectBank::lane_channel_symmetry` is a walk over every designed word the
    /// kernel reads -- eight coefficients and five four-field ramps per lane for the compressor,
    /// twenty-six words for the input chain. Pulling it once per lane per slot per block, which is
    /// what the collapse's dispatch needs, would cost more than the collapse saves.
    ///
    /// The cache is sound because the designed words of a **bound** bank cannot move: a
    /// console-free slot hands the bank an empty automation slice on every block
    /// (`EffectBankStage::process`), and `reset`/`restore_track_state_payload` -- the two calls
    /// that do move them -- are preparation-side and unreachable once a processor has been moved
    /// into a chain. `ConsoleEffectBankStage` is the slot that *does* drain writes, and it caches
    /// the same half for the same reason: a one-channel write clears the `LIVE` term, which that
    /// slot pulls live, and a `Both` write leaves the two channels bit-equal.
    designed: Box<[bool]>,
}

impl EffectBankStage {
    /// Errors with [`RackError::WidthMismatch`] if the prepared processor is not this width.
    pub fn new(
        processor: Box<dyn PreparedNativeEffectBank>,
        width: BankWidth,
        quantum: u32,
    ) -> Result<Self, RackError> {
        if processor.metadata().width != width {
            return Err(RackError::WidthMismatch);
        }
        if quantum == 0 {
            return Err(RackError::ZeroQuantum);
        }
        let designed = (0..width.lanes() as usize)
            .map(|lane| processor.lane_channel_symmetry(lane))
            .collect();
        Ok(Self {
            processor,
            width,
            quantum,
            offsets: vec![0_u32; width.lanes() as usize + 1].into_boxed_slice(),
            designed,
        })
    }
}

impl BankStage for EffectBankStage {
    /// A console-free bank has no live channel at all, so the two live terms cannot be false and
    /// the whole witness is the effect's own designed-word comparison.
    fn lane_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
        witness_of_designed(self.designed.get(lane).copied().unwrap_or(false))
    }

    /// Every prepared effect sits in `simd1`, `dynamic` or `simd2`, all three upstream of the
    /// fader (`TrackStage` order), so an effect slot is never seam-side.
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

/// The live-console twin of [`EffectBankStage`] (issue #140 A).
///
/// It is a **separate stage type** on purpose, exactly as `ConsoleMatrixProcessor` is a separate
/// processor from `MatrixProcessor` (#137 D1): a bank prepared without a console keeps
/// [`EffectBankStage`]'s storage and its `&[]`/zero-offset call, byte for byte, so "control off
/// costs nothing" stays an identity rather than a claim. Nothing in this type is reachable from a
/// session that asked for no control channel.
///
/// # What one block does, in order
///
/// 1. Drain every lane's bounded queue into that lane's window of the packed span array, in
///    canonical order (`EffectControlLane::stage`). This happens **before** any audio is touched,
///    so an admitted record takes effect on the first sample of this block -- the same
///    block-boundary rule the matrix stage proved for #137's E1.
/// 2. Capture the dry AoSoA block into the latency shunt, if any lane can be bypassed.
/// 3. Run the bank once with the packed spans and the per-lane offsets the effect contract
///    already defines (`EffectBankProcessBlock::automation_offsets`).
/// 4. Restore the latency-matched dry signal into exactly the bypassed lanes.
///
/// # Partition invariance and `to_bits` identity extend to command timelines
///
/// The spans a lane receives are a pure function of that lane's own queue and the block's
/// `first_sample`, and every lane is staged independently into a disjoint window. A bank therefore
/// applies a command timeline to lane `l` exactly as the per-node scalar path applies it to the
/// same effect: same spans, same block, same order.
/// One bank lane's witness from the effect's own designed-word comparison alone.
///
/// The other four terms are left set: a bank slot speaks only to `DESIGNED`, and `SOURCE`,
/// `RESTORED` and the two live terms are conjoined by whoever owns them. Leaving them set is what
/// makes the conjunction in [`BankChain::lane_symmetry`] mean "every stage agreed" rather than
/// "every stage claimed everything".
pub fn designed_lane_witness(
    processor: &dyn PreparedNativeEffectBank,
    lane: usize,
) -> ChannelSymmetryWitness {
    witness_of_designed(processor.lane_channel_symmetry(lane))
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

pub struct ConsoleEffectBankStage {
    processor: Box<dyn PreparedNativeEffectBank>,
    width: BankWidth,
    quantum: u32,
    /// `lanes + 1` packed offsets into [`Self::spans`], rewritten every block.
    offsets: Box<[u32]>,
    /// One control channel per lane; `None` for a lane no console addresses.
    lanes: Box<[Option<EffectControlLane>]>,
    /// One lane's staging window: the bank's own `automation_capacity` spans.
    ///
    /// One window serves every lane because a lane's staged prefix is copied into [`Self::packed`]
    /// the moment it is drained, before the next lane touches the window. The bank never sees this
    /// array; it sees `packed[..offsets[lanes]]`.
    staging: Box<[PreparedAutomationSpan]>,
    /// The packed, per-lane-partitioned span array the bank is handed. Lane `l` owns
    /// `[offsets[l], offsets[l + 1])`, which is the partition the effect contract already defines.
    packed: Box<[PreparedAutomationSpan]>,
    /// Latency-preserving dry shunt over the resident AoSoA block, or `None` when no lane of this
    /// slot can be bypassed live.
    shunt: Option<BypassShunt>,
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
    /// [`ConsoleEffectBankStage::drain`], read by the process body that follows it in the same
    /// block.
    staged_spans: usize,
    /// Records dropped because a lane's window was full of distinct targets. Zero by construction.
    dropped: u64,
    /// `Observe` records this slot had no capacity to apply. Zero by construction.
    unbound: u64,
}

impl ConsoleEffectBankStage {
    /// Builds the console stage for one bound bank slot.
    ///
    /// `latency` is the slot's declared [`effect_contract::PreparedEffectMetadata::latency`]. Every lane of a bank
    /// shares one [`EffectProgramKey`], so they share one latency and one AoSoA delay line:
    /// delaying the interleaved plane by `latency * lanes` words delays every lane by exactly
    /// `latency` frames.
    ///
    /// # Errors
    ///
    /// [`RackError::WidthMismatch`] if the processor is not this width or the lane count disagrees,
    /// and [`RackError::ZeroQuantum`] for a zero quantum.
    pub fn new(
        processor: Box<dyn PreparedNativeEffectBank>,
        width: BankWidth,
        quantum: u32,
        lanes: Vec<Option<EffectControlLane>>,
        observations: Vec<Option<ObservationLane>>,
        latency: usize,
    ) -> Result<Self, RackError> {
        if processor.metadata().width != width
            || lanes.len() != width.lanes() as usize
            || observations.len() != width.lanes() as usize
        {
            return Err(RackError::WidthMismatch);
        }
        if quantum == 0 {
            return Err(RackError::ZeroQuantum);
        }
        let lane_count = width.lanes() as usize;
        let capacity = processor.metadata().program_key.automation_capacity as usize;
        let total = capacity
            .checked_mul(lane_count)
            .ok_or(RackError::Overflow)?;

        let words = (quantum as usize)
            .checked_mul(lane_count)
            .ok_or(RackError::Overflow)?;
        let line = latency.checked_mul(lane_count).ok_or(RackError::Overflow)?;
        let shunt = lanes
            .iter()
            .any(Option::is_some)
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
        Ok(Self {
            processor,
            designed,
            width,
            quantum,
            offsets: vec![0_u32; lane_count + 1].into_boxed_slice(),
            lanes: lanes.into_boxed_slice(),
            staging: vec![IDLE_SPAN; capacity].into_boxed_slice(),
            packed: vec![IDLE_SPAN; total].into_boxed_slice(),
            shunt,
            observations: observed.then(|| observations.into_boxed_slice()),
            samples,
            staged_spans: 0,
            dropped: 0,
            unbound: 0,
        })
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

    /// Records refused because a lane's staging window was full. Read only off the render thread.
    #[must_use]
    pub const fn dropped_records(&self) -> u64 {
        self.dropped
    }
}

/// The value an unused staging slot holds. It is never handed to a bank: only `[..offsets[lanes]]`
/// is, and every span inside that range was written by this block's drain.
const IDLE_SPAN: PreparedAutomationSpan = PreparedAutomationSpan {
    kind: effect_contract::AutomationSpanKind::Point,
    channel: effect_contract::ParameterChannel::Both,
    parameter_index: 0,
    start_sample: 0,
    end_sample: 0,
    start_value: 0.0,
    end_value: 0.0,
};

impl BankStage for ConsoleEffectBankStage {
    /// The designed-word comparison, conjoined with the lane's own live terms.
    ///
    /// The live half comes from [`EffectControlLane::symmetry`], which the drain in
    /// [`Self::process`] maintains: a lane with no console channel has no live writes at all and
    /// contributes only the designed term.
    fn lane_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
        let designed = witness_of_designed(self.designed.get(lane).copied().unwrap_or(false));
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

    fn begin_block(&mut self, first_sample: u64) -> Result<(), RenderError> {
        self.drain(first_sample);
        Ok(())
    }

    // REALTIME_POLICY_BEGIN
    /// [`ConsoleEffectBankStage::process`] with the bank call collapsed onto the left plane.
    ///
    /// Every other step is the dual one, in the same order and on the same words: the drain runs
    /// before a sample is touched, the shunt captures and restores both planes, and the taps
    /// publish after the bank ran. The shunt's right plane is the ungathered scratch, which is
    /// sound for exactly one reason and it is worth stating: the seam overwrites the right plane
    /// after this slot, so nothing downstream ever reads what the shunt put there. The *left*
    /// restore is the one that matters and it is the dual one.
    ///
    /// A live bypass on any lane clears the witness' `UNBYPASSED` term, so a collapsed cohort has
    /// no bypassed lane and the restore loop is a no-op whenever this body runs at all. It is kept
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
        ConsoleEffectBankStage::observation_retained_bytes(self)
    }

    fn disarm_observations(&mut self) {
        ConsoleEffectBankStage::disarm_observations(self);
    }

    // REALTIME_POLICY_BEGIN
    fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
        self.process_inner::<false>(block)
    }
    // REALTIME_POLICY_END
}

impl ConsoleEffectBankStage {
    /// Step 1 of the block, hoisted out of [`Self::process_inner`] so that it runs before the
    /// chain's collapse dispatch reads the witness. See [`BankStage::begin_block`].
    fn drain(&mut self, first_sample: u64) {
        let lane_count = self.width.lanes() as usize;
        let mut packed = 0_usize;
        self.offsets[0] = 0;
        for lane in 0..lane_count {
            if let Some(channel) = self.lanes[lane].as_mut() {
                let observation = self
                    .observations
                    .as_deref_mut()
                    .and_then(|lanes| lanes.get_mut(lane))
                    .and_then(Option::as_mut);
                let staged = channel.stage(&mut self.staging, first_sample, observation);
                self.dropped = self.dropped.saturating_add(u64::from(staged.dropped));
                self.unbound = self.unbound.saturating_add(u64::from(staged.unbound));
                // Packed at this lane's own offset, immediately: that offset is what makes the
                // window reusable and what gives the lane its private partition of `packed`.
                self.packed[packed..packed + staged.staged]
                    .copy_from_slice(&self.staging[..staged.staged]);
                packed += staged.staged;
            }
            self.offsets[lane + 1] = packed as u32;
        }
        self.staged_spans = packed;
    }

    // REALTIME_POLICY_BEGIN
    /// The one console-slot body, dual or collapsed.
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
        // Issue #163 phase 4 item 4: which lanes, if any, are bypassed this block. Decided here
        // because it gates both the capture below and the restore in step 3, and because the
        // control drain in step 1 has already run — so this is the same verdict step 3 reaches,
        // read once instead of twice.
        //
        // `shunt.is_some()` means *some lane of this cohort has a control channel*, not that any
        // lane is bypassed. An eight-lane cohort with one console-driven lane and nothing
        // bypassed was paying a `quantum * lanes` two-plane copy every block for a dry block no
        // reader could observe.
        let any_bypassed = self.lanes[..lane_count]
            .iter()
            .any(|lane| lane.as_ref().is_some_and(EffectControlLane::bypassed));
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
        if let Some(shunt) = self
            .shunt
            .as_mut()
            .filter(|shunt| any_bypassed || shunt.feeds_line())
        {
            if MONO {
                shunt.capture(block.left, block.left);
            } else {
                shunt.capture(block.left, block.right);
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
        // 3. Latency-matched dry restore for exactly the bypassed lanes. `any_bypassed` is the
        //    disjunction of the per-lane test below, decided before the capture; when it is false
        //    this whole loop was already a no-op, and skipping it also proves the capture above
        //    had no reader.
        if let Some(shunt) = self.shunt.as_ref().filter(|_| any_bypassed) {
            let (dry_left, dry_right) = shunt.dry();
            for lane in 0..lane_count {
                if !self.lanes[lane]
                    .as_ref()
                    .is_some_and(EffectControlLane::bypassed)
                {
                    continue;
                }
                let mut index = lane;
                let words = frames as usize * lane_count;
                while index < words {
                    block.left[index] = dry_left[index];
                    block.right[index] = dry_right[index];
                    index += lane_count;
                }
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
        let required = lane_ids
            .iter()
            .try_fold(0usize, |required, lane| {
                lane.checked_mul(stride)
                    .and_then(|start| start.checked_add(frames))
                    .map(|end| required.max(end))
            })
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
    #[must_use]
    pub fn left_mut(&mut self) -> &mut [f32] {
        self.left
    }
    #[must_use]
    pub fn right_mut(&mut self) -> &mut [f32] {
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
        if cohort.lane_ids.is_empty() || cohort.lane_ids.len() > 8 {
            return;
        }
        let Some(max_lane) = cohort.lane_ids.iter().copied().max() else {
            return;
        };
        if cohort
            .lane_ids
            .iter()
            .enumerate()
            .any(|(index, lane)| cohort.lane_ids[..index].contains(lane))
        {
            return;
        }
        let Some(required) = max_lane
            .checked_add(1)
            .and_then(|lanes| lanes.checked_mul(cohort.stride))
        else {
            return;
        };
        if cohort.stride < cohort.frames
            || cohort.left.len() < required
            || cohort.right.len() < required
        {
            return;
        }
        for lane in cohort.lane_ids.iter().copied() {
            let start = lane * cohort.stride;
            self.fold_plane(
                lane,
                &mut cohort.left[start..start + cohort.frames],
                &mut cohort.right[start..start + cohort.frames],
            );
        }
    }
}

/// One bank chain: a resident L/R AoSoA block plus its ordered slots.
///
/// Exactly one gather and one scatter per [`run`](BankChain::run), whatever the slot count
/// (master plan §4.5).
///
/// # The mono collapse
///
/// A chain whose every active lane renders a track doing bit-identical work on both channels runs
/// its **upstream** slots over one plane and duplicates that plane into its seam-side slots. The
/// whole mechanism is four decisions and one copy:
///
/// 1. **Bind time.** `BankChain::collapse_prefix_of` works out how many
///    leading slots a collapsed block would run one-plane, and answers `0` -- never collapse --
///    unless the seam-side slots are a suffix, the prefix is non-empty, every prefix slot has
///    written a one-plane body, and every slot runs on exactly this chain's lanes.
/// 2. **Bind time, again.** [`arm_mono_collapse`](BankChain::arm_mono_collapse) records the
///    *structural* half of the witness, which a chain cannot see for itself. Unarmed is declining.
/// 3. **Per block, before a sample moves.** Every slot's live-console queue is drained
///    ([`BankStage::begin_block`]) and *then* the witness is read, in that order and for the
///    reason `begin_block` documents.
/// 4. **The seam.** After the prefix, one copy of the resident block's left plane into its right.
///    Everything downstream -- the fader, the matrix, the scatter, the route fold, the master
///    accumulation -- then runs the arithmetic it runs on a dual block, in its own operation order.
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
///   back ([`BankStage::channels_agree`]).
///
/// So the reachable cycle -- collapse, disengage on a forced-off arm or a lifted-then-restored
/// bypass, re-engage when the switch or the term comes back -- is a cycle the chain takes, and the
/// one transition M2 could not justify is now justified by an invariant rather than by an argument
/// about why the witness went false. Counted in [`collapse_transitions`](Self::collapse_transitions).
pub struct BankChain {
    scratch: AoSoaScratch,
    lanes: usize,
    active: Box<[bool]>,
    slots: Box<[BankSlot]>,
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
    /// `Coming back` section for the four clauses that maintain it; the awkward "next *dual*
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
    /// `[disengages, re-engages, agreement proofs]`. Evidence only; see
    /// [`collapse_transitions`](Self::collapse_transitions) for what each one counts and why a
    /// block count alone cannot say it.
    transitions: [u64; 3],
}

impl BankChain {
    /// Validates the whole shape once, off the render thread: `active` and every slot mask have
    /// exactly `lanes` entries, a slot may only be active on an active lane, and at least one lane
    /// is active. Scratch lanes start at `+0.0`, so an inactive lane can never hand stale garbage
    /// to the block boundary check.
    pub fn new(
        mut scratch: AoSoaScratch,
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
        scratch.left.fill(0.0);
        scratch.right.fill(0.0);
        // The tiled round trip needs every lane's planar view, and its scatter fully overwrites
        // every lane's planar buffer. Both are only true of a full bank, so a partial one keeps
        // the per-lane scalar path: that is what preserves the invariant above, that an inactive
        // lane is neither read from nor written to and can never hand stale garbage across the
        // block boundary. The standing 64-track fixture is all-full banks; the nine-track ragged
        // fixture is what exercises the fallback.
        let full_bank = active.iter().all(|lane| *lane);
        let staging = if full_bank { scratch.left.len() } else { 0 };
        let collapse_prefix = Self::collapse_prefix_of(&slots, &active);
        Ok(Self {
            scratch,
            lanes,
            active,
            slots: slots.into_boxed_slice(),
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
            transitions: [0; 3],
        })
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
    /// 3. **Every prefix slot has a one-plane body.** A dual body handed the ungathered right
    ///    plane would not merely write garbage into a plane the seam overwrites -- a linked
    ///    detector reads *both* planes to compute the level it applies to the left one, so the
    ///    left output would be wrong too.
    /// 4. **Every slot runs on exactly this chain's active lanes.** The collapse is
    ///    all-lanes-or-nothing and the witness is conjoined over the slots a lane runs; a slot that
    ///    is an identity on some lanes would make "every active lane is eligible" and "every slot
    ///    agreed for every active lane" two different statements.
    fn collapse_prefix_of(slots: &[BankSlot], active: &[bool]) -> usize {
        let prefix = slots
            .iter()
            .take_while(|slot| slot.stage.seam_side() == SeamSide::UpstreamOfSeam)
            .count();
        let seam_side_is_a_suffix = slots[prefix..]
            .iter()
            .all(|slot| slot.stage.seam_side() == SeamSide::SeamSide);
        let prefix_is_collapsible = slots[..prefix]
            .iter()
            .all(|slot| slot.stage.supports_mono_collapse());
        let lanes_agree = slots
            .iter()
            .all(|slot| slot.active_lanes.as_ref() == active);
        if prefix > 0 && seam_side_is_a_suffix && prefix_is_collapsible && lanes_agree {
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
    /// # Why this is a pull and not a stored aggregate
    ///
    /// The terms themselves are event-maintained -- a slot's live terms move only when a record is
    /// drained, and its designed term only when the plan is rebuilt -- so the aggregate is a
    /// conjunction of at most `slots` already-computed values. That is the cost the collapse
    /// design priced as "an AND over eight lane witnesses, nanoseconds", and it is why nothing
    /// here is cached: a cache would add a per-block invalidation check to buy back an `and` over
    /// four bools.
    ///
    /// What *is* cached is one level down, and it has to be: `lane_channel_symmetry` is a walk over
    /// every designed word a kernel reads, and [`BankChain::run`] pulls this once per lane per slot
    /// per block. `EffectBankStage::designed` and its console twin take that comparison once, at
    /// bind, where it is fixed for the life of the bound bank; the live terms stay live, because
    /// they are the ones a drain moves. So the per-block cost of the whole witness is an `and` over
    /// `slots * lanes` cached flags plus one stored byte per console-driven lane.
    ///
    /// The **input** bank is the third holder of that cache and the only one whose words move
    /// within a plan, because #210 phase 3 made its trim and polarity words live. It keeps the
    /// same shape anyway -- the builtins crate's `InputStage::symmetry` -- maintained by the five
    /// writers of a compared word rather than fixed at bind. Until #235 it alone re-derived its
    /// walk on every pull, which is what made this paragraph a description of two thirds of the
    /// tree rather than of all of it.
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
            if slot.active_lanes.get(lane).copied().unwrap_or(false) {
                witness = witness.and(slot.stage.lane_symmetry(lane));
            }
        }
        witness
    }

    /// One flag per **active** lane, in lane order: does that lane's whole witness hold?
    ///
    /// The localisable form of [`symmetry_counters`](Self::symmetry_counters), and the form the
    /// plan surface publishes: a count says how many lanes lost a term and never which, while
    /// "exactly that track and no other" is the claim the witness tests actually make. Inactive
    /// lanes are skipped rather than reported false, so the result indexes the same way a bank's
    /// member list does.
    #[must_use]
    pub fn active_lane_eligibility(&self) -> Vec<bool> {
        (0..self.lanes)
            .filter(|lane| self.active[*lane])
            .map(|lane| self.lane_symmetry(lane).eligible())
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
        (0..self.lanes)
            .filter(|lane| self.active[*lane])
            .all(|lane| self.lane_symmetry(lane).eligible())
    }

    /// Whether a **dual** block rendered under this cohort's witness leaves every active lane's two
    /// channels agreeing (mono-collapse M3).
    ///
    /// Strictly weaker than [`all_lanes_symmetric`](Self::all_lanes_symmetric), by exactly the
    /// `UNBYPASSED` term: see [`ChannelSymmetryWitness::AGREEING`] for why a bypass window is the
    /// one way to lose the witness without moving the two channels apart.
    ///
    /// Short-circuits, like its sibling, and [`run`](Self::run) reaches it only when the answer can
    /// change something -- see the guard chain there, which is what keeps this off the steady-state
    /// path of every session that is not in the middle of a bypass.
    fn all_lanes_preserve_agreement(&self) -> bool {
        (0..self.lanes)
            .filter(|lane| self.active[*lane])
            .all(|lane| self.lane_symmetry(lane).preserves_channel_agreement())
    }

    /// `[eligible active lanes, active lanes]` for this chain. Evidence and gates only.
    #[must_use]
    pub fn symmetry_counters(&self) -> [u64; 2] {
        let mut counters = [0_u64; 2];
        for lane in 0..self.lanes {
            if !self.active[lane] {
                continue;
            }
            counters[1] += 1;
            if self.lane_symmetry(lane).eligible() {
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
        if !lanes.iter().any(|lane| *lane) {
            self.fold = Box::default();
            return Ok(());
        }
        if self.staging_left.len() != self.scratch.left.len() {
            self.staging_left = vec![0.0; self.scratch.left.len()].into_boxed_slice();
            self.staging_right = vec![0.0; self.scratch.right.len()].into_boxed_slice();
        }
        self.fold = lanes;
        Ok(())
    }

    /// Lanes whose scatter this chain folds; empty when it folds none. Evidence only.
    #[must_use]
    pub fn fold_lanes(&self) -> &[bool] {
        &self.fold
    }

    // REALTIME_POLICY_BEGIN
    /// Gather every active lane, run every non-identity slot over the resident block, scatter every
    /// active lane back. No allocation, no shape `Result`, one transpose round-trip.
    pub fn run<M: BankMembers + ?Sized>(
        &mut self,
        members: &mut M,
        frames: u32,
        first_sample: u64,
    ) -> Result<(), RenderError> {
        debug_assert!(frames >= 1 && frames <= self.scratch.quantum);
        let len = frames as usize * self.lanes;
        // Step 0: every slot's live-console queue, drained before anything else. See
        // `BankStage::begin_block` for why this cannot be folded into `process`.
        for slot in &mut self.slots {
            // The same identity-slot guard `process` has always taken. A slot that is an identity
            // on every lane renders nothing, and draining its queues would consume records for a
            // block it does not run.
            if slot.active_lanes.iter().any(|lane| *lane) {
                slot.stage.begin_block(first_sample)?;
            }
        }
        // The dispatch. One `bool` per block per chain, decided before a sample is gathered, from
        // the event-maintained witness the slots already carry: `all_lanes_symmetric` is an `and`
        // over `slots * lanes` cached flags plus, for a console slot, one stored byte per lane.
        //
        // A command lands at a block boundary, so the eligibility this reads is the eligibility
        // that holds for every sample of this block -- which is what makes a per-block mode legal
        // at all.
        //
        // M3 adds one term to the M2 dispatch and no work to it: `collapse_channels_agree` is the
        // premise the witness does not supply, maintained at the bottom of this section.
        //
        // That sentence was **false as shipped** and is true again (#235). M3 hoisted the witness
        // out of the M2 conjunction's short-circuit into an unconditional per-block pull, which
        // reached an input-bank walk nothing cached: +39-42% on the dispatch-dominated rows and
        // +4.5% on the forced-off arm, on chains that could never collapse at all. Both halves
        // are repaired here -- `armed &&` below restores the short-circuit, and
        // `InputStage::symmetry` gives the input bank the caching the first paragraph above always
        // claimed for it -- and the second half also retires ~2 us the *eligible* arm had been
        // paying since M2, which no short-circuit can reach because that arm's walk is the one the
        // dispatch genuinely needs.
        let armed = self.collapse_prefix > 0 && self.collapse_source && !self.collapse_forced_off;
        // `armed &&` is M2's short-circuit, restored (#235). The walk runs only where its answer
        // can change something, and the `false` it substitutes elsewhere is not an approximation:
        //
        // * the recovery-window proof and the collapse decision below are both
        //   `armed && witness && ..`, so an unarmed chain's witness cannot reach either;
        // * the invariant's maintenance step is `.. && self.can_collapse()
        //   && self.collapse_channels_agree && !witness && !self.all_lanes_preserve_agreement()`.
        //   An unarmed chain that can collapse at all is the forced-off arm, and there the
        //   substituted `false` opens the `!witness` clause and hands the question to
        //   `all_lanes_preserve_agreement`. That is the *same verdict*, because `AGREEING` is a
        //   subset of `ALL`: eligible implies preserving, so `!eligible && !preserving` and
        //   `!preserving` clear the flag on exactly the same blocks. The arm pays one walk either
        //   way, and the invariant is maintained on it exactly as M3 wrote it.
        //
        // So a chain with no collapsible prefix and a stereo-source chain -- every session M2 left
        // alone -- take no walk at all, which is what the guard chain below claims and what the
        // hoist had stopped being true.
        let witness = armed && self.all_lanes_symmetric();
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
        // * a block whose witness was **eligible** is skipped, because eligible implies preserving
        //   -- `AGREEING` is a subset of `ALL` -- so the second walk would be asking a question the
        //   first already answered. Since #235 restored M2's short-circuit this clause is the one
        //   the *armed* arm takes; the forced-off arm reaches here with `witness == false` because
        //   the eligible walk was never taken, and answers the same question once through
        //   `all_lanes_preserve_agreement` instead. Same verdict, same one walk -- see the
        //   short-circuit's own note above.
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
            && !self.all_lanes_preserve_agreement()
        {
            self.collapse_channels_agree = false;
        }
        if collapse {
            self.collapses = self.collapses.saturating_add(1);
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
            if slot.active_lanes.iter().any(|lane| *lane) {
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
            // The seam. One extra copy of the resident block per collapsed chain per block, and it
            // is the whole of the duplication: everything from here on -- the fader, the matrix,
            // the scatter, the route fold, the master accumulation -- reads two planes and runs
            // exactly the arithmetic it runs on a dual block, in exactly its operation order. The
            // fold's `ll*l + lr*r` is *not* rewritten as `(ll + lr) * l` on the strength of
            // `l == r`: the two round differently, and the second is not what a dual run computes.
            let AoSoaScratch { left, right, .. } = &mut self.scratch;
            right[..len].copy_from_slice(&left[..len]);
            for slot in &mut self.slots[self.collapse_prefix..] {
                if slot.active_lanes.iter().any(|lane| *lane) {
                    slot.stage.process(BankBlock {
                        left: &mut self.scratch.left[..len],
                        right: &mut self.scratch.right[..len],
                        frames,
                        first_sample,
                        lanes: self.lanes,
                    })?;
                }
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
    // REALTIME_POLICY_END

    /// The disengage boundary: restore every collapsed stage's right-channel state and retire.
    ///
    /// A collapsed run evolves one channel. The counterfactual dual run's right state is, by the
    /// induction the witness maintains, the left one -- so copying it is not an approximation of
    /// the dual run, it **is** the dual run's state, and the first dual block after this renders
    /// exactly what a never-collapsed run would have.
    ///
    /// Only the prefix stages are copied, because only they ran one-plane: a seam-side slot ran
    /// dual on every block, collapsed or not, and its two channels are already whatever the strip
    /// asked for.
    ///
    /// This is a one-off of a few hundred kilobytes -- the compressor's two rings and the limiter's
    /// four, per cohort -- on the block that stops collapsing, and it happens at most once per
    /// chain per plan.
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
    fn gather_mono<M: BankMembers + ?Sized>(&mut self, members: &M, frames: u32) {
        if self.full_bank {
            match self.scratch.width {
                BankWidth::Four => {
                    self.gather_mono_tiled::<M, 4>(members, frames, transpose_tile_4)
                }
                BankWidth::Eight => {
                    self.gather_mono_tiled::<M, 8>(members, frames, transpose_tile_8);
                }
            }
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
    /// one source channel". Passing `false`, or never calling this at all, makes the chain decline
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
    /// Evidence only.
    #[must_use]
    pub const fn collapse_channels_agree(&self) -> bool {
        self.collapse_channels_agree
    }

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
            match self.scratch.width {
                BankWidth::Four => self.gather_tiled::<M, 4>(members, frames, transpose_tile_4),
                BankWidth::Eight => self.gather_tiled::<M, 8>(members, frames, transpose_tile_8),
            }
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
    /// costs a partial bank; the tiled path already lands there.
    fn scatter<M: BankMembers + ?Sized>(&mut self, members: &mut M, frames: u32) {
        if self.full_bank {
            match self.scratch.width {
                BankWidth::Four => self.scatter_tiled::<M, 4>(members, frames, transpose_tile_4),
                BankWidth::Eight => self.scatter_tiled::<M, 8>(members, frames, transpose_tile_8),
            }
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
                    BankPlaneViewsInner::Eight(pairs, _) => {
                        tile_scatter_direct_plane(
                            &self.scratch.left,
                            pairs,
                            frames_used,
                            false,
                            transpose_tile_8,
                        );
                        tile_scatter_direct_plane(
                            &self.scratch.right,
                            pairs,
                            frames_used,
                            true,
                            transpose_tile_8,
                        );
                    }
                }
                return;
            }
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
    /// Completed planar/AoSoA round-trips: exactly one per rendered block per chain.
    #[must_use]
    pub const fn transposes(&self) -> u64 {
        self.transposes
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

    /// Drop every subscription every slot of this chain carries (issue #143 D7).
    pub fn disarm_observations(&mut self) {
        for slot in self.slots.iter_mut() {
            slot.stage.disarm_observations();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct PassThrough;
    impl BankStage for PassThrough {
        fn process(&mut self, _block: BankBlock<'_>) -> Result<(), RenderError> {
            Ok(())
        }
    }

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
            if lanes == 4 {
                let mut left = left.iter_mut();
                let mut right = right.iter_mut();
                let pairs: [BankPlanePair<'_>; 4] = std::array::from_fn(|_| {
                    (
                        &mut **left.next().expect("validated lane"),
                        &mut **right.next().expect("validated lane"),
                    )
                });
                BankPlaneViews::from_four(pairs, frames)
            } else {
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

    struct WrongWidthPlanes {
        planes: Planes,
        per_lane_writes: usize,
    }
    impl BankMembers for WrongWidthPlanes {
        fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
            self.planes.plane(lane)
        }
        fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
            self.per_lane_writes += 1;
            self.planes.plane_mut(lane)
        }
        fn distinct_planes_mut(
            &mut self,
            _lanes: usize,
            frames: usize,
        ) -> Option<BankPlaneViews<'_>> {
            let mut left = self.planes.left.iter_mut().take(4);
            let mut right = self.planes.right.iter_mut().take(4);
            let pairs: [BankPlanePair<'_>; 4] = core::array::from_fn(|_| {
                (
                    left.next()
                        .expect("four malicious left lanes")
                        .as_mut_slice(),
                    right
                        .next()
                        .expect("four malicious right lanes")
                        .as_mut_slice(),
                )
            });
            BankPlaneViews::from_four(pairs, frames)
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
        chain.run(&mut members, 11, 0).expect("run");
        assert_eq!(members.per_lane_writes, 0);
        assert_planes_bit_equal(&members.planes, &expected, "direct call sentinel");
    }

    #[test]
    fn wrong_width_provider_is_dropped_before_complete_staged_fallback() {
        let mut state = 0x3994_3995_3996_3997;
        let source = hostile_planes(8, 11, &mut state);
        let expected = Planes {
            left: source.left.clone(),
            right: source.right.clone(),
        };
        let mut malformed = WrongWidthPlanes {
            planes: Planes {
                left: source
                    .left
                    .into_iter()
                    .chain([vec![f32::from_bits(0x7fc0_3998); 11]])
                    .collect(),
                right: source
                    .right
                    .into_iter()
                    .chain([vec![f32::from_bits(0x7fc0_3999); 11]])
                    .collect(),
            },
            per_lane_writes: 0,
        };
        let active = vec![true; 8].into_boxed_slice();
        let mut chain = BankChain::new(
            AoSoaScratch::new(BankWidth::Eight, 11).expect("scratch"),
            active.clone(),
            vec![slot(active.to_vec(), Box::new(PassThrough))],
        )
        .expect("chain");
        chain.run(&mut malformed, 11, 0).expect("staged fallback");
        assert_planes_bit_equal(
            &Planes {
                left: malformed.planes.left[..8].to_vec(),
                right: malformed.planes.right[..8].to_vec(),
            },
            &expected,
            "wrong-width staged fallback",
        );
        assert_eq!(malformed.per_lane_writes, 8);
        assert!(
            malformed.planes.left[8]
                .iter()
                .all(|word| word.to_bits() == 0x7fc0_3998)
        );
        assert!(
            malformed.planes.right[8]
                .iter()
                .all(|word| word.to_bits() == 0x7fc0_3999)
        );
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
        chain.run(&mut malformed, 11, 0).expect("staged fallback");
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
        for width in [BankWidth::Four, BankWidth::Eight] {
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
                chain.run(&mut planes, frames, 0).expect("run");
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
        for width in [BankWidth::Four, BankWidth::Eight] {
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
                    chain.run(&mut planes, frames, 0).expect("run");
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
                    .run(&mut staged, frames, 0)
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
        let lanes = 8_usize;
        let scratch = AoSoaScratch::new(BankWidth::Eight, frames).expect("scratch");
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
        chain.run(&mut absent, frames, 0).expect("unarmed run");
        assert!(
            absent.aux_left.iter().all(|value| *value == 0.5)
                && absent.aux_right.iter().all(|value| *value == -0.5),
            "an unarmed chain must not write an auxiliary destination"
        );

        // (3) Arm lanes 2 and 5, which share one destination.
        let mut armed_mask = vec![false; lanes];
        armed_mask[2] = true;
        armed_mask[5] = true;
        chain
            .arm_aux(armed_mask.clone().into_boxed_slice())
            .expect("arm");
        assert_eq!(chain.aux_lanes(), &armed_mask[..]);
        let mut armed = build(armed_mask.clone());
        chain.run(&mut armed, frames, 0).expect("armed run");

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
            // already held. Assigning instead of accumulating would give 5.0 and 10.0.
            assert_eq!(
                armed.aux_left[frame],
                0.5 + 2.0 + 5.0,
                "frame {frame}: two armed lanes must sum into one destination"
            );
            assert_eq!(
                armed.aux_right[frame],
                -0.5 + 4.0 + 10.0,
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
        let lanes = 8_usize;
        for full in [true, false] {
            let mut active = vec![true; lanes];
            if !full {
                active[6] = false;
                active[7] = false;
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
                    AoSoaScratch::new(BankWidth::Eight, frames).expect("scratch"),
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
            unarmed.run(&mut oracle, frames, 0).expect("unarmed run");
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
            folded.run(&mut armed, frames, 0).expect("folded run");

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

    #[test]
    fn all_active_folded_masks_use_one_cohort_with_physical_lane_ids() {
        let frames = 13_u32;
        for (width, active) in [
            (BankWidth::Four, vec![true, true, true, true]),
            (BankWidth::Eight, vec![true; 8]),
            (
                BankWidth::Eight,
                vec![true, false, true, false, false, true, false, false],
            ),
            (
                BankWidth::Eight,
                vec![false, false, true, false, false, false, false, false],
            ),
        ] {
            let lanes = width.lanes() as usize;
            let build = || PlanesWithFold {
                planes: Planes {
                    left: (0..lanes)
                        .map(|lane| vec![lane as f32 + 1.0; frames as usize])
                        .collect(),
                    right: (0..lanes)
                        .map(|lane| vec![-(lane as f32) - 1.0; frames as usize])
                        .collect(),
                },
                gains: vec![1.0; lanes],
                bus_left: vec![0.0; frames as usize],
                bus_right: vec![0.0; frames as usize],
                taken: Vec::new(),
                trace: Vec::new(),
                cohorts: Vec::new(),
            };
            let chain = || {
                BankChain::new(
                    AoSoaScratch::new(width, frames).expect("scratch"),
                    active.clone().into_boxed_slice(),
                    vec![slot(active.clone(), Box::new(ScaleByLane))],
                )
                .expect("chain")
            };
            let mut members = build();
            let mut default = DefaultFoldProvider(build());
            let mut optimized_chain = chain();
            let mut default_chain = chain();
            optimized_chain
                .arm_fold(active.clone().into_boxed_slice())
                .expect("fold");
            default_chain
                .arm_fold(active.clone().into_boxed_slice())
                .expect("fold");
            for block in 0..2 {
                optimized_chain
                    .run(&mut members, frames, block * u64::from(frames))
                    .expect("run");
                default_chain
                    .run(&mut default, frames, block * u64::from(frames))
                    .expect("default run");
            }
            let ids: Vec<usize> = (0..lanes).filter(|lane| active[*lane]).collect();
            let expected_cohorts = if ids.is_empty() {
                Vec::new()
            } else {
                vec![ids.clone(), ids]
            };
            assert_eq!(members.cohorts, expected_cohorts);
            assert!(members.trace.iter().all(|(kind, _)| *kind == 'f'));
            assert!(
                default.0.cohorts.is_empty(),
                "trait default delegates lane by lane"
            );
            assert_eq!(members.taken, default.0.taken);
            assert_eq!(members.bus_left, default.0.bus_left);
            assert_eq!(members.bus_right, default.0.bus_right);
            assert_eq!(members.planes.left, default.0.planes.left);
            assert_eq!(members.planes.right, default.0.planes.right);
            for (lane, is_active) in active.iter().copied().enumerate() {
                if !is_active {
                    assert!(
                        members.planes.left[lane]
                            .iter()
                            .all(|x| x.to_bits() == (lane as f32 + 1.0).to_bits())
                    );
                    assert!(
                        members.planes.right[lane]
                            .iter()
                            .all(|x| x.to_bits() == (-(lane as f32) - 1.0).to_bits())
                    );
                }
            }
        }
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
        assert!(FoldCohort::new(&[usize::MAX], &mut left, &mut right, 2, 1).is_err());
        assert!(FoldCohort::new(&[0], &mut left[..0], &mut right[..0], 0, 0).is_err());
        assert_eq!(left.map(f32::to_bits), left_before);
        assert_eq!(right.map(f32::to_bits), right_before);

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

    /// The fold refuses a mask it cannot render, exactly as the auxiliary seam does.
    #[test]
    fn arming_a_fold_refuses_an_unrenderable_mask() {
        let frames = 32_u32;
        let lanes = 8_usize;
        let mut active = vec![true; lanes];
        active[7] = false;
        let scratch = AoSoaScratch::new(BankWidth::Eight, frames).expect("scratch");
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
            "folding lane 7, which this chain does not render, is refused"
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
    fn arming_an_auxiliary_destination_refuses_an_unrenderable_mask() {
        let frames = 32_u32;
        let lanes = 8_usize;
        let mut active = vec![true; lanes];
        active[7] = false;
        let scratch = AoSoaScratch::new(BankWidth::Eight, frames).expect("scratch");
        let mut chain = BankChain::new(
            scratch,
            active.clone().into_boxed_slice(),
            vec![slot(active.clone(), Box::new(PassThrough))],
        )
        .expect("chain");
        assert_eq!(
            chain.arm_aux(vec![true; lanes - 1].into_boxed_slice()),
            Err(RackError::Shape),
            "a mask that is not the chain's lane count is refused"
        );
        assert_eq!(
            chain.arm_aux(vec![true; lanes].into_boxed_slice()),
            Err(RackError::Shape),
            "arming lane 7, which this chain does not render, is refused"
        );
        assert!(
            chain.aux_lanes().is_empty(),
            "a refused arm leaves the chain on the zero-cost path"
        );
        chain
            .arm_aux(active.into_boxed_slice())
            .expect("arming exactly the rendered lanes is admitted");
    }

    #[test]
    fn gather_scatter_round_trip_is_bit_exact() {
        let frames = 128_u32;
        let lanes = 8_usize;
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
        let active = vec![true, true, false, true, true, true, false, true];
        let mut chain = BankChain::new(
            AoSoaScratch::new(BankWidth::Eight, frames).expect("scratch"),
            active.clone().into_boxed_slice(),
            vec![slot(active.clone(), Box::new(PassThrough))],
        )
        .expect("chain");
        chain.run(&mut planes, frames, 0).expect("run");
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
        chain.run(&mut planes, frames, 0).expect("run");
    }

    /// T3: a slot that is an identity on every lane is never executed, and the chain still counts
    /// exactly one transpose per block.
    #[test]
    fn identity_everywhere_slot_is_not_executed() {
        struct Counting {
            slot: usize,
            calls: u64,
        }
        impl BankStage for Counting {
            fn process(&mut self, _block: BankBlock<'_>) -> Result<(), RenderError> {
                self.calls += 1;
                Ok(())
            }
            fn qualification_counters(&self) -> [u64; 2] {
                let mut counters = [0, 0];
                counters[self.slot] = self.calls;
                counters
            }
        }
        let frames = 16_u32;
        let mut planes = Planes {
            left: (0..4).map(|_| vec![0.0; frames as usize]).collect(),
            right: (0..4).map(|_| vec![0.0; frames as usize]).collect(),
        };
        let mut chain = BankChain::new(
            AoSoaScratch::new(BankWidth::Four, frames).expect("scratch"),
            vec![true; 4].into_boxed_slice(),
            vec![
                slot(vec![true; 4], Box::new(Counting { slot: 0, calls: 0 })),
                slot(vec![false; 4], Box::new(Counting { slot: 1, calls: 0 })),
                slot(vec![true; 4], Box::new(Counting { slot: 0, calls: 0 })),
            ],
        )
        .expect("chain");
        for block in 1..=5_u64 {
            chain
                .run(&mut planes, frames, block * u64::from(frames))
                .expect("run");
            assert_eq!(chain.transposes(), block);
        }
        assert_eq!(
            chain.qualification_counters(),
            [10, 0],
            "both live slots ran once per block; the identity-everywhere slot never ran"
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

    /// T5: the scratch is exactly two planes per bank (#96 F9 deleted the sidechain pair).
    #[test]
    fn scratch_allocates_exactly_two_planes() {
        let scratch = AoSoaScratch::new(BankWidth::Eight, 128).expect("scratch");
        assert_eq!(scratch.left.len(), 1024);
        assert_eq!(scratch.right.len(), 1024);
        assert_eq!(
            core::mem::size_of::<AoSoaScratch>(),
            core::mem::size_of::<(Box<[f32]>, Box<[f32]>, u32, BankWidth)>(),
            "AoSoaScratch owns exactly two planes plus width and quantum"
        );
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
        let lanes = 8_usize;
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
                AoSoaScratch::new(BankWidth::Eight, 512).expect("scratch"),
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
                    .run(&mut planes, count as u32, first as u64)
                    .expect("run");
                for (plane, gathered) in out.iter_mut().zip(planes.left.iter()) {
                    plane.extend_from_slice(gathered);
                }
                first += count;
            }
            out
        };
        let oracle = render(512);
        for partition in [1_usize, 7, 64, 128, 512] {
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
        chain.run(&mut planes, frames, 0).expect("run");
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
        let width = if lanes == 4 {
            BankWidth::Four
        } else {
            BankWidth::Eight
        };
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
        chain.run(&mut planes, 8, 0).expect("render");
        assert_eq!(chain.collapses(), 0);
        chain.arm_mono_collapse(true);
        chain.run(&mut planes, 8, 0).expect("render");
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
            chain.run(&mut planes, 8, 0).expect("render");
            dual.run(&mut reference, 8, 0).expect("render");
            assert_eq!(chain.collapses(), u64::from(symmetric));
            for lane in 0..4 {
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
            chain.run(&mut planes, frames, 0).expect("render");
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

    /// A chain whose seam-side slots are not a suffix declines, and so does one with no prefix.
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
        // An upstream slot with no one-plane body declines the whole chain.
        assert_eq!(
            mono_chain(4, 8, vec![Box::new(PassThrough), seam()]).collapse_prefix(),
            0
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
        chain.run(&mut planes, 8, 0).expect("render");
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
        chain.run(&mut planes, 8, 0).expect("render");
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
        chain.run(&mut planes, 8, 0).expect("render");
        assert!(chain.is_collapsed());
        chain.force_mono_collapse_off(true);
        chain.run(&mut planes, 8, 0).expect("render");
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
