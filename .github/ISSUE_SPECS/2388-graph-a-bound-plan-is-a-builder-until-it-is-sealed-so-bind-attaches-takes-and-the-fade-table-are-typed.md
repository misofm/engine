# Graph: a bound plan is a builder until it is sealed, so bind attaches, takes and the fade table are typed too

Successor of #2375, which made the carry install sequence a typestate: `graph::carry_builder(plan, program)` takes the plan by value from its program to `seal()`, which returns it. So no carry section can be installed without its program, and a half-assembled hand-over can't render or be published. Before the builder takes the plan and after it returns it, the bind-to-publish span is still a plain `PreparedRenderPlan` guarded by run-time rules:

- The three one-shot takes (`take_compensation_lines`, `take_strip_delay_lines`, `take_live_routes`, `crates/graph/src/lib.rs`) return their rows on the first call and an empty `Vec` on every later one. A second caller gets no rows and no error.
- The fade table (`install_fade_arms`) is a free function on the plan, refused on a second call (`GraphCarryInstallError::FadeArmsInstalled`, #2302). Its pairs are staged on the carry builder (`GraphCarryBuilder::fade_arms`). The only thing that orders the two is the run-time `StripLaneOutOfRange` check: a pair whose entry is past the table is refused. Nothing stops `install_fade_arms` on the plan `seal()` returned.
- A sealed plan can be handed to `carry_builder` again, and only a run-time refusal (`CarryProgramInstalled`, #2302) stops a second program.
- `prepare_or_project` (`crates/host-core/src/prepare.rs`) interleaves the joins with work on the same plan before the builder takes it. The work is bind reads (`builtin_input_lanes`, `builtin_strip_lanes`, `bank_effect_lanes`, `observer_rows`, `effect_carry_slot_bytes`) and a bind attach that writes to the plan (`attach_jump_lengths`).

## Contract
- Bind returns a bound plan that is not a `PreparedRenderPlan` and can't be rendered or published. Bind means `PreparedBuiltinsGraphArtifact::into_bound` and `into_bound_with_source_set` (`crates/builtins-compiler/src/lib.rs` ~8147, ~8204), which today return `PreparedBuiltinsGraphBound { pub plan, .. }` (~6605), and the graph-compiler bind entries they wrap.
- The bound plan hands out each bind identity table once, by value: the three takes become one call that consumes the right to make it, or bind itself returns the tables. "Empty on a second call" is not representable.
- The bind reads, the bind attach and the fade table install are methods of the bound plan. The fade table can be installed once, and the carry builder takes its pairs only after it. For example, the table install could return a token that `fade_arms` consumes, which retires the run-time ordering check.
- One `seal`, with or without a carry, is the only way to a `PreparedRenderPlan`. A sealed plan offers no install, so `CarryProgramInstalled` and `FadeArmsInstalled` become unrepresentable and are removed.
- No rendered bit, plan word or fixture byte moves. This is a refactor.

## Scope
- `crates/graph`: the bound-plan type, the builder entry, and removing the two refusals.
- `crates/builtins-compiler`: `into_bound` and `into_bound_with_source_set`, and `PreparedBuiltinsGraphBound`.
- `crates/graph-compiler`: its bind entries.
- `crates/host-core/src/prepare.rs`: `prepare_or_project`'s bind-to-publish span.
- The bind callers outside the crates: `tools/audit/src/builtins_graph.rs:741`, `tools/audit/src/fixture_builtins.rs:565`, `tools/audit/src/graph.rs:374` and `tools/console-workload/src/lib.rs:2363`.
- Every direct bind caller in the test suites of `graph`, `graph-compiler`, `builtins-compiler`, `host-core` and `source`.

Brief it with a declared file set. Split it if it can't reach a closable slice in half a day, for example the bound-plan type plus `prepare_or_project` first and the tools and test callers after.

## Gates
- Mutation evidence: each run-time rule the type replaces (a second take, a second fade table, a second program, a fade-arm carry before its table) becomes a compile error, shown as PR evidence, or keeps a test.
- No bit moved: rendered output and `graph_fixture` bytes are identical to base (PR evidence).
- Tests green for graph, graph-compiler, builtins-compiler, host-core (with and without `control-provider`), source and rack, and for the tools whose callers move: `cargo test --locked --release -p audit -p console-workload`.
- Clippy `-D warnings`, `cargo doc -D warnings`, `cargo fmt --check`, `scripts/check-workspace-policy.sh`, `graph_fixture --check`.

## Evidence / attempt record

### Attempt 1 (Terra), base `origin/main` @ `29f238005`

**Slice.** The whole issue in one slice: the bound-plan type, its builder entry and the two
refusals (`crates/graph`), `PreparedBuiltinsGraphBound` (`crates/builtins-compiler`),
`prepare_or_project`, the four tool callers and every direct test caller. Proposed successor
below, not filed.

**Claims re-verified at base** (`crates/graph/src/lib.rs`, `crates/host-core/src/prepare.rs` at
`29f238005`).
- The takes return rows once, then empty: `take_compensation_lines` 6089,
  `take_strip_delay_lines` 6186, `take_live_routes` 6194 (`core::mem::take`). Holds.
- `install_fade_arms` (4758) is a plan function refused on a second call (`FadeArmsInstalled`,
  4775), and `GraphCarryBuilder::fade_arms` orders its pairs after the table only by the range
  check (5440). It also works on a sealed plan. Holds.
- A sealed plan goes back into `carry_builder` (5201), refused only by `CarryProgramInstalled`
  (5230). Holds.
- `prepare_or_project` interleaves the attach (4906), the reads (4910, 4923, 5016, 5041, 5544),
  the takes (5098-5121) and the fade table (5597) on the plain plan before the builder (5700).
  Holds.
- Correction, not a drop: graph-compiler has no bind entry of its own. `into_bound*` wrap graph's
  `PreparedGraphPlan::bind` and `bind_with_source_set`, so those are the bind entries changed.
  graph-compiler's part is its re-exported alias and its test callers.

**Design.**
- `PreparedGraphPlan::bind` and `bind_with_source_set` return `GraphBoundPlan<S>`. It wraps the
  plan privately and has no `render`, so it can't be rendered or published (both need a
  `PreparedRenderPlan`). There are two ways to a `PreparedRenderPlan`: `GraphBoundPlan::seal`
  with no hand-over, and `GraphCarryBuilder::seal`.
- `S` names the one-shot steps the plan still has:
  - `BindTables` (from bind);
  - `TablesTaken`;
  - `FadeTable { entries }` (private field, so no caller can forge one).
- Each step consumes the plan and returns it in the next state:
  - `take_bind_tables(self) -> (GraphBoundPlan<TablesTaken>, GraphBindTables)` replaces the three
    takes. It is the one call, by value.
  - `install_fade_arms(self, arms)` is on `S: NoFadeTable`, a sealed trait for `BindTables` and
    `TablesTaken`. It returns `GraphBoundPlan<FadeTable>`. An empty `arms` installs nothing,
    which is what `prepare_or_project` did by not calling it.
  - `GraphCarryBuilder::fade_arms` exists only on `GraphCarryBuilder<FadeTable>`. It checks a
    pair's successor entry against the state's `entries`, the table's length taken at the install.
    That check now only validates the index; it no longer orders the pair after the table.
- The bind reads (`builtin_input_lanes`, `builtin_strip_lanes`, `bank_effect_lanes`,
  `observer_rows`, `effect_carry_slot_bytes`) and the bind attaches (`attach_jump_lengths`,
  `attach_automation_program`) are `&mut self` methods on every state. They lose their `Option`
  and `NotAGraphPlan` paths, because a bound plan is always a graph plan.
- `carry_builder<S>(GraphBoundPlan<S>, program)` returns `GraphCarryBuilder<S>`. A refusal hands
  back the `GraphBoundPlan<S>`.
- Removed as unrepresentable:
  - `GraphCarryInstallError::{CarryProgramInstalled, FadeArmsInstalled, NotAGraphPlan}`;
  - `GraphJumpLengthError::NotAGraphPlan`;
  - `GraphAutomationProgramError::NotAGraphPlan`;
  - the free functions `take_*`, `install_fade_arms`, the five bind reads, `attach_jump_lengths`
    and `attach_automation_program`.

  No other crate matched on these variants. The source test's `NotAGraphPlan` assertion no longer
  compiles and is deleted.
- builtins-compiler: `PreparedBuiltinsGraphBound<P = GraphBoundPlan>`. `seal()` returns
  `PreparedBuiltinsGraphSealed` (`= PreparedBuiltinsGraphBound<PreparedRenderPlan>`), which keeps
  the control halves and the test gate. `test_only_write` is on the sealed form. graph-compiler
  adds the matching alias `PreparedGraphBuiltinsSealed`.
- `prepare_or_project`:
  - the attach and the reads are methods;
  - the three takes are one `take_bind_tables` where the compensation lines were taken (all three
    are pure takes, so taking the other two earlier changes nothing);
  - the fade table is installed unconditionally (an empty install is a no-op, the same as the old
    skipped call);
  - `let plan = match carry_program { Some(_) => builder ... seal(), None => plan.seal() }`.

  The `host.inventory.identity` arms of the reads and takes are gone (unreachable: the plan is a
  graph plan). No outcome or error precedence moves.
- console-workload reads its effect lanes from the bound plan before the seal
  (`SessionRuntime::effect_lanes`; corrected in the second fold below). Its `bank_counters`, its
  ideal bank-slot runs and its reverb test read that copy.
- Test-only addition: `graph::test_only_bank_effect_lanes(&mut PreparedRenderPlan)` (doc-hidden,
  `test-support`). One test reads a host-prepared plan's lanes, which is sealed:
  host-core's `successor_swap::insert_forms`.
- Test callers move mechanically: a bind that is unwrapped and rendered gets `.seal()`. A fixture
  that also hand-builds a carry returns the bound plan:
  - graph-compiler's `console_banking::bind_arm`, now followed by `arm` on the sealed plan;
  - `silent_bank_skip::bind_unsealed`, which returns the bound plan and a closure that finishes it
    once sealed;
  - graph's `sidechain_pdc_plan` and `rt11` `bound`;
  - source's `bind` and `bind_with`.

  In those fixtures the mono-collapse arming and the gate take now run after the seal, as
  production's `prepare_or_project` runs them (5882 at base). Neither touches the carry sections.

**Mutation evidence: the run-time rules the type replaces are compile errors.** A scratch test
file (`crates/graph/tests/zz_scratch_2388.rs`, never committed) was written against the head API.
`rustc` output, trimmed:
```
second_take              E0599 no method named `take_bind_tables` found for struct `GraphBoundPlan<TablesTaken>`
second_fade_table        E0599 the method `install_fade_arms` exists for struct `GraphBoundPlan<FadeTable>`, but its trait bounds were not satisfied
second_program           E0308 expected `GraphBoundPlan<_>`, found `PreparedRenderPlan` (carry_builder on a sealed plan)
fade_pairs_before_table  E0599 no method named `fade_arms` found for struct `GraphCarryBuilder` (a `BindTables` builder)
fade_table_after_seal    E0599 no method named `install_fade_arms` found for struct `PreparedRenderPlan`
fade_table_mid_build     E0382 use of moved value: `plan` (the builder took it)
take_after_seal          E0425 cannot find function `take_compensation_lines` in crate `graph`
bind_read_after_seal     E0599 no method named `builtin_input_lanes` found for struct `PreparedRenderPlan`
render_unsealed          E0599 no method named `render` found for struct `GraphBoundPlan<S>`
publish_unsealed         E0308 expected `PreparedRenderPlan`, found `GraphBoundPlan` (plan_exchange)
forged_state             E0451 field `entries` of struct `graph::FadeTable` is private
forged_bound             E0451 fields `plan` and `state` of struct `graph::GraphBoundPlan` are private
foreign_state            E0277 the trait bound `Forged: graph::bound_state::Sealed` is not satisfied
```
The rows cover a second take, a second fade table, a second program, a fade-arm carry before its
table, and every install or read on a sealed plan, so the spec's four run-time rules are all among
them. The committed evidence is one `compile_fail,E0599` doctest on `GraphBoundPlan`: a bound plan
does not render. Its plain twin seals first (#1422 D2). It replaces #2375's `E0382` pair on
`GraphCarryBuilder`: a bound plan renders neither before nor after a builder takes it, so the
older pair's claim is a case of this one.

**Tests.**
- Deleted, because the rule they held is now a compile error (rows above):
  - `graph::tests::install_refuses_a_second_carry_program`;
  - `graph::runtime::tests::a_second_fade_table_is_refused_and_every_first_arm_still_fires`;
  - the `NotAGraphPlan` assertion in source's `install_refuses_out_of_range_occupied_and_repeated_sources`.
    Its refused-install-charges-nothing check now reads the refused `occupied` plan, sealed.
- Moved: `a_bad_input_move_is_refused_whole`, from `host-core/tests/successor_swap.rs` to
  `graph-compiler/tests/silent_bank_skip.rs`. A host-prepared plan is sealed, so a hand-over can be
  staged by hand only on a bound plan. Its claims stay:
  - a padding lane is refused `InputLaneOutOfRange`;
  - a repeated lane is refused `DuplicateInputLane`;
  - an unresolved predecessor lane refuses the whole hand-over at the swap block
    (`PredecessorMismatch`), and nothing moves.

  It is the only test of the first two refusals. The fixture is nine tracks of the silent-bank
  desk, so every width's last input bank is partial. Its "nothing moved" half compares every block
  through the plan's latency with a fresh twin. On this desk a first-block check alone would stay
  green under a carry that copies the resolving lanes, because the desk's latency hides the first
  block. (The base test's first-block check, on its own fixture, did go red; see the fold below.)
- New: the `GraphBoundPlan` doctest pair. Test value: a bound plan that dereferences to its
  `PreparedRenderPlan`, or exposes `render`, would let an unsealed plan render. No other test sees
  a type-level change.

**Mutations of the change (head).** The tree was restored after each one. Mutation runs touch the
restored file, so cargo rebuilds it.

| Mutation | Run | Result |
|---|---|---|
| `take_bind_tables` hands out empty tables, leaving the rows in the executor | `host-core --test successor_swap` | red, 24 of 128 |
| the install's `FadeTable { entries }` is 0, so every pair is past the table | same | red, 6: `a_pending_fade_survives_a_second_swap_on_every_width`, `a_pending_route_fade_survives_a_second_swap_on_every_width`, `a_fired_fade_does_not_repeat_on_every_width` and three route-fade rows |
| `bound_from_plan` attaches no automation program | `builtins-compiler --all-targets` | green |
| same | `capi --test resource_lifecycle` | red: `capi_retained_bytes_charge_every_byte_the_compile_retains` |
| `GraphBoundPlan` derefs to `PreparedRenderPlan` | `graph --doc` | red: the `compile_fail` doctest |
| the swap block skips the whole-section input check (copies the resolving lanes) | `graph-compiler --test silent_bank_skip a_bad_input_move` | red at the outcome: `Carried`, not `PredecessorMismatch`. With that assertion removed, red at the bit comparison (block 11) |
| `input_lane_mut` accepts a padding lane | same | red: `Ok(())`, not `Err(InputLaneOutOfRange)` |

**No bit moved.** #2375's scratch instrumentation, never committed, was applied identically to
`git archive origin/main` (`29f238005`) and `git archive` of the head code commit (`b72eb85ef`).
`PreparedRenderPlan::render_block` FNV-1a folds every rendered output block into a thread-local
accumulator, and the accumulator's destructor appends `<test binary> <test thread> <blocks>
<hash>` to a file.

Suites, with each crate's test-support features:
- `graph`, `graph-compiler` and `builtins-compiler`, every target;
- `source` lib;
- `host-core` lib and `successor_swap`, `warm_successor`, `withdrawn_successor`, `live_routes`,
  `browser_successor`, `swap_block_realtime` and `timeline` (`control-provider` on).

Both trees give 310 rows. Excluded:
- the moved `a_bad_input_move_is_refused_whole` (a different fixture at head);
- `live_routes_render_without_allocating` and `live_sends_render_without_allocating`, which publish
  from a concurrent producer thread, so their hashes are timing (#2375 excluded them too).

The other 307 rows (61 226 blocks) are identical on base and head: sorted listing sha256
`f458c08fdda34fec9ceb0abb0b5d387df7d69bac359e2ac01049ba2939f52a88` on both. `graph_fixture
--check` passes at head, so the fixture bytes are unchanged. No render path, executor field or
plan word is touched.

**Gates (head).**
- `cargo test --locked -p graph --all-targets --features graph/test-support`: 148 passed. Base
  had 150; the two tests named above are deleted.
- `-p graph-compiler --all-targets` (builtins-compiler, graph, effect-compiler and session
  test-support): 195 passed, 2 ignored. That count includes the moved test.
- `-p builtins-compiler --all-targets` (test-support): 58 passed, 2 ignored.
- `-p source --all-targets --features graph/test-support`: 84 passed.
- `-p rack --all-targets`: 75 passed.
- `-p host-core --all-targets`, with the workspace test-support features: 473 passed, 8 ignored.
  With `host-core/control-provider` added: 618 passed, 8 ignored. Base fold had 619; one test
  moved out.
- Doctests of graph, builtins-compiler, graph-compiler and host-core: 23 passed. That includes the
  new `compile_fail,E0599` pair.
- `cargo test --locked --release -p audit -p console-workload`: 84 passed.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: ok.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps` on graph, builtins-compiler,
  graph-compiler, host-core, builtins and source: ok.
- `cargo fmt --check`, `bash scripts/check-workspace-policy.sh` and `graph_fixture --check`: ok.
- `./target/release/realtime_policy`: ok (298 marked regions in 48 files). No marked region
  changed.

**Successor.** #2401. Its spec is at the coordinator's scratchpad `succ/t.md`, corrected in the
attempt 1 fold below.

**Verdict (attempt 1): PASS** (Sol, `c57c52d90`), minors only, folded below.

### Attempt 1 fold (Terra), same branch, `origin/main` @ `d060df5cd` merged in (JS and specs only)

1. **The moved test's history, corrected.** The base host-core test's first-block comparison
   went red under the partial-carry mutation, on its own fixture. Only the new desk fixture's
   latency would hide a first-block check. The "Moved" bullet above now says so.
2. **`test_only_bank_effect_lanes`'s doc, corrected.** The doc no longer says "render moves no
   binding". It now says that each row's location, owner and effect ID are bind's home location,
   and that its `kind` is the lane's current occupant's, which a pack can change (`lane_swap_fits`
   lets `BypassOnly` and `Uncontrolled` lanes trade, #2135). The `GraphBoundPlan` doc now says
   that a sealed plan offers no bind read "outside test-support".
3. **The successor, corrected and written as a ready-to-file spec** (`succ/t.md`):
   - `force_mono_collapse_off` is dropped: it is test-only and toggled between renders, not a
     bind-time join;
   - `take_revision_gate` is named as the engine-level one-shot it is, taken in control-plane
     `compile.rs` ~2470 and host-core `limiter_linked_session.rs` ~442;
   - `stamp_held_gate` (`prepare.rs` ~5905), `donate_sources` (`source.rs` ~1207) and the
     unreachable `host.inventory.identity` arm on `graph::plan_identity` (`prepare.rs` ~5771) are
     added.

**Gates (fold head).** `cargo test --locked -p graph --all-targets --features graph/test-support`:
148 passed. Graph doctests: 2 passed (the `compile_fail` pair). `RUSTDOCFLAGS="-D warnings"
cargo doc --locked --no-deps -p graph`: ok. `cargo fmt --check`: ok. The fold changes doc comments and this record
only, so no render path or test moved.

### Attempt 1 second fold (Terra): qualification on PR #2402

**The failure.** Three jobs failed with `error[E0425]: cannot find function
test_only_bank_effect_lanes in crate graph`:
- the AArch64 release digest gates;
- the release-mode lane, math and wasm-gates digest gates;
- the wasm guests' "Build the wasmtime runner".

console-workload's **library** called the `test-support`-gated read, and wasm-gates, wasm-gate-guest
and bench depend on that library without the feature. The local `cargo test --release -p audit -p
console-workload` gate passed only because console-workload's own dev-dependency turns
`graph/test-support` on.

**The fix.** The read is a bind read, so console-workload now takes it from the bound plan before
the seal (`bound.plan.bank_effect_lanes()` in `SessionRuntime::build`) and keeps the rows in
`SessionRuntime::effect_lanes`. `bank_counters`, `ideal_bank_slot_runs` and the reverb test read
that copy. Each row's location, owner and effect ID are bind's home location, the same whenever
they are read, and none of these readers uses `kind`. So the evidence is unchanged.
`test_only_bank_effect_lanes` stays test-support only, for host-core's `insert_forms` (built with
`host-core/test-support`, which turns on `graph/test-support`).

**Gates (second fold head).** The three failing jobs' own commands, run locally:
- `cargo build --locked --release -p wasm-gates`: ok.
- `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`: 129 passed,
  8 ignored. This is a superset of the AArch64 job's `-p wasm-gates` step, run here on x86-64.

Also run:
- `cargo build --locked --release -p bench`: ok.
- `cargo check --locked --workspace` (no features): ok.
- `cargo test --locked --release -p audit -p console-workload`: 84 passed.
- `cargo clippy --locked -p console-workload --all-targets -- -D warnings`: ok.
- `cargo fmt --check`: ok.
