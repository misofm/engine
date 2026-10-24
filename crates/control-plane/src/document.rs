//! `SESSION_REPLACE` (#1386): the check, parse and diff of a replacement document against the
//! committed model.
//!
//! The control plane sets [`replace`] as the controller's document hook
//! ([`protocol::DocumentReplace`]), so both hosts run it through the one core path,
//! `SessionState::command_with_document`. It runs on the control thread and keeps nothing of the
//! document: the borrowed bytes are checked, hashed, parsed and compiled, and only the edits and
//! the document's `control_smoothing` leave.
//!
//! **The diff (D2).** [`diff`] compares two normalized models entity by entity, by bits, never by
//! `PartialEq`: a fader at `-0.0` differs from one at `0.0`, as the live classifier's step 1 says.
//! One whole-entity edit per differing entity, never a finer edit, in this order:
//!
//! 1. the session-level setters (session ID, rate, quantum, render and output profiles);
//! 2. the removals: routes, automation, VCAs, tracks, submixes, outputs, sources;
//! 3. `SetConsole`;
//! 4. the upserts: sources, outputs, submixes, tracks, VCAs, routes, automation;
//! 5. one `SetControl` per session smoothing row whose length differs, when the document's arm is
//!    explicit.
//!
//! No edit checks a reference when it is applied: the store validates the final candidate once, so
//! a document that removes a source and the track that reads it is accepted. The revision and the
//! schema version are not diffed: the frame's exact revision is the guard, and a document of
//! another schema version does not parse. The default smoothing arm has no edit; the replace's
//! preparation sets the document's `control_smoothing` whole after the edits
//! (`SessionStore::prepare_replacement`).

use core::convert::Infallible;

use protocol::{
    DocumentDiff, DocumentReplaceRefusal, DocumentReplaceRequest, SessionEdit, StatusCode,
};
use session::{
    CONTROL_SMOOTHING_DEFAULT, ControlSmoothing, FieldKey, Lane, ModelVisitor, Owner, SessionModel,
    SourceBitDepth, StableId, Token, VisitModel, WalkOrder,
};
use sha2::{Digest, Sha256};

/// The document's length is not the frame's `document-bytes`, or no document came with it.
pub const DOCUMENT_LENGTH: &str = "session.replace.document_length";
/// The document is longer than the boot's `maximum_document_bytes`.
pub const DOCUMENT_TOO_LARGE: &str = "session.replace.document_too_large";
/// The document's SHA-256 is not the frame's `document-sha256`.
pub const DOCUMENT_DIGEST: &str = "session.replace.document_digest";
/// The document is not UTF-8, so it cannot parse.
pub const DOCUMENT_UTF8: &str = "session.replace.document_utf8";

/// The controller's `SESSION_REPLACE` hook (#1386 D3): in order, the document's length against
/// the frame's `document-bytes`, the length against `maximum_document_bytes` (0 is unbounded), the
/// SHA-256 against `document-sha256`, then the parse and the compile under the committed store's
/// caps, which validate it as a boot would, then [`diff`] against the committed model.
///
/// # Errors
///
/// [`DocumentReplaceRefusal`]: `INVALID_FIELD` for the length or the digest, `LIMIT_EXCEEDED` for
/// the bound, `VALIDATION_FAILED` for a document that is not UTF-8 or that does not parse or
/// validate (with the session's diagnostics).
pub fn replace(
    request: DocumentReplaceRequest<'_>,
) -> Result<DocumentDiff, DocumentReplaceRefusal> {
    let refused = |status, code| DocumentReplaceRefusal::Refused { status, code };
    let document = request.document;
    if u64::try_from(document.len()).ok() != Some(u64::from(request.request.document_bytes)) {
        return Err(refused(StatusCode::InvalidField, DOCUMENT_LENGTH));
    }
    if request.maximum_document_bytes != 0
        && u64::from(request.request.document_bytes) > request.maximum_document_bytes
    {
        return Err(refused(StatusCode::LimitExceeded, DOCUMENT_TOO_LARGE));
    }
    if Sha256::digest(document).as_slice() != request.request.document_sha256 {
        return Err(refused(StatusCode::InvalidField, DOCUMENT_DIGEST));
    }
    let text = core::str::from_utf8(document)
        .map_err(|_| refused(StatusCode::ValidationFailed, DOCUMENT_UTF8))?;
    let parsed = session::parse_session_json(text).map_err(DocumentReplaceRefusal::Invalid)?;
    let next = session::compile_session(&parsed, request.committed.caps())
        .map_err(DocumentReplaceRefusal::Invalid)?;
    let committed = request.committed.compiled().normalized_model();
    let next = next.normalized_model();
    let edits = diff(committed, next);
    if edits.is_empty() && same_smoothing(committed.control_smoothing, next.control_smoothing) {
        return Ok(DocumentDiff::Unchanged);
    }
    Ok(DocumentDiff::Changed {
        edits,
        control_smoothing: next.control_smoothing,
    })
}

/// The edits that turn `committed` into `next`, in the module's order. Both models must be
/// normalized (`CompiledSession::normalized_model`): every keyed list sorted by stable ID.
#[must_use]
pub fn diff(committed: &SessionModel, next: &SessionModel) -> Vec<SessionEdit> {
    let mut edits = Vec::new();
    if committed.session_id != next.session_id {
        edits.push(SessionEdit::SetSessionId {
            session_id: next.session_id.clone(),
        });
    }
    if committed.sample_rate_hz != next.sample_rate_hz {
        edits.push(SessionEdit::SetSampleRateHz {
            sample_rate_hz: next.sample_rate_hz,
        });
    }
    if committed.quantum_frames != next.quantum_frames {
        edits.push(SessionEdit::SetQuantumFrames {
            quantum_frames: next.quantum_frames,
        });
    }
    if !same_bits(&committed.render_profile, &next.render_profile) {
        edits.push(SessionEdit::SetRenderProfile {
            render_profile: next.render_profile.clone(),
        });
    }
    if !same_bits(&committed.output_profile, &next.output_profile) {
        edits.push(SessionEdit::SetOutputProfile {
            output_profile: next.output_profile.clone(),
        });
    }

    let id = |value: &StableId| value.clone();
    removals(
        &committed.routes,
        &next.routes,
        |r| &r.id,
        &mut edits,
        |r| SessionEdit::RemoveRoute { route_id: id(r) },
    );
    removals(
        &committed.automation,
        &next.automation,
        |a| &a.id,
        &mut edits,
        |a| SessionEdit::RemoveAutomation {
            automation_id: id(a),
        },
    );
    removals(
        &committed.vcas,
        &next.vcas,
        |v| &v.id,
        &mut edits,
        |v| SessionEdit::RemoveVca { vca_id: id(v) },
    );
    removals(
        &committed.tracks,
        &next.tracks,
        |t| &t.id,
        &mut edits,
        |t| SessionEdit::RemoveTrack { track_id: id(t) },
    );
    removals(
        &committed.submixes,
        &next.submixes,
        |s| &s.id,
        &mut edits,
        |s| SessionEdit::RemoveSubmix { submix_id: id(s) },
    );
    removals(
        &committed.outputs,
        &next.outputs,
        |o| &o.id,
        &mut edits,
        |o| SessionEdit::RemoveOutput { output_id: id(o) },
    );
    removals(
        &committed.sources,
        &next.sources,
        |s| &s.id,
        &mut edits,
        |s| SessionEdit::RemoveSource { source_id: id(s) },
    );

    if !same_bits(&committed.console, &next.console) {
        edits.push(SessionEdit::SetConsole {
            console: next.console.clone(),
            ramp_samples: None,
        });
    }

    upserts(
        &committed.sources,
        &next.sources,
        |s| &s.id,
        &mut edits,
        |s| SessionEdit::UpsertSource { source: s.clone() },
    );
    upserts(
        &committed.outputs,
        &next.outputs,
        |o| &o.id,
        &mut edits,
        |o| SessionEdit::UpsertOutput { output: o.clone() },
    );
    upserts(
        &committed.submixes,
        &next.submixes,
        |s| &s.id,
        &mut edits,
        |s| SessionEdit::UpsertSubmix { submix: s.clone() },
    );
    upserts(
        &committed.tracks,
        &next.tracks,
        |t| &t.id,
        &mut edits,
        |t| SessionEdit::UpsertTrack { track: t.clone() },
    );
    upserts(
        &committed.vcas,
        &next.vcas,
        |v| &v.id,
        &mut edits,
        |v| SessionEdit::UpsertVca { vca: v.clone() },
    );
    upserts(
        &committed.routes,
        &next.routes,
        |r| &r.id,
        &mut edits,
        |r| SessionEdit::UpsertRoute { route: r.clone() },
    );
    upserts(
        &committed.automation,
        &next.automation,
        |a| &a.id,
        &mut edits,
        |a| SessionEdit::UpsertAutomation {
            automation: a.clone(),
        },
    );

    // D2: an explicit arm writes each changed row; the default arm has no edit (the replace's
    // preparation sets it whole).
    if let Some(after) = next.control_smoothing {
        let before = committed
            .control_smoothing
            .unwrap_or(CONTROL_SMOOTHING_DEFAULT);
        for (control, old, new) in [
            (SMOOTHING_FADER_MS, before.fader_ms, after.fader_ms),
            (SMOOTHING_MUTE_MS, before.mute_ms, after.mute_ms),
            (SMOOTHING_PAN_MS, before.pan_ms, after.pan_ms),
        ] {
            if old.to_bits() != new.to_bits() {
                edits.push(SessionEdit::set_control(
                    protocol::control_core::ControlAddress {
                        owner: Owner::Session,
                        section: session::Section::Own,
                        control,
                        lane: Lane::Both,
                    },
                    new,
                    None,
                ));
            }
        }
    }
    edits
}

/// The session's own rows (`controls`' session table): `fader_ms`, `mute_ms`, `pan_ms`.
const SMOOTHING_FADER_MS: u32 = 1;
const SMOOTHING_MUTE_MS: u32 = 2;
const SMOOTHING_PAN_MS: u32 = 3;

/// One removal per entity of `committed` whose ID `next` does not hold, in ID order.
fn removals<T>(
    committed: &[T],
    next: &[T],
    id: impl Fn(&T) -> &StableId,
    edits: &mut Vec<SessionEdit>,
    edit: impl Fn(&StableId) -> SessionEdit,
) {
    for entity in committed {
        if find(next, id(entity), &id).is_none() {
            edits.push(edit(id(entity)));
        }
    }
}

/// One upsert per entity of `next` that `committed` lacks or holds with other bits, in ID order.
fn upserts<T: VisitModel>(
    committed: &[T],
    next: &[T],
    id: impl Fn(&T) -> &StableId,
    edits: &mut Vec<SessionEdit>,
    edit: impl Fn(&T) -> SessionEdit,
) {
    for entity in next {
        match find(committed, id(entity), &id) {
            Some(old) if same_bits(old, entity) => {}
            _ => edits.push(edit(entity)),
        }
    }
}

/// The entity of a list sorted by ID that has `wanted` as its ID.
fn find<'a, T>(list: &'a [T], wanted: &StableId, id: &impl Fn(&T) -> &StableId) -> Option<&'a T> {
    list.binary_search_by(|entity| id(entity).cmp(wanted))
        .ok()
        .map(|index| &list[index])
}

/// Whether two smoothing declarations hold the same arm and the same length bits.
fn same_smoothing(left: Option<ControlSmoothing>, right: Option<ControlSmoothing>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(left), Some(right)) => {
            left.fader_ms.to_bits() == right.fader_ms.to_bits()
                && left.mute_ms.to_bits() == right.mute_ms.to_bits()
                && left.pan_ms.to_bits() == right.pan_ms.to_bits()
        }
        _ => false,
    }
}

/// Whether two values walk to the same bits: every scalar, key and length of the session's own
/// model walk, a float by its bit pattern (so a zero's sign counts).
fn same_bits<T: VisitModel>(left: &T, right: &T) -> bool {
    bits(left) == bits(right)
}

fn bits<T: VisitModel>(value: &T) -> Vec<u8> {
    let mut sink = Bits(Vec::new());
    match value.visit(WalkOrder::Declared, &mut sink) {
        Ok(()) => sink.0,
        Err(never) => match never {},
    }
}

/// A model walk written as a self-delimiting byte stream: a tag per event, the key's ID, and the
/// value's bits (a length before each variable-length value).
struct Bits(Vec<u8>);

impl Bits {
    fn tag(&mut self, tag: u8, key: Option<FieldKey>) {
        self.0.push(tag);
        self.0
            .extend_from_slice(&key.map_or(u16::MAX, |key| key.id).to_le_bytes());
    }
    fn bytes(&mut self, value: &[u8]) {
        self.0
            .extend_from_slice(&(value.len() as u64).to_le_bytes());
        self.0.extend_from_slice(value);
    }
}

impl ModelVisitor for Bits {
    type Error = Infallible;
    fn record_begin(&mut self, key: Option<FieldKey>, wire_fields: u32) -> Result<(), Infallible> {
        self.tag(1, key);
        self.0.extend_from_slice(&wire_fields.to_le_bytes());
        Ok(())
    }
    fn record_schema(
        &mut self,
        module: &'static str,
        arm: Option<&'static str>,
    ) -> Result<(), Infallible> {
        self.tag(2, None);
        self.bytes(module.as_bytes());
        self.bytes(arm.unwrap_or("").as_bytes());
        Ok(())
    }
    fn record_end(&mut self) -> Result<(), Infallible> {
        self.tag(3, None);
        Ok(())
    }
    fn array_begin(&mut self, key: FieldKey, len: usize) -> Result<(), Infallible> {
        self.tag(4, Some(key));
        self.0.extend_from_slice(&(len as u64).to_le_bytes());
        Ok(())
    }
    fn array_end(&mut self) -> Result<(), Infallible> {
        self.tag(5, None);
        Ok(())
    }
    fn wire_tag(&mut self, tag: Token) -> Result<(), Infallible> {
        self.tag(6, None);
        self.0.push(tag.wire);
        Ok(())
    }
    fn bool(&mut self, key: FieldKey, value: bool) -> Result<(), Infallible> {
        self.tag(7, Some(key));
        self.0.push(u8::from(value));
        Ok(())
    }
    fn u8(&mut self, key: FieldKey, value: u8) -> Result<(), Infallible> {
        self.tag(8, Some(key));
        self.0.push(value);
        Ok(())
    }
    fn u32(&mut self, key: FieldKey, value: u32) -> Result<(), Infallible> {
        self.tag(9, Some(key));
        self.0.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }
    fn u64(&mut self, key: FieldKey, value: u64) -> Result<(), Infallible> {
        self.tag(10, Some(key));
        self.0.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }
    fn source_bit_depth(&mut self, key: FieldKey, value: SourceBitDepth) -> Result<(), Infallible> {
        self.tag(11, Some(key));
        self.bytes(format!("{value:?}").as_bytes());
        Ok(())
    }
    fn f32(&mut self, key: FieldKey, value: f32) -> Result<(), Infallible> {
        self.tag(12, Some(key));
        self.0.extend_from_slice(&value.to_bits().to_le_bytes());
        Ok(())
    }
    fn id(&mut self, key: FieldKey, value: &StableId) -> Result<(), Infallible> {
        self.tag(13, Some(key));
        self.bytes(value.as_str().as_bytes());
        Ok(())
    }
    fn id_item(&mut self, value: &StableId) -> Result<(), Infallible> {
        self.tag(14, None);
        self.bytes(value.as_str().as_bytes());
        Ok(())
    }
    fn text(&mut self, key: FieldKey, value: &str) -> Result<(), Infallible> {
        self.tag(15, Some(key));
        self.bytes(value.as_bytes());
        Ok(())
    }
    fn token(&mut self, key: FieldKey, value: Token) -> Result<(), Infallible> {
        self.tag(16, Some(key));
        self.0.push(value.wire);
        self.bytes(value.text.as_bytes());
        Ok(())
    }
}
