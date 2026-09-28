# Delete the multicore hand-over and the unused render input

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
- **Cost of removal:** about 95 `input: None` and 16 `input_channels: None` call sites, mostly in
  tests.
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
   `scripts/build-web-audioworklet.sh --module-only`.
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
