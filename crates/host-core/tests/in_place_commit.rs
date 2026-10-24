//! #1613: a live-only transaction committed without cloning and compiling the model.
//!
//! The protocol prepares a transaction of `SetControl`s, on existing values or adding parameter
//! entries (#1620), **in place** (`SessionStore::prepare_transaction_in_place`): the owners the edits name, edited, and nothing
//! of the model. The classifier reads the committed model and those owners as one view
//! (`classify_live_staged`), the catalog stage reads them through the same view, and the commit
//! swaps the owners in. This file holds the differential gate: the in-place chain must equal the
//! full chain (a clone of the model, compiled; the dense classifier; the dense stage) on every
//! transaction, and the committed session after any run of commits must equal the full path's,
//! byte for byte.
#![cfg(all(feature = "control-provider", feature = "test-support"))]

use std::collections::BTreeSet;

#[path = "support/edit_sessions.rs"]
mod edit_sessions;
use edit_sessions::*;

use controls::{Lane, ModelView, MonitorOverlay, Owner, Section};
use host_core::{
    ControlCatalog, LiveRamps, MonitorBase, StagedCatalog, classify_live_delta_with_monitor,
    classify_live_staged,
};
use protocol::{ExpectedRevision, PreparedSessionTransaction, SessionEdit, SessionStore};
use session::Vca;

/// What the pair did with the transactions it was given.
#[derive(Default)]
struct Tally {
    /// Transactions both chains refused with the same error.
    refused: usize,
    /// Transactions prepared in place and classified from their rows.
    in_place: usize,
    /// Transactions of solo writes only, prepared with no compilation.
    monitor_only: usize,
    /// Transactions the in-place preparation handed to the full one.
    full_at_preparation: usize,
    /// Transactions prepared in place that the fast classification could not judge.
    full_at_classification: usize,
    /// Transactions prepared but not committed, whose store stayed as it was.
    abandoned: usize,
    /// Row names the in-place commits wrote, and those the full path took.
    in_place_rows: BTreeSet<&'static str>,
    full_rows: BTreeSet<&'static str>,
    /// Verdicts of the full classifier for the transactions committed in place.
    live_with_records: usize,
    /// In-place stages that wrote values only, and those that built the whole catalog.
    staged_values: usize,
    staged_full: usize,
}

/// Two stores that must stay equal: one committed by the full path, one by the in-place preparation.
struct Pair {
    full: SessionStore,
    fast: SessionStore,
    overlay: MonitorOverlay,
}

impl Pair {
    fn new(model: session::SessionModel) -> Self {
        Self {
            full: store(model.clone()),
            fast: store(model),
            overlay: MonitorOverlay::default(),
        }
    }

    /// Both stores hold the same session: the model, its canonical text, its estimate, its revision.
    fn assert_equal(&self, label: &str) {
        assert_eq!(
            self.full.compiled().normalized_model(),
            self.fast.compiled().normalized_model(),
            "{label}: the model"
        );
        assert_eq!(
            self.full.canonical_snapshot(),
            self.fast.canonical_snapshot(),
            "{label}: the canonical snapshot"
        );
        assert_eq!(
            self.full.compiled().resource_estimate(),
            self.fast.compiled().resource_estimate(),
            "{label}: the resource estimate"
        );
        assert_eq!(
            self.full.revision(),
            self.fast.revision(),
            "{label}: revision"
        );
        // The snapshot is the canonical text of the model, whichever way the model came to be.
        assert_eq!(
            self.fast.canonical_snapshot(),
            session::canonical_session_json(self.fast.compiled().normalized_model())
                .expect("canonical")
                .as_str(),
            "{label}: the snapshot is the model's canonical JSON"
        );
    }

    /// One transaction through both chains, committed when `commit` is set.
    fn step(
        &mut self,
        edits: &[SessionEdit],
        names: &[&'static str],
        commit: bool,
        tally: &mut Tally,
    ) {
        let revision = ExpectedRevision::Exact(self.full.revision());
        let full = self.full.prepare_transaction_with_monitor(revision, edits);
        let fast = self.fast.prepare_transaction_in_place(revision, edits);
        let (full, fast) = match (full, fast) {
            (Err(left), Err(right)) => {
                assert_eq!(left, right, "the same refusal");
                tally.refused += 1;
                self.assert_equal("after a refusal");
                return;
            }
            (Ok(full), Ok(fast)) => (full, fast),
            (left, right) => panic!(
                "one chain refused: full {:?} fast {:?}",
                left.err(),
                right.err()
            ),
        };
        assert_eq!(
            full.revision(),
            fast.revision(),
            "the revision it commits as"
        );
        assert_eq!(full.monitor_edits(), fast.monitor_edits());
        assert_eq!(full.is_monitor_only(), fast.is_monitor_only());
        assert_eq!(full.edit_ramps(), fast.edit_ramps());
        assert_eq!(full.set_control_rows(), fast.set_control_rows());

        let current_full = self.full.compiled().normalized_model();
        let next_full = full
            .compiled()
            .expect("a compiled transaction")
            .normalized_model();
        let change = self.overlay.after(full.monitor_edits(), next_full);
        let fast_change = self.overlay.after(
            fast.monitor_edits(),
            self.fast.compiled().normalized_model(),
        );
        assert_eq!(change, fast_change, "the overlay the transaction leaves");
        let monitor = MonitorBase {
            committed: &self.overlay,
            next: &change.overlay,
            ramp: change.ramp,
        };
        let ramps = LiveRamps::for_session(next_full);
        let oracle = classify_live_delta_with_monitor(
            current_full,
            next_full,
            ramps,
            full.edit_ramps(),
            monitor,
        );

        if fast.is_in_place() {
            let committed = self.fast.compiled().normalized_model();
            let view = match fast.edited_owners() {
                Some(owners) => ModelView::edited(committed, owners),
                None => ModelView::whole(committed),
            };
            let rows = fast
                .set_control_rows()
                .expect("rows of a transaction of values");
            let delta = classify_live_staged(
                committed,
                view,
                LiveRamps::for_session(committed),
                fast.edit_ramps(),
                monitor,
                rows,
            );
            match delta {
                Some(mut delta) => {
                    if fast.is_monitor_only() {
                        tally.monitor_only += 1;
                    } else {
                        tally.in_place += 1;
                        tally.in_place_rows.extend(names);
                    }
                    assert!(delta.derived_owners.is_some());
                    let staged_owners = delta.derived_owners.take();
                    let oracle = oracle
                        .as_ref()
                        .expect("a live transaction is live in the oracle");
                    assert_eq!(&delta, oracle, "the delta of the in-place chain");
                    tally.live_with_records += usize::from(!delta.is_empty());
                    // The catalog stage: the in-place stage must give what the dense one gives.
                    // (`stage_live` also checks, in a debug build, that a values-only stage equals a
                    // full build of the model it stages.)
                    let rate = committed.sample_rate_hz;
                    let mut delta = delta;
                    let dense = ControlCatalog::build(current_full, &self.overlay, rate)
                        .expect("catalog")
                        .stage_live(ModelView::whole(next_full), &change.overlay, rate, oracle)
                        .expect("the dense stage");
                    delta.derived_owners = staged_owners;
                    let local = ControlCatalog::build(committed, &self.overlay, rate)
                        .expect("catalog")
                        .stage_live(view, &change.overlay, rate, &delta)
                        .expect("the stage over the view");
                    match (&dense, &local) {
                        (StagedCatalog::Values(left), StagedCatalog::Values(right)) => {
                            assert!(right.len() <= left.len(), "the local stage reads no more");
                            tally.staged_values += 1;
                        }
                        (StagedCatalog::Full(_), StagedCatalog::Full(_)) => tally.staged_full += 1,
                        _ => panic!("the stages differ in kind"),
                    }
                }
                None => {
                    tally.full_at_classification += 1;
                    tally.full_rows.extend(names);
                }
            }
        } else {
            tally.full_at_preparation += 1;
            tally.full_rows.extend(names);
        }
        // A transaction the host drops before it commits leaves the store as it was.
        let before = (
            self.fast.canonical_snapshot().to_owned(),
            self.fast.compiled().resource_estimate(),
            self.fast.revision(),
        );
        drop(fast);
        drop(full);
        assert_eq!(
            (
                self.fast.canonical_snapshot(),
                self.fast.compiled().resource_estimate(),
                self.fast.revision()
            ),
            (before.0.as_str(), before.1, before.2),
            "an abandoned transaction leaves the store as it was"
        );
        if !commit {
            tally.abandoned += 1;
            return;
        }
        let left = self.full.apply_transaction_with_monitor(revision, edits);
        let right = self.fast.apply_transaction_in_place(revision, edits);
        assert_eq!(left, right, "the commit");
        self.overlay = change.overlay;
        self.assert_equal("after a commit");
    }
}

/// An in-place preparation of `edits` on `store`, for the tests below that look at the form.
fn prepare_in_place(
    store: &SessionStore,
    edits: &[SessionEdit],
) -> Result<PreparedSessionTransaction, protocol::SessionStoreError> {
    store.prepare_transaction_in_place(ExpectedRevision::Exact(store.revision()), edits)
}

/// Gate 1a. The #1560 parity script: every edit through both chains, in order, with the solo steps
/// between, at two rates; the stores equal after each.
///
/// Red if the in-place commit differs from the full one anywhere on the script, or if the rows it
/// exists for do not take the in-place form.
#[test]
fn the_parity_script_commits_as_the_full_path_does() {
    for rate in [48_000, 96_000] {
        let mut pair = Pair::new(parity_session(rate));
        let mut tally = Tally::default();
        let solo = |strip: &str, on: bool| {
            vec![SessionEdit::set_control(
                controls::ControlAddress {
                    owner: Owner::Strip(id(strip)),
                    section: Section::Own,
                    control: SOLO,
                    lane: Lane::Both,
                },
                f32::from(u8::from(on)),
                None,
            )]
        };
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
        for step in &script {
            pair.step(&step.edits, &["script"], true, &mut tally);
        }
        assert!(
            tally.in_place > 30,
            "the script's rows took the in-place form: {}",
            tally.in_place
        );
        assert!(
            tally.refused >= 3,
            "the script's refusals were refused alike"
        );
        assert!(
            tally.full_at_preparation >= 2,
            "the structural and matrix-mode steps are full"
        );
        assert!(tally.monitor_only >= 4, "the solo steps");
    }
}

/// #1653 gate 3 through the in-place path (R17-1): the edits of a following route (a mute entry on
/// `t1`, its following send `t1-b`) through both chains, in order, at two rates; the stores equal
/// after each, and every value edit takes the in-place form with the full path's delta.
///
/// Red if the fast classifier lowers a following route's terms or `follows_mute` change
/// differently from the dense one (a missing terms trigger, a folded record), or if those rows do
/// not take the in-place form.
#[test]
fn a_following_routes_edits_commit_as_the_full_path_does() {
    for rate in [48_000, 96_000] {
        let mut pair = Pair::new(following_session(rate));
        let mut tally = Tally::default();
        let script = following_script();
        for edits in &script {
            pair.step(edits, &["following"], true, &mut tally);
        }
        assert_eq!(tally.refused, 0, "nothing is refused");
        assert_eq!(tally.monitor_only, 2, "the solo steps");
        assert_eq!(
            tally.in_place,
            script.len() - 2,
            "every value edit took the in-place form"
        );
    }
}

/// Gate 1b. Every row kind of each session first, deterministically: one committed transaction of
/// one edit per row name of the control list, and one structural edit ([`first_appearances`]).
/// Then a seeded random walk of at least 2,400 transactions of one to four edits drawn from every
/// row of the control list (every class, the session's rows, a strip's matrix mode, an insert's
/// link mode, the solo rows), with and without a per-edit ramp, over a random committed overlay,
/// sometimes with a structural edit, sometimes abandoned: the stores stay equal and the in-place
/// chain gives the full chain's delta, catalog stage and refusal.
///
/// The first appearances make the coverage assertion a property of the row list, not of the seed:
/// a rare row kind (an insert's link mode is one row among hundreds) is reached however a new row
/// shifts the random draws (#1340, whose live multiband bypass row left the walk without one full
/// `link_mode` commit at this seed).
///
/// Red if they differ anywhere, if a transaction the in-place form accepted is refused by the full
/// path, or if the generator did not reach every live row through the in-place form and every
/// fallback through the full one.
#[test]
fn a_random_walk_of_transactions_commits_as_the_full_path_does() {
    let mut pairs = [Pair::new(rich_session(10)), Pair::new(plain_session(4))];
    let mut draw = Draw(0x9E37_79B9_7F4A_7C15);
    let mut tally = Tally::default();
    for pair in &mut pairs {
        for (edits, names) in first_appearances(pair) {
            pair.step(&edits, &names, true, &mut tally);
        }
    }
    // The fallbacks are reached before the random tail, whatever its seed draws.
    for row in ["matrix_mode", "fader_ms", "link_mode", "<structural>"] {
        assert!(
            tally.full_rows.contains(row),
            "no first-appearance full commit of {row}"
        );
    }
    let mut cases = 0_usize;
    while cases < 2_600 {
        let at = usize::from(cases % 6 == 5);
        let pair = &mut pairs[at];
        let case = draw_case(&mut draw, pair.full.compiled().normalized_model());
        // The overlay the pair holds is the walk's own; the case's drawn overlay is not used.
        let commit = draw.next(8) != 0;
        pair.step(&case.edits, &case.names, commit, &mut tally);
        cases += 1;
    }
    for pair in &pairs {
        pair.assert_equal("end of the walk");
    }
    assert!(tally.in_place > 600, "{}", tally.in_place);
    assert!(tally.abandoned > 100, "{}", tally.abandoned);
    assert!(tally.refused > 50, "{}", tally.refused);
    assert!(
        tally.full_at_preparation > 100,
        "{}",
        tally.full_at_preparation
    );
    assert!(tally.live_with_records > 400, "{}", tally.live_with_records);
    println!(
        "staged values {} full {}",
        tally.staged_values, tally.staged_full
    );
    assert!(
        tally.staged_values > 8 * tally.staged_full,
        "a value-only commit writes values: {} values, {} full",
        tally.staged_values,
        tally.staged_full
    );
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
        assert!(
            tally.in_place_rows.contains(row),
            "no in-place commit of {row}"
        );
    }
}

/// One transaction per row name of `pair`'s control list, of one edit on the first row with that
/// name (in list order), with a value its domain accepts other than the committed one and no
/// ramp, and one structural edit: the deterministic first appearance of every row kind before the
/// walk's random tail.
fn first_appearances(pair: &Pair) -> Vec<(Vec<SessionEdit>, Vec<&'static str>)> {
    let model = pair.full.compiled().normalized_model();
    let mut draw = Draw(0x1340_1340_1340_1341);
    let mut seen = BTreeSet::new();
    let mut cases = Vec::new();
    for row in controls::list(model, &pair.overlay, model.sample_rate_hz) {
        if row.descriptor.class == controls::ControlClass::Derived
            || !row.descriptor.active
            || !seen.insert(row.descriptor.name)
        {
            continue;
        }
        // A value other than the committed one, so the edit is a change its path must carry.
        let mut value = edit_sessions::draw_value(&mut draw, &row);
        for _ in 0..16 {
            if value.to_bits() != row.value.to_bits() {
                break;
            }
            value = edit_sessions::draw_value(&mut draw, &row);
        }
        cases.push((
            vec![SessionEdit::set_control(row.address.clone(), value, None)],
            vec![row.descriptor.name],
        ));
    }
    let tracks = &model.tracks;
    cases.push((
        vec![SessionEdit::UpsertVca {
            vca: Vca {
                id: id("zz-first"),
                fader: session::DualMonoFader {
                    left_db: -1.0,
                    right_db: 0.0,
                    left_mute: false,
                    right_mute: false,
                },
                members: vec![tracks[0].id.clone()],
            },
        }],
        vec!["<structural>"],
    ));
    cases
}

/// What the in-place preparation hands to the full one, one transaction each and each beside a
/// fader edit: a row that moves more than itself (a strip's matrix mode), a session row, and a
/// structural edit. And what the full path refuses, the in-place entry refuses with the same error:
/// a value outside its domain and a ramp longer than a second.
///
/// Red if any of them is prepared in place, or refused differently.
#[test]
fn what_the_in_place_preparation_cannot_prove_is_prepared_by_the_full_path() {
    let store = store(rich_session(10));
    let row = |owner: Owner, section: Section, control: u32| controls::ControlAddress {
        owner,
        section,
        control,
        lane: Lane::Both,
    };
    let fader = |value: f32, ramp: Option<u32>| {
        SessionEdit::set_control(
            row(Owner::Strip(id("t0005")), Section::Builtins, FADER_DB),
            value,
            ramp,
        )
    };
    assert!(
        prepare_in_place(&store, &[fader(-7.0, None)])
            .expect("a fader moves")
            .is_in_place(),
        "a fader is prepared in place"
    );
    let full = [
        // The matrix mode flips which rows are active.
        SessionEdit::set_control(row(Owner::Strip(id("t0000")), Section::Own, 1), 1.0, None),
        // A session row changes every owner's descriptors.
        SessionEdit::set_control(row(Owner::Session, Section::Own, 1), 40.0, None),
        // A structural edit.
        SessionEdit::UpsertVca {
            vca: Vca {
                id: id("extra"),
                fader: session::DualMonoFader {
                    left_db: 0.0,
                    right_db: 0.0,
                    left_mute: false,
                    right_mute: false,
                },
                members: vec![id("t0000")],
            },
        },
    ];
    for edit in &full {
        for edits in [vec![edit.clone()], vec![fader(-7.0, None), edit.clone()]] {
            let prepared = prepare_in_place(&store, &edits).expect("the full path accepts it");
            assert!(!prepared.is_in_place(), "{edits:?} is the full path's");
        }
    }
    for edits in [
        vec![fader(400.0, None)],
        vec![fader(-3.0, Some(48_001))],
        vec![fader(-3.0, None), fader(f32::NAN, None)],
    ] {
        let full = store
            .prepare_transaction_with_monitor(ExpectedRevision::Exact(store.revision()), &edits);
        assert!(full.is_err(), "the full path refuses {edits:?}");
        assert_eq!(
            prepare_in_place(&store, &edits).err(),
            full.err(),
            "the in-place entry refuses {edits:?} alike"
        );
    }
}

/// #1620, gate 1. Transactions whose `SetControl`s add parameter entries to sparse lists: on the
/// console EQ that holds IDs 1 to 6 (above them, one lane and both), on an insert EQ that holds
/// none (then below and between the entries it gained), splitting a `both` entry into two lanes,
/// and across several effects of tracks and a submix, console and inserts, in one transaction.
/// Each is prepared in place, classified from its rows and committed; the stores (model, canonical
/// snapshot, estimate, revision) equal the full path's after each.
///
/// Red if an added entry is charged differently from a compile (estimate), lands where the full
/// path's sort does not put it (model, snapshot), or sends the transaction to the full path.
#[test]
fn edits_that_add_parameter_entries_commit_in_place_as_the_full_path_does() {
    let mut pair = Pair::new(rich_session(10));
    let mut tally = Tally::default();
    let row = |strip: &str, section: Section, control: u32, lane: Lane, value: f32| {
        set(
            address(Owner::Strip(id(strip)), section, control, lane),
            value,
        )
    };
    let console = |strip: &str, slot: &str, control: u32, lane: Lane, value: f32| {
        row(strip, Section::Console(id(slot)), control, lane, value)
    };
    let insert = |strip: &str, effect: &str, control: u32, lane: Lane, value: f32| {
        row(strip, Section::Inserts(id(effect)), control, lane, value)
    };
    let script: Vec<(Vec<SessionEdit>, bool)> = vec![
        // Above the console EQ's IDs 1 to 6: a `both` entry and a one-lane entry.
        (
            vec![
                console("t0000", "eq", 66, Lane::Both, 400.0),
                console("t0000", "eq", 82, Lane::Left, 6_000.0),
            ],
            true,
        ),
        // An empty insert list gains an entry, then one below and one between.
        (vec![insert("t0000", "eq", 82, Lane::Both, 6_000.0)], true),
        (
            vec![
                insert("t0000", "eq", 4, Lane::Right, -3.0),
                insert("t0000", "eq", 66, Lane::Left, 300.0),
            ],
            true,
        ),
        // A one-lane write on a `both` entry splits it.
        (vec![insert("t0000", "eq", 82, Lane::Left, 5_000.0)], true),
        // Abandoned: the store stays as it was.
        (vec![insert("t0000", "eq", 3, Lane::Left, 200.0)], false),
        // Several effects at once: tracks and the submix, console and inserts, both lanes.
        (
            vec![
                insert("t0001", "comp", 1, Lane::Left, -30.0),
                insert("t0002", "gate", 1, Lane::Both, 0.0),
                console("t0003", "post-clip", 1, Lane::Both, 12.0),
                console("bus", "eq", 4, Lane::Right, -2.0),
                insert("t0004", "multi", 3, Lane::Both, -40.0),
                console("t0009", "eq", 82, Lane::Both, 7_000.0),
                set(strip_row("t0005", FADER_DB, Lane::Both), -4.0),
            ],
            true,
        ),
        // The other lane of an entry that exists on one lane only.
        (vec![insert("t0001", "comp", 1, Lane::Right, -20.0)], true),
    ];
    let mut committed = 0;
    for (edits, commit) in &script {
        let before = pair.full.compiled().resource_estimate().parameter_count;
        let prepared = prepare_in_place(&pair.fast, edits).expect("prepares");
        assert!(prepared.is_in_place(), "{edits:?} is prepared in place");
        assert!(
            prepared
                .in_place_estimate()
                .expect("in place")
                .parameter_count
                > before,
            "{edits:?} adds parameter entries"
        );
        drop(prepared);
        pair.step(edits, &["added"], *commit, &mut tally);
        committed += usize::from(*commit);
    }
    assert_eq!(
        tally.in_place,
        script.len(),
        "every transaction was prepared and classified in place"
    );
    assert_eq!(
        (tally.full_at_preparation, tally.full_at_classification),
        (0, 0)
    );
    assert_eq!(tally.abandoned, script.len() - committed);
    pair.assert_equal("after the script");
}

/// #1620: an added entry is charged against the store's caps before anything is committed. With a
/// cap at the committed session's figure, a change of values still commits in place, and an edit
/// that adds an entry is refused, by both entries alike, with the store unchanged.
///
/// Red if the in-place path commits an added entry that a compile's caps refuse (an ack the full
/// path would not give).
#[test]
fn an_added_entry_over_the_caps_is_refused_before_any_commit() {
    let model = rich_session(10);
    let estimate = store(model.clone()).compiled().resource_estimate();
    // The compile's figures before the canonical text is written, which its caps bound.
    let upper_bound = estimate.compiled_model_bytes - estimate.canonical_bytes
        + estimate.canonical_upper_bound_bytes;
    let tight = [
        session::CompileCaps {
            max_compiled_model_bytes: upper_bound,
            ..caps()
        },
        session::CompileCaps {
            max_single_allocation_bytes: estimate.single_allocation_bytes,
            ..caps()
        },
    ];
    let fader = set(strip_row("t0005", FADER_DB, Lane::Both), -4.0);
    let added = set(
        address(
            Owner::Strip(id("t0000")),
            Section::Console(id("eq")),
            66,
            Lane::Both,
        ),
        400.0,
    );
    for caps in tight {
        let mut store = SessionStore::new(model.clone(), caps).expect("compiles at its caps");
        let current = store.revision();
        let revision = ExpectedRevision::Exact(current);
        assert!(
            prepare_in_place(&store, std::slice::from_ref(&fader))
                .expect("a fader moves")
                .is_in_place()
        );
        let edits = [fader.clone(), added.clone()];
        let full = store.prepare_transaction_with_monitor(revision, &edits);
        assert!(
            matches!(full, Err(protocol::SessionStoreError::Validation { .. })),
            "the compile refuses the added entry: {:?}",
            full.as_ref().err()
        );
        assert_eq!(prepare_in_place(&store, &edits).err(), full.err());
        let before = store.canonical_snapshot().to_owned();
        assert!(store.apply_transaction_in_place(revision, &edits).is_err());
        assert_eq!(store.revision(), current);
        assert_eq!(store.canonical_snapshot(), before);
    }
}
