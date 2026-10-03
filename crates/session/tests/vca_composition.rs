//! The VCA composition (#1242 D1): `vca_effective_db`, `SessionModel::vca_reach` and
//! `SessionModel::effective_strip_faders`, the one place preparation computes a VCA's effect.

use session::{
    DualMonoFader, EffectiveStripFader, SessionModel, StableId, Submix, Vca, parse_session_json,
    vca_effective_db,
};

const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("valid test ID")
}

/// A deterministic xorshift.
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
    /// A full-precision `f32` in `[low, high]`.
    fn in_range(&mut self, low: f32, high: f32) -> f32 {
        let unit = (self.next() >> 40) as f32 / (1_u64 << 24) as f32;
        (low + (high - low) * unit).clamp(low, high)
    }
}

/// The composition's own statement, evaluated independently: an `f64` sum in the given order, then
/// the domain clamp, then one rounding.
fn reference_db(member: f32, offsets: &[f32]) -> f32 {
    if offsets.is_empty() {
        return member;
    }
    let total = offsets
        .iter()
        .fold(f64::from(member), |sum, offset| sum + f64::from(*offset));
    total.clamp(-144.0, 24.0) as f32
}

/// Gate 1, the gain: random sums equal the `f64` reference bit for bit; no offset is the member's
/// own value bit for bit (`-0.0` and an out-of-domain value included); the sum clamps at both
/// edges; and the crafted order case is `(24 - 24) + 1e-30`, never `(24 + 1e-30) - 24 = 0`.
///
/// Red if the sum accumulates in `f32` (the random draws round differently), runs in another order
/// (the crafted case), clamps a member with no VCA, or does not clamp.
#[test]
fn vca_effective_db_sums_in_f64_in_order_and_clamps_once() {
    let mut rng = Rng(0x2545_f491_4f6c_dd1d);
    for _ in 0..20_000 {
        let member = rng.in_range(-144.0, 24.0);
        let count = rng.below(6);
        let offsets: Vec<f32> = (0..count).map(|_| rng.in_range(-144.0, 24.0)).collect();
        let actual = vca_effective_db(member, offsets.iter().copied());
        let expected = reference_db(member, &offsets);
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "{member} + {offsets:?}: {actual} != {expected}"
        );
    }
    for member in [-0.0_f32, 0.0, 24.5, -144.5, -6.25, f32::MIN_POSITIVE] {
        assert_eq!(
            vca_effective_db(member, []).to_bits(),
            member.to_bits(),
            "{member} with no offset"
        );
    }
    assert_eq!(vca_effective_db(20.0, [3.0, 2.0]), 24.0);
    assert_eq!(vca_effective_db(-140.0, [-6.0]), -144.0);
    assert_eq!(vca_effective_db(-144.0, [-144.0, -144.0]), -144.0);
    assert_eq!(vca_effective_db(24.0, [24.0]), 24.0);
    // An in-domain sum is not clamped: 24 + 6 - 6.
    assert_eq!(vca_effective_db(24.0, [6.0, -6.0]), 24.0);
    assert_eq!(vca_effective_db(-6.0, [6.0]).to_bits(), 0.0_f32.to_bits());
    assert_eq!(vca_effective_db(24.0, [-24.0, 1e-30]), 1e-30);
    assert_eq!(vca_effective_db(24.0, [1e-30, -24.0]), 0.0);
}

/// The fixture's `vocal`, two copies `t0` and `t1`, and submix `bus-a`: strips 0 to 3.
fn strips_model() -> SessionModel {
    let mut model = parse_session_json(EXAMPLE).expect("fixture");
    for name in ["t0", "t1"] {
        let mut track = model.tracks[0].clone();
        track.id = id(name);
        model.tracks.push(track);
    }
    model
        .submixes
        .push(Submix::unity(id("bus-a"), &model.console));
    let ids: Vec<&str> = model.strips().map(|strip| strip.id.as_str()).collect();
    assert_eq!(ids, ["vocal", "t0", "t1", "bus-a"]);
    model
}

fn vca(name: &str, offsets: [f32; 2], mutes: [bool; 2], members: &[&str]) -> Vca {
    Vca {
        id: id(name),
        fader: DualMonoFader {
            left_db: offsets[0],
            right_db: offsets[1],
            left_mute: mutes[0],
            right_mute: mutes[1],
        },
        members: members.iter().map(|member| id(member)).collect(),
    }
}

/// The reach as VCA IDs, per strip.
fn reach_ids(model: &SessionModel) -> Vec<Vec<&str>> {
    model
        .vca_reach()
        .into_iter()
        .map(|reach| {
            reach
                .into_iter()
                .map(|index| model.vcas[index].id.as_str())
                .collect()
        })
        .collect()
}

/// Gate 1, the reach. Declared out of ID order: `z-top` holds `a-mid` and `b-mid`, which both hold
/// `t0` (a diamond), `b-mid` also holds `bus-a`, and `m-leaf` holds `vocal`; `t1` is in no VCA.
/// Each strip's reach counts `z-top` once, includes the nested parent, is sorted by VCA ID, and
/// the effective faders sum each reaching offset once and OR each reaching mute, the strip's own
/// mute kept.
///
/// Red if reach double-counts a diamond (`t0` would sum `z-top` twice), misses a nested parent,
/// is left in declaration order, or reaches tracks only.
#[test]
fn vca_reach_counts_a_diamond_once_and_includes_nested_parents_in_id_order() {
    let mut model = strips_model();
    model.tracks[1].fader = DualMonoFader {
        left_db: 1.0,
        right_db: -2.5,
        left_mute: false,
        right_mute: true,
    };
    model.vcas = vec![
        vca("z-top", [-4.0, 0.5], [false, false], &["b-mid", "a-mid"]),
        vca("m-leaf", [-1.0, -1.0], [true, false], &["vocal"]),
        vca("b-mid", [-3.0, 0.25], [false, true], &["t0", "bus-a"]),
        vca("a-mid", [-2.0, 0.125], [false, false], &["t0"]),
    ];
    assert_eq!(
        reach_ids(&model),
        vec![
            vec!["m-leaf"],
            vec!["a-mid", "b-mid", "z-top"],
            vec![],
            vec!["b-mid", "z-top"],
        ]
    );
    let vocal = &model.tracks[0].fader;
    let bus = &model.submixes[0].fader;
    let expected = vec![
        EffectiveStripFader {
            db: [
                vca_effective_db(vocal.left_db, [-1.0]),
                vca_effective_db(vocal.right_db, [-1.0]),
            ],
            mute: [true, vocal.right_mute],
            vca_mute: [true, false],
        },
        EffectiveStripFader {
            db: [1.0 - 2.0 - 3.0 - 4.0, -2.5 + 0.125 + 0.25 + 0.5],
            mute: [false, true],
            vca_mute: [false, true],
        },
        EffectiveStripFader {
            db: [
                model.tracks[2].fader.left_db,
                model.tracks[2].fader.right_db,
            ],
            mute: [
                model.tracks[2].fader.left_mute,
                model.tracks[2].fader.right_mute,
            ],
            vca_mute: [false, false],
        },
        EffectiveStripFader {
            db: [bus.left_db - 3.0 - 4.0, bus.right_db + 0.25 + 0.5],
            mute: [bus.left_mute, true],
            vca_mute: [false, true],
        },
    ];
    assert_eq!(model.effective_strip_faders(), expected);
}

/// Gate 1, termination. An unvalidated model whose VCAs form a two-VCA cycle (`c1` holds `c2` and
/// `t0`, `c2` holds `c1`) and a self-member (`c3` holds itself and `t1`) still yields a reach,
/// each VCA once.
///
/// Red if the reach walk has no visited set (it never returns) or records a VCA once per visit.
#[test]
fn vca_reach_terminates_on_a_cyclic_model() {
    let mut model = strips_model();
    model.vcas = vec![
        vca("c2", [0.0; 2], [false; 2], &["c1"]),
        vca("c1", [0.0; 2], [false; 2], &["c2", "t0"]),
        vca("c3", [0.0; 2], [false; 2], &["c3", "t1"]),
    ];
    assert_eq!(
        reach_ids(&model),
        vec![vec![], vec!["c1", "c2"], vec!["c3"], vec![]]
    );
}
