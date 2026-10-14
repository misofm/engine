//! Sessions, the #1560 parity script and a seeded generator of transactions, shared by the tests
//! of the live-edit classifier (TBD9) and of the in-place commit (TBD10).
#![allow(dead_code)]

use controls::{
    ControlAddress, ControlClass, ControlKind, ControlRow, Lane, MonitorOverlay, Motion, Owner,
    OwnerKind, Section,
};
use protocol::{SessionEdit, SessionStore};
use session::{
    CompileCaps, Console, DualMonoFader, Effect, EffectIdentity, EffectQuality, LinkMode,
    MatrixOrPan, RouteDestination, RouteSource, SendTap, SessionModel, SidechainDeclaration,
    StableId, Submix, Vca, parse_session_json,
};

pub(crate) const FIXTURE: &str =
    include_str!("../../../../fixtures/session/v1/parametric-eq-nine-track.json");

pub(crate) fn caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

pub(crate) fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

pub(crate) fn store(model: SessionModel) -> SessionStore {
    SessionStore::new(model, caps()).expect("the session compiles")
}

pub(crate) fn insert(name: &str, native: &str) -> Effect {
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
pub(crate) fn plain_session(tracks: usize) -> SessionModel {
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
pub(crate) fn rich_session(tracks: usize) -> SessionModel {
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

pub(crate) fn address(owner: Owner, section: Section, control: u32, lane: Lane) -> ControlAddress {
    ControlAddress {
        owner,
        section,
        control,
        lane,
    }
}

pub(crate) fn strip_row(strip: &str, control: u32, lane: Lane) -> ControlAddress {
    address(Owner::Strip(id(strip)), Section::Builtins, control, lane)
}

pub(crate) fn own_row(strip: &str, control: u32) -> ControlAddress {
    address(Owner::Strip(id(strip)), Section::Own, control, Lane::Both)
}

pub(crate) const FADER_DB: u32 = 5;
pub(crate) const MUTE: u32 = 6;
pub(crate) const PAN: u32 = 12;
pub(crate) const TRIM_DB: u32 = 2;
pub(crate) const SOLO: u32 = 2;

pub(crate) fn set(address: ControlAddress, value: f32) -> SessionEdit {
    SessionEdit::set_control(address, value, None)
}

/// The parity session of `hosts/host-web/tests/common/mod.rs` (`model_at`): tracks `t1` (pan with a
/// window, an EQ and a compressor), `t2` (pan, a gate, a clipper and a transient shaper), `t3`
/// (matrix, a multiband compressor, a limiter and a delay), submix `b`, `t1` sending pre-fader into
/// `b` with `follows_mute`, and VCA `v` holding `t2`.
pub(crate) fn parity_session(rate: u32) -> SessionModel {
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
pub(crate) struct Scripted {
    pub(crate) edits: Vec<SessionEdit>,
    pub(crate) refused: bool,
}

pub(crate) fn scripted() -> Vec<Scripted> {
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

pub(crate) fn is_solo(edit: &SessionEdit) -> bool {
    matches!(edit, SessionEdit::SetControl { address, .. }
        if address.section == Section::Own && address.control == SOLO
            && matches!(address.owner, Owner::Strip(_)))
}

/// A small deterministic generator (xorshift64*).
pub(crate) struct Draw(pub(crate) u64);

impl Draw {
    pub(crate) fn next(&mut self, bound: usize) -> usize {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        (self.0.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 33) as usize % bound
    }

    pub(crate) fn unit(&mut self) -> f32 {
        self.next(1 << 20) as f32 / (1 << 20) as f32
    }
}

/// A value for `row` that its domain may accept: a toggle, a choice, the disabled value, or a point
/// of the range.
pub(crate) fn draw_value(draw: &mut Draw, row: &ControlRow) -> f32 {
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

/// One drawn transaction: the overlay committed before it, the edits and the name of the row each
/// edit writes.
pub(crate) struct Case {
    pub(crate) committed: MonitorOverlay,
    pub(crate) edits: Vec<SessionEdit>,
    pub(crate) names: Vec<&'static str>,
}

/// One to four edits drawn from the control list of `current` (every row of every class, the
/// session's rows, a strip's matrix mode, an insert's link mode, the solo rows), with and without a
/// per-edit ramp, over a random committed overlay, sometimes with a structural edit among them.
pub(crate) fn draw_case(draw: &mut Draw, current: &SessionModel) -> Case {
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
        let value = draw_value(draw, row);
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
    Case {
        committed,
        edits,
        names,
    }
}
