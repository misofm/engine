# Pad parametric EQ banks with inactive lanes

Slice P2b of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's M1 and amendment 6 in
`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`).

## Problem

A console EQ slot must bank at every track count (decision 12, "Banking"). After *Let an effect
bank bind a partial group with inactive lanes* (P2a), the EQ's `bind_homogeneous_bank`
(`crates/parametric-eq/src/lib.rs:3200`) still declines any request whose active mask is not full.

- The EQ already pads internally with identity words (`:3123-3170`). It refuses non-native widths
  (`:3210`).
- Its whole-bank D7 recovery zeroes or resets every lane (`:2926-2930`), so a fault on one lane
  would be reported against padded lanes too.

## Smallest closable slice

Opt the EQ into padded requests under P2a's contract:
- a padded lane carries a clone of an active member's request, is fed `+0.0`, and its output is
  discarded;
- D7 recovery and reports attribute active lanes only.

Keep #1070's order: validate every active member first, then decide bind or decline.

Authorized paths: `crates/parametric-eq/src/lib.rs` (bank binding, the D7 path and the bank's
lane bookkeeping only), its tests, and this spec.

## Relation to #888 and #887

- This slice **supersedes #888's absent-member half**: a cohort of fewer than W members binding as
  one bank.
- **#888 is amended** to keep only its identity-slot half: an insert cohort whose member lacks the
  EQ slot. That is an insert-only optimisation, and no console slice depends on it. See the
  amendment appended to #888's spec.
- #887 (tiled gather and scatter for partial banks) is a performance follow-up, not a
  prerequisite. The rack already runs partial chains.

## Dependencies

- *Let an effect bank bind a partial group with inactive lanes* (P2a, #1088).

This is batch C2. P2b-P2e edit disjoint effect crates, so after P2a merges they may land in
any order, one merge each.

## Objective gates

1. For every active count 1..W-1, a padded EQ bank's active lanes are bit-identical to the same
   tracks rendered per node. This holds on random input, on random parameters and ramps, and on
   the EQ's fixtures, at Simd8 on x86-64 and at Simd4 through `scripts/run-aarch64-tests.sh` or the
   wasm gates.
2. Coupling rule: the active lanes' bits do not depend on which active member the padded lanes
   clone. A test varies the clone source.
3. A padded lane is never scattered, and its state stays finite on `+0.0` input. A padded lane
   fed `+0.0` produces exactly `+0.0` out (not `-0.0`, not a denormal) and keeps its state finite
   and at rest, block after block (P2a verdict, L4).
4. D7: a planted non-finite state in one active lane recovers that lane and reports it alone.
   A bypassed lane counts as active: a bypassed lane fed a tripping value (for example `1e30`
   behind enough legal gain) leaves every enabled bank-mate's bits unchanged (P1 verdict, M2).
   Padded lanes are neither reported nor charged.
5. `cargo test -p parametric-eq -p graph-compiler -p graph` pass, as do
   `scripts/check-parametric-eq-render-contract.sh`, `scripts/check-effect-runtime-policy.sh`,
   `scripts/check-realtime-policy.sh` and the realtime audits. PR evidence: console digests are
   unchanged.

## Non-goals

- No kernel arithmetic change on active lanes.
- No planner policy change: console slots start padding in S2, and inserts keep today's rule.

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

Terra, 2026-09-30. Branch `codex/1089-pad-eq-banks` from `2320454c`, fast-forwarded to P2a's
`b2027254` (verdict L3: members-first masks) before any commit of this slice. Commits `8ce84d41`,
`49aa542e`, `fb624924`, `858ccb84` and this record. Every path is `crates/parametric-eq` (its
`src/lib.rs` and `tests/bank.rs`) or this spec.

### What landed

- **The bind** (`bind_homogeneous_bank`, now a thin call of `bind_bank(request, native_lanes)`).
  The #1088 guard is gone. `validate_shape` runs first, then the width check, then every lane's
  request, padded clones included, then the bind. So #1070's order holds: a malformed member is
  refused with `prepare`'s own code. The mask goes into the bank as `active: [bool; W]`.
  `bind_bank` exists so the unit tests can bind a four-lane bank on x86: the product passes
  `Backend::current().width()`, the tests pass 4 or 8.
- **D7 per lane** (`Channel::recover_failed_lanes`). A rejected block used to be zeroed whole
  and every lane's integrators cleared. Now only the lanes that failed the §4.4 bound are zeroed
  and cleared, by one bitwise `select` per frame and per integrator word. Every other lane keeps
  its words and state bit for bit. At `W = 1` this is the old recovery, word for word, so the
  scalar instance is unchanged. The dual and collapsed bodies share it.
- **Padded lanes** are prepared from their clone requests like any lane and run the same kernel.
  The bank:
  - keeps them out of every report (`invalid_spans` and both fault counters stay 0);
  - refuses a prepared target on one (`EffectTargetError::Capacity`) and a state restore
    (`effect.bank.track`), the two writes that could move it off `+0.0` at rest. A snapshot
    only reads and stays open.
  Nothing else reads `active` on the render path.
- **Why a padded lane writes `+0.0`.** A designed word set has `c1` in `[0, 1)`, `a2 > 0`,
  `a3 >= 0`, `m0 >= 0` and no `-0.0` word. From `x = +0.0` and `+0.0` integrators, every product
  in the frame body is a signed zero and every sum has a `+0.0` operand. So the frame writes
  `+0.0` and `flush` keeps both integrators `+0.0`. Elided and dry sections pass the `+0.0`
  through. The argument is on `recover_failed_lanes`, and gate 3's test measures it.
- **Two fixes the gates forced:**
  - **V8 spill gate.** The recovery inlined into the dual `process_bank` made TurboFan carry two
    values of the stationary depth-one tail through stack slots, the #977 mechanism.
    `run-wasm-gates.sh`'s V8 spill gate went red. `#[inline(never)]` (as on `snap_ended` and
    `start_ramp`) fixed it (`fb624924`).
  - **iOS `_memset_pattern16` ceiling.** Outlined, the lane mask was an `or` of one-hot
    compares against a splatted `1.0`. That took parametric-eq's iOS count to 160, above its
    #1018 ceiling of 151. One flag vector compared with `+0.0` needs no splat store: the count is
    now 146 (`858ccb84`).
- **The effect-runtime ratchet** pins `fn recover(` at zero, the per-value helper #95 collapsed.
  The per-lane block recovery is D7's own rule, not that helper, so it is named
  `recover_failed_lanes` (`49aa542e`).

### Gates

| Gate | Evidence | Result |
|---|---|---|
| 1. Padded bank = per-node, random | `padded_banks::a_padded_bank_renders_its_per_node_instances` (unit test in `src/lib.rs`). Banks are bound through `bind_bank` at **Simd4 and Simd8, both on x86**, for every member count `1..=W` (`W` is the control). Each scenario: random per-channel configurations; prepared targets (ramps, some cut short by another); resets; restores of `+-f32::MAX` integrators; ragged blocks; dual and collapsed bodies. Input is drawn per member: `-0.0`, subnormals, non-finite words, the ceiling itself, and a `9.5e29` sine that faults behind any boost. It is compared every block against one `prepare`d scalar instance per member: words (class A), report fields and state payload (common and left sections when collapsed). Dev reach (6 seeds): W4 24 scenarios, 768 blocks, 852 retargets, 19 resets, 44 restores, 11 collapsed, **154 blocks where one member faulted and another did not**. W8: 48 scenarios, 1,536 blocks, 1,672 retargets, 443 such blocks. Release (24 seeds): W4 3,072 blocks and 576 such blocks; W8 6,144 blocks and 1,812. | green |
| 1. Padded bank = per-node, the EQ's fixtures | The five `tests/bank.rs` scenarios (odd-live, admitted-select, skewed-pass, switched-off-cut, block-limit) now bind through `Layout`. Every member count `1..W-1` pads each bank with clones of its first member; the last bank takes the remainder. Each padded leg, dual and collapsed, must equal the full leg's digest, which folds every member's words, report and state after every block in track order. The **dual full leg now equals the per-node leg** and is asserted so: the scalar digest for four scenarios, and `cliff_scalar_digest(CLIFF_TRACKS)` for switched-off-cut, whose pinned scalar leg renders one overflow track. The scenarios carry faults on tracks 0 and 4, `-0.0`, NaN, `1e30` edges, restored huge integrators, prepared targets and ragged blocks. | green at Simd8 |
| 1. Simd4 | No AArch64 toolchain or qemu here, and the wasm gate guest binds no padded bank. So the Simd4 evidence on this host is the unit tests above, which bind four-lane banks through the production bind body. `tests/bank.rs` is width-agnostic and runs at Simd4 in CI's `aarch64-debug`. The leg resolved on x86: its 25 product crates plus `dsp-reference`, `conformance` and `target-smoke` list 1,746 tests with the leg's features, all five new unit tests and the new bind test among them. `aarch64-known-defects.py judge-skips debug` accepts the listing, and the no-silent-skip scan finds nothing. | resolved |
| 2. Clone source | Every differential scenario binds the same members twice. Bank 0's padded lanes clone one member; bank 1's clone a different member (when there are two). Both banks must equal the per-node instances every block. | green |
| 3. Never scattered; `+0.0` at rest | In the differential and in every fixture leg (`Layout::fold`), a padded lane is fed only what the bank left in it and never read by the caller. Every block it must have written exactly `+0.0` (bits 0), have an empty report, and hold its bind-time state payload. `padded_banks::a_padded_lane_fed_positive_zero_writes_positive_zero_at_rest` covers the L4 clause. Setup: every band family at the four gain/Q corners (`+-24` dB, Q `0.1`/`18`) on all four bands, both cuts on, right channel detuned, cloned into every padded lane of a one-member bank at both widths. The member is driven through every schedule: elided, refused (`-0.0`, then NaN), ramping, and faulting. Each padded lane's output words must all be bit 0, and all 24 integrator words of its payload must be bit 0, on every block. | green |
| 4. Planted non-finite state | `padded_banks::a_planted_non_finite_state_recovers_its_own_lane_alone`. Both widths, every member count, every member as the victim. A NaN or infinity is written straight into one integrator of a general band, left or right, since a restore refuses a non-finite word. The victim's scalar twin gets the same plant. Block 0: the victim alone reports, on that channel alone. Every member equals its twin in words, report and state for three blocks. Every bank-mate stays audible on the fault block. Padded lanes are neither reported nor written. | green |
| 4. Bypassed lane fed a tripping value (M2) | `padded_banks::a_bypassed_lane_fed_a_tripping_value_leaves_its_bank_mates_bits`. Both widths, every member count, every member bypassed in turn. The bypass goes through the real `BypassShunt`: capture, render, then restore the bypassed column. That lane is fed `6e29` behind a +24 dB bell, past `1e30` after the boost, on alternate blocks. Each bank renders twice, hot and quiet. Every enabled member's words, report and state must match between the two runs and match its per-node instance, which never faults. The bypassed lane is its dry input, and the hot lane did fault. | green |
| 4. Padded lanes neither reported nor charged | `padded_banks::a_padded_lane_is_never_written_reported_or_charged`, both widths. A target on a padded lane is refused (`Capacity`) and a restore is refused (`effect.bank.track`); the same calls on a member succeed. Spans addressed to the padded lane are not charged, while a member's are. A NaN fed to the padded lane (a caller breach) is recovered to `+0.0` at rest and not reported, and no member moves against a control bank. | green |
| Bind (P2a L1, L3) | `tests/bank.rs::a_padded_request_binds_after_every_lane_is_validated` replaces #1088's decline test. Every mask `0..members` binds. At every member count from two, a malformed **last** member (bad limits, or a NaN initial value) is refused with `prepare`'s code. A padded lane that is not a clone declines. A mask with a member after a padded lane is `effect.bank.mask_not_prefix`. | green |
| 5. Tests and scripts | `cargo test -p parametric-eq -p graph-compiler -p graph`: 28 binaries, 361 passed. `check-parametric-eq-render-contract.sh`, `check-effect-runtime-policy.sh` (and its mutation test), `check-realtime-policy.sh` (54 regions, and its mutation test): ok. **Realtime audits**, release: `audit` capi, delay, compressor, parametric-eq (100,000 blocks each) and gate-expander. Also the builtins, builtins-graph and graph traces, the protocol allocation audit, the realtime probes and 1,000,000-block trace, the builtins and builtins-graph probe mutation tests, and the effect-contract 1,000,000-call trace. All report 0 allocations, 0 deallocations, 0 syscalls and 0 violations. **Scratch probe**, deleted after use: a padded bank at every member count, dual and collapsed, under the audited allocator. After 16 warm-up blocks, 32,000 blocks with 13,442 reported lane faults made **0 allocator calls**. | green |
| PR evidence: console digests | `console-workload`'s ignored `digests` harness, in release, covers all 21 native session rows over 64 blocks. It was run at the head and with `b2027254`'s `parametric-eq/src/lib.rs` swapped in, sharing one target dir: **identical**. `console-workload`'s pinned release digests pass. | identical |

Also run, all green:

- **Build, lint, docs.** `cargo fmt --all --check`. `cargo clippy --locked --workspace
  --all-targets --all-features -- -D warnings`. `RUSTDOCFLAGS='-D warnings' cargo doc --locked
  --workspace --no-deps` is clean except the known `tools/console-workload/src/lib.rs:374`; that
  crate's only error is that line, and the rest of the workspace documents clean.
- **Debug.** CI's debug-a set: 92 binaries, 1,103 passed, 0 failed, 10 ignored. CI's debug-b DSP
  set: 149 binaries, 803 passed, 0 failed, 28 ignored.
- **Release.** `audit`, `bench`, `console-workload`, `parametric-eq`, `graph-compiler`, `graph`,
  `effect-contract` and `rack`: 50 binaries, 600 passed, 0 failed.
- **Cross-target.** `check-cross-targets.sh` passes: x86-64-v3; the AArch64 iOS and Android
  product crates checked and linted; wasm `simd128`, including `parametric-eq`; the armv7 and
  scalar-wasm refusals. The iOS `_memset_pattern16` row reads `parametric-eq 146 calls, down from
  151`.
- **Wasm.** `run-wasm-gates.sh` passes: native, wasm `simd128`, and the V8 EQ loops.
- **Artifact gates**, on a fresh `build-web-audioworklet.sh` set:
  `check-web-audioworklet.sh --without-metadata-regeneration`, including the render-export
  callgraph closure and the kernel roster; `check-browser-expected-resources.py --artifacts`;
  `check-scalar-oracle-absent.py --wasm`; `test-web-audioworklet.sh`; the V8 spill gate and its
  self-test.
- **Native.** `check-capi-abi.sh` and `check-scalar-oracle-absent.py --native`.
  `check-effect-contract.sh target/release/bench` (8 production factories).
  `check-console-fixtures.sh`, `check-builtins-fixtures.sh` (50 files) and
  `check-graph-determinism.sh` (100/100).
- **Policies.** Workspace, graph, rack and lane policies. The realtime audit-leak check and its
  test. `check-test-support-ci`, `check-script-reachability` and `check-ci-path-routing`.

### The fixture pins that moved

The dual and collapsed bank rows of all five `tests/bank.rs` pin tables moved. They were recorded
when a fault zeroed and reset its whole bank plane, and every scenario plants faults on some tracks
only. Now each lane recovers alone, as its per-node instance does. Three facts pin the move to that
one cause:

- The same refactored test, with the whole-plane recovery restored, prints all fifteen old rows.
  So the `Layout` refactor moved nothing.
- Each new dual row is the scalar (per-node) row. For switched-off-cut it is the new eight-track
  per-node digest. The test now asserts this.
- The collapsed body shares the recovery. Its members are checked against per-node instances in
  the differential's collapsed scenarios, and every padded collapsed leg equals the full one.

| scenario | dual row | collapsed row |
|---|---|---|
| odd-live (#976) | `247bc0b6…` -> `81015a5c…` (= scalar) | `e9041804…` -> `e6467c9f…` |
| admitted-select (#977) | `d68a2494…` -> `3719d502…` (= scalar) | `e5db81b9…` -> `d7a3f759…` |
| skewed-pass (#978) | `aad039b4…` -> `9fdeb65d…` (= scalar) | `602d2f39…` -> `b82939da…` |
| switched-off-cut (#979) | `9e6886cf…` -> `9cc67918…` (= 8-track per-node) | `171406a7…` -> `70307424…` |
| block-limit (#999) | `033bb41c…` -> `69929ee0…` (= scalar) | `0da773b7…` -> `03c418fc…` |

No scalar row moved.

### Red mutations (each reverted)

| Mutation | Red tests |
|---|---|
| Whole-plane recovery restored (the recovery mask selects every lane) | all four padded-bank unit tests then present; all five fixture tests, at the per-node assertion |
| Spans charged to a padded lane | the bookkeeping test |
| Faults reported for a padded lane | the bookkeeping test |
| Target on a padded lane accepted | the bookkeeping test |
| Restore on a padded lane accepted | the bookkeeping test |
| Lane 0 designed from the last lane's clone | the differential, on **bank 1 only** (the second clone source) at seed 0, where bank 0's clone happened to be lane 0's own values: gate 2's discrimination. Also the planted and bypassed tests |
| A padded lane's integrator seeded at bind | the `+0.0`-at-rest test, the differential, bookkeeping, planted |
| A padded lane writes one word | the differential, bookkeeping, planted |
| The lane mask selects no failed lane | three padded-bank tests, the `ramping_elision` differential, all five fixture tests |
| The #1088 guard restored after the member loop | four padded-bank unit tests (the fixture legs too) |
| A decline taken before the member loop on a malformed later member | the bind test only (L1) |

All were reverted, and the file was compared byte-for-byte with the committed one.

### The AudioWorklet artifact

**This change alters the AudioWorklet module, and this slice does not re-pin it.**

| | Module digest | Size |
|---|---|---|
| Base `b2027254` | `b2eeb2d98e2013bffc1271828291bbf3f59d1ba233491bd66703e6f5fc53175d` | 3,262,117 B |
| Head `858ccb84` | `30b66801e609c94dca2987f33733b09ee92513a7e64d17011f6927ba5707bff6` | 3,261,159 B |

- The head module is the one the delivery build, `run-wasm-gates.sh` and the gates above all used.
- The base was built with the base `parametric-eq/src/lib.rs` swapped into the same tree.
- It is 958 B smaller.
- The render closure, the kernel roster and the V8 spill gate pass on it.
- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` still holds `6c952a2c…`,
  which was already stale at base.

### Residuals

- **Lower the iOS memset row.** `scripts/lib/aarch64-known-defects.py`'s `parametric-eq` row can
  drop from 151 to 146. The script says so, and its ceiling is a ratchet, but the file is outside
  this slice.
- **Stale sentences outside this slice:**
  - `crates/parametric-eq/src/corpus.rs` says the boundary check "would couple lanes".
  - `conformance`'s randomized differential says a failing block resets "every lane's" state "in
    most launch effects". Its hostile-block allowance still holds.
  - `docs/EFFECT_CONTRACT_V1.md`'s D7 bullet says a failing block "zeroes its output" and resets
    "that effect's state". For the EQ, that is now per lane.
- **P1's M2 is closed for the EQ**, and only the EQ. The compressor (#1090), the gate, transient
  shaper and soft clip (#1092), and the multiband (no owner, per P1's verdict) still recover
  whole-bank.
- **Mono collapse on a padded bank.** `lane_channel_symmetry` answers truthfully for a padded
  lane, from its clone's words. So whether a padded bank collapses can depend on which member is
  cloned. The collapse moves no bit (it is class A), so this couples cost only, which decision 12
  allows. Answering `true` for a padded lane would decouple it, but the contract calls a wrong
  `true` the one unsound answer. This slice does not make that call.

## Sol verdict, attempt 1

Sol, 2026-09-30. Verified `2b993c6d` (`8ce84d41`..`858ccb84` and the evidence, on P2a's `b2027254`)
against this body, decision 12, the umbrella, P2a's contract and verdict, and P1's M2.

**PASS.** Every moved bit is the whole-bank to per-lane D7 change and nothing else, shipped plans
move no bit, render stays allocation-free, and five of Sol's six planted mutations go red. One
medium finding is a closing condition, not a blocker: gate 1's NEON leg is still owed by CI.

### Evidence

**The re-pinned rows (the hardest claim).** Five scenarios moved two rows each, dual and
collapsed. Scalar rows did not move. Two independent runs in a scratch worktree tie every moved
bit to the recovery:

- **E1.** Base's unrefactored `tests/bank.rs` against the head source. Six tests pass. The five
  scenario tests fail at their `bank` row, and the #1088 decline test fails as expected. The digests
  they print are exactly the new pins, dual and collapsed. Each dual digest equals its scalar row:
  odd-live `81015a5c…`, admitted-select `3719d502…`, skewed-pass `9fdeb65d…` and block-limit
  `69929ee0…`. So the `Layout` refactor moved nothing. For switched-off-cut, the head test asserts
  that the dual row equals `cliff_scalar_digest(8)`, and it passes.
- **E2.** The same file with the head source and `recover_failed_lanes` restored to
  `io.fill(0.0); self.reset_states()`. All ten old pins pass, and only the decline test fails. So
  nothing else in the source moves a pinned bit: not the bind, the `active` array or the report
  masking.
- **The rows that moved.** All five scenarios plant faults on some tracks only. The whole-plane
  recovery silenced the tracks beside those faults. Per-lane recovery is decision 12's coupling fix
  (M1, amendment 6, and M2's "D7 recovery … bit-neutral per lane"). No row without a fault moved.
  Every other EQ fixture test and pin is unchanged and passes.
- **At `W = 1`.** `lanes_mask::<f32>(1)` selects lane 0, so the scalar recovery is the old one word
  for word. The unchanged scalar pins confirm it.

**Coupling, from Sol's own differential.** A scratch module `sol_probe`, never committed, with its
own RNG, parameter draws from the descriptor domains, and input families. Configs are drawn with
per-channel enable and family too, and every draw prepared.

- **Widths and members.** Banks bind through `bind_bank` at W = 4 and 8, for every member count
  `1..=W`.
- **Three banks per scenario**, every block compared with one `prepare`d twin per track:
  1. padded, cloning one random member, with the padded lanes left resident;
  2. padded, each lane cloning a random member, with the padded lanes fed garbage every block
     (NaN, inf, `-1e30`, `-0.0`, subnormals, `3e38`), which breaches the contract;
  3. a full bank whose extra lanes are real tracks fed hostile and hot input.
- **What the blocks carry:**
  - a bypass shunt on random lanes, which are fed `6e29` and `9.5e29` sines half the time;
  - `-0.0`, subnormal, NaN, inf and ceiling-edge words;
  - raw spans on every lane, padded ones included;
  - prepared-target ramps, both resets, and `+-f32::MAX` restores;
  - ragged blocks, and the collapsed body.
- **What must match the twin:** every active lane's words (class A), report and state payload.
  Every padded lane stays unreported, and targets and restores addressed to one are refused.
- **Gate 3.** In bank 1, every padded lane wrote bit-exact `+0.0` and held its bind-time payload,
  with all 24 integrator words at `+0.0`, on every block.
- **Scale.** Release, 60 seeds: W4 9,600 blocks and W8 19,200 blocks, each through all three banks.
  It reached 3,253 and 11,118 blocks with an isolated fault, 897 and 3,139 bypassed lanes that
  faulted, 3,040 and 5,240 collapsed blocks, 15,290 and 61,850 spans, 8,456 and 17,044 retargets,
  681 and 1,346 restores, and 367 and 796 resets. **All green.** The same probe goes red with the
  whole-plane recovery restored.
- **Link modes.** The EQ declares dual-mono only. `Maximum` and `Average` are refused with
  `effect.link_mode.unsupported` by `prepare` and by every bind, full or padded, at both widths.
- **Long rest.** In release, 20 designs x 2 widths x 2,000 blocks. A one-member bank plays, ramps,
  refuses and faults, and its padded lanes stay at `+0.0` and at rest.

**Class A for shipped plans.**

- `console-workload`'s ignored `digests` harness, in release, at the head and at `acc64d42` (a
  detached worktree): **all 22 rows identical**, the 64-track console included.
- `cargo test --release -p console-workload`: 68 tests pass, and the pinned digests with them.
- `check-console-fixtures.sh` and `check-builtins-fixtures.sh` (50 files) pass, as does
  `check-graph-determinism.sh` (100/100).

**Realtime.**

- **Every CI `audit-native` step**, in release, reports 0 allocations, deallocations, locks and
  syscalls: `audit` capi, delay, compressor, parametric-eq (100,000 blocks) and gate-expander; the
  builtins, builtins-graph and graph traces; the protocol audit; the realtime probes and the
  1,000,000-block trace; the probe mutations; and the 1,000,000-call effect-contract trace.
- **The EQ audit never faults**, so it does not reach this slice's path. Sol's audited-allocator
  probe does. It renders a padded bank at every member count, dual and collapsed, with ragged
  blocks, spans, faulting members and garbage-fed padded lanes: 4,608 blocks and 2,513 lane faults
  made **0 allocator calls**, in dev and in release. The new code is loads, compares, selects and
  stores, and carries no I/O.
- **Callgraph.** `check-web-audioworklet.sh --without-metadata-regeneration` passes on a fresh
  delivery build: render closure 8, kernel roster ok. `check-parametric-eq-render-contract.sh`
  passes, and so does `check-realtime-policy.sh` with its test.

**The two gate fixes are genuine.**

- **The V8 spill gate.** The head passes. Its held loops are identical to base: the dual depth-1
  tail has 109 instructions, the mono pair 78 and the mono tail 53, all with no carried slot. The
  unheld dual pair goes from 182 to 183 instructions, with 10 slots at both.
  - With `#[inline(never)]` removed, the module (`14fc168c…`) fails the gate: one failure.
  - Outlining a cold fault path keeps the hot loops as they were at base. That fixes the cause; it
    does not hide it.
- **The iOS memset count.** 151 at base, 146 at the head, and 146 on a merge of
  `codex/batch-console-2` with this branch.
  - Per function: `process_bank<f32x4>` goes from 84 to 77, and base's outlined
    `nonfinite_lane_mask<f32x4>` (2) is gone.
  - Each `recover_failed_lanes` (`f32x4` and `f32x8`) carries 2.
  - The mask is built from one flag vector compared with `+0.0`, which is bit-equivalent (every
    test above). The libc calls are really gone, which is exactly what the ratchet measures.

**AArch64 (question 5).**

- **Lower the row.** Root should lower `parametric-eq` in `scripts/lib/aarch64-known-defects.py:67`
  from 151 to 146 when #1089 lands on the batch. The count is deterministic under the pinned 1.97.1,
  measured on the merged tree, and the script itself asks for it.
- **NEON risk is low.** The recovery adds only `load`, `lt`, `select` and `store`, all exact and
  bitwise (`vbslq`). No arithmetic changed, and nothing touches #1019's `fmaxnm` folding.
- **The iOS and Android rows** of `check-cross-targets.sh`, check and clippy `-D warnings` with all
  targets and all features, pass. So `padded_banks` compiles for AArch64. See M1 for what is still
  owed.

**The AudioWorklet artifact.**

- Base (`acc64d42`, built here) is `b2eeb2d9…` at 3,262,117 B. The head is `30b66801…` at
  3,261,159 B, −958 B. Both match Terra's digests.
- `twiggy diff`:
  - bind: `prepare_width` −5,188 against `bind_homogeneous_bank` +4,984;
  - render: dual `process_bank` −1,161, scalar `process` −562, `nonfinite_lane_mask` −392,
    `process_bank_mono` −183;
  - new: `recover_failed_lanes` +946 (`f32x4`) and +461 (`f32`);
  - names +90.
- All of it is in the EQ's bind and D7 path, so the change is proportionate. Not re-pinned.

**Gates.**

- **Lint.** `cargo fmt --all --check` passes. So does `cargo clippy --locked --workspace
  --all-targets --all-features -- -D warnings`.
- **Rustdoc.** `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` stops only at
  the known `tools/console-workload/src/lib.rs:374`. With `--exclude console-workload`, the
  workspace documents clean.
- **Cross-target.** `check-cross-targets.sh`: PASS.
- **Wasm `simd128` checks** all pass:
  - CI's two `target/ci/wasm-simd` lines;
  - `parametric-eq --all-targets` with `test-support`;
  - `host-web --all-features`.
- **Tests.**
  - `cargo test -p parametric-eq -p graph-compiler -p graph`: 28 binaries, 361 passed, in dev and
    in release.
  - CI debug-a: 92 binaries, 1,103 passed, 0 failed.
  - CI debug-b: 149 binaries, 803 passed, 0 failed.
  - `audit`, `bench` and `console-workload` in release: 114 passed.
- **Wasm and artifact.** `run-wasm-gates.sh` passes: native, `simd128` and the V8 EQ loops. So do
  `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm` and
  `--native`, `test-web-audioworklet.sh`, and the spill self-test.
- **Scripts.** Each passes:
  - `check-parametric-eq-render-contract.sh`;
  - `check-effect-runtime-policy.sh` and its test;
  - `check-realtime-policy.sh` and its test;
  - the workspace, lane, rack and graph policies;
  - `check-realtime-audit-leak.sh` and its test;
  - `check-conformance-boundaries.sh`;
  - `check-capi-abi.sh` and its self-test;
  - `check-effect-contract.sh` (8 factories).

**Test value (question 6).** Six mutations of Sol's own, each reverted, run against the
committed `padded_banks` unit tests and `tests/bank.rs`:

| # | Mutation | Result |
|---|---|---|
| S1 | `lanes_mask` never selects the last lane | red: the differential, bookkeeping and planted tests; block-limit |
| S2 | recovery clears `ic1` but not `ic2` | red: three unit tests, all five fixture tests |
| S3 | a padded-lane restore writes, then returns the refusal | **survives** (L1) |
| S4 | `active` built all-`true` (the mask ignored) | red: bookkeeping |
| S5 | `render_mono` drops `failures[1] = failures[0]` | red: the differential, all five fixture tests |
| S6 | the report masks on `active[0]` instead of `active[track]` | red: bookkeeping |

Sol's probe catches S3: a padded lane then writes a word other than `+0.0`.

**Merge (question 9).** `git merge-tree` reports **no conflicts** in any of the three merges:

- into `codex/batch-console-2` (`1f76681c`);
- with `codex/1090-pad-compressor-banks` (`f7849fe0`);
- batch + #1090 + this branch.

On the merged batch + #1089 tree, `cargo test --release -p parametric-eq --features test-support`
passes 131 tests.

### Findings

No high finding.

- **M1. Gate 1's NEON (or wasm) leg is not yet evidenced.** Gate 1 asks for Simd4 "through
  `scripts/run-aarch64-tests.sh` or the wasm gates".
  - Both local Simd4 runs, Terra's and Sol's, are x86 SSE `f32x4`. The wasm guest binds no padded
    bank, and this host has no arm64 runner or qemu.
  - The route is `full`, so CI's required `aarch64-debug` and `aarch64-release` legs will run
    `tests/bank.rs`'s padded legs at native Simd4 and the `padded_banks` unit tests on NEON.
  - **Condition: close #1089 only after both legs are green on the batch push.** The risk is low
    (see AArch64), and Terra disclosed the gap.
- **L1. The padded-lane restore refusal is not pinned all-or-none.**
  - `a_padded_lane_is_never_written_reported_or_charged` restores the padded lane with its own
    snapshot (`crates/parametric-eq/src/lib.rs:9451-9455`). So a refusal that writes first (S3)
    stays green.
  - Restoring a member's payload with a planted integrator, then asserting the padded lane's
    snapshot and its `+0.0` output, would pin it. Nothing in production addresses a padded lane
    today.
- **L2. "#1070's order holds" is overstated** (this spec, line 95).
  - `bind_bank` still declines at the first heterogeneous lane before it validates later lanes
    (`crates/parametric-eq/src/lib.rs:3328-3333`).
  - Sol's probe: lane 1 heterogeneous and lane 2 malformed returns `Ok(None)`, where `prepare`
    refuses lane 2 with `effect.prepare.capacity`. That holds at every member count from three, full
    or padded.
  - This is pre-existing, unchanged by this slice, and #1070's open defect. This slice only had to
    remove the #1088 guard without adding a decline, and it did. #1070 stays open.
- **L3. Row count.** The evidence says 21 digest rows (line 143); the harness prints 22. All 22
  are identical.
- **L4. Residuals outside the slice.**
  - Terra's stale-sentence list is accurate: `docs/EFFECT_CONTRACT_V1.md:39`,
    `crates/parametric-eq/src/corpus.rs:18-19` and conformance's randomized text.
  - #1090 adds a parallel per-lane recovery (`compressor::finish_lanes`). Once P2c-P2e land, the
    frame-zeroing half may belong beside `effect_runtime::bank::nonfinite_lane_mask`, in the spirit
    of #95's ratchet.
  - Root should give both an owner.
  - Terra's mono-collapse residual is cost-only, as stated.
