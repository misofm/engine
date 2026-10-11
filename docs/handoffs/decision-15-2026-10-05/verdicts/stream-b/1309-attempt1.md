PASS

# #1309 attempt 1 -- adversarial verdict

Commit under review: `18f3ca30c` (branch `codex/d15-stream-b`), parent `d6fa539a1`. Gates were run on
the merged tree `8c2d2c3df`, which is what will ship. I reviewed `git diff -M d6fa539a1 18f3ca30c`
against the spec body (D1-D12, Amendment 1, Amendment 2, Attempt record), AGENTS.md, decision 15
(D15-11) and the no-shortcuts principle. Every build, gate and mutation ran in a private export
(`/tmp/claude-1002/v1309/`), never in the shared worktree.

No BLOCKER or MAJOR finding. The extraction is a move. Every fallible check in `command` and
`commit_live` is in its old order. Every diagnostic site maps to its old code. The rendered bits and
the resource accounting are the same, except the one ruled exception, which is exactly 24 bytes,
computed from the types. The C ABI suite is unchanged and green. Every spec gate is green on the
merged tree. The new test is red on M1 and nothing else catches M1.

## The merge (`8c2d2c3df`)

`git diff 0a1176b3b 8c2d2c3df` (origin/main against the merged head) is exactly stream B's change,
and nothing else. Main's stream-J edits to `crates/capi/src/ffi.rs` (a test only),
`crates/capi/tests/resource_lifecycle.rs` (budget prose and four-lane ceilings) and
`crates/capi/src/runtime/tests.rs` merged cleanly beside the moved code. The new
`check-realtime-policy.sh` scans `REALTIME_POLICY_BEGIN` regions and `crates/capi/src/ffi.rs`. No
moved file carried a marker, so the move takes no coverage away. The policy passes (89 regions in 25
files). The CI path router classifies every `crates/` path as `full`, so a change only under
`crates/control-plane/` cannot skip the capi audit or the ABI jobs. The merge broke nothing.

## Findings

### MINOR

1. **The D6 byte table is defended only for `Limit`** (`crates/capi/src/runtime/error.rs:93-108`).
   I swapped the strings for `ResourceFault::Arithmetic` and `ResourceFault::Platform` in
   `failure_bytes` (M5). The full capi suite stayed green: 72 lib, 2 race and 11 lifecycle tests
   pass. No test in capi, its tests or `tools/audit` asserts `capi.resource.arithmetic`, `.platform`,
   `.allocation`, `capi.protocol.queue`, `capi.plan.exchange` or `capi.source.epoch`. Only
   `capi.resource.limit` is asserted (4 sites). The bytes are correct at this commit. I checked this
   statically. I extracted the ordered code sequence of every `failure(...)` and `diagnostic(...)`
   site in base `compile.rs`, `control.rs` and `plan.rs`, and compared it with the head sequence
   after the table's mapping. The sequences are identical, apart from three expected sites:
   `boxed_zeroed`'s `platform` and `allocation` (formerly `FixedBytes::try_new`, same codes), and the
   new overflow check on `checked_sum(adapter_allocation_rows)` (`arithmetic`, the code the
   combined sum had before). The table strings are the old literals byte for byte. But the move
   sends every site through one new, untested table. A typo or a swap in it now changes C ABI
   diagnostics everywhere, and no gate sees it. The spec froze "Otherwise no new test", so this is a
   question for root, not a defect of the attempt. Suggestion: a follow-up test that drives one real
   `arithmetic` and one real `allocation` refusal through `miso_engine_v1_compile_session`, which
   extreme but valid limits reach on 64-bit. Or pin the table: these strings are ABI-visible
   diagnostic vocabulary, which AGENTS.md permits pinning.

### NIT

1. **Open specs anchor code in the moved files.** At least #1305, #1313, #1325, #1326, #1352,
   #1357, #1363, #1365, #1371, #1390, #1394 and #1404 cite `crates/capi/src/runtime/{control,
   compile,plan}.rs` line numbers. #1326 authorizes "`crates/capi/src/runtime/compile.rs` (the one
   constant line)". `C_ABI_LIVE_LANES` is now at `crates/control-plane/src/compile.rs:25`. Root
   should re-anchor these specs before they are briefed. They are outside #1309's authorized paths.
2. **The header comment at `scripts/check-host-core-policy.sh:13` was not reflowed.** It is now 123
   characters wide. This is cosmetic only.
3. **`PlanState::owner()` is broader than its callers need** (`crates/control-plane/src/plan.rs:196`).
   `plan_carry_counts` and `plan_replacement_count` read only `carried_count`,
   `carry_mismatch_count` and `active_epoch`. Every `&self` method of `RealtimePlanOwner` is a pure
   read (`active_epoch`, `active_plan_id`, `deferred_count`, `carried_count`,
   `carry_mismatch_count`, `next_absolute_sample`). Because `Cell<bool>` makes `PlanState` `!Sync`,
   the reference cannot cross threads. So the accessor is safe and justified. Narrow accessors would
   expose less to the browser adapter. Acceptable as it is.

## Checks with no finding

- **Acked-batch question (can an ack precede a drop?).** No queue changed. I extracted `command`,
  `commit_live`, `live_admission`, `synchronize_plan_epochs` and `promote_pending_provider` from base
  and head and diffed them. The only differences are `cfg(test)` -> `cfg(feature = "test-support")`,
  `prepare_runtime::<A>`, `reports.push((epoch, A::Row::from(resources)))` (an infallible
  conversion, inside the no-fail window, after the protocol commit, the same position as before),
  `report: PlanResources::from(report)` in `live_admission`, and visibility. `commit_live` is
  291 lines at both, with one line changed (the cfg).
- **Diagnostics bytes.** `diagnostic(code)` is `format!("{code}\t$\n")`, as the old `failure`.
  `session_diagnostics` and `prepare_failure` moved verbatim. `graph`/`source`/`effect.resource.limit`
  are `CompileFailure::Diagnostics` (Amendment 1). `SourceFailure::report` moved verbatim to capi's
  `SourceFailureReport` (`capi.source.epoch`). `render_error_code` is the old in-`render` match,
  including the `_ => PLAN_REJECTED` arm. `RENDER_DIAGNOSTIC_CODE` keeps `capi.render.activity`
  (D7). It is the only `"capi…"` literal in the crate.
- **`session_error` ordering (Amendment 1).** At base, `FixedBytes::try_new(maximum_diagnostic_bytes)`
  was already the last fallible step (the second-last struct field, before the infallible
  `PlanState::new`). capi now builds it right after the control plane returns, so the failure order
  is unchanged. The only difference is that on that path a `PlanState` is now constructed and then
  dropped. The test-only lifecycle counters see this, and no test drives the path. The result bytes
  are the same.
- **Limits converted once.** `miso_engine_v1_compile_session` -> capi `compile_children`
  (`crates/capi/src/runtime/mod.rs:98`) converts `ControlLimits::from(limits)` once. `SessionState`
  stores `ControlLimits`. The `prepare_caps` wrapper in capi is `#[cfg(test)]`. `ControlLimits` and
  `PlanResources` are field for field the ABI structs minus the header and reserved words, with the
  two renames and `output_tail: TailSamples`. I checked this by extracting the field lists.
- **Adapter trait (Amendment 2).** `ControlAdapter` is exactly `type Row: From<PlanResources> + Copy;
  const ADAPTER_ALLOCATIONS: &'static [u64]`, with `PlanResources: From<A::Row>` as a where clause on
  the `SessionState` impl. `Layout::array::<T>(1).size() == size_of::<T>()`, so the two adapter rows
  have their old values. The sum overflow and the largest-allocation fold are order-independent.
- **`A::Row: Send + 'static`** (`crates/control-plane/src/compile.rs:705`) is required. The
  `Arc<SharedPlanState<Row>>` becomes `Arc<dyn host_core::PlanSampleSource>` (`'static`), and
  `Mutex<Vec<(u64, Row)>>` is `Sync` only for `Row: Send`. Putting the bound on `compile_children`
  keeps the trait text exactly as Amendment 2 wrote it. Justified.
- **New production accessors.** `owner()` (NIT 3) and `fp_env_attested()`/`attest_fp_env()`
  (`plan.rs:213`, `:218`) replace direct field reads by capi's FFI. The fields are no longer
  reachable from capi, and the #146 attestation stays on the render thread in capi's render entry.
  They are justified and not render-side locks.
- **`cfg_attr(not(feature = "test-support"), allow(unreachable_pub))`.** It is on exactly the
  test-exposed items: `LIVE_QUEUE_DEPTH`, `C_ABI_LIVE_LANES`, `CapiResources`,
  `CompiledModelAdmission`, `LiveEpochResources`, `validate_live_peak`, `prepare_caps` and
  `ProviderEpoch`. Without the feature they are re-exported nowhere, so they are crate-private in
  effect, and each one says so in its doc. Feature-gating visibility otherwise needs duplicate
  definitions or a proc-macro dependency. Acceptable. D8 holds: the moved `#[cfg(test)]` observers
  and hooks are all `#[cfg(feature = "test-support")]`, tests read through `test_*` accessors, and
  no production field became public.
- **test-support does not reach release.** `cargo tree -p capi -e normal,features -i control-plane`
  and `cargo tree -p audit -p bench -p capi -p session-validator -e normal,features -i control-plane`
  (CI's release set) both show only `control-plane feature "default"`. capi enables
  `control-plane/test-support` only in `[dev-dependencies]` (`crates/capi/Cargo.toml:26`). The
  workspace resolver is 3.
- **No render-side lock.** `PlanState::render` lost only its error mapping. It takes no lock and
  calls `render_contiguous` then atomic stores, as before. The `Mutex` is read only by
  `active_resources` (queries) and the control thread.
- **Gate 2 diffs.** The base and head test-name sets are identical plus
  `a_stored_report_row_reads_back_as_the_resources_it_was_built_from`. The per-file counts match
  (`abi.rs` 2, `ffi.rs` 16, `tests.rs` 23, `live_tests.rs` 29, live-peak 1, race 2, lifecycle 11).
  `tests.rs`/`live_tests.rs` diffs are accessor rewrites plus one panic-message `{code:?}`.
  `crates/capi/tests/` is unchanged by stream B. `live_peak_tests` is unchanged apart from
  indentation, `.into()` on the report row and limits, and `failure_bytes(failure)` for
  `failure.diagnostics`. I checked this with an indentation-insensitive diff.
- **D9/D10/D11/D12.** capi's normal dependencies are `control-plane`, `engine`, `effect-contract`
  and `lane`. `host-core`, `protocol`, `session` and `source` moved to dev-dependencies, which only
  tests name. The host-core policy reads the control-plane manifest, counts control-plane's edge,
  and scans `crates/control-plane/src`. A real-tree mutation (appending a `compile_session(...)`
  call to `crates/control-plane/src/compile.rs`) fails it, and reverting passes. The self-test keeps
  every old leg and adds six. The D11 row is last in the simd128 section, and it produced
  `libcontrol_plane-*.rmeta` for `wasm32-unknown-unknown/simd`, with no `libcapi` in that target
  dir. The D12 feature is added to `test-debug-a`.
- **Authorized paths.** Only the spec, `qualification.yml` (D12 line), `Cargo.toml` (member and
  workspace dependency), `Cargo.lock` (control-plane package and capi's edge), `crates/capi/**`,
  `crates/control-plane/**` and the three named scripts were touched.
  `docs/C_ABI_V1_QUALIFICATION.md` names no moved path (it cites `runtime::tests`, which stays in
  capi).

## Test value

- `runtime::report_row_tests::a_stored_report_row_reads_back_as_the_resources_it_was_built_from`
  (`crates/capi/src/runtime/mod.rs:300`). It is red when the stored-row readback
  (`PlanResources: From<PlanResourceReport>`) drops, swaps or mis-maps a field or the tail kind,
  for example collapsing every tail to `Finite` (M1). No existing test catches M1. Neither
  admission reads `output_tail` today, so M1 has no other observable effect yet. The test defends
  the lossless contract that `ControlAdapter::Row` documents and that the spec's Amendment 1
  requires.

## Mutation runs (full `cargo test --locked -p capi --no-fail-fast`, private tree)

| run | change | result |
|---|---|---|
| M1 | readback tail always `TailSamples::Finite(report.tail_samples)` | only the new test red (71/72 lib, 2/2 race, 11/11 lifecycle) |
| M4 | `From<CompileLimits>` swaps `maximum_effect_state_bytes` / `maximum_effect_scratch_bytes` | `double_live_oracle_drives_exact_and_one_below_c_caps` and `reference_session_retained_rows_stay_within_their_budgets` red: the D3 conversion is defended end to end |
| M5 | `failure_bytes` swaps the arithmetic and platform strings | all green (MINOR 1) |
| revert | none | all green (72, 2, 11) |

## Gates run (merged tree `8c2d2c3df`, `CARGO_TARGET_DIR=/tmp/claude-1002/v1309/target`)

1. **Same bits, same accounting.** The base is origin/main `0a1176b3b`. Code-wise it is
   `d6fa539a1` merged with main, because `8c2d2c3df` minus `0a1176b3b` is exactly stream B's diff.
   Main moved no resource row: my `0a1176b3b` capture is line-identical to the implementer's
   `d6fa539a1` capture (`base2-resource-serial.txt`).
   - `cargo build --locked --release -p audit -p capi && target/release/audit capi`: the base and
     head lines are identical. `pcm_digest` is `75f4e6c21da7a2a7`. Allocations, deallocations,
     locks, syscalls and total_violations are all 0.
   - `cargo test --locked -p capi --test resource_lifecycle -- --nocapture --test-threads=1`, with
     RaceCounts lines excluded. Every row is identical except the capi-retained rows, and each of
     those is exactly 24 lower: the observed capi values 274153/145536/132304/151394 ->
     274129/145512/132280/151370; `double-live requirement` 174780 -> 174756; `live peak`
     413237 -> 413213; `capi_retained_bytes` 274153 -> 274129 of 282432. 11 passed at both.
   - A scratch probe (not committed) computed the difference from the types at the head:
     `size_of::<CompileLimits>()` = 208, `size_of::<ControlLimits>()` = 184, difference 24. On
     the `Session` side, `size_of::<capi::Session>()` is 6624 at base and 6600 at head (-24).
     `size_of::<capi::Plan>()` is 416 at both.
2. `cargo test --locked -p capi --no-fail-fast`: 72 lib + 2 plan_swap_race + 11 resource_lifecycle
   tests pass.
3. `bash scripts/check-capi-abi.sh`: ok (shared and static). `--self-test`: ok.
4. `bash scripts/check-cross-targets.sh`: PASS, with the D11 row executed.
5. `check-host-core-policy.sh`, `test-host-core-policy.sh`, `check-workspace-policy.sh` and
   `check-realtime-policy.sh` (89 regions / 25 files): ok. `check-test-support-ci.py` (11
   packages), `check-release-shape.py` and `check-ci-path-routing.py`: ok.
6. `cargo test --locked -p control-plane --features test-support` (0 tests, ok);
   `cargo fmt --all -- --check` (ok);
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` (148 crates, ok);
   `cargo clippy --locked -p control-plane --all-targets -- -D warnings` (ok).

Evidence files: `/tmp/claude-1002/v1309/evidence/` (gate logs `g-*.txt`, `base-*/head-*` gate-1
captures, mutation logs `m*.txt`).
