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
they are; the bound-plan typestate that would subsume them is filed as successor #2388 (attempt 2, item 5).

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
(Superseded by attempt 2, item 2.) Behaviour change, internal faults only: an internal-fault `host.carry.install` from the input
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

**Successor.** Superseded by attempt 2's paragraph below.

**Verdict (attempt 1): FAIL** (Sol, `8a2f7867f`), one MAJOR and four MINORs. The builder itself
was judged sound.
1. MAJOR: the no-join-program branches in `prepare_or_project` (the `host.carry.install` refusal,
   the late `lines_or_sends_carry` program, and a prime with no program) were new code, reachable
   by preparing a successor at another rate by hand (as `tests/timeline.rs` does). Sol's
   reproducer reached the late program (`test_only_carry_counts == [0,0,0,0,1]`), and they were
   dead in effect: `adopt_predecessor_plan` refuses a different envelope before `carry_from`.
2. MINOR: the error-precedence note understated the reordering.
3. MINOR: a forgotten `seal()` still compiled, because the builder only borrowed the plan.
4. MINOR: `NoPrimeLead` also covered "no source set".
5. MINOR: the successor was to be written as a ready-to-file spec.

### Attempt 2 (Terra), same branch, `origin/main` @ `3ac00de23` merged in

**1. A successor with no join program carries nothing and is charged for nothing.** The proof is
now a comment in `prepare_or_project`, where the rule applies. The join builds a program for every
successor at its predecessor's rate and quantum, because the timeline carries (#1641 D3). So a
successor without one runs at another rate or quantum, which means it was prepared by hand; both
hosts refuse the change (#1290a). Such a plan never takes its predecessor's state. An exchange
refuses a candidate of another envelope at publication (`PlanPublisher::reserve_replacement`, `Incompatible`), and
`PreparedRenderPlan::adopt_predecessor_plan` (engine `realtime/plan.rs` 1445) returns
`PredecessorMismatch` before `carry_from`.

The fix: `fresh_timeline = successor.is_some() && carry_program.is_none()`. When it holds,
preparation clears these sections before anything is charged: the compensation-line moves, the
strip-delay moves and the prime's claim fills, the sends, and the carried entries of route fades
and fading copies. It also gives the prime no lead and no sidechain fills. So the line, send,
prime and fade-pair charges and the retained-bytes `debug_assert` terms are all zero, by
construction.

The other sections cannot be nonempty without a join program, because each needs an owner that
the program's condition already counts:
- inputs: `carried_inputs` is in the condition;
- strip moves need a fader or matrix join that is not prepared (`StripJoin::may_carry`);
- effect moves need an owner or surviving insert that is not prepared (`ChainJoin::may_carry`);
- meter moves need `meters_may_carry`;
- spectrum moves need the same rate and quantum;
- observations follow the effect moves;
- a strip's fade entry carries only with its fader move.

A `debug_assert` before the builder checks all of this. The `host.carry.install` refusal arm, the
late program and the prime-without-program refusal are deleted. The builder now starts only from
the join's program.

The route-fade-only case (a pending route-fade or copy entry with no join program) is handled the
same way: its entry is dropped with the lines. It is not refused, so no route-fade fixture is
needed to reach a refusal. Both hosts are unaffected: every host successor has a join program.

`tests/timeline.rs::a_successor_at_another_rate_carries_and_charges_nothing` is new. It uses Sol's
reproducer edit: a 44.1 kHz successor whose faders, pans and limiter lookahead change in a
prepared value and whose source changes length, so no owner the program counts can carry. The
line join still takes `eq0`'s limiter line. The test asserts:
- `test_only_carry_counts == [0; 5]` and `carry_program_retained_bytes == 0`;
- the hand-over is `PredecessorMismatch`.

Mutations, each run against `--test timeline`:
- Fresh-timeline rule off (lines charged, nothing installs them): red. The admission's
  retained-bytes assertion trips, `left: 0, right: 8`.
- Compensation lines kept (`delay_carry.clear()` deleted): red, the same assertion.
- Rule off plus attempt 1's late program: red, `left: [0, 0, 0, 0, 1], right: [0, 0, 0, 0, 0]`.

The existing test `a_successor_at_another_rate_or_quantum_starts_a_fresh_timeline` keeps its own
claim (a fresh timeline producer).

**2. Error precedence, recorded accurately** (corrected in the attempt-2 fold). These orderings
moved:
- `carry_builder`'s own validation (`NotAGraphPlan`, `CarryProgramInstalled`, `NoSourceSet`,
  `SourceIndexOutOfRange`, `SourceNotVacant`, `DuplicateSourceIndex`) ran at base right after
  bind (`prepare.rs` 4905). It now runs after every inventory read and join, the resource
  admissions (`host.graph.resource.limit`, `host.resource.limit`, `host.resource.arithmetic`) and
  `host.fade.install`.
- The input section's validation ran at base before the resource admission and before
  `host.fade.install`. It now runs after both.
- The strip section's validation (`install_builtin_strip_carry`) ran at base before
  `host.fade.install`. It now runs after it.
- The prime's quantum arithmetic (`host.resource.arithmetic`) now runs before every section's
  install, not after the route section.

The join only builds programs and sections these validations accept, so no host session reaches
a different code than at base. Outcomes changed only for hand-prepared successors with no join
program, that is, successors at another rate or quantum:
- One carrying only compensation lines, strip delays or sends got a source-less program and those
  sections at base. It now gets no program and no carry.
- One carrying only a fade pair, or only a prime, was refused `host.carry.install` at base. It now
  prepares, with no carry.

In both cases the base carry could never have run, because the envelope differs.

**3. The builder owns the plan.** `carry_builder(plan: PreparedRenderPlan, program) ->
Result<GraphCarryBuilder, GraphCarryRefusal>`, and `seal(self) -> PreparedRenderPlan` is
`#[must_use]`.
- A refusal hands the plan back (`GraphCarryRefusal::error`, `into_plan`); the plan is boxed so
  the `Err` stays small.
- A dropped builder drops its plan, so a forgotten seal leaves no plan to use.
- The builder reaches its executor through one downcast helper (`carry_executor`, an `expect`
  on the invariant `carry_builder` checked).
- Callers moved:
  - `prepare_or_project`: `plan = carry.seal()`;
  - the graph lib tests, `rt11_swap_carry_alloc`, `console_banking`, `silent_bank_skip`,
    `successor_swap` and the source tests: refusals now take the plan back from the refusal.
- Scratch compile errors against the head API (never committed):
```
section_without_a_program  E0425 cannot find function `install_builtin_input_carry` in this scope
forgotten_seal             E0382 use of moved value: `plan` (the builder took it)
render_mid_build           E0382 borrow of moved value: `plan`
second_builder_mid_build   E0382 use of moved value: `plan`
fade_table_mid_build       E0382 borrow of moved value: `plan`
section_after_seal         E0382 borrow of moved value: `carry` (moved by `seal`)
publish_mid_build          E0382 use of moved value: `plan`
builder_without_a_program  E0451 fields `plan`, `program` and `sections` of struct `GraphCarryBuilder` are private
sealed_plan_discarded      error: unused return value of `GraphCarryBuilder::seal` that must be used
```
In order:
- a section on a bare plan;
- a forgotten seal, then a render of the plan;
- a render mid-build;
- a second builder mid-build;
- a fade table mid-build;
- a section after the seal;
- publishing mid-build;
- a builder built by hand;
- a sealed plan discarded (`unused_must_use`, which the workspace denies).

The committed `compile_fail` doctest is now `E0382` (a render of the plan the builder took),
with its plain twin.

**4. `NoPrimeLead` split.** A prime on a plan with no source set is now refused `NoSourceSet`
(whose doc now covers sources, the timeline and a prime's fills). `NoPrimeLead` means only a
zero lead.

**5. Successor, filed by the coordinator.** "Graph: a bound plan is a builder until it is
sealed, so bind attaches, takes and the fade table are typed too". Its spec body is at the
coordinator's scratchpad `succ/l.md`.
- The bound plan returned by bind (`PreparedBuiltinsGraphArtifact::into_bound` /
  `into_bound_with_source_set` in `crates/builtins-compiler`) is not a `PreparedRenderPlan`.
- The three one-shot takes are handed out once, by value.
- The bind reads, the bind attach (`attach_jump_lengths`) and the fade table become its methods.
  The fade table install gives a token that the builder's `fade_arms` takes, which replaces
  today's run-time `StripLaneOutOfRange` ordering check.
- Only `seal` yields a `PreparedRenderPlan`, which retires `CarryProgramInstalled` and
  `FadeArmsInstalled`.
- Its scope includes the audit and console-workload bind callers.

**No bit moved (attempt 2).** The same scratch instrumentation as attempt 1, on the attempt-2 tree: the 360 rows (66 869
blocks, the two producer-thread tests excluded as before) are identical to the base listing,
sha256 `daf12f21e71fca57e3fbf4bdd02f9b3490de828bfaa9e727ac2695103fc0b1d5`. The merged `main`
commits touch only `hosts/host-web`, outside these suites. The render path and the executor's
fields are untouched; the fresh-timeline rule changes only plans that cannot adopt their
predecessor. `graph_fixture --check` passes.

**Gates (attempt 2 head).**
- `cargo test --locked -p graph --all-targets --features graph/test-support`: 150 passed.
- `-p graph-compiler --all-targets` (test-support features): 194 passed, 2 ignored.
- `-p source --all-targets --features graph/test-support`: 84 passed. `-p rack --all-targets`: 75.
- `-p host-core --all-targets` with the workspace test-support features: 472 passed, 8 ignored;
  with `host-core/control-provider`: 617 passed, 8 ignored (the new timeline test included).
- Doctests of the five crates: 23 passed (the `compile_fail,E0382` pair included).
- Clippy `-D warnings` on the five crates, with and without the features: ok.
  `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` (`host-core/control-provider`): ok.
- `cargo fmt --check`, `scripts/check-workspace-policy.sh`, `graph_fixture --check`: ok.
  `./target/release/realtime_policy`: ok (298 marked regions in 48 files).

**Verdict (attempt 2): PASS** (Sol, `f0048d467`), with five minors, folded below.

### Attempt 2 fold (Terra), same branch, `origin/main` @ the #2377 merge merged in

1. **The fresh-timeline rule clears `route_carry.retargets` too**, so no carried-send record is
   stamped into a successor that carries no send.
2. **The test reaches the clears that a fixture can reach.**
   `a_successor_at_another_rate_carries_and_charges_nothing` now runs on `bus_session`:
   - every track (`eq0` behind its limiter, and `added`) sends into a submix `bus`, which
     declares a 41/29-sample input delay;
   - the edit changes every strip's fader and pan or matrix, including `bus`'s.

   (A first version made the predecessor itself a successor that adds a send, to leave a route
   fade pending. Its entry binds no live route lanes, so no route fade armed, and that version
   was dropped.)

   Mutations against `--test timeline`, each deleting one clear:

| Clear deleted | Result |
|---|---|
| `delay_carry.clear()` | red: retained-bytes assertion, `left: 0, right: 8` |
| `strip_delay_carry.moves.clear()` | red: the same assertion, `left: 0, right: 12` (bus's alignment line) |
| rule off plus attempt 1's late program | red: `[0, 0, 0, 0, 1]` vs `[0; 5]` |
| `route_carry.moves`, `route_carry.retargets` | green: unreachable, see below |
| route-fade and copy `carried` | green: unreachable, see below |
| `strip_delay_carry.fills`, the `!fresh_timeline` sidechain gate | green: unreachable, see below |

   Why the green rows are unreachable from a fixture:
   - **Sends and route fades.** A send carries only between plans that bind live route lanes, and
     a route fade arms only on a plan that attaches live lanes. This fixture's entry binds none
     (`live_routes` is empty in both plans). Under live controls, every strip present in both
     models joins with a fader and a matrix family that is `Same` or `Live`, so it may carry and
     the join builds a program. A family is `Prepared` only for a value outside its setter's
     domain, which compile refuses, or for a strip whose kind changed. So a fresh-timeline
     successor with a carried send or route fade needs every shared strip to change kind, and
     that is not a fixture worth building.
   - **Fills and the sidechain gate.** They need a nonzero warm lead, and a `WarmLead` comes only
     from `warm_lead`.
   - **Why the clears stay.** Each is one line. Each keeps the rule "no join program, no carry,
     no charge" true by construction, and preparation's no-section assertion and the
     retained-bytes assertion would catch a stray section.
3. **Error precedence corrected** (attempt 2, item 2, above): the input and strip sections now
   validate after `host.fade.install`. The outcome changes against base are stated plainly
   there.
4. **The test's charge half is a debug-build claim.** Its red comes from preparation's internal
   `debug_assert_eq` on the retained carry bytes, and from the no-section `debug_assert`. No
   report field carries the admitted line, send or prime charge, so the test's doc says that this
   half holds in a debug build only, as the CI test jobs run. The install half (`[0; 5]` counts,
   zero retained bytes) holds in any build.
5. **Successor spec corrected:**
   - bind is `PreparedBuiltinsGraphArtifact::into_bound` / `into_bound_with_source_set` in
     `crates/builtins-compiler`, returning `PreparedBuiltinsGraphBound { pub plan }`, and that
     crate is in scope;
   - the audit and console-workload bind callers are in scope and gates;
   - the stale "builder's borrow" wording is gone: the fade table and its pairs are ordered only by
     the run-time `StripLaneOutOfRange` check, and `install_fade_arms` still works on a sealed
     plan;
   - `attach_jump_lengths` is called a bind attach.

**Gates (fold head).**
- `cargo test --locked -p host-core --all-targets` with the workspace test-support features and
  `host-core/control-provider`: 619 passed, 8 ignored.
- `-p graph --all-targets --features graph/test-support`: 150 passed. `-p graph-compiler`: 194
  passed, 2 ignored. `-p source`: 84 passed. Doctests of graph and host-core: 11 passed.
- Clippy `-D warnings` on graph, graph-compiler, host-core, source and rack, with and without
  the features: ok. `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` on the same crates: ok.
- `cargo fmt --check`, `scripts/check-workspace-policy.sh` and `graph_fixture --check`: ok.
- No bit moved: not rerun. The fold changes only the fresh-timeline branch of preparation (one
  more cleared table, on plans that cannot adopt) and a test, so no render path or adoptable
  plan changed.
