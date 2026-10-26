//! Issue #2129 S1b, gate 5 and the B1 row: a lane leaving settled bypass is reset alone and holds
//! the dry signal for its effect's warm-up `W`, then crossfades onto the output of a lane that never
//! paused, with no NaN and no subnormal; the limiter keeps its ceiling from the first wet frame; and
//! the lane's words after its un-bypass do not depend on its bank mates.
//!
//! Three legs of one bank of the host's width, through the rack's real live stage, at 48 kHz, with
//! every lane carrying a live bypass cell:
//!
//! * **resumed**: every lane bypassed, so the bank skips its wet path for 16 blocks; lane 0 is then
//!   un-bypassed.
//! * **reference**: lane 0 never bypassed; its bank never skips.
//! * **running**: lane 0 bypassed and un-bypassed at the same block as the resumed leg, but lane 1
//!   is enabled, so the bank never skipped and lane 0's wet path ran while it was bypassed.
//!
//! The soft clip (`W = latency + tail = 60`) is the launch effect whose warm-up exceeds its latency;
//! the true-peak limiter's is its latency `T` (486 at 48 kHz), and its wet output is `+0.0` until
//! then. The test reads each bank's declared `W` (`warmup_samples`), so a wrong declaration is
//! caught by what it renders.

use effect_compiler::launch_native_effect_registry;
use effect_contract::{
    BankWidth, BypassSwitch, EffectCellWriter, EffectControlLane, EffectControlRecord,
    EffectDescriptor, EffectLaneShape, EffectLaneSpec, EffectQuality, InitialParameterValue,
    LinkMode, ParameterChannel, ParameterChannelPolicy, PortRole, PrepareEffectBankRequest,
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
        ports: unrouted_ports(descriptor),
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

/// The ports `effect-compiler` prepares for an effect with no sidechain route.
fn unrouted_ports(descriptor: &EffectDescriptor) -> PreparedPorts {
    match descriptor
        .ports
        .iter()
        .find(|port| port.role == PortRole::SidechainInput)
    {
        Some(port) => PreparedPorts {
            sidechain: PreparedSidechainPort::Unconnected {
                id: port.id,
                required: port.required,
            },
        },
        None => PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
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

fn mix_at(length: u32, k: u32) -> f32 {
    lane::kernels::IndexedRamp::new([0.0; 4], [1.0, 0.0, 0.0, 0.0], length).coefficients_at(k)[0]
}

/// `dry + (wet - dry) * m`, as the shunt's crossfade computes it: a separate subtract, multiply
/// and add.
fn mixed(dry: f32, wet: f32, m: f32) -> f32 {
    dry + (wet - dry) * m
}

/// Gate 5, the crossfade rows, for the limiter and the soft clip on a sub-threshold triangle:
///
/// * the resumed leg skipped every block before [`RESUME`], and its output has no NaN and no
///   subnormal;
/// * it is the dry signal, bit for bit, over `[s_B, s_B + W)`;
/// * over `[s_B + W, s_B + W + RAMP)` it is the crossfade from the dry signal onto the reference
///   leg's output on the ramp law, `dry + (ref - dry) * m(k)` with `k = n - s_B - W + 1`, bit for
///   bit: the wet side is already the reference, so there is no dip;
/// * from `s_B + W + RAMP` it is the reference leg's output, bit for bit;
/// * and again with a step un-bypass (ramp `0`), whose first wet frame is `s_B + W` itself: at
///   `m(1)` of a ramp a one-frame-short `W` can round away, at a step it cannot.
///
/// Red mutations: the limiter's `W` one frame short (`T - 1`; its first wet frame reads the main
/// ring's reset `+0.0`), the soft clip's two frames short (`58`; its `59` is not observable, see
/// `soft_clip::warmup_samples`), no hold, a bank crossfade that ignores the hold.
#[test]
fn a_resumed_lane_holds_dry_for_its_warmup_then_crossfades_onto_the_reference() {
    let amplitude = 0.1;
    for (effect_id, ramp) in ["miso.true-peak-limiter", "miso.soft-clip"]
        .into_iter()
        .flat_map(|effect_id| [(effect_id, RAMP), (effect_id, 0)])
    {
        let resumed = render(effect_id, Leg::Resumed, amplitude, ramp);
        let reference = render(effect_id, Leg::Reference, amplitude, ramp);
        let resume = (RESUME * u64::from(QUANTUM)) as usize;
        let (latency, warmup) = (resumed.latency, resumed.warmup);
        assert_eq!(
            resumed.skips, RESUME,
            "{effect_id}: the bank skipped until it resumed"
        );
        assert_eq!(reference.skips, 0, "{effect_id}: never skipped");
        let fade = resume + warmup;
        for channel in 0..2 {
            let (out, wet) = (&resumed.output[channel], &reference.output[channel]);
            for (n, (sample, wet)) in out.iter().zip(wet).enumerate().skip(resume) {
                assert!(
                    sample.is_finite() && !sample.is_subnormal(),
                    "{effect_id}: channel {channel} sample {n} is {sample:e}"
                );
                let dry = dry(n, channel, latency, amplitude);
                let expected = if n < fade {
                    dry
                } else if n < fade + ramp as usize {
                    mixed(dry, *wet, mix_at(ramp, (n - fade + 1) as u32))
                } else {
                    *wet
                };
                assert_eq!(
                    sample.to_bits(),
                    expected.to_bits(),
                    "{effect_id}: channel {channel} sample {n} is {sample}, expected {expected} \
                     (dry {dry}, reference {wet}, crossfade from {fade})"
                );
            }
        }
    }
}

/// Issue #2129, the B1 row: a lane's un-bypass does not depend on its bank mates. Lane 0 is
/// un-bypassed at the same block with the same ramp in a bank whose other lanes are all bypassed
/// (so the bank skipped its wet path) and in a bank where lane 1 stays enabled (so it never
/// skipped and lane 0's wet path ran while it was bypassed). Lane 0's output words are the same
/// in both, from the first sample on, for a zero-latency effect and the two with a warm-up.
///
/// Red mutation: the D2/D3 law as first ruled, which reset and held a lane only when its whole
/// bank had skipped (the running leg continued warm and crossfaded at once).
#[test]
fn a_lanes_un_bypass_does_not_depend_on_its_bank_mates() {
    for effect_id in [
        "miso.compressor",
        "miso.true-peak-limiter",
        "miso.soft-clip",
    ] {
        let resumed = render(effect_id, Leg::Resumed, 0.1, RAMP);
        let running = render(effect_id, Leg::Running, 0.1, RAMP);
        assert_eq!(resumed.skips, RESUME, "{effect_id}: one bank skipped");
        assert_eq!(running.skips, 0, "{effect_id}: the other never did");
        for channel in 0..2 {
            for (n, (a, b)) in resumed.output[channel]
                .iter()
                .zip(&running.output[channel])
                .enumerate()
            {
                assert_eq!(
                    a.to_bits(),
                    b.to_bits(),
                    "{effect_id}: channel {channel} sample {n}: {a} after a skipped bank, {b} \
                     after a running one"
                );
            }
        }
    }
}

/// Gate 5, the ceiling row: the limiter on a triangle about 10 dB over its ceiling, un-bypassed with
/// a step, so every sample from `s_B + W` is wet. None is above the ceiling: from `T` every box
/// term is a post-reset minimum over a window that holds `n - N`, so the gain law's bound holds from
/// the first wet frame (the limiter's module docs).
///
/// Red mutation: no hold (the dry overs pass). `W = T - 1` is not red here: its extra frame is the
/// main ring's reset `+0.0`, under any ceiling; the crossfade row catches it.
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
