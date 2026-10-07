# Wire the engine and session design-bound caches into the C ABI

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-07 by the decision-15 root coordinator's split of *Cache design bounds across
preparations within a stated preparation budget* (#1457, Amendment 2). Ordered after #1457 in
stream G and after stream B's *Extract the C ABI control plane into a portable crate both hosts
call* (#1309, batch 1) is on `main`. Built on the `control-plane` crate that #1309 creates. Verify
every code anchor on the branch where both have landed before starting.

## Product outcome

A C ABI engine computes each input-section design bound at most once while its cache holds it:
`miso_engine_v1_compile_session` reads and fills the engine's cache, and every session keeps its
own cache for its structural rebuilds, so a rebuild of an unchanged session computes no design
bound. The prepared plans, the reports and every rendered bit are the same as without the caches.

## Context

- **What #1457 ships.** In `builtins`: `InputBoundCache` (keyed by `InputBoundKey`, which carries the
  rate; entry cap `INPUT_BOUND_CACHE_ENTRIES` = 8,192, cleared when full), the per-preparation
  budget `INPUT_BOUND_BUDGET_FRAMES` (1,510,000 frames walked, charged in strip order; past it a
  design reports the rate's live bound), and `input_section_bounds(rate, strips, cache)`. A cached
  design charges the budget its stored frames, so every prepared value is a pure function of the
  session, with or without a cache, cold or warm. `builtins-compiler`'s
  `prepare_session_builtins_with_live_controls` takes `bound_cache: Option<&mut InputBoundCache>`.
  In `host-core`, `prepare.rs`'s internal policy function
  (`prepare_host_runtime_with_live_controls_policy_and_spectrum`) takes the same parameter; no
  public host-core entry point passes a cache yet.
- **Root ruling (b), 2026-10-07 (#1457 Amendment 2).**
  - The engine's cache serves `miso_engine_v1_compile_session`.
  - Each session owns its own cache, seeded at compile (a copy of the engine's cache after that
    compile), for its rebuilds. No mutable state is shared between the engine and its sessions,
    and the header's thread-ownership notes do not change.
  - No parallel `_and_bound_cache` public variant at any layer: this slice adds the cache parameter
    to the existing host-core entry points the C ABI uses (`prepare_host_runtime_with_live_lanes`
    and `prepare_host_runtime_with_live_lanes_successor`), once, and updates their callers.
- **Where the C ABI prepares.** `compile_children` and `prepare_runtime` (in
  `crates/capi/src/runtime/compile.rs` before #1309; in `control-plane` after it), called from
  `miso_engine_v1_compile_session` (`crates/capi/src/ffi.rs`, which holds the `Engine`), and the
  structural-transaction rebuild (`prepare_runtime(.., Some(SuccessorBase))` in the session's
  controller, `control.rs`), which runs on the session handle.

## Decisions frozen for this slice

- **D1.** `Engine` (`crates/capi/src/abi.rs`) owns one `InputBoundCache`; a compile reads and
  fills it. Each session's control-plane state owns one `InputBoundCache`, seeded at compile with
  a copy of the engine's cache, and every rebuild of that session reads and fills it. Both live on
  control threads only; render never reads either, and render-owned memory carries none of it.
- **D2.** `prepare_host_runtime_with_live_lanes` and `prepare_host_runtime_with_live_lanes_successor`
  gain `bound_cache: Option<&mut InputBoundCache>`, passed through to the policy function. Callers
  without a cache pass `None`.

## Authorized paths (stream B's and F's, named exceptions)

- `crates/capi/src/abi.rs` (the `Engine` field), `crates/capi/src/ffi.rs` (the compile call).
- `crates/control-plane/src/` (the compile path, the session state and its rebuild call), as #1309
  leaves it.
- `crates/host-core/src/prepare.rs` (the two entry points) and every caller of those two entry
  points (`crates/capi`, `crates/control-plane` and their tests, `crates/host-core/tests`), each
  edit only adding the argument.
- A new test file under `crates/capi/tests/` or `crates/control-plane/tests/` (stream F's column)
  for gates 1 and 4.

## Objective gates

1. A rebuild of an unchanged session through the C ABI computes no design bound and walks no frame
   (counted with `builtins::test_support::fixed_input_bounds_computed` and
   `fixed_input_frames_walked`); a second `miso_engine_v1_compile_session` of the same document on
   the same engine likewise computes none.
2. Every prepared value and the plan resource report are identical with and without the caches,
   cold and warm, including a session past the budget (#1457 gates 3 and 5 stay green).
3. `target/release/audit capi` stays at 0 allocations and 0 syscalls on render; the PCM digest does
   not move; `scripts/check-capi-abi.sh` passes with no header change.
4. The native rebuild cost: the C ABI preparation of the 9-track and 64-track documents is recorded
   on a first compile and on a rebuild of the unchanged session (release, one invocation, one warmup,
   two measured rounds); the design-bound work is within #1457's budget on the first, and zero on
   the rebuild (gate 1).

## Non-goals

- The browser cache (#1470). The budget, the live-bound table and the cache type (#1457). Any
  render-side change.

## Dependencies

- #1457 (stream G), #1309 (stream B, batch 1, on `main`).

## Attempt record
