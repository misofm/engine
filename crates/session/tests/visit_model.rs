//! Public visitor schema/count/tag/order contract tests.

use std::convert::Infallible;

use session::{
    FieldKey, MatrixOrPan, ModelVisitor, Sidechain, SidechainDeclaration, StableId, Token,
    VisitModel, WalkOrder, keys, parse_session_json,
};

const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

#[derive(Default)]
struct Trace {
    records: Vec<(Option<FieldKey>, u32)>,
    arrays: Vec<(FieldKey, usize)>,
    tags: Vec<Token>,
    ids: Vec<(FieldKey, String)>,
}
impl ModelVisitor for Trace {
    type Error = Infallible;
    fn record_begin(&mut self, key: Option<FieldKey>, fields: u32) -> Result<(), Self::Error> {
        self.records.push((key, fields));
        Ok(())
    }
    fn record_end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn array_begin(&mut self, key: FieldKey, len: usize) -> Result<(), Self::Error> {
        self.arrays.push((key, len));
        Ok(())
    }
    fn array_end(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn wire_tag(&mut self, tag: Token) -> Result<(), Self::Error> {
        self.tags.push(tag);
        Ok(())
    }
    fn bool(&mut self, _: FieldKey, _: bool) -> Result<(), Self::Error> {
        Ok(())
    }
    fn u8(&mut self, _: FieldKey, _: u8) -> Result<(), Self::Error> {
        Ok(())
    }
    fn u32(&mut self, _: FieldKey, _: u32) -> Result<(), Self::Error> {
        Ok(())
    }
    fn u64(&mut self, _: FieldKey, _: u64) -> Result<(), Self::Error> {
        Ok(())
    }
    fn source_bit_depth(
        &mut self,
        _: FieldKey,
        _: session::SourceBitDepth,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn f32(&mut self, _: FieldKey, _: f32) -> Result<(), Self::Error> {
        Ok(())
    }
    fn id(&mut self, key: FieldKey, value: &session::StableId) -> Result<(), Self::Error> {
        self.ids.push((key, value.as_str().to_owned()));
        Ok(())
    }
    fn text(&mut self, _: FieldKey, _: &str) -> Result<(), Self::Error> {
        Ok(())
    }
    fn token(&mut self, _: FieldKey, _: Token) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[test]
fn visitor_counts_keys_tags_and_conditional_canonical_order_are_exact() {
    let mut model = parse_session_json(EXAMPLE).expect("fixture");
    let mut second = model.sources[0].clone();
    second.id = session::StableId::parse("alpha-source").expect("id");
    model.sources.insert(0, second);
    let expected_root = 8
        + model.sources.len()
        + model.tracks.len()
        + model.submixes.len()
        + model.outputs.len()
        + model.routes.len()
        + model.automation.len();

    let mut declared = Trace::default();
    model.visit(WalkOrder::Declared, &mut declared).unwrap();
    assert_eq!(declared.records[0], (None, expected_root as u32));
    assert!(
        declared.records.contains(&(None, 8)),
        "a track with no console entries has eight wire fields"
    );
    // Issue #210 phase 2 moved this from four to five. The count is the BTLV field count the
    // visitor declares for `ChannelBuiltins`; a model field added without it stays out of the wire
    // form silently, which is exactly the drift this row exists to catch.
    assert_eq!(
        declared
            .records
            .iter()
            .filter(|(key, _)| *key == Some(keys::builtins::LEFT)
                || *key == Some(keys::builtins::RIGHT))
            .map(|(_, fields)| *fields)
            .collect::<Vec<_>>(),
        vec![5; model.tracks.len() * 2],
        "every channel-builtins table declares five wire fields"
    );
    assert!(declared.arrays.contains(&(keys::rack::EFFECTS, 1)));
    assert_eq!(
        &declared.ids[3..5]
            .iter()
            .map(|(_, id)| id.as_str())
            .collect::<Vec<_>>(),
        &["alpha-source", "voice"]
    );

    model.sources.swap(0, 1);
    let mut canonical = Trace::default();
    model.visit(WalkOrder::Canonical, &mut canonical).unwrap();
    assert_eq!(
        &canonical.ids[3..5]
            .iter()
            .map(|(_, id)| id.as_str())
            .collect::<Vec<_>>(),
        &["alpha-source", "voice"]
    );

    let mut variants = Trace::default();
    MatrixOrPan::Pan {
        left: 1.0,
        right: 1.0,
        smoothing_samples: 1,
    }
    .visit(WalkOrder::Declared, &mut variants)
    .unwrap();
    MatrixOrPan::Matrix {
        ll: 1.0,
        lr: 0.0,
        rl: 0.0,
        rr: 1.0,
        smoothing_samples: 1,
    }
    .visit(WalkOrder::Declared, &mut variants)
    .unwrap();
    assert_eq!(
        variants.tags,
        [
            Token {
                text: "pan",
                wire: 1
            },
            Token {
                text: "matrix",
                wire: 2
            }
        ]
    );

    let sidechain = SidechainDeclaration::Routed(Sidechain {
        source: model.routes[0].source.clone(),
        port_id: StableId::parse("detector").expect("id"),
    });
    let mut routed = Trace::default();
    sidechain.visit(WalkOrder::Declared, &mut routed).unwrap();
    assert_eq!(
        routed.records.len(),
        2,
        "routed sidechain is inline plus its route-source record"
    );

    assert_eq!(
        [
            keys::session::SCHEMA_VERSION.id,
            keys::session::AUTOMATION.id
        ],
        [1, 14]
    );
    assert_eq!([keys::track::ID.id, keys::track::MATRIX.id], [1, 10]);
    assert_eq!([keys::effect::ID.id, keys::effect::SIDECHAIN.id], [1, 7]);
}

/// Decision 12's field registry: `inserts` keeps the retired `dynamic` rack's track field 7, the
/// retired `simd1`/`simd2` fields 6 and 8 are never reallocated, `console` is appended as the next
/// unallocated ID at the root (15) and on a track (11), and the nested console messages have
/// registries of their own. The visitor walks the root console before the tracks, and a track's
/// console entries between its builtins and its inserts, each entry counted as a repeated field.
#[test]
fn console_fields_are_appended_and_walked_in_schema_order() {
    assert_eq!(keys::session::CONSOLE.id, 15);
    assert_eq!([keys::track::CONSOLE.id, keys::track::INSERTS.id], [11, 7]);
    assert_eq!(
        [keys::console::PRE_INSERT.id, keys::console::POST_INSERT.id],
        [1, 2]
    );
    assert_eq!(
        [
            keys::console_slot::SLOT.id,
            keys::console_slot::IDENTITY.id,
            keys::console_slot::QUALITY.id,
            keys::console_slot::LINK_MODE.id
        ],
        [1, 2, 3, 4]
    );
    assert_eq!(
        [
            keys::console_entry::SLOT.id,
            keys::console_entry::BYPASS.id,
            keys::console_entry::PARAMS.id
        ],
        [1, 2, 3]
    );

    let mut model = parse_session_json(EXAMPLE).expect("fixture");
    let slot = |name: &str| session::ConsoleSlot {
        slot: StableId::parse(name).expect("id"),
        identity: session::EffectIdentity::Native {
            effect_id: StableId::parse("miso.compressor").expect("id"),
        },
        quality: session::EffectQuality::Normal,
        link_mode: session::LinkMode::DualMono,
    };
    model.console.pre_insert = vec![slot("desk-a")];
    model.console.post_insert = vec![slot("desk-b")];
    let params = model.tracks[0].inserts.effects[0].params.clone();
    model.tracks[0].console = vec![
        session::ConsoleEntry {
            slot: StableId::parse("desk-a").expect("id"),
            bypass: false,
            params: params.clone(),
        },
        session::ConsoleEntry {
            slot: StableId::parse("desk-b").expect("id"),
            bypass: true,
            params: Vec::new(),
        },
    ];
    let mut trace = Trace::default();
    model.visit(WalkOrder::Declared, &mut trace).unwrap();
    assert!(
        trace.records.contains(&(Some(keys::session::CONSOLE), 2)),
        "the root console counts one repeated field per slot"
    );
    assert!(
        trace.records.contains(&(None, 10)),
        "a track with two console entries has 8 + 2 wire fields"
    );
    assert!(
        trace.records.contains(&(None, 2 + params.len() as u32)),
        "a console entry counts its params as repeated fields"
    );
    assert!(trace.arrays.contains(&(keys::console::PRE_INSERT, 1)));
    assert!(trace.arrays.contains(&(keys::console::POST_INSERT, 1)));
    assert!(trace.arrays.contains(&(keys::track::CONSOLE, 2)));
    let console_array = trace
        .arrays
        .iter()
        .position(|(key, _)| *key == keys::track::CONSOLE)
        .expect("track console array");
    let tracks_array = trace
        .arrays
        .iter()
        .position(|(key, _)| *key == keys::session::TRACKS)
        .expect("tracks array");
    let pre_insert_array = trace
        .arrays
        .iter()
        .position(|(key, _)| *key == keys::console::PRE_INSERT)
        .expect("pre_insert array");
    assert!(
        pre_insert_array < tracks_array && tracks_array < console_array,
        "root console, then tracks, then each track's console entries"
    );
    assert!(
        trace
            .records
            .iter()
            .any(|(key, _)| *key == Some(keys::track::INSERTS)),
        "the inserts rack is walked"
    );
    let ids: Vec<&str> = trace.ids.iter().map(|(_, id)| id.as_str()).collect();
    let desk_a = ids.iter().position(|id| *id == "desk-a").expect("slot id");
    let vocal = ids.iter().position(|id| *id == "vocal").expect("track id");
    assert!(desk_a < vocal, "slots are walked before tracks");
}
