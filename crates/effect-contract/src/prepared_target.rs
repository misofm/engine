//! Fixed-size prepared native-effect target handoff types.
//!
//! A target carries the semantic words that were accepted and the designed words that a
//! prepared effect will apply at a block boundary. The representation is intentionally small,
//! copyable, and independent of any wire or allocator format.
//! Designing targets belongs to the control, main, or worker plane. Bounded validation is safe on
//! an admission path, while render-side application is exposed by the defaulted hooks on the
//! prepared effect traits.

use crate::{InitialParameterValue, ParameterChannel};

/// Number of `u32` words in one prepared effect target.
pub const PREPARED_EFFECT_TARGET_WORDS: usize = 12;

/// One validated target for one logical effect section and channel.
///
/// The `slot` value is owned by the effect factory. For parametric EQ it is the physical section
/// order, while other effects may assign their own stable logical sections. `words` contains the
/// effect's validated semantic configuration followed by its designed numeric words. This is an
/// internal Rust handoff and deliberately carries no wire representation or ownership handle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreparedEffectTarget {
    /// Effect-owned logical section identifier.
    pub slot: u32,
    /// Channel selected by the target.
    pub channel: ParameterChannel,
    /// Validated semantic and designed target words.
    pub words: [u32; PREPARED_EFFECT_TARGET_WORDS],
}

/// Complete candidate configuration and touched-entry mask for target preparation.
///
/// `values` is in descriptor order and contains the complete final candidate. `changed` has one
/// entry for every value and marks the entries touched by the current control batch. The slices
/// are borrowed only for the duration of preparation; no target retains either slice.
#[derive(Clone, Copy, Debug)]
pub struct EffectTargetRequest<'a> {
    /// Explicit sample rate used by the effect's coefficient designer.
    pub sample_rate: u32,
    /// Complete final candidate in descriptor order.
    pub values: &'a [InitialParameterValue],
    /// One touched bit for each entry in [`Self::values`].
    pub changed: &'a [bool],
}

/// Why prepared-target preparation or application was rejected.
///
/// The variants intentionally stay small so host-facing refusal mapping can reuse the existing
/// diagnostic vocabulary without introducing a second public error hierarchy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectTargetError {
    /// The effect does not implement prepared-target preparation or application.
    Unsupported,
    /// A request, target count, or target shape is malformed.
    Shape,
    /// A semantic value or selector is outside the effect's declared domain.
    Domain,
    /// Designed coefficient words are non-finite, unstable, or otherwise unsafe.
    Coefficients,
    /// The caller-provided target storage cannot hold the requested output.
    Capacity,
}

/// Optional control-plane capability for preparing fixed-size effect targets.
pub trait NativeEffectTargetPreparation: Send + Sync {
    /// Maximum number of targets this capability can produce for one complete configuration.
    fn maximum_targets(&self) -> usize;

    /// Design targets from a complete final candidate on the control or worker plane.
    ///
    /// Implementations may invoke their effect designer here. They must validate request shape,
    /// semantic domains, and output capacity before producing a successful count. Every error
    /// leaves `out` unchanged and returns no successful count. Render-side code never calls this method.
    fn prepare_targets(
        &self,
        request: EffectTargetRequest<'_>,
        out: &mut [PreparedEffectTarget],
    ) -> Result<usize, EffectTargetError>;

    /// Validate already-prepared targets without designing or mutating effect state.
    ///
    /// Implementations validate request shape, expected semantic words, selector coverage, and
    /// numerical safety. This operation performs no design, allocation, or mutation and is
    /// bounded by the supplied slices, so it may run during admission. A rejection leaves the
    /// request and target storage unchanged.
    fn validate_targets(
        &self,
        request: EffectTargetRequest<'_>,
        targets: &[PreparedEffectTarget],
    ) -> Result<(), EffectTargetError>;
}

#[cfg(test)]
mod tests {
    use super::{
        EffectTargetError, EffectTargetRequest, NativeEffectTargetPreparation,
        PREPARED_EFFECT_TARGET_WORDS, PreparedEffectTarget,
    };
    use core::mem::size_of;

    fn assert_copy<T: Copy>() {}

    #[test]
    fn prepared_target_has_frozen_copy_shape() {
        assert_eq!(PREPARED_EFFECT_TARGET_WORDS, 12);
        assert_eq!(size_of::<PreparedEffectTarget>(), 56);
        assert!(size_of::<crate::EffectControlRecord>() <= 64);
        assert_copy::<PreparedEffectTarget>();
        assert_copy::<EffectTargetRequest<'static>>();
        assert_copy::<EffectTargetError>();
        assert!(size_of::<PreparedEffectTarget>() <= 64);
    }

    #[test]
    fn target_preparation_trait_is_object_safe() {
        fn assert_object_safe(_: &dyn NativeEffectTargetPreparation) {}
        let _ = assert_object_safe;
    }
}
