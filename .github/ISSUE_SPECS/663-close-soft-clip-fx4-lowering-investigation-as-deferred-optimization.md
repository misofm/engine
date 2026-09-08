# Close soft-clip FX4 lowering investigation as deferred optimization

GitHub: https://github.com/misofm/engine/issues/663

Parent: #559 (FX4). Coordination: #560. Base:
`6f4a1b1c893ce04fbe8001e9d175702eba9a9d78`.

## Problem

The bounded soft-clip FX4 lowering investigation has exhausted its authorized
source attempts without delivering a product change. Its history establishes a
remaining compiler-lowering opportunity on some target shapes, but establishes
no measured performance problem and does not justify a fourth implementation
attempt.

## Smallest closable slice

Record a documentation-only disposition that closes the soft-clip part of FX4 as
**deferred optimization**, not as an eliminated residual:

- #647, #649, and #651 are failed qualifications with no inherited gate credit;
- #653 verified repeated identical cubic constant materializations on native AVX2
  W8 (`-1` and emitted folded `-3`) and Wasm scalar (`-3`, `-1`, and `+1`), while
  native scalar and Wasm SIMD128 W4 reuse one materialization;
- #656's shared-bundle rewrite was behaviorally valid but lowering-neutral on the
  target shapes that motivated it;
- #660 never completed an authorized qualification and ended at a procedural
  three-attempt hard stop;
- delivered product source remains unchanged; and
- no timing, improvement, projected saving, regression, or budget-miss claim is
  established.

Further soft-clip FX4 implementation requires a genuinely new weekly/performance
issue with a measured budget or an owner-approved justification. The paired helper
must not be restarted as a renamed fourth attempt. The true-peak-limiter FX4
obligation remains separate and open.

## Exact ownership

This issue owns only:

- this numbered issue spec;
- concise disposition rows in the #559 and #560 tracker specs and GitHub bodies.

It owns no Rust or test source, compiler payload, retained evidence, benchmark,
timing run, artifact, AudioWorklet pin, manifest, lockfile, workflow, or cleanup.
Lane B alone owns AudioWorklet qualification and pins.

Preserve the failed #643/#647/#649/#651/#656/#660 worktrees, branches and history;
all six #649 payloads under `/tmp/issue649-softclip-*`; the #656 candidate payloads
and lowering records under `/tmp/issue656-softclip-*`; all #660 attempt evidence;
and the separately retained #649/#656 tracked payload records. Do not remove,
rewrite, reconstruct, recapture, or rerun them.

## Gates and workflow

1. Sol briefs this issue from exact current main and confirms the shared active-slot
   count and disjoint ownership with lane B.
2. Astra LOW reviews the exact clean pushed brief and GitHub/spec parity before any
   disposition edit.
3. A Luna HIGH or XHIGH documentation executor writes only the authorized concise
   #559/#560 rows and this spec's evidence/decision record. It must verify that the
   product-source diff against current main is empty and make no new technical or
   performance inference.
4. Astra LOW adversarially reviews the exact pushed evidence checkpoint. Any
   correction consumes the next of at most three documentation attempts.
5. After PASS, use exact-head/current-main PR review, required qualification,
   guarded merge-parent verification, post-main qualification, GitHub issue/tracker
   synchronization, and clean delivered-worktree removal.

No implementation, compiler capture, benchmark, artifact qualification, pin
change, retained-state cleanup, or fourth paired-helper attempt is authorized.

## Scope verdict and decision record — 2026-09-08

- **Documentation attempt 1 — SCOPE PASS.** At feature head
  `64d8dfb180e6251122f5d21e7ca01a76df816cc7`, delivered `main`
  `6f4a1b1c893ce04fbe8001e9d175702eba9a9d78`, and tracker head
  `bd183f2a9470327b063265942d2b87e5b4234d58`, this remains a documentation-only
  disposition with no product-source change.
- **Decision:** close the soft-clip FX4 lowering investigation as **deferred
  optimization**, not an eliminated residual. #647/#649/#651 are failed
  qualifications with no inherited gate credit. #653 verified repeated native
  AVX2 W8 `-1` and emitted `-3`, and Wasm scalar `-3`, `-1`, and `+1`; native
  scalar and Wasm SIMD128 W4 reused one materialization. #656 was behaviorally
  valid but lowering-neutral. #660 did not complete an authorized qualification
  and ended at the procedural hard stop. Delivered product source is unchanged.
- No timing, improvement, projected-saving, regression, or budget-miss claim is
  established. Implementation requires a genuinely new weekly/performance issue
  with a measured budget or owner-approved justification; the paired helper must
  not restart as a renamed fourth attempt. True-peak-limiter FX4 remains separate.
  #663 and #664 occupy the two active slots, with #664 and Lane B retaining
  LocalRing/AudioWorklet ownership. All failed worktrees, branches, and temporary
  evidence remain preserved.
