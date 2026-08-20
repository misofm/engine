//! Canonical fixture and full tagged-surface round-trip checks.

use miso_engine_session::{
    Effect, EffectIdentity, EffectQuality, LinkMode, MatrixOrPan, ParameterChannel, Rack, Route,
    RouteDestination, RouteSource, SendTap, Sidechain, SidechainDeclaration, StableId,
    canonical_session_toml, parse_session_toml,
};

const REPRESENTATIVE: &str = include_str!("../../../fixtures/session/v1/canonical.toml");
const MINIMAL: &str = include_str!("../../../fixtures/session/v1/canonical-minimal.toml");

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("valid test ID")
}

#[test]
fn checked_in_fixtures_are_exact_canonical_bytes() {
    for fixture in [MINIMAL, REPRESENTATIVE] {
        let model = parse_session_toml(fixture).expect("fixture parses");
        assert_eq!(canonical_session_toml(&model).expect("canonical"), fixture);
    }
}

#[test]
fn full_tagged_surface_round_trips_without_field_loss() {
    let mut model = parse_session_toml(REPRESENTATIVE).expect("fixture parses");
    model
        .submixes
        .push(miso_engine_session::Submix { id: id("mix") });
    model.tracks[0].matrix_or_pan = MatrixOrPan::Matrix {
        ll: 1.25,
        lr: -0.25,
        rl: 0.5,
        rr: 0.75,
        smoothing_samples: 32,
    };
    model.tracks[0].simd1 = Rack {
        effects: vec![Effect {
            id: id("external"),
            identity: EffectIdentity::ThirdPartyCid {
                cid: "bafyopaque-v1-text".to_owned(),
            },
            quality: EffectQuality::High,
            bypass: true,
            link_mode: LinkMode::Maximum,
            params: Vec::new(),
            sidechain: SidechainDeclaration::Routed(Sidechain {
                source: RouteSource::Track {
                    track_id: id("vocal"),
                    tap: SendTap::PostFader,
                },
                port_id: id("detector-in"),
            }),
        }],
    };
    model.routes[0].destination = RouteDestination::SubmixInput {
        submix_id: id("mix"),
    };
    model.routes.push(Route {
        id: id("mix-to-main"),
        source: RouteSource::SubmixOutput {
            submix_id: id("mix"),
        },
        destination: RouteDestination::OutputInput {
            output_id: id("main-out"),
        },
        channel_matrix: miso_engine_session::ChannelMatrix {
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
        },
        gain_db: 0.0,
    });
    model.automation[0].target.channel = ParameterChannel::Both;

    let canonical = canonical_session_toml(&model).expect("full surface canonicalizes");
    let reparsed = parse_session_toml(&canonical).expect("full surface reparses");
    assert!(matches!(
        reparsed.tracks[0].matrix_or_pan,
        MatrixOrPan::Matrix { .. }
    ));
    assert!(matches!(
        reparsed.tracks[0].simd1.effects[0].sidechain,
        SidechainDeclaration::Routed(_)
    ));
    assert!(
        reparsed
            .routes
            .iter()
            .any(|route| matches!(route.source, RouteSource::SubmixOutput { .. }))
    );
    assert_eq!(
        canonical_session_toml(&reparsed).expect("stable"),
        canonical
    );
}
