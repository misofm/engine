//! The one harness the `tools/` benchmarks and audits share.
//!
//! Audit #104 counted, at `ae02d2a`, fourteen copies of the audited `GlobalAlloc` wrapper in three
//! behavioural variants, nine hand-rolled JSON string escapers (two of which emitted invalid JSON),
//! and eight nearest-rank percentile functions with three different edge behaviours -- one per
//! historical issue, each with its own unit tests. Finding F4 states the rule this crate exists to
//! enforce: *the second copy is the defect*. `scripts/check-bench-policy.sh` fails if a second copy
//! of any of them reappears under `tools/`.
//!
//! This crate is test scaffolding. Packages under `crates/` and `hosts/` may use it only as a
//! dev-dependency, and
//! `src/alloc.rs` is one of three `tools/` files that `tools/realtime-policy`'s `UNSAFE_ALLOWLIST`
//! lets contain `unsafe` code; it also holds `allow(unsafe_code)`.

pub mod alloc;
pub mod digest;
pub mod json;
pub mod metadata;
pub mod producer;
pub mod stats;
pub mod sysinfo;
pub mod timing;
