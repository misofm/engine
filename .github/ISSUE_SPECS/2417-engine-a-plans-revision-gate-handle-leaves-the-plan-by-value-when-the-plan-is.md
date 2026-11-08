# Engine: a plan's revision-gate handle leaves the plan by value when the plan is built

Successor of #2412. #2412 typed the source-read offset: it is set on the bound plan, and a sealed `PreparedRenderPlan` has no setter. Two engine-level steps of the same family stay as run-time rules on the sealed plan, in `crates/engine/src/realtime/plan.rs`:

- **The gate take.** `PreparedRenderPlan::take_revision_gate(&mut self) -> Option<PlanRevisionGate>` returns the plan's one control handle once, then `None`. Production takes it in control-plane (`crates/control-plane/src/compile.rs`, after `preparer.prepare`) and host-core (`crates/host-core/src/limiter_linked_session.rs`, `Console` construction). About 77 test sites take it too. Each does `.take_revision_gate().expect(..)`, so a second take is a panic or an `Option` arm, never a type error.
- **The held-gate stamp.** `PreparedRenderPlan::stamp_held_gate` stamps a write on the pin the plan still holds. It returns `None` once the handle is taken. `prepare_or_project` (`crates/host-core/src/prepare.rs`, the carry retargets, preparation's last fallible step) calls it on the sealed plan and maps `None` to `successor_revision_refusal()`.

They cannot be typed one at a time. The stamp's `None` arm exists because the take can come first. And no take on the sealed plan can be one-shot: a `&mut self` method on a type that persists can always be called again. So the handle has to leave the plan by value, where the plan is built.

Low priority: no defect is open. Today every site unwraps the `Option` once.

## Contract
- `PreparedRenderPlan::prepare` and `prepare_with_executor` return the plan together with its `PlanRevisionGate`, for example `(PreparedRenderPlan, PlanRevisionGate)`. The pin leaves at construction. The plan keeps the shared `Arc<RevisionGate>` for render's snapshot. The `gate_pin` field and `take_revision_gate` are removed. `owns_revision_gate` (`plan_exchange.rs`) stays.
- Graph's bind keeps the handle in `GraphBoundPlan`, and both seals return it: `GraphBoundPlan::seal` and `GraphCarryBuilder::seal` give `(PreparedRenderPlan, PlanRevisionGate)`. A second take does not type-check, because there is no take.
- `stamp_held_gate` is removed. `prepare_or_project` stamps the carry retargets with `PlanRevisionGate::stamp`, on the handle the seal returned. It is the same pin with the same open-stamp rule: the commit stamps the open revision again and publishes it. The `None` mapping goes. A refused revision (`Err`) stays a real refusal (`successor_revision_refusal()`).
- `PreparedHost` carries the handle from `prepare_or_project` to its two production consumers, control-plane's `compile.rs` and `limiter_linked_session`, which move it out instead of taking it.
- builtins-compiler's sealed fixture (`PreparedBuiltinsGraphSealed`) holds the handle instead of its lazy `test_gate`, so `test_only_write` stops taking it on first use.
- No rendered bit, plan word or fixture byte moves. This is a refactor.

## Scope and counts (at `00a9b9dcf`)
- About 22 `PreparedRenderPlan::prepare`/`prepare_with_executor` calls in 10 files: engine `realtime/mod.rs` (12), `plan.rs`, `plan_exchange.rs`, `tests/host_attachment.rs`, graph's bind, control-plane `plan.rs`, source lib, `bench-support/tests/plan_exchange_ceiling.rs`, audit `realtime.rs`, host-core `tests/host_attachment_render.rs`.
- 93 `.seal()` calls in 27 files. Most are in graph's lib (20) and tests, graph-compiler's lib (14) and tests, source lib (12) and builtins-compiler (8). Some are builtins-compiler's or graph-compiler's own seals, which return a struct with a `plan` field, so adding a `gate` field does not touch their callers.
- 79 `take_revision_gate()` calls in 41 files (82 matches in 43 files, docs included). 57 are in host-core. A single `(plan, gate)` helper does not absorb all of host-core's sites, because they come in four shapes:
  - a gate moved out of a `PreparedHost` that is then moved whole (`Self { prepared, gate }`);
  - an `Option` arm (`stored_fader_automation.rs`, graph-compiler `silent_bank_skip.rs`, `live_routes.rs`);
  - a destructure of `PreparedHost` (`let PreparedHost { plan, sources, inventory, .. } = host`), which gains `gate`;
  - `.take_revision_gate().expect(..).publish(revision)` on a successor.
  The brief names the helper or helpers, for example `PreparedHost::into_parts`, or a `gate` field moved out before the host is.

## Split (each part about half a day)
1. **Engine and graph.** `prepare` returns the handle, and bind and both seals carry it. Fix the engine, graph, graph-compiler, builtins-compiler, source, audit, bench, bench-support and console-workload sites. host-core's `prepare_or_project` takes the seal's handle and stamps on it, and `PreparedHost` gains the field. In this part, add a doc-hidden test-support accessor that host-core's existing tests call in place of `take_revision_gate`. This part closes the type rule for the take and the stamp in production code.
2. **host-core's test sites.** Move the 57 sites to the field or helper the brief names, and remove the accessor from part 1. If part 1 fits with every site moved, there is no part 2.

## Gates
- Mutation evidence: a second take, a stamp after the hand-out and a stamp on a sealed plan are each a compile error. Show it as PR evidence, or commit one `compile_fail` doctest pair (#1422 D2) on the seal's return.
- No bit moved: rendered output and `graph_fixture` bytes match the base, using #2375's method with one `write` per row (#2401's lesson). Run it on #2412's suite set: graph, graph-compiler and builtins-compiler on every target; source lib; host-core lib plus `successor_swap`, `warm_successor`, `withdrawn_successor`, `live_routes`, `browser_successor`, `swap_block_realtime`, `timeline`, `collapse_arming`, `randomized` and `source_read_clock` (with `control-provider`); control-plane lib.
- `cargo test --locked` for engine, graph, graph-compiler, builtins-compiler, host-core (with and without `control-provider`), control-plane, source, capi and rack, plus `cargo test --locked --release -p audit -p bench -p console-workload`.
- Builds without test-support features: `cargo check --locked --workspace` and `cargo build --locked --release -p bench -p bench-support -p wasm-gates`.
- The web call-graph gate: `build-web-audioworklet.sh --named-twin`, then `check-web-audioworklet.sh --without-metadata-regeneration`. The take and the stamp are control-side, so no render `call_indirect` should move.
- Clippy `-D warnings`, `cargo doc -D warnings`, `cargo fmt --check`, `scripts/check-workspace-policy.sh`, `scripts/test-workspace-policy.sh`, `scripts/check-rack-policy.sh` (rack source never spells the graph path, comments included), `graph_fixture --check` and the realtime policy tool.

## Evidence / attempt record

### Attempt 1 (Terra), base `origin/main` @ `35898c413`

**Slice.** Both parts: every site moved, so there is no part 2 and no temporary accessor.

**Claims re-verified at base** (`35898c413`).
- `PreparedRenderPlan::take_revision_gate` (`crates/engine/src/realtime/plan.rs:1012`) returns the
  handle once, then `None`; 79 `take_revision_gate()` calls in 41 files, 57 of them in host-core
  (56 test sites plus `limiter_linked_session.rs:442`). Production takes it at
  `crates/control-plane/src/compile.rs:2474`. Holds.
- `PreparedRenderPlan::stamp_held_gate` (`plan.rs:1032`) returns `None` once the handle is taken,
  and `prepare_or_project` maps that arm to `successor_revision_refusal()`
  (`crates/host-core/src/prepare.rs:5939`). Holds.
- builtins-compiler's `test_only_write` takes the handle lazily into `test_gate`
  (`crates/builtins-compiler/src/lib.rs:6656`). Holds.
- Counts: 19 `PreparedRenderPlan::prepare`/`prepare_with_executor` calls (the spec's ~22), 93
  `.seal()` calls. Nothing dropped.

**Design.**
- `PreparedRenderPlan::prepare` and `prepare_with_executor` return
  `Result<(PreparedRenderPlan, PlanRevisionGate), RenderError>`. The plan keeps its
  `Arc<RevisionGate>` for render's snapshot; `gate_pin`, `take_revision_gate` and
  `stamp_held_gate` are gone. `owns_revision_gate` stays.
- `graph::GraphBoundPlan` holds the handle (a private `gate` field, carried through every state),
  and `GraphBoundPlan::seal` and `GraphCarryBuilder::seal` return
  `(PreparedRenderPlan, PlanRevisionGate)`.
- `prepare_or_project` binds `(plan, gate)` from either seal and stamps the carry retargets with
  `gate.stamp(base.revision)`; a refused stamp is still `successor_revision_refusal()`, and the
  `None` arm is gone. The handle keeps the open stamp in its pin, as the plan's pin did, so the
  commit's second stamp at the same revision and its publish are unchanged.
- `PreparedHost` gains `pub gate: PlanRevisionGate`. control-plane's `prepare_runtime` moves it
  out with `prepared.plan`; `start_render_session` drops it (a started session publishes nothing,
  as before when the pin stayed in the plan).
- builtins-compiler: `PreparedBuiltinsGraphSealed` is its own struct with a `pub gate` field (no
  longer an alias of the bound struct with a lazy `test_gate`), and `PreparedBuiltinsGraphBound`
  loses its type parameter, which had no other instance.
- host-core's test sites, by shape: a struct that held `prepared` and a separate `gate` drops the
  `gate` field and reads `prepared.gate` (`Console`, `Session`, `Host`, `Arm`, `Rig`, `LivePlan`,
  `Live`); a `PreparedHost` destructure binds `gate`; a successor's `.take_revision_gate()
  .expect(..).publish(r)` is `gate.publish(r)`; the `Option` arms (`stored_mute_automation`,
  `stored_fader_automation`) read `prepared.gate` under their existing `handles` condition. Two
  tests moved a gate out of a host that kept rendering: `live_sends_render_without_allocating`
  splits the `Live`'s fields (the producer thread borrows `host.gate`, the render loop the plan and
  sources, through two free functions `Live::submit` and `Live::render_block` now call), and
  `warm_successor`'s `Stream` keeps the newest published plan's handle. graph-compiler's
  `live_routes` and `silent_bank_skip` keep their test-local `Option` (one moves the gate to a
  producer thread), filled from the seal's handle.
- Tests deleted: engine's `a_plan_hands_out_its_gate_handle_once`,
  `a_held_gate_stamps_on_the_pin_the_handle_inherits` and
  `the_handle_inherits_the_open_stamp_of_a_held_gate`. Their claims are now the type (no take, no
  held pin) or `GatePin`'s own tests (`a_second_revision_is_not_stamped_before_the_first_is_published`);
  the open-stamp-again path stays covered by host-core's successor tests. None added beyond the
  doctest pair below.

**Mutation evidence: the rules are compile errors.** Committed: one `compile_fail,E0599` doctest
pair on `GraphBoundPlan::seal` (a take on the sealed plan; its twin stamps on the returned
handle, #1422 D2). Test value: red if a plan offers a take again, so a second handle with its own
pin could exist; no other test sees a type-level change. PR evidence, a scratch test file in
host-core, never committed, each line a compile error at head:

| Rule | Error |
|---|---|
| a take on a plan from `GraphBoundPlan::seal` | E0599 `no method named take_revision_gate` |
| a stamp on a sealed plan (`stamp_held_gate`) | E0599 `no method named stamp_held_gate` |
| a take on a plan from `PreparedRenderPlan::prepare` | E0599 |
| a stamp on a plan from `GraphCarryBuilder::seal` | E0599 |
| a stamp after the hand-out (the seal's handle moved, then used again) | E0382 `use of moved value: gate` |
| a second take from a `PreparedHost` (`host.gate` moved twice) | E0382 `use of moved value: host.gate` |

At base each of the first four compiled, and a second take returned `None` at run time.

**Mutations of the change (run-time, head).** The tree was restored after each.

| Mutation | Run | Result |
|---|---|---|
| `PreparedRenderPlan::prepare` returns a handle of another gate | `engine --lib` | red: 9 tests (the exchange's own-gate assertion and every probe that publishes) |
| `GraphBoundPlan::seal` returns a handle of another plan's gate | `graph --lib` | red: 5 live-control tests (`a_live_control_parameter_command_applies_at_the_next_block_boundary` and four more) |
| `GraphCarryBuilder::seal` returns a handle of another plan's gate | host-core `successor_swap`, `warm_successor` | red: 20 and 3 tests |
| `prepare_or_project` stamps the carry retargets on another fresh gate (the commit still stamps and publishes the seal's handle) | host-core `successor_swap`, `warm_successor`, `live_routes` | green: an equivalent mutant. A fresh gate's first stamp at `r` is the key `(1, r)` on every gate, and the commit's stamp on the seal's handle publishes that same key, so the cells' records are read exactly as before. The base had the same blind spot through `stamp_held_gate` on another plan; with the stamp now on the one handle the seal returns, no production path names another gate. |

**No bit moved.** #2412's scratch instrumentation (`PreparedRenderPlan::render_block` folds every
rendered output block into a thread-local FNV-1a accumulator whose destructor appends
`<binary> <thread> <blocks> <hash>` in one `write`), applied by one script to archives of the base
(`35898c413`) and of the head checkpoint (`7b27a1633`), each in its own target dir, on #2412's
suite set: graph, graph-compiler and builtins-compiler every target; source lib; host-core lib and
`successor_swap`, `warm_successor`, `withdrawn_successor`, `live_routes`, `browser_successor`,
`swap_block_realtime`, `timeline`, `collapse_arming`, `randomized` and `source_read_clock`
(`control-provider` on); control-plane lib. 817 tests passed on each tree, 348 rows each. The two
rows of the concurrent-producer tests (`live_routes_render_without_allocating`,
`live_sends_render_without_allocating`) differ, as they did between two base reruns in #2412, and
are excluded as before. The other 346 rows (62 889 blocks) are identical; the sorted listing has
sha256 `42f79994865b97d9d2bc3f3adf8d4ebbec1fc8c0bc9794148cc762c9c46e8854` on both trees, the same
as #2412's. `graph_fixture --check` passes at head. The render path is untouched: the plan lost a
field render never read.

**Gates (head).** `F` = builtins-compiler, graph, effect-compiler, session and engine test-support
plus `engine/realtime-audit`.
- `cargo test --locked -p engine -p graph --all-targets` (graph and engine test-support,
  `engine/realtime-audit`): pass; `--doc`: 14 passed (12 at #2412, plus the new pair).
- `cargo test --locked -p graph-compiler -p builtins-compiler -p source -p rack --all-targets`
  (`F`): 412 passed, 4 ignored.
- `cargo test --locked -p host-core --all-targets` (`F`, host-core test-support): 473 passed, 8
  ignored; with `host-core/control-provider`: 618 passed, 8 ignored; `--doc`: 9 passed.
- `cargo test --locked -p control-plane -p capi --all-targets` (control-plane, host-core, graph,
  session and engine test-support): 340 passed. `--doc` for graph-compiler, builtins-compiler,
  control-plane and capi: 12 passed.
- `cargo test --locked --release -p audit -p bench -p console-workload`: 103 passed.
- No test-support features: `cargo check --locked --workspace` (and `--all-targets`) and
  `cargo build --locked --release -p audit -p bench -p capi -p session-validator -p realtime-policy
  -p bench-support -p wasm-gates`: clean.
- `cargo clippy --locked --all-targets -- -D warnings` for engine, graph, graph-compiler,
  builtins-compiler, source, host-core, control-plane, capi, bench, bench-support, audit and
  console-workload, with the workspace test-support features and without them: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps` for engine, graph, builtins-compiler,
  graph-compiler, host-core and control-plane, with and without test-support: clean.
- The web call-graph gate: `build-web-audioworklet.sh --named-twin`, then
  `check-web-audioworklet.sh --without-metadata-regeneration`: rc 0, `miso_engine_web_v1_render`
  `indirect_sites=12`, as at #2412.
- `cargo fmt --check`, `scripts/check-workspace-policy.sh`, `scripts/test-workspace-policy.sh`,
  `scripts/check-rack-policy.sh` (rack is untouched), `graph_fixture --check` and
  `realtime_policy` (314 marked regions in 48 files): pass.

**Follow-ups.** None filed. The equivalent mutant above is not a gap this issue opens: a test that
stamps on a gate other than the plan's would need a gate whose publication count differs from the
plan's, which no production path can build.
