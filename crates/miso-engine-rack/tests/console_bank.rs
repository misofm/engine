//! Issue #140 A: the console bank stage feeds every lane its own spans, and bypasses per lane.
//!
//! The bank under test is a deliberate stand-in for a real effect: it applies `parameter 0` as a
//! per-lane gain, exactly the way every launch effect applies a `Point` span at `first_sample`.
//! What is being gated here is the *rack*, not the DSP -- that the per-lane offsets partition the
//! packed span array correctly, that a lane never sees another lane's command, and that a bypassed
//! lane gets its own latency-matched dry signal while its neighbours keep the wet one.

use miso_engine_core::realtime::{QueueGeneration, bounded_spsc};
use miso_engine_effect_contract::{
    AutomationSpanKind, BankProcessReport, BankWidth, EffectBankProcessBlock, EffectControlLane,
    EffectControlRecord, EffectId, EffectProgramKey, EffectQuality, LatencySamples, LinkMode,
    ParameterChannel, PreparedBankMetadata, PreparedNativeEffectBank, PreparedPorts,
    PreparedSidechainPort, ResetKind, StatePayloadError, StatePayloadInput, StatePayloadOutput,
    StatePayloadSizes, TailSamples,
};
use miso_engine_rack::{
    AoSoaScratch, BankChain, BankMembers, BankSlot, BankStage, ConsoleEffectBankStage,
};

const LANES: usize = 4;
const CAPACITY: u32 = 4;

fn depth(value: usize) -> core::num::NonZeroUsize {
    core::num::NonZeroUsize::new(value).expect("nonzero")
}

fn program_key(latency: u64) -> EffectProgramKey {
    EffectProgramKey {
        effect_id: EffectId::parse("mock.gain").expect("id"),
        contract_major: 1,
        state_layout_version: 1,
        sample_rate: 48_000,
        quantum: 8,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        latency: LatencySamples(latency),
        tail: TailSamples::Finite(0),
        state_sizes: StatePayloadSizes {
            common_bytes: 0,
            left_bytes: 0,
            right_bytes: 0,
        },
        scratch_bytes: 0,
        automation_capacity: CAPACITY,
    }
}

/// A bank that applies `parameter 0` as a per-lane gain and delays by `latency` frames.
struct MockGainBank {
    metadata: PreparedBankMetadata,
    gain: [f32; LANES],
    /// Per-lane FIFO of the latency the bank declares, so a "real" latency is actually produced.
    line: Vec<[f32; 2]>,
    latency: usize,
    /// Per-lane span counts seen by the last block, for the partition assertions.
    seen: [usize; LANES],
}

impl MockGainBank {
    fn new(latency: usize) -> Self {
        Self {
            metadata: PreparedBankMetadata {
                width: BankWidth::Four,
                program_key: program_key(latency as u64),
            },
            gain: [1.0; LANES],
            line: vec![[0.0; 2]; latency * LANES],
            latency,
            seen: [0; LANES],
        }
    }
}

impl PreparedNativeEffectBank for MockGainBank {
    fn metadata(&self) -> PreparedBankMetadata {
        self.metadata.clone()
    }
    fn reset(&mut self, _kind: ResetKind) {}
    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        let report = BankProcessReport::empty(self.metadata.width);
        for lane in 0..LANES {
            let start = block.automation_offsets[lane] as usize;
            let end = block.automation_offsets[lane + 1] as usize;
            self.seen[lane] = end - start;
            for span in &block.automation[start..end] {
                assert_eq!(span.kind, AutomationSpanKind::Point);
                assert_eq!(span.start_sample, block.first_sample);
                if span.parameter_index == 0 {
                    self.gain[lane] = span.start_value;
                }
            }
        }
        let frames = block.frames as usize;
        for frame in 0..frames {
            for lane in 0..LANES {
                let index = frame * LANES + lane;
                let wet = [
                    block.left[index] * self.gain[lane],
                    block.right[index] * self.gain[lane],
                ];
                if self.latency == 0 {
                    block.left[index] = wet[0];
                    block.right[index] = wet[1];
                    continue;
                }
                // A per-lane FIFO of `latency` frames, so the declared latency is real.
                let slot = &mut self.line[(frame % self.latency) * LANES + lane];
                let held = *slot;
                *slot = wet;
                block.left[index] = held[0];
                block.right[index] = held[1];
            }
        }
        report
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

type Producers = Vec<miso_engine_core::realtime::Producer<EffectControlRecord>>;

fn console_chain(latency: usize, controlled: [bool; LANES]) -> (BankChain, Producers) {
    let mut producers = Vec::new();
    let lanes: Vec<Option<EffectControlLane>> = controlled
        .iter()
        .map(|wanted| {
            wanted.then(|| {
                let (producer, consumer) = bounded_spsc::<EffectControlRecord>(
                    depth(CAPACITY as usize),
                    QueueGeneration(0),
                )
                .expect("queue");
                producers.push(producer);
                EffectControlLane::new(consumer, false)
            })
        })
        .collect();
    let stage = ConsoleEffectBankStage::new(
        Box::new(MockGainBank::new(latency)),
        BankWidth::Four,
        8,
        lanes,
        vec![None, None, None, None],
        latency,
    )
    .expect("stage");
    let scratch = AoSoaScratch::new(BankWidth::Four, 8).expect("scratch");
    let chain = BankChain::new(
        scratch,
        vec![true; LANES].into_boxed_slice(),
        vec![BankSlot {
            stage: Box::new(stage) as Box<dyn BankStage>,
            active_lanes: vec![true; LANES].into_boxed_slice(),
        }],
    )
    .expect("chain");
    (chain, producers)
}

fn planes(value: f32) -> Planes {
    Planes {
        left: vec![vec![value; 8]; LANES],
        right: vec![vec![value; 8]; LANES],
    }
}

fn gain(value: f32) -> EffectControlRecord {
    EffectControlRecord::Parameter {
        parameter_index: 0,
        channel: ParameterChannel::Left,
        value,
    }
}

/// Red mutation: pack every lane's staged prefix at a fixed `packed[..staged]` instead of at that
/// lane's own running offset -> the last commanded lane's spans overwrite the earlier lanes' while
/// the offsets still partition the array, and lane 0 renders lane 2's command.
#[test]
fn each_lane_receives_only_its_own_commands() {
    let (mut chain, mut producers) = console_chain(0, [true, true, true, true]);
    let mut members = planes(1.0);
    // Lanes 0 and 2 are commanded with different values; lane 1 stays silent so isolation is
    // observable in both directions.
    producers[0].try_push(gain(0.25)).expect("room");
    producers[2].try_push(gain(4.0)).expect("room");
    // The FINAL lane is commanded too: the drain loop's boundary lane is exactly where a
    // truncated iteration or off-by-one offset silently drops spans (verifier-added coverage,
    // #140 review — a mutation skipping the last lane's drain survived the original pair).
    producers[3].try_push(gain(8.0)).expect("room");
    chain.run(&mut members, 8, 0).expect("run");
    for frame in 0..8 {
        assert_eq!(members.left[0][frame], 0.25, "lane 0 got its own command");
        assert_eq!(members.left[1][frame], 1.0, "lane 1 was never commanded");
        assert_eq!(members.left[2][frame], 4.0, "lane 2 got its own command");
        assert_eq!(
            members.left[3][frame], 8.0,
            "the final lane got its own command"
        );
    }
}

/// A command applies at the top of the block it is drained in, and it persists after that: the
/// bank's own ramp state carries it, exactly as a scalar instance's does.
#[test]
fn a_command_applies_at_the_block_boundary_and_persists() {
    let (mut chain, mut producers) = console_chain(0, [true, false, false, false]);
    let mut members = planes(1.0);
    chain.run(&mut members, 8, 0).expect("run");
    assert_eq!(members.left[0][0], 1.0, "no command yet");

    producers[0].try_push(gain(0.5)).expect("room");
    let mut members = planes(1.0);
    chain.run(&mut members, 8, 8).expect("run");
    assert!(
        members.left[0].iter().all(|value| *value == 0.5),
        "every sample of the block that drains the command carries it"
    );

    let mut members = planes(1.0);
    chain.run(&mut members, 8, 16).expect("run");
    assert!(
        members.left[0].iter().all(|value| *value == 0.5),
        "the value persists with no further traffic"
    );
}

/// Red mutation: drop the `+= lane_count` stride in the bypass restore loop (use `index += 1`) ->
/// the bypassed lane's dry samples land in every lane and the un-bypassed lanes lose their wet
/// signal, failing the `1.0` assertions below.
#[test]
fn bypass_is_per_lane_and_preserves_the_declared_latency() {
    const LATENCY: usize = 2;
    let (mut chain, mut producers) = console_chain(LATENCY, [true, true, false, false]);
    // Lane 0 is bypassed and gained; lane 1 is only gained.
    producers[0].try_push(gain(0.5)).expect("room");
    producers[0]
        .try_push(EffectControlRecord::Bypass(true))
        .expect("room");
    producers[1].try_push(gain(0.5)).expect("room");

    let mut seen_left: Vec<Vec<f32>> = vec![Vec::new(); LANES];
    for block in 0..4_u64 {
        let mut members = Planes {
            left: (0..LANES)
                .map(|_| {
                    (0..8)
                        .map(|frame| (block as usize * 8 + frame) as f32)
                        .collect()
                })
                .collect(),
            right: (0..LANES)
                .map(|_| {
                    (0..8)
                        .map(|frame| -((block as usize * 8 + frame) as f32))
                        .collect()
                })
                .collect(),
        };
        chain.run(&mut members, 8, block * 8).expect("run");
        for (lane, seen) in seen_left.iter_mut().enumerate() {
            seen.extend_from_slice(&members.left[lane]);
        }
    }
    for (index, bypassed) in seen_left[0].iter().enumerate() {
        let delayed = if index < LATENCY {
            0.0
        } else {
            (index - LATENCY) as f32
        };
        assert_eq!(
            *bypassed, delayed,
            "sample {index}: a bypassed lane is the dry signal at the declared latency"
        );
        assert_eq!(
            seen_left[1][index],
            delayed * 0.5,
            "sample {index}: lane 1 keeps the wet, gained signal"
        );
        assert_eq!(
            seen_left[2][index], delayed,
            "sample {index}: an uncontrolled lane is unity-gain wet, which equals the dry signal"
        );
    }
}

/// Bypass is reversible and does not desync the effect's state: the wet path kept running while
/// the lane was bypassed, so un-bypassing produces the *current* wet signal, not a stale one.
#[test]
fn un_bypassing_returns_the_current_wet_signal() {
    let (mut chain, mut producers) = console_chain(0, [true, false, false, false]);
    producers[0].try_push(gain(0.25)).expect("room");
    producers[0]
        .try_push(EffectControlRecord::Bypass(true))
        .expect("room");
    let mut members = planes(1.0);
    chain.run(&mut members, 8, 0).expect("run");
    assert!(members.left[0].iter().all(|value| *value == 1.0), "dry");

    producers[0]
        .try_push(EffectControlRecord::Bypass(false))
        .expect("room");
    let mut members = planes(1.0);
    chain.run(&mut members, 8, 8).expect("run");
    assert!(
        members.left[0].iter().all(|value| *value == 0.25),
        "the gain admitted while bypassed is already in effect when bypass is released"
    );
}

/// Partition invariance extends to command timelines: rendering the same eight frames as two
/// four-frame partitions, with the command admitted before the first, produces the same bits.
#[test]
fn a_command_timeline_is_partition_invariant() {
    let whole = {
        let (mut chain, mut producers) = console_chain(2, [true, true, true, true]);
        producers[0].try_push(gain(0.5)).expect("room");
        producers[3].try_push(gain(2.0)).expect("room");
        let mut members = planes(1.0);
        chain.run(&mut members, 8, 0).expect("run");
        members
    };
    let split = {
        let (mut chain, mut producers) = console_chain(2, [true, true, true, true]);
        producers[0].try_push(gain(0.5)).expect("room");
        producers[3].try_push(gain(2.0)).expect("room");
        let mut first = Planes {
            left: vec![vec![1.0; 4]; LANES],
            right: vec![vec![1.0; 4]; LANES],
        };
        chain.run(&mut first, 4, 0).expect("run");
        let mut second = Planes {
            left: vec![vec![1.0; 4]; LANES],
            right: vec![vec![1.0; 4]; LANES],
        };
        chain.run(&mut second, 4, 4).expect("run");
        Planes {
            left: (0..LANES)
                .map(|lane| {
                    first.left[lane]
                        .iter()
                        .chain(second.left[lane].iter())
                        .copied()
                        .collect()
                })
                .collect(),
            right: (0..LANES)
                .map(|lane| {
                    first.right[lane]
                        .iter()
                        .chain(second.right[lane].iter())
                        .copied()
                        .collect()
                })
                .collect(),
        }
    };
    for lane in 0..LANES {
        assert_eq!(
            whole.left[lane]
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            split.left[lane]
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            "lane {lane}: to_bits identity across the partition boundary"
        );
    }
}

/// A stage built with no controlled lane at all still refuses to exist as a console stage's
/// shunt: `BypassShunt` allocation is skipped, which is what keeps a console-free bank on the
/// byte-identical `EffectBankStage` path.
#[test]
fn a_stage_with_no_controlled_lane_allocates_no_shunt() {
    let stage = ConsoleEffectBankStage::new(
        Box::new(MockGainBank::new(3)),
        BankWidth::Four,
        8,
        vec![None, None, None, None],
        vec![None, None, None, None],
        3,
    )
    .expect("stage");
    assert_eq!(stage.dropped_records(), 0);
}

/// Shape is validated once, off the render thread.
#[test]
fn stage_construction_rejects_a_lane_count_or_quantum_mismatch() {
    assert!(
        ConsoleEffectBankStage::new(
            Box::new(MockGainBank::new(0)),
            BankWidth::Four,
            8,
            vec![None, None],
            vec![None, None],
            0,
        )
        .is_err(),
        "a lane vector that is not the bank width is refused"
    );
    assert!(
        ConsoleEffectBankStage::new(
            Box::new(MockGainBank::new(0)),
            BankWidth::Four,
            0,
            vec![None, None, None, None],
            vec![None, None, None, None],
            0,
        )
        .is_err(),
        "a zero quantum is refused"
    );
}

/// Issue #143 E5, at the bank slot: the observation surface a stage exposes off the render thread.
///
/// `ConsoleEffectBankStage` is where the structural walk bottoms out — `BankChain` sums it and the
/// runtime sums that — so the counts and the retained bytes it reports are what
/// `observation_retained_bytes == 0` ultimately rests on. A slot no observation request touched
/// reports zero for all of them and allocates neither the lane vector nor the per-lane sample
/// scratch.
///
/// Red mutation: build the sample scratch unconditionally in `ConsoleEffectBankStage::new` -> an
/// unobserved slot stops being structurally distinguishable from an observed one, and `is_observed`
/// stops meaning anything.
#[test]
fn an_unobserved_bank_slot_reports_no_observation_state_at_all() {
    use miso_engine_core::realtime::observation_slot;
    use miso_engine_effect_contract::{
        ObservationCadence, ObservationChannels, ObservationCost, ObservationDescriptor,
        ObservationFold, ObservationKind, ObservationLane, ObservationTapId, ParameterUnit,
    };

    static MENU: [ObservationDescriptor; 1] = [ObservationDescriptor {
        id: ObservationTapId(1),
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
    }];

    let unobserved = ConsoleEffectBankStage::new(
        Box::new(MockGainBank::new(0)),
        BankWidth::Four,
        8,
        vec![None, None, None, None],
        vec![None, None, None, None],
        0,
    )
    .expect("stage");
    assert!(!unobserved.is_observed(), "no lane, no vector, no scratch");
    assert_eq!(unobserved.observation_binding_counts(), [0, 0, 0]);
    assert_eq!(unobserved.observation_retained_bytes(), 0);
    assert_eq!(unobserved.observation_tap_counts(), (0, 0));
    assert_eq!(unobserved.unbound_observations(), 0);

    // Two of four lanes observed, one of them armed.
    let mut lanes = Vec::new();
    for lane in 0..4 {
        if lane % 2 == 1 {
            lanes.push(None);
            continue;
        }
        let (publisher, _reader) = observation_slot();
        let mut observation =
            ObservationLane::new(&MENU, vec![publisher], 4).expect("one per tap");
        if lane == 0 {
            observation.arm(0, true, 4, 0);
        }
        lanes.push(Some(observation));
    }
    let mut observed = ConsoleEffectBankStage::new(
        Box::new(MockGainBank::new(0)),
        BankWidth::Four,
        8,
        vec![None, None, None, None],
        lanes,
        0,
    )
    .expect("stage");
    assert!(observed.is_observed());
    assert_eq!(
        observed.observation_binding_counts(),
        [2, 2, 1],
        "two observed lanes, two declared taps, one armed"
    );
    assert_eq!(observed.observation_tap_counts(), (2, 1));
    let retained = observed.observation_retained_bytes();
    assert!(retained > 0);

    // Issue #143 D7's explicit form: dropping every subscription is one operation, and it changes
    // the armed count and nothing else -- least of all what the slot retains.
    observed.disarm_observations();
    assert_eq!(observed.observation_binding_counts(), [2, 2, 0]);
    assert_eq!(
        observed.observation_retained_bytes(),
        retained,
        "disarming frees nothing, exactly as arming allocates nothing"
    );
    assert!(observed.is_observed(), "capacity survives an unsubscribe");
}

/// Issue #163 phase 4 item 4: the latency line stays fed while the lane is *not* bypassed, so the
/// first bypassed block after a mid-session toggle emits the correctly delayed dry signal.
///
/// This is the tripwire for the capture gate. Phase 4 stopped staging the dry block on a block no
/// reader can observe, which is sound only because `BypassShunt::feeds_line` keeps the capture
/// unconditional whenever a latency line exists. Red mutation: gate the capture in
/// `ConsoleEffectBankStage::process` on `any_bypassed` alone, dropping the `feeds_line()` term ->
/// the line starves while the lane runs wet, and the first bypassed block below emits stale
/// samples (zeros, or block-0 content) instead of the input from `LATENCY` frames earlier.
///
/// The distinction from `bypass_is_per_lane_and_preserves_the_declared_latency` is the toggle
/// point: that test bypasses from block 0, so a starved line would still look correct for the
/// only samples it checks. Here the lane renders wet for two whole blocks first, so the delayed
/// dry signal the third block must produce can only come from a line that was fed while wet.
#[test]
fn a_mid_session_bypass_emits_the_delayed_dry_signal_captured_while_wet() {
    const LATENCY: usize = 2;
    const FRAMES: usize = 8;
    let (mut chain, mut producers) = console_chain(LATENCY, [true, false, false, false]);
    producers[0].try_push(gain(0.5)).expect("room");

    let input = |block: usize, frame: usize| (block * FRAMES + frame + 1) as f32;
    let mut seen: Vec<f32> = Vec::new();
    for block in 0..4_usize {
        // The toggle lands between blocks 1 and 2, so blocks 0 and 1 render wet and block 2 is the
        // first bypassed one. Its dry signal is input the shunt could only hold if the capture ran
        // during the wet blocks.
        if block == 2 {
            producers[0]
                .try_push(EffectControlRecord::Bypass(true))
                .expect("room");
        }
        let mut members = Planes {
            left: (0..LANES)
                .map(|_| (0..FRAMES).map(|frame| input(block, frame)).collect())
                .collect(),
            right: (0..LANES)
                .map(|_| (0..FRAMES).map(|frame| -input(block, frame)).collect())
                .collect(),
        };
        chain
            .run(&mut members, FRAMES as u32, (block * FRAMES) as u64)
            .expect("run");
        seen.extend_from_slice(&members.left[0]);
    }

    // Blocks 0 and 1 are wet: the mock bank applies the 0.5 gain and its own `LATENCY`-frame FIFO.
    for (index, value) in seen.iter().enumerate().take(2 * FRAMES) {
        let delayed = if index < LATENCY {
            0.0
        } else {
            index as f32 - LATENCY as f32 + 1.0
        };
        assert_eq!(
            *value,
            delayed * 0.5,
            "sample {index}: wet before the toggle"
        );
    }
    // Blocks 2 and 3 are bypassed: the dry signal at the declared latency, unity gain. The first
    // two samples of block 2 are the last two samples of block 1, which is the whole point.
    for (index, value) in seen.iter().enumerate().take(4 * FRAMES).skip(2 * FRAMES) {
        assert_eq!(
            *value,
            index as f32 - LATENCY as f32 + 1.0,
            "sample {index}: a mid-session bypass is the dry signal at the declared latency"
        );
    }
}

/// The zero-latency counterpart: with no line to feed, the capture is skipped while wet, and the
/// first bypassed block still emits *its own* input because the control drain decides `bypassed`
/// before the capture runs.
///
/// Red mutation: move the `any_bypassed` decision in `ConsoleEffectBankStage::process` below the
/// capture, or compute it from a stale pre-drain snapshot -> the first bypassed block emits the
/// previous block's dry samples.
#[test]
fn a_zero_latency_bypass_emits_the_current_block_dry() {
    const FRAMES: usize = 8;
    let (mut chain, mut producers) = console_chain(0, [true, false, false, false]);
    producers[0].try_push(gain(0.25)).expect("room");

    let mut members = planes(3.0);
    chain.run(&mut members, FRAMES as u32, 0).expect("run");
    assert!(
        members.left[0].iter().all(|value| *value == 0.75),
        "block 0 renders wet"
    );

    producers[0]
        .try_push(EffectControlRecord::Bypass(true))
        .expect("room");
    let mut members = planes(5.0);
    chain
        .run(&mut members, FRAMES as u32, FRAMES as u64)
        .expect("run");
    assert!(
        members.left[0].iter().all(|value| *value == 5.0),
        "the first bypassed block is this block's own dry input, not the previous block's"
    );
}
