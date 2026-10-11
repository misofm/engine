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

## Amendment 2 (root, 2026-10-11): the session seed is the newest plan's designs, charged exactly

- **D1 (amended again).** The design-bound seed is an epoch owner.
  - Each `ProviderEpoch` owns `bound_seed`: the journal of the preparation that made its plan, meaning every design that preparation used, at the time of use, each with its stored charge or stopped-walk horizon.
  - The compile seeds the first epoch from the journal it opens on the engine's cache.
  - A structural rebuild reads `newest_providers().bound_seed` read-only and opens a journal of its own. A design it computes goes into its journal only, never into that seed.
  - The candidate epoch owns the journal as its `bound_seed`. The commit publishes it with the candidate (`pending_providers.push`). A refused or abandoned rebuild drops it with the candidate, and every existing seed is unchanged.
  - An epoch's seed drops when the epoch is reclaimed.
  - `SessionState` holds no cache.
- **D3 (new). Exact form.**
  - `take_journal` returns the record in an exact-size, read-only form: entries sorted by key in one boxed slice of exactly its length, looked up by binary search.
  - `retained_bytes()` is `len × size_of::<(InputBoundKey, CachedDesign)>()`.
  - A preparation over a read-only form never changes it.
  - A journal is opened and taken inside one call of `prepare_runtime`, before `prepared_capi_resources`. It is never open across preparations.
  - D2's parameter type does not change.
- **D4 (new). Charge.**
  - The seed is a row of `epoch_retained` (`capi_resources`' `epoch_rows`), and its slice is a candidate for `largest`.
  - So `prepare_runtime`'s `active_retained` check and `validate_replacement_peak` refuse a compile or a rebuild whose seed would pass `maximum_capi_retained_bytes`. The refusal is the existing `ResourceFault::Limit`, it comes before any commit, and nothing changes.
  - At most 1,805 designs are recorded per preparation. Record their bytes at the exact size.
- **D5 (new). Engine cache.**
  - The C ABI header gets one comment-only paragraph. It says the engine owns a control-thread design-bound cache for its whole life, outside every compile limit, and that engine destroy frees it. It states a ceiling derived from `INPUT_BOUND_CACHE_ENTRIES`, the entry layout and the map's worst-case node occupancy. A measured figure for one fill order is not a ceiling.
  - `docs/C_ABI_V1_QUALIFICATION.md` states the same ceiling.
- **Gate 3 (amended).** `scripts/check-capi-abi.sh` passes with no change to the declarations, symbols or layout. The only header change is D5's comment.
- **Named exceptions.**
  - `crates/builtins/src/tail.rs`: the read-only form, `retained_bytes`, and a fix to the doc link and to the cap wording.
  - `crates/control-plane/src/control.rs` and `compile.rs` (`ProviderEpoch`).
  - `crates/capi/tests/resource_lifecycle.rs`: the `capi_epoch_terms` seed term, one more document, the engine-ceiling test, and restating the `capi_retained_bytes` budget row with its reason if the seed passes it.
  - `docs/REALTIME_DEPENDENCY_POLICY.md`: the `design_bound_cache.rs` ownership entry, whose unsafe is the C calls, render included.
  - `scripts/check-realtime-policy.sh` (one line).
  - The capi `builtins` test-support dev-dependency.
  - The header comment.
  - The browser module's sha256, recorded here.
- **Recorded consequence.** A revert to a design that the newest plan does not use computes that design again, within #1457's budget.

### Amendment 2 gates added or changed, each with its test-value sentence

1. **Existing `capi_retained_bytes_charge_every_byte_the_compile_retains` and `tiny_control_frame_still_accounts_three_provider_counters_exactly` go green again, unedited.** Defect: a seed that the compile leaves live and nothing charges (both are red on attempt 1).
2. **The same test gains one document with at least two distinct enabled input designs.** Defect: a seed charge that is right for one design but wrong per design, for example a fixed size, or a growable buffer's capacity in place of its length.
3. **`capi_epoch_terms` adds the prospective preparation's seed bytes, and `double_live_oracle_drives_exact_and_one_below_c_caps` asserts that the seed is not zero.** Defect: a rebuild whose successor seed is left out of the prospective epoch's charge, so a rebuild past `maximum_capi_retained_bytes` is admitted.
4. **New in `design_bound_cache.rs`: `a_rebuild_keeps_only_the_designs_its_plan_uses`.** Compile the 64-track document. A rebuild that changes one strip's HPF computes exactly 1 design. An unchanged rebuild then computes 0. Reverting the strip computes exactly 1. Defect: a session cache that keeps designs the newest plan does not use, growing toward 8,192 entries with nothing charging it (option B's shape), or a rebuild that recomputes strips it did not change.
5. **New: `a_refused_rebuild_leaves_the_seed_as_it_was`.** A rebuild that changes one strip is refused after its preparation, either through a control-plane test fault after admission or with a capi limit one byte below its peak. The same edit submitted again commits and computes exactly 1 design. Defect: a rebuild that writes its designs into the newest seed before commit (attempt 1's shape), so a refused rebuild grows session memory past its charge.
6. **New in `resource_lifecycle.rs`: `the_engine_cache_stays_within_its_stated_ceiling`.** Fill one engine's cache to `INPUT_BOUND_CACHE_ENTRIES`. The bytes engine destroy frees, less an empty engine's, are at most D5's ceiling. Defect: an entry-layout change, a cap that no longer applies, or a journal left open on the engine's cache, any of which pushes the engine's lifetime memory past the figure the header gives app developers.

Authority: root decision under the owner's delegation of 2026-10-11, after a fresh Opus review (ACCEPT WITH CHANGES) adopted verbatim. Attempt 1's pre-admission fill of the session cache (`control.rs` ~1106-1119) is a defect to remove.

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
rebuild is 14.6 ms faster than its cold compile; the design work removed is 14.5 ms at 18 ns a
charged frame-equivalent (807,296 x 18 ns). First-touch, first compile in a fresh process against the
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

### Attempt 2 (implementer, 2026-10-11) -- Amendment 2 implemented

Commits `6d56b1f15` (code and tests), `32241583c` (D5 header comment, qualification doc, ownership
entry), then two test follow-ups. Builds on attempt 1; the open question of attempt 1 is closed by
Amendment 2.

**What changed.**
- `builtins` `tail.rs` (named exception): `take_journal` returns `InputBoundRecord`, a sorted boxed
  slice of exactly its length, found by binary search; `retained_bytes()` = len x 240
  (`size_of::<(InputBoundKey, CachedDesign)>()`). `InputBoundCache<'seed>::with_seed(&record)` reads
  a record read-only (own entries first, then the record); a design computed goes into the
  cache's entries and journal only. The journal is a plain map with no cap (a clear of the entries
  loses none of it) and `begin_journal` restarts it. The doc link to the private
  `CachedDesign::ChargeAbove` is gone (workspace `cargo doc -D warnings` passes), and the cap
  wording says a full cache's 2.4 MiB is one fill order, not a ceiling.
- `control-plane`: `ProviderEpoch::bound_seed`; `prepare_runtime` takes the cache by `&mut`,
  opens the journal, closes it before `prepared_capi_resources` on every path, and charges the
  seed bytes as an `epoch_rows` row and a `largest` candidate (D4), so the existing
  `active_retained` check and `validate_replacement_peak` refuse before commit (`ResourceFault::
  Limit`). A compile passes the engine's cache (or a local one); a rebuild passes
  `InputBoundCache::with_seed(&newest_providers().bound_seed)`; the candidate epoch owns the new
  record. `SessionState` holds no cache. Attempt 1's `mem::take` pre-admission fill is gone.
- `capi`: `Engine::bound_cache` is `InputBoundCache<'static>` (a comment only); the header gets
  the D5 paragraph (comment only, `check-capi-abi.sh` passes).
- Docs: `C_ABI_V1_QUALIFICATION.md` "Design-bound caches (#1471)" derives the engine ceiling;
  `REALTIME_DEPENDENCY_POLICY.md` gets the `design_bound_cache.rs` ownership entry. The
  `check-realtime-policy.sh` line was already in attempt 1.
- `control.rs`/`compile.rs`/`abi.rs` hunks for stream B's overlap: `compile.rs` (PreparedRuntime
  field; `capi_resources`/`prepared_capi_resources` one parameter; `prepare_runtime` journal and
  signature; `compile_children` three lines and the epoch constructor), `control.rs`
  (`ProviderEpoch` field, two constructors, a test accessor; the removed `SessionState` field;
  the rebuild call and the candidate constructor), `lib.rs` (one re-export), `capi/abi.rs`
  (one field type and its comment).

**Engine ceiling (D5), derived.** 8,192 entries; entry = 124-byte key + 112-byte value; the std
`BTreeMap` (B = 6) keeps 5 to 11 entries in every node but the root, so at most
1 + 8,191 / 5 = 1,639 nodes, at most 273 internal; leaf 2,608 B, internal 2,704 B:
1,639 x 2,608 + 273 x 96 = **4,300,720 bytes** (64-bit; allocator bookkeeping up to 16 B a node
extra). The test fills 16,384 distinct designs (the cache fills, clears at the 8,193rd, fills
again) and measured 3,602,112 B freed at engine destroy over an empty engine's.

**Acked-batch question.** Can a commit be acknowledged while its seed is dropped? No. The
candidate epoch owns its seed from `prepare_runtime`; every refusal before `prepared.commit`
(admission, backpressure, test faults) drops the candidate with its seed and leaves the newest
seed untouched (nothing is written to it); after the protocol commit `pending_providers.push`
cannot fail (capacity was checked before the commit), and the ack is written after it. A seed
drops only with its epoch, after the plan it belongs to is reclaimed. A live commit prepares and
changes no seed.

**Gates run.** `cargo fmt --check`; `clippy --workspace --all-targets --all-features -D warnings`;
`RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`; `cargo test` for `builtins`
(test-support), `builtins-compiler`, `host-core`, `control-plane` (test-support) and `capi`, all
suites, `resource_lifecycle` included (12 tests, about 37 s of which the ceiling test is most);
`check-workspace-policy.sh`, `check-realtime-policy.sh`, `check-cross-targets.sh` (PASS),
`check-capi-abi.sh` (ok); `target/release/audit capi`: 0 allocations, 0 syscalls, `pcm_digest`
`cb10fbface44a3a4` (unchanged). Worklet chain (`build-web-audioworklet.sh --named-twin`,
`check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py
--artifacts`, `check-scalar-oracle-absent.py`, `test-web-audioworklet.sh`,
`check-web-audioworklet-v8-spill.py`) pass; **shipped browser module sha256
`f9e711daba98d87aa81d5a8ebfceafa25a9ea2fd2ff20091bc58e4faf71cda41`** (named twin
`a794dc778b730624b636ac2f330b762968c3d09d7ac1f8697cb6e61ec0c61719`); attempt 1's value is
superseded.

**Per-session seed bytes (gate 4, exact size now).** len x 240: 9 strips (1 design) 240 B;
64 strips (64 designs) 15,360 B; 65,537 strips (752 designs, attempt 1's count) 180,480 B; at
most 1,805 designs a preparation, 433,200 B. Attempt 1's 2,608 / 18,352 / 327,728 B figures were
a full-map copy; they are superseded. The timing table above is unchanged (the workload is the
same; not re-run, as Amendment 2 does not change it). A second plan's pending seed lives beside
the newest one until the older epoch is reclaimed, and the peak admission counts both.

**Mutation runs** (each red on the named defect, then reverted green; "other catchers" is the
rest of `capi` lib, `design_bound_cache`, `plan_swap_race`, `resource_lifecycle` or `tail_contract`
run under the same mutant):
1. Gate 1. Seed bytes passed as 0 to `prepared_capi_resources`: `capi_retained_bytes_charge_every_
   byte_the_compile_retains` (240 B short) and `tiny_control_frame_...` red, and
   `double_live_oracle_...` red. Both original tests are unedited and green on the real code.
2. Gate 2. `retained_bytes` = min(len, 1) x 240: only the `three input designs` document of
   `capi_retained_bytes_...` red (left 274,681 right 274,201); the four older documents (one
   design or none), `tiny_control_frame_...` and `double_live_...` stay green, so none catches it.
3. Gate 3. Seed charge 0 on a rebuild only (`successor.is_some()`): `double_live_oracle_...` red
   (a cap one below the true peak is admitted); every other `capi` suite green.
4. Gate 4. `begin_journal` pre-filled with the seed (a seed that keeps designs its plan does not
   use): `a_rebuild_keeps_only_the_designs_its_plan_uses` (the revert computes 0 not 1) and the
   `builtins` `a_cache_over_a_record_...` red; nothing else. With the seed ignored by `with_seed`
   (recompute everything) all five `design_bound_cache` tests are red, test 4 among them
   (64 designs, not 1), but the older ones catch that shape too.
5. Gate 5. The rebuild writes its record into the newest seed before admission:
   `a_refused_rebuild_leaves_the_seed_as_it_was` red (computes 0 not 1); every other suite green.
6. Gate 6. `INPUT_BOUND_CACHE_ENTRIES` = 16,384: `the_engine_cache_stays_within_its_stated_
   ceiling` red (7,151,872 B). **Limit, stated plainly:** a journal that `take_journal` leaves
   open is restarted by the next `begin_journal`, so it holds one preparation (at most 433,200 B),
   which fits the 0.70 MB of headroom between the measured 3.60 MB and the ceiling: that mutant
   stays green here. The test also goes red only for an entry that grows by more than about 85
   bytes. Closing the journal is defended by `prepare_runtime` taking it on every path and by the
   builtins journal tests; a sharper check needs an engine-internal test, not in Amendment 2's
   exceptions.
7. Builtins record tests. A lookup reaching only the first three entries: `a_cache_over_a_record_
   finds_every_design_and_journals_only_what_it_used` alone red (forty-design record). A seed hit
   not journaled: that test and three `design_bound_cache` tests red. A reversed record: the two
   older journal tests and the new one red.

**Test-value sentences.**
- `a_cache_over_a_record_finds_every_design_and_journals_only_what_it_used`: a record lookup that
  loses designs of a large record, or a journal that omits the seed hits or copies the unused ones.
- `a_rebuild_keeps_only_the_designs_its_plan_uses`: a session seed that keeps designs the newest
  plan does not use (union growth toward 8,192 entries with nothing charging it).
- `a_refused_rebuild_leaves_the_seed_as_it_was`: a rebuild that writes its designs into the newest
  seed before commit.
- `the_engine_cache_stays_within_its_stated_ceiling`: a cap that no longer applies, or an entry
  grown beyond the headroom, past the header's figure (journal limit above).
- `three input designs` in `capi_retained_bytes_...`: a seed charge right for one design but wrong
  per design.
- `double_live_oracle_...` seed term: a successor seed left out of the prospective epoch's charge.

**Open.** Gate 6's journal-left-open sub-defect is not caught within the ceiling's headroom (above).
The ceiling test takes about 37 s in debug. Stream B's overlapping hunks are listed above.

### Attempt 3 (implementer, 2026-10-11) -- the attempt-2 findings

Fixes exactly the attempt-2 verdict (`1471-attempt2.md`); the product code is unchanged except n1.
It supersedes attempt 2's gate 6 "Limit" paragraph and its "sharper check needs an engine-internal
test" sentence: both checks fit the engine-ceiling test's exception (C ABI only).

**M1 (gate 6), `resource_lifecycle.rs`.**
- New `a_compile_leaves_nothing_open_on_the_engine_cache`: engine A compiles a 200-design
  document; engine B compiles it and then a document with no input filter; the bytes engine destroy
  frees must be equal (84,208 = 84,208, an allocator-observed equality, not a pinned count).
- `the_engine_cache_stays_within_its_stated_ceiling` now first measures the leaf node (a
  one-design compile on a fresh engine frees exactly one: 2,608 B), derives the worst case with the
  header's formula (1,639 x leaf + 273 x 12 pointers = 4,300,720 B, from the cap constant) and
  asserts it is at most the stated figure; the 16,384-design fill stays. Shared helpers
  (`roomy_limits`, `create_engine`, `compile_on`) replace the closures.
- Mutation runs (each applied, red, reverted green):
  1. Journal left open (`take_journal` clones): the new test red (168,048 vs 84,208); the ceiling
     test also red (it measures a doubled node, 5,216 B, derived worst case 8,575,232). The old
     fill-only check was green here (attempt-2 verdict M6b).
  2. +8 bytes of value (`ChargedInputBound` gains a `u64`): the ceiling test red at once (leaf
     2,696 B, worst case 4,444,952 B above 4,300,720); the new test green. The old fill-only check
     was green (verdict M1).
  3. Cap constant 16,384: ceiling test red (derived worst case 8,598,832).
  4. Cap not applied (the map's cap `INPUT_BOUND_CACHE_ENTRIES * 2`, constant unchanged): ceiling
     test red by the fill (7,143,856 B freed); the new test green.
- Test-value sentences. The new test: a journal that stays open on the engine's cache after a
  compile (named; the ceiling test turns red under it too, by the doubled node it measures, with a
  node message). The ceiling test: an entry whose key or value grew by even 8 bytes, and a cap that
  no longer applies.

**m1, m2 (header comment and `C_ABI_V1_QUALIFICATION.md`).** The ceiling is stated as 4,300,720
bytes of requested heap between calls; the allocator's per-block rounding comes on top, differs by
platform (glibc, libmalloc, scudo) and has no stated figure. During a compile the cache also holds
the open journal: at most 1,805 designs (`INPUT_BOUND_BUDGET_FRAMES / 837` + 1), so 1 + 1,804 / 5 =
361 nodes, 60 internal: 361 x 2,608 + 60 x 96 = 947,248 B requested; the call frees it before it
returns. Comment-only header change (`check-capi-abi.sh` ok).

**m3 (gate 4, re-run on the final code).** Same frozen driver and workload as attempt 1 (scratch,
not committed, deleted); release, `taskset -c 7`, one invocation per document and trim setting
with the process's first compile, one warmup round and two measured rounds; no tuning or retry.
The figures are transcribed in the table below; the raw logs (attempts 1 and 3) were session-local
and were not retained.

| document | cold compile (ms, M1/M2) | rebuild (ms) | warm compile (ms) | designs, cold / rebuild, warm |
|---|---|---|---|---|
| 9 strips | 1.63 / 1.66 | 1.31 / 1.30 | 1.47 / 1.46 | 1 / 0 |
| 64 strips | 26.79 / 26.96 | 12.38 / 12.24 | 14.22 / 13.94 | 64 / 0 |

The rows match attempt 1's (9 strips 1.63 / 1.65, 1.29 / 1.28, 1.44 / 1.43; 64 strips 26.90 /
26.84, 12.33 / 12.31, 14.21 / 14.01) within run-to-run noise; trimming off the same (9 strips
1.67 / 1.63, 1.29 / 1.29, 1.46 / 1.44; 64 strips 27.07 / 26.93, 12.95 / 12.32, 14.68 / 14.67). The
rebuild now reads the sorted-slice seed and no longer moves a session map, and it is no slower.
First compile in a fresh process: 9 strips 2.34 ms (trim on) / 2.18 (off); 64 strips 34.93 / 29.26
(attempt 1: 2.41 / 2.46 and 29.18 / 30.28; the 64-strip first compile varies by a few ms between
processes). The 65,537-strip row and its first-touch figures stay attempt 1's: the compile
and rebuild path changes do 752 binary-search lookups and one 752-entry sort against 27 s of other
work, far below that row's run-to-run spread of seconds, so a re-run cannot resolve them. Engine
cache freed at destroy over an empty engine (now 368 B; the engine gained the journal slot, +32 B):
2,608 B for 9 strips, 18,352 B for 64 strips, as before.

**n1.** `capi_resources`' `largest` chain is again the original three-element array with
`.chain([effect_controls.largest_allocation_bytes(), bound_seed_bytes])`: the `compile.rs` hunk
against the slice base is one line, with no re-indent.

**n2 (gate 5).** `a_refused_rebuild_leaves_the_seed_as_it_was` now refuses through the gate's
named route, a capi limit one byte below the rebuild's peak, not pending-plan backpressure. The
peak is the smallest `maximum_capi_retained_bytes` at which a fresh engine compiles the 64-track
document and edit A (strip 7's HPF) commits, found by bisection (admission is monotone in the cap).
At one byte below, the compile is admitted, A is refused after its successor was prepared (it
computed its 1 design), and B (strips 7 and 8 both given A's design, a 63-design seed, 240 B lower
peak) commits under the same cap and computes 1 design. The control-plane test fault is not
reachable from capi's integration tests without a new public hook, and the same session's cap is
fixed, so a re-submit of A cannot commit; B is the committing edit. Mutation: the rebuild writes its
record into the newest seed before commit (`self.newest_providers_mut().bound_seed =
bound_seed.clone()` after `prepare_runtime`): the test red (`committed.designs` 0, not 1), the
other four `design_bound_cache` tests green. Test value: a rebuild that writes its designs into the
newest seed before its commit.

**n3. Edits outside the listed paths (each small and necessary).** `crates/builtins/src/lib.rs`
(re-export of `InputBoundRecord`; `get` became `lookup`, `as_deref_mut` at the call); `crates/
host-core/src/lib.rs` and `crates/control-plane/src/lib.rs` (re-exports, so capi and control-plane
need not depend on `builtins`); `crates/capi/src/runtime/mod.rs` (the `compile_children` wrapper's
parameter). Attempt 1 names most of them.

**Gates (all on this tree).** `cargo fmt --check`; `clippy --workspace --all-targets --all-features
-D warnings`; `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`; `cargo test` for
`builtins` (test-support), `builtins-compiler`, `host-core`, `control-plane` (test-support) and
`capi`, all suites (63 suites ok; `design_bound_cache` 5, `resource_lifecycle` 13, 36 s);
`check-workspace-policy.sh`, `check-realtime-policy.sh`, `check-cross-targets.sh` (PASS),
`check-capi-abi.sh` (ok); `target/release/audit capi`: 0 allocations, 0 syscalls, `pcm_digest`
`cb10fbface44a3a4`. Browser: `host-web` depends on neither `control-plane` nor `capi` and
`builtins` is unchanged since `e44b8e8b2`, so the shipped module sha256 stays
`f9e711daba98d87aa81d5a8ebfceafa25a9ea2fd2ff20091bc58e4faf71cda41`; the worklet chain was not
re-run.
