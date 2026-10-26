//! Issue #2129 S1, gate 5: a lane whose bypass ends on the block that resumes a skipped bank holds
//! the dry signal for its effect's warm-up, then crossfades onto output a never-bypassed lane
//! renders, with no dip, no NaN and no subnormal, and the limiter keeps its ceiling from the first
//! crossfaded frame.
//!
//! Three legs of one bank of the host's width, through the rack's real live stage, at 48 kHz, with
//! every lane carrying a live bypass cell:
//!
//! * **resumed**: every lane bypassed, so the bank skips its wet path for 16 blocks; lane 0 is then
//!   un-bypassed with a ramp. The block that resumes resets the bank and lane 0 holds dry for `W`.
//! * **reference**: lane 0 never bypassed; its bank never skips.
//! * **running**: lane 0 bypassed and un-bypassed at the same block as the resumed leg, but lane 1
//!   is enabled, so the bank never skipped and lane 0 takes today's crossfade with no hold.
//!
//! The true-peak limiter (`W = T + N = 966` at 48 kHz) and the soft clip (`W = latency + tail =
//! 60`) are the two launch effects whose warm-up exceeds their latency; the test reads each bank's
//! declared `W` (`warmup_samples`), so a wrong declaration is caught by what it renders.

use effect_compiler::launch_native_effect_registry;
use effect_contract::{
    BankWidth, BypassSwitch, EffectCellWriter, EffectControlLane, EffectControlRecord,
    EffectDescriptor, EffectLaneShape, EffectLaneSpec, EffectQuality, InitialParameterValue,
    LinkMode, ParameterChannel, ParameterChannelPolicy, PrepareEffectBankRequest,
    PrepareEffectLimits, PrepareEffectRequest, PreparedPorts, PreparedSidechainPort,
    effect_control_lane,
};
use engine::realtime::{LiveValuesSuperseded, RevisionGate};
use graph_compiler::Backend;
use rack::{AoSoaScratch, BankChain, BankMembers, BankSlot, BankStage, LiveControlEffectBankStage};

const RATE: u32 = 48_000;
const QUANTUM: u32 = 128;
/// The block lane 0 is un-bypassed at: sixteen skipped blocks before it.
const RESUME: u64 = 16;
const BLOCKS: u64 = 32;
/// The session ramp of the un-bypass in the crossfade rows.
const RAMP: u32 = 256;

fn width() -> BankWidth {
    BankWidth::for_backend(Backend::current()).expect("the delivery host has a bank width")
}

/// Every parameter at its declared default.
fn default_values(descriptor: &EffectDescriptor) -> Vec<InitialParameterValue> {
    let mut values = Vec::new();
    for (index, parameter) in descriptor.parameters.iter().enumerate() {
        let channels: &[ParameterChannel] = match parameter.channel_policy {
            ParameterChannelPolicy::Shared => &[ParameterChannel::Both],
            ParameterChannelPolicy::PerLane => &[ParameterChannel::Left, ParameterChannel::Right],
        };
        for channel in channels {
            values.push(InitialParameterValue {
                parameter_index: index as u32,
                channel: *channel,
                value: parameter.default_value,
            });
        }
    }
    values
}

fn request<'a>(
    descriptor: &EffectDescriptor,
    values: &'a [InitialParameterValue],
) -> PrepareEffectRequest<'a> {
    PrepareEffectRequest {
        sample_rate: RATE,
        quantum: QUANTUM,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 1 << 30,
            maximum_scratch_bytes: 1 << 28,
            maximum_automation_spans_per_block: 32,
        },
        tail_bound: launch_native_effect_registry()
            .expect("launch registry")
            .tail_bound(descriptor.id, RATE, EffectQuality::Normal)
            .expect("a declared row"),
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

/// Which lanes start bypassed, and whether lane 0 is un-bypassed at [`RESUME`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Leg {
    Resumed,
    Reference,
    Running,
}

/// The input of sample `n` on channel `channel`: a 100 Hz triangle at `amplitude` (exact
/// arithmetic, the same on every target), the right channel a quarter period behind.
fn input(n: u64, channel: usize, amplitude: f32) -> f32 {
    const PERIOD: u64 = 480;
    let phase = (n + PERIOD - channel as u64 * PERIOD / 4) % PERIOD;
    let unit = phase as f32 / PERIOD as f32;
    amplitude * (4.0 * (unit - 0.5).abs() - 1.0)
}

/// One leg: lane 0's output `[channel][sample]`, the bank's skip count, its latency and its
/// declared warm-up `W`.
struct Rendered {
    output: [Vec<f32>; 2],
    skips: u64,
    latency: usize,
    warmup: usize,
}

fn render(effect_id: &str, leg: Leg, amplitude: f32, ramp: u32) -> Rendered {
    let registry = launch_native_effect_registry().expect("launch registry");
    let factory = registry.get_shared_ascii(effect_id).expect("launch effect");
    let descriptor = factory.descriptor();
    let values = default_values(descriptor);
    let width = width();
    let lanes = width.lanes() as usize;
    let requests: Vec<PrepareEffectRequest<'_>> =
        (0..lanes).map(|_| request(descriptor, &values)).collect();
    let bank = factory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: Backend::current(),
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        })
        .expect("a well-formed bank request")
        .expect("the effect banks");
    let latency = usize::try_from(bank.metadata.program_key.latency.0).expect("latency fits");
    let warmup = bank
        .processor
        .warmup_samples(u32::try_from(latency).expect("latency fits")) as usize;
    let superseded = LiveValuesSuperseded::new();
    let mut writers: Vec<EffectCellWriter> = Vec::new();
    let mut controls: Vec<Option<EffectControlLane>> = Vec::new();
    for lane in 0..lanes {
        let bypassed = match leg {
            Leg::Resumed => true,
            Leg::Reference => lane != 0,
            Leg::Running => lane != 1,
        };
        let (writer, control) = effect_control_lane(
            EffectLaneSpec {
                shape: EffectLaneShape::from_live_parameters([], 0),
                bypass: bypassed,
                observation_depth: 0,
                link: None,
            },
            &superseded,
        )
        .expect("the live lane");
        writers.push(writer);
        controls.push(Some(control));
    }
    let stage = LiveControlEffectBankStage::new(
        bank,
        width,
        QUANTUM,
        controls,
        (0..lanes).map(|_| None).collect(),
        latency,
    )
    .expect("stage");
    let mut chain = BankChain::new(
        AoSoaScratch::new(width, QUANTUM).expect("scratch"),
        vec![true; lanes].into_boxed_slice(),
        vec![BankSlot {
            stage: Box::new(stage) as Box<dyn BankStage>,
            active_lanes: vec![true; lanes].into_boxed_slice(),
        }],
    )
    .expect("chain");
    let (gate, mut pin) = RevisionGate::pair(0);
    let mut output = [Vec::new(), Vec::new()];
    for block in 0..BLOCKS {
        if block == RESUME && leg != Leg::Reference {
            let revision = pin.last_published() + 1;
            let stamp = pin.stamp(revision).expect("a legal revision");
            writers[0]
                .write(
                    stamp,
                    EffectControlRecord::Bypass(BypassSwitch {
                        bypassed: false,
                        ramp_samples: ramp,
                    }),
                )
                .expect("a bypass cell");
            pin.publish(&gate, revision);
        }
        let first = block * u64::from(QUANTUM);
        let plane = |channel: usize| -> Vec<f32> {
            (0..u64::from(QUANTUM))
                .map(|frame| input(first + frame, channel, amplitude))
                .collect()
        };
        let mut members = Planes {
            left: (0..lanes).map(|_| plane(0)).collect(),
            right: (0..lanes).map(|_| plane(1)).collect(),
        };
        chain
            .run(&mut members, QUANTUM, first, gate.snapshot())
            .expect("bank render");
        output[0].extend_from_slice(&members.left[0]);
        output[1].extend_from_slice(&members.right[0]);
    }
    Rendered {
        output,
        skips: chain.bypass_skips(),
        latency,
        warmup,
    }
}

/// The latency-matched dry signal of sample `n`.
fn dry(n: usize, channel: usize, latency: usize, amplitude: f32) -> f32 {
    if n < latency {
        0.0
    } else {
        input((n - latency) as u64, channel, amplitude)
    }
}

/// Gate 5, the crossfade rows, for the limiter and the soft clip on a sub-threshold triangle:
///
/// * the resumed leg skipped every block before [`RESUME`], and its output has no NaN and no
///   subnormal;
/// * it is the dry signal, bit for bit, over `[s_B, s_B + W)`;
/// * from `s_B + W + RAMP` it is the reference leg's output, bit for bit;
/// * over the crossfade between them it is never outside `[min, max]` of the dry and reference
///   samples by more than `1e-6`: no dip;
/// * the running leg, whose bank never skipped, takes no hold: it is the reference from
///   `s_B + RAMP`.
///
/// Red mutations: the soft clip's `W` at its latency (31; the crossfade starts while its half-band
/// FIR reads reset zeros and dips below the dry and reference samples); a stage that marks a bank
/// stale while a lane is merely bypassed in a running bank (the running leg holds and resets, and
/// the soft clip's misses the reference from `s_B + RAMP`).
#[test]
fn a_resumed_lane_holds_dry_for_its_warmup_then_crossfades_without_a_dip() {
    let amplitude = 0.1;
    for effect_id in ["miso.true-peak-limiter", "miso.soft-clip"] {
        let resumed = render(effect_id, Leg::Resumed, amplitude, RAMP);
        let reference = render(effect_id, Leg::Reference, amplitude, RAMP);
        let running = render(effect_id, Leg::Running, amplitude, RAMP);
        let resume = (RESUME * u64::from(QUANTUM)) as usize;
        let (latency, warmup, end) = (resumed.latency, resumed.warmup, resumed.output[0].len());
        assert!(
            warmup > latency,
            "{effect_id}: the warm-up exceeds the latency"
        );
        assert_eq!(
            resumed.skips, RESUME,
            "{effect_id}: the bank skipped until it resumed"
        );
        assert_eq!(
            reference.skips + running.skips,
            0,
            "{effect_id}: never skipped"
        );
        for channel in 0..2 {
            let (out, wet) = (&resumed.output[channel], &reference.output[channel]);
            for (n, sample) in out.iter().enumerate() {
                assert!(
                    sample.is_finite() && !sample.is_subnormal(),
                    "{effect_id}: channel {channel} sample {n} is {sample:e}"
                );
            }
            for (n, sample) in out.iter().enumerate().take(resume + warmup).skip(resume) {
                assert_eq!(
                    sample.to_bits(),
                    dry(n, channel, latency, amplitude).to_bits(),
                    "{effect_id}: channel {channel} sample {n} holds dry"
                );
            }
            for n in resume + warmup..resume + warmup + RAMP as usize {
                let dry = dry(n, channel, latency, amplitude);
                let (low, high) = (dry.min(wet[n]), dry.max(wet[n]));
                assert!(
                    out[n] >= low - 1e-6 && out[n] <= high + 1e-6,
                    "{effect_id}: channel {channel} sample {n} is {} outside the dry {dry} and \
                     the reference {}",
                    out[n],
                    wet[n]
                );
            }
            for n in resume + warmup + RAMP as usize..end {
                assert_eq!(
                    out[n].to_bits(),
                    wet[n].to_bits(),
                    "{effect_id}: channel {channel} sample {n} is the reference"
                );
            }
            let settled = resume + RAMP as usize;
            for (n, (sample, wet)) in running.output[channel]
                .iter()
                .zip(wet)
                .enumerate()
                .skip(settled)
            {
                assert_eq!(
                    sample.to_bits(),
                    wet.to_bits(),
                    "{effect_id}: the running leg's channel {channel} sample {n} took no hold"
                );
            }
        }
    }
}

/// Gate 5, the ceiling row: the limiter on a triangle about 10 dB over its ceiling, un-bypassed with
/// a step, so every sample from `s_B + W` is wet. None is above the ceiling: at `T + N` the box ramp
/// averages only post-reset gains, so the gain law's bound holds from the first wet frame.
///
/// Red mutation: the limiter's `W` at `T` alone (its latency): the first `N` wet frames take the
/// box's reset gains of `1` and pass the overs.
#[test]
fn a_resumed_limiter_keeps_its_ceiling_from_the_first_wet_frame() {
    let effect_id = "miso.true-peak-limiter";
    let registry = launch_native_effect_registry().expect("launch registry");
    let descriptor = registry
        .get_shared_ascii(effect_id)
        .expect("launch effect")
        .descriptor();
    let ceiling = descriptor
        .parameters
        .iter()
        .find(|parameter| parameter.name == "ceiling_db")
        .expect("a ceiling parameter")
        .default_value;
    let limit = math::db_to_gain_f32(ceiling);
    let resumed = render(effect_id, Leg::Resumed, 3.0, 0);
    let resume = (RESUME * u64::from(QUANTUM)) as usize;
    let warmup = resumed.warmup;
    for channel in 0..2 {
        let out = &resumed.output[channel];
        assert!(
            out[resume..resume + warmup]
                .iter()
                .any(|sample| sample.abs() > limit),
            "{effect_id}: the dry hold carries overs, or the row proves nothing"
        );
        for (n, sample) in out.iter().enumerate().skip(resume + warmup) {
            assert!(
                sample.abs() <= limit * (1.0 + 1e-6),
                "{effect_id}: channel {channel} sample {n} is {sample} over the ceiling {limit}"
            );
        }
    }
}
