# Check realtime regions with a Rust syntax-tree tool: the drain-bound rule's mapping, scope and innermost bound

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0, D15-2).
Root rescoped #1418 after its two verdicts and ruled that the awk gate is replaced by a
syntax-tree tool. This is slice B1a of nine (C1 (#1445), A1 (#1438), A2 (#1439), B1a, B1b (#1441), B2a (#1442), C2 (#1446), B2b-1 (#1443), B2b-2 (#1444); A1's
spec lists what each holds):

- A1 and A2 built `tools/realtime-policy` and every non-drain check.
- **B1a (this issue)** ports the core of the drain-bound rule as `main` states it: how a pop maps
  to its region, the scope resolver, the visited syntax, pops, count bindings, reads before the
  loop and the innermost bounding loop. It commits every ported drain case that these rules
  decide.
- B1b ports the rest (outer loops, attributes and items, constructors), commits the remaining
  ported cases and the synthetic twins, and marks `drain_controls`.
- B2a refuses pops in closures, macros and function values; C2 deletes the awk gate; then, after
  the #1448 guard commit, B2b-1 and B2b-2 close the pops outside every region and the helper calls.

No production code changes.

## Problem (verified on `origin/main` at `6d28a80ec` and on `codex/d15-stream-b` at `b8392df66`)

The awk script and its self-test are the same on both, except that stream B batch 1 (#1309,
#1343, #1314, #1311, #1348) raises the floors and pads the self-test's base tree. The lines below
are `main`'s (`6d28a80ec`). On `b8392df66`, which lands first, every line of
`scripts/check-realtime-policy.sh` from `:70` on is two lower (the floors are `:75-76`, `counted()`
is `:208-228`, and the file has 463 lines), and every line of `scripts/test-realtime-policy.sh` from
`:440` on is one lower.

- **The drain rule lives only in awk.** `scripts/check-realtime-policy.sh:79-141` states it, and
  `:142-430` implements it as a lexer, a brace stack and a header reader. The rule: every
  render-thread `try_pop` runs at most once per record counted at block entry by
  `available_at_entry`, per pass of the finite loops around its drain. Three verdict rounds found
  grammar the lexer cannot see (slice A1, Problem).
- **The drain cases.** `scripts/test-realtime-policy.sh` holds `marked-unbounded-try-pop-drain`
  (`:599-600`) and every name in the loop at `:1387-1402` (56 names). Their bodies are at
  `:605-1385`, and their class is `drain_class` (`:617`). The base tree's drain shapes are valid
  (`:72-104`, `:150-179`, `:243-342`, `:414-436`).
- **The real drains** (re-measured at implementation time; see Hazards). On the batch-1 tree these
  are the marked pops:
  - `crates/builtins-compiler/src/lib.rs` `drain_fader_controls`: outer
    `for (lane, control) in controls.iter_mut().enumerate()` (`:1068`),
    `let Some(control) = control.as_mut() else { continue; };` (`:1069-1071`), count `:1074`,
    bound `:1075`, pop `:1076`;
  - `drain_matrix_controls` in the same file: `:1112`, `:1113-1115`, `:1118`, `:1119`, `:1120`;
  - `crates/graph/src/runtime.rs` route `drain` (`:896`): count `:897`, bound `:899`, pop
    `:900`. It has no outer loop;
  - `crates/effect-contract/src/live.rs` `EffectControlLane::stage` (`:341`): count `:349-352`
    (`.as_ref().map_or(0, Consumer::available_at_entry)`), alias `:359`, bound `:364-365`, pop
    `:366` (`.as_mut().map(Consumer::try_pop)`). It has no outer loop;
  - `crates/engine/src/realtime/spsc.rs:441`: the `fn try_pop` definition.

  `main` also marks one pop in an `if .. && let` header in no loop
  (`crates/engine/src/realtime/plan_exchange.rs:377`). Batch 1's #1343 removes it; the base tree's
  twin (`scripts/test-realtime-policy.sh:78`) keeps the shape tested.
- **Stream B's cell slices replace these drains.** #1312 (fader and matrix), #1346 (input,
  `drain_controls`), #1347 (the route drain in `runtime.rs`) and #1345 (the effect-parameter lane
  in `live.rs`) turn the live value lanes into latest-target cells (D15-2). Root ruled that the
  tool slices are not ordered against them. So every real-tree claim here is re-measured at
  implementation time. Each shape stays tested by a committed synthetic case, whatever the real
  tree then holds.

## Decisions

### Mapping

- **D1. Region nodes.**
  - A region's nodes are A2-D1's maximal nodes.
  - The **top** of a pop's path is the outermost region node on it.
  - A node the rule reads must lie inside the same region as the pop: the count binding, an alias,
    the bounding loop, or (in B1b) an outer loop or a slice parameter's `fn` signature. Otherwise
    the drain is refused. This is the awk's region-local reading (`counted()`, `:206-212`, searches
    only statements inside the region).
  - Inside a region that sits in a macro token tree (A2-D1), there is no syntax tree. Any
    `try_pop` token there is refused.
  - So a loop that encloses the region (for example a region opened inside a loop's body) is not
    read as a bounded outer loop: the drain is refused.
- **D2. Scope.**
  - The nearest binding of a name in scope is resolved by Rust's block scope. A binding is a
    `let`, a pattern of `let`, `if let`, `while let`, `match`, `for` or a closure, or a `fn`
    parameter.
  - For a count, that binding must be a `let` of a D6 form. Any other nearest binding refuses the
    drain (`pattern_rebound_count`, `count_rebound_by_for_pattern`).
  - Use one resolver. B1b, B2b-2, #1418 and #1426 extend it.
- **D3. Nested items.** A loop outside a nested `fn` item counts as "around" a pop inside that
  item, as the awk's brace stack treats it. This is fail closed: it can only add loops to judge.
  D4 lets the walk reach such a pop (`Stmt::Item` and `Item::Fn` are on its list), so D3 decides
  the case.
- **D4. Visited syntax: an exhaustive allow-list for `syn` 2.0.119.**
  - On the path from a pop up to its top, every node must be one of these:
    - `File`;
    - `Item::{Fn, Impl, Mod, Trait}`, `ImplItem::Fn`, `TraitItem::Fn`;
    - `Block`;
    - `Stmt::{Local, Expr, Item, Macro}`, with `LocalInit` and its `diverge` (`let .. else`)
      included;
    - `Arm`, its guard included;
    - `FieldValue` (a field of an `Expr::Struct`);
    - `Expr::{Array, Assign, Binary, Block, Break, Call, Cast, Closure, Continue, Field, ForLoop,
      Group, If, Index, Let, Lit, Loop, Macro, Match, MethodCall, Paren, Path, Range, RawAddr,
      Reference, Repeat, Return, Struct, Try, Tuple, Unary, Unsafe, While}`.
  - Anything else on that path refuses the drain: `Expr::{Async, Await, Const, Infer, TryBlock,
    Yield, Verbatim}`, `Item::{Const, Static, ..}` holding a pop, and any variant that a later
    `syn` adds.
  - `Expr::Closure` and `Expr::Macro` are transparent in this slice: the enclosing loops judge a
    pop in them, as the awk does. B2a replaces this with refusal.

### The rule, ported (one row per element of `main`'s rule)

- **D5. Pops** (`:89-90`, `:383-387`).
  - A pop is any of these:
    - a method call named `try_pop`. The identifier is compared after stripping `r#`, so
      `control.r#try_pop()` is a pop;
    - a path expression whose last segment is `try_pop`, called or passed as a value
      (`live.rs:366`);
    - the identifier `try_pop` in a macro token tree (`Expr::Macro` or `Stmt::Macro`), judged by
      the enclosing loops.
  - A `fn try_pop` definition is not a pop.
- **D6. Count bindings** (`:93-96`, `:206-226`). `<count>` is the D2 binding. It must be one of:
  - `let [mut] <count>[: T] = <recv>.available_at_entry()[.min(<cap>)];`
  - `let [mut] <count>[: T] = <recv>.map_or(0, <Type>::available_at_entry)[.min(<cap>)];`
  - `<recv>` may also carry one `.as_ref()` or `.as_mut()` before `.map_or`, as `live.rs:349-352`
    does;
  - `<recv>` is an identifier, then fields, zero-argument method calls, and indexes by an integer
    literal or a path (`self.controls[lane]`);
  - `<cap>` is an integer literal, or an identifier followed by `.field` or `::segment` steps
    (`MAX_RECORDS_PER_BLOCK`, `self.transfer_block_count`), as the awk's `cap` pattern says
    (`:220`). A method call is not a cap;
  - or one plain alias, `let [mut] <count> = <entry count>;` (`:222-224`; `live.rs:359`).
- **D7. Reads before the loop** (`:97-100`, `:236-260`).
  - Between the binding and the loop, every mention of `<count>` is the left operand of `==`,
    `!=`, `<`, `<=`, `>` or `>=`.
  - A mention inside a macro call is refused, except in `debug_assert!`. Its arguments are parsed
    as expressions and held to the same rule. A raw-identifier macro such as `r#while!` is a
    macro.
  - The loop header names `<count>` exactly once.
- **D8. The innermost loop bounds the pop** (`:89-93`, `:292-305`, `:349-352`). Its whole header,
  after attributes and a label, is one of:
  - `for P in 0..<count>`;
  - `for P in 0..<recv>.available_at_entry()`;
  - `while <count> != 0` or `while <count> > 0`. Its body's first statement is `<count> -= 1;`,
    and the body never names `<count>` again.

  A pop in a loop header is refused (`:113-114`, `:322-323`). That covers a `while` condition
  (`let` chains included), a `for` iterator expression or pattern, and a block nested there.
- **D8b. The per-queue counted pop** (root's ruling 1 on the fourth review, 2026-10-05: no
  exemption; it replaces the minimum form that the third-review fold drafted here). One remaining
  count per queue is read at entry; each pop is guarded by its own queue's count and decrements
  it. A pop that is not in a D8 innermost counted loop is also bounded when all of these hold:
  1. **Names.** `<fields>` is an identifier followed by one or more `.field` steps
     (`self.lanes`, `ready.meters`). `<Q>` (the queues' collection) and `<K>` (the counters) are
     two `<fields>`. The root identifier of each resolves by D2 to the same binding in the set
     loop and in the pass loop, and nothing assigns or rebinds that root in between. `<path>` is
     one or more `.field` steps (`.consumer`).
  2. **The set loop, at entry.** In the block that holds the form, one statement is
     `for (<r>, <c>) in <Q>.iter().zip(&mut <K>) { *<c> = <r><path>.available_at_entry()[.min(<cap>)]; }`
     (`&<Q>` for `<Q>.iter()` and `<K>.iter_mut()` for `&mut <K>` are the same form; `<cap>` as in
     D6). Its body is exactly that one statement: a second statement, even one that only adjusts
     `*<c>`, refuses the form. `<r>` and `<c>` are the pattern's bindings, resolved by D2.
     **Pairing.** A pass loop's set loop is the last statement before it, in source order, among
     the statements of the blocks that enclose the pass loop, that has this form for `<Q>` and
     `<K>` spelled the same; condition 1 then requires the same bindings. An earlier set loop is
     not part of the form. A set loop for the same `<K>` after that one, inside the window of
     condition 6, is a mention that condition 6 refuses.
  3. **The form's loops.** The pop's innermost loop is the **pass loop**. It and every loop
     between it and the set loop's block are later statements of that block, or nested in them;
     none of them encloses the set loop. Every loop that encloses the set loop is an outer loop of
     the drain (B1b-D1, and #1418's D1).
  4. **The pass loop** is `for <pat> in <Q>.iter_mut().zip(..)..` with one or more `.zip(&mut
     <fields>)` steps, one of which is `.zip(&mut <K>)`. Its iterator equals the set loop's for
     `<Q>` and `<K>` term for term, with `iter` read as `iter_mut` and `&` as `&mut`, and it has
     no adapter other than `.zip` (no `.enumerate()`, `.skip(..)`, `.rev()`), so element i of
     `<Q>` meets counter i.
     **Pattern positions.** With k `.zip` steps, `<pat>` must nest as the iterator does:
     `((..((<b0>, <b1>), <b2>)..), <bk>)`, each leaf a plain identifier binding. `<b0>` is the
     element of `<Q>`, and `<bj>` is the element of the j-th `.zip` step. `<r'>` is `<b0>` and
     `<c'>` is the leaf at the position of the `.zip(&mut <K>)` step, both resolved by D2. Any
     other pattern shape refuses the form.
  5. **The guard, the decrement and the pop.** The pass body's first statement is
     `if <cond> { continue; }`, where `<cond>` is `*<c'> == 0` or a `||` chain with `*<c'> == 0`
     as one operand. Its second statement is `*<c'> -= 1;`. Its third statement holds the pop,
     whose receiver is `<r'><path>`, the set loop's `<path>`, with `<r'>` resolved by D2 to the
     pass loop's binding. It is the only pop in the pass body. The pass body names `<c'>`
     nowhere else, and names `<r'>` only in the pop's receiver (so the counted queue cannot be
     swapped, moved or borrowed out of its place while the counter guards it).
  6. **The counters are written only there: a closed list.** The **window** runs from the end of
     the set loop to the end of the form's outermost loop. The **roots** are the root bindings of
     `<K>` and of `<Q>` (usually one binding), resolved by D2. Every mention, in the window, of an
     identifier that D2 resolves to a root, inside closures and macro token trees too, must be one
     of these (coordinator decision, 2026-10-06: condition 6 covers `<Q>` as well as `<K>`, with
     the case `per_queue_collection_swapped`; this is in scope for B1a because it closes the same
     fail-closed gap):
     1. **A field path that leaves `<K>` and `<Q>`.** A root followed by `.field` steps that differ
        from `<K>`'s, and from `<Q>`'s, at a step both have, so it is neither of them, nor a
        prefix of either, nor an extension of either. Field names are compared as identifiers.
        Its use does not matter, and it may stand inside a closure: `ready.meter_pending.iter()`,
        `ready.meter_loss_count = ..`, `&mut ready.spare`.
     2. **The pass loop's own header:** `<Q>.iter_mut()` and `&mut <K>` as the argument of
        condition 4's `.zip(&mut <K>)` step, in this form's one pass loop.
     3. **A shared read of `<Q>`:** `<Q>.get(..)`, `<Q>.iter()`, `<Q>.len()`, `<Q>.is_empty()`,
        `<Q>.first()`, `<Q>.last()` or `&<Q>`, inside a closure too (`ready.meters.get(index)` in
        the real poll's `.all(..)`). None of them can reach a queue mutably, and `try_pop` needs
        `&mut`.
     4. **A read loop:** `for <x> in <K>.iter()` or `for <x> in &<K>`, whose body names `<x>` only
        as `*<x>`, never as the left operand of an assignment or compound assignment, never under
        `&mut`, and never in a closure or a macro token tree (the pass bound `1 + Σ remaining`).
     5. **The terminal call.** A method-call statement whose receiver is a root or a prefix of
        `<K>` or `<Q>` (`ready.reset_meter_delivery(false);`), standing directly in a block of the
        function body, not inside a closure, a macro token tree or a nested `fn`, where the rest
        of that block holds no loop, no pop, no closure and no macro call, and its last statement
        is `return ..;` or a `break` out of the form's outermost loop. The meter poll's
        producer-reset branch has that shape (`hosts/host-web/src/lib.rs:3554-3558` on
        `6d28a80ec`).

     Every other mention refuses the form. Among them: the bare root as a value, `&root`,
     `&mut *root` or any reborrow into a local, a destructure (`let Ready { .. } = &mut *root`), a
     move; `<K>` or a prefix of it in any other position (an assignment, `.fill(..)`,
     `.iter_mut()`, `.len()`, a second set loop, `&mut <K>` anywhere but item 2); `<Q>` in any
     other position (`&mut <Q>`, `mem::swap(&mut <Q>, ..)`, an assignment, `.get_mut(..)`,
     `.iter_mut()` outside item 2); a call that takes a root or a prefix of `<K>` or `<Q>` other
     than item 5 (a function call, an argument, a call in a closure even if a `return` follows
     it there); and any mention in a macro token tree,
     except inside `debug_assert!`, whose arguments are parsed as expressions and held to this
     same list (as D7). One pass loop per set loop: a second pass loop on the same counters names
     `&mut <K>` outside item 2 and is refused.

  Then each queue pops at most once per decrement, and its counter starts at its own count at
  entry and only falls, so it pops at most that count over the whole form, whatever another
  thread does and however many passes run. `poll_meters` (stream H's *Bound the browser meter
  poll by each queue's count at entry*, D2) has this shape: checked against D2's code, it names
  `ready` in the window only as item-1 field paths (`ready.meter_pending` in the pass header and
  the checks, `ready.meter_loss_count`, `ready.meter_generation`, `ready.meter_header`,
  `ready.meter_snapshot_generation`), item 2 (`ready.meters.iter_mut()` and
  `&mut ready.meter_remaining`), item 3 (`ready.meters.get(index)` in the `.all(..)` closure),
  item 4 (the `passes` loop) and item 5 (`ready.reset_meter_delivery(false);`, then
  `ready.meter_loss_count = 1;`,
  `self.meter_activation_sample = ..;`, `return 0;`), with no macro in the window; and it names
  `meter` only in `meter.consumer.try_pop()`. The B1b-D1 finite forms must accept its pass loop
  (B1b-D1 takes one or more `.zip(..)` steps).
- **D9. Outer loops are not judged in this slice.** Any loop around the innermost bounding loop,
  and any loop of a D8b form other than its pass loop, is accepted here, and B1b adds the
  finite-outer-loop rule. This is a temporary gap in the tool only. It is safe: no CI run sees a
  commit between A1 and C2 (the batch pushes once), each slice's local gates run the awk, which
  refuses those shapes, B1b precedes C2 in the same push, and no slice reaches `main` without C2.
  The module documentation says so until B1b.

### The cases

- **D10. Ported drain cases.** Run every case of `marked-unbounded-try-pop-drain` and of the loop at
  `:1387-1402` (take the list from the file) through this slice's tool.
  - Commit, in A1's case format (A1-D9), every case that this slice refuses with {drain class}.
    Each expects {drain class}, except:
    - `open_tail_last_file` expects {misordered realtime policy markers, the file floor, the region
      floor}. It has no drain class: after the pairing finding its file has no regions (A1-D6).
    - `region_ends_in_comment` expects {malformed realtime policy marker} (A1-D6).
  - Every case that this slice accepts depends on a B1b rule. List each in the Evidence with that
    rule; B1b commits exactly that list. Expected (not binding): the outer-loop cases
    (`bounded_drain_inside_outer_loop`, `loop_inside_bounded_for`, `outer_*`,
    `paren_open_range_outer`, `wrapped_outer_while`, `second_loop_in_bounded_region`), the
    attribute and item cases (`count_under_cfg`, `count_shadowed_by_block_static`) and the
    constructor cases (`from_fn`, `bare_from_fn`, `repeat_with`, `array_from_fn_pop`).
  - No case puts a marker at a position that A2-D1 refuses (the second review checked all 56
    bodies). If the implementer finds one, add its class and record it.
- **D11. New cases.**
  - `control.r#try_pop()` in an unbounded loop (refused);
  - a count binding above BEGIN with its loop inside the region (refused, D1);
  - a pop inside a region in a `macro_rules!` body (refused, D1);
  - a pop in a nested `fn` inside an unbounded loop (refused, D3);
  - **D8b, accepted** (each red if the port refuses a per-queue form that bounds every queue):
    - `per_queue_meter_poll`, the meter poll's twin: the set loop over `self.lanes` and
      `self.remaining`, the `passes` sum, `for _ in 0..passes` around a pass loop over
      `self.lanes.iter_mut().zip(&mut self.slots).zip(&mut self.remaining)` whose guard is
      `if slot.is_some() || *left == 0 { continue; }`, and checks after it that read
      `self.slots` and `self.lanes.get(index)` inside a closure (as the real poll's `.all(..)`
      reads `ready.meters.get(index)`), write
      another field of `self`, `continue`, `break`, or call `self.reset_delivery()` and then
      `return`;
    - `per_queue_single_pass`: one pass loop, no outer loop, guard `if *left == 0 { continue; }`.

    The fourth-review fold's `per_queue_sibling_passes` is dropped (fifth review, NIT-5): no real
    drain has two pass loops on one set of counters, and under condition 6 the second pass loop's
    `&mut <K>` is outside item 2, so the shape is refused. Over-refusal is acceptable.
  - **D8b, refused (drain class)**. Each case breaks exactly one condition, and the defect it
    catches is named:
    - `per_queue_root_rebound` (condition 1): `fn drain(ready: &mut Ready, spare: &mut Ready)`;
      a correct set loop on `ready`, then `let ready = spare;`, then the sum and pass loops on
      `ready`. The roots are spelled alike but D2 binds them to different bindings. Defect: the
      set loop's and the pass loop's roots compared by spelling;
    - `per_queue_count_not_from_entry` (condition 2): the set loop writes
      `*left = lane.consumer.capacity();`. Defect: a count not read at entry;
    - `per_queue_set_loop_adjusts_count` (condition 2): the set loop's body is
      `*left = lane.consumer.available_at_entry(); *left += 4;`. Defect: a set loop judged by its
      first statement only;
    - `per_queue_counters_misaligned` (condition 4): the pass zips
      `self.remaining.iter_mut().skip(1)`. Defect: queue i guarded by counter i+1;
    - `per_queue_pop_from_wrong_zip_position` (condition 4): the set loop is over `self.lanes`;
      the pass loop is `for ((lane, other), left) in self.lanes.iter_mut().zip(&mut self.others).zip(&mut self.remaining)`,
      and the pop is `other.consumer.try_pop()`. Defect: any binding of the pass pattern taken as
      `<r'>`, so a queue from another collection pops under `<Q>`'s counter;
    - `per_queue_pop_other_queue` (condition 5): the counters are set from `lane.consumer`, and
      the guarded pop is `lane.spare.try_pop()`. Defect: a pop guarded by another queue's count;
    - `per_queue_no_decrement` (condition 5): the guard with no `*left -= 1;`. Defect: a count
      that never decrements bounds nothing;
    - `per_queue_two_pops_one_decrement` (condition 5): two pops of `lane.consumer` after one
      decrement. Defect: more pops than decrements;
    - `per_queue_receiver_rebound` (condition 5): the third statement is a block,
      `{ let lane = &mut self.spare; if let Ok(r) = lane.consumer.try_pop() { .. } }`. The
      receiver is spelled `lane.consumer`, but D2 binds it to the block's `let`. Defect:
      receivers compared by spelling, not by binding;
    - `per_queue_element_swapped_with_spare` (condition 5): after the pop, the pass body holds
      `core::mem::swap(&mut lane.consumer, &mut self.spare);` (fifth review, MINOR-2, FP-3). The
      spare queue then pops records not counted at entry. Defect: `<r'>` checked only in the pop's
      receiver, so the counted place can be replaced under the guard;
    - `per_queue_count_reread_in_pass` (condition 6): a correct form, plus a second set loop
      `for (lane, left) in self.lanes.iter().zip(&mut self.remaining) { *left = lane.consumer.available_at_entry(); }`
      after the pass loop, inside the outer loop, so each pass re-reads every count. Only
      condition 6 refuses it: the pass body is untouched, and the pairing (condition 2) pairs the
      pass loop with the first set loop. Defect: the window cut short at the end of the pass loop,
      which lets a count chase the producer;
    - `per_queue_collection_swapped` (condition 6): a correct form, plus
      `core::mem::swap(&mut self.lanes, &mut self.spare_lanes);` between the set loop and the
      pass loop. The pass then pops queues whose counts were never read. Defect: item 1 read as
      "any path that leaves `<K>`", so `<Q>` can be replaced in the window;
    - `per_queue_counter_written_by_call` (condition 6): `self.refill_counters();` (a `&mut self`
      method) between the set loop and the pass loop, with no `return` after it. Defect: the
      terminal-call exemption granted without its tail (no loop, no pop, ends in `return` or
      `break`) checked;

    The next three take the fifth review's shapes (`fn ..(ready: &mut Ready)`, with
    `Ready::refill_counters` a `&mut self` method that rewrites the counters):

    - `per_queue_counters_refilled_in_closure` (condition 6): after the pass loop, inside the
      outer loop, `let mut refill = || -> u32 { ready.refill_counters(); return 0; }; refill();`
      (fifth review, MAJOR-1, FP-1). The `return` leaves the closure, not the form. Defect: the
      terminal-call exemption applied inside a closure;
    - `per_queue_root_reborrowed` (condition 6): after the pass loop,
      `let me = &mut *ready; for left in me.remaining.iter_mut() { *left = 8; }` (FP-2). Defect: a
      reborrowed root not read as a mention of `<K>`;
    - `per_queue_counters_written_by_macro` (condition 6): a local
      `macro_rules! refill { ($r:expr) => { for left in $r.remaining.iter_mut() { *left = 8; } }; }`
      and `refill!(ready);` after the pass loop (FP-4). Defect: macro token trees skipped by
      condition 6;
  - an otherwise valid counted drain inside an `async` block. It expects {forbidden-body, drain}:
    the `async` keyword is forbidden (A2-D2), and `Expr::Async` on the pop's path refuses the
    drain (D4).

## Authorized paths

- `tools/realtime-policy/**`
- This spec

## Non-goals

- B1b's rules and its marker on `drain_controls`; B2a's refusals; B2b-1's and B2b-2's rules.
- #1418's queue-selection rule and #1426's receiver rule.
- Any change to a drain in production code. If a ported rule refuses a real drain, stop and
  report it.

## Hazards

- **Re-measure at implementation time.** The region and file counts and which real drains still
  pop. Each later slice that lands after another re-measures the floors (the realtime-policy floors
  row in STREAMS).
- **Both gates run until C2.**
- **Over-refusal is acceptable, a false pass is not.** D9's gap is the one stated exception, and
  the awk covers it until B1b.
- **What D8b condition 6 leaves to the compiler.** The list reads mentions of the roots of
  `<K>` and `<Q>` only. When a root is a borrow taken from another binding before the set loop (the meter poll's
  `ready`, from `self.ready.as_mut()`), a call on that other binding (`self.refill()`) could reach
  the counters, but the borrow checker refuses it while the root is still used in a later pass,
  and an alias made with `unsafe` cannot exist outside the unsafe allowlist's files
  (`hosts/host-web/src/lib.rs` is not on it; `ffi.rs` is). A form that reads the counters through
  a raw pointer is not D8b's form and is refused.

## Objective gates

1. **The real tree.** `cargo run --locked --release -q -p realtime-policy` prints the same counts
   as `bash scripts/check-realtime-policy.sh` on the same tree.
2. **The cases.** `cargo test --locked -p realtime-policy` passes, with every D10 and D11 case.
3. **Parity (PR evidence).** A throwaway script outside the repository runs `main`'s awk gate
   (gawk only: the probe TSV shows gawk and mawk equal on every probe) and the tool on each ported
   case. The verdicts agree on every case D10 commits, except the two D10 class changes. Every
   other ported case is on D10's deferred list. Name each difference.
4. **Mutations (PR evidence).** Apply each alone. Each turns red the named case, and the real
   tree still passes:
   - accept any mention of the count: `count_in_user_macro` and `inflated_count`;
   - ignore pops in headers: `wrapped_while_let` and `path_pop`;
   - resolve only `let` bindings (D2): `pattern_rebound_count` and
     `count_rebound_by_for_pattern`;
   - read nodes outside the region (D1): the binding-above-BEGIN case;
   - compare `try_pop` without stripping `r#`: the `r#try_pop` case;
   - accept `Expr::Async` (D4): the async case loses its drain class;
   - treat a nested `fn` as a new scope (D3): the nested-`fn` case passes;
   - D8b, condition 1: the set loop's and the pass loop's roots compared by spelling:
     `per_queue_root_rebound`;
   - D8b, condition 2: any set-loop initializer accepted: `per_queue_count_not_from_entry`;
   - D8b, condition 2: only the set loop's first statement checked:
     `per_queue_set_loop_adjusts_count`;
   - D8b, condition 4: adapters in the zip headers ignored: `per_queue_counters_misaligned`;
   - D8b, condition 4: any binding of the pass pattern accepted as `<r'>`:
     `per_queue_pop_from_wrong_zip_position`;
   - D8b, condition 5: the pop's receiver path not compared with the set loop's:
     `per_queue_pop_other_queue`;
   - D8b, condition 5: the decrement not required: `per_queue_no_decrement`;
   - D8b, condition 5: more than one pop per pass body accepted:
     `per_queue_two_pops_one_decrement`;
   - D8b, condition 5: receivers compared by spelling, without the resolver:
     `per_queue_receiver_rebound`;
   - D8b, condition 5: other mentions of `<r'>` in the pass body not checked:
     `per_queue_element_swapped_with_spare`;
   - D8b, condition 6: the window ends at the end of the pass loop, not of the outermost loop:
     `per_queue_count_reread_in_pass`;
   - D8b, condition 6: item 1 checks only `<K>`, not `<Q>`: `per_queue_collection_swapped`;
   - D8b, condition 6: the terminal-call exemption granted without its tail checked:
     `per_queue_counter_written_by_call`;
   - D8b, condition 6: the terminal-call exemption applied inside a closure:
     `per_queue_counters_refilled_in_closure`;
   - D8b, condition 6: a bare or reborrowed root (`&mut *root`) not read as a mention:
     `per_queue_root_reborrowed`;
   - D8b, condition 6: macro token trees not read: `per_queue_counters_written_by_macro`;
   - D8b, condition 6: items 1 and 3 refused inside a closure (over-refusal):
     `per_queue_meter_poll` is refused (`poll_meters` is not marked until the #1448 guard
     commit, so the real tree cannot show it here);
   - D8b removed (per-queue pops judged by D8 alone): the two accepted D8b cases are refused.

   If a named case is on D10's deferred list, name another committed case that the mutation turns
   red, or move the mutation to B1b.
5. **Run time (PR evidence).** Record the tool's time on the real tree, and on one synthetic
   marked file with a 3000-line loop body holding 300 counted drains.
6. **Shipped artifacts unchanged.** As A1's gate 6.
7. `cargo fmt --all -- --check`,
   `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`,
   `bash scripts/check-realtime-policy.sh` and `bash scripts/check-workspace-policy.sh` exit 0.

*Test value.*
- The ported cases keep their claims. Each is red if the visitor misses the pop, binding, read or
  innermost loop in the position the case holds.
- Ported lexer-era cases (`brace_in_*`, `raw_c_string_literals`, `wrapped_*`) are committed as a
  regression corpus. Root's ruling of 2026-10-05 is the authority for them.
- The D11 cases are red on, respectively: an `r#` comparison; reading outside the region; a
  skipped macro region; a nested item treated as a new scope; an allow-list that admits `async`.
- The D8b refusals are each red on the defect named with the case, and each breaks one condition
  only, so conditions 1, 2, 4, 5 and 6 each have a case of their own (condition 3's is #1418's
  `per_queue_inside_outer_loop`: B1a does not judge outer loops, D9): roots compared by spelling (1); a count not read at
  entry, or adjusted after it is read (2); misaligned counters, or a pop taken from the wrong zip
  position (4); a pop guarded by another queue's count, a count that never decrements, two pops per
  decrement, receivers compared by spelling, or the counted queue swapped under the guard (5); a
  count re-read each pass, the queues' collection replaced, or the counters raised by a call, a
  closure, a reborrowed root or a macro (6). The D8b acceptances are red if the port refuses the
  meter poll's shape, its closure reads included, or the single-pass form.

## Evidence

- Gates 1-7 output. Gate 3's table. Gate 4's runs. D10's deferred list, with the B1b rule for each.
- Which real drains still pop at implementation time.

## Dependencies

- After (same stream): A2.
- After (other streams): stream B batch 1 (#1309, #1343, #1314, #1311, #1348), through A1.
- Not ordered against #1312, #1346, #1347 and #1345 (root's ruling, 2026-10-05).
- Before: B1b.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused. The cases are the tool's own input.
- Attempt budget: three attempts, one adversarial verdict each.
- Size: half a day.

## Attempt record (attempt 1, J batch branch `codex/d15-stream-j3`)

Implemented on `394a85c6d` (A2 with its verdict follow-ups). No production code changed; no `Cargo.toml` or
`Cargo.lock` change. Edits: `tools/realtime-policy/**` and this spec.

**What is where.**
- `src/boundary.rs`: `Boundaries` (the file's leaves, container layout and the innermost container of every
  marker, built once and shared by the boundary check and the drain rule), `RegionNodes` (the region nodes, see
  the first deviation) and `words_in` (identifier tokens in a range).
- `src/drain/tree.rs` (D4, D3): the allow-list as a tree. `Node` is a reference into the parsed file;
  `Tree::children` lists the children of a node that D4 admits and the slot (`Via`) each one fills;
  `Tree::descend(pos)` follows them to a source position. A node outside the list has no entry, so a pop below it
  is never reached. Extents are cached.
- `src/drain/scope.rs` (D2): the one resolver, `nearest(steps, name)`. It reads a path of steps upward: the
  statements of a block before the one the path goes through, a `for` or closure pattern for its body, the `let`
  chain of an `if` or `while` for its condition and body, an arm's pattern, and a function's parameters, where the
  search ends.
- `src/drain/facts.rs` (D5): the pops of a file (method call, path expression, identifier in a macro's tokens;
  `r#` stripped), the uses of every single-segment name (a path expression, or an identifier in a macro's tokens),
  and the `debug_assert!` arguments parsed as expressions (D7).
- `src/drain/shapes.rs` (D6): the entry-count grammar, `<cap>`, `<recv>`, places (`self.lanes`).
- `src/drain/count.rs` (D6, D7, D8): `for P in 0..<count>`, `for P in 0..<recv>.available_at_entry()`,
  `while <count> != 0 | > 0`, the entry binding and its one alias, the reads between binding and loop.
- `src/drain/per_queue/mod.rs`, `window.rs` (D8b): the set loop, the pass loop, pairing and conditions 1, 2, 4, 5;
  condition 6's closed list of five.
- `src/drain/mod.rs` (D1): the judgment of one pop, and the module documentation of D9's gap.
- New class `UnboundedTryPop`, "marked realtime unbounded try_pop drain (bound it with available_at_entry)", the
  awk's message; one finding per refused line.
- Tests: `tests/policy/drain/` (`ported.rs` the 43 refused ported cases as data, `mod.rs` their test and the row
  helper, `forms.rs`, `per_queue.rs`). 79 tests became 113.

**Deviations from the spec's text and numbers.**
1. **Region nodes (coordinator note).** A2 built no node set, so B1a builds it: `Boundaries::region_nodes` takes,
   from the container layout, the children of every non-macro container whose extent (first token after outer
   attributes to the end) lies wholly between the markers, and keeps the maximal ones. A region with a marker in a
   macro token tree has none. The "top" of a pop's path is the first step whose extent is one of them.
2. **The allow-list is applied from the file down.** D4 constrains the path from the pop to its top. The tree is
   walked from the file, so every node above the top must be admitted too: a region inside an `async` block, a
   `const` block or an item `const` is refused. Stricter, fail closed.
3. **The innermost loop is the innermost loop whose body holds the pop**, and a pop in the header of any loop on
   its path is refused separately (D8). With one choice for both, the header rule would never decide anything.
4. **Mentions are uses, not tokens.** D7's "every mention of `<count>`" is read as a use that D2 resolves to the
   count's binding: a path expression, or an identifier in a macro's tokens. A binder in a pattern or a field
   or method name is no use. So a mention in a closed inner block that shadows the count passes
   (`form-shadow-in-a-closed-block`; the awk refuses it), while the loop header's "names `<count>` exactly once"
   stays a token count (so `for available in 0..available` is refused, as in the awk).
5. **A pop in a `macro_rules!` body, or in any item the list omits (`const`, `static`, an `impl` constant), is
   refused**; a region whose markers sit in a macro token tree has no region node and is refused. Three guards
   overlap there (`is_tokens`, the missing top, the unreached pop); each alone leaves the cases red-free, so the
   macro-region cases turn red only under the combined mutation below.
6. **D10's expected deferred list was a guess; the measured one is 14.** `second_loop_in_bounded_region` and
   `loop_inside_bounded_for` are refused here (their pops sit in an unbounded `loop`), so they are committed.
   `count_from_another_function` is ported with a changed place for the earlier binding (a new function
   `drain_fader_prepare` before the parameter-taking `drain_fader_tail`, both in the fader region), because the
   edit kinds cannot insert after a function; the claim (a binding in another function, same region) is the same.
7. **Real tree.** The spec's line numbers are `main`'s; on this tree the marked pops are
   `crates/builtins-compiler/src/lib.rs:1084` and `:1128`, `crates/effect-contract/src/live.rs:366` (the path
   value `Consumer::try_pop` under the alias `remaining`), `crates/graph/src/runtime.rs:900`; `spsc.rs:441` is the
   `fn try_pop` definition (not a pop); `plan_exchange.rs:377` is gone (#1343). All four pops pass. The cell
   slices (#1312, #1346, #1347, #1345) have not landed on this branch. Floors unchanged: 93 regions, 26 files.
8. Added beyond the spec: `per_queue_param_root` (the roots are a typed parameter, not `self`), which pins that
   parameters resolve as bindings; D8b's `two_pops_one_decrement` puts both pops in the third statement (a pop
   in a later statement is refused by "the third statement holds the pop" and would not test the pop count).
9. A set loop is form A only (`<r><path>.available_at_entry()`, no `map_or`), as D8b condition 2 spells it.

**Gates.**
1. `cargo run --locked --release -q -p realtime-policy` prints `realtime policy: ok (93 marked regions in 26
   files)`; `bash scripts/check-realtime-policy.sh` prints the same.
2. `cargo test --locked -p realtime-policy`: 113 passed.
3. Parity (gawk 5.2.1; the tool's release binary; every tree the harness builds, 327, kept by a throwaway patch
   of `Tree::drop`, reverted): 259 agree, 68 differ. 21 are A1's and 37 are A2's (unchanged lists). The 10 of
   this slice are all deliberate:
   - awk pass, tool fail: `allow-list-const-block`, `allow-list-const-item`, `allow-list-macro-item` (D4:
     nodes off the list, and `macro_rules!` bodies), `loop-around-the-region` (D1: a loop that encloses the
     region), `pop-in-a-macro-body-region`, `pop-in-a-macro-call-region` (D1: a region in a macro token tree).
   - awk fail, tool pass: `per-queue-meter-poll`, `per-queue-param-root`, `per-queue-single-pass` (D8b: the awk
     has no per-queue form), `form-shadow-in-a-closed-block` (deviation 4).
   All 57 ported cases (a separate run, each built from the script's own text): both gates refuse the 43
   committed cases, including the two class changes (`open-tail-last-file`: misordered markers and both floors;
   `region-ends-in-comment`: malformed marker). The other 14 pass the tool and fail the awk: they depend on B1b.
   **D10's deferred list, for B1b:** outer-loop rule (finite outer loops; the outer `loop`, `while`, `(0..)`,
   `iter::repeat`, an iterator parameter, a rebound slice parameter, a zip then `cycle`):
   `bounded_drain_inside_outer_loop`, `wrapped_outer_while`, `paren_open_range_outer`, `outer_repeat`,
   `outer_over_iterator_parameter`, `outer_over_rebound_slice_parameter`, `outer_zip_then_cycle`; outer loop
   and item rule (a block `const` shadows a slice parameter): `outer_over_slice_parameter_shadowed_by_block_const`;
   attribute rule (a `cfg` on the binding): `count_under_cfg`; item rule (a block `static` shadows the
   count): `count_shadowed_by_block_static`; constructor rule (`from_fn`, `repeat_with`, a pop in an
   `array::from_fn` closure): `from_fn`, `bare_from_fn`, `repeat_with`, `array_from_fn_pop`.
4. Mutations, each applied alone to the code, then reverted (`cargo test` after each; 113 green on the revert;
   the real tree passes under every mutation named here, but under the ones marked *). "red" lists the cases
   that went red; the spec's named case first.
   - Accept any mention of the count (the reads rule off): `count_in_user_macro` red, with `count-in-arithmetic`,
     `count-passed-to-a-function`, `reassigned_count`, `count-in-raw-identifier-macro` and three more. The
     other half of the spec's pair, `inflated_count`, is a D6 form, not a mention: accept any entry initializer:
     `inflated_count` red, with `shadowed_count`, `entry_name_in_binding`, `count_raised_by_max`,
     `bound_not_at_entry`.
   - Pops in headers ignored: `wrapped_while_let`, `path_pop`, `header_past_closed_body` and
     `marked-unbounded-try-pop-drain` red.
   - Resolve only `let` bindings: patterns of `let` only: `pattern_rebound_count` red (with `binder-tuple-let`,
     `-if-let`, `-match-arm`, `-while-let`, `count_rebound_by_for_pattern`, and the two accepted per-queue twins);
     binders other than `let` ignored: `count_rebound_by_for_pattern` red (with `binder-closure-parameter`,
     `-if-let`, `-while-let` and the twins).
   - Read nodes outside the region: `binding-above-begin` red (alone).
   - No `r#` strip on a method name: `raw-identifier-pop` red (alone).
   - Accept `Expr::Async`: `drain-in-async-block` red (alone). Skip the unreached-pop refusal: `drain-in-async-block`
     and `allow-list-const-block` red.
   - Nested `fn` as a new scope: `pop-in-a-nested-function-in-an-unbounded-loop` red (alone). Closure as a new
     scope: `pop-in-a-closure-in-an-unbounded-loop` red (alone).
   - D8b condition 1, roots by spelling: `per_queue_root_rebound` (alone). Condition 2, any set-loop initializer:
     `per_queue_count_not_from_entry` (with the twins, whose `.min(DEPTH)` the mutation breaks); first statement
     only: `per_queue_set_loop_adjusts_count` (alone). Condition 4, adapters in the zip headers ignored:
     `per_queue_counters_misaligned` (alone); any pass binding as the queue: `per_queue_pop_from_wrong_zip_position`
     (alone). Condition 5: receiver path not compared: `per_queue_pop_other_queue`; decrement not required:
     `per_queue_no_decrement`; more than one pop accepted (the pop count and the queue-mention clause both
     dropped, since either alone leaves it refused): `per_queue_two_pops_one_decrement`, with
     `per_queue_element_swapped_with_spare` and `per_queue_receiver_rebound`; receivers by spelling:
     `per_queue_receiver_rebound` (alone); the queue's other mentions unchecked: `per_queue_element_swapped_with_spare`
     (with `per_queue_receiver_rebound`). Condition 6: window ends at the pass loop: `per_queue_count_reread_in_pass`
     (with the closure, reborrow and macro cases, which stand after the pass loop); item 1 on `<K>` only:
     `per_queue_collection_swapped`; terminal call without its tail: `per_queue_counter_written_by_call`;
     terminal call in a closure: `per_queue_counters_refilled_in_closure`; a bare root not a mention:
     `per_queue_root_reborrowed`; macro tokens not read: `per_queue_counters_written_by_macro`; items 1 and 3
     refused inside a closure: `per_queue_meter_poll` (and `per_queue_param_root`, which has the same closure).
     D8b removed: `per_queue_meter_poll`, `per_queue_single_pass`, `per_queue_param_root`.
   - The macro-region cases (D11) turn red only when the three overlapping guards of deviation 5 are dropped
     together: `pop-in-a-macro-body-region`, `pop-in-a-macro-call-region`, `allow-list-*`, `drain-in-async-block`;
     each guard alone: no case red.
   - Mutations for the new rows (one red case each unless noted): header form refused
     (`form-header-entry-count`); `> 0` refused (`form-while-greater-than-zero`); a typed binding refused
     (`form-typed-count`, `form-alias-with-a-type`); `.min(<field>)` and `.min(<literal>)` refused (their rows);
     the function's outer attributes in the region node (`form-function-with-attributes`; the real tree fails
     too*); `!=`, `<`, `>`, `>=` dropped from the reads (`form-count-compared-before-the-loop`); macro pops
     refused outright (`form-pop-token-in-a-macro-in-a-counted-loop`); uses matched by name, not binding
     (`form-shadow-in-a-closed-block`); each refusal row of `a_count_that_the_rule_cannot_prove_is_refused` is red
     under the mutation that admits it (right operand, any binary operator, a call argument, `debug_assert!`
     arguments exempt, every `*assert` macro parsed, a method cap, the header token count, an inclusive range,
     `+= 1`, `-= 2`); collection loop counted bounded (`collection-loop-without-a-count`); each binder kind
     ignored in turn (`binder-match-arm`, `-if-let`, `-while-let`, `-closure-parameter`, `-tuple-let`); a
     typed parameter not a binding (`per_queue_param_root`); an `Expr::Const`, an item `const` and a `macro_rules!`
     item admitted (their `allow-list-*` rows); the loop that encloses the region ignored
     (`loop-around-the-region`); region nodes from the end container only (`region-spanning-an-impl`; the
     real tree fails too*); macro tokens not found as pops (`pop-token-in-a-macro-in-an-unbounded-loop` and the
     macro-region cases). Rows that the base tree already pins were removed after the sweep showed it (`.min(<path>)`,
     `map_or`, `==`, `<=`, `debug_assert!`): dropping those forms turns the base tree red.
5. Run time, release binary: real tree 1.00 s, 1.00 s, 1.01 s (A2: 1.02-1.04 s; the awk gate 0.53 s). The base
   tree plus one synthetic marked file (a `for lane in 0..4` loop of 3,007 lines holding 300 counted drains, the
   same count name 300 times, and again with 300 distinct names): 0.04 s each time (the awk: 0.43 s and 1.05 s).
   A stress run with every one of 566 source files of the workspace wrapped in markers: 2.9 s, exit 1 with 35
   drain findings in test files, no panic. Not tuned.
6. Shipped artifacts: `git diff --stat 394a85c6d..HEAD` outside `tools/realtime-policy` and `.github/ISSUE_SPECS`
   is empty; `cargo tree -e normal,features -p capi` names no `realtime-policy`; `cargo build --locked --release
   -p capi` built twice at the parent and twice at the change gives the same digests each time: `libcapi.a`
   `3431f546ec53477d...`, `libcapi.so` `0213cd820e2715de...` (A2's digests, taken at `394a85c6d`'s parent: the
   same). The `artifact-identity` job is CI-only.
7. `cargo fmt --all -- --check`, `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`,
   `check-workspace-policy.sh`, `check-bench-policy.sh`, `check-realtime-policy.sh`,
   `test-realtime-policy.sh`: all exit 0.

**Test value (the one-sentence answer, per new test).**
- `the_ported_drain_cases_are_refused`: red if the visitor misses the pop, binding, read or innermost loop in the
  position a ported case holds (the named mutations above, and the lexer-era cases as a regression corpus by
  root's ruling of 2026-10-05).
- `the_forms_that_d6_and_d8_state_pass`: red if a counted drain in a form D6 or D8 states (header entry count,
  `> 0`, typed, literal- or field-capped, an attributed function, `!=`/`<`/`>`/`>=` reads, a macro pop in a
  counted loop, a shadow in a closed block) is refused, which neither the base tree nor the real tree pins.
- `a_count_that_the_rule_cannot_prove_is_refused`: red if a count mention that is not a left comparison, an
  assertion other than `debug_assert!`, a method cap, an inclusive range or a `while` body not opening with
  `<count> -= 1;` is accepted.
- `a_loop_with_no_count_does_not_bound_a_pop`: red if a collection loop with no count bounds a pop.
- `every_kind_of_binder_takes_the_count`: red if a `match` arm, `if let`, `while let`, closure parameter or tuple
  `let` between the entry binding and the loop is not the count's nearest binding.
- `a_pop_below_a_node_outside_the_allow_list_is_refused`: red if the allow-list admits `Expr::Const`, an item
  `const` or a `macro_rules!` item.
- `a_counted_drain_in_an_async_block_is_refused_twice`: red if `Expr::Async` is walked or an unreached pop passes.
- `a_raw_identifier_pop_is_a_pop`: red if `r#try_pop` is compared without stripping `r#`.
- `a_pop_in_a_macro_or_a_closure_is_judged_by_the_loops_around_it`: red if macro-token pops are ignored or a
  closure hides the loop around it.
- `a_pop_in_a_nested_function_is_judged_by_the_loops_outside_it`: red if a nested `fn` starts a new loop scope.
- `a_count_binding_above_the_region_is_not_read`: red if the rule reads a node outside the pop's region.
- `a_loop_that_encloses_the_region_is_refused`: red if the loop around a region is ignored.
- `a_pop_in_a_region_inside_a_macro_body_is_refused`, `..._macro_call_is_refused`: red if a pop in tokens of a
  macro is judged by the loops inside those tokens (all three guards of deviation 5 together).
- `a_region_spanning_the_end_of_an_impl_reads_its_nodes`: red if the region nodes come from one container.
- `per_queue_*`: each refusal is red on the defect named with it in D11 (the mutation list above); the three
  acceptances are red if the meter poll's shape (its closure reads included), the single pass or a typed-parameter
  root is refused.

**Open items.**
- B1b: the 14 deferred cases (list above), the outer-loop, attribute, item and constructor rules, `drain_controls`.
  B1b's "region opened inside a loop's body" case already exists here (`loop-around-the-region`).
- D9's gap stands in the module documentation until B1b; the awk gate covers it.
- B2a replaces "closures and macros are transparent": `form-pop-token-in-a-macro-in-a-counted-loop` and
  `a_pop_in_a_macro_or_a_closure_is_judged_by_the_loops_around_it` are superseded then and are deleted in that
  change.
