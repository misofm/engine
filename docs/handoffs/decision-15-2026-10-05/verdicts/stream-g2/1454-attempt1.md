PASS

# #1454 attempt 1 -- adversarial verdict

Verifier: opus-xhigh, 2026-10-06. Reviewed `git diff f3956e63c a249b3cc1` (FramePairs, `crates/lane/src/kernels/builtins.rs`) and `git diff 235b5fe4c b5cfd2b60` (the Amendment 2 B2 fold-in, `crates/lane/src/kernels.rs`), built at `b5cfd2b60` from `git archive` exports under `/tmp/claude-1002/v1454/` (never in the worktree). Spec read at `b5cfd2b60` with Amendments 1 and 2 and the Attempt record. Both changed files are stream G's (`STREAMS.md`, Stream G "Owns: `crates/lane`") and inside the spec's Authorized paths as widened by B2.

No BLOCKER, no MAJOR. Two MINORs and three NITs for the follow-ups commit.

## Findings

### MINOR 1 -- the dual armed tail's ratchet is left at 2 with nothing routed to root

`scripts/check-web-audioworklet-v8-spill.py:229-230` (`ceiling=2`) and its docstring at `:129` ("dual tail 2, two general-purpose words"). At head the row carries 1 slot and the gate prints `1 carried slots: lower its ceiling` (reproduced). The gate's own rule (`:129-135`) is that a ceiling exists to make a rise visible and "fewer passes and asks for the ceiling to be lowered"; at 2, a return to two carried slots is now invisible. The script is not an Authorized path (spec "Authorized paths", B2 names two functions only), so the implementer was right not to edit it, and the record says so (spec `:379-381`). But the record neither asks root nor names a follow-up, so as delivered the slack has no owner -- an accepted gap under the owner's no-shortcuts rule. Action: root authorizes the path and the batch follow-ups lower the row to `ceiling=1` and update the docstring's dual-tail sentence (measured here: one general-purpose word, `[rbp-0xe8]`, against two at base, `[rbp-0xa8]` and `[rbp-0xb0]`).

### MINOR 2 -- gate 3's increases are named but the x86 one is not analysed, and the simd128 one's frequency is misstated

Spec `:384-392`. Gate 3 allows an increase when it is named, and the root's standing rule is that codegen decides where no benchmark document reaches a site. No benchmark covers native `process_bank_mono<f32x8>`, so "+42, named, not analysed further" leaves the question open. I analysed both increases. Neither is a render-cost regression on a real path, but the record should carry this analysis:

- **x86-64-v3 `process_bank_mono<f32x8>` 4,580 -> 4,622 (reproduced exactly).** The growth is static. With the `while frames_left` loop, LLVM no longer reduces the `frames < D` fallback of `svf_cascade_skewed_impl` (the interleaved form at `D = 2`, which runs at most one frame) to straight-line code. Head has four extra innermost loops that load and store the same frame (`(%r8,%rax,4)`): 55, 62, 63 and 67 instructions, about +247. Code elsewhere in the function shrinks to offset them. The skewed steady-state loops (store at `-32(...)`) change by -2 / +2 / +1 / +2 per frame (unarmed 60 -> 58, masked 62 -> 64, armed 72 -> 73, armed masked 75 -> 77). The depth-1 tails change by +1 / +3 / -1 / 0 (35 -> 36, 37 -> 40, 41 -> 40, 44 -> 44); the unarmed tail drops its three `64(%rsp)` stack loads. These are small, mixed changes on a host that is a benchmark proxy, not a shipping target.
- **simd128 module +135 (reproduced: +232 from four `core::array::try_from_fn` instantiations, EQ -48 / -3 / -46).** All four call sites are in the dual `process_bank` (func 1449), in the `UnarmedRest` (live-audio) instantiation of `interleave`. The `from_fn` calls sit inside `for pass in 0..pairs` (`crates/parametric-eq/src/lib.rs:2616`, `:2620-2621`, `:2624-2625`). So each function runs once per depth-2 pass, up to three times per block with `EQ_SECTION_COUNT = 6`, not "once each ... per block" (spec `:387-388`). The cost is about twelve calls per block. That is small beside the dual pair loop's -6 instructions per frame per pass (187 -> 181) and the dual tail's -9 per frame. It is still a per-block cost, not a per-frame cost.

### NIT 1 -- the attempt-1 TurboFan table says "every dual loop" but omits the identity-chain loops FramePairs also changed

Spec `:303-321`. In `InputStage<f32x4>::process` the four dual identity-chain loops (`identity_chain_block`, `identity_chain_ramp_block`, two inlined copies) move 40 -> 41, 86 -> 87, 51 -> 39, 84 -> 85 (V8, my listing through the spill gate's `analyse`). Three +1s are unnamed. No slot changes (0 -> 0, 6 -> 6). The required simd128 and x86 counts are complete at function level, so the gate's letter holds.

### NIT 2 -- two V8 changes outside the rows go unrecorded

Spec `:363-381`. (a) The held mono depth-2 pair, which runs on live audio, goes 79 -> 80 instructions. Its loop-invariant stack reloads also go 10 -> 13 per frame (no carried slot, so the row is clean). The record names the +1 but not the reloads. (b) The unheld dual masked armed depth-2 pair goes 14 -> 15 carried slots. It is not a row, it runs only on silent tails, and "no slot gained" is true of the rows. Record both so the fold-in's V8 picture is complete.

### NIT 3 -- `frames_left`'s doc names the dispatchers, not the loops

`crates/lane/src/kernels.rs:104` ("[`svf_cascade_interleaved_impl`]'s frame loop") and `:115` ("[`svf_cascade_skewed_impl`] keeps its indexed form"). The loops are in `svf_cascade_interleaved_form` and `svf_cascade_skewed_form`.

## Points judged

1. **`svf_cascade_skewed_form` left unchanged: adequate.** A1's mechanism, the `v128.const` rebuilt after an early exit, is absent from every pair loop at base and at head (V8: zero `vpunpcklqdq`/`vmovq` in every dual and mono pair loop). It was present only in the depth-1 tails: two rebuilds each at base, zero at head, dual and mono, masked and armed alike. So A1 needs no skewed change. The tried rewrite (`w1454/e2-skewed.diff`, read) made the held mono rows fail closed and gave a ceiling-0 row a slot. Gate 3 forbids both. B2 authorizes a skewed change but does not require one. Its successor clause is conditioned on the dual tail's rebuild, which the interleaved change alone removes. The per-step bounds branch that remains is pre-existing and is now stated at the function (`kernels.rs:760-761`, `:115-117`).
2. **Ratchet: not lowerable in this slice, but must be routed.** See MINOR 1.
3. **Naming without cause:** acceptable for simd128, whose cause is given and verified, apart from the frequency slip. Not enough for x86 under the codegen-decides rule. See MINOR 2. Neither is a per-frame regression on a real path. Live-path per-frame changes at head: browser dual tail -9, mono tail -9, dual pair -6, mono pair +1 (+3 invariant reloads); x86 pairs -2..+2, tails -1..+3.
4. **FramePairs and the frame splitting are sound for every length.** There is no `unsafe` in either commit. `FramePairs::new` cuts both planes to `min(len_l, len_r)`, and `min(a, b) / W == min(a / W, b / W)`, so it yields exactly the zip's pairs. A trailing partial frame and zero frames (`span < W`) yield nothing. Both planes advance by `W` from equal lengths, so the right split is always in bounds. `frames_left` checks every `io` plane and, when `ARMED`, every rest plane before any split. Every plane is cut to `span` first, so the loop runs exactly `frames` times. The unarmed rest planes (`[&[]; S]`) are never split. Termination needs `S >= 1` and `L::WIDTH >= 1`. `WIDTH` is 1, 4 or 8. `S = 0` panics in `stream_planes` (`rest[0]`) before the loop, at base and at head alike. My differential exercises partial frames, unequal plane lengths, blocks longer than the span and frames 0..9, 13, 31, 64, 128.
5. **No test required: confirmed by mutation.** Every plausible defect of the new shapes is caught by an existing committed test:

   | mutant | caught by |
   |---|---|
   | M1, `FramePairs::new` one frame short | builtins unit tests (`elision_tests::appended_cases_execute_identity_and_mixed_dispatch_with_full_controls`, `derive_new_scalar_expectations`, `filter_prefix_ends_at_countdown_for_dual_and_mono_paths`, `post_ramp_symmetry_extracts_each_word_once`) and `g5_native_digests_match_pins` |
   | M2, `next` uses `<=` | the same builtins unit tests and `g5_native_digests_match_pins` |
   | M3, `frames_left` uses `>` | lane `g2_bounded_cascade_is_the_cascade_and_judges_what_it_stores`, `g2_interleaved_cascade_equals_a_chain_of_blocks`, `g2_skewed_cascade_equals_the_interleaved_cascade`, `g2_skewed_cascade_arms_each_section_on_its_own_frame` (g5 stays green on this one) |
   | M4, armed rest plane not advanced | lane `g2_skewed_cascade_arms_each_section_on_its_own_frame` |

   The spec's Test value statement ("held by existing gates") is true.

Gate 2 (B1): the record states both rounds, P0 r1/r2, base vs P0, head vs P0 and head vs base, CPU pin, flags, interleaving, load 5.35 -> 4.21 on 32 threads, and equal digests (spec `:283-301`). Not re-run. The timed head is attempt 1's module `1ce4a5b4`. The fold-in's module (`01ea8870...`) is untimed under B1.

## Test value

No test added, rewritten or deleted. None is required: see point 5.

## Gates run (at `b5cfd2b60` unless stated)

- **Gate 1, my own differential** (`/tmp/claude-1002/v1454/v1454diff.rs`, lane example built against `f3956e63c` and `b5cfd2b60`). 42,336 scenario lines, three seeds, f32 / Simd4 / Simd8.
  - Cascades: all six cascade entry points; S 1-2; D 1-4; armed and unarmed; extra words 0, W-1, W+3; hostile words (signed zeros, subnormals, random-payload NaNs, infinities, 1e30, zero runs).
  - Chains: the five dual input-chain entry points (both filter-ramp trim modes; all-identity and mixed elision plans) with unequal plane lengths.
  - Results: the chain lines are **raw-bit identical**. The cascade lines are **class-A identical** (NaN fold as `dsp_reference::class_a`). 2,523 cascade lines differ only in NaN sign/payload, every one with NaN output, including the unchanged skewed form. That is codegen operand commutation, which the class-A rule folds; no rendered plan reaches it, because the input stage sanitises first.
  - Sensitivity: M1-M4 move 7,616 / 6,507 / 23,897 / 10,175 lines.
  - Pinned artifacts: `g5_native_digests_match_pins`, `BUILTINS_DIGESTS` / `E9_DIGESTS` (determinism tests), `conformance_fixtures --check`, `check-builtins-fixtures.sh` (50 files) and `check-graph-determinism.sh` (100/100) all pass.
- **Gate 3.**
  - `bash scripts/run-wasm-gates.sh` exits 0 (native + simd128 + V8 EQ loops, AMD EPYC 7313P). Every V8 row equals the record's table, base `235b5fe4c` and head.
  - InputStage V8 loops `f3956e63c` -> `a249b3cc1` reproduce the record (222/5/16 -> 196/3/11, ...).
  - simd128: fold-in +135 reproduced. Attempt 1 is -535 by my count against the record's -529: the same -58 `ZipImpl::new` (5 calls at base, confirmed), and InputStage -477 against -471, a counting-method difference.
  - x86: builtins -134 / -114 / -301 and parametric-eq -9 / +2 / +42 reproduced exactly.
  - Module SHAs: base `aa991860...` and fold-in base `b387e216...` match the record.
- **Gate 4.** `check-cross-targets.sh` PASS; `builtins` 5 `memset_pattern16` calls = its row (5); `parametric-eq` 0.
- **Gate 5.**
  - DSP-crate `cargo test` with the spec features: 441 passed.
  - DSP doctests (lane, builtins, parametric-eq): pass.
  - Release `lane` / `math` / `wasm-gates`: 123 passed.
  - `test-debug-a`: 1,458 passed.
  - Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh`: all pass.
  - Lane, builtins, workspace and realtime policies; `cargo doc` with `-D warnings`; clippy `-D warnings`; fmt: all pass.
  - Not run: AArch64 legs (CI only).

Evidence kept in `/tmp/claude-1002/v1454/ev/` (gate logs, differential outputs, mutant results, spill outputs, x86 loop extracts) and the harness and scripts in `/tmp/claude-1002/v1454/`. Build trees, target directories and module twins deleted.
