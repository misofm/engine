//! x86 AVX2 and separately gated AVX2+FMA TPT kernels.

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use super::TptKernelBlock;

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
