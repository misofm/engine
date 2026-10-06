//! Issue #1087 (console strip P1), gate 2: a shunt-bypassed lane is bit-identical to the same
//! instance prepared with `bypass = true` and rendered per node, which is how every session bypass
//! rendered before #1087.
//!
//! Preparation now lowers a session's `bypass` (on every effect but the delay and, since #1100,
//! the multiband) to a prepared `bypass = false` plus the lane's
//! initial shunt state (`EffectControlLane::without_channel`), so a bypassed track stays in its
//! effect bank. The bank runs every lane's wet path and the rack's `LiveControlEffectBankStage`
//! restores the latency-matched dry signal into exactly the bypassed lanes; a lane that ends up per
//! node takes the graph's per-node shunt (`runtime::LiveControlEffect`), which this file re-enacts
//! with the contract's own `BypassShunt` in the order that node runs it.
//!
//! The oracle is each lane's own scalar instance, prepared from the same request with the lane's
//! bypass as the *prepared* flag: today's per-node prepared-bypass render. Every launch effect and
//! every quality row and link mode it declares at 48 kHz is covered, with random parameters per
//! lane. The input carries random signal, runs of `-0.0`, subnormals, exact silence (which engages
//! the banks' silent fast paths) and, per effect, the first blocks of its latency line: the
//! true-peak limiter's `rate/100 + 6 = 486` samples and the soft clip's 31. Enabled lanes are
//! compared with their enabled scalar instance in the same run. NaNs fold to one value in every
//! comparison (decision 10).
//!
//! # The input domain
//!
//! An effect in a compiled plan only ever receives finite samples below `1e30` in magnitude: the
//! track's input stage sanitises everything else to `+0.0`, and every stage after it zeroes a block
//! that leaves that range (D7, `effect_runtime::bank::BLOCK_LIMIT`), so by induction every effect
//! input is inside it, and a shunt only ever emits its input. On that domain the per-node leg holds
//! up to `9.9e29`. Outside it the two paths differ by design -- a prepared bypass runs the effect's
//! own D7 check over the dry block and zeroes it, the shunt copies it -- and
//! `bypass_cohorts::non_finite_sources_render_todays_bits` gates that no source sample reaches an
//! effect there.
//!
//! The width is the host's bank width (`Simd8` on x86-64, `Simd4` on AArch64), so the same file is
//! the `Simd4` leg under `scripts/run-aarch64-tests.sh`.

use effect_compiler::{NEVER_BANKED_EFFECTS, launch_native_effect_registry};
use effect_contract::{
    BankWidth, EffectControlLane, EffectControlRecord, EffectDescriptor, EffectProcessBlock,
    EffectQuality, InitialParameterValue, LinkMode, ParameterChannel, ParameterChannelPolicy,
    ParameterDomain, ParameterUnit, PortRole, PrepareEffectBankRequest, PrepareEffectLimits,
    PrepareEffectRequest, PreparedNativeEffect, PreparedPorts, PreparedSidechainPort,
};
use engine::realtime::{QueueGeneration, bounded_spsc};
use graph_compiler::Backend;
use rack::{AoSoaScratch, BankChain, BankMembers, BankSlot, BankStage, LiveControlEffectBankStage};

const RATE: u32 = 48_000;
const QUANTUM: u32 = 128;
/// Sixteen blocks: past the limiter's four-block latency line, and through every input segment.
const BLOCKS: u64 = 16;

/// Every launch effect, by registry id.
const EFFECTS: [&str; 8] = [
    "miso.parametric-eq",
    "miso.compressor",
    "miso.gate-expander",
    "miso.multiband-compressor",
    "miso.true-peak-limiter",
    "miso.soft-clip",
    "miso.transient-shaper",
    "miso.delay",
];

fn width() -> BankWidth {
    BankWidth::for_backend(Backend::current())
        .expect("the delivery host has a bank width; the evidence is vacuous otherwise")
}

/// xorshift64*: deterministic and the same on every target.
struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self {
        let mut rng = Self(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1);
        for _ in 0..4 {
            rng.next();
        }
        rng
    }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }
    /// Uniform in `[0, 1)`.
    fn unit(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1_u64 << 24) as f32
    }
    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }
}

/// One lane's parameters: a random value in each parameter's declared domain, per channel.
fn initial_values(descriptor: &EffectDescriptor, rng: &mut Rng) -> Vec<InitialParameterValue> {
    let mut values = Vec::new();
    for (index, parameter) in descriptor.parameters.iter().enumerate() {
        let draw = |rng: &mut Rng| -> f32 {
            let value = match parameter.domain {
                ParameterDomain::Continuous => {
                    let (low, high) = (
                        parameter.minimum.expect("continuous minimum"),
                        parameter.maximum.expect("continuous maximum"),
                    );
                    // Times lean short, so attack, hold and release act inside a sixteen-block run.
                    let unit = match parameter.unit {
                        ParameterUnit::Milliseconds => {
                            let unit = rng.unit();
                            unit * unit * unit
                        }
                        _ => rng.unit(),
                    };
                    low + (high - low) * unit
                }
                ParameterDomain::Boolean => (rng.below(2)) as f32,
                ParameterDomain::Enumeration => {
                    parameter.enum_choices[rng.below(parameter.enum_choices.len() as u64) as usize]
                        .value
                }
            };
            let value = effect_contract::normalize_zero(value);
            if effect_contract::parameter_value_valid(parameter, value) {
                value
            } else {
                parameter.default_value
            }
        };
        match parameter.channel_policy {
            ParameterChannelPolicy::Shared => values.push(InitialParameterValue {
                parameter_index: index as u32,
                channel: ParameterChannel::Both,
                value: draw(rng),
            }),
            ParameterChannelPolicy::PerLane => {
                for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                    values.push(InitialParameterValue {
                        parameter_index: index as u32,
                        channel,
                        value: draw(rng),
                    });
                }
            }
        }
    }
    values
}

/// Every parameter at its declared default, per channel, as `effect-compiler` fills an unset one.
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

/// The ports `effect-compiler` prepares for an effect with no sidechain route.
fn unrouted_ports(descriptor: &EffectDescriptor) -> PreparedPorts {
    match descriptor
        .ports
        .iter()
        .find(|port| port.role == PortRole::SidechainInput)
    {
        Some(port) if !port.required => PreparedPorts {
            sidechain: PreparedSidechainPort::Unconnected {
                id: port.id,
                required: false,
            },
        },
        Some(_) => panic!(
            "{}: a required sidechain has no unrouted form",
            descriptor.id
        ),
        None => PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
    }
}

/// One effect configuration under test: a quality row and a link mode the effect declares.
#[derive(Clone, Copy, Debug)]
struct Case {
    quality: EffectQuality,
    link_mode: LinkMode,
}

fn cases(descriptor: &EffectDescriptor) -> Vec<Case> {
    let mut cases = Vec::new();
    for row in descriptor
        .qualities
        .iter()
        .filter(|row| row.sample_rate == RATE)
    {
        for link_mode in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
            if descriptor.supported_link_modes.contains(link_mode) {
                cases.push(Case {
                    quality: row.quality,
                    link_mode,
                });
            }
        }
    }
    assert!(!cases.is_empty(), "{} declares a 48 kHz row", descriptor.id);
    cases
}

fn request<'a>(
    descriptor: &EffectDescriptor,
    case: Case,
    bypass: bool,
    values: &'a [InitialParameterValue],
) -> PrepareEffectRequest<'a> {
    PrepareEffectRequest {
        sample_rate: RATE,
        quantum: QUANTUM,
        quality: case.quality,
        bypass,
        link_mode: case.link_mode,
        ports: unrouted_ports(descriptor),
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 1 << 30,
            maximum_scratch_bytes: 1 << 28,
            maximum_automation_spans_per_block: 32,
        },
    }
}

/// How loud a lane's input may get.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Level {
    /// Up to about `+32 dBFS`: hard enough to drive every dynamics processor and clipper.
    Loud,
    /// Loud, and every fourth block scaled towards the top of the domain an effect can receive in
    /// a compiled plan (`|x| <= 9.9e29`), where a wet path's own D7 check fires.
    Extreme,
}

/// One lane's input for one block: random signal, a `-0.0` run, exact silence and signal again,
/// in segments whose boundaries are staggered per lane so a bank sees mixed segments.
fn input_block(lane: usize, channel: usize, block: u64, level: Level, rng: &mut Rng) -> Vec<f32> {
    let segment = (block + lane as u64) % 8;
    let extreme = level == Level::Extreme && block % 4 == 1;
    (0..QUANTUM)
        .map(|frame| match segment {
            // A negative-zero run: the dry signal must come out `-0.0` at the declared latency,
            // and an arithmetic select (`0 * wet + dry`) would make it `+0.0` wherever the wet
            // path still rings.
            3 | 4 => -0.0,
            // Exact silence, which engages the effects' silent fast paths when a whole bank is
            // silent.
            6 => 0.0,
            // Quiet signal, about 60 dB down, which closes gates and expanders.
            5 if !extreme => 1.0e-3 * (2.0 * rng.unit() - 1.0),
            _ => {
                let draw = rng.unit();
                let value = match rng.below(64) {
                    0 => -0.0,
                    1 => 0.0,
                    2 => f32::from_bits(1 + (rng.next() & 0x7f_ffff) as u32),
                    3 => -f32::from_bits(1 + (rng.next() & 0x7f_ffff) as u32),
                    4 => 40.0 * (2.0 * draw - 1.0),
                    _ => {
                        let polarity = if (frame + channel as u32).is_multiple_of(2) {
                            1.0
                        } else {
                            -1.0
                        };
                        polarity * (1.6 * draw - 0.3)
                    }
                };
                if extreme {
                    (value * 9.0e28).clamp(-9.9e29, 9.9e29)
                } else {
                    value
                }
            }
        })
        .collect()
}

/// Word equality with every NaN folded to one value (decision 10).
fn folded(value: f32) -> u32 {
    if value.is_nan() {
        0x7fc0_0000
    } else {
        value.to_bits()
    }
}

fn assert_bits(what: &str, actual: &[f32], expected: &[f32]) {
    assert_eq!(actual.len(), expected.len(), "{what}: length");
    for (index, (a, e)) in actual.iter().zip(expected).enumerate() {
        assert_eq!(
            folded(*a),
            folded(*e),
            "{what}: sample {index} is {a:e} ({:#010x}), today's prepared bypass renders {e:e} \
             ({:#010x})",
            a.to_bits(),
            e.to_bits()
        );
    }
}

/// A scalar instance's render of one block, in place.
fn process_scalar(
    processor: &mut dyn PreparedNativeEffect,
    left: &mut [f32],
    right: &mut [f32],
    block: u64,
) {
    let block =
        EffectProcessBlock::new(left, right, None, block * u64::from(QUANTUM), &[], QUANTUM)
            .expect("block shape");
    let _ = processor.process(block);
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

/// A one-slot chain over `stage`, exactly as the graph runtime builds a single bound effect slot.
fn chain(stage: LiveControlEffectBankStage) -> BankChain {
    let lanes = width().lanes() as usize;
    BankChain::new(
        AoSoaScratch::new(width(), QUANTUM).expect("scratch"),
        vec![true; lanes].into_boxed_slice(),
        vec![BankSlot {
            stage: Box::new(stage) as Box<dyn BankStage>,
            active_lanes: vec![true; lanes].into_boxed_slice(),
        }],
    )
    .expect("chain")
}

/// The bypass patterns every bank is run under: a mix with both edge lanes bypassed, the
/// complement, one lone bypassed lane, and every lane bypassed.
fn patterns(lanes: usize) -> Vec<Vec<bool>> {
    vec![
        (0..lanes).map(|lane| lane % 3 != 1).collect(),
        (0..lanes).map(|lane| lane % 3 == 1).collect(),
        (0..lanes).map(|lane| lane == lanes / 2).collect(),
        vec![true; lanes],
    ]
}

/// Gate 2, banked leg.
///
/// For every bankable launch effect, quality row, link mode and bypass pattern, one bank of the
/// host's width is bound from requests prepared `bypass = false` (the lowering) and driven through
/// the rack's live stage with a channel-less lane on every bypassed track. Every lane's output
/// over sixteen blocks must equal its own scalar instance prepared with the lane's bypass as the
/// prepared flag. Each effect must also show that its bypass is audible here -- its enabled and
/// bypassed renders of a bypassed lane differ somewhere -- or the comparison proved nothing.
///
/// The input stays at [`Level::Loud`]. At [`Level::Extreme`] a bypassed lane stays bit-identical,
/// but its wet path can trip the whole-bank D7 recovery of the EQ, compressor, multiband, soft clip
/// and transient shaper, which zeroes and resets every lane of the bank: the lane coupling Sol's M2
/// names, which enabled lanes already have with each other and which P2b-P2e make per lane. The
/// spec's attempt-1 evidence records it.
///
/// Red mutations, each of which fails this test:
/// * the restore in `LiveControlEffectBankStage::process_inner` as an arithmetic select,
///   `fma(0, wet, dry)` -> a `-0.0` input run comes out `+0.0` wherever the wet path still rings;
/// * the shunt built only when a lane has a live channel (`has_channel()` without
///   `|| bypassed()`) -> no shunt, and a bypassed lane renders wet;
/// * `BypassShunt::capture` stops feeding its latency line -> the limiter's and the soft clip's
///   bypassed lanes emit undelayed words.
#[test]
fn a_shunt_bypassed_bank_lane_is_bit_identical_to_prepared_bypass() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let width = width();
    let lanes = width.lanes() as usize;
    let mut banked_effects = 0;
    for (effect_index, effect_id) in EFFECTS.iter().enumerate() {
        let factory = registry.get_shared_ascii(effect_id).expect("launch effect");
        let descriptor = factory.descriptor();
        let mut audible = 0_usize;
        let mut bound = false;
        for (case_index, case) in cases(descriptor).into_iter().enumerate() {
            for (pattern_index, bypassed) in patterns(lanes).into_iter().enumerate() {
                let seed = (effect_index * 1_000 + case_index * 10 + pattern_index) as u64;
                let mut rng = Rng::new(seed);
                let values: Vec<Vec<InitialParameterValue>> = (0..lanes)
                    .map(|_| initial_values(descriptor, &mut rng))
                    .collect();
                let requests: Vec<PrepareEffectRequest<'_>> = values
                    .iter()
                    .map(|values| request(descriptor, case, false, values))
                    .collect();
                let Some(bank) = factory
                    .bind_homogeneous_bank(PrepareEffectBankRequest {
                        backend: Backend::current(),
                        width,
                        requests: &requests,
                        active_mask: width.full_mask(),
                    })
                    .expect("a well-formed bank request")
                else {
                    assert!(
                        NEVER_BANKED_EFFECTS.contains(effect_id),
                        "{effect_id} declined a well-formed bank"
                    );
                    continue;
                };
                bound = true;
                let latency =
                    usize::try_from(bank.metadata.program_key.latency.0).expect("latency fits");
                let controls: Vec<Option<EffectControlLane>> = bypassed
                    .iter()
                    .map(|bypassed| bypassed.then(|| EffectControlLane::without_channel(true)))
                    .collect();
                let stage = LiveControlEffectBankStage::new(
                    bank,
                    width,
                    QUANTUM,
                    controls,
                    (0..lanes).map(|_| None).collect(),
                    latency,
                )
                .expect("stage");
                let mut chain = chain(stage);
                // Per lane: today's instance (the lane's bypass as the prepared flag) and, for the
                // audibility count, the same lane prepared enabled.
                let mut oracles: Vec<[Box<dyn PreparedNativeEffect>; 2]> = values
                    .iter()
                    .zip(&bypassed)
                    .map(|(values, bypassed)| {
                        [*bypassed, false].map(|prepared| {
                            factory
                                .prepare(request(descriptor, case, prepared, values))
                                .expect("scalar oracle")
                                .processor
                        })
                    })
                    .collect();
                let mut signal = Rng::new(seed ^ 0x5eed);
                for block in 0..BLOCKS {
                    let mut members = Planes {
                        left: Vec::new(),
                        right: Vec::new(),
                    };
                    for lane in 0..lanes {
                        members
                            .left
                            .push(input_block(lane, 0, block, Level::Loud, &mut signal));
                        members
                            .right
                            .push(input_block(lane, 1, block, Level::Loud, &mut signal));
                    }
                    let mut expected = [members.left.clone(), members.right.clone()];
                    let mut enabled = expected.clone();
                    chain
                        .run(&mut members, QUANTUM, block * u64::from(QUANTUM))
                        .expect("bank render");
                    for lane in 0..lanes {
                        let [today, wet] = &mut oracles[lane];
                        let [left, right] = &mut expected;
                        process_scalar(today.as_mut(), &mut left[lane], &mut right[lane], block);
                        let [wet_left, wet_right] = &mut enabled;
                        process_scalar(
                            wet.as_mut(),
                            &mut wet_left[lane],
                            &mut wet_right[lane],
                            block,
                        );
                        if bypassed[lane] {
                            audible += left[lane]
                                .iter()
                                .chain(&right[lane])
                                .zip(wet_left[lane].iter().chain(&wet_right[lane]))
                                .filter(|(dry, wet)| folded(**dry) != folded(**wet))
                                .count();
                        }
                        let what = format!(
                            "{effect_id} {case:?} pattern {pattern_index} lane {lane} (bypassed: \
                             {}) block {block}",
                            bypassed[lane]
                        );
                        assert_bits(&format!("{what} left"), &members.left[lane], &left[lane]);
                        assert_bits(&format!("{what} right"), &members.right[lane], &right[lane]);
                    }
                }
            }
        }
        if bound {
            banked_effects += 1;
            assert!(
                audible > 0,
                "{effect_id}: no bypassed lane rendered differently from its enabled self, so the \
                 comparison proves nothing"
            );
        }
    }
    assert_eq!(
        banked_effects,
        EFFECTS.len() - NEVER_BANKED_EFFECTS.len(),
        "every bankable launch effect bound a bank"
    );
}

/// Gate 2, per-node leg: every launch effect that can bank, rendered per node.
///
/// A session-bypassed instance of an effect whose bypass is lowered (every effect that can bank but
/// the multiband, issue #1100) is prepared `bypass = false` and carries
/// a channel-less lane, so where it ends up per node -- a partial cohort, a chain with a
/// sidechained slot, a scalar build -- the graph renders it as a `runtime::LiveControlEffect`:
/// capture the dry block into the shunt when the lane is bypassed or the shunt feeds a latency
/// line, run the effect, then copy the latency-matched dry block over the output. This re-enacts
/// that order with the contract's `BypassShunt` and requires today's prepared-bypass bits, at
/// [`Level::Extreme`]: there the wet path's own D7 check fires, and the shunt's copy must still be
/// today's dry block. (The session-level twin, through the real graph, is
/// `bypass_cohorts::a_session_bypass_renders_todays_bits`.)
///
/// Red mutation: `BypassShunt::capture` stops feeding its latency line -> the limiter's and the
/// soft clip's dry output is undelayed.
#[test]
fn a_shunt_bypassed_per_node_instance_is_bit_identical_to_prepared_bypass() {
    let registry = launch_native_effect_registry().expect("launch registry");
    for (effect_index, effect_id) in EFFECTS.iter().enumerate() {
        if NEVER_BANKED_EFFECTS.contains(effect_id) {
            continue;
        }
        let factory = registry.get_shared_ascii(effect_id).expect("launch effect");
        let descriptor = factory.descriptor();
        let mut audible = 0_usize;
        for (case_index, case) in cases(descriptor).into_iter().enumerate() {
            for trial in 0..4_u64 {
                let seed = (effect_index * 1_000 + case_index * 10) as u64 + 500 + trial * 31;
                let mut rng = Rng::new(seed);
                let values = initial_values(descriptor, &mut rng);
                let effect_contract::PreparedEffect {
                    processor: mut wet,
                    metadata: wet_metadata,
                } = factory
                    .prepare(request(descriptor, case, false, &values))
                    .expect("lowered instance");
                let mut oracle = factory
                    .prepare(request(descriptor, case, true, &values))
                    .expect("prepared-bypass oracle")
                    .processor;
                let mut enabled = factory
                    .prepare(request(descriptor, case, false, &values))
                    .expect("enabled instance")
                    .processor;
                let latency = usize::try_from(wet_metadata.latency.0).expect("latency fits");
                let mut shunt = effect_contract::BypassShunt::new(QUANTUM as usize, latency);
                let mut signal = Rng::new(seed ^ 0x5eed);
                for block in 0..BLOCKS {
                    let mut left = input_block(0, 0, block, Level::Extreme, &mut signal);
                    let mut right = input_block(0, 1, block, Level::Extreme, &mut signal);
                    let mut expected = [left.clone(), right.clone()];
                    let mut wet_only = expected.clone();
                    shunt.capture(&left, &right);
                    process_scalar(wet.as_mut(), &mut left, &mut right, block);
                    shunt.apply(&mut left, &mut right);
                    let [expected_left, expected_right] = &mut expected;
                    process_scalar(oracle.as_mut(), expected_left, expected_right, block);
                    let [wet_left, wet_right] = &mut wet_only;
                    process_scalar(enabled.as_mut(), wet_left, wet_right, block);
                    audible += expected_left
                        .iter()
                        .chain(expected_right.iter())
                        .zip(wet_left.iter().chain(wet_right.iter()))
                        .filter(|(dry, wet)| folded(**dry) != folded(**wet))
                        .count();
                    let what = format!("{effect_id} {case:?} trial {trial} block {block}");
                    assert_bits(&format!("{what} left"), &left, expected_left);
                    assert_bits(&format!("{what} right"), &right, expected_right);
                }
            }
        }
        assert!(
            audible > 0,
            "{effect_id}: the bypass never changed a sample, so the comparison proves nothing"
        );
    }
}

/// Gate 3 with real effects: a live toggle on a session-bypassed banked lane.
///
/// With live controls attached, the session bypass seeds the lane's live channel
/// (`attach_effect_live_controls` reads `initial_bypass`), so the lane starts bypassed and a live
/// `Bypass(false)` returns the *current* wet signal: its wet path ran all along. The reference is
/// the same lane rendered per node from a prepared-enabled instance and a shunt switched on the
/// same blocks, which is the per-node live path a session-enabled instance has always taken. The
/// limiter carries the longest latency line, so the dry signal after the re-bypass is the delayed
/// one on its first block.
///
/// Red mutation: the drain leaves `bypass` alone on an `EffectControlRecord::Bypass` record ->
/// lane 1 stays bypassed after block 5 and renders dry where the reference is wet. (That
/// `attach_effect_live_controls` seeds the live lane from the session bypass is pinned in
/// `effect-compiler`'s `native_session` tests.)
#[test]
fn a_live_toggle_on_a_session_bypassed_bank_lane_matches_the_per_node_live_path() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let width = width();
    let lanes = width.lanes() as usize;
    for effect_id in [
        "miso.true-peak-limiter",
        "miso.parametric-eq",
        "miso.soft-clip",
    ] {
        let factory = registry.get_shared_ascii(effect_id).expect("launch effect");
        let descriptor = factory.descriptor();
        let case = cases(descriptor)[0];
        let mut rng = Rng::new(0x1087);
        let values: Vec<Vec<InitialParameterValue>> = (0..lanes)
            .map(|_| initial_values(descriptor, &mut rng))
            .collect();
        let requests: Vec<PrepareEffectRequest<'_>> = values
            .iter()
            .map(|values| request(descriptor, case, false, values))
            .collect();
        let bank = factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: Backend::current(),
                width,
                requests: &requests,
                active_mask: width.full_mask(),
            })
            .expect("a well-formed bank request")
            .expect("a bankable effect binds at the host width");
        let latency = usize::try_from(bank.metadata.program_key.latency.0).expect("latency fits");
        // Lane 1 is session-bypassed with a live channel; lane 2 is session-bypassed without one.
        let (mut producer, consumer) = bounded_spsc::<EffectControlRecord>(
            core::num::NonZeroUsize::new(4).expect("depth"),
            QueueGeneration(0),
        )
        .expect("queue");
        let mut consumer = Some(consumer);
        let controls: Vec<Option<EffectControlLane>> = (0..lanes)
            .map(|lane| match lane {
                1 => Some(EffectControlLane::new(
                    consumer.take().expect("one live lane"),
                    true,
                )),
                2 => Some(EffectControlLane::without_channel(true)),
                _ => None,
            })
            .collect();
        let stage = LiveControlEffectBankStage::new(
            bank,
            width,
            QUANTUM,
            controls,
            (0..lanes).map(|_| None).collect(),
            latency,
        )
        .expect("stage");
        let mut chain = chain(stage);
        // Lane 1's live reference: a prepared-enabled instance and a shunt switched on the same
        // blocks. Every other lane's reference is its scalar instance with its session bypass as
        // the prepared flag.
        let mut live_wet = factory
            .prepare(request(descriptor, case, false, &values[1]))
            .expect("live reference")
            .processor;
        let mut live_shunt = effect_contract::BypassShunt::new(QUANTUM as usize, latency);
        let mut oracles: Vec<Box<dyn PreparedNativeEffect>> = values
            .iter()
            .enumerate()
            .map(|(lane, values)| {
                factory
                    .prepare(request(descriptor, case, lane == 2, values))
                    .expect("scalar oracle")
                    .processor
            })
            .collect();
        let mut signal = Rng::new(0x1087 ^ 0x5eed);
        let mut live_bypassed = true;
        for block in 0..BLOCKS {
            // Un-bypass at block 5 and re-bypass at block 10, each applied at the first sample of
            // the block that drains it.
            let toggle = match block {
                5 => Some(false),
                10 => Some(true),
                _ => None,
            };
            if let Some(value) = toggle {
                producer
                    .try_push(EffectControlRecord::Bypass(value))
                    .expect("room");
                live_bypassed = value;
            }
            let mut members = Planes {
                left: Vec::new(),
                right: Vec::new(),
            };
            for lane in 0..lanes {
                members
                    .left
                    .push(input_block(lane, 0, block, Level::Loud, &mut signal));
                members
                    .right
                    .push(input_block(lane, 1, block, Level::Loud, &mut signal));
            }
            let mut expected_left = members.left.clone();
            let mut expected_right = members.right.clone();
            chain
                .run(&mut members, QUANTUM, block * u64::from(QUANTUM))
                .expect("bank render");
            for lane in 0..lanes {
                if lane == 1 {
                    if live_bypassed || live_shunt.feeds_line() {
                        live_shunt.capture(&expected_left[lane], &expected_right[lane]);
                    }
                    process_scalar(
                        live_wet.as_mut(),
                        &mut expected_left[lane],
                        &mut expected_right[lane],
                        block,
                    );
                    if live_bypassed {
                        live_shunt.apply(&mut expected_left[lane], &mut expected_right[lane]);
                    }
                } else {
                    process_scalar(
                        oracles[lane].as_mut(),
                        &mut expected_left[lane],
                        &mut expected_right[lane],
                        block,
                    );
                }
                let what = format!("{effect_id} lane {lane} block {block}");
                assert_bits(
                    &format!("{what} left"),
                    &members.left[lane],
                    &expected_left[lane],
                );
                assert_bits(
                    &format!("{what} right"),
                    &members.right[lane],
                    &expected_right[lane],
                );
            }
        }
    }
}

/// `NEVER_BANKED_EFFECTS` is exactly the launch effects whose factory declines a well-formed bank
/// of default members at the host's width, and every launch effect is in the registry this file
/// walks.
///
/// Red mutations: add a banking effect to the list (its session bypass would stay prepared and
/// split its cohort again), or remove the delay (its bypass would take a shunt it gains no bank
/// from, and move the bits `the_delay_keeps_its_prepared_bypass` pins).
#[test]
fn the_never_banked_list_is_exactly_the_launch_effects_that_decline_a_bank() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let width = width();
    let lanes = width.lanes() as usize;
    let mut declined = Vec::new();
    for effect_id in EFFECTS {
        let factory = registry.get_shared_ascii(effect_id).expect("launch effect");
        let descriptor = factory.descriptor();
        let case = cases(descriptor)[0];
        let values = default_values(descriptor);
        let requests: Vec<PrepareEffectRequest<'_>> = (0..lanes)
            .map(|_| request(descriptor, case, false, &values))
            .collect();
        let bank = factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: Backend::current(),
                width,
                requests: &requests,
                active_mask: width.full_mask(),
            })
            .expect("a well-formed bank request");
        if bank.is_none() {
            declined.push(effect_id);
        }
    }
    assert_eq!(declined, NEVER_BANKED_EFFECTS.to_vec());
    let mut registered: Vec<&str> = registry
        .descriptors()
        .map(|descriptor| descriptor.id.as_str())
        .collect();
    registered.sort_unstable();
    let mut walked = EFFECTS.to_vec();
    walked.sort_unstable();
    assert_eq!(registered, walked, "this file walks every launch effect");
}

/// Why the delay keeps its prepared bypass: a shunt would move a bit.
///
/// The delay's D7 check reads its feedback ring as well as its output. A 1 ms delay at
/// `0.95` feedback fed `9e29` -- inside the range an effect can receive in a compiled plan --
/// grows a ring cell past `1e30` within one block, and the check zeroes the block. Under a
/// prepared bypass the zeroed block is the output; a shunt would copy the dry block over it. If
/// this ever renders the same bits, the delay's check no longer reads its state and it may take the
/// shunt like every other effect.
#[test]
fn the_delay_keeps_its_prepared_bypass_because_a_shunt_would_move_a_bit() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let factory = registry.get_shared_ascii("miso.delay").expect("the delay");
    let descriptor = factory.descriptor();
    let case = cases(descriptor)[0];
    let by_name = |name: &str| {
        descriptor
            .parameters
            .iter()
            .position(|parameter| parameter.display_name == name)
            .unwrap_or_else(|| panic!("delay parameter {name}")) as u32
    };
    let mut values = default_values(descriptor);
    for (name, value) in [("delay time", 1.0), ("feedback", 0.95), ("damping", 0.0)] {
        let index = by_name(name);
        for item in values
            .iter_mut()
            .filter(|item| item.parameter_index == index)
        {
            item.value = value;
        }
    }
    let mut prepared = factory
        .prepare(request(descriptor, case, true, &values))
        .expect("prepared bypass")
        .processor;
    let mut wet = factory
        .prepare(request(descriptor, case, false, &values))
        .expect("prepared enabled")
        .processor;
    let mut shunt = effect_contract::BypassShunt::new(QUANTUM as usize, 0);
    let mut moved = false;
    for block in 0..4 {
        let mut today = [
            vec![9.0e29_f32; QUANTUM as usize],
            vec![9.0e29_f32; QUANTUM as usize],
        ];
        let mut shunted = today.clone();
        let [left, right] = &mut today;
        process_scalar(prepared.as_mut(), left, right, block);
        let [left, right] = &mut shunted;
        shunt.capture(left, right);
        process_scalar(wet.as_mut(), left, right, block);
        shunt.apply(left, right);
        moved |= today
            .iter()
            .flatten()
            .zip(shunted.iter().flatten())
            .any(|(a, b)| folded(*a) != folded(*b));
    }
    assert!(
        moved,
        "the delay's prepared bypass and a shunt now agree; it may leave NEVER_BANKED_EFFECTS"
    );
}
