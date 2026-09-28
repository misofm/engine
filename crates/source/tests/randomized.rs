//! Issue #1051: the source ring against an independent model, over random edge-biased schedules.
//!
//! `seek_schedule_model.rs` drives the production `PcmSourceRing` through 256 frozen schedules of
//! one fixed shape: one channel, a quantum of four, one scripted sequence of submissions, seeks
//! and renders per schedule. This file draws the schedules instead, per seed and edge-biased:
//!
//! * the quantum from 1 to 16 frames, the capacity from one to eight quanta, one to three
//!   channels, a starting generation of 1 or far from it;
//! * submissions that are valid, stale, from a generation not yet sought, one frame early or late,
//!   one frame too long or short, empty, of the wrong channel count or plane length, or at the
//!   wrong sample rate, and terminal chunks of every length;
//! * seeks to increasing, repeated, lower and zero generations, and a second seek before the first
//!   is applied (the one-slot backpressure);
//! * every sample word from the hostile set -- both zeros, subnormals, infinities, NaN payloads --
//!   which the ring must carry to the render bit for bit.
//!
//! Every submit and seek verdict, every rendered word of every channel, every read-report field
//! and the stale-discard counter must be what the model predicts. The model is written from the
//! documented rules, not from the ring: a bounded FIFO of whole blocks, the submission checks in
//! their documented order, seeks applied at the next block boundary, stale blocks discarded there
//! (a late terminal block of the active generation still declaring the region's end), and an
//! underrun rendered as `+0.0` and counted.

use std::collections::VecDeque;

use dsp_reference::randomized::{Draw, run_seeds};
use engine::{QuantumFrames, SampleRateHz};
use source::{
    HostChunkError, HostPlanarChunk, PcmSourceRing, PcmSourceRingConfig, SourceCommand,
    SourceFrame, SourceGeneration, SourceSeekError,
};

const TEST: &str = "random_schedules_match_the_independent_ring_model";
const REPLAY: &str = "cargo test -p source --test randomized -- --exact \
                      random_schedules_match_the_independent_ring_model";
const RATE: SampleRateHz = SampleRateHz(48_000);

/// A submission verdict, as a class: the model states the kind, not the payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Submit {
    Accepted,
    WrongRate,
    Stale,
    ChannelCount,
    FrameCount,
    NonContiguous,
    EndOfRegion,
    PlaneLength,
    Full,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Seek {
    Accepted,
    GenerationZero,
    NotIncreasing,
    Backpressure,
}

#[derive(Clone, Debug)]
struct Block {
    generation: u64,
    start: u64,
    frames: u32,
    end_of_region: bool,
    /// Per channel, per frame.
    words: Vec<Vec<u32>>,
}

/// What one render must produce.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Render {
    /// Per channel, `quantum` words.
    words: Vec<Vec<u32>>,
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

/// The ring's documented behaviour, with no ring in it.
struct Model {
    quantum: u32,
    channels: usize,
    capacity: usize,
    data: VecDeque<Block>,
    command: Option<(u64, u64)>,
    producer_generation: u64,
    producer_next: u64,
    producer_end: bool,
    active_generation: u64,
    next_frame: u64,
    end_frame: Option<u64>,
    end_of_region: bool,
    read_frames: u64,
    underrun_frames: u64,
    underrun_events: u64,
    stale_discards: u64,
}

impl Model {
    fn new(quantum: u32, channels: usize, capacity: usize, generation: u64) -> Self {
        Self {
            quantum,
            channels,
            capacity,
            data: VecDeque::new(),
            command: None,
            producer_generation: generation,
            producer_next: 0,
            producer_end: false,
            active_generation: generation,
            next_frame: 0,
            end_frame: None,
            end_of_region: false,
            read_frames: 0,
            underrun_frames: 0,
            underrun_events: 0,
            stale_discards: 0,
        }
    }

    /// The submission checks in their documented order: rate, generation, channel count, frame
    /// count, contiguity, end of region, plane lengths, room.
    fn submit(&mut self, chunk: &Chunk) -> Submit {
        if chunk.rate != RATE {
            return Submit::WrongRate;
        }
        if chunk.block.generation != self.producer_generation {
            return Submit::Stale;
        }
        if chunk.block.words.len() != self.channels {
            return Submit::ChannelCount;
        }
        let frames = chunk.block.frames;
        if frames > self.quantum
            || (frames < self.quantum && !chunk.block.end_of_region)
            || (frames == 0 && !chunk.block.end_of_region)
        {
            return Submit::FrameCount;
        }
        if chunk.block.start != self.producer_next {
            return Submit::NonContiguous;
        }
        if self.producer_end {
            return Submit::EndOfRegion;
        }
        if chunk
            .block
            .words
            .iter()
            .any(|plane| plane.len() != frames as usize)
        {
            return Submit::PlaneLength;
        }
        if self.data.len() == self.capacity {
            return Submit::Full;
        }
        self.producer_next += u64::from(frames);
        self.producer_end |= chunk.block.end_of_region;
        self.data.push_back(chunk.block.clone());
        Submit::Accepted
    }

    fn seek(&mut self, generation: u64, frame: u64) -> Seek {
        if generation == 0 {
            return Seek::GenerationZero;
        }
        if generation <= self.producer_generation {
            return Seek::NotIncreasing;
        }
        if self.command.is_some() {
            return Seek::Backpressure;
        }
        self.producer_generation = generation;
        self.producer_next = frame;
        self.producer_end = false;
        self.command = Some((generation, frame));
        Seek::Accepted
    }

    fn render(&mut self) -> Render {
        if let Some((generation, frame)) = self.command.take() {
            self.active_generation = generation;
            self.next_frame = frame;
            self.end_frame = None;
            self.end_of_region = false;
        }
        let quantum = self.quantum as usize;
        let mut selected = None;
        while let Some(block) = self.data.pop_front() {
            if block.generation != self.active_generation || block.start < self.next_frame {
                // A terminal block of the active generation that arrives behind the read position
                // still declares where the region ends; its samples are discarded.
                if block.end_of_region && block.generation == self.active_generation {
                    self.end_frame = Some(block.start + u64::from(block.frames));
                }
                self.stale_discards += 1;
                continue;
            }
            if block.end_of_region {
                self.end_frame = Some(block.start + u64::from(block.frames));
            }
            selected = Some(block);
            break;
        }
        let mut words = vec![vec![0_u32; quantum]; self.channels];
        let (mut copied_frames, mut underrun_frames) = (0, 0);
        if self.end_frame.is_some_and(|end| self.next_frame >= end) {
            // At the region's end the selected block is held, not played: it keeps its slot until
            // a seek discards it (a zero-frame terminal block is the case that reaches here).
            if let Some(block) = selected {
                self.data.push_front(block);
            }
            self.end_of_region = true;
        } else if let Some(block) = selected.take_if(|block| block.start == self.next_frame) {
            for (plane, source) in words.iter_mut().zip(&block.words) {
                plane[..block.frames as usize].copy_from_slice(source);
            }
            copied_frames = block.frames;
            self.next_frame += u64::from(block.frames);
            self.read_frames += u64::from(block.frames);
            self.end_of_region |= block.end_of_region;
        } else {
            // A block ahead of the read position stays queued for its frame.
            if let Some(block) = selected {
                self.data.push_front(block);
            }
            underrun_frames = self
                .end_frame
                .map_or(u64::from(self.quantum), |end| {
                    end.saturating_sub(self.next_frame)
                })
                .min(u64::from(self.quantum)) as u32;
            self.next_frame += u64::from(self.quantum);
            if underrun_frames != 0 {
                self.underrun_frames += u64::from(underrun_frames);
                self.underrun_events += 1;
            }
            if self.end_frame.is_some_and(|end| self.next_frame >= end) {
                self.end_of_region = true;
            }
        }
        Render {
            words,
            copied_frames,
            underrun_frames,
            underrun_event: underrun_frames != 0,
            end_of_region: self.end_of_region,
            active_generation: self.active_generation,
            cumulative_read_frames: self.read_frames,
            cumulative_underrun_frames: self.underrun_frames,
            cumulative_underrun_events: self.underrun_events,
            stale_discards: self.stale_discards,
        }
    }
}

struct Chunk {
    rate: SampleRateHz,
    block: Block,
}

/// One sample word: the hostile set half the time, an ordinary level otherwise.
fn word(draw: &mut Draw) -> u32 {
    if draw.chance(1, 2) {
        draw.hostile().to_bits()
    } else if draw.chance(1, 4) {
        draw.subnormal().to_bits()
    } else {
        draw.noise(1.0).to_bits()
    }
}

/// One submission aimed at the model's producer, a fifth of the time wrong in one way.
fn draw_chunk(draw: &mut Draw, model: &Model) -> Chunk {
    let quantum = model.quantum;
    let mut generation = model.producer_generation;
    let mut start = model.producer_next;
    let mut end_of_region = draw.chance(1, 8);
    let mut frames = if end_of_region {
        draw.pick(&[0, 1, quantum / 2, quantum.saturating_sub(1), quantum])
    } else {
        quantum
    };
    let mut channels = model.channels;
    let mut rate = RATE;
    let mut ragged = false;
    if draw.chance(1, 5) {
        match draw.below(8) {
            0 => generation = generation.saturating_sub(1).max(1),
            1 => generation += 1,
            2 => start += 1,
            3 => start = start.saturating_sub(1),
            4 => frames = quantum + 1,
            5 => {
                frames = draw.pick(&[0, quantum.saturating_sub(1)]);
                end_of_region = false;
            }
            6 => {
                channels = if draw.chance(1, 2) {
                    channels + 1
                } else {
                    channels - 1
                }
            }
            _ => {
                if draw.chance(1, 2) {
                    rate = SampleRateHz(44_100);
                } else {
                    ragged = true;
                }
            }
        }
    }
    let words = (0..channels)
        .map(|channel| {
            let length = if ragged && channel == 0 {
                frames as usize + 1
            } else {
                frames as usize
            };
            (0..length).map(|_| word(draw)).collect()
        })
        .collect();
    Chunk {
        rate,
        block: Block {
            generation,
            start,
            frames,
            end_of_region,
            words,
        },
    }
}

fn classify(result: &Result<source::SubmitReport, HostChunkError>) -> Submit {
    match result {
        Ok(_) => Submit::Accepted,
        Err(HostChunkError::WrongSampleRate { .. }) => Submit::WrongRate,
        Err(HostChunkError::StaleGeneration { .. }) => Submit::Stale,
        Err(HostChunkError::ChannelCount { .. }) => Submit::ChannelCount,
        Err(HostChunkError::FrameCount { .. }) => Submit::FrameCount,
        Err(HostChunkError::NonContiguous { .. }) => Submit::NonContiguous,
        Err(HostChunkError::EndOfRegionAlreadySubmitted) => Submit::EndOfRegion,
        Err(HostChunkError::PlaneLength { .. }) => Submit::PlaneLength,
        Err(HostChunkError::Full { .. }) => Submit::Full,
        Err(error) => panic!("an unmodelled refusal {error:?}"),
    }
}

/// What the schedules reached, so a generator that stopped reaching a rule is a red.
#[derive(Debug, Default)]
struct Reach {
    submits: [u64; 9],
    seeks: [u64; 4],
    renders: u64,
    underruns: u64,
    ends: u64,
}

#[allow(clippy::too_many_lines)]
fn schedule(seed: u64, reach: &mut Reach) {
    let mut draw = Draw::new(seed);
    let quantum = draw.pick(&[1_u32, 2, 3, 4, 4, 8, 16]);
    let capacity = draw.pick(&[1_usize, 2, 3, 8]);
    let channels = draw.pick(&[1_usize, 1, 2, 3]);
    let generation = draw.pick(&[1_u64, 1, 1_000]);
    let config = PcmSourceRingConfig {
        channel_count: channels as u32,
        quantum_frames: QuantumFrames(quantum),
        frame_capacity: capacity as u64 * u64::from(quantum),
        initial_generation: SourceGeneration(generation),
    };
    let (producer, mut consumer, _) = PcmSourceRing::prepare(config).expect("a legal ring");
    let mut host = producer.into_host_chunk_provider(RATE);
    let mut model = Model::new(quantum, channels, capacity, generation);
    let mut planes: Vec<Vec<f32>> =
        vec![vec![f32::from_bits(0xffff_ffff); quantum as usize]; channels];
    for step in 0..96 {
        let context = format!(
            "seed {seed} step {step} (quantum {quantum}, capacity {capacity}, {channels} channels)"
        );
        match draw.below(20) {
            0..=9 => {
                let chunk = draw_chunk(&mut draw, &model);
                let floats: Vec<Vec<f32>> = chunk
                    .block
                    .words
                    .iter()
                    .map(|plane| plane.iter().map(|bits| f32::from_bits(*bits)).collect())
                    .collect();
                let borrowed: Vec<&[f32]> = floats.iter().map(Vec::as_slice).collect();
                let actual = host.submit(HostPlanarChunk {
                    sample_rate_hz: chunk.rate,
                    generation: SourceGeneration(chunk.block.generation),
                    start_frame: SourceFrame(chunk.block.start),
                    planes: &borrowed,
                    frames: chunk.block.frames,
                    end_of_region: chunk.block.end_of_region,
                });
                let expected = model.submit(&chunk);
                assert_eq!(classify(&actual), expected, "{context}: submit {actual:?}");
                reach.submits[expected as usize] += 1;
            }
            10..=11 => {
                let target = match draw.below(8) {
                    0 => 0,
                    1 => model.producer_generation,
                    2 => model.producer_generation.saturating_sub(1),
                    _ => model.producer_generation + 1 + draw.below(3) as u64,
                };
                let far = draw.next_u64() >> 20;
                let frame = draw.pick(&[
                    0_u64,
                    model.next_frame,
                    model.producer_next,
                    model.next_frame + u64::from(quantum),
                    far,
                ]);
                let actual = match host.try_seek(SourceCommand::Seek {
                    generation: SourceGeneration(target),
                    frame: SourceFrame(frame),
                }) {
                    Ok(()) => Seek::Accepted,
                    Err(SourceSeekError::GenerationZero) => Seek::GenerationZero,
                    Err(SourceSeekError::GenerationNotStrictlyIncreasing { .. }) => {
                        Seek::NotIncreasing
                    }
                    Err(SourceSeekError::Backpressure { .. }) => Seek::Backpressure,
                };
                let expected = model.seek(target, frame);
                assert_eq!(
                    actual, expected,
                    "{context}: seek to generation {target} frame {frame}"
                );
                reach.seeks[expected as usize] += 1;
            }
            _ => {
                for plane in &mut planes {
                    plane.fill(f32::from_bits(0xffff_ffff));
                }
                let mut outputs: Vec<&mut [f32]> =
                    planes.iter_mut().map(Vec::as_mut_slice).collect();
                let report = consumer.read_block(&mut outputs).expect("a legal read");
                let expected = model.render();
                let words: Vec<Vec<u32>> = planes
                    .iter()
                    .map(|plane| plane.iter().map(|value| value.to_bits()).collect())
                    .collect();
                let actual = Render {
                    words,
                    copied_frames: report.copied_frames,
                    underrun_frames: report.underrun_frames,
                    underrun_event: report.underrun_event,
                    end_of_region: report.end_of_region,
                    active_generation: report.active_generation.0,
                    cumulative_read_frames: report.cumulative_read_frames,
                    cumulative_underrun_frames: report.cumulative_underrun_frames,
                    cumulative_underrun_events: report.cumulative_underrun_events,
                    stale_discards: consumer.telemetry().stale_generation_discard_count,
                };
                assert_eq!(actual, expected, "{context}: render");
                reach.renders += 1;
                reach.underruns += u64::from(expected.underrun_event);
                reach.ends += u64::from(expected.end_of_region);
            }
        }
    }
}

#[test]
fn random_schedules_match_the_independent_ring_model() {
    let mut reach = Reach::default();
    let seeds = run_seeds(TEST, REPLAY, 256, |seed| schedule(seed, &mut reach));
    println!("{seeds} seeds: {reach:?}");
    assert!(
        reach.submits.iter().all(|count| *count > 0)
            && reach.seeks.iter().all(|count| *count > 0)
            && reach.underruns > 0
            && reach.ends > 0,
        "every verdict must be reached: {reach:?}"
    );
}
