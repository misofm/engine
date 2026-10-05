//! #1257 gate 5: `validate_live_peak` charges each term of #1053 D8 against the right plan.

use super::*;
use crate::runtime::tests::{SESSION, limits};

/// Every term gets its own bit, so a missing or swapped term moves every sum it is part of.
struct Terms {
    graph: u64,
    capi_retained: u64,
    largest_named: u64,
    epoch_retained: u64,
    prepared_protocol: u64,
    capi_largest: u64,
}

fn epoch(base: PlanResourceReport, terms: &Terms) -> LiveEpochResources {
    LiveEpochResources {
        report: PlanResourceReport {
            graph_session_plus_plan_bytes: terms.graph,
            capi_retained_bytes: terms.capi_retained,
            largest_named_allocation_bytes: terms.largest_named,
            ..base
        }
        .into(),
        capi: CapiResources {
            active_retained: u64::MAX,
            epoch_retained: terms.epoch_retained,
            prepared_protocol_retained: terms.prepared_protocol,
            largest: terms.capi_largest,
        },
    }
}

fn caps(graph: u64, capi: u64, largest: u64) -> CompileLimits {
    CompileLimits {
        maximum_graph_session_plus_plan_bytes: graph,
        maximum_capi_retained_bytes: capi,
        maximum_named_allocation_bytes: largest,
        ..limits()
    }
}

fn verdict(
    current: LiveEpochResources,
    pending: Option<LiveEpochResources>,
    models: CompiledModelAdmission,
    limits: CompileLimits,
) -> Result<(), String> {
    validate_live_peak(current, pending, models, limits.into())
        .map_err(|failure| String::from_utf8(failure_bytes(failure)).expect("UTF-8"))
}

/// Accepts with `cap` at `peak` and refuses with it one byte below, with `code`.
fn assert_cap(label: &str, peak: u64, code: &str, check: impl Fn(u64) -> Result<(), String>) {
    assert_eq!(check(peak), Ok(()), "{label}: accepted at the cap");
    assert_eq!(
        check(peak - 1),
        Err(format!("{code}\t$\n")),
        "{label}: refused one byte below"
    );
}

#[test]
fn the_live_admission_accepts_each_cap_and_refuses_one_byte_below() {
    let base = compile_children(SESSION, limits())
        .unwrap_or_else(|_| panic!("fixture compiles"))
        .plan
        .resources();
    let current_terms = Terms {
        graph: 1 << 0,
        capi_retained: 1 << 1,
        largest_named: 3,
        epoch_retained: 1 << 2,
        prepared_protocol: 1 << 3,
        capi_largest: 5,
    };
    let pending_terms = Terms {
        graph: 1 << 4,
        capi_retained: 1 << 5,
        largest_named: 7,
        epoch_retained: 1 << 6,
        prepared_protocol: 1 << 7,
        capi_largest: 9,
    };
    let models = CompiledModelAdmission {
        retained_bytes: 1 << 8,
        largest_allocation_bytes: 11,
    };
    let current = epoch(base, &current_terms);
    let pending = epoch(base, &pending_terms);
    let huge = u64::MAX;

    // Graph: both plans plus both compiled models.
    for (label, pending, peak) in [
        ("graph alone", None, 1 + (1 << 8)),
        ("graph pending", Some(pending), 1 + (1 << 4) + (1 << 8)),
    ] {
        assert_cap(label, peak, "graph.resource.limit", |cap| {
            verdict(current, pending, models, caps(cap, huge, huge))
        });
    }
    // capi: the newest row, the current epoch's own rows while a candidate waits, and the
    // newest epoch's prepared-protocol rows.
    for (label, pending, peak) in [
        ("capi alone", None, (1 << 1) + (1 << 3)),
        (
            "capi pending",
            Some(pending),
            (1 << 5) + (1 << 2) + (1 << 7),
        ),
    ] {
        assert_cap(label, peak, "capi.resource.limit", |cap| {
            verdict(current, pending, models, caps(huge, cap, huge))
        });
    }

    // Largest allocation: make each term in turn the strict maximum. The current epoch's capi
    // `largest` is a decoy while a candidate is pending: only the newest epoch's counts.
    let big = 1_000;
    for term in 0..4 {
        let mut current_terms = Terms { ..current_terms };
        let mut pending_terms = Terms { ..pending_terms };
        let mut models = models;
        match term {
            0 => current_terms.largest_named = big,
            1 => pending_terms.largest_named = big,
            2 => pending_terms.capi_largest = big,
            _ => models.largest_allocation_bytes = big,
        }
        current_terms.capi_largest = big * 2;
        let current = epoch(base, &current_terms);
        let pending = epoch(base, &pending_terms);
        assert_cap(
            &format!("largest pending, term {term}"),
            big,
            "capi.resource.limit",
            |cap| verdict(current, Some(pending), models, caps(huge, huge, cap)),
        );
    }
    for term in 0..3 {
        let mut current_terms = Terms { ..current_terms };
        let mut models = models;
        match term {
            0 => current_terms.largest_named = big,
            1 => current_terms.capi_largest = big,
            _ => models.largest_allocation_bytes = big,
        }
        let current = epoch(base, &current_terms);
        assert_cap(
            &format!("largest alone, term {term}"),
            big,
            "capi.resource.limit",
            |cap| verdict(current, None, models, caps(huge, huge, cap)),
        );
    }
}
