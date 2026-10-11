# Check realtime regions with a Rust syntax-tree tool: refuse a call to a popping function in a marked loop or closure

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0, D15-2).
This is slice B2b-2 of nine (C1 (#1445), A1 (#1438), A2 (#1439), B1a (#1440), B1b (#1441), B2a (#1442), C2 (#1446), B2b-1 (#1443), B2b-2; A1's spec lists what
each holds).
- B2b-1 put every non-test pop in a marked region or on the control-side allowlist, and made the
  one restructure this slice needs.
- **B2b-2 (this issue)** closes the last documented limit inside marked regions, a pop that a loop
  reaches through a helper (root ruling R3, 2026-10-05): inside a marked region, a call to a
  function that pops, in a loop or in a closure, is refused. This closes probe a08 and its
  variants.

No production code changes. It waits for #1345 (Amendment 1), which makes the last real render
loop that calls a popping function markable (Dependencies).

## Problem (verified on the stream B batch-1 tree)

The tree is `codex/d15-stream-b` at `b8392df66` (stream B batch 1: #1309, #1343, #1314, #1311,
#1348) merged with `origin/main` at `6d28a80ec`, as in B2b-1 (whose Problem says how the merge's two
non-`.rs` conflicts are handled: no cited line is in them). Measured with a scratch `syn` program
outside the repository.

- **a08 passes every earlier slice.** `v1302/adv/a08_helper.rs` puts one region around
  `impl Lane`. Its helper `next_record` pops once (`self.control.try_pop().ok()`), and its caller
  runs `while let Some(record) = self.next_record()`. The pop is in a region and in no loop, which
  the drain rule accepts (the base tree's plan-exchange twin, `scripts/test-realtime-policy.sh:78`,
  has that shape), so the queue is drained until empty.
- **Two cheap variants** (third review, MAJOR-2). Both compile and pop until the queue is empty:
  - a hoisted function value: `let next = Self::next_record;` before the loop, then
    `while let Some(record) = next(self) { .. }`. The callee `next` is a single-segment local;
  - an iterator whose `next` pops: in a marked region,
    `impl Iterator for Lane { fn next(&mut self) -> Option<Record> { self.control.try_pop().ok() } }`
    and `for record in &mut *lane { .. }` (or `lane.by_ref().count()`). The `for` header calls
    `next` only implicitly.
- **The name set on the real tree** (D1's definition). The names of the non-test functions whose
  own body pops are 29 on the batch-1 tree (30 on `main`: batch 1 removed `enter_block`'s pop). Among
  them are common method names: `process` (`crates/builtins-compiler/src/lib.rs:4251`, the
  test-support input drain, which R4 counts as non-test), `stage`
  (`crates/effect-contract/src/live.rs:341`), `drain` (`crates/graph/src/runtime.rs:896`),
  `cancel`, `drop` and `meters` (`tools/audit`).
- **Call-rule hits on the real tree**, with every planned region in place (B1b-D5, B2b-1-D6, and
  `poll_meters` from the #1448 guard commit):
  - before B2b-1's restructure, in a loop: `crates/graph/src/runtime.rs:3393` and
    `crates/rack/src/lib.rs:2566`, `:2592` (all `.process(`), whose receivers cannot reach the
    input drain (B2b-1-D6's evidence table); after it, none of the three, because `process` leaves
    the set (measured: 28 names);
  - in a loop: `crates/source/src/lib.rs:1406` (the acquire's seek re-observe), which the stream-B
    issue *Bound the source acquire ..* moves out of the loop;
  - **no call in a closure.** On `main` the plan exchange's `self.enter_block()` inside
    `in_render_scope(|| ..)` (`plan_exchange.rs:454`, `:479`) was a hit. On the batch-1 tree
    `enter_block` reads #1343's mailbox and does not pop, so both hits are gone.
  - Single-segment path values that name a popping function are local variables
    (`crates/graph/src/runtime.rs:3173`, `:3178-3179` and `:3210-3211` `meters`, `:4080` `stage`,
    and `crates/rack/src/lib.rs:2184` `stage`; the same lines on `main` and on `b8392df66`). In
    marked regions no path value with two or more segments names a set member (third review,
    measured on `6d28a80ec`), and no popping function is named `next`.
- **Real render loops outside every region that call a popping function** (the limit "a loop in an
  unmarked caller"). Root ruled (2026-10-05) that each goes into the issue that owns its file:
  - `SpectrumCaptureCollection::cancel` and `select` (`crates/host-core/src/spectrum.rs:876`,
    `:930`): stream H's *Bound every drain the AudioWorklet runs on its audio thread* (its D9)
    moves the pop into the collection's loop, and B2b-1 marks it;
  - `LiveControlEffectBankStage::drain` (`crates/rack/src/lib.rs:1257-1284`) calls
    `channel.stage(..)` in `for lane in 0..lane_count` (`:1261`, `:1268`). Stream A owns
    `crates/rack`; stream B's #1345, which already holds a named exception for the rack code that
    calls `stage`, rewrites both functions. Its Amendment 1 makes that loop pass this rule and marks
    it.

## Decisions

- **D1. The name set.** Every non-test `fn` item under the scan roots (free, inherent, trait, trait
  default; B2b-1-D1's definition: cfg evaluated with `test` false and every feature on, `#[test]`
  excluded, no exemption by name) whose own body holds a pop (B1a-D5) contributes its name.
  Closures count as part of the body; nested `fn` items are their own functions. `try_pop` itself
  is not in the set.
- **D2. The rule.** Each of these is a finding, "marked realtime call to a popping function inside
  a loop or closure":
  1. Inside a marked region, inside a loop (the body or the header of a `for`, `while` or `loop`; a
     `for` loop's iterator expression is outside it) or inside a closure body (any closure, an
     `array::from_fn` closure included):
     - a method call whose method is in the set;
     - a call whose callee path's last segment is in the set;
     - a call whose callee is a single-segment path that B1a-D2's resolver binds to a `let` whose
       initializer contains a set name anywhere, wherever that `let` sits (in or out of the
       region): as a call's callee, a method call's method, or a path's last segment, at any depth,
       including inside a closure body, a cast, a `&` expression, a block or parentheses. This
       covers the hoisted-value variants: `let next = Self::next_record;`,
       `let next = |lane: &mut Lane| lane.next_record();` and
       `let cast = Lane::next_record as fn(&mut Lane) -> Option<u32>;` above BEGIN, each followed
       by `while let Some(r) = next(self)` (or `cast(self)`) in the region (fourth review,
       MAJOR-7);
     - in a macro token tree, an identifier in the set followed by a `( .. )` group.
  2. Anywhere in a marked region, in or out of a loop: a path value (a path that is not a callee)
     whose last segment is in the set. A single-segment path value is exempt only when B1a-D2's
     resolver binds it to a local `let`, a pattern or a parameter (`meters`, `stage` above).
  3. Anywhere under the scan roots, in non-test code: an `impl` of a trait whose path's last
     segment (after `r#` is stripped) is `Iterator`, `DoubleEndedIterator` or `IntoIterator`, or a
     name that a `use` under the roots renames one of those to, when any method in it has a pop in
     its own body. The class is "iterator implementation that pops": iteration calls `next`
     implicitly, which no call-site rule sees. Measured: none exists today.

  It is fail closed: the name is matched, not the type.
- **D3. The only fix is restructuring.** The pop moves out of the loop (or the closure), or into
  the loop's own body where the drain rule judges it. It is never fixed by an exemption, and never
  by renaming a function or adding a wrapper to dodge the set. A refused call whose receiver
  provably cannot reach the popping function is resolved only by a restructure that root names in
  the issue that makes it; the rule does not change. B2b-1-D6 is the one such decision so far; any
  later case goes to root by name.
- **D4. Limits that stay** (stated in the drain rule's module documentation):
  - a loop in an unmarked caller. B2b-1-D2 still requires the helper's pop to be marked or listed,
    and every real instance is in a marked region after B2b-1, #1449 and #1345 Amendment 1;
  - a helper two calls deep (the loop calls `a`, `a` calls `b`, `b` pops);
  - a function value that enters the function as a parameter, a field or a return value;
  - an implicit call other than iteration (`Drop`, `Deref`, an operator trait).
- **D5. New cases.** One each, in A1's case format:
  - `v1302/adv/a08` as committed: helper and `while let` caller in one region (refused, call rule);
  - a08 with the helper in its own region and the caller in another (refused, call rule);
  - B2b-1's helper-outside case gains the call-rule class: {unmarked try_pop, call rule};
  - a08 with the pop moved out of the loop (`let record = self.next_record();` once, before a
    counted loop) (accepted);
  - a08's caller loop outside every region, helper marked (accepted);
  - a marked loop that calls `lane.sweep()` while a `#[cfg(feature = "x")] fn sweep` elsewhere
    under the roots pops (refused: the name set is fail closed and features count as on);
  - the same with only a `#[cfg(test)] fn sweep` that pops (accepted: test code is not in the set);
  - a marked closure, called in no loop, that calls the a08 helper (refused, closure);
  - `.map(Self::next_record)` in a marked loop (refused, path value). Defect it alone catches: a
    path value refused only where it stands as a whole `let` initializer, so a path passed as an
    argument escapes;
  - the hoisted value: `let next = Self::next_record;` and `while let Some(r) = next(self)`, both in
    the region (refused twice: the path value and the call);
  - the hoisted value with its `let` above BEGIN and the loop inside the region (refused: the call
    through the local);
  - the hoisted closure, `let next = |lane: &mut Lane| lane.next_record();` above BEGIN, and
    `while let Some(r) = next(self)` in the region (refused: the call through the local);
  - the hoisted cast, `let cast = Lane::next_record as fn(&mut Lane) -> Option<u32>;` above BEGIN,
    and `while let Some(r) = cast(self)` in the region (refused: the call through the local);
  - the iterator lane: `impl Iterator for Lane` whose `next` pops, and `for record in &mut *lane`
    in a marked region (refused: the iterator class);
  - the same `impl` with `lane.by_ref().count()` in place of the `for` (refused: the iterator
    class);
  - **the twin of #1345's bank drain** as its Amendment 1 D8 prescribes it: a pop-free loop that
    stages each lane's cells, then a drain-only observation loop over `self.lanes.iter_mut()`
    that names the lane only in its binding, its count and its pop, and hands each record to a
    sink not reached through the lane (accepted);
  - **the twin of #1449's `cancel_except`** is B2b-1's (its D9 twins); this slice runs the call rule
    on it: the reset loop calls only the pop-free `reset_after_cancel` (accepted).
- **D6. Documentation.** The drain rule's module documentation drops the helper limit of B1b-D4
  and states D1 to D4.

## Authorized paths

- `tools/realtime-policy/**`
- This spec

## Non-goals

- Any production change. The restructures are in B2b-1, the three production issues and #1345.
- #1418's and #1426's rules.
- Closing D4's limits.

## Hazards

- **Re-measure at implementation time:** the name set and every call-rule hit. If a hit remains on
  the real tree, stop and report it by name; do not change the rule or the code (D3).
- **The name set includes test-support and feature-gated code.** A later non-test helper that pops
  and shares a name with a method that a marked loop or closure calls will refuse that loop. The
  fix is a restructure (D3), named by root.
- **Over-refusal is acceptable, a false pass is not.**

## Objective gates

1. **The real tree.** `cargo run --locked --release -q -p realtime-policy` passes with the same
   counts as before this slice.
2. **The cases.** `cargo test --locked -p realtime-policy` passes, with every earlier case and
   every D5 case.
3. **Red on revert (PR evidence).**
   - Undo B2b-1-D6's restructure on a scratch export: the tool reports the three `.process(` calls
     under the call rule, and each matches a row of B2b-1-D6's evidence table.
   - Put `acquire_current_block`'s re-observe back into its first loop on a scratch export: the
     tool reports it.
   - Mark today's `LiveControlEffectBankStage::drain` shape (the parent of #1345 Amendment 1) on a
     scratch export: the tool reports `channel.stage(..)`.
   - Mark today's `SpectrumCaptureCollection::cancel` (the parent of #1449 D9) on a scratch export:
     the tool reports `capture.cancel()` in its loop.
4. **Mutations (PR evidence).** Apply each alone. Each turns red the named cases, and the real
   tree still passes:
   - the call rule dropped: a08 and its two-region variant;
   - closures left out of the call rule: the marked-closure case;
   - the call rule's set built with test code included: the `#[cfg(test)] fn sweep` case;
   - the call rule's set built with features off: the `#[cfg(feature = "x")] fn sweep` case;
   - path values read only in loops and closures: the in-region hoisted-value case's path-value
     finding;
   - calls through a local not resolved: the hoisted-value case with its `let` above BEGIN;
   - a local's initializer read only as a bare path: the hoisted-closure and hoisted-cast cases;
   - path values refused only as whole `let` initializers: the `.map(Self::next_record)` case;
   - the iterator class dropped: both iterator-lane cases;
   - the call rule applied outside marked regions: the accepted caller-outside case, and gate 1
     (for example `crates/control-plane/src/control.rs:947` calls `try_reclaim` in a loop).
5. `cargo fmt --all -- --check`,
   `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings` and
   `bash scripts/check-workspace-policy.sh` exit 0.

*Test value.*
- The a08 cases are red if a popping helper called in a marked loop passes again: the escape that
  drained a queue until empty while every gate was green. No earlier case reaches it.
- The hoisted-value cases are red if a function value taken outside the loop, or above the
  region, carries a popping helper into a marked loop. The closure and cast cases are red if the
  rule reads the local's initializer only as a bare path.
- The iterator cases are red if a drain written as iteration passes, which is the usual way to
  write a drain in Rust and which no call-site rule sees.
- The two `sweep` cases are red if the name set moves off R4's definition: a feature-gated helper
  left out (fail-open by name), or a test helper counted (refusing calls that no build reaches).
- The marked-closure case is red if a popping helper called from a closure passes again, the shape
  the plan exchange had on `main` before #1343.
- The accepted caller-outside case is red if the call rule runs outside marked regions. That
  over-refusal would refuse every unmarked control loop that calls an allowlisted drain.
- The pop-moved-out case is red if the rule refuses the restructure it prescribes.
- The #1345 twin keeps that shape tested after the real code changes again.
- The two-calls-deep limit has no case. It is stated in the module documentation only, so that a
  later rule that closes it has no committed case to delete.

## Evidence

- Gates 1-5 output. The call-rule name set's size, and every call-rule hit on the reviewed tree
  with how it was resolved.

## Dependencies

- After (same stream): B2b-1 (which comes after C2 and the #1448 guard commit).
- After (other streams): stream B #1345 (Amendment 1), which marks the rack bank loop in a shape
  this rule accepts.
- Before: #1418 (Amendment 1) and #1426 (Amendment 1), which start once the whole J batch is on
  `main`.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused. The cases are the tool's own input.
- Attempt budget: three attempts, one adversarial verdict each.
- Size: half a day.
