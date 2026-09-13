//! Off-render native effect session preparation.
#![allow(missing_docs)]
mod diagnostic;
mod migration;
mod prepare;
pub use diagnostic::*;
pub use migration::*;
/// Re-export the EQ's copied-word evaluator so host composition uses the exact owner analysis
/// implementation without adding a second host-side DSP implementation.
pub use parametric_eq::{
    EqResponseError, query_snapshot_magnitudes_into as query_parametric_eq_snapshot_magnitudes_into,
};
pub use prepare::*;
