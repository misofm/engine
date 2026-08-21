//! x86 AVX2 and separately gated AVX2+FMA TPT kernels.

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use super::{BiquadKernelBlock, TptKernelBlock};

#[inline(never)]
#[target_feature(enable = "avx2")]
unsafe fn process_tpt_x86_avx2_inner(block: TptKernelBlock<'_>) {
    // SAFETY: `PreparedTptBankKernelV1` admits this function only after AVX2 detection and validates
    // all slices as exactly eight lanes. Unaligned loads/stores stay within those slices.
    unsafe {
        let x = _mm256_loadu_ps(block.samples.as_ptr());
        let c1 = _mm256_loadu_ps(block.c1.as_ptr());
        let a2 = _mm256_loadu_ps(block.a2.as_ptr());
        let a3 = _mm256_loadu_ps(block.a3.as_ptr());
        let k = _mm256_loadu_ps(block.k.as_ptr());
        let old_s1 = _mm256_loadu_ps(block.s1.as_ptr());
        let old_s2 = _mm256_loadu_ps(block.s2.as_ptr());
        let v3 = _mm256_sub_ps(x, old_s2);
        let p1 = _mm256_mul_ps(a2, v3);
        let p2 = _mm256_mul_ps(c1, old_s1);
        let d1 = _mm256_sub_ps(p1, p2);
        let v1 = _mm256_add_ps(old_s1, d1);
        let p3 = _mm256_mul_ps(a2, old_s1);
        let p4 = _mm256_mul_ps(a3, v3);
        let d2 = _mm256_add_ps(p3, p4);
        let v2 = _mm256_add_ps(old_s2, d2);
        let n1 = _mm256_add_ps(old_s1, _mm256_add_ps(d1, d1));
        let n2 = _mm256_add_ps(old_s2, _mm256_add_ps(d2, d2));
        let th = _mm256_sub_ps(x, _mm256_mul_ps(k, v1));
        let high = _mm256_sub_ps(th, v2);
        let mask = _mm256_castsi256_ps(_mm256_loadu_si256(
            block.high_pass_mask.as_ptr().cast::<__m256i>(),
        ));
        let output = _mm256_blendv_ps(v2, high, mask);
        _mm256_storeu_ps(block.s1.as_mut_ptr(), n1);
        _mm256_storeu_ps(block.s2.as_mut_ptr(), n2);
        _mm256_storeu_ps(block.samples.as_mut_ptr(), output);
    }
}

#[inline(never)]
pub(super) fn process_tpt_x86_avx2(block: TptKernelBlock<'_>) {
    // SAFETY: this safe shim is retained only in a token created after AVX2 runtime detection.
    unsafe { process_tpt_x86_avx2_inner(block) }
}

#[inline(never)]
#[target_feature(enable = "avx2,fma")]
unsafe fn process_tpt_x86_avx2_fma_inner(block: TptKernelBlock<'_>) {
    // SAFETY: the prepared token proves AVX2+FMA and all validated slices contain eight lanes.
    unsafe {
        let x = _mm256_loadu_ps(block.samples.as_ptr());
        let c1 = _mm256_loadu_ps(block.c1.as_ptr());
        let a2 = _mm256_loadu_ps(block.a2.as_ptr());
        let a3 = _mm256_loadu_ps(block.a3.as_ptr());
        let k = _mm256_loadu_ps(block.k.as_ptr());
        let old_s1 = _mm256_loadu_ps(block.s1.as_ptr());
        let old_s2 = _mm256_loadu_ps(block.s2.as_ptr());
        let v3 = _mm256_sub_ps(x, old_s2);
        let p2 = _mm256_mul_ps(c1, old_s1);
        let p4 = _mm256_mul_ps(a3, v3);
        let d1 = _mm256_fmsub_ps(a2, v3, p2);
        let v1 = _mm256_add_ps(old_s1, d1);
        let d2 = _mm256_fmadd_ps(a2, old_s1, p4);
        let v2 = _mm256_add_ps(old_s2, d2);
        let n1 = _mm256_add_ps(old_s1, _mm256_add_ps(d1, d1));
        let n2 = _mm256_add_ps(old_s2, _mm256_add_ps(d2, d2));
        let th = _mm256_fnmadd_ps(k, v1, x);
        let high = _mm256_sub_ps(th, v2);
        let mask = _mm256_castsi256_ps(_mm256_loadu_si256(
            block.high_pass_mask.as_ptr().cast::<__m256i>(),
        ));
        let output = _mm256_blendv_ps(v2, high, mask);
        _mm256_storeu_ps(block.s1.as_mut_ptr(), n1);
        _mm256_storeu_ps(block.s2.as_mut_ptr(), n2);
        _mm256_storeu_ps(block.samples.as_mut_ptr(), output);
    }
}

#[inline(never)]
pub(super) fn process_tpt_x86_avx2_fma(block: TptKernelBlock<'_>) {
    // SAFETY: this safe shim is retained only in a token created after AVX2+FMA detection.
    unsafe { process_tpt_x86_avx2_fma_inner(block) }
}

#[inline(never)]
#[target_feature(enable = "avx2")]
unsafe fn process_biquad_x86_avx2_inner(block: BiquadKernelBlock<'_>) {
    // SAFETY: the prepared token proves AVX2 and validation proves every slice has eight lanes.
    unsafe {
        let x = _mm256_loadu_ps(block.samples.as_ptr());
        let b0 = _mm256_loadu_ps(block.b0.as_ptr());
        let b1 = _mm256_loadu_ps(block.b1.as_ptr());
        let b2 = _mm256_loadu_ps(block.b2.as_ptr());
        let a1 = _mm256_loadu_ps(block.a1.as_ptr());
        let a2 = _mm256_loadu_ps(block.a2.as_ptr());
        let old_x1 = _mm256_loadu_ps(block.x1.as_ptr());
        let old_x2 = _mm256_loadu_ps(block.x2.as_ptr());
        let old_y1 = _mm256_loadu_ps(block.y1.as_ptr());
        let old_y2 = _mm256_loadu_ps(block.y2.as_ptr());
        let p0 = _mm256_mul_ps(b0, x);
        let p1 = _mm256_mul_ps(b1, old_x1);
        let s0 = _mm256_add_ps(p0, p1);
        let p2 = _mm256_mul_ps(b2, old_x2);
        let s1 = _mm256_add_ps(s0, p2);
        let p3 = _mm256_mul_ps(a1, old_y1);
        let s2 = _mm256_sub_ps(s1, p3);
        let p4 = _mm256_mul_ps(a2, old_y2);
        let y = _mm256_sub_ps(s2, p4);
        let mask = _mm256_castsi256_ps(_mm256_loadu_si256(
            block.identity_mask.as_ptr().cast::<__m256i>(),
        ));
        let identity_x2 = old_x1;
        let identity_x1 = x;
        let identity_y2 = old_x1;
        let identity_y1 = x;
        let new_x2 = _mm256_blendv_ps(old_x1, identity_x2, mask);
        let new_x1 = _mm256_blendv_ps(x, identity_x1, mask);
        let new_y2 = _mm256_blendv_ps(old_y1, identity_y2, mask);
        let new_y1 = _mm256_blendv_ps(y, identity_y1, mask);
        let output = _mm256_blendv_ps(y, x, mask);
        _mm256_storeu_ps(block.x1.as_mut_ptr(), new_x1);
        _mm256_storeu_ps(block.x2.as_mut_ptr(), new_x2);
        _mm256_storeu_ps(block.y1.as_mut_ptr(), new_y1);
        _mm256_storeu_ps(block.y2.as_mut_ptr(), new_y2);
        _mm256_storeu_ps(block.samples.as_mut_ptr(), output);
    }
}

#[inline(never)]
pub(super) fn process_biquad_x86_avx2(block: BiquadKernelBlock<'_>) {
    // SAFETY: the token was made only after AVX2 feature detection.
    unsafe { process_biquad_x86_avx2_inner(block) }
}

#[inline(never)]
#[target_feature(enable = "avx2,fma")]
unsafe fn process_biquad_x86_avx2_fma_inner(block: BiquadKernelBlock<'_>) {
    // SAFETY: the prepared token proves AVX2+FMA and all slices contain exactly eight lanes.
    unsafe {
        let x = _mm256_loadu_ps(block.samples.as_ptr());
        let b0 = _mm256_loadu_ps(block.b0.as_ptr());
        let b1 = _mm256_loadu_ps(block.b1.as_ptr());
        let b2 = _mm256_loadu_ps(block.b2.as_ptr());
        let a1 = _mm256_loadu_ps(block.a1.as_ptr());
        let a2 = _mm256_loadu_ps(block.a2.as_ptr());
        let old_x1 = _mm256_loadu_ps(block.x1.as_ptr());
        let old_x2 = _mm256_loadu_ps(block.x2.as_ptr());
        let old_y1 = _mm256_loadu_ps(block.y1.as_ptr());
        let old_y2 = _mm256_loadu_ps(block.y2.as_ptr());
        let p0 = _mm256_mul_ps(b0, x);
        let s0 = _mm256_fmadd_ps(b1, old_x1, p0);
        let s1 = _mm256_fmadd_ps(b2, old_x2, s0);
        let s2 = _mm256_fnmadd_ps(a1, old_y1, s1);
        let y = _mm256_fnmadd_ps(a2, old_y2, s2);
        let mask = _mm256_castsi256_ps(_mm256_loadu_si256(
            block.identity_mask.as_ptr().cast::<__m256i>(),
        ));
        let new_y2 = _mm256_blendv_ps(old_y1, old_x1, mask);
        let new_y1 = _mm256_blendv_ps(y, x, mask);
        let output = _mm256_blendv_ps(y, x, mask);
        _mm256_storeu_ps(block.x1.as_mut_ptr(), x);
        _mm256_storeu_ps(block.x2.as_mut_ptr(), old_x1);
        _mm256_storeu_ps(block.y1.as_mut_ptr(), new_y1);
        _mm256_storeu_ps(block.y2.as_mut_ptr(), new_y2);
        _mm256_storeu_ps(block.samples.as_mut_ptr(), output);
    }
}

#[inline(never)]
pub(super) fn process_biquad_x86_avx2_fma(block: BiquadKernelBlock<'_>) {
    // SAFETY: the token was made only after AVX2+FMA feature detection.
    unsafe { process_biquad_x86_avx2_fma_inner(block) }
}
