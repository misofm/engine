//! The session every test in this crate reads: two tracks, a submix, a send into the submix, a
//! pre-fader and a post-fader route into the output, and a VCA.
#![allow(dead_code)]

use controls::{ControlAddress, ControlRow, Lane, Owner, Section};
use session::{
    ChannelMatrix, DualMonoFader, MatrixOrPan, Route, RouteDestination, RouteSource, SendTap,
    SessionModel, StableId, Submix, Vca, parse_session_json,
};

pub(crate) const RATE: u32 = 48_000;

pub(crate) fn id(value: &str) -> StableId {
    StableId::parse(value).expect("valid test ID")
}

pub(crate) fn address(owner: Owner, section: Section, control: u32, lane: Lane) -> ControlAddress {
    ControlAddress {
        owner,
        section,
        control,
        lane,
    }
}

pub(crate) fn strip_builtin(strip: &str, control: u32, lane: Lane) -> ControlAddress {
    address(Owner::Strip(id(strip)), Section::Builtins, control, lane)
}

pub(crate) fn strip_own(strip: &str, control: u32, lane: Lane) -> ControlAddress {
    address(Owner::Strip(id(strip)), Section::Own, control, lane)
}

pub(crate) fn route_own(route: &str, control: u32) -> ControlAddress {
    address(Owner::Route(id(route)), Section::Own, control, Lane::Both)
}

fn route(
    name: &str,
    source: RouteSource,
    destination: RouteDestination,
    follows_mute: bool,
) -> Route {
    Route {
        id: id(name),
        source,
        destination,
        channel_matrix: ChannelMatrix {
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
        },
        gain_db: 0.0,
        mute: false,
        follows_mute,
    }
}

/// Tracks `vocal` (pan mode) and `guitar` (matrix mode), submix `bus` (matrix mode), VCA `riders`
/// over `vocal`, and five routes.
pub(crate) fn fixture() -> SessionModel {
    let mut model = parse_session_json(include_str!(
        "../../../../fixtures/session/v1/canonical.json"
    ))
    .expect("fixture parses");
    let mut guitar = model.tracks[0].clone();
    guitar.id = id("guitar");
    guitar.matrix_or_pan = MatrixOrPan::Matrix {
        ll: 0.5,
        lr: 0.25,
        rl: -0.25,
        rr: 0.75,
        smoothing_samples: 32,
    };
    model.tracks.push(guitar);
    model
        .submixes
        .push(Submix::unity(id("bus"), &model.console));
    model.vcas.push(Vca {
        id: id("riders"),
        fader: DualMonoFader {
            left_db: -6.0,
            right_db: -3.0,
            left_mute: true,
            right_mute: false,
        },
        members: vec![id("vocal")],
    });
    let output = || RouteDestination::OutputInput {
        output_id: id("main-out"),
    };
    let from = |track: &str, tap| RouteSource::Track {
        track_id: id(track),
        tap,
    };
    model.routes = vec![
        route(
            "post-fader-out",
            from("vocal", SendTap::PostFader),
            output(),
            false,
        ),
        route(
            "pre-fader-out",
            from("guitar", SendTap::PreFader),
            output(),
            false,
        ),
        route(
            "send",
            from("vocal", SendTap::PostFader),
            RouteDestination::SubmixInput {
                submix_id: id("bus"),
            },
            true,
        ),
        route(
            "bus-out",
            RouteSource::Submix {
                submix_id: id("bus"),
                tap: SendTap::PostPan,
            },
            output(),
            false,
        ),
    ];
    model.automation.clear();
    model
}

/// The value of the row at `address` in a fresh listing.
pub(crate) fn row_of(model: &SessionModel, address: &ControlAddress) -> ControlRow {
    controls::list(model, RATE)
        .find(|row| row.address == *address)
        .expect("listed address")
}
