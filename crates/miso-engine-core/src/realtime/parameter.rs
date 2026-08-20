//! Fixed-capacity parameter values and sample-time event storage.

/// Pre-resolved parameter-schema slot.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParameterSlot(pub usize);
/// Epoch assigned to a successfully published render plan.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlanEpoch(pub u64);

/// A control event ordered by `(plan_epoch, absolute_sample)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParameterEvent {
    /// Prepared plan epoch against which the slot was resolved.
    pub plan_epoch: PlanEpoch,
    /// Absolute engine sample at which the point event takes effect.
    pub absolute_sample: u64,
    /// Pre-resolved parameter storage index.
    pub slot: ParameterSlot,
    /// Raw parameter-domain value; effect metadata owns validation and mapping.
    pub value: f32,
}

/// Fixed prepared defaults indexed directly by `ParameterSlot`.
pub struct ParameterValues {
    values: Box<[f32]>,
}
impl ParameterValues {
    /// Copy defaults into fixed prepared storage.
    #[must_use]
    pub fn new(defaults: &[f32]) -> Self {
        Self {
            values: defaults.to_vec().into_boxed_slice(),
        }
    }
    /// Get a pre-resolved slot.
    #[must_use]
    pub fn get(&self, slot: ParameterSlot) -> Option<f32> {
        self.values.get(slot.0).copied()
    }
    /// Set a pre-resolved slot.
    pub fn set(&mut self, slot: ParameterSlot, value: f32) -> bool {
        if let Some(target) = self.values.get_mut(slot.0) {
            *target = value;
            true
        } else {
            false
        }
    }
}
/// Event insertion error preserving ownership of the rejected event.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParameterEventError {
    /// Fixed event storage is full; contains the unconsumed event.
    Full(ParameterEvent),
    /// Event precedes the existing `(epoch, sample)` tail; contains the rejected event.
    OutOfOrder(ParameterEvent),
}
/// Prepared event storage. It never sorts, maps, or grows in render.
pub struct ParameterEventBuffer {
    events: Box<[ParameterEvent]>,
    len: usize,
    overflow_count: u64,
}
impl ParameterEventBuffer {
    /// Allocate exactly `capacity` event slots off render.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        let empty = ParameterEvent {
            plan_epoch: PlanEpoch(0),
            absolute_sample: 0,
            slot: ParameterSlot(0),
            value: 0.0,
        };
        Self {
            events: vec![empty; capacity].into_boxed_slice(),
            len: 0,
            overflow_count: 0,
        }
    }
    /// Number of logically stored events.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }
    /// Whether no event is currently queued.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// Prepared event capacity.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.events.len()
    }
    /// Saturating count of insertions rejected because storage was full.
    #[must_use]
    pub const fn overflow_count(&self) -> u64 {
        self.overflow_count
    }
    /// Empty the logical queue while retaining prepared memory.
    pub fn clear(&mut self) {
        self.len = 0;
    }
    /// Read the ordered event prefix.
    #[must_use]
    pub fn as_slice(&self) -> &[ParameterEvent] {
        &self.events[..self.len]
    }
    /// Append an already ordered event; reject and preserve it otherwise.
    pub fn try_push_ordered(&mut self, event: ParameterEvent) -> Result<(), ParameterEventError> {
        if self.len == self.events.len() {
            self.overflow_count = self.overflow_count.saturating_add(1);
            return Err(ParameterEventError::Full(event));
        }
        if self.len != 0 {
            let prior = self.events[self.len - 1];
            if (event.plan_epoch, event.absolute_sample) < (prior.plan_epoch, prior.absolute_sample)
            {
                return Err(ParameterEventError::OutOfOrder(event));
            }
        }
        self.events[self.len] = event;
        self.len += 1;
        Ok(())
    }
}
