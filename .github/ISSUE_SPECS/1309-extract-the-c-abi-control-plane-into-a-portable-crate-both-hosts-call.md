# Extract the C ABI control plane into a portable crate both hosts call

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-11).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The control plane the C ABI runs today (session store, live classifier call, successor
preparation, plan publication and retirement, provider epochs) lives in one portable crate,
`control-plane`. `capi` becomes a thin adapter over it. The crate builds for
`wasm32-unknown-unknown` with `simd128`, so the browser can adopt the same control plane
(*Run the browser control plane in a Worker and keep the AudioWorklet render-only*, #1332).
No C ABI behaviour changes: every result code, diagnostic string, response byte and rendered
bit is the same before and after, with one measured exception (Amendment 1): the `Session` fixed
row and `capi_retained_bytes` (so the retained-bytes value `miso_engine_v1_plan_resources`
returns) fall by exactly `size_of::<CompileLimits>() - size_of::<ControlLimits>()`, because the
session stores `ControlLimits` (D3).

## Context

- The control plane is capi adapter code today (`crates/capi/src/runtime/mod.rs:1-14`):
  - `crates/capi/src/runtime/control.rs` (1,584 lines): `SessionState` (`:237`),
    `ProviderEpoch` (`:26`), `StripLanes` (`:13`), `command` (`:843`), `commit_live` (`:1065`),
    `synchronize_plan_epochs` (`:783`), `submit`/`seek`/`seek_at` (`:1501`, `:1514`, `:1530`),
    `SourceFailure` (`:1552`).
  - `crates/capi/src/runtime/compile.rs` (994 lines): `prepare_runtime` (`:572`),
    `compile_children` (`:698`), the resource projection `capi_resources` (`:140`),
    `validate_replacement_peak` (`:339`), `validate_live_peak` (`:443`).
  - `crates/capi/src/runtime/plan.rs` (251 lines): `SharedPlanState` (`:5`), `PlanState` (`:53`)
    and its render entry (`:201`), `PlanQueries` (`:82`).
  - `crates/capi/src/runtime/error.rs` (108 lines): `FixedBytes`, `plan_error` (`:48`),
    `CompileFailure` (`:89`).
- C ABI types leak into this code:
  - `CompileLimits` (`crates/capi/src/abi.rs:88`) is read field by field;
  - `PlanResourceReport` (`abi.rs:245`) is the stored report row (`plan.rs:8`);
  - `capi_resources` charges `crate::Session` and `crate::Plan` (`compile.rs:233-234`);
  - `prepare_runtime` writes `struct_size`, `abi_version` and `TAIL_FINITE`/`TAIL_INFINITE`
    (`compile.rs:648-659`);
  - 61 `failure("capi.…")` calls spell C ABI diagnostic codes (`capi.resource.arithmetic`,
    `.limit`, `.platform`, `.allocation`, `capi.protocol.queue`, `capi.plan.exchange`);
    `SourceFailure::report` spells `capi.source.epoch` (`control.rs:1563`).
- capi's runtime tests reach the state through the FFI and through `#[cfg(test)]` observers
  (45 `cfg(test)` sites in `crates/capi/src/runtime/*.rs`; `update_test_owners` `control.rs:206`,
  `test_transaction_snapshot` `:625`, fault injection `TestStructuralFaultPhase` `:104`), and read
  a few fields directly (`children.session.providers.epoch`, `pending_providers`,
  `crates/capi/src/runtime/tests.rs:641-882`, `live_tests.rs:1281-1282`).
- Every dependency builds for the browser target: `cargo check --target wasm32-unknown-unknown`
  with `-C target-feature=+simd128` of `-p protocol -p host-core --features
  host-core/control-provider -p source -p session -p engine` passes on `6fb211594`.
- `scripts/check-host-core-policy.sh:81-89` requires that exactly capi enables host-core's
  `control-provider` feature, with the exact manifest line at `:81`;
  `scripts/test-host-core-policy.sh:13-36` mutates that layout.

## Decisions frozen for this slice

- **D1. Name.** Package `control-plane`, directory `crates/control-plane`, lib `control_plane`
  (AGENTS.md naming rule; not `core`). Workspace member and `[workspace.dependencies]` entry.
- **D2. What moves.** `control.rs`, `compile.rs`, `plan.rs` and `error.rs` (except
  `plan_error` and `FixedBytes`, which are the C ABI's last-error storage and stay in capi) move
  into the crate. Items capi calls become `pub`; the rest stay private to the crate. Type and
  function names stay as they are (`SessionState`, `PlanState`, `compile_children`, …), so the
  diff is a move, not a rewrite.
  Amendment 1: `boxed_zeroed`'s use of `FixedBytes` is replaced by a crate-local zeroed-buffer
  helper, and capi builds `CompiledChildren`' `session_error` (`FixedBytes`) itself after
  `compile_children` returns, with the same codes. `PlanState::render` returns the engine's
  `RenderError`, and capi maps it to today's `plan_error` codes.
- **D3. Plain limits.** The crate defines `ControlLimits`: every numeric field of
  `CompileLimits`, same names and types, without `struct_size`, `reserved0` and `reserved`, and
  with `maximum_capi_retained_bytes` named `maximum_control_retained_bytes`. capi validates its
  struct (`limits_are_valid`, `compile.rs:510`, stays in capi) and converts once in
  `miso_engine_v1_compile_session`. `prepare_caps` (`compile.rs:523`) takes `ControlLimits`.
- **D4. Plain report, adapter-shaped rows.** The crate defines `PlanResources`: every field of
  `PlanResourceReport` except `struct_size`, `abi_version` and `reserved`; the tail is
  `effect_contract::TailSamples`; `capi_retained_bytes` is named `control_retained_bytes`. The
  report table keeps the adapter's row type, so its bytes do not change:
  `SharedPlanState<Row>` stores `Mutex<Vec<(u64, Row)>>` with `Row: From<PlanResources> + Copy`
  (`crates/capi/src/runtime/plan.rs:8` stores `(u64, PlanResourceReport)` today). capi
  instantiates `Row = PlanResourceReport` and converts once, when a row is inserted; its
  `miso_engine_v1_plan_resources` copies the stored row as today.
  Amendment 1: the crate reads stored rows back (`replacement_base_report` into
  `validate_replacement_peak`, `live_admission` into `validate_live_peak`), so the bound is
  `Row: From<PlanResources> + Copy` and `PlanResources: From<Row>`. For `PlanResourceReport` the
  conversion is lossless: `TAIL_FINITE` maps to `TailSamples::Finite(n)` and `TAIL_INFINITE` to
  `TailSamples::Infinite`, and back.
  Amendment 2: the row type is the adapter's associated type `ControlAdapter::Row` (D5), so
  `SessionState<A: ControlAdapter>` and `SharedPlanState<A::Row>` are generic over the adapter.
- **D5. Adapter allocations.** The crate's constructor takes `adapter_allocations: &[u64]`, the byte
  sizes of the fixed allocations the adapter keeps per session: capi passes `Session` and `Plan`
  (today's `compile.rs:233-234` rows). They enter the fixed allocation rows and the
  largest-allocation fold exactly where those two rows are today. The report-table row
  (`compile.rs:232`, `checked_layout::<(u64, PlanResourceReport)>(2)`) is charged from the
  adapter's `Row` type (D4), so it is the same layout and the same bytes as today. Together, D4 and
  D5 keep every fixed row, `capi_retained_bytes` and the largest-allocation fold unchanged, which
  gate 1 checks.
  Amendment 1: one exception. `SessionState` stores `ControlLimits` (D3), which is smaller than
  `CompileLimits`, so the `Session` fixed row, and with it `capi_retained_bytes`, falls by exactly
  `size_of::<CompileLimits>() - size_of::<ControlLimits>()`. Every other fixed row and the
  largest-allocation fold stay the same.
  Amendment 2: replaced as a constructor argument. The crate charges `Session` and `Plan` again
  on every structural rebuild (`command` calls `prepare_runtime`, which charges them through
  `capi_resources`), and storing the sizes in `SessionState` would grow `Session`. So the adapter
  supplies them at the type level: the crate defines
  `trait ControlAdapter { type Row: From<PlanResources> + Copy; const ADAPTER_ALLOCATIONS:
  &'static [u64]; }` with the bound `PlanResources: From<A::Row>`, and the control plane is
  `SessionState<A: ControlAdapter>`. capi implements it once (`Row = PlanResourceReport`,
  `ADAPTER_ALLOCATIONS` = the sizes of `Session` and `Plan`). Nothing is stored, the values
  cannot differ between calls, and gate 1 stays as Amendment 1 wrote it. *Prepare through an
  adapter-supplied preparer in the control-plane crate* (#1400) may later add its
  `RuntimePreparer` to this trait; this slice does not pre-build it.
- **D6. Typed failures.** The crate never spells `capi.`. `CompileFailure` becomes an enum:
  `Resource(ResourceFault)` with `ResourceFault::{Arithmetic, Platform, Limit, Allocation,
  ProtocolQueue, PlanExchange}`, and `Diagnostics(Vec<u8>)` for the session and preparation
  diagnostic lines it builds today (`error.rs:99`, `prepare_failure`). `SourceFailure::Internal`
  stays a variant without text. capi maps each to today's exact bytes (`capi.resource.arithmetic\t$\n`
  and so on; `capi.source.epoch`).
  Amendment 1: `graph.resource.limit`, `source.resource.limit` and `effect.resource.limit` (from
  `validate_replacement_peak` and `validate_live_peak`) are not resource faults of the adapter;
  they become `CompileFailure::Diagnostics` with today's exact bytes.
- **D7. The render-activity code stays.** `RENDER_DIAGNOSTIC_CODE` (`control.rs:100`) keeps the
  text `capi.render.activity`, because `host-core` sizes it by that literal
  (`crates/host-core/src/control_provider.rs:117`) and it is protocol-visible. Choosing the
  browser's code is #1332's.
- **D8. Test support.** Every `#[cfg(test)]` observer and fault hook in the moved code becomes
  `#[cfg(feature = "test-support")]` in the crate. capi's `[dev-dependencies]` enable
  `control-plane/test-support`. Fields capi's tests read get `test-support` accessors; no field
  becomes public API. capi's tests stay in capi with unchanged assertions.
- **D9. Edges.** `control-plane` enables `host-core/control-provider`; capi depends on
  `control-plane` and keeps only the dependencies its remaining code names. The host-core policy
  moves with the edge (D10).
- **D10. Policy.** `scripts/check-host-core-policy.sh`: the exact-line check at `:81` reads
  `crates/control-plane/Cargo.toml`; the occurrence count at `:86-89` names control-plane, not
  capi; `crates/control-plane/src` joins `host_sources` (`:39`), so the crate is held to "no
  direct compile-pipeline call, no hand-decoded wire". `scripts/test-host-core-policy.sh` mirrors
  the new layout and keeps every mutation leg.
- **D11. Browser target row.** `scripts/check-cross-targets.sh` gains one row in the
  `simd128` section (after `:200`): `cargo check --quiet --locked --target wasm32-unknown-unknown
  -p control-plane`. It never builds `capi`.
- **D12. CI feature list.** The `test-debug-a` step in `.github/workflows/qualification.yml`
  (`:609-618`) adds `control-plane/test-support` to its `--features` list, as
  `scripts/check-test-support-ci.py` requires. Amendment 3: `scripts/test-test-support-ci.py`
  keeps a literal copy of that list as its mutation anchor (`DEBUG_A_FEATURES`); it gains
  `control-plane/test-support`, and its "every test-support feature removed from test-debug-a"
  case expects `control-plane` too, so the lint job's mutation suite stays green.

## Deliverables

1. `crates/control-plane` with the moved code, `ControlLimits`, `PlanResources`, typed failures,
   the `test-support` feature and a crate doc that names its job and its two adapters.
2. capi reduced to the FFI, its ABI types, `plan_error`, `FixedBytes`, the conversions of D3, D4
   and D6, and its tests.
3. The manifest, policy, cross-target and CI edits of D9-D12.

## Authorized paths

- `crates/control-plane/**` (new).
- `crates/capi/**`.
- `Cargo.toml` (members and `[workspace.dependencies]` only), `Cargo.lock`.
- `scripts/check-host-core-policy.sh`, `scripts/test-host-core-policy.sh` (D10),
  `scripts/check-cross-targets.sh` (D11 row only), `.github/workflows/qualification.yml` (D12
  only). These four are outside stream B's file set; the coordinator sequences them.
- `docs/C_ABI_V1_QUALIFICATION.md`: path references to the moved files only.

## Non-goals

- Any behaviour change, rename of a diagnostic or result code, or header change.
- The browser adopting the crate (#1332).
- Changing the classifier (`crates/host-core/src/live_delta.rs`) or preparation
  (`crates/host-core/src/prepare.rs`).
- Making the render-activity code adapter-chosen (D7).
- An adapter preparation hook. `prepare_runtime` moves as it is, with the C ABI's lane selection
  (`C_ABI_LIVE_LANES`, `LIVE_QUEUE_DEPTH`). The `RuntimePreparer` trait that lets each adapter
  supply its own preparation, with capi's implementation being today's live request and lane
  selection, lands with the browser's implementation in *Prepare through an adapter-supplied
  preparer in the control-plane crate* (#1400).

## Hazards

- **Behaviour drift by reordering.** Keep every fallible check in its current order inside
  `command` and `commit_live`; the acked-batch rule depends on it (`commit_live`'s contract,
  `control.rs:1039-1062`).
- **Test-only code in the product.** `test-support` must not reach the release `capi` build:
  `bash scripts/check-capi-abi.sh` builds `-p capi` alone.
- **`Mutex` in `SharedPlanState`.** It compiles for the browser target; no render path locks it
  (`plan.rs:41` is a query). Do not add a render-side lock.

## Objective gates

1. **Same bits, same accounting (PR evidence, not a committed pin).** At the base and the head:
   `cargo build --locked --release -p audit -p capi && target/release/audit capi`. Both report
   `allocations`, `deallocations`, `locks`, `syscalls` and `total_violations` 0 and the same
   `pcm_digest`. `cargo test --locked -p capi --test resource_lifecycle -- --nocapture` prints the
   same resource rows at both, `capi_retained_bytes` and the report-table row included (D4-D5),
   with one exception (Amendment 1): the `Session` fixed row and `capi_retained_bytes` fall by
   exactly `size_of::<CompileLimits>() - size_of::<ControlLimits>()`. The gate computes that
   difference from the two types at the head (not a literal) and checks base minus head equals it;
   every other row is identical.
2. **The C ABI suite is unchanged and green.** `cargo test --locked -p capi` passes with no test
   deleted or weakened, and one test added (the D4 row round trip below); `git diff --stat` on
   `crates/capi/src/runtime/tests.rs`, `live_tests.rs` and `crates/capi/tests/` shows import and
   accessor changes only. Amendment 1: the `live_peak_tests` module (today `compile.rs:837-994`)
   stays in capi with unchanged assertions; `validate_live_peak`, `LiveEpochResources` and
   `CapiResources` are exposed under the crate's `test-support` feature for it. Expected
   resource-byte assertions that read `capi_retained_bytes` or the `Session` row may change only by
   the gate-1 difference, computed from the two types.
3. **The header is unchanged.** `bash scripts/check-capi-abi.sh` and
   `bash scripts/check-capi-abi.sh --self-test` pass.
4. **Browser target.** `bash scripts/check-cross-targets.sh` passes with the D11 row.
5. **Policy.** `bash scripts/check-host-core-policy.sh`, `bash scripts/test-host-core-policy.sh`,
   `bash scripts/check-workspace-policy.sh`, `bash scripts/check-realtime-policy.sh`,
   `python3 -B scripts/check-test-support-ci.py`, `python3 -B scripts/check-release-shape.py`.
6. **Workspace.**
   - `cargo test --locked -p control-plane --features test-support`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `cargo clippy --locked -p control-plane --all-targets -- -D warnings` (no features)

## Test value

One new test (Amendment 1): in capi, `PlanResources -> PlanResourceReport -> PlanResources` is the
identity for a value with every field distinct and nonzero, once with `TailSamples::Finite(n)` and
once with `TailSamples::Infinite`. It turns red if a conversion drops, swaps or mis-maps a field or
a tail kind, which would corrupt the replacement and live peak checks that read stored rows back
(no existing test reads a stored row through the conversion); the mutation run is recorded in the
Attempt record. Otherwise no new test. The extraction is proven by the unchanged C ABI suite (gate 2) and the one-time
same-bits comparison (gate 1). The D11 cross-target row is a build check: it turns red if the crate
gains a dependency or API the browser target cannot compile, which no existing row checks.

## Amendment 1 (root, 2026-10-05)

The first implementation run stopped before any change because the spec contradicted itself; it
had no verdict and is not an attempt. Root ruled:

1. **Measured accounting change accepted.** Storing `ControlLimits` (D3) shrinks `Session`, so the
   `Session` fixed row and `capi_retained_bytes` (and the retained-bytes value
   `miso_engine_v1_plan_resources` returns) fall by exactly
   `size_of::<CompileLimits>() - size_of::<ControlLimits>()`. Product outcome, D5 and gate 1 carry
   this one exception; the gate computes it from the two types. No padding, and `CompileLimits`
   does not enter the crate.
2. **D4 bound.** `Row: From<PlanResources> + Copy` and `PlanResources: From<Row>`, with a
   lossless round-trip test (Test value).
3. **Authorized adjustments** (in D2, D6 and gate 2): a crate-local zeroed-buffer helper, with capi
   building `session_error` after `compile_children`; `PlanState::render` returns `RenderError`
   and capi maps it; `graph`/`source`/`effect` `.resource.limit` become
   `CompileFailure::Diagnostics` with today's exact bytes; `live_peak_tests` stays in capi with
   `validate_live_peak`, `LiveEpochResources` and `CapiResources` exposed under `test-support`.

## Amendment 2 (root, 2026-10-05)

D5's constructor argument could not meet gate 1: the crate needs the adapter's allocation sizes on
every structural rebuild, and storing them inside `Session` would change the `Session` row by more
than Amendment 1's exact shrink. Root ruled for one adapter type parameter:
`SessionState<A: ControlAdapter>`, whose trait carries `type Row: From<PlanResources> + Copy` (with
`PlanResources: From<A::Row>`) and `const ADAPTER_ALLOCATIONS: &'static [u64]`, implemented by
capi (D4, D5). Gate 1 is unchanged from Amendment 1. #1400's `RuntimePreparer` may later join
this trait; it is not pre-built here. The stopped runs before this amendment had no verdict and
are not attempts.

## Amendment 3 (root, 2026-10-05)

The stream-B batch-1 qualification found that the lint job's mutation suite
(`scripts/test-test-support-ci.py`) anchors on a literal copy of the `test-debug-a` feature list
that D12 changed. Root added the file to D12's paths: the copy gains `control-plane/test-support`
and the removed-features case expects `control-plane`.

## Dependencies

- None.

## Attempt record

### Attempt 1 (implementer, 2026-10-05)

Base re-captured at `d6fa539a1` (after the stream-I merge) before any change.

**What changed.** New crate `crates/control-plane` (lib `control_plane`): `control.rs`, `compile.rs`
and `plan.rs` moved by `git mv`; `error.rs` holds the typed failures (`CompileFailure::{Resource,
Diagnostics}`, `ResourceFault`); `adapter.rs` holds `ControlAdapter` (exactly `type Row:
From<PlanResources> + Copy; const ADAPTER_ALLOCATIONS: &'static [u64]`, with `PlanResources:
From<A::Row>` as a where clause on the items that read rows back), `ControlLimits` and
`PlanResources`. `SessionState<A>`, `PlanState<A>`, `PlanQueries<A>`, `SharedPlanState<A::Row>`
and `TestTransactionSnapshot<Row>` are generic; nothing adapter-specific is stored. capi keeps the
FFI, ABI types, `FixedBytes`, `plan_error`, `limits_are_valid`/`all_limits_nonzero`,
`CapiAdapter` (`Row = PlanResourceReport`, allocations = `size_of` of `Session` and `Plan`),
type aliases `SessionState`/`PlanState`/`PlanQueries`, the three `From` conversions, the
failure-byte, render-code and source-report mappings, and a `compile_children` wrapper that
converts the limits once and builds `session_error` after the control plane returns.
`live_peak_tests` moved to `crates/capi/src/runtime/live_peak_tests.rs` with unchanged assertions
(its helpers convert with `.into()` and map failures with `failure_bytes`). Test-only crate items
(`CapiResources`, `CompiledModelAdmission`, `LiveEpochResources`, `validate_live_peak`,
`ProviderEpoch`, `LIVE_QUEUE_DEPTH`, `C_ABI_LIVE_LANES`, `prepare_caps`) are re-exported only
under `test-support` and are crate-private otherwise (`#[cfg_attr(not(feature = "test-support"),
allow(unreachable_pub))]` on those items only). `PlanState` gained two production accessors capi
needs because the fields are no longer reachable: `owner()` (for `plan_carry_counts` and
`plan_replacement_count`) and `fp_env_attested()`/`attest_fp_env()`. Policy, cross-target and CI
edits per D9-D12. `docs/C_ABI_V1_QUALIFICATION.md` names none of the moved paths, so it is
unchanged. No check in `command` or `commit_live` moved; no render-side lock was added.

**Gate 1 (same bits, same accounting).** `target/release/audit capi`, base and head: identical
line, `pcm_digest` `75f4e6c21da7a2a7`, allocations, deallocations, locks, syscalls and
total_violations all 0. `resource_lifecycle -- --nocapture --test-threads=1` (serial, so the
diff is line for line; RaceCounts lines are timing-dependent and excluded): every row identical
except the capi-retained rows, each smaller by exactly 24 bytes:
`capi_retained_bytes` 274153 -> 274129; observed capi 274153/145536/132304/151394 ->
274129/145512/132280/151370; `capi: double-live requirement` 174780 -> 174756; `capi: live
peak` 413237 -> 413213. `size_of::<CompileLimits>() - size_of::<ControlLimits>()` computed from
the two types at the head by a scratch probe linking `capi` and `control-plane`: 208 - 184 = 24.
Largest-allocation, graph, source, effect, builtin and plan rows unchanged.

**Gate 2.** `cargo test --locked -p capi`: 72 lib tests (base 71 = 23 tests.rs + 29 live_tests.rs
+ 1 live_peak_tests + 16 ffi.rs + 2 abi.rs, plus the new one), 2 plan_swap_race, 11
resource_lifecycle, all pass. `tests.rs`/`live_tests.rs` diffs are accessor rewrites
(`.controller` -> `.test_controller()`, `.providers.epoch` -> `.test_providers().test_epoch()`,
`.plan.owner` -> `.owner()`/`.test_owner_mut()`, `.plan.shared.active_epoch` ->
`.test_active_epoch()`, `synchronize_plan_epochs` -> `test_synchronize_plan_epochs`) plus one
panic-message format (`{code}` -> `{code:?}`, since the render error is now `RenderError`).
`crates/capi/tests/` unchanged. No resource-byte assertion needed a change.

**New test and mutation evidence.**
`runtime::report_row_tests::a_stored_report_row_reads_back_as_the_resources_it_was_built_from`
(capi `runtime/mod.rs`): `PlanResources -> PlanResourceReport -> PlanResources` is the identity
for a value with every field distinct and nonzero, with `TailSamples::Finite(6)` and
`TailSamples::Infinite`. Mutations of the readback conversion, full capi suite with
`--no-fail-fast`:
- M1, every stored tail reads back as `Finite`: the new test red; every other capi test green
  (71 lib, 2 race, 11 lifecycle). This is the defect no existing test catches.
- M2, `source_total_bytes` and `source_overhead_bytes` swapped on readback: the new test red, and
  `double_live_oracle_drives_exact_and_one_below_c_caps` also red (admission-read fields are
  already defended end to end; the new test adds the fields and tail kind the admissions do not
  read).
- Reverted: all green.

**Gate 3.** `bash scripts/check-capi-abi.sh`: ok (shared and static). `--self-test`: ok.
`cargo tree -p capi -e normal,features -i control-plane`: only `control-plane feature
"default"`; `test-support` does not reach the release capi build.

**Gate 4.** `bash scripts/check-cross-targets.sh` with the D11 row: PASS (exit 0; the wasm32 simd128 `-p control-plane` check row ran in the matrix).

**Gate 5.** `check-host-core-policy.sh` ok; `test-host-core-policy.sh` ok (every old leg kept;
new legs: control-plane recompiles the pipeline, reinvents the identity processor, hand-decodes
the wire, drops control-provider, src deleted, manifest deleted; capi enables control-provider);
`check-workspace-policy.sh` ok; `check-realtime-policy.sh` ok; `check-test-support-ci.py` ok;
`check-release-shape.py` ok; `check-ci-path-routing.py` ok (CI file changed).

**Gate 6.** `cargo test --locked -p control-plane --features test-support` ok (no tests of its
own); `cargo fmt --all -- --check` ok; `cargo clippy --locked --workspace --all-targets
--all-features -- -D warnings` ok; `cargo clippy --locked -p control-plane --all-targets -- -D
warnings` ok.

**Open items.** None blocking. Reviewer attention: the `cfg_attr(..., allow(unreachable_pub))`
pattern for test-only exposure (the workspace denies `unreachable_pub`, and visibility cannot be
feature-gated otherwise without duplicating definitions).

### Batch follow-up to the attempt-1 verdict (worker, 2026-10-05)

Root approved three follow-ups from the attempt-1 verdict (MINOR 1, NIT 1, NIT 2).

**MINOR 1: the D6 byte table is pinned.** New test
`runtime::error::failure_bytes_tests::every_failure_kind_reports_its_frozen_c_abi_bytes`
(`crates/capi/src/runtime/error.rs`). It drives every `ResourceFault` variant through
`failure_bytes` and through `CompileRejection::from`, a `CompileFailure::Diagnostics` passthrough,
and `SourceFailure::Internal.report()`, and pins each kind's exact bytes. The pins are the
literals that base `d6fa539a1`'s `crates/capi/src/runtime/{compile,control,error}.rs` emitted
(`failure(code)` was `format!("{code}\t$\n")`; `SourceFailure::Internal` was
`(RESULT_INTERNAL, b"capi.source.epoch")`). These bytes are C ABI output, so AGENTS.md permits
the exact pin; the test says so. The expected-bytes match is exhaustive, so a new fault cannot
ship unpinned. Plausible defect it catches that no existing test does: a typo or a swap in the
one table every resource-fault site now goes through (the verdict's M5 stayed green on the
whole capi suite). Mutations, `cargo test -p capi --lib failure_bytes_tests`:
- `Arithmetic` and `Platform` strings swapped in `failure_bytes`: red (`Arithmetic`); reverted:
  green.
- `ProtocolQueue` mapped to `capi.plan.exchange`: red (`ProtocolQueue`).
- `capi.source.epoch` misspelt: red (the `SourceFailure` assertion).
- Reverted: green.

**NIT 1: open specs re-anchored.** Every open spec that cited `crates/capi/src/runtime/
{control,compile,plan}.rs` (or its bare `control.rs:`/`compile.rs:`/`plan.rs:` follow-on
anchors) now cites the current location in `crates/control-plane/src/`, each line checked
against the current tree (`3bb27fdc1` plus this commit). Items that stayed in capi keep capi
paths: the tail mapping is now `crates/capi/src/runtime/mod.rs:183-186`, and
`all_limits_nonzero` is `crates/capi/src/runtime/mod.rs:111-137`. Anchors only: no scope,
decision or gate changed. #1326's authorized constant line is now in
`crates/control-plane/src/compile.rs`. Not re-anchored: #349 (a dated audit table whose every
row is a snapshot) and #1350 (its `crates/capi/src/runtime/error.rs` is still where capi's own
diagnostics live). #1020's anchor names code that no longer exists, so it is pinned to the commit
it was written at (`d70956bf8`).

**NIT 2.** The header comment at `scripts/check-host-core-policy.sh:12-16` is reflowed to 95
columns; the wording is unchanged.

**Gates.** `cargo test --locked -p capi`: ok (73 lib, 2 race, 11 lifecycle). `cargo fmt --all --
--check`: ok. `cargo clippy --locked -p capi --all-targets --all-features -- -D warnings`: ok.
`check-host-core-policy.sh`, `test-host-core-policy.sh`, `check-workspace-policy.sh`: ok.

### Batch follow-up (Amendment 3, commit 043ed73c6)

`scripts/test-test-support-ci.py`: `DEBUG_A_FEATURES` gains `control-plane/test-support` (the exact
literal of the `test-debug-a` `--features` at `qualification.yml:619`), and the
"every test-support feature removed from test-debug-a" case expects `control-plane`.
`python3 -B scripts/test-test-support-ci.py` passes; reverting only the anchor edit fails with
"mutation anchor must occur once in qualification.yml", and restoring it passes.
`check-test-support-ci.py`, `check-workspace-policy.sh`, `check-realtime-policy.sh` (93 regions in
26 files, equal to the floors), `test-realtime-policy.sh`, `check-ci-path-routing.py` and
`cargo fmt --all -- --check` pass.

### Merge of origin/main (root decision D1.1)

Main's #1422 added a doctest step to `test-debug-a` (and one to `test-debug-b`). The merge keeps
`control-plane/test-support` in both `test-debug-a` steps, so the doctest step's `--features` list
equals its sibling test step's list. `scripts/check-test-support-ci.py` ignored `--doc` steps (they
are not whole-package runs), so a feature dropped from the doctest step alone passed. It now
requires each job's `--doc` step to enable the same packages and features as a whole-package step of
the same job. `scripts/test-test-support-ci.py` gains `in_a_doc` and `in_b_doc` (the `in_step`
scoping, keyed on the `--doc` command) and four cases: `control-plane/test-support` removed from the
`test-debug-a` doctest step only, `builtins/test-support` removed from the `test-debug-b` doctest
step only, `host-web` excluded from the `test-debug-a` doctest step only, and a package dropped from
the `test-debug-b` doctest step only. Mutation evidence: with main's unchanged checker the first case
is accepted ("mutation not refused", empty stderr); with the rule it is refused. On the real workflow,
deleting `control-plane/test-support` from the doctest step's `--features` makes
`check-test-support-ci.py` exit 1 with "the doctest step must enable the same packages and
features", and restoring it passes (12 packages). The self-test's two removed-feature cases expect
both `control-plane` and main's `effect-contract`.
