# Bound each realtime drain per queue: require the pop receiver to be the counted queue

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0, D15-2).
Found while implementing *Require every loop around a realtime drain to drain a different queue on
each pass* (#1418, attempt 1, commit `da878bd49` on `codex/d15-stream-j2`); root filed it as a
separate issue. No production code changes.

## Problem (verified on `codex/d15-stream-j2` at `5a73048e4`, which contains #1418)

- **The drain bound is a count of one queue, but the gate never checks which queue is popped.**
  `scripts/check-realtime-policy.sh` accepts a marked `try_pop` drain when its loop is bounded by
  an entry count (`counted()`, `:234-259`). `counted()` extracts the count's receiver into `crecv`
  (`:249-252`), but `crecv` is used only by #1418's outward queue-selection walk (`selwalk`,
  `:334` and `:346`). The pop is matched by the bare word `try_pop` (`check()`, `:538-541`): it
  records the line, the frame stack and the `from_fn` flag, never its receiver. A pop is then
  judged by its loops alone (`:561-569`). So
  `let available = a.available_at_entry(); for _ in 0..available { b.try_pop() }` passes: queue
  `b` is drained by `a`'s count, and the bound is no longer a bound on `b`.
- **The header form never sets `crecv`.** `for P in 0..<path>.available_at_entry()` (`classify()`,
  `:328-331`) passes its path straight to `selwalk`; a pop-receiver check must take the counted
  receiver from that path too.
- **No self-test case covers it.** In `scripts/test-realtime-policy.sh`, every counted case pops
  from the receiver it counts (`control`/`control`, `controls[lane]`/`controls[lane]`,
  `self.control_lane_consumer_for_this_strip` on both); no case pops from a different receiver.
- **The real tree already complies.** The five marked drains pop the queue they count:
  - `crates/builtins-compiler/src/lib.rs` `drain_controls`: count `:473`, bound `:474`, pop `:475`
    (`control`, bound at `:470` by `let Some(control) = control.as_mut() else`);
  - the same file's fader drain (`:1076`, `:1077`, `:1078`) and matrix drain (`:1120`, `:1121`,
    `:1122`), both `control`;
  - `crates/graph/src/runtime.rs` `drain`: count `:897`, bound `:899`, pop `:900`
    (`self.control.consumer`);
  - `crates/effect-contract/src/live.rs` `stage_records`: count `:349-352`
    (`self.control.as_ref().map_or(0, Consumer::available_at_entry)`, so `crecv` is
    `self . control`), bound `:359`, `:364-365`, pop `:366`
    (`self.control.as_mut().map(Consumer::try_pop)`). The receiver here precedes
    `.as_mut().map(Consumer::`, not `.try_pop`.
  `crates/engine/src/realtime/plan_exchange.rs:377` pops once in an `if ... && let` header, not in
  a counted loop.

## Decisions

- **D1. A counted pop pops the counted queue.** For every `try_pop` that the gate judges against a
  counted inner loop (`inner[f]`), the pop's receiver must equal that loop's counted receiver,
  token for token, after stripping one trailing `. as_ref ( )` or `. as_mut ( )` from each, as
  `counted()` already does for `crecv`. The counted receiver comes from the `let` count binding
  (`crecv`) or from the header path of `for P in 0..<path>.available_at_entry()`. A pop whose
  receiver differs, or cannot be read, fails with the existing drain class
  (`bound it with available_at_entry`).
- **D2. Two pop spellings, nothing else.** The pop's receiver is read from exactly two forms:
  `<recv> . try_pop ( )` and `<recv> [. as_mut ( ) | . as_ref ( )] . map ( <Ident> : : try_pop )`.
  Any other spelling of a pop inside a counted drain fails (fail safe; the drain is rewritten, not
  the rule relaxed).
- **D3. The real tree passes unchanged.** No source file changes; the five drains above pass.
- **D4. Comments.** The gate's rule comment (`:80-169`) states D1 and D2; the self-test's case
  comments say which defect each new case catches.

## Authorized paths

- `scripts/check-realtime-policy.sh`
- `scripts/test-realtime-policy.sh`
- This spec

## Non-goals

- Proving that a collection holds each queue once (`:136-139` stays as it is).
- Changing any drain in production code.
- Pops outside a counted loop (single pops in headers, as at `plan_exchange.rs:377`).

## Hazards

- `scripts/check-realtime-policy.sh` was rewritten by #1418, which is not on `main` yet; this
  issue lands after it.
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

- *Require every loop around a realtime drain to drain a different queue on each pass* (#1418):
  this issue edits the gate as #1418 leaves it.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused. The self-test runs the gate on synthetic regions,
  which is the gate's own input, not a grep of the repository.
- Attempt budget: three attempts, one adversarial verdict each.
