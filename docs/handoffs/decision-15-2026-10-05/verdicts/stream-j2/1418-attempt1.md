FAIL

# #1418 attempt 1 verdict (decision-15 stream J batch 2)

Commit under review: `da878bd49` (branch `codex/d15-stream-j2`), diff `da878bd49^..da878bd49`.
Verifier: opus-xhigh, read-only. The commit was exported to `/tmp/claude-1002/v1418/tree`, and all
probes ran there.

Root already knows these two items. They are not counted here: (a) the spec's Problem text is
wrong about the fader and matrix drains; (b) the gate does not check that the pop's receiver is
the counted queue.

## MAJOR

### MAJOR-1: the index form passes loops that drain the same queue on more than one pass, and passes an infinite loop

`scripts/check-realtime-policy.sh:373-377` (`selects`, second form) accepts `<base>[<ident>]`
when the loop's pattern binds `<ident>` without `mut` and no binding inside the loop names
`<ident>` or `<base>`. Two properties are not checked:

1. **The index values can repeat.** Only a `0..<bound>` range, the index slot of `.enumerate()`
   and the parameter of an `array::from_fn` closure give a different index on each pass. Any other
   accepted iterable can give the same index again.
2. **The base can change between passes.** A method call or an assignment on the base is not a
   binding, so `binder` does not see it.

The full gate passes each drain below, with the same result under gawk, mawk and busybox awk.
Shapes 1, 2, 3 and 5 compile with rustc 2021 against a mock `Consumer` whose `try_pop` takes
`&mut self` (the real signature, `crates/engine/src/realtime/spsc.rs:440`). When run, each one pops
the same queue twice in one call. Shape 4 is identical to the compiled demo except for one
demonstration cap line.

| # | Shape (each wraps the usual `let available = <recv>.available_at_entry(); for _ in 0..available { <recv>.try_pop() .. }`) | Result |
|---|---|---|
| 1 | `fn f(order: &[usize], controls: &mut [Consumer<Record>]) { for &lane in order { controls[lane] .. } }` | accepted; `order = [0, 0]` drains queue 0 twice |
| 2 | `for &lane in &self.order { self.controls[lane] .. }` | accepted; same |
| 3 | `for (_, &lane) in self.order.iter().enumerate() { let control = &mut self.controls[lane]; .. }` | accepted; same |
| 4 | `let mut lanes = (0..2).cycle(); for lane in &mut lanes { controls[lane] .. }` (or `for lane in &mut self.lane_cycle`) | accepted; **infinite** loop that drains both queues forever |
| 5 | `for lane in 0..2 { controls[lane] ..; controls.swap(0, 1); }` | accepted; lane 1 drains the queue that lane 0 drained |
| 6 | `let mut controls = &mut all[1..]; for lane in 0..2 { controls[lane] ..; controls = &mut all[0..]; }` | accepted; drains `all[1]` twice |

This is the property the issue exists to enforce (D1: "accepted only if the queue drained
changes with that loop's pass"; Hazards: "Over-refusal is acceptable, a false pass is not. Where
D1's walk is unsure, refuse."). Shape 4 also reopens #1302 J3-2, which D4 cites: an infinite
iterator behind `&mut` still passes when its pattern binds an index. The gate's comment states
three things that the code does not check:
- `:120` "an invariant base": the code does not prove invariance (shapes 5 and 6);
- `:135-136` "each element, or one index of an unchanged collection, is a different queue": this is
  false for an index read from a collection (shapes 1 to 3), and the gate does not check "unchanged";
- `:156-158` "`&mut` of an iterator held in a field or local ... is infinite; the queue-selection
  walk refuses it around a drain, since its pattern selects no queue": this is false when the
  pattern binds an index (shape 4).

D3 does not cover this case. D3's argument is about types: borrowck and a consumer that is not
`Clone` make the *queue* elements of a `&mut` iteration distinct. A table of `usize` indices with
duplicates compiles. The gate's own pattern-root form is sound for the same reason (`try_pop`
takes `&mut self`, so the base cannot be changed while a `for control in controls.iter_mut()` runs).
Only the index form, whose base the loop does not borrow, has the gap.

D1's index bullet, as written, does not restrict the loop kind. Root may want to amend it. A
correct rule refuses at no cost in the real tree, because no real marked drain uses the index form
(the real drains are the three builtins-compiler pattern/let-else drains and `graph/src/runtime.rs:897`,
which has no outer loop). A sufficient fix:
- accept `<base>[<ident>]` only when `<ident>` is the lone pattern of a `0..<bound>` range, the
  index slot of a trailing `.enumerate()`, or an `array::from_fn` closure parameter;
- refuse when the loop body names the base root anywhere other than in that exact receiver
  spelling (this covers method calls, assignments and `&mut` borrows of the base);
- add one expected failure each for shapes 1, 4 and 5.

## MINOR

### MINOR-1: no valid `if let` case. The path that once caused unbounded recursion has no test

The gate's comment (`:124-125`) and D1 accept `if let Some(<root>) = ..` (`resolve`, `:396`). No
fixture case reaches that path:
- M6: refuse every `if let` header in `resolve`. The self-test stays green and the real tree passes.
- M17: reintroduce the self-match bug by removing `if (hstart[h] >= pos) continue` at `:430`. The
  self-test and the real tree stay green. A probe with one `if let Some(control) = control.as_mut() { drain }`
  then makes gawk fail with `fatal: ... cannot allocate memory` after 22 s under `ulimit -v 4000000`.
  Without a limit it uses all memory (the implementer measured about 35 GB).

The gate fails closed in both cases. But the bug would show only on the first real `if let` drain,
and on a 7 GB CI runner it shows as an out-of-memory kill. A valid case is needed (test value
below).

### MINOR-2: most of the walk's refusal clauses have no fixture case

Each mutation below, applied alone to the new gate, leaves `test-realtime-policy.sh` green and
the real tree passing:
- M4: drop `binder(b, ..) == ""` (base rebound inside the loop);
- M5: drop `x != b`;
- M8: drop `macro_named` (a macro may bind the name);
- M9: `patbound` ignores `mut`;
- M11: drop `item(x)` in `binder` (`static`/`const`/`use`);
- M13: drop `lpat !~ tw(b)` (base named by the pattern);
- M14: invert the "a pure `if` binds nothing" rule.

The implementer's Attempt record lists hand probes for most of these ("refuses these drains in a
pattern loop: ... struct `let`, closure parameter, `match` arm, `let control = &mut *fixed`,
`for mut control`, `rebind!(control)`, `controls[0]`, base rebound"). None was committed. Each
probe is one fixture case that turns red on its named defect. Without them, a later edit can drop
one of these soundness guards and no test fails. (M7, which drops the let-else `else` requirement,
is dead code for Rust that compiles, because a refutable `let` without `else` does not compile. It
needs no case.)

### MINOR-3: the headline states a bound the gate does not prove

`:80-82`: "per queue: ... so no queue is drained twice in one block". D6 asks for "per queue, at
most the records counted at block entry". Two counted drains of one queue in sequence, with no
loop between them (`let a = c.available_at_entry(); for _ in 0..a {..} let b = c.available_at_entry(); for _ in 0..b {..}`),
pass the gate. They pass at region level and inside a `for control in controls.iter_mut()`.
The second count is not taken at block entry. The total stays finite (at most twice the capacity),
so this is not the unbounded class. The comment must either state the bound per drain loop, or
list sequential re-drains under "These stay outside it".

## NIT

- **NIT-1: run time grows faster than linearly.** It is not unbounded. `resolve` recursion
  terminates: each step looks only at bindings and headers that start strictly before the current
  offset. But each `binder` call runs `macro_named`, which scans backward from every mention, and
  `resolve` calls `binder` once per link in the rebinding chain. Measured with gawk (old gate in
  brackets):

  | Probe | New gate | Old gate |
  |---|---|---|
  | 1000-line loop body with 3000 mentions | 18.5 s | 0.36 s |
  | 3000-line loop body | more than 120 s | not run |
  | chain of 100 `let Some(control) = control.as_mut() else` | 8.4 s | 0.38 s |
  | chain of 300 | more than 100 s | not run |

  The real tree takes 0.6 s (gawk), 0.5 s (mawk) and 2.8 s (busybox). No realistic input is at
  risk. A follow-up could compute macro spans once per region.
- **NIT-2: one new fixture region adds almost no test value.** The `drain_controls`-shaped
  region (`scripts/test-realtime-policy.sh:458`) goes red on the same walk defects as the existing
  `drain_fader_records` (`:436`): M3 and M16 (drop `.as_mut()` acceptance) are red on both, at
  `:10` and `:39`. I could not find a walk defect that reaches it alone. Its remaining value is the
  90-region floor and mirroring the real region. Spec D4 asked for it, so this is for root, not the
  implementer.
- **NIT-3: over-refusals a future drain may meet.** All are acceptable under the Hazard and are
  listed only for awareness:
  - `let Some(control) = self.controls[lane].as_mut() else`: D1 allows `.as_mut()` only after a
    pattern-bound identifier;
  - `if ready() {} else if let Some(control) = control.as_mut() {..}`;
  - a bank × lane nest whose inner loop is a range (`for bank in &mut self.banks { for lane in 0..N { bank.controls[lane] .. } }`):
    a range sets the next receiver to "".

## Test value (new cases)

- `drain-outer-usize-max`: red if a `0..<path>` outer loop is accepted for being finite, with a
  receiver bound before the loop (M2). Each of the next four cases is the only case for its own
  `classify` branch.
- `drain-outer-capacity`: same defect, `0..<method path>` bound (M2).
- `drain-outer-unit-array-parameter`: same defect through the slice/array-parameter branch (M2).
- `drain-outer-borrowed-open-range`: same defect through the `&mut <fields>` branch (M2). It
  covers only patterns that bind nothing (see MAJOR-1 shape 4).
- `drain-outer-counted-loop`: same defect when the outer loop is itself a counted `0..<count>`
  drain loop (M2).
- `drain-outer-index-expression`: red if any index that contains a pattern identifier counts as
  selecting (M1, reproduced: the only case red).
- `drain-same-queue-lane-pairs`: red if an `array::from_fn` closure parameter counts as selecting
  when the receiver never uses it (M2).
- Valid `drain_lane_pairs` (`controls[lane]` in `from_fn`): red if the `from_fn` closure parameter
  is not kept as the loop pattern (M15: only `crates/builtins/src/lib.rs:14` red).
- Valid `BankSet::drain_banks` (two levels): red if the walk stops after the innermost selecting
  loop (M10: only `:91` red).
- Valid `drain_controls`-shaped region: red if the let-else or `.as_mut()` step is dropped (M3,
  M16), but `drain_fader_records` already catches both (NIT-2).
- Missing (MINOR-1): a valid `for control in controls.iter_mut() { if let Some(control) = control.as_mut() { <drain> } }`
  would be red on M6 and on M17, which nothing reaches now.

## Gates run (from the export, `/tmp/claude-1002/v1418/tree`)

1. `bash scripts/check-realtime-policy.sh` prints `realtime policy: ok (90 marked regions in 25 files)`
   under gawk 5.2.1, mawk 1.3.4 and busybox awk (a `PATH` shim for each; `command -v awk` was
   checked).
2. `bash scripts/test-realtime-policy.sh` prints `realtime policy mutation tests: ok` under the same
   three awks.
3. Red on revert: I ran the parent's `check-realtime-policy.sh` with the new test script, changed
   to list unexpected passes instead of exiting. Exactly the seven D4 cases pass the old gate:
   `drain-outer-usize-max`, `-capacity`, `-unit-array-parameter`, `-borrowed-open-range`,
   `-index-expression`, `-counted-loop` and `drain-same-queue-lane-pairs`. The floor case
   `marked-region-count-floor` also passes, as expected, because the old floor is 89. This agrees
   with the Attempt record.
4. Spec mutations, reproduced:
   - M1 (index `[ident[^]]*]`): only `drain-outer-index-expression` passes unexpectedly; the real
     tree passes.
   - M2 (`return patbound` becomes `return 1`): the six cases above pass unexpectedly.
   - M3 (drop the `let` step): the valid fixture fails at `crates/builtins/src/lib.rs:44` and at
     builtins-compiler `:10` and `:39`; the real tree fails at
     `crates/builtins-compiler/src/lib.rs:475`, `:1078` and `:1122`.

   Extra mutations M4 to M17 are as reported above.
5. `bash scripts/check-workspace-policy.sh` prints `workspace policy: ok`.

Other checks:
- D5: the `crates/builtins-compiler/src/lib.rs` change is exactly the two marker comment lines
  around `drain_controls` (`:453`, `:508`).
- CI runs both scripts (`.github/workflows/qualification.yml:514-515`).
- No Rust code changed, so the cross-target, clippy and worklet chains do not apply.
- The acked-batch question does not apply: no queue or ack path changed.

Evidence kept: `/tmp/claude-1002/v1418/rs/probe.rs` (the Rust demonstration). The export, the
mutation copies and the probe fixtures were deleted after this verdict.
