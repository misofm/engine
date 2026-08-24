# 098 Audit: miso-engine-graph (executor correctness bug, reductions, PDC)

One-line summary: Executor bank-order correctness (F1, wave 0), then vectorised reductions in stable node-ID order and slice-copy PDC.

**Authority: GitHub issue #98 and its plan comment.** This file is a stateless pointer, not a
second copy of the brief. The issue body carries the findings with `path:line` evidence; the plan
comment on the issue carries the numbered steps, evals, acceptance checklist and hazards; and the
master plan (the first comment on issue #83) decides everything cross-cutting -- the numeric
contract (D1-D12), the `Lane` trait and its per-operation semantics, the block-kernel contract, the
`miso-engine-math` and `miso-engine-effect-runtime` boundaries, the fixture re-pin policy of §8, the
workstream waves of §9 and the evals of §10. Where this file and those comments disagree, they win
and this file is corrected in the same checkpoint.

Read, in order: `AGENTS.md`; issue #125 (standing instructions for the audit workstream); issue #83
body, master-plan comment and execution-plan comment; then `gh issue view 98` and its plan
comment.

Do not re-decide anything the master plan decides, do not loosen a gate, and do not pin a fixture
from production output: fixtures are regenerated only from an independent `f64` oracle or from the
scalar `Lane` instantiation, with the old-to-new deviation and the audit finding cited in the
commit message.

## Decision record (wave 3, #98 F2-F7)

* **Level-major schedule** (F1, wave 0, closed under #123 at `885b919`). `sequential_schedule ==
  concat(dependency_levels)`, verified at bind by `has_valid_structural_layout`, so a homogeneous
  bank that renders at its first member is correct for every graph. Rendering at the *last* member
  is not a fix and was not adopted.
* **Both executors are built from the lowered `ExecutionProgram`** (#99 F2's seam). The sequential
  executor drives one liveness-coloured arena and reads its producers in place; the native
  dependency-wave executor gives every parcel its own arena and stages every edge between
  partitions. Node semantics -- kind dispatch, reduction, route, effect block, bank chain, PDC,
  observers -- have exactly one implementation, in `crates/miso-engine-graph/src/runtime.rs`.
* **D9 reductions**: stable edge-ID order, left to right, block-wide, through
  `lane::kernels::{sum2_block, sum_into_block}`. Fan-in 0 zero-fills, fan-in 1 copies (or is read
  in place), fan-in `n >= 2` is `in0 + in1` then `+= in_k`. This is *not* the previous balanced
  pairwise tree, and it changes rendered bits at fan-in four and above.
* **Route fold and op order** (D3): the linear gain is folded into the 2x2 coefficients once, at
  bind (`ll' = gain * ll`, ...); render spends one multiply and one fused multiply-add per output
  word, frozen as `l = fma(lr', r, ll' * l)` and `r = fma(rr', r, rl' * l)` in
  `lane::kernels::mix2x2_block`. The compiler's `RouteTransform` and its canonical line are #99's
  and are untouched.
* **PDC** is `lane::kernels::pdc_delay_block`: a two-segment slice exchange with no per-sample loop
  and no `%`. A block longer than the line walks it in line-length segments.
* **Sanitisation is removed from the graph** (D7). `RouteTransform::transform`, the per-pair
  `is_finite`/`is_subnormal` branches and the dead `sanitized_samples` counter are deleted, not
  moved: input sanitisation is the input stage's (#85) and output finiteness is the bank boundary
  check in `miso-engine-effect-runtime` (83c).
* **A binding on an elided node is refused** (`graph.scheduler.layout`) rather than silently
  dropped. The compiler never asks for one -- the three internal rack boundaries are not bindable.

### Fixtures re-pinned, with their oracle

| fixture / pin | old | new | oracle |
|---|---|---|---|
| `fixtures/graph/v1/summation-residuals.json` | `fixed-balanced-pairwise-f32`, max residual `3.0` | `left-to-right-f32`, max residual `0` | its existing `reference_f64` linear sum |
| frozen Issue-037 100-layout transcript | `0x0fc9_bdc8_ff12_0f6e` | `0x9dfc_dcf2_0e37_0ef5` | per-layout: recorded per-track post-matrix contributions folded left to right in the plan's own stable edge order, all 100 layouts, asserted before the literal |
| Issue-037 12-track 100,000-block hash | `0x2fd8_5286_518f_d13b` | `0x5b3e_672a_ae5d_97aa` | the same contributions with the folded 2x2 re-applied through `softfma::fma_f32_via_f64`, then reduced left to right |

`fixtures/graph/v1/direct-route.*` are contract fixtures and are unchanged by this work: the delta
in the tree is pre-existing drift on `origin/main` (the `estimate` line gained four bank-resource
columns in an earlier merge and the fixture was never regenerated), and both files regenerate
byte-identically from an unmodified `origin/main` checkout.

### Deferred, with owners

| finding | deferred to |
|---|---|
| F6 chain-level AoSoA residency and the in-register transpose | #96 (`BankChain` already owns the single gather/scatter call site) |
| F8 pull-model staging, F9 worker-pool lifetime | #100 (scheduler) |
| F11 graph runtime arena layout, F12 byte accounting | successor issue "graph runtime arena" |
| `ProcessReport` from dynamic-rack effects (still `let _ =`) | #95 |
