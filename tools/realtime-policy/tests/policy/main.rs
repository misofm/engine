//! Cases for the realtime policy tool.
//!
//! Every case materializes the base tree (`tests/fixtures/base`, which sits exactly on the case
//! policy's floors), applies a few edits, and asserts the exact set of finding classes.

mod binary;
mod boundary;
mod calls;
mod common;
mod doctests;
mod drain;
mod forbidden;
mod markers;
mod render_wrap;
mod run;
mod source_reads;
mod unmarked;
mod unsafe_ownership;
mod valid;
mod walk;
mod web_host;
mod whole_plan;
