//! The shared ring's seek and admission semantics against an independent model.
//!
//! 256 frozen schedules (bounded queues of one, two, three and eight quanta) drive the production
//! `PcmSourceRing` through its host endpoint, `HostChunkProvider`, which is the path the browser
//! adapter and the C ABI both feed: complete and short end-of-region chunks, a submission into a
//! full queue that must admit no prefix, stale-generation and late-generation blocks, two seeks per
//! schedule and underruns. Every submit result, every rendered sample, every read report field and
//! the stale-discard counter must match what the small model below predicts, and the generated
//! transcript is pinned so the schedules cannot drift.
//!
//! Moved here unchanged by #1033 from `audit fixture-source` (`tools/audit/src/source_fixture.rs`),
//! whose other half checked the native WAV/RF64 decoder that #1033 deleted.

use std::collections::VecDeque;

use engine::{QuantumFrames, SampleRateHz};
use sha2::{Digest, Sha256};
use source::{
    HostChunkError, HostPlanarChunk, PcmSourceRing, PcmSourceRingConfig, SourceCommand,
    SourceFrame, SourceGeneration,
};

#[test]
fn frozen_seek_schedules_match_the_independent_ring_model() {
    frozen_seek_ring_schedules().expect("frozen seek schedules");
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

// This is a deliberately small independent schedule oracle, not another source ring. It models
// only the frozen action language below: one producer, one bounded FIFO of complete quanta, and
// block-boundary seeks. The production ring is exercised only after this model has produced every
// expected outcome from the sealed transcript.
const SEEK_SCHEDULE_SEED: u64 = 0x0000_0000_010a_5ee1;
const SEEK_SCHEDULE_COUNT: usize = 256;
const SEEK_QUANTUM: u32 = 4;
const FROZEN_SEEK_TRANSCRIPT_SHA256: &str =
    "ec3b7fef8e86937d4431466d2cea8a68ec56feb2897bcdc655fa10d5bf30a41c";

#[derive(Clone, Debug)]
struct SeekSchedule {
    index: u16,
    capacity_quanta: usize,
    actions: Vec<SeekAction>,
}

#[derive(Clone, Debug)]
enum SeekAction {
    Submit {
        generation: u64,
        start_frame: u64,
        frames: u32,
        end_of_region: bool,
        sample_bits: u32,
    },
    Seek {
        generation: u64,
        frame: u64,
    },
    Render,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ModelSubmit {
    Accepted,
    Full,
    StaleGeneration,
    EndOfRegion,
    Invalid,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ModelOutcome {
    Submit(ModelSubmit),
    Seek,
    Render(ModelRender),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ModelRender {
    output_bits: [u32; SEEK_QUANTUM as usize],
    copied_frames: u32,
    underrun_frames: u32,
    underrun_event: bool,
    end_of_region: bool,
    active_generation: u64,
    cumulative_read_frames: u64,
    cumulative_underrun_frames: u64,
    cumulative_underrun_events: u64,
    stale_discards: u64,
}

#[derive(Clone, Copy, Debug)]
struct ModelBlock {
    generation: u64,
    start_frame: u64,
    frames: u32,
    end_of_region: bool,
    sample_bits: u32,
}

struct IndependentSeekModel {
    capacity: usize,
    free_blocks: usize,
    data: VecDeque<ModelBlock>,
    command: Option<(u64, u64)>,
    active_generation: u64,
    producer_generation: u64,
    producer_next_frame: u64,
    producer_end_submitted: bool,
    next_frame: u64,
    end_frame: Option<u64>,
    end_of_region: bool,
    cumulative_read_frames: u64,
    stale_discards: u64,
    underrun_frames: u64,
    underrun_events: u64,
}

impl IndependentSeekModel {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            free_blocks: capacity,
            data: VecDeque::with_capacity(capacity),
            command: None,
            active_generation: 1,
            producer_generation: 1,
            producer_next_frame: 0,
            producer_end_submitted: false,
            next_frame: 0,
            end_frame: None,
            end_of_region: false,
            cumulative_read_frames: 0,
            stale_discards: 0,
            underrun_frames: 0,
            underrun_events: 0,
        }
    }

    fn submit(&mut self, block: ModelBlock) -> ModelSubmit {
        if block.generation != self.producer_generation {
            return ModelSubmit::StaleGeneration;
        }
        if self.producer_end_submitted {
            return ModelSubmit::EndOfRegion;
        }
        if block.start_frame != self.producer_next_frame
            || block.frames > SEEK_QUANTUM
            || (block.frames < SEEK_QUANTUM && !block.end_of_region)
            || (block.frames == 0 && !block.end_of_region)
        {
            return ModelSubmit::Invalid;
        }
        if self.free_blocks == 0 || self.data.len() == self.capacity {
            return ModelSubmit::Full;
        }
        self.free_blocks -= 1;
        self.producer_next_frame = self
            .producer_next_frame
            .saturating_add(u64::from(block.frames));
        if block.end_of_region {
            self.producer_end_submitted = true;
        }
        self.data.push_back(block);
        ModelSubmit::Accepted
    }

    fn seek(&mut self, generation: u64, frame: u64) -> Result<(), String> {
        if generation <= self.producer_generation {
            return Err("schedule generated a non-increasing seek".to_owned());
        }
        if self.command.is_some() {
            return Err("schedule overfilled its bounded seek command slot".to_owned());
        }
        self.producer_generation = generation;
        self.producer_next_frame = frame;
        self.producer_end_submitted = false;
        self.command = Some((generation, frame));
        Ok(())
    }

    fn render(&mut self) -> ModelRender {
        if let Some((generation, frame)) = self.command.take() {
            self.active_generation = generation;
            self.next_frame = frame;
            self.end_frame = None;
            self.end_of_region = false;
        }
        let mut selected = None;
        while let Some(block) = self.data.pop_front() {
            if block.generation != self.active_generation || block.start_frame < self.next_frame {
                self.stale_discards = self.stale_discards.saturating_add(1);
                self.free_blocks += 1;
                continue;
            }
            if block.end_of_region {
                self.end_frame = Some(block.start_frame.saturating_add(u64::from(block.frames)));
            }
            selected = Some(block);
            break;
        }

        let mut output_bits = [0_u32; SEEK_QUANTUM as usize];
        let mut copied_frames = 0;
        let mut underrun_frames = 0;
        if self.end_frame.is_some_and(|end| self.next_frame >= end) {
            self.end_of_region = true;
        } else if let Some(block) = selected {
            if block.start_frame == self.next_frame {
                let frames = usize::try_from(block.frames).expect("frozen quantum fits usize");
                output_bits[..frames].fill(block.sample_bits);
                copied_frames = block.frames;
                self.next_frame = self.next_frame.saturating_add(u64::from(block.frames));
                self.cumulative_read_frames = self
                    .cumulative_read_frames
                    .saturating_add(u64::from(block.frames));
                if block.end_of_region {
                    self.end_of_region = true;
                }
                self.free_blocks += 1;
            } else {
                // The selected future block remains queued in the production endpoint. Frozen
                // schedules never produce one, so treat it as a generator/model defect.
                self.data.push_front(block);
                underrun_frames = self.available_until_end();
                self.note_underrun(underrun_frames);
            }
        } else {
            underrun_frames = self.available_until_end();
            self.note_underrun(underrun_frames);
        }

        ModelRender {
            output_bits,
            copied_frames,
            underrun_frames,
            underrun_event: underrun_frames != 0,
            end_of_region: self.end_of_region,
            active_generation: self.active_generation,
            cumulative_read_frames: self.cumulative_read_frames,
            cumulative_underrun_frames: self.underrun_frames,
            cumulative_underrun_events: self.underrun_events,
            stale_discards: self.stale_discards,
        }
    }

    fn available_until_end(&self) -> u32 {
        self.end_frame
            .map(|end| end.saturating_sub(self.next_frame))
            .unwrap_or(u64::from(SEEK_QUANTUM))
            .min(u64::from(SEEK_QUANTUM)) as u32
    }

    fn note_underrun(&mut self, frames: u32) {
        self.next_frame = self.next_frame.saturating_add(u64::from(SEEK_QUANTUM));
        if frames != 0 {
            self.underrun_frames = self.underrun_frames.saturating_add(u64::from(frames));
            self.underrun_events = self.underrun_events.saturating_add(1);
        }
        if self.end_frame.is_some_and(|end| self.next_frame >= end) {
            self.end_of_region = true;
        }
    }
}

fn frozen_seek_ring_schedules() -> Result<(), String> {
    let schedules = generate_frozen_seek_schedules();
    if schedules.len() != SEEK_SCHEDULE_COUNT {
        return Err(format!("expected {SEEK_SCHEDULE_COUNT} schedules"));
    }
    let transcript = seek_transcript_sha256(&schedules);
    if transcript != FROZEN_SEEK_TRANSCRIPT_SHA256 {
        return Err(format!(
            "frozen seek transcript mismatch: expected {FROZEN_SEEK_TRANSCRIPT_SHA256}, actual {transcript}"
        ));
    }
    for schedule in &schedules {
        let expected = model_schedule(schedule)?;
        exercise_production_schedule(schedule, &expected)?;
    }
    Ok(())
}

fn generate_frozen_seek_schedules() -> Vec<SeekSchedule> {
    let capacities = [1_usize, 2, 3, 8];
    let mut state = SEEK_SCHEDULE_SEED;
    (0..SEEK_SCHEDULE_COUNT)
        .map(|index| {
            state = xorshift64(state);
            let capacity_quanta = capacities[index % capacities.len()];
            let base = 1_024_u64 + u64::try_from(index).expect("fixed count") * 1_024;
            let first_seek = base;
            let second_seek = base + 512;
            let first_terminal_frames = if index % 17 == 0 {
                0
            } else {
                1 + u32::try_from(state & 0x03).expect("two bits") % (SEEK_QUANTUM - 1)
            };
            let second_terminal_frames = if index % 19 == 0 {
                0
            } else {
                1 + u32::try_from((state >> 8) & 0x03).expect("two bits") % (SEEK_QUANTUM - 1)
            };
            let mut actions = Vec::with_capacity(capacity_quanta * 3 + 20);
            for slot in 0..capacity_quanta {
                actions.push(SeekAction::Submit {
                    generation: 1,
                    start_frame: u64::try_from(slot).expect("small slot") * u64::from(SEEK_QUANTUM),
                    frames: SEEK_QUANTUM,
                    end_of_region: false,
                    sample_bits: schedule_sample_bits(index, slot, 1),
                });
            }
            // This is the frozen full transition; it must accept no prefix or state change.
            actions.push(SeekAction::Submit {
                generation: 1,
                start_frame: u64::try_from(capacity_quanta).expect("small capacity")
                    * u64::from(SEEK_QUANTUM),
                frames: SEEK_QUANTUM,
                end_of_region: false,
                sample_bits: schedule_sample_bits(index, capacity_quanta, 1),
            });
            actions.push(SeekAction::Seek {
                generation: 2,
                frame: first_seek,
            });
            actions.push(SeekAction::Render);
            // This late new-generation block is discarded after the declared request boundary.
            actions.push(SeekAction::Submit {
                generation: 2,
                start_frame: first_seek,
                frames: SEEK_QUANTUM,
                end_of_region: false,
                sample_bits: schedule_sample_bits(index, 0, 2),
            });
            actions.push(SeekAction::Render);
            actions.push(SeekAction::Submit {
                generation: 2,
                start_frame: first_seek + u64::from(SEEK_QUANTUM) * 2,
                frames: SEEK_QUANTUM,
                end_of_region: false,
                sample_bits: schedule_sample_bits(index, 1, 2),
            });
            actions.push(SeekAction::Render);
            for round in 0..=capacity_quanta {
                let start_frame = first_seek
                    + u64::from(SEEK_QUANTUM) * u64::try_from(round + 3).expect("small wrap round");
                actions.push(SeekAction::Submit {
                    generation: 2,
                    start_frame,
                    frames: SEEK_QUANTUM,
                    end_of_region: false,
                    sample_bits: schedule_sample_bits(index, round + 2, 2),
                });
                actions.push(SeekAction::Render);
            }
            let first_terminal_start = first_seek
                + u64::from(SEEK_QUANTUM)
                    * u64::try_from(capacity_quanta + 4).expect("small terminal round");
            actions.push(SeekAction::Submit {
                generation: 2,
                start_frame: first_terminal_start,
                frames: first_terminal_frames,
                end_of_region: true,
                sample_bits: schedule_sample_bits(index, capacity_quanta + 3, 2),
            });
            actions.push(SeekAction::Render);
            actions.push(SeekAction::Render);
            actions.push(SeekAction::Seek {
                generation: 3,
                frame: second_seek,
            });
            actions.push(SeekAction::Render);
            // A delayed old-generation attempt is rejected at the producer boundary.
            actions.push(SeekAction::Submit {
                generation: 2,
                start_frame: second_seek,
                frames: SEEK_QUANTUM,
                end_of_region: false,
                sample_bits: schedule_sample_bits(index, 0, 3),
            });
            actions.push(SeekAction::Submit {
                generation: 3,
                start_frame: second_seek,
                frames: SEEK_QUANTUM,
                end_of_region: false,
                sample_bits: schedule_sample_bits(index, 1, 3),
            });
            actions.push(SeekAction::Render);
            actions.push(SeekAction::Submit {
                generation: 3,
                start_frame: second_seek + u64::from(SEEK_QUANTUM) * 2,
                frames: second_terminal_frames,
                end_of_region: true,
                sample_bits: schedule_sample_bits(index, 2, 3),
            });
            actions.push(SeekAction::Render);
            actions.push(SeekAction::Render);
            SeekSchedule {
                index: u16::try_from(index).expect("fixed schedule count"),
                capacity_quanta,
                actions,
            }
        })
        .collect()
}

fn model_schedule(schedule: &SeekSchedule) -> Result<Vec<ModelOutcome>, String> {
    let mut model = IndependentSeekModel::new(schedule.capacity_quanta);
    schedule
        .actions
        .iter()
        .map(|action| match action {
            SeekAction::Submit {
                generation,
                start_frame,
                frames,
                end_of_region,
                sample_bits,
            } => Ok(ModelOutcome::Submit(model.submit(ModelBlock {
                generation: *generation,
                start_frame: *start_frame,
                frames: *frames,
                end_of_region: *end_of_region,
                sample_bits: *sample_bits,
            }))),
            SeekAction::Seek { generation, frame } => {
                model.seek(*generation, *frame)?;
                Ok(ModelOutcome::Seek)
            }
            SeekAction::Render => Ok(ModelOutcome::Render(model.render())),
        })
        .collect()
}

fn exercise_production_schedule(
    schedule: &SeekSchedule,
    expected: &[ModelOutcome],
) -> Result<(), String> {
    let config = PcmSourceRingConfig {
        channel_count: 1,
        quantum_frames: QuantumFrames(SEEK_QUANTUM),
        frame_capacity: u64::try_from(schedule.capacity_quanta).expect("small capacity")
            * u64::from(SEEK_QUANTUM),
        initial_generation: SourceGeneration(1),
    };
    let (producer, mut consumer, _) = PcmSourceRing::prepare(config)
        .map_err(|error| format!("schedule {} ring prepare: {error:?}", schedule.index))?;
    let mut host = producer.into_host_chunk_provider(SampleRateHz(48_000));
    let mut output = [f32::from_bits(0xffff_ffff); SEEK_QUANTUM as usize];
    for (step, (action, expected)) in schedule.actions.iter().zip(expected).enumerate() {
        match (action, expected) {
            (
                SeekAction::Submit {
                    generation,
                    start_frame,
                    frames,
                    end_of_region,
                    sample_bits,
                },
                ModelOutcome::Submit(expected_submit),
            ) => {
                let plane = [f32::from_bits(*sample_bits); SEEK_QUANTUM as usize];
                let planes = [&plane[..usize::try_from(*frames).expect("frozen frame count")]];
                let actual = host.submit(HostPlanarChunk {
                    sample_rate_hz: SampleRateHz(48_000),
                    generation: SourceGeneration(*generation),
                    start_frame: SourceFrame(*start_frame),
                    planes: &planes,
                    frames: *frames,
                    end_of_region: *end_of_region,
                });
                let actual_submit = match &actual {
                    Ok(_) => ModelSubmit::Accepted,
                    Err(HostChunkError::Full { .. }) => ModelSubmit::Full,
                    Err(HostChunkError::StaleGeneration { .. }) => ModelSubmit::StaleGeneration,
                    Err(HostChunkError::EndOfRegionAlreadySubmitted) => ModelSubmit::EndOfRegion,
                    Err(_) => ModelSubmit::Invalid,
                };
                if actual_submit != *expected_submit {
                    return Err(format!(
                        "schedule {} step {step} submit mismatch: expected {expected_submit:?}, actual {actual_submit:?} ({actual:?})",
                        schedule.index,
                    ));
                }
            }
            (SeekAction::Seek { generation, frame }, ModelOutcome::Seek) => host
                .try_seek(SourceCommand::Seek {
                    generation: SourceGeneration(*generation),
                    frame: SourceFrame(*frame),
                })
                .map_err(|error| {
                    format!("schedule {} step {step} seek: {error:?}", schedule.index)
                })?,
            (SeekAction::Render, ModelOutcome::Render(expected_render)) => {
                let report = consumer.read_block(&mut [&mut output]).map_err(|error| {
                    format!("schedule {} step {step} render: {error:?}", schedule.index)
                })?;
                let actual_bits = output.map(f32::to_bits);
                if actual_bits != expected_render.output_bits
                    || report.copied_frames != expected_render.copied_frames
                    || report.underrun_frames != expected_render.underrun_frames
                    || report.underrun_event != expected_render.underrun_event
                    || report.end_of_region != expected_render.end_of_region
                    || report.active_generation
                        != SourceGeneration(expected_render.active_generation)
                    || report.cumulative_read_frames != expected_render.cumulative_read_frames
                    || report.cumulative_underrun_frames
                        != expected_render.cumulative_underrun_frames
                    || report.cumulative_underrun_events
                        != expected_render.cumulative_underrun_events
                    || consumer.telemetry().stale_generation_discard_count
                        != expected_render.stale_discards
                {
                    return Err(format!(
                        "schedule {} step {step} render/model mismatch: expected {expected_render:?}, actual bits {actual_bits:?}, report {report:?}, telemetry {:?}",
                        schedule.index,
                        consumer.telemetry()
                    ));
                }
            }
            _ => {
                return Err(format!(
                    "schedule {} step {step} internal action/outcome mismatch",
                    schedule.index
                ));
            }
        }
    }
    Ok(())
}

fn seek_transcript_sha256(schedules: &[SeekSchedule]) -> String {
    let mut bytes = Vec::new();
    for schedule in schedules {
        bytes.extend_from_slice(&schedule.index.to_le_bytes());
        bytes.extend_from_slice(
            &u64::try_from(schedule.capacity_quanta)
                .expect("small capacity")
                .to_le_bytes(),
        );
        bytes.extend_from_slice(
            &u64::try_from(schedule.actions.len())
                .expect("small action count")
                .to_le_bytes(),
        );
        for action in &schedule.actions {
            match action {
                SeekAction::Submit {
                    generation,
                    start_frame,
                    frames,
                    end_of_region,
                    sample_bits,
                } => {
                    bytes.push(1);
                    bytes.extend_from_slice(&generation.to_le_bytes());
                    bytes.extend_from_slice(&start_frame.to_le_bytes());
                    bytes.extend_from_slice(&frames.to_le_bytes());
                    bytes.push(u8::from(*end_of_region));
                    bytes.extend_from_slice(&sample_bits.to_le_bytes());
                }
                SeekAction::Seek { generation, frame } => {
                    bytes.push(2);
                    bytes.extend_from_slice(&generation.to_le_bytes());
                    bytes.extend_from_slice(&frame.to_le_bytes());
                }
                SeekAction::Render => bytes.push(3),
            }
        }
    }
    sha256_hex(&bytes)
}

fn xorshift64(mut state: u64) -> u64 {
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    state
}

fn schedule_sample_bits(schedule: usize, slot: usize, generation: u64) -> u32 {
    let value = (schedule as f32 + 1.0) * 0.001 + slot as f32 * 0.01 + generation as f32 * 0.1;
    value.to_bits()
}
