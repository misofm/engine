//! `vca_reach_of` and `strips_reached_by` are the one-strip and the inverse forms of `vca_reach`
//! (#1612 D7): a host that classifies one VCA edit asks for the strips that VCA reaches, and for the
//! reach of each of them, without building the table of every strip. Red if either drifts from
//! `vca_reach` on a nested, shared or cyclic membership.

use session::{DualMonoFader, SessionModel, StableId, Submix, Track, Vca, parse_session_json};

const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("valid test ID")
}

/// A small deterministic generator (xorshift64*).
struct Draw(u64);

impl Draw {
    fn next(&mut self, bound: usize) -> usize {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        (self.0.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 33) as usize % bound
    }
}

fn model(draw: &mut Draw) -> SessionModel {
    let mut model = parse_session_json(EXAMPLE).expect("fixture parses");
    let template: Track = model.tracks[0].clone();
    model.tracks.clear();
    model.routes.clear();
    for index in 0..1 + draw.next(7) {
        let mut track = template.clone();
        track.id = id(&format!("t{index:02}"));
        model.tracks.push(track);
    }
    for index in 0..draw.next(3) {
        let submix = Submix::unity(id(&format!("u{index:02}")), &model.console);
        model.submixes.push(submix);
    }
    let vca_count = draw.next(5);
    let names: Vec<String> = (0..vca_count).map(|index| format!("v{index:02}")).collect();
    let pool: Vec<StableId> = model
        .strips()
        .map(|strip| strip.id.clone())
        .chain(names.iter().map(|name| id(name)))
        .collect();
    for name in &names {
        let members = (0..draw.next(5))
            .map(|_| pool[draw.next(pool.len())].clone())
            .collect();
        model.vcas.push(Vca {
            id: id(name),
            fader: DualMonoFader {
                left_db: 0.0,
                right_db: 0.0,
                left_mute: false,
                right_mute: false,
            },
            members,
        });
    }
    model
}

#[test]
fn the_one_strip_and_the_inverse_forms_equal_the_table() {
    let mut draw = Draw(0x9E37_79B9_7F4A_7C15);
    let mut nested_seen = false;
    for _ in 0..500 {
        let model = model(&mut draw);
        let table = model.vca_reach();
        let strips: Vec<StableId> = model.strips().map(|strip| strip.id.clone()).collect();
        for (index, strip) in strips.iter().enumerate() {
            assert_eq!(model.vca_reach_of(strip), table[index], "reach of {strip}");
            nested_seen |= table[index].len() > 1;
        }
        for vca in 0..model.vcas.len() {
            let inverse: Vec<usize> = (0..strips.len())
                .filter(|&strip| table[strip].contains(&vca))
                .collect();
            assert_eq!(model.strips_reached_by(vca), inverse, "strips of VCA {vca}");
        }
        assert!(model.strips_reached_by(model.vcas.len()).is_empty());
        // Batch 7 review M1: the pair count is the table's size, and stops past its limit.
        let pairs: u64 = table.iter().map(|reach| reach.len() as u64).sum();
        assert_eq!(model.vca_reach_pairs(u64::MAX), pairs);
        assert_eq!(model.vca_reach_pairs(pairs), pairs);
        if pairs > 0 {
            assert!(model.vca_reach_pairs(pairs - 1) > pairs - 1);
        }
    }
    assert!(nested_seen, "the generator reached a strip under two VCAs");
}
