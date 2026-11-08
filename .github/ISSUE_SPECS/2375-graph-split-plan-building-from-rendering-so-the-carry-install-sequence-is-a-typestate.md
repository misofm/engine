# Graph: split plan building from rendering so the carry install sequence is a typestate

Successor of #2302 (GR-6), filed when the implicit-state-machine tracker #2258 closed. #2302 closed only GR-6's re-install gaps: a second fade table or carry program on one plan is now refused. The type split GR-6 asked for is not done.

## Problem
`GraphExecutor` holds both the build-time carry fields (bind, take, program, sections, fade arms, seal) and the render state. The install order is enforced by run-time refusals (`No*CarryProgram`, and #2302's re-install refusals) spread over about sixteen install and take functions, and by `prepare_or_project`'s interleaved install sequence in `crates/host-core/src/prepare.rs`. A render-ready plan with a half-installed carry is representable, and only the refusals keep it from rendering.

## Contract
- A `GraphCarryBuilder`, or an equivalent typestate, owns the install sequence. Only its sealed output can become a renderable executor, so a half-installed carry does not type-check.
- Re-installing a section with the same program keeps its documented replace semantics.
- No rendered bit, plan word or fixture byte moves. This is a refactor.

## Scope warning
This touches `GraphExecutor`'s carry fields, every install and take function, `prepare_or_project`, and every direct test caller in four crates. It must be briefed with a declared file set, and split if it cannot reach a closable slice in half a day, for example the builder first and the callers moved after.

## Gates
- Mutation evidence: each install-order refusal the typestate replaces either becomes a compile error, shown as PR evidence, or keeps a test.
- No bit moved: rendered output and `graph_fixture` bytes are identical to base (PR evidence).
- Workspace tests are green for graph, graph-compiler, host-core and rack.

## Evidence / attempt record

### Attempt 1 (Terra), base `main` @ `90932ff03`

**Slice.** The issue's own split: this attempt lands the builder and moves every install caller
onto it (`prepare_or_project` and the direct test callers in `graph`, `graph-compiler`,
`host-core` and `source`). The bind-time reads and one-shot takes, and the fade table, stay as
they are; the bound-plan typestate that would subsume them is proposed as a successor below.

**Claims re-verified at base.**
- `GraphExecutor` holds the build-time carry fields beside the render state
  (`crates/graph/src/lib.rs` 4320-4441 at base), and every carry section was a free `install_*`
  function on `&mut PreparedRenderPlan` guarded by a run-time `No*CarryProgram` refusal: inputs
  (4586), strips (4662), fade-arm carry (4929), effects (5258), observers and observations (5582,
  5650), compensation lines (5820), strip delays (5959), prime (6077), routes (6312). Holds.
- Nothing stopped a half-assembled hand-over from rendering: the plan stayed a plain
  `PreparedRenderPlan` between installs, and `prepare_or_project` interleaved them with reads
  (`crates/host-core/src/prepare.rs` 4905 program, 5475 inputs, 5520 a slot-bytes read, 5547
  strips, 5575/5638 fade table and carry, 5656-5670 effects/observers/observations, 5682 a late
  program, 5693-5710 lines, sends, prime). Holds.
- The three takes (`take_compensation_lines`, `take_strip_delay_lines`, `take_live_routes`) are
  bind-identity reads, not part of the install order: they consult no program and install
  nothing. Kept out of this slice (successor below).

**Design.** `graph::carry_builder(&mut plan, program) -> Result<GraphCarryBuilder<'_>, _>` runs
the old program validation (`NotAGraphPlan`, `CarryProgramInstalled`, `NoSourceSet`, source
index checks). The builder holds `&mut GraphExecutor` for its life and *stages* each section
(`builtin_inputs`, `builtin_strips`, `fade_arms`, `effects`, `observers`, `observations`,
`delays`, `strip_delays`, `routes`, `prime`) with the old per-section validation, unchanged.
`seal(self)` writes the program and every staged section into the executor at once, and is the
only control-thread writer of those fields (render still updates the state they hold). Consequences:
- No section can be installed without the program it was validated for: there is no other API.
- While the builder lives the plan cannot render, be published, take a fade table or start a
  second builder (all need the borrow the builder holds).
- A builder dropped unsealed leaves the plan as it was (nothing is written before the seal), so
  a half-installed carry is not representable even on an error path.
- The executor's field layout and every render-path read are untouched; seal writes exactly the
  values the old installs wrote. Two allocations move to the seal, off the render thread as
  before: the effect section's carried slots (#1633 D1) and the observer record. A re-staged
  effect section now allocates slots for its final moves only (the old code allocated for every
  install call; no caller installed twice).
- `install_fade_arms` stays a plan function, not a builder section: a plan without a program may
  hold a fade table (a hand-prepared successor with a rate change has no program), and its pairs
  are staged on the builder, which can only start after the table (the builder holds the plan).
- `GraphCarryInstallError` loses the seven variants the typestate makes unrepresentable
  (`NoCarryProgram`, `NoStripCarryProgram`, `NoEffectCarryProgram`, `NoObserverCarryProgram`,
  `NoDelayCarryProgram`, `NoStripDelayCarryProgram`, `NoRouteCarryProgram`);
  `NoPrimeCarryProgram` is renamed `NoPrimeLead`, its two remaining causes (no source set, a zero
  lead). No crate matched on any of them.

**`prepare_or_project`.** The reads stay where they were; the slot-bytes read is before the
builder, as before. The fade table install moves ahead of the builder (it was between the strip
and effect sections; it only arms lanes, which no other section reads). The builder then takes
the plan from its program to its seal in the old section order. The late program (a line or send
carrying without the join's program) is now decided before the builder, from the same
conditions. One branch preserves base behaviour explicitly: with no join program, a nonempty
input, strip, fade-arm, effect, observer or observation section was refused by its install
(`host.carry.install`) before the late program could go in, so it still is; and a prime with no
program at all is still refused. Not reachable from either host (both refuse a rate or quantum
change, so every host successor has a join program); kept so the refactor moves no outcome.
Behaviour change, internal faults only: an internal-fault `host.carry.install` from the input
section (never seen) is now reported after the resource admission rather than before it.

**Mutation evidence: the refusals the typestate replaces are compile errors.** A scratch test
file, never committed, written against the head API (`rustc` output, trimmed):
```
section_without_a_program  E0425 cannot find function `install_builtin_input_carry` in this scope
render_mid_build           E0499 cannot borrow `*plan` as mutable more than once at a time (plan.render)
second_builder_mid_build   E0499 cannot borrow `*plan` as mutable more than once at a time (carry_builder)
fade_table_mid_build       E0499 cannot borrow `*plan` as mutable more than once at a time (install_fade_arms)
section_after_seal         E0382 borrow of moved value: `carry` (moved by `seal`)
publish_mid_build          E0505 cannot move out of `plan` because it is borrowed
builder_without_a_program  E0451 fields `executor`, `program` and `sections` of struct `GraphCarryBuilder` are private
```
A section on a bare plan (every old `No*CarryProgram` path) has no function to call, and a
builder cannot be assembled without `carry_builder`'s program; a render, a second builder or a
fade table install while a hand-over is half assembled is a second mutable borrow; a section
after the seal uses a moved builder; publishing (moving) the plan mid-build moves out of a
borrow. The render case is committed as a `compile_fail,E0499` doctest on `GraphCarryBuilder`
with a plain twin that seals first (the #1422 D2 pattern).

**Refusals that stay run-time, each with its test.**
- `CarryProgramInstalled` (a second builder on a sealed plan): `graph::tests::
  install_refuses_a_second_carry_program`, moved onto the builder.
- `FadeArmsInstalled`: `graph::runtime::tests::a_second_fade_table_is_refused_and_every_first_arm_still_fires`,
  unchanged.
- The per-section range and duplicate refusals: unchanged code, their tests moved onto the
  builder (`a_bad_input_move_is_refused_whole` in `host-core/tests/successor_swap.rs`, whose
  `NoCarryProgram` assertion is deleted because that call no longer compiles; the source-index
  tests in `crates/source/src/lib.rs`).

**Tests.** No new `#[test]`. Moved onto the builder, same assertions: the two `graph::tests`
program tests, `rt11_swap_carry_alloc`, `graph-compiler`'s `console_banking` and
`silent_bank_skip` carried pairs, `host-core`'s `a_bad_input_move_is_refused_whole`, and the
`source` carry tests. New: the `GraphCarryBuilder` doctest pair. Its test value: a builder that
stopped borrowing the plan (writing sections through as the old installs did, or owning a copy)
would let a half-assembled hand-over render, and turns the `compile_fail` doctest red; no
existing test sees a type-level change.

**Mutations of the change (head).**
Each mutation run against `host-core --test successor_swap`, `--test warm_successor`
(`control-provider`), `graph --lib` and `graph-compiler --tests`; tree restored after each.
- The seal drops the input section: red, 116 of 126 `successor_swap` and 22 of 23
  `warm_successor` (`a_bad_input_move_is_refused_whole` among them).
- The seal allocates no carried effect slots (#1633 D1): red, 36 `successor_swap`,
  `warm_successor` aborts, and `graph-compiler`'s
  `console_banking::a_swap_carries_a_per_node_effects_agreement_with_its_state`.
- The seal drops the fade-arm section: red, `a_pending_fade_survives_a_second_swap_on_every_width`
  and `a_pending_route_fade_survives_a_second_swap_on_every_width`.
- Preparation stages no fade-arm section: red, the same two.
- The seal assembles the observer section without its taps: red, 9 `observation_carry::*` in
  `successor_swap`.
- The seal drops the prime: red, 7 `warm_successor` (`a_primed_warm_successor_continues_its_predecessor_bit_for_bit` among them).
- `carry_builder` accepts a second program on a sealed plan: red,
  `graph::tests::install_refuses_a_second_carry_program` only.
- Preparation's two branches that keep base behaviour for a successor with no join program
  (the late line/send program, and the refusal of any other section without a program): green
  under every `host-core` test target (616 passed, `control-provider` on). Unreachable from either
  host (both refuse a rate or quantum change, so every host successor has a join program); the
  same paths had no test at base, where they were the `!has_carry_program` install and the
  `No*CarryProgram` refusals reached through `prepare_or_project`. Listed for the successor.

**No bit moved.** Scratch instrumentation, never committed and applied identically to a base copy (`git archive
90932ff03`) and to head: `PreparedRenderPlan::render_block` FNV-1a-folds every rendered output
block into a thread-local accumulator, and the accumulator's destructor appends `<test thread>
<blocks> <hash>` to a file (one `write` a line). Suites: `graph` (lib and every test target),
`graph-compiler` (lib and every test target), `source` lib, and `host-core` `successor_swap`,
`warm_successor`, `withdrawn_successor` (`control-provider`), `live_routes`, `browser_successor`,
`swap_block_realtime` and lib, with the test-support features of #2302's run. Two tests are
excluded because their hashes differ between three runs of base itself: `graph-compiler`'s
`live_routes_render_without_allocating` and `host-core`'s `live_sends_render_without_allocating`
publish from a concurrent producer thread, so which block applies which record is timing.
The rest: 360 rows, 66 869 blocks, identical on base and head (sorted listing sha256
`daf12f21e71fca57e3fbf4bdd02f9b3490de828bfaa9e727ac2695103fc0b1d5` on both). The render path is
untouched by the diff; `graph_fixture --check` passes at head, so the committed fixture bytes are
unchanged.

**Gates (head).**
- `cargo test --locked -p graph --all-targets --features graph/test-support`: 150 passed.
- `cargo test --locked -p graph-compiler --all-targets` (builtins-compiler, graph,
  effect-compiler, session test-support): 194 passed, 2 ignored.
- `cargo test --locked -p source --all-targets --features graph/test-support`: 84 passed.
- `cargo test --locked -p rack --all-targets`: 75 passed.
- `cargo test --locked -p host-core --all-targets` with the workspace test-support features: 471
  passed, 8 ignored; with `host-core/control-provider` added: 616 passed, 8 ignored.
- Doctests of the five crates, same features: 23 passed (the new pair included).
- `cargo clippy --locked -p graph -p graph-compiler -p host-core -p source -p rack --all-targets
  -- -D warnings`, with and without the test-support and `control-provider` features: ok.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps` on the five crates
  (`host-core/control-provider`, as the workspace build unifies it): ok.
- `cargo fmt --check`: ok. `bash scripts/check-workspace-policy.sh`: ok.
  `graph_fixture --check`: ok. `./target/release/realtime_policy`: ok (298 marked regions in 48
  files).

**Proposed successor (not filed).** "Graph: a bound plan is a builder until it is sealed." `into_bound*` returns an unpublishable
bound plan whose bind-time reads (`builtin_input_lanes`, `builtin_strip_lanes`,
`bank_effect_lanes`, `observer_rows`, `effect_carry_slot_bytes`, `attach_jump_lengths`) are its
methods, whose three one-shot takes (`take_compensation_lines`, `take_strip_delay_lines`,
`take_live_routes`) are handed out once by value at bind (no "empty on a second call"), whose fade
table install is its method, and whose `seal`, with or without a carry, is the only way to a
`PreparedRenderPlan`. That retires the last two run-time refusals, `CarryProgramInstalled` and
`FadeArmsInstalled` (a sealed plan offers no install). It touches graph-compiler's
`into_bound_with_source_set` and every bind caller in the test suites of four crates, and
`prepare_or_project`'s whole bind-to-publish span. The same issue should prove or test the two
no-join-program branches of `prepare_or_project` named above (or show them unreachable and
remove them).
