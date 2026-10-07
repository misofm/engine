# Bring the builtins dual loop back to P0's instruction count

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`).
Filed 2026-10-06 by root order (Amendment 2 of *Let the builtins splat their chain constants
without iOS memset calls*, #1451). Code anchors verified on `codex/d15-stream-g` at `44585c80f`
(#1451 attempt 1's record). Ordered directly after #1451.

## Product outcome

The browser's builtin input chain costs no more than it did before the SVF joint flush. Since
*Flush the SVF jointly so builtin and EQ filters reach exact rest* (#1328), V8 (TurboFan) compiles
the builtins' dual-channel stationary frame loop to 219-222 instructions in 5 blocks, against 196
in 3 blocks for **P0**, the per-word build. On the three 64-track browser documents the shipped
AudioWorklet module is +2.0 % to +4.9 % p50 over P0, above the owner's 2 % allowance
(`two-percent-allowance-for-better-code`). After this slice the loop is back near P0's shape, and
every browser document is within 2 % of P0 with no rendered bit moved.

## Context

- **The measured gap (#1451 attempt 1, one invocation, not retried).**
  `scripts/web-mixing-automation-benchmark.mjs run`, one warmup and two measured rounds per module,
  interleaved, `taskset -c 31`, `node --no-liftoff`, load 1.98 -> 2.18 on 32 threads; every output
  digest equal. Modules: P0 (SHA-256 `1968517eee5f2600e5a49b1c934488740e541b81f9b5a4fc052720c39b70e474`),
  base `5b44be979` (carried chain constants, `a87b3ab4...`), head `969ed73df` (#1451 D3, splatted
  constants, `9dde7edb...`).

  | p50 | P0 r1 / r2 (ns) | base vs P0 | head vs P0 |
  |---|---|---|---|
  | mono console, quiet | 151,719 / 152,430 | -0.6 % / -1.1 % | -0.8 % / -1.4 % |
  | mono console, restated | 153,913 / 154,184 | +0.6 % / +0.5 % | +0.1 % / +0.1 % |
  | mono console, automated | 158,652 / 158,020 | +0.0 % / +0.4 % | -0.3 % / +0.3 % |
  | sixty-four-track console | 233,474 / 233,364 | +2.4 % / +3.1 % | +2.7 % / +2.1 % |
  | app shape | 148,352 / 147,851 | +3.6 % / +3.9 % | +3.9 % / +4.9 % |
  | bus-and-send console | 335,719 / 336,921 | +3.0 % / +2.4 % | +3.1 % / +2.0 % |

- **The loop (#1451 attempt 1, TurboFan listings analysed with the V8 spill gate's own loop
  finder, `scripts/check-web-audioworklet-v8-spill.py`'s `analyse`/`describe`).** The loop shaped
  `2 streams, masked, unarmed, vmulps=30 vaddps=38 vsubps=8`; P0 has it inlined in
  `BuiltinInputBank::process`, base and head in `InputStage<f32x4>::process`:

  | module | instructions | blocks | carried stack slots |
  |---|---|---|---|
  | P0 | 196 | 3 | 12 |
  | base (carried words) | 219 | 5 | 16 |
  | head (splatted) | 222 | 5 | 16 |

  Op histogram, P0 / base / head: indexed `vmovdqu` memory loads 4 / 26 / 26 (base reloads the
  section coefficients each iteration, for example `vmovdqu xmm0,[r8+0x720]`); a bounds branch
  (`cmpl`, `jc`, `jz`, `testl`) 0 / 1 / 1 each (base: `testl r14,r14; jz` at the loop head); stack
  `vmovups` 52 / 42 / 41; head adds 3 x (`movq`, `vmovq`, `vpunpcklqdq`), TurboFan building a
  `v128.const` inside the loop. The arithmetic is the same in all three. **Cause, as far as
  measured:** the loop's structure since #1328 (two more blocks and 22 more memory operations), not
  the chain constants: #1451 showed that splatting them in place of carrying them is flat against
  base (+0.3 % / +0.9 % on the app shape) and that carried words are no dearer in V8 than splats.
- **The structure since #1328 (A9).** `input_chain_block`
  (`crates/lane/src/kernels/builtins.rs:597-615`) tests once per block whether some lane's silence
  counter can arm (`channel_arms`, `:516-518`) and runs `input_chain_block_body` (`:620-690`) in one
  of two forms, armed (`silence_step` per frame, joint flush) or unarmed (per-word flush, the
  counter advanced once per block by `lane::kernels::silence_skip_block`). The two forms are the
  same bits. P0 had one body with the per-word flush and no counter. The mono, elided and ramp
  variants (`input_chain_block_mono` `:1935`, `input_chain_block_elided` `:1638`,
  `input_chain_block_mono_elided` `:2083`) follow the same pattern. The owner's call site is
  `InputStage::process` (`crates/builtins/src/lib.rs:1766`), stream A's file.
- **P0.** #1328's per-word build: the tree before #1328's joint flush with `flush_pair` written as
  two `flush` (the #1328 attempt-5 verifier's recipe), the module #1451 also timed (SHA above). Its
  dual loop is 196 instructions in 3 blocks.
- **Not the cause, already done.** #1451 removed the iOS `memset_pattern16` cause (2,122 -> 16
  calls) and returned the chain constants to splats; the EQ's V8 rows are #1451's.

## Decisions frozen for this slice

- **D0. Root decision (2026-10-06).** The decision-15 root coordinator, under the owner's
  no-shortcuts delegation (`no-shortcuts-correctness-first`), ruled in #1451 Amendment 2 that the
  browser gap to P0 is the dual loop's structure and gets its own issue, this one, with the gate
  "p50 within +2 % of P0 on all four browser documents, bits identical".
- **D1. Codegen evidence first.** Before any fix, name why TurboFan reloads the section
  coefficients by indexed loads and adds the bounds branch in the post-#1328 loop. Compare the
  `simd128` (wasm) body of the loop in P0 and at the base of this slice (instruction counts, the
  loads inside the loop, where `InputStage::process` inlines the body) and the TurboFan listing.
  Record the cause in one sentence in the attempt record. If no cause in `crates/lane` is found,
  stop after D1 and return the evidence to root.
- **D2. Fix in the lane kernels, one shape for all targets.** Change `input_chain_block` /
  `input_chain_block_body` (and the variants that share the structure, only if D1 shows the same
  cause there) so that the stationary dual loop reads its coefficients from registers or
  loop-invariant locals as P0 does. No target-specific code (`no-target-specific-code`), no
  `cfg`, no `#[inline(never)]` added to the frame loop, no change to the #1328 law (two forms,
  arming, counter, joint flush). If the fix needs a change in `crates/builtins/src/lib.rs`, stop
  and ask root (stream A's file).
- **D3. Class A.** No rendered bit moves on any target; the two forms stay the same bits.
- **D4. P0 is #1451's P0** (Context). If that module is not available, rebuild it with the recipe
  and confirm its dual loop is 196 TurboFan instructions in 3 blocks before timing; if it is not,
  ask root. The base is not chosen after the measurement.

## Deliverables

1. D1's evidence and cause sentence in this spec's attempt record.
2. The D2 change, with a doc at the changed construction that states why it is written that way.
3. The PR evidence: the base-versus-head differential (gate 1), the timing table (gate 2), the
   TurboFan, `simd128` and x86-64-v3 instruction counts of the loop (gate 3).

## Authorized paths

- `crates/lane/src/kernels/builtins.rs` (`input_chain_block`, `input_chain_block_body`,
  `channel_arms` and, per D2, the variants sharing the structure)
- `crates/lane/src/kernels.rs` (`silence_skip_block`, `svf_step_when`, only if D1 names them)
- `crates/lane/tests/` only where a test names a changed item
- this spec

## Non-goals

- Any change to the #1328 law, to state records, sealed sizes or rendered bits.
- The EQ's loops and V8 rows (#1451), the multiband compressor (its own issue), the iOS ratchet
  workarounds (#1452).
- Native AArch64 timing (CI-only, no funded runner).
- Rejected alternatives:
  - Drop the two forms and go back to one body: the #1328 follow-up measured the one-body chain at
    about 3 points more p50 than the two forms on the 64-track documents.
  - Tune V8 flags or the benchmark: the gate is the shipped module under the standard runner.

## Hazards

- `crates/lane/src/kernels/builtins.rs` is a hot file shared with streams A, B and E
  (STREAMS.md); #1452 also edits it (the filter-ramp countdown). This slice touches only the
  stationary chain bodies.
- A change in register pressure moves V8's EQ rows too; gate 3 checks every held row.
- `check-lane-policy.sh` refuses some spellings inside `crates/lane`; use the crate's accepted forms.
- Another implementer may be working in the same worktree; commit exact paths only.

## Objective gates

1. **No rendered bit moves.** A base-versus-head differential over the scenario set #1451 used
   (908 scenarios: every builtin input-chain entry point at `f32`, `Simd4`, `Simd8`, and the EQ),
   or a superset: every output identical. PR evidence, not a committed test. Every pinned artifact
   is unchanged: `g5_native_digests_match_pins`, `BUILTINS_DIGESTS`, `E9_DIGESTS`, multiband
   `DIGESTS`, `conformance_fixtures --check`, `check-builtins-fixtures.sh`,
   `check-graph-determinism.sh`.
2. **Browser cost within 2 % of P0.** `scripts/web-mixing-automation-benchmark.mjs run` on the
   shipped module of head and of P0: one warmup and two measured rounds each, interleaved, pinned
   to one CPU, host load average recorded before and after; one invocation, not retried. p50 per
   render of each of the four browser documents -- the mono console (its quiet, restated and
   automated arms), the sixty-four-track console, the app shape and the bus-and-send console -- is
   at most +2 % of P0 in both rounds. Every output digest is equal between the modules. A miss is
   reported with the table and the listings and goes to root; it is not tuned and re-run.
3. **Codegen.** `bash scripts/run-wasm-gates.sh` exits 0 (every held V8 row clean, the armed rows
   at or below their ceilings). The PR records the TurboFan instruction count, blocks and carried
   slots of the builtins dual loop (head against P0's 196 / 3 / 12) and the `simd128` and
   x86-64-v3 instruction counts of the changed bodies, base and head (no increase, or each named).
4. **iOS ceilings unchanged or lower.** `bash scripts/check-cross-targets.sh` passes; the
   `builtins` count is at or below its row.
5. **Workspace gates.**
   - `cargo test --locked --all-targets -p lane -p math -p builtins -p parametric-eq -p dsp-reference -p conformance --features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`
   - `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`
   - the `test-debug-a` workspace command from `.github/workflows/qualification.yml`
   - the worklet chain, as in `qualification.yml` (`build-web-audioworklet.sh --named-twin`,
     `check-web-audioworklet.sh --without-metadata-regeneration`,
     `check-browser-expected-resources.py`, `test-web-audioworklet.sh`)
   - `bash scripts/check-lane-policy.sh`, `bash scripts/check-builtins-policy.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

This slice adds no new test. Its claims are held by existing gates: the pinned digests and the
differential turn red if the restructure moves a bit; the V8 spill gate turns red if a held loop
spills. The timing and the instruction counts are PR evidence, not committed tests.

## Dependencies

#1451 (and #1328, which it follows).

## Amendment 1 (root ruling, 2026-10-06, before attempt 1)

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation
(`no-shortcuts-correctness-first`), in the context of decision 15, on #1451's verifier MINOR 1
(2026-10-06). It governs where the sections above seem to disagree.

- **A1. The splatted `FLUSH_EPS` constant rebuilt inside the loop is in scope.** Since #1451 D3
  splatted the flush constant again, TurboFan rematerialises that `v128.const` inside the loop
  (one `movq`/`movl` + `vmovq` + `vpunpcklqdq` per iteration, where base reused the carried word;
  one stack `vmovups` fewer). The #1451 verifier measured the held unarmed V8 rows' instruction
  counts moving with it, base `5b44be979` -> #1451 head `cc2b4e0c1`, carried slots unchanged at 0:

  | held row | base | #1451 head |
  |---|---|---|
  | dual depth-1 tail | 107 | 110 |
  | dual depth-2 pair | 184 | 187 |
  | mono depth-2 pair | 81 | 79 |
  | mono depth-1 tail | 49 | 52 |

  This slice owns the dual loop's instruction count, so it owns this rematerialisation too, in
  the builtins dual loop and in these held rows (this narrows the Non-goals' "The EQ's loops and
  V8 rows (#1451)" for this one mechanism). D1 names why TurboFan rebuilds the constant inside the
  loop; D2 removes it with a lane-kernel change in the Authorized paths, one shape for all
  targets, with no carried word brought back for the iOS ratchet (#1451 removed that cause). If
  the fix needs a path outside the Authorized paths, stop and ask root. Gate 3 also records these
  four rows' TurboFan instruction counts and carried slots, base and head; no held row may gain a
  slot. If no lane-kernel shape changes TurboFan's choice, the attempt record gives the evidence
  and the rows stay as measured.

## Amendment 2 (root rulings, 2026-10-06, after attempt 1)

Made by the decision-15 root coordinator on attempt 1 (`a249b3cc1`).

- **B1. Gate 2 accepted as host noise.** Round 2 meets the gate on every row; round 1's three
  misses fall in a round where base was also +5.6 % to +9.4 % over P0 on the 64-track rows and
  head was at or below base on every 64-track row in both rounds, at a host load of 4.2 to 5.4.
  The codegen (196 / 3 / 11 against P0's 196 / 3 / 12, `simd128` -529) is the primary evidence.
  The benchmark is not re-run.
- **B2. Authorized paths widened.** `svf_cascade_interleaved_form` and `svf_cascade_skewed_form`
  in `crates/lane/src/kernels.rs`, for this issue's goal (the loops' instruction count, A1's
  constant rebuild in the EQ rows), in one fold-in commit under the same verdict. The false
  comment that the interleaved loop carries no per-frame bounds branch is fixed. If stopping the
  dual tail's rebuild needs more than these two functions, the remaining cause is recorded and a
  successor issue is filed.

## Attempt record

### Attempt 1 (2026-10-06)

Base `f3956e63c` (stream G2 tip, #1461 on #1451/#1452). One code change,
`crates/lane/src/kernels/builtins.rs`. Scratch tools and logs under the session scratchpad
`w1454/` (not committed).

**D1, cause.** In the shipped `simd128` module of the base, LLVM kept `core`'s
`ZipImpl::new` for two `ChunksExactMut<f32>` out of line (one 58-instruction function, five calls
in `InputStage<f32x4>::process`; P0 inlines every one), so each of those dual frame loops reads its
chunk size and frame addresses back from linear memory, which costs a per-frame bounds branch to
`slice_index_fail` and the reload of every section coefficient on every frame, because the frame
pointers loaded from memory cannot be told apart from the coefficients. Evidence, base against P0:

- `wasm2wat` of the stationary unarmed dual loop: base 24 `v128.load` (22 coefficient loads at
  `offset=1664..2032` from the coefficient pointer), 3 `br_if`, 1 `call` (`slice_index_fail`),
  stride `local.get 30`/`31` read from the stored iterator (`i32.load offset=16`); P0 2
  `v128.load`, 1 `br_if`, no call, coefficients in locals before the loop. The same outlined
  call precedes four more dual loops in the function: the stationary body's second inlined copy
  (unarmed and armed), the identity chain, and one copy of the trim-ramp body (unarmed).
- TurboFan (`--no-liftoff`, the spill gate's `analyse`): base 222 instructions / 5 blocks /
  16 carried slots, P0 196 / 3 / 12. The base loop's `testl r14,r14; jz` and `cmpl rbx,0x4; jc`
  are the stored chunk-size checks, its 22 `vmovdqu xmm,[rdx+0x6..0x8..]` the coefficient loads.
- The three `v128.const` rebuilt in the base loop (`NONFINITE_LIMIT`, `1.0`, `FLUSH_EPS`, each
  `movq`+`vmovq`+`vpunpcklqdq`) sit after the bounds branch. LLVM emits the constants inside the
  loop in P0, base and head alike (3, 4, 3 `v128.const`); TurboFan hoists them only in a loop with
  no early exit (P0, head). That V8's scheduler hoists a node only from a block that dominates
  every loop exit is inferred from these listings and the experiment below, not read in V8's
  source.

**A1 (the EQ held rows).** The four held rows' `FLUSH_EPS` rebuild has the same shape: the dual
depth-1 tail (110) and the mono depth-1 tail (52) rebuild `NONFINITE_LIMIT` and `FLUSH_EPS` per
frame after an early loop exit, the per-frame bounds branch of `io[stream][base..base + width]`
in `svf_cascade_interleaved_form` (`crates/lane/src/kernels.rs`), which the code comment there
says is absent. That function is not an Authorized path, so this attempt does not change it.
Experiment only (reverted, not committed): the interleaved form rewritten to split its planes
(the `FramePairs` shape) gives dual tail 110 -> 107, mono tail 52 -> 43 (no constant rebuilt),
dual pair (reported) 187 / 10 slots -> 172 / 6, mono pair 79 -> 81, dual armed tail 128 / 2 ->
129 / 1, dual armed pair 219 / 12 -> 221 / 11, mono armed pair 92 -> 96, mono armed tail 58 -> 60,
masked rows 86 -> 86 and 102 -> 103, every held row clean; the dual tail still rebuilds both
constants (one early exit remains). **For root:** the EQ rows need a change to
`svf_cascade_interleaved_form` (and, for the pairs, `svf_cascade_skewed_form`), outside this
spec's Authorized paths. The rows stay as measured: head equals base on every row.

**D2, the change (`crates/lane/src/kernels/builtins.rs`).** A private iterator `FramePairs`
replaces `left.chunks_exact_mut(W).zip(right.chunks_exact_mut(W))` in every dual input-chain
frame loop of the module: `input_chain_block_body`, `identity_chain_block` (the two stationary
bodies), `input_chain_ramp_block_body`, `identity_chain_ramp_block` and
`input_chain_ramp_block_filter_body` (the ramp bodies, where D1 found the same outlined call). It
cuts both planes to their common length once and splits one frame off each per step with
`#[inline(always)]` code, so the frames are the zip's and no bit can move; its doc states why. No
`cfg`, no `#[inline(never)]`, the #1328 law (two forms, arming, counter, joint flush) untouched.
The mono bodies iterate one plane and have no outlined call; the matrix and gain kernels are not
input-chain loops and are unchanged.

**Gate 1, no bit moved.** The #1451 harness (`bitid2.rs`), adapted to #1461's
`PreparedEffect { processor, metadata }` and #1452's filter-ramp signature, plus a superset of 160
scenarios (entries 4 and 5: the all-identity plan, stationary and ramping, which reach
`identity_chain_block` and `identity_chain_ramp_block`): **1068 of 1068 identical** base vs head;
the base's first 908 lines equal #1451's `bitid2-head-final.txt`. Sensitivity: `FramePairs::new`
cutting one frame short moves 200 of 1068 (every dual line of entries 0, 1, 2, 4, 5; the mixed plan,
entry 3, does not use it); reverted, identical. Pinned artifacts: `g5_native_digests_match_pins` (in `run-wasm-gates.sh` and the release `wasm-gates` tests), `BUILTINS_DIGESTS`, `E9_DIGESTS` and multiband `DIGESTS` (in the gate-5 test runs), `conformance_fixtures --check`, `check-builtins-fixtures.sh` and `check-graph-determinism.sh` all pass. AArch64 legs: CI only, not run.

**Gate 2, browser cost: MISS in round 1, met in round 2.** `web-mixing-automation-benchmark.mjs run`, controls
`scratchpad/bench-controls.json` (#1451's), modules frozen: P0 `1968517e...` (#1451's), base
`aa991860...`, head `1ce4a5b4...`; one warmup and two measured rounds each, interleaved (p0, base,
head warmups; p0 1, base 1, head 1, head 2, base 2, p0 2), `taskset -c 31`, `node --no-liftoff`,
load 5.35 -> 4.21 on 32 threads (about twice #1451's); one invocation, not retried. Every output
digest equal across all six measured runs, on every arm and document.

| p50 | P0 r1 / r2 (ns) | base vs P0 | head vs P0 | head vs base |
|---|---|---|---|---|
| mono console, quiet | 151,168 / 151,038 | +2.4 % / -1.2 % | **+2.4 %** / -0.8 % | -0.1 % / +0.4 % |
| mono console, restated | 154,124 / 153,843 | -1.1 % / -1.6 % | -0.8 % / -1.3 % | +0.3 % / +0.3 % |
| mono console, automated | 159,404 / 157,489 | -1.9 % / -1.4 % | -1.7 % / -0.9 % | +0.2 % / +0.5 % |
| sixty-four-track console | 234,547 / 236,650 | +8.5 % / +0.6 % | **+7.4 %** / -0.7 % | -1.0 % / -1.2 % |
| app shape | 149,435 / 147,822 | +9.4 % / +2.6 % | +0.8 % / +1.2 % | -7.9 % / -1.3 % |
| bus-and-send console | 340,768 / 339,807 | +5.6 % / +1.1 % | **+5.3 %** / +0.3 % | -0.3 % / -0.7 % |

Round 2 meets the gate on every row (head within -1.3 % .. +1.2 % of P0). Round 1 misses it on
three rows; in that round base is also +5.6 % to +9.4 % over P0 on the 64-track documents, against
+2.4 % to +3.9 % in #1451's quieter run, and head is at or below base on every 64-track row in both
rounds. So round 1 reads as host noise on CPU 31 during the base and head round-1 runs, but by the
gate's own rule it is a miss: reported, not re-run; root decides. **Root ruling (2026-10-06, Amendment 2 B1): accepted as host noise; not re-run.**

**Gate 3, codegen.** TurboFan, `InputStage<f32x4>::process`, every dual loop, in V8's listing
order (the function inlines each body twice; instructions / blocks / carried slots), P0's
stationary loop 196 / 3 / 12:

| loop | base | head |
|---|---|---|
| stationary, first copy, unarmed (the builtins dual loop) | 222 / 5 / 16 | **196 / 3 / 11** |
| stationary, first copy, armed | 239 / 3 / 16 | 242 / 3 / 16 |
| stationary, second copy, unarmed | 222 / 5 / 16 | 194 / 3 / 11 |
| stationary, second copy, armed | 259 / 5 / 18 | 238 / 3 / 16 |
| trim ramp, first copy, unarmed | 245 / 3 / 18 | 246 / 3 / 18 |
| trim ramp, first copy, armed | 289 / 3 / 22 | 290 / 3 / 22 |
| trim ramp, second copy, unarmed | 262 / 5 / 20 | 244 / 3 / 18 |
| trim ramp, second copy, armed | 287 / 3 / 22 | 288 / 3 / 22 |

The head's stationary unarmed loop has P0's op histogram except the exit (52 / 50 stack `vmovups`,
4 / 4 `vmovdqu`, no `v128.const` rebuilt, no bounds branch; P0 counts down `addl rsi,0xff; jnz`,
head compares the remaining length `cmpl rdx,0x3; ja`). Named increases: the armed stationary loop
+3 and the three ramp loops that already inlined the zip +1 each; no slot gained anywhere. The
A1 rows: unchanged, head equals base on all ten (`run-wasm-gates.sh` exits 0; dual depth-1 tail 110, dual pair 187 / 10 slots, mono pair 79, mono tail 52, every held row clean, armed rows at their ceilings or below), the D1 and A1 notes above give why. `simd128` (`wasm-objdump -d`, every instruction): module -529;
`InputStage<f32x4>::process` 15,876 -> 15,405, the outlined `ZipImpl::new` (58) gone; no other
function changes. x86-64-v3 (`cargo rustc --release -p builtins --lib --emit asm`, instruction
lines): `InputStage<f32>::process` 8,168 -> 8,034, `<f32x4>` 7,805 -> 7,691, `<f32x8>` 7,357 ->
7,056; no other function changes.

**Gate 4.** `check-cross-targets.sh` passes; `builtins` 5 `memset_pattern16` calls, within its #1018 row.

**Gate 5.** Pass: the DSP-crate `cargo test` with the spec features (441 passed), the release `lane`/`math`/`wasm-gates` tests, `test-debug-a` (1,453 passed), the worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh`), lane, builtins, workspace and realtime policies, `cargo doc` with `-D warnings`, clippy, fmt. `check-lane-policy.sh` first refused `left.len().min(right.len())` (D8 float-method rule); the span now uses `core::cmp::min`, the lane policy passes, the lane tests pass again (84), and the gated module's code is instruction-identical to the timed head (`wasm-objdump -d` equal; one data byte differs, a source-location string).

**Test value.** No test added, rewritten or deleted.

### Attempt 1, fold-in under Amendment 2 B2 (2026-10-06)

Base `235b5fe4c` (attempt 1 plus #1462 and two spec commits; module `b387e216...`, its EQ rows
equal attempt 1's base). One change, `crates/lane/src/kernels.rs`.

**The change.** `svf_cascade_interleaved_form` walks its planes by splitting one frame off the
front of each per step (`next_frame_mut`, `next_frame`) under one exit test, `frames_left`, in
place of indexing `io[stream][base..base + width]` and `rest[stream][base..base + width]`. LLVM
cannot prove `frame * width + width <= frames * width`, so the indexed loop kept a bounds branch
per frame, an early exit before the uses of the loop's `v128.const`s, and TurboFan rebuilt
`FLUSH_EPS` and `NONFINITE_LIMIT` per frame in both depth-one tails. The false comment ("carry no
per-frame bounds branch") is gone; `frames_left`'s doc gives the reason for the shape.

**`svf_cascade_skewed_form` is not changed, by evidence.** Its steady-state loop rewritten the same
way (frame windows whose front is frame `i - (D - 1)`, constant offsets per section, one exit
test; scratch diff `w1454/e2-skewed.diff`) gave dual pair 179 / 3 blocks / 10 slots and dual armed
pair 212 / 11, but LLVM then unrolled the unarmed mono pair by two (a 142-instruction loop of 4
SVF steps and 2 stores: the held mono-pair, mono-tail and masked-pair rows found no loop of their
shape and failed closed) and V8 gave the armed mono pair a carried slot (`[rbp-0x1c0]`, ceiling
0) and the armed masked mono pair another (`[rbp-0x170]`). Reverted. The pairs never rebuilt a
constant, so A1 needs no change there; their per-step bounds branches stay, and the comment in
the function now says so. No successor is filed: the dual tail's rebuild is gone with the
interleaved form alone.

**Gate 1.** The attempt-1 harness (adapted again for #1462's `PrepareEffectRequest::tail_bound`,
from `conformance::tail_bound_for_request`): **1068 of 1068 identical** base vs head, and the base
output equals attempt 1's base output. Sensitivity: `frames_left` asking for two frames (the loop
stops one frame early) moves 141 lines (60 `eq_bank`, 81 `eq_scalar`); reverted, identical.

**Gate 3, the V8 rows** (`run-wasm-gates.sh`; instructions / carried slots; every held row clean,
no slot gained):

| row | base | head |
|---|---|---|
| dual depth-1 tail (held, A1) | 110 / 0 (4 blocks) | **101 / 0** (3 blocks), no constant rebuilt |
| dual depth-2 pair (reported, A1) | 187 / 10 | 181 / 10 |
| mono depth-2 pair (held, A1) | 79 / 0 | 80 / 0 |
| mono depth-1 tail (held, A1) | 52 / 0 (4 blocks) | **43 / 0** (3 blocks), no constant rebuilt |
| mono depth-2 pair, masked (held) | 86 / 0 | 86 / 0 |
| dual armed depth-1 tail (ceiling 2) | 128 / 2 | 118 / 1 |
| dual armed depth-2 pair (ceiling 12) | 219 / 12 | 223 / 12 |
| mono armed depth-2 pair (ceiling 0) | 92 / 0 | 95 / 0 |
| mono armed depth-1 tail (ceiling 0) | 58 / 0 | 51 / 0 |
| mono armed depth-2 pair, masked (ceiling 0) | 102 / 0 | 102 / 0 |

Named increases: mono pair +1, dual armed pair +4, mono armed pair +3. The dual armed tail now
carries 1 slot under its ceiling of 2; the gate prints "lower its ceiling", which is the spill
gate's file and not changed here.

**Whole-function counts.** `simd128` (`wasm-objdump -d`): module +135. EQ
`PreparedNativeEffect::process` (scalar) 13,648 -> 13,600, `process_bank` 15,012 -> 15,009,
`process_bank_mono` 8,751 -> 8,705. **Named increase:** four `core::array::try_from_fn`
instantiations (two over `[SvfCoef<f32x4>; _]`, 52 instructions each, two over
`[SvfState<f32x4>; _]`, 64 each) are now out of line, called once each from `parametric_eq`'s
`interleave` closures (`UnarmedRest`) per block, outside the frame loops: an inlining decision in
`process_bank`, not in the changed function. Writing `frames_left` without iterator closures
leaves them out of line too (module +141). x86-64-v3 (`cargo rustc --release -p parametric-eq --lib
--emit asm`): `process` (scalar) 6,579 -> 6,570, `process_bank<f32x8>` 7,496 -> 7,498 (+2),
`process_bank_mono<f32x8>` 4,580 -> 4,622 (**+42**, named, not analysed further); no other
function changes.

**Gates.** Pass, on the final tree: the DSP-crate `cargo test` with the spec features (441
passed), the release `lane`/`math`/`wasm-gates` tests, `test-debug-a` (1,458 passed),
`conformance_fixtures --check`, `check-builtins-fixtures.sh`, `check-graph-determinism.sh`, the
worklet chain (build `--named-twin`, check, expected resources, test), `run-wasm-gates.sh` (rows
as above), `check-cross-targets.sh` (`builtins` 5 calls, within its row; `parametric-eq` within
its row), lane, builtins, workspace and realtime policies, `cargo doc` with `-D warnings`, clippy,
fmt. AArch64 legs: CI only, not run.

**Test value.** No test added, rewritten or deleted.

### Batch follow-ups (stream G2 part 1, after the attempt-1 PASS)

From `/home/bl/misofm/submix-verdicts/1454-attempt1.md`.

- **MINOR 1 (root authorized `scripts/check-web-audioworklet-v8-spill.py`).** The dual armed
  depth-1 tail's ceiling falls from 2 to 1, its row now says why, and the docstring's ceiling
  sentence reads "dual tail 1, one general-purpose word" and records the fall. `run-wasm-gates.sh`
  passes at 1 (the row reports 1 carried slot) and no longer prints "lower its ceiling" for it.
  The dual armed depth-2 pair, not in this item, now reports 11 slots under its ceiling of 12 and
  prints "lower its ceiling" (open item for root).
- **MINOR 2, the x86 increase analysed.** `process_bank_mono<f32x8>` +42 (4,580 -> 4,622) is
  static code, not per-frame cost on a real path. With the `while frames_left` loop LLVM no longer
  reduces `svf_cascade_skewed_impl`'s `frames < D` fallback (the interleaved form at `D = 2`, at
  most one frame) to straight-line code: head keeps it as four innermost loops of 55, 62, 63 and
  67 instructions (about +247), offset by code that shrinks elsewhere in the function. The skewed
  steady-state pair loops change by -2..+2 per frame (unarmed 60 -> 58, masked 62 -> 64, armed
  72 -> 73, armed masked 75 -> 77); the depth-1 tails by -1..+3 (35 -> 36, 37 -> 40, 41 -> 40,
  44 -> 44), and the unarmed tail loses its three `64(%rsp)` stack loads. x86-64-v3 is a benchmark
  proxy, not a shipping target.
- **MINOR 2, the simd128 frequency corrected.** The four out-of-line `core::array::try_from_fn`
  instantiations are called from the dual `process_bank`'s `UnarmedRest` `interleave` inside
  `for pass in 0..pairs` (`crates/parametric-eq/src/lib.rs:2616-2625`): once each per depth-2
  pass, up to three passes per block (`EQ_SECTION_COUNT = 6`), so about twelve calls per block,
  not "once each per block". Still a per-block cost, small beside the dual pair loop's -6
  instructions per frame per pass and the dual tail's -9 per frame.
- **NIT 1.** The attempt-1 TurboFan table omits the four dual identity-chain loops of
  `InputStage<f32x4>::process` that FramePairs also changed (`identity_chain_block`,
  `identity_chain_ramp_block` and two inlined copies): 40 -> 41, 86 -> 87, 51 -> 39, 84 -> 85
  instructions, slots unchanged (0 -> 0, 6 -> 6).
- **NIT 2.** Two V8 changes outside the rows: the held mono depth-2 pair (live audio) goes 79 ->
  80 instructions and its loop-invariant stack reloads 10 -> 13 per frame, with no carried slot;
  the unheld dual masked armed depth-2 pair (silent tails only, not a row) goes 14 -> 15 carried
  slots.
- **NIT 3.** `frames_left`'s doc (`crates/lane/src/kernels.rs`) now names the loops'
  functions, `svf_cascade_interleaved_form` and `svf_cascade_skewed_form`, not the `_impl`
  dispatchers.
- **Tests.** None added, rewritten or deleted (a ceiling and doc text only).
- **Gates.** All pass on the follow-up tree (x86_64): fmt; workspace clippy `-D warnings` with and
  without `--all-features`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps
  --exclude gate-expander` (gate-expander's own doc failure is part 2's);
  `check-workspace-policy.sh`, `check-realtime-policy.sh`, `check-effect-runtime-policy.sh`;
  test-debug-a 1,462 passed, 0 failed; test-debug-b 890 passed, 0 failed; `conformance_fixtures
  --check`; release `lane`/`math`/`wasm-gates` (G5 `g5_native_digests_match_pins`) 123 passed;
  `run-wasm-gates.sh` (native 144 cases, wasm simd128 144 cases, 0 mismatches; V8 spill ok);
  `check-cross-targets.sh` PASS (known-defect rows unchanged: builtins 5, host-core 4, soft-clip 1);
  the worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`,
  `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`,
  `test-web-audioworklet.sh` with a private TMPDIR left empty, the V8 spill gate on the named twin):
  all pass. AArch64: CI only.
