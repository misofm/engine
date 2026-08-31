//! Effect-contract conformance (issue 011 harness) against the production factory.
//!
//! Zero declared latency, the largest parameter table, and a per-lane payload carrying the runtime codec's two-word header.
//!
//! Issue #105 phase 2 F1: the harness runs against every production `NativeEffectFactory`, not
//! just its own reference mock. The whole test is the macro -- see
//! `conformance::effect_conformance_test!` for what it gates and why the two
//! dev-dependencies are load-bearing.
conformance::effect_conformance_test!(parametric_eq::ParametricEqFactory);
