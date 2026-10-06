# Give the multiband compressor an unarmed form for live audio

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`).
Filed 2026-10-06 by root order (Amendment 2 of *Let the builtins splat their chain constants
without iOS memset calls*, #1451). Code anchors verified on `codex/d15-stream-g` at `44585c80f`
(#1451 attempt 1's record). Ordered after #1451.

## Product outcome

On live audio the multiband compressor stops paying for exact rest. Today its crossover runs the
silence counter and the SVF joint flush on every frame, also on blocks where no lane can arm. The
builtin input chain and the parametric EQ already run those only on a block that can arm, and an
unarmed form (per-word flush, no counter in the loop) on every other block. After this slice the
multiband compressor has the same two forms with no rendered bit moved, kept only if codegen and a
descriptive p50 show it is better.

## Context

- **Why it has one body (#1328, record "Two forms per builtin chain body").** At #1328 A9 a second
  form doubled the crossover's splatted vector constants, and on `aarch64-apple-ios` every splat
  was a `memset_pattern16` libc call (#1018): `multiband-compressor`'s count would have risen from
  566 to 1128 against its ceiling. So the multiband kept one body with the counter and the joint
  flush on every frame. It was not in the browser documents measured then.
- **The ratchet cause is gone (#1451 D2).** `wide`'s `splat` reached LLVM as an array-repeat store
  loop that loop-idiom turned into Darwin's `memset_pattern16`; #1451 builds splats from an array
  literal in `lane::Lane::splat`/`zero`. `multiband-compressor`'s iOS count fell from 566 to 0 and
  its row in `scripts/lib/aarch64-known-defects.py` was deleted; a crate with calls and no row now
  fails `check-cross-targets.sh`. A second form no longer argues against the ratchet.
- **Code anchors.**
  - `crates/multiband-compressor/src/lib.rs`: `lr4_step` (`:509-514`), one frame of the LR4 split
    with `svf_step(.., rest, ..)` on both stages; `run_segment` (`:906`) computes
    `rest_near`/`rest_far` with `lane::silence_step` on every frame (`:945-946`); its caller `process_block` (`:1006`) sets
    `armed_after = L::splat(lane::silence_frames(sample_rate) as f32)` (`:1019-1020`) and runs
    `run_segment` per ramp segment (`:1022-1059`); the counter's state record (`:1347`) and its
    restore (`:1464`).
  - The pattern to follow: `lane::kernels::builtins::input_chain_block` (`:597-615`) tests
    `channel_arms` (`:516-518`, `silence_armable_holding` over the counter, the block length,
    `armed_after` and whether the sections hold state) once per block; if no lane can arm it
    advances each counter once with `lane::kernels::silence_skip_block` (`crates/lane/src/kernels.rs:327`)
    and runs the body with `svf_step_when(false, ..)` (`crates/lane/src/kernels.rs:1028`), the
    per-word flush. `flush_pair(n1, n2, +0.0)` is `(flush(n1), flush(n2))`, so the two forms are the
    same bits.
  - Pins: multiband `DIGESTS` (`crates/multiband-compressor/src/corpus.rs:252`,
    `src/corpus_digests.in`); `crates/multiband-compressor/src/corpus.rs:151-153` (the corpus's
    own counter use).
- **No browser document reaches the multiband.** The four browser documents and the native console
  rows run no multiband compressor, so p50 here is a scratch native harness and the worklet
  module's codegen (the multiband `f32x4` body is in the shipped module).

## Decisions frozen for this slice

- **D0. Root decision (2026-10-06).** The decision-15 root coordinator, under the owner's
  no-shortcuts delegation (`no-shortcuts-correctness-first`), ruled in #1451 Amendment 2 that the
  multiband's unarmed form is unblocked by #1451 D2 and gets its own issue, this one, gated
  bit-identical with codegen and p50 evidence.
- **D1. Same law, two forms.** Per segment, test once whether some lane's counter can arm (the
  builtins' `channel_arms` rule, over both crossover stages' state of each side). If none can,
  advance each side's counter once with `silence_skip_block` and run the crossover with the
  per-word flush (`svf_step_when(false, ..)`) and no counter in the frame loop; otherwise run
  today's body. Ramping and settled segments, every link mode and the bypass path keep their
  behaviour. No change to `N_SILENCE`, the counter semantics, the state record or its restore.
- **D2. Class A.** No rendered bit moves on any target.
- **D3. Keep-or-revert, decided before timing.** The unarmed form lands when gates 1-3 pass, the
  live-audio frame loop's instruction count on x86-64-v3 and `simd128` is lower than the base (or
  equal with a lower p50), and the descriptive p50 on live audio is not higher than the base and on
  a silent tail at most +2 % of it. Otherwise it is reverted, and the attempt record gives the
  counts, the p50 and the reason.

## Deliverables

1. The two-form crossover (D1), with a doc stating the forms and that they are the same bits.
2. The PR evidence: the base-versus-head differential (gate 1), the codegen counts and the p50
   table (gate 4), the iOS count (gate 2).

## Authorized paths

- `crates/multiband-compressor/src/lib.rs` (`run_segment`, its caller's per-segment dispatch, and
  `lr4_step` only if D1 needs a flag parameter)
- `crates/lane/src/kernels/builtins.rs` (`channel_arms` only, to make it `pub` for reuse, if
  needed) or a shared helper in `crates/lane/src/kernels.rs`
- `crates/multiband-compressor/tests/` only where a test names a changed item
- this spec

## Non-goals

- Any change to the #1328 law, the counter's payload, the multiband's tail or rest bounds (#1373),
  its live crossover (#1338) or bypass shunt (#1340).
- Native AArch64 timing (CI-only, no funded runner).
- A browser document with a multiband: not added here (`benchmark-real-paths-only`).
- Rejected alternatives: a run-time flag in the loop body in place of two instantiations (LLVM
  unswitched it at #1328: same code); keeping the single body without measurement (D0 orders the
  measurement).

## Hazards

- `crates/multiband-compressor/src/lib.rs` is a hot file: #1409 and #1338 edit `run_segment`'s ramp
  (STREAMS.md hot-file row), and #1340 and #1367 edit the crate; rebase onto whichever lands first
  and keep to the dispatch.
- A second instantiation of `run_segment` doubles its code size; the shipped worklet module's size
  is checked by `check-web-audioworklet.sh`.
- `check-lane-policy.sh` refuses some spellings inside `crates/lane`; use the crate's accepted forms.
- Another implementer may be working in the same worktree; commit exact paths only.

## Objective gates

1. **No rendered bit moves.** A base-versus-head differential through the multiband compressor at
   scalar, `Simd4` and `Simd8`, every link mode, bypassed and not, ramping and settled segments,
   live input, input that stops inside a block and at a block boundary, a silent tail through
   arming and exact rest, a state capture and restore mid-tail, at 44.1 and 96 kHz with quanta 1,
   128 and 4,096: every output and every state word identical. PR evidence, not a committed test.
   The multiband `DIGESTS`, `conformance_fixtures --check` and the multiband crate's tests are
   unchanged.
2. **iOS count stays zero.** `bash scripts/check-cross-targets.sh` passes with no
   `multiband-compressor` row (a crate with calls and no row fails).
3. **V8 and worklet.** `bash scripts/run-wasm-gates.sh` exits 0, and the worklet chain passes.
4. **Codegen and p50 (descriptive, D3).** The x86-64-v3 and `simd128` instruction counts of the
   multiband's live-audio frame loop, base and head, and the TurboFan count of the same loop in the
   shipped module. p50: a scratch native harness (not committed), the multiband bank at scalar,
   `Simd4` and `Simd8`, 128-frame blocks, all lanes live and one lane silent, one warmup and two
   measured rounds per build, pinned to one CPU, one invocation, not retried.
5. **Workspace gates.**
   - `cargo test --locked --all-targets -p lane -p multiband-compressor -p effect-runtime -p dsp-reference -p conformance --features math/lane,lane/test-support`
   - the `test-debug-a` workspace command from `.github/workflows/qualification.yml`
   - `cargo run --locked -p conformance --example conformance_fixtures -- --check`
   - `bash scripts/check-lane-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `python3 -B scripts/lib/aarch64-known-defects.py --self-test`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

This slice adds no new test if the existing ones turn red on a wrong form: the multiband `DIGESTS`
cover live and silent-tail frames. If the differential shows a scenario (for example a counter that
reaches `N_SILENCE` inside a block whose unarmed test said no lane can arm) that no existing test
reaches, add one test for it, which turns red if the per-block arming test is off by one frame.

## Dependencies

#1451 (and #1328, which it follows).

## Attempt record

None yet.
