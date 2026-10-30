# C ABI: a stopped host cannot refill a full ring before restart after a seek (residual of #2205)

GitHub #2213. Successor of #2205 (root ruling on its attempt-2 verify, 2026-10-30).

## Problem

#2205 adds `miso_engine_v1_session_seek_at_once`: a stopped mobile host that seeks lands the target
at the first block after restart and never plays the stop position. But a host that filled its
source rings before the stop cannot submit the new generation while stopped. The engine ring is
full of the old generation, and only the consumer discards old PCM. The consumer belongs to render
(`Plan::state`, "exclusive to the render thread"), and render does not run while the host is
stopped. So the first block after restart discards every old block and underruns, and the host
underruns until it refills.

The C ABI has no "render starts now" call. `Session` reaches the producers only, never the
consumers. The browser's single mode prepares its consumers in the worklet's message handler, on
the render thread itself; a C ABI host's render thread is host-owned and anonymous, so it needs a
contract of its own.

## Scope

Let a stopped C ABI host make room for the new generation before it restarts, without a
render-owned consumer being touched from another thread while render could run. Keep render
allocation-free, lock-free and syscall-free. No rendered bit moves on any existing path (class A).

## Contract

`uint32_t miso_engine_v1_plan_prepare_seek(miso_engine_v1_plan *plan)`, an in-place V1 amendment:

- **Bit.** `MISO_ENGINE_V1_FEATURE_PLAN_PREPARE_SEEK` = bit 13 (8192). The feature mask grows from
  8191 to 16383. The frozen exported set grows from 22 to 23 symbols. No struct, size or
  `ABI_VERSION` change: `Plan` is opaque, so its new atomic word is not a layout change.
- **Who touches the consumer.** Only the render side. The call takes the *plan* handle, so it sits
  on the render side by construction. The host calls it on its render thread between blocks, or on
  any thread while its render callback is stopped. It is never concurrent with
  `render_f32_planar` or with itself. Nothing crosses from `Session` into the plan's render state.
- **When.** After an admitted seek and before the restart: the stopped host calls
  `session_seek_at_once`, then `plan_prepare_seek`, then submits generation g from each source's
  frame, then restarts.
- **What it does.** Every source of the active plan observes its command slot under the hold clock
  (`PcmSourceConsumer::prepare_admitted_seek`, through
  `GraphPreparedSourceSetDriver::prepare_admitted_seeks`). A plain seek in the slot, or a plain seek
  held for its runway, applies now. The consumer then discards and recycles every queued block of
  an older generation (bounded by the ring's capacity), so the producer accepts the new generation.
  The set marks a pending generation change for the next block. The timeline is not moved here: the
  at-once word lands it at the next block, and a prepared consumer holds nothing, so the set lands
  the session seek there. An anchored seek stays held and nothing is discarded for it. A source
  with no admitted seek is unchanged; a call with nothing to prepare is `RESULT_OK`. It renders
  nothing, advances no clock and allocates nothing. A published candidate plan is not prepared; its
  consumers apply their seeks at the block that adopts it (as the browser's single mode).
- **How misuse is refused.** `Plan` has a `render_busy: AtomicU32` beside `last_error`. Both
  render-side entries claim it after the handle check with one compare-exchange 0 to 1 (`Acquire`)
  and release it with one `Release` store through a drop guard on every exit, an unwind included.
  A call that finds it claimed touches no plan state and returns `RESULT_RENDER_REJECTED` with
  `render.busy` (plan diagnostic 9). The claim never waits, spins or makes a syscall. Under any
  race at most one thread holds `&mut PlanState`: misuse costs one refused block or one refused
  prepare, never undefined behaviour. A successful prepare clears the plan's diagnostic.

## Root rulings (2026-10-30, on the plan's open questions)

1. Result only, no out-count. The host reads `source_seek_report` after the first block.
2. A busy refusal is `RESULT_RENDER_REJECTED` with `render.busy`. No new result code.
3. The guard is always on, not behind the feature bit. Its render cost is recorded once (below)
   against the 2 % allowance. It is one CAS on entry and one `Release` store on exit, with no wait,
   spin or syscall.
4. An anchored seek while stopped stays held and is not prepared.
5. Prepare serves any admitted plain seek (a live `session_seek` or a per-source seek too). The
   pairing with `session_seek_at_once` is documented, not enforced.
6. The browser's worker mode keeps the same latent residual: the control worker frees the SDK's
   shared ring only, and the engine ring relies on the Worker's headroom. Out of scope here; a
   possible successor, not filed.

## Gates

- `capi` `session_seek::a_stopped_host_refills_a_full_ring_before_restart` (the issue's gate): a
  host with `LONG`'s ring full of generation 1 seeks with `session_seek_at_once(2, 2000)`, calls
  `plan_prepare_seek` (`RESULT_OK`), submits the target's runway (every submit `RESULT_OK`), and
  restarts. The first block lands `(2, sample, 2000)`, reports 0 underrun frames, and is
  bit-identical to a fresh session's first block fed the same quantum. Red on base.
- `capi` `session_seek::a_render_side_call_during_render_is_refused_not_raced`: with the claim word
  held, `render_f32_planar` and `plan_prepare_seek` return `RESULT_RENDER_REJECTED` with
  `render.busy`; released, both succeed and a render clears the diagnostic.
- `source` `seek_landing::a_prepared_admitted_seek_frees_a_full_ring_for_its_generation` and
  `seek_landing::a_prepared_anchored_seek_stays_held`.
- `abi_smoke.c` and `header_smoke.cpp` pin bit 8192, mask 16383 and the signature; `abi_smoke.c`
  checks the bit against the queried mask and refuses a null plan with `INVALID_ARGUMENT`.
  `scripts/check-capi-abi.sh` freezes the 23-symbol export set.
- `realtime_policy`, `audit capi` (0 violations), workspace clippy, fmt, workspace policy.

## Evidence (attempt 1, 2026-10-30)

### Red claims of the new tests

- Gate: red on base. With the export routed to a no-op (the prepare call removed), the first
  target submit returns 6 (`BACKPRESSURE`); with the submits left unchecked, the restart block
  underruns 128 frames.
- Busy: red with the claim removed from `render_f32_planar` (the held-word render returns 0, not
  8); red with the release store removed (the next render after a successful prepare returns 8).
- Source, free: red with the acquire removed from `prepare_admitted_seek` (0 blocks discarded, not
  the 8 queued). Source, anchored: red when the hold clock lands an anchored seek.

### Guard cost (root ruling 3)

One descriptive paired run (one warmup round and two measured rounds of 20,000 blocks), base
`084f1b897` and head fat-LTO `libcapi.so` loaded side by side in one process, render calls
alternated block by block, the `console-c1-64` fixture (64 sources, 64 tracks, 48 kHz, quantum
128), AMD EPYC 7313P. Per render call:

| round | base mean ns | head mean ns | delta | base median | head median |
|---|---|---|---|---|---|
| 1 | 73,711.1 | 73,237.5 | -0.64 % | 73,389 | 73,530 (+0.19 %) |
| 2 | 73,662.4 | 73,143.9 | -0.70 % | 73,410 | 73,560 (+0.20 %) |

The guard is one CAS and one store, a few nanoseconds. The difference between the two builds is
within ±0.7 % in either direction, which is code-layout noise of two separate fat-LTO builds, and
inside the 2 % allowance.

### No bit moved (class A)

One-time comparison, not a committed test: every `fixtures/session/v1` session that compiles (17;
`canonical.json` is refused with `effect.native.unavailable`, `json-invalid-duplicate-key.json` is
invalid by design) rendered for 300 blocks through the base and the head `libcapi.so`, fed one
deterministic quantum per source per block, with a `session_seek_at_once(2, 0)` at block 150 and
generation 2 fed after it. 0 differing samples in every session.

`audit capi` (100,000 render calls with live edits, a structural swap, an anchored seek and a session
seek) gives the same `pcm_digest` at base and head: `30dcfd806e7ba058`, with 0 violations at head.

### Checks run at head

`cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D
warnings`; release tests of `capi`, `source`, `graph`, `engine` and `control-plane` with
`--all-features`; `scripts/check-workspace-policy.sh`; `realtime_policy` (ok, 291 marked regions);
`audit capi` (0 violations); `scripts/check-capi-abi.sh` (shared and static linkage, C and C++
smoke tests) and its `--self-test`.
