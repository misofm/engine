//! Transactional, non-publishable session compilation.

use std::collections::BTreeMap;

use miso_engine_core::{QuantumFrames, SampleRateHz};

use crate::{
    Diagnostic, DiagnosticCode, DiagnosticPath, DiagnosticSet, ResourceEstimate, SessionToml,
    StableId,
    canonical::write_canonical,
    estimate::{estimate_session, with_canonical_bytes},
    validate::validate_session,
};

/// Explicit compiler resource budgets. There is deliberately no default and no track-count cap.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompileCaps {
    /// Maximum retained model, indexes, and canonical snapshot bytes.
    pub max_compiled_model_bytes: u64,
    /// Maximum total declarative runtime bytes.
    pub max_requested_runtime_bytes: u64,
    /// Maximum individual model/runtime allocation.
    pub max_single_allocation_bytes: u64,
    /// Maximum declarative control queue items.
    pub max_queue_items: u64,
    /// Maximum total source-ring frames.
    pub max_source_ring_frames: u64,
    /// Maximum total source-ring bytes.
    pub max_source_ring_bytes: u64,
}

/// Immutable control-plane artifact for downstream compiler issues.
///
/// This is intentionally non-publishable: it has no prepared graph, realtime storage, plan
/// publisher, DSP state, or plan exchange capability.
#[derive(Clone, Debug)]
pub struct CompiledSession {
    normalized: NormalizedSession,
    canonical_toml: String,
    sample_rate: SampleRateHz,
    quantum: QuantumFrames,
    output_shape: OutputShape,
    source_indexes: BTreeMap<StableId, u64>,
    graph_entity_indexes: BTreeMap<StableId, u64>,
    resource_estimate: ResourceEstimate,
}
#[derive(Clone, Debug)]
struct NormalizedSession(SessionToml);

/// Planar PCM output shape retained by a compiled session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OutputShape {
    /// Number of planar `f32` channels. V1 is exactly dual-mono/two-channel.
    pub channels: u8,
}

impl CompiledSession {
    /// Borrow the canonical TOML snapshot.
    #[must_use]
    pub fn canonical_toml(&self) -> &str {
        &self.canonical_toml
    }
    /// Explicit session sample rate in hertz.
    #[must_use]
    pub const fn sample_rate(&self) -> SampleRateHz {
        self.sample_rate
    }
    /// Explicit render quantum in sample frames.
    #[must_use]
    pub const fn quantum(&self) -> QuantumFrames {
        self.quantum
    }
    /// Declared planar PCM output channel count.
    #[must_use]
    pub const fn output_shape(&self) -> OutputShape {
        self.output_shape
    }
    /// Stable normalized source position, if declared.
    #[must_use]
    pub fn source_index(&self, id: &StableId) -> Option<u64> {
        self.source_indexes.get(id).copied()
    }
    /// Stable normalized graph-entity position, if declared.
    #[must_use]
    pub fn graph_entity_index(&self, id: &StableId) -> Option<u64> {
        self.graph_entity_indexes.get(id).copied()
    }
    /// Checked, duration-independent resource estimate.
    #[must_use]
    pub const fn resource_estimate(&self) -> ResourceEstimate {
        self.resource_estimate
    }
    /// Number of normalized source declarations.
    #[must_use]
    pub fn source_count(&self) -> usize {
        self.normalized.0.sources.len()
    }
    /// Read-only normalized declaration retained by this non-publishable artifact.
    #[must_use]
    pub fn normalized_model(&self) -> &SessionToml {
        &self.normalized.0
    }
}
/// Validate and normalize a complete session into an immutable non-realtime artifact.
///
/// The checked preflight and all cap comparisons occur before canonical-string allocation,
/// model cloning, sorting, or index-map construction. No plan is prepared or published.
pub fn compile_session(
    session: &SessionToml,
    caps: CompileCaps,
) -> Result<CompiledSession, DiagnosticSet> {
    let estimate = estimate_session(session)?;
    check_caps(session, estimate, caps)?;
    validate_session(session)?;
    let canonical_toml = write_canonical(session);
    let estimate = with_canonical_bytes(estimate, canonical_toml.len())?;
    debug_assert!(estimate.compiled_model_bytes <= caps.max_compiled_model_bytes);
    let mut normalized = session.clone();
    normalized
        .sources
        .sort_by(|left, right| left.id.cmp(&right.id));
    normalized
        .tracks
        .sort_by(|left, right| left.id.cmp(&right.id));
    normalized
        .submixes
        .sort_by(|left, right| left.id.cmp(&right.id));
    normalized
        .outputs
        .sort_by(|left, right| left.id.cmp(&right.id));
    normalized
        .routes
        .sort_by(|left, right| left.id.cmp(&right.id));
    normalized
        .automation
        .sort_by(|left, right| left.id.cmp(&right.id));
    for track in &mut normalized.tracks {
        for rack in [&mut track.simd1, &mut track.dynamic, &mut track.simd2] {
            for effect in &mut rack.effects {
                effect.params.sort_by(|left, right| {
                    left.parameter_id
                        .cmp(&right.parameter_id)
                        .then(left.channel.cmp(&right.channel))
                });
            }
        }
    }
    let source_indexes = indexed(
        normalized.sources.iter().map(|item| &item.id),
        "source_indexes",
    )?;
    let graph_entity_indexes = indexed(
        normalized
            .tracks
            .iter()
            .map(|item| &item.id)
            .chain(normalized.submixes.iter().map(|item| &item.id))
            .chain(normalized.outputs.iter().map(|item| &item.id)),
        "graph_entity_indexes",
    )?;
    Ok(CompiledSession {
        normalized: NormalizedSession(normalized),
        canonical_toml,
        sample_rate: SampleRateHz(session.sample_rate_hz),
        quantum: QuantumFrames(session.quantum_frames),
        output_shape: OutputShape {
            channels: session.output_profile.channels,
        },
        source_indexes,
        graph_entity_indexes,
        resource_estimate: estimate,
    })
}
fn check_caps(
    session: &SessionToml,
    estimate: ResourceEstimate,
    caps: CompileCaps,
) -> Result<(), DiagnosticSet> {
    let mut diagnostics = Vec::new();
    for (value, limit, parent, leaf, message) in [
        (
            estimate.compiled_model_bytes,
            caps.max_compiled_model_bytes,
            "compile_caps",
            "max_compiled_model_bytes",
            "compiled-model byte budget exceeded",
        ),
        (
            estimate.requested_runtime_bytes,
            session.limits.memory_bytes,
            "limits",
            "memory_bytes",
            "session memory declaration is insufficient",
        ),
        (
            estimate.requested_runtime_bytes,
            caps.max_requested_runtime_bytes,
            "compile_caps",
            "max_requested_runtime_bytes",
            "requested runtime byte budget exceeded",
        ),
        (
            estimate.single_allocation_bytes,
            caps.max_single_allocation_bytes,
            "compile_caps",
            "max_single_allocation_bytes",
            "single-allocation budget exceeded",
        ),
        (
            estimate.queue_items,
            caps.max_queue_items,
            "compile_caps",
            "max_queue_items",
            "queue item budget exceeded",
        ),
        (
            estimate.source_ring_frames,
            caps.max_source_ring_frames,
            "compile_caps",
            "max_source_ring_frames",
            "source-ring frame budget exceeded",
        ),
        (
            estimate.source_ring_bytes,
            caps.max_source_ring_bytes,
            "compile_caps",
            "max_source_ring_bytes",
            "source-ring byte budget exceeded",
        ),
    ] {
        if value > limit {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::ResourceLimitExceeded,
                DiagnosticPath::root().key(parent).key(leaf),
                None,
                message,
            ));
        }
    }
    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(DiagnosticSet::from_vec(diagnostics))
    }
}
fn indexed<'a>(
    ids: impl Iterator<Item = &'a StableId>,
    path: &str,
) -> Result<BTreeMap<StableId, u64>, DiagnosticSet> {
    ids.enumerate()
        .map(|(index, id)| {
            u64::try_from(index)
                .map(|index| (id.clone(), index))
                .map_err(|_| {
                    DiagnosticSet::from_vec(vec![Diagnostic::new(
                        DiagnosticCode::CapacityArithmeticOverflow,
                        DiagnosticPath::root().key(path),
                        None,
                        "index cannot convert to u64",
                    )])
                })
        })
        .collect()
}
