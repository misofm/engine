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
(empty)
