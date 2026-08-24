//! Target-specific worker ownership.
//!
//! The native module owns the persistent pool, its lease and the wake protocol. The browser
//! module owns nothing: Wasm renders the exact same parcels sequentially, and its lease type
//! exists only so the block API has one signature on every target.

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use native::retained_queue_bytes;
#[cfg(not(target_arch = "wasm32"))]
pub use native::{NativeWorkerPoolV1, WorkerLeaseV1};

#[cfg(target_arch = "wasm32")]
mod browser;
#[cfg(target_arch = "wasm32")]
pub(crate) use browser::retained_queue_bytes;
#[cfg(target_arch = "wasm32")]
pub use browser::{NativeWorkerPoolV1, WorkerLeaseV1};
