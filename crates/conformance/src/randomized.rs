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
//! * **restored against continued.** At block boundaries -- always at the first one where a ramp
//!   the lane's automation started may be in flight (within 64 samples of the span) and
//!   preferentially after it, at every quantum, 32 included -- a lane's snapshot is restored into a freshly prepared instance,
//!   which then renders beside the lane: it must render and report exactly what the lane renders
//!   by continuing. A payload carries every word a continuation reads (#1278).
//! * **bind eligibility.** A cohort that differs in one program-key field declines, a malformed
//!   shape refuses with `effect.bank.requests`, and an invalid member refuses with the code
//!   `prepare` gives it (the three-outcome rule on `bind_homogeneous_bank`).
//!
//! And the invariants a differential cannot see when both sides share a defect: an unbypassed
//! effect keeps every output word inside the D7 block bound (`|x| < 1e30`, never NaN) whatever the
//! input, and no `process` call and no state-payload call (snapshot or restore, scalar or bank
//! lane: the plan-swap carry makes them in the swap block) allocates, frees, locks, logs or makes a
//! syscall.
//!
//! Comparisons treat every NaN as one value (`dsp_reference::randomized::same_word`, issue #1065):
//! the payload of a NaN an operation generates is the CPU's choice. Everything else, the sign of
//! zero included, is compared by bits.

use dsp_reference::randomized::{Draw, Profile, first_difference, run_seeds, same_word};
use effect_contract::{
    AutomationRate, AutomationSpanKind, BankProcessReport, BankWidth, EffectBankProcessBlock,
    EffectDescriptor, EffectProcessBlock, EffectTargetRequest, InitialParameterValue, LinkMode,
    NativeEffectFactory, NativeEffectTargetPreparation, PREPARED_EFFECT_TARGET_WORDS,
    ParameterChannel, ParameterChannelPolicy, ParameterDescriptor, ParameterDomain, PortRole,
    PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan,
    PreparedEffectMetadata, PreparedEffectTarget, PreparedNativeEffect, PreparedNativeEffectBank,
    PreparedPorts, PreparedSidechainPort, ProcessReport, QualityDescriptor, ResetKind,
    StatePayloadInput, StatePayloadOutput, StatePayloadSizes, canonical_bits,
    default_initial_values, is_negative_zero, normalize_zero, parameter_value_valid,
};
use engine::realtime::audit;
use lane::Backend;

/// How long after a span's last sample a ramp it started may still be in flight: every banked
/// launch effect smooths a parameter change over 64 samples (or 64 per-sample updates). The
/// harness cannot read an effect's ramps, so this is what "mid-ramp" means to its coverage.
const IN_FLIGHT_SAMPLES: u64 = 64;

/// The render quanta a scenario runs at (128 twice, as the common host quantum).
///
/// A scenario indexes them by its seed rather than drawing one, so every five consecutive seeds
/// visit every quantum (`32` at seed 3) at every bank width the build has, and
/// [`assert_reached`]'s quantum-32 clause needs only a ramp in flight in that scenario rather than
/// a lucky draw. A 4-lane (NEON/simd128) build binds one width per seed where an AVX2 build may
/// bind two, and drawn, twelve multiband seeds left the 4-lane build no automated quantum-32
/// scenario (#1278 follow-up).
const QUANTA: [u32; 5] = [128, 128, 64, 32, 256];

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
    /// decline only at a width this build does not execute, and only when `prepare` refuses the
    /// member with `effect.parameter.initial`.
    ///
    /// Owned by #1070.
    BindDeclinesBeforeValidating,
    /// A lane's rendered bits depend on where its in-flight ramps are cut: by another lane's
    /// retarget in the same bank (so the bank is not its scalar instances), and by a block
    /// boundary (so the render is not partition-invariant, master plan P1). Narrowing: a scenario
    /// either carries automation, on one lane only, or renders chunked blocks, never both; and a
    /// restore carries the lane's own untouched snapshot, so it starts no ramp elsewhere.
    ///
    /// Owned by #1069.
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
    /// Whether the effect derives the channel-symmetry witness (`channel_symmetry`,
    /// `lane_channel_symmetry`) rather than taking the contract's declining default. On a mono
    /// scenario, whose every parameter, automation span and restore is channel-symmetric, the
    /// witness must then hold on every lane, and a scalar instance must render its two channels
    /// alike.
    pub witness: bool,
    /// The effect's own reading of a snapshot: whether any ramp in it is in flight. Given, the
    /// in-flight coverage counts the continuations whose snapshot it says are mid-ramp, exactly;
    /// absent, it counts by the harness's proxy (see
    /// [`DifferentialCoverage::continuations_in_flight`]). The proxy over-counts where an
    /// accepted automation event can leave every ramp settled, which a prepared target that moves
    /// only a disabled band's parameters does (the EQ's).
    pub in_flight: Option<fn(&Payload) -> bool>,
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
    /// Whether the effect under test takes prepared targets (`target_preparation`), the route its
    /// automation enters by when it refuses raw spans (the parametric EQ).
    pub prepared_targets: bool,
    /// Prepared targets applied at a block boundary, to the scalar instance, its bank lane and
    /// any twin of that lane at once, and accepted by all of them.
    pub applied_targets: u64,
    /// Of those, the ones whose words differ from the words the lane was known to be heading for.
    /// Each starts a ramp unless its designed words are the lane's current ones (a disabled EQ
    /// band designs the identity whatever its parameters), so this is an upper bound; the
    /// in-flight clauses are what prove ramps moved.
    pub moving_targets: u64,
    /// Resets applied to every side at a block boundary.
    pub resets: u64,
    /// Restores both sides accepted.
    pub restores: u64,
    /// Restores both sides refused with one code.
    pub refused_restores: u64,
    /// Restores of a payload the effect's crafting hook wrote.
    pub crafted_restores: u64,
    /// Lane snapshots restored into a fresh instance, which then rendered beside the lane it was
    /// taken from (restored against continued).
    pub continuations: u64,
    /// Of those, the ones taken mid-ramp: as [`EffectDifferential::in_flight`] reads the lane's
    /// snapshot when the effect gives that reading, else by proxy, while a ramp the lane's
    /// automation started may be in flight: at a boundary less than 64 samples after the last
    /// sample of a span the lane **accepted**
    /// (its report counted no invalid span that block), or after a moving prepared target the
    /// lane accepted, with no reset and no replacing restore since (every banked launch effect
    /// smooths a change over 64 samples). A span the effect refused -- the EQ refuses every raw
    /// span -- starts nothing and credits nothing.
    pub continuations_in_flight: u64,
    /// Of those, the ones taken in a scenario whose quantum is 32, under the 64-sample ramps.
    pub continuations_in_flight_at_32: u64,
    /// Blocks a restored instance rendered beside its continued lane, compared word for word.
    pub continued_blocks: u64,
    /// Witness answers checked against a crafting hook's claim.
    pub witness_checks: u64,
    /// Blocks on a mono scenario whose every lane's witness was required to hold, and held.
    pub witness_holds: u64,
    /// Whether the effect under test derives the witness.
    pub witness: bool,
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
        self.prepared_targets |= other.prepared_targets;
        self.applied_targets += other.applied_targets;
        self.moving_targets += other.moving_targets;
        self.resets += other.resets;
        self.restores += other.restores;
        self.refused_restores += other.refused_restores;
        self.crafted_restores += other.crafted_restores;
        self.continuations += other.continuations;
        self.continuations_in_flight += other.continuations_in_flight;
        self.continuations_in_flight_at_32 += other.continuations_in_flight_at_32;
        self.continued_blocks += other.continued_blocks;
        self.witness_checks += other.witness_checks;
        self.witness_holds += other.witness_holds;
        self.witness |= other.witness;
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
        witness: spec.witness,
        prepared_targets: spec.factory.target_preparation().is_some(),
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

/// Runs one render-thread call -- `process`, `process_bank`, or a state-payload call, which the
/// plan-swap carry makes in the swap block -- inside an armed render scope, and fails on any
/// forbidden operation.
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
        "{what}: forbidden operations inside a render-thread call: {observed:?}"
    );
    result
}

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
    /// Under [`Known::RampCutsMoveBits`]: the scenario carries automation (on one lane) rather
    /// than chunked blocks. Indexed by the seed like the quantum, so every ten consecutive seeds
    /// pair every quantum with both, at every width (seed 3 automates at quantum 32).
    automate: bool,
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
        quantum: QUANTA[(seed % QUANTA.len() as u64) as usize],
        link: draw.pick(&links),
        bypass: draw.chance(1, 8),
        ports: draw_ports(&mut draw, descriptor),
        capacity: draw.pick(&[1_u32, 2, 4, 8, 16]),
        mono,
        hostile: !mono && draw.chance(1, 2),
        automate: (seed / QUANTA.len() as u64).is_multiple_of(2),
    };
    let bound = coverage.banks.iter().sum::<u64>();
    for &width in BankWidth::ALL {
        let backend = width.backend();
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
                coverage.resets += 1;
            }
            if draw.chance(1, if small { 5 } else { 40 }) {
                // Restored against continued: the oracle's snapshot, into the twin only. It must
                // restore and write itself back word for word, and from here the twin, restored,
                // renders exactly what the oracle renders by continuing -- mid-ramp included,
                // because a payload carries every word a continuation reads (#1278).
                let payload = snapshot_scalar(oracle.as_ref(), sizes, audited_calls);
                let input =
                    StatePayloadInput::new(&payload.common, &payload.left, &payload.right, sizes)
                        .expect("the prepared sizes");
                if let Err(error) = audited(audited_calls, "restore_state_payload", || {
                    twin.restore_state_payload(version, input)
                }) {
                    panic!(
                        "{context}: lane {lane}'s own snapshot refused ({})",
                        error.code
                    );
                }
                assert!(
                    same_payload(
                        &snapshot_scalar(twin.as_ref(), sizes, audited_calls),
                        &payload
                    ),
                    "{context}: lane {lane}'s snapshot does not survive its own restore"
                );
                coverage.restores += 1;
                coverage.continuations += 1;
            } else if shape.hostile && draw.chance(1, if small { 5 } else { 40 }) {
                // A rewritten payload into both: they accept or refuse together.
                let mut payload = snapshot_scalar(oracle.as_ref(), sizes, audited_calls);
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
                    restore_code(audited(audited_calls, "restore_state_payload", || {
                        oracle.restore_state_payload(version, input())
                    })),
                    restore_code(audited(audited_calls, "restore_state_payload", || {
                        twin.restore_state_payload(version, input())
                    })),
                );
                assert_eq!(a, b, "{context}: lane {lane}'s twins disagree on a restore");
                match a {
                    Ok(()) => coverage.restores += 1,
                    Err(_) => coverage.refused_restores += 1,
                }
            }
        }
        if spec.witness && shape.mono {
            for (lane, (oracle, twin)) in pairs.iter().enumerate() {
                assert!(
                    oracle.channel_symmetry() && twin.channel_symmetry(),
                    "{context}: lane {lane}'s witness declined on a channel-symmetric scenario"
                );
            }
            coverage.witness_holds += 1;
        }
        let frames = draw.frames(quantum);
        let silent = draw.chance(1, 6);
        let invalid = !shape.mono && draw.chance(1, 16);
        let chunk = (frames > 1 && !shape.hostile && !invalid && draw.chance(1, 2))
            .then(|| 1 + draw.below(frames - 1));
        for (lane, (oracle, twin)) in pairs.iter_mut().enumerate() {
            let profile = if terminal && lane == 0 {
                Profile::Hostile
            } else if silent {
                Profile::Silence
            } else {
                legal_profile(draw)
            };
            let other = if shape.mono || silent {
                profile
            } else {
                legal_profile(draw)
            };
            let side_profile = if silent {
                Profile::Silence
            } else {
                legal_profile(draw)
            };
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
            // Before the hostile block every word is legal, so a recovery there is a divergence
            // the zeroed block would hide (see `run_width`).
            assert!(
                terminal
                    || oracle_report.nonfinite_left_blocks + oracle_report.nonfinite_right_blocks
                        == 0,
                "{context}: lane {lane}: D7 recovered on a block whose input and state are legal \
                 ({oracle_report:?})"
            );
            if spec.witness && shape.mono {
                // The witness held and both planes carried one signal: one channel's work.
                if let Some(frame) = first_difference(&left, &right) {
                    panic!(
                        "{context}: lane {lane}'s witness held on one signal, yet its channels \
                         part at frame {frame}"
                    );
                }
            }
            if (small && chunk.is_none()) || block + 1 == spec.blocks {
                assert_eq!(
                    snapshot_scalar(oracle.as_ref(), sizes, audited_calls),
                    snapshot_scalar(twin.as_ref(), sizes, audited_calls),
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
    usize::from(width.lanes() == 8)
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
        active_mask: width.full_mask(),
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
        if shape.automate {
            draw.below(lanes)
        } else {
            lanes
        }
    });
    let hostile_block = shape.hostile.then(|| {
        let half = spec.blocks / 2;
        (half + draw.below(spec.blocks - half), draw.below(lanes))
    });
    // One planned reset per scenario, whatever the draws: a cheap effect's many seeds reach
    // resets by chance, an expensive effect's few seeds need not.
    let reset_at = draw.below(spec.blocks);
    // A mono scenario stays channel-symmetric until a restore writes the channels apart.
    let mut symmetric = shape.mono;
    // Restored against continued: a lane's snapshot restored into a fresh instance, rendered
    // beside the lane from then on. The plan-swap carry (#1269) is exactly this move.
    let mut continuation: Option<Continuation> = None;
    // Per lane, the sample before which a ramp its automation started may still be in flight.
    let mut in_flight_until = vec![0_u64; lanes];
    // Whether a continuation has been taken while a ramp may be in flight: the first boundary
    // that offers one always takes it, so a scenario whose automation leaves a ramp in flight at
    // a boundary reaches a mid-ramp continuation without a further draw.
    let mut taken_in_flight = false;
    // Prepared targets (#1278 attempt 3): an effect that refuses raw spans and takes its
    // automation as prepared targets (the EQ) gets ramps only this way. Per lane, the candidate
    // the lane was last given, and the target words it is known to be heading for (initially its
    // prepared configuration; forgotten when a restore replaces its state).
    let preparation = factory.target_preparation();
    let mut targets = Targets::new(preparation, shape, &values);
    for block in 0..spec.blocks {
        let context = format!("{width:?} block {block} at sample {first} ({shape:?})");
        // --- Block-boundary operations -------------------------------------------------------
        if block == reset_at || draw.chance(1, 24) {
            coverage.resets += 1;
            let kind = draw.pick(&[
                ResetKind::FullToDefaults,
                ResetKind::DiscontinuityKeepParameters,
            ]);
            if let Some(arm) = arm.as_mut() {
                disengage(arm, bank.as_ref(), sizes, audited_calls, &context, coverage);
            }
            for scalar in &mut scalars {
                scalar.reset(kind);
            }
            if let Some(continuation) = continuation.as_mut() {
                continuation.twin.reset(kind);
            }
            // A reset ends every ramp (a discontinuity snaps them, a full reset restarts from the
            // prepared configuration), so nothing is in flight until automation moves again.
            in_flight_until.fill(0);
            targets.reset(kind, &values);
            bank.reset(kind);
            if let Some(arm) = arm.as_mut() {
                arm.bank.reset(kind);
            }
        }
        let terminal = hostile_block.is_some_and(|(at, _)| at == block);
        if draw.chance(1, 6) || (terminal && draw.chance(1, 2)) {
            let restored = restore(
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
                audited_calls,
                &context,
                coverage,
            );
            symmetric &= !restored.asymmetric;
            if restored.replaced {
                // Another state now: whatever was in flight on the lane is not known to be.
                in_flight_until[restored.lane] = 0;
                targets.forget(restored.lane);
            }
            // The lane took another payload; the continued instance is no longer its twin.
            if continuation
                .as_ref()
                .is_some_and(|continuation| continuation.lane == restored.lane)
            {
                continuation = None;
            }
        }
        if continuation.is_some() && draw.chance(1, 8) {
            continuation = None;
        }
        // Preferentially while a ramp may be in flight (#1278 attempt 2): a restore that
        // re-derives a word the continuation reads diverges only mid-ramp, and a uniform draw
        // reached that rarely on an effect whose automation the scenario narrows to one lane.
        let flying: Vec<usize> = (0..lanes)
            .filter(|&lane| first < in_flight_until[lane])
            .collect();
        let take = if !flying.is_empty() && (!taken_in_flight || draw.chance(1, 2)) {
            taken_in_flight = true;
            Some(draw.pick(&flying))
        } else if continuation.is_none() && draw.chance(1, 3) {
            Some(draw.below(lanes))
        } else {
            None
        };
        if let Some(lane) = take {
            continuation = Some(Continuation::take(
                factory,
                requests[lane],
                scalars[lane].as_ref(),
                lane,
                sizes,
                version,
                audited_calls,
                &context,
            ));
            coverage.continuations += 1;
            let in_flight = spec
                .in_flight
                .map_or(first < in_flight_until[lane], |decode| {
                    decode(&snapshot_scalar(scalars[lane].as_ref(), sizes, false))
                });
            if in_flight {
                coverage.continuations_in_flight += 1;
                if shape.quantum == 32 {
                    coverage.continuations_in_flight_at_32 += 1;
                }
            }
        }
        // Prepared targets, after the take, so a continuation taken at a later boundary holds the
        // ramp mid-flight. One lane per boundary, applied to the scalar instance, its bank lane,
        // the mono arm's lane and the lane's continued twin at once: a host applies one target to
        // whichever instance renders the lane.
        if let Some(preparation) = preparation
            && draw.chance(1, 3)
        {
            let lane = automated.unwrap_or_else(|| draw.below(lanes));
            if lane < lanes {
                if let Some(arm) = arm.as_mut() {
                    disengage(arm, bank.as_ref(), sizes, audited_calls, &context, coverage);
                }
                let twin = continuation
                    .as_mut()
                    .filter(|continuation| continuation.lane == lane)
                    .map(|continuation| continuation.twin.as_mut());
                if targets.apply(
                    preparation,
                    draw,
                    descriptor,
                    shape,
                    lane,
                    Receivers {
                        scalar: scalars[lane].as_mut(),
                        bank: bank.as_mut(),
                        arm: arm.as_mut().map(|arm| arm.bank.as_mut()),
                        twin,
                    },
                    audited_calls,
                    &context,
                    coverage,
                ) {
                    in_flight_until[lane] = in_flight_until[lane].max(first + IN_FLIGHT_SAMPLES);
                }
            }
        }

        if spec.witness && symmetric {
            for (lane, scalar) in scalars.iter().enumerate() {
                assert!(
                    scalar.channel_symmetry() && bank.lane_channel_symmetry(lane),
                    "{context}: lane {lane}'s witness declined on a channel-symmetric scenario \
                     (scalar {}, bank {})",
                    scalar.channel_symmetry(),
                    bank.lane_channel_symmetry(lane)
                );
            }
            coverage.witness_holds += 1;
        }

        // --- Input -----------------------------------------------------------------------------
        let frames = draw.frames(quantum);
        let words = frames * lanes;
        let mut input_left = vec![0.0_f32; words];
        let mut input_right = vec![0.0_f32; words];
        let mut side_left = vec![0.0_f32; words];
        let mut side_right = vec![0.0_f32; words];
        // A sixth of the blocks are `+0.0` on every lane: the whole-bank silent fast paths
        // (#163 phase 4) engage only then, and their claim is earned block by block.
        let silent = draw.chance(1, 6);
        for lane in 0..lanes {
            let profile = if hostile_block == Some((block, lane)) {
                Profile::Hostile
            } else if silent {
                Profile::Silence
            } else {
                legal_profile(draw)
            };
            let other = if shape.mono || silent || draw.chance(2, 3) {
                profile
            } else {
                legal_profile(draw)
            };
            let side = if silent {
                Profile::Silence
            } else {
                legal_profile(draw)
            };
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
        // A fifth of the eligible blocks render dual anyway, as a chain does whenever anything
        // else asks it to: the disengage copy must then be the never-collapsed state, and the
        // channels still agree afterwards, so the collapse may engage again.
        let forced_dual = draw.chance(1, 5);
        let collapse = arm.as_mut().is_some_and(|arm| {
            let designed = (0..lanes).all(|lane| bank.lane_channel_symmetry(lane));
            let holds = arm.restored && designed;
            let eligible = arm.agree && holds && !forced_dual;
            if !eligible {
                disengage(arm, bank.as_ref(), sizes, audited_calls, &context, coverage);
                arm.agree &= holds;
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
            let continued = continuation
                .as_mut()
                .filter(|continuation| continuation.lane == lane)
                .map(|continuation| {
                    let (mut twin_left, mut twin_right) = (left.clone(), right.clone());
                    let report = render_scalar(
                        continuation.twin.as_mut(),
                        (&mut twin_left, &mut twin_right),
                        connected.then_some((&side_l, &side_r)),
                        first,
                        &lane_spans[lane],
                        shape.quantum,
                        chunk,
                        audited_calls,
                    );
                    (twin_left, twin_right, report)
                });
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
            if let Some((twin_left, twin_right, twin_report)) = continued {
                for (plane, continued, restored) in
                    [("left", &left, &twin_left), ("right", &right, &twin_right)]
                {
                    if let Some(frame) = first_difference(continued, restored) {
                        panic!(
                            "{context}: lane {lane} {plane} frame {frame}: the instance restored \
                             from its snapshot rendered {:#010x} where the lane, continuing, \
                             rendered {:#010x} (chunk {chunk:?}, spans {:?})",
                            restored[frame].to_bits(),
                            continued[frame].to_bits(),
                            lane_spans[lane],
                        );
                    }
                }
                assert_eq!(
                    twin_report, report,
                    "{context}: lane {lane}'s restored instance reports other than the lane"
                );
                coverage.continued_blocks += 1;
            }
            for frame in 0..frames {
                scalar_left[frame * lanes + lane] = left[frame];
                scalar_right[frame * lanes + lane] = right[frame];
            }
            scalar_reports.push(report);
        }
        if chunk.is_some() {
            coverage.chunked_blocks += 1;
        }
        // Only spans the lane accepted can have started a ramp: a lane whose report counts an
        // invalid span (the EQ counts every raw span) credits nothing for this block.
        for (lane, until) in in_flight_until.iter_mut().enumerate() {
            if scalar_reports[lane].invalid_spans == 0 {
                for span in &lane_spans[lane] {
                    *until = (*until).max(span.end_sample.saturating_add(IN_FLIGHT_SAMPLES));
                }
            }
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
        // Every word before the hostile block is finite and legal -- input, sidechain, spans,
        // restored state -- so a D7 recovery there is an effect that diverged on legal input, and
        // the zeroed block would pass the bound check vacuously: red, not a quiet end.
        let hostile_now = hostile_block.is_some_and(|(at, _)| at == block);
        assert!(
            hostile_now || !recovered(&bank_report, &scalar_reports, lanes),
            "{context}: D7 recovered on a block whose input and state are legal (bank {:?}, \
             scalar {:?})",
            report_lanes(&bank_report, lanes),
            scalar_reports,
        );
        if hostile_now {
            // D7 (docs/EFFECT_CONTRACT_V1.md): output finiteness is checked once per block *per
            // bank*. Every launch effect recovers the failing lane alone (#1089-#1092) except the
            // multiband, whose bank still zeroes and resets every lane. So on the block the
            // hostile words arrive in, a bank and its scalar instances may part by design, and the
            // allowance is the same for every effect: both must keep every word inside the bound,
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
            compare_state(
                &scalars,
                bank.as_ref(),
                sizes,
                audited_calls,
                &context,
                coverage,
            );
        }
        first += frames as u64;
    }
}

/// A fresh instance holding one lane's restored snapshot: restored against continued.
struct Continuation {
    /// The lane the snapshot was taken from.
    lane: usize,
    /// A fresh instance, prepared from the lane's request, holding the restored snapshot.
    twin: Box<dyn PreparedNativeEffect>,
}

impl Continuation {
    /// Prepares a fresh instance from `request` (control plane, unaudited), then snapshots `lane`
    /// and restores the payload into it inside render scopes. The lane's own snapshot must
    /// restore.
    #[allow(clippy::too_many_arguments)]
    fn take(
        factory: &dyn NativeEffectFactory,
        request: PrepareEffectRequest<'_>,
        source: &dyn PreparedNativeEffect,
        lane: usize,
        sizes: StatePayloadSizes,
        version: u32,
        armed: bool,
        context: &str,
    ) -> Self {
        let mut twin = factory
            .prepare(request)
            .expect("a lane's own request prepares again");
        let payload = snapshot_scalar(source, sizes, armed);
        let input = StatePayloadInput::new(&payload.common, &payload.left, &payload.right, sizes)
            .expect("the prepared sizes");
        if let Err(error) = audited(armed, "restore_state_payload", || {
            twin.restore_state_payload(version, input)
        }) {
            panic!(
                "{context}: lane {lane}'s snapshot refused by a fresh instance ({})",
                error.code
            );
        }
        // No word-for-word write-back here: a fresh instance's ring cursors need not be the
        // lane's, and a restore rotates a ring into the receiver's frame (the limiter does), so
        // the twin's own snapshot may legitimately differ. What it renders may not.
        Self { lane, twin }
    }
}

/// The instances one lane's prepared target is applied to at once.
struct Receivers<'a> {
    /// The lane's scalar instance.
    scalar: &'a mut (dyn PreparedNativeEffect + 'static),
    /// The dual bank, at the lane.
    bank: &'a mut (dyn PreparedNativeEffectBank + 'static),
    /// The mono arm's bank, at the lane, disengaged.
    arm: Option<&'a mut (dyn PreparedNativeEffectBank + 'static)>,
    /// The lane's continued twin, if one renders beside it.
    twin: Option<&'a mut (dyn PreparedNativeEffect + 'static)>,
}

/// Target words a lane is known to be heading for, per effect-owned slot and channel.
type Heading = Vec<(u32, ParameterChannel, [u32; PREPARED_EFFECT_TARGET_WORDS])>;

/// Prepared-target automation (#1278 attempt 3): the route an effect that refuses raw spans (the
/// parametric EQ, #807) takes its automation by, and so the only way the restored-against-continued
/// oracle can reach such an effect mid-ramp.
struct Targets {
    /// Whether the effect under test takes prepared targets at all.
    enabled: bool,
    sample_rate: u32,
    /// Control-plane target storage, `maximum_targets` long.
    out: Vec<PreparedEffectTarget>,
    /// Per lane, the complete candidate the lane was last given.
    candidates: Vec<Vec<InitialParameterValue>>,
    /// Per lane, the prepared configuration's target words.
    initial: Vec<Heading>,
    /// Per lane, the target words the lane is known to be heading for; an absent slot is unknown.
    heading: Vec<Heading>,
}

impl Targets {
    fn new(
        preparation: Option<&dyn NativeEffectTargetPreparation>,
        shape: Shape,
        values: &[Vec<InitialParameterValue>],
    ) -> Self {
        let Some(preparation) = preparation else {
            return Self {
                enabled: false,
                sample_rate: 0,
                out: Vec::new(),
                candidates: Vec::new(),
                initial: Vec::new(),
                heading: Vec::new(),
            };
        };
        let sample_rate = shape.quality.sample_rate;
        let mut out = vec![
            PreparedEffectTarget {
                slot: 0,
                channel: ParameterChannel::Left,
                words: [0; PREPARED_EFFECT_TARGET_WORDS],
            };
            preparation.maximum_targets()
        ];
        let initial: Vec<Heading> = values
            .iter()
            .map(|lane| {
                let changed = vec![true; lane.len()];
                let count = preparation
                    .prepare_targets(
                        EffectTargetRequest {
                            sample_rate,
                            values: lane,
                            changed: &changed,
                        },
                        &mut out,
                    )
                    .expect("a configuration that prepared also prepares as targets");
                let mut heading = Heading::new();
                for target in &out[..count] {
                    record(&mut heading, target);
                }
                heading
            })
            .collect();
        Self {
            enabled: true,
            sample_rate,
            out,
            candidates: values.to_vec(),
            heading: initial.clone(),
            initial,
        }
    }

    /// A full reset returns every lane to its prepared configuration; a discontinuity snaps every
    /// ramp to where it was heading, so the heading stands.
    fn reset(&mut self, kind: ResetKind, values: &[Vec<InitialParameterValue>]) {
        if self.enabled && kind == ResetKind::FullToDefaults {
            self.candidates = values.to_vec();
            self.heading.clone_from(&self.initial);
        }
    }

    /// A restore replaced `lane`'s state: where it is heading is no longer known.
    fn forget(&mut self, lane: usize) {
        if let Some(heading) = self.heading.get_mut(lane) {
            heading.clear();
        }
    }

    /// Draws a candidate that moves one to three of `lane`'s continuous values (both channels of
    /// a per-lane value on a mono scenario), prepares it off the audited scope, and applies every
    /// target to every receiver inside one. They must accept or refuse together. Returns whether
    /// an accepted target's words differ from the words the lane was known to be heading for --
    /// a ramp that moves (unless it lands exactly on the lane's current words).
    #[allow(clippy::too_many_arguments)]
    fn apply(
        &mut self,
        preparation: &dyn NativeEffectTargetPreparation,
        draw: &mut Draw,
        descriptor: &'static EffectDescriptor,
        shape: Shape,
        lane: usize,
        mut receivers: Receivers<'_>,
        armed: bool,
        context: &str,
        coverage: &mut DifferentialCoverage,
    ) -> bool {
        let mut candidate = self.candidates[lane].clone();
        let parameter_of =
            |entry: &InitialParameterValue| &descriptor.parameters[entry.parameter_index as usize];
        let continuous: Vec<usize> = (0..candidate.len())
            .filter(|&index| parameter_of(&candidate[index]).domain == ParameterDomain::Continuous)
            .collect();
        if continuous.is_empty() {
            return false;
        }
        let mut changed = vec![false; candidate.len()];
        for _ in 0..=draw.below(3) {
            let index = draw.pick(&continuous);
            let parameter = parameter_of(&candidate[index]);
            let value = draw_value(draw, parameter);
            candidate[index].value = value;
            changed[index] = true;
            if shape.mono && parameter.channel_policy == ParameterChannelPolicy::PerLane {
                let parameter_index = candidate[index].parameter_index;
                for (other, entry) in candidate.iter_mut().enumerate() {
                    if entry.parameter_index == parameter_index {
                        entry.value = value;
                        changed[other] = true;
                    }
                }
            }
        }
        let count = preparation
            .prepare_targets(
                EffectTargetRequest {
                    sample_rate: self.sample_rate,
                    values: &candidate,
                    changed: &changed,
                },
                &mut self.out,
            )
            .unwrap_or_else(|error| {
                panic!("{context}: lane {lane}'s legal candidate did not prepare ({error:?})")
            });
        let mut moving = false;
        let mut accepted = false;
        for target in &self.out[..count] {
            let scalar = audited(armed, "apply_prepared_target", || {
                receivers.scalar.apply_prepared_target(target)
            });
            let banked = audited(armed, "apply_prepared_target_lane", || {
                receivers.bank.apply_prepared_target_lane(lane, target)
            });
            assert_eq!(
                banked, scalar,
                "{context}: lane {lane}'s bank and scalar disagree on a prepared target"
            );
            if let Some(arm) = receivers.arm.as_deref_mut() {
                let twin = audited(armed, "apply_prepared_target_lane", || {
                    arm.apply_prepared_target_lane(lane, target)
                });
                assert_eq!(
                    twin, scalar,
                    "{context}: lane {lane}'s twin bank disagrees on a prepared target"
                );
            }
            if let Some(twin) = receivers.twin.as_deref_mut() {
                let continued = audited(armed, "apply_prepared_target", || {
                    twin.apply_prepared_target(target)
                });
                assert_eq!(
                    continued, scalar,
                    "{context}: lane {lane}'s restored instance disagrees on a prepared target"
                );
            }
            if scalar.is_ok() {
                accepted = true;
                coverage.applied_targets += 1;
                if record(&mut self.heading[lane], target) {
                    moving = true;
                    coverage.moving_targets += 1;
                }
            }
        }
        if accepted {
            self.candidates[lane] = candidate;
        }
        moving
    }
}

/// Records `target` as where its slot and channels are heading; whether a known heading moved.
fn record(heading: &mut Heading, target: &PreparedEffectTarget) -> bool {
    let channels: &[ParameterChannel] = match target.channel {
        ParameterChannel::Left => &[ParameterChannel::Left],
        ParameterChannel::Right => &[ParameterChannel::Right],
        ParameterChannel::Both => &[ParameterChannel::Left, ParameterChannel::Right],
    };
    let mut moved = false;
    for &channel in channels {
        if let Some(entry) = heading
            .iter_mut()
            .find(|entry| entry.0 == target.slot && entry.1 == channel)
        {
            moved |= entry.2 != target.words;
            entry.2 = target.words;
        } else {
            heading.push((target.slot, channel, target.words));
        }
    }
    moved
}

/// Ends a collapsed run: `desymmetrize_channels`, then every lane's state must be the dual bank's.
fn disengage(
    arm: &mut MonoArm,
    bank: &dyn PreparedNativeEffectBank,
    sizes: StatePayloadSizes,
    armed: bool,
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
            snapshot_lane(arm.bank.as_ref(), lane, sizes, armed),
            snapshot_lane(bank, lane, sizes, armed),
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

fn report_lanes(report: &BankProcessReport, lanes: usize) -> &[ProcessReport] {
    &report.reports[..lanes]
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
        let pieces = chunk.map(|_| {
            spans
                .iter()
                .filter(|span| span.start_sample >= from && span.start_sample < to)
                .copied()
                .collect::<Vec<_>>()
        });
        let block = EffectProcessBlock::new(
            &mut left[start..end],
            &mut right[start..end],
            side.map(|(l, r)| (&l[start..end], &r[start..end])),
            from,
            pieces.as_deref().unwrap_or(spans),
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

/// Snapshots a scalar instance into buffers allocated here, outside the audited call: the payload
/// call itself runs inside a render scope, as the plan-swap carry runs it in the swap block.
fn snapshot_scalar(
    effect: &dyn PreparedNativeEffect,
    sizes: StatePayloadSizes,
    armed: bool,
) -> Payload {
    let mut payload = empty_payload(sizes);
    let output = StatePayloadOutput::new(
        &mut payload.common,
        &mut payload.left,
        &mut payload.right,
        sizes,
    )
    .expect("the prepared sizes");
    audited(armed, "snapshot_state_payload", || {
        effect.snapshot_state_payload(output)
    })
    .expect("a prepared instance snapshots");
    payload
}

/// As [`snapshot_scalar`], for one bank lane.
fn snapshot_lane(
    bank: &dyn PreparedNativeEffectBank,
    lane: usize,
    sizes: StatePayloadSizes,
    armed: bool,
) -> Payload {
    let mut payload = empty_payload(sizes);
    let output = StatePayloadOutput::new(
        &mut payload.common,
        &mut payload.left,
        &mut payload.right,
        sizes,
    )
    .expect("the prepared sizes");
    audited(armed, "snapshot_track_state_payload", || {
        bank.snapshot_track_state_payload(lane as u32, output)
    })
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
    armed: bool,
    context: &str,
    coverage: &mut DifferentialCoverage,
) {
    for (lane, scalar) in scalars.iter().enumerate() {
        let (expected, banked) = (
            snapshot_scalar(scalar.as_ref(), sizes, armed),
            snapshot_lane(bank, lane, sizes, armed),
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
    armed: bool,
    context: &str,
    coverage: &mut DifferentialCoverage,
) -> Restored {
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
    let mut payload = snapshot_scalar(scalars[source].as_ref(), sizes, armed);
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
        disengage(arm, bank, sizes, armed, context, coverage);
    }
    let input = || {
        StatePayloadInput::new(&payload.common, &payload.left, &payload.right, sizes)
            .expect("the prepared sizes")
    };
    let scalar = restore_code(audited(armed, "restore_state_payload", || {
        scalars[lane].restore_state_payload(version, input())
    }));
    let banked = restore_code(audited(armed, "restore_track_state_payload", || {
        bank.restore_track_state_payload(lane as u32, version, input())
    }));
    assert_eq!(
        banked, scalar,
        "{context}: lane {lane}'s bank and scalar restores disagree"
    );
    if let Some(arm) = arm {
        let twin = restore_code(audited(armed, "restore_track_state_payload", || {
            arm.bank
                .restore_track_state_payload(lane as u32, version, input())
        }));
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
    if !rewritten && (scalar.is_ok() || source == lane) {
        // An untouched snapshot restores and writes itself back word for word, from the scalar
        // instance and from the bank lane. The lane's own always restores; another lane's may be
        // refused where a payload is bound to its lane's configuration (the EQ's is).
        assert_eq!(
            scalar,
            Ok(()),
            "{context}: lane {lane} refused its own snapshot"
        );
        assert!(
            same_payload(
                &snapshot_scalar(scalars[lane].as_ref(), sizes, armed),
                &payload
            ),
            "{context}: lane {lane}'s snapshot does not survive its own restore"
        );
        assert!(
            same_payload(&snapshot_lane(bank, lane, sizes, armed), &payload),
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
    Restored {
        lane,
        asymmetric: scalar.is_ok() && payload.left != payload.right,
        replaced: scalar.is_ok() && (rewritten || source != lane),
    }
}

/// What one restore event did.
struct Restored {
    /// The lane it restored into.
    lane: usize,
    /// It was accepted with left and right sections that differ.
    asymmetric: bool,
    /// It was accepted with a payload other than the lane's own untouched snapshot.
    replaced: bool,
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

    // A malformed shape: the wrong member count, or a backend of another width. (`Backend::Scalar`
    // exists only under `lane/test-support` since #1059, so a shipped build cannot form that
    // request; the probe does not name it.)
    let short = &base[..lanes - 1];
    let mut long = base.clone();
    long.push(base[0]);
    // Absent where the build has one bank width, as every 4-lane build does (#1110, #1112).
    let other_backend = Backend::VECTOR
        .iter()
        .copied()
        .find(|&other| other != backend);
    for (label, requests, backend) in [
        ("one member short", short, Some(backend)),
        ("one member long", &long[..], Some(backend)),
        ("another width's backend", &base[..], other_backend),
    ] {
        let Some(backend) = backend else { continue };
        match factory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests,
            active_mask: width.full_mask(),
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
        active_mask: width.full_mask(),
    });
    match (prepared, bound) {
        (Err(member_error), Err(bank_error)) => assert_eq!(
            bank_error.code, member_error.code,
            "{width:?}: bind refused an illegal member with another code than prepare"
        ),
        // #1070's allowance: the decline at a width this build does not execute, which answers
        // before any member is validated, and only for the illegal initial value this probe plants.
        (Err(member_error), Ok(None))
            if known.contains(&Known::BindDeclinesBeforeValidating)
                && width.lanes() as usize != Backend::current().width()
                && member_error.code == "effect.parameter.initial" => {}
        (Err(member_error), Ok(bank)) => panic!(
            "{width:?}: bind {} a cohort with a member prepare refuses ({}), where the \
             three-outcome rule refuses it first",
            if bank.is_some() { "banked" } else { "declined" },
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
        !coverage.witness || coverage.witness_holds > 0,
        "the witness was never required on a mono scenario: {coverage:?}"
    );
    assert!(
        coverage.blocks > 0 && coverage.chunked_blocks > 0 && coverage.bounded_blocks > 0,
        "blocks were not compared whole, chunked and bounded: {coverage:?}"
    );
    assert!(
        coverage.restores > 0 && coverage.resets > 0,
        "nothing was restored or reset: {coverage:?}"
    );
    assert!(
        coverage.continuations > 0
            && (coverage.banks.iter().sum::<u64>() == 0
                || (coverage.continued_blocks > 0
                    && coverage.continuations_in_flight > 0
                    && coverage.continuations_in_flight_at_32 > 0)),
        "no restored instance rendered beside its continued lane with a ramp in flight, at \
         quantum 32 included: {coverage:?}"
    );
    assert!(
        !coverage.prepared_targets
            || coverage.banks.iter().sum::<u64>() == 0
            || coverage.moving_targets > 0,
        "an effect that takes prepared targets was never given one that moves: {coverage:?}"
    );
    assert!(
        coverage.malformed_refusals > 0 && coverage.heterogeneous_declines > 0,
        "the bind-eligibility probes were not answered: {coverage:?}"
    );
    assert!(
        !coverage.mono_capable || (coverage.collapsed_blocks > 0 && coverage.disengages > 0),
        "a bank that supports the collapse never collapsed and disengaged: {coverage:?}"
    );
}

/// The D7 recovery's report against the contract (#1073), on fixed input: no seed.
///
/// D7 (`docs/EFFECT_CONTRACT_V1.md`) checks output finiteness once per block, zeroes a failing
/// block, resets state, and increments a **block** counter: "the contract's report counts blocks,
/// never samples". A scalar instance, and a bank at every width this build binds, render four
/// blocks of a clean signal at the effect's defaults, unbypassed; on the second block the left
/// plane of the last lane is NaN throughout. Returns every place the reports break the rule:
///
/// * a lane counts more than one block on one channel for one block (samples, not blocks);
/// * a lane whose input was clean counts a block;
/// * the failing lane's output is zeroed on the poisoned block, and that block counts nothing.
#[must_use]
pub fn d7_report_violations(factory: &dyn NativeEffectFactory) -> Vec<String> {
    const QUANTUM: u32 = 64;
    const BLOCKS: usize = 4;
    const POISONED: usize = 1;
    let descriptor = factory.descriptor();
    let link = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average]
        .into_iter()
        .find(|link| descriptor.supported_link_modes.contains(*link))
        .expect("an effect supports a link mode");
    let sidechain = descriptor
        .ports
        .iter()
        .find(|port| port.role == PortRole::SidechainInput)
        .map_or(PreparedSidechainPort::None, |port| {
            if port.required {
                PreparedSidechainPort::Connected {
                    id: port.id,
                    required: true,
                }
            } else {
                PreparedSidechainPort::Unconnected {
                    id: port.id,
                    required: false,
                }
            }
        });
    let connected = matches!(sidechain, PreparedSidechainPort::Connected { .. });
    let shape = Shape {
        quality: descriptor.qualities[0],
        quantum: QUANTUM,
        link,
        bypass: false,
        ports: PreparedPorts { sidechain },
        capacity: 1,
        mono: false,
        hostile: false,
        automate: false,
    };
    let values: Vec<InitialParameterValue> = default_initial_values(descriptor).collect();
    let frames = QUANTUM as usize;
    // A clean, lane- and channel-distinct signal well inside the D7 bound, and never zero.
    let signal = |block: usize, lane: usize, channel: usize, frame: usize| -> f32 {
        let step = ((block * frames + frame) * 37 + lane * 11 + channel * 5) % 97;
        (step as f32 - 48.25) / 194.0
    };
    let mut violations = Vec::new();
    let mut check = |who: &str,
                     block: usize,
                     lane: usize,
                     poisoned: bool,
                     report: ProcessReport,
                     zeroed: bool| {
        let counts = [report.nonfinite_left_blocks, report.nonfinite_right_blocks];
        if counts.iter().any(|count| *count > 1) {
            violations.push(format!(
                "{who} block {block} lane {lane}: counts {counts:?} for one block (samples, not \
                 blocks)"
            ));
        }
        if !poisoned && counts.iter().any(|count| *count > 0) {
            violations.push(format!(
                "{who} block {block} lane {lane}: counts {counts:?} though its input was clean"
            ));
        }
        // Only on the poisoned block itself: after a reset an effect with latency renders its
        // cleared delay line, which is zero without any recovery.
        if poisoned && block == POISONED && zeroed && counts == [0, 0] {
            violations.push(format!(
                "{who} block {block} lane {lane}: its output was zeroed and the report counts \
                 nothing"
            ));
        }
    };

    // The scalar instance: one lane, the poisoned one.
    let mut scalar = factory
        .prepare(request(shape, &values))
        .expect("the defaults prepare");
    for block in 0..BLOCKS {
        let first = (block * frames) as u64;
        let mut left: Vec<f32> = (0..frames).map(|f| signal(block, 0, 0, f)).collect();
        let mut right: Vec<f32> = (0..frames).map(|f| signal(block, 0, 1, f)).collect();
        if block == POISONED {
            left.fill(f32::NAN);
        }
        let side: Vec<f32> = (0..frames).map(|f| signal(block, 0, 2, f)).collect();
        let report = render_scalar(
            scalar.as_mut(),
            (&mut left, &mut right),
            connected.then_some((side.as_slice(), side.as_slice())),
            first,
            &[],
            QUANTUM,
            None,
            false,
        );
        let zeroed = left.iter().all(|word| word.to_bits() == 0);
        check("scalar", block, 0, block >= POISONED, report, zeroed);
    }

    for &width in BankWidth::ALL {
        let backend = width.backend();
        let lanes = width.lanes() as usize;
        let requests: Vec<PrepareEffectRequest<'_>> =
            (0..lanes).map(|_| request(shape, &values)).collect();
        let Ok(Some(mut bank)) = factory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        }) else {
            continue;
        };
        let who = format!("{width:?} bank");
        let failing = lanes - 1;
        let offsets = vec![0_u32; lanes + 1];
        for block in 0..BLOCKS {
            let first = (block * frames) as u64;
            let plane = |channel: usize| -> Vec<f32> {
                (0..frames * lanes)
                    .map(|word| signal(block, word % lanes, channel, word / lanes))
                    .collect()
            };
            let (mut left, mut right) = (plane(0), plane(1));
            let side = plane(2);
            if block == POISONED {
                for frame in 0..frames {
                    left[frame * lanes + failing] = f32::NAN;
                }
            }
            let report = bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut left,
                    &mut right,
                    connected.then_some((side.as_slice(), side.as_slice())),
                    QUANTUM,
                    width,
                    first,
                    &[],
                    &offsets,
                    QUANTUM,
                )
                .expect("a well-shaped bank block"),
            );
            for lane in 0..lanes {
                let zeroed = (0..frames).all(|frame| left[frame * lanes + lane].to_bits() == 0);
                let poisoned = lane == failing && block >= POISONED;
                check(&who, block, lane, poisoned, report.reports[lane], zeroed);
            }
        }
    }
    violations
}

/// Panics with every [`d7_report_violations`] of `factory`.
///
/// # Panics
///
/// When the D7 recovery's report breaks the contract anywhere.
pub fn assert_d7_reports(factory: &dyn NativeEffectFactory) {
    let violations = d7_report_violations(factory);
    assert!(
        violations.is_empty(),
        "{}: the D7 recovery's report breaks the contract (#1073):\n{}",
        factory.descriptor().display_name,
        violations.join("\n")
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
     known: $known:expr, banks_natively: $banks:expr, witness: $witness:expr $(,)?) => {
        $crate::randomized_effect_test!(
            $name, $factory, seeds: $seeds, blocks: $blocks, craft: $craft, known: $known,
            banks_natively: $banks, witness: $witness, in_flight: None,
        );
    };
    ($name:ident, $factory:expr, seeds: $seeds:expr, blocks: $blocks:expr, craft: $craft:expr,
     known: $known:expr, banks_natively: $banks:expr, witness: $witness:expr,
     in_flight: $in_flight:expr $(,)?) => {
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
                witness: $witness,
                in_flight: $in_flight,
            });
            println!("{coverage:#?}");
            $crate::assert_reached(&coverage);
        }
    };
}

impl EffectDifferential<'_> {
    /// Every refusal of the effect's **own** snapshot, taken at each sample of a ramp that
    /// rounds past its parameter's domain edge (#1278): no seed.
    ///
    /// The plan-swap carry restores every lane it carries, so a restore must accept every state
    /// the effect itself reaches. A smoothed ramp iterates `current + step` with a step rounded
    /// once, so a ramp that ends on a domain edge can round a few ulps past the edge on its way.
    /// For each smoothed continuous parameter, each edge and each quality row, this finds a start
    /// value a few hundred ulps inside the edge whose `f32` walk leaves the domain, prepares a
    /// scalar instance there on both channels, sends a `Point` to the edge, and renders every
    /// sample of the ramp. After each sample in a fixed position set it restores the instance's
    /// snapshot into a freshly prepared twin (#1301): for a ramp of `n` samples on quality row
    /// `r`, with `k` the first step of the probe's own `f32` walk that leaves the domain, the set
    /// is `{0, n - 2, n - 1} U {k - 2, k - 1, k} U {s : s mod 16 == 4r mod 16}`. Sample 0 reaches
    /// a refusal of the whole path, `k - 2 ..= k` the first sample past the edge with one sample
    /// of slack each way, `n - 2 ..= n - 1` the last moving sample and the snap, and the rotating
    /// stride positions the model does not predict. When
    /// [`dsp_reference::randomized::overridden`] is true (the nightly's scaled run, or a seed
    /// replay), it restores after every sample instead. It returns each refusal. (An associated
    /// function of this exported type, so an effect crate's tests reach it.)
    #[must_use]
    pub fn edge_ramp_restore_violations(factory: &dyn NativeEffectFactory) -> Vec<String> {
        edge_ramp_restore_violations(factory)
    }

    /// Panics with every [`Self::edge_ramp_restore_violations`] of `factory`.
    ///
    /// # Panics
    ///
    /// When the effect refuses a snapshot of its own state.
    pub fn assert_edge_ramps_restore(factory: &dyn NativeEffectFactory) {
        assert_edge_ramps_restore(factory);
    }
}

// The body of `EffectDifferential::edge_ramp_restore_violations`.
fn edge_ramp_restore_violations(factory: &dyn NativeEffectFactory) -> Vec<String> {
    let descriptor = factory.descriptor();
    let link = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average]
        .into_iter()
        .find(|link| descriptor.supported_link_modes.contains(*link))
        .expect("an effect supports a link mode");
    let sidechain = descriptor
        .ports
        .iter()
        .find(|port| port.role == PortRole::SidechainInput)
        .map_or(PreparedSidechainPort::None, |port| {
            if port.required {
                PreparedSidechainPort::Connected {
                    id: port.id,
                    required: true,
                }
            } else {
                PreparedSidechainPort::Unconnected {
                    id: port.id,
                    required: false,
                }
            }
        });
    let connected = matches!(sidechain, PreparedSidechainPort::Connected { .. });
    let every_sample = dsp_reference::randomized::overridden();
    let mut violations = Vec::new();
    for (row, quality) in descriptor.qualities.iter().enumerate() {
        let shape = Shape {
            quality: *quality,
            quantum: 64,
            link,
            bypass: false,
            ports: PreparedPorts { sidechain },
            capacity: 2,
            mono: false,
            hostile: false,
            automate: false,
        };
        let sizes = quality.maximum_state;
        for (index, parameter) in descriptor.parameters.iter().enumerate() {
            let (Some(low), Some(high)) = (parameter.minimum, parameter.maximum) else {
                continue;
            };
            if parameter.domain != ParameterDomain::Continuous
                || parameter.automation_rate == AutomationRate::None
                || parameter.smoothing_samples < 2
                || low >= high
            {
                continue;
            }
            let samples = parameter.smoothing_samples;
            for edge in [low, high] {
                let Some((start, first_out)) = overshooting_start(edge, low, high, samples) else {
                    continue;
                };
                let positions = edge_restore_positions(samples, first_out, row, every_sample);
                let mut values: Vec<InitialParameterValue> =
                    default_initial_values(descriptor).collect();
                for value in &mut values {
                    if value.parameter_index as usize == index {
                        value.value = start;
                    }
                }
                let Ok(mut effect) = factory.prepare(request(shape, &values)) else {
                    continue;
                };
                let channels: &[ParameterChannel] = match parameter.channel_policy {
                    ParameterChannelPolicy::Shared => &[ParameterChannel::Both],
                    ParameterChannelPolicy::PerLane => {
                        &[ParameterChannel::Left, ParameterChannel::Right]
                    }
                };
                let spans: Vec<PreparedAutomationSpan> = channels
                    .iter()
                    .map(|channel| PreparedAutomationSpan {
                        kind: AutomationSpanKind::Point,
                        channel: *channel,
                        parameter_index: index as u32,
                        start_sample: 0,
                        end_sample: 0,
                        start_value: edge,
                        end_value: edge,
                    })
                    .collect();
                for sample in 0..u64::from(samples) {
                    let wave = ((sample * 37 % 97) as f32 - 48.25) / 194.0;
                    let (mut left, mut right) = ([wave], [-wave]);
                    let side = [wave * 0.5];
                    let block = EffectProcessBlock::new(
                        &mut left,
                        &mut right,
                        connected.then_some((&side[..], &side[..])),
                        sample,
                        if sample == 0 { &spans } else { &[] },
                        shape.quantum,
                    )
                    .expect("a well-shaped block");
                    let _ = effect.process(block);
                    if positions.binary_search(&sample).is_err() {
                        continue;
                    }
                    let payload = snapshot_scalar(effect.as_ref(), sizes, false);
                    let mut twin = factory
                        .prepare(request(shape, &values))
                        .expect("the same request prepares again");
                    let input = StatePayloadInput::new(
                        &payload.common,
                        &payload.left,
                        &payload.right,
                        sizes,
                    )
                    .expect("the prepared sizes");
                    if let Err(error) =
                        twin.restore_state_payload(descriptor.state_layout_version, input)
                    {
                        violations.push(format!(
                            "{} at {} Hz: `{}` ramp from {start:e} ({:#010x}) to {edge}, after \
                             sample {sample}: its own snapshot is refused ({})",
                            descriptor.display_name,
                            quality.sample_rate,
                            parameter.display_name,
                            start.to_bits(),
                            error.code
                        ));
                        break;
                    }
                }
            }
        }
    }
    violations
}

/// A start a few hundred ulps inside `edge` whose `f32` ramp of `samples` steps to `edge`
/// (`step = (edge - start) / samples`, then `current += step` before the snap) leaves
/// `[low, high]`, if one is found, with the first step `k` (`1 <= k < samples`) whose `current`
/// is outside.
fn overshooting_start(edge: f32, low: f32, high: f32, samples: u32) -> Option<(f32, u32)> {
    let inward = |value: f32| {
        if edge == high {
            value.next_down()
        } else {
            value.next_up()
        }
    };
    let mut start = edge;
    for _ in 0..4096 {
        start = inward(start);
        if !(low..=high).contains(&start) {
            return None;
        }
        let step = (edge - start) / samples as f32;
        let mut current = start;
        for first_out in 1..samples {
            current += step;
            if !(low..=high).contains(&current) {
                return Some((start, first_out));
            }
        }
    }
    None
}

/// The ascending, distinct samples after which the edge-ramp probe restores a snapshot (#1301):
/// for a ramp of `samples` (`n`) on quality row `row` whose model walk first leaves the domain at
/// step `first_out` (`k`), `{0, n - 2, n - 1} U {k - 2, k - 1, k} U {s : s mod 16 == 4 row mod
/// 16}` within `0..n`, or every sample of `0..n` when `every_sample`.
fn edge_restore_positions(
    samples: u32,
    first_out: u32,
    row: usize,
    every_sample: bool,
) -> Vec<u64> {
    const STRIDE: u64 = 16;
    let n = u64::from(samples);
    if every_sample {
        return (0..n).collect();
    }
    let k = u64::from(first_out);
    let offset = (4 * row as u64) % STRIDE;
    let mut positions: Vec<u64> = [
        Some(0),
        n.checked_sub(2),
        n.checked_sub(1),
        k.checked_sub(2),
        k.checked_sub(1),
        Some(k),
    ]
    .into_iter()
    .flatten()
    .chain((offset..n).step_by(STRIDE as usize))
    .filter(|&sample| sample < n)
    .collect();
    positions.sort_unstable();
    positions.dedup();
    positions
}

// The body of `EffectDifferential::assert_edge_ramps_restore`.
fn assert_edge_ramps_restore(factory: &dyn NativeEffectFactory) {
    let violations = edge_ramp_restore_violations(factory);
    assert!(
        violations.is_empty(),
        "{}: a restore refuses the effect's own mid-ramp snapshot (#1278):\n{}",
        factory.descriptor().display_name,
        violations.join("\n")
    );
}

#[cfg(test)]
mod tests {
    use super::edge_restore_positions;

    #[test]
    fn edge_restore_positions_keep_the_reach_critical_samples() {
        for (n, k) in [(64, 34), (64, 49), (128, 66), (64, 1), (64, 63)] {
            for row in 0..4 {
                let positions = edge_restore_positions(n, k, row, false);
                let (n, k) = (u64::from(n), u64::from(k));
                for required in [0, k - 1, k, n - 2, n - 1] {
                    assert!(
                        positions.contains(&required),
                        "n {n}, k {k}, row {row}: {required} missing from {positions:?}"
                    );
                }
                assert!(positions.iter().all(|&sample| sample < n), "{positions:?}");
                assert!(
                    positions.len() as u64 <= n / 16 + 6,
                    "n {n}, k {k}, row {row}: {} positions",
                    positions.len()
                );
                assert_eq!(
                    edge_restore_positions(n as u32, k as u32, row, true),
                    (0..n).collect::<Vec<_>>()
                );
            }
        }
    }
}
