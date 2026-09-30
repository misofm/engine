# Keep a bypassed lane in its effect bank

Slice P1 of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's finding H1 and amendment 1 in
`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`).

## Problem

A console slot's bypass must keep the track in its bank (decision 12, "Bypass"). Today it splits
the cohort:

- `EffectProgramKey` includes `bypass` (`crates/effect-contract/src/lib.rs:984-999`), so a
  bypassed track and an enabled track never share a bank. The key's own doc (`:950-983`) records
  why it is still there: every effect's bank reads one `metadata.bypass` for the whole bank, and
  `parametric-eq` does not run the wet path when bypassed (`crates/parametric-eq/src/lib.rs:3666-3668`).
- The session `bypass` becomes the prepared bypass (`crates/effect-compiler/src/prepare.rs:356`).
- A per-lane mechanism already exists outside the effects:
  - `EffectControlRecord::Bypass` (`crates/effect-contract/src/live.rs:89-95`);
  - `BypassShunt` (`live.rs:761-883`), which runs the wet path, preserves latency and selects whole
    blocks per lane;
  - the rack's live bank stage (`ConsoleEffectBankStage`, `crates/rack/src/lib.rs:911-1030`), which
    holds one control lane per bank lane and one AoSoA shunt.
- The gap: the shunt is built only when some lane has a live control channel
  (`rack/src/lib.rs:996-999`), and live lanes are seeded from the prepared bypass
  (`effect-compiler/src/prepare.rs:1331-1333`), which has already split the cohort.

## Smallest closable slice

For every bankable native effect, not only console slots (the console does not exist until S1a,
and the rule is the same wherever the effect sits):

1. A session `bypass` lowers to prepared `bypass = false` plus the lane's initial `BypassShunt`
   state.
2. The shunt is built for a bank whenever any lane is bypassed, whether or not a live control
   channel is attached.
3. `bypass` leaves the grouping identity, so mixed-bypass cohorts bind one bank. PDC is unchanged:
   route timings come from `PreparedEffectMetadata.latency` only.

A lane that ends up per node may keep today's prepared-bypass path, which skips the wet path. It
may also use the shunt. Its bits must be identical either way; choose one and say why.

Authorized paths:
- `crates/effect-contract/src/lib.rs` (the key) and `src/live.rs` (shunt construction only);
- `crates/effect-compiler/src/prepare.rs`;
- `crates/graph-compiler/src/banks.rs`;
- `crates/rack/src/lib.rs`;
- their tests, and this spec.

## Owner decisions that bind this slice

- A bypassed lane runs the wet path. That cost is accepted.
- Latency is always paid. A bypassed slot keeps its fixed latency on every lane, and a bypassed
  lane's impulse lands on the same sample as an enabled lane's.
- Banking may couple lanes' cost, never their bits. The shunt's per-lane select is a bitwise select,
  never an arithmetic identity: `fma(0, wet, dry)` turns `-0.0` into `+0.0`.

## Dependencies

None beyond decision 12. This is the first slice of batch C2, which starts after batch C1 (B0 and
S0's baseline) has been pushed, so that S0 timed the engine without this change.

P1 and P2a both edit `effect-contract/src/lib.rs` and `graph-compiler/src/banks.rs`, so P1 merges
before P2a starts.

This changes insert cohorts as well as console slots. On the app's current shape, bypassed tracks
stop forming their own cohort. S0 and S4's app-shape row measures that change.

## Objective gates

1. A graph-compiler test: a cohort of W tracks with any mix of bypassed and enabled lanes binds
   one bank, at Simd8 on x86-64 and at Simd4 through `scripts/run-aarch64-tests.sh` or the wasm
   gates.
2. A differential test: each bypassed lane's output is bit-identical to today's per-node
   prepared-bypass render. It covers random input, a `-0.0` input run and a latent slot's first
   block. The latent slot is the true-peak limiter at `rate/100 + 6` samples. NaNs fold to one
   value (decision 10). A planted `fma(0, wet, dry)` select turns it red.
3. Toggling bypass live on a banked lane still works and still preserves latency. The existing
   live-bypass tests pass unchanged.
4. PR evidence, not a committed test: console digests unchanged for sessions with uniform bypass.
5. `scripts/check-realtime-policy.sh`, `scripts/check-effect-contract.sh`,
   `scripts/check-rack-policy.sh`, the realtime audits and the callgraph gates pass. Render
   allocates nothing, measured with `bench_support::alloc`'s counters after warm-up.

## Non-goals

- No change to effect kernels.
- #892 (feed the dry line without a copy and swap when nothing is bypassed) stays its own
  performance issue. It matters more after P1, because a latent limiter bank with any bypassed lane
  now feeds a shunt.
- No bank-wide skip when every lane is bypassed; it is out of scope under decision 12.

## Standing rules for the implementer

- Work only from this body, the umbrella issue and decision 12. Read the cited code first.
- Class A: every gate that says "bit-identical" is a hard stop, not a tolerance.
- Render stays allocation-, lock- and syscall-free. Only `crates/lane` names `wide` or intrinsics.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings` and the focused tests before every checkpoint.
- Every new test names the plausible defect that turns it red (AGENTS.md "Test value").

## Attempt 1 evidence

Terra, 2026-09-29. Branch `codex/1087-per-lane-bypass` from `6fdf5db2` (`codex/batch-console-1`),
commits `1482a0c9`, `baa03f09`, `dff76022` and this record. Not merged into the batch: P1 waits
for S0's baseline.

### What landed

- **The lowering** (`effect-compiler/src/prepare.rs`). A session `bypass` on an effect that can
  bank prepares the effect with `bypass = false`, records the session bit as
  `EffectPreparedEntry::initial_bypass`, and hangs `EffectControlLane::without_channel(true)` on the
  entry. `attach_effect_console` seeds a live lane from `initial_bypass`, so a live console starts
  where the session says and can now lift a session bypass (before, the lane was seeded from the
  prepared bypass and a live `Bypass(false)` left the effect's own prepared bypass in force).
- **A lane without a channel** (`effect-contract/src/live.rs`). `EffectControlLane`'s queue is
  optional; `without_channel` carries only the bypass and the `UNBYPASSED` seed, its drain stages
  nothing, and its retained queue payload is the empty layout. `has_channel()` tells the two apart.
- **The shunt whenever a lane is bypassed** (`rack/src/lib.rs`). `ConsoleEffectBankStage` builds its
  shunt when any lane has a live channel or is bypassed at preparation, and a slot with no live
  channel holds no staging window (below). The graph runtime already chooses this stage whenever a
  bank member carries a lane, so no graph change was needed.
- **The key** (`effect-contract/src/lib.rs`). `EffectProgramKey` keeps its `bypass` field, and its
  doc now says why. The session bypass left the grouping identity because it no longer reaches the
  prepared metadata the planner reads: every key the planner compares for an effect that can bank
  carries `bypass: false`. Removing the field would change nothing on the session path. It would
  let a direct caller bind a mixed *prepared*-bypass cohort, though, and every effect's bank still
  reads one flag for all its lanes (no kernel change is in scope). With the field in the key, every
  `bind_homogeneous_bank` still declines such a cohort.
- **Banking docs** (`graph-compiler/src/banks.rs`) now separate a bypassed slot (it has its node
  and latency and banks) from a skipped one (an identity slot, P2a's). No planner code changed.
- **Doc** `docs/EFFECT_CONTRACT_V1.md`: its bypass sentence said bypass was not in the key, which
  was false before and after. It now states the prepared flag and the session lowering.

### The per-node choice: the shunt, and one exception

A lane that ends up per node uses the shunt. Preparation cannot know which instances the planner
will bank, and the graph's per-node `ConsoleEffect` already has the exact order the shunt needs:
capture, then the effect, then the copy. With the shunt per node, one rule holds wherever the effect
sits, and a live console can lift a session bypass per node too. This choice keeps the slice inside
its paths. Keeping today's prepared bypass per node would need a post-banking pass that rewrites
entries in `compile.rs`, `ids.rs` or the graph runtime.

The exception is `effect_compiler::NEVER_BANKED_EFFECTS = ["miso.delay"]`. The delay's factory
declines every bank, so it gains nothing from the lowering. It is also the one launch effect whose
bits would move. Its D7 check reads its feedback ring and damping state as well as its output. A 1
ms, 0.95-feedback delay fed `9e29` trips that check. Its prepared bypass then zeroes the dry block,
and a shunt would copy the dry block over the zeros. So the delay keeps its prepared bypass.
`the_never_banked_list_is_exactly_the_launch_effects_that_decline_a_bank` holds the list to the
factories that decline a well-formed bank at the host width. `the_delay_keeps_its_prepared_bypass_
because_a_shunt_would_move_a_bit` pins the reason and goes red if the delay's check stops reading
state.

### Gates

| Gate | Evidence | Result |
|---|---|---|
| 1. A mixed-bypass cohort binds one bank | `graph-compiler/tests/bypass_cohorts.rs::a_mixed_bypass_cohort_binds_one_bank_per_slot`: one cohort of the host's width, standing strip `eq -> comp` (SIMD 1) and limiter (SIMD 2), 29 masks: none, all, each edge lane alone, alternating, one full-and-empty mix, 24 random draws, with each slot masked independently. One bank per slot holding every track, one full group. The `lib.rs` #1027 cohort test is inverted to `a_prepare_time_bypassed_slot_keeps_its_chain_in_the_cohort` (four slots bind, not two). | Simd8 green on x86-64. Simd4: the tests are width-agnostic and sit in the product crates, so `aarch64-debug` runs them. No AArch64 toolchain or qemu here; see "x86 resolution" below. |
| 2. Bit-identical to today's per-node prepared bypass | `graph-compiler/tests/bypass_shunt_identity.rs`. **Banked leg:** every bankable launch effect x every 48 kHz quality row x every supported link mode x 4 bypass patterns (mixed with both edge lanes, its complement, one lane, all). One bank of the host width, bound `bypass = false`, driven through `ConsoleEffectBankStage` with channel-less lanes. Random per-lane parameters, 16 blocks of 128. Input segments: random signal to about +32 dBFS, `-0.0` runs, subnormals, quiet (-60 dB) and exact silence (silent fast paths), staggered per lane. Every lane equals its own scalar instance prepared with the lane's bypass. That covers the limiter's 486-sample line (4 blocks of zeros first) and the soft clip's 31. Each effect must also show its bypass is audible, or the test fails as vacuous. **Per-node leg:** every lowered effect, 4 parameter draws per case, the graph's per-node order re-enacted with `BypassShunt`, input up to `9.9e29`, where the wet path's own D7 fires. **Live toggles:** limiter, EQ and soft clip. A session-bypassed banked lane with a live channel is un-bypassed at block 5 and re-bypassed at block 10. It equals the per-node live path of a session-enabled instance, and a channel-less bypassed neighbour and the enabled lanes stay on their oracles. **Through the graph:** `bypass_cohorts::a_session_bypass_renders_todays_bits` rebuilds today's lowering exactly: lowered entries are re-prepared `bypass = true` and lose their lane, so the planner splits the cohort again. It compares every track's `PostSimd1` and `PostSimd2PreFader` words and the output, lowered at the host width and at `Backend::Scalar` against today. NaNs fold to one value everywhere. | green |
| 2. `fma(0, wet, dry)` planted | The restore loop in `ConsoleEffectBankStage::process_inner` replaced by `0.0.mul_add(wet, dry)`. | red: banked leg (`parametric-eq ... lane 0 (bypassed) block 0 left: sample 7 is 0x00000000, today's prepared bypass renders 0x80000000`) and live-toggle test. Reverted. |
| 3. Live bypass still works and preserves latency | The existing live-bypass tests, unchanged: `rack/tests/console_bank.rs` (10), `graph` `ConsoleEffect` tests, `effect-contract/tests/live_control.rs`, `console-workload/tests/paired_spans.rs`. The new live-toggle test above covers real effects. `effect-compiler/tests/native_session.rs::a_live_console_lane_starts_from_the_session_bypass` pins the seeding. | green |
| 4. Console digests unchanged | Scratch probe, not committed: 64 blocks of each standing console fixture (intended and mono), 8 bypass variants each, unarmed and with the mono collapse armed. Built at `6fdf5db2` in a detached worktree and at this branch. **All 32 digests are identical**, the no-bypass rows included (`intended none 2e5a2d3a…`, `mono none d02ad91d…`). Effect banks per plan: 24 on the branch for every variant, against 22, 23 or 19 at base for the mixed variants (for example `eq-every-third`, `limiter-first-20`, `mixed-hash`). The pinned console digests in `console-workload`'s release tests also pass. | identical |
| 5. Policies, audits, callgraph | `check-realtime-policy` (54 regions), `check-rack-policy`, `check-effect-contract.sh` with the release bench, plus workspace, graph, effect-runtime, host-core, lane, session, builtins, protocol-control and bench policies. The Python gates: `check-test-support-ci`, `check-script-reachability`, `check-ci-path-routing`, `check-release-shape --self-test`, `check-scalar-oracle-absent --self-test`, `check-session-map-shape` and both vocabularies, all with `python3 -B`. Also `check-env-vocabulary`, `check-unfused-seal`, `check-conformance-boundaries`, `check-parametric-eq-render-contract`, the audit-leak and evidence-leak gates, and `test-realtime-policy`/`test-rack-policy`. Release audits: `capi`, `delay`, `compressor`, `parametric-eq`, `gate-expander` (bank bound). Also the builtins, builtins-graph and graph traces, the protocol allocation audit, the realtime probes and 1,000,000-block trace, and the effect-contract 1,000,000-call trace. Every audit reports 0 allocations, 0 syscalls and 0 violations. `check-builtins-fixtures`, and `check-console-fixtures` with the release validator. `check-web-audioworklet.sh` on a built artifact set (module `0f5c0ee7…`) includes the render closure (the callgraph gate). Also the V8 spill gate and its self-test, `check-scalar-oracle-absent --wasm`, `check-browser-expected-resources --artifacts` (native parity digests agree) and `test-web-audioworklet.sh`. | green |
| 5. Render allocates nothing | Scratch probe, not committed. It links `bench_support::alloc` from a temporary `console-workload` test (the only crates with the audited allocator are under `tools/`). 16 warm-up blocks, then 2,000 measured blocks, per-thread counters. Plans: 64 intended with mixed bypass (74 lanes, 24 banks); the app shape with 56 unselected tracks (112 lanes); 64 mono mixed; 13 tracks with the remainder per node; 3 tracks all per node (9 lanes, no banks); 64 unbypassed. | 0 allocations, 0 deallocations and 0 reallocations in every plan |

Also: `cargo check` and `cargo clippy --locked --workspace --all-targets --all-features -- -D
warnings` and `cargo fmt --all --check` are clean. CI's debug-a set (workspace minus DSP, audit,
bench and wasm tools, all test-support features) runs 91 binaries: 1,097 passed, 0 failed, 10
ignored. Release runs `effect-contract`, `effect-compiler`, `rack`, `graph-compiler`, `audit`,
`bench` and `console-workload`: 394 passed, 0 failed. The DSP crates' debug set (test-debug-b)
was not run. Nothing under them changed, and none of them uses the changed APIs.

**x86 resolution of the AArch64 legs.** Debug leg: the 25 product crates plus `dsp-reference`,
`conformance` and `target-smoke`, with the leg's features, pass `--no-run` and `-- --list`
(1,727 tests, including the new ones), and `aarch64-known-defects.py judge-skips debug` accepts the
listing. Release leg: `lane`/`math` with `math/lane` pass `--no-run` and `--list`, `judge-skips
release` accepts the two #1019 rows, and `console-workload` release builds. The leg's
no-silent-skip scan finds no match.

### Red mutations run (each reverted)

- Restore as `fma(0, wet, dry)`: red in the banked and live-toggle legs. The session-level test
  stays green, because the input section's filters normalise a source `-0.0`. Its doc says so.
- Shunt only for a live channel (drop `|| lane.bypassed()`): red in the banked leg, both
  `bypass_cohorts` render tests, and `console_bank::a_channel_less_bypassed_lane_...`.
- `BypassShunt::capture` stops feeding its line: red in all three identity tests (limiter first).
- No lowering (`bypass: effect.bypass`): red in `a_mixed_bypass_cohort_binds_one_bank_per_slot`
  (`masks [1, 80, 1]: one bank per slot`) and `a_session_bypass_renders_todays_bits`.
- The drain ignores `Bypass` records: red in the live-toggle test at block 5.
- The input stage's bound raised to infinity (`NONFINITE_LIMIT`): red in
  `non_finite_sources_render_todays_bits`.
- The graph's per-node witness conjunction dropped: red in host-core's adapted
  `the_scalar_console_effect_arm_maintains_its_own_live_terms`.
- The rack's window sized by the automation capacity whatever the lanes: the rack test and
  `compile_shapes` abort on allocation. The workspace run found this before the fix, below.

### Found by the workspace run, fixed

`compile_shapes::mixed_rack_depths_...` bypasses a banked slot under
`maximum_automation_spans_per_block: u32::MAX`. The new bank stage allocated its staging window of
`automation_capacity` spans per lane and aborted (687 GB). A channel-less lane never stages a span,
so a slot with no live channel now holds no window (`baa03f09`).

`host-core/tests/symmetry_witness.rs::the_scalar_console_effect_arm_maintains_its_own_live_terms`
reached its bank-free arm through a prepare-time bypass that split the EQ cohort. That split is
what #1087 removes, so its eight EQs banked. `edited_apart` now moves the bypassed tracks' EQ into
the dynamic rack, and the arm is per node again. The assertions are unchanged. **Path deviation,
for Sol:** this host-core test file and `docs/EFFECT_CONTRACT_V1.md` are outside the authorized
paths.

### Cost (admitted, not timed; S0 and S4 measure it)

- **A bypassed lane runs its wet path.** The EQ's prepared bypass skipped it entirely. The
  compressor, gate, limiter, soft clip and transient shaper already computed the wet word under a
  prepared bypass and selected the dry one. The multiband ran a bypass-specialised block.
- **A bank slot with any bypassed lane** runs `ConsoleEffectBankStage` in place of
  `EffectBankStage`. Per block that adds: the lane drain (one check per lane), the dry capture
  (`2 x quantum x W` words), the latency line (the limiter's 486 samples per lane, the soft clip's
  31), and the strided restore of each bypassed lane. Retained per slot at 128 frames and W = 8:
  8 KiB of dry block, plus 31 KiB of limiter line, and no staging window.
- **Per node**, a lowered bypassed instance renders as the graph's `ConsoleEffect`: the wet path,
  two block copies, and a staging window of `automation_capacity` spans (5 KiB at the C ABI's 128).
- **Mono collapse.** A bypassed lane clears `UNBYPASSED` from its first block, so a cohort holding
  one declines the collapse. Before, an all-bypassed cohort could collapse. Against that, tracks
  that used to fall to per-node remainders now bank.

### Residuals and follow-ups

- **Lane coupling at extreme levels (Sol's M2).** A bypassed lane's own output is identical over the
  whole input range an effect can receive. Its wet path still runs, though, and five effects'
  whole-bank D7 recovery zeroes and resets every lane when one lane's wet block reaches `1e30`:
  EQ, compressor, multiband, soft clip and transient shaper. A probe fed bypassed lanes up to
  `9.9e29`, about +600 dBFS, on every fourth block. The bypassed lanes showed 0 mismatches. The
  enabled bank-mates diverged: EQ 85,666 words, compressor 185,779, multiband 302,328, soft clip
  22,690 and transient shaper 262,674; the gate and limiter 0. Enabled lanes already couple this
  way with each other. P1 lets a bypassed lane trigger it as well. P2b, P2c and P2e make D7
  recovery per lane, and #1069 covers the multiband.
- **Input domain.** A shunt copies a non-finite or `>= 1e30` dry block where an effect's prepared
  bypass would zero it (probe: 0 mismatches for the EQ, whose bypass has no D7, and mismatches for
  the rest). No effect receives such a sample in a compiled plan. The input stage sanitises it, and
  every later stage zeroes a block that leaves the range. `non_finite_sources_render_todays_bits`
  gates this with NaN, both infinities, `1e30` and `-3e38` in the sources.
- **The graph's per-node `ConsoleEffect`** sizes its staging window by the automation capacity even
  for a channel-less lane. That wastes memory, and a caller that sets the capacity to `u32::MAX`
  and renders a per-node session bypass would abort on allocation. The rack twin is fixed. The
  graph one is four lines in `crates/graph/src/runtime.rs`, outside this slice; recommended as a
  follow-up.
- `effect-contract/src/symmetry.rs`'s "static-bypass convention" paragraph still says `UNBYPASSED`
  is seeded from the prepared bypass and that a console-free plan builds no lane. Both are now
  stale for effects that bank. The file is outside this slice's paths, and S1r rewrites that
  vocabulary.
- The allocation and digest probes are PR evidence, not committed tests. B0's app-shape row will
  exercise this path under the audited allocator.

## Sol verdict, attempt 1

Sol, 2026-09-29. Verified `b0c6f11f` (`1482a0c9`..`b0c6f11f` on `6fdf5db2`) against this body,
decision 12 and the design verification (H1, M2, amendments 1 and 7).

**PASS.** Every objective gate holds on independent evidence. The class-A proof holds against the
real base code, not only against a reconstruction of it, and three mutations of Sol's own go red.
Two medium findings fall outside this slice's paths. Root must give each an owner before batch C2
is pushed (conditions at the end). They are root actions, not a second attempt.

### Evidence

Sol wrote a scratch probe (not committed) and compiled it unchanged at `6fdf5db2` and at the
branch, so "today" below is the base engine itself.

**Gate 1, the goal.** 8 tracks of EQ -> compressor at Simd8, in SIMD rack 1 and, for the app
shape, in the dynamic rack. Both racks give the same plan:

| Bypass masks (EQ / comp) | Base | Branch |
|---|---|---|
| none | eq[8], comp[8] | eq[8], comp[8] |
| `11010110` / `11010110` (app subset) | no bank, 16 nodes per node | eq[8], comp[8] |
| `01001010` / `10010010` | no bank, 16 nodes per node | eq[8], comp[8] |
| `00000001` / `00000001` | no bank, 16 nodes per node | eq[8], comp[8] |
| all | eq[8], comp[8] | eq[8], comp[8] |

- **The 64-track consoles.** The standing intended and mono consoles bind 24 banks under every
  bypass variant. Base binds 21-23 for the mixed variants.
- **W = 4.** On this host `Backend::Simd4` binds banks for the limiter and the multiband only.
  With mask `10100101`, base binds {1,3,4,6} and {0,2,5,7}, and the branch binds {0-3} and {4-7}.
  NEON itself is unverified, because there is no AArch64 toolchain or qemu here. CI's
  `aarch64-debug` leg runs gate 1 when C2 is pushed.
- **The bank key.** Every session bypass on an effect that can bank reaches the planner as
  prepared `bypass = false`, so keeping the field in `EffectProgramKey` does not defeat the
  purpose; the table is the proof. The field only keeps a direct caller's mixed prepared-bypass
  cohort declined. That is right while every kernel reads one flag per bank.

**Gate 2, class A.** Every word at `PostInputBuiltins`, `PostSimd1`, `PostDynamic`,
`PostSimd2PreFader` and the output, base against branch, with NaN folded:

- **Effects.** 8 effects x their link modes x 3 feeds x 5 masks. The feeds are random, runs of
  `-0.0` and subnormals through identity builtins, and music. The branch rendered each at Simd8,
  Simd4 and Scalar: 792 renders, 0 differ. Only `normal` quality is declared at 48 kHz. The
  limiter's bypassed lanes are `+0.0` for exactly 486 samples, then the input from sample 0.
- **`-0.0` through a session.** The evidence says no session carries `-0.0` to a slot. One does:
  an enabled compressor at -12 dB makeup turns the smallest negative subnormals into `-0.0` ahead
  of a bypassed slot, which puts 1,392 to 16,315 `-0.0` words on the bypassed tracks per render.
  24 renders x 3 widths: 0 differ.
- **Other inputs, all identical:**
  - non-finite sources, 16 renders: NaN with two payloads, both infinities, `1e30`, `-3e38` and
    `+-9.99e29`;
  - `+0.0` gaps around a latent limiter, whole-session and odd tracks only, 24 renders;
  - keyed compressors (sidechain from the next track's `PostSimd1`) before the limiter, 3 renders;
  - both 64-track consoles, 6 bypass variants each.
- **Live toggles.** Branch: lanes 0, 3 and 7 session-bypassed on all three strip slots, lifted at
  block 5 and re-bypassed at 10. Base: the same lanes session-enabled, live-bypassed before
  block 0, then toggled at the same blocks. The two are identical. A plain live toggle on enabled
  lanes is identical on both sides too.
- **Sol's mutations**, each reverted:
  - The rack shunt's line one frame short (`latency - 1`): red in `bypass_cohorts` (2 tests),
    `bypass_shunt_identity` (2) and 52 probe renders.
  - The restore loop skips the last lane: red in `a_session_bypass_renders_todays_bits`, the
    banked identity leg and 106 renders. `console_bank` stays green, because its bypassed lane is
    lane 0.
  - Whole-bank shunt selection, restoring every lane when any is bypassed: red in 6 committed
    tests and 102 renders.
  - The named `fma(0, wet, dry)` restore is also red at session level, in 9 of the 24 `-0.0`
    renders (EQ, multiband, soft clip). `bypass_cohorts` cannot see it.

**Gate 5, render.** A counting global allocator watched 15 plans: 1, 3, 8, 13 and 16 tracks with
mixed bypass on all three strip slots, at Simd8, Simd4 and Scalar. After 16 warm-up blocks, 2,000
blocks made 0 allocator calls. The policies pass, and so do the audits:
- capi, delay, compressor, EQ and gate, 100,000 blocks each;
- the builtins, graph, realtime and effect-contract traces;
- the callgraph gate and the V8 spill gate. The module is `0f5c0ee7...`, as reported.

**The other gates**, all exit 0:
- cargo check, clippy `-D warnings`, fmt and rustdoc.
- Debug: CI's debug-a set, 1,097 passed and 10 ignored. The debug-b DSP set, which the
  implementer did not run: 790 passed, 28 ignored.
- Release: the affected crates with audit, bench and console-workload, 754 passed.
- Every lint-job script, the audit-native job, artifact and artifact-gates (including the SDK's
  generated, types, headless and package checks), wasm-guests, cross-target, release-shape with
  `panic=unwind`, and the docs and gate self-tests.
- The AArch64 legs resolved on x86. Debug `--no-run` and `--list` find 1,727 tests. Release builds
  too. `judge-skips` accepts both listings, and the silent-skip scan is clean.

**Merges.**
- Onto `c4906414`: clean.
- B0, then P1: clean. The workspace check, the console-workload and bench release tests and
  `test-console-benchmark.sh` pass.
- S1a on top: two textual conflicts, both additive (`NEVER_BANKED_EFFECTS` beside
  `CONSOLE_ELIGIBLE_EFFECTS` in `prepare.rs`, and new tests in `native_session.rs`). Three P1 test
  changes then fail to compile against S1a's model, because they name `simd1`, `dynamic` and
  `simd2`: `bypass_cohorts.rs`, the new `native_session.rs` test and the `symmetry_witness.rs`
  edit. `bypass_shunt_identity.rs` and the inverted `graph-compiler` unit test merge and compile.
  C2 lands before C3, so S1a's rebase ports these. `console-workload` also breaks under S1a, but
  that is B0 x S1a, not P1.

### Findings, ranked

**M1. A per-node session bypass now holds a live-console staging window. The memory is not
charged, and it can abort bind.**
- Before #1087 a console-free plan held no `ConsoleEffect`. Now every session-bypassed instance
  that renders per node is one, and `graph::runtime::ConsoleEffect::new` sizes its staging window
  by `automation_capacity` whatever the lane. `baa03f09` fixed the rack twin; the graph twin is
  unchanged.
- One track with a bypassed EQ and `maximum_automation_spans_per_block: u32::MAX` binds at base.
  At the branch it aborts: `memory allocation of 171798691800 bytes failed`, which is `u32::MAX`
  x 40 B.
- At 128 spans, measured:

  | Plan | Retained bytes added | Estimate added |
  |---|---|---|
  | One per-node bypassed EQ | 6,186 B | 72 B |
  | One 8-lane bypassed EQ slot | 7,808 B | 576 B |

  `effect_control_resource` charges neither the staging window nor any `BypassShunt` (its dry
  blocks, plus a limiter slot's 31 KiB line at W = 8). Live consoles already had this gap. P1
  extends it to console-free hosts such as the C ABI.
- No test covers the graph twin. It needs a bounded successor in C2 that:
  - gives a channel-less per-node lane no window, as `baa03f09` did for the rack;
  - charges shunts and windows in the estimate;
  - tests a per-node bypass at `u32::MAX` spans.

**M2. The D7 residual is accepted for four effects and has no owner for the multiband.**
- **The coupling, on the base engine.** One bypassed lane of an 8-lane bank is fed a legal,
  constant-magnitude signal. It silences all seven enabled bank-mates for as long as it stays hot:
  43,008 of 43,008 words at their rack taps. Base moves none. The bypassed lane's own bits never
  move.
- **Trigger levels at the bank input:**
  - compressor at +24 dB makeup: 1e29;
  - EQ with one band at +24 dB: 6e29;
  - multiband at its defaults: 6e29;
  - gate and limiter: never, because their gain is at most 1 or capped by the ceiling.
- **Reachable?** Yes with legal input: after the input stage, any finite sample below `1e30` is
  legal. Not with real audio. From 0 dBFS it needs about +500 dB of stacked legal gain before the
  slot, when trim, each EQ band and makeup each give at most +24 dB.
- **Why not a FAIL.** This is a new instance of the coupling decision 12 forbids, reached through
  the whole-bank D7 exception that already existed. P1 cannot mask D7 per lane without changing
  kernels, which is a non-goal. P2b (#1089), P2c (#1090) and P2e (#1092) make recovery per lane,
  and their gate 4 closes this, provided a bypassed lane counts as an active one. It is an accepted
  residual with those owners.
- **The multiband has no owner.** The evidence record says #1069 covers it, but #1069 is the
  ramp-cut defect and does not mention D7.

**L1.** P1 x S1a needs the test ports listed under Merges.

**L2.** No committed test or audit renders a session bypass under the audited allocator or strace.
The implementer's allocation probe and Sol's are both scratch. B0's app-shape row covers it once C2
lands. Name that, or add an audit subject.

**L3.** The `bypass_cohorts` doc is true for sources but not for sessions: `-0.0` reaches a
bypassed slot from an enabled upstream stage (see the evidence). A session-level `-0.0` case would
let this gate see the `fma` restore. Optional.

**L4.** A live console can now lift the session bypass of every effect except the delay, which is
seeded bypassed and keeps its prepared bypass. Record this for S1c.

**L5.** The two path deviations are accepted.
- `host-core/tests/symmetry_witness.rs` keeps its assertions and reaches the per-node arm through
  `edited_apart`.
- The `EFFECT_CONTRACT_V1.md` sentence was false both before and after this change.

**L6.** What allocated the 687 GB was `ConsoleEffectBankStage::new`'s `packed` window: `u32::MAX` x
4 lanes x 40 B, in `compile_shapes`' Simd4 leg. It comes after a 172 GB one-lane `staging` window.
Reproduced under a 16 GB address-space cap, the first failure is 171,798,691,800 B at
`rack/src/lib.rs:1026`. The rack fix is complete. If it regresses,
`console_bank::a_channel_less_bypassed_lane_is_shunted_without_a_staging_window` and
`compile_shapes` abort.

### Conditions on pushing C2 (root)

1. M1: open the successor above and land it in C2.
2. M2: give the multiband an owner. Either amend #1069 or open an issue, or add
   `miso.multiband-compressor` to the effects that keep a prepared bypass. It is not
   console-eligible until #1069 closes anyway.
3. Add a bypassed tripping lane to gate 4 of P2b, P2c and P2e.
4. Do not push P1 unless P2b, P2c and P2e have closed. Otherwise withhold the lowering for each
   effect whose P2 slice did not close.
