//! Issue #2394: a transaction of solo writes prepared on the whole path holds no compilation. It
//! changes no model and no revision, and its compilation is the committed one, which the store
//! already holds, so its preparation makes no clone of it.

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use protocol::control_core::{ControlAddress, Lane, Owner, Section};
use protocol::{ExpectedRevision, SessionEdit, SessionRevision, SessionStore};
use session::{CompileCaps, parse_session_json};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

/// The bytes `body` asks the allocator for on this thread.
fn requested_bytes<T>(body: impl FnOnce() -> T) -> (T, u64) {
    let mark = current_thread_counters();
    let value = body();
    (value, current_thread_delta_since(mark).requested_bytes)
}

/// Red if a whole-path solo preparation clones the committed `CompiledSession` again (#2394).
///
/// The preparation still clones the committed model as its candidate and builds its monitor-edit
/// vector, so the ceiling is one model clone plus one compilation clone: a preparation that also
/// clones the compilation requests at least that much, and one that does not requests the model,
/// the monitor write and nothing near a compilation.
#[test]
fn a_whole_path_solo_preparation_clones_no_compilation() {
    assert_installed();
    let model = parse_session_json(FIXTURE).expect("fixture");
    let track = model.tracks[0].id.clone();
    let caps = CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    };
    let store = SessionStore::new(model, caps).expect("the fixture compiles");
    let solo = [SessionEdit::SetControl {
        expected_effect: None,
        address: ControlAddress {
            owner: Owner::Strip(track),
            section: Section::Own,
            control: 2,
            lane: Lane::Both,
        },
        value: 1.0,
        ramp_samples: Some(96),
    }];
    let revision = ExpectedRevision::Exact(SessionRevision(7));
    // Warm every lazily allocated static the preparation reaches.
    drop(
        store
            .prepare_transaction_with_monitor(revision, &solo)
            .expect("a solo prepares"),
    );

    let (model_clone, model_bytes) =
        requested_bytes(|| store.compiled().normalized_model().clone());
    drop(model_clone);
    let (compiled_clone, compiled_bytes) = requested_bytes(|| store.compiled().clone());
    drop(compiled_clone);
    let (prepared, prepare_bytes) = requested_bytes(|| {
        store
            .prepare_transaction_with_monitor(revision, &solo)
            .expect("a solo prepares")
    });
    assert!(prepared.is_monitor_only() && !prepared.is_in_place());
    assert_eq!(prepared.monitor_edits().len(), 1);
    assert!(
        prepare_bytes < model_bytes + compiled_bytes,
        "a whole-path solo requested {prepare_bytes} bytes: at least its candidate model \
         ({model_bytes}) and a clone of the committed compilation ({compiled_bytes})"
    );
}
