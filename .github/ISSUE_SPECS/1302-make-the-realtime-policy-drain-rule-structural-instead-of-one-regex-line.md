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
