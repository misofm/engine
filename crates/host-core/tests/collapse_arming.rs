//! Issue #970: the mono collapse is armed only on a chain that gathers the track input.
//!
//! The collapse's structural term, `SOURCE`, says one thing: the track's **input** carries two
//! identical planes (`left_source_channel == right_source_channel`). It is decided per track from
//! the compiled session and joined onto bank chains at bind by
//! `PreparedRenderPlan::arm_mono_collapse`. A chain whose every lane begins at its track's
//! `PostInputBuiltins` stage gathers that input, so the term speaks for its planes. A **later**
//! chain of the same track does not: it reads planes that an earlier chain or a per-node op
//! produced, and whatever made them differ is in neither that chain's own witness nor the join.
//! Armed anyway, it collapses and copies its left plane into its right one: wrong audio, silently.
//!
//! A strip splits into more than one chain when a stage meter or a send taps an internal boundary,
//! when a per-node op sits between banked racks, or -- the commonest case -- when the builtin
//! cohorts (every track) and an effect rack's cohorts (only the tracks that carry that rack
//! program) have different lane sets, so the #208 merge declines with no tap and no per-node op.
//!
//! # The oracle
//!
//! Every probe renders one document twice: collapse armed exactly as `host-core` arms it, and the
//! same plan after `force_mono_collapse_off(true)`, which renders every chain dual. The collapse is
//! a bit-exact optimisation, so any output bit that differs between the two arms is wrong audio in
//! the armed one.
//!
//! # Red on the base, and the red mutation
//!
//! On the shipped rule (every bank chain whose lanes' tracks hold `SOURCE` is armed) each probe
//! named `*_renders_the_dual_bits` failed on the right plane, and the per-node-delay count probe
//! read the shipped value rather than half of it. The two controls pass under both rules. The red
//! mutation is the shipped rule itself: drop `gathers_track_input()` from the conjunction in
//! `Runtime::arm_mono_collapse` (`crates/graph/src/runtime.rs`).

use core::num::{NonZeroU32, NonZeroUsize};

use builtins::{BuiltinLaneSelector, MeterTap};
use builtins_compiler::TrackInputRecord;
use engine::realtime::{PlanUnitEligibility, PlanarBufferMut, RenderIo, RenderTime};
use host_core::{
    HostLiveControlRequest, HostPrepareCaps, HostShapePolicy, SourceSubmission,
    prepare_host_session_with_live_controls,
};
use session::{
    ChannelMatrix, EffectIdentity, EffectParam, ParameterChannel, ParameterUnit, Route,
    RouteDestination, RouteSource, SendTap, SessionModel, StableId, Submix, canonical_session_json,
    parse_session_json,
};

/// Eight tracks, one banked parametric EQ each as the `console.pre_insert` slot `eq`, every route
/// at `post_pan`.
const BANK: &str = include_str!("../../../fixtures/session/v1/parametric-eq-bank-console.json");
const QUANTUM: usize = 128;
const RATE: u32 = 48_000;
const BLOCKS: usize = 16;

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

fn live_controls(tap: MeterTap) -> HostLiveControlRequest {
    HostLiveControlRequest {
        control_queue_depth: Some(NonZeroUsize::new(8).expect("depth")),
        meter_period_frames: Some(NonZeroU32::new(QUANTUM as u32).expect("period")),
        meter_queue_depth: NonZeroUsize::new(16).expect("meter depth"),
        meter_tap: tap,
        observation_taps: 0,
        master_track: None,
    }
}

/// A deterministic broadband signal: every sample differs, so a copied plane cannot hide.
fn signal(index: usize) -> f32 {
    let mut state = (index as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ 0x1234_5678;
    state ^= state >> 33;
    state = state.wrapping_mul(0xff51_afd7_ed55_8ccd);
    state ^= state >> 29;
    (((state >> 40) as f32) / 16_777_216.0) - 0.5
}

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

/// The bank fixture with every track mono-mapped, so `SOURCE` holds on every track.
fn mono_model() -> SessionModel {
    let mut model = parse_session_json(BANK).expect("bank fixture parses");
    model.automation.clear();
    for track in &mut model.tracks {
        track.right_source_channel = track.left_source_channel;
    }
    model
}

fn canonical(model: &SessionModel) -> String {
    canonical_session_json(model).expect("probe canonicalizes")
}

/// Set one per-channel value of `parameter_id` on an effect, replacing any `both` declaration.
fn set_per_channel(
    params: &mut Vec<EffectParam>,
    parameter_id: u32,
    unit: ParameterUnit,
    l: f32,
    r: f32,
) {
    params.retain(|param| param.parameter_id != parameter_id);
    for (channel, value) in [(ParameterChannel::Left, l), (ParameterChannel::Right, r)] {
        params.push(EffectParam {
            parameter_id,
            channel,
            unit,
            value,
        });
    }
}

/// A second, symmetric copy of the fixture's EQ slot as the `console.post_insert` slot `eq2`, so
/// the strip has an upstream stage after `pre_insert` as well.
fn with_post_insert_eq(model: &mut SessionModel) {
    let mut second = model.console.pre_insert[0].clone();
    second.slot = id("eq2");
    model.console.post_insert = vec![second];
    for track in &mut model.tracks {
        let mut entry = track.console[0].clone();
        entry.slot = id("eq2");
        track.console.push(entry);
    }
}

/// Band gain (parameter 4) of the `pre_insert` EQ at `left_db` / `right_db`, and the symmetric
/// `post_insert` copy of [`with_post_insert_eq`].
fn eq_model(left_db: f32, right_db: f32) -> SessionModel {
    let mut model = mono_model();
    with_post_insert_eq(&mut model);
    for track in &mut model.tracks {
        set_per_channel(
            &mut track.console[0].params,
            4,
            ParameterUnit::Db,
            left_db,
            right_db,
        );
    }
    model
}

/// A per-node `miso.delay` insert (`left_ms` / `right_ms`) between the banked `pre_insert` EQ and
/// a second, banked EQ in `post_insert`. The delay banks nowhere, so the strip is two chains around
/// it.
fn delay_model(left_ms: f32, right_ms: f32) -> SessionModel {
    let mut model = mono_model();
    with_post_insert_eq(&mut model);
    let eq = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
    for track in &mut model.tracks {
        let mut delay = eq.clone();
        delay.id = id("dly");
        delay.identity = EffectIdentity::Native {
            effect_id: id("miso.delay"),
        };
        delay.params.clear();
        set_per_channel(
            &mut delay.params,
            1,
            ParameterUnit::Milliseconds,
            left_ms,
            right_ms,
        );
        track.inserts.effects = vec![delay];
    }
    model
}

/// A send from every track at `tap` into one submix, and that submix into the main output.
fn with_send(mut model: SessionModel, tap: SendTap) -> SessionModel {
    let bus = id("bus");
    let identity = ChannelMatrix {
        ll: 1.0,
        lr: 0.0,
        rl: 0.0,
        rr: 1.0,
    };
    model.submixes.push(Submix { id: bus.clone() });
    let output = match &model.routes[0].destination {
        RouteDestination::OutputInput { output_id } => output_id.clone(),
        RouteDestination::SubmixInput { .. } => panic!("the fixture routes tracks to its output"),
    };
    let tracks: Vec<StableId> = model.tracks.iter().map(|track| track.id.clone()).collect();
    for track_id in tracks {
        model.routes.push(Route {
            id: id(&format!("{}-send", track_id.as_str())),
            source: RouteSource::Track { track_id, tap },
            destination: RouteDestination::SubmixInput {
                submix_id: bus.clone(),
            },
            channel_matrix: identity.clone(),
            gain_db: -6.0,
        });
    }
    model.routes.push(Route {
        id: id("bus-main"),
        source: RouteSource::SubmixOutput { submix_id: bus },
        destination: RouteDestination::OutputInput { output_id: output },
        channel_matrix: identity,
        gain_db: 0.0,
    });
    model
}

/// Sixteen mono tracks: the fixture's eight plus eight clones, where every odd-indexed track
/// carries **no** effect. The builtin cohorts hold every track and the EQ cohorts only the even
/// ones, so the lane sets differ and the builtin chain and the EQ chain stay two chains -- no tap,
/// no per-node op, and no prepare-time asymmetry anywhere.
fn misaligned_model() -> SessionModel {
    let mut model = mono_model();
    let (track, route) = (model.tracks[0].clone(), model.routes[0].clone());
    for index in 8..16 {
        let name = format!("eq{index}");
        let mut clone = track.clone();
        clone.id = id(&name);
        model.tracks.push(clone);
        let mut send = route.clone();
        send.id = id(&format!("{name}-main"));
        send.source = RouteSource::Track {
            track_id: id(&name),
            tap: SendTap::PostPan,
        };
        model.routes.push(send);
    }
    // A console slot runs on every track (decision 12), so a strip that only the even tracks
    // carry is an insert on those tracks.
    let lowered: Vec<_> = model
        .tracks
        .iter()
        .map(|track| model.lower_track(track).pre_insert)
        .collect();
    model.console.pre_insert.clear();
    for (index, (track, eq)) in model.tracks.iter_mut().zip(lowered).enumerate() {
        track.console.clear();
        if index % 2 == 0 {
            track.inserts.effects = eq;
        }
    }
    model
}

/// One input-queue record pushed on track `track`'s trim/polarity channel before block `block`.
struct LiveWrite {
    block: usize,
    track: usize,
    record: TrackInputRecord,
}

struct Rendered {
    /// `[left, right]` output bits over every rendered block.
    planes: [Vec<u32>; 2],
    /// `bank_collapse_counters()`: `[blocks rendered collapsed, chains armed with a prefix]`.
    counters: [u64; 2],
    units: Vec<PlanUnitEligibility>,
}

fn render(document: &str, tap: MeterTap, live: Option<&LiveWrite>, forced_off: bool) -> Rendered {
    let (_, mut prepared, mut handles) =
        prepare_host_session_with_live_controls(document, &caps(), &live_controls(tap))
            .unwrap_or_else(|failure| {
                panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
            });
    prepared.plan.force_mono_collapse_off(forced_off);
    let mut planes = [Vec::new(), Vec::new()];
    for block in 0..BLOCKS {
        if let Some(write) = live.filter(|write| write.block == block) {
            handles.track_controls[write.track]
                .input
                .try_push(write.record)
                .expect("bounded queue room");
        }
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
        for meter in handles.meters.iter_mut() {
            while meter.consumer.try_pop().is_ok() {}
        }
    }
    Rendered {
        planes,
        counters: prepared.plan.bank_collapse_counters(),
        units: prepared.plan.unit_eligibility(),
    }
}

/// `(plane, sample)` of the first output bit the two arms disagree on.
fn first_difference(armed: &Rendered, dual: &Rendered) -> Option<(usize, usize)> {
    (0..2).find_map(|plane| {
        armed.planes[plane]
            .iter()
            .zip(&dual.planes[plane])
            .position(|(a, b)| a != b)
            .map(|sample| (plane, sample))
    })
}

/// Render armed and forced off, assert the two agree bit for bit, and return the armed arm.
fn assert_dual_bits(
    case: &str,
    document: &str,
    tap: MeterTap,
    live: Option<&LiveWrite>,
) -> Rendered {
    let armed = render(document, tap, live, false);
    let dual = render(document, tap, live, true);
    assert_eq!(
        dual.counters[0], 0,
        "{case}: the forced-off arm never collapses"
    );
    assert!(
        dual.planes[1]
            .iter()
            .any(|bits| f32::from_bits(*bits) != 0.0),
        "{case}: the oracle's right plane must carry audio, or a copied plane proves nothing"
    );
    assert_eq!(
        first_difference(&armed, &dual),
        None,
        "{case}: the armed render must equal the dual render bit for bit \
         (armed collapse counters {:?})",
        armed.counters
    );
    armed
}

/// The bank width the plan was built at: the widest bank chain's lane count.
fn width(units: &[PlanUnitEligibility]) -> usize {
    units
        .iter()
        .filter(|unit| unit.banked)
        .map(|unit| unit.lane_tracks.len())
        .max()
        .expect("the probe banks")
}

// ---------------------------------------------------------------------------------------------
// The reproducers.
// ---------------------------------------------------------------------------------------------

/// A stage meter at `PostSimd1` splits the strip after the asymmetric `pre_insert` EQ; the later
/// chain (the symmetric `post_insert` EQ) reads the planes that EQ made differ. Shipped: wrong from
/// sample 0.
#[test]
fn a_post_simd1_meter_after_an_asymmetric_eq_renders_the_dual_bits() {
    let document = canonical(&eq_model(6.0, -6.0));
    let armed = assert_dual_bits(
        "asymmetric simd1 EQ, PostSimd1 meter",
        &document,
        MeterTap::PostSimd1,
        None,
    );
    // The first chain gathers the input and is armed; the asymmetric EQ in it declines it.
    assert_eq!(
        armed.counters[0], 0,
        "no chain may collapse on this document"
    );
}

/// A per-node delay (5 ms left, 7 ms right) between two banked EQs. Shipped: the later chain
/// copied the left delay's echo into the right plane from sample 240 (5 ms at 48 kHz).
#[test]
fn an_asymmetric_per_node_delay_between_chains_renders_the_dual_bits() {
    let document = canonical(&delay_model(5.0, 7.0));
    let armed = assert_dual_bits(
        "asymmetric per-node delay",
        &document,
        MeterTap::PostMatrix,
        None,
    );
    assert!(
        armed.counters[0] > 0,
        "the input-gathering chain still collapses: the delay is after it"
    );
}

/// The count half of the delay probe. Each cohort's strip is two chains around the delay, and the
/// shipped rule armed both: `2 * cohorts`, measured on the base as `[32, 2]` at eight lanes and
/// `[64, 4]` at four. Only the chain that gathers the input may be armed, so the count is **half
/// the shipped value**, at either width.
#[test]
fn only_the_input_gathering_chain_of_a_split_strip_is_armed() {
    for (left, right) in [(5.0, 7.0), (5.0, 5.0)] {
        let armed = render(
            &canonical(&delay_model(left, right)),
            MeterTap::PostMatrix,
            None,
            false,
        );
        let tracks = mono_model().tracks.len();
        let cohorts = tracks / width(&armed.units);
        let shipped = 2 * cohorts as u64;
        // The strip really is two collapsible chains per cohort, both over mono tracks only, and
        // the shipped rule armed every one of them.
        let collapsible = armed
            .units
            .iter()
            .filter(|unit| unit.banked && unit.upstream_of_seam_stages > 0)
            .count() as u64;
        assert_eq!(
            collapsible, shipped,
            "{left}/{right} ms: two collapsible chains per cohort"
        );
        assert_eq!(
            armed.counters[1],
            shipped / 2,
            "{left}/{right} ms: half the shipped value -- the first chain of each cohort only"
        );
    }
}

/// A send at `insert_send` taps the boundary after the asymmetric `pre_insert` EQ and splits the
/// strip there.
#[test]
fn a_post_simd1_send_after_an_asymmetric_eq_renders_the_dual_bits() {
    let document = canonical(&with_send(eq_model(6.0, -6.0), SendTap::InsertSend));
    let armed = assert_dual_bits(
        "asymmetric simd1 EQ, post_simd1 send",
        &document,
        MeterTap::PostMatrix,
        None,
    );
    assert_eq!(
        armed.counters[0], 0,
        "no chain may collapse on this document"
    );
}

/// A send at `post_input_builtins` splits the strip after an asymmetric prepare-time trim: the
/// later chain (the symmetric EQs) reads planes the trim made differ.
#[test]
fn a_post_input_builtins_send_after_an_asymmetric_trim_renders_the_dual_bits() {
    let mut model = eq_model(0.0, 0.0);
    for track in &mut model.tracks {
        track.builtins.left.trim_db = 3.0;
        track.builtins.right.trim_db = -3.0;
    }
    let document = canonical(&with_send(model, SendTap::PostInput));
    let armed = assert_dual_bits(
        "asymmetric trim, post_input_builtins send",
        &document,
        MeterTap::PostMatrix,
        None,
    );
    assert_eq!(
        armed.counters[0], 0,
        "no chain may collapse on this document"
    );
}

/// A live left-only input write on a session whose builtin and effect cohorts misalign. Nothing is
/// asymmetric at prepare: the write reaches the builtin chain's witness, which declines, and the
/// later EQ chain -- shipped armed on `SOURCE` alone -- kept collapsing from block 4 on.
fn assert_live_left_only_write(record: TrackInputRecord) {
    let document = canonical(&misaligned_model());
    let live = LiveWrite {
        block: 4,
        track: 0,
        record,
    };
    let armed = assert_dual_bits(
        &format!("misaligned cohorts, live {record:?} on track 0"),
        &document,
        MeterTap::PostMatrix,
        Some(&live),
    );
    assert!(
        armed.counters[0] > 0,
        "the input-gathering chains still collapse until (and away from) the write"
    );
    // The split is real: the even tracks' EQ cohorts are chains of their own, beside the builtin
    // cohorts that hold all sixteen tracks.
    let builtin_cohorts = 16 / width(&armed.units);
    let collapsible = armed
        .units
        .iter()
        .filter(|unit| unit.banked && unit.upstream_of_seam_stages > 0)
        .count();
    assert!(
        collapsible > builtin_cohorts,
        "the EQ chains must be separate from the builtin chains ({collapsible} collapsible \
         chains, {builtin_cohorts} builtin cohorts)"
    );
}

#[test]
fn a_live_left_only_polarity_write_on_misaligned_cohorts_renders_the_dual_bits() {
    assert_live_left_only_write(TrackInputRecord::PolarityInvert {
        lanes: BuiltinLaneSelector::Left,
        inverted: true,
        smoothing_samples: 0,
    });
}

#[test]
fn a_live_left_only_trim_write_on_misaligned_cohorts_renders_the_dual_bits() {
    assert_live_left_only_write(TrackInputRecord::TrimDb {
        lanes: BuiltinLaneSelector::Left,
        db: -6.0,
        smoothing_samples: 0,
    });
}

// ---------------------------------------------------------------------------------------------
// The controls: they pass under both rules, and they keep the fix from disarming too much.
// ---------------------------------------------------------------------------------------------

/// A symmetric per-node delay: bit-identical, and the input-gathering chain still collapses on
/// every block -- the fix must not disarm the chain the structural witness does speak for.
#[test]
fn a_symmetric_per_node_delay_still_collapses_its_first_chain() {
    let document = canonical(&delay_model(5.0, 5.0));
    let armed = assert_dual_bits(
        "symmetric per-node delay",
        &document,
        MeterTap::PostMatrix,
        None,
    );
    assert_eq!(
        armed.counters,
        [armed.counters[1] * BLOCKS as u64, armed.counters[1]],
        "every armed chain collapses on every block"
    );
    assert!(armed.counters[1] > 0);
}

/// An asymmetric EQ in one unsplit chain: bit-identical, and the chain declines on its own witness.
#[test]
fn an_asymmetric_eq_in_one_chain_declines_on_its_own_witness() {
    let document = canonical(&eq_model(6.0, -6.0));
    let armed = assert_dual_bits(
        "asymmetric simd1 EQ, one chain",
        &document,
        MeterTap::PostMatrix,
        None,
    );
    assert_eq!(
        armed.counters[0], 0,
        "the asymmetric EQ declines its own chain"
    );
    assert!(
        armed.counters[1] > 0,
        "and the chain is armed: it gathers the input"
    );
}
