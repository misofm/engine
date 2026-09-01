//! Effect-contract conformance (issue 011 harness) against the production factory.
//!
//! A 1.5 MB delay line at 96 kHz -- the effect whose declared state the harness's old hard-coded 1 MiB limit rejected outright.
//!
//! Issue #105 phase 2 F1: the harness runs against every production `NativeEffectFactory`, not
//! just its own reference mock. The whole test is the macro -- see
//! `conformance::effect_conformance_test!` for what it gates and why the two
//! dev-dependencies are load-bearing.
conformance::effect_conformance_test!(delay::DelayFactory);
