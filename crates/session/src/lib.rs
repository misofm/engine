//! Strict, versioned canonical JSON session declarations and a control-plane-only compiler boundary.
//!
//! This crate deliberately does **not** prepare, publish, or otherwise own a realtime render
//! plan.  It accepts complete declarative input, validates only the semantics owned by issue 004,
//! and returns an immutable non-publishable compilation artifact for later compiler issues. The
//! [`VisitModel`] API exposes one schema-keyed emit-side walk for canonical and wire consumers.

mod address;
mod canonical;
mod compile;
mod diagnostic;
mod edit_ramp;
mod estimate;
mod id;
mod in_place;
mod json_preflight;
mod model;
mod parse;
#[cfg(feature = "test-support")]
pub mod test_support;
mod validate;
mod value;
mod vca;
mod visit;

pub use address::{ControlAddress, Lane, Owner, OwnerKind, Section, SectionKind};
pub use canonical::canonical_session_json;
pub use compile::{CompileCaps, CompiledSession, OutputShape, compile_session};
pub use diagnostic::{
    Diagnostic, DiagnosticCode, DiagnosticPath, DiagnosticSet, PathSegment, SourceSpan,
};
pub use edit_ramp::{EditRampRow, EditRampTarget, EditRamps};
pub use estimate::{ResourceEstimate, estimate_session_resources};
pub use id::{STABLE_ID_MAXIMUM_BYTES, StableId};
pub use in_place::{
    OwnerRef, OwnerSlots, OwnersGrowth, canonical_owner_bytes, owners_growth, owners_values_valid,
};
pub use model::*;
pub use parse::parse_session_json;
pub use vca::{
    FADER_MAXIMUM_DB, FADER_MINIMUM_DB, VcaNesting, VcaSum, vca_compose_db, vca_effective_db,
    vca_offsets_sum,
};
pub use visit::{FieldKey, ModelVisitor, Token, VARIANT_ARMS, VisitModel, WalkOrder, keys};

/// The only schema version accepted by [`parse_session_json`].
pub const SESSION_SCHEMA_VERSION_V1: u32 = 1;

/// The largest session `revision` the schema accepts: the largest revision a plan's revision
/// gate can hold (bit 63 of the gate word is render's announcement, #1432 D1).  `validate_session`
/// refuses a larger revision, so every session entry point shares this one domain.
pub const MAX_SESSION_REVISION: u64 = engine::realtime::GATE_REVISION_MAX;

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "json-syntax 0.12.5 empty-object CodeMap defect; prepared patch tracked at https://github.com/misofm/engine/issues/391; rerun on dependency updates"]
    fn json_syntax_empty_object_code_map_regression() {
        use json_syntax::Parse as _;

        let (_value, code_map) =
            json_syntax::Value::parse_str(r#"{"a":{},"b":1}"#).expect("JSON parses");

        assert_eq!(code_map[3].volume, 1);
        assert_eq!((code_map[3].span.start(), code_map[3].span.end()), (5, 7));
    }
}
