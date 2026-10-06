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

## Attempt record

None yet.
