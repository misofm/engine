FAIL

# #1418 attempt 2 verdict (decision-15 stream J batch 2)

Commit under review: `7caf972db` (branch `codex/d15-stream-j2`). Reviewed diff: `da878bd49..7caf972db`
for `scripts/check-realtime-policy.sh`, `scripts/test-realtime-policy.sh` and the spec. The commit
itself touches only these three files (authorized paths). The `crates/builtins-compiler/src/lib.rs`
changes in the range come from #1420 and do not touch the markers.
Verifier: opus-xhigh, read-only. The commit and attempt 1 were exported to
`/tmp/claude-1002/v1418b/{tree,base}`. All probes ran there, under `ulimit -v 4000000` and `timeout`.

Root rulings applied: (1) the narrowed index form implements D1's governing clause; (2) the
`drain_controls`-shaped fixture region is removed, as authorized; (3) run time is a note only, because
the real tree takes less than 10 s. Out of scope and not counted: pop receiver == counted receiver
(#1426); the Problem-text correction about the fader and matrix drains.

## Attempt-1 findings: status

- MAJOR-1 (index form): the six listed shapes are now refused, and each one has a committed case.
  But the walk still has false passes. See MAJOR-1 to MAJOR-3 below. MAJOR-3 (b) is a
  regression from attempt 1.
- MINOR-1 (no valid `if let` case): resolved. `drain_optional_controls` is red on K and on M17.
- MINOR-2 (refusal clauses with no case): M8, M11 and M14 now have cases (F, G, I). M5, M9 and M13 are
  redundant, as the record says. **M4 is not redundant**: its `macro_named(<base>)` part refused a
  macro call that names the base. `uses` does not (MAJOR-3 b).
- MINOR-3 (headline): resolved. The comment now lists sequential re-drains under "These stay
  outside it", which was one of the two options in the attempt-1 verdict.
- NIT-1 (run time): see NIT-1 below. Note only, as ruling 3 says.
- NIT-2: resolved by removal (ruling 2). NIT-3: the bank x lane nest is now accepted, correctly.

## MAJOR

### MAJOR-1: a `cfg`-gated `let` is read as the binding in scope. This lets the issue's headline pop-until-empty drain through

`scripts/check-realtime-policy.sh:442` (`resolve1`) and `:498` (`binder`) remove the attribute
prefix of a `let` and then accept the binding. They do not refuse a `cfg`-family attribute, as
`counted` does for the count binding at `:264` (#1302's `count_under_cfg` case). A binding that may
not be compiled is not "the one in scope" (D1). The full gate prints `realtime policy: ok` under
gawk, mawk and busybox for this shape. It compiles with rustc 1.97 (edition 2021):

```rust
fn cfg_let_outer(control: &mut Consumer<Record>, controls: &mut [Consumer<Record>]) {
    for lane in 0..usize::MAX {
        #[cfg(any())]
        let control = &mut controls[lane];
        let available = control.available_at_entry();
        if available == 0 { break; }
        for _ in 0..available {
            let Ok(record) = control.try_pop() else { break; };
            apply(record);
        }
    }
}
```

With the `let` compiled out, every pass drains the parameter `control` and reads its count again. The
loop stops only when the queue is empty: this is the shape in the spec's Problem section. Run with a
mock consumer, it pops queue 7 until it is empty. The same hole passes `let control = &mut controls[0];`
before a `0..2` loop (pops `[0, 0]`) and a `#[cfg(any())] let Some(queue) = control.as_mut() else`
over a parameter `queue` (pops `[9, 9, 9]`). This attribute is common in these drains: the real fader
and matrix drains carry `#[cfg(any(test, feature = "test-support"))]` inside their loops
(`crates/builtins-compiler/src/lib.rs:1081`, `:1125`). The fix is one line: in `resolve1`, refuse a binding
whose attribute prefix matches `cfg[A-Za-z0-9_]*`, as `:264` does. Add the case above as an expected
failure.

### MAJOR-2: a pattern root bound by a `.zip(..)` argument makes the next loop out check the wrong collection

`:438`: when the receiver's root is bound by the pattern, `NXT = itrecv[g]`. `itrecv` (`:358-361`) is
the receiver of the iterable, before any adapter. When the root comes from a `.zip(..)` argument,
the collection that yields the queue is the zip argument, not `itrecv`. The next loop out is then
accepted when it selects the zip *receiver*. Every pass of that loop drains the same zipped
collection again. The full gate accepts this under all three awks. It compiles, and it pops
`[0, 1, 0, 1]` (each queue once per bank):

```rust
impl Mixer {
    fn drain(&mut self) {
        for bank in &mut self.banks {
            for (gain, control) in bank.gains.iter().zip(self.controls.iter_mut()) {
                let available = control.available_at_entry();
                for _ in 0..available { let Ok(record) = control.try_pop() else { break; }; apply(record); }
            }
        }
    }
}
```

`zip(&mut self.controls)` and `.zip(..).enumerate()` (pattern `(lane, (gain, control))`) pass the
same way (`evidence/zipself.rs`). This shape is natural (per-bank gains zipped with one shared set of queues). Attempt 1 had
the same hole, so this is not a regression. A sufficient fix: when the loop's header holds a
`.zip(`, a pattern-root step sets `NXT = ""`, so that no loop outside it selects. No real drain
needs a zip inside a nest. Add one expected failure.

### MAJOR-3: `uses` accepts a mention as a receiver read when it only looks like one

`:470-476`: a mention of a checked root passes when the text after it starts with the spelling,
fields or plain indexes, and `.available_at_entry(`, `.try_pop(` or `.map_or(0,`. Two things that
follow such a start can still move the queue or change the base:

(a) **`map_or(0, <closure>)`**. The closure gets `&mut` of the queue. The full gate accepts this, and
it pops `[0, 0]`:

```rust
for (lane, control) in controls.iter_mut().enumerate() {        // controls: [Option<Consumer>; 2]
    control.as_mut().map_or(0, |c| { if lane == 1 { core::mem::swap(c, spare); } 0 });
    let Some(queue) = control.as_mut() else { continue; };
    <counted drain of queue>
    control.as_mut().map_or(0, |c| { if lane == 0 { core::mem::swap(c, spare); } 0 });
}
```

This is the committed `outer-element-swapped-with-spare` case, routed through `map_or`. The comment
at `:137` says that this move "is refused". The index form passes in the same way (`mapor_index_chain`,
pops `[0, 0]`). No case reaches the `map_or(0,` alternative: when I remove it from the pattern, the self-test
stays green and the real tree still passes. It exists only for a count of the form
`map_or(0, <Type>::available_at_entry)`.

(b) **A macro call whose argument looks like a receiver read.** In this example, `rotate!` is defined
outside the region as `($c:ident [ $n:literal ] . available_at_entry ( )) => { $c.swap(0, 1) }`:

```rust
for lane in 0..2 {
    rotate!(controls[0].available_at_entry());
    <counted drain of controls[lane]>
}
```

The new gate accepts this, and it pops `[1, 1]`. **Attempt 1 refused it** through
`binder(b, ..) == ""` (its `macro_named(<base>)`). The Attempt-2 record removes that clause as
redundant: "any rebinding, reassignment or macro use of a base or pattern-bound name is a mention
`uses` refuses". That statement is false for this shape. Restoring `binder(<base root>) == ""` (mutation R1)
keeps the self-test green and the real tree passing, and it refuses this shape. When the macro call
comes after the read (`rotate_after!(controls[lane].available_at_entry())`, which expands to
`if lane == 0 { controls.swap(0, 1) }`), both attempts accept it, and it pops `[0, 0]`.

A sufficient fix: accept `map_or` only as `map_or(0,<path>::available_at_entry)`, and refuse a mention
that is inside a macro call other than `debug_assert!` (`macro_named` over the loop body up to that
mention). Add expected failures for (a) and for (b) before and after the read. Correct the
Attempt-2 record's redundancy claim.

Taken together, the prototype fixes for MAJOR-1 to MAJOR-3 are four small edits
(`/tmp/claude-1002/v1418b/evidence/prototype-fixes.diff`). With them, the fixture behaves as expected
under gawk, mawk and busybox, the real tree passes, and every compiled shape above is refused, but
not MINOR-1.

## MINOR

### MINOR-1: a `macro_rules!` defined in the region can change the base with no mention in the loop body

```rust
fn drain(controls: &mut [Consumer<Record>; 2]) {
    macro_rules! rotate { () => { controls.swap(0, 1) }; }
    for lane in 0..2 { <counted drain of controls[lane]>; rotate!(); }
}
```

This compiles: a local macro sees the local `controls` from where it is defined. The gate accepts it,
and it pops `[0, 0]`. `uses` scans only the loop body, and `rotate!()` names nothing. The comment
(`:136-137`) says that a base changed between passes is refused. Pops in macros are a documented
limit, so this belongs to that family. Either refuse a macro call other than `debug_assert!` in the
body of a selecting loop, or list "a macro that changes the base or moves a queue" under "These stay
outside it". No real outer-loop drain calls a macro in its body (I checked `drain_controls`,
`drain_fader_controls` and `drain_matrix_controls`), so refusing it costs nothing today.

### MINOR-2: the 64-link cap is deeper than mawk's eval stack, so the three awks give different results

`:430`: `resolve` allows 64 links. mawk 1.3.4 stops with
`program limit exceeded: eval stack size=1024` (exit 2) at 36 links of `let control = &mut control;`
and at 55 links of `let Some(control) = control.as_mut() else { continue; };`. gawk 5.2.1 and busybox
accept both up to 63. The full gate on the valid fixture plus a 40-link chain prints `realtime policy: ok`
under gawk and busybox, and `realtime drain-bound scan failed (awk status 2)` under mawk. This
fails closed, and no real drain is near it. But the spec's Hazard requires the same result under all
three awks. With a cap of 2, every drain case and the valid fixture stay green (mutation cap2, run in the drain-case harness). A cap of 1 does not. A cap of 8
to 16, or an iterative walk, removes the gap.

## NIT

- **NIT-1: run time (ruling 3, note only).** Real tree: 0.53 s (gawk), 0.85 s (mawk), 1.78 s (busybox).
  Synthetic test: one `for lane in 0..2` body with 100 counted drains of `controls[lane]` (about 700 lines)
  takes 0.02 s on `main`'s gate, 56 s on attempt 1 and 38 s on attempt 2 (gawk; 28 s on mawk).
  300 drains take more than 300 s on both gawk and mawk. A 63-link chain takes 1.5 s. With the
  self-match bug put back (M17), the cap gives a clean refusal at once. Without the cap, gawk
  stops with an out-of-memory fatal error after 11 s.
- **NIT-2:** `scripts/test-realtime-policy.sh:1696-1697`: the comment "A pattern-bound element
  moved through a spare ..." is above `mutate_outer_index_base_rebound` (`:1699`). It belongs to
  `mutate_outer_element_swapped_with_spare` (`:1714`), which has no comment.
- **NIT-3:** `scripts/test-realtime-policy.sh:471`: "Three regions, as many as the real
  crates/builtins-compiler/src/lib.rs holds after #1418". The real file now holds four
  (`:453`, `:534`, `:1065`, `:1109`).

Probes that behaved correctly: `while let Some(control) = it.next()`, `iter_mut().rev()`,
`chunks_mut(1)`, a closure that drains, a closure parameter or an inner-block `let` that shadows the
queue, `mem::swap` or `mem::take` spelled with the base or the root, and `let c = &mut *controls; c.swap(..)`
inside the loop are all refused. Before the loop, that alias does not compile (E0499/E0502). A labelled
`continue 'lanes`, `split_at_mut` halves taken before the loop, `let control = &mut controls[lane]; let Some(control) = control.as_mut() else`
and a 60-level nest of distinctly named bank loops are accepted, correctly.

## Test value (new cases)

Each new case has a unique catch. I reproduced every claimed mutation (A to P, M1 to M3, M17) with a
harness that runs the gate's drain program on every fixture case.

- `drain-outer-index-table` and `drain-outer-index-table-field`: red if any pattern identifier counts as
  an index that changes each pass (A). Each is the only case for its iterable branch (slice
  parameter; `&<fields>`).
- `drain-outer-enumerated-index-table`: red if `enumidx` takes the element slot instead of the index
  slot (L2: the only unexpected pass).
- `drain-outer-borrowed-index-cycle`: red if the lone pattern of a loop over `&mut <iterator>` counts
  as an index (A). It is the only lone-index case over `&mut`.
- `drain-outer-borrowed-enumerated-cycle`: red if `.enumerate()` over `&mut` of an iterator gives an
  index (P: the only case).
- `drain-outer-index-base-swapped` and `-index-base-reassigned`: red if `uses` is dropped or always true
  (B, and the same with the close-time check removed). They cover a method call and an assignment on
  the base.
- `drain-outer-index-base-rebound`: red if the spelling lists are not restored after a failed
  pattern-root try (N: the only case).
- `drain-outer-element-swapped-with-spare`: red if `uses` does not check the pattern root (B). It is
  the only pattern-root move outside an `else` block.
- `drain-outer-element-moved-in-let-else`: red if the `else` block counts as part of the binding (C:
  the only case).
- `drain-outer-index-rebound`, `-index-rebound-by-macro` and `-index-shadowed-by-block-const`: red if
  `binder` is dropped from the index form (E). `macro_named` (F) and `item` (G) each turn only their own
  case red.
- `drain-outer-index-reassigned`: red if `lidx` accepts a `mut` pattern (H: the only case).
- `drain-outer-loop-around-unselected-base`: red if, after an index step, the next loop out may select
  the collection that the indexing loop iterates (D2: the only case).
- Valid `drain_bank_lanes`: red if `NXT` after an index step is the iterated collection (D) or if an
  unbound receiver is accepted (M2). `drain_gained_lanes`: red if `enumidx` is disabled (L).
  `drain_even_lanes`: red if a pure `if` header counts as a binding (I).
  `drain_optional_controls`: red if every `if let` is refused (K) or if the self-match bug is put
  back (M17).
- Not covered: the `map_or(0,` alternative in `uses` (MAJOR-3 a). The cap's value is not covered
  either (caps of 2, 3 and none stay green). This is acceptable for a safety cap.

## Gates run (from the export)

1. `bash scripts/check-realtime-policy.sh` prints `realtime policy: ok (90 marked regions in 25 files)`
   under gawk 5.2.1 (0.53 s), mawk 1.3.4 (0.85 s) and busybox awk (1.78 s), using `PATH` shims
   (`command -v awk` checked).
2. `bash scripts/test-realtime-policy.sh` prints `realtime policy mutation tests: ok` under the same
   three awks (52 s, 82 s, 79 s).
3. Red on revert: `main`'s gate (`da878bd49^`) passes the 7 D4 cases and the 15 attempt-2 cases,
   22 in all. Attempt 1's gate passes exactly the 10 cases the record lists, and it fails the
   valid fixture at `crates/builtins/src/lib.rs:90` (`drain_bank_lanes`).
4. Mutations: spec M1, M2 and M3 and the record's A to P and M17 all reproduce as recorded. M3 also
   reproduces on the real tree at `crates/builtins-compiler/src/lib.rs:475`, `:1078` and `:1122`. My
   extra mutations: R1 (restore `binder(<base>)`) and `maporpat` (drop `map_or(0,`) are green on the
   fixture and the real tree. WRhdr (drop the walked header from the read ranges) turns the valid
   fixture red at `crates/builtins/src/lib.rs:130`.
5. `bash scripts/check-workspace-policy.sh` prints `workspace policy: ok`.

The acked-batch question does not apply, because no queue or ack path changed. No Rust code changed.

Evidence kept in `/tmp/claude-1002/v1418b/evidence/`:
- `probe.rs`, `probe2.rs`, `probe3.rs`: the compiled demonstrations with their pop traces.
- `*.regions`: the same functions as marked regions. The full gate passes them under all three awks.
- `zipself.rs`: the two other zip shapes (accepted by the gate, refused by the prototype fix).
- `refchain40.rs`, `chain55.rs`: the mawk eval-stack inputs.
- `prototype-fixes.diff`, `mutations.py`, `runall.sh`, `probe.sh`.

The export, the fixtures, the mutation copies and the stress inputs were deleted.
