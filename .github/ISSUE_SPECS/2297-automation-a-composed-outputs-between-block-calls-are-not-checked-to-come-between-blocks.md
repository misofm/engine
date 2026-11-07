# Automation: a composed output's between-block calls are not checked to come between blocks

Tracker: #2258, item PR-11. Found by the implicit-state-machine audit of 2026-11-06.

## Problem
`Composed<T>` (`crates/automation/src/compose.rs`) walks a block in two halves: `start` pulls each
cursor's first event, then `next_event` merges until the block ends. Between them, each
`ComposeCursor` keeps the walk's scratch: the pulled event (`next`), the state its pull began from
(`before`, `resume`), `shown`, `consumed` and `moved`. That scratch means something only inside a
block. A `block: Option<(NodeSpan, SeekMode)>` word on the output marked a block in progress.

`retarget`, `defer_jump` and `clear_deferred_jump` said "call between blocks", but nothing checked
it. Seeds, carries, `cursors_mut` and `cursor_mut` were in the same position. Any of them, called
between `start` and the block's last `next_event`, writes cursor states that the scratch does not
see. A cursor that already pulled ahead keeps an event computed without the write, and a later
replay restores `before`, which drops the write.

## Contract
- Find every caller of the between-block methods (graph, builtins-compiler, rack and builtins
  included). Record whether any can call one mid-block. If one can, a regression test reaches it,
  red on main.
- Model a block as a `BlockWalk<'_, T>` borrow. `Composed::start` returns it; `next_event` and
  `finish` exist only on it. It holds the output's only borrow, so no between-block call compiles
  while a block is walked. The output's `block` word goes away.
- A `compile_fail` doctest pins the refusal, beside a plain twin that differs only in where the call
  stands.

## Non-goals
PR-12 (`CellState` flags and sentinels). No rendered bit, event or state word may move. No
allocation, lock or syscall is added to render.

## Gates
- A red-on-main regression test, or a recorded proof that no caller makes a mid-block call, plus the
  compile-time refusal.
- A one-time no-bit-moved comparison against base, as PR evidence.
- `cargo test --locked` for automation, graph, builtins-compiler and host-core; clippy
  `-D warnings`; `cargo fmt --check`; rustdoc `-D warnings`; `scripts/check-workspace-policy.sh`;
  the realtime policy tool.

## Evidence (attempt 1)

Base: `main` @ `875a4c17e`.

**Reachability: no caller makes a mid-block call today.** Only two crates use `Composed`: graph
(route columns) and builtins-compiler (composed fader and mute channels). Rack, builtins, host-core
and the tools do not call it. On base, every `start` is followed by `next_event` until `None`
before any other call on that output:
- *graph* (`runtime.rs`, `walk_columns`): it starts both columns and drains them in one loop. The
  `change` callback receives only events, since the walks are borrowed by `walk_columns`. The two
  `retarget` calls (`apply_cell`, `place_carried`) and every seed, cursor write and `copy_from` run
  in `begin_block`, `apply_cell` and `place_carried`, outside any walk.
- *builtins-compiler* (`FaderEventDriver`): it never calls `Composed::retarget`. `defer_jump`
  (`ComposedAdoption`, through `settle_adoption`) and `clear_deferred_jump` (`release_at_entry`,
  `offsets_jump`) run at the block's entry, before the channel's walk starts. Within a block, a fader
  channel's walk is restarted at each piece (`open`, `cut`). Each restart is preceded only by
  `each_state` (cursor states), and the previous piece is always spent at that point. `cut` runs at
  `E == last`, after the event at `E` was pulled. `open` runs at the piece's last offset, after that
  offset's pull (in `apply`'s fader loop or its mute loop) or once the block is spent. A piece's walk
  ends with its last offset, so `next` is `None` and the walk has ended. `finish` drains each walk
  before opening the next piece. A mute walk is started once per block and only pulled.

So no test can be red on main for a reachable mid-block call. The misuse compiles on base and is
wrong. A scratch test (base API, never committed) retargets one value cursor between `start` and the
first `next_event`, then compares with the same retarget made before `start`:

```
between blocks: [(0, true), (64, true), (128, true), (192, true), (200, false)]
mid block:      [(0, false), (64, true), (128, true), (192, true), (200, false)]
```

The event the cursor had already pulled at offset 0 escapes the deferral and retargets the output,
cutting the column jump short (R19-3).

**Model** (as folded after the attempt-1 verdict, below).
- `Composed::start(&'a mut self, &'a Program, span, mode) -> BlockWalk<'a, T>`. The walk holds the
  output's borrow, the program, the span and mode, and an `ended` flag.
- Three methods live only on the walk:
  - `next_event(compose)` gives the block's events.
  - `finish(&mut self, compose, between)` drains the block, then hands the output to a
    between-block closure.
  - `restart(&mut self, compose, span, mode)` drains the block and begins the next block or piece
    on the same output and program.
- `Drop` covers a walk abandoned before its end: each cursor whose event was pulled but not decided
  returns to the state its pull started from (`before`). The walk therefore stops at its last
  decided offset and loses no event, and a between-block call made after the drop sees that state.
- The `block` word is gone, and `copy_from` no longer copies it.
- `walk` is `start` followed by `next_event` until `None`.
- The per-cursor scratch stays in the preallocated table, so no allocation is added, but only a
  `BlockWalk` can reach it.

Callers:
- *graph*: `walk_columns` holds two `Option<(BlockWalk, roles)>`.
- *builtins-compiler*: `MuteEvents::Composed` holds the mute walk. `ChannelEvents::Composed` holds
  a `FaderComposition` (the `own_curve`/`own_db` pair, fixed through a block) and the walk. A piece
  boundary with a pending release is `finish(.., each_cursor_state(release))` then `restart`; a
  piece boundary without one is `restart` alone. A cut's one-sample landing is `restart` followed by
  a drain. A spent block's release is `finish`.
- `FaderComposite::next_event` is gone. `cursor_history_valid = true` is set once, where the
  channel's walk starts. The first pull follows at once, and nothing reads the flag while the driver
  holds the lane.
- `crates/builtins` and `crates/effect-contract` are untouched.

**Test.** One `compile_fail,E0499` doctest on `BlockWalk`, with its plain twin. The twin starts a
block, drains it, then calls `retarget`. The `compile_fail` copy is the same text with the
`retarget` line moved above the drain. rustdoc checks the `E0499` code only on nightly; on stable
any compile error satisfies the fence. Test value: it turns red if a change lets the output be
reached while a block walk lives, for example a walk that holds a pointer instead of the borrow,
or a `start` that takes `&self`. The plain twin turns red if the shared code stops compiling for
any other reason, so the `compile_fail` cannot pass vacuously. No runtime test is made useless. The
attempt-1 fold adds one runtime test (below).

**Mutations (head).**
- `BlockWalk` holds a `NonNull<Composed<T>>` instead of the borrow, and `start` takes `&mut self`
  for the call only. The `compile_fail` doctest is red: the mid-block `retarget` compiles. The plain
  twin stays green.
- `cursor_history_valid = true` deleted where the fader walk starts. Nothing is red in
  `builtins-compiler` or `host-core` (525 tests). The line it replaces, in the deleted
  `FaderComposite::next_event`, set the same flag at the same point, so this is a coverage gap
  inherited from base, not one this change opens. The flag's re-arm after an invalidating
  adoption has no test that reaches it. Listed as a follow-up.

**No bit moved.** This used a one-time scratch instrumentation of `compose.rs`, never committed.
It was applied the same way on base and on head, at the same two points of the walk: each emitted
event, and the block's end. It folded (FNV-1a, through an allocation-free `fmt::Write` sink)
everything that could move:
- every emitted `ComposedEvent` (`Debug`, `T` included);
- the output's `CellState`;
- every cursor's `CellState`, heard bits, `heard_at` and `target_at`, after each event.

At each block's end it printed one line. The run was `cargo test --no-fail-fast -p automation -p graph
-p builtins-compiler -p host-core --all-targets --features
builtins-compiler/test-support,graph/test-support,host-core/test-support -- --nocapture
--test-threads=1`. `--nocapture` keeps the harness's capture buffer from allocating, so the
allocation tests stay green. It covered the existing fixtures plus a scratch seeded generator
(`zz_pr11_hash.rs`, never committed). That generator drew 300 outputs of 1 to 4 value and gate
cursors, with `with_held_grid` and `with_jump_release` chosen at random. Each was walked over 80
blocks of random quantum, with seeks in both modes. Before each block it made random between-block
calls: `retarget`, `defer_jump`, `clear_deferred_jump`, a cursor's `hold_until`, a re-seed and
per-cursor jump lengths. It hashed every event and post-block state with SHA-256.

Per test binary, the walk counts and the SHA-256 of the line sequence are identical on base and
head (ordered and sorted):

| binary | block walks | ordered digest |
|---|---|---|
| automation (lib) | 67,380 | `0bca4345119e2acf` |
| automation `allocation` | 10,001 | `c8fdfa36c03c2ba3` |
| scratch seeded generator | 24,000 | `1a4f47397a614e9f` |
| builtins-compiler (lib) | 16,108 | `3ad94032f4befd4d` |
| graph (lib) | 3,557 | `f2dabdce97a6ab5d` |
| host-core `route_mute` | 3,954 | `633120a2335e3103` |
| host-core `stored_fader_automation` | 1,616 | `de0bb67dfc01b35e` |
| host-core `stored_vca_automation` | 11,610 | `3724575fa15b5027` |
| host-core `successor_swap` | 272 | `44ae39351880314e` |

That is 138,498 composed block walks in all. Graph columns, composed fader pieces, one-sample cut
landings and composed mutes are all included. The seeded generator's own digest is identical on
both: 18,867 events, `a31771ffa592af9fc2a930a1d5dfb15df87e7fcb508eeb8cee4051fbb13939e4`. All 719
tests of that run passed on both.

**Gates (head).**
- `cargo test --locked --no-fail-fast -p automation -p graph -p builtins-compiler -p host-core
  --all-targets --features builtins-compiler/test-support,graph/test-support,host-core/test-support`:
  718 passed, 0 failed.
- `cargo test --locked --doc` on the same packages and features: all pass, including both
  `BlockWalk` doctests.
- `cargo clippy --locked` on the same packages and features `--all-targets -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps -p automation -p graph -p builtins-compiler`:
  clean. host-core fails on a broken intra-doc link that is already on base
  (`crate::warm::production_config`, `prepare.rs:262`); this change does not touch host-core.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (292 marked regions in 46
  files)`. `BlockWalk` and its methods sit inside the existing marked region. The walk is a
  stack value holding two references, a span, a mode and a flag, so it adds no allocation, lock or
  call.

**Verdict (attempt 1): PASS, with these folded.**
- The adversarial review confirmed four points:
  - the builtins-compiler restructure is equivalent;
  - the doctest runs in CI (the workspace doctest step);
  - `cursor_history_valid` is set at an equivalent point;
  - realtime is clean, and the branch merges cleanly with `main`.
- It found the abandoned-walk hole, the same scratch state this issue is about. A `BlockWalk`
  dropped before its end lost the event it had pulled but not decided: the cursor's state had
  advanced past it, and the next `start` cleared `next`. `drop(output.start(..))` followed by a
  second walk of the same span emitted `[(64, ..), ..]` instead of `[(0, ..), ..]`. Folded:
  - **Fix.** The restore went into `Drop for BlockWalk`, not into `Composed::start` as the
    reviewer suggested. A cursor whose `next` is `Some` and not consumed returns to `before`, and
    `next` is cleared. A restore deferred to the next `start` would overwrite any between-block
    call made after the drop (a `defer_jump`, say) with the stale `before`. A walk at its end has
    no pulled event, so `Drop` does nothing for it.
  - **API.** `finish` takes `&mut self` and a between-block closure instead of moving the borrow
    out, because a type with `Drop` cannot move a field out. `restart` begins the next block or
    piece on the same walk. builtins-compiler's composed fader channel now holds the walk itself
    rather than an `Option`, and no longer needs its own `program` field.
  - **Test.** `compose::tests::a_dropped_block_walk_loses_no_pulled_event`. Case 1: a walk
    dropped before its first event, then the same span walked again, must equal a fresh output's
    events, cursor states and output state. Case 2: the same, with a `defer_jump(200)` made between
    the drop and the walk.
    - Red at `59ae78d70` (case 1): `left: ([(64, .., Grid, false), (128, ..), (192, ..)], ..)`
      against `right: ([(0, .., Grid, false), (64, ..), (128, ..), (192, ..)], ..)`.
    - Test value: red if a dropped walk leaves a pulled-ahead cursor past its undecided event, or
      if the restore waits for the next `start` (it would then overwrite the deferral). No caller
      abandons a walk today, so no other test reaches it.
  - **Mutations.**
    - Restore line deleted from `Drop`: red, case 1.
    - Restore moved from `Drop` into `start`, the reviewer's placement: red, case 2 (`a deferral
      made after a dropped walk`).
  - **No bit moved, again.** The same instrumentation and scratch generator were re-applied to the
    folded head. The run skipped only the new test, since its walks have no base counterpart. The
    per-binary digests are identical to the base table above: 138,498 walks in all, every ordered
    and sorted digest the same, and the seeded generator still at `a31771ff..39e4`. So no current
    caller ever drops an unfinished walk, and the restore never fires on their paths.
  - **Doc.** `BlockWalk`'s doc now says a dropped walk stops at its last decided offset and loses
    no event; it no longer describes the loss as acceptable.
- Doctest wording: the `compile_fail` twin is the plain twin with the `retarget` line moved, and
  the doc says rustdoc checks `E0499` only on nightly.
- `cursor_history_valid`'s untested re-arm is tracked by the coordinator in #2258.

  - **Placement.** The restore body is `BlockWalk::abandon`, inside the marked realtime region.
    The one-line `impl Drop` that calls it sits just after the region, because the policy's text
    predicate reads `fn drop(` as a `drop(` call. That is the same placement as the engine's other
    render-reachable `Drop` impls (`RetirementCredit`, `CanonicalFpEnv`). The hash run above had
    the same body inline in `drop`, before it moved.
  - **Doctests.** A `Drop` type's borrow lasts until it is dropped, so both twins now end the walk
    with `drop(block)`. The plain twin drops the walk and then calls `retarget`. The `compile_fail`
    twin is that text with the `retarget` line moved above the drain. The pointer-instead-of-borrow
    mutation above was run at `59ae78d70`, before `drop(block)` was added.

**Gates after the fold (head).**
- `cargo test --locked --no-fail-fast -p automation -p graph -p builtins-compiler -p host-core
  --all-targets --features builtins-compiler/test-support,graph/test-support,host-core/test-support`:
  719 passed (the new test included), 0 failed.
- Doctests on the same packages and features: all pass, including both `BlockWalk` twins.
- clippy `-D warnings` on the same set: clean. `cargo fmt --check`: clean.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps -p automation -p graph -p builtins-compiler`:
  clean.
- `bash scripts/check-workspace-policy.sh`: ok.
- `cargo run --locked --release -p realtime-policy`: `ok (292 marked regions in 46 files)`.

**Follow-ups (not in this issue).**
- `cursor_history_valid`'s re-arm at a fader walk's start has no test that reaches it (mutation
  above). Tracked in #2258.
- host-core's broken intra-doc link `crate::warm::production_config` (pre-existing).
