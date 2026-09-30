# Bound and charge the bypass shunts, and keep the multiband's prepared bypass

Slice P1b of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`). It closes the conditions in Sol's P1
verdict (`.github/ISSUE_SPECS/1087-console-p1-keep-a-bypassed-lane-in-its-effect-bank.md`, "Sol
verdict, attempt 1", M1, M2, L2 and L3, commit `e0e65ccb` on `codex/1087-per-lane-bypass`). Batch
C2 is not pushed until this slice closes.

## Problem

P1 (#1087) lowers a session `bypass` to prepared `bypass = false` plus a `BypassShunt`. That left
four residuals:

1. **An unbounded staging window (M1).** Every session-bypassed instance that renders per node is
   now a `graph::runtime::ConsoleEffect` (`crates/graph/src/runtime.rs:864-879`). Its constructor
   sizes a staging window from `automation_capacity`, whether or not the lane has a live control
   channel.
   - One track with a bypassed EQ and `maximum_automation_spans_per_block: u32::MAX` bound at base.
     At P1 it aborts bind with `memory allocation of 171798691800 bytes failed`
     (`u32::MAX` x 40 B).
   - `baa03f09` fixed the rack twin (`ConsoleEffectBankStage`). The graph twin is unchanged, and no
     test covers it.
2. **Uncharged memory (M1).** `effect_control_resource` (`crates/graph-compiler/src/estimate.rs:151`)
   charges neither the staging windows nor any `BypassShunt`: its dry blocks, and a latent slot's
   delay line (31 KiB for a limiter slot at W = 8). At 128 spans, Sol measured:

   | Plan | Retained bytes added | Estimate added |
   |---|---|---|
   | One per-node bypassed EQ | 6,186 B | 72 B |
   | One 8-lane bypassed EQ slot | 7,808 B | 576 B |

   Live consoles already had this gap. P1 extends it to console-free hosts such as the C ABI.
3. **Multiband D7 coupling has no owner (M2).** A bypassed lane now shares a bank with enabled
   lanes. One bypassed multiband lane fed about `6e29` trips the bank's whole-bank D7 recovery and
   silences its enabled bank-mates. This is legal input, but far beyond real audio. P2b, P2c and
   P2e make D7 per lane for the EQ, compressor, gate, transient shaper and soft-clip. Nothing does
   so for the multiband: #1069 is its ramp-cut defect and does not mention D7.
4. **No committed allocation witness (L2, L3).** The zero-allocation evidence for a session bypass
   is a scratch probe. `bypass_cohorts` never sees a `-0.0` reach a bypassed slot from an enabled
   upstream stage, so an arithmetic `fma(0, wet, dry)` restore passes it.

## Smallest closable slice

1. **Bound the window.** A channel-less per-node lane (no live control channel) holds no staging
   window, as `baa03f09` did for the rack. A lane with a channel keeps today's window.
2. **Charge the memory.** `effect_control_resource`, or its caller, charges every staging window
   and every `BypassShunt` that bind will allocate, in both the per-node and the banked form. The
   estimate must not undercount the retained bytes it describes.
3. **Keep the multiband's prepared bypass.** A session `bypass` on `miso.multiband-compressor`
   keeps today's prepared bypass and is not lowered to a shunt. Its mixed-bypass cohorts therefore
   decline a bank, exactly as before P1. Name the list separately from `NEVER_BANKED_EFFECTS`,
   because the multiband still banks uniform cohorts. The slice that makes the multiband
   console-eligible, after #1069, owns its per-lane D7 and lifts this exclusion.
4. **Witnesses.**
   - A committed test renders mixed session bypass, both banked and per node, at Simd8, Simd4 and
     Scalar under `bench_support::alloc`'s counters, and asserts zero allocator calls after warm-up.
   - `bypass_cohorts` gains a session-level case in which an enabled upstream stage puts `-0.0`
     on a bypassed slot's input (for example, a compressor with negative makeup ahead of the slot).

Authorized paths:
- `crates/graph/src/runtime.rs` (`ConsoleEffect` construction only);
- `crates/graph-compiler/src/estimate.rs` and its caller in `crates/effect-compiler/src/prepare.rs`;
- `crates/effect-compiler/src/prepare.rs` (the prepared-bypass list);
- `crates/effect-contract/src/live.rs`, only for a `BypassShunt` size helper;
- their tests, `bypass_cohorts`, and this spec.

## Dependencies

- P1 (#1087). Branch from P1's head and merge right after P1, before P2a.

## Objective gates

1. One track with a session-bypassed EQ and `maximum_automation_spans_per_block: u32::MAX`
   binds, and so does an 8-lane bank with one bypassed lane. A test for each. Reverting step 1
   aborts the first.
2. A test asserts that a session-bypassed plan's estimate is at least its measured retained bytes,
   for one per-node bypassed EQ and one 8-lane bypassed limiter slot at 128 spans. It fails if the
   shunt charge is removed.
3. An 8-track multiband cohort with a mixed bypass mask renders exactly as it does at P1's base
   (`6fdf5db2`): the same plan shape (no bank) and the same bits. One bypassed lane fed `6e29` leaves
   its enabled neighbours' bits unchanged.
4. The allocation test and the `-0.0` case pass. A planted `fma(0, wet, dry)` restore turns the
   `-0.0` case red.
5. `scripts/check-realtime-policy.sh`, `scripts/check-effect-contract.sh`,
   `scripts/check-rack-policy.sh`, the realtime audits and the callgraph gates pass. Every other
   shipped plan's digests are unchanged.

## Non-goals

- No per-lane D7 for the multiband.
- No change to the estimate of anything but staging windows and shunts.
- #892 (dry-line copy and swap) stays its own performance issue.

## Standing rules for the implementer

- Work only from this body, the umbrella issue, P1's spec and verdict, and decision 12. Read the
  cited code first.
- Class A: every gate that says "bit-identical", "the same bits" or "unchanged" is a hard stop,
  not a tolerance.
- Render stays allocation-, lock- and syscall-free. Only `crates/lane` names `wide` or intrinsics.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings` and the focused tests before every checkpoint.
- Every new test names the plausible defect that turns it red (AGENTS.md "Test value").

## Attempt 1 evidence

Terra, 2026-09-30. Branch `codex/1100-bypass-shunt-bounds` from `b0f37aa7` (batch
`codex/batch-console-2`, P1 merged). Commits `3bdba498`, `87f56440`, `112e69c8` and this record.
Not pushed.

### What landed

- **The window is bounded** (`graph/src/runtime.rs`, `ConsoleEffect::new`). A lane without a
  channel sizes its staging window to zero spans and skips the window check, as `baa03f09` did for
  `ConsoleEffectBankStage`. A live channel keeps its `automation_capacity` window. The drain of a
  channel-less lane stages nothing, so render reads `spans[..0]`.
- **The owners are charged** (`graph-compiler/src/estimate.rs`, `effect_control_resource`).
  - Per node, for every controlled entry that is not a bank member: the boxed `ConsoleEffect`, the
    staging window (live channel only), and the shunt at the render quantum and the effect's
    latency.
  - Banked, for every bank slot with a lane: the growth of `rack::ConsoleEffectBankStage` over the
    `rack::EffectBankStage` it replaces; the staging and packed windows (any live channel); and
    the shunt over the AoSoA block, `quantum * lanes` and `latency * lanes` (any live or bypassed
    lane).
  - The charges ride the existing scalar-owner fold, as the control lanes always have:
    `graph_metadata_bytes`, `incremental_plan_bytes`, `session_plus_plan_bytes` and
    `largest_allocation_bytes`. The semantic (canonical) estimate is untouched, so no plan hash
    moves.
  - `BypassShunt::allocated_bytes` and `BypassShunt::largest_allocation_bytes`
    (`effect-contract/src/live.rs`) are the size helpers. `ConsoleEffect` is private to `graph`,
    so its box is charged as the sum of its five field types, the identity `graph`'s
    `observation_size_accounting` test pins; gate 2 below sees it if that ever drifts.
- **The multiband keeps its prepared bypass** (`effect-compiler/src/prepare.rs`).
  `PREPARED_BYPASS_EFFECTS = ["miso.multiband-compressor"]`, separate from
  `NEVER_BANKED_EFFECTS`, and `lowers_session_bypass(id)` is the one predicate the lowering reads.
  A bypassed multiband is prepared `bypass = true` with no lane, exactly as before #1087. A live
  console still seeds its lane from `initial_bypass`, as at base.
- **The allocator counts released bytes** (`tools/bench-support/src/alloc.rs`).
  `Counters::released_bytes` adds `dealloc`'s size and a `realloc`'s old size, so
  `requested_bytes - released_bytes` over a window is what the window left live.

### Gates

| Gate | Evidence | Result |
|---|---|---|
| 1. `u32::MAX` spans binds, per node and banked | `graph-compiler/tests/bypass_resources.rs::a_console_free_bypass_binds_at_any_automation_capacity`: one track with a bypassed EQ (per node, no bank), and one host-width EQ bank with its last lane bypassed, both prepared at `maximum_automation_spans_per_block: u32::MAX`, bound and rendered for 4 blocks. Each asserts that the plan retains, and the estimate charges, less than 1 GiB. | green |
| 1. Revert step 1 | `ConsoleEffect::new` sizes the window whatever the lane. | red. This 503 GB host does not abort; it allocates the 172 GB window (140 s) and the retained-bytes assertion fails. That is why the test asserts retained bytes, not only that bind returns. |
| 1. Revert the rack twin | `ConsoleEffectBankStage::new` sizes its windows whatever the lanes. | red: `memory allocation of 1374389534400 bytes failed`, SIGABRT (the packed window, 8 lanes). |
| 2. Estimate >= measured retained bytes | `bypass_resources::a_bypass_or_a_console_is_charged_at_least_what_it_retains`. Each case is a pair of plans that differ only in the bypass or the console. It compares the difference in `incremental_plan_bytes` with the difference in bytes the audited allocator saw prepare -> compile -> bind leave live (producers dropped), at 128 spans. | green, and exact: |
| | one per-node bypassed EQ (P1: 6,186 retained against 72 charged) | retained 1,400 B, charged 1,400 B (72 lane + 304 `ConsoleEffect` + 1,024 dry) |
| | one bypassed lane of an 8-lane limiter bank | retained 40,048 B, charged 40,048 B (576 lane array + 176 stage growth + 39,296 shunt with the 486-sample line on every lane) |
| | a per-node live console (3 effects, 1 track) | retained 26,176 B, charged 26,176 B |
| | an 8-lane live console (3 banked slots) | retained 218,000 B, charged 218,000 B |
| 2. Mutations | Each removed from `effect_control_resource` in turn: per-node shunt; banked shunt; per-node window; banked windows; `ConsoleEffect` box; stage growth. | all red: 1,400 vs 376; 40,048 vs 752; 26,176 vs 10,816; 218,000 vs 79,760; 1,400 vs 1,096; 40,048 vs 39,872. `graph-compiler`'s #964 unit test `effect_control_resource_uses_independent_queue_and_owner_arithmetic` now carries the same terms by its own arithmetic and goes red too. |
| 3. Multiband: same plan shape and bits as `6fdf5db2` | Scratch probe (not committed), compiled unchanged at `6fdf5db2`, at P1 (`b0f37aa7`) and at this branch. 8 multiband tracks at defaults, identity inputs, masks `00000000`, `11111111`, `01101001`, `00000001`, `11111110` and `10101010`, at Simd8, Simd4 and Scalar. Each render records the bound banks and a digest of every word at each track's `PostInputBuiltins`, `PostSimd1` and `PostSimd2PreFader`. For each mask with a bypassed lane, the first bypassed lane is also fed a constant-magnitude `6e29`, and the enabled tracks' words are compared. | Branch = base: all 33 lines identical (18 renders, 15 hot runs). At Simd8 no mixed mask banks, and Simd4 binds only uniform banks. No hot run moves a neighbour. P1 differs exactly where expected: the four mixed masks bank the whole cohort at Simd8 and Simd4, and all 8 of those hot runs move the neighbours. Music digests are equal at all three commits. |
| 3. Committed | `bypass_cohorts::a_mixed_bypass_multiband_cohort_keeps_its_prepared_bypass`: prepared `bypass = true` and no lane on every bypassed instance. No bound bank mixes bypassed and enabled lanes, and at 8 lanes nothing binds. #1087's lowering, rebuilt in the test, banks the cohort; at music levels both render the same words, and so does Scalar. `bypass_cohorts::a_hot_bypassed_multiband_lane_leaves_its_neighbours_bits_unchanged`: lane 3 bypassed and fed `+-6e29` (sign flips every 8 samples). The enabled tracks' words at both taps are unchanged. The positive control, #1087's lowering, moves them. `effect-compiler/tests/native_session.rs` pins the list, the predicate over all 8 launch effects, and a live lane seeded bypassed on the multiband. | green. Mutation: drop the multiband from the lowering predicate. Both tests go red: `metadata.bypass` false, and 14 neighbour taps moved. |
| 3. Why the feed flips sign | A constant `6e29` does not trip the multiband's D7: the crossover passes DC without overshoot. A constant-magnitude square wave does. The positive control is in the committed test, so the feed cannot silently go vacuous. | |
| 4. Allocation-free render | `bypass_resources::a_mixed_session_bypass_renders_without_allocating`: 13 tracks, a different bypass mask on each strip slot. It runs at Simd8, Simd4 and Scalar, with 16 warm-up blocks, then 256 blocks under this thread's `bench_support::alloc` counters in `Count` mode. Every leg has bypassed instances per node. Every leg whose width fits the host has bypassed lanes in banks (on x86-64: Simd8 all three slots, Simd4 the limiter). A four-lane host binds nothing at Simd8, which D4 decides, and the test asserts that too. | green, (0, 0) allocations and frees on every leg. Mutation: `black_box(Vec::<u8>::with_capacity(1))` in `BypassShunt::capture`. Red, 2,560 allocations. |
| 4. `-0.0` from an enabled stage | `bypass_cohorts::a_negative_zero_from_an_enabled_stage_passes_a_bypassed_slot_unchanged`. Identity inputs, SIMD rack 1 reordered to `comp -> eq`, the compressor enabled everywhere at -12 dB makeup, the EQ bypassed on alternate tracks. The source alternates music with 32-sample runs of `-2^-149`. Lowered (banked) and Scalar (per node) must equal today's prepared bypass, and the bypassed EQ tracks must carry `-0.0` out of the slot (asserted). | green |
| 4. `fma(0, wet, dry)` planted | In the rack's restore loop, then in `BypassShunt::apply`. | red, each on its own leg: `-0.0 lowered ("ch00", "PostSimd1"): word 160 is 0x00000000, today's render has 0x80000000`, and the same word on `-0.0 scalar`. Every other `bypass_cohorts` test stays green, which is L3. |
| 5. Policies | `check-realtime-policy.sh` (54 regions, 15 files), `check-effect-contract.sh`, `check-rack-policy.sh`, `check-bench-policy.sh`, `check-workspace-policy.sh`, `check-graph-policy.sh`, `check-host-core-policy.sh`, `check-realtime-audit-leak.sh`, `check-conformance-boundaries.sh`, `check-lane-policy.sh`, `check-effect-runtime-policy.sh` | all exit 0 |
| 5. Audits and callgraph | The release audit job, step for step: `audit capi` (100,000 calls; 0 allocations, frees and violations); `audit delay`, `compressor` and `parametric-eq` (100,000 blocks each); `gate-expander`; the builtins, builtins-graph, graph, realtime (1,000,000) and effect-contract (1,000,000) traces; the protocol allocation audit; the realtime, builtins and builtins-graph probe mutation tests; `check-capi-abi.sh` and its self-test; `check-scalar-oracle-absent.py --native`; `check-builtins-fixtures.sh`; `check-effect-contract.sh target/release/bench`. The callgraph gate: `check-web-audioworklet.sh` on this branch's module. The render, meter-poll and command-submit closures are allocation-free. The V8 spill gate runs in `run-wasm-gates.sh`. | all exit 0 |
| 5. Other plans unchanged | `console-workload`'s pinned digests, in release. `check-console-fixtures.sh` against the release session-validator. `check-graph-determinism.sh` (100 fresh processes). `test-console-benchmark.sh`. `check-browser-expected-resources.py --artifacts` (browser digests and exact rows). `run-wasm-gates.sh` G5 (native and wasm corpus digests). P1's class-A tests in `bypass_cohorts` and `bypass_shunt_identity`, unchanged. The semantic estimate that feeds the canonical graph bytes is untouched; the #964 test asserts that controls do not move the canonical bytes. | all green, no digest re-pinned |

### The other gates

- `cargo check --workspace --all-targets --all-features`, clippy `-D warnings` and `fmt --check`:
  exit 0.
- Rustdoc, `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`. The only error
  is B0's known `tools/console-workload/src/lib.rs:374` fault. With `--exclude console-workload`,
  exit 0.
- Debug:
  - CI's debug-a set, with its features: 1,103 passed, 10 ignored, 0 failed. That includes
    `capi`'s retained-byte budgets, `host-core` and `host-web`.
  - The debug-b DSP set: 790 passed, 28 ignored.
- Release:
  - `graph`, `graph-compiler`, `effect-compiler`, `effect-contract` and `rack`: 389 passed,
    2 ignored. That includes `bypass_resources` and `bypass_cohorts` in release.
  - `audit`, `bench` and `console-workload`: 114 passed, 2 ignored.
  - `bench-support`'s own `current_thread_counts_every_allocator_operation` fails **in release
    only**, and fails the same way at `b0f37aa7`. LLVM elides its zeroed 16-byte alloc/free
    pair. CI runs that crate in debug, where it passes with the new field. Pre-existing, not
    touched.
- `scripts/check-cross-targets.sh` passes:
  - x86-64-v3;
  - `aarch64-apple-ios` and `aarch64-linux-android`: the product crates, `capi` included, checked
    and linted, with #1018's expected failures only;
  - wasm `simd128`;
  - the armv7 and scalar-wasm refusals.
- `scripts/run-wasm-gates.sh`, native + wasm `simd128` + the V8 spill gate: ok.
- The AudioWorklet artifact gates: `check-web-audioworklet.sh --without-metadata-regeneration`,
  `check-browser-expected-resources.py --artifacts` (digests, rows and budgets; 32 red
  mutations), `check-scalar-oracle-absent.py --wasm` and `test-web-audioworklet.sh`. All exit 0,
  and the render export's closure is allocation-free.
- **AArch64** has no toolchain or qemu here. The new tests read the host's width:
  - W = 4 gives a 4-lane bank in gates 1 and 2.
  - In gate 4, the Simd8 leg binds nothing, which the test expects.
  - `aarch64-debug` runs them when C2 is pushed.

### The AudioWorklet artifact: CHANGED, not re-pinned

`build-web-audioworklet.sh --module-only` gives `0f5c0ee7...` at `b0f37aa7` (Sol's P1 figure)
and `5d755046...` at this branch. `graph`'s runtime, `graph-compiler`'s estimate and
`effect-compiler`'s lowering all compile into the worklet. The release pin
`hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` (`6c952a2c...`) is untouched.

### Deviations from the authorized paths, and why

1. **`tools/bench-support/src/alloc.rs`** gains `Counters::released_bytes`. Gate 2 needs
   *measured* retained bytes, and the audited allocator counted no freed bytes.
   - A test-local counting allocator would put `unsafe` outside the realtime policy's approved
     list.
   - Requested bytes alone include every transient that compile and bind free, about 1 KB per
     bypass, and would force the estimate to overcharge.
   - The change is additive. No crate outside the file names `Counters` by its fields.
2. **`crates/graph-compiler/Cargo.toml` and `Cargo.lock`**: `bench-support` as a
   dev-dependency. `check-bench-policy.sh` allows this for `crates/`.
3. **`crates/graph-compiler/src/lib.rs`**, test module only. The #964 arithmetic test of
   `effect_control_resource` now carries the owner terms. It is the estimate's own unit test.
4. **The caller.** The spec names `effect-compiler/src/prepare.rs` as `effect_control_resource`'s
   caller. Its caller is `graph-compiler/src/compile.rs`, which is unchanged: the whole charge
   lives in `estimate.rs`.
5. **The owners' boxes are charged, not only windows and shunts.** Gate 2 needs the estimate to
   be at least the retained bytes, and a per-node shunt cannot exist without its `ConsoleEffect`
   box (304 B), nor a banked one without the console stage's growth (176 B). No other estimate
   row changed.

### For root

- **Stale doc sentences outside this slice's paths.** They say every effect that can bank is
  prepared enabled, which is no longer true for the multiband:
  - `docs/EFFECT_CONTRACT_V1.md:47-51`;
  - `effect-contract/src/lib.rs:974-977` (P2a's file);
  - the `BypassShunt` "When a shunt exists" paragraph in `effect-contract/src/live.rs`.
  The slice that lifts the multiband exclusion can fix them, or a one-line doc follow-up.
- **Live-console plans now carry a larger estimate.** Each lane's staging window of
  `automation_capacity` spans (40 B each) and every console owner's shunt are charged. Every
  host, `capi` and browser budget test above still passes. A host that sets the automation
  capacity near `u32::MAX` with a console attached is now refused at compile by its caps, where
  before it aborted at bind.

## Sol verdict, attempt 1

Sol, 2026-09-30. Verified `3bdba498`..`e3589bec` on `b0f37aa7` against this body, P1's verdict
(M1, M2, L2, L3), decision 12 and `baa03f09`.

**PASS.** Every objective gate holds on independent evidence. Twelve mutations of Sol's own go red,
including the two the brief named. There is no high or medium finding. The low findings are
documentation and records that root can take at merge.

### Evidence

**Class A: a differential render at three commits.** Sol wrote a scratch probe (not committed) and
compiled it unchanged at `6fdf5db2`, at `b0f37aa7` and at the branch. Each commit had its own fresh
release target directory. (A shared directory silently linked the branch's `graph` into the base
builds, because cargo keys path crates by relative path.) The probe ran 612 renders per commit at
Simd8, Simd4 and Scalar. It digested every word at `PostInputBuiltins`, `PostSimd1`, `PostDynamic`
and `PostSimd2PreFader` on every track, plus the output, with NaN folded. The renders covered:
- the multiband alone in SIMD rack 1, the dynamic rack and SIMD rack 2, on 8 tracks, with masks
  `00`, `ff`, `69`, `01`, `fe`, `aa`, `80`, `0f` and `3c`, fed music and random input;
- the multiband alone on 16, 5 and 4 tracks;
- each of the other seven launch effects alone, on 3, 8 and 13 tracks with mixed masks, fed music,
  runs of `-2^-149` (8,442 `-0.0` words reach the taps) and random input;
- the strip on 8, 13 and 16 tracks, with and without a mixed-bypass multiband in the dynamic rack,
  both console-free and with a live console attached;
- 63 pairs in which the first bypassed multiband lane is fed a `+-6e29` square wave and the other
  tracks' words are compared with a quiet render.

| Comparison | Bits | Bank shape |
|---|---|---|
| Branch vs `6fdf5db2`, all 612 renders | 0 differ | multiband 300/300 equal; the other effects differ exactly where #1087 intends |
| Branch vs `b0f37aa7`, the 288 renders without a multiband | 0 differ | 0 differ |
| Branch vs `b0f37aa7`, multiband | only the 39 hot renders in which P1 moved a neighbour | 161 differ: P1 banks the mixed masks |
| Hot neighbours moved | branch 0/63, `6fdf5db2` 0/63, `b0f37aa7` 39/63 (Simd8 and Simd4) | |

The shipped-plan digest gates are all green, and none is re-pinned:
- `console-workload` in release, including `the_console_strip_rows_render_their_pinned_bits`;
- `check-console-fixtures.sh`;
- `check-graph-determinism.sh` (100/100);
- `run-wasm-gates.sh`: native, wasm `simd128` G5 and the V8 spill gate;
- `check-browser-expected-resources.py --artifacts`, which checks digests and exact rows.

**Gate 1, the window bound.**
- `ConsoleEffect::new` now sizes the per-node window only for a live channel, the per-node twin of
  `baa03f09` (`crates/graph/src/runtime.rs:893-920`).
- A channel's consumer is private to `EffectControlLane` and is set only at construction.
  `attach_effect_console` replaces a lane before the graph compiles (`host-core/src/prepare.rs:844`),
  and bind moves each lane into its owner. No path attaches a channel to a bound owner, so no live
  lane can arrive and find no window.
- A live lane still gets its window. Sol's mutation 12 below empties it, and the graph's live
  tests go red, including `live_bypass_is_latency_preserving_and_reversible`.

**Gate 2, the estimate is an upper bound.** Sol's scratch sweep reuses the committed test's
method: the estimate delta against the audited allocator's retained delta, from prepare to bind,
with the producers dropped. It covers:
- all 8 launch effects;
- per node (Scalar, 1 and 3 tracks), W = 8 (8 and 13 tracks) and W = 4 (4 and 6 tracks);
- SIMD rack 1 and the dynamic rack;
- 1, 128 and 1,000 spans;
- one, all or alternate lanes bypassed; a live console on none, one or all but one bypassed lane;
- an `eq -> limiter -> comp` chain, with and without a console.

Results:
- 1,736 rows. 1,640 are exact (charged = retained), including every row without a multiband. None
  is overcharged.
- 96 multiband rows fall short. Each one is a mask that changes the plan's shape: the mixed cohort
  declines its bank, so effect-processor state moves outside `incremental_plan_bytes` (L3).
- With the shape held fixed (the same mask, console off against console on), 75 multiband rows are
  exact.
- Nothing double-charges. Stage growth, windows and the shunt are charged once per bank slot, and
  the per-node owner only for an entry that is not a bank member.

**Gate 3, the multiband.**
- `PREPARED_BYPASS_EFFECTS` is separate from `NEVER_BANKED_EFFECTS`.
- `lowers_session_bypass` is the one predicate the lowering reads.
- A live console seeds the multiband's lane from the session bypass, which equals the prepared
  bypass, exactly as `6fdf5db2` did.
- The class-A table above is the render evidence.

**Gate 4.** Render is allocation-free, and `-0.0` survives the bypass shunt. The committed tests
pass in dev and release, and both named `fma(0, wet, dry)` restores turn the `-0.0` case red
(mutations 9 and 10).

**Sol's mutations**, each reverted and each red:

| # | Mutation | Red in |
|---|---|---|
| 1 | Window bound reverted (`live = true`) | gate 1 aborts with `memory allocation of 171798691800 bytes failed` under a 32 GB address-space cap; gate 2 also red |
| 2 | Multiband dropped from `PREPARED_BYPASS_EFFECTS` | both new `bypass_cohorts` multiband tests (`metadata.bypass` false; 14 neighbour taps moved) and `native_session` |
| 3 | Every shunt charge removed | gate 2 (1,400 retained vs 376 charged) and the #964 unit test |
| 4 | Per-node shunt charge only | gate 2 (1,400 vs 376) |
| 5 | Banked shunt charge only | gate 2 (40,048 vs 752) |
| 6 | Banked shunt charged only for live lanes | gate 2 (40,048 vs 752) |
| 7 | `ConsoleEffect` box not charged | gate 2 (1,400 vs 1,096) |
| 8 | Per-node window not charged | gate 2, live console (26,176 vs 10,816) |
| 9 | `fma(0, wet, dry)` in `BypassShunt::apply` | `-0.0 scalar`: word 160 `0x00000000`, expected `0x80000000` |
| 10 | `fma(0, wet, dry)` in the rack's restore loop | `-0.0 lowered`: the same word |
| 11 | An allocation in the rack's banked restore | gate 4, Simd8: (768, 768) allocations and frees |
| 12 | A live lane's window emptied (`live = false`) | 3 `graph` console tests, 1 `graph-compiler` test, and `host-core` aborts |

**Gate 5 and the other gates.** All exit 0 unless stated.
- Format and lint:
  - `cargo fmt --check`;
  - clippy `--workspace --all-targets --all-features -D warnings`;
  - rustdoc `-D warnings`. Its only error is B0's `console-workload/src/lib.rs:374`. With
    `--exclude console-workload` it passes.
- Tests:
  - Debug, CI's debug-a set: 1,103 passed, 10 ignored.
  - Debug, the debug-b DSP set: 790 passed, 28 ignored.
  - Release, `graph`, `graph-compiler`, `effect-compiler`, `effect-contract` and `rack`: 389
    passed, 2 ignored.
  - Release, `audit`, `bench` and `console-workload`: 114 passed, 2 ignored.
  - `bench-support`'s `current_thread_counts_every_allocator_operation` fails in release only.
    It fails the same way at `b0f37aa7`, so it is pre-existing. Debug passes.
- Policies, and their mutation suites where CI runs them:
  - realtime (54 regions) and realtime-audit-leak;
  - effect-contract, rack, bench and graph;
  - workspace, host-core, conformance, lane, effect-runtime, artifact-evidence-leak, env
    vocabulary, builtins and session.
- The audit-native job, step for step:
  - `audit capi`, 100,000 calls: 0 allocations, frees, locks, syscalls and violations;
  - `audit delay`, `compressor` and `parametric-eq` at 100,000 blocks each, and `gate-expander`;
  - the builtins, builtins-graph and graph traces;
  - the protocol allocation audit;
  - the realtime probes and the 1,000,000-block trace;
  - the effect-contract trace at 1,000,000;
  - the builtins probe mutation tests;
  - `check-capi-abi.sh` and its self-test;
  - `check-scalar-oracle-absent.py`, `--native` and `--wasm`;
  - the builtins fixtures, `check-effect-contract.sh target/release/bench` and
    `test-console-benchmark.sh`.
- `check-cross-targets.sh`:
  - x86-64-v3;
  - `aarch64-apple-ios` and `aarch64-linux-android`, checked and linted, with only #1018's
    expected failures;
  - wasm `simd128`;
  - the armv7 and scalar-wasm refusals.
- The AudioWorklet gates on a delivery build: `check-web-audioworklet.sh`, whose callgraph gate
  finds the render closure allocation-free; `check-browser-expected-resources.py --artifacts`;
  and `test-web-audioworklet.sh`.

**The AudioWorklet artifact.**
- Sol reproduced both module hashes: `0f5c0ee7...` at `b0f37aa7` and `5d755046...` here. The
  module grows by 3,848 B (3,254,573 to 3,258,421), all of it control plane, per `twiggy diff`:
  - `effect_control_resource`: +858 B;
  - its `BTreeSet` to `BTreeMap` swap: +104 B of sort monomorphs and +632 B of drop glue;
  - the name section: +2,138 B;
  - `prepare_native_session_effects`: +106 B;
  - `graph::runtime::build_sequential`, bind-time: +10 B.
- No render function changed. The release pin is untouched, and Sol did not re-pin anything.

**Merge with P2a** (`codex/1088-partial-bank-mask`, `2320454c`).
- It merges with no textual conflicts.
- On the merged tree, `graph`, `graph-compiler`, `effect-compiler`, `effect-contract` and `rack`
  pass in debug: 395 passed, 0 failed.

**AArch64 was not run.** There is no toolchain or qemu here. The factories' width gates say the
Simd8 leg binds nothing on a 4-lane host:
- the EQ and the compressor bind only at the host's width;
- the limiter binds only up to it.

At Simd4, all three strip slots bank, which is what the new tests assume. `aarch64-debug` runs them
when C2 is pushed.

### Findings, ranked

**L1. The list of stale sentences is incomplete.** Each sentence below says that every session
bypass, or every bankable effect's, is lowered or prepared enabled. That is false for the multiband
since this slice.
- Outside this slice's paths; root fixes these at merge:
  - `docs/EFFECT_CONTRACT_V1.md:46-51` (listed);
  - `crates/effect-contract/src/lib.rs:959-961` (not listed) and `:974-977` (listed);
  - `crates/effect-contract/src/live.rs:116-118`, the `EffectControlLane` heading "A lane without a
    channel" (not listed);
  - `crates/effect-contract/src/live.rs:845-848` and `:859-861`, "When a shunt exists" (listed,
    without lines);
  - `crates/graph-compiler/src/banks.rs:64-66` and `:135-136` (not listed);
  - `crates/rack/src/lib.rs:897-898` (not listed);
  - test docs: `crates/graph-compiler/src/lib.rs:4169-4171` and
    `crates/graph-compiler/tests/bypass_shunt_identity.rs:522-523` (not listed).
- Inside this slice's own paths, which the slice should have fixed:
  - `crates/effect-compiler/src/prepare.rs:131-132`;
  - `crates/effect-compiler/src/prepare.rs:1394-1396`, which says "an effect that can bank is
    prepared enabled ... this channel replaces the channel-less lane";
  - the module doc of `crates/graph-compiler/tests/bypass_cohorts.rs:3-6`.

**L2. The deviations are accepted.** Each one is the minimum needed.
- `tools/bench-support/src/alloc.rs` adds `released_bytes`:
  - The change is additive. No existing field, and no gate or tool that reads one, changes
    meaning, and nothing serializes `Counters`.
  - The allocator is test- and tool-only; the bench policy and its mutation suite pass.
  - Render never frees, so the extra relaxed add in `dealloc` and `realloc` never runs in an armed
    window.
- The `graph-compiler` dev-dependency on `bench-support` is allowed for `crates/` by the bench
  policy.
- The #964 unit test pins exact totals, so it had to change. Mutation 3 turns it red.
- The owner-box (304 B) and stage-growth (176 B) charges go beyond the non-goal's "windows and
  shunts". Gate 2 needs them: mutation 7 shows the per-node case short by exactly the box.

**L3. The estimate and shape-changing multiband masks.** Declining a mixed multiband bank moves
effect state that the graph estimate does not describe: up to about 28 KB, and even a negative
estimate delta. Per-node processor state is charged at effect preparation (`EffectCompileCaps`),
the same as at `6fdf5db2`. It is neither a window nor a shunt, so it is outside this slice. Record
only.

**L4. The multiband joins the delay.** A live console cannot lift a session bypass on either of
them, because the prepared bypass stays in force. That matches `6fdf5db2`. Carry it into S1c
together with P1's L4. The slice that lifts the multiband exclusion ends it.

**L5.** The new tests' AArch64 legs are unverified here (see above).
