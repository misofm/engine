//! Shared effect scaffolding: the code every native effect needs, written once.
//!
//! The #83 audit found seven near-identical copies of the same five things across the effect
//! crates — a state-payload codec, a parameter ramp, an envelope follower, a dynamics gain
//! computer and a homogeneous-bank driver — that had drifted apart in their details. This crate is
//! the single home for all of them (master plan for issue #83, §6). It has no consumer in wave 1;
//! wave 2 moves each effect onto it and deletes its copy.
//!
//! # What is pinned here
//!
//! * [`ramp`] — decision D11: a linear ramp precomputes its per-sample increment with **one
//!   division at event time** and snaps to the target on the final sample. No per-sample division
//!   exists anywhere in the engine.
//! * [`envelope`] — the peak follower is the one-rounding
//!   `y = max(|x|, fma(c, y - |x|, |x|))` form, and `max` is the D8 select form, never
//!   `f32::max`.
//! * [`dynamics`] — the static curve is Giannoulis, Massberg and Reiss (JAES 2012) equation 4,
//!   evaluated branchlessly in the dB domain through the lane-wide `exp2`/`log2` of
//!   `miso-engine-math`.
//! * [`bank`] — decision D7 and master plan §4.4: output finiteness is checked **once per block
//!   per bank** with a vector compare, never per value. A failing block zeroes its output, resets
//!   the bank's state and increments a counter.
//! * [`state_payload`] — one versioned little-endian word codec for every effect's snapshot and
//!   restore, with the diagnostic codes the effects already use.
//! * [`params`] — domain validation and clamping driven by a descriptor, in the D8 select form.
//!
//! # Realtime rules
//!
//! Every lane-generic function is `#[inline(always)]`, allocation-free and branch-free per sample.
//! Allocation happens at prepare time only, in [`bank::HomogeneousBank::prepare`]; nothing on a
//! per-block path allocates, locks or calls the platform libm. Transcendentals come from
//! `miso-engine-math` (D6), fusion only from `miso_engine_lane::Lane::fma` (D3).
//!
//! # Width independence
//!
//! Everything generic over [`Lane`](miso_engine_lane::Lane) has one body, instantiated at
//! `WIDTH = 1` (the scalar oracle), 4 and 8. `tests/lane_identity.rs` proves by `to_bits` that the
//! three agree for every function in this crate, so lane identity is a property of the code rather
//! than of a fixture corpus.

#![no_std]

extern crate alloc;

pub mod bank;
pub mod dynamics;
pub mod envelope;
pub mod params;
pub mod ramp;
pub mod state_payload;

pub mod corpus;
