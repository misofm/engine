//! Issue #1255: `classify_live_delta` decides whether a committed session delta is a live track
//! fader, mute and pan update or a rebuild, and for a live one returns exactly the records to push.
//!
//! Gate 1 builds its models from a two-track session with an empty console; gate 2 compares the
//! classifier's domain with the render-side setters'; gate 3 pushes the classifier's records into a
//! live plan of the nine-track EQ fixture and compares the output with a rebuilt plan's.
#![cfg(feature = "control-provider")]

use controls::RebuildReason;

use builtins::{
    BuiltinFaderBank, BuiltinMatrixBank, BuiltinParameters, Matrix2x2, PreparedInputFilterTarget,
    pan_matrix,
};
use builtins_compiler::TrackInputRecord;
use dsp_reference::randomized::Draw;
use effect_contract::{
    BankWidth, EffectControlRecord, PREPARED_EFFECT_TARGET_WORDS, ParameterChannel as Lane,
    PreparedEffectTarget,
};
use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use host_core::{
    BuiltinLaneSelector, EQ_TARGET_CAPACITY, EqTargetEdit, EqTargetPreparer,
    HostLiveControlRequest, HostPrepareCaps, HostShapePolicy, InputFilterEdit, InputFilterPreparer,
    LiveDelta, LiveEffectAddress, LiveRampRow, LiveRamps, LiveRebuild, PreparedHost,
    SourceSubmission, TrackControlRecord, TrackFaderRecord, classify_live_delta,
    compile_host_session, prepare_host_runtime, prepare_host_runtime_with_live_controls,
};
use session::{
    Automation, AutomationSegment, AutomationShape, Console, ConsoleEntry, ConsoleSlot,
    ControlAddress, ControlSmoothing, DualMonoFader, EditRampRow, EditRampTarget, EditRamps,
    EffectIdentity, EffectParam, MatrixOrPan, Owner, ParameterChannel, ParameterUnit,
    RouteDestination, RouteSource, Section, SendTap, SessionModel, StableId, Submix, Vca,
    canonical_session_json, parse_session_json,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE: &str = "fixture-source";
const BLOCKS: usize = 8;
/// One strip's records, owned: its ID, its fader/mute records in push order, its matrix record.
type Strip = (String, Vec<TrackFaderRecord>, Option<TrackControlRecord>);
type Edit = fn(&mut SessionModel);

const STEP: LiveRamps = LiveRamps {
    fader_samples: 0,
    mute_samples: 0,
    pan_samples: 0,
    link_samples: 0,
};

fn caps() -> HostPrepareCaps {
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
        maximum_graph_session_plus_plan_bytes: 1_000_000_000,
        maximum_source_total_bytes: 100_000_000,
        maximum_source_overhead_bytes: 100_000_000,
        maximum_effect_state_bytes: 1_000_000_000,
        maximum_effect_scratch_bytes: 1_000_000_000,
        maximum_builtin_retained_bytes: 1_000_000_000,
        maximum_named_allocation_bytes: 1_000_000_000,
        maximum_meter_streams: 64,
        maximum_meter_items: 1 << 16,
        maximum_meter_bytes: 1 << 24,
    }
}

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

fn compile(model: &SessionModel) -> session::CompiledSession {
    let document = canonical_session_json(model).expect("canonical");
    compile_host_session(&document, &caps()).unwrap_or_else(|failure| {
        panic!("compile: {}", String::from_utf8_lossy(failure.as_bytes()))
    })
}

fn normalized(model: &SessionModel) -> SessionModel {
    compile(model).normalized_model().clone()
}

/// Every track at 0 dB, unmuted, centred with no smoothing.
fn settle(model: &mut SessionModel) {
    for track in &mut model.tracks {
        track.fader = DualMonoFader {
            left_db: 0.0,
            right_db: 0.0,
            left_mute: false,
            right_mute: false,
        };
        track.matrix_or_pan = MatrixOrPan::Pan {
            left: 0.0,
            right: 0.0,
            smoothing_samples: 0,
        };
    }
}

/// The fixture's first two tracks, with an empty console and no inserts, as a normalized model.
fn two_tracks() -> SessionModel {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    model.console = Console {
        pre_insert: Vec::new(),
        post_insert: Vec::new(),
    };
    model.tracks.truncate(2);
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
    }
    let kept: Vec<StableId> = model.tracks.iter().map(|track| track.id.clone()).collect();
    model.routes.retain(|route| {
        matches!(&route.source, RouteSource::Track { track_id, .. } if kept.contains(track_id))
    });
    settle(&mut model);
    normalized(&model)
}

/// `two_tracks` with the first track's only route sent through submix `bus`, and that route
/// following the track's mute when `follows_mute`.
fn with_bus(follows_mute: bool) -> SessionModel {
    let mut model = two_tracks();
    let first = model.tracks[0].id.clone();
    let bus = id("bus");
    let route = model
        .routes
        .iter_mut()
        .find(|route| matches!(&route.source, RouteSource::Track { track_id, .. } if *track_id == first))
        .expect("first track's route");
    let output = core::mem::replace(
        &mut route.destination,
        RouteDestination::SubmixInput {
            submix_id: bus.clone(),
        },
    );
    route.follows_mute = follows_mute;
    let mut bus_main = route.clone();
    bus_main.id = id("bus-main");
    bus_main.follows_mute = false;
    bus_main.source = RouteSource::Submix {
        submix_id: bus.clone(),
        tap: SendTap::PostPan,
    };
    bus_main.destination = output;
    model.routes.push(bus_main);
    model.submixes.push(Submix::unity(bus, &model.console));
    normalized(&model)
}

/// `current` with only its revision advanced, then `edit` applied.
fn edited(current: &SessionModel, edit: impl FnOnce(&mut SessionModel)) -> SessionModel {
    let mut next = current.clone();
    next.revision += 1;
    edit(&mut next);
    next
}

fn classify(
    current: &SessionModel,
    edit: impl FnOnce(&mut SessionModel),
) -> Result<Vec<Strip>, LiveRebuild> {
    let next = edited(current, edit);
    classify_live_delta(current, &next, STEP, &EditRamps::new()).map(|delta| flatten(&delta))
}

fn flatten(delta: &LiveDelta<'_>) -> Vec<Strip> {
    delta
        .strips
        .iter()
        .map(|strip| {
            (
                strip.strip_id.to_owned(),
                strip.fader_records().collect(),
                strip.matrix,
            )
        })
        .collect()
}

fn fader_db(lanes: BuiltinLaneSelector, db: f32, smoothing_samples: u32) -> TrackFaderRecord {
    TrackFaderRecord::FaderDb {
        lanes,
        db,
        smoothing_samples,
    }
}

fn mute(lanes: BuiltinLaneSelector, muted: bool, smoothing_samples: u32) -> TrackFaderRecord {
    TrackFaderRecord::Mute {
        lanes,
        muted,
        smoothing_samples,
    }
}

/// The builtin ID the control rows use for the input delay (`BUILTIN_PARAMETER_DESCRIPTORS`).
const DELAY_SAMPLES: u32 = 11;

/// The reason the control row of builtin `control` gives for being rebuild-only.
fn builtin_reason(control: u32) -> RebuildReason {
    let descriptor = controls::row_descriptor(
        controls::OwnerKind::Strip,
        controls::SectionKind::Builtins,
        control,
        48_000,
    )
    .expect("a builtin row");
    match descriptor.class {
        controls::ControlClass::RebuildOnly { reason, .. } => reason,
        other => panic!("builtin {control} is {other:?}, not rebuild-only"),
    }
}

fn track_id(model: &SessionModel, index: usize) -> String {
    model.tracks[index].id.as_str().to_owned()
}

/// Gate 1(b). Red if a fader record goes to the wrong lane, merges lanes wrongly or is emitted
/// for a sign-of-zero change that keeps the gain.
#[test]
fn fader_records_address_exactly_the_changed_lanes() {
    use BuiltinLaneSelector::{Both, Left, Right};
    let current = two_tracks();
    let first = track_id(&current, 0);
    assert_eq!(
        classify(&current, |next| next.tracks[0].fader.left_db = -6.0),
        Ok(vec![(first.clone(), vec![fader_db(Left, -6.0, 0)], None)])
    );
    assert_eq!(
        classify(&current, |next| {
            next.tracks[0].fader.left_db = -3.0;
            next.tracks[0].fader.right_db = -3.0;
        }),
        Ok(vec![(first.clone(), vec![fader_db(Both, -3.0, 0)], None)])
    );
    assert_eq!(
        classify(&current, |next| {
            next.tracks[0].fader.left_db = -2.0;
            next.tracks[0].fader.right_db = -4.0;
        }),
        Ok(vec![(
            first,
            vec![fader_db(Left, -2.0, 0), fader_db(Right, -4.0, 0)],
            None
        )])
    );
    assert_eq!(
        classify(&current, |next| {
            next.tracks[1].fader.left_db = -0.0;
            next.tracks[1].fader.right_db = -0.0;
        }),
        Ok(Vec::new())
    );
    // #1255 D5: a fader move on a lane muted in both models still gets its record, so the stage
    // remembers the gain for a later unmute.
    let mut muted = current.clone();
    muted.tracks[1].fader.left_mute = true;
    assert_eq!(
        classify(&muted, |next| next.tracks[1].fader.left_db = -6.0),
        Ok(vec![(
            track_id(&muted, 1),
            vec![fader_db(Left, -6.0, 0)],
            None
        )])
    );
}

/// Gate 1(c). Red if a mute record goes to the wrong lane or merges two lanes that change to
/// different values.
#[test]
fn mute_records_address_exactly_the_changed_lanes() {
    use BuiltinLaneSelector::{Both, Left, Right};
    let current = two_tracks();
    let second = track_id(&current, 1);
    assert_eq!(
        classify(&current, |next| next.tracks[1].fader.left_mute = true),
        Ok(vec![(second.clone(), vec![mute(Left, true, 0)], None)])
    );
    assert_eq!(
        classify(&current, |next| {
            next.tracks[1].fader.left_mute = true;
            next.tracks[1].fader.right_mute = true;
        }),
        Ok(vec![(second.clone(), vec![mute(Both, true, 0)], None)])
    );
    let split = edited(&current, |model| model.tracks[1].fader.left_mute = true);
    assert_eq!(
        classify(&split, |next| {
            next.tracks[1].fader.left_mute = false;
            next.tracks[1].fader.right_mute = true;
        }),
        Ok(vec![(
            second,
            vec![mute(Left, false, 0), mute(Right, true, 0)],
            None
        )])
    );
}

/// Gate 1(d). Red if the fixed fader-then-mute order changes.
#[test]
fn fader_records_precede_mute_records() {
    use BuiltinLaneSelector::Left;
    let current = two_tracks();
    assert_eq!(
        classify(&current, |next| {
            next.tracks[0].fader.left_mute = true;
            next.tracks[0].fader.left_db = -12.0;
        }),
        Ok(vec![(
            track_id(&current, 0),
            vec![fader_db(Left, -12.0, 0), mute(Left, true, 0)],
            None
        )])
    );
}

/// Gate 1(e). Red if a pan record carries a target other than the lowered one, or is emitted
/// when only the representation or the smoothing changes.
#[test]
fn a_matrix_record_is_emitted_only_when_the_lowered_target_changes() {
    let current = two_tracks();
    assert_eq!(
        classify(&current, |next| {
            next.tracks[0].matrix_or_pan = MatrixOrPan::Pan {
                left: -1.0,
                right: 1.0,
                smoothing_samples: 64,
            };
        }),
        Ok(vec![(
            track_id(&current, 0),
            Vec::new(),
            Some(TrackControlRecord {
                matrix: pan_matrix(-1.0, 1.0).expect("pan"),
                smoothing_samples: 64,
            })
        )])
    );
    let centre = pan_matrix(0.0, 0.0).expect("pan");
    assert_eq!(
        classify(&current, |next| {
            next.tracks[0].matrix_or_pan = MatrixOrPan::Matrix {
                ll: centre.ll,
                lr: centre.lr,
                rl: centre.rl,
                rr: centre.rr,
                smoothing_samples: 0,
            };
        }),
        Ok(Vec::new())
    );
    assert_eq!(
        classify(&current, |next| {
            next.tracks[0].matrix_or_pan = MatrixOrPan::Pan {
                left: 0.0,
                right: 0.0,
                smoothing_samples: 512,
            };
        }),
        Ok(Vec::new())
    );
}

fn set_ll(next: &mut SessionModel, ll: f32) {
    next.tracks[0].matrix_or_pan = MatrixOrPan::Matrix {
        ll,
        lr: 0.0,
        rl: 0.0,
        rr: 0.5,
        smoothing_samples: 0,
    };
}

/// Gate 1(f). Red if a value outside the render setter's domain is classified live.
#[test]
fn values_outside_the_setter_domain_need_a_rebuild() {
    let current = two_tracks();
    for db in [24.0_f32, -144.0] {
        assert!(classify(&current, |next| next.tracks[0].fader.left_db = db).is_ok());
    }
    for db in [24.0_f32.next_up(), (-144.0_f32).next_down(), f32::NAN] {
        assert_eq!(
            classify(&current, |next| next.tracks[0].fader.right_db = db),
            Err(LiveRebuild::Domain),
            "{db}"
        );
    }
    assert!(classify(&current, |next| set_ll(next, 1.0)).is_ok());
    assert_eq!(
        classify(&current, |next| set_ll(next, 1.0_f32.next_up())),
        Err(LiveRebuild::Domain)
    );
}

/// #1554 gate 1, D5. Red if a submix strip is left structural: its fader, mute and pan are live
/// records addressed to the submix's ID, as a track's are, and a track's records are not
/// addressed to it.
#[test]
fn a_submix_strip_fader_mute_and_pan_are_live() {
    let current = with_bus(false);
    assert_eq!(
        classify(&current, |next| next.submixes[0].fader.left_db = -6.0),
        Ok(vec![(
            "bus".to_owned(),
            vec![fader_db(BuiltinLaneSelector::Left, -6.0, 0)],
            None
        )])
    );
    assert_eq!(
        classify(&current, |next| {
            next.submixes[0].fader.left_mute = true;
            next.submixes[0].fader.right_mute = true;
        }),
        Ok(vec![(
            "bus".to_owned(),
            vec![mute(BuiltinLaneSelector::Both, true, 0)],
            None
        )])
    );
    let panned = classify(&current, |next| {
        next.submixes[0].matrix_or_pan = MatrixOrPan::Pan {
            left: 0.5,
            right: 0.5,
            smoothing_samples: 0,
        };
    })
    .expect("a submix pan is live");
    assert_eq!(panned.len(), 1);
    assert_eq!(panned[0].0, "bus");
    assert!(panned[0].1.is_empty() && panned[0].2.is_some());
}

/// A delta's strip records and its route records, owned.
type Both = (Vec<Strip>, Vec<RouteRecordRow>);

/// Classify with the session ramps `ramps` and the transaction's `edit_ramps`, and return both
/// kinds of record.
fn classify_both(
    current: &SessionModel,
    edit: impl FnOnce(&mut SessionModel),
    ramps: LiveRamps,
    edit_ramps: &EditRamps,
) -> Result<Both, LiveRebuild> {
    let next = edited(current, edit);
    classify_live_delta(current, &next, ramps, edit_ramps).map(|delta| {
        let routes = delta
            .routes
            .iter()
            .map(|route| {
                (
                    route.route_id.to_owned(),
                    route.record.folded().expect("a six-word route").target(),
                    route.record.folded().expect("a six-word route").mute(),
                    route.record.folded().expect("a six-word route").length(),
                )
            })
            .collect();
        (flatten(&delta), routes)
    })
}

/// #1558 D1 (was #1554 D5's guard). Red if a submix whose mute a `follows_mute` route reads is
/// refused, or its follower is not gated by the same transaction: the bus mutes in the same delta
/// as the send from it, and the other submix, which nothing follows, gives no route record.
#[test]
fn a_followed_submix_mute_change_is_live_and_gates_the_follower() {
    let mut model = with_bus(false);
    let bus = id("bus");
    let bus2 = id("bus2");
    // bus -> bus2 (following bus's mute) -> output.
    let to_output = model
        .routes
        .iter()
        .find(|route| route.id == id("bus-main"))
        .expect("the bus's output route")
        .clone();
    model.routes.retain(|route| route.id != id("bus-main"));
    let mut to_bus2 = to_output.clone();
    to_bus2.id = id("bus-bus2");
    to_bus2.destination = RouteDestination::SubmixInput {
        submix_id: bus2.clone(),
    };
    to_bus2.follows_mute = true;
    let mut bus2_out = to_output;
    bus2_out.id = id("bus2-main");
    bus2_out.source = RouteSource::Submix {
        submix_id: bus2.clone(),
        tap: SendTap::PostPan,
    };
    model.routes.push(to_bus2);
    model.routes.push(bus2_out);
    let console = model.console.clone();
    model.submixes.push(Submix::unity(bus2, &console));
    let current = normalized(&model);
    let (strips, routes) = classify_both(
        &current,
        |next| {
            let submix = next.submixes.iter_mut().find(|s| s.id == bus).expect("bus");
            submix.fader.left_mute = true;
        },
        STEP,
        &EditRamps::new(),
    )
    .expect("a followed submix mute is live");
    assert_eq!(
        strips,
        vec![(
            "bus".to_owned(),
            vec![mute(BuiltinLaneSelector::Left, true, 0)],
            None
        )]
    );
    assert_eq!(routes.len(), 1);
    assert_eq!(routes[0].0, "bus-bus2");
    // Only the left column is zeroed: the right source lane still sends.
    let route = current
        .routes
        .iter()
        .find(|route| route.id == id("bus-bus2"))
        .expect("the follower");
    assert_eq!(
        routes[0].1,
        folded(route.gain_db, matrix_of(route), false, [true, false])
    );
    // The same change on the other submix, which nothing follows, moves no route.
    let (strips, routes) = classify_both(
        &current,
        |next| {
            let submix = next
                .submixes
                .iter_mut()
                .find(|s| s.id == id("bus2"))
                .expect("bus2");
            submix.fader.left_mute = true;
        },
        STEP,
        &EditRamps::new(),
    )
    .expect("live");
    assert_eq!(strips.len(), 1);
    assert!(routes.is_empty());
}

/// Gate 1(i), #1558 gates 2 and 3 (was G2). Red if a mute change of a `follows_mute` source is
/// still a rebuild, if it is not gated in the same delta, or if a redundant route record is
/// emitted: a source that is muted again, a follow toggled on an unmuted source, a mute of a
/// track no route follows.
#[test]
fn a_followed_mute_change_is_live_and_gates_the_send_once() {
    let current = with_bus(true);
    let send = send_index(&current);
    let source = current
        .tracks
        .iter()
        .position(|track| {
            matches!(&current.routes[send].source, RouteSource::Track { track_id, .. } if *track_id == track.id)
        })
        .expect("followed track");
    let send_id = current.routes[send].id.as_str().to_owned();
    let matrix = matrix_of(&current.routes[send]);
    let gain = current.routes[send].gain_db;
    let run = |current: &SessionModel, edit: fn(&mut SessionModel, usize)| {
        classify_both(current, |next| edit(next, source), STEP, &EditRamps::new()).expect("live")
    };
    // The right lane mutes: one strip record and one route record whose right column is zeroed.
    let (strips, routes) = run(&current, |next, source| {
        next.tracks[source].fader.right_mute = true;
    });
    assert_eq!(
        strips,
        vec![(
            track_id(&current, source),
            vec![mute(BuiltinLaneSelector::Right, true, 0)],
            None
        )]
    );
    assert_eq!(
        routes,
        vec![(
            send_id.clone(),
            folded(gain, matrix, false, [false, true]),
            false,
            0
        )]
    );
    // Both lanes: the send is silenced by its source.
    let (_, routes) = run(&current, |next, source| {
        next.tracks[source].fader.left_mute = true;
        next.tracks[source].fader.right_mute = true;
    });
    assert_eq!(routes, vec![(send_id.clone(), [0.0; 4], true, 0)]);
    // Re-muting a muted source gives no record at all; unmuting it gives the open send back.
    let muted = edited(&current, |model| {
        model.tracks[source].fader.right_mute = true
    });
    let (strips, routes) = run(&muted, |next, source| {
        next.tracks[source].fader.right_mute = true;
    });
    assert!(strips.is_empty() && routes.is_empty());
    let (_, routes) = run(&muted, |next, source| {
        next.tracks[source].fader.right_mute = false;
    });
    assert_eq!(
        routes,
        vec![(
            send_id.clone(),
            folded(gain, matrix, false, [false; 2]),
            false,
            0
        )]
    );
    // A fader move on the followed source moves no send.
    let (strips, routes) = run(&current, |next, source| {
        next.tracks[source].fader.left_db = -6.0;
    });
    assert_eq!(strips.len(), 1);
    assert!(routes.is_empty());
    // Toggling `follows_mute` on an unmuted source gives no record; on a muted one it does.
    let toggled = classify_both(
        &current,
        |next| next.routes[send].follows_mute = false,
        STEP,
        &EditRamps::new(),
    )
    .expect("live");
    assert!(toggled.0.is_empty() && toggled.1.is_empty());
    let (_, routes) = classify_both(
        &muted,
        |next| next.routes[send].follows_mute = false,
        STEP,
        &EditRamps::new(),
    )
    .expect("live");
    assert_eq!(
        routes,
        vec![(send_id, folded(gain, matrix, false, [false; 2]), false, 0)]
    );
    // The same mute on a track nothing follows gives its strip record and no route record.
    let (strips, routes) = classify_both(
        &with_bus(false),
        |next| next.tracks[source].fader.right_mute = true,
        STEP,
        &EditRamps::new(),
    )
    .expect("live");
    assert_eq!(strips.len(), 1);
    assert!(routes.is_empty());
}

fn vca() -> Vca {
    Vca {
        id: id("vca"),
        fader: DualMonoFader {
            left_db: 0.0,
            right_db: 0.0,
            left_mute: false,
            right_mute: false,
        },
        members: Vec::new(),
    }
}

/// `two_tracks` with VCA `vca` holding the first track, at `left_db`/`right_db` and `muted`.
fn with_vca(left_db: f32, right_db: f32, muted: [bool; 2]) -> SessionModel {
    let mut model = two_tracks();
    let mut group = vca();
    group.fader = DualMonoFader {
        left_db,
        right_db,
        left_mute: muted[0],
        right_mute: muted[1],
    };
    group.members = vec![model.tracks[0].id.clone()];
    model.vcas.push(group);
    normalized(&model)
}

/// #1558 gates 1 and 3. Red if a VCA's move, a member change or a VCA added or removed still
/// forces a rebuild, if the record carries a raw value instead of the member's effective one, or if
/// a VCA move that leaves the member where it was is not silent: a member at +24 dB stays clamped.
#[test]
fn a_vca_move_is_the_members_effective_record() {
    use BuiltinLaneSelector::{Both, Left, Right};
    let current = with_vca(0.0, 0.0, [false; 2]);
    let first = track_id(&current, 0);
    assert_eq!(
        classify(&current, |next| next.vcas[0].fader.left_db = -6.0),
        Ok(vec![(first.clone(), vec![fader_db(Left, -6.0, 0)], None)])
    );
    // Both lanes to one value merge into one record; the other track is not in the group.
    assert_eq!(
        classify(&current, |next| {
            next.vcas[0].fader.left_db = -3.0;
            next.vcas[0].fader.right_db = -3.0;
        }),
        Ok(vec![(first.clone(), vec![fader_db(Both, -3.0, 0)], None)])
    );
    // The VCA offset adds to the member's own fader.
    let own = edited(&current, |model| model.tracks[0].fader.right_db = -2.0);
    assert_eq!(
        classify(&own, |next| next.vcas[0].fader.right_db = -6.0),
        Ok(vec![(first.clone(), vec![fader_db(Right, -8.0, 0)], None)])
    );
    // A VCA mute is the member's mute, lane by lane.
    assert_eq!(
        classify(&current, |next| next.vcas[0].fader.right_mute = true),
        Ok(vec![(first.clone(), vec![mute(Right, true, 0)], None)])
    );
    // A member at +24 dB stays at +24 dB under a positive offset: no record.
    let loud = edited(&current, |model| {
        model.tracks[0].fader.left_db = 24.0;
        model.tracks[0].fader.right_db = 24.0;
    });
    assert_eq!(
        classify(&loud, |next| {
            next.vcas[0].fader.left_db = 6.0;
            next.vcas[0].fader.right_db = 6.0;
        }),
        Ok(Vec::new())
    );
    // An exact replay writes nothing.
    assert_eq!(classify(&current, |_| {}), Ok(Vec::new()));
}

/// #1558 D5. Red if a membership change or a VCA added or removed is a rebuild, or does not move
/// exactly the members it reaches: a second member joins, a nested VCA passes its offset down, a VCA
/// is added over a member and removed again.
#[test]
fn vca_membership_and_the_vca_set_are_live() {
    use BuiltinLaneSelector::{Both, Left};
    let current = with_vca(-6.0, -6.0, [false; 2]);
    let (first, second) = (track_id(&current, 0), track_id(&current, 1));
    // The second track joins the group: it takes the offset.
    assert_eq!(
        classify(&current, |next| {
            let member = next.tracks[1].id.clone();
            next.vcas[0].members.push(member);
        }),
        Ok(vec![(second.clone(), vec![fader_db(Both, -6.0, 0)], None)])
    );
    // The first track leaves: it returns to its own fader.
    assert_eq!(
        classify(&current, |next| next.vcas[0].members.clear()),
        Ok(vec![(first.clone(), vec![fader_db(Both, 0.0, 0)], None)])
    );
    // A VCA added over the second track, and a VCA that holds the first VCA (nesting).
    assert_eq!(
        classify(&current, |next| {
            let mut group = vca();
            group.id = id("outer");
            group.fader.left_db = -3.0;
            group.members = vec![next.vcas[0].id.clone()];
            next.vcas.push(group);
        }),
        Ok(vec![(first.clone(), vec![fader_db(Left, -9.0, 0)], None)])
    );
    // The VCA removed: its member returns to 0 dB.
    assert_eq!(
        classify(&current, |next| next.vcas.clear()),
        Ok(vec![(first, vec![fader_db(Both, 0.0, 0)], None)])
    );
    // A VCA added over nothing changes nothing.
    assert_eq!(
        classify(&with_vca(0.0, 0.0, [false; 2]), |next| {
            let mut group = vca();
            group.id = id("empty");
            next.vcas.push(group);
        }),
        Ok(Vec::new())
    );
}

/// #1558 gate 2. Red if the records diff raw mutes: a track muted by its VCA that is also muted by
/// its own fader changes no effective value, so toggling its own mute gives no strip record and no
/// route record, with a following send in the session.
#[test]
fn records_follow_effective_values_not_raw_ones() {
    let mut model = with_bus(true);
    let source = model
        .tracks
        .iter()
        .position(|track| {
            matches!(&model.routes[send_index(&model)].source, RouteSource::Track { track_id, .. } if *track_id == track.id)
        })
        .expect("the send's source");
    let mut group = vca();
    group.fader.left_mute = true;
    group.members = vec![model.tracks[source].id.clone()];
    model.vcas.push(group);
    let current = normalized(&model);
    let (strips, routes) = classify_both(
        &current,
        |next| next.tracks[source].fader.left_mute = true,
        STEP,
        &EditRamps::new(),
    )
    .expect("live");
    assert!(strips.is_empty() && routes.is_empty());
    // Unmuting the VCA while the track's own mute holds is silent too.
    let held = edited(&current, |model| {
        model.tracks[source].fader.left_mute = true
    });
    let (strips, routes) = classify_both(
        &held,
        |next| next.vcas[0].fader.left_mute = false,
        STEP,
        &EditRamps::new(),
    )
    .expect("live");
    assert!(strips.is_empty() && routes.is_empty());
    // The same VCA mute lifted with no own mute reopens the left column of the send.
    let (strips, routes) = classify_both(
        &current,
        |next| next.vcas[0].fader.left_mute = false,
        STEP,
        &EditRamps::new(),
    )
    .expect("live");
    assert_eq!(strips.len(), 1);
    assert_eq!(routes.len(), 1);
}

/// #1558 gate 5, D7. Red if a record takes the wrong row's length: a VCA offset edit's own length
/// moves the member, the session's `fader_ms` length applies without one, a membership change takes
/// the `VcaOffset` row's session length, the strip's own `fader_db` edit wins over the VCA's on a
/// lane it moved, and a gate change from the source's mute takes the source's `Mute` length.
#[test]
fn vca_and_follow_records_take_the_ramp_of_the_edit_that_moved_them() {
    use BuiltinLaneSelector::{Both, Left};
    let session_ramps = LiveRamps {
        fader_samples: 100,
        mute_samples: 300,
        pan_samples: 200,
        link_samples: 0,
    };
    let current = with_vca(0.0, 0.0, [false; 2]);
    let vca_target = EditRampTarget::Vca(current.vcas[0].id.clone());
    let first = track_id(&current, 0);
    let strips = |current: &SessionModel,
                  edit: fn(&mut SessionModel),
                  ramps: &EditRamps|
     -> Vec<TrackFaderRecord> {
        let (strips, _) = classify_both(current, edit, session_ramps, ramps).expect("live");
        assert_eq!(strips.len(), 1);
        assert_eq!(strips[0].0, first);
        strips[0].1.clone()
    };
    let mut ramp = EditRamps::new();
    ramp.set(&vca_target, EditRampRow::VcaFader, Some(480));
    assert_eq!(
        strips(&current, |next| next.vcas[0].fader.left_db = -6.0, &ramp),
        vec![fader_db(Left, -6.0, 480)]
    );
    // No edit length: `fader_ms`.
    assert_eq!(
        strips(
            &current,
            |next| next.vcas[0].fader.left_db = -6.0,
            &EditRamps::new()
        ),
        vec![fader_db(Left, -6.0, 100)]
    );
    // A mute takes the VCA mute's edit, else `mute_ms`.
    let mut mute_ramp = EditRamps::new();
    mute_ramp.set(&vca_target, EditRampRow::VcaMute, Some(48));
    assert_eq!(
        strips(
            &current,
            |next| next.vcas[0].fader.left_mute = true,
            &mute_ramp
        ),
        vec![mute(Left, true, 48)]
    );
    assert_eq!(
        strips(
            &current,
            |next| next.vcas[0].fader.left_mute = true,
            &EditRamps::new()
        ),
        vec![mute(Left, true, 300)]
    );
    // The strip's own edit first: own length 0 steps the lane the strip moved, and the VCA's 480
    // applies to the lane only it moved; the two lanes are two records.
    let mut both = EditRamps::new();
    both.set(&vca_target, EditRampRow::VcaFader, Some(480));
    both.set(
        &EditRampTarget::Strip(current.tracks[0].id.clone()),
        EditRampRow::Fader,
        Some(0),
    );
    assert_eq!(
        strips(
            &current,
            |next| {
                next.tracks[0].fader.left_db = -2.0;
                next.vcas[0].fader.right_db = -6.0;
            },
            &both
        ),
        vec![
            fader_db(Left, -2.0, 0),
            fader_db(BuiltinLaneSelector::Right, -6.0, 480)
        ]
    );
    // One value, one length: both lanes merge.
    assert_eq!(
        strips(
            &current,
            |next| {
                next.vcas[0].fader.left_db = -6.0;
                next.vcas[0].fader.right_db = -6.0;
            },
            &ramp
        ),
        vec![fader_db(Both, -6.0, 480)]
    );
    // A membership change has no edit length: the `VcaOffset` row's session length.
    let loud = with_vca(-6.0, -6.0, [false; 2]);
    let (changed, _) = classify_both(
        &loud,
        |next| next.vcas[0].members.clear(),
        session_ramps,
        &ramp,
    )
    .expect("live");
    assert_eq!(changed[0].1, vec![fader_db(Both, 0.0, 100)]);
}

/// #1558 gate 5. Red if a gate change from the source's mute ramps on its own: the send takes the
/// length of the source strip's `Mute` record of the same transaction, so a strip and the sends that
/// follow it ramp together; a gate change that the send's own `follows_mute` or `mute` edit caused
/// takes that edit's length.
#[test]
fn a_followed_gate_ramps_with_its_source_strips_mute() {
    let session_ramps = LiveRamps {
        fader_samples: 100,
        mute_samples: 300,
        pan_samples: 200,
        link_samples: 0,
    };
    let current = with_bus(true);
    let send = send_index(&current);
    let source = 0;
    assert!(matches!(
        &current.routes[send].source,
        RouteSource::Track { track_id, .. } if *track_id == current.tracks[source].id
    ));
    let lengths = |current: &SessionModel, edit: fn(&mut SessionModel), edit_ramps: &EditRamps| {
        let (strips, routes) =
            classify_both(current, edit, session_ramps, edit_ramps).expect("live");
        (
            strips
                .iter()
                .flat_map(|strip| strip.1.clone())
                .collect::<Vec<_>>(),
            routes.iter().map(|route| route.3).collect::<Vec<_>>(),
        )
    };
    // The source's own mute edit, 96 samples: the send follows at 96.
    let mut ramp = EditRamps::new();
    ramp.set(
        &EditRampTarget::Strip(current.tracks[source].id.clone()),
        EditRampRow::Mute,
        Some(96),
    );
    let (records, routes) = lengths(
        &current,
        |next| next.tracks[0].fader.left_mute = true,
        &ramp,
    );
    assert_eq!(records, vec![mute(BuiltinLaneSelector::Left, true, 96)]);
    assert_eq!(routes, vec![96]);
    // No edit length: `mute_ms` for both.
    let (_, routes) = lengths(
        &current,
        |next| next.tracks[0].fader.left_mute = true,
        &EditRamps::new(),
    );
    assert_eq!(routes, vec![300]);
    // The send's own `follows_mute` edit on a muted source takes its edit's length.
    let muted = edited(&current, |model| model.tracks[0].fader.left_mute = true);
    let mut route_ramp = EditRamps::new();
    route_ramp.set(
        &EditRampTarget::Route(current.routes[send].id.clone()),
        EditRampRow::RouteMute,
        Some(7),
    );
    let (records, routes) = lengths(
        &muted,
        |next| {
            let send = send_index(next);
            next.routes[send].follows_mute = false;
        },
        &route_ramp,
    );
    assert!(records.is_empty());
    assert_eq!(routes, vec![7]);
}

/// #1558 D7 (decision record, batch 3 fold). A route whose gain also changes in the transaction
/// that its source's mute gated it takes the source's `Mute` length, not the `SendGain` row's and
/// not the route's own `RouteGain` edit: the gate governs the record, so the send follows its source's
/// ramp. Red if a record that carries a gain change and a source-caused gate change takes the gain's
/// length, so the send would ramp apart from the strip that gates it.
#[test]
fn a_gain_change_with_a_source_caused_gate_change_takes_the_sources_mute_length() {
    let session_ramps = LiveRamps {
        fader_samples: 100,
        mute_samples: 300,
        pan_samples: 200,
        link_samples: 0,
    };
    let current = with_bus(true);
    let send = send_index(&current);
    let mut ramp = EditRamps::new();
    ramp.set(
        &EditRampTarget::Strip(current.tracks[0].id.clone()),
        EditRampRow::Mute,
        Some(96),
    );
    ramp.set(
        &EditRampTarget::Route(current.routes[send].id.clone()),
        EditRampRow::RouteGain,
        Some(7),
    );
    let (_, routes) = classify_both(
        &current,
        |next| {
            next.tracks[0].fader.left_mute = true;
            let send = send_index(next);
            next.routes[send].gain_db = -3.0;
        },
        session_ramps,
        &ramp,
    )
    .expect("live");
    assert_eq!(
        routes.iter().map(|route| route.3).collect::<Vec<_>>(),
        vec![96]
    );
}

/// Gate 1(k). Red if a structural edit is classified live.
#[test]
fn structural_edits_need_a_rebuild() {
    let current = two_tracks();
    let edits: [(&str, Edit); 3] = [
        ("track removed", |next| {
            next.tracks.pop();
        }),
        ("track added", |next| {
            let mut track = next.tracks[1].clone();
            track.id = StableId::parse("zz-added").expect("id");
            next.tracks.push(track);
        }),
        ("track renamed", |next| {
            next.tracks[1].id = StableId::parse("zz-renamed").expect("id");
        }),
    ];
    for (name, edit) in edits {
        assert_eq!(
            classify(&current, edit),
            Err(LiveRebuild::Structure),
            "{name}"
        );
    }
}

/// A fader ride on the first track's builtin strip (row 5, a rendered row since #1647).
fn fader_ride(model: &SessionModel) -> Automation {
    Automation {
        id: id("fader-ride"),
        target: ControlAddress {
            owner: Owner::Strip(model.tracks[0].id.clone()),
            section: Section::Builtins,
            control: 5,
            lane: ParameterChannel::Both,
        },
        segments: vec![AutomationSegment {
            shape: AutomationShape::Linear,
            start_sample: 0,
            end_sample: 960,
            start_value: 0.0,
            end_value: -3.0,
            unit: ParameterUnit::Db,
        }],
        offset: session::AutomationOffset::default(),
    }
}

/// A ride on the first track's pan (builtin 12, left lane) or, with `matrix`, on its `matrix_ll`
/// (builtin 7, a shared row): the rows `matrix_mode` flips between.
fn pan_ride(model: &SessionModel, matrix: bool) -> Automation {
    let mut ride = fader_ride(model);
    ride.id = id("pan-ride");
    ride.target.control = if matrix { 7 } else { 12 };
    ride.target.lane = if matrix {
        ParameterChannel::Both
    } else {
        ParameterChannel::Left
    };
    ride.segments[0].unit = ParameterUnit::Linear;
    ride.segments[0].start_value = 0.0;
    ride.segments[0].end_value = 0.5;
    ride
}

fn switch_to_matrix(next: &mut SessionModel) {
    next.tracks[0].matrix_or_pan = MatrixOrPan::Matrix {
        ll: 1.0,
        lr: 0.0,
        rl: 0.0,
        rr: 1.0,
        smoothing_samples: 0,
    };
}

/// #1631 gate 1 and 2 and 3. A mode switch that leaves a stored pan entry on an inactive row is a
/// rebuild (the row rule refuses it `control.inactive` at preparation, before any commit); the
/// same switch with the entry retargeted to the matrix row, or on a strip without such an entry,
/// stays live.
///
/// Red if step 2 runs only when the automation table changes: the switch is then live and the
/// committed model holds an entry that its next rebuild refuses.
#[test]
fn a_mode_switch_that_strands_an_automation_needs_a_rebuild() {
    let mut declared = two_tracks();
    let ride = pan_ride(&declared, false);
    declared.automation.push(ride);
    let current = normalized(&declared);
    assert!(
        controls::automation_diagnostics(&current, current.sample_rate_hz).is_empty(),
        "the pan entry is accepted on a pan strip"
    );

    assert_eq!(
        classify(&current, switch_to_matrix),
        Err(LiveRebuild::AutomationTarget),
        "a switch under a pan entry"
    );
    assert!(
        classify(&current, |next| {
            switch_to_matrix(next);
            next.automation[0] = pan_ride(next, true);
        })
        .is_ok(),
        "the switch with the entry retargeted to the matrix row"
    );
    assert!(
        classify(&current, |next| {
            switch_to_matrix(next);
            next.automation.clear();
        })
        .is_ok(),
        "the switch with the entry removed"
    );

    let mut plain = two_tracks();
    plain.automation.push(fader_ride(&plain));
    let plain = normalized(&plain);
    assert!(
        classify(&plain, switch_to_matrix).is_ok(),
        "a strip with no pan or matrix entry switches live"
    );
    let mut other = two_tracks();
    let mut ride = pan_ride(&other, false);
    ride.target.owner = Owner::Strip(other.tracks[1].id.clone());
    other.automation.push(ride);
    assert!(
        classify(&normalized(&other), switch_to_matrix).is_ok(),
        "an entry on another strip is not stranded"
    );
    // The reverse: a matrix entry on a matrix strip, switched back to pan.
    let mut matrix = two_tracks();
    switch_to_matrix(&mut matrix);
    let ride = pan_ride(&matrix, true);
    matrix.automation.push(ride);
    let matrix = normalized(&matrix);
    assert_eq!(
        classify(&matrix, |next| {
            next.tracks[0].matrix_or_pan = MatrixOrPan::Pan {
                left: 0.0,
                right: 0.0,
                smoothing_samples: 0,
            };
        }),
        Err(LiveRebuild::AutomationTarget),
        "a switch back to pan under a matrix entry"
    );
}

/// #1645 fold. A delta that only moves a static LPF under a stored HPF ride (the automation table
/// unchanged) routes to the rebuild when the order rule refuses `next`: an LPF of 500 Hz under a
/// ride to 900 Hz. An LPF of 2 kHz, an LPF edit on a strip with no filter automation, and the same
/// crossing edit on a strip whose ride is on the fader stay live.
///
/// Red if step 2 re-checks the row rule only when the automation table or a row's activity
/// changes: the edit commits live, acked, and its next rebuild refuses it.
#[test]
fn a_static_filter_edit_that_crosses_an_automated_curve_needs_a_rebuild() {
    let hpf_ride = |model: &SessionModel| {
        let mut ride = fader_ride(model);
        ride.id = id("hpf-ride");
        ride.target.control = 3;
        ride.segments[0].unit = ParameterUnit::Hz;
        ride.segments[0].start_value = 100.0;
        ride.segments[0].end_value = 900.0;
        ride
    };
    let lpf_both = |hz: f32| {
        move |next: &mut SessionModel| {
            next.tracks[0].builtins.left.lpf_hz = hz;
            next.tracks[0].builtins.right.lpf_hz = hz;
        }
    };
    let mut declared = two_tracks();
    declared.tracks[0].builtins.left.lpf_hz = 1_000.0;
    declared.tracks[0].builtins.right.lpf_hz = 1_000.0;
    let mut automated = declared.clone();
    let ride = hpf_ride(&automated);
    automated.automation.push(ride);
    let automated = normalized(&automated);
    assert_eq!(
        classify(&automated, lpf_both(500.0)),
        Err(LiveRebuild::AutomationTarget),
        "a static LPF below the HPF ride"
    );
    assert_eq!(
        classify(&automated, |next| {
            next.tracks[0].builtins.right.lpf_hz = 500.0;
        }),
        Err(LiveRebuild::AutomationTarget),
        "one lane is enough"
    );
    assert!(
        classify(&automated, lpf_both(2_000.0)).is_ok(),
        "a static LPF above the ride"
    );
    let mut fader = declared;
    let ride = fader_ride(&fader);
    fader.automation.push(ride);
    let fader = normalized(&fader);
    assert!(
        classify(&fader, lpf_both(500.0)).is_ok(),
        "no filter automation: the static rule alone"
    );
}

/// The model-only edits: #1260 D1's four and #1365 D4's `control_smoothing`. The automation upsert
/// is an entry on `with_effects`' EQ band-1 gain, a valid block-rate target whose row does not
/// render until #1667, the last row to leave the mask (#1648 hazard): a fader entry is a rebuild
/// (#1648 D1).
fn model_only_edits() -> [(&'static str, Edit); 5] {
    [
        ("session id", |next| next.session_id = id("another-session")),
        ("render profile id", |next| {
            next.render_profile.id = id("another-render-profile");
        }),
        ("output profile id", |next| {
            next.output_profile.id = id("another-output-profile");
        }),
        ("automation upsert", |next| {
            let ride = eq_automation(next, 4, ParameterUnit::Db);
            next.automation.push(ride);
        }),
        ("control smoothing", |next| {
            next.control_smoothing = Some(ControlSmoothing {
                mute_ms: 1.0,
                fader_ms: 2.0,
                pan_ms: 3.0,
            });
        }),
    ]
}

/// #1260 gate 1. Red if a model-only field is still compared, or if one masks a live record.
#[test]
fn model_only_edits_are_live_with_no_records() {
    use BuiltinLaneSelector::Left;
    let current = with_effects();
    assert!(current.automation.is_empty());
    for (name, edit) in model_only_edits() {
        assert_eq!(classify(&current, edit), Ok(Vec::new()), "{name}");
        assert_eq!(
            classify(&current, |next| {
                edit(next);
                next.tracks[0].fader.left_db = -6.0;
            }),
            Ok(vec![(
                track_id(&current, 0),
                vec![fader_db(Left, -6.0, 0)],
                None
            )]),
            "{name} with a fader move"
        );
    }
    // All of them together, with the fader move.
    assert_eq!(
        classify(&current, |next| {
            for (_, edit) in model_only_edits() {
                edit(next);
            }
            next.tracks[0].fader.left_db = -6.0;
        }),
        Ok(vec![(
            track_id(&current, 0),
            vec![fader_db(Left, -6.0, 0)],
            None
        )])
    );
    // Removing stored automation of a row that does not render is model-only too.
    let mut automated = current.clone();
    automated
        .automation
        .push(eq_automation(&current, 4, ParameterUnit::Db));
    assert_eq!(
        classify(&automated, |next| next.automation.clear()),
        Ok(Vec::new())
    );
}

/// `fader_ride` on the first track's left lane only.
fn left_fader_ride(model: &SessionModel) -> Automation {
    let mut ride = fader_ride(model);
    ride.target.lane = ParameterChannel::Left;
    ride
}

/// #1648 gate 1 (as amended by the batch 17 spec check, departure 5). A fader entry (builtin row
/// 5, a row that renders) added, removed, given other segments, retargeted or renamed is
/// `Automation`; a change of its hand offset alone is not (departure 3). A mute entry (row 6,
/// which renders since #1651 T3a) added, changed or removed is `Automation` too. A static change
/// together with an automation change is a rebuild. A fader entry in unit `linear` keeps
/// `AutomationTarget` (#1645 D5), so the row rule runs before D1's check.
///
/// Test value: red if the mask still hides fader or mute entries (an edit is acked and never
/// heard), if the comparison reads the offset (a hand move would rebuild), or if D1's check runs
/// before the refusal route.
#[test]
fn a_fader_automation_edit_is_a_rebuild() {
    let current = two_tracks();
    let automated = normalized(&edited(&current, |model| {
        let ride = fader_ride(model);
        model.automation.push(ride);
    }));
    let cases: [(&str, &SessionModel, Edit); 7] = [
        ("added", &current, |next| {
            let ride = fader_ride(next);
            next.automation.push(ride);
        }),
        ("added on one lane", &current, |next| {
            let ride = left_fader_ride(next);
            next.automation.push(ride);
        }),
        ("removed", &automated, |next| next.automation.clear()),
        ("other segments", &automated, |next| {
            next.automation[0].segments[0].end_value = -6.0;
        }),
        ("a zero's sign", &automated, |next| {
            next.automation[0].segments[0].start_value = -0.0;
        }),
        ("retargeted to the other track", &automated, |next| {
            next.automation[0].target.owner = Owner::Strip(next.tracks[1].id.clone());
        }),
        ("renamed", &automated, |next| {
            next.automation[0].id = id("another-ride");
        }),
    ];
    for (name, base, edit) in cases {
        assert_eq!(classify(base, edit), Err(LiveRebuild::Automation), "{name}");
    }
    // The hand offset is not part of the curve (#1650 D5 makes it a live write of its own).
    assert!(
        classify(&automated, |next| {
            next.automation[0].offset.left_db = 2.0;
        })
        .is_ok(),
        "an offset-only change is not a rebuild"
    );
    // The mute row renders too (#1651 T3a): a mute entry added, changed and removed is a rebuild.
    let mute_ride = |model: &SessionModel| {
        let mut ride = fader_ride(model);
        ride.id = id("mute-ride");
        ride.target.control = 6;
        ride.segments[0] = AutomationSegment {
            shape: AutomationShape::Step,
            start_sample: 0,
            end_sample: 960,
            start_value: 1.0,
            end_value: 1.0,
            unit: ParameterUnit::Linear,
        };
        ride
    };
    let muted = normalized(&edited(&current, |model| {
        let ride = mute_ride(model);
        model.automation.push(ride);
    }));
    assert_eq!(
        classify(&current, |next| {
            let ride = mute_ride(next);
            next.automation.push(ride);
        }),
        Err(LiveRebuild::Automation),
        "a mute entry added"
    );
    assert_eq!(
        classify(&muted, |next| next.automation[0].segments[0].end_sample =
            1_920),
        Err(LiveRebuild::Automation),
        "a mute entry changed"
    );
    assert_eq!(
        classify(&muted, |next| next.automation.clear()),
        Err(LiveRebuild::Automation),
        "a mute entry removed"
    );
    // A static change in the same transaction as an automation change is a rebuild.
    assert!(
        classify(&current, |next| {
            next.tracks[0].fader.left_db = -6.0;
            let ride = fader_ride(next);
            next.automation.push(ride);
        })
        .is_err(),
        "a static change with an automation change"
    );
    // A fader entry the row refuses keeps the refusal's route (#1645 D5).
    assert_eq!(
        classify(&current, |next| {
            let mut ride = fader_ride(next);
            ride.segments[0].unit = ParameterUnit::Linear;
            next.automation.push(ride);
        }),
        Err(LiveRebuild::AutomationTarget),
        "a fader entry in unit linear"
    );
}

/// One strip's level records, owned: its ID, its fader/mute records in push order and its
/// offsets-cell writes `[left, right]` as `(S bits, ramp)` (#1649 D4).
type Level = (String, Vec<TrackFaderRecord>, [Option<(u64, u32)>; 2]);

fn level(delta: &LiveDelta<'_>) -> Vec<Level> {
    delta
        .strips
        .iter()
        .map(|strip| {
            (
                strip.strip_id.to_owned(),
                strip.fader_records().collect(),
                strip.offsets.map(|record| {
                    record.map(|record| (record.offsets_sum.to_bits(), record.smoothing_samples))
                }),
            )
        })
        .collect()
}

/// `(S bits, ramp)` of an offsets-cell write.
fn sum(offsets_sum: f64, ramp: u32) -> Option<(u64, u32)> {
    Some((offsets_sum.to_bits(), ramp))
}

/// #1648 D3 and #1649 D4, gate 5, in the shared level lowering: a VCA edit gives a lane that
/// `next` automates no `FaderDb` and, when the lane's `S` changes, one offsets-cell write of the
/// new `S` with the lane's fader length (departure 1: the one length rule), on the full
/// classifier and the fast path alike. Track 0 rides its fader on both lanes and is a member of
/// `vca`; track 1 is a member with no automation, and takes one `FaderDb` per lane as before.
///
/// Test value: red if a VCA move does not reach an automated lane's offsets cell (an acked ride
/// that is never heard), if a static `FaderDb` still lands on it (a record that fights the curve),
/// if the automated and the plain member take different lengths, if an unchanged `S` still
/// writes, if a nested VCA's `S` is summed in another order than `controls::effective` (the one
/// author of `S`), or if the fast path decides otherwise than the full path.
#[test]
fn a_vca_move_writes_the_automated_lanes_offsets_cell() {
    use BuiltinLaneSelector::{Both, Left, Right};
    let session_ramps = LiveRamps {
        fader_samples: 100,
        mute_samples: 300,
        pan_samples: 200,
        link_samples: 0,
    };
    let current = normalized(&edited(&with_vca(0.0, 0.0, [false; 2]), |model| {
        let second = model.tracks[1].id.clone();
        model.vcas[0].members.push(second);
        let ride = fader_ride(model);
        model.automation.push(ride);
    }));
    let (first, second) = (track_id(&current, 0), track_id(&current, 1));
    let run = |current: &SessionModel, edit: &dyn Fn(&mut SessionModel), ramps: &EditRamps| {
        let next = edited(current, edit);
        classify_live_delta(current, &next, session_ramps, ramps).map(|delta| level(&delta))
    };
    let both = |next: &mut SessionModel| {
        next.vcas[0].fader.left_db = -6.0;
        next.vcas[0].fader.right_db = -6.0;
    };
    assert_eq!(
        run(&current, &both, &EditRamps::new()),
        Ok(vec![
            (first.clone(), Vec::new(), [sum(-6.0, 100), sum(-6.0, 100)]),
            (second.clone(), vec![fader_db(Both, -6.0, 100)], [None; 2]),
        ]),
        "the automated member's cell and the plain member's record, at one length"
    );
    // The edit's own length reaches both members alike.
    let mut ramp = EditRamps::new();
    ramp.set(
        &EditRampTarget::Vca(current.vcas[0].id.clone()),
        EditRampRow::VcaFader,
        Some(256),
    );
    assert_eq!(
        run(&current, &both, &ramp),
        Ok(vec![
            (first.clone(), Vec::new(), [sum(-6.0, 256), sum(-6.0, 256)]),
            (second.clone(), vec![fader_db(Both, -6.0, 256)], [None; 2]),
        ])
    );
    assert_eq!(
        run(
            &current,
            &|next: &mut SessionModel| next.vcas[0].fader.left_db = -6.0,
            &EditRamps::new()
        ),
        Ok(vec![
            (first.clone(), Vec::new(), [sum(-6.0, 100), None]),
            (second.clone(), vec![fader_db(Left, -6.0, 100)], [None; 2]),
        ]),
        "one lane's move writes that lane's cell"
    );
    assert_eq!(
        run(
            &current,
            &|next: &mut SessionModel| {
                next.vcas[0].fader.left_db = -6.0;
                next.vcas[0].fader.left_mute = true;
            },
            &EditRamps::new()
        ),
        Ok(vec![
            (
                first.clone(),
                vec![mute(Left, true, 300)],
                [sum(-6.0, 100), None]
            ),
            (
                second.clone(),
                vec![fader_db(Left, -6.0, 100), mute(Left, true, 300)],
                [None; 2]
            ),
        ]),
        "the automated lane's mute keeps its record"
    );
    // Gate 3: a move to the current value, or of a VCA that reaches neither track, writes nothing.
    assert_eq!(
        run(
            &current,
            &|next: &mut SessionModel| next.vcas[0].fader.left_db = 0.0,
            &EditRamps::new()
        ),
        Ok(Vec::new())
    );
    let lone = normalized(&edited(&current, |model| {
        let mut other = vca();
        other.id = id("other");
        model.vcas.push(other);
    }));
    assert_eq!(
        run(
            &lone,
            &|next: &mut SessionModel| {
                let other = next.vcas.iter_mut().find(|vca| vca.id == id("other"));
                other.expect("other").fader.left_db = -6.0;
            },
            &EditRamps::new()
        ),
        Ok(Vec::new())
    );
    // A lane the entry does not drive keeps its `FaderDb` (#1648 D3).
    let left_only = normalized(&edited(&with_vca(0.0, 0.0, [false; 2]), |model| {
        let ride = left_fader_ride(model);
        model.automation.push(ride);
    }));
    assert_eq!(
        run(&left_only, &both, &EditRamps::new()),
        Ok(vec![(
            first.clone(),
            vec![fader_db(Right, -6.0, 100)],
            [sum(-6.0, 100), None]
        )]),
    );

    // Three nested VCAs reach track 0, declared and nested out of ID order, offsets 24, -24 and
    // 2e-30 in ascending ID order. Moving the third to 1e-30 writes the `S` a fresh plan of the
    // edited session computes: `(24 + -24) + 1e-30`, not the `0.0` of an order that adds `1e-30`
    // before `-24`.
    let nested = normalized(&edited(&two_tracks(), |model| {
        let track = model.tracks[0].id.clone();
        let vca = |name: &str, db: f32, members: Vec<StableId>| Vca {
            id: id(name),
            fader: DualMonoFader {
                left_db: db,
                right_db: db,
                left_mute: false,
                right_mute: false,
            },
            members,
        };
        // `vca-c` holds the track, `vca-a` holds `vca-c`, `vca-b` holds `vca-a`.
        model.vcas = vec![
            vca("vca-c", 2e-30, vec![track]),
            vca("vca-b", -24.0, vec![id("vca-a")]),
            vca("vca-a", 24.0, vec![id("vca-c")]),
        ];
        let ride = fader_ride(model);
        model.automation.push(ride);
    }));
    let edit = |next: &mut SessionModel| {
        let third = next
            .vcas
            .iter_mut()
            .find(|vca| vca.id == id("vca-c"))
            .expect("vca-c");
        third.fader.left_db = 1e-30;
        third.fader.right_db = 1e-30;
    };
    let after = edited(&nested, edit);
    let fresh =
        controls::effective(&after, &controls::MonitorOverlay::default()).strips[0].offsets_sum[0];
    assert_eq!(fresh.to_bits(), f64::from(1e-30_f32).to_bits(), "ID order");
    assert_ne!(
        fresh.to_bits(),
        ((f64::from(24.0_f32) + f64::from(1e-30_f32)) + f64::from(-24.0_f32)).to_bits(),
        "another order gives other bits"
    );
    assert_eq!(
        run(&nested, &edit, &EditRamps::new()),
        Ok(vec![(
            first.clone(),
            Vec::new(),
            [sum(fresh, 100), sum(fresh, 100)]
        )])
    );

    #[cfg(feature = "test-support")]
    {
        let next = edited(&current, both);
        let rows = [ControlAddress {
            owner: Owner::Vca(id("vca")),
            section: Section::Own,
            control: 1,
            lane: ParameterChannel::Both,
        }];
        let full = classify_live_delta(&current, &next, session_ramps, &EditRamps::new());
        let _ = host_core::take_fast_classification_count();
        let fast = host_core::classify_live_edit(
            &current,
            &next,
            session_ramps,
            &EditRamps::new(),
            host_core::MonitorBase::NONE,
            Some(&rows),
        );
        assert_eq!(
            host_core::take_fast_classification_count(),
            1,
            "the VCA move takes the fast path"
        );
        let fast = fast.map(|mut delta| {
            delta.derived_owners = None;
            delta
        });
        assert_eq!(fast, full, "the fast path writes the same cells");
    }
}

/// #1650b D3, D5 and root's ruling R17-1, in the shared classifier. Track 0 rides its fader on
/// both lanes under a VCA at -2 dB. A hand move (`SET_CONTROL fader_db`) on its left lane writes the
/// ride's left offset, not the static fader, and is live with one offsets-cell write of
/// `S = offset + VCA` on that lane, with the `Fader` row's length (the edit's own 256 samples, not
/// the VCA row's), and no `FaderDb`. An offset-only `UpsertAutomation` is the same live write. A
/// change of an automated lane's static fader (the fallback) is a rebuild. The fast path over the
/// whole model, and over the in-place view whose owners hold the ride (`controls::OwnerEdits`),
/// give the full classifier's delta.
///
/// Test value: red if the hand move lands on the static fader (`model_only` or a `FaderDb` the
/// curve overrides), if `S` leaves out the hand offset or adds it after the VCAs (two VCAs and a
/// hand offset below half an ulp of their sum's first term), if the move takes
/// the VCA row's length, if a static fallback change commits live without being heard, or if the
/// in-place view reads the lane's committed offset instead of the edited one.
#[test]
fn a_hand_move_on_an_automated_fader_writes_the_lanes_offsets_cell() {
    let session_ramps = LiveRamps {
        fader_samples: 100,
        mute_samples: 300,
        pan_samples: 200,
        link_samples: 0,
    };
    let current = normalized(&edited(&with_vca(-2.0, -2.0, [false; 2]), |model| {
        let ride = fader_ride(model);
        model.automation.push(ride);
    }));
    let first = track_id(&current, 0);
    let rate = current.sample_rate_hz;
    let fader = |lane| ControlAddress {
        owner: Owner::Strip(current.tracks[0].id.clone()),
        section: Section::Builtins,
        control: 5,
        lane,
    };
    let mut ramp = EditRamps::new();
    ramp.set(
        &EditRampTarget::Strip(current.tracks[0].id.clone()),
        EditRampRow::Fader,
        Some(256),
    );
    let next = edited(&current, |next| {
        controls::set(next, &fader(ParameterChannel::Left), 3.0, Some(256), rate)
            .expect("a hand move");
    });
    assert_eq!(
        next.automation[0].offset.left_db.to_bits(),
        3.0_f32.to_bits()
    );
    assert_eq!(
        next.tracks[0].fader, current.tracks[0].fader,
        "the fallback stays"
    );
    let s = f64::from(3.0_f32) + f64::from(-2.0_f32);
    let full = classify_live_delta(&current, &next, session_ramps, &ramp);
    assert_eq!(
        full.as_ref().map(level),
        Ok(vec![(first.clone(), Vec::new(), [sum(s, 256), None])]),
        "one offsets-cell write, the edit's length"
    );
    // An offset-only upsert of the ride is the same write, with the session's fader length.
    assert_eq!(
        classify_live_delta(
            &current,
            &edited(&current, |next| next.automation[0].offset.right_db = -1.5),
            session_ramps,
            &EditRamps::new(),
        )
        .map(|delta| level(&delta)),
        Ok(vec![(
            first.clone(),
            Vec::new(),
            [None, sum(f64::from(-1.5_f32) + f64::from(-2.0_f32), 100)]
        )])
    );
    // The static fader of an automated lane is no control value: a change of it is structure.
    assert_eq!(
        classify_live_delta(
            &current,
            &edited(&current, |next| next.tracks[0].fader.left_db = -9.0),
            session_ramps,
            &EditRamps::new(),
        )
        .map(|delta| level(&delta)),
        Err(LiveRebuild::Structure)
    );
    // D6 order (U3 fold, batch 17): two VCAs at +24 and -24 dB reach the track, and a hand move of
    // 1e-10 dB is below half an ulp of 24 in `f64`. `S` adds the hand offset first, then the VCAs
    // in ascending ID order: `(1e-10 + 24) + -24`, which keeps only the rounded part of 1e-10; the
    // order that adds the hand offset last gives `1e-10` itself.
    let two_vcas = normalized(&edited(&current, |model| {
        let track = model.tracks[0].id.clone();
        model.vcas = vec![
            Vca {
                id: id("vca-a"),
                fader: DualMonoFader {
                    left_db: 24.0,
                    right_db: 24.0,
                    left_mute: false,
                    right_mute: false,
                },
                members: vec![track.clone()],
            },
            Vca {
                id: id("vca-b"),
                fader: DualMonoFader {
                    left_db: -24.0,
                    right_db: -24.0,
                    left_mute: false,
                    right_mute: false,
                },
                members: vec![track],
            },
        ];
    }));
    let tiny = 1e-10_f32;
    let ordered = (f64::from(tiny) + 24.0) + -24.0;
    assert_ne!(
        ordered.to_bits(),
        f64::from(tiny).to_bits(),
        "the two orders differ"
    );
    assert_eq!(
        classify_live_delta(
            &two_vcas,
            &edited(&two_vcas, |next| {
                controls::set(next, &fader(ParameterChannel::Left), tiny, None, rate)
                    .expect("a hand move");
            }),
            session_ramps,
            &EditRamps::new(),
        )
        .map(|delta| level(&delta)),
        Ok(vec![(first.clone(), Vec::new(), [sum(ordered, 100), None])]),
        "the hand offset first, then the VCAs"
    );

    #[cfg(feature = "test-support")]
    {
        let rows = [fader(ParameterChannel::Left)];
        let full = full.map(|delta| level(&delta));
        let _ = host_core::take_fast_classification_count();
        let fast = host_core::classify_live_edit(
            &current,
            &next,
            session_ramps,
            &ramp,
            host_core::MonitorBase::NONE,
            Some(&rows),
        );
        assert_eq!(
            host_core::take_fast_classification_count(),
            1,
            "the fast path"
        );
        assert_eq!(
            fast.map(|delta| level(&delta)),
            full,
            "the fast path, whole"
        );
        // R17-1: the in-place owners hold the ride the move wrote, and the view reads it.
        let mut owners = controls::OwnerEdits::new(&current);
        owners
            .set(&fader(ParameterChannel::Left), 3.0, Some(256), rate)
            .expect("an in-place hand move");
        let (owners, slots) = owners.finish();
        assert_eq!(
            slots.automation.len(),
            1,
            "the ride is an owner of the edit"
        );
        let view = controls::ModelView::edited(&current, &owners);
        let mut dense = view.to_model();
        dense.revision = next.revision;
        assert_eq!(dense, next, "the view is the edited model");
        let staged = host_core::classify_live_staged(
            &current,
            view,
            session_ramps,
            &ramp,
            host_core::MonitorBase::NONE,
            &rows,
        )
        .expect("the fast path judges it");
        assert_eq!(Ok(level(&staged)), full, "the fast path, in place");
    }
}

/// #1260 gate 1. Red if a profile field that preparation reads is masked with its profile's ID.
#[test]
fn an_output_profile_channel_change_is_structural() {
    let current = two_tracks();
    assert_eq!(
        classify(&current, |next| {
            next.output_profile.channels = 1;
        }),
        Err(LiveRebuild::Structure)
    );
}

/// `model` with the session's `control_smoothing` set (#1054).
fn with_smoothing(
    model: &SessionModel,
    control_smoothing: Option<ControlSmoothing>,
) -> SessionModel {
    let mut model = model.clone();
    model.control_smoothing = control_smoothing;
    model
}

/// The records of one strip whose fader, mute and pan all change, under `control_smoothing`, at
/// the fixture's 48 kHz: `[FaderDb, Mute, matrix]` ramps, the matrix one for a pan that carries
/// `pan_smoothing_samples` of its own.
fn ramps_of_a_move(
    control_smoothing: Option<ControlSmoothing>,
    pan_smoothing_samples: u32,
) -> [u32; 3] {
    ramps_of_a_move_with(control_smoothing, pan_smoothing_samples, &EditRamps::new())
}

/// [`ramps_of_a_move`] for a transaction whose edits carried `edit_ramps` (#1394).
fn ramps_of_a_move_with(
    control_smoothing: Option<ControlSmoothing>,
    pan_smoothing_samples: u32,
    edit_ramps: &EditRamps,
) -> [u32; 3] {
    use BuiltinLaneSelector::Both;
    let current = with_smoothing(&two_tracks(), control_smoothing);
    let next = edited(&current, |next| {
        let track = &mut next.tracks[0];
        track.fader.left_db = -6.0;
        track.fader.right_db = -6.0;
        track.fader.left_mute = true;
        track.fader.right_mute = true;
        track.matrix_or_pan = MatrixOrPan::Pan {
            left: 0.5,
            right: 0.5,
            smoothing_samples: pan_smoothing_samples,
        };
    });
    let ramps = LiveRamps::for_session(&next);
    let delta = classify_live_delta(&current, &next, ramps, edit_ramps).expect("live");
    let [Some(db), Some(muted), None, None] = delta.strips[0].fader else {
        panic!(
            "one FaderDb and one Mute record: {:?}",
            delta.strips[0].fader
        );
    };
    let matrix = delta.strips[0].matrix.expect("a matrix record");
    assert_eq!(matrix.matrix, pan_matrix(0.5, 0.5).expect("pan"));
    let TrackFaderRecord::FaderDb {
        smoothing_samples: fader,
        ..
    } = db
    else {
        unreachable!()
    };
    let TrackFaderRecord::Mute {
        smoothing_samples: muted_ramp,
        ..
    } = muted
    else {
        unreachable!()
    };
    assert_eq!(db, fader_db(Both, -6.0, fader));
    assert_eq!(muted, mute(Both, true, muted_ramp));
    let muted = muted_ramp;
    [fader, muted, matrix.smoothing_samples]
}

/// Gate 3. Red if the classifier keeps a pan window of 0, ignores the session for one row, or maps
/// a row to the wrong key: the session's `control_smoothing` (or the default table) is what the
/// fader, mute and matrix records carry at 48 kHz, and a pan's own non-zero window still wins.
#[test]
fn records_carry_the_session_ramps_and_the_models_own_pan_window() {
    // The default table: fader 20 ms, mute 10 ms, pan 20 ms.
    assert_eq!(ramps_of_a_move(None, 0), [960, 480, 960]);
    assert_eq!(ramps_of_a_move(None, 96), [960, 480, 96]);
    // Three different keys, so that a row cannot read its neighbour's.
    let explicit = Some(ControlSmoothing {
        mute_ms: 5.0,
        fader_ms: 15.0,
        pan_ms: 25.0,
    });
    assert_eq!(ramps_of_a_move(explicit, 0), [720, 240, 1200]);
    assert_eq!(ramps_of_a_move(explicit, 96), [720, 240, 96]);
    // An explicit 0 is the hard switch, never "use the default".
    let steps = Some(ControlSmoothing {
        mute_ms: 0.0,
        fader_ms: 0.0,
        pan_ms: 0.0,
    });
    assert_eq!(ramps_of_a_move(steps, 0), [0, 0, 0]);
    // A session-wide hard switch does not override a pan that asks for its own window.
    assert_eq!(ramps_of_a_move(steps, 96), [0, 0, 96]);
}

/// #1394 gate 4. Red if the classifier ignores an edit's ramp, reads an explicit 0 as "use the
/// default", lets a model pan window beat the edit's, or applies one row's or one strip's length
/// to another.
#[test]
fn a_records_length_is_its_edits_own_when_the_edit_carried_one() {
    let models = two_tracks();
    let (first, second) = (models.tracks[0].id.clone(), models.tracks[1].id.clone());
    let ramps = |entries: &[(&StableId, EditRampRow, u32)]| {
        let mut ramps = EditRamps::new();
        for (strip, row, samples) in entries {
            ramps.set(
                &EditRampTarget::Strip((*strip).clone()),
                *row,
                Some(*samples),
            );
        }
        ramps
    };
    use EditRampRow::{Fader, Input, Link, Matrix, Mute};
    let default_table = [960, 480, 960];

    // Entries on the fader and matrix rows: those two records take 37, the mute keeps the
    // session's 10 ms, and the model's own pan window of 96 does not beat the edit.
    let fader_and_matrix = ramps(&[(&first, Fader, 37), (&first, Matrix, 37)]);
    assert_eq!(
        ramps_of_a_move_with(None, 0, &fader_and_matrix),
        [37, 480, 37]
    );
    assert_eq!(
        ramps_of_a_move_with(None, 96, &fader_and_matrix),
        [37, 480, 37]
    );
    // An explicit 0 steps that record only; it is not "unset", whatever the model's window says.
    let explicit_mute = ramps(&[(&first, Mute, 0)]);
    assert_eq!(ramps_of_a_move_with(None, 0, &explicit_mute), [960, 0, 960]);
    assert_eq!(
        ramps_of_a_move_with(None, 96, &ramps(&[(&first, Matrix, 0)])),
        [960, 480, 0]
    );
    // An edit's length beats an explicit session table too.
    let table = Some(ControlSmoothing {
        mute_ms: 5.0,
        fader_ms: 15.0,
        pan_ms: 25.0,
    });
    assert_eq!(
        ramps_of_a_move_with(table, 0, &fader_and_matrix),
        [37, 240, 37]
    );
    // Another strip's entries, and the rows this move does not write, change nothing.
    let others = ramps(&[
        (&second, Fader, 37),
        (&second, Mute, 37),
        (&second, Matrix, 37),
        (&first, Input, 37),
        (&first, Link, 37),
    ]);
    assert_eq!(ramps_of_a_move_with(None, 0, &others), default_table);
}

/// Gate 3, the raw matrix. Red if a raw matrix with no window of its own is given 0 instead of the
/// session's `pan_ms` (D5 covers a pan and a matrix alike).
#[test]
fn a_raw_matrix_without_a_window_takes_the_session_pan_ramp() {
    let explicit = Some(ControlSmoothing {
        mute_ms: 5.0,
        fader_ms: 15.0,
        pan_ms: 25.0,
    });
    let current = with_smoothing(&two_tracks(), explicit);
    let moved = |smoothing_samples| {
        let mut current = current.clone();
        current.tracks[0].matrix_or_pan = MatrixOrPan::Matrix {
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
            smoothing_samples,
        };
        let next = edited(&current, |next| {
            next.tracks[0].matrix_or_pan = MatrixOrPan::Matrix {
                ll: 0.25,
                lr: -0.5,
                rl: 0.5,
                rr: 1.0,
                smoothing_samples,
            };
        });
        let delta = classify_live_delta(
            &current,
            &next,
            LiveRamps::for_session(&next),
            &EditRamps::new(),
        )
        .expect("a matrix move is live");
        delta.strips[0]
            .matrix
            .expect("a matrix record")
            .smoothing_samples
    };
    assert_eq!(moved(0), 1200);
    assert_eq!(moved(64), 64);
}

/// Gate 3, the row table. Red if a row reads another row's key, or the polarity row gets the plain
/// mute length instead of twice it. The expected values are written from `docs/SESSION_SCHEMA_V1.md`'s
/// table, one per row. The rows come from walking `next_row`, so the test visits every row it can
/// reach from `Fader`; a new row fails to compile in both exhaustive `match`es (`expected` and
/// `next_row`) until someone classifies it and chains it in.
#[test]
fn for_row_returns_the_key_field_of_every_row() {
    use LiveRampRow::*;
    // At 48 kHz, 5 ms is 240 samples (mute), 15 ms is 720 (fader), 25 ms is 1200 (pan).
    const MUTE: u32 = 240;
    const FADER: u32 = 720;
    const PAN: u32 = 1200;
    fn expected(row: LiveRampRow) -> u32 {
        match row {
            Fader | InputTrim | SendGain | VcaOffset | LinkGlide => FADER,
            Mute | Solo | SendMute | SendFollowsMute | VcaMute | BypassCrossfade => MUTE,
            PolarityInvert => 2 * MUTE,
            Pan | Matrix | SendMatrix => PAN,
        }
    }
    /// The next row in the walk. Exhaustive, so a new row must be given an arm; the walk visits it
    /// only if some arm chains to it, and `None` ends the walk.
    fn next_row(row: LiveRampRow) -> Option<LiveRampRow> {
        match row {
            Fader => Some(InputTrim),
            InputTrim => Some(SendGain),
            SendGain => Some(VcaOffset),
            VcaOffset => Some(LinkGlide),
            LinkGlide => Some(Mute),
            Mute => Some(Solo),
            Solo => Some(PolarityInvert),
            PolarityInvert => Some(SendMute),
            SendMute => Some(SendFollowsMute),
            SendFollowsMute => Some(VcaMute),
            VcaMute => Some(BypassCrossfade),
            BypassCrossfade => Some(Pan),
            Pan => Some(Matrix),
            Matrix => Some(SendMatrix),
            SendMatrix => None,
        }
    }
    let explicit = with_smoothing(
        &two_tracks(),
        Some(ControlSmoothing {
            mute_ms: 5.0,
            fader_ms: 15.0,
            pan_ms: 25.0,
        }),
    );
    let ramps = LiveRamps::for_session(&explicit);
    assert_eq!(
        (
            ramps.mute_samples,
            ramps.fader_samples,
            ramps.pan_samples,
            ramps.link_samples
        ),
        (MUTE, FADER, PAN, FADER)
    );
    let mut row = Some(Fader);
    while let Some(current) = row {
        assert_eq!(ramps.for_row(current), expected(current), "{current:?}");
        row = next_row(current);
    }
    // The default table: polarity is 20 ms, twice the mute's 10 ms.
    let defaults = LiveRamps::for_session(&two_tracks());
    assert_eq!(defaults.for_row(PolarityInvert), 960);
    assert_eq!(defaults.for_row(Mute), 480);
    assert_eq!(defaults.for_row(LinkGlide), 960);
}

/// Gate 2. Red if the classifier admits a fader dB or a matrix coefficient the render-side setter
/// refuses (the drain would pop the record, fail the render call and lose an acked edit), or
/// refuses one it accepts.
#[test]
fn the_classifier_domain_is_the_render_setters() {
    let width = BankWidth::ALL[0];
    let mut faders =
        BuiltinFaderBank::new(width.backend(), width, vec![BuiltinParameters::default()])
            .expect("fader bank");
    let mut matrices =
        BuiltinMatrixBank::new(width.backend(), width, vec![(Matrix2x2::IDENTITY, 0)])
            .expect("matrix bank");
    let current = two_tracks();
    for db in [
        -144.0_f32,
        24.0,
        -0.0,
        0.0,
        (-144.0_f32).next_down(),
        24.0_f32.next_up(),
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
    ] {
        let live = classify(&current, |next| next.tracks[0].fader.left_db = db).is_ok();
        let setter = faders
            .set_fader_db(0, BuiltinLaneSelector::Left, db, 0)
            .is_ok();
        assert_eq!(live, setter, "fader {db}");
    }
    for value in [
        -1.0_f32,
        1.0,
        (-1.0_f32).next_down(),
        1.0_f32.next_up(),
        f32::NAN,
    ] {
        let live = classify(&current, |next| set_ll(next, value)).is_ok();
        let target = Matrix2x2 {
            ll: value,
            lr: 0.0,
            rl: 0.0,
            rr: 0.5,
        };
        let setter = matrices.set_target_smoothed(0, target, 0).is_ok();
        assert_eq!(live, setter, "coefficient {value}");
    }
}

fn feed(frames: usize) -> [Vec<f32>; 2] {
    let mut draw = Draw::new(0x1255);
    let mut plane = || -> Vec<f32> { (0..frames).map(|_| draw.noise(0.5)).collect() };
    [plane(), plane()]
}

/// Renders `BLOCKS` blocks from sample 0, feeding `planes`, and returns the output bits.
fn render(host: &mut PreparedHost, planes: &[Vec<f32>; 2]) -> Vec<u32> {
    render_blocks(host, planes, 0..BLOCKS)
}

/// Renders `blocks`, feeding `planes`, and returns the output bits.
fn render_blocks(
    host: &mut PreparedHost,
    planes: &[Vec<f32>; 2],
    blocks: core::ops::Range<usize>,
) -> Vec<u32> {
    let quantum = host.report.quantum_frames as usize;
    let rate = host.report.sample_rate_hz;
    let mut bits = Vec::with_capacity(quantum * 2 * blocks.len());
    for block in blocks {
        let range = block * quantum..(block + 1) * quantum;
        host.sources
            .submit(
                SOURCE.as_bytes(),
                SourceSubmission {
                    generation: 1,
                    start_frame: range.start as u64,
                    sample_rate_hz: rate,
                    planes: &[&planes[0][range.clone()], &planes[1][range.clone()]],
                    frames: quantum as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut samples = vec![0.0_f32; quantum * 2];
        let output = PlanarBufferMut::try_new(&mut samples, 2, quantum, quantum).expect("planes");
        host.plan
            .render(
                RenderIo { output },
                RenderTime {
                    absolute_sample: range.start as u64,
                },
            )
            .expect("render");
        bits.extend(samples.iter().map(|sample| sample.to_bits()));
    }
    bits
}

/// Gate 3. Red if a record targets a value other than the one preparation bakes from `next`: a
/// wrong lane, a wrong lowering or a dropped `Both`. Its follow-on unmutes a lane whose fader
/// moved while it stayed muted, so it is also red if that move's record was dropped (#1255 D5):
/// the unmuted lane would play the stale gain.
#[test]
fn pushed_records_render_the_rebuilt_plan() {
    let mut source = parse_session_json(FIXTURE).expect("fixture parses");
    settle(&mut source);
    source.tracks[6].fader.left_mute = true;
    source.tracks[8].fader.left_mute = true;
    let current_compiled = compile(&source);
    let current = current_compiled.normalized_model().clone();
    let centre = pan_matrix(0.0, 0.0).expect("pan");
    let next = normalized(&edited(&current, |next| {
        let tracks = &mut next.tracks;
        tracks[0].fader.left_db = -6.0;
        tracks[1].fader.left_db = -3.0;
        tracks[1].fader.right_db = -3.0;
        tracks[2].fader.left_db = -2.0;
        tracks[2].fader.right_db = -4.0;
        tracks[3].fader.left_db = -0.0;
        tracks[3].matrix_or_pan = MatrixOrPan::Matrix {
            ll: centre.ll,
            lr: centre.lr,
            rl: centre.rl,
            rr: centre.rr,
            smoothing_samples: 0,
        };
        tracks[4].fader.left_mute = true;
        tracks[5].fader.left_mute = true;
        tracks[5].fader.right_mute = true;
        tracks[6].fader.left_mute = false;
        tracks[6].fader.right_mute = true;
        tracks[7].fader.left_db = -12.0;
        tracks[7].fader.left_mute = true;
        tracks[8].fader.left_db = -9.0;
        tracks[8].matrix_or_pan = MatrixOrPan::Pan {
            left: -1.0,
            right: 1.0,
            smoothing_samples: 0,
        };
    }));
    let delta = classify_live_delta(&current, &next, STEP, &EditRamps::new()).expect("live");
    assert_eq!(
        delta.strips.len(),
        8,
        "every edited track but the sign-of-zero one"
    );

    let request = HostLiveControlRequest::default();
    let (mut live, mut handles) =
        prepare_host_runtime_with_live_controls(&current_compiled, &caps(), &request)
            .unwrap_or_else(|failure| {
                panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
            });
    let mut gate = live
        .plan
        .take_revision_gate()
        .expect("the plan's gate handle");
    push_delta(&mut handles, &mut gate, &delta);
    let prepare = |compiled: &session::CompiledSession| {
        prepare_host_runtime(compiled, &caps()).unwrap_or_else(|failure| {
            panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
        })
    };
    let planes = feed(4_096);
    let expected = render(&mut prepare(&compile(&next)), &planes);
    assert_ne!(
        expected,
        render(&mut prepare(&current_compiled), &planes),
        "the edits change the output"
    );
    assert_eq!(render(&mut live, &planes), expected);

    // Follow-on: unmute the lane whose fader moved while muted, and compare the next `BLOCKS`
    // with a rebuild of that model. Downstream of the fader every stage is stateless here (a
    // zero-smoothing pan and the output sum), so the rebuild's second half is the reference.
    let unmuted = normalized(&edited(&next, |next| {
        next.tracks[8].fader.left_mute = false
    }));
    let unmute = classify_live_delta(&next, &unmuted, STEP, &EditRamps::new()).expect("live");
    assert_eq!(
        flatten(&unmute),
        vec![(
            track_id(&next, 8),
            vec![mute(BuiltinLaneSelector::Left, false, 0)],
            None
        )]
    );
    push_delta(&mut handles, &mut gate, &unmute);
    let rebuilt = render_blocks(&mut prepare(&compile(&unmuted)), &planes, 0..2 * BLOCKS);
    let second_half = rebuilt[rebuilt.len() / 2..].to_vec();
    assert!(
        second_half.iter().any(|bits| f32::from_bits(*bits) != 0.0),
        "the reference window is not silent"
    );
    assert_eq!(
        render_blocks(&mut live, &planes, BLOCKS..2 * BLOCKS),
        second_half
    );
}

/// Writes every record of `delta` to its strip's cells as one transaction, stamped from the
/// plan's own gate and published on it.
fn push_delta(
    handles: &mut host_core::HostLiveControlHandles,
    gate: &mut engine::realtime::PlanRevisionGate,
    delta: &LiveDelta<'_>,
) {
    use builtins_compiler::TestOnlyWriteGate;
    gate.write_published(|stamp| {
        for strip in &delta.strips {
            let control = handles
                .strip_controls
                .iter_mut()
                .find(|control| &*control.track_id == strip.strip_id)
                .expect("strip lane");
            for record in strip.fader_records() {
                control.fader.write(stamp, record);
            }
            if let Some(record) = strip.matrix {
                control.producer.write(stamp, record);
            }
            let input = control.input.as_mut().expect("the input lane");
            for &record in &strip.input {
                input.write(stamp, record);
            }
        }
        for route in &delta.routes {
            match route.record {
                host_core::SendRecord::Folded(record) => {
                    let index =
                        host_core::route_producer_position(&handles.route_controls, route.route_id)
                            .expect("route lane");
                    handles.route_controls[index].write(stamp, record);
                }
                host_core::SendRecord::Following(record) => {
                    let index = host_core::follow_route_producer_position(
                        &handles.follow_route_controls,
                        route.route_id,
                    )
                    .expect("following route lane");
                    handles.follow_route_controls[index].write(stamp, record);
                }
            }
        }
    });
}

// Issue #1556: the strip input family (trim, polarity, input filters).

type InputStrip = (String, Vec<TrackInputRecord>);

fn classify_input(
    current: &SessionModel,
    edit: impl FnOnce(&mut SessionModel),
) -> Result<Vec<InputStrip>, LiveRebuild> {
    classify_input_with(current, edit, STEP, &EditRamps::new())
}

fn classify_input_with(
    current: &SessionModel,
    edit: impl FnOnce(&mut SessionModel),
    ramps: LiveRamps,
    edit_ramps: &EditRamps,
) -> Result<Vec<InputStrip>, LiveRebuild> {
    let next = edited(current, edit);
    classify_live_delta(current, &next, ramps, edit_ramps).map(|delta| {
        delta
            .strips
            .iter()
            .filter(|strip| !strip.input.is_empty())
            .map(|strip| (strip.strip_id.to_owned(), strip.input.clone()))
            .collect()
    })
}

fn trim(lanes: BuiltinLaneSelector, db: f32, smoothing_samples: u32) -> TrackInputRecord {
    TrackInputRecord::TrimDb {
        lanes,
        db,
        smoothing_samples,
    }
}

fn polarity(
    lanes: BuiltinLaneSelector,
    inverted: bool,
    smoothing_samples: u32,
) -> TrackInputRecord {
    TrackInputRecord::PolarityInvert {
        lanes,
        inverted,
        smoothing_samples,
    }
}

/// The filter records [`InputFilterPreparer`] designs for `edits` from `seeds` (left HPF, left LPF,
/// right HPF, right LPF), called directly: the records a classifier must emit are exactly these.
fn designed_filters(
    rate: u32,
    seeds: [f32; 4],
    edits: &[InputFilterEdit],
) -> Vec<TrackInputRecord> {
    let mut out = [PreparedInputFilterTarget {
        lanes: BuiltinLaneSelector::Both,
        section: 0,
        pair: [0.0; 2],
        coefficients: [0.0; 6],
    }; host_core::INPUT_FILTER_TARGET_CAPACITY];
    let (_, count) = InputFilterPreparer
        .prepare(rate, &seeds, edits, &mut out)
        .expect("the edits design");
    out[..count]
        .iter()
        .map(|&target| TrackInputRecord::PreparedFilter { target })
        .collect()
}

fn filter_edit(parameter_id: u32, channel: Lane, value0: f32, value1: f32) -> InputFilterEdit {
    InputFilterEdit {
        parameter_id,
        channel,
        value0,
        value1,
    }
}

/// `two_tracks` with the first track's lanes set to `left` and `right` `(hpf_hz, lpf_hz)`.
fn with_filters(left: (f32, f32), right: (f32, f32)) -> SessionModel {
    let mut model = two_tracks();
    set_filters(&mut model, left, right);
    normalized(&model)
}

fn set_filters(model: &mut SessionModel, left: (f32, f32), right: (f32, f32)) {
    let builtins = &mut model.tracks[0].builtins;
    (builtins.left.hpf_hz, builtins.left.lpf_hz) = left;
    (builtins.right.hpf_hz, builtins.right.lpf_hz) = right;
}

/// #1556 D3. Red if a trim record lands on the wrong lane, is not merged into one `Both` when both
/// lanes move to the same dB, is emitted for a move that keeps the gain (`0.0` to `-0.0`), or if
/// the gain domain is not the setter's.
#[test]
fn trim_records_address_exactly_the_changed_lanes() {
    use BuiltinLaneSelector::{Both, Left, Right};
    let current = two_tracks();
    let first = track_id(&current, 0);
    let one = |records: Vec<TrackInputRecord>| Ok(vec![(first.clone(), records)]);
    assert_eq!(
        classify_input(&current, |next| next.tracks[0].builtins.left.trim_db = -6.0),
        one(vec![trim(Left, -6.0, 0)])
    );
    assert_eq!(
        classify_input(&current, |next| {
            next.tracks[0].builtins.left.trim_db = -3.0;
            next.tracks[0].builtins.right.trim_db = -3.0;
        }),
        one(vec![trim(Both, -3.0, 0)])
    );
    assert_eq!(
        classify_input(&current, |next| {
            next.tracks[0].builtins.left.trim_db = -2.0;
            next.tracks[0].builtins.right.trim_db = -4.0;
        }),
        one(vec![trim(Left, -2.0, 0), trim(Right, -4.0, 0)])
    );
    // Only the lane that moved gets a record, whatever the other holds.
    let split = edited(&current, |model| {
        model.tracks[0].builtins.left.trim_db = -2.0
    });
    assert_eq!(
        classify_input(&split, |next| next.tracks[0].builtins.right.trim_db = -2.0),
        one(vec![trim(Right, -2.0, 0)])
    );
    // `0.0` to `-0.0` keeps the gain: the model's bits change and the plan does not.
    assert_eq!(
        classify_input(&current, |next| {
            next.tracks[0].builtins.left.trim_db = -0.0;
            next.tracks[0].builtins.right.trim_db = -0.0;
        }),
        Ok(Vec::new())
    );
    for db in [24.0_f32, -144.0] {
        assert!(classify_input(&current, |next| next.tracks[0].builtins.left.trim_db = db).is_ok());
    }
    for db in [24.0_f32.next_up(), (-144.0_f32).next_down(), f32::NAN] {
        assert_eq!(
            classify_input(&current, |next| next.tracks[0].builtins.right.trim_db = db),
            Err(LiveRebuild::Domain),
            "{db}"
        );
    }
}

/// #1556 D3. Red if a polarity record goes to the wrong lane, or splits two lanes that flip to
/// one value.
#[test]
fn polarity_records_address_exactly_the_changed_lanes() {
    use BuiltinLaneSelector::{Both, Left, Right};
    let current = two_tracks();
    let first = track_id(&current, 0);
    let one = |records: Vec<TrackInputRecord>| Ok(vec![(first.clone(), records)]);
    assert_eq!(
        classify_input(&current, |next| next.tracks[0]
            .builtins
            .left
            .polarity_invert = true),
        one(vec![polarity(Left, true, 0)])
    );
    assert_eq!(
        classify_input(&current, |next| {
            next.tracks[0].builtins.left.polarity_invert = true;
            next.tracks[0].builtins.right.polarity_invert = true;
        }),
        one(vec![polarity(Both, true, 0)])
    );
    // Two lanes that change to different values stay two records.
    let split = edited(&current, |model| {
        model.tracks[0].builtins.left.polarity_invert = true;
    });
    assert_eq!(
        classify_input(&split, |next| {
            next.tracks[0].builtins.left.polarity_invert = false;
            next.tracks[0].builtins.right.polarity_invert = true;
        }),
        one(vec![polarity(Left, false, 0), polarity(Right, true, 0)])
    );
    // A lane that keeps its value gets no record.
    assert_eq!(
        classify_input(&split, |next| next.tracks[0]
            .builtins
            .right
            .polarity_invert = true),
        one(vec![polarity(Right, true, 0)])
    );
}

/// #1556 D3. Red if a trim or polarity record reads another row's ramp: a trim takes the fader's
/// length, a polarity flip twice the mute's, and an edit's own length (an explicit 0 included) is
/// kept as given, not doubled.
#[test]
fn input_records_carry_their_rows_ramps() {
    use BuiltinLaneSelector::Both;
    let current = two_tracks();
    let first = track_id(&current, 0);
    let flip_and_ride = |next: &mut SessionModel| {
        let builtins = &mut next.tracks[0].builtins;
        for lane in [&mut builtins.left, &mut builtins.right] {
            lane.trim_db = -6.0;
            lane.polarity_invert = true;
        }
    };
    let table = Some(ControlSmoothing {
        mute_ms: 5.0,
        fader_ms: 15.0,
        pan_ms: 25.0,
    });
    let current = with_smoothing(&current, table);
    let next = edited(&current, flip_and_ride);
    let ramps = LiveRamps::for_session(&next);
    // At 48 kHz: fader 15 ms is 720 samples, mute 5 ms is 240, a flip twice that.
    assert_eq!(
        classify_input_with(&current, flip_and_ride, ramps, &EditRamps::new()),
        Ok(vec![(
            first.clone(),
            vec![trim(Both, -6.0, 720), polarity(Both, true, 480)]
        )])
    );
    for explicit in [0, 37] {
        let mut edit_ramps = EditRamps::new();
        edit_ramps.set(
            &EditRampTarget::Strip(current.tracks[0].id.clone()),
            EditRampRow::Input,
            Some(explicit),
        );
        assert_eq!(
            classify_input_with(&current, flip_and_ride, ramps, &edit_ramps),
            Ok(vec![(
                first.clone(),
                vec![trim(Both, -6.0, explicit), polarity(Both, true, explicit)]
            )])
        );
    }
    // The other strip's entry changes nothing.
    let mut other = EditRamps::new();
    other.set(
        &EditRampTarget::Strip(current.tracks[1].id.clone()),
        EditRampRow::Input,
        Some(37),
    );
    assert_eq!(
        classify_input_with(&current, flip_and_ride, ramps, &other),
        Ok(vec![(
            first,
            vec![trim(Both, -6.0, 720), polarity(Both, true, 480)]
        )])
    );
}

/// #1556 D4. Red if a filter target is designed from the wrong seeds, a one-sided change is sent
/// as a pair or the reverse, two equal lanes are not merged into one `Both` target, or a
/// `-0.0` cutoff is a change.
#[test]
fn filter_records_are_the_preparers_targets_for_the_changed_values() {
    let rate = two_tracks().sample_rate_hz;
    let case = |before: ((f32, f32), (f32, f32)), after: ((f32, f32), (f32, f32))| {
        let current = with_filters(before.0, before.1);
        classify_input(&current, |next| set_filters(next, after.0, after.1)).map(|strips| {
            strips
                .into_iter()
                .map(|(_, records)| records)
                .collect::<Vec<_>>()
        })
    };
    let seeds = |before: ((f32, f32), (f32, f32))| [before.0.0, before.0.1, before.1.0, before.1.1];
    let steady = ((200.0, 2_000.0), (0.0, 0.0));

    // One lane, one side: the left HPF moves; the right lane and the left LPF are untouched.
    let moved = ((300.0, 2_000.0), (0.0, 0.0));
    assert_eq!(
        case(steady, moved),
        Ok(vec![designed_filters(
            rate,
            seeds(steady),
            &[filter_edit(3, Lane::Left, 300.0, 0.0)]
        )])
    );
    // The left LPF disables.
    let disabled = ((200.0, 0.0), (0.0, 0.0));
    assert_eq!(
        case(steady, disabled),
        Ok(vec![designed_filters(
            rate,
            seeds(steady),
            &[filter_edit(4, Lane::Left, 0.0, 0.0)]
        )])
    );
    // The same one-sided change on both lanes is one `Both` edit; two lanes with the same final
    // pair design equal targets, which merge into one `Both` target.
    let matching = ((200.0, 2_000.0), (0.0, 2_000.0));
    let records = case(matching, ((50.0, 2_000.0), (50.0, 2_000.0))).expect("live");
    assert_eq!(
        records,
        vec![designed_filters(
            rate,
            seeds(matching),
            &[filter_edit(3, Lane::Both, 50.0, 0.0)]
        )]
    );
    assert!(
        matches!(
            records[0][..],
            [TrackInputRecord::PreparedFilter { target }] if target.lanes == BuiltinLaneSelector::Both
        ),
        "{records:?}"
    );
    // Lanes whose final pairs differ keep one target each, though the edit is one `Both` edit.
    let records = case(steady, ((50.0, 2_000.0), (50.0, 0.0))).expect("live");
    assert_eq!(
        records,
        vec![designed_filters(
            rate,
            seeds(steady),
            &[filter_edit(3, Lane::Both, 50.0, 0.0)]
        )]
    );
    assert_eq!(records[0].len(), 2);
    // Different sides on the two lanes: one edit each; the preparer returns the HPF section first.
    let sides = ((300.0, 2_000.0), (0.0, 4_000.0));
    assert_eq!(
        case(steady, sides),
        Ok(vec![designed_filters(
            rate,
            seeds(steady),
            &[
                filter_edit(3, Lane::Left, 300.0, 0.0),
                filter_edit(4, Lane::Right, 4_000.0, 0.0)
            ]
        )])
    );
    // Both of a lane's values moving is one atomic pair edit, which a pair of one-sided edits
    // would refuse: the HPF passes the old LPF.
    let crossing = ((3_000.0, 6_000.0), (0.0, 0.0));
    assert!(
        InputFilterPreparer
            .prepare(
                rate,
                &seeds(steady),
                &[filter_edit(3, Lane::Left, 3_000.0, 0.0)],
                &mut [PreparedInputFilterTarget {
                    lanes: BuiltinLaneSelector::Both,
                    section: 0,
                    pair: [0.0; 2],
                    coefficients: [0.0; 6],
                }; 4],
            )
            .is_err(),
        "the HPF alone crosses the LPF"
    );
    assert_eq!(
        case(steady, crossing),
        Ok(vec![designed_filters(
            rate,
            seeds(steady),
            &[filter_edit(0, Lane::Left, 3_000.0, 6_000.0)]
        )])
    );
    // The same pair on both lanes is one `Both` pair edit.
    let both_pairs = ((3_000.0, 6_000.0), (3_000.0, 6_000.0));
    assert_eq!(
        case(steady, both_pairs),
        Ok(vec![designed_filters(
            rate,
            seeds(steady),
            &[filter_edit(0, Lane::Both, 3_000.0, 6_000.0)]
        )])
    );
    // A filter-off value is `+0.0` only: `validate_input_filter_pair` refuses `-0.0` as it refuses
    // a rebuild's preparation, so the rebuild reports it and no live commit acks it.
    assert_eq!(
        case(steady, ((200.0, 2_000.0), (-0.0, -0.0))),
        Err(LiveRebuild::Domain)
    );
    // Rewriting the same values is no record.
    assert_eq!(case(steady, steady), Ok(Vec::new()));
}

/// #1556 D2. Red if the records of one strip are not trims, then polarities, then filter targets,
/// each left before right, or if a track's input values reach another strip.
#[test]
fn a_strips_input_records_are_trims_then_polarities_then_targets() {
    use BuiltinLaneSelector::{Both, Left, Right};
    let rate = two_tracks().sample_rate_hz;
    let current = with_filters((0.0, 0.0), (0.0, 0.0));
    let strips = classify_input(&current, |next| {
        let builtins = &mut next.tracks[0].builtins;
        builtins.right.hpf_hz = 100.0;
        builtins.left.lpf_hz = 8_000.0;
        builtins.right.polarity_invert = true;
        builtins.left.polarity_invert = true;
        builtins.left.trim_db = -3.0;
        builtins.right.trim_db = -4.0;
        // The second track's values do not move.
        next.tracks[1].fader.left_db = -6.0;
    })
    .expect("live");
    assert_eq!(strips.len(), 1, "one strip has input records");
    let (id, records) = &strips[0];
    assert_eq!(*id, track_id(&current, 0));
    let filters = designed_filters(
        rate,
        [0.0; 4],
        &[
            filter_edit(4, Lane::Left, 8_000.0, 0.0),
            filter_edit(3, Lane::Right, 100.0, 0.0),
        ],
    );
    assert_eq!(filters.len(), 2);
    let mut expected = vec![
        trim(Left, -3.0, 0),
        trim(Right, -4.0, 0),
        polarity(Both, true, 0),
    ];
    expected.extend(filters);
    assert_eq!(*records, expected);
}

/// #1556 D5. Red if a submix strip's input section stays structural or its records are addressed
/// to a track.
#[test]
fn a_submix_strip_input_section_is_live() {
    use BuiltinLaneSelector::{Both, Left};
    let rate = with_bus(false).sample_rate_hz;
    let current = with_bus(false);
    let strips = classify_input(&current, |next| {
        let builtins = &mut next.submixes[0].builtins;
        builtins.left.trim_db = -6.0;
        builtins.left.polarity_invert = true;
        builtins.right.polarity_invert = true;
        builtins.left.hpf_hz = 80.0;
    })
    .expect("a submix input section is live");
    assert_eq!(strips.len(), 1);
    assert_eq!(strips[0].0, "bus");
    let seeds = {
        let builtins = &current.submixes[0].builtins;
        [
            builtins.left.hpf_hz,
            builtins.left.lpf_hz,
            builtins.right.hpf_hz,
            builtins.right.lpf_hz,
        ]
    };
    let mut expected = vec![trim(Left, -6.0, 0), polarity(Both, true, 0)];
    expected.extend(designed_filters(
        rate,
        seeds,
        &[filter_edit(3, Lane::Left, 80.0, 0.0)],
    ));
    assert_eq!(strips[0].1, expected);
}

// Issue #1264: effect parameter records.

/// One owned effect entry of a delta: its strip ID, its live address and its records.
type EffectEntry = (String, LiveEffectAddress, Vec<EffectControlRecord>);

fn param(
    parameter_id: u32,
    channel: ParameterChannel,
    unit: ParameterUnit,
    value: f32,
) -> EffectParam {
    EffectParam {
        parameter_id,
        channel,
        unit,
        value,
    }
}

fn record(parameter_index: u32, channel: Lane, value: f32) -> EffectControlRecord {
    EffectControlRecord::Parameter {
        parameter_index,
        channel,
        value,
    }
}

/// `two_tracks` with three inserts on the first track -- a compressor (`insert(0)`), a
/// gate/expander (`insert(1)`) and a parametric EQ (`insert(2)`) -- and one `post_insert` console
/// soft-clip slot on every track (`console(0)`), all at their defaults.
fn with_effects() -> SessionModel {
    let fixture = parse_session_json(FIXTURE).expect("fixture parses");
    let template = fixture.lower_track(&fixture.tracks[0]).pre_insert[0].clone();
    let native = |instance: &str, effect: &str| {
        let mut native = template.clone();
        native.id = id(instance);
        native.identity = EffectIdentity::Native {
            effect_id: id(effect),
        };
        native.params.clear();
        native
    };
    let mut model = two_tracks();
    model.tracks[0].inserts.effects = vec![
        native("comp", "miso.compressor"),
        native("gate", "miso.gate-expander"),
        native("eq", "miso.parametric-eq"),
    ];
    model.console.post_insert.push(ConsoleSlot {
        slot: id("clip"),
        identity: EffectIdentity::Native {
            effect_id: id("miso.soft-clip"),
        },
        quality: template.quality,
        link_mode: template.link_mode,
    });
    for track in &mut model.tracks {
        track.console.push(ConsoleEntry {
            slot: id("clip"),
            bypass: false,
            params: Vec::new(),
        });
    }
    normalized(&model)
}

fn classify_effects(
    current: &SessionModel,
    edit: impl FnOnce(&mut SessionModel),
) -> Result<(Vec<Strip>, Vec<EffectEntry>), LiveRebuild> {
    let next = edited(current, edit);
    classify_live_delta(current, &next, STEP, &EditRamps::new()).map(|delta| {
        let effects = delta
            .effects
            .iter()
            .map(|entry| {
                assert!(!entry.records.is_empty(), "an entry carries a record");
                (
                    entry.strip_id.to_owned(),
                    entry.address,
                    entry.records.clone(),
                )
            })
            .collect();
        (flatten(&delta), effects)
    })
}

fn inserts(next: &mut SessionModel, index: usize) -> &mut Vec<EffectParam> {
    &mut next.tracks[0].inserts.effects[index].params
}

/// #1264 gate 1(a). Red if a live parameter change is not one record on exactly the lane, the
/// descriptor index and the instance it changes, or if a console slot is addressed other than by
/// its slot index.
#[test]
fn a_live_parameter_change_is_one_record_on_its_lane() {
    let current = with_effects();
    let first = track_id(&current, 0);
    // Compressor threshold (ID 1, index 0), left lane only.
    assert_eq!(
        classify_effects(&current, |next| {
            inserts(next, 0).push(param(1, ParameterChannel::Left, ParameterUnit::Db, -30.0));
        }),
        Ok((
            Vec::new(),
            vec![(
                first.clone(),
                LiveEffectAddress::insert(0),
                vec![record(0, Lane::Left, -30.0)]
            )]
        ))
    );
    // Soft-clip drive (ID 1) on the second track's console slot, with a fader move beside it.
    let second = track_id(&current, 1);
    assert_eq!(
        classify_effects(&current, |next| {
            next.tracks[1].console[0].params.push(param(
                1,
                ParameterChannel::Both,
                ParameterUnit::Db,
                6.0,
            ));
            next.tracks[0].fader.left_db = -6.0;
        }),
        Ok((
            vec![(
                first,
                vec![fader_db(BuiltinLaneSelector::Left, -6.0, 0)],
                None
            )],
            vec![(
                second,
                LiveEffectAddress::console(0),
                vec![record(0, Lane::Left, 6.0), record(0, Lane::Right, 6.0)]
            )]
        ))
    );
}

/// #1264 gate 1(c). Red if a removed parameter is not returned to the descriptor's default, as a
/// rebuild would prepare it.
#[test]
fn a_removed_parameter_returns_to_its_default() {
    let mut current = with_effects();
    // Compressor ratio (ID 2, index 1; default 4).
    inserts(&mut current, 0).push(param(2, ParameterChannel::Both, ParameterUnit::Ratio, 8.0));
    let current = normalized(&current);
    assert_eq!(
        classify_effects(&current, |next| inserts(next, 0).clear()),
        Ok((
            Vec::new(),
            vec![(
                track_id(&current, 0),
                LiveEffectAddress::insert(0),
                vec![record(1, Lane::Left, 4.0), record(1, Lane::Right, 4.0)]
            )]
        ))
    );
}

/// The reason the launch descriptor of `effect_id` gives for parameter `parameter_id` being
/// prepared (its `prepared_reason`).
fn prepared_reason(effect_id: &str, parameter_id: u32) -> RebuildReason {
    let registry = effect_compiler::launch_native_effect_registry().expect("launch registry");
    registry
        .get_ascii(effect_id)
        .expect("launch effect")
        .descriptor()
        .parameters
        .iter()
        .find(|parameter| parameter.id.0 == parameter_id)
        .expect("declared parameter")
        .prepared_reason
        .expect("a prepared parameter states its reason")
        .reason
}

/// #1264 gate 1(d), #1265 gate 1(c). Red if a parameter the plan keeps prepared -- a gate/expander
/// attack, or an EQ band's `enabled` or `kind` (`automation_rate` `None`, decision 14 F2) -- is
/// classified live: the edit would be acked and the effect would never apply it.
#[test]
fn prepared_parameter_changes_need_a_rebuild() {
    let current = with_effects();
    // Gate/expander attack (ID 5).
    assert_eq!(
        classify_effects(&current, |next| {
            inserts(next, 1).push(param(
                5,
                ParameterChannel::Both,
                ParameterUnit::Milliseconds,
                10.0,
            ));
        }),
        Err(LiveRebuild::RebuildOnly(prepared_reason(
            "miso.gate-expander",
            5
        )))
    );
    // The gate's threshold (ID 1) is live.
    assert!(
        classify_effects(&current, |next| {
            inserts(next, 1).push(param(1, ParameterChannel::Both, ParameterUnit::Db, -50.0));
        })
        .is_ok()
    );
    // EQ band 1 `enabled` (ID 1) and `kind` (ID 2, a low shelf), each beside a live gain.
    for prepared in [
        param(1, ParameterChannel::Left, ParameterUnit::Linear, 1.0),
        param(2, ParameterChannel::Both, ParameterUnit::Linear, 2.0),
    ] {
        assert_eq!(
            classify_effects(&current, |next| {
                inserts(next, 2).push(param(4, ParameterChannel::Left, ParameterUnit::Db, 3.0));
                inserts(next, 2).push(prepared.clone());
            }),
            Err(LiveRebuild::RebuildOnly(prepared_reason(
                "miso.parametric-eq",
                prepared.parameter_id
            ))),
            "{prepared:?}"
        );
    }
}

/// #1264 gate 1(e). Red if the classifier admits `params` that preparation refuses (a unit
/// mismatch, a value outside the domain, an unknown parameter ID), so that a live commit would
/// leave a committed model its own rebuild refuses.
#[test]
fn params_preparation_refuses_need_a_rebuild() {
    let current = with_effects();
    let refused: [(&str, EffectParam); 3] = [
        (
            "unit mismatch",
            param(1, ParameterChannel::Both, ParameterUnit::Ratio, -30.0),
        ),
        (
            "domain",
            param(
                1,
                ParameterChannel::Both,
                ParameterUnit::Db,
                0.0_f32.next_up(),
            ),
        ),
        (
            "unknown",
            param(99, ParameterChannel::Both, ParameterUnit::Db, 0.0),
        ),
    ];
    for (name, refused) in refused {
        assert_eq!(
            classify_effects(&current, |next| inserts(next, 0).push(refused)),
            Err(LiveRebuild::Domain),
            "{name}"
        );
    }
    // The domain's own bound is live.
    assert!(
        classify_effects(&current, |next| {
            inserts(next, 0).push(param(1, ParameterChannel::Both, ParameterUnit::Db, 0.0));
        })
        .is_ok()
    );
}

// Issue #1265: parametric EQ parameters through prepared targets.

/// `with_effects` with the EQ insert's band 1 enabled on both lanes at 1 kHz on the left and
/// 3 kHz on the right, with +2 dB on the left: values other than the defaults, so a designer
/// seeded with anything but them designs other targets.
fn with_shaped_eq() -> SessionModel {
    let mut model = with_effects();
    *inserts(&mut model, 2) = vec![
        param(1, ParameterChannel::Both, ParameterUnit::Linear, 1.0),
        param(3, ParameterChannel::Left, ParameterUnit::Hz, 1_000.0),
        param(3, ParameterChannel::Right, ParameterUnit::Hz, 3_000.0),
        param(4, ParameterChannel::Left, ParameterUnit::Db, 2.0),
    ];
    normalized(&model)
}

/// The values preparation gave the EQ insert's owner: its committed rows, read from a host-core
/// plan of `model` prepared with every live lane.
fn prepared_eq_seeds(model: &SessionModel) -> Vec<f32> {
    let request = HostLiveControlRequest::default();
    let (_, handles) = prepare_host_runtime_with_live_controls(&compile(model), &caps(), &request)
        .unwrap_or_else(|failure| {
            panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
        });
    let producer = handles
        .effect_controls
        .iter()
        .find(|producer| {
            *producer.track_id == *model.tracks[0].id.as_str() && &*producer.effect_id == "eq"
        })
        .expect("EQ producer");
    producer
        .owner()
        .expect("EQ owner")
        .committed()
        .iter()
        .map(|row| row.value)
        .collect()
}

/// The targets `EqTargetPreparer` designs for `edits` from `seeds`, called directly.
fn designed_targets(
    sample_rate: u32,
    seeds: &[f32],
    edits: &[EqTargetEdit],
) -> Vec<PreparedEffectTarget> {
    let preparer = EqTargetPreparer::new(
        host_core::parametric_eq_target_preparation_factory().expect("EQ capability"),
    )
    .expect("EQ preparer");
    let mut out = [PreparedEffectTarget {
        slot: 0,
        channel: Lane::Both,
        words: [0; PREPARED_EFFECT_TARGET_WORDS],
    }; EQ_TARGET_CAPACITY];
    let (_, count) = preparer
        .prepare(sample_rate, seeds, edits, &mut out)
        .expect("designed targets");
    out[..count].to_vec()
}

/// The EQ insert's one delta entry: its records and its targets.
fn eq_entry(
    current: &SessionModel,
    edit: impl FnOnce(&mut SessionModel),
) -> Result<(Vec<EffectControlRecord>, Option<Vec<PreparedEffectTarget>>), LiveRebuild> {
    let next = edited(current, edit);
    let delta = classify_live_delta(current, &next, STEP, &EditRamps::new())?;
    assert!(delta.strips.is_empty(), "no strip record");
    assert_eq!(delta.effects.len(), 1, "one instance");
    let entry = &delta.effects[0];
    assert_eq!(
        (entry.strip_id, entry.address),
        (current.tracks[0].id.as_str(), LiveEffectAddress::insert(2))
    );
    Ok((entry.records.clone(), entry.targets.clone()))
}

/// #1265 gate 1(a). Red if the classifier seeds the target designer with values other than the
/// ones preparation gave the EQ's owner, or passes other edits than the changed rows: its targets
/// must equal `EqTargetPreparer`'s for the owner's committed rows. The fixture runs at 48 kHz, so
/// a classifier that designs at a hardcoded 48 kHz stays green here; capi's gate 2 catches that
/// at the other launch rates.
#[test]
fn an_eq_band_gain_change_carries_its_edits_and_designed_targets() {
    let current = with_shaped_eq();
    let seeds = prepared_eq_seeds(&current);
    // Band 1 gain (ID 4, index 3), left lane only.
    let (records, targets) = eq_entry(&current, |next| {
        inserts(next, 2)[3].value = 5.0;
    })
    .expect("live");
    assert_eq!(records, vec![record(3, Lane::Left, 5.0)]);
    let expected = designed_targets(
        current.sample_rate_hz,
        &seeds,
        &[EqTargetEdit {
            parameter_id: 4,
            channel: Lane::Left,
            value: 5.0,
        }],
    );
    assert_eq!(expected.len(), 1, "one section on one lane");
    assert_eq!(targets, Some(expected));
}

/// #1265 gate 1(b). Red if a live EQ cut-filter `enabled` change is refused or carries no targets:
/// the HPF's `enabled` is `Block` rate and rides a target like any other live EQ value.
#[test]
fn an_eq_hpf_enable_change_carries_targets() {
    let current = with_shaped_eq();
    let seeds = prepared_eq_seeds(&current);
    let (records, targets) = eq_entry(&current, |next| {
        inserts(next, 2).push(param(
            65,
            ParameterChannel::Both,
            ParameterUnit::Linear,
            1.0,
        ));
    })
    .expect("live");
    // HPF enabled is index 24 (after the four bands' six fields), per lane.
    assert_eq!(
        records,
        vec![record(24, Lane::Left, 1.0), record(24, Lane::Right, 1.0)]
    );
    let expected = designed_targets(
        current.sample_rate_hz,
        &seeds,
        &[EqTargetEdit {
            parameter_id: 65,
            channel: Lane::Both,
            value: 1.0,
        }],
    );
    assert!(!expected.is_empty(), "the HPF is designed");
    assert_eq!(targets, Some(expected));
}

// Issue #1266: effect bypass through the latency-preserving shunt.

/// `with_effects` with the fixture's parametric EQ as a `pre_insert` console slot (`console(0)`,
/// so the soft-clip moves to `console(1)`), bypassed on every track, and a delay (`insert(3)`)
/// and a multiband compressor (`insert(4)`) after the first track's other inserts, enabled.
fn with_bypassable() -> SessionModel {
    let fixture = parse_session_json(FIXTURE).expect("fixture parses");
    let template = fixture.lower_track(&fixture.tracks[0]).pre_insert[0].clone();
    let mut model = with_effects();
    model
        .console
        .pre_insert
        .insert(0, fixture.console.pre_insert[0].clone());
    for track in &mut model.tracks {
        track.console.insert(
            0,
            ConsoleEntry {
                slot: fixture.console.pre_insert[0].slot.clone(),
                bypass: true,
                params: Vec::new(),
            },
        );
    }
    for (instance, effect) in [("dly", "miso.delay"), ("mb", "miso.multiband-compressor")] {
        let mut native = template.clone();
        native.id = id(instance);
        native.identity = EffectIdentity::Native {
            effect_id: id(effect),
        };
        native.params.clear();
        native.bypass = false;
        model.tracks[0].inserts.effects.push(native);
    }
    normalized(&model)
}

/// #1266 gate 1. Red if a bypass flip on an effect whose bypass rides its lane is not exactly one
/// `Bypass` record carrying the post-commit value, on its instance's address, ahead of its
/// parameter records -- or if the bypass mask hides a flip, so that it is acked and never pushed.
#[test]
fn a_live_bypass_flip_is_one_record_ahead_of_its_parameters() {
    let current = with_bypassable();
    let first = track_id(&current, 0);
    let second = track_id(&current, 1);
    // A compressor insert bypassed.
    assert_eq!(
        classify_effects(&current, |next| next.tracks[0].inserts.effects[0].bypass =
            true),
        Ok((
            Vec::new(),
            vec![(
                first.clone(),
                LiveEffectAddress::insert(0),
                vec![EffectControlRecord::Bypass(
                    effect_contract::BypassSwitch::step(true)
                )]
            )]
        ))
    );
    // A console EQ entry un-bypassed on the second track: a bare record, no targets.
    let next = edited(&current, |next| next.tracks[1].console[0].bypass = false);
    let delta = classify_live_delta(&current, &next, STEP, &EditRamps::new()).expect("live");
    assert_eq!(delta.effects.len(), 1);
    let entry = &delta.effects[0];
    assert_eq!(
        (entry.strip_id, entry.address, entry.records.as_slice()),
        (
            second.as_str(),
            LiveEffectAddress::console(0),
            &[EffectControlRecord::Bypass(
                effect_contract::BypassSwitch::step(false)
            )][..]
        )
    );
    assert_eq!(entry.targets, None);
    // A bypass flip beside a parameter change: the bypass first.
    assert_eq!(
        classify_effects(&current, |next| {
            next.tracks[0].inserts.effects[0].bypass = true;
            inserts(next, 0).push(param(1, ParameterChannel::Left, ParameterUnit::Db, -30.0));
        }),
        Ok((
            Vec::new(),
            vec![(
                first,
                LiveEffectAddress::insert(0),
                vec![
                    EffectControlRecord::Bypass(effect_contract::BypassSwitch::step(true)),
                    record(0, Lane::Left, -30.0)
                ]
            )]
        ))
    );
}

/// #1266 gate 1, decision 14 F4. Red if a prepared bypass -- the multiband compressor's -- is
/// classified live in either direction: the record would be acked and never heard, because the
/// running plan holds that bypass in its prepared state. (The delay's is live since #1339.)
#[test]
fn a_prepared_bypass_change_needs_a_rebuild() {
    // The multiband compressor (#1340): the reason is the row's.
    for (index, issue) in [(4, 1340)] {
        let enabled = with_bypassable();
        assert_eq!(
            classify_effects(&enabled, |next| {
                next.tracks[0].inserts.effects[index].bypass = true;
            }),
            Err(LiveRebuild::RebuildOnly(RebuildReason::RuledLiveNotBuilt {
                issue
            })),
            "insert {index} bypassed"
        );
        let mut bypassed = enabled.clone();
        bypassed.tracks[0].inserts.effects[index].bypass = true;
        let bypassed = normalized(&bypassed);
        assert_eq!(
            classify_effects(&bypassed, |next| {
                next.tracks[0].inserts.effects[index].bypass = false;
            }),
            Err(LiveRebuild::RebuildOnly(RebuildReason::RuledLiveNotBuilt {
                issue
            })),
            "insert {index} lifted"
        );
    }
}

/// #1554 D5 on the bypass. Red if a submix strip's effects are left structural: a submix effect's
/// bypass is a record addressed to the submix's ID, like a track's.
#[test]
fn a_submix_effect_bypass_is_live() {
    let fixture = parse_session_json(FIXTURE).expect("fixture parses");
    let mut model = with_bus(false);
    let mut compressor = fixture.lower_track(&fixture.tracks[0]).pre_insert[0].clone();
    compressor.id = id("comp");
    compressor.identity = EffectIdentity::Native {
        effect_id: id("miso.compressor"),
    };
    compressor.params.clear();
    compressor.bypass = false;
    model.submixes[0].inserts.effects.push(compressor);
    let current = normalized(&model);
    assert_eq!(
        classify_effects(&current, |next| next.submixes[0].inserts.effects[0]
            .bypass = true),
        Ok((
            Vec::new(),
            vec![(
                "bus".to_owned(),
                LiveEffectAddress::insert(0),
                vec![EffectControlRecord::Bypass(
                    effect_contract::BypassSwitch::step(true)
                )]
            )]
        ))
    );
}

/// `with_effects` with the first track routed into submix `bus`, whose strip carries the console
/// soft-clip slot's entry and a compressor insert of its own.
fn with_effect_bus() -> SessionModel {
    let fixture = parse_session_json(FIXTURE).expect("fixture parses");
    let mut compressor = fixture.lower_track(&fixture.tracks[0]).pre_insert[0].clone();
    compressor.id = id("bcomp");
    compressor.identity = EffectIdentity::Native {
        effect_id: id("miso.compressor"),
    };
    compressor.params.clear();
    compressor.bypass = false;
    let mut model = with_effects();
    let first = model.tracks[0].id.clone();
    let bus = id("bus");
    let route = model
        .routes
        .iter_mut()
        .find(|route| matches!(&route.source, RouteSource::Track { track_id, .. } if *track_id == first))
        .expect("first track's route");
    let output = core::mem::replace(
        &mut route.destination,
        RouteDestination::SubmixInput {
            submix_id: bus.clone(),
        },
    );
    let mut bus_main = route.clone();
    bus_main.id = id("bus-main");
    bus_main.follows_mute = false;
    bus_main.source = RouteSource::Submix {
        submix_id: bus.clone(),
        tap: SendTap::PostPan,
    };
    bus_main.destination = output;
    model.routes.push(bus_main);
    let mut submix = Submix::unity(bus, &model.console);
    submix.inserts.effects.push(compressor);
    model.submixes.push(submix);
    normalized(&model)
}

/// #1554 D5 on effect parameters. Red if a submix strip's console entry or insert parameter is
/// left structural, or is addressed to a track: each is a record at its own strip's ID and
/// instance address.
#[test]
fn submix_effect_params_are_live() {
    let current = with_effect_bus();
    assert_eq!(current.submixes.len(), 1);
    assert_eq!(
        current.submixes[0].console.len(),
        1,
        "bus carries the clip slot"
    );
    assert_eq!(
        classify_effects(&current, |next| {
            next.submixes[0].console[0].params.push(param(
                1,
                ParameterChannel::Both,
                ParameterUnit::Db,
                6.0,
            ));
        }),
        Ok((
            Vec::new(),
            vec![(
                "bus".to_owned(),
                LiveEffectAddress::console(0),
                vec![record(0, Lane::Left, 6.0), record(0, Lane::Right, 6.0)]
            )]
        )),
        "submix console entry"
    );
    assert_eq!(
        classify_effects(&current, |next| {
            next.submixes[0].inserts.effects[0].params.push(param(
                1,
                ParameterChannel::Left,
                ParameterUnit::Db,
                -30.0,
            ));
        }),
        Ok((
            Vec::new(),
            vec![(
                "bus".to_owned(),
                LiveEffectAddress::insert(0),
                vec![record(0, Lane::Left, -30.0)]
            )]
        )),
        "submix insert"
    );
}

/// An automation on `with_effects`' EQ insert (`insert(2)` of the first track).
fn eq_automation(model: &SessionModel, parameter_id: u32, unit: ParameterUnit) -> Automation {
    Automation {
        id: id("eq-ride"),
        target: ControlAddress {
            owner: Owner::Strip(model.tracks[0].id.clone()),
            section: Section::Inserts(id("eq")),
            control: parameter_id,
            lane: ParameterChannel::Left,
        },
        segments: vec![AutomationSegment {
            shape: AutomationShape::Step,
            start_sample: 0,
            end_sample: 960,
            start_value: 1.0,
            end_value: 1.0,
            unit,
        }],
        offset: session::AutomationOffset::default(),
    }
}

/// #1335 gate 4. An edit that only adds an automation on the EQ's `band-1-enabled`
/// (`automation_rate` `None`) routes to the rebuild, whose preparation refuses it; one that adds
/// an automation on the block-rate `band-1-gain` stays live with no records (#1260 D2).
///
/// Red mutations: no step-4 check -> the prepared target is live with no records, acked and never
/// heard; route every automation change to the rebuild -> the block-rate target is not live.
#[test]
fn an_automation_on_a_prepared_effect_parameter_needs_a_rebuild() {
    let mut declared = with_effects();
    // An automation names the row, not a declared parameter (#1566 D5): the instance states none.
    declared.tracks[0].inserts.effects[2].params = Vec::new();
    let current = normalized(&declared);
    assert_eq!(
        classify_effects(&current, |next| {
            let ride = eq_automation(next, 1, ParameterUnit::Linear);
            next.automation.push(ride);
        }),
        Err(LiveRebuild::AutomationTarget)
    );
    assert_eq!(
        classify_effects(&current, |next| {
            let ride = eq_automation(next, 4, ParameterUnit::Db);
            next.automation.push(ride);
        }),
        Ok((Vec::new(), Vec::new()))
    );
}

// Issue #1554: the classifier reads the control rows.

/// `with_effects` with a true-peak limiter insert (`insert(3)`) on the first track.
fn with_limiter() -> SessionModel {
    let mut model = with_effects();
    let mut limiter = model.tracks[0].inserts.effects[0].clone();
    limiter.id = id("limiter");
    limiter.identity = EffectIdentity::Native {
        effect_id: id("miso.true-peak-limiter"),
    };
    limiter.params.clear();
    model.tracks[0].inserts.effects.push(limiter);
    normalized(&model)
}

/// Gate 2. Red if a rebuild-only value is acked as live, or reports another reason than its own
/// row's; or if a live value edit beside a rebuild-only or structural one hides it.
#[test]
fn a_rebuild_only_row_is_a_rebuild_with_its_reason() {
    let current = two_tracks();
    assert_eq!(
        classify(&current, |next| next.tracks[0]
            .builtins
            .right
            .delay_samples = 64),
        Err(LiveRebuild::RebuildOnly(builtin_reason(DELAY_SAMPLES)))
    );
    // The limiter's lookahead, beside a live threshold edit of the same instance.
    let limiter = with_limiter();
    assert_eq!(
        classify_effects(&limiter, |next| {
            let params = &mut next.tracks[0].inserts.effects[3].params;
            params.push(param(1, ParameterChannel::Both, ParameterUnit::Db, -3.0));
            params.push(param(
                3,
                ParameterChannel::Both,
                ParameterUnit::Milliseconds,
                2.0,
            ));
        }),
        Err(LiveRebuild::RebuildOnly(prepared_reason(
            "miso.true-peak-limiter",
            3
        )))
    );
    // A send's gain: into the output, and into a submix.
    assert_eq!(
        classify(&current, |next| next.routes[0].gain_db = -1.0),
        Err(LiveRebuild::RebuildOnly(RebuildReason::Optimisation))
    );
    let bus = with_bus(false);
    let send = bus
        .routes
        .iter()
        .position(|route| matches!(route.destination, RouteDestination::SubmixInput { .. }))
        .expect("the send into the bus");
    // The send into the bus is live (#1557), and what it follows is live too (#1558).
    assert_eq!(
        classify(&bus, |next| next.routes[send].gain_db = -1.0),
        Ok(Vec::new())
    );
    assert!(
        classify(&with_bus(true), |next| next.routes[send].follows_mute =
            false)
        .is_ok()
    );
    // A live fader edit does not hide a rebuild-only edit, or a structural one.
    assert_eq!(
        classify(&current, |next| {
            next.tracks[0].fader.left_db = -6.0;
            next.tracks[1].builtins.left.delay_samples = 64;
        }),
        Err(LiveRebuild::RebuildOnly(builtin_reason(DELAY_SAMPLES)))
    );
    let effects = with_effects();
    assert_eq!(
        classify_effects(&effects, |next| {
            next.tracks[0].fader.left_db = -6.0;
            next.tracks[0].inserts.effects.swap(0, 1);
        }),
        Err(LiveRebuild::Structure)
    );
    assert_eq!(
        classify(&current, |next| {
            next.tracks[0].fader.left_db = -6.0;
            next.tracks.pop();
        }),
        Err(LiveRebuild::Structure)
    );
}

/// Gate 4 and #1264 gate 1(b). Red if the mask compares resolved values instead of
/// representations, or the reverse: a `both` entry rewritten as equal `left` and `right` entries
/// is a different field with the same value, so it is live with no record; a different value is
/// one record on the lane whose value changes only, and a `-0.0` for a `+0.0` default is no
/// record.
#[test]
fn rewriting_an_effect_entry_in_another_form_is_live_with_no_record() {
    let mut model = with_effects();
    model.tracks[0].inserts.effects[0].params.push(param(
        1,
        ParameterChannel::Both,
        ParameterUnit::Db,
        -30.0,
    ));
    let both = normalized(&model);
    let split = |left: f32, right: f32| {
        move |next: &mut SessionModel| {
            next.tracks[0].inserts.effects[0].params = vec![
                param(1, ParameterChannel::Left, ParameterUnit::Db, left),
                param(1, ParameterChannel::Right, ParameterUnit::Db, right),
            ];
        }
    };
    assert_eq!(
        classify_effects(&both, split(-30.0, -30.0)),
        Ok((Vec::new(), Vec::new()))
    );
    // The reverse direction, from the split form.
    let mut model = both.clone();
    split(-30.0, -30.0)(&mut model);
    let lanes = normalized(&model);
    assert_eq!(
        classify_effects(&lanes, |next| {
            next.tracks[0].inserts.effects[0].params =
                vec![param(1, ParameterChannel::Both, ParameterUnit::Db, -30.0)];
        }),
        Ok((Vec::new(), Vec::new()))
    );
    // One lane's value differs: one record on that lane only.
    assert_eq!(
        classify_effects(&both, split(-30.0, -20.0)),
        Ok((
            Vec::new(),
            vec![(
                track_id(&both, 0),
                LiveEffectAddress::insert(0),
                vec![record(0, Lane::Right, -20.0)]
            )]
        ))
    );
    // `-0.0` for the makeup's `+0.0` default is the same prepared value: no record (#1264
    // verdict NIT 5).
    assert_eq!(
        classify_effects(&both, |next| {
            inserts(next, 0).push(param(6, ParameterChannel::Both, ParameterUnit::Db, -0.0));
        }),
        Ok((Vec::new(), Vec::new()))
    );
}

/// 64 tracks and four submixes, each fed by one track.
#[cfg(feature = "test-support")]
fn many_strips() -> SessionModel {
    let mut model = two_tracks();
    let track = model.tracks[0].clone();
    let route = model.routes[0].clone();
    for index in 2..64 {
        let mut next = track.clone();
        next.id = id(&format!("t{index:02}"));
        model.tracks.push(next);
        let mut next = route.clone();
        next.id = id(&format!("r{index:02}"));
        next.source = RouteSource::Track {
            track_id: id(&format!("t{index:02}")),
            tap: SendTap::PostPan,
        };
        model.routes.push(next);
    }
    for index in 0..4 {
        let bus = id(&format!("bus{index}"));
        let mut send = route.clone();
        send.id = id(&format!("send{index}"));
        send.source = RouteSource::Track {
            track_id: id(&format!("t{:02}", index + 2)),
            tap: SendTap::PostPan,
        };
        send.destination = RouteDestination::SubmixInput {
            submix_id: bus.clone(),
        };
        let mut out = route.clone();
        out.id = id(&format!("bus{index}-main"));
        out.source = RouteSource::Submix {
            submix_id: bus.clone(),
            tap: SendTap::PostPan,
        };
        model.routes.push(send);
        model.routes.push(out);
        let console = model.console.clone();
        model.submixes.push(Submix::unity(bus, &console));
    }
    normalized(&model)
}

/// Gate 5. Red if the classifier lowers every owner again: one `fader_db` edit on 64 tracks and
/// four submixes lowers exactly one owner, on a track and on a submix, and an edit that moves
/// nothing lowers none.
#[cfg(feature = "test-support")]
#[test]
fn one_edit_lowers_one_owner() {
    let current = many_strips();
    assert_eq!((current.tracks.len(), current.submixes.len()), (64, 4));
    let lowered = |edit: fn(&mut SessionModel)| {
        host_core::take_lowered_owner_count();
        let records = classify(&current, edit).expect("a live edit");
        (host_core::take_lowered_owner_count(), records.len())
    };
    assert_eq!(lowered(|next| next.tracks[37].fader.left_db = -6.0), (1, 1));
    assert_eq!(
        lowered(|next| next.submixes[2].fader.right_db = -6.0),
        (1, 1)
    );
    assert_eq!(lowered(|_| {}), (0, 0));
    // Two families of one strip are two lowerings; two strips are two.
    assert_eq!(
        lowered(|next| {
            next.tracks[5].fader.left_mute = true;
            next.tracks[5].matrix_or_pan = MatrixOrPan::Pan {
                left: 0.5,
                right: 0.5,
                smoothing_samples: 0,
            };
        }),
        (2, 1)
    );
}

// Issue #1557: the route family (a send's gain, mute and matrix).

/// One route record, owned: the route's ID, its target, whether it silences, its ramp length.
type RouteRecordRow = (String, [f32; 4], bool, u32);

fn classify_routes_with(
    current: &SessionModel,
    edit: impl FnOnce(&mut SessionModel),
    ramps: LiveRamps,
    edit_ramps: &EditRamps,
) -> Result<Vec<RouteRecordRow>, LiveRebuild> {
    let next = edited(current, edit);
    classify_live_delta(current, &next, ramps, edit_ramps).map(|delta| {
        assert!(
            delta.strips.is_empty() && delta.effects.is_empty(),
            "a send edit moves no strip and no effect"
        );
        delta
            .routes
            .iter()
            .map(|route| {
                (
                    route.route_id.to_owned(),
                    route.record.folded().expect("a six-word route").target(),
                    route.record.folded().expect("a six-word route").mute(),
                    route.record.folded().expect("a six-word route").length(),
                )
            })
            .collect()
    })
}

fn classify_routes(
    current: &SessionModel,
    edit: impl FnOnce(&mut SessionModel),
) -> Result<Vec<RouteRecordRow>, LiveRebuild> {
    classify_routes_with(current, edit, STEP, &EditRamps::new())
}

/// The index of the send into the bus of [`with_bus`].
fn send_index(model: &SessionModel) -> usize {
    model
        .routes
        .iter()
        .position(|route| matches!(route.destination, RouteDestination::SubmixInput { .. }))
        .expect("the send into the bus")
}

fn matrix_of(route: &session::Route) -> [f32; 4] {
    let matrix = &route.channel_matrix;
    [matrix.ll, matrix.lr, matrix.rl, matrix.rr]
}

fn folded(gain_db: f32, matrix: [f32; 4], mute: bool, source_lane_muted: [bool; 2]) -> [f32; 4] {
    graph_compiler::route_coefficients(gain_db, matrix, mute, source_lane_muted)
        .expect("in the route domain")
}

/// Gate 1. Red if a send record carries other bits than the coefficients preparation folds from the
/// post-commit values, or is emitted for a send whose held target does not change: a gain edit on a
/// muted send (the silenced `+0.0` target stays as it was) or a sign-of-zero rewrite. The unmute
/// then carries the gain the muted edit set.
#[test]
fn send_records_are_the_folded_target_of_the_post_commit_values() {
    let current = with_bus(false);
    let send = send_index(&current);
    let id = current.routes[send].id.as_str().to_owned();
    let matrix = matrix_of(&current.routes[send]);
    let gain = current.routes[send].gain_db;

    assert_eq!(classify_routes(&current, |_| {}), Ok(Vec::new()));
    assert_eq!(
        classify_routes(&current, |next| next.routes[send].gain_db = -6.0),
        Ok(vec![(
            id.clone(),
            folded(-6.0, matrix, false, [false; 2]),
            false,
            0
        )])
    );
    let swapped = [0.0, 1.0, 1.0, 0.0];
    assert_eq!(
        classify_routes(&current, |next| {
            let m = &mut next.routes[send].channel_matrix;
            (m.ll, m.lr, m.rl, m.rr) = (0.0, 1.0, 1.0, 0.0);
        }),
        Ok(vec![(
            id.clone(),
            folded(gain, swapped, false, [false; 2]),
            false,
            0
        )])
    );
    assert_eq!(
        classify_routes(&current, |next| next.routes[send].mute = true),
        Ok(vec![(id.clone(), [0.0; 4], true, 0)])
    );

    // A gain edit on a muted send gives no record, and the later unmute carries the new gain.
    let muted = edited(&current, |model| model.routes[send].mute = true);
    assert_eq!(
        classify_routes(&muted, |next| next.routes[send].gain_db = -6.0),
        Ok(Vec::new())
    );
    let edited_while_muted = edited(&muted, |model| model.routes[send].gain_db = -6.0);
    assert_eq!(
        classify_routes(&edited_while_muted, |next| next.routes[send].mute = false),
        Ok(vec![(
            id,
            folded(-6.0, matrix, false, [false; 2]),
            false,
            0
        )])
    );
}

/// Gate 1. Red if a following send's record ignores the source lane mutes (the rebuilt plan would
/// zero a column the record leaves), or zeroes them for a send that does not follow.
#[test]
fn a_following_send_record_zeroes_the_muted_source_lane() {
    let base = with_bus(true);
    let send = send_index(&base);
    let source = base
        .tracks
        .iter()
        .position(|track| {
            matches!(&base.routes[send].source, RouteSource::Track { track_id, .. } if *track_id == track.id)
        })
        .expect("the send's source track");
    let id = base.routes[send].id.as_str().to_owned();
    let matrix = matrix_of(&base.routes[send]);
    let current = edited(&base, |model| model.tracks[source].fader.left_mute = true);
    assert_eq!(
        classify_routes(&current, |next| next.routes[send].gain_db = -3.0),
        Ok(vec![(
            id.clone(),
            folded(-3.0, matrix, false, [true, false]),
            false,
            0
        )])
    );
    // Both lanes muted: the send is silenced by its source, and a gain edit changes nothing.
    let both = edited(&current, |model| {
        model.tracks[source].fader.right_mute = true
    });
    assert_eq!(
        classify_routes(&both, |next| next.routes[send].gain_db = -3.0),
        Ok(Vec::new())
    );
    // The same edit on a send that does not follow keeps both columns.
    let plain = edited(&with_bus(false), |model| {
        model.tracks[source].fader.left_mute = true
    });
    assert_eq!(
        classify_routes(&plain, |next| next.routes[send].gain_db = -3.0),
        Ok(vec![(
            id,
            folded(-3.0, matrix, false, [false; 2]),
            false,
            0
        )])
    );
}

/// Gate 3. Red if a changed field moves faster than its own row's ramp, or a mute toggle takes a
/// gain's length.
#[test]
fn send_records_carry_the_longest_changed_rows_ramp_and_a_mute_takes_its_own() {
    let current = with_bus(false);
    let send = send_index(&current);
    let target = EditRampTarget::Route(current.routes[send].id.clone());
    let session_ramps = LiveRamps {
        fader_samples: 100,
        mute_samples: 300,
        pan_samples: 200,
        link_samples: 0,
    };
    let length = |edit: &EditRamps, change: fn(&mut SessionModel, usize)| {
        let records =
            classify_routes_with(&current, |next| change(next, send), session_ramps, edit)
                .expect("a live send edit");
        assert_eq!(records.len(), 1);
        records[0].3
    };
    let gain_and_matrix = |next: &mut SessionModel, send: usize| {
        next.routes[send].gain_db = -6.0;
        next.routes[send].channel_matrix.lr = 0.5;
    };
    // Session lengths: the gain's 100 and the matrix's 200 give 200.
    assert_eq!(length(&EditRamps::new(), gain_and_matrix), 200);
    // The edits' own lengths, 240 and 480: one record of 480.
    let mut edits = EditRamps::new();
    edits.set(&target, EditRampRow::RouteGain, Some(240));
    edits.set(&target, EditRampRow::RouteMatrix, Some(480));
    assert_eq!(length(&edits, gain_and_matrix), 480);
    // A row that did not change does not lengthen the ramp.
    assert_eq!(
        length(&edits, |next, send| next.routes[send].gain_db = -6.0),
        240
    );
    // The gate changes: the mute row governs, whatever else moved.
    let mut edits = EditRamps::new();
    edits.set(&target, EditRampRow::RouteMute, Some(7));
    edits.set(&target, EditRampRow::RouteGain, Some(900));
    let mute_and_gain = |next: &mut SessionModel, send: usize| {
        next.routes[send].mute = true;
        next.routes[send].gain_db = -6.0;
    };
    assert_eq!(length(&edits, mute_and_gain), 7);
    assert_eq!(length(&EditRamps::new(), mute_and_gain), 300);
    // An explicit zero is a step.
    let mut step = EditRamps::new();
    step.set(&target, EditRampRow::RouteGain, Some(0));
    assert_eq!(
        length(&step, |next, send| next.routes[send].gain_db = -6.0),
        0
    );
}

/// Gate 4 (classifier half). Red if a send value the route domain refuses, or a ramp the cell
/// cannot hold, is acked live: the rebuild's preparation refuses it with its own diagnostic.
#[test]
fn send_values_outside_the_route_domain_need_a_rebuild() {
    let current = with_bus(false);
    let send = send_index(&current);
    assert_eq!(
        classify_routes(&current, |next| next.routes[send].gain_db = f32::NAN),
        Err(LiveRebuild::Domain)
    );
    assert_eq!(
        classify_routes(&current, |next| next.routes[send].gain_db = 1.0e6),
        Err(LiveRebuild::Domain)
    );
    assert_eq!(
        classify_routes(&current, |next| next.routes[send].channel_matrix.ll = 1.0e6),
        Err(LiveRebuild::Domain)
    );
    // The value is checked whether or not the send is muted.
    let muted = edited(&current, |model| model.routes[send].mute = true);
    assert_eq!(
        classify_routes(&muted, |next| next.routes[send].gain_db = f32::INFINITY),
        Err(LiveRebuild::Domain)
    );
    let mut too_long = EditRamps::new();
    too_long.set(
        &EditRampTarget::Route(current.routes[send].id.clone()),
        EditRampRow::RouteGain,
        Some((1 << 22) + 1),
    );
    assert_eq!(
        classify_routes_with(
            &current,
            |next| next.routes[send].gain_db = -6.0,
            STEP,
            &too_long
        ),
        Err(LiveRebuild::Domain)
    );
}

/// Gate 2. Red if a route into the output loses its fold through a live edit: its gain, mute and
/// matrix are rebuilds for the optimisation reason, at any tap.
#[test]
fn output_route_values_stay_rebuilds() {
    let mut model = two_tracks();
    model.routes[0].source = RouteSource::Track {
        track_id: model.tracks[0].id.clone(),
        tap: SendTap::PreFader,
    };
    let current = normalized(&model);
    assert!(matches!(
        current.routes[0].destination,
        RouteDestination::OutputInput { .. }
    ));
    for edit in [
        (|next: &mut SessionModel| next.routes[0].gain_db = -6.0) as fn(&mut SessionModel),
        |next| next.routes[0].mute = true,
        |next| next.routes[0].channel_matrix.lr = 0.5,
    ] {
        assert_eq!(
            classify_routes(&current, edit),
            Err(LiveRebuild::RebuildOnly(RebuildReason::Optimisation))
        );
    }
}

/// Gate 1, Gate 5. Red if a pushed send record renders other bits than a plan prepared from the
/// post-commit model: gain, matrix and mute on one send, and the later unmute of a send whose gain
/// moved while it was muted. The live plan has the route lane, so its send loses its fold.
#[test]
fn pushed_send_records_render_the_rebuilt_plan() {
    let current = with_bus(false);
    let send = send_index(&current);
    let next = normalized(&edited(&current, |next| {
        next.routes[send].gain_db = -6.0;
        let m = &mut next.routes[send].channel_matrix;
        (m.ll, m.lr, m.rl, m.rr) = (0.25, 0.75, -0.5, 1.0);
    }));
    let delta = classify_live_delta(&current, &next, STEP, &EditRamps::new()).expect("live");
    assert_eq!(delta.routes.len(), 1);

    let current_compiled = compile(&current);
    let request = HostLiveControlRequest::default();
    let (mut live, mut handles) =
        prepare_host_runtime_with_live_controls(&current_compiled, &caps(), &request)
            .unwrap_or_else(|failure| {
                panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
            });
    assert_eq!(handles.route_controls.len(), 1);
    let mut gate = live
        .plan
        .take_revision_gate()
        .expect("the plan's gate handle");
    push_delta(&mut handles, &mut gate, &delta);
    let prepare = |compiled: &session::CompiledSession| {
        prepare_host_runtime(compiled, &caps()).unwrap_or_else(|failure| {
            panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
        })
    };
    let planes = feed(4_096);
    let expected = render(&mut prepare(&compile(&next)), &planes);
    assert_ne!(
        expected,
        render(&mut prepare(&current_compiled), &planes),
        "the edit changes the output"
    );
    assert_eq!(render(&mut live, &planes), expected);

    // Mute, then a gain move while muted (no record), then the unmute.
    let muted = normalized(&edited(&next, |model| model.routes[send].mute = true));
    let mute = classify_live_delta(&next, &muted, STEP, &EditRamps::new()).expect("live");
    push_delta(&mut handles, &mut gate, &mute);
    let moved = normalized(&edited(&muted, |model| model.routes[send].gain_db = -12.0));
    let held = classify_live_delta(&muted, &moved, STEP, &EditRamps::new()).expect("live");
    assert!(held.routes.is_empty());
    let unmuted = normalized(&edited(&moved, |model| model.routes[send].mute = false));
    let unmute = classify_live_delta(&moved, &unmuted, STEP, &EditRamps::new()).expect("live");
    assert_eq!(unmute.routes.len(), 1);
    push_delta(&mut handles, &mut gate, &unmute);
    let rebuilt = render_blocks(&mut prepare(&compile(&unmuted)), &planes, 0..2 * BLOCKS);
    let second_half = rebuilt[rebuilt.len() / 2..].to_vec();
    assert!(
        second_half.iter().any(|bits| f32::from_bits(*bits) != 0.0),
        "the reference window is not silent"
    );
    assert_eq!(
        render_blocks(&mut live, &planes, BLOCKS..2 * BLOCKS),
        second_half
    );
}

/// #1558 gate 1 (host-core half). Red if a pushed VCA, membership, follow or followed-mute record
/// renders other bits than a plan prepared from the committed model: a VCA nested in another, an
/// own fader edit under a VCA, a lane mute under a following send, `follows_mute` turned off while
/// the source is muted and on again, a member added, a VCA added over the bus and one removed. Every
/// step is a step (ramps of 0) on the one live plan, and each is compared to a fresh plan.
#[test]
fn pushed_vca_and_follow_records_render_the_rebuilt_plan() {
    let mut base = with_bus(true);
    let send = send_index(&base);
    let source = base.routes[send].source.clone();
    let RouteSource::Track { track_id, .. } = source else {
        panic!("a track send");
    };
    // A second send, pre-fader, from the second track into the bus, following its mute: it keeps
    // sending when its source mutes unless it follows, so the follow is audible.
    let second = base
        .tracks
        .iter()
        .find(|track| track.id != track_id)
        .expect("a second track")
        .id
        .clone();
    let mut extra = base.routes[send].clone();
    extra.id = id("zz-pre");
    extra.source = RouteSource::Track {
        track_id: second,
        tap: SendTap::PreFader,
    };
    extra.follows_mute = true;
    base.routes.push(extra);
    let mut inner = vca();
    inner.id = id("inner");
    inner.members = vec![track_id.clone()];
    let mut outer = vca();
    outer.id = id("outer");
    outer.members = vec![inner.id.clone()];
    base.vcas = vec![inner, outer];
    let current = normalized(&base);
    assert_eq!(
        current.tracks[0].id, track_id,
        "the edits address the first track"
    );
    let steps: [(&str, Edit); 11] = [
        ("inner offset", |m| {
            let v = m.vcas.iter_mut().find(|v| v.id == id("inner")).unwrap();
            v.fader.left_db = -6.0;
            v.fader.right_db = -6.0;
        }),
        ("outer mute left", |m| {
            m.vcas
                .iter_mut()
                .find(|v| v.id == id("outer"))
                .unwrap()
                .fader
                .left_mute = true;
        }),
        ("outer unmute", |m| {
            m.vcas
                .iter_mut()
                .find(|v| v.id == id("outer"))
                .unwrap()
                .fader
                .left_mute = false;
        }),
        ("own fader", |m| m.tracks[0].fader.left_db = -3.0),
        ("own mute left", |m| m.tracks[0].fader.left_mute = true),
        ("own mute both", |m| m.tracks[0].fader.right_mute = true),
        ("second track mutes", |m| {
            m.tracks[1].fader.left_mute = true;
            m.tracks[1].fader.right_mute = true;
        }),
        ("follows off", |m| {
            let i = m.routes.iter().position(|r| r.id == id("zz-pre")).unwrap();
            m.routes[i].follows_mute = false;
        }),
        ("follows on, unmute", |m| {
            let i = m.routes.iter().position(|r| r.id == id("zz-pre")).unwrap();
            m.routes[i].follows_mute = true;
            for track in &mut m.tracks[..2] {
                track.fader.left_mute = false;
                track.fader.right_mute = false;
            }
        }),
        ("member added", |m| {
            let member = m.tracks[1].id.clone();
            m.vcas
                .iter_mut()
                .find(|v| v.id == id("inner"))
                .unwrap()
                .members
                .push(member);
        }),
        ("vca over the bus, outer removed", |m| {
            let mut over = vca();
            over.id = id("over-bus");
            over.fader.left_db = -4.0;
            over.members = vec![id("bus")];
            m.vcas.push(over);
            m.vcas.retain(|v| v.id != id("outer"));
        }),
    ];
    let current_compiled = compile(&current);
    let (mut live, mut handles) = prepare_host_runtime_with_live_controls(
        &current_compiled,
        &caps(),
        &HostLiveControlRequest::default(),
    )
    .unwrap_or_else(|failure| panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes())));
    let mut gate = live
        .plan
        .take_revision_gate()
        .expect("the plan's gate handle");
    let window = 2;
    let quantum = live.report.quantum_frames as usize;
    let planes = feed(quantum * window * (steps.len() + 1));
    let mut committed = current;
    let mut rendered = render_blocks(&mut live, &planes, 0..window);
    let fresh = |model: &SessionModel| {
        prepare_host_runtime(&compile(model), &caps()).unwrap_or_else(|failure| {
            panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
        })
    };
    assert_eq!(
        rendered,
        render_blocks(&mut fresh(&committed), &planes, 0..window)
    );
    for (index, (name, step)) in steps.iter().enumerate() {
        let next = normalized(&edited(&committed, step));
        let delta = classify_live_delta(&committed, &next, STEP, &EditRamps::new())
            .unwrap_or_else(|reason| panic!("{name}: {reason:?}"));
        push_delta(&mut handles, &mut gate, &delta);
        let blocks = (index + 1) * window..(index + 2) * window;
        rendered = render_blocks(&mut live, &planes, blocks.clone());
        let expected = render_blocks(&mut fresh(&next), &planes, 0..blocks.end);
        let expected = expected[expected.len() * blocks.start / blocks.end..].to_vec();
        assert!(rendered == expected, "{name}");
        // Not vacuous: the step is audible, so a record that was never pushed would differ.
        let before = render_blocks(&mut fresh(&committed), &planes, 0..blocks.end);
        assert!(
            expected != before[before.len() * blocks.start / blocks.end..],
            "{name}: the step changes the output"
        );
        committed = next;
    }
    // Not vacuous: the last state renders something, and differs from the first.
    assert!(rendered.iter().any(|bits| f32::from_bits(*bits) != 0.0));
}

/// Gate 1. Red if the classifier lowers other routes than the one that changed, or emits its
/// records out of canonical route-ID order, or the producer lookup is not by route ID.
#[cfg(feature = "test-support")]
#[test]
fn one_send_edit_lowers_one_route_and_records_are_in_route_id_order() {
    let current = many_strips();
    let sends: Vec<usize> = current
        .routes
        .iter()
        .enumerate()
        .filter(|(_, route)| matches!(route.destination, RouteDestination::SubmixInput { .. }))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(sends.len(), 4);
    host_core::take_lowered_owner_count();
    let records = classify_routes(&current, |next| next.routes[sends[2]].gain_db = -6.0)
        .expect("a live edit");
    assert_eq!(host_core::take_lowered_owner_count(), 1);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].0, current.routes[sends[2]].id.as_str());
    let all = classify_routes(&current, |next| {
        for &send in &sends {
            next.routes[send].gain_db = -6.0;
        }
    })
    .expect("a live edit");
    let ids: Vec<&str> = all.iter().map(|row| row.0.as_str()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    assert_eq!(ids, sorted);
    assert_eq!(ids.len(), 4);
}

/// Classify `current` against `edit` of it with the monitor overlays `committed` and `next` and the
/// solo ramp `ramp`; the strip and route records, owned.
fn classify_monitor(
    current: &SessionModel,
    edit: impl FnOnce(&mut SessionModel),
    ramps: LiveRamps,
    edit_ramps: &EditRamps,
    overlays: [&controls::MonitorOverlay; 2],
    ramp: Option<u32>,
) -> Result<Both, LiveRebuild> {
    let next = edited(current, edit);
    host_core::classify_live_delta_with_monitor(
        current,
        &next,
        ramps,
        edit_ramps,
        host_core::MonitorBase {
            committed: overlays[0],
            next: overlays[1],
            ramp,
        },
    )
    .map(|delta| {
        let routes = delta
            .routes
            .iter()
            .map(|route| {
                (
                    route.route_id.to_owned(),
                    route.record.folded().expect("a six-word route").target(),
                    route.record.folded().expect("a six-word route").mute(),
                    route.record.folded().expect("a six-word route").length(),
                )
            })
            .collect();
        (flatten(&delta), routes)
    })
}

/// #1559 D6, D7. A change of the monitor overlay alone, with the model as it is, gives the other
/// tracks' mute records and the gate of the send their strip feeds; the soloed track and the
/// solo-safe bus get none. The records take the solo's ramp (the session's `mute_ms` when it gave
/// none, a step for 0), an unsolo reverses them, and a record that a model edit of the same
/// transaction also produced takes the model edit's ramp, with the send following its source.
///
/// Red if the overlay change is not an effective-value difference (no records), a solo-safe
/// submix or the soloed track mutes, the send is left open, a solo resolves its default from
/// another key, or the solo's ramp overrides a model edit's.
#[test]
fn a_change_of_the_monitor_overlay_alone_is_the_other_strips_mutes_and_the_followers_gate() {
    let current = with_bus(true);
    let send = send_index(&current);
    let send_id = current.routes[send].id.as_str().to_owned();
    let (gain, matrix) = (
        current.routes[send].gain_db,
        matrix_of(&current.routes[send]),
    );
    let fed = track_id(&current, 0);
    let other = track_id(&current, 1);
    let ramps = LiveRamps {
        fader_samples: 100,
        mute_samples: 40,
        pan_samples: 200,
        link_samples: 100,
    };
    let none = controls::MonitorOverlay::new();
    let soloed = controls::MonitorOverlay::with_soloed([id(&other)]);
    let closed = [0.0; 4];

    // Solo the track that feeds nothing: the fed track mutes and its send closes, both over mute_ms.
    let (strips, routes) = classify_monitor(
        &current,
        |_| {},
        ramps,
        &EditRamps::new(),
        [&none, &soloed],
        None,
    )
    .expect("live");
    assert_eq!(
        strips,
        vec![(
            fed.clone(),
            vec![mute(BuiltinLaneSelector::Both, true, 40)],
            None
        )]
    );
    assert_eq!(routes, vec![(send_id.clone(), closed, true, 40)]);

    // A step, and an explicit length.
    for (ramp, expected) in [(Some(0), 0), (Some(480), 480)] {
        let (strips, routes) = classify_monitor(
            &current,
            |_| {},
            ramps,
            &EditRamps::new(),
            [&none, &soloed],
            ramp,
        )
        .expect("live");
        assert_eq!(
            strips[0].1,
            vec![mute(BuiltinLaneSelector::Both, true, expected)]
        );
        assert_eq!(routes[0].3, expected);
    }

    // The unsolo reverses it: the track unmutes and the send reopens to the folded coefficients.
    let (strips, routes) = classify_monitor(
        &current,
        |_| {},
        ramps,
        &EditRamps::new(),
        [&soloed, &none],
        None,
    )
    .expect("live");
    assert_eq!(
        strips,
        vec![(
            fed.clone(),
            vec![mute(BuiltinLaneSelector::Both, false, 40)],
            None
        )]
    );
    assert_eq!(
        routes,
        vec![(
            send_id.clone(),
            folded(gain, matrix, false, [false; 2]),
            false,
            40
        )]
    );

    // No change of the overlay, no record.
    let (strips, routes) = classify_monitor(
        &current,
        |_| {},
        ramps,
        &EditRamps::new(),
        [&soloed, &soloed],
        Some(9),
    )
    .expect("live");
    assert!(strips.is_empty() && routes.is_empty());

    // The fed track's own mute edit, with its own length, beside the solo with another: the
    // record both produced takes the edit's, and so does the send that follows it.
    let mut edit_ramps = EditRamps::new();
    edit_ramps.set(&EditRampTarget::Strip(id(&fed)), EditRampRow::Mute, Some(3));
    let (strips, routes) = classify_monitor(
        &current,
        |next| {
            next.tracks[0].fader.left_mute = true;
            next.tracks[0].fader.right_mute = true;
        },
        ramps,
        &edit_ramps,
        [&none, &soloed],
        Some(9),
    )
    .expect("live");
    assert_eq!(
        strips,
        vec![(fed, vec![mute(BuiltinLaneSelector::Both, true, 3)], None)]
    );
    assert_eq!(routes, vec![(send_id, closed, true, 3)]);
}

/// #1559 D7 with #1558 D7. A transaction that changes a strip's VCA reach and a solo with an
/// explicit ramp gives the strip's mute record the model's length: a change in VCA reach is the
/// model's cause, and the solo's ramp keeps only a mute that the overlay alone moved. Here the
/// second track joins a muted VCA while the first track is soloed (which also mutes the second);
/// the record takes the VCA mute's default length, not the solo's 9. The same solo with no
/// membership change still takes its own 9.
///
/// Red if the solo's ramp wins over a membership change: the check for a model cause looks only
/// at VCAs whose own value changed, so a strip that joins a VCA that already held its mute is
/// missed.
#[test]
fn a_vca_reach_change_beside_a_solo_takes_the_models_ramp() {
    let current = with_vca(0.0, 0.0, [true, true]);
    let (first, second) = (track_id(&current, 0), track_id(&current, 1));
    let ramps = LiveRamps {
        fader_samples: 100,
        mute_samples: 40,
        pan_samples: 200,
        link_samples: 100,
    };
    let none = controls::MonitorOverlay::new();
    let soloed = controls::MonitorOverlay::with_soloed([id(&first)]);
    let mute_of = |join: bool, ramp: Option<u32>| {
        let (strips, _) = classify_monitor(
            &current,
            |next| {
                if join {
                    let member = next.tracks[1].id.clone();
                    next.vcas[0].members.push(member);
                }
            },
            ramps,
            &EditRamps::new(),
            [&none, &soloed],
            ramp,
        )
        .expect("live");
        strips
            .into_iter()
            .find(|strip| strip.0 == second)
            .map(|strip| strip.1)
    };
    // The solo alone: the second track mutes over the solo's length.
    assert_eq!(
        mute_of(false, Some(9)),
        Some(vec![mute(BuiltinLaneSelector::Both, true, 9)])
    );
    // The same solo with the second track joining the muted VCA: the model's length.
    assert_eq!(
        mute_of(true, Some(9)),
        Some(vec![mute(BuiltinLaneSelector::Both, true, 40)])
    );
}

// Issue #1639: the jump lengths of stored automation, in the plan's cell.

/// The classification of `current` to `edit(current)` under the post-commit lengths, as a host
/// classifies a transaction.
fn classify_jump(
    current: &SessionModel,
    edit: impl FnOnce(&mut SessionModel),
) -> (Option<host_core::GraphJumpLengths>, bool, usize) {
    let next = edited(current, edit);
    let delta = classify_live_delta(
        current,
        &next,
        LiveRamps::for_session(&next),
        &EditRamps::new(),
    )
    .expect("live");
    assert!(
        delta.strips.is_empty() && delta.effects.is_empty() && delta.routes.is_empty(),
        "no record beside the jump lengths"
    );
    (
        delta.jump_lengths,
        delta.is_empty(),
        usize::from(delta.settings_changed),
    )
}

/// `model` with one stored automation, `ride`.
fn automated(model: &SessionModel, ride: impl FnOnce(&SessionModel) -> Automation) -> SessionModel {
    let mut model = model.clone();
    let ride = ride(&model);
    model.automation.push(ride);
    model
}

/// A mute ride on the first track's builtin strip (builtin 6, left lane).
fn mute_ride(model: &SessionModel) -> Automation {
    let mut ride = fader_ride(model);
    ride.id = id("mute-ride");
    ride.target.control = 6;
    ride.target.lane = ParameterChannel::Left;
    ride.segments[0].shape = AutomationShape::Step;
    ride.segments[0].unit = ParameterUnit::Linear;
    ride.segments[0].start_value = 0.0;
    ride.segments[0].end_value = 1.0;
    ride
}

/// #1639 gate 2. A `control_smoothing` change on a session with a fader or a mute entry is a live
/// delta that carries the post-commit lengths and no record; without stored automation, or with
/// only an effect parameter entry, it is empty (`model_only`). A change in milliseconds that rounds
/// to the same sample counts is empty too. An effect's bypass entry reads the `mute_ms` length
/// (its crossfade), so it carries the lengths.
///
/// Test value: red if the edit is `live` with no entry that reads a length (a needless write),
/// `model_only` with one (the new lengths never reach render), or if equal sample counts still
/// write.
#[test]
fn a_control_smoothing_change_writes_the_jump_lengths_only_for_automation_that_reads_them() {
    let fast = ControlSmoothing {
        mute_ms: 1.0,
        fader_ms: 2.0,
        pan_ms: 3.0,
    };
    let base = two_tracks();
    let expected = host_core::GraphJumpLengths {
        fader_samples: 96,
        mute_samples: 48,
        pan_samples: 144,
    };
    for (name, current) in [
        ("fader entry", automated(&base, fader_ride)),
        ("mute entry", automated(&base, mute_ride)),
        (
            "pan entry",
            automated(&base, |model| pan_ride(model, false)),
        ),
    ] {
        assert!(
            controls::automation_diagnostics(&current, current.sample_rate_hz).is_empty(),
            "{name}: a valid entry"
        );
        assert_eq!(
            classify_jump(&current, |next| next.control_smoothing = Some(fast)),
            (Some(expected), false, 1),
            "{name}: live with the new lengths"
        );
    }

    // No stored automation: model-only.
    assert_eq!(
        classify_jump(&base, |next| next.control_smoothing = Some(fast)),
        (None, true, 1),
        "no automation"
    );

    // Only an effect parameter entry: its ramp is the effect's own, so model-only.
    let mut effects = with_effects();
    effects.tracks[0].inserts.effects[2].params = Vec::new();
    let effects = normalized(&effects);
    let gain = automated(&effects, |model| eq_automation(model, 4, ParameterUnit::Db));
    assert!(controls::automation_diagnostics(&gain, gain.sample_rate_hz).is_empty());
    assert_eq!(
        classify_jump(&gain, |next| next.control_smoothing = Some(fast)),
        (None, true, 1),
        "an effect parameter entry"
    );

    // An effect's bypass entry: its crossfade is the `mute_ms` length.
    let bypass = automated(&effects, |model| {
        let mut ride = eq_automation(model, controls::BYPASS_CONTROL, ParameterUnit::Linear);
        ride.target.section = Section::Inserts(id("comp"));
        ride.target.lane = ParameterChannel::Both;
        ride.segments[0].start_value = 0.0;
        ride
    });
    assert!(controls::automation_diagnostics(&bypass, bypass.sample_rate_hz).is_empty());
    assert_eq!(
        classify_jump(&bypass, |next| next.control_smoothing = Some(fast)),
        (Some(expected), false, 1),
        "an effect bypass entry"
    );

    // 10.0 ms and 10.001 ms are both 480 samples at 48 kHz: no write.
    let ten = ControlSmoothing {
        mute_ms: 10.0,
        fader_ms: 10.0,
        pan_ms: 10.0,
    };
    let current = with_smoothing(&automated(&base, fader_ride), Some(ten));
    assert_eq!(current.sample_rate_hz, 48_000);
    assert_eq!(
        classify_jump(&current, |next| {
            next.control_smoothing = Some(ControlSmoothing {
                fader_ms: 10.001,
                ..ten
            });
        }),
        (None, true, 1),
        "equal sample counts"
    );
}

/// #1639 D3: the rows whose jumps read a length are the rows `LiveRamps::for_row` gives a length:
/// a builtin fader, trim, mute, polarity, pan or matrix row, a strip's own and a VCA's ramped rows,
/// and an effect's bypass; never a filter, the delay or an effect parameter.
///
/// Test value: red if the row set is a hand-written list that drifts from the descriptors' ramp
/// rows, or if an effect's parameter is taken for its bypass.
#[test]
fn the_rows_that_read_a_jump_length_are_the_ramped_rows() {
    let strip = |section: Section, control: u32| ControlAddress {
        owner: Owner::Strip(id("track")),
        section,
        control,
        lane: ParameterChannel::Left,
    };
    let builtin = |control| host_core::automation_jump_row(&strip(Section::Builtins, control));
    assert_eq!(builtin(1), Some(LiveRampRow::PolarityInvert));
    assert_eq!(builtin(2), Some(LiveRampRow::InputTrim));
    assert_eq!(builtin(3), None, "the HPF keeps its fixed ramp");
    assert_eq!(builtin(4), None, "the LPF keeps its fixed ramp");
    assert_eq!(builtin(5), Some(LiveRampRow::Fader));
    assert_eq!(builtin(6), Some(LiveRampRow::Mute));
    for matrix in 7..=10 {
        assert_eq!(builtin(matrix), Some(LiveRampRow::Matrix));
    }
    assert_eq!(builtin(11), None, "the delay is not live");
    assert_eq!(builtin(12), Some(LiveRampRow::Pan));
    let effect =
        |control| host_core::automation_jump_row(&strip(Section::Inserts(id("eq")), control));
    assert_eq!(
        effect(controls::BYPASS_CONTROL),
        Some(LiveRampRow::BypassCrossfade)
    );
    assert_eq!(effect(4), None, "an effect parameter");
    // A VCA's own rows: its offset reads `fader_ms`, its mute `mute_ms`.
    let vca: Vec<LiveRampRow> = (0..64)
        .filter_map(|control| {
            host_core::automation_jump_row(&ControlAddress {
                owner: Owner::Vca(id("vca")),
                section: Section::Own,
                control,
                lane: ParameterChannel::Both,
            })
        })
        .collect();
    assert_eq!(vca, [LiveRampRow::VcaOffset, LiveRampRow::VcaMute]);
}

/// A mute curve on the first track (builtin row 6) on `lane`: unmuted, then muted from 1,000 to
/// 2,500, then unmuted (#1651).
fn mute_curve(model: &SessionModel, lane: ParameterChannel) -> Automation {
    let step = |start_sample, end_sample, value| AutomationSegment {
        shape: AutomationShape::Step,
        start_sample,
        end_sample,
        start_value: value,
        end_value: value,
        unit: ParameterUnit::Linear,
    };
    Automation {
        id: id("mute-curve"),
        target: ControlAddress {
            owner: Owner::Strip(model.tracks[0].id.clone()),
            section: Section::Builtins,
            control: 6,
            lane,
        },
        segments: vec![
            step(0, 1_000, 0.0),
            step(1_000, 2_500, 1.0),
            step(2_500, 4_000, 0.0),
        ],
        offset: session::AutomationOffset::default(),
    }
}

/// #1651 gate 4 (as amended for OQ1 = C and the batch 18 spec check). The first track's mute is
/// driven by a curve on `lane`; the track is a member of `vca`.
///
/// - A mute entry change rebuilds (`Err(LiveRebuild::Automation)`).
/// - A hand mute on an automated lane is a live `terms` write: the lane's `Mute` record, which
///   render ORs with the curve. A hand mute on the other, non-automated lane is its `Mute` record.
/// - A VCA mute of the track gives one `Mute` record (the terms write).
/// - The mute entry reaches no fader-only reader: a VCA fader move and a static `fader_db` change
///   on the track give `FaderDb` records and no offsets write (no hand offset, no `S`).
///
/// Test value: red if the mute row stays masked (a curve edit acked and never heard), if a hand
/// mute or a VCA mute on an automated lane is dropped or turned `model_only` (it would no longer
/// close an open curve), or if a fader-only reader sees the mute entry (the fader would take an
/// offsets write instead of its record).
#[test]
fn a_mute_curve_takes_its_terms_as_mute_records() {
    use BuiltinLaneSelector::{Both, Left, Right};
    let session_ramps = LiveRamps {
        fader_samples: 100,
        mute_samples: 300,
        pan_samples: 200,
        link_samples: 0,
    };
    for (lane, label) in [
        (ParameterChannel::Both, "a curve on both lanes"),
        (ParameterChannel::Left, "a curve on the left lane"),
    ] {
        let current = normalized(&edited(&with_vca(0.0, 0.0, [false; 2]), |model| {
            let curve = mute_curve(model, lane);
            model.automation.push(curve);
        }));
        let first = track_id(&current, 0);
        let run = |edit: &dyn Fn(&mut SessionModel)| {
            let next = edited(&current, edit);
            classify_live_delta(&current, &next, session_ramps, &EditRamps::new())
                .map(|delta| level(&delta))
        };
        assert_eq!(
            run(&|next| next.automation[0].segments[1].start_sample = 1_100),
            Err(LiveRebuild::Automation),
            "{label}: a mute entry change rebuilds"
        );
        assert_eq!(
            run(&|next| next.tracks[0].fader.left_mute = true),
            Ok(vec![(
                first.clone(),
                vec![mute(Left, true, 300)],
                [None; 2]
            )]),
            "{label}: a hand mute on the automated left lane is its Mute record (the terms write)"
        );
        assert_eq!(
            run(&|next| next.tracks[0].fader.right_mute = true),
            Ok(vec![(
                first.clone(),
                vec![mute(Right, true, 300)],
                [None; 2]
            )]),
            "{label}: a hand mute on the right lane is its Mute record"
        );
        assert_eq!(
            run(&|next| next.vcas[0].fader.left_mute = true),
            Ok(vec![(
                first.clone(),
                vec![mute(Left, true, 300)],
                [None; 2]
            )]),
            "{label}: a VCA mute of the track is one Mute record"
        );
        assert_eq!(
            run(&|next| {
                next.vcas[0].fader.left_mute = true;
                next.vcas[0].fader.right_mute = true;
            }),
            Ok(vec![(
                first.clone(),
                vec![mute(Both, true, 300)],
                [None; 2]
            )]),
            "{label}: a VCA mute of both lanes is one Mute record"
        );
        // The mute entry reaches no fader-only reader (#1651 T2).
        assert_eq!(
            run(&|next| {
                next.vcas[0].fader.left_db = -6.0;
                next.vcas[0].fader.right_db = -6.0;
            }),
            Ok(vec![(
                first.clone(),
                vec![fader_db(Both, -6.0, 100)],
                [None; 2]
            )]),
            "{label}: a VCA fader move is the member's FaderDb record, not an offsets write"
        );
        assert_eq!(
            run(&|next| next.tracks[0].fader.left_db = -6.0),
            Ok(vec![(
                first.clone(),
                vec![fader_db(Left, -6.0, 100)],
                [None; 2]
            )]),
            "{label}: a static fader change is its FaderDb record, not an offsets write"
        );
    }
}

// ---- #1653 gate 3: a following route's edits ---------------------------------------------------

/// [`with_bus`]`(follows_mute)` with a mute entry on the first track's **left** lane (steps 0, 1,
/// 0 at 1,000 and 2,500): its send into `bus` is a following route (#1652 D1), automated on the
/// left lane only.
fn with_automated_follow(follows_mute: bool) -> SessionModel {
    let mut model = with_bus(follows_mute);
    let step = |start: u64, end: u64, from: f32, to: f32| AutomationSegment {
        shape: AutomationShape::Step,
        start_sample: start,
        end_sample: end,
        start_value: from,
        end_value: to,
        unit: ParameterUnit::Linear,
    };
    model.automation.push(Automation {
        id: id("left-mute"),
        target: ControlAddress {
            owner: Owner::Strip(model.tracks[0].id.clone()),
            section: Section::Builtins,
            control: 6,
            lane: ParameterChannel::Left,
        },
        segments: vec![step(0, 1_000, 0.0, 1.0), step(1_000, 2_500, 1.0, 0.0)],
        offset: session::AutomationOffset::default(),
    });
    normalized(&model)
}

/// The delta of `current -> next` (the dense classifier), after checking that the fast path over
/// the rows `rows` gives the same delta (#1613, R17-1).
fn following_delta<'a>(
    current: &SessionModel,
    next: &'a SessionModel,
    rows: &[ControlAddress],
) -> LiveDelta<'a> {
    let ramps = LiveRamps::for_session(next);
    let dense = host_core::classify_live_delta_with_monitor(
        current,
        next,
        ramps,
        &EditRamps::new(),
        host_core::MonitorBase::NONE,
    )
    .expect("a live delta");
    if !rows.is_empty() {
        let mut staged = host_core::classify_live_staged(
            current,
            controls::ModelView::whole(next),
            ramps,
            &EditRamps::new(),
            host_core::MonitorBase::NONE,
            rows,
        )
        .expect("the fast path answers");
        staged.derived_owners = None;
        assert_eq!(staged, dense, "the fast path's delta");
    }
    dense
}

/// The seven-word records of `delta`, after checking it holds no folded one.
fn seven_word(delta: &LiveDelta<'_>) -> Vec<(String, host_core::FollowRouteControlRecord)> {
    delta
        .routes
        .iter()
        .map(|route| {
            let record = route
                .record
                .following()
                .unwrap_or_else(|| panic!("{}: a folded record", route.route_id));
            (route.route_id.to_owned(), *record)
        })
        .collect()
}

/// The `[left, right]` lanes and values of the `Mute` records of the delta's only strip.
fn mute_records(delta: &LiveDelta<'_>) -> Vec<(BuiltinLaneSelector, bool)> {
    delta
        .strips
        .iter()
        .flat_map(|strip| strip.fader_records())
        .filter_map(|record| match record {
            TrackFaderRecord::Mute { lanes, muted, .. } => Some((lanes, muted)),
            TrackFaderRecord::FaderDb { .. } => None,
        })
        .collect()
}

/// #1653 gate 3. On a following route (`track 0 -> bus`, left lane automated): a `follows_mute`
/// toggle gives one seven-word record and no folded record; a hand mute on the automated lane
/// gives #1651's terms write (its `Mute` record) and one seven-word record with the new `terms`; a
/// hand mute on the other lane gives that lane's `Mute` record and one seven-word record; a route
/// gain edit gives one seven-word record with the new open transform; a terms change while
/// `follows_mute` is off gives one seven-word record whose gate is open whatever the curve (no
/// audible change); an edit that leaves every word unchanged (a fader move on the source, an
/// identical rewrite) gives no route record. Each case gives the same delta through the fast path.
///
/// Test value: red if an edit still emits a folded record to a route whose cell has seven words
/// (the curve's next event would contradict it), if a terms change with `follows_mute` off is not
/// lowered (the cell keeps stale words), or if an unchanged cell is written.
#[test]
fn a_following_routes_edits_write_its_seven_word_cell_only() {
    let current = with_automated_follow(true);
    let track = current.tracks[0].id.clone();
    let send = current
        .routes
        .iter()
        .position(|route| route.follows_mute)
        .expect("the following send");
    let send_id = current.routes[send].id.clone();
    let route_row = |control: u32| ControlAddress {
        owner: Owner::Route(send_id.clone()),
        section: Section::Own,
        control,
        lane: ParameterChannel::Both,
    };
    let strip_row = |control: u32, lane: ParameterChannel| ControlAddress {
        owner: Owner::Strip(track.clone()),
        section: Section::Builtins,
        control,
        lane,
    };
    let automated = [true, false];

    // A `follows_mute` toggle.
    let next = edited(&current, |next| next.routes[send].follows_mute = false);
    let delta = following_delta(&current, &next, &[route_row(7)]);
    assert!(delta.strips.is_empty());
    let records = seven_word(&delta);
    assert_eq!(records.len(), 1, "one seven-word record");
    let flags = records[0].1.flags();
    assert_eq!(
        (flags.follows_mute, flags.terms, flags.automated),
        (false, [false; 2], automated)
    );

    // A hand mute on the automated lane, then on the other lane.
    for (lane, selector, terms) in [
        (
            ParameterChannel::Left,
            BuiltinLaneSelector::Left,
            [true, false],
        ),
        (
            ParameterChannel::Right,
            BuiltinLaneSelector::Right,
            [false, true],
        ),
    ] {
        let next = edited(&current, |next| match lane {
            ParameterChannel::Left => next.tracks[0].fader.left_mute = true,
            _ => next.tracks[0].fader.right_mute = true,
        });
        let delta = following_delta(&current, &next, &[strip_row(6, lane)]);
        assert_eq!(mute_records(&delta), vec![(selector, true)], "{lane:?}");
        let records = seven_word(&delta);
        assert_eq!(records.len(), 1, "{lane:?}: one seven-word record");
        assert_eq!(records[0].1.flags().terms, terms, "{lane:?}");
        assert!(records[0].1.flags().follows_mute, "{lane:?}");
    }

    // A route gain edit: the new open transform.
    let next = edited(&current, |next| next.routes[send].gain_db = -6.0);
    let delta = following_delta(&current, &next, &[route_row(1)]);
    assert!(delta.strips.is_empty());
    let records = seven_word(&delta);
    assert_eq!(records.len(), 1, "one seven-word record");
    let matrix = &next.routes[send].channel_matrix;
    let expected = graph_compiler::route_values(-6.0, [matrix.ll, matrix.lr, matrix.rl, matrix.rr])
        .expect("in domain");
    let words = |transform: graph::RouteTransform| {
        [
            transform.gain,
            transform.ll,
            transform.lr,
            transform.rl,
            transform.rr,
        ]
        .map(f32::to_bits)
    };
    assert_eq!(words(records[0].1.transform()), words(expected));

    // A terms change while `follows_mute` is off: a record, and the gate stays open.
    let unfollowing = edited(&current, |next| next.routes[send].follows_mute = false);
    let next = edited(&unfollowing, |next| next.tracks[0].fader.right_mute = true);
    let delta = following_delta(
        &unfollowing,
        &next,
        &[strip_row(6, ParameterChannel::Right)],
    );
    assert_eq!(
        mute_records(&delta),
        vec![(BuiltinLaneSelector::Right, true)]
    );
    let records = seven_word(&delta);
    assert_eq!(records.len(), 1, "one seven-word record");
    let flags = records[0].1.flags();
    assert_eq!((flags.follows_mute, flags.terms), (false, [false, true]));
    for curve in [[false; 2], [true, false], [false, true], [true; 2]] {
        assert_eq!(flags.gate(curve), graph::RouteGate::OPEN, "{curve:?}");
    }

    // Edits that leave every word of the cell unchanged.
    let next = edited(&current, |next| next.tracks[0].fader.left_db = -3.0);
    let delta = following_delta(&current, &next, &[strip_row(5, ParameterChannel::Left)]);
    assert_eq!(delta.strips.len(), 1, "the fader record");
    assert!(delta.routes.is_empty(), "a fader move writes no route cell");
    let next = edited(&current, |_| {});
    let delta = following_delta(&current, &next, &[]);
    assert!(delta.is_empty(), "an identical rewrite writes nothing");
}

/// #1653 D5 (the commit's pre-write check, [`host_core::follow_route_producer_for`]): a send whose
/// source strip automates its left mute binds the seven-word lane, its producer holds the plan's
/// automated lanes (`[true, false]`), and the check takes the classifier's record for that send
/// and refuses one whose automated lanes differ in either bit, leaving the cell unwritten.
///
/// Test value: red if the pre-write check compares no lane bit or only one (a record whose lanes
/// disagree with the plan would be written and acked while render ignores them).
#[cfg(feature = "test-support")]
#[test]
fn a_following_send_binds_its_lanes_and_the_commit_refuses_others() {
    let current = with_automated_follow(true);
    let (_, handles) = host_core::prepare_host_runtime_with_live_lanes(
        &compile(&current),
        &caps(),
        &HostLiveControlRequest::default(),
        host_core::HostLiveLanes::ALL,
        &host_core::LiveValuesSuperseded::default(),
    )
    .unwrap_or_else(|failure| panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes())));
    let producers = &handles.follow_route_controls;
    assert_eq!(producers.len(), 1, "the send's seven-word lane");
    let send = current
        .routes
        .iter()
        .position(|route| route.follows_mute)
        .expect("the following send");
    let send_id = current.routes[send].id.as_str();
    assert_eq!(&*producers[0].route_id, send_id);
    assert_eq!(producers[0].automated(), [true, false]);

    let next = edited(&current, |next| next.routes[send].gain_db = -6.0);
    let delta = following_delta(&current, &next, &[]);
    let record = seven_word(&delta)[0].1;
    assert_eq!(
        host_core::follow_route_producer_for(producers, send_id, &record),
        Some(0)
    );
    for bit in [4, 5] {
        let word = record.flags().to_word() ^ (1 << bit);
        let flags = graph::RouteFollowFlags::from_word(word).expect("a defined bit");
        let other =
            host_core::FollowRouteControlRecord::new(record.transform(), flags, record.length())
                .expect("in bounds");
        assert_eq!(
            host_core::follow_route_producer_for(producers, send_id, &other),
            None,
            "bit {bit} flipped"
        );
    }
    assert_eq!(producers[0].test_only_write_count(), 0, "nothing written");
}
