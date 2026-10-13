//! Twins of the render-thread drains that #1443 marks (D9): each is the shape of a real function
//! in `crates/source` or `crates/host-core`, so a later change to the drain rule that refuses it
//! fails here and not only on the real tree.

use super::accepts;

/// `take_recycled_block`: one pop at the end of a method chain, in no loop.
#[test]
fn a_single_pop_in_a_method_chain_passes() {
    accepts(
        "twin-method-chain",
        "impl Producer {\n    fn take_recycled_block(&mut self) -> Result<Block, ChunkError> {\n        self.recycle_consumer\n            .try_pop()\n            .map_err(|QueueEmpty { .. }| ChunkError::Full {\n                full_count: self.data_producer.full_count(),\n            })\n    }\n}",
    );
}

/// `acquire_current_block`: two counted phases with a re-observe between them.
#[test]
fn the_two_phase_acquire_passes() {
    accepts(
        "twin-two-phase-acquire",
        "impl Consumer {\n    fn acquire(&mut self, clock: Clock) {\n        if self.current.is_some() {\n            return;\n        }\n        let available = self\n            .data_consumer\n            .available_at_entry()\n            .min(self.transfer_block_count);\n        let mut popped = 0_usize;\n        let mut unobserved = None;\n        for _ in 0..available {\n            let Ok(block) = self.data_consumer.try_pop() else {\n                break;\n            };\n            popped += 1;\n            if self.is_unobserved(block.generation) {\n                unobserved = Some(block);\n                break;\n            }\n            if self.settle(block) {\n                return;\n            }\n        }\n        let Some(block) = unobserved else {\n            return;\n        };\n        self.observe(clock);\n        if self.settle(block) {\n            return;\n        }\n        let unpopped = available.saturating_sub(popped);\n        let rest = self.data_consumer.available_at_entry().min(unpopped);\n        for _ in 0..rest {\n            let Ok(block) = self.data_consumer.try_pop() else {\n                break;\n            };\n            if self.settle(block) {\n                return;\n            }\n        }\n    }\n}",
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

/// The collection's `cancel_except`: a drain loop over the captures that names `capture` only in
/// its count and its pop, then a reset loop of the same header with a pop-free call.
#[test]
fn the_collection_cancel_except_passes() {
    accepts(
        "twin-cancel-except",
        "impl Collection {\n    fn cancel_except(&mut self, keep: Option<usize>) {\n        for (index, capture) in self.captures.iter_mut().enumerate() {\n            if keep == Some(index) {\n                continue;\n            }\n            let available = capture.consumer.available_at_entry().min(RESULT_SLOTS);\n            for _ in 0..available {\n                if capture.consumer.try_pop().is_err() {\n                    break;\n                }\n            }\n        }\n        for (index, capture) in self.captures.iter_mut().enumerate() {\n            if keep == Some(index) {\n                continue;\n            }\n            capture.reset();\n        }\n    }\n}",
    );
}
