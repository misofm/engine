//! Issue #1242 D2: a following send reads its source strip's effective mute, the VCA's mute
//! included (VERIFY-2 M12), from the one composition (`SessionModel::effective_strip_faders`).

use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry, prepare_native_session_effects,
};
use graph::GraphCompileCaps;
use graph_compiler::{Backend, GraphBuiltinsCompileRequest, GraphCompiler};
use session::{
    CompileCaps, DualMonoFader, Route, RouteDestination, RouteSource, SendTap, SessionModel,
    StableId, Submix, Vca, compile_session, parse_session_json,
};

/// Nine tracks, `eq0` to `eq8`, each routed to the session output by its own route.
const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

fn sid(value: &str) -> StableId {
    StableId::parse(value).expect("literal stable ID")
}

/// The sealed canonical graph text of `model`'s production compile, without its `estimate` row
/// (which counts the session's own bytes, so a declared VCA moves it).
fn text(model: &SessionModel) -> Vec<String> {
    let session = compile_session(
        model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("the session compiles");
    let builtins = prepare_session_builtins(
        &session,
        &[],
        BuiltinCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_total_retained_payload_bytes: u64::MAX,
            maximum_total_meter_items: u64::MAX,
            maximum_total_meter_bytes: u64::MAX,
            maximum_single_allocation_bytes: u64::MAX,
            maximum_meter_streams: u64::MAX,
            maximum_period_frames: u32::MAX,
            maximum_peak_hold_frames: u32::MAX,
            maximum_smoothing_samples: u32::MAX,
        },
    )
    .expect("the builtins prepare");
    let effects = prepare_native_session_effects(
        &session,
        &launch_native_effect_registry().expect("launch registry"),
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 24,
            maximum_scratch_bytes: 1 << 24,
            maximum_automation_spans_per_block: 128,
        },
    )
    .expect("the native effects prepare");
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch: Backend::current(),
        plan_id: 1,
        effects,
        builtins,
        caps: GraphCompileCaps {
            maximum_nodes: u64::MAX,
            maximum_edges: u64::MAX,
            maximum_schedule_items: u64::MAX,
            maximum_dependency_levels: u64::MAX,
            maximum_audio_buffer_samples: u64::MAX,
            maximum_delay_samples_per_edge: u64::MAX,
            maximum_total_delay_samples: u64::MAX,
            maximum_graph_bytes: u64::MAX,
            maximum_plan_bytes: u64::MAX,
            maximum_single_allocation_bytes: u64::MAX,
            maximum_finite_tail_samples: u64::MAX,
        },
    })
    .unwrap_or_else(|failure| panic!("graph: {:?}", failure.diagnostics));
    String::from_utf8(GraphCompiler::evidence(artifact.graph(), artifact.report()).canonical_bytes)
        .expect("canonical text is UTF-8")
        .lines()
        .filter(|line| !line.starts_with("estimate\t"))
        .map(str::to_owned)
        .collect()
}

/// The nine-track fixture with bus `grp` (fed by `eq4`) and bus `verb`: `eq3` and `grp` each send
/// from `pre_fader` into `verb` with `follows_mute`, and `eq2` sends from `post_fader` without it.
/// VCA `drums` holds `eq2`, `eq3` and, through the nested VCA `kit`, `grp`.
fn vca_session(mutes: [bool; 2]) -> SessionModel {
    let mut model = parse_session_json(SESSION).expect("fixture parses");
    let template = model.routes[0].clone();
    let route =
        |id: &str, source: RouteSource, destination: RouteDestination, follows: bool| Route {
            id: sid(id),
            source,
            destination,
            follows_mute: follows,
            ..template.clone()
        };
    let into = |bus: &str| RouteDestination::SubmixInput {
        submix_id: sid(bus),
    };
    let track = |id: &str, tap| RouteSource::Track {
        track_id: sid(id),
        tap,
    };
    let bus = |id: &str, tap| RouteSource::Submix {
        submix_id: sid(id),
        tap,
    };
    for name in ["grp", "verb"] {
        model
            .submixes
            .push(Submix::unity(sid(name), &model.console));
    }
    model.routes.extend([
        route(
            "eq4-grp",
            track("eq4", SendTap::PostPan),
            into("grp"),
            false,
        ),
        route(
            "grp-verb",
            bus("grp", SendTap::PreFader),
            into("verb"),
            true,
        ),
        route(
            "kick-verb",
            track("eq2", SendTap::PostFader),
            into("verb"),
            false,
        ),
        route(
            "snare-verb",
            track("eq3", SendTap::PreFader),
            into("verb"),
            true,
        ),
        route(
            "verb-main",
            bus("verb", SendTap::PostPan),
            template.destination.clone(),
            false,
        ),
    ]);
    let fader = |db: f32, mutes: [bool; 2]| DualMonoFader {
        left_db: db,
        right_db: db,
        left_mute: mutes[0],
        right_mute: mutes[1],
    };
    model.vcas = vec![
        Vca {
            id: sid("kit"),
            fader: fader(0.0, [false; 2]),
            members: vec![sid("grp")],
        },
        Vca {
            id: sid("drums"),
            fader: fader(-6.0, mutes),
            members: vec![sid("eq2"), sid("eq3"), sid("kit")],
        },
    ];
    model
}

/// `model` with no VCA and every strip's own fader set to its effective value.
fn written_directly(model: &SessionModel) -> SessionModel {
    let effective = model.effective_strip_faders();
    let mut plain = model.clone();
    plain.vcas.clear();
    let faders = plain
        .tracks
        .iter_mut()
        .map(|track| &mut track.fader)
        .chain(plain.submixes.iter_mut().map(|submix| &mut submix.fader));
    for (fader, value) in faders.zip(effective) {
        (fader.left_db, fader.right_db) = (value.db[0], value.db[1]);
        (fader.left_mute, fader.right_mute) = (value.mute[0], value.mute[1]);
    }
    plain
}

/// #1242 gate 3, the sealed text. With VCA `drums` muted on the left lane, or on both, the sealed
/// graph text of the VCA session equals that of the same session with no VCA and each member's
/// own mutes set to its effective ones: `snare-verb` and, through the nested VCA, `grp-verb`
/// carry a `route-follow-zeroed` row with the VCA's lanes, and `kick-verb`, which does not
/// follow, carries none. With the VCA unmuted no route carries the row.
///
/// Red if route lowering reads the member's own mute (a fresh plan leaks a VCA-muted member's
/// pre-fader send), or reads it for tracks only (the `grp` row is missing).
#[test]
fn a_vca_muted_member_seals_its_follow_zeroed_rows() {
    let unmuted = text(&vca_session([false; 2]));
    assert!(
        unmuted
            .iter()
            .all(|line| !line.starts_with("route-follow-zeroed")),
        "an unmuted VCA zeroes no follow"
    );
    for mutes in [[true, false], [true, true]] {
        let model = vca_session(mutes);
        let actual = text(&model);
        assert_eq!(actual, text(&written_directly(&model)), "mutes {mutes:?}");
        let rows: Vec<&String> = actual
            .iter()
            .filter(|line| line.starts_with("route-follow-zeroed"))
            .collect();
        let lanes = format!("{}\t{}", u8::from(mutes[0]), u8::from(mutes[1]));
        assert_eq!(
            rows,
            [
                &format!("route-follow-zeroed\troute:grp-verb\t{lanes}"),
                &format!("route-follow-zeroed\troute:snare-verb\t{lanes}"),
            ],
            "mutes {mutes:?}"
        );
    }
}
