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

use miso_engine_core::realtime::RenderError;
use miso_engine_effect_contract::{
    BankWidth, BypassShunt, EffectBankProcessBlock, EffectControlLane, EffectProgramKeyV1,
    ObservationLaneV1, ObservationSampleV1, PreparedAutomationSpan, PreparedNativeEffectBank,
    PreparedSidechainPort, transpose_tile_4, transpose_tile_8,
};

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
/// SIMD chain: [`RackProgramV1::subsequence_mask`] compares `rack` first, and
/// `miso_engine_rack_compiler::plan_bank_groups` pools per location.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RackLocationV1 {
    Simd1 = 1,
    Simd2 = 2,
    Dynamic = 3,
}

impl RackLocationV1 {
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

impl BankSlotKey for EffectProgramKeyV1 {
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
/// There is no rate, quantum or routing field: every [`EffectProgramKeyV1`] slot already carries
/// `sample_rate`, `quantum` and `ports.sidechain`, so a second copy could only disagree (#96 F5.4).
///
/// `K` is the slot key, defaulting to [`EffectProgramKeyV1`] — the SIMD racks' key. A fixed graph
/// stage with a key of its own (the post-input builtin bank, #86) is planned by the same planner
/// without either side having to fabricate the other's key type.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RackProgramV1<K = EffectProgramKeyV1> {
    pub rack: RackLocationV1,
    pub slots: Box<[K]>,
}

impl<K: BankSlotKey> RackProgramV1<K> {
    #[must_use]
    pub fn new(rack: RackLocationV1, slots: Vec<K>) -> Self {
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
    /// [`EffectProgramKeyV1`] equality only - never by occurrence index (#96 F5.3). Greedy
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
}

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

/// The resident AoSoA block handed to one stage. `left.len() == right.len() == frames * lanes`.
pub struct BankBlock<'a> {
    pub left: &'a mut [f32],
    pub right: &'a mut [f32],
    pub frames: u32,
    pub first_sample: u64,
    pub lanes: usize,
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
        Ok(Self {
            processor,
            width,
            quantum,
            offsets: vec![0_u32; width.lanes() as usize + 1].into_boxed_slice(),
        })
    }
}

impl BankStage for EffectBankStage {
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
    observations: Option<Box<[Option<ObservationLaneV1>]>>,
    /// One reading per lane, filled by a single `observe_resident_bank` call per armed tap.
    ///
    /// Allocated at bind and only when the slot is observed at all, so an unobserved slot holds
    /// nothing and an observed one allocates nothing per block.
    samples: Box<[ObservationSampleV1]>,
    /// Records dropped because a lane's window was full of distinct targets. Zero by construction.
    dropped: u64,
    /// `Observe` records this slot had no capacity to apply. Zero by construction.
    unbound: u64,
}

impl ConsoleEffectBankStage {
    /// Builds the console stage for one bound bank slot.
    ///
    /// `latency` is the slot's declared [`miso_engine_effect_contract::PreparedEffectMetadata::latency`]. Every lane of a bank
    /// shares one [`EffectProgramKeyV1`], so they share one latency and one AoSoA delay line:
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
        observations: Vec<Option<ObservationLaneV1>>,
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
            vec![ObservationSampleV1::default(); lane_count].into_boxed_slice()
        } else {
            Vec::new().into_boxed_slice()
        };
        Ok(Self {
            processor,
            width,
            quantum,
            offsets: vec![0_u32; lane_count + 1].into_boxed_slice(),
            lanes: lanes.into_boxed_slice(),
            staging: vec![IDLE_SPAN; capacity].into_boxed_slice(),
            packed: vec![IDLE_SPAN; total].into_boxed_slice(),
            shunt,
            observations: observed.then(|| observations.into_boxed_slice()),
            samples,
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
            .map(ObservationLaneV1::retained_bytes)
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
    kind: miso_engine_effect_contract::AutomationSpanKind::Point,
    channel: miso_engine_effect_contract::ParameterChannel::Both,
    parameter_index: 0,
    start_sample: 0,
    end_sample: 0,
    start_value: 0.0,
    end_value: 0.0,
};

impl BankStage for ConsoleEffectBankStage {
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

    fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
        let lane_count = self.width.lanes() as usize;
        // 1. Drain, in lane order, into each lane's own window, then pack down.
        let mut packed = 0_usize;
        self.offsets[0] = 0;
        for lane in 0..lane_count {
            if let Some(channel) = self.lanes[lane].as_mut() {
                let observation = self
                    .observations
                    .as_deref_mut()
                    .and_then(|lanes| lanes.get_mut(lane))
                    .and_then(Option::as_mut);
                let staged = channel.stage(&mut self.staging, block.first_sample, observation);
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
        if let Some(shunt) = self
            .shunt
            .as_mut()
            .filter(|shunt| any_bypassed || shunt.feeds_line())
        {
            shunt.capture(block.left, block.right);
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
        let _ = self.processor.process_bank(bank);
        // 4. Publish every armed tap, after the bank ran. One `observe_resident_bank` call per
        // armed tap fills every lane, so a cohort of eight costs one vector extraction rather than
        // eight scalar reads.
        //
        // Issue #163 phase 4 item 3/6: the slot-level gate comes first. What the old comment here
        // called "one pass over a `bool` array" for a tap no lane armed was, for the slot as a
        // whole, an O(lanes) `max()` to find the tap count plus an O(taps x lanes) `.any()` walk
        // -- 4 096 flag loads per block for the 64-lane, 64-tap console shape, every block,
        // whether or not anything was subscribed. `ObservationLaneV1::any_armed` is now O(1) per
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
                .any(ObservationLaneV1::any_armed)
        }) {
            let taps = lanes
                .iter()
                .filter_map(Option::as_ref)
                .map(ObservationLaneV1::len)
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
}

/// One slot of a chain: a stage plus the lanes for which it is *not* an identity.
pub struct BankSlot {
    pub stage: Box<dyn BankStage>,
    pub active_lanes: Box<[bool]>,
}

/// Per-lane planar views a chain gathers from and scatters to. `lane < lanes` always.
pub trait BankMembers {
    fn plane(&self, lane: usize) -> (&[f32], &[f32]);
    fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]);
}

/// One bank chain: a resident L/R AoSoA block plus its ordered slots.
///
/// Exactly one gather and one scatter per [`run`](BankChain::run), whatever the slot count
/// (master plan §4.5).
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
        Ok(Self {
            scratch,
            lanes,
            active,
            slots: slots.into_boxed_slice(),
            transposes: 0,
            full_bank,
            staging_left: vec![0.0; staging].into_boxed_slice(),
            staging_right: vec![0.0; staging].into_boxed_slice(),
        })
    }

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
        self.gather(members, frames);
        self.transposes = self.transposes.saturating_add(1);
        for slot in &mut self.slots {
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
        self.scatter(members, frames);
        Ok(())
    }

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

    /// AoSoA -> planar for the whole bank, the inverse of [`Self::gather`] under the same rule.
    fn scatter<M: BankMembers + ?Sized>(&mut self, members: &mut M, frames: u32) {
        if self.full_bank {
            match self.scratch.width {
                BankWidth::Four => self.scatter_tiled::<M, 4>(members, frames, transpose_tile_4),
                BankWidth::Eight => self.scatter_tiled::<M, 8>(members, frames, transpose_tile_8),
            }
            return;
        }
        for lane in 0..self.lanes {
            if self.active[lane] {
                let (left, right) = members.plane_mut(lane);
                self.scratch.scatter_lane(lane, left, right, 0, frames);
            }
        }
    }

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
        for lane in 0..W {
            let (left, right) = members.plane_mut(lane);
            debug_assert!(left.len() == frames_used && right.len() == frames_used);
            left.copy_from_slice(&self.staging_left[lane * stride..lane * stride + frames_used]);
            right.copy_from_slice(&self.staging_right[lane * stride..lane * stride + frames_used]);
        }
    }

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
                chain.run(&mut planes, frames, 0).expect("run");
                assert_planes_bit_equal(
                    &planes,
                    &expected,
                    &format!("{lanes} lanes, {frames} frames"),
                );
                assert_eq!(chain.transposes(), 1);
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
                assert_planes_bit_equal(
                    &outcomes[0],
                    &outcomes[1],
                    &format!("tiled vs scalar, {lanes} lanes, {frames} frames"),
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
}
