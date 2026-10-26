//! Issue #2129 S1, gates 3 and 4: a live-control bank whose every lane is settled bypassed skips its
//! wet path, and the first block that runs it again resets it once, keeping parameters.
//!
//! The bank is a recording stand-in: parameter 0 (or a prepared target) is a per-lane gain, its
//! block multiplies by it, a `FullToDefaults` reset returns every gain to `1`, a
//! `DiscontinuityKeepParameters` reset keeps them, and every call is logged in order. Its two taps
//! read `7.0` from the running state: tap 0 declares the bypass value `0.0`, tap 1 declares none.
//! What is gated is the rack: which blocks reach the bank, what a skipped block emits and folds,
//! and when the bank is reset.

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
use rack::{AoSoaScratch, BankChain, BankMembers, BankSlot, BankStage, LiveControlEffectBankStage};

const LANES: usize = 4;
const QUANTUM: u32 = 8;
const BLOCKS: u64 = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Call {
    Reset(ResetKind),
    Process,
    Target,
}

#[derive(Default)]
struct Log {
    calls: Vec<Call>,
    gain: [f32; LANES],
}

struct Recorder(Arc<Mutex<Log>>);

impl PreparedNativeEffectBank for Recorder {
    fn retained_bytes(&self) -> u64 {
        engine::realtime::retained::arc(&self.0)
    }

    fn reset(&mut self, kind: ResetKind) {
        let mut log = self.0.lock().expect("log");
        log.calls.push(Call::Reset(kind));
        if kind == ResetKind::FullToDefaults {
            log.gain = [1.0; LANES];
        }
    }

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        let mut log = self.0.lock().expect("log");
        log.calls.push(Call::Process);
        for lane in 0..LANES {
            let spans = &block.automation[block.automation_offsets[lane] as usize
                ..block.automation_offsets[lane + 1] as usize];
            for span in spans.iter().filter(|span| span.parameter_index == 0) {
                log.gain[lane] = span.value;
            }
            for frame in 0..block.frames as usize {
                let index = frame * LANES + lane;
                block.left[index] *= log.gain[lane];
                block.right[index] *= log.gain[lane];
            }
        }
        BankProcessReport::empty(BankWidth::Four)
    }

    fn apply_prepared_target_lane(
        &mut self,
        lane: usize,
        target: &PreparedEffectTarget,
    ) -> Result<(), EffectTargetError> {
        let mut log = self.0.lock().expect("log");
        log.calls.push(Call::Target);
        *log.gain.get_mut(lane).ok_or(EffectTargetError::Shape)? = f32::from_bits(target.words[0]);
        Ok(())
    }

    fn observe_resident_bank(&self, _tap_index: u32, out: &mut [ObservationSample]) -> bool {
        for sample in out.iter_mut() {
            *sample = ObservationSample {
                left: 7.0,
                right: 7.0,
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

fn bound(log: &Arc<Mutex<Log>>) -> PreparedEffectBank {
    PreparedEffectBank {
        processor: Box::new(Recorder(Arc::clone(log))),
        metadata: PreparedBankMetadata {
            width: BankWidth::Four,
            program_key: EffectProgramKey {
                effect_id: EffectId::parse("mock.recorder").expect("id"),
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
                latency: LatencySamples(0),
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

const fn tap(id: u32, bypass_observation: Option<f32>) -> ObservationDescriptor {
    ObservationDescriptor {
        id: ObservationTapId(id),
        display_name: "Gain Reduction",
        display_unit: "dB",
        kind: ObservationKind::GainReductionDb,
        unit: ParameterUnit::Db,
        cost: ObservationCost::Resident,
        cadence: ObservationCadence::PerBlock,
        fold: ObservationFold::PeakMagnitude,
        channels: ObservationChannels::PerLane,
        minimum: 0.0,
        maximum: 100.0,
        bypass_observation,
    }
}
/// Tap 0 declares the bypass value `0.0`; tap 1 declares none and needs the wet path.
static TAPS: [ObservationDescriptor; 2] = [tap(1, Some(0.0)), tap(2, None)];

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

/// One chain of one live slot: every lane has a live channel (parameter 0 per lane, one target
/// slot, an observation queue) and both taps.
struct World {
    chain: BankChain,
    log: Arc<Mutex<Log>>,
    gate: RevisionGate,
    pin: GatePin,
    writers: Vec<EffectCellWriter>,
    /// `[lane][tap]`.
    readers: Vec<[ObservationReader; 2]>,
    block: u64,
}

impl World {
    fn new(bypassed: [bool; LANES]) -> Self {
        let (gate, pin) = RevisionGate::pair(0);
        let superseded = LiveValuesSuperseded::new();
        let log = Arc::new(Mutex::new(Log {
            calls: Vec::new(),
            gain: [1.0; LANES],
        }));
        let (mut writers, mut readers, mut controls, mut observations) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for bypass in bypassed {
            let (writer, control): (EffectCellWriter, EffectControlLane) = effect_control_lane(
                EffectLaneSpec {
                    shape: EffectLaneShape::from_live_parameters(
                        [(0, ParameterChannelPolicy::PerLane)],
                        1,
                    ),
                    bypass,
                    observation_depth: 4,
                    link: None,
                },
                &superseded,
            )
            .expect("lane");
            writers.push(writer);
            controls.push(Some(control));
            let ((first, first_reader), (second, second_reader)) =
                (observation_slot(), observation_slot());
            readers.push([first_reader, second_reader]);
            observations.push(Some(
                ObservationLane::new(&TAPS, vec![first, second], 1).expect("taps"),
            ));
        }
        let stage = LiveControlEffectBankStage::new(
            bound(&log),
            BankWidth::Four,
            QUANTUM,
            controls,
            observations,
            0,
        )
        .expect("stage");
        let chain = BankChain::new(
            AoSoaScratch::new(BankWidth::Four, QUANTUM).expect("scratch"),
            vec![true; LANES].into_boxed_slice(),
            vec![BankSlot {
                stage: Box::new(stage) as Box<dyn BankStage>,
                active_lanes: vec![true; LANES].into_boxed_slice(),
            }],
        )
        .expect("chain");
        Self {
            chain,
            log,
            gate,
            pin,
            writers,
            readers,
            block: 0,
        }
    }

    /// Write `record` to lane `lane` as a transaction of its own.
    fn push(&mut self, lane: usize, record: EffectControlRecord) {
        if let EffectControlRecord::Observe {
            tap_index, armed, ..
        } = record
        {
            self.writers[lane]
                .try_push_observe(tap_index, armed, 1, 0)
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

    /// Render one block of a per-lane constant `lane + 1` and return the output's left planes,
    /// with the calls the bank took in it.
    fn step(&mut self) -> (Vec<Vec<f32>>, Vec<Call>) {
        let plane = |lane: usize| vec![lane as f32 + 1.0; QUANTUM as usize];
        let mut members = Planes {
            left: (0..LANES).map(plane).collect(),
            right: (0..LANES).map(plane).collect(),
        };
        let before = self.log.lock().expect("log").calls.len();
        self.chain
            .run(
                &mut members,
                QUANTUM,
                self.block * u64::from(QUANTUM),
                self.gate.snapshot(),
            )
            .expect("block");
        self.block += 1;
        let calls = self.log.lock().expect("log").calls[before..].to_vec();
        (members.left, calls)
    }

    fn skips(&self) -> u64 {
        self.chain.bypass_skips()
    }

    fn gain(&self, lane: usize) -> f32 {
        self.log.lock().expect("log").gain[lane]
    }
}

const fn bypass(bypassed: bool, ramp_samples: u32) -> EffectControlRecord {
    EffectControlRecord::Bypass(BypassSwitch {
        bypassed,
        ramp_samples,
    })
}

const fn gain(value: f32) -> EffectControlRecord {
    EffectControlRecord::Parameter {
        parameter_index: 0,
        channel: ParameterChannel::Left,
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

const fn observe(tap_index: u32, armed: bool) -> EffectControlRecord {
    EffectControlRecord::Observe {
        tap_index,
        armed,
        window_blocks: 1,
        arm_tag: 0,
    }
}

/// The dry output of [`World::step`]'s input: lane `l` holds `l + 1` on every frame.
fn dry() -> Vec<Vec<f32>> {
    (0..LANES)
        .map(|lane| vec![lane as f32 + 1.0; QUANTUM as usize])
        .collect()
}

/// Gate 3: which blocks skip.
///
/// * every lane settled bypassed: all `BLOCKS` blocks skip, the bank is never called, and the
///   output is the dry block;
/// * one enabled lane: none skips;
/// * a staged span, a prepared target or an armed tap that declares no bypass value, on one
///   bypassed lane: that block (or, for the tap, every block it is armed) runs the bank, and the
///   next skips again;
/// * the only armed tap declares `0.0`: every block skips and folds `0.0` into the lane's window,
///   one window per block, where a running bank reads `7.0`;
/// * every lane crossfading to bypass: no block skips until the crossfades settle.
///
/// Red mutations: the predicate ignores `packed` (a span skipped), ignores `any_armed_computed`
/// (the `None` tap skipped), drops `accumulate_bypass` from the skipped block (no window), or reads
/// `bypassed()` in place of `settled_bypassed()` (the crossfade's blocks skipped).
#[test]
fn a_bank_whose_every_lane_is_settled_bypassed_skips_unless_a_lane_has_work() {
    let mut world = World::new([true; LANES]);
    for _ in 0..BLOCKS {
        let (output, calls) = world.step();
        assert_eq!(calls, [], "the bank is not called");
        assert_eq!(output, dry(), "a skipped block emits the dry block");
    }
    assert_eq!(world.skips(), BLOCKS, "every block skipped");

    let mut world = World::new([true, true, true, false]);
    for _ in 0..BLOCKS {
        let _ = world.step();
    }
    assert_eq!(world.skips(), 0, "one enabled lane keeps the bank running");

    let mut world = World::new([true; LANES]);
    let _ = world.step();
    for (what, record) in [
        ("a staged span", gain(0.5)),
        ("a prepared target", target(0.5)),
    ] {
        let skips = world.skips();
        world.push(1, record);
        let (_, calls) = world.step();
        assert!(calls.contains(&Call::Process), "{what}: the bank ran");
        assert_eq!(world.skips(), skips, "{what}: that block did not skip");
        let _ = world.step();
        assert_eq!(world.skips(), skips + 1, "{what}: the next block skipped");
    }
    world.push(2, observe(1, true));
    for _ in 0..3 {
        let skips = world.skips();
        let (_, calls) = world.step();
        assert!(
            calls.contains(&Call::Process),
            "an armed computed tap runs the bank"
        );
        assert_eq!(
            world.skips(),
            skips,
            "an armed computed tap blocks the skip"
        );
    }
    world.push(2, observe(1, false));
    let _ = world.step();
    let skips = world.skips();
    let _ = world.step();
    assert_eq!(world.skips(), skips + 1, "disarmed, the bank skips again");

    // The tap that declares `0.0`: skipped blocks fold it, one window per block.
    world.push(3, observe(0, true));
    for sequence in 1..=3 {
        let skips = world.skips();
        let _ = world.step();
        assert_eq!(
            world.skips(),
            skips + 1,
            "a declared tap does not block the skip"
        );
        let window = world.readers[3][0].read().expect("a window per block");
        assert_eq!(window.sequence, sequence, "one window per skipped block");
        assert_eq!(
            (window.left.to_bits(), window.right.to_bits()),
            (0, 0),
            "the bypass value, not the running state's 7.0"
        );
    }

    // Crossfading to bypass over 20 frames (three blocks of eight): the blocks run.
    let mut world = World::new([false; LANES]);
    for lane in 0..LANES {
        world.push(lane, bypass(true, 20));
    }
    for block in 0..3 {
        let (_, calls) = world.step();
        assert_eq!(
            calls,
            [Call::Process],
            "block {block}: a crossfading bank runs"
        );
    }
    let _ = world.step();
    assert_eq!(world.skips(), 1, "settled, the bank skips");
}

/// Gate 4: the block that runs a skipped bank again resets it once, with
/// `DiscontinuityKeepParameters`, before the bank processes; a parameter the lane took while the
/// bank was stale is its value after the reset; and a bank that never skipped is never reset.
///
/// Red mutations: drop the reset from `resume` (no reset call), pass `FullToDefaults` (the gain
/// returns to `1` and the kind differs), or leave `stale` set after the reset (every later block
/// resets).
#[test]
fn a_resumed_bank_resets_once_keeping_parameters_and_a_running_bank_never_resets() {
    let mut world = World::new([true; LANES]);
    for _ in 0..3 {
        let (_, calls) = world.step();
        assert_eq!(calls, []);
    }
    // A span while stale runs the bank: it resumes, so it resets first and then applies the span.
    world.push(0, gain(0.25));
    let (_, calls) = world.step();
    assert_eq!(
        calls,
        [
            Call::Reset(ResetKind::DiscontinuityKeepParameters),
            Call::Process
        ],
        "the span's block resets the stale bank, then runs"
    );
    for _ in 0..3 {
        let (_, calls) = world.step();
        assert_eq!(calls, [], "stale again");
    }
    // Lane 0 leaves bypass: its block resets once and runs; the next blocks only run.
    world.push(0, bypass(false, 0));
    let (output, calls) = world.step();
    assert_eq!(
        calls,
        [
            Call::Reset(ResetKind::DiscontinuityKeepParameters),
            Call::Process
        ],
        "the resuming block"
    );
    assert_eq!(world.gain(0), 0.25, "the reset kept the lane's parameter");
    assert_eq!(
        output[0],
        vec![0.25; QUANTUM as usize],
        "lane 0 renders wet"
    );
    assert_eq!(output[1..], dry()[1..], "the bypassed lanes stay dry");
    for _ in 0..3 {
        let (_, calls) = world.step();
        assert_eq!(calls, [Call::Process], "a running bank is not reset again");
    }

    let mut world = World::new([false, true, true, true]);
    for _ in 0..3 {
        let _ = world.step();
    }
    world.push(1, bypass(false, 0));
    for _ in 0..3 {
        let (_, calls) = world.step();
        assert_eq!(
            calls,
            [Call::Process],
            "a never-skipped bank is never reset"
        );
    }
    assert_eq!(world.skips(), 0);
}
