//! TBD9: the fast live-edit classifier.
//!
//! `classify_live_edit` reads a transaction of `SetControl`s from the rows it names, with no model
//! clone and no canonical JSON, and must give exactly what `classify_live_delta_with_monitor` gives
//! for the same models. This file holds the three gates:
//!
//! 1. the differential: the #1560 parity script and a seeded random set of edits, the full
//!    classifier as the oracle;
//! 2. the cost: the allocations of a live-only classification do not depend on the track count;
//! 3. one descriptive timing line (ignored: `cargo test --release -p host-core --all-features
//!    --test live_edit_classifier -- --ignored --nocapture`).
#![cfg(all(feature = "control-provider", feature = "test-support"))]

use std::collections::BTreeSet;
use std::time::Instant;

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use controls::{
    ControlAddress, ControlClass, ControlKind, ControlRow, Lane, MonitorOverlay, Motion, Owner,
    OwnerKind, Section,
};
use host_core::{
    LiveRamps, LiveRebuild, MonitorBase, classify_live_delta_with_monitor, classify_live_edit,
    take_fast_classification_count,
};
use protocol::{ExpectedRevision, PreparedSessionTransaction, SessionEdit, SessionStore};
use session::{
    CompileCaps, Console, DualMonoFader, Effect, EffectIdentity, EffectQuality, LinkMode,
    MatrixOrPan, RouteDestination, RouteSource, SendTap, SessionModel, SidechainDeclaration,
    StableId, Submix, Vca, parse_session_json,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

fn caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

fn store(model: SessionModel) -> SessionStore {
    SessionStore::new(model, caps()).expect("the session compiles")
}

fn insert(name: &str, native: &str) -> Effect {
    Effect {
        id: id(name),
        identity: EffectIdentity::Native {
            effect_id: id(native),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        params: Vec::new(),
        sidechain: SidechainDeclaration::None,
    }
}

/// `tracks` tracks over the fixture's one source, one output route each, no insert and no console
/// slot: the session #1562a timed.
fn plain_session(tracks: usize) -> SessionModel {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    model.console = Console {
        pre_insert: Vec::new(),
        post_insert: Vec::new(),
    };
    let template = model.tracks[0].clone();
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    for index in 0..tracks {
        let mut track = template.clone();
        track.id = id(&format!("t{index:04}"));
        track.console.clear();
        track.inserts.effects.clear();
        track.fader.left_db = -((index % 7) as f32);
        track.fader.right_db = -((index % 7) as f32);
        track.matrix_or_pan = MatrixOrPan::Pan {
            left: 0.0,
            right: 0.0,
            smoothing_samples: 0,
        };
        let mut route = route.clone();
        route.id = id(&format!("t{index:04}-main"));
        route.source = RouteSource::Track {
            track_id: track.id.clone(),
            tap: SendTap::PostPan,
        };
        model.tracks.push(track);
        model.routes.push(route);
    }
    model
}

/// The fixture's pre-insert console slot on every strip, an insert of a launch effect on each of
/// the first eight tracks, submix `bus` with sends from `t0000` (following its mute) and `t0001`,
/// VCA `v1` over `t0001` and `t0002` and VCA `v0` over `v1` and `t0003`. Every structure here is
/// the same for any `tracks >= 8`: only the number of plain tracks grows.
fn rich_session(tracks: usize) -> SessionModel {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    let template = model.tracks[0].clone();
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    let inserts: [(&str, &str, &str); 8] = [
        ("t0000", "eq", "miso.parametric-eq"),
        ("t0001", "comp", "miso.compressor"),
        ("t0002", "gate", "miso.gate-expander"),
        ("t0003", "delay", "miso.delay"),
        ("t0004", "multi", "miso.multiband-compressor"),
        ("t0005", "limiter", "miso.true-peak-limiter"),
        ("t0006", "clip", "miso.soft-clip"),
        ("t0007", "shaper", "miso.transient-shaper"),
    ];
    for index in 0..tracks {
        let mut track = template.clone();
        let name = format!("t{index:04}");
        track.id = id(&name);
        track.fader.left_db = -((index % 7) as f32);
        track.fader.right_db = -((index % 5) as f32);
        track.matrix_or_pan = if index % 3 == 0 {
            MatrixOrPan::Matrix {
                ll: 0.9,
                lr: 0.1,
                rl: 0.2,
                rr: 0.8,
                smoothing_samples: 0,
            }
        } else {
            MatrixOrPan::Pan {
                left: -0.5,
                right: 0.5,
                smoothing_samples: if index % 2 == 0 { 0 } else { 200 },
            }
        };
        track.inserts.effects = inserts
            .iter()
            .filter(|(owner, ..)| *owner == name)
            .map(|(_, effect, native)| insert(effect, native))
            .collect();
        let mut route = route.clone();
        route.id = id(&format!("{name}-main"));
        route.source = RouteSource::Track {
            track_id: track.id.clone(),
            tap: SendTap::PostPan,
        };
        model.tracks.push(track);
        model.routes.push(route);
    }
    let bus = id("bus");
    model
        .submixes
        .push(Submix::unity(bus.clone(), &model.console));
    for (track, follows_mute) in [("t0000", true), ("t0001", false)] {
        let mut send = model.routes[0].clone();
        send.id = id(&format!("{track}-bus"));
        send.source = RouteSource::Track {
            track_id: id(track),
            tap: SendTap::PreFader,
        };
        send.destination = RouteDestination::SubmixInput {
            submix_id: bus.clone(),
        };
        send.follows_mute = follows_mute;
        model.routes.push(send);
    }
    let mut out = model.routes[0].clone();
    out.id = id("bus-main");
    out.source = RouteSource::Submix {
        submix_id: bus,
        tap: SendTap::PostPan,
    };
    model.routes.push(out);
    let vca = |name: &str, members: &[&str]| Vca {
        id: id(name),
        fader: DualMonoFader {
            left_db: 0.0,
            right_db: 0.0,
            left_mute: false,
            right_mute: false,
        },
        members: members.iter().map(|member| id(member)).collect(),
    };
    model.vcas.push(vca("v1", &["t0001", "t0002"]));
    model.vcas.push(vca("v0", &["v1", "t0003"]));
    model
}

fn address(owner: Owner, section: Section, control: u32, lane: Lane) -> ControlAddress {
    ControlAddress {
        owner,
        section,
        control,
        lane,
    }
}

fn strip_row(strip: &str, control: u32, lane: Lane) -> ControlAddress {
    address(Owner::Strip(id(strip)), Section::Builtins, control, lane)
}

fn own_row(strip: &str, control: u32) -> ControlAddress {
    address(Owner::Strip(id(strip)), Section::Own, control, Lane::Both)
}

const FADER_DB: u32 = 5;
const MUTE: u32 = 6;
const PAN: u32 = 12;
const TRIM_DB: u32 = 2;
const SOLO: u32 = 2;

fn set(address: ControlAddress, value: f32) -> SessionEdit {
    SessionEdit::set_control(address, value, None)
}

/// `edits` prepared on `store` as the control plane prepares them, with the overlay and the ramp
/// the transaction leaves after `committed`. `None` when the protocol refuses the transaction.
fn prepare(
    store: &SessionStore,
    edits: &[SessionEdit],
    committed: &MonitorOverlay,
) -> Option<(PreparedSessionTransaction, MonitorOverlay, Option<u32>)> {
    let prepared = store
        .prepare_transaction_with_monitor(ExpectedRevision::Exact(store.revision()), edits)
        .ok()?;
    let change = committed.after(
        prepared.monitor_edits(),
        prepared.compiled().normalized_model(),
    );
    Some((prepared, change.overlay, change.ramp))
}

/// What a prepared transaction classified to: whether the fast path answered, and the verdict.
struct Verdict {
    fast_path: bool,
    result: Result<usize, LiveRebuild>,
}

/// Classify a prepared transaction with the full classifier and with `classify_live_edit`, and
/// assert that they agree on the whole delta or the same refusal.
fn judge(
    store: &SessionStore,
    prepared: &PreparedSessionTransaction,
    committed: &MonitorOverlay,
    overlay: &MonitorOverlay,
    ramp: Option<u32>,
) -> Verdict {
    let current = store.compiled().normalized_model();
    let next = prepared.compiled().normalized_model();
    let monitor = MonitorBase {
        committed,
        next: overlay,
        ramp,
    };
    let ramps = LiveRamps::for_session(next);
    let full =
        classify_live_delta_with_monitor(current, next, ramps, prepared.edit_ramps(), monitor);
    let _ = take_fast_classification_count();
    let fast = classify_live_edit(
        current,
        next,
        ramps,
        prepared.edit_ramps(),
        monitor,
        prepared.set_control_rows(),
    );
    let fast_path = take_fast_classification_count() == 1;
    assert_eq!(fast, full, "the fast classifier equals the full one");
    Verdict {
        fast_path,
        result: full.map(|delta| delta.strips.len() + delta.effects.len() + delta.routes.len()),
    }
}

// ---------------------------------------------------------------------------------------------
// Gate 1a: the #1560 parity script
// ---------------------------------------------------------------------------------------------

/// The parity session of `hosts/host-web/tests/common/mod.rs` (`model_at`): tracks `t1` (pan with a
/// window, an EQ and a compressor), `t2` (pan, a gate, a clipper and a transient shaper), `t3`
/// (matrix, a multiband compressor, a limiter and a delay), submix `b`, `t1` sending pre-fader into
/// `b` with `follows_mute`, and VCA `v` holding `t2`.
fn parity_session(rate: u32) -> SessionModel {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    model.sample_rate_hz = rate;
    model.console = Console {
        pre_insert: Vec::new(),
        post_insert: Vec::new(),
    };
    let template = model.tracks[0].clone();
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    type Inserts<'a> = &'a [(&'a str, &'a str)];
    let tracks: [(&str, Inserts); 3] = [
        (
            "t1",
            &[("eq", "miso.parametric-eq"), ("comp", "miso.compressor")],
        ),
        (
            "t2",
            &[
                ("gate", "miso.gate-expander"),
                ("clip", "miso.soft-clip"),
                ("shaper", "miso.transient-shaper"),
            ],
        ),
        (
            "t3",
            &[
                ("multi", "miso.multiband-compressor"),
                ("limiter", "miso.true-peak-limiter"),
                ("delay", "miso.delay"),
            ],
        ),
    ];
    for (index, (name, inserts)) in tracks.into_iter().enumerate() {
        let mut track = template.clone();
        track.id = id(name);
        track.console.clear();
        track.fader.left_db = -2.0 - index as f32;
        track.fader.right_db = -2.0 - index as f32;
        track.matrix_or_pan = match name {
            "t1" => MatrixOrPan::Pan {
                left: -0.8,
                right: 0.6,
                smoothing_samples: 200,
            },
            "t2" => MatrixOrPan::Pan {
                left: -0.5,
                right: 0.4,
                smoothing_samples: 0,
            },
            _ => MatrixOrPan::Matrix {
                ll: 0.9,
                lr: 0.1,
                rl: 0.2,
                rr: 0.8,
                smoothing_samples: 0,
            },
        };
        track.inserts.effects = inserts
            .iter()
            .map(|(effect, native)| insert(effect, native))
            .collect();
        let mut route = route.clone();
        route.id = id(&format!("{name}-main"));
        route.source = RouteSource::Track {
            track_id: track.id.clone(),
            tap: SendTap::PostPan,
        };
        model.tracks.push(track);
        model.routes.push(route);
    }
    let bus = id("b");
    model
        .submixes
        .push(Submix::unity(bus.clone(), &model.console));
    let mut send = model.routes[0].clone();
    send.id = id("t1-b");
    send.source = RouteSource::Track {
        track_id: id("t1"),
        tap: SendTap::PreFader,
    };
    send.destination = RouteDestination::SubmixInput {
        submix_id: bus.clone(),
    };
    send.follows_mute = true;
    let mut out = model.routes[0].clone();
    out.id = id("b-main");
    out.source = RouteSource::Submix {
        submix_id: bus,
        tap: SendTap::PostPan,
    };
    model.routes.push(send);
    model.routes.push(out);
    model.vcas.push(Vca {
        id: id("v"),
        fader: DualMonoFader {
            left_db: 0.0,
            right_db: 0.0,
            left_mute: false,
            right_mute: false,
        },
        members: vec![id("t2")],
    });
    model
}

/// One scripted edit of `control_parity.rs`'s `script()`, as the `SET_CONTROL` edits both hosts
/// send.
struct Scripted {
    edits: Vec<SessionEdit>,
    refused: bool,
}

fn scripted() -> Vec<Scripted> {
    let t = |name: &str| Owner::Strip(id(name));
    let insert_row = |track: &str, insert: &str, control: u32, lane: Lane| {
        address(t(track), Section::Inserts(id(insert)), control, lane)
    };
    let route = |control: u32| address(Owner::Route(id("t1-b")), Section::Own, control, Lane::Both);
    let vca = |control: u32, lane: Lane| address(Owner::Vca(id("v")), Section::Own, control, lane);
    let ok = |edits: Vec<SessionEdit>| Scripted {
        edits,
        refused: false,
    };
    let ramped = |edits: Vec<SessionEdit>, ramp: u32| Scripted {
        edits: edits
            .into_iter()
            .map(|edit| match edit {
                SessionEdit::SetControl { address, value, .. } => {
                    SessionEdit::set_control(address, value, Some(ramp))
                }
                other => other,
            })
            .collect(),
        refused: false,
    };
    let fader = |strip: &str, lane: Lane, db: f32| vec![set(strip_row(strip, FADER_DB, lane), db)];
    let mute = |strip: &str, lane: Lane, on: bool| {
        vec![set(strip_row(strip, MUTE, lane), f32::from(u8::from(on)))]
    };
    let pan = |strip: &str, left: f32, right: f32| {
        vec![
            set(strip_row(strip, PAN, Lane::Left), left),
            set(strip_row(strip, PAN, Lane::Right), right),
        ]
    };
    let matrix = |strip: &str, values: [f32; 4]| {
        values
            .into_iter()
            .zip(7..)
            .map(|(value, control)| set(strip_row(strip, control, Lane::Both), value))
            .collect::<Vec<_>>()
    };
    let parameter = |track: &str, effect: &str, parameter: u32, lane: Lane, value: f32| {
        vec![set(insert_row(track, effect, parameter, lane), value)]
    };
    let send_matrix = |values: [f32; 4]| {
        values
            .into_iter()
            .zip(3..)
            .map(|(value, control)| set(route(control), value))
            .collect::<Vec<_>>()
    };
    vec![
        ok(fader("t1", Lane::Both, -6.0)),
        ok(fader("t1", Lane::Both, -9.0)),
        ok(fader("t2", Lane::Left, -10.0)),
        ok(mute("t3", Lane::Both, true)),
        ramped(fader("b", Lane::Both, -4.0), 480),
        ramped(fader("t1", Lane::Both, -3.0), 0),
        ok(pan("t2", -0.2, 0.7)),
        ok(pan("t1", 0.1, -0.3)),
        ok(pan("t1", 0.4, -0.1)),
        ok(matrix("t3", [0.7, 0.3, -0.2, 0.9])),
        ok(vec![set(strip_row("t1", TRIM_DB, Lane::Both), -5.0)]),
        ok(vec![set(strip_row("t2", 1, Lane::Right), 1.0)]),
        ok(vec![
            set(strip_row("t3", 3, Lane::Both), 80.0),
            set(strip_row("t3", 4, Lane::Both), 8_000.0),
        ]),
        ok(parameter("t1", "comp", 1, Lane::Left, -30.0)),
        ok(parameter("t1", "comp", 1, Lane::Left, -34.0)),
        ok(parameter("t2", "gate", 1, Lane::Both, 0.0)),
        ok(parameter("t2", "clip", 1, Lane::Both, 12.0)),
        ok(parameter("t2", "shaper", 1, Lane::Both, 0.8)),
        ok(parameter("t3", "multi", 3, Lane::Both, -40.0)),
        ok(parameter("t3", "limiter", 1, Lane::Both, -12.0)),
        ok(parameter("t3", "delay", 4, Lane::Both, 0.8)),
        ok(parameter("t1", "eq", 4, Lane::Both, -9.0)),
        ok(vec![set(
            insert_row("t1", "comp", controls::BYPASS_CONTROL, Lane::Both),
            1.0,
        )]),
        ok(vec![set(route(1), -6.0)]),
        ok(send_matrix([0.5, 0.5, 0.2, 0.8])),
        ok(vec![set(route(2), 1.0)]),
        ok(vec![set(route(2), 0.0)]),
        ok(vec![set(vca(1, Lane::Both), -3.0)]),
        ok(vec![set(vca(2, Lane::Left), 1.0)]),
        ok(vec![set(vca(2, Lane::Left), 0.0)]),
        ok(mute("t1", Lane::Both, true)),
        ok(vec![set(route(7), 0.0)]),
        ok(vec![set(route(7), 1.0)]),
        // The matrix mode is a row whose write moves other rows: the full path.
        ok(vec![set(own_row("t3", 1), 0.0)]),
        ramped(vec![set(own_row("t3", 1), 1.0)], 480),
        // A member added to the VCA is a structural edit: the full path.
        ok(vec![SessionEdit::UpsertVca {
            vca: Vca {
                id: id("v"),
                fader: DualMonoFader {
                    left_db: -3.0,
                    right_db: -3.0,
                    left_mute: false,
                    right_mute: false,
                },
                members: vec![id("t2"), id("t3")],
            },
        }]),
        ok(mute("t3", Lane::Both, false)),
        // Repeats: the same values over a new ramp length.
        ramped(fader("t1", Lane::Both, -9.0), 480),
        ramped(fader("t2", Lane::Left, -10.0), 480),
        ramped(mute("t3", Lane::Both, false), 480),
        ramped(pan("t2", -0.2, 0.7), 480),
        ramped(matrix("t3", [0.7, 0.3, -0.2, 0.9]), 480),
        ramped(vec![set(route(1), -6.0)], 480),
        // Refusals.
        Scripted {
            edits: fader("t1", Lane::Both, 30.0),
            refused: true,
        },
        Scripted {
            edits: vec![set(own_row("b", SOLO), 1.0)],
            refused: true,
        },
        Scripted {
            edits: vec![set(
                address(Owner::Vca(id("nobody")), Section::Own, 1, Lane::Both),
                -3.0,
            )],
            refused: true,
        },
    ]
}

fn is_solo(edit: &SessionEdit) -> bool {
    matches!(edit, SessionEdit::SetControl { address, .. }
        if address.section == Section::Own && address.control == SOLO
            && matches!(address.owner, Owner::Strip(_)))
}

/// Gate 1a. The parity script's edits, each classified both ways on the session it is played on:
/// every edit alone from the start, and the whole script in order on a model that advances, with
/// solos in between, at two rates.
///
/// Red if the fast classifier differs from the full one on any scripted edit, or the rows it exists
/// for do not take the fast path.
#[test]
fn the_parity_script_classifies_as_the_full_classifier_does() {
    assert_installed();
    for rate in [48_000, 96_000] {
        let base = store(parity_session(rate));
        let none = MonitorOverlay::default();
        let mut fast_taken = 0;
        let mut accepted = 0;
        for step in scripted() {
            let Some((prepared, overlay, ramp)) = prepare(&base, &step.edits, &none) else {
                assert!(step.refused, "an accepted edit is accepted alone");
                continue;
            };
            assert!(!step.refused);
            let verdict = judge(&base, &prepared, &none, &overlay, ramp);
            accepted += 1;
            fast_taken += usize::from(verdict.fast_path);
        }
        assert!(fast_taken > 30, "the rows took the fast path: {fast_taken}");
        assert!(accepted > 40, "{accepted}");

        let mut advancing = store(parity_session(rate));
        let mut committed = MonitorOverlay::default();
        let solo = |strip: &str, on: bool| vec![set(own_row(strip, SOLO), f32::from(u8::from(on)))];
        let mut script = scripted();
        for (at, edits) in [
            (20, solo("t2", true)),
            (25, solo("t1", true)),
            (30, solo("t2", false)),
            (38, solo("t1", false)),
        ] {
            script.insert(
                at,
                Scripted {
                    edits,
                    refused: false,
                },
            );
        }
        let mut sequence_fast = 0;
        for step in script {
            let Some((prepared, overlay, ramp)) = prepare(&advancing, &step.edits, &committed)
            else {
                assert!(step.refused);
                continue;
            };
            let verdict = judge(&advancing, &prepared, &committed, &overlay, ramp);
            sequence_fast += usize::from(verdict.fast_path);
            drop(prepared);
            if !step.edits.iter().all(is_solo) {
                let revision = advancing.revision();
                advancing
                    .apply_transaction(ExpectedRevision::Exact(revision), &step.edits)
                    .expect("the script's edit applies");
            }
            committed = overlay;
        }
        assert!(sequence_fast > 30, "{sequence_fast}");
    }
}

// ---------------------------------------------------------------------------------------------
// Gate 1b: a seeded random set of edits
// ---------------------------------------------------------------------------------------------

/// A small deterministic generator (xorshift64*).
struct Draw(u64);

impl Draw {
    fn next(&mut self, bound: usize) -> usize {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        (self.0.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 33) as usize % bound
    }

    fn unit(&mut self) -> f32 {
        self.next(1 << 20) as f32 / (1 << 20) as f32
    }
}

/// A value for `row` that its domain may accept: a toggle, a choice, the disabled value, or a point
/// of the range.
fn draw_value(draw: &mut Draw, row: &ControlRow) -> f32 {
    let d = &row.descriptor;
    match d.kind {
        ControlKind::Toggle => draw.next(2) as f32,
        ControlKind::Enumeration => d.choices[draw.next(d.choices.len())].value,
        ControlKind::Continuous => {
            if let Some(disabled) = d.disabled_value
                && draw.next(6) == 0
            {
                return disabled;
            }
            let value = d.minimum + (d.maximum - d.minimum) * draw.unit();
            if d.unit == effect_contract::ParameterUnit::Samples {
                value.floor()
            } else {
                value
            }
        }
    }
}

/// Gate 1b. At least 2,000 seeded transactions of one to four edits, each drawn from the control
/// list of the session (every row of every class, the session's rows, a strip's matrix mode, an
/// insert's link mode, the solo rows), with and without a per-edit ramp, over a random committed
/// overlay, sometimes with a structural edit among them: the fast classifier equals the full one.
///
/// Red if it differs anywhere, and if the generator did not reach the rows the fast path exists for
/// through the fast path and the rows that must fall back through the full one.
#[test]
fn a_random_transaction_classifies_as_the_full_classifier_does() {
    assert_installed();
    let sessions = [store(rich_session(10)), store(plain_session(4))];
    let mut draw = Draw(0x9E37_79B9_7F4A_7C15);
    let mut cases = 0_usize;
    let mut fast_rows: BTreeSet<&'static str> = BTreeSet::new();
    let mut full_rows: BTreeSet<&'static str> = BTreeSet::new();
    let mut verdicts: BTreeSet<&'static str> = BTreeSet::new();
    let mut attempts = 0;
    while cases < 2_400 {
        attempts += 1;
        assert!(attempts < 40_000, "the generator keeps being refused");
        let store = &sessions[usize::from(cases % 6 == 5)];
        let current = store.compiled().normalized_model();
        let rate = current.sample_rate_hz;
        let tracks: Vec<StableId> = current.tracks.iter().map(|t| t.id.clone()).collect();
        let committed = MonitorOverlay::with_soloed(
            tracks
                .iter()
                .filter(|_| draw.next(5) == 0)
                .cloned()
                .collect::<Vec<_>>(),
        );
        // Half the transactions draw only rows the fast path takes, so that a transaction of
        // several edits is not refused by one rebuild-only row; and a row is drawn by its owner's
        // kind first, so the few routes and VCAs are not drowned by the strips' effect rows.
        let live_only = draw.next(2) == 0;
        let rows: Vec<ControlRow> = controls::list(current, &committed, rate)
            .filter(|row| row.descriptor.class != ControlClass::Derived && row.descriptor.active)
            .filter(|row| {
                !live_only
                    || (row.descriptor.class == ControlClass::Live
                        && !matches!(row.address.owner, Owner::Session)
                        && row.descriptor.name != "matrix_mode")
            })
            .collect();
        let by_kind: Vec<Vec<&ControlRow>> = [
            OwnerKind::Strip,
            OwnerKind::Route,
            OwnerKind::Vca,
            OwnerKind::Session,
        ]
        .into_iter()
        .map(|kind| {
            rows.iter()
                .filter(|row| row.address.owner.kind() == kind)
                .collect::<Vec<_>>()
        })
        .filter(|group| !group.is_empty())
        .collect();
        let mut edits = Vec::new();
        let mut names = Vec::new();
        for _ in 0..1 + draw.next(4) {
            let group = &by_kind[draw.next(by_kind.len())];
            let row = group[draw.next(group.len())];
            let value = draw_value(&mut draw, row);
            let ramp = (matches!(row.descriptor.motion, Motion::Ramp(_)) && draw.next(3) == 0)
                .then(|| [0, 64, 480][draw.next(3)]);
            edits.push(SessionEdit::set_control(row.address.clone(), value, ramp));
            names.push(row.descriptor.name);
        }
        if draw.next(16) == 0 {
            // A structural edit among the value edits: the full classifier decides.
            edits.push(SessionEdit::UpsertVca {
                vca: Vca {
                    id: id("zz-extra"),
                    fader: DualMonoFader {
                        left_db: -1.0,
                        right_db: 0.0,
                        left_mute: false,
                        right_mute: false,
                    },
                    members: vec![tracks[0].clone()],
                },
            });
            names.push("<structural>");
        }
        let Some((prepared, overlay, ramp)) = prepare(store, &edits, &committed) else {
            continue;
        };
        let verdict = judge(store, &prepared, &committed, &overlay, ramp);
        cases += 1;
        let target = if verdict.fast_path {
            &mut fast_rows
        } else {
            &mut full_rows
        };
        target.extend(names);
        verdicts.insert(match verdict.result {
            Ok(0) => "live, no record",
            Ok(_) => "live",
            Err(LiveRebuild::RebuildOnly(_)) => "rebuild-only",
            Err(LiveRebuild::Structure) => "structure",
            Err(LiveRebuild::Domain) => "domain",
            Err(LiveRebuild::AutomationTarget) => "automation",
        });
    }
    for row in [
        "fader_db",
        "mute",
        "pan",
        "matrix_ll",
        "trim_db",
        "polarity_invert",
        "hpf_hz",
        "lpf_hz",
        "bypass",
        "gain_db",
        "offset_db",
        "solo",
        "follows_mute",
        "pan_window_samples",
    ] {
        assert!(fast_rows.contains(row), "no fast classification of {row}");
    }
    for row in ["matrix_mode", "fader_ms", "link_mode", "<structural>"] {
        assert!(full_rows.contains(row), "no full classification of {row}");
    }
    for verdict in ["live", "rebuild-only", "live, no record"] {
        assert!(
            verdicts.contains(verdict),
            "no {verdict} verdict: {verdicts:?}"
        );
    }
}

// ---------------------------------------------------------------------------------------------
// Gate 2: the cost does not grow with the session
// ---------------------------------------------------------------------------------------------

/// The allocations and frees of one classification of `edits` by `classify_live_edit` on `store`,
/// after a warm-up call, and whether the fast path answered.
fn allocations(
    store: &SessionStore,
    edits: &[SessionEdit],
    committed: &MonitorOverlay,
) -> (u64, u64, bool) {
    let (prepared, overlay, ramp) = prepare(store, edits, committed).expect("the edit applies");
    let current = store.compiled().normalized_model();
    let next = prepared.compiled().normalized_model();
    let monitor = MonitorBase {
        committed,
        next: &overlay,
        ramp,
    };
    let run = || {
        let delta = classify_live_edit(
            current,
            next,
            LiveRamps::for_session(next),
            prepared.edit_ramps(),
            monitor,
            prepared.set_control_rows(),
        )
        .expect("live");
        assert!(!delta.is_empty(), "the edit changes a render value");
        drop(delta);
    };
    run();
    let _ = take_fast_classification_count();
    let mark = current_thread_counters();
    run();
    let spent = current_thread_delta_since(mark);
    (
        spent.allocations,
        spent.deallocations,
        take_fast_classification_count() == 1,
    )
}

/// Gate 2. One live-only transaction of each kind costs the same allocations and frees at 8, 64 and
/// 256 tracks, and the fast path answers.
///
/// Red if the classification clones, lists or walks the session: one allocation per strip, per
/// route or per VCA of the session moves the count with the track count.
#[test]
fn the_cost_of_a_live_edit_does_not_grow_with_the_session() {
    assert_installed();
    let sizes = [8, 64, 256];
    let stores: Vec<SessionStore> = sizes.iter().map(|&n| store(rich_session(n))).collect();
    let both_lanes = |strip: &str, control: u32| strip_row(strip, control, Lane::Both);
    let eq = |parameter: u32| {
        address(
            Owner::Strip(id("t0000")),
            Section::Inserts(id("eq")),
            parameter,
            Lane::Both,
        )
    };
    let kinds: Vec<(&str, Vec<SessionEdit>)> = vec![
        ("fader", vec![set(both_lanes("t0007", FADER_DB), -9.5)]),
        (
            "fader under a nested VCA",
            vec![set(both_lanes("t0002", FADER_DB), -9.5)],
        ),
        ("mute", vec![set(both_lanes("t0006", MUTE), 1.0)]),
        (
            "mute of a strip a send follows",
            vec![set(both_lanes("t0000", MUTE), 1.0)],
        ),
        (
            "pan",
            vec![
                set(strip_row("t0004", PAN, Lane::Left), 0.25),
                set(strip_row("t0004", PAN, Lane::Right), -0.25),
            ],
        ),
        ("trim", vec![set(both_lanes("t0005", TRIM_DB), -3.0)]),
        ("effect parameter", vec![set(eq(4), -4.0)]),
        (
            "send gain",
            vec![set(
                address(Owner::Route(id("t0000-bus")), Section::Own, 1, Lane::Both),
                -6.0,
            )],
        ),
        (
            "send mute",
            vec![set(
                address(Owner::Route(id("t0001-bus")), Section::Own, 2, Lane::Both),
                1.0,
            )],
        ),
        (
            "VCA offset",
            vec![set(
                address(Owner::Vca(id("v0")), Section::Own, 1, Lane::Both),
                -3.0,
            )],
        ),
        (
            "VCA mute",
            vec![set(
                address(Owner::Vca(id("v1")), Section::Own, 2, Lane::Left),
                1.0,
            )],
        ),
    ];
    for (name, edits) in kinds {
        let counts: Vec<(u64, u64, bool)> = stores
            .iter()
            .map(|store| allocations(store, &edits, &MonitorOverlay::default()))
            .collect();
        assert!(
            counts.iter().all(|count| count.2),
            "{name}: fast path {counts:?}"
        );
        assert!(counts[0].0 > 0, "{name}: the call is counted: {counts:?}");
        println!(
            "{name}: {} allocations, {} frees at {sizes:?} tracks",
            counts[0].0, counts[0].1
        );
        assert!(
            counts
                .iter()
                .all(|count| count.0 == counts[0].0 && count.1 == counts[0].1),
            "{name}: the allocations depend on the track count {sizes:?}: {counts:?}"
        );
    }
    // A solo that leaves some other strip soloed moves one strip.
    let others = MonitorOverlay::with_soloed([id("t0006")]);
    let solo = vec![set(own_row("t0005", SOLO), 1.0)];
    let counts: Vec<(u64, u64, bool)> = stores
        .iter()
        .map(|store| allocations(store, &solo, &others))
        .collect();
    assert!(
        counts.iter().all(|count| count.2),
        "solo: fast path {counts:?}"
    );
    assert!(
        counts
            .iter()
            .all(|count| count.0 == counts[0].0 && count.1 == counts[0].1),
        "solo: the allocations depend on the track count: {counts:?}"
    );
}

// ---------------------------------------------------------------------------------------------
// Gate 3: one descriptive timing line
// ---------------------------------------------------------------------------------------------

/// The timing line: a fader edit on the session #1562a timed, 200 calls, by the full classifier
/// (before) and by `classify_live_edit` (after).
#[test]
#[ignore = "descriptive timing: cargo test --release -p host-core --all-features --test live_edit_classifier -- --ignored --nocapture"]
fn timing_line() {
    for tracks in [64, 256] {
        let store = store(plain_session(tracks));
        let edits = [set(strip_row("t0003", FADER_DB, Lane::Both), -9.0)];
        let none = MonitorOverlay::default();
        let (prepared, overlay, ramp) = prepare(&store, &edits, &none).expect("applies");
        let current = store.compiled().normalized_model();
        let next = prepared.compiled().normalized_model();
        let monitor = MonitorBase {
            committed: &none,
            next: &overlay,
            ramp,
        };
        let ramps = LiveRamps::for_session(next);
        let calls = 200;
        let start = Instant::now();
        for _ in 0..calls {
            let delta = classify_live_delta_with_monitor(
                current,
                next,
                ramps,
                prepared.edit_ramps(),
                monitor,
            )
            .expect("live");
            assert_eq!(delta.strips.len(), 1);
        }
        let before = start.elapsed() / calls;
        let start = Instant::now();
        for _ in 0..calls {
            let delta = classify_live_edit(
                current,
                next,
                ramps,
                prepared.edit_ramps(),
                monitor,
                prepared.set_control_rows(),
            )
            .expect("live");
            assert_eq!(delta.strips.len(), 1);
        }
        let after = start.elapsed() / calls;
        // The protocol's preparation of the same transaction (a model clone and a compile), which
        // this slice does not change: the share of a whole command that stays linear in the session.
        let start = Instant::now();
        for _ in 0..calls {
            let prepared = prepare(&store, &edits, &none).expect("applies");
            drop(prepared);
        }
        let preparation = start.elapsed() / calls;
        println!(
            "classifier fader edit, {tracks} tracks: full {before:?} per call, fast {after:?} per call; protocol preparation of the transaction {preparation:?}"
        );
    }
}
