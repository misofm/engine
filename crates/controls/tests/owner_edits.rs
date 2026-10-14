//! #1613: a transaction of `SetControl`s applied to clones of the owners it names, and the model
//! read as the committed model plus those owners.
//!
//! Red if `OwnerEdits` writes anything the whole-model `set` does not (or refuses where it does not),
//! if a `ModelView` of the edited owners reads other than the dense edited model, if
//! `LocalEffective` gives other effective values than `effective`, or if the rows of an owner read
//! through the view differ from the dense `owner_rows_with`.

mod common;

use common::{RATE, effects_fixture, id};
use controls::{
    ControlAddress, ControlClass, ControlKind, ControlRow, Effective, EffectiveSource,
    LocalEffective, ModelView, MonitorOverlay, Motion, OwnerEdits, derived_rows_in,
    derived_rows_with, effective, list, owner_count, owner_rows_in, owner_rows_with, set,
};
use session::{DualMonoFader, SessionModel, Vca};

/// The effects fixture with a third track, a submix send that follows its source's mute, and nested
/// VCAs, every table in ID order (a compiled session's normalized order).
fn model() -> SessionModel {
    let mut model = effects_fixture();
    let mut bass = model.tracks[0].clone();
    bass.id = id("bass");
    model.tracks.push(bass);
    model.tracks.sort_by(|left, right| left.id.cmp(&right.id));
    model.vcas.push(Vca {
        id: id("master"),
        fader: DualMonoFader {
            left_db: -2.0,
            right_db: 1.5,
            left_mute: false,
            right_mute: true,
        },
        members: vec![id("riders"), id("bass"), id("bus")],
    });
    model.vcas.sort_by(|left, right| left.id.cmp(&right.id));
    model.routes.sort_by(|left, right| left.id.cmp(&right.id));
    model
}

struct Draw(u64);

impl Draw {
    fn next(&mut self, bound: usize) -> usize {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        (self.0.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 33) as usize % bound
    }

    fn unit(&mut self) -> f32 {
        self.next(1 << 20) as f32 / (1 << 20) as f32
    }
}

fn value_for(draw: &mut Draw, row: &ControlRow) -> f32 {
    let d = &row.descriptor;
    match d.kind {
        ControlKind::Toggle => draw.next(2) as f32,
        ControlKind::Enumeration => d.choices[draw.next(d.choices.len())].value,
        ControlKind::Continuous => {
            let value = d.minimum + (d.maximum - d.minimum) * draw.unit();
            if d.unit == effect_contract::ParameterUnit::Samples {
                value.floor()
            } else {
                value
            }
        }
    }
}

/// Rows the in-place preparation takes: live and not derived, active, of a strip, route or VCA.
fn editable(model: &SessionModel) -> Vec<ControlRow> {
    list(model, &MonitorOverlay::default(), RATE)
        .filter(|row| {
            row.descriptor.class == ControlClass::Live
                && row.descriptor.active
                && row.descriptor.scope == controls::ControlScope::Model
                && controls::writes_only_its_row(&row.address)
                && !matches!(row.address.owner, controls::Owner::Session)
        })
        .collect()
}

/// A run of seeded edits through `OwnerEdits` and through `set` on a whole model: the same refusals,
/// and the view of the edited owners is the whole model's, owner by owner.
#[test]
fn edited_owners_read_as_the_whole_edited_model() {
    let committed = model();
    let rows = editable(&committed);
    assert!(rows.len() > 100, "{}", rows.len());
    let mut draw = Draw(0x5EED_0010);
    for round in 0..300 {
        let mut whole = committed.clone();
        let mut edits = OwnerEdits::new(&committed);
        for _ in 0..1 + draw.next(5) {
            let row = &rows[draw.next(rows.len())];
            let value = value_for(&mut draw, row);
            let ramp = (matches!(row.descriptor.motion, Motion::Ramp(_)) && draw.next(3) == 0)
                .then_some(64);
            let expected = set(&mut whole, &row.address, value, ramp, RATE);
            let actual = edits.set(&row.address, value, ramp, RATE);
            assert_eq!(
                expected.map_err(controls::OwnerEditError::Control),
                actual,
                "round {round}: {:?} = {value}",
                row.address
            );
        }
        let (owners, slots) = edits.finish();
        let view = ModelView::edited(&committed, &owners);
        assert_eq!(
            view.to_model(),
            whole,
            "round {round}: the view is the edited model"
        );
        // Owner by owner, as the classifier and the catalog stage read them.
        for index in 0..committed.tracks.len() + committed.submixes.len() {
            assert_eq!(
                view.strip(index),
                whole_strip(&whole, index),
                "round {round}"
            );
        }
        for index in 0..whole.routes.len() {
            assert_eq!(view.route(index), &whole.routes[index], "round {round}");
        }
        for index in 0..whole.vcas.len() {
            assert_eq!(view.vca(index), &whole.vcas[index], "round {round}");
        }
        // Every slot names the owner in the committed model.
        for &(at, home) in &slots.tracks {
            assert_eq!(owners.tracks[at].id, committed.tracks[home].id);
        }
        for &(at, home) in &slots.routes {
            assert_eq!(owners.routes[at].id, committed.routes[home].id);
        }
        for &(at, home) in &slots.vcas {
            assert_eq!(owners.vcas[at].id, committed.vcas[home].id);
        }
        // The effective values through the view are the dense pass's, under random overlays.
        let overlay = MonitorOverlay::with_soloed(
            committed
                .tracks
                .iter()
                .filter(|_| draw.next(4) == 0)
                .map(|track| track.id.clone()),
        );
        let dense = effective(&whole, &overlay);
        let local = LocalEffective::new(view, &overlay);
        for strip in 0..dense.strips.len() {
            assert_eq!(
                local.strip(strip),
                dense.strips[strip],
                "round {round} strip {strip}"
            );
        }
        for route in 0..dense.routes.len() {
            assert_eq!(
                local.route(route),
                dense.routes[route],
                "round {round} route {route}"
            );
        }
        // The rows of every owner, and its derived rows, read through the view and the local
        // values, are the dense ones.
        let mut dense_rows = Vec::new();
        let mut view_rows = Vec::new();
        for owner in 0..owner_count(&whole) {
            dense_rows.clear();
            view_rows.clear();
            owner_rows_with(&whole, &dense, &overlay, owner, RATE, &mut dense_rows);
            owner_rows_in(view, &local, &overlay, owner, RATE, &mut view_rows);
            assert_eq!(view_rows, dense_rows, "round {round} owner {owner}");
            dense_rows.clear();
            view_rows.clear();
            derived_rows_with(&whole, &dense, owner, &mut dense_rows);
            derived_rows_in(view, &local, owner, &mut view_rows);
            assert_eq!(view_rows, dense_rows, "round {round} derived owner {owner}");
        }
    }
}

fn whole_strip(model: &SessionModel, index: usize) -> session::StripRef<'_> {
    model.strips().nth(index).expect("a strip")
}

/// The dense `Effective` is an `EffectiveSource` with the same answers it holds.
#[test]
fn the_dense_effective_values_are_a_source() {
    let model = model();
    let dense: Effective = effective(&model, &MonitorOverlay::default());
    for strip in 0..dense.strips.len() {
        assert_eq!(EffectiveSource::strip(&dense, strip), dense.strips[strip]);
    }
    for route in 0..dense.routes.len() {
        assert_eq!(
            EffectiveSource::route_gate(&dense, route),
            dense.routes[route].gate
        );
    }
}

/// A session row and an unknown owner are not owners an in-place edit can hold, and nothing is
/// written by a refusal.
#[test]
fn owners_an_edit_cannot_hold_are_refused() {
    let committed = model();
    let mut edits = OwnerEdits::new(&committed);
    let session_row = ControlAddress {
        owner: controls::Owner::Session,
        section: controls::Section::Own,
        control: 1,
        lane: controls::Lane::Both,
    };
    assert_eq!(
        edits.set(&session_row, 40.0, None, RATE),
        Err(controls::OwnerEditError::Session)
    );
    let nobody = ControlAddress {
        owner: controls::Owner::Strip(id("nobody")),
        section: controls::Section::Builtins,
        control: 5,
        lane: controls::Lane::Both,
    };
    assert!(edits.set(&nobody, -3.0, None, RATE).is_err());
    assert!(edits.owners().tracks.is_empty());
    assert!(edits.owners().routes.is_empty());
}
