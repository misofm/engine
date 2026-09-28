# Delete the multicore hand-over and the unused render input

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 3, rows A5 and
A6. No new ruling is needed.

- AGENTS.md already says the dependency-wave renderer "was built, qualified and then removed as
  production-unreachable".
- The product scope ruling (`docs/rulings/product-scope-stems-for-mixing.md`) excludes any external
  audio input.

## Context

### Part 1: auxiliary-worker hand-over

What is left of the removed dependency-wave renderer:

- **Trait defaults on `PreparedPlanExecutor`** (`crates/engine/src/realtime/plan.rs`):
  - `copy_worker_audit_snapshots` (`:466`);
  - `take_handover` (`:476`, documented as existing "so a persistent auxiliary worker pool
    outlives the plan that used it");
  - `accept_handover` (`:485`);
  - `dispatch_counters` (`:490`);
  - `pub type ExecutorHandover` (`:500`).
- **Their `PreparedRenderPlan` wrappers:** `dispatch_counters` (`:656`) and
  `copy_worker_audit_snapshots` (`:864`).
- **`plan_exchange.rs`:** the hand-over block in `enter_block` (`:399-416`), and the
  `RealtimePlanOwner` wrappers `copy_worker_audit_snapshots` and `dispatch_counters`
  (`:495-507`).

**Who uses it.** No production executor overrides these defaults: `graph`'s executor does not. The
only implementer is engine's own test executor `HandoverExecutor` (`realtime/mod.rs:340-450`),
whose test `enter_block_moves_the_executor_handover_and_returns_a_refused_one` exercises the
removed feature itself. The wrappers have no caller (rustc proof in the audit, section 2).

**Overlap with `01-…`.** `01-…` already deletes the `plan_exchange.rs` wrappers and the `plan.rs`
cascade. This issue removes the trait defaults, the type, the `enter_block` block and the test
executor.

### Part 2: the external render input no host uses

- **Where it lives:**
  - `RenderEnvelope::input_channels` (`plan.rs:117`);
  - `RenderIo::input` (`:168`);
  - `RenderError::InputShape` (`:515`, checked at `:930-934`);
  - the `input: Option<PlanarBufferRef>` parameter of `PreparedPlanExecutor::render` (`:304`).
- **Nothing feeds it or reads it.**
  - The only production executor ignores it (`crates/graph/src/lib.rs:2678`, `_input`).
  - The only production plan constructor sets `input_channels: None`
    (`crates/graph-compiler/src/compile.rs:828`).
  - Every host passes `input: None`: host-web (`lib.rs:4883`), host-core
    (`render_session.rs:148`) and capi (`runtime/plan.rs:203`).
  - `rg 'input: Some\(' crates/engine` finds nothing.
  - The C ABI render entry has no input planes (`crates/capi/include/miso_engine_v1.h:242-244`),
    so no exported surface changes.
- **Cost of removal:** `rg` counts 95 `input: None` and 38 `input_channels: None` lines across
  `crates`, `hosts` and `tools`, mostly in tests. A few may belong to unrelated structs; the
  compiler decides.
- **Effect on the shipped module:** it is compiled in (one shape check per render), so the pin
  moves. No console digest can move.

## Smallest closable slice

1. **Part 1.** Delete the four trait defaults, `ExecutorHandover`, the `enter_block` hand-over
   block and the two `PreparedRenderPlan` wrappers. Delete the `HandoverExecutor` test module and
   its test, and list the test as deleted: its subject was the removed feature.
2. **Part 2.**
   - Delete `RenderIo::input`, `RenderEnvelope::input_channels`, `RenderError::InputShape` and its
     check, and the `input` parameter of `PreparedPlanExecutor::render`.
   - Update every call site.
   - Delete the tests whose subject is the input shape check; list each with its reason.
   - Keep every other assertion in those files.
3. Update any doc outside `docs/handoffs/` that describes the render input plane or worker
   hand-over. The audit found no such text in `docs/REALTIME_DEPENDENCY_POLICY.md` or
   `docs/REALTIME_MEMORY.md`, so expect doc comments only.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web -p host-core`
     passes.
   - The loom leg passes:
     `RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked --release -p engine --lib spsc_loom`.
2. **Console digests.** The `gain_pan_profile digests` output
   (`cargo test --locked --release -p console-workload --test gain_pan_profile -- --ignored --exact digests --nocapture`)
   is byte-identical on base and change, and `bash scripts/run-wasm-gates.sh` passes.
3. **Shipped artifact.** Build base and change on one machine with
   `scripts/build-web-audioworklet.sh --module-only EMPTY_DIR`.
   - Explain every changed function in `wasm-objdump -d`. Expected: the render entry loses the
     input-shape branch; everything else changes only by panic-location line numbers.
   - Re-pin with that explanation. `bash scripts/check-web-audioworklet.sh` passes on the new
     module; its render-closure and trap counts may only fall.
4. **Realtime audits.** Rebuild `audit`. `bash scripts/trace-realtime-audit.sh target/release/audit 1000000`
   and `bash scripts/test-realtime-audit-probes.sh realtime target/release/audit` pass.
5. **CI routing.** No workflow step changes. `check-ci-path-routing.py` and
   `test-ci-path-routing.py` pass.
6. **No live claim lost.** The `-- --list` diff against base (with `00-…`'s features) contains
   only the tests listed in steps 1 and 2.

## Dependencies

`00-…`, `01-…`.

## Standing rules for the implementer

- No product behaviour change; a moved console digest is a hard stop.
- Commit on `codex/<issue>-delete-handover-and-render-input`. Do not run timed benchmarks.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, finding F3. Spot-checked facts hold: the only production executor is
`GraphExecutor` (`crates/graph/src/lib.rs:2626`), which ignores `_input` (`:2678`); the only
`PreparedPlanExecutor` implementers besides it are engine's test executors
(`realtime/mod.rs:52`, `:345`); the `plan_exchange.rs`/`plan.rs` wrappers were deleted in a scratch
copy and every build passed, `aarch64-apple-ios` and `aarch64-linux-android` included. Part 2 was
not compile-proved here.

**The corrected scope makes this draft touch the mobile path.** With the C ABI kept for mobile
playback (R2 amendment), `RealtimePlanOwner::enter_block` (`plan_exchange.rs:395-420`) is the
block-boundary plan swap capi uses on every structural edit, and `capi/src/runtime/plan.rs:201-206`
is a Part-2 call site. Neither the header nor capi's behaviour should change (the C render entry
has no input planes), but the gates must prove it:

1. Add `cargo test --locked -p engine -p capi` (plan replacement, resource lifecycle and the
   layout mirrors in `crates/capi/tests/resource_lifecycle.rs`) and the audit-native capi runtime
   audit with its inline validator (`qualification.yml:572-654`) on base and change.
2. Add `cargo check --target aarch64-apple-ios` and `--target aarch64-linux-android` for
   `-p engine -p graph -p capi -p host-core` (a toolchain with the aarch64 std; CI has none).
3. Keep `reserve_replacement`, `epoch`, `commit`, `next_absolute_sample` and `render_contiguous`:
   they are capi's, not multicore remnants.

## Carried from #1023 (root, 2026-09-28)

#1023 deferred engine `PlanarBufferRef`'s `try_new`, `plane`, `plane_range` and its fields
`storage`/`stride` (`crates/engine/src/realtime/buffer.rs:133-288`) to this issue. Deleting the
fields removes the struct's null-pointer niche, so `Option<PlanarBufferRef>` in `RenderIo` grows a
tag and `PreparedRenderPlan::render_inner` changes in the shipped module. Remove them together with
the render input this issue deletes, and account for the artifact change in the same place. See
the #1023 spec, "Kept".

## Attempt 1 evidence

Implementer: Terra (Claude Opus 5.5), branch `codex/1024-multicore-handover-render-input` from
`codex/batch-slim-1` at `ed0556a9`. Code commit `6034f220`, plus this record. Nothing is pushed.

**Size:** 44 files, 516 lines deleted and 70 added, net **-446** (Rust -507/+60, net -447;
`crates/engine` -291/+17). Most of the additions are rustfmt re-flowing call sites to
`RenderIo { output }`.

### Deleted

- **Part 1 (hand-over):**
  - `PreparedPlanExecutor::take_handover` and `accept_handover`, and `ExecutorHandover`;
  - the hand-over block in `RealtimePlanOwner::enter_block` (`old` is no longer `mut`);
  - `PreparedRenderPlan::executor_mut` (cascade: its only callers were that block and the test);
  - the test executor `HandoverExecutor`, its `handover_plan` helper and its test.
  - `copy_worker_audit_snapshots` and `dispatch_counters` were already gone with #1023.
- **Part 2 (render input):**
  - `RenderIo::input`, `RenderEnvelope::input_channels`, `RenderError::InputShape` and the
    per-block shape check in `render_inner`;
  - the `input` parameter of `PreparedPlanExecutor::render` (`GraphExecutor` and engine's test
    executor, and graph's `render_host` test helper that called it);
  - all 95 `input: None` and 38 `input_channels: None` lines. Every one was a `RenderIo` or
    `RenderEnvelope` field; `rg` now finds none of either outside `docs/handoffs/` and
    `artifacts/`. The one `input: Some(..)` left (`builtins-compiler` `lib.rs:3547`) is another
    struct.
- **Carried from #1023:** with the render input gone, engine's `PlanarBufferRef` had no user in any
  crate, host or tool. So the whole type goes, not just `try_new`, `plane`, `plane_range` and
  `storage`/`stride`: a struct left with private `channels`/`frames` fields and no constructor
  could never be built. The private `plane_range` helper goes with it. `validate_borrow` stays;
  `PlanarBufferMut::try_new` uses it.

### Tests deleted (step 1 and 2)

- `realtime::tests::enter_block_moves_the_executor_handover_and_returns_a_refused_one` (engine
  lib). Its subject was the removed hand-over.
- None for Part 2: no test anywhere asserted `RenderError::InputShape` or built a plan with
  `input_channels: Some(..)` (checked on the base with `rg`). Every other assertion in the touched
  files is kept; those edits only drop the `None` field lines.

### Docs and records

- `docs/REALTIME_MEMORY.md` no longer names `PlanarBufferRef`, and says the reference renderer
  checks the output shape rather than "I/O".
- `scripts/check-graph-policy.sh`: a comment cited the hand-over test as the reason a
  `#[cfg(test)]` module may implement the executor seam. It now cites engine's response-capture
  test, which still does. Comment only.
- `crates/graph/tests/MUTATIONS.md` retires row N5 (it named the deleted test), with a note beside
  the N7-N16 retirement.
- `rg` finds no other text outside `docs/handoffs/` and `artifacts/` that describes a render input
  plane or the worker hand-over. The dated audit `docs/audits/test-usefulness-2026-09-04/01-foundation.md`
  still says "hand-over" in its 2026-09-04 table; I left it as a dated record.

### Re-pin (gate 1 of the Amendments: capi layout mirrors)

`crates/capi/tests/resource_lifecycle.rs` failed exactly two tests, and only on byte totals.
Dumping every owner row on base and change showed which rows moved. `RenderEnvelope` loses an
8-byte `Option<NonZeroUsize>`, so each retained copy is 8 bytes smaller:

| row | base | change |
|---|---|---|
| publication queue slots (2 `PublishedPlan`) | 272 | 256 |
| retirement queue slots (2 `RetiredPlan`) | 256 | 240 |
| plan handle (active plan and pending candidate) | 416 | 400 |
| session handle (`PlanPublisher::envelope`) | 6,456 | 6,448 |

That is -56 in total. So active CAPI goes from 160,957 to 160,901, double-live CAPI and
`oracle.capi` from 204,375 to 204,319, the two `frozen_scratch_report` calls follow, and the
tiny-frame base goes from 178,490 to 178,434. Each site carries a `#1024` comment. Nothing else
moved:

- the protocol size pins pass unchanged;
- `check-browser-expected-resources.py` passes with no re-pin (wasm32 rows and PCM digests).

### Gates

All runs were on one machine, with rustc 1.97.1 (no `rust-src`) and `CARGO_INCREMENTAL=0`. The
base (`ed0556a9`) ran in a detached scratch worktree with its own target directory.

1. **Native.** All pass with no warnings:
   - `cargo check --locked --workspace --all-targets --all-features`;
   - `cargo clippy … --all-targets --all-features -- -D warnings` (re-run after touching the edited
     files);
   - `cargo fmt --all -- --check`;
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.
2. **wasm32.** `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target
   wasm32-unknown-unknown -p host-web -p host-core` passes with no warnings.
3. **aarch64 (Amendment 2).** On `aarch64-apple-ios` and `aarch64-linux-android` these pass:
   - `cargo check` and `cargo clippy -- -D warnings` of `--workspace --lib --all-features`
     (excluding `native-pcm-runner`, `stem-hasher`, `wasm-console` and `wasm-gates`, whose
     `blake3`/`wasmtime` build scripts need a C cross-compiler);
   - `cargo check --all-targets --all-features` of lane, engine, host-core, capi, source, graph
     and soft-clip.

   The only warnings are the pre-existing x86-gated test bodies in `host-core/tests/fp_environment.rs`
   and `lane/tests/fp_env.rs` (VERIFY F4, #1017).
4. **Loom.** `RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked --release -p engine
   --lib spsc_loom` passes.
5. **Console digests.** All 17 `gain_pan_profile digests` rows are byte-identical between base and
   change. Only the `finished in` line differs.
6. **`bash scripts/run-wasm-gates.sh`** passes on both (native, wasm scalar, wasm `simd128`, and
   the V8 spill gate on each commit's own `--module-only` module). The `wasm-gates.jsonl`
   evidence is **byte-identical**: 3 legs, 142 cases and 358 comparisons each, 0 mismatches.
7. **Shipped artifact (gate 3).** Both modules were built with
   `build-web-audioworklet.sh --module-only`.
   - **Hashes:** the base is `3f744b03…25da`, the batch's current unpinned module and #1023's
     change hash. The change is `f7bd75ca…b75d`.
   - **Size:** 3,486,775 to 3,486,194 bytes (-581).
   - **Exports:** identical (135, same names). The type section is identical (118).
   - **Data and element sections:** the data section is 16 bytes shorter: the `InputShape`
     debug name (10 bytes) and `GraphExecutor`'s two hand-over vtable slots, less padding. The
     element segment is 4 bytes shorter, two table entries.
   - **Function-by-function `wasm-objdump -d`.** Call targets were compared by name and
     `call_indirect` types by signature. The function count went from 2,736 to 2,733:
     - 2,323 functions are identical.
     - 385 differ only in integer immediates. These are static addresses and load/store offsets
       shifting by -16 or -8 as the data section shrinks, plus field offsets in types that embed
       a `RenderEnvelope` (4 bytes smaller on wasm32).
     - **Removed (3):**
       - `GraphExecutor`'s `take_handover` and `accept_handover` vtable defaults;
       - `PlanarBufferMut::try_new`'s symbol, which is only renamed: its impl-block index moved
         when `PlanarBufferRef`'s impl went, and its body is identical.
     - **24 differ in instructions:**
       - **Render path (3), as expected:**
         - `PreparedRenderPlan::render_inner` (298 to 250): the input-shape branch is gone;
         - `GraphExecutor::render` is identical after local renumbering (one parameter fewer);
         - host-web `AudioWorkletEngineHost::render_next` (736 to 733) no longer stores the
           `None` input into `RenderIo`.
       - **`RenderError`'s `Debug::fmt` (1):** one variant fewer.
       - **Layout moves (17), all control-plane:** copies, drops and compares of types that
         embed a `RenderEnvelope`, where the memcpy lowering mixes `v128`/`i64`/`i32` moves
         differently:
         - `PreparedRenderPlan::prepare_with_executor`;
         - graph's `PreparedGraphPlan::{new, bind_with_source_set,
           bind_with_source_set_and_observation_activation, with_builtin_banks}`,
           `GraphPreparedSourceSet::new`, `GraphExecutor::new` and two drop glues;
         - builtins-compiler's `restore_graph_bind_failure`, `into_bound_with_source_set` and
           `…_and_observation_activation`;
         - host-core's five `prepare_host_runtime_*`.
       - **Inlining moved (3), all control-plane:**
         - `PreparedGraphPlan::bind_optional_source_set` now inlines `has_valid_structural_layout`
           and `lowered`. It was the only caller of each, and both leave the module. The merged
           body is 201 instructions shorter than the three base bodies together.
         - `prepare_binding_validation` now calls an outlined `BTreeSet<GraphNodeId>` drop glue.
           That glue is the one new function.
         - `compile_graph` calls a `BTreeMap<&TailSamples>` drop glue it used to inline.
   - **Call-graph gates (`check-web-audioworklet-callgraph.py`, run on both modules): identical.**
     - render export: closure 8, traps 5;
     - `meter_poll`: 9 and 2;
     - `command_submit`: 39 and 77;
     - `--kernel-shape --kernel-min 11`: passes.
   - **Not run in full:** `check-web-audioworklet.sh` needs the pinned six-file artifact set,
     which `build-web-audioworklet.sh` refuses to write while the pin is stale.
   - **Not re-pinned**, per the batch rule: the batch boundary re-pins it, and the CI `artifact`
     job is the authority.
8. **Tests (gate 6).** The `-- --list` diff between base and change is exactly the one deleted
   test:
   - `test-debug-a` set: 1,534 to 1,533;
   - `--workspace --all-targets`: 2,552 to 2,551;
   - `test-debug-b` (834) and the release `audit`/`bench`/`console-workload` set (179) are
     identical.

   Runs, all with 0 failures:
   - `cargo test --workspace`: 2,531 passed;
   - `test-debug-a`: 1,525 passed;
   - `test-debug-b`: 807 passed;
   - release `-p audit -p bench -p console-workload`: 176 passed;
   - `cargo test --locked -p engine -p capi` (Amendment 1): 80 passed (base: 81; the difference is
     the deleted test).
9. **capi runtime audit (Amendment 1).** `audit capi` gives an identical record on base and change:
   - 100,000 calls, `pcm_digest` `ff6cdcb96cdcdad5`;
   - 0 allocations, syscalls and violations.

   `audit source-duration` differs only in measured RSS. Both pass the `qualification.yml` inline
   validator.
10. **Realtime audits (gate 4).** `audit` was rebuilt from `6034f220`. These pass:
    - `trace-realtime-audit.sh target/release/audit 1000000` (1,000,000 blocks);
    - `test-realtime-audit-probes.sh` `realtime` (7 operations), `builtins` and `builtins-graph`;
    - `trace-graph-audit.sh`, `trace-source-audit.sh`, `trace-builtins-audit.sh`,
      `trace-builtins-graph-audit.sh` and `run-protocol-allocation-audit.sh`, whose tool sources
      this change edits.
11. **CI routing and policy (gate 5).** No workflow changes. 55 script invocations pass on both
    base and change:
    - `check-ci-path-routing.py` and `test-ci-path-routing.py`;
    - the `check-`/`test-` workspace, realtime, lane, graph, rack, builtins, host-core,
      effect-runtime, bench, protocol-control and session policy scripts;
    - both native-pcm-runner policy tests;
    - the env and step vocabulary checks;
    - `check-capi-abi.sh` (and `--self-test`), `check-unfused-seal.sh` (and `--self-test`) and
      `check-sdk-generated.sh`;
    - `check-release-shape.py`, `check-sdk-deletions.py`, `check-test-support-ci.py` and
      `test-test-support-ci.py`;
    - `test-gate-lib.sh`, `test-realtime-trace-validator.sh` and `test-wasm-realtime-atomics.sh`;
    - the dsp-research, effect-descriptor and effect-package checks.

### Finding (tooling, not this change)

`scripts/test-bench-policy.sh`'s `sort_fault` shim keeps its call counter in
`${TMPDIR:-/tmp}/bench-sort-fault-<label>`, but the cleanup deletes `/tmp/bench-sort-fault-<label>`.
With `TMPDIR` set, a second run in the same `TMPDIR` starts from a stale count and reports "bench
policy mutation escaped: allocator-owner-sort-error". My change run hit this after the base run
had used the same `TMPDIR`. It passed once the stale files (mine) were removed. CI is unaffected
(no `TMPDIR`, fresh runner). It is worth a one-line fix in a tooling issue.
