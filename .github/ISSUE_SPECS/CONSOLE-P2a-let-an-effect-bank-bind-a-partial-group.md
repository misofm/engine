# Let an effect bank bind a partial group with inactive lanes

Slice P2a of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's M1 and amendment 6 in
`.github/ISSUE_SPECS/DRAFT-console-strip-VERIFY.md`, commit `03aceb94`).

## Problem

A console slot always banks, for every track count, with partial groups padded with inactive lanes
(decision 12, "Banking"). The effect contract cannot express a partial group:

- `validate_shape` refuses `requests.len() != lanes` (`crates/effect-contract/src/lib.rs:917-923`).
  The EQ and the limiter call it. The compressor, gate/expander, soft-clip, transient shaper,
  multiband and delay repeat the check by hand in their `bind_homogeneous_bank`.
- `effect_bank_resource` refuses a mask that is not all-true (`crates/graph-compiler/src/banks.rs:561-565`),
  which fails the compile.
- `GraphPreparedEffectBank.active_mask` already exists "so a padded group can be bound without a
  second bank shape" (`crates/graph/src/lib.rs:884-886`). The rack already runs partial chains:
  `BankChain` zero-fills scratch, and inactive lanes are never gathered or scattered.
- Builtins already pad partial banks. Effects do not (#96 F7).

## Smallest closable slice

The contract and planner half of padding. No shipped plan changes.

1. `PrepareEffectBankRequest` gains an explicit active mask with one entry per lane. Requests
   stay one per lane, so the existing length checks keep holding. `members <= lanes` means the
   mask's active count. `validate_shape` refuses an empty mask and a mask whose length is not
   `lanes`.
2. The padding contract (decision 12):
   - a padded lane carries a clone of an active member's prepared request, never zeros;
   - it is fed `+0.0`, and its output is discarded;
   - D7 recovery and D7 reports attribute active lanes only.
   Write the contract into the `PrepareEffectBankRequest` docs. The per-effect implementation is
   P2b-P2e.
3. `effect_bank_resource` accepts a partial mask. The planner can form a padded group and fill
   absent lanes with the clone.
4. Padding policy: the planner pads a group only when the group asks for it. Console slots ask from
   S2 on. Until then nothing asks, and inserts keep today's rule (full groups bank, remainders
   render per node; decision 12, "Inserts bank opportunistically, as today"). A test-only policy
   that pads every group exists for P2b-P2e's tests.
5. Every shipped factory declines a request whose mask is not all active, until its P2b-P2e slice
   opts in. That is one guard per `bind_homogeneous_bank`. A factory that ignored the mask would
   silently bind clone lanes as real ones.

Authorized paths:
- `crates/effect-contract/src/lib.rs`;
- `crates/effect-runtime/src/bank.rs`, only if a shared request-mask helper is needed;
- `crates/graph-compiler/src/banks.rs`;
- `crates/graph/src/lib.rs`, only for the existing `active_mask`;
- the `bind_homogeneous_bank` of each of the eight effect crates, only for the step-5 guard;
- their tests, and this spec.

## Dependencies

None beyond decision 12. Merge after *Add the console-strip benchmark rows* (B0).

## Objective gates

1. Contract tests:
   - `validate_shape` accepts every active count 1..W under a mask;
   - it refuses an empty mask and a wrong-length mask, each with its typed diagnostic.
2. A test-double factory that accepts padding binds a partial bank under the test-only policy. The
   graph carries the mask, and the rack never gathers or scatters an inactive lane: a planted
   scatter of an inactive lane turns the test red.
3. Every shipped factory declines a partial request (one test per factory; removing a guard turns
   it red), and no shipped plan changes. PR evidence, not a
   committed test: console digests and the graph and builtins manifests are unchanged.
4. `cargo test -p effect-contract -p graph-compiler -p graph -p rack` pass, as do
   `scripts/check-effect-contract.sh`, `scripts/check-graph-determinism.sh`,
   `scripts/check-realtime-policy.sh` and the realtime audits.

## Non-goals

- No effect opts in here.
- No change to which insert cohorts bank.
- #887 (tiled gather and scatter for partial banks) is a performance follow-up, not a prerequisite.

## Standing rules for the implementer

- Work only from this body, the umbrella issue and decision 12. Read the cited code first.
- Render stays allocation-, lock- and syscall-free. Only `crates/lane` names `wide` or intrinsics.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings` and the focused tests before every checkpoint.
- Every new test names the plausible defect that turns it red (AGENTS.md "Test value").
