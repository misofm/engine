//! Canonical fixture and full tagged-surface round-trip checks.

use session::{
    AutomationTarget, CompileCaps, Console, ConsoleEntry, ConsoleSlot, Effect, EffectIdentity,
    EffectParam, EffectQuality, LinkMode, MatrixOrPan, ParameterChannel, ParameterUnit, RackName,
    Route, RouteDestination, RouteSource, SendTap, Sidechain, SidechainDeclaration, StableId,
    canonical_session_json, compile_session, parse_session_json,
};

const REPRESENTATIVE: &str = include_str!("../../../fixtures/session/v1/canonical.json");
const MINIMAL: &str = include_str!("../../../fixtures/session/v1/canonical-minimal.json");

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
        let model = parse_session_json(fixture).expect("fixture parses");
        assert_eq!(canonical_session_json(&model).expect("canonical"), fixture);
    }
}

#[test]
fn signed_zero_and_double_rounding_values_survive_session_compilation() {
    let positive = f32::from_bits(0x15ae_43fd);
    let negative = f32::from_bits(0x95ae_43fd);
    let mut model = parse_session_json(REPRESENTATIVE).expect("fixture parses");
    model.routes[0].channel_matrix.lr = -0.0;
    model.tracks[0].builtins.left.trim_db = positive;
    model.routes[0].gain_db = negative;

    let canonical = canonical_session_json(&model).expect("direct canonicalization");
    assert!(canonical.contains("\"lr\": -0.0"));
    assert!(canonical.contains(&format!("\"trim_db\": {}", f64::from(positive))));
    assert!(canonical.contains(&format!("\"gain_db\": {}", f64::from(negative))));
    let reparsed = parse_session_json(&canonical).expect("canonical reparses");
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
    assert_eq!(compiled.canonical_json(), canonical);
    assert_eq!(
        canonical_session_json(normalized).expect("normalized recanonicalizes"),
        canonical
    );

    let canonical_ptr = compiled.canonical_json().as_ptr();
    let owned = compiled.into_canonical_json();
    assert_eq!(owned, canonical);
    assert_eq!(owned.as_ptr(), canonical_ptr);
}

#[test]
fn maximal_float_spellings_fit_the_canonical_size_estimate() {
    let tiny = f32::from_bits(1);
    let mut model = parse_session_json(REPRESENTATIVE).expect("fixture parses");
    let track = &mut model.tracks[0];
    for channel in [&mut track.builtins.left, &mut track.builtins.right] {
        channel.trim_db = tiny;
        channel.hpf_hz = tiny;
        channel.lpf_hz = tiny;
        // Not a float, but it is the widest spelling this key has: the canonical estimate must
        // cover a five-digit integer on every lane too (#210 phase 2).
        channel.delay_samples = session::CHANNEL_BUILTIN_DELAY_SAMPLES_MAXIMUM;
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

fn native(effect_id: &str) -> EffectIdentity {
    EffectIdentity::Native {
        effect_id: id(effect_id),
    }
}

fn param(parameter_id: u32, channel: ParameterChannel, value: f32) -> EffectParam {
    EffectParam {
        parameter_id,
        channel,
        unit: ParameterUnit::Db,
        value,
    }
}

/// A console with one slot in each section, and the one fixture track's entries for them.
fn with_console(model: &mut session::SessionModel) {
    model.console = Console {
        pre_insert: vec![ConsoleSlot {
            slot: id("desk-eq"),
            identity: native("miso.parametric-eq"),
            quality: EffectQuality::High,
            link_mode: LinkMode::DualMono,
        }],
        post_insert: vec![ConsoleSlot {
            slot: id("desk-limit"),
            identity: native("miso.true-peak-limiter"),
            quality: EffectQuality::Normal,
            link_mode: LinkMode::Maximum,
        }],
    };
    for track in &mut model.tracks {
        track.console = vec![
            ConsoleEntry {
                slot: id("desk-eq"),
                bypass: true,
                // Deliberately out of `(parameter_id, channel)` order: canonical output sorts.
                params: vec![
                    param(2, ParameterChannel::Right, -1.5),
                    param(2, ParameterChannel::Left, 1.5),
                ],
            },
            ConsoleEntry {
                slot: id("desk-limit"),
                bypass: false,
                params: Vec::new(),
            },
        ];
    }
}

#[test]
fn full_tagged_surface_round_trips_without_field_loss() {
    let mut model = parse_session_json(REPRESENTATIVE).expect("fixture parses");
    model.tracks[0].matrix_or_pan = MatrixOrPan::Matrix {
        ll: 1.25,
        lr: -0.25,
        rl: 0.5,
        rr: 0.75,
        smoothing_samples: 32,
    };
    with_console(&mut model);
    // After the console is declared, so the bus carries an entry for every slot (#1202 D4).
    model
        .submixes
        .push(session::Submix::unity(id("mix"), &model.console));
    model.tracks[0].inserts.effects.insert(
        0,
        Effect {
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
                    tap: SendTap::InsertSend,
                },
                port_id: id("detector-in"),
            }),
        },
    );
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
        channel_matrix: session::ChannelMatrix {
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
        },
        gain_db: 0.0,
    });
    model.automation[0].target.channel = ParameterChannel::Both;
    let mut console_automation = model.automation[0].clone();
    console_automation.id = id("desk-eq-gain");
    console_automation.target = AutomationTarget {
        entity_id: id("vocal"),
        rack: RackName::Console,
        effect_id: id("desk-eq"),
        parameter_id: 2,
        channel: ParameterChannel::Left,
    };
    model.automation.push(console_automation);

    let canonical = canonical_session_json(&model).expect("full surface canonicalizes");
    let reparsed = parse_session_json(&canonical).expect("full surface reparses");
    assert!(matches!(
        reparsed.tracks[0].matrix_or_pan,
        MatrixOrPan::Matrix { .. }
    ));
    assert!(matches!(
        reparsed.tracks[0].inserts.effects[0].sidechain,
        SidechainDeclaration::Routed(_)
    ));
    assert_eq!(reparsed.console, model.console, "slot declarations survive");
    assert_eq!(
        reparsed.tracks[0].console[0].params,
        [
            param(2, ParameterChannel::Left, 1.5),
            param(2, ParameterChannel::Right, -1.5)
        ],
        "a console entry's params are canonicalized by (parameter_id, channel)"
    );
    assert!(reparsed.tracks[0].console[0].bypass);
    assert!(
        reparsed
            .automation
            .iter()
            .any(|automation| automation.target.rack == RackName::Console)
    );
    assert!(
        reparsed
            .routes
            .iter()
            .any(|route| matches!(route.source, RouteSource::SubmixOutput { .. }))
    );
    assert_eq!(
        canonical_session_json(&reparsed).expect("stable"),
        canonical
    );
}

/// Decision 12 lets either console section and a track's inserts be empty. Each empty shape must
/// still write, reparse and rewrite byte for byte -- an empty array that the writer dropped, or a
/// parser that defaulted a missing `console`, would pass the populated fixture and fail here.
#[test]
fn empty_console_sections_and_empty_inserts_round_trip() {
    let full = {
        let mut model = parse_session_json(REPRESENTATIVE).expect("fixture parses");
        with_console(&mut model);
        model
    };
    type Shape = fn(&mut session::SessionModel);
    let shapes: [(&str, Shape); 4] = [
        ("everything empty", |model| {
            model.console.pre_insert.clear();
            model.console.post_insert.clear();
            model.tracks[0].console.clear();
            model.tracks[0].inserts.effects.clear();
            model.automation.clear();
        }),
        ("pre_insert only", |model| {
            model.console.post_insert.clear();
            model.tracks[0].console.truncate(1);
            model.tracks[0].inserts.effects.clear();
            model.automation.clear();
        }),
        ("post_insert only", |model| {
            model.console.pre_insert.clear();
            model.tracks[0].console.remove(0);
        }),
        ("both sections, empty inserts", |model| {
            model.tracks[0].inserts.effects.clear();
            model.automation.clear();
        }),
    ];
    for (name, shape) in shapes {
        let mut model = full.clone();
        shape(&mut model);
        // The reparsed model carries canonical parameter order.
        for entry in model.tracks.iter_mut().flat_map(|track| &mut track.console) {
            entry
                .params
                .sort_by_key(|param| (param.parameter_id, param.channel));
        }
        let canonical =
            canonical_session_json(&model).unwrap_or_else(|error| panic!("{name}: {error}"));
        assert!(
            canonical.contains("\"console\": {"),
            "{name}: the root console is written"
        );
        let reparsed =
            parse_session_json(&canonical).unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(reparsed, model, "{name}: no field is lost or invented");
        assert_eq!(
            canonical_session_json(&reparsed).expect("stable"),
            canonical,
            "{name}: canonical bytes are stable"
        );
    }
}
