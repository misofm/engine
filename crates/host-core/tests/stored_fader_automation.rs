//! Stored fader automation at preparation (#1646): its bytes are charged as builtin bank payload
//! (gate 2), and a fader automation keeps a mono track's collapse (gate 3). Gate 1 (flat renders
//! the static bits) is on the two hosts: `crates/capi/src/runtime/tests.rs` and
//! `hosts/host-web/src/tests.rs`. Preparation bakes every fader in the canonical floating-point
//! environment, whatever the caller's thread has set (D2).

// The D2 environment test writes the thread's floating-point control word, which is `unsafe`.
#![allow(unsafe_code)]

use automation::ProgramBuilder;
use builtins_compiler::fader_automation_sizes;
use engine::realtime::{PlanUnitEligibility, PlanarBufferMut, RenderIo, RenderTime};
use host_core::{
    HostPrepareCaps, HostPrepareReport, HostShapePolicy, PreparedHost, SourceSubmission,
    prepare_host_session,
};
use session::{
    Automation, AutomationSegment, AutomationShape, ControlAddress, Owner, ParameterChannel,
    ParameterUnit, Section, SessionModel, StableId, canonical_session_json, parse_session_json,
};

/// Eight tracks on one two-channel source, one banked parametric EQ each as the console slot.
const BANK: &str = include_str!("../../../fixtures/session/v1/parametric-eq-bank-console.json");
const QUANTUM: usize = 128;
const RATE: u32 = 48_000;
const BLOCKS: usize = 16;
/// The fader row (builtin parameter 5).
const FADER_DB: u32 = 5;

fn caps(maximum_graph_session_plus_plan_bytes: u64) -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 4_096,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_submixes: 100,
        maximum_vcas: 100,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes,
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

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

fn bank_model() -> SessionModel {
    let mut model = parse_session_json(BANK).expect("bank fixture parses");
    model.automation.clear();
    model
}

/// A flat fader automation at `value` dB on `lane` of track `track`: one `linear` segment from
/// `value` to `value`.
fn flat_fader(
    model: &SessionModel,
    track: usize,
    lane: ParameterChannel,
    value: f32,
) -> Automation {
    Automation {
        id: id(&format!("fader-{track}")),
        target: ControlAddress {
            owner: Owner::Strip(model.tracks[track].id.clone()),
            section: Section::Builtins,
            control: FADER_DB,
            lane,
        },
        segments: vec![AutomationSegment {
            shape: AutomationShape::Linear,
            start_sample: 0,
            end_sample: 48_000,
            start_value: value,
            end_value: value,
            unit: ParameterUnit::Db,
        }],
        offset: session::AutomationOffset::default(),
    }
}

/// [`prepare`], and the heap bytes the preparation left live on this thread (the audited
/// allocator's thread-scoped counters: requested minus released), which the prepared host holds:
/// the compiled model is dropped inside the window.
fn prepare_retained(model: &SessionModel) -> (PreparedHost, u64) {
    let document = canonical_session_json(model).expect("canonical");
    let mark = bench_support::alloc::current_thread_counters();
    let prepared = prepare_host_session(&document, &caps(u64::MAX))
        .map(|(_, prepared)| prepared)
        .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
    let moved = bench_support::alloc::current_thread_delta_since(mark);
    (prepared, moved.requested_bytes - moved.released_bytes)
}

fn prepare(model: &SessionModel, maximum: u64) -> Result<PreparedHost, String> {
    let document = canonical_session_json(model).expect("canonical");
    prepare_host_session(&document, &caps(maximum))
        .map(|(_, prepared)| prepared)
        .map_err(|failure| String::from_utf8_lossy(failure.as_bytes()).into_owned())
}

/// The bytes the host admits against `maximum_graph_session_plus_plan_bytes` for a plan prepared
/// with no live controls and no spectrum: the graph row, the compiled model and the carry program.
fn admitted(report: &HostPrepareReport) -> u64 {
    report.graph_session_plus_plan_bytes
        + report.session_model_bytes
        + report.spectrum_capture_retained_bytes
        + report.route_control_resources.total_bytes
        + report.carry_program_retained_bytes
}

/// The bank width the plan was built at: the widest bank chain's lane count.
fn width(units: &[PlanUnitEligibility]) -> u64 {
    units
        .iter()
        .filter(|unit| unit.banked)
        .map(|unit| unit.lane_tracks.len() as u64)
        .max()
        .expect("the session banks")
}

/// Gate 2: the same session with and without one automated fader lane. `builtin_bank_bytes`,
/// `graph_incremental_plan_bytes` and `graph_session_plus_plan_bytes` each differ by exactly the
/// lane's charge: the builder's byte report for its program and the program's shared allocation,
/// one automation slot per lane of its fader bank, and its offsets cell (D4). The charge is also
/// checked against the heap itself: the bytes the automated prepared host holds live, less the
/// plain one's, are exactly the charge, measured with the audited allocator's thread-scoped
/// counters (the compiled model is dropped inside the window). A cap one byte below the admitted total
/// refuses the session with `host.graph.resource.limit`, and the cap at the total prepares it.
///
/// Test value: red if any term of the charge (the program, the bank slots or the offsets cell)
/// escapes the graph rows, the caller's cap or (through `graph_session_plus_plan_bytes`) the C
/// ABI's replacement peak, if the charged sizes disagree with what lowering actually retains, or
/// if a session without automation pays for it.
#[test]
fn a_fader_automation_is_charged_to_the_graph_plan_rows_and_the_cap() {
    let plain = bank_model();
    let mut automated = plain.clone();
    let entry = flat_fader(&automated, 2, ParameterChannel::Left, -7.25);
    let segments = controls::stored_segments(&entry);
    automated.automation = vec![entry];

    bench_support::alloc::assert_installed();
    // Warm the process statics first, so neither measured preparation pays for them.
    drop(prepare_retained(&plain));
    let (base, base_retained) = prepare_retained(&plain);
    let (with, with_retained) = prepare_retained(&automated);
    let sizes = fader_automation_sizes();
    let lanes = width(&with.plan.unit_eligibility());
    let program = ProgramBuilder::bytes(&[controls::FADER_CELL_RULE.describe(&segments)]);
    assert!(program > 0, "the program holds the segment");
    let charge = program
        + sizes.program_overhead_bytes
        + lanes * sizes.lane_slot_bytes
        + sizes.offsets_cell_bytes;
    for (row, before, after) in [
        (
            "builtin_bank_bytes",
            base.report.builtin_bank_bytes,
            with.report.builtin_bank_bytes,
        ),
        (
            "graph_incremental_plan_bytes",
            base.report.graph_incremental_plan_bytes,
            with.report.graph_incremental_plan_bytes,
        ),
        (
            "graph_session_plus_plan_bytes",
            base.report.graph_session_plus_plan_bytes,
            with.report.graph_session_plus_plan_bytes,
        ),
    ] {
        assert_eq!(
            after - before,
            charge,
            "{row}: {before} -> {after}, the charge is {charge} at {lanes} lanes"
        );
    }

    assert_eq!(
        with_retained - base_retained,
        charge,
        "the heap the automation retains is its charge: {base_retained} -> {with_retained}"
    );

    let total = admitted(&with.report);
    assert!(
        admitted(&base.report) < total - 1,
        "the plain session fits the cap the automated one exceeds"
    );
    drop((base, with));
    prepare(&automated, total).expect("the cap at the admitted total prepares");
    let Err(refusal) = prepare(&automated, total - 1) else {
        panic!("one byte below the admitted total refuses");
    };
    assert!(
        refusal.contains("host.graph.resource.limit"),
        "refused with the graph cap's code: {refusal}"
    );
}

/// A deterministic broadband signal: every sample differs, so a copied plane cannot hide.
fn signal(index: usize) -> f32 {
    let mut state = (index as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ 0x1234_5678;
    state ^= state >> 33;
    state = state.wrapping_mul(0xff51_afd7_ed55_8ccd);
    state ^= state >> 29;
    (((state >> 40) as f32) / 16_777_216.0) - 0.5
}

/// `[left, right]` output bits over [`BLOCKS`] blocks, the source's two planes carrying the same
/// PCM, and the plan's `[blocks rendered collapsed, chains armed with a prefix]`.
fn render(model: &SessionModel) -> ([Vec<u32>; 2], [u64; 2]) {
    render_prepared(prepare(model, 100_000_000).expect("prepares"))
}

/// [`render`] of an already prepared host.
fn render_prepared(mut prepared: PreparedHost) -> ([Vec<u32>; 2], [u64; 2]) {
    let mut planes = [Vec::new(), Vec::new()];
    for block in 0..BLOCKS {
        let base = block * QUANTUM;
        let plane: Vec<f32> = (0..QUANTUM).map(|frame| signal(base + frame)).collect();
        prepared
            .sources
            .submit(
                b"fixture-source",
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
        let output =
            PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM).expect("output planes");
        prepared
            .plan
            .render(
                RenderIo { output },
                RenderTime {
                    absolute_sample: base as u64,
                },
            )
            .expect("render");
        planes[0].extend(samples[..QUANTUM].iter().map(|value| value.to_bits()));
        planes[1].extend(samples[QUANTUM..].iter().map(|value| value.to_bits()));
    }
    (planes, prepared.plan.bank_collapse_counters())
}

/// Gate 3 (D3): a mono track (source channels `0, 0`) with a flat `left`-only fader automation at
/// a value its right lane does not hold keeps its collapse: the plan renders as many collapsed
/// blocks as the same session without the automation, and its output equals, bit for bit, the
/// dual render of a twin whose track reads channels `0, 1`, which carry the same PCM.
///
/// Test value: red if fader automation declines the collapse (a plane per track for no audible
/// change), or if the collapsed render of the automated lane differs from its dual render.
#[test]
fn a_left_only_fader_automation_keeps_a_mono_tracks_collapse() {
    let mut mono = bank_model();
    for track in &mut mono.tracks {
        track.right_source_channel = track.left_source_channel;
    }
    mono.tracks[0].fader.right_db = -2.0;
    let mut automated = mono.clone();
    automated.automation = vec![flat_fader(&automated, 0, ParameterChannel::Left, -9.5)];
    let mut twin = automated.clone();
    twin.tracks[0].right_source_channel = 1;
    assert_ne!(
        twin.tracks[0].left_source_channel, twin.tracks[0].right_source_channel,
        "the twin reads two channels"
    );

    let (plain, plain_counters) = render(&mono);
    let (collapsed, counters) = render(&automated);
    let (dual, _) = render(&twin);
    assert!(counters[0] > 0, "the mono session collapses: {counters:?}");
    assert_ne!(plain[0], collapsed[0], "the automated lane is heard");
    assert_eq!(
        counters, plain_counters,
        "the fader automation changes no collapse"
    );
    assert!(
        dual[0] != dual[1],
        "the automated left lane differs from the right, so a copied plane cannot hide"
    );
    for plane in 0..2 {
        let first = collapsed[plane]
            .iter()
            .zip(&dual[plane])
            .position(|(a, b)| a != b);
        assert_eq!(
            first, None,
            "plane {plane}: the collapsed render equals the twin's dual render"
        );
    }
}

/// The caller words of the D2 environment test: MXCSR on `x86_64`, FPCR on AArch64.
#[cfg(target_arch = "x86_64")]
mod word {
    use lane::softfma::{MXCSR_DAZ, MXCSR_FTZ};

    /// Flush-to-zero, denormals-are-zero and round-toward-zero (RC `11`, bits 13-14).
    pub(super) fn hostile(saved: u32) -> u32 {
        saved | MXCSR_FTZ | MXCSR_DAZ | 0x6000
    }
}

/// FPCR `FZ` and `RMode` `11` (round toward zero); AArch64 has no separate DAZ bit.
#[cfg(target_arch = "aarch64")]
mod word {
    use lane::fpenv::FPCR_FZ;

    pub(super) fn hostile(saved: u64) -> u64 {
        saved | FPCR_FZ | (0b11 << 22)
    }
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
mod canonical_preparation {
    use super::*;
    use lane::fpenv::{
        FpControlWord, canonical_fp_control_word, read_fp_control_word, write_fp_control_word,
    };

    /// Writes the saved word back on drop, so a failed assertion leaves the thread as it was.
    struct Restore(FpControlWord);

    impl Drop for Restore {
        fn drop(&mut self) {
            // SAFETY: `self.0` is the word this test read from this thread before it changed it,
            // so it sets no reserved or `RES0` bit.
            unsafe { write_fp_control_word(self.0) };
        }
    }

    /// Runs `body` with the thread's control word set to `word`, then restores the saved word.
    fn under<T>(word: FpControlWord, body: impl FnOnce() -> T) -> T {
        let _restore = Restore(read_fp_control_word());
        // SAFETY: `word` is the canonical word or `word::hostile` of a word read from this thread,
        // which sets only FTZ, DAZ and RC (MXCSR) or FZ and RMode (FPCR): no reserved bit.
        unsafe { write_fp_control_word(word) };
        body()
    }

    /// A member dB whose composition with [`VCA_DB`] rounds differently under round-toward-zero,
    /// and whose two compositions convert to different gains (asserted).
    const VCA_MEMBER_DB: f32 = -0.01384;
    /// The left offset of the fixture's VCA.
    const VCA_DB: f32 = 1.0 / 3.0;

    /// Static faders of distinct dB on every lane, input trims and filters, no effect (an effect's
    /// own preparation is outside this gate), a flat automated lane on track 2 and a moving
    /// automated pair on track 5. One VCA reaches tracks 1, 2 and 5; track 1's left lane is
    /// [`VCA_MEMBER_DB`].
    fn fixture() -> (SessionModel, Vec<f32>) {
        let mut model = bank_model();
        model.console.pre_insert.clear();
        model.console.post_insert.clear();
        let mut decibels = Vec::new();
        for (index, track) in model.tracks.iter_mut().enumerate() {
            let k = index as f32;
            track.console.clear();
            track.fader.left_db = -0.37 - 1.13 * k;
            track.fader.right_db = 0.71 - 0.89 * k;
            track.builtins.left.trim_db = -1.3 + 0.21 * k;
            track.builtins.left.hpf_hz = 37.3 + 3.1 * k;
            track.builtins.right.lpf_hz = 15_013.0 - 271.0 * k;
            decibels.extend([track.fader.left_db, track.fader.right_db]);
        }
        let flat = flat_fader(&model, 2, ParameterChannel::Left, -7.31);
        let mut moving = flat_fader(&model, 5, ParameterChannel::Both, -3.17);
        moving.segments[0].end_value = 2.5;
        decibels.extend([-7.31, -3.17]);
        model.automation = vec![flat, moving];
        model.tracks[1].fader.left_db = VCA_MEMBER_DB;
        let members = [1, 2, 5]
            .map(|track| model.tracks[track].id.clone())
            .to_vec();
        model.vcas = vec![session::Vca {
            id: id("vca-a"),
            fader: session::DualMonoFader {
                left_db: VCA_DB,
                right_db: -0.1,
                left_mute: false,
                right_mute: false,
            },
            members,
        }];
        (model, decibels)
    }

    /// D2: a caller thread with FTZ, DAZ and round-toward-zero set prepares the plan a canonical
    /// thread prepares. Each arm prepares under its word; both render under the canonical word, so
    /// any difference is a prepared value. The fixture's dB-to-gain conversions and one VCA
    /// composition round differently under round-toward-zero, by a step that reaches the gain
    /// (both asserted), so the gate is not vacuous.
    ///
    /// Test value: red if preparation composes a strip's effective fader, or bakes a fader gain or
    /// an input coefficient, outside `CanonicalFpEnv` (remove the guard around host-core's
    /// `controls::effective`, in the builtins preparation body or in
    /// `into_graph_artifact_with_banks`), so a flat automated lane and a static fader would take
    /// the caller's rounding.
    #[test]
    fn a_hostile_caller_environment_prepares_the_canonical_faders() {
        let canonical = canonical_fp_control_word();
        let hostile = word::hostile(canonical);
        let (model, decibels) = fixture();

        let moved = under(hostile, || {
            decibels
                .iter()
                .filter(|&&db| {
                    let gain = builtins::checked_fader_gain(db).expect("in domain");
                    let pinned = {
                        let _pin = lane::CanonicalFpEnv::enter();
                        builtins::checked_fader_gain(db).expect("in domain")
                    };
                    gain.to_bits() != pinned.to_bits()
                })
                .count()
        });
        assert!(
            moved > 0,
            "the gate is vacuous: no fixture fader converts differently under the hostile word"
        );
        let offset = f64::from(VCA_DB);
        let composed = session::vca_compose_db(VCA_MEMBER_DB, offset);
        let rounded = under(hostile, || session::vca_compose_db(VCA_MEMBER_DB, offset));
        assert_ne!(
            builtins::checked_fader_gain(composed).map(f32::to_bits),
            builtins::checked_fader_gain(rounded).map(f32::to_bits),
            "the gate is vacuous: the VCA composition's rounding does not reach the gain"
        );

        let prepared_canonical = under(canonical, || prepare(&model, 100_000_000));
        let prepared_hostile = under(hostile, || prepare(&model, 100_000_000));
        let render_arm = |prepared: Result<PreparedHost, String>| {
            under(canonical, || render_prepared(prepared.expect("prepares")).0)
        };
        let reference = render_arm(prepared_canonical);
        let candidate = render_arm(prepared_hostile);
        assert!(
            reference[0].iter().any(|&bits| bits != 0),
            "the fixture renders sound"
        );
        for plane in 0..2 {
            let first = reference[plane]
                .iter()
                .zip(&candidate[plane])
                .position(|(a, b)| a != b);
            assert_eq!(
                first, None,
                "plane {plane}: the hostile caller's preparation renders the canonical bits"
            );
        }
    }
}
