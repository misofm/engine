//! verifier probe
use protocol::{ExpectedRevision, SessionEdit, SessionStore};
use session::{CompileCaps, EditRampRow, EditRampTarget, LinkMode, RackName, parse_session_json};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

fn caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

#[test]
fn probe_sizes() {
    println!("PreparedStructuralCommand {}", size_of::<protocol::PreparedStructuralCommand>());
    println!("EditRamps {}", size_of::<session::EditRamps>());
}

#[test]
fn probe_stale_effect_entry_after_put() {
    let model = parse_session_json(FIXTURE).expect("fixture");
    let store = SessionStore::new(model, caps()).expect("store");
    let m = store.compiled().normalized_model();
    let track = &m.tracks[0];
    let effect = track.inserts.effects[0].clone();
    let other = if effect.link_mode == LinkMode::Maximum { LinkMode::Average } else { LinkMode::Maximum };
    let mut replaced = effect.clone();
    replaced.link_mode = effect.link_mode; // the put writes the original link mode back, with no length
    let edits = [
        SessionEdit::SetEffectLinkMode {
            track_id: track.id.clone(),
            rack_name: RackName::Inserts,
            effect_id: effect.id.clone(),
            link_mode: other,
            ramp_samples: Some(37),
        },
        SessionEdit::PutTrackEffect {
            track_id: track.id.clone(),
            rack_name: RackName::Inserts,
            final_position: 0,
            effect: replaced,
        },
    ];
    let prepared = store
        .prepare_transaction(ExpectedRevision::Exact(store.revision()), &edits)
        .expect("prepares");
    let target = EditRampTarget::Effect { strip: track.id.clone(), rack: RackName::Inserts, effect: effect.id.clone() };
    println!("STALE after put: {:?}", prepared.edit_ramps().get(&target, EditRampRow::Link));
    let after = prepared.compiled().normalized_model().tracks[0].inserts.effects[0].link_mode;
    println!("final link {:?} (base {:?})", after, effect.link_mode);
}
