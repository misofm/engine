//! Effect-contract conformance against the production factory at the maximum band count (the
//! defaults enable all four bands): descriptor validation, allocation-free render, reset, state
//! round trip, partition invariance and lane isolation, at every launch rate, link mode and bypass.
conformance::effect_conformance_test!(dynamic_eq::DynamicEqFactory);
