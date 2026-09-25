#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: this target deliberately invokes the timed harness.
//! MQ-2: frozen descriptive measurement of the compressor's per-frame ramp spike.
//!
//! Run only through `scripts/run-issue880-mq2-benchmark.sh`; it performs the no-timing preflight,
//! requires an explicit output path, records the committed harness identity, and invokes this
//! ignored test once. Every arm has one 32-block warmup and two 32-block measured rounds. No timed
//! result is an acceptance threshold.

mod support;

use compressor::CompressorFactory;
use effect_contract::{
    AutomationSpanKind, BankWidth, EffectBankProcessBlock, InitialParameterValue,
    NativeEffectFactory, ParameterChannel, PrepareEffectBankRequest, PreparedAutomationSpan,
    PreparedNativeEffect, PreparedNativeEffectBank,
};
use lane::Backend;
use std::time::Instant;

const SAMPLE_RATE: u32 = 48_000;
const QUANTUM: u32 = 128;
const BANK_WIDTH: usize = 8;
const BANK_COUNT: usize = 8;
const TRACK_COUNT: usize = BANK_WIDTH * BANK_COUNT;
const RAMP_FRAMES: usize = 64;
const BLOCKS_PER_ROUND: usize = 32;
const MEASURED_ROUNDS: usize = 2;
const RELEASE_PARAMETER: u32 = 4;
const ATTACK_PARAMETER: u32 = 3;
// Each frame-zero event designs one exact target coefficient per lane and channel. The live start
// coefficient is reused from the current coefficient word; there is no sample-rate exp call.
const EXP_CALLS_PER_BANK_PER_PARAMETER: usize = BANK_WIDTH * 2;
const EMPTY_OFFSETS: [u32; BANK_WIDTH + 1] = [0; BANK_WIDTH + 1];

#[derive(Clone, Copy, Debug)]
enum Arm {
    ReleaseOnly,
    AttackAndRelease,
    NoAutomation,
}

impl Arm {
    fn name(self) -> &'static str {
        match self {
            Self::ReleaseOnly => "release_only",
            Self::AttackAndRelease => "attack_and_release",
            Self::NoAutomation => "no_automation",
        }
    }

    fn ramping_parameters(self) -> usize {
        match self {
            Self::ReleaseOnly => 1,
            Self::AttackAndRelease => 2,
            Self::NoAutomation => 0,
        }
    }
}

fn point(parameter_index: u32, channel: ParameterChannel, value: f32) -> PreparedAutomationSpan {
    PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel,
        parameter_index,
        start_sample: 0,
        end_sample: 0,
        start_value: value,
        end_value: value,
    }
}

struct SpanPlans {
    variants: [Vec<PreparedAutomationSpan>; 2],
    offsets: [u32; BANK_WIDTH + 1],
}

impl SpanPlans {
    fn new(arm: Arm) -> Option<Self> {
        if matches!(arm, Arm::NoAutomation) {
            return None;
        }
        let mut offsets = [0_u32; BANK_WIDTH + 1];
        let mut variants = [Vec::new(), Vec::new()];
        for lane in 0..BANK_WIDTH {
            for (variant, spans) in variants.iter_mut().enumerate() {
                match arm {
                    Arm::ReleaseOnly => {
                        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                            spans.push(point(
                                RELEASE_PARAMETER,
                                channel,
                                if variant == 0 { 800.0 } else { 1_600.0 },
                            ));
                        }
                    }
                    Arm::AttackAndRelease => {
                        for (parameter, value) in [
                            (ATTACK_PARAMETER, if variant == 0 { 20.0 } else { 80.0 }),
                            (
                                RELEASE_PARAMETER,
                                if variant == 0 { 600.0 } else { 1_800.0 },
                            ),
                        ] {
                            for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                                spans.push(point(parameter, channel, value));
                            }
                        }
                    }
                    Arm::NoAutomation => unreachable!(),
                }
            }
            offsets[lane + 1] = variants[0].len() as u32;
        }
        debug_assert_eq!(variants[0].len(), variants[1].len());
        Some(Self { variants, offsets })
    }

    fn for_block(
        &mut self,
        block: usize,
        first_sample: u64,
    ) -> (&[PreparedAutomationSpan], &[u32]) {
        let events = &mut self.variants[block % 2];
        for event in events.iter_mut() {
            event.start_sample = first_sample;
            event.end_sample = first_sample;
        }
        (events, &self.offsets)
    }
}

struct Audio {
    left: Vec<f32>,
    right: Vec<f32>,
}

impl Audio {
    fn seeded() -> Self {
        Self {
            left: support::noise(QUANTUM as usize * BANK_WIDTH, 0x8800_0001, 0.5),
            right: support::noise(QUANTUM as usize * BANK_WIDTH, 0x8800_0002, 0.5),
        }
    }
}

struct BankAudio {
    left: Vec<f32>,
    right: Vec<f32>,
}

struct BenchArm {
    banks: Vec<Box<dyn PreparedNativeEffectBank>>,
    buffers: Vec<BankAudio>,
    spans: Option<SpanPlans>,
}

fn defaults() -> [InitialParameterValue; support::PARAMETER_COUNT * 2] {
    support::initial_values()
}

fn make_banks(values: &[InitialParameterValue]) -> Vec<Box<dyn PreparedNativeEffectBank>> {
    assert_eq!(
        Backend::current(),
        Backend::Simd8,
        "MQ-2 is native x86-64-v3"
    );
    let requests: Vec<_> = (0..BANK_WIDTH).map(|_| support::request(values)).collect();
    (0..BANK_COUNT)
        .map(|_| {
            CompressorFactory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend: Backend::Simd8,
                    width: BankWidth::Eight,
                    requests: &requests,
                })
                .expect("valid eight-track bank")
                .expect("Simd8 compressor bank available")
        })
        .collect()
}

impl BenchArm {
    fn prepare(arm: Arm, audio: &Audio) -> Self {
        let values = defaults();
        Self {
            banks: make_banks(&values),
            buffers: (0..BANK_COUNT)
                .map(|_| BankAudio {
                    left: audio.left.clone(),
                    right: audio.right.clone(),
                })
                .collect(),
            spans: SpanPlans::new(arm),
        }
    }

    fn block(&mut self, block: usize, first_sample: u64, audio: &Audio) -> u64 {
        for buffer in &mut self.buffers {
            buffer.left.copy_from_slice(&audio.left);
            buffer.right.copy_from_slice(&audio.right);
        }
        let empty: &[PreparedAutomationSpan] = &[];
        let (events, offsets) = self
            .spans
            .as_mut()
            .map_or((empty, &EMPTY_OFFSETS[..]), |plans| {
                plans.for_block(block, first_sample)
            });

        let started = Instant::now();
        for (bank, buffer) in self.banks.iter_mut().zip(&mut self.buffers) {
            bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut buffer.left,
                    &mut buffer.right,
                    None,
                    QUANTUM,
                    BankWidth::Eight,
                    first_sample,
                    events,
                    offsets,
                    QUANTUM,
                )
                .expect("frozen MQ-2 block shape"),
            );
        }
        u64::try_from(started.elapsed().as_nanos()).unwrap_or(u64::MAX)
    }

    fn round(&mut self, first_block: usize, audio: &Audio) -> Vec<u64> {
        (0..BLOCKS_PER_ROUND)
            .map(|index| {
                let global_block = first_block + index;
                self.block(
                    global_block,
                    (global_block * QUANTUM as usize) as u64,
                    audio,
                )
            })
            .collect()
    }
}

fn mean(values: &[u64]) -> f64 {
    values.iter().map(|value| *value as f64).sum::<f64>() / values.len() as f64
}

fn json_u64s(values: &[u64]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn json_rounds(rounds: &[Vec<u64>]) -> String {
    format!(
        "[{}]",
        rounds
            .iter()
            .map(|round| json_u64s(round))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn json_f64s(values: &[f64]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| format!("{value:.3}"))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn summarize_arm(arm: Arm, measured: [Vec<u64>; MEASURED_ROUNDS]) -> String {
    let means_us: Vec<_> = measured.iter().map(|round| mean(round) / 1_000.0).collect();
    let maxima_us: Vec<_> = measured
        .iter()
        .map(|round| round.iter().copied().max().expect("nonempty round") as f64 / 1_000.0)
        .collect();
    let calls_per_block = rate_coefficient_calls_per_block(arm);
    format!(
        concat!(
            "{{\"name\":\"{}\",\"ramping_parameters\":{},",
            "\"rate_coefficient_calls_per_bank_per_parameter\":{},",
            "\"rate_coefficient_calls_per_block\":{},\"round_block_ns\":{},",
            "\"round_mean_us\":{},\"round_max_us\":{}}}"
        ),
        arm.name(),
        arm.ramping_parameters(),
        EXP_CALLS_PER_BANK_PER_PARAMETER,
        calls_per_block,
        json_rounds(&measured),
        json_f64s(&means_us),
        json_f64s(&maxima_us),
    )
}

fn rate_coefficient_call_counts() -> [usize; 3] {
    let ramping_channels_per_bank = BANK_WIDTH * 2;
    [Arm::ReleaseOnly, Arm::AttackAndRelease, Arm::NoAutomation]
        .map(|arm| BANK_COUNT * ramping_channels_per_bank * arm.ramping_parameters())
}

fn rate_coefficient_calls_per_block(arm: Arm) -> usize {
    BANK_COUNT * (BANK_WIDTH * 2) * arm.ramping_parameters()
}

fn request_points(arm: Arm, variant: usize, first_sample: u64) -> Vec<PreparedAutomationSpan> {
    let mut plans = SpanPlans::new(arm).expect("ramping arm");
    let events = &mut plans.variants[variant];
    for event in events.iter_mut() {
        event.start_sample = first_sample;
        event.end_sample = first_sample;
    }
    events.clone()
}

fn process_bank_block(bank: &mut dyn PreparedNativeEffectBank, block: PayloadProbeBlock<'_>) {
    bank.process_bank(
        EffectBankProcessBlock::new(
            block.left,
            block.right,
            None,
            block.frames,
            BankWidth::Eight,
            block.first_sample,
            block.events,
            block.offsets,
            QUANTUM,
        )
        .expect("ramp payload probe block"),
    );
}

struct PayloadProbeBlock<'a> {
    left: &'a mut [f32],
    right: &'a mut [f32],
    first_sample: u64,
    events: &'a [PreparedAutomationSpan],
    offsets: &'a [u32],
    frames: u32,
}

fn assert_lane_ramp_state(
    bank: &dyn PreparedNativeEffectBank,
    reference: &dyn PreparedNativeEffect,
    track: u32,
    parameter: usize,
    expected_target: f32,
    expected_remaining: u32,
) {
    let (left, right) = support::snapshot_track(bank, track, reference);
    let current_word = 1 + parameter * 3;
    for channel in [&left, &right] {
        assert_eq!(
            effect_runtime::state_payload::read_f32(channel, current_word + 1).to_bits(),
            expected_target.to_bits()
        );
        assert_eq!(
            effect_runtime::state_payload::read_u32(channel, current_word + 2),
            expected_remaining
        );
    }
}

/// Non-timed gate: verify both-channel lane events restart and complete the fixed 64-sample ramps.
#[test]
fn mq2_preflight_payloads_prove_ramps_restart_on_each_block() {
    if Backend::current() != Backend::Simd8 {
        return;
    }
    assert_eq!(rate_coefficient_call_counts(), [128, 256, 0]);
    assert_eq!(EXP_CALLS_PER_BANK_PER_PARAMETER, 16);

    let values = defaults();
    let reference = support::prepare(support::request(&values));
    let mut banks = make_banks(&values);
    let bank = &mut banks[0];
    let left_source = vec![0.25_f32; BANK_WIDTH];
    let right_source = vec![-0.25_f32; BANK_WIDTH];
    let offsets_a = SpanPlans::new(Arm::ReleaseOnly).unwrap().offsets;
    let release_a = request_points(Arm::ReleaseOnly, 0, 0);
    let mut left = left_source.clone();
    let mut right = right_source.clone();
    process_bank_block(
        bank.as_mut(),
        PayloadProbeBlock {
            left: &mut left,
            right: &mut right,
            first_sample: 0,
            events: &release_a,
            offsets: &offsets_a,
            frames: 1,
        },
    );
    for track in 0..BANK_WIDTH {
        assert_lane_ramp_state(
            bank.as_ref(),
            reference.as_ref(),
            track as u32,
            4,
            800.0,
            63,
        );
    }
    let empty: [PreparedAutomationSpan; 0] = [];
    let mut settle_left = vec![0.0_f32; BANK_WIDTH * (RAMP_FRAMES - 1)];
    let mut settle_right = vec![0.0_f32; BANK_WIDTH * (RAMP_FRAMES - 1)];
    process_bank_block(
        bank.as_mut(),
        PayloadProbeBlock {
            left: &mut settle_left,
            right: &mut settle_right,
            first_sample: 1,
            events: &empty,
            offsets: &EMPTY_OFFSETS,
            frames: (RAMP_FRAMES - 1) as u32,
        },
    );
    for track in 0..BANK_WIDTH {
        assert_lane_ramp_state(bank.as_ref(), reference.as_ref(), track as u32, 4, 800.0, 0);
    }

    let release_b = request_points(Arm::ReleaseOnly, 1, RAMP_FRAMES as u64);
    process_bank_block(
        bank.as_mut(),
        PayloadProbeBlock {
            left: &mut left[..BANK_WIDTH],
            right: &mut right[..BANK_WIDTH],
            first_sample: RAMP_FRAMES as u64,
            events: &release_b,
            offsets: &offsets_a,
            frames: 1,
        },
    );
    for track in 0..BANK_WIDTH {
        assert_lane_ramp_state(
            bank.as_ref(),
            reference.as_ref(),
            track as u32,
            4,
            1_600.0,
            63,
        );
    }

    let mut both_values = values;
    both_values[ATTACK_PARAMETER as usize * 2].value = 10.0;
    both_values[ATTACK_PARAMETER as usize * 2 + 1].value = 10.0;
    let both_reference = support::prepare(support::request(&both_values));
    let mut both_banks = make_banks(&both_values);
    let both = request_points(Arm::AttackAndRelease, 0, 0);
    let offsets_b = SpanPlans::new(Arm::AttackAndRelease).unwrap().offsets;
    let mut both_left = left_source;
    let mut both_right = right_source;
    process_bank_block(
        both_banks[0].as_mut(),
        PayloadProbeBlock {
            left: &mut both_left,
            right: &mut both_right,
            first_sample: 0,
            events: &both,
            offsets: &offsets_b,
            frames: 1,
        },
    );
    for track in 0..BANK_WIDTH {
        assert_lane_ramp_state(
            both_banks[0].as_ref(),
            both_reference.as_ref(),
            track as u32,
            3,
            20.0,
            63,
        );
        assert_lane_ramp_state(
            both_banks[0].as_ref(),
            both_reference.as_ref(),
            track as u32,
            4,
            600.0,
            63,
        );
    }
}

/// The one timed invocation. It measures only `process_bank` over the eight banks per block.
#[test]
#[ignore = "descriptive ramp benchmark; run once through scripts/run-issue880-mq2-benchmark.sh"]
fn mq2_compressor_ramp_spike() {
    assert_eq!(TRACK_COUNT, 64);
    assert_eq!(EXP_CALLS_PER_BANK_PER_PARAMETER, 16);
    let audio = Audio::seeded();
    eprintln!("MISO_ENGINE_BENCH_PHASE workload_started");
    let mut release = BenchArm::prepare(Arm::ReleaseOnly, &audio);
    let mut attack_release = BenchArm::prepare(Arm::AttackAndRelease, &audio);
    let mut no_automation = BenchArm::prepare(Arm::NoAutomation, &audio);
    let _warmup_release = release.round(0, &audio);
    let _warmup_attack_release = attack_release.round(0, &audio);
    let _warmup_no_automation = no_automation.round(0, &audio);
    eprintln!("MISO_ENGINE_BENCH_PHASE warmup_complete");
    eprintln!("MISO_ENGINE_BENCH_PHASE timed_started");
    let first_round = [
        release.round(BLOCKS_PER_ROUND, &audio),
        attack_release.round(BLOCKS_PER_ROUND, &audio),
        no_automation.round(BLOCKS_PER_ROUND, &audio),
    ];
    eprintln!("MISO_ENGINE_BENCH_PHASE round_1_complete");
    let second_round = [
        release.round(BLOCKS_PER_ROUND * 2, &audio),
        attack_release.round(BLOCKS_PER_ROUND * 2, &audio),
        no_automation.round(BLOCKS_PER_ROUND * 2, &audio),
    ];
    eprintln!("MISO_ENGINE_BENCH_PHASE round_2_complete");
    let arms = [Arm::ReleaseOnly, Arm::AttackAndRelease, Arm::NoAutomation].map(|arm| {
        let index = match arm {
            Arm::ReleaseOnly => 0,
            Arm::AttackAndRelease => 1,
            Arm::NoAutomation => 2,
        };
        summarize_arm(
            arm,
            [first_round[index].clone(), second_round[index].clone()],
        )
    });
    println!(
        "MQ2_RESULT {{\"schema_version\":2,\"task\":\"MQ-2\",\"sample_rate_hz\":{SAMPLE_RATE},\"quantum_frames\":{QUANTUM},\"bank_width\":{BANK_WIDTH},\"bank_count\":{BANK_COUNT},\"track_count\":{TRACK_COUNT},\"warmup_blocks_per_arm\":{BLOCKS_PER_ROUND},\"measured_blocks_per_round\":{BLOCKS_PER_ROUND},\"measured_rounds_per_arm\":{MEASURED_ROUNDS},\"ramp_frames\":{RAMP_FRAMES},\"events\":\"frame-zero point events on every lane and both channels\",\"arms\":[{}, {}, {}]}}",
        arms[0], arms[1], arms[2]
    );
}

/// Exercises marker extraction against this target's actual libtest output in the preflight.
#[test]
fn mq2_libtest_result_marker_probe() {
    println!(
        "MQ2_RESULT_PROBE {{\"probe\":true,\"simd8\":{}}}",
        Backend::current() == Backend::Simd8
    );
}
