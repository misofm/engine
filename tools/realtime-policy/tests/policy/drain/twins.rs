//! Twins of the render-thread drains that #1443 marks (D9): each is the shape of a real function
//! in `crates/source`, `crates/host-core` or `crates/rack`, so a later change to the drain rule
//! that refuses it fails here and not only on the real tree.

use super::accepts;

/// `take_recycled_block`: one pop at the end of a method chain, in no loop.
#[test]
fn a_single_pop_in_a_method_chain_passes() {
    accepts(
        "twin-method-chain",
        "impl Producer {\n    fn take_recycled_block(&mut self) -> Result<Block, ChunkError> {\n        self.recycle_consumer\n            .try_pop()\n            .map_err(|QueueEmpty { .. }| ChunkError::Full {\n                full_count: self.data_producer.full_count(),\n            })\n    }\n}",
    );
}

/// The spectrum capture's drains: a named cap, a `break` on an empty queue, and a read that matches
/// the pop and returns after the loop.
#[test]
fn the_spectrum_drains_pass() {
    accepts(
        "twin-spectrum-drains",
        "impl Capture {\n    fn cancel(&mut self) {\n        let available = self.consumer.available_at_entry().min(RESULT_SLOTS);\n        for _ in 0..available {\n            if self.consumer.try_pop().is_err() {\n                break;\n            }\n        }\n        self.reset();\n    }\n\n    fn read_record(&mut self) -> Result<Record, ReadError> {\n        match self.state.load() {\n            COMPLETE => match self.consumer.try_pop() {\n                Ok(record) => Ok(record),\n                Err(_) => Err(ReadError::Pending),\n            },\n            INVALID => {\n                let _ = self.consumer.try_pop();\n                Err(ReadError::Invalid)\n            }\n            _ => Err(ReadError::Invalid),\n        }\n    }\n\n    fn read_continuous(&mut self) -> Result<Record, ReadError> {\n        let available = self.consumer.available_at_entry().min(RESULT_SLOTS);\n        for _ in 0..available {\n            match self.consumer.try_pop() {\n                Ok(record) if self.stale(&record) => {}\n                Ok(record) => {\n                    return Ok(record);\n                }\n                Err(_) => return Err(ReadError::Pending),\n            }\n        }\n        Err(ReadError::Warming)\n    }\n}",
    );
}

/// `LiveControlEffectBankStage::drain_observe_queues` (`crates/rack`, #1816): the walk over the
/// stage's queue collection that the block drain and a plan swap's carry share, with the carry's
/// lane-mask `continue` first in the body and the tap collection bound outside the loop.
#[test]
fn the_carry_observe_lane_walk_passes() {
    accepts(
        "twin-carry-observe-lanes",
        "impl Stage {\n    fn drain_observe_queues(&mut self, first_sample: u64, lanes: u8) {\n        let mut observations = self.observations.as_deref_mut();\n        let mut unbound = 0_u64;\n        for (index, queue) in self.observe_queues.iter_mut() {\n            if lanes & (1 << *index) == 0 {\n                continue;\n            }\n            let mut available = queue.available_at_entry();\n            while available != 0 {\n                available -= 1;\n                let Ok(record) = queue.try_pop() else {\n                    break;\n                };\n                let observation = observations\n                    .as_deref_mut()\n                    .and_then(|lanes| lanes.get_mut(*index))\n                    .and_then(Option::as_mut);\n                if !apply_observe(record, first_sample, observation) {\n                    unbound = unbound.saturating_add(1);\n                }\n            }\n        }\n        self.unbound = self.unbound.saturating_add(unbound);\n    }\n}",
    );
}
