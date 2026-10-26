//! Effect-contract conformance against the production factory, including zero allocations in
//! `process` under `bench_support`'s audited allocator.
conformance::effect_conformance_test!(reverb::ReverbFactory);
