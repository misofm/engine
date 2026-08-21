//! Portable scalar reference for the architecture-owned TPT operation graph.

use super::{
    CompressorGainMixKernelBlock, DeltaKernelBlock, GateGainKernelBlock, SOFT_CLIP_HISTORY_WORDS,
    SOFT_CLIP_NONZERO_TAPS, SoftClipKernelBlock, TptKernelBlock, check_soft_clip_lanes,
};

/// Frozen scalar fixed-2x soft-clip high-rate phase.
#[inline(never)]
pub(super) fn process_soft_clip_scalar(block: SoftClipKernelBlock<'_>) -> u32 {
    let mut failed_lanes = 0;
    check_soft_clip_lanes(block.samples, &mut failed_lanes);
    let cursor = block.cursors[0] as usize;
    block.interpolation_history[cursor] = block.samples[0];
    let interpolated = soft_clip_convolve_scalar(
        block.interpolation_history,
        block.coefficients,
        cursor,
        &mut failed_lanes,
    );
    let shaped = soft_clip_cubic_scalar(interpolated, &mut failed_lanes);
    block.decimation_history[cursor] = shaped;
    block.samples[0] = soft_clip_convolve_scalar(
        block.decimation_history,
        block.coefficients,
        cursor,
        &mut failed_lanes,
    );
    if failed_lanes == 0 {
        block.cursors[0] = ((cursor + 1) % SOFT_CLIP_HISTORY_WORDS) as u32;
    }
    failed_lanes
}

#[allow(clippy::assign_op_pattern)]
fn soft_clip_convolve_scalar(
    history: &[f32],
    coefficients: &[f32],
    cursor: usize,
    failed_lanes: &mut u32,
) -> f32 {
    let mut accumulator = 0.0_f32;
    for tap in SOFT_CLIP_NONZERO_TAPS {
        let index = (cursor + SOFT_CLIP_HISTORY_WORDS - tap) % SOFT_CLIP_HISTORY_WORDS;
        let mut product = [coefficients[tap] * history[index]];
        check_soft_clip_lanes(&mut product, failed_lanes);
        let product = product[0];
        accumulator = accumulator + product;
        let mut sum = [accumulator];
        check_soft_clip_lanes(&mut sum, failed_lanes);
        accumulator = sum[0];
    }
    accumulator
}

fn soft_clip_cubic_scalar(value: f32, failed_lanes: &mut u32) -> f32 {
    if value <= -1.0 {
        -2.0_f32 / 3.0_f32
    } else if value >= 1.0 {
        2.0_f32 / 3.0_f32
    } else {
        let mut p0 = [value * value];
        check_soft_clip_lanes(&mut p0, failed_lanes);
        let mut p1 = [p0[0] * value];
        check_soft_clip_lanes(&mut p1, failed_lanes);
        let mut p2 = [p1[0] / 3.0_f32];
        check_soft_clip_lanes(&mut p2, failed_lanes);
        let mut output = [value - p2[0]];
        check_soft_clip_lanes(&mut output, failed_lanes);
        output[0]
    }
}

/// Frozen scalar gate gain-selection graph: one multiply plus exact dry identity selection.
#[inline(never)]
pub(super) fn process_gate_gain_scalar(block: GateGainKernelBlock<'_>) {
    let sample = block.samples[0];
    let p0 = sample * block.gains[0];
    block.samples[0] = if block.identity_mask[0] == u32::MAX {
        sample
    } else {
        p0
    };
}

/// Frozen noncontracting compressor dry/gain/mix graph for one scalar lane.
#[inline(never)]
pub(super) fn process_compressor_gain_mix_scalar(block: CompressorGainMixKernelBlock<'_>) {
    let dry = block.samples[0];
    let p0 = dry * block.gains[0];
    let p1 = p0 - dry;
    let p2 = block.mixes[0] * p1;
    let p3 = dry + p2;
    block.samples[0] = if block.dry_mask[0] == u32::MAX {
        dry
    } else if block.wet_mask[0] == u32::MAX {
        p0
    } else {
        p3
    };
}

#[inline(never)]
pub(super) fn process_tpt_scalar(block: TptKernelBlock<'_>) {
    let x = block.samples[0];
    let old_s1 = block.s1[0];
    let old_s2 = block.s2[0];
    let v3 = x - old_s2;
    let p1 = block.a2[0] * v3;
    let p2 = block.c1[0] * old_s1;
    let d1 = p1 - p2;
    let v1 = old_s1 + d1;
    let p3 = block.a2[0] * old_s1;
    let p4 = block.a3[0] * v3;
    let d2 = p3 + p4;
    let v2 = old_s2 + d2;
    let n1 = old_s1 + (d1 + d1);
    let n2 = old_s2 + (d2 + d2);
    let low = v2;
    let high = (x - block.k[0] * v1) - v2;
    block.s1[0] = n1;
    block.s2[0] = n2;
    block.samples[0] = if block.high_pass_mask[0] == u32::MAX {
        high
    } else {
        low
    };
}

/// Frozen noncontracting endpoint-conditioned delta graph for one scalar lane.
#[inline(never)]
pub(super) fn process_delta_scalar(block: DeltaKernelBlock<'_>) {
    let x = block.samples[0];
    let old_x1 = block.x1[0];
    let old_x2 = block.x2[0];
    let old_y1 = block.y1[0];
    let old_y2 = block.y2[0];
    let t0 = block.a[0] * x;
    let dx = old_x1 - t0;
    let t1 = block.a[0] * old_x1;
    let t2 = old_x2 - t1;
    let t3 = block.a[0] * dx;
    let ddx = t2 - t3;
    let p0 = block.n0[0] * x;
    let p1 = block.n1[0] * dx;
    let s0 = p0 + p1;
    let p2 = block.n2[0] * ddx;
    let num = s0 + p2;
    let q0 = block.a[0] * block.d1[0];
    let scale = (block.d0[0] - q0) + block.d2[0];
    let q1 = block.a[0] * block.d2[0];
    let q2 = (block.d1[0] - q1) - q1;
    let h0 = q2 * old_y1;
    let h1 = block.d2[0] * old_y2;
    let history = h0 + h1;
    let y = (num - history) / scale;
    if block.identity_mask[0] == u32::MAX {
        block.x2[0] = old_x1;
        block.x1[0] = x;
        block.y2[0] = old_x1;
        block.y1[0] = x;
        block.samples[0] = x;
    } else {
        block.x2[0] = old_x1;
        block.x1[0] = x;
        block.y2[0] = old_y1;
        block.y1[0] = y;
        block.samples[0] = y;
    }
}
