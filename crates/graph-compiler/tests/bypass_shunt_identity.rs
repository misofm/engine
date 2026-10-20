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
    BankWidth, EffectControlLane, EffectControlRecord, EffectDescriptor, EffectLaneShape,
    EffectLaneSpec, EffectProcessBlock, EffectQuality, InitialParameterValue, LinkMode,
    ParameterChannel, ParameterChannelPolicy, ParameterDomain, ParameterUnit, PortRole,
    PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest, PreparedNativeEffect,
    PreparedPorts, PreparedSidechainPort, effect_control_lane,
};
use engine::realtime::{LiveSnapshot, LiveValuesSuperseded, RevisionGate};
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
        tail_bound: launch_native_effect_registry()
            .expect("launch registry")
            .tail_bound(descriptor.id, RATE, case.quality)
            .expect("a declared row"),
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
                        .run(
                            &mut members,
                            QUANTUM,
                            block * u64::from(QUANTUM),
                            LiveSnapshot::ALL,
                        )
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

// ---- #1341: the bypass crossfade over the session ramp, per node ----------------------------------

/// What a per-node live instance does with its bypass: nothing (W, never bypassed), a session
/// bypass from preparation (D), or a live bypass by records.
enum Bypassing {
    Never,
    FromPreparation,
    /// `(block, bypassed, ramp_samples)`, in block order.
    Records(Vec<(u64, bool, u32)>),
}

/// One per-node live instance rendered in the order `runtime::LiveControlEffect` runs it: stage
/// the lane, capture the dry block when the lane needs it or the shunt feeds a line, run the
/// effect, then mix the dry block back in. Returns every block's output.
///
/// `L` is the width of the mix kernel, so the same run at two widths pins that the crossfade is a
/// function of the frame index and not of the vector width.
fn render_per_node<L: lane::Lane>(
    effect_id: &str,
    bypassing: &Bypassing,
    blocks: u64,
) -> (usize, Vec<[Vec<f32>; 2]>) {
    let registry = launch_native_effect_registry().expect("launch registry");
    let factory = registry.get_shared_ascii(effect_id).expect("launch effect");
    let descriptor = factory.descriptor();
    let case = cases(descriptor)[0];
    let values = default_values(descriptor);
    let effect_contract::PreparedEffect {
        processor: mut effect,
        metadata,
    } = factory
        .prepare(request(descriptor, case, false, &values))
        .expect("instance");
    let latency = usize::try_from(metadata.latency.0).expect("latency fits");
    let mut shunt = effect_contract::BypassShunt::new(QUANTUM as usize, latency);
    let superseded = LiveValuesSuperseded::new();
    let (gate, mut pin) = RevisionGate::pair(0);
    let (mut writer, mut lane) = match bypassing {
        Bypassing::Records(_) => {
            let (writer, lane) = effect_control_lane(
                EffectLaneSpec {
                    shape: EffectLaneShape::from_live_parameters([], 0),
                    bypass: false,
                    observation_depth: 0,
                },
                &superseded,
            )
            .expect("the live lane");
            (Some(writer), lane)
        }
        Bypassing::FromPreparation => (None, EffectControlLane::without_channel(true)),
        Bypassing::Never => (None, EffectControlLane::without_channel(false)),
    };
    let mut signal = Rng::new(0x1341);
    let mut output = Vec::new();
    for block in 0..blocks {
        if let (Bypassing::Records(records), Some(writer)) = (bypassing, writer.as_mut()) {
            for &(at, bypassed, ramp_samples) in records {
                if at == block {
                    let revision = pin.last_published() + 1;
                    let stamp = pin.stamp(revision).expect("a legal revision");
                    writer
                        .write(
                            stamp,
                            EffectControlRecord::Bypass(effect_contract::BypassSwitch {
                                bypassed,
                                ramp_samples,
                            }),
                        )
                        .expect("a bypass cell");
                    pin.publish(&gate, revision);
                }
            }
        }
        let first_sample = block * u64::from(QUANTUM);
        lane.stage(&mut [], first_sample, gate.snapshot());
        let mut left = input_block(0, 0, block, Level::Loud, &mut signal);
        let mut right = input_block(0, 1, block, Level::Loud, &mut signal);
        let needs_dry = lane.needs_dry();
        if needs_dry || shunt.feeds_line() {
            shunt.capture(&left, &right);
        }
        process_scalar(effect.as_mut(), &mut left, &mut right, block);
        if needs_dry {
            lane.apply_bypass::<L>(&shunt, &mut left, &mut right);
        }
        output.push([left, right]);
    }
    (latency, output)
}

/// The wet mix of a crossfade of `length` frames from `start` to `target`, at ramp index `k`, from
/// the indexed ramp law alone.
fn mix_at(start: f32, target: f32, length: u32, k: u32) -> f32 {
    lane::kernels::IndexedRamp::new([start, 0.0, 0.0, 0.0], [target, 0.0, 0.0, 0.0], length)
        .coefficients_at(k)[0]
}

/// `dry + (wet - dry) * m`, as a separate subtract, multiply and add.
fn mixed(dry: f32, wet: f32, m: f32) -> f32 {
    dry + (wet - dry) * m
}

/// A zero-latency effect that changes the loud test signal, and one with latency.
const ZERO_AND_LATENT: [&str; 2] = ["miso.compressor", "miso.true-peak-limiter"];

/// How many of `a`'s samples differ from `b`'s, in the blocks `from..`: the evidence that a
/// comparison of the two is not vacuous.
fn differing(a: &[[Vec<f32>; 2]], b: &[[Vec<f32>; 2]], from: usize) -> usize {
    a[from..]
        .iter()
        .zip(&b[from..])
        .flat_map(|(a, b)| a.iter().flatten().zip(b.iter().flatten()))
        .filter(|(a, b)| folded(**a) != folded(**b))
        .count()
}

const RAMP: u32 = 480;
const RECORD_BLOCK: u64 = 3;

/// Gate 1 of #1341: a live bypass crossfades over the session ramp, per node, and settles on
/// today's whole-block select. X is bypassed by a record at block 3 with a 480-sample ramp at
/// quantum 128, so the ramp ends inside block 6. Before the record X is W; inside the ramp it is
/// `D + (W - D) * m(k)` with `m` from the indexed ramp law alone and `k` counted from the record's
/// first frame; from the ramp's end it is D bit for bit. A zero-latency effect and one with
/// latency (the true-peak limiter), at the host's width and at scalar width.
///
/// Test value: red if `m` comes from an accumulated sum or a fused multiply-add, if the ramp is
/// off by a frame at the block boundary, if the settled frames compute the formula instead of
/// copying the dry words, or if the mix reads another width's `m`.
#[test]
fn a_live_bypass_crossfades_per_node_over_the_ramp_and_settles_on_the_select() {
    for effect_id in ZERO_AND_LATENT {
        let blocks = 12;
        let (latency, wet) = render_per_node::<lane::Native>(effect_id, &Bypassing::Never, blocks);
        let (_, dry) =
            render_per_node::<lane::Native>(effect_id, &Bypassing::FromPreparation, blocks);
        let records = Bypassing::Records(vec![(RECORD_BLOCK, true, RAMP)]);
        let (_, crossfaded) = render_per_node::<lane::Native>(effect_id, &records, blocks);
        let (_, scalar) = render_per_node::<f32>(effect_id, &records, blocks);
        assert_eq!(
            latency == 0,
            effect_id == ZERO_AND_LATENT[0],
            "{effect_id}: the pair covers one zero-latency effect and one with latency"
        );
        assert!(
            differing(&wet, &dry, RECORD_BLOCK as usize) > 0,
            "{effect_id}: the wet and dry outputs never differ, so the crossfade proves nothing"
        );
        assert!(
            differing(&crossfaded, &wet, RECORD_BLOCK as usize) > 0
                && differing(&crossfaded, &dry, RECORD_BLOCK as usize) > 0,
            "{effect_id}: the crossfade is neither the wet nor the dry output"
        );
        let mut inside = 0_usize;
        for block in 0..blocks {
            for channel in 0..2 {
                let what = format!("{effect_id} block {block} channel {channel}");
                assert_bits(
                    &format!("{what}: the host width equals scalar width"),
                    &crossfaded[block as usize][channel],
                    &scalar[block as usize][channel],
                );
                let expected: Vec<f32> = (0..QUANTUM)
                    .map(|frame| {
                        let (w, d) = (
                            wet[block as usize][channel][frame as usize],
                            dry[block as usize][channel][frame as usize],
                        );
                        if block < RECORD_BLOCK {
                            return w;
                        }
                        let k = (block - RECORD_BLOCK) * u64::from(QUANTUM) + u64::from(frame) + 1;
                        if k < u64::from(RAMP) {
                            inside += 1;
                            mixed(d, w, mix_at(1.0, 0.0, RAMP, k as u32))
                        } else {
                            d
                        }
                    })
                    .collect();
                assert_bits(&what, &crossfaded[block as usize][channel], &expected);
            }
        }
        assert_eq!(
            inside,
            2 * (RAMP as usize - 1),
            "{effect_id}: the ramp frames"
        );
    }
}

/// Gate 4 of #1341: a record with a zero-sample ramp is a step. X equals D from the first sample of
/// the record's block, which is what an un-ramped bypass always rendered.
///
/// Test value: red if a zero length is not a step, which would move today's bits for an explicit
/// `mute_ms = 0`.
#[test]
fn a_bypass_record_with_a_zero_ramp_steps_at_the_first_sample_of_its_block() {
    for effect_id in ZERO_AND_LATENT {
        let blocks = 8;
        let (_, wet) = render_per_node::<lane::Native>(effect_id, &Bypassing::Never, blocks);
        let (_, dry) =
            render_per_node::<lane::Native>(effect_id, &Bypassing::FromPreparation, blocks);
        let records = Bypassing::Records(vec![(RECORD_BLOCK, true, 0)]);
        let (_, stepped) = render_per_node::<lane::Native>(effect_id, &records, blocks);
        assert!(
            differing(&wet, &dry, RECORD_BLOCK as usize) > 0,
            "{effect_id}: the wet and dry outputs never differ, so the step proves nothing"
        );
        for block in 0..blocks as usize {
            let expected = if block < RECORD_BLOCK as usize {
                &wet
            } else {
                &dry
            };
            for channel in 0..2 {
                assert_bits(
                    &format!("{effect_id} block {block} channel {channel}"),
                    &stepped[block][channel],
                    &expected[block][channel],
                );
            }
        }
    }
}

/// Gate 3 of #1341, per node: the un-bypass record lands at block 5, at `k = 256` of the 480-frame
/// ramp. From block 5 the output follows `IndexedRamp::new([m(256), 0, 0, 0], [1, 0, 0, 0], 480)`
/// with no jump, and equals W exactly from 480 frames later. A second bypass record for the
/// target the lane is already heading for, in block 7, changes nothing.
///
/// Test value: red if a reversal restarts from the settled endpoint (a jump) instead of from the
/// current mix, or if a repeated target restarts the ramp.
#[test]
fn a_reversal_continues_from_the_current_mix_and_a_repeat_changes_nothing() {
    for effect_id in ZERO_AND_LATENT {
        let blocks = 14;
        let (_, wet) = render_per_node::<lane::Native>(effect_id, &Bypassing::Never, blocks);
        let (_, dry) =
            render_per_node::<lane::Native>(effect_id, &Bypassing::FromPreparation, blocks);
        let records = Bypassing::Records(vec![
            (RECORD_BLOCK, true, RAMP),
            (5, false, RAMP),
            (7, false, RAMP),
        ]);
        let (_, reversed) = render_per_node::<lane::Native>(effect_id, &records, blocks);
        assert!(
            differing(&wet, &dry, RECORD_BLOCK as usize) > 0,
            "{effect_id}: the wet and dry outputs never differ, so the reversal proves nothing"
        );
        // Blocks 3 and 4 rendered 256 ramp frames; the last of them used m(256).
        let resume = mix_at(1.0, 0.0, RAMP, 256);
        let mut continuing = 0_usize;
        for block in 0..blocks as usize {
            for channel in 0..2 {
                let expected: Vec<f32> = (0..QUANTUM as usize)
                    .map(|frame| {
                        let (w, d) = (wet[block][channel][frame], dry[block][channel][frame]);
                        let block = block as u64;
                        if block < RECORD_BLOCK {
                            w
                        } else if block < 5 {
                            let k = (block - RECORD_BLOCK) * u64::from(QUANTUM) + frame as u64 + 1;
                            mixed(d, w, mix_at(1.0, 0.0, RAMP, k as u32))
                        } else {
                            let k = (block - 5) * u64::from(QUANTUM) + frame as u64 + 1;
                            if k < u64::from(RAMP) {
                                continuing += 1;
                                mixed(d, w, mix_at(resume, 1.0, RAMP, k as u32))
                            } else {
                                w
                            }
                        }
                    })
                    .collect();
                assert_bits(
                    &format!("{effect_id} block {block} channel {channel}"),
                    &reversed[block][channel],
                    &expected,
                );
            }
        }
        assert_eq!(
            continuing,
            2 * (RAMP as usize - 1),
            "{effect_id}: the second ramp"
        );
    }
}

// ---- #1341a2: the bank crossfades exactly as the node does -----------------------------------------

/// How a bank lane starts.
#[derive(Clone, Copy, Debug)]
enum Start {
    /// A live channel, not bypassed.
    Wet,
    /// A live channel, session-bypassed.
    Bypassed,
    /// No channel, session-bypassed (it can never be lifted).
    FixedBypass,
    /// No control lane at all.
    Plain,
}

/// `(lane, block, bypassed, ramp_samples)`: the bypass records of the bank test, one revision each.
type Schedule = &'static [(usize, u64, bool, u32)];

/// The bank test's lanes (the first four on a four-lane bank), and their records:
///
/// * 0 is never switched (W);
/// * 1 bypasses at block 3 over 480 frames, which end inside block 6 (95 ramp frames there, so the
///   per-node kernel runs its scalar tail on them);
/// * 2 bypasses at block 3 and lifts at block 5, at `k = 256`: a reversal mid-ramp;
/// * 3 starts bypassed and lifts at block 6 over 300 frames;
/// * 4 bypasses at block 4 with a ramp of 0 (a step);
/// * 5 is bypassed for good without a channel, 6 has no control lane;
/// * 7 starts bypassed, lifts at block 3 over exactly one block and bypasses again at block 4 over
///   100 frames, which end inside block 4.
const BANK_LANES: [Start; 8] = [
    Start::Wet,
    Start::Wet,
    Start::Wet,
    Start::Bypassed,
    Start::Wet,
    Start::FixedBypass,
    Start::Plain,
    Start::Bypassed,
];
const BANK_SCHEDULE: Schedule = &[
    (1, 3, true, 480),
    (2, 3, true, 480),
    (2, 5, false, 480),
    (3, 6, false, 300),
    (4, 4, true, 0),
    (7, 3, false, 128),
    (7, 4, true, 100),
];

/// A per-node reference lane: the instance, its shunt and its control lane, driven in the order
/// `runtime::LiveControlEffect` runs them.
struct NodeLane {
    effect: Box<dyn PreparedNativeEffect>,
    shunt: effect_contract::BypassShunt,
    control: EffectControlLane,
    writer: Option<effect_contract::EffectCellWriter>,
}

fn live_lane(
    bypass: bool,
    superseded: &LiveValuesSuperseded,
) -> (effect_contract::EffectCellWriter, EffectControlLane) {
    effect_control_lane(
        EffectLaneSpec {
            shape: EffectLaneShape::from_live_parameters([], 0),
            bypass,
            observation_depth: 0,
        },
        superseded,
    )
    .expect("the live lane")
}

/// Gate 2 of #1341 (bank equals node) and gate 3 banked. One bank of `backend`'s width runs
/// `effect_id` on every lane of [`BANK_LANES`] through the rack's real live stage, with
/// [`BANK_SCHEDULE`]'s bypass records, and each lane's output equals the per-node render of the
/// same lane, bit for bit, in every block: a lane that bypasses over a ramp, one that reverses
/// mid-ramp, one that lifts, a step, a lane bypassed without a channel, a lane without a control
/// lane, and a lane that is never switched, next to one another in one bank.
fn bank_equals_node(effect_id: &str, backend: Backend) {
    let registry = launch_native_effect_registry().expect("launch registry");
    let factory = registry.get_shared_ascii(effect_id).expect("launch effect");
    let descriptor = factory.descriptor();
    let case = cases(descriptor)[0];
    let values = default_values(descriptor);
    let width = BankWidth::for_backend(backend).expect("a vector backend");
    let lanes = width.lanes() as usize;
    let starts = &BANK_LANES[..lanes];
    let requests: Vec<PrepareEffectRequest<'_>> = (0..lanes)
        .map(|_| request(descriptor, case, false, &values))
        .collect();
    let bank = factory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        })
        .expect("a well-formed bank request")
        .expect("a bankable effect binds at this width");
    let latency = usize::try_from(bank.metadata.program_key.latency.0).expect("latency fits");
    let superseded = LiveValuesSuperseded::new();
    let mut bank_writers: Vec<Option<effect_contract::EffectCellWriter>> = Vec::new();
    let mut controls: Vec<Option<EffectControlLane>> = Vec::new();
    let mut nodes: Vec<NodeLane> = Vec::new();
    for start in starts {
        let (writer, control) = match start {
            Start::Wet | Start::Bypassed => {
                let (writer, control) = live_lane(matches!(start, Start::Bypassed), &superseded);
                (Some(writer), Some(control))
            }
            Start::FixedBypass => (None, Some(EffectControlLane::without_channel(true))),
            Start::Plain => (None, None),
        };
        bank_writers.push(writer);
        controls.push(control);
        let (writer, control) = match start {
            Start::Wet | Start::Bypassed => {
                let (writer, control) = live_lane(matches!(start, Start::Bypassed), &superseded);
                (Some(writer), control)
            }
            Start::FixedBypass => (None, EffectControlLane::without_channel(true)),
            Start::Plain => (None, EffectControlLane::without_channel(false)),
        };
        nodes.push(NodeLane {
            effect: factory
                .prepare(request(descriptor, case, false, &values))
                .expect("per-node instance")
                .processor,
            shunt: effect_contract::BypassShunt::new(QUANTUM as usize, latency),
            control,
            writer,
        });
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
    // Lane 1 never bypassed: the evidence that its crossfade is not the wet or the dry output.
    let mut wet_only = factory
        .prepare(request(descriptor, case, false, &values))
        .expect("wet twin")
        .processor;
    let (gate, mut pin) = RevisionGate::pair(0);
    let mut signal = Rng::new(0x0013_41a2);
    let (mut between_lane_one, mut compared) = (0_usize, 0_usize);
    for block in 0..14_u64 {
        for &(lane, at, bypassed, ramp_samples) in BANK_SCHEDULE {
            if at != block || lane >= lanes {
                continue;
            }
            let revision = pin.last_published() + 1;
            let stamp = pin.stamp(revision).expect("a legal revision");
            let switch = effect_contract::BypassSwitch {
                bypassed,
                ramp_samples,
            };
            for writer in [bank_writers[lane].as_mut(), nodes[lane].writer.as_mut()] {
                writer
                    .expect("a lane with records has a channel")
                    .write(stamp, EffectControlRecord::Bypass(switch))
                    .expect("a bypass cell");
            }
            pin.publish(&gate, revision);
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
        let mut wet_left = members.left[1].clone();
        let mut wet_right = members.right[1].clone();
        let first_sample = block * u64::from(QUANTUM);
        chain
            .run(&mut members, QUANTUM, first_sample, gate.snapshot())
            .expect("bank render");
        process_scalar(wet_only.as_mut(), &mut wet_left, &mut wet_right, block);
        for (lane, node) in nodes.iter_mut().enumerate() {
            let (left, right) = (&mut expected_left[lane], &mut expected_right[lane]);
            node.control.stage(&mut [], first_sample, gate.snapshot());
            let needs_dry = node.control.needs_dry();
            if needs_dry || node.shunt.feeds_line() {
                node.shunt.capture(left, right);
            }
            process_scalar(node.effect.as_mut(), left, right, block);
            if needs_dry {
                node.control
                    .apply_bypass::<lane::Native>(&node.shunt, left, right);
            }
            let what = format!("{effect_id} at {backend:?}: lane {lane} block {block}");
            assert_bits(&format!("{what} left"), &members.left[lane], left);
            assert_bits(&format!("{what} right"), &members.right[lane], right);
        }
        if (3..7).contains(&block) {
            compared += QUANTUM as usize;
            between_lane_one += members.left[1]
                .iter()
                .zip(&wet_left)
                .filter(|(a, w)| folded(**a) != folded(**w))
                .count();
        }
    }
    assert!(
        between_lane_one * 2 > compared,
        "{effect_id}: lane 1's ramp never leaves the wet output, so the comparison proves nothing"
    );
}

/// Gate 2 and gate 3 (banked) of #1341: a bank renders every lane's bypass crossfade exactly as the
/// per-node path renders it, at the build's bank width (eight lanes on AVX2, four on NEON and
/// simd128: an effect binds only the build's width, so the other width cannot be bound here; the
/// kernel at each width is pinned lane by lane in `effect-contract`'s `live_control` tests), for a
/// zero-latency effect and one with latency. The per-node leg runs at the host's lane width and its
/// scalar tail (the ramp's last 95 frames in block 6 are not a whole vector).
///
/// Test value: red if a banked lane steps at its record instead of ramping (the interim this test
/// replaces), reads a neighbour's `m` or position, keeps a settled lane on the formula path, or if
/// a reversal restarts from the settled endpoint.
#[test]
fn a_bank_crossfades_every_lane_exactly_as_the_node_does() {
    for effect_id in ZERO_AND_LATENT {
        bank_equals_node(effect_id, Backend::current());
    }
}
