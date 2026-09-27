//! The `console_mixing_automation` row (issue #1003): what it automates, how each control reaches
//! the engine, and the premises its three arms rest on.
//!
//! # Why the definition lives here and not in the bench
//!
//! Three drivers read it: the native bench row, `tests/automation.rs` (the row's premises on every
//! change) and the browser arm, which rides the shipped `host_web.wasm` under V8 and takes this
//! table through the `mixing_automation_controls` example. A row whose controls were transcribed
//! three times would be three rows, so the table, the bases and the per-block values exist once.
//!
//! # The row
//!
//! The mono console (`sixty_four_track_console_mono`, the fixture as written) prepared with the
//! live-console control channel, with eight of its sixty-four tracks each riding one control: one
//! track per eight-lane bank, and every other four-lane bank.
//!
//! * **EQ band-1 gain** on `ch00`, `ch24`, `ch48`, pushed as **one owner edit on `Both`** -- the
//!   SDK's lowering (`prepared-control.js` -> `eq_target_prepare`), which designs one `Both` target
//!   for a symmetric edit and so keeps a mono track's collapse.
//! * **Compressor threshold** on `ch08`, `ch32`, `ch56` and **limiter ceiling** on `ch16`, `ch40`,
//!   pushed as a **Left then a Right `Parameter` record** with one value -- the web host's lowering
//!   of `channel = 2` for a per-lane parameter (`into_effect_records`).
//!
//! Every base is the value the track already holds, read from the session model, so `restated`
//! restates exactly what is held and must render `quiet`'s bits. `automated` alternates
//! `base + step` and `base - step`, clamped into the parameter's domain, so every block opens a
//! window.
//!
//! The limiter's step is not the diagnosis draft's `0.25` dB. A ceiling ridden a quarter decibel
//! around `-1.0` or `-1.75` dB never engages on this fixture's tone, so the limiter would take the
//! ramp and move no bit, and a whole-mix digest inequality could not say whether its ramps ran
//! (VERIFY-AUTOMATION F6). Its step is the smallest on a fixed ladder that moves the limiter-only
//! arm's bits natively *and* in the browser arm; [`LIMITER_STEP_DB`] records the ladder and the one
//! limiter track that cannot engage at any ceiling.

use bench_support::digest::Sha256Sink;
use effect_compiler::EffectRack;
use effect_contract::{EffectDescriptor, ParameterChannel};
use lane::Backend;
use session::ParameterChannel as SessionChannel;

use crate::{ObservationArm, PlanConfig, RenderFailed, SessionRuntime, Workload, console_model};

/// The session every arm renders: the mono fixture as written.
pub const WORKLOAD: Workload = Workload::SixtyFourTrackConsoleMono;

/// The live-console control channel and nothing else, on every arm, so the arms differ only in
/// the traffic that rides the channel.
pub const CONFIG: PlanConfig = PlanConfig {
    meters: false,
    control: true,
    observation: ObservationArm::Absent,
};

/// Untimed blocks every arm renders after its settling push and before anything is compared or
/// timed. Far more than the 64 samples a window takes.
pub const PREROLL_BLOCKS: u64 = 64;

/// Blocks the preflight compares after the pre-roll. The native tone is one frozen block repeated,
/// so a ride that engages on it engages early.
pub const PREFLIGHT_BLOCKS: u64 = 64;

/// The smoothing window every automated parameter declares. Checked against the descriptors by
/// [`MixingAutomation::resolve`], so the record states the window its surcharge is the cost of.
pub const SMOOTHING_SAMPLES: u32 = 64;

/// The EQ band-1 gain step, in dB: small enough to be ordinary console traffic, large enough that
/// the designed coefficient words move (a one-ULP step does not; see `console_hoist`).
pub const EQ_STEP_DB: f32 = 0.25;

/// The compressor threshold step, in dB: the diagnosis's, and the `automated_compressor_only`
/// preflight arm shows it moves rendered bits.
pub const COMPRESSOR_STEP_DB: f32 = 0.5;

/// The limiter ceiling step, in dB.
///
/// Chosen once, before timing, on the ladder `0.25, 1, 2, 4, 8, 12, 16, 20`, as the smallest step
/// whose `automated_limiter_only` preflight arm moves rendered bits in both arms, native and V8.
/// `base + step` clamps to the domain's `0` dB top, so the ride alternates `0` and `base - 8`.
///
/// It moves bits through `ch40` (held `-1.75` dB), which engages from a `-9.75` dB ceiling on
/// both arms. `ch16` (held `-1.0` dB) never engages natively anywhere in the ceiling's `[-24, 0]`
/// domain: its compressor (threshold `-30` dB, ratio `6.75`) holds the track below `-24` dBFS on
/// the native tone. Its ride still takes the ramp, so it is priced, but it moves no bit, and the
/// premise is therefore stated per effect (VERIFY-AUTOMATION A3), never per track.
pub const LIMITER_STEP_DB: f32 = 8.0;

/// The effect an automated control belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effect {
    /// `miso.parametric-eq`, band-1 gain.
    Eq,
    /// `miso.compressor`, threshold.
    Compressor,
    /// `miso.true-peak-limiter`, ceiling.
    Limiter,
}

impl Effect {
    /// The effect's contract id.
    #[must_use]
    pub const fn contract_id(self) -> &'static str {
        match self {
            Self::Eq => "miso.parametric-eq",
            Self::Compressor => "miso.compressor",
            Self::Limiter => "miso.true-peak-limiter",
        }
    }

    /// How a real host pushes a both-channel edit of this effect's parameter.
    #[must_use]
    pub const fn lowering(self) -> Lowering {
        match self {
            Self::Eq => Lowering::OwnerBoth,
            Self::Compressor | Self::Limiter => Lowering::LeftThenRight,
        }
    }

    /// The record-side name of this effect.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Eq => "eq",
            Self::Compressor => "compressor",
            Self::Limiter => "limiter",
        }
    }
}

/// How one both-channel edit reaches the effect's control channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lowering {
    /// One owner edit on [`ParameterChannel::Both`]. The owner accepts `(PerLane, Both)` and a
    /// symmetric edit designs one `Both` target, which the channel-symmetry witness preserves.
    OwnerBoth,
    /// A [`ParameterChannel::Left`] then a [`ParameterChannel::Right`] `Parameter` record with the
    /// same value: the web host's `(PerLane, channel = 2)` arm.
    LeftThenRight,
}

impl Lowering {
    /// The record-side name of this lowering.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::OwnerBoth => "owner_both",
            Self::LeftThenRight => "left_then_right",
        }
    }

    /// The channels one edit is pushed on, in push order.
    #[must_use]
    pub const fn channels(self) -> &'static [ParameterChannel] {
        match self {
            Self::OwnerBoth => &[ParameterChannel::Both],
            Self::LeftThenRight => &[ParameterChannel::Left, ParameterChannel::Right],
        }
    }
}

/// One automated control, by stable identity.
#[derive(Clone, Copy, Debug)]
pub struct Control {
    /// Session-stable track id.
    pub track_id: &'static str,
    /// The effect whose parameter rides.
    pub effect: Effect,
    /// Zero-based index into the effect descriptor's parameter table (not the wire id).
    pub parameter_index: u32,
    /// The parameter's descriptor name, checked by [`MixingAutomation::resolve`].
    pub parameter: &'static str,
    /// The ride's half-width, in the parameter's unit.
    pub step: f32,
}

const fn eq(track_id: &'static str) -> Control {
    Control {
        track_id,
        effect: Effect::Eq,
        parameter_index: 3,
        parameter: "band-1-gain",
        step: EQ_STEP_DB,
    }
}

const fn compressor(track_id: &'static str) -> Control {
    Control {
        track_id,
        effect: Effect::Compressor,
        parameter_index: 0,
        parameter: "threshold",
        step: COMPRESSOR_STEP_DB,
    }
}

const fn limiter(track_id: &'static str) -> Control {
    Control {
        track_id,
        effect: Effect::Limiter,
        parameter_index: 0,
        parameter: "ceiling",
        step: LIMITER_STEP_DB,
    }
}

/// The eight automated controls, in track order: one track per eight-lane bank.
pub const CONTROLS: [Control; 8] = [
    eq("ch00"),
    compressor("ch08"),
    limiter("ch16"),
    eq("ch24"),
    compressor("ch32"),
    limiter("ch40"),
    eq("ch48"),
    compressor("ch56"),
];

/// Which traffic an arm delivers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    /// Nothing is ever pushed, not even the settling write.
    Quiet,
    /// Every control restated at the value it holds, every block.
    Restated,
    /// Every control alternating `base + step` and `base - step`, every block.
    Automated,
}

/// The arms, in record order.
pub const ARMS: [Arm; 3] = [Arm::Quiet, Arm::Restated, Arm::Automated];

impl Arm {
    /// The record-side name of this arm.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Quiet => "quiet",
            Self::Restated => "restated",
            Self::Automated => "automated",
        }
    }
}

/// One control resolved against a prepared runtime.
#[derive(Clone, Debug)]
pub struct ResolvedControl {
    /// The control this resolves.
    pub control: Control,
    /// The runtime's control channel for `(track_id, effect contract id)`.
    pub channel: usize,
    /// The track's position in the session model: the web host's `trackIndex`.
    pub track_index: usize,
    /// The effect's session slot id.
    pub slot_id: String,
    /// The rack the effect sits in: `0` simd1, `1` dynamic, `2` simd2 (the wire's `rack`).
    pub rack: u8,
    /// The effect's position within its rack (the wire's `effectIndex`).
    pub effect_index: u32,
    /// The parameter's wire id (the wire's `parameterId`).
    pub parameter_id: u32,
    /// The value the track holds, which `restated` restates.
    pub base: f32,
    /// What `automated` pushes before an even block and before an odd block.
    pub values: [f32; 2],
}

impl ResolvedControl {
    /// The value `arm` pushes before `block`, or `None` if it pushes nothing.
    #[must_use]
    pub fn value(&self, arm: Arm, block: u64) -> Option<f32> {
        match arm {
            Arm::Quiet => None,
            Arm::Restated => Some(self.base),
            Arm::Automated => Some(self.values[usize::from(!block.is_multiple_of(2))]),
        }
    }
}

/// Why a control could not be resolved. Every variant is a defect in the row, never a condition
/// a measurement may report around.
#[derive(Clone, Debug, PartialEq)]
pub enum ResolveError {
    /// No prepared control channel carries `(track_id, effect)`.
    MissingChannel {
        /// The track the row names.
        track_id: &'static str,
        /// The effect contract id the row names.
        effect: &'static str,
    },
    /// The session model does not carry the slot the channel addresses.
    MissingSlot {
        /// The track the row names.
        track_id: &'static str,
    },
    /// The descriptor has no such parameter, it is not the one the row names, it is not
    /// automatable, or its window is not [`SMOOTHING_SAMPLES`].
    Parameter {
        /// The track the row names.
        track_id: &'static str,
    },
    /// The track's two channels hold different values, so a both-channel restatement would be an
    /// edit.
    AsymmetricHold {
        /// The track the row names.
        track_id: &'static str,
    },
    /// The ride does not move: its two values are equal, or one of them is the base.
    Stationary {
        /// The track the row names.
        track_id: &'static str,
    },
}

/// Pushes the row accepted and pushes it attempted.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PushTally {
    /// `push_parameter` calls made.
    pub attempted: u64,
    /// `push_parameter` calls the bounded queue or the owner accepted.
    pub accepted: u64,
}

/// The eight controls, resolved against one runtime.
#[derive(Clone, Debug)]
pub struct MixingAutomation {
    controls: Vec<ResolvedControl>,
}

impl MixingAutomation {
    /// Resolves every control by `(track_id, effect contract id)`, reads its held value from the
    /// session model and checks the parameter is what the row claims.
    ///
    /// # Errors
    ///
    /// Any [`ResolveError`]: a missing channel or slot, a parameter the row cannot ride, a held
    /// value that differs between the channels, or a ride that would not move.
    pub fn resolve(runtime: &SessionRuntime) -> Result<Self, ResolveError> {
        let model = console_model(WORKLOAD);
        let mut controls = Vec::with_capacity(CONTROLS.len());
        for control in CONTROLS {
            let effect = control.effect.contract_id();
            let channel = runtime.control_channel(control.track_id, effect).ok_or(
                ResolveError::MissingChannel {
                    track_id: control.track_id,
                    effect,
                },
            )?;
            let producer = &runtime.controls[channel];
            let descriptor: &'static EffectDescriptor = producer.descriptor;
            let parameter = descriptor
                .parameters
                .get(control.parameter_index as usize)
                .filter(|parameter| {
                    parameter.display_name == control.parameter
                        && parameter.automatable
                        && parameter.smoothing_samples == SMOOTHING_SAMPLES
                })
                .ok_or(ResolveError::Parameter {
                    track_id: control.track_id,
                })?;
            let missing_slot = ResolveError::MissingSlot {
                track_id: control.track_id,
            };
            let (track_index, track) = model
                .tracks
                .iter()
                .enumerate()
                .find(|(_, track)| track.id.as_str() == control.track_id)
                .ok_or_else(|| missing_slot.clone())?;
            let rack = match producer.rack {
                EffectRack::Simd1 => (0, &track.simd1),
                EffectRack::Dynamic => (1, &track.dynamic),
                EffectRack::Simd2 => (2, &track.simd2),
            };
            let slot = rack
                .1
                .effects
                .get(producer.effect_index as usize)
                .filter(|slot| slot.id.as_str() == &*producer.effect_id)
                .ok_or(missing_slot)?;
            let held = |lane: SessionChannel| {
                slot.params
                    .iter()
                    .find(|param| {
                        param.parameter_id == parameter.id.0
                            && (param.channel == lane || param.channel == SessionChannel::Both)
                    })
                    .map_or(parameter.default_value, |param| param.value)
            };
            let (left, right) = (held(SessionChannel::Left), held(SessionChannel::Right));
            if left.to_bits() != right.to_bits() {
                return Err(ResolveError::AsymmetricHold {
                    track_id: control.track_id,
                });
            }
            let clamp = |value: f32| {
                let low = parameter.minimum.unwrap_or(value);
                let high = parameter.maximum.unwrap_or(value);
                value.clamp(low, high)
            };
            let values = [clamp(left + control.step), clamp(left - control.step)];
            if values[0].to_bits() == values[1].to_bits()
                || values.iter().any(|value| value.to_bits() == left.to_bits())
            {
                return Err(ResolveError::Stationary {
                    track_id: control.track_id,
                });
            }
            controls.push(ResolvedControl {
                control,
                channel,
                track_index,
                slot_id: producer.effect_id.to_string(),
                rack: rack.0,
                effect_index: producer.effect_index,
                parameter_id: parameter.id.0,
                base: left,
                values,
            });
        }
        Ok(Self { controls })
    }

    /// The resolved controls, in [`CONTROLS`] order.
    #[must_use]
    pub fn controls(&self) -> &[ResolvedControl] {
        &self.controls
    }

    /// `push_parameter` calls one block of traffic makes for `effects` (`None` is every effect).
    #[must_use]
    pub fn pushes_per_block(&self, effects: Option<Effect>) -> u64 {
        self.selected(effects)
            .map(|control| control.control.effect.lowering().channels().len() as u64)
            .sum()
    }

    fn selected(&self, effects: Option<Effect>) -> impl Iterator<Item = &ResolvedControl> {
        self.controls
            .iter()
            .filter(move |control| effects.is_none_or(|effect| control.control.effect == effect))
    }

    /// Pushes every selected control's `value(arm, block)` in its host lowering. Off the clock.
    pub fn push(
        &self,
        runtime: &mut SessionRuntime,
        arm: Arm,
        block: u64,
        effects: Option<Effect>,
    ) -> PushTally {
        let mut tally = PushTally::default();
        for control in self.selected(effects) {
            let Some(value) = control.value(arm, block) else {
                continue;
            };
            for &side in control.control.effect.lowering().channels() {
                tally.attempted += 1;
                if runtime.push_parameter(
                    control.channel,
                    control.control.parameter_index,
                    side,
                    value,
                ) {
                    tally.accepted += 1;
                }
            }
        }
        tally
    }

    /// Pushes every selected control at its base once: the settling write of the pre-roll.
    pub fn settle(&self, runtime: &mut SessionRuntime, effects: Option<Effect>) -> PushTally {
        self.push(runtime, Arm::Restated, 0, effects)
    }
}

/// One prepared arm of the row: its runtime, its resolved controls and the traffic it delivers.
pub struct MixingArm {
    /// The traffic this arm delivers.
    pub arm: Arm,
    /// The effects whose controls it drives (`None` is all eight controls).
    pub effects: Option<Effect>,
    /// The prepared console.
    pub runtime: SessionRuntime,
    /// The eight controls, resolved against [`MixingArm::runtime`].
    pub automation: MixingAutomation,
    /// Every push after the settling write.
    pub tally: PushTally,
    /// The next block to render; the pre-roll renders blocks `0..PREROLL_BLOCKS`.
    pub block: u64,
}

impl MixingArm {
    /// Builds the mono console at `dispatch`, resolves the controls, settles every driven control
    /// at its base (except on `quiet`, which never writes), and renders the untimed pre-roll.
    ///
    /// # Panics
    ///
    /// If a control does not resolve, the settling write is refused, or a pre-roll block fails:
    /// each is a defect in the row, not a condition to measure around.
    #[must_use]
    pub fn prepare(arm: Arm, effects: Option<Effect>, dispatch: Backend) -> Self {
        let mut runtime = SessionRuntime::build_with_dispatch(WORKLOAD, CONFIG, dispatch);
        let automation = MixingAutomation::resolve(&runtime)
            .unwrap_or_else(|error| panic!("console_mixing_automation: {error:?}"));
        if arm != Arm::Quiet {
            let settled = automation.settle(&mut runtime, effects);
            assert_eq!(
                settled.accepted, settled.attempted,
                "console_mixing_automation: the settling write was refused"
            );
        }
        for block in 0..PREROLL_BLOCKS {
            runtime
                .render(block)
                .expect("console_mixing_automation pre-roll");
        }
        Self {
            arm,
            effects,
            runtime,
            automation,
            tally: PushTally::default(),
            block: PREROLL_BLOCKS,
        }
    }

    /// Pushes this block's traffic. Off the clock.
    pub fn drive(&mut self) {
        let tally = self
            .automation
            .push(&mut self.runtime, self.arm, self.block, self.effects);
        self.tally.attempted += tally.attempted;
        self.tally.accepted += tally.accepted;
    }

    /// Renders the next block. The only call a driver may put inside a clock.
    ///
    /// # Errors
    ///
    /// [`RenderFailed`] when the plan refused the block.
    pub fn render(&mut self) -> Result<(), RenderFailed> {
        let result = self.runtime.render(self.block);
        self.block += 1;
        result
    }
}

/// What the untimed preflight observed: seven arms over the pre-roll and [`PREFLIGHT_BLOCKS`].
#[derive(Clone, Debug)]
pub struct Preflight {
    /// The controls as the `quiet` arm resolved them.
    pub controls: Vec<ResolvedControl>,
    /// Output digests over the compared blocks, per [`PREFLIGHT_ARMS`] entry.
    pub digests: Vec<String>,
    /// `bank_collapse_counters` after the run, per [`PREFLIGHT_ARMS`] entry.
    pub collapse: Vec<[u64; 2]>,
    /// Pushes per [`PREFLIGHT_ARMS`] entry.
    pub tallies: Vec<PushTally>,
    /// Blocks each arm rendered: the pre-roll and the compared blocks.
    pub blocks: u64,
}

/// The preflight's arms, in [`Preflight`] order: the row's three, the three single-effect
/// variants of `automated` (VERIFY-AUTOMATION A3) and an EQ-only `restated` (A6).
pub const PREFLIGHT_ARMS: [(Arm, Option<Effect>); 7] = [
    (Arm::Quiet, None),
    (Arm::Restated, None),
    (Arm::Automated, None),
    (Arm::Automated, Some(Effect::Eq)),
    (Arm::Automated, Some(Effect::Compressor)),
    (Arm::Automated, Some(Effect::Limiter)),
    (Arm::Restated, Some(Effect::Eq)),
];

/// The record-side name of a [`PREFLIGHT_ARMS`] entry.
#[must_use]
pub fn preflight_arm_name(arm: Arm, effects: Option<Effect>) -> String {
    effects.map_or_else(
        || arm.name().to_owned(),
        |effect| format!("{}_{}_only", arm.name(), effect.name()),
    )
}

/// Builds every [`PREFLIGHT_ARMS`] entry at `dispatch`, renders the pre-roll and
/// [`PREFLIGHT_BLOCKS`] blocks of each arm's traffic, and reads what the premises need. Untimed.
///
/// # Panics
///
/// As [`MixingArm::prepare`], and if a compared block fails to render.
#[must_use]
pub fn preflight(dispatch: Backend) -> Preflight {
    let mut arms: Vec<MixingArm> = PREFLIGHT_ARMS
        .iter()
        .map(|&(arm, effects)| MixingArm::prepare(arm, effects, dispatch))
        .collect();
    let mut hashes: Vec<Sha256Sink> = arms.iter().map(|_| Sha256Sink::new()).collect();
    for _ in 0..PREFLIGHT_BLOCKS {
        for (arm, hash) in arms.iter_mut().zip(&mut hashes) {
            arm.drive();
            arm.render()
                .expect("console_mixing_automation preflight block");
            arm.runtime.hash_output(hash);
        }
    }
    Preflight {
        controls: arms[0].automation.controls().to_vec(),
        digests: hashes.into_iter().map(Sha256Sink::finish_hex).collect(),
        collapse: arms
            .iter()
            .map(|arm| arm.runtime.bank_collapse_counters())
            .collect(),
        tallies: arms.iter().map(|arm| arm.tally).collect(),
        blocks: PREROLL_BLOCKS + PREFLIGHT_BLOCKS,
    }
}

impl Preflight {
    fn index(arm: Arm, effects: Option<Effect>) -> usize {
        PREFLIGHT_ARMS
            .iter()
            .position(|entry| *entry == (arm, effects))
            .expect("a preflight arm")
    }

    /// The digest of one [`PREFLIGHT_ARMS`] entry.
    #[must_use]
    pub fn digest(&self, arm: Arm, effects: Option<Effect>) -> &str {
        &self.digests[Self::index(arm, effects)]
    }

    /// The collapse counters of one [`PREFLIGHT_ARMS`] entry.
    #[must_use]
    pub fn collapse(&self, arm: Arm, effects: Option<Effect>) -> [u64; 2] {
        self.collapse[Self::index(arm, effects)]
    }

    /// Asserts the row's premises. Panics on the first one that fails.
    ///
    /// * Every push is accepted.
    /// * `quiet` collapses every cohort on every block: the row is a statement about the collapse.
    /// * `restated` renders `quiet`'s bits, and `automated` does not.
    /// * Each single-effect `automated` renders other bits than `restated`: every automated effect
    ///   moves bits, so the whole-mix inequality is not carried by one effect alone.
    /// * An EQ-only `restated` keeps every cohort collapsed: the EQ is pushed in the SDK's `Both`
    ///   shape, not as the two one-channel targets a harness-only lowering would design.
    pub fn assert_premises(&self) {
        for (index, tally) in self.tallies.iter().enumerate() {
            let (arm, effects) = PREFLIGHT_ARMS[index];
            assert_eq!(
                tally.accepted,
                tally.attempted,
                "preflight {}: a push was refused",
                preflight_arm_name(arm, effects)
            );
        }
        let quiet = self.collapse(Arm::Quiet, None);
        assert!(
            quiet[1] > 0 && quiet[0] == quiet[1] * self.blocks,
            "preflight quiet: the mono console did not collapse every cohort on every block \
             ({quiet:?} over {} blocks)",
            self.blocks
        );
        let restated = self.digest(Arm::Restated, None);
        assert_eq!(
            self.digest(Arm::Quiet, None),
            restated,
            "preflight: restating the held values moved a rendered bit"
        );
        assert_ne!(
            self.digest(Arm::Automated, None),
            restated,
            "preflight: the automated arm rendered the restated arm's bits"
        );
        for effect in [Effect::Eq, Effect::Compressor, Effect::Limiter] {
            assert_ne!(
                self.digest(Arm::Automated, Some(effect)),
                restated,
                "preflight: automating the {} alone moved no rendered bit",
                effect.name()
            );
        }
        assert_eq!(
            self.collapse(Arm::Restated, Some(Effect::Eq)),
            quiet,
            "preflight: restating the EQ in the SDK's Both shape retired a cohort's collapse"
        );
    }
}
