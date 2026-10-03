//! VCA groups in the session (#1240): grammar, canonical form and validation.
//!
//! A VCA is a control-only fader whose offset and mute apply to every member it reaches; this
//! slice declares it, and preparation ignores it until #1242.

use serde_json::{Map, Value, json};
use session::{
    DiagnosticCode, DiagnosticSet, DualMonoFader, SessionModel, StableId, Submix, Vca,
    canonical_session_json, parse_session_json,
};

const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("valid test ID")
}

fn document(edit: impl FnOnce(&mut Map<String, Value>)) -> String {
    let mut root: Value = serde_json::from_str(EXAMPLE).expect("fixture JSON");
    edit(root.as_object_mut().expect("root object"));
    serde_json::to_string(&root).expect("serialize")
}

/// An in-place edit of a document's root object.
type RootEdit = fn(&mut Map<String, Value>);

fn vca_json(name: &str, members: Value) -> Value {
    json!({
        "id": name,
        "fader": {"left_db": -6.0, "right_db": 1.5, "left_mute": false, "right_mute": true},
        "members": members,
    })
}

fn with_vcas(vcas: Value) -> String {
    document(|root| {
        root.insert("vcas".to_owned(), vcas);
    })
}

fn diagnostics(error: &DiagnosticSet) -> Vec<(DiagnosticCode, String)> {
    error
        .diagnostics()
        .iter()
        .map(|item| (item.code, item.path.to_string()))
        .collect()
}

#[track_caller]
fn refused(source: &str, expected: &[(DiagnosticCode, &str)]) {
    let error = parse_session_json(source).expect_err("document must refuse");
    let expected: Vec<(DiagnosticCode, String)> = expected
        .iter()
        .map(|(code, path)| (*code, (*path).to_owned()))
        .collect();
    assert_eq!(diagnostics(&error), expected);
}

/// Gate 1, the grammar: `vcas` is required, and `members` is an array of stable-ID strings refused
/// at its own path otherwise.
#[test]
fn vcas_are_required_and_members_are_stable_id_strings() {
    let missing = document(|root| drop(root.remove("vcas")));
    refused(&missing, &[(DiagnosticCode::MissingField, "$.vcas")]);
    refused(
        &with_vcas(json!([vca_json("drums", json!("vocal"))])),
        &[(DiagnosticCode::WrongType, "$.vcas[0].members")],
    );
    refused(
        &with_vcas(json!([vca_json("drums", json!(["vocal", 7]))])),
        &[(DiagnosticCode::WrongType, "$.vcas[0].members[1]")],
    );
    refused(
        &with_vcas(json!([vca_json("drums", json!(["vocal", "Bad"]))])),
        &[(DiagnosticCode::InvalidId, "$.vcas[0].members[1]")],
    );
    refused(
        &with_vcas(
            json!([{"id": "drums", "fader": {"left_db": 0.0, "right_db": 0.0, "left_mute": false, "right_mute": false}}]),
        ),
        &[(DiagnosticCode::MissingField, "$.vcas[0].members")],
    );
    refused(
        &with_vcas(json!([{"id": "drums", "members": []}])),
        &[(DiagnosticCode::MissingField, "$.vcas[0].fader")],
    );
}

/// A deterministic xorshift for the forest generator.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound as u64) as usize
    }
    fn shuffle<T>(&mut self, items: &mut [T]) {
        for index in (1..items.len()).rev() {
            items.swap(index, self.below(index + 1));
        }
    }
}

/// The fixture with five more tracks and three submixes, so a forest has strips to group.
fn strips_model() -> (SessionModel, Vec<String>) {
    let mut model = parse_session_json(EXAMPLE).expect("fixture");
    let mut strips = vec!["vocal".to_owned()];
    for index in 0..5 {
        let mut track = model.tracks[0].clone();
        track.id = id(&format!("t{index}"));
        track.inserts.effects.clear();
        strips.push(track.id.as_str().to_owned());
        model.tracks.push(track);
    }
    model.automation.clear();
    for name in ["bus-a", "bus-b", "bus-c"] {
        model.submixes.push(Submix::unity(id(name), &model.console));
        strips.push(name.to_owned());
    }
    (model, strips)
}

/// A random acyclic VCA forest over `strips`: each VCA has a level in `0..4`, and its members are
/// strips and VCAs of strictly deeper levels, so nesting is at most four deep, VCAs overlap and
/// diamonds occur; VCAs and members are declared in shuffled order.
fn forest(rng: &mut Rng, strips: &[String]) -> Vec<Vca> {
    let count = 1 + rng.below(12);
    let levels: Vec<usize> = (0..count).map(|_| rng.below(4)).collect();
    let names: Vec<String> = (0..count)
        .map(|index| format!("vca-{:02}", (index * 37) % 101))
        .collect();
    let mut vcas: Vec<Vca> = (0..count)
        .map(|index| {
            let mut candidates: Vec<&str> = strips.iter().map(String::as_str).collect();
            candidates.extend(
                (0..count)
                    .filter(|other| levels[*other] > levels[index])
                    .map(|other| names[other].as_str()),
            );
            rng.shuffle(&mut candidates);
            let take = rng.below(candidates.len().min(16) + 1);
            let db = |rng: &mut Rng| (rng.below(337) as f32 - 288.0) * 0.5;
            Vca {
                id: id(&names[index]),
                fader: DualMonoFader {
                    left_db: db(rng),
                    right_db: db(rng),
                    left_mute: rng.below(2) == 1,
                    right_mute: rng.below(2) == 1,
                },
                members: candidates[..take].iter().map(|member| id(member)).collect(),
            }
        })
        .collect();
    rng.shuffle(&mut vcas);
    vcas
}

/// Gate 1, the canonical form: random forests round-trip exactly, with `vcas` written between
/// `submixes` and `outputs`, sorted by ID, and each `members` list sorted by ID, whatever the
/// declared order.
#[test]
fn random_vca_forests_round_trip_canonically() {
    let (base, strips) = strips_model();
    for seed in 0..32_u64 {
        let mut rng = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed + 1).wrapping_mul(0x2545_f491_4f6c_dd1d));
        let mut model = base.clone();
        model.vcas = forest(&mut rng, &strips);
        let text = canonical_session_json(&model)
            .unwrap_or_else(|error| panic!("seed {seed}: forest refused: {error:?}"));
        let parsed = parse_session_json(&text).expect("canonical text parses");
        assert_eq!(
            canonical_session_json(&parsed).expect("rewrite"),
            text,
            "seed {seed}"
        );
        assert_eq!(
            parse_session_json(&canonical_session_json(&parsed).expect("rewrite"))
                .expect("reparse"),
            parsed,
            "seed {seed}"
        );

        let mut expected = model.vcas.clone();
        expected.sort_by(|a, b| a.id.cmp(&b.id));
        for vca in &mut expected {
            vca.members.sort();
        }
        assert_eq!(
            parsed.vcas, expected,
            "seed {seed}: vcas and members are written sorted"
        );

        let root: Value = serde_json::from_str(&text).expect("JSON");
        let keys: Vec<&str> = root
            .as_object()
            .expect("root")
            .keys()
            .map(String::as_str)
            .collect();
        let at = |key: &str| text.find(&format!("\n  \"{key}\": ")).expect("root key");
        assert!(keys.contains(&"vcas"));
        assert!(
            at("submixes") < at("vcas") && at("vcas") < at("outputs"),
            "seed {seed}"
        );
    }
}

/// Gate 2: a VCA on a membership cycle is `vca.cycle` at `$.vcas[<i>]`, once per VCA on the cycle
/// in ascending declared index; a VCA off the cycle that reaches into it is not reported.
#[test]
fn vca_cycles_refuse_once_per_vca_in_index_order() {
    refused(
        &with_vcas(json!([vca_json("loop", json!(["vocal", "loop"]))])),
        &[(DiagnosticCode::VcaCycle, "$.vcas[0]")],
    );
    refused(
        &with_vcas(json!([
            vca_json("x", json!(["y"])),
            vca_json("feeder", json!(["x", "vocal"])),
            vca_json("z", json!(["x"])),
            vca_json("y", json!(["z", "vocal"])),
        ])),
        &[
            (DiagnosticCode::VcaCycle, "$.vcas[0]"),
            (DiagnosticCode::VcaCycle, "$.vcas[2]"),
            (DiagnosticCode::VcaCycle, "$.vcas[3]"),
        ],
    );
}

/// Gate 2: a diamond (one VCA reachable from another along two paths) is legal, and so is a strip
/// in several VCAs.
#[test]
fn a_diamond_and_overlapping_vcas_are_accepted() {
    let source = with_vcas(json!([
        vca_json("top", json!(["left", "right"])),
        vca_json("left", json!(["bottom", "vocal"])),
        vca_json("right", json!(["bottom", "vocal"])),
        vca_json("bottom", json!(["vocal"])),
        vca_json("idle", json!([])),
    ]));
    let model = parse_session_json(&source).expect("diamond is accepted");
    assert_eq!(model.vcas.len(), 5);
}

/// Gate 2: dangling, output, repeated and colliding references, each at its own path.
#[test]
fn vca_references_and_namespace_refuse_at_their_paths() {
    let missing = DiagnosticCode::MissingEntityReference;
    refused(
        &with_vcas(json!([vca_json("drums", json!(["vocal", "ghost"]))])),
        &[(missing, "$.vcas[0].members[1]")],
    );
    refused(
        &with_vcas(json!([vca_json("drums", json!(["main-out"]))])),
        &[(missing, "$.vcas[0].members[0]")],
    );
    refused(
        &with_vcas(json!([vca_json("drums", json!(["vocal", "vocal"]))])),
        &[(DiagnosticCode::DuplicateId, "$.vcas[0].members[1]")],
    );
    for (clash, others) in [
        ("vocal", json!([])),
        ("main-out", json!([])),
        ("drums", json!([vca_json("drums", json!([]))])),
    ] {
        let mut vcas = others.as_array().expect("array").clone();
        vcas.push(vca_json(clash, json!([])));
        let index = vcas.len() - 1;
        refused(
            &with_vcas(Value::Array(vcas)),
            &[(DiagnosticCode::DuplicateId, &format!("$.vcas[{index}].id"))],
        );
    }
}

/// Gate 2: a VCA ID never satisfies a lookup that wants a strip or an output -- a route source or
/// destination, a sidechain source or an automation target.
#[test]
fn a_vca_is_not_a_route_endpoint_sidechain_or_automation_target() {
    let missing = DiagnosticCode::MissingEntityReference;
    let vcas = json!([vca_json("drums", json!(["vocal"]))]);
    let cases: [(RootEdit, &str); 6] = [
        (
            |root| {
                root["routes"][0]["source"] =
                    json!({"kind": "track", "track_id": "drums", "tap": "post_pan"})
            },
            "$.routes[0].source.track_id",
        ),
        (
            |root| {
                root["routes"][0]["source"] =
                    json!({"kind": "submix", "submix_id": "drums", "tap": "post_pan"})
            },
            "$.routes[0].source.submix_id",
        ),
        (
            |root| {
                root["routes"][0]["destination"] =
                    json!({"kind": "output_input", "output_id": "drums"})
            },
            "$.routes[0].destination.output_id",
        ),
        (
            |root| {
                root["routes"][0]["destination"] =
                    json!({"kind": "submix_input", "submix_id": "drums"})
            },
            "$.routes[0].destination.submix_id",
        ),
        (
            |root| {
                root["tracks"][0]["inserts"]["effects"][0]["sidechain"] = json!({
                    "kind": "routed",
                    "source": {"kind": "track", "track_id": "drums", "tap": "post_fader"},
                    "port_id": "detector"
                });
            },
            "$.tracks[0].inserts.effects[0].sidechain.source.track_id",
        ),
        (
            |root| root["automation"][0]["target"]["entity_id"] = json!("drums"),
            "$.automation[0].target.entity_id",
        ),
    ];
    for (edit, path) in cases {
        let source = document(|root| {
            root.insert("vcas".to_owned(), vcas.clone());
            edit(root);
        });
        refused(&source, &[(missing, path)]);
    }
}

/// Gate 2: a VCA offset is in `[-144, 24]` dB on each lane, both ends included; outside is
/// `numeric.out_of_schema_range`, and a non-finite offset is `numeric.non_finite`.
#[test]
fn vca_offsets_are_bounded_to_the_fader_domain() {
    let with_fader = |left: f64, right: f64| {
        with_vcas(json!([{
            "id": "drums",
            "fader": {"left_db": left, "right_db": right, "left_mute": false, "right_mute": false},
            "members": ["vocal"]
        }]))
    };
    let range = DiagnosticCode::NumericOutOfSchemaRange;
    refused(
        &with_fader(24.5, 0.0),
        &[(range, "$.vcas[0].fader.left_db")],
    );
    refused(
        &with_fader(0.0, -144.5),
        &[(range, "$.vcas[0].fader.right_db")],
    );
    parse_session_json(&with_fader(24.0, -144.0)).expect("the domain's ends are accepted");
    parse_session_json(&with_fader(-144.0, 24.0)).expect("the domain's ends are accepted");

    let mut model = parse_session_json(&with_fader(0.0, 0.0)).expect("unity");
    model.vcas[0].fader.left_db = f32::NAN;
    let error = canonical_session_json(&model).expect_err("NaN refuses");
    assert_eq!(
        diagnostics(&error),
        [(
            DiagnosticCode::NumericNonFinite,
            "$.vcas[0].fader.left_db".to_owned()
        )]
    );
}
