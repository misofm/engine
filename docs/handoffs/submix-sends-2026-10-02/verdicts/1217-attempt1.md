# #1217 *Skip an inactive route in its destination's sum*: Sol verdict, attempt 1

- Reviewed: `git diff f48fe7c74 071c6c14a` (10 files, +1371/-134). Branch `codex/batch-submix-k3`,
  worktree `/home/bl/misofm/wt-submix-k3`.
- Binding: `AGENTS.md`; `.github/ISSUE_SPECS/1217-skip-an-inactive-route-in-its-destinations-sum.md`
  with its Attempt 1 record; DESIGN P4, 5.4 and 5.7; VERIFY-1 BLOCKER-1 and MAJOR-1; REVISION-1;
  VERIFY-2 MINOR 1 and MINOR 5; the owner preference `skip-work-on-silence.md`.
- How I ran it:
  - I did not modify the worktree, the branch or GitHub. `gh issue view 1217` is OPEN, and its title
    matches the spec's H1.
  - I exported head `071c6c14a` and parent `f48fe7c74` with `git archive` under
    `/tmp/claude-1002/v1217/`, each with its own target directory.
  - I ran mutations and probes in a third copy, so the gate tree stayed pristine.
  - My probe tests are saved beside this file as `1217-attempt1-verifier-scratch.rs`.
  - All scratch under `/tmp/claude-1002/v1217/` was deleted afterwards.

## Verdict: FAIL

There is **one BLOCKER**, and it is not a semantic defect: **the commit turns the required
`qualification` check red.**

- `scripts/check-web-audioworklet.sh` passes on the parent and fails on head. Its kernel-shape rule
  rejects eight `reduce_group_into<f32x4, N>` instantiations that the change outlined into the wasm
  module.
- A one-attribute fix makes it green again. I verified that fix (BLOCKER-1).

**There is no MAJOR.** The semantics are right, and I checked them adversarially beyond the spec's
gates:

- The activity rule (D1, D2) holds for every route shape I built: `post_pan`, mid-chain taps, bus to
  bus, routes into the output, routes from banked strips, and a `SumDelay` bus.
- The store-owner rule (D3) holds, including first-inactive, all-inactive and in place, in both the
  arena form and the host-master form.
- A muted delayed route stays active. I observed its zero-coefficient mix passing through its
  486-sample line directly in the rendered audio (a NaN probe and a signed-zero probe), not only
  through the counter.
- Every gate the spec lists is green and reproduces the record's numbers.

There are two MINOR findings, both about test strength, and three NITs. Attempt 2 needs only the
BLOCKER fix and the evidence that it is green. I recommend applying the MINORs as well, because
each one is a few lines and closes a hole I demonstrated.

## Gates (head `071c6c14a`, x86-64-v3 AVX2)

| Gate | Result |
|---|---|
| 6: `cargo build --locked --release -p audit -p bench -p capi -p session-validator` | rc 0 |
| 6: `check-graph-determinism.sh` | PASS (100/100) on head and on parent. `fresh-process-determinism.json` (331 B) is **`cmp`-identical** between them |
| 6: `graph_fixture -- --check` | rc 0 |
| 6: `check-console-fixtures.sh target/release/session_validator` | rc 0 |
| 6: `check-builtins-fixtures.sh . target/release/audit` | rc 0 |
| 6: `./target/release/audit capi` | rc 0: `allocations 0`, `deallocations 0`, `locks 0`, `syscalls 0`, `total_violations 0`, digest `ff6cdcb96cdcdad5` (unchanged) |
| 6: `cargo test --locked --release -p audit -p bench -p console-workload` | **110 passed, 0 failed**, including `every_standing_workload_folds_one_route_per_track` |
| 7: test-debug-a (exact DESIGN 7 command, `--no-fail-fast`) | rc 0: **1186 passed, 0 failed, 9 ignored, 106 binaries**, as recorded |
| 7: `check-`/`test-graph-policy.sh`, `check-`/`test-realtime-policy.sh`, `check-workspace-policy.sh` | all rc 0 |
| 7: `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | both rc 0. Clippy without features on `graph`, `graph-compiler`, `host-core` and `capi` is also rc 0, so the `test-support`-gated counter leaves no unused binding in a shipped build |
| 7: `run-aarch64-tests.sh debug` | **not run**: this host is x86 and has no aarch64 runner. It is deferred to CI `aarch64-debug` at the K3 push, which the spec permits |
| Extra: `check-cross-targets.sh` | PASS. iOS and Android aarch64 are checked and linted, and the wasm simd128 4-lane build compiles |
| Extra: **`check-web-audioworklet.sh`** (CI `qualification.yml:342`) | **FAIL on head, PASS on parent `f48fe7c74`.** See BLOCKER-1 |

## What I verified, and how

- **D1/D2, the activity rule.** `route_activity` marks a route inactive only when
  `gate.silences() && input.delay.is_none()`.
  - `InputRef.buffer` is the producer's buffer even when the input is delayed; the staging buffer is
    `DelayRef.staging`. So the last-writer walk attributes a delayed input to its route correctly.
  - Every route node has exactly one `RouteDestination` edge (`compile.rs` route lowering), and
    sidechain sources are strip taps, never route nodes. "One consumer" therefore holds, and the
    `inputs <= R` and `destinations <= R` bounds hold too.
  - `active` is written only at bind (`route_activity`) and never at render, so activity is fixed
    for the whole block.
  - "The block in which a ramp ends is mixed whole" and "decided at block start" cannot be exercised
    here: there are no live routes until #1220. The table is `&mut`, plain data and has no trait
    object, as D1 and D5 require.
- **D3, the sums.** Since slots retire one op late (`program.rs` pass 2), a multi-input op's output
  never aliases its inputs. So `Zero` followed by `Add` can never clobber an input.
  - The `Add` runs keep the one left-to-right chain, because `accumulate_run` reloads the running
    sum.
  - With every input active, the walk yields one `Store(0, count)`, which is today's reduction.
  - In my probes, every arrangement matches the D3 oracle or the session without the muted routes,
    bit for bit. The arrangements were first inactive, middle inactive, every input inactive
    (n = 1, 2, 3, 9) and in place, into a bus, into the output and into a `SumDelay` bus.
- **Fold interplay.**
  - A gated route declines `plain_route_gains`. The association proof then declines any master with
    a gated contributor.
  - Skipping `master_op` in `route_activity` is therefore sound: all of its contributors are
    retired, open routes.
- **Render.**
  - The gate 5 test is green (armed audit plus thread counters).
  - `audit capi` is clean.
  - `route_segments` takes a closure sink and allocates nothing.
- **The estimate (D6).**
  - Gate 4 measures 545 bytes charged against 237 retained.
  - 237 is exactly `64 + 9 + 21*4 + 1*8 + 9*8` (9 routes, 21 units, one destination, 9 inputs).
  - The `nodes` term bounds the units, because units <= ops <= spec nodes. Dropping that term would
    charge 217 < 237, so the gate is tight enough to see a lost term.
- **The test-only counters** are `#[cfg(any(test, feature = "test-support"))]` on both the statics
  and the call site. They are fixed-size thread-locals, so they allocate nothing.
  - Tools enabling `graph/test-support` (audit, console-workload) is a pre-existing pattern, not new
    here.
- **Stale replay across a later unmute.** In this slice an unmute is a session edit, which means a
  new plan.
  - `RenderSession`/`PreparedRenderPlan` carry no compensation-line state across a replacement, so a
    prepared unmute cannot replay anything.
  - The live hazard belongs to #1220. What this slice owns, "the line holds the zero mix, never
    unwritten arena words", is proven by my NaN and signed-zero probes (mutation M1 below turns
    both red on audio).
- **The deviations are acceptable.**
  - **Deviation 1**, `Option<(&mut RouteActivity, usize)>`: forced, since a `RuntimeOp` has no unit
    index and D5 forbids adding one. It still freezes the `&mut` borrow for #1220.
  - **Deviation 2**, the `debug_assert!` at bank dispatch: stronger than the brief, because it covers
    every member.
  - **Deviation 3**, the #1216 gate 5 test edited outside the authorized paths: forced, because D6
    charges the table in the sealed `estimate` row. See NIT-1 for a tighter form.
  - **Deleting #1216 gate 1** is correct under AGENTS.md: #1217 gate 1 supersedes it.

## Mutations (mine; each applied alone to `crates/graph/src/runtime.rs`, run, reverted)

The suite is `route_mute` (the 7 committed tests plus my 6 probes). For M4, a second run also used
sentinel-filled host planes.

| # | Mutation | Committed tests | My probes |
|---|---|---|---|
| M1 | A delayed muted route goes inactive (`&& input.delay.is_none()` dropped) | RED, **counter only**: `a_muted_delayed_route_stays_active` (0 against 8) | RED **on audio**: the NaN-through-the-line probe (no NaN at f+486) and the signed-zero probe (`plane 0 frame 486`: `+0.0`, expected `-0.0`); shape probes red |
| M2 | The destination ignores activity (adds every input) while the inactive route op is skipped | RED: gate 1 (`seed 2, Bus, r1 muted ... 0.0 != -0.0`), gate 1 in place (raw tap `-0.15693776`), gate 2 | RED |
| M3 | Skip the destination's `+0.0` fill when every contributor is inactive (a plausible silence shortcut) | RED, 1 of 7: the in-place test (raw tap) | My n >= 2 all-muted probe stayed green: the unwritten bus slot happened to hold zeros |
| M4 | Host-master form: `SumSegment::Zero => {}` (no `+0.0` store into the host planes) | **GREEN, all 7** | **RED once the host planes are pre-filled with `7.0`**: `seed 0, Output, r0 muted, plane 0: sample 0: 7.0 != 0.0`. See MINOR-1 |
| M7 | Arena form: a later active run stores instead of adding | RED: gate 1 and gate 2 | RED |
| M8 | Activity read by input position instead of route index | RED: gates 1 and 2, in place | RED |
| 1217-2 (re-run) | The inactive route op's early return is removed | RED (mix counters), as recorded | RED |

## Test value (one sentence each)

- `an_inactive_route_is_neither_mixed_nor_read`: red if a muted undelayed route is still mixed or
  added, or if the arena form moves the store owner (M2, M7, M8, 1217-2). It does **not** catch a
  host-form store that is never written (M4, MINOR-1).
- `an_inactive_in_place_sole_route_fills_its_bus_with_positive_zero`: red if an inactive sole input
  leaves the raw tap in its destination. Nothing else catches it (M3, 1217-4).
- `the_first_route_in_id_order_owns_the_store`: red if the store moves to the first active input, or
  an active first input's `-0.0` is lost.
- `a_muted_delayed_route_stays_active`: red if a delayed muted route goes inactive, but only through
  the mix counter. Its bit comparison cannot see the line's contents (MINOR-2).
- `a_route_activity_table_is_built_only_for_a_silencing_gate_and_is_charged`: red if the table is
  built unconditionally or is not charged (1217-9, 1217-10).
- `route_activity_renders_without_allocating`: red if the table or the route inputs are built or
  resized at render (1217-8).

## Findings

### BLOCKER-1: the change makes CI's required web kernel-shape gate fail

**Evidence.** `bash scripts/check-web-audioworklet.sh` exits 0 on parent `f48fe7c74`. On head
`071c6c14a` it exits 1 with eight lines like:

```
FAIL kernel _RINv...graph7runtime17reduce_group_intoNt...4wide6f32x4_5f32x4Kj1_EB4_: vector=1 scalar=3 (a vector instantiation must use strictly more vector than scalar arithmetic)
... Kj2_ vector=3 scalar=9 ... Kj8_ vector=15 scalar=45
```

**Cause.**
- `reduce_group_into<L, N>` is `#[inline]`. Before this change its only caller was
  `reduce_many_into`, and LLVM inlined it into the non-generic `reduce_plane_into`.
- Now `reduce_run_into` also reaches it from `reduce_gated_into`'s `Add` arm. With two callers,
  LLVM emits it as a standalone `f32x4` instantiation, and that instantiation carries the inlined
  `accumulate_run::<f32, N>` scalar tail.
- The tail never runs at a quantum that is a multiple of 4, but the static rule (the #926 lesson
  DESIGN 5.7 cites) counts it.
- `qualification.yml:342` runs this script, and it is the only required check on `main`, so the K3
  batch push would go red.

**Fix (verified).** Change `reduce_group_into`'s attribute from `#[inline]` to `#[inline(always)]`
(`runtime.rs:644`). This restores the parent's inlined shape, and `check-web-audioworklet.sh` then
exits 0, with the boot-budget and static/object checks green as well.

- The alternative is the #926 form: outline the `f32` tail as a non-generic `#[inline(never)]`
  function. It is a larger change and not needed here.
- Attempt 2 should run `check-web-audioworklet.sh` (and `check-cross-targets.sh`, green today) and
  record both in the evidence. Every later K3 slice that touches the reductions (#1220 especially)
  should too.

### MINOR-1: the host-master `+0.0` store is untested, because the harness zero-fills the host planes

**Evidence.**
- `route_mute.rs`'s `render()` hands the engine `[0.0_f32; QUANTUM * 2]` every block, and gate 5
  does the same with `pcm`.
- The engine never pre-clears the host planes: a successful block "never writes a plane twice"
  (`graph/src/lib.rs` render docs), and `PreparedRenderPlan::render_inner` zero-fills only when
  there is no executor.
- So in production, an Output op whose first route is inactive must store `+0.0` itself, and a host
  output buffer that is reused, or holds garbage, depends on it.
- Mutation M4 (`SumSegment::Zero => {}` in `reduce_gated_into`) is green across the whole committed
  suite. The spec's "the same rules hold for the host-master forms" is therefore defended only for
  the `Add` arm (1217-6), not for the store.

**Fix.**
- Pre-fill `samples` in `render()` and `pcm` in `assert_renders_without_allocating` with a
  non-silent sentinel such as `7.0`. I ran the whole file that way: the tree stays green and M4
  turns `an_inactive_route_is_neither_mixed_nor_read` red.
- Optionally add gate 1's "every contributor muted" Output case with two or more routes. That is the
  product-outcome sentence "a bus whose every contributor is muted renders exact `+0.0`", which today
  is pinned only for the one-contributor in-place bus. My probe
  `adv_every_contributor_muted_is_positive_zero` is a ready form.

### MINOR-2: gate 3's audio comparison cannot see the delay line; only the counter defends D2

**Evidence.**
- The oracle is `s` wherever `s` is nonzero, and `+0.0` in the first 486 frames. Adding a zero mix
  to a nonzero sample is exact, so the bit equality holds whether `d-e` is mixed through its line or
  skipped.
- Under M1 (`1217-7`), only `mixes[d-e] == 8` goes red.
- The spec asks the oracle to "mix `d`'s route with `[+0.0; 4]` through the 486-sample delay". As
  committed, the audio half of the gate has no D2 discrimination.

**Fix.** Add the signed-zero form, which I ran green on the tree and red under M1 on audio:

- Mute `s-e` as well. It is undelayed (it is the late arrival), so it is inactive.
- Feed `d` strictly negative samples on both lanes.

`e`'s input is then exactly `d`'s delayed zero mix, and the output is `+0.0` for frames `< 486` and
`-0.0` from frame 486 on, on both planes. This one assertion pins:

- the delay length;
- that the line holds the zero-coefficient mix rather than unwritten arena words;
- D3's store owner.

It is `adv_muted_delayed_route_zero_sign_follows_the_line` in the scratch file. Feeding `+inf` into
`d` and asserting NaN at exactly `f + 486` (`adv_muted_delayed_route_carries_its_zero_mix_through_the_line`)
is an equivalent alternative, and it also exercises DESIGN 5.4's NaN note.

### NIT-1: the edited #1216 gate 5 test sets aside the whole `estimate` row

It could instead assert that the two rows differ only in `graph_metadata_bytes`,
`incremental_plan_bytes` and `session_plus_plan_bytes` (plus `largest_allocation_bytes` if it
moves), each by exactly `graph::route_activity_bound_bytes(R, N)`. That would keep #1216's "mute
moves nothing else" claim over the estimate row too.

### NIT-2: the tag bit can collide with a large route index

`ROUTE_DESTINATION = 1 << 31` tags `units` entries, but `index()` only checks that a value fits in
`u32`. A route or destination index of `2^31` or more would be misread. This is unreachable under
any realistic cap. A `debug_assert!(value < ROUTE_DESTINATION as usize)` in `index` would make the
invariant explicit.

### NIT-3: the attempt record does not list the web and cross-target gates

The Attempt 1 gate list did not include `check-web-audioworklet.sh`, which is how BLOCKER-1 got
through. Add it, and `check-cross-targets.sh`, to the slice's evidence for attempt 2.

## Not verified here

- `run-aarch64-tests.sh debug`, the 4-lane native run. This host is x86 and has no arm64 runner or
  qemu. It is deferred to CI `aarch64-debug` at the K3 push. The wasm simd128 4-lane build compiles,
  and the cross-target matrix (iOS and Android aarch64 check plus clippy) passes.
- The V8 spill check (`check-web-audioworklet-v8-spill.py`) needs the pinned Node. It inspects the
  parametric EQ's loops, which this change does not touch.
