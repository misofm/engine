PASS

# #1451 attempt 1: adversarial verdict

- **Reviewed:** `git diff 5b44be979 cc2b4e0c1` on `codex/d15-stream-g`. The commits are `c386b8452`
  (D2), `37f74a07a` (D4 ceilings), `969ed73df` (D3 and re-pins), `44585c80f` (attempt record) and
  `cc2b4e0c1` (root's Amendment 2 completion). I read #1454, #1455 and #1456 only for consistency.
  The spec is binding as amended by Amendments 1 and 2.
- **Method:** I exported `cc2b4e0c1` (head) and `5b44be979` (base) with `git archive` to
  `/tmp/claude-1002/v1451/` and built only there. A third export held the mutants. I did not build,
  edit or check out anything in `/home/bl/misofm/wt-d15-g`.
- **Host:** AMD EPYC 7313P, rustc 1.97.1, Node v22.23.2. The load average was 7-13 because other
  builds were running. I did not time anything; gate 3 uses the implementer's recorded run, as B3
  says.
- **Verdict:** PASS. There is no BLOCKER and no MAJOR:
  - D1's root cause is real. I reproduced it independently.
  - D2 is one source on every target. It removes 2,122 -> 16 iOS calls, and no rendered bit moves.
  - D3 moves no bit.
  - Every restated gate holds.
  - There are three MINOR findings: missing gate-4 evidence about the held V8 rows, and two stale
    or unowned #1018 shapes in documentation. Root should take them up.
- **Evidence:** `/tmp/claude-1002/v1451/evidence/` (small files only; the trees and build
  directories are deleted).

## BLOCKER

None.

## MAJOR

None.

## MINOR

1. **Gate 4 evidence is incomplete. D3 changed three held (unarmed, live-audio) EQ loops in V8,
   and the record does not show it.** Gate 4 (original and restated) requires "the TurboFan
   instruction count ... of each held spill row". The attempt record gives only the armed pair
   (221 -> 219) and "every held row is clean". I ran the same spill checker on my base build
   (whose shipped module reproduces `a87b3ab4...` exactly) and on head:

   | row | base | D2 only (implementer) | head |
   |---|---|---|---|
   | dual depth-1 tail (held) | 107, 0 slots | 107 | **110**, 0 slots |
   | dual depth-2 pair (reported) | 184, 10 slots | 184 | **187**, 10 slots |
   | dual armed depth-1 tail | 127, 2 | 127 | 128, 2 |
   | dual armed depth-2 pair | 221, 11 | 221 | 219, 12 |
   | mono depth-2 pair (held) | 81, 0 | 81 | **79**, 0 |
   | mono depth-1 tail (held) | 49, 0 | 49 | **52**, 0 |
   | mono depth-2 pair, masked (held) | 86, 0 | 86 | 86, 0 |
   | mono armed rows (pair / tail / masked) | 95 / 56 / 100 | 94 / 56 / 100 | 92 / 58 / 102 |

   The live-audio tails gain one `movq`/`movl` + `vmovq` + `vpunpcklqdq` per iteration, and one
   stack `vmovups` goes away. This is TurboFan building the splatted `FLUSH_EPS` `v128.const` inside
   the loop, where base reused the carried word. It is the mechanism the record already names for
   the builtins dual loop (`evidence/loops-diff.txt`). D2 alone leaves every row as base. So
   Amendment 2's premise ("D3 ... adds one V8 slot to one armed EQ loop") is right about slots
   but incomplete about instructions.

   I read B4's "every held row unchanged and clean" as the spill gate's own definition: a held row
   carries zero slots, and the held rows' gate entries are not loosened. B2 says "stay held at zero
   carried slots". Under that reading gate 4 passes. Gate 3 also shows no measurable cost: head
   against base is at most +0.9 % p50. **Fix:** add the held-row table to the attempt record or
   PR. **Root:** confirm the B4 reading. If "unchanged" meant instruction counts, this is a gate
   miss.
2. **`docs/TARGET_MATRIX.md:178-200` (the #1018 register entry) still states the refuted cause and
   the pre-#1451 table.** It says "LLVM lowers a stored `f32x4` splat constant to `bl
   _memset_pattern16`" and lists 3,494 calls in ten crates. This now contradicts
   `scripts/check-cross-targets.sh:100-108`, `scripts/lib/aarch64-known-defects.py:54-71` and
   `crates/lane/src/wide_impl.rs:127-144`. The implementer could not change it: D4 allows edits to
   that file only to remove the entry at zero, and B1 restated the stale wording in two other files
   but not this one. No successor issue (#1452, #1456, #1018) owns a rewrite. **Root:** authorize
   the restatement in this slice, or assign it to #1456 or #1018.
3. **The `silence_skip_settle` outlining is a #1018-motivated shape with no owner and no
   own-merit reason.** The new doc at `crates/lane/src/kernels.rs:321-325` says the outlining "was
   chosen while every constant vector cost a `memset_pattern16` call ... the outlining stays as it
   is". The shape is not in D3's list or in #1452's five undos (the implementer's "For root" item 4
   raised this; Amendment 2 fixed only the wording). D3 keeps the other shapes because "each is
   also a correct form on its own". This doc gives no such reason. **Root:** state the shape's own
   reason (for example, one copy per width keeps the cold settle path out of each caller), or add
   it to #1452.

## NIT

1. `scripts/check-cross-targets.sh:108` is a 149-character comment line. The Amendment 2 edit
   joined the new text onto the old line; the rest of the comment wraps at 100.
2. The record shows the differential's sensitivity only for `REST_EPS`. The implementer's harness
   cannot detect a change to `NONFINITE_LIMIT`. With `limit = 1.0` in every builtins body it moves
   0 of 908 scenarios, because no harness input reaches the range [1, 1e30). This leaves no
   coverage hole: the existing `crates/lane/tests/input_chain_elision.rs` frozen-body tests fail
   on `NONFINITE_LIMIT * 0.5` (4 tests). Record this with the sensitivity claim.
3. The record does not say how it counted the simd128 D2 instructions. My count includes every
   instruction (`wasm-objdump -d`, continuation lines excluded). By that count D2 raises these
   functions:
   - compressor `process_block_mono` +87, where the record lists a decrease (3,830 -> 3,811);
   - compressor `process_block` +24;
   - graph `execute_op` +10 and `fold_cohort` +10;
   - `BuiltinInputBank::new` +11;
   - EQ `process_bank` +2.

   The module total is -480 (`evidence/wasm-func-deltas.txt`). Gate 4 gates only the builtins and
   EQ frame loops, and the EQ's V8 rows are identical between base and D2-only, so this is
   descriptive. Still, state the counting method and name the increases.
4. Two small text errors:
   - The spec's gate 2 says "(today 71 and 48)". The base ceilings were 40 and 47, which the
     record's table uses.
   - The record says "base and D2-only are identical on every row (mono armed pair 95 -> 94
     instructions)". The two halves of that sentence disagree.

## Checked and accepted

- **D1.** I built my own scratch crate (`wide` 1.6.1 only), a one-pole loop with a
  `splat(1e-20)` flush:
  - On `aarch64-apple-ios` it makes one `bl _memset_pattern16` per iteration inside the loop.
  - On `aarch64-linux-android` it makes none, and the array-literal form makes none on either.
  - `-print-changed` shows that `llvm.experimental.memset.pattern` first appears after
    `LoopIdiomRecognizePass`.
  - The unoptimised IR shows `repeat_loop_header`/`_body` in `wide::f32x4::splat`.
  - `wide-1.6.1/src/simd.rs:327` is `transmute([elem; $N])`.

  The root-cause sentence is correct.
- **D2.** It is one source on every target. There is no `cfg`, no pass flag and no
  `#[inline(never)]`, and `wide` is not patched. No product code calls `wide`'s `splat` directly
  any more. x86-64-v3 `builtins` and `parametric-eq` assembly is identical to base except one path
  string.
- **D3.** The removals match the spec. Removing `ArmedRest` outright, beyond the listed field, is
  justified: without its word it was a `&[f32]` plane. It sits in an authorized file. The `&[f32]`
  `skip` is the same as `ArmedRest::skip` was. The one-line `crates/builtins/src/lib.rs:1071` edit
  is A1's named exception.
- **Gate 6 re-pins**, each checked against 16 bytes per input stage:
  - `direct-route.resources.json`: -128 in `builtin_bank_bytes`, `incremental_plan_bytes` and
    `session_plus_plan_bytes`. That is one eight-lane bank, 4 x 8 x 4 bytes; every other field is
    unchanged.
  - `resources.jsonl`: -32 per track in processor and retained bytes (strip preparation plus
    input processor), -16 per track in the largest allocation (1, 4 and 65,537 tracks all
    exact), and the meter and count fields unchanged.
  - `fixture_builtins.rs`: 712 -> 696, 728 -> 712 and 1096 -> 1080. The audit checks each
    against `size_of` at run time.
  - The joined-manifest identity `ea915297...` equals `sha256(fixtures/builtins/v1/MANIFEST.tsv)`.
    In that manifest only the `resources.jsonl` row changed.
  - Commit `969ed73df` names each re-pin with its reason.
- **Amendment 2 completion.** The ceiling of 12 has its reason in the row comment and the module
  doc. The stale wording in `kernels.rs` and `check-cross-targets.sh` is restated correctly.
  `cargo doc -D warnings` failed at base (a private intra-doc link; the qualification lint job runs
  that step) and passes at head.
- **Successor specs.** #1454, #1455 and #1456 agree with #1451's numbers and the STREAMS.md order.
  The GitHub #1451 body equals the spec except for the title line and a trailing newline.
- **Acked-batch question.** Not applicable: this slice touches no queue.

## Test value

No test was added or rewritten. Four lane tests (`filter_ramp_line`, `input_chain_arming`,
`input_chain_elision`, `sanitise_counter`) only lose the removed `constants` field, and no test is
superseded. Nothing that the removed words guarded is now unguarded. The re-splatted constants are
still caught by existing tests, as the mutants below show: `NONFINITE_LIMIT` by the
`input_chain_elision` frozen bodies, `FLUSH_EPS` by the pinned digests and my differential, and
`1.0` and `REST_EPS` by the differential.

## Gates run (head `cc2b4e0c1` unless noted)

- **Gate 1, bits:**
  - **My independent differential** (`evidence/vdiff.rs`, an `effect-compiler` example) covers all
    eight native effects: scalar and 4-/8-lane banks, dual and mono-collapse, padded lanes,
    sidechains connected and unconnected, every supported link mode and quality, and 48k/128,
    44.1k/256 and 96k/32.
    - Inputs include noise, silence long enough to arm the joint flush, impulses into silence,
      subnormals and signed zeros, NaN/inf/3e38 spikes, DC, and decay to exact zero.
    - Parameter points and prepared-target ramps run mid-stream, and state payloads are hashed.
    - Result: **3,840 of 3,840 rendered scenarios identical** base against head.
    - Sensitivity:

      | mutant | scenarios moved |
      |---|---|
      | `Lane::zero` -> `-0.0` | 1,230 |
      | `flush` `FLUSH_EPS` x4 | 316 |
      | `flush_pair` `FLUSH_EPS` x4 | 418 |
      | `frexp` mantissa-mask splat `>> 1` | 595 |
  - **The implementer's harness, re-run:** 908 of 908 identical, and base output byte-equal to
    theirs. The builtins `one` -> 2.0 mutant moves 190.
  - **Pinned artifacts:**
    - The gate 5 effect suite passes, including the determinism digests: 879 passed, 0 failed.
    - `g5_native_digests_match_pins` passes; I re-ran it on a clean tree.
    - `conformance_fixtures --check`, `check-builtins-fixtures.sh` (50 files) and
      `check-graph-determinism.sh` (100/100) pass.
    - The worklet chain's browser-correctness digests agree with the module.
- **Gate 2:** `check-cross-targets.sh` PASS. The counts are builtins 5, host-core 4, soft-clip 1
  and true-peak-limiter 6; the other six crates are at 0 and their rows deleted. No row is raised.
  By function: `BuiltinInputBank::new` 2, `InputStage<f32>::new` 1, `BuiltinFaderBank::new` 1,
  `FaderMuteRampBuiltins::new` 1, limiter `clear_runtime` 3 and `ChannelState::new` 3,
  `SpectrumAnalyzer::analyze`/`analyze_continuous` 2 each, soft-clip `corpus::fill` 1. This matches
  the record.
- **Gate 3:** I recomputed it from the raw JSON. Head against base p50 is -0.90 % to +0.89 %; the
  worst case is the app shape, round 2, at +0.89 %. Every value is at most +2 % in both rounds.
  Every output digest is equal across p0, base and head. The frozen modules are authentic:
  - My base rebuild reproduces `a87b3ab4...`.
  - The benchmarked head (`969ed73df`, `9dde7edb...`) has a code section byte-identical to my
    `cc2b4e0c1` build. Only the data section differs, by panic line numbers.

  I did not re-run the benchmark (B3).
- **Gate 4:** `run-wasm-gates.sh` exit 0. The dual armed pair reads 12 slots at 219 instructions,
  at its ceiling of 12, and every held row is clean (see MINOR 1).
- **Gate 5:**
  - test-debug-a: 1,441 passed.
  - Release `lane`/`math`/`wasm-gates`: 120 passed.
  - Release `audit`/`bench`/`console-workload`: 113 passed.
  - The worklet chain (build, check, expected resources, test) passes.
  - Lane, builtins, workspace and realtime policies pass.
  - The known-defect self-test passes, and the V8 spill self-test runs 33 cases.
  - Workspace clippy `-D warnings`, `cargo fmt --check` and `cargo doc -D warnings` pass.
- **Gate 6:** see above.
- **Not verified:**
  - The AArch64 test legs (`aarch64-debug`, `aarch64-release`) run only in CI, on hardware this
    host does not have. AArch64 was compiled and linted only, through `check-cross-targets.sh`.
  - Native AArch64 timing is not funded.
