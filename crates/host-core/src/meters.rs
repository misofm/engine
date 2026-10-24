//! The one strip meter selection and its window (issue #1581 D1/D2, decision 17 C1/C13).
//!
//! Every preparation entry that attaches live controls meters the same strips the same way: one
//! [`MeterTap::PostMatrix`] [`MeterMetricSet::SAMPLE_PEAK`] meter per strip, in
//! [`SessionModel::strips`] order (the tracks, then the submixes), so a host reads strip `i`'s
//! left and right peak from meter `i`. A host does not choose its own list.

use core::num::NonZeroU32;

use builtins::{MeterMetricSet, MeterTap};
use session::SessionModel;

use crate::prepare::HostMeterRequest;

/// The strip meters every live-control preparation builds (issue #1581 D1): one per strip, in
/// [`SessionModel::strips`] order, each a post-matrix left and right sample peak.
#[must_use]
pub fn meter_selection(model: &SessionModel) -> Vec<HostMeterRequest> {
    model
        .strips()
        .map(|strip| HostMeterRequest {
            strip_id: strip.id.as_str().into(),
            tap: MeterTap::PostMatrix,
            metrics: MeterMetricSet::SAMPLE_PEAK,
        })
        .collect()
}

/// The meter window, in blocks, that a request's `meter_window_blocks == 0` stands for (issue
/// #1581 D2): the whole number of blocks nearest to 32 ms, and at least one.
///
/// `max(1, (rate_hz * 32 + 500 * quantum) / (1000 * quantum))`, in `u64`: 12 blocks at 48 kHz and
/// 128 frames (the browser's present default), 11 at 44.1 kHz, 24 at 96 kHz, 3 at 512 frames.
/// A `quantum_frames` of zero, which no prepared plan has, gives one block.
#[must_use]
pub fn default_meter_window_blocks(rate_hz: u32, quantum_frames: u32) -> NonZeroU32 {
    let quantum = u64::from(quantum_frames);
    let blocks = (u64::from(rate_hz) * 32 + 500 * quantum)
        .checked_div(1000 * quantum)
        .unwrap_or(1);
    u32::try_from(blocks)
        .ok()
        .and_then(NonZeroU32::new)
        .unwrap_or(NonZeroU32::MIN)
}

#[cfg(test)]
mod tests {
    use super::default_meter_window_blocks;

    /// Gate 5: the default window is the nearest whole block count to 32 ms.
    #[test]
    fn the_default_window_is_the_nearest_block_count_to_32_ms() {
        for (rate, quantum, blocks) in [
            (48_000, 128, 12),
            (44_100, 128, 11),
            (88_200, 128, 22),
            (96_000, 128, 24),
            (48_000, 512, 3),
            (48_000, 4096, 1),
        ] {
            assert_eq!(
                default_meter_window_blocks(rate, quantum).get(),
                blocks,
                "{rate} Hz at {quantum} frames"
            );
        }
    }
}
