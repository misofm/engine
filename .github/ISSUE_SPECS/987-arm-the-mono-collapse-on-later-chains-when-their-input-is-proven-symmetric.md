# Arm the mono collapse on later chains when their input is proven symmetric

## Product outcome

#970 fixes a wrong-audio bug by arming the mono collapse only on the chain that gathers a track's input; every later chain of a split strip renders dual. The #970 verification measured what that costs on realistic mono sessions: 64 mono tracks where half have no effects run +42% at 8 lanes and +48% at 4; one track in eight lacking the limiter runs +25% and +24%. Recover it exactly: arm a later chain when its input planes are proven identical (the upstream chain or per-node op collapsed and nothing between wrote L and R differently), rather than trusting the track's source witness.

## Smallest closable slice

Carry an exact per-chain "input is symmetric" fact from the producing chain's own collapse decision (and each per-node op's symmetry) to the consuming chain at bind or per block, and arm the later chain only on that fact. Research and evidence: `docs/handoffs/dual-mono-2026-09-27/DUAL-MONO.md` and `VERIFY-970-971.md`.

## Objective gates

- The #970 reproducers (asymmetric EQ plus a `PostSimd1` meter; asymmetric per-node delay; sends at `post_input` or `insert_send` (decision 12's names for the retired `post_input_builtins` and `post_simd1`); a live left-only trim write on misaligned cohorts) stay bit-correct.
- The two cost shapes above return to within noise of today's armed cost, with identical bits to the dual render.
- Every console digest unchanged; red mutations for each proof step.

## Dependencies

#970.

## Decision record

- 2026-10-24, root (accepting the scope note's split): class A, no rendered bit moves; gate on counters and bits, not on
  the µs numbers above (no standing console row has track inserts; #1039 retired descriptive benchmarks).
- **987a (rack).** `BankChain::set_input_symmetric(bool)`, a per-block input term, default `true`; the dispatch arms
  only when it holds. Unlike `SOURCE` it can be an episode, so a dual block with it down clears
  `collapse_channels_agree` (M3), and only a `channels_agree` proof brings the chain back.
  `BankChain::planes_identical()` publishes "this block's resident planes are bit-identical by construction": the
  block collapsed with every slot in the prefix, or it was skipped silent. Both are structural facts; no sample value
  is compared (owner rule: dual-mono only from the source's layout, no content detection).
- **987b (graph runtime, own branch, later).** At bind, the per-lane producer table for each bank unit; per block,
  AND the producers' `planes_identical()` into `set_input_symmetric`; arm `UnitBanking::Bank` only when every lane's
  track is eligible and every lane's producer is a same-track bank unit (per-node ops between: successor).
  `PlanUnitEligibility.armed_on_input` as evidence. Paths: `crates/graph/src/runtime.rs`, `crates/graph/src/lib.rs`,
  `crates/engine/src/realtime/plan.rs`, the graph-compiler console test, `crates/graph/tests/MUTATIONS.md`.
  `set_input_symmetric` is sticky, so 987b sets it on **every** block, including blocks that are not armed, and never
  reads a producer's `planes_identical` after that producer's `run` failed (for example in `begin_block`).

## Evidence

- 987a: `crates/rack/tests/mono_reengage.rs` `an_asymmetric_input_window_declines_and_waits_for_a_proof`; mutants in
  `crates/rack/tests/MUTATIONS.md` (issue #987).
- 987b: `crates/graph/src/runtime.rs` `input_producer_table` (bind) and `Runtime::execute` (per block);
  test `crates/graph-compiler/src/tests/console_banking.rs`
  `a_later_mono_chain_arms_on_its_producers_and_renders_the_dual_bits` (`[48, 4]` armed, `[24, 2]` on
  the #970 rule alone); mutants 987-1..3 in `crates/graph/tests/MUTATIONS.md`. One-time digest of
  every `native_session_rows()` workload, armed and forced off, 256 blocks after warm-up, base
  `0f8641cd3` against the branch: all 30 digests and collapse counters identical (no standing row
  has a later mono chain, as the scope note said).
- 987b deviation: `PlanUnitEligibility.armed_on_input` was not added. Adding a field breaks the
  struct literal in `tools/console-workload/tests/chain_shape.rs`, outside the authorized paths;
  `bank_collapse_counters` carries the evidence. A successor may add it with that path authorized.
