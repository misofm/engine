# Graph: the bind-to-publish steps left on a sealed plan are typed too

Successor of #2388. #2388 made bind return a `graph::GraphBoundPlan`, a builder that is not a `PreparedRenderPlan`. Its bind reads, bind attaches, bind-table take and fade table are methods of the bound plan, and its seal, or its carry builder's, is the only way to a renderable plan. Some one-shot or bind-time steps were outside that scope. They still run on the sealed `PreparedRenderPlan`, or keep a run-time refusal, between bind and publication:

- **The mono-collapse join.** `PreparedRenderPlan::arm_mono_collapse` (`crates/engine/src/realtime/plan.rs` ~1350, doc-hidden) is the structural join of the mono collapse. `prepare_or_project` runs it on the sealed plan (`crates/host-core/src/prepare.rs` ~5882), and so do the graph-compiler and builtins-compiler test fixtures. Its doc says "until a caller does this, no chain collapses", so it is a once-per-plan bind step that nothing types.
- **The source-read offset.** `set_source_read_offset` is set on the sealed plan before the inventory (`prepare.rs` ~5769).
- **The revision gate.** `PreparedRenderPlan::take_revision_gate` is an engine-level one-shot: it returns `Some` once, then `None`. It is taken in control-plane (`crates/control-plane/src/compile.rs` ~2470), in host-core (`crates/host-core/src/limiter_linked_session.rs` ~442) and in many tests.
- **The held-gate stamp.** `PreparedRenderPlan::stamp_held_gate` (`prepare.rs` ~5905) stamps a successor's carried live differences on its gate pin after the seal.
- **The source donation.** `graph::donate_sources` (`crates/host-core/src/source.rs` ~1207, through `donate_consumers`) swaps source consumers between two plans neither of which render owns. It returns `false` for pairs its own check refuses.
- **An unreachable identity arm.** `graph::plan_identity(&mut plan)` returns `Option`, so `prepare_or_project` keeps an unreachable `host.inventory.identity` arm (`prepare.rs` ~5771). The plan is the graph plan bind just built.
- **Two attach refusals.** `GraphBoundPlan::attach_jump_lengths` and `attach_automation_program` keep their run-time `AlreadyAttached` refusals for a second call.

`force_mono_collapse_off` is not in scope. It is a test-only switch, toggled between renders, not a bind-time join.

## Contract
- Each step above either moves onto the bound plan (a method of `GraphBoundPlan`, before the seal) or is typed as a one-shot by value: a call that consumes the right to make it, so a second call does not type-check. Where a step must stay on the sealed plan, for example because publication needs it there, the issue records why. The step then keeps a test of its refusal.
- `prepare_or_project` reads `plan_identity` without an `Option`, for example from the bound plan before its seal. Its `host.inventory.identity` arm is removed.
- An engine-level step (`arm_mono_collapse`, `set_source_read_offset`, `take_revision_gate`, `stamp_held_gate`) may need a bound-plan surface in `engine`, or a graph-side wrapper. The brief chooses one, with a declared file set. This crosses into `crates/engine`.
- No rendered bit, plan word or fixture byte moves. This is a refactor.

## Scope
- `crates/engine/src/realtime/plan.rs`: the four engine-level steps, if they move or become typed.
- `crates/graph`: the bound-plan surface and the two attaches.
- `crates/host-core/src/prepare.rs` and `crates/host-core/src/source.rs`: `prepare_or_project`'s post-seal span and the donation.
- `crates/control-plane/src/compile.rs` and `crates/host-core/src/limiter_linked_session.rs`: the gate take.
- Every direct test caller of the moved steps in those crates and in `graph-compiler`, `builtins-compiler`, `capi` and the tools.

Brief it with a declared file set, and split it if it can't reach a closable slice in half a day. For example, the graph-side steps (the two attaches, `plan_identity`, the mono-collapse join) could come first, and the engine-level gate steps after.

## Gates
- Mutation evidence: each run-time rule a type replaces (a second gate take, a second attach, a second mono-collapse join) becomes a compile error, shown as PR evidence, or keeps a test.
- No bit moved: rendered output and `graph_fixture` bytes are identical to base (PR evidence, with #2375's method).
- `cargo test --locked` for engine, graph, graph-compiler, builtins-compiler, host-core (with and without `control-provider`), control-plane, source, capi and rack, plus `cargo test --locked --release -p audit -p console-workload`.
- Clippy `-D warnings`, `cargo doc -D warnings`, `cargo fmt --check`, `scripts/check-workspace-policy.sh`, `graph_fixture --check` and the realtime policy tool.

## Evidence / attempt record

### Attempt 1 (Terra), base `origin/main` @ `9d9bec3e7`

**Slice.** The graph-side steps: the two attaches, `plan_identity` and the mono-collapse join
(the spec's own suggested first part). The engine-level gate steps (`take_revision_gate`,
`stamp_held_gate`, `set_source_read_offset`) are a proposed successor, not filed. The donation
stays on the sealed plans, for the reason below.

**Claims re-verified at base** (`9d9bec3e7`).
- `PreparedRenderPlan::arm_mono_collapse` (`crates/engine/src/realtime/plan.rs:1350`,
  doc-hidden) is the join. `prepare_or_project` runs it on the sealed plan
  (`crates/host-core/src/prepare.rs:5883`), and so do the graph-compiler, builtins-compiler and
  graph test fixtures and console-workload's library (`tools/console-workload/src/lib.rs:2427`).
  Holds. The rule it carries is `BankChain::arm_mono_collapse`'s obligation
  (`crates/rack/src/lib.rs:4894`): arm "once, before this chain renders its first block". The
  engine asserted only the render-scope half; nothing stopped a sealed plan that had rendered
  from being armed.
- `set_source_read_offset` is set on the sealed plan (`prepare.rs:5769`). Holds.
- `take_revision_gate` (`plan.rs:1020`) returns `Some` once, then `None`; taken at
  `crates/control-plane/src/compile.rs:2471`, `crates/host-core/src/limiter_linked_session.rs:442`
  and 77 more sites in 41 files. Holds.
- `stamp_held_gate` (`plan.rs:1040`) stamps on the sealed plan (`prepare.rs:5905`) and returns
  `None` once the handle is taken. Holds.
- `graph::donate_sources` (`crates/host-core/src/source.rs:1207`) returns `false` for pairs its
  own check refuses. Holds; kept, see below.
- `prepare_or_project` keeps an unreachable `host.inventory.identity` arm on
  `graph::plan_identity(&mut plan)` (`prepare.rs:5771`). Holds.
- `GraphBoundPlan::attach_jump_lengths` and `attach_automation_program` refuse a second call
  with `AlreadyAttached` (`crates/graph/src/lib.rs:2080`, `:2132`). Holds. Only
  `crates/graph/tests/jump_lengths.rs:238` tested a refusal; the program's had no test.

**Design.**
- Two new `GraphBoundPlan` states, `ProgramAttached` and `JumpLengthsAttached`, between
  `BindTables` and `TablesTaken`. The one-shot steps run in a fixed order, and any may be
  skipped: program, jump-length cell, bind tables, fade table, seal. Each step consumes the plan
  and returns it in its own state, which offers only the later steps:
  - `attach_automation_program(self, Option<Arc<Program>>) -> GraphBoundPlan<ProgramAttached>`,
    on a `BindTables` plan only. It takes an `Option` so the type does not depend on whether the
    session has stored automation; `None` attaches nothing, as before.
  - `attach_jump_lengths(self, ..) -> (GraphBoundPlan<JumpLengthsAttached>, producer)`, on the
    sealed trait `NoJumpLengths` (`BindTables`, `ProgramAttached`).
  - `take_bind_tables` moves to `HoldsBindTables` (`BindTables`, `ProgramAttached`,
    `JumpLengthsAttached`); `NoFadeTable` gains the two new states.
  - `GraphAutomationProgramError` and `GraphJumpLengthError` are removed. No other crate
    matched on them.
- `GraphBoundPlan::plan_identity(&self) -> u64`, on every state. `prepare_or_project` reads it
  before the seal, and its `host.inventory.identity` arm is gone. The free
  `graph::plan_identity(&mut PreparedRenderPlan) -> Option<u64>` stays for test callers that read
  a sealed predecessor; source's `None`-for-a-plain-plan assertion keeps it tested.
- The mono-collapse join moves onto the bound plan: `GraphBoundPlan::arm_mono_collapse(&mut
  self, eligible)`, on every state, with the engine's render-scope assertion. The engine's
  `PreparedRenderPlan::arm_mono_collapse` and the `PreparedPlanExecutor::arm_mono_collapse`
  trait hook are removed (graph was the hook's only implementer), so a sealed plan, which may
  have rendered, has no join. This types the half of the chain's obligation that mattered: a
  bound plan cannot render, so a chain is armed before its first block.
  - **Not one-shot, on purpose.** A second join before the seal is harmless: each call answers
    every chain afresh from `eligible` (`crates/graph/src/runtime.rs:5072`,
    `BankChain::arm_mono_collapse` is `collapse_source = structural`), so the last call before the
    seal wins and no chain has rendered under an earlier answer. A one-shot state would add a
    sixth state for no defect.
  - `prepare_or_project` joins just before the hand-over builder takes the plan. The join is
    infallible and reads only the compiled session, so moving it ahead of the projection checks
    changes no outcome or error precedence. No carry section reads the arming before the swap
    block: `GraphCarryBuilder::seal` (`crates/graph/src/lib.rs`) only moves staged sections into
    the executor, and the one cross-chain check that compares arming (`BankChain::swap_lanes`,
    `crates/rack/src/lib.rs:5129`) runs at render, after the seal either way.
  - `force_mono_collapse_off` stays on the sealed plan (out of scope, as the spec says).
- Callers moved: builtins-compiler's `bound_from_plan` attaches with the `Option`
  (`PreparedBuiltinsGraphBound`'s default plan type is now `GraphBoundPlan<ProgramAttached>`);
  `prepare_or_project` destructures the attach; console-workload joins on `bound.plan` before
  its seal. Test fixtures that armed after the seal now arm before it: graph `rt9`,
  graph-compiler `bank_levels`, `silent_bank_skip::bind_unsealed`, `console_banking::{arm,
  join, seal_forced}`, the lib tests' `bind_queued_eq(.., mono_collapse)` and the production
  console test, and builtins-compiler's `mono_armed` over the new
  `prepared_live_graph_fixture_bound`. Doc references to the engine join now name the graph one.
- control-plane keeps its `host.jump_lengths.attach` code for a handle set without a jump-length
  writer; only its comment changed (the attach no longer refuses).

**Left on the sealed plan, with the reason.**
- **The source donation.** The donor is a withdrawn plan: render owned it and handed it back
  through retirement, so it is a sealed `PreparedRenderPlan` by construction and cannot be a
  bound plan. The successor is the host's prepared plan, sealed at `prepare_or_project`'s end,
  and the donation runs after the protocol commit (`crates/control-plane/src/control.rs:6743`).
  The check-to-apply hand-off is already typed (`check_donation` returns the `DonationPlan`
  `apply_donation` consumes). Its run-time re-check is deliberate: it is the acked-batch guard
  against a caller that breaks the contract between check and apply (`source.rs:1214-1227`), so
  `donate_sources`'s `false` is that guard's answer, not a forgotten refusal. Its refusals keep
  their tests: `withdrawn_successor`'s `a_refused_or_mismatched_donation_moves_nothing`,
  `a_donation_never_separates_an_acked_ring_from_its_producer` and
  `a_successor_fed_between_the_check_and_the_apply_moves_nothing`.
- **The engine-level gate steps** are not dropped; they are the successor below.

**Mutation evidence: the run-time rules the types replace are compile errors.** A scratch test
file (`crates/graph/tests/zz_scratch_2401.rs`, never committed) against the head API; `rustc`,
trimmed:
```
second_program            E0599 no method named `attach_automation_program` found for struct `GraphBoundPlan<ProgramAttached>`
second_jump_cell          E0599 the method `attach_jump_lengths` exists for struct `GraphBoundPlan<JumpLengthsAttached>`, but its trait bounds were not satisfied
program_after_jump_cell   E0599 no method named `attach_automation_program` found for struct `GraphBoundPlan<JumpLengthsAttached>`
jump_cell_after_tables    E0599 the method `attach_jump_lengths` exists for struct `GraphBoundPlan<TablesTaken>`, but its trait bounds were not satisfied
jump_cell_after_fade      E0599 the method `attach_jump_lengths` exists for struct `GraphBoundPlan<FadeTable>`, but its trait bounds were not satisfied
jump_cell_after_seal      E0599 no method named `attach_jump_lengths` found for struct `PreparedRenderPlan`
program_after_seal        E0599 no method named `attach_automation_program` found for struct `PreparedRenderPlan`
join_after_seal           E0599 no method named `arm_mono_collapse` found for struct `PreparedRenderPlan`
join_after_carry_seal     E0599 no method named `arm_mono_collapse` found for struct `PreparedRenderPlan`
identity_after_seal       E0599 no method named `plan_identity` found for struct `PreparedRenderPlan`
old_refusals              E0433 cannot find type `GraphJumpLengthError` / `GraphAutomationProgramError`
forged_state              E0277 the trait bound `Forged: graph::bound_state::Sealed` is not satisfied
```
The rows cover a second attach of each kind, an attach out of order, and the join, the attaches
and the identity on a sealed plan. The committed evidence is one `compile_fail,E0599` doctest
pair on `GraphBoundPlan::arm_mono_collapse`: a sealed plan has no join, its plain twin joins the
bound plan (#1422 D2). Test value: red if `PreparedRenderPlan` offers the join again, so a plan
that has rendered could be armed and break the rack's agreement invariant; no other test sees a
type-level change.

**Tests.**
- Deleted, because the rule it held is now a compile error (`second_jump_cell` above): the
  `AlreadyAttached` assertion in `graph/tests/jump_lengths.rs`'s
  `the_block_entry_holds_the_newest_published_lengths`. The rest of that test is unchanged.
- New: the doctest pair above.

**Mutations of the change (head).** The tree was restored after each.

| Mutation | Run | Result |
|---|---|---|
| `attach_jump_lengths` builds the reader but does not install it | `graph --test jump_lengths` | red at the retained-row assertion (line 220) |
| `attach_automation_program` drops the program | `capi --test resource_lifecycle` | red: `capi_retained_bytes_charge_every_byte_the_compile_retains` |
| `GraphBoundPlan::arm_mono_collapse` arms nothing | `host-core --test collapse_arming` | red, 5 of 8 |
| `GraphBoundPlan::plan_identity` returns the identity plus one | `host-core --test successor_swap` (control-provider) | red, 102 of 128 |
| the engine's `PreparedRenderPlan::arm_mono_collapse` restored (base `plan.rs`) | `graph --doc` | red: the new `compile_fail` doctest |
| `impl NoJumpLengths for JumpLengthsAttached` | scratch file | `second_jump_cell` compiles (no committed test; the scratch rows are the evidence) |

**No bit moved.** #2375's scratch instrumentation, never committed, applied identically to the
base files (`9d9bec3e7`) and the head files in this worktree: `PreparedRenderPlan::render_block`
FNV-1a folds every rendered output block into a thread-local accumulator, and its destructor
appends `<test binary> <test thread> <blocks> <hash>` to a file. Suites, with each crate's
test-support features:
- `graph`, `graph-compiler` and `builtins-compiler`, every target;
- `source` lib;
- `host-core` lib and `successor_swap`, `warm_successor`, `withdrawn_successor`, `live_routes`,
  `browser_successor`, `swap_block_realtime`, `timeline`, `collapse_arming` and `randomized`
  (`control-provider` on). The last two are added to #2388's set because they arm through
  `prepare_or_project`.

781 tests passed on each tree, 319 rows each. The first runs' destructors wrote a row in several
writes, so concurrent test threads interleaved 4 rows on each tree. The three binaries those rows
came from (graph lib, source lib, host-core `live_routes`) were re-run on both trees with one
write per row, and their rows replace the first runs'. Two base rows of graph-compiler's
`live_routes` stayed interleaved; they read, by hand, `a_bus_whose_every_send_goes_inactive_is_refilled_with_positive_zero 6 f3638f875fbc9952`
and `a_new_cell_value_ramps_from_the_coefficients_the_route_is_at 5 90695e68fc2aaaf4`, the same as
head. Excluded, as #2388 did: `live_routes_render_without_allocating` and
`live_sends_render_without_allocating`, which publish from a concurrent producer thread, so their
hashes are timing (they differ between runs of one tree too). The other 317 rows (60 082 blocks)
are identical on base and head; the 315 machine-compared rows are equal line for line, and the
head listing of all 317 has sha256 `50760397edb47202fa30e0752c6b8ba394594f78dfb95a70693a2a6686df6f97`.
`graph_fixture --check` passes at head, so the fixture bytes are unchanged. No render-path body,
executor field or plan word changed: the `GraphExecutor` trait impl lost a control-side hook, and
the realtime policy tool reports the same 298 marked regions.

**Gates (head).**
- `cargo test --locked -p graph --all-targets --features graph/test-support`: 148 passed. Graph
  doctests: 4 passed (the new `compile_fail` pair included).
- `-p graph-compiler --all-targets` (graph, builtins-compiler, effect-compiler and session
  test-support): 195 passed, 2 ignored.
- `-p builtins-compiler --all-targets` (test-support): 58 passed, 2 ignored.
- `-p source --all-targets --features graph/test-support`: 84 passed.
- `-p host-core --all-targets` (host-core, graph, builtins-compiler and engine test-support): 473
  passed, 8 ignored; with `host-core/control-provider`: 618 passed, 8 ignored.
- `-p engine --all-targets --features engine/test-support,engine/realtime-audit`: 95 passed.
  Engine doctests: 2 passed.
- `-p control-plane --all-targets --features control-plane/test-support`: 102 passed.
- `-p capi --all-targets`: 238 passed. `-p rack --all-targets`: 75 passed.
- `cargo test --locked --release -p audit -p console-workload`: 84 passed.
- `cargo clippy --locked --all-targets -- -D warnings` on engine, graph, graph-compiler,
  builtins-compiler, host-core, control-plane, rack, effect-contract and console-workload, with
  and without the test-support features (and `host-core/control-provider`): ok.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps` on the same crates but
  console-workload: ok.
- #2388's lesson, the builds without test-support features: `cargo check --locked --workspace`
  and `cargo build --locked --release -p bench -p realtime-policy -p wasm-gates`: ok.
- `cargo fmt --check`, `bash scripts/check-workspace-policy.sh` and `graph_fixture --check`: ok.
- `./target/release/realtime_policy`: ok (298 marked regions in 48 files).

**Successor.** #2412: the engine-level gate steps (`take_revision_gate`,
`stamp_held_gate`, `set_source_read_offset`), low priority. Its ready-to-file spec is at the
coordinator's scratchpad `succ/x.md`, written in the attempt 1 fold below.

**Verdict (attempt 1): PASS** (Sol, `a729a1e7a`), minors only, folded below.

### Attempt 1 fold (Terra), same branch, `origin/main` @ `231494259` merged in (clean)

1. **The `GraphBoundPlan` doc, corrected.** It no longer says a sealed plan "offers none of"
   the join, the identity and the bind reads. It now says a sealed plan has no join and no bind
   read outside test-support, and that its identity is read through `graph::plan_identity`.
2. **A committed `compile_fail,E0599` doctest pair on `attach_jump_lengths`.** A second attach on
   a `JumpLengthsAttached` plan fails, and its plain twin, which attaches once, passes (#1422 D2).
   Test value: red if a `JumpLengthsAttached` plan takes the attach again (for example
   `impl NoJumpLengths for JumpLengthsAttached`). A second cell would replace the reader and leave
   the first producer's acked writes unread, which is the acked-batch question; no other test
   saw that mutant. Mutation: with that impl added, `graph --doc` goes red on the new
   `compile_fail` doctest (2 passed, 1 failed); restored, all 6 graph doctests pass. The
   mutation table's last row above is superseded: the mutant is now caught by a committed test.
3. **`PlanStateInventory::plan_identity`'s doc** (`crates/host-core/src/prepare.rs`) now cites
   `graph::GraphBoundPlan::plan_identity`, read before the seal.
4. **The successor** is written as a ready-to-file stateless spec (`succ/x.md`, H1 title first).
   It proposes a `(plan, gate)` helper per crate's test support to absorb most of the 79
   `take_revision_gate()` sites, and the `#2412` placeholder above marks where its number
   goes.

**Gates (fold head, private target dir, `-j 4`).**
- `cargo test --locked -p graph --all-targets --features graph/test-support`: 148 passed.
- Graph doctests: 6 passed (three `compile_fail` pairs).
- `cargo clippy --locked -p graph -p host-core --all-targets` (graph and host-core test-support,
  `host-core/control-provider`) `-- -D warnings`: ok.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p graph -p host-core`: ok.
- `cargo fmt --check`: ok.
- The fold changes doc comments, one doctest pair and this record, so no render path or test
  body moved.

### Attempt 1 second fold (Terra): qualification on PR #2413

**The failure.** In "fmt, clippy, doc, and hermetic policy gates", the step "Workspace policy
mutation tests" (`bash scripts/test-workspace-policy.sh`) failed. That script ends by running the
gate self-tests, and the rack policy refused the head:
`rack policy failure: control-plane, I/O, threading, synchronization, or logging leaked into rack
render code`, at `crates/rack/src/lib.rs:3868` and `:4895`. The doc comments this issue
retargeted there named the join `graph::GraphBoundPlan::arm_mono_collapse`.
`scripts/check-rack-policy.sh` forbids `\bgraph::` anywhere under `crates/rack/src`, comments
included, so that rack never names a graph path. The log's "directed fault unexpectedly passed"
and "rg exit 9" lines are that script's own counter-mutant controls, expected on every run; the
job fails only on the rack line.

**The fix.** The two rack comments now say "the graph crate's `GraphBoundPlan::arm_mono_collapse`".
They still make the same claim, and they no longer spell a graph path. The gate is unchanged.

**Gates (second fold head).**
- `bash scripts/test-workspace-policy.sh`: ok.
- `bash scripts/check-rack-policy.sh`: PASS. `bash scripts/test-rack-policy.sh`: ok.
- The job's other policy gates, rerun: workspace, graph, host-core, session, builtins, protocol
  control, bench, env vocabulary and conformance boundaries: ok.
- `cargo fmt --check`: ok.
- The fold changes two rack doc comments and this record.
