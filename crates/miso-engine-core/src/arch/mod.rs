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
type BiquadKernelFn = for<'a> fn(BiquadKernelBlock<'a>);

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

/// Preparation or shape failure for an architecture-owned direct-form-I biquad bank kernel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BiquadBankKernelError {
    /// The selected semantic backend cannot execute on this build/current processor.
    BackendUnavailable,
    /// Every slice must contain exactly the backend's logical lane count.
    LaneLength,
    /// An identity selection mask was neither all-zero nor all-one.
    MaskValue,
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

/// A safe, immutable prepared direct-form-I biquad bank dispatch token.
///
/// Construction performs all architecture feature detection. Render callers only provide exact
/// lane slices, so this token enters private architecture-specific functions without detection or
/// an unsafe caller contract.
#[derive(Clone, Copy)]
pub struct PreparedBiquadBankKernelV1 {
    backend: KernelBackendV1,
    process: BiquadKernelFn,
}

impl PreparedBiquadBankKernelV1 {
    /// Prepare the exact semantic backend when it is executable by this artifact and processor.
    pub fn try_new(backend: KernelBackendV1) -> Result<Self, BiquadBankKernelError> {
        let process = match backend {
            KernelBackendV1::Scalar => scalar::process_biquad_scalar,
            KernelBackendV1::WasmSimd128 => biquad_wasm_kernel()?,
            KernelBackendV1::Aarch64Neon => biquad_neon_kernel()?,
            KernelBackendV1::X86Avx2 => biquad_x86_avx2_kernel()?,
            KernelBackendV1::X86Avx2Fma => biquad_x86_avx2_fma_kernel()?,
        };
        Ok(Self { backend, process })
    }

    /// Semantic backend whose target preconditions were proved at preparation.
    #[must_use]
    pub const fn backend(self) -> KernelBackendV1 {
        self.backend
    }

    /// Execute one direct-form-I sample across one coefficient/state bank.
    ///
    /// Coefficient slices carry B0/B1/B2/A1/A2. `identity_mask` uses `u32::MAX` for exact
    /// identity/warm-history behavior and zero for the frozen recurrence. Every slice must have
    /// exactly the backend lane count; the mask accepts only those two bit patterns.
    #[allow(clippy::too_many_arguments)]
    pub fn process_biquad(
        self,
        samples: &mut [f32],
        b0: &[f32],
        b1: &[f32],
        b2: &[f32],
        a1: &[f32],
        a2: &[f32],
        x1: &mut [f32],
        x2: &mut [f32],
        y1: &mut [f32],
        y2: &mut [f32],
        identity_mask: &[u32],
    ) -> Result<(), BiquadBankKernelError> {
        let lanes = self.backend.lanes() as usize;
        if [
            samples.len(),
            b0.len(),
            b1.len(),
            b2.len(),
            a1.len(),
            a2.len(),
            x1.len(),
            x2.len(),
            y1.len(),
            y2.len(),
            identity_mask.len(),
        ]
        .into_iter()
        .any(|length| length != lanes)
        {
            return Err(BiquadBankKernelError::LaneLength);
        }
        if identity_mask
            .iter()
            .any(|mask| !matches!(*mask, 0 | u32::MAX))
        {
            return Err(BiquadBankKernelError::MaskValue);
        }
        (self.process)(BiquadKernelBlock {
            samples,
            b0,
            b1,
            b2,
            a1,
            a2,
            x1,
            x2,
            y1,
            y2,
            identity_mask,
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

pub(super) struct BiquadKernelBlock<'a> {
    pub(super) samples: &'a mut [f32],
    pub(super) b0: &'a [f32],
    pub(super) b1: &'a [f32],
    pub(super) b2: &'a [f32],
    pub(super) a1: &'a [f32],
    pub(super) a2: &'a [f32],
    pub(super) x1: &'a mut [f32],
    pub(super) x2: &'a mut [f32],
    pub(super) y1: &'a mut [f32],
    pub(super) y2: &'a mut [f32],
    pub(super) identity_mask: &'a [u32],
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
fn wasm_kernel() -> Result<TptKernelFn, TptBankKernelError> {
    Ok(wasm32::process_tpt_wasm_simd128)
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
fn biquad_wasm_kernel() -> Result<BiquadKernelFn, BiquadBankKernelError> {
    Ok(wasm32::process_biquad_wasm_simd128)
}

#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
fn biquad_wasm_kernel() -> Result<BiquadKernelFn, BiquadBankKernelError> {
    Err(BiquadBankKernelError::BackendUnavailable)
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
fn biquad_neon_kernel() -> Result<BiquadKernelFn, BiquadBankKernelError> {
    Ok(aarch64::process_biquad_aarch64_neon)
}

#[cfg(not(target_arch = "aarch64"))]
fn biquad_neon_kernel() -> Result<BiquadKernelFn, BiquadBankKernelError> {
    Err(BiquadBankKernelError::BackendUnavailable)
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
fn biquad_x86_avx2_kernel() -> Result<BiquadKernelFn, BiquadBankKernelError> {
    if std::is_x86_feature_detected!("avx2") {
        Ok(x86::process_biquad_x86_avx2)
    } else {
        Err(BiquadBankKernelError::BackendUnavailable)
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn biquad_x86_avx2_kernel() -> Result<BiquadKernelFn, BiquadBankKernelError> {
    Err(BiquadBankKernelError::BackendUnavailable)
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
fn biquad_x86_avx2_fma_kernel() -> Result<BiquadKernelFn, BiquadBankKernelError> {
    if std::is_x86_feature_detected!("avx2") && std::is_x86_feature_detected!("fma") {
        Ok(x86::process_biquad_x86_avx2_fma)
    } else {
        Err(BiquadBankKernelError::BackendUnavailable)
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn biquad_x86_avx2_fma_kernel() -> Result<BiquadKernelFn, BiquadBankKernelError> {
    Err(BiquadBankKernelError::BackendUnavailable)
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn x86_avx2_fma_kernel() -> Result<TptKernelFn, TptBankKernelError> {
    Err(TptBankKernelError::BackendUnavailable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_biquad_preserves_the_frozen_direct_form_i_graph() {
        let kernel = PreparedBiquadBankKernelV1::try_new(KernelBackendV1::Scalar).expect("scalar");
        let mut sample = [0.375_f32];
        let b0 = [0.75_f32];
        let b1 = [0.125_f32];
        let b2 = [-0.0625_f32];
        let a1 = [0.25_f32];
        let a2 = [-0.03125_f32];
        let mut x1 = [0.03125_f32];
        let mut x2 = [-0.015625_f32];
        let mut y1 = [0.0625_f32];
        let mut y2 = [-0.125_f32];
        let mask = [0_u32];
        let old_x1 = x1[0];
        let old_x2 = x2[0];
        let old_y1 = y1[0];
        let old_y2 = y2[0];
        let p0 = b0[0] * sample[0];
        let p1 = b1[0] * old_x1;
        let s0 = p0 + p1;
        let p2 = b2[0] * old_x2;
        let s1 = s0 + p2;
        let p3 = a1[0] * old_y1;
        let s2 = s1 - p3;
        let p4 = a2[0] * old_y2;
        let expected = s2 - p4;
        kernel
            .process_biquad(
                &mut sample,
                &b0,
                &b1,
                &b2,
                &a1,
                &a2,
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
    fn identity_mask_returns_dry_bits_and_warms_biquad_history() {
        let kernel = PreparedBiquadBankKernelV1::try_new(KernelBackendV1::Scalar).expect("scalar");
        let mut sample = [-0.0_f32];
        let coefficients = [1.0_f32];
        let mut x1 = [0.25_f32];
        let mut x2 = [-0.5_f32];
        let mut y1 = [0.75_f32];
        let mut y2 = [-0.125_f32];
        kernel
            .process_biquad(
                &mut sample,
                &coefficients,
                &coefficients,
                &coefficients,
                &coefficients,
                &coefficients,
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
    fn prepared_biquad_rejects_wrong_lengths_and_masks() {
        let kernel = PreparedBiquadBankKernelV1::try_new(KernelBackendV1::Scalar).expect("scalar");
        let mut sample = [0.0_f32];
        let coefficient = [0.0_f32];
        let mut state = [0.0_f32];
        assert_eq!(
            kernel.process_biquad(
                &mut sample,
                &[],
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
            Err(BiquadBankKernelError::LaneLength)
        );
        assert_eq!(
            kernel.process_biquad(
                &mut sample,
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
            Err(BiquadBankKernelError::MaskValue)
        );
    }

    #[test]
    fn unsupported_biquad_backend_fails_at_preparation() {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        assert_eq!(
            PreparedBiquadBankKernelV1::try_new(KernelBackendV1::WasmSimd128).err(),
            Some(BiquadBankKernelError::BackendUnavailable)
        );
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        assert_eq!(
            PreparedBiquadBankKernelV1::try_new(KernelBackendV1::X86Avx2).err(),
            Some(BiquadBankKernelError::BackendUnavailable)
        );
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn x86_base_matches_scalar_and_fma_stays_within_the_frozen_tolerance() {
        let Ok(base) = PreparedBiquadBankKernelV1::try_new(KernelBackendV1::X86Avx2) else {
            return;
        };
        let scalar = PreparedBiquadBankKernelV1::try_new(KernelBackendV1::Scalar).expect("scalar");
        let input = [0.125, -0.25, 0.375, -0.5, 0.625, -0.75, 0.875, -1.0];
        let b0 = [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2];
        let b1 = [0.1, -0.1, 0.05, -0.05, 0.125, -0.125, 0.25, -0.25];
        let b2 = [-0.05, 0.05, -0.025, 0.025, -0.0625, 0.0625, -0.125, 0.125];
        let a1 = [0.2, -0.2, 0.15, -0.15, 0.1, -0.1, 0.05, -0.05];
        let a2 = [-0.1, 0.1, -0.075, 0.075, -0.05, 0.05, -0.025, 0.025];
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
                .process_biquad(
                    &mut sample,
                    &[b0[lane]],
                    &[b1[lane]],
                    &[b2[lane]],
                    &[a1[lane]],
                    &[a2[lane]],
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
        base.process_biquad(
            &mut base_samples,
            &b0,
            &b1,
            &b2,
            &a1,
            &a2,
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

        let Ok(fma) = PreparedBiquadBankKernelV1::try_new(KernelBackendV1::X86Avx2Fma) else {
            return;
        };
        let mut fma_samples = input;
        let mut fma_x1 = old_x1;
        let mut fma_x2 = old_x2;
        let mut fma_y1 = old_y1;
        let mut fma_y2 = old_y2;
        fma.process_biquad(
            &mut fma_samples,
            &b0,
            &b1,
            &b2,
            &a1,
            &a2,
            &mut fma_x1,
            &mut fma_x2,
            &mut fma_y1,
            &mut fma_y2,
            &mask,
        )
        .expect("fma");
        for (candidate, reference) in fma_samples
            .into_iter()
            .chain(fma_x1)
            .chain(fma_x2)
            .chain(fma_y1)
            .chain(fma_y2)
            .zip(
                scalar_samples
                    .into_iter()
                    .chain(scalar_x1)
                    .chain(scalar_x2)
                    .chain(scalar_y1)
                    .chain(scalar_y2),
            )
        {
            assert!(
                (candidate - reference).abs() <= 1e-6 + 2e-5 * reference.abs(),
                "candidate={candidate:?}, reference={reference:?}"
            );
        }
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
}
