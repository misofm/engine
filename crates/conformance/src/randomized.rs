//! The randomized bank differential every launch effect runs (issue #1051).
//!
//! One seeded scenario per seed, through the shipped contract calls only -- `prepare`,
//! `bind_homogeneous_bank`, `process`, `process_bank`, `process_bank_mono`,
//! `desymmetrize_channels`, the payload calls and the channel-symmetry witness -- so a test that
//! passes is a statement about the path a host runs. The oracles:
//!
//! * **bank against scalar.** A homogeneous bank at every width this build binds renders, lane for
//!   lane, the words its scalar instances render, and reports and snapshots what they do. Banking
//!   regroups lanes and never changes per-lane arithmetic (AGENTS.md).
//! * **chunked against whole.** The scalar oracle renders some blocks in two pieces, which master
//!   plan P1 says no rendered bit may notice.
//! * **collapsed against dual.** A second bank renders `process_bank_mono` whenever the
//!   channel-symmetry witness and the scenario allow (the terms of
//!   `effect_contract::ChannelSymmetryWitness`, modelled here), and must render the dual bank's left
//!   plane, report what it reports, and hold its state after `desymmetrize_channels`. While it is
//!   collapsed the dual bank's right plane must equal its left one: that is the witness's claim.
//! * **restored against restored.** Snapshots of the lane itself or of another lane, words rewritten
//!   to subnormals, signed zeros, non-finite and extreme values, and payloads the effect's own
//!   crafting hook writes, restored into a scalar instance and into its bank lanes at once: they
//!   must accept or refuse together, with the same code, and render the same words afterwards.
//! * **bind eligibility.** A cohort that differs in one program-key field declines, a malformed
//!   shape refuses with `effect.bank.requests`, and an invalid member refuses with the code
//!   `prepare` gives it (the three-outcome rule on `bind_homogeneous_bank`).
//!
//! And the invariants a differential cannot see when both sides share a defect: an unbypassed
//! effect keeps every output word inside the D7 block bound (`|x| < 1e30`, never NaN) whatever the
//! input, and no `process` call allocates, locks, logs or makes a syscall.
//!
//! Comparisons treat every NaN as one value (`dsp_reference::randomized::same_word`, issue #1065):
//! the payload of a NaN an operation generates is the CPU's choice. Everything else, the sign of
//! zero included, is compared by bits.

use dsp_reference::randomized::{Draw, Profile, first_difference, run_seeds, same_word};
use effect_contract::{
    AutomationRate, AutomationSpanKind, BankProcessReport, BankWidth, EffectBankProcessBlock,
    EffectDescriptor, EffectProcessBlock, InitialParameterValue, LinkMode, NativeEffectFactory,
    ParameterChannel, ParameterChannelPolicy, ParameterDescriptor, ParameterDomain, PortRole,
    PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan,
    PreparedEffectMetadata, PreparedNativeEffect, PreparedNativeEffectBank, PreparedPorts,
    PreparedSidechainPort, ProcessReport, QualityDescriptor, ResetKind, StatePayloadInput,
    StatePayloadOutput, StatePayloadSizes, canonical_bits, default_initial_values,
    is_negative_zero, normalize_zero, parameter_value_valid,
};
use engine::realtime::audit;
use lane::Backend;

/// One state payload's three sections, as `snapshot_state_payload` writes them.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Payload {
    /// The common section.
    pub common: Vec<u8>,
    /// The left channel's section.
    pub left: Vec<u8>,
    /// The right channel's section.
    pub right: Vec<u8>,
}

/// An effect's own payload crafting: rewrite a lane's snapshot in place, and say what the
/// channel-symmetry witness must answer once it is restored (`Some`), or make no claim (`None`).
///
/// It is how an effect whose payload layout the harness cannot know draws the state words that
/// matter to it -- for the compressor, in-flight ramps whose channels differ in exactly one field.
pub type Craft = fn(&mut Draw, &PreparedEffectMetadata, &mut Payload) -> Option<bool>;

/// A defect this differential found, carried by name until the issue that owns it lands.
///
/// Each one narrows the generator exactly where the defect lives and nowhere else, so the rest of
/// the gate keeps running per pull request; the owning crate keeps an `#[ignore]`d twin of its
/// test without the narrowing as the defect's reproducer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Known {
    /// `bind_homogeneous_bank` declines (`Ok(None)`) at a width this build does not execute, or on
    /// a heterogeneous member, before it has validated every member -- where the three-outcome
    /// rule says an invalid member refuses first. Narrowing: the illegal-member probe accepts the
    /// decline.
    BindDeclinesBeforeValidating,
    /// The effect holds subnormal words it was legally given -- a subnormal parameter value
    /// inside the declared domain, a subnormal input sample in its history -- and its own restore
    /// then refuses the snapshot that carries them, so a snapshot does not survive its own
    /// restore. Narrowing: a refused own snapshot is accepted, so long as the scalar instance and
    /// every bank lane refuse it alike.
    SubnormalStateRefusedOnRestore,
    /// A lane's rendered bits depend on where its in-flight ramps are cut: by another lane's
    /// retarget in the same bank (so the bank is not its scalar instances), and by a block
    /// boundary (so the render is not partition-invariant, master plan P1). Narrowing: a scenario
    /// either carries automation, on one lane only, or renders chunked blocks, never both; and a
    /// restore carries the lane's own untouched snapshot, so it starts no ramp elsewhere.
    RampCutsMoveBits,
}

/// One effect's randomized differential.
pub struct EffectDifferential<'a> {
    /// The factory under test.
    pub factory: &'a dyn NativeEffectFactory,
    /// The test's name, printed beside a failing seed.
    pub test: &'a str,
    /// The command that reruns the test, printed beside a failing seed.
    pub replay: &'a str,
    /// Seeds per pull request; `dsp_reference::randomized::seeds` scales or replaces them.
    pub seeds: u64,
    /// Blocks rendered per seed and bound width.
    pub blocks: usize,
    /// The effect's payload crafting, if it has one.
    pub craft: Option<Craft>,
    /// The known defects this run narrows around; empty for the full-strength gate.
    pub known: &'a [Known],
    /// Whether the effect banks at the width this build executes (every launch effect but the
    /// delay, which renders per node everywhere). A bank that stops binding natively is then red
    /// rather than a quiet fall back to scalar instances.
    pub banks_natively: bool,
}

/// What the differential reached. A test asserts the parts it relies on, so a generator that
/// stopped reaching something is a red rather than a silently weaker gate.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DifferentialCoverage {
    /// Seeds run.
    pub seeds: u64,
    /// Whether the allocation audit could observe a violation in this binary.
    pub audited: bool,
    /// Banks bound, at four and at eight lanes.
    pub banks: [u64; 2],
    /// Banks bound at the width this build executes (`lane::Backend::current`).
    pub native_banks: u64,
    /// Whether the effect under test is expected to bank natively.
    pub banks_natively: bool,
    /// Homogeneous cohorts the factory declined, at four and at eight lanes.
    pub declined: [u64; 2],
    /// Cohorts with a member `prepare` refused, and whose bind refused with the same code.
    pub refused_cohorts: u64,
    /// Cohorts that differ in one program-key field, and declined.
    pub heterogeneous_declines: u64,
    /// Malformed bank shapes, refused.
    pub malformed_refusals: u64,
    /// Bank blocks compared with their scalar instances.
    pub blocks: u64,
    /// Blocks the scalar oracle rendered in two pieces.
    pub chunked_blocks: u64,
    /// Blocks the second bank rendered collapsed.
    pub collapsed_blocks: u64,
    /// Collapsed runs that ended in `desymmetrize_channels` and a state comparison.
    pub disengages: u64,
    /// Whether any bound bank supports the mono collapse.
    pub mono_capable: bool,
    /// Automation spans delivered, valid or not.
    pub spans: u64,
    /// Restores both sides accepted.
    pub restores: u64,
    /// Restores both sides refused with one code.
    pub refused_restores: u64,
    /// Restores of a payload the effect's crafting hook wrote.
    pub crafted_restores: u64,
    /// Witness answers checked against a crafting hook's claim.
    pub witness_checks: u64,
    /// State snapshots compared between the bank and its scalar instances.
    pub state_comparisons: u64,
    /// Unbypassed output blocks checked against the D7 bound.
    pub bounded_blocks: u64,
    /// Blocks that failed the D7 bound, and whose whole-bank recovery was checked.
    pub recoveries: u64,
    /// Scenarios run twin against twin, because no width bound a bank.
    pub scalar_scenarios: u64,
}

impl DifferentialCoverage {
    fn add(&mut self, other: &Self) {
        self.seeds += other.seeds;
        self.audited |= other.audited;
        for index in 0..2 {
            self.banks[index] += other.banks[index];
            self.declined[index] += other.declined[index];
        }
        self.refused_cohorts += other.refused_cohorts;
        self.heterogeneous_declines += other.heterogeneous_declines;
        self.malformed_refusals += other.malformed_refusals;
        self.blocks += other.blocks;
        self.chunked_blocks += other.chunked_blocks;
        self.collapsed_blocks += other.collapsed_blocks;
        self.disengages += other.disengages;
        self.mono_capable |= other.mono_capable;
        self.native_banks += other.native_banks;
        self.banks_natively |= other.banks_natively;
        self.spans += other.spans;
        self.restores += other.restores;
        self.refused_restores += other.refused_restores;
        self.crafted_restores += other.crafted_restores;
        self.witness_checks += other.witness_checks;
        self.state_comparisons += other.state_comparisons;
        self.bounded_blocks += other.bounded_blocks;
        self.recoveries += other.recoveries;
        self.scalar_scenarios += other.scalar_scenarios;
    }
}

/// Runs `spec` over its seeds and returns what it reached.
///
/// # Panics
///
/// On the first divergence, with the seed and the command that replays it.
#[must_use]
pub fn run_effect_differential(spec: &EffectDifferential<'_>) -> DifferentialCoverage {
    let audited = allocation_audit_is_real();
    let mut total = DifferentialCoverage {
        audited,
        banks_natively: spec.banks_natively,
        ..DifferentialCoverage::default()
    };
    run_seeds(spec.test, spec.replay, spec.seeds, |seed| {
        let mut coverage = DifferentialCoverage {
            seeds: 1,
            audited,
            ..DifferentialCoverage::default()
        };
        scenario(spec, seed, audited, &mut coverage);
        total.add(&coverage);
    });
    total
}

/// Whether an allocation inside a render scope is observable in this binary: the consumer enabled
/// `realtime-audit` and installed `bench_support`'s audited allocator in count mode.
fn allocation_audit_is_real() -> bool {
    if !audit::in_render_scope(audit::is_render_scope_active) {
        return false;
    }
    audit::reset();
    let observed = audit::in_render_scope(|| {
        let scratch = Vec::<u8>::with_capacity(64);
        core::hint::black_box(&scratch);
        audit::snapshot()
    });
    audit::reset();
    observed.allocations > 0
}

/// Runs one `process` call inside an armed render scope and fails on any forbidden operation.
fn audited<T>(armed: bool, what: &str, call: impl FnOnce() -> T) -> T {
    if !armed {
        return call();
    }
    audit::reset();
    let result = audit::in_render_scope(call);
    let observed = audit::snapshot();
    audit::reset();
    assert_eq!(
        observed.total(),
        0,
        "{what}: forbidden operations inside process: {observed:?}"
    );
    result
}

const WIDTHS: [(BankWidth, Backend); 2] = [
    (BankWidth::Four, Backend::Simd4),
    (BankWidth::Eight, Backend::Simd8),
];

/// Everything one seed shares across its widths: a bank's program key is one per cohort.
#[derive(Clone, Copy, Debug)]
struct Shape {
    quality: QualityDescriptor,
    quantum: u32,
    link: LinkMode,
    bypass: bool,
    ports: PreparedPorts,
    capacity: u32,
    /// Every lane's two channels carry one plane, one parameter set and one automation: the
    /// collapse can engage.
    mono: bool,
    /// Hostile input words and hostile payload words are drawn.
    hostile: bool,
}

fn scenario(
    spec: &EffectDifferential<'_>,
    seed: u64,
    audited: bool,
    coverage: &mut DifferentialCoverage,
) {
    let mut draw = Draw::new(seed);
    let descriptor = spec.factory.descriptor();
    let links: Vec<LinkMode> = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average]
        .into_iter()
        .filter(|link| descriptor.supported_link_modes.contains(*link))
        .collect();
    let mono = draw.chance(1, 3);
    let shape = Shape {
        quality: draw.pick(descriptor.qualities),
        quantum: draw.pick(&[128_u32, 128, 64, 32, 256]),
        link: draw.pick(&links),
        bypass: draw.chance(1, 8),
        ports: draw_ports(&mut draw, descriptor),
        capacity: draw.pick(&[1_u32, 2, 4, 8, 16]),
        mono,
        hostile: !mono && draw.chance(1, 2),
    };
    let bound = coverage.banks.iter().sum::<u64>();
    for (width, backend) in WIDTHS {
        run_width(spec, &mut draw, shape, width, backend, audited, coverage);
    }
    if coverage.banks.iter().sum::<u64>() == bound {
        run_scalar(spec, &mut draw, shape, audited, coverage);
    }
}

/// The differential for an effect that banks at no width this build executes (the delay renders
/// per node everywhere): each scalar instance against a twin prepared from the same request, which
/// renders some blocks in two pieces and takes the oracle's own snapshot back mid-stream. So the
/// chunked render must be the whole one (master plan P1), and a restored instance must continue
/// exactly as the one it was taken from.
#[allow(clippy::too_many_lines)]
fn run_scalar(
    spec: &EffectDifferential<'_>,
    draw: &mut Draw,
    shape: Shape,
    audited_calls: bool,
    coverage: &mut DifferentialCoverage,
) {
    let factory = spec.factory;
    let descriptor = factory.descriptor();
    let lanes = 1 + draw.below(2);
    let mut values: Vec<Vec<InitialParameterValue>> = Vec::with_capacity(lanes);
    for _ in 0..lanes {
        let lane = draw_values(draw, descriptor, shape.mono, &values);
        values.push(lane);
    }
    let requests: Vec<PrepareEffectRequest<'_>> =
        values.iter().map(|lane| request(shape, lane)).collect();
    let mut pairs = Vec::with_capacity(lanes);
    for member in &requests {
        match (factory.prepare(*member), factory.prepare(*member)) {
            (Ok(oracle), Ok(twin)) => pairs.push((oracle, twin)),
            (Err(first), Err(second)) => {
                assert_eq!(first.code, second.code, "prepare is deterministic");
                coverage.refused_cohorts += 1;
                return;
            }
            _ => panic!("one request prepared once and refused once"),
        }
    }
    coverage.scalar_scenarios += 1;
    let metadata = pairs[0].0.metadata();
    let sizes = metadata.state_sizes;
    let version = descriptor.state_layout_version;
    let quantum = shape.quantum as usize;
    let connected = matches!(
        shape.ports.sidechain,
        PreparedSidechainPort::Connected { .. }
    );
    // Large states (a delay line) are snapshotted rarely: writing one is most of a block's cost.
    let small = sizes.total().unwrap_or(u64::MAX) <= 1 << 16;
    let mut first = draw.pick(&[0_u64, 0, 1 << 20, (1 << 40) + 7]);
    let hostile_block = shape.hostile.then(|| {
        let half = spec.blocks / 2;
        half + draw.below(spec.blocks - half)
    });
    for block in 0..spec.blocks {
        let context = format!("scalar block {block} at sample {first} ({shape:?})");
        let terminal = hostile_block == Some(block);
        for (lane, (oracle, twin)) in pairs.iter_mut().enumerate() {
            if draw.chance(1, 8) {
                let kind = draw.pick(&[
                    ResetKind::FullToDefaults,
                    ResetKind::DiscontinuityKeepParameters,
                ]);
                oracle.reset(kind);
                twin.reset(kind);
            }
            if draw.chance(1, if small { 5 } else { 40 }) {
                // The oracle's own snapshot, into both: it must restore, write itself back word
                // for word, and leave the twins rendering alike. (Restored against *continued* is
                // not a contract mid-ramp: a payload carries a ramp's `current`, `target` and
                // `remaining`, and the restore re-derives its step.)
                let payload = snapshot_scalar(oracle.as_ref(), sizes);
                let input = || {
                    StatePayloadInput::new(&payload.common, &payload.left, &payload.right, sizes)
                        .expect("the prepared sizes")
                };
                for effect in [&mut *oracle, &mut *twin] {
                    if let Err(error) = effect.restore_state_payload(version, input()) {
                        assert!(
                            spec.known.contains(&Known::SubnormalStateRefusedOnRestore),
                            "{context}: lane {lane}'s own snapshot refused ({})",
                            error.code
                        );
                        continue;
                    }
                    assert!(
                        same_payload(&snapshot_scalar(effect.as_ref(), sizes), &payload),
                        "{context}: lane {lane}'s snapshot does not survive its own restore"
                    );
                }
                coverage.restores += 1;
            } else if shape.hostile && draw.chance(1, if small { 5 } else { 40 }) {
                // A rewritten payload into both: they accept or refuse together.
                let mut payload = snapshot_scalar(oracle.as_ref(), sizes);
                let section = match draw.below(3) {
                    0 => &mut payload.common,
                    1 => &mut payload.left,
                    _ => &mut payload.right,
                };
                rewrite_word(draw, section, terminal);
                let input = || {
                    StatePayloadInput::new(&payload.common, &payload.left, &payload.right, sizes)
                        .expect("the prepared sizes")
                };
                let (a, b) = (
                    restore_code(oracle.restore_state_payload(version, input())),
                    restore_code(twin.restore_state_payload(version, input())),
                );
                assert_eq!(a, b, "{context}: lane {lane}'s twins disagree on a restore");
                match a {
                    Ok(()) => coverage.restores += 1,
                    Err(_) => coverage.refused_restores += 1,
                }
            }
        }
        let frames = draw.frames(quantum);
        let invalid = !shape.mono && draw.chance(1, 16);
        let chunk = (frames > 1 && !shape.hostile && !invalid && draw.chance(1, 2))
            .then(|| 1 + draw.below(frames - 1));
        for (lane, (oracle, twin)) in pairs.iter_mut().enumerate() {
            let profile = if terminal && lane == 0 {
                Profile::Hostile
            } else {
                legal_profile(draw)
            };
            let other = if shape.mono {
                profile
            } else {
                legal_profile(draw)
            };
            let side_profile = legal_profile(draw);
            let mut left: Vec<f32> = (0..frames).map(|_| draw.sample(profile)).collect();
            let mut right: Vec<f32> = if shape.mono {
                left.clone()
            } else {
                (0..frames).map(|_| draw.sample(other)).collect()
            };
            let side_left: Vec<f32> = (0..frames).map(|_| draw.sample(side_profile)).collect();
            let side_right = if shape.mono {
                side_left.clone()
            } else {
                (0..frames).map(|_| draw.sample(side_profile)).collect()
            };
            let spans = draw_spans(
                draw,
                descriptor,
                first,
                frames,
                shape.capacity as usize,
                shape.mono,
                invalid,
            );
            coverage.spans += spans.len() as u64;
            let (mut twin_left, mut twin_right) = (left.clone(), right.clone());
            let side = connected.then_some((side_left.as_slice(), side_right.as_slice()));
            let oracle_report = render_scalar(
                oracle.as_mut(),
                (&mut left, &mut right),
                side,
                first,
                &spans,
                shape.quantum,
                None,
                audited_calls,
            );
            let twin_report = render_scalar(
                twin.as_mut(),
                (&mut twin_left, &mut twin_right),
                side,
                first,
                &spans,
                shape.quantum,
                chunk,
                audited_calls,
            );
            coverage.blocks += 1;
            if chunk.is_some() {
                coverage.chunked_blocks += 1;
            } else {
                assert_eq!(
                    oracle_report, twin_report,
                    "{context}: lane {lane}'s twin reports"
                );
            }
            for (plane, oracle_plane, twin_plane) in
                [("left", &left, &twin_left), ("right", &right, &twin_right)]
            {
                if let Some(frame) = first_difference(oracle_plane, twin_plane) {
                    panic!(
                        "{context}: lane {lane} {plane} frame {frame}: the twin rendered {:#010x} \
                         where the oracle rendered {:#010x} (chunk {chunk:?}, spans {spans:?})",
                        twin_plane[frame].to_bits(),
                        oracle_plane[frame].to_bits()
                    );
                }
            }
            if !shape.bypass {
                coverage.bounded_blocks += 1;
                assert!(
                    left.iter().chain(&right).all(|word| within_d7_bound(*word)),
                    "{context}: lane {lane} wrote a word outside the D7 bound"
                );
            }
            if (small && chunk.is_none()) || block + 1 == spec.blocks {
                assert_eq!(
                    snapshot_scalar(oracle.as_ref(), sizes),
                    snapshot_scalar(twin.as_ref(), sizes),
                    "{context}: lane {lane}'s twin holds other state"
                );
                coverage.state_comparisons += 1;
            }
        }
        if terminal {
            coverage.recoveries += 1;
            return;
        }
        first += frames as u64;
    }
}

fn draw_ports(draw: &mut Draw, descriptor: &'static EffectDescriptor) -> PreparedPorts {
    let sidechain = descriptor
        .ports
        .iter()
        .find(|port| port.role == PortRole::SidechainInput)
        .map_or(PreparedSidechainPort::None, |port| {
            if port.required || draw.chance(1, 3) {
                PreparedSidechainPort::Connected {
                    id: port.id,
                    required: port.required,
                }
            } else {
                PreparedSidechainPort::Unconnected {
                    id: port.id,
                    required: port.required,
                }
            }
        });
    PreparedPorts { sidechain }
}

/// A legal value of `parameter`, favouring the domain edges and the identities.
fn draw_value(draw: &mut Draw, parameter: &ParameterDescriptor) -> f32 {
    let value = match parameter.domain {
        ParameterDomain::Boolean => draw.pick(&[0.0_f32, 1.0]),
        ParameterDomain::Enumeration => draw.pick(parameter.enum_choices).value,
        ParameterDomain::Continuous => {
            let fallback = parameter.default_value;
            if draw.chance(1, 8) {
                fallback
            } else {
                draw.in_domain(
                    parameter.minimum.unwrap_or(fallback),
                    parameter.maximum.unwrap_or(fallback),
                )
            }
        }
    };
    if parameter_value_valid(parameter, value) && !is_negative_zero(value) {
        value
    } else {
        normalize_zero(parameter.default_value)
    }
}

/// One lane's initial values: drawn, or copied from an earlier lane; per-lane parameters drawn
/// channel-asymmetric unless the scenario is mono.
fn draw_values(
    draw: &mut Draw,
    descriptor: &'static EffectDescriptor,
    mono: bool,
    earlier: &[Vec<InitialParameterValue>],
) -> Vec<InitialParameterValue> {
    if !earlier.is_empty() && draw.chance(1, 4) {
        return earlier[draw.below(earlier.len())].clone();
    }
    let mut values: Vec<InitialParameterValue> = default_initial_values(descriptor).collect();
    let mut index = 0;
    while index < values.len() {
        let parameter = &descriptor.parameters[values[index].parameter_index as usize];
        let value = draw_value(draw, parameter);
        values[index].value = value;
        if parameter.channel_policy == ParameterChannelPolicy::PerLane {
            values[index + 1].value = if mono || draw.chance(1, 2) {
                value
            } else {
                draw_value(draw, parameter)
            };
            index += 2;
        } else {
            index += 1;
        }
    }
    values
}

fn limits(shape: Shape) -> PrepareEffectLimits {
    let scratch = shape
        .quality
        .scratch_bytes_per_frame
        .saturating_mul(u64::from(shape.quantum))
        .saturating_add(shape.quality.scratch_fixed_bytes);
    PrepareEffectLimits {
        maximum_total_state_bytes: shape
            .quality
            .maximum_state
            .total()
            .unwrap_or(u64::MAX)
            .max(1),
        maximum_scratch_bytes: scratch.max(1),
        maximum_automation_spans_per_block: shape.capacity,
    }
}

fn request<'a>(shape: Shape, values: &'a [InitialParameterValue]) -> PrepareEffectRequest<'a> {
    PrepareEffectRequest {
        sample_rate: shape.quality.sample_rate,
        quantum: shape.quantum,
        quality: shape.quality.quality,
        bypass: shape.bypass,
        link_mode: shape.link,
        ports: shape.ports,
        initial_values: values,
        limits: limits(shape),
    }
}

fn index_of(width: BankWidth) -> usize {
    usize::from(width == BankWidth::Eight)
}

/// The dual bank's collapsing twin, and the witness terms that decide its mode.
struct MonoArm {
    bank: Box<dyn PreparedNativeEffectBank>,
    collapsed: bool,
    /// No dual block has run under a failed witness since bind (`rack::BankChain`'s agreement
    /// invariant). Nothing restores it: every launch effect declines `channels_agree`.
    agree: bool,
    /// `RESTORED`: every restore's left and right sections compared byte-equal.
    restored: bool,
}

#[allow(clippy::too_many_lines)]
fn run_width(
    spec: &EffectDifferential<'_>,
    draw: &mut Draw,
    shape: Shape,
    width: BankWidth,
    backend: Backend,
    audited_calls: bool,
    coverage: &mut DifferentialCoverage,
) {
    let factory = spec.factory;
    let descriptor = factory.descriptor();
    let lanes = width.lanes() as usize;
    let mut values: Vec<Vec<InitialParameterValue>> = Vec::with_capacity(lanes);
    for _ in 0..lanes {
        let lane = draw_values(draw, descriptor, shape.mono, &values);
        values.push(lane);
    }
    let requests: Vec<PrepareEffectRequest<'_>> =
        values.iter().map(|lane| request(shape, lane)).collect();
    let bank_request = PrepareEffectBankRequest {
        backend,
        width,
        requests: &requests,
    };

    // Bind eligibility first: it needs no rendering.
    bind_eligibility(
        factory, draw, shape, &values, backend, width, spec.known, coverage,
    );

    // Bind first: a declined width prepares nothing more, which matters for a large state.
    let mut bank = match factory.bind_homogeneous_bank(bank_request) {
        Ok(Some(bank)) => bank,
        Ok(None) => {
            coverage.declined[index_of(width)] += 1;
            return;
        }
        Err(error) => {
            let refused = requests
                .iter()
                .find_map(|member| factory.prepare(*member).err())
                .unwrap_or_else(|| {
                    panic!(
                        "{width:?}: bind refused ({}) a cohort every member of which prepares",
                        error.code
                    )
                });
            assert_eq!(
                error.code, refused.code,
                "{width:?}: bind refused a member with a different code than prepare"
            );
            coverage.refused_cohorts += 1;
            return;
        }
    };
    let mut scalars: Vec<Box<dyn PreparedNativeEffect>> = requests
        .iter()
        .map(|member| {
            factory.prepare(*member).unwrap_or_else(|error| {
                panic!(
                    "{width:?}: bind accepted a cohort with a member prepare refuses ({})",
                    error.code
                )
            })
        })
        .collect();
    coverage.banks[index_of(width)] += 1;
    if backend == Backend::current() {
        coverage.native_banks += 1;
    }
    let metadata = scalars[0].metadata();
    assert_eq!(bank.metadata().width, width, "the bank's width");
    assert_eq!(
        bank.metadata().program_key,
        metadata.program_key(),
        "{width:?}: the bank's program key is its members'"
    );
    let sizes = metadata.state_sizes;
    let version = descriptor.state_layout_version;
    let quantum = shape.quantum as usize;
    let connected = matches!(
        shape.ports.sidechain,
        PreparedSidechainPort::Connected { .. }
    );
    let mono_capable = bank.supports_mono_collapse();
    coverage.mono_capable |= mono_capable;
    let mut arm = (shape.mono && mono_capable).then(|| MonoArm {
        bank: factory
            .bind_homogeneous_bank(bank_request)
            .expect("the same cohort binds twice")
            .expect("the same cohort binds twice"),
        collapsed: false,
        agree: true,
        restored: true,
    });
    // Large states (a delay line) are compared at the end only.
    let compare_every_block = sizes.total().unwrap_or(u64::MAX) <= 1 << 16;

    let mut first = draw.pick(&[0_u64, 0, 1 << 20, (1 << 40) + 7]);
    // One lane of one block in the second half of a hostile scenario carries the hostile words,
    // NaNs and infinities included. The track input stage sanitises non-finite words before any
    // effect sees them, so this is the contract's own D7 case rather than an engine input; and a
    // block that trips it resets the whole bank, so the scenario ends there (see below).
    // `Some(lane)`: only that lane carries automation; `Some(lanes)`: none does.
    let ramp_cuts = spec.known.contains(&Known::RampCutsMoveBits);
    let automated = ramp_cuts.then(|| {
        if draw.chance(1, 2) {
            draw.below(lanes)
        } else {
            lanes
        }
    });
    let hostile_block = shape.hostile.then(|| {
        let half = spec.blocks / 2;
        (half + draw.below(spec.blocks - half), draw.below(lanes))
    });
    for block in 0..spec.blocks {
        let context = format!("{width:?} block {block} at sample {first} ({shape:?})");
        // --- Block-boundary operations -------------------------------------------------------
        if draw.chance(1, 24) {
            let kind = draw.pick(&[
                ResetKind::FullToDefaults,
                ResetKind::DiscontinuityKeepParameters,
            ]);
            if let Some(arm) = arm.as_mut() {
                disengage(arm, bank.as_ref(), sizes, &context, coverage);
            }
            for scalar in &mut scalars {
                scalar.reset(kind);
            }
            bank.reset(kind);
            if let Some(arm) = arm.as_mut() {
                arm.bank.reset(kind);
            }
        }
        let terminal = hostile_block.is_some_and(|(at, _)| at == block);
        if draw.chance(1, 6) || (terminal && draw.chance(1, 2)) {
            restore(
                spec,
                draw,
                shape,
                terminal,
                &mut scalars,
                bank.as_mut(),
                arm.as_mut(),
                sizes,
                version,
                &metadata,
                &context,
                coverage,
            );
        }

        // --- Input -----------------------------------------------------------------------------
        let frames = draw.frames(quantum);
        let words = frames * lanes;
        let mut input_left = vec![0.0_f32; words];
        let mut input_right = vec![0.0_f32; words];
        let mut side_left = vec![0.0_f32; words];
        let mut side_right = vec![0.0_f32; words];
        for lane in 0..lanes {
            let profile = if hostile_block == Some((block, lane)) {
                Profile::Hostile
            } else {
                legal_profile(draw)
            };
            let other = if shape.mono || draw.chance(2, 3) {
                profile
            } else {
                legal_profile(draw)
            };
            let side = legal_profile(draw);
            for frame in 0..frames {
                let word = frame * lanes + lane;
                input_left[word] = draw.sample(profile);
                input_right[word] = if shape.mono {
                    input_left[word]
                } else {
                    draw.sample(other)
                };
                side_left[word] = draw.sample(side);
                side_right[word] = if shape.mono {
                    side_left[word]
                } else {
                    draw.sample(side)
                };
            }
        }
        let finite_input = input_left
            .iter()
            .chain(&input_right)
            .chain(&side_left)
            .chain(&side_right)
            .all(|word| word.is_finite());

        // --- Automation --------------------------------------------------------------------------
        let invalid = !shape.mono && draw.chance(1, 16);
        let mut spans = Vec::new();
        let mut offsets = vec![0_u32; lanes + 1];
        let mut lane_spans = Vec::with_capacity(lanes);
        for lane in 0..lanes {
            let lane_invalid = invalid && draw.chance(1, 2);
            if automated.is_some_and(|only| only != lane) {
                offsets[lane + 1] = spans.len() as u32;
                lane_spans.push(Vec::new());
                continue;
            }
            let drawn = draw_spans(
                draw,
                descriptor,
                first,
                frames,
                shape.capacity as usize,
                shape.mono,
                lane_invalid,
            );
            spans.extend_from_slice(&drawn);
            offsets[lane + 1] = spans.len() as u32;
            lane_spans.push(drawn);
        }
        coverage.spans += spans.len() as u64;

        // --- The mono arm's mode, decided before anything renders -------------------------------
        // The witness is asked of the dual bank: a collapsed bank's right channel is stale by
        // design, so asking the arm itself would compare it with a channel nothing has advanced.
        let collapse = arm.as_mut().is_some_and(|arm| {
            let designed = (0..lanes).all(|lane| bank.lane_channel_symmetry(lane));
            let eligible = arm.agree && arm.restored && designed;
            if !eligible {
                disengage(arm, bank.as_ref(), sizes, &context, coverage);
                arm.agree = false;
            }
            arm.collapsed = eligible;
            eligible
        });

        // --- Scalar oracle -----------------------------------------------------------------------
        // Only where no block-granular recovery can run: D7 zeroes a whole block whose output is
        // not finite, so a poisoned state is legitimately partition-dependent.
        let chunkable = !(ramp_cuts && automated.is_some_and(|only| only < lanes));
        let chunk = (chunkable
            && frames > 1
            && finite_input
            && !shape.hostile
            && !invalid
            && draw.chance(1, 3))
        .then(|| 1 + draw.below(frames - 1));
        let mut scalar_left = vec![0.0_f32; words];
        let mut scalar_right = vec![0.0_f32; words];
        let mut scalar_reports = Vec::with_capacity(lanes);
        for (lane, scalar) in scalars.iter_mut().enumerate() {
            let gather = |plane: &[f32]| -> Vec<f32> {
                (0..frames)
                    .map(|frame| plane[frame * lanes + lane])
                    .collect()
            };
            let (mut left, mut right) = (gather(&input_left), gather(&input_right));
            let (side_l, side_r) = (gather(&side_left), gather(&side_right));
            let report = render_scalar(
                scalar.as_mut(),
                (&mut left, &mut right),
                connected.then_some((&side_l, &side_r)),
                first,
                &lane_spans[lane],
                shape.quantum,
                chunk,
                audited_calls,
            );
            for frame in 0..frames {
                scalar_left[frame * lanes + lane] = left[frame];
                scalar_right[frame * lanes + lane] = right[frame];
            }
            scalar_reports.push(report);
        }
        if chunk.is_some() {
            coverage.chunked_blocks += 1;
        }

        // --- The bank, and its collapsing twin ------------------------------------------------------
        let (mut bank_left, mut bank_right) = (input_left.clone(), input_right.clone());
        let bank_report = {
            let block = EffectBankProcessBlock::new(
                &mut bank_left,
                &mut bank_right,
                connected.then_some((side_left.as_slice(), side_right.as_slice())),
                frames as u32,
                width,
                first,
                &spans,
                &offsets,
                shape.quantum,
            )
            .expect("a well-shaped bank block");
            audited(audited_calls, "process_bank", || bank.process_bank(block))
        };
        coverage.blocks += 1;
        if hostile_block.is_some_and(|(at, _)| at == block)
            || recovered(&bank_report, &scalar_reports, lanes)
        {
            // D7 (docs/EFFECT_CONTRACT_V1.md): output finiteness is checked once per block *per
            // bank*, and a failing block zeroes that bank's output and resets its state -- every
            // lane's, not only the failing one's, in most launch effects. So on the block the
            // hostile words arrive in, and on any block that trips the bound, the bank and its
            // scalar instances may part by design: both must keep every word inside the bound,
            // and the scenario ends there.
            if !shape.bypass {
                for (who, planes) in [
                    ("bank", [&bank_left, &bank_right]),
                    ("scalar oracle", [&scalar_left, &scalar_right]),
                ] {
                    assert!(
                        planes
                            .iter()
                            .all(|plane| plane.iter().all(|word| within_d7_bound(*word))),
                        "{context}: the {who} wrote a word outside the D7 bound"
                    );
                }
            }
            coverage.recoveries += 1;
            return;
        }
        for lane in 0..lanes {
            for (plane, scalar, banked) in [
                ("left", &scalar_left, &bank_left),
                ("right", &scalar_right, &bank_right),
            ] {
                if let Some(frame) = (0..frames).find(|frame| {
                    !same_word(scalar[frame * lanes + lane], banked[frame * lanes + lane])
                }) {
                    let word = frame * lanes + lane;
                    panic!(
                        "{context}: lane {lane} {plane} frame {frame}: the bank rendered {:#010x} \
                         where its scalar instance rendered {:#010x} (chunk {chunk:?}, bank report \
                         {:?}, scalar report {:?}, spans {:?})",
                        banked[word].to_bits(),
                        scalar[word].to_bits(),
                        bank_report.reports[lane],
                        scalar_reports[lane],
                        lane_spans[lane],
                    );
                }
            }
            if chunk.is_none() {
                assert_eq!(
                    bank_report.reports[lane], scalar_reports[lane],
                    "{context}: lane {lane}'s bank report is not its scalar instance's"
                );
            }
        }
        if !shape.bypass {
            coverage.bounded_blocks += 1;
            if let Some(word) = bank_left
                .iter()
                .chain(&bank_right)
                .position(|word| !within_d7_bound(*word))
            {
                panic!(
                    "{context}: an unbypassed effect wrote {:#010x} (word {word}), outside the D7 \
                     block bound",
                    bank_left
                        .iter()
                        .chain(&bank_right)
                        .nth(word)
                        .map_or(0, |value| value.to_bits())
                );
            }
        }
        if let Some(arm) = arm.as_mut() {
            let mut left = input_left.clone();
            let mut right = if collapse {
                // `block.right` is not gathered on a collapsed block; reading it is a defect.
                vec![f32::from_bits(0x7fc0_dead); words]
            } else {
                input_right.clone()
            };
            let block = EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                connected.then_some((side_left.as_slice(), side_right.as_slice())),
                frames as u32,
                width,
                first,
                &spans,
                &offsets,
                shape.quantum,
            )
            .expect("a well-shaped bank block");
            let report = if collapse {
                audited(audited_calls, "process_bank_mono", || {
                    arm.bank.process_bank_mono(block)
                })
            } else {
                audited(audited_calls, "process_bank", || {
                    arm.bank.process_bank(block)
                })
            };
            assert_eq!(
                report_lanes(&report, lanes),
                report_lanes(&bank_report, lanes),
                "{context}: the {} arm's report is not the dual bank's",
                if collapse { "collapsed" } else { "dual" }
            );
            if let Some(word) = first_difference(&left, &bank_left) {
                panic!(
                    "{context}: the {} arm rendered left word {word} as {:#010x}, the dual bank \
                     {:#010x}",
                    if collapse { "collapsed" } else { "twin" },
                    left[word].to_bits(),
                    bank_left[word].to_bits()
                );
            }
            if collapse {
                coverage.collapsed_blocks += 1;
                if let Some(word) = first_difference(&bank_right, &bank_left) {
                    panic!(
                        "{context}: every lane's witness held and both planes carried one signal, \
                         yet the dual bank rendered right word {word} as {:#010x} and left as \
                         {:#010x}: the witness was wrong",
                        bank_right[word].to_bits(),
                        bank_left[word].to_bits()
                    );
                }
            } else if let Some(word) = first_difference(&right, &bank_right) {
                panic!("{context}: the twin arm rendered right word {word} differently");
            }
        }

        // --- State ------------------------------------------------------------------------------
        if compare_every_block || block + 1 == spec.blocks {
            compare_state(&scalars, bank.as_ref(), sizes, &context, coverage);
        }
        first += frames as u64;
    }
}

/// Ends a collapsed run: `desymmetrize_channels`, then every lane's state must be the dual bank's.
fn disengage(
    arm: &mut MonoArm,
    bank: &dyn PreparedNativeEffectBank,
    sizes: StatePayloadSizes,
    context: &str,
    coverage: &mut DifferentialCoverage,
) {
    if !arm.collapsed {
        return;
    }
    arm.bank.desymmetrize_channels();
    arm.collapsed = false;
    coverage.disengages += 1;
    for lane in 0..arm.bank.metadata().width.lanes() as usize {
        assert_eq!(
            snapshot_lane(arm.bank.as_ref(), lane, sizes),
            snapshot_lane(bank, lane, sizes),
            "{context}: lane {lane}'s state after desymmetrize_channels is not the dual bank's"
        );
    }
}

/// D7's block bound, `x == x && |x| < 1e30`: `false` for a NaN, an infinity and anything that
/// large.
fn within_d7_bound(word: f32) -> bool {
    word.abs() < 1.0e30
}

/// Whether any lane of the bank or of its scalar instances failed the D7 bound this block.
fn recovered(bank: &BankProcessReport, scalars: &[ProcessReport], lanes: usize) -> bool {
    bank.reports[..lanes]
        .iter()
        .chain(scalars)
        .any(|report| report.nonfinite_left_blocks + report.nonfinite_right_blocks > 0)
}

fn report_lanes(report: &BankProcessReport, lanes: usize) -> Vec<ProcessReport> {
    report.reports[..lanes].to_vec()
}

/// A profile of legal input: finite, and at most `4.0` in magnitude.
fn legal_profile(draw: &mut Draw) -> Profile {
    loop {
        let profile = draw.profile();
        if profile.is_finite_legal() {
            return profile;
        }
    }
}

/// Up to `capacity` valid spans for one lane, in the contract's canonical order, or one extra
/// invalid span when `invalid`. In a mono scenario every per-lane span is a bit-equal `Left`/`Right`
/// twin, which is the pairing rule's one both-channel write.
fn draw_spans(
    draw: &mut Draw,
    descriptor: &'static EffectDescriptor,
    first: u64,
    frames: usize,
    capacity: usize,
    mono: bool,
    invalid: bool,
) -> Vec<PreparedAutomationSpan> {
    let automatable: Vec<usize> = descriptor
        .parameters
        .iter()
        .enumerate()
        .filter(|(_, parameter)| parameter.automation_rate != AutomationRate::None)
        .map(|(index, _)| index)
        .collect();
    let mut spans = Vec::new();
    if !automatable.is_empty() && draw.chance(1, 3) {
        for _ in 0..=draw.below(capacity) {
            let index = draw.pick(&automatable);
            let parameter = &descriptor.parameters[index];
            let (kind, start, end) = if parameter.automation_rate == AutomationRate::Block {
                (AutomationSpanKind::Point, first, first)
            } else {
                let offset = if draw.chance(1, 3) {
                    draw.pick(&[0, frames - 1])
                } else {
                    draw.below(frames)
                };
                let start = first + offset as u64;
                let kind = draw.pick(&[
                    AutomationSpanKind::Point,
                    AutomationSpanKind::Point,
                    AutomationSpanKind::Step,
                    AutomationSpanKind::Linear,
                    AutomationSpanKind::Linear,
                    AutomationSpanKind::Exponential,
                ]);
                let end = if kind == AutomationSpanKind::Point {
                    start
                } else {
                    start + 1 + draw.below(2 * frames) as u64
                };
                (kind, start, end)
            };
            // One span per parameter and start sample: two would sort as `L L R R`, which is no
            // longer twin pairs.
            if spans.iter().any(|span: &PreparedAutomationSpan| {
                span.parameter_index == index as u32 && span.start_sample == start
            }) {
                continue;
            }
            let start_value = draw_value(draw, parameter);
            let end_value = if kind == AutomationSpanKind::Point {
                start_value
            } else {
                draw_value(draw, parameter)
            };
            let kind = if kind == AutomationSpanKind::Exponential
                && !(start_value != 0.0
                    && end_value != 0.0
                    && start_value.is_sign_positive() == end_value.is_sign_positive())
            {
                AutomationSpanKind::Linear
            } else {
                kind
            };
            let channels: &[ParameterChannel] = match parameter.channel_policy {
                ParameterChannelPolicy::Shared => &[ParameterChannel::Both],
                ParameterChannelPolicy::PerLane if mono => {
                    &[ParameterChannel::Left, ParameterChannel::Right]
                }
                ParameterChannelPolicy::PerLane => {
                    if draw.chance(1, 2) {
                        &[ParameterChannel::Left]
                    } else {
                        &[ParameterChannel::Right]
                    }
                }
            };
            for channel in channels {
                spans.push(PreparedAutomationSpan {
                    kind,
                    channel: *channel,
                    parameter_index: index as u32,
                    start_sample: start,
                    end_sample: end,
                    start_value,
                    end_value,
                });
            }
        }
    }
    spans.sort_by_key(|span| (span.start_sample, span.parameter_index, span.channel));
    // Per (parameter, channel), a span starts at or after the previous one's end, and one that
    // starts exactly there continues its value. Twins share a history, so they stay or go together.
    let mut last: Vec<((u32, ParameterChannel), u64, u32)> = Vec::new();
    spans.retain(|span| {
        let key = (span.parameter_index, span.channel);
        if let Some(entry) = last.iter_mut().find(|entry| entry.0 == key) {
            if span.start_sample < entry.1
                || (span.start_sample == entry.1 && canonical_bits(span.start_value) != entry.2)
            {
                return false;
            }
            entry.1 = span.end_sample;
            entry.2 = canonical_bits(span.end_value);
        } else {
            last.push((key, span.end_sample, canonical_bits(span.end_value)));
        }
        true
    });
    spans.truncate(capacity);
    if mono
        && spans
            .last()
            .is_some_and(|span| span.channel == ParameterChannel::Left)
    {
        spans.pop();
    }
    if invalid {
        let parameter = draw.below(descriptor.parameters.len() + 1) as u32;
        let value = draw.pick(&[f32::NAN, f32::INFINITY, 1.0e30, -1.0e30, -0.0, 0.5]);
        spans.push(PreparedAutomationSpan {
            kind: draw.pick(&[AutomationSpanKind::Point, AutomationSpanKind::Linear]),
            channel: draw.pick(&[
                ParameterChannel::Left,
                ParameterChannel::Right,
                ParameterChannel::Both,
            ]),
            parameter_index: parameter,
            start_sample: first + frames as u64 - 1,
            end_sample: first + frames as u64 - 1,
            start_value: value,
            end_value: value,
        });
    }
    spans
}

#[allow(clippy::too_many_arguments)]
fn render_scalar(
    effect: &mut dyn PreparedNativeEffect,
    (left, right): (&mut [f32], &mut [f32]),
    side: Option<(&[f32], &[f32])>,
    first: u64,
    spans: &[PreparedAutomationSpan],
    quantum: u32,
    chunk: Option<usize>,
    audited_calls: bool,
) -> ProcessReport {
    let frames = left.len();
    let cut = chunk.unwrap_or(frames);
    let mut total = ProcessReport::default();
    for (start, end) in [(0, cut), (cut, frames)] {
        if start == end {
            continue;
        }
        let from = first + start as u64;
        let to = first + end as u64;
        let pieces: Vec<PreparedAutomationSpan> = spans
            .iter()
            .filter(|span| chunk.is_none() || (span.start_sample >= from && span.start_sample < to))
            .copied()
            .collect();
        let block = EffectProcessBlock::new(
            &mut left[start..end],
            &mut right[start..end],
            side.map(|(l, r)| (&l[start..end], &r[start..end])),
            from,
            &pieces,
            quantum,
        )
        .expect("a well-shaped block");
        let report = audited(audited_calls, "process", || effect.process(block));
        total.sanitized_main_samples += report.sanitized_main_samples;
        total.sanitized_sidechain_samples += report.sanitized_sidechain_samples;
        total.invalid_spans += report.invalid_spans;
        total.nonfinite_left_blocks += report.nonfinite_left_blocks;
        total.nonfinite_right_blocks += report.nonfinite_right_blocks;
    }
    total
}

fn empty_payload(sizes: StatePayloadSizes) -> Payload {
    Payload {
        common: vec![0; sizes.common_bytes as usize],
        left: vec![0; sizes.left_bytes as usize],
        right: vec![0; sizes.right_bytes as usize],
    }
}

fn snapshot_scalar(effect: &dyn PreparedNativeEffect, sizes: StatePayloadSizes) -> Payload {
    let mut payload = empty_payload(sizes);
    let output = StatePayloadOutput::new(
        &mut payload.common,
        &mut payload.left,
        &mut payload.right,
        sizes,
    )
    .expect("the prepared sizes");
    effect
        .snapshot_state_payload(output)
        .expect("a prepared instance snapshots");
    payload
}

fn snapshot_lane(
    bank: &dyn PreparedNativeEffectBank,
    lane: usize,
    sizes: StatePayloadSizes,
) -> Payload {
    let mut payload = empty_payload(sizes);
    let output = StatePayloadOutput::new(
        &mut payload.common,
        &mut payload.left,
        &mut payload.right,
        sizes,
    )
    .expect("the prepared sizes");
    bank.snapshot_track_state_payload(lane as u32, output)
        .expect("a bank lane snapshots");
    payload
}

/// Whether two payloads carry the same words, where a restore may write a signed zero back as
/// `+0.0` (the multiband compressor and the compressor normalise the zeros they read).
fn same_payload(a: &Payload, b: &Payload) -> bool {
    let zero = |word: u32| word & 0x7fff_ffff == 0;
    [
        (&a.common, &b.common),
        (&a.left, &b.left),
        (&a.right, &b.right),
    ]
    .into_iter()
    .all(|(x, y)| {
        x.len() == y.len()
            && x.chunks_exact(4).zip(y.chunks_exact(4)).all(|(p, q)| {
                let (p, q) = (
                    u32::from_le_bytes(p.try_into().expect("a word")),
                    u32::from_le_bytes(q.try_into().expect("a word")),
                );
                p == q || (zero(p) && zero(q))
            })
    })
}

fn compare_state(
    scalars: &[Box<dyn PreparedNativeEffect>],
    bank: &dyn PreparedNativeEffectBank,
    sizes: StatePayloadSizes,
    context: &str,
    coverage: &mut DifferentialCoverage,
) {
    for (lane, scalar) in scalars.iter().enumerate() {
        let (expected, banked) = (
            snapshot_scalar(scalar.as_ref(), sizes),
            snapshot_lane(bank, lane, sizes),
        );
        if expected != banked {
            let section = if expected.common != banked.common {
                "common"
            } else if expected.left != banked.left {
                "left"
            } else {
                "right"
            };
            panic!(
                "{context}: lane {lane}'s bank state differs from its scalar instance's in the {section} section"
            );
        }
        coverage.state_comparisons += 1;
    }
}

/// Edge words for a payload rewrite at any block: both zeros, subnormals, the smallest normal,
/// `+-1`, and small counts. Every one is a finite `f32`.
const PAYLOAD_WORDS: [u32; 13] = [
    0,
    0x8000_0000,
    1,
    2,
    3,
    0x8000_0001,
    0x007f_ffff,
    0x0080_0000,
    0x3f80_0000,
    0xbf80_0000,
    63,
    64,
    65,
];

/// The words a restore may carry only right before the hostile block, because a state that holds
/// one can trip D7 (see `run_width`): the infinities, NaN payloads, the largest finite words and
/// an all-ones count.
const TERMINAL_WORDS: [u32; 7] = [
    0x7f80_0000,
    0xff80_0000,
    0x7fc0_0000,
    0x7f80_0001,
    0x7f7f_ffff,
    0xff7f_ffff,
    0xffff_ffff,
];

fn rewrite_word(draw: &mut Draw, section: &mut [u8], terminal: bool) {
    if section.len() < 4 {
        return;
    }
    let word = draw.below(section.len() / 4) * 4;
    let value = if terminal && draw.chance(1, 2) {
        draw.pick(&TERMINAL_WORDS)
    } else if draw.chance(1, 3) {
        draw.subnormal().to_bits()
    } else {
        draw.pick(&PAYLOAD_WORDS)
    };
    section[word..word + 4].copy_from_slice(&value.to_le_bytes());
}

fn restore_code(
    result: Result<(), effect_contract::StatePayloadError>,
) -> Result<(), &'static str> {
    result.map_err(|error| error.code)
}

/// One restore event: a payload drawn from the lane's own state, another lane's, a rewrite or the
/// crafting hook, restored into the scalar instance and every bank lane that mirrors it.
#[allow(clippy::too_many_arguments)]
fn restore(
    spec: &EffectDifferential<'_>,
    draw: &mut Draw,
    shape: Shape,
    terminal: bool,
    scalars: &mut [Box<dyn PreparedNativeEffect>],
    bank: &mut dyn PreparedNativeEffectBank,
    arm: Option<&mut MonoArm>,
    sizes: StatePayloadSizes,
    version: u32,
    metadata: &PreparedEffectMetadata,
    context: &str,
    coverage: &mut DifferentialCoverage,
) {
    let lanes = scalars.len();
    let lane = draw.below(lanes);
    // Under `RampCutsMoveBits` a restore may not start a ramp on a lane the scenario keeps free
    // of them, so it restores the lane's own untouched snapshot only.
    let untouched = spec.known.contains(&Known::RampCutsMoveBits);
    let source = if !untouched && draw.chance(1, 4) {
        draw.below(lanes)
    } else {
        lane
    };
    let mut payload = snapshot_scalar(scalars[source].as_ref(), sizes);
    let mut claim = None;
    let mut rewritten = true;
    match if untouched { 3 } else { draw.below(4) } {
        0 if spec.craft.is_some() => {
            claim = spec.craft.expect("checked")(draw, metadata, &mut payload);
            coverage.crafted_restores += 1;
        }
        1 if shape.hostile => {
            for _ in 0..=draw.below(3) {
                let section = match draw.below(3) {
                    0 => &mut payload.common,
                    1 => &mut payload.left,
                    _ => &mut payload.right,
                };
                rewrite_word(draw, section, terminal);
            }
        }
        2 if shape.mono => {
            // A symmetric payload keeps `RESTORED`, so the collapse can engage after it.
            rewrite_word(draw, &mut payload.left, false);
            payload.right = payload.left.clone();
        }
        _ => rewritten = false,
    }
    let mut arm = arm;
    if let Some(arm) = arm.as_deref_mut() {
        disengage(arm, bank, sizes, context, coverage);
    }
    let input = || {
        StatePayloadInput::new(&payload.common, &payload.left, &payload.right, sizes)
            .expect("the prepared sizes")
    };
    let scalar = restore_code(scalars[lane].restore_state_payload(version, input()));
    let banked = restore_code(bank.restore_track_state_payload(lane as u32, version, input()));
    assert_eq!(
        banked, scalar,
        "{context}: lane {lane}'s bank and scalar restores disagree"
    );
    if let Some(arm) = arm {
        let twin = restore_code(arm.bank.restore_track_state_payload(
            lane as u32,
            version,
            input(),
        ));
        assert_eq!(
            twin, scalar,
            "{context}: lane {lane}'s twin bank restore disagrees"
        );
        if scalar.is_ok() && payload.left != payload.right {
            arm.restored = false;
        }
    }
    match scalar {
        Ok(()) => coverage.restores += 1,
        Err(_) => coverage.refused_restores += 1,
    }
    let tolerated = spec.known.contains(&Known::SubnormalStateRefusedOnRestore);
    if !rewritten && (scalar.is_ok() || (source == lane && !tolerated)) {
        // An untouched snapshot restores and writes itself back word for word, from the scalar
        // instance and from the bank lane. The lane's own always restores; another lane's may be
        // refused where a payload is bound to its lane's configuration (the EQ's is).
        assert_eq!(
            scalar,
            Ok(()),
            "{context}: lane {lane} refused its own snapshot"
        );
        assert!(
            same_payload(&snapshot_scalar(scalars[lane].as_ref(), sizes), &payload),
            "{context}: lane {lane}'s snapshot does not survive its own restore"
        );
        assert!(
            same_payload(&snapshot_lane(bank, lane, sizes), &payload),
            "{context}: bank lane {lane}'s snapshot does not survive its own restore"
        );
    }
    if let (Ok(()), Some(symmetric)) = (scalar, claim) {
        coverage.witness_checks += 1;
        assert_eq!(
            scalars[lane].channel_symmetry(),
            symmetric,
            "{context}: the scalar witness after a crafted restore"
        );
        assert_eq!(
            bank.lane_channel_symmetry(lane),
            symmetric,
            "{context}: lane {lane}'s bank witness after a crafted restore"
        );
    }
}

/// The three-outcome rule on `bind_homogeneous_bank`, on cohorts built to break it.
#[allow(clippy::too_many_arguments)]
fn bind_eligibility(
    factory: &dyn NativeEffectFactory,
    draw: &mut Draw,
    shape: Shape,
    values: &[Vec<InitialParameterValue>],
    backend: Backend,
    width: BankWidth,
    known: &[Known],
    coverage: &mut DifferentialCoverage,
) {
    let descriptor = factory.descriptor();
    let lanes = values.len();
    let base: Vec<PrepareEffectRequest<'_>> =
        values.iter().map(|lane| request(shape, lane)).collect();

    // A malformed shape: the wrong member count, or a backend of another width.
    let short = &base[..lanes - 1];
    let mut long = base.clone();
    long.push(base[0]);
    let other_backend = match backend {
        Backend::Simd4 => Backend::Simd8,
        _ => Backend::Simd4,
    };
    for (label, requests, backend) in [
        ("one member short", short, backend),
        ("one member long", &long[..], backend),
        ("another width's backend", &base[..], other_backend),
        ("the scalar backend", &base[..], Backend::Scalar),
    ] {
        match factory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests,
        }) {
            Err(error) if error.code == "effect.bank.requests" => coverage.malformed_refusals += 1,
            Err(error) => panic!(
                "{width:?}, {label}: refused with {}, not effect.bank.requests",
                error.code
            ),
            Ok(_) => panic!("{width:?}, {label}: a malformed bank request bound or declined"),
        }
    }

    // One member differs in one program-key field, or carries an illegal initial value.
    let member = draw.below(lanes);
    let mut varied = base.clone();
    let mut illegal = values[member].clone();
    let rows: Vec<QualityDescriptor> = descriptor
        .qualities
        .iter()
        .copied()
        .filter(|row| {
            row.sample_rate != shape.quality.sample_rate || row.quality != shape.quality.quality
        })
        .collect();
    let links: Vec<LinkMode> = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average]
        .into_iter()
        .filter(|link| *link != shape.link && descriptor.supported_link_modes.contains(*link))
        .collect();
    let mut expect_decline = true;
    match draw.below(6) {
        0 => varied[member].bypass = !shape.bypass,
        1 if !links.is_empty() => varied[member].link_mode = draw.pick(&links),
        2 if !rows.is_empty() => {
            let row = draw.pick(&rows);
            varied[member].sample_rate = row.sample_rate;
            varied[member].quality = row.quality;
        }
        3 => varied[member].quantum = if shape.quantum == 128 { 64 } else { 128 },
        4 => match shape.ports.sidechain {
            PreparedSidechainPort::Connected { id, required } if !required => {
                varied[member].ports.sidechain =
                    PreparedSidechainPort::Unconnected { id, required };
            }
            PreparedSidechainPort::Unconnected { id, required } => {
                varied[member].ports.sidechain = PreparedSidechainPort::Connected { id, required };
            }
            _ => varied[member].bypass = !shape.bypass,
        },
        _ => {
            let slot = draw.below(illegal.len());
            let parameter = &descriptor.parameters[illegal[slot].parameter_index as usize];
            illegal[slot].value = match parameter.domain {
                ParameterDomain::Continuous => draw.pick(&[
                    f32::NAN,
                    -0.0,
                    parameter.maximum.unwrap_or(0.0).next_up(),
                    parameter.minimum.unwrap_or(0.0).next_down(),
                ]),
                _ => draw.pick(&[f32::NAN, 0.5, f32::from_bits(1)]),
            };
            varied[member].initial_values = &illegal;
            expect_decline = false;
        }
    }
    let prepared = factory.prepare(varied[member]);
    let bound = factory.bind_homogeneous_bank(PrepareEffectBankRequest {
        backend,
        width,
        requests: &varied,
    });
    match (prepared, bound) {
        (Err(member_error), Err(bank_error)) => assert_eq!(
            bank_error.code, member_error.code,
            "{width:?}: bind refused an illegal member with another code than prepare"
        ),
        (Err(_), Ok(None)) if known.contains(&Known::BindDeclinesBeforeValidating) => {}
        (Err(member_error), Ok(_)) => panic!(
            "{width:?}: bind accepted a member prepare refuses ({})",
            member_error.code
        ),
        (Ok(_), Ok(None)) if expect_decline => coverage.heterogeneous_declines += 1,
        (Ok(_), Ok(None)) => {}
        (Ok(_), Ok(Some(_))) if expect_decline => {
            panic!("{width:?}: bind banked a cohort whose member {member} has another program key")
        }
        (Ok(_), Ok(Some(_))) => {}
        (Ok(_), Err(error)) => panic!(
            "{width:?}: bind refused ({}) a cohort every member of which prepares",
            error.code
        ),
    }
}

/// The reach every effect's differential must show, whatever the effect: the audit armed, a bank
/// bound, blocks compared, chunked and restored, and every bind-eligibility probe answered. An
/// effect that supports the mono collapse must also have collapsed and disengaged.
///
/// Replaying one seed (`MISO_ENGINE_RANDOMIZED_SEED`) asserts nothing here: one seed is not
/// expected to reach everything.
///
/// # Panics
///
/// When a part of the harness was never reached, which makes the gate weaker than it reads.
pub fn assert_reached(coverage: &DifferentialCoverage) {
    if dsp_reference::randomized::replaying() {
        return;
    }
    assert!(
        coverage.audited,
        "the allocation audit cannot fail in this binary: install bench_support's allocator and \
         enable conformance's realtime-audit feature"
    );
    assert!(
        coverage.banks.iter().sum::<u64>() > 0 || coverage.scalar_scenarios > 0,
        "neither a bank nor a scalar twin was compared: {coverage:?}"
    );
    assert!(
        !coverage.banks_natively || coverage.native_banks > 0,
        "the effect bound no bank at the width this build executes: {coverage:?}"
    );
    assert!(
        coverage.blocks > 0 && coverage.chunked_blocks > 0 && coverage.bounded_blocks > 0,
        "blocks were not compared whole, chunked and bounded: {coverage:?}"
    );
    assert!(coverage.restores > 0, "nothing was restored: {coverage:?}");
    assert!(
        coverage.malformed_refusals > 0 && coverage.heterogeneous_declines > 0,
        "the bind-eligibility probes were not answered: {coverage:?}"
    );
    assert!(
        !coverage.mono_capable || (coverage.collapsed_blocks > 0 && coverage.disengages > 0),
        "a bank that supports the collapse never collapsed and disengaged: {coverage:?}"
    );
}

/// One effect crate's randomized differential test: [`run_effect_differential`] over `$factory`,
/// then [`assert_reached`].
///
/// The consumer's `[dev-dependencies]` carry `conformance` with its `realtime-audit` feature and
/// `bench-support`, exactly as for `effect_conformance_test!`, and the macro must sit in a
/// `tests/randomized.rs` binary with no other `#[global_allocator]`: that is what lets the harness
/// see an allocation inside `process`. A failing seed prints the command that replays it.
#[macro_export]
macro_rules! randomized_effect_test {
    ($name:ident, $factory:expr, seeds: $seeds:expr, blocks: $blocks:expr, craft: $craft:expr,
     known: $known:expr, banks_natively: $banks:expr $(,)?) => {
        #[test]
        fn $name() {
            ::bench_support::alloc::assert_installed();
            ::bench_support::alloc::set_mode(::bench_support::alloc::Mode::Count);
            let coverage = $crate::run_effect_differential(&$crate::EffectDifferential {
                factory: &$factory,
                test: stringify!($name),
                replay: concat!(
                    "cargo test -p ",
                    env!("CARGO_PKG_NAME"),
                    " --test randomized -- --exact ",
                    stringify!($name)
                ),
                seeds: $seeds,
                blocks: $blocks,
                craft: $craft,
                known: $known,
                banks_natively: $banks,
            });
            println!("{coverage:#?}");
            $crate::assert_reached(&coverage);
        }
    };
}
