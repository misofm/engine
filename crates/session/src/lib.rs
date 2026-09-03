//! Strict, versioned canonical JSON session declarations and a control-plane-only compiler boundary.
//!
//! This crate deliberately does **not** prepare, publish, or otherwise own a realtime render
//! plan.  It accepts complete declarative input, validates only the semantics owned by issue 004,
//! and returns an immutable non-publishable compilation artifact for later compiler issues. The
//! [`VisitModel`] API exposes one schema-keyed emit-side walk for canonical and wire consumers.

mod canonical;
mod compile;
mod diagnostic;
mod estimate;
mod id;
mod json_preflight;
mod model;
mod parse;
mod validate;
mod value;
mod visit;

pub use canonical::canonical_session_json;
pub use compile::{CompileCaps, CompiledSession, OutputShape, compile_session};
pub use diagnostic::{
    Diagnostic, DiagnosticCode, DiagnosticPath, DiagnosticSet, PathSegment, SourceSpan,
};
pub use estimate::{ResourceEstimate, estimate_session_resources};
pub use id::StableId;
pub use model::*;
pub use parse::parse_session_json;
pub use validate::{BUILTIN_AUTOMATION_EFFECT_ID, BUILTIN_AUTOMATION_TARGETS};
pub use visit::{FieldKey, ModelVisitor, Token, VisitModel, WalkOrder, keys};

/// The only schema version accepted by [`parse_session_json`].
pub const SESSION_SCHEMA_VERSION_V1: u32 = 1;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

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
    fn canonical_round_trip_is_byte_stable() {
        let session = parse_session_json(EXAMPLE).expect("fixture parses");
        let first = canonical_session_json(&session).expect("canonical");
        assert_eq!(
            EXAMPLE, first,
            "checked-in fixture must already be canonical"
        );
        let reparsed = parse_session_json(&first).expect("canonical reparses");
        assert_eq!(first, canonical_session_json(&reparsed).expect("stable"));
        assert!(first.ends_with('\n'));
    }

    #[test]
    fn compile_is_transactional_and_non_publishable() {
        let mut session = parse_session_json(EXAMPLE).expect("fixture parses");
        session.routes[0].destination = RouteDestination::OutputInput {
            output_id: StableId::parse("missing").expect("stable"),
        };
        assert!(compile_session(&session, caps()).is_err());
    }
}
