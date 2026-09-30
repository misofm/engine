# Let an effect bank bind a partial group with inactive lanes

Slice P2a of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's M1 and amendment 6 in
`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`).

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

- *Keep a bypassed lane in its effect bank* (P1, #1087). This is merge order, not a functional
  dependency: both slices edit `effect-contract/src/lib.rs` and `graph-compiler/src/banks.rs`.

This is batch C2, after batch C1 has been pushed.

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

## Attempt 1 evidence

Terra, 2026-09-30. Branch `codex/1088-partial-bank-mask` from `b0f37aa7` (`codex/batch-console-2`,
P1 merged), commits `959b7465`, `310aaef7`, `acce448e` and this record.

### What landed

- **The mask** (`effect-contract/src/lib.rs`). `PrepareEffectBankRequest` gains
  `active_mask: &[bool]`, one entry per lane. Requests stay one per lane. `active_lanes()` is
  `members`, `is_padded()` says whether any lane is padded, and `BankWidth::full_mask()` is a full
  bank's mask. `validate_shape` keeps `effect.bank.requests` for the backend/width and request
  count, checked first, and adds `effect.bank.mask_length` (not one entry per lane) and
  `effect.bank.mask_empty` (no member). The contract accepts any non-empty mask; the members-first
  layout is the planner's, not the contract's.
- **The padding contract** is written on `PrepareEffectBankRequest`: a padded lane carries a clone
  of an active member's request, never zeros; it is fed `+0.0` and its output discarded; D7
  recovery and reports attribute active lanes only; an active lane's bits depend neither on the
  padded lanes nor on the clone source; padding is opt-in, and a factory that has not opted in
  declines after validating every member. One clause is made explicit because the premise needs it:
  `rack::BankChain` zero-fills a padded lane only at bind, so a factory that accepts padding must
  hold a padded lane at `+0.0` out for `+0.0` in. Otherwise the next slot of a chain, and the next
  block, would not be fed `+0.0`. This is what P2b-P2e's "dummy lanes stay `+0.0`" checks.
  `NativeEffectFactory::bind_homogeneous_bank` lists the padded decline among the `Ok(None)` cases.
- **The guard** (step 5). Every shipped factory declines `is_padded()` with `Ok(None)` after its
  member loop, so a malformed member in a padded request is still refused. The delay needs no
  separate guard, because its decline is unconditional. Six factories (compressor, gate/expander,
  soft-clip, transient shaper, multiband and delay) carried a hand-written copy of the old shape
  check that never read the mask. Each now calls `validate_shape()`, with the same code for every
  shape the old check refused. Without that change, a wrong-length mask would reach a factory
  unrefused.
- **The planner** (`graph-compiler/src/banks.rs`). `BankPadding::AsRequested` pads a partial group
  only when `group_asks_for_padding` says so. It says `false` for every group, so nothing asks and
  every shipped plan is unchanged. S2 changes that one body for console slots, and inserts keep
  today's rule. A padded slot binds the members on lanes `0..members`, a clone of the **first
  member's** request on every padded lane, and the group's mask, which also goes onto the bound
  `GraphPreparedEffectBank`. A slot binds when every **member** runs it; a padded lane has no
  slots and is not an identity. `bindable_slot_members` refuses any layout other than
  members-first with `graph.internal.invariant`. That layout is the one the graph runtime's gather
  and scatter index by.
- **The estimate.** `effect_bank_resource` accepts a bank of `1..=lanes` members whose mask is
  `true` on exactly lanes `0..members`. Scratch is charged per lane and member metadata per
  member. For a full bank both figures are unchanged.
- **The test-only policy** is `BankPadding::EveryGroup`, `#[cfg(test)]`, scoped per thread by
  `banks::test_only_with_bank_padding(policy, || ...)`.
  - It is `cfg(test)`, not a `test-support` feature, because a feature would need a
    `graph-compiler` manifest entry and a `qualification.yml` change (`check-test-support-ci.py`
    requires a whole-package CI step to enable it). Both are outside this slice.
  - **For P2b-P2e:** a planner-level padded test is a `graph-compiler` unit test, beside
    `src/tests/bank_padding.rs`, whose `PaddingDouble`, `padding_registry` and `intended(n)` it can
    reuse. An effect-level test binds through `bind_homogeneous_bank` with a mask directly and needs
    no policy.
  - Integration tests under `tests/` cannot reach the policy.
- **Docs.** `graph::GraphPreparedEffectBank::{members, active_mask}` and
  `graph_compiler::GraphRackBoundSlot::members` now describe padded banks.

### Gates

| Gate | Evidence | Result |
|---|---|---|
| 1. `validate_shape` under a mask | `effect-contract/tests/bank_mask.rs`, 5 tests. Every non-empty mask at both widths (15 at W4, 255 at W8) is accepted, with `active_lanes` equal to the popcount, `is_padded` iff `members < W`, and every count `1..=W` reached. The all-`false` mask is `effect.bank.mask_empty`. Every length in `0..=2W` other than `W`, all `true` and all `false`, is `effect.bank.mask_length`. A wrong request count or a backend/width mismatch stays `effect.bank.requests` and is checked before the mask. `full_mask` is a well-formed, unpadded bank. | green |
| 1. mutations | The length check removed: red (`a_wrong_length_mask_...`). The empty check removed: red (`an_empty_mask_...`). Only full masks accepted (the old rule): red (`every_nonempty_mask_...`). Each reverted. | red |
| 2. A test double binds a partial bank under the test-only policy | `graph-compiler/src/tests/bank_padding.rs::a_partial_group_binds_one_padded_bank_and_renders_the_per_node_bits`, on 1, W - 1 and W + 3 tracks of the intended console (`eq -> comp`, then the limiter). `PaddingDouble` wraps each launch effect: it validates, asserts that every padded lane's request is field-for-field (initial values bit-for-bit) a clone of a member's, counts the mask it was asked for, and binds the launch bank with the full mask. **Production policy:** only full masks are asked for, the partial group's 3 slots render per node, and the output is the bank-free oracle's bits. **`EveryGroup`:** nothing renders per node, one bank per slot, the partial group's 3 slots are asked for with `true` on lanes `0..partial`, and the full groups with the full mask. The output is the oracle's bits. **The graph carries the mask:** straight from `bind_rack_banks_indexed`, every `GraphPreparedEffectBank` has `active_mask` `true` on exactly its members' lanes, a member count of `W` or `partial`, and an estimate `effect_bank_resource` accepts. | green at Simd8 on x86-64. Simd4 runs in CI's `aarch64-debug`: the test is width-agnostic. |
| 2. planted scatter | `BankChain::scatter`'s partial path also scattered the last (padded) lane. | red: the render panics in `ArenaMembers::plane_mut` (`graph/src/runtime.rs:1644`), which hands a chain one plane per member. Reverted. |
| 2. other mutations | Padded lanes cloned with `initial_values: &[]`: red (the double's clone assertion). `group_asks_for_padding` returned `true`: red (production arm). The factory handed a full mask: red (masks asked for). The estimate refusing partial masks again: red (the compile is refused). The old every-lane-runs-the-slot rule: red (no padded bank). Each reverted. | red |
| 3. Every shipped factory declines a partial request | One test per factory: `a_padded_request_is_declined_until_the_<effect>_opts_in` in `parametric-eq/tests/bank.rs`, `compressor/tests/contract.rs`, `gate-expander/tests/contract.rs`, `soft-clip/tests/contract.rs`, `transient-shaper/tests/bank.rs`, `multiband-compressor/tests/product.rs` and `true-peak-limiter/src/lib.rs` (unit), plus `delay/src/lib.rs::a_padded_request_is_declined_and_still_validated`. The same members bind as a full bank (the control). Every prefix mask `1..W-1` is declined. A padded request with a malformed member is refused with `prepare`'s own code. The delay test also pins `mask_length` and `mask_empty`. | green |
| 3. mutations | All seven guards removed: 7 red. All seven hoisted above member validation: 7 red. The delay's pre-#1088 hand-written shape check restored: red. Each reverted. | red |
| 3. No shipped plan changes (PR evidence, not committed) | **Console digests:** `console-workload`'s ignored `digests` harness (64 blocks of every native session row: all 21, including B0's 9/10/13/16-track, app-shape and sparse rows) in release at `b0f37aa7` in a detached worktree and at the head: **identical**. **Scratch probe**, deleted after use: intended and mono consoles at N in {1, 3, 7, 9, 10, 13, 64}, no bypass and every third track bypassed, at Simd8 and Simd4. It digests the plan shape (every bound bank's members), the output and every track's `PostSimd1` and `PostSimd2PreFader` words over 64 blocks. **All 56 rows identical** to base. **Graph manifest:** `graph_fixture --check` and `checked_in_fixtures_are_the_generated_bytes` pass. **Builtins manifest:** `check-builtins-fixtures.sh` ok (50 files), and `audit builtins-graph` agrees with it. No file under `fixtures/` changed. | identical |
| 4. Crate tests and scripts | `cargo test -p effect-contract -p graph-compiler -p graph -p rack`: 27 binaries, 355 passed, 0 failed. `check-effect-contract.sh target/release/bench`: ok, 8 production factories. `check-graph-determinism.sh`: PASS 100/100. `check-realtime-policy.sh` and `test-realtime-policy.sh`: ok. **Realtime audits:** `audit` capi, delay, compressor, parametric-eq (100,000 blocks each) and gate-expander. The builtins, builtins-graph and graph traces. The protocol allocation audit. The realtime probes and the 1,000,000-block trace. The builtins and builtins-graph probe mutation tests. The effect-contract 1,000,000-call trace. Every one reports 0 allocations, 0 deallocations and 0 syscalls where it counts. | green |

Also run, all green:

- **Build and lint.** `cargo check --workspace --all-targets --all-features`. `cargo clippy --locked
  --workspace --all-targets --all-features -- -D warnings`. `cargo fmt --all --check`.
- **Rustdoc.** `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` is clean except the
  known base error at `tools/console-workload/src/lib.rs:374`, which is the only error in that
  crate.
- **Debug tests.**
  - CI's debug-a set: 92 binaries, 1,103 passed, 0 failed, 10 ignored.
  - The debug-b DSP set: 149 binaries, 798 passed, 0 failed, 28 ignored.
- **Release tests.** `audit`, `bench`, `console-workload`, `effect-contract`, `graph-compiler`,
  `graph` and `rack`: 469 passed, 0 failed. This includes `console-workload`'s pinned digests.
- **Cross-target.** `scripts/check-cross-targets.sh` passes. It checks and lints the AArch64 iOS
  and Android product crates (`capi`'s closure), the wasm `simd128` rows and the refusal rows. The
  wasm `simd128` `cargo check -p host-web --all-features` also passes.
- **Wasm and artifact.** `scripts/run-wasm-gates.sh` passes: native, wasm `simd128` and the V8 EQ
  loops. The artifact gates, run on a fresh build:
  - `check-web-audioworklet.sh --without-metadata-regeneration`, including the render-export
    callgraph closure;
  - `check-browser-expected-resources.py --artifacts` (native parity digests agree);
  - `check-scalar-oracle-absent.py --wasm`;
  - `test-web-audioworklet.sh`;
  - the V8 spill gate and its self-test.
- **Policies.** `check-effect-runtime-policy` and its test, `check-graph-policy`,
  `check-rack-policy`, `check-workspace-policy`, `check-test-support-ci`,
  `check-script-reachability`, `check-ci-path-routing`, `check-parametric-eq-render-contract`,
  and the lane, builtins, host-core, session, protocol-control and bench policies. The audit-leak
  and evidence-leak gates. `check-capi-abi.sh`. `check-scalar-oracle-absent --native`.
  `check-console-fixtures.sh`.

**The AudioWorklet artifact changes, and this slice does not re-pin it.**

| | Module digest | Size |
|---|---|---|
| Base `b0f37aa7` | `0f5c0ee7…` | 3,254,573 B |
| Head | `87d3b59a4f4ef319b3c3574e33d76d263c677f0b566073645ef07059c4b53c9f` | 3,261,324 B |

- The head module is `run-wasm-gates.sh`'s own build, byte for byte.
- The growth is 6,751 B: code +5,546 B over 8 more functions, data +432 B and names +765 B. It is
  all control-plane binding code.
- The render export's callgraph closure passes.
- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` still holds `6c952a2c…`,
  which was already stale at base.

### Red mutation run on the way

`check-graph-policy.sh` scans every `graph-compiler` source file outside an inline
`#[cfg(test)] mod`. So the first version of `src/tests/bank_padding.rs`, whose mask log was a
`std::sync::Mutex`, failed it. The log is now a lock-free histogram of the masks asked for
(`310aaef7`).

### Path deviations, for Sol

- **The new field** breaks every struct literal of `PrepareEffectBankRequest`. There are 79 of
  them, and each gains the one line `active_mask: <width>.full_mask(),`. Of those, the following
  are outside the effect crates' tests:
  - `conformance/src/randomized.rs` (4) and `conformance/tests/effect_contract.rs`;
  - `effect-compiler/tests/symmetry_designed_words.rs`;
  - `graph-compiler/tests/bypass_shunt_identity.rs` (3);
  - `compressor/examples/lane_sample_timing.rs`;
  - `tools/audit/src/{compressor,gate_expander}.rs`;
  - `tools/bench/src/console.rs`;
  - `tools/console-workload/tests/paired_spans.rs` (4).
- `graph-compiler/src/lib.rs`: the `GraphRackBoundSlot::members` doc, and `mod bank_padding;` in
  the test module.
- `graph/src/lib.rs`: a `members` doc line beside `active_mask`.
- **The six hand-written shape checks** replaced by `validate_shape()` go beyond "only the step-5
  guard". The guard needs a validated mask, as described above.
- **No overlap with P1b (#1100).** None of its files is touched: `graph/src/runtime.rs`,
  `graph-compiler/src/estimate.rs`, `effect-compiler/src/prepare.rs` and
  `effect-contract/src/live.rs`.

### Residuals

- **#971 under padding** is unchanged. Under the test-only policy the stranded-mono demotion still
  runs, and its move is kept only if it binds more banks. That objective measures nothing once
  groups pad, which is S2's amendment-10 decision.
- `graph::GraphResourceEstimate::effect_bank_count`'s doc still says "full" banks. It becomes stale
  when S2 lets a group ask, and the file is outside this slice.
- The `+0.0`-out clause holds for the builtin stages a chain may fuse with: their padding lanes are
  identity (`builtins::InputStage`, unit fader and matrix). Each effect's own obligation is
  P2b-P2e's.
