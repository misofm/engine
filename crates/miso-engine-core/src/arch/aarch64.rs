//! AArch64 four-lane NEON TPT kernel.

use core::arch::aarch64::*;

use super::{
    CompressorGainMixKernelBlock, DeltaKernelBlock, GateGainKernelBlock, SOFT_CLIP_HISTORY_WORDS,
    SOFT_CLIP_NONZERO_TAPS, SoftClipKernelBlock, TptKernelBlock,
};

#[inline(never)]
#[target_feature(enable = "neon")]
unsafe fn process_soft_clip_aarch64_neon_inner(block: SoftClipKernelBlock<'_>) {
    // SAFETY: the token validates four-lane slices and AArch64 NEON is a target property.
    unsafe {
        const LANES: usize = 4;
        for lane in 0..LANES {
            let cursor = block.cursors[lane] as usize;
            block.interpolation_history[cursor * LANES + lane] = block.samples[lane];
        }
        let interpolated = soft_clip_convolve_neon(
            block.interpolation_history,
            block.coefficients,
            block.cursors,
        );
        let shaped = soft_clip_cubic_neon(interpolated);
        for (lane, shaped_value) in shaped.into_iter().enumerate() {
            let cursor = block.cursors[lane] as usize;
            block.decimation_history[cursor * LANES + lane] = shaped_value;
        }
        let output =
            soft_clip_convolve_neon(block.decimation_history, block.coefficients, block.cursors);
        block.samples.copy_from_slice(&output);
        for cursor in block.cursors {
            *cursor = ((*cursor as usize + 1) % SOFT_CLIP_HISTORY_WORDS) as u32;
        }
    }
}

#[inline(never)]
pub(super) fn process_soft_clip_aarch64_neon(block: SoftClipKernelBlock<'_>) {
    // SAFETY: AArch64 NEON is available whenever this prepared entry point is selected.
    unsafe { process_soft_clip_aarch64_neon_inner(block) }
}

#[target_feature(enable = "neon")]
unsafe fn soft_clip_convolve_neon(
    history: &[f32],
    coefficients: &[f32],
    cursors: &[u32],
) -> [f32; 4] {
    // SAFETY: caller validated exact four-lane/sample-major storage before entering this helper.
    unsafe {
        const LANES: usize = 4;
        let mut accumulator = vdupq_n_f32(0.0);
        for tap in SOFT_CLIP_NONZERO_TAPS {
            let mut gathered = [0.0_f32; LANES];
            for lane in 0..LANES {
                let cursor = cursors[lane] as usize;
                let index = (cursor + SOFT_CLIP_HISTORY_WORDS - tap) % SOFT_CLIP_HISTORY_WORDS;
                gathered[lane] = history[index * LANES + lane];
            }
            let product = vmulq_f32(vdupq_n_f32(coefficients[tap]), vld1q_f32(gathered.as_ptr()));
            accumulator = vaddq_f32(accumulator, product);
        }
        let mut output = [0.0_f32; LANES];
        vst1q_f32(output.as_mut_ptr(), accumulator);
        output
    }
}

#[target_feature(enable = "neon")]
unsafe fn soft_clip_cubic_neon(input: [f32; 4]) -> [f32; 4] {
    // SAFETY: caller enters only through the target-feature-gated four-lane phase function.
    unsafe {
        let value = vld1q_f32(input.as_ptr());
        let negative_mask = vcleq_f32(value, vdupq_n_f32(-1.0));
        let positive_mask = vcgeq_f32(value, vdupq_n_f32(1.0));
        let saturated = vorrq_u32(negative_mask, positive_mask);
        let interior_mask = vmvnq_u32(saturated);
        let interior = vbslq_f32(interior_mask, value, vdupq_n_f32(0.0));
        let p0 = vmulq_f32(interior, interior);
        let p1 = vmulq_f32(p0, interior);
        let p2 = vdivq_f32(p1, vdupq_n_f32(3.0));
        let polynomial = vsubq_f32(interior, p2);
        let negative = vbslq_f32(negative_mask, vdupq_n_f32(-2.0 / 3.0), polynomial);
        let output = vbslq_f32(positive_mask, vdupq_n_f32(2.0 / 3.0), negative);
        let mut values = [0.0_f32; 4];
        vst1q_f32(values.as_mut_ptr(), output);
        values
    }
}

#[inline(never)]
#[target_feature(enable = "neon")]
unsafe fn process_gate_gain_aarch64_neon_inner(block: GateGainKernelBlock<'_>) {
    // SAFETY: the prepared token proves AArch64 NEON and validated slices contain four lanes.
    unsafe {
        let sample = vld1q_f32(block.samples.as_ptr());
        let gain = vld1q_f32(block.gains.as_ptr());
        let p0 = vmulq_f32(sample, gain);
        let identity_mask = vld1q_u32(block.identity_mask.as_ptr());
        let output = vbslq_f32(identity_mask, sample, p0);
        vst1q_f32(block.samples.as_mut_ptr(), output);
    }
}

#[inline(never)]
pub(super) fn process_gate_gain_aarch64_neon(block: GateGainKernelBlock<'_>) {
    // SAFETY: AArch64 NEON was selected at preparation, before any render callback.
    unsafe { process_gate_gain_aarch64_neon_inner(block) }
}

#[inline(never)]
#[target_feature(enable = "neon")]
unsafe fn process_compressor_gain_mix_aarch64_neon_inner(block: CompressorGainMixKernelBlock<'_>) {
    // SAFETY: the prepared token proves AArch64 NEON and all slices have the required four lanes.
    unsafe {
        let dry = vld1q_f32(block.samples.as_ptr());
        let gain = vld1q_f32(block.gains.as_ptr());
        let mix = vld1q_f32(block.mixes.as_ptr());
        let p0 = vmulq_f32(dry, gain);
        let p1 = vsubq_f32(p0, dry);
        let p2 = vmulq_f32(mix, p1);
        let p3 = vaddq_f32(dry, p2);
        let dry_mask = vld1q_u32(block.dry_mask.as_ptr());
        let wet_mask = vld1q_u32(block.wet_mask.as_ptr());
        let mixed_or_wet = vbslq_f32(wet_mask, p0, p3);
        let output = vbslq_f32(dry_mask, dry, mixed_or_wet);
        vst1q_f32(block.samples.as_mut_ptr(), output);
    }
}

#[inline(never)]
pub(super) fn process_compressor_gain_mix_aarch64_neon(block: CompressorGainMixKernelBlock<'_>) {
    // SAFETY: the prepared token admits this shim only on the mandatory AArch64 NEON facility.
    unsafe { process_compressor_gain_mix_aarch64_neon_inner(block) }
}

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
unsafe fn process_delta_aarch64_neon_inner(block: DeltaKernelBlock<'_>) {
    // SAFETY: AArch64 NEON is prepared before render and validation proves four-lane slices.
    unsafe {
        let x = vld1q_f32(block.samples.as_ptr());
        let a = vld1q_f32(block.a.as_ptr());
        let n0 = vld1q_f32(block.n0.as_ptr());
        let d0 = vld1q_f32(block.d0.as_ptr());
        let n1 = vld1q_f32(block.n1.as_ptr());
        let d1 = vld1q_f32(block.d1.as_ptr());
        let n2 = vld1q_f32(block.n2.as_ptr());
        let d2 = vld1q_f32(block.d2.as_ptr());
        let old_x1 = vld1q_f32(block.x1.as_ptr());
        let old_x2 = vld1q_f32(block.x2.as_ptr());
        let old_y1 = vld1q_f32(block.y1.as_ptr());
        let old_y2 = vld1q_f32(block.y2.as_ptr());
        let t0 = vmulq_f32(a, x);
        let dx = vsubq_f32(old_x1, t0);
        let t1 = vmulq_f32(a, old_x1);
        let t2 = vsubq_f32(old_x2, t1);
        let t3 = vmulq_f32(a, dx);
        let ddx = vsubq_f32(t2, t3);
        let p0 = vmulq_f32(n0, x);
        let p1 = vmulq_f32(n1, dx);
        let s0 = vaddq_f32(p0, p1);
        let p2 = vmulq_f32(n2, ddx);
        let num = vaddq_f32(s0, p2);
        let q0 = vmulq_f32(a, d1);
        let scale = vaddq_f32(vsubq_f32(d0, q0), d2);
        let q1 = vmulq_f32(a, d2);
        let q2 = vsubq_f32(vsubq_f32(d1, q1), q1);
        let h0 = vmulq_f32(q2, old_y1);
        let h1 = vmulq_f32(d2, old_y2);
        let history = vaddq_f32(h0, h1);
        let y = vdivq_f32(vsubq_f32(num, history), scale);
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
pub(super) fn process_delta_aarch64_neon(block: DeltaKernelBlock<'_>) {
    // SAFETY: the prepared token proves the AArch64 NEON target capability.
    unsafe { process_delta_aarch64_neon_inner(block) }
}
