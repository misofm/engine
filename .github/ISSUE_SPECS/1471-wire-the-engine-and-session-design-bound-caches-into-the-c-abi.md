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
  budget `INPUT_BOUND_BUDGET_FRAMES` (1,510,000 frame-equivalents of 18.0 ns, 27.18 ms,
  charged in strip order: each design computed charges the frames it walks plus
  `INPUT_BOUND_SECTION_CHARGE` = 580 per section, #1457 Amendments 3 and 4, with the 1.05 margin
  on the whole charged work per #1465's whole-work margin fix of 2026-10-08, on the stream G batch
  3 verifier's figure of record, so at least 837 a computed design and at most 1,804 designs a
  preparation; past it a design reports the
  rate's live bound), and `input_section_bounds(rate, strips, cache)`. A cached design charges the budget
  its stored charge, so every prepared value is a pure function of the session, with or without a
  cache, cold or warm.
- **Cache memory and clearing (#1457 Amendment 3, attempt-1 verdict m3).** A full cache of 8,192
  entries holds about 2.4 MiB (2,486,520 bytes measured on #1457 attempt 2, about 304 bytes an
  entry; the attempt-1 verifier measured 2,366,400 bytes, 2.26 MiB, before each entry gained its
  charge field). Seeding each session with a copy of the engine's cache costs up to that much per
  session. A rebuild is served entirely from a cache only while that cache has not been cleared
  since the session's designs were inserted: the engine's cache, which every compile on the engine
  fills, can reach its cap and clear in the middle of a compile, and a session seeded from it then
  lacks the designs inserted before the clear. `builtins-compiler`'s
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
   the same engine likewise computes none. *(#1457 Amendment 3, m3.)* The gate includes an engine
   cache near its cap: the session is compiled on an engine whose cache already holds enough other
   designs that the compile's insertions pass `INPUT_BOUND_CACHE_ENTRIES` and clear it, and the
   session's first rebuild still computes no design bound. D1's seeding (a copy of the engine's
   cache after the compile) fails this case when the compile's insertions clear the engine's cache,
   so the slice brings the seeding to root before implementation (for example seeding each session
   from the compile's own designs). The per-session seed's memory is recorded beside gate 4.
2. Every prepared value and the plan resource report are identical with and without the caches,
   cold and warm, including a session past the budget (#1457 gates 3 and 5 stay green).
3. `target/release/audit capi` stays at 0 allocations and 0 syscalls on render; the PCM digest does
   not move; `scripts/check-capi-abi.sh` passes with no header change.
4. The native rebuild cost: the C ABI preparation of the 9-track and 64-track documents is recorded
   on a first compile and on a rebuild of the unchanged session (release, one invocation, one warmup,
   two measured rounds); the design-bound work is within #1457's budget on the first, and zero on
   the rebuild (gate 1).
   *(Root ruling, #1457 Amendment 4, n1.)* The first preparation in a process pays an allocator
   first-touch cost that is not a cache cost: #1457 attempt 2 saw a 65,537-strip first preparation
   with a fresh cache take 11-12 ms more than the same preparation without a cache, and its
   verifier showed that with glibc heap trimming off a cold cache costs the same as no cache
   (39 ms) and only the process's first preparation pays the extra (about 54 ms; #1457 attempt 2
   verdict, `docs/handoffs/decision-15-2026-10-05/verdicts/stream-g2/1457-attempt2.md`, which
   states the figures of its `probe-order.log` and `probe-order-notrim.log`). This slice
   explains and measures that first-touch cost on the C ABI's first compile: how much of the first
   compile it is, at the 9-track and 64-track documents and at 65,537 strips, with heap trimming on
   (the default) and off, recorded beside the first-compile and rebuild figures.

## Non-goals

- The browser cache (#1470). The budget, the live-bound table and the cache type (#1457). Any
  render-side change.

## Dependencies

- #1457 (stream G), #1309 (stream B, batch 1, on `main`).

## Amendment 1 (root, 2026-10-11): seed each session from the compile's journal

The dfg coordinator brought gate 1's seeding question before implementation: `InputBoundCache::insert`
clears the whole cache when full and a hit inserts nothing, so a copy of the engine's cache taken
after a compile that cleared it mid-way lacks the designs hit or inserted before the clear, and the
session's first rebuild computes them again. Root chose option (1), the journal:

- **D1 (amended).** Each session's cache is seeded with "the designs that compile used, journaled at
  the time of use, each with its stored charge" (so a session rebuild's results stay a pure function
  of the session under #1457's stored-charge rule), in place of a copy of the engine's cache after
  that compile. D2 does not change: the parameter stays `Option<&mut InputBoundCache>`.
- **Named exception.** A `begin_journal`/`take_journal` pair on `InputBoundCache` in
  `crates/builtins/src/tail.rs` (with its unit tests): while a journal is open, every design a
  preparation uses, hit or computed (a `ChargeAbove` entry included), is also recorded at the time of
  use, so no clear loses it; `take_journal` returns exactly those designs as an `InputBoundCache`.
  The C ABI compile opens the journal on the engine's cache, prepares, and seeds the session from
  `take_journal`.
- **Gates.** Gate 1 passes with an engine cache that clears mid-compile: a test fills the engine's
  cache to just below its cap, compiles, then rebuilds and asserts zero recomputed designs and zero
  walked frames; a mutant that seeds from the post-compile copy turns it red. The per-session seed's
  memory is measured and recorded beside gate 4.

## Attempt record

### Attempt 1 (implementer, 2026-10-11) -- BLOCKED on one accounting question for root

**What landed** (`28de9b791` builtins journal; the wiring commit after it).
- `builtins` (`tail.rs`, named exception): `InputBoundCache::begin_journal` / `take_journal`. The open
  journal is a record with no entry cap of its own; `lookup` (the former `get`, now `&mut self`)
  records a hit, `insert` records a computed value, so a `ChargeAbove` is recorded as a `Bound` is.
  `take_journal` closes it and returns the record as a cache with the cache's cap (an empty cache
  when none is open). `input_section_bounds_within` reads through `lookup`.
- `host-core`: `prepare_host_runtime_with_live_lanes` and `..._successor` gain
  `bound_cache: Option<&mut InputBoundCache>` (D2), passed to the policy function; `InputBoundCache`
  is re-exported from `host-core/src/lib.rs` (capi and control-plane do not depend on `builtins`).
  Callers updated by adding `None`: `host-core/tests/live_lanes.rs`, capi `runtime/tests.rs`,
  `runtime/live_tests.rs`, `tests/resource_lifecycle.rs`.
- `control-plane`: `compile_children(document, limits, engine_cache: Option<&mut InputBoundCache>)`
  opens the journal on the cache (a local cache when `None`), prepares, closes the journal on every
  path, and seeds `SessionState::bound_cache` from `take_journal`. `prepare_runtime` takes the
  cache; the structural rebuild moves the session's cache out with `mem::take`, prepares with it and
  puts it back before the result is looked at. `InputBoundCache` is re-exported.
- `capi`: `Engine::bound_cache: RefCell<InputBoundCache>`; `miso_engine_v1_compile_session` passes
  it; the capi `compile_children` wrapper takes the same `Option` (test callers pass `None`).
- Policy: `scripts/check-realtime-policy.sh` lists the new test file in its `unsafe` allow-list
  (the file calls the exported C functions).

**Gate 1 and Amendment 1.** `capi/tests/design_bound_cache.rs`: (1) 64-track fixture, cold compile
computes 64 designs; two rebuilds in a row and a second compile compute 0 designs and walk 0 frames.
(2) The engine's cache is filled to 8,160 designs (11 compiles of at most 800 cheap distinct
designs, each asserted computed whole), then the 64-track document is compiled: its 33rd insertion
clears the cache (a second compile of it recomputes exactly 32, asserted), and the session's first
rebuild computes 0 designs and walks 0 frames. (3) 1,200 slow designs pass the budget (cold compile
computes 752): the warm compile and the rebuild compute 0 and walk 0 (the stopped design is cached
and journaled as a horizon).

**Mutation runs** (each red, then reverted green):
- Seed the session from the post-compile copy of the engine's cache: test 2 red (rebuild computes 32
  designs, 333,120 frames); test 1 stays green. No existing test reads the counters through capi or
  control-plane, so none catches it (`grep` of `fixed_input_*` users: builtins, builtins-compiler,
  this file).
- Seed the session empty: tests 1 and 2 red (64 designs). Pass `None` for the engine cache in
  `compile_session`: tests 1 and 2 red. Pass `None` for the session cache in the rebuild: tests 1
  and 2 red. None of these has another catcher.
- Journal: `lookup` does not record: `a_journal_holds_the_hits_...` red. `insert` does not record:
  `a_journal_holds_every_design_...` red. Only `Bound` entries journaled: the hits-and-stopped-walks
  test red and the past-budget capi test red (194 frames walked). Journal capped at the cache's cap:
  the through-a-clear test red.
- A cache hit that charges 0: the capi past-budget test red (570 designs computed on the warm
  compile), but `tail_contract::the_live_bound_is_taken_exactly_when_the_budget_is_exhausted` already
  catches it; the past-budget capi test's own catch is the horizon in the seed.
- Test-value sentences. `a_journal_holds_every_design_a_preparation_used_through_a_clear`: a journal
  that drops a design when the cache clears mid-preparation. `a_journal_holds_the_hits_and_the_stopped_walks_a_preparation_used`:
  a journal that records only computed values, or only bounds. `a_compile_that_clears_the_engine_cache_still_seeds_its_session`:
  seeding the session from the engine's cache after the compile (Amendment 1). `a_rebuild_and_a_recompile_compute_no_design_bound`:
  a session or engine cache that is not wired into the rebuild or the compile.
  `a_session_past_the_budget_rebuilds_without_walking`: a seed or hit that loses a stopped design's horizon.

**Gates.** `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -D warnings`,
`check-workspace-policy.sh`, `check-realtime-policy.sh`, `check-capi-abi.sh` (no header change),
`check-cross-targets.sh` PASS. `target/release/audit capi`: 0 allocations, 0 syscalls, `pcm_digest`
`cb10fbface44a3a4` (unchanged). Worklet chain (`build-web-audioworklet.sh --named-twin`,
`check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py
--artifacts`, `check-scalar-oracle-absent.py`, `check-web-audioworklet-v8-spill.py`,
`test-web-audioworklet.sh`) all pass; the shipped module's sha256 is
`02a77253fb99ac1280d1917bc077fd7891248d792571a690a8fec8cc78f9d71c` (the browser passes `None`; the
bytes still move because `InputBoundCache` gained a field and `lookup`).
Tests: builtins `tail_contract`, control-plane, host-core, builtins-compiler, capi lib and
`design_bound_cache` and `plan_swap_race` pass.

**Open: two tests red, awaiting a root ruling.** `resource_lifecycle`'s
`capi_retained_bytes_charge_every_byte_the_compile_retains` and
`tiny_control_frame_still_accounts_three_provider_counters_exactly` observe the bytes a C ABI compile
leaves live and require `capi_retained_bytes` to charge them exactly. The session's cache is such
memory (2,608 bytes on the nine-track fixture; with the seed mutated to empty both pass), and the
spec does not say how it is accounted. It also grows past its seed on every rebuild, up to
`INPUT_BOUND_CACHE_ENTRIES` entries (about 2.4 MiB), so a charge of the seed alone would let the
retained limit be passed silently. Options for root: (1) charge the cache's ceiling in
`control_retained_bytes` for every session (about +2.4 MiB a session in every report) and let the
exactness tests treat the cache as an owner charged at its ceiling; (2) give the session a lower
entry cap and charge that; (3) leave it out of `capi_retained_bytes` as the engine's cache is, state
it as a control-thread cache outside the limits, and have the tests subtract it.

**Gate 4 (descriptive; release, `taskset -c 7`, one invocation per document and trim setting, each
with the process's first compile, one warmup round and two measured rounds; a round is a cold
compile on a fresh engine, a rebuild of the unchanged session, a warm second compile).** The
workload is the nine-track fixture, `console-sixty-four-track.json`, and a generated 65,537-strip
document (the nine-track strip cloned with cutoffs `i mod 8000`); whole `miso_engine_v1_compile_session`
and `miso_engine_v1_submit_command` times, design-bound counters beside. The scratch driver is not
committed.

| document | cold compile (ms, M1/M2) | rebuild (ms) | warm compile (ms) | designs / frames / charged, cold | rebuild, warm |
|---|---|---|---|---|---|
| 9 strips | 1.63 / 1.65 | 1.29 / 1.28 | 1.44 / 1.43 | 1 / 12,033 / 13,193 | 0 / 0 / 0 |
| 64 strips | 26.90 / 26.84 | 12.33 / 12.31 | 14.21 / 14.01 | 64 / 658,816 / 807,296 | 0 / 0 / 0 |
| 65,537 strips | 26,348 / 26,119 | 27,216 / 27,817 | 28,075 / 29,568 | 752 / 637,168 / 1,509,488 | 0 / 0 / 0 |

The first compile's design-bound work is within #1457's budget (charged 13,193; 807,296; and
1,509,488 of 1,510,000 at 65,537 strips, where the budget stops the 753rd design). The 64-track
rebuild is 13.5 ms faster than its cold compile, 14.2 ms of design work removed at 18 ns a charged
frame-equivalent (807,296 x 18 ns). First-touch, first compile in a fresh process against the
measured cold compile: trimming on, 9 strips 2.41 vs 1.65 ms (0.76 ms, 31 % of the first compile),
64 strips 29.18 vs 26.87 ms (2.3 ms, 8 %); trimming off (`MALLOC_TRIM_THRESHOLD_=4294967295`), 9
strips 2.46 vs 1.66 ms, 64 strips 30.28 vs 27.20 ms (3.1 ms). So at these sizes first-touch is
about 1-3 ms and is not changed by trimming. At 65,537 strips the compile is 26-30 s whole (the
document's protocol store, graph and plan, not the design bounds), the run-to-run spread is a few
seconds, and the 54 ms first-touch of #1457's attempt 2 is under 0.2 % of it and cannot be
resolved: first 27,975 ms trimming on and 29,749 ms off against measured colds of 26,119-26,348 and
25,928-26,566 ms. The warm compile is not faster than the cold one at 65,537 strips for the same
reason (design bounds are 27 ms of 27 s).
**Per-session seed memory** (the engine cache after one cold compile holds the same designs in the
same insertion order as the journal; bytes freed by `engine_destroy` less the empty engine's 336):
9 strips (1 design) 2,608 B; 64 strips (64 designs) 18,352 B; 65,537 strips (752 designs)
327,728 B. A preparation can seed at most 1,805 designs, about 0.5 MiB.
