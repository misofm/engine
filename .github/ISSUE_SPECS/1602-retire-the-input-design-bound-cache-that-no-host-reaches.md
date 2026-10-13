# Retire the input-design bound cache that no host reaches

Successor of #1556, decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`). Code anchors verified on `82b1e05e1` (branch codex/controls-batch-3). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

The engine keeps no cache of input-section design bounds. `InputBoundCache`, its journal, its seed
record and every parameter, field, resource row, test and document line that serves them are
gone. A C ABI engine owns no control-thread memory outside its compile limits, and its header says
so. Every prepared value, report and rendered bit stays the same, because no host passes a design
to the cache today.

## Context

- **Why the cache is dead.** Since #1556 every C ABI plan carries the live input lane:
  `controls::live_lanes` sets `strip_input` when any strip has a live input-family row
  (`crates/controls/src/live_lanes.rs:26-41`, `:50-56`), and the core attaches it on every compile
  and rebuild (`crates/control-plane/src/compile.rs:597`). A strip with the live lane reports the
  rate's live bound and designs none. `crates/capi/tests/design_bound_cache.rs:305`
  (`a_c_abi_plan_bounds_no_input_design`) asserts 0 designs and 0 frames through the exported C
  ABI, and `crates/capi/tests/resource_lifecycle.rs:1799-1806` asserts a seed of 0 bytes.
  `docs/C_ABI_V1_QUALIFICATION.md:639-644` already calls the machinery dead.
- **The browser never used the cache.** `hosts/host-web/` names no `InputBoundCache` and no
  `bound_cache`. Its three preparation calls (`hosts/host-web/src/lib.rs:7193`, `:7210`, `:7226`)
  go through host-core entries that pass `None`. Today the browser still designs input bounds
  without a cache: `live_lanes` (`hosts/host-web/src/lib.rs:7850-7856`) picks
  `HostLiveLanes::FADER_AND_MATRIX` unless the boot word `live_control_command_queue_records`
  enables commands. #1562 D5 makes the browser attach `controls::live_lanes(model)` on every plan,
  and #1565 D4 deletes the boot word. After both, no host designs an input bound in production.
- **The cache in `builtins`.**
  - Type, cap, journal and seed: `crates/builtins/src/tail.rs:396-590`
    (`INPUT_BOUND_CACHE_ENTRIES` `:408`, `InputBoundCache` `:427-435`, `CachedDesign` `:439-445`,
    `InputBoundRecord` `:453-483`, `begin_journal` `:543`, `take_journal` `:553`, `lookup` `:563`,
    `insert` `:575`, `Default` `:586-590`). Doc mentions at `tail.rs:6`, `:221`.
  - Re-exports: `crates/builtins/src/lib.rs:47` (`CachedDesign`), `:49-50`.
  - The read and fill: `input_section_bounds` and `input_section_bounds_within`
    (`crates/builtins/src/lib.rs:3763-3840`) take `cache: Option<&mut InputBoundCache>`;
    `test_support::input_section_bounds_within` (`:6034-6041`) passes it on.
  - `InputBoundKey` and `input_bound_key` (`tail.rs:161-177`) also key the per-preparation
    `designs` map (`lib.rs:3793`). They stay.
- **The cache in `builtins-compiler`.** Import `crates/builtins-compiler/src/lib.rs:28`; the
  parameter of `prepare_session_builtins_with_live_controls` (`:4432-4453`) and of its inner
  function (`:4548`); the pass to `input_section_bounds` (`:4681-4703`); the test
  `a_preparation_walks_at_most_the_budget_and_a_warm_cache_walks_nothing` (`:13713`, cache legs
  at `:13746`, `:13776`).
- **The cache in `host-core`.** Re-export `crates/host-core/src/lib.rs:238-241`; import
  `crates/host-core/src/prepare.rs:11`; the `bound_cache` parameter of
  `prepare_host_runtime_with_live_lanes` (`:1020-1050`) and `_successor` (`:1056-1078`), of the
  policy function (`:1494`) and its pass (`:1965`); the test
  `a_design_bound_cache_changes_no_prepared_value` (`:3466-3594`). Callers in
  `crates/host-core/tests/{effect_observation,live_lanes,live_transaction,prepare}.rs` pass `None`.
- **The cache in `control-plane`.** Re-export `crates/control-plane/src/lib.rs:66-68`.
  `crates/control-plane/src/compile.rs`: `PreparedRuntime::bound_seed` (`:29`);
  `capi_resources`'s `bound_seed_bytes` parameter (`:164`), its epoch row (`:227-229`) and its
  largest-allocation term (`:286`); `prepared_capi_resources`'s parameter (`:311`, `:350`);
  `prepare_runtime`'s `bound_cache` with `begin_journal` and `take_journal` (`:574-618`, `:642`,
  `:715`); `compile_children`'s `engine_cache` (`:723-761`, `:801`, `:877`).
  `crates/control-plane/src/control.rs`: the epoch's `bound_seed` (`:45-49`), `test_bound_seed`
  (`:89-92`), the constructors (`:101-111`, `:125-135`) and the rebuild's
  `InputBoundCache::with_seed` (`:1146-1183`).
- **The cache in `capi`.** `Engine::bound_cache` (`crates/capi/src/abi.rs:422-436`); the compile
  call (`crates/capi/src/ffi.rs:383-387`); `engine_cache` (`crates/capi/src/runtime/mod.rs:97-107`).
  Test helpers that call the live-lanes entries: `crates/capi/src/runtime/live_tests.rs:2566`,
  `crates/capi/src/runtime/tests.rs:3008`. `crates/capi/Cargo.toml:24` names the cache test.
- **Resource rows and tests in `capi`.** `crates/capi/tests/resource_lifecycle.rs`:
  `three_cutoff_session` (`:523-532`), `HostHalf::bound_seed_bytes` (`:682-703`), its term in
  `capi_epoch_terms` (`:712-713`, `:743`), the "three input cutoffs" case (`:1001-1003`) and the
  zero-seed assertion (`:1799-1806`). `crates/capi/tests/design_bound_cache.rs` (whole file).
- **Header and documents.** The "Engine memory (issue 1471)" paragraph of
  `crates/capi/include/miso_engine_v1.h:120-134` states a 4,300,720-byte cache and a 947,248-byte
  open journal outside every compile limit. `docs/C_ABI_V1_QUALIFICATION.md:637-690` ("Design-bound
  caches (#1471)"). `docs/REALTIME_DEPENDENCY_POLICY.md:206`, `:352`.
  `tools/realtime-policy/src/policy.rs:172` lists the test file.
- **Other users.** `crates/builtins/examples/input_bound_budget.rs` (`:19`, `:90`, `:289`, `:324`,
  `:560-565`) measures a warm rebuild. `crates/builtins/tests/tail_contract.rs` uses the cache at
  `:437-466` and in four cache tests (`:578`, `:600`, `:634`, `:680`).
- **History.** #1457 shipped the cache; #1471 (spec removed at `1d71fe7d0`; read it with
  `git show 1d71fe7d0^:.github/ISSUE_SPECS/1471-wire-the-engine-and-session-design-bound-caches-into-the-c-abi.md`)
  wired the engine cache and the per-epoch seed into the C ABI. The open #1470 (*Cache design
  bounds across browser preparations in the control Worker*) plans the same cache for the browser.

## Decisions frozen for this slice

- **D1. The cache is retired, not kept for later.** No host reaches it after #1562 and #1565, and
  a live input lane on every strip is decision 17's model (C6). A host that needs fast bounded
  designs again re-earns a cache in a new issue.
- **D2. Removed from `builtins`.** `InputBoundCache`, `InputBoundRecord`, `CachedDesign`,
  `INPUT_BOUND_CACHE_ENTRIES`, the journal and the seed. `input_section_bounds(rate, strips)` and
  `test_support::input_section_bounds_within(rate, strips, budget)` lose the `cache` argument.
  The per-preparation dedupe by `InputBoundKey`, the budget, the stopped walk and the live-bound
  fallback stay unchanged.
- **D3. Removed parameters.** `prepare_session_builtins_with_live_controls`,
  `prepare_host_runtime_with_live_lanes`, `prepare_host_runtime_with_live_lanes_successor`, the
  host-core policy function, `prepare_runtime` and `compile_children` lose their cache parameter.
  No replacement variant is added. Every caller drops the argument.
- **D4. Removed state and rows.** `Engine::bound_cache`, `PreparedRuntime::bound_seed`, the epoch's
  `bound_seed` and `test_bound_seed`, and the `bound_seed_bytes` epoch row and largest-allocation
  term in `capi_resources`. The rebuild prepares with no seed.
- **D5. The C ABI guard stays.** `crates/capi/tests/design_bound_cache.rs` becomes
  `crates/capi/tests/input_design_bounds.rs`. It keeps `a_c_abi_plan_bounds_no_input_design`
  (compile and rebuild bound 0 designs and walk 0 frames) and drops every cache word.
- **D6. Header.** The "Engine memory" paragraph says the engine owns no control-thread memory
  outside the compile limits beyond its own handle. The header's ABI, layout and symbols do not
  change; `ABI_VERSION` does not change.
- **D7. #1470 is superseded.** The browser has no designs left to cache after #1562 D5. The
  coordinator closes #1470 as superseded when this slice's evidence is upstream.

## Deliverables

1. D2 in `crates/builtins` (source, `tests/tail_contract.rs`, `examples/input_bound_budget.rs`).
2. D3 in `crates/builtins-compiler`, `crates/host-core` and every caller.
3. D4 and D5 in `crates/control-plane` and `crates/capi`.
4. D6 in `crates/capi/include/miso_engine_v1.h`; the documents and the allow-list follow.
5. The rewritten tests of the gates below.

## Authorized paths

- `crates/builtins/src/{tail.rs,lib.rs}`, `crates/builtins/tests/tail_contract.rs`,
  `crates/builtins/examples/input_bound_budget.rs`.
- `crates/builtins-compiler/src/lib.rs`.
- `crates/host-core/src/{prepare.rs,lib.rs}`,
  `crates/host-core/tests/{effect_observation,live_lanes,live_transaction,prepare}.rs`.
- `crates/control-plane/src/{compile.rs,control.rs,lib.rs}`.
- `crates/capi/src/{abi.rs,ffi.rs,runtime/mod.rs,runtime/live_tests.rs,runtime/tests.rs}`,
  `crates/capi/tests/{design_bound_cache.rs,input_design_bounds.rs,resource_lifecycle.rs}`,
  `crates/capi/Cargo.toml` (comment only), `crates/capi/include/miso_engine_v1.h` (comments only).
- `tools/realtime-policy/src/policy.rs` (the renamed path only).
- `docs/C_ABI_V1_QUALIFICATION.md`, `docs/REALTIME_DEPENDENCY_POLICY.md`.

## Non-goals

- Removing the design-bound computation itself (`input_section_bounds`, `fixed_input_bound`, the
  budget, `INPUT_BOUND_SECTION_CHARGE`, `INPUT_BOUND_BUDGET_FRAMES`). Host-core's entries with
  `HostLiveLanes::FADER_AND_MATRIX` and the no-lane entries still reach it. Whether those entries
  and the computation retire is a separate successor, after #1565.
- Any change to host-web. It never held the cache.
- Any change to the live bound, the tail law or a prepared value.
- Editing closed handoff records under `docs/handoffs/`.

## Objective gates

Commands: `cargo test --locked -p builtins --features test-support`,
`cargo test --locked -p builtins-compiler`, `cargo test --locked -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo build --locked --release -p builtins --features test-support --example input_bound_budget`,
`scripts/check-workspace-policy.sh`, `cargo test --locked -p realtime-policy`,
`cargo build --locked --release -p audit -p capi -p realtime-policy` then
`./target/release/realtime_policy` and `./target/release/audit capi`.

1. **No cache left.** `git grep -n -E 'InputBoundCache|InputBoundRecord|CachedDesign|INPUT_BOUND_CACHE_ENTRIES|begin_journal|take_journal|bound_seed|bound_cache|engine_cache'`
   over `crates/`, `hosts/`, `tools/` and `docs/*.md` returns nothing. This is a review check, not
   a committed test.
2. **C ABI guard.** `input_design_bounds.rs`'s `a_c_abi_plan_bounds_no_input_design` passes: a
   compile and a structural rebuild of the 64-track fixture bound 0 designs and walk 0 frames.
3. **Budget law unchanged.** `tail_contract.rs`'s
   `the_live_bound_is_taken_exactly_when_the_budget_is_exhausted` and
   `a_design_walks_at_most_the_budget_it_is_given` pass with the cache legs removed and every
   expected value unchanged. The four cache-only tests (`:578`, `:600`, `:634`, `:680`), the
   builtins-compiler warm-cache test and host-core's `a_design_bound_cache_changes_no_prepared_value`
   are deleted; the builtins-compiler test keeps its budget leg under a new name without "cache".
4. **Resource accounting complete, ceilings lowered.** `resource_lifecycle.rs`'s complete-charge
   checks (`assert_capi_retained_bytes_are_complete`, `assert_host_owners_are_charged`) pass for
   every fixture with the seed row gone from both the engine's sum and the test's
   `capi_epoch_terms`. The "three input cutoffs" case and `bound_seed_bytes` are deleted. The
   header and `C_ABI_V1_QUALIFICATION.md` no longer state the 4,300,720-byte cache ceiling or the
   947,248-byte journal: the engine's memory outside the compile limits drops from those figures to
   its handle alone.
5. **Realtime.** `audit capi` reports 0 allocations, frees, locks and syscalls on render.
6. **No bit moved.** Before merge, the implementer runs `cargo test --locked -p capi` and
   `cargo test --locked -p host-core --all-features` on the base and on the change; every render
   comparison test passes on both. This is PR evidence, not a committed test.

## Test value

- Gate 2 turns red if a C ABI plan stops attaching the live input lane and designs bounds again
  with no cache to absorb the cost.
- Gate 3 turns red if removing the cache argument changes the dedupe, the budget charge or the
  live-bound fallback.
- Gate 4 turns red if the seed row is removed from the engine's sum but a retained owner is left
  uncharged, or the reverse.
- Deleted tests defend only the cache; their claims leave with it.

## Dependencies

- #1556 (every C ABI strip carries the live input lane), on `codex/controls-batch-3`.
- #1562 D5 (the browser attaches the core's live lanes) and #1565 (deletes the boot word). The
  cache retirement does not need them to be correct, because the browser never held the cache.
  The order follows the brief and the shared file `crates/host-core/src/lib.rs` (#1565 D5 edits
  its re-exports). D7's closure of #1470 needs #1562 D5 on `main`.
- Check open specs that edit `crates/host-core/src/prepare.rs` or the live-lanes entries (#1593,
  #1401) and declare an edit order before the slice starts.

## Size

Half a day to one day. Most of the work is deletion and dropping one argument at each caller.
