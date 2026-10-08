PASS

# #1458 attempt 1 (with the root-ruled fold-in) -- adversarial verdict

Commits under review: `7c43b2513` (one shared harness, parent `8fc41c5b0`) and `88c62e7f5` (fold-in,
parent `ffcbba6b7`; `ffcbba6b7` is #1459 and was not reviewed). Branch `codex/d15-stream-g2`.
Export `git archive 88c62e7f5`, built in `/tmp/claude-1002/v1458` (debug and release). The worktree
was not touched. Evidence (scripts and summaries): `evidence/1458-attempt1/`.

No BLOCKER or MAJOR. The move loses no reach. All 18 #1409 rows are red on the named test and green on
the real code. The two fold-in additions do what the root ruling asked. The findings below go into the
follow-ups commit (MINOR 1 is a code change) or to root.

## Findings

### MINOR 1 -- `follows_law` accepts a premature snap; a one-clause form closes it for every effect
`crates/conformance/src/ramp_endpoint.rs:338` (doc `:13-15`, `:323-328`). In flight, any frame may
jump to the target (`|| same(now.0, target)`), so a ramp that snaps early and keeps counting is green.
I verified this:
- Gate site 5 `final_sample = remaining <= 60`: green on the harness.
- Limiter site 9 `stepping = remaining > 40`: green on the harness.

The record (spec `:212`) says the gap exists. But it came from dropping the exact `remaining - 1` model
after the gate's `f32` encoding broke it, not from a closed design. An encoding-agnostic form exists:
it only needs zero versus non-zero.

`same(now.0, ramp_toward(current, step, target)) || (same(now.0, target) && now.3 == 0)`

I applied it in the export:
- It is green on all seven effects' real code (debug).
- It is red on both premature-snap mutants: gate frame 4, limiter frame 23.

The defect class does not escape the suite today. The gate's `reset_seeds_...`, `scalar_and_bank_...`
and g5 case 124 catch it, and the limiter's `the_lane_ramp_reproduces_the_scalar_ramp_bit_for_bit`
catches it. So this is not MAJOR. Under the owner principle, do not accept the gap: fold the clause in
and update both docs.

A one-sample early snap at the end of the ramp is a different case. It changes no bit with these moves,
because each move's clamp holds the word at its target from about frame 33. It is green even with the
tighter clause. Gate 1 catches it in the shared kernel: five effect-runtime tests are red on
`1 | 2 =>`. Closing it at gate 2 would need moves whose snap closes a real gap, and new moves are a
non-goal. This part of the hole does not matter.

### MINOR 2 -- The correction of #1409's record is accurate but incomplete; give root the cause and the other catchers
Spec `:177` and `:194-200`. I confirmed that both rows are green on the old copies at `8fc41c5b0`. The
`src` of delay, soft-clip, effect-runtime and lane is identical to `88c62e7f5`. Missing from the record:
- **Cause, delay row.** `f6f599d84` (#1409 attempt 2) set the delay time to 1 ms. I set the old copy
  back to 250 ms, and the old mix move `0x3f7fffa0` is red again on `left output`. With a live tap, the
  unclamped mix word's output bit at the crossing frame rounds away. "Not investigated further" can now
  be replaced with this cause.
- **Cause, soft clip row.** `00a0445c5` (#1452 undo 5). Its diff replaces the settled branch's
  `drive.add(c.drive_step)` with a hold, so the inverted choice freezes the words where it used to
  overshoot. The record's explanation is correct.
- **Neither mutant escapes the suite.**
  - The `advance_block` mutant is red in gate 1
    (`effect-runtime` `every_statement_of_the_law_stays_inside_its_endpoints_and_agrees`), as #1409's
    own row says.
  - The soft clip D5 inversion is red in `crates/soft-clip/tests/ramp_law.rs` (three tests).
  - So the fold-in restores gate-2 reach. It is not a first catch, and the test-value sentences below
    are worded to match.

**For root (outside #1458's paths).** #1409's spec is still at
`.github/ISSUE_SPECS/1409-keep-every-effect-parameter-ramp-inside-its-endpoints.md`; the rows are at
`:505` and `:516`. I recommend a pointer in that record, or on GitHub #1409 once the spec leaves:
"gate-2 reach of row 505 lost at `f6f599d84` and of row 516 at `00a0445c5`; both rows stayed red
elsewhere (gate 1; soft-clip `ramp_law.rs`); gate-2 reach restored by #1458 `88c62e7f5`."

### NIT 1 -- An over-width doc line
`crates/conformance/src/ramp_endpoint.rs:15` is 106 columns (`max_width = 100`). #1409's batch
follow-up rewrapped lines like this one.

### NIT 2 -- The delay test docs still say one move per word
`crates/delay/tests/ramp_endpoint.rs:5` ("Each word has one in-domain move, found by #1409's scan") and
`:58` ("One overshooting move per word"). There are now five moves for four words, and the fifth came
from #1458's scan.

### NIT 3 -- The record's line count is off by one
Multiband after the move is 180 lines, not 179 (spec `:135`), so the total is 787, not 786. Every other
count matches.

### NIT 4 -- The scan counts cannot be reproduced from the record
Spec `:220-228`. The 19 interior points and the "5% geometric spread" are not specified, and the script
is deleted, so 51,198 / 20,227 / 17,211 cannot be derived again. The claims that matter can be
reproduced: the new move is red on the mutant, and `0x3f7fffa0` is green on it. Either specify the
generator exactly or label the counts as one-time PR evidence. Not committing the scan is correct.

### NIT 5 (root) -- The spec's Test value section is out of date
Spec `:93` says "No new test behavior". The fold-in adds two new behaviors. The brief is root's, so
root should amend the line.

### NIT 6 (root) -- D1's wording does not match the implementation
D1 (`:43`) says "behind the feature the seven crates already enable". The module is not behind
`realtime-audit`. The record (`:108`) gives a sound reason: no other shared harness is gated, and the
feature only forwards engine render-audit instrumentation. Root may amend D1's wording.

## Judged questions

1. **Reach and specialisation.** No loss. `WORDS` and `MOVES` are identical to the old copies (compared
   token by token), except the one new delay move. The old harness bodies were identical across copies,
   apart from the snapshot signature and `.processor` placement.
   - The delay's `values_with` became `rest: &[(0, DELAY_TIME_MS)]`. It is applied before the move's
     `start`, in the same order as before. All seven site-7 rows are red as before.
   - The bank's resting lanes use `values(rest, None)`. That is identical to the old descriptor
     defaults, because every banking effect has `rest: &[]`.
   - The tail bound comes from the same `admit` + `registry.tail_bound` path as before
     (`tail_bound_of`).
   - A connected run on an effect without a port now panics instead of running unconnected. This is
     stricter, and only the compressor runs connected (unchanged).
   - Test names and counts are unchanged (13).
2. **`follows_law`.** It is correct on all seven effects: green in debug and release on x86_64, with a
   mixed `remaining` encoding (the gate writes `f32` bits, `lib.rs:740`; the others write `u32`). It has
   real reach: the motivating soft clip mutant goes red at frame 1, and so does a frozen gate ramp.
   - The early-snap gap mostly matters for a premature mid-ramp snap. See MINOR 1: closable with one
     clause, and caught elsewhere today.
   - The end-of-ramp early snap is unreachable with these moves, and gate 1 catches it.
   - Not run: AArch64 (CI only). The scalar `ramp_toward` is add/min/max with no FMA, and no move
     involves a signed zero, so I expect no false red there.
3. **The delay move.** It was chosen soundly: one ulp from an existing move, with the same word,
   target and `whole` flag.
   - It is red on `left output` in debug and in release, and green on the real code.
   - The first mix move is green on the mutant.
   - In the delay crate, only this move catches the mutant: the full `-p delay --all-targets` run has
     one red test, and the g5 corpus stays green.
   - Its catch goes through an output bit at the crossing frame, so it depends on the signal and on the
     1 ms tap. `f6f599d84` shows that such a catch can disappear silently, so any future edit to the
     delay time or the signal must run this mutant again.
   - The scan cannot be fully reproduced (NIT 4). The move and its evidence can.
4. **The #1409 correction.** Accurate, but incomplete (MINOR 2). The pointer is root's.
5. **The `randomized.rs` probes.** Not moving them and not running them again is in line with the spec:
   non-goals list "the `randomized.rs` probes", and Hazards say `randomized.rs` is not edited. Neither
   commit touches them or any render code. Their targets compile (workspace clippy `--all-targets`).
   The randomized tests of gate, limiter, multiband, compressor and soft-clip were green in my
   full-suite mutant runs, apart from failures caused by the mutants.

Scope and ownership: every changed path is in the spec's authorized list (conformance `src`, the seven
test files, the spec). STREAMS stream G slot 15. No render code changed. No queue, so the acked-batch
question does not apply. The naming and version-suffix rules hold.

## Test value

- **The 13 rewritten tests (same names, one harness).** A harness fix or defect applied to one effect's
  copy only, which would weaken that effect's gate with nothing going red. Now there is one copy. The
  18-row re-run shows that every gate kept its catches.
- **`follows_law` (in every `every_ramped_word_stays_inside_its_endpoints`).** A ramp word that holds
  while in flight in one effect's render turns it red, for example soft clip's output-gain or mix update
  losing its step in the `RAMPING` loop (verified for both words). Every other soft-clip test stays
  green, `ramp_law.rs` included. Only the g5 corpus digest (case 113) also moves, and a deliberate re-pin
  of that case would absorb it. The motivating D5 inversion is also red in `soft-clip/tests/ramp_law.rs`,
  so it is not the unique catch.
- **The new delay mix move `0x3f7fff9f -> 1.0`.** An unclamped block-start word reaching the delay's
  output (`advance_block`'s first word as `chunk_of` consumes it) turns it red. No other delay test and
  no corpus case catches it. Gate 1 catches the shared-kernel form, so this move's unique catch is a
  delay-side regression of the block-start word.

## Mutation re-run (`evidence/1458-attempt1/all-debug.txt`)

Each mutant was applied alone to the export and the named crates' `--test ramp_endpoint` run (debug),
then the file was restored. Afterwards the export matched `git archive 88c62e7f5` (`diff -r` clean).

| # | Mutant | Red on |
| --- | --- | --- |
| 1 | site 2 `next_value` -> `current += step` | compressor connected threshold f33; delay feedback f33; transient attack amount f48 (law check) |
| 2 | site 2 `advance_block` first word -> `current + step` | delay mix `0x3f7fff9f` `left output` (also release) |
| 3 | site 4 `advance_where` -> `add` | compressor unconnected threshold f33 |
| 4 | site 4 gather target from lane `W-1-lane` | compressor bank test threshold f0 |
| 5 | site 5 gate prologue -> `add` | gate threshold f33 |
| 6 | site 6 `run_segment` -> `add` | multiband low threshold f33 |
| 7 | site 6 `Side::segment` target from `W-1-track` | multiband bank test low threshold f0 |
| 8 | site 7 `ramp_word` -> `value + step` | delay feedback, final snapshot |
| 9 | site 7 D5 inverted (`if !ramping`) | delay feedback, final snapshot |
| 10 | site 8 drive -> `add` | soft clip drive gain f33 |
| 11 | site 8 D5 inverted | soft clip drive gain f1 (law check; also release) |
| 12 | site 8 `target_vector` from `W-1-lane` | soft clip bank test drive gain f0 |
| 13 | site 9 limiter -> `add` | limiter limit coefficient f33 |
| 14-18 | site 7 left damping / left feedback / right damping / right feedback / cross position | delay, final snapshot (damping, feedback, damping, feedback, cross feedback) |

All 18 are red. The 13 tests are green on the real code.

## Gates run

| Gate | Result |
| --- | --- |
| Gate 1, debug: `cargo test --locked -p compressor -p delay -p gate-expander -p multiband-compressor -p soft-clip -p transient-shaper -p true-peak-limiter --test ramp_endpoint` | 13 passed |
| Gate 1, release: the same with `--release` | 13 passed |
| Gate 2: the 18 rows above, plus the two fold-in mutants in release | all red |
| Gate 3: per crate, only the docs, `WORDS`, `MOVES`, `ENDPOINTS`, `DELAY_TIME_MS` and one harness call per test remain | holds |
| Gate 4: `cargo clippy --locked --workspace --all-targets -- -D warnings` | exit 0 (169 crates) |
| Gate 4: `cargo fmt --all -- --check` | exit 0 |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p conformance --no-deps` | exit 0 |
| `check-workspace-policy.sh`, `check-conformance-boundaries.sh`, `check-realtime-policy.sh` | exit 0 |
| `cargo test -p conformance` | pass |
| `g5_native_corpus` (release, `math/lane`) on the real code | 7 passed |

Extra probes, all reverted:
- The old copies with the two fold-in mutants.
- The 250 ms delay copy.
- Gate 1 against `advance_block`.
- The full soft-clip suite against D5.
- Frozen-ramp mutants against the full gate, limiter, multiband and compressor suites, and g5.
- Single-word soft clip freezes.
- Premature and early snaps against the current and the tightened check.

Not run: AArch64, wasm and the browser legs. No engine or render code changed, so
`check-cross-targets.sh` and the worklet chain do not apply.
