//! Portable scalar reference for the architecture-owned TPT operation graph.

use super::TptKernelBlock;

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
