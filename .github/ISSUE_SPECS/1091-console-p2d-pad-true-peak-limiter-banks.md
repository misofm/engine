# Pad true-peak limiter banks with inactive lanes

Slice P2d of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's M1, L1, L4 and amendment 6 in
`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`). No open issue covers the
limiter.

## Problem

The limiter is the standing `post_insert` slot of the console fixtures, and it must bank at every
track count (decision 12, "Banking"). After P2a, its `bind_homogeneous_bank`
(`crates/true-peak-limiter/src/lib.rs:4133`) still declines a padded request.

- It takes its fast body only when every lane shares window shape and phase (`:1424-1427`,
  `:716-728`). A padded lane configured any other way would drag the whole bank onto the slow body.
- Its whole-bank D7 recovery resets every lane (`:3527-3533`).
- Its latency is fixed per rate at `rate/100 + 6` samples (`:234-241`). `lookahead` has no
  automation rate, so per-track knobs never split a bank (L1).

## Smallest closable slice

Opt the limiter into padded requests under P2a's contract:
- a padded lane carries a clone of an active member's request, is fed `+0.0`, and its output is
  discarded;
- D7 recovery and reports attribute active lanes only.

Because the clone copies the member's window shape and phase, a padded bank takes the same body a
full bank of the same members would take.

Authorized paths: `crates/true-peak-limiter/src/lib.rs` (bank binding, the D7 path and lane
bookkeeping only), its tests, and this spec. Coordinate with the open limiter kernel issues
(#988-#992); this slice changes no kernel.

## Dependencies

- *Let an effect bank bind a partial group with inactive lanes* (P2a, #1088).

This is batch C2. P2b-P2e edit disjoint effect crates, so after P2a merges they may land in
any order, one merge each.

## Objective gates

1. For every active count 1..W-1, a padded limiter bank's active lanes are bit-identical to the
   same tracks rendered per node. This holds on random input, on the limiter's fixtures and
   `link_mode: maximum`, and from the first block (latency line filling), at Simd8 on x86-64 and at
   Simd4 through `scripts/run-aarch64-tests.sh` or the wasm gates.
2. A padded bank of members that share window shape and phase takes the fast body. A test observes
   the body choice, and a padded lane built from zeros or defaults turns it red.
3. Coupling rule: active lanes' bits do not depend on the clone source. A padded lane fed `+0.0`
   produces exactly `+0.0` out (not `-0.0`, not a denormal) and keeps its state finite and at rest,
   block after block, including through the limiter's lookahead (P2a verdict, L4).
4. D7: a planted non-finite state in one active lane recovers and reports that lane alone.
5. `cargo test -p true-peak-limiter -p graph-compiler -p graph` pass, as do
   `scripts/check-effect-runtime-policy.sh`, `scripts/check-realtime-policy.sh` and the realtime
   audits. PR evidence: console digests are unchanged.

## Non-goals

- No kernel change.
- No change to the latency formula.
- No planner policy change.

## Standing rules for the implementer

- Work only from this body, the umbrella issue and decision 12. Read the cited code first.
- Class A: every gate that says "bit-identical" is a hard stop, not a tolerance. NaNs fold to one
  value (decision 10).
- Render stays allocation-, lock- and syscall-free. Only `crates/lane` names `wide` or intrinsics.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings` and the focused tests before every checkpoint.
- Every new test names the plausible defect that turns it red (AGENTS.md "Test value"). A
  one-time "no bit moved" comparison against the pre-change base is PR evidence, not a committed
  test.

## Attempt 1 evidence

Terra, 2026-09-30. Branch `codex/1091-pad-limiter-banks` from `2320454c` (P2a attempt 1 on batch
C2), with `codex/1088-partial-bank-mask` merged at `b2027254` (P2a's verdict commit, L3). Commits
`d92ad06f`, `4a502bda` (merge), `73c03562`, `517f8c71`, `cd1f02bb` and this record. "Base" below
is `b2027254`.

### What landed

All in `crates/true-peak-limiter`: `src/lib.rs` (bank binding, the D7 path, lane bookkeeping) and
its tests.

- **The binding opts in.** The #1088 decline is gone. A padded lane's request is validated by the
  member loop like a member's, and it seeds the lane with that member's defaults, program key and
  window shape. `validate_shape` admits members-first masks only (P2a L3), so the core's new
  `active` word is `every_lane(members)`. The kernel never reads it.
- **Per-lane D7 recovery** (`LimiterCore::reset_failed_lanes`). The §4.4 check itself is unchanged:
  one `check_block` per channel, then `nonfinite_lane_mask` on failure.
  - **Every active lane failed.** Today's whole reset: both channels to defaults, cursors to zero,
    whole block zeroed. This is always the case for a scalar instance, so a per-node instance
    behaves exactly as before. The seedless scalar pin did not move.
  - **Otherwise** only the failed lanes are zeroed (`zero_lanes`) and reset
    (`ChannelState::reset_lane_to_defaults` = `seed_lane_defaults` + `clear_lane_runtime`, the
    whole reset at one lane's stride). The shared cursors run on. That is bit-neutral: every word
    a lane clear writes is uniform along its ring, and the kernel reads rings only at offsets from
    a cursor, which is the rotation `commit_lane` already relies on.
  - **The report is masked by `active`.** A failed padded lane is still recovered, so it keeps
    answering `+0.0` with `+0.0`, but it is neither reported nor counted.
- **The #990 linked record.** After a whole reset it is `lane_shapes_agree`, as before. After a
  partial reset of a dual block it is re-derived from the words with `gain_state_agrees`, as
  `restore_track` does. This runs on the failing path only. A collapsed block leaves it cleared,
  its recovery included.
- **Bookkeeping.**
  - A padded lane gets no automation. `process_bank_inner` skips it, so its `BankProcessReport`
    entry stays empty.
  - A padded lane has no state payload. `checked_member` refuses a snapshot or restore of it with
    `effect.state.track`.
- **`clear_runtime` is `#[inline(never)]`.** Moving the whole reset into the shared helper let LLVM
  inline it into `ChannelState::new`. That put 3 more `memset_pattern16` calls in the iOS release
  assembly, 104 to 107, and `check-cross-targets.sh` refuses that under #1018's ratchet. None of
  them was on the frame loop. One out-of-line copy restores 104 with the base's per-function
  distribution: `process_block` 75, `process_bank_inner<mono>` 23, `new` 3, `clear_runtime` 3.

### Gates

| Gate | Evidence | Result |
|---|---|---|
| 1. Padded bank = per node | `tests/padding.rs::a_padded_bank_renders_its_members_per_node_bits`, through the factory only. Every width this build binds (`Eight` and `Four` on x86-64-v3; `Four` alone on AArch64, where `run-aarch64-tests.sh` runs it), every active count `1..W-1`, members on lanes `0..members`, padded lanes cloning member 0. Scenarios: 2 random (`dual_mono`, `maximum`; per-channel random ceiling, release and lookahead; per block loud noise, quiet noise, `+0.0`, `-0.0`, subnormals or a square; random point automation), plus the console fixture's limiter (tracks `64-m..`, `maximum`) dual and collapsed, with a silence in which the silent fast path engages in the padded banks (confirmed with a temporary probe). Every word is compared, NaN folded, from block 0, while the 486-sample line fills. A padded lane is fed what the bank left in it and must be exactly `+0.0` after every block. | green |
| 1. graph level (PR evidence, scratch) | A temporary `graph-compiler` test, since deleted, beside P2a's `bank_padding.rs`. Under `BankPadding::EveryGroup`, with the **real** limiter factory (EQ and compressor behind P2a's `PaddingDouble`), intended and mono consoles at N in {1, 3, 5, 7, 9, 10, 13}, with 4 bypass variants (none, every third limiter, all limiters, mixed on all three slots) and the mono fixture armed. Run at `Simd8` and at `Simd4` (`try_compile_console_model_at`). **112 of 112 renders equal the bank-free oracle**, every one with a padded limiter bank bound. | identical |
| 2. Fast body | `a_padded_bank_of_uniform_members_takes_the_uniform_body` (unit). Members share a 3 ms lookahead, off the 5 ms default. Both widths, both links, every active count: the padded bank with clones takes `DualUniform` and `MonoUniform`, as the full control does. Padded lanes of descriptor defaults take `DualPerLane` and `MonoPerLane`. Padded lanes of zeros are refused at bind. | green |
| 2. mutation | M6: the binding seeds padded lanes from the descriptor defaults. | red |
| 3. Coupling rule | `tests/padding.rs::the_members_bits_do_not_depend_on_the_padded_lanes`. For every active count, padded with clones of each member, a different clone per padded lane, the descriptor defaults, and an asymmetric non-clone (split 0/10 ms lookahead; it moves the fixture bank off the uniform and linked bodies). Each is compared with the clone-of-member-0 render, which equals per node. | green |
| 3. P2a L4 clause | `a_padded_lane_stays_at_rest_through_the_lookahead` (unit, W4 and W8, both links, dual and collapsed, 12 blocks while members limit hard). Every block of every `PaddedRun` (gate 4's runs too) checks each padded lane: output exactly `+0.0` on both planes, and every state word bit-equal to `clear_runtime`'s except the phase. `tests/padding.rs` checks the `+0.0` output publicly on every block. | green |
| 3. mutations | M10 (members seeded from the reversed request list); M19 (rest box sum off by one: the rest test red, the public `+0.0` check green, as expected). | red |
| 4. D7, one lane | `a_failed_lane_is_recovered_and_reported_alone` (unit, `Simd4` and `Simd8`, both links, dual and collapsed, every active count `1..=W`). The oracle is a scalar twin per member. A NaN planted in one member's recursive word (first and last member): report `{blocks: 1, lanes: 1 << target}`; only that twin trips; the other members keep their bits; the failed member renders its twin's bits for 6 blocks after, from the bank's running cursors; cursors not reset (reset for 1 member, where it is every active lane). A NaN planted in a padded lane: recovered, never reported or counted, members untouched. Every member failing: the whole reset, report masked to the members. `tests/padding.rs::a_member_fed_a_non_finite_sample_fails_alone` does the same through the factory with a NaN input sample, for 2..=W members. | green |
| 4. mutations | M1 old whole-bank recovery; M2 unmasked report; M3 padded lanes not recovered; M4 partial recovery resets the cursors; M11 `zero_lanes` a no-op; M13 collapsed recovery relinks the record; M14 partial recovery zeroes every lane; M15 collapsed body recovers the whole bank; M16 and M17 `-0.0` written into the line or the block; M12 record forced linked after a partial reset (red in the randomized #990 and #1014 oracles). | red |
| 5. Tests | `cargo test -p true-peak-limiter -p graph-compiler -p graph`: 28 binaries, 292 passed. | green |
| 5. Scripts and audits | `check-effect-runtime-policy.sh`, `check-realtime-policy.sh` (54 regions), `test-realtime-policy.sh`. Release audits: `audit` capi, delay, compressor, parametric-eq (100,000 blocks each), gate-expander. The builtins, builtins-graph and graph traces, the protocol allocation audit, the realtime probes and 1,000,000-block trace, the builtins probe mutation tests, and the effect-contract 1,000,000-call trace. Every one reports 0 allocations, 0 deallocations and 0 syscalls. No `audit` subject renders the limiter, so its realtime gate is `tests/allocation.rs`, now with a padded leg (below). | green |
| 5. Console digests | `console-workload`'s ignored `digests` harness in release, base against head: **22 of 22 rows identical**. The pinned digests in its release tests pass. | identical |

### Also run, all green unless noted

- **Build and lint.** `cargo fmt --all --check`. `cargo clippy --locked --workspace --all-targets
  --all-features -- -D warnings`.
- **Rustdoc.** `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` fails only on the
  known `tools/console-workload/src/lib.rs:374`. The workspace without `console-workload` is clean.
- **Debug.** CI's debug-a set: 92 binaries, 1,103 passed, 10 ignored. The debug-b DSP set: 150
  binaries, 807 passed, 28 ignored. `conformance_fixtures --check`.
- **Release.** `true-peak-limiter`, `graph-compiler`, `graph`, `audit`, `bench`, `console-workload`,
  `effect-contract` and `rack`: 531 passed, 5 ignored. Includes the limiter's 1,000-scenario
  randomized oracle.
- **Cross-target.** `scripts/check-cross-targets.sh` passes, with the limiter at 104
  `memset_pattern16` calls. `host-web` checks at wasm `simd128` with `--all-features`. The
  limiter's tests type-check for `aarch64-linux-android` (`--all-targets`). The no-silent-skip scan
  of `run-aarch64-tests.sh` finds no match.
- **Wasm.** `scripts/run-wasm-gates.sh`: native, wasm `simd128` and the V8 EQ loops.
- **Artifact gates**, on a fresh delivery closure: `check-web-audioworklet.sh
  --without-metadata-regeneration` (includes the render-export callgraph closure),
  `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`,
  `test-web-audioworklet.sh`, and the V8 spill gate with its self-test.
- **audit-native.** `check-effect-contract.sh` (8 production factories), `check-capi-abi.sh` and
  its self-test, `check-scalar-oracle-absent.py --native`, `check-graph-determinism.sh` (100/100),
  `check-builtins-fixtures.sh` (50 files), `check-console-fixtures.sh`.
- **Policies.** graph, rack, workspace, lane, bench, builtins, host-core, session and
  protocol-control. `check-parametric-eq-render-contract.sh`, the audit-leak and evidence-leak
  gates, and `check-test-support-ci.py`, `check-script-reachability.py` and
  `check-ci-path-routing.py`.
- **P2a verdict L1.** `a_padded_request_binds_after_every_lane_is_validated` malforms every lane
  in turn, members and clones, also beside a foreign clone, and requires `prepare`'s own code. M20
  (only the first member validated) and M21 (the program-key decline hoisted above validation)
  are red.

**The AudioWorklet artifact changes, and this slice does not re-pin it.**

| | Module digest | Size |
|---|---|---|
| Base `b2027254` | `b2eeb2d98e2013bffc1271828291bbf3f59d1ba233491bd66703e6f5fc53175d` | 3,262,117 B |
| Head | `50310e8979b01e95be2a8936f52b634b97ac6802b4058b7bed106d1256709c84` | 3,264,902 B |

- The growth is 2,785 B: code +2,382 B over 2 more functions, data +248 B and names +153 B.
- The head module is `run-wasm-gates.sh`'s own build, byte for byte.
- The render-export callgraph closure passes.
- The committed pin is unchanged (`6c952a2c…`, stale before this slice).

### Test changes outside the new tests

- **`tests/seedless.rs`: the W8 and W4 bank pins are re-recorded.** Before, track 1's NaN zeroed
  and reset the whole bank, and the pins carried that coupling, which #1091 removes. The witness
  is tightened from "whole blocks zeroed" to "track 1's lane zeroed on exactly the reset blocks,
  and no other lane". **The scalar pin did not move** (`ec135dac…`).
- **`tests/allocation.rs`.** Its hostile `3.0e38` block never failed the §4.4 check: the gain law
  delivers `g = 0` on that sample. A temporary probe showed the check never fired in any leg. It
  now feeds a NaN, which fires in every leg: scalar, full banks (all lanes, the whole reset) and a
  new padded leg (one lane, the per-lane path), dual and collapsed. 0 allocations.
- **`the_linked_body_engages_exactly_where_the_record_allows`.** The case "the §4.4 reset
  re-establishes the record" now poisons every lane, which is the whole reset. A new case shows
  that one lane's recovery re-derives the record.
- **`a_padded_request_is_declined_until_the_limiter_opts_in`** is replaced by
  `a_padded_request_binds_after_every_lane_is_validated`.

### Found on the way

- **The collapsed recovery and the record.** The shared helper first restated the #990 record for
  both bodies, so a collapsed whole reset set it again. The release randomized oracle (1,000
  scenarios) failed on it with "collapsed but linked"; debug's 24 scenarios did not. Fixed in
  `517f8c71`, and every collapsed block of a `PaddedRun` now asserts it, so M13 is red in debug.
- **The iOS `memset_pattern16` ratchet**, above (`cd1f02bb`).

### Residuals

- **A partial recovery costs the fast body.** The reset lane restarts at van Herk phase 0, so
  `lanes_uniform` fails and the bank renders the per-lane body until a whole reset. This is cost
  and is bit-neutral. It follows only a failed block, which this limiter cannot reach from legal
  input: its gain is at most 1 (P1 verdict M2).
- **The limiter's D7 report is still the instance's own `NonFiniteReport`.** Only tests read it.
  `ProcessReport::nonfinite_*` stays unwired for this effect, as before. Padded lanes' entries are
  empty by construction.
- **No committed planner-level test runs the real limiter padded.** `graph-compiler` is outside
  this slice's paths. The 112-row scratch probe is the evidence; S2 (#1098) or a successor should
  commit one.
- **NEON.** There is no AArch64 toolchain or qemu here. `Simd4` ran on x86-64, and the tests
  type-check for Android arm64. CI's `aarch64-debug` runs them natively.

### Path deviations

None. `crates/effect-contract` changes arrive only through the P2a merge.
