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

## Amendment 1 (root, 2026-10-05)

**Status.** Root amends this issue before any attempt.
- #1418 is rescoped (its Amendment 1).
- *Check realtime regions with a Rust syntax-tree tool* (slices C1 (#1445), A1 (#1438), A2 (#1439), B1a (#1440), B1b (#1441), B2a (#1442), C2 (#1446),
  B2b-1 (#1443) and B2b-2 (#1444); "the tool issue" below) replaces `scripts/check-realtime-policy.sh` and its
  self-test with `tools/realtime-policy`, which reads `syn`'s syntax tree.
- This issue is now implemented in the tool's drain rule, after #1418. Nothing in awk changes.
- The attempt budget stays at three attempts.

**Problem, re-anchored on the tool.** The Problem above was re-verified on `codex/d15-stream-j2`
at `4387b935e`, after #1418's revert (`9d955dc66`); #1436 merged that branch, and
`scripts/check-realtime-policy.sh` is unchanged on `origin/main` at `6d28a80ec`. Its anchors
still hold there, except two ranges that are off by a few lines: `counted()` is `:206-226` (the
Problem says `:206-225`), and the judging loop is `:404-413` (the Problem says `:401-410`):
- `counted()` (`:206-226`) matches the count's receiver in its regex (`recv`, `:219`; the binding
  test `:221`) but never keeps it;
- a pop is the bare word `try_pop` (`:383-387`). It is recorded with its line, its loop stack and
  its `from_fn` flag, never its receiver;
- the header form `for P in 0..<path>.available_at_entry()` (`:292-293`) keeps no receiver either;
- each pop is judged by its loops alone (`:404-413`).

Those anchors describe the gap. This issue no longer edits the script: the tool issue ports this
rule as it stands (tool B1a-D5, B1a-D6 and B1a-D8). Its count row constrains the receiver's
form, not its identity, so the gap moves into the tool unchanged. This issue closes it there.

**Real-tree claims are re-measured.** Stream B's cell slices (#1312, #1346, #1347, #1345) remove
the fader, matrix, input, route and effect-parameter rings (D15-2). Root ruled that this issue is
not ordered against them. At implementation time, list the counted drains that `main` still marks
and record any that a cell slice removed. On `6d28a80ec` (the same lines on the stream B batch-1
tree), every counted drain pops the queue it counts:
- `crates/builtins-compiler/src/lib.rs` `drain_controls`: count `:472`, bound `:473`, pop `:474`
  (`control`, bound at `:469-471` by `let Some(control) = control.as_mut() else`). No line moves:
  tool B1b-D5's markers replace blank lines.
- The fader drain (`:1074`, `:1075`, `:1076`) and the matrix drain (`:1118`, `:1119`, `:1120`),
  both on `control`, bound the same way.
- `crates/graph/src/runtime.rs` `drain`: count `:897`, bound `:899`, pop `:900`
  (`self.control.consumer`).
- `crates/effect-contract/src/live.rs` `EffectControlLane::stage` (`:341`):
  - count `:349-352` (`self.control.as_ref().map_or(0, Consumer::available_at_entry)`);
  - alias `remaining` (`:359`);
  - bound `:364-365`;
  - pop `:366` (`self.control.as_mut().map(Consumer::try_pop)`).

On `main`, `crates/engine/src/realtime/plan_exchange.rs:377` pops once, in an `if .. && let`
header, in no counted loop; stream B batch 1 (#1343) removes that pop, and the tool's base tree
keeps its shape (`scripts/test-realtime-policy.sh:78`, ported).

After the J tool batch (B2b-1 and the #1448 guard commit), `main` also marks the drains that
production issues rewrote in the counted form: the source acquire's two phases
(`self.data_consumer`), the meter poll (`meter.consumer`, in tool B1a-D8b's per-queue form), the
spectrum drains (`self.consumer`), the spectrum collection's drain loop (`capture.consumer`), and,
after #1345 Amendment 1, the effect bank's observation loop. Each pops the queue it counts; list
them with the others at implementation time.

The tool's cases hold a synthetic twin of each of these shapes (tool B1b-D6, B2b-1-D9,
B2b-2-D5). The base tree's `stage_records` is the twin of `stage`. The twins keep this rule tested
after the cell slices.

**Decisions, as amended.**
- **D1 stands, in syntax-tree terms.** For every pop that the tool judges against a counted
  innermost loop (tool B1a-D8), three things must hold:
  1. The pop's receiver equals that loop's counted receiver, token for token, after one trailing
     `.as_ref()` or `.as_mut()` is stripped from each. The counted receiver comes from either:
     - the count binding (tool B1a-D6), through an alias where there is one;
     - the header path of `for P in 0..<path>.available_at_entry()`.
  2. The root identifier of each receiver resolves to the same binding, through the tool's one
     resolver (tool B1a-D2, as #1418 extends it). So a same-spelled receiver is refused if a `let`,
     a pattern or a closure parameter rebinds it between the count and the pop.
  3. **The receiver's place is not touched between the count and the pop** (fifth review,
     MINOR-2; the rule B1a-D8b condition 5 states for the per-queue form). From the count binding
     (or the header, for the header form) to the end of the counted loop, the receiver's root is
     named only in these places:
     - the count's own receiver and the pop's receiver;
     - a field path that leaves the receiver: the root followed by `.field` steps that differ from
       the receiver's at a step both have (`self.ramp`, `self.symmetry.admit(..)`,
       `self.staged_targets += 1` beside the receiver `self.control`), as B1a-D8b condition 6
       item 1 reads field paths;
     - inside `debug_assert!`, whose arguments are held to the same list.

     Anything else refuses the drain: the bare root as a value, a reborrow, a destructure, the
     receiver or a prefix of it in any other position (`mem::swap(&mut control, ..)`,
     `self.control = ..`), a call that takes the root or a prefix of the receiver
     (`self.refresh()`), and a mention in any other macro token tree. On `6d28a80ec` every real
     counted drain meets this (the receivers `control`, `self.control.consumer` and
     `self.control`; their bodies name `self` only in field paths that leave the receiver).
     Re-measure; if a real drain fails, stop and report it.

  A receiver that differs, or that cannot be read, gets the existing drain class.

  **Why the place rule sits here for D8 and in B1a for D8b** (fifth review, MINOR-2, option
  chosen: the same rule, applied to each form where that form's receiver identity is checked).
  D8b's receiver identity is checked in B1a, which lands in the J batch, so its place rule lands
  with it and the batch never ships the per-queue form without it. D8's receiver identity is this
  issue's D1, so its place rule is item 3 here; checking a place for a receiver whose identity
  is not yet compared would protect nothing. One rule stated once in this issue would leave the
  per-queue form open from the batch push until this issue lands.

  **The per-queue form is already this rule** (tool slice B1a-D8b, root's ruling 1 on the fourth
  review). Its conditions require the pop's receiver to be `<r'><path>`, the same `<path>` that
  the set loop reads on `<r>`, with `<r>` and `<r'>` drawn from the same collection at the same zip
  position and both resolved by the one resolver. So D1 does not judge a per-queue pop again, and
  this issue adds no per-queue case: B1a's `per_queue_pop_other_queue` (a different queue under
  the guard), `per_queue_receiver_rebound` (the same spelling bound to another `let`),
  `per_queue_pop_from_wrong_zip_position` (a binding from another zip step) and
  `per_queue_element_swapped_with_spare` (the counted place replaced, item 3's defect) hold the
  defects D1 catches, with B1a's mutations. Applying D1 to a per-queue pop would
  find no counted innermost loop and refuse it (mutation (e)). The meter poll
  (`meter.consumer`, `hosts/host-web/src/lib.rs`, after stream H's *Bound the browser meter poll
  by each queue's count at entry*) has this shape.
- **D2 stands.** A counted drain has exactly two pop forms:
  - `<recv>.try_pop()`;
  - `<recv>[.as_mut() | .as_ref()].map(<Type>::try_pop)`.

  Any other spelling inside a counted drain is refused, for example
  `Consumer::try_pop(control)` or `(control).try_pop()`. Outside a counted drain, tool B2a-D3
  still governs function values.
- **D3 stands.** The real tree passes unchanged.
- **D4, amended.**
  - D1 and D2 are stated in the drain rule's module documentation in the tool, not in an awk
    comment.
  - Each new case's doc comment names the defect it catches.

**Authorized paths, as amended.**
- `tools/realtime-policy/**`
- This spec

**Non-goals.** As above. #1418's queue-selection rule is in place and is not changed here.

**Hazards, as amended.**
- The two awk hazards are void.
- New: D1's binding check reuses the tool's one resolver. A second resolver that disagrees with
  the first is a false pass waiting to happen.

**Objective gates, as amended** (they replace gates 1-5).
1. **The real tree passes.** `cargo run --locked --release -q -p realtime-policy` prints the same
   region and file counts as before this change, re-measured on `main`.
2. **The cases.** `cargo test --locked -p realtime-policy` passes.
   - **Expected failures (drain class):**
     - `pop_receiver_differs`:
       `let available = control.available_at_entry(); for _ in 0..available { let Ok(record) = other.try_pop() else { break; }; .. }`;
     - `pop_receiver_differs_header`:
       `for _ in 0..control.available_at_entry() { other.try_pop() .. }`;
     - `pop_receiver_differs_map`: the `stage` shape with
       `self.other.as_mut().map(Consumer::try_pop)`;
     - `pop_receiver_unreadable_call`: `Consumer::try_pop(control)` in a counted loop;
     - `pop_receiver_unreadable_paren`: `(control).try_pop()` in a counted loop;
     - `pop_receiver_differs_index`: `controls[lane]` counted, `controls[other]` popped;
     - `pop_receiver_rebound`: `let control = other;` inside the counted loop, before
       `control.try_pop()`;
     - `pop_receiver_swapped_with_spare`: a correct counted drain of `self.control`, with
       `core::mem::swap(&mut self.control, &mut self.spare);` after the pop in the loop body
       (item 3). The spare's records then pop under the first queue's count;
     - `pop_receiver_swapped_before_loop`: `let available = control.available_at_entry();
       core::mem::swap(control, other); for _ in 0..available { control.try_pop() .. }` (item 3).
   - **Still accepted:** the `stage` twin (`as_ref` count, `as_mut().map(Consumer::try_pop)` pop,
     alias count), the `drain_controls` twin, the route-drain twin (field paths of `self` beside
     the receiver `self.control.consumer`), and B1a-D8b's two accepted per-queue cases (the meter
     poll's shape first).
3. **Red on revert (PR evidence).** Run the tool at this change's parent with this change's cases.
   Exactly the nine new cases pass, and no other case changes.

   B2a accepts `Consumer::try_pop(control)` as a callee (B2a-D3). D2 refuses it inside a counted
   drain, so `pop_receiver_unreadable_call` is new red here.
4. **Mutations (PR evidence).** Apply each alone:
   - (a) the receiver comparison always true: the D1 cases are red;
   - (b) the `as_mut`/`as_ref` strip removed: the `stage` twin is red. If `live.rs:366` still pops
     on `main` at implementation time, gate 1 also fails there. Otherwise record its removal;
   - (c) the header-path receiver ignored: `pop_receiver_differs_header` is red;
   - (d) spelling compared without binding resolution: `pop_receiver_rebound` is red;
   - (f) item 3 not checked: `pop_receiver_swapped_with_spare` and
     `pop_receiver_swapped_before_loop` are red;
   - (g) item 3 with no field-path allowance (every mention of the root refused): the route-drain
     twin is red, and gate 1 fails at the real drains still marked. If the twin's loop body names
     no other field of `self`, give it the real drain's `self.position = 0;` so that (g) has a
     committed red case;
   - (e) D1 applied to per-queue pops as to D8 pops: B1a-D8b's accepted
     per-queue cases are red (no counted innermost loop gives them a counted receiver), and gate 1
     fails at `poll_meters`.
5. `cargo fmt --all -- --check`,
   `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings` and
   `bash scripts/check-workspace-policy.sh` exit 0.

*Test value, as amended.* The *Test value* section above stands. Added:
- `pop_receiver_rebound` is red if receivers are compared by spelling alone. Then a queue reached
  through a rebound name is drained by another queue's count. No other case rebinds the receiver
  between the count and the pop.
- The two `unreadable` cases are red if a pop form outside D2 is read as a receiver instead of
  refused.
- The two `swapped` cases are red if the receiver is compared by identity but its place can be
  replaced between the count and the pop: the drain then pops another queue under this queue's
  count, which no identity case reaches.
- The twins keep mutation (b)'s defence after #1345 removes the real `stage` drain.

**Dependencies, as amended.**
- After (same stream): #1418 (Amendment 1), which comes after the whole J tool batch is on `main`
  (C1, A1, A2, B1a, B1b, B2a, C2, the #1448 guard commit, B2b-1 and B2b-2), hence after B2b-2 and
  B #1345 (Amendment 1). This issue is outside the batch (root, 2026-10-05).
- Not ordered against #1312, #1346 and #1347 (root's ruling, 2026-10-05). Real-tree claims are
  re-measured (above).
