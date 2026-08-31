//! Effect-contract conformance (issue 011 harness) against the production factory.
//!
//! A linear-phase halfband whose impulse response starts at sample 0 while its declared latency is 31 -- the effect the deleted first-non-zero-sample latency heuristic rejected.
//!
//! Issue #105 phase 2 F1: the harness runs against every production `NativeEffectFactory`, not
//! just its own reference mock. The whole test is the macro -- see
//! `conformance::effect_conformance_test!` for what it gates and why the two
//! dev-dependencies are load-bearing.
conformance::effect_conformance_test!(soft_clip::SoftClipFactory);
