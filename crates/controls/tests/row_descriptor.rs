//! `row_descriptor` is the listing's descriptor without a model (decision 17, slice S7c).

mod common;

use common::{RATE, fixture};
use controls::{
    ControlDescriptor, MonitorOverlay, Owner, OwnerKind, Section, SectionKind, list, row_descriptor,
};

/// The listing's descriptor of a row that the shape-independent table answers, with `active` read
/// from the table (a host that holds no model checks the strip's shape itself).
fn expected(descriptor: ControlDescriptor) -> ControlDescriptor {
    ControlDescriptor {
        active: true,
        ..descriptor
    }
}

/// Every row the browser admits, and every other own or builtin row, resolves to the descriptor
/// that `list` gives the same row of a session at the same rate: a table that drifted from the
/// listing (a second spelling of a domain) turns this red.
#[test]
fn a_model_free_descriptor_equals_the_listed_one() {
    let model = fixture();
    let mut checked = 0;
    for row in list(&model, &MonitorOverlay::default(), RATE) {
        let address = &row.address;
        // The send into the bus is the route the browser admits; the output routes have the fold
        // class, which `row_descriptor` does not answer for.
        if matches!(&address.owner, Owner::Route(id) if id.as_str() != "send") {
            continue;
        }
        // The submix `bus` lists the same rows as a track, and `vocal` is a track in pan mode.
        if matches!(&address.owner, Owner::Strip(id) if id.as_str() != "vocal") {
            continue;
        }
        let found = row_descriptor(
            address.owner.kind(),
            address.section.kind(),
            address.control,
            RATE,
        );
        assert_eq!(
            found.map(expected),
            Some(expected(row.descriptor)),
            "{address:?}"
        );
        checked += 1;
    }
    assert!(checked > 40, "the walk reached {checked} rows");
}

/// The effect sections and an unknown ID have no model-free descriptor.
#[test]
fn effect_sections_and_unknown_ids_have_no_descriptor() {
    for section in [SectionKind::Console, SectionKind::Inserts] {
        assert_eq!(row_descriptor(OwnerKind::Strip, section, 1, RATE), None);
    }
    assert_eq!(
        row_descriptor(OwnerKind::Strip, SectionKind::Builtins, 0, RATE),
        None
    );
    assert_eq!(
        row_descriptor(OwnerKind::Vca, SectionKind::Builtins, 1, RATE),
        None
    );
    assert_eq!(
        row_descriptor(OwnerKind::Route, SectionKind::Own, 1_000, RATE),
        None
    );
    assert_eq!(Section::Own.kind(), SectionKind::Own);
}

/// A rate-keyed row is resolved at the rate given.
#[test]
fn a_rate_keyed_row_follows_the_rate() {
    let at = |rate| {
        row_descriptor(OwnerKind::Strip, SectionKind::Builtins, 3, rate)
            .expect("hpf_hz")
            .maximum
    };
    assert!(at(96_000) > at(44_100));
}
