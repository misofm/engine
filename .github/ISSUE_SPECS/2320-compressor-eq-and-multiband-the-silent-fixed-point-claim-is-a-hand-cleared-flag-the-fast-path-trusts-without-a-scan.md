# Compressor, EQ and multiband: the silent fixed-point claim is a hand-cleared flag the fast path trusts without a scan

Tracker: #2258, items FX-6 (dead `silent_bypass`) and the FX-1 follow-up that #2264 left open
(compressor, parametric EQ, multiband). Found by the implicit-state-machine audit of 2026-11-06.

## Problem
The compressor (`crates/compressor/src/lib.rs`), the parametric EQ
(`crates/parametric-eq/src/lib.rs`) and the multiband compressor
(`crates/multiband-compressor/src/lib.rs`) keep the #163 phase 4 / #893 silent fixed-point
claim as a loose `silent_fixed_point: bool` that every between-block writer must clear by hand
(compressor 8 sites, EQ 11, multiband 7). While the claim stands, a quiet block takes the silent
fast path and skips the kernel. The quiet test does **not** re-read the state the claim stands for
(the compressor's `gain_reduction_db`, rate and link ramps; the EQ's integrators; the multiband's
band state), so a missed withdrawal would freeze a lane that is not at rest and move rendered
bits. Claim-implies-scan is checked nowhere outside `rest_tests`.

The compressor also carries `silent_bypass`, compared against `bypass` in both quiet tests, and the
merged limiter `SilentClaim` (#2264) carries the same word. In both effects `bypass` is fixed at
preparation and never reassigned, so the comparison is always true: dead state.

## Contract
- Find out, and record per effect, whether any mutator leaves a stale claim today. If one does, a
  regression test reaches it, red on main.
- One `SilentClaim` newtype with private state, shared by the three effects in `effect-runtime`
  (the crate whose purpose is one copy of what every effect needs). It is earned only from a
  rendered block's quiet test and the scan's answer, or from a whole-bank skip whose mask came from
  the scan, and withdrawn through one method.
- `debug_assert!(claim => scan)` where the claim is trusted: each effect's dual and collapsed fast
  path.
- Delete the compressor's `silent_bypass` and its quiet-test leg; delete the limiter
  `SilentClaim`'s `bypass` word and the `bypass` arguments that feed it (minimal: #2285 / PR #2314
  edits the same file).
- No rendered bit, no state word and no claim value may move.

## Non-goals
Moving the limiter onto the shared type (its skip rule differs and #2314 is open on the file).
The "withdrawing accessor" long-term model #2264's review recommended. New withdrawals on paths
that are benign today. FX-2.

## Gates
- A red-on-main regression test per effect, or a recorded proof that no stale claim is reachable.
- A one-time no-bit-moved comparison against base per effect, as PR evidence.
- `cargo test --locked` for the touched crates and their in-workspace consumers, clippy
  `-D warnings`, `cargo fmt --check`, rustdoc `-D warnings`, `scripts/check-workspace-policy.sh`,
  the realtime policy tool.

## Evidence (attempt 1)

Base: `main` @ `5d403acd8`.

**The bypass words are dead (FX-6).**
- Compressor: `Instance::bypass` is written only in `Instance::new`. `silent_bypass` started at
  `metadata.bypass` and was reassigned `self.bypass` after every kernel run, so
  `self.silent_bypass == self.bypass` was always true in both quiet tests.
- Limiter: `LimiterCoef::bypass` is written only in `LimiterCoef::new`. `SilentClaim::new` took
  `metadata.bypass` and every `settle`/`settle_skipped` passed `self.coefficients.bypass`, so
  `bypass_held` was always true and `proved_under(bypass)` was `proved`. `settle_skipped`'s
  `whole && bypass_held(..)` was `whole`.
- Keeping the leg for a future per-block bypass does not hold either: at rest both arms of each
  kernel's output `select` write the `+0.0` they read, and both effects advance their state on both
  arms (the compressor's detector runs under bypass, `kernel::Channel::rest_lanes`; the limiter's
  rings advance on both arms), so the rest state is a fixed point under either arm. Recorded in
  `crates/true-peak-limiter/tests/MUTATIONS.md`, which used to argue for the leg.
- EQ and multiband never had the word (the tracker's "pair" is the compressor's and the
  limiter's): a bypassed EQ bank renders nothing, and the multiband claim documents its bypass as
  fixed at preparation.

**Reachability: no stale claim is reachable today, in any of the three effects.** In each, the
quiet test reads only ramps and the input planes, never the state the claim stands for, so a
missed withdrawal would move bits, not only speed. Every `&mut` path, checked:

- *Compressor* (`Instance`, `PreparedCompressor`, `PreparedCompressorBank`). Earned only at the end
  of `render`/`render_mono` (quiet and the scan answered every lane) and by a whole-bank
  `advance_clocks` (the caller's mask debug-asserted against `lanes_at_rest`). Withdrawn by
  `retarget_link`, both resets, `reset_lane`, `restore` (after commit), `write_lane_words`
  (`copy_lane_from`, `swap_lanes_direct`, both lanes and the peer), the scalar
  `apply_parameter_point`, and any block carrying automation (scalar and bank).
  `exchange_lane_defaults` does not withdraw: it writes `defaults` (never read by the kernel), the
  lane's link mode and law, and seats the lane's link weights at rest at the new mode. The scan
  reads `gain_reduction_db` and every ramp's `remaining`, all unchanged or settled, and at rest a
  `+0.0` detector gives `+0.0` under any link law, so the claim still equals the scan. Benign, and
  `swap_bank_lanes` follows it with a restore that withdraws. `trade_defaults` runs only inside
  `swap_lanes_direct`, before `write_lane_words`. `desymmetrize` copies the left channel, which a
  collapsed claim is about, onto the right. A D7 recovery runs inside the block, before the settle.
- *Parametric EQ* (`PreparedParametricEq`, scalar and bank). Earned only at the end of
  `render`/`render_mono` and by a whole-bank, non-bypassed `advance_clocks`. Withdrawn by
  `apply_target_lane` (both surfaces), `reset_state`, `reset_lane_state`, a successful restore
  (both surfaces; a refusal is write-free), `copy_lane_from`, `swap_lanes_direct` (self and peer),
  and any block carrying automation (whose spans are refused but still withdraw).
  `exchange_lane_defaults` writes only `initial`/`initial_words`, which the kernel never reads.
  `desymmetrize` as above. A bypassed bank renders nothing and leaves the claim alone.
- *Multiband* (`PreparedMultibandCompressorBank`; the scalar instance has no claim). Earned only at
  the end of `process_bank_inner` (both bodies) and by a whole-bank `advance_clocks`. Withdrawn by
  both resets, `reset_lane`, an accepted `retarget_link`, every `restore_track_state_payload`
  (even a refused one) and any block carrying automation. It implements no direct copy, swap,
  defaults exchange or prepared target (the trait defaults change nothing; `swap_bank_lanes`
  refuses it at `lane_default_bytes`). `desymmetrize` as above.

So no test can be red on main for a reachable stale claim. Hardened instead, below.

**Model chosen.** One `SilentClaim` in `effect-runtime` (`crates/effect-runtime/src/silent.rs`),
the crate whose purpose is one copy of what every effect needs, rather than three private copies:
the three claims have the same transition table, which the type now owns.
- `new` (withdrawn), `withdraw`, `settled(quiet, scan, every)` (a rendered block; `scan` is a
  closure called only for a quiet block, as each effect's `&&` evaluated it, so no loud block pays a
  scan), `settle_skipped(whole)` (a whole-bank skip earns; a partial one keeps, as the three did),
  `trusted(scan, every)` (the fast path's read; debug-asserts the scan when the claim stands;
  release never calls `scan`) and `proved` (for gates that hash a bank's words).
- `debug_assert!(claim => scan)` at both fast paths of each effect, over the channels the claim was
  earned on: both on the dual body, the left alone on the collapsed one.
- The word is private to `effect-runtime`: an effect can no longer set it to `true` by hand, only
  earn it through `settled` or `settle_skipped`.
- Not shared with the limiter: its `settle_skipped` withdraws on a partial mask (its clocks move),
  its `lanes_at_rest` also trusts the claim, and PR #2314 (#2285) is open on the same file. The
  limiter change is the minimal one: the dead `bypass` word and its arguments.
- Every restatement is exact: `flag = quiet && scan() == all` is `settled(quiet, || scan(), all)`;
  `if whole { flag = true }` is `settle_skipped(whole)`; `quiet && flag` is
  `quiet && trusted(..)` (same value; the scan runs in debug builds only, and only on that branch).

**Test.** `rest_tests::a_stale_silent_claim_is_refused_where_it_is_trusted` in each of the three
crates (debug builds), at every vector width. It brings a bank to rest with the claim standing,
plants one off-rest word on lane 0 without withdrawing (compressor: the envelope; EQ: a section
integrator; multiband: a crossover integrator), and renders a `+0.0` block through the dual body
(word on the right channel) and the collapsed body (word on the left). Each must panic with the
claim-implies-scan assertion.
- On base `5d403acd8` (the test added, the claim read as the base field):

```
compressor:  W4 dual fast path: a stale claim was believed   (FAILED)
multiband:   W4 dual fast path: a stale claim was believed   (FAILED)
parametric-eq: W4 dual fast path: a stale claim was believed (FAILED)
```

- At head: passes in all three.
- Test value: deleting or weakening the assertion on either body of any effect (for instance
  scanning only the left channel on the dual body) turns it red; every other test that plants an
  off-rest word withdraws the claim by hand, so none reaches a stale claim.
- `silent::tests::the_claim_moves_only_as_the_table_says` (effect-runtime): a `settled` that scans a
  loud block, or a `settle_skipped` that withdraws on a partial mask, turns it red; neither moves a
  bit, so no effect gate sees either.
- `rest_tests::a_restore_off_rest_withdraws_the_standing_claim` in each of the three crates (debug
  builds): a resting bank with its claim standing restores an off-rest payload (a donor lane with
  the same planted word), must report the lane off rest and the claim withdrawn, and renders a
  `+0.0` block. Green on base too: it guards the one writer that can move a lane off rest, whose
  withdrawal no existing test pinned (mutations M10 to M12 below were green before it).

**Mutations (head).** Each run is the named test target; each mutant restored after its run.
- M1 compressor dual assertion scans the left channel only: red, `a_stale_silent_claim_...` (dual arm).
- M2 compressor collapsed assertion vacuous: red, the same test (collapsed arm).
- M3 EQ dual scan left only: red. M4 EQ collapsed vacuous: red.
- M5 multiband dual scan left side only: red. M6 multiband collapsed vacuous: red.
- M7 `SilentClaim::trusted` never scans: red in all three crates.
- M8 `settle_skipped` withdraws on a partial mask: red, `the_claim_moves_only_as_the_table_says`.
- M9 `settled` evaluates the scan before the quiet leg: red, the same test.
- M10 / M11 / M12 the restore withdrawal deleted (compressor `Instance::restore`, EQ bank
  `restore_track_state_payload`, multiband `restore_track_state_payload`): first run green over
  each crate's whole suite, so `a_restore_off_rest_withdraws_the_standing_claim` was added; with it,
  red in each crate.

**No bit moved.** A scratch harness, never committed: one driver shared by the three effects
(`fx6/nbm_driver.rs` in the coordinator's scratchpad) plus one glue file each, and #2264's own
limiter harness for the limiter. Per case: a bank of random members (padded lanes included),
rate, link mode and bypass, and a loud donor bank of the same requests, driven 900 steps through
loud bursts, long `+0.0` stretches, `-0.0` and tiny single words, short and full blocks, collapsed
segments with their disengage copy (symmetric cases), automation, both resets, lane resets, link
retargets (glide and snap), restores from the bank's own and the donor's payloads, in-bank and
cross-bank `swap_bank_lanes`, `copy_lane_from`, a standalone defaults exchange round trip, the EQ's
prepared-target retunes, and whole-bank `advance_clocks` skips whenever the bank reported rest.
After every step it hashed (FNV-1a) every output bit, every state word (each lane's envelope or
integrators, coefficient words, every ramp field, link ramps, silence counters, prepared defaults,
link modes, `collapsed`, every member's payload), the claim and the rest mask. 48 seeds x W4, W8,
debug profile, base `5d403acd8` and head.

| effect | digests | identical | quiet blocks with the claim standing | whole-bank skips |
|---|---|---|---|---|
| compressor | 96 | all | 9,405 | 10,912 |
| multiband | 96 | all | 13,163 | 16,753 |
| EQ, swaps and copies (no retunes) | 96 | all | 6,478 | 12,636 |
| EQ, retunes (swaps and copies replaced by lane resets) | 96 | all | 8,043 | 13,415 |
| limiter (#2264 harness) | 96 | all | 9,430 fast-path engagements | 10,939 |

- The head runs executed every new debug assertion over all 345,600 effect steps and none tripped.
- The limiter's base run hashed the same words as head: base's `bank_state_words` lost its
  constant `silent.bypass()` word for the comparison, the word this change deletes.
- The EQ is split in two because, on base and head alike, the harness's random retunes followed by
  a direct lane swap trip the EQ's own `a live EQ lane's target words are its targets' design`
  assertion (see the follow-ups); no claim is involved.
- After the runs, two cosmetic edits landed (a `let every = all_lanes::<_>()` local at the
  compressor's and the EQ's claim sites, and `cargo fmt`); neither changes a value.

**Gates (head).**
- `cargo test --locked -p effect-runtime -p compressor -p parametric-eq -p multiband-compressor -p true-peak-limiter`:
  all pass (81 test binaries), with every new debug assertion active over every render.
- `cargo test --locked -p effect-compiler -p graph-compiler -p de-esser -p dynamic-eq -p loudness -p rack -p graph`:
  all pass (70 binaries).
- `cargo test --locked -p host-core --features test-support`: all 58 test binaries pass.
- `cargo clippy --locked` on the five crates, `--all-targets -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps` on the five crates: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (294 marked regions in 46 files)`.
  The render change adds no allocation, lock or syscall: `SilentClaim` is one `bool` whose methods
  are `#[inline(always)]`, the scan closures run in debug builds only, and the compressor's and the
  limiter's dead bypass compares are gone.
- Overlap: PR #2314 (#2285) edits `crates/true-peak-limiter/src/lib.rs` too. This change there is
  the bypass word only; the two touch adjacent lines in `LimiterCore::advance_clocks`
  (`proved_under(..)` and `settle_skipped(..)` around #2314's `link.decide` hunk), so whichever
  lands second resolves a one-hunk textual conflict.

**Follow-ups (not in this issue).**
- Move the limiter onto the shared `SilentClaim` once #2314 merges. It needs a withdrawing partial
  skip (`settle_skipped` there sets `proved = whole`) and keeps its `lanes_at_rest` trust point.
- The long-term model #2264's review recommended (a withdrawing accessor for between-block writes)
  applies to all four effects equally.
- The scratch harness found, on base and head alike, that the EQ's `swap_lanes_direct` trips its
  own `a live EQ lane's target words are its targets' design` debug assertion after a schedule of
  random prepared-target retunes, resets and cross-bank swaps (step 765 of W4 seed 1). Not this
  issue's: no claim is involved, and the harness may be the one breaking the contract (its retunes
  design every band from the lane's prepared values, not its current ones). Worth a look.
