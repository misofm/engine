//! Effect-contract conformance (issue 011 harness) against the production factory.
//!
//! Two differently-timed envelope followers per lane.
//!
//! Issue #105 phase 2 F1: the harness runs against every production `NativeEffectFactory`, not
//! just its own reference mock. The whole test is the macro -- see
//! `miso_engine_conformance::effect_conformance_test!` for what it gates and why the two
//! dev-dependencies are load-bearing.
miso_engine_conformance::effect_conformance_test!(
    miso_engine_transient_shaper::TransientShaperFactory
);
