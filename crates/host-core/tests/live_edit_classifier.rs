//! #1612: the fast live-edit classifier.
//!
//! `classify_live_edit` reads a transaction of `SetControl`s from the rows it names, with no model
//! clone and no canonical JSON, and must give exactly what `classify_live_delta_with_monitor` gives
//! for the same models. This file holds the three gates:
//!
//! 1. the differential: the #1560 parity script and a seeded random set of edits, the full
//!    classifier as the oracle;
//! 2. the cost: the allocations of a live-only classification do not depend on the track count;
//! 3. one descriptive timing line (ignored: `cargo test --release -p host-core --all-features
//!    --test live_edit_classifier -- --ignored --nocapture`).
#![cfg(all(feature = "control-provider", feature = "test-support"))]

use std::collections::BTreeSet;
use std::time::Instant;

#[path = "support/edit_sessions.rs"]
mod edit_sessions;
use edit_sessions::*;

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use controls::{Lane, MonitorOverlay, Owner, Section};
use host_core::{
    LiveRamps, LiveRebuild, MonitorBase, classify_live_delta_with_monitor, classify_live_edit,
    take_fast_classification_count,
};
use protocol::{ExpectedRevision, PreparedSessionTransaction, SessionEdit, SessionStore};
/// `edits` prepared on `store` as the control plane prepares them, with the overlay and the ramp
/// the transaction leaves after `committed`. `None` when the protocol refuses the transaction.
fn prepare(
    store: &SessionStore,
    edits: &[SessionEdit],
    committed: &MonitorOverlay,
) -> Option<(PreparedSessionTransaction, MonitorOverlay, Option<u32>)> {
    let prepared = store
        .prepare_transaction_with_monitor(ExpectedRevision::Exact(store.revision()), edits)
        .ok()?;
    let change = committed.after(
        prepared.monitor_edits(),
        prepared
            .compiled()
            .expect("a compiled transaction")
            .normalized_model(),
    );
    Some((prepared, change.overlay, change.ramp))
}

/// What a prepared transaction classified to: whether the fast path answered, and the verdict.
struct Verdict {
    fast_path: bool,
    result: Result<usize, LiveRebuild>,
}

/// Classify a prepared transaction with the full classifier and with `classify_live_edit`, and
/// assert that they agree on the whole delta or the same refusal.
fn judge(
    store: &SessionStore,
    prepared: &PreparedSessionTransaction,
    committed: &MonitorOverlay,
    overlay: &MonitorOverlay,
    ramp: Option<u32>,
) -> Verdict {
    let current = store.compiled().normalized_model();
    let next = prepared
        .compiled()
        .expect("a compiled transaction")
        .normalized_model();
    let monitor = MonitorBase {
        committed,
        next: overlay,
        ramp,
    };
    let ramps = LiveRamps::for_session(next);
    let full =
        classify_live_delta_with_monitor(current, next, ramps, prepared.edit_ramps(), monitor);
    let _ = take_fast_classification_count();
    let fast = classify_live_edit(
        current,
        next,
        ramps,
        prepared.edit_ramps(),
        monitor,
        prepared.set_control_rows(),
    );
    let fast_path = take_fast_classification_count() == 1;
    // The fast path says which derived owners it reached; the full one reads every owner.
    let fast = fast.map(|mut delta| {
        assert_eq!(delta.derived_owners.is_some(), fast_path);
        delta.derived_owners = None;
        delta
    });
    assert_eq!(fast, full, "the fast classifier equals the full one");
    Verdict {
        fast_path,
        result: full.map(|delta| delta.strips.len() + delta.effects.len() + delta.routes.len()),
    }
}

// ---------------------------------------------------------------------------------------------
// Gate 1a: the #1560 parity script
// ---------------------------------------------------------------------------------------------

/// Gate 1a. The parity script's edits, each classified both ways on the session it is played on:
/// every edit alone from the start, and the whole script in order on a model that advances, with
/// solos in between, at two rates.
///
/// Red if the fast classifier differs from the full one on any scripted edit, or the rows it exists
/// for do not take the fast path.
#[test]
fn the_parity_script_classifies_as_the_full_classifier_does() {
    assert_installed();
    for rate in [48_000, 96_000] {
        let base = store(parity_session(rate));
        let none = MonitorOverlay::default();
        let mut fast_taken = 0;
        let mut accepted = 0;
        for step in scripted() {
            let Some((prepared, overlay, ramp)) = prepare(&base, &step.edits, &none) else {
                assert!(step.refused, "an accepted edit is accepted alone");
                continue;
            };
            assert!(!step.refused);
            let verdict = judge(&base, &prepared, &none, &overlay, ramp);
            accepted += 1;
            fast_taken += usize::from(verdict.fast_path);
        }
        assert!(fast_taken > 30, "the rows took the fast path: {fast_taken}");
        assert!(accepted > 40, "{accepted}");

        let mut advancing = store(parity_session(rate));
        let mut committed = MonitorOverlay::default();
        let solo = |strip: &str, on: bool| vec![set(own_row(strip, SOLO), f32::from(u8::from(on)))];
        let mut script = scripted();
        for (at, edits) in [
            (20, solo("t2", true)),
            (25, solo("t1", true)),
            (30, solo("t2", false)),
            (38, solo("t1", false)),
        ] {
            script.insert(
                at,
                Scripted {
                    edits,
                    refused: false,
                },
            );
        }
        let mut sequence_fast = 0;
        for step in script {
            let Some((prepared, overlay, ramp)) = prepare(&advancing, &step.edits, &committed)
            else {
                assert!(step.refused);
                continue;
            };
            let verdict = judge(&advancing, &prepared, &committed, &overlay, ramp);
            sequence_fast += usize::from(verdict.fast_path);
            drop(prepared);
            if !step.edits.iter().all(is_solo) {
                let revision = advancing.revision();
                advancing
                    .apply_transaction(ExpectedRevision::Exact(revision), &step.edits)
                    .expect("the script's edit applies");
            }
            committed = overlay;
        }
        assert!(sequence_fast > 30, "{sequence_fast}");
    }
}

// ---------------------------------------------------------------------------------------------
// Gate 1b: a seeded random set of edits
// ---------------------------------------------------------------------------------------------

/// Gate 1b. At least 2,000 seeded transactions of one to four edits, each drawn from the control
/// list of the session (every row of every class, the session's rows, a strip's matrix mode, an
/// insert's link mode, the solo rows), with and without a per-edit ramp, over a random committed
/// overlay, sometimes with a structural edit among them: the fast classifier equals the full one.
///
/// Red if it differs anywhere, and if the generator did not reach the rows the fast path exists for
/// through the fast path and the rows that must fall back through the full one.
#[test]
fn a_random_transaction_classifies_as_the_full_classifier_does() {
    assert_installed();
    let sessions = [store(rich_session(10)), store(plain_session(4))];
    let mut draw = Draw(0x9E37_79B9_7F4A_7C15);
    let mut cases = 0_usize;
    let mut fast_rows: BTreeSet<&'static str> = BTreeSet::new();
    let mut full_rows: BTreeSet<&'static str> = BTreeSet::new();
    let mut verdicts: BTreeSet<&'static str> = BTreeSet::new();
    let mut attempts = 0;
    while cases < 2_400 {
        attempts += 1;
        assert!(attempts < 40_000, "the generator keeps being refused");
        let store = &sessions[usize::from(cases % 6 == 5)];
        let Case {
            committed,
            edits,
            names,
        } = draw_case(&mut draw, store.compiled().normalized_model());
        let Some((prepared, overlay, ramp)) = prepare(store, &edits, &committed) else {
            continue;
        };
        let verdict = judge(store, &prepared, &committed, &overlay, ramp);
        cases += 1;
        let target = if verdict.fast_path {
            &mut fast_rows
        } else {
            &mut full_rows
        };
        target.extend(names);
        verdicts.insert(match verdict.result {
            Ok(0) => "live, no record",
            Ok(_) => "live",
            Err(LiveRebuild::RebuildOnly(_)) => "rebuild-only",
            Err(LiveRebuild::Structure) => "structure",
            Err(LiveRebuild::Domain) => "domain",
            Err(LiveRebuild::AutomationTarget) => "automation",
        });
    }
    for row in [
        "fader_db",
        "mute",
        "pan",
        "matrix_ll",
        "trim_db",
        "polarity_invert",
        "hpf_hz",
        "lpf_hz",
        "bypass",
        "gain_db",
        "offset_db",
        "solo",
        "follows_mute",
        "pan_window_samples",
    ] {
        assert!(fast_rows.contains(row), "no fast classification of {row}");
    }
    for row in ["matrix_mode", "fader_ms", "link_mode", "<structural>"] {
        assert!(full_rows.contains(row), "no full classification of {row}");
    }
    for verdict in ["live", "rebuild-only", "live, no record"] {
        assert!(
            verdicts.contains(verdict),
            "no {verdict} verdict: {verdicts:?}"
        );
    }
}

// ---------------------------------------------------------------------------------------------
// Gate 2: the cost does not grow with the session
// ---------------------------------------------------------------------------------------------

/// The allocations and frees of one classification of `edits` by `classify_live_edit` on `store`,
/// after a warm-up call, and whether the fast path answered.
fn allocations(
    store: &SessionStore,
    edits: &[SessionEdit],
    committed: &MonitorOverlay,
) -> (u64, u64, bool) {
    let (prepared, overlay, ramp) = prepare(store, edits, committed).expect("the edit applies");
    let current = store.compiled().normalized_model();
    let next = prepared
        .compiled()
        .expect("a compiled transaction")
        .normalized_model();
    let monitor = MonitorBase {
        committed,
        next: &overlay,
        ramp,
    };
    let run = || {
        let delta = classify_live_edit(
            current,
            next,
            LiveRamps::for_session(next),
            prepared.edit_ramps(),
            monitor,
            prepared.set_control_rows(),
        )
        .expect("live");
        assert!(!delta.is_empty(), "the edit changes a render value");
        drop(delta);
    };
    run();
    let _ = take_fast_classification_count();
    let mark = current_thread_counters();
    run();
    let spent = current_thread_delta_since(mark);
    (
        spent.allocations,
        spent.deallocations,
        take_fast_classification_count() == 1,
    )
}

/// Gate 2. One live-only transaction of each kind costs the same allocations and frees at 8, 64 and
/// 256 tracks, and the fast path answers.
///
/// Red if the classification clones, lists or walks the session: one allocation per strip, per
/// route or per VCA of the session moves the count with the track count.
#[test]
fn the_cost_of_a_live_edit_does_not_grow_with_the_session() {
    assert_installed();
    let sizes = [8, 64, 256];
    let stores: Vec<SessionStore> = sizes.iter().map(|&n| store(rich_session(n))).collect();
    let both_lanes = |strip: &str, control: u32| strip_row(strip, control, Lane::Both);
    let eq = |parameter: u32| {
        address(
            Owner::Strip(id("t0000")),
            Section::Inserts(id("eq")),
            parameter,
            Lane::Both,
        )
    };
    let kinds: Vec<(&str, Vec<SessionEdit>)> = vec![
        ("fader", vec![set(both_lanes("t0007", FADER_DB), -9.5)]),
        (
            "fader under a nested VCA",
            vec![set(both_lanes("t0002", FADER_DB), -9.5)],
        ),
        ("mute", vec![set(both_lanes("t0006", MUTE), 1.0)]),
        (
            "mute of a strip a send follows",
            vec![set(both_lanes("t0000", MUTE), 1.0)],
        ),
        (
            "pan",
            vec![
                set(strip_row("t0004", PAN, Lane::Left), 0.25),
                set(strip_row("t0004", PAN, Lane::Right), -0.25),
            ],
        ),
        ("trim", vec![set(both_lanes("t0005", TRIM_DB), -3.0)]),
        ("effect parameter", vec![set(eq(4), -4.0)]),
        (
            "send gain",
            vec![set(
                address(Owner::Route(id("t0000-bus")), Section::Own, 1, Lane::Both),
                -6.0,
            )],
        ),
        (
            "send mute",
            vec![set(
                address(Owner::Route(id("t0001-bus")), Section::Own, 2, Lane::Both),
                1.0,
            )],
        ),
        (
            "VCA offset",
            vec![set(
                address(Owner::Vca(id("v0")), Section::Own, 1, Lane::Both),
                -3.0,
            )],
        ),
        (
            "VCA mute",
            vec![set(
                address(Owner::Vca(id("v1")), Section::Own, 2, Lane::Left),
                1.0,
            )],
        ),
    ];
    for (name, edits) in kinds {
        let counts: Vec<(u64, u64, bool)> = stores
            .iter()
            .map(|store| allocations(store, &edits, &MonitorOverlay::default()))
            .collect();
        assert!(
            counts.iter().all(|count| count.2),
            "{name}: fast path {counts:?}"
        );
        assert!(counts[0].0 > 0, "{name}: the call is counted: {counts:?}");
        println!(
            "{name}: {} allocations, {} frees at {sizes:?} tracks",
            counts[0].0, counts[0].1
        );
        assert!(
            counts
                .iter()
                .all(|count| count.0 == counts[0].0 && count.1 == counts[0].1),
            "{name}: the allocations depend on the track count {sizes:?}: {counts:?}"
        );
    }
    // A solo that leaves some other strip soloed moves one strip.
    let others = MonitorOverlay::with_soloed([id("t0006")]);
    let solo = vec![set(own_row("t0005", SOLO), 1.0)];
    let counts: Vec<(u64, u64, bool)> = stores
        .iter()
        .map(|store| allocations(store, &solo, &others))
        .collect();
    assert!(
        counts.iter().all(|count| count.2),
        "solo: fast path {counts:?}"
    );
    assert!(
        counts
            .iter()
            .all(|count| count.0 == counts[0].0 && count.1 == counts[0].1),
        "solo: the allocations depend on the track count: {counts:?}"
    );
}

// ---------------------------------------------------------------------------------------------
// Gate 3: one descriptive timing line
// ---------------------------------------------------------------------------------------------

/// The timing line: a fader edit on the session #1562a timed, 200 calls, by the full classifier
/// (before) and by `classify_live_edit` (after).
#[test]
#[ignore = "descriptive timing: cargo test --release -p host-core --all-features --test live_edit_classifier -- --ignored --nocapture"]
fn timing_line() {
    for tracks in [64, 256] {
        let store = store(plain_session(tracks));
        let edits = [set(strip_row("t0003", FADER_DB, Lane::Both), -9.0)];
        let none = MonitorOverlay::default();
        let (prepared, overlay, ramp) = prepare(&store, &edits, &none).expect("applies");
        let current = store.compiled().normalized_model();
        let next = prepared
            .compiled()
            .expect("a compiled transaction")
            .normalized_model();
        let monitor = MonitorBase {
            committed: &none,
            next: &overlay,
            ramp,
        };
        let ramps = LiveRamps::for_session(next);
        let calls = 200;
        let start = Instant::now();
        for _ in 0..calls {
            let delta = classify_live_delta_with_monitor(
                current,
                next,
                ramps,
                prepared.edit_ramps(),
                monitor,
            )
            .expect("live");
            assert_eq!(delta.strips.len(), 1);
        }
        let before = start.elapsed() / calls;
        let start = Instant::now();
        for _ in 0..calls {
            let delta = classify_live_edit(
                current,
                next,
                ramps,
                prepared.edit_ramps(),
                monitor,
                prepared.set_control_rows(),
            )
            .expect("live");
            assert_eq!(delta.strips.len(), 1);
        }
        let after = start.elapsed() / calls;
        // The protocol's preparation of the same transaction (a model clone and a compile), which
        // this slice does not change: the share of a whole command that stays linear in the session.
        let start = Instant::now();
        for _ in 0..calls {
            let prepared = prepare(&store, &edits, &none).expect("applies");
            drop(prepared);
        }
        let preparation = start.elapsed() / calls;
        println!(
            "classifier fader edit, {tracks} tracks: full {before:?} per call, fast {after:?} per call; protocol preparation of the transaction {preparation:?}"
        );
    }
}
