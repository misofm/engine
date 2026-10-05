VERDICT: PASS

# #1302 attempt 3: verdict (stream J, commit 4f15b5c7e, base 16a5b920b; whole change ab9620a07..4f15b5c7e)

I reviewed the export `/tmp/claude-1002/v1302/src3` (a git archive of 4f15b5c7e). I did not touch the worktree. My
scratch work is in `/tmp/claude-1002/v1302/a3/`.

The #1302 commit changes only the three authorized paths: `scripts/check-realtime-policy.sh`,
`scripts/test-realtime-policy.sh` and the spec's attempt record. The other paths in `ab9620a07..4f15b5c7e` come from
#1304 and #1237. No Rust source changed. The floors stay at 25 files and 89 regions. The existing
`marked-unbounded-try-pop-drain` case is unchanged.

Attempt 3 replaces the indentation walk with a lexer and a brace stack. That removes the defect class of J1-1 and J2-1:

- Every attempt-2 escape is refused (`a2/c`, `a2/rc`, `a2/n`).
- A grammar fuzzer generated 3,767 rustfmt-parsable loop shapes, each with a pop in the body. The headers mix
  blocks, `match`, `if`/`else` chains, labelled and `const`/`unsafe` blocks, macros, closures, struct patterns,
  or-patterns, casts, `<{ .. }>`, literals and comments, and some loops follow block-like statements with no `;`.
  The pass refused all 3,767 under gawk, mawk and busybox awk.
- 800 bounded drains placed after random statements gave no false positive under any of the three awks.

The findings below are a lexer gap that only rare literals reach, one false finiteness claim, and false positives.
None is reachable through rustfmt layout or a normal drain. There is no BLOCKER and no MAJOR.

## Findings

### MINOR

**J3-1. The lexer does not handle raw C strings or raw strings with more than 16 hashes. It blanks real code, so an
unbounded drain passes.**

The defect is in `scripts/check-realtime-policy.sh`:
- The raw-string opener (`:153-157`) knows `r` and `br` but not `cr`.
- The raw-string closer (`:144`) compares against `substr("################", 1, rh)`, so it never matches more than
  16 hashes.

These three shapes go through the full gate with `realtime policy: ok (89 marked regions in 25 files)` under
gawk, mawk and busybox awk:
- `let _tag = cr"\";` before a `loop { let Ok(record) = control.try_pop() else { break; }; .. }`, with a later
  `// "`;
- `cr#"a"b"#`;
- `r#################"x"#################` (17 hashes; the string never closes, so the rest of the region is
  blank).

Each one is `rustfmt --edition 2024 --check` clean and compiles with rustc 1.97. Attempts 1 and 2, and the spec's
D2 sketch, refused all three, so this is a regression that the new lexer brought in. The gate's comment (`:83`)
says that "string and character literals are blanked". That is false for these literals.

No C-string literal and no raw string with 4 or more hashes exists anywhere in `crates hosts tools`. Normal code
cannot reach this gap, which is why it is MINOR.

*Fix.* About six lines; the prototype is `/tmp/claude-1002/v1302/a3/mut/FIX.diff`.
- Treat `c` like `b`: `(ch == "r" || (ch == "b" || ch == "c") && nx == "r")` and `j = i + (ch != "r") + 1`.
- Build the closing run of `#` per literal (`rhs = rhs "#"` in the opener loop) and compare `substr(s, i + 1, rh) == rhs`.
- Fail closed: after the lexing loop in `check()`, `if (st) hit(n)`. A region must not end inside a literal or a
  comment.

Results with the prototype:
- It refuses all three shapes.
- The real tree passes under all three awks.
- The reporting copy of the self-test shows no red.
- The fuzz corpora give the same results as before.

Add one `brace-in-*`-style case with a `cr"..\"` literal.

**J3-2. The comment says `&mut <fields>` is a finite outer loop. A `&mut` of an iterator is not finite.**
(`:97-99`, `:224`)

This drain passes the full gate under all three awks (`shapes/s04_and_mut_infinite.rs`, rustfmt-clean, compiles).
It pops until the queue is empty, because `available_at_entry` reads the producer index again on each call
(`spsc.rs:420-428`):

```rust
let mut rounds = 0_u32..;
for _ in &mut rounds {
    if control.available_at_entry() == 0 { break; }
    let available = control.available_at_entry();
    for _ in 0..available { /* pop */ }
}
```

The limits sentence (`:112`) lists only `available_at_entry`, `.len()`, `.iter()` and `.iter_mut()` as "taken at
their word".

*Fix.* Add `&<fields>` and `&mut <fields>` to that sentence and to the attempt record's limits. If you drop the
`&mut` form instead, the gate refuses the legitimate `for control in &mut self.controls` (shape `v/v07`).

**J3-3. The gate refuses several plausible bounded drains, and its message says "bound it with available_at_entry"
to a drain that already is bounded.**

The spec's Problem section says this rule exists for the FIFO drains that will remain after the cells: automation,
Observe and structural records. Each shape below is rustfmt-clean, compiles, and is refused under all three awks
(`/tmp/claude-1002/v1302/a3/v/`). The spec's D2 rule accepts all six. Attempt 3 refuses them because it narrows D2.6
and adds a whitelist of finite outer loops.

1. An indexed receiver: `let available = self.controls[lane].available_at_entry();` (`v19`). The entry-count
   chain (`:186-187`) has no index step.
2. A per-lane outer loop over a count: `for lane in 0..self.lanes` (`v03b`) or `for lane in 0..LANES` (`v04b`).
   Only `0..<literal>` and `0..<fields>.len()` are accepted (`:218`). Marked code today already has 5 loops of each
   form. `0..self.lanes` is as finite as `0..2`.
3. A capped count: `let available = control.available_at_entry().min(MAX_RECORDS_PER_BLOCK);` (`v02`).
4. A read-only use between the binding and the loop: `if available == 0 { return; }` (`v01`) or
   `debug_assert!(available <= 1024)` (`v13`). The comment does document this refusal.
5. An attributed bounded loop: `#[allow(..)] for _ in 0..available {` (`v20`). `counted()` strips attributes but
   `classify()` does not.
6. An outer loop over `controls.iter_mut().zip(gains)` (`v15`) or over a bare slice parameter (`v16`).

The valid fixture's `drain_two_lanes` (`test-realtime-policy.sh:263`) drains the same queue twice. The realistic
per-lane form, a different queue per lane, cannot be written there, because shape 1 is refused.

*Fix.* Make these changes:
- Accept `0..<path>` (any closed integer range) as a finite outer loop, on the same terms as `0..<literal>`.
- Allow `[<ident>]` index steps in the entry-count chain.
- Strip a leading `# [ .. ]` attribute in `classify()`, as `counted()` does.
- List what stays refused (read-only uses, caps) in the comment.

Then make `drain_two_lanes` drain `controls[lane]`.

### NIT

- **J3-4. "Finite" is taken at its word.**
  - The gate accepts `for _ in 0..18_446_744_073_709_551_615_usize` around a re-read drain as a finite outer loop,
    but refuses `0..usize::MAX` (`s06`).
  - It accepts `let _: [(); usize::MAX] = core::array::from_fn(|_| { .. })`, which calls the closure `usize::MAX`
    times (`s05`).

  Both pass the full gate. *Fix:* say in the comment that the outer forms are finite, not small.
- **J3-5. The count's binding ignores block items and region-start statements.**
  - In `s10`, a `static available: usize = usize::MAX;` declared after the loop, inside the block that holds the
    loop, shadows the earlier `let available = control.available_at_entry();`. Rust items are visible block-wide.
    A standalone rustc check prints 11 for a `let` bound of 3. The gate passes `s10`, because the single-mention
    span ends at the loop's `{`.
  - In `s08`, a frame-0 `let` from a region that starts inside a function body stays "in scope" after that
    function closes (`:180`, `onstack[0]`). A later function in the same region can then use it.

  Both need deliberate code. *Fix:* list them as limits, or count any item that names `<count>` anywhere in an
  enclosing open block as a mention.
- **J3-6. The headline claim is stronger than the rule.** The headline (`:79`) says "a render-thread drain pops at most
  the records present at block entry". The valid fixture's `drain_two_lanes` pops up to twice that from one queue.
  The "What it proves" list (`:88-105`) is accurate. *Fix:* reword the headline to the bound the list proves, or
  apply J3-3's fixture change.

## The questions asked

1. **Soundness.** Every shape in the caller's list is refused when it is unbounded:
   - lifetimes and char literals (`'a`, `'a'`, `b'{'`, `'\''`, `'\\'`, `'\u{7b}'`, `'"'`, multibyte chars);
   - byte strings and raw strings with hashes (up to 16);
   - nested block comments and doc comments;
   - `r#ident` and `r#try_pop`;
   - labels and labelled blocks in headers;
   - closures in headers;
   - `if let .. else if .. else` conditions;
   - `loop` in a `let`, async blocks, and macros with braces;
   - `<{ N }>` and struct literals in headers;
   - `unsafe` and `const` blocks.

   The set is `/tmp/claude-1002/v1302/a3/t/u01-u20`, all rustfmt-clean and compiled. The fuzzer adds 3,767 random
   headers with no escape. The escapes I found are J3-1 (rare literals), J3-2 (a `&mut` iterator outer loop) and
   J3-4/J3-5 (deliberate code only).
2. **False positives.** None on the real tree.
   - Gate 1 prints `ok (89 marked regions in 25 files)` under gawk 5.2.1, mawk 1.3.4 and busybox awk 1.36.1.
   - In a debug run, all 89 regions end at brace depth 0 and lexer state 0.
   - The unmarked input drain (`builtins-compiler/src/lib.rs:460-505`) passes when I mark it, as do the three
     test-oracle drain shapes.
   - Plausible future drains: see J3-3.
3. **Deviations.** They are within the spec's authority.
   - D2.1/D2.5: the indentation walk is replaced by tokens. This serves D1 (the rule must not be defeated by
     rustfmt), it keeps D2.2-D2.7's claim, and the attempt record documents it. The sketch says that the
     implementer owns the final form.
   - D2.6: D2.6 is an "only if" rule. The exact binding forms, the single mention, and the decrement-first `while`
     add conditions to it, and all six real sites pass.
   - Outer-loop finiteness, D3 inside the pass, and the dropped `fn try_pop` exemption were accepted in earlier
     verdicts.
   - The open-tail cases add an END/BEGIN pair only in their own fixtures, so the main fixture stays on its floors
     (D5).
   - The cost of the D2.6 narrowing is J3-3.
4. **A distinct mutant for each new case.** I built 25 mutants of my own (`/tmp/claude-1002/v1302/a3/mut/`, made by
   `mk.py`, `mk2.py` and `Mp2`). I did not reuse the implementer's mutant scripts. I ran each one through a reporting
   copy of the self-test (`report.sh`). The unmutated gate gives no red. Each mutant reddens exactly what the record
   says; see the test-value list.
   - The five required mutants all reproduce: Ma, Mc, Mh, Mn, Mr.
   - My Mp also reddens `paren-open-range-outer`. A variant with the attempt-2 rule (Mp2) reddens only
     `outer-repeat`.
   - The attempt-1 gate (e3375bc1d) reddens `wrapped-while-body`, `wrapped-for-bound`, `wrapped-outer-while`,
     `paren-led-header` and `continuation-led-header`. So they stay valid J1-1 reproducers.
5. **Tool-failure injection is still red.** I reproduced it independently with my own shim (`mut/inj.sh`).
   - Injecting status 2 into the drain-pass awk gives `realtime drain-bound scan failed (awk status 2)` and rc 1, at
     `partial` 0 and 1.
   - A gate copy that swallows the status (`else drain_hits=""; fi`) prints `realtime policy: ok` for both.
   - The committed counter-mutant passes inside gate 2.
6. **Limits.** The listed limits hold, apart from these:
   - `&`/`&mut <fields>` are missing from "taken at their word" (J3-2);
   - "literals are blanked" is false for `cr` and long-hash raw strings (J3-1);
   - "finite" does not mean small (J3-4);
   - items that shadow the count are not listed (J3-5).

   All other record claims check out:
   - the 35 shapes give 27 refused and 8 passing;
   - the earlier corpora match;
   - 1,125 of the written fixture files are rustfmt-clean (one more, empty-bodies `live.rs`, is not, but that comes
     from the earlier empty-bodies sed);
   - the real sites match.

## Gates run (in the export)

- **Gate 1:** `bash scripts/check-realtime-policy.sh` prints `realtime policy: ok (89 marked regions in 25 files)`, rc 0,
  under gawk, mawk and busybox awk (each forced through a PATH shim). The gate takes 0.5-1.5 s.
- **Gate 2:** `bash scripts/test-realtime-policy.sh` prints `realtime policy mutation tests: ok`, rc 0, under all three
  awks (about 35 s each).
- **Gate 3:**
  - Outermost loop only: case 7's shape gives 0 hits, against 1 on the base, under all three awks.
  - The old one-line regex gate (ab9620a07) against the new self-test: all 44 drain cases are red. The existing
    one-line case and the valid fixture stay green.
- **Gate 4:** `bash scripts/check-workspace-policy.sh && bash scripts/test-workspace-policy.sh` ends with
  `workspace policy mutation tests: ok`, rc 0. `shellcheck` is not installed, so it was not run.
- **Real sites (debug copy; identical under the three awks):**

  | Real site | Innermost loop | Outer loop |
  |---|---|---|
  | `builtins-compiler/src/lib.rs:1076` | `for _ in 0..available` | `controls.iter_mut().enumerate()`, finite |
  | `builtins-compiler/src/lib.rs:1120` | `for _ in 0..available` | `controls.iter_mut().enumerate()`, finite |
  | `effect-contract/src/live.rs:366` | `while remaining != 0` | none |
  | `graph/src/runtime.rs:900` | `for _ in 0..available` | none |
  | `plan_exchange.rs:377` | no loop | none |
  | `spsc.rs:440` | no loop | none |

## Test value (each reproduced: red on the named mutant of mine, green unmutated)

- `drain-while-match-body`: red if a `.` after a closed block starts a new statement (Ma). It is the J2-1
  reproducer.
- `drain-while-match-brace`: red if a head that holds `match` counts as a complete statement, so the `{` after the
  scrutinee opens a new block (Mb).
- `drain-struct-pattern-for`, `drain-if-else-condition`, `drain-as-cast-condition`: red if `in`, `else` or `as`
  after a closed block starts a new statement (Md, Me, Mf), one case each.
- `drain-block-operand-condition`: red if a head that ends on an operator counts as complete (Mc).
- `drain-brace-in-string-header`, `drain-brace-in-char-header`, `drain-brace-in-raw-string-header`,
  `drain-brace-in-block-comment`: red if strings, chars, raw strings or block comments are not blanked (Mh, Mi, Mk,
  Mj), one case each.
- `drain-count-rebound-by-for-pattern`: red if only an assignment between the binding and the loop disqualifies the
  count (Mm).
- `drain-inflated-count`: red if any binding that names `available_at_entry` counts (Mn).
- `drain-while-count-raised-in-body`: red if a `while` body may name its count after the decrement (Mo).
- `drain-outer-repeat`: red if any `for` that is not over an open range counts as a finite outer loop (Mp2).
- `drain-open-tail-last-file`: red if a region still open at the end of input is never checked (Mr).
- `drain-entry-name-in-binding` (rewritten) and `drain-entry-name-in-header`: red if `available_at_entry` matches
  inside a longer name, in the binding (Mu) or in the header (Mw). Each case has its own catch now, which settles
  J2-3.
- The open-tail valid run: red if a region open at a file end runs on into the next file's unmarked head (Mq).
- Valid-fixture additions, each red only on its own mutant:

  | Addition | Mutant | What the mutant does |
  |---|---|---|
  | `drain_lane_pairs` | Ms | an `array::from_fn` closure is never finite |
  | `drain_two_lanes` | Mt | `0..<literal>` is not finite |
  | `retire_lanes` | Mz | no new statement at an identifier after a block |
  | `retire_one` | My | no new statement at `{` after a complete body |
  | the comment that names `available` | Ml | line comments are not blanked |

- Mx (a pop in a loop header is allowed) reddens `marked-unbounded-try-pop-drain`, `wrapped-while-let`, `path-pop` and
  `header-past-closed-body`.

## Adversarial shapes (`/tmp/claude-1002/v1302/a3/`)

| Set | Shapes | Result | Correct? |
|---|---|---|---|
| Attempt-1 `adv/` | a01-a05, a11-a15, a17, b03 | refused | yes |
| | a06, a07, a08, b02 | pass | yes, listed limits |
| | a09, a10, a16, b01 | pass | yes |
| Attempt-2 `c/`, `rc/`, `n/` | c01-e04 and `rc` (3 hits); n01-n05, n08 | refused | yes |
| | n06, n07, n09 | pass | yes; n09 is a listed limit |
| Attempt-2 `v/` and `dangle` | v01-v07 | pass | yes |
| | `dangle` | refused (J2-2 fixed) | yes |
| Implementer's 35 shapes | 27 escapes | refused | yes |
| | 8 bounded shapes | pass | yes |
| Mine, `t/` | u01-u20 (caller's lexer and syntax list) | refused | yes |
| Mine, `shapes/` | s01-s03 | pass | no: J3-1 |
| | s04 | pass | no: J3-2 |
| | s05, s06 | pass | no: J3-4 |
| | s08, s10 | pass | no: J3-5 |
| | s07 (count reassigned after the inner loop, in a finite outer loop) | pass | yes: within the "finite" claim |
| Mine, `v/` | v05, v07-v09, v11, v14b | pass | yes |
| | v01-v04b, v13, v15, v16, v19, v20 | refused | no: false positives, J3-3 |
| | v18 (one pop per lane in a collection loop) | refused | yes: D2 refuses it too |
| Mine, `fuzz/` | 3,767 unbounded | refused | yes (0 escapes) |
| | 800 bounded | pass | yes (0 false positives) |

All rows hold under gawk, mawk and busybox awk.
