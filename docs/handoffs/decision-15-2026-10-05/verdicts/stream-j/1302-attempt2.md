VERDICT: FAIL

# #1302 attempt 2: verdict (stream J, commit 16a5b920b, base e3375bc1d; whole change ab9620a07..16a5b920b)

I reviewed this in the export `/tmp/claude-1002/v1302/src2` (a git archive of 16a5b920b) and did not touch the worktree.
The #1302 commit changes only the three authorized paths: `scripts/check-realtime-policy.sh`,
`scripts/test-realtime-policy.sh` and the spec's attempt record. The other paths in `e3375bc1d..16a5b920b` come from the
interleaved #1304 and #1237 commits. No Rust source changed, and the floors stay at 25 files and 89 regions.

Attempt 2 refuses the three shapes reproduced in attempt 1 (a01, a02, a17) and handles J1-2 through J1-6 as recorded.
The fix still does not cover every rustfmt layout. A wrapped header can put lines at the opener's own indentation that
`opener()` does not treat as part of the header. The gate's new claim (`:99-101`), that "wrapping a loop header
neither hides the loop nor its unbounded condition", is false for the layouts in J2-1.

## Findings

### MAJOR

**J2-1. Under rustfmt, a wrapped header can still hide its loop: by an or-pattern line, or by a `}` line followed by a `.method()` line.**
The defect is at `scripts/check-realtime-policy.sh:132-144` (`opener`) and `:211-213` (`check`). It has two causes:

- **(a) A header line led by `|`.** rustfmt puts each `|` line of a `while let` or-pattern at the `while`'s own
  indentation. `opener()` does not count a `|`-led line as a continuation, so:
  - a lone `{` resolves to the `|` line, which is not a loop;
  - a pop on the `|` line is at the opener's indentation, so `enclosing()` never sees the `while`.
- **(b) A `}` line inside the header.** `opener()` treats any line ending in `}` as a statement end (`:137`), at every
  step of its walk. When a header holds a `match`, `if` or block expression, rustfmt puts that expression's closing
  `}` at the opener's indentation and follows it with `.method()` and a lone `{`. The walk skips the `.method()`
  line, then meets the `}`, and calls the lone `{` a bare block. A pop on the `.try_pop()` line itself fails for the
  same reason as in (a): `check()` only asks whether the pop line is itself a loop opener (`:212-213`).

Three unbounded drains pass the gate (`realtime policy: ok (89 marked regions in 25 files)`) under gawk 5.2.1, mawk 1.3.4 and busybox awk 1.36.1, end to end through the real script in a fixture. All three are `rustfmt --edition 2024 --check` clean with the repo config and compile with `rustc --edition 2024`
(`/tmp/claude-1002/v1302/a2/rc/shapes.rs`):

```rust
        while let Ok(Record::GainChangeForTheLaneWithLongName(x))
        | Ok(Record::PanChangeForTheLaneWithLongName(x)) = control.try_pop()
        {
```
```rust
        while let Ok(record) = match self.lane_selector_for_this_block {
            Lane::Left => &mut self.left_consumer,
            Lane::Right => &mut self.right_consumer,
        }
        .try_pop()
        {
```
```rust
        while match self.lane_selector_for_this_block {
            Lane::Left => &self.left_consumer,
            Lane::Right => &self.right_consumer,
        }
        .has_records()
        {
            let Ok(record) = control.try_pop() else {
                break;
            };
```

The third shape is J1-1 itself: a pop in the body of a loop whose header rustfmt wraps passes as "no loop". These
shapes pass too, all rustfmt output, in `/tmp/claude-1002/v1302/a2/c/`:

| Shape | Files |
|---|---|
| a three-way or-pattern | d05 |
| a pop in the body of an or-pattern `while let` | c04 |
| the receiver chosen by `if`/`else`, `(if ..)` or `{ .. }`, then `.try_pop()` | d01, d07, d08 |
| a body pop under a `while if ..`, `while let .. = match ..` or `for _ in match ..` header | e01, e03, e04 |

The issue exists to stop rustfmt layout from defeating the rule (D1). For the same reason, attempt 1's J1-1 was
MAJOR.

*Fix.* A prototype was checked; the diff is `/tmp/claude-1002/v1302/a2/FIX_attempt2.diff`, 9 changed lines, and it
needs a comment update to match.
- In `opener`, add `|` to the continuation set in both places.
- Treat a `}`-ending line as a statement end only when it is the first same-indentation line before a lone `{`.
  Once the walk has skipped a continuation, a `}` line belongs to the header. A `;` line always ends the walk.
- In `check`, resolve the pop line first, with `r = opener(i); enclosing(r)`, and test `isloop(c[r])` /
  `bounded(r)`. A pop on a header continuation line is then judged by its header's opener.

Results under the prototype:
- It refuses all 14 escaping shapes under gawk, mawk and busybox awk.
- The real tree gives no hits under all three awks.
- Every earlier adversarial shape (a01-a17, b01-b03, c01-c18) gives the same result as before.
- Seven valid rustfmt shapes still pass: an or-pattern match arm, a guarded arm, a bare block after an `if`, a bounded
  drain in an `else`, a single `if let` pop on a `match` receiver, and others (`/tmp/claude-1002/v1302/a2/v/`).
- `bash scripts/test-realtime-policy.sh` prints `realtime policy mutation tests: ok`.

Add three `expect_failure` cases in existing regions: the or-pattern header pop, the `match`-receiver `.try_pop()`
pop, and the `while match ..`/`.has_records()` body pop.

### MINOR

**J2-2. Since the D3 scan moved into the awk pass, it no longer reaches a region that misordered markers leave open.**
The awk pass is at `scripts/check-realtime-policy.sh:222-224`. The marker check compares only counts, so a file
ordered `END .. BEGIN ..<EOF>` passes it. The body extraction (`:58-62`) takes that open tail, and the
forbidden-body predicate scans it. The drain pass runs `check()` only on an `END` line. Its `inside` state also runs
on into the next file, where that file's `BEGIN` resets `n`. So the tail is never checked.

In a fixture, I gave `hosts/host-web/src/lib.rs` the lines `END`, `fn render_next() {}`, `BEGIN`, then
`core::iter::from_fn(|| control.try_pop().ok()).for_each(apply);` and an unbounded `loop` drain.
- Attempt 2 prints `realtime policy: ok`.
- Attempt 1 refused the `from_fn` line, because its `gate_scan_forbidden` scanned the extracted bodies.

This is a coverage loss caused by the departure from D3, and it adds to the drain rule's existing blind spot.

*Fix.* Keep the region's file name when `BEGIN` is seen. Run `check()` on a region still open at `FNR == 1` and in an
`END {}` block. Add one case.

### NIT

- **J2-3. `drain-entry-name-in-header` has no catch of its own.** (`scripts/test-realtime-policy.sh:807`)
  - Mutant L (`available_at_entry` matched as a substring in the header) also reddens `drain-entry-name-in-binding`
    (`:795`). That case's header, `0..not_available_at_entry_cap`, also contains the substring. The attempt
    record's own table shows this.
  - *Fix:* write the binding case as `let cap = control.not_available_at_entry_cap();` followed by
    `for _ in 0..cap {`. K then reddens only the binding case, and L only the header case.
- **J2-4. The stated limits miss two rebinding shapes.** (`:106-112`) Both pass:
  - a `for` pattern that rebinds the count: `let available = ..available_at_entry(); for &available in limits { for _ in 0..available { pop } }`
    (`n/n04`);
  - a destructure inside the count's own statement: `let (available, _) = (usize::MAX, control.available_at_entry());`
    (`n/n05`). This shape is new in attempt 2, because pattern `let`s now count as bindings.

  *Fix:* add "a `for` pattern" to the rebinding list. Widen "inflated by arithmetic inside its own statement" to
  "any other value computed in its own `available_at_entry` statement".
- **J2-5. The enclosing-loop check accepts any `for` that is not over an open range.** (`:217`)
  - These outer loops around a bounded drain pass: `for _ in core::iter::repeat(())`, `for _ in (0..).step_by(1)` and
    `for _ in 0..usize::MAX` (`n/n01`-`n03`).
  - The claim's literal wording is accurate. But its stated reason ("re-drains without bound") also applies to these
    loops.
  - *Fix:* either list them as limits, or accept as enclosing loops only `for .. in 0..<ident>` and iteration over a
    collection.
- **J2-6. A bounded drain inside `array::from_fn` is refused, though the same drain in a per-lane `for` passes.**
  - Because `array::from_fn` counts as a loop opener, `core::array::from_fn(|lane| { let available = ..; for _ in 0..available { pop } .. })`
    is refused (`n/n06`). The equivalent `for lane in 0..2` passes (`n/n07`).
  - A bare `array::from_fn`, after `use core::array;`, is refused under the drain class.
  - Both fail safe. Say so in the comment, or accept them.
- **J2-7. The attempt record's reason for leaving D3's single `gate_scan_forbidden` is imprecise.**
  - The record says "a bare call cannot be told from `core::array::from_fn` without a lookbehind". In fact
    `(^|[^:[:alnum:]_])from_fn` separates a bare call from a qualified one without a lookbehind.
  - What awk adds is refusing every other qualified path, such as `it::from_fn`. That strengthening is sound, so
    reword the reason.

## The questions asked

- **Is J1-1 fixed?** Yes for a01, a02 and a17, and for the record's five fixture layouts. It is not fixed for the layouts in J2-1.
  - Of 29 further rustfmt-formatted candidates, these 18 are refused correctly:
    - c01: a wrapped `for` pattern;
    - c02: a turbofish;
    - c05: a break after `in`;
    - c06, c07: `match` and `if` conditions closed by `} {`;
    - c08: a struct pattern;
    - c09: an array root with a binary operator;
    - c10: a closure;
    - c11: an `as` cast;
    - c12: a let chain;
    - c13: a string;
    - c14: `matches!`;
    - c15: a unary operator;
    - c16: an index;
    - c17: a tuple pattern;
    - c18: a `?`;
    - d02, d03: index and array receivers.
  - The 11 in J2-1 pass. d06, an unparenthesised or-pattern `let`-`else`, is not valid Rust, so it is left out.
- **J1-2's departure from D3.** The scan sits in the drain pass's `awk`, whose status is checked (`:225`):
  - On a fixture whose only defect is a bare `from_fn`, an injected awk failure (status 2) prints
    `realtime drain-bound scan failed (awk status 2)` at `partial` 0 and 1.
  - A mutant that swallows the status passes that fixture with `ok`.
  - The committed counter-mutant `drain-bound` covers this, and `drain-bound-0/1` still pass.

  The scan is correct for the shapes listed below, apart from the coverage loss in J2-2:

  | Refused | Passes |
  |---|---|
  | bare calls | `core::array::from_fn` and `std::array::from_fn` with no pop |
  | `iter::` calls | `from_fn` in a comment |
  | `std::iter::` calls | |
  | a pop inside `core::array::from_fn` | |
- **The `array::from_fn` and `(0..)` rules.** Both work. `(0..)`, `((0..))` and `1..` are refused as enclosing loops;
  `(0..).take(n)` is not. Their residual gaps are J2-5 and J2-6.
- **False positives on the real tree.** None. Every check below is identical under gawk, mawk and busybox awk.
  - `realtime policy: ok (89 marked regions in 25 files)`.
  - A debug copy of the pass finds these innermost loops, matching the record:

    | Real site | Innermost loop |
    |---|---|
    | `builtins-compiler/src/lib.rs:1076` | `1075 for _ in 0..available {` |
    | `builtins-compiler/src/lib.rs:1120` | `1119 for _ in 0..available {` |
    | `effect-contract/src/live.rs:366` | `364 while remaining != 0 {` |
    | `plan_exchange.rs:377` | no loop |
    | `spsc.rs:440` | no loop |
    | `graph/src/runtime.rs:900` | `899 for _ in 0..available {` |
- **Does each new case have a distinct mutant?** All 14 mutants named in the record (A, B, C, D, H, J, K, L, M, N, O, P, Q, R)
  were rebuilt as scratch copies in `/tmp/claude-1002/v1302/a2/mut/`. Each was run through a reporting copy of the self-test. Every
  one reddens exactly what the record says:
  - A reddens the four wrapped cases;
  - B reddens `paren-led-header`;
  - C reddens `continuation-led-header`;
  - D reddens `wrapped-outer-while`;
  - H reddens `wrapped-for-bound`;
  - J reddens `bare-from-fn`;
  - K reddens `entry-name-in-binding`;
  - L reddens `entry-name-in-binding` and `entry-name-in-header`;
  - M reddens `header-past-closed-body`;
  - N reddens `array-from-fn-pop`;
  - O reddens `pattern-rebound-count`;
  - P and Q redden the valid fixture;
  - R reddens `paren-open-range-outer`.

  The unmutated base has no red case. The only case without a sole catch is `entry-name-in-header` (J2-3).
- **Are the listed limits honest?** For J1-3, yes: a06, a07, a08, a12 and b02 pass, and each is listed. But the
  rustfmt claim at `:82-86` and `:99-101` is false (J2-1), and two rebinding shapes are missing (J2-4).

## Gates run (in the export)

- **Gate 1:** `bash scripts/check-realtime-policy.sh` prints `realtime policy: ok (89 marked regions in 25 files)`, rc 0,
  under gawk, mawk and busybox awk (each forced through a PATH shim).
- **Gate 2:** `bash scripts/test-realtime-policy.sh` prints `realtime policy mutation tests: ok`, rc 0, under gawk, mawk and
  busybox awk (about 55 s each). The valid fixture and all 12 new mutated fixtures pass `rustfmt --edition 2024 --check` with
  max_width 100.
- **Gate 3 (scratch, on the attempt-2 gate):**
  - Restoring the old regex and deleting the pass reddens all 29 drain cases, including cases 1-3 and 6-8.
  - Checking the outermost loop reddens the valid fixture. Case 7's own shape (`adv/case7.rs.txt`) passes under that
    mutant (0 hits, against 1 on the base).
- **Gate 4:** `bash scripts/check-workspace-policy.sh && bash scripts/test-workspace-policy.sh` prints `workspace policy: ok` …
  `workspace policy mutation tests: ok`, rc 0. `shellcheck` is not installed on this host, so it was not run.

## Test value (one sentence each; each was reproduced red on the named scratch mutant and green unmutated)

- **`drain-wrapped-while-body`** turns red if a lone `{` is not resolved to its wrapped header's opener (mutant A).
  It is J1-1's regression reproducer, recorded red on the bug's revert.
- **`drain-wrapped-for-bound`** turns red if a wrapped header is judged by its first line alone (H): that line reads
  `for _ in 0..available`, while the full header also adds the ring's capacity.
- **`drain-wrapped-outer-while`** turns red if only the innermost block's opener is resolved, so an outer wrapped
  `while` around a bounded drain goes unseen (D).
- **`drain-paren-led-header`** turns red if a `) {` last header line is not resolved to its `for` (B).
- **`drain-continuation-led-header`** turns red if the resolution stops at a `.method()` or `]` line at the opener's
  indentation (C).
- **`drain-bare-from-fn`** turns red if D3 refuses only `iter::`-qualified constructors (J).
- **`drain-entry-name-in-binding`** turns red if `available_at_entry` is matched as a substring in the binding (K).
- **`drain-entry-name-in-header`** turns red if it is matched as a substring in the header (L). The binding case
  also catches L (J2-3).
- **`drain-header-past-closed-body`** turns red if the header form's `[^{}]*` becomes `.*` (M).
- **`drain-array-from-fn-pop`** turns red if `array::from_fn` is not a loop opener, so a constant-count pop slips past
  the exception (N).
- **`drain-pattern-rebound-count`** turns red if only `let [mut] <count> =` counts as a binding (O).
- **`drain-paren-open-range-outer`** turns red if `(0..)` does not count as an open range (R).
- **Valid-fixture additions:**
  - `retire_one` (a bare block after a `for`) turns red if a lone `{` after a `}` statement resolves to an earlier
    line (P).
  - The `core::array::from_fn`/`std::array::from_fn` input arrays turn red without the exception (Q).

## Adversarial shapes

| Set | Shapes | Result on attempt 2 | Matches the record? |
|---|---|---|---|
| Attempt-1 shapes (`/tmp/claude-1002/v1302/adv/`) | a01, a02, a17, a05, a13, a14, b03 | refused | yes |
| | a06, a07, a08, a12, b02 | pass | yes (listed limits) |
| | a09, a10, a16, b01 | pass | yes (correct) |
| | a03, a04, a11, a15 | refused | yes |
| New shapes (`/tmp/claude-1002/v1302/a2/`) | c03, c04, d01, d04, d05, d07, d08, e01-e04, and `rc/shapes.rs` (all three) | pass | no: false negatives (J2-1) |
| | n04, n05 | pass | no: unlisted limits (J2-4) |
| | n01-n03 | pass | NIT (J2-5) |
| | n06 | refused | fail-safe inconsistency (J2-6) |
| | `dangle/` | pass | no: regression (J2-2) |
