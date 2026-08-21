//! Safe preallocated AoSoA rack primitives.
//!
//! The compiler owns all structural decisions. This crate only accepts prepared dimensions and
//! invokes a prepared homogeneous bank over owned, sample-major scratch.
#![allow(missing_docs)]

use miso_engine_core::{KernelBackendV1, TargetCapabilities};
use miso_engine_effect_contract::{
    BankProcessReport, BankWidth, EffectBankProcessBlock, EffectProgramKeyV1,
    PreparedAutomationSpan, PreparedNativeEffectBank,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RackError {
    ZeroFrames,
    FramesAboveQuantum,
    WidthMismatch,
    Overflow,
    Shape,
    FirstSampleOverflow,
}

/// The retained dispatch result. `select` is control-plane-only and pure.
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
    #[must_use]
    pub const fn bank_width(self) -> Option<BankWidth> {
        match self.backend {
            KernelBackendV1::WasmSimd128 | KernelBackendV1::Aarch64Neon => Some(BankWidth::Four),
            KernelBackendV1::X86Avx2 | KernelBackendV1::X86Avx2Fma => Some(BankWidth::Eight),
            KernelBackendV1::Scalar => None,
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

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RoutingClassV1 {
    MainOnly = 1,
    SidechainUnconnected = 2,
    SidechainConnected = 3,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RackSlotKeyV1 {
    pub program: EffectProgramKeyV1,
    pub occurrence: u32,
}

/// Semantic cohort key: no track IDs, parameter values, state bytes, hashes, or serialization.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RackProgramSignatureV1 {
    pub rack: RackLocationV1,
    pub sample_rate: u32,
    pub quantum: u32,
    pub slots: Box<[RackSlotKeyV1]>,
    pub routing: RoutingClassV1,
}

impl RackProgramSignatureV1 {
    pub fn new(
        rack: RackLocationV1,
        sample_rate: u32,
        quantum: u32,
        slots: Vec<EffectProgramKeyV1>,
        routing: RoutingClassV1,
    ) -> Result<Self, RackError> {
        if sample_rate == 0 || quantum == 0 {
            return Err(RackError::Shape);
        }
        let mut occurrences = std::collections::BTreeMap::<EffectProgramKeyV1, u32>::new();
        let slots = slots
            .into_iter()
            .map(|program| {
                let occurrence = occurrences.entry(program.clone()).or_insert(0);
                let result = RackSlotKeyV1 {
                    program,
                    occurrence: *occurrence,
                };
                *occurrence = occurrence.saturating_add(1);
                result
            })
            .collect();
        Ok(Self {
            rack,
            sample_rate,
            quantum,
            slots,
            routing,
        })
    }
    #[must_use]
    pub fn is_subsequence_of(&self, candidate: &Self) -> Option<Box<[bool]>> {
        if self.rack != candidate.rack
            || self.sample_rate != candidate.sample_rate
            || self.quantum != candidate.quantum
            || self.routing != candidate.routing
        {
            return None;
        }
        let mut cursor = 0usize;
        let mut mask = vec![false; candidate.slots.len()];
        for slot in &self.slots {
            while cursor < candidate.slots.len() && candidate.slots[cursor] != *slot {
                cursor += 1;
            }
            if cursor == candidate.slots.len() {
                return None;
            }
            mask[cursor] = true;
            cursor += 1;
        }
        Some(mask.into_boxed_slice())
    }
}

/// Owned left/right sample-major scratch. Its logical index is `sample * lanes + lane`.
pub struct AoSoaScratch {
    width: BankWidth,
    quantum: u32,
    left: Box<[f32]>,
    right: Box<[f32]>,
    sidechain_left: Box<[f32]>,
    sidechain_right: Box<[f32]>,
}

impl AoSoaScratch {
    pub fn new(width: BankWidth, quantum: u32) -> Result<Self, RackError> {
        if quantum == 0 {
            return Err(RackError::ZeroFrames);
        }
        let length = (quantum as usize)
            .checked_mul(width.lanes() as usize)
            .ok_or(RackError::Overflow)?;
        Ok(Self {
            width,
            quantum,
            left: vec![0.0; length].into_boxed_slice(),
            right: vec![0.0; length].into_boxed_slice(),
            sidechain_left: vec![0.0; length].into_boxed_slice(),
            sidechain_right: vec![0.0; length].into_boxed_slice(),
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
    pub fn gather(
        &mut self,
        inputs_left: &[&[f32]],
        inputs_right: &[&[f32]],
        frames: u32,
    ) -> Result<(), RackError> {
        self.checked(frames, inputs_left.len(), inputs_right.len())?;
        let lanes = self.width.lanes() as usize;
        for sample in 0..frames as usize {
            for lane in 0..lanes {
                let index = sample * lanes + lane;
                self.left[index] = inputs_left[lane][sample];
                self.right[index] = inputs_right[lane][sample];
            }
        }
        Ok(())
    }

    /// Gather sidechain inputs into the separate owned AoSoA sidechain scratch.
    pub fn gather_sidechain(
        &mut self,
        inputs_left: &[&[f32]],
        inputs_right: &[&[f32]],
        frames: u32,
    ) -> Result<(), RackError> {
        self.checked(frames, inputs_left.len(), inputs_right.len())?;
        let lanes = self.width.lanes() as usize;
        for sample in 0..frames as usize {
            for lane in 0..lanes {
                let index = sample * lanes + lane;
                self.sidechain_left[index] = inputs_left[lane][sample];
                self.sidechain_right[index] = inputs_right[lane][sample];
            }
        }
        Ok(())
    }
    pub fn scatter(
        &self,
        outputs_left: &mut [&mut [f32]],
        outputs_right: &mut [&mut [f32]],
        frames: u32,
    ) -> Result<(), RackError> {
        self.checked(frames, outputs_left.len(), outputs_right.len())?;
        let lanes = self.width.lanes() as usize;
        for sample in 0..frames as usize {
            for lane in 0..lanes {
                let index = sample * lanes + lane;
                outputs_left[lane][sample] = self.left[index];
                outputs_right[lane][sample] = self.right[index];
            }
        }
        Ok(())
    }
    pub fn process(
        &mut self,
        bank: &mut dyn PreparedNativeEffectBank,
        frames: u32,
        first_sample: u64,
        automation: &[PreparedAutomationSpan],
        offsets: &[u32],
        sidechain: bool,
    ) -> Result<BankProcessReport, RackError> {
        if bank.metadata().width != self.width {
            return Err(RackError::WidthMismatch);
        }
        self.checked(
            frames,
            self.width.lanes() as usize,
            self.width.lanes() as usize,
        )?;
        first_sample
            .checked_add(u64::from(frames))
            .ok_or(RackError::FirstSampleOverflow)?;
        let length = frames as usize * self.width.lanes() as usize;
        let sidechain = sidechain.then_some((
            &self.sidechain_left[..length],
            &self.sidechain_right[..length],
        ));
        let block = EffectBankProcessBlock::new(
            &mut self.left[..length],
            &mut self.right[..length],
            sidechain,
            frames,
            self.width,
            first_sample,
            automation,
            offsets,
            self.quantum,
        )
        .map_err(|_| RackError::Shape)?;
        Ok(bank.process_bank(block))
    }
    fn checked(&self, frames: u32, left_lanes: usize, right_lanes: usize) -> Result<(), RackError> {
        if frames == 0 {
            return Err(RackError::ZeroFrames);
        }
        if frames > self.quantum {
            return Err(RackError::FramesAboveQuantum);
        }
        if left_lanes != self.width.lanes() as usize || right_lanes != self.width.lanes() as usize {
            return Err(RackError::Shape);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_core::TargetCapabilities;
    #[test]
    fn dispatch_requires_avx2_for_fma() {
        assert_eq!(
            KernelDispatch::select(TargetCapabilities::from_detected(false, false, false, true))
                .backend(),
            KernelBackendV1::Scalar
        );
        assert_eq!(
            KernelDispatch::select(TargetCapabilities::from_detected(false, false, true, false))
                .backend(),
            KernelBackendV1::X86Avx2
        );
    }

    #[test]
    fn sample_major_gather_and_scatter_preserve_dual_mono_lanes() {
        let mut scratch = AoSoaScratch::new(BankWidth::Four, 3).expect("prepare scratch");
        let left = [[1.0, 2.0], [3.0, 4.0], [5.0, 6.0], [7.0, 8.0]];
        let right = [[-1.0, -2.0], [-3.0, -4.0], [-5.0, -6.0], [-7.0, -8.0]];
        let left_refs: Vec<&[f32]> = left.iter().map(|lane| lane.as_slice()).collect();
        let right_refs: Vec<&[f32]> = right.iter().map(|lane| lane.as_slice()).collect();
        scratch.gather(&left_refs, &right_refs, 2).expect("gather");
        let mut out_left = [[0.0; 2]; 4];
        let mut out_right = [[0.0; 2]; 4];
        let mut out_left_refs: Vec<&mut [f32]> = out_left
            .iter_mut()
            .map(|lane| lane.as_mut_slice())
            .collect();
        let mut out_right_refs: Vec<&mut [f32]> = out_right
            .iter_mut()
            .map(|lane| lane.as_mut_slice())
            .collect();
        scratch
            .scatter(&mut out_left_refs, &mut out_right_refs, 2)
            .expect("scatter");
        assert_eq!(out_left, left);
        assert_eq!(out_right, right);
    }
}
