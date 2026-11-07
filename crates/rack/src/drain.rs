//! A [`BankChain`]'s step 0 and the proof it leaves (tracker #2258 GR-5, issue #2326).
//!
//! A live stage's process body reads the spans, targets and verdicts its drain wrote earlier in
//! the same block, so no stage body may run in a block whose drains did not. [`Drained`]'s field
//! is private to this module and [`BankChain::drain_slots`] is the only code here that builds one,
//! so the compiler makes the drain loop its only constructor. The two bodies that dispatch a
//! block's stage work, `render_slots` and `skip_block`, take it by value: each runs only after the
//! drains of its own block, and a block runs at most one of them.

use engine::realtime::{LiveSnapshot, RenderError};

use crate::BankChain;

/// Proof that one block's drains ran: every active slot's [`BankStage::begin_block`], before
/// anything else of the block.
///
/// Zero-sized, so it costs nothing at run time. A stage driven directly, outside a chain, carries
/// the same obligation by its contract ([`BankStage::begin_block`]); the trait's signatures do
/// not carry this proof.
///
/// [`BankStage::begin_block`]: crate::BankStage::begin_block
#[must_use = "a drained block is rendered or skipped"]
pub(crate) struct Drained(());

impl BankChain {
    // REALTIME_POLICY_BEGIN
    /// Step 0 of the block: every slot that has an active lane reads its live cells and drains its
    /// observation queues ([`BankStage::begin_block`]), before anything else of the block. See
    /// `BankStage::begin_block` for why this cannot be folded into `process`. Issue #2135: the
    /// settle edges are gathered here, one load per drained slot.
    ///
    /// The returned [`Drained`] is the only way to the block's stage bodies. A drain that fails
    /// returns before the proof exists, so no stage body runs in that block.
    ///
    /// [`BankStage::begin_block`]: crate::BankStage::begin_block
    pub(crate) fn drain_slots(
        &mut self,
        first_sample: u64,
        snapshot: LiveSnapshot,
    ) -> Result<Drained, RenderError> {
        let mut settle_edges = 0_u32;
        for slot in &mut self.slots {
            // The same identity-slot guard `process` has always taken. A slot that is an identity
            // on every lane renders nothing, and draining its queues would consume records for a
            // block it does not run. A slot with a live lane is active: the guard skips a stage
            // that has no live lane to drain, never a drain (#1504 D3).
            if slot.has_active_lanes() {
                slot.stage.begin_block(first_sample, snapshot)?;
                settle_edges |= slot.stage.settle_edges();
            }
        }
        self.settle_edges = settle_edges;
        Ok(Drained(()))
    }
    // REALTIME_POLICY_END
}
