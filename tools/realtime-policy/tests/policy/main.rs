//! Cases for the realtime policy tool.
//!
//! Every case materializes the base tree (`tests/fixtures/base`, which sits exactly on the case
//! policy's floors), applies a few edits, and asserts the exact set of finding classes.

mod binary;
mod boundary;
mod common;
mod forbidden;
mod markers;
mod run;
mod unsafe_ownership;
mod valid;
mod walk;
mod whole_plan;
