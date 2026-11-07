# Limiter: the silent fixed-point claim is a hand-cleared flag that lanes_at_rest trusts without a scan

Tracker: #2258, item FX-1 (true-peak limiter only). Found by the implicit-state-machine audit of
2026-11-06.

## Problem
`LimiterCore` (`crates/true-peak-limiter/src/lib.rs`) kept the #182 S2 silent fixed-point claim as
two loose fields, `silent_fixed_point` and `silent_bypass`. Every path that can move lane state
between two blocks had to clear the bool by hand. While the claim stands, `lanes_at_rest` returns
every lane without scanning, so a missed withdrawal would make a caller (`rack`'s whole-bank skip)
skip a bank that is not at rest and freeze its rings. Claim-equals-scan was asserted only inside
`rest_tests`' probe. `exchange_lane_defaults` did not withdraw the claim, and the quiet predicate
was written twice, in `process_block` and `process_block_mono`.

## Contract
- Find out, and record, whether any mutator path leaves a stale claim today. If one does, a
  regression test reaches it, red on main.
- Model the claim as a `SilentClaim` newtype whose state is private. It is earned only from the
  scan's answer or the documented whole-bank skip, and withdrawn through one method.
- `debug_assert!(claim => scan)` wherever the claim is trusted: `lanes_at_rest` and both silent
  fast paths.
- One quiet predicate shared by the dual and the collapsed block.
- `exchange_lane_defaults` withdraws the claim.

## Non-goals
The compressor, parametric EQ and multiband `silent_fixed_point` flags (follow-ups; there the flag
only affects speed). FX-2 (`gain_linked`/`relink_*`). No rendered bit, no state word and no flag
value may move.

## Gates
- A red-on-main regression test, or a recorded proof that no stale claim is reachable.
- A one-time no-bit-moved comparison against base, as PR evidence.
- `cargo test --locked -p true-peak-limiter` and its in-workspace consumers, clippy `-D warnings`,
  `cargo fmt --check`, `scripts/check-workspace-policy.sh`, the realtime policy tool.

## Evidence (attempt 1)

Base: `main` @ `de0878e23`.

**Reachability: no stale claim is reachable today.** The claim is earned in exactly three places,
each from the scan or from a caller's scan-derived mask: the end of `process_block` (`quiet` and
`scan_lanes_at_rest() == every lane`), the end of `process_block_mono` (the left channel's scan),
and `advance_clocks` (whole-bank mask; the caller's mask must be a subset of `lanes_at_rest`, which
`PreparedTruePeakLimiterBank::advance_clocks` debug-asserts). The scan reads `history`,
`main_ring`, `required_ring`, `box_ring`, `reduction`, `prefix`, `box_sum` against the lane's
window, the limit and release ramps, and whether the link weight is gliding. Every `&mut` path on
`LimiterCore` and on the two prepared types, checked against that list:
- *Both resets and `reset_lane`* withdraw. Without the withdrawal they would still be sound: they
  land on `clear_runtime`'s words with ramps snapped, which is the rest state (already recorded in
  `tests/MUTATIONS.md`, and confirmed by M6/M7 below).
- *`restore_track`* withdraws after commit. It is the one writer that can move a lane off rest, and
  it is covered (`a_restore_withdraws_the_silence_claim`, `a_restore_off_rest_withdraws_the_standing_claim`).
- *Automation* withdraws on both entry points before `apply_automation`. A zero-window span
  snaps a ramp, but at rest the output is `+0.0` whatever the coefficients, and the block then
  re-settles the claim from its own scan.
- *`retarget_link`* writes only the link mode and weight. A glide makes the claim not stand
  (`silent_claim_stands` reads `glide_frames() == 0`, and `quiet` reads the block's glide); a
  zero-sample retarget snaps the weight and leaves the lane at rest.
- *`exchange_lane_defaults`* (the #1747 lane swap) wrote defaults, the lane's mode and a resting
  link weight without withdrawing. `rest_link` can only end a glide, never move a lane off rest,
  and `swap_bank_lanes` restores a payload right after the exchange, which withdraws. Benign.
- *`desymmetrize`* copies the left channel over the right. A claim earned while collapsed is a
  claim about the left channel, which the right then equals.
- *A D7 recovery* runs after the claim is settled, inside the block, and returns each failed lane
  to `clear_runtime`'s words; the survivors were at rest by the scan.
- *Preparation* (`new`, `with_active_lanes`, `with_lane_links`) starts withdrawn.
- *Bypass* is fixed at preparation (`LimiterCoef::bypass` has no writer), so the bypass leg is inert.

So no test can be red on main for a stale claim. Hardened instead, below.

**Model chosen.** `SilentClaim` lives in its own module (`mod silent`), so its two words
(`proved`, `bypass`) are private to its methods and nothing else in the crate can write them:
`new`, `withdraw`, `settle(quiet, rest, every, bypass)` (rendered block: earned iff quiet and the
scan answered every lane), `settle_skipped(whole, bypass)` (skip), `bypass_held` and
`proved_under`. The restatement of `advance_clocks` is exact: the old
`if !(flag && held) { flag = held; bypass = now } if !whole { flag = false }` is
`flag = whole && held; bypass = now`, with the #990 recompute still guarded by
`!(flag && held)`.
- `debug_assert_eq!(scan, every lane)` at all three trust points: `lanes_at_rest` (both channels,
  or the left alone while collapsed, as the scan reads), the dual fast path (both channels), the
  collapsed fast path (left).
- `block_is_quiet(glide, left, Option<right>)` is the one admission predicate; `ramps_settled(mono)`
  is shared with `silent_claim_stands`. Same legs, same values; only the evaluation order of pure
  reads changed.
- `exchange_lane_defaults` withdraws the claim, so every between-block writer withdraws without
  exception and no writer's soundness rests on what its caller does next.

**Test.** `rest_tests::a_stale_silent_claim_is_refused_where_it_is_trusted` (debug builds). For
each vector width it brings a bank to rest with the claim standing, plants one off-rest `box_ring`
word on lane 0 without withdrawing (the shape of a future writer that forgets), and drives each
trust point: `lanes_at_rest` and the dual fast path (word on the right channel), the collapsed fast
path (word on the left). Each must panic with the claim-implies-scan assertion.
- On base `de0878e23` (test file added, helpers unchanged): `W4 lanes_at_rest: a stale claim was
  believed` (FAILED).
- At head: passes.
- Test value: deleting or weakening any of the three assertions (for instance scanning only the
  left channel on the dual body) turns it red; every other test that plants an off-rest word
  withdraws the claim by hand, so none reaches a stale claim.

**Mutations (head).** Each run is the whole `true-peak-limiter` test suite.
- Assertion at `lanes_at_rest` made vacuous: red, `a_stale_silent_claim_is_refused_where_it_is_trusted`.
- Dual fast-path assertion scanning the left channel only: red, the same test.
- Collapsed fast-path assertion made vacuous: red, the same test.
- `self.silent.withdraw()` deleted from `restore_track`: red, `a_restore_withdraws_the_silence_claim`,
  `a_restore_off_rest_withdraws_the_standing_claim` and `advance_clocks_is_a_skipped_zero_block`
  (base had the first two only; the new assertions now catch it on more paths).
- Withdrawal deleted from `process_bank`: red, `automation_withdraws_the_claim_on_the_bank_path_too`.
- Withdrawal deleted from `exchange_lane_defaults`, from `reset` or from `reset_lane`: nothing red,
  and nothing can be red. This is the reachability proof above: none of the three can move a lane off
  rest. Recorded in `tests/MUTATIONS.md` (rows 2264-1 to 2264-8).

**No bit moved.** A scratch harness, never committed, drove 48 random cases per vector width
(W4, W8) of 900 steps each through banks with random members, parameters, link modes, bypass and
collapse eligibility: loud bursts, long `+0.0` stretches, `-0.0` and tiny single words, short and
full blocks, collapsed segments with their disengage copy, automation, both resets, both lane
resets, link retargets (glide and snap), restores from the bank's own and from a loud donor's
payloads, in-bank and cross-bank lane swaps (`swap_bank_lanes`), and whole-bank skips
(`advance_clocks`) whenever the bank reported rest. It hashed (FNV-1a) every output bit, every state
word including the claim's two words and `gain_linked`, the rest mask and the fast-path engagement
count after every step.
- Base `de0878e23` and head: all 96 digests (48 seeds x W4, W8) identical, both in the debug profile.
  The runs took 9,430 fast-path engagements and 10,939 whole-bank skips, so both claim paths were
  exercised. The head run also ran every new debug assertion over all 86,400 steps, and none
  tripped.
- The crate's own fast-versus-forced-slow gates (`a_settled_silent_limiter_renders_exactly_the_never_fast_path`
  and the rest) compare every rendered bit and state word of each arm, and pass unchanged.

**Gates (head).**
- `cargo test --locked -p true-peak-limiter`: all pass (lib 58, every integration target).
- `cargo test --locked -p effect-compiler -p rack -p graph -p loudness`: all pass.
- `cargo test --locked -p host-core --features test-support`: all 58 test binaries pass, with the
  new debug assertions active over every limiter session render.
- `cargo clippy --locked -p true-peak-limiter --all-targets -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p true-peak-limiter --no-deps`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (292 marked regions in 46 files)`.
  The render change adds no call, allocation or lock: `SilentClaim` is two `bool`s with `const fn`
  methods, and the assertions are debug-only.

**Follow-ups (not in this issue).**
- Compressor, parametric EQ and multiband carry the same hand-cleared `silent_fixed_point` +
  `silent_bypass` pair (#2258 FX-1). There the flag only buys speed, but the same newtype and
  debug assertion would apply.
