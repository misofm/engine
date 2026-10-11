PASS

# #1469 attempt 1 -- adversarial verification

Branch `codex/d15-stream-g2`. I reviewed `git diff 34cd5d55f f49c62bfa` (commits 1977ffbf0, 996a65576,
603a6893a, f6b765d2d, be3bad451, bdd57e245, fb432bccc and f49c62bfa). I built an export of f49c62bfa
(`/tmp/claude-1002/v1469/tree`, `CARGO_TARGET_DIR=/tmp/claude-1002/v1469/target`,
`CARGO_INCREMENTAL=0`), a second export for the mutation runs (its own target dir) and a base
export of 34cd5d55f (its own target dir). I reviewed against the spec (D0-D7, Amendment 1 and its
named exceptions, stops 1-3, the expected-value ruling), AGENTS.md, decision 15, the STREAMS rows
and the owner principle.

No BLOCKER and no MAJOR. One MINOR and three NITs.

## Findings

### BLOCKER
None.

### MAJOR
None.

### MINOR

**m1. `NativeEffectRegistry` lost its doc comment to the test-only counter.**
`crates/effect-contract/src/lib.rs:2506-2527`. The counter and its reader were inserted between the
registry's doc block ("The native effects a host can prepare, ... never a processor.", :2506-2510)
and `pub struct NativeEffectRegistry` (:2527). A doc comment attaches to the next item, so that
text now documents `#[cfg(feature = "test-support")] static TAIL_BOUND_EVALUATIONS` (:2515). Without
`test-support` the cfg removes it with the static, so the public struct has no documentation in any
build. The rendered rustdoc confirms this: `target/doc/effect_contract/struct.NativeEffectRegistry.html`
no longer contains the text. With the feature, a private static carries the registry's
description. The crate has `#![allow(missing_docs)]`, so no gate sees it. Fix: move the static and
`tail_bound_evaluations()` above `/// The native effects a host can prepare` (or below the struct).
This changes no behaviour.

### NIT

**n1. The STREAMS row does not name all of Amendment 1's edits.**
`docs/handoffs/decision-15-2026-10-05/STREAMS.md:75` describes #1469's edit to the hot file
`crates/effect-compiler/src/prepare.rs` as "its static, builder and entry point" only. Amendment 1
also adds `launch_registry_owns_factory` there (:235) and changes the factory-charging loop of
`effect_control_resources` (:774). The 2026-10-07 hot-file note (:155-162) also leaves out stop 3's
`crates/host-core/tests/prepare.rs` and `crates/host-core/src/control_preparation.rs`. Neither of
those is a hot row, and the spec required a note only for stream H's files. But a slice that
rebases over `effect_control_resources` (G #1464, B #1315/#1345) is not told that the charge rule
changed.

**n2. The attempt record overstates MA's capi reach.**
`.github/ISSUE_SPECS/1469-...md:397-398` says "MA also turns three `capi` `resource_lifecycle` tests
red". At f49c62bfa, MA turns two red: `capi_retained_bytes_charge_every_byte_the_compile_retains`
and `tiny_control_frame_still_accounts_three_provider_counters_exactly`. I checked this both in a
full test-debug-a run under MA and alone, one test per process. The third test,
`exported_c_candidates_...`, was the first-build race, and it was red without MA at 603a6893a.
This is evidence wording only.

**n3. The warm-ups do not check that they built the registry.**
- `crates/capi/tests/resource_lifecycle.rs:142` discards the EQ preview's `Result` with `let _ =`.
- `crates/graph-compiler/tests/live_routes.rs:1231` does the same with the registry call.

Today the preview reaches the registry right after a nonzero shape check, so the warm-up works
(W1 below). If a later change makes the preview refuse earlier, the warm-up would stop working
without any error and the race would come back as flakiness. `.expect("the EQ preview builds the
launch registry")` would make the warm-up fail loudly instead.

## The five judged questions

1. **Is the OnceLock design sound for every host?** Yes.
   - `launch_native_effect_registry` is `get_or_init(build_...)`, and it clones the error
     (`prepare.rs:217-222`). The builder is private, and nothing can reset the registry.
   - There is a `Send + Sync` const assertion for `NativeEffectRegistry` and `RegistryError`.
   - Production calls come from three places only:
     - `host-core/src/prepare.rs:1348`, preparation on the capi control thread
       (`runtime/compile.rs`) or at browser preparation;
     - `response.rs:534`, the preview;
     - `live_delta.rs:281` and `:434`, reached from `capi/src/runtime/control.rs:1071`
       `commit_live` on the control thread.
   - No render function reaches the entry point. `check-realtime-policy.sh` passes (89 regions)
     with no allowlist edit.
   - `launch_registry_owns_factory` uses `OnceLock::get` (no wait, no build) and one `Arc`
     clone/drop.
   - Browser: wasm32 without atomics uses std's no-threads `Once`, so nothing can block. Each
     module instance has its own static. The shipped named twin contains the registry
     (`launch_native_effect_registry`, `build_launch_native_effect_registry` and
     `launch_registry_owns_factory` symbols).
   - C ABI: one static per loaded library copy.
   - Caching the error is correct. The builder's inputs are compiled-in `&'static` descriptors and
     constant factories, so a failure is deterministic. An allocation failure aborts instead of
     returning a `RegistryError`. A panic in the builder leaves the `OnceLock` empty, so it does
     not cache a bad value.
2. **Is the accounting rule correct and complete?** Yes.
   - Only two kinds of object keep an `Arc<dyn NativeEffectFactory>`: `EffectControlOwner`
     (`effect-compiler/src/control.rs:67`) and `EqTargetPreparer` (`control_preparation.rs:306`).
   - The owner walk (`prepare.rs:774`) skips a registry-owned factory. A fresh factory is still
     charged once by identity.
   - `factory_allocation_bytes` returns 0 only for the registry's own allocation.
   - host-web's EQ workspace gets a fresh `Arc::new` from `parametric_eq_target_preparation_factory`
     (`effect-compiler/src/lib.rs:28-37`), so `control_targets.rs:451,567` still charge it.
     Leaving that file unedited is correct.
   - Nothing estimates the factory before preparation (`graph-compiler/src/estimate.rs` has no
     factory term).
   - `docs/C_ABI_V1_QUALIFICATION.md:187-191` and `docs/EFFECT_CONTRACT_V1.md:107-109` both say
     that the registry's bytes are process-level and charged to no plan.
   - Both pre-existing exact-equality capi tests pass unchanged.
3. **Warm-up sweep.** I recomputed the list of allocation-counting files. It has 51 entries; the
   implementer's list had 49, and the two it left out, `hosts/host-web/src/render_lock.rs` and
   `hosts/host-web/tests/render_locked_staging.rs`, are in the binaries that were run anyway.
   - I ran every test of the 16 relevant binaries alone, one `--exact` test per fresh process, on
     the test-debug-a build: **388 ok, 0 FAIL**.
   - I also ran the `audit` and `bench` binary unit tests alone in release: **47 ok, 0 FAIL**.
   - Every `resource_lifecycle` window opens through `begin()`, which warms first (:157-166).
     `live_routes.rs` warms in `retained()` before `current_thread_counters()` (:1231-1232). Its
     other windows (:1123 render, :1343 after `compile`) do not touch the registry.
   - Removing either warm-up turns its tests red alone (W1, W2 below).
   - A window can still hold the first build only if no test that counts allocations reaches the
     registry inside a window before its own warm-up. The alone sweep is the strict case, and it
     is green. A parallel run can only move the build out of a window.
4. **Is the test-support counter kept out of product builds?** Yes.
   - `cargo tree -p capi -e features -i effect-contract` and
     `cargo tree -p host-web --target wasm32-unknown-unknown ...` show no `test-support`.
   - The shipped named twin has 0 occurrences of `tail_bound_evaluations`.
   - The only non-test path that turns it on is `tools/audit`'s existing
     `effect-compiler = { features = ["test-support"] }`, in the audit-native shard's unified
     `-p audit -p bench -p capi -p session-validator` build. That build already had test-support
     before this slice. The counter is an un-exported Rust function, so the ABI check is unchanged.
5. **Did any rendered bit move?** No.
   - `audit capi` gives `pcm_digest cb10fbface44a3a4`, 0 allocations, 0 deallocations and
     0 syscalls, on both the base (34cd5d55f, built separately) and the head.
   - The worklet chain passes, and `expected.json` is unchanged.
   - `check-graph-determinism.sh` passes 100/100.
   - `run-wasm-gates.sh` passes.

## Test value (one sentence each)

- **`the_launch_registry_is_built_once_per_process`** (`crates/host-core/tests/launch_registry_once.rs:133`):
  It goes red in these cases, and no other test counts registry builds:
  - preparation, the EQ preview or either live-classification branch builds its own launch
    registry (M2 1184, M3 1184, M4 416, M4b 416, each against 32);
  - the entry point bypasses the `OnceLock` (M1: `ptr::eq` fails in all four threads);
  - concurrent first use builds more than once (M5, racy `get`-then-`set`: 128 against 32, red in
    10 of 10 runs).
- **`effect_control_resources_charge_no_registry_owned_factory`** (`crates/effect-compiler/src/prepare.rs:1462`):
  - It goes red if `launch_registry_owns_factory` identifies a factory by effect ID instead of by
    allocation identity (MID: `.is_some()` instead of `Arc::ptr_eq`). In a full test-debug-a run
    under MID, only this test and the next one go red. The pre-existing
    `effect_control_resources_charge_actual_tables_and_shared_owner_once` stays green under MID,
    because whether it sees the defect depends on test order.
  - It also localizes MA (3238 against 3222). The two pre-existing capi exact-equality tests
    catch MA too.
- **`a_registry_owned_factory_is_charged_to_no_preparer`** (`crates/host-core/src/control_preparation.rs:507`):
  It goes red if `factory_allocation_bytes` charges the registry's own factory (MB: 16 against 0).
  No other test catches this, because host-web's workspace passes a fresh factory. It also goes red
  if the function stops charging a factory the preparer owns alone (MID: 0 against 16).
- **Rewritten expected values.** These follow the root's standing ruling; they are not new claims.
  Each goes red if a plan is charged for a registry-owned factory (MA):
  - `effect_control_browser_table_and_payload_reach_exact_budget_gate`
    (`hosts/host-web/src/tests.rs:3457`): E1, always charge, 1603 against 1619;
  - `effect_control_report_uses_actual_native_capacity_strings_and_owners`
    (`crates/host-core/tests/prepare.rs:438`): E2, always charge, 14409 against 14425.

  Both expectations use the same predicate as the implementation. The two new unit tests above
  cover a wrong predicate (MID).
- **Warm-ups** (test harness, named exceptions):
  - W1: remove the capi warm-up, and `capi_retained_bytes_...`, `exported_c_candidates_...` and
    `tiny_control_frame_...` go red alone; 8 ok, 3 FAIL.
  - W2: remove the `live_routes` warm-up, and `route_control_resources_cover_the_allocation` goes
    red alone ("attaching retains more: 51919 against 53692").
  - After each revert, every test is green alone.

Each mutant was applied in the mutation export and reverted with the original file. The gate 1
test and the focused tests were green after each revert.

## Gates run (on the f49c62bfa export)

- test-debug-a (exact qualification.yml command, `--no-fail-fast`): exit 0, 125 `test result: ok`,
  0 failed. `the_launch_registry_is_built_once_per_process ... ok` runs, and `control-provider`
  comes in through capi's dependency. host-core `tests/live_delta.rs` is unchanged and green.
- test-debug-a doctests: exit 0, 20 `test result: ok`.
- Gate 1 alone (`-p host-core --features test-support,control-provider --test launch_registry_once`):
  pass, 0.7 s.
- Alone sweep: 388 of 388 ok (debug, test-debug-a features) and 47 of 47 ok (`audit` and `bench`
  in release).
- `cargo build --release -p audit -p bench -p capi -p session-validator`, then `audit capi`:
  `pcm_digest cb10fbface44a3a4`, 0 allocations, 0 deallocations, 0 syscalls, 0 violations. The base
  34cd5d55f gives the same digest.
- `check-capi-abi.sh` (on the shard's prebuilt `libcapi.so` and `.a`): ok, shared and static.
- `cargo test --release -p audit -p bench -p console-workload`: exit 0.
- `check-test-support-ci.py`: passed. It reports `effect-contract/test-support: test-debug-a`
  among 11 packages.
- `check-workspace-policy.sh`: ok. `check-realtime-policy.sh`: ok (89 regions; no allowlist in
  the diff).
- Worklet chain, all exit 0, with nothing left in the private TMPDIR:
  - `build-web-audioworklet.sh --named-twin`: module `315a58d5...`;
  - `check-web-audioworklet.sh --without-metadata-regeneration`;
  - `check-browser-expected-resources.py --artifacts`;
  - `check-scalar-oracle-absent.py --wasm`;
  - `test-web-audioworklet.sh`.
- `check-graph-determinism.sh`: 100/100. `run-wasm-gates.sh`: ok.
- `check-cross-targets.sh`: PASS.
- `check-effect-contract.sh target/release/bench`: ok.
- fmt: pass. Clippy `--workspace --all-targets -D warnings`, with and without `--all-features`:
  pass.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`: fails only on `gate-expander`
  `corpus.rs:64`. That failure is already on the base (#1459), and this diff does not touch the
  file. With `--exclude gate-expander` it passes.
- Not run: test-debug-b. It is not in my gate list; the implementer recorded a pass. The only
  change it can see is `effect-contract`'s doc move and cfg-gated items.

Evidence (logs, mutation driver, alone-sweep results, audit JSON): `/tmp/claude-1002/v1469/ev/`.
