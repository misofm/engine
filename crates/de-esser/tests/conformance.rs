//! Effect-contract conformance against the production factory: descriptor validation,
//! allocation-free render, reset, state round trip, partition invariance and lane isolation, at
//! every launch rate, link mode and bypass.
conformance::effect_conformance_test!(de_esser::DeEsserFactory);
