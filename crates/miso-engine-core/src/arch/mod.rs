//! Architecture-owned SIMD kernels with safe, preparation-gated entry points.

#![allow(unsafe_code)]

mod scalar;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
mod wasm32;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86;

use crate::KernelBackendV1;

type TptKernelFn = for<'a> fn(TptKernelBlock<'a>);
type DeltaKernelFn = for<'a> fn(DeltaKernelBlock<'a>);
type CompressorGainMixKernelFn = for<'a> fn(CompressorGainMixKernelBlock<'a>);
type GateGainKernelFn = for<'a> fn(GateGainKernelBlock<'a>);
type SoftClipKernelFn = for<'a> fn(SoftClipKernelBlock<'a>);

const SOFT_CLIP_HISTORY_WORDS: usize = 63;
const SOFT_CLIP_NONZERO_TAPS: [usize; 31] = [
    2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 31, 32, 34, 36, 38, 40, 42, 44, 46, 48,
    50, 52, 54, 56, 58, 60,
];

/// Preparation or shape failure for an architecture-owned TPT bank kernel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TptBankKernelError {
    /// The selected semantic backend cannot execute on this build/current processor.
    BackendUnavailable,
    /// Every slice must contain exactly the backend's logical lane count.
    LaneLength,
    /// A high-pass selection mask was neither all-zero nor all-one.
    MaskValue,
}

/// Preparation or shape failure for an architecture-owned endpoint-conditioned delta bank kernel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeltaBankKernelError {
    /// The selected semantic backend cannot execute on this build/current processor.
    BackendUnavailable,
    /// Every slice must contain exactly the backend's logical lane count.
    LaneLength,
    /// An identity selection mask was neither all-zero nor all-one.
    MaskValue,
}

/// Preparation or input-shape failure for the compressor gain/mix bank kernel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompressorGainMixKernelError {
    /// The selected semantic backend cannot execute on this build/current processor.
    BackendUnavailable,
    /// Every slice must contain exactly the backend's logical lane count.
    LaneLength,
    /// A dry or wet mask value was neither all-zero nor all-one.
    MaskValue,
    /// A lane selected both exact dry and exact wet output.
    MaskOverlap,
}

/// Preparation or shape failure for the gate/expander gain-selection bank kernel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GateGainKernelError {
    /// The selected semantic backend cannot execute on this build/current processor.
    BackendUnavailable,
    /// Every slice must contain exactly the backend's logical lane count.
    LaneLength,
    /// An identity selection mask was neither all-zero nor all-one.
    MaskValue,
}

/// Preparation or shape failure for one soft-clip high-rate bank phase.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SoftClipBankKernelError {
    /// The selected semantic backend cannot execute on this build/current processor.
    BackendUnavailable,
    /// A per-lane slice did not match the backend's exact logical lane count.
    LaneLength,
    /// The supplied fixed FIR table did not contain exactly 63 finite coefficients.
    CoefficientTable,
    /// A sample-major history did not contain exactly `63 * lane_count` words.
    HistoryLength,
    /// A lane high-rate cursor was outside the 63-word history range.
    Cursor,
}

/// A safe, immutable dispatch token prepared before realtime rendering.
///
/// Construction performs the only required runtime feature detection. The retained function
/// pointer can therefore enter its architecture-specific `target_feature` function without
/// detection, allocation, locking, or an unsafe caller contract during render.
#[derive(Clone, Copy)]
pub struct PreparedTptBankKernelV1 {
    backend: KernelBackendV1,
    process: TptKernelFn,
}

impl PreparedTptBankKernelV1 {
    /// Prepare the exact semantic backend when it is executable by this artifact and processor.
    pub fn try_new(backend: KernelBackendV1) -> Result<Self, TptBankKernelError> {
        let process = match backend {
            KernelBackendV1::Scalar => scalar::process_tpt_scalar,
            KernelBackendV1::WasmSimd128 => wasm_kernel()?,
            KernelBackendV1::Aarch64Neon => neon_kernel()?,
            KernelBackendV1::X86Avx2 => x86_avx2_kernel()?,
            KernelBackendV1::X86Avx2Fma => x86_avx2_fma_kernel()?,
        };
        Ok(Self { backend, process })
    }

    /// Semantic backend whose target preconditions were proved at preparation.
    #[must_use]
    pub const fn backend(self) -> KernelBackendV1 {
        self.backend
    }

    /// Execute one sample across one coefficient/state bank.
    ///
    /// `high_pass_mask` uses `u32::MAX` for high-pass and zero for low-pass. All slices are
    /// validated before the prepared function pointer is called. State is updated in place and
    /// `samples` receives the selected low/high output.
    #[allow(clippy::too_many_arguments)]
    pub fn process_tpt(
        self,
        samples: &mut [f32],
        c1: &[f32],
        a2: &[f32],
        a3: &[f32],
        k: &[f32],
        s1: &mut [f32],
        s2: &mut [f32],
        high_pass_mask: &[u32],
    ) -> Result<(), TptBankKernelError> {
        let lanes = self.backend.lanes() as usize;
        if [
            samples.len(),
            c1.len(),
            a2.len(),
            a3.len(),
            k.len(),
            s1.len(),
            s2.len(),
            high_pass_mask.len(),
        ]
        .into_iter()
        .any(|length| length != lanes)
        {
            return Err(TptBankKernelError::LaneLength);
        }
        if high_pass_mask
            .iter()
            .any(|mask| !matches!(*mask, 0 | u32::MAX))
        {
            return Err(TptBankKernelError::MaskValue);
        }
        (self.process)(TptKernelBlock {
            samples,
            c1,
            a2,
            a3,
            k,
            s1,
            s2,
            high_pass_mask,
        });
        Ok(())
    }
}

/// A safe, immutable prepared endpoint-conditioned delta bank dispatch token.
///
/// Construction performs all architecture feature detection. Render callers only provide exact
/// lane slices, so this token enters private architecture-specific functions without detection or
/// an unsafe caller contract.
#[derive(Clone, Copy)]
pub struct PreparedDeltaBankKernelV1 {
    backend: KernelBackendV1,
    process: DeltaKernelFn,
}

impl PreparedDeltaBankKernelV1 {
    /// Prepare the exact semantic backend when it is executable by this artifact and processor.
    pub fn try_new(backend: KernelBackendV1) -> Result<Self, DeltaBankKernelError> {
        let process = match backend {
            KernelBackendV1::Scalar => scalar::process_delta_scalar,
            KernelBackendV1::WasmSimd128 => delta_wasm_kernel()?,
            KernelBackendV1::Aarch64Neon => delta_neon_kernel()?,
            KernelBackendV1::X86Avx2 => delta_x86_avx2_kernel()?,
            KernelBackendV1::X86Avx2Fma => delta_x86_avx2_fma_kernel()?,
        };
        Ok(Self { backend, process })
    }

    /// Semantic backend whose target preconditions were proved at preparation.
    #[must_use]
    pub const fn backend(self) -> KernelBackendV1 {
        self.backend
    }

    /// Execute one endpoint-conditioned delta sample across one coefficient/state bank.
    ///
    /// Coefficient slices carry `(a,n0,d0,n1,d1,n2,d2)`. `identity_mask` uses `u32::MAX` for
    /// exact identity/warm-history behavior and zero for the frozen noncontracting recurrence.
    /// Every slice must have exactly the backend lane count; the mask accepts only those two bit
    /// patterns.
    #[allow(clippy::too_many_arguments)]
    pub fn process_delta(
        self,
        samples: &mut [f32],
        a: &[f32],
        n0: &[f32],
        d0: &[f32],
        n1: &[f32],
        d1: &[f32],
        n2: &[f32],
        d2: &[f32],
        x1: &mut [f32],
        x2: &mut [f32],
        y1: &mut [f32],
        y2: &mut [f32],
        identity_mask: &[u32],
    ) -> Result<(), DeltaBankKernelError> {
        let lanes = self.backend.lanes() as usize;
        if [
            samples.len(),
            a.len(),
            n0.len(),
            d0.len(),
            n1.len(),
            d1.len(),
            n2.len(),
            d2.len(),
            x1.len(),
            x2.len(),
            y1.len(),
            y2.len(),
            identity_mask.len(),
        ]
        .into_iter()
        .any(|length| length != lanes)
        {
            return Err(DeltaBankKernelError::LaneLength);
        }
        if identity_mask
            .iter()
            .any(|mask| !matches!(*mask, 0 | u32::MAX))
        {
            return Err(DeltaBankKernelError::MaskValue);
        }
        (self.process)(DeltaKernelBlock {
            samples,
            a,
            n0,
            d0,
            n1,
            d1,
            n2,
            d2,
            x1,
            x2,
            y1,
            y2,
            identity_mask,
        });
        Ok(())
    }
}

/// A safe, immutable prepared compressor gain/mix bank dispatch token.
///
/// Preparation performs feature detection once. Render only validates fixed-width slices and calls
/// the retained safe function pointer; no SIMD feature detection or unsafe caller contract leaks
/// into an effect implementation.
#[derive(Clone, Copy)]
pub struct PreparedCompressorGainMixKernelV1 {
    backend: KernelBackendV1,
    process: CompressorGainMixKernelFn,
}

impl PreparedCompressorGainMixKernelV1 {
    /// Prepare the exact semantic backend when it is executable by this artifact and processor.
    pub fn try_new(backend: KernelBackendV1) -> Result<Self, CompressorGainMixKernelError> {
        let process = match backend {
            KernelBackendV1::Scalar => scalar::process_compressor_gain_mix_scalar,
            KernelBackendV1::WasmSimd128 => compressor_wasm_kernel()?,
            KernelBackendV1::Aarch64Neon => compressor_neon_kernel()?,
            KernelBackendV1::X86Avx2 => compressor_x86_avx2_kernel()?,
            KernelBackendV1::X86Avx2Fma => compressor_x86_avx2_fma_kernel()?,
        };
        Ok(Self { backend, process })
    }

    /// Semantic backend whose target preconditions were proved at preparation.
    #[must_use]
    pub const fn backend(self) -> KernelBackendV1 {
        self.backend
    }

    /// Apply the frozen noncontracting dry/gain/mix selection graph to one bank sample.
    ///
    /// Each slice must have exactly the prepared backend width. `dry_mask` and `wet_mask` accept
    /// only zero or `u32::MAX`; a lane may not select both exact identity paths.
    pub fn process_gain_mix(
        self,
        samples: &mut [f32],
        gains: &[f32],
        mixes: &[f32],
        dry_mask: &[u32],
        wet_mask: &[u32],
    ) -> Result<(), CompressorGainMixKernelError> {
        let lanes = self.backend.lanes() as usize;
        if [
            samples.len(),
            gains.len(),
            mixes.len(),
            dry_mask.len(),
            wet_mask.len(),
        ]
        .into_iter()
        .any(|length| length != lanes)
        {
            return Err(CompressorGainMixKernelError::LaneLength);
        }
        if dry_mask
            .iter()
            .chain(wet_mask)
            .any(|mask| !matches!(*mask, 0 | u32::MAX))
        {
            return Err(CompressorGainMixKernelError::MaskValue);
        }
        if dry_mask
            .iter()
            .zip(wet_mask)
            .any(|(dry, wet)| *dry == u32::MAX && *wet == u32::MAX)
        {
            return Err(CompressorGainMixKernelError::MaskOverlap);
        }
        (self.process)(CompressorGainMixKernelBlock {
            samples,
            gains,
            mixes,
            dry_mask,
            wet_mask,
        });
        Ok(())
    }
}

/// A safe, immutable prepared gate/expander gain-selection dispatch token.
///
/// The frozen graph is deliberately smaller than compressor mix: `p0 = sample * gain`, followed
/// by an exact dry selection only when the identity mask is all ones. Construction performs all
/// target detection; rendering executes a retained safe function pointer.
#[derive(Clone, Copy)]
pub struct PreparedGateGainKernelV1 {
    backend: KernelBackendV1,
    process: GateGainKernelFn,
}

impl PreparedGateGainKernelV1 {
    /// Prepares the exact semantic backend if it is executable by this artifact and processor.
    pub fn try_new(backend: KernelBackendV1) -> Result<Self, GateGainKernelError> {
        let process = match backend {
            KernelBackendV1::Scalar => scalar::process_gate_gain_scalar,
            KernelBackendV1::WasmSimd128 => gate_wasm_kernel()?,
            KernelBackendV1::Aarch64Neon => gate_neon_kernel()?,
            KernelBackendV1::X86Avx2 => gate_x86_avx2_kernel()?,
            KernelBackendV1::X86Avx2Fma => gate_x86_avx2_fma_kernel()?,
        };
        Ok(Self { backend, process })
    }

    /// Semantic backend whose target preconditions were proved at preparation.
    #[must_use]
    pub const fn backend(self) -> KernelBackendV1 {
        self.backend
    }

    /// Applies `p0 = sample * gain`, selecting exact `sample` for identity lanes.
    ///
    /// All slices must have exactly the prepared logical lane width. Each identity mask is either
    /// zero or `u32::MAX`; no fused multiply-add operation is part of this contract.
    pub fn process_gain(
        self,
        samples: &mut [f32],
        gains: &[f32],
        identity_mask: &[u32],
    ) -> Result<(), GateGainKernelError> {
        let lanes = self.backend.lanes() as usize;
        if [samples.len(), gains.len(), identity_mask.len()]
            .into_iter()
            .any(|length| length != lanes)
        {
            return Err(GateGainKernelError::LaneLength);
        }
        if identity_mask
            .iter()
            .any(|mask| !matches!(*mask, 0 | u32::MAX))
        {
            return Err(GateGainKernelError::MaskValue);
        }
        (self.process)(GateGainKernelBlock {
            samples,
            gains,
            identity_mask,
        });
        Ok(())
    }
}

/// A safe, immutable dispatch token for one fixed-2x soft-clip high-rate phase.
///
/// The caller supplies an effect-owned 63-word coefficient table and two sample-major histories.
/// Each logical lane owns an independent cursor, so reset, restore, and recovery remain
/// lane-local. Construction performs architecture feature detection only on the control plane.
#[derive(Clone, Copy)]
pub struct PreparedSoftClipBankKernelV1 {
    backend: KernelBackendV1,
    process: SoftClipKernelFn,
}

impl PreparedSoftClipBankKernelV1 {
    /// Prepare the selected backend if it is executable by this artifact and processor.
    pub fn try_new(backend: KernelBackendV1) -> Result<Self, SoftClipBankKernelError> {
        let process = match backend {
            KernelBackendV1::Scalar => scalar::process_soft_clip_scalar,
            KernelBackendV1::WasmSimd128 => soft_clip_wasm_kernel()?,
            KernelBackendV1::Aarch64Neon => soft_clip_neon_kernel()?,
            KernelBackendV1::X86Avx2 => soft_clip_x86_avx2_kernel()?,
            KernelBackendV1::X86Avx2Fma => soft_clip_x86_avx2_fma_kernel()?,
        };
        Ok(Self { backend, process })
    }

    /// Semantic backend whose feature preconditions were proved at preparation.
    #[must_use]
    pub const fn backend(self) -> KernelBackendV1 {
        self.backend
    }

    /// Execute one interpolate/cubic/decimate high-rate phase across exact logical lanes.
    ///
    /// `samples` are written to the interpolation history and replaced with the decimator result.
    /// The two histories use sample-major AoSoA storage: word `sample * lanes + lane`. The routine
    /// traverses the frozen ascending nonzero indices, performs separate multiply/add operations,
    /// writes the cubic result, then advances each lane cursor exactly once.
    pub fn process_phase(
        self,
        samples: &mut [f32],
        coefficients: &[f32],
        cursors: &mut [u32],
        interpolation_history: &mut [f32],
        decimation_history: &mut [f32],
    ) -> Result<(), SoftClipBankKernelError> {
        let lanes = self.backend.lanes() as usize;
        if samples.len() != lanes || cursors.len() != lanes {
            return Err(SoftClipBankKernelError::LaneLength);
        }
        if coefficients.len() != SOFT_CLIP_HISTORY_WORDS
            || coefficients.iter().any(|value| !value.is_finite())
        {
            return Err(SoftClipBankKernelError::CoefficientTable);
        }
        let history_len = SOFT_CLIP_HISTORY_WORDS
            .checked_mul(lanes)
            .ok_or(SoftClipBankKernelError::HistoryLength)?;
        if interpolation_history.len() != history_len || decimation_history.len() != history_len {
            return Err(SoftClipBankKernelError::HistoryLength);
        }
        if cursors
            .iter()
            .any(|cursor| *cursor as usize >= SOFT_CLIP_HISTORY_WORDS)
        {
            return Err(SoftClipBankKernelError::Cursor);
        }
        (self.process)(SoftClipKernelBlock {
            samples,
            coefficients,
            cursors,
            interpolation_history,
            decimation_history,
        });
        Ok(())
    }
}

pub(super) struct TptKernelBlock<'a> {
    pub(super) samples: &'a mut [f32],
    pub(super) c1: &'a [f32],
    pub(super) a2: &'a [f32],
    pub(super) a3: &'a [f32],
    pub(super) k: &'a [f32],
    pub(super) s1: &'a mut [f32],
    pub(super) s2: &'a mut [f32],
    pub(super) high_pass_mask: &'a [u32],
}

pub(super) struct DeltaKernelBlock<'a> {
    pub(super) samples: &'a mut [f32],
    pub(super) a: &'a [f32],
    pub(super) n0: &'a [f32],
    pub(super) d0: &'a [f32],
    pub(super) n1: &'a [f32],
    pub(super) d1: &'a [f32],
    pub(super) n2: &'a [f32],
    pub(super) d2: &'a [f32],
    pub(super) x1: &'a mut [f32],
    pub(super) x2: &'a mut [f32],
    pub(super) y1: &'a mut [f32],
    pub(super) y2: &'a mut [f32],
    pub(super) identity_mask: &'a [u32],
}

pub(super) struct CompressorGainMixKernelBlock<'a> {
    pub(super) samples: &'a mut [f32],
    pub(super) gains: &'a [f32],
    pub(super) mixes: &'a [f32],
    pub(super) dry_mask: &'a [u32],
    pub(super) wet_mask: &'a [u32],
}

pub(super) struct GateGainKernelBlock<'a> {
    pub(super) samples: &'a mut [f32],
    pub(super) gains: &'a [f32],
    pub(super) identity_mask: &'a [u32],
}

pub(super) struct SoftClipKernelBlock<'a> {
    pub(super) samples: &'a mut [f32],
    pub(super) coefficients: &'a [f32],
    pub(super) cursors: &'a mut [u32],
    pub(super) interpolation_history: &'a mut [f32],
    pub(super) decimation_history: &'a mut [f32],
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
fn wasm_kernel() -> Result<TptKernelFn, TptBankKernelError> {
    Ok(wasm32::process_tpt_wasm_simd128)
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
fn delta_wasm_kernel() -> Result<DeltaKernelFn, DeltaBankKernelError> {
    Ok(wasm32::process_delta_wasm_simd128)
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
fn compressor_wasm_kernel() -> Result<CompressorGainMixKernelFn, CompressorGainMixKernelError> {
    Ok(wasm32::process_compressor_gain_mix_wasm_simd128)
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
fn gate_wasm_kernel() -> Result<GateGainKernelFn, GateGainKernelError> {
    Ok(wasm32::process_gate_gain_wasm_simd128)
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
fn soft_clip_wasm_kernel() -> Result<SoftClipKernelFn, SoftClipBankKernelError> {
    Ok(wasm32::process_soft_clip_wasm_simd128)
}

#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
fn delta_wasm_kernel() -> Result<DeltaKernelFn, DeltaBankKernelError> {
    Err(DeltaBankKernelError::BackendUnavailable)
}

#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
fn compressor_wasm_kernel() -> Result<CompressorGainMixKernelFn, CompressorGainMixKernelError> {
    Err(CompressorGainMixKernelError::BackendUnavailable)
}

#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
fn gate_wasm_kernel() -> Result<GateGainKernelFn, GateGainKernelError> {
    Err(GateGainKernelError::BackendUnavailable)
}

#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
fn soft_clip_wasm_kernel() -> Result<SoftClipKernelFn, SoftClipBankKernelError> {
    Err(SoftClipBankKernelError::BackendUnavailable)
}

#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
fn wasm_kernel() -> Result<TptKernelFn, TptBankKernelError> {
    Err(TptBankKernelError::BackendUnavailable)
}

#[cfg(target_arch = "aarch64")]
fn neon_kernel() -> Result<TptKernelFn, TptBankKernelError> {
    Ok(aarch64::process_tpt_aarch64_neon)
}

#[cfg(target_arch = "aarch64")]
fn delta_neon_kernel() -> Result<DeltaKernelFn, DeltaBankKernelError> {
    Ok(aarch64::process_delta_aarch64_neon)
}

#[cfg(target_arch = "aarch64")]
fn compressor_neon_kernel() -> Result<CompressorGainMixKernelFn, CompressorGainMixKernelError> {
    Ok(aarch64::process_compressor_gain_mix_aarch64_neon)
}

#[cfg(target_arch = "aarch64")]
fn gate_neon_kernel() -> Result<GateGainKernelFn, GateGainKernelError> {
    Ok(aarch64::process_gate_gain_aarch64_neon)
}

#[cfg(target_arch = "aarch64")]
fn soft_clip_neon_kernel() -> Result<SoftClipKernelFn, SoftClipBankKernelError> {
    Ok(aarch64::process_soft_clip_aarch64_neon)
}

#[cfg(not(target_arch = "aarch64"))]
fn delta_neon_kernel() -> Result<DeltaKernelFn, DeltaBankKernelError> {
    Err(DeltaBankKernelError::BackendUnavailable)
}

#[cfg(not(target_arch = "aarch64"))]
fn compressor_neon_kernel() -> Result<CompressorGainMixKernelFn, CompressorGainMixKernelError> {
    Err(CompressorGainMixKernelError::BackendUnavailable)
}

#[cfg(not(target_arch = "aarch64"))]
fn gate_neon_kernel() -> Result<GateGainKernelFn, GateGainKernelError> {
    Err(GateGainKernelError::BackendUnavailable)
}

#[cfg(not(target_arch = "aarch64"))]
fn soft_clip_neon_kernel() -> Result<SoftClipKernelFn, SoftClipBankKernelError> {
    Err(SoftClipBankKernelError::BackendUnavailable)
}

#[cfg(not(target_arch = "aarch64"))]
fn neon_kernel() -> Result<TptKernelFn, TptBankKernelError> {
    Err(TptBankKernelError::BackendUnavailable)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn x86_avx2_kernel() -> Result<TptKernelFn, TptBankKernelError> {
    if std::is_x86_feature_detected!("avx2") {
        Ok(x86::process_tpt_x86_avx2)
    } else {
        Err(TptBankKernelError::BackendUnavailable)
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn delta_x86_avx2_kernel() -> Result<DeltaKernelFn, DeltaBankKernelError> {
    if std::is_x86_feature_detected!("avx2") {
        Ok(x86::process_delta_x86_avx2)
    } else {
        Err(DeltaBankKernelError::BackendUnavailable)
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn compressor_x86_avx2_kernel() -> Result<CompressorGainMixKernelFn, CompressorGainMixKernelError> {
    if std::is_x86_feature_detected!("avx2") {
        Ok(x86::process_compressor_gain_mix_x86_avx2)
    } else {
        Err(CompressorGainMixKernelError::BackendUnavailable)
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn gate_x86_avx2_kernel() -> Result<GateGainKernelFn, GateGainKernelError> {
    if std::is_x86_feature_detected!("avx2") {
        Ok(x86::process_gate_gain_x86_avx2)
    } else {
        Err(GateGainKernelError::BackendUnavailable)
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn soft_clip_x86_avx2_kernel() -> Result<SoftClipKernelFn, SoftClipBankKernelError> {
    if std::is_x86_feature_detected!("avx2") {
        Ok(x86::process_soft_clip_x86_avx2)
    } else {
        Err(SoftClipBankKernelError::BackendUnavailable)
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn delta_x86_avx2_kernel() -> Result<DeltaKernelFn, DeltaBankKernelError> {
    Err(DeltaBankKernelError::BackendUnavailable)
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn compressor_x86_avx2_kernel() -> Result<CompressorGainMixKernelFn, CompressorGainMixKernelError> {
    Err(CompressorGainMixKernelError::BackendUnavailable)
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn gate_x86_avx2_kernel() -> Result<GateGainKernelFn, GateGainKernelError> {
    Err(GateGainKernelError::BackendUnavailable)
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn soft_clip_x86_avx2_kernel() -> Result<SoftClipKernelFn, SoftClipBankKernelError> {
    Err(SoftClipBankKernelError::BackendUnavailable)
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn x86_avx2_kernel() -> Result<TptKernelFn, TptBankKernelError> {
    Err(TptBankKernelError::BackendUnavailable)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn x86_avx2_fma_kernel() -> Result<TptKernelFn, TptBankKernelError> {
    if std::is_x86_feature_detected!("avx2") && std::is_x86_feature_detected!("fma") {
        Ok(x86::process_tpt_x86_avx2_fma)
    } else {
        Err(TptBankKernelError::BackendUnavailable)
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn delta_x86_avx2_fma_kernel() -> Result<DeltaKernelFn, DeltaBankKernelError> {
    if std::is_x86_feature_detected!("avx2") && std::is_x86_feature_detected!("fma") {
        Ok(x86::process_delta_x86_avx2_fma)
    } else {
        Err(DeltaBankKernelError::BackendUnavailable)
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn compressor_x86_avx2_fma_kernel()
-> Result<CompressorGainMixKernelFn, CompressorGainMixKernelError> {
    if std::is_x86_feature_detected!("avx2") && std::is_x86_feature_detected!("fma") {
        // The compressor graph intentionally has no FMA contractions; this remains a separately
        // prepared backend to preserve the explicit capability/program-key distinction.
        Ok(x86::process_compressor_gain_mix_x86_avx2_fma)
    } else {
        Err(CompressorGainMixKernelError::BackendUnavailable)
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn gate_x86_avx2_fma_kernel() -> Result<GateGainKernelFn, GateGainKernelError> {
    if std::is_x86_feature_detected!("avx2") && std::is_x86_feature_detected!("fma") {
        // This separately selected backend aliases the noncontracting base graph: zero FMA sites.
        Ok(x86::process_gate_gain_x86_avx2_fma)
    } else {
        Err(GateGainKernelError::BackendUnavailable)
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn soft_clip_x86_avx2_fma_kernel() -> Result<SoftClipKernelFn, SoftClipBankKernelError> {
    if std::is_x86_feature_detected!("avx2") && std::is_x86_feature_detected!("fma") {
        // The separately selected FMA backend aliases the frozen noncontracting AVX2 graph.
        Ok(x86::process_soft_clip_x86_avx2_fma)
    } else {
        Err(SoftClipBankKernelError::BackendUnavailable)
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn delta_x86_avx2_fma_kernel() -> Result<DeltaKernelFn, DeltaBankKernelError> {
    Err(DeltaBankKernelError::BackendUnavailable)
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn compressor_x86_avx2_fma_kernel()
-> Result<CompressorGainMixKernelFn, CompressorGainMixKernelError> {
    Err(CompressorGainMixKernelError::BackendUnavailable)
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn gate_x86_avx2_fma_kernel() -> Result<GateGainKernelFn, GateGainKernelError> {
    Err(GateGainKernelError::BackendUnavailable)
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn soft_clip_x86_avx2_fma_kernel() -> Result<SoftClipKernelFn, SoftClipBankKernelError> {
    Err(SoftClipBankKernelError::BackendUnavailable)
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn x86_avx2_fma_kernel() -> Result<TptKernelFn, TptBankKernelError> {
    Err(TptBankKernelError::BackendUnavailable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_compressor_gain_mix_preserves_the_frozen_graph() {
        let kernel =
            PreparedCompressorGainMixKernelV1::try_new(KernelBackendV1::Scalar).expect("scalar");
        let mut samples = [-0.375_f32];
        let gains = [0.5_f32];
        let mixes = [0.25_f32];
        let dry = samples[0];
        let p0 = dry * gains[0];
        let p1 = p0 - dry;
        let p2 = mixes[0] * p1;
        let expected = dry + p2;
        kernel
            .process_gain_mix(&mut samples, &gains, &mixes, &[0], &[0])
            .expect("process");
        assert_eq!(samples[0].to_bits(), expected.to_bits());

        let mut dry_selected = [-0.0_f32];
        kernel
            .process_gain_mix(&mut dry_selected, &[2.0], &[1.0], &[u32::MAX], &[0])
            .expect("dry identity");
        assert_eq!(dry_selected[0].to_bits(), (-0.0_f32).to_bits());
        let mut wet_selected = [0.25_f32];
        kernel
            .process_gain_mix(&mut wet_selected, &[0.5], &[0.0], &[0], &[u32::MAX])
            .expect("wet identity");
        assert_eq!(wet_selected[0].to_bits(), 0.125_f32.to_bits());
    }

    #[test]
    fn compressor_gain_mix_rejects_invalid_width_masks_and_overlap() {
        let kernel =
            PreparedCompressorGainMixKernelV1::try_new(KernelBackendV1::Scalar).expect("scalar");
        let mut samples = [0.0_f32];
        assert_eq!(
            kernel.process_gain_mix(&mut samples, &[], &[0.0], &[0], &[0]),
            Err(CompressorGainMixKernelError::LaneLength)
        );
        assert_eq!(
            kernel.process_gain_mix(&mut samples, &[1.0], &[0.0], &[1], &[0]),
            Err(CompressorGainMixKernelError::MaskValue)
        );
        assert_eq!(
            kernel.process_gain_mix(&mut samples, &[1.0], &[0.0], &[u32::MAX], &[u32::MAX]),
            Err(CompressorGainMixKernelError::MaskOverlap)
        );
    }

    #[test]
    fn scalar_gate_gain_is_one_multiply_with_exact_identity_selection() {
        let kernel = PreparedGateGainKernelV1::try_new(KernelBackendV1::Scalar).expect("scalar");
        let mut sample = [-0.375_f32];
        kernel
            .process_gain(&mut sample, &[0.5], &[0])
            .expect("process");
        assert_eq!(sample[0].to_bits(), (-0.1875_f32).to_bits());
        let mut signed_zero = [-0.0_f32];
        kernel
            .process_gain(&mut signed_zero, &[2.0], &[u32::MAX])
            .expect("identity");
        assert_eq!(signed_zero[0].to_bits(), (-0.0_f32).to_bits());
        assert_eq!(
            kernel.process_gain(&mut sample, &[], &[0]),
            Err(GateGainKernelError::LaneLength)
        );
        assert_eq!(
            kernel.process_gain(&mut sample, &[1.0], &[1]),
            Err(GateGainKernelError::MaskValue)
        );
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn x86_gate_gain_matches_scalar_and_fma_has_zero_contractions() {
        let Ok(base) = PreparedGateGainKernelV1::try_new(KernelBackendV1::X86Avx2) else {
            return;
        };
        let scalar = PreparedGateGainKernelV1::try_new(KernelBackendV1::Scalar).expect("scalar");
        let original = [0.125, -0.25, 0.375, -0.5, 0.625, -0.75, 0.875, -1.0];
        let gains = [0.5, 0.75, 1.25, 1.5, 0.25, 2.0, 0.125, 1.0];
        let identity = [0, u32::MAX, 0, 0, u32::MAX, 0, 0, u32::MAX];
        let mut expected = original;
        for lane in 0..8 {
            scalar
                .process_gain(
                    &mut expected[lane..=lane],
                    &gains[lane..=lane],
                    &identity[lane..=lane],
                )
                .expect("scalar lane");
        }
        let mut actual = original;
        base.process_gain(&mut actual, &gains, &identity)
            .expect("base");
        assert_eq!(
            actual.map(f32::to_bits),
            expected.map(f32::to_bits),
            "base AVX2 graph"
        );
        let Ok(fma) = PreparedGateGainKernelV1::try_new(KernelBackendV1::X86Avx2Fma) else {
            return;
        };
        let mut fma_actual = original;
        fma.process_gain(&mut fma_actual, &gains, &identity)
            .expect("FMA alias");
        assert_eq!(fma_actual.map(f32::to_bits), expected.map(f32::to_bits));
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn x86_compressor_gain_mix_matches_scalar_and_fma_is_noncontracting() {
        let Ok(base) = PreparedCompressorGainMixKernelV1::try_new(KernelBackendV1::X86Avx2) else {
            return;
        };
        let scalar =
            PreparedCompressorGainMixKernelV1::try_new(KernelBackendV1::Scalar).expect("scalar");
        let original = [0.125, -0.25, 0.375, -0.5, 0.625, -0.75, 0.875, -1.0];
        let gains = [0.5, 0.75, 1.25, 1.5, 0.25, 2.0, 0.125, 1.0];
        let mixes = [0.0, 1.0, 0.25, 0.5, 0.75, 0.125, 1.0, 0.5];
        let dry_mask = [0, u32::MAX, 0, 0, 0, u32::MAX, 0, 0];
        let wet_mask = [0, 0, u32::MAX, 0, 0, 0, u32::MAX, 0];
        let mut expected = original;
        for lane in 0..8 {
            scalar
                .process_gain_mix(
                    &mut expected[lane..=lane],
                    &gains[lane..=lane],
                    &mixes[lane..=lane],
                    &dry_mask[lane..=lane],
                    &wet_mask[lane..=lane],
                )
                .expect("scalar lane");
        }
        let mut base_actual = original;
        base.process_gain_mix(&mut base_actual, &gains, &mixes, &dry_mask, &wet_mask)
            .expect("base");
        assert_eq!(base_actual.map(f32::to_bits), expected.map(f32::to_bits));
        let Ok(fma) = PreparedCompressorGainMixKernelV1::try_new(KernelBackendV1::X86Avx2Fma)
        else {
            return;
        };
        let mut fma_actual = original;
        fma.process_gain_mix(&mut fma_actual, &gains, &mixes, &dry_mask, &wet_mask)
            .expect("fma alias");
        assert_eq!(fma_actual.map(f32::to_bits), expected.map(f32::to_bits));
    }

    #[test]
    fn scalar_delta_preserves_the_frozen_noncontracting_graph() {
        let kernel = PreparedDeltaBankKernelV1::try_new(KernelBackendV1::Scalar).expect("scalar");
        let mut sample = [0.375_f32];
        let a = [1.0_f32];
        let n0 = [0.75_f32];
        let d0 = [1.0_f32];
        let n1 = [0.125_f32];
        let d1 = [0.25_f32];
        let n2 = [-0.0625_f32];
        let d2 = [-0.03125_f32];
        let mut x1 = [0.03125_f32];
        let mut x2 = [-0.015625_f32];
        let mut y1 = [0.0625_f32];
        let mut y2 = [-0.125_f32];
        let mask = [0_u32];
        let old_x1 = x1[0];
        let old_x2 = x2[0];
        let old_y1 = y1[0];
        let old_y2 = y2[0];
        let t0 = a[0] * sample[0];
        let dx = old_x1 - t0;
        let t1 = a[0] * old_x1;
        let t2 = old_x2 - t1;
        let t3 = a[0] * dx;
        let ddx = t2 - t3;
        let p0 = n0[0] * sample[0];
        let p1 = n1[0] * dx;
        let s0 = p0 + p1;
        let p2 = n2[0] * ddx;
        let num = s0 + p2;
        let q0 = a[0] * d1[0];
        let scale = (d0[0] - q0) + d2[0];
        let q1 = a[0] * d2[0];
        let q2 = (d1[0] - q1) - q1;
        let h0 = q2 * old_y1;
        let h1 = d2[0] * old_y2;
        let history = h0 + h1;
        let expected = (num - history) / scale;
        kernel
            .process_delta(
                &mut sample,
                &a,
                &n0,
                &d0,
                &n1,
                &d1,
                &n2,
                &d2,
                &mut x1,
                &mut x2,
                &mut y1,
                &mut y2,
                &mask,
            )
            .expect("process");
        assert_eq!(sample[0].to_bits(), expected.to_bits());
        assert_eq!(x1[0].to_bits(), 0.375_f32.to_bits());
        assert_eq!(x2[0].to_bits(), old_x1.to_bits());
        assert_eq!(y1[0].to_bits(), expected.to_bits());
        assert_eq!(y2[0].to_bits(), old_y1.to_bits());
    }

    #[test]
    fn identity_mask_returns_dry_bits_and_warms_delta_history() {
        let kernel = PreparedDeltaBankKernelV1::try_new(KernelBackendV1::Scalar).expect("scalar");
        let mut sample = [-0.0_f32];
        let coefficients = [1.0_f32];
        let zero = [0.0_f32];
        let mut x1 = [0.25_f32];
        let mut x2 = [-0.5_f32];
        let mut y1 = [0.75_f32];
        let mut y2 = [-0.125_f32];
        kernel
            .process_delta(
                &mut sample,
                &coefficients,
                &coefficients,
                &coefficients,
                &zero,
                &zero,
                &zero,
                &zero,
                &mut x1,
                &mut x2,
                &mut y1,
                &mut y2,
                &[u32::MAX],
            )
            .expect("process");
        assert_eq!(sample[0].to_bits(), (-0.0_f32).to_bits());
        assert_eq!(x1[0].to_bits(), (-0.0_f32).to_bits());
        assert_eq!(x2[0].to_bits(), 0.25_f32.to_bits());
        assert_eq!(y1[0].to_bits(), (-0.0_f32).to_bits());
        assert_eq!(y2[0].to_bits(), 0.25_f32.to_bits());
    }

    #[test]
    fn prepared_delta_rejects_wrong_lengths_and_masks() {
        let kernel = PreparedDeltaBankKernelV1::try_new(KernelBackendV1::Scalar).expect("scalar");
        let mut sample = [0.0_f32];
        let coefficient = [0.0_f32];
        let mut state = [0.0_f32];
        assert_eq!(
            kernel.process_delta(
                &mut sample,
                &[],
                &coefficient,
                &coefficient,
                &coefficient,
                &coefficient,
                &coefficient,
                &coefficient,
                &mut state,
                &mut [0.0],
                &mut [0.0],
                &mut [0.0],
                &[0],
            ),
            Err(DeltaBankKernelError::LaneLength)
        );
        assert_eq!(
            kernel.process_delta(
                &mut sample,
                &coefficient,
                &coefficient,
                &coefficient,
                &coefficient,
                &coefficient,
                &coefficient,
                &coefficient,
                &mut state,
                &mut [0.0],
                &mut [0.0],
                &mut [0.0],
                &[1],
            ),
            Err(DeltaBankKernelError::MaskValue)
        );
    }

    #[test]
    fn unsupported_delta_backend_fails_at_preparation() {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        assert_eq!(
            PreparedDeltaBankKernelV1::try_new(KernelBackendV1::WasmSimd128).err(),
            Some(DeltaBankKernelError::BackendUnavailable)
        );
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        assert_eq!(
            PreparedDeltaBankKernelV1::try_new(KernelBackendV1::X86Avx2).err(),
            Some(DeltaBankKernelError::BackendUnavailable)
        );
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn x86_base_and_fma_are_bit_identical_to_scalar_delta() {
        let Ok(base) = PreparedDeltaBankKernelV1::try_new(KernelBackendV1::X86Avx2) else {
            return;
        };
        let scalar = PreparedDeltaBankKernelV1::try_new(KernelBackendV1::Scalar).expect("scalar");
        let input = [0.125, -0.25, 0.375, -0.5, 0.625, -0.75, 0.875, -1.0];
        let a = [1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
        let n0 = [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2];
        let d0 = [1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7];
        let n1 = [0.1, -0.1, 0.05, -0.05, 0.125, -0.125, 0.25, -0.25];
        let d1 = [0.2, -0.2, 0.15, -0.15, 0.1, -0.1, 0.05, -0.05];
        let n2 = [-0.05, 0.05, -0.025, 0.025, -0.0625, 0.0625, -0.125, 0.125];
        let d2 = [-0.1, 0.1, -0.075, 0.075, -0.05, 0.05, -0.025, 0.025];
        let old_x1 = [0.01, 0.02, 0.03, 0.04, 0.05, 0.06, 0.07, 0.08];
        let old_x2 = [-0.01, -0.02, -0.03, -0.04, -0.05, -0.06, -0.07, -0.08];
        let old_y1 = [0.08, 0.07, 0.06, 0.05, 0.04, 0.03, 0.02, 0.01];
        let old_y2 = [-0.08, -0.07, -0.06, -0.05, -0.04, -0.03, -0.02, -0.01];
        let mask = [0, u32::MAX, 0, u32::MAX, 0, u32::MAX, 0, u32::MAX];

        let mut scalar_samples = input;
        let mut scalar_x1 = old_x1;
        let mut scalar_x2 = old_x2;
        let mut scalar_y1 = old_y1;
        let mut scalar_y2 = old_y2;
        for lane in 0..8 {
            let mut sample = [scalar_samples[lane]];
            let mut x1 = [scalar_x1[lane]];
            let mut x2 = [scalar_x2[lane]];
            let mut y1 = [scalar_y1[lane]];
            let mut y2 = [scalar_y2[lane]];
            scalar
                .process_delta(
                    &mut sample,
                    &[a[lane]],
                    &[n0[lane]],
                    &[d0[lane]],
                    &[n1[lane]],
                    &[d1[lane]],
                    &[n2[lane]],
                    &[d2[lane]],
                    &mut x1,
                    &mut x2,
                    &mut y1,
                    &mut y2,
                    &[mask[lane]],
                )
                .expect("scalar lane");
            scalar_samples[lane] = sample[0];
            scalar_x1[lane] = x1[0];
            scalar_x2[lane] = x2[0];
            scalar_y1[lane] = y1[0];
            scalar_y2[lane] = y2[0];
        }

        let mut base_samples = input;
        let mut base_x1 = old_x1;
        let mut base_x2 = old_x2;
        let mut base_y1 = old_y1;
        let mut base_y2 = old_y2;
        base.process_delta(
            &mut base_samples,
            &a,
            &n0,
            &d0,
            &n1,
            &d1,
            &n2,
            &d2,
            &mut base_x1,
            &mut base_x2,
            &mut base_y1,
            &mut base_y2,
            &mask,
        )
        .expect("base");
        assert_eq!(
            base_samples.map(f32::to_bits),
            scalar_samples.map(f32::to_bits)
        );
        assert_eq!(base_x1.map(f32::to_bits), scalar_x1.map(f32::to_bits));
        assert_eq!(base_x2.map(f32::to_bits), scalar_x2.map(f32::to_bits));
        assert_eq!(base_y1.map(f32::to_bits), scalar_y1.map(f32::to_bits));
        assert_eq!(base_y2.map(f32::to_bits), scalar_y2.map(f32::to_bits));

        let Ok(fma) = PreparedDeltaBankKernelV1::try_new(KernelBackendV1::X86Avx2Fma) else {
            return;
        };
        let mut fma_samples = input;
        let mut fma_x1 = old_x1;
        let mut fma_x2 = old_x2;
        let mut fma_y1 = old_y1;
        let mut fma_y2 = old_y2;
        fma.process_delta(
            &mut fma_samples,
            &a,
            &n0,
            &d0,
            &n1,
            &d1,
            &n2,
            &d2,
            &mut fma_x1,
            &mut fma_x2,
            &mut fma_y1,
            &mut fma_y2,
            &mask,
        )
        .expect("fma");
        assert_eq!(
            fma_samples.map(f32::to_bits),
            scalar_samples.map(f32::to_bits)
        );
        assert_eq!(fma_x1.map(f32::to_bits), scalar_x1.map(f32::to_bits));
        assert_eq!(fma_x2.map(f32::to_bits), scalar_x2.map(f32::to_bits));
        assert_eq!(fma_y1.map(f32::to_bits), scalar_y1.map(f32::to_bits));
        assert_eq!(fma_y2.map(f32::to_bits), scalar_y2.map(f32::to_bits));
    }

    #[test]
    fn scalar_kernel_preserves_the_frozen_incremental_graph() {
        let kernel = PreparedTptBankKernelV1::try_new(KernelBackendV1::Scalar).expect("scalar");
        let mut sample = [0.375_f32];
        let c1 = [0.125_f32];
        let a2 = [0.25_f32];
        let a3 = [0.0625_f32];
        let k = [core::f32::consts::SQRT_2];
        let mut s1 = [0.03125_f32];
        let mut s2 = [-0.015625_f32];
        let mask = [u32::MAX];
        let old_s1 = s1[0];
        let old_s2 = s2[0];
        let v3 = sample[0] - old_s2;
        let p1 = a2[0] * v3;
        let p2 = c1[0] * old_s1;
        let d1 = p1 - p2;
        let v1 = old_s1 + d1;
        let p3 = a2[0] * old_s1;
        let p4 = a3[0] * v3;
        let d2 = p3 + p4;
        let v2 = old_s2 + d2;
        let expected = (sample[0] - k[0] * v1) - v2;
        let expected_s1 = old_s1 + (d1 + d1);
        let expected_s2 = old_s2 + (d2 + d2);
        kernel
            .process_tpt(&mut sample, &c1, &a2, &a3, &k, &mut s1, &mut s2, &mask)
            .expect("process");
        assert_eq!(sample[0].to_bits(), expected.to_bits());
        assert_eq!(s1[0].to_bits(), expected_s1.to_bits());
        assert_eq!(s2[0].to_bits(), expected_s2.to_bits());
    }

    #[test]
    fn prepared_kernel_rejects_every_shape_mismatch() {
        let kernel = PreparedTptBankKernelV1::try_new(KernelBackendV1::Scalar).expect("scalar");
        let mut sample = [0.0];
        let one = [0.0];
        let mut state = [0.0];
        assert_eq!(
            kernel.process_tpt(
                &mut sample,
                &[],
                &one,
                &one,
                &one,
                &mut state,
                &mut [0.0],
                &[0],
            ),
            Err(TptBankKernelError::LaneLength)
        );
        assert_eq!(
            kernel.process_tpt(
                &mut sample,
                &one,
                &one,
                &one,
                &one,
                &mut state,
                &mut [0.0],
                &[1],
            ),
            Err(TptBankKernelError::MaskValue)
        );
    }

    #[test]
    fn preparation_is_the_only_safe_architecture_gate() {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            assert_eq!(
                PreparedTptBankKernelV1::try_new(KernelBackendV1::WasmSimd128).err(),
                Some(TptBankKernelError::BackendUnavailable)
            );
            assert_eq!(
                PreparedTptBankKernelV1::try_new(KernelBackendV1::Aarch64Neon).err(),
                Some(TptBankKernelError::BackendUnavailable)
            );
            assert_eq!(
                PreparedTptBankKernelV1::try_new(KernelBackendV1::X86Avx2).is_ok(),
                std::is_x86_feature_detected!("avx2")
            );
            assert_eq!(
                PreparedTptBankKernelV1::try_new(KernelBackendV1::X86Avx2Fma).is_ok(),
                std::is_x86_feature_detected!("avx2") && std::is_x86_feature_detected!("fma")
            );
        }
        #[cfg(target_arch = "aarch64")]
        {
            assert!(PreparedTptBankKernelV1::try_new(KernelBackendV1::Aarch64Neon).is_ok());
        }
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        {
            assert!(PreparedTptBankKernelV1::try_new(KernelBackendV1::WasmSimd128).is_ok());
        }
    }

    fn soft_clip_coefficients() -> [f32; SOFT_CLIP_HISTORY_WORDS] {
        let mut coefficients = [0.0; SOFT_CLIP_HISTORY_WORDS];
        for tap in SOFT_CLIP_NONZERO_TAPS {
            coefficients[tap] = if tap == 31 {
                0.5
            } else {
                (tap as f32 - 31.0) * 0.003_125
            };
        }
        coefficients
    }

    fn process_soft_clip_scalar_tracks(
        samples: &mut [f32; 8],
        coefficients: &[f32; SOFT_CLIP_HISTORY_WORDS],
        cursors: &mut [u32; 8],
        interpolation: &mut [f32; SOFT_CLIP_HISTORY_WORDS * 8],
        decimation: &mut [f32; SOFT_CLIP_HISTORY_WORDS * 8],
    ) {
        let scalar =
            PreparedSoftClipBankKernelV1::try_new(KernelBackendV1::Scalar).expect("scalar");
        for lane in 0..8 {
            let mut sample = [samples[lane]];
            let mut cursor = [cursors[lane]];
            let mut lane_interpolation = [0.0; SOFT_CLIP_HISTORY_WORDS];
            let mut lane_decimation = [0.0; SOFT_CLIP_HISTORY_WORDS];
            for word in 0..SOFT_CLIP_HISTORY_WORDS {
                lane_interpolation[word] = interpolation[word * 8 + lane];
                lane_decimation[word] = decimation[word * 8 + lane];
            }
            scalar
                .process_phase(
                    &mut sample,
                    coefficients,
                    &mut cursor,
                    &mut lane_interpolation,
                    &mut lane_decimation,
                )
                .expect("scalar phase");
            samples[lane] = sample[0];
            cursors[lane] = cursor[0];
            for word in 0..SOFT_CLIP_HISTORY_WORDS {
                interpolation[word * 8 + lane] = lane_interpolation[word];
                decimation[word * 8 + lane] = lane_decimation[word];
            }
        }
    }

    #[test]
    fn soft_clip_scalar_phase_preserves_cubic_zero_and_error_contracts() {
        let kernel =
            PreparedSoftClipBankKernelV1::try_new(KernelBackendV1::Scalar).expect("scalar");
        let mut coefficients = [0.0; SOFT_CLIP_HISTORY_WORDS];
        coefficients[31] = 1.0;
        let mut samples = [0.0];
        let mut cursors = [0];
        let mut interpolation = [0.0; SOFT_CLIP_HISTORY_WORDS];
        let mut decimation = [0.0; SOFT_CLIP_HISTORY_WORDS];
        interpolation[32] = 2.0;
        kernel
            .process_phase(
                &mut samples,
                &coefficients,
                &mut cursors,
                &mut interpolation,
                &mut decimation,
            )
            .expect("phase");
        assert_eq!(decimation[0].to_bits(), (2.0_f32 / 3.0_f32).to_bits());
        assert_eq!(cursors, [1]);

        let mut zero = [-0.0_f32];
        let mut zero_cursor = [0];
        let mut zero_interpolation = [0.0; SOFT_CLIP_HISTORY_WORDS];
        let mut zero_decimation = [0.0; SOFT_CLIP_HISTORY_WORDS];
        kernel
            .process_phase(
                &mut zero,
                &coefficients,
                &mut zero_cursor,
                &mut zero_interpolation,
                &mut zero_decimation,
            )
            .expect("zero phase");
        assert_eq!(zero[0].to_bits(), 0.0_f32.to_bits());

        assert_eq!(
            kernel.process_phase(
                &mut samples,
                &coefficients[..62],
                &mut cursors,
                &mut interpolation,
                &mut decimation,
            ),
            Err(SoftClipBankKernelError::CoefficientTable)
        );
        assert_eq!(
            kernel.process_phase(
                &mut samples,
                &coefficients,
                &mut cursors,
                &mut interpolation[..62],
                &mut decimation,
            ),
            Err(SoftClipBankKernelError::HistoryLength)
        );
        assert_eq!(
            kernel.process_phase(
                &mut samples,
                &coefficients,
                &mut [63],
                &mut interpolation,
                &mut decimation,
            ),
            Err(SoftClipBankKernelError::Cursor)
        );
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn x86_soft_clip_phase_matches_scalar_with_lane_local_cursors() {
        let Ok(base) = PreparedSoftClipBankKernelV1::try_new(KernelBackendV1::X86Avx2) else {
            return;
        };
        let coefficients = soft_clip_coefficients();
        let mut expected_samples = core::array::from_fn(|lane| (lane as f32 - 3.5) * 0.125);
        let mut expected_cursors = core::array::from_fn(|lane| ((lane * 7) % 63) as u32);
        let mut expected_interpolation = core::array::from_fn(|word| (word % 11) as f32 * 0.001);
        let mut expected_decimation = core::array::from_fn(|word| -((word % 13) as f32) * 0.0015);
        let mut actual_samples = expected_samples;
        let mut actual_cursors = expected_cursors;
        let mut actual_interpolation = expected_interpolation;
        let mut actual_decimation = expected_decimation;
        for phase in 0..71 {
            for lane in 0..8 {
                let input = ((phase * 17 + lane * 5) as f32 * 0.03125).sin() * 0.75;
                expected_samples[lane] = input;
                actual_samples[lane] = input;
            }
            process_soft_clip_scalar_tracks(
                &mut expected_samples,
                &coefficients,
                &mut expected_cursors,
                &mut expected_interpolation,
                &mut expected_decimation,
            );
            base.process_phase(
                &mut actual_samples,
                &coefficients,
                &mut actual_cursors,
                &mut actual_interpolation,
                &mut actual_decimation,
            )
            .expect("base phase");
        }
        assert_eq!(
            actual_samples.map(f32::to_bits),
            expected_samples.map(f32::to_bits)
        );
        assert_eq!(actual_cursors, expected_cursors);
        assert_eq!(
            actual_interpolation.map(f32::to_bits),
            expected_interpolation.map(f32::to_bits)
        );
        assert_eq!(
            actual_decimation.map(f32::to_bits),
            expected_decimation.map(f32::to_bits)
        );

        let Ok(fma) = PreparedSoftClipBankKernelV1::try_new(KernelBackendV1::X86Avx2Fma) else {
            return;
        };
        let mut fma_samples = actual_samples;
        let mut fma_cursors = actual_cursors;
        let mut fma_interpolation = actual_interpolation;
        let mut fma_decimation = actual_decimation;
        fma.process_phase(
            &mut fma_samples,
            &coefficients,
            &mut fma_cursors,
            &mut fma_interpolation,
            &mut fma_decimation,
        )
        .expect("fma phase");
        base.process_phase(
            &mut actual_samples,
            &coefficients,
            &mut actual_cursors,
            &mut actual_interpolation,
            &mut actual_decimation,
        )
        .expect("base phase");
        assert_eq!(
            fma_samples.map(f32::to_bits),
            actual_samples.map(f32::to_bits)
        );
        assert_eq!(fma_cursors, actual_cursors);
        assert_eq!(
            fma_interpolation.map(f32::to_bits),
            actual_interpolation.map(f32::to_bits)
        );
        assert_eq!(
            fma_decimation.map(f32::to_bits),
            actual_decimation.map(f32::to_bits)
        );
    }

    #[test]
    fn unsupported_soft_clip_backend_fails_at_preparation() {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        assert_eq!(
            PreparedSoftClipBankKernelV1::try_new(KernelBackendV1::WasmSimd128).err(),
            Some(SoftClipBankKernelError::BackendUnavailable)
        );
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        assert_eq!(
            PreparedSoftClipBankKernelV1::try_new(KernelBackendV1::X86Avx2).err(),
            Some(SoftClipBankKernelError::BackendUnavailable)
        );
    }
}
