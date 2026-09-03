//! Issue #178: a `SessionEdit` that addresses the strip **as a rack** is refused.
//!
//! `RackName` gained a fourth token so an automation *target* can name the strip. Nothing else in
//! the session grammar grew: the strip is a `DualMonoBuiltins` and holds no `effects` vector, so
//! every rack-addressed edit -- `SetTrackRack`, `PutTrackEffect`, `RemoveTrackEffect`,
//! `SetTrackEffectOrder`, `SetEffectQuality` and their siblings -- is addressing something that
//! does not exist when it names `builtins`.
//!
//! `rack_mut` answers [`SessionEditError::NotFound`], which is the same answer a named-but-absent
//! effect gets, rather than a panicking arm or a silent no-op. This file is the gate on that: an
//! `unreachable!()` there would abort the control thread on a well-formed wire message, and a
//! `&mut track.simd1` fallback would edit a rack the caller did not name.
//!
//! The strip **is** editable, through `SetTrackBuiltins`, and the last test says so -- without it
//! a refusal that refused everything would pass.
//!
//! Red mutation: give `rack_mut`'s `RackName::Builtins` arm `Ok(&mut track.simd1)` -> the four
//! refusal arms below start reporting `Ok` and the effect lands in `simd1`.

use protocol::{SessionEdit, SessionEditError, apply_session_edit};
use session::{RackName, SessionModel, parse_session_json};

const SESSION: &str = include_str!("../../../fixtures/session/v1/canonical.json");

fn session() -> SessionModel {
    parse_session_json(SESSION).expect("the canonical fixture parses")
}

fn track_id() -> session::StableId {
    session().tracks[0].id.clone()
}

#[test]
fn rack_addressed_edits_refuse_the_builtins_token() {
    let effect = session().tracks[0].dynamic.effects[0].clone();
    let rack = session().tracks[0].dynamic.clone();
    for (name, edit) in [
        (
            "SetTrackRack",
            SessionEdit::SetTrackRack {
                track_id: track_id(),
                rack_name: RackName::Builtins,
                rack: rack.clone(),
            },
        ),
        (
            "PutTrackEffect",
            SessionEdit::PutTrackEffect {
                track_id: track_id(),
                rack_name: RackName::Builtins,
                final_position: 0,
                effect: effect.clone(),
            },
        ),
        (
            "RemoveTrackEffect",
            SessionEdit::RemoveTrackEffect {
                track_id: track_id(),
                rack_name: RackName::Builtins,
                effect_id: effect.id.clone(),
            },
        ),
        (
            "SetTrackEffectOrder",
            SessionEdit::SetTrackEffectOrder {
                track_id: track_id(),
                rack_name: RackName::Builtins,
                effect_ids: vec![effect.id.clone()],
            },
        ),
    ] {
        let mut model = session();
        let before = model.clone();
        assert_eq!(
            apply_session_edit(&mut model, &edit),
            Err(SessionEditError::NotFound),
            "{name} addressed at the strip must be refused, not applied"
        );
        assert_eq!(
            model, before,
            "{name}: a refused edit must leave the model exactly as it was"
        );
    }
}

/// The same edits against a real rack are applied, so the refusal above is about the token and not
/// about the edits.
#[test]
fn the_same_edits_against_a_real_rack_are_applied() {
    let rack = session().tracks[0].dynamic.clone();
    let mut model = session();
    assert_eq!(
        apply_session_edit(
            &mut model,
            &SessionEdit::SetTrackRack {
                track_id: track_id(),
                rack_name: RackName::Simd1,
                rack: rack.clone(),
            },
        ),
        Ok(()),
    );
    assert_eq!(model.tracks[0].simd1, rack);
}

/// And the strip is edited through the edit that owns it, which the new token does not touch.
#[test]
fn the_strip_is_still_editable_through_set_track_builtins() {
    let mut model = session();
    let mut builtins = model.tracks[0].builtins.clone();
    builtins.left.trim_db = -6.0;
    assert_eq!(
        apply_session_edit(
            &mut model,
            &SessionEdit::SetTrackBuiltins {
                track_id: track_id(),
                builtins: builtins.clone(),
            },
        ),
        Ok(()),
    );
    assert_eq!(model.tracks[0].builtins, builtins);
}
