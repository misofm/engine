//! Off-render native effect session preparation.
#![allow(missing_docs)]
mod control;
mod diagnostic;
mod migration;
mod prepare;
pub use control::{
    EffectControlOwner, EffectControlOwnerError, EffectControlOwnerPhase,
    EffectControlResourceError,
};
pub use diagnostic::*;
pub use migration::*;
/// Re-export the EQ's copied-word evaluator so host composition uses the exact owner analysis
/// implementation without adding a second host-side DSP implementation.
pub use parametric_eq::{
    EqResponseError, query_snapshot_magnitudes_into as query_parametric_eq_snapshot_magnitudes_into,
};
pub use prepare::*;

/// Returns the native EQ factory only when its prepared-target capability is actually enabled.
///
/// This narrow probe is used by the host preparation facade; it deliberately does not construct
/// the effect registry or the rest of the native roster.  The launch factory remains unsupported
/// until the target-preparation cutover assignment.
pub fn parametric_eq_target_preparation_factory()
-> Option<std::sync::Arc<dyn effect_contract::NativeEffectFactory>> {
    let factory: std::sync::Arc<dyn effect_contract::NativeEffectFactory> =
        std::sync::Arc::new(parametric_eq::ParametricEqFactory);
    if factory.target_preparation().is_some() {
        Some(factory)
    } else {
        None
    }
}
