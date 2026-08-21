//! Portable scalar reference for the architecture-owned TPT operation graph.

use super::{BiquadKernelBlock, TptKernelBlock};

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

/// Frozen non-FMA direct-form-I operation graph for one scalar lane.
#[inline(never)]
pub(super) fn process_biquad_scalar(block: BiquadKernelBlock<'_>) {
    let x = block.samples[0];
    let old_x1 = block.x1[0];
    let old_x2 = block.x2[0];
    let old_y1 = block.y1[0];
    let old_y2 = block.y2[0];
    let p0 = block.b0[0] * x;
    let p1 = block.b1[0] * old_x1;
    let s0 = p0 + p1;
    let p2 = block.b2[0] * old_x2;
    let s1 = s0 + p2;
    let p3 = block.a1[0] * old_y1;
    let s2 = s1 - p3;
    let p4 = block.a2[0] * old_y2;
    let y = s2 - p4;
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
