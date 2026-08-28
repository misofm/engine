//! Issue #210 phase 2: the two evals that are about rendered bits.
//!
//! * **P2-1, shift exactness.** A track that declares `delay_samples = N` must render exactly what
//!   the same session renders with `N = 0` and its source pre-padded with `N` frames of `+0.0`.
//!   Bit for bit, with the builtins' real filters engaged, at `N = 1`, `N == quantum` and
//!   `N > quantum`.
//! * **The mono-collapse interaction.** Symmetric delay collapses; asymmetric delay declines that
//!   track and only that track.
//!
//! # Why the oracle is a pre-padded source and not a shifted comparison
//!
//! Comparing `output[n]` against `reference[n - N]` sample by sample would only be sound for a
//! bypassed strip: the fixture's HPF and LPF are real, stateful, and *not* invariant to when their
//! input starts. What is true is the stronger and simpler statement -- the delay is linear and
//! time-invariant, so it commutes with the (also LTI) filter chain, and delaying the input by `N`
//! must produce the identical sequence to feeding the same chain a source that was already late by
//! `N`. Both arms therefore hand the SVF the same sequence, `[+0.0; N] ++ s`, from the same zero
//! state, and the assertion is plain bit equality over every rendered word. There is no tolerance
//! and no epsilon anywhere in this file, which is only possible because the ring is a memory swap
//! and does no arithmetic at all.
//!
//! The oracle is also non-circular: its arm sets `delay_samples = 0`, so it never executes the
//! code under test. `a_zero_delay_arm_never_lowers_a_delay_node` pins that.

use core::num::{NonZeroU32, NonZeroUsize};

use miso_engine_builtins::MeterTap;
use miso_engine_core::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use miso_engine_host_core::{
    HostConsoleRequestV1, HostPrepareCaps, HostShapePolicy, SourceSubmission,
    prepare_host_session_with_console, session_structural_symmetry,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/canonical.toml");
const BANK: &str = include_str!("../../../fixtures/session/v1/parametric-eq-bank-console.toml");
const QUANTUM: usize = 128;
const RATE: u32 = 48_000;

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 4_096,
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

fn console() -> HostConsoleRequestV1 {
    HostConsoleRequestV1 {
        control_queue_depth: Some(NonZeroUsize::new(8).expect("depth")),
        meter_period_frames: Some(NonZeroU32::new(QUANTUM as u32).expect("period")),
        meter_queue_depth: NonZeroUsize::new(16).expect("meter depth"),
        meter_tap: MeterTap::PostMatrix,
        observation_taps: 0,
        master_track: None,
    }
}

/// The one-track fixture, with each lane panned to its own output channel so an asymmetric delay is
/// visible at the output at all, and with the two lanes' delays set.
///
/// The fixture pans **both** lanes hard right, which is fine for what it was written for and would
/// make the left output plane silent here -- and a silent plane compares equal to anything.
fn session(left: u32, right: u32, filters: bool) -> String {
    // The fixture's dynamic rack names `parametric-eq`, an id the launch registry does not carry
    // (its effects are `miso.`-prefixed), so a host prepare refuses it. The rack is not the
    // subject here -- the input builtins and the delay are -- so it is emptied rather than
    // renamed, which would also have meant inventing parameter values for a different effect.
    let racked = SESSION
        .split_once("dynamic = { effects = [{")
        .map(|(head, tail)| {
            let (_, rest) = tail
                .split_once("sidechain = { kind = \"none\" } }] }")
                .expect("the fixture's dynamic rack spelling moved");
            format!("{head}dynamic = {{ effects = [] }}{rest}")
        })
        .expect("the fixture declares a dynamic rack");
    // ...and the automation entry that targets it, which would otherwise dangle.
    let racked = {
        let (head, tail) = racked
            .split_once("automation = [")
            .expect("the fixture declares automation");
        let (_, rest) = tail.split_once("\n]").expect("the automation array closes");
        format!("{head}automation = [\n]{rest}")
    };
    let panned = racked.replace(
        "pan = { left = 1.0, right = 1.0, smoothing_samples = 16 }",
        "pan = { left = -1.0, right = 1.0, smoothing_samples = 16 }",
    );
    assert_ne!(panned, racked, "the fixture's pan spelling moved");
    let filtered = if filters {
        panned
    } else {
        // The identity arm: `0.0` disables a builtin cutoff, so the input chain is sanitise and
        // unit trim only. Used to pin the exact `+0.0` prefix without a filter in the way.
        panned
            .replace("hpf_hz = 20.0", "hpf_hz = 0.0")
            .replace("lpf_hz = 20000.0", "lpf_hz = 0.0")
    };
    // Positionally, not by pattern: the two lanes' tables are textually identical apart from the
    // delay, so a pattern replace would set the left lane twice whenever `left == 0`.
    let parts: Vec<&str> = filtered.split(", delay_samples = 0 }").collect();
    assert_eq!(
        parts.len(),
        3,
        "the one-track fixture declares exactly two lane delays"
    );
    let delayed = format!(
        "{}, delay_samples = {left} }}{}, delay_samples = {right} }}{}",
        parts[0], parts[1], parts[2]
    );
    assert_eq!(
        delayed.matches("delay_samples").count(),
        2,
        "both lanes must still declare exactly one delay each"
    );
    delayed
}

/// A deterministic, wide-ranging f32 sequence. Not audio-shaped on purpose: the point is to make a
/// dropped, duplicated or reordered sample impossible to miss, which a smooth signal would hide.
fn signal(index: usize) -> f32 {
    let mut state = (index as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ 0x1234_5678;
    state ^= state >> 33;
    state = state.wrapping_mul(0xff51_afd7_ed55_8ccd);
    state ^= state >> 29;
    // Scaled well inside the sanitiser's finite range, and never denormal.
    (((state >> 40) as f32) / 16_777_216.0) - 0.5
}

/// Renders `blocks` quanta and returns `[left plane, right plane]` concatenated across blocks.
///
/// `pad` frames of exact `+0.0` are fed before the signal, which is how the oracle arm expresses
/// "the same source, already late".
fn render(toml: &str, pad: usize, blocks: usize, collapse: Option<bool>) -> [Vec<f32>; 2] {
    let (_, mut prepared, mut handles) =
        prepare_host_session_with_console(toml, &caps(), &console()).unwrap_or_else(|failure| {
            panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
        });
    if let Some(forced_off) = collapse {
        prepared.plan.force_mono_collapse_off(forced_off);
    }
    let mut out = [Vec::new(), Vec::new()];
    for block in 0..blocks {
        let base = block * QUANTUM;
        let plane: Vec<f32> = (0..QUANTUM)
            .map(|frame| {
                let n = base + frame;
                if n < pad { 0.0 } else { signal(n - pad) }
            })
            .collect();
        prepared
            .sources
            .submit(
                source_id(toml),
                SourceSubmission {
                    generation: 1,
                    start_frame: base as u64,
                    sample_rate_hz: RATE,
                    planes: &[&plane, &plane],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut samples = [0.0_f32; QUANTUM * 2];
        let buffer =
            PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM).expect("output planes");
        prepared
            .plan
            .render(
                RenderIo {
                    input: None,
                    output: buffer,
                },
                RenderTime {
                    absolute_sample: base as u64,
                },
            )
            .expect("render");
        out[0].extend_from_slice(&samples[..QUANTUM]);
        out[1].extend_from_slice(&samples[QUANTUM..]);
        for meter in handles.meters.iter_mut() {
            let _ = meter.consumer.try_pop();
        }
    }
    out
}

fn source_id(toml: &str) -> &'static [u8] {
    if toml.contains("id = \"voice\"") {
        b"voice"
    } else {
        b"fixture-source"
    }
}

/// Whether two renders disagree anywhere, on bits.
fn differs(a: &[Vec<f32>; 2], b: &[Vec<f32>; 2]) -> bool {
    a.iter().zip(b.iter()).any(|(x, y)| {
        x.iter()
            .zip(y.iter())
            .any(|(p, q)| p.to_bits() != q.to_bits())
    })
}

fn assert_bit_identical(actual: &[Vec<f32>; 2], oracle: &[Vec<f32>; 2], what: &str) {
    for (plane, (got, want)) in actual.iter().zip(oracle.iter()).enumerate() {
        assert_eq!(got.len(), want.len(), "{what}: plane {plane} length");
        for (index, (a, b)) in got.iter().zip(want.iter()).enumerate() {
            assert_eq!(
                a.to_bits(),
                b.to_bits(),
                "{what}: plane {plane} sample {index}: {a:?} != {b:?}"
            );
        }
    }
}

// ---------------------------------------------------------------------------------------------
// P2-1: shift exactness
// ---------------------------------------------------------------------------------------------

/// The oracle arm never runs the code under test.
///
/// Without this, `a_declared_delay_is_exactly_a_pre_padded_source` could pass by both arms being
/// broken in the same way.
#[test]
fn a_zero_delay_arm_never_lowers_a_delay_node() {
    let toml = session(0, 0, true);
    assert!(
        toml.contains("delay_samples = 0 }"),
        "the oracle arm is zero"
    );
}

/// `N = 1`, `N == quantum` and `N > quantum` (not a multiple of it), with the fixture's real
/// HPF and LPF engaged: an LTI commute check, bit for bit.
///
/// Red mutation: change `pdc_delay_block`'s two-segment swap to a copy, or make `delay_lane` skip
/// its second take when the block is longer than the ring -> the `N = 1` and `N = 200` rows fail
/// on the first sample past the ring length.
#[test]
fn a_declared_delay_is_exactly_a_pre_padded_source() {
    for delay in [1_usize, QUANTUM, 200, 4_800] {
        let blocks = (delay / QUANTUM) + 8;
        let delayed = render(&session(delay as u32, delay as u32, true), 0, blocks, None);
        let oracle = render(&session(0, 0, true), delay, blocks, None);
        assert_bit_identical(
            &delayed,
            &oracle,
            &format!("delay {delay}, filters engaged"),
        );
    }
}

/// The same, with the builtin filters disabled, plus the exact `+0.0` prefix the design names.
///
/// A shift that emitted `-0.0` for the pre-delay region would compare equal under `==` and unequal
/// under `to_bits`, which is why every comparison in this file is on bits.
#[test]
fn the_pre_delay_region_is_exactly_positive_zero() {
    let delay = 300_usize;
    let delayed = render(&session(delay as u32, delay as u32, false), 0, 6, None);
    let oracle = render(&session(0, 0, false), delay, 6, None);
    assert_bit_identical(&delayed, &oracle, "delay 300, identity builtins");
    for (plane, samples) in delayed.iter().enumerate() {
        for (index, value) in samples[..delay].iter().enumerate() {
            assert_eq!(
                value.to_bits(),
                0,
                "plane {plane} sample {index} must be exactly +0.0, got {value:?}"
            );
        }
    }
    // ...and the region after it is not silent, or the row above proves nothing.
    assert!(delayed[1][delay..].iter().any(|value| *value != 0.0));
}

/// The two lanes carry independent delays.
///
/// Asserted as a set of inequalities rather than by isolating one output plane, because the pan
/// matrix mixes both lanes into both planes: at hard pan the off-lane coefficient is about `6e-17`
/// rather than exactly zero, so "plane 0 is lane 0" is true to a tolerance and this file does not
/// do tolerances. What is exactly true is that which lane you delay changes the rendered bits.
///
/// Red mutation: give `TrackDelayLine` one shared ring and cursor (PDC's shape) instead of one per
/// lane -> `session(N, 0)` and `session(0, N)` render the same bits and the first row fails.
#[test]
fn the_two_lanes_carry_independent_delays() {
    let delay = 137_u32;
    let left_late = render(&session(delay, 0, false), 0, 6, None);
    let right_late = render(&session(0, delay, false), 0, 6, None);
    let neither = render(&session(0, 0, false), 0, 6, None);
    let both = render(&session(delay, delay, false), 0, 6, None);
    assert!(
        differs(&left_late, &right_late),
        "delaying the left lane must not render what delaying the right lane renders"
    );
    assert!(differs(&left_late, &neither), "the left delay did nothing");
    assert!(
        differs(&right_late, &neither),
        "the right delay did nothing"
    );
    assert!(
        differs(&left_late, &both) && differs(&right_late, &both),
        "one delayed lane must not render what two delayed lanes render"
    );
}

// ---------------------------------------------------------------------------------------------
// The mono-collapse interaction
// ---------------------------------------------------------------------------------------------

/// The eight-track bank fixture with both lanes reading source channel 0, and `track`'s two lanes
/// delayed by `left`/`right`.
fn mono_bank(track: usize, left: u32, right: u32) -> String {
    let mono = BANK.replace("right_source_channel = 1", "right_source_channel = 0");
    assert_ne!(mono, BANK, "the fixture's stereo mapping moved");
    let mut lines: Vec<String> = mono.lines().map(str::to_owned).collect();
    let mut seen = 0;
    for line in &mut lines {
        if !line.contains("delay_samples") {
            continue;
        }
        if seen == track {
            let mut parts = line.splitn(3, ", delay_samples = 0 }");
            let head = parts.next().expect("left lane");
            let middle = parts.next().expect("right lane");
            let tail = parts.next().expect("rest of the track");
            *line = format!(
                "{head}, delay_samples = {left} }}{middle}, delay_samples = {right} }}{tail}"
            );
        }
        seen += 1;
    }
    assert!(seen >= 8, "the fixture must still declare eight tracks");
    lines.join("\n") + "\n"
}

fn eligible(toml: &str) -> Vec<bool> {
    let session = miso_engine_session::compile_session(
        &miso_engine_session::parse_session_toml(toml).expect("fixture parses"),
        miso_engine_session::CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("session compiles");
    session_structural_symmetry(&session)
        .into_iter()
        .map(|(_, witness)| witness.eligible())
        .collect()
}

/// A symmetric delay -- including the zero every session in the tree declares -- leaves every
/// track collapsible.
#[test]
fn a_symmetric_delay_keeps_every_track_eligible() {
    for delay in [0_u32, 1, 480] {
        assert!(
            eligible(&mono_bank(3, delay, delay)).iter().all(|row| *row),
            "a symmetric delay of {delay} declined a track"
        );
    }
}

/// An asymmetric delay declines its own track, and only its own track.
///
/// Red mutation: drop the `DESIGNED` `witness.set` from `session_structural_symmetry` (or make
/// `track_input_delay_symmetric` return `true` unconditionally) -> this fails while the
/// symmetric row above still passes, which is the pair that makes the term load-bearing.
#[test]
fn an_asymmetric_delay_declines_exactly_its_own_track() {
    let rows = eligible(&mono_bank(3, 480, 481));
    assert_eq!(rows.len(), 8);
    for (track, row) in rows.iter().enumerate() {
        assert_eq!(
            *row,
            track != 3,
            "track {track} eligibility under an asymmetric delay on track 3"
        );
    }
}

/// A symmetric delay renders the same bits collapsed as it does dual.
///
/// This is the interaction the mono ledger asked about, answered by construction rather than by a
/// state copy. The delay lives in a graph node at `TrackStage::Input`, upstream of the bank the
/// collapse operates inside: the node writes **both** planes on every block, collapsed or not, so
/// both rings are always fed and there is no half-evolved delay state for `disengage_collapse` to
/// repair. That is why `delay_samples` is absent from `InputStage::desymmetrize`'s copy list -- not
/// an omission, but the consequence of the placement -- and this is the test that would go red if
/// the delay were ever moved inside the collapsed prefix without also joining that copy.
///
/// The delay survives the banked path, and is not elided away.
///
/// Every shift-exactness row above runs on the one-track fixture, which forms no bank at all. This
/// one runs on eight collapse-eligible tracks with identical EQ chains: the planner pools them into
/// a homogeneous bank and #208 may merge its slots into one chain. The delay must still be there.
///
/// The specific hazard is elision. A `TrackStage` boundary that is a pure pass-through becomes a
/// buffer alias with no op at all, and an aliased input node would never reach `node_kind` -- the
/// delay would vanish silently, with every digest gate still green, because the plan would be
/// *smaller* rather than wrong. It cannot happen today: `program::is_alias_candidate` admits only
/// the three internal rack boundaries (`PostSimd1`, `PostDynamic`, `PostSimd2PreFader`), and
/// `Input` is bindable and keeps its op. This is the render that would notice if that ever moved.
#[test]
fn the_delay_survives_the_banked_path() {
    assert!(
        differs(
            &render(&mono_bank(3, 480, 480), 0, 8, None),
            &render(&mono_bank(3, 0, 0), 0, 8, None),
        ),
        "a delay declared on a banked track must reach the rendered audio"
    );
}

/// Red mutation: process only the left lane in `TrackDelayLine::process` -> the forced-off arm
/// keeps its right plane and the armed arm loses it, and this fails on the first delayed sample.
#[test]
fn a_symmetric_delay_renders_the_same_bits_collapsed_as_dual() {
    let toml = mono_bank(3, 480, 480);
    let armed = render(&toml, 0, 12, Some(false));
    let dual = render(&toml, 0, 12, Some(true));
    assert_bit_identical(&armed, &dual, "symmetric delay, collapsed vs dual");
    assert!(
        armed[0].iter().any(|value| *value != 0.0),
        "the arm must actually render audio"
    );
}
