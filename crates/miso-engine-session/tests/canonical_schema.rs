//! Canonical fixture and full tagged-surface round-trip checks.

use miso_engine_session::{
    CompileCaps, Effect, EffectIdentity, EffectQuality, LinkMode, MatrixOrPan, ParameterChannel,
    Rack, Route, RouteDestination, RouteSource, SendTap, Sidechain, SidechainDeclaration, StableId,
    canonical_session_toml, compile_session, parse_session_toml,
};

const REPRESENTATIVE: &str = include_str!("../../../fixtures/session/v1/canonical.toml");
const MINIMAL: &str = include_str!("../../../fixtures/session/v1/canonical-minimal.toml");
const PARAMETRIC_EQ: &str =
    include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.toml");

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("valid test ID")
}

fn unlimited_caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

#[test]
fn checked_in_fixtures_are_exact_canonical_bytes() {
    for fixture in [MINIMAL, REPRESENTATIVE] {
        let model = parse_session_toml(fixture).expect("fixture parses");
        assert_eq!(canonical_session_toml(&model).expect("canonical"), fixture);
    }
}

#[test]
fn parametric_eq_session_fixture_bytes_are_immutable() {
    let hash = PARAMETRIC_EQ
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
        });
    assert_eq!(PARAMETRIC_EQ.len(), 9_475);
    assert_eq!(hash, 0x96a3_be36_fc01_31fa);
}

#[test]
fn signed_zero_and_double_rounding_values_survive_session_compilation() {
    let positive = f32::from_bits(0x15ae_43fd);
    let negative = f32::from_bits(0x95ae_43fd);
    let mut model = parse_session_toml(REPRESENTATIVE).expect("fixture parses");
    model.routes[0].channel_matrix.lr = -0.0;
    model.tracks[0].builtins.left.trim_db = positive;
    model.routes[0].gain_db = negative;

    let canonical = canonical_session_toml(&model).expect("direct canonicalization");
    assert!(canonical.contains("lr = -0.0"));
    assert!(canonical.contains(&format!("trim_db = {}", f64::from(positive))));
    assert!(canonical.contains(&format!("gain_db = {}", f64::from(negative))));
    let reparsed = parse_session_toml(&canonical).expect("canonical reparses");
    assert_eq!(reparsed.routes[0].channel_matrix.lr.to_bits(), 0x8000_0000);
    assert_eq!(
        reparsed.tracks[0].builtins.left.trim_db.to_bits(),
        0x15ae_43fd
    );
    assert_eq!(reparsed.routes[0].gain_db.to_bits(), 0x95ae_43fd);

    let compiled = compile_session(&reparsed, unlimited_caps()).expect("session compiles");
    let normalized = compiled.normalized_model();
    assert_eq!(
        normalized.routes[0].channel_matrix.lr.to_bits(),
        0x8000_0000
    );
    assert_eq!(
        normalized.tracks[0].builtins.left.trim_db.to_bits(),
        0x15ae_43fd
    );
    assert_eq!(normalized.routes[0].gain_db.to_bits(), 0x95ae_43fd);
    assert_eq!(compiled.canonical_toml(), canonical);
    assert_eq!(
        canonical_session_toml(normalized).expect("normalized recanonicalizes"),
        canonical
    );
}

#[test]
fn maximal_float_spellings_fit_the_canonical_size_estimate() {
    let tiny = f32::from_bits(1);
    let mut model = parse_session_toml(REPRESENTATIVE).expect("fixture parses");
    let track = &mut model.tracks[0];
    for channel in [&mut track.builtins.left, &mut track.builtins.right] {
        channel.trim_db = tiny;
        channel.hpf_hz = tiny;
        channel.lpf_hz = tiny;
    }
    track.fader.left_db = tiny;
    track.fader.right_db = tiny;
    track.matrix_or_pan = MatrixOrPan::Pan {
        left: tiny,
        right: tiny,
        smoothing_samples: 0,
    };
    let compiled = compile_session(&model, unlimited_caps())
        .expect("ten maximal float spellings fit the preflight estimate");
    assert!(
        compiled.resource_estimate().canonical_bytes
            <= compiled.resource_estimate().canonical_upper_bound_bytes
    );
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
