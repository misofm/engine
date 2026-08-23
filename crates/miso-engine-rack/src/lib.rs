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
use miso_engine_core::{KernelBackendV1, TargetCapabilities};
use miso_engine_effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectProgramKeyV1, PreparedNativeEffectBank,
    PreparedSidechainPort,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RackError {
    ZeroQuantum,
    Overflow,
    Shape,
    WidthMismatch,
}

/// The retained dispatch result. `select` is control-plane-only and pure.
///
/// Transitional carrier of [`KernelBackendV1`]; deleted by #95 when the effect contract takes
/// `miso_engine_lane::Backend` directly. It deliberately holds **no** backend-to-width table of
/// its own: [`KernelBackendV1::lanes`] is the single source (#96 F5).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct KernelDispatch {
    backend: KernelBackendV1,
}

impl KernelDispatch {
    #[must_use]
    pub const fn select(capabilities: TargetCapabilities) -> Self {
        Self {
            backend: KernelBackendV1::select(capabilities),
        }
    }
    #[must_use]
    pub const fn backend(self) -> KernelBackendV1 {
        self.backend
    }
    /// Derived from core's single lane table. `KernelBackendV1` is `#[non_exhaustive]`, so
    /// matching on `lanes()` is the only wildcard-free route to it.
    #[must_use]
    pub const fn bank_width(self) -> Option<BankWidth> {
        match self.backend.lanes() {
            4 => Some(BankWidth::Four),
            8 => Some(BankWidth::Eight),
            _ => None,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RackLocationV1 {
    Simd1 = 1,
    Simd2 = 2,
}

/// The ordered per-track program of one SIMD rack.
///
/// There is no rate, quantum or routing field: every [`EffectProgramKeyV1`] slot already carries
/// `sample_rate`, `quantum` and `ports.sidechain`, so a second copy could only disagree (#96 F5.4).
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RackProgramV1 {
    pub rack: RackLocationV1,
    pub slots: Box<[EffectProgramKeyV1]>,
}

impl RackProgramV1 {
    #[must_use]
    pub fn new(rack: RackLocationV1, slots: Vec<EffectProgramKeyV1>) -> Self {
        Self {
            rack,
            slots: slots.into_boxed_slice(),
        }
    }
    /// `true` iff the program is non-empty and no slot declares a connected sidechain.
    ///
    /// An empty program needs no bank at all, and a connected sidechain reads a second graph
    /// buffer that a homogeneous bank has no port for (#96 F5.1/F9).
    #[must_use]
    pub fn is_bankable(&self) -> bool {
        !self.slots.is_empty()
            && !self.slots.iter().any(|slot| {
                matches!(
                    slot.ports.sidechain,
                    PreparedSidechainPort::Connected { .. }
                )
            })
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

    /// Copy one planar track into its stable AoSoA lane. Shape is a `debug_assert`: the compiler
    /// fixed it once (master plan §4.3).
    fn gather_lane(&mut self, lane: usize, left: &[f32], right: &[f32], frames: u32) {
        let lanes = self.width.lanes() as usize;
        let len = frames as usize * lanes;
        debug_assert!(lane < lanes);
        debug_assert!(left.len() == frames as usize && right.len() == frames as usize);
        debug_assert!(len <= self.left.len());
        for (chunk, &sample) in self.left[..len].chunks_exact_mut(lanes).zip(left) {
            chunk[lane] = sample;
        }
        for (chunk, &sample) in self.right[..len].chunks_exact_mut(lanes).zip(right) {
            chunk[lane] = sample;
        }
    }

    /// Copy one stable AoSoA lane back into its planar graph buffer.
    fn scatter_lane(&self, lane: usize, left: &mut [f32], right: &mut [f32], frames: u32) {
        let lanes = self.width.lanes() as usize;
        let len = frames as usize * lanes;
        debug_assert!(lane < lanes);
        debug_assert!(left.len() == frames as usize && right.len() == frames as usize);
        debug_assert!(len <= self.left.len());
        for (chunk, sample) in self.left[..len].chunks_exact(lanes).zip(left.iter_mut()) {
            *sample = chunk[lane];
        }
        for (chunk, sample) in self.right[..len].chunks_exact(lanes).zip(right.iter_mut()) {
            *sample = chunk[lane];
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
        Ok(Self {
            scratch,
            lanes,
            active,
            slots: slots.into_boxed_slice(),
            transposes: 0,
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
        for lane in 0..self.lanes {
            if self.active[lane] {
                let (left, right) = members.plane(lane);
                self.scratch.gather_lane(lane, left, right, frames);
            }
        }
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
        for lane in 0..self.lanes {
            if self.active[lane] {
                let (left, right) = members.plane_mut(lane);
                self.scratch.scatter_lane(lane, left, right, frames);
            }
        }
        Ok(())
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_core::TargetCapabilities;

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

    #[test]
    fn issue068_all_capability_tuples_select_exact_backend_and_width() {
        for wasm_simd128 in [false, true] {
            for aarch64_neon in [false, true] {
                for x86_avx2 in [false, true] {
                    for x86_fma in [false, true] {
                        let capabilities = TargetCapabilities::from_detected(
                            wasm_simd128,
                            aarch64_neon,
                            x86_avx2,
                            x86_fma,
                        );
                        let expected = if x86_avx2 && x86_fma {
                            KernelBackendV1::X86Avx2Fma
                        } else if x86_avx2 {
                            KernelBackendV1::X86Avx2
                        } else if aarch64_neon {
                            KernelBackendV1::Aarch64Neon
                        } else if wasm_simd128 {
                            KernelBackendV1::WasmSimd128
                        } else {
                            KernelBackendV1::Scalar
                        };
                        let expected_width = match expected.lanes() {
                            4 => Some(BankWidth::Four),
                            8 => Some(BankWidth::Eight),
                            _ => None,
                        };
                        let dispatch = KernelDispatch::select(capabilities);
                        assert_eq!(KernelBackendV1::select(capabilities), expected);
                        assert_eq!(dispatch.backend(), expected);
                        assert_eq!(dispatch.bank_width(), expected_width);
                    }
                }
            }
        }
    }

    #[test]
    fn dispatch_requires_avx2_for_fma() {
        for (wasm, neon, avx2, fma) in [
            (false, false, false, true),
            (false, false, true, false),
            (false, true, false, true),
            (true, false, false, true),
        ] {
            let capabilities = TargetCapabilities::from_detected(wasm, neon, avx2, fma);
            assert_eq!(
                KernelDispatch::select(capabilities).backend(),
                KernelBackendV1::select(capabilities)
            );
        }
        assert_eq!(
            KernelDispatch::select(TargetCapabilities::from_detected(false, false, false, true))
                .backend(),
            KernelBackendV1::Scalar
        );
    }

    /// T1: the gather/scatter round-trip is bit-exact for every active lane, including NaN
    /// payloads, `-0.0` and subnormals, and never writes an inactive lane's planar buffer.
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
            let mut out: Vec<Vec<f32>> = vec![Vec::with_capacity(frames); lanes];
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
                for lane in 0..lanes {
                    out[lane].extend_from_slice(&planes.left[lane]);
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
