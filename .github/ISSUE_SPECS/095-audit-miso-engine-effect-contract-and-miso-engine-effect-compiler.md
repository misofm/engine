# 095 Audit: miso-engine-effect-contract and miso-engine-effect-compiler

One-line summary: Clean the effect contract and compiler: per-value classification, dead automation runtime, bypass out of the program key, and the `SmoothingRule::Linear` redefinition of D11.

**Authority: GitHub issue #95 and its plan comment.** This file is a stateless pointer, not a
second copy of the brief. The issue body carries the findings with `path:line` evidence; the plan
comment on the issue carries the numbered steps, evals, acceptance checklist and hazards; and the
master plan (the first comment on issue #83) decides everything cross-cutting -- the numeric
contract (D1-D12), the `Lane` trait and its per-operation semantics, the block-kernel contract, the
`miso-engine-math` and `miso-engine-effect-runtime` boundaries, the fixture re-pin policy of §8, the
workstream waves of §9 and the evals of §10. Where this file and those comments disagree, they win
and this file is corrected in the same checkpoint.

Read, in order: `AGENTS.md`; issue #125 (standing instructions for the audit workstream); issue #83
body, master-plan comment and execution-plan comment; then `gh issue view 95` and its plan
comment.

Do not re-decide anything the master plan decides, do not loosen a gate, and do not pin a fixture
from production output: fixtures are regenerated only from an independent `f64` oracle or from the
scalar `Lane` instantiation, with the old-to-new deviation and the audit finding cited in the
commit message.

## Delivery record (2026-08-23, branch `audit-095-contract`)

Delivered, with the decision record amended in
`.github/ISSUE_SPECS/011-native-effect-contract-parameter-metadata-state-and-cid-package-specification.md`
and red mutations in `crates/miso-engine-effect-contract/tests/MUTATIONS.md`:

* **F1** per-value classification withdrawn from the contract text; `sanitize_sample` deleted after
  a grep confirmed its only remaining caller was the conformance mock. D7 replaces it.
* **F2 / D11** `ParameterSmoother` redefined to the precomputed-increment form and proven
  bit-identical to `effect-runtime::ramp::LinearRamp`.
* **F5 / D6** the contract's last four platform-libm calls moved to `miso-engine-math`; its
  `check-math-policy.sh` allowlist row deleted.
* **F6** orphan root header `include/miso_engine_effect_contract_v1.h` deleted and gated.
* **F8/F9/F10 (partial)** `canonical_bits`, `normalize_zero`, `is_negative_zero` and
  `parameter_value_valid` made public; the compiler's private copies deleted;
  `StatePayloadSizes::check`, `ValidatedPrepare`, `initial_value_slots` and
  `default_initial_values` added as the single statements of their rules.
* **Wave-2 ledger** the `bind_homogeneous_bank` `Err`/`Ok(None)` divergence unified and frozen on
  the trait; `scratch_fixed_bytes` defined as an admission ceiling (no descriptor byte moved); the
  payload-header rule ("the header word outranks the caller's claim") frozen at contract level.
* **E4** duplicated-helper manifest wired into `check-effect-runtime-policy.sh` as a two-way
  ratchet, with five mutations in `test-effect-runtime-policy.sh`.
* **E6** the conformance harness generalised off its own mock and run against
  `miso-engine-compressor` and `miso-engine-parametric-eq`; four harness bugs fixed, neither effect
  changed.

**Not delivered, handed over.** `bypass` stays in `EffectProgramKeyV1` (F4); eval E5 is **not run**,
not passed. The target design is written out on `EffectProgramKeyV1` in the contract source. The
blocker is that every effect's bank builds an all-or-nothing bypass mask from one
`metadata.bypass`, so the key change without per-lane kernels would apply lane 0's bypass to all
eight lanes. Owner: **#96** (rack/graph bank seam) plus the nine effect crates.

**Also handed over.** Collapsing `normalize_zero`, `is_negative_zero`, the parameter-domain
predicate, the mapping pair and `StatePayloadError` to one definition each needs
`miso-engine-effect-runtime` to depend on `miso-engine-effect-contract`, which needs the contract
made `no_std` (its only `std` uses are `BTreeMap`, `BTreeSet` and `Arc` -- all `alloc`) and then a
mechanical sweep of the nine effect crates' `state_error` bridges. Counts are pinned by the E4
manifest so they cannot drift before that change lands.
