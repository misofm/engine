//! x86 AVX2 and separately gated AVX2+FMA TPT kernels.

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use super::{
    CompressorGainMixKernelBlock, DeltaKernelBlock, GateGainKernelBlock, SOFT_CLIP_HISTORY_WORDS,
    SOFT_CLIP_NONZERO_TAPS, SoftClipKernelBlock, TptKernelBlock,
};

#[inline(never)]
#[target_feature(enable = "avx2")]
unsafe fn process_soft_clip_x86_avx2_inner(block: SoftClipKernelBlock<'_>) {
    // SAFETY: the token proves AVX2 and validates all eight-lane/sample-major slices.
    unsafe {
        const LANES: usize = 8;
        for lane in 0..LANES {
            let cursor = block.cursors[lane] as usize;
            block.interpolation_history[cursor * LANES + lane] = block.samples[lane];
        }
        let interpolated = soft_clip_convolve_x86(
            block.interpolation_history,
            block.coefficients,
            block.cursors,
        );
        let shaped = soft_clip_cubic_x86(interpolated);
        for (lane, shaped_value) in shaped.into_iter().enumerate() {
            let cursor = block.cursors[lane] as usize;
            block.decimation_history[cursor * LANES + lane] = shaped_value;
        }
        let output =
            soft_clip_convolve_x86(block.decimation_history, block.coefficients, block.cursors);
        block.samples.copy_from_slice(&output);
        for cursor in block.cursors {
            *cursor = ((*cursor as usize + 1) % SOFT_CLIP_HISTORY_WORDS) as u32;
        }
    }
}

#[inline(never)]
pub(super) fn process_soft_clip_x86_avx2(block: SoftClipKernelBlock<'_>) {
    // SAFETY: the prepared token retains this shim only after AVX2 detection.
    unsafe { process_soft_clip_x86_avx2_inner(block) }
}

#[inline(never)]
pub(super) fn process_soft_clip_x86_avx2_fma(block: SoftClipKernelBlock<'_>) {
    // The FMA selection deliberately aliases the frozen noncontracting AVX2 graph.
    process_soft_clip_x86_avx2(block);
}

#[target_feature(enable = "avx2")]
unsafe fn soft_clip_convolve_x86(
    history: &[f32],
    coefficients: &[f32],
    cursors: &[u32],
) -> [f32; 8] {
    // SAFETY: callers are the AVX2 phase implementation, where the token has validated all
    // lengths and each local gather has exactly eight f32 words.
    unsafe {
        const LANES: usize = 8;
        let mut accumulator = _mm256_setzero_ps();
        for tap in SOFT_CLIP_NONZERO_TAPS {
            let mut gathered = [0.0_f32; LANES];
            for lane in 0..LANES {
                let cursor = cursors[lane] as usize;
                let index = (cursor + SOFT_CLIP_HISTORY_WORDS - tap) % SOFT_CLIP_HISTORY_WORDS;
                gathered[lane] = history[index * LANES + lane];
            }
            let coefficient = _mm256_set1_ps(coefficients[tap]);
            let product = _mm256_mul_ps(coefficient, _mm256_loadu_ps(gathered.as_ptr()));
            accumulator = _mm256_add_ps(accumulator, product);
        }
        let mut output = [0.0_f32; LANES];
        _mm256_storeu_ps(output.as_mut_ptr(), accumulator);
        output
    }
}

#[target_feature(enable = "avx2")]
unsafe fn soft_clip_cubic_x86(input: [f32; 8]) -> [f32; 8] {
    // SAFETY: this helper runs only under the validated AVX2 phase token and accesses local
    // eight-lane arrays. It emits separate multiply, divide, and subtraction instructions.
    unsafe {
        let value = _mm256_loadu_ps(input.as_ptr());
        let negative_mask = _mm256_cmp_ps(value, _mm256_set1_ps(-1.0), _CMP_LE_OQ);
        let positive_mask = _mm256_cmp_ps(value, _mm256_set1_ps(1.0), _CMP_GE_OQ);
        let saturated = _mm256_or_ps(negative_mask, positive_mask);
        let all = _mm256_castsi256_ps(_mm256_set1_epi32(-1));
        let interior_mask = _mm256_andnot_ps(saturated, all);
        let zero = _mm256_setzero_ps();
        let interior = _mm256_blendv_ps(zero, value, interior_mask);
        let p0 = _mm256_mul_ps(interior, interior);
        let p1 = _mm256_mul_ps(p0, interior);
        let p2 = _mm256_div_ps(p1, _mm256_set1_ps(3.0));
        let polynomial = _mm256_sub_ps(interior, p2);
        let negative = _mm256_blendv_ps(polynomial, _mm256_set1_ps(-2.0 / 3.0), negative_mask);
        let output = _mm256_blendv_ps(negative, _mm256_set1_ps(2.0 / 3.0), positive_mask);
        let mut values = [0.0_f32; 8];
        _mm256_storeu_ps(values.as_mut_ptr(), output);
        values
    }
}

#[inline(never)]
#[target_feature(enable = "avx2")]
unsafe fn process_gate_gain_x86_avx2_inner(block: GateGainKernelBlock<'_>) {
    // SAFETY: the prepared token proves AVX2 and exact eight-lane slices for all unaligned I/O.
    unsafe {
        let sample = _mm256_loadu_ps(block.samples.as_ptr());
        let gain = _mm256_loadu_ps(block.gains.as_ptr());
        let p0 = _mm256_mul_ps(sample, gain);
        let identity_mask = _mm256_castsi256_ps(_mm256_loadu_si256(
            block.identity_mask.as_ptr().cast::<__m256i>(),
        ));
        let output = _mm256_blendv_ps(p0, sample, identity_mask);
        _mm256_storeu_ps(block.samples.as_mut_ptr(), output);
    }
}

#[inline(never)]
pub(super) fn process_gate_gain_x86_avx2(block: GateGainKernelBlock<'_>) {
    // SAFETY: the token retains this shim only after AVX2 detection.
    unsafe { process_gate_gain_x86_avx2_inner(block) }
}

#[inline(never)]
pub(super) fn process_gate_gain_x86_avx2_fma(block: GateGainKernelBlock<'_>) {
    // Frozen V1 explicitly aliases the base AVX2 graph; no FMA intrinsic or contraction occurs.
    process_gate_gain_x86_avx2(block);
}

#[inline(never)]
#[target_feature(enable = "avx2")]
unsafe fn process_compressor_gain_mix_x86_avx2_inner(block: CompressorGainMixKernelBlock<'_>) {
    // SAFETY: the prepared token proves AVX2 and exact eight-lane slices for each unaligned
    // vector access. The graph deliberately contains only separate mul/sub/add operations.
    unsafe {
        let dry = _mm256_loadu_ps(block.samples.as_ptr());
        let gain = _mm256_loadu_ps(block.gains.as_ptr());
        let mix = _mm256_loadu_ps(block.mixes.as_ptr());
        let p0 = _mm256_mul_ps(dry, gain);
        let p1 = _mm256_sub_ps(p0, dry);
        let p2 = _mm256_mul_ps(mix, p1);
        let p3 = _mm256_add_ps(dry, p2);
        let dry_mask = _mm256_castsi256_ps(_mm256_loadu_si256(
            block.dry_mask.as_ptr().cast::<__m256i>(),
        ));
        let wet_mask = _mm256_castsi256_ps(_mm256_loadu_si256(
            block.wet_mask.as_ptr().cast::<__m256i>(),
        ));
        let mixed_or_wet = _mm256_blendv_ps(p3, p0, wet_mask);
        let output = _mm256_blendv_ps(mixed_or_wet, dry, dry_mask);
        _mm256_storeu_ps(block.samples.as_mut_ptr(), output);
    }
}

#[inline(never)]
pub(super) fn process_compressor_gain_mix_x86_avx2(block: CompressorGainMixKernelBlock<'_>) {
    // SAFETY: the prepared token invokes this shim only after AVX2 detection.
    unsafe { process_compressor_gain_mix_x86_avx2_inner(block) }
}

#[inline(never)]
pub(super) fn process_compressor_gain_mix_x86_avx2_fma(block: CompressorGainMixKernelBlock<'_>) {
    // V1 explicitly aliases the base AVX2 graph: no FMA intrinsic or contraction is permitted.
    process_compressor_gain_mix_x86_avx2(block);
}

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
unsafe fn process_delta_x86_avx2_inner(block: DeltaKernelBlock<'_>) {
    // SAFETY: the prepared token proves AVX2 and validation proves every slice has eight lanes.
    unsafe {
        let x = _mm256_loadu_ps(block.samples.as_ptr());
        let a = _mm256_loadu_ps(block.a.as_ptr());
        let n0 = _mm256_loadu_ps(block.n0.as_ptr());
        let d0 = _mm256_loadu_ps(block.d0.as_ptr());
        let n1 = _mm256_loadu_ps(block.n1.as_ptr());
        let d1 = _mm256_loadu_ps(block.d1.as_ptr());
        let n2 = _mm256_loadu_ps(block.n2.as_ptr());
        let d2 = _mm256_loadu_ps(block.d2.as_ptr());
        let old_x1 = _mm256_loadu_ps(block.x1.as_ptr());
        let old_x2 = _mm256_loadu_ps(block.x2.as_ptr());
        let old_y1 = _mm256_loadu_ps(block.y1.as_ptr());
        let old_y2 = _mm256_loadu_ps(block.y2.as_ptr());
        let t0 = _mm256_mul_ps(a, x);
        let dx = _mm256_sub_ps(old_x1, t0);
        let t1 = _mm256_mul_ps(a, old_x1);
        let t2 = _mm256_sub_ps(old_x2, t1);
        let t3 = _mm256_mul_ps(a, dx);
        let ddx = _mm256_sub_ps(t2, t3);
        let p0 = _mm256_mul_ps(n0, x);
        let p1 = _mm256_mul_ps(n1, dx);
        let s0 = _mm256_add_ps(p0, p1);
        let p2 = _mm256_mul_ps(n2, ddx);
        let num = _mm256_add_ps(s0, p2);
        let q0 = _mm256_mul_ps(a, d1);
        let scale = _mm256_add_ps(_mm256_sub_ps(d0, q0), d2);
        let q1 = _mm256_mul_ps(a, d2);
        let q2 = _mm256_sub_ps(_mm256_sub_ps(d1, q1), q1);
        let h0 = _mm256_mul_ps(q2, old_y1);
        let h1 = _mm256_mul_ps(d2, old_y2);
        let history = _mm256_add_ps(h0, h1);
        let y = _mm256_div_ps(_mm256_sub_ps(num, history), scale);
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
pub(super) fn process_delta_x86_avx2(block: DeltaKernelBlock<'_>) {
    // SAFETY: the token was made only after AVX2 feature detection.
    unsafe { process_delta_x86_avx2_inner(block) }
}

#[inline(never)]
#[target_feature(enable = "avx2,fma")]
unsafe fn process_delta_x86_avx2_fma_inner(block: DeltaKernelBlock<'_>) {
    // SAFETY: the token proves AVX2+FMA and all slices contain exactly eight lanes. V1 permits
    // no contractions, so this is deliberately the same mul/add/sub/div graph as base AVX2.
    unsafe {
        let x = _mm256_loadu_ps(block.samples.as_ptr());
        let a = _mm256_loadu_ps(block.a.as_ptr());
        let n0 = _mm256_loadu_ps(block.n0.as_ptr());
        let d0 = _mm256_loadu_ps(block.d0.as_ptr());
        let n1 = _mm256_loadu_ps(block.n1.as_ptr());
        let d1 = _mm256_loadu_ps(block.d1.as_ptr());
        let n2 = _mm256_loadu_ps(block.n2.as_ptr());
        let d2 = _mm256_loadu_ps(block.d2.as_ptr());
        let old_x1 = _mm256_loadu_ps(block.x1.as_ptr());
        let old_x2 = _mm256_loadu_ps(block.x2.as_ptr());
        let old_y1 = _mm256_loadu_ps(block.y1.as_ptr());
        let old_y2 = _mm256_loadu_ps(block.y2.as_ptr());
        let t0 = _mm256_mul_ps(a, x);
        let dx = _mm256_sub_ps(old_x1, t0);
        let t1 = _mm256_mul_ps(a, old_x1);
        let t2 = _mm256_sub_ps(old_x2, t1);
        let t3 = _mm256_mul_ps(a, dx);
        let ddx = _mm256_sub_ps(t2, t3);
        let p0 = _mm256_mul_ps(n0, x);
        let p1 = _mm256_mul_ps(n1, dx);
        let s0 = _mm256_add_ps(p0, p1);
        let p2 = _mm256_mul_ps(n2, ddx);
        let num = _mm256_add_ps(s0, p2);
        let q0 = _mm256_mul_ps(a, d1);
        let scale = _mm256_add_ps(_mm256_sub_ps(d0, q0), d2);
        let q1 = _mm256_mul_ps(a, d2);
        let q2 = _mm256_sub_ps(_mm256_sub_ps(d1, q1), q1);
        let h0 = _mm256_mul_ps(q2, old_y1);
        let h1 = _mm256_mul_ps(d2, old_y2);
        let history = _mm256_add_ps(h0, h1);
        let y = _mm256_div_ps(_mm256_sub_ps(num, history), scale);
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
pub(super) fn process_delta_x86_avx2_fma(block: DeltaKernelBlock<'_>) {
    // SAFETY: the token was made only after AVX2+FMA feature detection.
    unsafe { process_delta_x86_avx2_fma_inner(block) }
}
