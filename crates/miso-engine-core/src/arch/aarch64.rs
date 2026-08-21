//! AArch64 four-lane NEON TPT kernel.

use core::arch::aarch64::*;

use super::{BiquadKernelBlock, TptKernelBlock};

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

#[inline(never)]
#[target_feature(enable = "neon")]
unsafe fn process_biquad_aarch64_neon_inner(block: BiquadKernelBlock<'_>) {
    // SAFETY: AArch64 NEON is prepared before render and validation proves four-lane slices.
    unsafe {
        let x = vld1q_f32(block.samples.as_ptr());
        let b0 = vld1q_f32(block.b0.as_ptr());
        let b1 = vld1q_f32(block.b1.as_ptr());
        let b2 = vld1q_f32(block.b2.as_ptr());
        let a1 = vld1q_f32(block.a1.as_ptr());
        let a2 = vld1q_f32(block.a2.as_ptr());
        let old_x1 = vld1q_f32(block.x1.as_ptr());
        let old_x2 = vld1q_f32(block.x2.as_ptr());
        let old_y1 = vld1q_f32(block.y1.as_ptr());
        let old_y2 = vld1q_f32(block.y2.as_ptr());
        let p0 = vmulq_f32(b0, x);
        let p1 = vmulq_f32(b1, old_x1);
        let s0 = vaddq_f32(p0, p1);
        let p2 = vmulq_f32(b2, old_x2);
        let s1 = vaddq_f32(s0, p2);
        let p3 = vmulq_f32(a1, old_y1);
        let s2 = vsubq_f32(s1, p3);
        let p4 = vmulq_f32(a2, old_y2);
        let y = vsubq_f32(s2, p4);
        let mask = vld1q_u32(block.identity_mask.as_ptr());
        let new_y2 = vbslq_f32(mask, old_x1, old_y1);
        let new_y1 = vbslq_f32(mask, x, y);
        let output = vbslq_f32(mask, x, y);
        vst1q_f32(block.x1.as_mut_ptr(), x);
        vst1q_f32(block.x2.as_mut_ptr(), old_x1);
        vst1q_f32(block.y1.as_mut_ptr(), new_y1);
        vst1q_f32(block.y2.as_mut_ptr(), new_y2);
        vst1q_f32(block.samples.as_mut_ptr(), output);
    }
}

#[inline(never)]
pub(super) fn process_biquad_aarch64_neon(block: BiquadKernelBlock<'_>) {
    // SAFETY: the prepared token proves the AArch64 NEON target capability.
    unsafe { process_biquad_aarch64_neon_inner(block) }
}
