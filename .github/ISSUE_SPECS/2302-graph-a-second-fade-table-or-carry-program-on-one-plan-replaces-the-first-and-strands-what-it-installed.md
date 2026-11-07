# Graph: a second fade table or carry program on one plan replaces the first and strands what it installed

Tracker: #2258, item GR-6 (the re-install gaps only). Found by the implicit-state-machine audit of
2026-11-06.

## Problem
A successor graph plan is assembled after bind by a sequence of free functions in
`crates/graph/src/lib.rs` (`install_carry_program`, nine section installs, `install_fade_arms`,
`install_fade_arm_carry`). Two of them silently accepted a second call on the same plan:

- `install_fade_arms` (`lib.rs` ~4874, the executor method ~4886) documented "it replaces any
  earlier table". The first call already armed its lanes (`arm_fade_in`, `arm_route_fade_in`,
  `arm_route_fade_out`): a fader lane is muted from the plan's first block until its entry fires.
  A replacement table that omits that lane leaves it muted for a fire no table holds, and an
  installed `fade_arm_carry` keeps pairs whose `successor` indices name entries of the first table.
- `install_carry_program` (`lib.rs` ~5401) replaced `executor.carry` unconditionally. Every
  predecessor-dependent section (input, strip, effect, observer, observation, delay, strip-delay,
  route, prime and fade-arm carry) can only be installed after a program and was validated for
  that program's predecessor; a second program naming another predecessor left them in place, and
  the swap block would run them against the other plan.

## Reachability (verified at `b1fc70c0f`)
Not reachable from production today. Every production call of every `graph::install_*`,
`graph::take_*` function is in one function, `host-core::prepare::prepare_or_project`
(`crates/host-core/src/prepare.rs` 3303-5986), on `plan`, a local moved out of the `bound` plan the
same function binds at ~4836. No function takes an existing plan and installs on it, and no other
crate (control-plane included) calls an install. So:
- `install_fade_arms` runs once (~5575), inside `if !fade_arms.is_empty() || ...`.
- `install_carry_program` runs at most once: at ~4905 when the join built a program, else at
  ~5682 under `if !has_carry_program`.
- A republished candidate (the control plane's withdraw-and-republish, `control.rs` ~5412) is the
  same plan republished whole, with no install; a retried rebuild re-runs preparation, which binds
  a fresh plan; a withdrawn successor is only a ring donor (`host-core/src/source.rs` ~1071, "the
  donor is never republished").
- No test installs a second program or table successfully either (the only second calls,
  `graph/src/lib.rs` ~11773 and `source/src/lib.rs` ~6340-6364, follow a refused first call).

The functions are public and called directly by tests in `graph`, `graph-compiler`, `host-core` and
`source`, and the failure is silent (a strip muted for the plan's life, or a mis-carried swap), so
this hardens the one-shot rule rather than leaving it to the caller (the #2257 precedent).

## Contract
- A plan holds one carry program for its life: `install_carry_program` on a plan that holds one is
  refused with `GraphCarryInstallError::CarryProgramInstalled`, leaving the plan as it was.
- A plan holds one fade table for its life: `install_fade_arms` on a plan whose table is not empty
  is refused with `GraphCarryInstallError::FadeArmsInstalled`, arming nothing. An install that
  armed nothing holds no table, so a later install is accepted (nothing can be stranded).
- With both refusals, no fade-arm carry can index a table it was not built for, and no section can
  outlive the program it was validated for.
- No render code changes, and no production or existing test call takes either refusal.

## Non-goals
The full `GraphCarryBuilder` typestate the tracker suggests (bind, take, program, sections, fade
arms, seal) is a follow-up: it restructures `GraphExecutor`'s carry fields, the sixteen install and
take functions and `prepare_or_project`'s interleaved install sequence, and every direct test caller
in four crates, and `host-core/src/prepare.rs` is shared with open branches. Re-installing a
section (same program, so same predecessor) keeps its documented replace semantics. Ordering of
sections relative to the program stays enforced by the existing `No*CarryProgram` refusals.

## Gates
- Red-on-base regression tests for both refusals.
- A one-time no-bit-moved comparison against base on the successor-swap suite, as PR evidence.
- `cargo test --locked -p graph` (with `graph/test-support`) and the host-core successor-swap
  suite, clippy `-D warnings`, `cargo fmt --check`, `cargo doc -D warnings`,
  `scripts/check-workspace-policy.sh`, the realtime policy tool.

## Evidence (attempt 1)

Base: `main` @ `b1fc70c0f`.

**Tests.**
- `graph::runtime::tests::a_second_fade_table_is_refused_and_every_first_arm_still_fires`: installs
  a table arming two fader lanes (FadeProbe fixture, no source, `D = 0`), then a second table naming
  only one; renders two blocks and asserts both lanes fired at sample 0, both watch entries fired,
  and the second install was refused `FadeArmsInstalled`. Defect it defends: a second install that
  replaces the table, leaving a lane the first armed muted with no entry to fire it. No existing
  test makes a second successful install.
- `graph::tests::install_refuses_a_second_carry_program`: installs a program for predecessor 1,
  then one for predecessor 2; asserts the plan still names predecessor 1 and the second install was
  refused `CarryProgramInstalled`. Defect it defends: a second program re-targeting sections
  validated for the first predecessor. No existing test makes a second successful install.

**Red on base** (tests added, final `assert_eq!(second, Err(..))` lines stubbed so the base
compiles; the behavioural asserts run first):
```
test tests::install_refuses_a_second_carry_program ... FAILED
assertion `left == right` failed: the first program stays
  left: Some(2)
 right: Some(1)
test runtime::tests::a_second_fade_table_is_refused_and_every_first_arm_still_fires ... FAILED
assertion `left == right` failed: every lane the first install armed fires
  left: [(1, 0)]
 right: [(0, 0), (1, 0)]
```
At head both pass.

**Mutations (head, whole `graph` lib suite each).**
- The `fade_arms.is_empty()` refusal deleted: red, `a_second_fade_table_is_refused_and_every_first_arm_still_fires`
  only (138 passed, 1 failed).
- The `carry.is_some()` refusal deleted: red, `install_refuses_a_second_carry_program` only
  (138 passed, 1 failed).

**No bit moved.** The render path is untouched (both changes are control-thread refusals at the
top of install functions no existing path calls twice). Checked anyway: a scratch instrumentation
of `crates/host-core/tests/successor_swap.rs`, never committed, FNV-1a-hashed every rendered block
of both render helpers (`render_block`, `render_into`) and printed it with the test name; the suite
ran single-threaded with `--features builtins-compiler/test-support,graph/test-support,host-core/test-support,engine/test-support,engine/realtime-audit,effect-compiler/test-support,protocol/test-support,session/test-support`.
Base and head: 126 passed, 1 ignored; 15976 block hashes each, identical line for line
(sha256 of the listing `e6947fa0dcf4017e0b2ed639c04a8f5d1b6f68908c1c24671be86dc4653378c8` on both).

**Gates run (head).**
- `cargo test --locked -p graph -p graph-compiler -p source --features graph/test-support,builtins-compiler/test-support`: ok (graph lib 139 passed).
- `cargo test --locked -p host-core` with the features above: `--test successor_swap` (126 passed,
  1 ignored), `swap_block_realtime`, `browser_successor`, `live_routes`: ok; plus
  `host-core/control-provider`, `--test warm_successor` (22) and `--test withdrawn_successor` (9): ok.
- `cargo clippy --locked -p graph --all-targets --features graph/test-support -- -D warnings`: ok.
- `cargo fmt --check`: ok. `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p graph`: ok.
- `bash scripts/check-workspace-policy.sh`: ok. `./target/release/realtime_policy`: ok (292 marked regions in 46 files).

**Follow-up (not in this issue).** The `GraphCarryBuilder` typestate: `graph::carry_builder(&mut
plan, program) -> GraphCarryBuilder<'_>` takes the plan's program once; its section methods
(inputs, strips, effects, observers, observations, delays, strip delays, routes, prime, fade arms
returning their entries, fade-arm carry taking those entries) consume or borrow the builder, and
`seal()` writes one immutable `GraphCarry` into the executor, so a section without a program, a
second program and a fade carry built for another table become unrepresentable rather than refused.
It needs `prepare_or_project`'s install sequence regrouped so no other plan borrow falls between
the builder's start and its seal, and the direct test callers in `graph`, `graph-compiler`,
`host-core` and `source` moved onto it.
The coordinator tracks this follow-up in #2258, where GR-6 is marked partly covered by #2302.

**Verdict (attempt 1): PASS.** Adversarial review of `daabdab98`:
- The split is justified: the two refusals are a strict subset of what the builder will
  guarantee, so the builder supersedes them without reversing them.
- The empty-first-table case is sound: an install that armed nothing holds no table, so accepting
  a later install strands nothing.
- Adding the two `GraphCarryInstallError` variants changes no exhaustive match elsewhere.
- The reachability proof is complete.
