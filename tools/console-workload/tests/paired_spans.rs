//! Issue #1004: the mono collapse survives a both-channel write that arrives as two records.
//!
//! The web host lowers a `PerLane` parameter's "both channels" command to a `Left` record and a
//! `Right` record, and before this issue each one cleared the lane's `LIVE` witness term at the
//! drain, so the first compressor or limiter touch on a mono stem retired its whole bank's collapse
//! for the rest of the plan. `EffectControlLane::stage` now defers a one-channel parameter record
//! to the end of its drain and keeps `LIVE` when the drain's staged spans pair (the drain-level
//! rule and its exhaustive model are `crates/effect-contract/tests/paired_spans.rs`).
//!
//! That is sound only if a kept drain leaves the two channels in bit-equal state, so this file is
//! the rendered proof, in four parts:
//!
//! 1. **Witness soundness, differential (gate 1).** The real EQ, compressor and true-peak limiter
//!    banks, alone and chained, each bound behind the production `ConsoleEffectBankStage` in a
//!    production `BankChain`, rendered twice from the same records: once with the collapse armed
//!    and once forced dual. After every block, every output word, every bank report and every
//!    lane's full state payload is compared by bits. The collapse counters must show it kept
//!    exactly where the rule keeps it.
//! 2. **The per-effect obligation (amendment A3).** Every launch effect with live `PerLane`
//!    parameters applies a twin pair, a pair at and past its domain edges, and a refused pair with
//!    channel-symmetric validity: bit-equal channels stay bit-equal.
//! 3. **Engagement on the console (gate 2).** The mono console keeps every cohort collapsed
//!    through both-channel rides and restatements, and a one-channel write or a near pair retires
//!    exactly its own cohort, while rendering the forced-dual bits block by block.
//! 4. **The pinned scenario (gate 3).** The mono console's 8-of-64 mixed ride at `Simd8` and
//!    `Simd4`, pinned to the digests of the base tree this issue started from. The digest may not
//!    move; only the collapse counters may.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, MutexGuard};

use bench_support::digest::Sha256Sink;
use console_workload::{ObservationArm, PlanConfig, SessionRuntime, Workload};
use effect_compiler::launch_native_effect_registry;
use effect_contract::{
    AutomationRate, AutomationSpanKind, BankProcessReport, BankWidth, ChannelSymmetryWitness,
    EffectBankProcessBlock, EffectControlLane, EffectControlRecord, EffectDescriptor,
    EffectProcessBlock, EffectQuality, EffectTargetError, EffectTargetRequest,
    InitialParameterValue, LinkMode, NativeEffectFactory, ObservationSample, ParameterChannel,
    ParameterChannelPolicy, ParameterDescriptor, ParameterDomain, PortRole,
    PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan,
    PreparedBankMetadata, PreparedEffectTarget, PreparedNativeEffect, PreparedNativeEffectBank,
    PreparedPorts, PreparedSidechainPort, ProcessBlockError, ResetKind, ResponseAnalysisError,
    ResponseSnapshotRequest, ResponseSnapshotSummary, SeamSide, StatePayloadError,
    StatePayloadInput, StatePayloadOutput, StatePayloadSizes, default_initial_values,
};
use engine::realtime::{Producer, QueueGeneration, RenderError, bounded_spsc};
use lane::Backend;
use rack::{
    AoSoaScratch, BankBlock, BankChain, BankMembers, BankSlot, BankStage, ConsoleEffectBankStage,
};

const RATE: u32 = 48_000;
const FRAMES: u32 = 128;
/// Live-console queue depth per lane, and the automation capacity every bank is prepared with, so
/// the staging window is exactly the queue (preparation refuses a deeper queue in production).
const QUEUE: usize = 16;

const EQ: &str = "miso.parametric-eq";
const COMPRESSOR: &str = "miso.compressor";
const LIMITER: &str = "miso.true-peak-limiter";

/// `band-1-gain`, `band-1-frequency` of the EQ; `threshold`, `ratio`, `attack` of the compressor;
/// `ceiling`, `release` of the limiter. Descriptor indices, not wire ids.
const EQ_GAIN: u32 = 3;
const EQ_FREQUENCY: u32 = 2;
const COMPRESSOR_THRESHOLD: u32 = 0;
const COMPRESSOR_RATIO: u32 = 1;
const COMPRESSOR_ATTACK: u32 = 3;
const LIMITER_CEILING: u32 = 0;
const LIMITER_RELEASE: u32 = 1;

// =============================================================================================
// Part 1: the bank-level differential
// =============================================================================================

/// The three effects that can sit in a collapse-eligible cohort: every other launch effect's bank
/// declines `supports_mono_collapse`, so a chain holding one never collapses
/// (`BankChain::collapse_prefix_of`). `every_collapse_capable_launch_effect_is_one_of_these`
/// keeps this list honest.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Fx {
    Eq,
    Compressor,
    Limiter,
}

impl Fx {
    const fn id(self) -> &'static str {
        match self {
            Self::Eq => EQ,
            Self::Compressor => COMPRESSOR,
            Self::Limiter => LIMITER,
        }
    }

    /// The console fixture's link modes: the limiter is linked, the other two dual-mono.
    const fn link_mode(self) -> LinkMode {
        match self {
            Self::Limiter => LinkMode::Maximum,
            Self::Eq | Self::Compressor => LinkMode::DualMono,
        }
    }

    /// Held values over the descriptor defaults, chosen so every effect is doing work: the EQ's
    /// band 1 is a live bell, the compressor and the limiter reduce the test signal.
    fn held(self) -> &'static [(u32, f32)] {
        match self {
            Self::Eq => &[(0, 1.0), (1, 1.0), (EQ_FREQUENCY, 90.0), (EQ_GAIN, -7.5)],
            Self::Compressor => &[
                (COMPRESSOR_THRESHOLD, -18.0),
                (COMPRESSOR_RATIO, 4.0),
                (2, 3.0),
                (COMPRESSOR_ATTACK, 2.0),
                (4, 40.0),
                (5, 0.0),
                (6, 1.0),
            ],
            Self::Limiter => &[(LIMITER_CEILING, -9.0), (LIMITER_RELEASE, 60.0), (2, 5.0)],
        }
    }

    /// The parameters the scenarios ride, with a value range inside each domain.
    fn rides(self) -> &'static [(u32, f32, f32)] {
        match self {
            Self::Eq => &[(EQ_GAIN, -12.0, 6.0), (EQ_FREQUENCY, 60.0, 400.0)],
            Self::Compressor => &[
                (COMPRESSOR_THRESHOLD, -30.0, -6.0),
                (COMPRESSOR_RATIO, 1.5, 8.0),
                (COMPRESSOR_ATTACK, 1.0, 20.0),
            ],
            Self::Limiter => &[
                (LIMITER_CEILING, -18.0, -3.0),
                (LIMITER_RELEASE, 20.0, 200.0),
            ],
        }
    }
}

fn registry() -> effect_contract::NativeEffectRegistry {
    launch_native_effect_registry().expect("launch effect registry")
}

fn factory(id: &str) -> Arc<dyn NativeEffectFactory> {
    registry()
        .get_shared_ascii(id)
        .unwrap_or_else(|| panic!("{id} is a launch effect"))
}

fn initial_values(fx: Fx) -> Vec<InitialParameterValue> {
    let descriptor = factory(fx.id()).descriptor();
    let mut values: Vec<InitialParameterValue> = default_initial_values(descriptor).collect();
    for (index, value) in fx.held() {
        for item in values
            .iter_mut()
            .filter(|item| item.parameter_index == *index)
        {
            item.value = *value;
        }
    }
    values
}

fn quality(descriptor: &'static EffectDescriptor) -> EffectQuality {
    if descriptor
        .qualities
        .iter()
        .any(|quality| quality.quality == EffectQuality::Normal && quality.sample_rate == RATE)
    {
        EffectQuality::Normal
    } else {
        descriptor
            .qualities
            .iter()
            .find(|quality| quality.sample_rate == RATE)
            .map(|quality| quality.quality)
            .expect("a launch-rate quality")
    }
}

/// The request a session with no routed sidechain prepares, exactly as `effect-compiler` builds it.
fn prepare_request<'a>(
    descriptor: &'static EffectDescriptor,
    values: &'a [InitialParameterValue],
    link_mode: LinkMode,
) -> Option<PrepareEffectRequest<'a>> {
    let sidechain = match descriptor
        .ports
        .iter()
        .find(|port| port.role == PortRole::SidechainInput)
    {
        None => PreparedSidechainPort::None,
        Some(port) if !port.required => PreparedSidechainPort::Unconnected {
            id: port.id,
            required: false,
        },
        Some(_) => return None,
    };
    Some(PrepareEffectRequest {
        sample_rate: RATE,
        quantum: FRAMES,
        quality: quality(descriptor),
        bypass: false,
        link_mode,
        ports: PreparedPorts { sidechain },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 1 << 28,
            maximum_scratch_bytes: 1 << 28,
            maximum_automation_spans_per_block: QUEUE as u32,
        },
    })
}

const fn backend_of(width: BankWidth) -> Backend {
    match width {
        BankWidth::Four => Backend::Simd4,
        BankWidth::Eight => Backend::Simd8,
    }
}

/// Binds `fx` as a homogeneous bank at `width`, or `None` where this build cannot (decision D4:
/// an x86-64-v3 build binds no four-lane EQ or compressor bank).
fn bind(fx: Fx, width: BankWidth) -> Option<Box<dyn PreparedNativeEffectBank>> {
    let factory = factory(fx.id());
    let values = initial_values(fx);
    let request =
        prepare_request(factory.descriptor(), &values, fx.link_mode()).expect("no required port");
    let requests = vec![request; width.lanes() as usize];
    factory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: backend_of(width),
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        })
        .expect("a well-formed bank request")
}

/// One track's state payload: its common, left and right sections.
type Payload = (Vec<u8>, Vec<u8>, Vec<u8>);

/// A bank the chain owns and the test can still reach: the chain calls through the lock, and the
/// test snapshots state, resets, restores and reads the last report between blocks.
struct Shared {
    bank: Box<dyn PreparedNativeEffectBank>,
    report: Option<BankProcessReport>,
}

#[derive(Clone)]
struct SharedBank(Arc<Mutex<Shared>>);

impl SharedBank {
    fn lock(&self) -> MutexGuard<'_, Shared> {
        self.0.lock().expect("bank lock")
    }

    fn take_report(&self) -> Option<BankProcessReport> {
        self.lock().report.take()
    }

    fn snapshot(&self, track: u32) -> Payload {
        let shared = self.lock();
        let sizes = shared.bank.metadata().program_key.state_sizes;
        let mut common = vec![0_u8; sizes.common_bytes as usize];
        let mut left = vec![0_u8; sizes.left_bytes as usize];
        let mut right = vec![0_u8; sizes.right_bytes as usize];
        shared
            .bank
            .snapshot_track_state_payload(
                track,
                StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes)
                    .expect("payload sizes"),
            )
            .expect("snapshot");
        (common, left, right)
    }

    fn restore(&self, track: u32, payload: &Payload) {
        let mut shared = self.lock();
        let metadata = shared.bank.metadata().program_key;
        let (common, left, right) = payload;
        shared
            .bank
            .restore_track_state_payload(
                track,
                metadata.state_layout_version,
                StatePayloadInput::new(common, left, right, metadata.state_sizes)
                    .expect("payload sizes"),
            )
            .expect("restore");
    }
}

impl PreparedNativeEffectBank for SharedBank {
    fn metadata(&self) -> PreparedBankMetadata {
        self.lock().bank.metadata()
    }
    fn reset(&mut self, kind: ResetKind) {
        self.lock().bank.reset(kind);
    }
    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        let mut shared = self.lock();
        let report = shared.bank.process_bank(block);
        shared.report = Some(report);
        report
    }
    fn apply_prepared_target_lane(
        &mut self,
        lane: usize,
        target: &PreparedEffectTarget,
    ) -> Result<(), EffectTargetError> {
        self.lock().bank.apply_prepared_target_lane(lane, target)
    }
    fn observe_resident_bank(&self, tap_index: u32, out: &mut [ObservationSample]) -> bool {
        self.lock().bank.observe_resident_bank(tap_index, out)
    }
    fn copy_response_snapshot_lane(
        &self,
        lane: usize,
        request: ResponseSnapshotRequest<'_>,
    ) -> Result<ResponseSnapshotSummary, ResponseAnalysisError> {
        self.lock().bank.copy_response_snapshot_lane(lane, request)
    }
    fn snapshot_track_state_payload(
        &self,
        track_index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.lock()
            .bank
            .snapshot_track_state_payload(track_index, output)
    }
    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.lock()
            .bank
            .restore_track_state_payload(track_index, state_layout_version, input)
    }
    fn lane_channel_symmetry(&self, lane: usize) -> bool {
        self.lock().bank.lane_channel_symmetry(lane)
    }
    fn supports_mono_collapse(&self) -> bool {
        self.lock().bank.supports_mono_collapse()
    }
    fn process_bank_mono(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        let mut shared = self.lock();
        let report = shared.bank.process_bank_mono(block);
        shared.report = Some(report);
        report
    }
    fn desymmetrize_channels(&mut self) {
        self.lock().bank.desymmetrize_channels();
    }
    fn channels_agree(&self) -> bool {
        self.lock().bank.channels_agree()
    }
}

/// A seam-side fader with different left and right gains, so the chain has the strip's seam
/// suffix and the collapsed block's duplicated plane really is split again after it.
struct Fader;

impl BankStage for Fader {
    fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
        for sample in block.left.iter_mut() {
            *sample *= 0.707_945_8;
        }
        for sample in block.right.iter_mut() {
            *sample *= 0.749_894_2;
        }
        Ok(())
    }
    fn seam_side(&self) -> SeamSide {
        SeamSide::SeamSide
    }
    fn lane_symmetry(&self, _lane: usize) -> ChannelSymmetryWitness {
        ChannelSymmetryWitness::SYMMETRIC
    }
}

#[derive(Clone)]
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

/// Hostile words, on both planes alike: signed zeros, subnormals, NaN, both infinities and a word
/// near `f32::MAX`.
const HOSTILE: [f32; 9] = [
    0.0,
    -0.0,
    f32::from_bits(1),
    -f32::from_bits(0x0000_8000),
    f32::NAN,
    f32::INFINITY,
    f32::NEG_INFINITY,
    3.0e38,
    -1.0e-30,
];

/// One mono block per lane: a lane-specific triangle plus deterministic noise, identical on the
/// two planes (the `SOURCE` term), loud enough that the compressor and the limiter reduce it.
fn input(lanes: usize, block: u64, hostile: bool) -> Planes {
    let plane: Vec<Vec<f32>> = (0..lanes)
        .map(|lane| {
            (0..u64::from(FRAMES))
                .map(|frame| {
                    let n = block * u64::from(FRAMES) + frame;
                    let period = 37 + 11 * lane as u64;
                    let phase = (n % period) as f32 / period as f32;
                    let triangle = 4.0 * (phase - 0.5).abs() - 1.0;
                    let mut state = n
                        .wrapping_mul(6_364_136_223_846_793_005)
                        .wrapping_add(1_442_695_040_888_963_407 ^ lane as u64);
                    state ^= state >> 29;
                    let noise = (state >> 40) as f32 / (1_u64 << 24) as f32 * 2.0 - 1.0;
                    let mut value = 0.7 * triangle + 0.25 * noise;
                    if hostile && (frame as usize) < HOSTILE.len() * 3 {
                        let pick = (frame as usize + lane + block as usize) % HOSTILE.len();
                        if frame % 3 == 0 {
                            value = HOSTILE[pick];
                        }
                    }
                    value
                })
                .collect()
        })
        .collect();
    Planes {
        left: plane.clone(),
        right: plane,
    }
}

/// One slot of a rig: its bank handle and one producer per lane.
struct Slot {
    fx: Fx,
    bank: SharedBank,
    producers: Vec<Producer<EffectControlRecord>>,
}

/// One production chain over real banks: `slots` console effect stages and a seam-side fader.
struct Rig {
    chain: BankChain,
    slots: Vec<Slot>,
    lanes: usize,
}

impl Rig {
    fn new(fxs: &[Fx], width: BankWidth, forced_dual: bool) -> Option<Self> {
        let lanes = width.lanes() as usize;
        let mut slots = Vec::new();
        let mut stages = Vec::new();
        for fx in fxs {
            let bank = bind(*fx, width)?;
            let metadata = bank.metadata().program_key;
            assert!(
                metadata.automation_capacity as usize >= QUEUE,
                "{}: the staging window must hold the whole queue",
                fx.id()
            );
            let latency = usize::try_from(metadata.latency.0).expect("latency");
            let shared = SharedBank(Arc::new(Mutex::new(Shared { bank, report: None })));
            let mut producers = Vec::new();
            let control: Vec<Option<EffectControlLane>> = (0..lanes)
                .map(|_| {
                    let (producer, consumer) = bounded_spsc::<EffectControlRecord>(
                        core::num::NonZeroUsize::new(QUEUE).expect("depth"),
                        QueueGeneration(0),
                    )
                    .expect("queue");
                    producers.push(producer);
                    Some(if *fx == Fx::Eq {
                        EffectControlLane::new_with_target_staging(consumer, false)
                    } else {
                        EffectControlLane::new(consumer, false)
                    })
                })
                .collect();
            let stage = ConsoleEffectBankStage::new(
                Box::new(shared.clone()),
                width,
                FRAMES,
                control,
                (0..lanes).map(|_| None).collect(),
                latency,
            )
            .expect("console bank stage");
            stages.push(Box::new(stage) as Box<dyn BankStage>);
            slots.push(Slot {
                fx: *fx,
                bank: shared,
                producers,
            });
        }
        stages.push(Box::new(Fader));
        let mut chain = BankChain::new(
            AoSoaScratch::new(width, FRAMES).expect("scratch"),
            vec![true; lanes].into_boxed_slice(),
            stages
                .into_iter()
                .map(|stage| BankSlot {
                    stage,
                    active_lanes: vec![true; lanes].into_boxed_slice(),
                })
                .collect(),
        )
        .expect("chain");
        chain.arm_mono_collapse(true);
        chain.force_mono_collapse_off(forced_dual);
        assert!(chain.can_collapse(), "every slot has a one-plane body");
        Some(Self {
            chain,
            slots,
            lanes,
        })
    }
}

/// The SDK's EQ owner, reduced to what shapes the target stream: a per-lane candidate, one
/// transaction per edit, and the factory's own target design (which coalesces a section whose two
/// channels agree into one `Both` target, `fill_targets`).
struct EqOwner {
    candidate: Vec<Vec<InitialParameterValue>>,
    factory: Arc<dyn NativeEffectFactory>,
}

impl EqOwner {
    fn new(lanes: usize) -> Self {
        Self {
            candidate: vec![initial_values(Fx::Eq); lanes],
            factory: factory(EQ),
        }
    }

    fn edit(
        &mut self,
        lane: usize,
        parameter: u32,
        channel: ParameterChannel,
        value: f32,
    ) -> Vec<PreparedEffectTarget> {
        let values = &mut self.candidate[lane];
        let mut changed = vec![false; values.len()];
        for (row, item) in values.iter_mut().enumerate() {
            let addressed = item.parameter_index == parameter
                && (channel == ParameterChannel::Both || item.channel == channel);
            if addressed {
                item.value = value;
                changed[row] = true;
            }
        }
        assert!(changed.iter().any(|row| *row), "the edit addressed a row");
        let preparation = self
            .factory
            .target_preparation()
            .expect("the EQ prepares targets");
        let mut out = vec![
            PreparedEffectTarget {
                slot: 0,
                channel: ParameterChannel::Both,
                words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
            };
            preparation.maximum_targets()
        ];
        let count = preparation
            .prepare_targets(
                EffectTargetRequest {
                    sample_rate: RATE,
                    values,
                    changed: &changed,
                },
                &mut out,
            )
            .expect("an in-domain EQ edit designs");
        out.truncate(count);
        out
    }
}

/// One thing a scenario does between two blocks, to both rigs alike.
#[derive(Clone, Copy, Debug)]
enum Write {
    /// One live-console parameter record into one lane's queue.
    Record {
        slot: usize,
        lane: usize,
        record: EffectControlRecord,
    },
    /// A record the producer must refuse because the queue is full.
    Refused {
        slot: usize,
        lane: usize,
        record: EffectControlRecord,
    },
    /// One EQ owner transaction on one lane, designed into targets as the SDK does.
    EqEdit {
        slot: usize,
        lane: usize,
        parameter: u32,
        channel: ParameterChannel,
        value: f32,
    },
    /// Reset every lane of one slot's bank.
    Reset { slot: usize, kind: ResetKind },
    /// Capture every lane's payload of one slot from the forced-dual reference.
    Capture { slot: usize },
    /// Restore the captured payloads into every lane of one slot, in both rigs.
    Restore { slot: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Expect {
    /// The collapse holds on every block.
    Kept,
    /// The collapse holds before this block and never from it on.
    RetiredAt(u64),
}

struct Scenario {
    name: String,
    blocks: u64,
    writes: Vec<(u64, Write)>,
    hostile: Vec<u64>,
    expect: Expect,
    /// The writes genuinely move the two channels apart, so the reference's own left and right
    /// state must diverge: the retirement was necessary, not merely conservative.
    asymmetric: bool,
}

impl Scenario {
    fn new(name: impl Into<String>, blocks: u64, expect: Expect) -> Self {
        Self {
            name: name.into(),
            blocks,
            writes: Vec::new(),
            hostile: Vec::new(),
            expect,
            asymmetric: false,
        }
    }
    fn asymmetric(mut self) -> Self {
        self.asymmetric = true;
        self
    }
    fn at(&mut self, block: u64, write: Write) -> &mut Self {
        self.writes.push((block, write));
        self
    }
}

fn record(
    slot: usize,
    lane: usize,
    parameter: u32,
    channel: ParameterChannel,
    value: f32,
) -> Write {
    Write::Record {
        slot,
        lane,
        record: EffectControlRecord::Parameter {
            parameter_index: parameter,
            channel,
            value,
        },
    }
}

/// A both-channel write in the product's shape: one `Both` owner edit on the EQ, a `Left` then a
/// `Right` record on the compressor and the limiter (`into_effect_records`'s `(PerLane, 2)` arm).
fn both_channel(
    scenario: &mut Scenario,
    block: u64,
    slot: usize,
    fx: Fx,
    lane: usize,
    parameter: u32,
    value: f32,
) {
    if fx == Fx::Eq {
        scenario.at(
            block,
            Write::EqEdit {
                slot,
                lane,
                parameter,
                channel: ParameterChannel::Both,
                value,
            },
        );
    } else {
        scenario.at(
            block,
            record(slot, lane, parameter, ParameterChannel::Left, value),
        );
        scenario.at(
            block,
            record(slot, lane, parameter, ParameterChannel::Right, value),
        );
    }
}

/// A one-channel write in the product's shape.
fn one_channel(
    scenario: &mut Scenario,
    block: u64,
    (slot, fx): (usize, Fx),
    lane: usize,
    parameter: u32,
    channel: ParameterChannel,
    value: f32,
) {
    if fx == Fx::Eq {
        scenario.at(
            block,
            Write::EqEdit {
                slot,
                lane,
                parameter,
                channel,
                value,
            },
        );
    } else {
        scenario.at(block, record(slot, lane, parameter, channel, value));
    }
}

fn next_up(value: f32) -> f32 {
    if value >= 0.0 {
        f32::from_bits(value.to_bits() + 1)
    } else {
        f32::from_bits(value.to_bits() - 1)
    }
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0 >> 33
    }
    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }
    fn within(&mut self, low: f32, high: f32) -> f32 {
        low + (high - low) * (self.below(1 << 20) as f32 / (1 << 20) as f32)
    }
}

/// Every scenario of gate 1 and amendment A4 that applies to `fxs`, for a bank of `lanes` lanes.
fn scenarios(fxs: &[Fx], lanes: usize) -> Vec<Scenario> {
    const WRITE: u64 = 5;
    let label = fxs
        .iter()
        .map(|fx| fx.id().trim_start_matches("miso."))
        .collect::<Vec<_>>()
        .join("+");
    let mut out = Vec::new();
    let has = |fx: Fx| fxs.iter().position(|candidate| *candidate == fx);
    let spans_slot = has(Fx::Compressor).or(has(Fx::Limiter));

    // Random both-channel rides on several lanes and every slot: the collapse must hold on every
    // block, whatever moves.
    for (seed, hostile) in [(1_u64, false), (2, false), (3, true)] {
        let mut scenario = Scenario::new(
            format!("{label}: random both-channel writes, seed {seed}, hostile {hostile}"),
            24,
            Expect::Kept,
        );
        let mut rng = Lcg(seed);
        for block in 1..24 {
            for _ in 0..rng.below(4) {
                let slot = rng.below(fxs.len() as u64) as usize;
                let fx = fxs[slot];
                let lane = rng.below(lanes as u64) as usize;
                let (parameter, low, high) =
                    fx.rides()[rng.below(fx.rides().len() as u64) as usize];
                let value = rng.within(low, high);
                both_channel(&mut scenario, block, slot, fx, lane, parameter, value);
            }
            if hostile && block % 3 == 0 {
                scenario.hostile.push(block);
            }
        }
        out.push(scenario);
    }

    // A restatement of one value on every lane, every block.
    let mut scenario = Scenario::new(format!("{label}: restatement"), 12, Expect::Kept);
    for block in 1..12 {
        for (slot, fx) in fxs.iter().enumerate() {
            let (parameter, value, _) = fx.rides()[0];
            for lane in 0..lanes {
                both_channel(&mut scenario, block, slot, *fx, lane, parameter, value);
            }
        }
    }
    out.push(scenario);

    for (slot, fx) in fxs.iter().copied().enumerate() {
        let (parameter, low, high) = fx.rides()[0];
        let (other, other_low, _) = fx.rides()[1];
        // Not the midpoint: that is the compressor's held threshold, and a write that restates
        // the held value cannot move the channels apart.
        let value = low + 0.3 * (high - low);

        let mut scenario = Scenario::new(
            format!("{label}: left-only write on slot {slot}"),
            12,
            Expect::RetiredAt(WRITE),
        )
        .asymmetric();
        one_channel(
            &mut scenario,
            WRITE,
            (slot, fx),
            2 % lanes,
            parameter,
            ParameterChannel::Left,
            value,
        );
        out.push(scenario);

        let mut scenario = Scenario::new(
            format!("{label}: right-only write on slot {slot}, hostile audio"),
            12,
            Expect::RetiredAt(WRITE),
        )
        .asymmetric();
        one_channel(
            &mut scenario,
            WRITE,
            (slot, fx),
            1,
            parameter,
            ParameterChannel::Right,
            value,
        );
        scenario.hostile = vec![WRITE - 1, WRITE, WRITE + 2];
        out.push(scenario);

        let mut scenario = Scenario::new(
            format!("{label}: values one ulp apart on slot {slot}"),
            12,
            Expect::RetiredAt(WRITE),
        )
        .asymmetric();
        both_channel(&mut scenario, WRITE - 2, slot, fx, 3, parameter, value);
        one_channel(
            &mut scenario,
            WRITE,
            (slot, fx),
            3,
            parameter,
            ParameterChannel::Left,
            value,
        );
        one_channel(
            &mut scenario,
            WRITE,
            (slot, fx),
            3,
            parameter,
            ParameterChannel::Right,
            next_up(value),
        );
        out.push(scenario);

        let mut scenario = Scenario::new(
            format!(
                "{label}: one channel written twice, the other once to the first value, slot {slot}"
            ),
            12,
            Expect::RetiredAt(WRITE),
        )
        .asymmetric();
        for (channel, written) in [
            (ParameterChannel::Left, low),
            (ParameterChannel::Left, high),
            (ParameterChannel::Right, low),
        ] {
            one_channel(
                &mut scenario,
                WRITE,
                (slot, fx),
                0,
                parameter,
                channel,
                written,
            );
        }
        out.push(scenario);

        let mut scenario = Scenario::new(
            format!("{label}: a pair split across two drains, slot {slot}"),
            12,
            Expect::RetiredAt(WRITE),
        )
        .asymmetric();
        one_channel(
            &mut scenario,
            WRITE,
            (slot, fx),
            0,
            parameter,
            ParameterChannel::Left,
            value,
        );
        one_channel(
            &mut scenario,
            WRITE + 1,
            (slot, fx),
            0,
            parameter,
            ParameterChannel::Right,
            value,
        );
        out.push(scenario);

        let mut scenario = Scenario::new(
            format!("{label}: a pair whose second half never arrives, slot {slot}"),
            12,
            Expect::RetiredAt(WRITE),
        )
        .asymmetric();
        both_channel(&mut scenario, WRITE - 2, slot, fx, 0, parameter, low);
        both_channel(&mut scenario, WRITE - 1, slot, fx, 0, parameter, high);
        one_channel(
            &mut scenario,
            WRITE,
            (slot, fx),
            0,
            parameter,
            ParameterChannel::Left,
            value,
        );
        out.push(scenario);

        let mut scenario = Scenario::new(
            format!("{label}: a pair plus an unpaired write of another parameter, slot {slot}"),
            12,
            Expect::RetiredAt(WRITE),
        )
        .asymmetric();
        both_channel(&mut scenario, WRITE, slot, fx, 1, parameter, value);
        one_channel(
            &mut scenario,
            WRITE,
            (slot, fx),
            1,
            other,
            ParameterChannel::Left,
            other_low,
        );
        out.push(scenario);

        if fx == Fx::Eq {
            // VERIFY-AUTOMATION F1, through the real EQ owner: left-only 4.5 dB, both 1.5 dB,
            // right-only 4.5 dB in one drain. The FIFO is `[L A, Both C, R A]`, and it leaves the
            // channels at C and A: it must retire, and it must render the dual bits.
            let mut scenario = Scenario::new(
                format!("{label}: the F1 target FIFO on slot {slot}"),
                12,
                Expect::RetiredAt(WRITE),
            )
            .asymmetric();
            for (channel, gain) in [
                (ParameterChannel::Left, 4.5),
                (ParameterChannel::Both, 1.5),
                (ParameterChannel::Right, 4.5),
            ] {
                one_channel(&mut scenario, WRITE, (slot, fx), 2, EQ_GAIN, channel, gain);
            }
            out.push(scenario);

            // The same symmetric edit made the harness's way, one owner transaction per channel:
            // two one-channel targets with equal words. The channels end equal, and the rule
            // (spans only) still retires: targets are never deferred.
            let mut scenario = Scenario::new(
                format!("{label}: per-channel EQ owner edits on slot {slot}"),
                12,
                Expect::RetiredAt(WRITE),
            );
            for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                one_channel(&mut scenario, WRITE, (slot, fx), 2, EQ_GAIN, channel, 4.5);
            }
            out.push(scenario);
        } else {
            // Last-wins staging decides the pair: `[L v1, L v2, R v2]` and `[L v1, R v2, L v2]`.
            let mut scenario = Scenario::new(
                format!("{label}: last-wins pairs on slot {slot}"),
                12,
                Expect::Kept,
            );
            for (channel, written) in [
                (ParameterChannel::Left, low),
                (ParameterChannel::Left, high),
                (ParameterChannel::Right, high),
            ] {
                scenario.at(WRITE, record(slot, 0, parameter, channel, written));
            }
            for (channel, written) in [
                (ParameterChannel::Left, low),
                (ParameterChannel::Right, high),
                (ParameterChannel::Left, high),
            ] {
                scenario.at(WRITE + 2, record(slot, 1, parameter, channel, written));
            }
            out.push(scenario);

            // A lone `Left` on one parameter and a lone `Right` on the next, carrying one value:
            // the two spans are adjacent in the sorted window (`(p, Left) < (p + 1, Right)`) and
            // are not a pair.
            let mut scenario = Scenario::new(
                format!("{label}: a Left on one parameter and a Right on the next, slot {slot}"),
                12,
                Expect::RetiredAt(WRITE),
            )
            .asymmetric();
            scenario.at(
                WRITE,
                record(slot, 2, parameter, ParameterChannel::Left, value),
            );
            scenario.at(
                WRITE,
                record(slot, 2, parameter + 1, ParameterChannel::Right, value),
            );
            out.push(scenario);

            // A `Both` span on a `PerLane` parameter is refused on both channels by every launch
            // effect, so it needs no twin and moves nothing; the pair beside it is kept.
            let mut scenario = Scenario::new(
                format!("{label}: a Both span beside a pair on slot {slot}"),
                12,
                Expect::Kept,
            );
            scenario.at(
                WRITE,
                record(slot, 0, other, ParameterChannel::Both, other_low),
            );
            both_channel(&mut scenario, WRITE, slot, fx, 0, parameter, value);
            out.push(scenario);
        }

        // Reset and restore in the middle of a pair: both are symmetric operations on channels
        // that agree, and the pair is still one drain's pair.
        let mut scenario = Scenario::new(
            format!("{label}: reset and restore in the middle of a pair, slot {slot}"),
            14,
            Expect::Kept,
        );
        scenario.at(3, Write::Capture { slot });
        for (block, middle) in [
            (
                WRITE,
                Write::Reset {
                    slot,
                    kind: ResetKind::DiscontinuityKeepParameters,
                },
            ),
            (
                WRITE + 2,
                Write::Reset {
                    slot,
                    kind: ResetKind::FullToDefaults,
                },
            ),
            (WRITE + 4, Write::Restore { slot }),
        ] {
            if fx == Fx::Eq {
                scenario.at(block, middle);
                both_channel(&mut scenario, block, slot, fx, 0, parameter, value);
            } else {
                scenario.at(
                    block,
                    record(slot, 0, parameter, ParameterChannel::Left, low),
                );
                scenario.at(block, middle);
                scenario.at(
                    block,
                    record(slot, 0, parameter, ParameterChannel::Right, low),
                );
            }
        }
        out.push(scenario);
    }

    if let Some(slot) = spans_slot {
        let fx = fxs[slot];
        let descriptor = factory(fx.id()).descriptor();
        let parameters = descriptor
            .parameters
            .iter()
            .enumerate()
            .filter(|(_, parameter)| parameter.automatable)
            .map(|(index, parameter)| (index as u32, parameter.default_value))
            .collect::<Vec<_>>();
        // A queue at capacity: sixteen records of pairs fill it, the seventeenth is refused at
        // the producer, and the drain pairs everything that was admitted.
        let mut scenario = Scenario::new(
            format!("{label}: a queue at capacity holding only pairs"),
            10,
            Expect::Kept,
        );
        for pair in 0..QUEUE / 2 {
            let (parameter, value) = parameters[pair % parameters.len()];
            both_channel(&mut scenario, WRITE, slot, fx, 0, parameter, value);
        }
        scenario.at(
            WRITE,
            Write::Refused {
                slot,
                lane: 0,
                record: EffectControlRecord::Parameter {
                    parameter_index: 0,
                    channel: ParameterChannel::Left,
                    value: parameters[0].1,
                },
            },
        );
        out.push(scenario);

        // A queue at capacity that refuses the second half of a pair: the drain sees a lone
        // write and retires. The refusal is at the producer, before anything is acknowledged.
        let mut scenario = Scenario::new(
            format!("{label}: a queue at capacity refusing the second half of a pair"),
            10,
            Expect::RetiredAt(WRITE),
        )
        .asymmetric();
        for pair in 0..QUEUE / 2 - 1 {
            let (parameter, value) = parameters[pair % parameters.len()];
            both_channel(&mut scenario, WRITE, slot, fx, 0, parameter, value);
        }
        let (parameter, _) = parameters[0];
        let (low, high) = (fx.rides()[0].1, fx.rides()[0].2);
        let moved = if parameters[0].1 == low { high } else { low };
        scenario.at(
            WRITE,
            record(slot, 0, parameter, ParameterChannel::Both, moved),
        );
        scenario.at(
            WRITE,
            record(slot, 0, parameter, ParameterChannel::Left, moved),
        );
        scenario.at(
            WRITE,
            Write::Refused {
                slot,
                lane: 0,
                record: EffectControlRecord::Parameter {
                    parameter_index: parameter,
                    channel: ParameterChannel::Right,
                    value: moved,
                },
            },
        );
        out.push(scenario);
    }
    out
}

/// What one rendered scenario did: the blocks the collapsing rig collapsed on.
struct Outcome {
    collapsed: Vec<bool>,
    diverged: bool,
}

/// Renders `scenario` on a collapsing rig and a forced-dual rig and compares them after every
/// block. Returns `None` where this build binds no bank of `fxs` at `width`.
fn differential(fxs: &[Fx], width: BankWidth, scenario: &Scenario) -> Option<Outcome> {
    let mut collapsing = Rig::new(fxs, width, false)?;
    let mut reference = Rig::new(fxs, width, true)?;
    let lanes = collapsing.lanes;
    let mut owners: Vec<EqOwner> = fxs.iter().map(|_| EqOwner::new(lanes)).collect();
    let mut captured: BTreeMap<usize, Vec<Payload>> = BTreeMap::new();
    let mut collapsed = Vec::new();
    let mut diverged = false;
    let _fp = lane::CanonicalFpEnv::enter();
    for block in 0..scenario.blocks {
        for (_, write) in scenario.writes.iter().filter(|(at, _)| *at == block) {
            match *write {
                Write::Record { slot, lane, record } => {
                    for rig in [&mut collapsing, &mut reference] {
                        rig.slots[slot].producers[lane]
                            .try_push(record)
                            .unwrap_or_else(|_| {
                                panic!("{}: block {block}: queue full", scenario.name)
                            });
                    }
                }
                Write::Refused { slot, lane, record } => {
                    for rig in [&mut collapsing, &mut reference] {
                        assert!(
                            rig.slots[slot].producers[lane].try_push(record).is_err(),
                            "{}: the queue must be at capacity",
                            scenario.name
                        );
                    }
                }
                Write::EqEdit {
                    slot,
                    lane,
                    parameter,
                    channel,
                    value,
                } => {
                    for target in owners[slot].edit(lane, parameter, channel, value) {
                        for rig in [&mut collapsing, &mut reference] {
                            rig.slots[slot].producers[lane]
                                .try_push(EffectControlRecord::PreparedTarget(target))
                                .unwrap_or_else(|_| {
                                    panic!("{}: block {block}: queue full", scenario.name)
                                });
                        }
                    }
                }
                Write::Reset { slot, kind } => {
                    for rig in [&mut collapsing, &mut reference] {
                        rig.slots[slot].bank.clone().reset(kind);
                    }
                }
                Write::Capture { slot } => {
                    let payloads = (0..lanes)
                        .map(|lane| reference.slots[slot].bank.snapshot(lane as u32))
                        .collect::<Vec<_>>();
                    for (lane, (_, left, right)) in payloads.iter().enumerate() {
                        assert_eq!(
                            left, right,
                            "{}: lane {lane}: a captured payload must be symmetric",
                            scenario.name
                        );
                    }
                    captured.insert(slot, payloads);
                }
                Write::Restore { slot } => {
                    let payloads = captured.get(&slot).expect("captured before restore");
                    for rig in [&collapsing, &reference] {
                        for (lane, payload) in payloads.iter().enumerate() {
                            rig.slots[slot].bank.restore(lane as u32, payload);
                        }
                    }
                }
            }
        }

        let hostile = scenario.hostile.contains(&block);
        let mut collapsing_planes = input(lanes, block, hostile);
        let mut reference_planes = collapsing_planes.clone();
        let before = collapsing.chain.collapses();
        let first_sample = block * u64::from(FRAMES);
        collapsing
            .chain
            .run(&mut collapsing_planes, FRAMES, first_sample)
            .expect("collapsing render");
        reference
            .chain
            .run(&mut reference_planes, FRAMES, first_sample)
            .expect("reference render");
        let mono = collapsing.chain.collapses() > before;
        collapsed.push(mono);
        assert_eq!(
            reference.chain.collapses(),
            0,
            "{}: the reference never collapses",
            scenario.name
        );

        // Every output word, by bits: the bank output is after each effect's own non-finite
        // check, so no NaN relaxation applies.
        for lane in 0..lanes {
            for (plane, (ours, theirs)) in [
                (
                    "left",
                    (&collapsing_planes.left[lane], &reference_planes.left[lane]),
                ),
                (
                    "right",
                    (
                        &collapsing_planes.right[lane],
                        &reference_planes.right[lane],
                    ),
                ),
            ] {
                let first = ours
                    .iter()
                    .zip(theirs.iter())
                    .position(|(a, b)| a.to_bits() != b.to_bits());
                assert!(
                    first.is_none(),
                    "{}: {:?}: block {block} lane {lane} {plane} frame {first:?}: the collapse \
                     rendered {:?}, the dual reference {:?} (collapsed: {mono})",
                    scenario.name,
                    width,
                    first.map(|frame| ours[frame]),
                    first.map(|frame| theirs[frame]),
                );
            }
        }

        for (slot, (ours, theirs)) in collapsing
            .slots
            .iter()
            .zip(reference.slots.iter())
            .enumerate()
        {
            // Every report, as the dual body reports it: a collapsed body duplicates its right
            // counters from the left.
            assert_eq!(
                ours.bank.take_report(),
                theirs.bank.take_report(),
                "{}: {width:?}: block {block} slot {slot} ({}): bank report",
                scenario.name,
                ours.fx.id()
            );
            // Every lane's whole state payload, by bytes. A collapsed bank's right channel is
            // frozen by contract, so on a collapsed block the claim is that the dual reference's
            // two channels agree and its left is ours.
            for lane in 0..lanes {
                let (common, left, right) = ours.bank.snapshot(lane as u32);
                let (dual_common, dual_left, dual_right) = theirs.bank.snapshot(lane as u32);
                let label = format!(
                    "{}: {width:?}: block {block} slot {slot} ({}) lane {lane}",
                    scenario.name,
                    ours.fx.id()
                );
                assert!(common == dual_common, "{label}: common state");
                assert!(left == dual_left, "{label}: left state");
                if mono {
                    assert!(
                        dual_left == dual_right,
                        "{label}: collapsed while the dual reference's channels disagree"
                    );
                } else {
                    assert!(right == dual_right, "{label}: right state");
                }
                if dual_left != dual_right {
                    diverged = true;
                }
            }
        }
    }
    Some(Outcome {
        collapsed,
        diverged,
    })
}

fn check(fxs: &[Fx], width: BankWidth, scenario: &Scenario) -> bool {
    let Some(outcome) = differential(fxs, width, scenario) else {
        return false;
    };
    let expected: Vec<bool> = (0..scenario.blocks)
        .map(|block| match scenario.expect {
            Expect::Kept => true,
            Expect::RetiredAt(at) => block < at,
        })
        .collect();
    assert_eq!(
        outcome.collapsed, expected,
        "{}: {width:?}: the collapse must be kept exactly where the rule keeps it",
        scenario.name
    );
    if scenario.asymmetric {
        assert!(
            outcome.diverged,
            "{}: {width:?}: the writes must actually move the channels apart, or a wrong keep \
             could not show",
            scenario.name
        );
    }
    true
}

/// Runs every scenario of `fxs` at both launch widths. The native width must bind; the other
/// binds where this build can (the limiter binds four lanes on an eight-lane host; the EQ and the
/// compressor do not, decision D4).
fn run_all(fxs: &[Fx]) {
    let native = BankWidth::for_backend(Backend::current()).expect("a SIMD backend");
    let mut ran = Vec::new();
    for width in [BankWidth::Eight, BankWidth::Four] {
        let lanes = width.lanes() as usize;
        let mut count = 0;
        for scenario in scenarios(fxs, lanes) {
            if check(fxs, width, &scenario) {
                count += 1;
            }
        }
        if count > 0 {
            ran.push((width, count));
        }
    }
    assert!(
        ran.iter().any(|(width, _)| *width == native),
        "{fxs:?}: the native width must bind: {ran:?}"
    );
    eprintln!("{fxs:?}: scenarios per width {ran:?}");
}

#[test]
fn the_eq_bank_renders_the_dual_bits_in_every_scenario() {
    run_all(&[Fx::Eq]);
}

#[test]
fn the_compressor_bank_renders_the_dual_bits_in_every_scenario() {
    run_all(&[Fx::Compressor]);
}

#[test]
fn the_limiter_bank_renders_the_dual_bits_in_every_scenario() {
    run_all(&[Fx::Limiter]);
}

#[test]
fn the_chained_strip_renders_the_dual_bits_in_every_scenario() {
    run_all(&[Fx::Eq, Fx::Compressor, Fx::Limiter]);
}

/// Every order of a mixed drain on the chained strip: one-channel, `Both` and paired writes in
/// one drain, across the EQ's target FIFO and the compressor's and the limiter's spans.
///
/// The expected answer is computed from the records, independently of the drain: a drain keeps
/// the collapse exactly when it carries no one-channel target and every one-channel parameter
/// write's final value is matched, by bits, by the other channel's final value.
#[test]
fn every_order_of_a_mixed_drain_keeps_the_collapse_exactly_where_the_rule_does() {
    const WRITE: u64 = 3;
    let fxs = [Fx::Eq, Fx::Compressor, Fx::Limiter];
    let width = BankWidth::for_backend(Backend::current()).expect("a SIMD backend");
    // Each atom is one record (or one EQ owner transaction) on lane 1.
    let span_sets: [&[(usize, u32, ParameterChannel, f32)]; 4] = [
        // A pair, a lone Left on another parameter, and a Both.
        &[
            (1, COMPRESSOR_THRESHOLD, ParameterChannel::Left, -24.0),
            (1, COMPRESSOR_THRESHOLD, ParameterChannel::Right, -24.0),
            (2, LIMITER_CEILING, ParameterChannel::Left, -12.0),
            (1, COMPRESSOR_RATIO, ParameterChannel::Both, 6.0),
        ],
        // Last-wins: L v1, L v2, R v2 on one parameter, and a Both elsewhere.
        &[
            (1, COMPRESSOR_THRESHOLD, ParameterChannel::Left, -24.0),
            (1, COMPRESSOR_THRESHOLD, ParameterChannel::Left, -20.0),
            (1, COMPRESSOR_THRESHOLD, ParameterChannel::Right, -20.0),
            (2, LIMITER_RELEASE, ParameterChannel::Both, 90.0),
        ],
        // Two pairs on two effects.
        &[
            (1, COMPRESSOR_ATTACK, ParameterChannel::Left, 5.0),
            (2, LIMITER_CEILING, ParameterChannel::Right, -12.0),
            (1, COMPRESSOR_ATTACK, ParameterChannel::Right, 5.0),
            (2, LIMITER_CEILING, ParameterChannel::Left, -12.0),
        ],
        // The F1 shape, in spans.
        &[
            (2, LIMITER_CEILING, ParameterChannel::Left, -12.0),
            (2, LIMITER_CEILING, ParameterChannel::Both, -6.0),
            (2, LIMITER_CEILING, ParameterChannel::Right, -12.0),
        ],
    ];
    let eq_sets: [&[(ParameterChannel, f32)]; 3] = [
        &[],
        &[(ParameterChannel::Both, 1.5)],
        // VERIFY-AUTOMATION F1: left 4.5, both 1.5, right 4.5.
        &[
            (ParameterChannel::Left, 4.5),
            (ParameterChannel::Both, 1.5),
            (ParameterChannel::Right, 4.5),
        ],
    ];
    let mut kept = 0;
    let mut retired = 0;
    for spans in span_sets {
        for eq in eq_sets {
            for span_order in permutations(spans) {
                for eq_order in permutations(eq) {
                    let mut scenario = Scenario::new(
                        format!("mixed drain: spans {span_order:?}, EQ {eq_order:?}"),
                        6,
                        Expect::Kept,
                    );
                    // The EQ's records interleave the spans' in the lane order the queue sees,
                    // but each effect has its own queue, so only the per-queue order matters.
                    let mut owner = EqOwner::new(width.lanes() as usize);
                    let mut one_channel_target = false;
                    for (channel, gain) in &eq_order {
                        for target in owner.edit(1, EQ_GAIN, *channel, *gain) {
                            one_channel_target |= target.channel.writes_one_channel();
                        }
                        scenario.at(
                            WRITE,
                            Write::EqEdit {
                                slot: 0,
                                lane: 1,
                                parameter: EQ_GAIN,
                                channel: *channel,
                                value: *gain,
                            },
                        );
                    }
                    for (slot, parameter, channel, value) in &span_order {
                        scenario.at(WRITE, record(*slot, 1, *parameter, *channel, *value));
                    }
                    let keeps = !one_channel_target
                        && (1..=2).all(|slot| {
                            model_pairs(
                                span_order
                                    .iter()
                                    .filter(|atom| atom.0 == slot)
                                    .map(|atom| (atom.1, atom.2, atom.3)),
                            )
                        });
                    scenario.expect = if keeps {
                        kept += 1;
                        Expect::Kept
                    } else {
                        retired += 1;
                        Expect::RetiredAt(WRITE)
                    };
                    assert!(check(&fxs, width, &scenario), "the native width binds");
                }
            }
        }
    }
    eprintln!("mixed drains: {kept} kept, {retired} retired");
    assert!(kept > 0 && retired > 0);
}

/// The model the mixed-drain test predicts with: final value per `(parameter, channel)`, and
/// every one-channel final value matched by the other channel's.
fn model_pairs(atoms: impl Iterator<Item = (u32, ParameterChannel, f32)>) -> bool {
    let mut last: BTreeMap<(u32, u32), u32> = BTreeMap::new();
    for (parameter, channel, value) in atoms {
        last.insert((parameter, channel as u32), value.to_bits());
    }
    last.iter().all(|((parameter, channel), bits)| {
        let twin = match *channel {
            1 => 2,
            2 => 1,
            _ => return true,
        };
        last.get(&(*parameter, twin)) == Some(bits)
    })
}

fn permutations<T: Copy>(atoms: &[T]) -> Vec<Vec<T>> {
    if atoms.len() <= 1 {
        return vec![atoms.to_vec()];
    }
    let mut out = Vec::new();
    for index in 0..atoms.len() {
        let mut rest = atoms.to_vec();
        let atom = rest.remove(index);
        for mut tail in permutations(&rest) {
            tail.insert(0, atom);
            out.push(tail);
        }
    }
    out
}

/// The set of effects that can sit in a collapse-eligible cohort is exactly [`Fx`], and none of
/// them declares a `Shared` automatable parameter.
///
/// The first half keeps the differential's coverage honest: an effect that gains a one-plane body
/// joins the collapse, and with it the pairing rule's obligation, so it has to join [`Fx`] and the
/// scenarios above. The second half is why a `Shared` parameter needs no scenario on the collapsed
/// path: none of the three has one. (A `Shared` parameter's `Left` or `Right` span is refused on
/// both channels by every launch effect, so a twin on one would move nothing either way; the
/// drain-level suite covers the `Both` interleavings.)
#[test]
fn every_collapse_capable_launch_effect_is_one_of_these() {
    let width = BankWidth::for_backend(Backend::current()).expect("a SIMD backend");
    let registry = registry();
    let mut capable = Vec::new();
    for descriptor in registry.descriptors() {
        let factory = registry.get(descriptor.id).expect("registered");
        let mut values: Vec<InitialParameterValue> = default_initial_values(descriptor).collect();
        if descriptor.id.as_str() == EQ {
            values = initial_values(Fx::Eq);
        }
        let mode = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average]
            .into_iter()
            .find(|mode| descriptor.supported_link_modes.contains(*mode))
            .expect("a link mode");
        let Some(request) = prepare_request(descriptor, &values, mode) else {
            continue;
        };
        let requests = vec![request; width.lanes() as usize];
        let Ok(Some(bank)) = factory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: Backend::current(),
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        }) else {
            continue;
        };
        if bank.supports_mono_collapse() {
            capable.push(descriptor.id.as_str().to_owned());
            assert!(
                descriptor
                    .parameters
                    .iter()
                    .filter(|parameter| parameter.automatable)
                    .all(|parameter| parameter.channel_policy == ParameterChannelPolicy::PerLane),
                "{}: a collapse-capable effect with a Shared automatable parameter needs a \
                 pairing review",
                descriptor.id.as_str()
            );
        }
    }
    capable.sort();
    let mut expected = vec![EQ.to_owned(), COMPRESSOR.to_owned(), LIMITER.to_owned()];
    expected.sort();
    assert_eq!(capable, expected);
}

// =============================================================================================
// Part 2: the per-effect obligation (amendment A3)
// =============================================================================================

/// A prepared effect under the twin test: a bank at the native width, or the scalar instance a
/// session renders per node where the effect binds no bank here.
enum Subject {
    Bank(Box<dyn PreparedNativeEffectBank>, BankWidth),
    Scalar(Box<dyn PreparedNativeEffect>),
}

impl Subject {
    fn lanes(&self) -> usize {
        match self {
            Self::Bank(_, width) => width.lanes() as usize,
            Self::Scalar(_) => 1,
        }
    }

    fn sizes(&self) -> StatePayloadSizes {
        match self {
            Self::Bank(bank, _) => bank.metadata().program_key.state_sizes,
            Self::Scalar(effect) => effect.metadata().state_sizes,
        }
    }

    /// Renders one block of mono input; `spans` go to lane 0 only.
    fn render(&mut self, block: u64, spans: &[PreparedAutomationSpan]) -> u64 {
        let lanes = self.lanes();
        let planes = input(lanes, block, false);
        let first_sample = block * u64::from(FRAMES);
        match self {
            Self::Bank(bank, width) => {
                let mut left = vec![0.0_f32; FRAMES as usize * lanes];
                for (lane, plane) in planes.left.iter().enumerate() {
                    for (frame, sample) in plane.iter().enumerate() {
                        left[frame * lanes + lane] = *sample;
                    }
                }
                let mut right = left.clone();
                let mut offsets = vec![spans.len() as u32; lanes + 1];
                offsets[0] = 0;
                let report = bank.process_bank(
                    EffectBankProcessBlock::new(
                        &mut left,
                        &mut right,
                        None,
                        FRAMES,
                        *width,
                        first_sample,
                        spans,
                        &offsets,
                        FRAMES,
                    )
                    .expect("bank block"),
                );
                report.reports[0].invalid_spans
            }
            Self::Scalar(effect) => {
                let mut left = planes.left[0].clone();
                let mut right = planes.right[0].clone();
                effect
                    .process(
                        EffectProcessBlock::new(
                            &mut left,
                            &mut right,
                            None,
                            first_sample,
                            spans,
                            FRAMES,
                        )
                        .expect("block"),
                    )
                    .invalid_spans
            }
        }
    }

    fn channels_agree(&self) -> Vec<bool> {
        let sizes = self.sizes();
        (0..self.lanes())
            .map(|lane| {
                let mut common = vec![0_u8; sizes.common_bytes as usize];
                let mut left = vec![0_u8; sizes.left_bytes as usize];
                let mut right = vec![0_u8; sizes.right_bytes as usize];
                let output = StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes)
                    .expect("payload sizes");
                match self {
                    Self::Bank(bank, _) => bank.snapshot_track_state_payload(lane as u32, output),
                    Self::Scalar(effect) => effect.snapshot_state_payload(output),
                }
                .expect("snapshot");
                left == right
            })
            .collect()
    }
}

/// The values a twin pair carries for one parameter: its default, its domain edges, a value on
/// each side just past them, and the words a host never sends but a drain would pass through
/// (`NaN`, both infinities, `-0.0`).
fn twin_values(parameter: &ParameterDescriptor) -> Vec<f32> {
    let mut values = vec![parameter.default_value];
    match parameter.domain {
        ParameterDomain::Boolean => values.extend([0.0, 1.0, 0.5, 2.0]),
        ParameterDomain::Enumeration => {
            values.extend((0..=parameter.enum_choices.len() as u32 + 1).map(|value| value as f32));
        }
        ParameterDomain::Continuous => {
            if let (Some(low), Some(high)) = (parameter.minimum, parameter.maximum) {
                let below = f32::from_bits(if low > 0.0 {
                    low.to_bits() - 1
                } else if low == 0.0 {
                    0x8000_0001
                } else {
                    low.to_bits() + 1
                });
                values.extend([low, high, 0.5 * (low + high), below, next_up(high)]);
            }
        }
    }
    values.extend([f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -0.0]);
    values
}

/// Amendment A3: a twin pair leaves bit-equal channels bit-equal, in every launch effect that
/// takes live `PerLane` parameters, at every link mode it supports.
///
/// Each pair goes through a real `EffectControlLane` drain -- the staging the render path hands the
/// effect -- and then one block renders. Accepted, at a domain edge, just past one, or refused as
/// non-finite, the two channels must come out holding the same bytes: the validity terms a pair
/// is judged by (kind, samples, value, parameter, order and capacity) are the same for both
/// halves, and the value is applied by the same code.
#[test]
fn every_launch_effect_applies_a_twin_pair_with_channel_symmetric_validity() {
    let _fp = lane::CanonicalFpEnv::enter();
    let width = BankWidth::for_backend(Backend::current()).expect("a SIMD backend");
    let registry = registry();
    let mut covered = Vec::new();
    for descriptor in registry.descriptors() {
        let factory = registry.get(descriptor.id).expect("registered");
        let live: Vec<(u32, &ParameterDescriptor)> = descriptor
            .parameters
            .iter()
            .enumerate()
            .filter(|(_, parameter)| {
                parameter.automatable
                    && parameter.automation_rate != AutomationRate::None
                    && parameter.channel_policy == ParameterChannelPolicy::PerLane
            })
            .map(|(index, parameter)| (index as u32, parameter))
            .collect();
        if live.is_empty() {
            covered.push(format!(
                "{}: no live PerLane parameter",
                descriptor.id.as_str()
            ));
            continue;
        }
        for mode in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
            if !descriptor.supported_link_modes.contains(mode) {
                continue;
            }
            let values: Vec<InitialParameterValue> = if descriptor.id.as_str() == EQ {
                initial_values(Fx::Eq)
            } else {
                default_initial_values(descriptor).collect()
            };
            let Some(request) = prepare_request(descriptor, &values, mode) else {
                covered.push(format!(
                    "{}: required port, skipped",
                    descriptor.id.as_str()
                ));
                continue;
            };
            let requests = vec![request; width.lanes() as usize];
            let mut subject = match factory.bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: Backend::current(),
                width,
                requests: &requests,
                active_mask: width.full_mask(),
            }) {
                Ok(Some(bank)) => Subject::Bank(bank, width),
                _ => Subject::Scalar(factory.prepare(request).expect("scalar prepare")),
            };
            let kind = match subject {
                Subject::Bank(ref bank, _) => {
                    if bank.supports_mono_collapse() {
                        "bank, collapse-capable"
                    } else {
                        "bank"
                    }
                }
                Subject::Scalar(_) => "scalar",
            };
            let (producer, consumer) = bounded_spsc::<EffectControlRecord>(
                core::num::NonZeroUsize::new(QUEUE).expect("depth"),
                QueueGeneration(0),
            )
            .expect("queue");
            let mut producer = producer;
            let mut control = EffectControlLane::new(consumer, false);
            let mut staging = vec![
                PreparedAutomationSpan {
                    kind: AutomationSpanKind::Point,
                    channel: ParameterChannel::Both,
                    parameter_index: 0,
                    start_sample: 0,
                    end_sample: 0,
                    start_value: 0.0,
                    end_value: 0.0,
                };
                QUEUE
            ];
            let mut block = 0_u64;
            for _ in 0..4 {
                subject.render(block, &[]);
                block += 1;
            }
            let label = format!("{} ({kind}, {mode:?})", descriptor.id.as_str());
            assert!(
                subject.channels_agree().iter().all(|agree| *agree),
                "{label}: mono input must leave the channels equal before any write"
            );
            let mut pairs = 0;
            let mut refused = 0;
            for (index, parameter) in &live {
                for value in twin_values(parameter) {
                    for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                        producer
                            .try_push(EffectControlRecord::Parameter {
                                parameter_index: *index,
                                channel,
                                value,
                            })
                            .expect("room");
                    }
                    let staged = control.stage(&mut staging, block * u64::from(FRAMES), None);
                    assert_eq!(staged.staged, 2);
                    assert!(
                        control.symmetry().holds(ChannelSymmetryWitness::LIVE),
                        "{label}: a twin pair keeps LIVE"
                    );
                    let invalid = subject.render(block, &staging[..staged.staged]);
                    block += 1;
                    assert!(
                        invalid == 0 || invalid == 2,
                        "{label}: parameter {index} value {value:?}: {invalid} of the pair's two \
                         spans refused -- validity must not depend on the channel"
                    );
                    pairs += 1;
                    refused += usize::from(invalid == 2);
                    // One more block so a ramp the pair opened is in flight and then settles.
                    subject.render(block, &[]);
                    block += 1;
                    let agree = subject.channels_agree();
                    assert!(
                        agree.iter().all(|lane| *lane),
                        "{label}: parameter {index} ({}) value {value:?}: the channels came \
                         apart: {agree:?}",
                        parameter.display_name
                    );
                }
            }
            covered.push(format!(
                "{label}: {} live PerLane parameters, {pairs} twin pairs, {refused} refused on \
                 both channels",
                live.len()
            ));
        }
    }
    for line in &covered {
        eprintln!("{line}");
    }
    assert!(
        covered
            .iter()
            .any(|line| line.contains(COMPRESSOR) && line.contains("collapse-capable")),
        "{covered:?}"
    );
    assert!(
        covered
            .iter()
            .any(|line| line.contains(LIMITER) && line.contains("collapse-capable")),
        "{covered:?}"
    );
}

/// Issue #1012: the staging-window bound the pairing rule rests on, refused at preparation for
/// every launch effect, scalar and banked.
///
/// A window of exactly `automation_capacity` spans is accepted; one span more (which could stage a
/// twin across the effect's cut-off) or one span fewer (which would drop admitted records) is
/// refused with the typed `ProcessBlockError::AutomationWindow`.
///
/// Red mutation (issue #1012): make the contract's `check_window` return `Ok(())`
/// unconditionally -> both refusals below fail, for every effect.
#[test]
fn every_launch_effect_refuses_a_staging_window_that_is_not_its_capacity() {
    let width = BankWidth::for_backend(Backend::current()).expect("a SIMD backend");
    let registry = registry();
    let window = |spans: usize| {
        vec![
            PreparedAutomationSpan {
                kind: AutomationSpanKind::Point,
                channel: ParameterChannel::Both,
                parameter_index: 0,
                start_sample: 0,
                end_sample: 0,
                start_value: 0.0,
                end_value: 0.0,
            };
            spans
        ]
    };
    let mut checked = Vec::new();
    for descriptor in registry.descriptors() {
        let factory = registry.get(descriptor.id).expect("registered");
        let values: Vec<InitialParameterValue> = if descriptor.id.as_str() == EQ {
            initial_values(Fx::Eq)
        } else {
            default_initial_values(descriptor).collect()
        };
        let mode = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average]
            .into_iter()
            .find(|mode| descriptor.supported_link_modes.contains(*mode))
            .expect("a link mode");
        let Some(request) = prepare_request(descriptor, &values, mode) else {
            continue;
        };
        let scalar = factory.prepare(request).expect("scalar prepare").metadata();
        let capacity = scalar.automation_capacity as usize;
        assert!(
            capacity > 0,
            "{}: a live effect has capacity",
            descriptor.id.as_str()
        );
        assert_eq!(
            EffectProcessBlock::check_automation_window(&window(capacity), &scalar),
            Ok(())
        );
        for wrong in [capacity + 1, capacity - 1] {
            assert_eq!(
                EffectProcessBlock::check_automation_window(&window(wrong), &scalar),
                Err(ProcessBlockError::AutomationWindow),
                "{}: a scalar window of {wrong} spans against capacity {capacity}",
                descriptor.id.as_str()
            );
        }
        let requests = vec![request; width.lanes() as usize];
        let banked = match factory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: Backend::current(),
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        }) {
            Ok(Some(bank)) => {
                let metadata = bank.metadata();
                let capacity = metadata.program_key.automation_capacity as usize;
                assert_eq!(
                    EffectBankProcessBlock::check_automation_window(&window(capacity), &metadata),
                    Ok(())
                );
                for wrong in [capacity + 1, capacity - 1] {
                    assert_eq!(
                        EffectBankProcessBlock::check_automation_window(&window(wrong), &metadata),
                        Err(ProcessBlockError::AutomationWindow),
                        "{}: a bank window of {wrong} spans against capacity {capacity}",
                        descriptor.id.as_str()
                    );
                }
                true
            }
            _ => false,
        };
        checked.push((descriptor.id.as_str().to_owned(), banked));
    }
    assert_eq!(checked.len(), registry.len(), "{checked:?}");
    assert!(
        checked.iter().filter(|(_, banked)| *banked).count() >= 3,
        "{checked:?}"
    );
}

// =============================================================================================
// Part 3: engagement on the mono console (gate 2)
// =============================================================================================

const CONTROL: PlanConfig = PlanConfig {
    meters: false,
    control: true,
    observation: ObservationArm::Absent,
};
const MONO: Workload = Workload::SixtyFourTrackConsoleMono;

/// The mono console's control channels by `(track id, effect type id)`. The fixture carries an
/// EQ, a compressor and a limiter on each of its 64 tracks, so there are exactly 192.
///
/// `control_identity` names a channel by the session's effect *instance* id; the first track's
/// channel of each type (`first_track_control_channel`) is what maps the instance id back to the
/// type, and the fixture uses one instance id per type on every track.
fn channels(runtime: &SessionRuntime) -> BTreeMap<(String, String), usize> {
    let types: BTreeMap<String, &'static str> = [EQ, COMPRESSOR, LIMITER]
        .into_iter()
        .map(|effect| {
            let channel = runtime
                .first_track_control_channel(effect)
                .unwrap_or_else(|| panic!("the mono console carries {effect}"));
            (runtime.control_identity(channel).1.to_owned(), effect)
        })
        .collect();
    let map: BTreeMap<(String, String), usize> = (0..64 * 3)
        .map(|channel| {
            let (track, instance) = runtime.control_identity(channel);
            let effect = types
                .get(instance)
                .unwrap_or_else(|| panic!("{track}: unexpected effect instance {instance}"));
            ((track.to_owned(), (*effect).to_owned()), channel)
        })
        .collect();
    assert_eq!(map.len(), 64 * 3, "one channel per track and effect");
    map
}

fn track_ids(map: &BTreeMap<(String, String), usize>) -> Vec<String> {
    let mut tracks: Vec<String> = map.keys().map(|(track, _)| track.clone()).collect();
    tracks.dedup();
    assert_eq!(tracks.len(), 64);
    tracks
}

/// A console write in the product's shapes.
#[derive(Clone, Copy, Debug)]
enum Push {
    /// The web host's both-channel command: one `Both` owner edit for the EQ, a `Left` and a
    /// `Right` record for the compressor and the limiter.
    Both(&'static str, u32, f32),
    /// One channel only.
    One(&'static str, u32, ParameterChannel, f32),
}

fn push(runtime: &mut SessionRuntime, channel: usize, write: Push) {
    let ok = match write {
        Push::Both(effect, parameter, value) if effect == EQ => {
            runtime.push_parameter(channel, parameter, ParameterChannel::Both, value)
        }
        Push::Both(_, parameter, value) => {
            runtime.push_parameter(channel, parameter, ParameterChannel::Left, value)
                && runtime.push_parameter(channel, parameter, ParameterChannel::Right, value)
        }
        Push::One(_, parameter, side, value) => {
            runtime.push_parameter(channel, parameter, side, value)
        }
    };
    assert!(ok, "the bounded control queue refused {write:?}");
}

const fn effect_of(write: Push) -> &'static str {
    match write {
        Push::Both(effect, ..) | Push::One(effect, ..) => effect,
    }
}

/// Renders the mono console twice -- collapse armed, and forced dual -- with `writes(block)`
/// pushed into both before each block, and returns the armed run's collapse counters. Every block
/// must render the forced-dual bits.
/// What a console scenario pushes before a block: `(track index, write)` pairs.
type Writes<'a> = &'a dyn Fn(u64, &[String]) -> Vec<(usize, Push)>;

fn console_run(dispatch: Backend, blocks: u64, writes: Writes<'_>) -> ([u64; 2], [u64; 3], String) {
    let mut collapsing = SessionRuntime::build_with_dispatch(MONO, CONTROL, dispatch);
    let mut dual = SessionRuntime::build_with_dispatch(MONO, CONTROL, dispatch);
    dual.force_mono_collapse_off(true);
    let map = channels(&collapsing);
    assert_eq!(map, channels(&dual), "both arms are one session");
    let tracks = track_ids(&map);
    let mut digest = Sha256Sink::new();
    for block in 0..blocks {
        for (track, write) in writes(block, &tracks) {
            let channel = map[&(tracks[track].clone(), effect_of(write).to_owned())];
            push(&mut collapsing, channel, write);
            push(&mut dual, channel, write);
        }
        collapsing.render(block).expect("console render");
        dual.render(block).expect("console render");
        let mut ours = Sha256Sink::new();
        let mut theirs = Sha256Sink::new();
        collapsing.hash_output(&mut ours);
        dual.hash_output(&mut theirs);
        collapsing.hash_output(&mut digest);
        assert_eq!(
            ours.finish_hex(),
            theirs.finish_hex(),
            "block {block}: the collapsed console must render the forced-dual bits"
        );
    }
    assert_eq!(dual.bank_collapse_counters()[0], 0);
    (
        collapsing.bank_collapse_counters(),
        collapsing.bank_collapse_transitions(),
        digest.finish_hex(),
    )
}

/// The diagnosis's 8-of-64 mixed ride: tracks 0, 8, …, 56, each riding one control, three EQ
/// gains, three compressor thresholds and two limiter ceilings, alternating either side of a base
/// every block. The limiter's base is low enough that it engages on this fixture (VERIFY F6).
fn mixed_ride(block: u64) -> Vec<(usize, Push)> {
    let sign = if block.is_multiple_of(2) { 1.0 } else { -1.0 };
    (0..8)
        .map(|index| {
            let track = index * 8;
            let write = match index % 3 {
                0 => Push::Both(EQ, EQ_GAIN, 3.0 + 0.25 * sign),
                1 => Push::Both(COMPRESSOR, COMPRESSOR_THRESHOLD, -24.0 + 0.5 * sign),
                _ => Push::Both(LIMITER, LIMITER_CEILING, -18.0 + 0.25 * sign),
            };
            (track, write)
        })
        .collect()
}

/// Every track's three controls set once, both channels, at the ride's bases.
fn settle(tracks: usize) -> Vec<(usize, Push)> {
    (0..tracks)
        .flat_map(|track| {
            [
                (track, Push::Both(EQ, EQ_GAIN, 3.0)),
                (track, Push::Both(COMPRESSOR, COMPRESSOR_THRESHOLD, -24.0)),
                (track, Push::Both(LIMITER, LIMITER_CEILING, -18.0)),
            ]
        })
        .collect()
}

#[test]
fn a_both_channel_ride_keeps_every_cohort_collapsed_on_every_block() {
    const BLOCKS: u64 = 24;
    let (counters, transitions, _) = console_run(Backend::current(), BLOCKS, &|block, tracks| {
        let mut writes = if block == 0 {
            settle(tracks.len())
        } else {
            Vec::new()
        };
        writes.extend(mixed_ride(block));
        writes
    });
    assert!(counters[1] > 0);
    assert_eq!(
        counters[0],
        BLOCKS * counters[1],
        "every cohort collapses on every block through a both-channel ride"
    );
    assert_eq!(transitions, [0, 0, 0]);
}

#[test]
fn a_restatement_keeps_every_cohort_collapsed() {
    const BLOCKS: u64 = 16;
    let (counters, _, restated) = console_run(Backend::current(), BLOCKS, &|_, tracks| {
        settle(tracks.len())
    });
    assert_eq!(counters[0], BLOCKS * counters[1]);
    // And a restatement is class A against setting the value once: the same bits.
    let (_, _, once) = console_run(Backend::current(), BLOCKS, &|block, tracks| {
        if block == 0 {
            settle(tracks.len())
        } else {
            Vec::new()
        }
    });
    assert_eq!(
        restated, once,
        "restating a held value moves no rendered bit"
    );
}

/// The product's EQ path never needed the rule: the SDK sends a symmetric both-channel EQ edit as
/// one `Both` target, which the witness always preserved. This holds on the base tree as well
/// (VERIFY-AUTOMATION F2), and is here so that stays true.
#[test]
fn an_eq_only_both_channel_ride_keeps_every_cohort_collapsed() {
    const BLOCKS: u64 = 16;
    let (counters, _, _) = console_run(Backend::current(), BLOCKS, &|block, tracks| {
        (0..tracks.len())
            .step_by(3)
            .map(|track| {
                let gain = if block.is_multiple_of(2) { 2.0 } else { -2.0 };
                (track, Push::Both(EQ, EQ_GAIN, gain))
            })
            .collect()
    });
    assert_eq!(counters[0], BLOCKS * counters[1]);
}

/// A one-channel write or a near pair retires exactly its own cohort, on the block it lands.
#[test]
fn a_one_channel_write_or_a_near_pair_retires_exactly_its_cohort() {
    const BLOCKS: u64 = 16;
    const SWITCH: u64 = 6;
    let value = -24.0_f32;
    let cases: [(&str, Vec<(u64, Push)>); 7] = [
        (
            "left-only compressor",
            vec![(
                SWITCH,
                Push::One(
                    COMPRESSOR,
                    COMPRESSOR_THRESHOLD,
                    ParameterChannel::Left,
                    value,
                ),
            )],
        ),
        (
            "left-only EQ owner edit",
            vec![(SWITCH, Push::One(EQ, EQ_GAIN, ParameterChannel::Left, 4.5))],
        ),
        (
            "values one ulp apart",
            vec![
                (
                    SWITCH,
                    Push::One(LIMITER, LIMITER_CEILING, ParameterChannel::Left, -18.0),
                ),
                (
                    SWITCH,
                    Push::One(
                        LIMITER,
                        LIMITER_CEILING,
                        ParameterChannel::Right,
                        next_up(-18.0),
                    ),
                ),
            ],
        ),
        (
            "one channel twice, the other once to the first value",
            vec![
                (
                    SWITCH,
                    Push::One(
                        COMPRESSOR,
                        COMPRESSOR_THRESHOLD,
                        ParameterChannel::Left,
                        value,
                    ),
                ),
                (
                    SWITCH,
                    Push::One(
                        COMPRESSOR,
                        COMPRESSOR_THRESHOLD,
                        ParameterChannel::Left,
                        -20.0,
                    ),
                ),
                (
                    SWITCH,
                    Push::One(
                        COMPRESSOR,
                        COMPRESSOR_THRESHOLD,
                        ParameterChannel::Right,
                        value,
                    ),
                ),
            ],
        ),
        (
            "a pair split across two drains",
            vec![
                (
                    SWITCH,
                    Push::One(
                        COMPRESSOR,
                        COMPRESSOR_THRESHOLD,
                        ParameterChannel::Left,
                        value,
                    ),
                ),
                (
                    SWITCH + 1,
                    Push::One(
                        COMPRESSOR,
                        COMPRESSOR_THRESHOLD,
                        ParameterChannel::Right,
                        value,
                    ),
                ),
            ],
        ),
        (
            "a pair plus an unpaired write of another parameter",
            vec![
                (SWITCH, Push::Both(COMPRESSOR, COMPRESSOR_THRESHOLD, value)),
                (
                    SWITCH,
                    Push::One(COMPRESSOR, COMPRESSOR_RATIO, ParameterChannel::Right, 3.0),
                ),
            ],
        ),
        (
            "per-channel EQ owner edits (two one-channel targets)",
            vec![
                (SWITCH, Push::One(EQ, EQ_GAIN, ParameterChannel::Left, 4.5)),
                (SWITCH, Push::One(EQ, EQ_GAIN, ParameterChannel::Right, 4.5)),
            ],
        ),
    ];
    for (name, writes) in cases {
        let (counters, transitions, _) = console_run(Backend::current(), BLOCKS, &|block, _| {
            let mut out: Vec<(usize, Push)> = mixed_ride(block);
            out.extend(
                writes
                    .iter()
                    .filter(|(at, _)| *at == block)
                    .map(|(_, write)| (13, *write)),
            );
            out
        });
        assert_eq!(
            counters[0],
            BLOCKS * counters[1] - (BLOCKS - SWITCH),
            "{name}: exactly the written track's cohort stops, on the block the write lands"
        );
        assert_eq!(transitions, [1, 0, 0], "{name}");
    }
}

// =============================================================================================
// Part 4: the pinned scenario (gate 3)
// =============================================================================================

/// The mono console's 8-of-64 mixed ride over 128 blocks, pinned to the output digest of the base
/// tree (`1010d50c`, code-identical to the diagnosis base `49f696c7`).
///
/// Every track's three controls are first set at the ride's bases (the EQ as `Both` owner edits,
/// the compressor and the limiter as `Left` then `Right` records -- the product's shapes), then the
/// ride runs. On the base tree the settle writes retire every cohort for the whole run; with the
/// pairing rule every cohort stays collapsed on every block at `Simd8`. The digest is the same:
/// only the counters move.
///
/// The ride has to be doing something for the pin to say anything (VERIFY-AUTOMATION F6), so the
/// run also asserts that each of the three rides moves the output away from the settled run.
#[test]
fn the_pinned_mixed_ride_renders_the_base_bits() {
    const BLOCKS: u64 = 128;
    // Taken on the base tree at both widths, which render the same bits. On the base tree the
    // collapse counters were `[0, 8]` at `Simd8` (the settle writes retire every cohort before the
    // first block renders) and `[2048, 16]` at `Simd4`, where this x86-64-v3 build binds no
    // four-lane EQ or compressor bank (decision D4), so no collapsed chain holds one.
    const BASE: &str = "9242f149101f3bcd2e48268096169f1670a4ec368fe11071408f6b44a49c45a4";
    const PINS: [(Backend, &str); 2] = [(Backend::Simd8, BASE), (Backend::Simd4, BASE)];
    let native = Backend::current();
    for (dispatch, pin) in PINS {
        if dispatch.width() > native.width() {
            continue;
        }
        let run = |ride: &dyn Fn(u64) -> Vec<(usize, Push)>| {
            let mut runtime = SessionRuntime::build_with_dispatch(MONO, CONTROL, dispatch);
            let map = channels(&runtime);
            let tracks = track_ids(&map);
            let mut digest = Sha256Sink::new();
            for block in 0..BLOCKS {
                let mut writes = if block == 0 {
                    settle(tracks.len())
                } else {
                    Vec::new()
                };
                writes.extend(ride(block));
                for (track, write) in writes {
                    let channel = map[&(tracks[track].clone(), effect_of(write).to_owned())];
                    push(&mut runtime, channel, write);
                }
                runtime.render(block).expect("console render");
                runtime.hash_output(&mut digest);
            }
            (digest.finish_hex(), runtime.bank_collapse_counters())
        };
        let (digest, counters) = run(&mixed_ride);
        eprintln!("{dispatch:?}: digest {digest} collapse counters {counters:?}");
        let (settled, _) = run(&|_| Vec::new());
        for part in 0..3 {
            let (only, _) = run(&|block| {
                mixed_ride(block)
                    .into_iter()
                    .enumerate()
                    .filter(|(index, _)| index % 3 == part)
                    .map(|(_, write)| write)
                    .collect()
            });
            assert_ne!(
                only, settled,
                "{dispatch:?}: ride part {part} must move the output"
            );
        }
        assert_eq!(digest, pin, "{dispatch:?}: the mixed ride moved a bit");
        if dispatch == native {
            assert_eq!(
                counters[0],
                BLOCKS * counters[1],
                "{dispatch:?}: every cohort collapsed on every block"
            );
        }
    }
}
