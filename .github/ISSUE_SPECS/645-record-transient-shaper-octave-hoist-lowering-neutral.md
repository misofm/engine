# Record transient-shaper octave hoist as lowering-neutral

GitHub: https://github.com/misofm/engine/issues/645

Audit parent: #559 FX4. Coordination: #560. Evidence parents: #635 and #643.
Base: `e6b2f1541c344d04eb68d4b8e3fafb72b591fe74`.

## Problem and decision boundary

#635 established that `DB_PER_OCTAVE` and `OCTAVES_PER_DB` are materialized in
mapped transient-shaper frame loops on native scalar/AVX2-W8 and Wasm
scalar/simd128-W4. #643 tested one run-scope source rewrite. Its source and ordinary
gates passed, but the final comparable lowering capture showed the mapped assembly
ranges unchanged. #643 therefore reached its three-attempt hard stop and closed
without a PR or merge.

The smallest closable successor records that bounded result and leaves delivered
source unchanged. It does not claim compiler optimality, impossibility of another
implementation, performance neutrality, a speedup, or projected savings. A
prepared-state rewrite is not authorized because the evidence does not justify its
persistent memory, load, register-pressure or spill risk.

## Exact ownership

This documentation-only issue owns:

- `.github/ISSUE_SPECS/645-record-transient-shaper-octave-hoist-lowering-neutral.md`;
- #559/#560 coordination records.

It owns no Rust/test source, manifests, dependencies, locks, build output, compiler
capture, benchmark, timing, floor-accounting document, workflow, generated
resource, AudioWorklet artifact or pin. Run no Cargo, rustc, formatter, browser,
SDK or artifact command. Lane B retains artifact authority and active #644 remains
disjoint.

## Evidence and disposition

The delivered main source remains the pre-#643 implementation: the two `L::splat`
expressions occur in `frame` at the two named multiply sites. #643's source
checkpoint `20a816aef545c7df36722b11945f234c493d80d9` is preserved only on its failed
branch and must not enter this delivery.

#643's sole final lowering capture ran at clean pushed authorization head
`f4a4525ba8a3b78c8548a3a207a052c6280757df` with Rust/Cargo 1.97.1 and LLVM 22.1.6.
The native, Wasm scalar and Wasm simd128 commands each returned 0. Its compact
hard-stop record is pushed at `e8522cbde1ad4d36048541f03c66f7778a51b1d3`.
The relevant results were:

| Shape | Ramping/stationary result after rewrite |
| --- | --- |
| native AVX2 W8 | both `vbroadcastss` occurrences remain in the mapped loops |
| native scalar | both folded `vmulss` operands remain in the mapped loops |
| Wasm scalar | both `f32.const` occurrences remain in the mapped loops |
| Wasm simd128 W4 | both `v128.const` occurrences remain in the mapped loops |

Those selected ranges were byte-identical to #635. The accepted disposition is:
**the tested run-scope rewrite produced no qualifying lowering change**. The
residual reason is retained compiler materialization after that tested rewrite.
The class-A floors remain unchanged.

This closes only the transient-shaper experiment for these two constants. FX4's
soft-clip and true-peak-limiter sites remain separate original obligations, each
requiring its own one-effect applicability review.

## Objective gates and delivery

1. Astra LOW verifies exact clean pushed scope, GitHub/local synchronization,
   disjoint #644 ownership, and that current main retains the original source.
2. Root Sol HIGH records only this disposition; no implementation agent or command
   execution is needed.
3. Astra LOW adversarially verifies the exact evidence identities, all #635/#643
   failures without relabeling, absence of #643 source from the branch, limited
   claim language, diff hygiene and current-main ancestry.
4. After exact-head/current-main PASS, open one documentation-only PR. Required CI,
   guarded exact-parent merge, successful post-main qualification, issue/tracker
   synchronization and clean delivered-worktree removal precede closure.
