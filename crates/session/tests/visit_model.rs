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
    let expected_root = 7
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
        declared.records.contains(&(None, 10)),
        "track has ten wire fields"
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
