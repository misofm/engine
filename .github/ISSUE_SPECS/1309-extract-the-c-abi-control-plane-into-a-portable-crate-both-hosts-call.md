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
bit is the same before and after.

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
- **D5. Adapter allocations.** The crate's constructor takes `adapter_allocations: &[u64]`, the byte
  sizes of the fixed allocations the adapter keeps per session: capi passes `Session` and `Plan`
  (today's `compile.rs:233-234` rows). They enter the fixed allocation rows and the
  largest-allocation fold exactly where those two rows are today. The report-table row
  (`compile.rs:232`, `checked_layout::<(u64, PlanResourceReport)>(2)`) is charged from the
  adapter's `Row` type (D4), so it is the same layout and the same bytes as today. Together, D4 and
  D5 keep every fixed row, `capi_retained_bytes` and the largest-allocation fold unchanged, which
  gate 1 checks.
- **D6. Typed failures.** The crate never spells `capi.`. `CompileFailure` becomes an enum:
  `Resource(ResourceFault)` with `ResourceFault::{Arithmetic, Platform, Limit, Allocation,
  ProtocolQueue, PlanExchange}`, and `Diagnostics(Vec<u8>)` for the session and preparation
  diagnostic lines it builds today (`error.rs:99`, `prepare_failure`). `SourceFailure::Internal`
  stays a variant without text. capi maps each to today's exact bytes (`capi.resource.arithmetic\t$\n`
  and so on; `capi.source.epoch`).
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
  `scripts/check-test-support-ci.py` requires.

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
   same resource rows at both, `capi_retained_bytes` and the report-table row included (D4-D5).
2. **The C ABI suite is unchanged and green.** `cargo test --locked -p capi` passes with no test
   added, deleted or weakened; `git diff --stat` on `crates/capi/src/runtime/tests.rs`,
   `live_tests.rs` and `crates/capi/tests/` shows import and accessor changes only.
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

No new test. The extraction is proven by the unchanged C ABI suite (gate 2) and the one-time
same-bits comparison (gate 1). The D11 cross-target row is a build check: it turns red if the crate
gains a dependency or API the browser target cannot compile, which no existing row checks.

## Dependencies

- None.
