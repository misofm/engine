//! Issue #1747: a render-time lane swap moves a strip's whole state between two lanes of one bank
//! chain, or of two chains, and allocates nothing.
//!
//! The bank under test is a stand-in effect with real per-lane state: a one-pole accumulator and a
//! gain ramp per channel, both in its state payload, so every rendered sample depends on the lane's
//! history. Its gain is the live parameter, and its two observation taps read the accumulator and
//! the gain. Each lane also keeps eight bytes of prepared per-lane defaults outside its payload.
//! A collapsing variant of the same bank declares the mono-collapse trio, so a chain armed for
//! mono sources renders it one-plane. What is gated is the rack's swap: that every per-lane field
//! -- payload, prepared defaults, control lane, observation queue, taps, dry shunt history, the
//! witness terms each stage caches and the chains' channel agreement -- follows the strip.
//!
//! This file has its own test binary so that `bench_support`'s audited allocator serves it.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use effect_contract::{
    BankProcessReport, BankWidth, BypassSwitch, EffectBankProcessBlock, EffectCellWriter,
    EffectControlLane, EffectControlRecord, EffectId, EffectLaneShape, EffectLaneSpec,
    EffectProgramKey, EffectQuality, EffectTargetError, LatencySamples, LinkMode,
    ObservationCadence, ObservationChannels, ObservationCost, ObservationDescriptor,
    ObservationFold, ObservationKind, ObservationLane, ObservationSample, ObservationTapId,
    PREPARED_EFFECT_TARGET_WORDS, ParameterChannel, ParameterChannelPolicy, ParameterUnit,
    PreparedBankMetadata, PreparedEffectBank, PreparedEffectTarget, PreparedNativeEffectBank,
    PreparedPorts, PreparedSidechainPort, ResetKind, StatePayloadError, StatePayloadInput,
    StatePayloadOutput, StatePayloadSizes, TailSamples, effect_control_lane,
};
use engine::realtime::{
    GatePin, LiveSnapshot, LiveValuesSuperseded, ObservationReader, RevisionGate, observation_slot,
};
use rack::{
    AoSoaScratch, BankChain, BankMembers, BankSlot, BankStage, ChainLanePeer, ChainLaneSwapRefusal,
    EffectBankStage, LiveControlEffectBankStage,
};

const QUANTUM: u32 = 8;
const LATENCY: usize = 3;
/// Frames a gain retarget ramps over: longer than a block, so a swap lands mid-ramp.
const GAIN_RAMP: u32 = 11;
/// Frames a bypass switch crossfades over.
const BYPASS_RAMP: u32 = 20;
/// Words per channel of one lane's payload: accumulator, gain, target, step, remaining, pole.
const CHANNEL_WORDS: usize = 6;
/// Every channel's accumulator pole, unless skewed.
const POLE: f32 = 0.875;
/// The right channel's pole of a skewed lane: a designed word the two channels do not share.
const SKEWED_POLE: f32 = 0.5;
const SIZES: StatePayloadSizes = StatePayloadSizes {
    common_bytes: 0,
    left_bytes: (CHANNEL_WORDS * 4) as u32,
    right_bytes: (CHANNEL_WORDS * 4) as u32,
};

/// SplitMix64.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound as u64) as usize
    }

    /// A value in `[-1, 1)`.
    fn signed(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1_u64 << 23) as f32 - 1.0
    }
}

/// One channel of one lane: a one-pole accumulator and a linear gain ramp.
#[derive(Clone, Copy)]
struct Channel {
    acc: f32,
    gain: f32,
    target: f32,
    step: f32,
    remaining: u32,
    pole: f32,
}

impl Channel {
    /// A channel at a random accumulator, mid-way through a random gain ramp.
    fn fresh(rng: &mut Rng) -> Self {
        let mut channel = Self {
            acc: rng.signed(),
            gain: 1.0,
            target: 1.0,
            step: 0.0,
            remaining: 0,
            pole: POLE,
        };
        channel.retarget(rng.signed() * 2.0);
        channel.remaining += rng.below(40) as u32;
        channel
    }

    /// The words the kernel reads that are not running state: the `DESIGNED` term compares them.
    fn designed(&self) -> [u32; 5] {
        [
            self.gain.to_bits(),
            self.target.to_bits(),
            self.step.to_bits(),
            self.remaining,
            self.pole.to_bits(),
        ]
    }

    fn retarget(&mut self, target: f32) {
        self.target = target;
        self.step = (target - self.gain) / GAIN_RAMP as f32;
        self.remaining = GAIN_RAMP;
    }

    fn tick(&mut self, input: f32) -> f32 {
        if self.remaining > 0 {
            self.gain += self.step;
            self.remaining -= 1;
            if self.remaining == 0 {
                self.gain = self.target;
            }
        }
        self.acc = self.acc * self.pole + input;
        self.acc * self.gain
    }

    fn write(&self, out: &mut [u8]) {
        let words = [
            self.acc.to_bits(),
            self.gain.to_bits(),
            self.target.to_bits(),
            self.step.to_bits(),
            self.remaining,
            self.pole.to_bits(),
        ];
        for (chunk, word) in out.as_chunks_mut::<4>().0.iter_mut().zip(words) {
            chunk.copy_from_slice(&word.to_le_bytes());
        }
    }

    fn read(bytes: &[u8]) -> Self {
        let word = |index: usize| {
            u32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().expect("word"))
        };
        Self {
            acc: f32::from_bits(word(0)),
            gain: f32::from_bits(word(1)),
            target: f32::from_bits(word(2)),
            step: f32::from_bits(word(3)),
            remaining: word(4),
            pole: f32::from_bits(word(5)),
        }
    }
}

/// A bank of `lanes` lanes, each `[left, right]`, that starts with a gain ramp in flight on every
/// lane, so a live-control-free slot is mid-ramp too. Each lane's prepared defaults are one
/// random word, shared with the test so it can read them where the bank sits in a chain. A
/// `collapsing` bank starts each lane with two equal channels and declares the mono-collapse trio.
struct StripBank {
    width: BankWidth,
    lanes: Vec<[Channel; 2]>,
    defaults: Arc<[AtomicU64]>,
    collapsing: bool,
}

impl StripBank {
    fn new(width: BankWidth, rng: &mut Rng, collapsing: bool) -> Self {
        let lanes = (0..width.lanes())
            .map(|_| {
                if collapsing {
                    let channel = Channel::fresh(rng);
                    [channel, channel]
                } else {
                    core::array::from_fn(|_| Channel::fresh(rng))
                }
            })
            .collect();
        let defaults = (0..width.lanes())
            .map(|_| AtomicU64::new(rng.next()))
            .collect();
        Self {
            width,
            lanes,
            defaults,
            collapsing,
        }
    }

    fn bound(self) -> PreparedEffectBank {
        let width = self.width;
        PreparedEffectBank {
            processor: Box::new(self),
            metadata: PreparedBankMetadata {
                width,
                program_key: EffectProgramKey {
                    effect_id: EffectId::parse("mock.strip").expect("id"),
                    contract_major: 1,
                    state_layout_version: 1,
                    sample_rate: 48_000,
                    quantum: QUANTUM,
                    quality: EffectQuality::Normal,
                    bypass: false,
                    link_mode: LinkMode::DualMono,
                    ports: PreparedPorts {
                        sidechain: PreparedSidechainPort::None,
                    },
                    latency: LatencySamples(LATENCY as u64),
                    tail: TailSamples::Finite(0),
                    tail_every_peak: TailSamples::Infinite,
                    rest: effect_contract::RestBound::Unstated,
                    composition: effect_contract::CompositionBound::Unstated,
                    state_sizes: SIZES,
                    scratch_bytes: 0,
                    automation_capacity: 4,
                },
            },
        }
    }
}

impl PreparedNativeEffectBank for StripBank {
    fn reset(&mut self, _kind: ResetKind) {}

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        let lanes = self.lanes.len();
        for (lane, channels) in self.lanes.iter_mut().enumerate() {
            let spans = &block.automation[block.automation_offsets[lane] as usize
                ..block.automation_offsets[lane + 1] as usize];
            for span in spans.iter().filter(|span| span.parameter_index == 0) {
                match span.channel {
                    ParameterChannel::Left => channels[0].retarget(span.value),
                    ParameterChannel::Right => channels[1].retarget(span.value),
                    ParameterChannel::Both => {
                        channels[0].retarget(span.value);
                        channels[1].retarget(span.value);
                    }
                }
            }
            for frame in 0..block.frames as usize {
                let index = frame * lanes + lane;
                block.left[index] = channels[0].tick(block.left[index]);
                block.right[index] = channels[1].tick(block.right[index]);
            }
        }
        BankProcessReport::empty(self.width)
    }

    fn supports_mono_collapse(&self) -> bool {
        self.collapsing
    }

    /// The left channel alone, as the dual body runs it; a collapsed block admits no right-only
    /// write (it clears `LIVE`), so `Right` spans never reach it.
    fn process_bank_mono(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        let lanes = self.lanes.len();
        for (lane, channels) in self.lanes.iter_mut().enumerate() {
            let spans = &block.automation[block.automation_offsets[lane] as usize
                ..block.automation_offsets[lane + 1] as usize];
            for span in spans.iter().filter(|span| span.parameter_index == 0) {
                if span.channel != ParameterChannel::Right {
                    channels[0].retarget(span.value);
                }
            }
            for frame in 0..block.frames as usize {
                let index = frame * lanes + lane;
                block.left[index] = channels[0].tick(block.left[index]);
            }
        }
        BankProcessReport::empty(self.width)
    }

    fn desymmetrize_channels(&mut self) {
        for channels in &mut self.lanes {
            channels[1] = channels[0];
        }
    }

    fn lane_channel_symmetry(&self, lane: usize) -> bool {
        self.lanes
            .get(lane)
            .is_some_and(|[left, right]| left.designed() == right.designed())
    }

    fn observe_resident_bank(&self, tap_index: u32, out: &mut [ObservationSample]) -> bool {
        for (sample, channels) in out.iter_mut().zip(&self.lanes) {
            let read = |channel: &Channel| {
                if tap_index == 0 {
                    channel.acc
                } else {
                    channel.gain
                }
            };
            *sample = ObservationSample {
                left: read(&channels[0]),
                right: read(&channels[1]),
            };
        }
        true
    }

    fn snapshot_track_state_payload(
        &self,
        track_index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let lane = self
            .lanes
            .get(track_index as usize)
            .ok_or(StatePayloadError {
                code: "effect.state.lane",
            })?;
        lane[0].write(output.left);
        lane[1].write(output.right);
        Ok(())
    }

    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        _state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        let lane = self
            .lanes
            .get_mut(track_index as usize)
            .ok_or(StatePayloadError {
                code: "effect.state.lane",
            })?;
        *lane = [Channel::read(input.left), Channel::read(input.right)];
        Ok(())
    }

    fn lane_default_bytes(&self) -> Option<usize> {
        Some(8)
    }

    /// Target slot 0's word 0 is a gain the ramp restarts toward, as a parameter span does, so
    /// a target applied twice renders other bits than one applied once.
    fn apply_prepared_target_lane(
        &mut self,
        lane: usize,
        target: &PreparedEffectTarget,
    ) -> Result<(), EffectTargetError> {
        let channels = self
            .lanes
            .get_mut(lane)
            .filter(|_| target.slot == 0)
            .ok_or(EffectTargetError::Shape)?;
        let value = f32::from_bits(target.words[0]);
        match target.channel {
            ParameterChannel::Left => channels[0].retarget(value),
            ParameterChannel::Right => channels[1].retarget(value),
            ParameterChannel::Both => {
                channels[0].retarget(value);
                channels[1].retarget(value);
            }
        }
        Ok(())
    }

    fn exchange_lane_defaults(&mut self, lane: usize, bytes: &mut [u8]) -> bool {
        let (Some(own), Ok(word)) = (self.defaults.get(lane), <[u8; 8]>::try_from(&*bytes)) else {
            return false;
        };
        let old = own.swap(u64::from_le_bytes(word), Ordering::Relaxed);
        bytes.copy_from_slice(&old.to_le_bytes());
        true
    }
}

const fn tap(id: u32) -> ObservationDescriptor {
    ObservationDescriptor {
        id: ObservationTapId(id),
        display_name: "Level",
        display_unit: "",
        kind: ObservationKind::GainReductionDb,
        unit: ParameterUnit::Db,
        cost: ObservationCost::Resident,
        cadence: ObservationCadence::PerBlock,
        fold: ObservationFold::PeakMagnitude,
        channels: ObservationChannels::PerLane,
        minimum: 0.0,
        maximum: 100.0,
    }
}
static TAPS: [ObservationDescriptor; 2] = [tap(1), tap(2)];

struct Planes {
    left: Vec<Vec<f32>>,
    right: Vec<Vec<f32>>,
}

impl BankMembers for Planes {
    fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
        (&self.left[lane], &self.right[lane])
    }
    fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
        (&mut self.left[lane], &mut self.right[lane])
    }
}

/// What differs between the chains of a [`World`].
#[derive(Clone, Copy)]
struct Shape {
    /// The last lane of chain 1 is padded (inactive in the chain and every slot).
    padded: bool,
    /// Lane 0 of chain 1 is an identity in slot 2.
    slot_gap: bool,
    /// Collapsing banks, mono inputs and both-channel writes, armed as given.
    collapse: Option<Collapse>,
}

/// A [`World`] whose banks can collapse.
#[derive(Clone, Copy)]
struct Collapse {
    /// Per chain: armed for mono sources ([`BankChain::arm_mono_collapse`]).
    armed: [bool; 2],
    /// `(slot, lane)` of chain 0 whose bank has the right pole [`SKEWED_POLE`]: a lane whose
    /// designed words differ between the channels.
    skew: Option<(usize, usize)>,
}

/// Two chains of three slots -- an `EffectBankStage`, a `LiveControlEffectBankStage` with a live
/// channel, a latency-3 dry shunt and two taps on every lane, and another `EffectBankStage` -- and
/// the control side of every strip. Strip `s` starts at chain `s / lanes`, lane `s % lanes`, and
/// `position` follows it through swaps.
struct World {
    lanes: usize,
    chains: [BankChain; 2],
    gate: RevisionGate,
    pin: GatePin,
    /// Per strip.
    writers: Vec<EffectCellWriter>,
    readers: Vec<[ObservationReader; 2]>,
    position: Vec<(usize, usize)>,
    first_sample: u64,
    /// Per chain, per slot: the bank's prepared defaults, per lane.
    defaults: [[Arc<[AtomicU64]>; 3]; 2],
    /// Mono inputs and both-channel writes ([`Shape::collapse`]).
    collapsing: bool,
    /// Render under the gate's snapshot, as production does; [`LiveSnapshot::ALL`] otherwise.
    snapshot: bool,
}

impl World {
    fn new(width: BankWidth, seed: u64, shape: Shape) -> Self {
        Self::with_target_slots(width, seed, shape, 0)
    }

    /// [`Self::new`] with `target_slots` prepared-target slots on every live lane.
    fn with_target_slots(width: BankWidth, seed: u64, shape: Shape, target_slots: usize) -> Self {
        let lanes = width.lanes() as usize;
        let mut rng = Rng(seed);
        let (gate, pin) = RevisionGate::pair(0);
        let superseded = LiveValuesSuperseded::new();
        let mut writers = Vec::new();
        let mut readers = Vec::new();
        let mut defaults = Vec::new();
        let collapsing = shape.collapse.is_some();
        let chains = core::array::from_fn(|chain| {
            let mut controls: Vec<Option<EffectControlLane>> = Vec::new();
            let mut observations = Vec::new();
            for lane in 0..lanes {
                let (writer, control) = effect_control_lane(
                    EffectLaneSpec {
                        shape: EffectLaneShape::from_live_parameters(
                            [(0, ParameterChannelPolicy::PerLane)],
                            target_slots,
                        ),
                        bypass: false,
                        observation_depth: 2,
                    },
                    &superseded,
                )
                .expect("lane");
                let mut control = control;
                // A collapsing world carries lanes in, so its lanes hold the carried slots.
                if collapsing {
                    control.install_carry_slots();
                }
                writers.push(writer);
                controls.push(Some(control));
                let (first, first_reader) = observation_slot();
                let (second, second_reader) = observation_slot();
                readers.push([first_reader, second_reader]);
                let mut observation =
                    ObservationLane::new(&TAPS, vec![first, second], 2).expect("taps");
                // Every other strip has its first tap armed at preparation.
                if (chain * lanes + lane).is_multiple_of(2) {
                    observation.arm(0, true, 2, 0);
                }
                observations.push(Some(observation));
            }
            let padded = chain == 1 && shape.padded;
            let active: Vec<bool> = (0..lanes)
                .map(|lane| !(padded && lane == lanes - 1))
                .collect();
            let mut gapped = active.clone();
            if chain == 1 && shape.slot_gap {
                gapped[0] = false;
            }
            let mut chain_defaults = Vec::new();
            let skew = shape.collapse.and_then(|collapse| collapse.skew);
            let mut bank = |slot: usize, rng: &mut Rng| {
                let mut bank = StripBank::new(width, rng, collapsing);
                if let Some((skewed, lane)) = skew
                    && chain == 0
                    && skewed == slot
                {
                    bank.lanes[lane][1].pole = SKEWED_POLE;
                }
                chain_defaults.push(Arc::clone(&bank.defaults));
                bank
            };
            let effect = |bank: StripBank| {
                Box::new(EffectBankStage::new(bank.bound(), width, QUANTUM).expect("stage"))
                    as Box<dyn BankStage>
            };
            let first = effect(bank(0, &mut rng));
            let live_bank = bank(1, &mut rng);
            let live = LiveControlEffectBankStage::new(
                live_bank.bound(),
                width,
                QUANTUM,
                controls,
                observations,
                LATENCY,
            )
            .expect("stage");
            let third = effect(bank(2, &mut rng));
            defaults.push(chain_defaults);
            let slots = vec![
                BankSlot {
                    stage: first,
                    active_lanes: active.clone().into_boxed_slice(),
                },
                BankSlot {
                    stage: Box::new(live),
                    active_lanes: active.clone().into_boxed_slice(),
                },
                BankSlot {
                    stage: third,
                    active_lanes: gapped.into_boxed_slice(),
                },
            ];
            let mut built = BankChain::new(
                AoSoaScratch::new(width, QUANTUM).expect("scratch"),
                active.into_boxed_slice(),
                slots,
            )
            .expect("chain");
            if let Some(collapse) = shape.collapse {
                built.arm_mono_collapse(collapse.armed[chain]);
            }
            built
        });
        Self {
            lanes,
            chains,
            gate,
            pin,
            writers,
            readers,
            position: (0..2 * lanes).map(|s| (s / lanes, s % lanes)).collect(),
            first_sample: 0,
            defaults: defaults
                .into_iter()
                .map(|chain| chain.try_into().expect("three slots"))
                .collect::<Vec<_>>()
                .try_into()
                .expect("two chains"),
            collapsing,
            snapshot: false,
        }
    }

    /// Every strip's prepared defaults, per slot, read where the strip now sits.
    fn strip_defaults(&self) -> Vec<[u64; 3]> {
        self.position
            .iter()
            .map(|&(chain, lane)| {
                core::array::from_fn(|slot| {
                    self.defaults[chain][slot][lane].load(Ordering::Relaxed)
                })
            })
            .collect()
    }

    fn strips(&self) -> usize {
        self.position.len()
    }

    /// The strip at `(chain, lane)`.
    fn strip_at(&self, at: (usize, usize)) -> usize {
        self.position
            .iter()
            .position(|position| *position == at)
            .expect("a strip sits at every position")
    }

    fn push(&mut self, strip: usize, record: EffectControlRecord) {
        let revision = self.pin.last_published() + 1;
        let stamp = self.pin.stamp(revision).expect("a legal revision");
        self.writers[strip]
            .write(stamp, record)
            .expect("the lane has a cell for the record");
        self.pin.publish(&self.gate, revision);
    }

    /// Render one block of `inputs[strip]` (`(left, right)`, `QUANTUM` frames) and return each
    /// strip's output bits.
    fn render(&mut self, inputs: &[(Vec<f32>, Vec<f32>)]) -> Vec<(Vec<u32>, Vec<u32>)> {
        let mut planes: Vec<Planes> = (0..2)
            .map(|_| Planes {
                left: vec![vec![0.0; QUANTUM as usize]; self.lanes],
                right: vec![vec![0.0; QUANTUM as usize]; self.lanes],
            })
            .collect();
        for (strip, (left, right)) in inputs.iter().enumerate() {
            let (chain, lane) = self.position[strip];
            planes[chain].left[lane].clone_from(left);
            planes[chain].right[lane].clone_from(right);
        }
        let snapshot = if self.snapshot {
            self.gate.snapshot()
        } else {
            LiveSnapshot::ALL
        };
        for (chain, planes) in self.chains.iter_mut().zip(planes.iter_mut()) {
            chain
                .run(planes, QUANTUM, self.first_sample, snapshot)
                .expect("run");
        }
        self.first_sample += u64::from(QUANTUM);
        (0..self.strips())
            .map(|strip| {
                let (chain, lane) = self.position[strip];
                let bits = |plane: &[f32]| plane.iter().map(|value| value.to_bits()).collect();
                (
                    bits(&planes[chain].left[lane]),
                    bits(&planes[chain].right[lane]),
                )
            })
            .collect()
    }

    /// Every strip's newest publication on each tap, as exact words.
    fn publications(&self) -> Vec<[Option<[u64; 6]>; 2]> {
        self.readers
            .iter()
            .map(|taps| {
                core::array::from_fn(|tap| {
                    taps[tap].read().map(|window| {
                        [
                            window.first_sample,
                            window.end_sample,
                            window.sequence,
                            u64::from(window.blocks),
                            u64::from(window.left.to_bits()),
                            u64::from(window.right.to_bits()),
                        ]
                    })
                })
            })
            .collect()
    }

    /// Swap `(chain, lane)` with `(other_chain, other_lane)`, and move the strips' positions.
    fn swap(
        &mut self,
        (chain, lane): (usize, usize),
        (other_chain, other_lane): (usize, usize),
        scratch: &mut [u8],
    ) -> Result<(), ChainLaneSwapRefusal> {
        let result = if chain == other_chain {
            self.chains[chain].swap_lanes(lane, ChainLanePeer::Same(other_lane), scratch)
        } else {
            let [first, second] = &mut self.chains;
            let (own, other) = if chain == 0 {
                (first, second)
            } else {
                (second, first)
            };
            own.swap_lanes(lane, ChainLanePeer::Other(other, other_lane), scratch)
        };
        if result.is_ok() {
            let (a, b) = (
                self.strip_at((chain, lane)),
                self.strip_at((other_chain, other_lane)),
            );
            self.position.swap(a, b);
        }
        result
    }

    /// [`Self::swap`], gated (#1747 gate 2): the swap allocates and frees nothing on the calling
    /// thread. The world has been built, so every process static is warm.
    fn swap_counted(
        &mut self,
        at: (usize, usize),
        other: (usize, usize),
        scratch: &mut [u8],
    ) -> Result<(), ChainLaneSwapRefusal> {
        let mark = current_thread_counters();
        let result = self.swap(at, other, scratch);
        let delta = current_thread_delta_since(mark);
        assert_eq!(
            (delta.allocations, delta.deallocations),
            (0, 0),
            "a lane swap allocated or freed"
        );
        result
    }

    /// The stage of slot `slot` of chain `chain`, as its type.
    fn stage_mut<T: 'static>(&mut self, chain: usize, slot: usize) -> &mut T {
        self.chains[chain]
            .slot_stage_mut(slot)
            .and_then(BankStage::as_any_mut)
            .and_then(|any| any.downcast_mut::<T>())
            .expect("a stage of that type")
    }

    fn scratch(&self) -> Vec<u8> {
        let bytes = self.chains[0]
            .lane_swap_scratch_bytes()
            .max(self.chains[1].lane_swap_scratch_bytes());
        vec![0_u8; bytes]
    }
}

fn gain(rng: &mut Rng) -> EffectControlRecord {
    EffectControlRecord::Parameter {
        parameter_index: 0,
        channel: if rng.below(2) == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: rng.signed() * 2.0,
    }
}

/// `record` on its own channel, and in a collapsing world its bit-equal twin on the other one
/// too: the pair the drain stages as one both-channel write.
fn writes(record: EffectControlRecord, collapsing: bool) -> Vec<EffectControlRecord> {
    let EffectControlRecord::Parameter { value, .. } = record else {
        return vec![record];
    };
    if !collapsing {
        return vec![record];
    }
    [ParameterChannel::Left, ParameterChannel::Right]
        .map(|channel| EffectControlRecord::Parameter {
            parameter_index: 0,
            channel,
            value,
        })
        .to_vec()
}

const fn bypass(bypassed: bool) -> EffectControlRecord {
    EffectControlRecord::Bypass(BypassSwitch {
        bypassed,
        ramp_samples: BYPASS_RAMP,
    })
}

/// Random planes per strip; `mono`: the right plane is the left one (a mono source).
fn inputs(rng: &mut Rng, strips: usize, mono: bool) -> Vec<(Vec<f32>, Vec<f32>)> {
    (0..strips)
        .map(|_| {
            let left: Vec<f32> = (0..QUANTUM).map(|_| rng.signed()).collect();
            let right = if mono {
                left.clone()
            } else {
                (0..QUANTUM).map(|_| rng.signed()).collect()
            };
            (left, right)
        })
        .collect()
}

/// Render one block in both worlds with the same per-strip inputs and live writes, and assert
/// every strip's output and publications agree.
fn step(swapped: &mut World, twin: &mut World, rng: &mut Rng, block: usize) {
    let strips = twin.strips();
    for strip in 0..strips {
        if rng.below(3) == 0 {
            for record in writes(gain(rng), twin.collapsing) {
                swapped.push(strip, record);
                twin.push(strip, record);
            }
        }
    }
    let inputs = inputs(rng, strips, twin.collapsing);
    let (got, expected) = (swapped.render(&inputs), twin.render(&inputs));
    for strip in 0..strips {
        assert!(
            got[strip] == expected[strip],
            "block {block}: strip {strip} output differs from its twin"
        );
    }
    assert!(
        swapped.publications() == twin.publications(),
        "block {block}: a publication differs from its twin"
    );
}

/// Gate 1 (#1747): two chains render, two random lanes of one chain swap and later a random lane
/// of each chain swaps with the other, each mid-ramp (gain ramps on every lane, a bypass crossfade
/// over a latency-3 dry shunt on a swapped strip, a tap armed and a subscription queued after the
/// swap), and every strip, fed and written as it moves, renders and publishes exactly what its
/// never-swapped twin does, and holds its twin's prepared defaults in every slot. Every swap
/// allocates and frees nothing (gate 2). Simd4 and, where built, Simd8.
///
/// Then the collapse case: both chains armed for mono sources, one odd strip in chain 0 -- a
/// designed word skewed in the live slot or in an effect slot, or an asymmetric payload carried
/// into the live slot -- that must keep its chain dual. Swapped before the first block, in the
/// chain and then across, the odd strip takes its witness term with it: chain 1 then renders it
/// dual and chain 0 collapses. And an odd strip skewed, driven apart and repaired by a carry
/// keeps chain 0's lost agreement with it when it crosses into chain 1 (the swap called from
/// chain 1). A cross-chain swap between an armed and an unarmed chain refuses (`Shape`).
///
/// Test value: a swap that leaves any per-lane part behind -- the payload, the prepared defaults,
/// the control lane, the observation queue's consumer, the taps, the shunt's line words, a
/// stage's cached `DESIGNED` or `RESTORED` term, or the chains' channel agreement -- swaps a
/// shared cursor, allocates, or moves a stereo strip into a mono-armed chain; no other test
/// performs a render-time swap.
#[test]
fn a_swapped_strip_renders_and_publishes_what_its_twin_does() {
    assert_installed();
    let shape = Shape {
        padded: false,
        slot_gap: false,
        collapse: None,
    };
    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        for seed in 0..12_u64 {
            let mut rng = Rng(seed ^ 0x1747);
            let mut swapped = World::new(width, seed, shape);
            let mut twin = World::new(width, seed, shape);
            let mut scratch = swapped.scratch();
            let first = rng.below(lanes);
            let second = (first + 1 + rng.below(lanes - 1)) % lanes;
            let across = (rng.below(lanes), rng.below(lanes));
            let mut block = 0;
            for _ in 0..3 {
                step(&mut swapped, &mut twin, &mut rng, block);
                block += 1;
            }
            // A crossfade toward bypass starts on one of the strips about to swap, and every
            // other strip's tap 1 is subscribed through its queue.
            let bypassed = swapped.strip_at((0, first));
            swapped.push(bypassed, bypass(true));
            twin.push(bypassed, bypass(true));
            step(&mut swapped, &mut twin, &mut rng, block);
            block += 1;
            swapped
                .swap_counted((0, first), (0, second), &mut scratch)
                .expect("an in-chain swap");
            for strip in (0..swapped.strips()).filter(|strip| !strip.is_multiple_of(2)) {
                swapped.writers[strip]
                    .try_push_observe(1, true, 2)
                    .expect("room");
                twin.writers[strip]
                    .try_push_observe(1, true, 2)
                    .expect("room");
            }
            for _ in 0..2 {
                step(&mut swapped, &mut twin, &mut rng, block);
                block += 1;
            }
            let crossing = swapped.strip_at((0, across.0));
            swapped.push(crossing, bypass(true));
            twin.push(crossing, bypass(true));
            step(&mut swapped, &mut twin, &mut rng, block);
            block += 1;
            swapped
                .swap_counted((0, across.0), (1, across.1), &mut scratch)
                .expect("a cross-chain swap");
            for strip in (0..swapped.strips()).filter(|strip| strip.is_multiple_of(2)) {
                swapped.writers[strip]
                    .try_push_observe(1, true, 3)
                    .expect("room");
                twin.writers[strip]
                    .try_push_observe(1, true, 3)
                    .expect("room");
            }
            for _ in 0..5 {
                step(&mut swapped, &mut twin, &mut rng, block);
                block += 1;
            }
            // The crossfaded strips settle dry and back wet, still followed.
            swapped.push(crossing, bypass(false));
            twin.push(crossing, bypass(false));
            for _ in 0..4 {
                step(&mut swapped, &mut twin, &mut rng, block);
                block += 1;
            }
            assert_eq!(
                swapped.strip_defaults(),
                twin.strip_defaults(),
                "{width:?} seed {seed}: a strip's prepared defaults stayed behind"
            );
        }
    }
    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        for seed in 0..4_u64 {
            for case in OddStrip::ALL {
                collapse_case(width, lanes, seed, case);
            }
        }
    }
}

/// Render `blocks` blocks with no write in both worlds and assert every strip's output and
/// publications agree.
fn quiet(
    skipping: &mut World,
    reading: &mut World,
    rng: &mut Rng,
    block: &mut usize,
    blocks: usize,
) {
    for _ in 0..blocks {
        let inputs = inputs(rng, reading.strips(), false);
        let (got, expected) = (skipping.render(&inputs), reading.render(&inputs));
        for strip in 0..reading.strips() {
            assert!(
                got[strip] == expected[strip],
                "block {block}: strip {strip} differs from a bank read every block"
            );
        }
        assert!(
            skipping.publications() == reading.publications(),
            "block {block}: a publication differs from a bank read every block"
        );
        *block += 1;
    }
}

/// Issue #1763: a live slot that skips its cell walk under a snapshot publication it already read
/// and found quiet renders and publishes, block for block, what the same world renders when every
/// block reads every lane (`LiveSnapshot::ALL`, which never skips; every write here is published
/// before the block that reads it, so the two read the same writes). Each step is followed by
/// blocks that bring no new publication: a prepared target, which stages no span, a bypass
/// crossfade, a lane moved mid-crossfade into a quiet chain (once as the swap's peer, once into
/// the chain the swap is called on), a plan-swap carry's unread write taken into a quiet slot, and
/// a write on a chain quiet for several blocks. The crossfade steps do not test the `ramping` term
/// of the quiet mark: a walk's verdicts kept over a running crossfade render the same bits, and
/// the term only refreshes them once the crossfade settles, which saves cost.
///
/// Test value: a quiet mark that ignores a staged target (the target is applied again on every
/// quiet block and restarts the gain ramp), that survives a lane swap on either side or a carry,
/// or that lets `LiveSnapshot::ALL` skip; no other rack test renders a live slot under an
/// unchanged publication across these steps.
#[test]
fn a_quiet_live_slot_renders_what_one_read_every_block_does() {
    let shape = Shape {
        padded: false,
        slot_gap: false,
        collapse: None,
    };
    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        let mut skipping = World::with_target_slots(width, 0x1763, shape, 1);
        skipping.snapshot = true;
        let mut reading = World::with_target_slots(width, 0x1763, shape, 1);
        let mut scratch = skipping.scratch();
        let mut rng = Rng(0x1763);
        let mut block = 0;
        let carried = lanes - 1;
        for world in [&mut skipping, &mut reading] {
            world
                .stage_mut::<LiveControlEffectBankStage>(0, 1)
                .install_carry_slots(carried);
        }
        let record = gain(&mut rng);
        for world in [&mut skipping, &mut reading] {
            world.push(0, record);
        }
        quiet(&mut skipping, &mut reading, &mut rng, &mut block, 3);
        // A prepared target on chain 0's first lane: it stages a target and no span, and its gain
        // ramp (eleven frames) outlasts a block.
        let mut words = [0_u32; PREPARED_EFFECT_TARGET_WORDS];
        words[0] = 0.625_f32.to_bits();
        let target = EffectControlRecord::PreparedTarget(PreparedEffectTarget {
            slot: 0,
            channel: ParameterChannel::Both,
            words,
        });
        let targeted = skipping.strip_at((0, 0));
        for world in [&mut skipping, &mut reading] {
            world.push(targeted, target);
        }
        quiet(&mut skipping, &mut reading, &mut rng, &mut block, 3);
        // A crossfade toward bypass starts on chain 1's last lane; chain 0 reads the publication
        // once and is quiet under it.
        let fading = skipping.strip_at((1, lanes - 1));
        for world in [&mut skipping, &mut reading] {
            world.push(fading, bypass(true));
        }
        quiet(&mut skipping, &mut reading, &mut rng, &mut block, 1);
        // Mid-crossfade, into the quiet chain 0 as the swap's peer.
        for world in [&mut skipping, &mut reading] {
            world
                .swap((1, lanes - 1), (0, 0), &mut scratch)
                .expect("a cross-chain swap");
        }
        quiet(&mut skipping, &mut reading, &mut rng, &mut block, 4);
        // Back toward wet, then into the quiet chain 1 as the chain the swap is called on.
        for world in [&mut skipping, &mut reading] {
            world.push(fading, bypass(false));
        }
        quiet(&mut skipping, &mut reading, &mut rng, &mut block, 1);
        for world in [&mut skipping, &mut reading] {
            world
                .swap((1, 0), (0, 0), &mut scratch)
                .expect("a cross-chain swap");
        }
        quiet(&mut skipping, &mut reading, &mut rng, &mut block, 4);
        // A predecessor's unread write carried into chain 0's quiet live slot.
        for world in [&mut skipping, &mut reading] {
            let superseded = LiveValuesSuperseded::new();
            let (gate, mut pin) = RevisionGate::pair(0);
            let controls = (0..lanes)
                .map(|lane| {
                    let (mut writer, control) = effect_control_lane(
                        EffectLaneSpec {
                            shape: EffectLaneShape::from_live_parameters(
                                [(0, ParameterChannelPolicy::PerLane)],
                                1,
                            ),
                            bypass: false,
                            observation_depth: 2,
                        },
                        &superseded,
                    )
                    .expect("lane");
                    if lane == carried {
                        let stamp = pin.stamp(1).expect("a legal revision");
                        writer.write(stamp, record).expect("a gain cell");
                        pin.publish(&gate, 1);
                    }
                    Some(control)
                })
                .collect();
            let source = LiveControlEffectBankStage::new(
                StripBank::new(width, &mut Rng(0x50_75), false).bound(),
                width,
                QUANTUM,
                controls,
                (0..lanes).map(|_| None).collect(),
                LATENCY,
            )
            .expect("stage");
            let stage = world.stage_mut::<LiveControlEffectBankStage>(0, 1);
            assert!(stage.carry_lane_from(carried, &source, carried, &mut scratch));
        }
        quiet(&mut skipping, &mut reading, &mut rng, &mut block, 3);
        // A write on chain 1 after its quiet stretch.
        let written = skipping.strip_at((1, 1 % lanes));
        let record = gain(&mut rng);
        for world in [&mut skipping, &mut reading] {
            world.push(written, record);
        }
        quiet(&mut skipping, &mut reading, &mut rng, &mut block, 2);
    }
}

/// What makes chain 0's odd strip keep its chain dual, in [`collapse_case`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OddStrip {
    /// A designed word skewed in the live slot (`LiveControlEffectBankStage::designed`).
    LiveDesigned,
    /// A designed word skewed in an effect slot (`EffectBankStage`'s cached witness).
    StageDesigned,
    /// An asymmetric payload carried into the live slot (its `unrestored` bit).
    Unrestored,
    /// Skewed in an effect slot, rendered apart, then repaired by a symmetric carry: its witness
    /// holds again but chain 0's agreement stays lost.
    Apart,
    /// No odd strip; chain 1 is not armed.
    Unarmed,
}

impl OddStrip {
    const ALL: [Self; 5] = [
        Self::LiveDesigned,
        Self::StageDesigned,
        Self::Unrestored,
        Self::Apart,
        Self::Unarmed,
    ];
}

/// A collapsing [`StripBank`] (`apart`: every lane's right accumulator off its left).
fn source_bank(width: BankWidth, seed: u64, apart: bool) -> StripBank {
    let mut bank = StripBank::new(width, &mut Rng(seed ^ 0x50_75), true);
    if apart {
        for channels in &mut bank.lanes {
            channels[1].acc = channels[0].acc + 0.5;
        }
    }
    bank
}

/// One collapse case of gate 1; see its documentation.
fn collapse_case(width: BankWidth, lanes: usize, seed: u64, case: OddStrip) {
    let mut rng = Rng(seed ^ 0x0c01);
    let odd = rng.below(lanes);
    let moved = (odd + 1 + rng.below(lanes - 1)) % lanes;
    let across = rng.below(lanes);
    let shape = Shape {
        padded: false,
        slot_gap: false,
        collapse: Some(Collapse {
            armed: [true, case != OddStrip::Unarmed],
            skew: match case {
                OddStrip::LiveDesigned => Some((1, odd)),
                OddStrip::StageDesigned | OddStrip::Apart => Some((0, odd)),
                OddStrip::Unrestored | OddStrip::Unarmed => None,
            },
        }),
    };
    let what = format!("{width:?} seed {seed} {case:?}");
    let mut swapped = World::new(width, seed, shape);
    let mut twin = World::new(width, seed, shape);
    let mut scratch = swapped.scratch();
    let mut block = 0;
    match case {
        OddStrip::Unarmed => assert_eq!(
            swapped.swap_counted((0, odd), (1, across), &mut scratch),
            Err(ChainLaneSwapRefusal::Shape),
            "{what}: a swap between an armed and an unarmed chain"
        ),
        OddStrip::Apart => {
            for _ in 0..3 {
                step(&mut swapped, &mut twin, &mut rng, block);
                block += 1;
            }
            let source =
                EffectBankStage::new(source_bank(width, seed, false).bound(), width, QUANTUM)
                    .expect("stage");
            for world in [&mut swapped, &mut twin] {
                let stage = world.stage_mut::<EffectBankStage>(0, 0);
                assert!(stage.carry_lane_from(odd, &source, odd, &mut scratch));
            }
            step(&mut swapped, &mut twin, &mut rng, block);
            block += 1;
            // Both directions, by seed parity, so each chain's half of the disengage and of the
            // agreement write is the receiving side on some seed.
            let (from, to) = if seed.is_multiple_of(2) {
                ((1, across), (0, odd))
            } else {
                ((0, odd), (1, across))
            };
            swapped
                .swap_counted(from, to, &mut scratch)
                .expect("a cross-chain swap");
        }
        _ => {
            if case == OddStrip::Unrestored {
                let superseded = LiveValuesSuperseded::new();
                let controls = (0..lanes)
                    .map(|_| {
                        let (_writer, control) = effect_control_lane(
                            EffectLaneSpec {
                                shape: EffectLaneShape::from_live_parameters(
                                    [(0, ParameterChannelPolicy::PerLane)],
                                    0,
                                ),
                                bypass: false,
                                observation_depth: 2,
                            },
                            &superseded,
                        )
                        .expect("lane");
                        Some(control)
                    })
                    .collect();
                let source = LiveControlEffectBankStage::new(
                    source_bank(width, seed, true).bound(),
                    width,
                    QUANTUM,
                    controls,
                    (0..lanes).map(|_| None).collect(),
                    LATENCY,
                )
                .expect("stage");
                for world in [&mut swapped, &mut twin] {
                    let stage = world.stage_mut::<LiveControlEffectBankStage>(0, 1);
                    assert!(stage.carry_lane_from(odd, &source, odd, &mut scratch));
                }
            }
            swapped
                .swap_counted((0, odd), (0, moved), &mut scratch)
                .expect("an in-chain swap");
            swapped
                .swap_counted((0, moved), (1, across), &mut scratch)
                .expect("a cross-chain swap");
        }
    }
    for _ in 0..6 {
        step(&mut swapped, &mut twin, &mut rng, block);
        block += 1;
    }
    // The case is live: the twin's armed chain without the odd strip rendered collapsed.
    let clean = if case == OddStrip::Unarmed { 0 } else { 1 };
    assert!(
        twin.chains[clean].collapses() > 0,
        "{what}: the twin's clean chain never collapsed"
    );
    assert!(
        twin.chains[0].collapses() == 0 || case == OddStrip::Unarmed,
        "{what}: the twin's odd chain collapsed"
    );
}

/// Every strip's output over a few blocks, and every publication, at the strips' current
/// positions: what a refused swap must leave equal to a twin.
fn assert_unchanged(world: &mut World, twin: &mut World, what: &str) {
    let mut rng = Rng(7);
    for block in 0..4 {
        step(world, twin, &mut rng, block);
    }
    assert_eq!(world.position, twin.position, "{what}: no strip moved");
}

/// #1747: a swap that names a padded lane, two lanes whose slot activity differs, or a scratch one
/// byte short refuses before it writes, and both chains render on exactly as a twin that was
/// never asked.
///
/// Test value: a refusal decided after the first slot already swapped, a padded lane that swaps,
/// or a scratch check that passes one byte short.
#[test]
fn a_refused_swap_leaves_both_chains_as_they_were() {
    assert_installed();
    let width = BankWidth::Four;
    let lanes = width.lanes() as usize;
    let cases = [
        (
            Shape {
                padded: true,
                slot_gap: false,
                collapse: None,
            },
            (0, 1),
            (1, lanes - 1),
            0,
            ChainLaneSwapRefusal::Lane,
        ),
        (
            Shape {
                padded: false,
                slot_gap: true,
                collapse: None,
            },
            (0, 2),
            (1, 0),
            0,
            ChainLaneSwapRefusal::Shape,
        ),
        (
            Shape {
                padded: false,
                slot_gap: false,
                collapse: None,
            },
            (0, 1),
            (1, 2),
            1,
            ChainLaneSwapRefusal::Slot,
        ),
        (
            Shape {
                padded: false,
                slot_gap: false,
                collapse: None,
            },
            (0, 1),
            (0, 2),
            1,
            ChainLaneSwapRefusal::Slot,
        ),
    ];
    for (index, (shape, at, other, short, refusal)) in cases.into_iter().enumerate() {
        let mut world = World::new(width, 3, shape);
        let mut twin = World::new(width, 3, shape);
        let mut rng = Rng(11);
        for block in 0..2 {
            step(&mut world, &mut twin, &mut rng, block);
        }
        let mut scratch = world.scratch();
        scratch.truncate(scratch.len() - short);
        assert_eq!(
            world.swap(at, other, &mut scratch),
            Err(refusal),
            "case {index}"
        );
        assert_unchanged(&mut world, &mut twin, &format!("case {index}"));
    }
}
