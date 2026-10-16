//! Issue #1344: prepare a successor across a withdrawn candidate plan.
//!
//! P0 is the plan render runs. A was prepared against P0, committed and taken back before render
//! adopted it. B replaces A: it is prepared against P0 with A's `carried_base` and P0's committed
//! model (D2), and takes A's fresh rings by a checked donation (D3-D5).
//!
//! The twin of every case is the path render would have taken had A not been withdrawn: A'
//! (prepared against a second copy P0' of P0, rendered identically) adopts P0' render-free, B' is
//! prepared against A' normally and adopts A'. B adopting P0 must render what B' renders.
//!
//! * **Gate 1, base across a withdrawn plan.** P0 has sources `a`, `b` and tracks `t1`-`t3` on
//!   `a` (`tb` reads `b`); `t3`'s transient-shaper insert and every input section carry audible
//!   state. Plans are prepared with live controls, so inserts bank with live lanes and can carry.
//!   A replaces `t1`'s insert, removes `b`, adds `c`; B replaces `t2`'s (the spec's
//!   insert-quality edit does not prepare: no launch effect has a second quality). B renders
//!   bit-identically to B' over 16 blocks, with `a` and `t3` carried, `t1` and `t2` restarted and
//!   `b` absent; B prepared with no base does not. (An owner A restarts and B reverts to its P0
//!   form starts fresh here, as in the sequential twin; root ruled that a defect and its fix is a
//!   successor issue, so no assertion pins it either way.)
//! * **Gate 2, `carried_base` follows the installed hand-over.** Over a session with a row in
//!   every inventory family, where A drops a strip and a source that sort first (so A's keys are
//!   not P0's) and restarts what a session edit can restart, A's base holds exactly the P0 rows
//!   whose keys A's hand-over reads (set equality of keys), and they belong to exactly the owners
//!   both plans hold less those A restarted; a plan with no base has none.
//! * **Gate 3, a source absent from P0 is never carried from A.** A adds `d` with 4 seconds, B with
//!   5: B's `d` is a fresh ring at generation 1, frame 0, and no donation applies.
//! * **Gate 4, donation.** `c` fed through A's producers and seeked (anchored) moves to B whole:
//!   B plays it as B' does, and without the donation it would not.
//! * **Gate 5, refusals move nothing.** A donor whose `c` consumer began a block is refused with
//!   `Consumed`; a redeclared or re-sized `c` is never donated. Nothing moves either way.

#![cfg(all(feature = "test-support", feature = "control-provider"))]

#[path = "support/successor.rs"]
mod successor;

use bench_support::alloc as bench_alloc;
use engine::realtime::{CarryOutcome, PlanarBufferMut, PreparedRenderPlan, RenderIo};
use graph_compiler::Backend;
use host_core::{
    DonationError, DonationSide, DonationSideMut, HostLiveControlRequest, HostLiveLanes,
    LiveValuesSuperseded, PlanStateInventory, PreparedHost, SourceControlSet, SourceSubmission,
    SpectrumCaptureRequest, SpectrumChannels, SpectrumTarget, SuccessorBase, apply_donation,
    check_donation, drops_acked_fresh_source, prepare_host_runtime_with_live_controls_and_spectrum,
    prepare_host_runtime_with_live_controls_and_spectrum_successor, spectrum_capture_resources_for,
    test_only_carry_read_keys, test_only_inventory_rows,
};
use session::{
    DualMonoFader, Effect, EffectIdentity, EffectQuality, LinkMode, RouteDestination, RouteSource,
    SendTap, SessionModel, SidechainDeclaration, Source, StableId, Submix, parse_session_json,
};
use std::collections::BTreeSet;
use successor::{QUANTUM, SUCCESSOR_REVISION, Session, caps};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
/// Blocks P0 renders before the swap.
const BEFORE: usize = 4;
/// Blocks B renders after it.
const AFTER: usize = 16;
/// The absolute sample of B's first block.
const SWAP_SAMPLE: u64 = (BEFORE * QUANTUM) as u64;

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("stable ID")
}

fn compressor(instance: &str) -> Effect {
    Effect {
        id: id(instance),
        identity: EffectIdentity::Native {
            effect_id: id("miso.compressor"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        params: Vec::new(),
        sidechain: SidechainDeclaration::None,
    }
}

/// A source declared like the fixture's, under `source`, with its own content and `frames`.
fn source(template: &Source, source: &str, frames: u64) -> Source {
    Source {
        id: id(source),
        content: template
            .content
            .replace("7e94", &format!("{:0>4}", &source[..source.len().min(4)])),
        frames,
        ..template.clone()
    }
}

/// Add track `track` on `source`, a copy of the template track with its own route to the output
/// and, when `insert`, a compressor insert.
fn add_track(
    model: &mut SessionModel,
    template: &SessionModel,
    track: &str,
    source: &str,
    insert: bool,
) {
    let mut added = template.tracks[0].clone();
    added.id = id(track);
    added.source_id = id(source);
    added.inserts.effects = if insert {
        vec![compressor("comp")]
    } else {
        Vec::new()
    };
    model.tracks.push(added);
    let mut route = template.routes[0].clone();
    route.id = id(&format!("{track}-main"));
    route.source = RouteSource::Track {
        track_id: id(track),
        tap: SendTap::PostPan,
    };
    model.routes.push(route);
}

fn remove_track(model: &mut SessionModel, track: &str) {
    model
        .tracks
        .retain(|existing| existing.id.as_str() != track);
    model
        .routes
        .retain(|route| route.id.as_str() != format!("{track}-main"));
}

/// Replace `track`'s insert with another effect under the same insert ID: a prepared change that
/// restarts the insert (no launch effect declares a second quality, so a quality edit does not
/// prepare).
fn replace_insert(model: &mut SessionModel, track: &str) {
    let track = model
        .tracks
        .iter_mut()
        .find(|existing| existing.id.as_str() == track)
        .expect("the track");
    track.inserts.effects[0].identity = EffectIdentity::Native {
        effect_id: id("miso.gate-expander"),
    };
}

/// The fixture with no console slot, so its sessions prepare at four lanes on an eight-lane
/// target too.
fn template() -> SessionModel {
    let mut template = parse_session_json(FIXTURE).expect("fixture parses");
    template.console.pre_insert.clear();
    for track in &mut template.tracks {
        track.console.clear();
    }
    template
}

/// P0: sources `a` and `b`; tracks `t1`, `t2`, `t3` on `a` and `tb` on `b`; `t1` and `t2` each
/// with a compressor insert, and `t3` with a transient shaper: an insert with audible state that
/// every plan here carries, in a bank of its own so that `t1`'s and `t2`'s edits leave its layout
/// alone (batch 15a fold, M3); no console slot.
fn p0_model() -> SessionModel {
    let template = template();
    let mut model = template.clone();
    model.tracks.clear();
    model.routes.clear();
    model.sources = vec![
        source(&template.sources[0], "a", 48_000),
        source(&template.sources[0], "b", 48_000),
    ];
    for (track, insert) in [("t1", true), ("t2", true), ("t3", true)] {
        add_track(&mut model, &template, track, "a", insert);
    }
    model.tracks[2].inserts.effects[0].identity = EffectIdentity::Native {
        effect_id: id("miso.transient-shaper"),
    };
    add_track(&mut model, &template, "tb", "b", false);
    model
}

/// A: P0 with `t1`'s insert replaced, `b` and `tb` removed, and `c` added with track `tc`.
fn a_model() -> SessionModel {
    let template = template();
    let mut model = p0_model();
    replace_insert(&mut model, "t1");
    remove_track(&mut model, "tb");
    model.sources.retain(|source| source.id.as_str() != "b");
    model
        .sources
        .push(source(&template.sources[0], "c", 48_000));
    add_track(&mut model, &template, "tc", "c", false);
    model
}

/// B: A with `t2`'s insert replaced too.
fn b_model() -> SessionModel {
    let mut model = a_model();
    replace_insert(&mut model, "t2");
    model
}

fn backends() -> Vec<Backend> {
    vec![
        #[cfg(target_feature = "avx2")]
        Backend::Simd8,
        Backend::Simd4,
    ]
}

/// One stereo PCM signal per source: no exact-zero sample, distinct per `seed`.
fn planes(seed: u32, frames: usize) -> [Vec<f32>; 2] {
    let plane = |channel: u32| {
        (0..frames as u32)
            .map(|frame| {
                let step = frame
                    .wrapping_mul(7_919)
                    .wrapping_add(seed.wrapping_mul(104_729))
                    .wrapping_add(channel * 31)
                    % 1_999;
                (step as f32 - 999.5) / 2_000.0
            })
            .collect::<Vec<f32>>()
    };
    [plane(0), plane(1)]
}

/// Submit one block of `signal` to `source` through `sources`: generation `generation`, source
/// frames `block * QUANTUM..`.
fn submit(
    sources: &mut SourceControlSet,
    source: &str,
    signal: &[Vec<f32>; 2],
    generation: u64,
    block: usize,
) -> Result<(), host_core::SourceControlError> {
    let range = block * QUANTUM..(block + 1) * QUANTUM;
    sources
        .submit(
            source.as_bytes(),
            SourceSubmission {
                generation,
                start_frame: range.start as u64,
                sample_rate_hz: 48_000,
                planes: &[&signal[0][range.clone()], &signal[1][range]],
                frames: QUANTUM as u32,
                end_of_region: false,
            },
        )
        .map(|_| ())
}

/// One rendered block's bits, and the allocator calls `(allocations, reallocations,
/// deallocations)` the render made on this thread.
fn render(plan: &mut PreparedRenderPlan) -> (Vec<u32>, (u64, u64, u64)) {
    let mut output = [f32::NAN; QUANTUM * 2];
    let sample = plan.next_absolute_sample();
    let mark = bench_alloc::current_thread_counters();
    plan.render_contiguous(
        RenderIo {
            output: PlanarBufferMut::try_new(&mut output, 2, QUANTUM, QUANTUM).expect("output"),
        },
        sample,
    )
    .unwrap_or_else(|error| panic!("render: {error:?}"));
    let delta = bench_alloc::current_thread_delta_since(mark);
    (
        output.iter().map(|sample| sample.to_bits()).collect(),
        (delta.allocations, delta.reallocations, delta.deallocations),
    )
}

/// `session` prepared against `inventory` and `committed` at `backend`, with the neutral base
/// values of the amendment.
fn successor_of(
    session: &Session,
    inventory: &PlanStateInventory,
    committed: &SessionModel,
    backend: Backend,
) -> PreparedHost {
    host_core::test_only_prepare_host_runtime_with_live_controls_successor_on(
        &session.compiled,
        &caps(),
        &HostLiveControlRequest::default(),
        SuccessorBase {
            inventory,
            committed,
            revision: SUCCESSOR_REVISION,
            edit_ramps: &session::EditRamps::new(),
            monitor: host_core::MonitorBase::NONE,
            observation: &host_core::NO_OBSERVATION,
        },
        backend,
    )
    .map(|(host, _)| host)
    .unwrap_or_else(|failure| panic!("successor: {failure:?}"))
}

/// `session` prepared fresh at `backend`, with live controls: its effects bank with live lanes,
/// so an insert's state can carry.
fn fresh(session: &Session, backend: Backend) -> PreparedHost {
    host_core::test_only_prepare_host_runtime_with_live_controls_on(
        &session.compiled,
        &caps(),
        &HostLiveControlRequest::default(),
        backend,
    )
    .map(|(host, _)| host)
    .unwrap_or_else(|failure| panic!("prepare: {failure:?}"))
}

/// Publish the revision the successor's carry retargets were stamped with on its own gate, as a
/// host's commit does before the successor renders.
fn commit(host: &mut PreparedHost) {
    host.plan
        .take_revision_gate()
        .expect("the successor's gate handle")
        .publish(SUCCESSOR_REVISION);
}

/// P0 prepared and rendered [`BEFORE`] blocks with `a` and `b` fed one block ahead.
fn running_p0(p0: &Session, backend: Backend, signal: &[[Vec<f32>; 2]; 2]) -> PreparedHost {
    let mut host = fresh(p0, backend);
    for block in 0..=BEFORE {
        submit(&mut host.sources, "a", &signal[0], 1, block).expect("feed a");
        submit(&mut host.sources, "b", &signal[1], 1, block).expect("feed b");
    }
    for _ in 0..BEFORE {
        render(&mut host.plan);
    }
    host
}

/// What the host does to `c` on the withdrawn candidate's producers before B is prepared (gate
/// 4): four blocks of generation 1, then an anchored seek to generation 2 at frame 0 two blocks
/// after the swap, with that generation's first [`AFTER`] blocks.
fn feed_c(sources: &mut SourceControlSet, signal: &[[Vec<f32>; 2]; 2]) {
    for block in 0..4 {
        submit(sources, "c", &signal[0], 1, block).expect("feed c, generation 1");
    }
    sources
        .seek_at(b"c", 2, 0, SWAP_SAMPLE + 2 * QUANTUM as u64)
        .expect("anchored seek on c");
    for block in 0..AFTER {
        submit(sources, "c", &signal[1], 2, block).expect("feed c, generation 2");
    }
}

/// The twin: A' adopts P0' render-free, B' is prepared against A' and adopts it, every producer
/// following its ring; then B' renders [`AFTER`] blocks with `a` fed one block ahead.
fn twin_run(
    sessions: &[Session; 3],
    backend: Backend,
    signal: &[[Vec<f32>; 2]; 2],
    c_signal: Option<&[[Vec<f32>; 2]; 2]>,
) -> Vec<Vec<u32>> {
    let [p0, a, b] = sessions;
    let mut p0_twin = running_p0(p0, backend, signal);
    let mut a_twin = successor_of(a, &p0_twin.inventory, &p0.model, backend);
    commit(&mut a_twin);
    a_twin.sources.adopt_persisting(&mut p0_twin.sources);
    assert_eq!(
        a_twin.plan.adopt_predecessor_plan(&mut p0_twin.plan),
        CarryOutcome::Carried
    );
    if let Some(c_signal) = c_signal {
        feed_c(&mut a_twin.sources, c_signal);
    }
    let mut b_twin = successor_of(b, &a_twin.inventory, &a.model, backend);
    commit(&mut b_twin);
    b_twin.sources.adopt_persisting(&mut a_twin.sources);
    assert_eq!(
        b_twin.plan.adopt_predecessor_plan(&mut a_twin.plan),
        CarryOutcome::Carried
    );
    play(&mut b_twin, signal)
}

/// Render [`AFTER`] blocks of `host`, feeding `a` one block ahead; every block allocates and
/// frees nothing.
fn play(host: &mut PreparedHost, signal: &[[Vec<f32>; 2]; 2]) -> Vec<Vec<u32>> {
    bench_alloc::assert_installed();
    (0..AFTER)
        .map(|block| {
            submit(&mut host.sources, "a", &signal[0], 1, BEFORE + block + 1).expect("feed a");
            let (bits, allocator) = render(&mut host.plan);
            assert_eq!(allocator, (0, 0, 0), "block {block}: allocator calls");
            bits
        })
        .collect()
}

fn sessions() -> [Session; 3] {
    [
        Session::compile(p0_model()),
        Session::compile(a_model()),
        Session::compile(b_model()),
    ]
}

fn signal() -> [[Vec<f32>; 2]; 2] {
    let frames = (BEFORE + AFTER + 2) * QUANTUM;
    [planes(1, frames), planes(2, frames)]
}

/// Gate 1 at one width.
fn base_across_a_withdrawn_plan(backend: Backend) {
    let sessions = sessions();
    let [p0, a, b] = &sessions;
    let signal = signal();
    let twin = twin_run(&sessions, backend, &signal, None);

    let mut running = running_p0(p0, backend, &signal);
    // A: prepared against P0, committed and taken back; render never adopts it.
    let withdrawn = successor_of(a, &running.inventory, &p0.model, backend);
    let base = withdrawn.carried_base.as_ref().expect("A has a base");
    let mut replacement = successor_of(b, base, &p0.model, backend);
    commit(&mut replacement);
    // `a` carries from P0: its producer moves, and its ring moves at adoption.
    assert_eq!(
        replacement.sources.adopt_persisting(&mut running.sources),
        1
    );
    let a_index = running.inventory.source("a").expect("P0 has a").index;
    assert_eq!(
        graph::installed_carry_reads(&replacement.plan)
            .expect("B has a hand-over")
            .sources,
        vec![a_index],
        "B carries a's ring from P0"
    );
    assert!(replacement.inventory.source("b").is_none(), "b is absent");
    assert_eq!(
        replacement.restarted_strips(),
        [Box::<str>::from("t1"), Box::from("t2")],
        "t1 and t2 restart"
    );
    assert_eq!(
        replacement.plan.adopt_predecessor_plan(&mut running.plan),
        CarryOutcome::Carried
    );
    let blocks = play(&mut replacement, &signal);
    for (block, (left, right)) in blocks.iter().zip(&twin).enumerate() {
        assert_eq!(left, right, "{backend:?} block {block}: B equals B'");
    }

    // Not vacuous: B prepared with no base starts `a`, `t3` and every carried owner fresh.
    let mut running = running_p0(p0, backend, &signal);
    let mut fresh = self::fresh(b, backend);
    for block in 0..=BEFORE {
        submit(&mut fresh.sources, "a", &signal[0], 1, block).expect("feed a");
    }
    assert_eq!(
        fresh.plan.adopt_predecessor_plan(&mut running.plan),
        CarryOutcome::NotRequested
    );
    let fresh_blocks = play(&mut fresh, &signal);
    assert_ne!(fresh_blocks, twin, "{backend:?}: carried state is audible");
}

#[test]
fn a_successor_prepared_across_a_withdrawn_plan_renders_as_its_twin() {
    for backend in backends() {
        base_across_a_withdrawn_plan(backend);
    }
}

/// Gate 2's session: three sources (`a`, `b`, `c`), sends into a submix (live routes), a true-peak limiter on
/// `t0001`, whose latency the other paths compensate (compensation lines), strip delays on `t` and
/// `t0002`, a compressor on `t`, `t0000` and `t0001` (effect lanes and observation taps), a console
/// slot on every strip, meters, and a spectrum capture on `t0000`. Track `t` and its source `a`
/// sort first, so a plan without them has other keys for every row after them.
fn rich_model() -> SessionModel {
    let template = parse_session_json(FIXTURE).expect("fixture parses");
    let mut model = template.clone();
    model.tracks.clear();
    model.routes.clear();
    model.sources = vec![
        source(&template.sources[0], "a", 48_000),
        source(&template.sources[0], "b", 48_000),
        source(&template.sources[0], "c", 48_000),
    ];
    for (track, source) in [
        ("t", "a"),
        ("t0000", "b"),
        ("t0001", "b"),
        ("t0002", "c"),
        ("t0003", "b"),
    ] {
        add_track(&mut model, &template, track, source, true);
    }
    for track in &mut model.tracks {
        if track.id.as_str() == "t0001" {
            track.inserts.effects.push(Effect {
                id: id("limiter"),
                identity: EffectIdentity::Native {
                    effect_id: id("miso.true-peak-limiter"),
                },
                ..compressor("limiter")
            });
        }
        if matches!(track.id.as_str(), "t0002" | "t0003") {
            track.inserts.effects.clear();
        }
        if matches!(track.id.as_str(), "t" | "t0002") {
            track.builtins.left.delay_samples = 64;
            track.builtins.right.delay_samples = 32;
        }
    }
    let bus = id("bus");
    model
        .submixes
        .push(Submix::unity(bus.clone(), &model.console));
    for track in ["t", "t0000", "t0001", "t0003"] {
        let mut send = model.routes[0].clone();
        send.id = id(&format!("{track}-bus"));
        send.source = RouteSource::Track {
            track_id: id(track),
            tap: SendTap::PreFader,
        };
        send.destination = RouteDestination::SubmixInput {
            submix_id: bus.clone(),
        };
        model.routes.push(send);
    }
    let mut out = model.routes[0].clone();
    out.id = id("bus-main");
    out.source = RouteSource::Submix {
        submix_id: bus,
        tap: SendTap::PostPan,
    };
    model.routes.push(out);
    // A fader of its own on `t0003`, so the strips' values are not all equal.
    model.tracks[4].fader = DualMonoFader {
        left_db: -6.0,
        right_db: -6.0,
        left_mute: false,
        right_mute: false,
    };
    model
}

/// Gate 2's A: P0 without track `t` (and its routes) and source `a`, which sort first, so every
/// family loses `t`'s (or `a`'s) row; and a restart where a session edit restarts an owner that
/// stays: `c` redeclared (sources) and `t0000`'s insert replaced (its effect lane and observation
/// taps). Under live input lanes an input-section or strip-stage value edit carries and
/// retargets, so those families restart only by removal; the plan's one spectrum capture carries,
/// since restarting it would leave the family unread.
fn rich_a_model() -> SessionModel {
    let mut model = rich_model();
    remove_track(&mut model, "t");
    model.routes.retain(|route| route.id.as_str() != "t-bus");
    model.sources.retain(|source| source.id.as_str() != "a");
    for source in &mut model.sources {
        if source.id.as_str() == "c" {
            source.frames = 2 * 48_000;
        }
    }
    replace_insert(&mut model, "t0000");
    model
}

fn live_request() -> HostLiveControlRequest {
    HostLiveControlRequest {
        meter_period_frames: core::num::NonZeroU32::new(QUANTUM as u32),
        observation_taps: 1,
        ..HostLiveControlRequest::default()
    }
}

fn spectrum_request() -> SpectrumCaptureRequest {
    let target = SpectrumTarget::TrackPostMatrix("t0000".into());
    SpectrumCaptureRequest {
        maximum_capture_bytes: spectrum_capture_resources_for(&target).retained_bytes,
        target,
        channels: SpectrumChannels::Stereo,
    }
}

fn prepare_rich(session: &Session, base: Option<SuccessorBase<'_>>) -> PreparedHost {
    let live = live_request();
    let superseded = LiveValuesSuperseded::default();
    let spectrum = spectrum_request();
    match base {
        None => prepare_host_runtime_with_live_controls_and_spectrum(
            &session.compiled,
            &caps(),
            &live,
            &spectrum,
            HostLiveLanes::ALL,
            &superseded,
        ),
        Some(base) => prepare_host_runtime_with_live_controls_and_spectrum_successor(
            &session.compiled,
            &caps(),
            &live,
            &spectrum,
            HostLiveLanes::ALL,
            &superseded,
            base,
        ),
    }
    .map(|(host, _, _)| host)
    .unwrap_or_else(|failure| panic!("prepare: {failure:?}"))
}

/// `(family, owner)` of every row of `rows`.
fn owners(rows: &BTreeSet<(&'static str, String, String)>) -> BTreeSet<(&'static str, String)> {
    rows.iter()
        .map(|(family, owner, _)| (*family, owner.clone()))
        .collect()
}

/// `(family, key)` of every row of `rows`.
fn keys(rows: &BTreeSet<(&'static str, String, String)>) -> BTreeSet<(&'static str, String)> {
    rows.iter()
        .map(|(family, _, key)| (*family, key.clone()))
        .collect()
}

/// Gate 2, tightened by the batch 15a fold (M3). Red if the restriction keeps a row the hand-over
/// does not read (an unfiltered `restricted_to`: the removed `t`, `a` and every restarted
/// owner would stay), or if `installed_carry_reads` reports successor keys for predecessor keys
/// (A lacks `t` and `a`, which sort first, so A's keys name other P0 rows: the owners differ).
#[test]
fn the_carried_base_holds_exactly_the_rows_the_hand_over_reads() {
    let p0 = Session::compile(rich_model());
    let a = Session::compile(rich_a_model());
    let running = prepare_rich(&p0, None);
    assert!(
        running.carried_base.is_none(),
        "a plan with no base has none"
    );
    assert_eq!(running.report.carried_base_retained_bytes, 0);
    let withdrawn = prepare_rich(
        &a,
        Some(SuccessorBase {
            inventory: &running.inventory,
            committed: &p0.model,
            revision: SUCCESSOR_REVISION,
            edit_ramps: &session::EditRamps::new(),
            monitor: host_core::MonitorBase::NONE,
            observation: &host_core::NO_OBSERVATION,
        }),
    );
    let base = withdrawn.carried_base.as_ref().expect("A has a base");
    assert_eq!(base.plan_identity(), running.inventory.plan_identity());
    assert_eq!(
        withdrawn.report.carried_base_retained_bytes,
        base.retained_bytes()
    );
    assert!(base.spectrum_pairing().is_empty());
    let reads = graph::installed_carry_reads(&withdrawn.plan).expect("A has a hand-over");
    assert_eq!(reads.predecessor, running.inventory.plan_identity());

    let p0_rows = test_only_inventory_rows(&running.inventory);
    let a_rows = test_only_inventory_rows(&withdrawn.inventory);
    let base_rows = test_only_inventory_rows(base);
    let read_keys = test_only_carry_read_keys(&reads);
    // Every family is exercised by the hand-over.
    for family in [
        "sources",
        "input sections",
        "strip stages",
        "effect lanes",
        "meters",
        "observations",
        "spectrum",
        "delay lines",
        "strip delays",
        "live routes",
    ] {
        assert!(
            read_keys.iter().any(|(read, _)| *read == family),
            "{family}: the session exercises the family"
        );
    }
    // Every base row is P0's row, unchanged.
    assert!(base_rows.is_subset(&p0_rows), "base rows are P0 rows");
    // The base holds exactly the keys the hand-over reads: no more, no fewer.
    assert_eq!(keys(&base_rows), read_keys, "base keys equal read keys");
    // Not vacuous: A's own keys differ from P0's, so a successor-key read names other rows.
    assert_ne!(keys(&a_rows), keys(&p0_rows), "A's keys are not P0's");
    // The owners: those both plans hold, less those A restarted. Independent of keys.
    let restarted = |(family, owner): &(&'static str, String)| match *family {
        "sources" => owner == "c",
        "effect lanes" | "observations" => owner.starts_with("t0000 ") && owner.ends_with(" comp"),
        _ => false,
    };
    let expected: BTreeSet<_> = owners(&p0_rows)
        .intersection(&owners(&a_rows))
        .filter(|owner| !restarted(owner))
        .cloned()
        .collect();
    assert_eq!(
        owners(&base_rows),
        expected,
        "base owners are A's carried owners"
    );
    for owner in owners(&p0_rows) {
        if restarted(&owner) || !owners(&a_rows).contains(&owner) {
            assert!(
                !owners(&base_rows).contains(&owner),
                "{owner:?}: removed or restarted, so not in the base"
            );
        }
    }
}

/// Gate 3: P0 lacks `d`; A adds it with 4 seconds of frames, B with 5.
#[test]
fn a_source_absent_from_the_running_plan_is_never_carried_from_the_withdrawn_one() {
    let template = template();
    let with_d = |frames: u64| {
        let mut model = a_model();
        model
            .sources
            .push(source(&template.sources[0], "d", frames));
        add_track(&mut model, &template, "td", "d", false);
        model
    };
    let p0 = Session::compile(p0_model());
    let a = Session::compile(with_d(4 * 48_000));
    let b = Session::compile(with_d(5 * 48_000));
    let backend = Backend::current();
    let signal = signal();
    let running = running_p0(&p0, backend, &signal);
    let mut withdrawn = successor_of(&a, &running.inventory, &p0.model, backend);
    // The host fed A's `d` and was acked.
    for block in 0..2 {
        submit(&mut withdrawn.sources, "d", &signal[0], 1, block).expect("feed A's d");
    }
    let base = withdrawn.carried_base.as_ref().expect("A has a base");
    assert!(
        base.source("d").is_none(),
        "P0 has no d, so A's base has none"
    );
    let mut replacement = successor_of(&b, base, &p0.model, backend);
    let d_index = replacement.inventory.source("d").expect("B has d").index;
    assert_eq!(
        graph::installed_carry_reads(&replacement.plan)
            .expect("B carries a")
            .sources,
        vec![running.inventory.source("a").expect("P0 has a").index],
        "B carries a from P0 and nothing for d"
    );
    assert!(
        replacement.plan.can_prepare_source_seek(d_index as usize),
        "B's d owns a ring"
    );
    // No donation applies: d was redeclared.
    let (into, from) = sides(&replacement, &b.model, &withdrawn, &a.model);
    let plan = check_donation(into, from).expect("the check passes");
    // The check has no side effect: run again, it finds the same.
    assert_eq!(Ok(&plan), check_donation(into, from).as_ref());
    // Only `c` (unchanged from A) is donated; `d` is not.
    assert_eq!(plan.len(), 1);
    // B's `d` is a fresh ring at generation 1, frame 0: A's has frames 0..256 and would refuse a
    // chunk that does not continue them.
    submit(&mut replacement.sources, "d", &signal[1], 1, 0).expect("B's d starts at frame 0");
    assert!(
        submit(&mut withdrawn.sources, "d", &signal[1], 1, 0).is_err(),
        "A's d continues where the host left it"
    );
}

/// Gate 4 at one width: `c` donated from the withdrawn A to B plays as B' plays it.
fn donation_plays_the_fed_ring(backend: Backend) {
    let sessions = sessions();
    let [p0, a, b] = &sessions;
    let signal = signal();
    let c_signal = [
        planes(3, (AFTER + 4) * QUANTUM),
        planes(4, (AFTER + 4) * QUANTUM),
    ];
    let twin = twin_run(&sessions, backend, &signal, Some(&c_signal));

    let run = |donate: bool| {
        let mut running = running_p0(p0, backend, &signal);
        let mut withdrawn = successor_of(a, &running.inventory, &p0.model, backend);
        feed_c(&mut withdrawn.sources, &c_signal);
        let base = withdrawn.carried_base.clone().expect("A has a base");
        let mut replacement = successor_of(b, &base, &p0.model, backend);
        commit(&mut replacement);
        if donate {
            let (into, from) = sides(&replacement, &b.model, &withdrawn, &a.model);
            let plan = check_donation(into, from).expect("the donation checks");
            assert_eq!(plan.len(), 1, "c is the one source both created fresh");
            assert_eq!(
                apply_donation(
                    DonationSideMut {
                        plan: &mut replacement.plan,
                        sources: &mut replacement.sources,
                    },
                    DonationSideMut {
                        plan: &mut withdrawn.plan,
                        sources: &mut withdrawn.sources,
                    },
                    plan,
                ),
                1
            );
            // The producer moved with its ring: the host continues generation 2 on B.
            submit(&mut replacement.sources, "c", &c_signal[1], 2, AFTER)
                .expect("B's c continues generation 2");
            // A's entry is vacant: a later submit to the donor is refused, never acked into
            // the fresh ring A now holds and drops (#1344 fold m1).
            assert_eq!(
                submit(&mut withdrawn.sources, "c", &c_signal[1], 2, AFTER),
                Err(host_core::SourceControlError::Vacated),
                "the donor's c is vacant"
            );
        }
        replacement.sources.adopt_persisting(&mut running.sources);
        assert_eq!(
            replacement.plan.adopt_predecessor_plan(&mut running.plan),
            CarryOutcome::Carried
        );
        play(&mut replacement, &signal)
    };
    let donated = run(true);
    for (block, (left, right)) in donated.iter().zip(&twin).enumerate() {
        assert_eq!(
            left, right,
            "{backend:?} block {block}: B plays c as B' does"
        );
    }
    assert_ne!(
        run(false),
        twin,
        "{backend:?}: without the donation, c's acked PCM and seek are not heard"
    );
}

#[test]
fn a_donated_ring_plays_its_acked_pcm_and_held_seek() {
    for backend in backends() {
        donation_plays_the_fed_ring(backend);
    }
}

/// The two sides of a donation from `withdrawn` (compiled from `donor_model`) to `replacement`
/// (compiled from `model`).
fn sides<'a>(
    replacement: &'a PreparedHost,
    model: &'a SessionModel,
    withdrawn: &'a PreparedHost,
    donor_model: &'a SessionModel,
) -> (DonationSide<'a>, DonationSide<'a>) {
    (
        DonationSide {
            plan: &replacement.plan,
            sources: &replacement.sources,
            inventory: &replacement.inventory,
            model,
        },
        DonationSide {
            plan: &withdrawn.plan,
            sources: &withdrawn.sources,
            inventory: &withdrawn.inventory,
            model: donor_model,
        },
    )
}

/// What a refusal must leave alone: which of B's and A's `c` entries own a ring, and the
/// producers, read by feeding each set the chunk only its own ring accepts.
fn assert_untouched(replacement: &mut PreparedHost, withdrawn: &mut PreparedHost, what: &str) {
    for (side, host) in [("B", &*replacement), ("A", &*withdrawn)] {
        let index = host.inventory.source("c").expect("c").index;
        assert!(
            host.plan.can_prepare_source_seek(index as usize),
            "{what}: {side}'s c owns its ring"
        );
    }
    let c_signal = planes(5, 8 * QUANTUM);
    submit(&mut replacement.sources, "c", &c_signal, 1, 0)
        .unwrap_or_else(|error| panic!("{what}: B keeps its fresh producer: {error:?}"));
    submit(&mut withdrawn.sources, "c", &c_signal, 1, 4)
        .unwrap_or_else(|error| panic!("{what}: A keeps its fed producer: {error:?}"));
}

#[test]
fn a_refused_or_mismatched_donation_moves_nothing() {
    let backend = Backend::current();
    let [p0, a, b] = &sessions();
    let signal = signal();
    let feed_four = |sources: &mut SourceControlSet| {
        for block in 0..4 {
            submit(sources, "c", &signal[0], 1, block).expect("feed c");
        }
    };

    // Consumed: a control-thread render of A began its `c` consumer.
    let running = running_p0(p0, backend, &signal);
    let mut withdrawn = successor_of(a, &running.inventory, &p0.model, backend);
    feed_four(&mut withdrawn.sources);
    render(&mut withdrawn.plan);
    let base = withdrawn.carried_base.clone().expect("A has a base");
    let mut replacement = successor_of(b, &base, &p0.model, backend);
    let (into, from) = sides(&replacement, &b.model, &withdrawn, &a.model);
    assert_eq!(check_donation(into, from), Err(DonationError::Consumed));
    assert_untouched(&mut replacement, &mut withdrawn, "consumed");

    // A redeclared `c` (other content), and a `c` on a ring of another size: not donated.
    let mut redeclared = b_model();
    let c = redeclared
        .sources
        .iter_mut()
        .find(|source| source.id.as_str() == "c")
        .expect("c");
    c.content = c.content.replace("0c", "0e");
    let redeclared = Session::compile(redeclared);
    let mut small = caps();
    small.source_ring_frames /= 2;
    for (what, session, caps) in [
        ("declaration", &redeclared, caps()),
        ("ring configuration", b, small),
    ] {
        let running = running_p0(p0, backend, &signal);
        let mut withdrawn = successor_of(a, &running.inventory, &p0.model, backend);
        feed_four(&mut withdrawn.sources);
        let base = withdrawn.carried_base.clone().expect("A has a base");
        let mut replacement = session
            .prepare_successor(&base, &p0.model, &caps, backend)
            .unwrap_or_else(|failure| panic!("{what}: successor: {failure:?}"));
        let (into, from) = sides(&replacement, &session.model, &withdrawn, &a.model);
        let plan = check_donation(into, from).expect("nothing to refuse");
        assert!(plan.is_empty(), "{what}: c is not donated");
        assert_eq!(
            apply_donation(
                DonationSideMut {
                    plan: &mut replacement.plan,
                    sources: &mut replacement.sources,
                },
                DonationSideMut {
                    plan: &mut withdrawn.plan,
                    sources: &mut withdrawn.sources,
                },
                plan,
            ),
            0
        );
        assert_untouched(&mut replacement, &mut withdrawn, what);
    }
}

/// Fold m1 (#1344, batch 15a). Red on a check that passes a fed successor producer the apply
/// then drops, and on an apply that moves the producers before a consumer swap that fails (the
/// donor's fed producer would leave its ring). The swap the producer half used to make (the donor
/// kept B's fresh producer and acked into the ring it then dropped) is caught by gate 4 instead:
/// its assert that the donor's `c` is `Vacated` after the donation
/// (`a_donated_ring_plays_its_acked_pcm_and_held_seek`).
#[test]
fn a_donation_never_separates_an_acked_ring_from_its_producer() {
    let backend = Backend::current();
    let [p0, a, b] = &sessions();
    let signal = signal();
    let prepare = || {
        let running = running_p0(p0, backend, &signal);
        let mut withdrawn = successor_of(a, &running.inventory, &p0.model, backend);
        for block in 0..4 {
            submit(&mut withdrawn.sources, "c", &signal[0], 1, block).expect("feed A's c");
        }
        let base = withdrawn.carried_base.clone().expect("A has a base");
        let replacement = successor_of(b, &base, &p0.model, backend);
        (withdrawn, replacement)
    };

    // A successor whose fresh `c` producer was fed is refused: the apply would drop it.
    let (mut withdrawn, mut replacement) = prepare();
    submit(&mut replacement.sources, "c", &signal[1], 1, 0).expect("feed B's c by hand");
    let (into, from) = sides(&replacement, &b.model, &withdrawn, &a.model);
    assert_eq!(check_donation(into, from), Err(DonationError::SuccessorFed));
    submit(&mut replacement.sources, "c", &signal[1], 1, 1).expect("B keeps its producer");
    submit(&mut withdrawn.sources, "c", &signal[0], 1, 4).expect("A keeps its producer");

    // A contract violation at apply: the checked plan is applied with a donor plan whose `c` is
    // vacant (B' prepared against A carries it), so the consumer half cannot move. Nothing moves:
    // A's fed producer stays with A's ring, and B keeps its fresh producer.
    let (mut withdrawn, mut replacement) = prepare();
    let (into, from) = sides(&replacement, &b.model, &withdrawn, &a.model);
    let plan = check_donation(into, from).expect("the donation checks");
    assert_eq!(plan.len(), 1);
    let mut vacant_c = successor_of(b, &withdrawn.inventory, &a.model, backend);
    assert!(
        !vacant_c
            .plan
            .can_prepare_source_seek(vacant_c.inventory.source("c").expect("c").index as usize),
        "the stand-in donor's c is vacant"
    );
    assert_eq!(
        apply_donation(
            DonationSideMut {
                plan: &mut replacement.plan,
                sources: &mut replacement.sources,
            },
            DonationSideMut {
                plan: &mut vacant_c.plan,
                sources: &mut withdrawn.sources,
            },
            plan,
        ),
        0,
        "a failing half moves nothing"
    );
    assert_untouched(
        &mut replacement,
        &mut withdrawn,
        "apply on another donor plan",
    );
}

/// Two fresh sides of gate 4's donation: A fed four blocks of `c`, B prepared across it.
fn fed_withdrawn_and_replacement(
    sessions: &[Session; 3],
    backend: Backend,
    signal: &[[Vec<f32>; 2]; 2],
) -> (PreparedHost, PreparedHost) {
    let [p0, a, b] = sessions;
    let running = running_p0(p0, backend, signal);
    let mut withdrawn = successor_of(a, &running.inventory, &p0.model, backend);
    for block in 0..4 {
        submit(&mut withdrawn.sources, "c", &signal[0], 1, block).expect("feed A's c");
    }
    let base = withdrawn.carried_base.clone().expect("A has a base");
    let replacement = successor_of(b, &base, &p0.model, backend);
    (withdrawn, replacement)
}

/// Batch 15a review fold 2, m-3: a successor producer that acked anything is fed, not only one
/// that acked PCM. A seek alone (the producer's generation moves off 1) and a seek to the region's
/// end closed by a zero-frame end-of-region chunk each make the check refuse with `SuccessorFed`.
///
/// Test value: red if `unfed` reads only the written frames (either case would be donated and
/// its producer dropped with what it acked).
#[test]
fn a_seek_or_an_empty_end_of_region_on_the_successor_refuses_the_donation() {
    let backend = Backend::current();
    let sessions = sessions();
    let [_, a, b] = &sessions;
    let signal = signal();

    let (withdrawn, mut replacement) = fed_withdrawn_and_replacement(&sessions, backend, &signal);
    replacement
        .sources
        .seek(b"c", 2, 0)
        .expect("a seek on B's c by hand");
    let (into, from) = sides(&replacement, &b.model, &withdrawn, &a.model);
    assert_eq!(
        check_donation(into, from),
        Err(DonationError::SuccessorFed),
        "seek only"
    );

    // The host seeks to the region's end and closes it with a zero-frame end-of-region chunk:
    // nothing written, and the producer has acked the end of the region. (A zero-frame
    // end-of-region chunk at generation 1 needs an empty region, which the validator refuses, so
    // the chunk only follows a seek.)
    let (withdrawn, mut replacement) = fed_withdrawn_and_replacement(&sessions, backend, &signal);
    let region = replacement.sources.region(b"c").expect("c's region");
    replacement
        .sources
        .seek(b"c", 2, region.end)
        .expect("a seek to the region's end");
    let empty: [f32; 0] = [];
    replacement
        .sources
        .submit(
            b"c",
            SourceSubmission {
                generation: 2,
                start_frame: region.end,
                sample_rate_hz: 48_000,
                planes: &[&empty, &empty],
                frames: 0,
                end_of_region: true,
            },
        )
        .expect("an empty end-of-region chunk on B's c");
    let (into, from) = sides(&replacement, &b.model, &withdrawn, &a.model);
    assert_eq!(
        check_donation(into, from),
        Err(DonationError::SuccessorFed),
        "zero-frame end of region"
    );
}

/// Batch 15a review fold 2, m-2: the host feeds B's `c` after the donation was checked and
/// before it is applied (a contract breach the apply must survive). The apply re-reads that the
/// successor's producer is unfed, refuses, and moves nothing: B keeps its fed ring and the
/// producer that acked into it (its next chunk continues from the acked one), and A keeps its
/// own (not vacated).
///
/// Test value: red if the apply's re-read does not include the successor producer's `unfed`
/// state, or if the re-read is removed: the consumer swap would then hand B's acked chunk to A
/// and drop B's fed producer (`donate_producers`' `debug_assert!` fires in a debug build).
#[test]
fn a_successor_fed_between_the_check_and_the_apply_moves_nothing() {
    let backend = Backend::current();
    let sessions = sessions();
    let [_, a, b] = &sessions;
    let signal = signal();
    let (mut withdrawn, mut replacement) =
        fed_withdrawn_and_replacement(&sessions, backend, &signal);
    let (into, from) = sides(&replacement, &b.model, &withdrawn, &a.model);
    let plan = check_donation(into, from).expect("the donation checks");
    assert_eq!(plan.len(), 1);
    submit(&mut replacement.sources, "c", &signal[1], 1, 0).expect("B's c acks a chunk");
    assert_eq!(
        apply_donation(
            DonationSideMut {
                plan: &mut replacement.plan,
                sources: &mut replacement.sources,
            },
            DonationSideMut {
                plan: &mut withdrawn.plan,
                sources: &mut withdrawn.sources,
            },
            plan,
        ),
        0,
        "a fed successor refuses the apply"
    );
    for (side, host) in [("B", &replacement), ("A", &withdrawn)] {
        let index = host.inventory.source("c").expect("c").index;
        assert!(
            host.plan.can_prepare_source_seek(index as usize),
            "{side}'s c owns its ring"
        );
    }
    submit(&mut replacement.sources, "c", &signal[1], 1, 1)
        .expect("B's producer continues after its acked chunk");
    submit(&mut withdrawn.sources, "c", &signal[0], 1, 4)
        .expect("A's producer continues after its acked chunks");
}

/// P0 with source `b` declared with `frames`: another declaration of `b`, so a plan prepared
/// against P0 restarts `b` in a fresh ring.
fn with_b_frames(frames: u64) -> SessionModel {
    let mut model = p0_model();
    model
        .sources
        .iter_mut()
        .find(|source| source.id.as_str() == "b")
        .expect("b")
        .frames = frames;
    model
}

/// `model` without source `source` and its track `track`.
fn without(mut model: SessionModel, source: &str, track: &str) -> SessionModel {
    remove_track(&mut model, track);
    model
        .sources
        .retain(|existing| existing.id.as_str() != source);
    model
}

/// Whether the acked-fresh-source guard refuses B (`b`) across the withdrawn A (`a`), A prepared
/// against the running P0. `feed` names the source the host feeds two blocks of through A's
/// producers (none when empty); `carry` hands P0's producers of the sources A carries over to A
/// first, as the control plane's commit does.
fn guard_refuses(a: SessionModel, b: SessionModel, feed: &str, carry: bool) -> bool {
    let backend = Backend::current();
    let p0 = Session::compile(p0_model());
    let a = Session::compile(a);
    let b = Session::compile(b);
    let signal = signal();
    let mut running = running_p0(&p0, backend, &signal);
    let mut withdrawn = successor_of(&a, &running.inventory, &p0.model, backend);
    if carry {
        assert!(withdrawn.sources.adopt_persisting(&mut running.sources) > 0);
    }
    if !feed.is_empty() {
        // A fresh ring starts at frame 0; a carried one continues P0's, fed through block BEFORE.
        let first = if carry && feed == "a" { BEFORE + 1 } else { 0 };
        for block in first..first + 2 {
            submit(&mut withdrawn.sources, feed, &signal[0], 1, block).expect("feed A");
        }
    }
    let base = withdrawn.carried_base.clone().expect("A has a base");
    let replacement = successor_of(&b, &base, &p0.model, backend);
    let (into, from) = sides(&replacement, &b.model, &withdrawn, &a.model);
    let plan = check_donation(into, from).expect("the donation checks");
    // Read-only: a second call finds the same.
    let refused = drops_acked_fresh_source(from, &plan);
    assert_eq!(refused, drops_acked_fresh_source(from, &plan));
    refused
}

/// Batch 15a fold 3 (#1310, root ruling M-A of 2026-10-16): the guard refuses exactly when a ring
/// A created (A added or redeclared the source) was fed and the donation does not move it to B.
/// (a)-(c): A redeclares P0's `b` and the host feeds it; B reverts `b` to P0's declaration,
/// redeclares it a third way, or removes it. (d): A adds `c` and the host feeds it; B removes or
/// redeclares it. Not refused: B keeps `c` as A declares it (donated); A's fresh `c` unfed; a
/// source A carries from P0 (`a`), fed through A's producer.
///
/// Test value: red if the guard reads only the reverted case (b-d pass), ignores the donation
/// plan (the donated `c` is refused), ignores the producer's fed state (the unfed `c` is refused)
/// or ignores freshness (the carried, fed `a` is refused).
#[test]
fn the_guard_refuses_exactly_a_fed_fresh_ring_the_donation_leaves_behind() {
    let redeclared_b = || with_b_frames(40_000);
    let c_as = |frames: u64| {
        let mut model = a_model();
        model
            .sources
            .iter_mut()
            .find(|source| source.id.as_str() == "c")
            .expect("c")
            .frames = frames;
        model
    };
    for (label, a, b, feed, carry, refused) in [
        (
            "(a) b reverted",
            redeclared_b(),
            p0_model(),
            "b",
            false,
            true,
        ),
        (
            "(b) b redeclared a third way",
            redeclared_b(),
            with_b_frames(44_000),
            "b",
            false,
            true,
        ),
        (
            "(c) b removed",
            redeclared_b(),
            without(p0_model(), "b", "tb"),
            "b",
            false,
            true,
        ),
        (
            "(d) c added, removed",
            a_model(),
            without(a_model(), "c", "tc"),
            "c",
            false,
            true,
        ),
        (
            "(d) c added, redeclared",
            a_model(),
            c_as(40_000),
            "c",
            false,
            true,
        ),
        ("c donated", a_model(), b_model(), "c", false, false),
        (
            "c fresh and unfed",
            a_model(),
            without(a_model(), "c", "tc"),
            "",
            false,
            false,
        ),
        (
            "a carried and fed",
            a_model(),
            without(a_model(), "c", "tc"),
            "a",
            true,
            false,
        ),
        (
            "b reverted, unfed",
            redeclared_b(),
            p0_model(),
            "",
            false,
            false,
        ),
    ] {
        assert_eq!(guard_refuses(a, b, feed, carry), refused, "{label}");
    }
}
