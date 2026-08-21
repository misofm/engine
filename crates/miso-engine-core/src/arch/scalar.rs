//! Portable scalar reference for the architecture-owned TPT operation graph.

use super::{CompressorGainMixKernelBlock, DeltaKernelBlock, GateGainKernelBlock, TptKernelBlock};

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
