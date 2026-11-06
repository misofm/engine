//! Issue #2243: the on-demand soak's mutation, compiled only with the `soak-probe` feature, which
//! no shipped build enables.

/// Allocate 4 KiB inside the caller's render-locked window and never free it: one counted render
/// allocation and 4 KiB of memory growth per rendered block.
pub(crate) fn leak_one_block() {
    std::mem::forget(std::hint::black_box(vec![0_u8; 4096]));
}
