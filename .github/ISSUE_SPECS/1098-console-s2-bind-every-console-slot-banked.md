# Bind every console slot banked for every track count

Slice S2 of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's H2, H5, M3, L3 and amendments 3,
9 and 10 in `docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`).

## Problem

Decision 12 makes console slots always bank, on every target and for every track count, with
partial groups padded and no member threshold. After P1, P2a-P2e and S1a:
- the mechanism exists;
- every eligible effect accepts padded requests;
- a bypassed lane stays in its bank;
- console slots lower to the `Simd1` and `Simd2` racks, which after S1a no session can reach any
  other way.

What is missing is the policy and the guarantee. The planner still pads nothing, so a console
remainder renders per node.

## Smallest closable slice

1. **Policy.** The planner forms one bank group per (slot, pool class, dependency level) for every
   console slot, and pads each to W (H2, M3). Inserts keep today's rule: full groups bank and
   remainders render per node (decision 12, "Inserts bank opportunistically, as today").
2. **Guarantee.** On every production backend (Simd8 and Simd4), a console slot never renders per
   node. If a console group does not bind banked, compilation fails with a typed diagnostic (for
   example `console.slot.unbanked`) naming the slot, pool class and level. There is no silent
   fallback. S1a's eligibility refusal makes that unreachable for valid sessions; the diagnostic
   guards against regressions. The test-only `Scalar` oracle (#1059), which exists only for tests
   and `test-support`, is exempt: it is the per-node reference that gate 2 compares against.
3. **#971 under padding** (amendment 10). Once every console group binds, the stranded-mono
   demotion's "keep the move only if it binds more banks" objective
   (`crates/graph-compiler/src/banks.rs:243-356`) measures nothing for console slots. Decide one of
   the following, state why in this spec's evidence, and pin the decision with a test:
   - restate the objective, for example minimise planes x banks across the track's slots;
   - retire the demotion for console slots.

   The mono pool is worth about 36 % on the standing console row (M3), so the choice must not
   silently demote mono tracks.
4. **Record what grouping costs.** Differing insert counts split `post_insert` into one group per
   level, and misaligned cohorts pay a planar/AoSoA round trip at chain fusion (H2). Do not fix it
   here. An ALAP alignment of `post_insert` banks is a successor only if S4 measures a need.

Authorized paths: `crates/graph-compiler/src/banks.rs` and its planner call sites, the compile
diagnostics, their tests, and this spec.

## Owner decisions that bind this slice

- Console slots always bank, with no threshold. The owner accepted H5's cost: a one- or
  two-member remainder costs more padded than per node at W=8.
- Banking may couple lanes' cost, never their bits.
- Bypass is per lane, and a bypassed lane stays in its bank.
- The silent fast path stays bank-wide. A per-lane silence skip is a later issue if S4's
  sparse-activity row warrants it.

## Dependencies

This is batch C4, after batch C2 (P1, P2a-P2e) and batch C3 (S1r, S1a-S1d) have been pushed.

- *Keep a bypassed lane in its effect bank* (P1).
- *Pad parametric EQ banks with inactive lanes* (P2b).
- *Pad compressor banks with inactive lanes* (P2c).
- *Pad true-peak limiter banks with inactive lanes* (P2d).
- *Pad gate/expander, transient shaper and soft-clip banks with inactive lanes* (P2e).
- *Add the session console and per-track inserts to the session schema* (S1a).

## Objective gates

1. **Binding.** Every console slot binds banked at N in {1, 3, 5, 9, 10, 13}. The test is on the
   compiled plan: no console slot has a per-node node. It covers:
   - mixed bypass;
   - mixed insert counts, so `post_insert` splits by level;
   - mono and stereo pool classes;
   - each eligible effect as a slot.

   It runs at Simd8 on x86-64 and at Simd4 through `scripts/run-aarch64-tests.sh` or the wasm gates
   (L3). The group count equals the sum over (pool class, level) of `ceil(n / W)`.
2. **Class A.** A committed randomized differential renders these sessions banked, on a
   production backend, and per node, through the test-only `Scalar` oracle. Every track is
   bit-identical in both, with NaNs folded (decision 10). A planted
   whole-bank decision that is not bit-neutral per lane turns it red. It names the true-peak
   limiter explicitly as a padded `post_insert` slot, with mixed bypass, at Simd8 and Simd4, fed
   hot enough that the gain path runs (#1091 verdict L3).
3. **Diagnostic.** A planted factory decline on a console group, on a production backend, fails
   the compile with the typed diagnostic and never yields a per-node plan. The `Scalar` oracle's
   exemption does not reach a production backend: a test compiles the same session on Simd8 with the
   planted decline and expects the refusal.
4. **#971.** The chosen rule is pinned by a test on a session with stranded mono tracks.
5. **Realtime.** `scripts/check-realtime-policy.sh`, the realtime audits and the callgraph gates
   pass, and render allocates nothing.
6. **Resources.** A padded bank's estimate charges member metadata per active member, not per
   lane: `graph-compiler/src/banks.rs`'s `checked_mul(members)`. A test pins it, and it goes red
   if the factor becomes `lanes` (P2a verdict, L2).
7. **PR evidence.** The 64-track console digests are unchanged. The ragged 9-track fixture's
   render is unchanged, and its plan now binds the remainder banked; explain every moved plan pin.

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

Terra, 2026-09-30, on `codex/1098-console-always-banks` from `8bb015ea` (the local C3 batch). Commits
`b19f278b` (policy, guarantee, #971), `01f1bfa5` (the gates and the adapted tests), `9daba0c9`,
`8894e678`, `fe8458e9` (fixes found by the gates), `19be9c2d` (a comment), and this record.

### What changed

`crates/graph-compiler/src/banks.rs` only, for the engine:

- **Policy.** `pads(group)` is true exactly for a console group: `is_console_rack` is the `Simd1`
  and `Simd2` racks, which after S1a hold exactly `pre_insert` and `post_insert`. An insert group
  (`Dynamic`) never pads, so a full insert group banks and its remainder renders per node, as
  before. The planner already formed one group per (rack, pool class, level of the chain's first
  slot) and chunked it by `W`; padding binds the partial one. Every console track carries every slot
  in one order, so a console rack's chains share one program, and each console slot therefore binds
  `sum over (class, level) of ceil(n / W)` banks.
- **Guarantee.** After the plan is final, `unbanked_console_slot` finds the first console node that
  no bound bank carries and fails the compile with `console.slot.unbanked` at
  `$.console.<section>[slot=<id>].bank[pool=<mono|stereo>,level=<level>]`. It runs only on a vector
  backend: `Backend::Scalar` returns with no banks before it, so the test-only oracle is exempt. A
  node missing from the dependency levels is `graph.internal.invariant`.
- **Removed.** P2a's test-only `BankPadding::EveryGroup` policy and its thread-local: the production
  policy now pads console groups, and nothing used the every-group mode after `bank_padding` was
  rewritten against the production policy (AGENTS.md: a superseding change deletes what it
  supersedes).
- `docs/SESSION_SCHEMA_V1.md` states the banking rule and the diagnostic.

### #971 under padding: retired for console slots

`stranded_mono_tracks` now counts a group as stranding its members only when it is a partial mono
group **that is not padded**. A console group is always padded, so a track that carries a console
slot (in a session with a console, every track) is never stranded and never moved. A session with
no console keeps #971 unchanged. Why retire rather than restate:

1. **The premise cannot hold.** The demotion rescues a mono track that banks nowhere. A console
   slot always banks, so a console track is never in that state.
2. **The objective measures nothing for console groups (M3).** Moving a console remainder can only
   merge console groups, so the bound-bank count never rises, and it scores a merge that saves a
   padded bank as a loss. It does rise when the move also fills an *insert* group, and then it
   demotes a mono track for an insert remainder, which is gate 4's session.
3. **The mono pool is the larger win.** A padded mono bank collapses to one plane; the pool is worth
   about 36 % on the standing row (M3). Planes x banks would demote a remainder only when it fits the
   stereo pool's padding at every slot and builtin stage, to save one one-plane padded bank per
   stage, and weighing that against a filled insert group needs a per-node cost nobody has measured
   at four lanes (H5 is eight-lane arithmetic on an uncontrolled record). A successor on S4's
   measured need, not here. The owner's two-percent rule favours the simpler rule.

On the dogfood layout (18 mono of 81) retirement is also the cheaper plan: all 18 stay mono in
`ceil(18 / W)` cohorts and the session binds exactly the all-stereo session's bank count at eight
and at four lanes (the padded remainders replace the partial ones), where #971's move would have
bound as many banks with two fewer collapsing tracks.

### Gates

| Gate | Test | Result |
|---|---|---|
| 1 Binding | `console_banking::every_console_slot_binds_banked_at_every_track_count` | pass at Simd8 |
| 2 Class A, randomized | `console_banking::randomized_console_sessions_render_the_per_node_bits_on_every_track` | pass, 24 seeds per PR; 400 seeds in release: 993 padded banks with a bypassed lane, 114 padded mono banks |
| 2 Class A, the limiter | `console_banking::a_hot_padded_post_insert_limiter_renders_the_per_node_bits_at_both_widths` | pass at Simd8 and Simd4 |
| 3 Diagnostic | `console_banking::a_console_group_a_factory_declines_fails_the_compile` | pass |
| 4 #971 | `console_banking::a_console_track_is_never_pooled_as_stereo_to_fill_a_bank` | pass |
| 5 Realtime | lint, audits, callgraph, `bypass_resources` console leg | see below |
| 6 Resources | `bank_padding::a_padded_bank_charges_member_metadata_per_active_member` | pass |
| 7 PR evidence | below | 22 of 22 digests identical |

**Gate 1.** Two consoles on every track, each with its own bypass mix and zero, one or two EQ/comp
inserts in turn, at N in {1, 3, 5, 9, 10, 13}: all six eligible effects (EQ, compressor,
gate/expander and transient shaper in `pre_insert`; soft-clip and limiter in `post_insert`), and the
collapse-capable EQ, compressor and limiter strip, on which even tracks pool mono. The gate/expander,
soft-clip and transient shaper keep the contract's default `channel_symmetry() == false`, so a track
carrying them pools stereo, which is why the mono pool is exercised on the second console. Per rack,
the group count equals the sum over (class, level of the first slot) of `ceil(n / W)`, every slot of
every group binds, `post_insert` sits at `min(n, 3)` levels, every track pools as its own class, and
a bank holds bypassed and unbypassed lanes.

**Gate 2.** Per-track bits are read at each track's post-pan stage by a graph observer (the parent
module's `BitRecorder`), NaNs folded to `0x7fc00000`, as well as the session output, against the same
session compiled at `Backend::Scalar`, both armed for the mono collapse as `host-core` arms it. The
generator draws N, a console of one to four distinct eligible effects split over the two sections,
per track zero to two random inserts, a random bypass per console slot, per-track parameter values
(EQ gain, compressor threshold, limiter ceiling), a mono or stereo feed, a silent, quiet or +6 to
+18 dBFS feed, and now and then a NaN sample (which the input section's bank sanitizes on its lane).
The named limiter case puts the limiter alone in `post_insert`, three levels deep by insert count,
per-track ceilings, a bypass pattern that mixes every (pool, level) group of two or more, and shows
the gain path runs (every unbypassed track's bits move against the all-bypassed session). It runs at
Simd8 and Simd4 wherever the limiter binds them (both on `x86-64-v3`); on a four-lane build the
Simd8 compile must be refused with `console.slot.unbanked`.

**Simd4.** The x86 limiter binds four lanes, so the named limiter case runs at Simd4 here. Every
other console effect binds only its build's width (D4), so gates 1, 2 (randomized), 3 and 4 run at
Simd4 on the AArch64 debug leg (`scripts/run-aarch64-tests.sh debug`: `graph-compiler` is a product
crate), which this host cannot run. The wasm gates ran here and hold the effects' four-lane kernels
to their pins. Every test was written width-agnostic and reviewed for a four-lane host.

**Gate 3.** `W + 1` mono tracks with the limiter as the one console slot, against a limiter double
that declines every padded request: at the host's width and at `Simd8` the compile is refused with
exactly `console.slot.unbanked`, and at the host's width the path is
`$.console.post_insert[slot=post-limiter].bank[pool=mono,level=<the padded limiter's level>]`. The
same session and registry compile at `Scalar` with no bank; the same strip as inserts compiles and
its remainder renders per node.

**Gate 4.** `ch00..=ch{W}` mono, `W - 1` stereo, the limiter as the console slot and EQ then
compressor as every track's inserts. Under #971's rule as it stood, `ch{W}` strands and its move
completes the stereo insert group (two banks) for one padded limiter bank, so it was kept. Now every
mono track stays mono, five banks bind (three limiter banks, the mono insert group's two), and the
remainder's limiter is a padded mono bank. The console-free counterpart (the same chain as inserts)
still moves `ch{W}` and binds six.

**Gate 6.** For each strip slot of `W + 1` tracks, the full bank's metadata less its id strings
exceeds the one-member padded bank's by exactly `(W - 1) * size_of::<EffectNodeId>()`, and both
charge the same scratch. It is the difference of two estimates, not a pinned byte count.

**Mutations** (each applied alone to the committed tree, the named tests run, then reverted):

| Mutation | Red |
|---|---|
| M1 console groups do not pad (`pads` false) | gate 1, gate 2 (both), gate 4 |
| M2 no guarantee (`unbanked_console_slot` finds nothing) | gate 3 |
| M3 a padded group strands its members again (drop `!pads(group)`) | gate 4 |
| M4 member metadata charged per lane | gate 6 |
| M5 whole-bank shunt: restore the dry signal on every lane of a bank with one bypassed lane (`rack`) | gate 2 randomized and gate 2 limiter |
| M7 every lane bound with the first member's request | gate 2 randomized and gate 2 limiter |
| M8 insert groups pad too | `bank_padding::a_console_remainder_binds_one_padded_bank_and_an_insert_remainder_renders_per_node` |

Recorded green, and why: a whole-bank D7 reset in the limiter (reset every lane when one fails) is
green, because no graph render reaches an effect's D7. The input section sanitizes any `|x| >= 1e30`
on its lane before the first effect, and no launch effect amplifies a finite block past that limit.
Per-lane D7 in each effect is P2b-P2e's gate (`51a1514a`), in each effect's own crate. Gate 2's first
version also missed M5 in the limiter case, because its bypass pattern equalled its level pattern;
`9daba0c9` fixed that before this record.

### Class A and the moved pins (PR evidence)

- **The 64-track console digests.** `cargo test --release -p console-workload --test
  gain_pan_profile digests -- --ignored` at the head and at `8bb015ea` (a `git archive` in scratch,
  built into the same `target/`): **22 of 22 rows identical**, the nine-, ten- and thirteen-track
  ragged strips and the nine-track baseline among them.
- **The ragged nine-track fixture.** Its render is unchanged (`nine_track_ragged_strip`
  `17613a3a…198a`, identical at base and head). Its plan now binds the remainder banked: the
  one-track tail's EQ, compressor and limiter are padded banks of one, so its whole strip fuses into
  one chain, `[chains, slots]` `[3, 9]` -> `[2, 12]` at eight lanes (`[4, 15]` -> `[3, 18]` at four).
  `chain_shape.rs` repins it.

Every moved pin, and why it moved:

| Where | Pin | Base -> head | Why |
|---|---|---|---|
| `console-workload/tests/chain_shape.rs` | ragged strip shape | `[full + 2, 6 * full + 3]` -> `[cohorts, 6 * cohorts]` | the tail's strip banks padded and fuses |
| same | seam-side-only chain | built at the native width -> the other width | the tail no longer has one; a per-node stage before the fader is now an insert's, which `SessionRuntime` makes at the other width |
| `graph-compiler` `add_a_track_…` | EQ banks, chains | `full_cohorts` -> `cohorts`; `2 * cohorts` -> `2 * observed + unobserved` | the ninth EQ binds padded; its cohort has no `PostSimd1` observer, so it fuses whole |
| `mixed_twelve_track_…`, `scalar_dispatch_…`, the audit-037 leg | banks | `12 / W` -> `ceil(12 / W)`, tails 0 | the conformance delay double is a console slot and pads |
| same, connected sidechain probe | outcome | per-node fallback -> `console.slot.unbanked` | a planted sidechain on a console slot is the regression the guarantee refuses |
| same, level-split probe | outcome | unscheduled member left out -> member at its own level banks alone, padded | H2: a level split costs a group, never a per-node render |
| `launch_parametric_eq_…`, `…_limiter_…`, `…_soft_clip_…`, `…_transient_shaper_…` | banks, tails, members | `n / W`, `n % W` -> `ceil(n / W)`, 0 | the remainder pads; for the limiter, soft-clip and transient shaper the estimate is compared, above each compile's semantic estimate, against the strip folded into inserts, whose IDs name another rack; the render and schedule against `Scalar` |
| same (EQ) | per-node arm | declining registry on the host -> refused, and `Scalar` | a declined console group fails the compile |
| `rack_placement_…` | banks per placement | equal -> console `ceil(10 / W)`, inserts `10 / W` | only the console remainder pads; bits and per-bank bytes stay equal |
| `the_merged_span_hold_…` | per-node arena | 193 -> 129 | the per-node plan is now the strip folded into inserts, where the compressor runs in place over the EQ; 256 banked unchanged |
| `console_sixty_four_track_fixture_…` | per-node arm | declining registry -> the strip folded into inserts; schedule against `Scalar` | as above |
| `a_single_odd_track_…` (#971) | banks, pools | 24, pools 56 / 8 -> `3 * (ceil(63 / W) + 1)`, pools 63 / 1 | retired demotion; the odd track banks alone, padded |
| `the_mono_pool_keeps_whole_cohorts_…` (#971) | mono pool, armed chains | first 16 -> all 18; `16 / W` -> `ceil(18 / W)` | retired demotion; the bank count still equals the all-stereo session's |
| leased-meter, route-fold, fuse tests | oracle chains, folds | 2 per cohort, 64 folds -> 0, 0 | the oracle is `Scalar`, where nothing banks |
| `a_meter_on_a_bank_member_…` | session | console -> strip as inserts | the shape needs a per-node EQ after the post-input bank |
| `bank_levels.rs` | native banks (Simd8 / Simd4) | 21 / 45 -> 23 / 47 (three less-one-EQ, mono); 20 / 43 -> 22 / 45; 2 / 6 -> 7 / 11 | H2: the early limiters form their own level's padded group and the remainder pads; the #962 shape's console pads and its limiter sits at three levels. The Simd4 pins are derived from the four-lane plan on `x86-64-v3` (the rule that reproduces every Simd8 pin on this host); the AArch64 leg runs them |
| same, all widths | foreign width | compiled -> may refuse with `console.slot.unbanked` | a width this build's factories decline (D4); the native width and `Scalar` must compile; the randomized probe tallies them (11 of 64 seeds at Simd4 here) |
| `bypass_resources.rs` | per-node cases | console -> inserts; plus a console leg | a lone console slot binds padded; the console leg renders padded banks with bypassed lanes allocation-free |
| `capi/tests/resource_lifecycle.rs` | effect-bank budgets | 9,024 / 704 (8 lanes), 9,024 / 832 (4) -> 18,048 / 896, 13,568 / 1,024 | structural, with its reason in place: the ninth EQ's padded bank (16,384 / 805 measured at eight lanes; 12,288 / 921 derived at four) |
| `host-core/src/limiter_linked_session.rs` | the foreign-width leg | console -> strip as inserts at the other width | the console cannot bank there; the words match `PIN` at every width |
| `console-workload` `SessionRuntime::build_full` | foreign width | console as written -> folded into inserts | as above; digests at Simd4 unchanged (`eq_ramping_scenario`, `paired_spans`) |

**One design reading to check.** The guarantee holds on every vector backend, so a compile at a
width this build's factories decline (Simd4 on `x86-64-v3` for any console but the limiter; Simd8
on AArch64) is refused rather than rendered per node. No host compiles at a foreign width
(`Backend::current()` is every host's dispatch, #99 F6); only tests did, and they now fold the
console into inserts there or accept the refusal. The alternative, exempting foreign widths, would
need the compiler to read the build's backend, which #99 F6 forbids, and would be exactly the silent
per-node fallback the spec rules out.

### What grouping costs (recorded, not fixed)

Measured on the intended strip (`pre_insert` EQ, compressor; `post_insert` limiter) at eight lanes,
a throwaway probe, one block:

| Session | `simd1` groups | `dynamic` groups | `simd2` groups | effect / builtin banks | `[chains, slots]` | round trips per block |
|---|---|---|---|---|---|---|
| 13 tracks, no inserts | 2 | 0 | 2 | 6 / 6 | `[2, 12]` | 2 |
| 13 tracks, 0/1/2 EQ inserts | 2 | 1 | 3 | 8 / 8 | `[6, 16]` | 6 |
| 64 tracks, no inserts | 8 | 0 | 8 | 24 / 24 | `[8, 48]` | 8 |
| 64 tracks, 0/1/2 EQ inserts | 8 | 6 | 9 | 32 / 26 | `[20, 58]` | 20 |

- **`post_insert` level splits (H2).** Differing insert counts put the limiter at one level per
  count: 64 tracks split 22 / 21 / 21, so `post_insert` forms 9 padded groups where 8 would do.
- **Round trips.** The same sessions' fader and matrix banks split by level too, the insert
  remainders render per node, and a cohort whose `post_insert` bank does not line up with its
  `pre_insert` one cannot fuse: 20 planar/AoSoA round trips per block against 8. Not all of that is
  the console's; inserts cost chains by themselves. An ALAP alignment of `post_insert` banks is the
  successor, only if S4 measures a need.
- **H5.** A remainder of one pays a whole padded bank for one member at eight lanes. The nine-,
  ten- and thirteen-track rows are where S4 measures it.

### Gates run

On an x86-64-v3 host (AMD EPYC 7313P), rustc 1.97.1, `CARGO_INCREMENTAL=0`, one `target/`, at
`19be9c2d` unless noted. Every step exited 0.

| Gate | Command | Result |
|---|---|---|
| fmt, clippy, doc | `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | ok (the doc run found a private intra-doc link, fixed in `8894e678`) |
| lint job | every `check-*`/`test-*` step of `qualification.yml`'s lint job, 46 commands | ok (`check-graph-policy.sh` found a `Mutex` in the new test file, fixed in `fe8458e9`) |
| test-debug-a | the job's `cargo test --workspace --all-targets --exclude …` with its features | 96 binaries, 1,150 passed, 0 failed |
| test-debug-b | the job's DSP-crate `cargo test` with its features; `conformance_fixtures -- --check` | 151 binaries, 834 passed; fixtures ok |
| test-release | `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane` | 109 passed |
| release, affected crates | `cargo test --locked --release -p graph-compiler -p host-core -p capi -p console-workload --features host-core/test-support` | 38 binaries, 407 passed |
| audit-native | release build; `cargo test --release -p audit -p bench -p console-workload` (114 passed); `audit capi` (100,000 calls on the nine-track EQ session, now with its padded bank: every violation counter 0, `pcm_digest` `ff6cdcb96cdcdad5`); `audit delay`, `compressor`, `parametric-eq` at 100,000 blocks; `audit gate-expander` (`total_violations` 0); the builtins, builtins-graph, graph and effect-contract traces; the protocol allocation audit; the realtime, builtins and builtins-graph probe mutations; `check-capi-abi.sh` and its self-test; scalar oracle absent from `libcapi.so`; graph determinism 100/100; builtins fixtures; console fixtures; `check-effect-contract.sh` (8 factories, 0 failed gates) | ok |
| wasm | the job's `simd128` checks of `target-smoke`, `protocol`, `dsp-reference`, `conformance`; `check-protocol-wasm-parity.sh`; `scripts/run-wasm-gates.sh` with its native leg and the V8 spill gate (142 cases, 358 comparisons, 0 mismatches on each leg; spill gate ok) | ok |
| cross-target | `scripts/check-cross-targets.sh`: aarch64 iOS and Android product crates checked and linted | PASS; the #1018 iOS `memset_pattern16` rows unchanged (parametric-eq 146) |
| artifact | `build-web-audioworklet.sh` | module `ac3a9353…efc9`, 3,289,705 B, against base `e7f2ad31…80be`, 3,283,569 B: +6,136 B (+0.19 %), the padded-bank bind paths now reachable in production |
| artifact-gates | `check-web-audioworklet.sh --without-metadata-regeneration` (the render, meter-poll and command-submit callgraph gates included), `check-browser-expected-resources.py --artifacts`, scalar oracle absent from the module, `test-web-audioworklet.sh`, V8 spill self-test and gate | ok |
| console benchmark | `scripts/test-console-benchmark.sh`; `scripts/operator/preflight-console-benchmark.sh --step s2-preflight-1098` | PASS, 0 workload launches |
| V8 harness | `run-web-mixing-automation-benchmark.sh prepare` and `preflight`, at the head and at `8bb015ea` (a detached worktree in scratch) | all nine preflight digests identical at base and head; the seven arm digests equal S0's recorded `preflight_output_sha256` (`014e5f5b…`, `e7025b5c…`, `2540aff4…`, `c29d12a7…`, `8db18991…`); the two documents (`d913ad96…`, `3dd8b2ff…`) have no S0 preflight digest, so base is their witness |
| AArch64 legs | not run | no arm64 host here; see "Simd4" above |

**A local slip, recorded.** The mutation script restored each file with an mtime older than its
mutated build, so after M5 Cargo kept the mutated `rack` in the debug profile. The first
test-debug-a run caught it (both gate-2 tests red with M5's message); `rack`, `banks.rs` and the
limiter were touched and rebuilt, the gate-2 tests passed, and test-debug-a and the debug steps of
the console preflight were run again (the table's numbers). The release artifacts, clippy and the
lint scripts were built before the slip or from fresh mtimes.

### Scope note

Beyond `banks.rs`, its call sites, the diagnostic and their tests, this attempt touched tests that
padding broke in other crates -- `capi/tests/resource_lifecycle.rs`,
`host-core/src/limiter_linked_session.rs`, `tools/console-workload` (the foreign-width fold in
`SessionRuntime::build_full` and `chain_shape.rs`) -- and `docs/SESSION_SCHEMA_V1.md`. Each change is
a test harness or a doc, and each is explained in the table above.
