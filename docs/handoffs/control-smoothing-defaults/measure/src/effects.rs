//! The six launch effects that have a live bypass shunt, prepared and rendered through their
//! shipped factories, and the shunt's latency-matched dry signal (#1055 questions 5 and 6).
//!
//! Every effect except the delay and the multiband compressor lowers a session `bypass` to a
//! per-lane shunt (`effect_compiler::lowers_session_bypass`, `crates/effect-compiler/src/prepare.rs:
//! 244-268`): the compressor, the gate-expander, the parametric EQ, the soft clip, the transient
//! shaper and the true-peak limiter. Each is prepared here exactly as `effect-compiler` prepares
//! one (`crates/effect-compiler/src/prepare.rs:388-470`): the descriptor's default initial values
//! with the named parameters overridden on both channels, an unconnected optional sidechain, the
//! requested link mode, `bypass = false` (the shunt carries the bypass), normal quality, and the
//! 128-frame quantum. The wet signal is the scalar instance's output, rendered block by block from
//! the first frame of the material, so its state is continuous as in a running session. The dry
//! signal is the engine's own `effect_contract::BypassShunt` fed every block, which delays the
//! input by exactly the effect's declared latency. Nothing here is timed.

use compressor::CompressorFactory;
use effect_contract::{
    BypassShunt, EffectProcessBlock, EffectQuality, InitialParameterValue, LinkMode,
    NativeEffectFactory, PortRole, PrepareEffectLimits, PrepareEffectRequest, PreparedNativeEffect,
    PreparedPorts, PreparedSidechainPort, ProcessReport, default_initial_values,
};
use gate_expander::GateExpanderFactory;
use parametric_eq::ParametricEqFactory;
use soft_clip::SoftClipFactory;
use transient_shaper::TransientShaperFactory;
use true_peak_limiter::TruePeakLimiterFactory;

use crate::material::Stereo;
use crate::strip::QUANTUM;

/// The gain reduction the compressor and limiter rows are calibrated to (#1055 question 5).
pub const REDUCTION_DB: f64 = 6.0;
/// The saturator's drive. Its output gain is then set so that the wet RMS equals the dry RMS.
pub const SATURATOR_DRIVE_DB: f32 = 12.0;
/// The EQ rows' band: one bell at this frequency and Q, at plus and minus this gain.
pub const EQ_FREQUENCY_HZ: f32 = 100.0;
pub const EQ_Q: f32 = core::f32::consts::FRAC_1_SQRT_2;
pub const EQ_GAIN_DB: f32 = 6.0;
/// The transient shaper's attack amount (`+50 %`).
pub const SHAPER_ATTACK: f32 = 0.5;
/// Calibration and level matching ignore the first half second, where envelopes settle.
const SETTLE_S: f64 = 0.5;

/// One effect at one representative setting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effect {
    /// Compressor, default ballistics (ratio 4, knee 6 dB, attack 10 ms, release 100 ms, no
    /// makeup, mix 1), threshold set for a peak gain reduction of [`REDUCTION_DB`].
    Compressor,
    /// True-peak limiter, default release and lookahead, ceiling set for a peak gain reduction of
    /// [`REDUCTION_DB`].
    Limiter,
    /// Parametric EQ band 1: bell, [`EQ_FREQUENCY_HZ`], Q [`EQ_Q`], `+`[`EQ_GAIN_DB`].
    EqBoost,
    /// The same bell at `-`[`EQ_GAIN_DB`].
    EqCut,
    /// Soft clip driven by [`SATURATOR_DRIVE_DB`], output level-matched to the dry RMS.
    Saturator,
    /// Transient shaper, attack amount [`SHAPER_ATTACK`], sustain 0, mix 1.
    TransientShaper,
    /// Gate: threshold -6 dBFS, ratio 20, range 80 dB, no hysteresis, attack 1 ms, hold 20 ms,
    /// release 50 ms, so that it opens on each kick's attack and closes in the body, where the
    /// measured switches fall.
    Gate,
}

impl Effect {
    pub const ALL: [Self; 7] = [
        Self::Compressor,
        Self::Limiter,
        Self::EqBoost,
        Self::EqCut,
        Self::Saturator,
        Self::TransientShaper,
        Self::Gate,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Self::Compressor => "compressor",
            Self::Limiter => "limiter",
            Self::EqBoost => "eq-boost",
            Self::EqCut => "eq-cut",
            Self::Saturator => "saturator",
            Self::TransientShaper => "transient-shaper",
            Self::Gate => "gate",
        }
    }

    /// The materials a setting is representative on: a gate closing on the kick, a transient
    /// shaper on material with transients, every other effect on all three.
    pub fn materials(self) -> &'static [&'static str] {
        match self {
            Self::Gate => &["kick"],
            Self::TransientShaper => &["kick", "mix"],
            _ => &["bass", "kick", "mix"],
        }
    }

    fn factory(self) -> &'static dyn NativeEffectFactory {
        match self {
            Self::Compressor => &CompressorFactory,
            Self::Limiter => &TruePeakLimiterFactory,
            Self::EqBoost | Self::EqCut => &ParametricEqFactory,
            Self::Saturator => &SoftClipFactory,
            Self::TransientShaper => &TransientShaperFactory,
            Self::Gate => &GateExpanderFactory,
        }
    }

    /// The fixed overrides; the calibrated parameter (threshold, ceiling, output) is added by
    /// [`Setting::calibrate`].
    fn fixed(self) -> Vec<(&'static str, f32)> {
        match self {
            Self::Compressor | Self::Limiter => Vec::new(),
            Self::EqBoost | Self::EqCut => vec![
                ("band-1-enabled", 1.0),
                ("band-1-kind", 1.0),
                ("band-1-frequency", EQ_FREQUENCY_HZ),
                (
                    "band-1-gain",
                    if self == Self::EqBoost {
                        EQ_GAIN_DB
                    } else {
                        -EQ_GAIN_DB
                    },
                ),
                ("band-1-q", EQ_Q),
            ],
            Self::Saturator => vec![("drive", SATURATOR_DRIVE_DB)],
            Self::TransientShaper => vec![("attack amount", SHAPER_ATTACK)],
            Self::Gate => vec![
                ("threshold", -6.0),
                ("ratio", 20.0),
                ("range", 80.0),
                ("hysteresis", 0.0),
                ("attack", 1.0),
                ("hold", 20.0),
                ("release", 50.0),
            ],
        }
    }
}

/// One prepared configuration: the effect, its parameter overrides and its link mode.
#[derive(Clone, Debug)]
pub struct Setting {
    pub effect: Effect,
    pub overrides: Vec<(&'static str, f32)>,
    pub link: LinkMode,
}

impl Setting {
    /// The representative setting of `effect` on `input` at `rate`, with the compressor's
    /// threshold, the limiter's ceiling or the saturator's output gain calibrated on `input`.
    pub fn calibrate(effect: Effect, rate: u32, input: &Stereo, link: LinkMode) -> Self {
        let mut setting = Self {
            effect,
            overrides: effect.fixed(),
            link,
        };
        match effect {
            Effect::Compressor => setting.bisect("threshold", -80.0, 0.0, rate, input),
            Effect::Limiter => setting.bisect("ceiling", -24.0, 0.0, rate, input),
            Effect::Saturator => {
                let wet = setting.render(rate, input);
                let from = settle(rate);
                let output =
                    20.0 * (rms(input, 0, from) / rms(&wet, setting.latency(rate), from)).log10();
                setting.overrides.push(("output", output as f32));
            }
            _ => {}
        }
        setting
    }

    /// Sets `parameter` so that the peak gain reduction on `input` is [`REDUCTION_DB`]: the
    /// reduction falls as the parameter rises, so a bisection converges; the result is the `f32`
    /// nearest the bracket's midpoint after 24 halvings (a bracket of at most 80 dB is then
    /// narrower than 0.00001 dB).
    fn bisect(&mut self, parameter: &'static str, low: f64, high: f64, rate: u32, input: &Stereo) {
        let (mut low, mut high) = (low, high);
        self.overrides.push((parameter, 0.0));
        let slot = self.overrides.len() - 1;
        for _ in 0..24 {
            let mid = 0.5 * (low + high);
            self.overrides[slot].1 = mid as f32;
            let wet = self.render(rate, input);
            if peak_reduction_db(input, &wet, self.latency(rate), settle(rate)) > REDUCTION_DB {
                low = mid;
            } else {
                high = mid;
            }
        }
        self.overrides[slot].1 = (0.5 * (low + high)) as f32;
    }

    /// Every override, `name=value` separated by spaces, for the CSV.
    pub fn describe(&self) -> String {
        let mut text: Vec<String> = self
            .overrides
            .iter()
            .map(|(name, value)| format!("{}={value}", name.replace(' ', "-")))
            .collect();
        if self.effect == Effect::Compressor {
            text.push(format!("link={}", link_name(self.link)));
        }
        text.join(" ")
    }

    pub fn prepare(&self, rate: u32) -> Box<dyn PreparedNativeEffect> {
        prepare(self.effect.factory(), rate, &self.overrides, self.link)
    }

    pub fn latency(&self, rate: u32) -> usize {
        usize::try_from(self.prepare(rate).metadata().latency.0).expect("latency")
    }

    /// The wet signal: `input` through a fresh instance, block by block.
    pub fn render(&self, rate: u32, input: &Stereo) -> Stereo {
        render(self.prepare(rate).as_mut(), input)
    }
}

pub const fn link_name(link: LinkMode) -> &'static str {
    match link {
        LinkMode::DualMono => "dual_mono",
        LinkMode::Maximum => "maximum",
        LinkMode::Average => "average",
    }
}

fn settle(rate: u32) -> usize {
    (SETTLE_S * f64::from(rate)).round() as usize
}

/// Prepares one scalar instance as `effect-compiler` does, with `overrides` on both channels.
pub fn prepare(
    factory: &dyn NativeEffectFactory,
    rate: u32,
    overrides: &[(&str, f32)],
    link: LinkMode,
) -> Box<dyn PreparedNativeEffect> {
    let descriptor = factory.descriptor();
    let mut values: Vec<InitialParameterValue> = default_initial_values(descriptor).collect();
    for &(name, value) in overrides {
        let index = descriptor
            .parameters
            .iter()
            .position(|parameter| parameter.display_name == name)
            .unwrap_or_else(|| panic!("{} has no parameter {name}", descriptor.id.as_str()));
        let mut found = false;
        for entry in values
            .iter_mut()
            .filter(|entry| entry.parameter_index as usize == index)
        {
            entry.value = value;
            found = true;
        }
        assert!(found, "{name} has an initial value");
    }
    let sidechain = match descriptor
        .ports
        .iter()
        .find(|port| port.role == PortRole::SidechainInput)
    {
        Some(port) => {
            assert!(
                !port.required,
                "the measured effects' sidechains are optional"
            );
            PreparedSidechainPort::Unconnected {
                id: port.id,
                required: false,
            }
        }
        None => PreparedSidechainPort::None,
    };
    factory
        .prepare(PrepareEffectRequest {
            sample_rate: rate,
            quantum: QUANTUM as u32,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: link,
            ports: PreparedPorts { sidechain },
            initial_values: &values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: u64::MAX,
                maximum_scratch_bytes: u64::MAX,
                maximum_automation_spans_per_block: 64,
            },
        })
        .unwrap_or_else(|error| panic!("{}: {}", descriptor.id.as_str(), error.code))
}

/// `input` through `effect`, in 128-frame blocks from frame 0. Every block must report nothing:
/// no invalid span, no sanitised or non-finite sample.
pub fn render(effect: &mut dyn PreparedNativeEffect, input: &Stereo) -> Stereo {
    let mut out = input.clone();
    let frames = out.left.len();
    let mut start = 0;
    while start < frames {
        let end = (start + QUANTUM).min(frames);
        let report = effect.process(
            EffectProcessBlock::new(
                &mut out.left[start..end],
                &mut out.right[start..end],
                None,
                start as u64,
                &[],
                QUANTUM as u32,
            )
            .expect("block"),
        );
        assert_eq!(report, ProcessReport::default(), "block at {start}");
        start = end;
    }
    out
}

/// The shunt's dry signal: `input` through the engine's `BypassShunt` for `latency` samples,
/// captured every block as the live path captures it when the shunt feeds a line.
pub fn shunt_dry(input: &Stereo, latency: usize) -> Stereo {
    let mut shunt = BypassShunt::new(QUANTUM, latency);
    let frames = input.left.len();
    let mut dry = Stereo {
        left: vec![0.0; frames],
        right: vec![0.0; frames],
    };
    let mut start = 0;
    while start < frames {
        let end = (start + QUANTUM).min(frames);
        shunt.capture(&input.left[start..end], &input.right[start..end]);
        let (left, right) = shunt.dry();
        dry.left[start..end].copy_from_slice(&left[..end - start]);
        dry.right[start..end].copy_from_slice(&right[..end - start]);
        start = end;
    }
    dry
}

/// The largest gain reduction, in dB, of `output` against `input` delayed by `latency`, over
/// frames from `from` on, read where the input is at least -26 dBFS so that the ratio is exact to
/// far better than the 0.01 dB the calibration needs.
pub fn peak_reduction_db(input: &Stereo, output: &Stereo, latency: usize, from: usize) -> f64 {
    let mut worst = 0.0_f64;
    for (x, y) in [(&input.left, &output.left), (&input.right, &output.right)] {
        for n in from.max(latency)..y.len() {
            let source = f64::from(x[n - latency]);
            if source.abs() >= 0.05 {
                let gain = f64::from(y[n]) / source;
                worst = worst.max(-20.0 * gain.abs().log10());
            }
        }
    }
    worst
}

/// RMS of both channels of `x` from `from + offset` on.
fn rms(x: &Stereo, offset: usize, from: usize) -> f64 {
    let (mut sum, mut count) = (0.0, 0.0);
    for lane in [&x.left, &x.right] {
        for &v in &lane[from + offset..] {
            sum += f64::from(v) * f64::from(v);
            count += 1.0;
        }
    }
    (sum / count).sqrt()
}

/// Pearson correlation of the two channels from `from` on.
pub fn channel_correlation(x: &Stereo, from: usize) -> f64 {
    let (mut lr, mut ll, mut rr) = (0.0, 0.0, 0.0);
    for n in from..x.left.len() {
        let (l, r) = (f64::from(x.left[n]), f64::from(x.right[n]));
        lr += l * r;
        ll += l * l;
        rr += r * r;
    }
    lr / (ll * rr).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tone(rate: u32, frames: usize, amplitude: f64) -> Stereo {
        Stereo::dual_mono(
            &(0..frames)
                .map(|n| {
                    amplitude * (core::f64::consts::TAU * 220.0 * n as f64 / f64::from(rate)).sin()
                })
                .collect::<Vec<_>>(),
        )
    }

    /// The shunt delays by exactly the declared latency, and a calibrated compressor reduces a
    /// steady tone by the calibration target.
    #[test]
    fn the_shunt_is_the_latency_matched_input_and_the_calibration_lands() {
        let rate = 48_000;
        let input = tone(rate, rate as usize, 0.5);
        let limiter = Setting {
            effect: Effect::Limiter,
            overrides: vec![("ceiling", -12.0)],
            link: LinkMode::DualMono,
        };
        let latency = limiter.latency(rate);
        assert!(latency > 0);
        let dry = shunt_dry(&input, latency);
        assert!(dry.left[..latency].iter().all(|&v| v == 0.0));
        assert!(
            dry.left[latency..]
                .iter()
                .zip(&input.left)
                .all(|(a, b)| a.to_bits() == b.to_bits())
        );
        let compressor = Setting::calibrate(Effect::Compressor, rate, &input, LinkMode::DualMono);
        let wet = compressor.render(rate, &input);
        let reduction = peak_reduction_db(&input, &wet, 0, settle(rate));
        assert!((reduction - REDUCTION_DB).abs() < 0.01, "{reduction}");
    }
}
