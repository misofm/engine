# Make the realtime-policy drain rule structural instead of one regex line

Stream J of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0, D15-2).
Code anchors verified on `main` at `6fb211594`.

Tooling follow-up of *Bound the builtin fader and matrix drains to the records present at block
entry* (#1253, slice of umbrella #1053, decision D11). Its verdict (Sol, attempt 1, N1;
`docs/handoffs/live-updates-1053/1253-attempt1.md`) found that the gate's unbounded-drain rule is a
single-line regex that three ordinary spellings pass. Recorded in
`docs/handoffs/live-updates-1053/README.md`, "Follow-up candidates". No production code changes.

## Problem

- **The rule.** `scripts/check-realtime-policy.sh:79-84` runs one `gate_scan_forbidden` over the
  extracted marked bodies with the pattern
  `while[[:space:]]+let[[:space:]]+Ok[[:space:]]*\(.*=.*\.try_pop[[:space:]]*\(`. It sees one
  line at a time.
- **What passes it.** The verdict put three unbounded drains into a marked fixture region, and the
  gate passed all three:
  1. the `while let` whose `try_pop()` rustfmt wraps onto a later line (a long receiver inside a
     deep `impl`), for example

     ```rust
                     while let Ok(record) = self
                         .some_long_receiver_name
                         .control_consumer_for_this_lane
                         .try_pop()
                     {
     ```
  2. `loop { let Ok(record) = control.try_pop() else { break }; .. }`;
  3. `while let Ok(record) = Consumer::try_pop(control) { .. }`.
- **What the gate's comment says.** `check-realtime-policy.sh:79-82`: a render-thread drain pops
  at most the records present at block entry. The gate proves less than that. (#1053 D11,
  `.github/ISSUE_SPECS/1053-*.md:125-128`, now says only that each builtin drain is bounded and
  that with cells it becomes a dirty-mask scan; it no longer names this gate.)
- **Why a region-level rule is not enough.** The verdict's suggestion, "a `try_pop(` in a marked
  body requires `available_at_entry` in the same region", is weak here because regions are large:
  `crates/graph/src/runtime.rs` has a region spanning `:321-933` that holds the route drain's
  `available_at_entry` (`:897`). A second, unbounded loop anywhere in that region would pass. It
  would also miss `crates/effect-contract/src/live.rs:366`, which names the pop as a path
  (`map(Consumer::try_pop)`), with no `try_pop(`.
- **Why it matters.** The C ABI prepares with `Concurrent` delivery, so a control thread can push
  while the render thread drains. A drain that pops until empty does data-dependent, unbounded
  render work, which `AGENTS.md` forbids.

### Every `try_pop` in a marked region today

The gate extracts the lines between each `REALTIME_POLICY_BEGIN` and `REALTIME_POLICY_END` in
`crates hosts tools` (25 files, 89 regions; `bash scripts/check-realtime-policy.sh` prints
`realtime policy: ok (89 marked regions in 25 files)`). In those bodies the word `try_pop` occurs
exactly six times. Each must pass the new rule:

1. `crates/builtins-compiler/src/lib.rs:1076`, `drain_fader_controls` (region `:1063-1105`).
   `let available = control.available_at_entry();` (`:1074`), then `for _ in 0..available {`
   (`:1075`). Bounded: a `for` over `0..` an entry count.
2. `crates/builtins-compiler/src/lib.rs:1120`, `drain_matrix_controls` (region `:1107-1135`). The
   same shape (`:1118`, `:1119`).
3. `crates/graph/src/runtime.rs:900`, the route `drain` (region `:321-933`).
   `let available = self.control.consumer.available_at_entry();` (`:897`), then
   `for _ in 0..available {` (`:899`). The same shape.
4. `crates/effect-contract/src/live.rs:366`, `EffectControlLane::stage` (region `:231-537`).
   `let available = self.control.as_ref().map_or(0, Consumer::available_at_entry);` across
   `:349-352`, `let mut remaining = available;` (`:359`), `while remaining != 0 {` (`:364`), and
   the pop named as a path, `.map(Consumer::try_pop)`. Bounded: `while <count> != 0` over a
   one-step alias of an entry count.
5. `crates/engine/src/realtime/plan_exchange.rs:377`, `enter_block` (region `:347-491`).
   `if self.pending.is_none() && let Ok(candidate) = self.publication.try_pop()`. One pop, in no
   loop.
6. `crates/engine/src/realtime/spsc.rs:440` (region `:295-459`).
   `pub fn try_pop(&mut self) -> Result<T, QueueEmpty> {`. The definition, not a call.

Not in any marked region, so out of this rule's reach: the input drain
`BuiltinBankProcessor::drain_controls` (`crates/builtins-compiler/src/lib.rs:460-506`, bounded
today), the three test-only oracle drains (`:4252`, `:4350`, `:4420`, all
`#[cfg(any(test, feature = "test-support"))]`), and `plan_exchange.rs:501`, `:513` and `:521`
(control-side retirement).

### The latest-target cells change the set, not the rule

Decision 15, D15-2 replaces the FIFO drains of live values with latest-target cells: strip fader,
mute and matrix (*Hold live values in latest-target cells on both hosts*, #1312), effect parameter,
bypass and EQ target (#1345), strip input lane (#1346) and route lanes (#1347). A cell drain is a
dirty-mask scan with no `try_pop`, so sites 1-4 above leave the set as those issues land. What
stays FIFO (automation, Observe records, structural and time-stamped records, and the plan
publication of site 5) keeps popping from SPSC queues, and any new FIFO drain in render must be
bounded. The rule is therefore written against shapes, not against today's six sites: the
self-test fixture (gate 2) carries every bounded shape itself, so the rule stays tested whether it
lands before or after the cells.

The marked code has no tab, no loop label, no `= loop {` and no `//` inside a string literal
(checked over the extracted bodies), and `cargo fmt --all -- --check` is a required qualification
step (`.github/workflows/qualification.yml:456`). The rule below relies on rustfmt's layout.

## Decisions

- **D1. Recommendation: the structural rule, not a narrower D11.** A ~40-line awk pass over each
  marked region (sketch below) passes all six sites above and refuses all three verdict variants;
  it was prototyped against this tree. That is within half a day, and it makes the gate's comment
  true. Narrowing D11 to "refuses the one-line `while let Ok(..) = ..try_pop()` spelling" would
  leave the concurrent FIFO drains that stay after the cells guarded by a rule that rustfmt alone
  can defeat.
- **D2. The rule.** Within each marked region (the lines strictly between a `BEGIN` and its `END`):
  1. *Code.* Drop everything from the first `//` on a line. Skip lines that are then blank.
     Indentation is the count of leading spaces.
  2. *Pop site.* A line whose code contains the word `try_pop` (not part of a longer identifier),
     except a line matching `fn[[:space:]]+try_pop` (the definition).
  3. *Loop opener.* A line whose trimmed code starts with `while` or `loop` as a word, or with
     `for ` and contains ` in `. (The ` in ` keeps `impl X for Y {` out.)
  4. *Function boundary.* A line whose trimmed code is `fn` after zero or more lowercase
     qualifiers (`pub`, `pub(crate)`, `const`, `unsafe`, ...).
  5. *Innermost loop of a pop site.* If the pop line is itself a loop opener, that line (the pop
     is in the loop's condition). Otherwise walk back from the pop line, keeping `min` = the pop
     line's indentation. A line with indentation below `min` encloses the pop: set `min` to it;
     stop with "no loop" if it is a function boundary; stop with that line if it is a loop
     opener. Reaching the region's first line means "no loop".
  6. *Bounded loop.* The loop's header is its opener line joined with the following lines up to
     the first whose code ends in `{`, whitespace collapsed. It is bounded only if it is
     - `for <pattern> in 0..<ident> {`, or `for <pattern> in 0..<expr> {` where `<expr>` contains
       `available_at_entry`; or
     - `while <ident> != 0 {` or `while <ident> > 0 {`;

     and `<ident>` is *an entry count*: some earlier line of the same region opens a statement
     `let [mut] <ident>[: <type>] = ...;` (joined up to the first line ending in `;`) that contains
     `available_at_entry`, or that is exactly `let [mut] <ident> = <other>;` where `<other>` is an
     entry count by the first form (one alias step, which `live.rs:359` needs).
  7. A pop site with no loop passes. A pop site whose innermost loop is not bounded fails, printed
     as `file:line:` with the line, under the class
     `marked realtime unbounded try_pop drain (bound it with available_at_entry)` (the current
     message, so the existing mutation case keeps its class).
- **D3. The iterator escape.** `iter::from_fn`, `iter::repeat_with` and `iter::successors` in a
  marked body are refused under the same class, by one `gate_scan_forbidden` over the extracted
  bodies (`core::array::from_fn` is used in marked code today, at
  `crates/builtins/src/lib.rs:2454` and elsewhere, and stays allowed). They are how a pop runs in
  a loop without a loop keyword.
- **D4. Supersede, do not stack.** The regex rule at `check-realtime-policy.sh:78-84` is deleted;
  D2 refuses everything it refused (the one-line form is a pop site on its own loop opener). Its
  comment moves to the new rule, with the rule's stated limits (Hazards).
- **D5. Floors unchanged.** No marker is added or removed, so the floors stay at 25 files and 89
  regions (`check-realtime-policy.sh:73-74`). The self-test fixture sits exactly on those floors
  (`scripts/test-realtime-policy.sh:284-300`) and its floor cases depend on that: add the new
  fixture shapes as extra lines inside existing fixture regions, never as a new file or region.
- **D6. The claim lives in the gate.** The new rule's comment states what the gate proves: it
  refuses a `try_pop` whose innermost loop in a marked region is not bounded by a count taken from
  `available_at_entry`, and refuses the unbounded iterator constructors; it reads rustfmt layout.
  #1053 is not edited (its D11 no longer names this gate).
- **D7. Independent of the cells.** No dependency either way on #1312, #1345, #1346 or #1347. If
  one lands first and removes a real site, nothing here changes except that site's evidence line.

### Sketch (prototyped on this tree; the implementer owns the final form)

```awk
function code(s,  i){ i=index(s,"//"); if(i) s=substr(s,1,i-1); sub(/[ \t]+$/,"",s); return s }
function ind(s){ match(s,/^ */); return RLENGTH }
function trim(s){ sub(/^ +/,"",s); return s }
function isloop(s,  t){ t=trim(s); return t ~ /^(while|loop)([^A-Za-z0-9_]|$)/ || t ~ /^for .* in / }
function isfn(s){ return trim(s) ~ /^([a-z()]+ +)*fn +[A-Za-z_]/ }
function counted(x, before, depth,   i,j,st,y){
  if (depth > 1) return 0
  for (i = 1; i < before; i++) if (c[i] ~ ("(^| )let +(mut +)?" x "( *:[^=]*)? *=")) {
    st = ""; for (j = i; j <= n; j++) { st = st " " c[j]; if (c[j] ~ /;$/) break }
    if (st ~ /available_at_entry/) return 1
    if (match(st, "= *[a-z_][a-z0-9_]* *;$")) {
      y = substr(st, RSTART, RLENGTH); gsub(/[= ;]/, "", y); if (counted(y, i, depth + 1)) return 1 } }
  return 0 }
function bounded(l,  h,j,x){
  h = ""; for (j = l; j <= n; j++) { h = h " " trim(c[j]); if (c[j] ~ /\{$/) break }
  gsub(/ +/, " ", h); sub(/^ /, "", h)
  if (h ~ /^for .* in 0\.\..*available_at_entry.* \{$/) return 1
  if (h ~ /^for .* in 0\.\.[A-Za-z_][A-Za-z0-9_]* \{$/) {
    x = h; sub(/^.* in 0\.\./, "", x); sub(/ \{$/, "", x); return counted(x, l, 0) }
  if (h ~ /^while [A-Za-z_][A-Za-z0-9_]* (!= 0|> 0) \{$/) {
    x = h; sub(/^while /, "", x); sub(/ .*/, "", x); return counted(x, l, 0) }
  return 0 }
function check(  i,k,m,l){
  for (i = 1; i <= n; i++) {
    if (c[i] !~ /(^|[^A-Za-z0-9_])try_pop([^A-Za-z0-9_]|$)/ || c[i] ~ /fn +try_pop/) continue
    l = 0
    if (isloop(c[i])) l = i
    else { m = ind(c[i]); for (k = i - 1; k >= 1; k--) { if (c[k] ~ /^ *$/) continue
      if (ind(c[k]) < m) { m = ind(c[k]); if (isfn(c[k])) break; if (isloop(c[k])) { l = k; break } } } }
    if (l && !bounded(l)) print FILENAME ":" ln[i] ":" c[i] } }
/REALTIME_POLICY_BEGIN/ { inside = 1; n = 0; next }
/REALTIME_POLICY_END/ { inside = 0; check(); next }
inside { n++; c[n] = code($0); ln[n] = FNR }
```

## Authorized paths

- `scripts/check-realtime-policy.sh`: the rule of D2-D4 and its comment only.
- `scripts/test-realtime-policy.sh`: fixture lines inside existing regions, the new mutation
  cases, and the tool-failure injection case of gate 2.
- This spec.

## Non-goals

- Any Rust source. The six sites already pass; if one does not pass the implemented rule, the rule
  is wrong, not the site.
- Marking more code (the input drain at `crates/builtins-compiler/src/lib.rs:460` stays unmarked
  here); raising or lowering a floor.
- A general Rust parser. Recursion, a pop inside a closure handed to some other repeating adapter,
  and a `use` that renames `iter::from_fn` stay outside the rule; the comment says so.
- Other pop-like APIs: `try_pop` is the only consumer pop of `crates/engine/src/realtime/spsc.rs`.

## Hazards

- **rustfmt layout.** The walk-back reads indentation. That is sound only on formatted code, which
  the required `cargo fmt --all -- --check` guarantees for the tree. Fixture text in the self-test
  must be in rustfmt's layout too (four-space indents, a wrapped header's `{` on its own line).
- **The fixture sits on the floors.** Adding a fixture file or region breaks
  `marked-file-count-floor` and `marked-region-count-floor` (D5). The `empty_bodies` run
  (`test-realtime-policy.sh:345-355`) deletes only one-line `fn name() {}` lines, so multi-line
  fixture bodies survive it; they must still pass.
- **Existing mutation anchors.** Cases `sed` one-line fixture bodies by name
  (`fn drain_fader_controls() {}` at `:277`, `fn stage() {}` at `:123`, `fn push() {}` at `:43`,
  `fn exchange_plane() {}` at `:66`). Keep each such line; add the new shapes beside it.
- **Tool status.** Every `awk`/`rg` call in this gate checks its exit status and names its step
  (`gate.sh`; the body extraction at `check-realtime-policy.sh:58-62`). The new pass must too, and
  the shim's `body-read:awk` matcher (`test-realtime-policy.sh:480`) matches any `awk` whose
  arguments name `runtime.rs`: run the new pass after the body extraction, or give its injection
  case a distinguishing matcher.

## Objective gates

1. **The tree passes, unchanged.** `bash scripts/check-realtime-policy.sh` prints
   `realtime policy: ok (N marked regions in M files)` with the counts of the tree it merges onto
   (89 and 25 at `6fb211594`; a cell slice that lands first may move them, with its floors).
2. **The self-test.** `bash scripts/test-realtime-policy.sh` prints
   `realtime policy mutation tests: ok`, with:
   - the valid fixture carrying, inside existing regions, the four bounded shapes of sites 1, 4, 5
     and 6: a `for _ in 0..available` drain, the `live.rs` alias-and-`while remaining != 0` drain
     with the pop as `map(Consumer::try_pop)`, an `if ... && let Ok(..) = ..try_pop()` single pop,
     and a `pub fn try_pop(&mut self)` definition;
   - the existing `marked-unbounded-try-pop-drain` case, unchanged;
   - one `expect_failure` case each, all with the class `bound it with available_at_entry`:
     1. the wrapped `while let` (the rustfmt shape quoted in Problem);
     2. `loop` with `let Ok(record) = control.try_pop() else { break };`;
     3. `while let Ok(record) = Consumer::try_pop(control) {`;
     4. `for _ in 0..64 {` around a pop (a constant bound);
     5. `let available = control.capacity();` then `for _ in 0..available {` around a pop (a bound
        not taken at entry);
     6. a second, unbounded `loop` around a pop, added to a region that already holds the bounded
        drain;
     7. a `loop` around a pop, nested inside a bounded `for _ in 0..available`;
     8. `core::iter::from_fn(|| control.try_pop().ok())` and, separately,
        `core::iter::repeat_with(..)`;
   - one `expect_tool_error` case for the new `awk` pass, for `partial` 0 and 1, naming its step.

   *Test value.* Cases 1-3 turn red if the rule goes back to matching one spelling on one line
   (the defect of the verdict's N1). Case 6 turns red if the rule is relaxed to "the region contains
   `available_at_entry`". Case 7 turns red if the rule checks the outermost enclosing loop instead
   of the innermost. Cases 4 and 5 turn red if any `for` over a range, or any identifier, counts as
   bounded. Case 8 turns red if D3's scan is dropped. The valid fixture turns red if the rule
   refuses one of today's six real shapes (for example, by losing the `fn try_pop` exemption or the
   one alias step).
3. **Mutation of the gate (PR evidence, not committed).** In a scratch copy of the gate, make the
   walk-back return the outermost loop instead of the innermost; show case 7 red. Restore the old
   regex in place of the new rule; show cases 1-3 and 6-8 red. Record both outputs.
4. **Policy.**
   - `bash scripts/check-workspace-policy.sh && bash scripts/test-workspace-policy.sh`
   - `shellcheck scripts/check-realtime-policy.sh scripts/test-realtime-policy.sh` if `shellcheck`
     is installed; otherwise record that it was not run.

*Test value* is answered per case in gate 2. No Rust test is added: the rule is a lint gate and its
mutation cases are its tests.

## Evidence

- Gate 1's line, and gate 2's final line.
- For each real site still present (six at `6fb211594`), the innermost loop line the rule found (or "no loop"), printed
  once by a debug run (not committed).
- Gate 3's two outputs.

## Dependencies

- None. #1253 is on `main`. The cell slices (#1312, #1345, #1346, #1347) may land before or after
  this one (D7).

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused. The realtime-policy script is a lint gate over
  marked source, and its self-test runs on fixtures; both are allowed.
- Commit on its own branch from synchronized `main`.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).

## Attempt record

### Attempt 1 (implementer, 2026-10-05)

**Changed.** `scripts/check-realtime-policy.sh`: the one-line regex is deleted (D4) and replaced by
the D3 iterator scan and a structural `awk` pass over every marked file, run after body
extraction, with its own checked status (`realtime drain-bound scan failed (awk status N)`).
`scripts/test-realtime-policy.sh`: the four bounded real shapes inside existing fixture regions
(D5: floors untouched), seventeen `expect_failure` drain cases, the `drain-bound` tool-failure
injection for `partial` 0 and 1, and a counter-mutant proving that injection reds only when the
status is observed. The `body-read:awk` matcher now excludes the new pass (it names `runtime.rs`
too), so the existing `per-file-read` counter-mutant still reaches its intended assertion.

**Where the rule goes beyond the D2 sketch, and why (all inside the two authorized scripts).**
1. *A loop keyword anywhere on a line opens a loop; only a line-leading one (after an optional
   `'label:`) can be bounded.* The sketch's start-of-line opener let the existing one-line case
   `fn drain_fader_controls() { while let Ok(record) = control.try_pop() { .. } }` pass (it is not a
   line-leading `while`), contradicting D4, and also let `'drain: loop {` and `let x = loop {`
   pass.
2. *Every loop enclosing the innermost one, up to the `fn`, must be a `for .. in` over a non-open
   range.* A bounded `for _ in 0..available` inside an outer `loop` that re-reads
   `available_at_entry` drains without bound; the real site 1 sits in a per-lane `for` and passes.
3. *The count is the nearest in-scope binding* (scope walk by indentation, stopping at the `fn`),
   *with no assignment to it between binding and loop.* The sketch took the earliest binding in
   the region, so a shadowing `let available = usize::MAX;` or an earlier function's binding
   passed.
4. *A `while <count> != 0` body's first statement must be `<count> -= 1;`, with no other
   assignment in the body.* Without it, a `while remaining != 0` that never decrements (or only
   conditionally) pops until empty.
5. *Header bounded forms use `[^{}]*` instead of `.*`*, so a one-line loop body cannot join onto a
   later line and borrow its `available_at_entry`.
6. *The `fn try_pop` exemption (D2.2) is dropped.* Under D2.7 a pop with no enclosing loop passes,
   and a definition never has one, so the exemption has no observable effect: a mutant removing it
   reddens nothing. The `pub fn try_pop(&mut self)` fixture shape stays (gate 2) and still defends
   "a pop outside a loop passes" (mutant M12c below).

**Gate 1.** `bash scripts/check-realtime-policy.sh` -> `realtime policy: ok (89 marked regions in
25 files)` (gawk 5.2.1; also under mawk and busybox awk).

**Gate 2.** `bash scripts/test-realtime-policy.sh` -> `realtime policy mutation tests: ok` (gawk and
mawk).

**Real sites (debug run, not committed; innermost loop found).**
`builtins-compiler/src/lib.rs:1076` -> `1075: for _ in 0..available {`;
`:1120` -> `1119: for _ in 0..available {`; `effect-contract/src/live.rs:366` ->
`364: while remaining != 0 {`; `engine/src/realtime/plan_exchange.rs:377` -> no loop;
`graph/src/runtime.rs:900` -> `899: for _ in 0..available {`; `engine/src/realtime/spsc.rs:440` is
the definition (no loop). Identical under gawk, mawk and busybox awk.

**Gate 3 and per-case test value (scratch mutants of the gate, each run through a copy of the
self-test that reports every case instead of stopping; not committed).** A case is "red" when the
mutant gate passes its fixture.

| Mutant of the gate | Cases red |
|---|---|
| M1 walk-back takes the outermost loop, checks only it | valid fixture (the per-lane `for` is not an entry count); on a fixture without that drain, `loop-inside-bounded-for` (case 7) |
| M2 old one-line regex restored, D3 scan and pass deleted | cases 1-8 (`wrapped-while-let`, `let-else-loop`, `path-pop`, `constant-bound`, `bound-not-at-entry`, `second-loop-in-bounded-region`, `loop-inside-bounded-for`, `from-fn`, `repeat-with`) and every extra case |
| M3 a region naming `available_at_entry` passes | existing `marked-unbounded-try-pop-drain`, case 6, case 7, `bounded-drain-inside-outer-loop`, `count-from-another-function`, `shadowed-count`, `reassigned-count`, both `while` cases |
| M4 any `for .. in 0..` is bounded | case 4, case 5, `count-from-another-function`, `shadowed-count`, `reassigned-count` |
| M5 any identifier is an entry count | case 5, `count-from-another-function`, `shadowed-count`, `reassigned-count` |
| M6 enclosing loops unchecked | `bounded-drain-inside-outer-loop` |
| M7 earliest binding anywhere in the region (the sketch) | `count-from-another-function`, `shadowed-count` |
| M8 no assignment check | `reassigned-count` |
| M9 no first-decrement check | `while-without-decrement`, `while-decrement-not-first` |
| M10 start-of-line openers only (the sketch) | existing `marked-unbounded-try-pop-drain`, `labelled-loop`, `loop-expression` |
| M11 D3 scan dropped | `from-fn`, `repeat-with` |
| M12b no alias step | valid fixture (the `live.rs` shape) |
| M12c a pop outside any loop is refused | valid fixture (`plan_exchange` and the `try_pop` definition) |

Unmutated, every case is green. Test value per case: 1-3 red if the rule reverts to one spelling on
one line (M2); 4 and 5 red if any range or any identifier counts (M4, M5); 6 red on a region-level
rule (M3); 7 red if the outermost loop is checked instead of the innermost (M1); 8 red if D3's scan
is dropped (M11); `bounded-drain-inside-outer-loop` red if enclosing loops are unchecked (M6);
`count-from-another-function` and `shadowed-count` red if the binding search ignores scope
(M7); `reassigned-count` red without the assignment check (M8); the two `while` cases red without
the first-decrement check (M9); `labelled-loop` and `loop-expression` red if only line-leading
unlabelled openers count (M10); the valid fixture red if the rule refuses a real shape (M12b,
M12c). `drain-bound-0/1` red if the pass's awk status is swallowed (counter-mutant
`drain-bound`, committed, proves it).

**Gate 4.** `bash scripts/check-workspace-policy.sh` -> `workspace policy: ok`;
`bash scripts/test-workspace-policy.sh` -> `workspace policy mutation tests: ok`. `shellcheck` is
not installed on this host: not run.

### Attempt 2 (implementer, 2026-10-05)

Answers verdict `1302-attempt1.md` (FAIL). Same two scripts; no Rust source; floors unchanged.

**Changed in `scripts/check-realtime-policy.sh`.**
- *J1-1 (MAJOR).* New `opener(k)`: a block that opens on a lone `{`, or on a line led by `)`, `]`,
  `}` or `.` (the last line of a wrapped header: `) {`, `] {`, `}) {`, `} else {`), belongs to the
  nearest earlier line at the same indentation that is not itself such a continuation nor `where`.
  A lone `{` after a statement ending in `;` or `}` is a bare block and opens itself. `enclosing()`
  records the resolved opener in the chain, and the binding scope walk of `counted()` resolves it
  too (so a `where` fn's lone `{` stops at the `fn`). The comment now states rustfmt's layout
  correctly.
- *J1-2.* The D3 scan moves into the awk pass (comments stripped, same class, same checked awk
  status) and refuses `from_fn`, `repeat_with` and `successors` as words, path-qualified or bare;
  `core::array::from_fn` and `std::array::from_fn` are excepted. Deviation from D3's "one
  `gate_scan_forbidden`": a bare call cannot be told from `core::array::from_fn` without a
  lookbehind, which `rg`'s default engine lacks. Added: `array::from_fn` is a loop opener that is
  never bounded (it runs its closure a constant number of times), so the exception cannot carry a
  constant-count pop.
- *J1-3.* The comment's limits now name a pop in a helper method or local closure a loop calls,
  and a loop enclosing the drain outside the marked region.
- *J1-4.* `available_at_entry` is matched as a word in the binding statement and in the
  `for .. in 0..<expr>` header.
- *J1-5.* A `let` whose pattern binds the count (`let Some(available) = ..`) is its nearest binding;
  `if let`, closure and match-arm rebinding stay listed limits.
- *J1-6.* `header-past-closed-body` defends `[^{}]*`.
- *J1-7 (part).* `for _ in (0..) {` counts as an open range in the enclosing-loop check (the
  comment's claim said "open range"; adv `b03` passed). Adapter chains (`map_while` over
  `iter::repeat`/open ranges) stay a listed limit.

**Gates.** Gate 1: `realtime policy: ok (89 marked regions in 25 files)`. Gate 2:
`realtime policy mutation tests: ok`. Both under gawk 5.2.1, mawk 1.3.4 and busybox awk 1.36.1.
Gate 4: `workspace policy: ok`, `workspace policy mutation tests: ok`; `shellcheck` not installed,
not run. Every new fixture shape is `rustfmt --edition 2024 --check` clean. Real sites (debug copy,
not committed, identical under all three awks): `builtins-compiler/src/lib.rs:1076` -> `1075`,
`:1120` -> `1119` (`for _ in 0..available {`); `effect-contract/src/live.rs:366` ->
`364: while remaining != 0 {`; `plan_exchange.rs:377` and `spsc.rs:440` -> no loop;
`graph/src/runtime.rs:900` -> `899: for _ in 0..available {`. The verifier's adversarial shapes:
a01, a02, a17, a05, a13, a14 are now refused; a06, a07, a08, a12, b02 pass as listed limits; a09,
a10, a16, b01 pass correctly; b03 is refused.

**Test value (scratch mutants of the gate run through a copy of the self-test that reports every
drain case and the valid/empty-bodies runs; base copy: no red).**

| Mutant of the gate | Red |
|---|---|
| A lone `{` not resolved to its opener (attempt 1's J1-1) | `wrapped-while-body`, `wrapped-for-bound`, `wrapped-outer-while`, `continuation-led-header` |
| B only a lone `{` resolved, not a `)`/`]`/`}`-led last header line | `paren-led-header` only |
| C a lone `{` resolved without skipping `.method()`/`)`/`]` header lines (the verifier's prototype) | `continuation-led-header` only |
| D only the innermost block's opener resolved | `wrapped-outer-while` only |
| H a header judged by its opener line alone | `wrapped-for-bound` only |
| J D3 refuses only `iter::`-qualified constructors (attempt 1) | `bare-from-fn` only |
| K `available_at_entry` a substring in the binding | `entry-name-in-binding` only |
| L `available_at_entry` a substring in the header | `entry-name-in-header`, `entry-name-in-binding` |
| M `.*` for `[^{}]*` in the header form | `header-past-closed-body` only |
| N `array::from_fn` not a loop opener | `array-from-fn-pop` only |
| O only `let [mut] <count> =` is a binding (attempt 1) | `pattern-rebound-count` only |
| P a lone `{` after a `;`/`}` statement resolved to an earlier line | valid fixture (bare block after a `for` in `plan_exchange.rs`) |
| Q no `core::array::from_fn`/`std::array::from_fn` exception | valid fixture (input fixture's lane arrays) |
| R `(0..)` not an open range | `paren-open-range-outer` only |

Per case: `wrapped-while-body` (the verifier's a01) is red on A, the J1-1 defect itself; A also
reds three other cases, each of which has its own sole catch below. `wrapped-for-bound` is red if
a wrapped header is judged by its first line (H: `for _ in 0..available` hides `+ capacity`).
`wrapped-outer-while` is red if enclosing blocks' wrapped headers go unresolved (D).
`paren-led-header` is red if `) {` is not resolved (B). `continuation-led-header` is red if the
resolution stops at a `.take(64)` line (C). `bare-from-fn` red on J, `entry-name-in-binding` on K,
`entry-name-in-header` on L, `header-past-closed-body` on M, `array-from-fn-pop` on N,
`pattern-rebound-count` on O, `paren-open-range-outer` on R; the two valid-fixture additions red on
P and Q. Each case is green with the mutant reverted.
