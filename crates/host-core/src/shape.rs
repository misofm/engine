//! Internal self-configuration facts for single-verb boot.
//!
//! Every field is read from the compiled session preparation itself consumes. There is no raw-JSON
//! query entry point and no second parser: hosts call [`compiled_session_shape`] after their one
//! canonical parse/compile pass.

use session::CompiledSession;

use crate::diagnostics::{PrepareDiagnostics, PrepareRejection};
use crate::prepare::count_effects;

/// Address-free facts a host derives from one compiled session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostSessionShape {
    /// Declared schema version.
    pub schema_version: u32,
    /// Caller-controlled document revision.
    pub revision: u64,
    /// Session sample rate in hertz.
    pub sample_rate_hz: u32,
    /// Session render quantum in frames.
    pub quantum_frames: u32,
    /// Declared output channel count.
    pub output_channels: u32,
    /// Largest declared source channel count, or zero when there are no sources.
    pub maximum_source_channels: u32,
    /// Longest declared source ID in bytes, or zero when there are no sources.
    pub longest_source_id_bytes: u64,
    /// Number of sources.
    pub source_count: u64,
    /// Number of tracks.
    pub track_count: u64,
    /// Number of routes.
    pub route_count: u64,
    /// Number of effects across all track racks.
    pub effect_count: u64,
}

/// Read boot self-configuration from an already compiled session.
///
/// # Errors
///
/// Returns the same platform/count diagnostics preparation uses if a count cannot be represented.
pub fn compiled_session_shape(
    compiled: &CompiledSession,
) -> Result<HostSessionShape, PrepareDiagnostics> {
    let model = compiled.normalized_model();
    Ok(HostSessionShape {
        schema_version: model.schema_version,
        revision: model.revision,
        sample_rate_hz: compiled.sample_rate().0,
        quantum_frames: compiled.quantum().0,
        output_channels: u32::from(model.output_profile.channels),
        maximum_source_channels: model
            .sources
            .iter()
            .map(|source| u32::from(source.channels))
            .max()
            .unwrap_or(0),
        longest_source_id_bytes: count(
            model
                .sources
                .iter()
                .map(|source| source.id.as_str().len())
                .max()
                .unwrap_or(0),
        )?,
        source_count: count(model.sources.len())?,
        track_count: count(model.tracks.len())?,
        route_count: count(model.routes.len())?,
        effect_count: count_effects(model)?,
    })
}

fn count(value: usize) -> Result<u64, PrepareDiagnostics> {
    u64::try_from(value)
        .map_err(|_| PrepareDiagnostics::fixed(PrepareRejection::Platform, "host.count"))
}
