# Pad gate/expander, transient shaper and soft-clip banks with inactive lanes

Slice P2e of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's M1 and amendment 6 in
`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`). No open issue covers these
three effects.

## Problem

The gate/expander, transient shaper and soft-clip are on the console eligibility list (decision 12,
"Eligibility"), so each must bank at every track count. After P2a, each `bind_homogeneous_bank`
still declines a padded request:

- gate/expander at `crates/gate-expander/src/lib.rs:1036`;
- transient shaper at `crates/transient-shaper/src/lib.rs:816`;
- soft-clip at `crates/soft-clip/src/lib.rs:879`.

Two of them also need a fix before they can pad:

- **The gate** fills memberless lanes with all-zero parameters, which lie outside its declared
  domains (`gate-expander/src/lib.rs:1049`, `:308-315`).
- **Soft-clip** charges every lane in its D7 recovery (`soft-clip/src/lib.rs:1035-1048`).

## Smallest closable slice

Opt each of the three effects into padded requests under P2a's contract:
- a padded lane carries a clone of an active member's request, never zeros, is fed `+0.0`, and its
  output is discarded;
- D7 recovery and reports attribute active lanes only.

The gate's zero-filled memberless lanes are replaced by the clone rule. Work in three checkpoints,
one per effect. If the slice outgrows half a day, root may split it per effect without a rebrief.

Authorized paths: the bank binding, D7 path and lane bookkeeping of `crates/gate-expander/src/lib.rs`,
`crates/transient-shaper/src/lib.rs` and `crates/soft-clip/src/lib.rs`, their tests, and this spec.

## Dependencies

- *Let an effect bank bind a partial group with inactive lanes* (P2a, #1088).

This is batch C2. P2b-P2e edit disjoint effect crates, so after P2a merges they may land in
any order, one merge each.

## Objective gates

1. For each effect and every active count 1..W-1, a padded bank's active lanes are bit-identical
   to the same tracks rendered per node. This holds on random input, on random parameters and on
   each effect's fixtures, at Simd8 on x86-64 and at Simd4 through `scripts/run-aarch64-tests.sh`
   or the wasm gates.
2. The gate: no lane is ever prepared with a parameter outside its declared domain. A planted
   zero-filled lane turns the test red.
3. Coupling rule: active lanes' bits do not depend on the clone source. For each of the gate, the
   transient shaper and soft-clip, a padded lane fed `+0.0` produces exactly `+0.0` out (not
   `-0.0`, not a denormal) and keeps its state finite and at rest, block after block (P2a verdict,
   L4).
4. D7: for each effect, a planted non-finite state in one active lane recovers and reports that
   lane alone. Soft-clip charges active lanes only.
   A bypassed lane counts as active: a bypassed lane fed a tripping value (for example `1e30`
   behind enough legal gain) leaves every enabled bank-mate's bits unchanged (P1 verdict, M2).
5. `cargo test -p gate-expander -p transient-shaper -p soft-clip -p graph-compiler -p graph` pass,
   as do `scripts/check-effect-runtime-policy.sh`, `scripts/check-realtime-policy.sh` and the
   realtime audits. PR evidence: console digests are unchanged.

## Non-goals

- No kernel change.
- #894 (silent-block admission for the gate and transient shaper) stays a separate performance
  issue.
- The multiband compressor is not padded until #1069 closes (decision 12, "Eligibility"). Its
  padding is a later issue.

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

Terra, 2026-09-30. Branch `codex/1092-pad-gate-shaper-clip-banks` from `2320454c` (P2a attempt 1 on
batch C2), with P2a's verdict commit `b2027254` merged (`06d8295d`). Commits `6f9204d8`,
`bdb44c24`, `66d3a532`, `78695d4d`, `0e25413b`, `ce1c7aca` and this record. "Base" below is
`b2027254`, the tree this slice changes.

### What landed

All three factories drop the #1088 decline and bind a padded request under P2a's contract. They
now rely on its members-first masks (L3) only where it is free; each keeps its active lanes as a
bitmask read from the mask.

- **Clone lanes.** Each lane, active or padded, is prepared from its own request, which for a
  padded lane is the caller's clone, so every lane is validated (a malformed clone is refused with
  `prepare`'s code). **The gate** no longer starts its lane table from zeros: every row, including
  the rows past a W4 bank's width and the scalar instance's unused rows, starts as a clone of
  lane 0's validated values (`bind_bank`, `prepare`).
- **Padded lanes take no automation**, so their entry in `BankProcessReport` stays empty even when
  a caller hands them spans.
- **D7, per lane.**
  - **Gate:** its recovery was already lane- and channel-local. A padded lane is still recovered
    but is never reported.
  - **Transient shaper:** the shared whole-bank `finish_block` is replaced by
    `Shaper::recover_lanes`. A lane that fails on either channel has both channels zeroed and both
    envelopes reset, exactly what its scalar instance does, and every other lane keeps its bits.
    Only active lanes are charged to the bank's `NonFiniteReport`. The contract report still counts
    nothing for this effect: that is #1073.
  - **Soft clip:** `SoftClip::recover_lanes` zeroes the failing lane's two channels and its rows of
    every history and snaps its ramps (`Channel::reset_lane_discontinuity`). The shared cursor is
    not moved: a zero ring reads the same at any rotation, the property a lane restore already
    relies on, so this is bit for bit the scalar instance's whole reset. Only the failing active
    lanes are charged, in the scalar's unit (the block's frames on both channels, #1073's to
    change). Before, every lane of the bank was reset and charged.
- **The D4 width check stays a compile-time constant.** `bind_homogeneous_bank` calls
  `bind_bank::<true>`; the unit tests call `bind_bank::<false>` to bind the other width's bank on
  the same host through the same code. Where the shaper and the soft clip box the concrete bank
  outside the generic binder, the check is repeated ahead of the only vtable reference (`boxed`).
  Without that, the four-lane browser module kept the eight-lane shaper and soft clip (+56,846 B,
  measured on the way; `0e25413b`).

### Gates

The committed tests are `src/padding_tests.rs` in each crate. Every one runs at **both** widths on
every host: the native width through the public factory, the other through `bind_bank::<false>`.
NaNs fold to one word throughout.

| Gate | Evidence | Result |
|---|---|---|
| 1. Padded active lanes = per node | `every_padded_bank_renders_its_members_per_node_bits`: every active count `1..W` at W4 and W8, 3 link modes (the soft clip: 3 seeds, it is dual-mono only), a drawn launch rate, drawn parameters in every domain (ends and defaults weighted in), drawn point automation, 10 blocks of drawn length, input mixing noise at six levels, `+0.0`, `-0.0`, subnormals and bursts. Every block, every active lane's words and report equal its scalar instance's, and at the end its state payload does. `padded_banks_render_the_fixtures_per_node`: the 7 conformance PCM fixtures (`fixtures/conformance/v1`), each at its own rate, four passes in blocks of 128, 64, 37 and 1, through every active count at both widths, with the crate's audible fixture parameters and the defaults. | green, 3 crates x 2 widths |
| 1. Simd4 | On x86-64 the W4 banks run in the same unit tests (`wide::f32x4`). NEON itself runs in CI's `aarch64-debug` leg: the three are product crates, and the tests are width-agnostic. Locally, `cargo clippy --all-targets -D warnings` for `aarch64-linux-android` and `aarch64-apple-ios` compiles them; there is no AArch64 runner or qemu here. The silent-skip scan of `run-aarch64-tests.sh` finds nothing (it found the gate's first spelling of the D4 check, fixed in `ce1c7aca`). | green on x86; NEON in CI |
| 1. Through the real graph (PR evidence, scratch probe, deleted) | A 1-64 track console with gate -> shaper in SIMD rack 1 and soft clip in SIMD rack 2, mixed session bypass, the real launch factories, P2a's `BankPadding::EveryGroup`, 24 blocks, against the bank-free per-node oracle. **Head:** every slot banks (1 track: 3 padded banks; 29 tracks: 12 banks), 0 per-node slots, output bits equal the oracle at 1, 2, 7, 8, 9, 11, 15, 29 and 64 tracks. **Base:** the factories decline, the same groups render per node (up to 21 per-node slots), same bits. | identical |
| 2. The gate: no lane outside its domain | `no_lane_is_prepared_outside_its_declared_domain`: every lane of banks of every active count at both widths; each lane's three times and its four ramps' current and target must lie inside the descriptor's `[minimum, maximum]`, and its payload must restore into a fresh scalar instance. | green |
| 3. Clone source | `active_lanes_do_not_depend_on_the_clone_source`: members-first masks of 1, 2, W/2+1 and W-1 members, one member at every parameter's minimum and one at every maximum; each render passes gate 1's checks, and the renders of one mask agree word for word across every clone source. | green |
| 3. Padded lane rest clause (L4) | Inside every gate-1 and gate-3 render, every block, every padded lane: exactly `+0.0` out (bits `0`, so not `-0.0` and not a subnormal), every state word finite, and the state equal to an idle track's (a scalar instance of its clone fed `+0.0` and no automation), while every other block hands it stray spans, one malformed. The shaper's and soft clip's padded lanes must also stay at their prepared state. **The gate's cannot:** fed `+0.0`, its detector sits at the level floor, so a padded gate lane closes and its gain releases toward its range exactly as an idle track's does; its parameters never move. "At rest" for the gate is read as that. | green |
| 4. Planted non-finite state | `a_planted_nonfinite_state_recovers_and_{reports,is_charged}_to_its_lane_alone`, W4 and W8, members-first mask with two padded lanes. **Gate:** a NaN planted in one lane's gain (the existing `cfg(test)` hook): its left block is zeroed and reset, it alone is reported, every other word (its right channel included) is the uninjected control's. **Shaper and soft clip:** a NaN on one lane's left input: both its channels zeroed, it alone charged (one block and its lane bit in `NonFiniteReport`; for the soft clip also its report), every other lane's bits and report equal the control's. The shaper's envelopes are reset. The soft clip's lane is checked against its scalar instance through the recovery with a drive ramp in flight: its history is zero and every ramp snapped right after, and output, report and state equal the scalar's a block later. **Planted in a padded lane** (against the caller's `+0.0` duty, to reach the case): recovered, nothing reported or charged, active lanes unchanged. | green |
| 4. Bypassed lane fed a tripping value | `a_tripping_lane_leaves_every_bank_mates_bits`: 2, W-1 and W members; one lane fed on every seventh frame of every other block a tripping value behind legal gain (the shaper: `2e29` through its +18 dB attack boost; the soft clip: `1e29` at zero mix and +24 dB output; the gate, which never raises a level: `1e30`), an infinity or a NaN. Every other lane keeps the control render's bits (and reports); the hot lane trips on every hostile block and is the only one charged. At the effect a session-bypassed lane is an active lane prepared `bypass = false` whose output the shunt later replaces (#1087), so this is its wet path. | green |
| 4. Through the rack's shunt (PR evidence, scratch probe, deleted) | P1's M2 probe re-built: one full bank per launch effect, every third lane session-bypassed (channel-less lanes, `ConsoleEffectBankStage`), bypassed lanes fed up to `9.9e29` every fourth block, enabled lanes loud; count of enabled bank-mates' words that differ from an all-loud control, 8 seeds x 16 blocks. **Base:** soft clip 20,604 of 163,840, transient shaper 97,998, gate 0. **Head:** soft clip 0, transient shaper 0, gate 0. Bypassed lanes off their prepared-bypass oracle: 0 everywhere. (EQ 79,891, compressor 52,446 and multiband 117,716 are P2b's, P2c's and #1069's.) | 0 moved |
| 5. Tests and scripts | CI's debug-b DSP set (the three crates, conformance and the other DSP crates): 149 binaries, 814 passed, 0 failed, 28 ignored. CI's debug-a set (graph-compiler, graph, effect-compiler, rack, host-core, capi...): 92 binaries, 1,103 passed, 0 failed, 10 ignored. Release, `gate-expander transient-shaper soft-clip graph-compiler graph effect-compiler effect-contract rack audit bench console-workload`: 77 binaries, 611 passed, 0 failed (console-workload's pinned digests included). `check-effect-runtime-policy.sh`: ok. `check-realtime-policy.sh` (54 regions) and `test-realtime-policy.sh`: ok. Realtime audits, release: `audit capi` (100,000 calls), `delay`, `compressor`, `parametric-eq` (100,000 blocks each) and `gate-expander` (bank bound): 0 allocations, 0 locks, 0 syscalls, 0 violations. `trace-graph-audit.sh`: PASS, 1,000,000 blocks. `check-effect-contract.sh target/release/bench`: ok, 8 factories. | green |
| 5. Render allocates nothing, padded (PR evidence, scratch probe, deleted) | A counting global allocator around `process_bank` of padded banks of all three effects, every active count `1..8`, 2,000 blocks each with automation on active lanes, strays on padded ones and a NaN tripping lane 0 every eleventh block: 0 allocator calls (frees included). | 0 |
| 5. Console digests (PR evidence) | `console-workload`'s ignored `digests` harness, 64 blocks of all 22 native session rows, release, base (separate target directory) and head: **identical**, all 22. | identical |

### Red mutations

Run on the tests' final shape (`78695d4d`); the later commits change only the D4 check's form. Each
reverted.

| Mutation | Red |
|---|---|
| The #1088 decline restored (all three) | every padding test that binds, and `a_padded_request_binds_and_still_validates_every_lane` |
| Gate: padded lanes bound from zeros instead of their clone | gate 2's test; gates 1 and 3 through the rest clause (state is not an idle track's) |
| Gate: a padded lane's recovery reported | the planted-state test (padded case) |
| Gate: whole-bank recovery | planted-state, tripping-lane, and the existing `injected_nonfinite_gain_has_scalar_parity_at_the_native_width` |
| All three: members read from the tail of the requests | gates 1 (random and fixtures) and 3 |
| All three: padded lanes run their spans | gates 1 and 3 (the stray's report) |
| Soft clip: padded lanes run their spans into a discarded report, so only their state moves | gates 1 and 3: "padded lane 1's state is not an idle track's" (the L4 clause alone) |
| Shaper: whole-bank recovery | planted-state, tripping-lane, `a_nonfinite_lane_is_rejected_alone` |
| Shaper: a padded lane charged | planted-state |
| Shaper: recovery keeps the envelopes | planted-state, both boundary tests |
| Shaper: the failing lane keeps its right channel | planted-state, tripping-lane, both boundary tests, the randomized differential |
| Shaper: no boundary check | planted-state, tripping-lane, both boundary tests, conformance, randomized (the MUTATIONS.md row 6 note) |
| Soft clip: whole-bank recovery | planted-state, tripping-lane, `a_bank_block_fails_and_recovers_the_failing_lane_alone` |
| Soft clip: every lane charged (the old report) | planted-state, tripping-lane, the boundary bank test |
| Soft clip: a padded lane charged | planted-state |
| Soft clip: recovery keeps the histories | planted-state, tripping-lane, both boundary tests |
| Soft clip: recovery leaves the ramps | planted-state ("parameter 0's ramp is not at rest"); the scalar oracle shares the code, so only the explicit check sees it |
| Soft clip: the failing lane keeps its right channel | planted-state, tripping-lane |

### The AudioWorklet artifact changes, and this slice does not re-pin it

| | Module digest | Size |
|---|---|---|
| Base `b2027254` | `b2eeb2d98e2013bffc1271828291bbf3f59d1ba233491bd66703e6f5fc53175d` | 3,262,117 B |
| Head | `3efdd4b8ac6623b0270214f67f16bd8142892a20a91c81b486c1d49307920714` | 3,264,718 B |

- +2,601 B (twiggy): the per-lane D7 recovery and history reset, the padded binding and the gate's
  lane-table fill. No eight-lane code.
- The head module is `run-wasm-gates.sh`'s build and the artifact job's build, byte for byte.
- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` still holds `6c952a2c...`,
  already stale at base.
- Artifact gates on a full build: `check-web-audioworklet.sh --without-metadata-regeneration`
  (including the render-export closure), `check-browser-expected-resources.py --artifacts` (native
  parity digests agree), `check-scalar-oracle-absent.py --wasm`, `test-web-audioworklet.sh`, and
  the V8 spill gate inside `run-wasm-gates.sh`: all pass.

### Also run, all green

- `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D
  warnings`.
- Rustdoc: `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` fails only on the known
  `tools/console-workload/src/lib.rs:374`; the workspace without that crate is clean, and so are the
  three crates with `--document-private-items`.
- `scripts/check-cross-targets.sh`: PASS (AArch64 iOS and Android product crates, wasm `simd128`,
  refusal rows). The wasm `simd128` `cargo check -p host-web --all-features` passes.
- `scripts/run-wasm-gates.sh`: ok (native, wasm `simd128`, V8 EQ loops).
- `check-workspace-policy.sh`, `check-console-fixtures.sh`.

### Path deviations, for Sol

- New test modules `crates/{gate-expander,transient-shaper,soft-clip}/src/padding_tests.rs`
  (`#[cfg(test)]`), so the other width binds through the crate-private binder.
- The gate's scalar `prepare` fills its lane table with the validated values instead of zeros (lane
  bookkeeping; only row 0 is read), and `inject_nonfinite_gain` is `pub(crate)` for the new module.
- `crates/transient-shaper/tests/MUTATIONS.md`: a note that row 6's second test was renamed.
- The binders were split (`bind_bank::<NATIVE_ONLY>`, and `boxed` in two crates) so tests reach the
  other width without a runtime flag in production.

### Found on the way

- **A shared target directory lies about the base.** A base worktree built into this worktree's
  `target/` gets the same package hashes, and its dep-info paths are relative, so the head's next
  build reused the base's effect crates as fresh: a first head run of the rack probe showed the base
  numbers. Every base-versus-head comparison above was re-run with the base in its own directory
  (`target/base-1092`, deleted) after cleaning the three crates.
- **The module growth in `0e25413b`'s message**, described above.

### Residuals

- `crates/soft-clip/src/kernel.rs:45` still says output finiteness is checked by `finish_block`.
  The file is outside this slice's paths ("no kernel change"); the check itself is unchanged, only
  the recovery moved.
- #1073 is untouched: the shaper's contract report counts no D7 block, and the gate and soft clip
  count frames. This slice only keeps those counts to the failing active lane.
- A padded gate lane's state is an idle track's, not frozen (gate 3, L4 row above).
- The M2 coupling remains for the EQ, compressor and multiband (P2b, P2c, #1069).
- The iOS `memset_pattern16` expected-failure counts (#1018) were not compared with base.

## Sol verdict, attempt 1

Sol, 2026-09-30. Verified `7307238b` (`2320454c` with `b2027254` merged) against this body, the
padding contract on `PrepareEffectBankRequest`, P2a's verdict (L1, L3, L4), P1's M2 and decision
12. Class A base: `acc64d42` (the batch with P2a merged). Its tree differs from the head only by
this slice and the C1 rustdoc line.

**PASS.** Every objective gate holds on independent evidence. No shipped plan moves a bit, render
stays allocation- and syscall-free, and P1's M2 coupling is gone for all three effects. Six of
Sol's seven planted mutations go red in the committed tests. The survivor and the other findings
are low.

### Evidence

**Coupling (question 1): Sol's own differential.** A scratch probe, not committed, was compiled as
a unit-test module in each crate so that both widths bind on x86-64. It uses its own generator and
drives, per effect:

- 2,000 seeds, alternating W4 and W8, sweeping members `1..W`;
- every link mode (the soft clip is dual-mono only), a drawn launch rate, and a prepared-bypass
  bank one time in ten;
- random parameters, including domain ends and defaults, with two members sharing values one time
  in four;
- random point automation, plus invalid strays: `Linear`, `Both`, NaN, and out of order;
- 1 to 128 frames per block;
- a third of the tracks hostile, standing in for bypassed wet paths: 1e10 to 9.9e29, `1e30`,
  +/-inf, NaN, 3e38, `-0.0`, subnormals. Enabled tracks get a hot block one time in ten.

Each seed renders these configurations:

- the full bank;
- a padded bank cloning **each** member in turn;
- a padded bank whose padded lanes are fed hostile input, which breaks the caller's `+0.0` duty;
- the public factory at the native width.

Results, per effect: 9,500 bank renders and about 395,000 lane-blocks.

- **Every active lane equals its own scalar instance** in every configuration: output words (NaN
  folded), `ProcessReport`, and the final state payload.
- Every padded lane fed `+0.0` writes bits `0` and is never reported.
- **The gate's sidechain:** a connected sidechain declines at every member count. Only the
  unconnected port banks, as before.

**Per-lane D7 (question 2).**

- The same probe reads the shaper's and soft clip's internal `NonFiniteReport`.
  - Per block, the charge is exactly the OR of the active lanes whose scalar instance tripped.
  - `nonfinite_blocks` advances only then, including when a hostile padded lane trips alone.
  - Totals: 31,758 and 25,441 active-lane trips, and 23,885 and 18,694 charged blocks.
- Lanes are recovered alone, and the gate's per-channel reports equal the scalar's (21,761
  trips).
- Linked L/R is a unit exactly as the scalar is:
  - the shaper zeroes and resets both channels of a failing lane in every link mode;
  - the gate stays channel-local (#48).
- **Base against head, public API.** 400 scalar seeds per effect with tripping blocks, state
  payloads included, and 300 full W8 banks that never trip. All six digests are identical at
  `acc64d42` and the head. So:
  - the soft clip's recovery no longer returning the cursor to 0 moves no bit (its snapshot is
    age-relative);
  - a lane trips exactly when the old whole-bank D7 tripped it.

**P1's M2 through the rack (P1's gate 4).**

- **Probe.** `bypass_shunt_identity.rs`'s harness through `ConsoleEffectBankStage`, copied to
  scratch. Bypassed lanes run at `Level::Extreme`, enabled lanes loud. Run: 8 seeds x every case
  x 3 mixed patterns x 16 blocks.
- **Enabled bank-mates' moved words, base to head:**
  - soft clip: 72,999 to **0** (of 491,520);
  - shaper: 783,044 to **0** (of 1,474,560);
  - gate: 0 to 0.
- Bypassed lanes: 0 moved at both. EQ, compressor and multiband are unchanged (P2b, P2c, #1069).

**The gate change (question 3).**

- At base no lane below the width was ever memberless: a padded request was declined, and a full
  bank fills every lane. So the "zeros" were only the lane-table rows at or past `WIDTH`.
- Those rows are never read: `defaults[lane]` is read only by `seed_lane`
  (`gate-expander/src/lib.rs:454`), for lanes below `WIDTH`, from `new` and `reset_lane_full`.
- The fill is therefore unobservable in any shipped plan, and the digests agree. Gate 2's
  planted zero lane is red only for rows below the width, which is correct.

**Class A (question 4).** `console-workload`'s ignored `digests` harness, release, `acc64d42`
(detached scratch worktree, own target directory) against the head: **all 22 rows identical**.

**Realtime (question 5).**

- **Padded banks, Sol's probe.** A counting global allocator plus strace markers around
  `process_bank` of padded banks of all three effects, at W8:
  - every link mode and active count, 2,000 blocks each (112,000 blocks);
  - automation, a NaN on lane 0 every 11th block and 9.9e29 every 13th.
  - Result: **0 allocator calls, and 0 syscalls in all 56 marked windows**.
- **CI's `audit-native` steps, release.** All report 0 violations, allocations, locks and
  syscalls:
  - `audit` capi, delay, compressor and parametric-eq (100,000 blocks each);
  - `audit gate-expander`;
  - the builtins, builtins-graph and graph traces;
  - the protocol allocation audit;
  - the realtime probes and the 1,000,000-block trace;
  - the 1,000,000-call effect-contract trace, and `check-effect-contract.sh` (8 factories).
- **Policies.** `check-realtime-policy.sh`, `test-realtime-policy.sh` and
  `check-realtime-audit-leak.sh` pass.
- **Callgraph.** `check-web-audioworklet.sh --without-metadata-regeneration`, including the
  render-export closure, passes on a full delivery build.
- **V8 spill gate:** ok.
- **#1018 memset counts** (`aarch64-apple-ios`, release asm): gate-expander 181, transient-shaper
  534, soft-clip 22, at both `acc64d42` and the head. That equals the ceilings in
  `scripts/lib/aarch64-known-defects.py`, and `check-cross-targets.sh` judges them.

**Test value (question 6).** Seven mutations of Sol's own, each reverted:

| # | Mutation | Result |
|---|---|---|
| 1 | Gate: the padded-lane skip moved above the zeroing and reset (`lib.rs:683-688`), so a padded lane is neither recovered nor reset | red: `a_planted_nonfinite_state_recovers_and_reports_its_lane_alone` |
| 2 | Shaper: recovery resets `env.slow` only (`lib.rs:519`) | red: `a_nonfinite_block_is_zeroed_and_the_envelopes_are_reset`, `a_nonfinite_lane_is_rejected_alone`, the planted-state test |
| 3 | Soft clip: the lane reset skips the `dry` history (`lib.rs:403`) | red: 3 tests |
| 4 | Soft clip: the bank charges the left channel only (`lib.rs:1160`) | red: 3 tests, and Sol's probe |
| 5 | Soft clip: recovery also returns the shared cursor to 0 (base's whole `clear`) | red: `a_tripping_lane_leaves_every_bank_mates_bits`, and Sol's probe |
| 6 | Shaper: `nonfinite_lanes \|= charged` (`lib.rs:525`) | **survives** the committed tests; red only in Sol's probe (L1) |
| 7 | Soft clip: the same `\|=` (`lib.rs:893`) | **survives** (L1) |

Mutations 1-3 are invisible to any scalar-oracle differential, Sol's included, because the scalar
instance runs the same recovery code. The committed explicit checks catch them.

**Gates (question 8).** All green at the head:

- **Lint.** `cargo fmt --all --check`, and `cargo clippy --locked --workspace --all-targets
  --all-features -- -D warnings`.
- **Rustdoc.** `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` stops only at
  the known `tools/console-workload/src/lib.rs:374`. With `--exclude console-workload`, the
  workspace documents clean.
- **Wasm.** The wasm `simd128` `cargo check -p host-web --all-features` passes.
- **Cross-target.** `scripts/check-cross-targets.sh` passes: AArch64 iOS and Android product
  crates checked and linted `-D warnings`, the wasm `simd128` rows, and the armv7 and scalar-wasm
  refusals.
- **Silent-skip scan.** The `run-aarch64-tests.sh` scan finds nothing.
- **Tests.**
  - The three crates plus graph-compiler and graph: 48 binaries, 338 passed, 0 failed, 7 ignored,
    in dev and in release.
  - The merged tree (below): the same crates plus effect-compiler, release, 58 binaries, 380
    passed, 0 failed.
- **`scripts/run-wasm-gates.sh`:** native, wasm `simd128` and the V8 EQ loops.
- **Policies.** `check-effect-runtime-policy.sh` and its test, `check-workspace-policy.sh`,
  `check-lane-policy.sh`, `check-console-fixtures.sh` and `check-artifact-evidence-leak.sh`.
- **Artifact gates.** `check-browser-expected-resources.py --artifacts`,
  `check-scalar-oracle-absent.py --wasm` and `test-web-audioworklet.sh`.

**Artifact (question 9).**

- **Digests.** Base `b2eeb2d9…` (3,262,117 B), built at `acc64d42`. Head `3efdd4b8…`
  (3,264,718 B), which is both `run-wasm-gates.sh`'s build and the full delivery build. Both
  match the evidence record.
- **`twiggy diff`, +2,601 B.** Bind code:
  - soft clip bind +1,535 against `SoftClip::new` -1,129 inlined;
  - gate `prepare` +920, which is the lane-table fill;
  - gate bind +329.
- **The per-lane D7 path**, inlined into its render functions:
  - soft clip scalar `process` +845;
  - soft clip bank `process_bank` +713;
  - `reset_lane_discontinuity` +704;
  - shaper W4 `process_block` +674.
- Names -723.
- No eight-lane effect symbol is in either module.
- About 0.08 % of the module, and cold-path code only. Proportionate. Not re-pinned.

**Merge (question 10).** No conflicts. `git merge-tree` is clean for the head:

- into `codex/batch-console-2` (`1f76681c`);
- against `codex/1089-pad-eq-banks` (`2b993c6d`) and `codex/1090-pad-compressor-banks`
  (`f7849fe0`);
- in the sequence batch + 1089 + 1090 + 1092.

The batch + 1092 tree passes the release tests above. Its M2 and base-compat probes print the same
numbers as the head's.

**Not verified here.** NEON itself. The W4 banks run as `wide::f32x4` on x86-64. There is no
AArch64 runner, so CI's `aarch64-debug` leg carries NEON.

### Findings

No high or medium finding.

- **L1. `NonFiniteReport::nonfinite_lanes` is unpinned across blocks.**
  - Where: `crates/transient-shaper/src/lib.rs:525` and `crates/soft-clip/src/lib.rs:893`.
  - The field's contract is the lanes of the **most recently** rejected block. Accumulating them
    (`|=`) survives every committed test, because each test trips one lane only.
  - Nothing in production reads the field (the shaper's contract report is #1073's), so no bit or
    report moves.
  - Pin it with two different lanes tripping in consecutive blocks, or leave it to #1073.
- **L2. Stale docs after the per-lane recovery.** Root may apply the first correction as written.
  - `crates/soft-clip/src/kernel.rs:44-45` currently reads:
    ```
    //! either sign. Output finiteness is checked once per block by
    //! `effect_runtime::bank::finish_block`; there is no per-value check anywhere.
    ```
    Replace those two lines with:
    ```
    //! either sign. Output finiteness is checked once per block by `effect_runtime::bank::check_block`
    //! in `SoftClip::process`, and `SoftClip::recover_lanes` recovers a failing lane alone (#1092);
    //! there is no per-value check anywhere.
    ```
  - `crates/transient-shaper/tests/MUTATIONS.md:25` (row 6) still names `finish_block` as the
    mutation site. The note at `:94` explains the rename, which is enough.
- **L3. The gate's padded lane is "at rest" as an idle track, not frozen.**
  - Where: `crates/gate-expander/src/padding_tests.rs:34`.
  - Fed `+0.0`, it closes and its gain releases toward the range. Its output is `+0.0` throughout.
  - Sol measured when a scalar gate fed `+0.0` reaches a fixed point: its state payload last
    changes at block 1 (every parameter at its minimum), 348 (defaults) or 4,461 (maximum times)
    of 128 frames at 48 kHz.
  - The reading is accepted for gate 3, but gate 3's wording ("at rest, block after block") is
    literal only for the shaper and the soft clip.
  - #894's silent admission will need this settling time, not an immediate rest.
- **L4. The implementer's residuals are confirmed.**
  - #1073's report units are untouched: the gate and soft clip count frames and the shaper
    counts none. They stay out of scope; this slice only keeps the charge on the failing active
    lane.
  - The iOS memset residual is closed above.
  - The path deviations are accepted: `inject_nonfinite_gain` is `pub(crate)`, the scalar
    `prepare` fill is unobservable, and the test modules and the split binders are acceptable.
