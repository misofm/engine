//! The per-track channel-symmetry witness, end to end: session in, census out.
//!
//! # What this file is defending
//!
//! Mono-collapse will process one plane for a track whose two channels are doing identical work.
//! The witness is the bit that says so, and everything here is about the ways that bit must be
//! *false*. A witness that is wrongly false costs a missed optimisation; a witness that is wrongly
//! true renders the wrong audio. So every test below drives a real prepared session through the
//! real drain and asserts a decline, and the two positive tests exist only to prove the negatives
//! are not vacuous.
//!
//! **Nothing here reads the witness to decide anything rendered.** The census is
//! `PreparedRenderPlan::symmetry_counters`, which is sealed while the render audit is armed, and
//! the render output is deliberately never compared: this phase changes zero behaviour.
//!
//! The fixture is `parametric-eq-bank-console`: eight identical SIMD-rack EQ chains, every
//! parameter declared `channel = "both"`, so the planner forms a full homogeneous bank and every
//! designed word starts symmetric.
//!
//! # The two halves, and why the census does not answer everything
//!
//! The witness has two owners, because its terms are decided at different times by different
//! parties:
//!
//! * `session_structural_symmetry` answers the `SOURCE` term from the **compiled session**,
//!   before any plan exists. That is the planner's pooling class -- the later phase partitions
//!   cohorts by it -- and it is a control-plane function rather than a field of a prepared object
//!   for a concrete reason: the prepared input section's byte size is a sealed fixture-ABI number
//!   (the sealed builtin-compiler mutation-matrix transcript), and a phase that changes no behaviour must not move a sealed
//!   byte count to carry a bit nothing rendered reads.
//! * `PreparedRenderPlan::symmetry_counters` answers the other four terms from the **built
//!   runtime**: what preparation designed, what a restore contradicted, and what the live drains
//!   have admitted.
//!
//! The collapse decision is their conjunction, and the two are asserted separately below because
//! they are separately wrong-able. The checked-in fixture's source mapping is `left = 0,
//! right = 1` -- stereo, so the structural half declines it -- while its parameters are entirely
//! symmetric, so the runtime half admits it. That disagreement is the shape of the design, not a
//! bug: the mono variant used by the live-term tests is the same text with the right channel
//! remapped, mutated here rather than checked in as a second fixture so the two cannot drift.

use core::num::{NonZeroU32, NonZeroUsize};

use miso_engine_builtins::MeterTap;
use miso_engine_effect_contract::{EffectControlRecord, ParameterChannel};
use miso_engine_host_core::{
    ChannelSymmetryWitness, EffectRack, HostConsoleHandles, HostConsoleRequest,
    HostPrepareCaps, HostShapePolicy, PreparedHost, SourceSubmission,
    prepare_host_session_with_console, session_structural_symmetry,
};
use miso_engine_session::CompiledSession;

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-bank-console.toml");
const QUANTUM: usize = 128;
const TRACKS: usize = 8;
/// `band-1-gain` is parameter id 4 of `miso.parametric-eq`, which is index 3 of its table.
const BAND_GAIN_INDEX: u32 = 3;

/// The fixture with both dual-mono lanes reading source channel 0: a mono source mapping.
fn mono_session() -> String {
    let mutated = SESSION.replace("right_source_channel = 1", "right_source_channel = 0");
    assert_ne!(mutated, SESSION, "the fixture's stereo mapping moved");
    mutated
}

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 100_000_000,
        maximum_source_total_bytes: 10_000_000,
        maximum_source_overhead_bytes: 10_000_000,
        maximum_effect_state_bytes: 100_000_000,
        maximum_effect_scratch_bytes: 100_000_000,
        maximum_builtin_retained_bytes: 100_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        maximum_meter_streams: 64,
        maximum_meter_items: 1 << 16,
        maximum_meter_bytes: 1 << 24,
    }
}

fn console() -> HostConsoleRequest {
    HostConsoleRequest {
        control_queue_depth: Some(NonZeroUsize::new(8).expect("depth")),
        meter_period_frames: Some(NonZeroU32::new(QUANTUM as u32).expect("period")),
        meter_queue_depth: NonZeroUsize::new(16).expect("meter depth"),
        meter_tap: MeterTap::PostMatrix,
        observation_taps: 0,
        master_track: None,
    }
}

struct Console {
    prepared: PreparedHost,
    handles: HostConsoleHandles,
    block: usize,
}

fn prepare(toml: &str) -> (CompiledSession, Console) {
    let (session, console) = prepare_unbanked(toml);
    // The census is a statement about banked lanes, so a plan that never banked would make every
    // assertion below prove nothing.
    assert!(
        console.prepared.report.effect_bank_scratch_bytes > 0,
        "the cohort planner bound at least one homogeneous bank on this host"
    );
    (session, console)
}

/// [`prepare`] without the "this arm banked" precondition.
///
/// One arm in this file deliberately does not bank: a session with one prepare-time-asymmetric
/// track splits the cohort planner's pool by collapse class (mono-collapse M1), and neither half
/// of an eight-track fixture reaches an eight-lane bank width. That arm is asserted per track
/// rather than on the census, so the precondition is not one it owes.
fn prepare_unbanked(toml: &str) -> (CompiledSession, Console) {
    let (session, prepared, handles) = prepare_host_session_with_console(toml, &caps(), &console())
        .unwrap_or_else(|failure| {
            panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
        });
    assert_eq!(handles.tracks.len(), TRACKS);
    (
        session,
        Console {
            prepared,
            handles,
            block: 0,
        },
    )
}

/// `[collapse-eligible lanes, lanes]` over the built runtime.
fn census(console: &Console) -> [u64; 2] {
    console.prepared.plan.symmetry_counters()
}

/// Renders `blocks` quanta of a constant signal. The output is deliberately discarded: this phase
/// asserts nothing about rendered bits, only that the drain ran and moved the witness.
fn render(console: &mut Console, blocks: usize) {
    use miso_engine_core::realtime::{PlanarBufferMut, RenderIo, RenderTime};
    let plane = [0.25_f32; QUANTUM];
    for step in 0..blocks {
        let block = console.block + step;
        console
            .prepared
            .sources
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 1,
                    start_frame: (block * QUANTUM) as u64,
                    sample_rate_hz: 48_000,
                    planes: &[&plane, &plane],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut samples = [0.0_f32; QUANTUM * 2];
        let output =
            PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM).expect("output planes");
        console
            .prepared
            .plan
            .render(
                RenderIo {
                    input: None,
                    output,
                },
                RenderTime {
                    absolute_sample: (block * QUANTUM) as u64,
                },
            )
            .expect("render");
        for meter in console.handles.meters.iter_mut() {
            let _ = meter.consumer.try_pop();
        }
    }
    console.block += blocks;
}

fn push(console: &mut Console, track_id: &str, record: EffectControlRecord) {
    let producer = console
        .handles
        .effect_controls
        .iter_mut()
        .find(|producer| {
            producer.track_id.as_ref() == track_id
                && producer.rack == EffectRack::Simd1
                && producer.effect_index == 0
        })
        .expect("a control channel for the addressed effect");
    producer
        .producer
        .try_push(record)
        .expect("room in the bounded queue");
}

fn parameter(channel: ParameterChannel, value: f32) -> EffectControlRecord {
    EffectControlRecord::Parameter {
        parameter_index: BAND_GAIN_INDEX,
        channel,
        value,
    }
}

// ---------------------------------------------------------------------------------------------
// The structural term
// ---------------------------------------------------------------------------------------------

/// Red mutation: make `track_mono_source` return `true` unconditionally -> the stereo row's
/// `assert!(!witness.eligible())` fails. Make it return `false` unconditionally -> both mono rows
/// fail.
#[test]
fn the_structural_witness_follows_the_source_mapping() {
    // Every case is a *valid* mapping of the fixture's two-channel source; session validation
    // refuses any channel index at or above `channel_count`, which is why a one-channel source is
    // not a separate case (it can only ever be mapped `0, 0`).
    let cases: [(&str, &str, bool); 3] = [
        // The checked-in fixture: the two lanes read different channels of one stereo source.
        ("stereo", "right_source_channel = 1", false),
        // Both lanes on channel 0: the mono mapping a collapse needs.
        ("mono-left", "right_source_channel = 0", true),
        // Both lanes on channel 1: mono too. "Mono" is *same channel*, not *channel zero*.
        ("mono-right", "right_source_channel = 1", true),
    ];
    for (name, right, expected) in cases {
        let mut toml = SESSION.replace("right_source_channel = 1", right);
        if name == "mono-right" {
            toml = toml.replace("left_source_channel = 0", "left_source_channel = 1");
        }
        let (session, _console) = prepare(&toml);
        let witnesses = session_structural_symmetry(&session);
        assert_eq!(witnesses.len(), TRACKS, "{name}: one witness per track");
        for (track, witness) in &witnesses {
            assert_eq!(
                witness.holds(ChannelSymmetryWitness::SOURCE),
                expected,
                "{name}: track {track} SOURCE term"
            );
            assert_eq!(
                witness.eligible(),
                expected,
                "{name}: track {track} structural eligibility"
            );
            if !expected {
                assert_eq!(
                    witness.declined(),
                    ChannelSymmetryWitness::SOURCE,
                    "{name}: a stereo mapping declines on SOURCE and nothing else"
                );
            }
        }
    }
}

/// The two halves are separate and both are load-bearing: the runtime census is **source
/// agnostic**, and a session that is fully symmetric at runtime can still be structurally
/// ineligible.
///
/// Red mutation: make `symmetry_counters` fold `ChannelSymmetryWitness::DECLINED` for any node
/// kind, or make `track_mono_source` constant -> one of the two halves stops discriminating and
/// this fails.
#[test]
fn the_two_halves_of_the_witness_are_decided_independently() {
    let (stereo_session, stereo) = prepare(SESSION);
    let (mono_session_model, mono) = prepare(&mono_session());

    let [stereo_eligible, stereo_lanes] = census(&stereo);
    let [mono_eligible, mono_lanes] = census(&mono);
    assert!(stereo_lanes >= TRACKS as u64, "every track is counted");
    assert_eq!(
        [stereo_eligible, stereo_lanes],
        [mono_eligible, mono_lanes],
        "the runtime census sees no difference: the source mapping is not one of its four terms"
    );
    assert_eq!(
        stereo_eligible, stereo_lanes,
        "and both sessions are fully symmetric at runtime, having only both-channel parameters"
    );

    // The structural half, from the compiled session alone, with no plan at all -- and it is the
    // half that separates them.
    assert!(
        session_structural_symmetry(&stereo_session)
            .iter()
            .all(|(_, witness)| !witness.eligible()),
        "the planner's half declines every track of the stereo session"
    );
    assert!(
        session_structural_symmetry(&mono_session_model)
            .iter()
            .all(|(_, witness)| witness.eligible()),
        "and admits every track of the mono one"
    );

    // The conjunction is what a collapse would ask, and it is nobody's default: an eligible lane
    // and an ineligible track compose to ineligible.
    let combined = ChannelSymmetryWitness::SYMMETRIC
        .and(session_structural_symmetry(&stereo_session)[0].1);
    assert!(!combined.eligible());
    assert_eq!(combined.declined(), ChannelSymmetryWitness::SOURCE);
}

/// The input builtins' designed-word term, through a real session.
///
/// Polarity inversion is folded into the trim word (`InputLane::trim_signed`), so inverting one
/// channel of one track moves exactly one of the twenty-six words the input chain's kernel reads
/// -- and its *sign bit*, which a comparison on values rather than bits would have missed.
///
/// # Why this asserts per track and not on the census
///
/// It used to compare `symmetry_counters` before and after: `[before - 1, lanes]`, one lane fewer
/// over the same denominator. Mono-collapse M1 made that comparison invalid, and the reason is the
/// census caveat written on `PreparedPlanExecutor::symmetry_counters` -- the two arms no longer
/// have the same unit inventory. Inverting one channel of one track clears that track's `DESIGNED`
/// term, which is one of the two terms the cohort planner's pool class rests on, so the fixture's
/// eight tracks split into a seven-track mono pool and a one-track stereo one. Neither reaches the
/// eight-lane bank width on this host, so the arm binds **no effect bank at all**: 25 lanes become
/// 41, and a difference of one lane cannot be read off a denominator that moved by sixteen.
///
/// The claim was never about the totals anyway. It is "exactly that track, and no other", and
/// `PlanUnitEligibility` states it directly: the runtime witness is per lane and `lane_tracks`
/// says which track each lane is. That form is also immune to the plan shape, so it holds whether
/// the track's stages are bank lanes or per-node ops.
///
/// Red mutation: make `InputStage::lane_channel_symmetry` return `true` unconditionally, or drop
/// the `trim` comparison from it -> the `inverted` arm reports no declining track and this fails,
/// and every other test in the file stays green.
#[test]
fn an_asymmetric_input_builtin_declines_its_own_track_only() {
    let mono = mono_session();
    let mutated = mono.replacen(
        "right = { polarity_invert = false, trim_db = 0.0",
        "right = { polarity_invert = true, trim_db = 0.0",
        1,
    );
    assert_ne!(mutated, mono, "the fixture's builtins block moved");

    let (_session, baseline) = prepare(&mono);
    let (_session, inverted) = prepare_unbanked(&mutated);
    assert!(
        declining_tracks(&baseline).is_empty(),
        "the mono fixture starts with every track eligible in every lane"
    );
    let declining = declining_tracks(&inverted);
    assert_eq!(
        declining.len(),
        1,
        "exactly one track declined, and it is the inverted one: {declining:?}"
    );
    // The fixture's tracks are `eq1`..`eq8` and the mutation edits the first `builtins.right`
    // table in the text, so the declining track is the first in normalized order.
    let first = session_structural_symmetry(&_session)
        .first()
        .map(|(track, _)| track.clone())
        .expect("a track");
    assert_eq!(declining.iter().next().map(String::as_str), Some(&*first));
}

/// Every track at least one of whose plan lanes declines the runtime witness.
///
/// The track-keyed reading of `PreparedRenderPlan::unit_eligibility`: one row per scheduling unit,
/// each carrying its lanes' tracks and its lanes' verdicts side by side. Lanes that name no track
/// -- a route, the master reduction -- are skipped, because they are not per-track upstream work
/// and cannot decline.
fn declining_tracks(console: &Console) -> std::collections::BTreeSet<String> {
    let mut declining = std::collections::BTreeSet::new();
    for row in console.prepared.plan.unit_eligibility() {
        for (track, eligible) in row.lane_tracks.iter().zip(row.lane_eligible.iter()) {
            if !track.is_empty() && !*eligible {
                declining.insert(track.to_string());
            }
        }
    }
    declining
}

/// The mono fixture with per-track edits, addressed by track index.
///
/// The two levers are the two the tests below need, and they are levers on *different* things.
/// `bypass` sets a track's EQ to prepare-time bypass, which changes its `EffectProgramKey` and so
/// its cohort; `invert` flips one channel's polarity, which clears that track's `DESIGNED` term and
/// so its **pool class** (mono-collapse M1). Two independent two-valued splits give four cohorts
/// over eight tracks, which is how [`the_scalar_console_effect_arm_maintains_its_own_live_terms`]
/// reaches a plan with no effect bank at any launch lane width.
fn edited(bypass: &[usize], invert: &[usize]) -> String {
    let mut track: Option<usize> = None;
    let mut out = String::new();
    for line in mono_session().lines() {
        let mut line = line.to_owned();
        if let Some(id) = line
            .strip_prefix("id = \"eq")
            .and_then(|rest| rest.strip_suffix('"'))
        {
            track = id.parse::<usize>().ok();
        }
        let index = track.unwrap_or(usize::MAX);
        if line.starts_with("simd1 = ") && bypass.contains(&index) {
            line = line.replacen("bypass = false", "bypass = true", 1);
        }
        if line.starts_with("builtins = ") && invert.contains(&index) {
            let (head, tail) = line.split_at(line.find("right = {").expect("a right table"));
            line = format!(
                "{head}{}",
                tail.replacen("polarity_invert = false", "polarity_invert = true", 1)
            );
        }
        out.push_str(&line);
        out.push('\n');
    }
    out
}

/// A prepare-time bypass seeds the `UNBYPASSED` term false, before a block is ever rendered.
///
/// # The debt this pays, and why it is a separate test from the live one
///
/// `a_live_bypass_declines_its_lane_and_lifting_it_re_earns_the_term` drives the bypass through
/// the queue, so it can only observe the term after a drain. The other half of the rule is the
/// *seeding*: `EffectControlLane::new(control, bypass)` takes the prepared bypass and clears
/// `UNBYPASSED` from it, so an instance the session declared bypassed declines from the moment the
/// plan exists. Nothing exercised that line, and it is the line that decides what a plan says
/// about a session nobody has sent a command to yet -- which is the state every plan starts in.
///
/// The assertion is deliberately made with **no render at all**: a render would drain the queue
/// and the live path could account for the result instead.
///
/// Red mutation: drop the `symmetry.set(UNBYPASSED, !bypass)` line from `EffectControlLane::new`
/// -> the declining set is empty and this fails, while
/// `a_live_bypass_declines_its_lane_and_lifting_it_re_earns_the_term` stays green, because that
/// test's `Bypass(true)` record sets the term through `admit` rather than through the seed.
#[test]
fn a_prepare_time_bypass_seeds_the_unbypassed_term_before_any_render() {
    let (_session, bypassed) = prepare_unbanked(&edited(&[3], &[]));
    let declining = declining_tracks(&bypassed);
    assert_eq!(
        declining.iter().map(String::as_str).collect::<Vec<_>>(),
        vec!["eq3"],
        "the declared-bypassed track declines, and only it, with nothing yet drained"
    );
}

/// The **scalar** console-effect arm maintains its own live terms.
///
/// # The debt this pays
///
/// Two objects drain a live-console queue into a witness: `ConsoleEffectBankStage`, one lane of a
/// bank at a time, and `runtime::NodeKind::ConsoleEffect`, one per-node instance at a time. Every
/// other test in this file runs on the checked-in fixture, which banks all eight of its tracks
/// into one cohort -- so only the first arm was ever executed, and the census would not have
/// noticed if `NodeKind::ConsoleEffect`'s `.and(console.control.symmetry())` had been dropped
/// outright.
///
/// # How the arm is made bank-free at every launch width
///
/// Two independent splits over the eight tracks: a prepare-time bypass (which changes the
/// `EffectProgramKey`, so bypassed and unbypassed tracks can never share a bank) crossed with a
/// polarity inversion (which clears `DESIGNED`, so mono-collapse M1's pool class separates them).
/// Four cohorts of two, and the narrowest launch bank is four lanes, so **no** effect bank binds
/// and every EQ is a per-node `ConsoleEffect`. The assertion on `effect_bank_scratch_bytes` is
/// what makes that a fact rather than an intention.
///
/// Red mutation: drop `.and(console.control.symmetry())` from `NodeKind::channel_symmetry`'s
/// `ConsoleEffect` arm -> the parameter half of this test fails and every banked test stays green.
#[test]
fn the_scalar_console_effect_arm_maintains_its_own_live_terms() {
    let toml = edited(&[4, 5, 6, 7], &[2, 3, 6, 7]);
    let (_session, mut console) = prepare_unbanked(&toml);
    assert_eq!(
        console.prepared.report.effect_bank_scratch_bytes, 0,
        "four cohorts of two cannot fill a bank at any launch width, so every EQ is per-node"
    );
    // The starting picture, entirely from preparation: the inverted tracks lost `DESIGNED` at
    // their input stage and the bypassed ones lost `UNBYPASSED` at their EQ.
    assert_eq!(
        declining_tracks(&console)
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        vec!["eq2", "eq3", "eq4", "eq5", "eq6", "eq7"]
    );

    // Lifting a prepare-time bypass through the scalar drain re-earns the term.
    push(&mut console, "eq4", EffectControlRecord::Bypass(false));
    render(&mut console, 1);
    assert!(
        !declining_tracks(&console).contains("eq4"),
        "the per-node drain admitted the record and the lane re-earned UNBYPASSED"
    );

    // ... and a single-channel parameter write through the same drain declines exactly its lane.
    push(
        &mut console,
        "eq0",
        parameter(ParameterChannel::Left, -18.0),
    );
    render(&mut console, 1);
    let declining = declining_tracks(&console);
    assert!(
        declining.contains("eq0"),
        "the per-node drain's LIVE term must move"
    );
    assert!(
        !declining.contains("eq1"),
        "and must move for that instance alone: eq1 is its cohort partner"
    );
}

// ---------------------------------------------------------------------------------------------
// The live terms
// ---------------------------------------------------------------------------------------------

/// Red mutation: delete the `self.symmetry.admit(&record)` line from `EffectControlLane::stage`
/// -> the census does not move and this fails at the `after` assertion.
#[test]
fn a_left_channel_command_declines_exactly_one_lane() {
    let (_session, mut console) = prepare(&mono_session());
    let [before, lanes] = census(&console);
    assert_eq!(before, lanes, "the mono fixture starts fully eligible");

    push(
        &mut console,
        "eq2",
        parameter(ParameterChannel::Left, -24.0),
    );
    // The record is still in the queue: admission into the *rendered* state is the drain, and the
    // drain is at the top of the next block. Until then the witness has seen nothing.
    assert_eq!(
        census(&console),
        [before, lanes],
        "a queued record has not been admitted yet"
    );

    render(&mut console, 1);
    let [after, after_lanes] = census(&console);
    assert_eq!(after_lanes, lanes, "the lane count is structural");
    assert_eq!(
        after,
        before - 1,
        "exactly the commanded lane declined; no cross-lane coupling"
    );
}

/// Two per-lane writes that leave the channels **agreeing** still decline the lane.
///
/// # Why this test exists, and why the answer is "decline"
///
/// It is the one case that separates the two mechanisms. A `Left` retarget followed by a `Right`
/// retarget to the same value is how the ABI addresses a `PerLane` parameter, and it lands the two
/// channels on identical designed words -- so the *recompute* (each effect's
/// `channel_symmetry`) says symmetric, while the *event* hook says a single-channel write was
/// admitted. Every other test in this file is satisfied by either mechanism, so only this one
/// pins the drain hook itself.
///
/// The witness declines, deliberately. It is event-maintained precisely so it costs nothing per
/// block, and the price of that is that it cannot see two writes cancel; declining is the safe
/// direction (a missed collapse, never a wrong render), and the later phase's re-engagement rule
/// is where a lane earns its way back -- on proven state equality, not on the words agreeing.
///
/// Red mutation: delete `self.symmetry.admit(&record)` from `EffectControlLane::stage` -> this
/// fails, and it is the only test in the file that does.
#[test]
fn two_per_lane_writes_that_agree_still_decline_the_lane() {
    let (_session, mut console) = prepare(&mono_session());
    let [before, lanes] = census(&console);
    assert_eq!(before, lanes, "the fixture starts fully eligible");

    push(&mut console, "eq2", parameter(ParameterChannel::Left, -6.0));
    push(
        &mut console,
        "eq2",
        parameter(ParameterChannel::Right, -6.0),
    );
    render(&mut console, 8);
    assert_eq!(
        census(&console),
        [before - 1, lanes],
        "the admitted single-channel writes decline the lane even though the words now agree"
    );
}

/// Red mutation: make `ParameterChannel::writes_one_channel` return `true` for `Both` -> this
/// fails, and `a_left_channel_command_declines_exactly_one_lane` still passes. The pair is what
/// pins the rule in both directions.
#[test]
fn a_both_channel_command_preserves_every_lane_including_mid_ramp() {
    let (_session, mut console) = prepare(&mono_session());
    let [before, lanes] = census(&console);
    assert_eq!(before, lanes, "the mono fixture starts fully eligible");

    // The EQ's coefficient ramp is 64 sample updates and the quantum is 128, so a command drained
    // at the top of block N is mid-flight for the first half of that block and settled by its end.
    // Sampling the census after one block *and* after several is what proves the witness survives
    // the ramp rather than merely surviving its endpoint: the designed-word comparison includes
    // `step`, `target` and `remaining`, all of which are non-trivial while a ramp is in flight.
    push(
        &mut console,
        "eq2",
        parameter(ParameterChannel::Both, -24.0),
    );
    render(&mut console, 1);
    assert_eq!(
        census(&console),
        [before, lanes],
        "a both-channel retarget advances both ramps identically"
    );
    render(&mut console, 4);
    assert_eq!(
        census(&console),
        [before, lanes],
        "and is still symmetric once the ramp has settled"
    );

    // A second retarget while the first is still in flight, then a third: the ramp is restarted
    // from a mid-flight `current` on both channels at once, which is the case a witness that only
    // compared settled targets would get wrong.
    for value in [-6.0, -12.0, 0.0] {
        push(
            &mut console,
            "eq2",
            parameter(ParameterChannel::Both, value),
        );
        render(&mut console, 1);
        assert_eq!(
            census(&console),
            [before, lanes],
            "a both-channel retarget over a ramp in flight stays symmetric"
        );
    }
}

/// Red mutation: change `EffectControlRecord::symmetry_event`'s `Bypass` arm to
/// `SymmetryEvent::Preserve` -> the bypass half fails while every parameter test stays green.
#[test]
fn a_live_bypass_declines_its_lane_and_lifting_it_re_earns_the_term() {
    let (_session, mut console) = prepare(&mono_session());
    let [before, lanes] = census(&console);
    assert_eq!(before, lanes, "the mono fixture starts fully eligible");

    push(&mut console, "eq5", EffectControlRecord::Bypass(true));
    render(&mut console, 1);
    assert_eq!(
        census(&console),
        [before - 1, lanes],
        "a bypassed lane declines: its dry shunt copies both planes, so the cohort cannot collapse"
    );

    push(&mut console, "eq5", EffectControlRecord::Bypass(false));
    render(&mut console, 1);
    assert_eq!(
        census(&console),
        [before, lanes],
        "lifting the bypass re-earns the term: bypass moves no designed word"
    );

    // ... but an asymmetric parameter write is not undone by bypass traffic, because the two terms
    // are separate bits and only one of them is reversible in this phase.
    push(
        &mut console,
        "eq5",
        parameter(ParameterChannel::Right, -3.0),
    );
    push(&mut console, "eq5", EffectControlRecord::Bypass(true));
    push(&mut console, "eq5", EffectControlRecord::Bypass(false));
    render(&mut console, 1);
    assert_eq!(
        census(&console),
        [before - 1, lanes],
        "an asymmetric write is monotone within a plan"
    );
}

/// A subscription is not a parameter write. Red mutation: make the `Observe` arm of
/// `symmetry_event` return `Desymmetrize` -> this fails.
#[test]
fn an_observe_record_never_moves_the_witness() {
    let (_session, mut console) = prepare(&mono_session());
    let [before, lanes] = census(&console);
    push(
        &mut console,
        "eq1",
        EffectControlRecord::Observe {
            tap_index: 0,
            armed: true,
            window_blocks: 1,
        },
    );
    render(&mut console, 1);
    assert_eq!(
        census(&console),
        [before, lanes],
        "arming a tap changes what is read, never what is rendered"
    );
}

/// Seam-side traffic -- fader, mute, pan, matrix -- is excluded by design, and the exclusion is
/// structural: `TrackFaderRecord` and `TrackControlRecord` are seam-side record types, so no
/// drain of theirs can reach a witness.
///
/// Red mutation: change `SEAM_SIDE_WITNESS` to `DECLINED`, or make
/// `ChannelSymmetryWitness::admit` ignore `R::SEAM` -> this fails.
#[test]
fn seam_side_console_traffic_leaves_every_lane_eligible() {
    use miso_engine_builtins::BuiltinLaneSelector;
    use miso_engine_builtins::Matrix2x2;
    use miso_engine_builtins_compiler::{TrackControlRecord, TrackFaderRecord};

    let (_session, mut console) = prepare(&mono_session());
    let [before, lanes] = census(&console);
    assert_eq!(before, lanes, "the mono fixture starts fully eligible");

    let control = console
        .handles
        .track_controls
        .iter_mut()
        .find(|producer| producer.track_id.as_ref() == "eq3")
        .expect("a control channel for the addressed track");
    control
        .fader
        .try_push(TrackFaderRecord::FaderDb {
            // One lane only: the most asymmetric fader move the ABI can express.
            lanes: BuiltinLaneSelector::Left,
            db: -18.0,
            smoothing_samples: 64,
        })
        .expect("room in the bounded queue");
    control
        .producer
        .try_push(TrackControlRecord {
            matrix: Matrix2x2 {
                ll: 0.25,
                lr: 0.75,
                rl: 0.5,
                rr: 0.5,
            },
            smoothing_samples: 64,
        })
        .expect("room in the bounded queue");

    render(&mut console, 4);
    assert_eq!(
        census(&console),
        [before, lanes],
        "the collapse duplicates its one plane into the fader and the matrix, so their per-channel \
         words are free to differ"
    );
}
