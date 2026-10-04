//! Issue #1255: `classify_live_delta` decides whether a committed session delta is a live track
//! fader, mute and pan update or a rebuild, and for a live one returns exactly the records to push.
//!
//! Gate 1 builds its models from a two-track session with an empty console; gate 2 compares the
//! classifier's domain with the render-side setters'; gate 3 pushes the classifier's records into a
//! live plan of the nine-track EQ fixture and compares the output with a rebuilt plan's.
#![cfg(feature = "control-provider")]

use core::num::NonZeroUsize;

use builtins::{BuiltinFaderBank, BuiltinMatrixBank, BuiltinParameters, Matrix2x2, pan_matrix};
use dsp_reference::randomized::Draw;
use effect_contract::{
    BankWidth, EffectControlRecord, PREPARED_EFFECT_TARGET_WORDS, ParameterChannel as Lane,
    PreparedEffectTarget,
};
use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use host_core::{
    BuiltinLaneSelector, EQ_TARGET_CAPACITY, EqTargetEdit, EqTargetPreparer,
    HostLiveControlRequest, HostPrepareCaps, HostShapePolicy, LiveDelta, LiveEffectAddress,
    LiveRamps, LiveRebuild, LiveStripRecords, PreparedHost, SourceSubmission, TrackControlRecord,
    TrackFaderRecord, classify_live_delta, compile_host_session, prepare_host_runtime,
    prepare_host_runtime_with_live_controls,
};
use session::{
    Automation, AutomationSegment, AutomationShape, AutomationTarget, Console, ConsoleEntry,
    ConsoleSlot, DualMonoFader, EffectIdentity, EffectParam, MatrixOrPan, ParameterChannel,
    ParameterUnit, RackName, RouteDestination, RouteSource, SendTap, SessionModel, StableId,
    Submix, Vca, canonical_session_json, parse_session_json,
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
};

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 4_096,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_submixes: 100,
        maximum_vcas: 100,
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
    classify_live_delta(current, &next, STEP).map(|delta| flatten(&delta))
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

fn track_id(model: &SessionModel, index: usize) -> String {
    model.tracks[index].id.as_str().to_owned()
}

/// Gate 1(a). Red if the classifier emits a redundant record for a transaction that rewrites
/// identical values.
#[test]
fn a_revision_only_delta_is_live_with_no_records() {
    let current = two_tracks();
    assert_eq!(classify(&current, |_| {}), Ok(Vec::new()));
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

/// Gate 1(g). Red if the mask comparison uses `PartialEq` and misses a sign-of-zero edit of a
/// field that is not live.
#[test]
fn a_sign_of_zero_trim_edit_is_structural() {
    let current = two_tracks();
    assert_eq!(current.tracks[0].builtins.left.trim_db.to_bits(), 0);
    assert_eq!(
        classify(&current, |next| next.tracks[0].builtins.left.trim_db = -0.0),
        Err(LiveRebuild::Structure)
    );
}

/// Gate 1(h), G1. Red if a submix fader change is classified live.
#[test]
fn a_submix_fader_change_is_structural() {
    let current = with_bus(false);
    assert_eq!(
        classify(&current, |next| next.submixes[0].fader.left_db = -6.0),
        Err(LiveRebuild::Structure)
    );
}

/// Gate 1(i), G2. Red if a mute change of a `follows_mute` source is classified live.
#[test]
fn a_followed_mute_change_needs_a_rebuild() {
    let current = with_bus(true);
    let source = current
        .tracks
        .iter()
        .position(|track| {
            current.routes.iter().any(|route| {
                route.follows_mute
                    && matches!(&route.source, RouteSource::Track { track_id, .. } if *track_id == track.id)
            })
        })
        .expect("followed track");
    assert_eq!(
        classify(&current, |next| next.tracks[source].fader.right_mute = true),
        Err(LiveRebuild::FollowedMute)
    );
    assert_eq!(
        classify(&current, |next| next.tracks[source].fader.left_db = -6.0),
        Ok(vec![(
            track_id(&current, source),
            vec![fader_db(BuiltinLaneSelector::Left, -6.0, 0)],
            None
        )])
    );
    // The same mute change is live when the route does not follow.
    assert!(
        classify(&with_bus(false), |next| next.tracks[source]
            .fader
            .right_mute = true)
        .is_ok()
    );
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

/// Gate 1(j), G3. Red if a VCA in either model no longer forces a rebuild.
#[test]
fn any_vca_needs_a_rebuild() {
    let plain = two_tracks();
    let fader_move = |next: &mut SessionModel| next.tracks[0].fader.left_db = -6.0;
    let mut with_vca = plain.clone();
    with_vca.vcas.push(vca());
    assert_eq!(
        classify(&with_vca, |next| {
            next.vcas.clear();
            fader_move(next);
        }),
        Err(LiveRebuild::Vca)
    );
    assert_eq!(
        classify(&plain, |next| {
            next.vcas.push(vca());
            fader_move(next);
        }),
        Err(LiveRebuild::Vca)
    );
}

/// Gate 1(k). Red if a structural edit is classified live.
#[test]
fn structural_edits_need_a_rebuild() {
    let current = two_tracks();
    let edits: [(&str, Edit); 4] = [
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
        ("route gain", |next| next.routes[0].gain_db = -1.0),
    ];
    for (name, edit) in edits {
        assert_eq!(
            classify(&current, edit),
            Err(LiveRebuild::Structure),
            "{name}"
        );
    }
}

/// A fader ride on the first track's builtin strip: stored automation, which nothing renders.
fn fader_ride(model: &SessionModel) -> Automation {
    Automation {
        id: id("fader-ride"),
        target: AutomationTarget {
            entity_id: model.tracks[0].id.clone(),
            rack: RackName::Builtins,
            effect_id: id(session::BUILTIN_AUTOMATION_EFFECT_ID),
            parameter_id: 5,
            channel: ParameterChannel::Both,
        },
        segments: vec![AutomationSegment {
            shape: AutomationShape::Linear,
            start_sample: 0,
            end_sample: 960,
            start_value: 0.0,
            end_value: -3.0,
            unit: ParameterUnit::Db,
        }],
    }
}

/// The four model-only edits of #1260 D1.
fn model_only_edits() -> [(&'static str, Edit); 4] {
    [
        ("session id", |next| next.session_id = id("another-session")),
        ("render profile id", |next| {
            next.render_profile.id = id("another-render-profile");
        }),
        ("output profile id", |next| {
            next.output_profile.id = id("another-output-profile");
        }),
        ("automation upsert", |next| {
            let ride = fader_ride(next);
            next.automation.push(ride);
        }),
    ]
}

/// #1260 gate 1. Red if a model-only field is still compared, or if one masks a live record.
#[test]
fn model_only_edits_are_live_with_no_records() {
    use BuiltinLaneSelector::Left;
    let current = two_tracks();
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
    // All four together, with the fader move.
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
    // Removing stored automation is model-only too.
    let mut automated = current.clone();
    automated.automation.push(fader_ride(&current));
    assert_eq!(
        classify(&automated, |next| next.automation.clear()),
        Ok(Vec::new())
    );
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

/// Gate 1(l). Red if the ramp seam is ignored: fader and mute records carry `LiveRamps`, the
/// matrix record the model's smoothing.
#[test]
fn records_carry_the_ramps_and_the_model_smoothing() {
    use BuiltinLaneSelector::Both;
    let current = two_tracks();
    let next = edited(&current, |next| {
        let track = &mut next.tracks[0];
        track.fader.left_db = -6.0;
        track.fader.right_db = -6.0;
        track.fader.left_mute = true;
        track.fader.right_mute = true;
        track.matrix_or_pan = MatrixOrPan::Pan {
            left: 0.5,
            right: 0.5,
            smoothing_samples: 96,
        };
    });
    let ramps = LiveRamps {
        fader_samples: 480,
        mute_samples: 240,
    };
    let delta = classify_live_delta(&current, &next, ramps).expect("live");
    assert_eq!(
        delta.strips,
        vec![LiveStripRecords {
            strip_id: next.tracks[0].id.as_str(),
            fader: [
                Some(fader_db(Both, -6.0, 480)),
                Some(mute(Both, true, 240)),
                None,
                None
            ],
            matrix: Some(TrackControlRecord {
                matrix: pan_matrix(0.5, 0.5).expect("pan"),
                smoothing_samples: 96,
            }),
        }]
    );
    assert_eq!(LiveRamps::for_session(&next), STEP);
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
    let delta = classify_live_delta(&current, &next, STEP).expect("live");
    assert_eq!(
        delta.strips.len(),
        8,
        "every edited track but the sign-of-zero one"
    );

    let request = HostLiveControlRequest {
        control_queue_depth: Some(NonZeroUsize::new(16).expect("depth")),
        ..HostLiveControlRequest::default()
    };
    let (mut live, mut handles) =
        prepare_host_runtime_with_live_controls(&current_compiled, &caps(), &request)
            .unwrap_or_else(|failure| {
                panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
            });
    push_delta(&mut handles, &delta);
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
    let unmute = classify_live_delta(&next, &unmuted, STEP).expect("live");
    assert_eq!(
        flatten(&unmute),
        vec![(
            track_id(&next, 8),
            vec![mute(BuiltinLaneSelector::Left, false, 0)],
            None
        )]
    );
    push_delta(&mut handles, &unmute);
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

/// Pushes every record of `delta` to its strip's lanes.
fn push_delta(handles: &mut host_core::HostLiveControlHandles, delta: &LiveDelta<'_>) {
    for strip in &delta.strips {
        let control = handles
            .strip_controls
            .iter_mut()
            .find(|control| &*control.track_id == strip.strip_id)
            .expect("strip lane");
        for record in strip.fader_records() {
            control.fader.try_push(record).expect("fader queue room");
        }
        if let Some(record) = strip.matrix {
            control
                .producer
                .try_push(record)
                .expect("matrix queue room");
        }
    }
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
    classify_live_delta(current, &next, STEP).map(|delta| {
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

/// #1264 gate 1(b). Red if a rewritten representation emits a redundant record: a `Both` value
/// split into per-lane values records only the lane whose value changes.
#[test]
fn a_both_value_split_into_lanes_records_only_the_changed_lane() {
    let mut current = with_effects();
    inserts(&mut current, 0).push(param(1, ParameterChannel::Both, ParameterUnit::Db, -20.0));
    let current = normalized(&current);
    assert_eq!(
        classify_effects(&current, |next| {
            *inserts(next, 0) = vec![
                param(1, ParameterChannel::Left, ParameterUnit::Db, -20.0),
                param(1, ParameterChannel::Right, ParameterUnit::Db, -30.0),
            ];
        }),
        Ok((
            Vec::new(),
            vec![(
                track_id(&current, 0),
                LiveEffectAddress::insert(0),
                vec![record(0, Lane::Right, -30.0)]
            )]
        ))
    );
    // The same values in the other representation: no record at all.
    assert_eq!(
        classify_effects(&current, |next| {
            *inserts(next, 0) = vec![
                param(1, ParameterChannel::Left, ParameterUnit::Db, -20.0),
                param(1, ParameterChannel::Right, ParameterUnit::Db, -20.0),
            ];
        }),
        Ok((Vec::new(), Vec::new()))
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
        Err(LiveRebuild::Prepared)
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
            Err(LiveRebuild::Prepared),
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

/// #1264 gate 1(f). Red if the params mask also hides a structural change of the instances: an
/// insert reorder, or a bypass flip (#1266's) beside a parameter change.
#[test]
fn an_insert_reorder_is_structural() {
    let current = with_effects();
    assert_eq!(
        classify_effects(&current, |next| next.tracks[0].inserts.effects.swap(0, 1)),
        Err(LiveRebuild::Structure)
    );
    assert_eq!(
        classify_effects(&current, |next| {
            next.tracks[0].inserts.effects[0].bypass = true;
            inserts(next, 0).push(param(1, ParameterChannel::Left, ParameterUnit::Db, -30.0));
        }),
        Err(LiveRebuild::Structure)
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
    let request = HostLiveControlRequest {
        control_queue_depth: Some(NonZeroUsize::new(16).expect("depth")),
        ..HostLiveControlRequest::default()
    };
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
    let delta = classify_live_delta(current, &next, STEP)?;
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
/// ones preparation gave the EQ's owner, designs at another rate, or passes other edits than the
/// changed rows: its targets must equal `EqTargetPreparer`'s for the owner's committed rows.
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

/// #1265 gate 1(d). Red if the classifier admits an EQ value preparation refuses (a Q above its
/// domain), so that a live commit would leave a committed model its own rebuild refuses.
#[test]
fn an_out_of_domain_eq_q_needs_a_rebuild() {
    let current = with_shaped_eq();
    assert_eq!(
        eq_entry(&current, |next| {
            inserts(next, 2).push(param(5, ParameterChannel::Left, ParameterUnit::Ratio, 18.5));
        }),
        Err(LiveRebuild::Domain)
    );
    // The domain's own bound is live.
    assert!(
        eq_entry(&current, |next| {
            inserts(next, 2).push(param(5, ParameterChannel::Left, ParameterUnit::Ratio, 18.0));
        })
        .is_ok()
    );
}
