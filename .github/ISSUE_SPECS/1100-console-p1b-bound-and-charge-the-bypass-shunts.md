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
