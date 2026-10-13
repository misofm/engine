//! A lane under test with the plan's revision gate around it: the control side stamps and
//! publishes transactions the way a host does, and the render side reads under the snapshot of
//! the gate, one block at a time (decision 15, D15-2; issue #1345).
#![allow(dead_code)]

use effect_contract::{
    AutomationSpanKind, ChannelSymmetryWitness, EffectCellWriter, EffectControlLane,
    EffectControlRecord, EffectLaneShape, EffectLaneSpec, EffectWriteRefusal,
    PREPARED_EFFECT_TARGET_WORDS, ParameterChannel, PreparedAutomationSpan, PreparedEffectTarget,
    Staged, effect_control_lane,
};
use engine::realtime::{GatePin, LiveSnapshot, LiveValuesSuperseded, RevisionGate};

pub(crate) const LIVE: u8 = ChannelSymmetryWitness::LIVE;
pub(crate) const UNBYPASSED: u8 = ChannelSymmetryWitness::UNBYPASSED;

pub(crate) fn idle() -> PreparedAutomationSpan {
    PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel: ParameterChannel::Both,
        parameter_index: u32::MAX,
        start_sample: 0,
        end_sample: 0,
        start_value: 0.0,
        end_value: 0.0,
    }
}

pub(crate) fn parameter(index: u32, channel: ParameterChannel, value: f32) -> EffectControlRecord {
    EffectControlRecord::Parameter {
        parameter_index: index,
        channel,
        value,
    }
}

pub(crate) fn left(index: u32, value: f32) -> EffectControlRecord {
    parameter(index, ParameterChannel::Left, value)
}

pub(crate) fn right(index: u32, value: f32) -> EffectControlRecord {
    parameter(index, ParameterChannel::Right, value)
}

pub(crate) fn both(index: u32, value: f32) -> EffectControlRecord {
    parameter(index, ParameterChannel::Both, value)
}

pub(crate) fn target(slot: u32, channel: ParameterChannel, marker: u32) -> EffectControlRecord {
    EffectControlRecord::PreparedTarget(PreparedEffectTarget {
        slot,
        channel,
        words: [marker; PREPARED_EFFECT_TARGET_WORDS],
    })
}

/// One lane, its writer, the plan's gate and a staging window.
pub(crate) struct Rig {
    pub(crate) gate: RevisionGate,
    pub(crate) pin: GatePin,
    pub(crate) writer: EffectCellWriter,
    pub(crate) lane: EffectControlLane,
    pub(crate) superseded: LiveValuesSuperseded,
    pub(crate) staging: Vec<PreparedAutomationSpan>,
    pub(crate) block: u64,
}

impl Rig {
    /// A lane of `shape` with a staging window of exactly its parameter cells.
    pub(crate) fn new(shape: EffectLaneShape, observation_depth: usize) -> Self {
        let window = shape.parameter_cells();
        Self::with_window(shape, observation_depth, window)
    }

    pub(crate) fn with_window(
        shape: EffectLaneShape,
        observation_depth: usize,
        window: usize,
    ) -> Self {
        let superseded = LiveValuesSuperseded::new();
        let (writer, lane) = effect_control_lane(
            EffectLaneSpec {
                shape,
                bypass: false,
                observation_depth,
            },
            &superseded,
        )
        .expect("a lane");
        let (gate, pin) = RevisionGate::pair(0);
        Self {
            gate,
            pin,
            writer,
            lane,
            superseded,
            staging: vec![idle(); window],
            block: 0,
        }
    }

    /// Preflight every record, then write them all under one stamp and publish its revision: one
    /// transaction. Returns the refusal of the first record the lane has no cell for, with
    /// nothing written and no revision used.
    pub(crate) fn commit(
        &mut self,
        records: &[EffectControlRecord],
    ) -> Result<(), EffectWriteRefusal> {
        for record in records {
            self.writer.check(record)?;
        }
        let revision = self.pin.last_published() + 1;
        let stamp = self.pin.stamp(revision).expect("a legal revision");
        for record in records {
            self.writer.write(stamp, *record).expect("preflighted");
        }
        self.pin.publish(&self.gate, revision);
        Ok(())
    }

    /// Each record as a transaction of its own, in order.
    pub(crate) fn commit_each(&mut self, records: &[EffectControlRecord]) {
        for record in records {
            self.commit(core::slice::from_ref(record))
                .expect("every record has a cell");
        }
    }

    /// One block: the gate's snapshot, then the lane's cells under it.
    pub(crate) fn stage(&mut self) -> Staged {
        let snapshot = self.gate.snapshot();
        self.stage_under(snapshot)
    }

    pub(crate) fn stage_under(&mut self, snapshot: LiveSnapshot) -> Staged {
        let first_sample = self.block * 128;
        self.block += 1;
        self.lane.stage(&mut self.staging, first_sample, snapshot)
    }

    pub(crate) fn live(&self) -> bool {
        self.lane.symmetry().holds(LIVE)
    }

    pub(crate) fn staged_spans(&self, staged: Staged) -> Vec<(u32, ParameterChannel, u32)> {
        self.staging[..staged.staged]
            .iter()
            .map(|span| {
                (
                    span.parameter_index,
                    span.channel,
                    span.start_value.to_bits(),
                )
            })
            .collect()
    }
}
