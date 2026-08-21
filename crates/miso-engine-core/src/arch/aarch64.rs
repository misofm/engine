//! AArch64 four-lane NEON TPT kernel.

use core::arch::aarch64::*;

use super::TptKernelBlock;

#[inline(never)]
#[target_feature(enable = "neon")]
unsafe fn process_tpt_aarch64_neon_inner(block: TptKernelBlock<'_>) {
    // SAFETY: AArch64 admits this token only for its mandatory NEON facility; validated slices
    // contain four lanes and every unaligned load/store remains within those slices.
    unsafe {
        let x = vld1q_f32(block.samples.as_ptr());
        let c1 = vld1q_f32(block.c1.as_ptr());
        let a2 = vld1q_f32(block.a2.as_ptr());
        let a3 = vld1q_f32(block.a3.as_ptr());
        let k = vld1q_f32(block.k.as_ptr());
        let old_s1 = vld1q_f32(block.s1.as_ptr());
        let old_s2 = vld1q_f32(block.s2.as_ptr());
        let v3 = vsubq_f32(x, old_s2);
        let p1 = vmulq_f32(a2, v3);
        let p2 = vmulq_f32(c1, old_s1);
        let d1 = vsubq_f32(p1, p2);
        let v1 = vaddq_f32(old_s1, d1);
        let p3 = vmulq_f32(a2, old_s1);
        let p4 = vmulq_f32(a3, v3);
        let d2 = vaddq_f32(p3, p4);
        let v2 = vaddq_f32(old_s2, d2);
        let n1 = vaddq_f32(old_s1, vaddq_f32(d1, d1));
        let n2 = vaddq_f32(old_s2, vaddq_f32(d2, d2));
        let th = vsubq_f32(x, vmulq_f32(k, v1));
        let high = vsubq_f32(th, v2);
        let mask = vld1q_u32(block.high_pass_mask.as_ptr());
        let output = vbslq_f32(mask, high, v2);
        vst1q_f32(block.s1.as_mut_ptr(), n1);
        vst1q_f32(block.s2.as_mut_ptr(), n2);
        vst1q_f32(block.samples.as_mut_ptr(), output);
    }
}

#[inline(never)]
pub(super) fn process_tpt_aarch64_neon(block: TptKernelBlock<'_>) {
    // SAFETY: AArch64 NEON is a target property proved when this safe token is constructed.
    unsafe { process_tpt_aarch64_neon_inner(block) }
}
