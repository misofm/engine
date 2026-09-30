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

## Sol verdict, attempt 1

Sol, 2026-09-30. Verified `2320454c` (`959b7465`..`2320454c` on `b0f37aa7`) against this body,
decision 12, the umbrella, VERIFY.md M1 and amendment 6, and specs #1089-#1092 and #1098.

**PASS.** Every objective gate holds on independent evidence, no shipped plan moves a bank or a
bit, render is untouched, and five of Sol's seven planted mutations go red. The two survivors and
the other findings are low. None blocks P2b-P2e.

### Evidence

**Class A (gate 3), independently.**

- **Console digests.** `console-workload`'s ignored `digests` harness in release, at `b0f37aa7`
  (a detached worktree) and at the head: all 22 rows identical.
- **Sol's own differential.** A scratch probe (not committed), the same file compiled at base and
  head, reusing `bank_levels.rs`'s realistic console generator. 638 sessions: the intended and mono
  consoles truncated to 19 track counts from 1 to 64, and 600 random consoles with every launch
  effect, ragged strips, bypasses, sidechains, sends and mono pools. Each at `Simd4` and `Simd8`,
  1,276 rows. Per row it records every bound effect bank's members in lane order, the bound slots,
  the plan's group count, the builtin bank members, the whole `GraphResourceEstimate`, and a hash
  of 16 rendered blocks, unarmed and with the mono collapse armed. **All 1,276 rows are
  byte-identical to base.**
  - At `Simd8`, 339 rows bind effect banks, and all seven bankable effects bind: EQ, compressor,
    gate/expander, multiband, limiter, soft-clip and transient shaper. So none of the six rewritten
    shape checks or seven guards declines a full bank.
  - At `Simd4` on this host, the limiter and multiband banks bind (339 rows), also identical.
- **Fixtures.** No file under `fixtures/`, `hosts/` or `scripts/` changed.
  `checked_in_fixtures_are_the_generated_bytes`, `check-builtins-fixtures.sh`,
  `check-console-fixtures.sh` and `check-graph-determinism.sh` (100/100) pass.

**Out-of-path edits (question 1).**

- **The field.** Every out-of-path hunk is exactly the line `active_mask: <width>.full_mask(),`
  and nothing else (82 such lines across the diff). A new public field on a literal-constructed
  struct forces them. There is no smaller form.
- **The six `validate_shape()` swaps.** Each replaced check was
  `!has_matching_backend_width() || requests.len() != lanes`, which is `validate_shape`'s first
  check, with the same code and still checked first (the contract test pins the order). The two
  new codes can only fire on a mask that no pre-#1088 caller can build, because every caller now
  passes its own width's `full_mask()`. So no refusal changed its code, its order or the request
  it reports, and no accepted shape changed. The swap is forced: without it, a wrong-length
  all-`true` mask would reach `is_padded()` as "full" and bind.
- `graph/src/lib.rs` and `graph-compiler/src/lib.rs` carry docs and one `mod` line only.
- No overlap with P1b.

**The padding contract (question 3).** Sound for P2b-P2e.

- **Inactive lanes, gather and scatter.** The graph hands a chain one plane per member.
  `BankChain::gather`/`scatter`'s partial paths test `active[lane]`. A padded lane has `None`
  for its control lane, observation lane and shunt lane. Response snapshots index members
  (`member % lanes`, members-first). So neither the planner nor the rack can gather, scatter,
  observe or report a padded lane.
- **Reports.** `EffectBankStage` and `ConsoleEffectBankStage` drop `BankProcessReport`, so no
  graph or rack counter can count a padded lane. D7 attribution lives only inside each effect,
  which is exactly P2b-P2e's clause.
- **A factory that ignores the mask** would run clone lanes whose output the rack discards. What it
  could leak is whole-bank coupling (D7 reset, fast-path gates), which is why every shipped
  factory declines. The guards are verified below.
- **The `+0.0`-out clause** that the contract adds is necessary. It is right: `BankChain`
  zero-fills only at bind.

**Realtime (question 4).**

- No render-path source changed: `rack` and `graph/src/runtime.rs` are untouched, and the new
  `effect-contract` methods and factory code are bind-time only.
- `check-realtime-policy.sh` and its test, `check-realtime-audit-leak.sh` and
  `check-artifact-evidence-leak.sh` pass.
- **Every CI `audit-native` realtime step, in release:** `audit` capi, delay, compressor and
  parametric-eq (100,000 blocks), and gate-expander. The builtins, builtins-graph and graph
  traces. The protocol allocation audit. The realtime probes and the 1,000,000-block trace. The
  builtins and builtins-graph probe mutations. The 1,000,000-call effect-contract trace. All
  report 0 allocations, deallocations, locks and syscalls where they count.
- **The callgraph gate.** `check-web-audioworklet.sh --without-metadata-regeneration`, including
  the render-export closure, passes on a fresh delivery build.

**The AudioWorklet artifact (question 5).**

- Base `0f5c0ee7…` is 3,254,573 B, and head `87d3b59a…` is 3,261,324 B. Both digests match
  Terra's, and the head is `run-wasm-gates.sh`'s own build.
- `twiggy diff` puts all of the +6,751 B in bind-time code. No render function is in the diff.
  - EQ: `prepare_width` is outlined from `bind_homogeneous_bank` (+5,188 / −5,040).
  - `graph_compiler::banks::bind_group_banks`: +1,140.
  - The other factories' binds: multiband +953, transient shaper +872, compressor +446,
    gate +403, delay +164.
  - Constructors the inliner now outlines: `SoftClip::new` +1,129 against its bind −839,
    `LimiterCore::new` +724 against −402, and the multiband `Instance::new` at both widths +523.
  - Names +765, rodata +432.
- About 0.2 % of the module, and proportionate. Not re-pinned.

**Test value (question 6).** Seven mutations of Sol's own, each reverted:

| # | Mutation | Result |
|---|---|---|
| 1 | `is_padded` off by one (`active + 1 < len`) | red: `every_nonempty_mask_…` and the gate/expander decline test |
| 2 | `BankChain::gather` partial path also gathers padded lanes | red: `bank_padding` panics in `ArenaMembers::plane` |
| 3 | The bound `GraphPreparedEffectBank.active_mask` set all `true` | red: the padded compile is refused |
| 4 | Soft-clip's guard removed | red: `…_soft_clip_opts_in` ("1 of 8 lanes active") |
| 5 | `validate_shape` checks the mask before the requests | red: `the_request_refusals_keep_their_code_and_come_first` |
| 6 | `effect_bank_resource` charges member metadata per lane | **survives** (L2) |
| 7 | Compressor guard hoisted above its member loop, below lane 0's validation | **survives** (L1) |

Every new test names its defect. Beyond these, the EQ is the one factory whose guard sits before
some member checks (`prepare_width`'s `band_targets`). A scratch probe over every continuous EQ
parameter's bounds and midpoint, at all four launch rates, found no request that
`expected_prepared_metadata` accepts and `prepare_width` refuses. So the guard's order has no
observable effect there. #1070 owns the EQ's order.

**Gates (question 7).** All green at the head, in scratch worktrees:

- **Lint.** `cargo fmt --all --check`, and `cargo clippy --locked --workspace --all-targets
  --all-features -- -D warnings`.
- **Rustdoc.** `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` stops only at
  the known `tools/console-workload/src/lib.rs:374`, which is pre-existing and untouched here. With
  `--exclude console-workload`, the workspace documents clean.
- **Cross-target.** `scripts/check-cross-targets.sh`: PASS.
  - The AArch64 iOS and Android product crates are checked and linted with `-D warnings`.
  - The wasm `simd128` rows pass, and armv7 and scalar wasm are refused.
  - The #1018 memset counts stay within their ceilings.
  - The wasm `simd128` `cargo check -p host-web --all-features` passes.
- **Tests.**
  - Dev: effect-contract, graph-compiler, graph, rack, rack-compiler, the eight effect crates,
    conformance, effect-compiler, console-workload, audit and bench. 151 binaries, 974 passed,
    0 failed.
  - The same set in release: 152 binaries, 983 passed, 0 failed. The extra binary is Sol's
    ignored probe.
- **Wasm.** `scripts/run-wasm-gates.sh` passes: native, wasm `simd128` and the V8 EQ loops. On a
  fresh delivery build, `check-browser-expected-resources.py --artifacts`,
  `check-scalar-oracle-absent.py --wasm` and `test-web-audioworklet.sh` also pass.
- **Scripts.** `check-effect-contract.sh target/release/bench` (8 production factories). The
  effect-runtime, realtime, rack and graph policies with their mutation tests.
  `check-parametric-eq-render-contract`, `check-conformance-boundaries`, `check-workspace-policy`
  and `check-lane-policy`. `check-capi-abi.sh`, and `check-scalar-oracle-absent --native`.

**Merge (question 8).** `codex/1100-bypass-shunt-bounds` moved during verification, from
`112e69c8` to `e3589bec`.

- `git merge-tree` against both tips reports no conflicts. `graph-compiler/src/lib.rs`
  auto-merges.
- The merged tree passes `cargo test -p graph-compiler -p effect-contract -p graph`: 24 binaries,
  305 passed, including `bank_padding` beside P1b's `bypass_resources`.

### Findings

No high or medium finding.

- **L1. The decline tests pin member validation for lane 0 only.**
  - Each of the seven `a_padded_request_is_declined_until_…` tests malforms `malformed[0]`, for
    example `crates/compressor/tests/contract.rs:627`. Every factory validates `requests[0]` as
    `first` before its loop.
  - So a guard hoisted above the member loop stays green (mutation 7, compressor), while a padded
    request with a malformed member on lane 1 or later is declined instead of refused. That breaks
    the "after it has validated every member" clause (`crates/effect-contract/src/lib.rs:931-934`).
  - No production path reaches it. P2b-P2e replace these guards, so their tests should malform a
    non-first member.
- **L2. The per-member metadata charge is untested.**
  - `crates/graph-compiler/src/banks.rs:715` (`checked_mul(members)`) can be changed to `lanes`
    with every test green.
  - That is the safe direction, over-charging, and it touches padded banks only, which nothing
    binds yet. S2's resource gate should pin it.
- **L3. The contract is wider than the planner and the graph.**
  - `validate_shape` and the docs (`crates/effect-contract/src/lib.rs:936-938`) admit any
    non-empty mask.
  - `effect_bank_resource` (`banks.rs:684-693`) and the graph's gather and scatter admit only
    members-first masks.
  - An opted-in factory that assumes a prefix mask is correct for every plan, but wrong by the
    contract. P2b-P2e should either test a non-prefix mask or the contract should narrow to
    prefix masks. Either is acceptable. Pick one before P2b lands.
- **L4. The `+0.0`-out clause is not in P2b-P2e's gates.**
  - The contract (`crates/effect-contract/src/lib.rs:919-924`) makes an opted-in factory keep a
    padded lane at `+0.0` out for `+0.0` in.
  - #1089-#1092's gate 3 asks only that its state stay finite, and #1090 asks for rest state.
  - A padded lane that emits `-0.0` or a denormal would feed the next slot a non-`+0.0` input and
    defeat silent admission: cost, never an active lane's bits. Root should add "`+0.0` out for
    `+0.0` in" to those gates.
