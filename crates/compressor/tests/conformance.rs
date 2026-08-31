//! Effect-contract conformance (issue 011 harness) against the production factory.
//!
//! 882 samples of lookahead at 44.1 kHz, a linked detector and a lookahead ring whose write index advances on silence.
//!
//! Issue #105 phase 2 F1: the harness runs against every production `NativeEffectFactory`, not
//! just its own reference mock. The whole test is the macro -- see
//! `conformance::effect_conformance_test!` for what it gates and why the two
//! dev-dependencies are load-bearing.
conformance::effect_conformance_test!(compressor::CompressorFactory);
