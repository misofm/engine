//! Issue #1781: the live-control effect bank stage's rest mask and its clock advance.
//!
//! `LiveControlEffectBankStage::lanes_at_rest` is the effect's own mask minus every lane with work
//! of its own in the block: a staged span, a staged prepared target, a lane that reads the dry
//! shunt (bypassed or crossfading) and an armed observation tap. Its `advance_clocks` forwards the
//! effect's clocks and feeds the dry shunt's latency line the skipped `+0.0` block.
//!
//! The effect is a stand-in with an honest predicate: per lane and channel a one-pole state word
//! (`acc`, at rest only at `+0.0` bits) times a gain the live parameter or a prepared target sets,
//! and a per-lane clock the block's length advances. The gate is a differential: a stage that
//! skips every block the mask allows (every lane at rest, every input word `+0.0`) through
//! `advance_clocks` must render, publish and hold exactly what a twin that renders every block
//! does, and every lane the mask names on a silent block must render `+0.0`.

use std::sync::{Arc, Mutex};

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
    GatePin, LiveValuesSuperseded, ObservationReader, RevisionGate, observation_slot,
};
use rack::{BankBlock, BankStage, LiveControlEffectBankStage};

const LANES: usize = 4;
const ALL: u32 = (1 << LANES) - 1;
const QUANTUM: u32 = 16;
const LATENCY: usize = 3;
const WORDS: usize = QUANTUM as usize * LANES;
/// Each lane's pole: two lanes with no memory beyond the last frame, two with a decaying tail.
const POLES: [f32; LANES] = [0.0, 0.0, 0.5, 0.25];

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

    fn signed(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1_u64 << 23) as f32 - 1.0
    }
}

/// The stand-in effect's state, shared with the test so it can compare two banks' words.
#[derive(Clone, Copy, Debug, PartialEq)]
struct MockState {
    /// `[lane][channel]`.
    acc: [[f32; 2]; LANES],
    gain: [f32; LANES],
    /// Frames rendered or skipped, per lane: the clock.
    clock: [u64; LANES],
}

struct MockBank(Arc<Mutex<MockState>>);

impl MockBank {
    fn bound(self) -> PreparedEffectBank {
        PreparedEffectBank {
            processor: Box::new(self),
            metadata: PreparedBankMetadata {
                width: BankWidth::Four,
                program_key: EffectProgramKey {
                    effect_id: EffectId::parse("mock.rest").expect("id"),
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
                    state_sizes: StatePayloadSizes {
                        common_bytes: 0,
                        left_bytes: 0,
                        right_bytes: 0,
                    },
                    scratch_bytes: 0,
                    automation_capacity: 4,
                },
            },
        }
    }
}

impl PreparedNativeEffectBank for MockBank {
    fn retained_bytes(&self) -> u64 {
        engine::realtime::retained::arc(&self.0)
    }
    fn reset(&mut self, _kind: ResetKind) {}
    fn reset_lane(&mut self, _lane: usize, _kind: ResetKind) {}

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        let mut state = self.0.lock().expect("state");
        for (lane, pole) in POLES.iter().copied().enumerate() {
            let spans = &block.automation[block.automation_offsets[lane] as usize
                ..block.automation_offsets[lane + 1] as usize];
            for span in spans.iter().filter(|span| span.parameter_index == 0) {
                state.gain[lane] = span.value;
            }
            for frame in 0..block.frames as usize {
                let index = frame * LANES + lane;
                for (channel, plane) in [&mut *block.left, &mut *block.right]
                    .into_iter()
                    .enumerate()
                {
                    let acc = state.acc[lane][channel] * pole + plane[index];
                    state.acc[lane][channel] = acc;
                    plane[index] = acc * state.gain[lane];
                }
            }
            state.clock[lane] += u64::from(block.frames);
        }
        BankProcessReport::empty(BankWidth::Four)
    }

    /// `acc` at `+0.0` bits on both channels: a `+0.0` frame then leaves it there and writes
    /// `+0.0 * gain`, which is `+0.0` for the positive gains the test sets.
    fn lanes_at_rest(&self) -> u32 {
        let state = self.0.lock().expect("state");
        (0..LANES).fold(0, |mask, lane| {
            let rest = state.acc[lane].iter().all(|acc| acc.to_bits() == 0);
            mask | (u32::from(rest) << lane)
        })
    }

    fn advance_clocks(&mut self, lanes_mask: u32, frames: u32) {
        let mut state = self.0.lock().expect("state");
        for lane in (0..LANES).filter(|lane| lanes_mask & (1 << lane) != 0) {
            state.clock[lane] += u64::from(frames);
        }
    }

    fn apply_prepared_target_lane(
        &mut self,
        lane: usize,
        target: &PreparedEffectTarget,
    ) -> Result<(), EffectTargetError> {
        let mut state = self.0.lock().expect("state");
        let gain = state.gain.get_mut(lane).ok_or(EffectTargetError::Shape)?;
        *gain = f32::from_bits(target.words[0]);
        Ok(())
    }

    fn observe_resident_bank(&self, _tap_index: u32, out: &mut [ObservationSample]) -> bool {
        let state = self.0.lock().expect("state");
        for (sample, acc) in out.iter_mut().zip(&state.acc) {
            *sample = ObservationSample {
                left: acc[0],
                right: acc[1],
            };
        }
        true
    }

    fn snapshot_track_state_payload(
        &self,
        _track_index: u32,
        _output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        Ok(())
    }

    fn restore_track_state_payload(
        &mut self,
        _track_index: u32,
        _state_layout_version: u32,
        _input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        Ok(())
    }
}

const fn tap() -> ObservationDescriptor {
    ObservationDescriptor {
        id: ObservationTapId(1),
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
static TAPS: [ObservationDescriptor; 1] = [tap()];

/// One stage: lanes 0 to 2 carry a live channel (parameter 0 per lane, one target slot, an
/// observation queue and a tap), lane 3 carries none. Latency 3, so the stage holds a shunt line.
struct World {
    stage: LiveControlEffectBankStage,
    state: Arc<Mutex<MockState>>,
    gate: RevisionGate,
    pin: GatePin,
    writers: Vec<EffectCellWriter>,
    readers: Vec<ObservationReader>,
    first_sample: u64,
}

impl World {
    fn new() -> Self {
        let (gate, pin) = RevisionGate::pair(0);
        let superseded = LiveValuesSuperseded::new();
        let state = Arc::new(Mutex::new(MockState {
            acc: [[0.0; 2]; LANES],
            gain: [1.0; LANES],
            clock: [0; LANES],
        }));
        let mut writers = Vec::new();
        let mut readers = Vec::new();
        let mut controls: Vec<Option<EffectControlLane>> = Vec::new();
        let mut observations = Vec::new();
        for _ in 0..LANES - 1 {
            let (writer, control) = effect_control_lane(
                EffectLaneSpec {
                    shape: EffectLaneShape::from_live_parameters(
                        [(0, ParameterChannelPolicy::PerLane)],
                        1,
                    ),
                    bypass: false,
                    observation_depth: 2,
                    link: None,
                },
                &superseded,
            )
            .expect("lane");
            writers.push(writer);
            controls.push(Some(control));
            let (slot, reader) = observation_slot();
            readers.push(reader);
            observations.push(Some(
                ObservationLane::new(&TAPS, vec![slot], 1).expect("taps"),
            ));
        }
        controls.push(None);
        observations.push(None);
        let stage = LiveControlEffectBankStage::new(
            MockBank(Arc::clone(&state)).bound(),
            BankWidth::Four,
            QUANTUM,
            controls,
            observations,
            LATENCY,
        )
        .expect("stage");
        Self {
            stage,
            state,
            gate,
            pin,
            writers,
            readers,
            first_sample: 0,
        }
    }

    fn push(&mut self, lane: usize, record: EffectControlRecord) {
        if let EffectControlRecord::Observe {
            tap_index,
            armed,
            window_blocks,
            ..
        } = record
        {
            self.writers[lane]
                .try_push_observe(tap_index, armed, window_blocks, 0)
                .expect("room");
            return;
        }
        let revision = self.pin.last_published() + 1;
        let stamp = self.pin.stamp(revision).expect("a legal revision");
        self.writers[lane]
            .write(stamp, record)
            .expect("the lane has a cell for the record");
        self.pin.publish(&self.gate, revision);
    }

    /// This block's drain, under the gate's snapshot as production renders, and the mask after it.
    fn begin(&mut self) -> u32 {
        self.stage
            .begin_block(self.first_sample, self.gate.snapshot())
            .expect("drain");
        self.stage.lanes_at_rest()
    }

    /// Render the block in place, or skip it through `advance_clocks` (the planes are `+0.0`).
    fn finish(&mut self, left: &mut [f32], right: &mut [f32], skip: bool) {
        let block = BankBlock {
            left,
            right,
            frames: QUANTUM,
            first_sample: self.first_sample,
            lanes: LANES,
        };
        if skip {
            self.stage.advance_clocks(ALL, block);
        } else {
            self.stage.process(block).expect("process");
        }
        self.first_sample += u64::from(QUANTUM);
    }

    fn publications(&self) -> Vec<Option<[u64; 6]>> {
        self.readers
            .iter()
            .map(|reader| {
                reader.read().map(|window| {
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
            .collect()
    }
}

/// A twin that renders every block and a stage that skips what the mask allows.
struct Pair {
    rendered: World,
    skipping: World,
    skips: usize,
    /// Silent blocks the effect alone would have allowed and the stage refused.
    refused: usize,
    block: usize,
}

impl Pair {
    fn new() -> Self {
        Self {
            rendered: World::new(),
            skipping: World::new(),
            skips: 0,
            refused: 0,
            block: 0,
        }
    }

    fn push(&mut self, lane: usize, record: EffectControlRecord) {
        self.rendered.push(lane, record);
        self.skipping.push(lane, record);
    }

    /// One block of `input` (`(left, right)` interleaved, `WORDS` each) in both worlds. Returns
    /// the mask after the drain.
    fn step(&mut self, input: &(Vec<f32>, Vec<f32>)) -> u32 {
        let block = self.block;
        self.block += 1;
        let effect_rest =
            PreparedNativeEffectBank::lanes_at_rest(&MockBank(Arc::clone(&self.rendered.state)));
        let mask = self.rendered.begin();
        assert_eq!(
            mask,
            self.skipping.begin(),
            "block {block}: the twins' masks"
        );
        assert_eq!(
            mask & !effect_rest,
            0,
            "block {block}: a lane the effect refused"
        );
        let silent = |lane: usize| {
            (0..QUANTUM as usize).all(|frame| {
                let index = frame * LANES + lane;
                input.0[index].to_bits() == 0 && input.1[index].to_bits() == 0
            })
        };
        let all_silent = (0..LANES).all(silent);
        let skip = mask == ALL && all_silent;
        if all_silent && effect_rest == ALL && mask != ALL {
            self.refused += 1;
        }
        let (mut left, mut right) = input.clone();
        self.rendered.finish(&mut left, &mut right, false);
        for lane in (0..LANES).filter(|&lane| mask & (1 << lane) != 0 && silent(lane)) {
            for frame in 0..QUANTUM as usize {
                let index = frame * LANES + lane;
                assert!(
                    left[index].to_bits() == 0 && right[index].to_bits() == 0,
                    "block {block}, lane {lane}: claimed at rest, but a silent block wrote \
                     ({}, {}) at frame {frame}",
                    left[index],
                    right[index],
                );
            }
        }
        let (mut other_left, mut other_right) = input.clone();
        self.skipping
            .finish(&mut other_left, &mut other_right, skip);
        if skip {
            self.skips += 1;
        }
        let bits = |plane: &[f32]| plane.iter().map(|word| word.to_bits()).collect::<Vec<_>>();
        assert!(
            bits(&left) == bits(&other_left) && bits(&right) == bits(&other_right),
            "block {block}: the skipping stage's output differs from its twin's"
        );
        assert_eq!(
            self.rendered.publications(),
            self.skipping.publications(),
            "block {block}: a publication differs"
        );
        assert_eq!(
            *self.rendered.state.lock().expect("state"),
            *self.skipping.state.lock().expect("state"),
            "block {block}: the effect's words, clocks included"
        );
        mask
    }
}

fn silence() -> (Vec<f32>, Vec<f32>) {
    (vec![0.0; WORDS], vec![0.0; WORDS])
}

/// Signal on the lanes with no memory beyond the last frame (pole `0`), but `+0.0` on the block's
/// last frame, so the effect is at rest after it while the shunt's latency line still holds
/// signal.
fn signal_ending_silent(rng: &mut Rng) -> (Vec<f32>, Vec<f32>) {
    let mut planes = (vec![0.0; WORDS], vec![0.0; WORDS]);
    for frame in 0..QUANTUM as usize - 1 {
        for lane in (0..LANES).filter(|&lane| POLES[lane] == 0.0) {
            planes.0[frame * LANES + lane] = rng.signed();
            planes.1[frame * LANES + lane] = rng.signed();
        }
    }
    planes
}

fn gain(lane: usize, value: f32) -> EffectControlRecord {
    EffectControlRecord::Parameter {
        parameter_index: 0,
        channel: if lane.is_multiple_of(2) {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value,
    }
}

fn target(value: f32) -> EffectControlRecord {
    let mut words = [0_u32; PREPARED_EFFECT_TARGET_WORDS];
    words[0] = value.to_bits();
    EffectControlRecord::PreparedTarget(PreparedEffectTarget {
        slot: 0,
        channel: ParameterChannel::Both,
        words,
    })
}

const fn bypass(bypassed: bool, ramp_samples: u32) -> EffectControlRecord {
    EffectControlRecord::Bypass(BypassSwitch {
        bypassed,
        ramp_samples,
    })
}

const fn observe(armed: bool) -> EffectControlRecord {
    EffectControlRecord::Observe {
        tap_index: 0,
        armed,
        window_blocks: 1,
        arm_tag: 0,
    }
}

/// Gates 1 and 2 for the live-control effect bank stage (issue #1781).
///
/// Scripted, each term on its own: a staged span, a staged prepared target, a bypass (crossfading
/// and settled) and an armed tap each clear exactly their lane's bit for exactly the blocks they
/// hold; a block of signal whose last frame is silent leaves the effect at rest with signal in
/// the latency line, the next silent block is skipped, and a step bypass after it renders the
/// line's words exactly as the twin that rendered the skipped block. Then a random schedule of
/// signal, silence, spans, targets, bypass switches and tap subscriptions on every lane. Every
/// block, the skipping stage renders, publishes and holds (the effect's words and clocks) exactly
/// what its twin does, and every lane the mask names on a silent block renders `+0.0`.
#[test]
fn the_live_stage_rests_only_lanes_with_no_work_and_skips_exactly() {
    let mut pair = Pair::new();
    let mut rng = Rng(0x1781);
    let quiet = silence();
    assert_eq!(pair.step(&quiet), ALL, "a fresh stage rests");
    assert_eq!(
        pair.step(&quiet),
        ALL,
        "under the same snapshot (the quiet skip)"
    );
    // A staged span: its lane only, for its block only.
    pair.push(1, gain(1, 0.5));
    assert_eq!(pair.step(&quiet), ALL & !(1 << 1), "a staged span");
    assert_eq!(pair.step(&quiet), ALL, "the span was applied");
    // A staged prepared target.
    pair.push(2, target(0.75));
    assert_eq!(pair.step(&quiet), ALL & !(1 << 2), "a staged target");
    assert_eq!(pair.step(&quiet), ALL, "the target was applied");
    // An armed tap, for as long as it is armed.
    pair.push(0, observe(true));
    for _ in 0..3 {
        assert_eq!(pair.step(&quiet), ALL & !1, "an armed tap");
    }
    pair.push(0, observe(false));
    assert_eq!(pair.step(&quiet), ALL, "the tap was disarmed");
    // A bypass crossfade over 40 frames, then the settled bypass, then back to wet.
    pair.push(1, bypass(true, 40));
    for _ in 0..5 {
        assert_eq!(pair.step(&quiet), ALL & !(1 << 1), "a bypassed lane");
    }
    pair.push(1, bypass(false, 20));
    assert_eq!(
        pair.step(&quiet),
        ALL & !(1 << 1),
        "a crossfade back to wet"
    );
    assert_eq!(
        pair.step(&quiet),
        ALL & !(1 << 1),
        "a crossfade back to wet"
    );
    assert_eq!(pair.step(&quiet), ALL, "the crossfade ended");
    // The line: signal whose last frame is silent, a skipped silent block, then a step bypass
    // that reads the line's words.
    for lane in (0..LANES - 1).filter(|&lane| POLES[lane] == 0.0) {
        let skips = pair.skips;
        let _ = pair.step(&signal_ending_silent(&mut rng));
        assert_eq!(pair.step(&quiet), ALL, "the effect rests after the signal");
        assert_eq!(pair.skips, skips + 1, "the silent block was skipped");
        pair.push(lane, bypass(true, 0));
        let _ = pair.step(&quiet);
        pair.push(lane, bypass(false, 0));
        let _ = pair.step(&quiet);
    }
    // The random schedule.
    let mut active = [0_usize; LANES];
    for _ in 0..4_000 {
        for lane in 0..LANES - 1 {
            match rng.below(40) {
                0 => pair.push(lane, gain(lane, 0.25 + rng.below(4) as f32 * 0.5)),
                1 => pair.push(lane, target(0.25 + rng.below(4) as f32 * 0.5)),
                2 => pair.push(lane, bypass(rng.below(4) == 0, rng.below(3) as u32 * 24)),
                3 => pair.push(lane, observe(rng.below(4) == 0)),
                _ => {}
            }
        }
        let mut input = silence();
        for (lane, run) in active.iter_mut().enumerate() {
            if *run == 0 && rng.below(60) == 0 {
                *run = 1 + rng.below(3);
            }
            if *run == 0 {
                continue;
            }
            *run -= 1;
            for frame in 0..QUANTUM as usize {
                let index = frame * LANES + lane;
                input.0[index] = rng.signed();
                input.1[index] = rng.signed();
            }
        }
        let _ = pair.step(&input);
    }
    eprintln!("skips {}, refusals {}", pair.skips, pair.refused);
    assert!(pair.skips > 100, "too few skipped blocks ({})", pair.skips);
    assert!(pair.refused > 100, "too few refusals ({})", pair.refused);
}

/// Issue #1107 (the #1781 verifier's F1): the plain effect bank stage forwards the effect's rest
/// mask and its clock advance. A stage that skips every silent block its mask allows through
/// `advance_clocks` must render and hold (the effect's words and clocks) exactly what a twin that
/// renders every block does, and must skip some.
///
/// Test value: red if `EffectBankStage` stops forwarding `lanes_at_rest` (it would never skip)
/// or `advance_clocks` (a skipped block would leave the effect's clocks behind its twin's).
#[test]
fn the_effect_stage_forwards_rest_and_clocks() {
    let state = || {
        Arc::new(Mutex::new(MockState {
            acc: [[0.0; 2]; LANES],
            gain: [1.0; LANES],
            clock: [0; LANES],
        }))
    };
    let (rendered_state, skipping_state) = (state(), state());
    let stage = |state: &Arc<Mutex<MockState>>| {
        rack::EffectBankStage::new(
            MockBank(Arc::clone(state)).bound(),
            BankWidth::Four,
            QUANTUM,
        )
        .expect("stage")
    };
    let (mut rendered, mut skipping) = (stage(&rendered_state), stage(&skipping_state));
    let mut rng = Rng(0x1107);
    let mut skips = 0;
    for block in 0..40_u64 {
        let input = if rng.below(3) == 0 {
            let mut planes = silence();
            for word in planes.0.iter_mut().chain(planes.1.iter_mut()) {
                *word = rng.signed();
            }
            planes
        } else {
            silence()
        };
        let silent = input
            .0
            .iter()
            .chain(&input.1)
            .all(|word| word.to_bits() == 0);
        let first_sample = block * u64::from(QUANTUM);
        let (mut left, mut right) = input.clone();
        rendered
            .process(BankBlock {
                left: &mut left,
                right: &mut right,
                frames: QUANTUM,
                first_sample,
                lanes: LANES,
            })
            .expect("process");
        let (mut other_left, mut other_right) = input;
        let block_view = BankBlock {
            left: &mut other_left,
            right: &mut other_right,
            frames: QUANTUM,
            first_sample,
            lanes: LANES,
        };
        if silent && skipping.lanes_at_rest() == ALL {
            skips += 1;
            skipping.advance_clocks(ALL, block_view);
        } else {
            skipping.process(block_view).expect("process");
        }
        let bits = |plane: &[f32]| plane.iter().map(|word| word.to_bits()).collect::<Vec<_>>();
        assert!(
            bits(&left) == bits(&other_left) && bits(&right) == bits(&other_right),
            "block {block}: output"
        );
        assert_eq!(
            *rendered_state.lock().expect("state"),
            *skipping_state.lock().expect("state"),
            "block {block}: the effect's words, clocks included"
        );
    }
    assert!(skips > 0, "the stage skipped");
}
