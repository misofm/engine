//! The one harness the `tools/` benchmarks and audits share.
//!
//! Audit #104 counted, at `ae02d2a`, fourteen copies of the audited `GlobalAlloc` wrapper in three
//! behavioural variants, nine hand-rolled JSON string escapers (two of which emitted invalid JSON),
//! and eight nearest-rank percentile functions with three different edge behaviours -- one per
//! historical issue, each with its own unit tests. Finding F4 states the rule this crate exists to
//! enforce: *the second copy is the defect*. `scripts/check-bench-policy.sh` fails if a second copy
//! of any of them reappears under `tools/`.
//!
//! This crate is test scaffolding. No package under `crates/` or `hosts/` may depend on it, and
//! `scripts/check-realtime-policy.sh` names `src/alloc.rs` as the single file under `tools/`
//! allowed to contain `allow(unsafe_code)`.

pub mod alloc;
pub mod digest;
pub mod json;
pub mod metadata;
pub mod stats;
pub mod timing;
