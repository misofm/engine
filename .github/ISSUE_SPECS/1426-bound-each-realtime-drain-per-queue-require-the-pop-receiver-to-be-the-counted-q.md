# Bound each realtime drain per queue: require the pop receiver to be the counted queue

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0, D15-2).
Found while implementing *Require every loop around a realtime drain to drain a different queue on
each pass* (#1418, attempt 1, commit `da878bd49` on `codex/d15-stream-j2`); root filed it as a
separate issue. No production code changes.

**Blocked (root, 2026-10-05).** This issue is blocked on Amendment 1 of #1418. Root rescoped #1418
after two FAIL verdicts and reverted its gate changes (`9d955dc66`); the drain rule moves to a Rust
syntax-tree tool (`tools/realtime-policy`). Do not start an attempt until that amendment and its
tool issues are filed. The amendment says whether this rule lands in the awk gate cited below or in
the tool; if in the tool, this body is restated against it first. The anchors below are re-verified
on the gate as it is after the revert (`scripts/check-realtime-policy.sh` is at its pre-#1418
state).

## Problem (verified on `codex/d15-stream-j2` at `4387b935e`, after #1418's revert)

- **The drain bound is a count of one queue, but the gate never checks which queue is popped.**
  `scripts/check-realtime-policy.sh` accepts a marked `try_pop` drain when its innermost loop is
  bounded by an entry count (`counted()`, `:206-225`). `counted()` matches the count's binding
  against a receiver pattern (`recv`, `:219`; the binding test at `:221`) but never keeps the
  receiver it matched. The pop is matched by the bare word `try_pop` (`check()`, `:383-387`): it
  records the line, the frame stack and the `from_fn` flag, never its receiver. A pop is then
  judged by its loops alone (`:401-410`). So
  `let available = a.available_at_entry(); for _ in 0..available { b.try_pop() }` passes: queue
  `b` is drained by `a`'s count, and the bound is no longer a bound on `b`.
- **The header form keeps no receiver either.** `for P in 0..<path>.available_at_entry()`
  (`classify()`, `:292-293`) sets `inner[f]` without keeping `<path>`; a pop-receiver check must
  take the counted receiver from that path too.
- **No self-test case covers it.** In `scripts/test-realtime-policy.sh`, every counted case pops
  from the receiver it counts (`control`/`control`, `controls[lane]`/`controls[lane]`,
  `self.control_lane_consumer_for_this_strip` on both); no case pops from a different receiver.
- **The real tree already complies.** Every counted drain pops the queue it counts:
  - `crates/builtins-compiler/src/lib.rs` fader drain (`drain_fader_controls`, `:1064`: count
    `:1074`, bound `:1075`, pop `:1076`) and matrix drain (`drain_matrix_controls`, `:1108`: count
    `:1118`, bound `:1119`, pop `:1120`), both `control`;
  - `crates/graph/src/runtime.rs` `drain`: count `:897`, bound `:899`, pop `:900`
    (`self.control.consumer`);
  - `crates/effect-contract/src/live.rs` `stage_records`: count `:349-352`
    (`self.control.as_ref().map_or(0, Consumer::available_at_entry)`, so the counted receiver is
    `self . control`), bound `:359`, `:364-365`, pop `:366`
    (`self.control.as_mut().map(Consumer::try_pop)`). The receiver here precedes
    `.as_mut().map(Consumer::`, not `.try_pop`.
  - `crates/builtins-compiler/src/lib.rs` `drain_controls` (count `:472`, bound `:473`, pop `:474`;
    `control` is bound at `:469` by `let Some(control) = control.as_mut() else`) complies too, but
    at the head it is outside every marked region: #1418 attempt 1's markers were reverted, and
    #1418's amendment marks it.
  `crates/engine/src/realtime/plan_exchange.rs:377` pops once in an `if ... && let` header, not in
  a counted loop.

## Decisions

- **D1. A counted pop pops the counted queue.** For every `try_pop` that the gate judges against a
  counted inner loop (`inner[f]`), the pop's receiver must equal that loop's counted receiver,
  token for token, after stripping one trailing `. as_ref ( )` or `. as_mut ( )` from each. The
  counted receiver comes from the `let` count binding that `counted()` accepts (`:221`) or from the
  header path of `for P in 0..<path>.available_at_entry()` (`:292`). A pop whose
  receiver differs, or cannot be read, fails with the existing drain class
  (`bound it with available_at_entry`).
- **D2. Two pop spellings, nothing else.** The pop's receiver is read from exactly two forms:
  `<recv> . try_pop ( )` and `<recv> [. as_mut ( ) | . as_ref ( )] . map ( <Ident> : : try_pop )`.
  Any other spelling of a pop inside a counted drain fails (fail safe; the drain is rewritten, not
  the rule relaxed).
- **D3. The real tree passes unchanged.** No source file changes; the counted drains above pass.
- **D4. Comments.** The gate's rule comment (`:78-144`) states D1 and D2; the self-test's case
  comments say which defect each new case catches.

## Authorized paths

- `scripts/check-realtime-policy.sh`
- `scripts/test-realtime-policy.sh`
- This spec

## Non-goals

- Taking collections at their word (the comment at `:126-130` stays as it is).
- Changing any drain in production code.
- Pops outside a counted loop (single pops in headers, as at `plan_exchange.rs:377`).

## Hazards

- #1418's attempts rewrote `scripts/check-realtime-policy.sh` and were reverted (`9d955dc66`).
  #1418's Amendment 1 decides where the drain rule lives; this issue lands after it and is
  re-anchored on whatever it leaves.
- The gate runs under gawk, mawk and busybox awk. Any new function must not recurse on its own
  output (#1418's attempt 1 found an `if let` header that recursed without bound).

## Objective gates

1. **The real tree passes.** `bash scripts/check-realtime-policy.sh` exits 0 with the same region
   and file counts, under gawk, mawk and busybox awk (each through a `PATH` shim).
2. **The self-test passes** (`bash scripts/test-realtime-policy.sh`) under the same three awks,
   with these new expected failures (drain class), each a `mutate_<name>` registered in the drain
   list:
   - `pop_receiver_differs`: `let available = control.available_at_entry(); for _ in 0..available
     { let Ok(record) = other.try_pop() ... }`;
   - `pop_receiver_differs_header`: `for _ in 0..control.available_at_entry() { other.try_pop() }`;
   - `pop_receiver_differs_map`: the live.rs shape with `self.other.as_mut().map(Consumer::try_pop)`;
   - `pop_receiver_unreadable`: a pop spelled outside D2's two forms, e.g. `(control).try_pop()` or
     `Consumer::try_pop(control)`;
   - `pop_receiver_differs_index`: `controls[lane]` counted, `controls[other]` popped.
   and the valid fixture keeps passing, including its live.rs `map(Consumer::try_pop)` drain.
3. **Red on revert (PR evidence).** The gate at its parent commit plus the new self-test lets
   exactly the new cases through, and no other case.
4. **Mutations (PR evidence).** (a) The receiver comparison always true: the D1 cases are red.
   (b) The `as_mut`/`as_ref` strip removed: the valid fixture's live.rs drain is red, and gate 1 on
   the real tree fails at `crates/effect-contract/src/live.rs:366`. (c) The header-path counted
   receiver ignored: `pop_receiver_differs_header` is red.
5. `bash scripts/check-workspace-policy.sh` exits 0.

*Test value.* Each new case is red if the gate again accepts a drain whose bound counts one queue
and whose pop drains another, which lets a realtime drain run past its entry count; no existing
case pops a different receiver from the one it counts.

## Evidence

- Gates 1 and 2 under each awk; gate 3's case list; gate 4's mutation runs.

## Dependencies

- **Blocked on** Amendment 1 of *Require every loop around a realtime drain to drain a different
  queue on each pass* (#1418) and the stream J tool issues it names (`tools/realtime-policy`).
  This issue edits the drain rule where that amendment leaves it.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused. The self-test runs the gate on synthetic regions,
  which is the gate's own input, not a grep of the repository.
- Attempt budget: three attempts, one adversarial verdict each.
